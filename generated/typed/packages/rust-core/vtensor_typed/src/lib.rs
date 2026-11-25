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
    C4(Adt1, Adt1),
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
        Adt0::C1(v29, v30, v31) => {
            if match v30.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h34, _) => {
                    h34
                }
            } {
                f16(v29, v30)
            } else {
                let a36 = match v30.clone() {
                    Adt2::C0(_, _, _, h35, _, _, _, _, _, _, _, _) => {
                        h35
                    }
                };
                let a37 = 33u64;
                if if nat_eq(a36, a37) {
                    true
                } else {
                    let a39 = match v30.clone() {
                        Adt2::C0(_, _, _, h38, _, _, _, _, _, _, _, _) => {
                            h38
                        }
                    };
                    let a40 = 89u64;
                    nat_eq(a39, a40)
                } {
                    f17(v29, v30)
                } else {
                    let a42 = match v30.clone() {
                        Adt2::C0(_, _, _, h41, _, _, _, _, _, _, _, _) => {
                            h41
                        }
                    };
                    let a43 = 66u64;
                    if if nat_eq(a42, a43) {
                        true
                    } else {
                        let a45 = match v30.clone() {
                            Adt2::C0(_, _, _, h44, _, _, _, _, _, _, _, _) => {
                                h44
                            }
                        };
                        let a46 = 67u64;
                        nat_eq(a45, a46)
                    } {
                        f18(v29, v30)
                    } else {
                        let a48 = match v30.clone() {
                            Adt2::C0(_, _, _, h47, _, _, _, _, _, _, _, _) => {
                                h47
                            }
                        };
                        let a49 = 41u64;
                        if nat_eq(a48, a49) {
                            f19(v29, v30, v31)
                        } else {
                            let a51 = 98u64;
                            let a52 = match v30.clone() {
                                Adt2::C0(_, _, _, h50, _, _, _, _, _, _, _, _) => {
                                    h50
                                }
                            };
                            if nat_le(a51, a52) {
                                if f20(match v30.clone() {
                                    Adt2::C0(_, _, _, _, h53, _, _, _, _, _, _, _) => {
                                        h53
                                    }
                                }) {
                                    f21(v29, v30, v31)
                                } else {
                                    f22(v29, v30)
                                }
                            } else {
                                f23(v29, v30, v31)
                            }
                        }
                    }
                }
            }
        }
        Adt0::C2(v11, v12, v13, v14, v15, v16) => {
            let a21 = v11;
            let a22 = 41u64;
            if nat_eq(a21, a22) {
                if v12 {
                    Ok(Adt3::C1)
                } else {
                    f11(v13, v14, v15, v16)
                }
            } else {
                Ok(Adt3::C0)
            }
        }
        Adt0::C3(v17, v18, v19, v20, v21, v22, v23, v24, v25) => {
            let a23 = v17;
            let a24 = 33u64;
            if nat_eq(a23, a24) {
                if v22 {
                    Ok(Adt3::C1)
                } else {
                    let m25 = v24;
                    match m25 {
                        None => {
                            Ok(Adt3::C1)
                        }
                        Some(v26) => {
                            f12(v23, v26, v25)
                        }
                    }
                }
            } else {
                let a26 = v17;
                let a27 = 66u64;
                if if nat_eq(a26, a27) {
                    true
                } else {
                    let a28 = v17;
                    let a29 = 67u64;
                    nat_eq(a28, a29)
                } {
                    if v22 {
                        Ok(Adt3::C1)
                    } else {
                        let m30 = v24;
                        match m30 {
                            None => {
                                f13(v17, v18, v23, v25)
                            }
                            Some(_) => {
                                Ok(Adt3::C1)
                            }
                        }
                    }
                } else {
                    let a31 = v17;
                    let a32 = 98u64;
                    if nat_eq(a31, a32) {
                        if if f14(v18) {
                            !v22
                        } else {
                            false
                        } {
                            let m33 = v24;
                            match m33 {
                                None => {
                                    f15(v18, false, v19, v20, v21, v23.clone(), v23, v25)
                                }
                                Some(v28) => {
                                    f15(v18, true, v19, v20, v21, v23, v28, v25)
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
        Adt0::C4(v8, v9) => {
            let v10: u64 = match v8.clone() {
                Adt1::C0(_, h8, _, _, _, _, _, _, _, _, _) => {
                    h8
                }
            };
            if if if if if if f5(v8.clone(), v9.clone()) {
                let a11 = match v8.clone() {
                    Adt1::C0(h9, _, _, _, _, _, _, _, _, _, _) => {
                        h9
                    }
                };
                let a12 = match v9.clone() {
                    Adt1::C0(h10, _, _, _, _, _, _, _, _, _, _) => {
                        h10
                    }
                };
                nat_eq(a11, a12)
            } else {
                false
            } {
                f6(v8.clone())
            } else {
                false
            } {
                f6(v9.clone())
            } else {
                false
            } {
                let a15 = match v9.clone() {
                    Adt1::C0(_, _, _, _, _, h13, _, _, _, _, _) => {
                        h13
                    }
                };
                let a16 = f7(match v9.clone() {
                    Adt1::C0(h14, _, _, _, _, _, _, _, _, _, _) => {
                        h14
                    }
                });
                nat_eq(a15, a16)
            } else {
                false
            } {
                f8(v8.clone(), v9.clone())?
            } else {
                false
            } {
                Ok(Adt3::C2(Adt2::C0({
                    let a17 = f9(v8.clone())?;
                    let a18 = v10;
                    let a19 = 0u64;
                    nat_quot(a17, a18, a19)
                }, 3u64, f10(match v9.clone() {
                    Adt1::C0(h20, _, _, _, _, _, _, _, _, _, _) => {
                        h20
                    }
                }), 89u64, 0u64, 0u64, 0u64, v8.clone(), v8, v9, false, v10)))
            } else {
                Ok(Adt3::C1)
            }
        }
    }
}

pub fn f1(v0: u64) -> Option<u64> {
    let a54 = v0;
    let a55 = 2u64;
    if nat_eq(a54, a55) {
        Some(0u64)
    } else {
        let a56 = v0;
        let a57 = 5u64;
        if nat_eq(a56, a57) {
            Some(1u64)
        } else {
            let a58 = v0;
            let a59 = 6u64;
            if nat_eq(a58, a59) {
                Some(2u64)
            } else {
                let a60 = v0;
                let a61 = 7u64;
                if nat_eq(a60, a61) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    if if f24(v1) {
        true
    } else {
        f24(v3)
    } {
        true
    } else {
        if if f25(v1, v2, v3) {
            true
        } else {
            let a62 = v1;
            let a63 = 43u64;
            if if nat_eq(a62, a63) {
                let a64 = v0;
                let a65 = 0u64;
                nat_eq(a64, a65)
            } else {
                false
            } {
                let a66 = v2;
                let a67 = 0u64;
                nat_eq(a66, a67)
            } else {
                false
            }
        } {
            false
        } else {
            let a68 = v1;
            let a69 = v3;
            if if nat_eq(a68, a69) {
                f26(v0, v1, v2)
            } else {
                false
            } {
                true
            } else {
                let a70 = v0;
                let a71 = 0u64;
                if if nat_eq(a70, a71) {
                    let a72 = f27(v1);
                    let a73 = 2u64;
                    nat_eq(a72, a73)
                } else {
                    false
                } {
                    f28(v3)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a76 = match v1.clone() {
        Adt1::C0(_, h74, _, _, _, _, _, _, _, _, _) => {
            h74
        }
    };
    let a77 = match v2.clone() {
        Adt1::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
            h75
        }
    };
    if if if if if if if if if if if if if if nat_eq(a76, a77) {
        true
    } else {
        let a80 = match v1.clone() {
            Adt1::C0(h78, _, _, _, _, _, _, _, _, _, _) => {
                h78
            }
        };
        let a81 = match v3.clone() {
            Adt1::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                h79
            }
        };
        nat_eq(a80, a81)
    } {
        f29(v1.clone(), v2.clone())
    } else {
        false
    } {
        f5(v1.clone(), v3.clone())
    } else {
        false
    } {
        f30(v0, match v1.clone() {
            Adt1::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                h82
            }
        }, v2.clone(), match v3.clone() {
            Adt1::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                h83
            }
        })
    } else {
        false
    } {
        f31(match v1.clone() {
            Adt1::C0(h84, _, _, _, _, _, _, _, _, _, _) => {
                h84
            }
        }, match v3.clone() {
            Adt1::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        })
    } else {
        false
    } {
        let a87 = 0u64;
        let a88 = f7(match v3.clone() {
            Adt1::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                h86
            }
        });
        nat_lt(a87, a88)
    } else {
        false
    } {
        let a91 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h89, _, _, _, _, _) => {
                h89
            }
        };
        let a92 = f7(match v1.clone() {
            Adt1::C0(h90, _, _, _, _, _, _, _, _, _, _) => {
                h90
            }
        });
        nat_eq(a91, a92)
    } else {
        false
    } {
        let a95 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h93, _, _, _, _, _) => {
                h93
            }
        };
        let a96 = f7(match v3.clone() {
            Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                h94
            }
        });
        nat_eq(a95, a96)
    } else {
        false
    } {
        let a102 = {
            let a99 = match v1.clone() {
                Adt1::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                    h97
                }
            };
            let a100 = f32(match v1.clone() {
                Adt1::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                    h98
                }
            });
            let a101 = 1u64;
            nat_rem(a99, a100, a101)
        };
        let a103 = 0u64;
        nat_eq(a102, a103)
    } else {
        false
    } {
        let a109 = {
            let a106 = match v3.clone() {
                Adt1::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                    h104
                }
            };
            let a107 = f32(match v3.clone() {
                Adt1::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
                    h105
                }
            });
            let a108 = 1u64;
            nat_rem(a106, a107, a108)
        };
        let a110 = 0u64;
        nat_eq(a109, a110)
    } else {
        false
    } {
        f8(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f8(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f33({
            let a117 = {
                let a113 = match v3.clone() {
                    Adt1::C0(_, h111, _, _, _, _, _, _, _, _, _) => {
                        h111
                    }
                };
                let a114 = f32(match v3.clone() {
                    Adt1::C0(h112, _, _, _, _, _, _, _, _, _, _) => {
                        h112
                    }
                });
                let a115 = 0u64;
                nat_quot(a113, a114, a115)
            };
            let a118 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h116, _, _, _, _, _) => {
                    h116
                }
            };
            nat_mul(a117, a118)?
        }, match v3.clone() {
            Adt1::C0(_, _, h119, _, _, _, _, _, _, _, _) => {
                h119
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h120, _, _, _, _, _, _, _) => {
                h120
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h121, _, _, _, _, _, _) => {
                h121
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h122, _, _, _, _) => {
                h122
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h123, _, _, _) => {
                h123
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h124, _, _) => {
                h124
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f34(v1.clone()) {
            0u64
        } else {
            let a130 = match v1.clone() {
                Adt1::C0(_, _, h125, _, _, _, _, _, _, _, _) => {
                    h125
                }
            };
            let a131 = {
                let a128 = match v1.clone() {
                    Adt1::C0(_, _, _, h126, _, _, _, _, _, _, _) => {
                        h126
                    }
                };
                let a129 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h127, _, _, _, _, _, _) => {
                        h127
                    }
                };
                nat_mul(a128, a129)?
            };
            nat_mul(a130, a131)?
        }, 3u64, f10(match v3.clone() {
            Adt1::C0(h132, _, _, _, _, _, _, _, _, _, _) => {
                h132
            }
        }), v0, 0u64, 0u64, 0u64, v1.clone(), v2, v3, true, match v1 {
            Adt1::C0(_, h133, _, _, _, _, _, _, _, _, _) => {
                h133
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f5(v1.clone(), v3.clone()) {
        f35(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f34(v1.clone()) {
            true
        } else {
            let a136 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h134, _, _, _, _, _) => {
                    h134
                }
            };
            let a137 = f36(match v1.clone() {
                Adt1::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                    h135
                }
            });
            if if if if if if if nat_eq(a136, a137) {
                let a140 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h138, _, _, _, _, _) => {
                        h138
                    }
                };
                let a141 = f36(match v3.clone() {
                    Adt1::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
                        h139
                    }
                });
                nat_eq(a140, a141)
            } else {
                false
            } {
                f37(v1.clone())
            } else {
                false
            } {
                f37(v2.clone())
            } else {
                false
            } {
                f37(v3.clone())
            } else {
                false
            } {
                f38(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f8(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f8(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f34(v1.clone()) {
            true
        } else {
            f39(v3.clone())?
        } {
            0u64
        } else {
            let a144 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h142, _) => {
                    h142
                }
            };
            let a145 = f36(match v3.clone() {
                Adt1::C0(h143, _, _, _, _, _, _, _, _, _, _) => {
                    h143
                }
            });
            let a146 = 0u64;
            nat_quot(a144, a145, a146)
        };
        Ok(Adt3::C2(Adt2::C0(f40(f9(v1.clone())?, v4, f41(v0, match v1.clone() {
            Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _) => {
                h147
            }
        }, match v2.clone() {
            Adt1::C0(h148, _, _, _, _, _, _, _, _, _, _) => {
                h148
            }
        }, match v3.clone() {
            Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
                h149
            }
        })?)?, {
            let a150 = 3u64;
            let a151 = v4;
            nat_add(a150, a151)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1) -> bool {
    let a154 = match v0.clone() {
        Adt1::C0(_, h152, _, _, _, _, _, _, _, _, _) => {
            h152
        }
    };
    let a155 = match v1.clone() {
        Adt1::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
            h153
        }
    };
    if if if nat_eq(a154, a155) {
        let a158 = match v0.clone() {
            Adt1::C0(_, _, h156, _, _, _, _, _, _, _, _) => {
                h156
            }
        };
        let a159 = match v1.clone() {
            Adt1::C0(_, _, h157, _, _, _, _, _, _, _, _) => {
                h157
            }
        };
        nat_eq(a158, a159)
    } else {
        false
    } {
        let a162 = match v0.clone() {
            Adt1::C0(_, _, _, h160, _, _, _, _, _, _, _) => {
                h160
            }
        };
        let a163 = match v1.clone() {
            Adt1::C0(_, _, _, h161, _, _, _, _, _, _, _) => {
                h161
            }
        };
        nat_eq(a162, a163)
    } else {
        false
    } {
        let a166 = match v0 {
            Adt1::C0(_, _, _, _, h164, _, _, _, _, _, _) => {
                h164
            }
        };
        let a167 = match v1 {
            Adt1::C0(_, _, _, _, h165, _, _, _, _, _, _) => {
                h165
            }
        };
        nat_eq(a166, a167)
    } else {
        false
    }
}

pub fn f6(v0: Adt1) -> bool {
    let v1: u64 = match v0.clone() {
        Adt1::C0(h168, _, _, _, _, _, _, _, _, _, _) => {
            h168
        }
    };
    let a169 = 0u64;
    let a170 = f7(v1);
    if if nat_lt(a169, a170) {
        let a172 = f7(v1);
        let a173 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h171, _, _, _, _, _) => {
                h171
            }
        };
        nat_le(a172, a173)
    } else {
        false
    } {
        let a178 = {
            let a175 = match v0 {
                Adt1::C0(_, h174, _, _, _, _, _, _, _, _, _) => {
                    h174
                }
            };
            let a176 = f32(v1);
            let a177 = 1u64;
            nat_rem(a175, a176, a177)
        };
        let a179 = 0u64;
        nat_eq(a178, a179)
    } else {
        false
    }
}

pub fn f7(v0: u64) -> u64 {
    let m180 = f42(v0);
    match m180 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1(v1) => {
            match v1 {
                Adt5::C0(_, _, h181) => {
                    h181
                }
            }
        }
    }
}

