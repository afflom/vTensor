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
    C0(u64, bool, Adt1, Adt2, Adt2, Option<Adt2>, Adt2),
    C1(Adt3, u64, u64, u64),
    C2(Adt3, u64),
    C3(Adt1, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64, Adt1, Adt2, Adt2),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2(Adt3),
    C3(u64, u64),
    C4(u64, u64, u64),
    C5(u64, u64),
    C6(u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt4> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7) => {
            f1(v1, v2, v3, v4, v5, v6, v7)
        }
        Adt0::C1(v8, v9, v10, v11) => {
            f2(v8, v9, v10, v11)
        }
        Adt0::C2(v12, v13) => {
            f3(match v12.clone() {
                Adt3::C0(_, _, _, _, _, _, h2, _, _) => {
                    h2
                }
            }, match v12.clone() {
                Adt3::C0(_, _, h3, _, _, _, _, _, _) => {
                    h3
                }
            }, match v12.clone() {
                Adt3::C0(h4, _, _, _, _, _, _, _, _) => {
                    h4
                }
            }, match v12 {
                Adt3::C0(_, h5, _, _, _, _, _, _, _) => {
                    h5
                }
            }, v13)
        }
        Adt0::C3(v14, v15) => {
            f4(v14, v15)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: Adt1, v3: Adt2, v4: Adt2, v5: Option<Adt2>, v6: Adt2) -> R<Adt4> {
    let a6 = v0;
    let a7 = 45u64;
    if if nat_eq(a6, a7) {
        true
    } else {
        let a8 = v0;
        let a9 = 46u64;
        nat_eq(a8, a9)
    } {
        let v7: u64 = {
            let a11 = match v3.clone() {
                Adt2::C0(h10, _, _, _, _, _, _, _, _, _, _) => {
                    h10
                }
            };
            let a12 = 0u64;
            if nat_eq(a11, a12) {
                4u64
            } else {
                2u64
            }
        };
        let v8: u64 = {
            let a18 = {
                let a14 = match v2.clone() {
                    Adt1::C0(_, h13, _, _, _, _, _) => {
                        h13
                    }
                };
                let a15 = 24u64;
                if nat_eq(a14, a15) {
                    match v3.clone() {
                        Adt2::C0(_, h16, _, _, _, _, _, _, _, _, _) => {
                            h16
                        }
                    }
                } else {
                    match v2.clone() {
                        Adt1::C0(h17, _, _, _, _, _, _) => {
                            h17
                        }
                    }
                }
            };
            let a19 = 2u64;
            let a20 = 0u64;
            nat_quot(a18, a19, a20)
        };
        let v9: u64 = {
            let a23 = match v3.clone() {
                Adt2::C0(_, _, _, h21, _, _, _, _, _, _, _) => {
                    h21
                }
            };
            let a24 = if f5(match v2.clone() {
                Adt1::C0(_, h22, _, _, _, _, _) => {
                    h22
                }
            }) {
                4u64
            } else {
                1u64
            };
            nat_mul(a23, a24)?
        };
        if if if if if if if if if if if if if if if if if !v1 {
            let a26 = match v3.clone() {
                Adt2::C0(h25, _, _, _, _, _, _, _, _, _, _) => {
                    h25
                }
            };
            let a27 = 0u64;
            if nat_eq(a26, a27) {
                true
            } else {
                let a29 = match v3.clone() {
                    Adt2::C0(h28, _, _, _, _, _, _, _, _, _, _) => {
                        h28
                    }
                };
                let a30 = 1u64;
                nat_eq(a29, a30)
            }
        } else {
            false
        } {
            let a33 = match v3.clone() {
                Adt2::C0(h31, _, _, _, _, _, _, _, _, _, _) => {
                    h31
                }
            };
            let a34 = match v6.clone() {
                Adt2::C0(h32, _, _, _, _, _, _, _, _, _, _) => {
                    h32
                }
            };
            nat_eq(a33, a34)
        } else {
            false
        } {
            f6(v3.clone(), v6.clone())
        } else {
            false
        } {
            let a36 = match v3.clone() {
                Adt2::C0(_, _, _, _, _, h35, _, _, _, _, _) => {
                    h35
                }
            };
            let a37 = v7;
            nat_eq(a36, a37)
        } else {
            false
        } {
            let a39 = match v6.clone() {
                Adt2::C0(_, _, _, _, _, h38, _, _, _, _, _) => {
                    h38
                }
            };
            let a40 = v7;
            nat_eq(a39, a40)
        } else {
            false
        } {
            f7(v2.clone(), match v3.clone() {
                Adt2::C0(_, h41, _, _, _, _, _, _, _, _, _) => {
                    h41
                }
            })?
        } else {
            false
        } {
            if f8(v3.clone(), v6.clone()) {
                true
            } else {
                f9(v3.clone(), v6.clone())?
            }
        } else {
            false
        } {
            f9(v4.clone(), v6.clone())?
        } else {
            false
        } {
            let a43 = match v4.clone() {
                Adt2::C0(h42, _, _, _, _, _, _, _, _, _, _) => {
                    h42
                }
            };
            let a44 = 26u64;
            nat_eq(a43, a44)
        } else {
            false
        } {
            let a46 = match v4.clone() {
                Adt2::C0(_, h45, _, _, _, _, _, _, _, _, _) => {
                    h45
                }
            };
            let a47 = v9;
            nat_eq(a46, a47)
        } else {
            false
        } {
            let a49 = match v4.clone() {
                Adt2::C0(_, _, h48, _, _, _, _, _, _, _, _) => {
                    h48
                }
            };
            let a50 = 1u64;
            nat_eq(a49, a50)
        } else {
            false
        } {
            let a52 = match v4.clone() {
                Adt2::C0(_, _, _, h51, _, _, _, _, _, _, _) => {
                    h51
                }
            };
            let a53 = 1u64;
            nat_eq(a52, a53)
        } else {
            false
        } {
            let a55 = match v4.clone() {
                Adt2::C0(_, _, _, _, h54, _, _, _, _, _, _) => {
                    h54
                }
            };
            let a56 = 1u64;
            nat_eq(a55, a56)
        } else {
            false
        } {
            let a60 = {
                let a57 = v9;
                let a58 = 4u64;
                nat_mul(a57, a58)?
            };
            let a61 = match v4 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h59, _) => {
                    h59
                }
            };
            nat_le(a60, a61)
        } else {
            false
        } {
            f10(v5, v6.clone(), v8)?
        } else {
            false
        } {
            let a69 = {
                let a67 = match v3.clone() {
                    Adt2::C0(_, _, h62, _, _, _, _, _, _, _, _) => {
                        h62
                    }
                };
                let a68 = {
                    let a65 = match v3.clone() {
                        Adt2::C0(_, _, _, h63, _, _, _, _, _, _, _) => {
                            h63
                        }
                    };
                    let a66 = match v3.clone() {
                        Adt2::C0(_, _, _, _, h64, _, _, _, _, _, _) => {
                            h64
                        }
                    };
                    nat_mul(a65, a66)?
                };
                nat_mul(a67, a68)?
            };
            let a70 = 2147483647u64;
            nat_le(a69, a70)
        } else {
            false
        } {
            Ok(Adt4::C2(Adt3::C0(v7, v8, match v3.clone() {
                Adt2::C0(_, h71, _, _, _, _, _, _, _, _, _) => {
                    h71
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h72, _, _, _, _, _, _, _, _) => {
                    h72
                }
            }, match v3.clone() {
                Adt2::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                    h73
                }
            }, match v3.clone() {
                Adt2::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
                    h74
                }
            }, v2, v3, v6)))
        } else {
            Ok(Adt4::C1)
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f2(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    let a76 = v1;
    let a77 = match v0.clone() {
        Adt3::C0(_, _, _, h75, _, _, _, _, _) => {
            h75
        }
    };
    if if if nat_lt(a76, a77) {
        let a79 = v2;
        let a80 = match v0.clone() {
            Adt3::C0(_, _, _, _, h78, _, _, _, _) => {
                h78
            }
        };
        nat_lt(a79, a80)
    } else {
        false
    } {
        let a82 = v3;
        let a83 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h81, _, _, _) => {
                h81
            }
        };
        nat_lt(a82, a83)
    } else {
        false
    } {
        Ok(Adt4::C3(f11(match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h84, _) => {
                h84
            }
        }, v1, v2, v3)?, f11(match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, h85) => {
                h85
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt4::C1)
    }
}

