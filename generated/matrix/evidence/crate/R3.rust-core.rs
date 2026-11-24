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
            f1(v1, v2, v3.clone(), v4.clone(), v5, f2(v3, v4, v6))
        }
        Adt0::C1(v7, v8) => {
            let m3 = match v7.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2, _, _) => {
                    h2
                }
            };
            match m3 {
                None => {
                    f3(v7, v8)
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
            f4(v10, v11, v12.clone(), v13.clone(), v14, v15, f2(v12, v13, v16))
        }
        Adt0::C4(v17, v18, v19) => {
            f5(v17, v18, v19)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: bool) -> R<Adt4> {
    if v1 {
        Ok(Adt4::C0)
    } else {
        let m32 = f6(match v2.clone() {
            Adt1::C0(h31, _, _, _, _, _, _, _, _) => {
                h31
            }
        });
        match m32 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v6) => {
                f7(v0, v2, v3, v4, match v6.clone() {
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

pub fn f2(v0: Adt1, v1: Adt1, v2: bool) -> bool {
    if v2 {
        !f8(match v0 {
            Adt1::C0(h35, _, _, _, _, _, _, _, _) => {
                h35
            }
        }, match v1.clone() {
            Adt1::C0(h36, _, _, _, _, _, _, _, _) => {
                h36
            }
        }, match v1 {
            Adt1::C0(_, _, _, _, _, h37, _, _, _) => {
                h37
            }
        })
    } else {
        false
    }
}

pub fn f3(v0: Adt2, v1: u64) -> R<Adt4> {
    let a39 = v1;
    let a40 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h38, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h38
        }
    };
    if nat_lt(a39, a40) {
        let v2: u64 = {
            let a42 = v1;
            let a43 = match v0.clone() {
                Adt2::C0(_, _, _, h41, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h41
                }
            };
            let a44 = 0u64;
            nat_rem(a42, a43, a44)
        };
        let v3: u64 = {
            let a46 = v1;
            let a47 = match v0.clone() {
                Adt2::C0(_, _, _, h45, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h45
                }
            };
            let a48 = 0u64;
            nat_quot(a46, a47, a48)
        };
        let v4: u64 = {
            let a50 = v3;
            let a51 = match v0.clone() {
                Adt2::C0(_, _, _, _, h49, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h49
                }
            };
            let a52 = 0u64;
            nat_rem(a50, a51, a52)
        };
        let v5: u64 = {
            let a54 = v3;
            let a55 = match v0.clone() {
                Adt2::C0(_, _, _, _, h53, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h53
                }
            };
            let a56 = 0u64;
            nat_quot(a54, a55, a56)
        };
        Ok(Adt4::C2({
            let a71 = {
                let a62 = {
                    let a58 = v4;
                    let a59 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h57, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h57
                        }
                    };
                    let a60 = 0u64;
                    nat_quot(a58, a59, a60)
                };
                let a63 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h61, _, _, _, _, _, _, _, _, _, _) => {
                        h61
                    }
                };
                nat_mul(a62, a63)?
            };
            let a72 = {
                let a69 = {
                    let a65 = v5;
                    let a66 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h64, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h64
                        }
                    };
                    let a67 = 0u64;
                    nat_quot(a65, a66, a67)
                };
                let a70 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h68, _, _, _, _, _, _, _, _, _) => {
                        h68
                    }
                };
                nat_mul(a69, a70)?
            };
            nat_add(a71, a72)?
        }, {
            let a84 = {
                let a79 = {
                    let a74 = v2;
                    let a75 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h73, _, _, _, _, _, _, _, _) => {
                            h73
                        }
                    };
                    nat_mul(a74, a75)?
                };
                let a80 = {
                    let a77 = v4;
                    let a78 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h76, _, _, _, _, _, _, _) => {
                            h76
                        }
                    };
                    nat_mul(a77, a78)?
                };
                nat_add(a79, a80)?
            };
            let a85 = {
                let a82 = v5;
                let a83 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h81, _, _, _, _, _, _) => {
                        h81
                    }
                };
                nat_mul(a82, a83)?
            };
            nat_add(a84, a85)?
        }, {
            let a97 = {
                let a92 = {
                    let a87 = v2;
                    let a88 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h86, _, _, _, _, _) => {
                            h86
                        }
                    };
                    nat_mul(a87, a88)?
                };
                let a93 = {
                    let a90 = v4;
                    let a91 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h89, _, _, _, _) => {
                            h89
                        }
                    };
                    nat_mul(a90, a91)?
                };
                nat_add(a92, a93)?
            };
            let a98 = {
                let a95 = v5;
                let a96 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h94, _, _, _) => {
                        h94
                    }
                };
                nat_mul(a95, a96)?
            };
            nat_add(a97, a98)?
        }))
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f4(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: bool) -> R<Adt4> {
    let a99 = v0;
    let a100 = 29u64;
    if if if if if if if if if if if if if if if if if if if if nat_eq(a99, a100) {
        !v1
    } else {
        false
    } {
        let a102 = match v4.clone() {
            Adt1::C0(h101, _, _, _, _, _, _, _, _) => {
                h101
            }
        };
        let a103 = 26u64;
        nat_eq(a102, a103)
    } else {
        false
    } {
        let a105 = match v3.clone() {
            Adt1::C0(h104, _, _, _, _, _, _, _, _) => {
                h104
            }
        };
        let a106 = 0u64;
        if if nat_eq(a105, a106) {
            true
        } else {
            if v6 {
                f9(match v3.clone() {
                    Adt1::C0(h107, _, _, _, _, _, _, _, _) => {
                        h107
                    }
                })
            } else {
                false
            }
        } {
            true
        } else {
            f8(match v2.clone() {
                Adt1::C0(h108, _, _, _, _, _, _, _, _) => {
                    h108
                }
            }, match v3.clone() {
                Adt1::C0(h109, _, _, _, _, _, _, _, _) => {
                    h109
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h110, _, _, _) => {
                    h110
                }
            })
        }
    } else {
        false
    } {
        let a112 = match v5.clone() {
            Adt1::C0(h111, _, _, _, _, _, _, _, _) => {
                h111
            }
        };
        let a113 = 0u64;
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a115 = match v2.clone() {
            Adt1::C0(_, _, _, _, h114, _, _, _, _) => {
                h114
            }
        };
        let a116 = 1u64;
        nat_eq(a115, a116)
    } else {
        false
    } {
        let a118 = match v3.clone() {
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
            Adt1::C0(_, _, _, _, h120, _, _, _, _) => {
                h120
            }
        };
        let a122 = 1u64;
        nat_eq(a121, a122)
    } else {
        false
    } {
        let a124 = match v5.clone() {
            Adt1::C0(_, _, _, _, h123, _, _, _, _) => {
                h123
            }
        };
        let a125 = 1u64;
        nat_eq(a124, a125)
    } else {
        false
    } {
        let a127 = match v4.clone() {
            Adt1::C0(_, _, _, h126, _, _, _, _, _) => {
                h126
            }
        };
        let a128 = 1u64;
        nat_eq(a127, a128)
    } else {
        false
    } {
        let a131 = match v4.clone() {
            Adt1::C0(_, _, h129, _, _, _, _, _, _) => {
                h129
            }
        };
        let a132 = match v3.clone() {
            Adt1::C0(_, _, _, h130, _, _, _, _, _) => {
                h130
            }
        };
        nat_eq(a131, a132)
    } else {
        false
    } {
        let a135 = match v2.clone() {
            Adt1::C0(_, h133, _, _, _, _, _, _, _) => {
                h133
            }
        };
        let a136 = match v3.clone() {
            Adt1::C0(_, h134, _, _, _, _, _, _, _) => {
                h134
            }
        };
        nat_eq(a135, a136)
    } else {
        false
    } {
        let a138 = 0u64;
        let a139 = match v3.clone() {
            Adt1::C0(_, _, h137, _, _, _, _, _, _) => {
                h137
            }
        };
        nat_lt(a138, a139)
    } else {
        false
    } {
        let a145 = {
            let a142 = match v4.clone() {
                Adt1::C0(_, h140, _, _, _, _, _, _, _) => {
                    h140
                }
            };
            let a143 = match v3.clone() {
                Adt1::C0(_, _, h141, _, _, _, _, _, _) => {
                    h141
                }
            };
            let a144 = 0u64;
            nat_rem(a142, a143, a144)
        };
        let a146 = 0u64;
        nat_eq(a145, a146)
    } else {
        false
    } {
        let a149 = match v5.clone() {
            Adt1::C0(_, h147, _, _, _, _, _, _, _) => {
                h147
            }
        };
        let a150 = match v2.clone() {
            Adt1::C0(_, _, h148, _, _, _, _, _, _) => {
                h148
            }
        };
        nat_eq(a149, a150)
    } else {
        false
    } {
        let a153 = match v5.clone() {
            Adt1::C0(_, _, h151, _, _, _, _, _, _) => {
                h151
            }
        };
        let a154 = match v4.clone() {
            Adt1::C0(_, h152, _, _, _, _, _, _, _) => {
                h152
            }
        };
        nat_eq(a153, a154)
    } else {
        false
    } {
        let a157 = match v5.clone() {
            Adt1::C0(_, _, _, h155, _, _, _, _, _) => {
                h155
            }
        };
        let a158 = match v3.clone() {
            Adt1::C0(_, _, _, h156, _, _, _, _, _) => {
                h156
            }
        };
        nat_eq(a157, a158)
    } else {
        false
    } {
        let a161 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h159, _, _, _) => {
                h159
            }
        };
        let a162 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h160, _, _) => {
                h160
            }
        };
        nat_le(a161, a162)
    } else {
        false
    } {
        let a167 = {
            let a164 = match v5.clone() {
                Adt1::C0(_, h163, _, _, _, _, _, _, _) => {
                    h163
                }
            };
            let a165 = 4u64;
            nat_mul(a164, a165)?
        };
        let a168 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, h166, _, _) => {
                h166
            }
        };
        nat_le(a167, a168)
    } else {
        false
    } {
        let a174 = {
            let a171 = match v5.clone() {
                Adt1::C0(_, _, h169, _, _, _, _, _, _) => {
                    h169
                }
            };
            let a172 = match v5.clone() {
                Adt1::C0(_, _, _, _, _, _, h170, _, _) => {
                    h170
                }
            };
            nat_mul(a171, a172)?
        };
        let a175 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h173, _) => {
                h173
            }
        };
        nat_le(a174, a175)
    } else {
        false
    } {
        let m177 = f6(match v2.clone() {
            Adt1::C0(h176, _, _, _, _, _, _, _, _) => {
                h176
            }
        });
        match m177 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v7) => {
                if f10(v2.clone(), v3.clone(), v5.clone(), true, v6, match v7.clone() {
                    Adt6::C0(_, h178, _) => {
                        h178
                    }
                }, match v7.clone() {
                    Adt6::C0(_, _, h179) => {
                        h179
                    }
                }) {
                    Ok(Adt4::C1(f11(v2.clone(), v3.clone(), v5, match v7.clone() {
                        Adt6::C0(_, h180, _) => {
                            h180
                        }
                    }, match v7 {
                        Adt6::C0(_, _, h181) => {
                            h181
                        }
                    }, match v2.clone() {
                        Adt1::C0(_, _, h182, _, _, _, _, _, _) => {
                            h182
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h183, _, _, _, _, _, _, _) => {
                            h183
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h184, _, _, _, _, _, _) => {
                            h184
                        }
                    }, 1u64, 1u64, 1u64, Some(Adt3::C0(match v2 {
                        Adt1::C0(_, _, _, h185, _, _, _, _, _) => {
                            h185
                        }
                    }, match v3 {
                        Adt1::C0(_, _, h186, _, _, _, _, _, _) => {
                            h186
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h187, _, _, _) => {
                            h187
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h188, _, _) => {
                            h188
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

pub fn f5(v0: Adt2, v1: u64, v2: u64) -> R<Adt4> {
    let m190 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h189, _, _) => {
            h189
        }
    };
    match m190 {
        None => {
            Ok(Adt4::C0)
        }
        Some(v3) => {
            let a192 = v1;
            let a193 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h191, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h191
                }
            };
            if nat_lt(a192, a193) {
                let v4: u64 = {
                    let a195 = v1;
                    let a196 = match v0.clone() {
                        Adt2::C0(_, _, _, h194, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h194
                        }
                    };
                    let a197 = 0u64;
                    nat_rem(a195, a196, a197)
                };
                let v5: u64 = {
                    let a199 = v1;
                    let a200 = match v0.clone() {
                        Adt2::C0(_, _, _, h198, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h198
                        }
                    };
                    let a201 = 0u64;
                    nat_quot(a199, a200, a201)
                };
                let v6: u64 = {
                    let a208 = {
                        let a203 = v4;
                        let a204 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h202, _, _, _, _, _) => {
                                h202
                            }
                        };
                        nat_mul(a203, a204)?
                    };
                    let a209 = {
                        let a206 = v5;
                        let a207 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h205, _, _, _, _) => {
                                h205
                            }
                        };
                        nat_mul(a206, a207)?
                    };
                    nat_add(a208, a209)?
                };
                let a210 = v2;
                let a211 = 4294967295u64;
                if nat_eq(a210, a211) {
                    Ok(Adt4::C5(v6))
                } else {
                    let a213 = v2;
                    let a214 = match v3.clone() {
                        Adt3::C0(h212, _, _, _) => {
                            h212
                        }
                    };
                    if nat_lt(a213, a214) {
                        Ok(Adt4::C2({
                            let a216 = v2;
                            let a217 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h215, _, _, _, _, _, _, _, _, _, _) => {
                                    h215
                                }
                            };
                            nat_mul(a216, a217)?
                        }, {
                            let a228 = {
                                let a223 = {
                                    let a219 = v4;
                                    let a220 = match v3 {
                                        Adt3::C0(_, h218, _, _) => {
                                            h218
                                        }
                                    };
                                    let a221 = 0u64;
                                    nat_rem(a219, a220, a221)
                                };
                                let a224 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h222, _, _, _, _, _, _, _, _) => {
                                        h222
                                    }
                                };
                                nat_mul(a223, a224)?
                            };
                            let a229 = {
                                let a226 = v5;
                                let a227 = match v0 {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h225, _, _, _, _, _, _, _) => {
                                        h225
                                    }
                                };
                                nat_mul(a226, a227)?
                            };
                            nat_add(a228, a229)?
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

pub fn f6(v0: u64) -> Adt5 {
    let a230 = v0;
    let a231 = 0u64;
    if nat_eq(a230, a231) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a232 = v0;
        let a233 = 1u64;
        if nat_eq(a232, a233) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a234 = v0;
            let a235 = 2u64;
            if nat_eq(a234, a235) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a236 = v0;
                let a237 = 3u64;
                if nat_eq(a236, a237) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a238 = v0;
                    let a239 = 6u64;
                    if nat_eq(a238, a239) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a240 = v0;
                        let a241 = 7u64;
                        if nat_eq(a240, a241) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a242 = v0;
                            let a243 = 8u64;
                            if nat_eq(a242, a243) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a244 = v0;
                                let a245 = 9u64;
                                if nat_eq(a244, a245) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a246 = v0;
                                    let a247 = 10u64;
                                    if nat_eq(a246, a247) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a248 = v0;
                                        let a249 = 11u64;
                                        if nat_eq(a248, a249) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a250 = v0;
                                            let a251 = 12u64;
                                            if nat_eq(a250, a251) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a252 = v0;
                                                let a253 = 13u64;
                                                if nat_eq(a252, a253) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a254 = v0;
                                                    let a255 = 14u64;
                                                    if nat_eq(a254, a255) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a256 = v0;
                                                        let a257 = 15u64;
                                                        if nat_eq(a256, a257) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a258 = v0;
                                                            let a259 = 16u64;
                                                            if nat_eq(a258, a259) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a260 = v0;
                                                                let a261 = 17u64;
                                                                if nat_eq(a260, a261) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a262 = v0;
                                                                    let a263 = 18u64;
                                                                    if nat_eq(a262, a263) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a264 = v0;
                                                                        let a265 = 19u64;
                                                                        if nat_eq(a264, a265) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a266 = v0;
                                                                            let a267 = 20u64;
                                                                            if nat_eq(a266, a267) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a268 = v0;
                                                                                let a269 = 21u64;
                                                                                if nat_eq(a268, a269) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a270 = v0;
                                                                                    let a271 = 22u64;
                                                                                    if nat_eq(a270, a271) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a272 = v0;
                                                                                        let a273 = 23u64;
                                                                                        if nat_eq(a272, a273) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a274 = v0;
                                                                                            let a275 = 24u64;
                                                                                            if nat_eq(a274, a275) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a276 = v0;
                                                                                                let a277 = 25u64;
                                                                                                if nat_eq(a276, a277) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a278 = v0;
                                                                                                    let a279 = 26u64;
                                                                                                    if nat_eq(a278, a279) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a280 = v0;
                                                                                                        let a281 = 27u64;
                                                                                                        if nat_eq(a280, a281) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a282 = v0;
                                                                                                            let a283 = 28u64;
                                                                                                            if nat_eq(a282, a283) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a284 = v0;
                                                                                                                let a285 = 29u64;
                                                                                                                if nat_eq(a284, a285) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a286 = v0;
                                                                                                                    let a287 = 30u64;
                                                                                                                    if nat_eq(a286, a287) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a288 = v0;
                                                                                                                        let a289 = 34u64;
                                                                                                                        if nat_eq(a288, a289) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a290 = v0;
                                                                                                                            let a291 = 35u64;
                                                                                                                            if nat_eq(a290, a291) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a292 = v0;
                                                                                                                                let a293 = 39u64;
                                                                                                                                if nat_eq(a292, a293) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a294 = v0;
                                                                                                                                    let a295 = 40u64;
                                                                                                                                    if nat_eq(a294, a295) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a296 = v0;
                                                                                                                                        let a297 = 41u64;
                                                                                                                                        if nat_eq(a296, a297) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a298 = v0;
                                                                                                                                            let a299 = 42u64;
                                                                                                                                            if nat_eq(a298, a299) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a300 = v0;
                                                                                                                                                let a301 = 43u64;
                                                                                                                                                if nat_eq(a300, a301) {
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

pub fn f7(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64, v5: u64, v6: bool) -> R<Adt4> {
    if f10(v1.clone(), v2.clone(), v3.clone(), v6, v6, v4, v5) {
        let m315 = f12(v0, 0u64, 0u64, match v3.clone() {
            Adt1::C0(h302, _, _, _, _, _, _, _, _) => {
                h302
            }
        }, match v1.clone() {
            Adt1::C0(_, h303, _, _, _, _, _, _, _) => {
                h303
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h304, _, _, _, _, _, _) => {
                h304
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h305, _, _, _, _, _) => {
                h305
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h306, _, _, _, _) => {
                h306
            }
        }, match v2.clone() {
            Adt1::C0(_, h307, _, _, _, _, _, _, _) => {
                h307
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h308, _, _, _, _, _, _) => {
                h308
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h309, _, _, _, _, _) => {
                h309
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h310, _, _, _, _) => {
                h310
            }
        }, match v3.clone() {
            Adt1::C0(_, h311, _, _, _, _, _, _, _) => {
                h311
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h312, _, _, _, _, _, _) => {
                h312
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h313, _, _, _, _, _) => {
                h313
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h314, _, _, _, _) => {
                h314
            }
        })?;
        match m315 {
            Adt7::C0 => {
                Ok(Adt4::C0)
            }
            Adt7::C1 => {
                Ok(Adt4::C0)
            }
            Adt7::C2(v7, _, v9, v10, v11, v12, v13, _, _, _) => {
                Ok(Adt4::C1(f11(v1, v2, v3, v4, v5, v7, v9, v10, v11, v12, v13, None::<Adt3>, v6)?))
            }
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> bool {
    if f13(v0) {
        let a316 = v1;
        let a317 = 15u64;
        if nat_eq(a316, a317) {
            let a318 = v2;
            let a319 = 292u64;
            nat_eq(a318, a319)
        } else {
            false
        }
    } else {
        if f14(v0) {
            let a320 = v1;
            let a321 = 9u64;
            if nat_eq(a320, a321) {
                let a322 = v2;
                let a323 = 36u64;
                nat_eq(a322, a323)
            } else {
                false
            }
        } else {
            if if f15(v0) {
                true
            } else {
                f16(v0)
            } {
                let a324 = v1;
                let a325 = 8u64;
                if nat_eq(a324, a325) {
                    let a326 = v2;
                    let a327 = 34u64;
                    nat_eq(a326, a327)
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f9(v0: u64) -> bool {
    let a328 = v0;
    let a329 = 0u64;
    if if nat_eq(a328, a329) {
        true
    } else {
        let a330 = v0;
        let a331 = 1u64;
        nat_eq(a330, a331)
    } {
        true
    } else {
        let a332 = v0;
        let a333 = 30u64;
        nat_eq(a332, a333)
    }
}

pub fn f10(v0: Adt1, v1: Adt1, v2: Adt1, v3: bool, v4: bool, v5: u64, v6: u64) -> bool {
    if if if if if if if if f17(match v0.clone() {
        Adt1::C0(h334, _, _, _, _, _, _, _, _) => {
            h334
        }
    }) {
        true
    } else {
        if v3 {
            if f9(match v0.clone() {
                Adt1::C0(h335, _, _, _, _, _, _, _, _) => {
                    h335
                }
            }) {
                true
            } else {
                let a337 = match v0.clone() {
                    Adt1::C0(h336, _, _, _, _, _, _, _, _) => {
                        h336
                    }
                };
                let a338 = 43u64;
                nat_eq(a337, a338)
            }
        } else {
            false
        }
    } {
        let a343 = {
            let a340 = match v0.clone() {
                Adt1::C0(_, h339, _, _, _, _, _, _, _) => {
                    h339
                }
            };
            let a341 = v5;
            let a342 = 0u64;
            nat_rem(a340, a341, a342)
        };
        let a344 = 0u64;
        nat_eq(a343, a344)
    } else {
        false
    } {
        let a346 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h345, _, _, _) => {
                h345
            }
        };
        let a347 = v6;
        nat_eq(a346, a347)
    } else {
        false
    } {
        if f18(match v1.clone() {
            Adt1::C0(h348, _, _, _, _, _, _, _, _) => {
                h348
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, h349, _, _, _) => {
                h349
            }
        }, v4) {
            true
        } else {
            f8(match v0 {
                Adt1::C0(h350, _, _, _, _, _, _, _, _) => {
                    h350
                }
            }, match v1.clone() {
                Adt1::C0(h351, _, _, _, _, _, _, _, _) => {
                    h351
                }
            }, match v1 {
                Adt1::C0(_, _, _, _, _, h352, _, _, _) => {
                    h352
                }
            })
        }
    } else {
        false
    } {
        let a354 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h353, _, _, _) => {
                h353
            }
        };
        let a355 = 4u64;
        nat_eq(a354, a355)
    } else {
        false
    } {
        let a358 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h356, _, _, _) => {
                h356
            }
        };
        let a359 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h357, _, _) => {
                h357
            }
        };
        nat_le(a358, a359)
    } else {
        false
    } {
        let a362 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h360, _, _) => {
                h360
            }
        };
        let a363 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h361, _) => {
                h361
            }
        };
        nat_le(a362, a363)
    } else {
        false
    } {
        let a366 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h364, _) => {
                h364
            }
        };
        let a367 = match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h365) => {
                h365
            }
        };
        nat_le(a366, a367)
    } else {
        false
    }
}