pub fn f8(v0: Adt1, v1: Adt1) -> R<bool> {
    if f43(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f44(v0, v1))
    }
}

pub fn f9(v0: Adt1) -> R<u64> {
    let a190 = {
        let a184 = match v0.clone() {
            Adt1::C0(_, h182, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        let a185 = match v0.clone() {
            Adt1::C0(_, _, h183, _, _, _, _, _, _, _, _) => {
                h183
            }
        };
        nat_mul(a184, a185)?
    };
    let a191 = {
        let a188 = match v0.clone() {
            Adt1::C0(_, _, _, h186, _, _, _, _, _, _, _) => {
                h186
            }
        };
        let a189 = match v0 {
            Adt1::C0(_, _, _, _, h187, _, _, _, _, _, _) => {
                h187
            }
        };
        nat_mul(a188, a189)?
    };
    nat_mul(a190, a191)
}

pub fn f10(v0: u64) -> Option<u64> {
    let a192 = v0;
    let a193 = 16u64;
    if if if nat_eq(a192, a193) {
        true
    } else {
        let a194 = v0;
        let a195 = 17u64;
        nat_eq(a194, a195)
    } {
        true
    } else {
        let a196 = v0;
        let a197 = 19u64;
        nat_eq(a196, a197)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f11(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f45(v2, v3.clone()) {
        let a199 = match v0.clone() {
            Adt1::C0(h198, _, _, _, _, _, _, _, _, _, _) => {
                h198
            }
        };
        let a200 = 0u64;
        if nat_eq(a199, a200) {
            true
        } else {
            let a202 = match v0.clone() {
                Adt1::C0(h201, _, _, _, _, _, _, _, _, _, _) => {
                    h201
                }
            };
            let a203 = 1u64;
            nat_eq(a202, a203)
        }
    } else {
        false
    } {
        let a205 = 0u64;
        let a206 = f7(match v3.clone() {
            Adt1::C0(h204, _, _, _, _, _, _, _, _, _, _) => {
                h204
            }
        });
        nat_lt(a205, a206)
    } else {
        false
    } {
        let a208 = 0u64;
        let a209 = f46(match v1.clone() {
            Adt1::C0(h207, _, _, _, _, _, _, _, _, _, _) => {
                h207
            }
        });
        nat_lt(a208, a209)
    } else {
        false
    } {
        let a212 = match v0.clone() {
            Adt1::C0(_, h210, _, _, _, _, _, _, _, _, _) => {
                h210
            }
        };
        let a213 = match v3.clone() {
            Adt1::C0(_, h211, _, _, _, _, _, _, _, _, _) => {
                h211
            }
        };
        if nat_eq(a212, a213) {
            let a219 = {
                let a216 = match v0.clone() {
                    Adt1::C0(_, h214, _, _, _, _, _, _, _, _, _) => {
                        h214
                    }
                };
                let a217 = f32(match v3.clone() {
                    Adt1::C0(h215, _, _, _, _, _, _, _, _, _, _) => {
                        h215
                    }
                });
                let a218 = 1u64;
                nat_rem(a216, a217, a218)
            };
            let a220 = 0u64;
            nat_eq(a219, a220)
        } else {
            false
        }
    } else {
        false
    } {
        let a223 = match v0.clone() {
            Adt1::C0(_, _, _, h221, _, _, _, _, _, _, _) => {
                h221
            }
        };
        let a224 = match v3.clone() {
            Adt1::C0(_, _, _, h222, _, _, _, _, _, _, _) => {
                h222
            }
        };
        nat_eq(a223, a224)
    } else {
        false
    } {
        let a227 = match v0.clone() {
            Adt1::C0(_, _, _, _, h225, _, _, _, _, _, _) => {
                h225
            }
        };
        let a228 = match v3.clone() {
            Adt1::C0(_, _, _, _, h226, _, _, _, _, _, _) => {
                h226
            }
        };
        nat_eq(a227, a228)
    } else {
        false
    } {
        let a231 = match v0.clone() {
            Adt1::C0(_, _, h229, _, _, _, _, _, _, _, _) => {
                h229
            }
        };
        let a232 = match v1.clone() {
            Adt1::C0(_, h230, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a234 = match v1.clone() {
            Adt1::C0(_, _, _, _, h233, _, _, _, _, _, _) => {
                h233
            }
        };
        let a235 = 1u64;
        nat_eq(a234, a235)
    } else {
        false
    } {
        let a237 = 0u64;
        let a238 = match v1.clone() {
            Adt1::C0(_, _, h236, _, _, _, _, _, _, _, _) => {
                h236
            }
        };
        nat_lt(a237, a238)
    } else {
        false
    } {
        let a240 = 0u64;
        let a241 = match v1.clone() {
            Adt1::C0(_, _, _, h239, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_lt(a240, a241)
    } else {
        false
    } {
        let a247 = {
            let a244 = match v0.clone() {
                Adt1::C0(_, _, _, h242, _, _, _, _, _, _, _) => {
                    h242
                }
            };
            let a245 = match v1.clone() {
                Adt1::C0(_, _, h243, _, _, _, _, _, _, _, _) => {
                    h243
                }
            };
            let a246 = 0u64;
            nat_rem(a244, a245, a246)
        };
        let a248 = 0u64;
        nat_eq(a247, a248)
    } else {
        false
    } {
        let a254 = {
            let a251 = match v0.clone() {
                Adt1::C0(_, _, _, _, h249, _, _, _, _, _, _) => {
                    h249
                }
            };
            let a252 = match v1.clone() {
                Adt1::C0(_, _, _, h250, _, _, _, _, _, _, _) => {
                    h250
                }
            };
            let a253 = 0u64;
            nat_rem(a251, a252, a253)
        };
        let a255 = 0u64;
        nat_eq(a254, a255)
    } else {
        false
    } {
        f47(v0.clone())
    } else {
        false
    } {
        let a257 = match v3.clone() {
            Adt1::C0(_, h256, _, _, _, _, _, _, _, _, _) => {
                h256
            }
        };
        let a258 = 1u64;
        if nat_eq(a257, a258) {
            true
        } else {
            let a261 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h259, _, _, _, _, _) => {
                    h259
                }
            };
            let a262 = f7(match v3.clone() {
                Adt1::C0(h260, _, _, _, _, _, _, _, _, _, _) => {
                    h260
                }
            });
            nat_eq(a261, a262)
        }
    } else {
        false
    } {
        let v4: u64 = f9(v1.clone())?;
        let v5: u64 = {
            let a268 = match v0.clone() {
                Adt1::C0(_, _, h263, _, _, _, _, _, _, _, _) => {
                    h263
                }
            };
            let a269 = {
                let a266 = match v0.clone() {
                    Adt1::C0(_, _, _, h264, _, _, _, _, _, _, _) => {
                        h264
                    }
                };
                let a267 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h265, _, _, _, _, _, _) => {
                        h265
                    }
                };
                nat_mul(a266, a267)?
            };
            nat_mul(a268, a269)?
        };
        let v6: u64 = if f34(v0.clone()) {
            0u64
        } else {
            let a277 = {
                let a270 = 2u64;
                let a271 = v4;
                nat_mul(a270, a271)?
            };
            let a278 = {
                let a275 = v5;
                let a276 = {
                    let a273 = 1u64;
                    let a274 = f48(v0.clone(), match v3.clone() {
                        Adt1::C0(h272, _, _, _, _, _, _, _, _, _, _) => {
                            h272
                        }
                    })?;
                    nat_add(a273, a274)?
                };
                nat_mul(a275, a276)?
            };
            nat_add(a277, a278)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f34(v0.clone()) {
            3u64
        } else {
            let a279 = 3u64;
            let a280 = v4;
            nat_add(a279, a280)?
        }, f10(match v3.clone() {
            Adt1::C0(h281, _, _, _, _, _, _, _, _, _, _) => {
                h281
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3.clone(), false, f49(match v3 {
            Adt1::C0(h282, _, _, _, _, _, _, _, _, _, _) => {
                h282
            }
        }))))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f50(v0.clone(), v2.clone());
    let v4: u64 = f9(v0.clone())?;
    if if if if if f45(v1.clone(), v2.clone()) {
        let a283 = v4;
        let a284 = f9(v2.clone())?;
        nat_eq(a283, a284)
    } else {
        false
    } {
        f6(v0.clone())
    } else {
        false
    } {
        f6(v2.clone())
    } else {
        false
    } {
        if f24(match v0.clone() {
            Adt1::C0(h285, _, _, _, _, _, _, _, _, _, _) => {
                h285
            }
        }) {
            true
        } else {
            f24(match v2.clone() {
                Adt1::C0(h286, _, _, _, _, _, _, _, _, _, _) => {
                    h286
                }
            })
        }
    } else {
        false
    } {
        let v5: u64 = {
            let a287 = 0u64;
            let a288 = v3;
            if nat_lt(a287, a288) {
                let a289 = v4;
                let a290 = v3;
                let a291 = 0u64;
                nat_quot(a289, a290, a291)
            } else {
                let a299 = {
                    let a293 = v4;
                    let a294 = match v0.clone() {
                        Adt1::C0(_, h292, _, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    let a295 = 0u64;
                    nat_quot(a293, a294, a295)
                };
                let a300 = {
                    let a296 = v4;
                    let a297 = f51(v2.clone());
                    let a298 = 0u64;
                    nat_quot(a296, a297, a298)
                };
                nat_add(a299, a300)?
            }
        };
        Ok(Adt3::C2(Adt2::C0(v5, 3u64, f10(match v2.clone() {
            Adt1::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
                h301
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false, {
            let a302 = 0u64;
            let a303 = v3;
            if nat_lt(a302, a303) {
                v3
            } else {
                v4
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a305 = match v2.clone() {
        Adt1::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
            h304
        }
    };
    let a306 = 0u64;
    if if if if if if if if if if if nat_eq(a305, a306) {
        let a308 = match v3.clone() {
            Adt1::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
                h307
            }
        };
        let a309 = 26u64;
        nat_eq(a308, a309)
    } else {
        false
    } {
        let a311 = match v2.clone() {
            Adt1::C0(_, h310, _, _, _, _, _, _, _, _, _) => {
                h310
            }
        };
        let a312 = 2147483647u64;
        nat_le(a311, a312)
    } else {
        false
    } {
        let a315 = match v3.clone() {
            Adt1::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                h313
            }
        };
        let a316 = match v2.clone() {
            Adt1::C0(_, h314, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        };
        nat_le(a315, a316)
    } else {
        false
    } {
        let a319 = match v2.clone() {
            Adt1::C0(_, _, h317, _, _, _, _, _, _, _, _) => {
                h317
            }
        };
        let a320 = match v3.clone() {
            Adt1::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                h318
            }
        };
        nat_eq(a319, a320)
    } else {
        false
    } {
        let a323 = match v2.clone() {
            Adt1::C0(_, _, _, h321, _, _, _, _, _, _, _) => {
                h321
            }
        };
        let a324 = match v3.clone() {
            Adt1::C0(_, _, _, h322, _, _, _, _, _, _, _) => {
                h322
            }
        };
        nat_eq(a323, a324)
    } else {
        false
    } {
        let a327 = match v2.clone() {
            Adt1::C0(_, _, _, _, h325, _, _, _, _, _, _) => {
                h325
            }
        };
        let a328 = match v3.clone() {
            Adt1::C0(_, _, _, _, h326, _, _, _, _, _, _) => {
                h326
            }
        };
        nat_eq(a327, a328)
    } else {
        false
    } {
        let a329 = v0;
        let a330 = 67u64;
        if nat_eq(a329, a330) {
            true
        } else {
            let a331 = v1;
            let a332 = 1u64;
            if nat_le(a331, a332) {
                let a335 = match v3.clone() {
                    Adt1::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                        h333
                    }
                };
                let a336 = match v2.clone() {
                    Adt1::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                        h334
                    }
                };
                nat_eq(a335, a336)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f52(v2.clone())?
    } else {
        false
    } {
        f52(v3.clone())?
    } else {
        false
    } {
        f43(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a342 = match v3.clone() {
                Adt1::C0(_, _, h337, _, _, _, _, _, _, _, _) => {
                    h337
                }
            };
            let a343 = {
                let a340 = match v3.clone() {
                    Adt1::C0(_, _, _, h338, _, _, _, _, _, _, _) => {
                        h338
                    }
                };
                let a341 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h339, _, _, _, _, _, _) => {
                        h339
                    }
                };
                nat_mul(a340, a341)?
            };
            nat_mul(a342, a343)?
        }, 3u64, None::<u64>, v0, {
            let a344 = v0;
            let a345 = 67u64;
            if nat_eq(a344, a345) {
                1u64
            } else {
                v1
            }
        }, 0u64, 0u64, v2.clone(), v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a346 = v0;
    let a347 = 6u64;
    nat_le(a346, a347)
}

pub fn f15(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a348 = v2;
    let a349 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a348, a349) {
        let a351 = 0u64;
        let a352 = f27(match v5.clone() {
            Adt1::C0(h350, _, _, _, _, _, _, _, _, _, _) => {
                h350
            }
        });
        nat_lt(a351, a352)
    } else {
        false
    } {
        let a355 = match v5.clone() {
            Adt1::C0(h353, _, _, _, _, _, _, _, _, _, _) => {
                h353
            }
        };
        let a356 = match v6.clone() {
            Adt1::C0(h354, _, _, _, _, _, _, _, _, _, _) => {
                h354
            }
        };
        nat_eq(a355, a356)
    } else {
        false
    } {
        let a359 = match v5.clone() {
            Adt1::C0(h357, _, _, _, _, _, _, _, _, _, _) => {
                h357
            }
        };
        let a360 = match v7.clone() {
            Adt1::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                h358
            }
        };
        nat_eq(a359, a360)
    } else {
        false
    } {
        f5(v5.clone(), v6.clone())
    } else {
        false
    } {
        f53(v5.clone())?
    } else {
        false
    } {
        f53(v6.clone())?
    } else {
        false
    } {
        f53(v7.clone())?
    } else {
        false
    } {
        let a363 = match v5.clone() {
            Adt1::C0(_, _, h361, _, _, _, _, _, _, _, _) => {
                h361
            }
        };
        let a364 = match v7.clone() {
            Adt1::C0(_, _, h362, _, _, _, _, _, _, _, _) => {
                h362
            }
        };
        nat_eq(a363, a364)
    } else {
        false
    } {
        let a367 = match v5.clone() {
            Adt1::C0(_, _, _, h365, _, _, _, _, _, _, _) => {
                h365
            }
        };
        let a368 = match v7.clone() {
            Adt1::C0(_, _, _, h366, _, _, _, _, _, _, _) => {
                h366
            }
        };
        nat_eq(a367, a368)
    } else {
        false
    } {
        let a371 = match v5.clone() {
            Adt1::C0(_, _, _, _, h369, _, _, _, _, _, _) => {
                h369
            }
        };
        let a372 = match v7.clone() {
            Adt1::C0(_, _, _, _, h370, _, _, _, _, _, _) => {
                h370
            }
        };
        nat_eq(a371, a372)
    } else {
        false
    } {
        let a379 = match v7.clone() {
            Adt1::C0(_, h373, _, _, _, _, _, _, _, _, _) => {
                h373
            }
        };
        let a380 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h374, _, _, _, _, _, _, _, _, _) => {
                    h374
                }
            }
        } else {
            let a376 = match v5.clone() {
                Adt1::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                    h375
                }
            };
            let a377 = 2u64;
            let a378 = 0u64;
            nat_quot(a376, a377, a378)
        };
        nat_eq(a379, a380)
    } else {
        false
    } {
        let a382 = match v7.clone() {
            Adt1::C0(_, h381, _, _, _, _, _, _, _, _, _) => {
                h381
            }
        };
        let a383 = 2147483647u64;
        nat_le(a382, a383)
    } else {
        false
    } {
        f8(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f8(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f20(v0) {
            let a386 = 1u64;
            let a387 = {
                let a384 = f54(v0);
                let a385 = f9(v7.clone())?;
                nat_mul(a384, a385)?
            };
            nat_add(a386, a387)?
        } else {
            let a388 = 4u64;
            let a389 = f9(v7.clone())?;
            nat_mul(a388, a389)?
        }, if f20(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a390 = 99u64;
            let a391 = v2;
            nat_add(a390, a391)?
        }, v0, v3, v4, v5, v6, v7, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f16(v0: u64, v1: Adt2) -> R<Adt3> {
    let a393 = v0;
    let a394 = match v1.clone() {
        Adt2::C0(h392, _, _, _, _, _, _, _, _, _, _, _) => {
            h392
        }
    };
    if nat_lt(a393, a394) {
        let v2: u64 = {
            let a398 = v0;
            let a399 = {
                let m397 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h396, _, _, _, _) => {
                        h396
                    }
                };
                match m397 {
                    Adt1::C0(_, h395, _, _, _, _, _, _, _, _, _) => {
                        h395
                    }
                }
            };
            nat_mul(a398, a399)?
        };
        Ok(Adt3::C7(f55(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h400, _, _, _, _) => {
                h400
            }
        })?, f56(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h401, _, _, _, _) => {
                h401
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h402, _, _, _) => {
                h402
            }
        })?, f55(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h403, _, _) => {
                h403
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f17(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h404, _, _, _, _) => {
            h404
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h405, _, _) => {
            h405
        }
    };
    let a407 = v0;
    let a408 = match v1.clone() {
        Adt2::C0(h406, _, _, _, _, _, _, _, _, _, _, _) => {
            h406
        }
    };
    if nat_lt(a407, a408) {
        let v4: u64 = {
            let a410 = match v1.clone() {
                Adt2::C0(_, _, _, h409, _, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            let a411 = 89u64;
            if nat_eq(a410, a411) {
                match v2.clone() {
                    Adt1::C0(_, h412, _, _, _, _, _, _, _, _, _) => {
                        h412
                    }
                }
            } else {
                f50(v2.clone(), v3.clone())
            }
        };
        let a413 = 0u64;
        let a414 = v4;
        if nat_lt(a413, a414) {
            let v5: u64 = {
                let a415 = v0;
                let a416 = v4;
                nat_mul(a415, a416)?
            };
            Ok(Adt3::C6(f57(v5, v2)?, f57(v5, v3)?))
        } else {
            let v6: u64 = {
                let a419 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h417) => {
                        h417
                    }
                };
                let a420 = match v2.clone() {
                    Adt1::C0(_, h418, _, _, _, _, _, _, _, _, _) => {
                        h418
                    }
                };
                let a421 = 0u64;
                nat_quot(a419, a420, a421)
            };
            let a422 = v0;
            let a423 = v6;
            if nat_lt(a422, a423) {
                let v7: u64 = {
                    let a425 = v0;
                    let a426 = match v2.clone() {
                        Adt1::C0(_, h424, _, _, _, _, _, _, _, _, _) => {
                            h424
                        }
                    };
                    nat_mul(a425, a426)?
                };
                Ok(Adt3::C8(f57(v7, v2.clone())?, v7, match v2 {
                    Adt1::C0(_, h427, _, _, _, _, _, _, _, _, _) => {
                        h427
                    }
                }))
            } else {
                let v8: u64 = f51(v3.clone());
                let v9: u64 = {
                    let a430 = {
                        let a428 = v0;
                        let a429 = v6;
                        nat_sub(a428, a429)
                    };
                    let a431 = v8;
                    nat_mul(a430, a431)?
                };
                Ok(Adt3::C9(v9, f57(v9, v3)?, v8))
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f18(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h432, _, _, _, _) => {
            h432
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h433, _, _) => {
            h433
        }
    };
    let a435 = v0;
    let a436 = match v1.clone() {
        Adt2::C0(h434, _, _, _, _, _, _, _, _, _, _, _) => {
            h434
        }
    };
    if nat_lt(a435, a436) {
        Ok(Adt3::C5({
            let a438 = v0;
            let a439 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h437, _, _, _, _) => {
                    h437
                }
            };
            nat_mul(a438, a439)?
        }, {
            let a441 = v0;
            let a442 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h440, _, _, _, _) => {
                    h440
                }
            };
            nat_mul(a441, a442)?
        }, match v2 {
            Adt1::C0(_, h443, _, _, _, _, _, _, _, _, _) => {
                h443
            }
        }, match v3 {
            Adt1::C0(_, h444, _, _, _, _, _, _, _, _, _) => {
                h444
            }
        }, {
            let a446 = match v1 {
                Adt2::C0(_, _, _, _, h445, _, _, _, _, _, _, _) => {
                    h445
                }
            };
            let a447 = 1u64;
            nat_eq(a446, a447)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f19(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a449 = v0;
    let a450 = match v1.clone() {
        Adt2::C0(h448, _, _, _, _, _, _, _, _, _, _, _) => {
            h448
        }
    };
    if nat_lt(a449, a450) {
        let v3: u64 = {
            let a452 = 2u64;
            let a453 = f9(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h451, _, _, _) => {
                    h451
                }
            })?;
            nat_mul(a452, a453)?
        };
        let a454 = v0;
        let a455 = v3;
        if nat_lt(a454, a455) {
            f58(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h456, _, _, _) => {
                    h456
                }
            }, {
                let m459 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h458, _, _) => {
                        h458
                    }
                };
                match m459 {
                    Adt1::C0(_, _, h457, _, _, _, _, _, _, _, _) => {
                        h457
                    }
                }
            }, v2)
        } else {
            f59({
                let a460 = v0;
                let a461 = v3;
                nat_sub(a460, a461)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h462, _, _, _, _) => {
                    h462
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h463, _, _, _) => {
                    h463
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h464, _, _) => {
                    h464
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f20(v0: u64) -> bool {
    let a465 = v0;
    let a466 = 3u64;
    if nat_eq(a465, a466) {
        true
    } else {
        let a467 = v0;
        let a468 = 6u64;
        nat_eq(a467, a468)
    }
}

pub fn f21(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h469, _, _, _, _, _) => {
            h469
        }
    };
    let a471 = v0;
    let a472 = match v1.clone() {
        Adt2::C0(h470, _, _, _, _, _, _, _, _, _, _, _) => {
            h470
        }
    };
    if nat_lt(a471, a472) {
        let a473 = v0;
        let a474 = 0u64;
        if nat_eq(a473, a474) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a475 = v0;
                let a476 = 1u64;
                nat_sub(a475, a476)
            };
            let v5: u64 = {
                let a478 = v4;
                let a479 = f54(match v1.clone() {
                    Adt2::C0(_, _, _, _, h477, _, _, _, _, _, _, _) => {
                        h477
                    }
                });
                let a480 = 0u64;
                nat_rem(a478, a479, a480)
            };
            let v6: u64 = {
                let a485 = 4u64;
                let a486 = {
                    let a482 = v4;
                    let a483 = f54(match v1.clone() {
                        Adt2::C0(_, _, _, _, h481, _, _, _, _, _, _, _) => {
                            h481
                        }
                    });
                    let a484 = 0u64;
                    nat_quot(a482, a483, a484)
                };
                nat_mul(a485, a486)?
            };
            let a487 = v5;
            let a488 = 2u64;
            if nat_lt(a487, a488) {
                f22({
                    let a489 = v6;
                    let a490 = v5;
                    nat_add(a489, a490)?
                }, v1)
            } else {
                let a491 = v5;
                let a492 = 2u64;
                if nat_eq(a491, a492) {
                    Ok(f60(0u64, 2u64, v3))
                } else {
                    let a493 = v5;
                    let a494 = 3u64;
                    if nat_eq(a493, a494) {
                        Ok(f61(0u64, 2u64, v3, {
                            let a495 = v2;
                            let a496 = 2u64;
                            nat_eq(a495, a496)
                        }))
                    } else {
                        let a497 = v5;
                        let a498 = 4u64;
                        if nat_eq(a497, a498) {
                            Ok(f60(1u64, 1u64, 3u64))
                        } else {
                            let a499 = v5;
                            let a500 = 5u64;
                            if nat_eq(a499, a500) {
                                Ok(f61(1u64, 1u64, 3u64, {
                                    let a501 = v2;
                                    let a502 = 0u64;
                                    nat_eq(a501, a502)
                                }))
                            } else {
                                let a503 = v5;
                                let a504 = 6u64;
                                if nat_eq(a503, a504) {
                                    Ok(f60(1u64, 2u64, v3))
                                } else {
                                    let a505 = v5;
                                    let a506 = 7u64;
                                    if nat_eq(a505, a506) {
                                        Ok(f61(1u64, 2u64, v3, {
                                            let a507 = v2;
                                            let a508 = 2u64;
                                            nat_eq(a507, a508)
                                        }))
                                    } else {
                                        let a510 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h509, _, _, _, _, _, _, _) => {
                                                h509
                                            }
                                        };
                                        let a511 = 3u64;
                                        if nat_eq(a510, a511) {
                                            let a512 = v5;
                                            let a513 = 15u64;
                                            if nat_eq(a512, a513) {
                                                f22({
                                                    let a514 = v6;
                                                    let a515 = 3u64;
                                                    nat_add(a514, a515)?
                                                }, v1)
                                            } else {
                                                Ok(f62(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h516, _, _, _, _, _, _) => {
                                                        h516
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a517 = v5;
                                            let a518 = 8u64;
                                            if nat_eq(a517, a518) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m521 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h520, _, _) => {
                                                            h520
                                                        }
                                                    };
                                                    match m521 {
                                                        Adt1::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
                                                            h519
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f22({
                                                    let a522 = v6;
                                                    let a523 = 3u64;
                                                    nat_add(a522, a523)?
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

pub fn f22(v0: u64, v1: Adt2) -> R<Adt3> {
    let a525 = v0;
    let a526 = match v1.clone() {
        Adt2::C0(h524, _, _, _, _, _, _, _, _, _, _, _) => {
            h524
        }
    };
    if nat_lt(a525, a526) {
        f63({
            let a527 = v0;
            let a528 = 4u64;
            let a529 = 0u64;
            nat_rem(a527, a528, a529)
        }, {
            let a530 = v0;
            let a531 = 4u64;
            let a532 = 0u64;
            nat_quot(a530, a531, a532)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f23(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h533, _, _, _, _) => {
            h533
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h534, _, _, _) => {
            h534
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h535, _, _) => {
            h535
        }
    };
    let v6: u64 = {
        let a537 = match v1.clone() {
            Adt2::C0(_, h536, _, _, _, _, _, _, _, _, _, _) => {
                h536
            }
        };
        let a538 = 3u64;
        if nat_eq(a537, a538) {
            0u64
        } else {
            let a539 = f9(v3.clone())?;
            let a540 = 2u64;
            nat_mul(a539, a540)?
        }
    };
    let a542 = match v1.clone() {
        Adt2::C0(h541, _, _, _, _, _, _, _, _, _, _, _) => {
            h541
        }
    };
    let a543 = v0;
    if nat_le(a542, a543) {
        Ok(Adt3::C1)
    } else {
        let a544 = v0;
        let a545 = v6;
        if nat_lt(a544, a545) {
            let v7: u64 = {
                let a546 = v0;
                let a547 = 2u64;
                let a548 = 0u64;
                nat_quot(a546, a547, a548)
            };
            let v8: u64 = {
                let a553 = 3u64;
                let a554 = {
                    let a550 = f55(v7, v5.clone())?;
                    let a551 = f36(match v5 {
                        Adt1::C0(h549, _, _, _, _, _, _, _, _, _, _) => {
                            h549
                        }
                    });
                    let a552 = 0u64;
                    nat_quot(a550, a551, a552)
                };
                nat_add(a553, a554)?
            };
            let a558 = {
                let a555 = v0;
                let a556 = 2u64;
                let a557 = 0u64;
                nat_rem(a555, a556, a557)
            };
            let a559 = 0u64;
            if nat_eq(a558, a559) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a560 = v2;
                let a561 = 0u64;
                if nat_eq(a560, a561) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a562 = v0;
                let a563 = v6;
                nat_sub(a562, a563)
            };
            if f64(match v3.clone() {
                Adt1::C0(h564, _, _, _, _, _, _, _, _, _, _) => {
                    h564
                }
            }) {
                f65(v9, v1)
            } else {
                let v10: u64 = f66(match v3.clone() {
                    Adt1::C0(h565, _, _, _, _, _, _, _, _, _, _) => {
                        h565
                    }
                }, match v4.clone() {
                    Adt1::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
                        h566
                    }
                }, match v5.clone() {
                    Adt1::C0(h567, _, _, _, _, _, _, _, _, _, _) => {
                        h567
                    }
                })?;
                let v11: u64 = {
                    let a568 = v9;
                    let a569 = v10;
                    let a570 = 0u64;
                    nat_quot(a568, a569, a570)
                };
                let v12: u64 = {
                    let a571 = v9;
                    let a572 = v10;
                    let a573 = 0u64;
                    nat_rem(a571, a572, a573)
                };
                let v13: u64 = {
                    let a575 = match v3.clone() {
                        Adt1::C0(h574, _, _, _, _, _, _, _, _, _, _) => {
                            h574
                        }
                    };
                    let a576 = 0u64;
                    if nat_eq(a575, a576) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let v14: u64 = {
                    let a578 = match v4.clone() {
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
                let a580 = v12;
                let a581 = v13;
                if nat_lt(a580, a581) {
                    Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                        Adt1::C0(h582, _, _, _, _, _, _, _, _, _, _) => {
                            h582
                        }
                    }, 0u64, 0u64, f55(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f36(match v3 {
                        Adt1::C0(h583, _, _, _, _, _, _, _, _, _, _) => {
                            h583
                        }
                    }), 4u64))
                } else {
                    let a586 = v12;
                    let a587 = {
                        let a584 = v13;
                        let a585 = v14;
                        nat_add(a584, a585)?
                    };
                    if nat_lt(a586, a587) {
                        Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                            Adt1::C0(h588, _, _, _, _, _, _, _, _, _, _) => {
                                h588
                            }
                        }, 0u64, 4u64, f56(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f36(match v4 {
                            Adt1::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
                                h589
                            }
                        }), 4u64))
                    } else {
                        let a592 = v12;
                        let a593 = {
                            let a590 = v13;
                            let a591 = v14;
                            nat_add(a590, a591)?
                        };
                        if nat_eq(a592, a593) {
                            Ok(Adt3::C3(1u64, match v1 {
                                Adt2::C0(_, _, _, h594, _, _, _, _, _, _, _, _) => {
                                    h594
                                }
                            }, 0u64, 0u64, {
                                let a595 = v13;
                                let a596 = 0u64;
                                if nat_eq(a595, a596) {
                                    0u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a597 = v13;
                                let a598 = 0u64;
                                if nat_eq(a597, a598) {
                                    f55(v11, v3.clone())?
                                } else {
                                    0u64
                                }
                            }, {
                                let a599 = v14;
                                let a600 = 0u64;
                                if nat_eq(a599, a600) {
                                    4u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a601 = v14;
                                let a602 = 0u64;
                                if nat_eq(a601, a602) {
                                    f56(v11, v3, v4)?
                                } else {
                                    1u64
                                }
                            }, {
                                let a604 = match v5.clone() {
                                    Adt1::C0(h603, _, _, _, _, _, _, _, _, _, _) => {
                                        h603
                                    }
                                };
                                let a605 = 0u64;
                                if nat_eq(a604, a605) {
                                    3u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a607 = match v5.clone() {
                                    Adt1::C0(h606, _, _, _, _, _, _, _, _, _, _) => {
                                        h606
                                    }
                                };
                                let a608 = 0u64;
                                if nat_eq(a607, a608) {
                                    f55(v11, v5)?
                                } else {
                                    0u64
                                }
                            }, 4u64, 4u64))
                        } else {
                            Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                                Adt1::C0(h609, _, _, _, _, _, _, _, _, _, _) => {
                                    h609
                                }
                            }, 1u64, 0u64, 2u64, 0u64, 3u64, f55(v11, v5.clone())?, 4u64, f36(match v5 {
                                Adt1::C0(h610, _, _, _, _, _, _, _, _, _, _) => {
                                    h610
                                }
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> bool {
    let a611 = 6u64;
    let a612 = v0;
    if if if if nat_le(a611, a612) {
        let a613 = v0;
        let a614 = 23u64;
        nat_le(a613, a614)
    } else {
        false
    } {
        true
    } else {
        let a615 = v0;
        let a616 = 2u64;
        if nat_eq(a615, a616) {
            true
        } else {
            let a617 = v0;
            let a618 = 3u64;
            nat_eq(a617, a618)
        }
    } {
        true
    } else {
        let a619 = v0;
        let a620 = 29u64;
        nat_eq(a619, a620)
    } {
        true
    } else {
        let a621 = 34u64;
        let a622 = v0;
        if if nat_le(a621, a622) {
            let a623 = v0;
            let a624 = 35u64;
            nat_le(a623, a624)
        } else {
            false
        } {
            true
        } else {
            let a625 = 39u64;
            let a626 = v0;
            if nat_le(a625, a626) {
                let a627 = v0;
                let a628 = 42u64;
                nat_le(a627, a628)
            } else {
                false
            }
        }
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> bool {
    let a629 = v0;
    let a630 = 0u64;
    if if if nat_eq(a629, a630) {
        let a631 = v1;
        let a632 = 0u64;
        nat_eq(a631, a632)
    } else {
        false
    } {
        let a633 = v2;
        let a634 = 0u64;
        nat_eq(a633, a634)
    } else {
        false
    } {
        true
    } else {
        let a635 = v0;
        let a636 = 1u64;
        if if nat_eq(a635, a636) {
            true
        } else {
            let a637 = v0;
            let a638 = 30u64;
            nat_eq(a637, a638)
        } {
            let a639 = v1;
            let a640 = v0;
            if if nat_eq(a639, a640) {
                let a641 = v2;
                let a642 = v0;
                nat_eq(a641, a642)
            } else {
                false
            } {
                true
            } else {
                let a643 = v1;
                let a644 = 0u64;
                if nat_eq(a643, a644) {
                    let a645 = v2;
                    let a646 = v0;
                    if nat_eq(a645, a646) {
                        true
                    } else {
                        let a647 = v2;
                        let a648 = 0u64;
                        nat_eq(a647, a648)
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

pub fn f26(v0: u64, v1: u64, v2: u64) -> bool {
    let a649 = v1;
    let a650 = 27u64;
    if if if nat_eq(a649, a650) {
        true
    } else {
        let a651 = v1;
        let a652 = 28u64;
        nat_eq(a651, a652)
    } {
        let a653 = v1;
        let a654 = v2;
        nat_eq(a653, a654)
    } else {
        false
    } {
        let a655 = v0;
        let a656 = 3u64;
        nat_eq(a655, a656)
    } else {
        f28(v1)
    }
}

pub fn f27(v0: u64) -> u64 {
    let a657 = v0;
    let a658 = 0u64;
    if nat_eq(a657, a658) {
        4u64
    } else {
        let a659 = v0;
        let a660 = 1u64;
        if if nat_eq(a659, a660) {
            true
        } else {
            let a661 = v0;
            let a662 = 30u64;
            nat_eq(a661, a662)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f28(v0: u64) -> bool {
    let a663 = 0u64;
    let a664 = f67(v0);
    if nat_lt(a663, a664) {
        true
    } else {
        let a665 = 24u64;
        let a666 = v0;
        if nat_le(a665, a666) {
            let a667 = v0;
            let a668 = 28u64;
            nat_le(a667, a668)
        } else {
            false
        }
    }
}

pub fn f29(v0: Adt1, v1: Adt1) -> bool {
    if f34(v1.clone()) {
        f34(v0)
    } else {
        let a674 = {
            let a671 = match v0.clone() {
                Adt1::C0(_, h669, _, _, _, _, _, _, _, _, _) => {
                    h669
                }
            };
            let a672 = match v1.clone() {
                Adt1::C0(_, h670, _, _, _, _, _, _, _, _, _) => {
                    h670
                }
            };
            let a673 = 1u64;
            nat_rem(a671, a672, a673)
        };
        let a675 = 0u64;
        if if if nat_eq(a674, a675) {
            let a681 = {
                let a678 = match v0.clone() {
                    Adt1::C0(_, _, h676, _, _, _, _, _, _, _, _) => {
                        h676
                    }
                };
                let a679 = match v1.clone() {
                    Adt1::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                        h677
                    }
                };
                let a680 = 1u64;
                nat_rem(a678, a679, a680)
            };
            let a682 = 0u64;
            nat_eq(a681, a682)
        } else {
            false
        } {
            let a688 = {
                let a685 = match v0.clone() {
                    Adt1::C0(_, _, _, h683, _, _, _, _, _, _, _) => {
                        h683
                    }
                };
                let a686 = match v1.clone() {
                    Adt1::C0(_, _, _, h684, _, _, _, _, _, _, _) => {
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
                let a692 = match v0 {
                    Adt1::C0(_, _, _, _, h690, _, _, _, _, _, _) => {
                        h690
                    }
                };
                let a693 = match v1 {
                    Adt1::C0(_, _, _, _, h691, _, _, _, _, _, _) => {
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
        }
    }
}

pub fn f30(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h697, _, _, _, _, _, _, _, _, _, _) => {
            h697
        }
    };
    let a698 = v1;
    let a699 = v3;
    if if nat_eq(a698, a699) {
        true
    } else {
        let a700 = v0;
        let a701 = 0u64;
        if nat_eq(a700, a701) {
            let a702 = v4;
            let a703 = 0u64;
            if if nat_eq(a702, a703) {
                true
            } else {
                f24(v3)
            } {
                true
            } else {
                f28(v3)
            }
        } else {
            false
        }
    } {
        if if if f28(v4) {
            true
        } else {
            f24(v4)
        } {
            let a705 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h704, _, _, _, _, _) => {
                    h704
                }
            };
            let a706 = f7(v4);
            nat_eq(a705, a706)
        } else {
            false
        } {
            let a711 = {
                let a708 = match v2 {
                    Adt1::C0(_, h707, _, _, _, _, _, _, _, _, _) => {
                        h707
                    }
                };
                let a709 = f32(v4);
                let a710 = 1u64;
                nat_rem(a708, a709, a710)
            };
            let a712 = 0u64;
            nat_eq(a711, a712)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f31(v0: u64, v1: u64) -> bool {
    if f24(v0) {
        true
    } else {
        let a713 = v0;
        let a714 = v1;
        if if nat_eq(a713, a714) {
            f28(v0)
        } else {
            false
        } {
            true
        } else {
            let a715 = f27(v0);
            let a716 = 2u64;
            if nat_eq(a715, a716) {
                if f24(v1) {
                    true
                } else {
                    f28(v1)
                }
            } else {
                false
            }
        }
    }
}

pub fn f32(v0: u64) -> u64 {
    if f24(v0) {
        let m717 = f42(v0);
        match m717 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h718, _) => {
                        h718
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
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

pub fn f34(v0: Adt1) -> bool {
    f69(match v0.clone() {
        Adt1::C0(_, h719, _, _, _, _, _, _, _, _, _) => {
            h719
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h720, _, _, _, _, _, _, _, _) => {
            h720
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h721, _, _, _, _, _, _, _) => {
            h721
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h722, _, _, _, _, _, _) => {
            h722
        }
    })
}

pub fn f35(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a724 = match v1.clone() {
        Adt1::C0(h723, _, _, _, _, _, _, _, _, _, _) => {
            h723
        }
    };
    let a725 = 43u64;
    if nat_eq(a724, a725) {
        let a726 = v0;
        let a727 = 0u64;
        if if if if if if nat_eq(a726, a727) {
            let a729 = match v2.clone() {
                Adt1::C0(h728, _, _, _, _, _, _, _, _, _, _) => {
                    h728
                }
            };
            let a730 = 0u64;
            nat_eq(a729, a730)
        } else {
            false
        } {
            let a732 = match v3.clone() {
                Adt1::C0(h731, _, _, _, _, _, _, _, _, _, _) => {
                    h731
                }
            };
            let a733 = 0u64;
            if nat_eq(a732, a733) {
                true
            } else {
                let a735 = match v3.clone() {
                    Adt1::C0(h734, _, _, _, _, _, _, _, _, _, _) => {
                        h734
                    }
                };
                let a736 = 43u64;
                nat_eq(a735, a736)
            }
        } else {
            false
        } {
            let a738 = match v3.clone() {
                Adt1::C0(h737, _, _, _, _, _, _, _, _, _, _) => {
                    h737
                }
            };
            let a739 = 43u64;
            if nat_eq(a738, a739) {
                f29(v1.clone(), v2.clone())
            } else {
                f5(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a741 = match v2 {
                Adt1::C0(_, _, _, _, _, h740, _, _, _, _, _) => {
                    h740
                }
            };
            let a742 = 4u64;
            nat_eq(a741, a742)
        } else {
            false
        } {
            let a745 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h743, _, _, _, _, _) => {
                    h743
                }
            };
            let a746 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h744, _, _, _, _) => {
                    h744
                }
            };
            if nat_le(a745, a746) {
                let a749 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h747, _, _, _, _) => {
                        h747
                    }
                };
                let a750 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h748, _, _, _) => {
                        h748
                    }
                };
                if nat_le(a749, a750) {
                    let a753 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h751, _, _, _) => {
                            h751
                        }
                    };
                    let a754 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h752, _, _) => {
                            h752
                        }
                    };
                    nat_le(a753, a754)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a762 = {
                let a760 = match v1.clone() {
                    Adt1::C0(_, _, h755, _, _, _, _, _, _, _, _) => {
                        h755
                    }
                };
                let a761 = {
                    let a758 = match v1.clone() {
                        Adt1::C0(_, _, _, h756, _, _, _, _, _, _, _) => {
                            h756
                        }
                    };
                    let a759 = match v1 {
                        Adt1::C0(_, _, _, _, h757, _, _, _, _, _, _) => {
                            h757
                        }
                    };
                    nat_mul(a758, a759)?
                };
                nat_mul(a760, a761)?
            };
            let a763 = 2147483647u64;
            Ok(nat_le(a762, a763))
        } else {
            Ok(false)
        }
    } else {
        if f29(v1.clone(), v2.clone()) {
            if f25(match v1.clone() {
                Adt1::C0(h764, _, _, _, _, _, _, _, _, _, _) => {
                    h764
                }
            }, match v2.clone() {
                Adt1::C0(h765, _, _, _, _, _, _, _, _, _, _) => {
                    h765
                }
            }, match v3.clone() {
                Adt1::C0(h766, _, _, _, _, _, _, _, _, _, _) => {
                    h766
                }
            }) {
                Ok(true)
            } else {
                let a767 = v0;
                let a768 = 3u64;
                if if if if nat_lt(a767, a768) {
                    f64(match v1.clone() {
                        Adt1::C0(h769, _, _, _, _, _, _, _, _, _, _) => {
                            h769
                        }
                    })
                } else {
                    false
                } {
                    let a772 = match v1.clone() {
                        Adt1::C0(h770, _, _, _, _, _, _, _, _, _, _) => {
                            h770
                        }
                    };
                    let a773 = match v2.clone() {
                        Adt1::C0(h771, _, _, _, _, _, _, _, _, _, _) => {
                            h771
                        }
                    };
                    nat_eq(a772, a773)
                } else {
                    false
                } {
                    let a776 = match v1 {
                        Adt1::C0(h774, _, _, _, _, _, _, _, _, _, _) => {
                            h774
                        }
                    };
                    let a777 = match v3 {
                        Adt1::C0(h775, _, _, _, _, _, _, _, _, _, _) => {
                            h775
                        }
                    };
                    nat_eq(a776, a777)
                } else {
                    false
                } {
                    let a779 = match v2 {
                        Adt1::C0(_, _, _, _, _, h778, _, _, _, _, _) => {
                            h778
                        }
                    };
                    let a780 = 8u64;
                    Ok(nat_eq(a779, a780))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f36(v0: u64) -> u64 {
    if f64(v0) {
        8u64
    } else {
        f67(v0)
    }
}

pub fn f37(v0: Adt1) -> bool {
    let v1: u64 = f36(match v0.clone() {
        Adt1::C0(h781, _, _, _, _, _, _, _, _, _, _) => {
            h781
        }
    });
    let a786 = {
        let a783 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h782) => {
                h782
            }
        };
        let a784 = v1;
        let a785 = 1u64;
        nat_rem(a783, a784, a785)
    };
    let a787 = 0u64;
    if if nat_eq(a786, a787) {
        f70(match v0.clone() {
            Adt1::C0(_, h788, _, _, _, _, _, _, _, _, _) => {
                h788
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h789, _, _, _, _, _) => {
                h789
            }
        }, v1)
    } else {
        false
    } {
        f71(match v0.clone() {
            Adt1::C0(_, _, h790, _, _, _, _, _, _, _, _) => {
                h790
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h791, _, _, _, _, _, _, _) => {
                h791
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h792, _, _, _, _, _, _) => {
                h792
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h793, _, _, _, _) => {
                h793
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h794, _, _, _) => {
                h794
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h795, _, _) => {
                h795
            }
        }, v1)
    } else {
        false
    }
}

pub fn f38(v0: Adt1, v1: Adt1) -> R<bool> {
    let a803 = {
        let a801 = match v0.clone() {
            Adt1::C0(_, _, h796, _, _, _, _, _, _, _, _) => {
                h796
            }
        };
        let a802 = {
            let a799 = match v0.clone() {
                Adt1::C0(_, _, _, h797, _, _, _, _, _, _, _) => {
                    h797
                }
            };
            let a800 = match v0.clone() {
                Adt1::C0(_, _, _, _, h798, _, _, _, _, _, _) => {
                    h798
                }
            };
            nat_mul(a799, a800)?
        };
        nat_mul(a801, a802)?
    };
    let a804 = 9223372036854775807u64;
    if nat_le(a803, a804) {
        if f47(v1.clone()) {
            let a806 = match v1 {
                Adt1::C0(_, h805, _, _, _, _, _, _, _, _, _) => {
                    h805
                }
            };
            let a807 = 2147483647u64;
            Ok(nat_le(a806, a807))
        } else {
            let a809 = match v0 {
                Adt1::C0(_, h808, _, _, _, _, _, _, _, _, _) => {
                    h808
                }
            };
            let a810 = 2147483647u64;
            Ok(nat_le(a809, a810))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: Adt1) -> R<bool> {
    f33({
        let a813 = match v0.clone() {
            Adt1::C0(_, h811, _, _, _, _, _, _, _, _, _) => {
                h811
            }
        };
        let a814 = f36(match v0.clone() {
            Adt1::C0(h812, _, _, _, _, _, _, _, _, _, _) => {
                h812
            }
        });
        nat_mul(a813, a814)?
    }, match v0.clone() {
        Adt1::C0(_, _, h815, _, _, _, _, _, _, _, _) => {
            h815
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h816, _, _, _, _, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h817, _, _, _, _, _, _) => {
            h817
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h818, _, _, _, _) => {
            h818
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h819, _, _, _) => {
            h819
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h820, _, _) => {
            h820
        }
    })
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a827 = {
        let a821 = v0;
        let a822 = v2;
        nat_mul(a821, a822)?
    };
    let a828 = {
        let a823 = v1;
        let a824 = 0u64;
        if nat_eq(a823, a824) {
            0u64
        } else {
            let a825 = v0;
            let a826 = 2u64;
            nat_mul(a825, a826)?
        }
    };
    nat_add(a827, a828)
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if f64(v1) {
        Ok(f72(v0, v1))
    } else {
        f66(v1, v2, v3)
    }
}

pub fn f42(v0: u64) -> Adt4 {
    let a829 = v0;
    let a830 = 0u64;
    if nat_eq(a829, a830) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a831 = v0;
        let a832 = 1u64;
        if nat_eq(a831, a832) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a833 = v0;
            let a834 = 2u64;
            if nat_eq(a833, a834) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a835 = v0;
                let a836 = 3u64;
                if nat_eq(a835, a836) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a837 = v0;
                    let a838 = 6u64;
                    if nat_eq(a837, a838) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a839 = v0;
                        let a840 = 7u64;
                        if nat_eq(a839, a840) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a841 = v0;
                            let a842 = 8u64;
                            if nat_eq(a841, a842) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a843 = v0;
                                let a844 = 9u64;
                                if nat_eq(a843, a844) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a845 = v0;
                                    let a846 = 10u64;
                                    if nat_eq(a845, a846) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a847 = v0;
                                        let a848 = 11u64;
                                        if nat_eq(a847, a848) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a849 = v0;
                                            let a850 = 12u64;
                                            if nat_eq(a849, a850) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a851 = v0;
                                                let a852 = 13u64;
                                                if nat_eq(a851, a852) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a853 = v0;
                                                    let a854 = 14u64;
                                                    if nat_eq(a853, a854) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a855 = v0;
                                                        let a856 = 15u64;
                                                        if nat_eq(a855, a856) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a857 = v0;
                                                            let a858 = 16u64;
                                                            if nat_eq(a857, a858) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a859 = v0;
                                                                let a860 = 17u64;
                                                                if nat_eq(a859, a860) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a861 = v0;
                                                                    let a862 = 18u64;
                                                                    if nat_eq(a861, a862) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a863 = v0;
                                                                        let a864 = 19u64;
                                                                        if nat_eq(a863, a864) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a865 = v0;
                                                                            let a866 = 20u64;
                                                                            if nat_eq(a865, a866) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a867 = v0;
                                                                                let a868 = 21u64;
                                                                                if nat_eq(a867, a868) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a869 = v0;
                                                                                    let a870 = 22u64;
                                                                                    if nat_eq(a869, a870) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a871 = v0;
                                                                                        let a872 = 23u64;
                                                                                        if nat_eq(a871, a872) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a873 = v0;
                                                                                            let a874 = 24u64;
                                                                                            if nat_eq(a873, a874) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a875 = v0;
                                                                                                let a876 = 25u64;
                                                                                                if nat_eq(a875, a876) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a877 = v0;
                                                                                                    let a878 = 26u64;
                                                                                                    if nat_eq(a877, a878) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a879 = v0;
                                                                                                        let a880 = 27u64;
                                                                                                        if nat_eq(a879, a880) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a881 = v0;
                                                                                                            let a882 = 28u64;
                                                                                                            if nat_eq(a881, a882) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a883 = v0;
                                                                                                                let a884 = 29u64;
                                                                                                                if nat_eq(a883, a884) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a885 = v0;
                                                                                                                    let a886 = 30u64;
                                                                                                                    if nat_eq(a885, a886) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a887 = v0;
                                                                                                                        let a888 = 34u64;
                                                                                                                        if nat_eq(a887, a888) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a889 = v0;
                                                                                                                            let a890 = 35u64;
                                                                                                                            if nat_eq(a889, a890) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a891 = v0;
                                                                                                                                let a892 = 39u64;
                                                                                                                                if nat_eq(a891, a892) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a893 = v0;
                                                                                                                                    let a894 = 40u64;
                                                                                                                                    if nat_eq(a893, a894) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a895 = v0;
                                                                                                                                        let a896 = 41u64;
                                                                                                                                        if nat_eq(a895, a896) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a897 = v0;
                                                                                                                                            let a898 = 42u64;
                                                                                                                                            if nat_eq(a897, a898) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a899 = v0;
                                                                                                                                                let a900 = 43u64;
                                                                                                                                                if nat_eq(a899, a900) {
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

pub fn f43(v0: Adt1, v1: Adt1) -> R<bool> {
    let a902 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h901, _) => {
            h901
        }
    };
    let a903 = 0u64;
    if nat_eq(a902, a903) {
        Ok(true)
    } else {
        let a905 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h904, _) => {
                h904
            }
        };
        let a906 = 0u64;
        if nat_eq(a905, a906) {
            Ok(true)
        } else {
            let a908 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h907) => {
                    h907
                }
            };
            let a909 = 0u64;
            if nat_eq(a908, a909) {
                Ok(true)
            } else {
                let a911 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h910) => {
                        h910
                    }
                };
                let a912 = 0u64;
                if nat_eq(a911, a912) {
                    Ok(true)
                } else {
                    let a918 = {
                        let a915 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h913) => {
                                h913
                            }
                        };
                        let a916 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h914, _) => {
                                h914
                            }
                        };
                        nat_add(a915, a916)?
                    };
                    let a919 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h917) => {
                            h917
                        }
                    };
                    if nat_le(a918, a919) {
                        Ok(true)
                    } else {
                        let a925 = {
                            let a922 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h920) => {
                                    h920
                                }
                            };
                            let a923 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h921, _) => {
                                    h921
                                }
                            };
                            nat_add(a922, a923)?
                        };
                        let a926 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h924) => {
                                h924
                            }
                        };
                        Ok(nat_le(a925, a926))
                    }
                }
            }
        }
    }
}

pub fn f44(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a929 = match v0.clone() {
            Adt1::C0(h927, _, _, _, _, _, _, _, _, _, _) => {
                h927
            }
        };
        let a930 = match v1.clone() {
            Adt1::C0(h928, _, _, _, _, _, _, _, _, _, _) => {
                h928
            }
        };
        nat_eq(a929, a930)
    } else {
        false
    } {
        let a933 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h931) => {
                h931
            }
        };
        let a934 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h932) => {
                h932
            }
        };
        nat_eq(a933, a934)
    } else {
        false
    } {
        let a937 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h935, _, _, _, _, _) => {
                h935
            }
        };
        let a938 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h936, _, _, _, _, _) => {
                h936
            }
        };
        nat_eq(a937, a938)
    } else {
        false
    } {
        let a941 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h939, _, _, _, _) => {
                h939
            }
        };
        let a942 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h940, _, _, _, _) => {
                h940
            }
        };
        nat_eq(a941, a942)
    } else {
        false
    } {
        let a945 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h943, _, _, _) => {
                h943
            }
        };
        let a946 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h944, _, _, _) => {
                h944
            }
        };
        nat_eq(a945, a946)
    } else {
        false
    } {
        let a949 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h947, _, _) => {
                h947
            }
        };
        let a950 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h948, _, _) => {
                h948
            }
        };
        nat_eq(a949, a950)
    } else {
        false
    }
}

pub fn f45(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a953 = match v0.clone() {
            Adt1::C0(h951, _, _, _, _, _, _, _, _, _, _) => {
                h951
            }
        };
        let a954 = match v1.clone() {
            Adt1::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
                h952
            }
        };
        nat_eq(a953, a954)
    } else {
        false
    } {
        let a957 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h955, _, _, _, _, _) => {
                h955
            }
        };
        let a958 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h956, _, _, _, _, _) => {
                h956
            }
        };
        nat_eq(a957, a958)
    } else {
        false
    } {
        let a961 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h959, _, _, _, _) => {
                h959
            }
        };
        let a962 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h960, _, _, _, _) => {
                h960
            }
        };
        nat_eq(a961, a962)
    } else {
        false
    } {
        let a965 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h963, _, _, _) => {
                h963
            }
        };
        let a966 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h964, _, _, _) => {
                h964
            }
        };
        nat_eq(a965, a966)
    } else {
        false
    } {
        let a969 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h967, _, _) => {
                h967
            }
        };
        let a970 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h968, _, _) => {
                h968
            }
        };
        nat_eq(a969, a970)
    } else {
        false
    } {
        let a972 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h971) => {
                h971
            }
        };
        let a973 = 0u64;
        if if nat_eq(a972, a973) {
            true
        } else {
            let a975 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h974) => {
                    h974
                }
            };
            let a976 = 0u64;
            nat_eq(a975, a976)
        } {
            true
        } else {
            let a979 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h977) => {
                    h977
                }
            };
            let a980 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h978) => {
                    h978
                }
            };
            nat_eq(a979, a980)
        }
    } else {
        false
    }
}

