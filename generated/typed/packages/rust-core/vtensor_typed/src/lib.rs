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
    C10(Adt2, Adt2),
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
                let a36 = 98u64;
                let a37 = match v30.clone() {
                    Adt2::C0(_, _, _, h35, _, _, _, _, _, _, _, _) => {
                        h35
                    }
                };
                if nat_le(a36, a37) {
                    f16(v29, v30)
                } else {
                    f17(v29, v30)
                }
            } else {
                let a39 = match v30.clone() {
                    Adt2::C0(_, _, _, h38, _, _, _, _, _, _, _, _) => {
                        h38
                    }
                };
                let a40 = 33u64;
                if if nat_eq(a39, a40) {
                    true
                } else {
                    let a42 = match v30.clone() {
                        Adt2::C0(_, _, _, h41, _, _, _, _, _, _, _, _) => {
                            h41
                        }
                    };
                    let a43 = 89u64;
                    nat_eq(a42, a43)
                } {
                    f18(v29, v30)
                } else {
                    let a45 = match v30.clone() {
                        Adt2::C0(_, _, _, h44, _, _, _, _, _, _, _, _) => {
                            h44
                        }
                    };
                    let a46 = 66u64;
                    if if nat_eq(a45, a46) {
                        true
                    } else {
                        let a48 = match v30.clone() {
                            Adt2::C0(_, _, _, h47, _, _, _, _, _, _, _, _) => {
                                h47
                            }
                        };
                        let a49 = 67u64;
                        nat_eq(a48, a49)
                    } {
                        f19(v29, v30)
                    } else {
                        let a51 = match v30.clone() {
                            Adt2::C0(_, _, _, h50, _, _, _, _, _, _, _, _) => {
                                h50
                            }
                        };
                        let a52 = 41u64;
                        if nat_eq(a51, a52) {
                            f20(v29, v30, v31)
                        } else {
                            let a54 = 98u64;
                            let a55 = match v30.clone() {
                                Adt2::C0(_, _, _, h53, _, _, _, _, _, _, _, _) => {
                                    h53
                                }
                            };
                            if nat_le(a54, a55) {
                                if f21(match v30.clone() {
                                    Adt2::C0(_, _, _, _, h56, _, _, _, _, _, _, _) => {
                                        h56
                                    }
                                }) {
                                    f22(v29, v30, v31)
                                } else {
                                    f23(v29, v30)
                                }
                            } else {
                                f24(v29, v30, v31)
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
    let a57 = v0;
    let a58 = 2u64;
    if nat_eq(a57, a58) {
        Some(0u64)
    } else {
        let a59 = v0;
        let a60 = 5u64;
        if nat_eq(a59, a60) {
            Some(1u64)
        } else {
            let a61 = v0;
            let a62 = 6u64;
            if nat_eq(a61, a62) {
                Some(2u64)
            } else {
                let a63 = v0;
                let a64 = 7u64;
                if nat_eq(a63, a64) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    if if f25(v1) {
        true
    } else {
        f25(v3)
    } {
        true
    } else {
        if if f26(v1, v2, v3) {
            true
        } else {
            let a65 = v1;
            let a66 = 43u64;
            if if nat_eq(a65, a66) {
                let a67 = v0;
                let a68 = 0u64;
                nat_eq(a67, a68)
            } else {
                false
            } {
                let a69 = v2;
                let a70 = 0u64;
                nat_eq(a69, a70)
            } else {
                false
            }
        } {
            false
        } else {
            let a71 = v1;
            let a72 = v3;
            if if nat_eq(a71, a72) {
                f27(v0, v1, v2)
            } else {
                false
            } {
                true
            } else {
                let a73 = v0;
                let a74 = 0u64;
                if if nat_eq(a73, a74) {
                    let a75 = f28(v1);
                    let a76 = 2u64;
                    nat_eq(a75, a76)
                } else {
                    false
                } {
                    f29(v3)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a79 = match v1.clone() {
        Adt1::C0(_, h77, _, _, _, _, _, _, _, _, _) => {
            h77
        }
    };
    let a80 = match v2.clone() {
        Adt1::C0(_, h78, _, _, _, _, _, _, _, _, _) => {
            h78
        }
    };
    if if if if if if if if if if if if if if nat_eq(a79, a80) {
        true
    } else {
        let a83 = match v1.clone() {
            Adt1::C0(h81, _, _, _, _, _, _, _, _, _, _) => {
                h81
            }
        };
        let a84 = match v3.clone() {
            Adt1::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                h82
            }
        };
        nat_eq(a83, a84)
    } {
        f30(v1.clone(), v2.clone())
    } else {
        false
    } {
        f5(v1.clone(), v3.clone())
    } else {
        false
    } {
        f31(v0, match v1.clone() {
            Adt1::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        }, v2.clone(), match v3.clone() {
            Adt1::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                h86
            }
        })
    } else {
        false
    } {
        f32(match v1.clone() {
            Adt1::C0(h87, _, _, _, _, _, _, _, _, _, _) => {
                h87
            }
        }, match v3.clone() {
            Adt1::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                h88
            }
        })
    } else {
        false
    } {
        let a90 = 0u64;
        let a91 = f7(match v3.clone() {
            Adt1::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                h89
            }
        });
        nat_lt(a90, a91)
    } else {
        false
    } {
        let a94 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h92, _, _, _, _, _) => {
                h92
            }
        };
        let a95 = f7(match v1.clone() {
            Adt1::C0(h93, _, _, _, _, _, _, _, _, _, _) => {
                h93
            }
        });
        nat_eq(a94, a95)
    } else {
        false
    } {
        let a98 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h96, _, _, _, _, _) => {
                h96
            }
        };
        let a99 = f7(match v3.clone() {
            Adt1::C0(h97, _, _, _, _, _, _, _, _, _, _) => {
                h97
            }
        });
        nat_eq(a98, a99)
    } else {
        false
    } {
        let a105 = {
            let a102 = match v1.clone() {
                Adt1::C0(_, h100, _, _, _, _, _, _, _, _, _) => {
                    h100
                }
            };
            let a103 = f33(match v1.clone() {
                Adt1::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                    h101
                }
            });
            let a104 = 1u64;
            nat_rem(a102, a103, a104)
        };
        let a106 = 0u64;
        nat_eq(a105, a106)
    } else {
        false
    } {
        let a112 = {
            let a109 = match v3.clone() {
                Adt1::C0(_, h107, _, _, _, _, _, _, _, _, _) => {
                    h107
                }
            };
            let a110 = f33(match v3.clone() {
                Adt1::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                    h108
                }
            });
            let a111 = 1u64;
            nat_rem(a109, a110, a111)
        };
        let a113 = 0u64;
        nat_eq(a112, a113)
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
        f34({
            let a120 = {
                let a116 = match v3.clone() {
                    Adt1::C0(_, h114, _, _, _, _, _, _, _, _, _) => {
                        h114
                    }
                };
                let a117 = f33(match v3.clone() {
                    Adt1::C0(h115, _, _, _, _, _, _, _, _, _, _) => {
                        h115
                    }
                });
                let a118 = 0u64;
                nat_quot(a116, a117, a118)
            };
            let a121 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h119, _, _, _, _, _) => {
                    h119
                }
            };
            nat_mul(a120, a121)?
        }, match v3.clone() {
            Adt1::C0(_, _, h122, _, _, _, _, _, _, _, _) => {
                h122
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h123, _, _, _, _, _, _, _) => {
                h123
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                h124
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h125, _, _, _, _) => {
                h125
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h126, _, _, _) => {
                h126
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h127, _, _) => {
                h127
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f35(v1.clone()) {
            0u64
        } else {
            let a133 = match v1.clone() {
                Adt1::C0(_, _, h128, _, _, _, _, _, _, _, _) => {
                    h128
                }
            };
            let a134 = {
                let a131 = match v1.clone() {
                    Adt1::C0(_, _, _, h129, _, _, _, _, _, _, _) => {
                        h129
                    }
                };
                let a132 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h130, _, _, _, _, _, _) => {
                        h130
                    }
                };
                nat_mul(a131, a132)?
            };
            nat_mul(a133, a134)?
        }, 3u64, f10(match v3.clone() {
            Adt1::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }), v0, 0u64, 0u64, 0u64, v1.clone(), v2, v3, true, match v1 {
            Adt1::C0(_, h136, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f5(v1.clone(), v3.clone()) {
        f36(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f35(v1.clone()) {
            true
        } else {
            let a139 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h137, _, _, _, _, _) => {
                    h137
                }
            };
            let a140 = f37(match v1.clone() {
                Adt1::C0(h138, _, _, _, _, _, _, _, _, _, _) => {
                    h138
                }
            });
            if if if if if if if nat_eq(a139, a140) {
                let a143 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h141, _, _, _, _, _) => {
                        h141
                    }
                };
                let a144 = f37(match v3.clone() {
                    Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
                        h142
                    }
                });
                nat_eq(a143, a144)
            } else {
                false
            } {
                f38(v1.clone())
            } else {
                false
            } {
                f38(v2.clone())
            } else {
                false
            } {
                f38(v3.clone())
            } else {
                false
            } {
                f39(v1.clone(), v2.clone())?
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
        let v4: u64 = if if f35(v1.clone()) {
            true
        } else {
            f40(v3.clone())?
        } {
            0u64
        } else {
            let a147 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h145, _) => {
                    h145
                }
            };
            let a148 = f37(match v3.clone() {
                Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _) => {
                    h146
                }
            });
            let a149 = 0u64;
            nat_quot(a147, a148, a149)
        };
        Ok(Adt3::C2(Adt2::C0(f41(f9(v1.clone())?, v4, f42(v0, match v1.clone() {
            Adt1::C0(h150, _, _, _, _, _, _, _, _, _, _) => {
                h150
            }
        }, match v2.clone() {
            Adt1::C0(h151, _, _, _, _, _, _, _, _, _, _) => {
                h151
            }
        }, match v3.clone() {
            Adt1::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
                h152
            }
        })?)?, {
            let a153 = 3u64;
            let a154 = v4;
            nat_add(a153, a154)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1) -> bool {
    let a157 = match v0.clone() {
        Adt1::C0(_, h155, _, _, _, _, _, _, _, _, _) => {
            h155
        }
    };
    let a158 = match v1.clone() {
        Adt1::C0(_, h156, _, _, _, _, _, _, _, _, _) => {
            h156
        }
    };
    if if if nat_eq(a157, a158) {
        let a161 = match v0.clone() {
            Adt1::C0(_, _, h159, _, _, _, _, _, _, _, _) => {
                h159
            }
        };
        let a162 = match v1.clone() {
            Adt1::C0(_, _, h160, _, _, _, _, _, _, _, _) => {
                h160
            }
        };
        nat_eq(a161, a162)
    } else {
        false
    } {
        let a165 = match v0.clone() {
            Adt1::C0(_, _, _, h163, _, _, _, _, _, _, _) => {
                h163
            }
        };
        let a166 = match v1.clone() {
            Adt1::C0(_, _, _, h164, _, _, _, _, _, _, _) => {
                h164
            }
        };
        nat_eq(a165, a166)
    } else {
        false
    } {
        let a169 = match v0 {
            Adt1::C0(_, _, _, _, h167, _, _, _, _, _, _) => {
                h167
            }
        };
        let a170 = match v1 {
            Adt1::C0(_, _, _, _, h168, _, _, _, _, _, _) => {
                h168
            }
        };
        nat_eq(a169, a170)
    } else {
        false
    }
}

pub fn f6(v0: Adt1) -> bool {
    let v1: u64 = match v0.clone() {
        Adt1::C0(h171, _, _, _, _, _, _, _, _, _, _) => {
            h171
        }
    };
    let a172 = 0u64;
    let a173 = f7(v1);
    if if nat_lt(a172, a173) {
        let a175 = f7(v1);
        let a176 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h174, _, _, _, _, _) => {
                h174
            }
        };
        nat_le(a175, a176)
    } else {
        false
    } {
        let a181 = {
            let a178 = match v0 {
                Adt1::C0(_, h177, _, _, _, _, _, _, _, _, _) => {
                    h177
                }
            };
            let a179 = f33(v1);
            let a180 = 1u64;
            nat_rem(a178, a179, a180)
        };
        let a182 = 0u64;
        nat_eq(a181, a182)
    } else {
        false
    }
}

pub fn f7(v0: u64) -> u64 {
    let m183 = f43(v0);
    match m183 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1(v1) => {
            match v1 {
                Adt5::C0(_, _, h184) => {
                    h184
                }
            }
        }
    }
}

pub fn f8(v0: Adt1, v1: Adt1) -> R<bool> {
    if f44(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f45(v0, v1))
    }
}

pub fn f9(v0: Adt1) -> R<u64> {
    let a193 = {
        let a187 = match v0.clone() {
            Adt1::C0(_, h185, _, _, _, _, _, _, _, _, _) => {
                h185
            }
        };
        let a188 = match v0.clone() {
            Adt1::C0(_, _, h186, _, _, _, _, _, _, _, _) => {
                h186
            }
        };
        nat_mul(a187, a188)?
    };
    let a194 = {
        let a191 = match v0.clone() {
            Adt1::C0(_, _, _, h189, _, _, _, _, _, _, _) => {
                h189
            }
        };
        let a192 = match v0 {
            Adt1::C0(_, _, _, _, h190, _, _, _, _, _, _) => {
                h190
            }
        };
        nat_mul(a191, a192)?
    };
    nat_mul(a193, a194)
}

