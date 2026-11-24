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
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h21) => {
                    h21
                }
            } {
                f10(v26, v27)
            } else {
                let a23 = match v27.clone() {
                    Adt2::C0(_, _, _, h22, _, _, _, _, _, _, _) => {
                        h22
                    }
                };
                let a24 = 33u64;
                if nat_eq(a23, a24) {
                    f11(v26, v27)
                } else {
                    let a26 = match v27.clone() {
                        Adt2::C0(_, _, _, h25, _, _, _, _, _, _, _) => {
                            h25
                        }
                    };
                    let a27 = 66u64;
                    if if nat_eq(a26, a27) {
                        true
                    } else {
                        let a29 = match v27.clone() {
                            Adt2::C0(_, _, _, h28, _, _, _, _, _, _, _) => {
                                h28
                            }
                        };
                        let a30 = 67u64;
                        nat_eq(a29, a30)
                    } {
                        f12(v26, v27)
                    } else {
                        let a32 = match v27.clone() {
                            Adt2::C0(_, _, _, h31, _, _, _, _, _, _, _) => {
                                h31
                            }
                        };
                        let a33 = 41u64;
                        if nat_eq(a32, a33) {
                            f13(v26, v27, v28)
                        } else {
                            let a35 = 98u64;
                            let a36 = match v27.clone() {
                                Adt2::C0(_, _, _, h34, _, _, _, _, _, _, _) => {
                                    h34
                                }
                            };
                            if nat_le(a35, a36) {
                                if f14(match v27.clone() {
                                    Adt2::C0(_, _, _, _, h37, _, _, _, _, _, _) => {
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
        let a46 = v0;
        let a47 = 0u64;
        if if if nat_eq(a46, a47) {
            let a48 = f19(v1);
            let a49 = 2u64;
            nat_eq(a48, a49)
        } else {
            false
        } {
            let a50 = 0u64;
            let a51 = f20(v3);
            nat_lt(a50, a51)
        } else {
            false
        } {
            !f21(v1, v2, v3)
        } else {
            false
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a54 = match v1.clone() {
        Adt1::C0(_, h52, _, _, _, _, _, _, _, _, _) => {
            h52
        }
    };
    let a55 = match v2.clone() {
        Adt1::C0(_, h53, _, _, _, _, _, _, _, _, _) => {
            h53
        }
    };
    if if if if if if if if if if if if if if nat_eq(a54, a55) {
        true
    } else {
        let a58 = match v1.clone() {
            Adt1::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        };
        let a59 = match v3.clone() {
            Adt1::C0(h57, _, _, _, _, _, _, _, _, _, _) => {
                h57
            }
        };
        nat_eq(a58, a59)
    } {
        f22(v1.clone(), v2.clone())
    } else {
        false
    } {
        f23(v1.clone(), v3.clone())
    } else {
        false
    } {
        f24(v0, match v1.clone() {
            Adt1::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        }, v2.clone(), match v3.clone() {
            Adt1::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        })
    } else {
        false
    } {
        f25(match v1.clone() {
            Adt1::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        }, match v3.clone() {
            Adt1::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        })
    } else {
        false
    } {
        let a65 = 0u64;
        let a66 = f26(match v3.clone() {
            Adt1::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        });
        nat_lt(a65, a66)
    } else {
        false
    } {
        let a69 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h67, _, _, _, _, _) => {
                h67
            }
        };
        let a70 = f26(match v1.clone() {
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        });
        nat_eq(a69, a70)
    } else {
        false
    } {
        let a73 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h71, _, _, _, _, _) => {
                h71
            }
        };
        let a74 = f26(match v3.clone() {
            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        });
        nat_eq(a73, a74)
    } else {
        false
    } {
        let a80 = {
            let a77 = match v1.clone() {
                Adt1::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            };
            let a78 = f27(match v1.clone() {
                Adt1::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                    h76
                }
            });
            let a79 = 1u64;
            nat_rem(a77, a78, a79)
        };
        let a81 = 0u64;
        nat_eq(a80, a81)
    } else {
        false
    } {
        let a87 = {
            let a84 = match v3.clone() {
                Adt1::C0(_, h82, _, _, _, _, _, _, _, _, _) => {
                    h82
                }
            };
            let a85 = f27(match v3.clone() {
                Adt1::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                    h83
                }
            });
            let a86 = 1u64;
            nat_rem(a84, a85, a86)
        };
        let a88 = 0u64;
        nat_eq(a87, a88)
    } else {
        false
    } {
        f28(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f28(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f29({
            let a95 = {
                let a91 = match v3.clone() {
                    Adt1::C0(_, h89, _, _, _, _, _, _, _, _, _) => {
                        h89
                    }
                };
                let a92 = f27(match v3.clone() {
                    Adt1::C0(h90, _, _, _, _, _, _, _, _, _, _) => {
                        h90
                    }
                });
                let a93 = 0u64;
                nat_quot(a91, a92, a93)
            };
            let a96 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h94, _, _, _, _, _) => {
                    h94
                }
            };
            nat_mul(a95, a96)?
        }, match v3.clone() {
            Adt1::C0(_, _, h97, _, _, _, _, _, _, _, _) => {
                h97
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h98, _, _, _, _, _, _, _) => {
                h98
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h99, _, _, _, _, _, _) => {
                h99
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h100, _, _, _, _) => {
                h100
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h101, _, _, _) => {
                h101
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h102, _, _) => {
                h102
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f30(v1.clone()) {
            0u64
        } else {
            let a108 = match v1.clone() {
                Adt1::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                    h103
                }
            };
            let a109 = {
                let a106 = match v1.clone() {
                    Adt1::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                        h104
                    }
                };
                let a107 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h105, _, _, _, _, _, _) => {
                        h105
                    }
                };
                nat_mul(a106, a107)?
            };
            nat_mul(a108, a109)?
        }, 3u64, f31(match v3.clone() {
            Adt1::C0(h110, _, _, _, _, _, _, _, _, _, _) => {
                h110
            }
        }), v0, 0u64, 0u64, 0u64, v1, v2, v3, true)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f23(v1.clone(), v3.clone()) {
        f32(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f30(v1.clone()) {
            true
        } else {
            let a113 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h111, _, _, _, _, _) => {
                    h111
                }
            };
            let a114 = f20(match v1.clone() {
                Adt1::C0(h112, _, _, _, _, _, _, _, _, _, _) => {
                    h112
                }
            });
            if if if if if if if nat_eq(a113, a114) {
                let a117 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h115, _, _, _, _, _) => {
                        h115
                    }
                };
                let a118 = f20(match v3.clone() {
                    Adt1::C0(h116, _, _, _, _, _, _, _, _, _, _) => {
                        h116
                    }
                });
                nat_eq(a117, a118)
            } else {
                false
            } {
                f33(v1.clone())
            } else {
                false
            } {
                f33(v2.clone())
            } else {
                false
            } {
                f33(v3.clone())
            } else {
                false
            } {
                f34(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f28(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f28(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f30(v1.clone()) {
            true
        } else {
            f35(v3.clone())?
        } {
            0u64
        } else {
            let a121 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h119, _) => {
                    h119
                }
            };
            let a122 = f20(match v3.clone() {
                Adt1::C0(h120, _, _, _, _, _, _, _, _, _, _) => {
                    h120
                }
            });
            let a123 = 0u64;
            nat_quot(a121, a122, a123)
        };
        Ok(Adt3::C2(Adt2::C0(f36(f37(v1.clone())?, v4, f38(match v1.clone() {
            Adt1::C0(h124, _, _, _, _, _, _, _, _, _, _) => {
                h124
            }
        }, match v2.clone() {
            Adt1::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
                h125
            }
        }, match v3.clone() {
            Adt1::C0(h126, _, _, _, _, _, _, _, _, _, _) => {
                h126
            }
        })?)?, {
            let a127 = 3u64;
            let a128 = v4;
            nat_add(a127, a128)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f39(v2, v3.clone()) {
        let a130 = match v0.clone() {
            Adt1::C0(h129, _, _, _, _, _, _, _, _, _, _) => {
                h129
            }
        };
        let a131 = 0u64;
        if nat_eq(a130, a131) {
            true
        } else {
            let a133 = match v0.clone() {
                Adt1::C0(h132, _, _, _, _, _, _, _, _, _, _) => {
                    h132
                }
            };
            let a134 = 1u64;
            nat_eq(a133, a134)
        }
    } else {
        false
    } {
        let a136 = 0u64;
        let a137 = f26(match v3.clone() {
            Adt1::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        });
        nat_lt(a136, a137)
    } else {
        false
    } {
        let a139 = 0u64;
        let a140 = f40(match v1.clone() {
            Adt1::C0(h138, _, _, _, _, _, _, _, _, _, _) => {
                h138
            }
        });
        nat_lt(a139, a140)
    } else {
        false
    } {
        let a143 = match v0.clone() {
            Adt1::C0(_, h141, _, _, _, _, _, _, _, _, _) => {
                h141
            }
        };
        let a144 = match v3.clone() {
            Adt1::C0(_, h142, _, _, _, _, _, _, _, _, _) => {
                h142
            }
        };
        if nat_eq(a143, a144) {
            let a150 = {
                let a147 = match v0.clone() {
                    Adt1::C0(_, h145, _, _, _, _, _, _, _, _, _) => {
                        h145
                    }
                };
                let a148 = f27(match v3.clone() {
                    Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _) => {
                        h146
                    }
                });
                let a149 = 1u64;
                nat_rem(a147, a148, a149)
            };
            let a151 = 0u64;
            nat_eq(a150, a151)
        } else {
            false
        }
    } else {
        false
    } {
        let a154 = match v0.clone() {
            Adt1::C0(_, _, _, h152, _, _, _, _, _, _, _) => {
                h152
            }
        };
        let a155 = match v3.clone() {
            Adt1::C0(_, _, _, h153, _, _, _, _, _, _, _) => {
                h153
            }
        };
        nat_eq(a154, a155)
    } else {
        false
    } {
        let a158 = match v0.clone() {
            Adt1::C0(_, _, _, _, h156, _, _, _, _, _, _) => {
                h156
            }
        };
        let a159 = match v3.clone() {
            Adt1::C0(_, _, _, _, h157, _, _, _, _, _, _) => {
                h157
            }
        };
        nat_eq(a158, a159)
    } else {
        false
    } {
        let a162 = match v0.clone() {
            Adt1::C0(_, _, h160, _, _, _, _, _, _, _, _) => {
                h160
            }
        };
        let a163 = match v1.clone() {
            Adt1::C0(_, h161, _, _, _, _, _, _, _, _, _) => {
                h161
            }
        };
        nat_eq(a162, a163)
    } else {
        false
    } {
        let a165 = match v1.clone() {
            Adt1::C0(_, _, _, _, h164, _, _, _, _, _, _) => {
                h164
            }
        };
        let a166 = 1u64;
        nat_eq(a165, a166)
    } else {
        false
    } {
        let a168 = 0u64;
        let a169 = match v1.clone() {
            Adt1::C0(_, _, h167, _, _, _, _, _, _, _, _) => {
                h167
            }
        };
        nat_lt(a168, a169)
    } else {
        false
    } {
        let a171 = 0u64;
        let a172 = match v1.clone() {
            Adt1::C0(_, _, _, h170, _, _, _, _, _, _, _) => {
                h170
            }
        };
        nat_lt(a171, a172)
    } else {
        false
    } {
        let a178 = {
            let a175 = match v0.clone() {
                Adt1::C0(_, _, _, h173, _, _, _, _, _, _, _) => {
                    h173
                }
            };
            let a176 = match v1.clone() {
                Adt1::C0(_, _, h174, _, _, _, _, _, _, _, _) => {
                    h174
                }
            };
            let a177 = 0u64;
            nat_rem(a175, a176, a177)
        };
        let a179 = 0u64;
        nat_eq(a178, a179)
    } else {
        false
    } {
        let a185 = {
            let a182 = match v0.clone() {
                Adt1::C0(_, _, _, _, h180, _, _, _, _, _, _) => {
                    h180
                }
            };
            let a183 = match v1.clone() {
                Adt1::C0(_, _, _, h181, _, _, _, _, _, _, _) => {
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
        f41(v0.clone())
    } else {
        false
    } {
        let a188 = match v3.clone() {
            Adt1::C0(_, h187, _, _, _, _, _, _, _, _, _) => {
                h187
            }
        };
        let a189 = 1u64;
        if nat_eq(a188, a189) {
            true
        } else {
            let a192 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h190, _, _, _, _, _) => {
                    h190
                }
            };
            let a193 = f26(match v3.clone() {
                Adt1::C0(h191, _, _, _, _, _, _, _, _, _, _) => {
                    h191
                }
            });
            nat_eq(a192, a193)
        }
    } else {
        false
    } {
        let v4: u64 = f37(v1.clone())?;
        let v5: u64 = {
            let a199 = match v0.clone() {
                Adt1::C0(_, _, h194, _, _, _, _, _, _, _, _) => {
                    h194
                }
            };
            let a200 = {
                let a197 = match v0.clone() {
                    Adt1::C0(_, _, _, h195, _, _, _, _, _, _, _) => {
                        h195
                    }
                };
                let a198 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h196, _, _, _, _, _, _) => {
                        h196
                    }
                };
                nat_mul(a197, a198)?
            };
            nat_mul(a199, a200)?
        };
        let v6: u64 = if f30(v0.clone()) {
            0u64
        } else {
            let a208 = {
                let a201 = 2u64;
                let a202 = v4;
                nat_mul(a201, a202)?
            };
            let a209 = {
                let a206 = v5;
                let a207 = {
                    let a204 = 1u64;
                    let a205 = f42(v0.clone(), match v3.clone() {
                        Adt1::C0(h203, _, _, _, _, _, _, _, _, _, _) => {
                            h203
                        }
                    })?;
                    nat_add(a204, a205)?
                };
                nat_mul(a206, a207)?
            };
            nat_add(a208, a209)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f30(v0.clone()) {
            3u64
        } else {
            let a210 = 3u64;
            let a211 = v4;
            nat_add(a210, a211)?
        }, f31(match v3.clone() {
            Adt1::C0(h212, _, _, _, _, _, _, _, _, _, _) => {
                h212
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f27(match v2.clone() {
        Adt1::C0(h213, _, _, _, _, _, _, _, _, _, _) => {
            h213
        }
    });
    if if if if if if if f18(match v2.clone() {
        Adt1::C0(h214, _, _, _, _, _, _, _, _, _, _) => {
            h214
        }
    }) {
        f39(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a216 = 0u64;
        let a217 = f19(match v0.clone() {
            Adt1::C0(h215, _, _, _, _, _, _, _, _, _, _) => {
                h215
            }
        });
        nat_lt(a216, a217)
    } else {
        false
    } {
        let a220 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h218, _, _, _, _, _) => {
                h218
            }
        };
        let a221 = f19(match v0.clone() {
            Adt1::C0(h219, _, _, _, _, _, _, _, _, _, _) => {
                h219
            }
        });
        nat_eq(a220, a221)
    } else {
        false
    } {
        let a226 = {
            let a223 = match v0.clone() {
                Adt1::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                    h222
                }
            };
            let a224 = v3;
            let a225 = 1u64;
            nat_rem(a223, a224, a225)
        };
        let a227 = 0u64;
        if nat_eq(a226, a227) {
            let a232 = {
                let a229 = match v2.clone() {
                    Adt1::C0(_, h228, _, _, _, _, _, _, _, _, _) => {
                        h228
                    }
                };
                let a230 = v3;
                let a231 = 1u64;
                nat_rem(a229, a230, a231)
            };
            let a233 = 0u64;
            nat_eq(a232, a233)
        } else {
            false
        }
    } else {
        false
    } {
        let a236 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h234, _, _, _, _, _) => {
                h234
            }
        };
        let a237 = f26(match v2.clone() {
            Adt1::C0(h235, _, _, _, _, _, _, _, _, _, _) => {
                h235
            }
        });
        nat_eq(a236, a237)
    } else {
        false
    } {
        let a238 = f37(v0.clone())?;
        let a239 = f37(v2.clone())?;
        nat_eq(a238, a239)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a240 = f37(v0.clone())?;
            let a241 = v3;
            let a242 = 0u64;
            nat_quot(a240, a241, a242)
        }, 3u64, f31(match v2.clone() {
            Adt1::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                h243
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a245 = match v2.clone() {
        Adt1::C0(h244, _, _, _, _, _, _, _, _, _, _) => {
            h244
        }
    };
    let a246 = 0u64;
    if if if if if if if if if if if nat_eq(a245, a246) {
        let a248 = match v3.clone() {
            Adt1::C0(h247, _, _, _, _, _, _, _, _, _, _) => {
                h247
            }
        };
        let a249 = 26u64;
        nat_eq(a248, a249)
    } else {
        false
    } {
        let a251 = match v2.clone() {
            Adt1::C0(_, h250, _, _, _, _, _, _, _, _, _) => {
                h250
            }
        };
        let a252 = 2147483647u64;
        nat_le(a251, a252)
    } else {
        false
    } {
        let a255 = match v3.clone() {
            Adt1::C0(_, h253, _, _, _, _, _, _, _, _, _) => {
                h253
            }
        };
        let a256 = match v2.clone() {
            Adt1::C0(_, h254, _, _, _, _, _, _, _, _, _) => {
                h254
            }
        };
        nat_le(a255, a256)
    } else {
        false
    } {
        let a259 = match v2.clone() {
            Adt1::C0(_, _, h257, _, _, _, _, _, _, _, _) => {
                h257
            }
        };
        let a260 = match v3.clone() {
            Adt1::C0(_, _, h258, _, _, _, _, _, _, _, _) => {
                h258
            }
        };
        nat_eq(a259, a260)
    } else {
        false
    } {
        let a263 = match v2.clone() {
            Adt1::C0(_, _, _, h261, _, _, _, _, _, _, _) => {
                h261
            }
        };
        let a264 = match v3.clone() {
            Adt1::C0(_, _, _, h262, _, _, _, _, _, _, _) => {
                h262
            }
        };
        nat_eq(a263, a264)
    } else {
        false
    } {
        let a267 = match v2.clone() {
            Adt1::C0(_, _, _, _, h265, _, _, _, _, _, _) => {
                h265
            }
        };
        let a268 = match v3.clone() {
            Adt1::C0(_, _, _, _, h266, _, _, _, _, _, _) => {
                h266
            }
        };
        nat_eq(a267, a268)
    } else {
        false
    } {
        let a269 = v0;
        let a270 = 67u64;
        if nat_eq(a269, a270) {
            true
        } else {
            let a271 = v1;
            let a272 = 1u64;
            if nat_le(a271, a272) {
                let a275 = match v3.clone() {
                    Adt1::C0(_, h273, _, _, _, _, _, _, _, _, _) => {
                        h273
                    }
                };
                let a276 = match v2.clone() {
                    Adt1::C0(_, h274, _, _, _, _, _, _, _, _, _) => {
                        h274
                    }
                };
                nat_eq(a275, a276)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f43(v2.clone())?
    } else {
        false
    } {
        f43(v3.clone())?
    } else {
        false
    } {
        f44(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a282 = match v3.clone() {
                Adt1::C0(_, _, h277, _, _, _, _, _, _, _, _) => {
                    h277
                }
            };
            let a283 = {
                let a280 = match v3.clone() {
                    Adt1::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                        h278
                    }
                };
                let a281 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h279, _, _, _, _, _, _) => {
                        h279
                    }
                };
                nat_mul(a280, a281)?
            };
            nat_mul(a282, a283)?
        }, 3u64, None::<u64>, v0, {
            let a284 = v0;
            let a285 = 67u64;
            if nat_eq(a284, a285) {
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
    let a286 = v0;
    let a287 = 6u64;
    nat_le(a286, a287)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a288 = v2;
    let a289 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a288, a289) {
        let a291 = 0u64;
        let a292 = f19(match v5.clone() {
            Adt1::C0(h290, _, _, _, _, _, _, _, _, _, _) => {
                h290
            }
        });
        nat_lt(a291, a292)
    } else {
        false
    } {
        let a295 = match v5.clone() {
            Adt1::C0(h293, _, _, _, _, _, _, _, _, _, _) => {
                h293
            }
        };
        let a296 = match v6.clone() {
            Adt1::C0(h294, _, _, _, _, _, _, _, _, _, _) => {
                h294
            }
        };
        nat_eq(a295, a296)
    } else {
        false
    } {
        let a299 = match v5.clone() {
            Adt1::C0(h297, _, _, _, _, _, _, _, _, _, _) => {
                h297
            }
        };
        let a300 = match v7.clone() {
            Adt1::C0(h298, _, _, _, _, _, _, _, _, _, _) => {
                h298
            }
        };
        nat_eq(a299, a300)
    } else {
        false
    } {
        f23(v5.clone(), v6.clone())
    } else {
        false
    } {
        f45(v5.clone())?
    } else {
        false
    } {
        f45(v6.clone())?
    } else {
        false
    } {
        f45(v7.clone())?
    } else {
        false
    } {
        let a303 = match v5.clone() {
            Adt1::C0(_, _, h301, _, _, _, _, _, _, _, _) => {
                h301
            }
        };
        let a304 = match v7.clone() {
            Adt1::C0(_, _, h302, _, _, _, _, _, _, _, _) => {
                h302
            }
        };
        nat_eq(a303, a304)
    } else {
        false
    } {
        let a307 = match v5.clone() {
            Adt1::C0(_, _, _, h305, _, _, _, _, _, _, _) => {
                h305
            }
        };
        let a308 = match v7.clone() {
            Adt1::C0(_, _, _, h306, _, _, _, _, _, _, _) => {
                h306
            }
        };
        nat_eq(a307, a308)
    } else {
        false
    } {
        let a311 = match v5.clone() {
            Adt1::C0(_, _, _, _, h309, _, _, _, _, _, _) => {
                h309
            }
        };
        let a312 = match v7.clone() {
            Adt1::C0(_, _, _, _, h310, _, _, _, _, _, _) => {
                h310
            }
        };
        nat_eq(a311, a312)
    } else {
        false
    } {
        let a319 = match v7.clone() {
            Adt1::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                h313
            }
        };
        let a320 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h314, _, _, _, _, _, _, _, _, _) => {
                    h314
                }
            }
        } else {
            let a316 = match v5.clone() {
                Adt1::C0(_, h315, _, _, _, _, _, _, _, _, _) => {
                    h315
                }
            };
            let a317 = 2u64;
            let a318 = 0u64;
            nat_quot(a316, a317, a318)
        };
        nat_eq(a319, a320)
    } else {
        false
    } {
        let a322 = match v7.clone() {
            Adt1::C0(_, h321, _, _, _, _, _, _, _, _, _) => {
                h321
            }
        };
        let a323 = 2147483647u64;
        nat_le(a322, a323)
    } else {
        false
    } {
        f28(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f28(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f14(v0) {
            let a326 = 1u64;
            let a327 = {
                let a324 = f46(v0);
                let a325 = f37(v7.clone())?;
                nat_mul(a324, a325)?
            };
            nat_add(a326, a327)?
        } else {
            let a328 = 4u64;
            let a329 = f37(v7.clone())?;
            nat_mul(a328, a329)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a330 = 99u64;
            let a331 = v2;
            nat_add(a330, a331)?
        }, v0, v3, v4, v5, v6, v7, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a333 = v0;
    let a334 = match v1.clone() {
        Adt2::C0(h332, _, _, _, _, _, _, _, _, _, _) => {
            h332
        }
    };
    if nat_lt(a333, a334) {
        let v2: u64 = {
            let a338 = v0;
            let a339 = {
                let m337 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h336, _, _, _) => {
                        h336
                    }
                };
                match m337 {
                    Adt1::C0(_, h335, _, _, _, _, _, _, _, _, _) => {
                        h335
                    }
                }
            };
            nat_mul(a338, a339)?
        };
        Ok(Adt3::C7(f47(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h340, _, _, _) => {
                h340
            }
        })?, f48(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h341, _, _, _) => {
                h341
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h342, _, _) => {
                h342
            }
        })?, f47(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h343, _) => {
                h343
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a345 = v0;
    let a346 = match v1.clone() {
        Adt2::C0(h344, _, _, _, _, _, _, _, _, _, _) => {
            h344
        }
    };
    if nat_lt(a345, a346) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h347, _) => {
                h347
            }
        };
        let v3: u64 = f27(match v2.clone() {
            Adt1::C0(h348, _, _, _, _, _, _, _, _, _, _) => {
                h348
            }
        });
        let v4: u64 = {
            let a350 = match v2.clone() {
                Adt1::C0(_, h349, _, _, _, _, _, _, _, _, _) => {
                    h349
                }
            };
            let a351 = v3;
            let a352 = 0u64;
            nat_quot(a350, a351, a352)
        };
        let v5: u64 = {
            let a353 = v0;
            let a354 = v4;
            let a355 = 0u64;
            nat_quot(a353, a354, a355)
        };
        let v6: u64 = {
            let a357 = v5;
            let a358 = match v2.clone() {
                Adt1::C0(_, _, h356, _, _, _, _, _, _, _, _) => {
                    h356
                }
            };
            let a359 = 0u64;
            nat_quot(a357, a358, a359)
        };
        Ok(Adt3::C6(f47({
            let a360 = v0;
            let a361 = v3;
            nat_mul(a360, a361)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h362, _, _, _) => {
                h362
            }
        })?, f49({
            let a363 = v0;
            let a364 = v4;
            let a365 = 0u64;
            nat_rem(a363, a364, a365)
        }, {
            let a367 = v5;
            let a368 = match v2.clone() {
                Adt1::C0(_, _, h366, _, _, _, _, _, _, _, _) => {
                    h366
                }
            };
            let a369 = 0u64;
            nat_rem(a367, a368, a369)
        }, {
            let a371 = v6;
            let a372 = match v2.clone() {
                Adt1::C0(_, _, _, h370, _, _, _, _, _, _, _) => {
                    h370
                }
            };
            let a373 = 0u64;
            nat_rem(a371, a372, a373)
        }, {
            let a375 = v6;
            let a376 = match v2.clone() {
                Adt1::C0(_, _, _, h374, _, _, _, _, _, _, _) => {
                    h374
                }
            };
            let a377 = 0u64;
            nat_quot(a375, a376, a377)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h378, _, _, _, _, _) => {
                h378
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h379, _, _, _, _) => {
                h379
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h380, _, _, _) => {
                h380
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h381, _, _) => {
                h381
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h382, _, _, _) => {
            h382
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h383, _) => {
            h383
        }
    };
    let a385 = v0;
    let a386 = match v1.clone() {
        Adt2::C0(h384, _, _, _, _, _, _, _, _, _, _) => {
            h384
        }
    };
    if nat_lt(a385, a386) {
        Ok(Adt3::C5({
            let a388 = v0;
            let a389 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h387, _, _, _, _) => {
                    h387
                }
            };
            nat_mul(a388, a389)?
        }, {
            let a391 = v0;
            let a392 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h390, _, _, _, _) => {
                    h390
                }
            };
            nat_mul(a391, a392)?
        }, match v2 {
            Adt1::C0(_, h393, _, _, _, _, _, _, _, _, _) => {
                h393
            }
        }, match v3 {
            Adt1::C0(_, h394, _, _, _, _, _, _, _, _, _) => {
                h394
            }
        }, {
            let a396 = match v1 {
                Adt2::C0(_, _, _, _, h395, _, _, _, _, _, _) => {
                    h395
                }
            };
            let a397 = 1u64;
            nat_eq(a396, a397)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a399 = v0;
    let a400 = match v1.clone() {
        Adt2::C0(h398, _, _, _, _, _, _, _, _, _, _) => {
            h398
        }
    };
    if nat_lt(a399, a400) {
        let v3: u64 = {
            let a402 = 2u64;
            let a403 = f37(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h401, _, _) => {
                    h401
                }
            })?;
            nat_mul(a402, a403)?
        };
        let a404 = v0;
        let a405 = v3;
        if nat_lt(a404, a405) {
            f50(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h406, _, _) => {
                    h406
                }
            }, {
                let m409 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h408, _) => {
                        h408
                    }
                };
                match m409 {
                    Adt1::C0(_, _, h407, _, _, _, _, _, _, _, _) => {
                        h407
                    }
                }
            }, v2)
        } else {
            f51({
                let a410 = v0;
                let a411 = v3;
                nat_sub(a410, a411)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h412, _, _, _) => {
                    h412
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h413, _, _) => {
                    h413
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h414, _) => {
                    h414
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a415 = v0;
    let a416 = 3u64;
    if nat_eq(a415, a416) {
        true
    } else {
        let a417 = v0;
        let a418 = 6u64;
        nat_eq(a417, a418)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h419, _, _, _, _) => {
            h419
        }
    };
    let a421 = v0;
    let a422 = match v1.clone() {
        Adt2::C0(h420, _, _, _, _, _, _, _, _, _, _) => {
            h420
        }
    };
    if nat_lt(a421, a422) {
        let a423 = v0;
        let a424 = 0u64;
        if nat_eq(a423, a424) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a425 = v0;
                let a426 = 1u64;
                nat_sub(a425, a426)
            };
            let v5: u64 = {
                let a428 = v4;
                let a429 = f46(match v1.clone() {
                    Adt2::C0(_, _, _, _, h427, _, _, _, _, _, _) => {
                        h427
                    }
                });
                let a430 = 0u64;
                nat_rem(a428, a429, a430)
            };
            let v6: u64 = {
                let a435 = 4u64;
                let a436 = {
                    let a432 = v4;
                    let a433 = f46(match v1.clone() {
                        Adt2::C0(_, _, _, _, h431, _, _, _, _, _, _) => {
                            h431
                        }
                    });
                    let a434 = 0u64;
                    nat_quot(a432, a433, a434)
                };
                nat_mul(a435, a436)?
            };
            let a437 = v5;
            let a438 = 2u64;
            if nat_lt(a437, a438) {
                f16({
                    let a439 = v6;
                    let a440 = v5;
                    nat_add(a439, a440)?
                }, v1)
            } else {
                let a441 = v5;
                let a442 = 2u64;
                if nat_eq(a441, a442) {
                    Ok(f52(0u64, 2u64, v3))
                } else {
                    let a443 = v5;
                    let a444 = 3u64;
                    if nat_eq(a443, a444) {
                        Ok(f53(0u64, 2u64, v3, {
                            let a445 = v2;
                            let a446 = 2u64;
                            nat_eq(a445, a446)
                        }))
                    } else {
                        let a447 = v5;
                        let a448 = 4u64;
                        if nat_eq(a447, a448) {
                            Ok(f52(1u64, 1u64, 3u64))
                        } else {
                            let a449 = v5;
                            let a450 = 5u64;
                            if nat_eq(a449, a450) {
                                Ok(f53(1u64, 1u64, 3u64, {
                                    let a451 = v2;
                                    let a452 = 0u64;
                                    nat_eq(a451, a452)
                                }))
                            } else {
                                let a453 = v5;
                                let a454 = 6u64;
                                if nat_eq(a453, a454) {
                                    Ok(f52(1u64, 2u64, v3))
                                } else {
                                    let a455 = v5;
                                    let a456 = 7u64;
                                    if nat_eq(a455, a456) {
                                        Ok(f53(1u64, 2u64, v3, {
                                            let a457 = v2;
                                            let a458 = 2u64;
                                            nat_eq(a457, a458)
                                        }))
                                    } else {
                                        let a460 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h459, _, _, _, _, _, _) => {
                                                h459
                                            }
                                        };
                                        let a461 = 3u64;
                                        if nat_eq(a460, a461) {
                                            let a462 = v5;
                                            let a463 = 15u64;
                                            if nat_eq(a462, a463) {
                                                f16({
                                                    let a464 = v6;
                                                    let a465 = 3u64;
                                                    nat_add(a464, a465)?
                                                }, v1)
                                            } else {
                                                Ok(f54(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h466, _, _, _, _, _) => {
                                                        h466
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a467 = v5;
                                            let a468 = 8u64;
                                            if nat_eq(a467, a468) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m471 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h470, _) => {
                                                            h470
                                                        }
                                                    };
                                                    match m471 {
                                                        Adt1::C0(h469, _, _, _, _, _, _, _, _, _, _) => {
                                                            h469
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a472 = v6;
                                                    let a473 = 3u64;
                                                    nat_add(a472, a473)?
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
        Adt2::C0(_, _, _, _, _, _, _, h474, _, _, _) => {
            h474
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h475, _, _) => {
            h475
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h476, _) => {
            h476
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h477, _, _, _, _, _, _, _) => {
            h477
        }
    };
    let v6: u64 = {
        let a478 = v0;
        let a479 = 4u64;
        let a480 = 0u64;
        nat_rem(a478, a479, a480)
    };
    let v7: u64 = {
        let a481 = v0;
        let a482 = 4u64;
        let a483 = 0u64;
        nat_quot(a481, a482, a483)
    };
    let v8: u64 = {
        let a485 = v7;
        let a486 = match v4.clone() {
            Adt1::C0(_, h484, _, _, _, _, _, _, _, _, _) => {
                h484
            }
        };
        let a487 = 0u64;
        nat_quot(a485, a486, a487)
    };
    let v9: u64 = f19(match v4.clone() {
        Adt1::C0(h488, _, _, _, _, _, _, _, _, _, _) => {
            h488
        }
    });
    let v10: u64 = {
        let a493 = {
            let a490 = v7;
            let a491 = match v4.clone() {
                Adt1::C0(_, h489, _, _, _, _, _, _, _, _, _) => {
                    h489
                }
            };
            let a492 = 0u64;
            nat_rem(a490, a491, a492)
        };
        let a494 = v9;
        nat_mul(a493, a494)?
    };
    let v11: u64 = {
        let a496 = match v4.clone() {
            Adt1::C0(_, h495, _, _, _, _, _, _, _, _, _) => {
                h495
            }
        };
        let a497 = v9;
        nat_mul(a496, a497)?
    };
    let a499 = v0;
    let a500 = match v1.clone() {
        Adt2::C0(h498, _, _, _, _, _, _, _, _, _, _) => {
            h498
        }
    };
    if nat_lt(a499, a500) {
        let a501 = v6;
        let a502 = 0u64;
        if nat_eq(a501, a502) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h503, _, _, _, _, _, _, _, _, _, _) => {
                    h503
                }
            }, 0u64, 0u64, {
                let a511 = {
                    let a507 = {
                        let a505 = v8;
                        let a506 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h504, _, _, _, _) => {
                                h504
                            }
                        };
                        nat_mul(a505, a506)?
                    };
                    let a508 = v10;
                    nat_add(a507, a508)?
                };
                let a512 = {
                    let a509 = v5;
                    let a510 = 100u64;
                    if nat_eq(a509, a510) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a511, a512)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a513 = v6;
            let a514 = 1u64;
            if nat_eq(a513, a514) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
                        h515
                    }
                }, 0u64, {
                    let a516 = v5;
                    let a517 = 98u64;
                    if nat_eq(a516, a517) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a525 = {
                        let a521 = {
                            let a519 = v8;
                            let a520 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h518, _, _, _, _) => {
                                    h518
                                }
                            };
                            nat_mul(a519, a520)?
                        };
                        let a522 = v10;
                        nat_add(a521, a522)?
                    };
                    let a526 = {
                        let a523 = v5;
                        let a524 = 99u64;
                        if nat_eq(a523, a524) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a525, a526)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a527 = v6;
                let a528 = 2u64;
                if nat_eq(a527, a528) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h529, _, _, _, _, _, _) => {
                            h529
                        }
                    }, match v4 {
                        Adt1::C0(h530, _, _, _, _, _, _, _, _, _, _) => {
                            h530
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h531, _, _, _, _, _, _, _, _, _, _) => {
                            h531
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a535 = {
                            let a533 = v8;
                            let a534 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h532, _, _, _, _) => {
                                    h532
                                }
                            };
                            nat_mul(a533, a534)?
                        };
                        let a536 = v10;
                        nat_add(a535, a536)?
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
        Adt2::C0(_, _, _, _, _, _, _, h537, _, _, _) => {
            h537
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h538, _, _) => {
            h538
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h539, _) => {
            h539
        }
    };
    let v6: u64 = {
        let a541 = match v1.clone() {
            Adt2::C0(_, h540, _, _, _, _, _, _, _, _, _) => {
                h540
            }
        };
        let a542 = 3u64;
        if nat_eq(a541, a542) {
            0u64
        } else {
            let a543 = f37(v3.clone())?;
            let a544 = 2u64;
            nat_mul(a543, a544)?
        }
    };
    let a546 = match v1.clone() {
        Adt2::C0(h545, _, _, _, _, _, _, _, _, _, _) => {
            h545
        }
    };
    let a547 = v0;
    if nat_le(a546, a547) {
        Ok(Adt3::C1)
    } else {
        let a548 = v0;
        let a549 = v6;
        if nat_lt(a548, a549) {
            let v7: u64 = {
                let a550 = v0;
                let a551 = 2u64;
                let a552 = 0u64;
                nat_quot(a550, a551, a552)
            };
            let v8: u64 = {
                let a557 = 3u64;
                let a558 = {
                    let a554 = f47(v7, v5.clone())?;
                    let a555 = f20(match v5 {
                        Adt1::C0(h553, _, _, _, _, _, _, _, _, _, _) => {
                            h553
                        }
                    });
                    let a556 = 0u64;
                    nat_quot(a554, a555, a556)
                };
                nat_add(a557, a558)?
            };
            let a562 = {
                let a559 = v0;
                let a560 = 2u64;
                let a561 = 0u64;
                nat_rem(a559, a560, a561)
            };
            let a563 = 0u64;
            if nat_eq(a562, a563) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a564 = v2;
                let a565 = 0u64;
                if nat_eq(a564, a565) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a566 = v0;
                let a567 = v6;
                nat_sub(a566, a567)
            };
            let v10: u64 = f38(match v3.clone() {
                Adt1::C0(h568, _, _, _, _, _, _, _, _, _, _) => {
                    h568
                }
            }, match v4.clone() {
                Adt1::C0(h569, _, _, _, _, _, _, _, _, _, _) => {
                    h569
                }
            }, match v5.clone() {
                Adt1::C0(h570, _, _, _, _, _, _, _, _, _, _) => {
                    h570
                }
            })?;
            let v11: u64 = {
                let a571 = v9;
                let a572 = v10;
                let a573 = 0u64;
                nat_quot(a571, a572, a573)
            };
            let v12: u64 = {
                let a574 = v9;
                let a575 = v10;
                let a576 = 0u64;
                nat_rem(a574, a575, a576)
            };
            let v13: u64 = {
                let a578 = match v3.clone() {
                    Adt1::C0(h577, _, _, _, _, _, _, _, _, _, _) => {
                        h577
                    }
                };
                let a579 = 0u64;
                if nat_eq(a578, a579) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a581 = match v4.clone() {
                    Adt1::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                        h580
                    }
                };
                let a582 = 0u64;
                if nat_eq(a581, a582) {
                    0u64
                } else {
                    1u64
                }
            };
            let a583 = v12;
            let a584 = v13;
            if nat_lt(a583, a584) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h585, _, _, _, _, _, _, _, _, _, _) => {
                        h585
                    }
                }, 0u64, 0u64, f47(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f20(match v3 {
                    Adt1::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                        h586
                    }
                }), 4u64))
            } else {
                let a589 = v12;
                let a590 = {
                    let a587 = v13;
                    let a588 = v14;
                    nat_add(a587, a588)?
                };
                if nat_lt(a589, a590) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h591, _, _, _, _, _, _, _, _, _, _) => {
                            h591
                        }
                    }, 0u64, 4u64, f48(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f20(match v4 {
                        Adt1::C0(h592, _, _, _, _, _, _, _, _, _, _) => {
                            h592
                        }
                    }), 4u64))
                } else {
                    let a595 = v12;
                    let a596 = {
                        let a593 = v13;
                        let a594 = v14;
                        nat_add(a593, a594)?
                    };
                    if nat_eq(a595, a596) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h597, _, _, _, _, _, _, _) => {
                                h597
                            }
                        }, 0u64, 0u64, {
                            let a598 = v13;
                            let a599 = 0u64;
                            if nat_eq(a598, a599) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a600 = v13;
                            let a601 = 0u64;
                            if nat_eq(a600, a601) {
                                f47(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a602 = v14;
                            let a603 = 0u64;
                            if nat_eq(a602, a603) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a604 = v14;
                            let a605 = 0u64;
                            if nat_eq(a604, a605) {
                                f48(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a607 = match v5.clone() {
                                Adt1::C0(h606, _, _, _, _, _, _, _, _, _, _) => {
                                    h606
                                }
                            };
                            let a608 = 0u64;
                            if nat_eq(a607, a608) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a610 = match v5.clone() {
                                Adt1::C0(h609, _, _, _, _, _, _, _, _, _, _) => {
                                    h609
                                }
                            };
                            let a611 = 0u64;
                            if nat_eq(a610, a611) {
                                f47(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h612, _, _, _, _, _, _, _, _, _, _) => {
                                h612
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f47(v11, v5.clone())?, 4u64, f20(match v5 {
                            Adt1::C0(h613, _, _, _, _, _, _, _, _, _, _) => {
                                h613
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> bool {
    let a614 = 6u64;
    let a615 = v0;
    if if if if nat_le(a614, a615) {
        let a616 = v0;
        let a617 = 23u64;
        nat_le(a616, a617)
    } else {
        false
    } {
        true
    } else {
        let a618 = v0;
        let a619 = 2u64;
        if nat_eq(a618, a619) {
            true
        } else {
            let a620 = v0;
            let a621 = 3u64;
            nat_eq(a620, a621)
        }
    } {
        true
    } else {
        let a622 = v0;
        let a623 = 29u64;
        nat_eq(a622, a623)
    } {
        true
    } else {
        let a624 = 34u64;
        let a625 = v0;
        if if nat_le(a624, a625) {
            let a626 = v0;
            let a627 = 35u64;
            nat_le(a626, a627)
        } else {
            false
        } {
            true
        } else {
            let a628 = 39u64;
            let a629 = v0;
            if nat_le(a628, a629) {
                let a630 = v0;
                let a631 = 42u64;
                nat_le(a630, a631)
            } else {
                false
            }
        }
    }
}

pub fn f19(v0: u64) -> u64 {
    let a632 = v0;
    let a633 = 0u64;
    if nat_eq(a632, a633) {
        4u64
    } else {
        let a634 = v0;
        let a635 = 1u64;
        if if nat_eq(a634, a635) {
            true
        } else {
            let a636 = v0;
            let a637 = 30u64;
            nat_eq(a636, a637)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f20(v0: u64) -> u64 {
    let a638 = v0;
    let a639 = 43u64;
    if nat_eq(a638, a639) {
        1u64
    } else {
        f19(v0)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64) -> bool {
    let a640 = v0;
    let a641 = 0u64;
    if if if nat_eq(a640, a641) {
        let a642 = v1;
        let a643 = 0u64;
        nat_eq(a642, a643)
    } else {
        false
    } {
        let a644 = v2;
        let a645 = 0u64;
        nat_eq(a644, a645)
    } else {
        false
    } {
        true
    } else {
        let a646 = v0;
        let a647 = 1u64;
        if if nat_eq(a646, a647) {
            true
        } else {
            let a648 = v0;
            let a649 = 30u64;
            nat_eq(a648, a649)
        } {
            let a650 = v1;
            let a651 = v0;
            if if nat_eq(a650, a651) {
                let a652 = v2;
                let a653 = v0;
                nat_eq(a652, a653)
            } else {
                false
            } {
                true
            } else {
                let a654 = v1;
                let a655 = 0u64;
                if nat_eq(a654, a655) {
                    let a656 = v2;
                    let a657 = v0;
                    if nat_eq(a656, a657) {
                        true
                    } else {
                        let a658 = v2;
                        let a659 = 0u64;
                        nat_eq(a658, a659)
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

pub fn f22(v0: Adt1, v1: Adt1) -> bool {
    if f30(v1.clone()) {
        f30(v0)
    } else {
        let a665 = {
            let a662 = match v0.clone() {
                Adt1::C0(_, h660, _, _, _, _, _, _, _, _, _) => {
                    h660
                }
            };
            let a663 = match v1.clone() {
                Adt1::C0(_, h661, _, _, _, _, _, _, _, _, _) => {
                    h661
                }
            };
            let a664 = 1u64;
            nat_rem(a662, a663, a664)
        };
        let a666 = 0u64;
        if if if nat_eq(a665, a666) {
            let a672 = {
                let a669 = match v0.clone() {
                    Adt1::C0(_, _, h667, _, _, _, _, _, _, _, _) => {
                        h667
                    }
                };
                let a670 = match v1.clone() {
                    Adt1::C0(_, _, h668, _, _, _, _, _, _, _, _) => {
                        h668
                    }
                };
                let a671 = 1u64;
                nat_rem(a669, a670, a671)
            };
            let a673 = 0u64;
            nat_eq(a672, a673)
        } else {
            false
        } {
            let a679 = {
                let a676 = match v0.clone() {
                    Adt1::C0(_, _, _, h674, _, _, _, _, _, _, _) => {
                        h674
                    }
                };
                let a677 = match v1.clone() {
                    Adt1::C0(_, _, _, h675, _, _, _, _, _, _, _) => {
                        h675
                    }
                };
                let a678 = 1u64;
                nat_rem(a676, a677, a678)
            };
            let a680 = 0u64;
            nat_eq(a679, a680)
        } else {
            false
        } {
            let a686 = {
                let a683 = match v0 {
                    Adt1::C0(_, _, _, _, h681, _, _, _, _, _, _) => {
                        h681
                    }
                };
                let a684 = match v1 {
                    Adt1::C0(_, _, _, _, h682, _, _, _, _, _, _) => {
                        h682
                    }
                };
                let a685 = 1u64;
                nat_rem(a683, a684, a685)
            };
            let a687 = 0u64;
            nat_eq(a686, a687)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    let a690 = match v0.clone() {
        Adt1::C0(_, h688, _, _, _, _, _, _, _, _, _) => {
            h688
        }
    };
    let a691 = match v1.clone() {
        Adt1::C0(_, h689, _, _, _, _, _, _, _, _, _) => {
            h689
        }
    };
    if if if nat_eq(a690, a691) {
        let a694 = match v0.clone() {
            Adt1::C0(_, _, h692, _, _, _, _, _, _, _, _) => {
                h692
            }
        };
        let a695 = match v1.clone() {
            Adt1::C0(_, _, h693, _, _, _, _, _, _, _, _) => {
                h693
            }
        };
        nat_eq(a694, a695)
    } else {
        false
    } {
        let a698 = match v0.clone() {
            Adt1::C0(_, _, _, h696, _, _, _, _, _, _, _) => {
                h696
            }
        };
        let a699 = match v1.clone() {
            Adt1::C0(_, _, _, h697, _, _, _, _, _, _, _) => {
                h697
            }
        };
        nat_eq(a698, a699)
    } else {
        false
    } {
        let a702 = match v0 {
            Adt1::C0(_, _, _, _, h700, _, _, _, _, _, _) => {
                h700
            }
        };
        let a703 = match v1 {
            Adt1::C0(_, _, _, _, h701, _, _, _, _, _, _) => {
                h701
            }
        };
        nat_eq(a702, a703)
    } else {
        false
    }
}

pub fn f24(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
            h704
        }
    };
    let a705 = v1;
    let a706 = v3;
    if if if nat_eq(a705, a706) {
        f18(v1)
    } else {
        false
    } {
        true
    } else {
        let a707 = v0;
        let a708 = 0u64;
        if nat_eq(a707, a708) {
            let a709 = v4;
            let a710 = 0u64;
            if if nat_eq(a709, a710) {
                true
            } else {
                f18(v3)
            } {
                true
            } else {
                let a711 = 0u64;
                let a712 = f20(v3);
                nat_lt(a711, a712)
            }
        } else {
            false
        }
    } {
        let a713 = 0u64;
        let a714 = f20(v4);
        if if if nat_lt(a713, a714) {
            true
        } else {
            if f18(v4) {
                let a715 = v4;
                let a716 = 9u64;
                !if nat_eq(a715, a716) {
                    true
                } else {
                    let a717 = v4;
                    let a718 = 15u64;
                    nat_eq(a717, a718)
                }
            } else {
                false
            }
        } {
            let a720 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h719, _, _, _, _, _) => {
                    h719
                }
            };
            let a721 = f26(v4);
            nat_eq(a720, a721)
        } else {
            false
        } {
            let a726 = {
                let a723 = match v2 {
                    Adt1::C0(_, h722, _, _, _, _, _, _, _, _, _) => {
                        h722
                    }
                };
                let a724 = f27(v4);
                let a725 = 1u64;
                nat_rem(a723, a724, a725)
            };
            let a727 = 0u64;
            nat_eq(a726, a727)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64) -> bool {
    if f18(v0) {
        let a728 = v0;
        let a729 = 9u64;
        !if nat_eq(a728, a729) {
            true
        } else {
            let a730 = v0;
            let a731 = 15u64;
            nat_eq(a730, a731)
        }
    } else {
        let a732 = f19(v0);
        let a733 = 2u64;
        if nat_eq(a732, a733) {
            if f18(v1) {
                true
            } else {
                let a734 = 0u64;
                let a735 = f20(v1);
                nat_lt(a734, a735)
            }
        } else {
            false
        }
    }
}

pub fn f26(v0: u64) -> u64 {
    if f18(v0) {
        let m736 = f55(v0);
        match m736 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h737) => {
                        h737
                    }
                }
            }
        }
    } else {
        let a738 = v0;
        let a739 = 26u64;
        if nat_eq(a738, a739) {
            4u64
        } else {
            f20(v0)
        }
    }
}

pub fn f27(v0: u64) -> u64 {
    if f18(v0) {
        let m740 = f55(v0);
        match m740 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h741, _) => {
                        h741
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f28(v0: Adt1, v1: Adt1) -> R<bool> {
    if f44(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f56(v0, v1))
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f57(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f57(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f57(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f57(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f57(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f57(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f30(v0: Adt1) -> bool {
    f58(match v0.clone() {
        Adt1::C0(_, h742, _, _, _, _, _, _, _, _, _) => {
            h742
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h743, _, _, _, _, _, _, _, _) => {
            h743
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h744, _, _, _, _, _, _, _) => {
            h744
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h745, _, _, _, _, _, _) => {
            h745
        }
    })
}

pub fn f31(v0: u64) -> Option<u64> {
    let a746 = v0;
    let a747 = 16u64;
    if if if nat_eq(a746, a747) {
        true
    } else {
        let a748 = v0;
        let a749 = 17u64;
        nat_eq(a748, a749)
    } {
        true
    } else {
        let a750 = v0;
        let a751 = 19u64;
        nat_eq(a750, a751)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f32(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a753 = match v1.clone() {
        Adt1::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
            h752
        }
    };
    let a754 = 43u64;
    if nat_eq(a753, a754) {
        let a755 = v0;
        let a756 = 0u64;
        if if if if if if nat_eq(a755, a756) {
            let a758 = match v2.clone() {
                Adt1::C0(h757, _, _, _, _, _, _, _, _, _, _) => {
                    h757
                }
            };
            let a759 = 0u64;
            nat_eq(a758, a759)
        } else {
            false
        } {
            let a761 = match v3.clone() {
                Adt1::C0(h760, _, _, _, _, _, _, _, _, _, _) => {
                    h760
                }
            };
            let a762 = 0u64;
            if nat_eq(a761, a762) {
                true
            } else {
                let a764 = match v3.clone() {
                    Adt1::C0(h763, _, _, _, _, _, _, _, _, _, _) => {
                        h763
                    }
                };
                let a765 = 43u64;
                nat_eq(a764, a765)
            }
        } else {
            false
        } {
            let a767 = match v3.clone() {
                Adt1::C0(h766, _, _, _, _, _, _, _, _, _, _) => {
                    h766
                }
            };
            let a768 = 43u64;
            if nat_eq(a767, a768) {
                f22(v1.clone(), v2.clone())
            } else {
                f23(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a770 = match v2 {
                Adt1::C0(_, _, _, _, _, h769, _, _, _, _, _) => {
                    h769
                }
            };
            let a771 = 4u64;
            nat_eq(a770, a771)
        } else {
            false
        } {
            let a774 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h772, _, _, _, _, _) => {
                    h772
                }
            };
            let a775 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h773, _, _, _, _) => {
                    h773
                }
            };
            if nat_le(a774, a775) {
                let a778 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h776, _, _, _, _) => {
                        h776
                    }
                };
                let a779 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h777, _, _, _) => {
                        h777
                    }
                };
                if nat_le(a778, a779) {
                    let a782 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h780, _, _, _) => {
                            h780
                        }
                    };
                    let a783 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h781, _, _) => {
                            h781
                        }
                    };
                    nat_le(a782, a783)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a791 = {
                let a789 = match v1.clone() {
                    Adt1::C0(_, _, h784, _, _, _, _, _, _, _, _) => {
                        h784
                    }
                };
                let a790 = {
                    let a787 = match v1.clone() {
                        Adt1::C0(_, _, _, h785, _, _, _, _, _, _, _) => {
                            h785
                        }
                    };
                    let a788 = match v1 {
                        Adt1::C0(_, _, _, _, h786, _, _, _, _, _, _) => {
                            h786
                        }
                    };
                    nat_mul(a787, a788)?
                };
                nat_mul(a789, a790)?
            };
            let a792 = 2147483647u64;
            Ok(nat_le(a791, a792))
        } else {
            Ok(false)
        }
    } else {
        if f22(v1.clone(), v2.clone()) {
            Ok(f21(match v1 {
                Adt1::C0(h793, _, _, _, _, _, _, _, _, _, _) => {
                    h793
                }
            }, match v2 {
                Adt1::C0(h794, _, _, _, _, _, _, _, _, _, _) => {
                    h794
                }
            }, match v3 {
                Adt1::C0(h795, _, _, _, _, _, _, _, _, _, _) => {
                    h795
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f33(v0: Adt1) -> bool {
    let v1: u64 = f20(match v0.clone() {
        Adt1::C0(h796, _, _, _, _, _, _, _, _, _, _) => {
            h796
        }
    });
    let a801 = {
        let a798 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h797) => {
                h797
            }
        };
        let a799 = v1;
        let a800 = 1u64;
        nat_rem(a798, a799, a800)
    };
    let a802 = 0u64;
    if if nat_eq(a801, a802) {
        f59(match v0.clone() {
            Adt1::C0(_, h803, _, _, _, _, _, _, _, _, _) => {
                h803
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h804, _, _, _, _, _) => {
                h804
            }
        }, v1)
    } else {
        false
    } {
        f60(match v0.clone() {
            Adt1::C0(_, _, h805, _, _, _, _, _, _, _, _) => {
                h805
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h806, _, _, _, _, _, _, _) => {
                h806
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h807, _, _, _, _, _, _) => {
                h807
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h808, _, _, _, _) => {
                h808
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h809, _, _, _) => {
                h809
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h810, _, _) => {
                h810
            }
        }, v1)
    } else {
        false
    }
}

pub fn f34(v0: Adt1, v1: Adt1) -> R<bool> {
    let a818 = {
        let a816 = match v0.clone() {
            Adt1::C0(_, _, h811, _, _, _, _, _, _, _, _) => {
                h811
            }
        };
        let a817 = {
            let a814 = match v0.clone() {
                Adt1::C0(_, _, _, h812, _, _, _, _, _, _, _) => {
                    h812
                }
            };
            let a815 = match v0.clone() {
                Adt1::C0(_, _, _, _, h813, _, _, _, _, _, _) => {
                    h813
                }
            };
            nat_mul(a814, a815)?
        };
        nat_mul(a816, a817)?
    };
    let a819 = 9223372036854775807u64;
    if nat_le(a818, a819) {
        if f41(v1.clone()) {
            let a821 = match v1 {
                Adt1::C0(_, h820, _, _, _, _, _, _, _, _, _) => {
                    h820
                }
            };
            let a822 = 2147483647u64;
            Ok(nat_le(a821, a822))
        } else {
            let a824 = match v0 {
                Adt1::C0(_, h823, _, _, _, _, _, _, _, _, _) => {
                    h823
                }
            };
            let a825 = 2147483647u64;
            Ok(nat_le(a824, a825))
        }
    } else {
        Ok(false)
    }
}

pub fn f35(v0: Adt1) -> R<bool> {
    f29({
        let a828 = match v0.clone() {
            Adt1::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                h826
            }
        };
        let a829 = f20(match v0.clone() {
            Adt1::C0(h827, _, _, _, _, _, _, _, _, _, _) => {
                h827
            }
        });
        nat_mul(a828, a829)?
    }, match v0.clone() {
        Adt1::C0(_, _, h830, _, _, _, _, _, _, _, _) => {
            h830
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h831, _, _, _, _, _, _, _) => {
            h831
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h832, _, _, _, _, _, _) => {
            h832
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h833, _, _, _, _) => {
            h833
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h834, _, _, _) => {
            h834
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h835, _, _) => {
            h835
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a842 = {
        let a836 = v0;
        let a837 = v2;
        nat_mul(a836, a837)?
    };
    let a843 = {
        let a838 = v1;
        let a839 = 0u64;
        if nat_eq(a838, a839) {
            0u64
        } else {
            let a840 = v0;
            let a841 = 2u64;
            nat_mul(a840, a841)?
        }
    };
    nat_add(a842, a843)
}

pub fn f37(v0: Adt1) -> R<u64> {
    let a852 = {
        let a846 = match v0.clone() {
            Adt1::C0(_, h844, _, _, _, _, _, _, _, _, _) => {
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
    let a853 = {
        let a850 = match v0.clone() {
            Adt1::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = match v0 {
            Adt1::C0(_, _, _, _, h849, _, _, _, _, _, _) => {
                h849
            }
        };
        nat_mul(a850, a851)?
    };
    nat_mul(a852, a853)
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a864 = {
        let a860 = {
            let a856 = 1u64;
            let a857 = {
                let a854 = v0;
                let a855 = 0u64;
                if nat_eq(a854, a855) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a856, a857)?
        };
        let a861 = {
            let a858 = v1;
            let a859 = 0u64;
            if nat_eq(a858, a859) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a860, a861)?
    };
    let a865 = {
        let a862 = v2;
        let a863 = 0u64;
        if nat_eq(a862, a863) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a864, a865)
}

pub fn f39(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f23(v0.clone(), v1.clone()) {
        let a868 = match v0.clone() {
            Adt1::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
                h866
            }
        };
        let a869 = match v1.clone() {
            Adt1::C0(h867, _, _, _, _, _, _, _, _, _, _) => {
                h867
            }
        };
        nat_eq(a868, a869)
    } else {
        false
    } {
        let a872 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h870, _, _, _, _, _) => {
                h870
            }
        };
        let a873 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h871, _, _, _, _, _) => {
                h871
            }
        };
        nat_eq(a872, a873)
    } else {
        false
    } {
        let a876 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h874, _, _, _, _) => {
                h874
            }
        };
        let a877 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h875, _, _, _, _) => {
                h875
            }
        };
        nat_eq(a876, a877)
    } else {
        false
    } {
        let a880 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h878, _, _, _) => {
                h878
            }
        };
        let a881 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h879, _, _, _) => {
                h879
            }
        };
        nat_eq(a880, a881)
    } else {
        false
    } {
        let a884 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h882, _, _) => {
                h882
            }
        };
        let a885 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h883, _, _) => {
                h883
            }
        };
        nat_eq(a884, a885)
    } else {
        false
    } {
        let a887 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h886) => {
                h886
            }
        };
        let a888 = 0u64;
        if if nat_eq(a887, a888) {
            true
        } else {
            let a890 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h889) => {
                    h889
                }
            };
            let a891 = 0u64;
            nat_eq(a890, a891)
        } {
            true
        } else {
            let a894 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h892) => {
                    h892
                }
            };
            let a895 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h893) => {
                    h893
                }
            };
            nat_eq(a894, a895)
        }
    } else {
        false
    }
}

pub fn f40(v0: u64) -> u64 {
    let a896 = v0;
    let a897 = 26u64;
    if nat_eq(a896, a897) {
        4u64
    } else {
        let a898 = v0;
        let a899 = 27u64;
        if nat_eq(a898, a899) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f41(v0: Adt1) -> bool {
    let a901 = match v0.clone() {
        Adt1::C0(_, h900, _, _, _, _, _, _, _, _, _) => {
            h900
        }
    };
    let a902 = 1u64;
    if nat_eq(a901, a902) {
        true
    } else {
        let a905 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h903, _, _, _, _, _) => {
                h903
            }
        };
        let a906 = f20(match v0 {
            Adt1::C0(h904, _, _, _, _, _, _, _, _, _, _) => {
                h904
            }
        });
        nat_eq(a905, a906)
    }
}

pub fn f42(v0: Adt1, v1: u64) -> R<u64> {
    let a912 = {
        let a908 = match v0.clone() {
            Adt1::C0(_, h907, _, _, _, _, _, _, _, _, _) => {
                h907
            }
        };
        let a909 = f27(v1);
        let a910 = 0u64;
        nat_quot(a908, a909, a910)
    };
    let a913 = f61(match v0 {
        Adt1::C0(h911, _, _, _, _, _, _, _, _, _, _) => {
            h911
        }
    }, v1);
    nat_mul(a912, a913)
}

pub fn f43(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a915 = match v0.clone() {
            Adt1::C0(_, h914, _, _, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a916 = 4u64;
        nat_mul(a915, a916)?
    };
    let v2: u64 = {
        let a919 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h917, _, _, _, _) => {
                h917
            }
        };
        let a920 = match v0.clone() {
            Adt1::C0(_, _, h918, _, _, _, _, _, _, _, _) => {
                h918
            }
        };
        nat_mul(a919, a920)?
    };
    let v3: u64 = {
        let a922 = v2;
        let a923 = match v0.clone() {
            Adt1::C0(_, _, _, h921, _, _, _, _, _, _, _) => {
                h921
            }
        };
        nat_mul(a922, a923)?
    };
    let a925 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h924, _, _, _, _, _) => {
            h924
        }
    };
    let a926 = 4u64;
    if if if if nat_eq(a925, a926) {
        let a931 = {
            let a928 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h927) => {
                    h927
                }
            };
            let a929 = 4u64;
            let a930 = 1u64;
            nat_rem(a928, a929, a930)
        };
        let a932 = 0u64;
        nat_eq(a931, a932)
    } else {
        false
    } {
        let a940 = {
            let a938 = match v0.clone() {
                Adt1::C0(_, _, h933, _, _, _, _, _, _, _, _) => {
                    h933
                }
            };
            let a939 = {
                let a936 = match v0.clone() {
                    Adt1::C0(_, _, _, h934, _, _, _, _, _, _, _) => {
                        h934
                    }
                };
                let a937 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h935, _, _, _, _, _, _) => {
                        h935
                    }
                };
                nat_mul(a936, a937)?
            };
            nat_mul(a938, a939)?
        };
        let a941 = 1u64;
        if nat_le(a940, a941) {
            true
        } else {
            let a943 = v1;
            let a944 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h942, _, _, _, _) => {
                    h942
                }
            };
            if nat_le(a943, a944) {
                let a949 = {
                    let a946 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h945, _, _, _, _) => {
                            h945
                        }
                    };
                    let a947 = 4u64;
                    let a948 = 1u64;
                    nat_rem(a946, a947, a948)
                };
                let a950 = 0u64;
                nat_eq(a949, a950)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a952 = match v0.clone() {
            Adt1::C0(_, _, _, h951, _, _, _, _, _, _, _) => {
                h951
            }
        };
        let a953 = 1u64;
        if nat_le(a952, a953) {
            true
        } else {
            let a955 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h954, _, _, _) => {
                    h954
                }
            };
            let a956 = v2;
            nat_eq(a955, a956)
        }
    } else {
        false
    } {
        let a958 = match v0.clone() {
            Adt1::C0(_, _, _, _, h957, _, _, _, _, _, _) => {
                h957
            }
        };
        let a959 = 1u64;
        if nat_le(a958, a959) {
            Ok(true)
        } else {
            let a961 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h960, _, _) => {
                    h960
                }
            };
            let a962 = v3;
            Ok(nat_eq(a961, a962))
        }
    } else {
        Ok(false)
    }
}