pub fn f3(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt4> {
    let v5: u64 = {
        let a87 = match v0.clone() {
            Adt1::C0(_, _, h86, _, _, _, _) => {
                h86
            }
        };
        let a88 = 2u64;
        let a89 = 0u64;
        nat_quot(a87, a88, a89)
    };
    let v6: u64 = {
        let a90 = v4;
        let a91 = v5;
        nat_sub(a90, a91)
    };
    let v7: u64 = {
        let a98 = match v0.clone() {
            Adt1::C0(_, _, h92, _, _, _, _) => {
                h92
            }
        };
        let a99 = {
            let a94 = match v0.clone() {
                Adt1::C0(_, h93, _, _, _, _, _) => {
                    h93
                }
            };
            let a95 = 0u64;
            if nat_eq(a94, a95) {
                let a96 = 2u64;
                let a97 = v6;
                nat_mul(a96, a97)?
            } else {
                v6
            }
        };
        nat_add(a98, a99)?
    };
    let v8: u64 = {
        let a101 = match v0 {
            Adt1::C0(_, h100, _, _, _, _, _) => {
                h100
            }
        };
        let a102 = 0u64;
        if nat_eq(a101, a102) {
            1u64
        } else {
            v3
        }
    };
    let a106 = v4;
    let a107 = {
        let a103 = v1;
        let a104 = 2u64;
        let a105 = 0u64;
        nat_quot(a103, a104, a105)
    };
    if nat_lt(a106, a107) {
        let a108 = v5;
        let a109 = v4;
        if if nat_le(a108, a109) {
            let a110 = v6;
            let a111 = v3;
            nat_lt(a110, a111)
        } else {
            false
        } {
            Ok(Adt4::C4(v6, {
                let a112 = v7;
                let a113 = v2;
                nat_mul(a112, a113)?
            }, {
                let a116 = {
                    let a114 = v7;
                    let a115 = v8;
                    nat_add(a114, a115)?
                };
                let a117 = v2;
                nat_mul(a116, a117)?
            }))
        } else {
            Ok(Adt4::C5({
                let a120 = {
                    let a118 = 2u64;
                    let a119 = v4;
                    nat_mul(a118, a119)?
                };
                let a121 = v2;
                nat_mul(a120, a121)?
            }, {
                let a126 = {
                    let a124 = {
                        let a122 = 2u64;
                        let a123 = v4;
                        nat_mul(a122, a123)?
                    };
                    let a125 = 1u64;
                    nat_add(a124, a125)?
                };
                let a127 = v2;
                nat_mul(a126, a127)?
            }))
        }
    } else {
        Ok(Adt4::C1)
    }
}

pub fn f4(v0: Adt1, v1: u64) -> R<Adt4> {
    let v2: u64 = match v0.clone() {
        Adt1::C0(_, _, _, h128, _, _, _) => {
            h128
        }
    };
    let v3: u64 = match v0.clone() {
        Adt1::C0(_, _, _, _, h129, _, _) => {
            h129
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h130, _) => {
            h130
        }
    };
    let v5: u64 = {
        let a131 = v2;
        let a132 = v3;
        nat_add(a131, a132)?
    };
    let v6: u64 = {
        let a135 = {
            let a133 = v2;
            let a134 = v3;
            nat_add(a133, a134)?
        };
        let a136 = v4;
        nat_add(a135, a136)?
    };
    let v7: u64 = {
        let a138 = v6;
        let a139 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h137) => {
                h137
            }
        };
        nat_add(a138, a139)?
    };
    let v8: u64 = {
        let a140 = v1;
        let a141 = v7;
        let a142 = 0u64;
        nat_rem(a140, a141, a142)
    };
    let v9: u64 = {
        let a146 = {
            let a143 = v8;
            let a144 = 3u64;
            let a145 = 0u64;
            nat_rem(a143, a144, a145)
        };
        let a147 = 1u64;
        if if nat_eq(a146, a147) {
            let a150 = v8;
            let a151 = {
                let a148 = 3u64;
                let a149 = v3;
                nat_mul(a148, a149)?
            };
            nat_lt(a150, a151)
        } else {
            false
        } {
            1u64
        } else {
            let a155 = {
                let a152 = v8;
                let a153 = 3u64;
                let a154 = 0u64;
                nat_rem(a152, a153, a154)
            };
            let a156 = 2u64;
            if if nat_eq(a155, a156) {
                let a159 = v8;
                let a160 = {
                    let a157 = 3u64;
                    let a158 = v4;
                    nat_mul(a157, a158)?
                };
                nat_lt(a159, a160)
            } else {
                false
            } {
                2u64
            } else {
                let a164 = {
                    let a161 = v8;
                    let a162 = 3u64;
                    let a163 = 0u64;
                    nat_rem(a161, a162, a163)
                };
                let a165 = 0u64;
                if if nat_eq(a164, a165) {
                    let a168 = v8;
                    let a169 = {
                        let a166 = 3u64;
                        let a167 = v2;
                        nat_mul(a166, a167)?
                    };
                    nat_lt(a168, a169)
                } else {
                    false
                } {
                    0u64
                } else {
                    3u64
                }
            }
        }
    };
    let v10: u64 = {
        let a170 = v8;
        let a171 = v2;
        if nat_lt(a170, a171) {
            0u64
        } else {
            let a172 = v8;
            let a173 = v5;
            if nat_lt(a172, a173) {
                1u64
            } else {
                let a174 = v8;
                let a175 = v6;
                if nat_lt(a174, a175) {
                    2u64
                } else {
                    3u64
                }
            }
        }
    };
    let v11: u64 = {
        let a177 = match v0.clone() {
            Adt1::C0(_, h176, _, _, _, _, _) => {
                h176
            }
        };
        let a178 = 40u64;
        if nat_eq(a177, a178) {
            v9
        } else {
            if f5(match v0.clone() {
                Adt1::C0(_, h179, _, _, _, _, _) => {
                    h179
                }
            }) {
                v10
            } else {
                0u64
            }
        }
    };
    let v12: u64 = {
        let a180 = v8;
        let a181 = 0u64;
        if nat_eq(a180, a181) {
            1u64
        } else {
            let a182 = v8;
            let a183 = v2;
            if nat_eq(a182, a183) {
                2u64
            } else {
                let a184 = v8;
                let a185 = v5;
                if nat_eq(a184, a185) {
                    3u64
                } else {
                    let a186 = v8;
                    let a187 = v6;
                    if nat_eq(a186, a187) {
                        4u64
                    } else {
                        0u64
                    }
                }
            }
        }
    };
    Ok(Adt4::C6(v11, {
        let a189 = match v0 {
            Adt1::C0(_, h188, _, _, _, _, _) => {
                h188
            }
        };
        let a190 = 24u64;
        if nat_eq(a189, a190) {
            v12
        } else {
            0u64
        }
    }))
}