pub fn f46(v0: u64) -> u64 {
    let a981 = v0;
    let a982 = 26u64;
    if nat_eq(a981, a982) {
        4u64
    } else {
        let a983 = v0;
        let a984 = 27u64;
        if nat_eq(a983, a984) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f47(v0: Adt1) -> bool {
    let a986 = match v0.clone() {
        Adt1::C0(_, h985, _, _, _, _, _, _, _, _, _) => {
            h985
        }
    };
    let a987 = 1u64;
    if nat_eq(a986, a987) {
        true
    } else {
        let a990 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h988, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = f67(match v0 {
            Adt1::C0(h989, _, _, _, _, _, _, _, _, _, _) => {
                h989
            }
        });
        nat_eq(a990, a991)
    }
}

pub fn f48(v0: Adt1, v1: u64) -> R<u64> {
    let a997 = {
        let a993 = match v0.clone() {
            Adt1::C0(_, h992, _, _, _, _, _, _, _, _, _) => {
                h992
            }
        };
        let a994 = f32(v1);
        let a995 = 0u64;
        nat_quot(a993, a994, a995)
    };
    let a998 = f73(match v0 {
        Adt1::C0(h996, _, _, _, _, _, _, _, _, _, _) => {
            h996
        }
    }, v1);
    nat_mul(a997, a998)
}

pub fn f49(v0: u64) -> u64 {
    if if f24(v0) {
        true
    } else {
        let a999 = 24u64;
        let a1000 = v0;
        if nat_le(a999, a1000) {
            let a1001 = v0;
            let a1002 = 28u64;
            nat_le(a1001, a1002)
        } else {
            false
        }
    } {
        f32(v0)
    } else {
        0u64
    }
}

pub fn f50(v0: Adt1, v1: Adt1) -> u64 {
    let a1005 = match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1003, _, _, _, _, _) => {
            h1003
        }
    };
    let a1006 = f7(match v1.clone() {
        Adt1::C0(h1004, _, _, _, _, _, _, _, _, _, _) => {
            h1004
        }
    });
    if nat_eq(a1005, a1006) {
        f74(v0, v1)
    } else {
        0u64
    }
}

