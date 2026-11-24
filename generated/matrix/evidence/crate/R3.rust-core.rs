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
            f1(v1, v2, v3.clone(), v4.clone(), v5, f2(v1, v3, v4, v6))
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
            f4(v10, v11, v12.clone(), v13.clone(), v14, v15, f2(v10, v12, v13, v16))
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

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: bool) -> bool {
    let a36 = match v2.clone() {
        Adt1::C0(h35, _, _, _, _, _, _, _, _) => {
            h35
        }
    };
    let a37 = 30u64;
    if if nat_eq(a36, a37) {
        let a38 = v0;
        let a39 = 28u64;
        if if nat_eq(a38, a39) {
            let a41 = match v1.clone() {
                Adt1::C0(h40, _, _, _, _, _, _, _, _) => {
                    h40
                }
            };
            let a42 = 1u64;
            nat_eq(a41, a42)
        } else {
            false
        } {
            true
        } else {
            let a43 = v0;
            let a44 = 29u64;
            if nat_eq(a43, a44) {
                let a46 = match v1.clone() {
                    Adt1::C0(h45, _, _, _, _, _, _, _, _) => {
                        h45
                    }
                };
                let a47 = 30u64;
                nat_eq(a46, a47)
            } else {
                false
            }
        }
    } else {
        false
    } {
        true
    } else {
        if v3 {
            !f8(match v1 {
                Adt1::C0(h48, _, _, _, _, _, _, _, _) => {
                    h48
                }
            }, match v2.clone() {
                Adt1::C0(h49, _, _, _, _, _, _, _, _) => {
                    h49
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, _, h50, _, _, _) => {
                    h50
                }
            })
        } else {
            false
        }
    }
}