pub fn f10(v0: u64) -> Option<u64> {
    let a195 = v0;
    let a196 = 16u64;
    if if if nat_eq(a195, a196) {
        true
    } else {
        let a197 = v0;
        let a198 = 17u64;
        nat_eq(a197, a198)
    } {
        true
    } else {
        let a199 = v0;
        let a200 = 19u64;
        nat_eq(a199, a200)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f11(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f46(v2, v3.clone()) {
        let a202 = match v0.clone() {
            Adt1::C0(h201, _, _, _, _, _, _, _, _, _, _) => {
                h201
            }
        };
        let a203 = 0u64;
        if nat_eq(a202, a203) {
            true
        } else {
            let a205 = match v0.clone() {
                Adt1::C0(h204, _, _, _, _, _, _, _, _, _, _) => {
                    h204
                }
            };
            let a206 = 1u64;
            nat_eq(a205, a206)
        }
    } else {
        false
    } {
        let a208 = 0u64;
        let a209 = f7(match v3.clone() {
            Adt1::C0(h207, _, _, _, _, _, _, _, _, _, _) => {
                h207
            }
        });
        nat_lt(a208, a209)
    } else {
        false
    } {
        let a211 = 0u64;
        let a212 = f47(match v1.clone() {
            Adt1::C0(h210, _, _, _, _, _, _, _, _, _, _) => {
                h210
            }
        });
        nat_lt(a211, a212)
    } else {
        false
    } {
        let a215 = match v0.clone() {
            Adt1::C0(_, h213, _, _, _, _, _, _, _, _, _) => {
                h213
            }
        };
        let a216 = match v3.clone() {
            Adt1::C0(_, h214, _, _, _, _, _, _, _, _, _) => {
                h214
            }
        };
        if nat_eq(a215, a216) {
            let a222 = {
                let a219 = match v0.clone() {
                    Adt1::C0(_, h217, _, _, _, _, _, _, _, _, _) => {
                        h217
                    }
                };
                let a220 = f33(match v3.clone() {
                    Adt1::C0(h218, _, _, _, _, _, _, _, _, _, _) => {
                        h218
                    }
                });
                let a221 = 1u64;
                nat_rem(a219, a220, a221)
            };
            let a223 = 0u64;
            nat_eq(a222, a223)
        } else {
            false
        }
    } else {
        false
    } {
        let a226 = match v0.clone() {
            Adt1::C0(_, _, _, h224, _, _, _, _, _, _, _) => {
                h224
            }
        };
        let a227 = match v3.clone() {
            Adt1::C0(_, _, _, h225, _, _, _, _, _, _, _) => {
                h225
            }
        };
        nat_eq(a226, a227)
    } else {
        false
    } {
        let a230 = match v0.clone() {
            Adt1::C0(_, _, _, _, h228, _, _, _, _, _, _) => {
                h228
            }
        };
        let a231 = match v3.clone() {
            Adt1::C0(_, _, _, _, h229, _, _, _, _, _, _) => {
                h229
            }
        };
        nat_eq(a230, a231)
    } else {
        false
    } {
        let a234 = match v0.clone() {
            Adt1::C0(_, _, h232, _, _, _, _, _, _, _, _) => {
                h232
            }
        };
        let a235 = match v1.clone() {
            Adt1::C0(_, h233, _, _, _, _, _, _, _, _, _) => {
                h233
            }
        };
        nat_eq(a234, a235)
    } else {
        false
    } {
        let a237 = match v1.clone() {
            Adt1::C0(_, _, _, _, h236, _, _, _, _, _, _) => {
                h236
            }
        };
        let a238 = 1u64;
        nat_eq(a237, a238)
    } else {
        false
    } {
        let a240 = 0u64;
        let a241 = match v1.clone() {
            Adt1::C0(_, _, h239, _, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_lt(a240, a241)
    } else {
        false
    } {
        let a243 = 0u64;
        let a244 = match v1.clone() {
            Adt1::C0(_, _, _, h242, _, _, _, _, _, _, _) => {
                h242
            }
        };
        nat_lt(a243, a244)
    } else {
        false
    } {
        let a250 = {
            let a247 = match v0.clone() {
                Adt1::C0(_, _, _, h245, _, _, _, _, _, _, _) => {
                    h245
                }
            };
            let a248 = match v1.clone() {
                Adt1::C0(_, _, h246, _, _, _, _, _, _, _, _) => {
                    h246
                }
            };
            let a249 = 0u64;
            nat_rem(a247, a248, a249)
        };
        let a251 = 0u64;
        nat_eq(a250, a251)
    } else {
        false
    } {
        let a257 = {
            let a254 = match v0.clone() {
                Adt1::C0(_, _, _, _, h252, _, _, _, _, _, _) => {
                    h252
                }
            };
            let a255 = match v1.clone() {
                Adt1::C0(_, _, _, h253, _, _, _, _, _, _, _) => {
                    h253
                }
            };
            let a256 = 0u64;
            nat_rem(a254, a255, a256)
        };
        let a258 = 0u64;
        nat_eq(a257, a258)
    } else {
        false
    } {
        f48(v0.clone())
    } else {
        false
    } {
        let a260 = match v3.clone() {
            Adt1::C0(_, h259, _, _, _, _, _, _, _, _, _) => {
                h259
            }
        };
        let a261 = 1u64;
        if nat_eq(a260, a261) {
            true
        } else {
            let a264 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h262, _, _, _, _, _) => {
                    h262
                }
            };
            let a265 = f7(match v3.clone() {
                Adt1::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
                    h263
                }
            });
            nat_eq(a264, a265)
        }
    } else {
        false
    } {
        let v4: u64 = f9(v1.clone())?;
        let v5: u64 = {
            let a271 = match v0.clone() {
                Adt1::C0(_, _, h266, _, _, _, _, _, _, _, _) => {
                    h266
                }
            };
            let a272 = {
                let a269 = match v0.clone() {
                    Adt1::C0(_, _, _, h267, _, _, _, _, _, _, _) => {
                        h267
                    }
                };
                let a270 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h268, _, _, _, _, _, _) => {
                        h268
                    }
                };
                nat_mul(a269, a270)?
            };
            nat_mul(a271, a272)?
        };
        let v6: u64 = if f35(v0.clone()) {
            0u64
        } else {
            let a280 = {
                let a273 = 2u64;
                let a274 = v4;
                nat_mul(a273, a274)?
            };
            let a281 = {
                let a278 = v5;
                let a279 = {
                    let a276 = 1u64;
                    let a277 = f49(v0.clone(), match v3.clone() {
                        Adt1::C0(h275, _, _, _, _, _, _, _, _, _, _) => {
                            h275
                        }
                    })?;
                    nat_add(a276, a277)?
                };
                nat_mul(a278, a279)?
            };
            nat_add(a280, a281)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f35(v0.clone()) {
            3u64
        } else {
            let a282 = 3u64;
            let a283 = v4;
            nat_add(a282, a283)?
        }, f10(match v3.clone() {
            Adt1::C0(h284, _, _, _, _, _, _, _, _, _, _) => {
                h284
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3.clone(), false, f50(match v3 {
            Adt1::C0(h285, _, _, _, _, _, _, _, _, _, _) => {
                h285
            }
        }))))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f51(v0.clone(), v2.clone());
    let v4: u64 = f9(v0.clone())?;
    if if if if if f46(v1.clone(), v2.clone()) {
        let a286 = v4;
        let a287 = f9(v2.clone())?;
        nat_eq(a286, a287)
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
        if f25(match v0.clone() {
            Adt1::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                h288
            }
        }) {
            true
        } else {
            f25(match v2.clone() {
                Adt1::C0(h289, _, _, _, _, _, _, _, _, _, _) => {
                    h289
                }
            })
        }
    } else {
        false
    } {
        let v5: u64 = {
            let a290 = 0u64;
            let a291 = v3;
            if nat_lt(a290, a291) {
                let a292 = v4;
                let a293 = v3;
                let a294 = 0u64;
                nat_quot(a292, a293, a294)
            } else {
                let a302 = {
                    let a296 = v4;
                    let a297 = match v0.clone() {
                        Adt1::C0(_, h295, _, _, _, _, _, _, _, _, _) => {
                            h295
                        }
                    };
                    let a298 = 0u64;
                    nat_quot(a296, a297, a298)
                };
                let a303 = {
                    let a299 = v4;
                    let a300 = f52(v2.clone());
                    let a301 = 0u64;
                    nat_quot(a299, a300, a301)
                };
                nat_add(a302, a303)?
            }
        };
        Ok(Adt3::C2(Adt2::C0(v5, 3u64, f10(match v2.clone() {
            Adt1::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
                h304
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false, {
            let a305 = 0u64;
            let a306 = v3;
            if nat_lt(a305, a306) {
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
    let a308 = match v2.clone() {
        Adt1::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
            h307
        }
    };
    let a309 = 0u64;
    if if if if if if if if if if if nat_eq(a308, a309) {
        let a311 = match v3.clone() {
            Adt1::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
                h310
            }
        };
        let a312 = 26u64;
        nat_eq(a311, a312)
    } else {
        false
    } {
        let a314 = match v2.clone() {
            Adt1::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                h313
            }
        };
        let a315 = 2147483647u64;
        nat_le(a314, a315)
    } else {
        false
    } {
        let a318 = match v3.clone() {
            Adt1::C0(_, h316, _, _, _, _, _, _, _, _, _) => {
                h316
            }
        };
        let a319 = match v2.clone() {
            Adt1::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                h317
            }
        };
        nat_le(a318, a319)
    } else {
        false
    } {
        let a322 = match v2.clone() {
            Adt1::C0(_, _, h320, _, _, _, _, _, _, _, _) => {
                h320
            }
        };
        let a323 = match v3.clone() {
            Adt1::C0(_, _, h321, _, _, _, _, _, _, _, _) => {
                h321
            }
        };
        nat_eq(a322, a323)
    } else {
        false
    } {
        let a326 = match v2.clone() {
            Adt1::C0(_, _, _, h324, _, _, _, _, _, _, _) => {
                h324
            }
        };
        let a327 = match v3.clone() {
            Adt1::C0(_, _, _, h325, _, _, _, _, _, _, _) => {
                h325
            }
        };
        nat_eq(a326, a327)
    } else {
        false
    } {
        let a330 = match v2.clone() {
            Adt1::C0(_, _, _, _, h328, _, _, _, _, _, _) => {
                h328
            }
        };
        let a331 = match v3.clone() {
            Adt1::C0(_, _, _, _, h329, _, _, _, _, _, _) => {
                h329
            }
        };
        nat_eq(a330, a331)
    } else {
        false
    } {
        let a332 = v0;
        let a333 = 67u64;
        if nat_eq(a332, a333) {
            true
        } else {
            let a334 = v1;
            let a335 = 1u64;
            if nat_le(a334, a335) {
                let a338 = match v3.clone() {
                    Adt1::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                        h336
                    }
                };
                let a339 = match v2.clone() {
                    Adt1::C0(_, h337, _, _, _, _, _, _, _, _, _) => {
                        h337
                    }
                };
                nat_eq(a338, a339)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f53(v2.clone())?
    } else {
        false
    } {
        f53(v3.clone())?
    } else {
        false
    } {
        f44(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a345 = match v3.clone() {
                Adt1::C0(_, _, h340, _, _, _, _, _, _, _, _) => {
                    h340
                }
            };
            let a346 = {
                let a343 = match v3.clone() {
                    Adt1::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                        h341
                    }
                };
                let a344 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h342, _, _, _, _, _, _) => {
                        h342
                    }
                };
                nat_mul(a343, a344)?
            };
            nat_mul(a345, a346)?
        }, 3u64, None::<u64>, v0, {
            let a347 = v0;
            let a348 = 67u64;
            if nat_eq(a347, a348) {
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
    let a349 = v0;
    let a350 = 6u64;
    nat_le(a349, a350)
}

pub fn f15(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a352 = 0u64;
    let a353 = f28(match v5.clone() {
        Adt1::C0(h351, _, _, _, _, _, _, _, _, _, _) => {
            h351
        }
    });
    if nat_lt(a352, a353) {
        f54(v0, v1, v2, v3, v4, v5, v6, v7)
    } else {
        let a354 = v2;
        let a355 = 1u64;
        if if if if if if if if if if if if if if if if nat_le(a354, a355) {
            let a358 = match v5.clone() {
                Adt1::C0(h356, _, _, _, _, _, _, _, _, _, _) => {
                    h356
                }
            };
            let a359 = match v7.clone() {
                Adt1::C0(h357, _, _, _, _, _, _, _, _, _, _) => {
                    h357
                }
            };
            nat_eq(a358, a359)
        } else {
            false
        } {
            let a362 = match v6.clone() {
                Adt1::C0(h360, _, _, _, _, _, _, _, _, _, _) => {
                    h360
                }
            };
            let a363 = match v5.clone() {
                Adt1::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
                    h361
                }
            };
            nat_eq(a362, a363)
        } else {
            false
        } {
            f5(v5.clone(), v6.clone())
        } else {
            false
        } {
            f6(v5.clone())
        } else {
            false
        } {
            f6(v6.clone())
        } else {
            false
        } {
            f6(v7.clone())
        } else {
            false
        } {
            let a366 = match v7.clone() {
                Adt1::C0(_, _, _, _, _, h364, _, _, _, _, _) => {
                    h364
                }
            };
            let a367 = f7(match v7.clone() {
                Adt1::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
                    h365
                }
            });
            nat_eq(a366, a367)
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
            let a370 = match v5.clone() {
                Adt1::C0(_, _, h368, _, _, _, _, _, _, _, _) => {
                    h368
                }
            };
            let a371 = match v7.clone() {
                Adt1::C0(_, _, h369, _, _, _, _, _, _, _, _) => {
                    h369
                }
            };
            nat_eq(a370, a371)
        } else {
            false
        } {
            let a374 = match v5.clone() {
                Adt1::C0(_, _, _, h372, _, _, _, _, _, _, _) => {
                    h372
                }
            };
            let a375 = match v7.clone() {
                Adt1::C0(_, _, _, h373, _, _, _, _, _, _, _) => {
                    h373
                }
            };
            nat_eq(a374, a375)
        } else {
            false
        } {
            let a378 = match v5.clone() {
                Adt1::C0(_, _, _, _, h376, _, _, _, _, _, _) => {
                    h376
                }
            };
            let a379 = match v7.clone() {
                Adt1::C0(_, _, _, _, h377, _, _, _, _, _, _) => {
                    h377
                }
            };
            nat_eq(a378, a379)
        } else {
            false
        } {
            let a386 = match v7.clone() {
                Adt1::C0(_, h380, _, _, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            let a387 = if v1 {
                match v5.clone() {
                    Adt1::C0(_, h381, _, _, _, _, _, _, _, _, _) => {
                        h381
                    }
                }
            } else {
                let a383 = match v5.clone() {
                    Adt1::C0(_, h382, _, _, _, _, _, _, _, _, _) => {
                        h382
                    }
                };
                let a384 = 2u64;
                let a385 = 0u64;
                nat_quot(a383, a384, a385)
            };
            nat_eq(a386, a387)
        } else {
            false
        } {
            let a389 = match v7.clone() {
                Adt1::C0(_, h388, _, _, _, _, _, _, _, _, _) => {
                    h388
                }
            };
            let a390 = 2147483647u64;
            nat_le(a389, a390)
        } else {
            false
        } {
            f34({
                let a397 = {
                    let a393 = match v7.clone() {
                        Adt1::C0(_, h391, _, _, _, _, _, _, _, _, _) => {
                            h391
                        }
                    };
                    let a394 = f33(match v7.clone() {
                        Adt1::C0(h392, _, _, _, _, _, _, _, _, _, _) => {
                            h392
                        }
                    });
                    let a395 = 0u64;
                    nat_quot(a393, a394, a395)
                };
                let a398 = match v7.clone() {
                    Adt1::C0(_, _, _, _, _, h396, _, _, _, _, _) => {
                        h396
                    }
                };
                nat_mul(a397, a398)?
            }, match v7.clone() {
                Adt1::C0(_, _, h399, _, _, _, _, _, _, _, _) => {
                    h399
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, h400, _, _, _, _, _, _, _) => {
                    h400
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, h401, _, _, _, _, _, _) => {
                    h401
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, h402, _, _, _, _) => {
                    h402
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h403, _, _, _) => {
                    h403
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h404, _, _) => {
                    h404
                }
            })?
        } else {
            false
        } {
            let v8: u64 = match v7.clone() {
                Adt1::C0(_, h405, _, _, _, _, _, _, _, _, _) => {
                    h405
                }
            };
            let v9: u64 = {
                let a406 = v8;
                let a407 = 4u64;
                nat_mul(a406, a407)?
            };
            let v10: Adt1 = Adt1::C0(0u64, v8, 1u64, 1u64, 1u64, 4u64, v9, v9, v9, v9, 0u64);
            Ok(Adt3::C10(Adt2::C0({
                let a408 = f9(v7.clone())?;
                let a409 = v8;
                let a410 = 0u64;
                nat_quot(a408, a409, a410)
            }, 3u64, f10(match v7.clone() {
                Adt1::C0(h411, _, _, _, _, _, _, _, _, _, _) => {
                    h411
                }
            }), if v1 {
                98u64
            } else {
                let a412 = 99u64;
                let a413 = v2;
                nat_add(a412, a413)?
            }, v0, v3, v4, v5, v6, v7, true, v8), f55(v0, true, 0u64, v3, v4, v10.clone(), v10.clone(), v10)?))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f16(v0: u64, v1: Adt2) -> R<Adt3> {
    let a415 = v0;
    let a416 = match v1.clone() {
        Adt2::C0(h414, _, _, _, _, _, _, _, _, _, _, _) => {
            h414
        }
    };
    if nat_lt(a415, a416) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h417, _, _, _, _) => {
                h417
            }
        };
        let v3: u64 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h418) => {
                h418
            }
        };
        let v4: u64 = {
            let a420 = v0;
            let a421 = match v2.clone() {
                Adt1::C0(_, h419, _, _, _, _, _, _, _, _, _) => {
                    h419
                }
            };
            nat_mul(a420, a421)?
        };
        let v5: u64 = match v1.clone() {
            Adt2::C0(_, _, _, h422, _, _, _, _, _, _, _, _) => {
                h422
            }
        };
        Ok(Adt3::C7(f56({
            let a425 = v4;
            let a426 = {
                let a423 = v5;
                let a424 = 100u64;
                if nat_eq(a423, a424) {
                    v3
                } else {
                    0u64
                }
            };
            nat_add(a425, a426)?
        }, v2)?, f56({
            let a429 = v4;
            let a430 = {
                let a427 = v5;
                let a428 = 99u64;
                if nat_eq(a427, a428) {
                    v3
                } else {
                    0u64
                }
            };
            nat_add(a429, a430)?
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h431, _, _, _) => {
                h431
            }
        })?, f56({
            let a432 = v0;
            let a433 = v3;
            nat_mul(a432, a433)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h434, _, _) => {
                h434
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f17(v0: u64, v1: Adt2) -> R<Adt3> {
    let a436 = v0;
    let a437 = match v1.clone() {
        Adt2::C0(h435, _, _, _, _, _, _, _, _, _, _, _) => {
            h435
        }
    };
    if nat_lt(a436, a437) {
        let v2: u64 = {
            let a441 = v0;
            let a442 = {
                let m440 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h439, _, _, _, _) => {
                        h439
                    }
                };
                match m440 {
                    Adt1::C0(_, h438, _, _, _, _, _, _, _, _, _) => {
                        h438
                    }
                }
            };
            nat_mul(a441, a442)?
        };
        Ok(Adt3::C7(f57(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h443, _, _, _, _) => {
                h443
            }
        })?, f58(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h444, _, _, _, _) => {
                h444
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h445, _, _, _) => {
                h445
            }
        })?, f57(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h446, _, _) => {
                h446
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f18(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h447, _, _, _, _) => {
            h447
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h448, _, _) => {
            h448
        }
    };
    let a450 = v0;
    let a451 = match v1.clone() {
        Adt2::C0(h449, _, _, _, _, _, _, _, _, _, _, _) => {
            h449
        }
    };
    if nat_lt(a450, a451) {
        let v4: u64 = {
            let a453 = match v1.clone() {
                Adt2::C0(_, _, _, h452, _, _, _, _, _, _, _, _) => {
                    h452
                }
            };
            let a454 = 89u64;
            if nat_eq(a453, a454) {
                match v2.clone() {
                    Adt1::C0(_, h455, _, _, _, _, _, _, _, _, _) => {
                        h455
                    }
                }
            } else {
                f51(v2.clone(), v3.clone())
            }
        };
        let a456 = 0u64;
        let a457 = v4;
        if nat_lt(a456, a457) {
            let v5: u64 = {
                let a458 = v0;
                let a459 = v4;
                nat_mul(a458, a459)?
            };
            Ok(Adt3::C6(f56(v5, v2)?, f56(v5, v3)?))
        } else {
            let v6: u64 = {
                let a462 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h460) => {
                        h460
                    }
                };
                let a463 = match v2.clone() {
                    Adt1::C0(_, h461, _, _, _, _, _, _, _, _, _) => {
                        h461
                    }
                };
                let a464 = 0u64;
                nat_quot(a462, a463, a464)
            };
            let a465 = v0;
            let a466 = v6;
            if nat_lt(a465, a466) {
                let v7: u64 = {
                    let a468 = v0;
                    let a469 = match v2.clone() {
                        Adt1::C0(_, h467, _, _, _, _, _, _, _, _, _) => {
                            h467
                        }
                    };
                    nat_mul(a468, a469)?
                };
                Ok(Adt3::C8(f56(v7, v2.clone())?, v7, match v2 {
                    Adt1::C0(_, h470, _, _, _, _, _, _, _, _, _) => {
                        h470
                    }
                }))
            } else {
                let v8: u64 = f52(v3.clone());
                let v9: u64 = {
                    let a473 = {
                        let a471 = v0;
                        let a472 = v6;
                        nat_sub(a471, a472)
                    };
                    let a474 = v8;
                    nat_mul(a473, a474)?
                };
                Ok(Adt3::C9(v9, f56(v9, v3)?, v8))
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f19(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h475, _, _, _, _) => {
            h475
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h476, _, _) => {
            h476
        }
    };
    let a478 = v0;
    let a479 = match v1.clone() {
        Adt2::C0(h477, _, _, _, _, _, _, _, _, _, _, _) => {
            h477
        }
    };
    if nat_lt(a478, a479) {
        Ok(Adt3::C5({
            let a481 = v0;
            let a482 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h480, _, _, _, _) => {
                    h480
                }
            };
            nat_mul(a481, a482)?
        }, {
            let a484 = v0;
            let a485 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h483, _, _, _, _) => {
                    h483
                }
            };
            nat_mul(a484, a485)?
        }, match v2 {
            Adt1::C0(_, h486, _, _, _, _, _, _, _, _, _) => {
                h486
            }
        }, match v3 {
            Adt1::C0(_, h487, _, _, _, _, _, _, _, _, _) => {
                h487
            }
        }, {
            let a489 = match v1 {
                Adt2::C0(_, _, _, _, h488, _, _, _, _, _, _, _) => {
                    h488
                }
            };
            let a490 = 1u64;
            nat_eq(a489, a490)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f20(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a492 = v0;
    let a493 = match v1.clone() {
        Adt2::C0(h491, _, _, _, _, _, _, _, _, _, _, _) => {
            h491
        }
    };
    if nat_lt(a492, a493) {
        let v3: u64 = {
            let a495 = 2u64;
            let a496 = f9(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h494, _, _, _) => {
                    h494
                }
            })?;
            nat_mul(a495, a496)?
        };
        let a497 = v0;
        let a498 = v3;
        if nat_lt(a497, a498) {
            f59(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h499, _, _, _) => {
                    h499
                }
            }, {
                let m502 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h501, _, _) => {
                        h501
                    }
                };
                match m502 {
                    Adt1::C0(_, _, h500, _, _, _, _, _, _, _, _) => {
                        h500
                    }
                }
            }, v2)
        } else {
            f60({
                let a503 = v0;
                let a504 = v3;
                nat_sub(a503, a504)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h505, _, _, _, _) => {
                    h505
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h506, _, _, _) => {
                    h506
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h507, _, _) => {
                    h507
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f21(v0: u64) -> bool {
    let a508 = v0;
    let a509 = 3u64;
    if nat_eq(a508, a509) {
        true
    } else {
        let a510 = v0;
        let a511 = 6u64;
        nat_eq(a510, a511)
    }
}

pub fn f22(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h512, _, _, _, _, _) => {
            h512
        }
    };
    let a514 = v0;
    let a515 = match v1.clone() {
        Adt2::C0(h513, _, _, _, _, _, _, _, _, _, _, _) => {
            h513
        }
    };
    if nat_lt(a514, a515) {
        let a516 = v0;
        let a517 = 0u64;
        if nat_eq(a516, a517) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a518 = v0;
                let a519 = 1u64;
                nat_sub(a518, a519)
            };
            let v5: u64 = {
                let a521 = v4;
                let a522 = f61(match v1.clone() {
                    Adt2::C0(_, _, _, _, h520, _, _, _, _, _, _, _) => {
                        h520
                    }
                });
                let a523 = 0u64;
                nat_rem(a521, a522, a523)
            };
            let v6: u64 = {
                let a528 = 4u64;
                let a529 = {
                    let a525 = v4;
                    let a526 = f61(match v1.clone() {
                        Adt2::C0(_, _, _, _, h524, _, _, _, _, _, _, _) => {
                            h524
                        }
                    });
                    let a527 = 0u64;
                    nat_quot(a525, a526, a527)
                };
                nat_mul(a528, a529)?
            };
            let a530 = v5;
            let a531 = 2u64;
            if nat_lt(a530, a531) {
                f23({
                    let a532 = v6;
                    let a533 = v5;
                    nat_add(a532, a533)?
                }, v1)
            } else {
                let a534 = v5;
                let a535 = 2u64;
                if nat_eq(a534, a535) {
                    Ok(f62(0u64, 2u64, v3))
                } else {
                    let a536 = v5;
                    let a537 = 3u64;
                    if nat_eq(a536, a537) {
                        Ok(f63(0u64, 2u64, v3, {
                            let a538 = v2;
                            let a539 = 2u64;
                            nat_eq(a538, a539)
                        }))
                    } else {
                        let a540 = v5;
                        let a541 = 4u64;
                        if nat_eq(a540, a541) {
                            Ok(f62(1u64, 1u64, 3u64))
                        } else {
                            let a542 = v5;
                            let a543 = 5u64;
                            if nat_eq(a542, a543) {
                                Ok(f63(1u64, 1u64, 3u64, {
                                    let a544 = v2;
                                    let a545 = 0u64;
                                    nat_eq(a544, a545)
                                }))
                            } else {
                                let a546 = v5;
                                let a547 = 6u64;
                                if nat_eq(a546, a547) {
                                    Ok(f62(1u64, 2u64, v3))
                                } else {
                                    let a548 = v5;
                                    let a549 = 7u64;
                                    if nat_eq(a548, a549) {
                                        Ok(f63(1u64, 2u64, v3, {
                                            let a550 = v2;
                                            let a551 = 2u64;
                                            nat_eq(a550, a551)
                                        }))
                                    } else {
                                        let a553 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h552, _, _, _, _, _, _, _) => {
                                                h552
                                            }
                                        };
                                        let a554 = 3u64;
                                        if nat_eq(a553, a554) {
                                            let a555 = v5;
                                            let a556 = 15u64;
                                            if nat_eq(a555, a556) {
                                                f23({
                                                    let a557 = v6;
                                                    let a558 = 3u64;
                                                    nat_add(a557, a558)?
                                                }, v1)
                                            } else {
                                                Ok(f64(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h559, _, _, _, _, _, _) => {
                                                        h559
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a560 = v5;
                                            let a561 = 8u64;
                                            if nat_eq(a560, a561) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m564 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h563, _, _) => {
                                                            h563
                                                        }
                                                    };
                                                    match m564 {
                                                        Adt1::C0(h562, _, _, _, _, _, _, _, _, _, _) => {
                                                            h562
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f23({
                                                    let a565 = v6;
                                                    let a566 = 3u64;
                                                    nat_add(a565, a566)?
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

pub fn f23(v0: u64, v1: Adt2) -> R<Adt3> {
    let a568 = v0;
    let a569 = match v1.clone() {
        Adt2::C0(h567, _, _, _, _, _, _, _, _, _, _, _) => {
            h567
        }
    };
    if nat_lt(a568, a569) {
        f65({
            let a570 = v0;
            let a571 = 4u64;
            let a572 = 0u64;
            nat_rem(a570, a571, a572)
        }, {
            let a573 = v0;
            let a574 = 4u64;
            let a575 = 0u64;
            nat_quot(a573, a574, a575)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f24(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h576, _, _, _, _) => {
            h576
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h577, _, _, _) => {
            h577
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h578, _, _) => {
            h578
        }
    };
    let v6: u64 = {
        let a580 = match v1.clone() {
            Adt2::C0(_, h579, _, _, _, _, _, _, _, _, _, _) => {
                h579
            }
        };
        let a581 = 3u64;
        if nat_eq(a580, a581) {
            0u64
        } else {
            let a582 = f9(v3.clone())?;
            let a583 = 2u64;
            nat_mul(a582, a583)?
        }
    };
    let a585 = match v1.clone() {
        Adt2::C0(h584, _, _, _, _, _, _, _, _, _, _, _) => {
            h584
        }
    };
    let a586 = v0;
    if nat_le(a585, a586) {
        Ok(Adt3::C1)
    } else {
        let a587 = v0;
        let a588 = v6;
        if nat_lt(a587, a588) {
            let v7: u64 = {
                let a589 = v0;
                let a590 = 2u64;
                let a591 = 0u64;
                nat_quot(a589, a590, a591)
            };
            let v8: u64 = {
                let a596 = 3u64;
                let a597 = {
                    let a593 = f57(v7, v5.clone())?;
                    let a594 = f37(match v5 {
                        Adt1::C0(h592, _, _, _, _, _, _, _, _, _, _) => {
                            h592
                        }
                    });
                    let a595 = 0u64;
                    nat_quot(a593, a594, a595)
                };
                nat_add(a596, a597)?
            };
            let a601 = {
                let a598 = v0;
                let a599 = 2u64;
                let a600 = 0u64;
                nat_rem(a598, a599, a600)
            };
            let a602 = 0u64;
            if nat_eq(a601, a602) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a603 = v2;
                let a604 = 0u64;
                if nat_eq(a603, a604) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a605 = v0;
                let a606 = v6;
                nat_sub(a605, a606)
            };
            if f66(match v3.clone() {
                Adt1::C0(h607, _, _, _, _, _, _, _, _, _, _) => {
                    h607
                }
            }) {
                f67(v9, v1)
            } else {
                let v10: u64 = f68(match v3.clone() {
                    Adt1::C0(h608, _, _, _, _, _, _, _, _, _, _) => {
                        h608
                    }
                }, match v4.clone() {
                    Adt1::C0(h609, _, _, _, _, _, _, _, _, _, _) => {
                        h609
                    }
                }, match v5.clone() {
                    Adt1::C0(h610, _, _, _, _, _, _, _, _, _, _) => {
                        h610
                    }
                })?;
                let v11: u64 = {
                    let a611 = v9;
                    let a612 = v10;
                    let a613 = 0u64;
                    nat_quot(a611, a612, a613)
                };
                let v12: u64 = {
                    let a614 = v9;
                    let a615 = v10;
                    let a616 = 0u64;
                    nat_rem(a614, a615, a616)
                };
                let v13: u64 = {
                    let a618 = match v3.clone() {
                        Adt1::C0(h617, _, _, _, _, _, _, _, _, _, _) => {
                            h617
                        }
                    };
                    let a619 = 0u64;
                    if nat_eq(a618, a619) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let v14: u64 = {
                    let a621 = match v4.clone() {
                        Adt1::C0(h620, _, _, _, _, _, _, _, _, _, _) => {
                            h620
                        }
                    };
                    let a622 = 0u64;
                    if nat_eq(a621, a622) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let a623 = v12;
                let a624 = v13;
                if nat_lt(a623, a624) {
                    Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                        Adt1::C0(h625, _, _, _, _, _, _, _, _, _, _) => {
                            h625
                        }
                    }, 0u64, 0u64, f57(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f37(match v3 {
                        Adt1::C0(h626, _, _, _, _, _, _, _, _, _, _) => {
                            h626
                        }
                    }), 4u64))
                } else {
                    let a629 = v12;
                    let a630 = {
                        let a627 = v13;
                        let a628 = v14;
                        nat_add(a627, a628)?
                    };
                    if nat_lt(a629, a630) {
                        Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                            Adt1::C0(h631, _, _, _, _, _, _, _, _, _, _) => {
                                h631
                            }
                        }, 0u64, 4u64, f58(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f37(match v4 {
                            Adt1::C0(h632, _, _, _, _, _, _, _, _, _, _) => {
                                h632
                            }
                        }), 4u64))
                    } else {
                        let a635 = v12;
                        let a636 = {
                            let a633 = v13;
                            let a634 = v14;
                            nat_add(a633, a634)?
                        };
                        if nat_eq(a635, a636) {
                            Ok(Adt3::C3(1u64, match v1 {
                                Adt2::C0(_, _, _, h637, _, _, _, _, _, _, _, _) => {
                                    h637
                                }
                            }, 0u64, 0u64, {
                                let a638 = v13;
                                let a639 = 0u64;
                                if nat_eq(a638, a639) {
                                    0u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a640 = v13;
                                let a641 = 0u64;
                                if nat_eq(a640, a641) {
                                    f57(v11, v3.clone())?
                                } else {
                                    0u64
                                }
                            }, {
                                let a642 = v14;
                                let a643 = 0u64;
                                if nat_eq(a642, a643) {
                                    4u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a644 = v14;
                                let a645 = 0u64;
                                if nat_eq(a644, a645) {
                                    f58(v11, v3, v4)?
                                } else {
                                    1u64
                                }
                            }, {
                                let a647 = match v5.clone() {
                                    Adt1::C0(h646, _, _, _, _, _, _, _, _, _, _) => {
                                        h646
                                    }
                                };
                                let a648 = 0u64;
                                if nat_eq(a647, a648) {
                                    3u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a650 = match v5.clone() {
                                    Adt1::C0(h649, _, _, _, _, _, _, _, _, _, _) => {
                                        h649
                                    }
                                };
                                let a651 = 0u64;
                                if nat_eq(a650, a651) {
                                    f57(v11, v5)?
                                } else {
                                    0u64
                                }
                            }, 4u64, 4u64))
                        } else {
                            Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                                Adt1::C0(h652, _, _, _, _, _, _, _, _, _, _) => {
                                    h652
                                }
                            }, 1u64, 0u64, 2u64, 0u64, 3u64, f57(v11, v5.clone())?, 4u64, f37(match v5 {
                                Adt1::C0(h653, _, _, _, _, _, _, _, _, _, _) => {
                                    h653
                                }
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> bool {
    let a654 = 6u64;
    let a655 = v0;
    if if if if nat_le(a654, a655) {
        let a656 = v0;
        let a657 = 23u64;
        nat_le(a656, a657)
    } else {
        false
    } {
        true
    } else {
        let a658 = v0;
        let a659 = 2u64;
        if nat_eq(a658, a659) {
            true
        } else {
            let a660 = v0;
            let a661 = 3u64;
            nat_eq(a660, a661)
        }
    } {
        true
    } else {
        let a662 = v0;
        let a663 = 29u64;
        nat_eq(a662, a663)
    } {
        true
    } else {
        let a664 = 34u64;
        let a665 = v0;
        if if nat_le(a664, a665) {
            let a666 = v0;
            let a667 = 35u64;
            nat_le(a666, a667)
        } else {
            false
        } {
            true
        } else {
            let a668 = 39u64;
            let a669 = v0;
            if nat_le(a668, a669) {
                let a670 = v0;
                let a671 = 42u64;
                nat_le(a670, a671)
            } else {
                false
            }
        }
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> bool {
    let a672 = v0;
    let a673 = 0u64;
    if if if nat_eq(a672, a673) {
        let a674 = v1;
        let a675 = 0u64;
        nat_eq(a674, a675)
    } else {
        false
    } {
        let a676 = v2;
        let a677 = 0u64;
        nat_eq(a676, a677)
    } else {
        false
    } {
        true
    } else {
        let a678 = v0;
        let a679 = 1u64;
        if if nat_eq(a678, a679) {
            true
        } else {
            let a680 = v0;
            let a681 = 30u64;
            nat_eq(a680, a681)
        } {
            let a682 = v1;
            let a683 = v0;
            if if nat_eq(a682, a683) {
                let a684 = v2;
                let a685 = v0;
                nat_eq(a684, a685)
            } else {
                false
            } {
                true
            } else {
                let a686 = v1;
                let a687 = 0u64;
                if nat_eq(a686, a687) {
                    let a688 = v2;
                    let a689 = v0;
                    if nat_eq(a688, a689) {
                        true
                    } else {
                        let a690 = v2;
                        let a691 = 0u64;
                        nat_eq(a690, a691)
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

pub fn f27(v0: u64, v1: u64, v2: u64) -> bool {
    let a692 = v1;
    let a693 = 27u64;
    if if if nat_eq(a692, a693) {
        true
    } else {
        let a694 = v1;
        let a695 = 28u64;
        nat_eq(a694, a695)
    } {
        let a696 = v1;
        let a697 = v2;
        nat_eq(a696, a697)
    } else {
        false
    } {
        let a698 = v0;
        let a699 = 3u64;
        nat_eq(a698, a699)
    } else {
        f29(v1)
    }
}

pub fn f28(v0: u64) -> u64 {
    let a700 = v0;
    let a701 = 0u64;
    if nat_eq(a700, a701) {
        4u64
    } else {
        let a702 = v0;
        let a703 = 1u64;
        if if nat_eq(a702, a703) {
            true
        } else {
            let a704 = v0;
            let a705 = 30u64;
            nat_eq(a704, a705)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f29(v0: u64) -> bool {
    let a706 = 0u64;
    let a707 = f69(v0);
    if nat_lt(a706, a707) {
        true
    } else {
        let a708 = 24u64;
        let a709 = v0;
        if nat_le(a708, a709) {
            let a710 = v0;
            let a711 = 28u64;
            nat_le(a710, a711)
        } else {
            false
        }
    }
}

pub fn f30(v0: Adt1, v1: Adt1) -> bool {
    if f35(v1.clone()) {
        f35(v0)
    } else {
        let a717 = {
            let a714 = match v0.clone() {
                Adt1::C0(_, h712, _, _, _, _, _, _, _, _, _) => {
                    h712
                }
            };
            let a715 = match v1.clone() {
                Adt1::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                    h713
                }
            };
            let a716 = 1u64;
            nat_rem(a714, a715, a716)
        };
        let a718 = 0u64;
        if if if nat_eq(a717, a718) {
            let a724 = {
                let a721 = match v0.clone() {
                    Adt1::C0(_, _, h719, _, _, _, _, _, _, _, _) => {
                        h719
                    }
                };
                let a722 = match v1.clone() {
                    Adt1::C0(_, _, h720, _, _, _, _, _, _, _, _) => {
                        h720
                    }
                };
                let a723 = 1u64;
                nat_rem(a721, a722, a723)
            };
            let a725 = 0u64;
            nat_eq(a724, a725)
        } else {
            false
        } {
            let a731 = {
                let a728 = match v0.clone() {
                    Adt1::C0(_, _, _, h726, _, _, _, _, _, _, _) => {
                        h726
                    }
                };
                let a729 = match v1.clone() {
                    Adt1::C0(_, _, _, h727, _, _, _, _, _, _, _) => {
                        h727
                    }
                };
                let a730 = 1u64;
                nat_rem(a728, a729, a730)
            };
            let a732 = 0u64;
            nat_eq(a731, a732)
        } else {
            false
        } {
            let a738 = {
                let a735 = match v0 {
                    Adt1::C0(_, _, _, _, h733, _, _, _, _, _, _) => {
                        h733
                    }
                };
                let a736 = match v1 {
                    Adt1::C0(_, _, _, _, h734, _, _, _, _, _, _) => {
                        h734
                    }
                };
                let a737 = 1u64;
                nat_rem(a735, a736, a737)
            };
            let a739 = 0u64;
            nat_eq(a738, a739)
        } else {
            false
        }
    }
}

pub fn f31(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h740, _, _, _, _, _, _, _, _, _, _) => {
            h740
        }
    };
    let a741 = v1;
    let a742 = v3;
    if if nat_eq(a741, a742) {
        true
    } else {
        let a743 = v0;
        let a744 = 0u64;
        if nat_eq(a743, a744) {
            let a745 = v4;
            let a746 = 0u64;
            if if nat_eq(a745, a746) {
                true
            } else {
                f25(v3)
            } {
                true
            } else {
                f29(v3)
            }
        } else {
            false
        }
    } {
        if if if f29(v4) {
            true
        } else {
            f25(v4)
        } {
            let a748 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h747, _, _, _, _, _) => {
                    h747
                }
            };
            let a749 = f7(v4);
            nat_eq(a748, a749)
        } else {
            false
        } {
            let a754 = {
                let a751 = match v2 {
                    Adt1::C0(_, h750, _, _, _, _, _, _, _, _, _) => {
                        h750
                    }
                };
                let a752 = f33(v4);
                let a753 = 1u64;
                nat_rem(a751, a752, a753)
            };
            let a755 = 0u64;
            nat_eq(a754, a755)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f32(v0: u64, v1: u64) -> bool {
    if f25(v0) {
        true
    } else {
        let a756 = v0;
        let a757 = v1;
        if if nat_eq(a756, a757) {
            f29(v0)
        } else {
            false
        } {
            true
        } else {
            let a758 = f28(v0);
            let a759 = 2u64;
            if nat_eq(a758, a759) {
                if f25(v1) {
                    true
                } else {
                    f29(v1)
                }
            } else {
                false
            }
        }
    }
}

pub fn f33(v0: u64) -> u64 {
    if f25(v0) {
        let m760 = f43(v0);
        match m760 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h761, _) => {
                        h761
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f70(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f70(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f70(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f70(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f70(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f70(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f35(v0: Adt1) -> bool {
    f71(match v0.clone() {
        Adt1::C0(_, h762, _, _, _, _, _, _, _, _, _) => {
            h762
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h763, _, _, _, _, _, _, _, _) => {
            h763
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h764, _, _, _, _, _, _, _) => {
            h764
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h765, _, _, _, _, _, _) => {
            h765
        }
    })
}

pub fn f36(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a767 = match v1.clone() {
        Adt1::C0(h766, _, _, _, _, _, _, _, _, _, _) => {
            h766
        }
    };
    let a768 = 43u64;
    if nat_eq(a767, a768) {
        let a769 = v0;
        let a770 = 0u64;
        if if if if if if nat_eq(a769, a770) {
            let a772 = match v2.clone() {
                Adt1::C0(h771, _, _, _, _, _, _, _, _, _, _) => {
                    h771
                }
            };
            let a773 = 0u64;
            nat_eq(a772, a773)
        } else {
            false
        } {
            let a775 = match v3.clone() {
                Adt1::C0(h774, _, _, _, _, _, _, _, _, _, _) => {
                    h774
                }
            };
            let a776 = 0u64;
            if nat_eq(a775, a776) {
                true
            } else {
                let a778 = match v3.clone() {
                    Adt1::C0(h777, _, _, _, _, _, _, _, _, _, _) => {
                        h777
                    }
                };
                let a779 = 43u64;
                nat_eq(a778, a779)
            }
        } else {
            false
        } {
            let a781 = match v3.clone() {
                Adt1::C0(h780, _, _, _, _, _, _, _, _, _, _) => {
                    h780
                }
            };
            let a782 = 43u64;
            if nat_eq(a781, a782) {
                f30(v1.clone(), v2.clone())
            } else {
                f5(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a784 = match v2 {
                Adt1::C0(_, _, _, _, _, h783, _, _, _, _, _) => {
                    h783
                }
            };
            let a785 = 4u64;
            nat_eq(a784, a785)
        } else {
            false
        } {
            let a788 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h786, _, _, _, _, _) => {
                    h786
                }
            };
            let a789 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h787, _, _, _, _) => {
                    h787
                }
            };
            if nat_le(a788, a789) {
                let a792 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h790, _, _, _, _) => {
                        h790
                    }
                };
                let a793 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h791, _, _, _) => {
                        h791
                    }
                };
                if nat_le(a792, a793) {
                    let a796 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h794, _, _, _) => {
                            h794
                        }
                    };
                    let a797 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h795, _, _) => {
                            h795
                        }
                    };
                    nat_le(a796, a797)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a805 = {
                let a803 = match v1.clone() {
                    Adt1::C0(_, _, h798, _, _, _, _, _, _, _, _) => {
                        h798
                    }
                };
                let a804 = {
                    let a801 = match v1.clone() {
                        Adt1::C0(_, _, _, h799, _, _, _, _, _, _, _) => {
                            h799
                        }
                    };
                    let a802 = match v1 {
                        Adt1::C0(_, _, _, _, h800, _, _, _, _, _, _) => {
                            h800
                        }
                    };
                    nat_mul(a801, a802)?
                };
                nat_mul(a803, a804)?
            };
            let a806 = 2147483647u64;
            Ok(nat_le(a805, a806))
        } else {
            Ok(false)
        }
    } else {
        if f30(v1.clone(), v2.clone()) {
            if f26(match v1.clone() {
                Adt1::C0(h807, _, _, _, _, _, _, _, _, _, _) => {
                    h807
                }
            }, match v2.clone() {
                Adt1::C0(h808, _, _, _, _, _, _, _, _, _, _) => {
                    h808
                }
            }, match v3.clone() {
                Adt1::C0(h809, _, _, _, _, _, _, _, _, _, _) => {
                    h809
                }
            }) {
                Ok(true)
            } else {
                let a810 = v0;
                let a811 = 3u64;
                if if if if nat_lt(a810, a811) {
                    f66(match v1.clone() {
                        Adt1::C0(h812, _, _, _, _, _, _, _, _, _, _) => {
                            h812
                        }
                    })
                } else {
                    false
                } {
                    let a815 = match v1.clone() {
                        Adt1::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
                            h813
                        }
                    };
                    let a816 = match v2.clone() {
                        Adt1::C0(h814, _, _, _, _, _, _, _, _, _, _) => {
                            h814
                        }
                    };
                    nat_eq(a815, a816)
                } else {
                    false
                } {
                    let a819 = match v1 {
                        Adt1::C0(h817, _, _, _, _, _, _, _, _, _, _) => {
                            h817
                        }
                    };
                    let a820 = match v3 {
                        Adt1::C0(h818, _, _, _, _, _, _, _, _, _, _) => {
                            h818
                        }
                    };
                    nat_eq(a819, a820)
                } else {
                    false
                } {
                    let a822 = match v2 {
                        Adt1::C0(_, _, _, _, _, h821, _, _, _, _, _) => {
                            h821
                        }
                    };
                    let a823 = 8u64;
                    Ok(nat_eq(a822, a823))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f37(v0: u64) -> u64 {
    if f66(v0) {
        8u64
    } else {
        f69(v0)
    }
}

pub fn f38(v0: Adt1) -> bool {
    let v1: u64 = f37(match v0.clone() {
        Adt1::C0(h824, _, _, _, _, _, _, _, _, _, _) => {
            h824
        }
    });
    let a829 = {
        let a826 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h825) => {
                h825
            }
        };
        let a827 = v1;
        let a828 = 1u64;
        nat_rem(a826, a827, a828)
    };
    let a830 = 0u64;
    if if nat_eq(a829, a830) {
        f72(match v0.clone() {
            Adt1::C0(_, h831, _, _, _, _, _, _, _, _, _) => {
                h831
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h832, _, _, _, _, _) => {
                h832
            }
        }, v1)
    } else {
        false
    } {
        f73(match v0.clone() {
            Adt1::C0(_, _, h833, _, _, _, _, _, _, _, _) => {
                h833
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h834, _, _, _, _, _, _, _) => {
                h834
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h835, _, _, _, _, _, _) => {
                h835
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h836, _, _, _, _) => {
                h836
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h837, _, _, _) => {
                h837
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h838, _, _) => {
                h838
            }
        }, v1)
    } else {
        false
    }
}

pub fn f39(v0: Adt1, v1: Adt1) -> R<bool> {
    let a846 = {
        let a844 = match v0.clone() {
            Adt1::C0(_, _, h839, _, _, _, _, _, _, _, _) => {
                h839
            }
        };
        let a845 = {
            let a842 = match v0.clone() {
                Adt1::C0(_, _, _, h840, _, _, _, _, _, _, _) => {
                    h840
                }
            };
            let a843 = match v0.clone() {
                Adt1::C0(_, _, _, _, h841, _, _, _, _, _, _) => {
                    h841
                }
            };
            nat_mul(a842, a843)?
        };
        nat_mul(a844, a845)?
    };
    let a847 = 9223372036854775807u64;
    if nat_le(a846, a847) {
        if f48(v1.clone()) {
            let a849 = match v1 {
                Adt1::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                    h848
                }
            };
            let a850 = 2147483647u64;
            Ok(nat_le(a849, a850))
        } else {
            let a852 = match v0 {
                Adt1::C0(_, h851, _, _, _, _, _, _, _, _, _) => {
                    h851
                }
            };
            let a853 = 2147483647u64;
            Ok(nat_le(a852, a853))
        }
    } else {
        Ok(false)
    }
}

pub fn f40(v0: Adt1) -> R<bool> {
    f34({
        let a856 = match v0.clone() {
            Adt1::C0(_, h854, _, _, _, _, _, _, _, _, _) => {
                h854
            }
        };
        let a857 = f37(match v0.clone() {
            Adt1::C0(h855, _, _, _, _, _, _, _, _, _, _) => {
                h855
            }
        });
        nat_mul(a856, a857)?
    }, match v0.clone() {
        Adt1::C0(_, _, h858, _, _, _, _, _, _, _, _) => {
            h858
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h859, _, _, _, _, _, _, _) => {
            h859
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h860, _, _, _, _, _, _) => {
            h860
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h861, _, _, _, _) => {
            h861
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h862, _, _, _) => {
            h862
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h863, _, _) => {
            h863
        }
    })
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a870 = {
        let a864 = v0;
        let a865 = v2;
        nat_mul(a864, a865)?
    };
    let a871 = {
        let a866 = v1;
        let a867 = 0u64;
        if nat_eq(a866, a867) {
            0u64
        } else {
            let a868 = v0;
            let a869 = 2u64;
            nat_mul(a868, a869)?
        }
    };
    nat_add(a870, a871)
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if f66(v1) {
        Ok(f74(v0, v1))
    } else {
        f68(v1, v2, v3)
    }
}

pub fn f43(v0: u64) -> Adt4 {
    let a872 = v0;
    let a873 = 0u64;
    if nat_eq(a872, a873) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a874 = v0;
        let a875 = 1u64;
        if nat_eq(a874, a875) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a876 = v0;
            let a877 = 2u64;
            if nat_eq(a876, a877) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a878 = v0;
                let a879 = 3u64;
                if nat_eq(a878, a879) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a880 = v0;
                    let a881 = 6u64;
                    if nat_eq(a880, a881) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a882 = v0;
                        let a883 = 7u64;
                        if nat_eq(a882, a883) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a884 = v0;
                            let a885 = 8u64;
                            if nat_eq(a884, a885) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a886 = v0;
                                let a887 = 9u64;
                                if nat_eq(a886, a887) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a888 = v0;
                                    let a889 = 10u64;
                                    if nat_eq(a888, a889) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a890 = v0;
                                        let a891 = 11u64;
                                        if nat_eq(a890, a891) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a892 = v0;
                                            let a893 = 12u64;
                                            if nat_eq(a892, a893) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a894 = v0;
                                                let a895 = 13u64;
                                                if nat_eq(a894, a895) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a896 = v0;
                                                    let a897 = 14u64;
                                                    if nat_eq(a896, a897) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a898 = v0;
                                                        let a899 = 15u64;
                                                        if nat_eq(a898, a899) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a900 = v0;
                                                            let a901 = 16u64;
                                                            if nat_eq(a900, a901) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a902 = v0;
                                                                let a903 = 17u64;
                                                                if nat_eq(a902, a903) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a904 = v0;
                                                                    let a905 = 18u64;
                                                                    if nat_eq(a904, a905) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a906 = v0;
                                                                        let a907 = 19u64;
                                                                        if nat_eq(a906, a907) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a908 = v0;
                                                                            let a909 = 20u64;
                                                                            if nat_eq(a908, a909) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a910 = v0;
                                                                                let a911 = 21u64;
                                                                                if nat_eq(a910, a911) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a912 = v0;
                                                                                    let a913 = 22u64;
                                                                                    if nat_eq(a912, a913) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a914 = v0;
                                                                                        let a915 = 23u64;
                                                                                        if nat_eq(a914, a915) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a916 = v0;
                                                                                            let a917 = 24u64;
                                                                                            if nat_eq(a916, a917) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a918 = v0;
                                                                                                let a919 = 25u64;
                                                                                                if nat_eq(a918, a919) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a920 = v0;
                                                                                                    let a921 = 26u64;
                                                                                                    if nat_eq(a920, a921) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a922 = v0;
                                                                                                        let a923 = 27u64;
                                                                                                        if nat_eq(a922, a923) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a924 = v0;
                                                                                                            let a925 = 28u64;
                                                                                                            if nat_eq(a924, a925) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a926 = v0;
                                                                                                                let a927 = 29u64;
                                                                                                                if nat_eq(a926, a927) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a928 = v0;
                                                                                                                    let a929 = 30u64;
                                                                                                                    if nat_eq(a928, a929) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a930 = v0;
                                                                                                                        let a931 = 34u64;
                                                                                                                        if nat_eq(a930, a931) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a932 = v0;
                                                                                                                            let a933 = 35u64;
                                                                                                                            if nat_eq(a932, a933) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a934 = v0;
                                                                                                                                let a935 = 39u64;
                                                                                                                                if nat_eq(a934, a935) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a936 = v0;
                                                                                                                                    let a937 = 40u64;
                                                                                                                                    if nat_eq(a936, a937) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a938 = v0;
                                                                                                                                        let a939 = 41u64;
                                                                                                                                        if nat_eq(a938, a939) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a940 = v0;
                                                                                                                                            let a941 = 42u64;
                                                                                                                                            if nat_eq(a940, a941) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a942 = v0;
                                                                                                                                                let a943 = 43u64;
                                                                                                                                                if nat_eq(a942, a943) {
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

pub fn f44(v0: Adt1, v1: Adt1) -> R<bool> {
    let a945 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h944, _) => {
            h944
        }
    };
    let a946 = 0u64;
    if nat_eq(a945, a946) {
        Ok(true)
    } else {
        let a948 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h947, _) => {
                h947
            }
        };
        let a949 = 0u64;
        if nat_eq(a948, a949) {
            Ok(true)
        } else {
            let a951 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h950) => {
                    h950
                }
            };
            let a952 = 0u64;
            if nat_eq(a951, a952) {
                Ok(true)
            } else {
                let a954 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h953) => {
                        h953
                    }
                };
                let a955 = 0u64;
                if nat_eq(a954, a955) {
                    Ok(true)
                } else {
                    let a961 = {
                        let a958 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h956) => {
                                h956
                            }
                        };
                        let a959 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h957, _) => {
                                h957
                            }
                        };
                        nat_add(a958, a959)?
                    };
                    let a962 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h960) => {
                            h960
                        }
                    };
                    if nat_le(a961, a962) {
                        Ok(true)
                    } else {
                        let a968 = {
                            let a965 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h963) => {
                                    h963
                                }
                            };
                            let a966 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h964, _) => {
                                    h964
                                }
                            };
                            nat_add(a965, a966)?
                        };
                        let a969 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h967) => {
                                h967
                            }
                        };
                        Ok(nat_le(a968, a969))
                    }
                }
            }
        }
    }
}

pub fn f45(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a972 = match v0.clone() {
            Adt1::C0(h970, _, _, _, _, _, _, _, _, _, _) => {
                h970
            }
        };
        let a973 = match v1.clone() {
            Adt1::C0(h971, _, _, _, _, _, _, _, _, _, _) => {
                h971
            }
        };
        nat_eq(a972, a973)
    } else {
        false
    } {
        let a976 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h974) => {
                h974
            }
        };
        let a977 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h975) => {
                h975
            }
        };
        nat_eq(a976, a977)
    } else {
        false
    } {
        let a980 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h978, _, _, _, _, _) => {
                h978
            }
        };
        let a981 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h979, _, _, _, _, _) => {
                h979
            }
        };
        nat_eq(a980, a981)
    } else {
        false
    } {
        let a984 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h982, _, _, _, _) => {
                h982
            }
        };
        let a985 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h983, _, _, _, _) => {
                h983
            }
        };
        nat_eq(a984, a985)
    } else {
        false
    } {
        let a988 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h986, _, _, _) => {
                h986
            }
        };
        let a989 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h987, _, _, _) => {
                h987
            }
        };
        nat_eq(a988, a989)
    } else {
        false
    } {
        let a992 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h990, _, _) => {
                h990
            }
        };
        let a993 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h991, _, _) => {
                h991
            }
        };
        nat_eq(a992, a993)
    } else {
        false
    }
}