pub fn f51(v0: Adt1) -> u64 {
    let a1009 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1007, _, _, _, _, _) => {
            h1007
        }
    };
    let a1010 = f7(match v0.clone() {
        Adt1::C0(h1008, _, _, _, _, _, _, _, _, _, _) => {
            h1008
        }
    });
    if nat_eq(a1009, a1010) {
        match v0 {
            Adt1::C0(_, h1011, _, _, _, _, _, _, _, _, _) => {
                h1011
            }
        }
    } else {
        f32(match v0 {
            Adt1::C0(h1012, _, _, _, _, _, _, _, _, _, _) => {
                h1012
            }
        })
    }
}

pub fn f52(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1014 = match v0.clone() {
            Adt1::C0(_, h1013, _, _, _, _, _, _, _, _, _) => {
                h1013
            }
        };
        let a1015 = 4u64;
        nat_mul(a1014, a1015)?
    };
    let v2: u64 = {
        let a1018 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1016, _, _, _, _) => {
                h1016
            }
        };
        let a1019 = match v0.clone() {
            Adt1::C0(_, _, h1017, _, _, _, _, _, _, _, _) => {
                h1017
            }
        };
        nat_mul(a1018, a1019)?
    };
    let v3: u64 = {
        let a1021 = v2;
        let a1022 = match v0.clone() {
            Adt1::C0(_, _, _, h1020, _, _, _, _, _, _, _) => {
                h1020
            }
        };
        nat_mul(a1021, a1022)?
    };
    let a1024 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1023, _, _, _, _, _) => {
            h1023
        }
    };
    let a1025 = 4u64;
    if if if if nat_eq(a1024, a1025) {
        let a1030 = {
            let a1027 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1026) => {
                    h1026
                }
            };
            let a1028 = 4u64;
            let a1029 = 1u64;
            nat_rem(a1027, a1028, a1029)
        };
        let a1031 = 0u64;
        nat_eq(a1030, a1031)
    } else {
        false
    } {
        let a1039 = {
            let a1037 = match v0.clone() {
                Adt1::C0(_, _, h1032, _, _, _, _, _, _, _, _) => {
                    h1032
                }
            };
            let a1038 = {
                let a1035 = match v0.clone() {
                    Adt1::C0(_, _, _, h1033, _, _, _, _, _, _, _) => {
                        h1033
                    }
                };
                let a1036 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h1034, _, _, _, _, _, _) => {
                        h1034
                    }
                };
                nat_mul(a1035, a1036)?
            };
            nat_mul(a1037, a1038)?
        };
        let a1040 = 1u64;
        if nat_le(a1039, a1040) {
            true
        } else {
            let a1042 = v1;
            let a1043 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1041, _, _, _, _) => {
                    h1041
                }
            };
            if nat_le(a1042, a1043) {
                let a1048 = {
                    let a1045 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h1044, _, _, _, _) => {
                            h1044
                        }
                    };
                    let a1046 = 4u64;
                    let a1047 = 1u64;
                    nat_rem(a1045, a1046, a1047)
                };
                let a1049 = 0u64;
                nat_eq(a1048, a1049)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a1051 = match v0.clone() {
            Adt1::C0(_, _, _, h1050, _, _, _, _, _, _, _) => {
                h1050
            }
        };
        let a1052 = 1u64;
        if nat_le(a1051, a1052) {
            true
        } else {
            let a1054 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1053, _, _, _) => {
                    h1053
                }
            };
            let a1055 = v2;
            nat_eq(a1054, a1055)
        }
    } else {
        false
    } {
        let a1057 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1056, _, _, _, _, _, _) => {
                h1056
            }
        };
        let a1058 = 1u64;
        if nat_le(a1057, a1058) {
            Ok(true)
        } else {
            let a1060 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h1059, _, _) => {
                    h1059
                }
            };
            let a1061 = v3;
            Ok(nat_eq(a1060, a1061))
        }
    } else {
        Ok(false)
    }
}

