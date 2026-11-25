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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, Option<Adt3>, Option<u64>, u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Adt2),
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
    C5(u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(Adt7),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6) => {
            f1(v1, v2, v3, v4, None::<Adt1>, v5, v6)
        }
        Adt0::C1(v7, v8) => {
            let m3 = match v7.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2, _, _, _) => {
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
                        Adt2::C0(_, _, _, _, _, h4, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4
                        }
                    };
                    if nat_lt(a5, a6) {
                        Ok(Adt5::C4({
                            let a21 = {
                                let a12 = {
                                    let a8 = v8;
                                    let a9 = match v7.clone() {
                                        Adt2::C0(_, _, _, h7, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
                                        Adt2::C0(_, _, _, h14, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
                        Ok(Adt5::C0)
                    }
                }
            }
        }
        Adt0::C2(v20, v21, v22) => {
            let a24 = v22;
            let a25 = match v20.clone() {
                Adt2::C0(_, _, h23, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h23
                }
            };
            if nat_lt(a24, a25) {
                Ok(Adt5::C3({
                    let a29 = v21;
                    let a30 = {
                        let a27 = v22;
                        let a28 = match v20 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h26, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h26
                            }
                        };
                        nat_mul(a27, a28)?
                    };
                    nat_add(a29, a30)?
                }))
            } else {
                Ok(Adt5::C0)
            }
        }
        Adt0::C3(v10, v11, v12, v13, v14, v15, v16) => {
            f1(v10, v11, v12, v13, Some(v14), v15, v16)
        }
        Adt0::C4(v17, v18, v19) => {
            f3(v17, v18, v19)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Adt1, v6: bool) -> R<Adt5> {
    let a31 = v0;
    let a32 = 29u64;
    if if nat_eq(a31, a32) {
        let a35 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h33, _, _) => {
                h33
            }
        };
        let a36 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h34, _, _, _) => {
                h34
            }
        };
        nat_lt(a35, a36)
    } else {
        false
    } {
        Ok(Adt5::C0)
    } else {
        let m37 = f4(v0, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), f5(v0, v2.clone(), v3.clone(), v6))?;
        match m37 {
            Adt5::C0 => {
                if if f6(v2.clone()) {
                    f6(v3.clone())
                } else {
                    false
                } {
                    f4(v0, v1, f7(v2), f7(v3), v4, v5, true)
                } else {
                    Ok(Adt5::C0)
                }
            }
            Adt5::C1(v7) => {
                Ok(Adt5::C1(v7))
            }
            Adt5::C2(_, _, _) => {
                Ok(Adt5::C0)
            }
            Adt5::C3(_) => {
                Ok(Adt5::C0)
            }
            Adt5::C4(_) => {
                Ok(Adt5::C0)
            }
            Adt5::C5(_) => {
                Ok(Adt5::C0)
            }
        }
    }
}