pub fn f3(v0: Adt2, v1: u64) -> R<Adt4> {
    let a52 = v1;
    let a53 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h51, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h51
        }
    };
    if nat_lt(a52, a53) {
        let v2: u64 = {
            let a55 = v1;
            let a56 = match v0.clone() {
                Adt2::C0(_, _, _, h54, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h54
                }
            };
            let a57 = 0u64;
            nat_rem(a55, a56, a57)
        };
        let v3: u64 = {
            let a59 = v1;
            let a60 = match v0.clone() {
                Adt2::C0(_, _, _, h58, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h58
                }
            };
            let a61 = 0u64;
            nat_quot(a59, a60, a61)
        };
        let v4: u64 = {
            let a63 = v3;
            let a64 = match v0.clone() {
                Adt2::C0(_, _, _, _, h62, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h62
                }
            };
            let a65 = 0u64;
            nat_rem(a63, a64, a65)
        };
        let v5: u64 = {
            let a67 = v3;
            let a68 = match v0.clone() {
                Adt2::C0(_, _, _, _, h66, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h66
                }
            };
            let a69 = 0u64;
            nat_quot(a67, a68, a69)
        };
        Ok(Adt4::C2({
            let a84 = {
                let a75 = {
                    let a71 = v4;
                    let a72 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h70, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h70
                        }
                    };
                    let a73 = 0u64;
                    nat_quot(a71, a72, a73)
                };
                let a76 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h74, _, _, _, _, _, _, _, _, _, _) => {
                        h74
                    }
                };
                nat_mul(a75, a76)?
            };
            let a85 = {
                let a82 = {
                    let a78 = v5;
                    let a79 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h77, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h77
                        }
                    };
                    let a80 = 0u64;
                    nat_quot(a78, a79, a80)
                };
                let a83 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h81, _, _, _, _, _, _, _, _, _) => {
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
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h86, _, _, _, _, _, _, _, _) => {
                            h86
                        }
                    };
                    nat_mul(a87, a88)?
                };
                let a93 = {
                    let a90 = v4;
                    let a91 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h89, _, _, _, _, _, _, _) => {
                            h89
                        }
                    };
                    nat_mul(a90, a91)?
                };
                nat_add(a92, a93)?
            };
            let a98 = {
                let a95 = v5;
                let a96 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h94, _, _, _, _, _, _) => {
                        h94
                    }
                };
                nat_mul(a95, a96)?
            };
            nat_add(a97, a98)?
        }, {
            let a110 = {
                let a105 = {
                    let a100 = v2;
                    let a101 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h99, _, _, _, _, _) => {
                            h99
                        }
                    };
                    nat_mul(a100, a101)?
                };
                let a106 = {
                    let a103 = v4;
                    let a104 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h102, _, _, _, _) => {
                            h102
                        }
                    };
                    nat_mul(a103, a104)?
                };
                nat_add(a105, a106)?
            };
            let a111 = {
                let a108 = v5;
                let a109 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h107, _, _, _) => {
                        h107
                    }
                };
                nat_mul(a108, a109)?
            };
            nat_add(a110, a111)?
        }))
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f4(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: bool) -> R<Adt4> {
    let a112 = v0;
    let a113 = 29u64;
    if if if if if if if if if if if if if if if if if if if if nat_eq(a112, a113) {
        !v1
    } else {
        false
    } {
        let a115 = match v4.clone() {
            Adt1::C0(h114, _, _, _, _, _, _, _, _) => {
                h114
            }
        };
        let a116 = 26u64;
        nat_eq(a115, a116)
    } else {
        false
    } {
        let a118 = match v3.clone() {
            Adt1::C0(h117, _, _, _, _, _, _, _, _) => {
                h117
            }
        };
        let a119 = 0u64;
        if if nat_eq(a118, a119) {
            true
        } else {
            if v6 {
                f9(match v3.clone() {
                    Adt1::C0(h120, _, _, _, _, _, _, _, _) => {
                        h120
                    }
                })
            } else {
                false
            }
        } {
            true
        } else {
            f8(match v2.clone() {
                Adt1::C0(h121, _, _, _, _, _, _, _, _) => {
                    h121
                }
            }, match v3.clone() {
                Adt1::C0(h122, _, _, _, _, _, _, _, _) => {
                    h122
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h123, _, _, _) => {
                    h123
                }
            })
        }
    } else {
        false
    } {
        let a125 = match v5.clone() {
            Adt1::C0(h124, _, _, _, _, _, _, _, _) => {
                h124
            }
        };
        let a126 = 0u64;
        nat_eq(a125, a126)
    } else {
        false
    } {
        let a128 = match v2.clone() {
            Adt1::C0(_, _, _, _, h127, _, _, _, _) => {
                h127
            }
        };
        let a129 = 1u64;
        nat_eq(a128, a129)
    } else {
        false
    } {
        let a131 = match v3.clone() {
            Adt1::C0(_, _, _, _, h130, _, _, _, _) => {
                h130
            }
        };
        let a132 = 1u64;
        nat_eq(a131, a132)
    } else {
        false
    } {
        let a134 = match v4.clone() {
            Adt1::C0(_, _, _, _, h133, _, _, _, _) => {
                h133
            }
        };
        let a135 = 1u64;
        nat_eq(a134, a135)
    } else {
        false
    } {
        let a137 = match v5.clone() {
            Adt1::C0(_, _, _, _, h136, _, _, _, _) => {
                h136
            }
        };
        let a138 = 1u64;
        nat_eq(a137, a138)
    } else {
        false
    } {
        let a140 = match v4.clone() {
            Adt1::C0(_, _, _, h139, _, _, _, _, _) => {
                h139
            }
        };
        let a141 = 1u64;
        nat_eq(a140, a141)
    } else {
        false
    } {
        let a144 = match v4.clone() {
            Adt1::C0(_, _, h142, _, _, _, _, _, _) => {
                h142
            }
        };
        let a145 = match v3.clone() {
            Adt1::C0(_, _, _, h143, _, _, _, _, _) => {
                h143
            }
        };
        nat_eq(a144, a145)
    } else {
        false
    } {
        let a148 = match v2.clone() {
            Adt1::C0(_, h146, _, _, _, _, _, _, _) => {
                h146
            }
        };
        let a149 = match v3.clone() {
            Adt1::C0(_, h147, _, _, _, _, _, _, _) => {
                h147
            }
        };
        nat_eq(a148, a149)
    } else {
        false
    } {
        let a151 = 0u64;
        let a152 = match v3.clone() {
            Adt1::C0(_, _, h150, _, _, _, _, _, _) => {
                h150
            }
        };
        nat_lt(a151, a152)
    } else {
        false
    } {
        let a158 = {
            let a155 = match v4.clone() {
                Adt1::C0(_, h153, _, _, _, _, _, _, _) => {
                    h153
                }
            };
            let a156 = match v3.clone() {
                Adt1::C0(_, _, h154, _, _, _, _, _, _) => {
                    h154
                }
            };
            let a157 = 0u64;
            nat_rem(a155, a156, a157)
        };
        let a159 = 0u64;
        nat_eq(a158, a159)
    } else {
        false
    } {
        let a162 = match v5.clone() {
            Adt1::C0(_, h160, _, _, _, _, _, _, _) => {
                h160
            }
        };
        let a163 = match v2.clone() {
            Adt1::C0(_, _, h161, _, _, _, _, _, _) => {
                h161
            }
        };
        nat_eq(a162, a163)
    } else {
        false
    } {
        let a166 = match v5.clone() {
            Adt1::C0(_, _, h164, _, _, _, _, _, _) => {
                h164
            }
        };
        let a167 = match v4.clone() {
            Adt1::C0(_, h165, _, _, _, _, _, _, _) => {
                h165
            }
        };
        nat_eq(a166, a167)
    } else {
        false
    } {
        let a170 = match v5.clone() {
            Adt1::C0(_, _, _, h168, _, _, _, _, _) => {
                h168
            }
        };
        let a171 = match v3.clone() {
            Adt1::C0(_, _, _, h169, _, _, _, _, _) => {
                h169
            }
        };
        nat_eq(a170, a171)
    } else {
        false
    } {
        let a174 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h172, _, _, _) => {
                h172
            }
        };
        let a175 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h173, _, _) => {
                h173
            }
        };
        nat_le(a174, a175)
    } else {
        false
    } {
        let a180 = {
            let a177 = match v5.clone() {
                Adt1::C0(_, h176, _, _, _, _, _, _, _) => {
                    h176
                }
            };
            let a178 = 4u64;
            nat_mul(a177, a178)?
        };
        let a181 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, h179, _, _) => {
                h179
            }
        };
        nat_le(a180, a181)
    } else {
        false
    } {
        let a187 = {
            let a184 = match v5.clone() {
                Adt1::C0(_, _, h182, _, _, _, _, _, _) => {
                    h182
                }
            };
            let a185 = match v5.clone() {
                Adt1::C0(_, _, _, _, _, _, h183, _, _) => {
                    h183
                }
            };
            nat_mul(a184, a185)?
        };
        let a188 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h186, _) => {
                h186
            }
        };
        nat_le(a187, a188)
    } else {
        false
    } {
        let m190 = f6(match v2.clone() {
            Adt1::C0(h189, _, _, _, _, _, _, _, _) => {
                h189
            }
        });
        match m190 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v7) => {
                if f10(v2.clone(), v3.clone(), v5.clone(), true, v6, match v7.clone() {
                    Adt6::C0(_, h191, _) => {
                        h191
                    }
                }, match v7.clone() {
                    Adt6::C0(_, _, h192) => {
                        h192
                    }
                }) {
                    Ok(Adt4::C1(f11(v2.clone(), v3.clone(), v5, match v7.clone() {
                        Adt6::C0(_, h193, _) => {
                            h193
                        }
                    }, match v7 {
                        Adt6::C0(_, _, h194) => {
                            h194
                        }
                    }, match v2.clone() {
                        Adt1::C0(_, _, h195, _, _, _, _, _, _) => {
                            h195
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h196, _, _, _, _, _, _, _) => {
                            h196
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h197, _, _, _, _, _, _) => {
                            h197
                        }
                    }, 1u64, 1u64, 1u64, Some(Adt3::C0(match v2 {
                        Adt1::C0(_, _, _, h198, _, _, _, _, _) => {
                            h198
                        }
                    }, match v3 {
                        Adt1::C0(_, _, h199, _, _, _, _, _, _) => {
                            h199
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h200, _, _, _) => {
                            h200
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h201, _, _) => {
                            h201
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
    let m203 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h202, _, _) => {
            h202
        }
    };
    match m203 {
        None => {
            Ok(Adt4::C0)
        }
        Some(v3) => {
            let a205 = v1;
            let a206 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h204, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h204
                }
            };
            if nat_lt(a205, a206) {
                let v4: u64 = {
                    let a208 = v1;
                    let a209 = match v0.clone() {
                        Adt2::C0(_, _, _, h207, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h207
                        }
                    };
                    let a210 = 0u64;
                    nat_rem(a208, a209, a210)
                };
                let v5: u64 = {
                    let a212 = v1;
                    let a213 = match v0.clone() {
                        Adt2::C0(_, _, _, h211, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h211
                        }
                    };
                    let a214 = 0u64;
                    nat_quot(a212, a213, a214)
                };
                let v6: u64 = {
                    let a221 = {
                        let a216 = v4;
                        let a217 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h215, _, _, _, _, _) => {
                                h215
                            }
                        };
                        nat_mul(a216, a217)?
                    };
                    let a222 = {
                        let a219 = v5;
                        let a220 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h218, _, _, _, _) => {
                                h218
                            }
                        };
                        nat_mul(a219, a220)?
                    };
                    nat_add(a221, a222)?
                };
                let a223 = v2;
                let a224 = 4294967295u64;
                if nat_eq(a223, a224) {
                    Ok(Adt4::C5(v6))
                } else {
                    let a226 = v2;
                    let a227 = match v3.clone() {
                        Adt3::C0(h225, _, _, _) => {
                            h225
                        }
                    };
                    if nat_lt(a226, a227) {
                        Ok(Adt4::C2({
                            let a229 = v2;
                            let a230 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h228, _, _, _, _, _, _, _, _, _, _) => {
                                    h228
                                }
                            };
                            nat_mul(a229, a230)?
                        }, {
                            let a241 = {
                                let a236 = {
                                    let a232 = v4;
                                    let a233 = match v3 {
                                        Adt3::C0(_, h231, _, _) => {
                                            h231
                                        }
                                    };
                                    let a234 = 0u64;
                                    nat_rem(a232, a233, a234)
                                };
                                let a237 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h235, _, _, _, _, _, _, _, _) => {
                                        h235
                                    }
                                };
                                nat_mul(a236, a237)?
                            };
                            let a242 = {
                                let a239 = v5;
                                let a240 = match v0 {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h238, _, _, _, _, _, _, _) => {
                                        h238
                                    }
                                };
                                nat_mul(a239, a240)?
                            };
                            nat_add(a241, a242)?
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
    let a243 = v0;
    let a244 = 0u64;
    if nat_eq(a243, a244) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a245 = v0;
        let a246 = 1u64;
        if nat_eq(a245, a246) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a247 = v0;
            let a248 = 2u64;
            if nat_eq(a247, a248) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a249 = v0;
                let a250 = 3u64;
                if nat_eq(a249, a250) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a251 = v0;
                    let a252 = 6u64;
                    if nat_eq(a251, a252) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a253 = v0;
                        let a254 = 7u64;
                        if nat_eq(a253, a254) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a255 = v0;
                            let a256 = 8u64;
                            if nat_eq(a255, a256) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a257 = v0;
                                let a258 = 9u64;
                                if nat_eq(a257, a258) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a259 = v0;
                                    let a260 = 10u64;
                                    if nat_eq(a259, a260) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a261 = v0;
                                        let a262 = 11u64;
                                        if nat_eq(a261, a262) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a263 = v0;
                                            let a264 = 12u64;
                                            if nat_eq(a263, a264) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a265 = v0;
                                                let a266 = 13u64;
                                                if nat_eq(a265, a266) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a267 = v0;
                                                    let a268 = 14u64;
                                                    if nat_eq(a267, a268) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a269 = v0;
                                                        let a270 = 15u64;
                                                        if nat_eq(a269, a270) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a271 = v0;
                                                            let a272 = 16u64;
                                                            if nat_eq(a271, a272) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a273 = v0;
                                                                let a274 = 17u64;
                                                                if nat_eq(a273, a274) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a275 = v0;
                                                                    let a276 = 18u64;
                                                                    if nat_eq(a275, a276) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a277 = v0;
                                                                        let a278 = 19u64;
                                                                        if nat_eq(a277, a278) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a279 = v0;
                                                                            let a280 = 20u64;
                                                                            if nat_eq(a279, a280) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a281 = v0;
                                                                                let a282 = 21u64;
                                                                                if nat_eq(a281, a282) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a283 = v0;
                                                                                    let a284 = 22u64;
                                                                                    if nat_eq(a283, a284) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a285 = v0;
                                                                                        let a286 = 23u64;
                                                                                        if nat_eq(a285, a286) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a287 = v0;
                                                                                            let a288 = 24u64;
                                                                                            if nat_eq(a287, a288) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a289 = v0;
                                                                                                let a290 = 25u64;
                                                                                                if nat_eq(a289, a290) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a291 = v0;
                                                                                                    let a292 = 26u64;
                                                                                                    if nat_eq(a291, a292) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a293 = v0;
                                                                                                        let a294 = 27u64;
                                                                                                        if nat_eq(a293, a294) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a295 = v0;
                                                                                                            let a296 = 28u64;
                                                                                                            if nat_eq(a295, a296) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a297 = v0;
                                                                                                                let a298 = 29u64;
                                                                                                                if nat_eq(a297, a298) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a299 = v0;
                                                                                                                    let a300 = 30u64;
                                                                                                                    if nat_eq(a299, a300) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a301 = v0;
                                                                                                                        let a302 = 34u64;
                                                                                                                        if nat_eq(a301, a302) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a303 = v0;
                                                                                                                            let a304 = 35u64;
                                                                                                                            if nat_eq(a303, a304) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a305 = v0;
                                                                                                                                let a306 = 39u64;
                                                                                                                                if nat_eq(a305, a306) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a307 = v0;
                                                                                                                                    let a308 = 40u64;
                                                                                                                                    if nat_eq(a307, a308) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a309 = v0;
                                                                                                                                        let a310 = 41u64;
                                                                                                                                        if nat_eq(a309, a310) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a311 = v0;
                                                                                                                                            let a312 = 42u64;
                                                                                                                                            if nat_eq(a311, a312) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a313 = v0;
                                                                                                                                                let a314 = 43u64;
                                                                                                                                                if nat_eq(a313, a314) {
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
        let m328 = f12(v0, 0u64, 0u64, match v3.clone() {
            Adt1::C0(h315, _, _, _, _, _, _, _, _) => {
                h315
            }
        }, match v1.clone() {
            Adt1::C0(_, h316, _, _, _, _, _, _, _) => {
                h316
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h317, _, _, _, _, _, _) => {
                h317
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h318, _, _, _, _, _) => {
                h318
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h319, _, _, _, _) => {
                h319
            }
        }, match v2.clone() {
            Adt1::C0(_, h320, _, _, _, _, _, _, _) => {
                h320
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h321, _, _, _, _, _, _) => {
                h321
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h322, _, _, _, _, _) => {
                h322
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h323, _, _, _, _) => {
                h323
            }
        }, match v3.clone() {
            Adt1::C0(_, h324, _, _, _, _, _, _, _) => {
                h324
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h325, _, _, _, _, _, _) => {
                h325
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h326, _, _, _, _, _) => {
                h326
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h327, _, _, _, _) => {
                h327
            }
        })?;
        match m328 {
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
        let a329 = v1;
        let a330 = 15u64;
        if nat_eq(a329, a330) {
            let a331 = v2;
            let a332 = 292u64;
            nat_eq(a331, a332)
        } else {
            false
        }
    } else {
        if f14(v0) {
            let a333 = v1;
            let a334 = 9u64;
            if nat_eq(a333, a334) {
                let a335 = v2;
                let a336 = 36u64;
                nat_eq(a335, a336)
            } else {
                false
            }
        } else {
            if if f15(v0) {
                true
            } else {
                f16(v0)
            } {
                let a337 = v1;
                let a338 = 8u64;
                if nat_eq(a337, a338) {
                    let a339 = v2;
                    let a340 = 34u64;
                    nat_eq(a339, a340)
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
    let a341 = v0;
    let a342 = 0u64;
    if if nat_eq(a341, a342) {
        true
    } else {
        let a343 = v0;
        let a344 = 1u64;
        nat_eq(a343, a344)
    } {
        true
    } else {
        let a345 = v0;
        let a346 = 30u64;
        nat_eq(a345, a346)
    }
}

pub fn f10(v0: Adt1, v1: Adt1, v2: Adt1, v3: bool, v4: bool, v5: u64, v6: u64) -> bool {
    if if if if if if if if f17(match v0.clone() {
        Adt1::C0(h347, _, _, _, _, _, _, _, _) => {
            h347
        }
    }) {
        true
    } else {
        if v3 {
            if f9(match v0.clone() {
                Adt1::C0(h348, _, _, _, _, _, _, _, _) => {
                    h348
                }
            }) {
                true
            } else {
                let a350 = match v0.clone() {
                    Adt1::C0(h349, _, _, _, _, _, _, _, _) => {
                        h349
                    }
                };
                let a351 = 43u64;
                nat_eq(a350, a351)
            }
        } else {
            false
        }
    } {
        let a356 = {
            let a353 = match v0.clone() {
                Adt1::C0(_, h352, _, _, _, _, _, _, _) => {
                    h352
                }
            };
            let a354 = v5;
            let a355 = 0u64;
            nat_rem(a353, a354, a355)
        };
        let a357 = 0u64;
        nat_eq(a356, a357)
    } else {
        false
    } {
        let a359 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h358, _, _, _) => {
                h358
            }
        };
        let a360 = v6;
        nat_eq(a359, a360)
    } else {
        false
    } {
        if f18(match v1.clone() {
            Adt1::C0(h361, _, _, _, _, _, _, _, _) => {
                h361
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, h362, _, _, _) => {
                h362
            }
        }, v4) {
            true
        } else {
            f8(match v0 {
                Adt1::C0(h363, _, _, _, _, _, _, _, _) => {
                    h363
                }
            }, match v1.clone() {
                Adt1::C0(h364, _, _, _, _, _, _, _, _) => {
                    h364
                }
            }, match v1 {
                Adt1::C0(_, _, _, _, _, h365, _, _, _) => {
                    h365
                }
            })
        }
    } else {
        false
    } {
        let a367 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h366, _, _, _) => {
                h366
            }
        };
        let a368 = 4u64;
        nat_eq(a367, a368)
    } else {
        false
    } {
        let a371 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h369, _, _, _) => {
                h369
            }
        };
        let a372 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h370, _, _) => {
                h370
            }
        };
        nat_le(a371, a372)
    } else {
        false
    } {
        let a375 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h373, _, _) => {
                h373
            }
        };
        let a376 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h374, _) => {
                h374
            }
        };
        nat_le(a375, a376)
    } else {
        false
    } {
        let a379 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h377, _) => {
                h377
            }
        };
        let a380 = match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h378) => {
                h378
            }
        };
        nat_le(a379, a380)
    } else {
        false
    }
}

