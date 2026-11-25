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
    if if if if if if if if if if f56(v2.clone())? {
        let a349 = match v3.clone() {
            Adt1::C0(h348, _, _, _, _, _, _, _, _, _, _) => {
                h348
            }
        };
        let a350 = 26u64;
        nat_eq(a349, a350)
    } else {
        false
    } {
        let a352 = match v2.clone() {
            Adt1::C0(_, h351, _, _, _, _, _, _, _, _, _) => {
                h351
            }
        };
        let a353 = 2147483647u64;
        nat_le(a352, a353)
    } else {
        false
    } {
        let a356 = match v3.clone() {
            Adt1::C0(_, h354, _, _, _, _, _, _, _, _, _) => {
                h354
            }
        };
        let a357 = match v2.clone() {
            Adt1::C0(_, h355, _, _, _, _, _, _, _, _, _) => {
                h355
            }
        };
        nat_le(a356, a357)
    } else {
        false
    } {
        let a360 = match v2.clone() {
            Adt1::C0(_, _, h358, _, _, _, _, _, _, _, _) => {
                h358
            }
        };
        let a361 = match v3.clone() {
            Adt1::C0(_, _, h359, _, _, _, _, _, _, _, _) => {
                h359
            }
        };
        nat_eq(a360, a361)
    } else {
        false
    } {
        let a364 = match v2.clone() {
            Adt1::C0(_, _, _, h362, _, _, _, _, _, _, _) => {
                h362
            }
        };
        let a365 = match v3.clone() {
            Adt1::C0(_, _, _, h363, _, _, _, _, _, _, _) => {
                h363
            }
        };
        nat_eq(a364, a365)
    } else {
        false
    } {
        let a368 = match v2.clone() {
            Adt1::C0(_, _, _, _, h366, _, _, _, _, _, _) => {
                h366
            }
        };
        let a369 = match v3.clone() {
            Adt1::C0(_, _, _, _, h367, _, _, _, _, _, _) => {
                h367
            }
        };
        nat_eq(a368, a369)
    } else {
        false
    } {
        let a370 = v0;
        let a371 = 67u64;
        if nat_eq(a370, a371) {
            true
        } else {
            let a372 = v1;
            let a373 = 1u64;
            if nat_le(a372, a373) {
                let a376 = match v3.clone() {
                    Adt1::C0(_, h374, _, _, _, _, _, _, _, _, _) => {
                        h374
                    }
                };
                let a377 = match v2.clone() {
                    Adt1::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                        h375
                    }
                };
                nat_eq(a376, a377)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f57(v3.clone())?
    } else {
        false
    } {
        f46(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a379 = match v3.clone() {
                Adt1::C0(_, h378, _, _, _, _, _, _, _, _, _) => {
                    h378
                }
            };
            let a380 = 0u64;
            if nat_eq(a379, a380) {
                0u64
            } else {
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
            }
        }, 3u64, None::<u64>, v0, {
            let a388 = v0;
            let a389 = 67u64;
            if nat_eq(a388, a389) {
                1u64
            } else {
                v1
            }
        }, 0u64, 0u64, v2.clone(), v2.clone(), v3, false, {
            let a391 = match v2.clone() {
                Adt1::C0(h390, _, _, _, _, _, _, _, _, _, _) => {
                    h390
                }
            };
            let a392 = 0u64;
            if nat_eq(a391, a392) {
                0u64
            } else {
                match v2 {
                    Adt1::C0(_, h393, _, _, _, _, _, _, _, _, _) => {
                        h393
                    }
                }
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f15(v0: u64) -> bool {
    let a394 = v0;
    let a395 = 6u64;
    nat_le(a394, a395)
}

pub fn f16(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a397 = 0u64;
    let a398 = f30(match v5.clone() {
        Adt1::C0(h396, _, _, _, _, _, _, _, _, _, _) => {
            h396
        }
    });
    if nat_lt(a397, a398) {
        f58(v0, v1, v2, v3, v4, v5, v6, v7)
    } else {
        let a399 = v2;
        let a400 = 1u64;
        if if if if if if if if if if if if if if if if nat_le(a399, a400) {
            let a403 = match v5.clone() {
                Adt1::C0(h401, _, _, _, _, _, _, _, _, _, _) => {
                    h401
                }
            };
            let a404 = match v7.clone() {
                Adt1::C0(h402, _, _, _, _, _, _, _, _, _, _) => {
                    h402
                }
            };
            nat_eq(a403, a404)
        } else {
            false
        } {
            let a407 = match v6.clone() {
                Adt1::C0(h405, _, _, _, _, _, _, _, _, _, _) => {
                    h405
                }
            };
            let a408 = match v5.clone() {
                Adt1::C0(h406, _, _, _, _, _, _, _, _, _, _) => {
                    h406
                }
            };
            nat_eq(a407, a408)
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
            let a411 = match v7.clone() {
                Adt1::C0(_, _, _, _, _, h409, _, _, _, _, _) => {
                    h409
                }
            };
            let a412 = f7(match v7.clone() {
                Adt1::C0(h410, _, _, _, _, _, _, _, _, _, _) => {
                    h410
                }
            });
            nat_eq(a411, a412)
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
            let a415 = match v5.clone() {
                Adt1::C0(_, _, h413, _, _, _, _, _, _, _, _) => {
                    h413
                }
            };
            let a416 = match v7.clone() {
                Adt1::C0(_, _, h414, _, _, _, _, _, _, _, _) => {
                    h414
                }
            };
            nat_eq(a415, a416)
        } else {
            false
        } {
            let a419 = match v5.clone() {
                Adt1::C0(_, _, _, h417, _, _, _, _, _, _, _) => {
                    h417
                }
            };
            let a420 = match v7.clone() {
                Adt1::C0(_, _, _, h418, _, _, _, _, _, _, _) => {
                    h418
                }
            };
            nat_eq(a419, a420)
        } else {
            false
        } {
            let a423 = match v5.clone() {
                Adt1::C0(_, _, _, _, h421, _, _, _, _, _, _) => {
                    h421
                }
            };
            let a424 = match v7.clone() {
                Adt1::C0(_, _, _, _, h422, _, _, _, _, _, _) => {
                    h422
                }
            };
            nat_eq(a423, a424)
        } else {
            false
        } {
            let a431 = match v7.clone() {
                Adt1::C0(_, h425, _, _, _, _, _, _, _, _, _) => {
                    h425
                }
            };
            let a432 = if v1 {
                match v5.clone() {
                    Adt1::C0(_, h426, _, _, _, _, _, _, _, _, _) => {
                        h426
                    }
                }
            } else {
                let a428 = match v5.clone() {
                    Adt1::C0(_, h427, _, _, _, _, _, _, _, _, _) => {
                        h427
                    }
                };
                let a429 = 2u64;
                let a430 = 0u64;
                nat_quot(a428, a429, a430)
            };
            nat_eq(a431, a432)
        } else {
            false
        } {
            let a434 = match v7.clone() {
                Adt1::C0(_, h433, _, _, _, _, _, _, _, _, _) => {
                    h433
                }
            };
            let a435 = 2147483647u64;
            nat_le(a434, a435)
        } else {
            false
        } {
            f36({
                let a442 = {
                    let a438 = match v7.clone() {
                        Adt1::C0(_, h436, _, _, _, _, _, _, _, _, _) => {
                            h436
                        }
                    };
                    let a439 = f35(match v7.clone() {
                        Adt1::C0(h437, _, _, _, _, _, _, _, _, _, _) => {
                            h437
                        }
                    });
                    let a440 = 0u64;
                    nat_quot(a438, a439, a440)
                };
                let a443 = match v7.clone() {
                    Adt1::C0(_, _, _, _, _, h441, _, _, _, _, _) => {
                        h441
                    }
                };
                nat_mul(a442, a443)?
            }, match v7.clone() {
                Adt1::C0(_, _, h444, _, _, _, _, _, _, _, _) => {
                    h444
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, h445, _, _, _, _, _, _, _) => {
                    h445
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, h446, _, _, _, _, _, _) => {
                    h446
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, h447, _, _, _, _) => {
                    h447
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h448, _, _, _) => {
                    h448
                }
            }, match v7.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h449, _, _) => {
                    h449
                }
            })?
        } else {
            false
        } {
            let v8: u64 = match v7.clone() {
                Adt1::C0(_, h450, _, _, _, _, _, _, _, _, _) => {
                    h450
                }
            };
            let v9: u64 = {
                let a451 = v8;
                let a452 = 4u64;
                nat_mul(a451, a452)?
            };
            let v10: Adt1 = Adt1::C0(0u64, v8, 1u64, 1u64, 1u64, 4u64, v9, v9, v9, v9, 0u64);
            Ok(Adt3::C10(Adt2::C0({
                let a453 = f9(v7.clone())?;
                let a454 = v8;
                let a455 = 0u64;
                nat_quot(a453, a454, a455)
            }, 3u64, f10(match v7.clone() {
                Adt1::C0(h456, _, _, _, _, _, _, _, _, _, _) => {
                    h456
                }
            }), if v1 {
                98u64
            } else {
                let a457 = 99u64;
                let a458 = v2;
                nat_add(a457, a458)?
            }, v0, v3, v4, v5, v6, v7, true, v8), f59(v0, true, 0u64, v3, v4, v10.clone(), v10.clone(), v10)?))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f17(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h459, _, _, _, _) => {
            h459
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h460, _, _) => {
            h460
        }
    };
    let v4: u64 = {
        let a461 = f9(v2.clone())?;
        let a462 = f9(v3.clone())?;
        let a463 = 0u64;
        nat_quot(a461, a462, a463)
    };
    let v5: u64 = f53(v4, v3.clone())?;
    let v6: u64 = {
        let a464 = v0;
        let a465 = v5;
        let a466 = 0u64;
        nat_quot(a464, a465, a466)
    };
    let v7: u64 = {
        let a467 = v0;
        let a468 = v5;
        let a469 = 0u64;
        nat_rem(a467, a468, a469)
    };
    let a471 = v0;
    let a472 = match v1 {
        Adt2::C0(h470, _, _, _, _, _, _, _, _, _, _, _) => {
            h470
        }
    };
    if nat_lt(a471, a472) {
        let a473 = v7;
        let a474 = 0u64;
        if nat_eq(a473, a474) {
            Ok(Adt3::C11(None::<u64>))
        } else {
            let a475 = v7;
            let a476 = v4;
            if nat_le(a475, a476) {
                Ok(Adt3::C11(Some(f60(v6, {
                    let a477 = v7;
                    let a478 = 1u64;
                    nat_sub(a477, a478)
                }, v2, v3)?)))
            } else {
                let v8: u64 = f55(v3.clone());
                let v9: u64 = {
                    let a483 = {
                        let a481 = v7;
                        let a482 = {
                            let a479 = 1u64;
                            let a480 = v4;
                            nat_add(a479, a480)?
                        };
                        nat_sub(a481, a482)
                    };
                    let a484 = v8;
                    nat_mul(a483, a484)?
                };
                Ok(Adt3::C9(v9, f61({
                    let a488 = {
                        let a486 = v6;
                        let a487 = match v3.clone() {
                            Adt1::C0(_, h485, _, _, _, _, _, _, _, _, _) => {
                                h485
                            }
                        };
                        nat_mul(a486, a487)?
                    };
                    let a489 = v9;
                    nat_add(a488, a489)?
                }, v3)?, v8))
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f18(v0: u64, v1: Adt2) -> R<Adt3> {
    let a491 = v0;
    let a492 = match v1.clone() {
        Adt2::C0(h490, _, _, _, _, _, _, _, _, _, _, _) => {
            h490
        }
    };
    if nat_lt(a491, a492) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h493, _, _, _, _) => {
                h493
            }
        };
        let v3: u64 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h494) => {
                h494
            }
        };
        let v4: u64 = {
            let a496 = v0;
            let a497 = match v2.clone() {
                Adt1::C0(_, h495, _, _, _, _, _, _, _, _, _) => {
                    h495
                }
            };
            nat_mul(a496, a497)?
        };
        let v5: u64 = match v1.clone() {
            Adt2::C0(_, _, _, h498, _, _, _, _, _, _, _, _) => {
                h498
            }
        };
        Ok(Adt3::C7(f61({
            let a501 = v4;
            let a502 = {
                let a499 = v5;
                let a500 = 100u64;
                if nat_eq(a499, a500) {
                    v3
                } else {
                    0u64
                }
            };
            nat_add(a501, a502)?
        }, v2)?, f61({
            let a505 = v4;
            let a506 = {
                let a503 = v5;
                let a504 = 99u64;
                if nat_eq(a503, a504) {
                    v3
                } else {
                    0u64
                }
            };
            nat_add(a505, a506)?
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h507, _, _, _) => {
                h507
            }
        })?, f61({
            let a508 = v0;
            let a509 = v3;
            nat_mul(a508, a509)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h510, _, _) => {
                h510
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f19(v0: u64, v1: Adt2) -> R<Adt3> {
    let a512 = v0;
    let a513 = match v1.clone() {
        Adt2::C0(h511, _, _, _, _, _, _, _, _, _, _, _) => {
            h511
        }
    };
    if nat_lt(a512, a513) {
        let v2: u64 = {
            let a517 = v0;
            let a518 = {
                let m516 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h515, _, _, _, _) => {
                        h515
                    }
                };
                match m516 {
                    Adt1::C0(_, h514, _, _, _, _, _, _, _, _, _) => {
                        h514
                    }
                }
            };
            nat_mul(a517, a518)?
        };
        Ok(Adt3::C7(f62(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h519, _, _, _, _) => {
                h519
            }
        })?, f63(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h520, _, _, _, _) => {
                h520
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h521, _, _, _) => {
                h521
            }
        })?, f62(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h522, _, _) => {
                h522
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f20(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h523, _, _, _, _) => {
            h523
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h524, _, _) => {
            h524
        }
    };
    let a526 = v0;
    let a527 = match v1.clone() {
        Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _, _) => {
            h525
        }
    };
    if nat_lt(a526, a527) {
        let v4: u64 = {
            let a529 = match v1.clone() {
                Adt2::C0(_, _, _, h528, _, _, _, _, _, _, _, _) => {
                    h528
                }
            };
            let a530 = 89u64;
            if nat_eq(a529, a530) {
                match v2.clone() {
                    Adt1::C0(_, h531, _, _, _, _, _, _, _, _, _) => {
                        h531
                    }
                }
            } else {
                f54(v2.clone(), v3.clone())
            }
        };
        let a532 = 0u64;
        let a533 = v4;
        if nat_lt(a532, a533) {
            let v5: u64 = {
                let a534 = v0;
                let a535 = v4;
                nat_mul(a534, a535)?
            };
            Ok(Adt3::C6(f61(v5, v2)?, f61(v5, v3)?))
        } else {
            let v6: u64 = {
                let a538 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h536) => {
                        h536
                    }
                };
                let a539 = match v2.clone() {
                    Adt1::C0(_, h537, _, _, _, _, _, _, _, _, _) => {
                        h537
                    }
                };
                let a540 = 0u64;
                nat_quot(a538, a539, a540)
            };
            let a541 = v0;
            let a542 = v6;
            if nat_lt(a541, a542) {
                let v7: u64 = {
                    let a544 = v0;
                    let a545 = match v2.clone() {
                        Adt1::C0(_, h543, _, _, _, _, _, _, _, _, _) => {
                            h543
                        }
                    };
                    nat_mul(a544, a545)?
                };
                Ok(Adt3::C8(f61(v7, v2.clone())?, v7, match v2 {
                    Adt1::C0(_, h546, _, _, _, _, _, _, _, _, _) => {
                        h546
                    }
                }))
            } else {
                let v8: u64 = f55(v3.clone());
                let v9: u64 = {
                    let a549 = {
                        let a547 = v0;
                        let a548 = v6;
                        nat_sub(a547, a548)
                    };
                    let a550 = v8;
                    nat_mul(a549, a550)?
                };
                Ok(Adt3::C9(v9, f61(v9, v3)?, v8))
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f21(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h551, _, _, _, _) => {
            h551
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h552, _, _) => {
            h552
        }
    };
    let a554 = v0;
    let a555 = match v1.clone() {
        Adt2::C0(h553, _, _, _, _, _, _, _, _, _, _, _) => {
            h553
        }
    };
    if nat_lt(a554, a555) {
        Ok(Adt3::C5(f61({
            let a557 = v0;
            let a558 = match v2.clone() {
                Adt1::C0(_, h556, _, _, _, _, _, _, _, _, _) => {
                    h556
                }
            };
            nat_mul(a557, a558)?
        }, v2.clone())?, {
            let a560 = v0;
            let a561 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h559, _, _, _, _) => {
                    h559
                }
            };
            nat_mul(a560, a561)?
        }, match v2 {
            Adt1::C0(_, h562, _, _, _, _, _, _, _, _, _) => {
                h562
            }
        }, match v3 {
            Adt1::C0(_, h563, _, _, _, _, _, _, _, _, _) => {
                h563
            }
        }, {
            let a565 = match v1 {
                Adt2::C0(_, _, _, _, h564, _, _, _, _, _, _, _) => {
                    h564
                }
            };
            let a566 = 1u64;
            nat_eq(a565, a566)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f22(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a568 = v0;
    let a569 = match v1.clone() {
        Adt2::C0(h567, _, _, _, _, _, _, _, _, _, _, _) => {
            h567
        }
    };
    if nat_lt(a568, a569) {
        let v3: u64 = {
            let a571 = 2u64;
            let a572 = f9(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h570, _, _, _) => {
                    h570
                }
            })?;
            nat_mul(a571, a572)?
        };
        let a573 = v0;
        let a574 = v3;
        if nat_lt(a573, a574) {
            f64(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h575, _, _, _) => {
                    h575
                }
            }, {
                let m578 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h577, _, _) => {
                        h577
                    }
                };
                match m578 {
                    Adt1::C0(_, _, h576, _, _, _, _, _, _, _, _) => {
                        h576
                    }
                }
            }, v2)
        } else {
            f65({
                let a579 = v0;
                let a580 = v3;
                nat_sub(a579, a580)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h581, _, _, _, _) => {
                    h581
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h582, _, _, _) => {
                    h582
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h583, _, _) => {
                    h583
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f23(v0: u64) -> bool {
    let a584 = v0;
    let a585 = 3u64;
    if nat_eq(a584, a585) {
        true
    } else {
        let a586 = v0;
        let a587 = 6u64;
        nat_eq(a586, a587)
    }
}

pub fn f24(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h588, _, _, _, _, _) => {
            h588
        }
    };
    let a590 = v0;
    let a591 = match v1.clone() {
        Adt2::C0(h589, _, _, _, _, _, _, _, _, _, _, _) => {
            h589
        }
    };
    if nat_lt(a590, a591) {
        let a592 = v0;
        let a593 = 0u64;
        if nat_eq(a592, a593) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a594 = v0;
                let a595 = 1u64;
                nat_sub(a594, a595)
            };
            let v5: u64 = {
                let a597 = v4;
                let a598 = f66(match v1.clone() {
                    Adt2::C0(_, _, _, _, h596, _, _, _, _, _, _, _) => {
                        h596
                    }
                });
                let a599 = 0u64;
                nat_rem(a597, a598, a599)
            };
            let v6: u64 = {
                let a604 = 4u64;
                let a605 = {
                    let a601 = v4;
                    let a602 = f66(match v1.clone() {
                        Adt2::C0(_, _, _, _, h600, _, _, _, _, _, _, _) => {
                            h600
                        }
                    });
                    let a603 = 0u64;
                    nat_quot(a601, a602, a603)
                };
                nat_mul(a604, a605)?
            };
            let a606 = v5;
            let a607 = 2u64;
            if nat_lt(a606, a607) {
                f25({
                    let a608 = v6;
                    let a609 = v5;
                    nat_add(a608, a609)?
                }, v1)
            } else {
                let a610 = v5;
                let a611 = 2u64;
                if nat_eq(a610, a611) {
                    Ok(f67(0u64, 2u64, v3))
                } else {
                    let a612 = v5;
                    let a613 = 3u64;
                    if nat_eq(a612, a613) {
                        Ok(f68(0u64, 2u64, v3, {
                            let a614 = v2;
                            let a615 = 2u64;
                            nat_eq(a614, a615)
                        }))
                    } else {
                        let a616 = v5;
                        let a617 = 4u64;
                        if nat_eq(a616, a617) {
                            Ok(f67(1u64, 1u64, 3u64))
                        } else {
                            let a618 = v5;
                            let a619 = 5u64;
                            if nat_eq(a618, a619) {
                                Ok(f68(1u64, 1u64, 3u64, {
                                    let a620 = v2;
                                    let a621 = 0u64;
                                    nat_eq(a620, a621)
                                }))
                            } else {
                                let a622 = v5;
                                let a623 = 6u64;
                                if nat_eq(a622, a623) {
                                    Ok(f67(1u64, 2u64, v3))
                                } else {
                                    let a624 = v5;
                                    let a625 = 7u64;
                                    if nat_eq(a624, a625) {
                                        Ok(f68(1u64, 2u64, v3, {
                                            let a626 = v2;
                                            let a627 = 2u64;
                                            nat_eq(a626, a627)
                                        }))
                                    } else {
                                        let a629 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h628, _, _, _, _, _, _, _) => {
                                                h628
                                            }
                                        };
                                        let a630 = 3u64;
                                        if nat_eq(a629, a630) {
                                            let a631 = v5;
                                            let a632 = 15u64;
                                            if nat_eq(a631, a632) {
                                                f25({
                                                    let a633 = v6;
                                                    let a634 = 3u64;
                                                    nat_add(a633, a634)?
                                                }, v1)
                                            } else {
                                                Ok(f69(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h635, _, _, _, _, _, _) => {
                                                        h635
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a636 = v5;
                                            let a637 = 8u64;
                                            if nat_eq(a636, a637) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m640 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h639, _, _) => {
                                                            h639
                                                        }
                                                    };
                                                    match m640 {
                                                        Adt1::C0(h638, _, _, _, _, _, _, _, _, _, _) => {
                                                            h638
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f25({
                                                    let a641 = v6;
                                                    let a642 = 3u64;
                                                    nat_add(a641, a642)?
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
    let a644 = v0;
    let a645 = match v1.clone() {
        Adt2::C0(h643, _, _, _, _, _, _, _, _, _, _, _) => {
            h643
        }
    };
    if nat_lt(a644, a645) {
        f70({
            let a646 = v0;
            let a647 = 4u64;
            let a648 = 0u64;
            nat_rem(a646, a647, a648)
        }, {
            let a649 = v0;
            let a650 = 4u64;
            let a651 = 0u64;
            nat_quot(a649, a650, a651)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f26(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h652, _, _, _, _) => {
            h652
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h653, _, _, _) => {
            h653
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h654, _, _) => {
            h654
        }
    };
    let v6: u64 = {
        let a656 = match v1.clone() {
            Adt2::C0(_, h655, _, _, _, _, _, _, _, _, _, _) => {
                h655
            }
        };
        let a657 = 3u64;
        if nat_eq(a656, a657) {
            0u64
        } else {
            let a658 = f9(v3.clone())?;
            let a659 = 2u64;
            nat_mul(a658, a659)?
        }
    };
    let a661 = match v1.clone() {
        Adt2::C0(h660, _, _, _, _, _, _, _, _, _, _, _) => {
            h660
        }
    };
    let a662 = v0;
    if nat_le(a661, a662) {
        Ok(Adt3::C1)
    } else {
        let a663 = v0;
        let a664 = v6;
        if nat_lt(a663, a664) {
            let v7: u64 = {
                let a665 = v0;
                let a666 = 2u64;
                let a667 = 0u64;
                nat_quot(a665, a666, a667)
            };
            let v8: u64 = {
                let a672 = 3u64;
                let a673 = {
                    let a669 = f62(v7, v5.clone())?;
                    let a670 = f39(match v5 {
                        Adt1::C0(h668, _, _, _, _, _, _, _, _, _, _) => {
                            h668
                        }
                    });
                    let a671 = 0u64;
                    nat_quot(a669, a670, a671)
                };
                nat_add(a672, a673)?
            };
            let a677 = {
                let a674 = v0;
                let a675 = 2u64;
                let a676 = 0u64;
                nat_rem(a674, a675, a676)
            };
            let a678 = 0u64;
            if nat_eq(a677, a678) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a679 = v2;
                let a680 = 0u64;
                if nat_eq(a679, a680) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a681 = v0;
                let a682 = v6;
                nat_sub(a681, a682)
            };
            if f71(match v3.clone() {
                Adt1::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                    h683
                }
            }) {
                f72(v9, v1)
            } else {
                let v10: u64 = f73(match v3.clone() {
                    Adt1::C0(h684, _, _, _, _, _, _, _, _, _, _) => {
                        h684
                    }
                }, match v4.clone() {
                    Adt1::C0(h685, _, _, _, _, _, _, _, _, _, _) => {
                        h685
                    }
                }, match v5.clone() {
                    Adt1::C0(h686, _, _, _, _, _, _, _, _, _, _) => {
                        h686
                    }
                })?;
                let v11: u64 = {
                    let a687 = v9;
                    let a688 = v10;
                    let a689 = 0u64;
                    nat_quot(a687, a688, a689)
                };
                let v12: u64 = {
                    let a690 = v9;
                    let a691 = v10;
                    let a692 = 0u64;
                    nat_rem(a690, a691, a692)
                };
                let v13: u64 = {
                    let a694 = match v3.clone() {
                        Adt1::C0(h693, _, _, _, _, _, _, _, _, _, _) => {
                            h693
                        }
                    };
                    let a695 = 0u64;
                    if nat_eq(a694, a695) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let v14: u64 = {
                    let a697 = match v4.clone() {
                        Adt1::C0(h696, _, _, _, _, _, _, _, _, _, _) => {
                            h696
                        }
                    };
                    let a698 = 0u64;
                    if nat_eq(a697, a698) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let a699 = v12;
                let a700 = v13;
                if nat_lt(a699, a700) {
                    Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                        Adt1::C0(h701, _, _, _, _, _, _, _, _, _, _) => {
                            h701
                        }
                    }, 0u64, 0u64, f62(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f39(match v3 {
                        Adt1::C0(h702, _, _, _, _, _, _, _, _, _, _) => {
                            h702
                        }
                    }), 4u64))
                } else {
                    let a705 = v12;
                    let a706 = {
                        let a703 = v13;
                        let a704 = v14;
                        nat_add(a703, a704)?
                    };
                    if nat_lt(a705, a706) {
                        Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                            Adt1::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
                                h707
                            }
                        }, 0u64, 4u64, f63(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f39(match v4 {
                            Adt1::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                                h708
                            }
                        }), 4u64))
                    } else {
                        let a711 = v12;
                        let a712 = {
                            let a709 = v13;
                            let a710 = v14;
                            nat_add(a709, a710)?
                        };
                        if nat_eq(a711, a712) {
                            Ok(Adt3::C3(1u64, match v1 {
                                Adt2::C0(_, _, _, h713, _, _, _, _, _, _, _, _) => {
                                    h713
                                }
                            }, 0u64, 0u64, {
                                let a714 = v13;
                                let a715 = 0u64;
                                if nat_eq(a714, a715) {
                                    0u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a716 = v13;
                                let a717 = 0u64;
                                if nat_eq(a716, a717) {
                                    f62(v11, v3.clone())?
                                } else {
                                    0u64
                                }
                            }, {
                                let a718 = v14;
                                let a719 = 0u64;
                                if nat_eq(a718, a719) {
                                    4u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a720 = v14;
                                let a721 = 0u64;
                                if nat_eq(a720, a721) {
                                    f63(v11, v3, v4)?
                                } else {
                                    1u64
                                }
                            }, {
                                let a723 = match v5.clone() {
                                    Adt1::C0(h722, _, _, _, _, _, _, _, _, _, _) => {
                                        h722
                                    }
                                };
                                let a724 = 0u64;
                                if nat_eq(a723, a724) {
                                    3u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a726 = match v5.clone() {
                                    Adt1::C0(h725, _, _, _, _, _, _, _, _, _, _) => {
                                        h725
                                    }
                                };
                                let a727 = 0u64;
                                if nat_eq(a726, a727) {
                                    f62(v11, v5)?
                                } else {
                                    0u64
                                }
                            }, 4u64, 4u64))
                        } else {
                            Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                                Adt1::C0(h728, _, _, _, _, _, _, _, _, _, _) => {
                                    h728
                                }
                            }, 1u64, 0u64, 2u64, 0u64, 3u64, f62(v11, v5.clone())?, 4u64, f39(match v5 {
                                Adt1::C0(h729, _, _, _, _, _, _, _, _, _, _) => {
                                    h729
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
    let a730 = 6u64;
    let a731 = v0;
    if if if if nat_le(a730, a731) {
        let a732 = v0;
        let a733 = 23u64;
        nat_le(a732, a733)
    } else {
        false
    } {
        true
    } else {
        let a734 = v0;
        let a735 = 2u64;
        if nat_eq(a734, a735) {
            true
        } else {
            let a736 = v0;
            let a737 = 3u64;
            nat_eq(a736, a737)
        }
    } {
        true
    } else {
        let a738 = v0;
        let a739 = 29u64;
        nat_eq(a738, a739)
    } {
        true
    } else {
        let a740 = 34u64;
        let a741 = v0;
        if if nat_le(a740, a741) {
            let a742 = v0;
            let a743 = 35u64;
            nat_le(a742, a743)
        } else {
            false
        } {
            true
        } else {
            let a744 = 39u64;
            let a745 = v0;
            if nat_le(a744, a745) {
                let a746 = v0;
                let a747 = 42u64;
                nat_le(a746, a747)
            } else {
                false
            }
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> bool {
    let a748 = v0;
    let a749 = 0u64;
    if if if nat_eq(a748, a749) {
        let a750 = v1;
        let a751 = 0u64;
        nat_eq(a750, a751)
    } else {
        false
    } {
        let a752 = v2;
        let a753 = 0u64;
        nat_eq(a752, a753)
    } else {
        false
    } {
        true
    } else {
        let a754 = v0;
        let a755 = 1u64;
        if if nat_eq(a754, a755) {
            true
        } else {
            let a756 = v0;
            let a757 = 30u64;
            nat_eq(a756, a757)
        } {
            let a758 = v1;
            let a759 = v0;
            if if nat_eq(a758, a759) {
                let a760 = v2;
                let a761 = v0;
                nat_eq(a760, a761)
            } else {
                false
            } {
                true
            } else {
                let a762 = v1;
                let a763 = 0u64;
                if nat_eq(a762, a763) {
                    let a764 = v2;
                    let a765 = v0;
                    if nat_eq(a764, a765) {
                        true
                    } else {
                        let a766 = v2;
                        let a767 = 0u64;
                        nat_eq(a766, a767)
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
    let a768 = v1;
    let a769 = 27u64;
    if if if nat_eq(a768, a769) {
        true
    } else {
        let a770 = v1;
        let a771 = 28u64;
        nat_eq(a770, a771)
    } {
        let a772 = v1;
        let a773 = v2;
        nat_eq(a772, a773)
    } else {
        false
    } {
        let a774 = v0;
        let a775 = 3u64;
        nat_eq(a774, a775)
    } else {
        f31(v1)
    }
}

pub fn f30(v0: u64) -> u64 {
    let a776 = v0;
    let a777 = 0u64;
    if nat_eq(a776, a777) {
        4u64
    } else {
        let a778 = v0;
        let a779 = 1u64;
        if if nat_eq(a778, a779) {
            true
        } else {
            let a780 = v0;
            let a781 = 30u64;
            nat_eq(a780, a781)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f31(v0: u64) -> bool {
    let a782 = 0u64;
    let a783 = f74(v0);
    if nat_lt(a782, a783) {
        true
    } else {
        let a784 = 24u64;
        let a785 = v0;
        if nat_le(a784, a785) {
            let a786 = v0;
            let a787 = 28u64;
            nat_le(a786, a787)
        } else {
            false
        }
    }
}

pub fn f32(v0: Adt1, v1: Adt1) -> bool {
    if f37(v1.clone()) {
        f37(v0)
    } else {
        let a793 = {
            let a790 = match v0.clone() {
                Adt1::C0(_, h788, _, _, _, _, _, _, _, _, _) => {
                    h788
                }
            };
            let a791 = match v1.clone() {
                Adt1::C0(_, h789, _, _, _, _, _, _, _, _, _) => {
                    h789
                }
            };
            let a792 = 1u64;
            nat_rem(a790, a791, a792)
        };
        let a794 = 0u64;
        if if if nat_eq(a793, a794) {
            let a800 = {
                let a797 = match v0.clone() {
                    Adt1::C0(_, _, h795, _, _, _, _, _, _, _, _) => {
                        h795
                    }
                };
                let a798 = match v1.clone() {
                    Adt1::C0(_, _, h796, _, _, _, _, _, _, _, _) => {
                        h796
                    }
                };
                let a799 = 1u64;
                nat_rem(a797, a798, a799)
            };
            let a801 = 0u64;
            nat_eq(a800, a801)
        } else {
            false
        } {
            let a807 = {
                let a804 = match v0.clone() {
                    Adt1::C0(_, _, _, h802, _, _, _, _, _, _, _) => {
                        h802
                    }
                };
                let a805 = match v1.clone() {
                    Adt1::C0(_, _, _, h803, _, _, _, _, _, _, _) => {
                        h803
                    }
                };
                let a806 = 1u64;
                nat_rem(a804, a805, a806)
            };
            let a808 = 0u64;
            nat_eq(a807, a808)
        } else {
            false
        } {
            let a814 = {
                let a811 = match v0 {
                    Adt1::C0(_, _, _, _, h809, _, _, _, _, _, _) => {
                        h809
                    }
                };
                let a812 = match v1 {
                    Adt1::C0(_, _, _, _, h810, _, _, _, _, _, _) => {
                        h810
                    }
                };
                let a813 = 1u64;
                nat_rem(a811, a812, a813)
            };
            let a815 = 0u64;
            nat_eq(a814, a815)
        } else {
            false
        }
    }
}

pub fn f33(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h816, _, _, _, _, _, _, _, _, _, _) => {
            h816
        }
    };
    let a817 = v1;
    let a818 = v3;
    if if nat_eq(a817, a818) {
        true
    } else {
        let a819 = v0;
        let a820 = 0u64;
        if nat_eq(a819, a820) {
            let a821 = v4;
            let a822 = 0u64;
            if if nat_eq(a821, a822) {
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
            let a824 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h823, _, _, _, _, _) => {
                    h823
                }
            };
            let a825 = f7(v4);
            nat_eq(a824, a825)
        } else {
            false
        } {
            let a830 = {
                let a827 = match v2 {
                    Adt1::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                        h826
                    }
                };
                let a828 = f35(v4);
                let a829 = 1u64;
                nat_rem(a827, a828, a829)
            };
            let a831 = 0u64;
            nat_eq(a830, a831)
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
        let a832 = v0;
        let a833 = v1;
        if if nat_eq(a832, a833) {
            f31(v0)
        } else {
            false
        } {
            true
        } else {
            let a834 = f30(v0);
            let a835 = 2u64;
            if nat_eq(a834, a835) {
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
        let m836 = f45(v0);
        match m836 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h837, _) => {
                        h837
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f75(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f75(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f75(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f75(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f75(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f75(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f37(v0: Adt1) -> bool {
    f76(match v0.clone() {
        Adt1::C0(_, h838, _, _, _, _, _, _, _, _, _) => {
            h838
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h839, _, _, _, _, _, _, _, _) => {
            h839
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h840, _, _, _, _, _, _, _) => {
            h840
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h841, _, _, _, _, _, _) => {
            h841
        }
    })
}

pub fn f38(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a843 = match v1.clone() {
        Adt1::C0(h842, _, _, _, _, _, _, _, _, _, _) => {
            h842
        }
    };
    let a844 = 43u64;
    if nat_eq(a843, a844) {
        let a845 = v0;
        let a846 = 0u64;
        if if if if if if nat_eq(a845, a846) {
            let a848 = match v2.clone() {
                Adt1::C0(h847, _, _, _, _, _, _, _, _, _, _) => {
                    h847
                }
            };
            let a849 = 0u64;
            nat_eq(a848, a849)
        } else {
            false
        } {
            let a851 = match v3.clone() {
                Adt1::C0(h850, _, _, _, _, _, _, _, _, _, _) => {
                    h850
                }
            };
            let a852 = 0u64;
            if nat_eq(a851, a852) {
                true
            } else {
                let a854 = match v3.clone() {
                    Adt1::C0(h853, _, _, _, _, _, _, _, _, _, _) => {
                        h853
                    }
                };
                let a855 = 43u64;
                nat_eq(a854, a855)
            }
        } else {
            false
        } {
            let a857 = match v3.clone() {
                Adt1::C0(h856, _, _, _, _, _, _, _, _, _, _) => {
                    h856
                }
            };
            let a858 = 43u64;
            if nat_eq(a857, a858) {
                f32(v1.clone(), v2.clone())
            } else {
                f5(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a860 = match v2 {
                Adt1::C0(_, _, _, _, _, h859, _, _, _, _, _) => {
                    h859
                }
            };
            let a861 = 4u64;
            nat_eq(a860, a861)
        } else {
            false
        } {
            let a864 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h862, _, _, _, _, _) => {
                    h862
                }
            };
            let a865 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h863, _, _, _, _) => {
                    h863
                }
            };
            if nat_le(a864, a865) {
                let a868 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h866, _, _, _, _) => {
                        h866
                    }
                };
                let a869 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h867, _, _, _) => {
                        h867
                    }
                };
                if nat_le(a868, a869) {
                    let a872 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h870, _, _, _) => {
                            h870
                        }
                    };
                    let a873 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h871, _, _) => {
                            h871
                        }
                    };
                    nat_le(a872, a873)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a881 = {
                let a879 = match v1.clone() {
                    Adt1::C0(_, _, h874, _, _, _, _, _, _, _, _) => {
                        h874
                    }
                };
                let a880 = {
                    let a877 = match v1.clone() {
                        Adt1::C0(_, _, _, h875, _, _, _, _, _, _, _) => {
                            h875
                        }
                    };
                    let a878 = match v1 {
                        Adt1::C0(_, _, _, _, h876, _, _, _, _, _, _) => {
                            h876
                        }
                    };
                    nat_mul(a877, a878)?
                };
                nat_mul(a879, a880)?
            };
            let a882 = 2147483647u64;
            Ok(nat_le(a881, a882))
        } else {
            Ok(false)
        }
    } else {
        if f32(v1.clone(), v2.clone()) {
            if f28(match v1.clone() {
                Adt1::C0(h883, _, _, _, _, _, _, _, _, _, _) => {
                    h883
                }
            }, match v2.clone() {
                Adt1::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
                    h884
                }
            }, match v3.clone() {
                Adt1::C0(h885, _, _, _, _, _, _, _, _, _, _) => {
                    h885
                }
            }) {
                Ok(true)
            } else {
                let a886 = v0;
                let a887 = 3u64;
                if if if if nat_lt(a886, a887) {
                    f71(match v1.clone() {
                        Adt1::C0(h888, _, _, _, _, _, _, _, _, _, _) => {
                            h888
                        }
                    })
                } else {
                    false
                } {
                    let a891 = match v1.clone() {
                        Adt1::C0(h889, _, _, _, _, _, _, _, _, _, _) => {
                            h889
                        }
                    };
                    let a892 = match v2.clone() {
                        Adt1::C0(h890, _, _, _, _, _, _, _, _, _, _) => {
                            h890
                        }
                    };
                    nat_eq(a891, a892)
                } else {
                    false
                } {
                    let a895 = match v1 {
                        Adt1::C0(h893, _, _, _, _, _, _, _, _, _, _) => {
                            h893
                        }
                    };
                    let a896 = match v3 {
                        Adt1::C0(h894, _, _, _, _, _, _, _, _, _, _) => {
                            h894
                        }
                    };
                    nat_eq(a895, a896)
                } else {
                    false
                } {
                    let a898 = match v2 {
                        Adt1::C0(_, _, _, _, _, h897, _, _, _, _, _) => {
                            h897
                        }
                    };
                    let a899 = 8u64;
                    Ok(nat_eq(a898, a899))
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
    if f71(v0) {
        8u64
    } else {
        f74(v0)
    }
}

pub fn f40(v0: Adt1) -> bool {
    let v1: u64 = f39(match v0.clone() {
        Adt1::C0(h900, _, _, _, _, _, _, _, _, _, _) => {
            h900
        }
    });
    let a905 = {
        let a902 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h901) => {
                h901
            }
        };
        let a903 = v1;
        let a904 = 1u64;
        nat_rem(a902, a903, a904)
    };
    let a906 = 0u64;
    if if nat_eq(a905, a906) {
        f77(match v0.clone() {
            Adt1::C0(_, h907, _, _, _, _, _, _, _, _, _) => {
                h907
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h908, _, _, _, _, _) => {
                h908
            }
        }, v1)
    } else {
        false
    } {
        f78(match v0.clone() {
            Adt1::C0(_, _, h909, _, _, _, _, _, _, _, _) => {
                h909
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h910, _, _, _, _, _, _, _) => {
                h910
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h911, _, _, _, _, _, _) => {
                h911
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h912, _, _, _, _) => {
                h912
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h913, _, _, _) => {
                h913
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h914, _, _) => {
                h914
            }
        }, v1)
    } else {
        false
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> R<bool> {
    let a922 = {
        let a920 = match v0.clone() {
            Adt1::C0(_, _, h915, _, _, _, _, _, _, _, _) => {
                h915
            }
        };
        let a921 = {
            let a918 = match v0.clone() {
                Adt1::C0(_, _, _, h916, _, _, _, _, _, _, _) => {
                    h916
                }
            };
            let a919 = match v0.clone() {
                Adt1::C0(_, _, _, _, h917, _, _, _, _, _, _) => {
                    h917
                }
            };
            nat_mul(a918, a919)?
        };
        nat_mul(a920, a921)?
    };
    let a923 = 9223372036854775807u64;
    if nat_le(a922, a923) {
        if f50(v1.clone()) {
            let a925 = match v1 {
                Adt1::C0(_, h924, _, _, _, _, _, _, _, _, _) => {
                    h924
                }
            };
            let a926 = 2147483647u64;
            Ok(nat_le(a925, a926))
        } else {
            let a928 = match v0 {
                Adt1::C0(_, h927, _, _, _, _, _, _, _, _, _) => {
                    h927
                }
            };
            let a929 = 2147483647u64;
            Ok(nat_le(a928, a929))
        }
    } else {
        Ok(false)
    }
}

pub fn f42(v0: Adt1) -> R<bool> {
    f36({
        let a932 = match v0.clone() {
            Adt1::C0(_, h930, _, _, _, _, _, _, _, _, _) => {
                h930
            }
        };
        let a933 = f39(match v0.clone() {
            Adt1::C0(h931, _, _, _, _, _, _, _, _, _, _) => {
                h931
            }
        });
        nat_mul(a932, a933)?
    }, match v0.clone() {
        Adt1::C0(_, _, h934, _, _, _, _, _, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h935, _, _, _, _, _, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h936, _, _, _, _, _, _) => {
            h936
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h937, _, _, _, _) => {
            h937
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h938, _, _, _) => {
            h938
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h939, _, _) => {
            h939
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a946 = {
        let a940 = v0;
        let a941 = v2;
        nat_mul(a940, a941)?
    };
    let a947 = {
        let a942 = v1;
        let a943 = 0u64;
        if nat_eq(a942, a943) {
            0u64
        } else {
            let a944 = v0;
            let a945 = 2u64;
            nat_mul(a944, a945)?
        }
    };
    nat_add(a946, a947)
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if f71(v1) {
        Ok(f79(v0, v1))
    } else {
        f73(v1, v2, v3)
    }
}

pub fn f45(v0: u64) -> Adt4 {
    let a948 = v0;
    let a949 = 0u64;
    if nat_eq(a948, a949) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a950 = v0;
        let a951 = 1u64;
        if nat_eq(a950, a951) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a952 = v0;
            let a953 = 2u64;
            if nat_eq(a952, a953) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a954 = v0;
                let a955 = 3u64;
                if nat_eq(a954, a955) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a956 = v0;
                    let a957 = 6u64;
                    if nat_eq(a956, a957) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a958 = v0;
                        let a959 = 7u64;
                        if nat_eq(a958, a959) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a960 = v0;
                            let a961 = 8u64;
                            if nat_eq(a960, a961) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a962 = v0;
                                let a963 = 9u64;
                                if nat_eq(a962, a963) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a964 = v0;
                                    let a965 = 10u64;
                                    if nat_eq(a964, a965) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a966 = v0;
                                        let a967 = 11u64;
                                        if nat_eq(a966, a967) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a968 = v0;
                                            let a969 = 12u64;
                                            if nat_eq(a968, a969) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a970 = v0;
                                                let a971 = 13u64;
                                                if nat_eq(a970, a971) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a972 = v0;
                                                    let a973 = 14u64;
                                                    if nat_eq(a972, a973) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a974 = v0;
                                                        let a975 = 15u64;
                                                        if nat_eq(a974, a975) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a976 = v0;
                                                            let a977 = 16u64;
                                                            if nat_eq(a976, a977) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a978 = v0;
                                                                let a979 = 17u64;
                                                                if nat_eq(a978, a979) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a980 = v0;
                                                                    let a981 = 18u64;
                                                                    if nat_eq(a980, a981) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a982 = v0;
                                                                        let a983 = 19u64;
                                                                        if nat_eq(a982, a983) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a984 = v0;
                                                                            let a985 = 20u64;
                                                                            if nat_eq(a984, a985) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a986 = v0;
                                                                                let a987 = 21u64;
                                                                                if nat_eq(a986, a987) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a988 = v0;
                                                                                    let a989 = 22u64;
                                                                                    if nat_eq(a988, a989) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a990 = v0;
                                                                                        let a991 = 23u64;
                                                                                        if nat_eq(a990, a991) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a992 = v0;
                                                                                            let a993 = 24u64;
                                                                                            if nat_eq(a992, a993) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a994 = v0;
                                                                                                let a995 = 25u64;
                                                                                                if nat_eq(a994, a995) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a996 = v0;
                                                                                                    let a997 = 26u64;
                                                                                                    if nat_eq(a996, a997) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a998 = v0;
                                                                                                        let a999 = 27u64;
                                                                                                        if nat_eq(a998, a999) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1000 = v0;
                                                                                                            let a1001 = 28u64;
                                                                                                            if nat_eq(a1000, a1001) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1002 = v0;
                                                                                                                let a1003 = 29u64;
                                                                                                                if nat_eq(a1002, a1003) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1004 = v0;
                                                                                                                    let a1005 = 30u64;
                                                                                                                    if nat_eq(a1004, a1005) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1006 = v0;
                                                                                                                        let a1007 = 34u64;
                                                                                                                        if nat_eq(a1006, a1007) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1008 = v0;
                                                                                                                            let a1009 = 35u64;
                                                                                                                            if nat_eq(a1008, a1009) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1010 = v0;
                                                                                                                                let a1011 = 39u64;
                                                                                                                                if nat_eq(a1010, a1011) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1012 = v0;
                                                                                                                                    let a1013 = 40u64;
                                                                                                                                    if nat_eq(a1012, a1013) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1014 = v0;
                                                                                                                                        let a1015 = 41u64;
                                                                                                                                        if nat_eq(a1014, a1015) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1016 = v0;
                                                                                                                                            let a1017 = 42u64;
                                                                                                                                            if nat_eq(a1016, a1017) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1018 = v0;
                                                                                                                                                let a1019 = 43u64;
                                                                                                                                                if nat_eq(a1018, a1019) {
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
    let a1021 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1020, _) => {
            h1020
        }
    };
    let a1022 = 0u64;
    if nat_eq(a1021, a1022) {
        Ok(true)
    } else {
        let a1024 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h1023, _) => {
                h1023
            }
        };
        let a1025 = 0u64;
        if nat_eq(a1024, a1025) {
            Ok(true)
        } else {
            let a1027 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1026) => {
                    h1026
                }
            };
            let a1028 = 0u64;
            if nat_eq(a1027, a1028) {
                Ok(true)
            } else {
                let a1030 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1029) => {
                        h1029
                    }
                };
                let a1031 = 0u64;
                if nat_eq(a1030, a1031) {
                    Ok(true)
                } else {
                    let a1037 = {
                        let a1034 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1032) => {
                                h1032
                            }
                        };
                        let a1035 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h1033, _) => {
                                h1033
                            }
                        };
                        nat_add(a1034, a1035)?
                    };
                    let a1038 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1036) => {
                            h1036
                        }
                    };
                    if nat_le(a1037, a1038) {
                        Ok(true)
                    } else {
                        let a1044 = {
                            let a1041 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1039) => {
                                    h1039
                                }
                            };
                            let a1042 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h1040, _) => {
                                    h1040
                                }
                            };
                            nat_add(a1041, a1042)?
                        };
                        let a1045 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1043) => {
                                h1043
                            }
                        };
                        Ok(nat_le(a1044, a1045))
                    }
                }
            }
        }
    }
}

pub fn f47(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a1048 = match v0.clone() {
            Adt1::C0(h1046, _, _, _, _, _, _, _, _, _, _) => {
                h1046
            }
        };
        let a1049 = match v1.clone() {
            Adt1::C0(h1047, _, _, _, _, _, _, _, _, _, _) => {
                h1047
            }
        };
        nat_eq(a1048, a1049)
    } else {
        false
    } {
        let a1052 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1050) => {
                h1050
            }
        };
        let a1053 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1051) => {
                h1051
            }
        };
        nat_eq(a1052, a1053)
    } else {
        false
    } {
        let a1056 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1054, _, _, _, _, _) => {
                h1054
            }
        };
        let a1057 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1055, _, _, _, _, _) => {
                h1055
            }
        };
        nat_eq(a1056, a1057)
    } else {
        false
    } {
        let a1060 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1058, _, _, _, _) => {
                h1058
            }
        };
        let a1061 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1059, _, _, _, _) => {
                h1059
            }
        };
        nat_eq(a1060, a1061)
    } else {
        false
    } {
        let a1064 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1062, _, _, _) => {
                h1062
            }
        };
        let a1065 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1063, _, _, _) => {
                h1063
            }
        };
        nat_eq(a1064, a1065)
    } else {
        false
    } {
        let a1068 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1066, _, _) => {
                h1066
            }
        };
        let a1069 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1067, _, _) => {
                h1067
            }
        };
        nat_eq(a1068, a1069)
    } else {
        false
    }
}

