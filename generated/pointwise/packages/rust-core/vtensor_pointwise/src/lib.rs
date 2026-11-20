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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
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
                            f6(v1, v6, v3, v4, v5)?
                        }
                    }
                }
            }))
        }
        Adt0::C1(v8, v9, v10, v11) => {
            f7(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a14 = v0;
    let a15 = 1u64;
    if if nat_eq(a14, a15) {
        true
    } else {
        let a16 = v0;
        let a17 = 33u64;
        nat_eq(a16, a17)
    } {
        true
    } else {
        let a18 = v0;
        let a19 = 34u64;
        nat_eq(a18, a19)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a22 = match v1.clone() {
        Adt2::C0(h20, _, _, _, _, _, _, _, _, _) => {
            h20
        }
    };
    let a23 = match v3.clone() {
        Adt2::C0(h21, _, _, _, _, _, _, _, _, _) => {
            h21
        }
    };
    if if nat_eq(a22, a23) {
        let a24 = v0;
        let a25 = 33u64;
        if nat_eq(a24, a25) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m27 = f8(match v1.clone() {
            Adt2::C0(h26, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        });
        match m27 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h28, _) => {
                        h28
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a33 = {
                    let a30 = match v1.clone() {
                        Adt2::C0(_, h29, _, _, _, _, _, _, _, _) => {
                            h29
                        }
                    };
                    let a31 = v5;
                    let a32 = 1u64;
                    nat_rem(a30, a31, a32)
                };
                let a34 = 0u64;
                if if if nat_eq(a33, a34) {
                    let a39 = {
                        let a36 = match v3.clone() {
                            Adt2::C0(_, h35, _, _, _, _, _, _, _, _) => {
                                h35
                            }
                        };
                        let a37 = v5;
                        let a38 = 1u64;
                        nat_rem(a36, a37, a38)
                    };
                    let a40 = 0u64;
                    nat_eq(a39, a40)
                } else {
                    false
                } {
                    let a41 = v6;
                    let a42 = f9(v3.clone(), v5)?;
                    nat_eq(a41, a42)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h43) => {
                            h43
                        }
                    }, f10(v1, v5), f10(v3, v5), Adt4::C0))
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
    let a45 = match v0.clone() {
        Adt2::C0(h44, _, _, _, _, _, _, _, _, _) => {
            h44
        }
    };
    let a46 = 0u64;
    if if if if nat_eq(a45, a46) {
        true
    } else {
        let a48 = match v0.clone() {
            Adt2::C0(h47, _, _, _, _, _, _, _, _, _) => {
                h47
            }
        };
        let a49 = 1u64;
        nat_eq(a48, a49)
    } {
        true
    } else {
        let a51 = match v0.clone() {
            Adt2::C0(h50, _, _, _, _, _, _, _, _, _) => {
                h50
            }
        };
        let a52 = 25u64;
        if nat_eq(a51, a52) {
            true
        } else {
            let a54 = match v0.clone() {
                Adt2::C0(h53, _, _, _, _, _, _, _, _, _) => {
                    h53
                }
            };
            let a55 = 26u64;
            nat_eq(a54, a55)
        }
    } {
        true
    } else {
        let a57 = match v0.clone() {
            Adt2::C0(h56, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        };
        let a58 = 30u64;
        nat_eq(a57, a58)
    } {
        let a61 = match v0.clone() {
            Adt2::C0(h59, _, _, _, _, _, _, _, _, _) => {
                h59
            }
        };
        let a62 = match v1.clone() {
            Adt2::C0(h60, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        };
        if if nat_eq(a61, a62) {
            f11(match v1.clone() {
                Adt2::C0(_, h63, _, _, _, _, _, _, _, _) => {
                    h63
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h64, _, _, _, _, _, _, _) => {
                    h64
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h65, _, _, _, _, _, _) => {
                    h65
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h66, _, _, _, _, _) => {
                    h66
                }
            }, match v0.clone() {
                Adt2::C0(_, h67, _, _, _, _, _, _, _, _) => {
                    h67
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h68, _, _, _, _, _, _, _) => {
                    h68
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h69, _, _, _, _, _, _) => {
                    h69
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h70, _, _, _, _, _) => {
                    h70
                }
            })
        } else {
            false
        } {
            let m72 = f8(match v0.clone() {
                Adt2::C0(h71, _, _, _, _, _, _, _, _, _) => {
                    h71
                }
            });
            match m72 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f9(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h73) => {
                            h73
                        }
                    }, f10(v0, 1u64), f10(v1, 1u64), Adt4::C1))
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
    let m74 = v2;
    match m74 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a75 = v0;
            let a76 = 4u64;
            if if if nat_lt(a75, a76) {
                let a79 = match v1.clone() {
                    Adt2::C0(h77, _, _, _, _, _, _, _, _, _) => {
                        h77
                    }
                };
                let a80 = match v4.clone() {
                    Adt2::C0(h78, _, _, _, _, _, _, _, _, _) => {
                        h78
                    }
                };
                if nat_eq(a79, a80) {
                    let a83 = match v1.clone() {
                        Adt2::C0(h81, _, _, _, _, _, _, _, _, _) => {
                            h81
                        }
                    };
                    let a84 = match v3.clone() {
                        Adt2::C0(h82, _, _, _, _, _, _, _, _, _) => {
                            h82
                        }
                    };
                    nat_eq(a83, a84)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f12(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h85, _, _, _, _, _, _, _, _) => {
                        h85
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h86, _, _, _, _, _, _, _, _) => {
                        h86
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h87, _, _, _, _, _, _, _, _) => {
                        h87
                    }
                })? {
                    f12(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h88, _, _, _, _, _, _, _) => {
                            h88
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h89, _, _, _, _, _, _, _) => {
                            h89
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h90, _, _, _, _, _, _, _) => {
                            h90
                        }
                    })?
                } else {
                    false
                } {
                    f12(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h91, _, _, _, _, _, _) => {
                            h91
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h92, _, _, _, _, _, _) => {
                            h92
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h93, _, _, _, _, _, _) => {
                            h93
                        }
                    })?
                } else {
                    false
                } {
                    f12(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h94, _, _, _, _, _) => {
                            h94
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h95, _, _, _, _, _) => {
                            h95
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h96, _, _, _, _, _) => {
                            h96
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m98 = f8(match v1.clone() {
                    Adt2::C0(h97, _, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                });
                match m98 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h99, _) => {
                                h99
                            }
                        };
                        let a104 = {
                            let a101 = match v1.clone() {
                                Adt2::C0(_, h100, _, _, _, _, _, _, _, _) => {
                                    h100
                                }
                            };
                            let a102 = v6;
                            let a103 = 1u64;
                            nat_rem(a101, a102, a103)
                        };
                        let a105 = 0u64;
                        if if if nat_eq(a104, a105) {
                            let a110 = {
                                let a107 = match v4.clone() {
                                    Adt2::C0(_, h106, _, _, _, _, _, _, _, _) => {
                                        h106
                                    }
                                };
                                let a108 = v6;
                                let a109 = 1u64;
                                nat_rem(a107, a108, a109)
                            };
                            let a111 = 0u64;
                            nat_eq(a110, a111)
                        } else {
                            false
                        } {
                            let a116 = {
                                let a113 = match v3.clone() {
                                    Adt2::C0(_, h112, _, _, _, _, _, _, _, _) => {
                                        h112
                                    }
                                };
                                let a114 = v6;
                                let a115 = 1u64;
                                nat_rem(a113, a114, a115)
                            };
                            let a117 = 0u64;
                            nat_eq(a116, a117)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f9(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h118) => {
                                    h118
                                }
                            }, f10(v1, v6), f10(v3, v6), Adt4::C2(v0, f10(v4, v6))))
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
    let a120 = match v2.clone() {
        Adt2::C0(h119, _, _, _, _, _, _, _, _, _) => {
            h119
        }
    };
    let a121 = 0u64;
    if if nat_eq(a120, a121) {
        let a123 = match v3.clone() {
            Adt2::C0(h122, _, _, _, _, _, _, _, _, _) => {
                h122
            }
        };
        let a124 = 0u64;
        nat_eq(a123, a124)
    } else {
        false
    } {
        let a125 = v0;
        let a126 = 61u64;
        if nat_eq(a125, a126) {
            let v4: bool = {
                let a128 = 0u64;
                let a129 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h127, _, _, _, _, _, _, _) => {
                        h127
                    }
                };
                nat_lt(a128, a129)
            };
            if if if if if if f13(match v2.clone() {
                Adt2::C0(_, h130, _, _, _, _, _, _, _, _) => {
                    h130
                }
            }, match v1.clone() {
                Adt1::C0(h131, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h131
                }
            }, match v1.clone() {
                Adt1::C0(_, h132, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h132
                }
            }, match v3.clone() {
                Adt2::C0(_, h133, _, _, _, _, _, _, _, _) => {
                    h133
                }
            })? {
                f13(match v2.clone() {
                    Adt2::C0(_, _, h134, _, _, _, _, _, _, _) => {
                        h134
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h135, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h135
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h136, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h136
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h137, _, _, _, _, _, _, _) => {
                        h137
                    }
                })?
            } else {
                false
            } {
                f13(match v2.clone() {
                    Adt2::C0(_, _, _, h138, _, _, _, _, _, _) => {
                        h138
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h139, _, _, _, _, _, _, _, _, _, _, _) => {
                        h139
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h140, _, _, _, _, _, _, _, _, _, _) => {
                        h140
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h141, _, _, _, _, _, _) => {
                        h141
                    }
                })?
            } else {
                false
            } {
                f13(match v2.clone() {
                    Adt2::C0(_, _, _, _, h142, _, _, _, _, _) => {
                        h142
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h143, _, _, _, _, _, _, _, _, _) => {
                        h143
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h144, _, _, _, _, _, _, _, _) => {
                        h144
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h145, _, _, _, _, _) => {
                        h145
                    }
                })?
            } else {
                false
            } {
                let a147 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h146, _, _, _, _, _, _, _) => {
                        h146
                    }
                };
                let a148 = 4294967295u64;
                nat_le(a147, a148)
            } else {
                false
            } {
                if v4 {
                    !f14(match v2.clone() {
                        Adt2::C0(_, h149, _, _, _, _, _, _, _, _) => {
                            h149
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h150, _, _, _, _, _, _, _) => {
                            h150
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h151, _, _, _, _, _, _) => {
                            h151
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h152, _, _, _, _, _) => {
                            h152
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f10(v2, 1u64), f10(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h153, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h153
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h154, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h155, _, _, _, _, _, _, _, _, _, _, _) => {
                        h155
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h156, _, _, _, _, _, _, _, _, _) => {
                        h156
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a157 = v0;
            let a158 = 62u64;
            if nat_eq(a157, a158) {
                let a160 = match v1.clone() {
                    Adt1::C0(h159, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h159
                    }
                };
                let a161 = 2147483648u64;
                if if if if if nat_lt(a160, a161) {
                    let a163 = match v1.clone() {
                        Adt1::C0(_, h162, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h162
                        }
                    };
                    let a164 = 2147483648u64;
                    nat_lt(a163, a164)
                } else {
                    false
                } {
                    let a167 = match v1.clone() {
                        Adt1::C0(h165, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h165
                        }
                    };
                    let a168 = match v2.clone() {
                        Adt2::C0(_, h166, _, _, _, _, _, _, _, _) => {
                            h166
                        }
                    };
                    nat_lt(a167, a168)
                } else {
                    false
                } {
                    let a171 = match v1.clone() {
                        Adt1::C0(_, h169, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h169
                        }
                    };
                    let a172 = match v2.clone() {
                        Adt2::C0(_, h170, _, _, _, _, _, _, _, _) => {
                            h170
                        }
                    };
                    nat_lt(a171, a172)
                } else {
                    false
                } {
                    let a175 = match v2.clone() {
                        Adt2::C0(_, _, h173, _, _, _, _, _, _, _) => {
                            h173
                        }
                    };
                    let a176 = match v3.clone() {
                        Adt2::C0(_, _, h174, _, _, _, _, _, _, _) => {
                            h174
                        }
                    };
                    if if if nat_eq(a175, a176) {
                        let a179 = match v2.clone() {
                            Adt2::C0(_, _, _, h177, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        let a180 = match v3.clone() {
                            Adt2::C0(_, _, _, h178, _, _, _, _, _, _) => {
                                h178
                            }
                        };
                        nat_eq(a179, a180)
                    } else {
                        false
                    } {
                        let a183 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h181, _, _, _, _, _) => {
                                h181
                            }
                        };
                        let a184 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h182, _, _, _, _, _) => {
                                h182
                            }
                        };
                        nat_eq(a183, a184)
                    } else {
                        false
                    } {
                        let a193 = match v3.clone() {
                            Adt2::C0(_, h185, _, _, _, _, _, _, _, _) => {
                                h185
                            }
                        };
                        let a194 = {
                            let a191 = {
                                let a188 = match v2.clone() {
                                    Adt2::C0(_, h186, _, _, _, _, _, _, _, _) => {
                                        h186
                                    }
                                };
                                let a189 = match v1.clone() {
                                    Adt1::C0(h187, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h187
                                    }
                                };
                                nat_add(a188, a189)?
                            };
                            let a192 = match v1.clone() {
                                Adt1::C0(_, h190, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h190
                                }
                            };
                            nat_add(a191, a192)?
                        };
                        nat_eq(a193, a194)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f10(v2, 1u64), f10(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h195, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h195
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a196 = v0;
                let a197 = 63u64;
                if nat_eq(a196, a197) {
                    if if if if if f15(match v2.clone() {
                        Adt2::C0(_, h198, _, _, _, _, _, _, _, _) => {
                            h198
                        }
                    }, match v1.clone() {
                        Adt1::C0(h199, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h199
                        }
                    })? {
                        f15(match v2.clone() {
                            Adt2::C0(_, _, h200, _, _, _, _, _, _, _) => {
                                h200
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h201, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h201
                            }
                        })?
                    } else {
                        false
                    } {
                        f15(match v2.clone() {
                            Adt2::C0(_, _, _, h202, _, _, _, _, _, _) => {
                                h202
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h203, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h203
                            }
                        })?
                    } else {
                        false
                    } {
                        f15(match v2.clone() {
                            Adt2::C0(_, _, _, _, h204, _, _, _, _, _) => {
                                h204
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h205, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h205
                            }
                        })?
                    } else {
                        false
                    } {
                        let a208 = match v2.clone() {
                            Adt2::C0(_, h206, _, _, _, _, _, _, _, _) => {
                                h206
                            }
                        };
                        let a209 = match v3.clone() {
                            Adt2::C0(_, h207, _, _, _, _, _, _, _, _) => {
                                h207
                            }
                        };
                        if if if nat_eq(a208, a209) {
                            let a212 = match v2.clone() {
                                Adt2::C0(_, _, h210, _, _, _, _, _, _, _) => {
                                    h210
                                }
                            };
                            let a213 = match v3.clone() {
                                Adt2::C0(_, _, h211, _, _, _, _, _, _, _) => {
                                    h211
                                }
                            };
                            nat_eq(a212, a213)
                        } else {
                            false
                        } {
                            let a216 = match v2.clone() {
                                Adt2::C0(_, _, _, h214, _, _, _, _, _, _) => {
                                    h214
                                }
                            };
                            let a217 = match v3.clone() {
                                Adt2::C0(_, _, _, h215, _, _, _, _, _, _) => {
                                    h215
                                }
                            };
                            nat_eq(a216, a217)
                        } else {
                            false
                        } {
                            let a220 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h218, _, _, _, _, _) => {
                                    h218
                                }
                            };
                            let a221 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h219, _, _, _, _, _) => {
                                    h219
                                }
                            };
                            nat_eq(a220, a221)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f10(v2, 1u64), f10(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h222, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h222
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h223, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h223
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h224, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h224
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h225, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h225
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

pub fn f6(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m226 = v3;
    match m226 {
        None => {
            f16(v0, v1, match v2.clone() {
                Adt2::C0(h227, _, _, _, _, _, _, _, _, _) => {
                    h227
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h228, _, _, _, _, _, _, _, _, _) => {
                    h228
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h229, _, _, _, _, _, _, _, _) => {
                    h229
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h230, _, _, _, _, _, _, _) => {
                    h230
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h231, _, _, _, _, _, _) => {
                    h231
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h232, _, _, _, _, _) => {
                    h232
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h233, _, _, _, _, _, _, _, _) => {
                    h233
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h234, _, _, _, _, _, _, _) => {
                    h234
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h235, _, _, _, _, _, _) => {
                    h235
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h236, _, _, _, _, _) => {
                    h236
                }
            })
        }
        Some(v5) => {
            f16(v0, v1, match v2.clone() {
                Adt2::C0(h237, _, _, _, _, _, _, _, _, _) => {
                    h237
                }
            }, match v5.clone() {
                Adt2::C0(h238, _, _, _, _, _, _, _, _, _) => {
                    h238
                }
            }, match v4.clone() {
                Adt2::C0(h239, _, _, _, _, _, _, _, _, _) => {
                    h239
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h240, _, _, _, _, _, _, _, _) => {
                    h240
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h241, _, _, _, _, _, _, _) => {
                    h241
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h242, _, _, _, _, _, _) => {
                    h242
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h243, _, _, _, _, _) => {
                    h243
                }
            }, match v5.clone() {
                Adt2::C0(_, h244, _, _, _, _, _, _, _, _) => {
                    h244
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h245, _, _, _, _, _, _, _) => {
                    h245
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h246, _, _, _, _, _, _) => {
                    h246
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h247, _, _, _, _, _) => {
                    h247
                }
            }, match v4.clone() {
                Adt2::C0(_, h248, _, _, _, _, _, _, _, _) => {
                    h248
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h249, _, _, _, _, _, _, _) => {
                    h249
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h250, _, _, _, _, _, _) => {
                    h250
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h251, _, _, _, _, _) => {
                    h251
                }
            })
        }
    }
}

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m252 = v3;
    match m252 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f17(v0, v1)?, f17(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f18(v0, v1, v2.clone())?, f17(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f19(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f20(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f21(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f20(v0, v1, v2, v12, v13, v14, v15, true)
        }
    }
}

pub fn f8(v0: u64) -> Adt7 {
    let a253 = v0;
    let a254 = 0u64;
    if nat_eq(a253, a254) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a255 = v0;
        let a256 = 1u64;
        if nat_eq(a255, a256) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a257 = v0;
            let a258 = 2u64;
            if nat_eq(a257, a258) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a259 = v0;
                let a260 = 3u64;
                if nat_eq(a259, a260) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a261 = v0;
                    let a262 = 6u64;
                    if nat_eq(a261, a262) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a263 = v0;
                        let a264 = 7u64;
                        if nat_eq(a263, a264) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a265 = v0;
                            let a266 = 8u64;
                            if nat_eq(a265, a266) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a267 = v0;
                                let a268 = 9u64;
                                if nat_eq(a267, a268) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a269 = v0;
                                    let a270 = 10u64;
                                    if nat_eq(a269, a270) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a271 = v0;
                                        let a272 = 11u64;
                                        if nat_eq(a271, a272) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a273 = v0;
                                            let a274 = 12u64;
                                            if nat_eq(a273, a274) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a275 = v0;
                                                let a276 = 13u64;
                                                if nat_eq(a275, a276) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a277 = v0;
                                                    let a278 = 14u64;
                                                    if nat_eq(a277, a278) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a279 = v0;
                                                        let a280 = 15u64;
                                                        if nat_eq(a279, a280) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a281 = v0;
                                                            let a282 = 16u64;
                                                            if nat_eq(a281, a282) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a283 = v0;
                                                                let a284 = 17u64;
                                                                if nat_eq(a283, a284) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a285 = v0;
                                                                    let a286 = 18u64;
                                                                    if nat_eq(a285, a286) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a287 = v0;
                                                                        let a288 = 19u64;
                                                                        if nat_eq(a287, a288) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a289 = v0;
                                                                            let a290 = 20u64;
                                                                            if nat_eq(a289, a290) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a291 = v0;
                                                                                let a292 = 21u64;
                                                                                if nat_eq(a291, a292) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a293 = v0;
                                                                                    let a294 = 22u64;
                                                                                    if nat_eq(a293, a294) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a295 = v0;
                                                                                        let a296 = 23u64;
                                                                                        if nat_eq(a295, a296) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a297 = v0;
                                                                                            let a298 = 24u64;
                                                                                            if nat_eq(a297, a298) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a299 = v0;
                                                                                                let a300 = 25u64;
                                                                                                if nat_eq(a299, a300) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a301 = v0;
                                                                                                    let a302 = 26u64;
                                                                                                    if nat_eq(a301, a302) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a303 = v0;
                                                                                                        let a304 = 27u64;
                                                                                                        if nat_eq(a303, a304) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a305 = v0;
                                                                                                            let a306 = 28u64;
                                                                                                            if nat_eq(a305, a306) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a307 = v0;
                                                                                                                let a308 = 29u64;
                                                                                                                if nat_eq(a307, a308) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a309 = v0;
                                                                                                                    let a310 = 30u64;
                                                                                                                    if nat_eq(a309, a310) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a311 = v0;
                                                                                                                        let a312 = 34u64;
                                                                                                                        if nat_eq(a311, a312) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a313 = v0;
                                                                                                                            let a314 = 35u64;
                                                                                                                            if nat_eq(a313, a314) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a315 = v0;
                                                                                                                                let a316 = 39u64;
                                                                                                                                if nat_eq(a315, a316) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a317 = v0;
                                                                                                                                    let a318 = 40u64;
                                                                                                                                    if nat_eq(a317, a318) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a319 = v0;
                                                                                                                                        let a320 = 41u64;
                                                                                                                                        if nat_eq(a319, a320) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a321 = v0;
                                                                                                                                            let a322 = 42u64;
                                                                                                                                            if nat_eq(a321, a322) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a323 = v0;
                                                                                                                                                let a324 = 43u64;
                                                                                                                                                if nat_eq(a323, a324) {
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

pub fn f9(v0: Adt2, v1: u64) -> R<u64> {
    let a336 = {
        let a330 = {
            let a326 = match v0.clone() {
                Adt2::C0(_, h325, _, _, _, _, _, _, _, _) => {
                    h325
                }
            };
            let a327 = v1;
            let a328 = 0u64;
            nat_quot(a326, a327, a328)
        };
        let a331 = match v0.clone() {
            Adt2::C0(_, _, h329, _, _, _, _, _, _, _) => {
                h329
            }
        };
        nat_mul(a330, a331)?
    };
    let a337 = {
        let a334 = match v0.clone() {
            Adt2::C0(_, _, _, h332, _, _, _, _, _, _) => {
                h332
            }
        };
        let a335 = match v0 {
            Adt2::C0(_, _, _, _, h333, _, _, _, _, _) => {
                h333
            }
        };
        nat_mul(a334, a335)?
    };
    nat_mul(a336, a337)
}

pub fn f10(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a339 = match v0.clone() {
            Adt2::C0(_, h338, _, _, _, _, _, _, _, _) => {
                h338
            }
        };
        let a340 = v1;
        let a341 = 0u64;
        nat_quot(a339, a340, a341)
    }, match v0.clone() {
        Adt2::C0(_, _, h342, _, _, _, _, _, _, _) => {
            h342
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h343, _, _, _, _, _, _) => {
            h343
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h344, _, _, _, _, _) => {
            h344
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h345, _, _, _, _) => {
            h345
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h346, _, _, _) => {
            h346
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h347, _, _) => {
            h347
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h348, _) => {
            h348
        }
    })
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f14(v4, v5, v6, v7) {
        f14(v0, v1, v2, v3)
    } else {
        let a352 = {
            let a349 = v0;
            let a350 = v4;
            let a351 = 1u64;
            nat_rem(a349, a350, a351)
        };
        let a353 = 0u64;
        if if if nat_eq(a352, a353) {
            let a357 = {
                let a354 = v1;
                let a355 = v5;
                let a356 = 1u64;
                nat_rem(a354, a355, a356)
            };
            let a358 = 0u64;
            nat_eq(a357, a358)
        } else {
            false
        } {
            let a362 = {
                let a359 = v2;
                let a360 = v6;
                let a361 = 1u64;
                nat_rem(a359, a360, a361)
            };
            let a363 = 0u64;
            nat_eq(a362, a363)
        } else {
            false
        } {
            let a367 = {
                let a364 = v3;
                let a365 = v7;
                let a366 = 1u64;
                nat_rem(a364, a365, a366)
            };
            let a368 = 0u64;
            nat_eq(a367, a368)
        } else {
            false
        }
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a369 = v0;
    let a370 = v1;
    if nat_eq(a369, a370) {
        let a373 = v4;
        let a374 = {
            let a371 = v2;
            let a372 = v3;
            nat_add(a371, a372)?
        };
        Ok(nat_eq(a373, a374))
    } else {
        let a375 = v2;
        let a376 = v3;
        if nat_eq(a375, a376) {
            let a377 = v4;
            let a378 = v2;
            Ok(nat_eq(a377, a378))
        } else {
            Ok(false)
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a381 = {
            let a379 = v0;
            let a380 = f22(v1);
            nat_add(a379, a380)?
        };
        let a382 = f22(v2);
        nat_add(a381, a382)?
    };
    let v5: u64 = {
        let a383 = f23(v1);
        let a384 = f23(v2);
        nat_add(a383, a384)?
    };
    let a385 = v1;
    let a386 = 4294967295u64;
    if if nat_le(a385, a386) {
        let a387 = v2;
        let a388 = 4294967295u64;
        nat_le(a387, a388)
    } else {
        false
    } {
        let a389 = v5;
        let a390 = v4;
        if nat_le(a389, a390) {
            let a393 = v3;
            let a394 = {
                let a391 = v4;
                let a392 = v5;
                nat_sub(a391, a392)
            };
            Ok(nat_eq(a393, a394))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a395 = v0;
    let a396 = 0u64;
    if if if nat_eq(a395, a396) {
        true
    } else {
        let a397 = v1;
        let a398 = 0u64;
        nat_eq(a397, a398)
    } {
        true
    } else {
        let a399 = v2;
        let a400 = 0u64;
        nat_eq(a399, a400)
    } {
        true
    } else {
        let a401 = v3;
        let a402 = 0u64;
        nat_eq(a401, a402)
    }
}

pub fn f15(v0: u64, v1: u64) -> R<bool> {
    let a403 = v1;
    let a404 = 4294967295u64;
    if nat_le(a403, a404) {
        let a407 = {
            let a405 = f22(v1);
            let a406 = f23(v1);
            nat_add(a405, a406)?
        };
        let a408 = v0;
        Ok(nat_lt(a407, a408))
    } else {
        Ok(false)
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m409 = f24(v0, v1);
    match m409 {
        None => {
            if v5 {
                let m410 = f25(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m410 {
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
                let a411 = 4u64;
                let a412 = v28;
                nat_le(a411, a412)
            };
            let a413 = v2;
            let a414 = 0u64;
            if if if if if nat_eq(a413, a414) {
                let a415 = v4;
                let a416 = 0u64;
                nat_eq(a415, a416)
            } else {
                false
            } {
                let a417 = v6;
                let a418 = v14;
                if nat_eq(a417, a418) {
                    let a419 = v7;
                    let a420 = v15;
                    nat_eq(a419, a420)
                } else {
                    false
                }
            } else {
                false
            } {
                let a421 = v8;
                let a422 = v16;
                if nat_eq(a421, a422) {
                    let a423 = v9;
                    let a424 = v17;
                    nat_eq(a423, a424)
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
                        let a425 = v3;
                        let a426 = 0u64;
                        nat_eq(a425, a426)
                    } else {
                        false
                    } {
                        f11(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a431 = {
                        let a427 = v6;
                        let a428 = v7;
                        nat_mul(a427, a428)?
                    };
                    let a432 = {
                        let a429 = v8;
                        let a430 = v9;
                        nat_mul(a429, a430)?
                    };
                    nat_mul(a431, a432)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f17(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a434 = v0;
        let a435 = match v1.clone() {
            Adt3::C0(h433, _, _, _, _, _, _, _) => {
                h433
            }
        };
        let a436 = 0u64;
        nat_quot(a434, a435, a436)
    };
    let v3: u64 = {
        let a438 = v2;
        let a439 = match v1.clone() {
            Adt3::C0(_, h437, _, _, _, _, _, _) => {
                h437
            }
        };
        let a440 = 0u64;
        nat_quot(a438, a439, a440)
    };
    f26({
        let a442 = v0;
        let a443 = match v1.clone() {
            Adt3::C0(h441, _, _, _, _, _, _, _) => {
                h441
            }
        };
        let a444 = 0u64;
        nat_rem(a442, a443, a444)
    }, {
        let a446 = v2;
        let a447 = match v1.clone() {
            Adt3::C0(_, h445, _, _, _, _, _, _) => {
                h445
            }
        };
        let a448 = 0u64;
        nat_rem(a446, a447, a448)
    }, {
        let a450 = v3;
        let a451 = match v1.clone() {
            Adt3::C0(_, _, h449, _, _, _, _, _) => {
                h449
            }
        };
        let a452 = 0u64;
        nat_rem(a450, a451, a452)
    }, {
        let a454 = v3;
        let a455 = match v1.clone() {
            Adt3::C0(_, _, h453, _, _, _, _, _) => {
                h453
            }
        };
        let a456 = 0u64;
        nat_quot(a454, a455, a456)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h457, _, _, _) => {
            h457
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h458, _, _) => {
            h458
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h459, _) => {
            h459
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h460) => {
            h460
        }
    })
}

pub fn f18(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a462 = v0;
        let a463 = match v2.clone() {
            Adt3::C0(h461, _, _, _, _, _, _, _) => {
                h461
            }
        };
        let a464 = 0u64;
        nat_quot(a462, a463, a464)
    };
    let v4: u64 = {
        let a466 = v3;
        let a467 = match v2.clone() {
            Adt3::C0(_, h465, _, _, _, _, _, _) => {
                h465
            }
        };
        let a468 = 0u64;
        nat_quot(a466, a467, a468)
    };
    f27({
        let a470 = v0;
        let a471 = match v2.clone() {
            Adt3::C0(h469, _, _, _, _, _, _, _) => {
                h469
            }
        };
        let a472 = 0u64;
        nat_rem(a470, a471, a472)
    }, {
        let a474 = v3;
        let a475 = match v2.clone() {
            Adt3::C0(_, h473, _, _, _, _, _, _) => {
                h473
            }
        };
        let a476 = 0u64;
        nat_rem(a474, a475, a476)
    }, {
        let a478 = v4;
        let a479 = match v2.clone() {
            Adt3::C0(_, _, h477, _, _, _, _, _) => {
                h477
            }
        };
        let a480 = 0u64;
        nat_rem(a478, a479, a480)
    }, {
        let a482 = v4;
        let a483 = match v2 {
            Adt3::C0(_, _, h481, _, _, _, _, _) => {
                h481
            }
        };
        let a484 = 0u64;
        nat_quot(a482, a483, a484)
    }, match v1.clone() {
        Adt3::C0(h485, _, _, _, _, _, _, _) => {
            h485
        }
    }, match v1.clone() {
        Adt3::C0(_, h486, _, _, _, _, _, _) => {
            h486
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h487, _, _, _, _, _) => {
            h487
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h488, _, _, _, _) => {
            h488
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h489, _, _, _) => {
            h489
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h490, _, _) => {
            h490
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h491, _) => {
            h491
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h492) => {
            h492
        }
    })
}

pub fn f19(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a494 = v0;
        let a495 = match v3.clone() {
            Adt3::C0(h493, _, _, _, _, _, _, _) => {
                h493
            }
        };
        let a496 = 0u64;
        nat_quot(a494, a495, a496)
    };
    let v6: u64 = {
        let a498 = v5;
        let a499 = match v3.clone() {
            Adt3::C0(_, h497, _, _, _, _, _, _) => {
                h497
            }
        };
        let a500 = 0u64;
        nat_quot(a498, a499, a500)
    };
    let v7: u64 = {
        let a502 = v0;
        let a503 = match v3.clone() {
            Adt3::C0(h501, _, _, _, _, _, _, _) => {
                h501
            }
        };
        let a504 = 0u64;
        nat_rem(a502, a503, a504)
    };
    let v8: u64 = {
        let a506 = v5;
        let a507 = match v3.clone() {
            Adt3::C0(_, h505, _, _, _, _, _, _) => {
                h505
            }
        };
        let a508 = 0u64;
        nat_rem(a506, a507, a508)
    };
    let v9: u64 = {
        let a510 = v6;
        let a511 = match v3.clone() {
            Adt3::C0(_, _, h509, _, _, _, _, _) => {
                h509
            }
        };
        let a512 = 0u64;
        nat_rem(a510, a511, a512)
    };
    let v10: u64 = {
        let a514 = v6;
        let a515 = match v3.clone() {
            Adt3::C0(_, _, h513, _, _, _, _, _) => {
                h513
            }
        };
        let a516 = 0u64;
        nat_quot(a514, a515, a516)
    };
    let v11: u64 = {
        let a517 = v4;
        let a518 = 0u64;
        if nat_eq(a517, a518) {
            match v1.clone() {
                Adt3::C0(h519, _, _, _, _, _, _, _) => {
                    h519
                }
            }
        } else {
            let a520 = v4;
            let a521 = 1u64;
            if nat_eq(a520, a521) {
                match v1.clone() {
                    Adt3::C0(_, h522, _, _, _, _, _, _) => {
                        h522
                    }
                }
            } else {
                let a523 = v4;
                let a524 = 2u64;
                if nat_eq(a523, a524) {
                    match v1.clone() {
                        Adt3::C0(_, _, h525, _, _, _, _, _) => {
                            h525
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h526, _, _, _, _) => {
                            h526
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a533 = v11;
        let a534 = {
            let a527 = v4;
            let a528 = 0u64;
            if nat_eq(a527, a528) {
                v7
            } else {
                let a529 = v4;
                let a530 = 1u64;
                if nat_eq(a529, a530) {
                    v8
                } else {
                    let a531 = v4;
                    let a532 = 2u64;
                    if nat_eq(a531, a532) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a533, a534)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f26(if if v12 {
        let a535 = v4;
        let a536 = 0u64;
        nat_eq(a535, a536)
    } else {
        false
    } {
        let a537 = v7;
        let a538 = v11;
        nat_sub(a537, a538)
    } else {
        v7
    }, if if v12 {
        let a539 = v4;
        let a540 = 1u64;
        nat_eq(a539, a540)
    } else {
        false
    } {
        let a541 = v8;
        let a542 = v11;
        nat_sub(a541, a542)
    } else {
        v8
    }, if if v12 {
        let a543 = v4;
        let a544 = 2u64;
        nat_eq(a543, a544)
    } else {
        false
    } {
        let a545 = v9;
        let a546 = v11;
        nat_sub(a545, a546)
    } else {
        v9
    }, if if v12 {
        let a547 = v4;
        let a548 = 3u64;
        nat_eq(a547, a548)
    } else {
        false
    } {
        let a549 = v10;
        let a550 = v11;
        nat_sub(a549, a550)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h551, _, _, _) => {
            h551
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h552, _, _) => {
            h552
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h553, _) => {
            h553
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h554) => {
            h554
        }
    })?, f17(v0, v3)?))
}

pub fn f20(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f28(v0, v2.clone(), 0u64);
    let v9: u64 = f28(v0, v2.clone(), 1u64);
    let v10: u64 = f28(v0, v2.clone(), 2u64);
    let v11: u64 = f28(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f29(v1.clone(), f30(v8, v3, match v1.clone() {
            Adt3::C0(h555, _, _, _, _, _, _, _) => {
                h555
            }
        })?, f30(v9, v4, match v1.clone() {
            Adt3::C0(_, h556, _, _, _, _, _, _) => {
                h556
            }
        })?, f30(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h557, _, _, _, _, _) => {
                h557
            }
        })?, f30(v11, v6, match v1 {
            Adt3::C0(_, _, _, h558, _, _, _, _) => {
                h558
            }
        })?)?, f17(v0, v2)?))
    } else {
        if if if if f31(v8, v3, match v1.clone() {
            Adt3::C0(h559, _, _, _, _, _, _, _) => {
                h559
            }
        })? {
            f31(v9, v4, match v1.clone() {
                Adt3::C0(_, h560, _, _, _, _, _, _) => {
                    h560
                }
            })?
        } else {
            false
        } {
            f31(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h561, _, _, _, _, _) => {
                    h561
                }
            })?
        } else {
            false
        } {
            f31(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h562, _, _, _, _) => {
                    h562
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f29(v1, f32(v8, v3)?, f32(v9, v4)?, f32(v10, v5)?, f32(v11, v6)?)?, f17(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f17(v0, v2)?))
        }
    }
}

pub fn f21(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f28(v0, v2.clone(), 0u64);
    let v5: u64 = f28(v0, v2.clone(), 1u64);
    let v6: u64 = f28(v0, v2.clone(), 2u64);
    let v7: u64 = f28(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f29(v1.clone(), f33(v4, v3, match v1 {
        Adt3::C0(h563, _, _, _, _, _, _, _) => {
            h563
        }
    })?, v5, v6, v7)?, f17(v0, v2)?))
}

pub fn f22(v0: u64) -> u64 {
    let a564 = v0;
    let a565 = 2147483648u64;
    if nat_lt(a564, a565) {
        v0
    } else {
        0u64
    }
}

pub fn f23(v0: u64) -> u64 {
    let a566 = v0;
    let a567 = 2147483648u64;
    if nat_lt(a566, a567) {
        0u64
    } else {
        let a568 = 4294967296u64;
        let a569 = v0;
        nat_sub(a568, a569)
    }
}

pub fn f24(v0: u64, v1: u64) -> Option<u64> {
    let a570 = v0;
    let a571 = 2u64;
    if nat_eq(a570, a571) {
        Some(0u64)
    } else {
        let a572 = v0;
        let a573 = 5u64;
        if nat_eq(a572, a573) {
            Some(1u64)
        } else {
            let a574 = v0;
            let a575 = 6u64;
            if nat_eq(a574, a575) {
                Some(2u64)
            } else {
                let a576 = v0;
                let a577 = 7u64;
                if nat_eq(a576, a577) {
                    Some(3u64)
                } else {
                    let a578 = v0;
                    let a579 = 9u64;
                    if nat_eq(a578, a579) {
                        Some(4u64)
                    } else {
                        let a580 = v0;
                        let a581 = 8u64;
                        if nat_eq(a580, a581) {
                            Some(5u64)
                        } else {
                            let a582 = v0;
                            let a583 = 89u64;
                            if nat_eq(a582, a583) {
                                let a584 = v1;
                                let a585 = 2u64;
                                if nat_eq(a584, a585) {
                                    Some(6u64)
                                } else {
                                    let a586 = v1;
                                    let a587 = 0u64;
                                    if nat_eq(a586, a587) {
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

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a588 = v0;
    let a589 = 28u64;
    if if if if nat_eq(a588, a589) {
        let a590 = v1;
        let a591 = 0u64;
        nat_eq(a590, a591)
    } else {
        false
    } {
        let a592 = v2;
        let a593 = 0u64;
        nat_eq(a592, a593)
    } else {
        false
    } {
        let a594 = v3;
        let a595 = 0u64;
        nat_eq(a594, a595)
    } else {
        false
    } {
        let a596 = v4;
        let a597 = v8;
        if if if if if if if if nat_eq(a596, a597) {
            let a598 = v5;
            let a599 = v12;
            nat_eq(a598, a599)
        } else {
            false
        } {
            let a600 = v9;
            let a601 = v13;
            nat_eq(a600, a601)
        } else {
            false
        } {
            let a602 = v10;
            let a603 = v14;
            nat_eq(a602, a603)
        } else {
            false
        } {
            let a604 = v11;
            let a605 = v15;
            nat_eq(a604, a605)
        } else {
            false
        } {
            let a606 = 0u64;
            let a607 = v6;
            nat_lt(a606, a607)
        } else {
            false
        } {
            let a608 = 0u64;
            let a609 = v7;
            nat_lt(a608, a609)
        } else {
            false
        } {
            let a613 = {
                let a610 = v10;
                let a611 = v6;
                let a612 = 1u64;
                nat_rem(a610, a611, a612)
            };
            let a614 = 0u64;
            if nat_eq(a613, a614) {
                let a618 = {
                    let a615 = v11;
                    let a616 = v7;
                    let a617 = 1u64;
                    nat_rem(a615, a616, a617)
                };
                let a619 = 0u64;
                nat_eq(a618, a619)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a620 = v10;
                let a621 = v6;
                let a622 = 0u64;
                nat_quot(a620, a621, a622)
            }, {
                let a623 = v11;
                let a624 = v7;
                let a625 = 0u64;
                nat_quot(a623, a624, a625)
            }, {
                let a626 = v5;
                let a627 = v4;
                nat_mul(a626, a627)?
            }, {
                let a628 = v4;
                let a629 = v9;
                nat_mul(a628, a629)?
            }, {
                let a630 = v5;
                let a631 = v9;
                nat_mul(a630, a631)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a644 = {
        let a640 = {
            let a636 = {
                let a632 = v0;
                let a633 = v4;
                nat_mul(a632, a633)?
            };
            let a637 = {
                let a634 = v1;
                let a635 = v5;
                nat_mul(a634, a635)?
            };
            nat_add(a636, a637)?
        };
        let a641 = {
            let a638 = v2;
            let a639 = v6;
            nat_mul(a638, a639)?
        };
        nat_add(a640, a641)?
    };
    let a645 = {
        let a642 = v3;
        let a643 = v7;
        nat_mul(a642, a643)?
    };
    nat_add(a644, a645)
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f26({
        let a646 = v0;
        let a647 = v4;
        let a648 = 0u64;
        nat_rem(a646, a647, a648)
    }, {
        let a649 = v1;
        let a650 = v5;
        let a651 = 0u64;
        nat_rem(a649, a650, a651)
    }, {
        let a652 = v2;
        let a653 = v6;
        let a654 = 0u64;
        nat_rem(a652, a653, a654)
    }, {
        let a655 = v3;
        let a656 = v7;
        let a657 = 0u64;
        nat_rem(a655, a656, a657)
    }, v8, v9, v10, v11)
}

pub fn f28(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a659 = v0;
        let a660 = match v1.clone() {
            Adt3::C0(h658, _, _, _, _, _, _, _) => {
                h658
            }
        };
        let a661 = 0u64;
        nat_quot(a659, a660, a661)
    };
    let v4: u64 = {
        let a663 = v3;
        let a664 = match v1.clone() {
            Adt3::C0(_, h662, _, _, _, _, _, _) => {
                h662
            }
        };
        let a665 = 0u64;
        nat_quot(a663, a664, a665)
    };
    let a666 = v2;
    let a667 = 0u64;
    if nat_eq(a666, a667) {
        let a669 = v0;
        let a670 = match v1 {
            Adt3::C0(h668, _, _, _, _, _, _, _) => {
                h668
            }
        };
        let a671 = 0u64;
        nat_rem(a669, a670, a671)
    } else {
        let a672 = v2;
        let a673 = 1u64;
        if nat_eq(a672, a673) {
            let a675 = v3;
            let a676 = match v1 {
                Adt3::C0(_, h674, _, _, _, _, _, _) => {
                    h674
                }
            };
            let a677 = 0u64;
            nat_rem(a675, a676, a677)
        } else {
            let a678 = v2;
            let a679 = 2u64;
            if nat_eq(a678, a679) {
                let a681 = v4;
                let a682 = match v1 {
                    Adt3::C0(_, _, h680, _, _, _, _, _) => {
                        h680
                    }
                };
                let a683 = 0u64;
                nat_rem(a681, a682, a683)
            } else {
                let a685 = v4;
                let a686 = match v1 {
                    Adt3::C0(_, _, h684, _, _, _, _, _) => {
                        h684
                    }
                };
                let a687 = 0u64;
                nat_quot(a685, a686, a687)
            }
        }
    }
}

pub fn f29(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f26(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h688, _, _, _) => {
            h688
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h689, _, _) => {
            h689
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h690, _) => {
            h690
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h691) => {
            h691
        }
    })
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a692 = v1;
    let a693 = 2147483648u64;
    if nat_lt(a692, a693) {
        let a704 = {
            let a702 = {
                let a697 = {
                    let a694 = v0;
                    let a695 = v2;
                    let a696 = 0u64;
                    nat_rem(a694, a695, a696)
                };
                let a698 = v2;
                nat_add(a697, a698)?
            };
            let a703 = {
                let a699 = v1;
                let a700 = v2;
                let a701 = 0u64;
                nat_rem(a699, a700, a701)
            };
            nat_sub(a702, a703)
        };
        let a705 = v2;
        let a706 = 0u64;
        Ok(nat_rem(a704, a705, a706))
    } else {
        let a709 = {
            let a707 = v0;
            let a708 = f23(v1);
            nat_add(a707, a708)?
        };
        let a710 = v2;
        let a711 = 0u64;
        Ok(nat_rem(a709, a710, a711))
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a714 = f22(v1);
    let a715 = {
        let a712 = v0;
        let a713 = f23(v1);
        nat_add(a712, a713)?
    };
    if nat_le(a714, a715) {
        let a716 = f32(v0, v1)?;
        let a717 = v2;
        Ok(nat_lt(a716, a717))
    } else {
        Ok(false)
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a720 = {
        let a718 = v0;
        let a719 = f23(v1);
        nat_add(a718, a719)?
    };
    let a721 = f22(v1);
    Ok(nat_sub(a720, a721))
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a722 = v0;
    let a723 = v1;
    if nat_lt(a722, a723) {
        let a724 = v1;
        let a725 = v0;
        Ok(nat_sub(a724, a725))
    } else {
        let a728 = v0;
        let a729 = {
            let a726 = v1;
            let a727 = v2;
            nat_add(a726, a727)?
        };
        if nat_lt(a728, a729) {
            let a730 = v0;
            let a731 = v1;
            Ok(nat_sub(a730, a731))
        } else {
            let a738 = {
                let a734 = 2u64;
                let a735 = {
                    let a732 = v2;
                    let a733 = 1u64;
                    nat_sub(a732, a733)
                };
                nat_mul(a734, a735)?
            };
            let a739 = {
                let a736 = v0;
                let a737 = v1;
                nat_sub(a736, a737)
            };
            Ok(nat_sub(a738, a739))
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
