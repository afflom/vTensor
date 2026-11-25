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
    C11(Option<u64>),
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
        Adt0::C1(v30, v31, v32) => {
            if match v31.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h37, _) => {
                    h37
                }
            } {
                let a39 = match v31.clone() {
                    Adt2::C0(_, _, _, h38, _, _, _, _, _, _, _, _) => {
                        h38
                    }
                };
                let a40 = 20u64;
                if nat_eq(a39, a40) {
                    f17(v30, v31)
                } else {
                    let a42 = 98u64;
                    let a43 = match v31.clone() {
                        Adt2::C0(_, _, _, h41, _, _, _, _, _, _, _, _) => {
                            h41
                        }
                    };
                    if nat_le(a42, a43) {
                        f18(v30, v31)
                    } else {
                        f19(v30, v31)
                    }
                }
            } else {
                let a45 = match v31.clone() {
                    Adt2::C0(_, _, _, h44, _, _, _, _, _, _, _, _) => {
                        h44
                    }
                };
                let a46 = 33u64;
                if if nat_eq(a45, a46) {
                    true
                } else {
                    let a48 = match v31.clone() {
                        Adt2::C0(_, _, _, h47, _, _, _, _, _, _, _, _) => {
                            h47
                        }
                    };
                    let a49 = 89u64;
                    nat_eq(a48, a49)
                } {
                    f20(v30, v31)
                } else {
                    let a51 = match v31.clone() {
                        Adt2::C0(_, _, _, h50, _, _, _, _, _, _, _, _) => {
                            h50
                        }
                    };
                    let a52 = 66u64;
                    if if nat_eq(a51, a52) {
                        true
                    } else {
                        let a54 = match v31.clone() {
                            Adt2::C0(_, _, _, h53, _, _, _, _, _, _, _, _) => {
                                h53
                            }
                        };
                        let a55 = 67u64;
                        nat_eq(a54, a55)
                    } {
                        f21(v30, v31)
                    } else {
                        let a57 = match v31.clone() {
                            Adt2::C0(_, _, _, h56, _, _, _, _, _, _, _, _) => {
                                h56
                            }
                        };
                        let a58 = 41u64;
                        if nat_eq(a57, a58) {
                            f22(v30, v31, v32)
                        } else {
                            let a60 = 98u64;
                            let a61 = match v31.clone() {
                                Adt2::C0(_, _, _, h59, _, _, _, _, _, _, _, _) => {
                                    h59
                                }
                            };
                            if nat_le(a60, a61) {
                                if f23(match v31.clone() {
                                    Adt2::C0(_, _, _, _, h62, _, _, _, _, _, _, _) => {
                                        h62
                                    }
                                }) {
                                    f24(v30, v31, v32)
                                } else {
                                    f25(v30, v31)
                                }
                            } else {
                                f26(v30, v31, v32)
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
            let a24 = 20u64;
            if nat_eq(a23, a24) {
                if v22 {
                    Ok(Adt3::C1)
                } else {
                    let m25 = v24;
                    match m25 {
                        None => {
                            f12(v23, v25)
                        }
                        Some(_) => {
                            Ok(Adt3::C1)
                        }
                    }
                }
            } else {
                let a26 = v17;
                let a27 = 33u64;
                if nat_eq(a26, a27) {
                    if v22 {
                        Ok(Adt3::C1)
                    } else {
                        let m28 = v24;
                        match m28 {
                            None => {
                                Ok(Adt3::C1)
                            }
                            Some(v27) => {
                                f13(v23, v27, v25)
                            }
                        }
                    }
                } else {
                    let a29 = v17;
                    let a30 = 66u64;
                    if if nat_eq(a29, a30) {
                        true
                    } else {
                        let a31 = v17;
                        let a32 = 67u64;
                        nat_eq(a31, a32)
                    } {
                        if v22 {
                            Ok(Adt3::C1)
                        } else {
                            let m33 = v24;
                            match m33 {
                                None => {
                                    f14(v17, v18, v23, v25)
                                }
                                Some(_) => {
                                    Ok(Adt3::C1)
                                }
                            }
                        }
                    } else {
                        let a34 = v17;
                        let a35 = 98u64;
                        if nat_eq(a34, a35) {
                            if if f15(v18) {
                                !v22
                            } else {
                                false
                            } {
                                let m36 = v24;
                                match m36 {
                                    None => {
                                        f16(v18, false, v19, v20, v21, v23.clone(), v23, v25)
                                    }
                                    Some(v29) => {
                                        f16(v18, true, v19, v20, v21, v23, v29, v25)
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
    let a63 = v0;
    let a64 = 2u64;
    if nat_eq(a63, a64) {
        Some(0u64)
    } else {
        let a65 = v0;
        let a66 = 5u64;
        if nat_eq(a65, a66) {
            Some(1u64)
        } else {
            let a67 = v0;
            let a68 = 6u64;
            if nat_eq(a67, a68) {
                Some(2u64)
            } else {
                let a69 = v0;
                let a70 = 7u64;
                if nat_eq(a69, a70) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    if if f27(v1) {
        true
    } else {
        f27(v3)
    } {
        true
    } else {
        if if f28(v1, v2, v3) {
            true
        } else {
            let a71 = v1;
            let a72 = 43u64;
            if if nat_eq(a71, a72) {
                let a73 = v0;
                let a74 = 0u64;
                nat_eq(a73, a74)
            } else {
                false
            } {
                let a75 = v2;
                let a76 = 0u64;
                nat_eq(a75, a76)
            } else {
                false
            }
        } {
            false
        } else {
            let a77 = v1;
            let a78 = v3;
            if if nat_eq(a77, a78) {
                f29(v0, v1, v2)
            } else {
                false
            } {
                true
            } else {
                let a79 = v0;
                let a80 = 0u64;
                if if nat_eq(a79, a80) {
                    let a81 = f30(v1);
                    let a82 = 2u64;
                    nat_eq(a81, a82)
                } else {
                    false
                } {
                    f31(v3)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a85 = match v1.clone() {
        Adt1::C0(_, h83, _, _, _, _, _, _, _, _, _) => {
            h83
        }
    };
    let a86 = match v2.clone() {
        Adt1::C0(_, h84, _, _, _, _, _, _, _, _, _) => {
            h84
        }
    };
    if if if if if if if if if if if if if if nat_eq(a85, a86) {
        true
    } else {
        let a89 = match v1.clone() {
            Adt1::C0(h87, _, _, _, _, _, _, _, _, _, _) => {
                h87
            }
        };
        let a90 = match v3.clone() {
            Adt1::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                h88
            }
        };
        nat_eq(a89, a90)
    } {
        f32(v1.clone(), v2.clone())
    } else {
        false
    } {
        f5(v1.clone(), v3.clone())
    } else {
        false
    } {
        f33(v0, match v1.clone() {
            Adt1::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
                h91
            }
        }, v2.clone(), match v3.clone() {
            Adt1::C0(h92, _, _, _, _, _, _, _, _, _, _) => {
                h92
            }
        })
    } else {
        false
    } {
        f34(match v1.clone() {
            Adt1::C0(h93, _, _, _, _, _, _, _, _, _, _) => {
                h93
            }
        }, match v3.clone() {
            Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                h94
            }
        })
    } else {
        false
    } {
        let a96 = 0u64;
        let a97 = f7(match v3.clone() {
            Adt1::C0(h95, _, _, _, _, _, _, _, _, _, _) => {
                h95
            }
        });
        nat_lt(a96, a97)
    } else {
        false
    } {
        let a100 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h98, _, _, _, _, _) => {
                h98
            }
        };
        let a101 = f7(match v1.clone() {
            Adt1::C0(h99, _, _, _, _, _, _, _, _, _, _) => {
                h99
            }
        });
        nat_eq(a100, a101)
    } else {
        false
    } {
        let a104 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h102, _, _, _, _, _) => {
                h102
            }
        };
        let a105 = f7(match v3.clone() {
            Adt1::C0(h103, _, _, _, _, _, _, _, _, _, _) => {
                h103
            }
        });
        nat_eq(a104, a105)
    } else {
        false
    } {
        let a111 = {
            let a108 = match v1.clone() {
                Adt1::C0(_, h106, _, _, _, _, _, _, _, _, _) => {
                    h106
                }
            };
            let a109 = f35(match v1.clone() {
                Adt1::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
                    h107
                }
            });
            let a110 = 1u64;
            nat_rem(a108, a109, a110)
        };
        let a112 = 0u64;
        nat_eq(a111, a112)
    } else {
        false
    } {
        let a118 = {
            let a115 = match v3.clone() {
                Adt1::C0(_, h113, _, _, _, _, _, _, _, _, _) => {
                    h113
                }
            };
            let a116 = f35(match v3.clone() {
                Adt1::C0(h114, _, _, _, _, _, _, _, _, _, _) => {
                    h114
                }
            });
            let a117 = 1u64;
            nat_rem(a115, a116, a117)
        };
        let a119 = 0u64;
        nat_eq(a118, a119)
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
        f36({
            let a126 = {
                let a122 = match v3.clone() {
                    Adt1::C0(_, h120, _, _, _, _, _, _, _, _, _) => {
                        h120
                    }
                };
                let a123 = f35(match v3.clone() {
                    Adt1::C0(h121, _, _, _, _, _, _, _, _, _, _) => {
                        h121
                    }
                });
                let a124 = 0u64;
                nat_quot(a122, a123, a124)
            };
            let a127 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h125, _, _, _, _, _) => {
                    h125
                }
            };
            nat_mul(a126, a127)?
        }, match v3.clone() {
            Adt1::C0(_, _, h128, _, _, _, _, _, _, _, _) => {
                h128
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h129, _, _, _, _, _, _, _) => {
                h129
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h130, _, _, _, _, _, _) => {
                h130
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h131, _, _, _, _) => {
                h131
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h132, _, _, _) => {
                h132
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h133, _, _) => {
                h133
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f37(v1.clone()) {
            0u64
        } else {
            let a139 = match v1.clone() {
                Adt1::C0(_, _, h134, _, _, _, _, _, _, _, _) => {
                    h134
                }
            };
            let a140 = {
                let a137 = match v1.clone() {
                    Adt1::C0(_, _, _, h135, _, _, _, _, _, _, _) => {
                        h135
                    }
                };
                let a138 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h136, _, _, _, _, _, _) => {
                        h136
                    }
                };
                nat_mul(a137, a138)?
            };
            nat_mul(a139, a140)?
        }, 3u64, f10(match v3.clone() {
            Adt1::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                h141
            }
        }), v0, 0u64, 0u64, 0u64, v1.clone(), v2, v3, true, match v1 {
            Adt1::C0(_, h142, _, _, _, _, _, _, _, _, _) => {
                h142
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f5(v1.clone(), v3.clone()) {
        f38(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f37(v1.clone()) {
            true
        } else {
            let a145 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h143, _, _, _, _, _) => {
                    h143
                }
            };
            let a146 = f39(match v1.clone() {
                Adt1::C0(h144, _, _, _, _, _, _, _, _, _, _) => {
                    h144
                }
            });
            if if if if if if if nat_eq(a145, a146) {
                let a149 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h147, _, _, _, _, _) => {
                        h147
                    }
                };
                let a150 = f39(match v3.clone() {
                    Adt1::C0(h148, _, _, _, _, _, _, _, _, _, _) => {
                        h148
                    }
                });
                nat_eq(a149, a150)
            } else {
                false
            } {
                f40(v1.clone())
            } else {
                false
            } {
                f40(v2.clone())
            } else {
                false
            } {
                f40(v3.clone())
            } else {
                false
            } {
                f41(v1.clone(), v2.clone())?
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
        let v4: u64 = if if f37(v1.clone()) {
            true
        } else {
            f42(v3.clone())?
        } {
            0u64
        } else {
            let a153 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h151, _) => {
                    h151
                }
            };
            let a154 = f39(match v3.clone() {
                Adt1::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
                    h152
                }
            });
            let a155 = 0u64;
            nat_quot(a153, a154, a155)
        };
        Ok(Adt3::C2(Adt2::C0(f43(f9(v1.clone())?, v4, f44(v0, match v1.clone() {
            Adt1::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                h156
            }
        }, match v2.clone() {
            Adt1::C0(h157, _, _, _, _, _, _, _, _, _, _) => {
                h157
            }
        }, match v3.clone() {
            Adt1::C0(h158, _, _, _, _, _, _, _, _, _, _) => {
                h158
            }
        })?)?, {
            let a159 = 3u64;
            let a160 = v4;
            nat_add(a159, a160)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1) -> bool {
    let a163 = match v0.clone() {
        Adt1::C0(_, h161, _, _, _, _, _, _, _, _, _) => {
            h161
        }
    };
    let a164 = match v1.clone() {
        Adt1::C0(_, h162, _, _, _, _, _, _, _, _, _) => {
            h162
        }
    };
    if if if nat_eq(a163, a164) {
        let a167 = match v0.clone() {
            Adt1::C0(_, _, h165, _, _, _, _, _, _, _, _) => {
                h165
            }
        };
        let a168 = match v1.clone() {
            Adt1::C0(_, _, h166, _, _, _, _, _, _, _, _) => {
                h166
            }
        };
        nat_eq(a167, a168)
    } else {
        false
    } {
        let a171 = match v0.clone() {
            Adt1::C0(_, _, _, h169, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a172 = match v1.clone() {
            Adt1::C0(_, _, _, h170, _, _, _, _, _, _, _) => {
                h170
            }
        };
        nat_eq(a171, a172)
    } else {
        false
    } {
        let a175 = match v0 {
            Adt1::C0(_, _, _, _, h173, _, _, _, _, _, _) => {
                h173
            }
        };
        let a176 = match v1 {
            Adt1::C0(_, _, _, _, h174, _, _, _, _, _, _) => {
                h174
            }
        };
        nat_eq(a175, a176)
    } else {
        false
    }
}

pub fn f6(v0: Adt1) -> bool {
    let v1: u64 = match v0.clone() {
        Adt1::C0(h177, _, _, _, _, _, _, _, _, _, _) => {
            h177
        }
    };
    let a178 = 0u64;
    let a179 = f7(v1);
    if if nat_lt(a178, a179) {
        let a181 = f7(v1);
        let a182 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h180, _, _, _, _, _) => {
                h180
            }
        };
        nat_le(a181, a182)
    } else {
        false
    } {
        let a187 = {
            let a184 = match v0 {
                Adt1::C0(_, h183, _, _, _, _, _, _, _, _, _) => {
                    h183
                }
            };
            let a185 = f35(v1);
            let a186 = 1u64;
            nat_rem(a184, a185, a186)
        };
        let a188 = 0u64;
        nat_eq(a187, a188)
    } else {
        false
    }
}

pub fn f7(v0: u64) -> u64 {
    let m189 = f45(v0);
    match m189 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1(v1) => {
            match v1 {
                Adt5::C0(_, _, h190) => {
                    h190
                }
            }
        }
    }
}

pub fn f8(v0: Adt1, v1: Adt1) -> R<bool> {
    if f46(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f47(v0, v1))
    }
}

pub fn f9(v0: Adt1) -> R<u64> {
    let a199 = {
        let a193 = match v0.clone() {
            Adt1::C0(_, h191, _, _, _, _, _, _, _, _, _) => {
                h191
            }
        };
        let a194 = match v0.clone() {
            Adt1::C0(_, _, h192, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        nat_mul(a193, a194)?
    };
    let a200 = {
        let a197 = match v0.clone() {
            Adt1::C0(_, _, _, h195, _, _, _, _, _, _, _) => {
                h195
            }
        };
        let a198 = match v0 {
            Adt1::C0(_, _, _, _, h196, _, _, _, _, _, _) => {
                h196
            }
        };
        nat_mul(a197, a198)?
    };
    nat_mul(a199, a200)
}

pub fn f10(v0: u64) -> Option<u64> {
    let a201 = v0;
    let a202 = 16u64;
    if if if nat_eq(a201, a202) {
        true
    } else {
        let a203 = v0;
        let a204 = 17u64;
        nat_eq(a203, a204)
    } {
        true
    } else {
        let a205 = v0;
        let a206 = 19u64;
        nat_eq(a205, a206)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f11(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f48(v2, v3.clone()) {
        let a208 = match v0.clone() {
            Adt1::C0(h207, _, _, _, _, _, _, _, _, _, _) => {
                h207
            }
        };
        let a209 = 0u64;
        if nat_eq(a208, a209) {
            true
        } else {
            let a211 = match v0.clone() {
                Adt1::C0(h210, _, _, _, _, _, _, _, _, _, _) => {
                    h210
                }
            };
            let a212 = 1u64;
            nat_eq(a211, a212)
        }
    } else {
        false
    } {
        let a214 = 0u64;
        let a215 = f7(match v3.clone() {
            Adt1::C0(h213, _, _, _, _, _, _, _, _, _, _) => {
                h213
            }
        });
        nat_lt(a214, a215)
    } else {
        false
    } {
        let a217 = 0u64;
        let a218 = f49(match v1.clone() {
            Adt1::C0(h216, _, _, _, _, _, _, _, _, _, _) => {
                h216
            }
        });
        nat_lt(a217, a218)
    } else {
        false
    } {
        let a221 = match v0.clone() {
            Adt1::C0(_, h219, _, _, _, _, _, _, _, _, _) => {
                h219
            }
        };
        let a222 = match v3.clone() {
            Adt1::C0(_, h220, _, _, _, _, _, _, _, _, _) => {
                h220
            }
        };
        if nat_eq(a221, a222) {
            let a228 = {
                let a225 = match v0.clone() {
                    Adt1::C0(_, h223, _, _, _, _, _, _, _, _, _) => {
                        h223
                    }
                };
                let a226 = f35(match v3.clone() {
                    Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
                        h224
                    }
                });
                let a227 = 1u64;
                nat_rem(a225, a226, a227)
            };
            let a229 = 0u64;
            nat_eq(a228, a229)
        } else {
            false
        }
    } else {
        false
    } {
        let a232 = match v0.clone() {
            Adt1::C0(_, _, _, h230, _, _, _, _, _, _, _) => {
                h230
            }
        };
        let a233 = match v3.clone() {
            Adt1::C0(_, _, _, h231, _, _, _, _, _, _, _) => {
                h231
            }
        };
        nat_eq(a232, a233)
    } else {
        false
    } {
        let a236 = match v0.clone() {
            Adt1::C0(_, _, _, _, h234, _, _, _, _, _, _) => {
                h234
            }
        };
        let a237 = match v3.clone() {
            Adt1::C0(_, _, _, _, h235, _, _, _, _, _, _) => {
                h235
            }
        };
        nat_eq(a236, a237)
    } else {
        false
    } {
        let a240 = match v0.clone() {
            Adt1::C0(_, _, h238, _, _, _, _, _, _, _, _) => {
                h238
            }
        };
        let a241 = match v1.clone() {
            Adt1::C0(_, h239, _, _, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_eq(a240, a241)
    } else {
        false
    } {
        let a243 = match v1.clone() {
            Adt1::C0(_, _, _, _, h242, _, _, _, _, _, _) => {
                h242
            }
        };
        let a244 = 1u64;
        nat_eq(a243, a244)
    } else {
        false
    } {
        let a246 = 0u64;
        let a247 = match v1.clone() {
            Adt1::C0(_, _, h245, _, _, _, _, _, _, _, _) => {
                h245
            }
        };
        nat_lt(a246, a247)
    } else {
        false
    } {
        let a249 = 0u64;
        let a250 = match v1.clone() {
            Adt1::C0(_, _, _, h248, _, _, _, _, _, _, _) => {
                h248
            }
        };
        nat_lt(a249, a250)
    } else {
        false
    } {
        let a256 = {
            let a253 = match v0.clone() {
                Adt1::C0(_, _, _, h251, _, _, _, _, _, _, _) => {
                    h251
                }
            };
            let a254 = match v1.clone() {
                Adt1::C0(_, _, h252, _, _, _, _, _, _, _, _) => {
                    h252
                }
            };
            let a255 = 0u64;
            nat_rem(a253, a254, a255)
        };
        let a257 = 0u64;
        nat_eq(a256, a257)
    } else {
        false
    } {
        let a263 = {
            let a260 = match v0.clone() {
                Adt1::C0(_, _, _, _, h258, _, _, _, _, _, _) => {
                    h258
                }
            };
            let a261 = match v1.clone() {
                Adt1::C0(_, _, _, h259, _, _, _, _, _, _, _) => {
                    h259
                }
            };
            let a262 = 0u64;
            nat_rem(a260, a261, a262)
        };
        let a264 = 0u64;
        nat_eq(a263, a264)
    } else {
        false
    } {
        f50(v0.clone())
    } else {
        false
    } {
        let a266 = match v3.clone() {
            Adt1::C0(_, h265, _, _, _, _, _, _, _, _, _) => {
                h265
            }
        };
        let a267 = 1u64;
        if nat_eq(a266, a267) {
            true
        } else {
            let a270 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h268, _, _, _, _, _) => {
                    h268
                }
            };
            let a271 = f7(match v3.clone() {
                Adt1::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                    h269
                }
            });
            nat_eq(a270, a271)
        }
    } else {
        false
    } {
        let v4: u64 = f9(v1.clone())?;
        let v5: u64 = {
            let a277 = match v0.clone() {
                Adt1::C0(_, _, h272, _, _, _, _, _, _, _, _) => {
                    h272
                }
            };
            let a278 = {
                let a275 = match v0.clone() {
                    Adt1::C0(_, _, _, h273, _, _, _, _, _, _, _) => {
                        h273
                    }
                };
                let a276 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h274, _, _, _, _, _, _) => {
                        h274
                    }
                };
                nat_mul(a275, a276)?
            };
            nat_mul(a277, a278)?
        };
        let v6: u64 = if f37(v0.clone()) {
            0u64
        } else {
            let a286 = {
                let a279 = 2u64;
                let a280 = v4;
                nat_mul(a279, a280)?
            };
            let a287 = {
                let a284 = v5;
                let a285 = {
                    let a282 = 1u64;
                    let a283 = f51(v0.clone(), match v3.clone() {
                        Adt1::C0(h281, _, _, _, _, _, _, _, _, _, _) => {
                            h281
                        }
                    })?;
                    nat_add(a282, a283)?
                };
                nat_mul(a284, a285)?
            };
            nat_add(a286, a287)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f37(v0.clone()) {
            3u64
        } else {
            let a288 = 3u64;
            let a289 = v4;
            nat_add(a288, a289)?
        }, f10(match v3.clone() {
            Adt1::C0(h290, _, _, _, _, _, _, _, _, _, _) => {
                h290
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3.clone(), false, f52(match v3 {
            Adt1::C0(h291, _, _, _, _, _, _, _, _, _, _) => {
                h291
            }
        }))))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: Adt1, v1: Adt1) -> R<Adt3> {
    let v2: u64 = match v1.clone() {
        Adt1::C0(h292, _, _, _, _, _, _, _, _, _, _) => {
            h292
        }
    };
    let v3: u64 = match v1.clone() {
        Adt1::C0(_, h293, _, _, _, _, _, _, _, _, _) => {
            h293
        }
    };
    let v4: u64 = {
        let a294 = v3;
        let a295 = f35(v2);
        let a296 = 0u64;
        nat_quot(a294, a295, a296)
    };
    let v5: u64 = {
        let a302 = {
            let a300 = {
                let a297 = v4;
                let a298 = 1u64;
                nat_sub(a297, a298)
            };
            let a301 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h299, _, _, _, _, _) => {
                    h299
                }
            };
            nat_mul(a300, a301)?
        };
        let a303 = f7(v2);
        nat_add(a302, a303)?
    };
    let a305 = v2;
    let a306 = match v0.clone() {
        Adt1::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
            h304
        }
    };
    if if if if if if if if nat_eq(a305, a306) {
        let a307 = 0u64;
        let a308 = v2;
        nat_lt(a307, a308)
    } else {
        false
    } {
        let a309 = v2;
        let a310 = 24u64;
        if nat_lt(a309, a310) {
            true
        } else {
            let a311 = 28u64;
            let a312 = v2;
            nat_lt(a311, a312)
        }
    } else {
        false
    } {
        f32(v0.clone(), v1.clone())
    } else {
        false
    } {
        f6(v0.clone())
    } else {
        false
    } {
        f6(v1.clone())
    } else {
        false
    } {
        f46(v0.clone(), v1.clone())?
    } else {
        false
    } {
        if f37(v1.clone()) {
            true
        } else {
            f36(v5, match v1.clone() {
                Adt1::C0(_, _, h313, _, _, _, _, _, _, _, _) => {
                    h313
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, h314, _, _, _, _, _, _, _) => {
                    h314
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h315, _, _, _, _, _, _) => {
                    h315
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h316, _, _, _, _) => {
                    h316
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h317, _, _, _) => {
                    h317
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h318, _, _) => {
                    h318
                }
            })?
        }
    } else {
        false
    } {
        let v6: u64 = f9(v1.clone())?;
        let v7: u64 = {
            let a319 = f9(v0.clone())?;
            let a320 = v6;
            let a321 = 0u64;
            nat_quot(a319, a320, a321)
        };
        Ok(Adt3::C2(Adt2::C0({
            let a325 = {
                let a322 = v6;
                let a323 = v3;
                let a324 = 0u64;
                nat_quot(a322, a323, a324)
            };
            let a326 = f53(v7, v1.clone())?;
            nat_mul(a325, a326)?
        }, 3u64, f10(v2), 20u64, 0u64, 0u64, 0u64, v0.clone(), v0, v1, true, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f54(v0.clone(), v2.clone());
    let v4: u64 = f9(v0.clone())?;
    if if if if if f48(v1.clone(), v2.clone()) {
        let a327 = v4;
        let a328 = f9(v2.clone())?;
        nat_eq(a327, a328)
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
        if f27(match v0.clone() {
            Adt1::C0(h329, _, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        }) {
            true
        } else {
            f27(match v2.clone() {
                Adt1::C0(h330, _, _, _, _, _, _, _, _, _, _) => {
                    h330
                }
            })
        }
    } else {
        false
    } {
        let v5: u64 = {
            let a331 = 0u64;
            let a332 = v3;
            if nat_lt(a331, a332) {
                let a333 = v4;
                let a334 = v3;
                let a335 = 0u64;
                nat_quot(a333, a334, a335)
            } else {
                let a343 = {
                    let a337 = v4;
                    let a338 = match v0.clone() {
                        Adt1::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                            h336
                        }
                    };
                    let a339 = 0u64;
                    nat_quot(a337, a338, a339)
                };
                let a344 = {
                    let a340 = v4;
                    let a341 = f55(v2.clone());
                    let a342 = 0u64;
                    nat_quot(a340, a341, a342)
                };
                nat_add(a343, a344)?
            }
        };
        Ok(Adt3::C2(Adt2::C0(v5, 3u64, f10(match v2.clone() {
            Adt1::C0(h345, _, _, _, _, _, _, _, _, _, _) => {
                h345
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false, {
            let a346 = 0u64;
            let a347 = v3;
            if nat_lt(a346, a347) {
                v3
            } else {
                v4
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a349 = match v2.clone() {
        Adt1::C0(h348, _, _, _, _, _, _, _, _, _, _) => {
            h348
        }
    };
    let a350 = 0u64;
    if if if if if if if if if if if nat_eq(a349, a350) {
        let a352 = match v3.clone() {
            Adt1::C0(h351, _, _, _, _, _, _, _, _, _, _) => {
                h351
            }
        };
        let a353 = 26u64;
        nat_eq(a352, a353)
    } else {
        false
    } {
        let a355 = match v2.clone() {
            Adt1::C0(_, h354, _, _, _, _, _, _, _, _, _) => {
                h354
            }
        };
        let a356 = 2147483647u64;
        nat_le(a355, a356)
    } else {
        false
    } {
        let a359 = match v3.clone() {
            Adt1::C0(_, h357, _, _, _, _, _, _, _, _, _) => {
                h357
            }
        };
        let a360 = match v2.clone() {
            Adt1::C0(_, h358, _, _, _, _, _, _, _, _, _) => {
                h358
            }
        };
        nat_le(a359, a360)
    } else {
        false
    } {
        let a363 = match v2.clone() {
            Adt1::C0(_, _, h361, _, _, _, _, _, _, _, _) => {
                h361
            }
        };
        let a364 = match v3.clone() {
            Adt1::C0(_, _, h362, _, _, _, _, _, _, _, _) => {
                h362
            }
        };
        nat_eq(a363, a364)
    } else {
        false
    } {
        let a367 = match v2.clone() {
            Adt1::C0(_, _, _, h365, _, _, _, _, _, _, _) => {
                h365
            }
        };
        let a368 = match v3.clone() {
            Adt1::C0(_, _, _, h366, _, _, _, _, _, _, _) => {
                h366
            }
        };
        nat_eq(a367, a368)
    } else {
        false
    } {
        let a371 = match v2.clone() {
            Adt1::C0(_, _, _, _, h369, _, _, _, _, _, _) => {
                h369
            }
        };
        let a372 = match v3.clone() {
            Adt1::C0(_, _, _, _, h370, _, _, _, _, _, _) => {
                h370
            }
        };
        nat_eq(a371, a372)
    } else {
        false
    } {
        let a373 = v0;
        let a374 = 67u64;
        if nat_eq(a373, a374) {
            true
        } else {
            let a375 = v1;
            let a376 = 1u64;
            if nat_le(a375, a376) {
                let a379 = match v3.clone() {
                    Adt1::C0(_, h377, _, _, _, _, _, _, _, _, _) => {
                        h377
                    }
                };
                let a380 = match v2.clone() {
                    Adt1::C0(_, h378, _, _, _, _, _, _, _, _, _) => {
                        h378
                    }
                };
                nat_eq(a379, a380)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f56(v2.clone())?
    } else {
        false
    } {
        f56(v3.clone())?
    } else {
        false
    } {
        f46(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a386 = match v3.clone() {
                Adt1::C0(_, _, h381, _, _, _, _, _, _, _, _) => {
                    h381
                }
            };
            let a387 = {
                let a384 = match v3.clone() {
                    Adt1::C0(_, _, _, h382, _, _, _, _, _, _, _) => {
                        h382
                    }
                };
                let a385 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h383, _, _, _, _, _, _) => {
                        h383
                    }
                };
                nat_mul(a384, a385)?
            };
            nat_mul(a386, a387)?
        }, 3u64, None::<u64>, v0, {
            let a388 = v0;
            let a389 = 67u64;
            if nat_eq(a388, a389) {
                1u64
            } else {
                v1
            }
        }, 0u64, 0u64, v2.clone(), v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f15(v0: u64) -> bool {
    let a390 = v0;
    let a391 = 6u64;
    nat_le(a390, a391)
}

pub fn f16(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a393 = 0u64;
    let a394 = f30(match v5.clone() {
        Adt1::C0(h392, _, _, _, _, _, _, _, _, _, _) => {
            h392
        }
    });
    if nat_lt(a393, a394) {
        f57(v0, v1, v2, v3, v4, v5, v6, v7)
    } else {
        let a395 = v2;
        let a396 = 1u64;
        if if if if if if if if if if if if if if if if nat_le(a395, a396) {
            let a399 = match v5.clone() {
                Adt1::C0(h397, _, _, _, _, _, _, _, _, _, _) => {
                    h397
                }
            };
            let a400 = match v7.clone() {
                Adt1::C0(h398, _, _, _, _, _, _, _, _, _, _) => {
                    h398
                }
            };
            nat_eq(a399, a400)
        } else {
            false
        } {
            let a403 = match v6.clone() {
                Adt1::C0(h401, _, _, _, _, _, _, _, _, _, _) => {
                    h401
                }
            };
            let a404 = match v5.clone() {
                Adt1::C0(h402, _, _, _, _, _, _, _, _, _, _) => {
                    h402
                }
            };
            nat_eq(a403, a404)
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
            let a407 = match v7.clone() {
                Adt1::C0(_, _, _, _, _, h405, _, _, _, _, _) => {
                    h405
                }
            };
            let a408 = f7(match v7.clone() {
                Adt1::C0(h406, _, _, _, _, _, _, _, _, _, _) => {
                    h406
                }
            });
            nat_eq(a407, a408)
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
            let a411 = match v5.clone() {
                Adt1::C0(_, _, h409, _, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            let a412 = match v7.clone() {
                Adt1::C0(_, _, h410, _, _, _, _, _, _, _, _) => {
                    h410
                }
            };
            nat_eq(a411, a412)
        } else {
            false
        } {
            let a415 = match v5.clone() {
                Adt1::C0(_, _, _, h413, _, _, _, _, _, _, _) => {
                    h413
                }
            };
            let a416 = match v7.clone() {
                Adt1::C0(_, _, _, h414, _, _, _, _, _, _, _) => {
                    h414
                }
            };
            nat_eq(a415, a416)
        } else {
            false
        } {
            let a419 = match v5.clone() {
                Adt1::C0(_, _, _, _, h417, _, _, _, _, _, _) => {
                    h417
                }
            };
            let a420 = match v7.clone() {
                Adt1::C0(_, _, _, _, h418, _, _, _, _, _, _) => {
                    h418
                }
            };
            nat_eq(a419, a420)
        } else {
            false
        } {
            let a427 = match v7.clone() {
                Adt1::C0(_, h421, _, _, _, _, _, _, _, _, _) => {
                    h421
                }
            };
            let a428 = if v1 {
                match v5.clone() {
                    Adt1::C0(_, h422, _, _, _, _, _, _, _, _, _) => {
                        h422
                    }
                }
            } else {
                let a424 = match v5.clone() {
                    Adt1::C0(_, h423, _, _, _, _, _, _, _, _, _) => {
                        h423
                    }
                };
                let a425 = 2u64;
                let a426 = 0u64;
                nat_quot(a424, a425, a426)
            };
            nat_eq(a427, a428)
        } else {
            false
        } {
            let a430 = match v7.clone() {
                Adt1::C0(_, h429, _, _, _, _, _, _, _, _, _) => {
                    h429
                }
            };
            let a431 = 2147483647u64;
            nat_le(a430, a431)
        } else {
            false
        } {
            f36({
                let a438 = {
                    let a434 = match v7.clone() {
                        Adt1::C0(_, h432, _, _, _, _, _, _, _, _, _) => {
                            h432
                        }
                    };
                    let a435 = f35(match v7.clone() {
                        Adt1::C0(h433, _, _, _, _, _, _, _, _, _, _) => {
                            h433
                        }
                    });
                    let a436 = 0u64;
                    nat_quot(a434, a435, a436)
                };
                let a439 = match v7.clone() {
                    Adt1::C0(_, _, _, _, _, h437, _, _, _, _, _) => {
                        h437
                    }
                };
                nat_mul(a438, a439)?
            }, match v7.clone() {
                Adt1::C0(_, _, h440, _, _, _, _, _, _, _, _) => {
                    h440
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, h441, _, _, _, _, _, _, _) => {
                    h441
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, h442, _, _, _, _, _, _) => {
                    h442
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, h443, _, _, _, _) => {
                    h443
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h444, _, _, _) => {
                    h444
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h445, _, _) => {
                    h445
                }
            })?
        } else {
            false
        } {
            let v8: u64 = match v7.clone() {
                Adt1::C0(_, h446, _, _, _, _, _, _, _, _, _) => {
                    h446
                }
            };
            let v9: u64 = {
                let a447 = v8;
                let a448 = 4u64;
                nat_mul(a447, a448)?
            };
            let v10: Adt1 = Adt1::C0(0u64, v8, 1u64, 1u64, 1u64, 4u64, v9, v9, v9, v9, 0u64);
            Ok(Adt3::C10(Adt2::C0({
                let a449 = f9(v7.clone())?;
                let a450 = v8;
                let a451 = 0u64;
                nat_quot(a449, a450, a451)
            }, 3u64, f10(match v7.clone() {
                Adt1::C0(h452, _, _, _, _, _, _, _, _, _, _) => {
                    h452
                }
            }), if v1 {
                98u64
            } else {
                let a453 = 99u64;
                let a454 = v2;
                nat_add(a453, a454)?
            }, v0, v3, v4, v5, v6, v7, true, v8), f58(v0, true, 0u64, v3, v4, v10.clone(), v10.clone(), v10)?))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f17(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h455, _, _, _, _) => {
            h455
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h456, _, _) => {
            h456
        }
    };
    let v4: u64 = {
        let a457 = f9(v2.clone())?;
        let a458 = f9(v3.clone())?;
        let a459 = 0u64;
        nat_quot(a457, a458, a459)
    };
    let v5: u64 = f53(v4, v3.clone())?;
    let v6: u64 = {
        let a460 = v0;
        let a461 = v5;
        let a462 = 0u64;
        nat_quot(a460, a461, a462)
    };
    let v7: u64 = {
        let a463 = v0;
        let a464 = v5;
        let a465 = 0u64;
        nat_rem(a463, a464, a465)
    };
    let a467 = v0;
    let a468 = match v1 {
        Adt2::C0(h466, _, _, _, _, _, _, _, _, _, _, _) => {
            h466
        }
    };
    if nat_lt(a467, a468) {
        let a469 = v7;
        let a470 = 0u64;
        if nat_eq(a469, a470) {
            Ok(Adt3::C11(None::<u64>))
        } else {
            let a471 = v7;
            let a472 = v4;
            if nat_le(a471, a472) {
                Ok(Adt3::C11(Some(f59(v6, {
                    let a473 = v7;
                    let a474 = 1u64;
                    nat_sub(a473, a474)
                }, v2, v3)?)))
            } else {
                let v8: u64 = f55(v3.clone());
                let v9: u64 = {
                    let a479 = {
                        let a477 = v7;
                        let a478 = {
                            let a475 = 1u64;
                            let a476 = v4;
                            nat_add(a475, a476)?
                        };
                        nat_sub(a477, a478)
                    };
                    let a480 = v8;
                    nat_mul(a479, a480)?
                };
                Ok(Adt3::C9(v9, f60({
                    let a484 = {
                        let a482 = v6;
                        let a483 = match v3.clone() {
                            Adt1::C0(_, h481, _, _, _, _, _, _, _, _, _) => {
                                h481
                            }
                        };
                        nat_mul(a482, a483)?
                    };
                    let a485 = v9;
                    nat_add(a484, a485)?
                }, v3)?, v8))
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f18(v0: u64, v1: Adt2) -> R<Adt3> {
    let a487 = v0;
    let a488 = match v1.clone() {
        Adt2::C0(h486, _, _, _, _, _, _, _, _, _, _, _) => {
            h486
        }
    };
    if nat_lt(a487, a488) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h489, _, _, _, _) => {
                h489
            }
        };
        let v3: u64 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h490) => {
                h490
            }
        };
        let v4: u64 = {
            let a492 = v0;
            let a493 = match v2.clone() {
                Adt1::C0(_, h491, _, _, _, _, _, _, _, _, _) => {
                    h491
                }
            };
            nat_mul(a492, a493)?
        };
        let v5: u64 = match v1.clone() {
            Adt2::C0(_, _, _, h494, _, _, _, _, _, _, _, _) => {
                h494
            }
        };
        Ok(Adt3::C7(f60({
            let a497 = v4;
            let a498 = {
                let a495 = v5;
                let a496 = 100u64;
                if nat_eq(a495, a496) {
                    v3
                } else {
                    0u64
                }
            };
            nat_add(a497, a498)?
        }, v2)?, f60({
            let a501 = v4;
            let a502 = {
                let a499 = v5;
                let a500 = 99u64;
                if nat_eq(a499, a500) {
                    v3
                } else {
                    0u64
                }
            };
            nat_add(a501, a502)?
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h503, _, _, _) => {
                h503
            }
        })?, f60({
            let a504 = v0;
            let a505 = v3;
            nat_mul(a504, a505)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h506, _, _) => {
                h506
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f19(v0: u64, v1: Adt2) -> R<Adt3> {
    let a508 = v0;
    let a509 = match v1.clone() {
        Adt2::C0(h507, _, _, _, _, _, _, _, _, _, _, _) => {
            h507
        }
    };
    if nat_lt(a508, a509) {
        let v2: u64 = {
            let a513 = v0;
            let a514 = {
                let m512 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h511, _, _, _, _) => {
                        h511
                    }
                };
                match m512 {
                    Adt1::C0(_, h510, _, _, _, _, _, _, _, _, _) => {
                        h510
                    }
                }
            };
            nat_mul(a513, a514)?
        };
        Ok(Adt3::C7(f61(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h515, _, _, _, _) => {
                h515
            }
        })?, f62(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h516, _, _, _, _) => {
                h516
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h517, _, _, _) => {
                h517
            }
        })?, f61(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h518, _, _) => {
                h518
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f20(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h519, _, _, _, _) => {
            h519
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h520, _, _) => {
            h520
        }
    };
    let a522 = v0;
    let a523 = match v1.clone() {
        Adt2::C0(h521, _, _, _, _, _, _, _, _, _, _, _) => {
            h521
        }
    };
    if nat_lt(a522, a523) {
        let v4: u64 = {
            let a525 = match v1.clone() {
                Adt2::C0(_, _, _, h524, _, _, _, _, _, _, _, _) => {
                    h524
                }
            };
            let a526 = 89u64;
            if nat_eq(a525, a526) {
                match v2.clone() {
                    Adt1::C0(_, h527, _, _, _, _, _, _, _, _, _) => {
                        h527
                    }
                }
            } else {
                f54(v2.clone(), v3.clone())
            }
        };
        let a528 = 0u64;
        let a529 = v4;
        if nat_lt(a528, a529) {
            let v5: u64 = {
                let a530 = v0;
                let a531 = v4;
                nat_mul(a530, a531)?
            };
            Ok(Adt3::C6(f60(v5, v2)?, f60(v5, v3)?))
        } else {
            let v6: u64 = {
                let a534 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h532) => {
                        h532
                    }
                };
                let a535 = match v2.clone() {
                    Adt1::C0(_, h533, _, _, _, _, _, _, _, _, _) => {
                        h533
                    }
                };
                let a536 = 0u64;
                nat_quot(a534, a535, a536)
            };
            let a537 = v0;
            let a538 = v6;
            if nat_lt(a537, a538) {
                let v7: u64 = {
                    let a540 = v0;
                    let a541 = match v2.clone() {
                        Adt1::C0(_, h539, _, _, _, _, _, _, _, _, _) => {
                            h539
                        }
                    };
                    nat_mul(a540, a541)?
                };
                Ok(Adt3::C8(f60(v7, v2.clone())?, v7, match v2 {
                    Adt1::C0(_, h542, _, _, _, _, _, _, _, _, _) => {
                        h542
                    }
                }))
            } else {
                let v8: u64 = f55(v3.clone());
                let v9: u64 = {
                    let a545 = {
                        let a543 = v0;
                        let a544 = v6;
                        nat_sub(a543, a544)
                    };
                    let a546 = v8;
                    nat_mul(a545, a546)?
                };
                Ok(Adt3::C9(v9, f60(v9, v3)?, v8))
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f21(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h547, _, _, _, _) => {
            h547
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h548, _, _) => {
            h548
        }
    };
    let a550 = v0;
    let a551 = match v1.clone() {
        Adt2::C0(h549, _, _, _, _, _, _, _, _, _, _, _) => {
            h549
        }
    };
    if nat_lt(a550, a551) {
        Ok(Adt3::C5({
            let a553 = v0;
            let a554 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h552, _, _, _, _) => {
                    h552
                }
            };
            nat_mul(a553, a554)?
        }, {
            let a556 = v0;
            let a557 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h555, _, _, _, _) => {
                    h555
                }
            };
            nat_mul(a556, a557)?
        }, match v2 {
            Adt1::C0(_, h558, _, _, _, _, _, _, _, _, _) => {
                h558
            }
        }, match v3 {
            Adt1::C0(_, h559, _, _, _, _, _, _, _, _, _) => {
                h559
            }
        }, {
            let a561 = match v1 {
                Adt2::C0(_, _, _, _, h560, _, _, _, _, _, _, _) => {
                    h560
                }
            };
            let a562 = 1u64;
            nat_eq(a561, a562)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f22(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a564 = v0;
    let a565 = match v1.clone() {
        Adt2::C0(h563, _, _, _, _, _, _, _, _, _, _, _) => {
            h563
        }
    };
    if nat_lt(a564, a565) {
        let v3: u64 = {
            let a567 = 2u64;
            let a568 = f9(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h566, _, _, _) => {
                    h566
                }
            })?;
            nat_mul(a567, a568)?
        };
        let a569 = v0;
        let a570 = v3;
        if nat_lt(a569, a570) {
            f63(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h571, _, _, _) => {
                    h571
                }
            }, {
                let m574 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h573, _, _) => {
                        h573
                    }
                };
                match m574 {
                    Adt1::C0(_, _, h572, _, _, _, _, _, _, _, _) => {
                        h572
                    }
                }
            }, v2)
        } else {
            f64({
                let a575 = v0;
                let a576 = v3;
                nat_sub(a575, a576)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h577, _, _, _, _) => {
                    h577
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h578, _, _, _) => {
                    h578
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h579, _, _) => {
                    h579
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f23(v0: u64) -> bool {
    let a580 = v0;
    let a581 = 3u64;
    if nat_eq(a580, a581) {
        true
    } else {
        let a582 = v0;
        let a583 = 6u64;
        nat_eq(a582, a583)
    }
}

pub fn f24(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h584, _, _, _, _, _) => {
            h584
        }
    };
    let a586 = v0;
    let a587 = match v1.clone() {
        Adt2::C0(h585, _, _, _, _, _, _, _, _, _, _, _) => {
            h585
        }
    };
    if nat_lt(a586, a587) {
        let a588 = v0;
        let a589 = 0u64;
        if nat_eq(a588, a589) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a590 = v0;
                let a591 = 1u64;
                nat_sub(a590, a591)
            };
            let v5: u64 = {
                let a593 = v4;
                let a594 = f65(match v1.clone() {
                    Adt2::C0(_, _, _, _, h592, _, _, _, _, _, _, _) => {
                        h592
                    }
                });
                let a595 = 0u64;
                nat_rem(a593, a594, a595)
            };
            let v6: u64 = {
                let a600 = 4u64;
                let a601 = {
                    let a597 = v4;
                    let a598 = f65(match v1.clone() {
                        Adt2::C0(_, _, _, _, h596, _, _, _, _, _, _, _) => {
                            h596
                        }
                    });
                    let a599 = 0u64;
                    nat_quot(a597, a598, a599)
                };
                nat_mul(a600, a601)?
            };
            let a602 = v5;
            let a603 = 2u64;
            if nat_lt(a602, a603) {
                f25({
                    let a604 = v6;
                    let a605 = v5;
                    nat_add(a604, a605)?
                }, v1)
            } else {
                let a606 = v5;
                let a607 = 2u64;
                if nat_eq(a606, a607) {
                    Ok(f66(0u64, 2u64, v3))
                } else {
                    let a608 = v5;
                    let a609 = 3u64;
                    if nat_eq(a608, a609) {
                        Ok(f67(0u64, 2u64, v3, {
                            let a610 = v2;
                            let a611 = 2u64;
                            nat_eq(a610, a611)
                        }))
                    } else {
                        let a612 = v5;
                        let a613 = 4u64;
                        if nat_eq(a612, a613) {
                            Ok(f66(1u64, 1u64, 3u64))
                        } else {
                            let a614 = v5;
                            let a615 = 5u64;
                            if nat_eq(a614, a615) {
                                Ok(f67(1u64, 1u64, 3u64, {
                                    let a616 = v2;
                                    let a617 = 0u64;
                                    nat_eq(a616, a617)
                                }))
                            } else {
                                let a618 = v5;
                                let a619 = 6u64;
                                if nat_eq(a618, a619) {
                                    Ok(f66(1u64, 2u64, v3))
                                } else {
                                    let a620 = v5;
                                    let a621 = 7u64;
                                    if nat_eq(a620, a621) {
                                        Ok(f67(1u64, 2u64, v3, {
                                            let a622 = v2;
                                            let a623 = 2u64;
                                            nat_eq(a622, a623)
                                        }))
                                    } else {
                                        let a625 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h624, _, _, _, _, _, _, _) => {
                                                h624
                                            }
                                        };
                                        let a626 = 3u64;
                                        if nat_eq(a625, a626) {
                                            let a627 = v5;
                                            let a628 = 15u64;
                                            if nat_eq(a627, a628) {
                                                f25({
                                                    let a629 = v6;
                                                    let a630 = 3u64;
                                                    nat_add(a629, a630)?
                                                }, v1)
                                            } else {
                                                Ok(f68(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h631, _, _, _, _, _, _) => {
                                                        h631
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a632 = v5;
                                            let a633 = 8u64;
                                            if nat_eq(a632, a633) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m636 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h635, _, _) => {
                                                            h635
                                                        }
                                                    };
                                                    match m636 {
                                                        Adt1::C0(h634, _, _, _, _, _, _, _, _, _, _) => {
                                                            h634
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f25({
                                                    let a637 = v6;
                                                    let a638 = 3u64;
                                                    nat_add(a637, a638)?
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

pub fn f25(v0: u64, v1: Adt2) -> R<Adt3> {
    let a640 = v0;
    let a641 = match v1.clone() {
        Adt2::C0(h639, _, _, _, _, _, _, _, _, _, _, _) => {
            h639
        }
    };
    if nat_lt(a640, a641) {
        f69({
            let a642 = v0;
            let a643 = 4u64;
            let a644 = 0u64;
            nat_rem(a642, a643, a644)
        }, {
            let a645 = v0;
            let a646 = 4u64;
            let a647 = 0u64;
            nat_quot(a645, a646, a647)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f26(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h648, _, _, _, _) => {
            h648
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h649, _, _, _) => {
            h649
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h650, _, _) => {
            h650
        }
    };
    let v6: u64 = {
        let a652 = match v1.clone() {
            Adt2::C0(_, h651, _, _, _, _, _, _, _, _, _, _) => {
                h651
            }
        };
        let a653 = 3u64;
        if nat_eq(a652, a653) {
            0u64
        } else {
            let a654 = f9(v3.clone())?;
            let a655 = 2u64;
            nat_mul(a654, a655)?
        }
    };
    let a657 = match v1.clone() {
        Adt2::C0(h656, _, _, _, _, _, _, _, _, _, _, _) => {
            h656
        }
    };
    let a658 = v0;
    if nat_le(a657, a658) {
        Ok(Adt3::C1)
    } else {
        let a659 = v0;
        let a660 = v6;
        if nat_lt(a659, a660) {
            let v7: u64 = {
                let a661 = v0;
                let a662 = 2u64;
                let a663 = 0u64;
                nat_quot(a661, a662, a663)
            };
            let v8: u64 = {
                let a668 = 3u64;
                let a669 = {
                    let a665 = f61(v7, v5.clone())?;
                    let a666 = f39(match v5 {
                        Adt1::C0(h664, _, _, _, _, _, _, _, _, _, _) => {
                            h664
                        }
                    });
                    let a667 = 0u64;
                    nat_quot(a665, a666, a667)
                };
                nat_add(a668, a669)?
            };
            let a673 = {
                let a670 = v0;
                let a671 = 2u64;
                let a672 = 0u64;
                nat_rem(a670, a671, a672)
            };
            let a674 = 0u64;
            if nat_eq(a673, a674) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a675 = v2;
                let a676 = 0u64;
                if nat_eq(a675, a676) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a677 = v0;
                let a678 = v6;
                nat_sub(a677, a678)
            };
            if f70(match v3.clone() {
                Adt1::C0(h679, _, _, _, _, _, _, _, _, _, _) => {
                    h679
                }
            }) {
                f71(v9, v1)
            } else {
                let v10: u64 = f72(match v3.clone() {
                    Adt1::C0(h680, _, _, _, _, _, _, _, _, _, _) => {
                        h680
                    }
                }, match v4.clone() {
                    Adt1::C0(h681, _, _, _, _, _, _, _, _, _, _) => {
                        h681
                    }
                }, match v5.clone() {
                    Adt1::C0(h682, _, _, _, _, _, _, _, _, _, _) => {
                        h682
                    }
                })?;
                let v11: u64 = {
                    let a683 = v9;
                    let a684 = v10;
                    let a685 = 0u64;
                    nat_quot(a683, a684, a685)
                };
                let v12: u64 = {
                    let a686 = v9;
                    let a687 = v10;
                    let a688 = 0u64;
                    nat_rem(a686, a687, a688)
                };
                let v13: u64 = {
                    let a690 = match v3.clone() {
                        Adt1::C0(h689, _, _, _, _, _, _, _, _, _, _) => {
                            h689
                        }
                    };
                    let a691 = 0u64;
                    if nat_eq(a690, a691) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let v14: u64 = {
                    let a693 = match v4.clone() {
                        Adt1::C0(h692, _, _, _, _, _, _, _, _, _, _) => {
                            h692
                        }
                    };
                    let a694 = 0u64;
                    if nat_eq(a693, a694) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let a695 = v12;
                let a696 = v13;
                if nat_lt(a695, a696) {
                    Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                        Adt1::C0(h697, _, _, _, _, _, _, _, _, _, _) => {
                            h697
                        }
                    }, 0u64, 0u64, f61(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f39(match v3 {
                        Adt1::C0(h698, _, _, _, _, _, _, _, _, _, _) => {
                            h698
                        }
                    }), 4u64))
                } else {
                    let a701 = v12;
                    let a702 = {
                        let a699 = v13;
                        let a700 = v14;
                        nat_add(a699, a700)?
                    };
                    if nat_lt(a701, a702) {
                        Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                            Adt1::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
                                h703
                            }
                        }, 0u64, 4u64, f62(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f39(match v4 {
                            Adt1::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
                                h704
                            }
                        }), 4u64))
                    } else {
                        let a707 = v12;
                        let a708 = {
                            let a705 = v13;
                            let a706 = v14;
                            nat_add(a705, a706)?
                        };
                        if nat_eq(a707, a708) {
                            Ok(Adt3::C3(1u64, match v1 {
                                Adt2::C0(_, _, _, h709, _, _, _, _, _, _, _, _) => {
                                    h709
                                }
                            }, 0u64, 0u64, {
                                let a710 = v13;
                                let a711 = 0u64;
                                if nat_eq(a710, a711) {
                                    0u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a712 = v13;
                                let a713 = 0u64;
                                if nat_eq(a712, a713) {
                                    f61(v11, v3.clone())?
                                } else {
                                    0u64
                                }
                            }, {
                                let a714 = v14;
                                let a715 = 0u64;
                                if nat_eq(a714, a715) {
                                    4u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a716 = v14;
                                let a717 = 0u64;
                                if nat_eq(a716, a717) {
                                    f62(v11, v3, v4)?
                                } else {
                                    1u64
                                }
                            }, {
                                let a719 = match v5.clone() {
                                    Adt1::C0(h718, _, _, _, _, _, _, _, _, _, _) => {
                                        h718
                                    }
                                };
                                let a720 = 0u64;
                                if nat_eq(a719, a720) {
                                    3u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a722 = match v5.clone() {
                                    Adt1::C0(h721, _, _, _, _, _, _, _, _, _, _) => {
                                        h721
                                    }
                                };
                                let a723 = 0u64;
                                if nat_eq(a722, a723) {
                                    f61(v11, v5)?
                                } else {
                                    0u64
                                }
                            }, 4u64, 4u64))
                        } else {
                            Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                                Adt1::C0(h724, _, _, _, _, _, _, _, _, _, _) => {
                                    h724
                                }
                            }, 1u64, 0u64, 2u64, 0u64, 3u64, f61(v11, v5.clone())?, 4u64, f39(match v5 {
                                Adt1::C0(h725, _, _, _, _, _, _, _, _, _, _) => {
                                    h725
                                }
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> bool {
    let a726 = 6u64;
    let a727 = v0;
    if if if if nat_le(a726, a727) {
        let a728 = v0;
        let a729 = 23u64;
        nat_le(a728, a729)
    } else {
        false
    } {
        true
    } else {
        let a730 = v0;
        let a731 = 2u64;
        if nat_eq(a730, a731) {
            true
        } else {
            let a732 = v0;
            let a733 = 3u64;
            nat_eq(a732, a733)
        }
    } {
        true
    } else {
        let a734 = v0;
        let a735 = 29u64;
        nat_eq(a734, a735)
    } {
        true
    } else {
        let a736 = 34u64;
        let a737 = v0;
        if if nat_le(a736, a737) {
            let a738 = v0;
            let a739 = 35u64;
            nat_le(a738, a739)
        } else {
            false
        } {
            true
        } else {
            let a740 = 39u64;
            let a741 = v0;
            if nat_le(a740, a741) {
                let a742 = v0;
                let a743 = 42u64;
                nat_le(a742, a743)
            } else {
                false
            }
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> bool {
    let a744 = v0;
    let a745 = 0u64;
    if if if nat_eq(a744, a745) {
        let a746 = v1;
        let a747 = 0u64;
        nat_eq(a746, a747)
    } else {
        false
    } {
        let a748 = v2;
        let a749 = 0u64;
        nat_eq(a748, a749)
    } else {
        false
    } {
        true
    } else {
        let a750 = v0;
        let a751 = 1u64;
        if if nat_eq(a750, a751) {
            true
        } else {
            let a752 = v0;
            let a753 = 30u64;
            nat_eq(a752, a753)
        } {
            let a754 = v1;
            let a755 = v0;
            if if nat_eq(a754, a755) {
                let a756 = v2;
                let a757 = v0;
                nat_eq(a756, a757)
            } else {
                false
            } {
                true
            } else {
                let a758 = v1;
                let a759 = 0u64;
                if nat_eq(a758, a759) {
                    let a760 = v2;
                    let a761 = v0;
                    if nat_eq(a760, a761) {
                        true
                    } else {
                        let a762 = v2;
                        let a763 = 0u64;
                        nat_eq(a762, a763)
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

pub fn f29(v0: u64, v1: u64, v2: u64) -> bool {
    let a764 = v1;
    let a765 = 27u64;
    if if if nat_eq(a764, a765) {
        true
    } else {
        let a766 = v1;
        let a767 = 28u64;
        nat_eq(a766, a767)
    } {
        let a768 = v1;
        let a769 = v2;
        nat_eq(a768, a769)
    } else {
        false
    } {
        let a770 = v0;
        let a771 = 3u64;
        nat_eq(a770, a771)
    } else {
        f31(v1)
    }
}

pub fn f30(v0: u64) -> u64 {
    let a772 = v0;
    let a773 = 0u64;
    if nat_eq(a772, a773) {
        4u64
    } else {
        let a774 = v0;
        let a775 = 1u64;
        if if nat_eq(a774, a775) {
            true
        } else {
            let a776 = v0;
            let a777 = 30u64;
            nat_eq(a776, a777)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f31(v0: u64) -> bool {
    let a778 = 0u64;
    let a779 = f73(v0);
    if nat_lt(a778, a779) {
        true
    } else {
        let a780 = 24u64;
        let a781 = v0;
        if nat_le(a780, a781) {
            let a782 = v0;
            let a783 = 28u64;
            nat_le(a782, a783)
        } else {
            false
        }
    }
}

pub fn f32(v0: Adt1, v1: Adt1) -> bool {
    if f37(v1.clone()) {
        f37(v0)
    } else {
        let a789 = {
            let a786 = match v0.clone() {
                Adt1::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                    h784
                }
            };
            let a787 = match v1.clone() {
                Adt1::C0(_, h785, _, _, _, _, _, _, _, _, _) => {
                    h785
                }
            };
            let a788 = 1u64;
            nat_rem(a786, a787, a788)
        };
        let a790 = 0u64;
        if if if nat_eq(a789, a790) {
            let a796 = {
                let a793 = match v0.clone() {
                    Adt1::C0(_, _, h791, _, _, _, _, _, _, _, _) => {
                        h791
                    }
                };
                let a794 = match v1.clone() {
                    Adt1::C0(_, _, h792, _, _, _, _, _, _, _, _) => {
                        h792
                    }
                };
                let a795 = 1u64;
                nat_rem(a793, a794, a795)
            };
            let a797 = 0u64;
            nat_eq(a796, a797)
        } else {
            false
        } {
            let a803 = {
                let a800 = match v0.clone() {
                    Adt1::C0(_, _, _, h798, _, _, _, _, _, _, _) => {
                        h798
                    }
                };
                let a801 = match v1.clone() {
                    Adt1::C0(_, _, _, h799, _, _, _, _, _, _, _) => {
                        h799
                    }
                };
                let a802 = 1u64;
                nat_rem(a800, a801, a802)
            };
            let a804 = 0u64;
            nat_eq(a803, a804)
        } else {
            false
        } {
            let a810 = {
                let a807 = match v0 {
                    Adt1::C0(_, _, _, _, h805, _, _, _, _, _, _) => {
                        h805
                    }
                };
                let a808 = match v1 {
                    Adt1::C0(_, _, _, _, h806, _, _, _, _, _, _) => {
                        h806
                    }
                };
                let a809 = 1u64;
                nat_rem(a807, a808, a809)
            };
            let a811 = 0u64;
            nat_eq(a810, a811)
        } else {
            false
        }
    }
}

pub fn f33(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h812, _, _, _, _, _, _, _, _, _, _) => {
            h812
        }
    };
    let a813 = v1;
    let a814 = v3;
    if if nat_eq(a813, a814) {
        true
    } else {
        let a815 = v0;
        let a816 = 0u64;
        if nat_eq(a815, a816) {
            let a817 = v4;
            let a818 = 0u64;
            if if nat_eq(a817, a818) {
                true
            } else {
                f27(v3)
            } {
                true
            } else {
                f31(v3)
            }
        } else {
            false
        }
    } {
        if if if f31(v4) {
            true
        } else {
            f27(v4)
        } {
            let a820 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h819, _, _, _, _, _) => {
                    h819
                }
            };
            let a821 = f7(v4);
            nat_eq(a820, a821)
        } else {
            false
        } {
            let a826 = {
                let a823 = match v2 {
                    Adt1::C0(_, h822, _, _, _, _, _, _, _, _, _) => {
                        h822
                    }
                };
                let a824 = f35(v4);
                let a825 = 1u64;
                nat_rem(a823, a824, a825)
            };
            let a827 = 0u64;
            nat_eq(a826, a827)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64) -> bool {
    if f27(v0) {
        true
    } else {
        let a828 = v0;
        let a829 = v1;
        if if nat_eq(a828, a829) {
            f31(v0)
        } else {
            false
        } {
            true
        } else {
            let a830 = f30(v0);
            let a831 = 2u64;
            if nat_eq(a830, a831) {
                if f27(v1) {
                    true
                } else {
                    f31(v1)
                }
            } else {
                false
            }
        }
    }
}

pub fn f35(v0: u64) -> u64 {
    if f27(v0) {
        let m832 = f45(v0);
        match m832 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h833, _) => {
                        h833
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f74(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f74(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f74(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f74(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f74(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f74(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f37(v0: Adt1) -> bool {
    f75(match v0.clone() {
        Adt1::C0(_, h834, _, _, _, _, _, _, _, _, _) => {
            h834
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h835, _, _, _, _, _, _, _, _) => {
            h835
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h836, _, _, _, _, _, _, _) => {
            h836
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h837, _, _, _, _, _, _) => {
            h837
        }
    })
}

pub fn f38(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a839 = match v1.clone() {
        Adt1::C0(h838, _, _, _, _, _, _, _, _, _, _) => {
            h838
        }
    };
    let a840 = 43u64;
    if nat_eq(a839, a840) {
        let a841 = v0;
        let a842 = 0u64;
        if if if if if if nat_eq(a841, a842) {
            let a844 = match v2.clone() {
                Adt1::C0(h843, _, _, _, _, _, _, _, _, _, _) => {
                    h843
                }
            };
            let a845 = 0u64;
            nat_eq(a844, a845)
        } else {
            false
        } {
            let a847 = match v3.clone() {
                Adt1::C0(h846, _, _, _, _, _, _, _, _, _, _) => {
                    h846
                }
            };
            let a848 = 0u64;
            if nat_eq(a847, a848) {
                true
            } else {
                let a850 = match v3.clone() {
                    Adt1::C0(h849, _, _, _, _, _, _, _, _, _, _) => {
                        h849
                    }
                };
                let a851 = 43u64;
                nat_eq(a850, a851)
            }
        } else {
            false
        } {
            let a853 = match v3.clone() {
                Adt1::C0(h852, _, _, _, _, _, _, _, _, _, _) => {
                    h852
                }
            };
            let a854 = 43u64;
            if nat_eq(a853, a854) {
                f32(v1.clone(), v2.clone())
            } else {
                f5(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a856 = match v2 {
                Adt1::C0(_, _, _, _, _, h855, _, _, _, _, _) => {
                    h855
                }
            };
            let a857 = 4u64;
            nat_eq(a856, a857)
        } else {
            false
        } {
            let a860 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h858, _, _, _, _, _) => {
                    h858
                }
            };
            let a861 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h859, _, _, _, _) => {
                    h859
                }
            };
            if nat_le(a860, a861) {
                let a864 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h862, _, _, _, _) => {
                        h862
                    }
                };
                let a865 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h863, _, _, _) => {
                        h863
                    }
                };
                if nat_le(a864, a865) {
                    let a868 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h866, _, _, _) => {
                            h866
                        }
                    };
                    let a869 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h867, _, _) => {
                            h867
                        }
                    };
                    nat_le(a868, a869)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a877 = {
                let a875 = match v1.clone() {
                    Adt1::C0(_, _, h870, _, _, _, _, _, _, _, _) => {
                        h870
                    }
                };
                let a876 = {
                    let a873 = match v1.clone() {
                        Adt1::C0(_, _, _, h871, _, _, _, _, _, _, _) => {
                            h871
                        }
                    };
                    let a874 = match v1 {
                        Adt1::C0(_, _, _, _, h872, _, _, _, _, _, _) => {
                            h872
                        }
                    };
                    nat_mul(a873, a874)?
                };
                nat_mul(a875, a876)?
            };
            let a878 = 2147483647u64;
            Ok(nat_le(a877, a878))
        } else {
            Ok(false)
        }
    } else {
        if f32(v1.clone(), v2.clone()) {
            if f28(match v1.clone() {
                Adt1::C0(h879, _, _, _, _, _, _, _, _, _, _) => {
                    h879
                }
            }, match v2.clone() {
                Adt1::C0(h880, _, _, _, _, _, _, _, _, _, _) => {
                    h880
                }
            }, match v3.clone() {
                Adt1::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
                    h881
                }
            }) {
                Ok(true)
            } else {
                let a882 = v0;
                let a883 = 3u64;
                if if if if nat_lt(a882, a883) {
                    f70(match v1.clone() {
                        Adt1::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
                            h884
                        }
                    })
                } else {
                    false
                } {
                    let a887 = match v1.clone() {
                        Adt1::C0(h885, _, _, _, _, _, _, _, _, _, _) => {
                            h885
                        }
                    };
                    let a888 = match v2.clone() {
                        Adt1::C0(h886, _, _, _, _, _, _, _, _, _, _) => {
                            h886
                        }
                    };
                    nat_eq(a887, a888)
                } else {
                    false
                } {
                    let a891 = match v1 {
                        Adt1::C0(h889, _, _, _, _, _, _, _, _, _, _) => {
                            h889
                        }
                    };
                    let a892 = match v3 {
                        Adt1::C0(h890, _, _, _, _, _, _, _, _, _, _) => {
                            h890
                        }
                    };
                    nat_eq(a891, a892)
                } else {
                    false
                } {
                    let a894 = match v2 {
                        Adt1::C0(_, _, _, _, _, h893, _, _, _, _, _) => {
                            h893
                        }
                    };
                    let a895 = 8u64;
                    Ok(nat_eq(a894, a895))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f39(v0: u64) -> u64 {
    if f70(v0) {
        8u64
    } else {
        f73(v0)
    }
}

pub fn f40(v0: Adt1) -> bool {
    let v1: u64 = f39(match v0.clone() {
        Adt1::C0(h896, _, _, _, _, _, _, _, _, _, _) => {
            h896
        }
    });
    let a901 = {
        let a898 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h897) => {
                h897
            }
        };
        let a899 = v1;
        let a900 = 1u64;
        nat_rem(a898, a899, a900)
    };
    let a902 = 0u64;
    if if nat_eq(a901, a902) {
        f76(match v0.clone() {
            Adt1::C0(_, h903, _, _, _, _, _, _, _, _, _) => {
                h903
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h904, _, _, _, _, _) => {
                h904
            }
        }, v1)
    } else {
        false
    } {
        f77(match v0.clone() {
            Adt1::C0(_, _, h905, _, _, _, _, _, _, _, _) => {
                h905
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h906, _, _, _, _, _, _, _) => {
                h906
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h907, _, _, _, _, _, _) => {
                h907
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h908, _, _, _, _) => {
                h908
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h909, _, _, _) => {
                h909
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h910, _, _) => {
                h910
            }
        }, v1)
    } else {
        false
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> R<bool> {
    let a918 = {
        let a916 = match v0.clone() {
            Adt1::C0(_, _, h911, _, _, _, _, _, _, _, _) => {
                h911
            }
        };
        let a917 = {
            let a914 = match v0.clone() {
                Adt1::C0(_, _, _, h912, _, _, _, _, _, _, _) => {
                    h912
                }
            };
            let a915 = match v0.clone() {
                Adt1::C0(_, _, _, _, h913, _, _, _, _, _, _) => {
                    h913
                }
            };
            nat_mul(a914, a915)?
        };
        nat_mul(a916, a917)?
    };
    let a919 = 9223372036854775807u64;
    if nat_le(a918, a919) {
        if f50(v1.clone()) {
            let a921 = match v1 {
                Adt1::C0(_, h920, _, _, _, _, _, _, _, _, _) => {
                    h920
                }
            };
            let a922 = 2147483647u64;
            Ok(nat_le(a921, a922))
        } else {
            let a924 = match v0 {
                Adt1::C0(_, h923, _, _, _, _, _, _, _, _, _) => {
                    h923
                }
            };
            let a925 = 2147483647u64;
            Ok(nat_le(a924, a925))
        }
    } else {
        Ok(false)
    }
}

pub fn f42(v0: Adt1) -> R<bool> {
    f36({
        let a928 = match v0.clone() {
            Adt1::C0(_, h926, _, _, _, _, _, _, _, _, _) => {
                h926
            }
        };
        let a929 = f39(match v0.clone() {
            Adt1::C0(h927, _, _, _, _, _, _, _, _, _, _) => {
                h927
            }
        });
        nat_mul(a928, a929)?
    }, match v0.clone() {
        Adt1::C0(_, _, h930, _, _, _, _, _, _, _, _) => {
            h930
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h931, _, _, _, _, _, _, _) => {
            h931
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h932, _, _, _, _, _, _) => {
            h932
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h933, _, _, _, _) => {
            h933
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h934, _, _, _) => {
            h934
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h935, _, _) => {
            h935
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a942 = {
        let a936 = v0;
        let a937 = v2;
        nat_mul(a936, a937)?
    };
    let a943 = {
        let a938 = v1;
        let a939 = 0u64;
        if nat_eq(a938, a939) {
            0u64
        } else {
            let a940 = v0;
            let a941 = 2u64;
            nat_mul(a940, a941)?
        }
    };
    nat_add(a942, a943)
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if f70(v1) {
        Ok(f78(v0, v1))
    } else {
        f72(v1, v2, v3)
    }
}

pub fn f45(v0: u64) -> Adt4 {
    let a944 = v0;
    let a945 = 0u64;
    if nat_eq(a944, a945) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a946 = v0;
        let a947 = 1u64;
        if nat_eq(a946, a947) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a948 = v0;
            let a949 = 2u64;
            if nat_eq(a948, a949) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a950 = v0;
                let a951 = 3u64;
                if nat_eq(a950, a951) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a952 = v0;
                    let a953 = 6u64;
                    if nat_eq(a952, a953) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a954 = v0;
                        let a955 = 7u64;
                        if nat_eq(a954, a955) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a956 = v0;
                            let a957 = 8u64;
                            if nat_eq(a956, a957) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a958 = v0;
                                let a959 = 9u64;
                                if nat_eq(a958, a959) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a960 = v0;
                                    let a961 = 10u64;
                                    if nat_eq(a960, a961) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a962 = v0;
                                        let a963 = 11u64;
                                        if nat_eq(a962, a963) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a964 = v0;
                                            let a965 = 12u64;
                                            if nat_eq(a964, a965) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a966 = v0;
                                                let a967 = 13u64;
                                                if nat_eq(a966, a967) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a968 = v0;
                                                    let a969 = 14u64;
                                                    if nat_eq(a968, a969) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a970 = v0;
                                                        let a971 = 15u64;
                                                        if nat_eq(a970, a971) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a972 = v0;
                                                            let a973 = 16u64;
                                                            if nat_eq(a972, a973) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a974 = v0;
                                                                let a975 = 17u64;
                                                                if nat_eq(a974, a975) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a976 = v0;
                                                                    let a977 = 18u64;
                                                                    if nat_eq(a976, a977) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a978 = v0;
                                                                        let a979 = 19u64;
                                                                        if nat_eq(a978, a979) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a980 = v0;
                                                                            let a981 = 20u64;
                                                                            if nat_eq(a980, a981) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a982 = v0;
                                                                                let a983 = 21u64;
                                                                                if nat_eq(a982, a983) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a984 = v0;
                                                                                    let a985 = 22u64;
                                                                                    if nat_eq(a984, a985) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a986 = v0;
                                                                                        let a987 = 23u64;
                                                                                        if nat_eq(a986, a987) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a988 = v0;
                                                                                            let a989 = 24u64;
                                                                                            if nat_eq(a988, a989) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a990 = v0;
                                                                                                let a991 = 25u64;
                                                                                                if nat_eq(a990, a991) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a992 = v0;
                                                                                                    let a993 = 26u64;
                                                                                                    if nat_eq(a992, a993) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a994 = v0;
                                                                                                        let a995 = 27u64;
                                                                                                        if nat_eq(a994, a995) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a996 = v0;
                                                                                                            let a997 = 28u64;
                                                                                                            if nat_eq(a996, a997) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a998 = v0;
                                                                                                                let a999 = 29u64;
                                                                                                                if nat_eq(a998, a999) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1000 = v0;
                                                                                                                    let a1001 = 30u64;
                                                                                                                    if nat_eq(a1000, a1001) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1002 = v0;
                                                                                                                        let a1003 = 34u64;
                                                                                                                        if nat_eq(a1002, a1003) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1004 = v0;
                                                                                                                            let a1005 = 35u64;
                                                                                                                            if nat_eq(a1004, a1005) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1006 = v0;
                                                                                                                                let a1007 = 39u64;
                                                                                                                                if nat_eq(a1006, a1007) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1008 = v0;
                                                                                                                                    let a1009 = 40u64;
                                                                                                                                    if nat_eq(a1008, a1009) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1010 = v0;
                                                                                                                                        let a1011 = 41u64;
                                                                                                                                        if nat_eq(a1010, a1011) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1012 = v0;
                                                                                                                                            let a1013 = 42u64;
                                                                                                                                            if nat_eq(a1012, a1013) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1014 = v0;
                                                                                                                                                let a1015 = 43u64;
                                                                                                                                                if nat_eq(a1014, a1015) {
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

pub fn f46(v0: Adt1, v1: Adt1) -> R<bool> {
    let a1017 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1016, _) => {
            h1016
        }
    };
    let a1018 = 0u64;
    if nat_eq(a1017, a1018) {
        Ok(true)
    } else {
        let a1020 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h1019, _) => {
                h1019
            }
        };
        let a1021 = 0u64;
        if nat_eq(a1020, a1021) {
            Ok(true)
        } else {
            let a1023 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1022) => {
                    h1022
                }
            };
            let a1024 = 0u64;
            if nat_eq(a1023, a1024) {
                Ok(true)
            } else {
                let a1026 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1025) => {
                        h1025
                    }
                };
                let a1027 = 0u64;
                if nat_eq(a1026, a1027) {
                    Ok(true)
                } else {
                    let a1033 = {
                        let a1030 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1028) => {
                                h1028
                            }
                        };
                        let a1031 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h1029, _) => {
                                h1029
                            }
                        };
                        nat_add(a1030, a1031)?
                    };
                    let a1034 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1032) => {
                            h1032
                        }
                    };
                    if nat_le(a1033, a1034) {
                        Ok(true)
                    } else {
                        let a1040 = {
                            let a1037 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1035) => {
                                    h1035
                                }
                            };
                            let a1038 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h1036, _) => {
                                    h1036
                                }
                            };
                            nat_add(a1037, a1038)?
                        };
                        let a1041 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1039) => {
                                h1039
                            }
                        };
                        Ok(nat_le(a1040, a1041))
                    }
                }
            }
        }
    }
}

pub fn f47(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a1044 = match v0.clone() {
            Adt1::C0(h1042, _, _, _, _, _, _, _, _, _, _) => {
                h1042
            }
        };
        let a1045 = match v1.clone() {
            Adt1::C0(h1043, _, _, _, _, _, _, _, _, _, _) => {
                h1043
            }
        };
        nat_eq(a1044, a1045)
    } else {
        false
    } {
        let a1048 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1046) => {
                h1046
            }
        };
        let a1049 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1047) => {
                h1047
            }
        };
        nat_eq(a1048, a1049)
    } else {
        false
    } {
        let a1052 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1050, _, _, _, _, _) => {
                h1050
            }
        };
        let a1053 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1051, _, _, _, _, _) => {
                h1051
            }
        };
        nat_eq(a1052, a1053)
    } else {
        false
    } {
        let a1056 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1054, _, _, _, _) => {
                h1054
            }
        };
        let a1057 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1055, _, _, _, _) => {
                h1055
            }
        };
        nat_eq(a1056, a1057)
    } else {
        false
    } {
        let a1060 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1058, _, _, _) => {
                h1058
            }
        };
        let a1061 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1059, _, _, _) => {
                h1059
            }
        };
        nat_eq(a1060, a1061)
    } else {
        false
    } {
        let a1064 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1062, _, _) => {
                h1062
            }
        };
        let a1065 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1063, _, _) => {
                h1063
            }
        };
        nat_eq(a1064, a1065)
    } else {
        false
    }
}

pub fn f48(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a1068 = match v0.clone() {
            Adt1::C0(h1066, _, _, _, _, _, _, _, _, _, _) => {
                h1066
            }
        };
        let a1069 = match v1.clone() {
            Adt1::C0(h1067, _, _, _, _, _, _, _, _, _, _) => {
                h1067
            }
        };
        nat_eq(a1068, a1069)
    } else {
        false
    } {
        let a1072 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1070, _, _, _, _, _) => {
                h1070
            }
        };
        let a1073 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1071, _, _, _, _, _) => {
                h1071
            }
        };
        nat_eq(a1072, a1073)
    } else {
        false
    } {
        let a1076 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1074, _, _, _, _) => {
                h1074
            }
        };
        let a1077 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1075, _, _, _, _) => {
                h1075
            }
        };
        nat_eq(a1076, a1077)
    } else {
        false
    } {
        let a1080 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1078, _, _, _) => {
                h1078
            }
        };
        let a1081 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1079, _, _, _) => {
                h1079
            }
        };
        nat_eq(a1080, a1081)
    } else {
        false
    } {
        let a1084 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1082, _, _) => {
                h1082
            }
        };
        let a1085 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1083, _, _) => {
                h1083
            }
        };
        nat_eq(a1084, a1085)
    } else {
        false
    } {
        let a1087 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1086) => {
                h1086
            }
        };
        let a1088 = 0u64;
        if if nat_eq(a1087, a1088) {
            true
        } else {
            let a1090 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1089) => {
                    h1089
                }
            };
            let a1091 = 0u64;
            nat_eq(a1090, a1091)
        } {
            true
        } else {
            let a1094 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1092) => {
                    h1092
                }
            };
            let a1095 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1093) => {
                    h1093
                }
            };
            nat_eq(a1094, a1095)
        }
    } else {
        false
    }
}