pub fn f2(v0: Adt2, v1: u64) -> R<Adt5> {
    let a39 = v1;
    let a40 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h38, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h38
        }
    };
    if nat_lt(a39, a40) {
        let v2: u64 = {
            let a42 = v1;
            let a43 = match v0.clone() {
                Adt2::C0(_, _, _, h41, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h41
                }
            };
            let a44 = 0u64;
            nat_rem(a42, a43, a44)
        };
        let v3: u64 = {
            let a46 = v1;
            let a47 = match v0.clone() {
                Adt2::C0(_, _, _, h45, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h45
                }
            };
            let a48 = 0u64;
            nat_quot(a46, a47, a48)
        };
        let v4: u64 = {
            let a50 = v3;
            let a51 = match v0.clone() {
                Adt2::C0(_, _, _, _, h49, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h49
                }
            };
            let a52 = 0u64;
            nat_rem(a50, a51, a52)
        };
        let v5: u64 = {
            let a54 = v3;
            let a55 = match v0.clone() {
                Adt2::C0(_, _, _, _, h53, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h53
                }
            };
            let a56 = 0u64;
            nat_quot(a54, a55, a56)
        };
        Ok(Adt5::C2({
            let a71 = {
                let a62 = {
                    let a58 = v4;
                    let a59 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h57, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h57
                        }
                    };
                    let a60 = 0u64;
                    nat_quot(a58, a59, a60)
                };
                let a63 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h61, _, _, _, _, _, _, _, _, _, _, _) => {
                        h61
                    }
                };
                nat_mul(a62, a63)?
            };
            let a72 = {
                let a69 = {
                    let a65 = v5;
                    let a66 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h64, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h64
                        }
                    };
                    let a67 = 0u64;
                    nat_quot(a65, a66, a67)
                };
                let a70 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h68, _, _, _, _, _, _, _, _, _, _) => {
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
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h73, _, _, _, _, _, _, _, _, _) => {
                            h73
                        }
                    };
                    nat_mul(a74, a75)?
                };
                let a80 = {
                    let a77 = v4;
                    let a78 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h76, _, _, _, _, _, _, _, _) => {
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
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h81, _, _, _, _, _, _, _) => {
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
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h86, _, _, _, _, _, _) => {
                            h86
                        }
                    };
                    nat_mul(a87, a88)?
                };
                let a93 = {
                    let a90 = v4;
                    let a91 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h89, _, _, _, _, _) => {
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
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h94, _, _, _, _) => {
                        h94
                    }
                };
                nat_mul(a95, a96)?
            };
            nat_add(a97, a98)?
        }))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f3(v0: Adt2, v1: u64, v2: u64) -> R<Adt5> {
    let m100 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h99, _, _, _) => {
            h99
        }
    };
    match m100 {
        None => {
            Ok(Adt5::C0)
        }
        Some(v3) => {
            let a102 = v1;
            let a103 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h101, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h101
                }
            };
            if nat_lt(a102, a103) {
                let v4: u64 = {
                    let a105 = v1;
                    let a106 = match v0.clone() {
                        Adt2::C0(_, _, _, h104, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h104
                        }
                    };
                    let a107 = 0u64;
                    nat_rem(a105, a106, a107)
                };
                let v5: u64 = {
                    let a109 = v1;
                    let a110 = match v0.clone() {
                        Adt2::C0(_, _, _, h108, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h108
                        }
                    };
                    let a111 = 0u64;
                    nat_quot(a109, a110, a111)
                };
                let v6: u64 = {
                    let a118 = {
                        let a113 = v4;
                        let a114 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h112, _, _, _, _, _, _) => {
                                h112
                            }
                        };
                        nat_mul(a113, a114)?
                    };
                    let a119 = {
                        let a116 = v5;
                        let a117 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h115, _, _, _, _, _) => {
                                h115
                            }
                        };
                        nat_mul(a116, a117)?
                    };
                    nat_add(a118, a119)?
                };
                let a120 = v2;
                let a121 = 4294967295u64;
                if nat_eq(a120, a121) {
                    Ok(Adt5::C5(v6))
                } else {
                    let a123 = v2;
                    let a124 = match v3.clone() {
                        Adt3::C0(h122, _, _, _) => {
                            h122
                        }
                    };
                    if nat_lt(a123, a124) {
                        Ok(Adt5::C2({
                            let a126 = v2;
                            let a127 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h125, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h125
                                }
                            };
                            nat_mul(a126, a127)?
                        }, {
                            let a138 = {
                                let a133 = {
                                    let a129 = v4;
                                    let a130 = match v3 {
                                        Adt3::C0(_, h128, _, _) => {
                                            h128
                                        }
                                    };
                                    let a131 = 0u64;
                                    nat_rem(a129, a130, a131)
                                };
                                let a134 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h132, _, _, _, _, _, _, _, _, _) => {
                                        h132
                                    }
                                };
                                nat_mul(a133, a134)?
                            };
                            let a139 = {
                                let a136 = v5;
                                let a137 = match v0 {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h135, _, _, _, _, _, _, _, _) => {
                                        h135
                                    }
                                };
                                nat_mul(a136, a137)?
                            };
                            nat_add(a138, a139)?
                        }, v6))
                    } else {
                        Ok(Adt5::C0)
                    }
                }
            } else {
                Ok(Adt5::C0)
            }
        }
    }
}

pub fn f4(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Adt1, v6: bool) -> R<Adt5> {
    let m140 = v4;
    match m140 {
        None => {
            f8(v0, v1, v2, v3, v5, v6)
        }
        Some(v7) => {
            f9(v0, v1, v2, v3, v7, v5, v6)
        }
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: Adt1, v3: bool) -> bool {
    let a142 = match v2.clone() {
        Adt1::C0(h141, _, _, _, _, _, _, _, _) => {
            h141
        }
    };
    let a143 = 30u64;
    if if nat_eq(a142, a143) {
        let a144 = v0;
        let a145 = 28u64;
        if if nat_eq(a144, a145) {
            let a147 = match v1.clone() {
                Adt1::C0(h146, _, _, _, _, _, _, _, _) => {
                    h146
                }
            };
            let a148 = 1u64;
            nat_eq(a147, a148)
        } else {
            false
        } {
            true
        } else {
            let a149 = v0;
            let a150 = 29u64;
            if nat_eq(a149, a150) {
                let a152 = match v1.clone() {
                    Adt1::C0(h151, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                };
                let a153 = 30u64;
                nat_eq(a152, a153)
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
            !f10(match v1 {
                Adt1::C0(h154, _, _, _, _, _, _, _, _) => {
                    h154
                }
            }, match v2.clone() {
                Adt1::C0(h155, _, _, _, _, _, _, _, _) => {
                    h155
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, _, h156, _, _, _) => {
                    h156
                }
            })
        } else {
            false
        }
    }
}

pub fn f6(v0: Adt1) -> bool {
    let m158 = f11(match v0.clone() {
        Adt1::C0(h157, _, _, _, _, _, _, _, _) => {
            h157
        }
    });
    match m158 {
        Adt6::C0 => {
            false
        }
        Adt6::C1(v1) => {
            let a161 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h159, _, _, _) => {
                    h159
                }
            };
            let a162 = match v1.clone() {
                Adt7::C0(_, _, h160) => {
                    h160
                }
            };
            if nat_eq(a161, a162) {
                let a168 = {
                    let a165 = match v0 {
                        Adt1::C0(_, h163, _, _, _, _, _, _, _) => {
                            h163
                        }
                    };
                    let a166 = match v1 {
                        Adt7::C0(_, h164, _) => {
                            h164
                        }
                    };
                    let a167 = 1u64;
                    nat_rem(a165, a166, a167)
                };
                let a169 = 0u64;
                nat_eq(a168, a169)
            } else {
                false
            }
        }
    }
}

pub fn f7(v0: Adt1) -> Adt1 {
    Adt1::C0(0u64, match v0.clone() {
        Adt1::C0(_, h170, _, _, _, _, _, _, _) => {
            h170
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h171, _, _, _, _, _, _) => {
            h171
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h172, _, _, _, _, _) => {
            h172
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h173, _, _, _, _) => {
            h173
        }
    }, 4u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h174, _, _) => {
            h174
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h175, _) => {
            h175
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h176) => {
            h176
        }
    })
}