pub fn f53(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1064 = match v0.clone() {
            Adt1::C0(_, _, h1062, _, _, _, _, _, _, _, _) => {
                h1062
            }
        };
        let a1065 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1063, _, _, _, _) => {
                h1063
            }
        };
        nat_mul(a1064, a1065)?
    };
    if if f47(v0.clone()) {
        let a1067 = match v0.clone() {
            Adt1::C0(_, _, _, h1066, _, _, _, _, _, _, _) => {
                h1066
            }
        };
        let a1068 = 1u64;
        if nat_eq(a1067, a1068) {
            true
        } else {
            let a1070 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1069, _, _, _) => {
                    h1069
                }
            };
            let a1071 = v1;
            nat_eq(a1070, a1071)
        }
    } else {
        false
    } {
        let a1073 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1072, _, _, _, _, _, _) => {
                h1072
            }
        };
        let a1074 = 1u64;
        if nat_eq(a1073, a1074) {
            Ok(true)
        } else {
            let a1079 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1075, _, _) => {
                    h1075
                }
            };
            let a1080 = {
                let a1077 = v1;
                let a1078 = match v0 {
                    Adt1::C0(_, _, _, h1076, _, _, _, _, _, _, _) => {
                        h1076
                    }
                };
                nat_mul(a1077, a1078)?
            };
            Ok(nat_eq(a1079, a1080))
        }
    } else {
        Ok(false)
    }
}