pub fn f48(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a1072 = match v0.clone() {
            Adt1::C0(h1070, _, _, _, _, _, _, _, _, _, _) => {
                h1070
            }
        };
        let a1073 = match v1.clone() {
            Adt1::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                h1071
            }
        };
        nat_eq(a1072, a1073)
    } else {
        false
    } {
        let a1076 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1074, _, _, _, _, _) => {
                h1074
            }
        };
        let a1077 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1075, _, _, _, _, _) => {
                h1075
            }
        };
        nat_eq(a1076, a1077)
    } else {
        false
    } {
        let a1080 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1078, _, _, _, _) => {
                h1078
            }
        };
        let a1081 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1079, _, _, _, _) => {
                h1079
            }
        };
        nat_eq(a1080, a1081)
    } else {
        false
    } {
        let a1084 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1082, _, _, _) => {
                h1082
            }
        };
        let a1085 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1083, _, _, _) => {
                h1083
            }
        };
        nat_eq(a1084, a1085)
    } else {
        false
    } {
        let a1088 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1086, _, _) => {
                h1086
            }
        };
        let a1089 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1087, _, _) => {
                h1087
            }
        };
        nat_eq(a1088, a1089)
    } else {
        false
    } {
        let a1091 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1090) => {
                h1090
            }
        };
        let a1092 = 0u64;
        if if nat_eq(a1091, a1092) {
            true
        } else {
            let a1094 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1093) => {
                    h1093
                }
            };
            let a1095 = 0u64;
            nat_eq(a1094, a1095)
        } {
            true
        } else {
            let a1098 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1096) => {
                    h1096
                }
            };
            let a1099 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1097) => {
                    h1097
                }
            };
            nat_eq(a1098, a1099)
        }
    } else {
        false
    }
}

