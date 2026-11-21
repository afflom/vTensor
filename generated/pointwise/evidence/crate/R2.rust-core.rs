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
                                                        f12(v1, v6, v3, v4, v5)?
                                                    }
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
            f13(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a40 = v0;
    let a41 = 1u64;
    if if nat_eq(a40, a41) {
        true
    } else {
        let a42 = v0;
        let a43 = 33u64;
        nat_eq(a42, a43)
    } {
        true
    } else {
        let a44 = v0;
        let a45 = 34u64;
        nat_eq(a44, a45)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a48 = match v1.clone() {
        Adt2::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
            h46
        }
    };
    let a49 = match v3.clone() {
        Adt2::C0(h47, _, _, _, _, _, _, _, _, _, _) => {
            h47
        }
    };
    if nat_eq(a48, a49) {
        f14(v0, v1, v2, v3)
    } else {
        if if if if f15(match v1.clone() {
            Adt2::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                h50
            }
        }) {
            f15(match v3.clone() {
                Adt2::C0(h51, _, _, _, _, _, _, _, _, _, _) => {
                    h51
                }
            })
        } else {
            false
        } {
            let a52 = v0;
            let a53 = 33u64;
            if nat_eq(a52, a53) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a54 = f16(v1.clone(), 1u64)?;
            let a55 = f16(v3.clone(), 1u64)?;
            nat_eq(a54, a55)
        } else {
            false
        } {
            let m57 = f17(match v3.clone() {
                Adt2::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                    h56
                }
            });
            match m57 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f16(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h58) => {
                            h58
                        }
                    }, f18(v1.clone(), 1u64), f18(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                            h59
                        }
                    }, match v3 {
                        Adt2::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                            h60
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
    let a62 = match v0.clone() {
        Adt2::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
            h61
        }
    };
    let a63 = 0u64;
    if if if if nat_eq(a62, a63) {
        true
    } else {
        let a65 = match v0.clone() {
            Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        let a66 = 1u64;
        nat_eq(a65, a66)
    } {
        true
    } else {
        let a68 = match v0.clone() {
            Adt2::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                h67
            }
        };
        let a69 = 25u64;
        if nat_eq(a68, a69) {
            true
        } else {
            let a71 = match v0.clone() {
                Adt2::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
                    h70
                }
            };
            let a72 = 26u64;
            nat_eq(a71, a72)
        }
    } {
        true
    } else {
        let a74 = match v0.clone() {
            Adt2::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        };
        let a75 = 30u64;
        nat_eq(a74, a75)
    } {
        let a78 = match v0.clone() {
            Adt2::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                h76
            }
        };
        let a79 = match v1.clone() {
            Adt2::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                h77
            }
        };
        if if nat_eq(a78, a79) {
            f19(match v1.clone() {
                Adt2::C0(_, h80, _, _, _, _, _, _, _, _, _) => {
                    h80
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h81, _, _, _, _, _, _, _, _) => {
                    h81
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h82, _, _, _, _, _, _, _) => {
                    h82
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h83, _, _, _, _, _, _) => {
                    h83
                }
            }, match v0.clone() {
                Adt2::C0(_, h84, _, _, _, _, _, _, _, _, _) => {
                    h84
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h85, _, _, _, _, _, _, _, _) => {
                    h85
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h86, _, _, _, _, _, _, _) => {
                    h86
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h87, _, _, _, _, _, _) => {
                    h87
                }
            })
        } else {
            false
        } {
            let m89 = f17(match v0.clone() {
                Adt2::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                    h88
                }
            });
            match m89 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h90) => {
                            h90
                        }
                    }, f18(v0, 1u64), f18(v1, 1u64), Adt4::C1))
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
    let m91 = v2;
    match m91 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a92 = v0;
            let a93 = 4u64;
            if if if nat_lt(a92, a93) {
                let a96 = match v1.clone() {
                    Adt2::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                        h94
                    }
                };
                let a97 = match v4.clone() {
                    Adt2::C0(h95, _, _, _, _, _, _, _, _, _, _) => {
                        h95
                    }
                };
                if nat_eq(a96, a97) {
                    let a100 = match v1.clone() {
                        Adt2::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                            h98
                        }
                    };
                    let a101 = match v3.clone() {
                        Adt2::C0(h99, _, _, _, _, _, _, _, _, _, _) => {
                            h99
                        }
                    };
                    nat_eq(a100, a101)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f20(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h102, _, _, _, _, _, _, _, _, _) => {
                        h102
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h103, _, _, _, _, _, _, _, _, _) => {
                        h103
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                })? {
                    f20(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h105, _, _, _, _, _, _, _, _) => {
                            h105
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h106, _, _, _, _, _, _, _, _) => {
                            h106
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h107, _, _, _, _, _, _, _, _) => {
                            h107
                        }
                    })?
                } else {
                    false
                } {
                    f20(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h108, _, _, _, _, _, _, _) => {
                            h108
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h109, _, _, _, _, _, _, _) => {
                            h109
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h110, _, _, _, _, _, _, _) => {
                            h110
                        }
                    })?
                } else {
                    false
                } {
                    f20(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h111, _, _, _, _, _, _) => {
                            h111
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h112, _, _, _, _, _, _) => {
                            h112
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h113, _, _, _, _, _, _) => {
                            h113
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m115 = f17(match v1.clone() {
                    Adt2::C0(h114, _, _, _, _, _, _, _, _, _, _) => {
                        h114
                    }
                });
                match m115 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h116, _) => {
                                h116
                            }
                        };
                        let a121 = {
                            let a118 = match v1.clone() {
                                Adt2::C0(_, h117, _, _, _, _, _, _, _, _, _) => {
                                    h117
                                }
                            };
                            let a119 = v6;
                            let a120 = 1u64;
                            nat_rem(a118, a119, a120)
                        };
                        let a122 = 0u64;
                        if if if nat_eq(a121, a122) {
                            let a127 = {
                                let a124 = match v4.clone() {
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
                            let a133 = {
                                let a130 = match v3.clone() {
                                    Adt2::C0(_, h129, _, _, _, _, _, _, _, _, _) => {
                                        h129
                                    }
                                };
                                let a131 = v6;
                                let a132 = 1u64;
                                nat_rem(a130, a131, a132)
                            };
                            let a134 = 0u64;
                            nat_eq(a133, a134)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h135) => {
                                    h135
                                }
                            }, f18(v1, v6), f18(v3, v6), Adt4::C2(v0, f18(v4, v6))))
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
    let a137 = match v2.clone() {
        Adt2::C0(h136, _, _, _, _, _, _, _, _, _, _) => {
            h136
        }
    };
    let a138 = 0u64;
    if if nat_eq(a137, a138) {
        let a140 = match v3.clone() {
            Adt2::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
                h139
            }
        };
        let a141 = 0u64;
        nat_eq(a140, a141)
    } else {
        false
    } {
        let a142 = v0;
        let a143 = 61u64;
        if nat_eq(a142, a143) {
            let v4: bool = {
                let a145 = 0u64;
                let a146 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h144, _, _, _, _, _, _, _) => {
                        h144
                    }
                };
                nat_lt(a145, a146)
            };
            if if if if if if f21(match v2.clone() {
                Adt2::C0(_, h147, _, _, _, _, _, _, _, _, _) => {
                    h147
                }
            }, match v1.clone() {
                Adt1::C0(h148, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h148
                }
            }, match v1.clone() {
                Adt1::C0(_, h149, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h149
                }
            }, match v3.clone() {
                Adt2::C0(_, h150, _, _, _, _, _, _, _, _, _) => {
                    h150
                }
            })? {
                f21(match v2.clone() {
                    Adt2::C0(_, _, h151, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h152, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h152
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h153, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h153
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h154, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                })?
            } else {
                false
            } {
                f21(match v2.clone() {
                    Adt2::C0(_, _, _, h155, _, _, _, _, _, _, _) => {
                        h155
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h156, _, _, _, _, _, _, _, _, _, _, _) => {
                        h156
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h157, _, _, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h158, _, _, _, _, _, _, _) => {
                        h158
                    }
                })?
            } else {
                false
            } {
                f21(match v2.clone() {
                    Adt2::C0(_, _, _, _, h159, _, _, _, _, _, _) => {
                        h159
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h160, _, _, _, _, _, _, _, _, _) => {
                        h160
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h161, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h162, _, _, _, _, _, _) => {
                        h162
                    }
                })?
            } else {
                false
            } {
                let a164 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h163, _, _, _, _, _, _, _) => {
                        h163
                    }
                };
                let a165 = 4294967295u64;
                nat_le(a164, a165)
            } else {
                false
            } {
                if v4 {
                    !f22(match v2.clone() {
                        Adt2::C0(_, h166, _, _, _, _, _, _, _, _, _) => {
                            h166
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h167, _, _, _, _, _, _, _, _) => {
                            h167
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h168, _, _, _, _, _, _, _) => {
                            h168
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h169, _, _, _, _, _, _) => {
                            h169
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f16(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h170, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h170
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h171, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h171
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h172, _, _, _, _, _, _, _, _, _, _, _) => {
                        h172
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h173, _, _, _, _, _, _, _, _, _) => {
                        h173
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a174 = v0;
            let a175 = 62u64;
            if nat_eq(a174, a175) {
                let a177 = match v1.clone() {
                    Adt1::C0(h176, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h176
                    }
                };
                let a178 = 2147483648u64;
                if if if if if nat_lt(a177, a178) {
                    let a180 = match v1.clone() {
                        Adt1::C0(_, h179, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h179
                        }
                    };
                    let a181 = 2147483648u64;
                    nat_lt(a180, a181)
                } else {
                    false
                } {
                    let a184 = match v1.clone() {
                        Adt1::C0(h182, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h182
                        }
                    };
                    let a185 = match v2.clone() {
                        Adt2::C0(_, h183, _, _, _, _, _, _, _, _, _) => {
                            h183
                        }
                    };
                    nat_lt(a184, a185)
                } else {
                    false
                } {
                    let a188 = match v1.clone() {
                        Adt1::C0(_, h186, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h186
                        }
                    };
                    let a189 = match v2.clone() {
                        Adt2::C0(_, h187, _, _, _, _, _, _, _, _, _) => {
                            h187
                        }
                    };
                    nat_lt(a188, a189)
                } else {
                    false
                } {
                    let a192 = match v2.clone() {
                        Adt2::C0(_, _, h190, _, _, _, _, _, _, _, _) => {
                            h190
                        }
                    };
                    let a193 = match v3.clone() {
                        Adt2::C0(_, _, h191, _, _, _, _, _, _, _, _) => {
                            h191
                        }
                    };
                    if if if nat_eq(a192, a193) {
                        let a196 = match v2.clone() {
                            Adt2::C0(_, _, _, h194, _, _, _, _, _, _, _) => {
                                h194
                            }
                        };
                        let a197 = match v3.clone() {
                            Adt2::C0(_, _, _, h195, _, _, _, _, _, _, _) => {
                                h195
                            }
                        };
                        nat_eq(a196, a197)
                    } else {
                        false
                    } {
                        let a200 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h198, _, _, _, _, _, _) => {
                                h198
                            }
                        };
                        let a201 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                                h199
                            }
                        };
                        nat_eq(a200, a201)
                    } else {
                        false
                    } {
                        let a210 = match v3.clone() {
                            Adt2::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                                h202
                            }
                        };
                        let a211 = {
                            let a208 = {
                                let a205 = match v2.clone() {
                                    Adt2::C0(_, h203, _, _, _, _, _, _, _, _, _) => {
                                        h203
                                    }
                                };
                                let a206 = match v1.clone() {
                                    Adt1::C0(h204, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h204
                                    }
                                };
                                nat_add(a205, a206)?
                            };
                            let a209 = match v1.clone() {
                                Adt1::C0(_, h207, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h207
                                }
                            };
                            nat_add(a208, a209)?
                        };
                        nat_eq(a210, a211)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f16(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h212, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h212
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a213 = v0;
                let a214 = 63u64;
                if nat_eq(a213, a214) {
                    if if if if if f23(match v2.clone() {
                        Adt2::C0(_, h215, _, _, _, _, _, _, _, _, _) => {
                            h215
                        }
                    }, match v1.clone() {
                        Adt1::C0(h216, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h216
                        }
                    })? {
                        f23(match v2.clone() {
                            Adt2::C0(_, _, h217, _, _, _, _, _, _, _, _) => {
                                h217
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h218, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h218
                            }
                        })?
                    } else {
                        false
                    } {
                        f23(match v2.clone() {
                            Adt2::C0(_, _, _, h219, _, _, _, _, _, _, _) => {
                                h219
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h220, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h220
                            }
                        })?
                    } else {
                        false
                    } {
                        f23(match v2.clone() {
                            Adt2::C0(_, _, _, _, h221, _, _, _, _, _, _) => {
                                h221
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h222, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h222
                            }
                        })?
                    } else {
                        false
                    } {
                        let a225 = match v2.clone() {
                            Adt2::C0(_, h223, _, _, _, _, _, _, _, _, _) => {
                                h223
                            }
                        };
                        let a226 = match v3.clone() {
                            Adt2::C0(_, h224, _, _, _, _, _, _, _, _, _) => {
                                h224
                            }
                        };
                        if if if nat_eq(a225, a226) {
                            let a229 = match v2.clone() {
                                Adt2::C0(_, _, h227, _, _, _, _, _, _, _, _) => {
                                    h227
                                }
                            };
                            let a230 = match v3.clone() {
                                Adt2::C0(_, _, h228, _, _, _, _, _, _, _, _) => {
                                    h228
                                }
                            };
                            nat_eq(a229, a230)
                        } else {
                            false
                        } {
                            let a233 = match v2.clone() {
                                Adt2::C0(_, _, _, h231, _, _, _, _, _, _, _) => {
                                    h231
                                }
                            };
                            let a234 = match v3.clone() {
                                Adt2::C0(_, _, _, h232, _, _, _, _, _, _, _) => {
                                    h232
                                }
                            };
                            nat_eq(a233, a234)
                        } else {
                            false
                        } {
                            let a237 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h235, _, _, _, _, _, _) => {
                                    h235
                                }
                            };
                            let a238 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h236, _, _, _, _, _, _) => {
                                    h236
                                }
                            };
                            nat_eq(a237, a238)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f16(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h239, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h239
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h240, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h240
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h241, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h241
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h242, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h242
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
    let m243 = v3;
    match m243 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a244 = v0;
                let a245 = 4u64;
                nat_eq(a244, a245)
            };
            let v7: bool = {
                let a247 = 0u64;
                let a248 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h246, _, _, _, _, _, _, _, _, _, _, _) => {
                        h246
                    }
                };
                nat_lt(a247, a248)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h249, _, _, _, _, _, _, _, _, _) => {
                    h249
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h250, _, _, _, _, _, _, _, _) => {
                    h250
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h251, _, _, _, _, _, _, _) => {
                    h251
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h252, _, _, _, _, _, _) => {
                    h252
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h253, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h253
                }
            }, match v1.clone() {
                Adt1::C0(_, h254, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h254
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h255, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h255
                }
            });
            if if if if if if if if if if if if v6 {
                let a257 = match v2.clone() {
                    Adt2::C0(h256, _, _, _, _, _, _, _, _, _, _) => {
                        h256
                    }
                };
                let a258 = 0u64;
                if nat_eq(a257, a258) {
                    let a260 = match v5.clone() {
                        Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                            h259
                        }
                    };
                    let a261 = 0u64;
                    nat_eq(a260, a261)
                } else {
                    false
                }
            } else {
                let a263 = match v2.clone() {
                    Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                        h262
                    }
                };
                let a264 = 0u64;
                if if nat_eq(a263, a264) {
                    true
                } else {
                    let a266 = match v2.clone() {
                        Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                            h265
                        }
                    };
                    let a267 = 26u64;
                    nat_eq(a266, a267)
                } {
                    let a269 = match v5.clone() {
                        Adt2::C0(h268, _, _, _, _, _, _, _, _, _, _) => {
                            h268
                        }
                    };
                    let a270 = 0u64;
                    if nat_eq(a269, a270) {
                        true
                    } else {
                        let a272 = match v5.clone() {
                            Adt2::C0(h271, _, _, _, _, _, _, _, _, _, _) => {
                                h271
                            }
                        };
                        let a273 = 26u64;
                        nat_eq(a272, a273)
                    }
                } else {
                    false
                }
            } {
                let a276 = match v2.clone() {
                    Adt2::C0(h274, _, _, _, _, _, _, _, _, _, _) => {
                        h274
                    }
                };
                let a277 = match v4.clone() {
                    Adt2::C0(h275, _, _, _, _, _, _, _, _, _, _) => {
                        h275
                    }
                };
                nat_eq(a276, a277)
            } else {
                false
            } {
                f24(v2.clone(), v4.clone())
            } else {
                false
            } {
                f25(v2.clone())?
            } else {
                false
            } {
                f25(v4.clone())?
            } else {
                false
            } {
                let a279 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h278, _, _, _, _, _) => {
                        h278
                    }
                };
                let a280 = 4u64;
                nat_eq(a279, a280)
            } else {
                false
            } {
                let a281 = v8;
                let a282 = f16(v2.clone(), 1u64)?;
                nat_le(a281, a282)
            } else {
                false
            } {
                let a284 = match v1.clone() {
                    Adt1::C0(_, _, _, h283, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h283
                    }
                };
                let a285 = 1073741824u64;
                nat_lt(a284, a285)
            } else {
                false
            } {
                let a287 = match v1.clone() {
                    Adt1::C0(h286, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h286
                    }
                };
                let a288 = 4294967295u64;
                if if if if nat_le(a287, a288) {
                    let a290 = match v1.clone() {
                        Adt1::C0(_, h289, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h289
                        }
                    };
                    let a291 = 4294967295u64;
                    nat_le(a290, a291)
                } else {
                    false
                } {
                    let a293 = match v1.clone() {
                        Adt1::C0(_, _, h292, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    let a294 = 4294967295u64;
                    nat_le(a293, a294)
                } else {
                    false
                } {
                    let a296 = match v1.clone() {
                        Adt1::C0(_, _, _, h295, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h295
                        }
                    };
                    let a297 = 4294967295u64;
                    nat_le(a296, a297)
                } else {
                    false
                } {
                    let a299 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h298, _, _, _, _, _, _, _, _, _, _, _) => {
                            h298
                        }
                    };
                    let a300 = 4294967295u64;
                    nat_le(a299, a300)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a303 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h301) => {
                            h301
                        }
                    };
                    let a304 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h302) => {
                            h302
                        }
                    };
                    nat_eq(a303, a304)
                } else {
                    true
                }
            } else {
                false
            } {
                f26(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h305, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h305
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h306, _) => {
                        h306
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f16(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a307 = v10;
                    let a308 = v8;
                    nat_add(a307, a308)?
                }, 4u64, f18(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h309, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h309
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a311 = match v0.clone() {
        Adt2::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
            h310
        }
    };
    let a312 = 0u64;
    if if if nat_eq(a311, a312) {
        let a314 = match v1.clone() {
            Adt2::C0(h313, _, _, _, _, _, _, _, _, _, _) => {
                h313
            }
        };
        let a315 = 0u64;
        nat_eq(a314, a315)
    } else {
        false
    } {
        f19(match v0.clone() {
            Adt2::C0(_, h316, _, _, _, _, _, _, _, _, _) => {
                h316
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h317, _, _, _, _, _, _, _, _) => {
                h317
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h318, _, _, _, _, _, _, _) => {
                h318
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h319, _, _, _, _, _, _) => {
                h319
            }
        }, match v1.clone() {
            Adt2::C0(_, h320, _, _, _, _, _, _, _, _, _) => {
                h320
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h321, _, _, _, _, _, _, _, _) => {
                h321
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h322, _, _, _, _, _, _, _) => {
                h322
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h323, _, _, _, _, _, _) => {
                h323
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a324 = v2;
            let a325 = f16(v0.clone(), 1u64)?;
            nat_add(a324, a325)?
        }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a327 = match v1.clone() {
        Adt2::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
            h326
        }
    };
    let a328 = 0u64;
    if if if nat_eq(a327, a328) {
        true
    } else {
        let a330 = match v1.clone() {
            Adt2::C0(h329, _, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        };
        let a331 = 1u64;
        nat_eq(a330, a331)
    } {
        true
    } else {
        let a333 = match v1.clone() {
            Adt2::C0(h332, _, _, _, _, _, _, _, _, _, _) => {
                h332
            }
        };
        let a334 = 30u64;
        nat_eq(a333, a334)
    } {
        let a337 = match v1.clone() {
            Adt2::C0(h335, _, _, _, _, _, _, _, _, _, _) => {
                h335
            }
        };
        let a338 = match v2.clone() {
            Adt2::C0(h336, _, _, _, _, _, _, _, _, _, _) => {
                h336
            }
        };
        if if if if if nat_eq(a337, a338) {
            let a341 = match v1.clone() {
                Adt2::C0(_, h339, _, _, _, _, _, _, _, _, _) => {
                    h339
                }
            };
            let a342 = match v2.clone() {
                Adt2::C0(_, h340, _, _, _, _, _, _, _, _, _) => {
                    h340
                }
            };
            nat_eq(a341, a342)
        } else {
            false
        } {
            let a345 = match v1.clone() {
                Adt2::C0(_, _, _, h343, _, _, _, _, _, _, _) => {
                    h343
                }
            };
            let a346 = match v2.clone() {
                Adt2::C0(_, _, _, h344, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            nat_eq(a345, a346)
        } else {
            false
        } {
            let a349 = match v1.clone() {
                Adt2::C0(_, _, _, _, h347, _, _, _, _, _, _) => {
                    h347
                }
            };
            let a350 = match v2.clone() {
                Adt2::C0(_, _, _, _, h348, _, _, _, _, _, _) => {
                    h348
                }
            };
            nat_eq(a349, a350)
        } else {
            false
        } {
            let a351 = v0;
            let a352 = 4u64;
            if nat_eq(a351, a352) {
                let a354 = match v1.clone() {
                    Adt2::C0(_, _, h353, _, _, _, _, _, _, _, _) => {
                        h353
                    }
                };
                let a355 = 1u64;
                if nat_eq(a354, a355) {
                    let a358 = match v1.clone() {
                        Adt2::C0(_, h356, _, _, _, _, _, _, _, _, _) => {
                            h356
                        }
                    };
                    let a359 = match v2.clone() {
                        Adt2::C0(_, _, h357, _, _, _, _, _, _, _, _) => {
                            h357
                        }
                    };
                    nat_eq(a358, a359)
                } else {
                    false
                }
            } else {
                let a360 = v0;
                let a361 = 4u64;
                if if nat_lt(a360, a361) {
                    let a364 = match v1.clone() {
                        Adt2::C0(_, h362, _, _, _, _, _, _, _, _, _) => {
                            h362
                        }
                    };
                    let a365 = match v1.clone() {
                        Adt2::C0(_, _, h363, _, _, _, _, _, _, _, _) => {
                            h363
                        }
                    };
                    nat_eq(a364, a365)
                } else {
                    false
                } {
                    let a368 = match v1.clone() {
                        Adt2::C0(_, _, h366, _, _, _, _, _, _, _, _) => {
                            h366
                        }
                    };
                    let a369 = match v2.clone() {
                        Adt2::C0(_, _, h367, _, _, _, _, _, _, _, _) => {
                            h367
                        }
                    };
                    nat_eq(a368, a369)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m371 = f17(match v1.clone() {
                Adt2::C0(h370, _, _, _, _, _, _, _, _, _, _) => {
                    h370
                }
            });
            match m371 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h372) => {
                            h372
                        }
                    }, f18(v1, 1u64), f18(v2, 1u64), Adt4::C8(v0)))
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
    if if if f27(match v1.clone() {
        Adt2::C0(h373, _, _, _, _, _, _, _, _, _, _) => {
            h373
        }
    }, v0) {
        let a376 = match v1.clone() {
            Adt2::C0(h374, _, _, _, _, _, _, _, _, _, _) => {
                h374
            }
        };
        let a377 = match v2.clone() {
            Adt2::C0(h375, _, _, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        nat_eq(a376, a377)
    } else {
        false
    } {
        let a380 = match v1.clone() {
            Adt2::C0(_, h378, _, _, _, _, _, _, _, _, _) => {
                h378
            }
        };
        let a381 = match v2.clone() {
            Adt2::C0(_, h379, _, _, _, _, _, _, _, _, _) => {
                h379
            }
        };
        if if if nat_eq(a380, a381) {
            let a384 = match v1.clone() {
                Adt2::C0(_, _, h382, _, _, _, _, _, _, _, _) => {
                    h382
                }
            };
            let a385 = match v2.clone() {
                Adt2::C0(_, _, h383, _, _, _, _, _, _, _, _) => {
                    h383
                }
            };
            nat_eq(a384, a385)
        } else {
            false
        } {
            let a388 = match v1.clone() {
                Adt2::C0(_, _, _, h386, _, _, _, _, _, _, _) => {
                    h386
                }
            };
            let a389 = match v2.clone() {
                Adt2::C0(_, _, _, h387, _, _, _, _, _, _, _) => {
                    h387
                }
            };
            nat_eq(a388, a389)
        } else {
            false
        } {
            let a392 = match v1.clone() {
                Adt2::C0(_, _, _, _, h390, _, _, _, _, _, _) => {
                    h390
                }
            };
            let a393 = match v2.clone() {
                Adt2::C0(_, _, _, _, h391, _, _, _, _, _, _) => {
                    h391
                }
            };
            nat_eq(a392, a393)
        } else {
            false
        }
    } else {
        false
    } {
        let m395 = f17(match v1.clone() {
            Adt2::C0(h394, _, _, _, _, _, _, _, _, _, _) => {
                h394
            }
        });
        match m395 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h396) => {
                        h396
                    }
                }, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h397, _, _, _, _, _, _, _, _, _, _) => {
                        h397
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
            Adt1::C0(h398, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h398
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h399, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h399
            }
        }
    };
    let a400 = 0u64;
    let a401 = v4;
    if if nat_lt(a400, a401) {
        let a402 = v4;
        let a403 = 2147483648u64;
        nat_lt(a402, a403)
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
        let v7: u64 = f28(match v5.clone() {
            Adt2::C0(_, _, h404, _, _, _, _, _, _, _, _) => {
                h404
            }
        }, v4)?;
        let v8: u64 = f28(match v5.clone() {
            Adt2::C0(_, _, _, h405, _, _, _, _, _, _, _) => {
                h405
            }
        }, v4)?;
        let a407 = match v5.clone() {
            Adt2::C0(h406, _, _, _, _, _, _, _, _, _, _) => {
                h406
            }
        };
        let a408 = 0u64;
        if if if if if nat_eq(a407, a408) {
            let a410 = match v6.clone() {
                Adt2::C0(h409, _, _, _, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            let a411 = 0u64;
            nat_eq(a410, a411)
        } else {
            false
        } {
            let a413 = match v5.clone() {
                Adt2::C0(_, _, _, _, h412, _, _, _, _, _, _) => {
                    h412
                }
            };
            let a414 = 1u64;
            nat_eq(a413, a414)
        } else {
            false
        } {
            let a417 = match v5.clone() {
                Adt2::C0(_, h415, _, _, _, _, _, _, _, _, _) => {
                    h415
                }
            };
            let a418 = match v6.clone() {
                Adt2::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
                    h416
                }
            };
            nat_eq(a417, a418)
        } else {
            false
        } {
            if v0 {
                let a420 = match v5.clone() {
                    Adt2::C0(_, _, h419, _, _, _, _, _, _, _, _) => {
                        h419
                    }
                };
                let a421 = 2147483648u64;
                if if if nat_lt(a420, a421) {
                    let a423 = match v5.clone() {
                        Adt2::C0(_, _, _, h422, _, _, _, _, _, _, _) => {
                            h422
                        }
                    };
                    let a424 = 2147483648u64;
                    nat_lt(a423, a424)
                } else {
                    false
                } {
                    let a427 = {
                        let a425 = v7;
                        let a426 = v8;
                        nat_mul(a425, a426)?
                    };
                    let a428 = 2147483648u64;
                    nat_le(a427, a428)
                } else {
                    false
                } {
                    f29(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a430 = match v1.clone() {
                    Adt1::C0(h429, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h429
                    }
                };
                let a431 = v7;
                if if if if nat_eq(a430, a431) {
                    let a433 = match v1 {
                        Adt1::C0(_, h432, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h432
                        }
                    };
                    let a434 = v8;
                    nat_eq(a433, a434)
                } else {
                    false
                } {
                    f30(v7, v8, v4)?
                } else {
                    false
                } {
                    let a436 = match v6.clone() {
                        Adt2::C0(_, _, h435, _, _, _, _, _, _, _, _) => {
                            h435
                        }
                    };
                    let a437 = v4;
                    if nat_eq(a436, a437) {
                        let a439 = match v6.clone() {
                            Adt2::C0(_, _, _, h438, _, _, _, _, _, _, _) => {
                                h438
                            }
                        };
                        let a440 = v4;
                        nat_eq(a439, a440)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a444 = match v6 {
                        Adt2::C0(_, _, _, _, h441, _, _, _, _, _, _) => {
                            h441
                        }
                    };
                    let a445 = {
                        let a442 = v7;
                        let a443 = v8;
                        nat_mul(a442, a443)?
                    };
                    nat_eq(a444, a445)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f16(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a447 = match v2.clone() {
        Adt2::C0(h446, _, _, _, _, _, _, _, _, _, _) => {
            h446
        }
    };
    let a448 = 0u64;
    if if if if if nat_eq(a447, a448) {
        true
    } else {
        let a450 = match v2.clone() {
            Adt2::C0(h449, _, _, _, _, _, _, _, _, _, _) => {
                h449
            }
        };
        let a451 = 1u64;
        nat_eq(a450, a451)
    } {
        true
    } else {
        let a453 = match v2.clone() {
            Adt2::C0(h452, _, _, _, _, _, _, _, _, _, _) => {
                h452
            }
        };
        let a454 = 30u64;
        nat_eq(a453, a454)
    } {
        let a457 = match v2.clone() {
            Adt2::C0(h455, _, _, _, _, _, _, _, _, _, _) => {
                h455
            }
        };
        let a458 = match v3.clone() {
            Adt2::C0(h456, _, _, _, _, _, _, _, _, _, _) => {
                h456
            }
        };
        nat_eq(a457, a458)
    } else {
        false
    } {
        if if f24(v2.clone(), v3.clone()) {
            let a460 = match v1.clone() {
                Adt1::C0(h459, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h459
                }
            };
            let a461 = 4294967296u64;
            nat_lt(a460, a461)
        } else {
            false
        } {
            let a463 = match v1.clone() {
                Adt1::C0(_, h462, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h462
                }
            };
            let a464 = 4294967296u64;
            nat_lt(a463, a464)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f16(v3.clone(), 1u64)?, {
            let a466 = match v2.clone() {
                Adt2::C0(h465, _, _, _, _, _, _, _, _, _, _) => {
                    h465
                }
            };
            let a467 = 0u64;
            if nat_eq(a466, a467) {
                4u64
            } else {
                2u64
            }
        }, f18(v2.clone(), 1u64), f18(v3, 1u64), Adt4::C13({
            let a468 = v0;
            let a469 = 31u64;
            if nat_eq(a468, a469) {
                0u64
            } else {
                let a470 = v0;
                let a471 = 47u64;
                if nat_eq(a470, a471) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h472, _, _, _, _, _, _, _, _, _, _) => {
                h472
            }
        }, match v1.clone() {
            Adt1::C0(h473, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h473
            }
        }, match v1 {
            Adt1::C0(_, h474, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h474
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f12(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m475 = f31(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m475 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f18(v2.clone(), 1u64), f18(v4, 1u64), Adt4::C10(v5, v6, {
                let m476 = v3;
                match m476 {
                    None => {
                        f18(v2, 1u64)
                    }
                    Some(v12) => {
                        f18(v12, 1u64)
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

pub fn f13(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m477 = v3;
    match m477 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f32(v0, v1)?, f32(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f33(v0, v1, v2.clone())?, f32(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f34(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f35(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f36(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f35(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f37(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f38(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f39(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f32(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f40(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f32(v0, v1)?, f32(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f41(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f32(v0, v1)?, f32(v0, v2)?))
        }
    }
}

pub fn f14(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a480 = match v1.clone() {
        Adt2::C0(h478, _, _, _, _, _, _, _, _, _, _) => {
            h478
        }
    };
    let a481 = match v3.clone() {
        Adt2::C0(h479, _, _, _, _, _, _, _, _, _, _) => {
            h479
        }
    };
    if if nat_eq(a480, a481) {
        let a482 = v0;
        let a483 = 33u64;
        if nat_eq(a482, a483) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m485 = f17(match v1.clone() {
            Adt2::C0(h484, _, _, _, _, _, _, _, _, _, _) => {
                h484
            }
        });
        match m485 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h486, _) => {
                        h486
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a491 = {
                    let a488 = match v1.clone() {
                        Adt2::C0(_, h487, _, _, _, _, _, _, _, _, _) => {
                            h487
                        }
                    };
                    let a489 = v5;
                    let a490 = 1u64;
                    nat_rem(a488, a489, a490)
                };
                let a492 = 0u64;
                if if if nat_eq(a491, a492) {
                    let a497 = {
                        let a494 = match v3.clone() {
                            Adt2::C0(_, h493, _, _, _, _, _, _, _, _, _) => {
                                h493
                            }
                        };
                        let a495 = v5;
                        let a496 = 1u64;
                        nat_rem(a494, a495, a496)
                    };
                    let a498 = 0u64;
                    nat_eq(a497, a498)
                } else {
                    false
                } {
                    let a499 = v6;
                    let a500 = f16(v3.clone(), v5)?;
                    nat_eq(a499, a500)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h501) => {
                            h501
                        }
                    }, f18(v1, v5), f18(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f15(v0: u64) -> bool {
    let a502 = v0;
    let a503 = 0u64;
    if if nat_eq(a502, a503) {
        true
    } else {
        let a504 = v0;
        let a505 = 1u64;
        nat_eq(a504, a505)
    } {
        true
    } else {
        let a506 = v0;
        let a507 = 30u64;
        nat_eq(a506, a507)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> R<u64> {
    let a519 = {
        let a513 = {
            let a509 = match v0.clone() {
                Adt2::C0(_, h508, _, _, _, _, _, _, _, _, _) => {
                    h508
                }
            };
            let a510 = v1;
            let a511 = 0u64;
            nat_quot(a509, a510, a511)
        };
        let a514 = match v0.clone() {
            Adt2::C0(_, _, h512, _, _, _, _, _, _, _, _) => {
                h512
            }
        };
        nat_mul(a513, a514)?
    };
    let a520 = {
        let a517 = match v0.clone() {
            Adt2::C0(_, _, _, h515, _, _, _, _, _, _, _) => {
                h515
            }
        };
        let a518 = match v0 {
            Adt2::C0(_, _, _, _, h516, _, _, _, _, _, _) => {
                h516
            }
        };
        nat_mul(a517, a518)?
    };
    nat_mul(a519, a520)
}

pub fn f17(v0: u64) -> Adt7 {
    let a521 = v0;
    let a522 = 0u64;
    if nat_eq(a521, a522) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a523 = v0;
        let a524 = 1u64;
        if nat_eq(a523, a524) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a525 = v0;
            let a526 = 2u64;
            if nat_eq(a525, a526) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a527 = v0;
                let a528 = 3u64;
                if nat_eq(a527, a528) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a529 = v0;
                    let a530 = 6u64;
                    if nat_eq(a529, a530) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a531 = v0;
                        let a532 = 7u64;
                        if nat_eq(a531, a532) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a533 = v0;
                            let a534 = 8u64;
                            if nat_eq(a533, a534) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a535 = v0;
                                let a536 = 9u64;
                                if nat_eq(a535, a536) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a537 = v0;
                                    let a538 = 10u64;
                                    if nat_eq(a537, a538) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a539 = v0;
                                        let a540 = 11u64;
                                        if nat_eq(a539, a540) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a541 = v0;
                                            let a542 = 12u64;
                                            if nat_eq(a541, a542) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a543 = v0;
                                                let a544 = 13u64;
                                                if nat_eq(a543, a544) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a545 = v0;
                                                    let a546 = 14u64;
                                                    if nat_eq(a545, a546) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a547 = v0;
                                                        let a548 = 15u64;
                                                        if nat_eq(a547, a548) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a549 = v0;
                                                            let a550 = 16u64;
                                                            if nat_eq(a549, a550) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a551 = v0;
                                                                let a552 = 17u64;
                                                                if nat_eq(a551, a552) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a553 = v0;
                                                                    let a554 = 18u64;
                                                                    if nat_eq(a553, a554) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a555 = v0;
                                                                        let a556 = 19u64;
                                                                        if nat_eq(a555, a556) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a557 = v0;
                                                                            let a558 = 20u64;
                                                                            if nat_eq(a557, a558) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a559 = v0;
                                                                                let a560 = 21u64;
                                                                                if nat_eq(a559, a560) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a561 = v0;
                                                                                    let a562 = 22u64;
                                                                                    if nat_eq(a561, a562) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a563 = v0;
                                                                                        let a564 = 23u64;
                                                                                        if nat_eq(a563, a564) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a565 = v0;
                                                                                            let a566 = 24u64;
                                                                                            if nat_eq(a565, a566) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a567 = v0;
                                                                                                let a568 = 25u64;
                                                                                                if nat_eq(a567, a568) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a569 = v0;
                                                                                                    let a570 = 26u64;
                                                                                                    if nat_eq(a569, a570) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a571 = v0;
                                                                                                        let a572 = 27u64;
                                                                                                        if nat_eq(a571, a572) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a573 = v0;
                                                                                                            let a574 = 28u64;
                                                                                                            if nat_eq(a573, a574) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a575 = v0;
                                                                                                                let a576 = 29u64;
                                                                                                                if nat_eq(a575, a576) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a577 = v0;
                                                                                                                    let a578 = 30u64;
                                                                                                                    if nat_eq(a577, a578) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a579 = v0;
                                                                                                                        let a580 = 34u64;
                                                                                                                        if nat_eq(a579, a580) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a581 = v0;
                                                                                                                            let a582 = 35u64;
                                                                                                                            if nat_eq(a581, a582) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a583 = v0;
                                                                                                                                let a584 = 39u64;
                                                                                                                                if nat_eq(a583, a584) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a585 = v0;
                                                                                                                                    let a586 = 40u64;
                                                                                                                                    if nat_eq(a585, a586) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a587 = v0;
                                                                                                                                        let a588 = 41u64;
                                                                                                                                        if nat_eq(a587, a588) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a589 = v0;
                                                                                                                                            let a590 = 42u64;
                                                                                                                                            if nat_eq(a589, a590) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a591 = v0;
                                                                                                                                                let a592 = 43u64;
                                                                                                                                                if nat_eq(a591, a592) {
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

pub fn f18(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a594 = match v0.clone() {
            Adt2::C0(_, h593, _, _, _, _, _, _, _, _, _) => {
                h593
            }
        };
        let a595 = v1;
        let a596 = 0u64;
        nat_quot(a594, a595, a596)
    }, match v0.clone() {
        Adt2::C0(_, _, h597, _, _, _, _, _, _, _, _) => {
            h597
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h598, _, _, _, _, _, _, _) => {
            h598
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h599, _, _, _, _, _, _) => {
            h599
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h600, _, _, _, _, _) => {
            h600
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h601, _, _, _, _) => {
            h601
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h602, _, _, _) => {
            h602
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h603, _, _) => {
            h603
        }
    })
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f22(v4, v5, v6, v7) {
        f22(v0, v1, v2, v3)
    } else {
        let a607 = {
            let a604 = v0;
            let a605 = v4;
            let a606 = 1u64;
            nat_rem(a604, a605, a606)
        };
        let a608 = 0u64;
        if if if nat_eq(a607, a608) {
            let a612 = {
                let a609 = v1;
                let a610 = v5;
                let a611 = 1u64;
                nat_rem(a609, a610, a611)
            };
            let a613 = 0u64;
            nat_eq(a612, a613)
        } else {
            false
        } {
            let a617 = {
                let a614 = v2;
                let a615 = v6;
                let a616 = 1u64;
                nat_rem(a614, a615, a616)
            };
            let a618 = 0u64;
            nat_eq(a617, a618)
        } else {
            false
        } {
            let a622 = {
                let a619 = v3;
                let a620 = v7;
                let a621 = 1u64;
                nat_rem(a619, a620, a621)
            };
            let a623 = 0u64;
            nat_eq(a622, a623)
        } else {
            false
        }
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a624 = v0;
    let a625 = v1;
    if nat_eq(a624, a625) {
        let a628 = v4;
        let a629 = {
            let a626 = v2;
            let a627 = v3;
            nat_add(a626, a627)?
        };
        Ok(nat_eq(a628, a629))
    } else {
        let a630 = v2;
        let a631 = v3;
        if nat_eq(a630, a631) {
            let a632 = v4;
            let a633 = v2;
            Ok(nat_eq(a632, a633))
        } else {
            Ok(false)
        }
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a636 = {
            let a634 = v0;
            let a635 = f42(v1);
            nat_add(a634, a635)?
        };
        let a637 = f42(v2);
        nat_add(a636, a637)?
    };
    let v5: u64 = {
        let a638 = f43(v1);
        let a639 = f43(v2);
        nat_add(a638, a639)?
    };
    let a640 = v1;
    let a641 = 4294967295u64;
    if if nat_le(a640, a641) {
        let a642 = v2;
        let a643 = 4294967295u64;
        nat_le(a642, a643)
    } else {
        false
    } {
        let a644 = v5;
        let a645 = v4;
        if nat_le(a644, a645) {
            let a648 = v3;
            let a649 = {
                let a646 = v4;
                let a647 = v5;
                nat_sub(a646, a647)
            };
            Ok(nat_eq(a648, a649))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a650 = v0;
    let a651 = 0u64;
    if if if nat_eq(a650, a651) {
        true
    } else {
        let a652 = v1;
        let a653 = 0u64;
        nat_eq(a652, a653)
    } {
        true
    } else {
        let a654 = v2;
        let a655 = 0u64;
        nat_eq(a654, a655)
    } {
        true
    } else {
        let a656 = v3;
        let a657 = 0u64;
        nat_eq(a656, a657)
    }
}

pub fn f23(v0: u64, v1: u64) -> R<bool> {
    let a658 = v1;
    let a659 = 4294967295u64;
    if nat_le(a658, a659) {
        let a662 = {
            let a660 = f42(v1);
            let a661 = f43(v1);
            nat_add(a660, a661)?
        };
        let a663 = v0;
        Ok(nat_lt(a662, a663))
    } else {
        Ok(false)
    }
}

pub fn f24(v0: Adt2, v1: Adt2) -> bool {
    let a666 = match v0.clone() {
        Adt2::C0(_, h664, _, _, _, _, _, _, _, _, _) => {
            h664
        }
    };
    let a667 = match v1.clone() {
        Adt2::C0(_, h665, _, _, _, _, _, _, _, _, _) => {
            h665
        }
    };
    if if if nat_eq(a666, a667) {
        let a670 = match v0.clone() {
            Adt2::C0(_, _, h668, _, _, _, _, _, _, _, _) => {
                h668
            }
        };
        let a671 = match v1.clone() {
            Adt2::C0(_, _, h669, _, _, _, _, _, _, _, _) => {
                h669
            }
        };
        nat_eq(a670, a671)
    } else {
        false
    } {
        let a674 = match v0.clone() {
            Adt2::C0(_, _, _, h672, _, _, _, _, _, _, _) => {
                h672
            }
        };
        let a675 = match v1.clone() {
            Adt2::C0(_, _, _, h673, _, _, _, _, _, _, _) => {
                h673
            }
        };
        nat_eq(a674, a675)
    } else {
        false
    } {
        let a678 = match v0 {
            Adt2::C0(_, _, _, _, h676, _, _, _, _, _, _) => {
                h676
            }
        };
        let a679 = match v1 {
            Adt2::C0(_, _, _, _, h677, _, _, _, _, _, _) => {
                h677
            }
        };
        nat_eq(a678, a679)
    } else {
        false
    }
}

pub fn f25(v0: Adt2) -> R<bool> {
    let a681 = match v0.clone() {
        Adt2::C0(_, h680, _, _, _, _, _, _, _, _, _) => {
            h680
        }
    };
    let a682 = 1u64;
    if if if if nat_eq(a681, a682) {
        true
    } else {
        let a684 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h683, _, _, _, _, _) => {
                h683
            }
        };
        let a685 = 4u64;
        nat_eq(a684, a685)
    } {
        let a687 = match v0.clone() {
            Adt2::C0(_, _, h686, _, _, _, _, _, _, _, _) => {
                h686
            }
        };
        let a688 = 1u64;
        if nat_eq(a687, a688) {
            true
        } else {
            let a693 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h689, _, _, _, _) => {
                    h689
                }
            };
            let a694 = {
                let a691 = match v0.clone() {
                    Adt2::C0(_, h690, _, _, _, _, _, _, _, _, _) => {
                        h690
                    }
                };
                let a692 = 4u64;
                nat_mul(a691, a692)?
            };
            nat_eq(a693, a694)
        }
    } else {
        false
    } {
        let a696 = match v0.clone() {
            Adt2::C0(_, _, _, h695, _, _, _, _, _, _, _) => {
                h695
            }
        };
        let a697 = 1u64;
        if nat_eq(a696, a697) {
            true
        } else {
            let a705 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h698, _, _, _) => {
                    h698
                }
            };
            let a706 = {
                let a703 = {
                    let a701 = match v0.clone() {
                        Adt2::C0(_, h699, _, _, _, _, _, _, _, _, _) => {
                            h699
                        }
                    };
                    let a702 = match v0.clone() {
                        Adt2::C0(_, _, h700, _, _, _, _, _, _, _, _) => {
                            h700
                        }
                    };
                    nat_mul(a701, a702)?
                };
                let a704 = 4u64;
                nat_mul(a703, a704)?
            };
            nat_eq(a705, a706)
        }
    } else {
        false
    } {
        let a708 = match v0.clone() {
            Adt2::C0(_, _, _, _, h707, _, _, _, _, _, _) => {
                h707
            }
        };
        let a709 = 1u64;
        if nat_eq(a708, a709) {
            Ok(true)
        } else {
            let a720 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h710, _, _) => {
                    h710
                }
            };
            let a721 = {
                let a718 = {
                    let a716 = {
                        let a713 = match v0.clone() {
                            Adt2::C0(_, h711, _, _, _, _, _, _, _, _, _) => {
                                h711
                            }
                        };
                        let a714 = match v0.clone() {
                            Adt2::C0(_, _, h712, _, _, _, _, _, _, _, _) => {
                                h712
                            }
                        };
                        nat_mul(a713, a714)?
                    };
                    let a717 = match v0 {
                        Adt2::C0(_, _, _, h715, _, _, _, _, _, _, _) => {
                            h715
                        }
                    };
                    nat_mul(a716, a717)?
                };
                let a719 = 4u64;
                nat_mul(a718, a719)?
            };
            Ok(nat_eq(a720, a721))
        }
    } else {
        Ok(false)
    }
}

pub fn f26(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f44(v0.clone(), false)?;
    let v6: u64 = f44(v0, true)?;
    let a722 = v6;
    let a723 = v1;
    if if nat_le(a722, a723) {
        let a730 = {
            let a728 = {
                let a724 = v1;
                let a725 = v5;
                nat_add(a724, a725)?
            };
            let a729 = {
                let a726 = v3;
                let a727 = 0u64;
                if nat_eq(a726, a727) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a728, a729)?
        };
        let a731 = v2;
        nat_le(a730, a731)
    } else {
        false
    } {
        if v4 {
            let a736 = {
                let a734 = {
                    let a732 = v1;
                    let a733 = v5;
                    nat_add(a732, a733)?
                };
                let a735 = v6;
                nat_sub(a734, a735)
            };
            let a737 = v2;
            Ok(nat_lt(a736, a737))
        } else {
            let a742 = {
                let a740 = {
                    let a738 = v1;
                    let a739 = v5;
                    nat_add(a738, a739)?
                };
                let a741 = v6;
                nat_sub(a740, a741)
            };
            let a743 = v2;
            Ok(nat_le(a742, a743))
        }
    } else {
        Ok(false)
    }
}

pub fn f27(v0: u64, v1: u64) -> bool {
    let a744 = v0;
    let a745 = 0u64;
    if if nat_eq(a744, a745) {
        true
    } else {
        let a746 = v0;
        let a747 = 1u64;
        nat_eq(a746, a747)
    } {
        let a748 = v1;
        let a749 = 4294967296u64;
        nat_lt(a748, a749)
    } else {
        false
    }
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let a754 = {
        let a752 = v0;
        let a753 = {
            let a750 = v1;
            let a751 = 1u64;
            nat_sub(a750, a751)
        };
        nat_add(a752, a753)?
    };
    let a755 = v1;
    let a756 = 0u64;
    Ok(nat_quot(a754, a755, a756))
}

pub fn f29(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f22(match v0.clone() {
        Adt2::C0(_, h757, _, _, _, _, _, _, _, _, _) => {
            h757
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h758, _, _, _, _, _, _, _, _) => {
            h758
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h759, _, _, _, _, _, _, _) => {
            h759
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h760, _, _, _, _, _, _) => {
            h760
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a763 = match v1.clone() {
                Adt2::C0(_, h761, _, _, _, _, _, _, _, _, _) => {
                    h761
                }
            };
            let a764 = match v1.clone() {
                Adt2::C0(_, _, h762, _, _, _, _, _, _, _, _) => {
                    h762
                }
            };
            nat_mul(a763, a764)?
        };
        let v5: u64 = {
            let a766 = v4;
            let a767 = match v1.clone() {
                Adt2::C0(_, _, _, h765, _, _, _, _, _, _, _) => {
                    h765
                }
            };
            nat_mul(a766, a767)?
        };
        let a780 = {
            let a778 = {
                let a769 = match v0.clone() {
                    Adt2::C0(_, h768, _, _, _, _, _, _, _, _, _) => {
                        h768
                    }
                };
                let a770 = 1u64;
                nat_sub(a769, a770)
            };
            let a779 = {
                let a776 = f45(match v0.clone() {
                    Adt2::C0(_, _, h771, _, _, _, _, _, _, _, _) => {
                        h771
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h772, _, _, _, _, _, _, _, _, _) => {
                        h772
                    }
                })?;
                let a777 = f45(match v0 {
                    Adt2::C0(_, _, _, h773, _, _, _, _, _, _, _) => {
                        h773
                    }
                }, v2, {
                    let a774 = v3;
                    let a775 = v5;
                    nat_mul(a774, a775)?
                }, v4)?;
                nat_add(a776, a777)?
            };
            nat_add(a778, a779)?
        };
        let a781 = f16(v1, 1u64)?;
        Ok(nat_lt(a780, a781))
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a782 = v0;
    let a783 = 2147483648u64;
    if if if if nat_lt(a782, a783) {
        let a784 = v1;
        let a785 = 2147483648u64;
        nat_lt(a784, a785)
    } else {
        false
    } {
        let a788 = {
            let a786 = v0;
            let a787 = v1;
            nat_mul(a786, a787)?
        };
        let a789 = 2147483648u64;
        nat_lt(a788, a789)
    } else {
        false
    } {
        let a794 = {
            let a792 = {
                let a790 = v0;
                let a791 = 1u64;
                nat_sub(a790, a791)
            };
            let a793 = v2;
            nat_mul(a792, a793)?
        };
        let a795 = 2147483648u64;
        nat_lt(a794, a795)
    } else {
        false
    } {
        let a800 = {
            let a798 = {
                let a796 = v1;
                let a797 = 1u64;
                nat_sub(a796, a797)
            };
            let a799 = v2;
            nat_mul(a798, a799)?
        };
        let a801 = 2147483648u64;
        Ok(nat_lt(a800, a801))
    } else {
        Ok(false)
    }
}

pub fn f31(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m802 = v3;
    match m802 {
        None => {
            f46(v0, v1, match v2.clone() {
                Adt2::C0(h803, _, _, _, _, _, _, _, _, _, _) => {
                    h803
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h804, _, _, _, _, _, _, _, _, _, _) => {
                    h804
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h805, _, _, _, _, _, _, _, _, _) => {
                    h805
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h806, _, _, _, _, _, _, _, _) => {
                    h806
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h807, _, _, _, _, _, _, _) => {
                    h807
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h808, _, _, _, _, _, _) => {
                    h808
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h809, _, _, _, _, _, _, _, _, _) => {
                    h809
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h810, _, _, _, _, _, _, _, _) => {
                    h810
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h811, _, _, _, _, _, _, _) => {
                    h811
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h812, _, _, _, _, _, _) => {
                    h812
                }
            })
        }
        Some(v5) => {
            f46(v0, v1, match v2.clone() {
                Adt2::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
                    h813
                }
            }, match v5.clone() {
                Adt2::C0(h814, _, _, _, _, _, _, _, _, _, _) => {
                    h814
                }
            }, match v4.clone() {
                Adt2::C0(h815, _, _, _, _, _, _, _, _, _, _) => {
                    h815
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h816, _, _, _, _, _, _, _, _, _) => {
                    h816
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h817, _, _, _, _, _, _, _, _) => {
                    h817
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h818, _, _, _, _, _, _, _) => {
                    h818
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h819, _, _, _, _, _, _) => {
                    h819
                }
            }, match v5.clone() {
                Adt2::C0(_, h820, _, _, _, _, _, _, _, _, _) => {
                    h820
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h821, _, _, _, _, _, _, _, _) => {
                    h821
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h822, _, _, _, _, _, _, _) => {
                    h822
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h823, _, _, _, _, _, _) => {
                    h823
                }
            }, match v4.clone() {
                Adt2::C0(_, h824, _, _, _, _, _, _, _, _, _) => {
                    h824
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h825, _, _, _, _, _, _, _, _) => {
                    h825
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h826, _, _, _, _, _, _, _) => {
                    h826
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h827, _, _, _, _, _, _) => {
                    h827
                }
            })
        }
    }
}

pub fn f32(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a829 = v0;
        let a830 = match v1.clone() {
            Adt3::C0(h828, _, _, _, _, _, _, _) => {
                h828
            }
        };
        let a831 = 0u64;
        nat_quot(a829, a830, a831)
    };
    let v3: u64 = {
        let a833 = v2;
        let a834 = match v1.clone() {
            Adt3::C0(_, h832, _, _, _, _, _, _) => {
                h832
            }
        };
        let a835 = 0u64;
        nat_quot(a833, a834, a835)
    };
    f47({
        let a837 = v0;
        let a838 = match v1.clone() {
            Adt3::C0(h836, _, _, _, _, _, _, _) => {
                h836
            }
        };
        let a839 = 0u64;
        nat_rem(a837, a838, a839)
    }, {
        let a841 = v2;
        let a842 = match v1.clone() {
            Adt3::C0(_, h840, _, _, _, _, _, _) => {
                h840
            }
        };
        let a843 = 0u64;
        nat_rem(a841, a842, a843)
    }, {
        let a845 = v3;
        let a846 = match v1.clone() {
            Adt3::C0(_, _, h844, _, _, _, _, _) => {
                h844
            }
        };
        let a847 = 0u64;
        nat_rem(a845, a846, a847)
    }, {
        let a849 = v3;
        let a850 = match v1.clone() {
            Adt3::C0(_, _, h848, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = 0u64;
        nat_quot(a849, a850, a851)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h852, _, _, _) => {
            h852
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h853, _, _) => {
            h853
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h854, _) => {
            h854
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h855) => {
            h855
        }
    })
}

pub fn f33(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a857 = v0;
        let a858 = match v2.clone() {
            Adt3::C0(h856, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = 0u64;
        nat_quot(a857, a858, a859)
    };
    let v4: u64 = {
        let a861 = v3;
        let a862 = match v2.clone() {
            Adt3::C0(_, h860, _, _, _, _, _, _) => {
                h860
            }
        };
        let a863 = 0u64;
        nat_quot(a861, a862, a863)
    };
    f48({
        let a865 = v0;
        let a866 = match v2.clone() {
            Adt3::C0(h864, _, _, _, _, _, _, _) => {
                h864
            }
        };
        let a867 = 0u64;
        nat_rem(a865, a866, a867)
    }, {
        let a869 = v3;
        let a870 = match v2.clone() {
            Adt3::C0(_, h868, _, _, _, _, _, _) => {
                h868
            }
        };
        let a871 = 0u64;
        nat_rem(a869, a870, a871)
    }, {
        let a873 = v4;
        let a874 = match v2.clone() {
            Adt3::C0(_, _, h872, _, _, _, _, _) => {
                h872
            }
        };
        let a875 = 0u64;
        nat_rem(a873, a874, a875)
    }, {
        let a877 = v4;
        let a878 = match v2 {
            Adt3::C0(_, _, h876, _, _, _, _, _) => {
                h876
            }
        };
        let a879 = 0u64;
        nat_quot(a877, a878, a879)
    }, match v1.clone() {
        Adt3::C0(h880, _, _, _, _, _, _, _) => {
            h880
        }
    }, match v1.clone() {
        Adt3::C0(_, h881, _, _, _, _, _, _) => {
            h881
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h882, _, _, _, _, _) => {
            h882
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h883, _, _, _, _) => {
            h883
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h884, _, _, _) => {
            h884
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h885, _, _) => {
            h885
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h886, _) => {
            h886
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h887) => {
            h887
        }
    })
}

pub fn f34(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a889 = v0;
        let a890 = match v3.clone() {
            Adt3::C0(h888, _, _, _, _, _, _, _) => {
                h888
            }
        };
        let a891 = 0u64;
        nat_quot(a889, a890, a891)
    };
    let v6: u64 = {
        let a893 = v5;
        let a894 = match v3.clone() {
            Adt3::C0(_, h892, _, _, _, _, _, _) => {
                h892
            }
        };
        let a895 = 0u64;
        nat_quot(a893, a894, a895)
    };
    let v7: u64 = {
        let a897 = v0;
        let a898 = match v3.clone() {
            Adt3::C0(h896, _, _, _, _, _, _, _) => {
                h896
            }
        };
        let a899 = 0u64;
        nat_rem(a897, a898, a899)
    };
    let v8: u64 = {
        let a901 = v5;
        let a902 = match v3.clone() {
            Adt3::C0(_, h900, _, _, _, _, _, _) => {
                h900
            }
        };
        let a903 = 0u64;
        nat_rem(a901, a902, a903)
    };
    let v9: u64 = {
        let a905 = v6;
        let a906 = match v3.clone() {
            Adt3::C0(_, _, h904, _, _, _, _, _) => {
                h904
            }
        };
        let a907 = 0u64;
        nat_rem(a905, a906, a907)
    };
    let v10: u64 = {
        let a909 = v6;
        let a910 = match v3.clone() {
            Adt3::C0(_, _, h908, _, _, _, _, _) => {
                h908
            }
        };
        let a911 = 0u64;
        nat_quot(a909, a910, a911)
    };
    let v11: u64 = {
        let a912 = v4;
        let a913 = 0u64;
        if nat_eq(a912, a913) {
            match v1.clone() {
                Adt3::C0(h914, _, _, _, _, _, _, _) => {
                    h914
                }
            }
        } else {
            let a915 = v4;
            let a916 = 1u64;
            if nat_eq(a915, a916) {
                match v1.clone() {
                    Adt3::C0(_, h917, _, _, _, _, _, _) => {
                        h917
                    }
                }
            } else {
                let a918 = v4;
                let a919 = 2u64;
                if nat_eq(a918, a919) {
                    match v1.clone() {
                        Adt3::C0(_, _, h920, _, _, _, _, _) => {
                            h920
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h921, _, _, _, _) => {
                            h921
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a928 = v11;
        let a929 = {
            let a922 = v4;
            let a923 = 0u64;
            if nat_eq(a922, a923) {
                v7
            } else {
                let a924 = v4;
                let a925 = 1u64;
                if nat_eq(a924, a925) {
                    v8
                } else {
                    let a926 = v4;
                    let a927 = 2u64;
                    if nat_eq(a926, a927) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a928, a929)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f47(if if v12 {
        let a930 = v4;
        let a931 = 0u64;
        nat_eq(a930, a931)
    } else {
        false
    } {
        let a932 = v7;
        let a933 = v11;
        nat_sub(a932, a933)
    } else {
        v7
    }, if if v12 {
        let a934 = v4;
        let a935 = 1u64;
        nat_eq(a934, a935)
    } else {
        false
    } {
        let a936 = v8;
        let a937 = v11;
        nat_sub(a936, a937)
    } else {
        v8
    }, if if v12 {
        let a938 = v4;
        let a939 = 2u64;
        nat_eq(a938, a939)
    } else {
        false
    } {
        let a940 = v9;
        let a941 = v11;
        nat_sub(a940, a941)
    } else {
        v9
    }, if if v12 {
        let a942 = v4;
        let a943 = 3u64;
        nat_eq(a942, a943)
    } else {
        false
    } {
        let a944 = v10;
        let a945 = v11;
        nat_sub(a944, a945)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h946, _, _, _) => {
            h946
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h947, _, _) => {
            h947
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h948, _) => {
            h948
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h949) => {
            h949
        }
    })?, f32(v0, v3)?))
}

pub fn f35(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f49(v0, v2.clone(), 0u64);
    let v9: u64 = f49(v0, v2.clone(), 1u64);
    let v10: u64 = f49(v0, v2.clone(), 2u64);
    let v11: u64 = f49(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f50(v1.clone(), f51(v8, v3, match v1.clone() {
            Adt3::C0(h950, _, _, _, _, _, _, _) => {
                h950
            }
        })?, f51(v9, v4, match v1.clone() {
            Adt3::C0(_, h951, _, _, _, _, _, _) => {
                h951
            }
        })?, f51(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h952, _, _, _, _, _) => {
                h952
            }
        })?, f51(v11, v6, match v1 {
            Adt3::C0(_, _, _, h953, _, _, _, _) => {
                h953
            }
        })?)?, f32(v0, v2)?))
    } else {
        if if if if f52(v8, v3, match v1.clone() {
            Adt3::C0(h954, _, _, _, _, _, _, _) => {
                h954
            }
        })? {
            f52(v9, v4, match v1.clone() {
                Adt3::C0(_, h955, _, _, _, _, _, _) => {
                    h955
                }
            })?
        } else {
            false
        } {
            f52(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h956, _, _, _, _, _) => {
                    h956
                }
            })?
        } else {
            false
        } {
            f52(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h957, _, _, _, _) => {
                    h957
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f50(v1, f53(v8, v3)?, f53(v9, v4)?, f53(v10, v5)?, f53(v11, v6)?)?, f32(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f32(v0, v2)?))
        }
    }
}

pub fn f36(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f49(v0, v2.clone(), 0u64);
    let v5: u64 = f49(v0, v2.clone(), 1u64);
    let v6: u64 = f49(v0, v2.clone(), 2u64);
    let v7: u64 = f49(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f50(v1.clone(), f54(v4, v3, match v1 {
        Adt3::C0(h958, _, _, _, _, _, _, _) => {
            h958
        }
    })?, v5, v6, v7)?, f32(v0, v2)?))
}

pub fn f37(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a959 = v0;
    let a960 = v3;
    if nat_lt(a959, a960) {
        Ok(Adt5::C1(false, {
            let a961 = v0;
            let a962 = 4u64;
            nat_mul(a961, a962)?
        }, {
            let a963 = v0;
            let a964 = 4u64;
            nat_mul(a963, a964)?
        }))
    } else {
        let v6: u64 = {
            let a965 = v0;
            let a966 = v3;
            nat_sub(a965, a966)
        };
        let v7: u64 = f32(v6, v1)?;
        let v8: u64 = f55(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f38(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a967 = v0;
    let a968 = v3;
    if nat_lt(a967, a968) {
        Ok(Adt5::C1(true, 0u64, f32(v0, v2)?))
    } else {
        let v4: u64 = {
            let a969 = v0;
            let a970 = v3;
            nat_sub(a969, a970)
        };
        let v5: u64 = f33(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f32(v4, v1)?, v5))
    }
}

pub fn f39(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a972 = v1;
        let a973 = match v3.clone() {
            Adt3::C0(h971, _, _, _, _, _, _, _) => {
                h971
            }
        };
        let a974 = 0u64;
        nat_rem(a972, a973, a974)
    };
    let v5: u64 = {
        let a980 = {
            let a976 = v1;
            let a977 = match v3.clone() {
                Adt3::C0(h975, _, _, _, _, _, _, _) => {
                    h975
                }
            };
            let a978 = 0u64;
            nat_quot(a976, a977, a978)
        };
        let a981 = match v3.clone() {
            Adt3::C0(_, h979, _, _, _, _, _, _) => {
                h979
            }
        };
        let a982 = 0u64;
        nat_rem(a980, a981, a982)
    };
    let v6: bool = f56(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f33(v1, v2, v3.clone())?
    } else {
        0u64
    }, f32(v1, v3)?))
}

pub fn f40(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f32(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f33(v0, v5, v2.clone())?
    }, f32(v0, v2)?))
}

pub fn f41(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a984 = v0;
        let a985 = match v2.clone() {
            Adt3::C0(h983, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a986 = 0u64;
        nat_rem(a984, a985, a986)
    };
    let v7: u64 = {
        let a988 = v0;
        let a989 = match v2.clone() {
            Adt3::C0(h987, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a990 = 0u64;
        nat_quot(a988, a989, a990)
    };
    let v8: u64 = {
        let a992 = v7;
        let a993 = match v2.clone() {
            Adt3::C0(_, h991, _, _, _, _, _, _) => {
                h991
            }
        };
        let a994 = 0u64;
        nat_rem(a992, a993, a994)
    };
    let v9: u64 = {
        let a996 = v7;
        let a997 = match v2.clone() {
            Adt3::C0(_, h995, _, _, _, _, _, _) => {
                h995
            }
        };
        let a998 = 0u64;
        nat_quot(a996, a997, a998)
    };
    let v10: u64 = {
        let a1000 = v9;
        let a1001 = match v2.clone() {
            Adt3::C0(_, _, h999, _, _, _, _, _) => {
                h999
            }
        };
        let a1002 = 0u64;
        nat_rem(a1000, a1001, a1002)
    };
    let v11: u64 = {
        let a1004 = v9;
        let a1005 = match v2.clone() {
            Adt3::C0(_, _, h1003, _, _, _, _, _) => {
                h1003
            }
        };
        let a1006 = 0u64;
        nat_quot(a1004, a1005, a1006)
    };
    let v12: u64 = f32(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1015 = {
                let a1010 = {
                    let a1007 = v10;
                    let a1008 = v4;
                    let a1009 = 0u64;
                    nat_quot(a1007, a1008, a1009)
                };
                let a1011 = v5;
                nat_mul(a1010, a1011)?
            };
            let a1016 = {
                let a1012 = v8;
                let a1013 = v4;
                let a1014 = 0u64;
                nat_quot(a1012, a1013, a1014)
            };
            nat_add(a1015, a1016)?
        };
        let v14: u64 = {
            let a1036 = v6;
            let a1037 = {
                let a1034 = match v1.clone() {
                    Adt3::C0(h1017, _, _, _, _, _, _, _) => {
                        h1017
                    }
                };
                let a1035 = {
                    let a1032 = {
                        let a1018 = v8;
                        let a1019 = v4;
                        let a1020 = 0u64;
                        nat_rem(a1018, a1019, a1020)
                    };
                    let a1033 = {
                        let a1030 = match v1.clone() {
                            Adt3::C0(_, h1021, _, _, _, _, _, _) => {
                                h1021
                            }
                        };
                        let a1031 = {
                            let a1028 = {
                                let a1022 = v10;
                                let a1023 = v4;
                                let a1024 = 0u64;
                                nat_rem(a1022, a1023, a1024)
                            };
                            let a1029 = {
                                let a1026 = match v1.clone() {
                                    Adt3::C0(_, _, h1025, _, _, _, _, _) => {
                                        h1025
                                    }
                                };
                                let a1027 = v13;
                                nat_mul(a1026, a1027)?
                            };
                            nat_add(a1028, a1029)?
                        };
                        nat_mul(a1030, a1031)?
                    };
                    nat_add(a1032, a1033)?
                };
                nat_mul(a1034, a1035)?
            };
            nat_add(a1036, a1037)?
        };
        Ok(Adt5::C1(false, f32(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1043 = {
                let a1041 = {
                    let a1038 = v11;
                    let a1039 = v5;
                    let a1040 = 0u64;
                    nat_rem(a1038, a1039, a1040)
                };
                let a1042 = v4;
                nat_mul(a1041, a1042)?
            };
            let a1044 = v8;
            nat_add(a1043, a1044)?
        };
        let v16: u64 = {
            let a1050 = {
                let a1048 = {
                    let a1045 = v11;
                    let a1046 = v5;
                    let a1047 = 0u64;
                    nat_quot(a1045, a1046, a1047)
                };
                let a1049 = v4;
                nat_mul(a1048, a1049)?
            };
            let a1051 = v10;
            nat_add(a1050, a1051)?
        };
        let a1053 = v15;
        let a1054 = match v1.clone() {
            Adt3::C0(_, h1052, _, _, _, _, _, _) => {
                h1052
            }
        };
        if if nat_lt(a1053, a1054) {
            let a1056 = v16;
            let a1057 = match v1.clone() {
                Adt3::C0(_, _, h1055, _, _, _, _, _) => {
                    h1055
                }
            };
            nat_lt(a1056, a1057)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f32({
                let a1066 = v6;
                let a1067 = {
                    let a1064 = match v1.clone() {
                        Adt3::C0(h1058, _, _, _, _, _, _, _) => {
                            h1058
                        }
                    };
                    let a1065 = {
                        let a1062 = v15;
                        let a1063 = {
                            let a1060 = match v1.clone() {
                                Adt3::C0(_, h1059, _, _, _, _, _, _) => {
                                    h1059
                                }
                            };
                            let a1061 = v16;
                            nat_mul(a1060, a1061)?
                        };
                        nat_add(a1062, a1063)?
                    };
                    nat_mul(a1064, a1065)?
                };
                nat_add(a1066, a1067)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f42(v0: u64) -> u64 {
    let a1068 = v0;
    let a1069 = 2147483648u64;
    if nat_lt(a1068, a1069) {
        v0
    } else {
        0u64
    }
}

pub fn f43(v0: u64) -> u64 {
    let a1070 = v0;
    let a1071 = 2147483648u64;
    if nat_lt(a1070, a1071) {
        0u64
    } else {
        let a1072 = 4294967296u64;
        let a1073 = v0;
        nat_sub(a1072, a1073)
    }
}

pub fn f44(v0: Adt3, v1: bool) -> R<u64> {
    f57(v0.clone(), {
        let a1075 = match v0.clone() {
            Adt3::C0(h1074, _, _, _, _, _, _, _) => {
                h1074
            }
        };
        let a1076 = 1u64;
        nat_sub(a1075, a1076)
    }, {
        let a1078 = match v0.clone() {
            Adt3::C0(_, h1077, _, _, _, _, _, _) => {
                h1077
            }
        };
        let a1079 = 1u64;
        nat_sub(a1078, a1079)
    }, {
        let a1081 = match v0.clone() {
            Adt3::C0(_, _, h1080, _, _, _, _, _) => {
                h1080
            }
        };
        let a1082 = 1u64;
        nat_sub(a1081, a1082)
    }, {
        let a1084 = match v0 {
            Adt3::C0(_, _, _, h1083, _, _, _, _) => {
                h1083
            }
        };
        let a1085 = 1u64;
        nat_sub(a1084, a1085)
    }, v1)
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1086 = v0;
    let a1087 = 0u64;
    if nat_eq(a1086, a1087) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1088 = v0;
            let a1089 = 1u64;
            nat_sub(a1088, a1089)
        };
        let v5: u64 = {
            let a1090 = v4;
            let a1091 = v1;
            let a1092 = 0u64;
            nat_quot(a1090, a1091, a1092)
        };
        let v6: u64 = {
            let a1100 = {
                let a1093 = v5;
                let a1094 = v2;
                nat_mul(a1093, a1094)?
            };
            let a1101 = {
                let a1098 = {
                    let a1095 = v4;
                    let a1096 = v1;
                    let a1097 = 0u64;
                    nat_rem(a1095, a1096, a1097)
                };
                let a1099 = v3;
                nat_mul(a1098, a1099)?
            };
            nat_add(a1100, a1101)?
        };
        let a1102 = v5;
        let a1103 = 0u64;
        if nat_eq(a1102, a1103) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1112 = {
                    let a1106 = {
                        let a1104 = v5;
                        let a1105 = 1u64;
                        nat_sub(a1104, a1105)
                    };
                    let a1107 = v2;
                    nat_mul(a1106, a1107)?
                };
                let a1113 = {
                    let a1110 = {
                        let a1108 = v1;
                        let a1109 = 1u64;
                        nat_sub(a1108, a1109)
                    };
                    let a1111 = v3;
                    nat_mul(a1110, a1111)?
                };
                nat_add(a1112, a1113)?
            };
            let a1114 = v6;
            let a1115 = v7;
            if nat_lt(a1114, a1115) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1116 = f58(v0, v1);
    match m1116 {
        None => {
            if v5 {
                let m1117 = f59(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1117 {
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
                let a1118 = 4u64;
                let a1119 = v28;
                nat_le(a1118, a1119)
            };
            let a1120 = v2;
            let a1121 = 0u64;
            if if if if if nat_eq(a1120, a1121) {
                let a1122 = v4;
                let a1123 = 0u64;
                nat_eq(a1122, a1123)
            } else {
                false
            } {
                let a1124 = v6;
                let a1125 = v14;
                if nat_eq(a1124, a1125) {
                    let a1126 = v7;
                    let a1127 = v15;
                    nat_eq(a1126, a1127)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1128 = v8;
                let a1129 = v16;
                if nat_eq(a1128, a1129) {
                    let a1130 = v9;
                    let a1131 = v17;
                    nat_eq(a1130, a1131)
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
                        let a1132 = v3;
                        let a1133 = 0u64;
                        nat_eq(a1132, a1133)
                    } else {
                        false
                    } {
                        f19(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1138 = {
                        let a1134 = v6;
                        let a1135 = v7;
                        nat_mul(a1134, a1135)?
                    };
                    let a1139 = {
                        let a1136 = v8;
                        let a1137 = v9;
                        nat_mul(a1136, a1137)?
                    };
                    nat_mul(a1138, a1139)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1152 = {
        let a1148 = {
            let a1144 = {
                let a1140 = v0;
                let a1141 = v4;
                nat_mul(a1140, a1141)?
            };
            let a1145 = {
                let a1142 = v1;
                let a1143 = v5;
                nat_mul(a1142, a1143)?
            };
            nat_add(a1144, a1145)?
        };
        let a1149 = {
            let a1146 = v2;
            let a1147 = v6;
            nat_mul(a1146, a1147)?
        };
        nat_add(a1148, a1149)?
    };
    let a1153 = {
        let a1150 = v3;
        let a1151 = v7;
        nat_mul(a1150, a1151)?
    };
    nat_add(a1152, a1153)
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f47({
        let a1154 = v0;
        let a1155 = v4;
        let a1156 = 0u64;
        nat_rem(a1154, a1155, a1156)
    }, {
        let a1157 = v1;
        let a1158 = v5;
        let a1159 = 0u64;
        nat_rem(a1157, a1158, a1159)
    }, {
        let a1160 = v2;
        let a1161 = v6;
        let a1162 = 0u64;
        nat_rem(a1160, a1161, a1162)
    }, {
        let a1163 = v3;
        let a1164 = v7;
        let a1165 = 0u64;
        nat_rem(a1163, a1164, a1165)
    }, v8, v9, v10, v11)
}

pub fn f49(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1167 = v0;
        let a1168 = match v1.clone() {
            Adt3::C0(h1166, _, _, _, _, _, _, _) => {
                h1166
            }
        };
        let a1169 = 0u64;
        nat_quot(a1167, a1168, a1169)
    };
    let v4: u64 = {
        let a1171 = v3;
        let a1172 = match v1.clone() {
            Adt3::C0(_, h1170, _, _, _, _, _, _) => {
                h1170
            }
        };
        let a1173 = 0u64;
        nat_quot(a1171, a1172, a1173)
    };
    let a1174 = v2;
    let a1175 = 0u64;
    if nat_eq(a1174, a1175) {
        let a1177 = v0;
        let a1178 = match v1 {
            Adt3::C0(h1176, _, _, _, _, _, _, _) => {
                h1176
            }
        };
        let a1179 = 0u64;
        nat_rem(a1177, a1178, a1179)
    } else {
        let a1180 = v2;
        let a1181 = 1u64;
        if nat_eq(a1180, a1181) {
            let a1183 = v3;
            let a1184 = match v1 {
                Adt3::C0(_, h1182, _, _, _, _, _, _) => {
                    h1182
                }
            };
            let a1185 = 0u64;
            nat_rem(a1183, a1184, a1185)
        } else {
            let a1186 = v2;
            let a1187 = 2u64;
            if nat_eq(a1186, a1187) {
                let a1189 = v4;
                let a1190 = match v1 {
                    Adt3::C0(_, _, h1188, _, _, _, _, _) => {
                        h1188
                    }
                };
                let a1191 = 0u64;
                nat_rem(a1189, a1190, a1191)
            } else {
                let a1193 = v4;
                let a1194 = match v1 {
                    Adt3::C0(_, _, h1192, _, _, _, _, _) => {
                        h1192
                    }
                };
                let a1195 = 0u64;
                nat_quot(a1193, a1194, a1195)
            }
        }
    }
}

pub fn f50(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f47(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1196, _, _, _) => {
            h1196
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1197, _, _) => {
            h1197
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1198, _) => {
            h1198
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1199) => {
            h1199
        }
    })
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1200 = v1;
    let a1201 = 2147483648u64;
    if nat_lt(a1200, a1201) {
        let a1212 = {
            let a1210 = {
                let a1205 = {
                    let a1202 = v0;
                    let a1203 = v2;
                    let a1204 = 0u64;
                    nat_rem(a1202, a1203, a1204)
                };
                let a1206 = v2;
                nat_add(a1205, a1206)?
            };
            let a1211 = {
                let a1207 = v1;
                let a1208 = v2;
                let a1209 = 0u64;
                nat_rem(a1207, a1208, a1209)
            };
            nat_sub(a1210, a1211)
        };
        let a1213 = v2;
        let a1214 = 0u64;
        Ok(nat_rem(a1212, a1213, a1214))
    } else {
        let a1217 = {
            let a1215 = v0;
            let a1216 = f43(v1);
            nat_add(a1215, a1216)?
        };
        let a1218 = v2;
        let a1219 = 0u64;
        Ok(nat_rem(a1217, a1218, a1219))
    }
}

pub fn f52(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1222 = f42(v1);
    let a1223 = {
        let a1220 = v0;
        let a1221 = f43(v1);
        nat_add(a1220, a1221)?
    };
    if nat_le(a1222, a1223) {
        let a1224 = f53(v0, v1)?;
        let a1225 = v2;
        Ok(nat_lt(a1224, a1225))
    } else {
        Ok(false)
    }
}

pub fn f53(v0: u64, v1: u64) -> R<u64> {
    let a1228 = {
        let a1226 = v0;
        let a1227 = f43(v1);
        nat_add(a1226, a1227)?
    };
    let a1229 = f42(v1);
    Ok(nat_sub(a1228, a1229))
}

pub fn f54(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1230 = v0;
    let a1231 = v1;
    if nat_lt(a1230, a1231) {
        let a1232 = v1;
        let a1233 = v0;
        Ok(nat_sub(a1232, a1233))
    } else {
        let a1236 = v0;
        let a1237 = {
            let a1234 = v1;
            let a1235 = v2;
            nat_add(a1234, a1235)?
        };
        if nat_lt(a1236, a1237) {
            let a1238 = v0;
            let a1239 = v1;
            Ok(nat_sub(a1238, a1239))
        } else {
            let a1246 = {
                let a1242 = 2u64;
                let a1243 = {
                    let a1240 = v2;
                    let a1241 = 1u64;
                    nat_sub(a1240, a1241)
                };
                nat_mul(a1242, a1243)?
            };
            let a1247 = {
                let a1244 = v0;
                let a1245 = v1;
                nat_sub(a1244, a1245)
            };
            Ok(nat_sub(a1246, a1247))
        }
    }
}

pub fn f55(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1250 = {
        let a1248 = v2;
        let a1249 = f57(v1.clone(), f49(v0, v1.clone(), 0u64), f49(v0, v1.clone(), 1u64), f49(v0, v1.clone(), 2u64), f49(v0, v1.clone(), 3u64), false)?;
        nat_add(a1248, a1249)?
    };
    let a1251 = f57(v1.clone(), f49(v0, v1.clone(), 0u64), f49(v0, v1.clone(), 1u64), f49(v0, v1.clone(), 2u64), f49(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1250, a1251))
}

pub fn f56(v0: u64, v1: u64, v2: u64) -> bool {
    let a1252 = v0;
    let a1253 = 0u64;
    if nat_eq(a1252, a1253) {
        let a1254 = v2;
        let a1255 = v1;
        nat_le(a1254, a1255)
    } else {
        let a1256 = v0;
        let a1257 = 1u64;
        if nat_eq(a1256, a1257) {
            let a1258 = v2;
            let a1259 = v1;
            nat_lt(a1258, a1259)
        } else {
            let a1260 = v0;
            let a1261 = 2u64;
            if nat_eq(a1260, a1261) {
                let a1262 = v1;
                let a1263 = v2;
                nat_le(a1262, a1263)
            } else {
                let a1264 = v0;
                let a1265 = 3u64;
                if nat_eq(a1264, a1265) {
                    let a1266 = v1;
                    let a1267 = v2;
                    nat_lt(a1266, a1267)
                } else {
                    let a1268 = v1;
                    let a1269 = v2;
                    nat_eq(a1268, a1269)
                }
            }
        }
    }
}

pub fn f57(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1290 = {
        let a1284 = {
            let a1278 = {
                let a1272 = v1;
                let a1273 = if v5 {
                    f43(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1270, _, _, _) => {
                            h1270
                        }
                    })
                } else {
                    f42(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1271, _, _, _) => {
                            h1271
                        }
                    })
                };
                nat_mul(a1272, a1273)?
            };
            let a1279 = {
                let a1276 = v2;
                let a1277 = if v5 {
                    f43(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1274, _, _) => {
                            h1274
                        }
                    })
                } else {
                    f42(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1275, _, _) => {
                            h1275
                        }
                    })
                };
                nat_mul(a1276, a1277)?
            };
            nat_add(a1278, a1279)?
        };
        let a1285 = {
            let a1282 = v3;
            let a1283 = if v5 {
                f43(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1280, _) => {
                        h1280
                    }
                })
            } else {
                f42(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1281, _) => {
                        h1281
                    }
                })
            };
            nat_mul(a1282, a1283)?
        };
        nat_add(a1284, a1285)?
    };
    let a1291 = {
        let a1288 = v4;
        let a1289 = if v5 {
            f43(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1286) => {
                    h1286
                }
            })
        } else {
            f42(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1287) => {
                    h1287
                }
            })
        };
        nat_mul(a1288, a1289)?
    };
    nat_add(a1290, a1291)
}

pub fn f58(v0: u64, v1: u64) -> Option<u64> {
    let a1292 = v0;
    let a1293 = 2u64;
    if nat_eq(a1292, a1293) {
        Some(0u64)
    } else {
        let a1294 = v0;
        let a1295 = 5u64;
        if nat_eq(a1294, a1295) {
            Some(1u64)
        } else {
            let a1296 = v0;
            let a1297 = 6u64;
            if nat_eq(a1296, a1297) {
                Some(2u64)
            } else {
                let a1298 = v0;
                let a1299 = 7u64;
                if nat_eq(a1298, a1299) {
                    Some(3u64)
                } else {
                    let a1300 = v0;
                    let a1301 = 9u64;
                    if nat_eq(a1300, a1301) {
                        Some(4u64)
                    } else {
                        let a1302 = v0;
                        let a1303 = 8u64;
                        if nat_eq(a1302, a1303) {
                            Some(5u64)
                        } else {
                            let a1304 = v0;
                            let a1305 = 89u64;
                            if nat_eq(a1304, a1305) {
                                let a1306 = v1;
                                let a1307 = 2u64;
                                if nat_eq(a1306, a1307) {
                                    Some(6u64)
                                } else {
                                    let a1308 = v1;
                                    let a1309 = 0u64;
                                    if nat_eq(a1308, a1309) {
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

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1310 = v0;
    let a1311 = 28u64;
    if if if if nat_eq(a1310, a1311) {
        let a1312 = v1;
        let a1313 = 0u64;
        nat_eq(a1312, a1313)
    } else {
        false
    } {
        let a1314 = v2;
        let a1315 = 0u64;
        nat_eq(a1314, a1315)
    } else {
        false
    } {
        let a1316 = v3;
        let a1317 = 0u64;
        nat_eq(a1316, a1317)
    } else {
        false
    } {
        let a1318 = v4;
        let a1319 = v8;
        if if if if if if if if nat_eq(a1318, a1319) {
            let a1320 = v5;
            let a1321 = v12;
            nat_eq(a1320, a1321)
        } else {
            false
        } {
            let a1322 = v9;
            let a1323 = v13;
            nat_eq(a1322, a1323)
        } else {
            false
        } {
            let a1324 = v10;
            let a1325 = v14;
            nat_eq(a1324, a1325)
        } else {
            false
        } {
            let a1326 = v11;
            let a1327 = v15;
            nat_eq(a1326, a1327)
        } else {
            false
        } {
            let a1328 = 0u64;
            let a1329 = v6;
            nat_lt(a1328, a1329)
        } else {
            false
        } {
            let a1330 = 0u64;
            let a1331 = v7;
            nat_lt(a1330, a1331)
        } else {
            false
        } {
            let a1335 = {
                let a1332 = v10;
                let a1333 = v6;
                let a1334 = 1u64;
                nat_rem(a1332, a1333, a1334)
            };
            let a1336 = 0u64;
            if nat_eq(a1335, a1336) {
                let a1340 = {
                    let a1337 = v11;
                    let a1338 = v7;
                    let a1339 = 1u64;
                    nat_rem(a1337, a1338, a1339)
                };
                let a1341 = 0u64;
                nat_eq(a1340, a1341)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1342 = v10;
                let a1343 = v6;
                let a1344 = 0u64;
                nat_quot(a1342, a1343, a1344)
            }, {
                let a1345 = v11;
                let a1346 = v7;
                let a1347 = 0u64;
                nat_quot(a1345, a1346, a1347)
            }, {
                let a1348 = v5;
                let a1349 = v4;
                nat_mul(a1348, a1349)?
            }, {
                let a1350 = v4;
                let a1351 = v9;
                nat_mul(a1350, a1351)?
            }, {
                let a1352 = v5;
                let a1353 = v9;
                nat_mul(a1352, a1353)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}