pub fn f54(v0: u64) -> u64 {
    let a1081 = v0;
    let a1082 = 3u64;
    if nat_eq(a1081, a1082) {
        16u64
    } else {
        10u64
    }
}

pub fn f55(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1084 = v0;
        let a1085 = match v1.clone() {
            Adt1::C0(_, h1083, _, _, _, _, _, _, _, _, _) => {
                h1083
            }
        };
        let a1086 = 0u64;
        nat_quot(a1084, a1085, a1086)
    };
    let v3: u64 = {
        let a1088 = v2;
        let a1089 = match v1.clone() {
            Adt1::C0(_, _, h1087, _, _, _, _, _, _, _, _) => {
                h1087
            }
        };
        let a1090 = 0u64;
        nat_quot(a1088, a1089, a1090)
    };
    f75({
        let a1092 = v0;
        let a1093 = match v1.clone() {
            Adt1::C0(_, h1091, _, _, _, _, _, _, _, _, _) => {
                h1091
            }
        };
        let a1094 = 0u64;
        nat_rem(a1092, a1093, a1094)
    }, {
        let a1096 = v2;
        let a1097 = match v1.clone() {
            Adt1::C0(_, _, h1095, _, _, _, _, _, _, _, _) => {
                h1095
            }
        };
        let a1098 = 0u64;
        nat_rem(a1096, a1097, a1098)
    }, {
        let a1100 = v3;
        let a1101 = match v1.clone() {
            Adt1::C0(_, _, _, h1099, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1102 = 0u64;
        nat_rem(a1100, a1101, a1102)
    }, {
        let a1104 = v3;
        let a1105 = match v1.clone() {
            Adt1::C0(_, _, _, h1103, _, _, _, _, _, _, _) => {
                h1103
            }
        };
        let a1106 = 0u64;
        nat_quot(a1104, a1105, a1106)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1107, _, _, _, _, _) => {
            h1107
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1108, _, _, _, _) => {
            h1108
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1109, _, _, _) => {
            h1109
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1110, _, _) => {
            h1110
        }
    })
}

