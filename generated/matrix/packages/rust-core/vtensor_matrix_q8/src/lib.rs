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
        if if nat_eq(a102, a103) {
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
        } {
            true
        } else {
            f8(match v2.clone() {
                Adt1::C0(h105, _, _, _, _, _, _, _, _) => {
                    h105
                }
            }, match v3.clone() {
                Adt1::C0(h106, _, _, _, _, _, _, _, _) => {
                    h106
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h107, _, _, _) => {
                    h107
                }
            }, v6)
        }
    } else {
        false
    } {
        let a109 = match v5.clone() {
            Adt1::C0(h108, _, _, _, _, _, _, _, _) => {
                h108
            }
        };
        let a110 = 0u64;
        nat_eq(a109, a110)
    } else {
        false
    } {
        let a112 = match v2.clone() {
            Adt1::C0(_, _, _, _, h111, _, _, _, _) => {
                h111
            }
        };
        let a113 = 1u64;
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a115 = match v3.clone() {
            Adt1::C0(_, _, _, _, h114, _, _, _, _) => {
                h114
            }
        };
        let a116 = 1u64;
        nat_eq(a115, a116)
    } else {
        false
    } {
        let a118 = match v4.clone() {
            Adt1::C0(_, _, _, _, h117, _, _, _, _) => {
                h117
            }
        };
        let a119 = 1u64;
        nat_eq(a118, a119)
    } else {
        false
    } {
        let a121 = match v5.clone() {
            Adt1::C0(_, _, _, _, h120, _, _, _, _) => {
                h120
            }
        };
        let a122 = 1u64;
        nat_eq(a121, a122)
    } else {
        false
    } {
        let a124 = match v4.clone() {
            Adt1::C0(_, _, _, h123, _, _, _, _, _) => {
                h123
            }
        };
        let a125 = 1u64;
        nat_eq(a124, a125)
    } else {
        false
    } {
        let a128 = match v4.clone() {
            Adt1::C0(_, _, h126, _, _, _, _, _, _) => {
                h126
            }
        };
        let a129 = match v3.clone() {
            Adt1::C0(_, _, _, h127, _, _, _, _, _) => {
                h127
            }
        };
        nat_eq(a128, a129)
    } else {
        false
    } {
        let a132 = match v2.clone() {
            Adt1::C0(_, h130, _, _, _, _, _, _, _) => {
                h130
            }
        };
        let a133 = match v3.clone() {
            Adt1::C0(_, h131, _, _, _, _, _, _, _) => {
                h131
            }
        };
        nat_eq(a132, a133)
    } else {
        false
    } {
        let a135 = 0u64;
        let a136 = match v3.clone() {
            Adt1::C0(_, _, h134, _, _, _, _, _, _) => {
                h134
            }
        };
        nat_lt(a135, a136)
    } else {
        false
    } {
        let a142 = {
            let a139 = match v4.clone() {
                Adt1::C0(_, h137, _, _, _, _, _, _, _) => {
                    h137
                }
            };
            let a140 = match v3.clone() {
                Adt1::C0(_, _, h138, _, _, _, _, _, _) => {
                    h138
                }
            };
            let a141 = 0u64;
            nat_rem(a139, a140, a141)
        };
        let a143 = 0u64;
        nat_eq(a142, a143)
    } else {
        false
    } {
        let a146 = match v5.clone() {
            Adt1::C0(_, h144, _, _, _, _, _, _, _) => {
                h144
            }
        };
        let a147 = match v2.clone() {
            Adt1::C0(_, _, h145, _, _, _, _, _, _) => {
                h145
            }
        };
        nat_eq(a146, a147)
    } else {
        false
    } {
        let a150 = match v5.clone() {
            Adt1::C0(_, _, h148, _, _, _, _, _, _) => {
                h148
            }
        };
        let a151 = match v4.clone() {
            Adt1::C0(_, h149, _, _, _, _, _, _, _) => {
                h149
            }
        };
        nat_eq(a150, a151)
    } else {
        false
    } {
        let a154 = match v5.clone() {
            Adt1::C0(_, _, _, h152, _, _, _, _, _) => {
                h152
            }
        };
        let a155 = match v3.clone() {
            Adt1::C0(_, _, _, h153, _, _, _, _, _) => {
                h153
            }
        };
        nat_eq(a154, a155)
    } else {
        false
    } {
        let a158 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h156, _, _, _) => {
                h156
            }
        };
        let a159 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h157, _, _) => {
                h157
            }
        };
        nat_le(a158, a159)
    } else {
        false
    } {
        let a164 = {
            let a161 = match v5.clone() {
                Adt1::C0(_, h160, _, _, _, _, _, _, _) => {
                    h160
                }
            };
            let a162 = 4u64;
            nat_mul(a161, a162)?
        };
        let a165 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, h163, _, _) => {
                h163
            }
        };
        nat_le(a164, a165)
    } else {
        false
    } {
        let a171 = {
            let a168 = match v5.clone() {
                Adt1::C0(_, _, h166, _, _, _, _, _, _) => {
                    h166
                }
            };
            let a169 = match v5.clone() {
                Adt1::C0(_, _, _, _, _, _, h167, _, _) => {
                    h167
                }
            };
            nat_mul(a168, a169)?
        };
        let a172 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h170, _) => {
                h170
            }
        };
        nat_le(a171, a172)
    } else {
        false
    } {
        let m174 = f5(match v2.clone() {
            Adt1::C0(h173, _, _, _, _, _, _, _, _) => {
                h173
            }
        });
        match m174 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v7) => {
                if f9(v2.clone(), v3.clone(), v5.clone(), true, v6, match v7.clone() {
                    Adt6::C0(_, h175, _) => {
                        h175
                    }
                }, match v7.clone() {
                    Adt6::C0(_, _, h176) => {
                        h176
                    }
                }) {
                    Ok(Adt4::C1(f10(v2.clone(), v3.clone(), v5, match v7.clone() {
                        Adt6::C0(_, h177, _) => {
                            h177
                        }
                    }, match v7 {
                        Adt6::C0(_, _, h178) => {
                            h178
                        }
                    }, match v2.clone() {
                        Adt1::C0(_, _, h179, _, _, _, _, _, _) => {
                            h179
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h180, _, _, _, _, _, _, _) => {
                            h180
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h181, _, _, _, _, _, _) => {
                            h181
                        }
                    }, 1u64, 1u64, 1u64, Some(Adt3::C0(match v2 {
                        Adt1::C0(_, _, _, h182, _, _, _, _, _) => {
                            h182
                        }
                    }, match v3 {
                        Adt1::C0(_, _, h183, _, _, _, _, _, _) => {
                            h183
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h184, _, _, _) => {
                            h184
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h185, _, _) => {
                            h185
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
    let m187 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h186, _, _) => {
            h186
        }
    };
    match m187 {
        None => {
            Ok(Adt4::C0)
        }
        Some(v3) => {
            let a189 = v1;
            let a190 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h188, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h188
                }
            };
            if nat_lt(a189, a190) {
                let v4: u64 = {
                    let a192 = v1;
                    let a193 = match v0.clone() {
                        Adt2::C0(_, _, _, h191, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h191
                        }
                    };
                    let a194 = 0u64;
                    nat_rem(a192, a193, a194)
                };
                let v5: u64 = {
                    let a196 = v1;
                    let a197 = match v0.clone() {
                        Adt2::C0(_, _, _, h195, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h195
                        }
                    };
                    let a198 = 0u64;
                    nat_quot(a196, a197, a198)
                };
                let v6: u64 = {
                    let a205 = {
                        let a200 = v4;
                        let a201 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h199, _, _, _, _, _) => {
                                h199
                            }
                        };
                        nat_mul(a200, a201)?
                    };
                    let a206 = {
                        let a203 = v5;
                        let a204 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h202, _, _, _, _) => {
                                h202
                            }
                        };
                        nat_mul(a203, a204)?
                    };
                    nat_add(a205, a206)?
                };
                let a207 = v2;
                let a208 = 4294967295u64;
                if nat_eq(a207, a208) {
                    Ok(Adt4::C5(v6))
                } else {
                    let a210 = v2;
                    let a211 = match v3.clone() {
                        Adt3::C0(h209, _, _, _) => {
                            h209
                        }
                    };
                    if nat_lt(a210, a211) {
                        Ok(Adt4::C2({
                            let a213 = v2;
                            let a214 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h212, _, _, _, _, _, _, _, _, _, _) => {
                                    h212
                                }
                            };
                            nat_mul(a213, a214)?
                        }, {
                            let a225 = {
                                let a220 = {
                                    let a216 = v4;
                                    let a217 = match v3 {
                                        Adt3::C0(_, h215, _, _) => {
                                            h215
                                        }
                                    };
                                    let a218 = 0u64;
                                    nat_rem(a216, a217, a218)
                                };
                                let a221 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h219, _, _, _, _, _, _, _, _) => {
                                        h219
                                    }
                                };
                                nat_mul(a220, a221)?
                            };
                            let a226 = {
                                let a223 = v5;
                                let a224 = match v0 {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h222, _, _, _, _, _, _, _) => {
                                        h222
                                    }
                                };
                                nat_mul(a223, a224)?
                            };
                            nat_add(a225, a226)?
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
    let a227 = v0;
    let a228 = 0u64;
    if nat_eq(a227, a228) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a229 = v0;
        let a230 = 1u64;
        if nat_eq(a229, a230) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a231 = v0;
            let a232 = 2u64;
            if nat_eq(a231, a232) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a233 = v0;
                let a234 = 3u64;
                if nat_eq(a233, a234) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a235 = v0;
                    let a236 = 6u64;
                    if nat_eq(a235, a236) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a237 = v0;
                        let a238 = 7u64;
                        if nat_eq(a237, a238) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a239 = v0;
                            let a240 = 8u64;
                            if nat_eq(a239, a240) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a241 = v0;
                                let a242 = 9u64;
                                if nat_eq(a241, a242) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a243 = v0;
                                    let a244 = 10u64;
                                    if nat_eq(a243, a244) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a245 = v0;
                                        let a246 = 11u64;
                                        if nat_eq(a245, a246) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a247 = v0;
                                            let a248 = 12u64;
                                            if nat_eq(a247, a248) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a249 = v0;
                                                let a250 = 13u64;
                                                if nat_eq(a249, a250) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a251 = v0;
                                                    let a252 = 14u64;
                                                    if nat_eq(a251, a252) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a253 = v0;
                                                        let a254 = 15u64;
                                                        if nat_eq(a253, a254) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a255 = v0;
                                                            let a256 = 16u64;
                                                            if nat_eq(a255, a256) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a257 = v0;
                                                                let a258 = 17u64;
                                                                if nat_eq(a257, a258) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a259 = v0;
                                                                    let a260 = 18u64;
                                                                    if nat_eq(a259, a260) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a261 = v0;
                                                                        let a262 = 19u64;
                                                                        if nat_eq(a261, a262) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a263 = v0;
                                                                            let a264 = 20u64;
                                                                            if nat_eq(a263, a264) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a265 = v0;
                                                                                let a266 = 21u64;
                                                                                if nat_eq(a265, a266) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a267 = v0;
                                                                                    let a268 = 22u64;
                                                                                    if nat_eq(a267, a268) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a269 = v0;
                                                                                        let a270 = 23u64;
                                                                                        if nat_eq(a269, a270) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a271 = v0;
                                                                                            let a272 = 24u64;
                                                                                            if nat_eq(a271, a272) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a273 = v0;
                                                                                                let a274 = 25u64;
                                                                                                if nat_eq(a273, a274) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a275 = v0;
                                                                                                    let a276 = 26u64;
                                                                                                    if nat_eq(a275, a276) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a277 = v0;
                                                                                                        let a278 = 27u64;
                                                                                                        if nat_eq(a277, a278) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a279 = v0;
                                                                                                            let a280 = 28u64;
                                                                                                            if nat_eq(a279, a280) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a281 = v0;
                                                                                                                let a282 = 29u64;
                                                                                                                if nat_eq(a281, a282) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a283 = v0;
                                                                                                                    let a284 = 30u64;
                                                                                                                    if nat_eq(a283, a284) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a285 = v0;
                                                                                                                        let a286 = 34u64;
                                                                                                                        if nat_eq(a285, a286) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a287 = v0;
                                                                                                                            let a288 = 35u64;
                                                                                                                            if nat_eq(a287, a288) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a289 = v0;
                                                                                                                                let a290 = 39u64;
                                                                                                                                if nat_eq(a289, a290) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a291 = v0;
                                                                                                                                    let a292 = 40u64;
                                                                                                                                    if nat_eq(a291, a292) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a293 = v0;
                                                                                                                                        let a294 = 41u64;
                                                                                                                                        if nat_eq(a293, a294) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a295 = v0;
                                                                                                                                            let a296 = 42u64;
                                                                                                                                            if nat_eq(a295, a296) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a297 = v0;
                                                                                                                                                let a298 = 43u64;
                                                                                                                                                if nat_eq(a297, a298) {
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
    if f9(v1.clone(), v2.clone(), v3.clone(), v6, v6, v4, v5) {
        let m312 = f11(v0, 0u64, 0u64, match v3.clone() {
            Adt1::C0(h299, _, _, _, _, _, _, _, _) => {
                h299
            }
        }, match v1.clone() {
            Adt1::C0(_, h300, _, _, _, _, _, _, _) => {
                h300
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h301, _, _, _, _, _, _) => {
                h301
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h302, _, _, _, _, _) => {
                h302
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h303, _, _, _, _) => {
                h303
            }
        }, match v2.clone() {
            Adt1::C0(_, h304, _, _, _, _, _, _, _) => {
                h304
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h305, _, _, _, _, _, _) => {
                h305
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h306, _, _, _, _, _) => {
                h306
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h307, _, _, _, _) => {
                h307
            }
        }, match v3.clone() {
            Adt1::C0(_, h308, _, _, _, _, _, _, _) => {
                h308
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h309, _, _, _, _, _, _) => {
                h309
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h310, _, _, _, _, _) => {
                h310
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h311, _, _, _, _) => {
                h311
            }
        })?;
        match m312 {
            Adt7::C0 => {
                Ok(Adt4::C0)
            }
            Adt7::C1 => {
                Ok(Adt4::C0)
            }
            Adt7::C2(v7, _, v9, v10, v11, v12, v13, _, _, _) => {
                Ok(Adt4::C1(f10(v1, v2, v3, v4, v5, v7, v9, v10, v11, v12, v13, None::<Adt3>, v6)?))
            }
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f7(v0: u64) -> bool {
    let a313 = v0;
    let a314 = 0u64;
    if if nat_eq(a313, a314) {
        true
    } else {
        let a315 = v0;
        let a316 = 1u64;
        nat_eq(a315, a316)
    } {
        true
    } else {
        let a317 = v0;
        let a318 = 30u64;
        nat_eq(a317, a318)
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64, v3: bool) -> bool {
    if v3 {
        false
    } else {
        if f12(v0) {
            let a319 = v1;
            let a320 = 15u64;
            if nat_eq(a319, a320) {
                let a321 = v2;
                let a322 = 292u64;
                nat_eq(a321, a322)
            } else {
                false
            }
        } else {
            if f13(v0) {
                let a323 = v1;
                let a324 = 9u64;
                if nat_eq(a323, a324) {
                    let a325 = v2;
                    let a326 = 36u64;
                    nat_eq(a325, a326)
                } else {
                    false
                }
            } else {
                if if f14(v0) {
                    true
                } else {
                    f15(v0)
                } {
                    let a327 = v1;
                    let a328 = 8u64;
                    if nat_eq(a327, a328) {
                        let a329 = v2;
                        let a330 = 34u64;
                        nat_eq(a329, a330)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
}

pub fn f9(v0: Adt1, v1: Adt1, v2: Adt1, v3: bool, v4: bool, v5: u64, v6: u64) -> bool {
    if if if if if if if if f16(match v0.clone() {
        Adt1::C0(h331, _, _, _, _, _, _, _, _) => {
            h331
        }
    }) {
        true
    } else {
        if v3 {
            if f7(match v0.clone() {
                Adt1::C0(h332, _, _, _, _, _, _, _, _) => {
                    h332
                }
            }) {
                true
            } else {
                let a334 = match v0.clone() {
                    Adt1::C0(h333, _, _, _, _, _, _, _, _) => {
                        h333
                    }
                };
                let a335 = 43u64;
                nat_eq(a334, a335)
            }
        } else {
            false
        }
    } {
        let a340 = {
            let a337 = match v0.clone() {
                Adt1::C0(_, h336, _, _, _, _, _, _, _) => {
                    h336
                }
            };
            let a338 = v5;
            let a339 = 0u64;
            nat_rem(a337, a338, a339)
        };
        let a341 = 0u64;
        nat_eq(a340, a341)
    } else {
        false
    } {
        let a343 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h342, _, _, _) => {
                h342
            }
        };
        let a344 = v6;
        nat_eq(a343, a344)
    } else {
        false
    } {
        if f17(match v1.clone() {
            Adt1::C0(h345, _, _, _, _, _, _, _, _) => {
                h345
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, h346, _, _, _) => {
                h346
            }
        }, v4) {
            true
        } else {
            f8(match v0 {
                Adt1::C0(h347, _, _, _, _, _, _, _, _) => {
                    h347
                }
            }, match v1.clone() {
                Adt1::C0(h348, _, _, _, _, _, _, _, _) => {
                    h348
                }
            }, match v1 {
                Adt1::C0(_, _, _, _, _, h349, _, _, _) => {
                    h349
                }
            }, v4)
        }
    } else {
        false
    } {
        let a351 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h350, _, _, _) => {
                h350
            }
        };
        let a352 = 4u64;
        nat_eq(a351, a352)
    } else {
        false
    } {
        let a355 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h353, _, _, _) => {
                h353
            }
        };
        let a356 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h354, _, _) => {
                h354
            }
        };
        nat_le(a355, a356)
    } else {
        false
    } {
        let a359 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h357, _, _) => {
                h357
            }
        };
        let a360 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h358, _) => {
                h358
            }
        };
        nat_le(a359, a360)
    } else {
        false
    } {
        let a363 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h361, _) => {
                h361
            }
        };
        let a364 = match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h362) => {
                h362
            }
        };
        nat_le(a363, a364)
    } else {
        false
    }
}