pub fn f49(v0: u64) -> u64 {
    let a1096 = v0;
    let a1097 = 26u64;
    if nat_eq(a1096, a1097) {
        4u64
    } else {
        let a1098 = v0;
        let a1099 = 27u64;
        if nat_eq(a1098, a1099) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f50(v0: Adt1) -> bool {
    let a1101 = match v0.clone() {
        Adt1::C0(_, h1100, _, _, _, _, _, _, _, _, _) => {
            h1100
        }
    };
    let a1102 = 1u64;
    if nat_eq(a1101, a1102) {
        true
    } else {
        let a1105 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1103, _, _, _, _, _) => {
                h1103
            }
        };
        let a1106 = f73(match v0 {
            Adt1::C0(h1104, _, _, _, _, _, _, _, _, _, _) => {
                h1104
            }
        });
        nat_eq(a1105, a1106)
    }
}

pub fn f51(v0: Adt1, v1: u64) -> R<u64> {
    let a1112 = {
        let a1108 = match v0.clone() {
            Adt1::C0(_, h1107, _, _, _, _, _, _, _, _, _) => {
                h1107
            }
        };
        let a1109 = f35(v1);
        let a1110 = 0u64;
        nat_quot(a1108, a1109, a1110)
    };
    let a1113 = f79(match v0 {
        Adt1::C0(h1111, _, _, _, _, _, _, _, _, _, _) => {
            h1111
        }
    }, v1);
    nat_mul(a1112, a1113)
}

