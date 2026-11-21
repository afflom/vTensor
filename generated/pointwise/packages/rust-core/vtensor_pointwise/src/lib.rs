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
    C13(u64, u64, u64, u64),
    C14(u64, u64, u64),
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
                                                    let a34 = v1;
                                                    let a35 = 31u64;
                                                    if if if nat_eq(a34, a35) {
                                                        true
                                                    } else {
                                                        let a36 = v1;
                                                        let a37 = 47u64;
                                                        nat_eq(a36, a37)
                                                    } {
                                                        true
                                                    } else {
                                                        let a38 = v1;
                                                        let a39 = 68u64;
                                                        nat_eq(a38, a39)
                                                    } {
                                                        f11(v1, v2, v3, v5)?
                                                    } else {
                                                        let m40 = f12(v1, v6);
                                                        match m40 {
                                                            None => {
                                                                f13(v1, v6, v3, v4, v5)?
                                                            }
                                                            Some(v8) => {
                                                                f14(v8, v3, v5)?
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
            }))
        }
        Adt0::C1(v9, v10, v11, v12) => {
            f15(v9, v10, v11, v12)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a41 = v0;
    let a42 = 1u64;
    if if nat_eq(a41, a42) {
        true
    } else {
        let a43 = v0;
        let a44 = 33u64;
        nat_eq(a43, a44)
    } {
        true
    } else {
        let a45 = v0;
        let a46 = 34u64;
        nat_eq(a45, a46)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a49 = match v1.clone() {
        Adt2::C0(h47, _, _, _, _, _, _, _, _, _, _) => {
            h47
        }
    };
    let a50 = match v3.clone() {
        Adt2::C0(h48, _, _, _, _, _, _, _, _, _, _) => {
            h48
        }
    };
    if nat_eq(a49, a50) {
        f16(v0, v1, v2, v3)
    } else {
        if if if if f17(match v1.clone() {
            Adt2::C0(h51, _, _, _, _, _, _, _, _, _, _) => {
                h51
            }
        }) {
            f17(match v3.clone() {
                Adt2::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
                    h52
                }
            })
        } else {
            false
        } {
            let a53 = v0;
            let a54 = 33u64;
            if nat_eq(a53, a54) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a55 = f18(v1.clone(), 1u64)?;
            let a56 = f18(v3.clone(), 1u64)?;
            nat_eq(a55, a56)
        } else {
            false
        } {
            let m58 = f19(match v3.clone() {
                Adt2::C0(h57, _, _, _, _, _, _, _, _, _, _) => {
                    h57
                }
            });
            match m58 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f18(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h59) => {
                            h59
                        }
                    }, f20(v1.clone(), 1u64), f20(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                            h60
                        }
                    }, match v3 {
                        Adt2::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                            h61
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
    let a63 = match v0.clone() {
        Adt2::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
            h62
        }
    };
    let a64 = 0u64;
    if if if if nat_eq(a63, a64) {
        true
    } else {
        let a66 = match v0.clone() {
            Adt2::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        };
        let a67 = 1u64;
        nat_eq(a66, a67)
    } {
        true
    } else {
        let a69 = match v0.clone() {
            Adt2::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        let a70 = 25u64;
        if nat_eq(a69, a70) {
            true
        } else {
            let a72 = match v0.clone() {
                Adt2::C0(h71, _, _, _, _, _, _, _, _, _, _) => {
                    h71
                }
            };
            let a73 = 26u64;
            nat_eq(a72, a73)
        }
    } {
        true
    } else {
        let a75 = match v0.clone() {
            Adt2::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        };
        let a76 = 30u64;
        nat_eq(a75, a76)
    } {
        let a79 = match v0.clone() {
            Adt2::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                h77
            }
        };
        let a80 = match v1.clone() {
            Adt2::C0(h78, _, _, _, _, _, _, _, _, _, _) => {
                h78
            }
        };
        if if nat_eq(a79, a80) {
            f21(match v1.clone() {
                Adt2::C0(_, h81, _, _, _, _, _, _, _, _, _) => {
                    h81
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h82, _, _, _, _, _, _, _, _) => {
                    h82
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h83, _, _, _, _, _, _, _) => {
                    h83
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h84, _, _, _, _, _, _) => {
                    h84
                }
            }, match v0.clone() {
                Adt2::C0(_, h85, _, _, _, _, _, _, _, _, _) => {
                    h85
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h86, _, _, _, _, _, _, _, _) => {
                    h86
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h87, _, _, _, _, _, _, _) => {
                    h87
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h88, _, _, _, _, _, _) => {
                    h88
                }
            })
        } else {
            false
        } {
            let m90 = f19(match v0.clone() {
                Adt2::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                    h89
                }
            });
            match m90 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f18(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h91) => {
                            h91
                        }
                    }, f20(v0, 1u64), f20(v1, 1u64), Adt4::C1))
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
    let m92 = v2;
    match m92 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a93 = v0;
            let a94 = 4u64;
            if if if nat_lt(a93, a94) {
                let a97 = match v1.clone() {
                    Adt2::C0(h95, _, _, _, _, _, _, _, _, _, _) => {
                        h95
                    }
                };
                let a98 = match v4.clone() {
                    Adt2::C0(h96, _, _, _, _, _, _, _, _, _, _) => {
                        h96
                    }
                };
                if nat_eq(a97, a98) {
                    let a101 = match v1.clone() {
                        Adt2::C0(h99, _, _, _, _, _, _, _, _, _, _) => {
                            h99
                        }
                    };
                    let a102 = match v3.clone() {
                        Adt2::C0(h100, _, _, _, _, _, _, _, _, _, _) => {
                            h100
                        }
                    };
                    nat_eq(a101, a102)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f22(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h103, _, _, _, _, _, _, _, _, _) => {
                        h103
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h105, _, _, _, _, _, _, _, _, _) => {
                        h105
                    }
                })? {
                    f22(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h106, _, _, _, _, _, _, _, _) => {
                            h106
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h107, _, _, _, _, _, _, _, _) => {
                            h107
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h108, _, _, _, _, _, _, _, _) => {
                            h108
                        }
                    })?
                } else {
                    false
                } {
                    f22(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h109, _, _, _, _, _, _, _) => {
                            h109
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h110, _, _, _, _, _, _, _) => {
                            h110
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h111, _, _, _, _, _, _, _) => {
                            h111
                        }
                    })?
                } else {
                    false
                } {
                    f22(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h112, _, _, _, _, _, _) => {
                            h112
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h113, _, _, _, _, _, _) => {
                            h113
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h114, _, _, _, _, _, _) => {
                            h114
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m116 = f19(match v1.clone() {
                    Adt2::C0(h115, _, _, _, _, _, _, _, _, _, _) => {
                        h115
                    }
                });
                match m116 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h117, _) => {
                                h117
                            }
                        };
                        let a122 = {
                            let a119 = match v1.clone() {
                                Adt2::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                                    h118
                                }
                            };
                            let a120 = v6;
                            let a121 = 1u64;
                            nat_rem(a119, a120, a121)
                        };
                        let a123 = 0u64;
                        if if if nat_eq(a122, a123) {
                            let a128 = {
                                let a125 = match v4.clone() {
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
                            let a134 = {
                                let a131 = match v3.clone() {
                                    Adt2::C0(_, h130, _, _, _, _, _, _, _, _, _) => {
                                        h130
                                    }
                                };
                                let a132 = v6;
                                let a133 = 1u64;
                                nat_rem(a131, a132, a133)
                            };
                            let a135 = 0u64;
                            nat_eq(a134, a135)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f18(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h136) => {
                                    h136
                                }
                            }, f20(v1, v6), f20(v3, v6), Adt4::C2(v0, f20(v4, v6))))
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
    let a138 = match v2.clone() {
        Adt2::C0(h137, _, _, _, _, _, _, _, _, _, _) => {
            h137
        }
    };
    let a139 = 0u64;
    if if nat_eq(a138, a139) {
        let a141 = match v3.clone() {
            Adt2::C0(h140, _, _, _, _, _, _, _, _, _, _) => {
                h140
            }
        };
        let a142 = 0u64;
        nat_eq(a141, a142)
    } else {
        false
    } {
        let a143 = v0;
        let a144 = 61u64;
        if nat_eq(a143, a144) {
            let v4: bool = {
                let a146 = 0u64;
                let a147 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h145, _, _, _, _, _, _, _) => {
                        h145
                    }
                };
                nat_lt(a146, a147)
            };
            if if if if if if f23(match v2.clone() {
                Adt2::C0(_, h148, _, _, _, _, _, _, _, _, _) => {
                    h148
                }
            }, match v1.clone() {
                Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h149
                }
            }, match v1.clone() {
                Adt1::C0(_, h150, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h150
                }
            }, match v3.clone() {
                Adt2::C0(_, h151, _, _, _, _, _, _, _, _, _) => {
                    h151
                }
            })? {
                f23(match v2.clone() {
                    Adt2::C0(_, _, h152, _, _, _, _, _, _, _, _) => {
                        h152
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h153, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h153
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h154, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h155, _, _, _, _, _, _, _, _) => {
                        h155
                    }
                })?
            } else {
                false
            } {
                f23(match v2.clone() {
                    Adt2::C0(_, _, _, h156, _, _, _, _, _, _, _) => {
                        h156
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h157, _, _, _, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h158, _, _, _, _, _, _, _, _, _, _) => {
                        h158
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h159, _, _, _, _, _, _, _) => {
                        h159
                    }
                })?
            } else {
                false
            } {
                f23(match v2.clone() {
                    Adt2::C0(_, _, _, _, h160, _, _, _, _, _, _) => {
                        h160
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h161, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h162, _, _, _, _, _, _, _, _) => {
                        h162
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h163, _, _, _, _, _, _) => {
                        h163
                    }
                })?
            } else {
                false
            } {
                let a165 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h164, _, _, _, _, _, _, _) => {
                        h164
                    }
                };
                let a166 = 4294967295u64;
                nat_le(a165, a166)
            } else {
                false
            } {
                if v4 {
                    !f24(match v2.clone() {
                        Adt2::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
                            h167
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h168, _, _, _, _, _, _, _, _) => {
                            h168
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h169, _, _, _, _, _, _, _) => {
                            h169
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h170, _, _, _, _, _, _) => {
                            h170
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f18(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h171, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h171
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h172, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h172
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h173, _, _, _, _, _, _, _, _, _, _, _) => {
                        h173
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h174, _, _, _, _, _, _, _, _, _) => {
                        h174
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a175 = v0;
            let a176 = 62u64;
            if nat_eq(a175, a176) {
                let a178 = match v1.clone() {
                    Adt1::C0(h177, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h177
                    }
                };
                let a179 = 2147483648u64;
                if if if if if nat_lt(a178, a179) {
                    let a181 = match v1.clone() {
                        Adt1::C0(_, h180, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h180
                        }
                    };
                    let a182 = 2147483648u64;
                    nat_lt(a181, a182)
                } else {
                    false
                } {
                    let a185 = match v1.clone() {
                        Adt1::C0(h183, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h183
                        }
                    };
                    let a186 = match v2.clone() {
                        Adt2::C0(_, h184, _, _, _, _, _, _, _, _, _) => {
                            h184
                        }
                    };
                    nat_lt(a185, a186)
                } else {
                    false
                } {
                    let a189 = match v1.clone() {
                        Adt1::C0(_, h187, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h187
                        }
                    };
                    let a190 = match v2.clone() {
                        Adt2::C0(_, h188, _, _, _, _, _, _, _, _, _) => {
                            h188
                        }
                    };
                    nat_lt(a189, a190)
                } else {
                    false
                } {
                    let a193 = match v2.clone() {
                        Adt2::C0(_, _, h191, _, _, _, _, _, _, _, _) => {
                            h191
                        }
                    };
                    let a194 = match v3.clone() {
                        Adt2::C0(_, _, h192, _, _, _, _, _, _, _, _) => {
                            h192
                        }
                    };
                    if if if nat_eq(a193, a194) {
                        let a197 = match v2.clone() {
                            Adt2::C0(_, _, _, h195, _, _, _, _, _, _, _) => {
                                h195
                            }
                        };
                        let a198 = match v3.clone() {
                            Adt2::C0(_, _, _, h196, _, _, _, _, _, _, _) => {
                                h196
                            }
                        };
                        nat_eq(a197, a198)
                    } else {
                        false
                    } {
                        let a201 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                                h199
                            }
                        };
                        let a202 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h200, _, _, _, _, _, _) => {
                                h200
                            }
                        };
                        nat_eq(a201, a202)
                    } else {
                        false
                    } {
                        let a211 = match v3.clone() {
                            Adt2::C0(_, h203, _, _, _, _, _, _, _, _, _) => {
                                h203
                            }
                        };
                        let a212 = {
                            let a209 = {
                                let a206 = match v2.clone() {
                                    Adt2::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                                        h204
                                    }
                                };
                                let a207 = match v1.clone() {
                                    Adt1::C0(h205, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h205
                                    }
                                };
                                nat_add(a206, a207)?
                            };
                            let a210 = match v1.clone() {
                                Adt1::C0(_, h208, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h208
                                }
                            };
                            nat_add(a209, a210)?
                        };
                        nat_eq(a211, a212)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f18(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h213, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h213
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a214 = v0;
                let a215 = 63u64;
                if nat_eq(a214, a215) {
                    if if if if if f25(match v2.clone() {
                        Adt2::C0(_, h216, _, _, _, _, _, _, _, _, _) => {
                            h216
                        }
                    }, match v1.clone() {
                        Adt1::C0(h217, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h217
                        }
                    })? {
                        f25(match v2.clone() {
                            Adt2::C0(_, _, h218, _, _, _, _, _, _, _, _) => {
                                h218
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h219, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h219
                            }
                        })?
                    } else {
                        false
                    } {
                        f25(match v2.clone() {
                            Adt2::C0(_, _, _, h220, _, _, _, _, _, _, _) => {
                                h220
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h221, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h221
                            }
                        })?
                    } else {
                        false
                    } {
                        f25(match v2.clone() {
                            Adt2::C0(_, _, _, _, h222, _, _, _, _, _, _) => {
                                h222
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h223, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h223
                            }
                        })?
                    } else {
                        false
                    } {
                        let a226 = match v2.clone() {
                            Adt2::C0(_, h224, _, _, _, _, _, _, _, _, _) => {
                                h224
                            }
                        };
                        let a227 = match v3.clone() {
                            Adt2::C0(_, h225, _, _, _, _, _, _, _, _, _) => {
                                h225
                            }
                        };
                        if if if nat_eq(a226, a227) {
                            let a230 = match v2.clone() {
                                Adt2::C0(_, _, h228, _, _, _, _, _, _, _, _) => {
                                    h228
                                }
                            };
                            let a231 = match v3.clone() {
                                Adt2::C0(_, _, h229, _, _, _, _, _, _, _, _) => {
                                    h229
                                }
                            };
                            nat_eq(a230, a231)
                        } else {
                            false
                        } {
                            let a234 = match v2.clone() {
                                Adt2::C0(_, _, _, h232, _, _, _, _, _, _, _) => {
                                    h232
                                }
                            };
                            let a235 = match v3.clone() {
                                Adt2::C0(_, _, _, h233, _, _, _, _, _, _, _) => {
                                    h233
                                }
                            };
                            nat_eq(a234, a235)
                        } else {
                            false
                        } {
                            let a238 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h236, _, _, _, _, _, _) => {
                                    h236
                                }
                            };
                            let a239 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h237, _, _, _, _, _, _) => {
                                    h237
                                }
                            };
                            nat_eq(a238, a239)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f18(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h240, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h240
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h241, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h241
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h242, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h242
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h243, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h243
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
    let m244 = v3;
    match m244 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a245 = v0;
                let a246 = 4u64;
                nat_eq(a245, a246)
            };
            let v7: bool = {
                let a248 = 0u64;
                let a249 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h247, _, _, _, _, _, _, _, _, _, _, _) => {
                        h247
                    }
                };
                nat_lt(a248, a249)
            };
            let v8: u64 = f18(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h250, _, _, _, _, _, _, _, _, _) => {
                    h250
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h251, _, _, _, _, _, _, _, _) => {
                    h251
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h252, _, _, _, _, _, _, _) => {
                    h252
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h253, _, _, _, _, _, _) => {
                    h253
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h254, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h254
                }
            }, match v1.clone() {
                Adt1::C0(_, h255, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h255
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h256, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h256
                }
            });
            if if if if if if if if if if if if v6 {
                let a258 = match v2.clone() {
                    Adt2::C0(h257, _, _, _, _, _, _, _, _, _, _) => {
                        h257
                    }
                };
                let a259 = 0u64;
                if nat_eq(a258, a259) {
                    let a261 = match v5.clone() {
                        Adt2::C0(h260, _, _, _, _, _, _, _, _, _, _) => {
                            h260
                        }
                    };
                    let a262 = 0u64;
                    nat_eq(a261, a262)
                } else {
                    false
                }
            } else {
                let a264 = match v2.clone() {
                    Adt2::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
                        h263
                    }
                };
                let a265 = 0u64;
                if if nat_eq(a264, a265) {
                    true
                } else {
                    let a267 = match v2.clone() {
                        Adt2::C0(h266, _, _, _, _, _, _, _, _, _, _) => {
                            h266
                        }
                    };
                    let a268 = 26u64;
                    nat_eq(a267, a268)
                } {
                    let a270 = match v5.clone() {
                        Adt2::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                            h269
                        }
                    };
                    let a271 = 0u64;
                    if nat_eq(a270, a271) {
                        true
                    } else {
                        let a273 = match v5.clone() {
                            Adt2::C0(h272, _, _, _, _, _, _, _, _, _, _) => {
                                h272
                            }
                        };
                        let a274 = 26u64;
                        nat_eq(a273, a274)
                    }
                } else {
                    false
                }
            } {
                let a277 = match v2.clone() {
                    Adt2::C0(h275, _, _, _, _, _, _, _, _, _, _) => {
                        h275
                    }
                };
                let a278 = match v4.clone() {
                    Adt2::C0(h276, _, _, _, _, _, _, _, _, _, _) => {
                        h276
                    }
                };
                nat_eq(a277, a278)
            } else {
                false
            } {
                f26(v2.clone(), v4.clone())
            } else {
                false
            } {
                f27(v2.clone())?
            } else {
                false
            } {
                f27(v4.clone())?
            } else {
                false
            } {
                let a280 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h279, _, _, _, _, _) => {
                        h279
                    }
                };
                let a281 = 4u64;
                nat_eq(a280, a281)
            } else {
                false
            } {
                let a282 = v8;
                let a283 = f18(v2.clone(), 1u64)?;
                nat_le(a282, a283)
            } else {
                false
            } {
                let a285 = match v1.clone() {
                    Adt1::C0(_, _, _, h284, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h284
                    }
                };
                let a286 = 1073741824u64;
                nat_lt(a285, a286)
            } else {
                false
            } {
                let a288 = match v1.clone() {
                    Adt1::C0(h287, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h287
                    }
                };
                let a289 = 4294967295u64;
                if if if if nat_le(a288, a289) {
                    let a291 = match v1.clone() {
                        Adt1::C0(_, h290, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h290
                        }
                    };
                    let a292 = 4294967295u64;
                    nat_le(a291, a292)
                } else {
                    false
                } {
                    let a294 = match v1.clone() {
                        Adt1::C0(_, _, h293, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h293
                        }
                    };
                    let a295 = 4294967295u64;
                    nat_le(a294, a295)
                } else {
                    false
                } {
                    let a297 = match v1.clone() {
                        Adt1::C0(_, _, _, h296, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h296
                        }
                    };
                    let a298 = 4294967295u64;
                    nat_le(a297, a298)
                } else {
                    false
                } {
                    let a300 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h299, _, _, _, _, _, _, _, _, _, _, _) => {
                            h299
                        }
                    };
                    let a301 = 4294967295u64;
                    nat_le(a300, a301)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a304 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h302) => {
                            h302
                        }
                    };
                    let a305 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h303) => {
                            h303
                        }
                    };
                    nat_eq(a304, a305)
                } else {
                    true
                }
            } else {
                false
            } {
                f28(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h306, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h306
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h307, _) => {
                        h307
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f18(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a308 = v10;
                    let a309 = v8;
                    nat_add(a308, a309)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h310, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h310
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a312 = match v0.clone() {
        Adt2::C0(h311, _, _, _, _, _, _, _, _, _, _) => {
            h311
        }
    };
    let a313 = 0u64;
    if if if nat_eq(a312, a313) {
        let a315 = match v1.clone() {
            Adt2::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        };
        let a316 = 0u64;
        nat_eq(a315, a316)
    } else {
        false
    } {
        f21(match v0.clone() {
            Adt2::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                h317
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                h318
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h319, _, _, _, _, _, _, _) => {
                h319
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                h320
            }
        }, match v1.clone() {
            Adt2::C0(_, h321, _, _, _, _, _, _, _, _, _) => {
                h321
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h322, _, _, _, _, _, _, _, _) => {
                h322
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h323, _, _, _, _, _, _, _) => {
                h323
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h324, _, _, _, _, _, _) => {
                h324
            }
        })
    } else {
        false
    } {
        let v2: u64 = f18(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a325 = v2;
            let a326 = f18(v0.clone(), 1u64)?;
            nat_add(a325, a326)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a328 = match v1.clone() {
        Adt2::C0(h327, _, _, _, _, _, _, _, _, _, _) => {
            h327
        }
    };
    let a329 = 0u64;
    if if if nat_eq(a328, a329) {
        true
    } else {
        let a331 = match v1.clone() {
            Adt2::C0(h330, _, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        let a332 = 1u64;
        nat_eq(a331, a332)
    } {
        true
    } else {
        let a334 = match v1.clone() {
            Adt2::C0(h333, _, _, _, _, _, _, _, _, _, _) => {
                h333
            }
        };
        let a335 = 30u64;
        nat_eq(a334, a335)
    } {
        let a338 = match v1.clone() {
            Adt2::C0(h336, _, _, _, _, _, _, _, _, _, _) => {
                h336
            }
        };
        let a339 = match v2.clone() {
            Adt2::C0(h337, _, _, _, _, _, _, _, _, _, _) => {
                h337
            }
        };
        if if if if if nat_eq(a338, a339) {
            let a342 = match v1.clone() {
                Adt2::C0(_, h340, _, _, _, _, _, _, _, _, _) => {
                    h340
                }
            };
            let a343 = match v2.clone() {
                Adt2::C0(_, h341, _, _, _, _, _, _, _, _, _) => {
                    h341
                }
            };
            nat_eq(a342, a343)
        } else {
            false
        } {
            let a346 = match v1.clone() {
                Adt2::C0(_, _, _, h344, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            let a347 = match v2.clone() {
                Adt2::C0(_, _, _, h345, _, _, _, _, _, _, _) => {
                    h345
                }
            };
            nat_eq(a346, a347)
        } else {
            false
        } {
            let a350 = match v1.clone() {
                Adt2::C0(_, _, _, _, h348, _, _, _, _, _, _) => {
                    h348
                }
            };
            let a351 = match v2.clone() {
                Adt2::C0(_, _, _, _, h349, _, _, _, _, _, _) => {
                    h349
                }
            };
            nat_eq(a350, a351)
        } else {
            false
        } {
            let a352 = v0;
            let a353 = 4u64;
            if nat_eq(a352, a353) {
                let a355 = match v1.clone() {
                    Adt2::C0(_, _, h354, _, _, _, _, _, _, _, _) => {
                        h354
                    }
                };
                let a356 = 1u64;
                if nat_eq(a355, a356) {
                    let a359 = match v1.clone() {
                        Adt2::C0(_, h357, _, _, _, _, _, _, _, _, _) => {
                            h357
                        }
                    };
                    let a360 = match v2.clone() {
                        Adt2::C0(_, _, h358, _, _, _, _, _, _, _, _) => {
                            h358
                        }
                    };
                    nat_eq(a359, a360)
                } else {
                    false
                }
            } else {
                let a361 = v0;
                let a362 = 4u64;
                if if nat_lt(a361, a362) {
                    let a365 = match v1.clone() {
                        Adt2::C0(_, h363, _, _, _, _, _, _, _, _, _) => {
                            h363
                        }
                    };
                    let a366 = match v1.clone() {
                        Adt2::C0(_, _, h364, _, _, _, _, _, _, _, _) => {
                            h364
                        }
                    };
                    nat_eq(a365, a366)
                } else {
                    false
                } {
                    let a369 = match v1.clone() {
                        Adt2::C0(_, _, h367, _, _, _, _, _, _, _, _) => {
                            h367
                        }
                    };
                    let a370 = match v2.clone() {
                        Adt2::C0(_, _, h368, _, _, _, _, _, _, _, _) => {
                            h368
                        }
                    };
                    nat_eq(a369, a370)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m372 = f19(match v1.clone() {
                Adt2::C0(h371, _, _, _, _, _, _, _, _, _, _) => {
                    h371
                }
            });
            match m372 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f18(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h373) => {
                            h373
                        }
                    }, f20(v1, 1u64), f20(v2, 1u64), Adt4::C8(v0)))
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
    if if if f29(match v1.clone() {
        Adt2::C0(h374, _, _, _, _, _, _, _, _, _, _) => {
            h374
        }
    }, v0) {
        let a377 = match v1.clone() {
            Adt2::C0(h375, _, _, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        let a378 = match v2.clone() {
            Adt2::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        };
        nat_eq(a377, a378)
    } else {
        false
    } {
        let a381 = match v1.clone() {
            Adt2::C0(_, h379, _, _, _, _, _, _, _, _, _) => {
                h379
            }
        };
        let a382 = match v2.clone() {
            Adt2::C0(_, h380, _, _, _, _, _, _, _, _, _) => {
                h380
            }
        };
        if if if nat_eq(a381, a382) {
            let a385 = match v1.clone() {
                Adt2::C0(_, _, h383, _, _, _, _, _, _, _, _) => {
                    h383
                }
            };
            let a386 = match v2.clone() {
                Adt2::C0(_, _, h384, _, _, _, _, _, _, _, _) => {
                    h384
                }
            };
            nat_eq(a385, a386)
        } else {
            false
        } {
            let a389 = match v1.clone() {
                Adt2::C0(_, _, _, h387, _, _, _, _, _, _, _) => {
                    h387
                }
            };
            let a390 = match v2.clone() {
                Adt2::C0(_, _, _, h388, _, _, _, _, _, _, _) => {
                    h388
                }
            };
            nat_eq(a389, a390)
        } else {
            false
        } {
            let a393 = match v1.clone() {
                Adt2::C0(_, _, _, _, h391, _, _, _, _, _, _) => {
                    h391
                }
            };
            let a394 = match v2.clone() {
                Adt2::C0(_, _, _, _, h392, _, _, _, _, _, _) => {
                    h392
                }
            };
            nat_eq(a393, a394)
        } else {
            false
        }
    } else {
        false
    } {
        let m396 = f19(match v1.clone() {
            Adt2::C0(h395, _, _, _, _, _, _, _, _, _, _) => {
                h395
            }
        });
        match m396 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f18(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h397) => {
                        h397
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h398, _, _, _, _, _, _, _, _, _, _) => {
                        h398
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
            Adt1::C0(h399, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h399
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h400, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h400
            }
        }
    };
    let a401 = 0u64;
    let a402 = v4;
    if if nat_lt(a401, a402) {
        let a403 = v4;
        let a404 = 2147483648u64;
        nat_lt(a403, a404)
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
        let v7: u64 = f30(match v5.clone() {
            Adt2::C0(_, _, h405, _, _, _, _, _, _, _, _) => {
                h405
            }
        }, v4)?;
        let v8: u64 = f30(match v5.clone() {
            Adt2::C0(_, _, _, h406, _, _, _, _, _, _, _) => {
                h406
            }
        }, v4)?;
        let a408 = match v5.clone() {
            Adt2::C0(h407, _, _, _, _, _, _, _, _, _, _) => {
                h407
            }
        };
        let a409 = 0u64;
        if if if if if nat_eq(a408, a409) {
            let a411 = match v6.clone() {
                Adt2::C0(h410, _, _, _, _, _, _, _, _, _, _) => {
                    h410
                }
            };
            let a412 = 0u64;
            nat_eq(a411, a412)
        } else {
            false
        } {
            let a414 = match v5.clone() {
                Adt2::C0(_, _, _, _, h413, _, _, _, _, _, _) => {
                    h413
                }
            };
            let a415 = 1u64;
            nat_eq(a414, a415)
        } else {
            false
        } {
            let a418 = match v5.clone() {
                Adt2::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
                    h416
                }
            };
            let a419 = match v6.clone() {
                Adt2::C0(_, h417, _, _, _, _, _, _, _, _, _) => {
                    h417
                }
            };
            nat_eq(a418, a419)
        } else {
            false
        } {
            if v0 {
                let a421 = match v5.clone() {
                    Adt2::C0(_, _, h420, _, _, _, _, _, _, _, _) => {
                        h420
                    }
                };
                let a422 = 2147483648u64;
                if if if nat_lt(a421, a422) {
                    let a424 = match v5.clone() {
                        Adt2::C0(_, _, _, h423, _, _, _, _, _, _, _) => {
                            h423
                        }
                    };
                    let a425 = 2147483648u64;
                    nat_lt(a424, a425)
                } else {
                    false
                } {
                    let a428 = {
                        let a426 = v7;
                        let a427 = v8;
                        nat_mul(a426, a427)?
                    };
                    let a429 = 2147483648u64;
                    nat_le(a428, a429)
                } else {
                    false
                } {
                    f31(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a431 = match v1.clone() {
                    Adt1::C0(h430, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h430
                    }
                };
                let a432 = v7;
                if if if if nat_eq(a431, a432) {
                    let a434 = match v1 {
                        Adt1::C0(_, h433, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h433
                        }
                    };
                    let a435 = v8;
                    nat_eq(a434, a435)
                } else {
                    false
                } {
                    f32(v7, v8, v4)?
                } else {
                    false
                } {
                    let a437 = match v6.clone() {
                        Adt2::C0(_, _, h436, _, _, _, _, _, _, _, _) => {
                            h436
                        }
                    };
                    let a438 = v4;
                    if nat_eq(a437, a438) {
                        let a440 = match v6.clone() {
                            Adt2::C0(_, _, _, h439, _, _, _, _, _, _, _) => {
                                h439
                            }
                        };
                        let a441 = v4;
                        nat_eq(a440, a441)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a445 = match v6 {
                        Adt2::C0(_, _, _, _, h442, _, _, _, _, _, _) => {
                            h442
                        }
                    };
                    let a446 = {
                        let a443 = v7;
                        let a444 = v8;
                        nat_mul(a443, a444)?
                    };
                    nat_eq(a445, a446)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f18(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a448 = match v2.clone() {
        Adt2::C0(h447, _, _, _, _, _, _, _, _, _, _) => {
            h447
        }
    };
    let a449 = 0u64;
    if if if if if nat_eq(a448, a449) {
        true
    } else {
        let a451 = match v2.clone() {
            Adt2::C0(h450, _, _, _, _, _, _, _, _, _, _) => {
                h450
            }
        };
        let a452 = 1u64;
        nat_eq(a451, a452)
    } {
        true
    } else {
        let a454 = match v2.clone() {
            Adt2::C0(h453, _, _, _, _, _, _, _, _, _, _) => {
                h453
            }
        };
        let a455 = 30u64;
        nat_eq(a454, a455)
    } {
        let a458 = match v2.clone() {
            Adt2::C0(h456, _, _, _, _, _, _, _, _, _, _) => {
                h456
            }
        };
        let a459 = match v3.clone() {
            Adt2::C0(h457, _, _, _, _, _, _, _, _, _, _) => {
                h457
            }
        };
        nat_eq(a458, a459)
    } else {
        false
    } {
        if if f26(v2.clone(), v3.clone()) {
            let a461 = match v1.clone() {
                Adt1::C0(h460, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h460
                }
            };
            let a462 = 4294967296u64;
            nat_lt(a461, a462)
        } else {
            false
        } {
            let a464 = match v1.clone() {
                Adt1::C0(_, h463, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h463
                }
            };
            let a465 = 4294967296u64;
            nat_lt(a464, a465)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f18(v3.clone(), 1u64)?, {
            let a467 = match v2.clone() {
                Adt2::C0(h466, _, _, _, _, _, _, _, _, _, _) => {
                    h466
                }
            };
            let a468 = 0u64;
            if nat_eq(a467, a468) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a469 = v0;
            let a470 = 31u64;
            if nat_eq(a469, a470) {
                0u64
            } else {
                let a471 = v0;
                let a472 = 47u64;
                if nat_eq(a471, a472) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h473, _, _, _, _, _, _, _, _, _, _) => {
                h473
            }
        }, match v1.clone() {
            Adt1::C0(h474, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h474
            }
        }, match v1 {
            Adt1::C0(_, h475, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h475
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f12(v0: u64, v1: u64) -> Option<u64> {
    let a476 = v0;
    let a477 = 8u64;
    if nat_eq(a476, a477) {
        Some(5u64)
    } else {
        let a478 = v0;
        let a479 = 9u64;
        if nat_eq(a478, a479) {
            Some(6u64)
        } else {
            let a480 = v0;
            let a481 = 89u64;
            if nat_eq(a480, a481) {
                let a482 = v1;
                let a483 = 0u64;
                if nat_eq(a482, a483) {
                    Some(0u64)
                } else {
                    let a484 = v1;
                    let a485 = 1u64;
                    if nat_eq(a484, a485) {
                        Some(1u64)
                    } else {
                        let a486 = v1;
                        let a487 = 2u64;
                        if nat_eq(a486, a487) {
                            Some(2u64)
                        } else {
                            let a488 = v1;
                            let a489 = 3u64;
                            if nat_eq(a488, a489) {
                                Some(3u64)
                            } else {
                                let a490 = v1;
                                let a491 = 6u64;
                                if nat_eq(a490, a491) {
                                    Some(4u64)
                                } else {
                                    let a492 = v1;
                                    let a493 = 12u64;
                                    if nat_eq(a492, a493) {
                                        Some(7u64)
                                    } else {
                                        let a494 = v1;
                                        let a495 = 11u64;
                                        if nat_eq(a494, a495) {
                                            Some(8u64)
                                        } else {
                                            let a496 = v1;
                                            let a497 = 18u64;
                                            if nat_eq(a496, a497) {
                                                Some(9u64)
                                            } else {
                                                let a498 = v1;
                                                let a499 = 19u64;
                                                if nat_eq(a498, a499) {
                                                    Some(10u64)
                                                } else {
                                                    let a500 = v1;
                                                    let a501 = 20u64;
                                                    if nat_eq(a500, a501) {
                                                        Some(11u64)
                                                    } else {
                                                        let a502 = v1;
                                                        let a503 = 21u64;
                                                        if nat_eq(a502, a503) {
                                                            Some(12u64)
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
                        }
                    }
                }
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m504 = f33(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m504 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v5, v6, {
                let m505 = v3;
                match m505 {
                    None => {
                        f20(v2, 1u64)
                    }
                    Some(v12) => {
                        f20(v12, 1u64)
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

pub fn f14(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a507 = match v1.clone() {
        Adt2::C0(h506, _, _, _, _, _, _, _, _, _, _) => {
            h506
        }
    };
    let a508 = 0u64;
    if if if if if nat_eq(a507, a508) {
        true
    } else {
        let a510 = match v1.clone() {
            Adt2::C0(h509, _, _, _, _, _, _, _, _, _, _) => {
                h509
            }
        };
        let a511 = 1u64;
        nat_eq(a510, a511)
    } {
        true
    } else {
        let a513 = match v1.clone() {
            Adt2::C0(h512, _, _, _, _, _, _, _, _, _, _) => {
                h512
            }
        };
        let a514 = 30u64;
        nat_eq(a513, a514)
    } {
        let a517 = match v1.clone() {
            Adt2::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
                h515
            }
        };
        let a518 = match v2.clone() {
            Adt2::C0(h516, _, _, _, _, _, _, _, _, _, _) => {
                h516
            }
        };
        if nat_eq(a517, a518) {
            true
        } else {
            let a520 = match v2.clone() {
                Adt2::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
                    h519
                }
            };
            let a521 = 0u64;
            nat_eq(a520, a521)
        }
    } else {
        false
    } {
        f26(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f18(v2.clone(), 1u64)?, {
            let a523 = match v2.clone() {
                Adt2::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                    h522
                }
            };
            let a524 = 0u64;
            if nat_eq(a523, a524) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                h525
            }
        }, match v2 {
            Adt2::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                h526
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f15(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m527 = v3;
    match m527 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f34(v0, v1)?, f34(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f35(v0, v1, v2.clone())?, f34(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f36(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f37(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f38(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f37(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f39(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f40(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f41(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f34(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f42(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f34(v0, v1)?, f34(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f43(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f34(v0, v1)?, f34(v0, v2)?))
        }
        Adt4::C14(_, v36, v37) => {
            Ok(Adt5::C3(v36, v37, f34(v0, v1)?, f34(v0, v2)?))
        }
    }
}

pub fn f16(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a530 = match v1.clone() {
        Adt2::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
            h528
        }
    };
    let a531 = match v3.clone() {
        Adt2::C0(h529, _, _, _, _, _, _, _, _, _, _) => {
            h529
        }
    };
    if if nat_eq(a530, a531) {
        let a532 = v0;
        let a533 = 33u64;
        if nat_eq(a532, a533) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m535 = f19(match v1.clone() {
            Adt2::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                h534
            }
        });
        match m535 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h536, _) => {
                        h536
                    }
                };
                let v6: u64 = f18(v1.clone(), v5)?;
                let a541 = {
                    let a538 = match v1.clone() {
                        Adt2::C0(_, h537, _, _, _, _, _, _, _, _, _) => {
                            h537
                        }
                    };
                    let a539 = v5;
                    let a540 = 1u64;
                    nat_rem(a538, a539, a540)
                };
                let a542 = 0u64;
                if if if nat_eq(a541, a542) {
                    let a547 = {
                        let a544 = match v3.clone() {
                            Adt2::C0(_, h543, _, _, _, _, _, _, _, _, _) => {
                                h543
                            }
                        };
                        let a545 = v5;
                        let a546 = 1u64;
                        nat_rem(a544, a545, a546)
                    };
                    let a548 = 0u64;
                    nat_eq(a547, a548)
                } else {
                    false
                } {
                    let a549 = v6;
                    let a550 = f18(v3.clone(), v5)?;
                    nat_eq(a549, a550)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h551) => {
                            h551
                        }
                    }, f20(v1, v5), f20(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f17(v0: u64) -> bool {
    let a552 = v0;
    let a553 = 0u64;
    if if nat_eq(a552, a553) {
        true
    } else {
        let a554 = v0;
        let a555 = 1u64;
        nat_eq(a554, a555)
    } {
        true
    } else {
        let a556 = v0;
        let a557 = 30u64;
        nat_eq(a556, a557)
    }
}

pub fn f18(v0: Adt2, v1: u64) -> R<u64> {
    let a569 = {
        let a563 = {
            let a559 = match v0.clone() {
                Adt2::C0(_, h558, _, _, _, _, _, _, _, _, _) => {
                    h558
                }
            };
            let a560 = v1;
            let a561 = 0u64;
            nat_quot(a559, a560, a561)
        };
        let a564 = match v0.clone() {
            Adt2::C0(_, _, h562, _, _, _, _, _, _, _, _) => {
                h562
            }
        };
        nat_mul(a563, a564)?
    };
    let a570 = {
        let a567 = match v0.clone() {
            Adt2::C0(_, _, _, h565, _, _, _, _, _, _, _) => {
                h565
            }
        };
        let a568 = match v0 {
            Adt2::C0(_, _, _, _, h566, _, _, _, _, _, _) => {
                h566
            }
        };
        nat_mul(a567, a568)?
    };
    nat_mul(a569, a570)
}

pub fn f19(v0: u64) -> Adt7 {
    let a571 = v0;
    let a572 = 0u64;
    if nat_eq(a571, a572) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a573 = v0;
        let a574 = 1u64;
        if nat_eq(a573, a574) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a575 = v0;
            let a576 = 2u64;
            if nat_eq(a575, a576) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a577 = v0;
                let a578 = 3u64;
                if nat_eq(a577, a578) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a579 = v0;
                    let a580 = 6u64;
                    if nat_eq(a579, a580) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a581 = v0;
                        let a582 = 7u64;
                        if nat_eq(a581, a582) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a583 = v0;
                            let a584 = 8u64;
                            if nat_eq(a583, a584) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a585 = v0;
                                let a586 = 9u64;
                                if nat_eq(a585, a586) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a587 = v0;
                                    let a588 = 10u64;
                                    if nat_eq(a587, a588) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a589 = v0;
                                        let a590 = 11u64;
                                        if nat_eq(a589, a590) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a591 = v0;
                                            let a592 = 12u64;
                                            if nat_eq(a591, a592) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a593 = v0;
                                                let a594 = 13u64;
                                                if nat_eq(a593, a594) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a595 = v0;
                                                    let a596 = 14u64;
                                                    if nat_eq(a595, a596) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a597 = v0;
                                                        let a598 = 15u64;
                                                        if nat_eq(a597, a598) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a599 = v0;
                                                            let a600 = 16u64;
                                                            if nat_eq(a599, a600) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a601 = v0;
                                                                let a602 = 17u64;
                                                                if nat_eq(a601, a602) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a603 = v0;
                                                                    let a604 = 18u64;
                                                                    if nat_eq(a603, a604) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a605 = v0;
                                                                        let a606 = 19u64;
                                                                        if nat_eq(a605, a606) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a607 = v0;
                                                                            let a608 = 20u64;
                                                                            if nat_eq(a607, a608) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a609 = v0;
                                                                                let a610 = 21u64;
                                                                                if nat_eq(a609, a610) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a611 = v0;
                                                                                    let a612 = 22u64;
                                                                                    if nat_eq(a611, a612) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a613 = v0;
                                                                                        let a614 = 23u64;
                                                                                        if nat_eq(a613, a614) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a615 = v0;
                                                                                            let a616 = 24u64;
                                                                                            if nat_eq(a615, a616) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a617 = v0;
                                                                                                let a618 = 25u64;
                                                                                                if nat_eq(a617, a618) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a619 = v0;
                                                                                                    let a620 = 26u64;
                                                                                                    if nat_eq(a619, a620) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a621 = v0;
                                                                                                        let a622 = 27u64;
                                                                                                        if nat_eq(a621, a622) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a623 = v0;
                                                                                                            let a624 = 28u64;
                                                                                                            if nat_eq(a623, a624) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a625 = v0;
                                                                                                                let a626 = 29u64;
                                                                                                                if nat_eq(a625, a626) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a627 = v0;
                                                                                                                    let a628 = 30u64;
                                                                                                                    if nat_eq(a627, a628) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a629 = v0;
                                                                                                                        let a630 = 34u64;
                                                                                                                        if nat_eq(a629, a630) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a631 = v0;
                                                                                                                            let a632 = 35u64;
                                                                                                                            if nat_eq(a631, a632) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a633 = v0;
                                                                                                                                let a634 = 39u64;
                                                                                                                                if nat_eq(a633, a634) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a635 = v0;
                                                                                                                                    let a636 = 40u64;
                                                                                                                                    if nat_eq(a635, a636) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a637 = v0;
                                                                                                                                        let a638 = 41u64;
                                                                                                                                        if nat_eq(a637, a638) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a639 = v0;
                                                                                                                                            let a640 = 42u64;
                                                                                                                                            if nat_eq(a639, a640) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a641 = v0;
                                                                                                                                                let a642 = 43u64;
                                                                                                                                                if nat_eq(a641, a642) {
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

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a644 = match v0.clone() {
            Adt2::C0(_, h643, _, _, _, _, _, _, _, _, _) => {
                h643
            }
        };
        let a645 = v1;
        let a646 = 0u64;
        nat_quot(a644, a645, a646)
    }, match v0.clone() {
        Adt2::C0(_, _, h647, _, _, _, _, _, _, _, _) => {
            h647
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h648, _, _, _, _, _, _, _) => {
            h648
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h649, _, _, _, _, _, _) => {
            h649
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h650, _, _, _, _, _) => {
            h650
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h651, _, _, _, _) => {
            h651
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h652, _, _, _) => {
            h652
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h653, _, _) => {
            h653
        }
    })
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f24(v4, v5, v6, v7) {
        f24(v0, v1, v2, v3)
    } else {
        let a657 = {
            let a654 = v0;
            let a655 = v4;
            let a656 = 1u64;
            nat_rem(a654, a655, a656)
        };
        let a658 = 0u64;
        if if if nat_eq(a657, a658) {
            let a662 = {
                let a659 = v1;
                let a660 = v5;
                let a661 = 1u64;
                nat_rem(a659, a660, a661)
            };
            let a663 = 0u64;
            nat_eq(a662, a663)
        } else {
            false
        } {
            let a667 = {
                let a664 = v2;
                let a665 = v6;
                let a666 = 1u64;
                nat_rem(a664, a665, a666)
            };
            let a668 = 0u64;
            nat_eq(a667, a668)
        } else {
            false
        } {
            let a672 = {
                let a669 = v3;
                let a670 = v7;
                let a671 = 1u64;
                nat_rem(a669, a670, a671)
            };
            let a673 = 0u64;
            nat_eq(a672, a673)
        } else {
            false
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a674 = v0;
    let a675 = v1;
    if nat_eq(a674, a675) {
        let a678 = v4;
        let a679 = {
            let a676 = v2;
            let a677 = v3;
            nat_add(a676, a677)?
        };
        Ok(nat_eq(a678, a679))
    } else {
        let a680 = v2;
        let a681 = v3;
        if nat_eq(a680, a681) {
            let a682 = v4;
            let a683 = v2;
            Ok(nat_eq(a682, a683))
        } else {
            Ok(false)
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a686 = {
            let a684 = v0;
            let a685 = f44(v1);
            nat_add(a684, a685)?
        };
        let a687 = f44(v2);
        nat_add(a686, a687)?
    };
    let v5: u64 = {
        let a688 = f45(v1);
        let a689 = f45(v2);
        nat_add(a688, a689)?
    };
    let a690 = v1;
    let a691 = 4294967295u64;
    if if nat_le(a690, a691) {
        let a692 = v2;
        let a693 = 4294967295u64;
        nat_le(a692, a693)
    } else {
        false
    } {
        let a694 = v5;
        let a695 = v4;
        if nat_le(a694, a695) {
            let a698 = v3;
            let a699 = {
                let a696 = v4;
                let a697 = v5;
                nat_sub(a696, a697)
            };
            Ok(nat_eq(a698, a699))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a700 = v0;
    let a701 = 0u64;
    if if if nat_eq(a700, a701) {
        true
    } else {
        let a702 = v1;
        let a703 = 0u64;
        nat_eq(a702, a703)
    } {
        true
    } else {
        let a704 = v2;
        let a705 = 0u64;
        nat_eq(a704, a705)
    } {
        true
    } else {
        let a706 = v3;
        let a707 = 0u64;
        nat_eq(a706, a707)
    }
}

pub fn f25(v0: u64, v1: u64) -> R<bool> {
    let a708 = v1;
    let a709 = 4294967295u64;
    if nat_le(a708, a709) {
        let a712 = {
            let a710 = f44(v1);
            let a711 = f45(v1);
            nat_add(a710, a711)?
        };
        let a713 = v0;
        Ok(nat_lt(a712, a713))
    } else {
        Ok(false)
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    let a716 = match v0.clone() {
        Adt2::C0(_, h714, _, _, _, _, _, _, _, _, _) => {
            h714
        }
    };
    let a717 = match v1.clone() {
        Adt2::C0(_, h715, _, _, _, _, _, _, _, _, _) => {
            h715
        }
    };
    if if if nat_eq(a716, a717) {
        let a720 = match v0.clone() {
            Adt2::C0(_, _, h718, _, _, _, _, _, _, _, _) => {
                h718
            }
        };
        let a721 = match v1.clone() {
            Adt2::C0(_, _, h719, _, _, _, _, _, _, _, _) => {
                h719
            }
        };
        nat_eq(a720, a721)
    } else {
        false
    } {
        let a724 = match v0.clone() {
            Adt2::C0(_, _, _, h722, _, _, _, _, _, _, _) => {
                h722
            }
        };
        let a725 = match v1.clone() {
            Adt2::C0(_, _, _, h723, _, _, _, _, _, _, _) => {
                h723
            }
        };
        nat_eq(a724, a725)
    } else {
        false
    } {
        let a728 = match v0 {
            Adt2::C0(_, _, _, _, h726, _, _, _, _, _, _) => {
                h726
            }
        };
        let a729 = match v1 {
            Adt2::C0(_, _, _, _, h727, _, _, _, _, _, _) => {
                h727
            }
        };
        nat_eq(a728, a729)
    } else {
        false
    }
}

pub fn f27(v0: Adt2) -> R<bool> {
    let a731 = match v0.clone() {
        Adt2::C0(_, h730, _, _, _, _, _, _, _, _, _) => {
            h730
        }
    };
    let a732 = 1u64;
    if if if if nat_eq(a731, a732) {
        true
    } else {
        let a734 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h733, _, _, _, _, _) => {
                h733
            }
        };
        let a735 = 4u64;
        nat_eq(a734, a735)
    } {
        let a737 = match v0.clone() {
            Adt2::C0(_, _, h736, _, _, _, _, _, _, _, _) => {
                h736
            }
        };
        let a738 = 1u64;
        if nat_eq(a737, a738) {
            true
        } else {
            let a743 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h739, _, _, _, _) => {
                    h739
                }
            };
            let a744 = {
                let a741 = match v0.clone() {
                    Adt2::C0(_, h740, _, _, _, _, _, _, _, _, _) => {
                        h740
                    }
                };
                let a742 = 4u64;
                nat_mul(a741, a742)?
            };
            nat_eq(a743, a744)
        }
    } else {
        false
    } {
        let a746 = match v0.clone() {
            Adt2::C0(_, _, _, h745, _, _, _, _, _, _, _) => {
                h745
            }
        };
        let a747 = 1u64;
        if nat_eq(a746, a747) {
            true
        } else {
            let a755 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h748, _, _, _) => {
                    h748
                }
            };
            let a756 = {
                let a753 = {
                    let a751 = match v0.clone() {
                        Adt2::C0(_, h749, _, _, _, _, _, _, _, _, _) => {
                            h749
                        }
                    };
                    let a752 = match v0.clone() {
                        Adt2::C0(_, _, h750, _, _, _, _, _, _, _, _) => {
                            h750
                        }
                    };
                    nat_mul(a751, a752)?
                };
                let a754 = 4u64;
                nat_mul(a753, a754)?
            };
            nat_eq(a755, a756)
        }
    } else {
        false
    } {
        let a758 = match v0.clone() {
            Adt2::C0(_, _, _, _, h757, _, _, _, _, _, _) => {
                h757
            }
        };
        let a759 = 1u64;
        if nat_eq(a758, a759) {
            Ok(true)
        } else {
            let a770 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h760, _, _) => {
                    h760
                }
            };
            let a771 = {
                let a768 = {
                    let a766 = {
                        let a763 = match v0.clone() {
                            Adt2::C0(_, h761, _, _, _, _, _, _, _, _, _) => {
                                h761
                            }
                        };
                        let a764 = match v0.clone() {
                            Adt2::C0(_, _, h762, _, _, _, _, _, _, _, _) => {
                                h762
                            }
                        };
                        nat_mul(a763, a764)?
                    };
                    let a767 = match v0 {
                        Adt2::C0(_, _, _, h765, _, _, _, _, _, _, _) => {
                            h765
                        }
                    };
                    nat_mul(a766, a767)?
                };
                let a769 = 4u64;
                nat_mul(a768, a769)?
            };
            Ok(nat_eq(a770, a771))
        }
    } else {
        Ok(false)
    }
}

pub fn f28(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f46(v0.clone(), false)?;
    let v6: u64 = f46(v0, true)?;
    let a772 = v6;
    let a773 = v1;
    if if nat_le(a772, a773) {
        let a780 = {
            let a778 = {
                let a774 = v1;
                let a775 = v5;
                nat_add(a774, a775)?
            };
            let a779 = {
                let a776 = v3;
                let a777 = 0u64;
                if nat_eq(a776, a777) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a778, a779)?
        };
        let a781 = v2;
        nat_le(a780, a781)
    } else {
        false
    } {
        if v4 {
            let a786 = {
                let a784 = {
                    let a782 = v1;
                    let a783 = v5;
                    nat_add(a782, a783)?
                };
                let a785 = v6;
                nat_sub(a784, a785)
            };
            let a787 = v2;
            Ok(nat_lt(a786, a787))
        } else {
            let a792 = {
                let a790 = {
                    let a788 = v1;
                    let a789 = v5;
                    nat_add(a788, a789)?
                };
                let a791 = v6;
                nat_sub(a790, a791)
            };
            let a793 = v2;
            Ok(nat_le(a792, a793))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: u64, v1: u64) -> bool {
    let a794 = v0;
    let a795 = 0u64;
    if if nat_eq(a794, a795) {
        true
    } else {
        let a796 = v0;
        let a797 = 1u64;
        nat_eq(a796, a797)
    } {
        let a798 = v1;
        let a799 = 4294967296u64;
        nat_lt(a798, a799)
    } else {
        false
    }
}

pub fn f30(v0: u64, v1: u64) -> R<u64> {
    let a804 = {
        let a802 = v0;
        let a803 = {
            let a800 = v1;
            let a801 = 1u64;
            nat_sub(a800, a801)
        };
        nat_add(a802, a803)?
    };
    let a805 = v1;
    let a806 = 0u64;
    Ok(nat_quot(a804, a805, a806))
}

pub fn f31(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f24(match v0.clone() {
        Adt2::C0(_, h807, _, _, _, _, _, _, _, _, _) => {
            h807
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h808, _, _, _, _, _, _, _, _) => {
            h808
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h809, _, _, _, _, _, _, _) => {
            h809
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h810, _, _, _, _, _, _) => {
            h810
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a813 = match v1.clone() {
                Adt2::C0(_, h811, _, _, _, _, _, _, _, _, _) => {
                    h811
                }
            };
            let a814 = match v1.clone() {
                Adt2::C0(_, _, h812, _, _, _, _, _, _, _, _) => {
                    h812
                }
            };
            nat_mul(a813, a814)?
        };
        let v5: u64 = {
            let a816 = v4;
            let a817 = match v1.clone() {
                Adt2::C0(_, _, _, h815, _, _, _, _, _, _, _) => {
                    h815
                }
            };
            nat_mul(a816, a817)?
        };
        let a830 = {
            let a828 = {
                let a819 = match v0.clone() {
                    Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                        h818
                    }
                };
                let a820 = 1u64;
                nat_sub(a819, a820)
            };
            let a829 = {
                let a826 = f47(match v0.clone() {
                    Adt2::C0(_, _, h821, _, _, _, _, _, _, _, _) => {
                        h821
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h822, _, _, _, _, _, _, _, _, _) => {
                        h822
                    }
                })?;
                let a827 = f47(match v0 {
                    Adt2::C0(_, _, _, h823, _, _, _, _, _, _, _) => {
                        h823
                    }
                }, v2, {
                    let a824 = v3;
                    let a825 = v5;
                    nat_mul(a824, a825)?
                }, v4)?;
                nat_add(a826, a827)?
            };
            nat_add(a828, a829)?
        };
        let a831 = f18(v1, 1u64)?;
        Ok(nat_lt(a830, a831))
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a832 = v0;
    let a833 = 2147483648u64;
    if if if if nat_lt(a832, a833) {
        let a834 = v1;
        let a835 = 2147483648u64;
        nat_lt(a834, a835)
    } else {
        false
    } {
        let a838 = {
            let a836 = v0;
            let a837 = v1;
            nat_mul(a836, a837)?
        };
        let a839 = 2147483648u64;
        nat_lt(a838, a839)
    } else {
        false
    } {
        let a844 = {
            let a842 = {
                let a840 = v0;
                let a841 = 1u64;
                nat_sub(a840, a841)
            };
            let a843 = v2;
            nat_mul(a842, a843)?
        };
        let a845 = 2147483648u64;
        nat_lt(a844, a845)
    } else {
        false
    } {
        let a850 = {
            let a848 = {
                let a846 = v1;
                let a847 = 1u64;
                nat_sub(a846, a847)
            };
            let a849 = v2;
            nat_mul(a848, a849)?
        };
        let a851 = 2147483648u64;
        Ok(nat_lt(a850, a851))
    } else {
        Ok(false)
    }
}

pub fn f33(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m852 = v3;
    match m852 {
        None => {
            f48(v0, v1, match v2.clone() {
                Adt2::C0(h853, _, _, _, _, _, _, _, _, _, _) => {
                    h853
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h854, _, _, _, _, _, _, _, _, _, _) => {
                    h854
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h855, _, _, _, _, _, _, _, _, _) => {
                    h855
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h856, _, _, _, _, _, _, _, _) => {
                    h856
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h857, _, _, _, _, _, _, _) => {
                    h857
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h858, _, _, _, _, _, _) => {
                    h858
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h859, _, _, _, _, _, _, _, _, _) => {
                    h859
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h860, _, _, _, _, _, _, _, _) => {
                    h860
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h861, _, _, _, _, _, _, _) => {
                    h861
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h862, _, _, _, _, _, _) => {
                    h862
                }
            })
        }
        Some(v5) => {
            f48(v0, v1, match v2.clone() {
                Adt2::C0(h863, _, _, _, _, _, _, _, _, _, _) => {
                    h863
                }
            }, match v5.clone() {
                Adt2::C0(h864, _, _, _, _, _, _, _, _, _, _) => {
                    h864
                }
            }, match v4.clone() {
                Adt2::C0(h865, _, _, _, _, _, _, _, _, _, _) => {
                    h865
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h866, _, _, _, _, _, _, _, _, _) => {
                    h866
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h867, _, _, _, _, _, _, _, _) => {
                    h867
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h868, _, _, _, _, _, _, _) => {
                    h868
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h869, _, _, _, _, _, _) => {
                    h869
                }
            }, match v5.clone() {
                Adt2::C0(_, h870, _, _, _, _, _, _, _, _, _) => {
                    h870
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h871, _, _, _, _, _, _, _, _) => {
                    h871
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h872, _, _, _, _, _, _, _) => {
                    h872
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h873, _, _, _, _, _, _) => {
                    h873
                }
            }, match v4.clone() {
                Adt2::C0(_, h874, _, _, _, _, _, _, _, _, _) => {
                    h874
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h875, _, _, _, _, _, _, _, _) => {
                    h875
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h876, _, _, _, _, _, _, _) => {
                    h876
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h877, _, _, _, _, _, _) => {
                    h877
                }
            })
        }
    }
}

pub fn f34(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a879 = v0;
        let a880 = match v1.clone() {
            Adt3::C0(h878, _, _, _, _, _, _, _) => {
                h878
            }
        };
        let a881 = 0u64;
        nat_quot(a879, a880, a881)
    };
    let v3: u64 = {
        let a883 = v2;
        let a884 = match v1.clone() {
            Adt3::C0(_, h882, _, _, _, _, _, _) => {
                h882
            }
        };
        let a885 = 0u64;
        nat_quot(a883, a884, a885)
    };
    f49({
        let a887 = v0;
        let a888 = match v1.clone() {
            Adt3::C0(h886, _, _, _, _, _, _, _) => {
                h886
            }
        };
        let a889 = 0u64;
        nat_rem(a887, a888, a889)
    }, {
        let a891 = v2;
        let a892 = match v1.clone() {
            Adt3::C0(_, h890, _, _, _, _, _, _) => {
                h890
            }
        };
        let a893 = 0u64;
        nat_rem(a891, a892, a893)
    }, {
        let a895 = v3;
        let a896 = match v1.clone() {
            Adt3::C0(_, _, h894, _, _, _, _, _) => {
                h894
            }
        };
        let a897 = 0u64;
        nat_rem(a895, a896, a897)
    }, {
        let a899 = v3;
        let a900 = match v1.clone() {
            Adt3::C0(_, _, h898, _, _, _, _, _) => {
                h898
            }
        };
        let a901 = 0u64;
        nat_quot(a899, a900, a901)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h902, _, _, _) => {
            h902
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h903, _, _) => {
            h903
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h904, _) => {
            h904
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h905) => {
            h905
        }
    })
}

pub fn f35(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a907 = v0;
        let a908 = match v2.clone() {
            Adt3::C0(h906, _, _, _, _, _, _, _) => {
                h906
            }
        };
        let a909 = 0u64;
        nat_quot(a907, a908, a909)
    };
    let v4: u64 = {
        let a911 = v3;
        let a912 = match v2.clone() {
            Adt3::C0(_, h910, _, _, _, _, _, _) => {
                h910
            }
        };
        let a913 = 0u64;
        nat_quot(a911, a912, a913)
    };
    f50({
        let a915 = v0;
        let a916 = match v2.clone() {
            Adt3::C0(h914, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a917 = 0u64;
        nat_rem(a915, a916, a917)
    }, {
        let a919 = v3;
        let a920 = match v2.clone() {
            Adt3::C0(_, h918, _, _, _, _, _, _) => {
                h918
            }
        };
        let a921 = 0u64;
        nat_rem(a919, a920, a921)
    }, {
        let a923 = v4;
        let a924 = match v2.clone() {
            Adt3::C0(_, _, h922, _, _, _, _, _) => {
                h922
            }
        };
        let a925 = 0u64;
        nat_rem(a923, a924, a925)
    }, {
        let a927 = v4;
        let a928 = match v2 {
            Adt3::C0(_, _, h926, _, _, _, _, _) => {
                h926
            }
        };
        let a929 = 0u64;
        nat_quot(a927, a928, a929)
    }, match v1.clone() {
        Adt3::C0(h930, _, _, _, _, _, _, _) => {
            h930
        }
    }, match v1.clone() {
        Adt3::C0(_, h931, _, _, _, _, _, _) => {
            h931
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h932, _, _, _, _, _) => {
            h932
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h933, _, _, _, _) => {
            h933
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h934, _, _, _) => {
            h934
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h935, _, _) => {
            h935
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h936, _) => {
            h936
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h937) => {
            h937
        }
    })
}

pub fn f36(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a939 = v0;
        let a940 = match v3.clone() {
            Adt3::C0(h938, _, _, _, _, _, _, _) => {
                h938
            }
        };
        let a941 = 0u64;
        nat_quot(a939, a940, a941)
    };
    let v6: u64 = {
        let a943 = v5;
        let a944 = match v3.clone() {
            Adt3::C0(_, h942, _, _, _, _, _, _) => {
                h942
            }
        };
        let a945 = 0u64;
        nat_quot(a943, a944, a945)
    };
    let v7: u64 = {
        let a947 = v0;
        let a948 = match v3.clone() {
            Adt3::C0(h946, _, _, _, _, _, _, _) => {
                h946
            }
        };
        let a949 = 0u64;
        nat_rem(a947, a948, a949)
    };
    let v8: u64 = {
        let a951 = v5;
        let a952 = match v3.clone() {
            Adt3::C0(_, h950, _, _, _, _, _, _) => {
                h950
            }
        };
        let a953 = 0u64;
        nat_rem(a951, a952, a953)
    };
    let v9: u64 = {
        let a955 = v6;
        let a956 = match v3.clone() {
            Adt3::C0(_, _, h954, _, _, _, _, _) => {
                h954
            }
        };
        let a957 = 0u64;
        nat_rem(a955, a956, a957)
    };
    let v10: u64 = {
        let a959 = v6;
        let a960 = match v3.clone() {
            Adt3::C0(_, _, h958, _, _, _, _, _) => {
                h958
            }
        };
        let a961 = 0u64;
        nat_quot(a959, a960, a961)
    };
    let v11: u64 = {
        let a962 = v4;
        let a963 = 0u64;
        if nat_eq(a962, a963) {
            match v1.clone() {
                Adt3::C0(h964, _, _, _, _, _, _, _) => {
                    h964
                }
            }
        } else {
            let a965 = v4;
            let a966 = 1u64;
            if nat_eq(a965, a966) {
                match v1.clone() {
                    Adt3::C0(_, h967, _, _, _, _, _, _) => {
                        h967
                    }
                }
            } else {
                let a968 = v4;
                let a969 = 2u64;
                if nat_eq(a968, a969) {
                    match v1.clone() {
                        Adt3::C0(_, _, h970, _, _, _, _, _) => {
                            h970
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h971, _, _, _, _) => {
                            h971
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a978 = v11;
        let a979 = {
            let a972 = v4;
            let a973 = 0u64;
            if nat_eq(a972, a973) {
                v7
            } else {
                let a974 = v4;
                let a975 = 1u64;
                if nat_eq(a974, a975) {
                    v8
                } else {
                    let a976 = v4;
                    let a977 = 2u64;
                    if nat_eq(a976, a977) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a978, a979)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f49(if if v12 {
        let a980 = v4;
        let a981 = 0u64;
        nat_eq(a980, a981)
    } else {
        false
    } {
        let a982 = v7;
        let a983 = v11;
        nat_sub(a982, a983)
    } else {
        v7
    }, if if v12 {
        let a984 = v4;
        let a985 = 1u64;
        nat_eq(a984, a985)
    } else {
        false
    } {
        let a986 = v8;
        let a987 = v11;
        nat_sub(a986, a987)
    } else {
        v8
    }, if if v12 {
        let a988 = v4;
        let a989 = 2u64;
        nat_eq(a988, a989)
    } else {
        false
    } {
        let a990 = v9;
        let a991 = v11;
        nat_sub(a990, a991)
    } else {
        v9
    }, if if v12 {
        let a992 = v4;
        let a993 = 3u64;
        nat_eq(a992, a993)
    } else {
        false
    } {
        let a994 = v10;
        let a995 = v11;
        nat_sub(a994, a995)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h996, _, _, _) => {
            h996
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h997, _, _) => {
            h997
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h998, _) => {
            h998
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h999) => {
            h999
        }
    })?, f34(v0, v3)?))
}

pub fn f37(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f51(v0, v2.clone(), 0u64);
    let v9: u64 = f51(v0, v2.clone(), 1u64);
    let v10: u64 = f51(v0, v2.clone(), 2u64);
    let v11: u64 = f51(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f52(v1.clone(), f53(v8, v3, match v1.clone() {
            Adt3::C0(h1000, _, _, _, _, _, _, _) => {
                h1000
            }
        })?, f53(v9, v4, match v1.clone() {
            Adt3::C0(_, h1001, _, _, _, _, _, _) => {
                h1001
            }
        })?, f53(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1002, _, _, _, _, _) => {
                h1002
            }
        })?, f53(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1003, _, _, _, _) => {
                h1003
            }
        })?)?, f34(v0, v2)?))
    } else {
        if if if if f54(v8, v3, match v1.clone() {
            Adt3::C0(h1004, _, _, _, _, _, _, _) => {
                h1004
            }
        })? {
            f54(v9, v4, match v1.clone() {
                Adt3::C0(_, h1005, _, _, _, _, _, _) => {
                    h1005
                }
            })?
        } else {
            false
        } {
            f54(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1006, _, _, _, _, _) => {
                    h1006
                }
            })?
        } else {
            false
        } {
            f54(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1007, _, _, _, _) => {
                    h1007
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f52(v1, f55(v8, v3)?, f55(v9, v4)?, f55(v10, v5)?, f55(v11, v6)?)?, f34(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f34(v0, v2)?))
        }
    }
}

pub fn f38(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f51(v0, v2.clone(), 0u64);
    let v5: u64 = f51(v0, v2.clone(), 1u64);
    let v6: u64 = f51(v0, v2.clone(), 2u64);
    let v7: u64 = f51(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f52(v1.clone(), f56(v4, v3, match v1 {
        Adt3::C0(h1008, _, _, _, _, _, _, _) => {
            h1008
        }
    })?, v5, v6, v7)?, f34(v0, v2)?))
}

pub fn f39(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1009 = v0;
    let a1010 = v3;
    if nat_lt(a1009, a1010) {
        Ok(Adt5::C1(false, {
            let a1011 = v0;
            let a1012 = 4u64;
            nat_mul(a1011, a1012)?
        }, {
            let a1013 = v0;
            let a1014 = 4u64;
            nat_mul(a1013, a1014)?
        }))
    } else {
        let v6: u64 = {
            let a1015 = v0;
            let a1016 = v3;
            nat_sub(a1015, a1016)
        };
        let v7: u64 = f34(v6, v1)?;
        let v8: u64 = f57(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f40(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1017 = v0;
    let a1018 = v3;
    if nat_lt(a1017, a1018) {
        Ok(Adt5::C1(true, 0u64, f34(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1019 = v0;
            let a1020 = v3;
            nat_sub(a1019, a1020)
        };
        let v5: u64 = f35(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f34(v4, v1)?, v5))
    }
}

pub fn f41(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1022 = v1;
        let a1023 = match v3.clone() {
            Adt3::C0(h1021, _, _, _, _, _, _, _) => {
                h1021
            }
        };
        let a1024 = 0u64;
        nat_rem(a1022, a1023, a1024)
    };
    let v5: u64 = {
        let a1030 = {
            let a1026 = v1;
            let a1027 = match v3.clone() {
                Adt3::C0(h1025, _, _, _, _, _, _, _) => {
                    h1025
                }
            };
            let a1028 = 0u64;
            nat_quot(a1026, a1027, a1028)
        };
        let a1031 = match v3.clone() {
            Adt3::C0(_, h1029, _, _, _, _, _, _) => {
                h1029
            }
        };
        let a1032 = 0u64;
        nat_rem(a1030, a1031, a1032)
    };
    let v6: bool = f58(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f35(v1, v2, v3.clone())?
    } else {
        0u64
    }, f34(v1, v3)?))
}

pub fn f42(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f34(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f35(v0, v5, v2.clone())?
    }, f34(v0, v2)?))
}

pub fn f43(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1034 = v0;
        let a1035 = match v2.clone() {
            Adt3::C0(h1033, _, _, _, _, _, _, _) => {
                h1033
            }
        };
        let a1036 = 0u64;
        nat_rem(a1034, a1035, a1036)
    };
    let v7: u64 = {
        let a1038 = v0;
        let a1039 = match v2.clone() {
            Adt3::C0(h1037, _, _, _, _, _, _, _) => {
                h1037
            }
        };
        let a1040 = 0u64;
        nat_quot(a1038, a1039, a1040)
    };
    let v8: u64 = {
        let a1042 = v7;
        let a1043 = match v2.clone() {
            Adt3::C0(_, h1041, _, _, _, _, _, _) => {
                h1041
            }
        };
        let a1044 = 0u64;
        nat_rem(a1042, a1043, a1044)
    };
    let v9: u64 = {
        let a1046 = v7;
        let a1047 = match v2.clone() {
            Adt3::C0(_, h1045, _, _, _, _, _, _) => {
                h1045
            }
        };
        let a1048 = 0u64;
        nat_quot(a1046, a1047, a1048)
    };
    let v10: u64 = {
        let a1050 = v9;
        let a1051 = match v2.clone() {
            Adt3::C0(_, _, h1049, _, _, _, _, _) => {
                h1049
            }
        };
        let a1052 = 0u64;
        nat_rem(a1050, a1051, a1052)
    };
    let v11: u64 = {
        let a1054 = v9;
        let a1055 = match v2.clone() {
            Adt3::C0(_, _, h1053, _, _, _, _, _) => {
                h1053
            }
        };
        let a1056 = 0u64;
        nat_quot(a1054, a1055, a1056)
    };
    let v12: u64 = f34(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1065 = {
                let a1060 = {
                    let a1057 = v10;
                    let a1058 = v4;
                    let a1059 = 0u64;
                    nat_quot(a1057, a1058, a1059)
                };
                let a1061 = v5;
                nat_mul(a1060, a1061)?
            };
            let a1066 = {
                let a1062 = v8;
                let a1063 = v4;
                let a1064 = 0u64;
                nat_quot(a1062, a1063, a1064)
            };
            nat_add(a1065, a1066)?
        };
        let v14: u64 = {
            let a1086 = v6;
            let a1087 = {
                let a1084 = match v1.clone() {
                    Adt3::C0(h1067, _, _, _, _, _, _, _) => {
                        h1067
                    }
                };
                let a1085 = {
                    let a1082 = {
                        let a1068 = v8;
                        let a1069 = v4;
                        let a1070 = 0u64;
                        nat_rem(a1068, a1069, a1070)
                    };
                    let a1083 = {
                        let a1080 = match v1.clone() {
                            Adt3::C0(_, h1071, _, _, _, _, _, _) => {
                                h1071
                            }
                        };
                        let a1081 = {
                            let a1078 = {
                                let a1072 = v10;
                                let a1073 = v4;
                                let a1074 = 0u64;
                                nat_rem(a1072, a1073, a1074)
                            };
                            let a1079 = {
                                let a1076 = match v1.clone() {
                                    Adt3::C0(_, _, h1075, _, _, _, _, _) => {
                                        h1075
                                    }
                                };
                                let a1077 = v13;
                                nat_mul(a1076, a1077)?
                            };
                            nat_add(a1078, a1079)?
                        };
                        nat_mul(a1080, a1081)?
                    };
                    nat_add(a1082, a1083)?
                };
                nat_mul(a1084, a1085)?
            };
            nat_add(a1086, a1087)?
        };
        Ok(Adt5::C1(false, f34(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1093 = {
                let a1091 = {
                    let a1088 = v11;
                    let a1089 = v5;
                    let a1090 = 0u64;
                    nat_rem(a1088, a1089, a1090)
                };
                let a1092 = v4;
                nat_mul(a1091, a1092)?
            };
            let a1094 = v8;
            nat_add(a1093, a1094)?
        };
        let v16: u64 = {
            let a1100 = {
                let a1098 = {
                    let a1095 = v11;
                    let a1096 = v5;
                    let a1097 = 0u64;
                    nat_quot(a1095, a1096, a1097)
                };
                let a1099 = v4;
                nat_mul(a1098, a1099)?
            };
            let a1101 = v10;
            nat_add(a1100, a1101)?
        };
        let a1103 = v15;
        let a1104 = match v1.clone() {
            Adt3::C0(_, h1102, _, _, _, _, _, _) => {
                h1102
            }
        };
        if if nat_lt(a1103, a1104) {
            let a1106 = v16;
            let a1107 = match v1.clone() {
                Adt3::C0(_, _, h1105, _, _, _, _, _) => {
                    h1105
                }
            };
            nat_lt(a1106, a1107)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f34({
                let a1116 = v6;
                let a1117 = {
                    let a1114 = match v1.clone() {
                        Adt3::C0(h1108, _, _, _, _, _, _, _) => {
                            h1108
                        }
                    };
                    let a1115 = {
                        let a1112 = v15;
                        let a1113 = {
                            let a1110 = match v1.clone() {
                                Adt3::C0(_, h1109, _, _, _, _, _, _) => {
                                    h1109
                                }
                            };
                            let a1111 = v16;
                            nat_mul(a1110, a1111)?
                        };
                        nat_add(a1112, a1113)?
                    };
                    nat_mul(a1114, a1115)?
                };
                nat_add(a1116, a1117)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f44(v0: u64) -> u64 {
    let a1118 = v0;
    let a1119 = 2147483648u64;
    if nat_lt(a1118, a1119) {
        v0
    } else {
        0u64
    }
}

pub fn f45(v0: u64) -> u64 {
    let a1120 = v0;
    let a1121 = 2147483648u64;
    if nat_lt(a1120, a1121) {
        0u64
    } else {
        let a1122 = 4294967296u64;
        let a1123 = v0;
        nat_sub(a1122, a1123)
    }
}

pub fn f46(v0: Adt3, v1: bool) -> R<u64> {
    f59(v0.clone(), {
        let a1125 = match v0.clone() {
            Adt3::C0(h1124, _, _, _, _, _, _, _) => {
                h1124
            }
        };
        let a1126 = 1u64;
        nat_sub(a1125, a1126)
    }, {
        let a1128 = match v0.clone() {
            Adt3::C0(_, h1127, _, _, _, _, _, _) => {
                h1127
            }
        };
        let a1129 = 1u64;
        nat_sub(a1128, a1129)
    }, {
        let a1131 = match v0.clone() {
            Adt3::C0(_, _, h1130, _, _, _, _, _) => {
                h1130
            }
        };
        let a1132 = 1u64;
        nat_sub(a1131, a1132)
    }, {
        let a1134 = match v0 {
            Adt3::C0(_, _, _, h1133, _, _, _, _) => {
                h1133
            }
        };
        let a1135 = 1u64;
        nat_sub(a1134, a1135)
    }, v1)
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1136 = v0;
    let a1137 = 0u64;
    if nat_eq(a1136, a1137) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1138 = v0;
            let a1139 = 1u64;
            nat_sub(a1138, a1139)
        };
        let v5: u64 = {
            let a1140 = v4;
            let a1141 = v1;
            let a1142 = 0u64;
            nat_quot(a1140, a1141, a1142)
        };
        let v6: u64 = {
            let a1150 = {
                let a1143 = v5;
                let a1144 = v2;
                nat_mul(a1143, a1144)?
            };
            let a1151 = {
                let a1148 = {
                    let a1145 = v4;
                    let a1146 = v1;
                    let a1147 = 0u64;
                    nat_rem(a1145, a1146, a1147)
                };
                let a1149 = v3;
                nat_mul(a1148, a1149)?
            };
            nat_add(a1150, a1151)?
        };
        let a1152 = v5;
        let a1153 = 0u64;
        if nat_eq(a1152, a1153) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1162 = {
                    let a1156 = {
                        let a1154 = v5;
                        let a1155 = 1u64;
                        nat_sub(a1154, a1155)
                    };
                    let a1157 = v2;
                    nat_mul(a1156, a1157)?
                };
                let a1163 = {
                    let a1160 = {
                        let a1158 = v1;
                        let a1159 = 1u64;
                        nat_sub(a1158, a1159)
                    };
                    let a1161 = v3;
                    nat_mul(a1160, a1161)?
                };
                nat_add(a1162, a1163)?
            };
            let a1164 = v6;
            let a1165 = v7;
            if nat_lt(a1164, a1165) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1166 = f60(v0, v1);
    match m1166 {
        None => {
            if v5 {
                let m1167 = f61(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1167 {
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
                let a1168 = 4u64;
                let a1169 = v28;
                nat_le(a1168, a1169)
            };
            let a1170 = v2;
            let a1171 = 0u64;
            if if if if if nat_eq(a1170, a1171) {
                let a1172 = v4;
                let a1173 = 0u64;
                nat_eq(a1172, a1173)
            } else {
                false
            } {
                let a1174 = v6;
                let a1175 = v14;
                if nat_eq(a1174, a1175) {
                    let a1176 = v7;
                    let a1177 = v15;
                    nat_eq(a1176, a1177)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1178 = v8;
                let a1179 = v16;
                if nat_eq(a1178, a1179) {
                    let a1180 = v9;
                    let a1181 = v17;
                    nat_eq(a1180, a1181)
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
                        let a1182 = v3;
                        let a1183 = 0u64;
                        nat_eq(a1182, a1183)
                    } else {
                        false
                    } {
                        f21(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1188 = {
                        let a1184 = v6;
                        let a1185 = v7;
                        nat_mul(a1184, a1185)?
                    };
                    let a1189 = {
                        let a1186 = v8;
                        let a1187 = v9;
                        nat_mul(a1186, a1187)?
                    };
                    nat_mul(a1188, a1189)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1202 = {
        let a1198 = {
            let a1194 = {
                let a1190 = v0;
                let a1191 = v4;
                nat_mul(a1190, a1191)?
            };
            let a1195 = {
                let a1192 = v1;
                let a1193 = v5;
                nat_mul(a1192, a1193)?
            };
            nat_add(a1194, a1195)?
        };
        let a1199 = {
            let a1196 = v2;
            let a1197 = v6;
            nat_mul(a1196, a1197)?
        };
        nat_add(a1198, a1199)?
    };
    let a1203 = {
        let a1200 = v3;
        let a1201 = v7;
        nat_mul(a1200, a1201)?
    };
    nat_add(a1202, a1203)
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f49({
        let a1204 = v0;
        let a1205 = v4;
        let a1206 = 0u64;
        nat_rem(a1204, a1205, a1206)
    }, {
        let a1207 = v1;
        let a1208 = v5;
        let a1209 = 0u64;
        nat_rem(a1207, a1208, a1209)
    }, {
        let a1210 = v2;
        let a1211 = v6;
        let a1212 = 0u64;
        nat_rem(a1210, a1211, a1212)
    }, {
        let a1213 = v3;
        let a1214 = v7;
        let a1215 = 0u64;
        nat_rem(a1213, a1214, a1215)
    }, v8, v9, v10, v11)
}

pub fn f51(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1217 = v0;
        let a1218 = match v1.clone() {
            Adt3::C0(h1216, _, _, _, _, _, _, _) => {
                h1216
            }
        };
        let a1219 = 0u64;
        nat_quot(a1217, a1218, a1219)
    };
    let v4: u64 = {
        let a1221 = v3;
        let a1222 = match v1.clone() {
            Adt3::C0(_, h1220, _, _, _, _, _, _) => {
                h1220
            }
        };
        let a1223 = 0u64;
        nat_quot(a1221, a1222, a1223)
    };
    let a1224 = v2;
    let a1225 = 0u64;
    if nat_eq(a1224, a1225) {
        let a1227 = v0;
        let a1228 = match v1 {
            Adt3::C0(h1226, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1229 = 0u64;
        nat_rem(a1227, a1228, a1229)
    } else {
        let a1230 = v2;
        let a1231 = 1u64;
        if nat_eq(a1230, a1231) {
            let a1233 = v3;
            let a1234 = match v1 {
                Adt3::C0(_, h1232, _, _, _, _, _, _) => {
                    h1232
                }
            };
            let a1235 = 0u64;
            nat_rem(a1233, a1234, a1235)
        } else {
            let a1236 = v2;
            let a1237 = 2u64;
            if nat_eq(a1236, a1237) {
                let a1239 = v4;
                let a1240 = match v1 {
                    Adt3::C0(_, _, h1238, _, _, _, _, _) => {
                        h1238
                    }
                };
                let a1241 = 0u64;
                nat_rem(a1239, a1240, a1241)
            } else {
                let a1243 = v4;
                let a1244 = match v1 {
                    Adt3::C0(_, _, h1242, _, _, _, _, _) => {
                        h1242
                    }
                };
                let a1245 = 0u64;
                nat_quot(a1243, a1244, a1245)
            }
        }
    }
}

pub fn f52(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f49(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1246, _, _, _) => {
            h1246
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1247, _, _) => {
            h1247
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1248, _) => {
            h1248
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1249) => {
            h1249
        }
    })
}

pub fn f53(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1250 = v1;
    let a1251 = 2147483648u64;
    if nat_lt(a1250, a1251) {
        let a1262 = {
            let a1260 = {
                let a1255 = {
                    let a1252 = v0;
                    let a1253 = v2;
                    let a1254 = 0u64;
                    nat_rem(a1252, a1253, a1254)
                };
                let a1256 = v2;
                nat_add(a1255, a1256)?
            };
            let a1261 = {
                let a1257 = v1;
                let a1258 = v2;
                let a1259 = 0u64;
                nat_rem(a1257, a1258, a1259)
            };
            nat_sub(a1260, a1261)
        };
        let a1263 = v2;
        let a1264 = 0u64;
        Ok(nat_rem(a1262, a1263, a1264))
    } else {
        let a1267 = {
            let a1265 = v0;
            let a1266 = f45(v1);
            nat_add(a1265, a1266)?
        };
        let a1268 = v2;
        let a1269 = 0u64;
        Ok(nat_rem(a1267, a1268, a1269))
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1272 = f44(v1);
    let a1273 = {
        let a1270 = v0;
        let a1271 = f45(v1);
        nat_add(a1270, a1271)?
    };
    if nat_le(a1272, a1273) {
        let a1274 = f55(v0, v1)?;
        let a1275 = v2;
        Ok(nat_lt(a1274, a1275))
    } else {
        Ok(false)
    }
}

pub fn f55(v0: u64, v1: u64) -> R<u64> {
    let a1278 = {
        let a1276 = v0;
        let a1277 = f45(v1);
        nat_add(a1276, a1277)?
    };
    let a1279 = f44(v1);
    Ok(nat_sub(a1278, a1279))
}

pub fn f56(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1280 = v0;
    let a1281 = v1;
    if nat_lt(a1280, a1281) {
        let a1282 = v1;
        let a1283 = v0;
        Ok(nat_sub(a1282, a1283))
    } else {
        let a1286 = v0;
        let a1287 = {
            let a1284 = v1;
            let a1285 = v2;
            nat_add(a1284, a1285)?
        };
        if nat_lt(a1286, a1287) {
            let a1288 = v0;
            let a1289 = v1;
            Ok(nat_sub(a1288, a1289))
        } else {
            let a1296 = {
                let a1292 = 2u64;
                let a1293 = {
                    let a1290 = v2;
                    let a1291 = 1u64;
                    nat_sub(a1290, a1291)
                };
                nat_mul(a1292, a1293)?
            };
            let a1297 = {
                let a1294 = v0;
                let a1295 = v1;
                nat_sub(a1294, a1295)
            };
            Ok(nat_sub(a1296, a1297))
        }
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1300 = {
        let a1298 = v2;
        let a1299 = f59(v1.clone(), f51(v0, v1.clone(), 0u64), f51(v0, v1.clone(), 1u64), f51(v0, v1.clone(), 2u64), f51(v0, v1.clone(), 3u64), false)?;
        nat_add(a1298, a1299)?
    };
    let a1301 = f59(v1.clone(), f51(v0, v1.clone(), 0u64), f51(v0, v1.clone(), 1u64), f51(v0, v1.clone(), 2u64), f51(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1300, a1301))
}

pub fn f58(v0: u64, v1: u64, v2: u64) -> bool {
    let a1302 = v0;
    let a1303 = 0u64;
    if nat_eq(a1302, a1303) {
        let a1304 = v2;
        let a1305 = v1;
        nat_le(a1304, a1305)
    } else {
        let a1306 = v0;
        let a1307 = 1u64;
        if nat_eq(a1306, a1307) {
            let a1308 = v2;
            let a1309 = v1;
            nat_lt(a1308, a1309)
        } else {
            let a1310 = v0;
            let a1311 = 2u64;
            if nat_eq(a1310, a1311) {
                let a1312 = v1;
                let a1313 = v2;
                nat_le(a1312, a1313)
            } else {
                let a1314 = v0;
                let a1315 = 3u64;
                if nat_eq(a1314, a1315) {
                    let a1316 = v1;
                    let a1317 = v2;
                    nat_lt(a1316, a1317)
                } else {
                    let a1318 = v1;
                    let a1319 = v2;
                    nat_eq(a1318, a1319)
                }
            }
        }
    }
}

pub fn f59(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1340 = {
        let a1334 = {
            let a1328 = {
                let a1322 = v1;
                let a1323 = if v5 {
                    f45(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1320, _, _, _) => {
                            h1320
                        }
                    })
                } else {
                    f44(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1321, _, _, _) => {
                            h1321
                        }
                    })
                };
                nat_mul(a1322, a1323)?
            };
            let a1329 = {
                let a1326 = v2;
                let a1327 = if v5 {
                    f45(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1324, _, _) => {
                            h1324
                        }
                    })
                } else {
                    f44(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1325, _, _) => {
                            h1325
                        }
                    })
                };
                nat_mul(a1326, a1327)?
            };
            nat_add(a1328, a1329)?
        };
        let a1335 = {
            let a1332 = v3;
            let a1333 = if v5 {
                f45(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1330, _) => {
                        h1330
                    }
                })
            } else {
                f44(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1331, _) => {
                        h1331
                    }
                })
            };
            nat_mul(a1332, a1333)?
        };
        nat_add(a1334, a1335)?
    };
    let a1341 = {
        let a1338 = v4;
        let a1339 = if v5 {
            f45(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1336) => {
                    h1336
                }
            })
        } else {
            f44(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1337) => {
                    h1337
                }
            })
        };
        nat_mul(a1338, a1339)?
    };
    nat_add(a1340, a1341)
}

pub fn f60(v0: u64, v1: u64) -> Option<u64> {
    let a1342 = v0;
    let a1343 = 2u64;
    if nat_eq(a1342, a1343) {
        Some(0u64)
    } else {
        let a1344 = v0;
        let a1345 = 5u64;
        if nat_eq(a1344, a1345) {
            Some(1u64)
        } else {
            let a1346 = v0;
            let a1347 = 6u64;
            if nat_eq(a1346, a1347) {
                Some(2u64)
            } else {
                let a1348 = v0;
                let a1349 = 7u64;
                if nat_eq(a1348, a1349) {
                    Some(3u64)
                } else {
                    let a1350 = v0;
                    let a1351 = 9u64;
                    if nat_eq(a1350, a1351) {
                        Some(4u64)
                    } else {
                        let a1352 = v0;
                        let a1353 = 8u64;
                        if nat_eq(a1352, a1353) {
                            Some(5u64)
                        } else {
                            let a1354 = v0;
                            let a1355 = 89u64;
                            if nat_eq(a1354, a1355) {
                                let a1356 = v1;
                                let a1357 = 2u64;
                                if nat_eq(a1356, a1357) {
                                    Some(6u64)
                                } else {
                                    let a1358 = v1;
                                    let a1359 = 0u64;
                                    if nat_eq(a1358, a1359) {
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

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1360 = v0;
    let a1361 = 28u64;
    if if if if nat_eq(a1360, a1361) {
        let a1362 = v1;
        let a1363 = 0u64;
        nat_eq(a1362, a1363)
    } else {
        false
    } {
        let a1364 = v2;
        let a1365 = 0u64;
        nat_eq(a1364, a1365)
    } else {
        false
    } {
        let a1366 = v3;
        let a1367 = 0u64;
        nat_eq(a1366, a1367)
    } else {
        false
    } {
        let a1368 = v4;
        let a1369 = v8;
        if if if if if if if if nat_eq(a1368, a1369) {
            let a1370 = v5;
            let a1371 = v12;
            nat_eq(a1370, a1371)
        } else {
            false
        } {
            let a1372 = v9;
            let a1373 = v13;
            nat_eq(a1372, a1373)
        } else {
            false
        } {
            let a1374 = v10;
            let a1375 = v14;
            nat_eq(a1374, a1375)
        } else {
            false
        } {
            let a1376 = v11;
            let a1377 = v15;
            nat_eq(a1376, a1377)
        } else {
            false
        } {
            let a1378 = 0u64;
            let a1379 = v6;
            nat_lt(a1378, a1379)
        } else {
            false
        } {
            let a1380 = 0u64;
            let a1381 = v7;
            nat_lt(a1380, a1381)
        } else {
            false
        } {
            let a1385 = {
                let a1382 = v10;
                let a1383 = v6;
                let a1384 = 1u64;
                nat_rem(a1382, a1383, a1384)
            };
            let a1386 = 0u64;
            if nat_eq(a1385, a1386) {
                let a1390 = {
                    let a1387 = v11;
                    let a1388 = v7;
                    let a1389 = 1u64;
                    nat_rem(a1387, a1388, a1389)
                };
                let a1391 = 0u64;
                nat_eq(a1390, a1391)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1392 = v10;
                let a1393 = v6;
                let a1394 = 0u64;
                nat_quot(a1392, a1393, a1394)
            }, {
                let a1395 = v11;
                let a1396 = v7;
                let a1397 = 0u64;
                nat_quot(a1395, a1396, a1397)
            }, {
                let a1398 = v5;
                let a1399 = v4;
                nat_mul(a1398, a1399)?
            }, {
                let a1400 = v4;
                let a1401 = v9;
                nat_mul(a1400, a1401)?
            }, {
                let a1402 = v5;
                let a1403 = v9;
                nat_mul(a1402, a1403)?
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