pub fn f11(v0: Adt1, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: Option<Adt3>, v12: bool) -> R<Adt2> {
    Ok(Adt2::C0(if v12 {
        match v0.clone() {
            Adt1::C0(_, h381, _, _, _, _, _, _, _) => {
                h381
            }
        }
    } else {
        let a384 = match v0.clone() {
            Adt1::C0(_, h382, _, _, _, _, _, _, _) => {
                h382
            }
        };
        let a385 = f19(match v0.clone() {
            Adt1::C0(h383, _, _, _, _, _, _, _, _) => {
                h383
            }
        }, v3);
        let a386 = 0u64;
        nat_quot(a384, a385, a386)
    }, {
        let a391 = {
            let a388 = match v0.clone() {
                Adt1::C0(_, h387, _, _, _, _, _, _, _) => {
                    h387
                }
            };
            let a389 = v3;
            let a390 = 0u64;
            nat_quot(a388, a389, a390)
        };
        let a392 = v4;
        nat_mul(a391, a392)?
    }, v5, v6, v7, {
        let a395 = v6;
        let a396 = {
            let a393 = v7;
            let a394 = v8;
            nat_mul(a393, a394)?
        };
        nat_mul(a395, a396)?
    }, {
        let a401 = v5;
        let a402 = {
            let a399 = v6;
            let a400 = {
                let a397 = v7;
                let a398 = v8;
                nat_mul(a397, a398)?
            };
            nat_mul(a399, a400)?
        };
        nat_mul(a401, a402)?
    }, v9, v10, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h403, _, _) => {
            h403
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h404, _) => {
            h404
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h405) => {
            h405
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h406, _, _) => {
            h406
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h407, _) => {
            h407
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h408) => {
            h408
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h409, _, _) => {
            h409
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h410, _) => {
            h410
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h411) => {
            h411
        }
    }, v11, if v12 {
        Some(0u64)
    } else {
        if if f9(match v0.clone() {
            Adt1::C0(h412, _, _, _, _, _, _, _, _) => {
                h412
            }
        }) {
            true
        } else {
            let a414 = match v0.clone() {
                Adt1::C0(h413, _, _, _, _, _, _, _, _) => {
                    h413
                }
            };
            let a415 = 43u64;
            nat_eq(a414, a415)
        } {
            Some(f20(match v0.clone() {
                Adt1::C0(h416, _, _, _, _, _, _, _, _) => {
                    h416
                }
            }))
        } else {
            None::<u64>
        }
    }, match v0 {
        Adt1::C0(h417, _, _, _, _, _, _, _, _) => {
            h417
        }
    }))
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt7> {
    let a418 = v0;
    let a419 = 28u64;
    if if if if nat_eq(a418, a419) {
        let a420 = v1;
        let a421 = 0u64;
        nat_eq(a420, a421)
    } else {
        false
    } {
        let a422 = v2;
        let a423 = 0u64;
        nat_eq(a422, a423)
    } else {
        false
    } {
        let a424 = v3;
        let a425 = 0u64;
        nat_eq(a424, a425)
    } else {
        false
    } {
        let a426 = v4;
        let a427 = v8;
        if if if if if if if if nat_eq(a426, a427) {
            let a428 = v5;
            let a429 = v12;
            nat_eq(a428, a429)
        } else {
            false
        } {
            let a430 = v9;
            let a431 = v13;
            nat_eq(a430, a431)
        } else {
            false
        } {
            let a432 = v10;
            let a433 = v14;
            nat_eq(a432, a433)
        } else {
            false
        } {
            let a434 = v11;
            let a435 = v15;
            nat_eq(a434, a435)
        } else {
            false
        } {
            let a436 = 0u64;
            let a437 = v6;
            nat_lt(a436, a437)
        } else {
            false
        } {
            let a438 = 0u64;
            let a439 = v7;
            nat_lt(a438, a439)
        } else {
            false
        } {
            let a443 = {
                let a440 = v10;
                let a441 = v6;
                let a442 = 1u64;
                nat_rem(a440, a441, a442)
            };
            let a444 = 0u64;
            if nat_eq(a443, a444) {
                let a448 = {
                    let a445 = v11;
                    let a446 = v7;
                    let a447 = 1u64;
                    nat_rem(a445, a446, a447)
                };
                let a449 = 0u64;
                nat_eq(a448, a449)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt7::C2(v5, v4, v9, v10, v11, {
                let a450 = v10;
                let a451 = v6;
                let a452 = 0u64;
                nat_quot(a450, a451, a452)
            }, {
                let a453 = v11;
                let a454 = v7;
                let a455 = 0u64;
                nat_quot(a453, a454, a455)
            }, {
                let a456 = v5;
                let a457 = v4;
                nat_mul(a456, a457)?
            }, {
                let a458 = v4;
                let a459 = v9;
                nat_mul(a458, a459)?
            }, {
                let a460 = v5;
                let a461 = v9;
                nat_mul(a460, a461)?
            }))
        } else {
            Ok(Adt7::C1)
        }
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f13(v0: u64) -> bool {
    let a462 = 10u64;
    let a463 = v0;
    if if if if if nat_le(a462, a463) {
        let a464 = v0;
        let a465 = 14u64;
        nat_le(a464, a465)
    } else {
        false
    } {
        true
    } else {
        let a466 = 16u64;
        let a467 = v0;
        if nat_le(a466, a467) {
            let a468 = v0;
            let a469 = 19u64;
            nat_le(a468, a469)
        } else {
            false
        }
    } {
        true
    } else {
        let a470 = 21u64;
        let a471 = v0;
        if nat_le(a470, a471) {
            let a472 = v0;
            let a473 = 23u64;
            nat_le(a472, a473)
        } else {
            false
        }
    } {
        true
    } else {
        let a474 = 34u64;
        let a475 = v0;
        if nat_le(a474, a475) {
            let a476 = v0;
            let a477 = 35u64;
            nat_le(a476, a477)
        } else {
            false
        }
    } {
        true
    } else {
        let a478 = v0;
        let a479 = 29u64;
        nat_eq(a478, a479)
    }
}

pub fn f14(v0: u64) -> bool {
    let a480 = v0;
    let a481 = 3u64;
    if nat_eq(a480, a481) {
        true
    } else {
        let a482 = v0;
        let a483 = 7u64;
        nat_eq(a482, a483)
    }
}

pub fn f15(v0: u64) -> bool {
    let a484 = v0;
    let a485 = 2u64;
    if if if if nat_eq(a484, a485) {
        true
    } else {
        let a486 = v0;
        let a487 = 6u64;
        nat_eq(a486, a487)
    } {
        true
    } else {
        let a488 = v0;
        let a489 = 8u64;
        nat_eq(a488, a489)
    } {
        true
    } else {
        let a490 = v0;
        let a491 = 20u64;
        nat_eq(a490, a491)
    } {
        true
    } else {
        f21(v0)
    }
}

pub fn f16(v0: u64) -> bool {
    let a492 = v0;
    let a493 = 39u64;
    if nat_eq(a492, a493) {
        true
    } else {
        let a494 = v0;
        let a495 = 40u64;
        nat_eq(a494, a495)
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
    let a496 = v0;
    let a497 = 0u64;
    if if nat_eq(a496, a497) {
        let a498 = v1;
        let a499 = 4u64;
        nat_eq(a498, a499)
    } else {
        false
    } {
        true
    } else {
        let a500 = v0;
        let a501 = 1u64;
        if if nat_eq(a500, a501) {
            true
        } else {
            if v2 {
                let a502 = v0;
                let a503 = 30u64;
                nat_eq(a502, a503)
            } else {
                false
            }
        } {
            let a504 = v1;
            let a505 = 2u64;
            nat_eq(a504, a505)
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
    let a506 = v0;
    let a507 = 30u64;
    if nat_eq(a506, a507) {
        30u64
    } else {
        0u64
    }
}

pub fn f21(v0: u64) -> bool {
    let a508 = v0;
    let a509 = 41u64;
    if nat_eq(a508, a509) {
        true
    } else {
        let a510 = v0;
        let a511 = 42u64;
        nat_eq(a510, a511)
    }
}

pub fn f22(v0: u64) -> bool {
    if f15(v0) {
        true
    } else {
        f14(v0)
    }
}