pub fn f44(v0: Adt1, v1: Adt1) -> R<bool> {
    let a964 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h963, _) => {
            h963
        }
    };
    let a965 = 0u64;
    if nat_eq(a964, a965) {
        Ok(true)
    } else {
        let a967 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h966, _) => {
                h966
            }
        };
        let a968 = 0u64;
        if nat_eq(a967, a968) {
            Ok(true)
        } else {
            let a970 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h969) => {
                    h969
                }
            };
            let a971 = 0u64;
            if nat_eq(a970, a971) {
                Ok(true)
            } else {
                let a973 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h972) => {
                        h972
                    }
                };
                let a974 = 0u64;
                if nat_eq(a973, a974) {
                    Ok(true)
                } else {
                    let a980 = {
                        let a977 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h975) => {
                                h975
                            }
                        };
                        let a978 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h976, _) => {
                                h976
                            }
                        };
                        nat_add(a977, a978)?
                    };
                    let a981 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h979) => {
                            h979
                        }
                    };
                    if nat_le(a980, a981) {
                        Ok(true)
                    } else {
                        let a987 = {
                            let a984 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h982) => {
                                    h982
                                }
                            };
                            let a985 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h983, _) => {
                                    h983
                                }
                            };
                            nat_add(a984, a985)?
                        };
                        let a988 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h986) => {
                                h986
                            }
                        };
                        Ok(nat_le(a987, a988))
                    }
                }
            }
        }
    }
}

