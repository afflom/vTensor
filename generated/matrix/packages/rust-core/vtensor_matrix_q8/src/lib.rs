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
    C3(u64, bool, Adt1, Adt1, Adt1, Adt1),
    C4(Adt2, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, Option<Adt3>, Option<u64>, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Adt2),
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
    C5(u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt4> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5) => {
            f1(v1, v2, v3, v4, v5)
        }
        Adt0::C1(v6, v7) => {
            let m3 = match v6.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2, _, _) => {
                    h2
                }
            };
            match m3 {
                None => {
                    f2(v6, v7)
                }
                Some(v8) => {
                    let a5 = v7;
                    let a6 = match v6.clone() {
                        Adt2::C0(_, _, _, _, _, h4, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4
                        }
                    };
                    if nat_lt(a5, a6) {
                        Ok(Adt4::C4({
                            let a21 = {
                                let a12 = {
                                    let a8 = v7;
                                    let a9 = match v6.clone() {
                                        Adt2::C0(_, _, _, h7, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h7
                                        }
                                    };
                                    let a10 = 0u64;
                                    nat_rem(a8, a9, a10)
                                };
                                let a13 = match v8.clone() {
                                    Adt3::C0(_, _, h11, _) => {
                                        h11
                                    }
                                };
                                nat_mul(a12, a13)?
                            };
                            let a22 = {
                                let a19 = {
                                    let a15 = v7;
                                    let a16 = match v6 {
                                        Adt2::C0(_, _, _, h14, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h14
                                        }
                                    };
                                    let a17 = 0u64;
                                    nat_quot(a15, a16, a17)
                                };
                                let a20 = match v8 {
                                    Adt3::C0(_, _, _, h18) => {
                                        h18
                                    }
                                };
                                nat_mul(a19, a20)?
                            };
                            nat_add(a21, a22)?
                        }))
                    } else {
                        Ok(Adt4::C0)
                    }
                }
            }
        }
        Adt0::C2(v18, v19, v20) => {
            let a24 = v20;
            let a25 = match v18.clone() {
                Adt2::C0(_, _, h23, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h23
                }
            };
            if nat_lt(a24, a25) {
                Ok(Adt4::C3({
                    let a29 = v19;
                    let a30 = {
                        let a27 = v20;
                        let a28 = match v18 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h26, _, _, _, _, _, _, _, _, _, _, _) => {
                                h26
                            }
                        };
                        nat_mul(a27, a28)?
                    };
                    nat_add(a29, a30)?
                }))
            } else {
                Ok(Adt4::C0)
            }
        }
        Adt0::C3(v9, v10, v11, v12, v13, v14) => {
            f3(v9, v10, v11, v12, v13, v14)
        }
        Adt0::C4(v15, v16, v17) => {
            f4(v15, v16, v17)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1) -> R<Adt4> {
    if v1 {
        Ok(Adt4::C0)
    } else {
        let m32 = f5(match v2.clone() {
            Adt1::C0(h31, _, _, _, _, _, _, _, _) => {
                h31
            }
        });
        match m32 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v5) => {
                f6(v0, v2, v3, v4, match v5.clone() {
                    Adt6::C0(_, h33, _) => {
                        h33
                    }
                }, match v5 {
                    Adt6::C0(_, _, h34) => {
                        h34
                    }
                })
            }
        }
    }
}