pub fn f49(v0: u64) -> u64 {
    let a1100 = v0;
    let a1101 = 26u64;
    if nat_eq(a1100, a1101) {
        4u64
    } else {
        let a1102 = v0;
        let a1103 = 27u64;
        if nat_eq(a1102, a1103) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f50(v0: Adt1) -> bool {
    let a1105 = match v0.clone() {
        Adt1::C0(_, h1104, _, _, _, _, _, _, _, _, _) => {
            h1104
        }
    };
    let a1106 = 1u64;
    if nat_eq(a1105, a1106) {
        true
    } else {
        let a1109 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1107, _, _, _, _, _) => {
                h1107
            }
        };
        let a1110 = f74(match v0 {
            Adt1::C0(h1108, _, _, _, _, _, _, _, _, _, _) => {
                h1108
            }
        });
        nat_eq(a1109, a1110)
    }
}

pub fn f51(v0: Adt1, v1: u64) -> R<u64> {
    let a1116 = {
        let a1112 = match v0.clone() {
            Adt1::C0(_, h1111, _, _, _, _, _, _, _, _, _) => {
                h1111
            }
        };
        let a1113 = f35(v1);
        let a1114 = 0u64;
        nat_quot(a1112, a1113, a1114)
    };
    let a1117 = f80(match v0 {
        Adt1::C0(h1115, _, _, _, _, _, _, _, _, _, _) => {
            h1115
        }
    }, v1);
    nat_mul(a1116, a1117)
}