pub fn f56(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1112 = v0;
        let a1113 = match v1.clone() {
            Adt1::C0(_, h1111, _, _, _, _, _, _, _, _, _) => {
                h1111
            }
        };
        let a1114 = 0u64;
        nat_quot(a1112, a1113, a1114)
    };
    let v4: u64 = {
        let a1116 = v3;
        let a1117 = match v1.clone() {
            Adt1::C0(_, _, h1115, _, _, _, _, _, _, _, _) => {
                h1115
            }
        };
        let a1118 = 0u64;
        nat_quot(a1116, a1117, a1118)
    };
    f75({
        let a1124 = {
            let a1120 = v0;
            let a1121 = match v1.clone() {
                Adt1::C0(_, h1119, _, _, _, _, _, _, _, _, _) => {
                    h1119
                }
            };
            let a1122 = 0u64;
            nat_rem(a1120, a1121, a1122)
        };
        let a1125 = match v2.clone() {
            Adt1::C0(_, h1123, _, _, _, _, _, _, _, _, _) => {
                h1123
            }
        };
        let a1126 = 0u64;
        nat_rem(a1124, a1125, a1126)
    }, {
        let a1132 = {
            let a1128 = v3;
            let a1129 = match v1.clone() {
                Adt1::C0(_, _, h1127, _, _, _, _, _, _, _, _) => {
                    h1127
                }
            };
            let a1130 = 0u64;
            nat_rem(a1128, a1129, a1130)
        };
        let a1133 = match v2.clone() {
            Adt1::C0(_, _, h1131, _, _, _, _, _, _, _, _) => {
                h1131
            }
        };
        let a1134 = 0u64;
        nat_rem(a1132, a1133, a1134)
    }, {
        let a1140 = {
            let a1136 = v4;
            let a1137 = match v1.clone() {
                Adt1::C0(_, _, _, h1135, _, _, _, _, _, _, _) => {
                    h1135
                }
            };
            let a1138 = 0u64;
            nat_rem(a1136, a1137, a1138)
        };
        let a1141 = match v2.clone() {
            Adt1::C0(_, _, _, h1139, _, _, _, _, _, _, _) => {
                h1139
            }
        };
        let a1142 = 0u64;
        nat_rem(a1140, a1141, a1142)
    }, {
        let a1148 = {
            let a1144 = v4;
            let a1145 = match v1 {
                Adt1::C0(_, _, _, h1143, _, _, _, _, _, _, _) => {
                    h1143
                }
            };
            let a1146 = 0u64;
            nat_quot(a1144, a1145, a1146)
        };
        let a1149 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1147, _, _, _, _, _, _) => {
                h1147
            }
        };
        let a1150 = 0u64;
        nat_rem(a1148, a1149, a1150)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1151, _, _, _, _, _) => {
            h1151
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1152, _, _, _, _) => {
            h1152
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1153, _, _, _) => {
            h1153
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1154, _, _) => {
            h1154
        }
    })
}

pub fn f57(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1156 = v0;
        let a1157 = match v1.clone() {
            Adt1::C0(_, h1155, _, _, _, _, _, _, _, _, _) => {
                h1155
            }
        };
        let a1158 = 0u64;
        nat_quot(a1156, a1157, a1158)
    };
    let v3: u64 = {
        let a1160 = v2;
        let a1161 = match v1.clone() {
            Adt1::C0(_, _, h1159, _, _, _, _, _, _, _, _) => {
                h1159
            }
        };
        let a1162 = 0u64;
        nat_quot(a1160, a1161, a1162)
    };
    let v4: u64 = {
        let a1168 = {
            let a1164 = v0;
            let a1165 = match v1.clone() {
                Adt1::C0(_, h1163, _, _, _, _, _, _, _, _, _) => {
                    h1163
                }
            };
            let a1166 = 0u64;
            nat_rem(a1164, a1165, a1166)
        };
        let a1169 = f32(match v1.clone() {
            Adt1::C0(h1167, _, _, _, _, _, _, _, _, _, _) => {
                h1167
            }
        });
        let a1170 = 0u64;
        nat_quot(a1168, a1169, a1170)
    };
    f75(v4, {
        let a1172 = v2;
        let a1173 = match v1.clone() {
            Adt1::C0(_, _, h1171, _, _, _, _, _, _, _, _) => {
                h1171
            }
        };
        let a1174 = 0u64;
        nat_rem(a1172, a1173, a1174)
    }, {
        let a1176 = v3;
        let a1177 = match v1.clone() {
            Adt1::C0(_, _, _, h1175, _, _, _, _, _, _, _) => {
                h1175
            }
        };
        let a1178 = 0u64;
        nat_rem(a1176, a1177, a1178)
    }, {
        let a1180 = v3;
        let a1181 = match v1.clone() {
            Adt1::C0(_, _, _, h1179, _, _, _, _, _, _, _) => {
                h1179
            }
        };
        let a1182 = 0u64;
        nat_quot(a1180, a1181, a1182)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1183, _, _, _, _, _) => {
            h1183
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1184, _, _, _, _) => {
            h1184
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1185, _, _, _) => {
            h1185
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1186, _, _) => {
            h1186
        }
    })
}