pub fn f2(v0: Adt2, v1: u64) -> R<Adt4> {
    let a36 = v1;
    let a37 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h35, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h35
        }
    };
    if nat_lt(a36, a37) {
        let v2: u64 = {
            let a39 = v1;
            let a40 = match v0.clone() {
                Adt2::C0(_, _, _, h38, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h38
                }
            };
            let a41 = 0u64;
            nat_rem(a39, a40, a41)
        };
        let v3: u64 = {
            let a43 = v1;
            let a44 = match v0.clone() {
                Adt2::C0(_, _, _, h42, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h42
                }
            };
            let a45 = 0u64;
            nat_quot(a43, a44, a45)
        };
        let v4: u64 = {
            let a47 = v3;
            let a48 = match v0.clone() {
                Adt2::C0(_, _, _, _, h46, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h46
                }
            };
            let a49 = 0u64;
            nat_rem(a47, a48, a49)
        };
        let v5: u64 = {
            let a51 = v3;
            let a52 = match v0.clone() {
                Adt2::C0(_, _, _, _, h50, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h50
                }
            };
            let a53 = 0u64;
            nat_quot(a51, a52, a53)
        };
        Ok(Adt4::C2({
            let a68 = {
                let a59 = {
                    let a55 = v4;
                    let a56 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h54, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h54
                        }
                    };
                    let a57 = 0u64;
                    nat_quot(a55, a56, a57)
                };
                let a60 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h58, _, _, _, _, _, _, _, _, _, _) => {
                        h58
                    }
                };
                nat_mul(a59, a60)?
            };
            let a69 = {
                let a66 = {
                    let a62 = v5;
                    let a63 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h61, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h61
                        }
                    };
                    let a64 = 0u64;
                    nat_quot(a62, a63, a64)
                };
                let a67 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h65, _, _, _, _, _, _, _, _, _) => {
                        h65
                    }
                };
                nat_mul(a66, a67)?
            };
            nat_add(a68, a69)?
        }, {
            let a81 = {
                let a76 = {
                    let a71 = v2;
                    let a72 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h70, _, _, _, _, _, _, _, _) => {
                            h70
                        }
                    };
                    nat_mul(a71, a72)?
                };
                let a77 = {
                    let a74 = v4;
                    let a75 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h73, _, _, _, _, _, _, _) => {
                            h73
                        }
                    };
                    nat_mul(a74, a75)?
                };
                nat_add(a76, a77)?
            };
            let a82 = {
                let a79 = v5;
                let a80 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h78, _, _, _, _, _, _) => {
                        h78
                    }
                };
                nat_mul(a79, a80)?
            };
            nat_add(a81, a82)?
        }, {
            let a94 = {
                let a89 = {
                    let a84 = v2;
                    let a85 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h83, _, _, _, _, _) => {
                            h83
                        }
                    };
                    nat_mul(a84, a85)?
                };
                let a90 = {
                    let a87 = v4;
                    let a88 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h86, _, _, _, _) => {
                            h86
                        }
                    };
                    nat_mul(a87, a88)?
                };
                nat_add(a89, a90)?
            };
            let a95 = {
                let a92 = v5;
                let a93 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h91, _, _, _) => {
                        h91
                    }
                };
                nat_mul(a92, a93)?
            };
            nat_add(a94, a95)?
        }))
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f3(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Adt4> {
    let a96 = v0;
    let a97 = 29u64;
    if if if if if if if if if if if if if if if if if if if if nat_eq(a96, a97) {
        !v1
    } else {
        false
    } {
        let a99 = match v4.clone() {
            Adt1::C0(h98, _, _, _, _, _, _, _, _) => {
                h98
            }
        };
        let a100 = 26u64;
        nat_eq(a99, a100)
    } else {
        false
    } {
        let a102 = match v3.clone() {
            Adt1::C0(h101, _, _, _, _, _, _, _, _) => {
                h101
            }
        };
        let a103 = 0u64;
        nat_eq(a102, a103)
    } else {
        false
    } {
        let a105 = match v5.clone() {
            Adt1::C0(h104, _, _, _, _, _, _, _, _) => {
                h104
            }
        };
        let a106 = 0u64;
        nat_eq(a105, a106)
    } else {
        false
    } {
        let a108 = match v2.clone() {
            Adt1::C0(_, _, _, _, h107, _, _, _, _) => {
                h107
            }
        };
        let a109 = 1u64;
        nat_eq(a108, a109)
    } else {
        false
    } {
        let a111 = match v3.clone() {
            Adt1::C0(_, _, _, _, h110, _, _, _, _) => {
                h110
            }
        };
        let a112 = 1u64;
        nat_eq(a111, a112)
    } else {
        false
    } {
        let a114 = match v4.clone() {
            Adt1::C0(_, _, _, _, h113, _, _, _, _) => {
                h113
            }
        };
        let a115 = 1u64;
        nat_eq(a114, a115)
    } else {
        false
    } {
        let a117 = match v5.clone() {
            Adt1::C0(_, _, _, _, h116, _, _, _, _) => {
                h116
            }
        };
        let a118 = 1u64;
        nat_eq(a117, a118)
    } else {
        false
    } {
        let a120 = match v4.clone() {
            Adt1::C0(_, _, _, h119, _, _, _, _, _) => {
                h119
            }
        };
        let a121 = 1u64;
        nat_eq(a120, a121)
    } else {
        false
    } {
        let a124 = match v4.clone() {
            Adt1::C0(_, _, h122, _, _, _, _, _, _) => {
                h122
            }
        };
        let a125 = match v3.clone() {
            Adt1::C0(_, _, _, h123, _, _, _, _, _) => {
                h123
            }
        };
        nat_eq(a124, a125)
    } else {
        false
    } {
        let a128 = match v2.clone() {
            Adt1::C0(_, h126, _, _, _, _, _, _, _) => {
                h126
            }
        };
        let a129 = match v3.clone() {
            Adt1::C0(_, h127, _, _, _, _, _, _, _) => {
                h127
            }
        };
        nat_eq(a128, a129)
    } else {
        false
    } {
        let a131 = 0u64;
        let a132 = match v3.clone() {
            Adt1::C0(_, _, h130, _, _, _, _, _, _) => {
                h130
            }
        };
        nat_lt(a131, a132)
    } else {
        false
    } {
        let a138 = {
            let a135 = match v4.clone() {
                Adt1::C0(_, h133, _, _, _, _, _, _, _) => {
                    h133
                }
            };
            let a136 = match v3.clone() {
                Adt1::C0(_, _, h134, _, _, _, _, _, _) => {
                    h134
                }
            };
            let a137 = 0u64;
            nat_rem(a135, a136, a137)
        };
        let a139 = 0u64;
        nat_eq(a138, a139)
    } else {
        false
    } {
        let a142 = match v5.clone() {
            Adt1::C0(_, h140, _, _, _, _, _, _, _) => {
                h140
            }
        };
        let a143 = match v2.clone() {
            Adt1::C0(_, _, h141, _, _, _, _, _, _) => {
                h141
            }
        };
        nat_eq(a142, a143)
    } else {
        false
    } {
        let a146 = match v5.clone() {
            Adt1::C0(_, _, h144, _, _, _, _, _, _) => {
                h144
            }
        };
        let a147 = match v4.clone() {
            Adt1::C0(_, h145, _, _, _, _, _, _, _) => {
                h145
            }
        };
        nat_eq(a146, a147)
    } else {
        false
    } {
        let a150 = match v5.clone() {
            Adt1::C0(_, _, _, h148, _, _, _, _, _) => {
                h148
            }
        };
        let a151 = match v3.clone() {
            Adt1::C0(_, _, _, h149, _, _, _, _, _) => {
                h149
            }
        };
        nat_eq(a150, a151)
    } else {
        false
    } {
        let a154 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h152, _, _, _) => {
                h152
            }
        };
        let a155 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h153, _, _) => {
                h153
            }
        };
        nat_le(a154, a155)
    } else {
        false
    } {
        let a160 = {
            let a157 = match v5.clone() {
                Adt1::C0(_, h156, _, _, _, _, _, _, _) => {
                    h156
                }
            };
            let a158 = 4u64;
            nat_mul(a157, a158)?
        };
        let a161 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, h159, _, _) => {
                h159
            }
        };
        nat_le(a160, a161)
    } else {
        false
    } {
        let a167 = {
            let a164 = match v5.clone() {
                Adt1::C0(_, _, h162, _, _, _, _, _, _) => {
                    h162
                }
            };
            let a165 = match v5.clone() {
                Adt1::C0(_, _, _, _, _, _, h163, _, _) => {
                    h163
                }
            };
            nat_mul(a164, a165)?
        };
        let a168 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h166, _) => {
                h166
            }
        };
        nat_le(a167, a168)
    } else {
        false
    } {
        let m170 = f5(match v2.clone() {
            Adt1::C0(h169, _, _, _, _, _, _, _, _) => {
                h169
            }
        });
        match m170 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v6) => {
                if f7(v2.clone(), v3.clone(), v5.clone(), true, match v6.clone() {
                    Adt6::C0(_, h171, _) => {
                        h171
                    }
                }, match v6.clone() {
                    Adt6::C0(_, _, h172) => {
                        h172
                    }
                }) {
                    Ok(Adt4::C1(f8(v2.clone(), v3.clone(), v5, match v6.clone() {
                        Adt6::C0(_, h173, _) => {
                            h173
                        }
                    }, match v6 {
                        Adt6::C0(_, _, h174) => {
                            h174
                        }
                    }, match v2.clone() {
                        Adt1::C0(_, _, h175, _, _, _, _, _, _) => {
                            h175
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h176, _, _, _, _, _, _, _) => {
                            h176
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h177, _, _, _, _, _, _) => {
                            h177
                        }
                    }, 1u64, 1u64, 1u64, Some(Adt3::C0(match v2 {
                        Adt1::C0(_, _, _, h178, _, _, _, _, _) => {
                            h178
                        }
                    }, match v3 {
                        Adt1::C0(_, _, h179, _, _, _, _, _, _) => {
                            h179
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h180, _, _, _) => {
                            h180
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h181, _, _) => {
                            h181
                        }
                    })))?))
                } else {
                    Ok(Adt4::C0)
                }
            }
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f4(v0: Adt2, v1: u64, v2: u64) -> R<Adt4> {
    let m183 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h182, _, _) => {
            h182
        }
    };
    match m183 {
        None => {
            Ok(Adt4::C0)
        }
        Some(v3) => {
            let a185 = v1;
            let a186 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h184, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h184
                }
            };
            if nat_lt(a185, a186) {
                let v4: u64 = {
                    let a188 = v1;
                    let a189 = match v0.clone() {
                        Adt2::C0(_, _, _, h187, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h187
                        }
                    };
                    let a190 = 0u64;
                    nat_rem(a188, a189, a190)
                };
                let v5: u64 = {
                    let a192 = v1;
                    let a193 = match v0.clone() {
                        Adt2::C0(_, _, _, h191, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h191
                        }
                    };
                    let a194 = 0u64;
                    nat_quot(a192, a193, a194)
                };
                let v6: u64 = {
                    let a201 = {
                        let a196 = v4;
                        let a197 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h195, _, _, _, _, _) => {
                                h195
                            }
                        };
                        nat_mul(a196, a197)?
                    };
                    let a202 = {
                        let a199 = v5;
                        let a200 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h198, _, _, _, _) => {
                                h198
                            }
                        };
                        nat_mul(a199, a200)?
                    };
                    nat_add(a201, a202)?
                };
                let a203 = v2;
                let a204 = 4294967295u64;
                if nat_eq(a203, a204) {
                    Ok(Adt4::C5(v6))
                } else {
                    let a206 = v2;
                    let a207 = match v3.clone() {
                        Adt3::C0(h205, _, _, _) => {
                            h205
                        }
                    };
                    if nat_lt(a206, a207) {
                        Ok(Adt4::C2({
                            let a209 = v2;
                            let a210 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h208, _, _, _, _, _, _, _, _, _, _) => {
                                    h208
                                }
                            };
                            nat_mul(a209, a210)?
                        }, {
                            let a221 = {
                                let a216 = {
                                    let a212 = v4;
                                    let a213 = match v3 {
                                        Adt3::C0(_, h211, _, _) => {
                                            h211
                                        }
                                    };
                                    let a214 = 0u64;
                                    nat_rem(a212, a213, a214)
                                };
                                let a217 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h215, _, _, _, _, _, _, _, _) => {
                                        h215
                                    }
                                };
                                nat_mul(a216, a217)?
                            };
                            let a222 = {
                                let a219 = v5;
                                let a220 = match v0 {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h218, _, _, _, _, _, _, _) => {
                                        h218
                                    }
                                };
                                nat_mul(a219, a220)?
                            };
                            nat_add(a221, a222)?
                        }, v6))
                    } else {
                        Ok(Adt4::C0)
                    }
                }
            } else {
                Ok(Adt4::C0)
            }
        }
    }
}