pub fn f52(v0: u64) -> u64 {
    if if f27(v0) {
        true
    } else {
        let a1118 = 24u64;
        let a1119 = v0;
        if nat_le(a1118, a1119) {
            let a1120 = v0;
            let a1121 = 28u64;
            nat_le(a1120, a1121)
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
    let a1128 = {
        let a1122 = 1u64;
        let a1123 = v0;
        nat_add(a1122, a1123)?
    };
    let a1129 = {
        let a1125 = match v1.clone() {
            Adt1::C0(_, h1124, _, _, _, _, _, _, _, _, _) => {
                h1124
            }
        };
        let a1126 = f55(v1);
        let a1127 = 0u64;
        nat_quot(a1125, a1126, a1127)
    };
    nat_add(a1128, a1129)
}

pub fn f54(v0: Adt1, v1: Adt1) -> u64 {
    let a1132 = match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1130, _, _, _, _, _) => {
            h1130
        }
    };
    let a1133 = f7(match v1.clone() {
        Adt1::C0(h1131, _, _, _, _, _, _, _, _, _, _) => {
            h1131
        }
    });
    if nat_eq(a1132, a1133) {
        f81(v0, v1)
    } else {
        0u64
    }
}

pub fn f55(v0: Adt1) -> u64 {
    let a1136 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1134, _, _, _, _, _) => {
            h1134
        }
    };
    let a1137 = f7(match v0.clone() {
        Adt1::C0(h1135, _, _, _, _, _, _, _, _, _, _) => {
            h1135
        }
    });
    if nat_eq(a1136, a1137) {
        match v0 {
            Adt1::C0(_, h1138, _, _, _, _, _, _, _, _, _) => {
                h1138
            }
        }
    } else {
        f35(match v0 {
            Adt1::C0(h1139, _, _, _, _, _, _, _, _, _, _) => {
                h1139
            }
        })
    }
}

