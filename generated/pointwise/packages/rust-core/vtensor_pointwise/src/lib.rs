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
    C9(u64, u64),
    C10(u64, bool, Adt3),
    C11(u64, u64),
    C12(bool, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
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
                                            let a26 = v1;
                                            let a27 = 70u64;
                                            if nat_eq(a26, a27) {
                                                f9(v6, v3, v5)?
                                            } else {
                                                let a28 = v1;
                                                let a29 = 75u64;
                                                if if nat_eq(a28, a29) {
                                                    true
                                                } else {
                                                    let a30 = v1;
                                                    let a31 = 76u64;
                                                    nat_eq(a30, a31)
                                                } {
                                                    f10({
                                                        let a32 = v1;
                                                        let a33 = 76u64;
                                                        nat_eq(a32, a33)
                                                    }, v2, v3, v5)?
                                                } else {
                                                    f11(v1, v6, v3, v4, v5)?
                                                }
                                            }
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
            f12(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a34 = v0;
    let a35 = 1u64;
    if if nat_eq(a34, a35) {
        true
    } else {
        let a36 = v0;
        let a37 = 33u64;
        nat_eq(a36, a37)
    } {
        true
    } else {
        let a38 = v0;
        let a39 = 34u64;
        nat_eq(a38, a39)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a42 = match v1.clone() {
        Adt2::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
            h40
        }
    };
    let a43 = match v3.clone() {
        Adt2::C0(h41, _, _, _, _, _, _, _, _, _, _) => {
            h41
        }
    };
    if nat_eq(a42, a43) {
        f13(v0, v1, v2, v3)
    } else {
        if if if if f14(match v1.clone() {
            Adt2::C0(h44, _, _, _, _, _, _, _, _, _, _) => {
                h44
            }
        }) {
            f14(match v3.clone() {
                Adt2::C0(h45, _, _, _, _, _, _, _, _, _, _) => {
                    h45
                }
            })
        } else {
            false
        } {
            let a46 = v0;
            let a47 = 33u64;
            if nat_eq(a46, a47) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a48 = f15(v1.clone(), 1u64)?;
            let a49 = f15(v3.clone(), 1u64)?;
            nat_eq(a48, a49)
        } else {
            false
        } {
            let m51 = f16(match v3.clone() {
                Adt2::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                    h50
                }
            });
            match m51 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f15(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h52) => {
                            h52
                        }
                    }, f17(v1.clone(), 1u64), f17(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h53, _, _, _, _, _, _, _, _, _, _) => {
                            h53
                        }
                    }, match v3 {
                        Adt2::C0(h54, _, _, _, _, _, _, _, _, _, _) => {
                            h54
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f3(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a56 = match v0.clone() {
        Adt2::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
            h55
        }
    };
    let a57 = 0u64;
    if if if if nat_eq(a56, a57) {
        true
    } else {
        let a59 = match v0.clone() {
            Adt2::C0(h58, _, _, _, _, _, _, _, _, _, _) => {
                h58
            }
        };
        let a60 = 1u64;
        nat_eq(a59, a60)
    } {
        true
    } else {
        let a62 = match v0.clone() {
            Adt2::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        };
        let a63 = 25u64;
        if nat_eq(a62, a63) {
            true
        } else {
            let a65 = match v0.clone() {
                Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                    h64
                }
            };
            let a66 = 26u64;
            nat_eq(a65, a66)
        }
    } {
        true
    } else {
        let a68 = match v0.clone() {
            Adt2::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                h67
            }
        };
        let a69 = 30u64;
        nat_eq(a68, a69)
    } {
        let a72 = match v0.clone() {
            Adt2::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        };
        let a73 = match v1.clone() {
            Adt2::C0(h71, _, _, _, _, _, _, _, _, _, _) => {
                h71
            }
        };
        if if nat_eq(a72, a73) {
            f18(match v1.clone() {
                Adt2::C0(_, h74, _, _, _, _, _, _, _, _, _) => {
                    h74
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h75, _, _, _, _, _, _, _, _) => {
                    h75
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h76, _, _, _, _, _, _, _) => {
                    h76
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h77, _, _, _, _, _, _) => {
                    h77
                }
            }, match v0.clone() {
                Adt2::C0(_, h78, _, _, _, _, _, _, _, _, _) => {
                    h78
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h79, _, _, _, _, _, _, _, _) => {
                    h79
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h80, _, _, _, _, _, _, _) => {
                    h80
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h81, _, _, _, _, _, _) => {
                    h81
                }
            })
        } else {
            false
        } {
            let m83 = f16(match v0.clone() {
                Adt2::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                    h82
                }
            });
            match m83 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f15(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h84) => {
                            h84
                        }
                    }, f17(v0, 1u64), f17(v1, 1u64), Adt4::C1))
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
    let m85 = v2;
    match m85 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a86 = v0;
            let a87 = 4u64;
            if if if nat_lt(a86, a87) {
                let a90 = match v1.clone() {
                    Adt2::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                        h88
                    }
                };
                let a91 = match v4.clone() {
                    Adt2::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                        h89
                    }
                };
                if nat_eq(a90, a91) {
                    let a94 = match v1.clone() {
                        Adt2::C0(h92, _, _, _, _, _, _, _, _, _, _) => {
                            h92
                        }
                    };
                    let a95 = match v3.clone() {
                        Adt2::C0(h93, _, _, _, _, _, _, _, _, _, _) => {
                            h93
                        }
                    };
                    nat_eq(a94, a95)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f19(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h96, _, _, _, _, _, _, _, _, _) => {
                        h96
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h98, _, _, _, _, _, _, _, _, _) => {
                        h98
                    }
                })? {
                    f19(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h99, _, _, _, _, _, _, _, _) => {
                            h99
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h100, _, _, _, _, _, _, _, _) => {
                            h100
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h101, _, _, _, _, _, _, _, _) => {
                            h101
                        }
                    })?
                } else {
                    false
                } {
                    f19(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h102, _, _, _, _, _, _, _) => {
                            h102
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h103, _, _, _, _, _, _, _) => {
                            h103
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                            h104
                        }
                    })?
                } else {
                    false
                } {
                    f19(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h105, _, _, _, _, _, _) => {
                            h105
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h106, _, _, _, _, _, _) => {
                            h106
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h107, _, _, _, _, _, _) => {
                            h107
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m109 = f16(match v1.clone() {
                    Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                        h108
                    }
                });
                match m109 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h110, _) => {
                                h110
                            }
                        };
                        let a115 = {
                            let a112 = match v1.clone() {
                                Adt2::C0(_, h111, _, _, _, _, _, _, _, _, _) => {
                                    h111
                                }
                            };
                            let a113 = v6;
                            let a114 = 1u64;
                            nat_rem(a112, a113, a114)
                        };
                        let a116 = 0u64;
                        if if if nat_eq(a115, a116) {
                            let a121 = {
                                let a118 = match v4.clone() {
                                    Adt2::C0(_, h117, _, _, _, _, _, _, _, _, _) => {
                                        h117
                                    }
                                };
                                let a119 = v6;
                                let a120 = 1u64;
                                nat_rem(a118, a119, a120)
                            };
                            let a122 = 0u64;
                            nat_eq(a121, a122)
                        } else {
                            false
                        } {
                            let a127 = {
                                let a124 = match v3.clone() {
                                    Adt2::C0(_, h123, _, _, _, _, _, _, _, _, _) => {
                                        h123
                                    }
                                };
                                let a125 = v6;
                                let a126 = 1u64;
                                nat_rem(a124, a125, a126)
                            };
                            let a128 = 0u64;
                            nat_eq(a127, a128)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f15(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h129) => {
                                    h129
                                }
                            }, f17(v1, v6), f17(v3, v6), Adt4::C2(v0, f17(v4, v6))))
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
    let a131 = match v2.clone() {
        Adt2::C0(h130, _, _, _, _, _, _, _, _, _, _) => {
            h130
        }
    };
    let a132 = 0u64;
    if if nat_eq(a131, a132) {
        let a134 = match v3.clone() {
            Adt2::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
                h133
            }
        };
        let a135 = 0u64;
        nat_eq(a134, a135)
    } else {
        false
    } {
        let a136 = v0;
        let a137 = 61u64;
        if nat_eq(a136, a137) {
            let v4: bool = {
                let a139 = 0u64;
                let a140 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h138, _, _, _, _, _, _, _) => {
                        h138
                    }
                };
                nat_lt(a139, a140)
            };
            if if if if if if f20(match v2.clone() {
                Adt2::C0(_, h141, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            }, match v1.clone() {
                Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h142
                }
            }, match v1.clone() {
                Adt1::C0(_, h143, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h143
                }
            }, match v3.clone() {
                Adt2::C0(_, h144, _, _, _, _, _, _, _, _, _) => {
                    h144
                }
            })? {
                f20(match v2.clone() {
                    Adt2::C0(_, _, h145, _, _, _, _, _, _, _, _) => {
                        h145
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h146, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h146
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h147, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h147
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h148, _, _, _, _, _, _, _, _) => {
                        h148
                    }
                })?
            } else {
                false
            } {
                f20(match v2.clone() {
                    Adt2::C0(_, _, _, h149, _, _, _, _, _, _, _) => {
                        h149
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h150, _, _, _, _, _, _, _, _, _, _, _) => {
                        h150
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h151, _, _, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h152, _, _, _, _, _, _, _) => {
                        h152
                    }
                })?
            } else {
                false
            } {
                f20(match v2.clone() {
                    Adt2::C0(_, _, _, _, h153, _, _, _, _, _, _) => {
                        h153
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h154, _, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h155, _, _, _, _, _, _, _, _) => {
                        h155
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h156, _, _, _, _, _, _) => {
                        h156
                    }
                })?
            } else {
                false
            } {
                let a158 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h157, _, _, _, _, _, _, _) => {
                        h157
                    }
                };
                let a159 = 4294967295u64;
                nat_le(a158, a159)
            } else {
                false
            } {
                if v4 {
                    !f21(match v2.clone() {
                        Adt2::C0(_, h160, _, _, _, _, _, _, _, _, _) => {
                            h160
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h161, _, _, _, _, _, _, _, _) => {
                            h161
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h162, _, _, _, _, _, _, _) => {
                            h162
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h163, _, _, _, _, _, _) => {
                            h163
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h164, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h164
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h165, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h166, _, _, _, _, _, _, _, _, _, _, _) => {
                        h166
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h167, _, _, _, _, _, _, _, _, _) => {
                        h167
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a168 = v0;
            let a169 = 62u64;
            if nat_eq(a168, a169) {
                let a171 = match v1.clone() {
                    Adt1::C0(h170, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h170
                    }
                };
                let a172 = 2147483648u64;
                if if if if if nat_lt(a171, a172) {
                    let a174 = match v1.clone() {
                        Adt1::C0(_, h173, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h173
                        }
                    };
                    let a175 = 2147483648u64;
                    nat_lt(a174, a175)
                } else {
                    false
                } {
                    let a178 = match v1.clone() {
                        Adt1::C0(h176, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h176
                        }
                    };
                    let a179 = match v2.clone() {
                        Adt2::C0(_, h177, _, _, _, _, _, _, _, _, _) => {
                            h177
                        }
                    };
                    nat_lt(a178, a179)
                } else {
                    false
                } {
                    let a182 = match v1.clone() {
                        Adt1::C0(_, h180, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h180
                        }
                    };
                    let a183 = match v2.clone() {
                        Adt2::C0(_, h181, _, _, _, _, _, _, _, _, _) => {
                            h181
                        }
                    };
                    nat_lt(a182, a183)
                } else {
                    false
                } {
                    let a186 = match v2.clone() {
                        Adt2::C0(_, _, h184, _, _, _, _, _, _, _, _) => {
                            h184
                        }
                    };
                    let a187 = match v3.clone() {
                        Adt2::C0(_, _, h185, _, _, _, _, _, _, _, _) => {
                            h185
                        }
                    };
                    if if if nat_eq(a186, a187) {
                        let a190 = match v2.clone() {
                            Adt2::C0(_, _, _, h188, _, _, _, _, _, _, _) => {
                                h188
                            }
                        };
                        let a191 = match v3.clone() {
                            Adt2::C0(_, _, _, h189, _, _, _, _, _, _, _) => {
                                h189
                            }
                        };
                        nat_eq(a190, a191)
                    } else {
                        false
                    } {
                        let a194 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h192, _, _, _, _, _, _) => {
                                h192
                            }
                        };
                        let a195 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h193, _, _, _, _, _, _) => {
                                h193
                            }
                        };
                        nat_eq(a194, a195)
                    } else {
                        false
                    } {
                        let a204 = match v3.clone() {
                            Adt2::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                                h196
                            }
                        };
                        let a205 = {
                            let a202 = {
                                let a199 = match v2.clone() {
                                    Adt2::C0(_, h197, _, _, _, _, _, _, _, _, _) => {
                                        h197
                                    }
                                };
                                let a200 = match v1.clone() {
                                    Adt1::C0(h198, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h198
                                    }
                                };
                                nat_add(a199, a200)?
                            };
                            let a203 = match v1.clone() {
                                Adt1::C0(_, h201, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h201
                                }
                            };
                            nat_add(a202, a203)?
                        };
                        nat_eq(a204, a205)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h206, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h206
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a207 = v0;
                let a208 = 63u64;
                if nat_eq(a207, a208) {
                    if if if if if f22(match v2.clone() {
                        Adt2::C0(_, h209, _, _, _, _, _, _, _, _, _) => {
                            h209
                        }
                    }, match v1.clone() {
                        Adt1::C0(h210, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h210
                        }
                    })? {
                        f22(match v2.clone() {
                            Adt2::C0(_, _, h211, _, _, _, _, _, _, _, _) => {
                                h211
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h212, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h212
                            }
                        })?
                    } else {
                        false
                    } {
                        f22(match v2.clone() {
                            Adt2::C0(_, _, _, h213, _, _, _, _, _, _, _) => {
                                h213
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h214, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h214
                            }
                        })?
                    } else {
                        false
                    } {
                        f22(match v2.clone() {
                            Adt2::C0(_, _, _, _, h215, _, _, _, _, _, _) => {
                                h215
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h216, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h216
                            }
                        })?
                    } else {
                        false
                    } {
                        let a219 = match v2.clone() {
                            Adt2::C0(_, h217, _, _, _, _, _, _, _, _, _) => {
                                h217
                            }
                        };
                        let a220 = match v3.clone() {
                            Adt2::C0(_, h218, _, _, _, _, _, _, _, _, _) => {
                                h218
                            }
                        };
                        if if if nat_eq(a219, a220) {
                            let a223 = match v2.clone() {
                                Adt2::C0(_, _, h221, _, _, _, _, _, _, _, _) => {
                                    h221
                                }
                            };
                            let a224 = match v3.clone() {
                                Adt2::C0(_, _, h222, _, _, _, _, _, _, _, _) => {
                                    h222
                                }
                            };
                            nat_eq(a223, a224)
                        } else {
                            false
                        } {
                            let a227 = match v2.clone() {
                                Adt2::C0(_, _, _, h225, _, _, _, _, _, _, _) => {
                                    h225
                                }
                            };
                            let a228 = match v3.clone() {
                                Adt2::C0(_, _, _, h226, _, _, _, _, _, _, _) => {
                                    h226
                                }
                            };
                            nat_eq(a227, a228)
                        } else {
                            false
                        } {
                            let a231 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h229, _, _, _, _, _, _) => {
                                    h229
                                }
                            };
                            let a232 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h230, _, _, _, _, _, _) => {
                                    h230
                                }
                            };
                            nat_eq(a231, a232)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h233, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h233
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h234, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h234
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h235, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h235
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h236, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h236
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
    let m237 = v3;
    match m237 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a238 = v0;
                let a239 = 4u64;
                nat_eq(a238, a239)
            };
            let v7: bool = {
                let a241 = 0u64;
                let a242 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h240, _, _, _, _, _, _, _, _, _, _, _) => {
                        h240
                    }
                };
                nat_lt(a241, a242)
            };
            let v8: u64 = f15(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h243, _, _, _, _, _, _, _, _, _) => {
                    h243
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h244, _, _, _, _, _, _, _, _) => {
                    h244
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h245, _, _, _, _, _, _, _) => {
                    h245
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h246, _, _, _, _, _, _) => {
                    h246
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h247, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h247
                }
            }, match v1.clone() {
                Adt1::C0(_, h248, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h248
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h249, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h249
                }
            });
            if if if if if if if if if if if if v6 {
                let a251 = match v2.clone() {
                    Adt2::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                        h250
                    }
                };
                let a252 = 0u64;
                if nat_eq(a251, a252) {
                    let a254 = match v5.clone() {
                        Adt2::C0(h253, _, _, _, _, _, _, _, _, _, _) => {
                            h253
                        }
                    };
                    let a255 = 0u64;
                    nat_eq(a254, a255)
                } else {
                    false
                }
            } else {
                let a257 = match v2.clone() {
                    Adt2::C0(h256, _, _, _, _, _, _, _, _, _, _) => {
                        h256
                    }
                };
                let a258 = 0u64;
                if if nat_eq(a257, a258) {
                    true
                } else {
                    let a260 = match v2.clone() {
                        Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                            h259
                        }
                    };
                    let a261 = 26u64;
                    nat_eq(a260, a261)
                } {
                    let a263 = match v5.clone() {
                        Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                            h262
                        }
                    };
                    let a264 = 0u64;
                    if nat_eq(a263, a264) {
                        true
                    } else {
                        let a266 = match v5.clone() {
                            Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                                h265
                            }
                        };
                        let a267 = 26u64;
                        nat_eq(a266, a267)
                    }
                } else {
                    false
                }
            } {
                let a270 = match v2.clone() {
                    Adt2::C0(h268, _, _, _, _, _, _, _, _, _, _) => {
                        h268
                    }
                };
                let a271 = match v4.clone() {
                    Adt2::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                        h269
                    }
                };
                nat_eq(a270, a271)
            } else {
                false
            } {
                f23(v2.clone(), v4.clone())
            } else {
                false
            } {
                f24(v2.clone())?
            } else {
                false
            } {
                f24(v4.clone())?
            } else {
                false
            } {
                let a273 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h272, _, _, _, _, _) => {
                        h272
                    }
                };
                let a274 = 4u64;
                nat_eq(a273, a274)
            } else {
                false
            } {
                let a275 = v8;
                let a276 = f15(v2.clone(), 1u64)?;
                nat_le(a275, a276)
            } else {
                false
            } {
                let a278 = match v1.clone() {
                    Adt1::C0(_, _, _, h277, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h277
                    }
                };
                let a279 = 1073741824u64;
                nat_lt(a278, a279)
            } else {
                false
            } {
                let a281 = match v1.clone() {
                    Adt1::C0(h280, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h280
                    }
                };
                let a282 = 4294967295u64;
                if if if if nat_le(a281, a282) {
                    let a284 = match v1.clone() {
                        Adt1::C0(_, h283, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h283
                        }
                    };
                    let a285 = 4294967295u64;
                    nat_le(a284, a285)
                } else {
                    false
                } {
                    let a287 = match v1.clone() {
                        Adt1::C0(_, _, h286, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h286
                        }
                    };
                    let a288 = 4294967295u64;
                    nat_le(a287, a288)
                } else {
                    false
                } {
                    let a290 = match v1.clone() {
                        Adt1::C0(_, _, _, h289, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h289
                        }
                    };
                    let a291 = 4294967295u64;
                    nat_le(a290, a291)
                } else {
                    false
                } {
                    let a293 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h292, _, _, _, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    let a294 = 4294967295u64;
                    nat_le(a293, a294)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a297 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h295) => {
                            h295
                        }
                    };
                    let a298 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h296) => {
                            h296
                        }
                    };
                    nat_eq(a297, a298)
                } else {
                    true
                }
            } else {
                false
            } {
                f25(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h299, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h299
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h300, _) => {
                        h300
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f15(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a301 = v10;
                    let a302 = v8;
                    nat_add(a301, a302)?
                }, 4u64, f17(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h303, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h303
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a305 = match v0.clone() {
        Adt2::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
            h304
        }
    };
    let a306 = 0u64;
    if if if nat_eq(a305, a306) {
        let a308 = match v1.clone() {
            Adt2::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
                h307
            }
        };
        let a309 = 0u64;
        nat_eq(a308, a309)
    } else {
        false
    } {
        f18(match v0.clone() {
            Adt2::C0(_, h310, _, _, _, _, _, _, _, _, _) => {
                h310
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h311, _, _, _, _, _, _, _, _) => {
                h311
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h312, _, _, _, _, _, _, _) => {
                h312
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h313, _, _, _, _, _, _) => {
                h313
            }
        }, match v1.clone() {
            Adt2::C0(_, h314, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h315, _, _, _, _, _, _, _, _) => {
                h315
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h316, _, _, _, _, _, _, _) => {
                h316
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h317, _, _, _, _, _, _) => {
                h317
            }
        })
    } else {
        false
    } {
        let v2: u64 = f15(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a318 = v2;
            let a319 = f15(v0.clone(), 1u64)?;
            nat_add(a318, a319)?
        }, 4u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a321 = match v1.clone() {
        Adt2::C0(h320, _, _, _, _, _, _, _, _, _, _) => {
            h320
        }
    };
    let a322 = 0u64;
    if if if nat_eq(a321, a322) {
        true
    } else {
        let a324 = match v1.clone() {
            Adt2::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a325 = 1u64;
        nat_eq(a324, a325)
    } {
        true
    } else {
        let a327 = match v1.clone() {
            Adt2::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
                h326
            }
        };
        let a328 = 30u64;
        nat_eq(a327, a328)
    } {
        let a331 = match v1.clone() {
            Adt2::C0(h329, _, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        };
        let a332 = match v2.clone() {
            Adt2::C0(h330, _, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        if if if if if nat_eq(a331, a332) {
            let a335 = match v1.clone() {
                Adt2::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                    h333
                }
            };
            let a336 = match v2.clone() {
                Adt2::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                    h334
                }
            };
            nat_eq(a335, a336)
        } else {
            false
        } {
            let a339 = match v1.clone() {
                Adt2::C0(_, _, _, h337, _, _, _, _, _, _, _) => {
                    h337
                }
            };
            let a340 = match v2.clone() {
                Adt2::C0(_, _, _, h338, _, _, _, _, _, _, _) => {
                    h338
                }
            };
            nat_eq(a339, a340)
        } else {
            false
        } {
            let a343 = match v1.clone() {
                Adt2::C0(_, _, _, _, h341, _, _, _, _, _, _) => {
                    h341
                }
            };
            let a344 = match v2.clone() {
                Adt2::C0(_, _, _, _, h342, _, _, _, _, _, _) => {
                    h342
                }
            };
            nat_eq(a343, a344)
        } else {
            false
        } {
            let a345 = v0;
            let a346 = 4u64;
            if nat_eq(a345, a346) {
                let a348 = match v1.clone() {
                    Adt2::C0(_, _, h347, _, _, _, _, _, _, _, _) => {
                        h347
                    }
                };
                let a349 = 1u64;
                if nat_eq(a348, a349) {
                    let a352 = match v1.clone() {
                        Adt2::C0(_, h350, _, _, _, _, _, _, _, _, _) => {
                            h350
                        }
                    };
                    let a353 = match v2.clone() {
                        Adt2::C0(_, _, h351, _, _, _, _, _, _, _, _) => {
                            h351
                        }
                    };
                    nat_eq(a352, a353)
                } else {
                    false
                }
            } else {
                let a354 = v0;
                let a355 = 4u64;
                if if nat_lt(a354, a355) {
                    let a358 = match v1.clone() {
                        Adt2::C0(_, h356, _, _, _, _, _, _, _, _, _) => {
                            h356
                        }
                    };
                    let a359 = match v1.clone() {
                        Adt2::C0(_, _, h357, _, _, _, _, _, _, _, _) => {
                            h357
                        }
                    };
                    nat_eq(a358, a359)
                } else {
                    false
                } {
                    let a362 = match v1.clone() {
                        Adt2::C0(_, _, h360, _, _, _, _, _, _, _, _) => {
                            h360
                        }
                    };
                    let a363 = match v2.clone() {
                        Adt2::C0(_, _, h361, _, _, _, _, _, _, _, _) => {
                            h361
                        }
                    };
                    nat_eq(a362, a363)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m365 = f16(match v1.clone() {
                Adt2::C0(h364, _, _, _, _, _, _, _, _, _, _) => {
                    h364
                }
            });
            match m365 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f15(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h366) => {
                            h366
                        }
                    }, f17(v1, 1u64), f17(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f9(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f26(match v1.clone() {
        Adt2::C0(h367, _, _, _, _, _, _, _, _, _, _) => {
            h367
        }
    }, v0) {
        let a370 = match v1.clone() {
            Adt2::C0(h368, _, _, _, _, _, _, _, _, _, _) => {
                h368
            }
        };
        let a371 = match v2.clone() {
            Adt2::C0(h369, _, _, _, _, _, _, _, _, _, _) => {
                h369
            }
        };
        nat_eq(a370, a371)
    } else {
        false
    } {
        let a374 = match v1.clone() {
            Adt2::C0(_, h372, _, _, _, _, _, _, _, _, _) => {
                h372
            }
        };
        let a375 = match v2.clone() {
            Adt2::C0(_, h373, _, _, _, _, _, _, _, _, _) => {
                h373
            }
        };
        if if if nat_eq(a374, a375) {
            let a378 = match v1.clone() {
                Adt2::C0(_, _, h376, _, _, _, _, _, _, _, _) => {
                    h376
                }
            };
            let a379 = match v2.clone() {
                Adt2::C0(_, _, h377, _, _, _, _, _, _, _, _) => {
                    h377
                }
            };
            nat_eq(a378, a379)
        } else {
            false
        } {
            let a382 = match v1.clone() {
                Adt2::C0(_, _, _, h380, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            let a383 = match v2.clone() {
                Adt2::C0(_, _, _, h381, _, _, _, _, _, _, _) => {
                    h381
                }
            };
            nat_eq(a382, a383)
        } else {
            false
        } {
            let a386 = match v1.clone() {
                Adt2::C0(_, _, _, _, h384, _, _, _, _, _, _) => {
                    h384
                }
            };
            let a387 = match v2.clone() {
                Adt2::C0(_, _, _, _, h385, _, _, _, _, _, _) => {
                    h385
                }
            };
            nat_eq(a386, a387)
        } else {
            false
        }
    } else {
        false
    } {
        let m389 = f16(match v1.clone() {
            Adt2::C0(h388, _, _, _, _, _, _, _, _, _, _) => {
                h388
            }
        });
        match m389 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f15(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h390) => {
                        h390
                    }
                }, f17(v1.clone(), 1u64), f17(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h391, _, _, _, _, _, _, _, _, _, _) => {
                        h391
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f10(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h392, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h392
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h393, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h393
            }
        }
    };
    let a394 = 0u64;
    let a395 = v4;
    if if nat_lt(a394, a395) {
        let a396 = v4;
        let a397 = 2147483648u64;
        nat_lt(a396, a397)
    } else {
        false
    } {
        let v5: Adt2 = if v0 {
            v3.clone()
        } else {
            v2.clone()
        };
        let v6: Adt2 = if v0 {
            v2.clone()
        } else {
            v3.clone()
        };
        let v7: u64 = f27(match v5.clone() {
            Adt2::C0(_, _, h398, _, _, _, _, _, _, _, _) => {
                h398
            }
        }, v4)?;
        let v8: u64 = f27(match v5.clone() {
            Adt2::C0(_, _, _, h399, _, _, _, _, _, _, _) => {
                h399
            }
        }, v4)?;
        let a401 = match v5.clone() {
            Adt2::C0(h400, _, _, _, _, _, _, _, _, _, _) => {
                h400
            }
        };
        let a402 = 0u64;
        if if if if if nat_eq(a401, a402) {
            let a404 = match v6.clone() {
                Adt2::C0(h403, _, _, _, _, _, _, _, _, _, _) => {
                    h403
                }
            };
            let a405 = 0u64;
            nat_eq(a404, a405)
        } else {
            false
        } {
            let a407 = match v5.clone() {
                Adt2::C0(_, _, _, _, h406, _, _, _, _, _, _) => {
                    h406
                }
            };
            let a408 = 1u64;
            nat_eq(a407, a408)
        } else {
            false
        } {
            let a411 = match v5.clone() {
                Adt2::C0(_, h409, _, _, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            let a412 = match v6.clone() {
                Adt2::C0(_, h410, _, _, _, _, _, _, _, _, _) => {
                    h410
                }
            };
            nat_eq(a411, a412)
        } else {
            false
        } {
            if v0 {
                let a414 = match v5.clone() {
                    Adt2::C0(_, _, h413, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                };
                let a415 = 2147483648u64;
                if if if nat_lt(a414, a415) {
                    let a417 = match v5.clone() {
                        Adt2::C0(_, _, _, h416, _, _, _, _, _, _, _) => {
                            h416
                        }
                    };
                    let a418 = 2147483648u64;
                    nat_lt(a417, a418)
                } else {
                    false
                } {
                    let a421 = {
                        let a419 = v7;
                        let a420 = v8;
                        nat_mul(a419, a420)?
                    };
                    let a422 = 2147483648u64;
                    nat_le(a421, a422)
                } else {
                    false
                } {
                    f28(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a424 = match v1.clone() {
                    Adt1::C0(h423, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h423
                    }
                };
                let a425 = v7;
                if if if if nat_eq(a424, a425) {
                    let a427 = match v1 {
                        Adt1::C0(_, h426, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h426
                        }
                    };
                    let a428 = v8;
                    nat_eq(a427, a428)
                } else {
                    false
                } {
                    f29(v7, v8, v4)?
                } else {
                    false
                } {
                    let a430 = match v6.clone() {
                        Adt2::C0(_, _, h429, _, _, _, _, _, _, _, _) => {
                            h429
                        }
                    };
                    let a431 = v4;
                    if nat_eq(a430, a431) {
                        let a433 = match v6.clone() {
                            Adt2::C0(_, _, _, h432, _, _, _, _, _, _, _) => {
                                h432
                            }
                        };
                        let a434 = v4;
                        nat_eq(a433, a434)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a438 = match v6 {
                        Adt2::C0(_, _, _, _, h435, _, _, _, _, _, _) => {
                            h435
                        }
                    };
                    let a439 = {
                        let a436 = v7;
                        let a437 = v8;
                        nat_mul(a436, a437)?
                    };
                    nat_eq(a438, a439)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m440 = f30(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m440 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f17(v2.clone(), 1u64), f17(v4, 1u64), Adt4::C10(v5, v6, {
                let m441 = v3;
                match m441 {
                    None => {
                        f17(v2, 1u64)
                    }
                    Some(v12) => {
                        f17(v12, 1u64)
                    }
                }
            })))
        }
        Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
            Ok(Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22))
        }
        Adt6::C4(v23, v24, v25, v26, v27) => {
            Ok(Adt6::C4(v23, v24, v25, v26, v27))
        }
    }
}

pub fn f12(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m442 = v3;
    match m442 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f31(v0, v1)?, f31(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f32(v0, v1, v2.clone())?, f31(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f33(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f34(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f35(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f34(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f36(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f37(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f38(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f31(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f39(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f31(v0, v1)?, f31(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f40(v0, v1, v2, v28, v29, v30)
        }
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a445 = match v1.clone() {
        Adt2::C0(h443, _, _, _, _, _, _, _, _, _, _) => {
            h443
        }
    };
    let a446 = match v3.clone() {
        Adt2::C0(h444, _, _, _, _, _, _, _, _, _, _) => {
            h444
        }
    };
    if if nat_eq(a445, a446) {
        let a447 = v0;
        let a448 = 33u64;
        if nat_eq(a447, a448) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m450 = f16(match v1.clone() {
            Adt2::C0(h449, _, _, _, _, _, _, _, _, _, _) => {
                h449
            }
        });
        match m450 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h451, _) => {
                        h451
                    }
                };
                let v6: u64 = f15(v1.clone(), v5)?;
                let a456 = {
                    let a453 = match v1.clone() {
                        Adt2::C0(_, h452, _, _, _, _, _, _, _, _, _) => {
                            h452
                        }
                    };
                    let a454 = v5;
                    let a455 = 1u64;
                    nat_rem(a453, a454, a455)
                };
                let a457 = 0u64;
                if if if nat_eq(a456, a457) {
                    let a462 = {
                        let a459 = match v3.clone() {
                            Adt2::C0(_, h458, _, _, _, _, _, _, _, _, _) => {
                                h458
                            }
                        };
                        let a460 = v5;
                        let a461 = 1u64;
                        nat_rem(a459, a460, a461)
                    };
                    let a463 = 0u64;
                    nat_eq(a462, a463)
                } else {
                    false
                } {
                    let a464 = v6;
                    let a465 = f15(v3.clone(), v5)?;
                    nat_eq(a464, a465)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h466) => {
                            h466
                        }
                    }, f17(v1, v5), f17(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a467 = v0;
    let a468 = 0u64;
    if if nat_eq(a467, a468) {
        true
    } else {
        let a469 = v0;
        let a470 = 1u64;
        nat_eq(a469, a470)
    } {
        true
    } else {
        let a471 = v0;
        let a472 = 30u64;
        nat_eq(a471, a472)
    }
}

pub fn f15(v0: Adt2, v1: u64) -> R<u64> {
    let a484 = {
        let a478 = {
            let a474 = match v0.clone() {
                Adt2::C0(_, h473, _, _, _, _, _, _, _, _, _) => {
                    h473
                }
            };
            let a475 = v1;
            let a476 = 0u64;
            nat_quot(a474, a475, a476)
        };
        let a479 = match v0.clone() {
            Adt2::C0(_, _, h477, _, _, _, _, _, _, _, _) => {
                h477
            }
        };
        nat_mul(a478, a479)?
    };
    let a485 = {
        let a482 = match v0.clone() {
            Adt2::C0(_, _, _, h480, _, _, _, _, _, _, _) => {
                h480
            }
        };
        let a483 = match v0 {
            Adt2::C0(_, _, _, _, h481, _, _, _, _, _, _) => {
                h481
            }
        };
        nat_mul(a482, a483)?
    };
    nat_mul(a484, a485)
}

pub fn f16(v0: u64) -> Adt7 {
    let a486 = v0;
    let a487 = 0u64;
    if nat_eq(a486, a487) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a488 = v0;
        let a489 = 1u64;
        if nat_eq(a488, a489) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a490 = v0;
            let a491 = 2u64;
            if nat_eq(a490, a491) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a492 = v0;
                let a493 = 3u64;
                if nat_eq(a492, a493) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a494 = v0;
                    let a495 = 6u64;
                    if nat_eq(a494, a495) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a496 = v0;
                        let a497 = 7u64;
                        if nat_eq(a496, a497) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a498 = v0;
                            let a499 = 8u64;
                            if nat_eq(a498, a499) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a500 = v0;
                                let a501 = 9u64;
                                if nat_eq(a500, a501) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a502 = v0;
                                    let a503 = 10u64;
                                    if nat_eq(a502, a503) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a504 = v0;
                                        let a505 = 11u64;
                                        if nat_eq(a504, a505) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a506 = v0;
                                            let a507 = 12u64;
                                            if nat_eq(a506, a507) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a508 = v0;
                                                let a509 = 13u64;
                                                if nat_eq(a508, a509) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a510 = v0;
                                                    let a511 = 14u64;
                                                    if nat_eq(a510, a511) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a512 = v0;
                                                        let a513 = 15u64;
                                                        if nat_eq(a512, a513) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a514 = v0;
                                                            let a515 = 16u64;
                                                            if nat_eq(a514, a515) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a516 = v0;
                                                                let a517 = 17u64;
                                                                if nat_eq(a516, a517) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a518 = v0;
                                                                    let a519 = 18u64;
                                                                    if nat_eq(a518, a519) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a520 = v0;
                                                                        let a521 = 19u64;
                                                                        if nat_eq(a520, a521) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a522 = v0;
                                                                            let a523 = 20u64;
                                                                            if nat_eq(a522, a523) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a524 = v0;
                                                                                let a525 = 21u64;
                                                                                if nat_eq(a524, a525) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a526 = v0;
                                                                                    let a527 = 22u64;
                                                                                    if nat_eq(a526, a527) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a528 = v0;
                                                                                        let a529 = 23u64;
                                                                                        if nat_eq(a528, a529) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a530 = v0;
                                                                                            let a531 = 24u64;
                                                                                            if nat_eq(a530, a531) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a532 = v0;
                                                                                                let a533 = 25u64;
                                                                                                if nat_eq(a532, a533) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a534 = v0;
                                                                                                    let a535 = 26u64;
                                                                                                    if nat_eq(a534, a535) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a536 = v0;
                                                                                                        let a537 = 27u64;
                                                                                                        if nat_eq(a536, a537) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a538 = v0;
                                                                                                            let a539 = 28u64;
                                                                                                            if nat_eq(a538, a539) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a540 = v0;
                                                                                                                let a541 = 29u64;
                                                                                                                if nat_eq(a540, a541) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a542 = v0;
                                                                                                                    let a543 = 30u64;
                                                                                                                    if nat_eq(a542, a543) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a544 = v0;
                                                                                                                        let a545 = 34u64;
                                                                                                                        if nat_eq(a544, a545) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a546 = v0;
                                                                                                                            let a547 = 35u64;
                                                                                                                            if nat_eq(a546, a547) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a548 = v0;
                                                                                                                                let a549 = 39u64;
                                                                                                                                if nat_eq(a548, a549) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a550 = v0;
                                                                                                                                    let a551 = 40u64;
                                                                                                                                    if nat_eq(a550, a551) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a552 = v0;
                                                                                                                                        let a553 = 41u64;
                                                                                                                                        if nat_eq(a552, a553) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a554 = v0;
                                                                                                                                            let a555 = 42u64;
                                                                                                                                            if nat_eq(a554, a555) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a556 = v0;
                                                                                                                                                let a557 = 43u64;
                                                                                                                                                if nat_eq(a556, a557) {
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

pub fn f17(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a559 = match v0.clone() {
            Adt2::C0(_, h558, _, _, _, _, _, _, _, _, _) => {
                h558
            }
        };
        let a560 = v1;
        let a561 = 0u64;
        nat_quot(a559, a560, a561)
    }, match v0.clone() {
        Adt2::C0(_, _, h562, _, _, _, _, _, _, _, _) => {
            h562
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h563, _, _, _, _, _, _, _) => {
            h563
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h564, _, _, _, _, _, _) => {
            h564
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h565, _, _, _, _, _) => {
            h565
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h566, _, _, _, _) => {
            h566
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h567, _, _, _) => {
            h567
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h568, _, _) => {
            h568
        }
    })
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f21(v4, v5, v6, v7) {
        f21(v0, v1, v2, v3)
    } else {
        let a572 = {
            let a569 = v0;
            let a570 = v4;
            let a571 = 1u64;
            nat_rem(a569, a570, a571)
        };
        let a573 = 0u64;
        if if if nat_eq(a572, a573) {
            let a577 = {
                let a574 = v1;
                let a575 = v5;
                let a576 = 1u64;
                nat_rem(a574, a575, a576)
            };
            let a578 = 0u64;
            nat_eq(a577, a578)
        } else {
            false
        } {
            let a582 = {
                let a579 = v2;
                let a580 = v6;
                let a581 = 1u64;
                nat_rem(a579, a580, a581)
            };
            let a583 = 0u64;
            nat_eq(a582, a583)
        } else {
            false
        } {
            let a587 = {
                let a584 = v3;
                let a585 = v7;
                let a586 = 1u64;
                nat_rem(a584, a585, a586)
            };
            let a588 = 0u64;
            nat_eq(a587, a588)
        } else {
            false
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a589 = v0;
    let a590 = v1;
    if nat_eq(a589, a590) {
        let a593 = v4;
        let a594 = {
            let a591 = v2;
            let a592 = v3;
            nat_add(a591, a592)?
        };
        Ok(nat_eq(a593, a594))
    } else {
        let a595 = v2;
        let a596 = v3;
        if nat_eq(a595, a596) {
            let a597 = v4;
            let a598 = v2;
            Ok(nat_eq(a597, a598))
        } else {
            Ok(false)
        }
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a601 = {
            let a599 = v0;
            let a600 = f41(v1);
            nat_add(a599, a600)?
        };
        let a602 = f41(v2);
        nat_add(a601, a602)?
    };
    let v5: u64 = {
        let a603 = f42(v1);
        let a604 = f42(v2);
        nat_add(a603, a604)?
    };
    let a605 = v1;
    let a606 = 4294967295u64;
    if if nat_le(a605, a606) {
        let a607 = v2;
        let a608 = 4294967295u64;
        nat_le(a607, a608)
    } else {
        false
    } {
        let a609 = v5;
        let a610 = v4;
        if nat_le(a609, a610) {
            let a613 = v3;
            let a614 = {
                let a611 = v4;
                let a612 = v5;
                nat_sub(a611, a612)
            };
            Ok(nat_eq(a613, a614))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a615 = v0;
    let a616 = 0u64;
    if if if nat_eq(a615, a616) {
        true
    } else {
        let a617 = v1;
        let a618 = 0u64;
        nat_eq(a617, a618)
    } {
        true
    } else {
        let a619 = v2;
        let a620 = 0u64;
        nat_eq(a619, a620)
    } {
        true
    } else {
        let a621 = v3;
        let a622 = 0u64;
        nat_eq(a621, a622)
    }
}

pub fn f22(v0: u64, v1: u64) -> R<bool> {
    let a623 = v1;
    let a624 = 4294967295u64;
    if nat_le(a623, a624) {
        let a627 = {
            let a625 = f41(v1);
            let a626 = f42(v1);
            nat_add(a625, a626)?
        };
        let a628 = v0;
        Ok(nat_lt(a627, a628))
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> bool {
    let a631 = match v0.clone() {
        Adt2::C0(_, h629, _, _, _, _, _, _, _, _, _) => {
            h629
        }
    };
    let a632 = match v1.clone() {
        Adt2::C0(_, h630, _, _, _, _, _, _, _, _, _) => {
            h630
        }
    };
    if if if nat_eq(a631, a632) {
        let a635 = match v0.clone() {
            Adt2::C0(_, _, h633, _, _, _, _, _, _, _, _) => {
                h633
            }
        };
        let a636 = match v1.clone() {
            Adt2::C0(_, _, h634, _, _, _, _, _, _, _, _) => {
                h634
            }
        };
        nat_eq(a635, a636)
    } else {
        false
    } {
        let a639 = match v0.clone() {
            Adt2::C0(_, _, _, h637, _, _, _, _, _, _, _) => {
                h637
            }
        };
        let a640 = match v1.clone() {
            Adt2::C0(_, _, _, h638, _, _, _, _, _, _, _) => {
                h638
            }
        };
        nat_eq(a639, a640)
    } else {
        false
    } {
        let a643 = match v0 {
            Adt2::C0(_, _, _, _, h641, _, _, _, _, _, _) => {
                h641
            }
        };
        let a644 = match v1 {
            Adt2::C0(_, _, _, _, h642, _, _, _, _, _, _) => {
                h642
            }
        };
        nat_eq(a643, a644)
    } else {
        false
    }
}

pub fn f24(v0: Adt2) -> R<bool> {
    let a646 = match v0.clone() {
        Adt2::C0(_, h645, _, _, _, _, _, _, _, _, _) => {
            h645
        }
    };
    let a647 = 1u64;
    if if if if nat_eq(a646, a647) {
        true
    } else {
        let a649 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h648, _, _, _, _, _) => {
                h648
            }
        };
        let a650 = 4u64;
        nat_eq(a649, a650)
    } {
        let a652 = match v0.clone() {
            Adt2::C0(_, _, h651, _, _, _, _, _, _, _, _) => {
                h651
            }
        };
        let a653 = 1u64;
        if nat_eq(a652, a653) {
            true
        } else {
            let a658 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h654, _, _, _, _) => {
                    h654
                }
            };
            let a659 = {
                let a656 = match v0.clone() {
                    Adt2::C0(_, h655, _, _, _, _, _, _, _, _, _) => {
                        h655
                    }
                };
                let a657 = 4u64;
                nat_mul(a656, a657)?
            };
            nat_eq(a658, a659)
        }
    } else {
        false
    } {
        let a661 = match v0.clone() {
            Adt2::C0(_, _, _, h660, _, _, _, _, _, _, _) => {
                h660
            }
        };
        let a662 = 1u64;
        if nat_eq(a661, a662) {
            true
        } else {
            let a670 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h663, _, _, _) => {
                    h663
                }
            };
            let a671 = {
                let a668 = {
                    let a666 = match v0.clone() {
                        Adt2::C0(_, h664, _, _, _, _, _, _, _, _, _) => {
                            h664
                        }
                    };
                    let a667 = match v0.clone() {
                        Adt2::C0(_, _, h665, _, _, _, _, _, _, _, _) => {
                            h665
                        }
                    };
                    nat_mul(a666, a667)?
                };
                let a669 = 4u64;
                nat_mul(a668, a669)?
            };
            nat_eq(a670, a671)
        }
    } else {
        false
    } {
        let a673 = match v0.clone() {
            Adt2::C0(_, _, _, _, h672, _, _, _, _, _, _) => {
                h672
            }
        };
        let a674 = 1u64;
        if nat_eq(a673, a674) {
            Ok(true)
        } else {
            let a685 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h675, _, _) => {
                    h675
                }
            };
            let a686 = {
                let a683 = {
                    let a681 = {
                        let a678 = match v0.clone() {
                            Adt2::C0(_, h676, _, _, _, _, _, _, _, _, _) => {
                                h676
                            }
                        };
                        let a679 = match v0.clone() {
                            Adt2::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                                h677
                            }
                        };
                        nat_mul(a678, a679)?
                    };
                    let a682 = match v0 {
                        Adt2::C0(_, _, _, h680, _, _, _, _, _, _, _) => {
                            h680
                        }
                    };
                    nat_mul(a681, a682)?
                };
                let a684 = 4u64;
                nat_mul(a683, a684)?
            };
            Ok(nat_eq(a685, a686))
        }
    } else {
        Ok(false)
    }
}

pub fn f25(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f43(v0.clone(), false)?;
    let v6: u64 = f43(v0, true)?;
    let a687 = v6;
    let a688 = v1;
    if if nat_le(a687, a688) {
        let a695 = {
            let a693 = {
                let a689 = v1;
                let a690 = v5;
                nat_add(a689, a690)?
            };
            let a694 = {
                let a691 = v3;
                let a692 = 0u64;
                if nat_eq(a691, a692) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a693, a694)?
        };
        let a696 = v2;
        nat_le(a695, a696)
    } else {
        false
    } {
        if v4 {
            let a701 = {
                let a699 = {
                    let a697 = v1;
                    let a698 = v5;
                    nat_add(a697, a698)?
                };
                let a700 = v6;
                nat_sub(a699, a700)
            };
            let a702 = v2;
            Ok(nat_lt(a701, a702))
        } else {
            let a707 = {
                let a705 = {
                    let a703 = v1;
                    let a704 = v5;
                    nat_add(a703, a704)?
                };
                let a706 = v6;
                nat_sub(a705, a706)
            };
            let a708 = v2;
            Ok(nat_le(a707, a708))
        }
    } else {
        Ok(false)
    }
}

pub fn f26(v0: u64, v1: u64) -> bool {
    let a709 = v0;
    let a710 = 0u64;
    if if nat_eq(a709, a710) {
        true
    } else {
        let a711 = v0;
        let a712 = 1u64;
        nat_eq(a711, a712)
    } {
        let a713 = v1;
        let a714 = 4294967296u64;
        nat_lt(a713, a714)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64) -> R<u64> {
    let a719 = {
        let a717 = v0;
        let a718 = {
            let a715 = v1;
            let a716 = 1u64;
            nat_sub(a715, a716)
        };
        nat_add(a717, a718)?
    };
    let a720 = v1;
    let a721 = 0u64;
    Ok(nat_quot(a719, a720, a721))
}

pub fn f28(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f21(match v0.clone() {
        Adt2::C0(_, h722, _, _, _, _, _, _, _, _, _) => {
            h722
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h723, _, _, _, _, _, _, _, _) => {
            h723
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h724, _, _, _, _, _, _, _) => {
            h724
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h725, _, _, _, _, _, _) => {
            h725
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a728 = match v1.clone() {
                Adt2::C0(_, h726, _, _, _, _, _, _, _, _, _) => {
                    h726
                }
            };
            let a729 = match v1.clone() {
                Adt2::C0(_, _, h727, _, _, _, _, _, _, _, _) => {
                    h727
                }
            };
            nat_mul(a728, a729)?
        };
        let v5: u64 = {
            let a731 = v4;
            let a732 = match v1.clone() {
                Adt2::C0(_, _, _, h730, _, _, _, _, _, _, _) => {
                    h730
                }
            };
            nat_mul(a731, a732)?
        };
        let a745 = {
            let a743 = {
                let a734 = match v0.clone() {
                    Adt2::C0(_, h733, _, _, _, _, _, _, _, _, _) => {
                        h733
                    }
                };
                let a735 = 1u64;
                nat_sub(a734, a735)
            };
            let a744 = {
                let a741 = f44(match v0.clone() {
                    Adt2::C0(_, _, h736, _, _, _, _, _, _, _, _) => {
                        h736
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h737, _, _, _, _, _, _, _, _, _) => {
                        h737
                    }
                })?;
                let a742 = f44(match v0 {
                    Adt2::C0(_, _, _, h738, _, _, _, _, _, _, _) => {
                        h738
                    }
                }, v2, {
                    let a739 = v3;
                    let a740 = v5;
                    nat_mul(a739, a740)?
                }, v4)?;
                nat_add(a741, a742)?
            };
            nat_add(a743, a744)?
        };
        let a746 = f15(v1, 1u64)?;
        Ok(nat_lt(a745, a746))
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a747 = v0;
    let a748 = 2147483648u64;
    if if if if nat_lt(a747, a748) {
        let a749 = v1;
        let a750 = 2147483648u64;
        nat_lt(a749, a750)
    } else {
        false
    } {
        let a753 = {
            let a751 = v0;
            let a752 = v1;
            nat_mul(a751, a752)?
        };
        let a754 = 2147483648u64;
        nat_lt(a753, a754)
    } else {
        false
    } {
        let a759 = {
            let a757 = {
                let a755 = v0;
                let a756 = 1u64;
                nat_sub(a755, a756)
            };
            let a758 = v2;
            nat_mul(a757, a758)?
        };
        let a760 = 2147483648u64;
        nat_lt(a759, a760)
    } else {
        false
    } {
        let a765 = {
            let a763 = {
                let a761 = v1;
                let a762 = 1u64;
                nat_sub(a761, a762)
            };
            let a764 = v2;
            nat_mul(a763, a764)?
        };
        let a766 = 2147483648u64;
        Ok(nat_lt(a765, a766))
    } else {
        Ok(false)
    }
}

pub fn f30(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m767 = v3;
    match m767 {
        None => {
            f45(v0, v1, match v2.clone() {
                Adt2::C0(h768, _, _, _, _, _, _, _, _, _, _) => {
                    h768
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h769, _, _, _, _, _, _, _, _, _, _) => {
                    h769
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h770, _, _, _, _, _, _, _, _, _) => {
                    h770
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h771, _, _, _, _, _, _, _, _) => {
                    h771
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h772, _, _, _, _, _, _, _) => {
                    h772
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h773, _, _, _, _, _, _) => {
                    h773
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h774, _, _, _, _, _, _, _, _, _) => {
                    h774
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h775, _, _, _, _, _, _, _, _) => {
                    h775
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h776, _, _, _, _, _, _, _) => {
                    h776
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h777, _, _, _, _, _, _) => {
                    h777
                }
            })
        }
        Some(v5) => {
            f45(v0, v1, match v2.clone() {
                Adt2::C0(h778, _, _, _, _, _, _, _, _, _, _) => {
                    h778
                }
            }, match v5.clone() {
                Adt2::C0(h779, _, _, _, _, _, _, _, _, _, _) => {
                    h779
                }
            }, match v4.clone() {
                Adt2::C0(h780, _, _, _, _, _, _, _, _, _, _) => {
                    h780
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h781, _, _, _, _, _, _, _, _, _) => {
                    h781
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h782, _, _, _, _, _, _, _, _) => {
                    h782
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h783, _, _, _, _, _, _, _) => {
                    h783
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h784, _, _, _, _, _, _) => {
                    h784
                }
            }, match v5.clone() {
                Adt2::C0(_, h785, _, _, _, _, _, _, _, _, _) => {
                    h785
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h786, _, _, _, _, _, _, _, _) => {
                    h786
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h787, _, _, _, _, _, _, _) => {
                    h787
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h788, _, _, _, _, _, _) => {
                    h788
                }
            }, match v4.clone() {
                Adt2::C0(_, h789, _, _, _, _, _, _, _, _, _) => {
                    h789
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h790, _, _, _, _, _, _, _, _) => {
                    h790
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h791, _, _, _, _, _, _, _) => {
                    h791
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h792, _, _, _, _, _, _) => {
                    h792
                }
            })
        }
    }
}

pub fn f31(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a794 = v0;
        let a795 = match v1.clone() {
            Adt3::C0(h793, _, _, _, _, _, _, _) => {
                h793
            }
        };
        let a796 = 0u64;
        nat_quot(a794, a795, a796)
    };
    let v3: u64 = {
        let a798 = v2;
        let a799 = match v1.clone() {
            Adt3::C0(_, h797, _, _, _, _, _, _) => {
                h797
            }
        };
        let a800 = 0u64;
        nat_quot(a798, a799, a800)
    };
    f46({
        let a802 = v0;
        let a803 = match v1.clone() {
            Adt3::C0(h801, _, _, _, _, _, _, _) => {
                h801
            }
        };
        let a804 = 0u64;
        nat_rem(a802, a803, a804)
    }, {
        let a806 = v2;
        let a807 = match v1.clone() {
            Adt3::C0(_, h805, _, _, _, _, _, _) => {
                h805
            }
        };
        let a808 = 0u64;
        nat_rem(a806, a807, a808)
    }, {
        let a810 = v3;
        let a811 = match v1.clone() {
            Adt3::C0(_, _, h809, _, _, _, _, _) => {
                h809
            }
        };
        let a812 = 0u64;
        nat_rem(a810, a811, a812)
    }, {
        let a814 = v3;
        let a815 = match v1.clone() {
            Adt3::C0(_, _, h813, _, _, _, _, _) => {
                h813
            }
        };
        let a816 = 0u64;
        nat_quot(a814, a815, a816)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h817, _, _, _) => {
            h817
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h818, _, _) => {
            h818
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h819, _) => {
            h819
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h820) => {
            h820
        }
    })
}

pub fn f32(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a822 = v0;
        let a823 = match v2.clone() {
            Adt3::C0(h821, _, _, _, _, _, _, _) => {
                h821
            }
        };
        let a824 = 0u64;
        nat_quot(a822, a823, a824)
    };
    let v4: u64 = {
        let a826 = v3;
        let a827 = match v2.clone() {
            Adt3::C0(_, h825, _, _, _, _, _, _) => {
                h825
            }
        };
        let a828 = 0u64;
        nat_quot(a826, a827, a828)
    };
    f47({
        let a830 = v0;
        let a831 = match v2.clone() {
            Adt3::C0(h829, _, _, _, _, _, _, _) => {
                h829
            }
        };
        let a832 = 0u64;
        nat_rem(a830, a831, a832)
    }, {
        let a834 = v3;
        let a835 = match v2.clone() {
            Adt3::C0(_, h833, _, _, _, _, _, _) => {
                h833
            }
        };
        let a836 = 0u64;
        nat_rem(a834, a835, a836)
    }, {
        let a838 = v4;
        let a839 = match v2.clone() {
            Adt3::C0(_, _, h837, _, _, _, _, _) => {
                h837
            }
        };
        let a840 = 0u64;
        nat_rem(a838, a839, a840)
    }, {
        let a842 = v4;
        let a843 = match v2 {
            Adt3::C0(_, _, h841, _, _, _, _, _) => {
                h841
            }
        };
        let a844 = 0u64;
        nat_quot(a842, a843, a844)
    }, match v1.clone() {
        Adt3::C0(h845, _, _, _, _, _, _, _) => {
            h845
        }
    }, match v1.clone() {
        Adt3::C0(_, h846, _, _, _, _, _, _) => {
            h846
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h847, _, _, _, _, _) => {
            h847
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h848, _, _, _, _) => {
            h848
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h849, _, _, _) => {
            h849
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h850, _, _) => {
            h850
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h851, _) => {
            h851
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h852) => {
            h852
        }
    })
}

pub fn f33(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a854 = v0;
        let a855 = match v3.clone() {
            Adt3::C0(h853, _, _, _, _, _, _, _) => {
                h853
            }
        };
        let a856 = 0u64;
        nat_quot(a854, a855, a856)
    };
    let v6: u64 = {
        let a858 = v5;
        let a859 = match v3.clone() {
            Adt3::C0(_, h857, _, _, _, _, _, _) => {
                h857
            }
        };
        let a860 = 0u64;
        nat_quot(a858, a859, a860)
    };
    let v7: u64 = {
        let a862 = v0;
        let a863 = match v3.clone() {
            Adt3::C0(h861, _, _, _, _, _, _, _) => {
                h861
            }
        };
        let a864 = 0u64;
        nat_rem(a862, a863, a864)
    };
    let v8: u64 = {
        let a866 = v5;
        let a867 = match v3.clone() {
            Adt3::C0(_, h865, _, _, _, _, _, _) => {
                h865
            }
        };
        let a868 = 0u64;
        nat_rem(a866, a867, a868)
    };
    let v9: u64 = {
        let a870 = v6;
        let a871 = match v3.clone() {
            Adt3::C0(_, _, h869, _, _, _, _, _) => {
                h869
            }
        };
        let a872 = 0u64;
        nat_rem(a870, a871, a872)
    };
    let v10: u64 = {
        let a874 = v6;
        let a875 = match v3.clone() {
            Adt3::C0(_, _, h873, _, _, _, _, _) => {
                h873
            }
        };
        let a876 = 0u64;
        nat_quot(a874, a875, a876)
    };
    let v11: u64 = {
        let a877 = v4;
        let a878 = 0u64;
        if nat_eq(a877, a878) {
            match v1.clone() {
                Adt3::C0(h879, _, _, _, _, _, _, _) => {
                    h879
                }
            }
        } else {
            let a880 = v4;
            let a881 = 1u64;
            if nat_eq(a880, a881) {
                match v1.clone() {
                    Adt3::C0(_, h882, _, _, _, _, _, _) => {
                        h882
                    }
                }
            } else {
                let a883 = v4;
                let a884 = 2u64;
                if nat_eq(a883, a884) {
                    match v1.clone() {
                        Adt3::C0(_, _, h885, _, _, _, _, _) => {
                            h885
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h886, _, _, _, _) => {
                            h886
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a893 = v11;
        let a894 = {
            let a887 = v4;
            let a888 = 0u64;
            if nat_eq(a887, a888) {
                v7
            } else {
                let a889 = v4;
                let a890 = 1u64;
                if nat_eq(a889, a890) {
                    v8
                } else {
                    let a891 = v4;
                    let a892 = 2u64;
                    if nat_eq(a891, a892) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a893, a894)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f46(if if v12 {
        let a895 = v4;
        let a896 = 0u64;
        nat_eq(a895, a896)
    } else {
        false
    } {
        let a897 = v7;
        let a898 = v11;
        nat_sub(a897, a898)
    } else {
        v7
    }, if if v12 {
        let a899 = v4;
        let a900 = 1u64;
        nat_eq(a899, a900)
    } else {
        false
    } {
        let a901 = v8;
        let a902 = v11;
        nat_sub(a901, a902)
    } else {
        v8
    }, if if v12 {
        let a903 = v4;
        let a904 = 2u64;
        nat_eq(a903, a904)
    } else {
        false
    } {
        let a905 = v9;
        let a906 = v11;
        nat_sub(a905, a906)
    } else {
        v9
    }, if if v12 {
        let a907 = v4;
        let a908 = 3u64;
        nat_eq(a907, a908)
    } else {
        false
    } {
        let a909 = v10;
        let a910 = v11;
        nat_sub(a909, a910)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h911, _, _, _) => {
            h911
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h912, _, _) => {
            h912
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h913, _) => {
            h913
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h914) => {
            h914
        }
    })?, f31(v0, v3)?))
}

pub fn f34(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f48(v0, v2.clone(), 0u64);
    let v9: u64 = f48(v0, v2.clone(), 1u64);
    let v10: u64 = f48(v0, v2.clone(), 2u64);
    let v11: u64 = f48(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f49(v1.clone(), f50(v8, v3, match v1.clone() {
            Adt3::C0(h915, _, _, _, _, _, _, _) => {
                h915
            }
        })?, f50(v9, v4, match v1.clone() {
            Adt3::C0(_, h916, _, _, _, _, _, _) => {
                h916
            }
        })?, f50(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h917, _, _, _, _, _) => {
                h917
            }
        })?, f50(v11, v6, match v1 {
            Adt3::C0(_, _, _, h918, _, _, _, _) => {
                h918
            }
        })?)?, f31(v0, v2)?))
    } else {
        if if if if f51(v8, v3, match v1.clone() {
            Adt3::C0(h919, _, _, _, _, _, _, _) => {
                h919
            }
        })? {
            f51(v9, v4, match v1.clone() {
                Adt3::C0(_, h920, _, _, _, _, _, _) => {
                    h920
                }
            })?
        } else {
            false
        } {
            f51(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h921, _, _, _, _, _) => {
                    h921
                }
            })?
        } else {
            false
        } {
            f51(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h922, _, _, _, _) => {
                    h922
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f49(v1, f52(v8, v3)?, f52(v9, v4)?, f52(v10, v5)?, f52(v11, v6)?)?, f31(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f31(v0, v2)?))
        }
    }
}

pub fn f35(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f48(v0, v2.clone(), 0u64);
    let v5: u64 = f48(v0, v2.clone(), 1u64);
    let v6: u64 = f48(v0, v2.clone(), 2u64);
    let v7: u64 = f48(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f49(v1.clone(), f53(v4, v3, match v1 {
        Adt3::C0(h923, _, _, _, _, _, _, _) => {
            h923
        }
    })?, v5, v6, v7)?, f31(v0, v2)?))
}

pub fn f36(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a924 = v0;
    let a925 = v3;
    if nat_lt(a924, a925) {
        Ok(Adt5::C1(false, {
            let a926 = v0;
            let a927 = 4u64;
            nat_mul(a926, a927)?
        }, {
            let a928 = v0;
            let a929 = 4u64;
            nat_mul(a928, a929)?
        }))
    } else {
        let v6: u64 = {
            let a930 = v0;
            let a931 = v3;
            nat_sub(a930, a931)
        };
        let v7: u64 = f31(v6, v1)?;
        let v8: u64 = f54(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f37(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a932 = v0;
    let a933 = v3;
    if nat_lt(a932, a933) {
        Ok(Adt5::C1(true, 0u64, f31(v0, v2)?))
    } else {
        let v4: u64 = {
            let a934 = v0;
            let a935 = v3;
            nat_sub(a934, a935)
        };
        let v5: u64 = f32(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f31(v4, v1)?, v5))
    }
}

pub fn f38(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a937 = v1;
        let a938 = match v3.clone() {
            Adt3::C0(h936, _, _, _, _, _, _, _) => {
                h936
            }
        };
        let a939 = 0u64;
        nat_rem(a937, a938, a939)
    };
    let v5: u64 = {
        let a945 = {
            let a941 = v1;
            let a942 = match v3.clone() {
                Adt3::C0(h940, _, _, _, _, _, _, _) => {
                    h940
                }
            };
            let a943 = 0u64;
            nat_quot(a941, a942, a943)
        };
        let a946 = match v3.clone() {
            Adt3::C0(_, h944, _, _, _, _, _, _) => {
                h944
            }
        };
        let a947 = 0u64;
        nat_rem(a945, a946, a947)
    };
    let v6: bool = f55(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f32(v1, v2, v3.clone())?
    } else {
        0u64
    }, f31(v1, v3)?))
}

pub fn f39(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f31(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f32(v0, v5, v2.clone())?
    }, f31(v0, v2)?))
}

pub fn f40(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a949 = v0;
        let a950 = match v2.clone() {
            Adt3::C0(h948, _, _, _, _, _, _, _) => {
                h948
            }
        };
        let a951 = 0u64;
        nat_rem(a949, a950, a951)
    };
    let v7: u64 = {
        let a953 = v0;
        let a954 = match v2.clone() {
            Adt3::C0(h952, _, _, _, _, _, _, _) => {
                h952
            }
        };
        let a955 = 0u64;
        nat_quot(a953, a954, a955)
    };
    let v8: u64 = {
        let a957 = v7;
        let a958 = match v2.clone() {
            Adt3::C0(_, h956, _, _, _, _, _, _) => {
                h956
            }
        };
        let a959 = 0u64;
        nat_rem(a957, a958, a959)
    };
    let v9: u64 = {
        let a961 = v7;
        let a962 = match v2.clone() {
            Adt3::C0(_, h960, _, _, _, _, _, _) => {
                h960
            }
        };
        let a963 = 0u64;
        nat_quot(a961, a962, a963)
    };
    let v10: u64 = {
        let a965 = v9;
        let a966 = match v2.clone() {
            Adt3::C0(_, _, h964, _, _, _, _, _) => {
                h964
            }
        };
        let a967 = 0u64;
        nat_rem(a965, a966, a967)
    };
    let v11: u64 = {
        let a969 = v9;
        let a970 = match v2.clone() {
            Adt3::C0(_, _, h968, _, _, _, _, _) => {
                h968
            }
        };
        let a971 = 0u64;
        nat_quot(a969, a970, a971)
    };
    let v12: u64 = f31(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a980 = {
                let a975 = {
                    let a972 = v10;
                    let a973 = v4;
                    let a974 = 0u64;
                    nat_quot(a972, a973, a974)
                };
                let a976 = v5;
                nat_mul(a975, a976)?
            };
            let a981 = {
                let a977 = v8;
                let a978 = v4;
                let a979 = 0u64;
                nat_quot(a977, a978, a979)
            };
            nat_add(a980, a981)?
        };
        let v14: u64 = {
            let a1001 = v6;
            let a1002 = {
                let a999 = match v1.clone() {
                    Adt3::C0(h982, _, _, _, _, _, _, _) => {
                        h982
                    }
                };
                let a1000 = {
                    let a997 = {
                        let a983 = v8;
                        let a984 = v4;
                        let a985 = 0u64;
                        nat_rem(a983, a984, a985)
                    };
                    let a998 = {
                        let a995 = match v1.clone() {
                            Adt3::C0(_, h986, _, _, _, _, _, _) => {
                                h986
                            }
                        };
                        let a996 = {
                            let a993 = {
                                let a987 = v10;
                                let a988 = v4;
                                let a989 = 0u64;
                                nat_rem(a987, a988, a989)
                            };
                            let a994 = {
                                let a991 = match v1.clone() {
                                    Adt3::C0(_, _, h990, _, _, _, _, _) => {
                                        h990
                                    }
                                };
                                let a992 = v13;
                                nat_mul(a991, a992)?
                            };
                            nat_add(a993, a994)?
                        };
                        nat_mul(a995, a996)?
                    };
                    nat_add(a997, a998)?
                };
                nat_mul(a999, a1000)?
            };
            nat_add(a1001, a1002)?
        };
        Ok(Adt5::C1(false, f31(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1008 = {
                let a1006 = {
                    let a1003 = v11;
                    let a1004 = v5;
                    let a1005 = 0u64;
                    nat_rem(a1003, a1004, a1005)
                };
                let a1007 = v4;
                nat_mul(a1006, a1007)?
            };
            let a1009 = v8;
            nat_add(a1008, a1009)?
        };
        let v16: u64 = {
            let a1015 = {
                let a1013 = {
                    let a1010 = v11;
                    let a1011 = v5;
                    let a1012 = 0u64;
                    nat_quot(a1010, a1011, a1012)
                };
                let a1014 = v4;
                nat_mul(a1013, a1014)?
            };
            let a1016 = v10;
            nat_add(a1015, a1016)?
        };
        let a1018 = v15;
        let a1019 = match v1.clone() {
            Adt3::C0(_, h1017, _, _, _, _, _, _) => {
                h1017
            }
        };
        if if nat_lt(a1018, a1019) {
            let a1021 = v16;
            let a1022 = match v1.clone() {
                Adt3::C0(_, _, h1020, _, _, _, _, _) => {
                    h1020
                }
            };
            nat_lt(a1021, a1022)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f31({
                let a1031 = v6;
                let a1032 = {
                    let a1029 = match v1.clone() {
                        Adt3::C0(h1023, _, _, _, _, _, _, _) => {
                            h1023
                        }
                    };
                    let a1030 = {
                        let a1027 = v15;
                        let a1028 = {
                            let a1025 = match v1.clone() {
                                Adt3::C0(_, h1024, _, _, _, _, _, _) => {
                                    h1024
                                }
                            };
                            let a1026 = v16;
                            nat_mul(a1025, a1026)?
                        };
                        nat_add(a1027, a1028)?
                    };
                    nat_mul(a1029, a1030)?
                };
                nat_add(a1031, a1032)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f41(v0: u64) -> u64 {
    let a1033 = v0;
    let a1034 = 2147483648u64;
    if nat_lt(a1033, a1034) {
        v0
    } else {
        0u64
    }
}

pub fn f42(v0: u64) -> u64 {
    let a1035 = v0;
    let a1036 = 2147483648u64;
    if nat_lt(a1035, a1036) {
        0u64
    } else {
        let a1037 = 4294967296u64;
        let a1038 = v0;
        nat_sub(a1037, a1038)
    }
}

pub fn f43(v0: Adt3, v1: bool) -> R<u64> {
    f56(v0.clone(), {
        let a1040 = match v0.clone() {
            Adt3::C0(h1039, _, _, _, _, _, _, _) => {
                h1039
            }
        };
        let a1041 = 1u64;
        nat_sub(a1040, a1041)
    }, {
        let a1043 = match v0.clone() {
            Adt3::C0(_, h1042, _, _, _, _, _, _) => {
                h1042
            }
        };
        let a1044 = 1u64;
        nat_sub(a1043, a1044)
    }, {
        let a1046 = match v0.clone() {
            Adt3::C0(_, _, h1045, _, _, _, _, _) => {
                h1045
            }
        };
        let a1047 = 1u64;
        nat_sub(a1046, a1047)
    }, {
        let a1049 = match v0 {
            Adt3::C0(_, _, _, h1048, _, _, _, _) => {
                h1048
            }
        };
        let a1050 = 1u64;
        nat_sub(a1049, a1050)
    }, v1)
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1051 = v0;
    let a1052 = 0u64;
    if nat_eq(a1051, a1052) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1053 = v0;
            let a1054 = 1u64;
            nat_sub(a1053, a1054)
        };
        let v5: u64 = {
            let a1055 = v4;
            let a1056 = v1;
            let a1057 = 0u64;
            nat_quot(a1055, a1056, a1057)
        };
        let v6: u64 = {
            let a1065 = {
                let a1058 = v5;
                let a1059 = v2;
                nat_mul(a1058, a1059)?
            };
            let a1066 = {
                let a1063 = {
                    let a1060 = v4;
                    let a1061 = v1;
                    let a1062 = 0u64;
                    nat_rem(a1060, a1061, a1062)
                };
                let a1064 = v3;
                nat_mul(a1063, a1064)?
            };
            nat_add(a1065, a1066)?
        };
        let a1067 = v5;
        let a1068 = 0u64;
        if nat_eq(a1067, a1068) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1077 = {
                    let a1071 = {
                        let a1069 = v5;
                        let a1070 = 1u64;
                        nat_sub(a1069, a1070)
                    };
                    let a1072 = v2;
                    nat_mul(a1071, a1072)?
                };
                let a1078 = {
                    let a1075 = {
                        let a1073 = v1;
                        let a1074 = 1u64;
                        nat_sub(a1073, a1074)
                    };
                    let a1076 = v3;
                    nat_mul(a1075, a1076)?
                };
                nat_add(a1077, a1078)?
            };
            let a1079 = v6;
            let a1080 = v7;
            if nat_lt(a1079, a1080) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1081 = f57(v0, v1);
    match m1081 {
        None => {
            if v5 {
                let m1082 = f58(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1082 {
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
                let a1083 = 4u64;
                let a1084 = v28;
                nat_le(a1083, a1084)
            };
            let a1085 = v2;
            let a1086 = 0u64;
            if if if if if nat_eq(a1085, a1086) {
                let a1087 = v4;
                let a1088 = 0u64;
                nat_eq(a1087, a1088)
            } else {
                false
            } {
                let a1089 = v6;
                let a1090 = v14;
                if nat_eq(a1089, a1090) {
                    let a1091 = v7;
                    let a1092 = v15;
                    nat_eq(a1091, a1092)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1093 = v8;
                let a1094 = v16;
                if nat_eq(a1093, a1094) {
                    let a1095 = v9;
                    let a1096 = v17;
                    nat_eq(a1095, a1096)
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
                        let a1097 = v3;
                        let a1098 = 0u64;
                        nat_eq(a1097, a1098)
                    } else {
                        false
                    } {
                        f18(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1103 = {
                        let a1099 = v6;
                        let a1100 = v7;
                        nat_mul(a1099, a1100)?
                    };
                    let a1104 = {
                        let a1101 = v8;
                        let a1102 = v9;
                        nat_mul(a1101, a1102)?
                    };
                    nat_mul(a1103, a1104)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1117 = {
        let a1113 = {
            let a1109 = {
                let a1105 = v0;
                let a1106 = v4;
                nat_mul(a1105, a1106)?
            };
            let a1110 = {
                let a1107 = v1;
                let a1108 = v5;
                nat_mul(a1107, a1108)?
            };
            nat_add(a1109, a1110)?
        };
        let a1114 = {
            let a1111 = v2;
            let a1112 = v6;
            nat_mul(a1111, a1112)?
        };
        nat_add(a1113, a1114)?
    };
    let a1118 = {
        let a1115 = v3;
        let a1116 = v7;
        nat_mul(a1115, a1116)?
    };
    nat_add(a1117, a1118)
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f46({
        let a1119 = v0;
        let a1120 = v4;
        let a1121 = 0u64;
        nat_rem(a1119, a1120, a1121)
    }, {
        let a1122 = v1;
        let a1123 = v5;
        let a1124 = 0u64;
        nat_rem(a1122, a1123, a1124)
    }, {
        let a1125 = v2;
        let a1126 = v6;
        let a1127 = 0u64;
        nat_rem(a1125, a1126, a1127)
    }, {
        let a1128 = v3;
        let a1129 = v7;
        let a1130 = 0u64;
        nat_rem(a1128, a1129, a1130)
    }, v8, v9, v10, v11)
}

pub fn f48(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1132 = v0;
        let a1133 = match v1.clone() {
            Adt3::C0(h1131, _, _, _, _, _, _, _) => {
                h1131
            }
        };
        let a1134 = 0u64;
        nat_quot(a1132, a1133, a1134)
    };
    let v4: u64 = {
        let a1136 = v3;
        let a1137 = match v1.clone() {
            Adt3::C0(_, h1135, _, _, _, _, _, _) => {
                h1135
            }
        };
        let a1138 = 0u64;
        nat_quot(a1136, a1137, a1138)
    };
    let a1139 = v2;
    let a1140 = 0u64;
    if nat_eq(a1139, a1140) {
        let a1142 = v0;
        let a1143 = match v1 {
            Adt3::C0(h1141, _, _, _, _, _, _, _) => {
                h1141
            }
        };
        let a1144 = 0u64;
        nat_rem(a1142, a1143, a1144)
    } else {
        let a1145 = v2;
        let a1146 = 1u64;
        if nat_eq(a1145, a1146) {
            let a1148 = v3;
            let a1149 = match v1 {
                Adt3::C0(_, h1147, _, _, _, _, _, _) => {
                    h1147
                }
            };
            let a1150 = 0u64;
            nat_rem(a1148, a1149, a1150)
        } else {
            let a1151 = v2;
            let a1152 = 2u64;
            if nat_eq(a1151, a1152) {
                let a1154 = v4;
                let a1155 = match v1 {
                    Adt3::C0(_, _, h1153, _, _, _, _, _) => {
                        h1153
                    }
                };
                let a1156 = 0u64;
                nat_rem(a1154, a1155, a1156)
            } else {
                let a1158 = v4;
                let a1159 = match v1 {
                    Adt3::C0(_, _, h1157, _, _, _, _, _) => {
                        h1157
                    }
                };
                let a1160 = 0u64;
                nat_quot(a1158, a1159, a1160)
            }
        }
    }
}

pub fn f49(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f46(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1161, _, _, _) => {
            h1161
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1162, _, _) => {
            h1162
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1163, _) => {
            h1163
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1164) => {
            h1164
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1165 = v1;
    let a1166 = 2147483648u64;
    if nat_lt(a1165, a1166) {
        let a1177 = {
            let a1175 = {
                let a1170 = {
                    let a1167 = v0;
                    let a1168 = v2;
                    let a1169 = 0u64;
                    nat_rem(a1167, a1168, a1169)
                };
                let a1171 = v2;
                nat_add(a1170, a1171)?
            };
            let a1176 = {
                let a1172 = v1;
                let a1173 = v2;
                let a1174 = 0u64;
                nat_rem(a1172, a1173, a1174)
            };
            nat_sub(a1175, a1176)
        };
        let a1178 = v2;
        let a1179 = 0u64;
        Ok(nat_rem(a1177, a1178, a1179))
    } else {
        let a1182 = {
            let a1180 = v0;
            let a1181 = f42(v1);
            nat_add(a1180, a1181)?
        };
        let a1183 = v2;
        let a1184 = 0u64;
        Ok(nat_rem(a1182, a1183, a1184))
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1187 = f41(v1);
    let a1188 = {
        let a1185 = v0;
        let a1186 = f42(v1);
        nat_add(a1185, a1186)?
    };
    if nat_le(a1187, a1188) {
        let a1189 = f52(v0, v1)?;
        let a1190 = v2;
        Ok(nat_lt(a1189, a1190))
    } else {
        Ok(false)
    }
}

pub fn f52(v0: u64, v1: u64) -> R<u64> {
    let a1193 = {
        let a1191 = v0;
        let a1192 = f42(v1);
        nat_add(a1191, a1192)?
    };
    let a1194 = f41(v1);
    Ok(nat_sub(a1193, a1194))
}

pub fn f53(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1195 = v0;
    let a1196 = v1;
    if nat_lt(a1195, a1196) {
        let a1197 = v1;
        let a1198 = v0;
        Ok(nat_sub(a1197, a1198))
    } else {
        let a1201 = v0;
        let a1202 = {
            let a1199 = v1;
            let a1200 = v2;
            nat_add(a1199, a1200)?
        };
        if nat_lt(a1201, a1202) {
            let a1203 = v0;
            let a1204 = v1;
            Ok(nat_sub(a1203, a1204))
        } else {
            let a1211 = {
                let a1207 = 2u64;
                let a1208 = {
                    let a1205 = v2;
                    let a1206 = 1u64;
                    nat_sub(a1205, a1206)
                };
                nat_mul(a1207, a1208)?
            };
            let a1212 = {
                let a1209 = v0;
                let a1210 = v1;
                nat_sub(a1209, a1210)
            };
            Ok(nat_sub(a1211, a1212))
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1215 = {
        let a1213 = v2;
        let a1214 = f56(v1.clone(), f48(v0, v1.clone(), 0u64), f48(v0, v1.clone(), 1u64), f48(v0, v1.clone(), 2u64), f48(v0, v1.clone(), 3u64), false)?;
        nat_add(a1213, a1214)?
    };
    let a1216 = f56(v1.clone(), f48(v0, v1.clone(), 0u64), f48(v0, v1.clone(), 1u64), f48(v0, v1.clone(), 2u64), f48(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1215, a1216))
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> bool {
    let a1217 = v0;
    let a1218 = 0u64;
    if nat_eq(a1217, a1218) {
        let a1219 = v2;
        let a1220 = v1;
        nat_le(a1219, a1220)
    } else {
        let a1221 = v0;
        let a1222 = 1u64;
        if nat_eq(a1221, a1222) {
            let a1223 = v2;
            let a1224 = v1;
            nat_lt(a1223, a1224)
        } else {
            let a1225 = v0;
            let a1226 = 2u64;
            if nat_eq(a1225, a1226) {
                let a1227 = v1;
                let a1228 = v2;
                nat_le(a1227, a1228)
            } else {
                let a1229 = v0;
                let a1230 = 3u64;
                if nat_eq(a1229, a1230) {
                    let a1231 = v1;
                    let a1232 = v2;
                    nat_lt(a1231, a1232)
                } else {
                    let a1233 = v1;
                    let a1234 = v2;
                    nat_eq(a1233, a1234)
                }
            }
        }
    }
}

pub fn f56(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1255 = {
        let a1249 = {
            let a1243 = {
                let a1237 = v1;
                let a1238 = if v5 {
                    f42(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1235, _, _, _) => {
                            h1235
                        }
                    })
                } else {
                    f41(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1236, _, _, _) => {
                            h1236
                        }
                    })
                };
                nat_mul(a1237, a1238)?
            };
            let a1244 = {
                let a1241 = v2;
                let a1242 = if v5 {
                    f42(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1239, _, _) => {
                            h1239
                        }
                    })
                } else {
                    f41(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1240, _, _) => {
                            h1240
                        }
                    })
                };
                nat_mul(a1241, a1242)?
            };
            nat_add(a1243, a1244)?
        };
        let a1250 = {
            let a1247 = v3;
            let a1248 = if v5 {
                f42(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1245, _) => {
                        h1245
                    }
                })
            } else {
                f41(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1246, _) => {
                        h1246
                    }
                })
            };
            nat_mul(a1247, a1248)?
        };
        nat_add(a1249, a1250)?
    };
    let a1256 = {
        let a1253 = v4;
        let a1254 = if v5 {
            f42(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1251) => {
                    h1251
                }
            })
        } else {
            f41(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1252) => {
                    h1252
                }
            })
        };
        nat_mul(a1253, a1254)?
    };
    nat_add(a1255, a1256)
}

pub fn f57(v0: u64, v1: u64) -> Option<u64> {
    let a1257 = v0;
    let a1258 = 2u64;
    if nat_eq(a1257, a1258) {
        Some(0u64)
    } else {
        let a1259 = v0;
        let a1260 = 5u64;
        if nat_eq(a1259, a1260) {
            Some(1u64)
        } else {
            let a1261 = v0;
            let a1262 = 6u64;
            if nat_eq(a1261, a1262) {
                Some(2u64)
            } else {
                let a1263 = v0;
                let a1264 = 7u64;
                if nat_eq(a1263, a1264) {
                    Some(3u64)
                } else {
                    let a1265 = v0;
                    let a1266 = 9u64;
                    if nat_eq(a1265, a1266) {
                        Some(4u64)
                    } else {
                        let a1267 = v0;
                        let a1268 = 8u64;
                        if nat_eq(a1267, a1268) {
                            Some(5u64)
                        } else {
                            let a1269 = v0;
                            let a1270 = 89u64;
                            if nat_eq(a1269, a1270) {
                                let a1271 = v1;
                                let a1272 = 2u64;
                                if nat_eq(a1271, a1272) {
                                    Some(6u64)
                                } else {
                                    let a1273 = v1;
                                    let a1274 = 0u64;
                                    if nat_eq(a1273, a1274) {
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

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1275 = v0;
    let a1276 = 28u64;
    if if if if nat_eq(a1275, a1276) {
        let a1277 = v1;
        let a1278 = 0u64;
        nat_eq(a1277, a1278)
    } else {
        false
    } {
        let a1279 = v2;
        let a1280 = 0u64;
        nat_eq(a1279, a1280)
    } else {
        false
    } {
        let a1281 = v3;
        let a1282 = 0u64;
        nat_eq(a1281, a1282)
    } else {
        false
    } {
        let a1283 = v4;
        let a1284 = v8;
        if if if if if if if if nat_eq(a1283, a1284) {
            let a1285 = v5;
            let a1286 = v12;
            nat_eq(a1285, a1286)
        } else {
            false
        } {
            let a1287 = v9;
            let a1288 = v13;
            nat_eq(a1287, a1288)
        } else {
            false
        } {
            let a1289 = v10;
            let a1290 = v14;
            nat_eq(a1289, a1290)
        } else {
            false
        } {
            let a1291 = v11;
            let a1292 = v15;
            nat_eq(a1291, a1292)
        } else {
            false
        } {
            let a1293 = 0u64;
            let a1294 = v6;
            nat_lt(a1293, a1294)
        } else {
            false
        } {
            let a1295 = 0u64;
            let a1296 = v7;
            nat_lt(a1295, a1296)
        } else {
            false
        } {
            let a1300 = {
                let a1297 = v10;
                let a1298 = v6;
                let a1299 = 1u64;
                nat_rem(a1297, a1298, a1299)
            };
            let a1301 = 0u64;
            if nat_eq(a1300, a1301) {
                let a1305 = {
                    let a1302 = v11;
                    let a1303 = v7;
                    let a1304 = 1u64;
                    nat_rem(a1302, a1303, a1304)
                };
                let a1306 = 0u64;
                nat_eq(a1305, a1306)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1307 = v10;
                let a1308 = v6;
                let a1309 = 0u64;
                nat_quot(a1307, a1308, a1309)
            }, {
                let a1310 = v11;
                let a1311 = v7;
                let a1312 = 0u64;
                nat_quot(a1310, a1311, a1312)
            }, {
                let a1313 = v5;
                let a1314 = v4;
                nat_mul(a1313, a1314)?
            }, {
                let a1315 = v4;
                let a1316 = v9;
                nat_mul(a1315, a1316)?
            }, {
                let a1317 = v5;
                let a1318 = v9;
                nat_mul(a1317, a1318)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