pub fn f8(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: bool) -> R<Adt5> {
    if v1 {
        Ok(Adt5::C0)
    } else {
        let m178 = f11(match v2.clone() {
            Adt1::C0(h177, _, _, _, _, _, _, _, _) => {
                h177
            }
        });
        match m178 {
            Adt6::C0 => {
                Ok(Adt5::C0)
            }
            Adt6::C1(v6) => {
                f12(v0, v2, v3, v4, match v6.clone() {
                    Adt7::C0(_, h179, _) => {
                        h179
                    }
                }, match v6 {
                    Adt7::C0(_, _, h180) => {
                        h180
                    }
                }, v5)
            }
        }
    }
}

pub fn f9(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: bool) -> R<Adt5> {
    let a181 = v0;
    let a182 = 29u64;
    if if if if if if if if if if if if if if if if if if if nat_eq(a181, a182) {
        !v1
    } else {
        false
    } {
        let a184 = match v4.clone() {
            Adt1::C0(h183, _, _, _, _, _, _, _, _) => {
                h183
            }
        };
        let a185 = 26u64;
        nat_eq(a184, a185)
    } else {
        false
    } {
        let a187 = match v3.clone() {
            Adt1::C0(h186, _, _, _, _, _, _, _, _) => {
                h186
            }
        };
        let a188 = 0u64;
        if if nat_eq(a187, a188) {
            true
        } else {
            if v6 {
                f13(match v3.clone() {
                    Adt1::C0(h189, _, _, _, _, _, _, _, _) => {
                        h189
                    }
                })
            } else {
                false
            }
        } {
            true
        } else {
            f10(match v2.clone() {
                Adt1::C0(h190, _, _, _, _, _, _, _, _) => {
                    h190
                }
            }, match v3.clone() {
                Adt1::C0(h191, _, _, _, _, _, _, _, _) => {
                    h191
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h192, _, _, _) => {
                    h192
                }
            })
        }
    } else {
        false
    } {
        let a194 = match v5.clone() {
            Adt1::C0(h193, _, _, _, _, _, _, _, _) => {
                h193
            }
        };
        let a195 = 0u64;
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a197 = match v2.clone() {
            Adt1::C0(_, _, _, _, h196, _, _, _, _) => {
                h196
            }
        };
        let a198 = 1u64;
        nat_eq(a197, a198)
    } else {
        false
    } {
        let a200 = match v3.clone() {
            Adt1::C0(_, _, _, _, h199, _, _, _, _) => {
                h199
            }
        };
        let a201 = 1u64;
        nat_eq(a200, a201)
    } else {
        false
    } {
        let a203 = match v4.clone() {
            Adt1::C0(_, _, _, _, h202, _, _, _, _) => {
                h202
            }
        };
        let a204 = 1u64;
        nat_eq(a203, a204)
    } else {
        false
    } {
        let a206 = match v5.clone() {
            Adt1::C0(_, _, _, _, h205, _, _, _, _) => {
                h205
            }
        };
        let a207 = 1u64;
        nat_eq(a206, a207)
    } else {
        false
    } {
        let a209 = match v4.clone() {
            Adt1::C0(_, _, _, h208, _, _, _, _, _) => {
                h208
            }
        };
        let a210 = 1u64;
        nat_eq(a209, a210)
    } else {
        false
    } {
        let a213 = match v4.clone() {
            Adt1::C0(_, _, h211, _, _, _, _, _, _) => {
                h211
            }
        };
        let a214 = match v3.clone() {
            Adt1::C0(_, _, _, h212, _, _, _, _, _) => {
                h212
            }
        };
        nat_eq(a213, a214)
    } else {
        false
    } {
        let a217 = match v2.clone() {
            Adt1::C0(_, h215, _, _, _, _, _, _, _) => {
                h215
            }
        };
        let a218 = match v3.clone() {
            Adt1::C0(_, h216, _, _, _, _, _, _, _) => {
                h216
            }
        };
        nat_eq(a217, a218)
    } else {
        false
    } {
        let a220 = 0u64;
        let a221 = match v3.clone() {
            Adt1::C0(_, _, h219, _, _, _, _, _, _) => {
                h219
            }
        };
        nat_lt(a220, a221)
    } else {
        false
    } {
        let a227 = {
            let a224 = match v4.clone() {
                Adt1::C0(_, h222, _, _, _, _, _, _, _) => {
                    h222
                }
            };
            let a225 = match v3.clone() {
                Adt1::C0(_, _, h223, _, _, _, _, _, _) => {
                    h223
                }
            };
            let a226 = 0u64;
            nat_rem(a224, a225, a226)
        };
        let a228 = 0u64;
        nat_eq(a227, a228)
    } else {
        false
    } {
        let a231 = match v5.clone() {
            Adt1::C0(_, h229, _, _, _, _, _, _, _) => {
                h229
            }
        };
        let a232 = match v2.clone() {
            Adt1::C0(_, _, h230, _, _, _, _, _, _) => {
                h230
            }
        };
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a235 = match v5.clone() {
            Adt1::C0(_, _, h233, _, _, _, _, _, _) => {
                h233
            }
        };
        let a236 = match v4.clone() {
            Adt1::C0(_, h234, _, _, _, _, _, _, _) => {
                h234
            }
        };
        nat_eq(a235, a236)
    } else {
        false
    } {
        let a239 = match v5.clone() {
            Adt1::C0(_, _, _, h237, _, _, _, _, _) => {
                h237
            }
        };
        let a240 = match v3.clone() {
            Adt1::C0(_, _, _, h238, _, _, _, _, _) => {
                h238
            }
        };
        nat_eq(a239, a240)
    } else {
        false
    } {
        let a245 = {
            let a242 = match v5.clone() {
                Adt1::C0(_, h241, _, _, _, _, _, _, _) => {
                    h241
                }
            };
            let a243 = 4u64;
            nat_mul(a242, a243)?
        };
        let a246 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, h244, _, _) => {
                h244
            }
        };
        nat_le(a245, a246)
    } else {
        false
    } {
        let a252 = {
            let a249 = match v5.clone() {
                Adt1::C0(_, _, h247, _, _, _, _, _, _) => {
                    h247
                }
            };
            let a250 = match v5.clone() {
                Adt1::C0(_, _, _, _, _, _, h248, _, _) => {
                    h248
                }
            };
            nat_mul(a249, a250)?
        };
        let a253 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h251, _) => {
                h251
            }
        };
        nat_le(a252, a253)
    } else {
        false
    } {
        let m255 = f11(match v2.clone() {
            Adt1::C0(h254, _, _, _, _, _, _, _, _) => {
                h254
            }
        });
        match m255 {
            Adt6::C0 => {
                Ok(Adt5::C0)
            }
            Adt6::C1(v7) => {
                if f14(v2.clone(), v3.clone(), v5.clone(), true, v6, match v7.clone() {
                    Adt7::C0(_, h256, _) => {
                        h256
                    }
                }, match v7.clone() {
                    Adt7::C0(_, _, h257) => {
                        h257
                    }
                }) {
                    Ok(Adt5::C1(f15(v2.clone(), v3.clone(), v5, match v7.clone() {
                        Adt7::C0(_, h258, _) => {
                            h258
                        }
                    }, match v7 {
                        Adt7::C0(_, _, h259) => {
                            h259
                        }
                    }, match v2.clone() {
                        Adt1::C0(_, _, h260, _, _, _, _, _, _) => {
                            h260
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h261, _, _, _, _, _, _, _) => {
                            h261
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h262, _, _, _, _, _, _) => {
                            h262
                        }
                    }, 1u64, 1u64, 1u64, Some(Adt3::C0(match v2 {
                        Adt1::C0(_, _, _, h263, _, _, _, _, _) => {
                            h263
                        }
                    }, match v3 {
                        Adt1::C0(_, _, h264, _, _, _, _, _, _) => {
                            h264
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h265, _, _, _) => {
                            h265
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h266, _, _) => {
                            h266
                        }
                    })), v6)?))
                } else {
                    Ok(Adt5::C0)
                }
            }
        }
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> bool {
    if f16(v0) {
        let a267 = v1;
        let a268 = 15u64;
        if nat_eq(a267, a268) {
            let a269 = v2;
            let a270 = 292u64;
            nat_eq(a269, a270)
        } else {
            false
        }
    } else {
        if f17(v0) {
            let a271 = v1;
            let a272 = 9u64;
            if nat_eq(a271, a272) {
                let a273 = v2;
                let a274 = 36u64;
                nat_eq(a273, a274)
            } else {
                false
            }
        } else {
            if if f18(v0) {
                true
            } else {
                f19(v0)
            } {
                let a275 = v1;
                let a276 = 8u64;
                if nat_eq(a275, a276) {
                    let a277 = v2;
                    let a278 = 34u64;
                    nat_eq(a277, a278)
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f11(v0: u64) -> Adt6 {
    let a279 = v0;
    let a280 = 0u64;
    if nat_eq(a279, a280) {
        Adt6::C1(Adt7::C0(0u64, 1u64, 4u64))
    } else {
        let a281 = v0;
        let a282 = 1u64;
        if nat_eq(a281, a282) {
            Adt6::C1(Adt7::C0(1u64, 1u64, 2u64))
        } else {
            let a283 = v0;
            let a284 = 2u64;
            if nat_eq(a283, a284) {
                Adt6::C1(Adt7::C0(2u64, 32u64, 18u64))
            } else {
                let a285 = v0;
                let a286 = 3u64;
                if nat_eq(a285, a286) {
                    Adt6::C1(Adt7::C0(3u64, 32u64, 20u64))
                } else {
                    let a287 = v0;
                    let a288 = 6u64;
                    if nat_eq(a287, a288) {
                        Adt6::C1(Adt7::C0(6u64, 32u64, 22u64))
                    } else {
                        let a289 = v0;
                        let a290 = 7u64;
                        if nat_eq(a289, a290) {
                            Adt6::C1(Adt7::C0(7u64, 32u64, 24u64))
                        } else {
                            let a291 = v0;
                            let a292 = 8u64;
                            if nat_eq(a291, a292) {
                                Adt6::C1(Adt7::C0(8u64, 32u64, 34u64))
                            } else {
                                let a293 = v0;
                                let a294 = 9u64;
                                if nat_eq(a293, a294) {
                                    Adt6::C1(Adt7::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a295 = v0;
                                    let a296 = 10u64;
                                    if nat_eq(a295, a296) {
                                        Adt6::C1(Adt7::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a297 = v0;
                                        let a298 = 11u64;
                                        if nat_eq(a297, a298) {
                                            Adt6::C1(Adt7::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a299 = v0;
                                            let a300 = 12u64;
                                            if nat_eq(a299, a300) {
                                                Adt6::C1(Adt7::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a301 = v0;
                                                let a302 = 13u64;
                                                if nat_eq(a301, a302) {
                                                    Adt6::C1(Adt7::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a303 = v0;
                                                    let a304 = 14u64;
                                                    if nat_eq(a303, a304) {
                                                        Adt6::C1(Adt7::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a305 = v0;
                                                        let a306 = 15u64;
                                                        if nat_eq(a305, a306) {
                                                            Adt6::C1(Adt7::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a307 = v0;
                                                            let a308 = 16u64;
                                                            if nat_eq(a307, a308) {
                                                                Adt6::C1(Adt7::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a309 = v0;
                                                                let a310 = 17u64;
                                                                if nat_eq(a309, a310) {
                                                                    Adt6::C1(Adt7::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a311 = v0;
                                                                    let a312 = 18u64;
                                                                    if nat_eq(a311, a312) {
                                                                        Adt6::C1(Adt7::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a313 = v0;
                                                                        let a314 = 19u64;
                                                                        if nat_eq(a313, a314) {
                                                                            Adt6::C1(Adt7::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a315 = v0;
                                                                            let a316 = 20u64;
                                                                            if nat_eq(a315, a316) {
                                                                                Adt6::C1(Adt7::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a317 = v0;
                                                                                let a318 = 21u64;
                                                                                if nat_eq(a317, a318) {
                                                                                    Adt6::C1(Adt7::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a319 = v0;
                                                                                    let a320 = 22u64;
                                                                                    if nat_eq(a319, a320) {
                                                                                        Adt6::C1(Adt7::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a321 = v0;
                                                                                        let a322 = 23u64;
                                                                                        if nat_eq(a321, a322) {
                                                                                            Adt6::C1(Adt7::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a323 = v0;
                                                                                            let a324 = 24u64;
                                                                                            if nat_eq(a323, a324) {
                                                                                                Adt6::C1(Adt7::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a325 = v0;
                                                                                                let a326 = 25u64;
                                                                                                if nat_eq(a325, a326) {
                                                                                                    Adt6::C1(Adt7::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a327 = v0;
                                                                                                    let a328 = 26u64;
                                                                                                    if nat_eq(a327, a328) {
                                                                                                        Adt6::C1(Adt7::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a329 = v0;
                                                                                                        let a330 = 27u64;
                                                                                                        if nat_eq(a329, a330) {
                                                                                                            Adt6::C1(Adt7::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a331 = v0;
                                                                                                            let a332 = 28u64;
                                                                                                            if nat_eq(a331, a332) {
                                                                                                                Adt6::C1(Adt7::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a333 = v0;
                                                                                                                let a334 = 29u64;
                                                                                                                if nat_eq(a333, a334) {
                                                                                                                    Adt6::C1(Adt7::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a335 = v0;
                                                                                                                    let a336 = 30u64;
                                                                                                                    if nat_eq(a335, a336) {
                                                                                                                        Adt6::C1(Adt7::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a337 = v0;
                                                                                                                        let a338 = 34u64;
                                                                                                                        if nat_eq(a337, a338) {
                                                                                                                            Adt6::C1(Adt7::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a339 = v0;
                                                                                                                            let a340 = 35u64;
                                                                                                                            if nat_eq(a339, a340) {
                                                                                                                                Adt6::C1(Adt7::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a341 = v0;
                                                                                                                                let a342 = 39u64;
                                                                                                                                if nat_eq(a341, a342) {
                                                                                                                                    Adt6::C1(Adt7::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a343 = v0;
                                                                                                                                    let a344 = 40u64;
                                                                                                                                    if nat_eq(a343, a344) {
                                                                                                                                        Adt6::C1(Adt7::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a345 = v0;
                                                                                                                                        let a346 = 41u64;
                                                                                                                                        if nat_eq(a345, a346) {
                                                                                                                                            Adt6::C1(Adt7::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a347 = v0;
                                                                                                                                            let a348 = 42u64;
                                                                                                                                            if nat_eq(a347, a348) {
                                                                                                                                                Adt6::C1(Adt7::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a349 = v0;
                                                                                                                                                let a350 = 43u64;
                                                                                                                                                if nat_eq(a349, a350) {
                                                                                                                                                    Adt6::C1(Adt7::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt6::C0
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

pub fn f12(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64, v5: u64, v6: bool) -> R<Adt5> {
    if f14(v1.clone(), v2.clone(), v3.clone(), v6, v6, v4, v5) {
        let m364 = f20(v0, 0u64, 0u64, match v3.clone() {
            Adt1::C0(h351, _, _, _, _, _, _, _, _) => {
                h351
            }
        }, match v1.clone() {
            Adt1::C0(_, h352, _, _, _, _, _, _, _) => {
                h352
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h353, _, _, _, _, _, _) => {
                h353
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h354, _, _, _, _, _) => {
                h354
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h355, _, _, _, _) => {
                h355
            }
        }, match v2.clone() {
            Adt1::C0(_, h356, _, _, _, _, _, _, _) => {
                h356
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h357, _, _, _, _, _, _) => {
                h357
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h358, _, _, _, _, _) => {
                h358
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h359, _, _, _, _) => {
                h359
            }
        }, match v3.clone() {
            Adt1::C0(_, h360, _, _, _, _, _, _, _) => {
                h360
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h361, _, _, _, _, _, _) => {
                h361
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h362, _, _, _, _, _) => {
                h362
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h363, _, _, _, _) => {
                h363
            }
        })?;
        match m364 {
            Adt8::C0 => {
                Ok(Adt5::C0)
            }
            Adt8::C1 => {
                Ok(Adt5::C0)
            }
            Adt8::C2(v7, _, v9, v10, v11, v12, v13, _, _, _) => {
                Ok(Adt5::C1(f15(v1, v2, v3, v4, v5, v7, v9, v10, v11, v12, v13, None::<Adt3>, v6)?))
            }
        }
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f13(v0: u64) -> bool {
    let a365 = v0;
    let a366 = 0u64;
    if if nat_eq(a365, a366) {
        true
    } else {
        let a367 = v0;
        let a368 = 1u64;
        nat_eq(a367, a368)
    } {
        true
    } else {
        let a369 = v0;
        let a370 = 30u64;
        nat_eq(a369, a370)
    }
}

pub fn f14(v0: Adt1, v1: Adt1, v2: Adt1, v3: bool, v4: bool, v5: u64, v6: u64) -> bool {
    if if if if if if if if f21(match v0.clone() {
        Adt1::C0(h371, _, _, _, _, _, _, _, _) => {
            h371
        }
    }) {
        true
    } else {
        if v3 {
            if f13(match v0.clone() {
                Adt1::C0(h372, _, _, _, _, _, _, _, _) => {
                    h372
                }
            }) {
                true
            } else {
                let a374 = match v0.clone() {
                    Adt1::C0(h373, _, _, _, _, _, _, _, _) => {
                        h373
                    }
                };
                let a375 = 43u64;
                nat_eq(a374, a375)
            }
        } else {
            false
        }
    } {
        let a380 = {
            let a377 = match v0.clone() {
                Adt1::C0(_, h376, _, _, _, _, _, _, _) => {
                    h376
                }
            };
            let a378 = v5;
            let a379 = 0u64;
            nat_rem(a377, a378, a379)
        };
        let a381 = 0u64;
        nat_eq(a380, a381)
    } else {
        false
    } {
        let a383 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h382, _, _, _) => {
                h382
            }
        };
        let a384 = v6;
        nat_eq(a383, a384)
    } else {
        false
    } {
        if f22(match v1.clone() {
            Adt1::C0(h385, _, _, _, _, _, _, _, _) => {
                h385
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, h386, _, _, _) => {
                h386
            }
        }, v4) {
            true
        } else {
            f10(match v0 {
                Adt1::C0(h387, _, _, _, _, _, _, _, _) => {
                    h387
                }
            }, match v1.clone() {
                Adt1::C0(h388, _, _, _, _, _, _, _, _) => {
                    h388
                }
            }, match v1 {
                Adt1::C0(_, _, _, _, _, h389, _, _, _) => {
                    h389
                }
            })
        }
    } else {
        false
    } {
        let a391 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h390, _, _, _) => {
                h390
            }
        };
        let a392 = 4u64;
        nat_eq(a391, a392)
    } else {
        false
    } {
        let a395 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h393, _, _, _) => {
                h393
            }
        };
        let a396 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h394, _, _) => {
                h394
            }
        };
        nat_le(a395, a396)
    } else {
        false
    } {
        let a399 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h397, _, _) => {
                h397
            }
        };
        let a400 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h398, _) => {
                h398
            }
        };
        nat_le(a399, a400)
    } else {
        false
    } {
        let a403 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h401, _) => {
                h401
            }
        };
        let a404 = match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h402) => {
                h402
            }
        };
        nat_le(a403, a404)
    } else {
        false
    }
}

pub fn f15(v0: Adt1, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: Option<Adt3>, v12: bool) -> R<Adt2> {
    Ok(Adt2::C0(if v12 {
        match v0.clone() {
            Adt1::C0(_, h405, _, _, _, _, _, _, _) => {
                h405
            }
        }
    } else {
        let a408 = match v0.clone() {
            Adt1::C0(_, h406, _, _, _, _, _, _, _) => {
                h406
            }
        };
        let a409 = f23(match v0.clone() {
            Adt1::C0(h407, _, _, _, _, _, _, _, _) => {
                h407
            }
        }, v3);
        let a410 = 0u64;
        nat_quot(a408, a409, a410)
    }, {
        let a415 = {
            let a412 = match v0.clone() {
                Adt1::C0(_, h411, _, _, _, _, _, _, _) => {
                    h411
                }
            };
            let a413 = v3;
            let a414 = 0u64;
            nat_quot(a412, a413, a414)
        };
        let a416 = v4;
        nat_mul(a415, a416)?
    }, v5, v6, v7, {
        let a419 = v6;
        let a420 = {
            let a417 = v7;
            let a418 = v8;
            nat_mul(a417, a418)?
        };
        nat_mul(a419, a420)?
    }, {
        let a425 = v5;
        let a426 = {
            let a423 = v6;
            let a424 = {
                let a421 = v7;
                let a422 = v8;
                nat_mul(a421, a422)?
            };
            nat_mul(a423, a424)?
        };
        nat_mul(a425, a426)?
    }, v9, v10, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h427, _, _) => {
            h427
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h428, _) => {
            h428
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h429) => {
            h429
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h430, _, _) => {
            h430
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h431, _) => {
            h431
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h432) => {
            h432
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h433, _, _) => {
            h433
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h434, _) => {
            h434
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h435) => {
            h435
        }
    }, v11, if v12 {
        Some(0u64)
    } else {
        if if f13(match v0.clone() {
            Adt1::C0(h436, _, _, _, _, _, _, _, _) => {
                h436
            }
        }) {
            true
        } else {
            let a438 = match v0.clone() {
                Adt1::C0(h437, _, _, _, _, _, _, _, _) => {
                    h437
                }
            };
            let a439 = 43u64;
            nat_eq(a438, a439)
        } {
            Some(f24(match v0.clone() {
                Adt1::C0(h440, _, _, _, _, _, _, _, _) => {
                    h440
                }
            }))
        } else {
            None::<u64>
        }
    }, match v0.clone() {
        Adt1::C0(h441, _, _, _, _, _, _, _, _) => {
            h441
        }
    }, f25(match v0 {
        Adt1::C0(h442, _, _, _, _, _, _, _, _) => {
            h442
        }
    })))
}

pub fn f16(v0: u64) -> bool {
    let a443 = 10u64;
    let a444 = v0;
    if if if if if nat_le(a443, a444) {
        let a445 = v0;
        let a446 = 14u64;
        nat_le(a445, a446)
    } else {
        false
    } {
        true
    } else {
        let a447 = 16u64;
        let a448 = v0;
        if nat_le(a447, a448) {
            let a449 = v0;
            let a450 = 19u64;
            nat_le(a449, a450)
        } else {
            false
        }
    } {
        true
    } else {
        let a451 = 21u64;
        let a452 = v0;
        if nat_le(a451, a452) {
            let a453 = v0;
            let a454 = 23u64;
            nat_le(a453, a454)
        } else {
            false
        }
    } {
        true
    } else {
        let a455 = 34u64;
        let a456 = v0;
        if nat_le(a455, a456) {
            let a457 = v0;
            let a458 = 35u64;
            nat_le(a457, a458)
        } else {
            false
        }
    } {
        true
    } else {
        let a459 = v0;
        let a460 = 29u64;
        nat_eq(a459, a460)
    }
}

pub fn f17(v0: u64) -> bool {
    let a461 = v0;
    let a462 = 3u64;
    if nat_eq(a461, a462) {
        true
    } else {
        let a463 = v0;
        let a464 = 7u64;
        nat_eq(a463, a464)
    }
}

pub fn f18(v0: u64) -> bool {
    let a465 = v0;
    let a466 = 2u64;
    if if if if nat_eq(a465, a466) {
        true
    } else {
        let a467 = v0;
        let a468 = 6u64;
        nat_eq(a467, a468)
    } {
        true
    } else {
        let a469 = v0;
        let a470 = 8u64;
        nat_eq(a469, a470)
    } {
        true
    } else {
        let a471 = v0;
        let a472 = 20u64;
        nat_eq(a471, a472)
    } {
        true
    } else {
        f26(v0)
    }
}

pub fn f19(v0: u64) -> bool {
    let a473 = v0;
    let a474 = 39u64;
    if nat_eq(a473, a474) {
        true
    } else {
        let a475 = v0;
        let a476 = 40u64;
        nat_eq(a475, a476)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt8> {
    let a477 = v0;
    let a478 = 28u64;
    if if if if nat_eq(a477, a478) {
        let a479 = v1;
        let a480 = 0u64;
        nat_eq(a479, a480)
    } else {
        false
    } {
        let a481 = v2;
        let a482 = 0u64;
        nat_eq(a481, a482)
    } else {
        false
    } {
        let a483 = v3;
        let a484 = 0u64;
        nat_eq(a483, a484)
    } else {
        false
    } {
        let a485 = v4;
        let a486 = v8;
        if if if if if if if if nat_eq(a485, a486) {
            let a487 = v5;
            let a488 = v12;
            nat_eq(a487, a488)
        } else {
            false
        } {
            let a489 = v9;
            let a490 = v13;
            nat_eq(a489, a490)
        } else {
            false
        } {
            let a491 = v10;
            let a492 = v14;
            nat_eq(a491, a492)
        } else {
            false
        } {
            let a493 = v11;
            let a494 = v15;
            nat_eq(a493, a494)
        } else {
            false
        } {
            let a495 = 0u64;
            let a496 = v6;
            nat_lt(a495, a496)
        } else {
            false
        } {
            let a497 = 0u64;
            let a498 = v7;
            nat_lt(a497, a498)
        } else {
            false
        } {
            let a502 = {
                let a499 = v10;
                let a500 = v6;
                let a501 = 1u64;
                nat_rem(a499, a500, a501)
            };
            let a503 = 0u64;
            if nat_eq(a502, a503) {
                let a507 = {
                    let a504 = v11;
                    let a505 = v7;
                    let a506 = 1u64;
                    nat_rem(a504, a505, a506)
                };
                let a508 = 0u64;
                nat_eq(a507, a508)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt8::C2(v5, v4, v9, v10, v11, {
                let a509 = v10;
                let a510 = v6;
                let a511 = 0u64;
                nat_quot(a509, a510, a511)
            }, {
                let a512 = v11;
                let a513 = v7;
                let a514 = 0u64;
                nat_quot(a512, a513, a514)
            }, {
                let a515 = v5;
                let a516 = v4;
                nat_mul(a515, a516)?
            }, {
                let a517 = v4;
                let a518 = v9;
                nat_mul(a517, a518)?
            }, {
                let a519 = v5;
                let a520 = v9;
                nat_mul(a519, a520)?
            }))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f21(v0: u64) -> bool {
    if if f27(v0) {
        true
    } else {
        f19(v0)
    } {
        true
    } else {
        f16(v0)
    }
}

pub fn f22(v0: u64, v1: u64, v2: bool) -> bool {
    let a521 = v0;
    let a522 = 0u64;
    if if nat_eq(a521, a522) {
        let a523 = v1;
        let a524 = 4u64;
        nat_eq(a523, a524)
    } else {
        false
    } {
        true
    } else {
        let a525 = v0;
        let a526 = 1u64;
        if if nat_eq(a525, a526) {
            true
        } else {
            if v2 {
                let a527 = v0;
                let a528 = 30u64;
                nat_eq(a527, a528)
            } else {
                false
            }
        } {
            let a529 = v1;
            let a530 = 2u64;
            nat_eq(a529, a530)
        } else {
            false
        }
    }
}

pub fn f23(v0: u64, v1: u64) -> u64 {
    if if f19(v0) {
        true
    } else {
        f26(v0)
    } {
        32u64
    } else {
        v1
    }
}

pub fn f24(v0: u64) -> u64 {
    let a531 = v0;
    let a532 = 30u64;
    if nat_eq(a531, a532) {
        30u64
    } else {
        0u64
    }
}

pub fn f25(v0: u64) -> Option<Adt4> {
    let m533 = f28(v0);
    match m533 {
        Adt9::C0 => {
            None::<Adt4>
        }
        Adt9::C1(v1, v2, v3, v4) => {
            Some(Adt4::C0(v1, v2, v3, v4))
        }
    }
}

pub fn f26(v0: u64) -> bool {
    let a534 = v0;
    let a535 = 41u64;
    if nat_eq(a534, a535) {
        true
    } else {
        let a536 = v0;
        let a537 = 42u64;
        nat_eq(a536, a537)
    }
}

pub fn f27(v0: u64) -> bool {
    if f18(v0) {
        true
    } else {
        f17(v0)
    }
}

pub fn f28(v0: u64) -> Adt9 {
    let a538 = v0;
    let a539 = 0u64;
    if if if if nat_eq(a538, a539) {
        true
    } else {
        let a540 = v0;
        let a541 = 1u64;
        nat_eq(a540, a541)
    } {
        true
    } else {
        let a542 = v0;
        let a543 = 30u64;
        nat_eq(a542, a543)
    } {
        true
    } else {
        let a544 = v0;
        let a545 = 43u64;
        nat_eq(a544, a545)
    } {
        Adt9::C1(2u64, 1u64, 0u64, 2u64)
    } else {
        Adt9::C0
    }
}
