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
    C0(u64, Adt1, Adt2, Option<Adt2>, Adt2),
    C1(u64, Adt3, Adt3, Adt4),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2(u64, Adt3),
    C3(u64, u64, u64, u64, bool),
    C4(u64),
    C5(u64, u64, u64, u64),
    C6(u64, u64, bool),
    C7(u64),
    C8(u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64),
    C3(u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1(Adt8),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5) => {
            let v6: u64 = match v2.clone() {
                Adt1::C0(h2, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2
                }
            };
            Ok(Adt5::C0(if f1(v1) {
                f2(v1, v3, {
                    let m3 = v4;
                    match m3 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }, v5)?
            } else {
                let a4 = v1;
                let a5 = 19u64;
                if nat_eq(a4, a5) {
                    f3(v3, v5)?
                } else {
                    let a6 = v1;
                    let a7 = 21u64;
                    if nat_eq(a6, a7) {
                        f4(v6, v3, v4, v5)?
                    } else {
                        let a8 = v1;
                        let a9 = 61u64;
                        if if if nat_eq(a8, a9) {
                            true
                        } else {
                            let a10 = v1;
                            let a11 = 62u64;
                            nat_eq(a10, a11)
                        } {
                            true
                        } else {
                            let a12 = v1;
                            let a13 = 63u64;
                            nat_eq(a12, a13)
                        } {
                            f5(v1, v2, v3, v5)?
                        } else {
                            let a14 = v1;
                            let a15 = 4u64;
                            if if nat_eq(a14, a15) {
                                true
                            } else {
                                let a16 = v1;
                                let a17 = 32u64;
                                nat_eq(a16, a17)
                            } {
                                f6(v1, v2, v3, v4, v5)?
                            } else {
                                let a18 = v1;
                                let a19 = 20u64;
                                if nat_eq(a18, a19) {
                                    f7(v3, v5)?
                                } else {
                                    let a20 = v1;
                                    let a21 = 42u64;
                                    if nat_eq(a20, a21) {
                                        f8(4u64, v3, v5)?
                                    } else {
                                        let a22 = v1;
                                        let a23 = 69u64;
                                        if nat_eq(a22, a23) {
                                            let a24 = v6;
                                            let a25 = 4u64;
                                            if nat_lt(a24, a25) {
                                                f8(v6, v3, v5)?
                                            } else {
                                                Adt6::C1
                                            }
                                        } else {
                                            f9(v1, v6, v3, v4, v5)?
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }))
        }
        Adt0::C1(v8, v9, v10, v11) => {
            f10(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a26 = v0;
    let a27 = 1u64;
    if if nat_eq(a26, a27) {
        true
    } else {
        let a28 = v0;
        let a29 = 33u64;
        nat_eq(a28, a29)
    } {
        true
    } else {
        let a30 = v0;
        let a31 = 34u64;
        nat_eq(a30, a31)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a34 = match v1.clone() {
        Adt2::C0(h32, _, _, _, _, _, _, _, _, _, _) => {
            h32
        }
    };
    let a35 = match v3.clone() {
        Adt2::C0(h33, _, _, _, _, _, _, _, _, _, _) => {
            h33
        }
    };
    if if nat_eq(a34, a35) {
        let a36 = v0;
        let a37 = 33u64;
        if nat_eq(a36, a37) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m39 = f11(match v1.clone() {
            Adt2::C0(h38, _, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        });
        match m39 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h40, _) => {
                        h40
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a45 = {
                    let a42 = match v1.clone() {
                        Adt2::C0(_, h41, _, _, _, _, _, _, _, _, _) => {
                            h41
                        }
                    };
                    let a43 = v5;
                    let a44 = 1u64;
                    nat_rem(a42, a43, a44)
                };
                let a46 = 0u64;
                if if if nat_eq(a45, a46) {
                    let a51 = {
                        let a48 = match v3.clone() {
                            Adt2::C0(_, h47, _, _, _, _, _, _, _, _, _) => {
                                h47
                            }
                        };
                        let a49 = v5;
                        let a50 = 1u64;
                        nat_rem(a48, a49, a50)
                    };
                    let a52 = 0u64;
                    nat_eq(a51, a52)
                } else {
                    false
                } {
                    let a53 = v6;
                    let a54 = f12(v3.clone(), v5)?;
                    nat_eq(a53, a54)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h55) => {
                            h55
                        }
                    }, f13(v1, v5), f13(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f3(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a57 = match v0.clone() {
        Adt2::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
            h56
        }
    };
    let a58 = 0u64;
    if if if if nat_eq(a57, a58) {
        true
    } else {
        let a60 = match v0.clone() {
            Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                h59
            }
        };
        let a61 = 1u64;
        nat_eq(a60, a61)
    } {
        true
    } else {
        let a63 = match v0.clone() {
            Adt2::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        };
        let a64 = 25u64;
        if nat_eq(a63, a64) {
            true
        } else {
            let a66 = match v0.clone() {
                Adt2::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                    h65
                }
            };
            let a67 = 26u64;
            nat_eq(a66, a67)
        }
    } {
        true
    } else {
        let a69 = match v0.clone() {
            Adt2::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        let a70 = 30u64;
        nat_eq(a69, a70)
    } {
        let a73 = match v0.clone() {
            Adt2::C0(h71, _, _, _, _, _, _, _, _, _, _) => {
                h71
            }
        };
        let a74 = match v1.clone() {
            Adt2::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        };
        if if nat_eq(a73, a74) {
            f14(match v1.clone() {
                Adt2::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h76, _, _, _, _, _, _, _, _) => {
                    h76
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h77, _, _, _, _, _, _, _) => {
                    h77
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h78, _, _, _, _, _, _) => {
                    h78
                }
            }, match v0.clone() {
                Adt2::C0(_, h79, _, _, _, _, _, _, _, _, _) => {
                    h79
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h80, _, _, _, _, _, _, _, _) => {
                    h80
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h81, _, _, _, _, _, _, _) => {
                    h81
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h82, _, _, _, _, _, _) => {
                    h82
                }
            })
        } else {
            false
        } {
            let m84 = f11(match v0.clone() {
                Adt2::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                    h83
                }
            });
            match m84 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h85) => {
                            h85
                        }
                    }, f13(v0, 1u64), f13(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f4(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m86 = v2;
    match m86 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a87 = v0;
            let a88 = 4u64;
            if if if nat_lt(a87, a88) {
                let a91 = match v1.clone() {
                    Adt2::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                        h89
                    }
                };
                let a92 = match v4.clone() {
                    Adt2::C0(h90, _, _, _, _, _, _, _, _, _, _) => {
                        h90
                    }
                };
                if nat_eq(a91, a92) {
                    let a95 = match v1.clone() {
                        Adt2::C0(h93, _, _, _, _, _, _, _, _, _, _) => {
                            h93
                        }
                    };
                    let a96 = match v3.clone() {
                        Adt2::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                            h94
                        }
                    };
                    nat_eq(a95, a96)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f15(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h98, _, _, _, _, _, _, _, _, _) => {
                        h98
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                        h99
                    }
                })? {
                    f15(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h100, _, _, _, _, _, _, _, _) => {
                            h100
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h101, _, _, _, _, _, _, _, _) => {
                            h101
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h102, _, _, _, _, _, _, _, _) => {
                            h102
                        }
                    })?
                } else {
                    false
                } {
                    f15(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h103, _, _, _, _, _, _, _) => {
                            h103
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                            h104
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h105, _, _, _, _, _, _, _) => {
                            h105
                        }
                    })?
                } else {
                    false
                } {
                    f15(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h106, _, _, _, _, _, _) => {
                            h106
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h107, _, _, _, _, _, _) => {
                            h107
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h108, _, _, _, _, _, _) => {
                            h108
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m110 = f11(match v1.clone() {
                    Adt2::C0(h109, _, _, _, _, _, _, _, _, _, _) => {
                        h109
                    }
                });
                match m110 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h111, _) => {
                                h111
                            }
                        };
                        let a116 = {
                            let a113 = match v1.clone() {
                                Adt2::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
                                    h112
                                }
                            };
                            let a114 = v6;
                            let a115 = 1u64;
                            nat_rem(a113, a114, a115)
                        };
                        let a117 = 0u64;
                        if if if nat_eq(a116, a117) {
                            let a122 = {
                                let a119 = match v4.clone() {
                                    Adt2::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                                        h118
                                    }
                                };
                                let a120 = v6;
                                let a121 = 1u64;
                                nat_rem(a119, a120, a121)
                            };
                            let a123 = 0u64;
                            nat_eq(a122, a123)
                        } else {
                            false
                        } {
                            let a128 = {
                                let a125 = match v3.clone() {
                                    Adt2::C0(_, h124, _, _, _, _, _, _, _, _, _) => {
                                        h124
                                    }
                                };
                                let a126 = v6;
                                let a127 = 1u64;
                                nat_rem(a125, a126, a127)
                            };
                            let a129 = 0u64;
                            nat_eq(a128, a129)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h130) => {
                                    h130
                                }
                            }, f13(v1, v6), f13(v3, v6), Adt4::C2(v0, f13(v4, v6))))
                        } else {
                            Ok(Adt6::C1)
                        }
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a132 = match v2.clone() {
        Adt2::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
            h131
        }
    };
    let a133 = 0u64;
    if if nat_eq(a132, a133) {
        let a135 = match v3.clone() {
            Adt2::C0(h134, _, _, _, _, _, _, _, _, _, _) => {
                h134
            }
        };
        let a136 = 0u64;
        nat_eq(a135, a136)
    } else {
        false
    } {
        let a137 = v0;
        let a138 = 61u64;
        if nat_eq(a137, a138) {
            let v4: bool = {
                let a140 = 0u64;
                let a141 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h139, _, _, _, _, _, _, _) => {
                        h139
                    }
                };
                nat_lt(a140, a141)
            };
            if if if if if if f16(match v2.clone() {
                Adt2::C0(_, h142, _, _, _, _, _, _, _, _, _) => {
                    h142
                }
            }, match v1.clone() {
                Adt1::C0(h143, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h143
                }
            }, match v1.clone() {
                Adt1::C0(_, h144, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h144
                }
            }, match v3.clone() {
                Adt2::C0(_, h145, _, _, _, _, _, _, _, _, _) => {
                    h145
                }
            })? {
                f16(match v2.clone() {
                    Adt2::C0(_, _, h146, _, _, _, _, _, _, _, _) => {
                        h146
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h147, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h147
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h148, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h148
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h149, _, _, _, _, _, _, _, _) => {
                        h149
                    }
                })?
            } else {
                false
            } {
                f16(match v2.clone() {
                    Adt2::C0(_, _, _, h150, _, _, _, _, _, _, _) => {
                        h150
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h151, _, _, _, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h152, _, _, _, _, _, _, _, _, _, _) => {
                        h152
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h153, _, _, _, _, _, _, _) => {
                        h153
                    }
                })?
            } else {
                false
            } {
                f16(match v2.clone() {
                    Adt2::C0(_, _, _, _, h154, _, _, _, _, _, _) => {
                        h154
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h155, _, _, _, _, _, _, _, _, _) => {
                        h155
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h156, _, _, _, _, _, _, _, _) => {
                        h156
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h157, _, _, _, _, _, _) => {
                        h157
                    }
                })?
            } else {
                false
            } {
                let a159 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h158, _, _, _, _, _, _, _) => {
                        h158
                    }
                };
                let a160 = 4294967295u64;
                nat_le(a159, a160)
            } else {
                false
            } {
                if v4 {
                    !f17(match v2.clone() {
                        Adt2::C0(_, h161, _, _, _, _, _, _, _, _, _) => {
                            h161
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h162, _, _, _, _, _, _, _, _) => {
                            h162
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h163, _, _, _, _, _, _, _) => {
                            h163
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h164, _, _, _, _, _, _) => {
                            h164
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h165, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h166, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h166
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h167, _, _, _, _, _, _, _, _, _, _, _) => {
                        h167
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h168, _, _, _, _, _, _, _, _, _) => {
                        h168
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a169 = v0;
            let a170 = 62u64;
            if nat_eq(a169, a170) {
                let a172 = match v1.clone() {
                    Adt1::C0(h171, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h171
                    }
                };
                let a173 = 2147483648u64;
                if if if if if nat_lt(a172, a173) {
                    let a175 = match v1.clone() {
                        Adt1::C0(_, h174, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h174
                        }
                    };
                    let a176 = 2147483648u64;
                    nat_lt(a175, a176)
                } else {
                    false
                } {
                    let a179 = match v1.clone() {
                        Adt1::C0(h177, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h177
                        }
                    };
                    let a180 = match v2.clone() {
                        Adt2::C0(_, h178, _, _, _, _, _, _, _, _, _) => {
                            h178
                        }
                    };
                    nat_lt(a179, a180)
                } else {
                    false
                } {
                    let a183 = match v1.clone() {
                        Adt1::C0(_, h181, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h181
                        }
                    };
                    let a184 = match v2.clone() {
                        Adt2::C0(_, h182, _, _, _, _, _, _, _, _, _) => {
                            h182
                        }
                    };
                    nat_lt(a183, a184)
                } else {
                    false
                } {
                    let a187 = match v2.clone() {
                        Adt2::C0(_, _, h185, _, _, _, _, _, _, _, _) => {
                            h185
                        }
                    };
                    let a188 = match v3.clone() {
                        Adt2::C0(_, _, h186, _, _, _, _, _, _, _, _) => {
                            h186
                        }
                    };
                    if if if nat_eq(a187, a188) {
                        let a191 = match v2.clone() {
                            Adt2::C0(_, _, _, h189, _, _, _, _, _, _, _) => {
                                h189
                            }
                        };
                        let a192 = match v3.clone() {
                            Adt2::C0(_, _, _, h190, _, _, _, _, _, _, _) => {
                                h190
                            }
                        };
                        nat_eq(a191, a192)
                    } else {
                        false
                    } {
                        let a195 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h193, _, _, _, _, _, _) => {
                                h193
                            }
                        };
                        let a196 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h194, _, _, _, _, _, _) => {
                                h194
                            }
                        };
                        nat_eq(a195, a196)
                    } else {
                        false
                    } {
                        let a205 = match v3.clone() {
                            Adt2::C0(_, h197, _, _, _, _, _, _, _, _, _) => {
                                h197
                            }
                        };
                        let a206 = {
                            let a203 = {
                                let a200 = match v2.clone() {
                                    Adt2::C0(_, h198, _, _, _, _, _, _, _, _, _) => {
                                        h198
                                    }
                                };
                                let a201 = match v1.clone() {
                                    Adt1::C0(h199, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h199
                                    }
                                };
                                nat_add(a200, a201)?
                            };
                            let a204 = match v1.clone() {
                                Adt1::C0(_, h202, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h202
                                }
                            };
                            nat_add(a203, a204)?
                        };
                        nat_eq(a205, a206)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h207, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h207
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a208 = v0;
                let a209 = 63u64;
                if nat_eq(a208, a209) {
                    if if if if if f18(match v2.clone() {
                        Adt2::C0(_, h210, _, _, _, _, _, _, _, _, _) => {
                            h210
                        }
                    }, match v1.clone() {
                        Adt1::C0(h211, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h211
                        }
                    })? {
                        f18(match v2.clone() {
                            Adt2::C0(_, _, h212, _, _, _, _, _, _, _, _) => {
                                h212
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h213, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h213
                            }
                        })?
                    } else {
                        false
                    } {
                        f18(match v2.clone() {
                            Adt2::C0(_, _, _, h214, _, _, _, _, _, _, _) => {
                                h214
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h215, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h215
                            }
                        })?
                    } else {
                        false
                    } {
                        f18(match v2.clone() {
                            Adt2::C0(_, _, _, _, h216, _, _, _, _, _, _) => {
                                h216
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h217, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h217
                            }
                        })?
                    } else {
                        false
                    } {
                        let a220 = match v2.clone() {
                            Adt2::C0(_, h218, _, _, _, _, _, _, _, _, _) => {
                                h218
                            }
                        };
                        let a221 = match v3.clone() {
                            Adt2::C0(_, h219, _, _, _, _, _, _, _, _, _) => {
                                h219
                            }
                        };
                        if if if nat_eq(a220, a221) {
                            let a224 = match v2.clone() {
                                Adt2::C0(_, _, h222, _, _, _, _, _, _, _, _) => {
                                    h222
                                }
                            };
                            let a225 = match v3.clone() {
                                Adt2::C0(_, _, h223, _, _, _, _, _, _, _, _) => {
                                    h223
                                }
                            };
                            nat_eq(a224, a225)
                        } else {
                            false
                        } {
                            let a228 = match v2.clone() {
                                Adt2::C0(_, _, _, h226, _, _, _, _, _, _, _) => {
                                    h226
                                }
                            };
                            let a229 = match v3.clone() {
                                Adt2::C0(_, _, _, h227, _, _, _, _, _, _, _) => {
                                    h227
                                }
                            };
                            nat_eq(a228, a229)
                        } else {
                            false
                        } {
                            let a232 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h230, _, _, _, _, _, _) => {
                                    h230
                                }
                            };
                            let a233 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h231, _, _, _, _, _, _) => {
                                    h231
                                }
                            };
                            nat_eq(a232, a233)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h234, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h234
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h235, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h235
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h236, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h236
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h237, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h237
                            }
                        })))
                    } else {
                        Ok(Adt6::C1)
                    }
                } else {
                    Ok(Adt6::C0)
                }
            }
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m238 = v3;
    match m238 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a239 = v0;
                let a240 = 4u64;
                nat_eq(a239, a240)
            };
            let v7: bool = {
                let a242 = 0u64;
                let a243 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h241, _, _, _, _, _, _, _, _, _, _, _) => {
                        h241
                    }
                };
                nat_lt(a242, a243)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h244, _, _, _, _, _, _, _, _, _) => {
                    h244
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h245, _, _, _, _, _, _, _, _) => {
                    h245
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h246, _, _, _, _, _, _, _) => {
                    h246
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h247, _, _, _, _, _, _) => {
                    h247
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h248, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h248
                }
            }, match v1.clone() {
                Adt1::C0(_, h249, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h249
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h250, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h250
                }
            });
            if if if if if if if if if if if if v6 {
                let a252 = match v2.clone() {
                    Adt2::C0(h251, _, _, _, _, _, _, _, _, _, _) => {
                        h251
                    }
                };
                let a253 = 0u64;
                if nat_eq(a252, a253) {
                    let a255 = match v5.clone() {
                        Adt2::C0(h254, _, _, _, _, _, _, _, _, _, _) => {
                            h254
                        }
                    };
                    let a256 = 0u64;
                    nat_eq(a255, a256)
                } else {
                    false
                }
            } else {
                let a258 = match v2.clone() {
                    Adt2::C0(h257, _, _, _, _, _, _, _, _, _, _) => {
                        h257
                    }
                };
                let a259 = 0u64;
                if if nat_eq(a258, a259) {
                    true
                } else {
                    let a261 = match v2.clone() {
                        Adt2::C0(h260, _, _, _, _, _, _, _, _, _, _) => {
                            h260
                        }
                    };
                    let a262 = 26u64;
                    nat_eq(a261, a262)
                } {
                    let a264 = match v5.clone() {
                        Adt2::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
                            h263
                        }
                    };
                    let a265 = 0u64;
                    if nat_eq(a264, a265) {
                        true
                    } else {
                        let a267 = match v5.clone() {
                            Adt2::C0(h266, _, _, _, _, _, _, _, _, _, _) => {
                                h266
                            }
                        };
                        let a268 = 26u64;
                        nat_eq(a267, a268)
                    }
                } else {
                    false
                }
            } {
                let a271 = match v2.clone() {
                    Adt2::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                        h269
                    }
                };
                let a272 = match v4.clone() {
                    Adt2::C0(h270, _, _, _, _, _, _, _, _, _, _) => {
                        h270
                    }
                };
                nat_eq(a271, a272)
            } else {
                false
            } {
                f19(v2.clone(), v4.clone())
            } else {
                false
            } {
                f20(v2.clone())?
            } else {
                false
            } {
                f20(v4.clone())?
            } else {
                false
            } {
                let a274 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h273, _, _, _, _, _) => {
                        h273
                    }
                };
                let a275 = 4u64;
                nat_eq(a274, a275)
            } else {
                false
            } {
                let a276 = v8;
                let a277 = f12(v2.clone(), 1u64)?;
                nat_le(a276, a277)
            } else {
                false
            } {
                let a279 = match v1.clone() {
                    Adt1::C0(_, _, _, h278, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h278
                    }
                };
                let a280 = 1073741824u64;
                nat_lt(a279, a280)
            } else {
                false
            } {
                let a282 = match v1.clone() {
                    Adt1::C0(h281, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h281
                    }
                };
                let a283 = 4294967295u64;
                if if if if nat_le(a282, a283) {
                    let a285 = match v1.clone() {
                        Adt1::C0(_, h284, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h284
                        }
                    };
                    let a286 = 4294967295u64;
                    nat_le(a285, a286)
                } else {
                    false
                } {
                    let a288 = match v1.clone() {
                        Adt1::C0(_, _, h287, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h287
                        }
                    };
                    let a289 = 4294967295u64;
                    nat_le(a288, a289)
                } else {
                    false
                } {
                    let a291 = match v1.clone() {
                        Adt1::C0(_, _, _, h290, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h290
                        }
                    };
                    let a292 = 4294967295u64;
                    nat_le(a291, a292)
                } else {
                    false
                } {
                    let a294 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h293, _, _, _, _, _, _, _, _, _, _, _) => {
                            h293
                        }
                    };
                    let a295 = 4294967295u64;
                    nat_le(a294, a295)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a298 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h296) => {
                            h296
                        }
                    };
                    let a299 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h297) => {
                            h297
                        }
                    };
                    nat_eq(a298, a299)
                } else {
                    true
                }
            } else {
                false
            } {
                f21(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h300, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h300
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h301, _) => {
                        h301
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f12(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a302 = v10;
                    let a303 = v8;
                    nat_add(a302, a303)?
                }, 4u64, f13(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h304, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h304
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a306 = match v0.clone() {
        Adt2::C0(h305, _, _, _, _, _, _, _, _, _, _) => {
            h305
        }
    };
    let a307 = 0u64;
    if if if nat_eq(a306, a307) {
        let a309 = match v1.clone() {
            Adt2::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                h308
            }
        };
        let a310 = 0u64;
        nat_eq(a309, a310)
    } else {
        false
    } {
        f14(match v0.clone() {
            Adt2::C0(_, h311, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                h312
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h313, _, _, _, _, _, _, _) => {
                h313
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h314, _, _, _, _, _, _) => {
                h314
            }
        }, match v1.clone() {
            Adt2::C0(_, h315, _, _, _, _, _, _, _, _, _) => {
                h315
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h316, _, _, _, _, _, _, _, _) => {
                h316
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h317, _, _, _, _, _, _, _) => {
                h317
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h318, _, _, _, _, _, _) => {
                h318
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a319 = v2;
            let a320 = f12(v0.clone(), 1u64)?;
            nat_add(a319, a320)?
        }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a322 = match v1.clone() {
        Adt2::C0(h321, _, _, _, _, _, _, _, _, _, _) => {
            h321
        }
    };
    let a323 = 0u64;
    if if if nat_eq(a322, a323) {
        true
    } else {
        let a325 = match v1.clone() {
            Adt2::C0(h324, _, _, _, _, _, _, _, _, _, _) => {
                h324
            }
        };
        let a326 = 1u64;
        nat_eq(a325, a326)
    } {
        true
    } else {
        let a328 = match v1.clone() {
            Adt2::C0(h327, _, _, _, _, _, _, _, _, _, _) => {
                h327
            }
        };
        let a329 = 30u64;
        nat_eq(a328, a329)
    } {
        let a332 = match v1.clone() {
            Adt2::C0(h330, _, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        let a333 = match v2.clone() {
            Adt2::C0(h331, _, _, _, _, _, _, _, _, _, _) => {
                h331
            }
        };
        if if if if if nat_eq(a332, a333) {
            let a336 = match v1.clone() {
                Adt2::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                    h334
                }
            };
            let a337 = match v2.clone() {
                Adt2::C0(_, h335, _, _, _, _, _, _, _, _, _) => {
                    h335
                }
            };
            nat_eq(a336, a337)
        } else {
            false
        } {
            let a340 = match v1.clone() {
                Adt2::C0(_, _, _, h338, _, _, _, _, _, _, _) => {
                    h338
                }
            };
            let a341 = match v2.clone() {
                Adt2::C0(_, _, _, h339, _, _, _, _, _, _, _) => {
                    h339
                }
            };
            nat_eq(a340, a341)
        } else {
            false
        } {
            let a344 = match v1.clone() {
                Adt2::C0(_, _, _, _, h342, _, _, _, _, _, _) => {
                    h342
                }
            };
            let a345 = match v2.clone() {
                Adt2::C0(_, _, _, _, h343, _, _, _, _, _, _) => {
                    h343
                }
            };
            nat_eq(a344, a345)
        } else {
            false
        } {
            let a346 = v0;
            let a347 = 4u64;
            if nat_eq(a346, a347) {
                let a349 = match v1.clone() {
                    Adt2::C0(_, _, h348, _, _, _, _, _, _, _, _) => {
                        h348
                    }
                };
                let a350 = 1u64;
                if nat_eq(a349, a350) {
                    let a353 = match v1.clone() {
                        Adt2::C0(_, h351, _, _, _, _, _, _, _, _, _) => {
                            h351
                        }
                    };
                    let a354 = match v2.clone() {
                        Adt2::C0(_, _, h352, _, _, _, _, _, _, _, _) => {
                            h352
                        }
                    };
                    nat_eq(a353, a354)
                } else {
                    false
                }
            } else {
                let a355 = v0;
                let a356 = 4u64;
                if if nat_lt(a355, a356) {
                    let a359 = match v1.clone() {
                        Adt2::C0(_, h357, _, _, _, _, _, _, _, _, _) => {
                            h357
                        }
                    };
                    let a360 = match v1.clone() {
                        Adt2::C0(_, _, h358, _, _, _, _, _, _, _, _) => {
                            h358
                        }
                    };
                    nat_eq(a359, a360)
                } else {
                    false
                } {
                    let a363 = match v1.clone() {
                        Adt2::C0(_, _, h361, _, _, _, _, _, _, _, _) => {
                            h361
                        }
                    };
                    let a364 = match v2.clone() {
                        Adt2::C0(_, _, h362, _, _, _, _, _, _, _, _) => {
                            h362
                        }
                    };
                    nat_eq(a363, a364)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m366 = f11(match v1.clone() {
                Adt2::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
                    h365
                }
            });
            match m366 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h367) => {
                            h367
                        }
                    }, f13(v1, 1u64), f13(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f9(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m368 = v3;
    match m368 {
        None => {
            f22(v0, v1, match v2.clone() {
                Adt2::C0(h369, _, _, _, _, _, _, _, _, _, _) => {
                    h369
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h370, _, _, _, _, _, _, _, _, _, _) => {
                    h370
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h371, _, _, _, _, _, _, _, _, _) => {
                    h371
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h372, _, _, _, _, _, _, _, _) => {
                    h372
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h373, _, _, _, _, _, _, _) => {
                    h373
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h374, _, _, _, _, _, _) => {
                    h374
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                    h375
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h376, _, _, _, _, _, _, _, _) => {
                    h376
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h377, _, _, _, _, _, _, _) => {
                    h377
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h378, _, _, _, _, _, _) => {
                    h378
                }
            })
        }
        Some(v5) => {
            f22(v0, v1, match v2.clone() {
                Adt2::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
                    h379
                }
            }, match v5.clone() {
                Adt2::C0(h380, _, _, _, _, _, _, _, _, _, _) => {
                    h380
                }
            }, match v4.clone() {
                Adt2::C0(h381, _, _, _, _, _, _, _, _, _, _) => {
                    h381
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h382, _, _, _, _, _, _, _, _, _) => {
                    h382
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h383, _, _, _, _, _, _, _, _) => {
                    h383
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h384, _, _, _, _, _, _, _) => {
                    h384
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h385, _, _, _, _, _, _) => {
                    h385
                }
            }, match v5.clone() {
                Adt2::C0(_, h386, _, _, _, _, _, _, _, _, _) => {
                    h386
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h387, _, _, _, _, _, _, _, _) => {
                    h387
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h388, _, _, _, _, _, _, _) => {
                    h388
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h389, _, _, _, _, _, _) => {
                    h389
                }
            }, match v4.clone() {
                Adt2::C0(_, h390, _, _, _, _, _, _, _, _, _) => {
                    h390
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h391, _, _, _, _, _, _, _, _) => {
                    h391
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h392, _, _, _, _, _, _, _) => {
                    h392
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h393, _, _, _, _, _, _) => {
                    h393
                }
            })
        }
    }
}

pub fn f10(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m394 = v3;
    match m394 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f23(v0, v1)?, f23(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f24(v0, v1, v2.clone())?, f23(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f25(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f26(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f27(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f26(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f28(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f29(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f30(v20, v0, v1, v2)
        }
    }
}

pub fn f11(v0: u64) -> Adt7 {
    let a395 = v0;
    let a396 = 0u64;
    if nat_eq(a395, a396) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a397 = v0;
        let a398 = 1u64;
        if nat_eq(a397, a398) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a399 = v0;
            let a400 = 2u64;
            if nat_eq(a399, a400) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a401 = v0;
                let a402 = 3u64;
                if nat_eq(a401, a402) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a403 = v0;
                    let a404 = 6u64;
                    if nat_eq(a403, a404) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a405 = v0;
                        let a406 = 7u64;
                        if nat_eq(a405, a406) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a407 = v0;
                            let a408 = 8u64;
                            if nat_eq(a407, a408) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a409 = v0;
                                let a410 = 9u64;
                                if nat_eq(a409, a410) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a411 = v0;
                                    let a412 = 10u64;
                                    if nat_eq(a411, a412) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a413 = v0;
                                        let a414 = 11u64;
                                        if nat_eq(a413, a414) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a415 = v0;
                                            let a416 = 12u64;
                                            if nat_eq(a415, a416) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a417 = v0;
                                                let a418 = 13u64;
                                                if nat_eq(a417, a418) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a419 = v0;
                                                    let a420 = 14u64;
                                                    if nat_eq(a419, a420) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a421 = v0;
                                                        let a422 = 15u64;
                                                        if nat_eq(a421, a422) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a423 = v0;
                                                            let a424 = 16u64;
                                                            if nat_eq(a423, a424) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a425 = v0;
                                                                let a426 = 17u64;
                                                                if nat_eq(a425, a426) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a427 = v0;
                                                                    let a428 = 18u64;
                                                                    if nat_eq(a427, a428) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a429 = v0;
                                                                        let a430 = 19u64;
                                                                        if nat_eq(a429, a430) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a431 = v0;
                                                                            let a432 = 20u64;
                                                                            if nat_eq(a431, a432) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a433 = v0;
                                                                                let a434 = 21u64;
                                                                                if nat_eq(a433, a434) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a435 = v0;
                                                                                    let a436 = 22u64;
                                                                                    if nat_eq(a435, a436) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a437 = v0;
                                                                                        let a438 = 23u64;
                                                                                        if nat_eq(a437, a438) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a439 = v0;
                                                                                            let a440 = 24u64;
                                                                                            if nat_eq(a439, a440) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a441 = v0;
                                                                                                let a442 = 25u64;
                                                                                                if nat_eq(a441, a442) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a443 = v0;
                                                                                                    let a444 = 26u64;
                                                                                                    if nat_eq(a443, a444) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a445 = v0;
                                                                                                        let a446 = 27u64;
                                                                                                        if nat_eq(a445, a446) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a447 = v0;
                                                                                                            let a448 = 28u64;
                                                                                                            if nat_eq(a447, a448) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a449 = v0;
                                                                                                                let a450 = 29u64;
                                                                                                                if nat_eq(a449, a450) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a451 = v0;
                                                                                                                    let a452 = 30u64;
                                                                                                                    if nat_eq(a451, a452) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a453 = v0;
                                                                                                                        let a454 = 34u64;
                                                                                                                        if nat_eq(a453, a454) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a455 = v0;
                                                                                                                            let a456 = 35u64;
                                                                                                                            if nat_eq(a455, a456) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a457 = v0;
                                                                                                                                let a458 = 39u64;
                                                                                                                                if nat_eq(a457, a458) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a459 = v0;
                                                                                                                                    let a460 = 40u64;
                                                                                                                                    if nat_eq(a459, a460) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a461 = v0;
                                                                                                                                        let a462 = 41u64;
                                                                                                                                        if nat_eq(a461, a462) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a463 = v0;
                                                                                                                                            let a464 = 42u64;
                                                                                                                                            if nat_eq(a463, a464) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a465 = v0;
                                                                                                                                                let a466 = 43u64;
                                                                                                                                                if nat_eq(a465, a466) {
                                                                                                                                                    Adt7::C1(Adt8::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt7::C0
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

pub fn f12(v0: Adt2, v1: u64) -> R<u64> {
    let a478 = {
        let a472 = {
            let a468 = match v0.clone() {
                Adt2::C0(_, h467, _, _, _, _, _, _, _, _, _) => {
                    h467
                }
            };
            let a469 = v1;
            let a470 = 0u64;
            nat_quot(a468, a469, a470)
        };
        let a473 = match v0.clone() {
            Adt2::C0(_, _, h471, _, _, _, _, _, _, _, _) => {
                h471
            }
        };
        nat_mul(a472, a473)?
    };
    let a479 = {
        let a476 = match v0.clone() {
            Adt2::C0(_, _, _, h474, _, _, _, _, _, _, _) => {
                h474
            }
        };
        let a477 = match v0 {
            Adt2::C0(_, _, _, _, h475, _, _, _, _, _, _) => {
                h475
            }
        };
        nat_mul(a476, a477)?
    };
    nat_mul(a478, a479)
}

pub fn f13(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a481 = match v0.clone() {
            Adt2::C0(_, h480, _, _, _, _, _, _, _, _, _) => {
                h480
            }
        };
        let a482 = v1;
        let a483 = 0u64;
        nat_quot(a481, a482, a483)
    }, match v0.clone() {
        Adt2::C0(_, _, h484, _, _, _, _, _, _, _, _) => {
            h484
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h485, _, _, _, _, _, _, _) => {
            h485
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h486, _, _, _, _, _, _) => {
            h486
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h487, _, _, _, _, _) => {
            h487
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h488, _, _, _, _) => {
            h488
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h489, _, _, _) => {
            h489
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h490, _, _) => {
            h490
        }
    })
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f17(v4, v5, v6, v7) {
        f17(v0, v1, v2, v3)
    } else {
        let a494 = {
            let a491 = v0;
            let a492 = v4;
            let a493 = 1u64;
            nat_rem(a491, a492, a493)
        };
        let a495 = 0u64;
        if if if nat_eq(a494, a495) {
            let a499 = {
                let a496 = v1;
                let a497 = v5;
                let a498 = 1u64;
                nat_rem(a496, a497, a498)
            };
            let a500 = 0u64;
            nat_eq(a499, a500)
        } else {
            false
        } {
            let a504 = {
                let a501 = v2;
                let a502 = v6;
                let a503 = 1u64;
                nat_rem(a501, a502, a503)
            };
            let a505 = 0u64;
            nat_eq(a504, a505)
        } else {
            false
        } {
            let a509 = {
                let a506 = v3;
                let a507 = v7;
                let a508 = 1u64;
                nat_rem(a506, a507, a508)
            };
            let a510 = 0u64;
            nat_eq(a509, a510)
        } else {
            false
        }
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a511 = v0;
    let a512 = v1;
    if nat_eq(a511, a512) {
        let a515 = v4;
        let a516 = {
            let a513 = v2;
            let a514 = v3;
            nat_add(a513, a514)?
        };
        Ok(nat_eq(a515, a516))
    } else {
        let a517 = v2;
        let a518 = v3;
        if nat_eq(a517, a518) {
            let a519 = v4;
            let a520 = v2;
            Ok(nat_eq(a519, a520))
        } else {
            Ok(false)
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a523 = {
            let a521 = v0;
            let a522 = f31(v1);
            nat_add(a521, a522)?
        };
        let a524 = f31(v2);
        nat_add(a523, a524)?
    };
    let v5: u64 = {
        let a525 = f32(v1);
        let a526 = f32(v2);
        nat_add(a525, a526)?
    };
    let a527 = v1;
    let a528 = 4294967295u64;
    if if nat_le(a527, a528) {
        let a529 = v2;
        let a530 = 4294967295u64;
        nat_le(a529, a530)
    } else {
        false
    } {
        let a531 = v5;
        let a532 = v4;
        if nat_le(a531, a532) {
            let a535 = v3;
            let a536 = {
                let a533 = v4;
                let a534 = v5;
                nat_sub(a533, a534)
            };
            Ok(nat_eq(a535, a536))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a537 = v0;
    let a538 = 0u64;
    if if if nat_eq(a537, a538) {
        true
    } else {
        let a539 = v1;
        let a540 = 0u64;
        nat_eq(a539, a540)
    } {
        true
    } else {
        let a541 = v2;
        let a542 = 0u64;
        nat_eq(a541, a542)
    } {
        true
    } else {
        let a543 = v3;
        let a544 = 0u64;
        nat_eq(a543, a544)
    }
}

pub fn f18(v0: u64, v1: u64) -> R<bool> {
    let a545 = v1;
    let a546 = 4294967295u64;
    if nat_le(a545, a546) {
        let a549 = {
            let a547 = f31(v1);
            let a548 = f32(v1);
            nat_add(a547, a548)?
        };
        let a550 = v0;
        Ok(nat_lt(a549, a550))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a553 = match v0.clone() {
        Adt2::C0(_, h551, _, _, _, _, _, _, _, _, _) => {
            h551
        }
    };
    let a554 = match v1.clone() {
        Adt2::C0(_, h552, _, _, _, _, _, _, _, _, _) => {
            h552
        }
    };
    if if if nat_eq(a553, a554) {
        let a557 = match v0.clone() {
            Adt2::C0(_, _, h555, _, _, _, _, _, _, _, _) => {
                h555
            }
        };
        let a558 = match v1.clone() {
            Adt2::C0(_, _, h556, _, _, _, _, _, _, _, _) => {
                h556
            }
        };
        nat_eq(a557, a558)
    } else {
        false
    } {
        let a561 = match v0.clone() {
            Adt2::C0(_, _, _, h559, _, _, _, _, _, _, _) => {
                h559
            }
        };
        let a562 = match v1.clone() {
            Adt2::C0(_, _, _, h560, _, _, _, _, _, _, _) => {
                h560
            }
        };
        nat_eq(a561, a562)
    } else {
        false
    } {
        let a565 = match v0 {
            Adt2::C0(_, _, _, _, h563, _, _, _, _, _, _) => {
                h563
            }
        };
        let a566 = match v1 {
            Adt2::C0(_, _, _, _, h564, _, _, _, _, _, _) => {
                h564
            }
        };
        nat_eq(a565, a566)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a568 = match v0.clone() {
        Adt2::C0(_, h567, _, _, _, _, _, _, _, _, _) => {
            h567
        }
    };
    let a569 = 1u64;
    if if if if nat_eq(a568, a569) {
        true
    } else {
        let a571 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h570, _, _, _, _, _) => {
                h570
            }
        };
        let a572 = 4u64;
        nat_eq(a571, a572)
    } {
        let a574 = match v0.clone() {
            Adt2::C0(_, _, h573, _, _, _, _, _, _, _, _) => {
                h573
            }
        };
        let a575 = 1u64;
        if nat_eq(a574, a575) {
            true
        } else {
            let a580 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h576, _, _, _, _) => {
                    h576
                }
            };
            let a581 = {
                let a578 = match v0.clone() {
                    Adt2::C0(_, h577, _, _, _, _, _, _, _, _, _) => {
                        h577
                    }
                };
                let a579 = 4u64;
                nat_mul(a578, a579)?
            };
            nat_eq(a580, a581)
        }
    } else {
        false
    } {
        let a583 = match v0.clone() {
            Adt2::C0(_, _, _, h582, _, _, _, _, _, _, _) => {
                h582
            }
        };
        let a584 = 1u64;
        if nat_eq(a583, a584) {
            true
        } else {
            let a592 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h585, _, _, _) => {
                    h585
                }
            };
            let a593 = {
                let a590 = {
                    let a588 = match v0.clone() {
                        Adt2::C0(_, h586, _, _, _, _, _, _, _, _, _) => {
                            h586
                        }
                    };
                    let a589 = match v0.clone() {
                        Adt2::C0(_, _, h587, _, _, _, _, _, _, _, _) => {
                            h587
                        }
                    };
                    nat_mul(a588, a589)?
                };
                let a591 = 4u64;
                nat_mul(a590, a591)?
            };
            nat_eq(a592, a593)
        }
    } else {
        false
    } {
        let a595 = match v0.clone() {
            Adt2::C0(_, _, _, _, h594, _, _, _, _, _, _) => {
                h594
            }
        };
        let a596 = 1u64;
        if nat_eq(a595, a596) {
            Ok(true)
        } else {
            let a607 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h597, _, _) => {
                    h597
                }
            };
            let a608 = {
                let a605 = {
                    let a603 = {
                        let a600 = match v0.clone() {
                            Adt2::C0(_, h598, _, _, _, _, _, _, _, _, _) => {
                                h598
                            }
                        };
                        let a601 = match v0.clone() {
                            Adt2::C0(_, _, h599, _, _, _, _, _, _, _, _) => {
                                h599
                            }
                        };
                        nat_mul(a600, a601)?
                    };
                    let a604 = match v0 {
                        Adt2::C0(_, _, _, h602, _, _, _, _, _, _, _) => {
                            h602
                        }
                    };
                    nat_mul(a603, a604)?
                };
                let a606 = 4u64;
                nat_mul(a605, a606)?
            };
            Ok(nat_eq(a607, a608))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f33(v0.clone(), false)?;
    let v6: u64 = f33(v0, true)?;
    let a609 = v6;
    let a610 = v1;
    if if nat_le(a609, a610) {
        let a617 = {
            let a615 = {
                let a611 = v1;
                let a612 = v5;
                nat_add(a611, a612)?
            };
            let a616 = {
                let a613 = v3;
                let a614 = 0u64;
                if nat_eq(a613, a614) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a615, a616)?
        };
        let a618 = v2;
        nat_le(a617, a618)
    } else {
        false
    } {
        if v4 {
            let a623 = {
                let a621 = {
                    let a619 = v1;
                    let a620 = v5;
                    nat_add(a619, a620)?
                };
                let a622 = v6;
                nat_sub(a621, a622)
            };
            let a624 = v2;
            Ok(nat_lt(a623, a624))
        } else {
            let a629 = {
                let a627 = {
                    let a625 = v1;
                    let a626 = v5;
                    nat_add(a625, a626)?
                };
                let a628 = v6;
                nat_sub(a627, a628)
            };
            let a630 = v2;
            Ok(nat_le(a629, a630))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m631 = f34(v0, v1);
    match m631 {
        None => {
            if v5 {
                let m632 = f35(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m632 {
                    Adt9::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt9::C1 => {
                        Ok(Adt6::C1)
                    }
                    Adt9::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt6::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a633 = 4u64;
                let a634 = v28;
                nat_le(a633, a634)
            };
            let a635 = v2;
            let a636 = 0u64;
            if if if if if nat_eq(a635, a636) {
                let a637 = v4;
                let a638 = 0u64;
                nat_eq(a637, a638)
            } else {
                false
            } {
                let a639 = v6;
                let a640 = v14;
                if nat_eq(a639, a640) {
                    let a641 = v7;
                    let a642 = v15;
                    nat_eq(a641, a642)
                } else {
                    false
                }
            } else {
                false
            } {
                let a643 = v8;
                let a644 = v16;
                if nat_eq(a643, a644) {
                    let a645 = v9;
                    let a646 = v17;
                    nat_eq(a645, a646)
                } else {
                    false
                }
            } else {
                false
            } {
                if v29 {
                    true
                } else {
                    if if v5 {
                        let a647 = v3;
                        let a648 = 0u64;
                        nat_eq(a647, a648)
                    } else {
                        false
                    } {
                        f14(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a653 = {
                        let a649 = v6;
                        let a650 = v7;
                        nat_mul(a649, a650)?
                    };
                    let a654 = {
                        let a651 = v8;
                        let a652 = v9;
                        nat_mul(a651, a652)?
                    };
                    nat_mul(a653, a654)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f23(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a656 = v0;
        let a657 = match v1.clone() {
            Adt3::C0(h655, _, _, _, _, _, _, _) => {
                h655
            }
        };
        let a658 = 0u64;
        nat_quot(a656, a657, a658)
    };
    let v3: u64 = {
        let a660 = v2;
        let a661 = match v1.clone() {
            Adt3::C0(_, h659, _, _, _, _, _, _) => {
                h659
            }
        };
        let a662 = 0u64;
        nat_quot(a660, a661, a662)
    };
    f36({
        let a664 = v0;
        let a665 = match v1.clone() {
            Adt3::C0(h663, _, _, _, _, _, _, _) => {
                h663
            }
        };
        let a666 = 0u64;
        nat_rem(a664, a665, a666)
    }, {
        let a668 = v2;
        let a669 = match v1.clone() {
            Adt3::C0(_, h667, _, _, _, _, _, _) => {
                h667
            }
        };
        let a670 = 0u64;
        nat_rem(a668, a669, a670)
    }, {
        let a672 = v3;
        let a673 = match v1.clone() {
            Adt3::C0(_, _, h671, _, _, _, _, _) => {
                h671
            }
        };
        let a674 = 0u64;
        nat_rem(a672, a673, a674)
    }, {
        let a676 = v3;
        let a677 = match v1.clone() {
            Adt3::C0(_, _, h675, _, _, _, _, _) => {
                h675
            }
        };
        let a678 = 0u64;
        nat_quot(a676, a677, a678)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h679, _, _, _) => {
            h679
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h680, _, _) => {
            h680
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h681, _) => {
            h681
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h682) => {
            h682
        }
    })
}

pub fn f24(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a684 = v0;
        let a685 = match v2.clone() {
            Adt3::C0(h683, _, _, _, _, _, _, _) => {
                h683
            }
        };
        let a686 = 0u64;
        nat_quot(a684, a685, a686)
    };
    let v4: u64 = {
        let a688 = v3;
        let a689 = match v2.clone() {
            Adt3::C0(_, h687, _, _, _, _, _, _) => {
                h687
            }
        };
        let a690 = 0u64;
        nat_quot(a688, a689, a690)
    };
    f37({
        let a692 = v0;
        let a693 = match v2.clone() {
            Adt3::C0(h691, _, _, _, _, _, _, _) => {
                h691
            }
        };
        let a694 = 0u64;
        nat_rem(a692, a693, a694)
    }, {
        let a696 = v3;
        let a697 = match v2.clone() {
            Adt3::C0(_, h695, _, _, _, _, _, _) => {
                h695
            }
        };
        let a698 = 0u64;
        nat_rem(a696, a697, a698)
    }, {
        let a700 = v4;
        let a701 = match v2.clone() {
            Adt3::C0(_, _, h699, _, _, _, _, _) => {
                h699
            }
        };
        let a702 = 0u64;
        nat_rem(a700, a701, a702)
    }, {
        let a704 = v4;
        let a705 = match v2 {
            Adt3::C0(_, _, h703, _, _, _, _, _) => {
                h703
            }
        };
        let a706 = 0u64;
        nat_quot(a704, a705, a706)
    }, match v1.clone() {
        Adt3::C0(h707, _, _, _, _, _, _, _) => {
            h707
        }
    }, match v1.clone() {
        Adt3::C0(_, h708, _, _, _, _, _, _) => {
            h708
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h709, _, _, _, _, _) => {
            h709
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h710, _, _, _, _) => {
            h710
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h711, _, _, _) => {
            h711
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h712, _, _) => {
            h712
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h713, _) => {
            h713
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h714) => {
            h714
        }
    })
}

pub fn f25(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a716 = v0;
        let a717 = match v3.clone() {
            Adt3::C0(h715, _, _, _, _, _, _, _) => {
                h715
            }
        };
        let a718 = 0u64;
        nat_quot(a716, a717, a718)
    };
    let v6: u64 = {
        let a720 = v5;
        let a721 = match v3.clone() {
            Adt3::C0(_, h719, _, _, _, _, _, _) => {
                h719
            }
        };
        let a722 = 0u64;
        nat_quot(a720, a721, a722)
    };
    let v7: u64 = {
        let a724 = v0;
        let a725 = match v3.clone() {
            Adt3::C0(h723, _, _, _, _, _, _, _) => {
                h723
            }
        };
        let a726 = 0u64;
        nat_rem(a724, a725, a726)
    };
    let v8: u64 = {
        let a728 = v5;
        let a729 = match v3.clone() {
            Adt3::C0(_, h727, _, _, _, _, _, _) => {
                h727
            }
        };
        let a730 = 0u64;
        nat_rem(a728, a729, a730)
    };
    let v9: u64 = {
        let a732 = v6;
        let a733 = match v3.clone() {
            Adt3::C0(_, _, h731, _, _, _, _, _) => {
                h731
            }
        };
        let a734 = 0u64;
        nat_rem(a732, a733, a734)
    };
    let v10: u64 = {
        let a736 = v6;
        let a737 = match v3.clone() {
            Adt3::C0(_, _, h735, _, _, _, _, _) => {
                h735
            }
        };
        let a738 = 0u64;
        nat_quot(a736, a737, a738)
    };
    let v11: u64 = {
        let a739 = v4;
        let a740 = 0u64;
        if nat_eq(a739, a740) {
            match v1.clone() {
                Adt3::C0(h741, _, _, _, _, _, _, _) => {
                    h741
                }
            }
        } else {
            let a742 = v4;
            let a743 = 1u64;
            if nat_eq(a742, a743) {
                match v1.clone() {
                    Adt3::C0(_, h744, _, _, _, _, _, _) => {
                        h744
                    }
                }
            } else {
                let a745 = v4;
                let a746 = 2u64;
                if nat_eq(a745, a746) {
                    match v1.clone() {
                        Adt3::C0(_, _, h747, _, _, _, _, _) => {
                            h747
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h748, _, _, _, _) => {
                            h748
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a755 = v11;
        let a756 = {
            let a749 = v4;
            let a750 = 0u64;
            if nat_eq(a749, a750) {
                v7
            } else {
                let a751 = v4;
                let a752 = 1u64;
                if nat_eq(a751, a752) {
                    v8
                } else {
                    let a753 = v4;
                    let a754 = 2u64;
                    if nat_eq(a753, a754) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a755, a756)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f36(if if v12 {
        let a757 = v4;
        let a758 = 0u64;
        nat_eq(a757, a758)
    } else {
        false
    } {
        let a759 = v7;
        let a760 = v11;
        nat_sub(a759, a760)
    } else {
        v7
    }, if if v12 {
        let a761 = v4;
        let a762 = 1u64;
        nat_eq(a761, a762)
    } else {
        false
    } {
        let a763 = v8;
        let a764 = v11;
        nat_sub(a763, a764)
    } else {
        v8
    }, if if v12 {
        let a765 = v4;
        let a766 = 2u64;
        nat_eq(a765, a766)
    } else {
        false
    } {
        let a767 = v9;
        let a768 = v11;
        nat_sub(a767, a768)
    } else {
        v9
    }, if if v12 {
        let a769 = v4;
        let a770 = 3u64;
        nat_eq(a769, a770)
    } else {
        false
    } {
        let a771 = v10;
        let a772 = v11;
        nat_sub(a771, a772)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h773, _, _, _) => {
            h773
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h774, _, _) => {
            h774
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h775, _) => {
            h775
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h776) => {
            h776
        }
    })?, f23(v0, v3)?))
}

pub fn f26(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f38(v0, v2.clone(), 0u64);
    let v9: u64 = f38(v0, v2.clone(), 1u64);
    let v10: u64 = f38(v0, v2.clone(), 2u64);
    let v11: u64 = f38(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f39(v1.clone(), f40(v8, v3, match v1.clone() {
            Adt3::C0(h777, _, _, _, _, _, _, _) => {
                h777
            }
        })?, f40(v9, v4, match v1.clone() {
            Adt3::C0(_, h778, _, _, _, _, _, _) => {
                h778
            }
        })?, f40(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h779, _, _, _, _, _) => {
                h779
            }
        })?, f40(v11, v6, match v1 {
            Adt3::C0(_, _, _, h780, _, _, _, _) => {
                h780
            }
        })?)?, f23(v0, v2)?))
    } else {
        if if if if f41(v8, v3, match v1.clone() {
            Adt3::C0(h781, _, _, _, _, _, _, _) => {
                h781
            }
        })? {
            f41(v9, v4, match v1.clone() {
                Adt3::C0(_, h782, _, _, _, _, _, _) => {
                    h782
                }
            })?
        } else {
            false
        } {
            f41(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h783, _, _, _, _, _) => {
                    h783
                }
            })?
        } else {
            false
        } {
            f41(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h784, _, _, _, _) => {
                    h784
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f39(v1, f42(v8, v3)?, f42(v9, v4)?, f42(v10, v5)?, f42(v11, v6)?)?, f23(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f23(v0, v2)?))
        }
    }
}

pub fn f27(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f38(v0, v2.clone(), 0u64);
    let v5: u64 = f38(v0, v2.clone(), 1u64);
    let v6: u64 = f38(v0, v2.clone(), 2u64);
    let v7: u64 = f38(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f39(v1.clone(), f43(v4, v3, match v1 {
        Adt3::C0(h785, _, _, _, _, _, _, _) => {
            h785
        }
    })?, v5, v6, v7)?, f23(v0, v2)?))
}

pub fn f28(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a786 = v0;
    let a787 = v3;
    if nat_lt(a786, a787) {
        Ok(Adt5::C1(false, {
            let a788 = v0;
            let a789 = 4u64;
            nat_mul(a788, a789)?
        }, {
            let a790 = v0;
            let a791 = 4u64;
            nat_mul(a790, a791)?
        }))
    } else {
        let v6: u64 = {
            let a792 = v0;
            let a793 = v3;
            nat_sub(a792, a793)
        };
        let v7: u64 = f23(v6, v1)?;
        let v8: u64 = f44(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f29(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a794 = v0;
    let a795 = v3;
    if nat_lt(a794, a795) {
        Ok(Adt5::C1(true, 0u64, f23(v0, v2)?))
    } else {
        let v4: u64 = {
            let a796 = v0;
            let a797 = v3;
            nat_sub(a796, a797)
        };
        Ok(Adt5::C3(f23(v4, v1.clone())?, f24(v4, v2, v1)?))
    }
}

pub fn f30(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a799 = v1;
        let a800 = match v3.clone() {
            Adt3::C0(h798, _, _, _, _, _, _, _) => {
                h798
            }
        };
        let a801 = 0u64;
        nat_rem(a799, a800, a801)
    };
    let v5: u64 = {
        let a807 = {
            let a803 = v1;
            let a804 = match v3.clone() {
                Adt3::C0(h802, _, _, _, _, _, _, _) => {
                    h802
                }
            };
            let a805 = 0u64;
            nat_quot(a803, a804, a805)
        };
        let a808 = match v3.clone() {
            Adt3::C0(_, h806, _, _, _, _, _, _) => {
                h806
            }
        };
        let a809 = 0u64;
        nat_rem(a807, a808, a809)
    };
    let v6: bool = f45(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f24(v1, v2, v3.clone())?
    } else {
        0u64
    }, f23(v1, v3)?))
}

pub fn f31(v0: u64) -> u64 {
    let a810 = v0;
    let a811 = 2147483648u64;
    if nat_lt(a810, a811) {
        v0
    } else {
        0u64
    }
}

pub fn f32(v0: u64) -> u64 {
    let a812 = v0;
    let a813 = 2147483648u64;
    if nat_lt(a812, a813) {
        0u64
    } else {
        let a814 = 4294967296u64;
        let a815 = v0;
        nat_sub(a814, a815)
    }
}

pub fn f33(v0: Adt3, v1: bool) -> R<u64> {
    f46(v0.clone(), {
        let a817 = match v0.clone() {
            Adt3::C0(h816, _, _, _, _, _, _, _) => {
                h816
            }
        };
        let a818 = 1u64;
        nat_sub(a817, a818)
    }, {
        let a820 = match v0.clone() {
            Adt3::C0(_, h819, _, _, _, _, _, _) => {
                h819
            }
        };
        let a821 = 1u64;
        nat_sub(a820, a821)
    }, {
        let a823 = match v0.clone() {
            Adt3::C0(_, _, h822, _, _, _, _, _) => {
                h822
            }
        };
        let a824 = 1u64;
        nat_sub(a823, a824)
    }, {
        let a826 = match v0 {
            Adt3::C0(_, _, _, h825, _, _, _, _) => {
                h825
            }
        };
        let a827 = 1u64;
        nat_sub(a826, a827)
    }, v1)
}

pub fn f34(v0: u64, v1: u64) -> Option<u64> {
    let a828 = v0;
    let a829 = 2u64;
    if nat_eq(a828, a829) {
        Some(0u64)
    } else {
        let a830 = v0;
        let a831 = 5u64;
        if nat_eq(a830, a831) {
            Some(1u64)
        } else {
            let a832 = v0;
            let a833 = 6u64;
            if nat_eq(a832, a833) {
                Some(2u64)
            } else {
                let a834 = v0;
                let a835 = 7u64;
                if nat_eq(a834, a835) {
                    Some(3u64)
                } else {
                    let a836 = v0;
                    let a837 = 9u64;
                    if nat_eq(a836, a837) {
                        Some(4u64)
                    } else {
                        let a838 = v0;
                        let a839 = 8u64;
                        if nat_eq(a838, a839) {
                            Some(5u64)
                        } else {
                            let a840 = v0;
                            let a841 = 89u64;
                            if nat_eq(a840, a841) {
                                let a842 = v1;
                                let a843 = 2u64;
                                if nat_eq(a842, a843) {
                                    Some(6u64)
                                } else {
                                    let a844 = v1;
                                    let a845 = 0u64;
                                    if nat_eq(a844, a845) {
                                        Some(7u64)
                                    } else {
                                        None::<u64>
                                    }
                                }
                            } else {
                                None::<u64>
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a846 = v0;
    let a847 = 28u64;
    if if if if nat_eq(a846, a847) {
        let a848 = v1;
        let a849 = 0u64;
        nat_eq(a848, a849)
    } else {
        false
    } {
        let a850 = v2;
        let a851 = 0u64;
        nat_eq(a850, a851)
    } else {
        false
    } {
        let a852 = v3;
        let a853 = 0u64;
        nat_eq(a852, a853)
    } else {
        false
    } {
        let a854 = v4;
        let a855 = v8;
        if if if if if if if if nat_eq(a854, a855) {
            let a856 = v5;
            let a857 = v12;
            nat_eq(a856, a857)
        } else {
            false
        } {
            let a858 = v9;
            let a859 = v13;
            nat_eq(a858, a859)
        } else {
            false
        } {
            let a860 = v10;
            let a861 = v14;
            nat_eq(a860, a861)
        } else {
            false
        } {
            let a862 = v11;
            let a863 = v15;
            nat_eq(a862, a863)
        } else {
            false
        } {
            let a864 = 0u64;
            let a865 = v6;
            nat_lt(a864, a865)
        } else {
            false
        } {
            let a866 = 0u64;
            let a867 = v7;
            nat_lt(a866, a867)
        } else {
            false
        } {
            let a871 = {
                let a868 = v10;
                let a869 = v6;
                let a870 = 1u64;
                nat_rem(a868, a869, a870)
            };
            let a872 = 0u64;
            if nat_eq(a871, a872) {
                let a876 = {
                    let a873 = v11;
                    let a874 = v7;
                    let a875 = 1u64;
                    nat_rem(a873, a874, a875)
                };
                let a877 = 0u64;
                nat_eq(a876, a877)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a878 = v10;
                let a879 = v6;
                let a880 = 0u64;
                nat_quot(a878, a879, a880)
            }, {
                let a881 = v11;
                let a882 = v7;
                let a883 = 0u64;
                nat_quot(a881, a882, a883)
            }, {
                let a884 = v5;
                let a885 = v4;
                nat_mul(a884, a885)?
            }, {
                let a886 = v4;
                let a887 = v9;
                nat_mul(a886, a887)?
            }, {
                let a888 = v5;
                let a889 = v9;
                nat_mul(a888, a889)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a902 = {
        let a898 = {
            let a894 = {
                let a890 = v0;
                let a891 = v4;
                nat_mul(a890, a891)?
            };
            let a895 = {
                let a892 = v1;
                let a893 = v5;
                nat_mul(a892, a893)?
            };
            nat_add(a894, a895)?
        };
        let a899 = {
            let a896 = v2;
            let a897 = v6;
            nat_mul(a896, a897)?
        };
        nat_add(a898, a899)?
    };
    let a903 = {
        let a900 = v3;
        let a901 = v7;
        nat_mul(a900, a901)?
    };
    nat_add(a902, a903)
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f36({
        let a904 = v0;
        let a905 = v4;
        let a906 = 0u64;
        nat_rem(a904, a905, a906)
    }, {
        let a907 = v1;
        let a908 = v5;
        let a909 = 0u64;
        nat_rem(a907, a908, a909)
    }, {
        let a910 = v2;
        let a911 = v6;
        let a912 = 0u64;
        nat_rem(a910, a911, a912)
    }, {
        let a913 = v3;
        let a914 = v7;
        let a915 = 0u64;
        nat_rem(a913, a914, a915)
    }, v8, v9, v10, v11)
}

pub fn f38(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a917 = v0;
        let a918 = match v1.clone() {
            Adt3::C0(h916, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a919 = 0u64;
        nat_quot(a917, a918, a919)
    };
    let v4: u64 = {
        let a921 = v3;
        let a922 = match v1.clone() {
            Adt3::C0(_, h920, _, _, _, _, _, _) => {
                h920
            }
        };
        let a923 = 0u64;
        nat_quot(a921, a922, a923)
    };
    let a924 = v2;
    let a925 = 0u64;
    if nat_eq(a924, a925) {
        let a927 = v0;
        let a928 = match v1 {
            Adt3::C0(h926, _, _, _, _, _, _, _) => {
                h926
            }
        };
        let a929 = 0u64;
        nat_rem(a927, a928, a929)
    } else {
        let a930 = v2;
        let a931 = 1u64;
        if nat_eq(a930, a931) {
            let a933 = v3;
            let a934 = match v1 {
                Adt3::C0(_, h932, _, _, _, _, _, _) => {
                    h932
                }
            };
            let a935 = 0u64;
            nat_rem(a933, a934, a935)
        } else {
            let a936 = v2;
            let a937 = 2u64;
            if nat_eq(a936, a937) {
                let a939 = v4;
                let a940 = match v1 {
                    Adt3::C0(_, _, h938, _, _, _, _, _) => {
                        h938
                    }
                };
                let a941 = 0u64;
                nat_rem(a939, a940, a941)
            } else {
                let a943 = v4;
                let a944 = match v1 {
                    Adt3::C0(_, _, h942, _, _, _, _, _) => {
                        h942
                    }
                };
                let a945 = 0u64;
                nat_quot(a943, a944, a945)
            }
        }
    }
}

pub fn f39(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f36(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h946, _, _, _) => {
            h946
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h947, _, _) => {
            h947
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h948, _) => {
            h948
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h949) => {
            h949
        }
    })
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a950 = v1;
    let a951 = 2147483648u64;
    if nat_lt(a950, a951) {
        let a962 = {
            let a960 = {
                let a955 = {
                    let a952 = v0;
                    let a953 = v2;
                    let a954 = 0u64;
                    nat_rem(a952, a953, a954)
                };
                let a956 = v2;
                nat_add(a955, a956)?
            };
            let a961 = {
                let a957 = v1;
                let a958 = v2;
                let a959 = 0u64;
                nat_rem(a957, a958, a959)
            };
            nat_sub(a960, a961)
        };
        let a963 = v2;
        let a964 = 0u64;
        Ok(nat_rem(a962, a963, a964))
    } else {
        let a967 = {
            let a965 = v0;
            let a966 = f32(v1);
            nat_add(a965, a966)?
        };
        let a968 = v2;
        let a969 = 0u64;
        Ok(nat_rem(a967, a968, a969))
    }
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a972 = f31(v1);
    let a973 = {
        let a970 = v0;
        let a971 = f32(v1);
        nat_add(a970, a971)?
    };
    if nat_le(a972, a973) {
        let a974 = f42(v0, v1)?;
        let a975 = v2;
        Ok(nat_lt(a974, a975))
    } else {
        Ok(false)
    }
}

pub fn f42(v0: u64, v1: u64) -> R<u64> {
    let a978 = {
        let a976 = v0;
        let a977 = f32(v1);
        nat_add(a976, a977)?
    };
    let a979 = f31(v1);
    Ok(nat_sub(a978, a979))
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a980 = v0;
    let a981 = v1;
    if nat_lt(a980, a981) {
        let a982 = v1;
        let a983 = v0;
        Ok(nat_sub(a982, a983))
    } else {
        let a986 = v0;
        let a987 = {
            let a984 = v1;
            let a985 = v2;
            nat_add(a984, a985)?
        };
        if nat_lt(a986, a987) {
            let a988 = v0;
            let a989 = v1;
            Ok(nat_sub(a988, a989))
        } else {
            let a996 = {
                let a992 = 2u64;
                let a993 = {
                    let a990 = v2;
                    let a991 = 1u64;
                    nat_sub(a990, a991)
                };
                nat_mul(a992, a993)?
            };
            let a997 = {
                let a994 = v0;
                let a995 = v1;
                nat_sub(a994, a995)
            };
            Ok(nat_sub(a996, a997))
        }
    }
}

pub fn f44(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1000 = {
        let a998 = v2;
        let a999 = f46(v1.clone(), f38(v0, v1.clone(), 0u64), f38(v0, v1.clone(), 1u64), f38(v0, v1.clone(), 2u64), f38(v0, v1.clone(), 3u64), false)?;
        nat_add(a998, a999)?
    };
    let a1001 = f46(v1.clone(), f38(v0, v1.clone(), 0u64), f38(v0, v1.clone(), 1u64), f38(v0, v1.clone(), 2u64), f38(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1000, a1001))
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> bool {
    let a1002 = v0;
    let a1003 = 0u64;
    if nat_eq(a1002, a1003) {
        let a1004 = v2;
        let a1005 = v1;
        nat_le(a1004, a1005)
    } else {
        let a1006 = v0;
        let a1007 = 1u64;
        if nat_eq(a1006, a1007) {
            let a1008 = v2;
            let a1009 = v1;
            nat_lt(a1008, a1009)
        } else {
            let a1010 = v0;
            let a1011 = 2u64;
            if nat_eq(a1010, a1011) {
                let a1012 = v1;
                let a1013 = v2;
                nat_le(a1012, a1013)
            } else {
                let a1014 = v0;
                let a1015 = 3u64;
                if nat_eq(a1014, a1015) {
                    let a1016 = v1;
                    let a1017 = v2;
                    nat_lt(a1016, a1017)
                } else {
                    let a1018 = v1;
                    let a1019 = v2;
                    nat_eq(a1018, a1019)
                }
            }
        }
    }
}

pub fn f46(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1040 = {
        let a1034 = {
            let a1028 = {
                let a1022 = v1;
                let a1023 = if v5 {
                    f32(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1020, _, _, _) => {
                            h1020
                        }
                    })
                } else {
                    f31(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1021, _, _, _) => {
                            h1021
                        }
                    })
                };
                nat_mul(a1022, a1023)?
            };
            let a1029 = {
                let a1026 = v2;
                let a1027 = if v5 {
                    f32(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1024, _, _) => {
                            h1024
                        }
                    })
                } else {
                    f31(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1025, _, _) => {
                            h1025
                        }
                    })
                };
                nat_mul(a1026, a1027)?
            };
            nat_add(a1028, a1029)?
        };
        let a1035 = {
            let a1032 = v3;
            let a1033 = if v5 {
                f32(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1030, _) => {
                        h1030
                    }
                })
            } else {
                f31(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1031, _) => {
                        h1031
                    }
                })
            };
            nat_mul(a1032, a1033)?
        };
        nat_add(a1034, a1035)?
    };
    let a1041 = {
        let a1038 = v4;
        let a1039 = if v5 {
            f32(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1036) => {
                    h1036
                }
            })
        } else {
            f31(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1037) => {
                    h1037
                }
            })
        };
        nat_mul(a1038, a1039)?
    };
    nat_add(a1040, a1041)
}
