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
    C0(u64, bool, Adt1, Adt1, Adt1, bool),
    C1(Adt2, u64),
    C2(Adt2, u64, u64),
    C3(u64, bool, Adt1, Adt1, Adt1, Adt1, bool),
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
        Adt0::C0(v1, v2, v3, v4, v5, v6) => {
            f1(v1, v2, v3, v4, v5, v6)
        }
        Adt0::C1(v7, v8) => {
            let m3 = match v7.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2, _, _) => {
                    h2
                }
            };
            match m3 {
                None => {
                    f2(v7, v8)
                }
                Some(v9) => {
                    let a5 = v8;
                    let a6 = match v7.clone() {
                        Adt2::C0(_, _, _, _, _, h4, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4
                        }
                    };
                    if nat_lt(a5, a6) {
                        Ok(Adt4::C4({
                            let a21 = {
                                let a12 = {
                                    let a8 = v8;
                                    let a9 = match v7.clone() {
                                        Adt2::C0(_, _, _, h7, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h7
                                        }
                                    };
                                    let a10 = 0u64;
                                    nat_rem(a8, a9, a10)
                                };
                                let a13 = match v9.clone() {
                                    Adt3::C0(_, _, h11, _) => {
                                        h11
                                    }
                                };
                                nat_mul(a12, a13)?
                            };
                            let a22 = {
                                let a19 = {
                                    let a15 = v8;
                                    let a16 = match v7 {
                                        Adt2::C0(_, _, _, h14, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h14
                                        }
                                    };
                                    let a17 = 0u64;
                                    nat_quot(a15, a16, a17)
                                };
                                let a20 = match v9 {
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
        Adt0::C2(v20, v21, v22) => {
            let a24 = v22;
            let a25 = match v20.clone() {
                Adt2::C0(_, _, h23, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h23
                }
            };
            if nat_lt(a24, a25) {
                Ok(Adt4::C3({
                    let a29 = v21;
                    let a30 = {
                        let a27 = v22;
                        let a28 = match v20 {
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
        Adt0::C3(v10, v11, v12, v13, v14, v15, v16) => {
            f3(v10, v11, v12, v13, v14, v15, v16)
        }
        Adt0::C4(v17, v18, v19) => {
            f4(v17, v18, v19)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: bool) -> R<Adt4> {
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
            Adt5::C1(v6) => {
                f6(v0, v2, v3, v4, match v6.clone() {
                    Adt6::C0(_, h33, _) => {
                        h33
                    }
                }, match v6 {
                    Adt6::C0(_, _, h34) => {
                        h34
                    }
                }, v5)
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

pub fn f3(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: bool) -> R<Adt4> {
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
        if nat_eq(a102, a103) {
            true
        } else {
            if v6 {
                f7(match v3.clone() {
                    Adt1::C0(h104, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                })
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a106 = match v5.clone() {
            Adt1::C0(h105, _, _, _, _, _, _, _, _) => {
                h105
            }
        };
        let a107 = 0u64;
        nat_eq(a106, a107)
    } else {
        false
    } {
        let a109 = match v2.clone() {
            Adt1::C0(_, _, _, _, h108, _, _, _, _) => {
                h108
            }
        };
        let a110 = 1u64;
        nat_eq(a109, a110)
    } else {
        false
    } {
        let a112 = match v3.clone() {
            Adt1::C0(_, _, _, _, h111, _, _, _, _) => {
                h111
            }
        };
        let a113 = 1u64;
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a115 = match v4.clone() {
            Adt1::C0(_, _, _, _, h114, _, _, _, _) => {
                h114
            }
        };
        let a116 = 1u64;
        nat_eq(a115, a116)
    } else {
        false
    } {
        let a118 = match v5.clone() {
            Adt1::C0(_, _, _, _, h117, _, _, _, _) => {
                h117
            }
        };
        let a119 = 1u64;
        nat_eq(a118, a119)
    } else {
        false
    } {
        let a121 = match v4.clone() {
            Adt1::C0(_, _, _, h120, _, _, _, _, _) => {
                h120
            }
        };
        let a122 = 1u64;
        nat_eq(a121, a122)
    } else {
        false
    } {
        let a125 = match v4.clone() {
            Adt1::C0(_, _, h123, _, _, _, _, _, _) => {
                h123
            }
        };
        let a126 = match v3.clone() {
            Adt1::C0(_, _, _, h124, _, _, _, _, _) => {
                h124
            }
        };
        nat_eq(a125, a126)
    } else {
        false
    } {
        let a129 = match v2.clone() {
            Adt1::C0(_, h127, _, _, _, _, _, _, _) => {
                h127
            }
        };
        let a130 = match v3.clone() {
            Adt1::C0(_, h128, _, _, _, _, _, _, _) => {
                h128
            }
        };
        nat_eq(a129, a130)
    } else {
        false
    } {
        let a132 = 0u64;
        let a133 = match v3.clone() {
            Adt1::C0(_, _, h131, _, _, _, _, _, _) => {
                h131
            }
        };
        nat_lt(a132, a133)
    } else {
        false
    } {
        let a139 = {
            let a136 = match v4.clone() {
                Adt1::C0(_, h134, _, _, _, _, _, _, _) => {
                    h134
                }
            };
            let a137 = match v3.clone() {
                Adt1::C0(_, _, h135, _, _, _, _, _, _) => {
                    h135
                }
            };
            let a138 = 0u64;
            nat_rem(a136, a137, a138)
        };
        let a140 = 0u64;
        nat_eq(a139, a140)
    } else {
        false
    } {
        let a143 = match v5.clone() {
            Adt1::C0(_, h141, _, _, _, _, _, _, _) => {
                h141
            }
        };
        let a144 = match v2.clone() {
            Adt1::C0(_, _, h142, _, _, _, _, _, _) => {
                h142
            }
        };
        nat_eq(a143, a144)
    } else {
        false
    } {
        let a147 = match v5.clone() {
            Adt1::C0(_, _, h145, _, _, _, _, _, _) => {
                h145
            }
        };
        let a148 = match v4.clone() {
            Adt1::C0(_, h146, _, _, _, _, _, _, _) => {
                h146
            }
        };
        nat_eq(a147, a148)
    } else {
        false
    } {
        let a151 = match v5.clone() {
            Adt1::C0(_, _, _, h149, _, _, _, _, _) => {
                h149
            }
        };
        let a152 = match v3.clone() {
            Adt1::C0(_, _, _, h150, _, _, _, _, _) => {
                h150
            }
        };
        nat_eq(a151, a152)
    } else {
        false
    } {
        let a155 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h153, _, _, _) => {
                h153
            }
        };
        let a156 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h154, _, _) => {
                h154
            }
        };
        nat_le(a155, a156)
    } else {
        false
    } {
        let a161 = {
            let a158 = match v5.clone() {
                Adt1::C0(_, h157, _, _, _, _, _, _, _) => {
                    h157
                }
            };
            let a159 = 4u64;
            nat_mul(a158, a159)?
        };
        let a162 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, h160, _, _) => {
                h160
            }
        };
        nat_le(a161, a162)
    } else {
        false
    } {
        let a168 = {
            let a165 = match v5.clone() {
                Adt1::C0(_, _, h163, _, _, _, _, _, _) => {
                    h163
                }
            };
            let a166 = match v5.clone() {
                Adt1::C0(_, _, _, _, _, _, h164, _, _) => {
                    h164
                }
            };
            nat_mul(a165, a166)?
        };
        let a169 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h167, _) => {
                h167
            }
        };
        nat_le(a168, a169)
    } else {
        false
    } {
        let m171 = f5(match v2.clone() {
            Adt1::C0(h170, _, _, _, _, _, _, _, _) => {
                h170
            }
        });
        match m171 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v7) => {
                if f8(v2.clone(), v3.clone(), v5.clone(), true, v6, match v7.clone() {
                    Adt6::C0(_, h172, _) => {
                        h172
                    }
                }, match v7.clone() {
                    Adt6::C0(_, _, h173) => {
                        h173
                    }
                }) {
                    Ok(Adt4::C1(f9(v2.clone(), v3.clone(), v5, match v7.clone() {
                        Adt6::C0(_, h174, _) => {
                            h174
                        }
                    }, match v7 {
                        Adt6::C0(_, _, h175) => {
                            h175
                        }
                    }, match v2.clone() {
                        Adt1::C0(_, _, h176, _, _, _, _, _, _) => {
                            h176
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h177, _, _, _, _, _, _, _) => {
                            h177
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h178, _, _, _, _, _, _) => {
                            h178
                        }
                    }, 1u64, 1u64, 1u64, Some(Adt3::C0(match v2 {
                        Adt1::C0(_, _, _, h179, _, _, _, _, _) => {
                            h179
                        }
                    }, match v3 {
                        Adt1::C0(_, _, h180, _, _, _, _, _, _) => {
                            h180
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h181, _, _, _) => {
                            h181
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h182, _, _) => {
                            h182
                        }
                    })), v6)?))
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
    let m184 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h183, _, _) => {
            h183
        }
    };
    match m184 {
        None => {
            Ok(Adt4::C0)
        }
        Some(v3) => {
            let a186 = v1;
            let a187 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h185, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h185
                }
            };
            if nat_lt(a186, a187) {
                let v4: u64 = {
                    let a189 = v1;
                    let a190 = match v0.clone() {
                        Adt2::C0(_, _, _, h188, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h188
                        }
                    };
                    let a191 = 0u64;
                    nat_rem(a189, a190, a191)
                };
                let v5: u64 = {
                    let a193 = v1;
                    let a194 = match v0.clone() {
                        Adt2::C0(_, _, _, h192, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h192
                        }
                    };
                    let a195 = 0u64;
                    nat_quot(a193, a194, a195)
                };
                let v6: u64 = {
                    let a202 = {
                        let a197 = v4;
                        let a198 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h196, _, _, _, _, _) => {
                                h196
                            }
                        };
                        nat_mul(a197, a198)?
                    };
                    let a203 = {
                        let a200 = v5;
                        let a201 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h199, _, _, _, _) => {
                                h199
                            }
                        };
                        nat_mul(a200, a201)?
                    };
                    nat_add(a202, a203)?
                };
                let a204 = v2;
                let a205 = 4294967295u64;
                if nat_eq(a204, a205) {
                    Ok(Adt4::C5(v6))
                } else {
                    let a207 = v2;
                    let a208 = match v3.clone() {
                        Adt3::C0(h206, _, _, _) => {
                            h206
                        }
                    };
                    if nat_lt(a207, a208) {
                        Ok(Adt4::C2({
                            let a210 = v2;
                            let a211 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h209, _, _, _, _, _, _, _, _, _, _) => {
                                    h209
                                }
                            };
                            nat_mul(a210, a211)?
                        }, {
                            let a222 = {
                                let a217 = {
                                    let a213 = v4;
                                    let a214 = match v3 {
                                        Adt3::C0(_, h212, _, _) => {
                                            h212
                                        }
                                    };
                                    let a215 = 0u64;
                                    nat_rem(a213, a214, a215)
                                };
                                let a218 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h216, _, _, _, _, _, _, _, _) => {
                                        h216
                                    }
                                };
                                nat_mul(a217, a218)?
                            };
                            let a223 = {
                                let a220 = v5;
                                let a221 = match v0 {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h219, _, _, _, _, _, _, _) => {
                                        h219
                                    }
                                };
                                nat_mul(a220, a221)?
                            };
                            nat_add(a222, a223)?
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
    let a224 = v0;
    let a225 = 0u64;
    if nat_eq(a224, a225) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a226 = v0;
        let a227 = 1u64;
        if nat_eq(a226, a227) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a228 = v0;
            let a229 = 2u64;
            if nat_eq(a228, a229) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a230 = v0;
                let a231 = 3u64;
                if nat_eq(a230, a231) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a232 = v0;
                    let a233 = 6u64;
                    if nat_eq(a232, a233) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a234 = v0;
                        let a235 = 7u64;
                        if nat_eq(a234, a235) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a236 = v0;
                            let a237 = 8u64;
                            if nat_eq(a236, a237) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a238 = v0;
                                let a239 = 9u64;
                                if nat_eq(a238, a239) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a240 = v0;
                                    let a241 = 10u64;
                                    if nat_eq(a240, a241) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a242 = v0;
                                        let a243 = 11u64;
                                        if nat_eq(a242, a243) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a244 = v0;
                                            let a245 = 12u64;
                                            if nat_eq(a244, a245) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a246 = v0;
                                                let a247 = 13u64;
                                                if nat_eq(a246, a247) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a248 = v0;
                                                    let a249 = 14u64;
                                                    if nat_eq(a248, a249) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a250 = v0;
                                                        let a251 = 15u64;
                                                        if nat_eq(a250, a251) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a252 = v0;
                                                            let a253 = 16u64;
                                                            if nat_eq(a252, a253) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a254 = v0;
                                                                let a255 = 17u64;
                                                                if nat_eq(a254, a255) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a256 = v0;
                                                                    let a257 = 18u64;
                                                                    if nat_eq(a256, a257) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a258 = v0;
                                                                        let a259 = 19u64;
                                                                        if nat_eq(a258, a259) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a260 = v0;
                                                                            let a261 = 20u64;
                                                                            if nat_eq(a260, a261) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a262 = v0;
                                                                                let a263 = 21u64;
                                                                                if nat_eq(a262, a263) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a264 = v0;
                                                                                    let a265 = 22u64;
                                                                                    if nat_eq(a264, a265) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a266 = v0;
                                                                                        let a267 = 23u64;
                                                                                        if nat_eq(a266, a267) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a268 = v0;
                                                                                            let a269 = 24u64;
                                                                                            if nat_eq(a268, a269) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a270 = v0;
                                                                                                let a271 = 25u64;
                                                                                                if nat_eq(a270, a271) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a272 = v0;
                                                                                                    let a273 = 26u64;
                                                                                                    if nat_eq(a272, a273) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a274 = v0;
                                                                                                        let a275 = 27u64;
                                                                                                        if nat_eq(a274, a275) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a276 = v0;
                                                                                                            let a277 = 28u64;
                                                                                                            if nat_eq(a276, a277) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a278 = v0;
                                                                                                                let a279 = 29u64;
                                                                                                                if nat_eq(a278, a279) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a280 = v0;
                                                                                                                    let a281 = 30u64;
                                                                                                                    if nat_eq(a280, a281) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a282 = v0;
                                                                                                                        let a283 = 34u64;
                                                                                                                        if nat_eq(a282, a283) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a284 = v0;
                                                                                                                            let a285 = 35u64;
                                                                                                                            if nat_eq(a284, a285) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a286 = v0;
                                                                                                                                let a287 = 39u64;
                                                                                                                                if nat_eq(a286, a287) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a288 = v0;
                                                                                                                                    let a289 = 40u64;
                                                                                                                                    if nat_eq(a288, a289) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a290 = v0;
                                                                                                                                        let a291 = 41u64;
                                                                                                                                        if nat_eq(a290, a291) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a292 = v0;
                                                                                                                                            let a293 = 42u64;
                                                                                                                                            if nat_eq(a292, a293) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a294 = v0;
                                                                                                                                                let a295 = 43u64;
                                                                                                                                                if nat_eq(a294, a295) {
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

pub fn f6(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64, v5: u64, v6: bool) -> R<Adt4> {
    if f8(v1.clone(), v2.clone(), v3.clone(), v6, v6, v4, v5) {
        let m309 = f10(v0, 0u64, 0u64, match v3.clone() {
            Adt1::C0(h296, _, _, _, _, _, _, _, _) => {
                h296
            }
        }, match v1.clone() {
            Adt1::C0(_, h297, _, _, _, _, _, _, _) => {
                h297
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h298, _, _, _, _, _, _) => {
                h298
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h299, _, _, _, _, _) => {
                h299
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h300, _, _, _, _) => {
                h300
            }
        }, match v2.clone() {
            Adt1::C0(_, h301, _, _, _, _, _, _, _) => {
                h301
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h302, _, _, _, _, _, _) => {
                h302
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h303, _, _, _, _, _) => {
                h303
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h304, _, _, _, _) => {
                h304
            }
        }, match v3.clone() {
            Adt1::C0(_, h305, _, _, _, _, _, _, _) => {
                h305
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h306, _, _, _, _, _, _) => {
                h306
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h307, _, _, _, _, _) => {
                h307
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h308, _, _, _, _) => {
                h308
            }
        })?;
        match m309 {
            Adt7::C0 => {
                Ok(Adt4::C0)
            }
            Adt7::C1 => {
                Ok(Adt4::C0)
            }
            Adt7::C2(v7, _, v9, v10, v11, v12, v13, _, _, _) => {
                Ok(Adt4::C1(f9(v1, v2, v3, v4, v5, v7, v9, v10, v11, v12, v13, None::<Adt3>, v6)?))
            }
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f7(v0: u64) -> bool {
    let a310 = v0;
    let a311 = 0u64;
    if if nat_eq(a310, a311) {
        true
    } else {
        let a312 = v0;
        let a313 = 1u64;
        nat_eq(a312, a313)
    } {
        true
    } else {
        let a314 = v0;
        let a315 = 30u64;
        nat_eq(a314, a315)
    }
}

pub fn f8(v0: Adt1, v1: Adt1, v2: Adt1, v3: bool, v4: bool, v5: u64, v6: u64) -> bool {
    if if if if if if if if f11(match v0.clone() {
        Adt1::C0(h316, _, _, _, _, _, _, _, _) => {
            h316
        }
    }) {
        true
    } else {
        if v3 {
            if f7(match v0.clone() {
                Adt1::C0(h317, _, _, _, _, _, _, _, _) => {
                    h317
                }
            }) {
                true
            } else {
                let a319 = match v0.clone() {
                    Adt1::C0(h318, _, _, _, _, _, _, _, _) => {
                        h318
                    }
                };
                let a320 = 43u64;
                nat_eq(a319, a320)
            }
        } else {
            false
        }
    } {
        let a325 = {
            let a322 = match v0.clone() {
                Adt1::C0(_, h321, _, _, _, _, _, _, _) => {
                    h321
                }
            };
            let a323 = v5;
            let a324 = 0u64;
            nat_rem(a322, a323, a324)
        };
        let a326 = 0u64;
        nat_eq(a325, a326)
    } else {
        false
    } {
        let a328 = match v0 {
            Adt1::C0(_, _, _, _, _, h327, _, _, _) => {
                h327
            }
        };
        let a329 = v6;
        nat_eq(a328, a329)
    } else {
        false
    } {
        f12(match v1.clone() {
            Adt1::C0(h330, _, _, _, _, _, _, _, _) => {
                h330
            }
        }, match v1 {
            Adt1::C0(_, _, _, _, _, h331, _, _, _) => {
                h331
            }
        }, v4)
    } else {
        false
    } {
        let a333 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h332, _, _, _) => {
                h332
            }
        };
        let a334 = 4u64;
        nat_eq(a333, a334)
    } else {
        false
    } {
        let a337 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h335, _, _, _) => {
                h335
            }
        };
        let a338 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h336, _, _) => {
                h336
            }
        };
        nat_le(a337, a338)
    } else {
        false
    } {
        let a341 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h339, _, _) => {
                h339
            }
        };
        let a342 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h340, _) => {
                h340
            }
        };
        nat_le(a341, a342)
    } else {
        false
    } {
        let a345 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h343, _) => {
                h343
            }
        };
        let a346 = match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h344) => {
                h344
            }
        };
        nat_le(a345, a346)
    } else {
        false
    }
}