pub fn f52(v0: u64) -> u64 {
    if if f27(v0) {
        true
    } else {
        let a1114 = 24u64;
        let a1115 = v0;
        if nat_le(a1114, a1115) {
            let a1116 = v0;
            let a1117 = 28u64;
            nat_le(a1116, a1117)
        } else {
            false
        }
    } {
        f35(v0)
    } else {
        0u64
    }
}

pub fn f53(v0: u64, v1: Adt1) -> R<u64> {
    let a1124 = {
        let a1118 = 1u64;
        let a1119 = v0;
        nat_add(a1118, a1119)?
    };
    let a1125 = {
        let a1121 = match v1.clone() {
            Adt1::C0(_, h1120, _, _, _, _, _, _, _, _, _) => {
                h1120
            }
        };
        let a1122 = f55(v1);
        let a1123 = 0u64;
        nat_quot(a1121, a1122, a1123)
    };
    nat_add(a1124, a1125)
}

pub fn f54(v0: Adt1, v1: Adt1) -> u64 {
    let a1128 = match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1126, _, _, _, _, _) => {
            h1126
        }
    };
    let a1129 = f7(match v1.clone() {
        Adt1::C0(h1127, _, _, _, _, _, _, _, _, _, _) => {
            h1127
        }
    });
    if nat_eq(a1128, a1129) {
        f80(v0, v1)
    } else {
        0u64
    }
}