pub fn f46(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a996 = match v0.clone() {
            Adt1::C0(h994, _, _, _, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a997 = match v1.clone() {
            Adt1::C0(h995, _, _, _, _, _, _, _, _, _, _) => {
                h995
            }
        };
        nat_eq(a996, a997)
    } else {
        false
    } {
        let a1000 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h998, _, _, _, _, _) => {
                h998
            }
        };
        let a1001 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h999, _, _, _, _, _) => {
                h999
            }
        };
        nat_eq(a1000, a1001)
    } else {
        false
    } {
        let a1004 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1002, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1003, _, _, _, _) => {
                h1003
            }
        };
        nat_eq(a1004, a1005)
    } else {
        false
    } {
        let a1008 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1006, _, _, _) => {
                h1006
            }
        };
        let a1009 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1007, _, _, _) => {
                h1007
            }
        };
        nat_eq(a1008, a1009)
    } else {
        false
    } {
        let a1012 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1010, _, _) => {
                h1010
            }
        };
        let a1013 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1011, _, _) => {
                h1011
            }
        };
        nat_eq(a1012, a1013)
    } else {
        false
    } {
        let a1015 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1014) => {
                h1014
            }
        };
        let a1016 = 0u64;
        if if nat_eq(a1015, a1016) {
            true
        } else {
            let a1018 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1017) => {
                    h1017
                }
            };
            let a1019 = 0u64;
            nat_eq(a1018, a1019)
        } {
            true
        } else {
            let a1022 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1020) => {
                    h1020
                }
            };
            let a1023 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1021) => {
                    h1021
                }
            };
            nat_eq(a1022, a1023)
        }
    } else {
        false
    }
}