pub fn f45(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a991 = match v0.clone() {
            Adt1::C0(_, _, h989, _, _, _, _, _, _, _, _) => {
                h989
            }
        };
        let a992 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h990, _, _, _, _) => {
                h990
            }
        };
        nat_mul(a991, a992)?
    };
    if if f41(v0.clone()) {
        let a994 = match v0.clone() {
            Adt1::C0(_, _, _, h993, _, _, _, _, _, _, _) => {
                h993
            }
        };
        let a995 = 1u64;
        if nat_eq(a994, a995) {
            true
        } else {
            let a997 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h996, _, _, _) => {
                    h996
                }
            };
            let a998 = v1;
            nat_eq(a997, a998)
        }
    } else {
        false
    } {
        let a1000 = match v0.clone() {
            Adt1::C0(_, _, _, _, h999, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1001 = 1u64;
        if nat_eq(a1000, a1001) {
            Ok(true)
        } else {
            let a1006 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1002, _, _) => {
                    h1002
                }
            };
            let a1007 = {
                let a1004 = v1;
                let a1005 = match v0 {
                    Adt1::C0(_, _, _, h1003, _, _, _, _, _, _, _) => {
                        h1003
                    }
                };
                nat_mul(a1004, a1005)?
            };
            Ok(nat_eq(a1006, a1007))
        }
    } else {
        Ok(false)
    }
}