pub fn f56(v0: Adt1) -> R<bool> {
    let v1: u64 = match v0.clone() {
        Adt1::C0(h1140, _, _, _, _, _, _, _, _, _, _) => {
            h1140
        }
    };
    let a1141 = v1;
    let a1142 = 0u64;
    if nat_eq(a1141, a1142) {
        f57(v0)
    } else {
        let a1143 = v1;
        let a1144 = 26u64;
        if if nat_lt(a1143, a1144) {
            true
        } else {
            let a1145 = 28u64;
            let a1146 = v1;
            nat_lt(a1145, a1146)
        } {
            Ok(f6(v0))
        } else {
            Ok(false)
        }
    }
}

pub fn f57(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1148 = match v0.clone() {
            Adt1::C0(_, h1147, _, _, _, _, _, _, _, _, _) => {
                h1147
            }
        };
        let a1149 = 4u64;
        nat_mul(a1148, a1149)?
    };
    let v2: u64 = {
        let a1152 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1150, _, _, _, _) => {
                h1150
            }
        };
        let a1153 = match v0.clone() {
            Adt1::C0(_, _, h1151, _, _, _, _, _, _, _, _) => {
                h1151
            }
        };
        nat_mul(a1152, a1153)?
    };
    let v3: u64 = {
        let a1155 = v2;
        let a1156 = match v0.clone() {
            Adt1::C0(_, _, _, h1154, _, _, _, _, _, _, _) => {
                h1154
            }
        };
        nat_mul(a1155, a1156)?
    };
    let a1158 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1157, _, _, _, _, _) => {
            h1157
        }
    };
    let a1159 = 4u64;
    if if if if nat_eq(a1158, a1159) {
        let a1164 = {
            let a1161 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1160) => {
                    h1160
                }
            };
            let a1162 = 4u64;
            let a1163 = 1u64;
            nat_rem(a1161, a1162, a1163)
        };
        let a1165 = 0u64;
        nat_eq(a1164, a1165)
    } else {
        false
    } {
        let a1173 = {
            let a1171 = match v0.clone() {
                Adt1::C0(_, _, h1166, _, _, _, _, _, _, _, _) => {
                    h1166
                }
            };
            let a1172 = {
                let a1169 = match v0.clone() {
                    Adt1::C0(_, _, _, h1167, _, _, _, _, _, _, _) => {
                        h1167
                    }
                };
                let a1170 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h1168, _, _, _, _, _, _) => {
                        h1168
                    }
                };
                nat_mul(a1169, a1170)?
            };
            nat_mul(a1171, a1172)?
        };
        let a1174 = 1u64;
        if nat_le(a1173, a1174) {
            true
        } else {
            let a1176 = v1;
            let a1177 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1175, _, _, _, _) => {
                    h1175
                }
            };
            if nat_le(a1176, a1177) {
                let a1182 = {
                    let a1179 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h1178, _, _, _, _) => {
                            h1178
                        }
                    };
                    let a1180 = 4u64;
                    let a1181 = 1u64;
                    nat_rem(a1179, a1180, a1181)
                };
                let a1183 = 0u64;
                nat_eq(a1182, a1183)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a1185 = match v0.clone() {
            Adt1::C0(_, _, _, h1184, _, _, _, _, _, _, _) => {
                h1184
            }
        };
        let a1186 = 1u64;
        if nat_le(a1185, a1186) {
            true
        } else {
            let a1188 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1187, _, _, _) => {
                    h1187
                }
            };
            let a1189 = v2;
            nat_eq(a1188, a1189)
        }
    } else {
        false
    } {
        let a1191 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1190, _, _, _, _, _, _) => {
                h1190
            }
        };
        let a1192 = 1u64;
        if nat_le(a1191, a1192) {
            Ok(true)
        } else {
            let a1194 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h1193, _, _) => {
                    h1193
                }
            };
            let a1195 = v3;
            Ok(nat_eq(a1194, a1195))
        }
    } else {
        Ok(false)
    }
}

