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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
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
                Adt2::C0(_, _, _, _, _, h2, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2
                }
            };
            if nat_lt(a3, a4) {
                let v8: u64 = {
                    let a6 = v7;
                    let a7 = match v6.clone() {
                        Adt2::C0(_, _, _, h5, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h5
                        }
                    };
                    let a8 = 0u64;
                    nat_rem(a6, a7, a8)
                };
                let v9: u64 = {
                    let a10 = v7;
                    let a11 = match v6.clone() {
                        Adt2::C0(_, _, _, h9, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h9
                        }
                    };
                    let a12 = 0u64;
                    nat_quot(a10, a11, a12)
                };
                let v10: u64 = {
                    let a14 = v9;
                    let a15 = match v6.clone() {
                        Adt2::C0(_, _, _, _, h13, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h13
                        }
                    };
                    let a16 = 0u64;
                    nat_rem(a14, a15, a16)
                };
                let v11: u64 = {
                    let a18 = v9;
                    let a19 = match v6.clone() {
                        Adt2::C0(_, _, _, _, h17, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
                                Adt2::C0(_, _, _, _, _, _, _, h21, _, _, _, _, _, _, _, _, _, _) => {
                                    h21
                                }
                            };
                            let a24 = 0u64;
                            nat_quot(a22, a23, a24)
                        };
                        let a27 = match v6.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h25, _, _, _, _, _, _, _) => {
                                h25
                            }
                        };
                        nat_mul(a26, a27)?
                    };
                    let a36 = {
                        let a33 = {
                            let a29 = v11;
                            let a30 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h28, _, _, _, _, _, _, _, _, _) => {
                                    h28
                                }
                            };
                            let a31 = 0u64;
                            nat_quot(a29, a30, a31)
                        };
                        let a34 = match v6.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h32, _, _, _, _, _, _) => {
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
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h37, _, _, _, _, _) => {
                                    h37
                                }
                            };
                            nat_mul(a38, a39)?
                        };
                        let a44 = {
                            let a41 = v10;
                            let a42 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h40, _, _, _, _) => {
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
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h45, _, _, _) => {
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
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h50, _, _) => {
                                    h50
                                }
                            };
                            nat_mul(a51, a52)?
                        };
                        let a57 = {
                            let a54 = v10;
                            let a55 = match v6.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h53, _) => {
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
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h58) => {
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
                Adt2::C0(_, _, h63, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h63
                }
            };
            if nat_lt(a64, a65) {
                Ok(Adt3::C3({
                    let a69 = v13;
                    let a70 = {
                        let a67 = v14;
                        let a68 = match v12 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h66, _, _, _, _, _, _, _, _) => {
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
    if if if if if if if if if !v1 {
        let a72 = match v2.clone() {
            Adt1::C0(h71, _, _, _, _, _, _, _, _) => {
                h71
            }
        };
        let a73 = 8u64;
        nat_eq(a72, a73)
    } else {
        false
    } {
        let a78 = {
            let a75 = match v2.clone() {
                Adt1::C0(_, h74, _, _, _, _, _, _, _) => {
                    h74
                }
            };
            let a76 = 32u64;
            let a77 = 0u64;
            nat_rem(a75, a76, a77)
        };
        let a79 = 0u64;
        nat_eq(a78, a79)
    } else {
        false
    } {
        let a81 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h80, _, _, _) => {
                h80
            }
        };
        let a82 = 34u64;
        nat_eq(a81, a82)
    } else {
        false
    } {
        let a84 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h83, _, _, _) => {
                h83
            }
        };
        let a85 = 4u64;
        nat_eq(a84, a85)
    } else {
        false
    } {
        let a87 = match v4.clone() {
            Adt1::C0(_, _, _, _, _, h86, _, _, _) => {
                h86
            }
        };
        let a88 = 4u64;
        nat_eq(a87, a88)
    } else {
        false
    } {
        let a91 = match v4.clone() {
            Adt1::C0(_, _, _, _, _, h89, _, _, _) => {
                h89
            }
        };
        let a92 = match v4.clone() {
            Adt1::C0(_, _, _, _, _, _, h90, _, _) => {
                h90
            }
        };
        nat_le(a91, a92)
    } else {
        false
    } {
        let a95 = match v4.clone() {
            Adt1::C0(_, _, _, _, _, _, h93, _, _) => {
                h93
            }
        };
        let a96 = match v4.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h94, _) => {
                h94
            }
        };
        nat_le(a95, a96)
    } else {
        false
    } {
        let a99 = match v4.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h97, _) => {
                h97
            }
        };
        let a100 = match v4.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h98) => {
                h98
            }
        };
        nat_le(a99, a100)
    } else {
        false
    } {
        let m115 = f2(v0, 0u64, match v3.clone() {
            Adt1::C0(h101, _, _, _, _, _, _, _, _) => {
                h101
            }
        }, match v4.clone() {
            Adt1::C0(h102, _, _, _, _, _, _, _, _) => {
                h102
            }
        }, match v2.clone() {
            Adt1::C0(_, h103, _, _, _, _, _, _, _) => {
                h103
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h104, _, _, _, _, _, _) => {
                h104
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h105, _, _, _, _, _) => {
                h105
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h106, _, _, _, _) => {
                h106
            }
        }, match v3.clone() {
            Adt1::C0(_, h107, _, _, _, _, _, _, _) => {
                h107
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h108, _, _, _, _, _, _) => {
                h108
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h109, _, _, _, _, _) => {
                h109
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h110, _, _, _, _) => {
                h110
            }
        }, match v4.clone() {
            Adt1::C0(_, h111, _, _, _, _, _, _, _) => {
                h111
            }
        }, match v4.clone() {
            Adt1::C0(_, _, h112, _, _, _, _, _, _) => {
                h112
            }
        }, match v4.clone() {
            Adt1::C0(_, _, _, h113, _, _, _, _, _) => {
                h113
            }
        }, match v4.clone() {
            Adt1::C0(_, _, _, _, h114, _, _, _, _) => {
                h114
            }
        })?;
        match m115 {
            Adt4::C0 => {
                Ok(Adt3::C0)
            }
            Adt4::C1 => {
                Ok(Adt3::C0)
            }
            Adt4::C2(v5, _, v7, v8, v9, v10, v11, _, _, v14) => {
                Ok(Adt3::C1(Adt2::C0({
                    let a117 = match v2.clone() {
                        Adt1::C0(_, h116, _, _, _, _, _, _, _) => {
                            h116
                        }
                    };
                    let a118 = 32u64;
                    let a119 = 0u64;
                    nat_quot(a117, a118, a119)
                }, {
                    let a124 = {
                        let a121 = match v2.clone() {
                            Adt1::C0(_, h120, _, _, _, _, _, _, _) => {
                                h120
                            }
                        };
                        let a122 = 32u64;
                        let a123 = 0u64;
                        nat_quot(a121, a122, a123)
                    };
                    let a125 = 34u64;
                    nat_mul(a124, a125)?
                }, v5, v7, v8, {
                    let a128 = v7;
                    let a129 = {
                        let a126 = v8;
                        let a127 = v9;
                        nat_mul(a126, a127)?
                    };
                    nat_mul(a128, a129)?
                }, {
                    let a132 = v14;
                    let a133 = {
                        let a130 = v8;
                        let a131 = v9;
                        nat_mul(a130, a131)?
                    };
                    nat_mul(a132, a133)?
                }, v10, v11, match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, h134, _, _) => {
                        h134
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h135, _) => {
                        h135
                    }
                }, match v2 {
                    Adt1::C0(_, _, _, _, _, _, _, _, h136) => {
                        h136
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h137, _, _) => {
                        h137
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h138, _) => {
                        h138
                    }
                }, match v3 {
                    Adt1::C0(_, _, _, _, _, _, _, _, h139) => {
                        h139
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, _, _, h140, _, _) => {
                        h140
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h141, _) => {
                        h141
                    }
                }, match v4 {
                    Adt1::C0(_, _, _, _, _, _, _, _, h142) => {
                        h142
                    }
                })))
            }
        }
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt4> {
    let a143 = v0;
    let a144 = 28u64;
    if if if if nat_eq(a143, a144) {
        let a145 = v1;
        let a146 = 0u64;
        nat_eq(a145, a146)
    } else {
        false
    } {
        let a147 = v2;
        let a148 = 0u64;
        nat_eq(a147, a148)
    } else {
        false
    } {
        let a149 = v3;
        let a150 = 0u64;
        nat_eq(a149, a150)
    } else {
        false
    } {
        let a151 = v4;
        let a152 = v8;
        if if if if if if if if nat_eq(a151, a152) {
            let a153 = v5;
            let a154 = v12;
            nat_eq(a153, a154)
        } else {
            false
        } {
            let a155 = v9;
            let a156 = v13;
            nat_eq(a155, a156)
        } else {
            false
        } {
            let a157 = v10;
            let a158 = v14;
            nat_eq(a157, a158)
        } else {
            false
        } {
            let a159 = v11;
            let a160 = v15;
            nat_eq(a159, a160)
        } else {
            false
        } {
            let a161 = 0u64;
            let a162 = v6;
            nat_lt(a161, a162)
        } else {
            false
        } {
            let a163 = 0u64;
            let a164 = v7;
            nat_lt(a163, a164)
        } else {
            false
        } {
            let a168 = {
                let a165 = v10;
                let a166 = v6;
                let a167 = 1u64;
                nat_rem(a165, a166, a167)
            };
            let a169 = 0u64;
            if nat_eq(a168, a169) {
                let a173 = {
                    let a170 = v11;
                    let a171 = v7;
                    let a172 = 1u64;
                    nat_rem(a170, a171, a172)
                };
                let a174 = 0u64;
                nat_eq(a173, a174)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt4::C2(v5, v4, v9, v10, v11, {
                let a175 = v10;
                let a176 = v6;
                let a177 = 0u64;
                nat_quot(a175, a176, a177)
            }, {
                let a178 = v11;
                let a179 = v7;
                let a180 = 0u64;
                nat_quot(a178, a179, a180)
            }, {
                let a181 = v5;
                let a182 = v4;
                nat_mul(a181, a182)?
            }, {
                let a183 = v4;
                let a184 = v9;
                nat_mul(a183, a184)?
            }, {
                let a185 = v5;
                let a186 = v9;
                nat_mul(a185, a186)?
            }))
        } else {
            Ok(Adt4::C1)
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