pub fn f47(v0: u64) -> u64 {
    let a1024 = v0;
    let a1025 = 26u64;
    if nat_eq(a1024, a1025) {
        4u64
    } else {
        let a1026 = v0;
        let a1027 = 27u64;
        if nat_eq(a1026, a1027) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f48(v0: Adt1) -> bool {
    let a1029 = match v0.clone() {
        Adt1::C0(_, h1028, _, _, _, _, _, _, _, _, _) => {
            h1028
        }
    };
    let a1030 = 1u64;
    if nat_eq(a1029, a1030) {
        true
    } else {
        let a1033 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1031, _, _, _, _, _) => {
                h1031
            }
        };
        let a1034 = f69(match v0 {
            Adt1::C0(h1032, _, _, _, _, _, _, _, _, _, _) => {
                h1032
            }
        });
        nat_eq(a1033, a1034)
    }
}

pub fn f49(v0: Adt1, v1: u64) -> R<u64> {
    let a1040 = {
        let a1036 = match v0.clone() {
            Adt1::C0(_, h1035, _, _, _, _, _, _, _, _, _) => {
                h1035
            }
        };
        let a1037 = f33(v1);
        let a1038 = 0u64;
        nat_quot(a1036, a1037, a1038)
    };
    let a1041 = f75(match v0 {
        Adt1::C0(h1039, _, _, _, _, _, _, _, _, _, _) => {
            h1039
        }
    }, v1);
    nat_mul(a1040, a1041)
}