pub fn f58(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a1196 = v2;
    let a1197 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a1196, a1197) {
        let a1199 = 0u64;
        let a1200 = f30(match v5.clone() {
            Adt1::C0(h1198, _, _, _, _, _, _, _, _, _, _) => {
                h1198
            }
        });
        nat_lt(a1199, a1200)
    } else {
        false
    } {
        let a1203 = match v5.clone() {
            Adt1::C0(h1201, _, _, _, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        let a1204 = match v6.clone() {
            Adt1::C0(h1202, _, _, _, _, _, _, _, _, _, _) => {
                h1202
            }
        };
        nat_eq(a1203, a1204)
    } else {
        false
    } {
        let a1207 = match v5.clone() {
            Adt1::C0(h1205, _, _, _, _, _, _, _, _, _, _) => {
                h1205
            }
        };
        let a1208 = match v7.clone() {
            Adt1::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                h1206
            }
        };
        nat_eq(a1207, a1208)
    } else {
        false
    } {
        f5(v5.clone(), v6.clone())
    } else {
        false
    } {
        f82(v5.clone())?
    } else {
        false
    } {
        f82(v6.clone())?
    } else {
        false
    } {
        f82(v7.clone())?
    } else {
        false
    } {
        let a1211 = match v5.clone() {
            Adt1::C0(_, _, h1209, _, _, _, _, _, _, _, _) => {
                h1209
            }
        };
        let a1212 = match v7.clone() {
            Adt1::C0(_, _, h1210, _, _, _, _, _, _, _, _) => {
                h1210
            }
        };
        nat_eq(a1211, a1212)
    } else {
        false
    } {
        let a1215 = match v5.clone() {
            Adt1::C0(_, _, _, h1213, _, _, _, _, _, _, _) => {
                h1213
            }
        };
        let a1216 = match v7.clone() {
            Adt1::C0(_, _, _, h1214, _, _, _, _, _, _, _) => {
                h1214
            }
        };
        nat_eq(a1215, a1216)
    } else {
        false
    } {
        let a1219 = match v5.clone() {
            Adt1::C0(_, _, _, _, h1217, _, _, _, _, _, _) => {
                h1217
            }
        };
        let a1220 = match v7.clone() {
            Adt1::C0(_, _, _, _, h1218, _, _, _, _, _, _) => {
                h1218
            }
        };
        nat_eq(a1219, a1220)
    } else {
        false
    } {
        let a1227 = match v7.clone() {
            Adt1::C0(_, h1221, _, _, _, _, _, _, _, _, _) => {
                h1221
            }
        };
        let a1228 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h1222, _, _, _, _, _, _, _, _, _) => {
                    h1222
                }
            }
        } else {
            let a1224 = match v5.clone() {
                Adt1::C0(_, h1223, _, _, _, _, _, _, _, _, _) => {
                    h1223
                }
            };
            let a1225 = 2u64;
            let a1226 = 0u64;
            nat_quot(a1224, a1225, a1226)
        };
        nat_eq(a1227, a1228)
    } else {
        false
    } {
        let a1230 = match v7.clone() {
            Adt1::C0(_, h1229, _, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1231 = 2147483647u64;
        nat_le(a1230, a1231)
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
        Ok(Adt3::C2(f59(v0, v1, v2, v3, v4, v5, v6, v7)?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f59(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt2> {
    Ok(Adt2::C0(if f23(v0) {
        let a1234 = 1u64;
        let a1235 = {
            let a1232 = f66(v0);
            let a1233 = f9(v7.clone())?;
            nat_mul(a1232, a1233)?
        };
        nat_add(a1234, a1235)?
    } else {
        let a1236 = 4u64;
        let a1237 = f9(v7.clone())?;
        nat_mul(a1236, a1237)?
    }, if f23(v0) {
        4u64
    } else {
        3u64
    }, None::<u64>, if v1 {
        98u64
    } else {
        let a1238 = 99u64;
        let a1239 = v2;
        nat_add(a1238, a1239)?
    }, v0, v3, v4, v5, v6, v7, false, 0u64))
}

pub fn f60(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<u64> {
    let v4: u64 = {
        let a1242 = match v2.clone() {
            Adt1::C0(_, h1240, _, _, _, _, _, _, _, _, _) => {
                h1240
            }
        };
        let a1243 = match v3.clone() {
            Adt1::C0(_, h1241, _, _, _, _, _, _, _, _, _) => {
                h1241
            }
        };
        let a1244 = 0u64;
        nat_quot(a1242, a1243, a1244)
    };
    let v5: u64 = {
        let a1247 = match v2.clone() {
            Adt1::C0(_, _, h1245, _, _, _, _, _, _, _, _) => {
                h1245
            }
        };
        let a1248 = match v3.clone() {
            Adt1::C0(_, _, h1246, _, _, _, _, _, _, _, _) => {
                h1246
            }
        };
        let a1249 = 0u64;
        nat_quot(a1247, a1248, a1249)
    };
    let v6: u64 = {
        let a1252 = match v2.clone() {
            Adt1::C0(_, _, _, h1250, _, _, _, _, _, _, _) => {
                h1250
            }
        };
        let a1253 = match v3.clone() {
            Adt1::C0(_, _, _, h1251, _, _, _, _, _, _, _) => {
                h1251
            }
        };
        let a1254 = 0u64;
        nat_quot(a1252, a1253, a1254)
    };
    let v7: u64 = {
        let a1255 = v1;
        let a1256 = v4;
        let a1257 = 0u64;
        nat_quot(a1255, a1256, a1257)
    };
    let v8: u64 = {
        let a1258 = v7;
        let a1259 = v5;
        let a1260 = 0u64;
        nat_quot(a1258, a1259, a1260)
    };
    let v9: u64 = {
        let a1262 = v0;
        let a1263 = match v3.clone() {
            Adt1::C0(_, _, h1261, _, _, _, _, _, _, _, _) => {
                h1261
            }
        };
        let a1264 = 0u64;
        nat_quot(a1262, a1263, a1264)
    };
    f83({
        let a1272 = {
            let a1269 = {
                let a1265 = v1;
                let a1266 = v4;
                let a1267 = 0u64;
                nat_rem(a1265, a1266, a1267)
            };
            let a1270 = match v3.clone() {
                Adt1::C0(_, h1268, _, _, _, _, _, _, _, _, _) => {
                    h1268
                }
            };
            nat_mul(a1269, a1270)?
        };
        let a1273 = f35(match v2.clone() {
            Adt1::C0(h1271, _, _, _, _, _, _, _, _, _, _) => {
                h1271
            }
        });
        let a1274 = 0u64;
        nat_quot(a1272, a1273, a1274)
    }, {
        let a1285 = {
            let a1279 = {
                let a1275 = v7;
                let a1276 = v5;
                let a1277 = 0u64;
                nat_rem(a1275, a1276, a1277)
            };
            let a1280 = match v3.clone() {
                Adt1::C0(_, _, h1278, _, _, _, _, _, _, _, _) => {
                    h1278
                }
            };
            nat_mul(a1279, a1280)?
        };
        let a1286 = {
            let a1282 = v0;
            let a1283 = match v3.clone() {
                Adt1::C0(_, _, h1281, _, _, _, _, _, _, _, _) => {
                    h1281
                }
            };
            let a1284 = 0u64;
            nat_rem(a1282, a1283, a1284)
        };
        nat_add(a1285, a1286)?
    }, {
        let a1297 = {
            let a1291 = {
                let a1287 = v8;
                let a1288 = v6;
                let a1289 = 0u64;
                nat_rem(a1287, a1288, a1289)
            };
            let a1292 = match v3.clone() {
                Adt1::C0(_, _, _, h1290, _, _, _, _, _, _, _) => {
                    h1290
                }
            };
            nat_mul(a1291, a1292)?
        };
        let a1298 = {
            let a1294 = v9;
            let a1295 = match v3.clone() {
                Adt1::C0(_, _, _, h1293, _, _, _, _, _, _, _) => {
                    h1293
                }
            };
            let a1296 = 0u64;
            nat_rem(a1294, a1295, a1296)
        };
        nat_add(a1297, a1298)?
    }, {
        let a1309 = {
            let a1303 = {
                let a1299 = v8;
                let a1300 = v6;
                let a1301 = 0u64;
                nat_quot(a1299, a1300, a1301)
            };
            let a1304 = match v3.clone() {
                Adt1::C0(_, _, _, _, h1302, _, _, _, _, _, _) => {
                    h1302
                }
            };
            nat_mul(a1303, a1304)?
        };
        let a1310 = {
            let a1306 = v9;
            let a1307 = match v3 {
                Adt1::C0(_, _, _, h1305, _, _, _, _, _, _, _) => {
                    h1305
                }
            };
            let a1308 = 0u64;
            nat_quot(a1306, a1307, a1308)
        };
        nat_add(a1309, a1310)?
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1311, _, _, _, _, _) => {
            h1311
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1312, _, _, _, _) => {
            h1312
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1313, _, _, _) => {
            h1313
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1314, _, _) => {
            h1314
        }
    })
}

pub fn f61(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1316 = v0;
        let a1317 = match v1.clone() {
            Adt1::C0(_, h1315, _, _, _, _, _, _, _, _, _) => {
                h1315
            }
        };
        let a1318 = 0u64;
        nat_quot(a1316, a1317, a1318)
    };
    let v3: u64 = {
        let a1320 = v2;
        let a1321 = match v1.clone() {
            Adt1::C0(_, _, h1319, _, _, _, _, _, _, _, _) => {
                h1319
            }
        };
        let a1322 = 0u64;
        nat_quot(a1320, a1321, a1322)
    };
    let v4: u64 = {
        let a1328 = {
            let a1324 = v0;
            let a1325 = match v1.clone() {
                Adt1::C0(_, h1323, _, _, _, _, _, _, _, _, _) => {
                    h1323
                }
            };
            let a1326 = 0u64;
            nat_rem(a1324, a1325, a1326)
        };
        let a1329 = f35(match v1.clone() {
            Adt1::C0(h1327, _, _, _, _, _, _, _, _, _, _) => {
                h1327
            }
        });
        let a1330 = 0u64;
        nat_quot(a1328, a1329, a1330)
    };
    f83(v4, {
        let a1332 = v2;
        let a1333 = match v1.clone() {
            Adt1::C0(_, _, h1331, _, _, _, _, _, _, _, _) => {
                h1331
            }
        };
        let a1334 = 0u64;
        nat_rem(a1332, a1333, a1334)
    }, {
        let a1336 = v3;
        let a1337 = match v1.clone() {
            Adt1::C0(_, _, _, h1335, _, _, _, _, _, _, _) => {
                h1335
            }
        };
        let a1338 = 0u64;
        nat_rem(a1336, a1337, a1338)
    }, {
        let a1340 = v3;
        let a1341 = match v1.clone() {
            Adt1::C0(_, _, _, h1339, _, _, _, _, _, _, _) => {
                h1339
            }
        };
        let a1342 = 0u64;
        nat_quot(a1340, a1341, a1342)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1343, _, _, _, _, _) => {
            h1343
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1344, _, _, _, _) => {
            h1344
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1345, _, _, _) => {
            h1345
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1346, _, _) => {
            h1346
        }
    })
}

pub fn f62(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1348 = v0;
        let a1349 = match v1.clone() {
            Adt1::C0(_, h1347, _, _, _, _, _, _, _, _, _) => {
                h1347
            }
        };
        let a1350 = 0u64;
        nat_quot(a1348, a1349, a1350)
    };
    let v3: u64 = {
        let a1352 = v2;
        let a1353 = match v1.clone() {
            Adt1::C0(_, _, h1351, _, _, _, _, _, _, _, _) => {
                h1351
            }
        };
        let a1354 = 0u64;
        nat_quot(a1352, a1353, a1354)
    };
    f83({
        let a1356 = v0;
        let a1357 = match v1.clone() {
            Adt1::C0(_, h1355, _, _, _, _, _, _, _, _, _) => {
                h1355
            }
        };
        let a1358 = 0u64;
        nat_rem(a1356, a1357, a1358)
    }, {
        let a1360 = v2;
        let a1361 = match v1.clone() {
            Adt1::C0(_, _, h1359, _, _, _, _, _, _, _, _) => {
                h1359
            }
        };
        let a1362 = 0u64;
        nat_rem(a1360, a1361, a1362)
    }, {
        let a1364 = v3;
        let a1365 = match v1.clone() {
            Adt1::C0(_, _, _, h1363, _, _, _, _, _, _, _) => {
                h1363
            }
        };
        let a1366 = 0u64;
        nat_rem(a1364, a1365, a1366)
    }, {
        let a1368 = v3;
        let a1369 = match v1.clone() {
            Adt1::C0(_, _, _, h1367, _, _, _, _, _, _, _) => {
                h1367
            }
        };
        let a1370 = 0u64;
        nat_quot(a1368, a1369, a1370)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1371, _, _, _, _, _) => {
            h1371
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1372, _, _, _, _) => {
            h1372
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1373, _, _, _) => {
            h1373
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1374, _, _) => {
            h1374
        }
    })
}

pub fn f63(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1376 = v0;
        let a1377 = match v1.clone() {
            Adt1::C0(_, h1375, _, _, _, _, _, _, _, _, _) => {
                h1375
            }
        };
        let a1378 = 0u64;
        nat_quot(a1376, a1377, a1378)
    };
    let v4: u64 = {
        let a1380 = v3;
        let a1381 = match v1.clone() {
            Adt1::C0(_, _, h1379, _, _, _, _, _, _, _, _) => {
                h1379
            }
        };
        let a1382 = 0u64;
        nat_quot(a1380, a1381, a1382)
    };
    f83({
        let a1388 = {
            let a1384 = v0;
            let a1385 = match v1.clone() {
                Adt1::C0(_, h1383, _, _, _, _, _, _, _, _, _) => {
                    h1383
                }
            };
            let a1386 = 0u64;
            nat_rem(a1384, a1385, a1386)
        };
        let a1389 = match v2.clone() {
            Adt1::C0(_, h1387, _, _, _, _, _, _, _, _, _) => {
                h1387
            }
        };
        let a1390 = 0u64;
        nat_rem(a1388, a1389, a1390)
    }, {
        let a1396 = {
            let a1392 = v3;
            let a1393 = match v1.clone() {
                Adt1::C0(_, _, h1391, _, _, _, _, _, _, _, _) => {
                    h1391
                }
            };
            let a1394 = 0u64;
            nat_rem(a1392, a1393, a1394)
        };
        let a1397 = match v2.clone() {
            Adt1::C0(_, _, h1395, _, _, _, _, _, _, _, _) => {
                h1395
            }
        };
        let a1398 = 0u64;
        nat_rem(a1396, a1397, a1398)
    }, {
        let a1404 = {
            let a1400 = v4;
            let a1401 = match v1.clone() {
                Adt1::C0(_, _, _, h1399, _, _, _, _, _, _, _) => {
                    h1399
                }
            };
            let a1402 = 0u64;
            nat_rem(a1400, a1401, a1402)
        };
        let a1405 = match v2.clone() {
            Adt1::C0(_, _, _, h1403, _, _, _, _, _, _, _) => {
                h1403
            }
        };
        let a1406 = 0u64;
        nat_rem(a1404, a1405, a1406)
    }, {
        let a1412 = {
            let a1408 = v4;
            let a1409 = match v1 {
                Adt1::C0(_, _, _, h1407, _, _, _, _, _, _, _) => {
                    h1407
                }
            };
            let a1410 = 0u64;
            nat_quot(a1408, a1409, a1410)
        };
        let a1413 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1411, _, _, _, _, _, _) => {
                h1411
            }
        };
        let a1414 = 0u64;
        nat_rem(a1412, a1413, a1414)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1415, _, _, _, _, _) => {
            h1415
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1416, _, _, _, _) => {
            h1416
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1417, _, _, _) => {
            h1417
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1418, _, _) => {
            h1418
        }
    })
}

