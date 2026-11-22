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
    C0(u64, bool, Adt1, Adt1, Adt1),
    C1(Adt2, u64),
    C2(Adt2, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(Adt2),
    C2(u64, u64, u64),
    C3(u64),
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

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt3> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5) => {
            f1(v1, v2, v3, v4, v5)
        }
        Adt0::C1(v6, v7) => {
            let a3 = v7;
            let a4 = match v6.clone() {
                Adt2::C0(_, _, _, _, _, h2, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2
                }
            };
            if nat_lt(a3, a4) {
                let v8: u64 = {
                    let a6 = v7;
                    let a7 = match v6.clone() {
                        Adt2::C0(_, _, _, h5, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h5
                        }
                    };
                    let a8 = 0u64;
                    nat_rem(a6, a7, a8)
                };
                let v9: u64 = {
                    let a10 = v7;
                    let a11 = match v6.clone() {
                        Adt2::C0(_, _, _, h9, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h9
                        }
                    };
                    let a12 = 0u64;
                    nat_quot(a10, a11, a12)
                };
                let v10: u64 = {
                    let a14 = v9;
                    let a15 = match v6.clone() {
                        Adt2::C0(_, _, _, _, h13, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h13
                        }
                    };
                    let a16 = 0u64;
                    nat_rem(a14, a15, a16)
                };
                let v11: u64 = {
                    let a18 = v9;
                    let a19 = match v6.clone() {
                        Adt2::C0(_, _, _, _, h17, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h17
                        }
                    };
                    let a20 = 0u64;
                    nat_quot(a18, a19, a20)
                };
                Ok(Adt3::C2({
                    let a35 = {
                        let a26 = {
                            let a22 = v10;
                            let a23 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h21, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h21
                                }
                            };
                            let a24 = 0u64;
                            nat_quot(a22, a23, a24)
                        };
                        let a27 = match v6.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h25, _, _, _, _, _, _, _, _) => {
                                h25
                            }
                        };
                        nat_mul(a26, a27)?
                    };
                    let a36 = {
                        let a33 = {
                            let a29 = v11;
                            let a30 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h28, _, _, _, _, _, _, _, _, _, _) => {
                                    h28
                                }
                            };
                            let a31 = 0u64;
                            nat_quot(a29, a30, a31)
                        };
                        let a34 = match v6.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h32, _, _, _, _, _, _, _) => {
                                h32
                            }
                        };
                        nat_mul(a33, a34)?
                    };
                    nat_add(a35, a36)?
                }, {
                    let a48 = {
                        let a43 = {
                            let a38 = v8;
                            let a39 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h37, _, _, _, _, _, _) => {
                                    h37
                                }
                            };
                            nat_mul(a38, a39)?
                        };
                        let a44 = {
                            let a41 = v10;
                            let a42 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h40, _, _, _, _, _) => {
                                    h40
                                }
                            };
                            nat_mul(a41, a42)?
                        };
                        nat_add(a43, a44)?
                    };
                    let a49 = {
                        let a46 = v11;
                        let a47 = match v6.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h45, _, _, _, _) => {
                                h45
                            }
                        };
                        nat_mul(a46, a47)?
                    };
                    nat_add(a48, a49)?
                }, {
                    let a61 = {
                        let a56 = {
                            let a51 = v8;
                            let a52 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h50, _, _, _) => {
                                    h50
                                }
                            };
                            nat_mul(a51, a52)?
                        };
                        let a57 = {
                            let a54 = v10;
                            let a55 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h53, _, _) => {
                                    h53
                                }
                            };
                            nat_mul(a54, a55)?
                        };
                        nat_add(a56, a57)?
                    };
                    let a62 = {
                        let a59 = v11;
                        let a60 = match v6 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h58, _) => {
                                h58
                            }
                        };
                        nat_mul(a59, a60)?
                    };
                    nat_add(a61, a62)?
                }))
            } else {
                Ok(Adt3::C0)
            }
        }
        Adt0::C2(v12, v13, v14) => {
            let a64 = v14;
            let a65 = match v12.clone() {
                Adt2::C0(_, _, h63, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h63
                }
            };
            if nat_lt(a64, a65) {
                Ok(Adt3::C3({
                    let a69 = v13;
                    let a70 = {
                        let a67 = v14;
                        let a68 = match v12 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h66, _, _, _, _, _, _, _, _, _) => {
                                h66
                            }
                        };
                        nat_mul(a67, a68)?
                    };
                    nat_add(a69, a70)?
                }))
            } else {
                Ok(Adt3::C0)
            }
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1) -> R<Adt3> {
    if v1 {
        Ok(Adt3::C0)
    } else {
        let m72 = f2(match v2.clone() {
            Adt1::C0(h71, _, _, _, _, _, _, _, _) => {
                h71
            }
        });
        match m72 {
            Adt4::C0 => {
                Ok(Adt3::C0)
            }
            Adt4::C1(v5) => {
                f3(v0, v2, v3, v4, match v5.clone() {
                    Adt5::C0(_, h73, _) => {
                        h73
                    }
                }, match v5 {
                    Adt5::C0(_, _, h74) => {
                        h74
                    }
                })
            }
        }
    }
}