pub fn f5(v0: u64) -> bool {
    let a191 = v0;
    let a192 = 8u64;
    if if nat_eq(a191, a192) {
        true
    } else {
        let a193 = v0;
        let a194 = 24u64;
        nat_eq(a193, a194)
    } {
        true
    } else {
        let a195 = v0;
        let a196 = 40u64;
        nat_eq(a195, a196)
    }
}

pub fn f6(v0: Adt2, v1: Adt2) -> bool {
    let a199 = match v0.clone() {
        Adt2::C0(_, h197, _, _, _, _, _, _, _, _, _) => {
            h197
        }
    };
    let a200 = match v1.clone() {
        Adt2::C0(_, h198, _, _, _, _, _, _, _, _, _) => {
            h198
        }
    };
    if if if nat_eq(a199, a200) {
        let a203 = match v0.clone() {
            Adt2::C0(_, _, h201, _, _, _, _, _, _, _, _) => {
                h201
            }
        };
        let a204 = match v1.clone() {
            Adt2::C0(_, _, h202, _, _, _, _, _, _, _, _) => {
                h202
            }
        };
        nat_eq(a203, a204)
    } else {
        false
    } {
        let a207 = match v0.clone() {
            Adt2::C0(_, _, _, h205, _, _, _, _, _, _, _) => {
                h205
            }
        };
        let a208 = match v1.clone() {
            Adt2::C0(_, _, _, h206, _, _, _, _, _, _, _) => {
                h206
            }
        };
        nat_eq(a207, a208)
    } else {
        false
    } {
        let a211 = match v0 {
            Adt2::C0(_, _, _, _, h209, _, _, _, _, _, _) => {
                h209
            }
        };
        let a212 = match v1 {
            Adt2::C0(_, _, _, _, h210, _, _, _, _, _, _) => {
                h210
            }
        };
        nat_eq(a211, a212)
    } else {
        false
    }
}

