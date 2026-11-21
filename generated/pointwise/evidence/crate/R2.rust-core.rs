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
    C2(u64, Adt3, Adt3, u64),
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
    C16(u64, u64),
    C17,
    C18(u64, Adt3),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
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
                                                            let a42 = v1;
                                                            let a43 = 17u64;
                                                            if nat_eq(a42, a43) {
                                                                f13(v3, v5)?
                                                            } else {
                                                                let a44 = v1;
                                                                let a45 = 18u64;
                                                                if nat_eq(a44, a45) {
                                                                    f14(v3, v4, v5)?
                                                                } else {
                                                                    let a46 = v1;
                                                                    let a47 = 13u64;
                                                                    if if if nat_eq(a46, a47) {
                                                                        true
                                                                    } else {
                                                                        let a48 = v1;
                                                                        let a49 = 14u64;
                                                                        nat_eq(a48, a49)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a50 = v1;
                                                                        let a51 = 16u64;
                                                                        nat_eq(a50, a51)
                                                                    } {
                                                                        f15(v1, v3, v5)?
                                                                    } else {
                                                                        let m52 = f16(v1, v6);
                                                                        match m52 {
                                                                            None => {
                                                                                f17(v1, v6, v3, v4, v5)?
                                                                            }
                                                                            Some(v8) => {
                                                                                f18(v8, v3, v5)?
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
            f19(v9, v10, v11, v12)
        }
        Adt0::C2(v13, v14, v15, v16) => {
            f20(v13, v14, v15, v16)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a53 = v0;
    let a54 = 1u64;
    if if nat_eq(a53, a54) {
        true
    } else {
        let a55 = v0;
        let a56 = 33u64;
        nat_eq(a55, a56)
    } {
        true
    } else {
        let a57 = v0;
        let a58 = 34u64;
        nat_eq(a57, a58)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a61 = match v1.clone() {
        Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
            h59
        }
    };
    let a62 = match v3.clone() {
        Adt2::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
            h60
        }
    };
    if nat_eq(a61, a62) {
        f21(v0, v1, v2, v3)
    } else {
        if if if if f22(match v1.clone() {
            Adt2::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        }) {
            f22(match v3.clone() {
                Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                    h64
                }
            })
        } else {
            false
        } {
            let a65 = v0;
            let a66 = 33u64;
            if nat_eq(a65, a66) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a67 = f23(v1.clone(), 1u64)?;
            let a68 = f23(v3.clone(), 1u64)?;
            nat_eq(a67, a68)
        } else {
            false
        } {
            let m70 = f24(match v3.clone() {
                Adt2::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                    h69
                }
            });
            match m70 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f23(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h71) => {
                            h71
                        }
                    }, f25(v1.clone(), 1u64), f25(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                            h72
                        }
                    }, match v3 {
                        Adt2::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                            h73
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
    let a75 = match v0.clone() {
        Adt2::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
            h74
        }
    };
    let a76 = 0u64;
    if if if if nat_eq(a75, a76) {
        true
    } else {
        let a78 = match v0.clone() {
            Adt2::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                h77
            }
        };
        let a79 = 1u64;
        nat_eq(a78, a79)
    } {
        true
    } else {
        let a81 = match v0.clone() {
            Adt2::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                h80
            }
        };
        let a82 = 25u64;
        if nat_eq(a81, a82) {
            true
        } else {
            let a84 = match v0.clone() {
                Adt2::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                    h83
                }
            };
            let a85 = 26u64;
            nat_eq(a84, a85)
        }
    } {
        true
    } else {
        let a87 = match v0.clone() {
            Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                h86
            }
        };
        let a88 = 30u64;
        nat_eq(a87, a88)
    } {
        let a91 = match v0.clone() {
            Adt2::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                h89
            }
        };
        let a92 = match v1.clone() {
            Adt2::C0(h90, _, _, _, _, _, _, _, _, _, _) => {
                h90
            }
        };
        if if nat_eq(a91, a92) {
            f26(match v1.clone() {
                Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                    h93
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                    h94
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h95, _, _, _, _, _, _, _) => {
                    h95
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h96, _, _, _, _, _, _) => {
                    h96
                }
            }, match v0.clone() {
                Adt2::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                    h97
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h98, _, _, _, _, _, _, _, _) => {
                    h98
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                    h99
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                    h100
                }
            })
        } else {
            false
        } {
            let m102 = f24(match v0.clone() {
                Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                    h101
                }
            });
            match m102 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f23(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h103) => {
                            h103
                        }
                    }, f25(v0, 1u64), f25(v1, 1u64), Adt4::C1))
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
    let m104 = v2;
    match m104 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a105 = v0;
            let a106 = 4u64;
            if if if nat_lt(a105, a106) {
                let a109 = match v1.clone() {
                    Adt2::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
                        h107
                    }
                };
                let a110 = match v4.clone() {
                    Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                        h108
                    }
                };
                if nat_eq(a109, a110) {
                    let a113 = match v1.clone() {
                        Adt2::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
                            h111
                        }
                    };
                    let a114 = match v3.clone() {
                        Adt2::C0(h112, _, _, _, _, _, _, _, _, _, _) => {
                            h112
                        }
                    };
                    nat_eq(a113, a114)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f27(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h115, _, _, _, _, _, _, _, _, _) => {
                        h115
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h116, _, _, _, _, _, _, _, _, _) => {
                        h116
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h117, _, _, _, _, _, _, _, _, _) => {
                        h117
                    }
                })? {
                    f27(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h118, _, _, _, _, _, _, _, _) => {
                            h118
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h119, _, _, _, _, _, _, _, _) => {
                            h119
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h120, _, _, _, _, _, _, _, _) => {
                            h120
                        }
                    })?
                } else {
                    false
                } {
                    f27(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h121, _, _, _, _, _, _, _) => {
                            h121
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h122, _, _, _, _, _, _, _) => {
                            h122
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h123, _, _, _, _, _, _, _) => {
                            h123
                        }
                    })?
                } else {
                    false
                } {
                    f27(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                            h124
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h125, _, _, _, _, _, _) => {
                            h125
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h126, _, _, _, _, _, _) => {
                            h126
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m128 = f24(match v1.clone() {
                    Adt2::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                        h127
                    }
                });
                match m128 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h129, _) => {
                                h129
                            }
                        };
                        let a134 = {
                            let a131 = match v1.clone() {
                                Adt2::C0(_, h130, _, _, _, _, _, _, _, _, _) => {
                                    h130
                                }
                            };
                            let a132 = v6;
                            let a133 = 1u64;
                            nat_rem(a131, a132, a133)
                        };
                        let a135 = 0u64;
                        if if if nat_eq(a134, a135) {
                            let a140 = {
                                let a137 = match v4.clone() {
                                    Adt2::C0(_, h136, _, _, _, _, _, _, _, _, _) => {
                                        h136
                                    }
                                };
                                let a138 = v6;
                                let a139 = 1u64;
                                nat_rem(a137, a138, a139)
                            };
                            let a141 = 0u64;
                            nat_eq(a140, a141)
                        } else {
                            false
                        } {
                            let a146 = {
                                let a143 = match v3.clone() {
                                    Adt2::C0(_, h142, _, _, _, _, _, _, _, _, _) => {
                                        h142
                                    }
                                };
                                let a144 = v6;
                                let a145 = 1u64;
                                nat_rem(a143, a144, a145)
                            };
                            let a147 = 0u64;
                            nat_eq(a146, a147)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f23(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h148) => {
                                    h148
                                }
                            }, f25(v1, v6), f25(v3, v6), Adt4::C2(v0, f25(v4, v6))))
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
    let a150 = match v2.clone() {
        Adt2::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
            h149
        }
    };
    let a151 = 0u64;
    if if nat_eq(a150, a151) {
        let a153 = match v3.clone() {
            Adt2::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
                h152
            }
        };
        let a154 = 0u64;
        nat_eq(a153, a154)
    } else {
        false
    } {
        let a155 = v0;
        let a156 = 61u64;
        if nat_eq(a155, a156) {
            let v4: bool = {
                let a158 = 0u64;
                let a159 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h157, _, _, _, _, _, _, _) => {
                        h157
                    }
                };
                nat_lt(a158, a159)
            };
            if if if if if if f28(match v2.clone() {
                Adt2::C0(_, h160, _, _, _, _, _, _, _, _, _) => {
                    h160
                }
            }, match v1.clone() {
                Adt1::C0(h161, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h161
                }
            }, match v1.clone() {
                Adt1::C0(_, h162, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h162
                }
            }, match v3.clone() {
                Adt2::C0(_, h163, _, _, _, _, _, _, _, _, _) => {
                    h163
                }
            })? {
                f28(match v2.clone() {
                    Adt2::C0(_, _, h164, _, _, _, _, _, _, _, _) => {
                        h164
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h165, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h166, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h166
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h167, _, _, _, _, _, _, _, _) => {
                        h167
                    }
                })?
            } else {
                false
            } {
                f28(match v2.clone() {
                    Adt2::C0(_, _, _, h168, _, _, _, _, _, _, _) => {
                        h168
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h169, _, _, _, _, _, _, _, _, _, _, _) => {
                        h169
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h170, _, _, _, _, _, _, _, _, _, _) => {
                        h170
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h171, _, _, _, _, _, _, _) => {
                        h171
                    }
                })?
            } else {
                false
            } {
                f28(match v2.clone() {
                    Adt2::C0(_, _, _, _, h172, _, _, _, _, _, _) => {
                        h172
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h173, _, _, _, _, _, _, _, _, _) => {
                        h173
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h174, _, _, _, _, _, _, _, _) => {
                        h174
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h175, _, _, _, _, _, _) => {
                        h175
                    }
                })?
            } else {
                false
            } {
                let a177 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h176, _, _, _, _, _, _, _) => {
                        h176
                    }
                };
                let a178 = 4294967295u64;
                nat_le(a177, a178)
            } else {
                false
            } {
                if v4 {
                    !f29(match v2.clone() {
                        Adt2::C0(_, h179, _, _, _, _, _, _, _, _, _) => {
                            h179
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h180, _, _, _, _, _, _, _, _) => {
                            h180
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h181, _, _, _, _, _, _, _) => {
                            h181
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h182, _, _, _, _, _, _) => {
                            h182
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h183, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h183
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h184, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h184
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h185, _, _, _, _, _, _, _, _, _, _, _) => {
                        h185
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h186, _, _, _, _, _, _, _, _, _) => {
                        h186
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a187 = v0;
            let a188 = 62u64;
            if nat_eq(a187, a188) {
                let a190 = match v1.clone() {
                    Adt1::C0(h189, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h189
                    }
                };
                let a191 = 2147483648u64;
                if if if if if nat_lt(a190, a191) {
                    let a193 = match v1.clone() {
                        Adt1::C0(_, h192, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h192
                        }
                    };
                    let a194 = 2147483648u64;
                    nat_lt(a193, a194)
                } else {
                    false
                } {
                    let a197 = match v1.clone() {
                        Adt1::C0(h195, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h195
                        }
                    };
                    let a198 = match v2.clone() {
                        Adt2::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                            h196
                        }
                    };
                    nat_lt(a197, a198)
                } else {
                    false
                } {
                    let a201 = match v1.clone() {
                        Adt1::C0(_, h199, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h199
                        }
                    };
                    let a202 = match v2.clone() {
                        Adt2::C0(_, h200, _, _, _, _, _, _, _, _, _) => {
                            h200
                        }
                    };
                    nat_lt(a201, a202)
                } else {
                    false
                } {
                    let a205 = match v2.clone() {
                        Adt2::C0(_, _, h203, _, _, _, _, _, _, _, _) => {
                            h203
                        }
                    };
                    let a206 = match v3.clone() {
                        Adt2::C0(_, _, h204, _, _, _, _, _, _, _, _) => {
                            h204
                        }
                    };
                    if if if nat_eq(a205, a206) {
                        let a209 = match v2.clone() {
                            Adt2::C0(_, _, _, h207, _, _, _, _, _, _, _) => {
                                h207
                            }
                        };
                        let a210 = match v3.clone() {
                            Adt2::C0(_, _, _, h208, _, _, _, _, _, _, _) => {
                                h208
                            }
                        };
                        nat_eq(a209, a210)
                    } else {
                        false
                    } {
                        let a213 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h211, _, _, _, _, _, _) => {
                                h211
                            }
                        };
                        let a214 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h212, _, _, _, _, _, _) => {
                                h212
                            }
                        };
                        nat_eq(a213, a214)
                    } else {
                        false
                    } {
                        let a223 = match v3.clone() {
                            Adt2::C0(_, h215, _, _, _, _, _, _, _, _, _) => {
                                h215
                            }
                        };
                        let a224 = {
                            let a221 = {
                                let a218 = match v2.clone() {
                                    Adt2::C0(_, h216, _, _, _, _, _, _, _, _, _) => {
                                        h216
                                    }
                                };
                                let a219 = match v1.clone() {
                                    Adt1::C0(h217, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h217
                                    }
                                };
                                nat_add(a218, a219)?
                            };
                            let a222 = match v1.clone() {
                                Adt1::C0(_, h220, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h220
                                }
                            };
                            nat_add(a221, a222)?
                        };
                        nat_eq(a223, a224)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h225, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h225
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a226 = v0;
                let a227 = 63u64;
                if nat_eq(a226, a227) {
                    if if if if if f30(match v2.clone() {
                        Adt2::C0(_, h228, _, _, _, _, _, _, _, _, _) => {
                            h228
                        }
                    }, match v1.clone() {
                        Adt1::C0(h229, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h229
                        }
                    })? {
                        f30(match v2.clone() {
                            Adt2::C0(_, _, h230, _, _, _, _, _, _, _, _) => {
                                h230
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h231, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h231
                            }
                        })?
                    } else {
                        false
                    } {
                        f30(match v2.clone() {
                            Adt2::C0(_, _, _, h232, _, _, _, _, _, _, _) => {
                                h232
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h233, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h233
                            }
                        })?
                    } else {
                        false
                    } {
                        f30(match v2.clone() {
                            Adt2::C0(_, _, _, _, h234, _, _, _, _, _, _) => {
                                h234
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h235, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h235
                            }
                        })?
                    } else {
                        false
                    } {
                        let a238 = match v2.clone() {
                            Adt2::C0(_, h236, _, _, _, _, _, _, _, _, _) => {
                                h236
                            }
                        };
                        let a239 = match v3.clone() {
                            Adt2::C0(_, h237, _, _, _, _, _, _, _, _, _) => {
                                h237
                            }
                        };
                        if if if nat_eq(a238, a239) {
                            let a242 = match v2.clone() {
                                Adt2::C0(_, _, h240, _, _, _, _, _, _, _, _) => {
                                    h240
                                }
                            };
                            let a243 = match v3.clone() {
                                Adt2::C0(_, _, h241, _, _, _, _, _, _, _, _) => {
                                    h241
                                }
                            };
                            nat_eq(a242, a243)
                        } else {
                            false
                        } {
                            let a246 = match v2.clone() {
                                Adt2::C0(_, _, _, h244, _, _, _, _, _, _, _) => {
                                    h244
                                }
                            };
                            let a247 = match v3.clone() {
                                Adt2::C0(_, _, _, h245, _, _, _, _, _, _, _) => {
                                    h245
                                }
                            };
                            nat_eq(a246, a247)
                        } else {
                            false
                        } {
                            let a250 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h248, _, _, _, _, _, _) => {
                                    h248
                                }
                            };
                            let a251 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h249, _, _, _, _, _, _) => {
                                    h249
                                }
                            };
                            nat_eq(a250, a251)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h252, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h252
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h253, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h253
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h254, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h254
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h255, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h255
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
    let m256 = v3;
    match m256 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a257 = v0;
                let a258 = 4u64;
                nat_eq(a257, a258)
            };
            let v7: bool = {
                let a260 = 0u64;
                let a261 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h259, _, _, _, _, _, _, _, _, _, _, _) => {
                        h259
                    }
                };
                nat_lt(a260, a261)
            };
            let v8: u64 = f23(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h262, _, _, _, _, _, _, _, _, _) => {
                    h262
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h263, _, _, _, _, _, _, _, _) => {
                    h263
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h264, _, _, _, _, _, _, _) => {
                    h264
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h265, _, _, _, _, _, _) => {
                    h265
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h266, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h266
                }
            }, match v1.clone() {
                Adt1::C0(_, h267, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h267
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h268, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h268
                }
            });
            if if if if if if if if if if if if v6 {
                let a270 = match v2.clone() {
                    Adt2::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                        h269
                    }
                };
                let a271 = 0u64;
                if nat_eq(a270, a271) {
                    let a273 = match v5.clone() {
                        Adt2::C0(h272, _, _, _, _, _, _, _, _, _, _) => {
                            h272
                        }
                    };
                    let a274 = 0u64;
                    nat_eq(a273, a274)
                } else {
                    false
                }
            } else {
                let a276 = match v2.clone() {
                    Adt2::C0(h275, _, _, _, _, _, _, _, _, _, _) => {
                        h275
                    }
                };
                let a277 = 0u64;
                if if nat_eq(a276, a277) {
                    true
                } else {
                    let a279 = match v2.clone() {
                        Adt2::C0(h278, _, _, _, _, _, _, _, _, _, _) => {
                            h278
                        }
                    };
                    let a280 = 26u64;
                    nat_eq(a279, a280)
                } {
                    let a282 = match v5.clone() {
                        Adt2::C0(h281, _, _, _, _, _, _, _, _, _, _) => {
                            h281
                        }
                    };
                    let a283 = 0u64;
                    if nat_eq(a282, a283) {
                        true
                    } else {
                        let a285 = match v5.clone() {
                            Adt2::C0(h284, _, _, _, _, _, _, _, _, _, _) => {
                                h284
                            }
                        };
                        let a286 = 26u64;
                        nat_eq(a285, a286)
                    }
                } else {
                    false
                }
            } {
                let a289 = match v2.clone() {
                    Adt2::C0(h287, _, _, _, _, _, _, _, _, _, _) => {
                        h287
                    }
                };
                let a290 = match v4.clone() {
                    Adt2::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                        h288
                    }
                };
                nat_eq(a289, a290)
            } else {
                false
            } {
                f31(v2.clone(), v4.clone())
            } else {
                false
            } {
                f32(v2.clone())?
            } else {
                false
            } {
                f32(v4.clone())?
            } else {
                false
            } {
                let a292 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h291, _, _, _, _, _) => {
                        h291
                    }
                };
                let a293 = 4u64;
                nat_eq(a292, a293)
            } else {
                false
            } {
                let a294 = v8;
                let a295 = f23(v2.clone(), 1u64)?;
                nat_le(a294, a295)
            } else {
                false
            } {
                let a297 = match v1.clone() {
                    Adt1::C0(_, _, _, h296, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h296
                    }
                };
                let a298 = 1073741824u64;
                nat_lt(a297, a298)
            } else {
                false
            } {
                let a300 = match v1.clone() {
                    Adt1::C0(h299, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h299
                    }
                };
                let a301 = 4294967295u64;
                if if if if nat_le(a300, a301) {
                    let a303 = match v1.clone() {
                        Adt1::C0(_, h302, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h302
                        }
                    };
                    let a304 = 4294967295u64;
                    nat_le(a303, a304)
                } else {
                    false
                } {
                    let a306 = match v1.clone() {
                        Adt1::C0(_, _, h305, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h305
                        }
                    };
                    let a307 = 4294967295u64;
                    nat_le(a306, a307)
                } else {
                    false
                } {
                    let a309 = match v1.clone() {
                        Adt1::C0(_, _, _, h308, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h308
                        }
                    };
                    let a310 = 4294967295u64;
                    nat_le(a309, a310)
                } else {
                    false
                } {
                    let a312 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h311, _, _, _, _, _, _, _, _, _, _, _) => {
                            h311
                        }
                    };
                    let a313 = 4294967295u64;
                    nat_le(a312, a313)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a316 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h314) => {
                            h314
                        }
                    };
                    let a317 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h315) => {
                            h315
                        }
                    };
                    nat_eq(a316, a317)
                } else {
                    true
                }
            } else {
                false
            } {
                f33(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h318, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h318
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h319, _) => {
                        h319
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f23(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a320 = v10;
                    let a321 = v8;
                    nat_add(a320, a321)?
                }, 4u64, f25(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h322, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h322
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a324 = match v0.clone() {
        Adt2::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
            h323
        }
    };
    let a325 = 0u64;
    if if if nat_eq(a324, a325) {
        let a327 = match v1.clone() {
            Adt2::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
                h326
            }
        };
        let a328 = 0u64;
        nat_eq(a327, a328)
    } else {
        false
    } {
        f26(match v0.clone() {
            Adt2::C0(_, h329, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                h330
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h331, _, _, _, _, _, _, _) => {
                h331
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h332, _, _, _, _, _, _) => {
                h332
            }
        }, match v1.clone() {
            Adt2::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                h333
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h334, _, _, _, _, _, _, _, _) => {
                h334
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h335, _, _, _, _, _, _, _) => {
                h335
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h336, _, _, _, _, _, _) => {
                h336
            }
        })
    } else {
        false
    } {
        let v2: u64 = f23(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a337 = v2;
            let a338 = f23(v0.clone(), 1u64)?;
            nat_add(a337, a338)?
        }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a340 = match v1.clone() {
        Adt2::C0(h339, _, _, _, _, _, _, _, _, _, _) => {
            h339
        }
    };
    let a341 = 0u64;
    if if if nat_eq(a340, a341) {
        true
    } else {
        let a343 = match v1.clone() {
            Adt2::C0(h342, _, _, _, _, _, _, _, _, _, _) => {
                h342
            }
        };
        let a344 = 1u64;
        nat_eq(a343, a344)
    } {
        true
    } else {
        let a346 = match v1.clone() {
            Adt2::C0(h345, _, _, _, _, _, _, _, _, _, _) => {
                h345
            }
        };
        let a347 = 30u64;
        nat_eq(a346, a347)
    } {
        let a350 = match v1.clone() {
            Adt2::C0(h348, _, _, _, _, _, _, _, _, _, _) => {
                h348
            }
        };
        let a351 = match v2.clone() {
            Adt2::C0(h349, _, _, _, _, _, _, _, _, _, _) => {
                h349
            }
        };
        if if if if if nat_eq(a350, a351) {
            let a354 = match v1.clone() {
                Adt2::C0(_, h352, _, _, _, _, _, _, _, _, _) => {
                    h352
                }
            };
            let a355 = match v2.clone() {
                Adt2::C0(_, h353, _, _, _, _, _, _, _, _, _) => {
                    h353
                }
            };
            nat_eq(a354, a355)
        } else {
            false
        } {
            let a358 = match v1.clone() {
                Adt2::C0(_, _, _, h356, _, _, _, _, _, _, _) => {
                    h356
                }
            };
            let a359 = match v2.clone() {
                Adt2::C0(_, _, _, h357, _, _, _, _, _, _, _) => {
                    h357
                }
            };
            nat_eq(a358, a359)
        } else {
            false
        } {
            let a362 = match v1.clone() {
                Adt2::C0(_, _, _, _, h360, _, _, _, _, _, _) => {
                    h360
                }
            };
            let a363 = match v2.clone() {
                Adt2::C0(_, _, _, _, h361, _, _, _, _, _, _) => {
                    h361
                }
            };
            nat_eq(a362, a363)
        } else {
            false
        } {
            let a364 = v0;
            let a365 = 4u64;
            if nat_eq(a364, a365) {
                let a367 = match v1.clone() {
                    Adt2::C0(_, _, h366, _, _, _, _, _, _, _, _) => {
                        h366
                    }
                };
                let a368 = 1u64;
                if nat_eq(a367, a368) {
                    let a371 = match v1.clone() {
                        Adt2::C0(_, h369, _, _, _, _, _, _, _, _, _) => {
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
            } else {
                let a373 = v0;
                let a374 = 4u64;
                if if nat_lt(a373, a374) {
                    let a377 = match v1.clone() {
                        Adt2::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                            h375
                        }
                    };
                    let a378 = match v1.clone() {
                        Adt2::C0(_, _, h376, _, _, _, _, _, _, _, _) => {
                            h376
                        }
                    };
                    nat_eq(a377, a378)
                } else {
                    false
                } {
                    let a381 = match v1.clone() {
                        Adt2::C0(_, _, h379, _, _, _, _, _, _, _, _) => {
                            h379
                        }
                    };
                    let a382 = match v2.clone() {
                        Adt2::C0(_, _, h380, _, _, _, _, _, _, _, _) => {
                            h380
                        }
                    };
                    nat_eq(a381, a382)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m384 = f24(match v1.clone() {
                Adt2::C0(h383, _, _, _, _, _, _, _, _, _, _) => {
                    h383
                }
            });
            match m384 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f23(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h385) => {
                            h385
                        }
                    }, f25(v1, 1u64), f25(v2, 1u64), Adt4::C8(v0)))
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
    if if if f34(match v1.clone() {
        Adt2::C0(h386, _, _, _, _, _, _, _, _, _, _) => {
            h386
        }
    }, v0) {
        let a389 = match v1.clone() {
            Adt2::C0(h387, _, _, _, _, _, _, _, _, _, _) => {
                h387
            }
        };
        let a390 = match v2.clone() {
            Adt2::C0(h388, _, _, _, _, _, _, _, _, _, _) => {
                h388
            }
        };
        nat_eq(a389, a390)
    } else {
        false
    } {
        let a393 = match v1.clone() {
            Adt2::C0(_, h391, _, _, _, _, _, _, _, _, _) => {
                h391
            }
        };
        let a394 = match v2.clone() {
            Adt2::C0(_, h392, _, _, _, _, _, _, _, _, _) => {
                h392
            }
        };
        if if if nat_eq(a393, a394) {
            let a397 = match v1.clone() {
                Adt2::C0(_, _, h395, _, _, _, _, _, _, _, _) => {
                    h395
                }
            };
            let a398 = match v2.clone() {
                Adt2::C0(_, _, h396, _, _, _, _, _, _, _, _) => {
                    h396
                }
            };
            nat_eq(a397, a398)
        } else {
            false
        } {
            let a401 = match v1.clone() {
                Adt2::C0(_, _, _, h399, _, _, _, _, _, _, _) => {
                    h399
                }
            };
            let a402 = match v2.clone() {
                Adt2::C0(_, _, _, h400, _, _, _, _, _, _, _) => {
                    h400
                }
            };
            nat_eq(a401, a402)
        } else {
            false
        } {
            let a405 = match v1.clone() {
                Adt2::C0(_, _, _, _, h403, _, _, _, _, _, _) => {
                    h403
                }
            };
            let a406 = match v2.clone() {
                Adt2::C0(_, _, _, _, h404, _, _, _, _, _, _) => {
                    h404
                }
            };
            nat_eq(a405, a406)
        } else {
            false
        }
    } else {
        false
    } {
        let m408 = f24(match v1.clone() {
            Adt2::C0(h407, _, _, _, _, _, _, _, _, _, _) => {
                h407
            }
        });
        match m408 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f23(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h409) => {
                        h409
                    }
                }, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h410, _, _, _, _, _, _, _, _, _, _) => {
                        h410
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
            Adt1::C0(h411, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h411
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h412, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h412
            }
        }
    };
    let a413 = 0u64;
    let a414 = v4;
    if if nat_lt(a413, a414) {
        let a415 = v4;
        let a416 = 2147483648u64;
        nat_lt(a415, a416)
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
        let v7: u64 = f35(match v5.clone() {
            Adt2::C0(_, _, h417, _, _, _, _, _, _, _, _) => {
                h417
            }
        }, v4)?;
        let v8: u64 = f35(match v5.clone() {
            Adt2::C0(_, _, _, h418, _, _, _, _, _, _, _) => {
                h418
            }
        }, v4)?;
        let a420 = match v5.clone() {
            Adt2::C0(h419, _, _, _, _, _, _, _, _, _, _) => {
                h419
            }
        };
        let a421 = 0u64;
        if if if if if nat_eq(a420, a421) {
            let a423 = match v6.clone() {
                Adt2::C0(h422, _, _, _, _, _, _, _, _, _, _) => {
                    h422
                }
            };
            let a424 = 0u64;
            nat_eq(a423, a424)
        } else {
            false
        } {
            let a426 = match v5.clone() {
                Adt2::C0(_, _, _, _, h425, _, _, _, _, _, _) => {
                    h425
                }
            };
            let a427 = 1u64;
            nat_eq(a426, a427)
        } else {
            false
        } {
            let a430 = match v5.clone() {
                Adt2::C0(_, h428, _, _, _, _, _, _, _, _, _) => {
                    h428
                }
            };
            let a431 = match v6.clone() {
                Adt2::C0(_, h429, _, _, _, _, _, _, _, _, _) => {
                    h429
                }
            };
            nat_eq(a430, a431)
        } else {
            false
        } {
            if v0 {
                let a433 = match v5.clone() {
                    Adt2::C0(_, _, h432, _, _, _, _, _, _, _, _) => {
                        h432
                    }
                };
                let a434 = 2147483648u64;
                if if if nat_lt(a433, a434) {
                    let a436 = match v5.clone() {
                        Adt2::C0(_, _, _, h435, _, _, _, _, _, _, _) => {
                            h435
                        }
                    };
                    let a437 = 2147483648u64;
                    nat_lt(a436, a437)
                } else {
                    false
                } {
                    let a440 = {
                        let a438 = v7;
                        let a439 = v8;
                        nat_mul(a438, a439)?
                    };
                    let a441 = 2147483648u64;
                    nat_le(a440, a441)
                } else {
                    false
                } {
                    f36(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a443 = match v1.clone() {
                    Adt1::C0(h442, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h442
                    }
                };
                let a444 = v7;
                if if if if nat_eq(a443, a444) {
                    let a446 = match v1 {
                        Adt1::C0(_, h445, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h445
                        }
                    };
                    let a447 = v8;
                    nat_eq(a446, a447)
                } else {
                    false
                } {
                    f37(v7, v8, v4)?
                } else {
                    false
                } {
                    let a449 = match v6.clone() {
                        Adt2::C0(_, _, h448, _, _, _, _, _, _, _, _) => {
                            h448
                        }
                    };
                    let a450 = v4;
                    if nat_eq(a449, a450) {
                        let a452 = match v6.clone() {
                            Adt2::C0(_, _, _, h451, _, _, _, _, _, _, _) => {
                                h451
                            }
                        };
                        let a453 = v4;
                        nat_eq(a452, a453)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a457 = match v6 {
                        Adt2::C0(_, _, _, _, h454, _, _, _, _, _, _) => {
                            h454
                        }
                    };
                    let a458 = {
                        let a455 = v7;
                        let a456 = v8;
                        nat_mul(a455, a456)?
                    };
                    nat_eq(a457, a458)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a460 = match v2.clone() {
        Adt2::C0(h459, _, _, _, _, _, _, _, _, _, _) => {
            h459
        }
    };
    let a461 = 0u64;
    if if if if if nat_eq(a460, a461) {
        true
    } else {
        let a463 = match v2.clone() {
            Adt2::C0(h462, _, _, _, _, _, _, _, _, _, _) => {
                h462
            }
        };
        let a464 = 1u64;
        nat_eq(a463, a464)
    } {
        true
    } else {
        let a466 = match v2.clone() {
            Adt2::C0(h465, _, _, _, _, _, _, _, _, _, _) => {
                h465
            }
        };
        let a467 = 30u64;
        nat_eq(a466, a467)
    } {
        let a470 = match v2.clone() {
            Adt2::C0(h468, _, _, _, _, _, _, _, _, _, _) => {
                h468
            }
        };
        let a471 = match v3.clone() {
            Adt2::C0(h469, _, _, _, _, _, _, _, _, _, _) => {
                h469
            }
        };
        nat_eq(a470, a471)
    } else {
        false
    } {
        if if f31(v2.clone(), v3.clone()) {
            let a473 = match v1.clone() {
                Adt1::C0(h472, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h472
                }
            };
            let a474 = 4294967296u64;
            nat_lt(a473, a474)
        } else {
            false
        } {
            let a476 = match v1.clone() {
                Adt1::C0(_, h475, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h475
                }
            };
            let a477 = 4294967296u64;
            nat_lt(a476, a477)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f23(v3.clone(), 1u64)?, {
            let a479 = match v2.clone() {
                Adt2::C0(h478, _, _, _, _, _, _, _, _, _, _) => {
                    h478
                }
            };
            let a480 = 0u64;
            if nat_eq(a479, a480) {
                4u64
            } else {
                2u64
            }
        }, f25(v2.clone(), 1u64), f25(v3, 1u64), Adt4::C13({
            let a481 = v0;
            let a482 = 31u64;
            if nat_eq(a481, a482) {
                0u64
            } else {
                let a483 = v0;
                let a484 = 47u64;
                if nat_eq(a483, a484) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h485, _, _, _, _, _, _, _, _, _, _) => {
                h485
            }
        }, match v1.clone() {
            Adt1::C0(h486, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h486
            }
        }, match v1 {
            Adt1::C0(_, h487, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h487
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f12(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a489 = match v0.clone() {
        Adt2::C0(h488, _, _, _, _, _, _, _, _, _, _) => {
            h488
        }
    };
    let a490 = 0u64;
    if if if nat_eq(a489, a490) {
        let a492 = match v1.clone() {
            Adt2::C0(h491, _, _, _, _, _, _, _, _, _, _) => {
                h491
            }
        };
        let a493 = 0u64;
        nat_eq(a492, a493)
    } else {
        false
    } {
        f31(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f23(v1.clone(), 1u64)?, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f13(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a495 = match v0.clone() {
        Adt2::C0(h494, _, _, _, _, _, _, _, _, _, _) => {
            h494
        }
    };
    let a496 = 0u64;
    if if nat_eq(a495, a496) {
        let a498 = match v1.clone() {
            Adt2::C0(h497, _, _, _, _, _, _, _, _, _, _) => {
                h497
            }
        };
        let a499 = 26u64;
        nat_eq(a498, a499)
    } else {
        false
    } {
        let a501 = match v0.clone() {
            Adt2::C0(_, _, _, h500, _, _, _, _, _, _, _) => {
                h500
            }
        };
        let a502 = 1u64;
        if if if if if if if nat_eq(a501, a502) {
            let a504 = match v0.clone() {
                Adt2::C0(_, _, _, _, h503, _, _, _, _, _, _) => {
                    h503
                }
            };
            let a505 = 1u64;
            nat_eq(a504, a505)
        } else {
            false
        } {
            let a507 = match v0.clone() {
                Adt2::C0(_, h506, _, _, _, _, _, _, _, _, _) => {
                    h506
                }
            };
            let a508 = 2147483647u64;
            nat_le(a507, a508)
        } else {
            false
        } {
            let a511 = match v1.clone() {
                Adt2::C0(_, h509, _, _, _, _, _, _, _, _, _) => {
                    h509
                }
            };
            let a512 = match v0.clone() {
                Adt2::C0(_, _, h510, _, _, _, _, _, _, _, _) => {
                    h510
                }
            };
            nat_eq(a511, a512)
        } else {
            false
        } {
            let a514 = match v1.clone() {
                Adt2::C0(_, _, h513, _, _, _, _, _, _, _, _) => {
                    h513
                }
            };
            let a515 = 1u64;
            nat_eq(a514, a515)
        } else {
            false
        } {
            let a517 = match v1.clone() {
                Adt2::C0(_, _, _, h516, _, _, _, _, _, _, _) => {
                    h516
                }
            };
            let a518 = 1u64;
            nat_eq(a517, a518)
        } else {
            false
        } {
            let a520 = match v1.clone() {
                Adt2::C0(_, _, _, _, h519, _, _, _, _, _, _) => {
                    h519
                }
            };
            let a521 = 1u64;
            nat_eq(a520, a521)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a524 = match v0.clone() {
                    Adt2::C0(_, _, h522, _, _, _, _, _, _, _, _) => {
                        h522
                    }
                };
                let a525 = f38(match v0.clone() {
                    Adt2::C0(_, h523, _, _, _, _, _, _, _, _, _) => {
                        h523
                    }
                })?;
                nat_mul(a524, a525)?
            }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f14(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m526 = v1;
    match m526 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f39(f25(v0.clone(), 1u64))?;
            let a528 = match v0.clone() {
                Adt2::C0(h527, _, _, _, _, _, _, _, _, _, _) => {
                    h527
                }
            };
            let a529 = 26u64;
            if if if nat_eq(a528, a529) {
                let a531 = match v3.clone() {
                    Adt2::C0(h530, _, _, _, _, _, _, _, _, _, _) => {
                        h530
                    }
                };
                let a532 = 26u64;
                nat_eq(a531, a532)
            } else {
                false
            } {
                let a534 = match v2.clone() {
                    Adt2::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
                        h533
                    }
                };
                let a535 = 27u64;
                nat_eq(a534, a535)
            } else {
                false
            } {
                if if if if if if f31(v0.clone(), v3.clone()) {
                    let a537 = match v2.clone() {
                        Adt2::C0(_, h536, _, _, _, _, _, _, _, _, _) => {
                            h536
                        }
                    };
                    let a538 = 1u64;
                    nat_eq(a537, a538)
                } else {
                    false
                } {
                    let a540 = match v2.clone() {
                        Adt2::C0(_, _, h539, _, _, _, _, _, _, _, _) => {
                            h539
                        }
                    };
                    let a541 = 1u64;
                    nat_eq(a540, a541)
                } else {
                    false
                } {
                    let a543 = match v2.clone() {
                        Adt2::C0(_, _, _, h542, _, _, _, _, _, _, _) => {
                            h542
                        }
                    };
                    let a544 = 1u64;
                    nat_eq(a543, a544)
                } else {
                    false
                } {
                    let a546 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h545, _, _, _, _, _, _) => {
                            h545
                        }
                    };
                    let a547 = 1u64;
                    nat_eq(a546, a547)
                } else {
                    false
                } {
                    let a548 = v4;
                    let a549 = 9223372036854775808u64;
                    nat_lt(a548, a549)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a552 = 2u64;
                        let a553 = {
                            let a550 = v4;
                            let a551 = 2u64;
                            nat_mul(a550, a551)?
                        };
                        nat_add(a552, a553)?
                    }, 8u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C18(v4, f25(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h554, _, _, _, _, _, _, _, _, _, _) => {
            h554
        }
    };
    let a555 = v3;
    let a556 = 0u64;
    if if nat_eq(a555, a556) {
        true
    } else {
        let a557 = v0;
        let a558 = 13u64;
        if nat_eq(a557, a558) {
            let a559 = v3;
            let a560 = 1u64;
            if nat_eq(a559, a560) {
                true
            } else {
                let a561 = v3;
                let a562 = 30u64;
                nat_eq(a561, a562)
            }
        } else {
            false
        }
    } {
        let a564 = v3;
        let a565 = match v2.clone() {
            Adt2::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
                h563
            }
        };
        if if nat_eq(a564, a565) {
            let a566 = v0;
            let a567 = 13u64;
            if nat_eq(a566, a567) {
                let a569 = match v2.clone() {
                    Adt2::C0(_, h568, _, _, _, _, _, _, _, _, _) => {
                        h568
                    }
                };
                let a570 = 1u64;
                if if if nat_eq(a569, a570) {
                    let a572 = match v2.clone() {
                        Adt2::C0(_, _, h571, _, _, _, _, _, _, _, _) => {
                            h571
                        }
                    };
                    let a573 = 1u64;
                    nat_eq(a572, a573)
                } else {
                    false
                } {
                    let a575 = match v2.clone() {
                        Adt2::C0(_, _, _, h574, _, _, _, _, _, _, _) => {
                            h574
                        }
                    };
                    let a576 = 1u64;
                    nat_eq(a575, a576)
                } else {
                    false
                } {
                    let a578 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h577, _, _, _, _, _, _) => {
                            h577
                        }
                    };
                    let a579 = 1u64;
                    nat_eq(a578, a579)
                } else {
                    false
                }
            } else {
                let a581 = match v2.clone() {
                    Adt2::C0(_, h580, _, _, _, _, _, _, _, _, _) => {
                        h580
                    }
                };
                let a582 = 1u64;
                if if if nat_eq(a581, a582) {
                    let a585 = match v1.clone() {
                        Adt2::C0(_, _, h583, _, _, _, _, _, _, _, _) => {
                            h583
                        }
                    };
                    let a586 = match v2.clone() {
                        Adt2::C0(_, _, h584, _, _, _, _, _, _, _, _) => {
                            h584
                        }
                    };
                    nat_eq(a585, a586)
                } else {
                    false
                } {
                    let a589 = match v1.clone() {
                        Adt2::C0(_, _, _, h587, _, _, _, _, _, _, _) => {
                            h587
                        }
                    };
                    let a590 = match v2.clone() {
                        Adt2::C0(_, _, _, h588, _, _, _, _, _, _, _) => {
                            h588
                        }
                    };
                    nat_eq(a589, a590)
                } else {
                    false
                } {
                    let a593 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h591, _, _, _, _, _, _) => {
                            h591
                        }
                    };
                    let a594 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h592, _, _, _, _, _, _) => {
                            h592
                        }
                    };
                    nat_eq(a593, a594)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f25(v1, 1u64);
            Ok(Adt6::C4(f40(v4.clone(), v0, v3)?, {
                let a595 = v3;
                let a596 = 0u64;
                if nat_eq(a595, a596) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f25(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f16(v0: u64, v1: u64) -> Option<u64> {
    let a597 = v0;
    let a598 = 8u64;
    if nat_eq(a597, a598) {
        Some(5u64)
    } else {
        let a599 = v0;
        let a600 = 9u64;
        if nat_eq(a599, a600) {
            Some(6u64)
        } else {
            let a601 = v0;
            let a602 = 89u64;
            if nat_eq(a601, a602) {
                let a603 = v1;
                let a604 = 0u64;
                if nat_eq(a603, a604) {
                    Some(0u64)
                } else {
                    let a605 = v1;
                    let a606 = 1u64;
                    if nat_eq(a605, a606) {
                        Some(1u64)
                    } else {
                        let a607 = v1;
                        let a608 = 2u64;
                        if nat_eq(a607, a608) {
                            Some(2u64)
                        } else {
                            let a609 = v1;
                            let a610 = 3u64;
                            if nat_eq(a609, a610) {
                                Some(3u64)
                            } else {
                                let a611 = v1;
                                let a612 = 6u64;
                                if nat_eq(a611, a612) {
                                    Some(4u64)
                                } else {
                                    let a613 = v1;
                                    let a614 = 12u64;
                                    if nat_eq(a613, a614) {
                                        Some(7u64)
                                    } else {
                                        let a615 = v1;
                                        let a616 = 11u64;
                                        if nat_eq(a615, a616) {
                                            Some(8u64)
                                        } else {
                                            let a617 = v1;
                                            let a618 = 18u64;
                                            if nat_eq(a617, a618) {
                                                Some(9u64)
                                            } else {
                                                let a619 = v1;
                                                let a620 = 19u64;
                                                if nat_eq(a619, a620) {
                                                    Some(10u64)
                                                } else {
                                                    let a621 = v1;
                                                    let a622 = 20u64;
                                                    if nat_eq(a621, a622) {
                                                        Some(11u64)
                                                    } else {
                                                        let a623 = v1;
                                                        let a624 = 21u64;
                                                        if nat_eq(a623, a624) {
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

pub fn f17(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m625 = f41(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m625 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f25(v2.clone(), 1u64), f25(v4, 1u64), Adt4::C10(v5, v6, {
                let m626 = v3;
                match m626 {
                    None => {
                        f25(v2, 1u64)
                    }
                    Some(v12) => {
                        f25(v12, 1u64)
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

pub fn f18(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a628 = match v1.clone() {
        Adt2::C0(h627, _, _, _, _, _, _, _, _, _, _) => {
            h627
        }
    };
    let a629 = 0u64;
    if if if if if nat_eq(a628, a629) {
        true
    } else {
        let a631 = match v1.clone() {
            Adt2::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                h630
            }
        };
        let a632 = 1u64;
        nat_eq(a631, a632)
    } {
        true
    } else {
        let a634 = match v1.clone() {
            Adt2::C0(h633, _, _, _, _, _, _, _, _, _, _) => {
                h633
            }
        };
        let a635 = 30u64;
        nat_eq(a634, a635)
    } {
        let a638 = match v1.clone() {
            Adt2::C0(h636, _, _, _, _, _, _, _, _, _, _) => {
                h636
            }
        };
        let a639 = match v2.clone() {
            Adt2::C0(h637, _, _, _, _, _, _, _, _, _, _) => {
                h637
            }
        };
        if nat_eq(a638, a639) {
            true
        } else {
            let a641 = match v2.clone() {
                Adt2::C0(h640, _, _, _, _, _, _, _, _, _, _) => {
                    h640
                }
            };
            let a642 = 0u64;
            nat_eq(a641, a642)
        }
    } else {
        false
    } {
        f31(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f23(v2.clone(), 1u64)?, {
            let a644 = match v2.clone() {
                Adt2::C0(h643, _, _, _, _, _, _, _, _, _, _) => {
                    h643
                }
            };
            let a645 = 0u64;
            if nat_eq(a644, a645) {
                4u64
            } else {
                2u64
            }
        }, f25(v1.clone(), 1u64), f25(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h646, _, _, _, _, _, _, _, _, _, _) => {
                h646
            }
        }, match v2 {
            Adt2::C0(h647, _, _, _, _, _, _, _, _, _, _) => {
                h647
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f19(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m648 = v3;
    match m648 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f42(v0, v1)?, f42(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f43(v0, v1, v2.clone())?, f42(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f44(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f45(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f46(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f45(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f47(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f48(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f49(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f42(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f50(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f42(v0, v1)?, f42(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f51(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f42(v0, v1)?, f42(v0, v2)?))
        }
        Adt4::C14(_, v36, v37) => {
            Ok(Adt5::C3(v36, v37, f42(v0, v1)?, f42(v0, v2)?))
        }
        Adt4::C15 => {
            f52(v0, v1, v2)
        }
        Adt4::C16(v38, v39) => {
            f53(v0, v1, v2, v38, v39)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v40, v41) => {
            f54(v0, v1, v41, v40)
        }
    }
}

pub fn f20(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f38(match v1.clone() {
        Adt3::C0(h649, _, _, _, _, _, _, _) => {
            h649
        }
    })?;
    let v5: u64 = {
        let a650 = v0;
        let a651 = v4;
        let a652 = 0u64;
        nat_quot(a650, a651, a652)
    };
    let v6: u64 = {
        let a653 = v0;
        let a654 = v4;
        let a655 = 0u64;
        nat_rem(a653, a654, a655)
    };
    let a656 = v6;
    let a657 = 0u64;
    if nat_eq(a656, a657) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a658 = v6;
        let a659 = 1u64;
        if nat_eq(a658, a659) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a662 = {
                let a660 = v6;
                let a661 = 1u64;
                nat_add(a660, a661)?
            };
            let a663 = v4;
            if nat_eq(a662, a663) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f42(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a664 = v6;
                    let a665 = 2u64;
                    nat_sub(a664, a665)
                };
                let v8: u64 = {
                    let a666 = v7;
                    let a667 = 3u64;
                    let a668 = 0u64;
                    nat_quot(a666, a667, a668)
                };
                let v9: u64 = {
                    let a669 = v7;
                    let a670 = 3u64;
                    let a671 = 0u64;
                    nat_rem(a669, a670, a671)
                };
                let v10: u64 = f42({
                    let a675 = {
                        let a673 = v5;
                        let a674 = match v1.clone() {
                            Adt3::C0(h672, _, _, _, _, _, _, _) => {
                                h672
                            }
                        };
                        nat_mul(a673, a674)?
                    };
                    let a676 = v8;
                    nat_add(a675, a676)?
                }, v1)?;
                let a677 = v9;
                let a678 = 0u64;
                if nat_eq(a677, a678) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a679 = v9;
                    let a680 = 1u64;
                    if nat_eq(a679, a680) {
                        let a681 = v3;
                        let a682 = 2u64;
                        if nat_eq(a681, a682) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a683 = v3;
                        let a684 = 2u64;
                        if if nat_eq(a683, a684) {
                            true
                        } else {
                            let a685 = v3;
                            let a686 = 3u64;
                            nat_eq(a685, a686)
                        } {
                            Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(19u64, 2u64, v8, 2u64, 0u64, 1u64, 1u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f21(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a689 = match v1.clone() {
        Adt2::C0(h687, _, _, _, _, _, _, _, _, _, _) => {
            h687
        }
    };
    let a690 = match v3.clone() {
        Adt2::C0(h688, _, _, _, _, _, _, _, _, _, _) => {
            h688
        }
    };
    if if nat_eq(a689, a690) {
        let a691 = v0;
        let a692 = 33u64;
        if nat_eq(a691, a692) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m694 = f24(match v1.clone() {
            Adt2::C0(h693, _, _, _, _, _, _, _, _, _, _) => {
                h693
            }
        });
        match m694 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h695, _) => {
                        h695
                    }
                };
                let v6: u64 = f23(v1.clone(), v5)?;
                let a700 = {
                    let a697 = match v1.clone() {
                        Adt2::C0(_, h696, _, _, _, _, _, _, _, _, _) => {
                            h696
                        }
                    };
                    let a698 = v5;
                    let a699 = 1u64;
                    nat_rem(a697, a698, a699)
                };
                let a701 = 0u64;
                if if if nat_eq(a700, a701) {
                    let a706 = {
                        let a703 = match v3.clone() {
                            Adt2::C0(_, h702, _, _, _, _, _, _, _, _, _) => {
                                h702
                            }
                        };
                        let a704 = v5;
                        let a705 = 1u64;
                        nat_rem(a703, a704, a705)
                    };
                    let a707 = 0u64;
                    nat_eq(a706, a707)
                } else {
                    false
                } {
                    let a708 = v6;
                    let a709 = f23(v3.clone(), v5)?;
                    nat_eq(a708, a709)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h710) => {
                            h710
                        }
                    }, f25(v1, v5), f25(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f22(v0: u64) -> bool {
    let a711 = v0;
    let a712 = 0u64;
    if if nat_eq(a711, a712) {
        true
    } else {
        let a713 = v0;
        let a714 = 1u64;
        nat_eq(a713, a714)
    } {
        true
    } else {
        let a715 = v0;
        let a716 = 30u64;
        nat_eq(a715, a716)
    }
}

pub fn f23(v0: Adt2, v1: u64) -> R<u64> {
    let a728 = {
        let a722 = {
            let a718 = match v0.clone() {
                Adt2::C0(_, h717, _, _, _, _, _, _, _, _, _) => {
                    h717
                }
            };
            let a719 = v1;
            let a720 = 0u64;
            nat_quot(a718, a719, a720)
        };
        let a723 = match v0.clone() {
            Adt2::C0(_, _, h721, _, _, _, _, _, _, _, _) => {
                h721
            }
        };
        nat_mul(a722, a723)?
    };
    let a729 = {
        let a726 = match v0.clone() {
            Adt2::C0(_, _, _, h724, _, _, _, _, _, _, _) => {
                h724
            }
        };
        let a727 = match v0 {
            Adt2::C0(_, _, _, _, h725, _, _, _, _, _, _) => {
                h725
            }
        };
        nat_mul(a726, a727)?
    };
    nat_mul(a728, a729)
}

pub fn f24(v0: u64) -> Adt7 {
    let a730 = v0;
    let a731 = 0u64;
    if nat_eq(a730, a731) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a732 = v0;
        let a733 = 1u64;
        if nat_eq(a732, a733) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a734 = v0;
            let a735 = 2u64;
            if nat_eq(a734, a735) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a736 = v0;
                let a737 = 3u64;
                if nat_eq(a736, a737) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a738 = v0;
                    let a739 = 6u64;
                    if nat_eq(a738, a739) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a740 = v0;
                        let a741 = 7u64;
                        if nat_eq(a740, a741) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a742 = v0;
                            let a743 = 8u64;
                            if nat_eq(a742, a743) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a744 = v0;
                                let a745 = 9u64;
                                if nat_eq(a744, a745) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a746 = v0;
                                    let a747 = 10u64;
                                    if nat_eq(a746, a747) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a748 = v0;
                                        let a749 = 11u64;
                                        if nat_eq(a748, a749) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a750 = v0;
                                            let a751 = 12u64;
                                            if nat_eq(a750, a751) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a752 = v0;
                                                let a753 = 13u64;
                                                if nat_eq(a752, a753) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a754 = v0;
                                                    let a755 = 14u64;
                                                    if nat_eq(a754, a755) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a756 = v0;
                                                        let a757 = 15u64;
                                                        if nat_eq(a756, a757) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a758 = v0;
                                                            let a759 = 16u64;
                                                            if nat_eq(a758, a759) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a760 = v0;
                                                                let a761 = 17u64;
                                                                if nat_eq(a760, a761) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a762 = v0;
                                                                    let a763 = 18u64;
                                                                    if nat_eq(a762, a763) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a764 = v0;
                                                                        let a765 = 19u64;
                                                                        if nat_eq(a764, a765) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a766 = v0;
                                                                            let a767 = 20u64;
                                                                            if nat_eq(a766, a767) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a768 = v0;
                                                                                let a769 = 21u64;
                                                                                if nat_eq(a768, a769) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a770 = v0;
                                                                                    let a771 = 22u64;
                                                                                    if nat_eq(a770, a771) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a772 = v0;
                                                                                        let a773 = 23u64;
                                                                                        if nat_eq(a772, a773) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a774 = v0;
                                                                                            let a775 = 24u64;
                                                                                            if nat_eq(a774, a775) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a776 = v0;
                                                                                                let a777 = 25u64;
                                                                                                if nat_eq(a776, a777) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a778 = v0;
                                                                                                    let a779 = 26u64;
                                                                                                    if nat_eq(a778, a779) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a780 = v0;
                                                                                                        let a781 = 27u64;
                                                                                                        if nat_eq(a780, a781) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a782 = v0;
                                                                                                            let a783 = 28u64;
                                                                                                            if nat_eq(a782, a783) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a784 = v0;
                                                                                                                let a785 = 29u64;
                                                                                                                if nat_eq(a784, a785) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a786 = v0;
                                                                                                                    let a787 = 30u64;
                                                                                                                    if nat_eq(a786, a787) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a788 = v0;
                                                                                                                        let a789 = 34u64;
                                                                                                                        if nat_eq(a788, a789) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a790 = v0;
                                                                                                                            let a791 = 35u64;
                                                                                                                            if nat_eq(a790, a791) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a792 = v0;
                                                                                                                                let a793 = 39u64;
                                                                                                                                if nat_eq(a792, a793) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a794 = v0;
                                                                                                                                    let a795 = 40u64;
                                                                                                                                    if nat_eq(a794, a795) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a796 = v0;
                                                                                                                                        let a797 = 41u64;
                                                                                                                                        if nat_eq(a796, a797) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a798 = v0;
                                                                                                                                            let a799 = 42u64;
                                                                                                                                            if nat_eq(a798, a799) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a800 = v0;
                                                                                                                                                let a801 = 43u64;
                                                                                                                                                if nat_eq(a800, a801) {
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

pub fn f25(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a803 = match v0.clone() {
            Adt2::C0(_, h802, _, _, _, _, _, _, _, _, _) => {
                h802
            }
        };
        let a804 = v1;
        let a805 = 0u64;
        nat_quot(a803, a804, a805)
    }, match v0.clone() {
        Adt2::C0(_, _, h806, _, _, _, _, _, _, _, _) => {
            h806
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h807, _, _, _, _, _, _, _) => {
            h807
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h808, _, _, _, _, _, _) => {
            h808
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h809, _, _, _, _, _) => {
            h809
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h810, _, _, _, _) => {
            h810
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h811, _, _, _) => {
            h811
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h812, _, _) => {
            h812
        }
    })
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f29(v4, v5, v6, v7) {
        f29(v0, v1, v2, v3)
    } else {
        let a816 = {
            let a813 = v0;
            let a814 = v4;
            let a815 = 1u64;
            nat_rem(a813, a814, a815)
        };
        let a817 = 0u64;
        if if if nat_eq(a816, a817) {
            let a821 = {
                let a818 = v1;
                let a819 = v5;
                let a820 = 1u64;
                nat_rem(a818, a819, a820)
            };
            let a822 = 0u64;
            nat_eq(a821, a822)
        } else {
            false
        } {
            let a826 = {
                let a823 = v2;
                let a824 = v6;
                let a825 = 1u64;
                nat_rem(a823, a824, a825)
            };
            let a827 = 0u64;
            nat_eq(a826, a827)
        } else {
            false
        } {
            let a831 = {
                let a828 = v3;
                let a829 = v7;
                let a830 = 1u64;
                nat_rem(a828, a829, a830)
            };
            let a832 = 0u64;
            nat_eq(a831, a832)
        } else {
            false
        }
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a833 = v0;
    let a834 = v1;
    if nat_eq(a833, a834) {
        let a837 = v4;
        let a838 = {
            let a835 = v2;
            let a836 = v3;
            nat_add(a835, a836)?
        };
        Ok(nat_eq(a837, a838))
    } else {
        let a839 = v2;
        let a840 = v3;
        if nat_eq(a839, a840) {
            let a841 = v4;
            let a842 = v2;
            Ok(nat_eq(a841, a842))
        } else {
            Ok(false)
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a845 = {
            let a843 = v0;
            let a844 = f55(v1);
            nat_add(a843, a844)?
        };
        let a846 = f55(v2);
        nat_add(a845, a846)?
    };
    let v5: u64 = {
        let a847 = f56(v1);
        let a848 = f56(v2);
        nat_add(a847, a848)?
    };
    let a849 = v1;
    let a850 = 4294967295u64;
    if if nat_le(a849, a850) {
        let a851 = v2;
        let a852 = 4294967295u64;
        nat_le(a851, a852)
    } else {
        false
    } {
        let a853 = v5;
        let a854 = v4;
        if nat_le(a853, a854) {
            let a857 = v3;
            let a858 = {
                let a855 = v4;
                let a856 = v5;
                nat_sub(a855, a856)
            };
            Ok(nat_eq(a857, a858))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a859 = v0;
    let a860 = 0u64;
    if if if nat_eq(a859, a860) {
        true
    } else {
        let a861 = v1;
        let a862 = 0u64;
        nat_eq(a861, a862)
    } {
        true
    } else {
        let a863 = v2;
        let a864 = 0u64;
        nat_eq(a863, a864)
    } {
        true
    } else {
        let a865 = v3;
        let a866 = 0u64;
        nat_eq(a865, a866)
    }
}

pub fn f30(v0: u64, v1: u64) -> R<bool> {
    let a867 = v1;
    let a868 = 4294967295u64;
    if nat_le(a867, a868) {
        let a871 = {
            let a869 = f55(v1);
            let a870 = f56(v1);
            nat_add(a869, a870)?
        };
        let a872 = v0;
        Ok(nat_lt(a871, a872))
    } else {
        Ok(false)
    }
}

pub fn f31(v0: Adt2, v1: Adt2) -> bool {
    let a875 = match v0.clone() {
        Adt2::C0(_, h873, _, _, _, _, _, _, _, _, _) => {
            h873
        }
    };
    let a876 = match v1.clone() {
        Adt2::C0(_, h874, _, _, _, _, _, _, _, _, _) => {
            h874
        }
    };
    if if if nat_eq(a875, a876) {
        let a879 = match v0.clone() {
            Adt2::C0(_, _, h877, _, _, _, _, _, _, _, _) => {
                h877
            }
        };
        let a880 = match v1.clone() {
            Adt2::C0(_, _, h878, _, _, _, _, _, _, _, _) => {
                h878
            }
        };
        nat_eq(a879, a880)
    } else {
        false
    } {
        let a883 = match v0.clone() {
            Adt2::C0(_, _, _, h881, _, _, _, _, _, _, _) => {
                h881
            }
        };
        let a884 = match v1.clone() {
            Adt2::C0(_, _, _, h882, _, _, _, _, _, _, _) => {
                h882
            }
        };
        nat_eq(a883, a884)
    } else {
        false
    } {
        let a887 = match v0 {
            Adt2::C0(_, _, _, _, h885, _, _, _, _, _, _) => {
                h885
            }
        };
        let a888 = match v1 {
            Adt2::C0(_, _, _, _, h886, _, _, _, _, _, _) => {
                h886
            }
        };
        nat_eq(a887, a888)
    } else {
        false
    }
}

pub fn f32(v0: Adt2) -> R<bool> {
    let a890 = match v0.clone() {
        Adt2::C0(_, h889, _, _, _, _, _, _, _, _, _) => {
            h889
        }
    };
    let a891 = 1u64;
    if if if if nat_eq(a890, a891) {
        true
    } else {
        let a893 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h892, _, _, _, _, _) => {
                h892
            }
        };
        let a894 = 4u64;
        nat_eq(a893, a894)
    } {
        let a896 = match v0.clone() {
            Adt2::C0(_, _, h895, _, _, _, _, _, _, _, _) => {
                h895
            }
        };
        let a897 = 1u64;
        if nat_eq(a896, a897) {
            true
        } else {
            let a902 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h898, _, _, _, _) => {
                    h898
                }
            };
            let a903 = {
                let a900 = match v0.clone() {
                    Adt2::C0(_, h899, _, _, _, _, _, _, _, _, _) => {
                        h899
                    }
                };
                let a901 = 4u64;
                nat_mul(a900, a901)?
            };
            nat_eq(a902, a903)
        }
    } else {
        false
    } {
        let a905 = match v0.clone() {
            Adt2::C0(_, _, _, h904, _, _, _, _, _, _, _) => {
                h904
            }
        };
        let a906 = 1u64;
        if nat_eq(a905, a906) {
            true
        } else {
            let a914 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h907, _, _, _) => {
                    h907
                }
            };
            let a915 = {
                let a912 = {
                    let a910 = match v0.clone() {
                        Adt2::C0(_, h908, _, _, _, _, _, _, _, _, _) => {
                            h908
                        }
                    };
                    let a911 = match v0.clone() {
                        Adt2::C0(_, _, h909, _, _, _, _, _, _, _, _) => {
                            h909
                        }
                    };
                    nat_mul(a910, a911)?
                };
                let a913 = 4u64;
                nat_mul(a912, a913)?
            };
            nat_eq(a914, a915)
        }
    } else {
        false
    } {
        let a917 = match v0.clone() {
            Adt2::C0(_, _, _, _, h916, _, _, _, _, _, _) => {
                h916
            }
        };
        let a918 = 1u64;
        if nat_eq(a917, a918) {
            Ok(true)
        } else {
            let a929 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h919, _, _) => {
                    h919
                }
            };
            let a930 = {
                let a927 = {
                    let a925 = {
                        let a922 = match v0.clone() {
                            Adt2::C0(_, h920, _, _, _, _, _, _, _, _, _) => {
                                h920
                            }
                        };
                        let a923 = match v0.clone() {
                            Adt2::C0(_, _, h921, _, _, _, _, _, _, _, _) => {
                                h921
                            }
                        };
                        nat_mul(a922, a923)?
                    };
                    let a926 = match v0 {
                        Adt2::C0(_, _, _, h924, _, _, _, _, _, _, _) => {
                            h924
                        }
                    };
                    nat_mul(a925, a926)?
                };
                let a928 = 4u64;
                nat_mul(a927, a928)?
            };
            Ok(nat_eq(a929, a930))
        }
    } else {
        Ok(false)
    }
}

pub fn f33(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f57(v0.clone(), false)?;
    let v6: u64 = f57(v0, true)?;
    let a931 = v6;
    let a932 = v1;
    if if nat_le(a931, a932) {
        let a939 = {
            let a937 = {
                let a933 = v1;
                let a934 = v5;
                nat_add(a933, a934)?
            };
            let a938 = {
                let a935 = v3;
                let a936 = 0u64;
                if nat_eq(a935, a936) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a937, a938)?
        };
        let a940 = v2;
        nat_le(a939, a940)
    } else {
        false
    } {
        if v4 {
            let a945 = {
                let a943 = {
                    let a941 = v1;
                    let a942 = v5;
                    nat_add(a941, a942)?
                };
                let a944 = v6;
                nat_sub(a943, a944)
            };
            let a946 = v2;
            Ok(nat_lt(a945, a946))
        } else {
            let a951 = {
                let a949 = {
                    let a947 = v1;
                    let a948 = v5;
                    nat_add(a947, a948)?
                };
                let a950 = v6;
                nat_sub(a949, a950)
            };
            let a952 = v2;
            Ok(nat_le(a951, a952))
        }
    } else {
        Ok(false)
    }
}

pub fn f34(v0: u64, v1: u64) -> bool {
    let a953 = v0;
    let a954 = 0u64;
    if if nat_eq(a953, a954) {
        true
    } else {
        let a955 = v0;
        let a956 = 1u64;
        nat_eq(a955, a956)
    } {
        let a957 = v1;
        let a958 = 4294967296u64;
        nat_lt(a957, a958)
    } else {
        false
    }
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let a963 = {
        let a961 = v0;
        let a962 = {
            let a959 = v1;
            let a960 = 1u64;
            nat_sub(a959, a960)
        };
        nat_add(a961, a962)?
    };
    let a964 = v1;
    let a965 = 0u64;
    Ok(nat_quot(a963, a964, a965))
}

pub fn f36(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f29(match v0.clone() {
        Adt2::C0(_, h966, _, _, _, _, _, _, _, _, _) => {
            h966
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h967, _, _, _, _, _, _, _, _) => {
            h967
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h968, _, _, _, _, _, _, _) => {
            h968
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h969, _, _, _, _, _, _) => {
            h969
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a972 = match v1.clone() {
                Adt2::C0(_, h970, _, _, _, _, _, _, _, _, _) => {
                    h970
                }
            };
            let a973 = match v1.clone() {
                Adt2::C0(_, _, h971, _, _, _, _, _, _, _, _) => {
                    h971
                }
            };
            nat_mul(a972, a973)?
        };
        let v5: u64 = {
            let a975 = v4;
            let a976 = match v1.clone() {
                Adt2::C0(_, _, _, h974, _, _, _, _, _, _, _) => {
                    h974
                }
            };
            nat_mul(a975, a976)?
        };
        let a989 = {
            let a987 = {
                let a978 = match v0.clone() {
                    Adt2::C0(_, h977, _, _, _, _, _, _, _, _, _) => {
                        h977
                    }
                };
                let a979 = 1u64;
                nat_sub(a978, a979)
            };
            let a988 = {
                let a985 = f58(match v0.clone() {
                    Adt2::C0(_, _, h980, _, _, _, _, _, _, _, _) => {
                        h980
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h981, _, _, _, _, _, _, _, _, _) => {
                        h981
                    }
                })?;
                let a986 = f58(match v0 {
                    Adt2::C0(_, _, _, h982, _, _, _, _, _, _, _) => {
                        h982
                    }
                }, v2, {
                    let a983 = v3;
                    let a984 = v5;
                    nat_mul(a983, a984)?
                }, v4)?;
                nat_add(a985, a986)?
            };
            nat_add(a987, a988)?
        };
        let a990 = f23(v1, 1u64)?;
        Ok(nat_lt(a989, a990))
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a991 = v0;
    let a992 = 2147483648u64;
    if if if if nat_lt(a991, a992) {
        let a993 = v1;
        let a994 = 2147483648u64;
        nat_lt(a993, a994)
    } else {
        false
    } {
        let a997 = {
            let a995 = v0;
            let a996 = v1;
            nat_mul(a995, a996)?
        };
        let a998 = 2147483648u64;
        nat_lt(a997, a998)
    } else {
        false
    } {
        let a1003 = {
            let a1001 = {
                let a999 = v0;
                let a1000 = 1u64;
                nat_sub(a999, a1000)
            };
            let a1002 = v2;
            nat_mul(a1001, a1002)?
        };
        let a1004 = 2147483648u64;
        nat_lt(a1003, a1004)
    } else {
        false
    } {
        let a1009 = {
            let a1007 = {
                let a1005 = v1;
                let a1006 = 1u64;
                nat_sub(a1005, a1006)
            };
            let a1008 = v2;
            nat_mul(a1007, a1008)?
        };
        let a1010 = 2147483648u64;
        Ok(nat_lt(a1009, a1010))
    } else {
        Ok(false)
    }
}

pub fn f38(v0: u64) -> R<u64> {
    let a1013 = 3u64;
    let a1014 = {
        let a1011 = v0;
        let a1012 = 3u64;
        nat_mul(a1011, a1012)?
    };
    nat_add(a1013, a1014)
}

pub fn f39(v0: Adt3) -> R<u64> {
    if f29(match v0.clone() {
        Adt3::C0(h1015, _, _, _, _, _, _, _) => {
            h1015
        }
    }, match v0.clone() {
        Adt3::C0(_, h1016, _, _, _, _, _, _) => {
            h1016
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1017, _, _, _, _, _) => {
            h1017
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1018, _, _, _, _) => {
            h1018
        }
    }) {
        Ok(0u64)
    } else {
        let a1027 = match v0.clone() {
            Adt3::C0(h1019, _, _, _, _, _, _, _) => {
                h1019
            }
        };
        let a1028 = {
            let a1025 = match v0.clone() {
                Adt3::C0(_, h1020, _, _, _, _, _, _) => {
                    h1020
                }
            };
            let a1026 = {
                let a1023 = match v0.clone() {
                    Adt3::C0(_, _, h1021, _, _, _, _, _) => {
                        h1021
                    }
                };
                let a1024 = match v0 {
                    Adt3::C0(_, _, _, h1022, _, _, _, _) => {
                        h1022
                    }
                };
                nat_mul(a1023, a1024)?
            };
            nat_mul(a1025, a1026)?
        };
        nat_mul(a1027, a1028)
    }
}

pub fn f40(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1029 = v1;
    let a1030 = 13u64;
    if if nat_eq(a1029, a1030) {
        f29(match v0.clone() {
            Adt3::C0(h1031, _, _, _, _, _, _, _) => {
                h1031
            }
        }, match v0.clone() {
            Adt3::C0(_, h1032, _, _, _, _, _, _) => {
                h1032
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1033, _, _, _, _, _) => {
                h1033
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1034, _, _, _, _) => {
                h1034
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1040 = match v0.clone() {
                Adt3::C0(_, h1035, _, _, _, _, _, _) => {
                    h1035
                }
            };
            let a1041 = {
                let a1038 = match v0.clone() {
                    Adt3::C0(_, _, h1036, _, _, _, _, _) => {
                        h1036
                    }
                };
                let a1039 = match v0.clone() {
                    Adt3::C0(_, _, _, h1037, _, _, _, _) => {
                        h1037
                    }
                };
                nat_mul(a1038, a1039)?
            };
            nat_mul(a1040, a1041)?
        };
        let a1042 = v1;
        let a1043 = 13u64;
        if nat_eq(a1042, a1043) {
            let a1052 = 2u64;
            let a1053 = if f59(v0.clone(), v2) {
                let a1045 = v3;
                let a1046 = f60(match v0 {
                    Adt3::C0(h1044, _, _, _, _, _, _, _) => {
                        h1044
                    }
                }, false)?;
                nat_mul(a1045, a1046)?
            } else {
                let a1050 = v3;
                let a1051 = {
                    let a1048 = match v0 {
                        Adt3::C0(h1047, _, _, _, _, _, _, _) => {
                            h1047
                        }
                    };
                    let a1049 = 2u64;
                    nat_mul(a1048, a1049)?
                };
                nat_mul(a1050, a1051)?
            };
            nat_add(a1052, a1053)
        } else {
            let a1057 = v3;
            let a1058 = f60(match v0 {
                Adt3::C0(h1054, _, _, _, _, _, _, _) => {
                    h1054
                }
            }, {
                let a1055 = v1;
                let a1056 = 16u64;
                nat_eq(a1055, a1056)
            })?;
            nat_mul(a1057, a1058)
        }
    }
}

pub fn f41(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1059 = v3;
    match m1059 {
        None => {
            f61(v0, v1, match v2.clone() {
                Adt2::C0(h1060, _, _, _, _, _, _, _, _, _, _) => {
                    h1060
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1061, _, _, _, _, _, _, _, _, _, _) => {
                    h1061
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1062, _, _, _, _, _, _, _, _, _) => {
                    h1062
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1063, _, _, _, _, _, _, _, _) => {
                    h1063
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1064, _, _, _, _, _, _, _) => {
                    h1064
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1065, _, _, _, _, _, _) => {
                    h1065
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1066, _, _, _, _, _, _, _, _, _) => {
                    h1066
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1067, _, _, _, _, _, _, _, _) => {
                    h1067
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1068, _, _, _, _, _, _, _) => {
                    h1068
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1069, _, _, _, _, _, _) => {
                    h1069
                }
            })
        }
        Some(v5) => {
            f61(v0, v1, match v2.clone() {
                Adt2::C0(h1070, _, _, _, _, _, _, _, _, _, _) => {
                    h1070
                }
            }, match v5.clone() {
                Adt2::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                    h1071
                }
            }, match v4.clone() {
                Adt2::C0(h1072, _, _, _, _, _, _, _, _, _, _) => {
                    h1072
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1073, _, _, _, _, _, _, _, _, _) => {
                    h1073
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1074, _, _, _, _, _, _, _, _) => {
                    h1074
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1075, _, _, _, _, _, _, _) => {
                    h1075
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1076, _, _, _, _, _, _) => {
                    h1076
                }
            }, match v5.clone() {
                Adt2::C0(_, h1077, _, _, _, _, _, _, _, _, _) => {
                    h1077
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1078, _, _, _, _, _, _, _, _) => {
                    h1078
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1079, _, _, _, _, _, _, _) => {
                    h1079
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1080, _, _, _, _, _, _) => {
                    h1080
                }
            }, match v4.clone() {
                Adt2::C0(_, h1081, _, _, _, _, _, _, _, _, _) => {
                    h1081
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1082, _, _, _, _, _, _, _, _) => {
                    h1082
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1083, _, _, _, _, _, _, _) => {
                    h1083
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1084, _, _, _, _, _, _) => {
                    h1084
                }
            })
        }
    }
}

pub fn f42(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1086 = v0;
        let a1087 = match v1.clone() {
            Adt3::C0(h1085, _, _, _, _, _, _, _) => {
                h1085
            }
        };
        let a1088 = 0u64;
        nat_quot(a1086, a1087, a1088)
    };
    let v3: u64 = {
        let a1090 = v2;
        let a1091 = match v1.clone() {
            Adt3::C0(_, h1089, _, _, _, _, _, _) => {
                h1089
            }
        };
        let a1092 = 0u64;
        nat_quot(a1090, a1091, a1092)
    };
    f62({
        let a1094 = v0;
        let a1095 = match v1.clone() {
            Adt3::C0(h1093, _, _, _, _, _, _, _) => {
                h1093
            }
        };
        let a1096 = 0u64;
        nat_rem(a1094, a1095, a1096)
    }, {
        let a1098 = v2;
        let a1099 = match v1.clone() {
            Adt3::C0(_, h1097, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1100 = 0u64;
        nat_rem(a1098, a1099, a1100)
    }, {
        let a1102 = v3;
        let a1103 = match v1.clone() {
            Adt3::C0(_, _, h1101, _, _, _, _, _) => {
                h1101
            }
        };
        let a1104 = 0u64;
        nat_rem(a1102, a1103, a1104)
    }, {
        let a1106 = v3;
        let a1107 = match v1.clone() {
            Adt3::C0(_, _, h1105, _, _, _, _, _) => {
                h1105
            }
        };
        let a1108 = 0u64;
        nat_quot(a1106, a1107, a1108)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1109, _, _, _) => {
            h1109
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1110, _, _) => {
            h1110
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1111, _) => {
            h1111
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1112) => {
            h1112
        }
    })
}

pub fn f43(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1114 = v0;
        let a1115 = match v2.clone() {
            Adt3::C0(h1113, _, _, _, _, _, _, _) => {
                h1113
            }
        };
        let a1116 = 0u64;
        nat_quot(a1114, a1115, a1116)
    };
    let v4: u64 = {
        let a1118 = v3;
        let a1119 = match v2.clone() {
            Adt3::C0(_, h1117, _, _, _, _, _, _) => {
                h1117
            }
        };
        let a1120 = 0u64;
        nat_quot(a1118, a1119, a1120)
    };
    f63({
        let a1122 = v0;
        let a1123 = match v2.clone() {
            Adt3::C0(h1121, _, _, _, _, _, _, _) => {
                h1121
            }
        };
        let a1124 = 0u64;
        nat_rem(a1122, a1123, a1124)
    }, {
        let a1126 = v3;
        let a1127 = match v2.clone() {
            Adt3::C0(_, h1125, _, _, _, _, _, _) => {
                h1125
            }
        };
        let a1128 = 0u64;
        nat_rem(a1126, a1127, a1128)
    }, {
        let a1130 = v4;
        let a1131 = match v2.clone() {
            Adt3::C0(_, _, h1129, _, _, _, _, _) => {
                h1129
            }
        };
        let a1132 = 0u64;
        nat_rem(a1130, a1131, a1132)
    }, {
        let a1134 = v4;
        let a1135 = match v2 {
            Adt3::C0(_, _, h1133, _, _, _, _, _) => {
                h1133
            }
        };
        let a1136 = 0u64;
        nat_quot(a1134, a1135, a1136)
    }, match v1.clone() {
        Adt3::C0(h1137, _, _, _, _, _, _, _) => {
            h1137
        }
    }, match v1.clone() {
        Adt3::C0(_, h1138, _, _, _, _, _, _) => {
            h1138
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1139, _, _, _, _, _) => {
            h1139
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1140, _, _, _, _) => {
            h1140
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1141, _, _, _) => {
            h1141
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1142, _, _) => {
            h1142
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1143, _) => {
            h1143
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1144) => {
            h1144
        }
    })
}

pub fn f44(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1146 = v0;
        let a1147 = match v3.clone() {
            Adt3::C0(h1145, _, _, _, _, _, _, _) => {
                h1145
            }
        };
        let a1148 = 0u64;
        nat_quot(a1146, a1147, a1148)
    };
    let v6: u64 = {
        let a1150 = v5;
        let a1151 = match v3.clone() {
            Adt3::C0(_, h1149, _, _, _, _, _, _) => {
                h1149
            }
        };
        let a1152 = 0u64;
        nat_quot(a1150, a1151, a1152)
    };
    let v7: u64 = {
        let a1154 = v0;
        let a1155 = match v3.clone() {
            Adt3::C0(h1153, _, _, _, _, _, _, _) => {
                h1153
            }
        };
        let a1156 = 0u64;
        nat_rem(a1154, a1155, a1156)
    };
    let v8: u64 = {
        let a1158 = v5;
        let a1159 = match v3.clone() {
            Adt3::C0(_, h1157, _, _, _, _, _, _) => {
                h1157
            }
        };
        let a1160 = 0u64;
        nat_rem(a1158, a1159, a1160)
    };
    let v9: u64 = {
        let a1162 = v6;
        let a1163 = match v3.clone() {
            Adt3::C0(_, _, h1161, _, _, _, _, _) => {
                h1161
            }
        };
        let a1164 = 0u64;
        nat_rem(a1162, a1163, a1164)
    };
    let v10: u64 = {
        let a1166 = v6;
        let a1167 = match v3.clone() {
            Adt3::C0(_, _, h1165, _, _, _, _, _) => {
                h1165
            }
        };
        let a1168 = 0u64;
        nat_quot(a1166, a1167, a1168)
    };
    let v11: u64 = {
        let a1169 = v4;
        let a1170 = 0u64;
        if nat_eq(a1169, a1170) {
            match v1.clone() {
                Adt3::C0(h1171, _, _, _, _, _, _, _) => {
                    h1171
                }
            }
        } else {
            let a1172 = v4;
            let a1173 = 1u64;
            if nat_eq(a1172, a1173) {
                match v1.clone() {
                    Adt3::C0(_, h1174, _, _, _, _, _, _) => {
                        h1174
                    }
                }
            } else {
                let a1175 = v4;
                let a1176 = 2u64;
                if nat_eq(a1175, a1176) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1177, _, _, _, _, _) => {
                            h1177
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1178, _, _, _, _) => {
                            h1178
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1185 = v11;
        let a1186 = {
            let a1179 = v4;
            let a1180 = 0u64;
            if nat_eq(a1179, a1180) {
                v7
            } else {
                let a1181 = v4;
                let a1182 = 1u64;
                if nat_eq(a1181, a1182) {
                    v8
                } else {
                    let a1183 = v4;
                    let a1184 = 2u64;
                    if nat_eq(a1183, a1184) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1185, a1186)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f62(if if v12 {
        let a1187 = v4;
        let a1188 = 0u64;
        nat_eq(a1187, a1188)
    } else {
        false
    } {
        let a1189 = v7;
        let a1190 = v11;
        nat_sub(a1189, a1190)
    } else {
        v7
    }, if if v12 {
        let a1191 = v4;
        let a1192 = 1u64;
        nat_eq(a1191, a1192)
    } else {
        false
    } {
        let a1193 = v8;
        let a1194 = v11;
        nat_sub(a1193, a1194)
    } else {
        v8
    }, if if v12 {
        let a1195 = v4;
        let a1196 = 2u64;
        nat_eq(a1195, a1196)
    } else {
        false
    } {
        let a1197 = v9;
        let a1198 = v11;
        nat_sub(a1197, a1198)
    } else {
        v9
    }, if if v12 {
        let a1199 = v4;
        let a1200 = 3u64;
        nat_eq(a1199, a1200)
    } else {
        false
    } {
        let a1201 = v10;
        let a1202 = v11;
        nat_sub(a1201, a1202)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1203, _, _, _) => {
            h1203
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1204, _, _) => {
            h1204
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1205, _) => {
            h1205
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1206) => {
            h1206
        }
    })?, f42(v0, v3)?))
}

pub fn f45(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f64(v0, v2.clone(), 0u64);
    let v9: u64 = f64(v0, v2.clone(), 1u64);
    let v10: u64 = f64(v0, v2.clone(), 2u64);
    let v11: u64 = f64(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f65(v1.clone(), f66(v8, v3, match v1.clone() {
            Adt3::C0(h1207, _, _, _, _, _, _, _) => {
                h1207
            }
        })?, f66(v9, v4, match v1.clone() {
            Adt3::C0(_, h1208, _, _, _, _, _, _) => {
                h1208
            }
        })?, f66(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1209, _, _, _, _, _) => {
                h1209
            }
        })?, f66(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1210, _, _, _, _) => {
                h1210
            }
        })?)?, f42(v0, v2)?))
    } else {
        if if if if f67(v8, v3, match v1.clone() {
            Adt3::C0(h1211, _, _, _, _, _, _, _) => {
                h1211
            }
        })? {
            f67(v9, v4, match v1.clone() {
                Adt3::C0(_, h1212, _, _, _, _, _, _) => {
                    h1212
                }
            })?
        } else {
            false
        } {
            f67(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1213, _, _, _, _, _) => {
                    h1213
                }
            })?
        } else {
            false
        } {
            f67(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1214, _, _, _, _) => {
                    h1214
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f65(v1, f68(v8, v3)?, f68(v9, v4)?, f68(v10, v5)?, f68(v11, v6)?)?, f42(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f42(v0, v2)?))
        }
    }
}

pub fn f46(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f64(v0, v2.clone(), 0u64);
    let v5: u64 = f64(v0, v2.clone(), 1u64);
    let v6: u64 = f64(v0, v2.clone(), 2u64);
    let v7: u64 = f64(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f65(v1.clone(), f69(v4, v3, match v1 {
        Adt3::C0(h1215, _, _, _, _, _, _, _) => {
            h1215
        }
    })?, v5, v6, v7)?, f42(v0, v2)?))
}

pub fn f47(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1216 = v0;
    let a1217 = v3;
    if nat_lt(a1216, a1217) {
        Ok(Adt5::C1(false, {
            let a1218 = v0;
            let a1219 = 4u64;
            nat_mul(a1218, a1219)?
        }, {
            let a1220 = v0;
            let a1221 = 4u64;
            nat_mul(a1220, a1221)?
        }))
    } else {
        let v6: u64 = {
            let a1222 = v0;
            let a1223 = v3;
            nat_sub(a1222, a1223)
        };
        let v7: u64 = f42(v6, v1)?;
        let v8: u64 = f70(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f48(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1224 = v0;
    let a1225 = v3;
    if nat_lt(a1224, a1225) {
        Ok(Adt5::C1(true, 0u64, f42(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1226 = v0;
            let a1227 = v3;
            nat_sub(a1226, a1227)
        };
        let v5: u64 = f43(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f42(v4, v1)?, v5))
    }
}

pub fn f49(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1229 = v1;
        let a1230 = match v3.clone() {
            Adt3::C0(h1228, _, _, _, _, _, _, _) => {
                h1228
            }
        };
        let a1231 = 0u64;
        nat_rem(a1229, a1230, a1231)
    };
    let v5: u64 = {
        let a1237 = {
            let a1233 = v1;
            let a1234 = match v3.clone() {
                Adt3::C0(h1232, _, _, _, _, _, _, _) => {
                    h1232
                }
            };
            let a1235 = 0u64;
            nat_quot(a1233, a1234, a1235)
        };
        let a1238 = match v3.clone() {
            Adt3::C0(_, h1236, _, _, _, _, _, _) => {
                h1236
            }
        };
        let a1239 = 0u64;
        nat_rem(a1237, a1238, a1239)
    };
    let v6: bool = f71(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f43(v1, v2, v3.clone())?
    } else {
        0u64
    }, f42(v1, v3)?))
}

pub fn f50(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f42(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f43(v0, v5, v2.clone())?
    }, f42(v0, v2)?))
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1241 = v0;
        let a1242 = match v2.clone() {
            Adt3::C0(h1240, _, _, _, _, _, _, _) => {
                h1240
            }
        };
        let a1243 = 0u64;
        nat_rem(a1241, a1242, a1243)
    };
    let v7: u64 = {
        let a1245 = v0;
        let a1246 = match v2.clone() {
            Adt3::C0(h1244, _, _, _, _, _, _, _) => {
                h1244
            }
        };
        let a1247 = 0u64;
        nat_quot(a1245, a1246, a1247)
    };
    let v8: u64 = {
        let a1249 = v7;
        let a1250 = match v2.clone() {
            Adt3::C0(_, h1248, _, _, _, _, _, _) => {
                h1248
            }
        };
        let a1251 = 0u64;
        nat_rem(a1249, a1250, a1251)
    };
    let v9: u64 = {
        let a1253 = v7;
        let a1254 = match v2.clone() {
            Adt3::C0(_, h1252, _, _, _, _, _, _) => {
                h1252
            }
        };
        let a1255 = 0u64;
        nat_quot(a1253, a1254, a1255)
    };
    let v10: u64 = {
        let a1257 = v9;
        let a1258 = match v2.clone() {
            Adt3::C0(_, _, h1256, _, _, _, _, _) => {
                h1256
            }
        };
        let a1259 = 0u64;
        nat_rem(a1257, a1258, a1259)
    };
    let v11: u64 = {
        let a1261 = v9;
        let a1262 = match v2.clone() {
            Adt3::C0(_, _, h1260, _, _, _, _, _) => {
                h1260
            }
        };
        let a1263 = 0u64;
        nat_quot(a1261, a1262, a1263)
    };
    let v12: u64 = f42(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1272 = {
                let a1267 = {
                    let a1264 = v10;
                    let a1265 = v4;
                    let a1266 = 0u64;
                    nat_quot(a1264, a1265, a1266)
                };
                let a1268 = v5;
                nat_mul(a1267, a1268)?
            };
            let a1273 = {
                let a1269 = v8;
                let a1270 = v4;
                let a1271 = 0u64;
                nat_quot(a1269, a1270, a1271)
            };
            nat_add(a1272, a1273)?
        };
        let v14: u64 = {
            let a1293 = v6;
            let a1294 = {
                let a1291 = match v1.clone() {
                    Adt3::C0(h1274, _, _, _, _, _, _, _) => {
                        h1274
                    }
                };
                let a1292 = {
                    let a1289 = {
                        let a1275 = v8;
                        let a1276 = v4;
                        let a1277 = 0u64;
                        nat_rem(a1275, a1276, a1277)
                    };
                    let a1290 = {
                        let a1287 = match v1.clone() {
                            Adt3::C0(_, h1278, _, _, _, _, _, _) => {
                                h1278
                            }
                        };
                        let a1288 = {
                            let a1285 = {
                                let a1279 = v10;
                                let a1280 = v4;
                                let a1281 = 0u64;
                                nat_rem(a1279, a1280, a1281)
                            };
                            let a1286 = {
                                let a1283 = match v1.clone() {
                                    Adt3::C0(_, _, h1282, _, _, _, _, _) => {
                                        h1282
                                    }
                                };
                                let a1284 = v13;
                                nat_mul(a1283, a1284)?
                            };
                            nat_add(a1285, a1286)?
                        };
                        nat_mul(a1287, a1288)?
                    };
                    nat_add(a1289, a1290)?
                };
                nat_mul(a1291, a1292)?
            };
            nat_add(a1293, a1294)?
        };
        Ok(Adt5::C1(false, f42(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1300 = {
                let a1298 = {
                    let a1295 = v11;
                    let a1296 = v5;
                    let a1297 = 0u64;
                    nat_rem(a1295, a1296, a1297)
                };
                let a1299 = v4;
                nat_mul(a1298, a1299)?
            };
            let a1301 = v8;
            nat_add(a1300, a1301)?
        };
        let v16: u64 = {
            let a1307 = {
                let a1305 = {
                    let a1302 = v11;
                    let a1303 = v5;
                    let a1304 = 0u64;
                    nat_quot(a1302, a1303, a1304)
                };
                let a1306 = v4;
                nat_mul(a1305, a1306)?
            };
            let a1308 = v10;
            nat_add(a1307, a1308)?
        };
        let a1310 = v15;
        let a1311 = match v1.clone() {
            Adt3::C0(_, h1309, _, _, _, _, _, _) => {
                h1309
            }
        };
        if if nat_lt(a1310, a1311) {
            let a1313 = v16;
            let a1314 = match v1.clone() {
                Adt3::C0(_, _, h1312, _, _, _, _, _) => {
                    h1312
                }
            };
            nat_lt(a1313, a1314)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f42({
                let a1323 = v6;
                let a1324 = {
                    let a1321 = match v1.clone() {
                        Adt3::C0(h1315, _, _, _, _, _, _, _) => {
                            h1315
                        }
                    };
                    let a1322 = {
                        let a1319 = v15;
                        let a1320 = {
                            let a1317 = match v1.clone() {
                                Adt3::C0(_, h1316, _, _, _, _, _, _) => {
                                    h1316
                                }
                            };
                            let a1318 = v16;
                            nat_mul(a1317, a1318)?
                        };
                        nat_add(a1319, a1320)?
                    };
                    nat_mul(a1321, a1322)?
                };
                nat_add(a1323, a1324)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f42(v0, v2.clone())?;
    let v4: u64 = f42(v0, v1.clone())?;
    let a1329 = {
        let a1326 = v0;
        let a1327 = match v1 {
            Adt3::C0(h1325, _, _, _, _, _, _, _) => {
                h1325
            }
        };
        let a1328 = 0u64;
        nat_rem(a1326, a1327, a1328)
    };
    let a1330 = 0u64;
    if nat_eq(a1329, a1330) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C2(0u64, 2u64, f42({
            let a1331 = v0;
            let a1332 = 1u64;
            nat_sub(a1331, a1332)
        }, v2)?, 0u64, v4, v3))
    }
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1333 = v3;
    let a1334 = 13u64;
    if nat_eq(a1333, a1334) {
        f72(v0, v1, v4)
    } else {
        let v5: u64 = f60(match v1.clone() {
            Adt3::C0(h1335, _, _, _, _, _, _, _) => {
                h1335
            }
        }, {
            let a1336 = v3;
            let a1337 = 16u64;
            nat_eq(a1336, a1337)
        })?;
        let v6: u64 = {
            let a1338 = v0;
            let a1339 = v5;
            let a1340 = 0u64;
            nat_quot(a1338, a1339, a1340)
        };
        let v7: u64 = {
            let a1341 = v0;
            let a1342 = v5;
            let a1343 = 0u64;
            nat_rem(a1341, a1342, a1343)
        };
        let v8: u64 = {
            let a1347 = 1u64;
            let a1348 = {
                let a1345 = match v1.clone() {
                    Adt3::C0(h1344, _, _, _, _, _, _, _) => {
                        h1344
                    }
                };
                let a1346 = 2u64;
                nat_mul(a1345, a1346)?
            };
            nat_add(a1347, a1348)?
        };
        let a1349 = v7;
        let a1350 = v8;
        if nat_lt(a1349, a1350) {
            f73(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f42(v6, v2)?;
            let a1351 = v3;
            let a1352 = 14u64;
            if nat_eq(a1351, a1352) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1353 = v7;
                let a1354 = v8;
                if nat_eq(a1353, a1354) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1357 = v7;
                    let a1358 = {
                        let a1355 = v8;
                        let a1356 = 1u64;
                        nat_add(a1355, a1356)?
                    };
                    if nat_eq(a1357, a1358) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1359, _, _, _, _, _, _, _) => {
                                h1359
                            }
                        }, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(5u64, 1u64, 1u64, 1u64, 2u64, 3u64, v9))
                    }
                }
            }
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1360 = v0;
    let a1361 = 0u64;
    if nat_eq(a1360, a1361) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1368 = {
            let a1362 = v0;
            let a1363 = 1u64;
            nat_add(a1362, a1363)?
        };
        let a1369 = {
            let a1366 = 2u64;
            let a1367 = {
                let a1364 = v3;
                let a1365 = 2u64;
                nat_mul(a1364, a1365)?
            };
            nat_add(a1366, a1367)?
        };
        if nat_eq(a1368, a1369) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1370 = v0;
                let a1371 = 1u64;
                nat_sub(a1370, a1371)
            };
            let v5: u64 = {
                let a1372 = v4;
                let a1373 = 2u64;
                let a1374 = 0u64;
                nat_quot(a1372, a1373, a1374)
            };
            let a1378 = {
                let a1375 = v4;
                let a1376 = 2u64;
                let a1377 = 0u64;
                nat_rem(a1375, a1376, a1377)
            };
            let a1379 = 0u64;
            if nat_eq(a1378, a1379) {
                Ok(Adt5::C4(17u64, 0u64, f42(v5, v1)?, 4u64, f42(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f55(v0: u64) -> u64 {
    let a1380 = v0;
    let a1381 = 2147483648u64;
    if nat_lt(a1380, a1381) {
        v0
    } else {
        0u64
    }
}

pub fn f56(v0: u64) -> u64 {
    let a1382 = v0;
    let a1383 = 2147483648u64;
    if nat_lt(a1382, a1383) {
        0u64
    } else {
        let a1384 = 4294967296u64;
        let a1385 = v0;
        nat_sub(a1384, a1385)
    }
}

pub fn f57(v0: Adt3, v1: bool) -> R<u64> {
    f74(v0.clone(), {
        let a1387 = match v0.clone() {
            Adt3::C0(h1386, _, _, _, _, _, _, _) => {
                h1386
            }
        };
        let a1388 = 1u64;
        nat_sub(a1387, a1388)
    }, {
        let a1390 = match v0.clone() {
            Adt3::C0(_, h1389, _, _, _, _, _, _) => {
                h1389
            }
        };
        let a1391 = 1u64;
        nat_sub(a1390, a1391)
    }, {
        let a1393 = match v0.clone() {
            Adt3::C0(_, _, h1392, _, _, _, _, _) => {
                h1392
            }
        };
        let a1394 = 1u64;
        nat_sub(a1393, a1394)
    }, {
        let a1396 = match v0 {
            Adt3::C0(_, _, _, h1395, _, _, _, _) => {
                h1395
            }
        };
        let a1397 = 1u64;
        nat_sub(a1396, a1397)
    }, v1)
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1398 = v0;
    let a1399 = 0u64;
    if nat_eq(a1398, a1399) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1400 = v0;
            let a1401 = 1u64;
            nat_sub(a1400, a1401)
        };
        let v5: u64 = {
            let a1402 = v4;
            let a1403 = v1;
            let a1404 = 0u64;
            nat_quot(a1402, a1403, a1404)
        };
        let v6: u64 = {
            let a1412 = {
                let a1405 = v5;
                let a1406 = v2;
                nat_mul(a1405, a1406)?
            };
            let a1413 = {
                let a1410 = {
                    let a1407 = v4;
                    let a1408 = v1;
                    let a1409 = 0u64;
                    nat_rem(a1407, a1408, a1409)
                };
                let a1411 = v3;
                nat_mul(a1410, a1411)?
            };
            nat_add(a1412, a1413)?
        };
        let a1414 = v5;
        let a1415 = 0u64;
        if nat_eq(a1414, a1415) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1424 = {
                    let a1418 = {
                        let a1416 = v5;
                        let a1417 = 1u64;
                        nat_sub(a1416, a1417)
                    };
                    let a1419 = v2;
                    nat_mul(a1418, a1419)?
                };
                let a1425 = {
                    let a1422 = {
                        let a1420 = v1;
                        let a1421 = 1u64;
                        nat_sub(a1420, a1421)
                    };
                    let a1423 = v3;
                    nat_mul(a1422, a1423)?
                };
                nat_add(a1424, a1425)?
            };
            let a1426 = v6;
            let a1427 = v7;
            if nat_lt(a1426, a1427) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f59(v0: Adt3, v1: u64) -> bool {
    let a1428 = v1;
    let a1429 = 0u64;
    if !nat_eq(a1428, a1429) {
        true
    } else {
        let a1431 = match v0 {
            Adt3::C0(_, _, _, _, h1430, _, _, _) => {
                h1430
            }
        };
        let a1432 = 4u64;
        nat_eq(a1431, a1432)
    }
}

pub fn f60(v0: u64, v1: bool) -> R<u64> {
    let a1435 = {
        let a1433 = v0;
        let a1434 = 2u64;
        nat_mul(a1433, a1434)?
    };
    let a1436 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a1435, a1436)
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1437 = f75(v0, v1);
    match m1437 {
        None => {
            if v5 {
                let m1438 = f76(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1438 {
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
                let a1439 = 4u64;
                let a1440 = v28;
                nat_le(a1439, a1440)
            };
            let a1441 = v2;
            let a1442 = 0u64;
            if if if if if nat_eq(a1441, a1442) {
                let a1443 = v4;
                let a1444 = 0u64;
                nat_eq(a1443, a1444)
            } else {
                false
            } {
                let a1445 = v6;
                let a1446 = v14;
                if nat_eq(a1445, a1446) {
                    let a1447 = v7;
                    let a1448 = v15;
                    nat_eq(a1447, a1448)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1449 = v8;
                let a1450 = v16;
                if nat_eq(a1449, a1450) {
                    let a1451 = v9;
                    let a1452 = v17;
                    nat_eq(a1451, a1452)
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
                        let a1453 = v3;
                        let a1454 = 0u64;
                        nat_eq(a1453, a1454)
                    } else {
                        false
                    } {
                        f26(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1459 = {
                        let a1455 = v6;
                        let a1456 = v7;
                        nat_mul(a1455, a1456)?
                    };
                    let a1460 = {
                        let a1457 = v8;
                        let a1458 = v9;
                        nat_mul(a1457, a1458)?
                    };
                    nat_mul(a1459, a1460)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1473 = {
        let a1469 = {
            let a1465 = {
                let a1461 = v0;
                let a1462 = v4;
                nat_mul(a1461, a1462)?
            };
            let a1466 = {
                let a1463 = v1;
                let a1464 = v5;
                nat_mul(a1463, a1464)?
            };
            nat_add(a1465, a1466)?
        };
        let a1470 = {
            let a1467 = v2;
            let a1468 = v6;
            nat_mul(a1467, a1468)?
        };
        nat_add(a1469, a1470)?
    };
    let a1474 = {
        let a1471 = v3;
        let a1472 = v7;
        nat_mul(a1471, a1472)?
    };
    nat_add(a1473, a1474)
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f62({
        let a1475 = v0;
        let a1476 = v4;
        let a1477 = 0u64;
        nat_rem(a1475, a1476, a1477)
    }, {
        let a1478 = v1;
        let a1479 = v5;
        let a1480 = 0u64;
        nat_rem(a1478, a1479, a1480)
    }, {
        let a1481 = v2;
        let a1482 = v6;
        let a1483 = 0u64;
        nat_rem(a1481, a1482, a1483)
    }, {
        let a1484 = v3;
        let a1485 = v7;
        let a1486 = 0u64;
        nat_rem(a1484, a1485, a1486)
    }, v8, v9, v10, v11)
}

pub fn f64(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1488 = v0;
        let a1489 = match v1.clone() {
            Adt3::C0(h1487, _, _, _, _, _, _, _) => {
                h1487
            }
        };
        let a1490 = 0u64;
        nat_quot(a1488, a1489, a1490)
    };
    let v4: u64 = {
        let a1492 = v3;
        let a1493 = match v1.clone() {
            Adt3::C0(_, h1491, _, _, _, _, _, _) => {
                h1491
            }
        };
        let a1494 = 0u64;
        nat_quot(a1492, a1493, a1494)
    };
    let a1495 = v2;
    let a1496 = 0u64;
    if nat_eq(a1495, a1496) {
        let a1498 = v0;
        let a1499 = match v1 {
            Adt3::C0(h1497, _, _, _, _, _, _, _) => {
                h1497
            }
        };
        let a1500 = 0u64;
        nat_rem(a1498, a1499, a1500)
    } else {
        let a1501 = v2;
        let a1502 = 1u64;
        if nat_eq(a1501, a1502) {
            let a1504 = v3;
            let a1505 = match v1 {
                Adt3::C0(_, h1503, _, _, _, _, _, _) => {
                    h1503
                }
            };
            let a1506 = 0u64;
            nat_rem(a1504, a1505, a1506)
        } else {
            let a1507 = v2;
            let a1508 = 2u64;
            if nat_eq(a1507, a1508) {
                let a1510 = v4;
                let a1511 = match v1 {
                    Adt3::C0(_, _, h1509, _, _, _, _, _) => {
                        h1509
                    }
                };
                let a1512 = 0u64;
                nat_rem(a1510, a1511, a1512)
            } else {
                let a1514 = v4;
                let a1515 = match v1 {
                    Adt3::C0(_, _, h1513, _, _, _, _, _) => {
                        h1513
                    }
                };
                let a1516 = 0u64;
                nat_quot(a1514, a1515, a1516)
            }
        }
    }
}

pub fn f65(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f62(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1517, _, _, _) => {
            h1517
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1518, _, _) => {
            h1518
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1519, _) => {
            h1519
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1520) => {
            h1520
        }
    })
}

pub fn f66(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1521 = v1;
    let a1522 = 2147483648u64;
    if nat_lt(a1521, a1522) {
        let a1533 = {
            let a1531 = {
                let a1526 = {
                    let a1523 = v0;
                    let a1524 = v2;
                    let a1525 = 0u64;
                    nat_rem(a1523, a1524, a1525)
                };
                let a1527 = v2;
                nat_add(a1526, a1527)?
            };
            let a1532 = {
                let a1528 = v1;
                let a1529 = v2;
                let a1530 = 0u64;
                nat_rem(a1528, a1529, a1530)
            };
            nat_sub(a1531, a1532)
        };
        let a1534 = v2;
        let a1535 = 0u64;
        Ok(nat_rem(a1533, a1534, a1535))
    } else {
        let a1538 = {
            let a1536 = v0;
            let a1537 = f56(v1);
            nat_add(a1536, a1537)?
        };
        let a1539 = v2;
        let a1540 = 0u64;
        Ok(nat_rem(a1538, a1539, a1540))
    }
}

pub fn f67(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1543 = f55(v1);
    let a1544 = {
        let a1541 = v0;
        let a1542 = f56(v1);
        nat_add(a1541, a1542)?
    };
    if nat_le(a1543, a1544) {
        let a1545 = f68(v0, v1)?;
        let a1546 = v2;
        Ok(nat_lt(a1545, a1546))
    } else {
        Ok(false)
    }
}

pub fn f68(v0: u64, v1: u64) -> R<u64> {
    let a1549 = {
        let a1547 = v0;
        let a1548 = f56(v1);
        nat_add(a1547, a1548)?
    };
    let a1550 = f55(v1);
    Ok(nat_sub(a1549, a1550))
}

pub fn f69(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1551 = v0;
    let a1552 = v1;
    if nat_lt(a1551, a1552) {
        let a1553 = v1;
        let a1554 = v0;
        Ok(nat_sub(a1553, a1554))
    } else {
        let a1557 = v0;
        let a1558 = {
            let a1555 = v1;
            let a1556 = v2;
            nat_add(a1555, a1556)?
        };
        if nat_lt(a1557, a1558) {
            let a1559 = v0;
            let a1560 = v1;
            Ok(nat_sub(a1559, a1560))
        } else {
            let a1567 = {
                let a1563 = 2u64;
                let a1564 = {
                    let a1561 = v2;
                    let a1562 = 1u64;
                    nat_sub(a1561, a1562)
                };
                nat_mul(a1563, a1564)?
            };
            let a1568 = {
                let a1565 = v0;
                let a1566 = v1;
                nat_sub(a1565, a1566)
            };
            Ok(nat_sub(a1567, a1568))
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1571 = {
        let a1569 = v2;
        let a1570 = f74(v1.clone(), f64(v0, v1.clone(), 0u64), f64(v0, v1.clone(), 1u64), f64(v0, v1.clone(), 2u64), f64(v0, v1.clone(), 3u64), false)?;
        nat_add(a1569, a1570)?
    };
    let a1572 = f74(v1.clone(), f64(v0, v1.clone(), 0u64), f64(v0, v1.clone(), 1u64), f64(v0, v1.clone(), 2u64), f64(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1571, a1572))
}

pub fn f71(v0: u64, v1: u64, v2: u64) -> bool {
    let a1573 = v0;
    let a1574 = 0u64;
    if nat_eq(a1573, a1574) {
        let a1575 = v2;
        let a1576 = v1;
        nat_le(a1575, a1576)
    } else {
        let a1577 = v0;
        let a1578 = 1u64;
        if nat_eq(a1577, a1578) {
            let a1579 = v2;
            let a1580 = v1;
            nat_lt(a1579, a1580)
        } else {
            let a1581 = v0;
            let a1582 = 2u64;
            if nat_eq(a1581, a1582) {
                let a1583 = v1;
                let a1584 = v2;
                nat_le(a1583, a1584)
            } else {
                let a1585 = v0;
                let a1586 = 3u64;
                if nat_eq(a1585, a1586) {
                    let a1587 = v1;
                    let a1588 = v2;
                    nat_lt(a1587, a1588)
                } else {
                    let a1589 = v1;
                    let a1590 = v2;
                    nat_eq(a1589, a1590)
                }
            }
        }
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a1591 = v0;
    let a1592 = 0u64;
    if nat_eq(a1591, a1592) {
        Ok(f77(0u64))
    } else {
        let a1595 = {
            let a1593 = v0;
            let a1594 = 1u64;
            nat_add(a1593, a1594)?
        };
        let a1596 = f40(v1.clone(), 13u64, v2)?;
        if nat_eq(a1595, a1596) {
            Ok(Adt5::C4({
                let a1597 = v2;
                let a1598 = 0u64;
                if nat_eq(a1597, a1598) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a1599 = v0;
                let a1600 = 1u64;
                nat_sub(a1599, a1600)
            };
            if f59(v1.clone(), v2) {
                let v4: u64 = f60(match v1.clone() {
                    Adt3::C0(h1601, _, _, _, _, _, _, _) => {
                        h1601
                    }
                }, false)?;
                let v5: u64 = {
                    let a1602 = v3;
                    let a1603 = v4;
                    let a1604 = 0u64;
                    nat_quot(a1602, a1603, a1604)
                };
                let v6: u64 = {
                    let a1605 = v3;
                    let a1606 = v4;
                    let a1607 = 0u64;
                    nat_rem(a1605, a1606, a1607)
                };
                let a1610 = {
                    let a1608 = v6;
                    let a1609 = 1u64;
                    nat_add(a1608, a1609)?
                };
                let a1611 = v4;
                if nat_eq(a1610, a1611) {
                    Ok(Adt5::C4({
                        let a1612 = v2;
                        let a1613 = 0u64;
                        if nat_eq(a1612, a1613) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f73(v6, v5, v1, v2)
                }
            } else {
                f78({
                    let a1614 = v3;
                    let a1615 = 2u64;
                    let a1616 = 0u64;
                    nat_quot(a1614, a1615, a1616)
                }, {
                    let a1617 = v3;
                    let a1618 = 2u64;
                    let a1619 = 0u64;
                    nat_rem(a1617, a1618, a1619)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1620 = v0;
    let a1621 = 0u64;
    if nat_eq(a1620, a1621) {
        Ok(f77(1u64))
    } else {
        f78({
            let a1630 = {
                let a1623 = v1;
                let a1624 = match v2.clone() {
                    Adt3::C0(h1622, _, _, _, _, _, _, _) => {
                        h1622
                    }
                };
                nat_mul(a1623, a1624)?
            };
            let a1631 = {
                let a1627 = {
                    let a1625 = v0;
                    let a1626 = 1u64;
                    nat_sub(a1625, a1626)
                };
                let a1628 = 2u64;
                let a1629 = 0u64;
                nat_quot(a1627, a1628, a1629)
            };
            nat_add(a1630, a1631)?
        }, {
            let a1634 = {
                let a1632 = v0;
                let a1633 = 1u64;
                nat_sub(a1632, a1633)
            };
            let a1635 = 2u64;
            let a1636 = 0u64;
            nat_rem(a1634, a1635, a1636)
        }, v2, v3, 1u64)
    }
}

pub fn f74(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1657 = {
        let a1651 = {
            let a1645 = {
                let a1639 = v1;
                let a1640 = if v5 {
                    f56(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1637, _, _, _) => {
                            h1637
                        }
                    })
                } else {
                    f55(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1638, _, _, _) => {
                            h1638
                        }
                    })
                };
                nat_mul(a1639, a1640)?
            };
            let a1646 = {
                let a1643 = v2;
                let a1644 = if v5 {
                    f56(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1641, _, _) => {
                            h1641
                        }
                    })
                } else {
                    f55(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1642, _, _) => {
                            h1642
                        }
                    })
                };
                nat_mul(a1643, a1644)?
            };
            nat_add(a1645, a1646)?
        };
        let a1652 = {
            let a1649 = v3;
            let a1650 = if v5 {
                f56(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1647, _) => {
                        h1647
                    }
                })
            } else {
                f55(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1648, _) => {
                        h1648
                    }
                })
            };
            nat_mul(a1649, a1650)?
        };
        nat_add(a1651, a1652)?
    };
    let a1658 = {
        let a1655 = v4;
        let a1656 = if v5 {
            f56(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1653) => {
                    h1653
                }
            })
        } else {
            f55(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1654) => {
                    h1654
                }
            })
        };
        nat_mul(a1655, a1656)?
    };
    nat_add(a1657, a1658)
}

pub fn f75(v0: u64, v1: u64) -> Option<u64> {
    let a1659 = v0;
    let a1660 = 2u64;
    if nat_eq(a1659, a1660) {
        Some(0u64)
    } else {
        let a1661 = v0;
        let a1662 = 5u64;
        if nat_eq(a1661, a1662) {
            Some(1u64)
        } else {
            let a1663 = v0;
            let a1664 = 6u64;
            if nat_eq(a1663, a1664) {
                Some(2u64)
            } else {
                let a1665 = v0;
                let a1666 = 7u64;
                if nat_eq(a1665, a1666) {
                    Some(3u64)
                } else {
                    let a1667 = v0;
                    let a1668 = 9u64;
                    if nat_eq(a1667, a1668) {
                        Some(4u64)
                    } else {
                        let a1669 = v0;
                        let a1670 = 8u64;
                        if nat_eq(a1669, a1670) {
                            Some(5u64)
                        } else {
                            let a1671 = v0;
                            let a1672 = 89u64;
                            if nat_eq(a1671, a1672) {
                                let a1673 = v1;
                                let a1674 = 2u64;
                                if nat_eq(a1673, a1674) {
                                    Some(6u64)
                                } else {
                                    let a1675 = v1;
                                    let a1676 = 0u64;
                                    if nat_eq(a1675, a1676) {
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

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1677 = v0;
    let a1678 = 28u64;
    if if if if nat_eq(a1677, a1678) {
        let a1679 = v1;
        let a1680 = 0u64;
        nat_eq(a1679, a1680)
    } else {
        false
    } {
        let a1681 = v2;
        let a1682 = 0u64;
        nat_eq(a1681, a1682)
    } else {
        false
    } {
        let a1683 = v3;
        let a1684 = 0u64;
        nat_eq(a1683, a1684)
    } else {
        false
    } {
        let a1685 = v4;
        let a1686 = v8;
        if if if if if if if if nat_eq(a1685, a1686) {
            let a1687 = v5;
            let a1688 = v12;
            nat_eq(a1687, a1688)
        } else {
            false
        } {
            let a1689 = v9;
            let a1690 = v13;
            nat_eq(a1689, a1690)
        } else {
            false
        } {
            let a1691 = v10;
            let a1692 = v14;
            nat_eq(a1691, a1692)
        } else {
            false
        } {
            let a1693 = v11;
            let a1694 = v15;
            nat_eq(a1693, a1694)
        } else {
            false
        } {
            let a1695 = 0u64;
            let a1696 = v6;
            nat_lt(a1695, a1696)
        } else {
            false
        } {
            let a1697 = 0u64;
            let a1698 = v7;
            nat_lt(a1697, a1698)
        } else {
            false
        } {
            let a1702 = {
                let a1699 = v10;
                let a1700 = v6;
                let a1701 = 1u64;
                nat_rem(a1699, a1700, a1701)
            };
            let a1703 = 0u64;
            if nat_eq(a1702, a1703) {
                let a1707 = {
                    let a1704 = v11;
                    let a1705 = v7;
                    let a1706 = 1u64;
                    nat_rem(a1704, a1705, a1706)
                };
                let a1708 = 0u64;
                nat_eq(a1707, a1708)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1709 = v10;
                let a1710 = v6;
                let a1711 = 0u64;
                nat_quot(a1709, a1710, a1711)
            }, {
                let a1712 = v11;
                let a1713 = v7;
                let a1714 = 0u64;
                nat_quot(a1712, a1713, a1714)
            }, {
                let a1715 = v5;
                let a1716 = v4;
                nat_mul(a1715, a1716)?
            }, {
                let a1717 = v4;
                let a1718 = v9;
                nat_mul(a1717, a1718)?
            }, {
                let a1719 = v5;
                let a1720 = v9;
                nat_mul(a1719, a1720)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f77(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f78(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1721 = v1;
    let a1722 = 0u64;
    if nat_eq(a1721, a1722) {
        Ok(Adt5::C4({
            let a1723 = v3;
            let a1724 = 0u64;
            if nat_eq(a1723, a1724) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f42(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a1725 = v3;
            let a1726 = 0u64;
            if nat_eq(a1725, a1726) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}