pub fn f55(v0: Adt1) -> u64 {
    let a1132 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1130, _, _, _, _, _) => {
            h1130
        }
    };
    let a1133 = f7(match v0.clone() {
        Adt1::C0(h1131, _, _, _, _, _, _, _, _, _, _) => {
            h1131
        }
    });
    if nat_eq(a1132, a1133) {
        match v0 {
            Adt1::C0(_, h1134, _, _, _, _, _, _, _, _, _) => {
                h1134
            }
        }
    } else {
        f35(match v0 {
            Adt1::C0(h1135, _, _, _, _, _, _, _, _, _, _) => {
                h1135
            }
        })
    }
}

pub fn f56(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1137 = match v0.clone() {
            Adt1::C0(_, h1136, _, _, _, _, _, _, _, _, _) => {
                h1136
            }
        };
        let a1138 = 4u64;
        nat_mul(a1137, a1138)?
    };
    let v2: u64 = {
        let a1141 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1139, _, _, _, _) => {
                h1139
            }
        };
        let a1142 = match v0.clone() {
            Adt1::C0(_, _, h1140, _, _, _, _, _, _, _, _) => {
                h1140
            }
        };
        nat_mul(a1141, a1142)?
    };
    let v3: u64 = {
        let a1144 = v2;
        let a1145 = match v0.clone() {
            Adt1::C0(_, _, _, h1143, _, _, _, _, _, _, _) => {
                h1143
            }
        };
        nat_mul(a1144, a1145)?
    };
    let a1147 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1146, _, _, _, _, _) => {
            h1146
        }
    };
    let a1148 = 4u64;
    if if if if nat_eq(a1147, a1148) {
        let a1153 = {
            let a1150 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1149) => {
                    h1149
                }
            };
            let a1151 = 4u64;
            let a1152 = 1u64;
            nat_rem(a1150, a1151, a1152)
        };
        let a1154 = 0u64;
        nat_eq(a1153, a1154)
    } else {
        false
    } {
        let a1162 = {
            let a1160 = match v0.clone() {
                Adt1::C0(_, _, h1155, _, _, _, _, _, _, _, _) => {
                    h1155
                }
            };
            let a1161 = {
                let a1158 = match v0.clone() {
                    Adt1::C0(_, _, _, h1156, _, _, _, _, _, _, _) => {
                        h1156
                    }
                };
                let a1159 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h1157, _, _, _, _, _, _) => {
                        h1157
                    }
                };
                nat_mul(a1158, a1159)?
            };
            nat_mul(a1160, a1161)?
        };
        let a1163 = 1u64;
        if nat_le(a1162, a1163) {
            true
        } else {
            let a1165 = v1;
            let a1166 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1164, _, _, _, _) => {
                    h1164
                }
            };
            if nat_le(a1165, a1166) {
                let a1171 = {
                    let a1168 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h1167, _, _, _, _) => {
                            h1167
                        }
                    };
                    let a1169 = 4u64;
                    let a1170 = 1u64;
                    nat_rem(a1168, a1169, a1170)
                };
                let a1172 = 0u64;
                nat_eq(a1171, a1172)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a1174 = match v0.clone() {
            Adt1::C0(_, _, _, h1173, _, _, _, _, _, _, _) => {
                h1173
            }
        };
        let a1175 = 1u64;
        if nat_le(a1174, a1175) {
            true
        } else {
            let a1177 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1176, _, _, _) => {
                    h1176
                }
            };
            let a1178 = v2;
            nat_eq(a1177, a1178)
        }
    } else {
        false
    } {
        let a1180 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1179, _, _, _, _, _, _) => {
                h1179
            }
        };
        let a1181 = 1u64;
        if nat_le(a1180, a1181) {
            Ok(true)
        } else {
            let a1183 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h1182, _, _) => {
                    h1182
                }
            };
            let a1184 = v3;
            Ok(nat_eq(a1183, a1184))
        }
    } else {
        Ok(false)
    }
}