pub fn f7(v0: Adt1, v1: u64) -> R<bool> {
    let v2: u64 = match v0.clone() {
        Adt1::C0(_, h213, _, _, _, _, _) => {
            h213
        }
    };
    let v3: u64 = match v0.clone() {
        Adt1::C0(h214, _, _, _, _, _, _) => {
            h214
        }
    };
    let v4: u64 = {
        let a223 = {
            let a220 = {
                let a217 = match v0.clone() {
                    Adt1::C0(_, _, _, h215, _, _, _) => {
                        h215
                    }
                };
                let a218 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h216, _, _) => {
                        h216
                    }
                };
                nat_add(a217, a218)?
            };
            let a221 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h219, _) => {
                    h219
                }
            };
            nat_add(a220, a221)?
        };
        let a224 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h222) => {
                h222
            }
        };
        nat_add(a223, a224)?
    };
    let a225 = 0u64;
    let a226 = v3;
    if if if if if if if if nat_lt(a225, a226) {
        f12(v3)
    } else {
        false
    } {
        f12(v1)
    } else {
        false
    } {
        f12(match v0.clone() {
            Adt1::C0(_, _, h227, _, _, _, _) => {
                h227
            }
        })
    } else {
        false
    } {
        let a231 = {
            let a229 = v3;
            let a230 = match v0.clone() {
                Adt1::C0(_, _, h228, _, _, _, _) => {
                    h228
                }
            };
            nat_add(a229, a230)?
        };
        let a232 = v1;
        nat_le(a231, a232)
    } else {
        false
    } {
        let a233 = v1;
        let a234 = 2147483647u64;
        nat_le(a233, a234)
    } else {
        false
    } {
        let a235 = v2;
        let a236 = 0u64;
        if if nat_eq(a235, a236) {
            true
        } else {
            let a237 = v2;
            let a238 = 2u64;
            nat_eq(a237, a238)
        } {
            true
        } else {
            f5(v2)
        }
    } else {
        false
    } {
        if f5(v2) {
            let a246 = 0u64;
            let a247 = {
                let a244 = {
                    let a241 = match v0.clone() {
                        Adt1::C0(_, _, _, h239, _, _, _) => {
                            h239
                        }
                    };
                    let a242 = match v0.clone() {
                        Adt1::C0(_, _, _, _, h240, _, _) => {
                            h240
                        }
                    };
                    nat_add(a241, a242)?
                };
                let a245 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h243, _) => {
                        h243
                    }
                };
                nat_add(a244, a245)?
            };
            if nat_lt(a246, a247) {
                let a248 = v4;
                let a249 = v1;
                nat_le(a248, a249)
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        let a250 = v2;
        let a251 = 24u64;
        if nat_eq(a250, a251) {
            let a253 = match v0 {
                Adt1::C0(_, _, h252, _, _, _, _) => {
                    h252
                }
            };
            let a254 = 0u64;
            if nat_eq(a253, a254) {
                let a257 = {
                    let a255 = v3;
                    let a256 = 2u64;
                    nat_mul(a255, a256)?
                };
                let a258 = v1;
                Ok(nat_eq(a257, a258))
            } else {
                Ok(false)
            }
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

pub fn f8(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f6(v0.clone(), v1.clone()) {
        let a261 = match v0.clone() {
            Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                h259
            }
        };
        let a262 = match v1.clone() {
            Adt2::C0(h260, _, _, _, _, _, _, _, _, _, _) => {
                h260
            }
        };
        nat_eq(a261, a262)
    } else {
        false
    } {
        let a265 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h263) => {
                h263
            }
        };
        let a266 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h264) => {
                h264
            }
        };
        nat_eq(a265, a266)
    } else {
        false
    } {
        let a269 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h267, _, _, _, _, _) => {
                h267
            }
        };
        let a270 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h268, _, _, _, _, _) => {
                h268
            }
        };
        nat_eq(a269, a270)
    } else {
        false
    } {
        let a273 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h271, _, _, _, _) => {
                h271
            }
        };
        let a274 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h272, _, _, _, _) => {
                h272
            }
        };
        nat_eq(a273, a274)
    } else {
        false
    } {
        let a277 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h275, _, _, _) => {
                h275
            }
        };
        let a278 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h276, _, _, _) => {
                h276
            }
        };
        nat_eq(a277, a278)
    } else {
        false
    } {
        let a281 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h279, _, _) => {
                h279
            }
        };
        let a282 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h280, _, _) => {
                h280
            }
        };
        nat_eq(a281, a282)
    } else {
        false
    }
}