pub fn f50(v0: u64) -> u64 {
    if if f25(v0) {
        true
    } else {
        let a1042 = 24u64;
        let a1043 = v0;
        if nat_le(a1042, a1043) {
            let a1044 = v0;
            let a1045 = 28u64;
            nat_le(a1044, a1045)
        } else {
            false
        }
    } {
        f33(v0)
    } else {
        0u64
    }
}

pub fn f51(v0: Adt1, v1: Adt1) -> u64 {
    let a1048 = match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1046, _, _, _, _, _) => {
            h1046
        }
    };
    let a1049 = f7(match v1.clone() {
        Adt1::C0(h1047, _, _, _, _, _, _, _, _, _, _) => {
            h1047
        }
    });
    if nat_eq(a1048, a1049) {
        f76(v0, v1)
    } else {
        0u64
    }
}

pub fn f52(v0: Adt1) -> u64 {
    let a1052 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1050, _, _, _, _, _) => {
            h1050
        }
    };
    let a1053 = f7(match v0.clone() {
        Adt1::C0(h1051, _, _, _, _, _, _, _, _, _, _) => {
            h1051
        }
    });
    if nat_eq(a1052, a1053) {
        match v0 {
            Adt1::C0(_, h1054, _, _, _, _, _, _, _, _, _) => {
                h1054
            }
        }
    } else {
        f33(match v0 {
            Adt1::C0(h1055, _, _, _, _, _, _, _, _, _, _) => {
                h1055
            }
        })
    }
}