pub fn f9(v0: Adt1, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: Option<Adt3>, v12: bool) -> R<Adt2> {
    Ok(Adt2::C0(if v12 {
        match v0.clone() {
            Adt1::C0(_, h347, _, _, _, _, _, _, _) => {
                h347
            }
        }
    } else {
        let a350 = match v0.clone() {
            Adt1::C0(_, h348, _, _, _, _, _, _, _) => {
                h348
            }
        };
        let a351 = f13(match v0.clone() {
            Adt1::C0(h349, _, _, _, _, _, _, _, _) => {
                h349
            }
        }, v3);
        let a352 = 0u64;
        nat_quot(a350, a351, a352)
    }, {
        let a357 = {
            let a354 = match v0.clone() {
                Adt1::C0(_, h353, _, _, _, _, _, _, _) => {
                    h353
                }
            };
            let a355 = v3;
            let a356 = 0u64;
            nat_quot(a354, a355, a356)
        };
        let a358 = v4;
        nat_mul(a357, a358)?
    }, v5, v6, v7, {
        let a361 = v6;
        let a362 = {
            let a359 = v7;
            let a360 = v8;
            nat_mul(a359, a360)?
        };
        nat_mul(a361, a362)?
    }, {
        let a367 = v5;
        let a368 = {
            let a365 = v6;
            let a366 = {
                let a363 = v7;
                let a364 = v8;
                nat_mul(a363, a364)?
            };
            nat_mul(a365, a366)?
        };
        nat_mul(a367, a368)?
    }, v9, v10, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h369, _, _) => {
            h369
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h370, _) => {
            h370
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h371) => {
            h371
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h372, _, _) => {
            h372
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h373, _) => {
            h373
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h374) => {
            h374
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h375, _, _) => {
            h375
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h376, _) => {
            h376
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h377) => {
            h377
        }
    }, v11, if v12 {
        Some(0u64)
    } else {
        if if f7(match v0.clone() {
            Adt1::C0(h378, _, _, _, _, _, _, _, _) => {
                h378
            }
        }) {
            true
        } else {
            let a380 = match v0.clone() {
                Adt1::C0(h379, _, _, _, _, _, _, _, _) => {
                    h379
                }
            };
            let a381 = 43u64;
            nat_eq(a380, a381)
        } {
            Some(f14(match v0.clone() {
                Adt1::C0(h382, _, _, _, _, _, _, _, _) => {
                    h382
                }
            }))
        } else {
            None::<u64>
        }
    }, match v0 {
        Adt1::C0(h383, _, _, _, _, _, _, _, _) => {
            h383
        }
    }))
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt7> {
    let a384 = v0;
    let a385 = 28u64;
    if if if if nat_eq(a384, a385) {
        let a386 = v1;
        let a387 = 0u64;
        nat_eq(a386, a387)
    } else {
        false
    } {
        let a388 = v2;
        let a389 = 0u64;
        nat_eq(a388, a389)
    } else {
        false
    } {
        let a390 = v3;
        let a391 = 0u64;
        nat_eq(a390, a391)
    } else {
        false
    } {
        let a392 = v4;
        let a393 = v8;
        if if if if if if if if nat_eq(a392, a393) {
            let a394 = v5;
            let a395 = v12;
            nat_eq(a394, a395)
        } else {
            false
        } {
            let a396 = v9;
            let a397 = v13;
            nat_eq(a396, a397)
        } else {
            false
        } {
            let a398 = v10;
            let a399 = v14;
            nat_eq(a398, a399)
        } else {
            false
        } {
            let a400 = v11;
            let a401 = v15;
            nat_eq(a400, a401)
        } else {
            false
        } {
            let a402 = 0u64;
            let a403 = v6;
            nat_lt(a402, a403)
        } else {
            false
        } {
            let a404 = 0u64;
            let a405 = v7;
            nat_lt(a404, a405)
        } else {
            false
        } {
            let a409 = {
                let a406 = v10;
                let a407 = v6;
                let a408 = 1u64;
                nat_rem(a406, a407, a408)
            };
            let a410 = 0u64;
            if nat_eq(a409, a410) {
                let a414 = {
                    let a411 = v11;
                    let a412 = v7;
                    let a413 = 1u64;
                    nat_rem(a411, a412, a413)
                };
                let a415 = 0u64;
                nat_eq(a414, a415)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt7::C2(v5, v4, v9, v10, v11, {
                let a416 = v10;
                let a417 = v6;
                let a418 = 0u64;
                nat_quot(a416, a417, a418)
            }, {
                let a419 = v11;
                let a420 = v7;
                let a421 = 0u64;
                nat_quot(a419, a420, a421)
            }, {
                let a422 = v5;
                let a423 = v4;
                nat_mul(a422, a423)?
            }, {
                let a424 = v4;
                let a425 = v9;
                nat_mul(a424, a425)?
            }, {
                let a426 = v5;
                let a427 = v9;
                nat_mul(a426, a427)?
            }))
        } else {
            Ok(Adt7::C1)
        }
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f11(v0: u64) -> bool {
    if if f15(v0) {
        true
    } else {
        f16(v0)
    } {
        true
    } else {
        f17(v0)
    }
}

pub fn f12(v0: u64, v1: u64, v2: bool) -> bool {
    let a428 = v0;
    let a429 = 0u64;
    if if nat_eq(a428, a429) {
        let a430 = v1;
        let a431 = 4u64;
        nat_eq(a430, a431)
    } else {
        false
    } {
        true
    } else {
        let a432 = v0;
        let a433 = 1u64;
        if if nat_eq(a432, a433) {
            true
        } else {
            if v2 {
                let a434 = v0;
                let a435 = 30u64;
                nat_eq(a434, a435)
            } else {
                false
            }
        } {
            let a436 = v1;
            let a437 = 2u64;
            nat_eq(a436, a437)
        } else {
            false
        }
    }
}

pub fn f13(v0: u64, v1: u64) -> u64 {
    if if f16(v0) {
        true
    } else {
        f18(v0)
    } {
        32u64
    } else {
        v1
    }
}

pub fn f14(v0: u64) -> u64 {
    let a438 = v0;
    let a439 = 30u64;
    if nat_eq(a438, a439) {
        30u64
    } else {
        0u64
    }
}

pub fn f15(v0: u64) -> bool {
    if f19(v0) {
        true
    } else {
        f20(v0)
    }
}

pub fn f16(v0: u64) -> bool {
    let a440 = v0;
    let a441 = 39u64;
    if nat_eq(a440, a441) {
        true
    } else {
        let a442 = v0;
        let a443 = 40u64;
        nat_eq(a442, a443)
    }
}

pub fn f17(v0: u64) -> bool {
    let a444 = 10u64;
    let a445 = v0;
    if if if if if nat_le(a444, a445) {
        let a446 = v0;
        let a447 = 14u64;
        nat_le(a446, a447)
    } else {
        false
    } {
        true
    } else {
        let a448 = 16u64;
        let a449 = v0;
        if nat_le(a448, a449) {
            let a450 = v0;
            let a451 = 19u64;
            nat_le(a450, a451)
        } else {
            false
        }
    } {
        true
    } else {
        let a452 = 21u64;
        let a453 = v0;
        if nat_le(a452, a453) {
            let a454 = v0;
            let a455 = 23u64;
            nat_le(a454, a455)
        } else {
            false
        }
    } {
        true
    } else {
        let a456 = 34u64;
        let a457 = v0;
        if nat_le(a456, a457) {
            let a458 = v0;
            let a459 = 35u64;
            nat_le(a458, a459)
        } else {
            false
        }
    } {
        true
    } else {
        let a460 = v0;
        let a461 = 29u64;
        nat_eq(a460, a461)
    }
}

pub fn f18(v0: u64) -> bool {
    let a462 = v0;
    let a463 = 41u64;
    if nat_eq(a462, a463) {
        true
    } else {
        let a464 = v0;
        let a465 = 42u64;
        nat_eq(a464, a465)
    }
}

pub fn f19(v0: u64) -> bool {
    let a466 = v0;
    let a467 = 2u64;
    if if if if nat_eq(a466, a467) {
        true
    } else {
        let a468 = v0;
        let a469 = 6u64;
        nat_eq(a468, a469)
    } {
        true
    } else {
        let a470 = v0;
        let a471 = 8u64;
        nat_eq(a470, a471)
    } {
        true
    } else {
        let a472 = v0;
        let a473 = 20u64;
        nat_eq(a472, a473)
    } {
        true
    } else {
        f18(v0)
    }
}

pub fn f20(v0: u64) -> bool {
    let a474 = v0;
    let a475 = 3u64;
    if nat_eq(a474, a475) {
        true
    } else {
        let a476 = v0;
        let a477 = 7u64;
        nat_eq(a476, a477)
    }
}

pub fn run(p0: Adt0) -> R<Adt4> {
    f0(p0)
}