pub fn f10(v0: Adt1, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: Option<Adt3>, v12: bool) -> R<Adt2> {
    Ok(Adt2::C0(if v12 {
        match v0.clone() {
            Adt1::C0(_, h365, _, _, _, _, _, _, _) => {
                h365
            }
        }
    } else {
        let a368 = match v0.clone() {
            Adt1::C0(_, h366, _, _, _, _, _, _, _) => {
                h366
            }
        };
        let a369 = f18(match v0.clone() {
            Adt1::C0(h367, _, _, _, _, _, _, _, _) => {
                h367
            }
        }, v3);
        let a370 = 0u64;
        nat_quot(a368, a369, a370)
    }, {
        let a375 = {
            let a372 = match v0.clone() {
                Adt1::C0(_, h371, _, _, _, _, _, _, _) => {
                    h371
                }
            };
            let a373 = v3;
            let a374 = 0u64;
            nat_quot(a372, a373, a374)
        };
        let a376 = v4;
        nat_mul(a375, a376)?
    }, v5, v6, v7, {
        let a379 = v6;
        let a380 = {
            let a377 = v7;
            let a378 = v8;
            nat_mul(a377, a378)?
        };
        nat_mul(a379, a380)?
    }, {
        let a385 = v5;
        let a386 = {
            let a383 = v6;
            let a384 = {
                let a381 = v7;
                let a382 = v8;
                nat_mul(a381, a382)?
            };
            nat_mul(a383, a384)?
        };
        nat_mul(a385, a386)?
    }, v9, v10, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h387, _, _) => {
            h387
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h388, _) => {
            h388
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h389) => {
            h389
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h390, _, _) => {
            h390
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h391, _) => {
            h391
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h392) => {
            h392
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h393, _, _) => {
            h393
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h394, _) => {
            h394
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h395) => {
            h395
        }
    }, v11, if v12 {
        Some(0u64)
    } else {
        if if f7(match v0.clone() {
            Adt1::C0(h396, _, _, _, _, _, _, _, _) => {
                h396
            }
        }) {
            true
        } else {
            let a398 = match v0.clone() {
                Adt1::C0(h397, _, _, _, _, _, _, _, _) => {
                    h397
                }
            };
            let a399 = 43u64;
            nat_eq(a398, a399)
        } {
            Some(f19(match v0.clone() {
                Adt1::C0(h400, _, _, _, _, _, _, _, _) => {
                    h400
                }
            }))
        } else {
            None::<u64>
        }
    }, match v0 {
        Adt1::C0(h401, _, _, _, _, _, _, _, _) => {
            h401
        }
    }))
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt7> {
    let a402 = v0;
    let a403 = 28u64;
    if if if if nat_eq(a402, a403) {
        let a404 = v1;
        let a405 = 0u64;
        nat_eq(a404, a405)
    } else {
        false
    } {
        let a406 = v2;
        let a407 = 0u64;
        nat_eq(a406, a407)
    } else {
        false
    } {
        let a408 = v3;
        let a409 = 0u64;
        nat_eq(a408, a409)
    } else {
        false
    } {
        let a410 = v4;
        let a411 = v8;
        if if if if if if if if nat_eq(a410, a411) {
            let a412 = v5;
            let a413 = v12;
            nat_eq(a412, a413)
        } else {
            false
        } {
            let a414 = v9;
            let a415 = v13;
            nat_eq(a414, a415)
        } else {
            false
        } {
            let a416 = v10;
            let a417 = v14;
            nat_eq(a416, a417)
        } else {
            false
        } {
            let a418 = v11;
            let a419 = v15;
            nat_eq(a418, a419)
        } else {
            false
        } {
            let a420 = 0u64;
            let a421 = v6;
            nat_lt(a420, a421)
        } else {
            false
        } {
            let a422 = 0u64;
            let a423 = v7;
            nat_lt(a422, a423)
        } else {
            false
        } {
            let a427 = {
                let a424 = v10;
                let a425 = v6;
                let a426 = 1u64;
                nat_rem(a424, a425, a426)
            };
            let a428 = 0u64;
            if nat_eq(a427, a428) {
                let a432 = {
                    let a429 = v11;
                    let a430 = v7;
                    let a431 = 1u64;
                    nat_rem(a429, a430, a431)
                };
                let a433 = 0u64;
                nat_eq(a432, a433)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt7::C2(v5, v4, v9, v10, v11, {
                let a434 = v10;
                let a435 = v6;
                let a436 = 0u64;
                nat_quot(a434, a435, a436)
            }, {
                let a437 = v11;
                let a438 = v7;
                let a439 = 0u64;
                nat_quot(a437, a438, a439)
            }, {
                let a440 = v5;
                let a441 = v4;
                nat_mul(a440, a441)?
            }, {
                let a442 = v4;
                let a443 = v9;
                nat_mul(a442, a443)?
            }, {
                let a444 = v5;
                let a445 = v9;
                nat_mul(a444, a445)?
            }))
        } else {
            Ok(Adt7::C1)
        }
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f12(v0: u64) -> bool {
    let a446 = 10u64;
    let a447 = v0;
    if if if if if nat_le(a446, a447) {
        let a448 = v0;
        let a449 = 14u64;
        nat_le(a448, a449)
    } else {
        false
    } {
        true
    } else {
        let a450 = 16u64;
        let a451 = v0;
        if nat_le(a450, a451) {
            let a452 = v0;
            let a453 = 19u64;
            nat_le(a452, a453)
        } else {
            false
        }
    } {
        true
    } else {
        let a454 = 21u64;
        let a455 = v0;
        if nat_le(a454, a455) {
            let a456 = v0;
            let a457 = 23u64;
            nat_le(a456, a457)
        } else {
            false
        }
    } {
        true
    } else {
        let a458 = 34u64;
        let a459 = v0;
        if nat_le(a458, a459) {
            let a460 = v0;
            let a461 = 35u64;
            nat_le(a460, a461)
        } else {
            false
        }
    } {
        true
    } else {
        let a462 = v0;
        let a463 = 29u64;
        nat_eq(a462, a463)
    }
}

pub fn f13(v0: u64) -> bool {
    let a464 = v0;
    let a465 = 3u64;
    if nat_eq(a464, a465) {
        true
    } else {
        let a466 = v0;
        let a467 = 7u64;
        nat_eq(a466, a467)
    }
}

pub fn f14(v0: u64) -> bool {
    let a468 = v0;
    let a469 = 2u64;
    if if if if nat_eq(a468, a469) {
        true
    } else {
        let a470 = v0;
        let a471 = 6u64;
        nat_eq(a470, a471)
    } {
        true
    } else {
        let a472 = v0;
        let a473 = 8u64;
        nat_eq(a472, a473)
    } {
        true
    } else {
        let a474 = v0;
        let a475 = 20u64;
        nat_eq(a474, a475)
    } {
        true
    } else {
        f20(v0)
    }
}

pub fn f15(v0: u64) -> bool {
    let a476 = v0;
    let a477 = 39u64;
    if nat_eq(a476, a477) {
        true
    } else {
        let a478 = v0;
        let a479 = 40u64;
        nat_eq(a478, a479)
    }
}

pub fn f16(v0: u64) -> bool {
    if if f21(v0) {
        true
    } else {
        f15(v0)
    } {
        true
    } else {
        f12(v0)
    }
}

pub fn f17(v0: u64, v1: u64, v2: bool) -> bool {
    let a480 = v0;
    let a481 = 0u64;
    if if nat_eq(a480, a481) {
        let a482 = v1;
        let a483 = 4u64;
        nat_eq(a482, a483)
    } else {
        false
    } {
        true
    } else {
        let a484 = v0;
        let a485 = 1u64;
        if if nat_eq(a484, a485) {
            true
        } else {
            if v2 {
                let a486 = v0;
                let a487 = 30u64;
                nat_eq(a486, a487)
            } else {
                false
            }
        } {
            let a488 = v1;
            let a489 = 2u64;
            nat_eq(a488, a489)
        } else {
            false
        }
    }
}

pub fn f18(v0: u64, v1: u64) -> u64 {
    if if f15(v0) {
        true
    } else {
        f20(v0)
    } {
        32u64
    } else {
        v1
    }
}

pub fn f19(v0: u64) -> u64 {
    let a490 = v0;
    let a491 = 30u64;
    if nat_eq(a490, a491) {
        30u64
    } else {
        0u64
    }
}

pub fn f20(v0: u64) -> bool {
    let a492 = v0;
    let a493 = 41u64;
    if nat_eq(a492, a493) {
        true
    } else {
        let a494 = v0;
        let a495 = 42u64;
        nat_eq(a494, a495)
    }
}

pub fn f21(v0: u64) -> bool {
    if f14(v0) {
        true
    } else {
        f13(v0)
    }
}

pub fn run(p0: Adt0) -> R<Adt4> {
    f0(p0)
}