pub fn f46(v0: u64) -> u64 {
    let a1008 = v0;
    let a1009 = 3u64;
    if nat_eq(a1008, a1009) {
        16u64
    } else {
        10u64
    }
}

pub fn f47(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1011 = v0;
        let a1012 = match v1.clone() {
            Adt1::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1013 = 0u64;
        nat_quot(a1011, a1012, a1013)
    };
    let v3: u64 = {
        let a1015 = v2;
        let a1016 = match v1.clone() {
            Adt1::C0(_, _, h1014, _, _, _, _, _, _, _, _) => {
                h1014
            }
        };
        let a1017 = 0u64;
        nat_quot(a1015, a1016, a1017)
    };
    f49({
        let a1019 = v0;
        let a1020 = match v1.clone() {
            Adt1::C0(_, h1018, _, _, _, _, _, _, _, _, _) => {
                h1018
            }
        };
        let a1021 = 0u64;
        nat_rem(a1019, a1020, a1021)
    }, {
        let a1023 = v2;
        let a1024 = match v1.clone() {
            Adt1::C0(_, _, h1022, _, _, _, _, _, _, _, _) => {
                h1022
            }
        };
        let a1025 = 0u64;
        nat_rem(a1023, a1024, a1025)
    }, {
        let a1027 = v3;
        let a1028 = match v1.clone() {
            Adt1::C0(_, _, _, h1026, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1029 = 0u64;
        nat_rem(a1027, a1028, a1029)
    }, {
        let a1031 = v3;
        let a1032 = match v1.clone() {
            Adt1::C0(_, _, _, h1030, _, _, _, _, _, _, _) => {
                h1030
            }
        };
        let a1033 = 0u64;
        nat_quot(a1031, a1032, a1033)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1034, _, _, _, _, _) => {
            h1034
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1035, _, _, _, _) => {
            h1035
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1036, _, _, _) => {
            h1036
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1037, _, _) => {
            h1037
        }
    })
}

pub fn f48(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1039 = v0;
        let a1040 = match v1.clone() {
            Adt1::C0(_, h1038, _, _, _, _, _, _, _, _, _) => {
                h1038
            }
        };
        let a1041 = 0u64;
        nat_quot(a1039, a1040, a1041)
    };
    let v4: u64 = {
        let a1043 = v3;
        let a1044 = match v1.clone() {
            Adt1::C0(_, _, h1042, _, _, _, _, _, _, _, _) => {
                h1042
            }
        };
        let a1045 = 0u64;
        nat_quot(a1043, a1044, a1045)
    };
    f49({
        let a1051 = {
            let a1047 = v0;
            let a1048 = match v1.clone() {
                Adt1::C0(_, h1046, _, _, _, _, _, _, _, _, _) => {
                    h1046
                }
            };
            let a1049 = 0u64;
            nat_rem(a1047, a1048, a1049)
        };
        let a1052 = match v2.clone() {
            Adt1::C0(_, h1050, _, _, _, _, _, _, _, _, _) => {
                h1050
            }
        };
        let a1053 = 0u64;
        nat_rem(a1051, a1052, a1053)
    }, {
        let a1059 = {
            let a1055 = v3;
            let a1056 = match v1.clone() {
                Adt1::C0(_, _, h1054, _, _, _, _, _, _, _, _) => {
                    h1054
                }
            };
            let a1057 = 0u64;
            nat_rem(a1055, a1056, a1057)
        };
        let a1060 = match v2.clone() {
            Adt1::C0(_, _, h1058, _, _, _, _, _, _, _, _) => {
                h1058
            }
        };
        let a1061 = 0u64;
        nat_rem(a1059, a1060, a1061)
    }, {
        let a1067 = {
            let a1063 = v4;
            let a1064 = match v1.clone() {
                Adt1::C0(_, _, _, h1062, _, _, _, _, _, _, _) => {
                    h1062
                }
            };
            let a1065 = 0u64;
            nat_rem(a1063, a1064, a1065)
        };
        let a1068 = match v2.clone() {
            Adt1::C0(_, _, _, h1066, _, _, _, _, _, _, _) => {
                h1066
            }
        };
        let a1069 = 0u64;
        nat_rem(a1067, a1068, a1069)
    }, {
        let a1075 = {
            let a1071 = v4;
            let a1072 = match v1 {
                Adt1::C0(_, _, _, h1070, _, _, _, _, _, _, _) => {
                    h1070
                }
            };
            let a1073 = 0u64;
            nat_quot(a1071, a1072, a1073)
        };
        let a1076 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1074, _, _, _, _, _, _) => {
                h1074
            }
        };
        let a1077 = 0u64;
        nat_rem(a1075, a1076, a1077)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1078, _, _, _, _, _) => {
            h1078
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1079, _, _, _, _) => {
            h1079
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1080, _, _, _) => {
            h1080
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1081, _, _) => {
            h1081
        }
    })
}