pub fn f58(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1187 = v0;
        let a1188 = 2u64;
        let a1189 = 0u64;
        nat_quot(a1187, a1188, a1189)
    };
    let v5: u64 = f46(match v1.clone() {
        Adt1::C0(h1190, _, _, _, _, _, _, _, _, _, _) => {
            h1190
        }
    });
    let a1194 = {
        let a1191 = v0;
        let a1192 = 2u64;
        let a1193 = 0u64;
        nat_rem(a1191, a1192, a1193)
    };
    let a1195 = 0u64;
    if nat_eq(a1194, a1195) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f55(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1196 = v3;
        let a1197 = v2;
        if if nat_lt(a1196, a1197) {
            let a1200 = v3;
            let a1201 = {
                let a1198 = v5;
                let a1199 = 4u64;
                if nat_eq(a1198, a1199) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1200, a1201)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1202 = 3u64;
                let a1203 = v4;
                nat_add(a1202, a1203)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f59(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1205 = 1u64;
        let a1206 = f48(v1.clone(), match v3.clone() {
            Adt1::C0(h1204, _, _, _, _, _, _, _, _, _, _) => {
                h1204
            }
        })?;
        nat_add(a1205, a1206)?
    };
    let v6: u64 = {
        let a1207 = v0;
        let a1208 = v5;
        let a1209 = 0u64;
        nat_rem(a1207, a1208, a1209)
    };
    let v7: u64 = {
        let a1210 = v0;
        let a1211 = v5;
        let a1212 = 0u64;
        nat_quot(a1210, a1211, a1212)
    };
    let v8: u64 = {
        let a1214 = v7;
        let a1215 = match v1.clone() {
            Adt1::C0(_, _, h1213, _, _, _, _, _, _, _, _) => {
                h1213
            }
        };
        let a1216 = 0u64;
        nat_rem(a1214, a1215, a1216)
    };
    let v9: u64 = {
        let a1218 = v7;
        let a1219 = match v1.clone() {
            Adt1::C0(_, _, h1217, _, _, _, _, _, _, _, _) => {
                h1217
            }
        };
        let a1220 = 0u64;
        nat_quot(a1218, a1219, a1220)
    };
    let v10: u64 = {
        let a1222 = v9;
        let a1223 = match v1.clone() {
            Adt1::C0(_, _, _, h1221, _, _, _, _, _, _, _) => {
                h1221
            }
        };
        let a1224 = 0u64;
        nat_rem(a1222, a1223, a1224)
    };
    let v11: u64 = {
        let a1226 = v9;
        let a1227 = match v1.clone() {
            Adt1::C0(_, _, _, h1225, _, _, _, _, _, _, _) => {
                h1225
            }
        };
        let a1228 = 0u64;
        nat_quot(a1226, a1227, a1228)
    };
    let a1229 = v6;
    let a1230 = 0u64;
    if nat_eq(a1229, a1230) {
        let v12: u64 = {
            let a1247 = v8;
            let a1248 = {
                let a1245 = match v2.clone() {
                    Adt1::C0(_, h1231, _, _, _, _, _, _, _, _, _) => {
                        h1231
                    }
                };
                let a1246 = {
                    let a1243 = {
                        let a1233 = v10;
                        let a1234 = match v2.clone() {
                            Adt1::C0(_, _, h1232, _, _, _, _, _, _, _, _) => {
                                h1232
                            }
                        };
                        let a1235 = 0u64;
                        nat_rem(a1233, a1234, a1235)
                    };
                    let a1244 = {
                        let a1241 = match v2.clone() {
                            Adt1::C0(_, _, h1236, _, _, _, _, _, _, _, _) => {
                                h1236
                            }
                        };
                        let a1242 = {
                            let a1238 = v11;
                            let a1239 = match v2 {
                                Adt1::C0(_, _, _, h1237, _, _, _, _, _, _, _) => {
                                    h1237
                                }
                            };
                            let a1240 = 0u64;
                            nat_rem(a1238, a1239, a1240)
                        };
                        nat_mul(a1241, a1242)?
                    };
                    nat_add(a1243, a1244)?
                };
                nat_mul(a1245, a1246)?
            };
            nat_add(a1247, a1248)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1249 = 3u64;
            let a1250 = v12;
            nat_add(a1249, a1250)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1252 = v4;
        let a1253 = match v3.clone() {
            Adt1::C0(_, _, h1251, _, _, _, _, _, _, _, _) => {
                h1251
            }
        };
        if nat_lt(a1252, a1253) {
            let v13: u64 = {
                let a1258 = {
                    let a1254 = v6;
                    let a1255 = 1u64;
                    nat_sub(a1254, a1255)
                };
                let a1259 = f73(match v1.clone() {
                    Adt1::C0(h1256, _, _, _, _, _, _, _, _, _, _) => {
                        h1256
                    }
                }, match v3.clone() {
                    Adt1::C0(h1257, _, _, _, _, _, _, _, _, _, _) => {
                        h1257
                    }
                });
                let a1260 = 0u64;
                nat_quot(a1258, a1259, a1260)
            };
            let v14: u64 = f75({
                let a1262 = v13;
                let a1263 = f32(match v3.clone() {
                    Adt1::C0(h1261, _, _, _, _, _, _, _, _, _, _) => {
                        h1261
                    }
                });
                nat_mul(a1262, a1263)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1264, _, _, _, _, _) => {
                    h1264
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1265, _, _, _, _) => {
                    h1265
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1266, _, _, _) => {
                    h1266
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1267, _, _) => {
                    h1267
                }
            })?;
            let v15: u64 = f75(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1268, _, _, _, _, _) => {
                    h1268
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1269, _, _, _, _) => {
                    h1269
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1270, _, _, _) => {
                    h1270
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1271, _, _) => {
                    h1271
                }
            })?;
            let a1273 = 0u64;
            let a1274 = f49(match v3.clone() {
                Adt1::C0(h1272, _, _, _, _, _, _, _, _, _, _) => {
                    h1272
                }
            });
            if nat_lt(a1273, a1274) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f76(match v1 {
                    Adt1::C0(h1275, _, _, _, _, _, _, _, _, _, _) => {
                        h1275
                    }
                }, match v3 {
                    Adt1::C0(h1276, _, _, _, _, _, _, _, _, _, _) => {
                        h1276
                    }
                }, {
                    let a1277 = v6;
                    let a1278 = 1u64;
                    nat_sub(a1277, a1278)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f60(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f62(v0: u64, v1: u64) -> Adt3 {
    let a1279 = v0;
    let a1280 = 8u64;
    if nat_eq(a1279, a1280) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1281 = v0;
        let a1282 = 9u64;
        if nat_eq(a1281, a1282) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1283 = v0;
            let a1284 = 10u64;
            if nat_eq(a1283, a1284) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1285 = v0;
                let a1286 = 11u64;
                if nat_eq(a1285, a1286) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1287 = v0;
                    let a1288 = 12u64;
                    if nat_eq(a1287, a1288) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1289 = v0;
                        let a1290 = 13u64;
                        if nat_eq(a1289, a1290) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1291 = v0;
                            let a1292 = 14u64;
                            if nat_eq(a1291, a1292) {
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

pub fn f63(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1293, _, _, _, _) => {
            h1293
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1294, _, _, _) => {
            h1294
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1295, _, _) => {
            h1295
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1296, _, _, _, _, _, _, _, _) => {
            h1296
        }
    };
    let v7: u64 = {
        let a1298 = v1;
        let a1299 = match v5.clone() {
            Adt1::C0(_, h1297, _, _, _, _, _, _, _, _, _) => {
                h1297
            }
        };
        let a1300 = 0u64;
        nat_quot(a1298, a1299, a1300)
    };
    let v8: u64 = f27(match v5.clone() {
        Adt1::C0(h1301, _, _, _, _, _, _, _, _, _, _) => {
            h1301
        }
    });
    let v9: u64 = {
        let a1306 = {
            let a1303 = v1;
            let a1304 = match v5.clone() {
                Adt1::C0(_, h1302, _, _, _, _, _, _, _, _, _) => {
                    h1302
                }
            };
            let a1305 = 0u64;
            nat_rem(a1303, a1304, a1305)
        };
        let a1307 = v8;
        nat_mul(a1306, a1307)?
    };
    let v10: u64 = {
        let a1309 = match v5.clone() {
            Adt1::C0(_, h1308, _, _, _, _, _, _, _, _, _) => {
                h1308
            }
        };
        let a1310 = v8;
        nat_mul(a1309, a1310)?
    };
    let a1311 = v0;
    let a1312 = 0u64;
    if nat_eq(a1311, a1312) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1313, _, _, _, _, _, _, _, _, _, _) => {
                h1313
            }
        }, 0u64, 0u64, {
            let a1321 = {
                let a1317 = {
                    let a1315 = v7;
                    let a1316 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1314, _, _, _, _) => {
                            h1314
                        }
                    };
                    nat_mul(a1315, a1316)?
                };
                let a1318 = v9;
                nat_add(a1317, a1318)?
            };
            let a1322 = {
                let a1319 = v6;
                let a1320 = 100u64;
                if nat_eq(a1319, a1320) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1321, a1322)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1323 = v0;
        let a1324 = 1u64;
        if nat_eq(a1323, a1324) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1325, _, _, _, _, _, _, _, _, _, _) => {
                    h1325
                }
            }, 0u64, {
                let a1326 = v6;
                let a1327 = 98u64;
                if nat_eq(a1326, a1327) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1335 = {
                    let a1331 = {
                        let a1329 = v7;
                        let a1330 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1328, _, _, _, _) => {
                                h1328
                            }
                        };
                        nat_mul(a1329, a1330)?
                    };
                    let a1332 = v9;
                    nat_add(a1331, a1332)?
                };
                let a1336 = {
                    let a1333 = v6;
                    let a1334 = 99u64;
                    if nat_eq(a1333, a1334) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1335, a1336)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1337 = v0;
            let a1338 = 2u64;
            if nat_eq(a1337, a1338) {
                Ok(Adt3::C4(f77(match v2 {
                    Adt2::C0(_, _, _, _, h1339, _, _, _, _, _, _, _) => {
                        h1339
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1340, _, _, _, _, _, _, _, _, _, _) => {
                        h1340
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1341, _, _, _, _, _, _, _, _, _, _) => {
                        h1341
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1345 = {
                        let a1343 = v7;
                        let a1344 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1342, _, _, _, _) => {
                                h1342
                            }
                        };
                        nat_mul(a1343, a1344)?
                    };
                    let a1346 = v9;
                    nat_add(a1345, a1346)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f64(v0: u64) -> bool {
    let a1347 = v0;
    let a1348 = 27u64;
    if nat_eq(a1347, a1348) {
        true
    } else {
        let a1349 = v0;
        let a1350 = 28u64;
        nat_eq(a1349, a1350)
    }
}

pub fn f65(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1351, _, _, _, _) => {
            h1351
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1352, _, _, _) => {
            h1352
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1353, _, _) => {
            h1353
        }
    };
    let v5: u64 = match v1 {
        Adt2::C0(_, _, _, h1354, _, _, _, _, _, _, _, _) => {
            h1354
        }
    };
    let v6: u64 = f72(v5, match v2.clone() {
        Adt1::C0(h1355, _, _, _, _, _, _, _, _, _, _) => {
            h1355
        }
    });
    let v7: u64 = {
        let a1356 = v0;
        let a1357 = v6;
        let a1358 = 0u64;
        nat_quot(a1356, a1357, a1358)
    };
    let v8: u64 = f56(v7, v2.clone(), v3)?;
    let a1359 = v6;
    let a1360 = 2u64;
    if if nat_eq(a1359, a1360) {
        let a1364 = {
            let a1361 = v0;
            let a1362 = v6;
            let a1363 = 0u64;
            nat_rem(a1361, a1362, a1363)
        };
        let a1365 = 0u64;
        nat_eq(a1364, a1365)
    } else {
        false
    } {
        Ok(Adt3::C3(6u64, 15u64, 0u64, 0u64, 4u64, v8, 2u64, 9223372036854775808u64, 0u64, 0u64, 8u64, 8u64))
    } else {
        let v9: u64 = {
            let a1367 = match v2.clone() {
                Adt1::C0(h1366, _, _, _, _, _, _, _, _, _, _) => {
                    h1366
                }
            };
            let a1368 = 27u64;
            if nat_eq(a1367, a1368) {
                let a1369 = 15u64;
                let a1370 = v5;
                nat_add(a1369, a1370)?
            } else {
                let a1371 = v5;
                let a1372 = 2u64;
                if nat_eq(a1371, a1372) {
                    14u64
                } else {
                    0u64
                }
            }
        };
        Ok(Adt3::C3(6u64, v9, 0u64, 0u64, 0u64, f55(v7, v2)?, {
            let a1373 = v6;
            let a1374 = 2u64;
            if nat_eq(a1373, a1374) {
                1u64
            } else {
                4u64
            }
        }, {
            let a1375 = v6;
            let a1376 = 2u64;
            if nat_eq(a1375, a1376) {
                0u64
            } else {
                v8
            }
        }, 3u64, f55(v7, v4)?, 8u64, 8u64))
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1387 = {
        let a1383 = {
            let a1379 = 1u64;
            let a1380 = {
                let a1377 = v0;
                let a1378 = 0u64;
                if nat_eq(a1377, a1378) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a1379, a1380)?
        };
        let a1384 = {
            let a1381 = v1;
            let a1382 = 0u64;
            if nat_eq(a1381, a1382) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a1383, a1384)?
    };
    let a1388 = {
        let a1385 = v2;
        let a1386 = 0u64;
        if nat_eq(a1385, a1386) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a1387, a1388)
}

pub fn f67(v0: u64) -> u64 {
    let a1389 = v0;
    let a1390 = 43u64;
    if nat_eq(a1389, a1390) {
        1u64
    } else {
        f27(v0)
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1391 = v1;
    let a1392 = 1u64;
    if if if nat_le(a1391, a1392) {
        true
    } else {
        let a1393 = v0;
        let a1394 = v2;
        nat_le(a1393, a1394)
    } {
        let a1395 = v3;
        let a1396 = 1u64;
        if nat_le(a1395, a1396) {
            true
        } else {
            let a1403 = {
                let a1401 = v0;
                let a1402 = {
                    let a1399 = {
                        let a1397 = v1;
                        let a1398 = 1u64;
                        nat_sub(a1397, a1398)
                    };
                    let a1400 = v2;
                    nat_mul(a1399, a1400)?
                };
                nat_add(a1401, a1402)?
            };
            let a1404 = v4;
            nat_le(a1403, a1404)
        }
    } else {
        false
    } {
        let a1405 = v5;
        let a1406 = 1u64;
        if nat_le(a1405, a1406) {
            Ok(true)
        } else {
            let a1419 = {
                let a1417 = {
                    let a1411 = v0;
                    let a1412 = {
                        let a1409 = {
                            let a1407 = v1;
                            let a1408 = 1u64;
                            nat_sub(a1407, a1408)
                        };
                        let a1410 = v2;
                        nat_mul(a1409, a1410)?
                    };
                    nat_add(a1411, a1412)?
                };
                let a1418 = {
                    let a1415 = {
                        let a1413 = v3;
                        let a1414 = 1u64;
                        nat_sub(a1413, a1414)
                    };
                    let a1416 = v4;
                    nat_mul(a1415, a1416)?
                };
                nat_add(a1417, a1418)?
            };
            let a1420 = v6;
            Ok(nat_le(a1419, a1420))
        }
    } else {
        Ok(false)
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1421 = v0;
    let a1422 = 0u64;
    if if if nat_eq(a1421, a1422) {
        true
    } else {
        let a1423 = v1;
        let a1424 = 0u64;
        nat_eq(a1423, a1424)
    } {
        true
    } else {
        let a1425 = v2;
        let a1426 = 0u64;
        nat_eq(a1425, a1426)
    } {
        true
    } else {
        let a1427 = v3;
        let a1428 = 0u64;
        nat_eq(a1427, a1428)
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> bool {
    let a1429 = v0;
    let a1430 = 1u64;
    if nat_le(a1429, a1430) {
        true
    } else {
        let a1434 = {
            let a1431 = v1;
            let a1432 = v2;
            let a1433 = 1u64;
            nat_rem(a1431, a1432, a1433)
        };
        let a1435 = 0u64;
        nat_eq(a1434, a1435)
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
    let a1436 = v1;
    let a1437 = 28u64;
    if if nat_eq(a1436, a1437) {
        let a1438 = v0;
        let a1439 = 1u64;
        nat_eq(a1438, a1439)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f73(v0: u64, v1: u64) -> u64 {
    let a1440 = v0;
    let a1441 = 1u64;
    if if nat_eq(a1440, a1441) {
        let a1442 = v1;
        let a1443 = 43u64;
        nat_eq(a1442, a1443)
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
        Adt1::C0(h1444, _, _, _, _, _, _, _, _, _, _) => {
            h1444
        }
    };
    let v3: u64 = match v1.clone() {
        Adt1::C0(h1445, _, _, _, _, _, _, _, _, _, _) => {
            h1445
        }
    };
    if if if f5(v0.clone(), v1.clone()) {
        if f24(v2) {
            true
        } else {
            f24(v3)
        }
    } else {
        false
    } {
        if if if f24(v2) {
            true
        } else {
            f28(v2)
        } {
            if f24(v3) {
                true
            } else {
                f28(v3)
            }
        } else {
            false
        } {
            let a1450 = {
                let a1447 = match v0.clone() {
                    Adt1::C0(_, h1446, _, _, _, _, _, _, _, _, _) => {
                        h1446
                    }
                };
                let a1448 = f32(v2);
                let a1449 = 1u64;
                nat_rem(a1447, a1448, a1449)
            };
            let a1451 = 0u64;
            if nat_eq(a1450, a1451) {
                let a1456 = {
                    let a1453 = match v1.clone() {
                        Adt1::C0(_, h1452, _, _, _, _, _, _, _, _, _) => {
                            h1452
                        }
                    };
                    let a1454 = f32(v3);
                    let a1455 = 1u64;
                    nat_rem(a1453, a1454, a1455)
                };
                let a1457 = 0u64;
                nat_eq(a1456, a1457)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        let a1459 = match v0.clone() {
            Adt1::C0(_, h1458, _, _, _, _, _, _, _, _, _) => {
                h1458
            }
        };
        let a1460 = 0u64;
        if nat_eq(a1459, a1460) {
            f32(v3)
        } else {
            match v0 {
                Adt1::C0(_, h1461, _, _, _, _, _, _, _, _, _) => {
                    h1461
                }
            }
        }
    } else {
        if if if f24(v3) {
            let a1462 = 0u64;
            let a1463 = f27(v2);
            nat_lt(a1462, a1463)
        } else {
            false
        } {
            let a1468 = {
                let a1465 = match v0 {
                    Adt1::C0(_, h1464, _, _, _, _, _, _, _, _, _) => {
                        h1464
                    }
                };
                let a1466 = f32(v3);
                let a1467 = 1u64;
                nat_rem(a1465, a1466, a1467)
            };
            let a1469 = 0u64;
            if nat_eq(a1468, a1469) {
                let a1474 = {
                    let a1471 = match v1 {
                        Adt1::C0(_, h1470, _, _, _, _, _, _, _, _, _) => {
                            h1470
                        }
                    };
                    let a1472 = f32(v3);
                    let a1473 = 1u64;
                    nat_rem(a1471, a1472, a1473)
                };
                let a1475 = 0u64;
                nat_eq(a1474, a1475)
            } else {
                false
            }
        } else {
            false
        } {
            f32(v3)
        } else {
            0u64
        }
    }
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1488 = {
        let a1484 = {
            let a1480 = {
                let a1476 = v0;
                let a1477 = v4;
                nat_mul(a1476, a1477)?
            };
            let a1481 = {
                let a1478 = v1;
                let a1479 = v5;
                nat_mul(a1478, a1479)?
            };
            nat_add(a1480, a1481)?
        };
        let a1485 = {
            let a1482 = v2;
            let a1483 = v6;
            nat_mul(a1482, a1483)?
        };
        nat_add(a1484, a1485)?
    };
    let a1489 = {
        let a1486 = v3;
        let a1487 = v7;
        nat_mul(a1486, a1487)?
    };
    nat_add(a1488, a1489)
}

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1490 = f73(v0, v1);
    let a1491 = 2u64;
    if nat_eq(a1490, a1491) {
        let a1495 = {
            let a1492 = v2;
            let a1493 = 2u64;
            let a1494 = 0u64;
            nat_rem(a1492, a1493, a1494)
        };
        let a1496 = 0u64;
        if nat_eq(a1495, a1496) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f27(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f7(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f27(v0), f7(v1))
    }
}

pub fn f77(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1497 = v0;
    let a1498 = 2u64;
    if if nat_eq(a1497, a1498) {
        let a1500 = match v1.clone() {
            Adt1::C0(h1499, _, _, _, _, _, _, _, _, _, _) => {
                h1499
            }
        };
        let a1501 = 0u64;
        if nat_eq(a1500, a1501) {
            let a1513 = {
                let a1503 = v2;
                let a1504 = match v1.clone() {
                    Adt1::C0(_, h1502, _, _, _, _, _, _, _, _, _) => {
                        h1502
                    }
                };
                let a1505 = 0u64;
                nat_rem(a1503, a1504, a1505)
            };
            let a1514 = {
                let a1511 = match v1.clone() {
                    Adt1::C0(_, h1506, _, _, _, _, _, _, _, _, _) => {
                        h1506
                    }
                };
                let a1512 = {
                    let a1508 = match v1 {
                        Adt1::C0(_, h1507, _, _, _, _, _, _, _, _, _) => {
                            h1507
                        }
                    };
                    let a1509 = 4u64;
                    let a1510 = 0u64;
                    nat_rem(a1508, a1509, a1510)
                };
                nat_sub(a1511, a1512)
            };
            nat_lt(a1513, a1514)
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