pub fn f57(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a1185 = v2;
    let a1186 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a1185, a1186) {
        let a1188 = 0u64;
        let a1189 = f30(match v5.clone() {
            Adt1::C0(h1187, _, _, _, _, _, _, _, _, _, _) => {
                h1187
            }
        });
        nat_lt(a1188, a1189)
    } else {
        false
    } {
        let a1192 = match v5.clone() {
            Adt1::C0(h1190, _, _, _, _, _, _, _, _, _, _) => {
                h1190
            }
        };
        let a1193 = match v6.clone() {
            Adt1::C0(h1191, _, _, _, _, _, _, _, _, _, _) => {
                h1191
            }
        };
        nat_eq(a1192, a1193)
    } else {
        false
    } {
        let a1196 = match v5.clone() {
            Adt1::C0(h1194, _, _, _, _, _, _, _, _, _, _) => {
                h1194
            }
        };
        let a1197 = match v7.clone() {
            Adt1::C0(h1195, _, _, _, _, _, _, _, _, _, _) => {
                h1195
            }
        };
        nat_eq(a1196, a1197)
    } else {
        false
    } {
        f5(v5.clone(), v6.clone())
    } else {
        false
    } {
        f81(v5.clone())?
    } else {
        false
    } {
        f81(v6.clone())?
    } else {
        false
    } {
        f81(v7.clone())?
    } else {
        false
    } {
        let a1200 = match v5.clone() {
            Adt1::C0(_, _, h1198, _, _, _, _, _, _, _, _) => {
                h1198
            }
        };
        let a1201 = match v7.clone() {
            Adt1::C0(_, _, h1199, _, _, _, _, _, _, _, _) => {
                h1199
            }
        };
        nat_eq(a1200, a1201)
    } else {
        false
    } {
        let a1204 = match v5.clone() {
            Adt1::C0(_, _, _, h1202, _, _, _, _, _, _, _) => {
                h1202
            }
        };
        let a1205 = match v7.clone() {
            Adt1::C0(_, _, _, h1203, _, _, _, _, _, _, _) => {
                h1203
            }
        };
        nat_eq(a1204, a1205)
    } else {
        false
    } {
        let a1208 = match v5.clone() {
            Adt1::C0(_, _, _, _, h1206, _, _, _, _, _, _) => {
                h1206
            }
        };
        let a1209 = match v7.clone() {
            Adt1::C0(_, _, _, _, h1207, _, _, _, _, _, _) => {
                h1207
            }
        };
        nat_eq(a1208, a1209)
    } else {
        false
    } {
        let a1216 = match v7.clone() {
            Adt1::C0(_, h1210, _, _, _, _, _, _, _, _, _) => {
                h1210
            }
        };
        let a1217 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h1211, _, _, _, _, _, _, _, _, _) => {
                    h1211
                }
            }
        } else {
            let a1213 = match v5.clone() {
                Adt1::C0(_, h1212, _, _, _, _, _, _, _, _, _) => {
                    h1212
                }
            };
            let a1214 = 2u64;
            let a1215 = 0u64;
            nat_quot(a1213, a1214, a1215)
        };
        nat_eq(a1216, a1217)
    } else {
        false
    } {
        let a1219 = match v7.clone() {
            Adt1::C0(_, h1218, _, _, _, _, _, _, _, _, _) => {
                h1218
            }
        };
        let a1220 = 2147483647u64;
        nat_le(a1219, a1220)
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
        Ok(Adt3::C2(f58(v0, v1, v2, v3, v4, v5, v6, v7)?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f58(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt2> {
    Ok(Adt2::C0(if f23(v0) {
        let a1223 = 1u64;
        let a1224 = {
            let a1221 = f65(v0);
            let a1222 = f9(v7.clone())?;
            nat_mul(a1221, a1222)?
        };
        nat_add(a1223, a1224)?
    } else {
        let a1225 = 4u64;
        let a1226 = f9(v7.clone())?;
        nat_mul(a1225, a1226)?
    }, if f23(v0) {
        4u64
    } else {
        3u64
    }, None::<u64>, if v1 {
        98u64
    } else {
        let a1227 = 99u64;
        let a1228 = v2;
        nat_add(a1227, a1228)?
    }, v0, v3, v4, v5, v6, v7, false, 0u64))
}

pub fn f59(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<u64> {
    let v4: u64 = {
        let a1231 = match v2.clone() {
            Adt1::C0(_, h1229, _, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1232 = match v3.clone() {
            Adt1::C0(_, h1230, _, _, _, _, _, _, _, _, _) => {
                h1230
            }
        };
        let a1233 = 0u64;
        nat_quot(a1231, a1232, a1233)
    };
    let v5: u64 = {
        let a1236 = match v2.clone() {
            Adt1::C0(_, _, h1234, _, _, _, _, _, _, _, _) => {
                h1234
            }
        };
        let a1237 = match v3.clone() {
            Adt1::C0(_, _, h1235, _, _, _, _, _, _, _, _) => {
                h1235
            }
        };
        let a1238 = 0u64;
        nat_quot(a1236, a1237, a1238)
    };
    let v6: u64 = {
        let a1241 = match v2.clone() {
            Adt1::C0(_, _, _, h1239, _, _, _, _, _, _, _) => {
                h1239
            }
        };
        let a1242 = match v3.clone() {
            Adt1::C0(_, _, _, h1240, _, _, _, _, _, _, _) => {
                h1240
            }
        };
        let a1243 = 0u64;
        nat_quot(a1241, a1242, a1243)
    };
    let v7: u64 = {
        let a1244 = v1;
        let a1245 = v4;
        let a1246 = 0u64;
        nat_quot(a1244, a1245, a1246)
    };
    let v8: u64 = {
        let a1247 = v7;
        let a1248 = v5;
        let a1249 = 0u64;
        nat_quot(a1247, a1248, a1249)
    };
    let v9: u64 = {
        let a1251 = v0;
        let a1252 = match v3.clone() {
            Adt1::C0(_, _, h1250, _, _, _, _, _, _, _, _) => {
                h1250
            }
        };
        let a1253 = 0u64;
        nat_quot(a1251, a1252, a1253)
    };
    f82({
        let a1261 = {
            let a1258 = {
                let a1254 = v1;
                let a1255 = v4;
                let a1256 = 0u64;
                nat_rem(a1254, a1255, a1256)
            };
            let a1259 = match v3.clone() {
                Adt1::C0(_, h1257, _, _, _, _, _, _, _, _, _) => {
                    h1257
                }
            };
            nat_mul(a1258, a1259)?
        };
        let a1262 = f35(match v2.clone() {
            Adt1::C0(h1260, _, _, _, _, _, _, _, _, _, _) => {
                h1260
            }
        });
        let a1263 = 0u64;
        nat_quot(a1261, a1262, a1263)
    }, {
        let a1274 = {
            let a1268 = {
                let a1264 = v7;
                let a1265 = v5;
                let a1266 = 0u64;
                nat_rem(a1264, a1265, a1266)
            };
            let a1269 = match v3.clone() {
                Adt1::C0(_, _, h1267, _, _, _, _, _, _, _, _) => {
                    h1267
                }
            };
            nat_mul(a1268, a1269)?
        };
        let a1275 = {
            let a1271 = v0;
            let a1272 = match v3.clone() {
                Adt1::C0(_, _, h1270, _, _, _, _, _, _, _, _) => {
                    h1270
                }
            };
            let a1273 = 0u64;
            nat_rem(a1271, a1272, a1273)
        };
        nat_add(a1274, a1275)?
    }, {
        let a1286 = {
            let a1280 = {
                let a1276 = v8;
                let a1277 = v6;
                let a1278 = 0u64;
                nat_rem(a1276, a1277, a1278)
            };
            let a1281 = match v3.clone() {
                Adt1::C0(_, _, _, h1279, _, _, _, _, _, _, _) => {
                    h1279
                }
            };
            nat_mul(a1280, a1281)?
        };
        let a1287 = {
            let a1283 = v9;
            let a1284 = match v3.clone() {
                Adt1::C0(_, _, _, h1282, _, _, _, _, _, _, _) => {
                    h1282
                }
            };
            let a1285 = 0u64;
            nat_rem(a1283, a1284, a1285)
        };
        nat_add(a1286, a1287)?
    }, {
        let a1298 = {
            let a1292 = {
                let a1288 = v8;
                let a1289 = v6;
                let a1290 = 0u64;
                nat_quot(a1288, a1289, a1290)
            };
            let a1293 = match v3.clone() {
                Adt1::C0(_, _, _, _, h1291, _, _, _, _, _, _) => {
                    h1291
                }
            };
            nat_mul(a1292, a1293)?
        };
        let a1299 = {
            let a1295 = v9;
            let a1296 = match v3 {
                Adt1::C0(_, _, _, h1294, _, _, _, _, _, _, _) => {
                    h1294
                }
            };
            let a1297 = 0u64;
            nat_quot(a1295, a1296, a1297)
        };
        nat_add(a1298, a1299)?
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1300, _, _, _, _, _) => {
            h1300
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1301, _, _, _, _) => {
            h1301
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1302, _, _, _) => {
            h1302
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1303, _, _) => {
            h1303
        }
    })
}

pub fn f60(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1305 = v0;
        let a1306 = match v1.clone() {
            Adt1::C0(_, h1304, _, _, _, _, _, _, _, _, _) => {
                h1304
            }
        };
        let a1307 = 0u64;
        nat_quot(a1305, a1306, a1307)
    };
    let v3: u64 = {
        let a1309 = v2;
        let a1310 = match v1.clone() {
            Adt1::C0(_, _, h1308, _, _, _, _, _, _, _, _) => {
                h1308
            }
        };
        let a1311 = 0u64;
        nat_quot(a1309, a1310, a1311)
    };
    let v4: u64 = {
        let a1317 = {
            let a1313 = v0;
            let a1314 = match v1.clone() {
                Adt1::C0(_, h1312, _, _, _, _, _, _, _, _, _) => {
                    h1312
                }
            };
            let a1315 = 0u64;
            nat_rem(a1313, a1314, a1315)
        };
        let a1318 = f35(match v1.clone() {
            Adt1::C0(h1316, _, _, _, _, _, _, _, _, _, _) => {
                h1316
            }
        });
        let a1319 = 0u64;
        nat_quot(a1317, a1318, a1319)
    };
    f82(v4, {
        let a1321 = v2;
        let a1322 = match v1.clone() {
            Adt1::C0(_, _, h1320, _, _, _, _, _, _, _, _) => {
                h1320
            }
        };
        let a1323 = 0u64;
        nat_rem(a1321, a1322, a1323)
    }, {
        let a1325 = v3;
        let a1326 = match v1.clone() {
            Adt1::C0(_, _, _, h1324, _, _, _, _, _, _, _) => {
                h1324
            }
        };
        let a1327 = 0u64;
        nat_rem(a1325, a1326, a1327)
    }, {
        let a1329 = v3;
        let a1330 = match v1.clone() {
            Adt1::C0(_, _, _, h1328, _, _, _, _, _, _, _) => {
                h1328
            }
        };
        let a1331 = 0u64;
        nat_quot(a1329, a1330, a1331)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1332, _, _, _, _, _) => {
            h1332
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1333, _, _, _, _) => {
            h1333
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1334, _, _, _) => {
            h1334
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1335, _, _) => {
            h1335
        }
    })
}

pub fn f61(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1337 = v0;
        let a1338 = match v1.clone() {
            Adt1::C0(_, h1336, _, _, _, _, _, _, _, _, _) => {
                h1336
            }
        };
        let a1339 = 0u64;
        nat_quot(a1337, a1338, a1339)
    };
    let v3: u64 = {
        let a1341 = v2;
        let a1342 = match v1.clone() {
            Adt1::C0(_, _, h1340, _, _, _, _, _, _, _, _) => {
                h1340
            }
        };
        let a1343 = 0u64;
        nat_quot(a1341, a1342, a1343)
    };
    f82({
        let a1345 = v0;
        let a1346 = match v1.clone() {
            Adt1::C0(_, h1344, _, _, _, _, _, _, _, _, _) => {
                h1344
            }
        };
        let a1347 = 0u64;
        nat_rem(a1345, a1346, a1347)
    }, {
        let a1349 = v2;
        let a1350 = match v1.clone() {
            Adt1::C0(_, _, h1348, _, _, _, _, _, _, _, _) => {
                h1348
            }
        };
        let a1351 = 0u64;
        nat_rem(a1349, a1350, a1351)
    }, {
        let a1353 = v3;
        let a1354 = match v1.clone() {
            Adt1::C0(_, _, _, h1352, _, _, _, _, _, _, _) => {
                h1352
            }
        };
        let a1355 = 0u64;
        nat_rem(a1353, a1354, a1355)
    }, {
        let a1357 = v3;
        let a1358 = match v1.clone() {
            Adt1::C0(_, _, _, h1356, _, _, _, _, _, _, _) => {
                h1356
            }
        };
        let a1359 = 0u64;
        nat_quot(a1357, a1358, a1359)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1360, _, _, _, _, _) => {
            h1360
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1361, _, _, _, _) => {
            h1361
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1362, _, _, _) => {
            h1362
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1363, _, _) => {
            h1363
        }
    })
}

pub fn f62(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1365 = v0;
        let a1366 = match v1.clone() {
            Adt1::C0(_, h1364, _, _, _, _, _, _, _, _, _) => {
                h1364
            }
        };
        let a1367 = 0u64;
        nat_quot(a1365, a1366, a1367)
    };
    let v4: u64 = {
        let a1369 = v3;
        let a1370 = match v1.clone() {
            Adt1::C0(_, _, h1368, _, _, _, _, _, _, _, _) => {
                h1368
            }
        };
        let a1371 = 0u64;
        nat_quot(a1369, a1370, a1371)
    };
    f82({
        let a1377 = {
            let a1373 = v0;
            let a1374 = match v1.clone() {
                Adt1::C0(_, h1372, _, _, _, _, _, _, _, _, _) => {
                    h1372
                }
            };
            let a1375 = 0u64;
            nat_rem(a1373, a1374, a1375)
        };
        let a1378 = match v2.clone() {
            Adt1::C0(_, h1376, _, _, _, _, _, _, _, _, _) => {
                h1376
            }
        };
        let a1379 = 0u64;
        nat_rem(a1377, a1378, a1379)
    }, {
        let a1385 = {
            let a1381 = v3;
            let a1382 = match v1.clone() {
                Adt1::C0(_, _, h1380, _, _, _, _, _, _, _, _) => {
                    h1380
                }
            };
            let a1383 = 0u64;
            nat_rem(a1381, a1382, a1383)
        };
        let a1386 = match v2.clone() {
            Adt1::C0(_, _, h1384, _, _, _, _, _, _, _, _) => {
                h1384
            }
        };
        let a1387 = 0u64;
        nat_rem(a1385, a1386, a1387)
    }, {
        let a1393 = {
            let a1389 = v4;
            let a1390 = match v1.clone() {
                Adt1::C0(_, _, _, h1388, _, _, _, _, _, _, _) => {
                    h1388
                }
            };
            let a1391 = 0u64;
            nat_rem(a1389, a1390, a1391)
        };
        let a1394 = match v2.clone() {
            Adt1::C0(_, _, _, h1392, _, _, _, _, _, _, _) => {
                h1392
            }
        };
        let a1395 = 0u64;
        nat_rem(a1393, a1394, a1395)
    }, {
        let a1401 = {
            let a1397 = v4;
            let a1398 = match v1 {
                Adt1::C0(_, _, _, h1396, _, _, _, _, _, _, _) => {
                    h1396
                }
            };
            let a1399 = 0u64;
            nat_quot(a1397, a1398, a1399)
        };
        let a1402 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1400, _, _, _, _, _, _) => {
                h1400
            }
        };
        let a1403 = 0u64;
        nat_rem(a1401, a1402, a1403)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1404, _, _, _, _, _) => {
            h1404
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1405, _, _, _, _) => {
            h1405
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1406, _, _, _) => {
            h1406
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1407, _, _) => {
            h1407
        }
    })
}