pub fn f49(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1094 = {
        let a1090 = {
            let a1086 = {
                let a1082 = v0;
                let a1083 = v4;
                nat_mul(a1082, a1083)?
            };
            let a1087 = {
                let a1084 = v1;
                let a1085 = v5;
                nat_mul(a1084, a1085)?
            };
            nat_add(a1086, a1087)?
        };
        let a1091 = {
            let a1088 = v2;
            let a1089 = v6;
            nat_mul(a1088, a1089)?
        };
        nat_add(a1090, a1091)?
    };
    let a1095 = {
        let a1092 = v3;
        let a1093 = v7;
        nat_mul(a1092, a1093)?
    };
    nat_add(a1094, a1095)
}

pub fn f50(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1096 = v0;
        let a1097 = 2u64;
        let a1098 = 0u64;
        nat_quot(a1096, a1097, a1098)
    };
    let v5: u64 = f40(match v1.clone() {
        Adt1::C0(h1099, _, _, _, _, _, _, _, _, _, _) => {
            h1099
        }
    });
    let a1103 = {
        let a1100 = v0;
        let a1101 = 2u64;
        let a1102 = 0u64;
        nat_rem(a1100, a1101, a1102)
    };
    let a1104 = 0u64;
    if nat_eq(a1103, a1104) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f47(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1105 = v3;
        let a1106 = v2;
        if if nat_lt(a1105, a1106) {
            let a1109 = v3;
            let a1110 = {
                let a1107 = v5;
                let a1108 = 4u64;
                if nat_eq(a1107, a1108) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1109, a1110)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1111 = 3u64;
                let a1112 = v4;
                nat_add(a1111, a1112)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f51(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1114 = 1u64;
        let a1115 = f42(v1.clone(), match v3.clone() {
            Adt1::C0(h1113, _, _, _, _, _, _, _, _, _, _) => {
                h1113
            }
        })?;
        nat_add(a1114, a1115)?
    };
    let v6: u64 = {
        let a1116 = v0;
        let a1117 = v5;
        let a1118 = 0u64;
        nat_rem(a1116, a1117, a1118)
    };
    let v7: u64 = {
        let a1119 = v0;
        let a1120 = v5;
        let a1121 = 0u64;
        nat_quot(a1119, a1120, a1121)
    };
    let v8: u64 = {
        let a1123 = v7;
        let a1124 = match v1.clone() {
            Adt1::C0(_, _, h1122, _, _, _, _, _, _, _, _) => {
                h1122
            }
        };
        let a1125 = 0u64;
        nat_rem(a1123, a1124, a1125)
    };
    let v9: u64 = {
        let a1127 = v7;
        let a1128 = match v1.clone() {
            Adt1::C0(_, _, h1126, _, _, _, _, _, _, _, _) => {
                h1126
            }
        };
        let a1129 = 0u64;
        nat_quot(a1127, a1128, a1129)
    };
    let v10: u64 = {
        let a1131 = v9;
        let a1132 = match v1.clone() {
            Adt1::C0(_, _, _, h1130, _, _, _, _, _, _, _) => {
                h1130
            }
        };
        let a1133 = 0u64;
        nat_rem(a1131, a1132, a1133)
    };
    let v11: u64 = {
        let a1135 = v9;
        let a1136 = match v1.clone() {
            Adt1::C0(_, _, _, h1134, _, _, _, _, _, _, _) => {
                h1134
            }
        };
        let a1137 = 0u64;
        nat_quot(a1135, a1136, a1137)
    };
    let a1138 = v6;
    let a1139 = 0u64;
    if nat_eq(a1138, a1139) {
        let v12: u64 = {
            let a1156 = v8;
            let a1157 = {
                let a1154 = match v2.clone() {
                    Adt1::C0(_, h1140, _, _, _, _, _, _, _, _, _) => {
                        h1140
                    }
                };
                let a1155 = {
                    let a1152 = {
                        let a1142 = v10;
                        let a1143 = match v2.clone() {
                            Adt1::C0(_, _, h1141, _, _, _, _, _, _, _, _) => {
                                h1141
                            }
                        };
                        let a1144 = 0u64;
                        nat_rem(a1142, a1143, a1144)
                    };
                    let a1153 = {
                        let a1150 = match v2.clone() {
                            Adt1::C0(_, _, h1145, _, _, _, _, _, _, _, _) => {
                                h1145
                            }
                        };
                        let a1151 = {
                            let a1147 = v11;
                            let a1148 = match v2 {
                                Adt1::C0(_, _, _, h1146, _, _, _, _, _, _, _) => {
                                    h1146
                                }
                            };
                            let a1149 = 0u64;
                            nat_rem(a1147, a1148, a1149)
                        };
                        nat_mul(a1150, a1151)?
                    };
                    nat_add(a1152, a1153)?
                };
                nat_mul(a1154, a1155)?
            };
            nat_add(a1156, a1157)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1158 = 3u64;
            let a1159 = v12;
            nat_add(a1158, a1159)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1161 = v4;
        let a1162 = match v3.clone() {
            Adt1::C0(_, _, h1160, _, _, _, _, _, _, _, _) => {
                h1160
            }
        };
        if nat_lt(a1161, a1162) {
            let v13: u64 = {
                let a1167 = {
                    let a1163 = v6;
                    let a1164 = 1u64;
                    nat_sub(a1163, a1164)
                };
                let a1168 = f61(match v1.clone() {
                    Adt1::C0(h1165, _, _, _, _, _, _, _, _, _, _) => {
                        h1165
                    }
                }, match v3.clone() {
                    Adt1::C0(h1166, _, _, _, _, _, _, _, _, _, _) => {
                        h1166
                    }
                });
                let a1169 = 0u64;
                nat_quot(a1167, a1168, a1169)
            };
            let v14: u64 = f49({
                let a1171 = v13;
                let a1172 = f27(match v3.clone() {
                    Adt1::C0(h1170, _, _, _, _, _, _, _, _, _, _) => {
                        h1170
                    }
                });
                nat_mul(a1171, a1172)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1173, _, _, _, _, _) => {
                    h1173
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1174, _, _, _, _) => {
                    h1174
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1175, _, _, _) => {
                    h1175
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1176, _, _) => {
                    h1176
                }
            })?;
            let v15: u64 = f49(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1177, _, _, _, _, _) => {
                    h1177
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1178, _, _, _, _) => {
                    h1178
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1179, _, _, _) => {
                    h1179
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1180, _, _) => {
                    h1180
                }
            })?;
            if if f18(match v3.clone() {
                Adt1::C0(h1181, _, _, _, _, _, _, _, _, _, _) => {
                    h1181
                }
            }) {
                true
            } else {
                let a1183 = match v3.clone() {
                    Adt1::C0(h1182, _, _, _, _, _, _, _, _, _, _) => {
                        h1182
                    }
                };
                let a1184 = 26u64;
                nat_eq(a1183, a1184)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f62(match v1 {
                    Adt1::C0(h1185, _, _, _, _, _, _, _, _, _, _) => {
                        h1185
                    }
                }, match v3 {
                    Adt1::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
                        h1186
                    }
                }, {
                    let a1187 = v6;
                    let a1188 = 1u64;
                    nat_sub(a1187, a1188)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f52(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f54(v0: u64, v1: u64) -> Adt3 {
    let a1189 = v0;
    let a1190 = 8u64;
    if nat_eq(a1189, a1190) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1191 = v0;
        let a1192 = 9u64;
        if nat_eq(a1191, a1192) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1193 = v0;
            let a1194 = 10u64;
            if nat_eq(a1193, a1194) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1195 = v0;
                let a1196 = 11u64;
                if nat_eq(a1195, a1196) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1197 = v0;
                    let a1198 = 12u64;
                    if nat_eq(a1197, a1198) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1199 = v0;
                        let a1200 = 13u64;
                        if nat_eq(a1199, a1200) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1201 = v0;
                            let a1202 = 14u64;
                            if nat_eq(a1201, a1202) {
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

pub fn f55(v0: u64) -> Adt4 {
    let a1203 = v0;
    let a1204 = 0u64;
    if nat_eq(a1203, a1204) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1205 = v0;
        let a1206 = 1u64;
        if nat_eq(a1205, a1206) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1207 = v0;
            let a1208 = 2u64;
            if nat_eq(a1207, a1208) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1209 = v0;
                let a1210 = 3u64;
                if nat_eq(a1209, a1210) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1211 = v0;
                    let a1212 = 6u64;
                    if nat_eq(a1211, a1212) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1213 = v0;
                        let a1214 = 7u64;
                        if nat_eq(a1213, a1214) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1215 = v0;
                            let a1216 = 8u64;
                            if nat_eq(a1215, a1216) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1217 = v0;
                                let a1218 = 9u64;
                                if nat_eq(a1217, a1218) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1219 = v0;
                                    let a1220 = 10u64;
                                    if nat_eq(a1219, a1220) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1221 = v0;
                                        let a1222 = 11u64;
                                        if nat_eq(a1221, a1222) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1223 = v0;
                                            let a1224 = 12u64;
                                            if nat_eq(a1223, a1224) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1225 = v0;
                                                let a1226 = 13u64;
                                                if nat_eq(a1225, a1226) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1227 = v0;
                                                    let a1228 = 14u64;
                                                    if nat_eq(a1227, a1228) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1229 = v0;
                                                        let a1230 = 15u64;
                                                        if nat_eq(a1229, a1230) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1231 = v0;
                                                            let a1232 = 16u64;
                                                            if nat_eq(a1231, a1232) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1233 = v0;
                                                                let a1234 = 17u64;
                                                                if nat_eq(a1233, a1234) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1235 = v0;
                                                                    let a1236 = 18u64;
                                                                    if nat_eq(a1235, a1236) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1237 = v0;
                                                                        let a1238 = 19u64;
                                                                        if nat_eq(a1237, a1238) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1239 = v0;
                                                                            let a1240 = 20u64;
                                                                            if nat_eq(a1239, a1240) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1241 = v0;
                                                                                let a1242 = 21u64;
                                                                                if nat_eq(a1241, a1242) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1243 = v0;
                                                                                    let a1244 = 22u64;
                                                                                    if nat_eq(a1243, a1244) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1245 = v0;
                                                                                        let a1246 = 23u64;
                                                                                        if nat_eq(a1245, a1246) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1247 = v0;
                                                                                            let a1248 = 24u64;
                                                                                            if nat_eq(a1247, a1248) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1249 = v0;
                                                                                                let a1250 = 25u64;
                                                                                                if nat_eq(a1249, a1250) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1251 = v0;
                                                                                                    let a1252 = 26u64;
                                                                                                    if nat_eq(a1251, a1252) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1253 = v0;
                                                                                                        let a1254 = 27u64;
                                                                                                        if nat_eq(a1253, a1254) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1255 = v0;
                                                                                                            let a1256 = 28u64;
                                                                                                            if nat_eq(a1255, a1256) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1257 = v0;
                                                                                                                let a1258 = 29u64;
                                                                                                                if nat_eq(a1257, a1258) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1259 = v0;
                                                                                                                    let a1260 = 30u64;
                                                                                                                    if nat_eq(a1259, a1260) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1261 = v0;
                                                                                                                        let a1262 = 34u64;
                                                                                                                        if nat_eq(a1261, a1262) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1263 = v0;
                                                                                                                            let a1264 = 35u64;
                                                                                                                            if nat_eq(a1263, a1264) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1265 = v0;
                                                                                                                                let a1266 = 39u64;
                                                                                                                                if nat_eq(a1265, a1266) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1267 = v0;
                                                                                                                                    let a1268 = 40u64;
                                                                                                                                    if nat_eq(a1267, a1268) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1269 = v0;
                                                                                                                                        let a1270 = 41u64;
                                                                                                                                        if nat_eq(a1269, a1270) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1271 = v0;
                                                                                                                                            let a1272 = 42u64;
                                                                                                                                            if nat_eq(a1271, a1272) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1273 = v0;
                                                                                                                                                let a1274 = 43u64;
                                                                                                                                                if nat_eq(a1273, a1274) {
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

pub fn f56(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f23(v0.clone(), v1.clone()) {
        let a1277 = match v0.clone() {
            Adt1::C0(h1275, _, _, _, _, _, _, _, _, _, _) => {
                h1275
            }
        };
        let a1278 = match v1.clone() {
            Adt1::C0(h1276, _, _, _, _, _, _, _, _, _, _) => {
                h1276
            }
        };
        nat_eq(a1277, a1278)
    } else {
        false
    } {
        let a1281 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1279) => {
                h1279
            }
        };
        let a1282 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1280) => {
                h1280
            }
        };
        nat_eq(a1281, a1282)
    } else {
        false
    } {
        let a1285 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1283, _, _, _, _, _) => {
                h1283
            }
        };
        let a1286 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1284, _, _, _, _, _) => {
                h1284
            }
        };
        nat_eq(a1285, a1286)
    } else {
        false
    } {
        let a1289 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1287, _, _, _, _) => {
                h1287
            }
        };
        let a1290 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1288, _, _, _, _) => {
                h1288
            }
        };
        nat_eq(a1289, a1290)
    } else {
        false
    } {
        let a1293 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1291, _, _, _) => {
                h1291
            }
        };
        let a1294 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1292, _, _, _) => {
                h1292
            }
        };
        nat_eq(a1293, a1294)
    } else {
        false
    } {
        let a1297 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1295, _, _) => {
                h1295
            }
        };
        let a1298 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1296, _, _) => {
                h1296
            }
        };
        nat_eq(a1297, a1298)
    } else {
        false
    }
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1299 = v1;
    let a1300 = 1u64;
    if if if nat_le(a1299, a1300) {
        true
    } else {
        let a1301 = v0;
        let a1302 = v2;
        nat_le(a1301, a1302)
    } {
        let a1303 = v3;
        let a1304 = 1u64;
        if nat_le(a1303, a1304) {
            true
        } else {
            let a1311 = {
                let a1309 = v0;
                let a1310 = {
                    let a1307 = {
                        let a1305 = v1;
                        let a1306 = 1u64;
                        nat_sub(a1305, a1306)
                    };
                    let a1308 = v2;
                    nat_mul(a1307, a1308)?
                };
                nat_add(a1309, a1310)?
            };
            let a1312 = v4;
            nat_le(a1311, a1312)
        }
    } else {
        false
    } {
        let a1313 = v5;
        let a1314 = 1u64;
        if nat_le(a1313, a1314) {
            Ok(true)
        } else {
            let a1327 = {
                let a1325 = {
                    let a1319 = v0;
                    let a1320 = {
                        let a1317 = {
                            let a1315 = v1;
                            let a1316 = 1u64;
                            nat_sub(a1315, a1316)
                        };
                        let a1318 = v2;
                        nat_mul(a1317, a1318)?
                    };
                    nat_add(a1319, a1320)?
                };
                let a1326 = {
                    let a1323 = {
                        let a1321 = v3;
                        let a1322 = 1u64;
                        nat_sub(a1321, a1322)
                    };
                    let a1324 = v4;
                    nat_mul(a1323, a1324)?
                };
                nat_add(a1325, a1326)?
            };
            let a1328 = v6;
            Ok(nat_le(a1327, a1328))
        }
    } else {
        Ok(false)
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1329 = v0;
    let a1330 = 0u64;
    if if if nat_eq(a1329, a1330) {
        true
    } else {
        let a1331 = v1;
        let a1332 = 0u64;
        nat_eq(a1331, a1332)
    } {
        true
    } else {
        let a1333 = v2;
        let a1334 = 0u64;
        nat_eq(a1333, a1334)
    } {
        true
    } else {
        let a1335 = v3;
        let a1336 = 0u64;
        nat_eq(a1335, a1336)
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64) -> bool {
    let a1337 = v0;
    let a1338 = 1u64;
    if nat_le(a1337, a1338) {
        true
    } else {
        let a1342 = {
            let a1339 = v1;
            let a1340 = v2;
            let a1341 = 1u64;
            nat_rem(a1339, a1340, a1341)
        };
        let a1343 = 0u64;
        nat_eq(a1342, a1343)
    }
}

pub fn f60(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f59(v0, v3, v6) {
        f59(v1, v4, v6)
    } else {
        false
    } {
        f59(v2, v5, v6)
    } else {
        false
    }
}

pub fn f61(v0: u64, v1: u64) -> u64 {
    let a1344 = v0;
    let a1345 = 1u64;
    if if nat_eq(a1344, a1345) {
        let a1346 = v1;
        let a1347 = 43u64;
        nat_eq(a1346, a1347)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1348 = f61(v0, v1);
    let a1349 = 2u64;
    if nat_eq(a1348, a1349) {
        let a1353 = {
            let a1350 = v2;
            let a1351 = 2u64;
            let a1352 = 0u64;
            nat_rem(a1350, a1351, a1352)
        };
        let a1354 = 0u64;
        if nat_eq(a1353, a1354) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f19(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f26(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f19(v0), f26(v1))
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