pub fn f11(v0: Adt1, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: Option<Adt3>, v12: bool) -> R<Adt2> {
    Ok(Adt2::C0(if v12 {
        match v0.clone() {
            Adt1::C0(_, h368, _, _, _, _, _, _, _) => {
                h368
            }
        }
    } else {
        let a371 = match v0.clone() {
            Adt1::C0(_, h369, _, _, _, _, _, _, _) => {
                h369
            }
        };
        let a372 = f19(match v0.clone() {
            Adt1::C0(h370, _, _, _, _, _, _, _, _) => {
                h370
            }
        }, v3);
        let a373 = 0u64;
        nat_quot(a371, a372, a373)
    }, {
        let a378 = {
            let a375 = match v0.clone() {
                Adt1::C0(_, h374, _, _, _, _, _, _, _) => {
                    h374
                }
            };
            let a376 = v3;
            let a377 = 0u64;
            nat_quot(a375, a376, a377)
        };
        let a379 = v4;
        nat_mul(a378, a379)?
    }, v5, v6, v7, {
        let a382 = v6;
        let a383 = {
            let a380 = v7;
            let a381 = v8;
            nat_mul(a380, a381)?
        };
        nat_mul(a382, a383)?
    }, {
        let a388 = v5;
        let a389 = {
            let a386 = v6;
            let a387 = {
                let a384 = v7;
                let a385 = v8;
                nat_mul(a384, a385)?
            };
            nat_mul(a386, a387)?
        };
        nat_mul(a388, a389)?
    }, v9, v10, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h390, _, _) => {
            h390
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h391, _) => {
            h391
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h392) => {
            h392
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h393, _, _) => {
            h393
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h394, _) => {
            h394
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h395) => {
            h395
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h396, _, _) => {
            h396
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h397, _) => {
            h397
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h398) => {
            h398
        }
    }, v11, if v12 {
        Some(0u64)
    } else {
        if if f9(match v0.clone() {
            Adt1::C0(h399, _, _, _, _, _, _, _, _) => {
                h399
            }
        }) {
            true
        } else {
            let a401 = match v0.clone() {
                Adt1::C0(h400, _, _, _, _, _, _, _, _) => {
                    h400
                }
            };
            let a402 = 43u64;
            nat_eq(a401, a402)
        } {
            Some(f20(match v0.clone() {
                Adt1::C0(h403, _, _, _, _, _, _, _, _) => {
                    h403
                }
            }))
        } else {
            None::<u64>
        }
    }, match v0 {
        Adt1::C0(h404, _, _, _, _, _, _, _, _) => {
            h404
        }
    }))
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt7> {
    let a405 = v0;
    let a406 = 28u64;
    if if if if nat_eq(a405, a406) {
        let a407 = v1;
        let a408 = 0u64;
        nat_eq(a407, a408)
    } else {
        false
    } {
        let a409 = v2;
        let a410 = 0u64;
        nat_eq(a409, a410)
    } else {
        false
    } {
        let a411 = v3;
        let a412 = 0u64;
        nat_eq(a411, a412)
    } else {
        false
    } {
        let a413 = v4;
        let a414 = v8;
        if if if if if if if if nat_eq(a413, a414) {
            let a415 = v5;
            let a416 = v12;
            nat_eq(a415, a416)
        } else {
            false
        } {
            let a417 = v9;
            let a418 = v13;
            nat_eq(a417, a418)
        } else {
            false
        } {
            let a419 = v10;
            let a420 = v14;
            nat_eq(a419, a420)
        } else {
            false
        } {
            let a421 = v11;
            let a422 = v15;
            nat_eq(a421, a422)
        } else {
            false
        } {
            let a423 = 0u64;
            let a424 = v6;
            nat_lt(a423, a424)
        } else {
            false
        } {
            let a425 = 0u64;
            let a426 = v7;
            nat_lt(a425, a426)
        } else {
            false
        } {
            let a430 = {
                let a427 = v10;
                let a428 = v6;
                let a429 = 1u64;
                nat_rem(a427, a428, a429)
            };
            let a431 = 0u64;
            if nat_eq(a430, a431) {
                let a435 = {
                    let a432 = v11;
                    let a433 = v7;
                    let a434 = 1u64;
                    nat_rem(a432, a433, a434)
                };
                let a436 = 0u64;
                nat_eq(a435, a436)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt7::C2(v5, v4, v9, v10, v11, {
                let a437 = v10;
                let a438 = v6;
                let a439 = 0u64;
                nat_quot(a437, a438, a439)
            }, {
                let a440 = v11;
                let a441 = v7;
                let a442 = 0u64;
                nat_quot(a440, a441, a442)
            }, {
                let a443 = v5;
                let a444 = v4;
                nat_mul(a443, a444)?
            }, {
                let a445 = v4;
                let a446 = v9;
                nat_mul(a445, a446)?
            }, {
                let a447 = v5;
                let a448 = v9;
                nat_mul(a447, a448)?
            }))
        } else {
            Ok(Adt7::C1)
        }
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f13(v0: u64) -> bool {
    let a449 = 10u64;
    let a450 = v0;
    if if if if if nat_le(a449, a450) {
        let a451 = v0;
        let a452 = 14u64;
        nat_le(a451, a452)
    } else {
        false
    } {
        true
    } else {
        let a453 = 16u64;
        let a454 = v0;
        if nat_le(a453, a454) {
            let a455 = v0;
            let a456 = 19u64;
            nat_le(a455, a456)
        } else {
            false
        }
    } {
        true
    } else {
        let a457 = 21u64;
        let a458 = v0;
        if nat_le(a457, a458) {
            let a459 = v0;
            let a460 = 23u64;
            nat_le(a459, a460)
        } else {
            false
        }
    } {
        true
    } else {
        let a461 = 34u64;
        let a462 = v0;
        if nat_le(a461, a462) {
            let a463 = v0;
            let a464 = 35u64;
            nat_le(a463, a464)
        } else {
            false
        }
    } {
        true
    } else {
        let a465 = v0;
        let a466 = 29u64;
        nat_eq(a465, a466)
    }
}

pub fn f14(v0: u64) -> bool {
    let a467 = v0;
    let a468 = 3u64;
    if nat_eq(a467, a468) {
        true
    } else {
        let a469 = v0;
        let a470 = 7u64;
        nat_eq(a469, a470)
    }
}

pub fn f15(v0: u64) -> bool {
    let a471 = v0;
    let a472 = 2u64;
    if if if if nat_eq(a471, a472) {
        true
    } else {
        let a473 = v0;
        let a474 = 6u64;
        nat_eq(a473, a474)
    } {
        true
    } else {
        let a475 = v0;
        let a476 = 8u64;
        nat_eq(a475, a476)
    } {
        true
    } else {
        let a477 = v0;
        let a478 = 20u64;
        nat_eq(a477, a478)
    } {
        true
    } else {
        f21(v0)
    }
}

pub fn f16(v0: u64) -> bool {
    let a479 = v0;
    let a480 = 39u64;
    if nat_eq(a479, a480) {
        true
    } else {
        let a481 = v0;
        let a482 = 40u64;
        nat_eq(a481, a482)
    }
}

pub fn f17(v0: u64) -> bool {
    if if f22(v0) {
        true
    } else {
        f16(v0)
    } {
        true
    } else {
        f13(v0)
    }
}

pub fn f18(v0: u64, v1: u64, v2: bool) -> bool {
    let a483 = v0;
    let a484 = 0u64;
    if if nat_eq(a483, a484) {
        let a485 = v1;
        let a486 = 4u64;
        nat_eq(a485, a486)
    } else {
        false
    } {
        true
    } else {
        let a487 = v0;
        let a488 = 1u64;
        if if nat_eq(a487, a488) {
            true
        } else {
            if v2 {
                let a489 = v0;
                let a490 = 30u64;
                nat_eq(a489, a490)
            } else {
                false
            }
        } {
            let a491 = v1;
            let a492 = 2u64;
            nat_eq(a491, a492)
        } else {
            false
        }
    }
}

pub fn f19(v0: u64, v1: u64) -> u64 {
    if if f16(v0) {
        true
    } else {
        f21(v0)
    } {
        32u64
    } else {
        v1
    }
}

pub fn f20(v0: u64) -> u64 {
    let a493 = v0;
    let a494 = 30u64;
    if nat_eq(a493, a494) {
        30u64
    } else {
        0u64
    }
}

pub fn f21(v0: u64) -> bool {
    let a495 = v0;
    let a496 = 41u64;
    if nat_eq(a495, a496) {
        true
    } else {
        let a497 = v0;
        let a498 = 42u64;
        nat_eq(a497, a498)
    }
}

pub fn f22(v0: u64) -> bool {
    if f15(v0) {
        true
    } else {
        f14(v0)
    }
}