pub fn f5(v0: u64) -> Adt5 {
    let a223 = v0;
    let a224 = 0u64;
    if nat_eq(a223, a224) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a225 = v0;
        let a226 = 1u64;
        if nat_eq(a225, a226) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a227 = v0;
            let a228 = 2u64;
            if nat_eq(a227, a228) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a229 = v0;
                let a230 = 3u64;
                if nat_eq(a229, a230) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a231 = v0;
                    let a232 = 6u64;
                    if nat_eq(a231, a232) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a233 = v0;
                        let a234 = 7u64;
                        if nat_eq(a233, a234) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a235 = v0;
                            let a236 = 8u64;
                            if nat_eq(a235, a236) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a237 = v0;
                                let a238 = 9u64;
                                if nat_eq(a237, a238) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a239 = v0;
                                    let a240 = 10u64;
                                    if nat_eq(a239, a240) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a241 = v0;
                                        let a242 = 11u64;
                                        if nat_eq(a241, a242) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a243 = v0;
                                            let a244 = 12u64;
                                            if nat_eq(a243, a244) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a245 = v0;
                                                let a246 = 13u64;
                                                if nat_eq(a245, a246) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a247 = v0;
                                                    let a248 = 14u64;
                                                    if nat_eq(a247, a248) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a249 = v0;
                                                        let a250 = 15u64;
                                                        if nat_eq(a249, a250) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a251 = v0;
                                                            let a252 = 16u64;
                                                            if nat_eq(a251, a252) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a253 = v0;
                                                                let a254 = 17u64;
                                                                if nat_eq(a253, a254) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a255 = v0;
                                                                    let a256 = 18u64;
                                                                    if nat_eq(a255, a256) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a257 = v0;
                                                                        let a258 = 19u64;
                                                                        if nat_eq(a257, a258) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a259 = v0;
                                                                            let a260 = 20u64;
                                                                            if nat_eq(a259, a260) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a261 = v0;
                                                                                let a262 = 21u64;
                                                                                if nat_eq(a261, a262) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a263 = v0;
                                                                                    let a264 = 22u64;
                                                                                    if nat_eq(a263, a264) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a265 = v0;
                                                                                        let a266 = 23u64;
                                                                                        if nat_eq(a265, a266) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a267 = v0;
                                                                                            let a268 = 24u64;
                                                                                            if nat_eq(a267, a268) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a269 = v0;
                                                                                                let a270 = 25u64;
                                                                                                if nat_eq(a269, a270) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a271 = v0;
                                                                                                    let a272 = 26u64;
                                                                                                    if nat_eq(a271, a272) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a273 = v0;
                                                                                                        let a274 = 27u64;
                                                                                                        if nat_eq(a273, a274) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a275 = v0;
                                                                                                            let a276 = 28u64;
                                                                                                            if nat_eq(a275, a276) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a277 = v0;
                                                                                                                let a278 = 29u64;
                                                                                                                if nat_eq(a277, a278) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a279 = v0;
                                                                                                                    let a280 = 30u64;
                                                                                                                    if nat_eq(a279, a280) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a281 = v0;
                                                                                                                        let a282 = 34u64;
                                                                                                                        if nat_eq(a281, a282) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a283 = v0;
                                                                                                                            let a284 = 35u64;
                                                                                                                            if nat_eq(a283, a284) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a285 = v0;
                                                                                                                                let a286 = 39u64;
                                                                                                                                if nat_eq(a285, a286) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a287 = v0;
                                                                                                                                    let a288 = 40u64;
                                                                                                                                    if nat_eq(a287, a288) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a289 = v0;
                                                                                                                                        let a290 = 41u64;
                                                                                                                                        if nat_eq(a289, a290) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a291 = v0;
                                                                                                                                            let a292 = 42u64;
                                                                                                                                            if nat_eq(a291, a292) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a293 = v0;
                                                                                                                                                let a294 = 43u64;
                                                                                                                                                if nat_eq(a293, a294) {
                                                                                                                                                    Adt5::C1(Adt6::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt5::C0
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

pub fn f6(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64, v5: u64) -> R<Adt4> {
    if f7(v1.clone(), v2.clone(), v3.clone(), false, v4, v5) {
        let m308 = f9(v0, 0u64, 0u64, match v3.clone() {
            Adt1::C0(h295, _, _, _, _, _, _, _, _) => {
                h295
            }
        }, match v1.clone() {
            Adt1::C0(_, h296, _, _, _, _, _, _, _) => {
                h296
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h297, _, _, _, _, _, _) => {
                h297
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h298, _, _, _, _, _) => {
                h298
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h299, _, _, _, _) => {
                h299
            }
        }, match v2.clone() {
            Adt1::C0(_, h300, _, _, _, _, _, _, _) => {
                h300
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h301, _, _, _, _, _, _) => {
                h301
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h302, _, _, _, _, _) => {
                h302
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h303, _, _, _, _) => {
                h303
            }
        }, match v3.clone() {
            Adt1::C0(_, h304, _, _, _, _, _, _, _) => {
                h304
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h305, _, _, _, _, _, _) => {
                h305
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h306, _, _, _, _, _) => {
                h306
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h307, _, _, _, _) => {
                h307
            }
        })?;
        match m308 {
            Adt7::C0 => {
                Ok(Adt4::C0)
            }
            Adt7::C1 => {
                Ok(Adt4::C0)
            }
            Adt7::C2(v6, _, v8, v9, v10, v11, v12, _, _, _) => {
                Ok(Adt4::C1(f8(v1, v2, v3, v4, v5, v6, v8, v9, v10, v11, v12, None::<Adt3>)?))
            }
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f7(v0: Adt1, v1: Adt1, v2: Adt1, v3: bool, v4: u64, v5: u64) -> bool {
    if if if if if if if if f10(match v0.clone() {
        Adt1::C0(h309, _, _, _, _, _, _, _, _) => {
            h309
        }
    }) {
        true
    } else {
        if v3 {
            if f11(match v0.clone() {
                Adt1::C0(h310, _, _, _, _, _, _, _, _) => {
                    h310
                }
            }) {
                true
            } else {
                let a312 = match v0.clone() {
                    Adt1::C0(h311, _, _, _, _, _, _, _, _) => {
                        h311
                    }
                };
                let a313 = 43u64;
                nat_eq(a312, a313)
            }
        } else {
            false
        }
    } {
        let a318 = {
            let a315 = match v0.clone() {
                Adt1::C0(_, h314, _, _, _, _, _, _, _) => {
                    h314
                }
            };
            let a316 = v4;
            let a317 = 0u64;
            nat_rem(a315, a316, a317)
        };
        let a319 = 0u64;
        nat_eq(a318, a319)
    } else {
        false
    } {
        let a321 = match v0 {
            Adt1::C0(_, _, _, _, _, h320, _, _, _) => {
                h320
            }
        };
        let a322 = v5;
        nat_eq(a321, a322)
    } else {
        false
    } {
        f12(match v1.clone() {
            Adt1::C0(h323, _, _, _, _, _, _, _, _) => {
                h323
            }
        }, match v1 {
            Adt1::C0(_, _, _, _, _, h324, _, _, _) => {
                h324
            }
        })
    } else {
        false
    } {
        let a326 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h325, _, _, _) => {
                h325
            }
        };
        let a327 = 4u64;
        nat_eq(a326, a327)
    } else {
        false
    } {
        let a330 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h328, _, _, _) => {
                h328
            }
        };
        let a331 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h329, _, _) => {
                h329
            }
        };
        nat_le(a330, a331)
    } else {
        false
    } {
        let a334 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h332, _, _) => {
                h332
            }
        };
        let a335 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h333, _) => {
                h333
            }
        };
        nat_le(a334, a335)
    } else {
        false
    } {
        let a338 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h336, _) => {
                h336
            }
        };
        let a339 = match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h337) => {
                h337
            }
        };
        nat_le(a338, a339)
    } else {
        false
    }
}

