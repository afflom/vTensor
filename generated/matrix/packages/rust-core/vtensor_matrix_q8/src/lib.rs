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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, bool),
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
    let a148 = match v1.clone() {
        Adt1::C0(h147, _, _, _, _, _, _, _, _) => {
            h147
        }
    };
    let a149 = 8u64;
    if if if if if if if if if nat_eq(a148, a149) {
        true
    } else {
        let a151 = match v1.clone() {
            Adt1::C0(h150, _, _, _, _, _, _, _, _) => {
                h150
            }
        };
        let a152 = 2u64;
        nat_eq(a151, a152)
    } {
        let a157 = {
            let a154 = match v1.clone() {
                Adt1::C0(_, h153, _, _, _, _, _, _, _) => {
                    h153
                }
            };
            let a155 = v4;
            let a156 = 0u64;
            nat_rem(a154, a155, a156)
        };
        let a158 = 0u64;
        nat_eq(a157, a158)
    } else {
        false
    } {
        let a160 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h159, _, _, _) => {
                h159
            }
        };
        let a161 = v5;
        nat_eq(a160, a161)
    } else {
        false
    } {
        let a163 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h162, _, _, _) => {
                h162
            }
        };
        let a164 = 4u64;
        nat_eq(a163, a164)
    } else {
        false
    } {
        let a166 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h165, _, _, _) => {
                h165
            }
        };
        let a167 = 4u64;
        nat_eq(a166, a167)
    } else {
        false
    } {
        let a170 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h168, _, _, _) => {
                h168
            }
        };
        let a171 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h169, _, _) => {
                h169
            }
        };
        nat_le(a170, a171)
    } else {
        false
    } {
        let a174 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h172, _, _) => {
                h172
            }
        };
        let a175 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h173, _) => {
                h173
            }
        };
        nat_le(a174, a175)
    } else {
        false
    } {
        let a178 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h176, _) => {
                h176
            }
        };
        let a179 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h177) => {
                h177
            }
        };
        nat_le(a178, a179)
    } else {
        false
    } {
        let m194 = f4(v0, 0u64, match v2.clone() {
            Adt1::C0(h180, _, _, _, _, _, _, _, _) => {
                h180
            }
        }, match v3.clone() {
            Adt1::C0(h181, _, _, _, _, _, _, _, _) => {
                h181
            }
        }, match v1.clone() {
            Adt1::C0(_, h182, _, _, _, _, _, _, _) => {
                h182
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h183, _, _, _, _, _, _) => {
                h183
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h184, _, _, _, _, _) => {
                h184
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h185, _, _, _, _) => {
                h185
            }
        }, match v2.clone() {
            Adt1::C0(_, h186, _, _, _, _, _, _, _) => {
                h186
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h187, _, _, _, _, _, _) => {
                h187
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h188, _, _, _, _, _) => {
                h188
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h189, _, _, _, _) => {
                h189
            }
        }, match v3.clone() {
            Adt1::C0(_, h190, _, _, _, _, _, _, _) => {
                h190
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h191, _, _, _, _, _, _) => {
                h191
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h192, _, _, _, _, _) => {
                h192
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h193, _, _, _, _) => {
                h193
            }
        })?;
        match m194 {
            Adt6::C0 => {
                Ok(Adt3::C0)
            }
            Adt6::C1 => {
                Ok(Adt3::C0)
            }
            Adt6::C2(v6, _, v8, v9, v10, v11, v12, _, _, v15) => {
                Ok(Adt3::C1(Adt2::C0({
                    let a196 = match v1.clone() {
                        Adt1::C0(_, h195, _, _, _, _, _, _, _) => {
                            h195
                        }
                    };
                    let a197 = v4;
                    let a198 = 0u64;
                    nat_quot(a196, a197, a198)
                }, {
                    let a203 = {
                        let a200 = match v1.clone() {
                            Adt1::C0(_, h199, _, _, _, _, _, _, _) => {
                                h199
                            }
                        };
                        let a201 = v4;
                        let a202 = 0u64;
                        nat_quot(a200, a201, a202)
                    };
                    let a204 = v5;
                    nat_mul(a203, a204)?
                }, v6, v8, v9, {
                    let a207 = v8;
                    let a208 = {
                        let a205 = v9;
                        let a206 = v10;
                        nat_mul(a205, a206)?
                    };
                    nat_mul(a207, a208)?
                }, {
                    let a211 = v15;
                    let a212 = {
                        let a209 = v9;
                        let a210 = v10;
                        nat_mul(a209, a210)?
                    };
                    nat_mul(a211, a212)?
                }, v11, v12, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h213, _, _) => {
                        h213
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h214, _) => {
                        h214
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h215) => {
                        h215
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, h216, _, _) => {
                        h216
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h217, _) => {
                        h217
                    }
                }, match v2 {
                    Adt1::C0(_, _, _, _, _, _, _, _, h218) => {
                        h218
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h219, _, _) => {
                        h219
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h220, _) => {
                        h220
                    }
                }, match v3 {
                    Adt1::C0(_, _, _, _, _, _, _, _, h221) => {
                        h221
                    }
                }, {
                    let a223 = match v1 {
                        Adt1::C0(h222, _, _, _, _, _, _, _, _) => {
                            h222
                        }
                    };
                    let a224 = 2u64;
                    nat_eq(a223, a224)
                })))
            }
        }
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt6> {
    let a225 = v0;
    let a226 = 28u64;
    if if if if nat_eq(a225, a226) {
        let a227 = v1;
        let a228 = 0u64;
        nat_eq(a227, a228)
    } else {
        false
    } {
        let a229 = v2;
        let a230 = 0u64;
        nat_eq(a229, a230)
    } else {
        false
    } {
        let a231 = v3;
        let a232 = 0u64;
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a233 = v4;
        let a234 = v8;
        if if if if if if if if nat_eq(a233, a234) {
            let a235 = v5;
            let a236 = v12;
            nat_eq(a235, a236)
        } else {
            false
        } {
            let a237 = v9;
            let a238 = v13;
            nat_eq(a237, a238)
        } else {
            false
        } {
            let a239 = v10;
            let a240 = v14;
            nat_eq(a239, a240)
        } else {
            false
        } {
            let a241 = v11;
            let a242 = v15;
            nat_eq(a241, a242)
        } else {
            false
        } {
            let a243 = 0u64;
            let a244 = v6;
            nat_lt(a243, a244)
        } else {
            false
        } {
            let a245 = 0u64;
            let a246 = v7;
            nat_lt(a245, a246)
        } else {
            false
        } {
            let a250 = {
                let a247 = v10;
                let a248 = v6;
                let a249 = 1u64;
                nat_rem(a247, a248, a249)
            };
            let a251 = 0u64;
            if nat_eq(a250, a251) {
                let a255 = {
                    let a252 = v11;
                    let a253 = v7;
                    let a254 = 1u64;
                    nat_rem(a252, a253, a254)
                };
                let a256 = 0u64;
                nat_eq(a255, a256)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt6::C2(v5, v4, v9, v10, v11, {
                let a257 = v10;
                let a258 = v6;
                let a259 = 0u64;
                nat_quot(a257, a258, a259)
            }, {
                let a260 = v11;
                let a261 = v7;
                let a262 = 0u64;
                nat_quot(a260, a261, a262)
            }, {
                let a263 = v5;
                let a264 = v4;
                nat_mul(a263, a264)?
            }, {
                let a265 = v4;
                let a266 = v9;
                nat_mul(a265, a266)?
            }, {
                let a267 = v5;
                let a268 = v9;
                nat_mul(a267, a268)?
            }))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