pub fn f9(v0: Adt2, v1: Adt2) -> R<bool> {
    let a284 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h283, _) => {
            h283
        }
    };
    let a285 = 0u64;
    if nat_eq(a284, a285) {
        Ok(true)
    } else {
        let a287 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h286, _) => {
                h286
            }
        };
        let a288 = 0u64;
        if nat_eq(a287, a288) {
            Ok(true)
        } else {
            let a290 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h289) => {
                    h289
                }
            };
            let a291 = 0u64;
            if nat_eq(a290, a291) {
                Ok(true)
            } else {
                let a293 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h292) => {
                        h292
                    }
                };
                let a294 = 0u64;
                if nat_eq(a293, a294) {
                    Ok(true)
                } else {
                    let a300 = {
                        let a297 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h295) => {
                                h295
                            }
                        };
                        let a298 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h296, _) => {
                                h296
                            }
                        };
                        nat_add(a297, a298)?
                    };
                    let a301 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h299) => {
                            h299
                        }
                    };
                    if nat_le(a300, a301) {
                        Ok(true)
                    } else {
                        let a307 = {
                            let a304 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h302) => {
                                    h302
                                }
                            };
                            let a305 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h303, _) => {
                                    h303
                                }
                            };
                            nat_add(a304, a305)?
                        };
                        let a308 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h306) => {
                                h306
                            }
                        };
                        Ok(nat_le(a307, a308))
                    }
                }
            }
        }
    }
}