pub fn f63(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1408 = v0;
        let a1409 = 2u64;
        let a1410 = 0u64;
        nat_quot(a1408, a1409, a1410)
    };
    let v5: u64 = f49(match v1.clone() {
        Adt1::C0(h1411, _, _, _, _, _, _, _, _, _, _) => {
            h1411
        }
    });
    let a1415 = {
        let a1412 = v0;
        let a1413 = 2u64;
        let a1414 = 0u64;
        nat_rem(a1412, a1413, a1414)
    };
    let a1416 = 0u64;
    if nat_eq(a1415, a1416) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f61(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1417 = v3;
        let a1418 = v2;
        if if nat_lt(a1417, a1418) {
            let a1421 = v3;
            let a1422 = {
                let a1419 = v5;
                let a1420 = 4u64;
                if nat_eq(a1419, a1420) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1421, a1422)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1423 = 3u64;
                let a1424 = v4;
                nat_add(a1423, a1424)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f64(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1426 = 1u64;
        let a1427 = f51(v1.clone(), match v3.clone() {
            Adt1::C0(h1425, _, _, _, _, _, _, _, _, _, _) => {
                h1425
            }
        })?;
        nat_add(a1426, a1427)?
    };
    let v6: u64 = {
        let a1428 = v0;
        let a1429 = v5;
        let a1430 = 0u64;
        nat_rem(a1428, a1429, a1430)
    };
    let v7: u64 = {
        let a1431 = v0;
        let a1432 = v5;
        let a1433 = 0u64;
        nat_quot(a1431, a1432, a1433)
    };
    let v8: u64 = {
        let a1435 = v7;
        let a1436 = match v1.clone() {
            Adt1::C0(_, _, h1434, _, _, _, _, _, _, _, _) => {
                h1434
            }
        };
        let a1437 = 0u64;
        nat_rem(a1435, a1436, a1437)
    };
    let v9: u64 = {
        let a1439 = v7;
        let a1440 = match v1.clone() {
            Adt1::C0(_, _, h1438, _, _, _, _, _, _, _, _) => {
                h1438
            }
        };
        let a1441 = 0u64;
        nat_quot(a1439, a1440, a1441)
    };
    let v10: u64 = {
        let a1443 = v9;
        let a1444 = match v1.clone() {
            Adt1::C0(_, _, _, h1442, _, _, _, _, _, _, _) => {
                h1442
            }
        };
        let a1445 = 0u64;
        nat_rem(a1443, a1444, a1445)
    };
    let v11: u64 = {
        let a1447 = v9;
        let a1448 = match v1.clone() {
            Adt1::C0(_, _, _, h1446, _, _, _, _, _, _, _) => {
                h1446
            }
        };
        let a1449 = 0u64;
        nat_quot(a1447, a1448, a1449)
    };
    let a1450 = v6;
    let a1451 = 0u64;
    if nat_eq(a1450, a1451) {
        let v12: u64 = {
            let a1468 = v8;
            let a1469 = {
                let a1466 = match v2.clone() {
                    Adt1::C0(_, h1452, _, _, _, _, _, _, _, _, _) => {
                        h1452
                    }
                };
                let a1467 = {
                    let a1464 = {
                        let a1454 = v10;
                        let a1455 = match v2.clone() {
                            Adt1::C0(_, _, h1453, _, _, _, _, _, _, _, _) => {
                                h1453
                            }
                        };
                        let a1456 = 0u64;
                        nat_rem(a1454, a1455, a1456)
                    };
                    let a1465 = {
                        let a1462 = match v2.clone() {
                            Adt1::C0(_, _, h1457, _, _, _, _, _, _, _, _) => {
                                h1457
                            }
                        };
                        let a1463 = {
                            let a1459 = v11;
                            let a1460 = match v2 {
                                Adt1::C0(_, _, _, h1458, _, _, _, _, _, _, _) => {
                                    h1458
                                }
                            };
                            let a1461 = 0u64;
                            nat_rem(a1459, a1460, a1461)
                        };
                        nat_mul(a1462, a1463)?
                    };
                    nat_add(a1464, a1465)?
                };
                nat_mul(a1466, a1467)?
            };
            nat_add(a1468, a1469)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1470 = 3u64;
            let a1471 = v12;
            nat_add(a1470, a1471)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1473 = v4;
        let a1474 = match v3.clone() {
            Adt1::C0(_, _, h1472, _, _, _, _, _, _, _, _) => {
                h1472
            }
        };
        if nat_lt(a1473, a1474) {
            let v13: u64 = {
                let a1479 = {
                    let a1475 = v6;
                    let a1476 = 1u64;
                    nat_sub(a1475, a1476)
                };
                let a1480 = f79(match v1.clone() {
                    Adt1::C0(h1477, _, _, _, _, _, _, _, _, _, _) => {
                        h1477
                    }
                }, match v3.clone() {
                    Adt1::C0(h1478, _, _, _, _, _, _, _, _, _, _) => {
                        h1478
                    }
                });
                let a1481 = 0u64;
                nat_quot(a1479, a1480, a1481)
            };
            let v14: u64 = f82({
                let a1483 = v13;
                let a1484 = f35(match v3.clone() {
                    Adt1::C0(h1482, _, _, _, _, _, _, _, _, _, _) => {
                        h1482
                    }
                });
                nat_mul(a1483, a1484)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1485, _, _, _, _, _) => {
                    h1485
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1486, _, _, _, _) => {
                    h1486
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1487, _, _, _) => {
                    h1487
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1488, _, _) => {
                    h1488
                }
            })?;
            let v15: u64 = f82(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1489, _, _, _, _, _) => {
                    h1489
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1490, _, _, _, _) => {
                    h1490
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1491, _, _, _) => {
                    h1491
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1492, _, _) => {
                    h1492
                }
            })?;
            let a1494 = 0u64;
            let a1495 = f52(match v3.clone() {
                Adt1::C0(h1493, _, _, _, _, _, _, _, _, _, _) => {
                    h1493
                }
            });
            if nat_lt(a1494, a1495) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f83(match v1 {
                    Adt1::C0(h1496, _, _, _, _, _, _, _, _, _, _) => {
                        h1496
                    }
                }, match v3 {
                    Adt1::C0(h1497, _, _, _, _, _, _, _, _, _, _) => {
                        h1497
                    }
                }, {
                    let a1498 = v6;
                    let a1499 = 1u64;
                    nat_sub(a1498, a1499)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f65(v0: u64) -> u64 {
    let a1500 = v0;
    let a1501 = 3u64;
    if nat_eq(a1500, a1501) {
        16u64
    } else {
        10u64
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f67(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f68(v0: u64, v1: u64) -> Adt3 {
    let a1502 = v0;
    let a1503 = 8u64;
    if nat_eq(a1502, a1503) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1504 = v0;
        let a1505 = 9u64;
        if nat_eq(a1504, a1505) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1506 = v0;
            let a1507 = 10u64;
            if nat_eq(a1506, a1507) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1508 = v0;
                let a1509 = 11u64;
                if nat_eq(a1508, a1509) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1510 = v0;
                    let a1511 = 12u64;
                    if nat_eq(a1510, a1511) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1512 = v0;
                        let a1513 = 13u64;
                        if nat_eq(a1512, a1513) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1514 = v0;
                            let a1515 = 14u64;
                            if nat_eq(a1514, a1515) {
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

pub fn f69(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1516, _, _, _, _) => {
            h1516
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1517, _, _, _) => {
            h1517
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1518, _, _) => {
            h1518
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1519, _, _, _, _, _, _, _, _) => {
            h1519
        }
    };
    let v7: u64 = {
        let a1521 = v1;
        let a1522 = match v5.clone() {
            Adt1::C0(_, h1520, _, _, _, _, _, _, _, _, _) => {
                h1520
            }
        };
        let a1523 = 0u64;
        nat_quot(a1521, a1522, a1523)
    };
    let v8: u64 = f30(match v5.clone() {
        Adt1::C0(h1524, _, _, _, _, _, _, _, _, _, _) => {
            h1524
        }
    });
    let v9: u64 = {
        let a1529 = {
            let a1526 = v1;
            let a1527 = match v5.clone() {
                Adt1::C0(_, h1525, _, _, _, _, _, _, _, _, _) => {
                    h1525
                }
            };
            let a1528 = 0u64;
            nat_rem(a1526, a1527, a1528)
        };
        let a1530 = v8;
        nat_mul(a1529, a1530)?
    };
    let v10: u64 = {
        let a1532 = match v5.clone() {
            Adt1::C0(_, h1531, _, _, _, _, _, _, _, _, _) => {
                h1531
            }
        };
        let a1533 = v8;
        nat_mul(a1532, a1533)?
    };
    let a1534 = v0;
    let a1535 = 0u64;
    if nat_eq(a1534, a1535) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1536, _, _, _, _, _, _, _, _, _, _) => {
                h1536
            }
        }, 0u64, 0u64, {
            let a1544 = {
                let a1540 = {
                    let a1538 = v7;
                    let a1539 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1537, _, _, _, _) => {
                            h1537
                        }
                    };
                    nat_mul(a1538, a1539)?
                };
                let a1541 = v9;
                nat_add(a1540, a1541)?
            };
            let a1545 = {
                let a1542 = v6;
                let a1543 = 100u64;
                if nat_eq(a1542, a1543) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1544, a1545)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1546 = v0;
        let a1547 = 1u64;
        if nat_eq(a1546, a1547) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1548, _, _, _, _, _, _, _, _, _, _) => {
                    h1548
                }
            }, 0u64, {
                let a1549 = v6;
                let a1550 = 98u64;
                if nat_eq(a1549, a1550) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1558 = {
                    let a1554 = {
                        let a1552 = v7;
                        let a1553 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1551, _, _, _, _) => {
                                h1551
                            }
                        };
                        nat_mul(a1552, a1553)?
                    };
                    let a1555 = v9;
                    nat_add(a1554, a1555)?
                };
                let a1559 = {
                    let a1556 = v6;
                    let a1557 = 99u64;
                    if nat_eq(a1556, a1557) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1558, a1559)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1560 = v0;
            let a1561 = 2u64;
            if nat_eq(a1560, a1561) {
                Ok(Adt3::C4(f84(match v2 {
                    Adt2::C0(_, _, _, _, h1562, _, _, _, _, _, _, _) => {
                        h1562
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1563, _, _, _, _, _, _, _, _, _, _) => {
                        h1563
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1564, _, _, _, _, _, _, _, _, _, _) => {
                        h1564
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1568 = {
                        let a1566 = v7;
                        let a1567 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1565, _, _, _, _) => {
                                h1565
                            }
                        };
                        nat_mul(a1566, a1567)?
                    };
                    let a1569 = v9;
                    nat_add(a1568, a1569)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f70(v0: u64) -> bool {
    let a1570 = v0;
    let a1571 = 27u64;
    if nat_eq(a1570, a1571) {
        true
    } else {
        let a1572 = v0;
        let a1573 = 28u64;
        nat_eq(a1572, a1573)
    }
}

pub fn f71(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1574, _, _, _, _) => {
            h1574
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1575, _, _, _) => {
            h1575
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1576, _, _) => {
            h1576
        }
    };
    let v5: u64 = match v1 {
        Adt2::C0(_, _, _, h1577, _, _, _, _, _, _, _, _) => {
            h1577
        }
    };
    let v6: u64 = f78(v5, match v2.clone() {
        Adt1::C0(h1578, _, _, _, _, _, _, _, _, _, _) => {
            h1578
        }
    });
    let v7: u64 = {
        let a1579 = v0;
        let a1580 = v6;
        let a1581 = 0u64;
        nat_quot(a1579, a1580, a1581)
    };
    let v8: u64 = f62(v7, v2.clone(), v3)?;
    let a1582 = v6;
    let a1583 = 2u64;
    if if nat_eq(a1582, a1583) {
        let a1587 = {
            let a1584 = v0;
            let a1585 = v6;
            let a1586 = 0u64;
            nat_rem(a1584, a1585, a1586)
        };
        let a1588 = 0u64;
        nat_eq(a1587, a1588)
    } else {
        false
    } {
        Ok(Adt3::C3(6u64, 15u64, 0u64, 0u64, 4u64, v8, 2u64, 9223372036854775808u64, 0u64, 0u64, 8u64, 8u64))
    } else {
        let v9: u64 = {
            let a1590 = match v2.clone() {
                Adt1::C0(h1589, _, _, _, _, _, _, _, _, _, _) => {
                    h1589
                }
            };
            let a1591 = 27u64;
            if nat_eq(a1590, a1591) {
                let a1592 = 15u64;
                let a1593 = v5;
                nat_add(a1592, a1593)?
            } else {
                let a1594 = v5;
                let a1595 = 2u64;
                if nat_eq(a1594, a1595) {
                    14u64
                } else {
                    0u64
                }
            }
        };
        Ok(Adt3::C3(6u64, v9, 0u64, 0u64, 0u64, f61(v7, v2)?, {
            let a1596 = v6;
            let a1597 = 2u64;
            if nat_eq(a1596, a1597) {
                1u64
            } else {
                4u64
            }
        }, {
            let a1598 = v6;
            let a1599 = 2u64;
            if nat_eq(a1598, a1599) {
                0u64
            } else {
                v8
            }
        }, 3u64, f61(v7, v4)?, 8u64, 8u64))
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1610 = {
        let a1606 = {
            let a1602 = 1u64;
            let a1603 = {
                let a1600 = v0;
                let a1601 = 0u64;
                if nat_eq(a1600, a1601) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a1602, a1603)?
        };
        let a1607 = {
            let a1604 = v1;
            let a1605 = 0u64;
            if nat_eq(a1604, a1605) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a1606, a1607)?
    };
    let a1611 = {
        let a1608 = v2;
        let a1609 = 0u64;
        if nat_eq(a1608, a1609) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a1610, a1611)
}

pub fn f73(v0: u64) -> u64 {
    let a1612 = v0;
    let a1613 = 43u64;
    if nat_eq(a1612, a1613) {
        1u64
    } else {
        f30(v0)
    }
}

pub fn f74(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1614 = v1;
    let a1615 = 1u64;
    if if if nat_le(a1614, a1615) {
        true
    } else {
        let a1616 = v0;
        let a1617 = v2;
        nat_le(a1616, a1617)
    } {
        let a1618 = v3;
        let a1619 = 1u64;
        if nat_le(a1618, a1619) {
            true
        } else {
            let a1626 = {
                let a1624 = v0;
                let a1625 = {
                    let a1622 = {
                        let a1620 = v1;
                        let a1621 = 1u64;
                        nat_sub(a1620, a1621)
                    };
                    let a1623 = v2;
                    nat_mul(a1622, a1623)?
                };
                nat_add(a1624, a1625)?
            };
            let a1627 = v4;
            nat_le(a1626, a1627)
        }
    } else {
        false
    } {
        let a1628 = v5;
        let a1629 = 1u64;
        if nat_le(a1628, a1629) {
            Ok(true)
        } else {
            let a1642 = {
                let a1640 = {
                    let a1634 = v0;
                    let a1635 = {
                        let a1632 = {
                            let a1630 = v1;
                            let a1631 = 1u64;
                            nat_sub(a1630, a1631)
                        };
                        let a1633 = v2;
                        nat_mul(a1632, a1633)?
                    };
                    nat_add(a1634, a1635)?
                };
                let a1641 = {
                    let a1638 = {
                        let a1636 = v3;
                        let a1637 = 1u64;
                        nat_sub(a1636, a1637)
                    };
                    let a1639 = v4;
                    nat_mul(a1638, a1639)?
                };
                nat_add(a1640, a1641)?
            };
            let a1643 = v6;
            Ok(nat_le(a1642, a1643))
        }
    } else {
        Ok(false)
    }
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1644 = v0;
    let a1645 = 0u64;
    if if if nat_eq(a1644, a1645) {
        true
    } else {
        let a1646 = v1;
        let a1647 = 0u64;
        nat_eq(a1646, a1647)
    } {
        true
    } else {
        let a1648 = v2;
        let a1649 = 0u64;
        nat_eq(a1648, a1649)
    } {
        true
    } else {
        let a1650 = v3;
        let a1651 = 0u64;
        nat_eq(a1650, a1651)
    }
}

pub fn f76(v0: u64, v1: u64, v2: u64) -> bool {
    let a1652 = v0;
    let a1653 = 1u64;
    if nat_le(a1652, a1653) {
        true
    } else {
        let a1657 = {
            let a1654 = v1;
            let a1655 = v2;
            let a1656 = 1u64;
            nat_rem(a1654, a1655, a1656)
        };
        let a1658 = 0u64;
        nat_eq(a1657, a1658)
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f76(v0, v3, v6) {
        f76(v1, v4, v6)
    } else {
        false
    } {
        f76(v2, v5, v6)
    } else {
        false
    }
}

pub fn f78(v0: u64, v1: u64) -> u64 {
    let a1659 = v1;
    let a1660 = 28u64;
    if if nat_eq(a1659, a1660) {
        let a1661 = v0;
        let a1662 = 1u64;
        nat_eq(a1661, a1662)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f79(v0: u64, v1: u64) -> u64 {
    let a1663 = v0;
    let a1664 = 1u64;
    if if nat_eq(a1663, a1664) {
        let a1665 = v1;
        let a1666 = 43u64;
        nat_eq(a1665, a1666)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f80(v0: Adt1, v1: Adt1) -> u64 {
    let v2: u64 = match v0.clone() {
        Adt1::C0(h1667, _, _, _, _, _, _, _, _, _, _) => {
            h1667
        }
    };
    let v3: u64 = match v1.clone() {
        Adt1::C0(h1668, _, _, _, _, _, _, _, _, _, _) => {
            h1668
        }
    };
    if if if f5(v0.clone(), v1.clone()) {
        if f27(v2) {
            true
        } else {
            f27(v3)
        }
    } else {
        false
    } {
        if if if f27(v2) {
            true
        } else {
            f31(v2)
        } {
            if f27(v3) {
                true
            } else {
                f31(v3)
            }
        } else {
            false
        } {
            let a1673 = {
                let a1670 = match v0.clone() {
                    Adt1::C0(_, h1669, _, _, _, _, _, _, _, _, _) => {
                        h1669
                    }
                };
                let a1671 = f35(v2);
                let a1672 = 1u64;
                nat_rem(a1670, a1671, a1672)
            };
            let a1674 = 0u64;
            if nat_eq(a1673, a1674) {
                let a1679 = {
                    let a1676 = match v1.clone() {
                        Adt1::C0(_, h1675, _, _, _, _, _, _, _, _, _) => {
                            h1675
                        }
                    };
                    let a1677 = f35(v3);
                    let a1678 = 1u64;
                    nat_rem(a1676, a1677, a1678)
                };
                let a1680 = 0u64;
                nat_eq(a1679, a1680)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        let a1682 = match v0.clone() {
            Adt1::C0(_, h1681, _, _, _, _, _, _, _, _, _) => {
                h1681
            }
        };
        let a1683 = 0u64;
        if nat_eq(a1682, a1683) {
            f35(v3)
        } else {
            match v0 {
                Adt1::C0(_, h1684, _, _, _, _, _, _, _, _, _) => {
                    h1684
                }
            }
        }
    } else {
        if if if f27(v3) {
            let a1685 = 0u64;
            let a1686 = f30(v2);
            nat_lt(a1685, a1686)
        } else {
            false
        } {
            let a1691 = {
                let a1688 = match v0 {
                    Adt1::C0(_, h1687, _, _, _, _, _, _, _, _, _) => {
                        h1687
                    }
                };
                let a1689 = f35(v3);
                let a1690 = 1u64;
                nat_rem(a1688, a1689, a1690)
            };
            let a1692 = 0u64;
            if nat_eq(a1691, a1692) {
                let a1697 = {
                    let a1694 = match v1 {
                        Adt1::C0(_, h1693, _, _, _, _, _, _, _, _, _) => {
                            h1693
                        }
                    };
                    let a1695 = f35(v3);
                    let a1696 = 1u64;
                    nat_rem(a1694, a1695, a1696)
                };
                let a1698 = 0u64;
                nat_eq(a1697, a1698)
            } else {
                false
            }
        } else {
            false
        } {
            f35(v3)
        } else {
            0u64
        }
    }
}

pub fn f81(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1701 = match v0.clone() {
            Adt1::C0(_, _, h1699, _, _, _, _, _, _, _, _) => {
                h1699
            }
        };
        let a1702 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1700, _, _, _, _) => {
                h1700
            }
        };
        nat_mul(a1701, a1702)?
    };
    if if f50(v0.clone()) {
        let a1704 = match v0.clone() {
            Adt1::C0(_, _, _, h1703, _, _, _, _, _, _, _) => {
                h1703
            }
        };
        let a1705 = 1u64;
        if nat_eq(a1704, a1705) {
            true
        } else {
            let a1707 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1706, _, _, _) => {
                    h1706
                }
            };
            let a1708 = v1;
            nat_eq(a1707, a1708)
        }
    } else {
        false
    } {
        let a1710 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1709, _, _, _, _, _, _) => {
                h1709
            }
        };
        let a1711 = 1u64;
        if nat_eq(a1710, a1711) {
            Ok(true)
        } else {
            let a1716 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1712, _, _) => {
                    h1712
                }
            };
            let a1717 = {
                let a1714 = v1;
                let a1715 = match v0 {
                    Adt1::C0(_, _, _, h1713, _, _, _, _, _, _, _) => {
                        h1713
                    }
                };
                nat_mul(a1714, a1715)?
            };
            Ok(nat_eq(a1716, a1717))
        }
    } else {
        Ok(false)
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1730 = {
        let a1726 = {
            let a1722 = {
                let a1718 = v0;
                let a1719 = v4;
                nat_mul(a1718, a1719)?
            };
            let a1723 = {
                let a1720 = v1;
                let a1721 = v5;
                nat_mul(a1720, a1721)?
            };
            nat_add(a1722, a1723)?
        };
        let a1727 = {
            let a1724 = v2;
            let a1725 = v6;
            nat_mul(a1724, a1725)?
        };
        nat_add(a1726, a1727)?
    };
    let a1731 = {
        let a1728 = v3;
        let a1729 = v7;
        nat_mul(a1728, a1729)?
    };
    nat_add(a1730, a1731)
}

pub fn f83(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1732 = f79(v0, v1);
    let a1733 = 2u64;
    if nat_eq(a1732, a1733) {
        let a1737 = {
            let a1734 = v2;
            let a1735 = 2u64;
            let a1736 = 0u64;
            nat_rem(a1734, a1735, a1736)
        };
        let a1738 = 0u64;
        if nat_eq(a1737, a1738) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f30(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f7(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f30(v0), f7(v1))
    }
}

pub fn f84(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1739 = v0;
    let a1740 = 2u64;
    if if if nat_eq(a1739, a1740) {
        true
    } else {
        let a1741 = v0;
        let a1742 = 5u64;
        nat_eq(a1741, a1742)
    } {
        let a1744 = match v1.clone() {
            Adt1::C0(h1743, _, _, _, _, _, _, _, _, _, _) => {
                h1743
            }
        };
        let a1745 = 0u64;
        if nat_eq(a1744, a1745) {
            let a1757 = {
                let a1747 = v2;
                let a1748 = match v1.clone() {
                    Adt1::C0(_, h1746, _, _, _, _, _, _, _, _, _) => {
                        h1746
                    }
                };
                let a1749 = 0u64;
                nat_rem(a1747, a1748, a1749)
            };
            let a1758 = {
                let a1755 = match v1.clone() {
                    Adt1::C0(_, h1750, _, _, _, _, _, _, _, _, _) => {
                        h1750
                    }
                };
                let a1756 = {
                    let a1752 = match v1 {
                        Adt1::C0(_, h1751, _, _, _, _, _, _, _, _, _) => {
                            h1751
                        }
                    };
                    let a1753 = 4u64;
                    let a1754 = 0u64;
                    nat_rem(a1752, a1753, a1754)
                };
                nat_sub(a1755, a1756)
            };
            nat_lt(a1757, a1758)
        } else {
            false
        }
    } else {
        false
    } {
        let a1759 = v0;
        let a1760 = 2u64;
        if nat_eq(a1759, a1760) {
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