pub fn f53(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1057 = match v0.clone() {
            Adt1::C0(_, h1056, _, _, _, _, _, _, _, _, _) => {
                h1056
            }
        };
        let a1058 = 4u64;
        nat_mul(a1057, a1058)?
    };
    let v2: u64 = {
        let a1061 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1059, _, _, _, _) => {
                h1059
            }
        };
        let a1062 = match v0.clone() {
            Adt1::C0(_, _, h1060, _, _, _, _, _, _, _, _) => {
                h1060
            }
        };
        nat_mul(a1061, a1062)?
    };
    let v3: u64 = {
        let a1064 = v2;
        let a1065 = match v0.clone() {
            Adt1::C0(_, _, _, h1063, _, _, _, _, _, _, _) => {
                h1063
            }
        };
        nat_mul(a1064, a1065)?
    };
    let a1067 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1066, _, _, _, _, _) => {
            h1066
        }
    };
    let a1068 = 4u64;
    if if if if nat_eq(a1067, a1068) {
        let a1073 = {
            let a1070 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1069) => {
                    h1069
                }
            };
            let a1071 = 4u64;
            let a1072 = 1u64;
            nat_rem(a1070, a1071, a1072)
        };
        let a1074 = 0u64;
        nat_eq(a1073, a1074)
    } else {
        false
    } {
        let a1082 = {
            let a1080 = match v0.clone() {
                Adt1::C0(_, _, h1075, _, _, _, _, _, _, _, _) => {
                    h1075
                }
            };
            let a1081 = {
                let a1078 = match v0.clone() {
                    Adt1::C0(_, _, _, h1076, _, _, _, _, _, _, _) => {
                        h1076
                    }
                };
                let a1079 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h1077, _, _, _, _, _, _) => {
                        h1077
                    }
                };
                nat_mul(a1078, a1079)?
            };
            nat_mul(a1080, a1081)?
        };
        let a1083 = 1u64;
        if nat_le(a1082, a1083) {
            true
        } else {
            let a1085 = v1;
            let a1086 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1084, _, _, _, _) => {
                    h1084
                }
            };
            if nat_le(a1085, a1086) {
                let a1091 = {
                    let a1088 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h1087, _, _, _, _) => {
                            h1087
                        }
                    };
                    let a1089 = 4u64;
                    let a1090 = 1u64;
                    nat_rem(a1088, a1089, a1090)
                };
                let a1092 = 0u64;
                nat_eq(a1091, a1092)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a1094 = match v0.clone() {
            Adt1::C0(_, _, _, h1093, _, _, _, _, _, _, _) => {
                h1093
            }
        };
        let a1095 = 1u64;
        if nat_le(a1094, a1095) {
            true
        } else {
            let a1097 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1096, _, _, _) => {
                    h1096
                }
            };
            let a1098 = v2;
            nat_eq(a1097, a1098)
        }
    } else {
        false
    } {
        let a1100 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1099, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1101 = 1u64;
        if nat_le(a1100, a1101) {
            Ok(true)
        } else {
            let a1103 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h1102, _, _) => {
                    h1102
                }
            };
            let a1104 = v3;
            Ok(nat_eq(a1103, a1104))
        }
    } else {
        Ok(false)
    }
}

pub fn f54(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a1105 = v2;
    let a1106 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a1105, a1106) {
        let a1108 = 0u64;
        let a1109 = f28(match v5.clone() {
            Adt1::C0(h1107, _, _, _, _, _, _, _, _, _, _) => {
                h1107
            }
        });
        nat_lt(a1108, a1109)
    } else {
        false
    } {
        let a1112 = match v5.clone() {
            Adt1::C0(h1110, _, _, _, _, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1113 = match v6.clone() {
            Adt1::C0(h1111, _, _, _, _, _, _, _, _, _, _) => {
                h1111
            }
        };
        nat_eq(a1112, a1113)
    } else {
        false
    } {
        let a1116 = match v5.clone() {
            Adt1::C0(h1114, _, _, _, _, _, _, _, _, _, _) => {
                h1114
            }
        };
        let a1117 = match v7.clone() {
            Adt1::C0(h1115, _, _, _, _, _, _, _, _, _, _) => {
                h1115
            }
        };
        nat_eq(a1116, a1117)
    } else {
        false
    } {
        f5(v5.clone(), v6.clone())
    } else {
        false
    } {
        f77(v5.clone())?
    } else {
        false
    } {
        f77(v6.clone())?
    } else {
        false
    } {
        f77(v7.clone())?
    } else {
        false
    } {
        let a1120 = match v5.clone() {
            Adt1::C0(_, _, h1118, _, _, _, _, _, _, _, _) => {
                h1118
            }
        };
        let a1121 = match v7.clone() {
            Adt1::C0(_, _, h1119, _, _, _, _, _, _, _, _) => {
                h1119
            }
        };
        nat_eq(a1120, a1121)
    } else {
        false
    } {
        let a1124 = match v5.clone() {
            Adt1::C0(_, _, _, h1122, _, _, _, _, _, _, _) => {
                h1122
            }
        };
        let a1125 = match v7.clone() {
            Adt1::C0(_, _, _, h1123, _, _, _, _, _, _, _) => {
                h1123
            }
        };
        nat_eq(a1124, a1125)
    } else {
        false
    } {
        let a1128 = match v5.clone() {
            Adt1::C0(_, _, _, _, h1126, _, _, _, _, _, _) => {
                h1126
            }
        };
        let a1129 = match v7.clone() {
            Adt1::C0(_, _, _, _, h1127, _, _, _, _, _, _) => {
                h1127
            }
        };
        nat_eq(a1128, a1129)
    } else {
        false
    } {
        let a1136 = match v7.clone() {
            Adt1::C0(_, h1130, _, _, _, _, _, _, _, _, _) => {
                h1130
            }
        };
        let a1137 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h1131, _, _, _, _, _, _, _, _, _) => {
                    h1131
                }
            }
        } else {
            let a1133 = match v5.clone() {
                Adt1::C0(_, h1132, _, _, _, _, _, _, _, _, _) => {
                    h1132
                }
            };
            let a1134 = 2u64;
            let a1135 = 0u64;
            nat_quot(a1133, a1134, a1135)
        };
        nat_eq(a1136, a1137)
    } else {
        false
    } {
        let a1139 = match v7.clone() {
            Adt1::C0(_, h1138, _, _, _, _, _, _, _, _, _) => {
                h1138
            }
        };
        let a1140 = 2147483647u64;
        nat_le(a1139, a1140)
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
        Ok(Adt3::C2(f55(v0, v1, v2, v3, v4, v5, v6, v7)?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f55(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt2> {
    Ok(Adt2::C0(if f21(v0) {
        let a1143 = 1u64;
        let a1144 = {
            let a1141 = f61(v0);
            let a1142 = f9(v7.clone())?;
            nat_mul(a1141, a1142)?
        };
        nat_add(a1143, a1144)?
    } else {
        let a1145 = 4u64;
        let a1146 = f9(v7.clone())?;
        nat_mul(a1145, a1146)?
    }, if f21(v0) {
        4u64
    } else {
        3u64
    }, None::<u64>, if v1 {
        98u64
    } else {
        let a1147 = 99u64;
        let a1148 = v2;
        nat_add(a1147, a1148)?
    }, v0, v3, v4, v5, v6, v7, false, 0u64))
}

pub fn f56(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1150 = v0;
        let a1151 = match v1.clone() {
            Adt1::C0(_, h1149, _, _, _, _, _, _, _, _, _) => {
                h1149
            }
        };
        let a1152 = 0u64;
        nat_quot(a1150, a1151, a1152)
    };
    let v3: u64 = {
        let a1154 = v2;
        let a1155 = match v1.clone() {
            Adt1::C0(_, _, h1153, _, _, _, _, _, _, _, _) => {
                h1153
            }
        };
        let a1156 = 0u64;
        nat_quot(a1154, a1155, a1156)
    };
    let v4: u64 = {
        let a1162 = {
            let a1158 = v0;
            let a1159 = match v1.clone() {
                Adt1::C0(_, h1157, _, _, _, _, _, _, _, _, _) => {
                    h1157
                }
            };
            let a1160 = 0u64;
            nat_rem(a1158, a1159, a1160)
        };
        let a1163 = f33(match v1.clone() {
            Adt1::C0(h1161, _, _, _, _, _, _, _, _, _, _) => {
                h1161
            }
        });
        let a1164 = 0u64;
        nat_quot(a1162, a1163, a1164)
    };
    f78(v4, {
        let a1166 = v2;
        let a1167 = match v1.clone() {
            Adt1::C0(_, _, h1165, _, _, _, _, _, _, _, _) => {
                h1165
            }
        };
        let a1168 = 0u64;
        nat_rem(a1166, a1167, a1168)
    }, {
        let a1170 = v3;
        let a1171 = match v1.clone() {
            Adt1::C0(_, _, _, h1169, _, _, _, _, _, _, _) => {
                h1169
            }
        };
        let a1172 = 0u64;
        nat_rem(a1170, a1171, a1172)
    }, {
        let a1174 = v3;
        let a1175 = match v1.clone() {
            Adt1::C0(_, _, _, h1173, _, _, _, _, _, _, _) => {
                h1173
            }
        };
        let a1176 = 0u64;
        nat_quot(a1174, a1175, a1176)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1177, _, _, _, _, _) => {
            h1177
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1178, _, _, _, _) => {
            h1178
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1179, _, _, _) => {
            h1179
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1180, _, _) => {
            h1180
        }
    })
}

pub fn f57(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1182 = v0;
        let a1183 = match v1.clone() {
            Adt1::C0(_, h1181, _, _, _, _, _, _, _, _, _) => {
                h1181
            }
        };
        let a1184 = 0u64;
        nat_quot(a1182, a1183, a1184)
    };
    let v3: u64 = {
        let a1186 = v2;
        let a1187 = match v1.clone() {
            Adt1::C0(_, _, h1185, _, _, _, _, _, _, _, _) => {
                h1185
            }
        };
        let a1188 = 0u64;
        nat_quot(a1186, a1187, a1188)
    };
    f78({
        let a1190 = v0;
        let a1191 = match v1.clone() {
            Adt1::C0(_, h1189, _, _, _, _, _, _, _, _, _) => {
                h1189
            }
        };
        let a1192 = 0u64;
        nat_rem(a1190, a1191, a1192)
    }, {
        let a1194 = v2;
        let a1195 = match v1.clone() {
            Adt1::C0(_, _, h1193, _, _, _, _, _, _, _, _) => {
                h1193
            }
        };
        let a1196 = 0u64;
        nat_rem(a1194, a1195, a1196)
    }, {
        let a1198 = v3;
        let a1199 = match v1.clone() {
            Adt1::C0(_, _, _, h1197, _, _, _, _, _, _, _) => {
                h1197
            }
        };
        let a1200 = 0u64;
        nat_rem(a1198, a1199, a1200)
    }, {
        let a1202 = v3;
        let a1203 = match v1.clone() {
            Adt1::C0(_, _, _, h1201, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        let a1204 = 0u64;
        nat_quot(a1202, a1203, a1204)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1205, _, _, _, _, _) => {
            h1205
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1206, _, _, _, _) => {
            h1206
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1207, _, _, _) => {
            h1207
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1208, _, _) => {
            h1208
        }
    })
}

pub fn f58(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1210 = v0;
        let a1211 = match v1.clone() {
            Adt1::C0(_, h1209, _, _, _, _, _, _, _, _, _) => {
                h1209
            }
        };
        let a1212 = 0u64;
        nat_quot(a1210, a1211, a1212)
    };
    let v4: u64 = {
        let a1214 = v3;
        let a1215 = match v1.clone() {
            Adt1::C0(_, _, h1213, _, _, _, _, _, _, _, _) => {
                h1213
            }
        };
        let a1216 = 0u64;
        nat_quot(a1214, a1215, a1216)
    };
    f78({
        let a1222 = {
            let a1218 = v0;
            let a1219 = match v1.clone() {
                Adt1::C0(_, h1217, _, _, _, _, _, _, _, _, _) => {
                    h1217
                }
            };
            let a1220 = 0u64;
            nat_rem(a1218, a1219, a1220)
        };
        let a1223 = match v2.clone() {
            Adt1::C0(_, h1221, _, _, _, _, _, _, _, _, _) => {
                h1221
            }
        };
        let a1224 = 0u64;
        nat_rem(a1222, a1223, a1224)
    }, {
        let a1230 = {
            let a1226 = v3;
            let a1227 = match v1.clone() {
                Adt1::C0(_, _, h1225, _, _, _, _, _, _, _, _) => {
                    h1225
                }
            };
            let a1228 = 0u64;
            nat_rem(a1226, a1227, a1228)
        };
        let a1231 = match v2.clone() {
            Adt1::C0(_, _, h1229, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1232 = 0u64;
        nat_rem(a1230, a1231, a1232)
    }, {
        let a1238 = {
            let a1234 = v4;
            let a1235 = match v1.clone() {
                Adt1::C0(_, _, _, h1233, _, _, _, _, _, _, _) => {
                    h1233
                }
            };
            let a1236 = 0u64;
            nat_rem(a1234, a1235, a1236)
        };
        let a1239 = match v2.clone() {
            Adt1::C0(_, _, _, h1237, _, _, _, _, _, _, _) => {
                h1237
            }
        };
        let a1240 = 0u64;
        nat_rem(a1238, a1239, a1240)
    }, {
        let a1246 = {
            let a1242 = v4;
            let a1243 = match v1 {
                Adt1::C0(_, _, _, h1241, _, _, _, _, _, _, _) => {
                    h1241
                }
            };
            let a1244 = 0u64;
            nat_quot(a1242, a1243, a1244)
        };
        let a1247 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1245, _, _, _, _, _, _) => {
                h1245
            }
        };
        let a1248 = 0u64;
        nat_rem(a1246, a1247, a1248)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1249, _, _, _, _, _) => {
            h1249
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1250, _, _, _, _) => {
            h1250
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1251, _, _, _) => {
            h1251
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1252, _, _) => {
            h1252
        }
    })
}