pub fn f8(v0: Adt1, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: Option<Adt3>) -> R<Adt2> {
    Ok(Adt2::C0({
        let a342 = match v0.clone() {
            Adt1::C0(_, h340, _, _, _, _, _, _, _) => {
                h340
            }
        };
        let a343 = if f13(match v0.clone() {
            Adt1::C0(h341, _, _, _, _, _, _, _, _) => {
                h341
            }
        }) {
            32u64
        } else {
            v3
        };
        let a344 = 0u64;
        nat_quot(a342, a343, a344)
    }, {
        let a349 = {
            let a346 = match v0.clone() {
                Adt1::C0(_, h345, _, _, _, _, _, _, _) => {
                    h345
                }
            };
            let a347 = v3;
            let a348 = 0u64;
            nat_quot(a346, a347, a348)
        };
        let a350 = v4;
        nat_mul(a349, a350)?
    }, v5, v6, v7, {
        let a353 = v6;
        let a354 = {
            let a351 = v7;
            let a352 = v8;
            nat_mul(a351, a352)?
        };
        nat_mul(a353, a354)?
    }, {
        let a359 = v5;
        let a360 = {
            let a357 = v6;
            let a358 = {
                let a355 = v7;
                let a356 = v8;
                nat_mul(a355, a356)?
            };
            nat_mul(a357, a358)?
        };
        nat_mul(a359, a360)?
    }, v9, v10, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h361, _, _) => {
            h361
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h362, _) => {
            h362
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h363) => {
            h363
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h364, _, _) => {
            h364
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h365, _) => {
            h365
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h366) => {
            h366
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h367, _, _) => {
            h367
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h368, _) => {
            h368
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h369) => {
            h369
        }
    }, v11, if if f11(match v0.clone() {
        Adt1::C0(h370, _, _, _, _, _, _, _, _) => {
            h370
        }
    }) {
        true
    } else {
        let a372 = match v0.clone() {
            Adt1::C0(h371, _, _, _, _, _, _, _, _) => {
                h371
            }
        };
        let a373 = 43u64;
        nat_eq(a372, a373)
    } {
        Some(f14(match v0.clone() {
            Adt1::C0(h374, _, _, _, _, _, _, _, _) => {
                h374
            }
        }))
    } else {
        None::<u64>
    }, match v0 {
        Adt1::C0(h375, _, _, _, _, _, _, _, _) => {
            h375
        }
    }))
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt7> {
    let a376 = v0;
    let a377 = 28u64;
    if if if if nat_eq(a376, a377) {
        let a378 = v1;
        let a379 = 0u64;
        nat_eq(a378, a379)
    } else {
        false
    } {
        let a380 = v2;
        let a381 = 0u64;
        nat_eq(a380, a381)
    } else {
        false
    } {
        let a382 = v3;
        let a383 = 0u64;
        nat_eq(a382, a383)
    } else {
        false
    } {
        let a384 = v4;
        let a385 = v8;
        if if if if if if if if nat_eq(a384, a385) {
            let a386 = v5;
            let a387 = v12;
            nat_eq(a386, a387)
        } else {
            false
        } {
            let a388 = v9;
            let a389 = v13;
            nat_eq(a388, a389)
        } else {
            false
        } {
            let a390 = v10;
            let a391 = v14;
            nat_eq(a390, a391)
        } else {
            false
        } {
            let a392 = v11;
            let a393 = v15;
            nat_eq(a392, a393)
        } else {
            false
        } {
            let a394 = 0u64;
            let a395 = v6;
            nat_lt(a394, a395)
        } else {
            false
        } {
            let a396 = 0u64;
            let a397 = v7;
            nat_lt(a396, a397)
        } else {
            false
        } {
            let a401 = {
                let a398 = v10;
                let a399 = v6;
                let a400 = 1u64;
                nat_rem(a398, a399, a400)
            };
            let a402 = 0u64;
            if nat_eq(a401, a402) {
                let a406 = {
                    let a403 = v11;
                    let a404 = v7;
                    let a405 = 1u64;
                    nat_rem(a403, a404, a405)
                };
                let a407 = 0u64;
                nat_eq(a406, a407)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt7::C2(v5, v4, v9, v10, v11, {
                let a408 = v10;
                let a409 = v6;
                let a410 = 0u64;
                nat_quot(a408, a409, a410)
            }, {
                let a411 = v11;
                let a412 = v7;
                let a413 = 0u64;
                nat_quot(a411, a412, a413)
            }, {
                let a414 = v5;
                let a415 = v4;
                nat_mul(a414, a415)?
            }, {
                let a416 = v4;
                let a417 = v9;
                nat_mul(a416, a417)?
            }, {
                let a418 = v5;
                let a419 = v9;
                nat_mul(a418, a419)?
            }))
        } else {
            Ok(Adt7::C1)
        }
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f10(v0: u64) -> bool {
    if f13(v0) {
        true
    } else {
        if if if f15(v0) {
            true
        } else {
            let a420 = v0;
            let a421 = 10u64;
            nat_eq(a420, a421)
        } {
            true
        } else {
            let a422 = v0;
            let a423 = 12u64;
            nat_eq(a422, a423)
        } {
            true
        } else {
            let a424 = v0;
            let a425 = 13u64;
            nat_eq(a424, a425)
        }
    }
}

pub fn f11(v0: u64) -> bool {
    let a426 = v0;
    let a427 = 0u64;
    if if nat_eq(a426, a427) {
        true
    } else {
        let a428 = v0;
        let a429 = 1u64;
        nat_eq(a428, a429)
    } {
        true
    } else {
        let a430 = v0;
        let a431 = 30u64;
        nat_eq(a430, a431)
    }
}

pub fn f12(v0: u64, v1: u64) -> bool {
    let a432 = v0;
    let a433 = 0u64;
    if if nat_eq(a432, a433) {
        let a434 = v1;
        let a435 = 4u64;
        nat_eq(a434, a435)
    } else {
        false
    } {
        true
    } else {
        let a436 = v0;
        let a437 = 1u64;
        if nat_eq(a436, a437) {
            let a438 = v1;
            let a439 = 2u64;
            nat_eq(a438, a439)
        } else {
            false
        }
    }
}

pub fn f13(v0: u64) -> bool {
    let a440 = v0;
    let a441 = 41u64;
    if nat_eq(a440, a441) {
        true
    } else {
        let a442 = v0;
        let a443 = 42u64;
        nat_eq(a442, a443)
    }
}

pub fn f14(v0: u64) -> u64 {
    let a444 = v0;
    let a445 = 30u64;
    if nat_eq(a444, a445) {
        30u64
    } else {
        0u64
    }
}

pub fn f15(v0: u64) -> bool {
    let a446 = v0;
    let a447 = 2u64;
    if if if nat_eq(a446, a447) {
        true
    } else {
        let a448 = v0;
        let a449 = 6u64;
        nat_eq(a448, a449)
    } {
        true
    } else {
        let a450 = v0;
        let a451 = 8u64;
        nat_eq(a450, a451)
    } {
        true
    } else {
        let a452 = v0;
        let a453 = 20u64;
        nat_eq(a452, a453)
    }
}

pub fn run(p0: Adt0) -> R<Adt4> {
    f0(p0)
}