pub fn f2(v0: u64) -> Adt4 {
    let a75 = v0;
    let a76 = 0u64;
    if nat_eq(a75, a76) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a77 = v0;
        let a78 = 1u64;
        if nat_eq(a77, a78) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a79 = v0;
            let a80 = 2u64;
            if nat_eq(a79, a80) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a81 = v0;
                let a82 = 3u64;
                if nat_eq(a81, a82) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a83 = v0;
                    let a84 = 6u64;
                    if nat_eq(a83, a84) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a85 = v0;
                        let a86 = 7u64;
                        if nat_eq(a85, a86) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a87 = v0;
                            let a88 = 8u64;
                            if nat_eq(a87, a88) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a89 = v0;
                                let a90 = 9u64;
                                if nat_eq(a89, a90) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a91 = v0;
                                    let a92 = 10u64;
                                    if nat_eq(a91, a92) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a93 = v0;
                                        let a94 = 11u64;
                                        if nat_eq(a93, a94) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a95 = v0;
                                            let a96 = 12u64;
                                            if nat_eq(a95, a96) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a97 = v0;
                                                let a98 = 13u64;
                                                if nat_eq(a97, a98) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a99 = v0;
                                                    let a100 = 14u64;
                                                    if nat_eq(a99, a100) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a101 = v0;
                                                        let a102 = 15u64;
                                                        if nat_eq(a101, a102) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a103 = v0;
                                                            let a104 = 16u64;
                                                            if nat_eq(a103, a104) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a105 = v0;
                                                                let a106 = 17u64;
                                                                if nat_eq(a105, a106) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a107 = v0;
                                                                    let a108 = 18u64;
                                                                    if nat_eq(a107, a108) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a109 = v0;
                                                                        let a110 = 19u64;
                                                                        if nat_eq(a109, a110) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a111 = v0;
                                                                            let a112 = 20u64;
                                                                            if nat_eq(a111, a112) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a113 = v0;
                                                                                let a114 = 21u64;
                                                                                if nat_eq(a113, a114) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a115 = v0;
                                                                                    let a116 = 22u64;
                                                                                    if nat_eq(a115, a116) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a117 = v0;
                                                                                        let a118 = 23u64;
                                                                                        if nat_eq(a117, a118) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a119 = v0;
                                                                                            let a120 = 24u64;
                                                                                            if nat_eq(a119, a120) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a121 = v0;
                                                                                                let a122 = 25u64;
                                                                                                if nat_eq(a121, a122) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a123 = v0;
                                                                                                    let a124 = 26u64;
                                                                                                    if nat_eq(a123, a124) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a125 = v0;
                                                                                                        let a126 = 27u64;
                                                                                                        if nat_eq(a125, a126) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a127 = v0;
                                                                                                            let a128 = 28u64;
                                                                                                            if nat_eq(a127, a128) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a129 = v0;
                                                                                                                let a130 = 29u64;
                                                                                                                if nat_eq(a129, a130) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a131 = v0;
                                                                                                                    let a132 = 30u64;
                                                                                                                    if nat_eq(a131, a132) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a133 = v0;
                                                                                                                        let a134 = 34u64;
                                                                                                                        if nat_eq(a133, a134) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a135 = v0;
                                                                                                                            let a136 = 35u64;
                                                                                                                            if nat_eq(a135, a136) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a137 = v0;
                                                                                                                                let a138 = 39u64;
                                                                                                                                if nat_eq(a137, a138) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a139 = v0;
                                                                                                                                    let a140 = 40u64;
                                                                                                                                    if nat_eq(a139, a140) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a141 = v0;
                                                                                                                                        let a142 = 41u64;
                                                                                                                                        if nat_eq(a141, a142) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a143 = v0;
                                                                                                                                            let a144 = 42u64;
                                                                                                                                            if nat_eq(a143, a144) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a145 = v0;
                                                                                                                                                let a146 = 43u64;
                                                                                                                                                if nat_eq(a145, a146) {
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

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64, v5: u64) -> R<Adt3> {
    if if if if if if if if f4(match v1.clone() {
        Adt1::C0(h147, _, _, _, _, _, _, _, _) => {
            h147
        }
    }) {
        let a152 = {
            let a149 = match v1.clone() {
                Adt1::C0(_, h148, _, _, _, _, _, _, _) => {
                    h148
                }
            };
            let a150 = v4;
            let a151 = 0u64;
            nat_rem(a149, a150, a151)
        };
        let a153 = 0u64;
        nat_eq(a152, a153)
    } else {
        false
    } {
        let a155 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h154, _, _, _) => {
                h154
            }
        };
        let a156 = v5;
        nat_eq(a155, a156)
    } else {
        false
    } {
        let a158 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h157, _, _, _) => {
                h157
            }
        };
        let a159 = 4u64;
        nat_eq(a158, a159)
    } else {
        false
    } {
        let a161 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h160, _, _, _) => {
                h160
            }
        };
        let a162 = 4u64;
        nat_eq(a161, a162)
    } else {
        false
    } {
        let a165 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h163, _, _, _) => {
                h163
            }
        };
        let a166 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h164, _, _) => {
                h164
            }
        };
        nat_le(a165, a166)
    } else {
        false
    } {
        let a169 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h167, _, _) => {
                h167
            }
        };
        let a170 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h168, _) => {
                h168
            }
        };
        nat_le(a169, a170)
    } else {
        false
    } {
        let a173 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h171, _) => {
                h171
            }
        };
        let a174 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h172) => {
                h172
            }
        };
        nat_le(a173, a174)
    } else {
        false
    } {
        let m189 = f5(v0, 0u64, match v2.clone() {
            Adt1::C0(h175, _, _, _, _, _, _, _, _) => {
                h175
            }
        }, match v3.clone() {
            Adt1::C0(h176, _, _, _, _, _, _, _, _) => {
                h176
            }
        }, match v1.clone() {
            Adt1::C0(_, h177, _, _, _, _, _, _, _) => {
                h177
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h178, _, _, _, _, _, _) => {
                h178
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h179, _, _, _, _, _) => {
                h179
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h180, _, _, _, _) => {
                h180
            }
        }, match v2.clone() {
            Adt1::C0(_, h181, _, _, _, _, _, _, _) => {
                h181
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h182, _, _, _, _, _, _) => {
                h182
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h183, _, _, _, _, _) => {
                h183
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h184, _, _, _, _) => {
                h184
            }
        }, match v3.clone() {
            Adt1::C0(_, h185, _, _, _, _, _, _, _) => {
                h185
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h186, _, _, _, _, _, _) => {
                h186
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h187, _, _, _, _, _) => {
                h187
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h188, _, _, _, _) => {
                h188
            }
        })?;
        match m189 {
            Adt6::C0 => {
                Ok(Adt3::C0)
            }
            Adt6::C1 => {
                Ok(Adt3::C0)
            }
            Adt6::C2(v6, _, v8, v9, v10, v11, v12, _, _, v15) => {
                Ok(Adt3::C1(Adt2::C0({
                    let a191 = match v1.clone() {
                        Adt1::C0(_, h190, _, _, _, _, _, _, _) => {
                            h190
                        }
                    };
                    let a192 = v4;
                    let a193 = 0u64;
                    nat_quot(a191, a192, a193)
                }, {
                    let a198 = {
                        let a195 = match v1.clone() {
                            Adt1::C0(_, h194, _, _, _, _, _, _, _) => {
                                h194
                            }
                        };
                        let a196 = v4;
                        let a197 = 0u64;
                        nat_quot(a195, a196, a197)
                    };
                    let a199 = v5;
                    nat_mul(a198, a199)?
                }, v6, v8, v9, {
                    let a202 = v8;
                    let a203 = {
                        let a200 = v9;
                        let a201 = v10;
                        nat_mul(a200, a201)?
                    };
                    nat_mul(a202, a203)?
                }, {
                    let a206 = v15;
                    let a207 = {
                        let a204 = v9;
                        let a205 = v10;
                        nat_mul(a204, a205)?
                    };
                    nat_mul(a206, a207)?
                }, v11, v12, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h208, _, _) => {
                        h208
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h209, _) => {
                        h209
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h210) => {
                        h210
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, h211, _, _) => {
                        h211
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h212, _) => {
                        h212
                    }
                }, match v2 {
                    Adt1::C0(_, _, _, _, _, _, _, _, h213) => {
                        h213
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h214, _, _) => {
                        h214
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h215, _) => {
                        h215
                    }
                }, match v3 {
                    Adt1::C0(_, _, _, _, _, _, _, _, h216) => {
                        h216
                    }
                }, match v1 {
                    Adt1::C0(h217, _, _, _, _, _, _, _, _) => {
                        h217
                    }
                })))
            }
        }
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f4(v0: u64) -> bool {
    let a218 = v0;
    let a219 = 2u64;
    if if nat_eq(a218, a219) {
        true
    } else {
        let a220 = v0;
        let a221 = 8u64;
        nat_eq(a220, a221)
    } {
        true
    } else {
        let a222 = v0;
        let a223 = 10u64;
        nat_eq(a222, a223)
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt6> {
    let a224 = v0;
    let a225 = 28u64;
    if if if if nat_eq(a224, a225) {
        let a226 = v1;
        let a227 = 0u64;
        nat_eq(a226, a227)
    } else {
        false
    } {
        let a228 = v2;
        let a229 = 0u64;
        nat_eq(a228, a229)
    } else {
        false
    } {
        let a230 = v3;
        let a231 = 0u64;
        nat_eq(a230, a231)
    } else {
        false
    } {
        let a232 = v4;
        let a233 = v8;
        if if if if if if if if nat_eq(a232, a233) {
            let a234 = v5;
            let a235 = v12;
            nat_eq(a234, a235)
        } else {
            false
        } {
            let a236 = v9;
            let a237 = v13;
            nat_eq(a236, a237)
        } else {
            false
        } {
            let a238 = v10;
            let a239 = v14;
            nat_eq(a238, a239)
        } else {
            false
        } {
            let a240 = v11;
            let a241 = v15;
            nat_eq(a240, a241)
        } else {
            false
        } {
            let a242 = 0u64;
            let a243 = v6;
            nat_lt(a242, a243)
        } else {
            false
        } {
            let a244 = 0u64;
            let a245 = v7;
            nat_lt(a244, a245)
        } else {
            false
        } {
            let a249 = {
                let a246 = v10;
                let a247 = v6;
                let a248 = 1u64;
                nat_rem(a246, a247, a248)
            };
            let a250 = 0u64;
            if nat_eq(a249, a250) {
                let a254 = {
                    let a251 = v11;
                    let a252 = v7;
                    let a253 = 1u64;
                    nat_rem(a251, a252, a253)
                };
                let a255 = 0u64;
                nat_eq(a254, a255)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt6::C2(v5, v4, v9, v10, v11, {
                let a256 = v10;
                let a257 = v6;
                let a258 = 0u64;
                nat_quot(a256, a257, a258)
            }, {
                let a259 = v11;
                let a260 = v7;
                let a261 = 0u64;
                nat_quot(a259, a260, a261)
            }, {
                let a262 = v5;
                let a263 = v4;
                nat_mul(a262, a263)?
            }, {
                let a264 = v4;
                let a265 = v9;
                nat_mul(a264, a265)?
            }, {
                let a266 = v5;
                let a267 = v9;
                nat_mul(a266, a267)?
            }))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}