pub fn f64(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1419 = v0;
        let a1420 = 2u64;
        let a1421 = 0u64;
        nat_quot(a1419, a1420, a1421)
    };
    let v5: u64 = f49(match v1.clone() {
        Adt1::C0(h1422, _, _, _, _, _, _, _, _, _, _) => {
            h1422
        }
    });
    let a1426 = {
        let a1423 = v0;
        let a1424 = 2u64;
        let a1425 = 0u64;
        nat_rem(a1423, a1424, a1425)
    };
    let a1427 = 0u64;
    if nat_eq(a1426, a1427) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f62(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1428 = v3;
        let a1429 = v2;
        if if nat_lt(a1428, a1429) {
            let a1432 = v3;
            let a1433 = {
                let a1430 = v5;
                let a1431 = 4u64;
                if nat_eq(a1430, a1431) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1432, a1433)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1434 = 3u64;
                let a1435 = v4;
                nat_add(a1434, a1435)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f65(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1437 = 1u64;
        let a1438 = f51(v1.clone(), match v3.clone() {
            Adt1::C0(h1436, _, _, _, _, _, _, _, _, _, _) => {
                h1436
            }
        })?;
        nat_add(a1437, a1438)?
    };
    let v6: u64 = {
        let a1439 = v0;
        let a1440 = v5;
        let a1441 = 0u64;
        nat_rem(a1439, a1440, a1441)
    };
    let v7: u64 = {
        let a1442 = v0;
        let a1443 = v5;
        let a1444 = 0u64;
        nat_quot(a1442, a1443, a1444)
    };
    let v8: u64 = {
        let a1446 = v7;
        let a1447 = match v1.clone() {
            Adt1::C0(_, _, h1445, _, _, _, _, _, _, _, _) => {
                h1445
            }
        };
        let a1448 = 0u64;
        nat_rem(a1446, a1447, a1448)
    };
    let v9: u64 = {
        let a1450 = v7;
        let a1451 = match v1.clone() {
            Adt1::C0(_, _, h1449, _, _, _, _, _, _, _, _) => {
                h1449
            }
        };
        let a1452 = 0u64;
        nat_quot(a1450, a1451, a1452)
    };
    let v10: u64 = {
        let a1454 = v9;
        let a1455 = match v1.clone() {
            Adt1::C0(_, _, _, h1453, _, _, _, _, _, _, _) => {
                h1453
            }
        };
        let a1456 = 0u64;
        nat_rem(a1454, a1455, a1456)
    };
    let v11: u64 = {
        let a1458 = v9;
        let a1459 = match v1.clone() {
            Adt1::C0(_, _, _, h1457, _, _, _, _, _, _, _) => {
                h1457
            }
        };
        let a1460 = 0u64;
        nat_quot(a1458, a1459, a1460)
    };
    let a1461 = v6;
    let a1462 = 0u64;
    if nat_eq(a1461, a1462) {
        let v12: u64 = {
            let a1479 = v8;
            let a1480 = {
                let a1477 = match v2.clone() {
                    Adt1::C0(_, h1463, _, _, _, _, _, _, _, _, _) => {
                        h1463
                    }
                };
                let a1478 = {
                    let a1475 = {
                        let a1465 = v10;
                        let a1466 = match v2.clone() {
                            Adt1::C0(_, _, h1464, _, _, _, _, _, _, _, _) => {
                                h1464
                            }
                        };
                        let a1467 = 0u64;
                        nat_rem(a1465, a1466, a1467)
                    };
                    let a1476 = {
                        let a1473 = match v2.clone() {
                            Adt1::C0(_, _, h1468, _, _, _, _, _, _, _, _) => {
                                h1468
                            }
                        };
                        let a1474 = {
                            let a1470 = v11;
                            let a1471 = match v2 {
                                Adt1::C0(_, _, _, h1469, _, _, _, _, _, _, _) => {
                                    h1469
                                }
                            };
                            let a1472 = 0u64;
                            nat_rem(a1470, a1471, a1472)
                        };
                        nat_mul(a1473, a1474)?
                    };
                    nat_add(a1475, a1476)?
                };
                nat_mul(a1477, a1478)?
            };
            nat_add(a1479, a1480)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1481 = 3u64;
            let a1482 = v12;
            nat_add(a1481, a1482)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1484 = v4;
        let a1485 = match v3.clone() {
            Adt1::C0(_, _, h1483, _, _, _, _, _, _, _, _) => {
                h1483
            }
        };
        if nat_lt(a1484, a1485) {
            let v13: u64 = {
                let a1490 = {
                    let a1486 = v6;
                    let a1487 = 1u64;
                    nat_sub(a1486, a1487)
                };
                let a1491 = f80(match v1.clone() {
                    Adt1::C0(h1488, _, _, _, _, _, _, _, _, _, _) => {
                        h1488
                    }
                }, match v3.clone() {
                    Adt1::C0(h1489, _, _, _, _, _, _, _, _, _, _) => {
                        h1489
                    }
                });
                let a1492 = 0u64;
                nat_quot(a1490, a1491, a1492)
            };
            let v14: u64 = f83({
                let a1494 = v13;
                let a1495 = f35(match v3.clone() {
                    Adt1::C0(h1493, _, _, _, _, _, _, _, _, _, _) => {
                        h1493
                    }
                });
                nat_mul(a1494, a1495)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1496, _, _, _, _, _) => {
                    h1496
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1497, _, _, _, _) => {
                    h1497
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1498, _, _, _) => {
                    h1498
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1499, _, _) => {
                    h1499
                }
            })?;
            let v15: u64 = f83(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1500, _, _, _, _, _) => {
                    h1500
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1501, _, _, _, _) => {
                    h1501
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1502, _, _, _) => {
                    h1502
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1503, _, _) => {
                    h1503
                }
            })?;
            let a1505 = 0u64;
            let a1506 = f52(match v3.clone() {
                Adt1::C0(h1504, _, _, _, _, _, _, _, _, _, _) => {
                    h1504
                }
            });
            if nat_lt(a1505, a1506) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f84(match v1 {
                    Adt1::C0(h1507, _, _, _, _, _, _, _, _, _, _) => {
                        h1507
                    }
                }, match v3 {
                    Adt1::C0(h1508, _, _, _, _, _, _, _, _, _, _) => {
                        h1508
                    }
                }, {
                    let a1509 = v6;
                    let a1510 = 1u64;
                    nat_sub(a1509, a1510)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f66(v0: u64) -> u64 {
    let a1511 = v0;
    let a1512 = 3u64;
    if nat_eq(a1511, a1512) {
        16u64
    } else {
        10u64
    }
}

pub fn f67(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f69(v0: u64, v1: u64) -> Adt3 {
    let a1513 = v0;
    let a1514 = 8u64;
    if nat_eq(a1513, a1514) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1515 = v0;
        let a1516 = 9u64;
        if nat_eq(a1515, a1516) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1517 = v0;
            let a1518 = 10u64;
            if nat_eq(a1517, a1518) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1519 = v0;
                let a1520 = 11u64;
                if nat_eq(a1519, a1520) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1521 = v0;
                    let a1522 = 12u64;
                    if nat_eq(a1521, a1522) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1523 = v0;
                        let a1524 = 13u64;
                        if nat_eq(a1523, a1524) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1525 = v0;
                            let a1526 = 14u64;
                            if nat_eq(a1525, a1526) {
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

pub fn f70(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1527, _, _, _, _) => {
            h1527
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1528, _, _, _) => {
            h1528
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1529, _, _) => {
            h1529
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1530, _, _, _, _, _, _, _, _) => {
            h1530
        }
    };
    let v7: u64 = {
        let a1532 = v1;
        let a1533 = match v5.clone() {
            Adt1::C0(_, h1531, _, _, _, _, _, _, _, _, _) => {
                h1531
            }
        };
        let a1534 = 0u64;
        nat_quot(a1532, a1533, a1534)
    };
    let v8: u64 = f30(match v5.clone() {
        Adt1::C0(h1535, _, _, _, _, _, _, _, _, _, _) => {
            h1535
        }
    });
    let v9: u64 = {
        let a1540 = {
            let a1537 = v1;
            let a1538 = match v5.clone() {
                Adt1::C0(_, h1536, _, _, _, _, _, _, _, _, _) => {
                    h1536
                }
            };
            let a1539 = 0u64;
            nat_rem(a1537, a1538, a1539)
        };
        let a1541 = v8;
        nat_mul(a1540, a1541)?
    };
    let v10: u64 = {
        let a1543 = match v5.clone() {
            Adt1::C0(_, h1542, _, _, _, _, _, _, _, _, _) => {
                h1542
            }
        };
        let a1544 = v8;
        nat_mul(a1543, a1544)?
    };
    let a1545 = v0;
    let a1546 = 0u64;
    if nat_eq(a1545, a1546) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1547, _, _, _, _, _, _, _, _, _, _) => {
                h1547
            }
        }, 0u64, 0u64, {
            let a1555 = {
                let a1551 = {
                    let a1549 = v7;
                    let a1550 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1548, _, _, _, _) => {
                            h1548
                        }
                    };
                    nat_mul(a1549, a1550)?
                };
                let a1552 = v9;
                nat_add(a1551, a1552)?
            };
            let a1556 = {
                let a1553 = v6;
                let a1554 = 100u64;
                if nat_eq(a1553, a1554) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1555, a1556)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1557 = v0;
        let a1558 = 1u64;
        if nat_eq(a1557, a1558) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1559, _, _, _, _, _, _, _, _, _, _) => {
                    h1559
                }
            }, 0u64, {
                let a1560 = v6;
                let a1561 = 98u64;
                if nat_eq(a1560, a1561) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1569 = {
                    let a1565 = {
                        let a1563 = v7;
                        let a1564 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1562, _, _, _, _) => {
                                h1562
                            }
                        };
                        nat_mul(a1563, a1564)?
                    };
                    let a1566 = v9;
                    nat_add(a1565, a1566)?
                };
                let a1570 = {
                    let a1567 = v6;
                    let a1568 = 99u64;
                    if nat_eq(a1567, a1568) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1569, a1570)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1571 = v0;
            let a1572 = 2u64;
            if nat_eq(a1571, a1572) {
                Ok(Adt3::C4(f85(match v2 {
                    Adt2::C0(_, _, _, _, h1573, _, _, _, _, _, _, _) => {
                        h1573
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1574, _, _, _, _, _, _, _, _, _, _) => {
                        h1574
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1575, _, _, _, _, _, _, _, _, _, _) => {
                        h1575
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1579 = {
                        let a1577 = v7;
                        let a1578 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1576, _, _, _, _) => {
                                h1576
                            }
                        };
                        nat_mul(a1577, a1578)?
                    };
                    let a1580 = v9;
                    nat_add(a1579, a1580)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f71(v0: u64) -> bool {
    let a1581 = v0;
    let a1582 = 27u64;
    if nat_eq(a1581, a1582) {
        true
    } else {
        let a1583 = v0;
        let a1584 = 28u64;
        nat_eq(a1583, a1584)
    }
}

pub fn f72(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1585, _, _, _, _) => {
            h1585
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1586, _, _, _) => {
            h1586
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1587, _, _) => {
            h1587
        }
    };
    let v5: u64 = match v1 {
        Adt2::C0(_, _, _, h1588, _, _, _, _, _, _, _, _) => {
            h1588
        }
    };
    let v6: u64 = f79(v5, match v2.clone() {
        Adt1::C0(h1589, _, _, _, _, _, _, _, _, _, _) => {
            h1589
        }
    });
    let v7: u64 = {
        let a1590 = v0;
        let a1591 = v6;
        let a1592 = 0u64;
        nat_quot(a1590, a1591, a1592)
    };
    let v8: u64 = f63(v7, v2.clone(), v3)?;
    let a1593 = v6;
    let a1594 = 2u64;
    if if nat_eq(a1593, a1594) {
        let a1598 = {
            let a1595 = v0;
            let a1596 = v6;
            let a1597 = 0u64;
            nat_rem(a1595, a1596, a1597)
        };
        let a1599 = 0u64;
        nat_eq(a1598, a1599)
    } else {
        false
    } {
        Ok(Adt3::C3(6u64, 15u64, 0u64, 0u64, 4u64, v8, 2u64, 9223372036854775808u64, 0u64, 0u64, 8u64, 8u64))
    } else {
        let v9: u64 = {
            let a1601 = match v2.clone() {
                Adt1::C0(h1600, _, _, _, _, _, _, _, _, _, _) => {
                    h1600
                }
            };
            let a1602 = 27u64;
            if nat_eq(a1601, a1602) {
                let a1603 = 15u64;
                let a1604 = v5;
                nat_add(a1603, a1604)?
            } else {
                let a1605 = v5;
                let a1606 = 2u64;
                if nat_eq(a1605, a1606) {
                    14u64
                } else {
                    0u64
                }
            }
        };
        Ok(Adt3::C3(6u64, v9, 0u64, 0u64, 0u64, f62(v7, v2)?, {
            let a1607 = v6;
            let a1608 = 2u64;
            if nat_eq(a1607, a1608) {
                1u64
            } else {
                4u64
            }
        }, {
            let a1609 = v6;
            let a1610 = 2u64;
            if nat_eq(a1609, a1610) {
                0u64
            } else {
                v8
            }
        }, 3u64, f62(v7, v4)?, 8u64, 8u64))
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1621 = {
        let a1617 = {
            let a1613 = 1u64;
            let a1614 = {
                let a1611 = v0;
                let a1612 = 0u64;
                if nat_eq(a1611, a1612) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a1613, a1614)?
        };
        let a1618 = {
            let a1615 = v1;
            let a1616 = 0u64;
            if nat_eq(a1615, a1616) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a1617, a1618)?
    };
    let a1622 = {
        let a1619 = v2;
        let a1620 = 0u64;
        if nat_eq(a1619, a1620) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a1621, a1622)
}

pub fn f74(v0: u64) -> u64 {
    let a1623 = v0;
    let a1624 = 43u64;
    if nat_eq(a1623, a1624) {
        1u64
    } else {
        f30(v0)
    }
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1625 = v1;
    let a1626 = 1u64;
    if if if nat_le(a1625, a1626) {
        true
    } else {
        let a1627 = v0;
        let a1628 = v2;
        nat_le(a1627, a1628)
    } {
        let a1629 = v3;
        let a1630 = 1u64;
        if nat_le(a1629, a1630) {
            true
        } else {
            let a1637 = {
                let a1635 = v0;
                let a1636 = {
                    let a1633 = {
                        let a1631 = v1;
                        let a1632 = 1u64;
                        nat_sub(a1631, a1632)
                    };
                    let a1634 = v2;
                    nat_mul(a1633, a1634)?
                };
                nat_add(a1635, a1636)?
            };
            let a1638 = v4;
            nat_le(a1637, a1638)
        }
    } else {
        false
    } {
        let a1639 = v5;
        let a1640 = 1u64;
        if nat_le(a1639, a1640) {
            Ok(true)
        } else {
            let a1653 = {
                let a1651 = {
                    let a1645 = v0;
                    let a1646 = {
                        let a1643 = {
                            let a1641 = v1;
                            let a1642 = 1u64;
                            nat_sub(a1641, a1642)
                        };
                        let a1644 = v2;
                        nat_mul(a1643, a1644)?
                    };
                    nat_add(a1645, a1646)?
                };
                let a1652 = {
                    let a1649 = {
                        let a1647 = v3;
                        let a1648 = 1u64;
                        nat_sub(a1647, a1648)
                    };
                    let a1650 = v4;
                    nat_mul(a1649, a1650)?
                };
                nat_add(a1651, a1652)?
            };
            let a1654 = v6;
            Ok(nat_le(a1653, a1654))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1655 = v0;
    let a1656 = 0u64;
    if if if nat_eq(a1655, a1656) {
        true
    } else {
        let a1657 = v1;
        let a1658 = 0u64;
        nat_eq(a1657, a1658)
    } {
        true
    } else {
        let a1659 = v2;
        let a1660 = 0u64;
        nat_eq(a1659, a1660)
    } {
        true
    } else {
        let a1661 = v3;
        let a1662 = 0u64;
        nat_eq(a1661, a1662)
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64) -> bool {
    let a1663 = v0;
    let a1664 = 1u64;
    if nat_le(a1663, a1664) {
        true
    } else {
        let a1668 = {
            let a1665 = v1;
            let a1666 = v2;
            let a1667 = 1u64;
            nat_rem(a1665, a1666, a1667)
        };
        let a1669 = 0u64;
        nat_eq(a1668, a1669)
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f77(v0, v3, v6) {
        f77(v1, v4, v6)
    } else {
        false
    } {
        f77(v2, v5, v6)
    } else {
        false
    }
}

pub fn f79(v0: u64, v1: u64) -> u64 {
    let a1670 = v1;
    let a1671 = 28u64;
    if if nat_eq(a1670, a1671) {
        let a1672 = v0;
        let a1673 = 1u64;
        nat_eq(a1672, a1673)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f80(v0: u64, v1: u64) -> u64 {
    let a1674 = v0;
    let a1675 = 1u64;
    if if nat_eq(a1674, a1675) {
        let a1676 = v1;
        let a1677 = 43u64;
        nat_eq(a1676, a1677)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f81(v0: Adt1, v1: Adt1) -> u64 {
    let v2: u64 = match v0.clone() {
        Adt1::C0(h1678, _, _, _, _, _, _, _, _, _, _) => {
            h1678
        }
    };
    let v3: u64 = match v1.clone() {
        Adt1::C0(h1679, _, _, _, _, _, _, _, _, _, _) => {
            h1679
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
            let a1684 = {
                let a1681 = match v0.clone() {
                    Adt1::C0(_, h1680, _, _, _, _, _, _, _, _, _) => {
                        h1680
                    }
                };
                let a1682 = f35(v2);
                let a1683 = 1u64;
                nat_rem(a1681, a1682, a1683)
            };
            let a1685 = 0u64;
            if nat_eq(a1684, a1685) {
                let a1690 = {
                    let a1687 = match v1.clone() {
                        Adt1::C0(_, h1686, _, _, _, _, _, _, _, _, _) => {
                            h1686
                        }
                    };
                    let a1688 = f35(v3);
                    let a1689 = 1u64;
                    nat_rem(a1687, a1688, a1689)
                };
                let a1691 = 0u64;
                nat_eq(a1690, a1691)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        let a1693 = match v0.clone() {
            Adt1::C0(_, h1692, _, _, _, _, _, _, _, _, _) => {
                h1692
            }
        };
        let a1694 = 0u64;
        if nat_eq(a1693, a1694) {
            f35(v3)
        } else {
            match v0 {
                Adt1::C0(_, h1695, _, _, _, _, _, _, _, _, _) => {
                    h1695
                }
            }
        }
    } else {
        if if if f27(v3) {
            let a1696 = 0u64;
            let a1697 = f30(v2);
            nat_lt(a1696, a1697)
        } else {
            false
        } {
            let a1702 = {
                let a1699 = match v0 {
                    Adt1::C0(_, h1698, _, _, _, _, _, _, _, _, _) => {
                        h1698
                    }
                };
                let a1700 = f35(v3);
                let a1701 = 1u64;
                nat_rem(a1699, a1700, a1701)
            };
            let a1703 = 0u64;
            if nat_eq(a1702, a1703) {
                let a1708 = {
                    let a1705 = match v1 {
                        Adt1::C0(_, h1704, _, _, _, _, _, _, _, _, _) => {
                            h1704
                        }
                    };
                    let a1706 = f35(v3);
                    let a1707 = 1u64;
                    nat_rem(a1705, a1706, a1707)
                };
                let a1709 = 0u64;
                nat_eq(a1708, a1709)
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

pub fn f82(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1712 = match v0.clone() {
            Adt1::C0(_, _, h1710, _, _, _, _, _, _, _, _) => {
                h1710
            }
        };
        let a1713 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1711, _, _, _, _) => {
                h1711
            }
        };
        nat_mul(a1712, a1713)?
    };
    if if f50(v0.clone()) {
        let a1715 = match v0.clone() {
            Adt1::C0(_, _, _, h1714, _, _, _, _, _, _, _) => {
                h1714
            }
        };
        let a1716 = 1u64;
        if nat_eq(a1715, a1716) {
            true
        } else {
            let a1718 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1717, _, _, _) => {
                    h1717
                }
            };
            let a1719 = v1;
            nat_eq(a1718, a1719)
        }
    } else {
        false
    } {
        let a1721 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1720, _, _, _, _, _, _) => {
                h1720
            }
        };
        let a1722 = 1u64;
        if nat_eq(a1721, a1722) {
            Ok(true)
        } else {
            let a1727 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1723, _, _) => {
                    h1723
                }
            };
            let a1728 = {
                let a1725 = v1;
                let a1726 = match v0 {
                    Adt1::C0(_, _, _, h1724, _, _, _, _, _, _, _) => {
                        h1724
                    }
                };
                nat_mul(a1725, a1726)?
            };
            Ok(nat_eq(a1727, a1728))
        }
    } else {
        Ok(false)
    }
}

pub fn f83(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1741 = {
        let a1737 = {
            let a1733 = {
                let a1729 = v0;
                let a1730 = v4;
                nat_mul(a1729, a1730)?
            };
            let a1734 = {
                let a1731 = v1;
                let a1732 = v5;
                nat_mul(a1731, a1732)?
            };
            nat_add(a1733, a1734)?
        };
        let a1738 = {
            let a1735 = v2;
            let a1736 = v6;
            nat_mul(a1735, a1736)?
        };
        nat_add(a1737, a1738)?
    };
    let a1742 = {
        let a1739 = v3;
        let a1740 = v7;
        nat_mul(a1739, a1740)?
    };
    nat_add(a1741, a1742)
}

pub fn f84(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1743 = f80(v0, v1);
    let a1744 = 2u64;
    if nat_eq(a1743, a1744) {
        let a1748 = {
            let a1745 = v2;
            let a1746 = 2u64;
            let a1747 = 0u64;
            nat_rem(a1745, a1746, a1747)
        };
        let a1749 = 0u64;
        if nat_eq(a1748, a1749) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f30(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f7(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f30(v0), f7(v1))
    }
}

pub fn f85(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1750 = v0;
    let a1751 = 2u64;
    if if if nat_eq(a1750, a1751) {
        true
    } else {
        let a1752 = v0;
        let a1753 = 5u64;
        nat_eq(a1752, a1753)
    } {
        let a1755 = match v1.clone() {
            Adt1::C0(h1754, _, _, _, _, _, _, _, _, _, _) => {
                h1754
            }
        };
        let a1756 = 0u64;
        if nat_eq(a1755, a1756) {
            let a1768 = {
                let a1758 = v2;
                let a1759 = match v1.clone() {
                    Adt1::C0(_, h1757, _, _, _, _, _, _, _, _, _) => {
                        h1757
                    }
                };
                let a1760 = 0u64;
                nat_rem(a1758, a1759, a1760)
            };
            let a1769 = {
                let a1766 = match v1.clone() {
                    Adt1::C0(_, h1761, _, _, _, _, _, _, _, _, _) => {
                        h1761
                    }
                };
                let a1767 = {
                    let a1763 = match v1 {
                        Adt1::C0(_, h1762, _, _, _, _, _, _, _, _, _) => {
                            h1762
                        }
                    };
                    let a1764 = 4u64;
                    let a1765 = 0u64;
                    nat_rem(a1763, a1764, a1765)
                };
                nat_sub(a1766, a1767)
            };
            nat_lt(a1768, a1769)
        } else {
            false
        }
    } else {
        false
    } {
        let a1770 = v0;
        let a1771 = 2u64;
        if nat_eq(a1770, a1771) {
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