pub fn f10(v0: Option<Adt2>, v1: Adt2, v2: u64) -> R<bool> {
    let m309 = v0;
    match m309 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a311 = match v3.clone() {
                Adt2::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
                    h310
                }
            };
            let a312 = 0u64;
            if if if nat_eq(a311, a312) {
                let a314 = v2;
                let a315 = match v3.clone() {
                    Adt2::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                        h313
                    }
                };
                nat_le(a314, a315)
            } else {
                false
            } {
                let a319 = {
                    let a316 = v2;
                    let a317 = 4u64;
                    nat_mul(a316, a317)?
                };
                let a320 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h318, _) => {
                        h318
                    }
                };
                nat_le(a319, a320)
            } else {
                false
            } {
                f9(v3, v1)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f11(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a332 = {
        let a327 = {
            let a322 = v1;
            let a323 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h321, _, _, _, _) => {
                    h321
                }
            };
            nat_mul(a322, a323)?
        };
        let a328 = {
            let a325 = v2;
            let a326 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h324, _, _, _) => {
                    h324
                }
            };
            nat_mul(a325, a326)?
        };
        nat_add(a327, a328)?
    };
    let a333 = {
        let a330 = v3;
        let a331 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h329, _, _) => {
                h329
            }
        };
        nat_mul(a330, a331)?
    };
    nat_add(a332, a333)
}

pub fn f12(v0: u64) -> bool {
    let a337 = {
        let a334 = v0;
        let a335 = 2u64;
        let a336 = 0u64;
        nat_rem(a334, a335, a336)
    };
    let a338 = 0u64;
    nat_eq(a337, a338)
}

pub fn run(p0: Adt0) -> R<Adt4> {
    f0(p0)
}