pub fn f59(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1253 = v0;
        let a1254 = 2u64;
        let a1255 = 0u64;
        nat_quot(a1253, a1254, a1255)
    };
    let v5: u64 = f47(match v1.clone() {
        Adt1::C0(h1256, _, _, _, _, _, _, _, _, _, _) => {
            h1256
        }
    });
    let a1260 = {
        let a1257 = v0;
        let a1258 = 2u64;
        let a1259 = 0u64;
        nat_rem(a1257, a1258, a1259)
    };
    let a1261 = 0u64;
    if nat_eq(a1260, a1261) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f57(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1262 = v3;
        let a1263 = v2;
        if if nat_lt(a1262, a1263) {
            let a1266 = v3;
            let a1267 = {
                let a1264 = v5;
                let a1265 = 4u64;
                if nat_eq(a1264, a1265) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1266, a1267)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1268 = 3u64;
                let a1269 = v4;
                nat_add(a1268, a1269)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f60(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1271 = 1u64;
        let a1272 = f49(v1.clone(), match v3.clone() {
            Adt1::C0(h1270, _, _, _, _, _, _, _, _, _, _) => {
                h1270
            }
        })?;
        nat_add(a1271, a1272)?
    };
    let v6: u64 = {
        let a1273 = v0;
        let a1274 = v5;
        let a1275 = 0u64;
        nat_rem(a1273, a1274, a1275)
    };
    let v7: u64 = {
        let a1276 = v0;
        let a1277 = v5;
        let a1278 = 0u64;
        nat_quot(a1276, a1277, a1278)
    };
    let v8: u64 = {
        let a1280 = v7;
        let a1281 = match v1.clone() {
            Adt1::C0(_, _, h1279, _, _, _, _, _, _, _, _) => {
                h1279
            }
        };
        let a1282 = 0u64;
        nat_rem(a1280, a1281, a1282)
    };
    let v9: u64 = {
        let a1284 = v7;
        let a1285 = match v1.clone() {
            Adt1::C0(_, _, h1283, _, _, _, _, _, _, _, _) => {
                h1283
            }
        };
        let a1286 = 0u64;
        nat_quot(a1284, a1285, a1286)
    };
    let v10: u64 = {
        let a1288 = v9;
        let a1289 = match v1.clone() {
            Adt1::C0(_, _, _, h1287, _, _, _, _, _, _, _) => {
                h1287
            }
        };
        let a1290 = 0u64;
        nat_rem(a1288, a1289, a1290)
    };
    let v11: u64 = {
        let a1292 = v9;
        let a1293 = match v1.clone() {
            Adt1::C0(_, _, _, h1291, _, _, _, _, _, _, _) => {
                h1291
            }
        };
        let a1294 = 0u64;
        nat_quot(a1292, a1293, a1294)
    };
    let a1295 = v6;
    let a1296 = 0u64;
    if nat_eq(a1295, a1296) {
        let v12: u64 = {
            let a1313 = v8;
            let a1314 = {
                let a1311 = match v2.clone() {
                    Adt1::C0(_, h1297, _, _, _, _, _, _, _, _, _) => {
                        h1297
                    }
                };
                let a1312 = {
                    let a1309 = {
                        let a1299 = v10;
                        let a1300 = match v2.clone() {
                            Adt1::C0(_, _, h1298, _, _, _, _, _, _, _, _) => {
                                h1298
                            }
                        };
                        let a1301 = 0u64;
                        nat_rem(a1299, a1300, a1301)
                    };
                    let a1310 = {
                        let a1307 = match v2.clone() {
                            Adt1::C0(_, _, h1302, _, _, _, _, _, _, _, _) => {
                                h1302
                            }
                        };
                        let a1308 = {
                            let a1304 = v11;
                            let a1305 = match v2 {
                                Adt1::C0(_, _, _, h1303, _, _, _, _, _, _, _) => {
                                    h1303
                                }
                            };
                            let a1306 = 0u64;
                            nat_rem(a1304, a1305, a1306)
                        };
                        nat_mul(a1307, a1308)?
                    };
                    nat_add(a1309, a1310)?
                };
                nat_mul(a1311, a1312)?
            };
            nat_add(a1313, a1314)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1315 = 3u64;
            let a1316 = v12;
            nat_add(a1315, a1316)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1318 = v4;
        let a1319 = match v3.clone() {
            Adt1::C0(_, _, h1317, _, _, _, _, _, _, _, _) => {
                h1317
            }
        };
        if nat_lt(a1318, a1319) {
            let v13: u64 = {
                let a1324 = {
                    let a1320 = v6;
                    let a1321 = 1u64;
                    nat_sub(a1320, a1321)
                };
                let a1325 = f75(match v1.clone() {
                    Adt1::C0(h1322, _, _, _, _, _, _, _, _, _, _) => {
                        h1322
                    }
                }, match v3.clone() {
                    Adt1::C0(h1323, _, _, _, _, _, _, _, _, _, _) => {
                        h1323
                    }
                });
                let a1326 = 0u64;
                nat_quot(a1324, a1325, a1326)
            };
            let v14: u64 = f78({
                let a1328 = v13;
                let a1329 = f33(match v3.clone() {
                    Adt1::C0(h1327, _, _, _, _, _, _, _, _, _, _) => {
                        h1327
                    }
                });
                nat_mul(a1328, a1329)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1330, _, _, _, _, _) => {
                    h1330
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1331, _, _, _, _) => {
                    h1331
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1332, _, _, _) => {
                    h1332
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1333, _, _) => {
                    h1333
                }
            })?;
            let v15: u64 = f78(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1334, _, _, _, _, _) => {
                    h1334
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1335, _, _, _, _) => {
                    h1335
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1336, _, _, _) => {
                    h1336
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1337, _, _) => {
                    h1337
                }
            })?;
            let a1339 = 0u64;
            let a1340 = f50(match v3.clone() {
                Adt1::C0(h1338, _, _, _, _, _, _, _, _, _, _) => {
                    h1338
                }
            });
            if nat_lt(a1339, a1340) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f79(match v1 {
                    Adt1::C0(h1341, _, _, _, _, _, _, _, _, _, _) => {
                        h1341
                    }
                }, match v3 {
                    Adt1::C0(h1342, _, _, _, _, _, _, _, _, _, _) => {
                        h1342
                    }
                }, {
                    let a1343 = v6;
                    let a1344 = 1u64;
                    nat_sub(a1343, a1344)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f61(v0: u64) -> u64 {
    let a1345 = v0;
    let a1346 = 3u64;
    if nat_eq(a1345, a1346) {
        16u64
    } else {
        10u64
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f64(v0: u64, v1: u64) -> Adt3 {
    let a1347 = v0;
    let a1348 = 8u64;
    if nat_eq(a1347, a1348) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1349 = v0;
        let a1350 = 9u64;
        if nat_eq(a1349, a1350) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1351 = v0;
            let a1352 = 10u64;
            if nat_eq(a1351, a1352) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1353 = v0;
                let a1354 = 11u64;
                if nat_eq(a1353, a1354) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1355 = v0;
                    let a1356 = 12u64;
                    if nat_eq(a1355, a1356) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1357 = v0;
                        let a1358 = 13u64;
                        if nat_eq(a1357, a1358) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1359 = v0;
                            let a1360 = 14u64;
                            if nat_eq(a1359, a1360) {
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

pub fn f65(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1361, _, _, _, _) => {
            h1361
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1362, _, _, _) => {
            h1362
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1363, _, _) => {
            h1363
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1364, _, _, _, _, _, _, _, _) => {
            h1364
        }
    };
    let v7: u64 = {
        let a1366 = v1;
        let a1367 = match v5.clone() {
            Adt1::C0(_, h1365, _, _, _, _, _, _, _, _, _) => {
                h1365
            }
        };
        let a1368 = 0u64;
        nat_quot(a1366, a1367, a1368)
    };
    let v8: u64 = f28(match v5.clone() {
        Adt1::C0(h1369, _, _, _, _, _, _, _, _, _, _) => {
            h1369
        }
    });
    let v9: u64 = {
        let a1374 = {
            let a1371 = v1;
            let a1372 = match v5.clone() {
                Adt1::C0(_, h1370, _, _, _, _, _, _, _, _, _) => {
                    h1370
                }
            };
            let a1373 = 0u64;
            nat_rem(a1371, a1372, a1373)
        };
        let a1375 = v8;
        nat_mul(a1374, a1375)?
    };
    let v10: u64 = {
        let a1377 = match v5.clone() {
            Adt1::C0(_, h1376, _, _, _, _, _, _, _, _, _) => {
                h1376
            }
        };
        let a1378 = v8;
        nat_mul(a1377, a1378)?
    };
    let a1379 = v0;
    let a1380 = 0u64;
    if nat_eq(a1379, a1380) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1381, _, _, _, _, _, _, _, _, _, _) => {
                h1381
            }
        }, 0u64, 0u64, {
            let a1389 = {
                let a1385 = {
                    let a1383 = v7;
                    let a1384 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1382, _, _, _, _) => {
                            h1382
                        }
                    };
                    nat_mul(a1383, a1384)?
                };
                let a1386 = v9;
                nat_add(a1385, a1386)?
            };
            let a1390 = {
                let a1387 = v6;
                let a1388 = 100u64;
                if nat_eq(a1387, a1388) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1389, a1390)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1391 = v0;
        let a1392 = 1u64;
        if nat_eq(a1391, a1392) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1393, _, _, _, _, _, _, _, _, _, _) => {
                    h1393
                }
            }, 0u64, {
                let a1394 = v6;
                let a1395 = 98u64;
                if nat_eq(a1394, a1395) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1403 = {
                    let a1399 = {
                        let a1397 = v7;
                        let a1398 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1396, _, _, _, _) => {
                                h1396
                            }
                        };
                        nat_mul(a1397, a1398)?
                    };
                    let a1400 = v9;
                    nat_add(a1399, a1400)?
                };
                let a1404 = {
                    let a1401 = v6;
                    let a1402 = 99u64;
                    if nat_eq(a1401, a1402) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1403, a1404)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1405 = v0;
            let a1406 = 2u64;
            if nat_eq(a1405, a1406) {
                Ok(Adt3::C4(f80(match v2 {
                    Adt2::C0(_, _, _, _, h1407, _, _, _, _, _, _, _) => {
                        h1407
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1408, _, _, _, _, _, _, _, _, _, _) => {
                        h1408
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1409, _, _, _, _, _, _, _, _, _, _) => {
                        h1409
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1413 = {
                        let a1411 = v7;
                        let a1412 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1410, _, _, _, _) => {
                                h1410
                            }
                        };
                        nat_mul(a1411, a1412)?
                    };
                    let a1414 = v9;
                    nat_add(a1413, a1414)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f66(v0: u64) -> bool {
    let a1415 = v0;
    let a1416 = 27u64;
    if nat_eq(a1415, a1416) {
        true
    } else {
        let a1417 = v0;
        let a1418 = 28u64;
        nat_eq(a1417, a1418)
    }
}

pub fn f67(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1419, _, _, _, _) => {
            h1419
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1420, _, _, _) => {
            h1420
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1421, _, _) => {
            h1421
        }
    };
    let v5: u64 = match v1 {
        Adt2::C0(_, _, _, h1422, _, _, _, _, _, _, _, _) => {
            h1422
        }
    };
    let v6: u64 = f74(v5, match v2.clone() {
        Adt1::C0(h1423, _, _, _, _, _, _, _, _, _, _) => {
            h1423
        }
    });
    let v7: u64 = {
        let a1424 = v0;
        let a1425 = v6;
        let a1426 = 0u64;
        nat_quot(a1424, a1425, a1426)
    };
    let v8: u64 = f58(v7, v2.clone(), v3)?;
    let a1427 = v6;
    let a1428 = 2u64;
    if if nat_eq(a1427, a1428) {
        let a1432 = {
            let a1429 = v0;
            let a1430 = v6;
            let a1431 = 0u64;
            nat_rem(a1429, a1430, a1431)
        };
        let a1433 = 0u64;
        nat_eq(a1432, a1433)
    } else {
        false
    } {
        Ok(Adt3::C3(6u64, 15u64, 0u64, 0u64, 4u64, v8, 2u64, 9223372036854775808u64, 0u64, 0u64, 8u64, 8u64))
    } else {
        let v9: u64 = {
            let a1435 = match v2.clone() {
                Adt1::C0(h1434, _, _, _, _, _, _, _, _, _, _) => {
                    h1434
                }
            };
            let a1436 = 27u64;
            if nat_eq(a1435, a1436) {
                let a1437 = 15u64;
                let a1438 = v5;
                nat_add(a1437, a1438)?
            } else {
                let a1439 = v5;
                let a1440 = 2u64;
                if nat_eq(a1439, a1440) {
                    14u64
                } else {
                    0u64
                }
            }
        };
        Ok(Adt3::C3(6u64, v9, 0u64, 0u64, 0u64, f57(v7, v2)?, {
            let a1441 = v6;
            let a1442 = 2u64;
            if nat_eq(a1441, a1442) {
                1u64
            } else {
                4u64
            }
        }, {
            let a1443 = v6;
            let a1444 = 2u64;
            if nat_eq(a1443, a1444) {
                0u64
            } else {
                v8
            }
        }, 3u64, f57(v7, v4)?, 8u64, 8u64))
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1455 = {
        let a1451 = {
            let a1447 = 1u64;
            let a1448 = {
                let a1445 = v0;
                let a1446 = 0u64;
                if nat_eq(a1445, a1446) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a1447, a1448)?
        };
        let a1452 = {
            let a1449 = v1;
            let a1450 = 0u64;
            if nat_eq(a1449, a1450) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a1451, a1452)?
    };
    let a1456 = {
        let a1453 = v2;
        let a1454 = 0u64;
        if nat_eq(a1453, a1454) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a1455, a1456)
}

pub fn f69(v0: u64) -> u64 {
    let a1457 = v0;
    let a1458 = 43u64;
    if nat_eq(a1457, a1458) {
        1u64
    } else {
        f28(v0)
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1459 = v1;
    let a1460 = 1u64;
    if if if nat_le(a1459, a1460) {
        true
    } else {
        let a1461 = v0;
        let a1462 = v2;
        nat_le(a1461, a1462)
    } {
        let a1463 = v3;
        let a1464 = 1u64;
        if nat_le(a1463, a1464) {
            true
        } else {
            let a1471 = {
                let a1469 = v0;
                let a1470 = {
                    let a1467 = {
                        let a1465 = v1;
                        let a1466 = 1u64;
                        nat_sub(a1465, a1466)
                    };
                    let a1468 = v2;
                    nat_mul(a1467, a1468)?
                };
                nat_add(a1469, a1470)?
            };
            let a1472 = v4;
            nat_le(a1471, a1472)
        }
    } else {
        false
    } {
        let a1473 = v5;
        let a1474 = 1u64;
        if nat_le(a1473, a1474) {
            Ok(true)
        } else {
            let a1487 = {
                let a1485 = {
                    let a1479 = v0;
                    let a1480 = {
                        let a1477 = {
                            let a1475 = v1;
                            let a1476 = 1u64;
                            nat_sub(a1475, a1476)
                        };
                        let a1478 = v2;
                        nat_mul(a1477, a1478)?
                    };
                    nat_add(a1479, a1480)?
                };
                let a1486 = {
                    let a1483 = {
                        let a1481 = v3;
                        let a1482 = 1u64;
                        nat_sub(a1481, a1482)
                    };
                    let a1484 = v4;
                    nat_mul(a1483, a1484)?
                };
                nat_add(a1485, a1486)?
            };
            let a1488 = v6;
            Ok(nat_le(a1487, a1488))
        }
    } else {
        Ok(false)
    }
}

pub fn f71(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1489 = v0;
    let a1490 = 0u64;
    if if if nat_eq(a1489, a1490) {
        true
    } else {
        let a1491 = v1;
        let a1492 = 0u64;
        nat_eq(a1491, a1492)
    } {
        true
    } else {
        let a1493 = v2;
        let a1494 = 0u64;
        nat_eq(a1493, a1494)
    } {
        true
    } else {
        let a1495 = v3;
        let a1496 = 0u64;
        nat_eq(a1495, a1496)
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64) -> bool {
    let a1497 = v0;
    let a1498 = 1u64;
    if nat_le(a1497, a1498) {
        true
    } else {
        let a1502 = {
            let a1499 = v1;
            let a1500 = v2;
            let a1501 = 1u64;
            nat_rem(a1499, a1500, a1501)
        };
        let a1503 = 0u64;
        nat_eq(a1502, a1503)
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f72(v0, v3, v6) {
        f72(v1, v4, v6)
    } else {
        false
    } {
        f72(v2, v5, v6)
    } else {
        false
    }
}

pub fn f74(v0: u64, v1: u64) -> u64 {
    let a1504 = v1;
    let a1505 = 28u64;
    if if nat_eq(a1504, a1505) {
        let a1506 = v0;
        let a1507 = 1u64;
        nat_eq(a1506, a1507)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f75(v0: u64, v1: u64) -> u64 {
    let a1508 = v0;
    let a1509 = 1u64;
    if if nat_eq(a1508, a1509) {
        let a1510 = v1;
        let a1511 = 43u64;
        nat_eq(a1510, a1511)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f76(v0: Adt1, v1: Adt1) -> u64 {
    let v2: u64 = match v0.clone() {
        Adt1::C0(h1512, _, _, _, _, _, _, _, _, _, _) => {
            h1512
        }
    };
    let v3: u64 = match v1.clone() {
        Adt1::C0(h1513, _, _, _, _, _, _, _, _, _, _) => {
            h1513
        }
    };
    if if if f5(v0.clone(), v1.clone()) {
        if f25(v2) {
            true
        } else {
            f25(v3)
        }
    } else {
        false
    } {
        if if if f25(v2) {
            true
        } else {
            f29(v2)
        } {
            if f25(v3) {
                true
            } else {
                f29(v3)
            }
        } else {
            false
        } {
            let a1518 = {
                let a1515 = match v0.clone() {
                    Adt1::C0(_, h1514, _, _, _, _, _, _, _, _, _) => {
                        h1514
                    }
                };
                let a1516 = f33(v2);
                let a1517 = 1u64;
                nat_rem(a1515, a1516, a1517)
            };
            let a1519 = 0u64;
            if nat_eq(a1518, a1519) {
                let a1524 = {
                    let a1521 = match v1.clone() {
                        Adt1::C0(_, h1520, _, _, _, _, _, _, _, _, _) => {
                            h1520
                        }
                    };
                    let a1522 = f33(v3);
                    let a1523 = 1u64;
                    nat_rem(a1521, a1522, a1523)
                };
                let a1525 = 0u64;
                nat_eq(a1524, a1525)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        let a1527 = match v0.clone() {
            Adt1::C0(_, h1526, _, _, _, _, _, _, _, _, _) => {
                h1526
            }
        };
        let a1528 = 0u64;
        if nat_eq(a1527, a1528) {
            f33(v3)
        } else {
            match v0 {
                Adt1::C0(_, h1529, _, _, _, _, _, _, _, _, _) => {
                    h1529
                }
            }
        }
    } else {
        if if if f25(v3) {
            let a1530 = 0u64;
            let a1531 = f28(v2);
            nat_lt(a1530, a1531)
        } else {
            false
        } {
            let a1536 = {
                let a1533 = match v0 {
                    Adt1::C0(_, h1532, _, _, _, _, _, _, _, _, _) => {
                        h1532
                    }
                };
                let a1534 = f33(v3);
                let a1535 = 1u64;
                nat_rem(a1533, a1534, a1535)
            };
            let a1537 = 0u64;
            if nat_eq(a1536, a1537) {
                let a1542 = {
                    let a1539 = match v1 {
                        Adt1::C0(_, h1538, _, _, _, _, _, _, _, _, _) => {
                            h1538
                        }
                    };
                    let a1540 = f33(v3);
                    let a1541 = 1u64;
                    nat_rem(a1539, a1540, a1541)
                };
                let a1543 = 0u64;
                nat_eq(a1542, a1543)
            } else {
                false
            }
        } else {
            false
        } {
            f33(v3)
        } else {
            0u64
        }
    }
}

pub fn f77(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1546 = match v0.clone() {
            Adt1::C0(_, _, h1544, _, _, _, _, _, _, _, _) => {
                h1544
            }
        };
        let a1547 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1545, _, _, _, _) => {
                h1545
            }
        };
        nat_mul(a1546, a1547)?
    };
    if if f48(v0.clone()) {
        let a1549 = match v0.clone() {
            Adt1::C0(_, _, _, h1548, _, _, _, _, _, _, _) => {
                h1548
            }
        };
        let a1550 = 1u64;
        if nat_eq(a1549, a1550) {
            true
        } else {
            let a1552 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1551, _, _, _) => {
                    h1551
                }
            };
            let a1553 = v1;
            nat_eq(a1552, a1553)
        }
    } else {
        false
    } {
        let a1555 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1554, _, _, _, _, _, _) => {
                h1554
            }
        };
        let a1556 = 1u64;
        if nat_eq(a1555, a1556) {
            Ok(true)
        } else {
            let a1561 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1557, _, _) => {
                    h1557
                }
            };
            let a1562 = {
                let a1559 = v1;
                let a1560 = match v0 {
                    Adt1::C0(_, _, _, h1558, _, _, _, _, _, _, _) => {
                        h1558
                    }
                };
                nat_mul(a1559, a1560)?
            };
            Ok(nat_eq(a1561, a1562))
        }
    } else {
        Ok(false)
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1575 = {
        let a1571 = {
            let a1567 = {
                let a1563 = v0;
                let a1564 = v4;
                nat_mul(a1563, a1564)?
            };
            let a1568 = {
                let a1565 = v1;
                let a1566 = v5;
                nat_mul(a1565, a1566)?
            };
            nat_add(a1567, a1568)?
        };
        let a1572 = {
            let a1569 = v2;
            let a1570 = v6;
            nat_mul(a1569, a1570)?
        };
        nat_add(a1571, a1572)?
    };
    let a1576 = {
        let a1573 = v3;
        let a1574 = v7;
        nat_mul(a1573, a1574)?
    };
    nat_add(a1575, a1576)
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1577 = f75(v0, v1);
    let a1578 = 2u64;
    if nat_eq(a1577, a1578) {
        let a1582 = {
            let a1579 = v2;
            let a1580 = 2u64;
            let a1581 = 0u64;
            nat_rem(a1579, a1580, a1581)
        };
        let a1583 = 0u64;
        if nat_eq(a1582, a1583) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f28(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f7(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f28(v0), f7(v1))
    }
}

pub fn f80(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1584 = v0;
    let a1585 = 2u64;
    if if if nat_eq(a1584, a1585) {
        true
    } else {
        let a1586 = v0;
        let a1587 = 5u64;
        nat_eq(a1586, a1587)
    } {
        let a1589 = match v1.clone() {
            Adt1::C0(h1588, _, _, _, _, _, _, _, _, _, _) => {
                h1588
            }
        };
        let a1590 = 0u64;
        if nat_eq(a1589, a1590) {
            let a1602 = {
                let a1592 = v2;
                let a1593 = match v1.clone() {
                    Adt1::C0(_, h1591, _, _, _, _, _, _, _, _, _) => {
                        h1591
                    }
                };
                let a1594 = 0u64;
                nat_rem(a1592, a1593, a1594)
            };
            let a1603 = {
                let a1600 = match v1.clone() {
                    Adt1::C0(_, h1595, _, _, _, _, _, _, _, _, _) => {
                        h1595
                    }
                };
                let a1601 = {
                    let a1597 = match v1 {
                        Adt1::C0(_, h1596, _, _, _, _, _, _, _, _, _) => {
                            h1596
                        }
                    };
                    let a1598 = 4u64;
                    let a1599 = 0u64;
                    nat_rem(a1597, a1598, a1599)
                };
                nat_sub(a1600, a1601)
            };
            nat_lt(a1602, a1603)
        } else {
            false
        }
    } else {
        false
    } {
        let a1604 = v0;
        let a1605 = 2u64;
        if nat_eq(a1604, a1605) {
            7u64
        } else {
            8u64
        }
    } else {
        v0
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
