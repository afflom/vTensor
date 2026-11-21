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
    C15,
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
                                                        let a40 = v1;
                                                        let a41 = 15u64;
                                                        if nat_eq(a40, a41) {
                                                            f12(v3, v5)?
                                                        } else {
                                                            let m42 = f13(v1, v6);
                                                            match m42 {
                                                                None => {
                                                                    f14(v1, v6, v3, v4, v5)?
                                                                }
                                                                Some(v8) => {
                                                                    f15(v8, v3, v5)?
                                                                }
                                                            }
                                                        }
                                                    }
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
            f16(v9, v10, v11, v12)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a43 = v0;
    let a44 = 1u64;
    if if nat_eq(a43, a44) {
        true
    } else {
        let a45 = v0;
        let a46 = 33u64;
        nat_eq(a45, a46)
    } {
        true
    } else {
        let a47 = v0;
        let a48 = 34u64;
        nat_eq(a47, a48)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a51 = match v1.clone() {
        Adt2::C0(h49, _, _, _, _, _, _, _, _, _, _) => {
            h49
        }
    };
    let a52 = match v3.clone() {
        Adt2::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
            h50
        }
    };
    if nat_eq(a51, a52) {
        f17(v0, v1, v2, v3)
    } else {
        if if if if f18(match v1.clone() {
            Adt2::C0(h53, _, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        }) {
            f18(match v3.clone() {
                Adt2::C0(h54, _, _, _, _, _, _, _, _, _, _) => {
                    h54
                }
            })
        } else {
            false
        } {
            let a55 = v0;
            let a56 = 33u64;
            if nat_eq(a55, a56) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a57 = f19(v1.clone(), 1u64)?;
            let a58 = f19(v3.clone(), 1u64)?;
            nat_eq(a57, a58)
        } else {
            false
        } {
            let m60 = f20(match v3.clone() {
                Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                    h59
                }
            });
            match m60 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f19(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h61) => {
                            h61
                        }
                    }, f21(v1.clone(), 1u64), f21(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                            h62
                        }
                    }, match v3 {
                        Adt2::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                            h63
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
    let a65 = match v0.clone() {
        Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
            h64
        }
    };
    let a66 = 0u64;
    if if if if nat_eq(a65, a66) {
        true
    } else {
        let a68 = match v0.clone() {
            Adt2::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                h67
            }
        };
        let a69 = 1u64;
        nat_eq(a68, a69)
    } {
        true
    } else {
        let a71 = match v0.clone() {
            Adt2::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        };
        let a72 = 25u64;
        if nat_eq(a71, a72) {
            true
        } else {
            let a74 = match v0.clone() {
                Adt2::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                    h73
                }
            };
            let a75 = 26u64;
            nat_eq(a74, a75)
        }
    } {
        true
    } else {
        let a77 = match v0.clone() {
            Adt2::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                h76
            }
        };
        let a78 = 30u64;
        nat_eq(a77, a78)
    } {
        let a81 = match v0.clone() {
            Adt2::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                h79
            }
        };
        let a82 = match v1.clone() {
            Adt2::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                h80
            }
        };
        if if nat_eq(a81, a82) {
            f22(match v1.clone() {
                Adt2::C0(_, h83, _, _, _, _, _, _, _, _, _) => {
                    h83
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h84, _, _, _, _, _, _, _, _) => {
                    h84
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h85, _, _, _, _, _, _, _) => {
                    h85
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h86, _, _, _, _, _, _) => {
                    h86
                }
            }, match v0.clone() {
                Adt2::C0(_, h87, _, _, _, _, _, _, _, _, _) => {
                    h87
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h88, _, _, _, _, _, _, _, _) => {
                    h88
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                    h89
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h90, _, _, _, _, _, _) => {
                    h90
                }
            })
        } else {
            false
        } {
            let m92 = f20(match v0.clone() {
                Adt2::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
                    h91
                }
            });
            match m92 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f19(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h93) => {
                            h93
                        }
                    }, f21(v0, 1u64), f21(v1, 1u64), Adt4::C1))
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
    let m94 = v2;
    match m94 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a95 = v0;
            let a96 = 4u64;
            if if if nat_lt(a95, a96) {
                let a99 = match v1.clone() {
                    Adt2::C0(h97, _, _, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                };
                let a100 = match v4.clone() {
                    Adt2::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                        h98
                    }
                };
                if nat_eq(a99, a100) {
                    let a103 = match v1.clone() {
                        Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                            h101
                        }
                    };
                    let a104 = match v3.clone() {
                        Adt2::C0(h102, _, _, _, _, _, _, _, _, _, _) => {
                            h102
                        }
                    };
                    nat_eq(a103, a104)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f23(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h105, _, _, _, _, _, _, _, _, _) => {
                        h105
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h106, _, _, _, _, _, _, _, _, _) => {
                        h106
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h107, _, _, _, _, _, _, _, _, _) => {
                        h107
                    }
                })? {
                    f23(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h108, _, _, _, _, _, _, _, _) => {
                            h108
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h109, _, _, _, _, _, _, _, _) => {
                            h109
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h110, _, _, _, _, _, _, _, _) => {
                            h110
                        }
                    })?
                } else {
                    false
                } {
                    f23(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h111, _, _, _, _, _, _, _) => {
                            h111
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h112, _, _, _, _, _, _, _) => {
                            h112
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h113, _, _, _, _, _, _, _) => {
                            h113
                        }
                    })?
                } else {
                    false
                } {
                    f23(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h114, _, _, _, _, _, _) => {
                            h114
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h115, _, _, _, _, _, _) => {
                            h115
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h116, _, _, _, _, _, _) => {
                            h116
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m118 = f20(match v1.clone() {
                    Adt2::C0(h117, _, _, _, _, _, _, _, _, _, _) => {
                        h117
                    }
                });
                match m118 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h119, _) => {
                                h119
                            }
                        };
                        let a124 = {
                            let a121 = match v1.clone() {
                                Adt2::C0(_, h120, _, _, _, _, _, _, _, _, _) => {
                                    h120
                                }
                            };
                            let a122 = v6;
                            let a123 = 1u64;
                            nat_rem(a121, a122, a123)
                        };
                        let a125 = 0u64;
                        if if if nat_eq(a124, a125) {
                            let a130 = {
                                let a127 = match v4.clone() {
                                    Adt2::C0(_, h126, _, _, _, _, _, _, _, _, _) => {
                                        h126
                                    }
                                };
                                let a128 = v6;
                                let a129 = 1u64;
                                nat_rem(a127, a128, a129)
                            };
                            let a131 = 0u64;
                            nat_eq(a130, a131)
                        } else {
                            false
                        } {
                            let a136 = {
                                let a133 = match v3.clone() {
                                    Adt2::C0(_, h132, _, _, _, _, _, _, _, _, _) => {
                                        h132
                                    }
                                };
                                let a134 = v6;
                                let a135 = 1u64;
                                nat_rem(a133, a134, a135)
                            };
                            let a137 = 0u64;
                            nat_eq(a136, a137)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f19(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h138) => {
                                    h138
                                }
                            }, f21(v1, v6), f21(v3, v6), Adt4::C2(v0, f21(v4, v6))))
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
    let a140 = match v2.clone() {
        Adt2::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
            h139
        }
    };
    let a141 = 0u64;
    if if nat_eq(a140, a141) {
        let a143 = match v3.clone() {
            Adt2::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
                h142
            }
        };
        let a144 = 0u64;
        nat_eq(a143, a144)
    } else {
        false
    } {
        let a145 = v0;
        let a146 = 61u64;
        if nat_eq(a145, a146) {
            let v4: bool = {
                let a148 = 0u64;
                let a149 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h147, _, _, _, _, _, _, _) => {
                        h147
                    }
                };
                nat_lt(a148, a149)
            };
            if if if if if if f24(match v2.clone() {
                Adt2::C0(_, h150, _, _, _, _, _, _, _, _, _) => {
                    h150
                }
            }, match v1.clone() {
                Adt1::C0(h151, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h151
                }
            }, match v1.clone() {
                Adt1::C0(_, h152, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h152
                }
            }, match v3.clone() {
                Adt2::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                    h153
                }
            })? {
                f24(match v2.clone() {
                    Adt2::C0(_, _, h154, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h155, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h155
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h156, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h156
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h157, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                })?
            } else {
                false
            } {
                f24(match v2.clone() {
                    Adt2::C0(_, _, _, h158, _, _, _, _, _, _, _) => {
                        h158
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h159, _, _, _, _, _, _, _, _, _, _, _) => {
                        h159
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h160, _, _, _, _, _, _, _, _, _, _) => {
                        h160
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h161, _, _, _, _, _, _, _) => {
                        h161
                    }
                })?
            } else {
                false
            } {
                f24(match v2.clone() {
                    Adt2::C0(_, _, _, _, h162, _, _, _, _, _, _) => {
                        h162
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h163, _, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h164, _, _, _, _, _, _, _, _) => {
                        h164
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h165, _, _, _, _, _, _) => {
                        h165
                    }
                })?
            } else {
                false
            } {
                let a167 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h166, _, _, _, _, _, _, _) => {
                        h166
                    }
                };
                let a168 = 4294967295u64;
                nat_le(a167, a168)
            } else {
                false
            } {
                if v4 {
                    !f25(match v2.clone() {
                        Adt2::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
                            h169
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h170, _, _, _, _, _, _, _, _) => {
                            h170
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h171, _, _, _, _, _, _, _) => {
                            h171
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h172, _, _, _, _, _, _) => {
                            h172
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f19(v3.clone(), 1u64)?, 4u64, f21(v2, 1u64), f21(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h173, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h173
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h174, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h174
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h175, _, _, _, _, _, _, _, _, _, _, _) => {
                        h175
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h176, _, _, _, _, _, _, _, _, _) => {
                        h176
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a177 = v0;
            let a178 = 62u64;
            if nat_eq(a177, a178) {
                let a180 = match v1.clone() {
                    Adt1::C0(h179, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h179
                    }
                };
                let a181 = 2147483648u64;
                if if if if if nat_lt(a180, a181) {
                    let a183 = match v1.clone() {
                        Adt1::C0(_, h182, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h182
                        }
                    };
                    let a184 = 2147483648u64;
                    nat_lt(a183, a184)
                } else {
                    false
                } {
                    let a187 = match v1.clone() {
                        Adt1::C0(h185, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h185
                        }
                    };
                    let a188 = match v2.clone() {
                        Adt2::C0(_, h186, _, _, _, _, _, _, _, _, _) => {
                            h186
                        }
                    };
                    nat_lt(a187, a188)
                } else {
                    false
                } {
                    let a191 = match v1.clone() {
                        Adt1::C0(_, h189, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h189
                        }
                    };
                    let a192 = match v2.clone() {
                        Adt2::C0(_, h190, _, _, _, _, _, _, _, _, _) => {
                            h190
                        }
                    };
                    nat_lt(a191, a192)
                } else {
                    false
                } {
                    let a195 = match v2.clone() {
                        Adt2::C0(_, _, h193, _, _, _, _, _, _, _, _) => {
                            h193
                        }
                    };
                    let a196 = match v3.clone() {
                        Adt2::C0(_, _, h194, _, _, _, _, _, _, _, _) => {
                            h194
                        }
                    };
                    if if if nat_eq(a195, a196) {
                        let a199 = match v2.clone() {
                            Adt2::C0(_, _, _, h197, _, _, _, _, _, _, _) => {
                                h197
                            }
                        };
                        let a200 = match v3.clone() {
                            Adt2::C0(_, _, _, h198, _, _, _, _, _, _, _) => {
                                h198
                            }
                        };
                        nat_eq(a199, a200)
                    } else {
                        false
                    } {
                        let a203 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h201, _, _, _, _, _, _) => {
                                h201
                            }
                        };
                        let a204 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h202, _, _, _, _, _, _) => {
                                h202
                            }
                        };
                        nat_eq(a203, a204)
                    } else {
                        false
                    } {
                        let a213 = match v3.clone() {
                            Adt2::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
                                h205
                            }
                        };
                        let a214 = {
                            let a211 = {
                                let a208 = match v2.clone() {
                                    Adt2::C0(_, h206, _, _, _, _, _, _, _, _, _) => {
                                        h206
                                    }
                                };
                                let a209 = match v1.clone() {
                                    Adt1::C0(h207, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h207
                                    }
                                };
                                nat_add(a208, a209)?
                            };
                            let a212 = match v1.clone() {
                                Adt1::C0(_, h210, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h210
                                }
                            };
                            nat_add(a211, a212)?
                        };
                        nat_eq(a213, a214)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f19(v3.clone(), 1u64)?, 4u64, f21(v2, 1u64), f21(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h215, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h215
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a216 = v0;
                let a217 = 63u64;
                if nat_eq(a216, a217) {
                    if if if if if f26(match v2.clone() {
                        Adt2::C0(_, h218, _, _, _, _, _, _, _, _, _) => {
                            h218
                        }
                    }, match v1.clone() {
                        Adt1::C0(h219, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h219
                        }
                    })? {
                        f26(match v2.clone() {
                            Adt2::C0(_, _, h220, _, _, _, _, _, _, _, _) => {
                                h220
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h221, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h221
                            }
                        })?
                    } else {
                        false
                    } {
                        f26(match v2.clone() {
                            Adt2::C0(_, _, _, h222, _, _, _, _, _, _, _) => {
                                h222
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h223, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h223
                            }
                        })?
                    } else {
                        false
                    } {
                        f26(match v2.clone() {
                            Adt2::C0(_, _, _, _, h224, _, _, _, _, _, _) => {
                                h224
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h225, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h225
                            }
                        })?
                    } else {
                        false
                    } {
                        let a228 = match v2.clone() {
                            Adt2::C0(_, h226, _, _, _, _, _, _, _, _, _) => {
                                h226
                            }
                        };
                        let a229 = match v3.clone() {
                            Adt2::C0(_, h227, _, _, _, _, _, _, _, _, _) => {
                                h227
                            }
                        };
                        if if if nat_eq(a228, a229) {
                            let a232 = match v2.clone() {
                                Adt2::C0(_, _, h230, _, _, _, _, _, _, _, _) => {
                                    h230
                                }
                            };
                            let a233 = match v3.clone() {
                                Adt2::C0(_, _, h231, _, _, _, _, _, _, _, _) => {
                                    h231
                                }
                            };
                            nat_eq(a232, a233)
                        } else {
                            false
                        } {
                            let a236 = match v2.clone() {
                                Adt2::C0(_, _, _, h234, _, _, _, _, _, _, _) => {
                                    h234
                                }
                            };
                            let a237 = match v3.clone() {
                                Adt2::C0(_, _, _, h235, _, _, _, _, _, _, _) => {
                                    h235
                                }
                            };
                            nat_eq(a236, a237)
                        } else {
                            false
                        } {
                            let a240 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h238, _, _, _, _, _, _) => {
                                    h238
                                }
                            };
                            let a241 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h239, _, _, _, _, _, _) => {
                                    h239
                                }
                            };
                            nat_eq(a240, a241)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f19(v3.clone(), 1u64)?, 4u64, f21(v2, 1u64), f21(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h242, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h242
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h243, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h243
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h244, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h244
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h245, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h245
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
    let m246 = v3;
    match m246 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a247 = v0;
                let a248 = 4u64;
                nat_eq(a247, a248)
            };
            let v7: bool = {
                let a250 = 0u64;
                let a251 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h249, _, _, _, _, _, _, _, _, _, _, _) => {
                        h249
                    }
                };
                nat_lt(a250, a251)
            };
            let v8: u64 = f19(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h252, _, _, _, _, _, _, _, _, _) => {
                    h252
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h253, _, _, _, _, _, _, _, _) => {
                    h253
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h254, _, _, _, _, _, _, _) => {
                    h254
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h255, _, _, _, _, _, _) => {
                    h255
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h256, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h256
                }
            }, match v1.clone() {
                Adt1::C0(_, h257, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h257
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h258, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h258
                }
            });
            if if if if if if if if if if if if v6 {
                let a260 = match v2.clone() {
                    Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                        h259
                    }
                };
                let a261 = 0u64;
                if nat_eq(a260, a261) {
                    let a263 = match v5.clone() {
                        Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                            h262
                        }
                    };
                    let a264 = 0u64;
                    nat_eq(a263, a264)
                } else {
                    false
                }
            } else {
                let a266 = match v2.clone() {
                    Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a267 = 0u64;
                if if nat_eq(a266, a267) {
                    true
                } else {
                    let a269 = match v2.clone() {
                        Adt2::C0(h268, _, _, _, _, _, _, _, _, _, _) => {
                            h268
                        }
                    };
                    let a270 = 26u64;
                    nat_eq(a269, a270)
                } {
                    let a272 = match v5.clone() {
                        Adt2::C0(h271, _, _, _, _, _, _, _, _, _, _) => {
                            h271
                        }
                    };
                    let a273 = 0u64;
                    if nat_eq(a272, a273) {
                        true
                    } else {
                        let a275 = match v5.clone() {
                            Adt2::C0(h274, _, _, _, _, _, _, _, _, _, _) => {
                                h274
                            }
                        };
                        let a276 = 26u64;
                        nat_eq(a275, a276)
                    }
                } else {
                    false
                }
            } {
                let a279 = match v2.clone() {
                    Adt2::C0(h277, _, _, _, _, _, _, _, _, _, _) => {
                        h277
                    }
                };
                let a280 = match v4.clone() {
                    Adt2::C0(h278, _, _, _, _, _, _, _, _, _, _) => {
                        h278
                    }
                };
                nat_eq(a279, a280)
            } else {
                false
            } {
                f27(v2.clone(), v4.clone())
            } else {
                false
            } {
                f28(v2.clone())?
            } else {
                false
            } {
                f28(v4.clone())?
            } else {
                false
            } {
                let a282 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h281, _, _, _, _, _) => {
                        h281
                    }
                };
                let a283 = 4u64;
                nat_eq(a282, a283)
            } else {
                false
            } {
                let a284 = v8;
                let a285 = f19(v2.clone(), 1u64)?;
                nat_le(a284, a285)
            } else {
                false
            } {
                let a287 = match v1.clone() {
                    Adt1::C0(_, _, _, h286, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h286
                    }
                };
                let a288 = 1073741824u64;
                nat_lt(a287, a288)
            } else {
                false
            } {
                let a290 = match v1.clone() {
                    Adt1::C0(h289, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h289
                    }
                };
                let a291 = 4294967295u64;
                if if if if nat_le(a290, a291) {
                    let a293 = match v1.clone() {
                        Adt1::C0(_, h292, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    let a294 = 4294967295u64;
                    nat_le(a293, a294)
                } else {
                    false
                } {
                    let a296 = match v1.clone() {
                        Adt1::C0(_, _, h295, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h295
                        }
                    };
                    let a297 = 4294967295u64;
                    nat_le(a296, a297)
                } else {
                    false
                } {
                    let a299 = match v1.clone() {
                        Adt1::C0(_, _, _, h298, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h298
                        }
                    };
                    let a300 = 4294967295u64;
                    nat_le(a299, a300)
                } else {
                    false
                } {
                    let a302 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h301, _, _, _, _, _, _, _, _, _, _, _) => {
                            h301
                        }
                    };
                    let a303 = 4294967295u64;
                    nat_le(a302, a303)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a306 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h304) => {
                            h304
                        }
                    };
                    let a307 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h305) => {
                            h305
                        }
                    };
                    nat_eq(a306, a307)
                } else {
                    true
                }
            } else {
                false
            } {
                f29(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h308, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h308
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h309, _) => {
                        h309
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f19(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a310 = v10;
                    let a311 = v8;
                    nat_add(a310, a311)?
                }, 4u64, f21(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h312, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h312
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a314 = match v0.clone() {
        Adt2::C0(h313, _, _, _, _, _, _, _, _, _, _) => {
            h313
        }
    };
    let a315 = 0u64;
    if if if nat_eq(a314, a315) {
        let a317 = match v1.clone() {
            Adt2::C0(h316, _, _, _, _, _, _, _, _, _, _) => {
                h316
            }
        };
        let a318 = 0u64;
        nat_eq(a317, a318)
    } else {
        false
    } {
        f22(match v0.clone() {
            Adt2::C0(_, h319, _, _, _, _, _, _, _, _, _) => {
                h319
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h320, _, _, _, _, _, _, _, _) => {
                h320
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h321, _, _, _, _, _, _, _) => {
                h321
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h322, _, _, _, _, _, _) => {
                h322
            }
        }, match v1.clone() {
            Adt2::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h324, _, _, _, _, _, _, _, _) => {
                h324
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h325, _, _, _, _, _, _, _) => {
                h325
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h326, _, _, _, _, _, _) => {
                h326
            }
        })
    } else {
        false
    } {
        let v2: u64 = f19(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a327 = v2;
            let a328 = f19(v0.clone(), 1u64)?;
            nat_add(a327, a328)?
        }, 4u64, f21(v0, 1u64), f21(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a330 = match v1.clone() {
        Adt2::C0(h329, _, _, _, _, _, _, _, _, _, _) => {
            h329
        }
    };
    let a331 = 0u64;
    if if if nat_eq(a330, a331) {
        true
    } else {
        let a333 = match v1.clone() {
            Adt2::C0(h332, _, _, _, _, _, _, _, _, _, _) => {
                h332
            }
        };
        let a334 = 1u64;
        nat_eq(a333, a334)
    } {
        true
    } else {
        let a336 = match v1.clone() {
            Adt2::C0(h335, _, _, _, _, _, _, _, _, _, _) => {
                h335
            }
        };
        let a337 = 30u64;
        nat_eq(a336, a337)
    } {
        let a340 = match v1.clone() {
            Adt2::C0(h338, _, _, _, _, _, _, _, _, _, _) => {
                h338
            }
        };
        let a341 = match v2.clone() {
            Adt2::C0(h339, _, _, _, _, _, _, _, _, _, _) => {
                h339
            }
        };
        if if if if if nat_eq(a340, a341) {
            let a344 = match v1.clone() {
                Adt2::C0(_, h342, _, _, _, _, _, _, _, _, _) => {
                    h342
                }
            };
            let a345 = match v2.clone() {
                Adt2::C0(_, h343, _, _, _, _, _, _, _, _, _) => {
                    h343
                }
            };
            nat_eq(a344, a345)
        } else {
            false
        } {
            let a348 = match v1.clone() {
                Adt2::C0(_, _, _, h346, _, _, _, _, _, _, _) => {
                    h346
                }
            };
            let a349 = match v2.clone() {
                Adt2::C0(_, _, _, h347, _, _, _, _, _, _, _) => {
                    h347
                }
            };
            nat_eq(a348, a349)
        } else {
            false
        } {
            let a352 = match v1.clone() {
                Adt2::C0(_, _, _, _, h350, _, _, _, _, _, _) => {
                    h350
                }
            };
            let a353 = match v2.clone() {
                Adt2::C0(_, _, _, _, h351, _, _, _, _, _, _) => {
                    h351
                }
            };
            nat_eq(a352, a353)
        } else {
            false
        } {
            let a354 = v0;
            let a355 = 4u64;
            if nat_eq(a354, a355) {
                let a357 = match v1.clone() {
                    Adt2::C0(_, _, h356, _, _, _, _, _, _, _, _) => {
                        h356
                    }
                };
                let a358 = 1u64;
                if nat_eq(a357, a358) {
                    let a361 = match v1.clone() {
                        Adt2::C0(_, h359, _, _, _, _, _, _, _, _, _) => {
                            h359
                        }
                    };
                    let a362 = match v2.clone() {
                        Adt2::C0(_, _, h360, _, _, _, _, _, _, _, _) => {
                            h360
                        }
                    };
                    nat_eq(a361, a362)
                } else {
                    false
                }
            } else {
                let a363 = v0;
                let a364 = 4u64;
                if if nat_lt(a363, a364) {
                    let a367 = match v1.clone() {
                        Adt2::C0(_, h365, _, _, _, _, _, _, _, _, _) => {
                            h365
                        }
                    };
                    let a368 = match v1.clone() {
                        Adt2::C0(_, _, h366, _, _, _, _, _, _, _, _) => {
                            h366
                        }
                    };
                    nat_eq(a367, a368)
                } else {
                    false
                } {
                    let a371 = match v1.clone() {
                        Adt2::C0(_, _, h369, _, _, _, _, _, _, _, _) => {
                            h369
                        }
                    };
                    let a372 = match v2.clone() {
                        Adt2::C0(_, _, h370, _, _, _, _, _, _, _, _) => {
                            h370
                        }
                    };
                    nat_eq(a371, a372)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m374 = f20(match v1.clone() {
                Adt2::C0(h373, _, _, _, _, _, _, _, _, _, _) => {
                    h373
                }
            });
            match m374 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f19(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h375) => {
                            h375
                        }
                    }, f21(v1, 1u64), f21(v2, 1u64), Adt4::C8(v0)))
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
    if if if f30(match v1.clone() {
        Adt2::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
            h376
        }
    }, v0) {
        let a379 = match v1.clone() {
            Adt2::C0(h377, _, _, _, _, _, _, _, _, _, _) => {
                h377
            }
        };
        let a380 = match v2.clone() {
            Adt2::C0(h378, _, _, _, _, _, _, _, _, _, _) => {
                h378
            }
        };
        nat_eq(a379, a380)
    } else {
        false
    } {
        let a383 = match v1.clone() {
            Adt2::C0(_, h381, _, _, _, _, _, _, _, _, _) => {
                h381
            }
        };
        let a384 = match v2.clone() {
            Adt2::C0(_, h382, _, _, _, _, _, _, _, _, _) => {
                h382
            }
        };
        if if if nat_eq(a383, a384) {
            let a387 = match v1.clone() {
                Adt2::C0(_, _, h385, _, _, _, _, _, _, _, _) => {
                    h385
                }
            };
            let a388 = match v2.clone() {
                Adt2::C0(_, _, h386, _, _, _, _, _, _, _, _) => {
                    h386
                }
            };
            nat_eq(a387, a388)
        } else {
            false
        } {
            let a391 = match v1.clone() {
                Adt2::C0(_, _, _, h389, _, _, _, _, _, _, _) => {
                    h389
                }
            };
            let a392 = match v2.clone() {
                Adt2::C0(_, _, _, h390, _, _, _, _, _, _, _) => {
                    h390
                }
            };
            nat_eq(a391, a392)
        } else {
            false
        } {
            let a395 = match v1.clone() {
                Adt2::C0(_, _, _, _, h393, _, _, _, _, _, _) => {
                    h393
                }
            };
            let a396 = match v2.clone() {
                Adt2::C0(_, _, _, _, h394, _, _, _, _, _, _) => {
                    h394
                }
            };
            nat_eq(a395, a396)
        } else {
            false
        }
    } else {
        false
    } {
        let m398 = f20(match v1.clone() {
            Adt2::C0(h397, _, _, _, _, _, _, _, _, _, _) => {
                h397
            }
        });
        match m398 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f19(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h399) => {
                        h399
                    }
                }, f21(v1.clone(), 1u64), f21(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h400, _, _, _, _, _, _, _, _, _, _) => {
                        h400
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
            Adt1::C0(h401, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h401
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h402, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h402
            }
        }
    };
    let a403 = 0u64;
    let a404 = v4;
    if if nat_lt(a403, a404) {
        let a405 = v4;
        let a406 = 2147483648u64;
        nat_lt(a405, a406)
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
        let v7: u64 = f31(match v5.clone() {
            Adt2::C0(_, _, h407, _, _, _, _, _, _, _, _) => {
                h407
            }
        }, v4)?;
        let v8: u64 = f31(match v5.clone() {
            Adt2::C0(_, _, _, h408, _, _, _, _, _, _, _) => {
                h408
            }
        }, v4)?;
        let a410 = match v5.clone() {
            Adt2::C0(h409, _, _, _, _, _, _, _, _, _, _) => {
                h409
            }
        };
        let a411 = 0u64;
        if if if if if nat_eq(a410, a411) {
            let a413 = match v6.clone() {
                Adt2::C0(h412, _, _, _, _, _, _, _, _, _, _) => {
                    h412
                }
            };
            let a414 = 0u64;
            nat_eq(a413, a414)
        } else {
            false
        } {
            let a416 = match v5.clone() {
                Adt2::C0(_, _, _, _, h415, _, _, _, _, _, _) => {
                    h415
                }
            };
            let a417 = 1u64;
            nat_eq(a416, a417)
        } else {
            false
        } {
            let a420 = match v5.clone() {
                Adt2::C0(_, h418, _, _, _, _, _, _, _, _, _) => {
                    h418
                }
            };
            let a421 = match v6.clone() {
                Adt2::C0(_, h419, _, _, _, _, _, _, _, _, _) => {
                    h419
                }
            };
            nat_eq(a420, a421)
        } else {
            false
        } {
            if v0 {
                let a423 = match v5.clone() {
                    Adt2::C0(_, _, h422, _, _, _, _, _, _, _, _) => {
                        h422
                    }
                };
                let a424 = 2147483648u64;
                if if if nat_lt(a423, a424) {
                    let a426 = match v5.clone() {
                        Adt2::C0(_, _, _, h425, _, _, _, _, _, _, _) => {
                            h425
                        }
                    };
                    let a427 = 2147483648u64;
                    nat_lt(a426, a427)
                } else {
                    false
                } {
                    let a430 = {
                        let a428 = v7;
                        let a429 = v8;
                        nat_mul(a428, a429)?
                    };
                    let a431 = 2147483648u64;
                    nat_le(a430, a431)
                } else {
                    false
                } {
                    f32(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a433 = match v1.clone() {
                    Adt1::C0(h432, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h432
                    }
                };
                let a434 = v7;
                if if if if nat_eq(a433, a434) {
                    let a436 = match v1 {
                        Adt1::C0(_, h435, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h435
                        }
                    };
                    let a437 = v8;
                    nat_eq(a436, a437)
                } else {
                    false
                } {
                    f33(v7, v8, v4)?
                } else {
                    false
                } {
                    let a439 = match v6.clone() {
                        Adt2::C0(_, _, h438, _, _, _, _, _, _, _, _) => {
                            h438
                        }
                    };
                    let a440 = v4;
                    if nat_eq(a439, a440) {
                        let a442 = match v6.clone() {
                            Adt2::C0(_, _, _, h441, _, _, _, _, _, _, _) => {
                                h441
                            }
                        };
                        let a443 = v4;
                        nat_eq(a442, a443)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a447 = match v6 {
                        Adt2::C0(_, _, _, _, h444, _, _, _, _, _, _) => {
                            h444
                        }
                    };
                    let a448 = {
                        let a445 = v7;
                        let a446 = v8;
                        nat_mul(a445, a446)?
                    };
                    nat_eq(a447, a448)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f19(v3.clone(), 1u64)?, 4u64, f21(v2, 1u64), f21(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a450 = match v2.clone() {
        Adt2::C0(h449, _, _, _, _, _, _, _, _, _, _) => {
            h449
        }
    };
    let a451 = 0u64;
    if if if if if nat_eq(a450, a451) {
        true
    } else {
        let a453 = match v2.clone() {
            Adt2::C0(h452, _, _, _, _, _, _, _, _, _, _) => {
                h452
            }
        };
        let a454 = 1u64;
        nat_eq(a453, a454)
    } {
        true
    } else {
        let a456 = match v2.clone() {
            Adt2::C0(h455, _, _, _, _, _, _, _, _, _, _) => {
                h455
            }
        };
        let a457 = 30u64;
        nat_eq(a456, a457)
    } {
        let a460 = match v2.clone() {
            Adt2::C0(h458, _, _, _, _, _, _, _, _, _, _) => {
                h458
            }
        };
        let a461 = match v3.clone() {
            Adt2::C0(h459, _, _, _, _, _, _, _, _, _, _) => {
                h459
            }
        };
        nat_eq(a460, a461)
    } else {
        false
    } {
        if if f27(v2.clone(), v3.clone()) {
            let a463 = match v1.clone() {
                Adt1::C0(h462, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h462
                }
            };
            let a464 = 4294967296u64;
            nat_lt(a463, a464)
        } else {
            false
        } {
            let a466 = match v1.clone() {
                Adt1::C0(_, h465, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h465
                }
            };
            let a467 = 4294967296u64;
            nat_lt(a466, a467)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f19(v3.clone(), 1u64)?, {
            let a469 = match v2.clone() {
                Adt2::C0(h468, _, _, _, _, _, _, _, _, _, _) => {
                    h468
                }
            };
            let a470 = 0u64;
            if nat_eq(a469, a470) {
                4u64
            } else {
                2u64
            }
        }, f21(v2.clone(), 1u64), f21(v3, 1u64), Adt4::C13({
            let a471 = v0;
            let a472 = 31u64;
            if nat_eq(a471, a472) {
                0u64
            } else {
                let a473 = v0;
                let a474 = 47u64;
                if nat_eq(a473, a474) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h475, _, _, _, _, _, _, _, _, _, _) => {
                h475
            }
        }, match v1.clone() {
            Adt1::C0(h476, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h476
            }
        }, match v1 {
            Adt1::C0(_, h477, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h477
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f12(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a479 = match v0.clone() {
        Adt2::C0(h478, _, _, _, _, _, _, _, _, _, _) => {
            h478
        }
    };
    let a480 = 0u64;
    if if if nat_eq(a479, a480) {
        let a482 = match v1.clone() {
            Adt2::C0(h481, _, _, _, _, _, _, _, _, _, _) => {
                h481
            }
        };
        let a483 = 0u64;
        nat_eq(a482, a483)
    } else {
        false
    } {
        f27(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f19(v1.clone(), 1u64)?, 4u64, f21(v0, 1u64), f21(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f13(v0: u64, v1: u64) -> Option<u64> {
    let a484 = v0;
    let a485 = 8u64;
    if nat_eq(a484, a485) {
        Some(5u64)
    } else {
        let a486 = v0;
        let a487 = 9u64;
        if nat_eq(a486, a487) {
            Some(6u64)
        } else {
            let a488 = v0;
            let a489 = 89u64;
            if nat_eq(a488, a489) {
                let a490 = v1;
                let a491 = 0u64;
                if nat_eq(a490, a491) {
                    Some(0u64)
                } else {
                    let a492 = v1;
                    let a493 = 1u64;
                    if nat_eq(a492, a493) {
                        Some(1u64)
                    } else {
                        let a494 = v1;
                        let a495 = 2u64;
                        if nat_eq(a494, a495) {
                            Some(2u64)
                        } else {
                            let a496 = v1;
                            let a497 = 3u64;
                            if nat_eq(a496, a497) {
                                Some(3u64)
                            } else {
                                let a498 = v1;
                                let a499 = 6u64;
                                if nat_eq(a498, a499) {
                                    Some(4u64)
                                } else {
                                    let a500 = v1;
                                    let a501 = 12u64;
                                    if nat_eq(a500, a501) {
                                        Some(7u64)
                                    } else {
                                        let a502 = v1;
                                        let a503 = 11u64;
                                        if nat_eq(a502, a503) {
                                            Some(8u64)
                                        } else {
                                            let a504 = v1;
                                            let a505 = 18u64;
                                            if nat_eq(a504, a505) {
                                                Some(9u64)
                                            } else {
                                                let a506 = v1;
                                                let a507 = 19u64;
                                                if nat_eq(a506, a507) {
                                                    Some(10u64)
                                                } else {
                                                    let a508 = v1;
                                                    let a509 = 20u64;
                                                    if nat_eq(a508, a509) {
                                                        Some(11u64)
                                                    } else {
                                                        let a510 = v1;
                                                        let a511 = 21u64;
                                                        if nat_eq(a510, a511) {
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

pub fn f14(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m512 = f34(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m512 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f21(v2.clone(), 1u64), f21(v4, 1u64), Adt4::C10(v5, v6, {
                let m513 = v3;
                match m513 {
                    None => {
                        f21(v2, 1u64)
                    }
                    Some(v12) => {
                        f21(v12, 1u64)
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

pub fn f15(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a515 = match v1.clone() {
        Adt2::C0(h514, _, _, _, _, _, _, _, _, _, _) => {
            h514
        }
    };
    let a516 = 0u64;
    if if if if if nat_eq(a515, a516) {
        true
    } else {
        let a518 = match v1.clone() {
            Adt2::C0(h517, _, _, _, _, _, _, _, _, _, _) => {
                h517
            }
        };
        let a519 = 1u64;
        nat_eq(a518, a519)
    } {
        true
    } else {
        let a521 = match v1.clone() {
            Adt2::C0(h520, _, _, _, _, _, _, _, _, _, _) => {
                h520
            }
        };
        let a522 = 30u64;
        nat_eq(a521, a522)
    } {
        let a525 = match v1.clone() {
            Adt2::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                h523
            }
        };
        let a526 = match v2.clone() {
            Adt2::C0(h524, _, _, _, _, _, _, _, _, _, _) => {
                h524
            }
        };
        if nat_eq(a525, a526) {
            true
        } else {
            let a528 = match v2.clone() {
                Adt2::C0(h527, _, _, _, _, _, _, _, _, _, _) => {
                    h527
                }
            };
            let a529 = 0u64;
            nat_eq(a528, a529)
        }
    } else {
        false
    } {
        f27(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f19(v2.clone(), 1u64)?, {
            let a531 = match v2.clone() {
                Adt2::C0(h530, _, _, _, _, _, _, _, _, _, _) => {
                    h530
                }
            };
            let a532 = 0u64;
            if nat_eq(a531, a532) {
                4u64
            } else {
                2u64
            }
        }, f21(v1.clone(), 1u64), f21(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
                h533
            }
        }, match v2 {
            Adt2::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                h534
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f16(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m535 = v3;
    match m535 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f35(v0, v1)?, f35(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f36(v0, v1, v2.clone())?, f35(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f37(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f38(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f39(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f38(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f40(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f41(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f42(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f35(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f43(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f35(v0, v1)?, f35(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f44(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f35(v0, v1)?, f35(v0, v2)?))
        }
        Adt4::C14(_, v36, v37) => {
            Ok(Adt5::C3(v36, v37, f35(v0, v1)?, f35(v0, v2)?))
        }
        Adt4::C15 => {
            f45(v0, v1, v2)
        }
    }
}

pub fn f17(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a538 = match v1.clone() {
        Adt2::C0(h536, _, _, _, _, _, _, _, _, _, _) => {
            h536
        }
    };
    let a539 = match v3.clone() {
        Adt2::C0(h537, _, _, _, _, _, _, _, _, _, _) => {
            h537
        }
    };
    if if nat_eq(a538, a539) {
        let a540 = v0;
        let a541 = 33u64;
        if nat_eq(a540, a541) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m543 = f20(match v1.clone() {
            Adt2::C0(h542, _, _, _, _, _, _, _, _, _, _) => {
                h542
            }
        });
        match m543 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h544, _) => {
                        h544
                    }
                };
                let v6: u64 = f19(v1.clone(), v5)?;
                let a549 = {
                    let a546 = match v1.clone() {
                        Adt2::C0(_, h545, _, _, _, _, _, _, _, _, _) => {
                            h545
                        }
                    };
                    let a547 = v5;
                    let a548 = 1u64;
                    nat_rem(a546, a547, a548)
                };
                let a550 = 0u64;
                if if if nat_eq(a549, a550) {
                    let a555 = {
                        let a552 = match v3.clone() {
                            Adt2::C0(_, h551, _, _, _, _, _, _, _, _, _) => {
                                h551
                            }
                        };
                        let a553 = v5;
                        let a554 = 1u64;
                        nat_rem(a552, a553, a554)
                    };
                    let a556 = 0u64;
                    nat_eq(a555, a556)
                } else {
                    false
                } {
                    let a557 = v6;
                    let a558 = f19(v3.clone(), v5)?;
                    nat_eq(a557, a558)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h559) => {
                            h559
                        }
                    }, f21(v1, v5), f21(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f18(v0: u64) -> bool {
    let a560 = v0;
    let a561 = 0u64;
    if if nat_eq(a560, a561) {
        true
    } else {
        let a562 = v0;
        let a563 = 1u64;
        nat_eq(a562, a563)
    } {
        true
    } else {
        let a564 = v0;
        let a565 = 30u64;
        nat_eq(a564, a565)
    }
}

pub fn f19(v0: Adt2, v1: u64) -> R<u64> {
    let a577 = {
        let a571 = {
            let a567 = match v0.clone() {
                Adt2::C0(_, h566, _, _, _, _, _, _, _, _, _) => {
                    h566
                }
            };
            let a568 = v1;
            let a569 = 0u64;
            nat_quot(a567, a568, a569)
        };
        let a572 = match v0.clone() {
            Adt2::C0(_, _, h570, _, _, _, _, _, _, _, _) => {
                h570
            }
        };
        nat_mul(a571, a572)?
    };
    let a578 = {
        let a575 = match v0.clone() {
            Adt2::C0(_, _, _, h573, _, _, _, _, _, _, _) => {
                h573
            }
        };
        let a576 = match v0 {
            Adt2::C0(_, _, _, _, h574, _, _, _, _, _, _) => {
                h574
            }
        };
        nat_mul(a575, a576)?
    };
    nat_mul(a577, a578)
}

pub fn f20(v0: u64) -> Adt7 {
    let a579 = v0;
    let a580 = 0u64;
    if nat_eq(a579, a580) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a581 = v0;
        let a582 = 1u64;
        if nat_eq(a581, a582) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a583 = v0;
            let a584 = 2u64;
            if nat_eq(a583, a584) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a585 = v0;
                let a586 = 3u64;
                if nat_eq(a585, a586) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a587 = v0;
                    let a588 = 6u64;
                    if nat_eq(a587, a588) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a589 = v0;
                        let a590 = 7u64;
                        if nat_eq(a589, a590) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a591 = v0;
                            let a592 = 8u64;
                            if nat_eq(a591, a592) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a593 = v0;
                                let a594 = 9u64;
                                if nat_eq(a593, a594) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a595 = v0;
                                    let a596 = 10u64;
                                    if nat_eq(a595, a596) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a597 = v0;
                                        let a598 = 11u64;
                                        if nat_eq(a597, a598) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a599 = v0;
                                            let a600 = 12u64;
                                            if nat_eq(a599, a600) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a601 = v0;
                                                let a602 = 13u64;
                                                if nat_eq(a601, a602) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a603 = v0;
                                                    let a604 = 14u64;
                                                    if nat_eq(a603, a604) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a605 = v0;
                                                        let a606 = 15u64;
                                                        if nat_eq(a605, a606) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a607 = v0;
                                                            let a608 = 16u64;
                                                            if nat_eq(a607, a608) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a609 = v0;
                                                                let a610 = 17u64;
                                                                if nat_eq(a609, a610) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a611 = v0;
                                                                    let a612 = 18u64;
                                                                    if nat_eq(a611, a612) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a613 = v0;
                                                                        let a614 = 19u64;
                                                                        if nat_eq(a613, a614) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a615 = v0;
                                                                            let a616 = 20u64;
                                                                            if nat_eq(a615, a616) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a617 = v0;
                                                                                let a618 = 21u64;
                                                                                if nat_eq(a617, a618) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a619 = v0;
                                                                                    let a620 = 22u64;
                                                                                    if nat_eq(a619, a620) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a621 = v0;
                                                                                        let a622 = 23u64;
                                                                                        if nat_eq(a621, a622) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a623 = v0;
                                                                                            let a624 = 24u64;
                                                                                            if nat_eq(a623, a624) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a625 = v0;
                                                                                                let a626 = 25u64;
                                                                                                if nat_eq(a625, a626) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a627 = v0;
                                                                                                    let a628 = 26u64;
                                                                                                    if nat_eq(a627, a628) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a629 = v0;
                                                                                                        let a630 = 27u64;
                                                                                                        if nat_eq(a629, a630) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a631 = v0;
                                                                                                            let a632 = 28u64;
                                                                                                            if nat_eq(a631, a632) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a633 = v0;
                                                                                                                let a634 = 29u64;
                                                                                                                if nat_eq(a633, a634) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a635 = v0;
                                                                                                                    let a636 = 30u64;
                                                                                                                    if nat_eq(a635, a636) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a637 = v0;
                                                                                                                        let a638 = 34u64;
                                                                                                                        if nat_eq(a637, a638) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a639 = v0;
                                                                                                                            let a640 = 35u64;
                                                                                                                            if nat_eq(a639, a640) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a641 = v0;
                                                                                                                                let a642 = 39u64;
                                                                                                                                if nat_eq(a641, a642) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a643 = v0;
                                                                                                                                    let a644 = 40u64;
                                                                                                                                    if nat_eq(a643, a644) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a645 = v0;
                                                                                                                                        let a646 = 41u64;
                                                                                                                                        if nat_eq(a645, a646) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a647 = v0;
                                                                                                                                            let a648 = 42u64;
                                                                                                                                            if nat_eq(a647, a648) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a649 = v0;
                                                                                                                                                let a650 = 43u64;
                                                                                                                                                if nat_eq(a649, a650) {
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

pub fn f21(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a652 = match v0.clone() {
            Adt2::C0(_, h651, _, _, _, _, _, _, _, _, _) => {
                h651
            }
        };
        let a653 = v1;
        let a654 = 0u64;
        nat_quot(a652, a653, a654)
    }, match v0.clone() {
        Adt2::C0(_, _, h655, _, _, _, _, _, _, _, _) => {
            h655
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h656, _, _, _, _, _, _, _) => {
            h656
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h657, _, _, _, _, _, _) => {
            h657
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h658, _, _, _, _, _) => {
            h658
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h659, _, _, _, _) => {
            h659
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h660, _, _, _) => {
            h660
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h661, _, _) => {
            h661
        }
    })
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f25(v4, v5, v6, v7) {
        f25(v0, v1, v2, v3)
    } else {
        let a665 = {
            let a662 = v0;
            let a663 = v4;
            let a664 = 1u64;
            nat_rem(a662, a663, a664)
        };
        let a666 = 0u64;
        if if if nat_eq(a665, a666) {
            let a670 = {
                let a667 = v1;
                let a668 = v5;
                let a669 = 1u64;
                nat_rem(a667, a668, a669)
            };
            let a671 = 0u64;
            nat_eq(a670, a671)
        } else {
            false
        } {
            let a675 = {
                let a672 = v2;
                let a673 = v6;
                let a674 = 1u64;
                nat_rem(a672, a673, a674)
            };
            let a676 = 0u64;
            nat_eq(a675, a676)
        } else {
            false
        } {
            let a680 = {
                let a677 = v3;
                let a678 = v7;
                let a679 = 1u64;
                nat_rem(a677, a678, a679)
            };
            let a681 = 0u64;
            nat_eq(a680, a681)
        } else {
            false
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a682 = v0;
    let a683 = v1;
    if nat_eq(a682, a683) {
        let a686 = v4;
        let a687 = {
            let a684 = v2;
            let a685 = v3;
            nat_add(a684, a685)?
        };
        Ok(nat_eq(a686, a687))
    } else {
        let a688 = v2;
        let a689 = v3;
        if nat_eq(a688, a689) {
            let a690 = v4;
            let a691 = v2;
            Ok(nat_eq(a690, a691))
        } else {
            Ok(false)
        }
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a694 = {
            let a692 = v0;
            let a693 = f46(v1);
            nat_add(a692, a693)?
        };
        let a695 = f46(v2);
        nat_add(a694, a695)?
    };
    let v5: u64 = {
        let a696 = f47(v1);
        let a697 = f47(v2);
        nat_add(a696, a697)?
    };
    let a698 = v1;
    let a699 = 4294967295u64;
    if if nat_le(a698, a699) {
        let a700 = v2;
        let a701 = 4294967295u64;
        nat_le(a700, a701)
    } else {
        false
    } {
        let a702 = v5;
        let a703 = v4;
        if nat_le(a702, a703) {
            let a706 = v3;
            let a707 = {
                let a704 = v4;
                let a705 = v5;
                nat_sub(a704, a705)
            };
            Ok(nat_eq(a706, a707))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a708 = v0;
    let a709 = 0u64;
    if if if nat_eq(a708, a709) {
        true
    } else {
        let a710 = v1;
        let a711 = 0u64;
        nat_eq(a710, a711)
    } {
        true
    } else {
        let a712 = v2;
        let a713 = 0u64;
        nat_eq(a712, a713)
    } {
        true
    } else {
        let a714 = v3;
        let a715 = 0u64;
        nat_eq(a714, a715)
    }
}

pub fn f26(v0: u64, v1: u64) -> R<bool> {
    let a716 = v1;
    let a717 = 4294967295u64;
    if nat_le(a716, a717) {
        let a720 = {
            let a718 = f46(v1);
            let a719 = f47(v1);
            nat_add(a718, a719)?
        };
        let a721 = v0;
        Ok(nat_lt(a720, a721))
    } else {
        Ok(false)
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> bool {
    let a724 = match v0.clone() {
        Adt2::C0(_, h722, _, _, _, _, _, _, _, _, _) => {
            h722
        }
    };
    let a725 = match v1.clone() {
        Adt2::C0(_, h723, _, _, _, _, _, _, _, _, _) => {
            h723
        }
    };
    if if if nat_eq(a724, a725) {
        let a728 = match v0.clone() {
            Adt2::C0(_, _, h726, _, _, _, _, _, _, _, _) => {
                h726
            }
        };
        let a729 = match v1.clone() {
            Adt2::C0(_, _, h727, _, _, _, _, _, _, _, _) => {
                h727
            }
        };
        nat_eq(a728, a729)
    } else {
        false
    } {
        let a732 = match v0.clone() {
            Adt2::C0(_, _, _, h730, _, _, _, _, _, _, _) => {
                h730
            }
        };
        let a733 = match v1.clone() {
            Adt2::C0(_, _, _, h731, _, _, _, _, _, _, _) => {
                h731
            }
        };
        nat_eq(a732, a733)
    } else {
        false
    } {
        let a736 = match v0 {
            Adt2::C0(_, _, _, _, h734, _, _, _, _, _, _) => {
                h734
            }
        };
        let a737 = match v1 {
            Adt2::C0(_, _, _, _, h735, _, _, _, _, _, _) => {
                h735
            }
        };
        nat_eq(a736, a737)
    } else {
        false
    }
}

pub fn f28(v0: Adt2) -> R<bool> {
    let a739 = match v0.clone() {
        Adt2::C0(_, h738, _, _, _, _, _, _, _, _, _) => {
            h738
        }
    };
    let a740 = 1u64;
    if if if if nat_eq(a739, a740) {
        true
    } else {
        let a742 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h741, _, _, _, _, _) => {
                h741
            }
        };
        let a743 = 4u64;
        nat_eq(a742, a743)
    } {
        let a745 = match v0.clone() {
            Adt2::C0(_, _, h744, _, _, _, _, _, _, _, _) => {
                h744
            }
        };
        let a746 = 1u64;
        if nat_eq(a745, a746) {
            true
        } else {
            let a751 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h747, _, _, _, _) => {
                    h747
                }
            };
            let a752 = {
                let a749 = match v0.clone() {
                    Adt2::C0(_, h748, _, _, _, _, _, _, _, _, _) => {
                        h748
                    }
                };
                let a750 = 4u64;
                nat_mul(a749, a750)?
            };
            nat_eq(a751, a752)
        }
    } else {
        false
    } {
        let a754 = match v0.clone() {
            Adt2::C0(_, _, _, h753, _, _, _, _, _, _, _) => {
                h753
            }
        };
        let a755 = 1u64;
        if nat_eq(a754, a755) {
            true
        } else {
            let a763 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h756, _, _, _) => {
                    h756
                }
            };
            let a764 = {
                let a761 = {
                    let a759 = match v0.clone() {
                        Adt2::C0(_, h757, _, _, _, _, _, _, _, _, _) => {
                            h757
                        }
                    };
                    let a760 = match v0.clone() {
                        Adt2::C0(_, _, h758, _, _, _, _, _, _, _, _) => {
                            h758
                        }
                    };
                    nat_mul(a759, a760)?
                };
                let a762 = 4u64;
                nat_mul(a761, a762)?
            };
            nat_eq(a763, a764)
        }
    } else {
        false
    } {
        let a766 = match v0.clone() {
            Adt2::C0(_, _, _, _, h765, _, _, _, _, _, _) => {
                h765
            }
        };
        let a767 = 1u64;
        if nat_eq(a766, a767) {
            Ok(true)
        } else {
            let a778 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h768, _, _) => {
                    h768
                }
            };
            let a779 = {
                let a776 = {
                    let a774 = {
                        let a771 = match v0.clone() {
                            Adt2::C0(_, h769, _, _, _, _, _, _, _, _, _) => {
                                h769
                            }
                        };
                        let a772 = match v0.clone() {
                            Adt2::C0(_, _, h770, _, _, _, _, _, _, _, _) => {
                                h770
                            }
                        };
                        nat_mul(a771, a772)?
                    };
                    let a775 = match v0 {
                        Adt2::C0(_, _, _, h773, _, _, _, _, _, _, _) => {
                            h773
                        }
                    };
                    nat_mul(a774, a775)?
                };
                let a777 = 4u64;
                nat_mul(a776, a777)?
            };
            Ok(nat_eq(a778, a779))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f48(v0.clone(), false)?;
    let v6: u64 = f48(v0, true)?;
    let a780 = v6;
    let a781 = v1;
    if if nat_le(a780, a781) {
        let a788 = {
            let a786 = {
                let a782 = v1;
                let a783 = v5;
                nat_add(a782, a783)?
            };
            let a787 = {
                let a784 = v3;
                let a785 = 0u64;
                if nat_eq(a784, a785) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a786, a787)?
        };
        let a789 = v2;
        nat_le(a788, a789)
    } else {
        false
    } {
        if v4 {
            let a794 = {
                let a792 = {
                    let a790 = v1;
                    let a791 = v5;
                    nat_add(a790, a791)?
                };
                let a793 = v6;
                nat_sub(a792, a793)
            };
            let a795 = v2;
            Ok(nat_lt(a794, a795))
        } else {
            let a800 = {
                let a798 = {
                    let a796 = v1;
                    let a797 = v5;
                    nat_add(a796, a797)?
                };
                let a799 = v6;
                nat_sub(a798, a799)
            };
            let a801 = v2;
            Ok(nat_le(a800, a801))
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: u64, v1: u64) -> bool {
    let a802 = v0;
    let a803 = 0u64;
    if if nat_eq(a802, a803) {
        true
    } else {
        let a804 = v0;
        let a805 = 1u64;
        nat_eq(a804, a805)
    } {
        let a806 = v1;
        let a807 = 4294967296u64;
        nat_lt(a806, a807)
    } else {
        false
    }
}

pub fn f31(v0: u64, v1: u64) -> R<u64> {
    let a812 = {
        let a810 = v0;
        let a811 = {
            let a808 = v1;
            let a809 = 1u64;
            nat_sub(a808, a809)
        };
        nat_add(a810, a811)?
    };
    let a813 = v1;
    let a814 = 0u64;
    Ok(nat_quot(a812, a813, a814))
}

pub fn f32(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f25(match v0.clone() {
        Adt2::C0(_, h815, _, _, _, _, _, _, _, _, _) => {
            h815
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h816, _, _, _, _, _, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h817, _, _, _, _, _, _, _) => {
            h817
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h818, _, _, _, _, _, _) => {
            h818
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a821 = match v1.clone() {
                Adt2::C0(_, h819, _, _, _, _, _, _, _, _, _) => {
                    h819
                }
            };
            let a822 = match v1.clone() {
                Adt2::C0(_, _, h820, _, _, _, _, _, _, _, _) => {
                    h820
                }
            };
            nat_mul(a821, a822)?
        };
        let v5: u64 = {
            let a824 = v4;
            let a825 = match v1.clone() {
                Adt2::C0(_, _, _, h823, _, _, _, _, _, _, _) => {
                    h823
                }
            };
            nat_mul(a824, a825)?
        };
        let a838 = {
            let a836 = {
                let a827 = match v0.clone() {
                    Adt2::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                        h826
                    }
                };
                let a828 = 1u64;
                nat_sub(a827, a828)
            };
            let a837 = {
                let a834 = f49(match v0.clone() {
                    Adt2::C0(_, _, h829, _, _, _, _, _, _, _, _) => {
                        h829
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h830, _, _, _, _, _, _, _, _, _) => {
                        h830
                    }
                })?;
                let a835 = f49(match v0 {
                    Adt2::C0(_, _, _, h831, _, _, _, _, _, _, _) => {
                        h831
                    }
                }, v2, {
                    let a832 = v3;
                    let a833 = v5;
                    nat_mul(a832, a833)?
                }, v4)?;
                nat_add(a834, a835)?
            };
            nat_add(a836, a837)?
        };
        let a839 = f19(v1, 1u64)?;
        Ok(nat_lt(a838, a839))
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a840 = v0;
    let a841 = 2147483648u64;
    if if if if nat_lt(a840, a841) {
        let a842 = v1;
        let a843 = 2147483648u64;
        nat_lt(a842, a843)
    } else {
        false
    } {
        let a846 = {
            let a844 = v0;
            let a845 = v1;
            nat_mul(a844, a845)?
        };
        let a847 = 2147483648u64;
        nat_lt(a846, a847)
    } else {
        false
    } {
        let a852 = {
            let a850 = {
                let a848 = v0;
                let a849 = 1u64;
                nat_sub(a848, a849)
            };
            let a851 = v2;
            nat_mul(a850, a851)?
        };
        let a853 = 2147483648u64;
        nat_lt(a852, a853)
    } else {
        false
    } {
        let a858 = {
            let a856 = {
                let a854 = v1;
                let a855 = 1u64;
                nat_sub(a854, a855)
            };
            let a857 = v2;
            nat_mul(a856, a857)?
        };
        let a859 = 2147483648u64;
        Ok(nat_lt(a858, a859))
    } else {
        Ok(false)
    }
}

pub fn f34(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m860 = v3;
    match m860 {
        None => {
            f50(v0, v1, match v2.clone() {
                Adt2::C0(h861, _, _, _, _, _, _, _, _, _, _) => {
                    h861
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h862, _, _, _, _, _, _, _, _, _, _) => {
                    h862
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h863, _, _, _, _, _, _, _, _, _) => {
                    h863
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h864, _, _, _, _, _, _, _, _) => {
                    h864
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h865, _, _, _, _, _, _, _) => {
                    h865
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h866, _, _, _, _, _, _) => {
                    h866
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h867, _, _, _, _, _, _, _, _, _) => {
                    h867
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h868, _, _, _, _, _, _, _, _) => {
                    h868
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h869, _, _, _, _, _, _, _) => {
                    h869
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h870, _, _, _, _, _, _) => {
                    h870
                }
            })
        }
        Some(v5) => {
            f50(v0, v1, match v2.clone() {
                Adt2::C0(h871, _, _, _, _, _, _, _, _, _, _) => {
                    h871
                }
            }, match v5.clone() {
                Adt2::C0(h872, _, _, _, _, _, _, _, _, _, _) => {
                    h872
                }
            }, match v4.clone() {
                Adt2::C0(h873, _, _, _, _, _, _, _, _, _, _) => {
                    h873
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h874, _, _, _, _, _, _, _, _, _) => {
                    h874
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h875, _, _, _, _, _, _, _, _) => {
                    h875
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h876, _, _, _, _, _, _, _) => {
                    h876
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h877, _, _, _, _, _, _) => {
                    h877
                }
            }, match v5.clone() {
                Adt2::C0(_, h878, _, _, _, _, _, _, _, _, _) => {
                    h878
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h879, _, _, _, _, _, _, _, _) => {
                    h879
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h880, _, _, _, _, _, _, _) => {
                    h880
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h881, _, _, _, _, _, _) => {
                    h881
                }
            }, match v4.clone() {
                Adt2::C0(_, h882, _, _, _, _, _, _, _, _, _) => {
                    h882
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h883, _, _, _, _, _, _, _, _) => {
                    h883
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h884, _, _, _, _, _, _, _) => {
                    h884
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h885, _, _, _, _, _, _) => {
                    h885
                }
            })
        }
    }
}

pub fn f35(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a887 = v0;
        let a888 = match v1.clone() {
            Adt3::C0(h886, _, _, _, _, _, _, _) => {
                h886
            }
        };
        let a889 = 0u64;
        nat_quot(a887, a888, a889)
    };
    let v3: u64 = {
        let a891 = v2;
        let a892 = match v1.clone() {
            Adt3::C0(_, h890, _, _, _, _, _, _) => {
                h890
            }
        };
        let a893 = 0u64;
        nat_quot(a891, a892, a893)
    };
    f51({
        let a895 = v0;
        let a896 = match v1.clone() {
            Adt3::C0(h894, _, _, _, _, _, _, _) => {
                h894
            }
        };
        let a897 = 0u64;
        nat_rem(a895, a896, a897)
    }, {
        let a899 = v2;
        let a900 = match v1.clone() {
            Adt3::C0(_, h898, _, _, _, _, _, _) => {
                h898
            }
        };
        let a901 = 0u64;
        nat_rem(a899, a900, a901)
    }, {
        let a903 = v3;
        let a904 = match v1.clone() {
            Adt3::C0(_, _, h902, _, _, _, _, _) => {
                h902
            }
        };
        let a905 = 0u64;
        nat_rem(a903, a904, a905)
    }, {
        let a907 = v3;
        let a908 = match v1.clone() {
            Adt3::C0(_, _, h906, _, _, _, _, _) => {
                h906
            }
        };
        let a909 = 0u64;
        nat_quot(a907, a908, a909)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h910, _, _, _) => {
            h910
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h911, _, _) => {
            h911
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h912, _) => {
            h912
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h913) => {
            h913
        }
    })
}

pub fn f36(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a915 = v0;
        let a916 = match v2.clone() {
            Adt3::C0(h914, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a917 = 0u64;
        nat_quot(a915, a916, a917)
    };
    let v4: u64 = {
        let a919 = v3;
        let a920 = match v2.clone() {
            Adt3::C0(_, h918, _, _, _, _, _, _) => {
                h918
            }
        };
        let a921 = 0u64;
        nat_quot(a919, a920, a921)
    };
    f52({
        let a923 = v0;
        let a924 = match v2.clone() {
            Adt3::C0(h922, _, _, _, _, _, _, _) => {
                h922
            }
        };
        let a925 = 0u64;
        nat_rem(a923, a924, a925)
    }, {
        let a927 = v3;
        let a928 = match v2.clone() {
            Adt3::C0(_, h926, _, _, _, _, _, _) => {
                h926
            }
        };
        let a929 = 0u64;
        nat_rem(a927, a928, a929)
    }, {
        let a931 = v4;
        let a932 = match v2.clone() {
            Adt3::C0(_, _, h930, _, _, _, _, _) => {
                h930
            }
        };
        let a933 = 0u64;
        nat_rem(a931, a932, a933)
    }, {
        let a935 = v4;
        let a936 = match v2 {
            Adt3::C0(_, _, h934, _, _, _, _, _) => {
                h934
            }
        };
        let a937 = 0u64;
        nat_quot(a935, a936, a937)
    }, match v1.clone() {
        Adt3::C0(h938, _, _, _, _, _, _, _) => {
            h938
        }
    }, match v1.clone() {
        Adt3::C0(_, h939, _, _, _, _, _, _) => {
            h939
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h940, _, _, _, _, _) => {
            h940
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h941, _, _, _, _) => {
            h941
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h942, _, _, _) => {
            h942
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h943, _, _) => {
            h943
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h944, _) => {
            h944
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h945) => {
            h945
        }
    })
}

pub fn f37(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a947 = v0;
        let a948 = match v3.clone() {
            Adt3::C0(h946, _, _, _, _, _, _, _) => {
                h946
            }
        };
        let a949 = 0u64;
        nat_quot(a947, a948, a949)
    };
    let v6: u64 = {
        let a951 = v5;
        let a952 = match v3.clone() {
            Adt3::C0(_, h950, _, _, _, _, _, _) => {
                h950
            }
        };
        let a953 = 0u64;
        nat_quot(a951, a952, a953)
    };
    let v7: u64 = {
        let a955 = v0;
        let a956 = match v3.clone() {
            Adt3::C0(h954, _, _, _, _, _, _, _) => {
                h954
            }
        };
        let a957 = 0u64;
        nat_rem(a955, a956, a957)
    };
    let v8: u64 = {
        let a959 = v5;
        let a960 = match v3.clone() {
            Adt3::C0(_, h958, _, _, _, _, _, _) => {
                h958
            }
        };
        let a961 = 0u64;
        nat_rem(a959, a960, a961)
    };
    let v9: u64 = {
        let a963 = v6;
        let a964 = match v3.clone() {
            Adt3::C0(_, _, h962, _, _, _, _, _) => {
                h962
            }
        };
        let a965 = 0u64;
        nat_rem(a963, a964, a965)
    };
    let v10: u64 = {
        let a967 = v6;
        let a968 = match v3.clone() {
            Adt3::C0(_, _, h966, _, _, _, _, _) => {
                h966
            }
        };
        let a969 = 0u64;
        nat_quot(a967, a968, a969)
    };
    let v11: u64 = {
        let a970 = v4;
        let a971 = 0u64;
        if nat_eq(a970, a971) {
            match v1.clone() {
                Adt3::C0(h972, _, _, _, _, _, _, _) => {
                    h972
                }
            }
        } else {
            let a973 = v4;
            let a974 = 1u64;
            if nat_eq(a973, a974) {
                match v1.clone() {
                    Adt3::C0(_, h975, _, _, _, _, _, _) => {
                        h975
                    }
                }
            } else {
                let a976 = v4;
                let a977 = 2u64;
                if nat_eq(a976, a977) {
                    match v1.clone() {
                        Adt3::C0(_, _, h978, _, _, _, _, _) => {
                            h978
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h979, _, _, _, _) => {
                            h979
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a986 = v11;
        let a987 = {
            let a980 = v4;
            let a981 = 0u64;
            if nat_eq(a980, a981) {
                v7
            } else {
                let a982 = v4;
                let a983 = 1u64;
                if nat_eq(a982, a983) {
                    v8
                } else {
                    let a984 = v4;
                    let a985 = 2u64;
                    if nat_eq(a984, a985) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a986, a987)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f51(if if v12 {
        let a988 = v4;
        let a989 = 0u64;
        nat_eq(a988, a989)
    } else {
        false
    } {
        let a990 = v7;
        let a991 = v11;
        nat_sub(a990, a991)
    } else {
        v7
    }, if if v12 {
        let a992 = v4;
        let a993 = 1u64;
        nat_eq(a992, a993)
    } else {
        false
    } {
        let a994 = v8;
        let a995 = v11;
        nat_sub(a994, a995)
    } else {
        v8
    }, if if v12 {
        let a996 = v4;
        let a997 = 2u64;
        nat_eq(a996, a997)
    } else {
        false
    } {
        let a998 = v9;
        let a999 = v11;
        nat_sub(a998, a999)
    } else {
        v9
    }, if if v12 {
        let a1000 = v4;
        let a1001 = 3u64;
        nat_eq(a1000, a1001)
    } else {
        false
    } {
        let a1002 = v10;
        let a1003 = v11;
        nat_sub(a1002, a1003)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1004, _, _, _) => {
            h1004
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1005, _, _) => {
            h1005
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1006, _) => {
            h1006
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1007) => {
            h1007
        }
    })?, f35(v0, v3)?))
}

pub fn f38(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f53(v0, v2.clone(), 0u64);
    let v9: u64 = f53(v0, v2.clone(), 1u64);
    let v10: u64 = f53(v0, v2.clone(), 2u64);
    let v11: u64 = f53(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f54(v1.clone(), f55(v8, v3, match v1.clone() {
            Adt3::C0(h1008, _, _, _, _, _, _, _) => {
                h1008
            }
        })?, f55(v9, v4, match v1.clone() {
            Adt3::C0(_, h1009, _, _, _, _, _, _) => {
                h1009
            }
        })?, f55(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1010, _, _, _, _, _) => {
                h1010
            }
        })?, f55(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1011, _, _, _, _) => {
                h1011
            }
        })?)?, f35(v0, v2)?))
    } else {
        if if if if f56(v8, v3, match v1.clone() {
            Adt3::C0(h1012, _, _, _, _, _, _, _) => {
                h1012
            }
        })? {
            f56(v9, v4, match v1.clone() {
                Adt3::C0(_, h1013, _, _, _, _, _, _) => {
                    h1013
                }
            })?
        } else {
            false
        } {
            f56(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1014, _, _, _, _, _) => {
                    h1014
                }
            })?
        } else {
            false
        } {
            f56(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1015, _, _, _, _) => {
                    h1015
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f54(v1, f57(v8, v3)?, f57(v9, v4)?, f57(v10, v5)?, f57(v11, v6)?)?, f35(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f35(v0, v2)?))
        }
    }
}

pub fn f39(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f53(v0, v2.clone(), 0u64);
    let v5: u64 = f53(v0, v2.clone(), 1u64);
    let v6: u64 = f53(v0, v2.clone(), 2u64);
    let v7: u64 = f53(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f54(v1.clone(), f58(v4, v3, match v1 {
        Adt3::C0(h1016, _, _, _, _, _, _, _) => {
            h1016
        }
    })?, v5, v6, v7)?, f35(v0, v2)?))
}

pub fn f40(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1017 = v0;
    let a1018 = v3;
    if nat_lt(a1017, a1018) {
        Ok(Adt5::C1(false, {
            let a1019 = v0;
            let a1020 = 4u64;
            nat_mul(a1019, a1020)?
        }, {
            let a1021 = v0;
            let a1022 = 4u64;
            nat_mul(a1021, a1022)?
        }))
    } else {
        let v6: u64 = {
            let a1023 = v0;
            let a1024 = v3;
            nat_sub(a1023, a1024)
        };
        let v7: u64 = f35(v6, v1)?;
        let v8: u64 = f59(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f41(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1025 = v0;
    let a1026 = v3;
    if nat_lt(a1025, a1026) {
        Ok(Adt5::C1(true, 0u64, f35(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1027 = v0;
            let a1028 = v3;
            nat_sub(a1027, a1028)
        };
        let v5: u64 = f36(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f35(v4, v1)?, v5))
    }
}

pub fn f42(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1030 = v1;
        let a1031 = match v3.clone() {
            Adt3::C0(h1029, _, _, _, _, _, _, _) => {
                h1029
            }
        };
        let a1032 = 0u64;
        nat_rem(a1030, a1031, a1032)
    };
    let v5: u64 = {
        let a1038 = {
            let a1034 = v1;
            let a1035 = match v3.clone() {
                Adt3::C0(h1033, _, _, _, _, _, _, _) => {
                    h1033
                }
            };
            let a1036 = 0u64;
            nat_quot(a1034, a1035, a1036)
        };
        let a1039 = match v3.clone() {
            Adt3::C0(_, h1037, _, _, _, _, _, _) => {
                h1037
            }
        };
        let a1040 = 0u64;
        nat_rem(a1038, a1039, a1040)
    };
    let v6: bool = f60(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f36(v1, v2, v3.clone())?
    } else {
        0u64
    }, f35(v1, v3)?))
}

pub fn f43(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f35(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f36(v0, v5, v2.clone())?
    }, f35(v0, v2)?))
}

pub fn f44(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1042 = v0;
        let a1043 = match v2.clone() {
            Adt3::C0(h1041, _, _, _, _, _, _, _) => {
                h1041
            }
        };
        let a1044 = 0u64;
        nat_rem(a1042, a1043, a1044)
    };
    let v7: u64 = {
        let a1046 = v0;
        let a1047 = match v2.clone() {
            Adt3::C0(h1045, _, _, _, _, _, _, _) => {
                h1045
            }
        };
        let a1048 = 0u64;
        nat_quot(a1046, a1047, a1048)
    };
    let v8: u64 = {
        let a1050 = v7;
        let a1051 = match v2.clone() {
            Adt3::C0(_, h1049, _, _, _, _, _, _) => {
                h1049
            }
        };
        let a1052 = 0u64;
        nat_rem(a1050, a1051, a1052)
    };
    let v9: u64 = {
        let a1054 = v7;
        let a1055 = match v2.clone() {
            Adt3::C0(_, h1053, _, _, _, _, _, _) => {
                h1053
            }
        };
        let a1056 = 0u64;
        nat_quot(a1054, a1055, a1056)
    };
    let v10: u64 = {
        let a1058 = v9;
        let a1059 = match v2.clone() {
            Adt3::C0(_, _, h1057, _, _, _, _, _) => {
                h1057
            }
        };
        let a1060 = 0u64;
        nat_rem(a1058, a1059, a1060)
    };
    let v11: u64 = {
        let a1062 = v9;
        let a1063 = match v2.clone() {
            Adt3::C0(_, _, h1061, _, _, _, _, _) => {
                h1061
            }
        };
        let a1064 = 0u64;
        nat_quot(a1062, a1063, a1064)
    };
    let v12: u64 = f35(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1073 = {
                let a1068 = {
                    let a1065 = v10;
                    let a1066 = v4;
                    let a1067 = 0u64;
                    nat_quot(a1065, a1066, a1067)
                };
                let a1069 = v5;
                nat_mul(a1068, a1069)?
            };
            let a1074 = {
                let a1070 = v8;
                let a1071 = v4;
                let a1072 = 0u64;
                nat_quot(a1070, a1071, a1072)
            };
            nat_add(a1073, a1074)?
        };
        let v14: u64 = {
            let a1094 = v6;
            let a1095 = {
                let a1092 = match v1.clone() {
                    Adt3::C0(h1075, _, _, _, _, _, _, _) => {
                        h1075
                    }
                };
                let a1093 = {
                    let a1090 = {
                        let a1076 = v8;
                        let a1077 = v4;
                        let a1078 = 0u64;
                        nat_rem(a1076, a1077, a1078)
                    };
                    let a1091 = {
                        let a1088 = match v1.clone() {
                            Adt3::C0(_, h1079, _, _, _, _, _, _) => {
                                h1079
                            }
                        };
                        let a1089 = {
                            let a1086 = {
                                let a1080 = v10;
                                let a1081 = v4;
                                let a1082 = 0u64;
                                nat_rem(a1080, a1081, a1082)
                            };
                            let a1087 = {
                                let a1084 = match v1.clone() {
                                    Adt3::C0(_, _, h1083, _, _, _, _, _) => {
                                        h1083
                                    }
                                };
                                let a1085 = v13;
                                nat_mul(a1084, a1085)?
                            };
                            nat_add(a1086, a1087)?
                        };
                        nat_mul(a1088, a1089)?
                    };
                    nat_add(a1090, a1091)?
                };
                nat_mul(a1092, a1093)?
            };
            nat_add(a1094, a1095)?
        };
        Ok(Adt5::C1(false, f35(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1101 = {
                let a1099 = {
                    let a1096 = v11;
                    let a1097 = v5;
                    let a1098 = 0u64;
                    nat_rem(a1096, a1097, a1098)
                };
                let a1100 = v4;
                nat_mul(a1099, a1100)?
            };
            let a1102 = v8;
            nat_add(a1101, a1102)?
        };
        let v16: u64 = {
            let a1108 = {
                let a1106 = {
                    let a1103 = v11;
                    let a1104 = v5;
                    let a1105 = 0u64;
                    nat_quot(a1103, a1104, a1105)
                };
                let a1107 = v4;
                nat_mul(a1106, a1107)?
            };
            let a1109 = v10;
            nat_add(a1108, a1109)?
        };
        let a1111 = v15;
        let a1112 = match v1.clone() {
            Adt3::C0(_, h1110, _, _, _, _, _, _) => {
                h1110
            }
        };
        if if nat_lt(a1111, a1112) {
            let a1114 = v16;
            let a1115 = match v1.clone() {
                Adt3::C0(_, _, h1113, _, _, _, _, _) => {
                    h1113
                }
            };
            nat_lt(a1114, a1115)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f35({
                let a1124 = v6;
                let a1125 = {
                    let a1122 = match v1.clone() {
                        Adt3::C0(h1116, _, _, _, _, _, _, _) => {
                            h1116
                        }
                    };
                    let a1123 = {
                        let a1120 = v15;
                        let a1121 = {
                            let a1118 = match v1.clone() {
                                Adt3::C0(_, h1117, _, _, _, _, _, _) => {
                                    h1117
                                }
                            };
                            let a1119 = v16;
                            nat_mul(a1118, a1119)?
                        };
                        nat_add(a1120, a1121)?
                    };
                    nat_mul(a1122, a1123)?
                };
                nat_add(a1124, a1125)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f45(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f35(v0, v2.clone())?;
    let v4: u64 = f35(v0, v1.clone())?;
    let a1130 = {
        let a1127 = v0;
        let a1128 = match v1 {
            Adt3::C0(h1126, _, _, _, _, _, _, _) => {
                h1126
            }
        };
        let a1129 = 0u64;
        nat_rem(a1127, a1128, a1129)
    };
    let a1131 = 0u64;
    if nat_eq(a1130, a1131) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C2(0u64, 2u64, f35({
            let a1132 = v0;
            let a1133 = 1u64;
            nat_sub(a1132, a1133)
        }, v2)?, 0u64, v4, v3))
    }
}

pub fn f46(v0: u64) -> u64 {
    let a1134 = v0;
    let a1135 = 2147483648u64;
    if nat_lt(a1134, a1135) {
        v0
    } else {
        0u64
    }
}

pub fn f47(v0: u64) -> u64 {
    let a1136 = v0;
    let a1137 = 2147483648u64;
    if nat_lt(a1136, a1137) {
        0u64
    } else {
        let a1138 = 4294967296u64;
        let a1139 = v0;
        nat_sub(a1138, a1139)
    }
}

pub fn f48(v0: Adt3, v1: bool) -> R<u64> {
    f61(v0.clone(), {
        let a1141 = match v0.clone() {
            Adt3::C0(h1140, _, _, _, _, _, _, _) => {
                h1140
            }
        };
        let a1142 = 1u64;
        nat_sub(a1141, a1142)
    }, {
        let a1144 = match v0.clone() {
            Adt3::C0(_, h1143, _, _, _, _, _, _) => {
                h1143
            }
        };
        let a1145 = 1u64;
        nat_sub(a1144, a1145)
    }, {
        let a1147 = match v0.clone() {
            Adt3::C0(_, _, h1146, _, _, _, _, _) => {
                h1146
            }
        };
        let a1148 = 1u64;
        nat_sub(a1147, a1148)
    }, {
        let a1150 = match v0 {
            Adt3::C0(_, _, _, h1149, _, _, _, _) => {
                h1149
            }
        };
        let a1151 = 1u64;
        nat_sub(a1150, a1151)
    }, v1)
}

pub fn f49(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1152 = v0;
    let a1153 = 0u64;
    if nat_eq(a1152, a1153) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1154 = v0;
            let a1155 = 1u64;
            nat_sub(a1154, a1155)
        };
        let v5: u64 = {
            let a1156 = v4;
            let a1157 = v1;
            let a1158 = 0u64;
            nat_quot(a1156, a1157, a1158)
        };
        let v6: u64 = {
            let a1166 = {
                let a1159 = v5;
                let a1160 = v2;
                nat_mul(a1159, a1160)?
            };
            let a1167 = {
                let a1164 = {
                    let a1161 = v4;
                    let a1162 = v1;
                    let a1163 = 0u64;
                    nat_rem(a1161, a1162, a1163)
                };
                let a1165 = v3;
                nat_mul(a1164, a1165)?
            };
            nat_add(a1166, a1167)?
        };
        let a1168 = v5;
        let a1169 = 0u64;
        if nat_eq(a1168, a1169) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1178 = {
                    let a1172 = {
                        let a1170 = v5;
                        let a1171 = 1u64;
                        nat_sub(a1170, a1171)
                    };
                    let a1173 = v2;
                    nat_mul(a1172, a1173)?
                };
                let a1179 = {
                    let a1176 = {
                        let a1174 = v1;
                        let a1175 = 1u64;
                        nat_sub(a1174, a1175)
                    };
                    let a1177 = v3;
                    nat_mul(a1176, a1177)?
                };
                nat_add(a1178, a1179)?
            };
            let a1180 = v6;
            let a1181 = v7;
            if nat_lt(a1180, a1181) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1182 = f62(v0, v1);
    match m1182 {
        None => {
            if v5 {
                let m1183 = f63(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1183 {
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
                let a1184 = 4u64;
                let a1185 = v28;
                nat_le(a1184, a1185)
            };
            let a1186 = v2;
            let a1187 = 0u64;
            if if if if if nat_eq(a1186, a1187) {
                let a1188 = v4;
                let a1189 = 0u64;
                nat_eq(a1188, a1189)
            } else {
                false
            } {
                let a1190 = v6;
                let a1191 = v14;
                if nat_eq(a1190, a1191) {
                    let a1192 = v7;
                    let a1193 = v15;
                    nat_eq(a1192, a1193)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1194 = v8;
                let a1195 = v16;
                if nat_eq(a1194, a1195) {
                    let a1196 = v9;
                    let a1197 = v17;
                    nat_eq(a1196, a1197)
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
                        let a1198 = v3;
                        let a1199 = 0u64;
                        nat_eq(a1198, a1199)
                    } else {
                        false
                    } {
                        f22(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1204 = {
                        let a1200 = v6;
                        let a1201 = v7;
                        nat_mul(a1200, a1201)?
                    };
                    let a1205 = {
                        let a1202 = v8;
                        let a1203 = v9;
                        nat_mul(a1202, a1203)?
                    };
                    nat_mul(a1204, a1205)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1218 = {
        let a1214 = {
            let a1210 = {
                let a1206 = v0;
                let a1207 = v4;
                nat_mul(a1206, a1207)?
            };
            let a1211 = {
                let a1208 = v1;
                let a1209 = v5;
                nat_mul(a1208, a1209)?
            };
            nat_add(a1210, a1211)?
        };
        let a1215 = {
            let a1212 = v2;
            let a1213 = v6;
            nat_mul(a1212, a1213)?
        };
        nat_add(a1214, a1215)?
    };
    let a1219 = {
        let a1216 = v3;
        let a1217 = v7;
        nat_mul(a1216, a1217)?
    };
    nat_add(a1218, a1219)
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f51({
        let a1220 = v0;
        let a1221 = v4;
        let a1222 = 0u64;
        nat_rem(a1220, a1221, a1222)
    }, {
        let a1223 = v1;
        let a1224 = v5;
        let a1225 = 0u64;
        nat_rem(a1223, a1224, a1225)
    }, {
        let a1226 = v2;
        let a1227 = v6;
        let a1228 = 0u64;
        nat_rem(a1226, a1227, a1228)
    }, {
        let a1229 = v3;
        let a1230 = v7;
        let a1231 = 0u64;
        nat_rem(a1229, a1230, a1231)
    }, v8, v9, v10, v11)
}

pub fn f53(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1233 = v0;
        let a1234 = match v1.clone() {
            Adt3::C0(h1232, _, _, _, _, _, _, _) => {
                h1232
            }
        };
        let a1235 = 0u64;
        nat_quot(a1233, a1234, a1235)
    };
    let v4: u64 = {
        let a1237 = v3;
        let a1238 = match v1.clone() {
            Adt3::C0(_, h1236, _, _, _, _, _, _) => {
                h1236
            }
        };
        let a1239 = 0u64;
        nat_quot(a1237, a1238, a1239)
    };
    let a1240 = v2;
    let a1241 = 0u64;
    if nat_eq(a1240, a1241) {
        let a1243 = v0;
        let a1244 = match v1 {
            Adt3::C0(h1242, _, _, _, _, _, _, _) => {
                h1242
            }
        };
        let a1245 = 0u64;
        nat_rem(a1243, a1244, a1245)
    } else {
        let a1246 = v2;
        let a1247 = 1u64;
        if nat_eq(a1246, a1247) {
            let a1249 = v3;
            let a1250 = match v1 {
                Adt3::C0(_, h1248, _, _, _, _, _, _) => {
                    h1248
                }
            };
            let a1251 = 0u64;
            nat_rem(a1249, a1250, a1251)
        } else {
            let a1252 = v2;
            let a1253 = 2u64;
            if nat_eq(a1252, a1253) {
                let a1255 = v4;
                let a1256 = match v1 {
                    Adt3::C0(_, _, h1254, _, _, _, _, _) => {
                        h1254
                    }
                };
                let a1257 = 0u64;
                nat_rem(a1255, a1256, a1257)
            } else {
                let a1259 = v4;
                let a1260 = match v1 {
                    Adt3::C0(_, _, h1258, _, _, _, _, _) => {
                        h1258
                    }
                };
                let a1261 = 0u64;
                nat_quot(a1259, a1260, a1261)
            }
        }
    }
}

pub fn f54(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f51(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1262, _, _, _) => {
            h1262
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1263, _, _) => {
            h1263
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1264, _) => {
            h1264
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1265) => {
            h1265
        }
    })
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1266 = v1;
    let a1267 = 2147483648u64;
    if nat_lt(a1266, a1267) {
        let a1278 = {
            let a1276 = {
                let a1271 = {
                    let a1268 = v0;
                    let a1269 = v2;
                    let a1270 = 0u64;
                    nat_rem(a1268, a1269, a1270)
                };
                let a1272 = v2;
                nat_add(a1271, a1272)?
            };
            let a1277 = {
                let a1273 = v1;
                let a1274 = v2;
                let a1275 = 0u64;
                nat_rem(a1273, a1274, a1275)
            };
            nat_sub(a1276, a1277)
        };
        let a1279 = v2;
        let a1280 = 0u64;
        Ok(nat_rem(a1278, a1279, a1280))
    } else {
        let a1283 = {
            let a1281 = v0;
            let a1282 = f47(v1);
            nat_add(a1281, a1282)?
        };
        let a1284 = v2;
        let a1285 = 0u64;
        Ok(nat_rem(a1283, a1284, a1285))
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1288 = f46(v1);
    let a1289 = {
        let a1286 = v0;
        let a1287 = f47(v1);
        nat_add(a1286, a1287)?
    };
    if nat_le(a1288, a1289) {
        let a1290 = f57(v0, v1)?;
        let a1291 = v2;
        Ok(nat_lt(a1290, a1291))
    } else {
        Ok(false)
    }
}

pub fn f57(v0: u64, v1: u64) -> R<u64> {
    let a1294 = {
        let a1292 = v0;
        let a1293 = f47(v1);
        nat_add(a1292, a1293)?
    };
    let a1295 = f46(v1);
    Ok(nat_sub(a1294, a1295))
}

pub fn f58(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1296 = v0;
    let a1297 = v1;
    if nat_lt(a1296, a1297) {
        let a1298 = v1;
        let a1299 = v0;
        Ok(nat_sub(a1298, a1299))
    } else {
        let a1302 = v0;
        let a1303 = {
            let a1300 = v1;
            let a1301 = v2;
            nat_add(a1300, a1301)?
        };
        if nat_lt(a1302, a1303) {
            let a1304 = v0;
            let a1305 = v1;
            Ok(nat_sub(a1304, a1305))
        } else {
            let a1312 = {
                let a1308 = 2u64;
                let a1309 = {
                    let a1306 = v2;
                    let a1307 = 1u64;
                    nat_sub(a1306, a1307)
                };
                nat_mul(a1308, a1309)?
            };
            let a1313 = {
                let a1310 = v0;
                let a1311 = v1;
                nat_sub(a1310, a1311)
            };
            Ok(nat_sub(a1312, a1313))
        }
    }
}

pub fn f59(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1316 = {
        let a1314 = v2;
        let a1315 = f61(v1.clone(), f53(v0, v1.clone(), 0u64), f53(v0, v1.clone(), 1u64), f53(v0, v1.clone(), 2u64), f53(v0, v1.clone(), 3u64), false)?;
        nat_add(a1314, a1315)?
    };
    let a1317 = f61(v1.clone(), f53(v0, v1.clone(), 0u64), f53(v0, v1.clone(), 1u64), f53(v0, v1.clone(), 2u64), f53(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1316, a1317))
}

pub fn f60(v0: u64, v1: u64, v2: u64) -> bool {
    let a1318 = v0;
    let a1319 = 0u64;
    if nat_eq(a1318, a1319) {
        let a1320 = v2;
        let a1321 = v1;
        nat_le(a1320, a1321)
    } else {
        let a1322 = v0;
        let a1323 = 1u64;
        if nat_eq(a1322, a1323) {
            let a1324 = v2;
            let a1325 = v1;
            nat_lt(a1324, a1325)
        } else {
            let a1326 = v0;
            let a1327 = 2u64;
            if nat_eq(a1326, a1327) {
                let a1328 = v1;
                let a1329 = v2;
                nat_le(a1328, a1329)
            } else {
                let a1330 = v0;
                let a1331 = 3u64;
                if nat_eq(a1330, a1331) {
                    let a1332 = v1;
                    let a1333 = v2;
                    nat_lt(a1332, a1333)
                } else {
                    let a1334 = v1;
                    let a1335 = v2;
                    nat_eq(a1334, a1335)
                }
            }
        }
    }
}

pub fn f61(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1356 = {
        let a1350 = {
            let a1344 = {
                let a1338 = v1;
                let a1339 = if v5 {
                    f47(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1336, _, _, _) => {
                            h1336
                        }
                    })
                } else {
                    f46(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1337, _, _, _) => {
                            h1337
                        }
                    })
                };
                nat_mul(a1338, a1339)?
            };
            let a1345 = {
                let a1342 = v2;
                let a1343 = if v5 {
                    f47(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1340, _, _) => {
                            h1340
                        }
                    })
                } else {
                    f46(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1341, _, _) => {
                            h1341
                        }
                    })
                };
                nat_mul(a1342, a1343)?
            };
            nat_add(a1344, a1345)?
        };
        let a1351 = {
            let a1348 = v3;
            let a1349 = if v5 {
                f47(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1346, _) => {
                        h1346
                    }
                })
            } else {
                f46(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1347, _) => {
                        h1347
                    }
                })
            };
            nat_mul(a1348, a1349)?
        };
        nat_add(a1350, a1351)?
    };
    let a1357 = {
        let a1354 = v4;
        let a1355 = if v5 {
            f47(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1352) => {
                    h1352
                }
            })
        } else {
            f46(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1353) => {
                    h1353
                }
            })
        };
        nat_mul(a1354, a1355)?
    };
    nat_add(a1356, a1357)
}

pub fn f62(v0: u64, v1: u64) -> Option<u64> {
    let a1358 = v0;
    let a1359 = 2u64;
    if nat_eq(a1358, a1359) {
        Some(0u64)
    } else {
        let a1360 = v0;
        let a1361 = 5u64;
        if nat_eq(a1360, a1361) {
            Some(1u64)
        } else {
            let a1362 = v0;
            let a1363 = 6u64;
            if nat_eq(a1362, a1363) {
                Some(2u64)
            } else {
                let a1364 = v0;
                let a1365 = 7u64;
                if nat_eq(a1364, a1365) {
                    Some(3u64)
                } else {
                    let a1366 = v0;
                    let a1367 = 9u64;
                    if nat_eq(a1366, a1367) {
                        Some(4u64)
                    } else {
                        let a1368 = v0;
                        let a1369 = 8u64;
                        if nat_eq(a1368, a1369) {
                            Some(5u64)
                        } else {
                            let a1370 = v0;
                            let a1371 = 89u64;
                            if nat_eq(a1370, a1371) {
                                let a1372 = v1;
                                let a1373 = 2u64;
                                if nat_eq(a1372, a1373) {
                                    Some(6u64)
                                } else {
                                    let a1374 = v1;
                                    let a1375 = 0u64;
                                    if nat_eq(a1374, a1375) {
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

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1376 = v0;
    let a1377 = 28u64;
    if if if if nat_eq(a1376, a1377) {
        let a1378 = v1;
        let a1379 = 0u64;
        nat_eq(a1378, a1379)
    } else {
        false
    } {
        let a1380 = v2;
        let a1381 = 0u64;
        nat_eq(a1380, a1381)
    } else {
        false
    } {
        let a1382 = v3;
        let a1383 = 0u64;
        nat_eq(a1382, a1383)
    } else {
        false
    } {
        let a1384 = v4;
        let a1385 = v8;
        if if if if if if if if nat_eq(a1384, a1385) {
            let a1386 = v5;
            let a1387 = v12;
            nat_eq(a1386, a1387)
        } else {
            false
        } {
            let a1388 = v9;
            let a1389 = v13;
            nat_eq(a1388, a1389)
        } else {
            false
        } {
            let a1390 = v10;
            let a1391 = v14;
            nat_eq(a1390, a1391)
        } else {
            false
        } {
            let a1392 = v11;
            let a1393 = v15;
            nat_eq(a1392, a1393)
        } else {
            false
        } {
            let a1394 = 0u64;
            let a1395 = v6;
            nat_lt(a1394, a1395)
        } else {
            false
        } {
            let a1396 = 0u64;
            let a1397 = v7;
            nat_lt(a1396, a1397)
        } else {
            false
        } {
            let a1401 = {
                let a1398 = v10;
                let a1399 = v6;
                let a1400 = 1u64;
                nat_rem(a1398, a1399, a1400)
            };
            let a1402 = 0u64;
            if nat_eq(a1401, a1402) {
                let a1406 = {
                    let a1403 = v11;
                    let a1404 = v7;
                    let a1405 = 1u64;
                    nat_rem(a1403, a1404, a1405)
                };
                let a1407 = 0u64;
                nat_eq(a1406, a1407)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1408 = v10;
                let a1409 = v6;
                let a1410 = 0u64;
                nat_quot(a1408, a1409, a1410)
            }, {
                let a1411 = v11;
                let a1412 = v7;
                let a1413 = 0u64;
                nat_quot(a1411, a1412, a1413)
            }, {
                let a1414 = v5;
                let a1415 = v4;
                nat_mul(a1414, a1415)?
            }, {
                let a1416 = v4;
                let a1417 = v9;
                nat_mul(a1416, a1417)?
            }, {
                let a1418 = v5;
                let a1419 = v9;
                nat_mul(a1418, a1419)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}
