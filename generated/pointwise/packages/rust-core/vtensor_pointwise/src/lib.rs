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
    C16(u64, u64),
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
                                                            let a43 = 13u64;
                                                            if if if nat_eq(a42, a43) {
                                                                true
                                                            } else {
                                                                let a44 = v1;
                                                                let a45 = 14u64;
                                                                nat_eq(a44, a45)
                                                            } {
                                                                true
                                                            } else {
                                                                let a46 = v1;
                                                                let a47 = 16u64;
                                                                nat_eq(a46, a47)
                                                            } {
                                                                f13(v1, v3, v5)?
                                                            } else {
                                                                let m48 = f14(v1, v6);
                                                                match m48 {
                                                                    None => {
                                                                        f15(v1, v6, v3, v4, v5)?
                                                                    }
                                                                    Some(v8) => {
                                                                        f16(v8, v3, v5)?
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
            f17(v9, v10, v11, v12)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a49 = v0;
    let a50 = 1u64;
    if if nat_eq(a49, a50) {
        true
    } else {
        let a51 = v0;
        let a52 = 33u64;
        nat_eq(a51, a52)
    } {
        true
    } else {
        let a53 = v0;
        let a54 = 34u64;
        nat_eq(a53, a54)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a57 = match v1.clone() {
        Adt2::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
            h55
        }
    };
    let a58 = match v3.clone() {
        Adt2::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
            h56
        }
    };
    if nat_eq(a57, a58) {
        f18(v0, v1, v2, v3)
    } else {
        if if if if f19(match v1.clone() {
            Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                h59
            }
        }) {
            f19(match v3.clone() {
                Adt2::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                    h60
                }
            })
        } else {
            false
        } {
            let a61 = v0;
            let a62 = 33u64;
            if nat_eq(a61, a62) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a63 = f20(v1.clone(), 1u64)?;
            let a64 = f20(v3.clone(), 1u64)?;
            nat_eq(a63, a64)
        } else {
            false
        } {
            let m66 = f21(match v3.clone() {
                Adt2::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                    h65
                }
            });
            match m66 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f20(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h67) => {
                            h67
                        }
                    }, f22(v1.clone(), 1u64), f22(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                            h68
                        }
                    }, match v3 {
                        Adt2::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                            h69
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
    let a71 = match v0.clone() {
        Adt2::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
            h70
        }
    };
    let a72 = 0u64;
    if if if if nat_eq(a71, a72) {
        true
    } else {
        let a74 = match v0.clone() {
            Adt2::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        };
        let a75 = 1u64;
        nat_eq(a74, a75)
    } {
        true
    } else {
        let a77 = match v0.clone() {
            Adt2::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                h76
            }
        };
        let a78 = 25u64;
        if nat_eq(a77, a78) {
            true
        } else {
            let a80 = match v0.clone() {
                Adt2::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                    h79
                }
            };
            let a81 = 26u64;
            nat_eq(a80, a81)
        }
    } {
        true
    } else {
        let a83 = match v0.clone() {
            Adt2::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                h82
            }
        };
        let a84 = 30u64;
        nat_eq(a83, a84)
    } {
        let a87 = match v0.clone() {
            Adt2::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        };
        let a88 = match v1.clone() {
            Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                h86
            }
        };
        if if nat_eq(a87, a88) {
            f23(match v1.clone() {
                Adt2::C0(_, h89, _, _, _, _, _, _, _, _, _) => {
                    h89
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h90, _, _, _, _, _, _, _, _) => {
                    h90
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h91, _, _, _, _, _, _, _) => {
                    h91
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h92, _, _, _, _, _, _) => {
                    h92
                }
            }, match v0.clone() {
                Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                    h93
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                    h94
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h95, _, _, _, _, _, _, _) => {
                    h95
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h96, _, _, _, _, _, _) => {
                    h96
                }
            })
        } else {
            false
        } {
            let m98 = f21(match v0.clone() {
                Adt2::C0(h97, _, _, _, _, _, _, _, _, _, _) => {
                    h97
                }
            });
            match m98 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f20(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h99) => {
                            h99
                        }
                    }, f22(v0, 1u64), f22(v1, 1u64), Adt4::C1))
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
    let m100 = v2;
    match m100 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a101 = v0;
            let a102 = 4u64;
            if if if nat_lt(a101, a102) {
                let a105 = match v1.clone() {
                    Adt2::C0(h103, _, _, _, _, _, _, _, _, _, _) => {
                        h103
                    }
                };
                let a106 = match v4.clone() {
                    Adt2::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                };
                if nat_eq(a105, a106) {
                    let a109 = match v1.clone() {
                        Adt2::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
                            h107
                        }
                    };
                    let a110 = match v3.clone() {
                        Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                            h108
                        }
                    };
                    nat_eq(a109, a110)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f24(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h111, _, _, _, _, _, _, _, _, _) => {
                        h111
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
                        h112
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h113, _, _, _, _, _, _, _, _, _) => {
                        h113
                    }
                })? {
                    f24(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h114, _, _, _, _, _, _, _, _) => {
                            h114
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h115, _, _, _, _, _, _, _, _) => {
                            h115
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h116, _, _, _, _, _, _, _, _) => {
                            h116
                        }
                    })?
                } else {
                    false
                } {
                    f24(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h117, _, _, _, _, _, _, _) => {
                            h117
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h118, _, _, _, _, _, _, _) => {
                            h118
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h119, _, _, _, _, _, _, _) => {
                            h119
                        }
                    })?
                } else {
                    false
                } {
                    f24(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h120, _, _, _, _, _, _) => {
                            h120
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h121, _, _, _, _, _, _) => {
                            h121
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h122, _, _, _, _, _, _) => {
                            h122
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m124 = f21(match v1.clone() {
                    Adt2::C0(h123, _, _, _, _, _, _, _, _, _, _) => {
                        h123
                    }
                });
                match m124 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h125, _) => {
                                h125
                            }
                        };
                        let a130 = {
                            let a127 = match v1.clone() {
                                Adt2::C0(_, h126, _, _, _, _, _, _, _, _, _) => {
                                    h126
                                }
                            };
                            let a128 = v6;
                            let a129 = 1u64;
                            nat_rem(a127, a128, a129)
                        };
                        let a131 = 0u64;
                        if if if nat_eq(a130, a131) {
                            let a136 = {
                                let a133 = match v4.clone() {
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
                            let a142 = {
                                let a139 = match v3.clone() {
                                    Adt2::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
                                        h138
                                    }
                                };
                                let a140 = v6;
                                let a141 = 1u64;
                                nat_rem(a139, a140, a141)
                            };
                            let a143 = 0u64;
                            nat_eq(a142, a143)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f20(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h144) => {
                                    h144
                                }
                            }, f22(v1, v6), f22(v3, v6), Adt4::C2(v0, f22(v4, v6))))
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
    let a146 = match v2.clone() {
        Adt2::C0(h145, _, _, _, _, _, _, _, _, _, _) => {
            h145
        }
    };
    let a147 = 0u64;
    if if nat_eq(a146, a147) {
        let a149 = match v3.clone() {
            Adt2::C0(h148, _, _, _, _, _, _, _, _, _, _) => {
                h148
            }
        };
        let a150 = 0u64;
        nat_eq(a149, a150)
    } else {
        false
    } {
        let a151 = v0;
        let a152 = 61u64;
        if nat_eq(a151, a152) {
            let v4: bool = {
                let a154 = 0u64;
                let a155 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h153, _, _, _, _, _, _, _) => {
                        h153
                    }
                };
                nat_lt(a154, a155)
            };
            if if if if if if f25(match v2.clone() {
                Adt2::C0(_, h156, _, _, _, _, _, _, _, _, _) => {
                    h156
                }
            }, match v1.clone() {
                Adt1::C0(h157, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h157
                }
            }, match v1.clone() {
                Adt1::C0(_, h158, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h158
                }
            }, match v3.clone() {
                Adt2::C0(_, h159, _, _, _, _, _, _, _, _, _) => {
                    h159
                }
            })? {
                f25(match v2.clone() {
                    Adt2::C0(_, _, h160, _, _, _, _, _, _, _, _) => {
                        h160
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h161, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h162, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h162
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h163, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                })?
            } else {
                false
            } {
                f25(match v2.clone() {
                    Adt2::C0(_, _, _, h164, _, _, _, _, _, _, _) => {
                        h164
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h165, _, _, _, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h166, _, _, _, _, _, _, _, _, _, _) => {
                        h166
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h167, _, _, _, _, _, _, _) => {
                        h167
                    }
                })?
            } else {
                false
            } {
                f25(match v2.clone() {
                    Adt2::C0(_, _, _, _, h168, _, _, _, _, _, _) => {
                        h168
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h169, _, _, _, _, _, _, _, _, _) => {
                        h169
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h170, _, _, _, _, _, _, _, _) => {
                        h170
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h171, _, _, _, _, _, _) => {
                        h171
                    }
                })?
            } else {
                false
            } {
                let a173 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h172, _, _, _, _, _, _, _) => {
                        h172
                    }
                };
                let a174 = 4294967295u64;
                nat_le(a173, a174)
            } else {
                false
            } {
                if v4 {
                    !f26(match v2.clone() {
                        Adt2::C0(_, h175, _, _, _, _, _, _, _, _, _) => {
                            h175
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h176, _, _, _, _, _, _, _, _) => {
                            h176
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h177, _, _, _, _, _, _, _) => {
                            h177
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h178, _, _, _, _, _, _) => {
                            h178
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f20(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h179, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h179
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h180, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h180
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h181, _, _, _, _, _, _, _, _, _, _, _) => {
                        h181
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h182, _, _, _, _, _, _, _, _, _) => {
                        h182
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a183 = v0;
            let a184 = 62u64;
            if nat_eq(a183, a184) {
                let a186 = match v1.clone() {
                    Adt1::C0(h185, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h185
                    }
                };
                let a187 = 2147483648u64;
                if if if if if nat_lt(a186, a187) {
                    let a189 = match v1.clone() {
                        Adt1::C0(_, h188, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h188
                        }
                    };
                    let a190 = 2147483648u64;
                    nat_lt(a189, a190)
                } else {
                    false
                } {
                    let a193 = match v1.clone() {
                        Adt1::C0(h191, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h191
                        }
                    };
                    let a194 = match v2.clone() {
                        Adt2::C0(_, h192, _, _, _, _, _, _, _, _, _) => {
                            h192
                        }
                    };
                    nat_lt(a193, a194)
                } else {
                    false
                } {
                    let a197 = match v1.clone() {
                        Adt1::C0(_, h195, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
                    let a201 = match v2.clone() {
                        Adt2::C0(_, _, h199, _, _, _, _, _, _, _, _) => {
                            h199
                        }
                    };
                    let a202 = match v3.clone() {
                        Adt2::C0(_, _, h200, _, _, _, _, _, _, _, _) => {
                            h200
                        }
                    };
                    if if if nat_eq(a201, a202) {
                        let a205 = match v2.clone() {
                            Adt2::C0(_, _, _, h203, _, _, _, _, _, _, _) => {
                                h203
                            }
                        };
                        let a206 = match v3.clone() {
                            Adt2::C0(_, _, _, h204, _, _, _, _, _, _, _) => {
                                h204
                            }
                        };
                        nat_eq(a205, a206)
                    } else {
                        false
                    } {
                        let a209 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h207, _, _, _, _, _, _) => {
                                h207
                            }
                        };
                        let a210 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h208, _, _, _, _, _, _) => {
                                h208
                            }
                        };
                        nat_eq(a209, a210)
                    } else {
                        false
                    } {
                        let a219 = match v3.clone() {
                            Adt2::C0(_, h211, _, _, _, _, _, _, _, _, _) => {
                                h211
                            }
                        };
                        let a220 = {
                            let a217 = {
                                let a214 = match v2.clone() {
                                    Adt2::C0(_, h212, _, _, _, _, _, _, _, _, _) => {
                                        h212
                                    }
                                };
                                let a215 = match v1.clone() {
                                    Adt1::C0(h213, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h213
                                    }
                                };
                                nat_add(a214, a215)?
                            };
                            let a218 = match v1.clone() {
                                Adt1::C0(_, h216, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h216
                                }
                            };
                            nat_add(a217, a218)?
                        };
                        nat_eq(a219, a220)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f20(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h221, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h221
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a222 = v0;
                let a223 = 63u64;
                if nat_eq(a222, a223) {
                    if if if if if f27(match v2.clone() {
                        Adt2::C0(_, h224, _, _, _, _, _, _, _, _, _) => {
                            h224
                        }
                    }, match v1.clone() {
                        Adt1::C0(h225, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h225
                        }
                    })? {
                        f27(match v2.clone() {
                            Adt2::C0(_, _, h226, _, _, _, _, _, _, _, _) => {
                                h226
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h227, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h227
                            }
                        })?
                    } else {
                        false
                    } {
                        f27(match v2.clone() {
                            Adt2::C0(_, _, _, h228, _, _, _, _, _, _, _) => {
                                h228
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h229, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h229
                            }
                        })?
                    } else {
                        false
                    } {
                        f27(match v2.clone() {
                            Adt2::C0(_, _, _, _, h230, _, _, _, _, _, _) => {
                                h230
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h231, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h231
                            }
                        })?
                    } else {
                        false
                    } {
                        let a234 = match v2.clone() {
                            Adt2::C0(_, h232, _, _, _, _, _, _, _, _, _) => {
                                h232
                            }
                        };
                        let a235 = match v3.clone() {
                            Adt2::C0(_, h233, _, _, _, _, _, _, _, _, _) => {
                                h233
                            }
                        };
                        if if if nat_eq(a234, a235) {
                            let a238 = match v2.clone() {
                                Adt2::C0(_, _, h236, _, _, _, _, _, _, _, _) => {
                                    h236
                                }
                            };
                            let a239 = match v3.clone() {
                                Adt2::C0(_, _, h237, _, _, _, _, _, _, _, _) => {
                                    h237
                                }
                            };
                            nat_eq(a238, a239)
                        } else {
                            false
                        } {
                            let a242 = match v2.clone() {
                                Adt2::C0(_, _, _, h240, _, _, _, _, _, _, _) => {
                                    h240
                                }
                            };
                            let a243 = match v3.clone() {
                                Adt2::C0(_, _, _, h241, _, _, _, _, _, _, _) => {
                                    h241
                                }
                            };
                            nat_eq(a242, a243)
                        } else {
                            false
                        } {
                            let a246 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h244, _, _, _, _, _, _) => {
                                    h244
                                }
                            };
                            let a247 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h245, _, _, _, _, _, _) => {
                                    h245
                                }
                            };
                            nat_eq(a246, a247)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f20(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C5(match v1.clone() {
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
                        }, match v1 {
                            Adt1::C0(_, _, _, h251, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h251
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
    let m252 = v3;
    match m252 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a253 = v0;
                let a254 = 4u64;
                nat_eq(a253, a254)
            };
            let v7: bool = {
                let a256 = 0u64;
                let a257 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h255, _, _, _, _, _, _, _, _, _, _, _) => {
                        h255
                    }
                };
                nat_lt(a256, a257)
            };
            let v8: u64 = f20(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h258, _, _, _, _, _, _, _, _, _) => {
                    h258
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h259, _, _, _, _, _, _, _, _) => {
                    h259
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h260, _, _, _, _, _, _, _) => {
                    h260
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h261, _, _, _, _, _, _) => {
                    h261
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h262, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h262
                }
            }, match v1.clone() {
                Adt1::C0(_, h263, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h263
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h264, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h264
                }
            });
            if if if if if if if if if if if if v6 {
                let a266 = match v2.clone() {
                    Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a267 = 0u64;
                if nat_eq(a266, a267) {
                    let a269 = match v5.clone() {
                        Adt2::C0(h268, _, _, _, _, _, _, _, _, _, _) => {
                            h268
                        }
                    };
                    let a270 = 0u64;
                    nat_eq(a269, a270)
                } else {
                    false
                }
            } else {
                let a272 = match v2.clone() {
                    Adt2::C0(h271, _, _, _, _, _, _, _, _, _, _) => {
                        h271
                    }
                };
                let a273 = 0u64;
                if if nat_eq(a272, a273) {
                    true
                } else {
                    let a275 = match v2.clone() {
                        Adt2::C0(h274, _, _, _, _, _, _, _, _, _, _) => {
                            h274
                        }
                    };
                    let a276 = 26u64;
                    nat_eq(a275, a276)
                } {
                    let a278 = match v5.clone() {
                        Adt2::C0(h277, _, _, _, _, _, _, _, _, _, _) => {
                            h277
                        }
                    };
                    let a279 = 0u64;
                    if nat_eq(a278, a279) {
                        true
                    } else {
                        let a281 = match v5.clone() {
                            Adt2::C0(h280, _, _, _, _, _, _, _, _, _, _) => {
                                h280
                            }
                        };
                        let a282 = 26u64;
                        nat_eq(a281, a282)
                    }
                } else {
                    false
                }
            } {
                let a285 = match v2.clone() {
                    Adt2::C0(h283, _, _, _, _, _, _, _, _, _, _) => {
                        h283
                    }
                };
                let a286 = match v4.clone() {
                    Adt2::C0(h284, _, _, _, _, _, _, _, _, _, _) => {
                        h284
                    }
                };
                nat_eq(a285, a286)
            } else {
                false
            } {
                f28(v2.clone(), v4.clone())
            } else {
                false
            } {
                f29(v2.clone())?
            } else {
                false
            } {
                f29(v4.clone())?
            } else {
                false
            } {
                let a288 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h287, _, _, _, _, _) => {
                        h287
                    }
                };
                let a289 = 4u64;
                nat_eq(a288, a289)
            } else {
                false
            } {
                let a290 = v8;
                let a291 = f20(v2.clone(), 1u64)?;
                nat_le(a290, a291)
            } else {
                false
            } {
                let a293 = match v1.clone() {
                    Adt1::C0(_, _, _, h292, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h292
                    }
                };
                let a294 = 1073741824u64;
                nat_lt(a293, a294)
            } else {
                false
            } {
                let a296 = match v1.clone() {
                    Adt1::C0(h295, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h295
                    }
                };
                let a297 = 4294967295u64;
                if if if if nat_le(a296, a297) {
                    let a299 = match v1.clone() {
                        Adt1::C0(_, h298, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h298
                        }
                    };
                    let a300 = 4294967295u64;
                    nat_le(a299, a300)
                } else {
                    false
                } {
                    let a302 = match v1.clone() {
                        Adt1::C0(_, _, h301, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h301
                        }
                    };
                    let a303 = 4294967295u64;
                    nat_le(a302, a303)
                } else {
                    false
                } {
                    let a305 = match v1.clone() {
                        Adt1::C0(_, _, _, h304, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h304
                        }
                    };
                    let a306 = 4294967295u64;
                    nat_le(a305, a306)
                } else {
                    false
                } {
                    let a308 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h307, _, _, _, _, _, _, _, _, _, _, _) => {
                            h307
                        }
                    };
                    let a309 = 4294967295u64;
                    nat_le(a308, a309)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a312 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h310) => {
                            h310
                        }
                    };
                    let a313 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h311) => {
                            h311
                        }
                    };
                    nat_eq(a312, a313)
                } else {
                    true
                }
            } else {
                false
            } {
                f30(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h314, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h314
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h315, _) => {
                        h315
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f20(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a316 = v10;
                    let a317 = v8;
                    nat_add(a316, a317)?
                }, 4u64, f22(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h318, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h318
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a320 = match v0.clone() {
        Adt2::C0(h319, _, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    };
    let a321 = 0u64;
    if if if nat_eq(a320, a321) {
        let a323 = match v1.clone() {
            Adt2::C0(h322, _, _, _, _, _, _, _, _, _, _) => {
                h322
            }
        };
        let a324 = 0u64;
        nat_eq(a323, a324)
    } else {
        false
    } {
        f23(match v0.clone() {
            Adt2::C0(_, h325, _, _, _, _, _, _, _, _, _) => {
                h325
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h326, _, _, _, _, _, _, _, _) => {
                h326
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h327, _, _, _, _, _, _, _) => {
                h327
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h328, _, _, _, _, _, _) => {
                h328
            }
        }, match v1.clone() {
            Adt2::C0(_, h329, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                h330
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h331, _, _, _, _, _, _, _) => {
                h331
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h332, _, _, _, _, _, _) => {
                h332
            }
        })
    } else {
        false
    } {
        let v2: u64 = f20(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a333 = v2;
            let a334 = f20(v0.clone(), 1u64)?;
            nat_add(a333, a334)?
        }, 4u64, f22(v0, 1u64), f22(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a336 = match v1.clone() {
        Adt2::C0(h335, _, _, _, _, _, _, _, _, _, _) => {
            h335
        }
    };
    let a337 = 0u64;
    if if if nat_eq(a336, a337) {
        true
    } else {
        let a339 = match v1.clone() {
            Adt2::C0(h338, _, _, _, _, _, _, _, _, _, _) => {
                h338
            }
        };
        let a340 = 1u64;
        nat_eq(a339, a340)
    } {
        true
    } else {
        let a342 = match v1.clone() {
            Adt2::C0(h341, _, _, _, _, _, _, _, _, _, _) => {
                h341
            }
        };
        let a343 = 30u64;
        nat_eq(a342, a343)
    } {
        let a346 = match v1.clone() {
            Adt2::C0(h344, _, _, _, _, _, _, _, _, _, _) => {
                h344
            }
        };
        let a347 = match v2.clone() {
            Adt2::C0(h345, _, _, _, _, _, _, _, _, _, _) => {
                h345
            }
        };
        if if if if if nat_eq(a346, a347) {
            let a350 = match v1.clone() {
                Adt2::C0(_, h348, _, _, _, _, _, _, _, _, _) => {
                    h348
                }
            };
            let a351 = match v2.clone() {
                Adt2::C0(_, h349, _, _, _, _, _, _, _, _, _) => {
                    h349
                }
            };
            nat_eq(a350, a351)
        } else {
            false
        } {
            let a354 = match v1.clone() {
                Adt2::C0(_, _, _, h352, _, _, _, _, _, _, _) => {
                    h352
                }
            };
            let a355 = match v2.clone() {
                Adt2::C0(_, _, _, h353, _, _, _, _, _, _, _) => {
                    h353
                }
            };
            nat_eq(a354, a355)
        } else {
            false
        } {
            let a358 = match v1.clone() {
                Adt2::C0(_, _, _, _, h356, _, _, _, _, _, _) => {
                    h356
                }
            };
            let a359 = match v2.clone() {
                Adt2::C0(_, _, _, _, h357, _, _, _, _, _, _) => {
                    h357
                }
            };
            nat_eq(a358, a359)
        } else {
            false
        } {
            let a360 = v0;
            let a361 = 4u64;
            if nat_eq(a360, a361) {
                let a363 = match v1.clone() {
                    Adt2::C0(_, _, h362, _, _, _, _, _, _, _, _) => {
                        h362
                    }
                };
                let a364 = 1u64;
                if nat_eq(a363, a364) {
                    let a367 = match v1.clone() {
                        Adt2::C0(_, h365, _, _, _, _, _, _, _, _, _) => {
                            h365
                        }
                    };
                    let a368 = match v2.clone() {
                        Adt2::C0(_, _, h366, _, _, _, _, _, _, _, _) => {
                            h366
                        }
                    };
                    nat_eq(a367, a368)
                } else {
                    false
                }
            } else {
                let a369 = v0;
                let a370 = 4u64;
                if if nat_lt(a369, a370) {
                    let a373 = match v1.clone() {
                        Adt2::C0(_, h371, _, _, _, _, _, _, _, _, _) => {
                            h371
                        }
                    };
                    let a374 = match v1.clone() {
                        Adt2::C0(_, _, h372, _, _, _, _, _, _, _, _) => {
                            h372
                        }
                    };
                    nat_eq(a373, a374)
                } else {
                    false
                } {
                    let a377 = match v1.clone() {
                        Adt2::C0(_, _, h375, _, _, _, _, _, _, _, _) => {
                            h375
                        }
                    };
                    let a378 = match v2.clone() {
                        Adt2::C0(_, _, h376, _, _, _, _, _, _, _, _) => {
                            h376
                        }
                    };
                    nat_eq(a377, a378)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m380 = f21(match v1.clone() {
                Adt2::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
                    h379
                }
            });
            match m380 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f20(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h381) => {
                            h381
                        }
                    }, f22(v1, 1u64), f22(v2, 1u64), Adt4::C8(v0)))
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
    if if if f31(match v1.clone() {
        Adt2::C0(h382, _, _, _, _, _, _, _, _, _, _) => {
            h382
        }
    }, v0) {
        let a385 = match v1.clone() {
            Adt2::C0(h383, _, _, _, _, _, _, _, _, _, _) => {
                h383
            }
        };
        let a386 = match v2.clone() {
            Adt2::C0(h384, _, _, _, _, _, _, _, _, _, _) => {
                h384
            }
        };
        nat_eq(a385, a386)
    } else {
        false
    } {
        let a389 = match v1.clone() {
            Adt2::C0(_, h387, _, _, _, _, _, _, _, _, _) => {
                h387
            }
        };
        let a390 = match v2.clone() {
            Adt2::C0(_, h388, _, _, _, _, _, _, _, _, _) => {
                h388
            }
        };
        if if if nat_eq(a389, a390) {
            let a393 = match v1.clone() {
                Adt2::C0(_, _, h391, _, _, _, _, _, _, _, _) => {
                    h391
                }
            };
            let a394 = match v2.clone() {
                Adt2::C0(_, _, h392, _, _, _, _, _, _, _, _) => {
                    h392
                }
            };
            nat_eq(a393, a394)
        } else {
            false
        } {
            let a397 = match v1.clone() {
                Adt2::C0(_, _, _, h395, _, _, _, _, _, _, _) => {
                    h395
                }
            };
            let a398 = match v2.clone() {
                Adt2::C0(_, _, _, h396, _, _, _, _, _, _, _) => {
                    h396
                }
            };
            nat_eq(a397, a398)
        } else {
            false
        } {
            let a401 = match v1.clone() {
                Adt2::C0(_, _, _, _, h399, _, _, _, _, _, _) => {
                    h399
                }
            };
            let a402 = match v2.clone() {
                Adt2::C0(_, _, _, _, h400, _, _, _, _, _, _) => {
                    h400
                }
            };
            nat_eq(a401, a402)
        } else {
            false
        }
    } else {
        false
    } {
        let m404 = f21(match v1.clone() {
            Adt2::C0(h403, _, _, _, _, _, _, _, _, _, _) => {
                h403
            }
        });
        match m404 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f20(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h405) => {
                        h405
                    }
                }, f22(v1.clone(), 1u64), f22(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h406, _, _, _, _, _, _, _, _, _, _) => {
                        h406
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
            Adt1::C0(h407, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h407
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h408, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h408
            }
        }
    };
    let a409 = 0u64;
    let a410 = v4;
    if if nat_lt(a409, a410) {
        let a411 = v4;
        let a412 = 2147483648u64;
        nat_lt(a411, a412)
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
        let v7: u64 = f32(match v5.clone() {
            Adt2::C0(_, _, h413, _, _, _, _, _, _, _, _) => {
                h413
            }
        }, v4)?;
        let v8: u64 = f32(match v5.clone() {
            Adt2::C0(_, _, _, h414, _, _, _, _, _, _, _) => {
                h414
            }
        }, v4)?;
        let a416 = match v5.clone() {
            Adt2::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
                h415
            }
        };
        let a417 = 0u64;
        if if if if if nat_eq(a416, a417) {
            let a419 = match v6.clone() {
                Adt2::C0(h418, _, _, _, _, _, _, _, _, _, _) => {
                    h418
                }
            };
            let a420 = 0u64;
            nat_eq(a419, a420)
        } else {
            false
        } {
            let a422 = match v5.clone() {
                Adt2::C0(_, _, _, _, h421, _, _, _, _, _, _) => {
                    h421
                }
            };
            let a423 = 1u64;
            nat_eq(a422, a423)
        } else {
            false
        } {
            let a426 = match v5.clone() {
                Adt2::C0(_, h424, _, _, _, _, _, _, _, _, _) => {
                    h424
                }
            };
            let a427 = match v6.clone() {
                Adt2::C0(_, h425, _, _, _, _, _, _, _, _, _) => {
                    h425
                }
            };
            nat_eq(a426, a427)
        } else {
            false
        } {
            if v0 {
                let a429 = match v5.clone() {
                    Adt2::C0(_, _, h428, _, _, _, _, _, _, _, _) => {
                        h428
                    }
                };
                let a430 = 2147483648u64;
                if if if nat_lt(a429, a430) {
                    let a432 = match v5.clone() {
                        Adt2::C0(_, _, _, h431, _, _, _, _, _, _, _) => {
                            h431
                        }
                    };
                    let a433 = 2147483648u64;
                    nat_lt(a432, a433)
                } else {
                    false
                } {
                    let a436 = {
                        let a434 = v7;
                        let a435 = v8;
                        nat_mul(a434, a435)?
                    };
                    let a437 = 2147483648u64;
                    nat_le(a436, a437)
                } else {
                    false
                } {
                    f33(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a439 = match v1.clone() {
                    Adt1::C0(h438, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h438
                    }
                };
                let a440 = v7;
                if if if if nat_eq(a439, a440) {
                    let a442 = match v1 {
                        Adt1::C0(_, h441, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h441
                        }
                    };
                    let a443 = v8;
                    nat_eq(a442, a443)
                } else {
                    false
                } {
                    f34(v7, v8, v4)?
                } else {
                    false
                } {
                    let a445 = match v6.clone() {
                        Adt2::C0(_, _, h444, _, _, _, _, _, _, _, _) => {
                            h444
                        }
                    };
                    let a446 = v4;
                    if nat_eq(a445, a446) {
                        let a448 = match v6.clone() {
                            Adt2::C0(_, _, _, h447, _, _, _, _, _, _, _) => {
                                h447
                            }
                        };
                        let a449 = v4;
                        nat_eq(a448, a449)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a453 = match v6 {
                        Adt2::C0(_, _, _, _, h450, _, _, _, _, _, _) => {
                            h450
                        }
                    };
                    let a454 = {
                        let a451 = v7;
                        let a452 = v8;
                        nat_mul(a451, a452)?
                    };
                    nat_eq(a453, a454)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f20(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a456 = match v2.clone() {
        Adt2::C0(h455, _, _, _, _, _, _, _, _, _, _) => {
            h455
        }
    };
    let a457 = 0u64;
    if if if if if nat_eq(a456, a457) {
        true
    } else {
        let a459 = match v2.clone() {
            Adt2::C0(h458, _, _, _, _, _, _, _, _, _, _) => {
                h458
            }
        };
        let a460 = 1u64;
        nat_eq(a459, a460)
    } {
        true
    } else {
        let a462 = match v2.clone() {
            Adt2::C0(h461, _, _, _, _, _, _, _, _, _, _) => {
                h461
            }
        };
        let a463 = 30u64;
        nat_eq(a462, a463)
    } {
        let a466 = match v2.clone() {
            Adt2::C0(h464, _, _, _, _, _, _, _, _, _, _) => {
                h464
            }
        };
        let a467 = match v3.clone() {
            Adt2::C0(h465, _, _, _, _, _, _, _, _, _, _) => {
                h465
            }
        };
        nat_eq(a466, a467)
    } else {
        false
    } {
        if if f28(v2.clone(), v3.clone()) {
            let a469 = match v1.clone() {
                Adt1::C0(h468, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h468
                }
            };
            let a470 = 4294967296u64;
            nat_lt(a469, a470)
        } else {
            false
        } {
            let a472 = match v1.clone() {
                Adt1::C0(_, h471, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h471
                }
            };
            let a473 = 4294967296u64;
            nat_lt(a472, a473)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f20(v3.clone(), 1u64)?, {
            let a475 = match v2.clone() {
                Adt2::C0(h474, _, _, _, _, _, _, _, _, _, _) => {
                    h474
                }
            };
            let a476 = 0u64;
            if nat_eq(a475, a476) {
                4u64
            } else {
                2u64
            }
        }, f22(v2.clone(), 1u64), f22(v3, 1u64), Adt4::C13({
            let a477 = v0;
            let a478 = 31u64;
            if nat_eq(a477, a478) {
                0u64
            } else {
                let a479 = v0;
                let a480 = 47u64;
                if nat_eq(a479, a480) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h481, _, _, _, _, _, _, _, _, _, _) => {
                h481
            }
        }, match v1.clone() {
            Adt1::C0(h482, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h482
            }
        }, match v1 {
            Adt1::C0(_, h483, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h483
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f12(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a485 = match v0.clone() {
        Adt2::C0(h484, _, _, _, _, _, _, _, _, _, _) => {
            h484
        }
    };
    let a486 = 0u64;
    if if if nat_eq(a485, a486) {
        let a488 = match v1.clone() {
            Adt2::C0(h487, _, _, _, _, _, _, _, _, _, _) => {
                h487
            }
        };
        let a489 = 0u64;
        nat_eq(a488, a489)
    } else {
        false
    } {
        f28(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f20(v1.clone(), 1u64)?, 4u64, f22(v0, 1u64), f22(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h490, _, _, _, _, _, _, _, _, _, _) => {
            h490
        }
    };
    let a491 = v3;
    let a492 = 0u64;
    if if nat_eq(a491, a492) {
        true
    } else {
        let a493 = v0;
        let a494 = 13u64;
        if nat_eq(a493, a494) {
            let a495 = v3;
            let a496 = 1u64;
            if nat_eq(a495, a496) {
                true
            } else {
                let a497 = v3;
                let a498 = 30u64;
                nat_eq(a497, a498)
            }
        } else {
            false
        }
    } {
        let a500 = v3;
        let a501 = match v2.clone() {
            Adt2::C0(h499, _, _, _, _, _, _, _, _, _, _) => {
                h499
            }
        };
        if if nat_eq(a500, a501) {
            let a502 = v0;
            let a503 = 13u64;
            if nat_eq(a502, a503) {
                let a505 = match v2.clone() {
                    Adt2::C0(_, h504, _, _, _, _, _, _, _, _, _) => {
                        h504
                    }
                };
                let a506 = 1u64;
                if if if nat_eq(a505, a506) {
                    let a508 = match v2.clone() {
                        Adt2::C0(_, _, h507, _, _, _, _, _, _, _, _) => {
                            h507
                        }
                    };
                    let a509 = 1u64;
                    nat_eq(a508, a509)
                } else {
                    false
                } {
                    let a511 = match v2.clone() {
                        Adt2::C0(_, _, _, h510, _, _, _, _, _, _, _) => {
                            h510
                        }
                    };
                    let a512 = 1u64;
                    nat_eq(a511, a512)
                } else {
                    false
                } {
                    let a514 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h513, _, _, _, _, _, _) => {
                            h513
                        }
                    };
                    let a515 = 1u64;
                    nat_eq(a514, a515)
                } else {
                    false
                }
            } else {
                let a517 = match v2.clone() {
                    Adt2::C0(_, h516, _, _, _, _, _, _, _, _, _) => {
                        h516
                    }
                };
                let a518 = 1u64;
                if if if nat_eq(a517, a518) {
                    let a521 = match v1.clone() {
                        Adt2::C0(_, _, h519, _, _, _, _, _, _, _, _) => {
                            h519
                        }
                    };
                    let a522 = match v2.clone() {
                        Adt2::C0(_, _, h520, _, _, _, _, _, _, _, _) => {
                            h520
                        }
                    };
                    nat_eq(a521, a522)
                } else {
                    false
                } {
                    let a525 = match v1.clone() {
                        Adt2::C0(_, _, _, h523, _, _, _, _, _, _, _) => {
                            h523
                        }
                    };
                    let a526 = match v2.clone() {
                        Adt2::C0(_, _, _, h524, _, _, _, _, _, _, _) => {
                            h524
                        }
                    };
                    nat_eq(a525, a526)
                } else {
                    false
                } {
                    let a529 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h527, _, _, _, _, _, _) => {
                            h527
                        }
                    };
                    let a530 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h528, _, _, _, _, _, _) => {
                            h528
                        }
                    };
                    nat_eq(a529, a530)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f22(v1, 1u64);
            Ok(Adt6::C4(f35(v4.clone(), v0, v3)?, {
                let a531 = v3;
                let a532 = 0u64;
                if nat_eq(a531, a532) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f22(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f14(v0: u64, v1: u64) -> Option<u64> {
    let a533 = v0;
    let a534 = 8u64;
    if nat_eq(a533, a534) {
        Some(5u64)
    } else {
        let a535 = v0;
        let a536 = 9u64;
        if nat_eq(a535, a536) {
            Some(6u64)
        } else {
            let a537 = v0;
            let a538 = 89u64;
            if nat_eq(a537, a538) {
                let a539 = v1;
                let a540 = 0u64;
                if nat_eq(a539, a540) {
                    Some(0u64)
                } else {
                    let a541 = v1;
                    let a542 = 1u64;
                    if nat_eq(a541, a542) {
                        Some(1u64)
                    } else {
                        let a543 = v1;
                        let a544 = 2u64;
                        if nat_eq(a543, a544) {
                            Some(2u64)
                        } else {
                            let a545 = v1;
                            let a546 = 3u64;
                            if nat_eq(a545, a546) {
                                Some(3u64)
                            } else {
                                let a547 = v1;
                                let a548 = 6u64;
                                if nat_eq(a547, a548) {
                                    Some(4u64)
                                } else {
                                    let a549 = v1;
                                    let a550 = 12u64;
                                    if nat_eq(a549, a550) {
                                        Some(7u64)
                                    } else {
                                        let a551 = v1;
                                        let a552 = 11u64;
                                        if nat_eq(a551, a552) {
                                            Some(8u64)
                                        } else {
                                            let a553 = v1;
                                            let a554 = 18u64;
                                            if nat_eq(a553, a554) {
                                                Some(9u64)
                                            } else {
                                                let a555 = v1;
                                                let a556 = 19u64;
                                                if nat_eq(a555, a556) {
                                                    Some(10u64)
                                                } else {
                                                    let a557 = v1;
                                                    let a558 = 20u64;
                                                    if nat_eq(a557, a558) {
                                                        Some(11u64)
                                                    } else {
                                                        let a559 = v1;
                                                        let a560 = 21u64;
                                                        if nat_eq(a559, a560) {
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

pub fn f15(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m561 = f36(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m561 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f22(v2.clone(), 1u64), f22(v4, 1u64), Adt4::C10(v5, v6, {
                let m562 = v3;
                match m562 {
                    None => {
                        f22(v2, 1u64)
                    }
                    Some(v12) => {
                        f22(v12, 1u64)
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

pub fn f16(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a564 = match v1.clone() {
        Adt2::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
            h563
        }
    };
    let a565 = 0u64;
    if if if if if nat_eq(a564, a565) {
        true
    } else {
        let a567 = match v1.clone() {
            Adt2::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
                h566
            }
        };
        let a568 = 1u64;
        nat_eq(a567, a568)
    } {
        true
    } else {
        let a570 = match v1.clone() {
            Adt2::C0(h569, _, _, _, _, _, _, _, _, _, _) => {
                h569
            }
        };
        let a571 = 30u64;
        nat_eq(a570, a571)
    } {
        let a574 = match v1.clone() {
            Adt2::C0(h572, _, _, _, _, _, _, _, _, _, _) => {
                h572
            }
        };
        let a575 = match v2.clone() {
            Adt2::C0(h573, _, _, _, _, _, _, _, _, _, _) => {
                h573
            }
        };
        if nat_eq(a574, a575) {
            true
        } else {
            let a577 = match v2.clone() {
                Adt2::C0(h576, _, _, _, _, _, _, _, _, _, _) => {
                    h576
                }
            };
            let a578 = 0u64;
            nat_eq(a577, a578)
        }
    } else {
        false
    } {
        f28(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f20(v2.clone(), 1u64)?, {
            let a580 = match v2.clone() {
                Adt2::C0(h579, _, _, _, _, _, _, _, _, _, _) => {
                    h579
                }
            };
            let a581 = 0u64;
            if nat_eq(a580, a581) {
                4u64
            } else {
                2u64
            }
        }, f22(v1.clone(), 1u64), f22(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h582, _, _, _, _, _, _, _, _, _, _) => {
                h582
            }
        }, match v2 {
            Adt2::C0(h583, _, _, _, _, _, _, _, _, _, _) => {
                h583
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f17(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m584 = v3;
    match m584 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f37(v0, v1)?, f37(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f38(v0, v1, v2.clone())?, f37(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f39(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f40(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f41(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f40(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f42(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f43(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f44(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f37(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f45(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f37(v0, v1)?, f37(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f46(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f37(v0, v1)?, f37(v0, v2)?))
        }
        Adt4::C14(_, v36, v37) => {
            Ok(Adt5::C3(v36, v37, f37(v0, v1)?, f37(v0, v2)?))
        }
        Adt4::C15 => {
            f47(v0, v1, v2)
        }
        Adt4::C16(v38, v39) => {
            f48(v0, v1, v2, v38, v39)
        }
    }
}

pub fn f18(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a587 = match v1.clone() {
        Adt2::C0(h585, _, _, _, _, _, _, _, _, _, _) => {
            h585
        }
    };
    let a588 = match v3.clone() {
        Adt2::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
            h586
        }
    };
    if if nat_eq(a587, a588) {
        let a589 = v0;
        let a590 = 33u64;
        if nat_eq(a589, a590) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m592 = f21(match v1.clone() {
            Adt2::C0(h591, _, _, _, _, _, _, _, _, _, _) => {
                h591
            }
        });
        match m592 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h593, _) => {
                        h593
                    }
                };
                let v6: u64 = f20(v1.clone(), v5)?;
                let a598 = {
                    let a595 = match v1.clone() {
                        Adt2::C0(_, h594, _, _, _, _, _, _, _, _, _) => {
                            h594
                        }
                    };
                    let a596 = v5;
                    let a597 = 1u64;
                    nat_rem(a595, a596, a597)
                };
                let a599 = 0u64;
                if if if nat_eq(a598, a599) {
                    let a604 = {
                        let a601 = match v3.clone() {
                            Adt2::C0(_, h600, _, _, _, _, _, _, _, _, _) => {
                                h600
                            }
                        };
                        let a602 = v5;
                        let a603 = 1u64;
                        nat_rem(a601, a602, a603)
                    };
                    let a605 = 0u64;
                    nat_eq(a604, a605)
                } else {
                    false
                } {
                    let a606 = v6;
                    let a607 = f20(v3.clone(), v5)?;
                    nat_eq(a606, a607)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h608) => {
                            h608
                        }
                    }, f22(v1, v5), f22(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f19(v0: u64) -> bool {
    let a609 = v0;
    let a610 = 0u64;
    if if nat_eq(a609, a610) {
        true
    } else {
        let a611 = v0;
        let a612 = 1u64;
        nat_eq(a611, a612)
    } {
        true
    } else {
        let a613 = v0;
        let a614 = 30u64;
        nat_eq(a613, a614)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> R<u64> {
    let a626 = {
        let a620 = {
            let a616 = match v0.clone() {
                Adt2::C0(_, h615, _, _, _, _, _, _, _, _, _) => {
                    h615
                }
            };
            let a617 = v1;
            let a618 = 0u64;
            nat_quot(a616, a617, a618)
        };
        let a621 = match v0.clone() {
            Adt2::C0(_, _, h619, _, _, _, _, _, _, _, _) => {
                h619
            }
        };
        nat_mul(a620, a621)?
    };
    let a627 = {
        let a624 = match v0.clone() {
            Adt2::C0(_, _, _, h622, _, _, _, _, _, _, _) => {
                h622
            }
        };
        let a625 = match v0 {
            Adt2::C0(_, _, _, _, h623, _, _, _, _, _, _) => {
                h623
            }
        };
        nat_mul(a624, a625)?
    };
    nat_mul(a626, a627)
}

pub fn f21(v0: u64) -> Adt7 {
    let a628 = v0;
    let a629 = 0u64;
    if nat_eq(a628, a629) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a630 = v0;
        let a631 = 1u64;
        if nat_eq(a630, a631) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a632 = v0;
            let a633 = 2u64;
            if nat_eq(a632, a633) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a634 = v0;
                let a635 = 3u64;
                if nat_eq(a634, a635) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a636 = v0;
                    let a637 = 6u64;
                    if nat_eq(a636, a637) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a638 = v0;
                        let a639 = 7u64;
                        if nat_eq(a638, a639) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a640 = v0;
                            let a641 = 8u64;
                            if nat_eq(a640, a641) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a642 = v0;
                                let a643 = 9u64;
                                if nat_eq(a642, a643) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a644 = v0;
                                    let a645 = 10u64;
                                    if nat_eq(a644, a645) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a646 = v0;
                                        let a647 = 11u64;
                                        if nat_eq(a646, a647) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a648 = v0;
                                            let a649 = 12u64;
                                            if nat_eq(a648, a649) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a650 = v0;
                                                let a651 = 13u64;
                                                if nat_eq(a650, a651) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a652 = v0;
                                                    let a653 = 14u64;
                                                    if nat_eq(a652, a653) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a654 = v0;
                                                        let a655 = 15u64;
                                                        if nat_eq(a654, a655) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a656 = v0;
                                                            let a657 = 16u64;
                                                            if nat_eq(a656, a657) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a658 = v0;
                                                                let a659 = 17u64;
                                                                if nat_eq(a658, a659) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a660 = v0;
                                                                    let a661 = 18u64;
                                                                    if nat_eq(a660, a661) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a662 = v0;
                                                                        let a663 = 19u64;
                                                                        if nat_eq(a662, a663) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a664 = v0;
                                                                            let a665 = 20u64;
                                                                            if nat_eq(a664, a665) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a666 = v0;
                                                                                let a667 = 21u64;
                                                                                if nat_eq(a666, a667) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a668 = v0;
                                                                                    let a669 = 22u64;
                                                                                    if nat_eq(a668, a669) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a670 = v0;
                                                                                        let a671 = 23u64;
                                                                                        if nat_eq(a670, a671) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a672 = v0;
                                                                                            let a673 = 24u64;
                                                                                            if nat_eq(a672, a673) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a674 = v0;
                                                                                                let a675 = 25u64;
                                                                                                if nat_eq(a674, a675) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a676 = v0;
                                                                                                    let a677 = 26u64;
                                                                                                    if nat_eq(a676, a677) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a678 = v0;
                                                                                                        let a679 = 27u64;
                                                                                                        if nat_eq(a678, a679) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a680 = v0;
                                                                                                            let a681 = 28u64;
                                                                                                            if nat_eq(a680, a681) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a682 = v0;
                                                                                                                let a683 = 29u64;
                                                                                                                if nat_eq(a682, a683) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a684 = v0;
                                                                                                                    let a685 = 30u64;
                                                                                                                    if nat_eq(a684, a685) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a686 = v0;
                                                                                                                        let a687 = 34u64;
                                                                                                                        if nat_eq(a686, a687) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a688 = v0;
                                                                                                                            let a689 = 35u64;
                                                                                                                            if nat_eq(a688, a689) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a690 = v0;
                                                                                                                                let a691 = 39u64;
                                                                                                                                if nat_eq(a690, a691) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a692 = v0;
                                                                                                                                    let a693 = 40u64;
                                                                                                                                    if nat_eq(a692, a693) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a694 = v0;
                                                                                                                                        let a695 = 41u64;
                                                                                                                                        if nat_eq(a694, a695) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a696 = v0;
                                                                                                                                            let a697 = 42u64;
                                                                                                                                            if nat_eq(a696, a697) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a698 = v0;
                                                                                                                                                let a699 = 43u64;
                                                                                                                                                if nat_eq(a698, a699) {
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

pub fn f22(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a701 = match v0.clone() {
            Adt2::C0(_, h700, _, _, _, _, _, _, _, _, _) => {
                h700
            }
        };
        let a702 = v1;
        let a703 = 0u64;
        nat_quot(a701, a702, a703)
    }, match v0.clone() {
        Adt2::C0(_, _, h704, _, _, _, _, _, _, _, _) => {
            h704
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h705, _, _, _, _, _, _, _) => {
            h705
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h706, _, _, _, _, _, _) => {
            h706
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h707, _, _, _, _, _) => {
            h707
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h708, _, _, _, _) => {
            h708
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h709, _, _, _) => {
            h709
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h710, _, _) => {
            h710
        }
    })
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f26(v4, v5, v6, v7) {
        f26(v0, v1, v2, v3)
    } else {
        let a714 = {
            let a711 = v0;
            let a712 = v4;
            let a713 = 1u64;
            nat_rem(a711, a712, a713)
        };
        let a715 = 0u64;
        if if if nat_eq(a714, a715) {
            let a719 = {
                let a716 = v1;
                let a717 = v5;
                let a718 = 1u64;
                nat_rem(a716, a717, a718)
            };
            let a720 = 0u64;
            nat_eq(a719, a720)
        } else {
            false
        } {
            let a724 = {
                let a721 = v2;
                let a722 = v6;
                let a723 = 1u64;
                nat_rem(a721, a722, a723)
            };
            let a725 = 0u64;
            nat_eq(a724, a725)
        } else {
            false
        } {
            let a729 = {
                let a726 = v3;
                let a727 = v7;
                let a728 = 1u64;
                nat_rem(a726, a727, a728)
            };
            let a730 = 0u64;
            nat_eq(a729, a730)
        } else {
            false
        }
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a731 = v0;
    let a732 = v1;
    if nat_eq(a731, a732) {
        let a735 = v4;
        let a736 = {
            let a733 = v2;
            let a734 = v3;
            nat_add(a733, a734)?
        };
        Ok(nat_eq(a735, a736))
    } else {
        let a737 = v2;
        let a738 = v3;
        if nat_eq(a737, a738) {
            let a739 = v4;
            let a740 = v2;
            Ok(nat_eq(a739, a740))
        } else {
            Ok(false)
        }
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a743 = {
            let a741 = v0;
            let a742 = f49(v1);
            nat_add(a741, a742)?
        };
        let a744 = f49(v2);
        nat_add(a743, a744)?
    };
    let v5: u64 = {
        let a745 = f50(v1);
        let a746 = f50(v2);
        nat_add(a745, a746)?
    };
    let a747 = v1;
    let a748 = 4294967295u64;
    if if nat_le(a747, a748) {
        let a749 = v2;
        let a750 = 4294967295u64;
        nat_le(a749, a750)
    } else {
        false
    } {
        let a751 = v5;
        let a752 = v4;
        if nat_le(a751, a752) {
            let a755 = v3;
            let a756 = {
                let a753 = v4;
                let a754 = v5;
                nat_sub(a753, a754)
            };
            Ok(nat_eq(a755, a756))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a757 = v0;
    let a758 = 0u64;
    if if if nat_eq(a757, a758) {
        true
    } else {
        let a759 = v1;
        let a760 = 0u64;
        nat_eq(a759, a760)
    } {
        true
    } else {
        let a761 = v2;
        let a762 = 0u64;
        nat_eq(a761, a762)
    } {
        true
    } else {
        let a763 = v3;
        let a764 = 0u64;
        nat_eq(a763, a764)
    }
}

pub fn f27(v0: u64, v1: u64) -> R<bool> {
    let a765 = v1;
    let a766 = 4294967295u64;
    if nat_le(a765, a766) {
        let a769 = {
            let a767 = f49(v1);
            let a768 = f50(v1);
            nat_add(a767, a768)?
        };
        let a770 = v0;
        Ok(nat_lt(a769, a770))
    } else {
        Ok(false)
    }
}

pub fn f28(v0: Adt2, v1: Adt2) -> bool {
    let a773 = match v0.clone() {
        Adt2::C0(_, h771, _, _, _, _, _, _, _, _, _) => {
            h771
        }
    };
    let a774 = match v1.clone() {
        Adt2::C0(_, h772, _, _, _, _, _, _, _, _, _) => {
            h772
        }
    };
    if if if nat_eq(a773, a774) {
        let a777 = match v0.clone() {
            Adt2::C0(_, _, h775, _, _, _, _, _, _, _, _) => {
                h775
            }
        };
        let a778 = match v1.clone() {
            Adt2::C0(_, _, h776, _, _, _, _, _, _, _, _) => {
                h776
            }
        };
        nat_eq(a777, a778)
    } else {
        false
    } {
        let a781 = match v0.clone() {
            Adt2::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
                h779
            }
        };
        let a782 = match v1.clone() {
            Adt2::C0(_, _, _, h780, _, _, _, _, _, _, _) => {
                h780
            }
        };
        nat_eq(a781, a782)
    } else {
        false
    } {
        let a785 = match v0 {
            Adt2::C0(_, _, _, _, h783, _, _, _, _, _, _) => {
                h783
            }
        };
        let a786 = match v1 {
            Adt2::C0(_, _, _, _, h784, _, _, _, _, _, _) => {
                h784
            }
        };
        nat_eq(a785, a786)
    } else {
        false
    }
}

pub fn f29(v0: Adt2) -> R<bool> {
    let a788 = match v0.clone() {
        Adt2::C0(_, h787, _, _, _, _, _, _, _, _, _) => {
            h787
        }
    };
    let a789 = 1u64;
    if if if if nat_eq(a788, a789) {
        true
    } else {
        let a791 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h790, _, _, _, _, _) => {
                h790
            }
        };
        let a792 = 4u64;
        nat_eq(a791, a792)
    } {
        let a794 = match v0.clone() {
            Adt2::C0(_, _, h793, _, _, _, _, _, _, _, _) => {
                h793
            }
        };
        let a795 = 1u64;
        if nat_eq(a794, a795) {
            true
        } else {
            let a800 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h796, _, _, _, _) => {
                    h796
                }
            };
            let a801 = {
                let a798 = match v0.clone() {
                    Adt2::C0(_, h797, _, _, _, _, _, _, _, _, _) => {
                        h797
                    }
                };
                let a799 = 4u64;
                nat_mul(a798, a799)?
            };
            nat_eq(a800, a801)
        }
    } else {
        false
    } {
        let a803 = match v0.clone() {
            Adt2::C0(_, _, _, h802, _, _, _, _, _, _, _) => {
                h802
            }
        };
        let a804 = 1u64;
        if nat_eq(a803, a804) {
            true
        } else {
            let a812 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h805, _, _, _) => {
                    h805
                }
            };
            let a813 = {
                let a810 = {
                    let a808 = match v0.clone() {
                        Adt2::C0(_, h806, _, _, _, _, _, _, _, _, _) => {
                            h806
                        }
                    };
                    let a809 = match v0.clone() {
                        Adt2::C0(_, _, h807, _, _, _, _, _, _, _, _) => {
                            h807
                        }
                    };
                    nat_mul(a808, a809)?
                };
                let a811 = 4u64;
                nat_mul(a810, a811)?
            };
            nat_eq(a812, a813)
        }
    } else {
        false
    } {
        let a815 = match v0.clone() {
            Adt2::C0(_, _, _, _, h814, _, _, _, _, _, _) => {
                h814
            }
        };
        let a816 = 1u64;
        if nat_eq(a815, a816) {
            Ok(true)
        } else {
            let a827 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h817, _, _) => {
                    h817
                }
            };
            let a828 = {
                let a825 = {
                    let a823 = {
                        let a820 = match v0.clone() {
                            Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                                h818
                            }
                        };
                        let a821 = match v0.clone() {
                            Adt2::C0(_, _, h819, _, _, _, _, _, _, _, _) => {
                                h819
                            }
                        };
                        nat_mul(a820, a821)?
                    };
                    let a824 = match v0 {
                        Adt2::C0(_, _, _, h822, _, _, _, _, _, _, _) => {
                            h822
                        }
                    };
                    nat_mul(a823, a824)?
                };
                let a826 = 4u64;
                nat_mul(a825, a826)?
            };
            Ok(nat_eq(a827, a828))
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f51(v0.clone(), false)?;
    let v6: u64 = f51(v0, true)?;
    let a829 = v6;
    let a830 = v1;
    if if nat_le(a829, a830) {
        let a837 = {
            let a835 = {
                let a831 = v1;
                let a832 = v5;
                nat_add(a831, a832)?
            };
            let a836 = {
                let a833 = v3;
                let a834 = 0u64;
                if nat_eq(a833, a834) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a835, a836)?
        };
        let a838 = v2;
        nat_le(a837, a838)
    } else {
        false
    } {
        if v4 {
            let a843 = {
                let a841 = {
                    let a839 = v1;
                    let a840 = v5;
                    nat_add(a839, a840)?
                };
                let a842 = v6;
                nat_sub(a841, a842)
            };
            let a844 = v2;
            Ok(nat_lt(a843, a844))
        } else {
            let a849 = {
                let a847 = {
                    let a845 = v1;
                    let a846 = v5;
                    nat_add(a845, a846)?
                };
                let a848 = v6;
                nat_sub(a847, a848)
            };
            let a850 = v2;
            Ok(nat_le(a849, a850))
        }
    } else {
        Ok(false)
    }
}

pub fn f31(v0: u64, v1: u64) -> bool {
    let a851 = v0;
    let a852 = 0u64;
    if if nat_eq(a851, a852) {
        true
    } else {
        let a853 = v0;
        let a854 = 1u64;
        nat_eq(a853, a854)
    } {
        let a855 = v1;
        let a856 = 4294967296u64;
        nat_lt(a855, a856)
    } else {
        false
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a861 = {
        let a859 = v0;
        let a860 = {
            let a857 = v1;
            let a858 = 1u64;
            nat_sub(a857, a858)
        };
        nat_add(a859, a860)?
    };
    let a862 = v1;
    let a863 = 0u64;
    Ok(nat_quot(a861, a862, a863))
}

pub fn f33(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f26(match v0.clone() {
        Adt2::C0(_, h864, _, _, _, _, _, _, _, _, _) => {
            h864
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h865, _, _, _, _, _, _, _, _) => {
            h865
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h866, _, _, _, _, _, _, _) => {
            h866
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h867, _, _, _, _, _, _) => {
            h867
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a870 = match v1.clone() {
                Adt2::C0(_, h868, _, _, _, _, _, _, _, _, _) => {
                    h868
                }
            };
            let a871 = match v1.clone() {
                Adt2::C0(_, _, h869, _, _, _, _, _, _, _, _) => {
                    h869
                }
            };
            nat_mul(a870, a871)?
        };
        let v5: u64 = {
            let a873 = v4;
            let a874 = match v1.clone() {
                Adt2::C0(_, _, _, h872, _, _, _, _, _, _, _) => {
                    h872
                }
            };
            nat_mul(a873, a874)?
        };
        let a887 = {
            let a885 = {
                let a876 = match v0.clone() {
                    Adt2::C0(_, h875, _, _, _, _, _, _, _, _, _) => {
                        h875
                    }
                };
                let a877 = 1u64;
                nat_sub(a876, a877)
            };
            let a886 = {
                let a883 = f52(match v0.clone() {
                    Adt2::C0(_, _, h878, _, _, _, _, _, _, _, _) => {
                        h878
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h879, _, _, _, _, _, _, _, _, _) => {
                        h879
                    }
                })?;
                let a884 = f52(match v0 {
                    Adt2::C0(_, _, _, h880, _, _, _, _, _, _, _) => {
                        h880
                    }
                }, v2, {
                    let a881 = v3;
                    let a882 = v5;
                    nat_mul(a881, a882)?
                }, v4)?;
                nat_add(a883, a884)?
            };
            nat_add(a885, a886)?
        };
        let a888 = f20(v1, 1u64)?;
        Ok(nat_lt(a887, a888))
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a889 = v0;
    let a890 = 2147483648u64;
    if if if if nat_lt(a889, a890) {
        let a891 = v1;
        let a892 = 2147483648u64;
        nat_lt(a891, a892)
    } else {
        false
    } {
        let a895 = {
            let a893 = v0;
            let a894 = v1;
            nat_mul(a893, a894)?
        };
        let a896 = 2147483648u64;
        nat_lt(a895, a896)
    } else {
        false
    } {
        let a901 = {
            let a899 = {
                let a897 = v0;
                let a898 = 1u64;
                nat_sub(a897, a898)
            };
            let a900 = v2;
            nat_mul(a899, a900)?
        };
        let a902 = 2147483648u64;
        nat_lt(a901, a902)
    } else {
        false
    } {
        let a907 = {
            let a905 = {
                let a903 = v1;
                let a904 = 1u64;
                nat_sub(a903, a904)
            };
            let a906 = v2;
            nat_mul(a905, a906)?
        };
        let a908 = 2147483648u64;
        Ok(nat_lt(a907, a908))
    } else {
        Ok(false)
    }
}

pub fn f35(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a914 = match v0.clone() {
            Adt3::C0(_, h909, _, _, _, _, _, _) => {
                h909
            }
        };
        let a915 = {
            let a912 = match v0.clone() {
                Adt3::C0(_, _, h910, _, _, _, _, _) => {
                    h910
                }
            };
            let a913 = match v0.clone() {
                Adt3::C0(_, _, _, h911, _, _, _, _) => {
                    h911
                }
            };
            nat_mul(a912, a913)?
        };
        nat_mul(a914, a915)?
    };
    let a916 = v1;
    let a917 = 13u64;
    if nat_eq(a916, a917) {
        let a926 = 2u64;
        let a927 = if f53(v0.clone(), v2) {
            let a919 = v3;
            let a920 = f54(match v0 {
                Adt3::C0(h918, _, _, _, _, _, _, _) => {
                    h918
                }
            }, false)?;
            nat_mul(a919, a920)?
        } else {
            let a924 = v3;
            let a925 = {
                let a922 = match v0 {
                    Adt3::C0(h921, _, _, _, _, _, _, _) => {
                        h921
                    }
                };
                let a923 = 2u64;
                nat_mul(a922, a923)?
            };
            nat_mul(a924, a925)?
        };
        nat_add(a926, a927)
    } else {
        let a931 = v3;
        let a932 = f54(match v0 {
            Adt3::C0(h928, _, _, _, _, _, _, _) => {
                h928
            }
        }, {
            let a929 = v1;
            let a930 = 16u64;
            nat_eq(a929, a930)
        })?;
        nat_mul(a931, a932)
    }
}

pub fn f36(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m933 = v3;
    match m933 {
        None => {
            f55(v0, v1, match v2.clone() {
                Adt2::C0(h934, _, _, _, _, _, _, _, _, _, _) => {
                    h934
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h935, _, _, _, _, _, _, _, _, _, _) => {
                    h935
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h936, _, _, _, _, _, _, _, _, _) => {
                    h936
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                    h937
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h938, _, _, _, _, _, _, _) => {
                    h938
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h939, _, _, _, _, _, _) => {
                    h939
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h940, _, _, _, _, _, _, _, _, _) => {
                    h940
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h941, _, _, _, _, _, _, _, _) => {
                    h941
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h942, _, _, _, _, _, _, _) => {
                    h942
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h943, _, _, _, _, _, _) => {
                    h943
                }
            })
        }
        Some(v5) => {
            f55(v0, v1, match v2.clone() {
                Adt2::C0(h944, _, _, _, _, _, _, _, _, _, _) => {
                    h944
                }
            }, match v5.clone() {
                Adt2::C0(h945, _, _, _, _, _, _, _, _, _, _) => {
                    h945
                }
            }, match v4.clone() {
                Adt2::C0(h946, _, _, _, _, _, _, _, _, _, _) => {
                    h946
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h947, _, _, _, _, _, _, _, _, _) => {
                    h947
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h948, _, _, _, _, _, _, _, _) => {
                    h948
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h949, _, _, _, _, _, _, _) => {
                    h949
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h950, _, _, _, _, _, _) => {
                    h950
                }
            }, match v5.clone() {
                Adt2::C0(_, h951, _, _, _, _, _, _, _, _, _) => {
                    h951
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h952, _, _, _, _, _, _, _, _) => {
                    h952
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h953, _, _, _, _, _, _, _) => {
                    h953
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h954, _, _, _, _, _, _) => {
                    h954
                }
            }, match v4.clone() {
                Adt2::C0(_, h955, _, _, _, _, _, _, _, _, _) => {
                    h955
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h956, _, _, _, _, _, _, _, _) => {
                    h956
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h957, _, _, _, _, _, _, _) => {
                    h957
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h958, _, _, _, _, _, _) => {
                    h958
                }
            })
        }
    }
}

pub fn f37(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a960 = v0;
        let a961 = match v1.clone() {
            Adt3::C0(h959, _, _, _, _, _, _, _) => {
                h959
            }
        };
        let a962 = 0u64;
        nat_quot(a960, a961, a962)
    };
    let v3: u64 = {
        let a964 = v2;
        let a965 = match v1.clone() {
            Adt3::C0(_, h963, _, _, _, _, _, _) => {
                h963
            }
        };
        let a966 = 0u64;
        nat_quot(a964, a965, a966)
    };
    f56({
        let a968 = v0;
        let a969 = match v1.clone() {
            Adt3::C0(h967, _, _, _, _, _, _, _) => {
                h967
            }
        };
        let a970 = 0u64;
        nat_rem(a968, a969, a970)
    }, {
        let a972 = v2;
        let a973 = match v1.clone() {
            Adt3::C0(_, h971, _, _, _, _, _, _) => {
                h971
            }
        };
        let a974 = 0u64;
        nat_rem(a972, a973, a974)
    }, {
        let a976 = v3;
        let a977 = match v1.clone() {
            Adt3::C0(_, _, h975, _, _, _, _, _) => {
                h975
            }
        };
        let a978 = 0u64;
        nat_rem(a976, a977, a978)
    }, {
        let a980 = v3;
        let a981 = match v1.clone() {
            Adt3::C0(_, _, h979, _, _, _, _, _) => {
                h979
            }
        };
        let a982 = 0u64;
        nat_quot(a980, a981, a982)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h983, _, _, _) => {
            h983
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h984, _, _) => {
            h984
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h985, _) => {
            h985
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h986) => {
            h986
        }
    })
}

pub fn f38(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a988 = v0;
        let a989 = match v2.clone() {
            Adt3::C0(h987, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a990 = 0u64;
        nat_quot(a988, a989, a990)
    };
    let v4: u64 = {
        let a992 = v3;
        let a993 = match v2.clone() {
            Adt3::C0(_, h991, _, _, _, _, _, _) => {
                h991
            }
        };
        let a994 = 0u64;
        nat_quot(a992, a993, a994)
    };
    f57({
        let a996 = v0;
        let a997 = match v2.clone() {
            Adt3::C0(h995, _, _, _, _, _, _, _) => {
                h995
            }
        };
        let a998 = 0u64;
        nat_rem(a996, a997, a998)
    }, {
        let a1000 = v3;
        let a1001 = match v2.clone() {
            Adt3::C0(_, h999, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1002 = 0u64;
        nat_rem(a1000, a1001, a1002)
    }, {
        let a1004 = v4;
        let a1005 = match v2.clone() {
            Adt3::C0(_, _, h1003, _, _, _, _, _) => {
                h1003
            }
        };
        let a1006 = 0u64;
        nat_rem(a1004, a1005, a1006)
    }, {
        let a1008 = v4;
        let a1009 = match v2 {
            Adt3::C0(_, _, h1007, _, _, _, _, _) => {
                h1007
            }
        };
        let a1010 = 0u64;
        nat_quot(a1008, a1009, a1010)
    }, match v1.clone() {
        Adt3::C0(h1011, _, _, _, _, _, _, _) => {
            h1011
        }
    }, match v1.clone() {
        Adt3::C0(_, h1012, _, _, _, _, _, _) => {
            h1012
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1013, _, _, _, _, _) => {
            h1013
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1014, _, _, _, _) => {
            h1014
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1015, _, _, _) => {
            h1015
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1016, _, _) => {
            h1016
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1017, _) => {
            h1017
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1018) => {
            h1018
        }
    })
}

pub fn f39(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1020 = v0;
        let a1021 = match v3.clone() {
            Adt3::C0(h1019, _, _, _, _, _, _, _) => {
                h1019
            }
        };
        let a1022 = 0u64;
        nat_quot(a1020, a1021, a1022)
    };
    let v6: u64 = {
        let a1024 = v5;
        let a1025 = match v3.clone() {
            Adt3::C0(_, h1023, _, _, _, _, _, _) => {
                h1023
            }
        };
        let a1026 = 0u64;
        nat_quot(a1024, a1025, a1026)
    };
    let v7: u64 = {
        let a1028 = v0;
        let a1029 = match v3.clone() {
            Adt3::C0(h1027, _, _, _, _, _, _, _) => {
                h1027
            }
        };
        let a1030 = 0u64;
        nat_rem(a1028, a1029, a1030)
    };
    let v8: u64 = {
        let a1032 = v5;
        let a1033 = match v3.clone() {
            Adt3::C0(_, h1031, _, _, _, _, _, _) => {
                h1031
            }
        };
        let a1034 = 0u64;
        nat_rem(a1032, a1033, a1034)
    };
    let v9: u64 = {
        let a1036 = v6;
        let a1037 = match v3.clone() {
            Adt3::C0(_, _, h1035, _, _, _, _, _) => {
                h1035
            }
        };
        let a1038 = 0u64;
        nat_rem(a1036, a1037, a1038)
    };
    let v10: u64 = {
        let a1040 = v6;
        let a1041 = match v3.clone() {
            Adt3::C0(_, _, h1039, _, _, _, _, _) => {
                h1039
            }
        };
        let a1042 = 0u64;
        nat_quot(a1040, a1041, a1042)
    };
    let v11: u64 = {
        let a1043 = v4;
        let a1044 = 0u64;
        if nat_eq(a1043, a1044) {
            match v1.clone() {
                Adt3::C0(h1045, _, _, _, _, _, _, _) => {
                    h1045
                }
            }
        } else {
            let a1046 = v4;
            let a1047 = 1u64;
            if nat_eq(a1046, a1047) {
                match v1.clone() {
                    Adt3::C0(_, h1048, _, _, _, _, _, _) => {
                        h1048
                    }
                }
            } else {
                let a1049 = v4;
                let a1050 = 2u64;
                if nat_eq(a1049, a1050) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1051, _, _, _, _, _) => {
                            h1051
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1052, _, _, _, _) => {
                            h1052
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1059 = v11;
        let a1060 = {
            let a1053 = v4;
            let a1054 = 0u64;
            if nat_eq(a1053, a1054) {
                v7
            } else {
                let a1055 = v4;
                let a1056 = 1u64;
                if nat_eq(a1055, a1056) {
                    v8
                } else {
                    let a1057 = v4;
                    let a1058 = 2u64;
                    if nat_eq(a1057, a1058) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1059, a1060)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f56(if if v12 {
        let a1061 = v4;
        let a1062 = 0u64;
        nat_eq(a1061, a1062)
    } else {
        false
    } {
        let a1063 = v7;
        let a1064 = v11;
        nat_sub(a1063, a1064)
    } else {
        v7
    }, if if v12 {
        let a1065 = v4;
        let a1066 = 1u64;
        nat_eq(a1065, a1066)
    } else {
        false
    } {
        let a1067 = v8;
        let a1068 = v11;
        nat_sub(a1067, a1068)
    } else {
        v8
    }, if if v12 {
        let a1069 = v4;
        let a1070 = 2u64;
        nat_eq(a1069, a1070)
    } else {
        false
    } {
        let a1071 = v9;
        let a1072 = v11;
        nat_sub(a1071, a1072)
    } else {
        v9
    }, if if v12 {
        let a1073 = v4;
        let a1074 = 3u64;
        nat_eq(a1073, a1074)
    } else {
        false
    } {
        let a1075 = v10;
        let a1076 = v11;
        nat_sub(a1075, a1076)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1077, _, _, _) => {
            h1077
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1078, _, _) => {
            h1078
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1079, _) => {
            h1079
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1080) => {
            h1080
        }
    })?, f37(v0, v3)?))
}

pub fn f40(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f58(v0, v2.clone(), 0u64);
    let v9: u64 = f58(v0, v2.clone(), 1u64);
    let v10: u64 = f58(v0, v2.clone(), 2u64);
    let v11: u64 = f58(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f59(v1.clone(), f60(v8, v3, match v1.clone() {
            Adt3::C0(h1081, _, _, _, _, _, _, _) => {
                h1081
            }
        })?, f60(v9, v4, match v1.clone() {
            Adt3::C0(_, h1082, _, _, _, _, _, _) => {
                h1082
            }
        })?, f60(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1083, _, _, _, _, _) => {
                h1083
            }
        })?, f60(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1084, _, _, _, _) => {
                h1084
            }
        })?)?, f37(v0, v2)?))
    } else {
        if if if if f61(v8, v3, match v1.clone() {
            Adt3::C0(h1085, _, _, _, _, _, _, _) => {
                h1085
            }
        })? {
            f61(v9, v4, match v1.clone() {
                Adt3::C0(_, h1086, _, _, _, _, _, _) => {
                    h1086
                }
            })?
        } else {
            false
        } {
            f61(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1087, _, _, _, _, _) => {
                    h1087
                }
            })?
        } else {
            false
        } {
            f61(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1088, _, _, _, _) => {
                    h1088
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f59(v1, f62(v8, v3)?, f62(v9, v4)?, f62(v10, v5)?, f62(v11, v6)?)?, f37(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f37(v0, v2)?))
        }
    }
}

pub fn f41(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 1u64);
    let v6: u64 = f58(v0, v2.clone(), 2u64);
    let v7: u64 = f58(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f59(v1.clone(), f63(v4, v3, match v1 {
        Adt3::C0(h1089, _, _, _, _, _, _, _) => {
            h1089
        }
    })?, v5, v6, v7)?, f37(v0, v2)?))
}

pub fn f42(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1090 = v0;
    let a1091 = v3;
    if nat_lt(a1090, a1091) {
        Ok(Adt5::C1(false, {
            let a1092 = v0;
            let a1093 = 4u64;
            nat_mul(a1092, a1093)?
        }, {
            let a1094 = v0;
            let a1095 = 4u64;
            nat_mul(a1094, a1095)?
        }))
    } else {
        let v6: u64 = {
            let a1096 = v0;
            let a1097 = v3;
            nat_sub(a1096, a1097)
        };
        let v7: u64 = f37(v6, v1)?;
        let v8: u64 = f64(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f43(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1098 = v0;
    let a1099 = v3;
    if nat_lt(a1098, a1099) {
        Ok(Adt5::C1(true, 0u64, f37(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1100 = v0;
            let a1101 = v3;
            nat_sub(a1100, a1101)
        };
        let v5: u64 = f38(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f37(v4, v1)?, v5))
    }
}

pub fn f44(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1103 = v1;
        let a1104 = match v3.clone() {
            Adt3::C0(h1102, _, _, _, _, _, _, _) => {
                h1102
            }
        };
        let a1105 = 0u64;
        nat_rem(a1103, a1104, a1105)
    };
    let v5: u64 = {
        let a1111 = {
            let a1107 = v1;
            let a1108 = match v3.clone() {
                Adt3::C0(h1106, _, _, _, _, _, _, _) => {
                    h1106
                }
            };
            let a1109 = 0u64;
            nat_quot(a1107, a1108, a1109)
        };
        let a1112 = match v3.clone() {
            Adt3::C0(_, h1110, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1113 = 0u64;
        nat_rem(a1111, a1112, a1113)
    };
    let v6: bool = f65(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f38(v1, v2, v3.clone())?
    } else {
        0u64
    }, f37(v1, v3)?))
}

pub fn f45(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f37(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f38(v0, v5, v2.clone())?
    }, f37(v0, v2)?))
}

pub fn f46(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1115 = v0;
        let a1116 = match v2.clone() {
            Adt3::C0(h1114, _, _, _, _, _, _, _) => {
                h1114
            }
        };
        let a1117 = 0u64;
        nat_rem(a1115, a1116, a1117)
    };
    let v7: u64 = {
        let a1119 = v0;
        let a1120 = match v2.clone() {
            Adt3::C0(h1118, _, _, _, _, _, _, _) => {
                h1118
            }
        };
        let a1121 = 0u64;
        nat_quot(a1119, a1120, a1121)
    };
    let v8: u64 = {
        let a1123 = v7;
        let a1124 = match v2.clone() {
            Adt3::C0(_, h1122, _, _, _, _, _, _) => {
                h1122
            }
        };
        let a1125 = 0u64;
        nat_rem(a1123, a1124, a1125)
    };
    let v9: u64 = {
        let a1127 = v7;
        let a1128 = match v2.clone() {
            Adt3::C0(_, h1126, _, _, _, _, _, _) => {
                h1126
            }
        };
        let a1129 = 0u64;
        nat_quot(a1127, a1128, a1129)
    };
    let v10: u64 = {
        let a1131 = v9;
        let a1132 = match v2.clone() {
            Adt3::C0(_, _, h1130, _, _, _, _, _) => {
                h1130
            }
        };
        let a1133 = 0u64;
        nat_rem(a1131, a1132, a1133)
    };
    let v11: u64 = {
        let a1135 = v9;
        let a1136 = match v2.clone() {
            Adt3::C0(_, _, h1134, _, _, _, _, _) => {
                h1134
            }
        };
        let a1137 = 0u64;
        nat_quot(a1135, a1136, a1137)
    };
    let v12: u64 = f37(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1146 = {
                let a1141 = {
                    let a1138 = v10;
                    let a1139 = v4;
                    let a1140 = 0u64;
                    nat_quot(a1138, a1139, a1140)
                };
                let a1142 = v5;
                nat_mul(a1141, a1142)?
            };
            let a1147 = {
                let a1143 = v8;
                let a1144 = v4;
                let a1145 = 0u64;
                nat_quot(a1143, a1144, a1145)
            };
            nat_add(a1146, a1147)?
        };
        let v14: u64 = {
            let a1167 = v6;
            let a1168 = {
                let a1165 = match v1.clone() {
                    Adt3::C0(h1148, _, _, _, _, _, _, _) => {
                        h1148
                    }
                };
                let a1166 = {
                    let a1163 = {
                        let a1149 = v8;
                        let a1150 = v4;
                        let a1151 = 0u64;
                        nat_rem(a1149, a1150, a1151)
                    };
                    let a1164 = {
                        let a1161 = match v1.clone() {
                            Adt3::C0(_, h1152, _, _, _, _, _, _) => {
                                h1152
                            }
                        };
                        let a1162 = {
                            let a1159 = {
                                let a1153 = v10;
                                let a1154 = v4;
                                let a1155 = 0u64;
                                nat_rem(a1153, a1154, a1155)
                            };
                            let a1160 = {
                                let a1157 = match v1.clone() {
                                    Adt3::C0(_, _, h1156, _, _, _, _, _) => {
                                        h1156
                                    }
                                };
                                let a1158 = v13;
                                nat_mul(a1157, a1158)?
                            };
                            nat_add(a1159, a1160)?
                        };
                        nat_mul(a1161, a1162)?
                    };
                    nat_add(a1163, a1164)?
                };
                nat_mul(a1165, a1166)?
            };
            nat_add(a1167, a1168)?
        };
        Ok(Adt5::C1(false, f37(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1174 = {
                let a1172 = {
                    let a1169 = v11;
                    let a1170 = v5;
                    let a1171 = 0u64;
                    nat_rem(a1169, a1170, a1171)
                };
                let a1173 = v4;
                nat_mul(a1172, a1173)?
            };
            let a1175 = v8;
            nat_add(a1174, a1175)?
        };
        let v16: u64 = {
            let a1181 = {
                let a1179 = {
                    let a1176 = v11;
                    let a1177 = v5;
                    let a1178 = 0u64;
                    nat_quot(a1176, a1177, a1178)
                };
                let a1180 = v4;
                nat_mul(a1179, a1180)?
            };
            let a1182 = v10;
            nat_add(a1181, a1182)?
        };
        let a1184 = v15;
        let a1185 = match v1.clone() {
            Adt3::C0(_, h1183, _, _, _, _, _, _) => {
                h1183
            }
        };
        if if nat_lt(a1184, a1185) {
            let a1187 = v16;
            let a1188 = match v1.clone() {
                Adt3::C0(_, _, h1186, _, _, _, _, _) => {
                    h1186
                }
            };
            nat_lt(a1187, a1188)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f37({
                let a1197 = v6;
                let a1198 = {
                    let a1195 = match v1.clone() {
                        Adt3::C0(h1189, _, _, _, _, _, _, _) => {
                            h1189
                        }
                    };
                    let a1196 = {
                        let a1193 = v15;
                        let a1194 = {
                            let a1191 = match v1.clone() {
                                Adt3::C0(_, h1190, _, _, _, _, _, _) => {
                                    h1190
                                }
                            };
                            let a1192 = v16;
                            nat_mul(a1191, a1192)?
                        };
                        nat_add(a1193, a1194)?
                    };
                    nat_mul(a1195, a1196)?
                };
                nat_add(a1197, a1198)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f47(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f37(v0, v2.clone())?;
    let v4: u64 = f37(v0, v1.clone())?;
    let a1203 = {
        let a1200 = v0;
        let a1201 = match v1 {
            Adt3::C0(h1199, _, _, _, _, _, _, _) => {
                h1199
            }
        };
        let a1202 = 0u64;
        nat_rem(a1200, a1201, a1202)
    };
    let a1204 = 0u64;
    if nat_eq(a1203, a1204) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C2(0u64, 2u64, f37({
            let a1205 = v0;
            let a1206 = 1u64;
            nat_sub(a1205, a1206)
        }, v2)?, 0u64, v4, v3))
    }
}

pub fn f48(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1207 = v3;
    let a1208 = 13u64;
    if nat_eq(a1207, a1208) {
        f66(v0, v1, v4)
    } else {
        let v5: u64 = f54(match v1.clone() {
            Adt3::C0(h1209, _, _, _, _, _, _, _) => {
                h1209
            }
        }, {
            let a1210 = v3;
            let a1211 = 16u64;
            nat_eq(a1210, a1211)
        })?;
        let v6: u64 = {
            let a1212 = v0;
            let a1213 = v5;
            let a1214 = 0u64;
            nat_quot(a1212, a1213, a1214)
        };
        let v7: u64 = {
            let a1215 = v0;
            let a1216 = v5;
            let a1217 = 0u64;
            nat_rem(a1215, a1216, a1217)
        };
        let v8: u64 = {
            let a1221 = 1u64;
            let a1222 = {
                let a1219 = match v1.clone() {
                    Adt3::C0(h1218, _, _, _, _, _, _, _) => {
                        h1218
                    }
                };
                let a1220 = 2u64;
                nat_mul(a1219, a1220)?
            };
            nat_add(a1221, a1222)?
        };
        let a1223 = v7;
        let a1224 = v8;
        if nat_lt(a1223, a1224) {
            f67(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f37(v6, v2)?;
            let a1225 = v3;
            let a1226 = 14u64;
            if nat_eq(a1225, a1226) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1227 = v7;
                let a1228 = v8;
                if nat_eq(a1227, a1228) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1231 = v7;
                    let a1232 = {
                        let a1229 = v8;
                        let a1230 = 1u64;
                        nat_add(a1229, a1230)?
                    };
                    if nat_eq(a1231, a1232) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1233, _, _, _, _, _, _, _) => {
                                h1233
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

pub fn f49(v0: u64) -> u64 {
    let a1234 = v0;
    let a1235 = 2147483648u64;
    if nat_lt(a1234, a1235) {
        v0
    } else {
        0u64
    }
}

pub fn f50(v0: u64) -> u64 {
    let a1236 = v0;
    let a1237 = 2147483648u64;
    if nat_lt(a1236, a1237) {
        0u64
    } else {
        let a1238 = 4294967296u64;
        let a1239 = v0;
        nat_sub(a1238, a1239)
    }
}

pub fn f51(v0: Adt3, v1: bool) -> R<u64> {
    f68(v0.clone(), {
        let a1241 = match v0.clone() {
            Adt3::C0(h1240, _, _, _, _, _, _, _) => {
                h1240
            }
        };
        let a1242 = 1u64;
        nat_sub(a1241, a1242)
    }, {
        let a1244 = match v0.clone() {
            Adt3::C0(_, h1243, _, _, _, _, _, _) => {
                h1243
            }
        };
        let a1245 = 1u64;
        nat_sub(a1244, a1245)
    }, {
        let a1247 = match v0.clone() {
            Adt3::C0(_, _, h1246, _, _, _, _, _) => {
                h1246
            }
        };
        let a1248 = 1u64;
        nat_sub(a1247, a1248)
    }, {
        let a1250 = match v0 {
            Adt3::C0(_, _, _, h1249, _, _, _, _) => {
                h1249
            }
        };
        let a1251 = 1u64;
        nat_sub(a1250, a1251)
    }, v1)
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1252 = v0;
    let a1253 = 0u64;
    if nat_eq(a1252, a1253) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1254 = v0;
            let a1255 = 1u64;
            nat_sub(a1254, a1255)
        };
        let v5: u64 = {
            let a1256 = v4;
            let a1257 = v1;
            let a1258 = 0u64;
            nat_quot(a1256, a1257, a1258)
        };
        let v6: u64 = {
            let a1266 = {
                let a1259 = v5;
                let a1260 = v2;
                nat_mul(a1259, a1260)?
            };
            let a1267 = {
                let a1264 = {
                    let a1261 = v4;
                    let a1262 = v1;
                    let a1263 = 0u64;
                    nat_rem(a1261, a1262, a1263)
                };
                let a1265 = v3;
                nat_mul(a1264, a1265)?
            };
            nat_add(a1266, a1267)?
        };
        let a1268 = v5;
        let a1269 = 0u64;
        if nat_eq(a1268, a1269) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1278 = {
                    let a1272 = {
                        let a1270 = v5;
                        let a1271 = 1u64;
                        nat_sub(a1270, a1271)
                    };
                    let a1273 = v2;
                    nat_mul(a1272, a1273)?
                };
                let a1279 = {
                    let a1276 = {
                        let a1274 = v1;
                        let a1275 = 1u64;
                        nat_sub(a1274, a1275)
                    };
                    let a1277 = v3;
                    nat_mul(a1276, a1277)?
                };
                nat_add(a1278, a1279)?
            };
            let a1280 = v6;
            let a1281 = v7;
            if nat_lt(a1280, a1281) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f53(v0: Adt3, v1: u64) -> bool {
    let a1282 = v1;
    let a1283 = 0u64;
    if !nat_eq(a1282, a1283) {
        true
    } else {
        let a1285 = match v0 {
            Adt3::C0(_, _, _, _, h1284, _, _, _) => {
                h1284
            }
        };
        let a1286 = 4u64;
        nat_eq(a1285, a1286)
    }
}

pub fn f54(v0: u64, v1: bool) -> R<u64> {
    let a1289 = {
        let a1287 = v0;
        let a1288 = 2u64;
        nat_mul(a1287, a1288)?
    };
    let a1290 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a1289, a1290)
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1291 = f69(v0, v1);
    match m1291 {
        None => {
            if v5 {
                let m1292 = f70(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1292 {
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
                let a1293 = 4u64;
                let a1294 = v28;
                nat_le(a1293, a1294)
            };
            let a1295 = v2;
            let a1296 = 0u64;
            if if if if if nat_eq(a1295, a1296) {
                let a1297 = v4;
                let a1298 = 0u64;
                nat_eq(a1297, a1298)
            } else {
                false
            } {
                let a1299 = v6;
                let a1300 = v14;
                if nat_eq(a1299, a1300) {
                    let a1301 = v7;
                    let a1302 = v15;
                    nat_eq(a1301, a1302)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1303 = v8;
                let a1304 = v16;
                if nat_eq(a1303, a1304) {
                    let a1305 = v9;
                    let a1306 = v17;
                    nat_eq(a1305, a1306)
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
                        let a1307 = v3;
                        let a1308 = 0u64;
                        nat_eq(a1307, a1308)
                    } else {
                        false
                    } {
                        f23(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1313 = {
                        let a1309 = v6;
                        let a1310 = v7;
                        nat_mul(a1309, a1310)?
                    };
                    let a1314 = {
                        let a1311 = v8;
                        let a1312 = v9;
                        nat_mul(a1311, a1312)?
                    };
                    nat_mul(a1313, a1314)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1327 = {
        let a1323 = {
            let a1319 = {
                let a1315 = v0;
                let a1316 = v4;
                nat_mul(a1315, a1316)?
            };
            let a1320 = {
                let a1317 = v1;
                let a1318 = v5;
                nat_mul(a1317, a1318)?
            };
            nat_add(a1319, a1320)?
        };
        let a1324 = {
            let a1321 = v2;
            let a1322 = v6;
            nat_mul(a1321, a1322)?
        };
        nat_add(a1323, a1324)?
    };
    let a1328 = {
        let a1325 = v3;
        let a1326 = v7;
        nat_mul(a1325, a1326)?
    };
    nat_add(a1327, a1328)
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f56({
        let a1329 = v0;
        let a1330 = v4;
        let a1331 = 0u64;
        nat_rem(a1329, a1330, a1331)
    }, {
        let a1332 = v1;
        let a1333 = v5;
        let a1334 = 0u64;
        nat_rem(a1332, a1333, a1334)
    }, {
        let a1335 = v2;
        let a1336 = v6;
        let a1337 = 0u64;
        nat_rem(a1335, a1336, a1337)
    }, {
        let a1338 = v3;
        let a1339 = v7;
        let a1340 = 0u64;
        nat_rem(a1338, a1339, a1340)
    }, v8, v9, v10, v11)
}

pub fn f58(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1342 = v0;
        let a1343 = match v1.clone() {
            Adt3::C0(h1341, _, _, _, _, _, _, _) => {
                h1341
            }
        };
        let a1344 = 0u64;
        nat_quot(a1342, a1343, a1344)
    };
    let v4: u64 = {
        let a1346 = v3;
        let a1347 = match v1.clone() {
            Adt3::C0(_, h1345, _, _, _, _, _, _) => {
                h1345
            }
        };
        let a1348 = 0u64;
        nat_quot(a1346, a1347, a1348)
    };
    let a1349 = v2;
    let a1350 = 0u64;
    if nat_eq(a1349, a1350) {
        let a1352 = v0;
        let a1353 = match v1 {
            Adt3::C0(h1351, _, _, _, _, _, _, _) => {
                h1351
            }
        };
        let a1354 = 0u64;
        nat_rem(a1352, a1353, a1354)
    } else {
        let a1355 = v2;
        let a1356 = 1u64;
        if nat_eq(a1355, a1356) {
            let a1358 = v3;
            let a1359 = match v1 {
                Adt3::C0(_, h1357, _, _, _, _, _, _) => {
                    h1357
                }
            };
            let a1360 = 0u64;
            nat_rem(a1358, a1359, a1360)
        } else {
            let a1361 = v2;
            let a1362 = 2u64;
            if nat_eq(a1361, a1362) {
                let a1364 = v4;
                let a1365 = match v1 {
                    Adt3::C0(_, _, h1363, _, _, _, _, _) => {
                        h1363
                    }
                };
                let a1366 = 0u64;
                nat_rem(a1364, a1365, a1366)
            } else {
                let a1368 = v4;
                let a1369 = match v1 {
                    Adt3::C0(_, _, h1367, _, _, _, _, _) => {
                        h1367
                    }
                };
                let a1370 = 0u64;
                nat_quot(a1368, a1369, a1370)
            }
        }
    }
}

pub fn f59(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f56(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1371, _, _, _) => {
            h1371
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1372, _, _) => {
            h1372
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1373, _) => {
            h1373
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1374) => {
            h1374
        }
    })
}

pub fn f60(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1375 = v1;
    let a1376 = 2147483648u64;
    if nat_lt(a1375, a1376) {
        let a1387 = {
            let a1385 = {
                let a1380 = {
                    let a1377 = v0;
                    let a1378 = v2;
                    let a1379 = 0u64;
                    nat_rem(a1377, a1378, a1379)
                };
                let a1381 = v2;
                nat_add(a1380, a1381)?
            };
            let a1386 = {
                let a1382 = v1;
                let a1383 = v2;
                let a1384 = 0u64;
                nat_rem(a1382, a1383, a1384)
            };
            nat_sub(a1385, a1386)
        };
        let a1388 = v2;
        let a1389 = 0u64;
        Ok(nat_rem(a1387, a1388, a1389))
    } else {
        let a1392 = {
            let a1390 = v0;
            let a1391 = f50(v1);
            nat_add(a1390, a1391)?
        };
        let a1393 = v2;
        let a1394 = 0u64;
        Ok(nat_rem(a1392, a1393, a1394))
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1397 = f49(v1);
    let a1398 = {
        let a1395 = v0;
        let a1396 = f50(v1);
        nat_add(a1395, a1396)?
    };
    if nat_le(a1397, a1398) {
        let a1399 = f62(v0, v1)?;
        let a1400 = v2;
        Ok(nat_lt(a1399, a1400))
    } else {
        Ok(false)
    }
}

pub fn f62(v0: u64, v1: u64) -> R<u64> {
    let a1403 = {
        let a1401 = v0;
        let a1402 = f50(v1);
        nat_add(a1401, a1402)?
    };
    let a1404 = f49(v1);
    Ok(nat_sub(a1403, a1404))
}

pub fn f63(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1405 = v0;
    let a1406 = v1;
    if nat_lt(a1405, a1406) {
        let a1407 = v1;
        let a1408 = v0;
        Ok(nat_sub(a1407, a1408))
    } else {
        let a1411 = v0;
        let a1412 = {
            let a1409 = v1;
            let a1410 = v2;
            nat_add(a1409, a1410)?
        };
        if nat_lt(a1411, a1412) {
            let a1413 = v0;
            let a1414 = v1;
            Ok(nat_sub(a1413, a1414))
        } else {
            let a1421 = {
                let a1417 = 2u64;
                let a1418 = {
                    let a1415 = v2;
                    let a1416 = 1u64;
                    nat_sub(a1415, a1416)
                };
                nat_mul(a1417, a1418)?
            };
            let a1422 = {
                let a1419 = v0;
                let a1420 = v1;
                nat_sub(a1419, a1420)
            };
            Ok(nat_sub(a1421, a1422))
        }
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1425 = {
        let a1423 = v2;
        let a1424 = f68(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a1423, a1424)?
    };
    let a1426 = f68(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1425, a1426))
}

pub fn f65(v0: u64, v1: u64, v2: u64) -> bool {
    let a1427 = v0;
    let a1428 = 0u64;
    if nat_eq(a1427, a1428) {
        let a1429 = v2;
        let a1430 = v1;
        nat_le(a1429, a1430)
    } else {
        let a1431 = v0;
        let a1432 = 1u64;
        if nat_eq(a1431, a1432) {
            let a1433 = v2;
            let a1434 = v1;
            nat_lt(a1433, a1434)
        } else {
            let a1435 = v0;
            let a1436 = 2u64;
            if nat_eq(a1435, a1436) {
                let a1437 = v1;
                let a1438 = v2;
                nat_le(a1437, a1438)
            } else {
                let a1439 = v0;
                let a1440 = 3u64;
                if nat_eq(a1439, a1440) {
                    let a1441 = v1;
                    let a1442 = v2;
                    nat_lt(a1441, a1442)
                } else {
                    let a1443 = v1;
                    let a1444 = v2;
                    nat_eq(a1443, a1444)
                }
            }
        }
    }
}

pub fn f66(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a1445 = v0;
    let a1446 = 0u64;
    if nat_eq(a1445, a1446) {
        Ok(f71(0u64))
    } else {
        let a1449 = {
            let a1447 = v0;
            let a1448 = 1u64;
            nat_add(a1447, a1448)?
        };
        let a1450 = f35(v1.clone(), 13u64, v2)?;
        if nat_eq(a1449, a1450) {
            Ok(Adt5::C4({
                let a1451 = v2;
                let a1452 = 0u64;
                if nat_eq(a1451, a1452) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a1453 = v0;
                let a1454 = 1u64;
                nat_sub(a1453, a1454)
            };
            if f53(v1.clone(), v2) {
                let v4: u64 = f54(match v1.clone() {
                    Adt3::C0(h1455, _, _, _, _, _, _, _) => {
                        h1455
                    }
                }, false)?;
                let v5: u64 = {
                    let a1456 = v3;
                    let a1457 = v4;
                    let a1458 = 0u64;
                    nat_quot(a1456, a1457, a1458)
                };
                let v6: u64 = {
                    let a1459 = v3;
                    let a1460 = v4;
                    let a1461 = 0u64;
                    nat_rem(a1459, a1460, a1461)
                };
                let a1464 = {
                    let a1462 = v6;
                    let a1463 = 1u64;
                    nat_add(a1462, a1463)?
                };
                let a1465 = v4;
                if nat_eq(a1464, a1465) {
                    Ok(Adt5::C4({
                        let a1466 = v2;
                        let a1467 = 0u64;
                        if nat_eq(a1466, a1467) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f67(v6, v5, v1, v2)
                }
            } else {
                f72({
                    let a1468 = v3;
                    let a1469 = 2u64;
                    let a1470 = 0u64;
                    nat_quot(a1468, a1469, a1470)
                }, {
                    let a1471 = v3;
                    let a1472 = 2u64;
                    let a1473 = 0u64;
                    nat_rem(a1471, a1472, a1473)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f67(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1474 = v0;
    let a1475 = 0u64;
    if nat_eq(a1474, a1475) {
        Ok(f71(1u64))
    } else {
        f72({
            let a1484 = {
                let a1477 = v1;
                let a1478 = match v2.clone() {
                    Adt3::C0(h1476, _, _, _, _, _, _, _) => {
                        h1476
                    }
                };
                nat_mul(a1477, a1478)?
            };
            let a1485 = {
                let a1481 = {
                    let a1479 = v0;
                    let a1480 = 1u64;
                    nat_sub(a1479, a1480)
                };
                let a1482 = 2u64;
                let a1483 = 0u64;
                nat_quot(a1481, a1482, a1483)
            };
            nat_add(a1484, a1485)?
        }, {
            let a1488 = {
                let a1486 = v0;
                let a1487 = 1u64;
                nat_sub(a1486, a1487)
            };
            let a1489 = 2u64;
            let a1490 = 0u64;
            nat_rem(a1488, a1489, a1490)
        }, v2, v3, 1u64)
    }
}

pub fn f68(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1511 = {
        let a1505 = {
            let a1499 = {
                let a1493 = v1;
                let a1494 = if v5 {
                    f50(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1491, _, _, _) => {
                            h1491
                        }
                    })
                } else {
                    f49(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1492, _, _, _) => {
                            h1492
                        }
                    })
                };
                nat_mul(a1493, a1494)?
            };
            let a1500 = {
                let a1497 = v2;
                let a1498 = if v5 {
                    f50(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1495, _, _) => {
                            h1495
                        }
                    })
                } else {
                    f49(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1496, _, _) => {
                            h1496
                        }
                    })
                };
                nat_mul(a1497, a1498)?
            };
            nat_add(a1499, a1500)?
        };
        let a1506 = {
            let a1503 = v3;
            let a1504 = if v5 {
                f50(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1501, _) => {
                        h1501
                    }
                })
            } else {
                f49(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1502, _) => {
                        h1502
                    }
                })
            };
            nat_mul(a1503, a1504)?
        };
        nat_add(a1505, a1506)?
    };
    let a1512 = {
        let a1509 = v4;
        let a1510 = if v5 {
            f50(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1507) => {
                    h1507
                }
            })
        } else {
            f49(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1508) => {
                    h1508
                }
            })
        };
        nat_mul(a1509, a1510)?
    };
    nat_add(a1511, a1512)
}

pub fn f69(v0: u64, v1: u64) -> Option<u64> {
    let a1513 = v0;
    let a1514 = 2u64;
    if nat_eq(a1513, a1514) {
        Some(0u64)
    } else {
        let a1515 = v0;
        let a1516 = 5u64;
        if nat_eq(a1515, a1516) {
            Some(1u64)
        } else {
            let a1517 = v0;
            let a1518 = 6u64;
            if nat_eq(a1517, a1518) {
                Some(2u64)
            } else {
                let a1519 = v0;
                let a1520 = 7u64;
                if nat_eq(a1519, a1520) {
                    Some(3u64)
                } else {
                    let a1521 = v0;
                    let a1522 = 9u64;
                    if nat_eq(a1521, a1522) {
                        Some(4u64)
                    } else {
                        let a1523 = v0;
                        let a1524 = 8u64;
                        if nat_eq(a1523, a1524) {
                            Some(5u64)
                        } else {
                            let a1525 = v0;
                            let a1526 = 89u64;
                            if nat_eq(a1525, a1526) {
                                let a1527 = v1;
                                let a1528 = 2u64;
                                if nat_eq(a1527, a1528) {
                                    Some(6u64)
                                } else {
                                    let a1529 = v1;
                                    let a1530 = 0u64;
                                    if nat_eq(a1529, a1530) {
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

pub fn f70(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1531 = v0;
    let a1532 = 28u64;
    if if if if nat_eq(a1531, a1532) {
        let a1533 = v1;
        let a1534 = 0u64;
        nat_eq(a1533, a1534)
    } else {
        false
    } {
        let a1535 = v2;
        let a1536 = 0u64;
        nat_eq(a1535, a1536)
    } else {
        false
    } {
        let a1537 = v3;
        let a1538 = 0u64;
        nat_eq(a1537, a1538)
    } else {
        false
    } {
        let a1539 = v4;
        let a1540 = v8;
        if if if if if if if if nat_eq(a1539, a1540) {
            let a1541 = v5;
            let a1542 = v12;
            nat_eq(a1541, a1542)
        } else {
            false
        } {
            let a1543 = v9;
            let a1544 = v13;
            nat_eq(a1543, a1544)
        } else {
            false
        } {
            let a1545 = v10;
            let a1546 = v14;
            nat_eq(a1545, a1546)
        } else {
            false
        } {
            let a1547 = v11;
            let a1548 = v15;
            nat_eq(a1547, a1548)
        } else {
            false
        } {
            let a1549 = 0u64;
            let a1550 = v6;
            nat_lt(a1549, a1550)
        } else {
            false
        } {
            let a1551 = 0u64;
            let a1552 = v7;
            nat_lt(a1551, a1552)
        } else {
            false
        } {
            let a1556 = {
                let a1553 = v10;
                let a1554 = v6;
                let a1555 = 1u64;
                nat_rem(a1553, a1554, a1555)
            };
            let a1557 = 0u64;
            if nat_eq(a1556, a1557) {
                let a1561 = {
                    let a1558 = v11;
                    let a1559 = v7;
                    let a1560 = 1u64;
                    nat_rem(a1558, a1559, a1560)
                };
                let a1562 = 0u64;
                nat_eq(a1561, a1562)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1563 = v10;
                let a1564 = v6;
                let a1565 = 0u64;
                nat_quot(a1563, a1564, a1565)
            }, {
                let a1566 = v11;
                let a1567 = v7;
                let a1568 = 0u64;
                nat_quot(a1566, a1567, a1568)
            }, {
                let a1569 = v5;
                let a1570 = v4;
                nat_mul(a1569, a1570)?
            }, {
                let a1571 = v4;
                let a1572 = v9;
                nat_mul(a1571, a1572)?
            }, {
                let a1573 = v5;
                let a1574 = v9;
                nat_mul(a1573, a1574)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f71(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f72(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1575 = v1;
    let a1576 = 0u64;
    if nat_eq(a1575, a1576) {
        Ok(Adt5::C4({
            let a1577 = v3;
            let a1578 = 0u64;
            if nat_eq(a1577, a1578) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f37(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a1579 = v3;
            let a1580 = 0u64;
            if nat_eq(a1579, a1580) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
