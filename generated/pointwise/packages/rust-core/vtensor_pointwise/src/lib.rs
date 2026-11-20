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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64),
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
                                f7(v1, v6, v3, v4, v5)?
                            }
                        }
                    }
                }
            }))
        }
        Adt0::C1(v8, v9, v10, v11) => {
            f8(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a18 = v0;
    let a19 = 1u64;
    if if nat_eq(a18, a19) {
        true
    } else {
        let a20 = v0;
        let a21 = 33u64;
        nat_eq(a20, a21)
    } {
        true
    } else {
        let a22 = v0;
        let a23 = 34u64;
        nat_eq(a22, a23)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a26 = match v1.clone() {
        Adt2::C0(h24, _, _, _, _, _, _, _, _, _, _) => {
            h24
        }
    };
    let a27 = match v3.clone() {
        Adt2::C0(h25, _, _, _, _, _, _, _, _, _, _) => {
            h25
        }
    };
    if if nat_eq(a26, a27) {
        let a28 = v0;
        let a29 = 33u64;
        if nat_eq(a28, a29) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m31 = f9(match v1.clone() {
            Adt2::C0(h30, _, _, _, _, _, _, _, _, _, _) => {
                h30
            }
        });
        match m31 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h32, _) => {
                        h32
                    }
                };
                let v6: u64 = f10(v1.clone(), v5)?;
                let a37 = {
                    let a34 = match v1.clone() {
                        Adt2::C0(_, h33, _, _, _, _, _, _, _, _, _) => {
                            h33
                        }
                    };
                    let a35 = v5;
                    let a36 = 1u64;
                    nat_rem(a34, a35, a36)
                };
                let a38 = 0u64;
                if if if nat_eq(a37, a38) {
                    let a43 = {
                        let a40 = match v3.clone() {
                            Adt2::C0(_, h39, _, _, _, _, _, _, _, _, _) => {
                                h39
                            }
                        };
                        let a41 = v5;
                        let a42 = 1u64;
                        nat_rem(a40, a41, a42)
                    };
                    let a44 = 0u64;
                    nat_eq(a43, a44)
                } else {
                    false
                } {
                    let a45 = v6;
                    let a46 = f10(v3.clone(), v5)?;
                    nat_eq(a45, a46)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h47) => {
                            h47
                        }
                    }, f11(v1, v5), f11(v3, v5), Adt4::C0))
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
    let a49 = match v0.clone() {
        Adt2::C0(h48, _, _, _, _, _, _, _, _, _, _) => {
            h48
        }
    };
    let a50 = 0u64;
    if if if if nat_eq(a49, a50) {
        true
    } else {
        let a52 = match v0.clone() {
            Adt2::C0(h51, _, _, _, _, _, _, _, _, _, _) => {
                h51
            }
        };
        let a53 = 1u64;
        nat_eq(a52, a53)
    } {
        true
    } else {
        let a55 = match v0.clone() {
            Adt2::C0(h54, _, _, _, _, _, _, _, _, _, _) => {
                h54
            }
        };
        let a56 = 25u64;
        if nat_eq(a55, a56) {
            true
        } else {
            let a58 = match v0.clone() {
                Adt2::C0(h57, _, _, _, _, _, _, _, _, _, _) => {
                    h57
                }
            };
            let a59 = 26u64;
            nat_eq(a58, a59)
        }
    } {
        true
    } else {
        let a61 = match v0.clone() {
            Adt2::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        };
        let a62 = 30u64;
        nat_eq(a61, a62)
    } {
        let a65 = match v0.clone() {
            Adt2::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        };
        let a66 = match v1.clone() {
            Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        if if nat_eq(a65, a66) {
            f12(match v1.clone() {
                Adt2::C0(_, h67, _, _, _, _, _, _, _, _, _) => {
                    h67
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h68, _, _, _, _, _, _, _, _) => {
                    h68
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h69, _, _, _, _, _, _, _) => {
                    h69
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h70, _, _, _, _, _, _) => {
                    h70
                }
            }, match v0.clone() {
                Adt2::C0(_, h71, _, _, _, _, _, _, _, _, _) => {
                    h71
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h72, _, _, _, _, _, _, _, _) => {
                    h72
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                    h73
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
                    h74
                }
            })
        } else {
            false
        } {
            let m76 = f9(match v0.clone() {
                Adt2::C0(h75, _, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            });
            match m76 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f10(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h77) => {
                            h77
                        }
                    }, f11(v0, 1u64), f11(v1, 1u64), Adt4::C1))
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
    let m78 = v2;
    match m78 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a79 = v0;
            let a80 = 4u64;
            if if if nat_lt(a79, a80) {
                let a83 = match v1.clone() {
                    Adt2::C0(h81, _, _, _, _, _, _, _, _, _, _) => {
                        h81
                    }
                };
                let a84 = match v4.clone() {
                    Adt2::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                        h82
                    }
                };
                if nat_eq(a83, a84) {
                    let a87 = match v1.clone() {
                        Adt2::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                            h85
                        }
                    };
                    let a88 = match v3.clone() {
                        Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                            h86
                        }
                    };
                    nat_eq(a87, a88)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f13(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h89, _, _, _, _, _, _, _, _, _) => {
                        h89
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h90, _, _, _, _, _, _, _, _, _) => {
                        h90
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h91, _, _, _, _, _, _, _, _, _) => {
                        h91
                    }
                })? {
                    f13(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h92, _, _, _, _, _, _, _, _) => {
                            h92
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h93, _, _, _, _, _, _, _, _) => {
                            h93
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                            h94
                        }
                    })?
                } else {
                    false
                } {
                    f13(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h95, _, _, _, _, _, _, _) => {
                            h95
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h96, _, _, _, _, _, _, _) => {
                            h96
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h97, _, _, _, _, _, _, _) => {
                            h97
                        }
                    })?
                } else {
                    false
                } {
                    f13(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h98, _, _, _, _, _, _) => {
                            h98
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h99, _, _, _, _, _, _) => {
                            h99
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                            h100
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m102 = f9(match v1.clone() {
                    Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                        h101
                    }
                });
                match m102 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h103, _) => {
                                h103
                            }
                        };
                        let a108 = {
                            let a105 = match v1.clone() {
                                Adt2::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                                    h104
                                }
                            };
                            let a106 = v6;
                            let a107 = 1u64;
                            nat_rem(a105, a106, a107)
                        };
                        let a109 = 0u64;
                        if if if nat_eq(a108, a109) {
                            let a114 = {
                                let a111 = match v4.clone() {
                                    Adt2::C0(_, h110, _, _, _, _, _, _, _, _, _) => {
                                        h110
                                    }
                                };
                                let a112 = v6;
                                let a113 = 1u64;
                                nat_rem(a111, a112, a113)
                            };
                            let a115 = 0u64;
                            nat_eq(a114, a115)
                        } else {
                            false
                        } {
                            let a120 = {
                                let a117 = match v3.clone() {
                                    Adt2::C0(_, h116, _, _, _, _, _, _, _, _, _) => {
                                        h116
                                    }
                                };
                                let a118 = v6;
                                let a119 = 1u64;
                                nat_rem(a117, a118, a119)
                            };
                            let a121 = 0u64;
                            nat_eq(a120, a121)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f10(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h122) => {
                                    h122
                                }
                            }, f11(v1, v6), f11(v3, v6), Adt4::C2(v0, f11(v4, v6))))
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
    let a124 = match v2.clone() {
        Adt2::C0(h123, _, _, _, _, _, _, _, _, _, _) => {
            h123
        }
    };
    let a125 = 0u64;
    if if nat_eq(a124, a125) {
        let a127 = match v3.clone() {
            Adt2::C0(h126, _, _, _, _, _, _, _, _, _, _) => {
                h126
            }
        };
        let a128 = 0u64;
        nat_eq(a127, a128)
    } else {
        false
    } {
        let a129 = v0;
        let a130 = 61u64;
        if nat_eq(a129, a130) {
            let v4: bool = {
                let a132 = 0u64;
                let a133 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h131, _, _, _, _, _, _, _) => {
                        h131
                    }
                };
                nat_lt(a132, a133)
            };
            if if if if if if f14(match v2.clone() {
                Adt2::C0(_, h134, _, _, _, _, _, _, _, _, _) => {
                    h134
                }
            }, match v1.clone() {
                Adt1::C0(h135, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h135
                }
            }, match v1.clone() {
                Adt1::C0(_, h136, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h136
                }
            }, match v3.clone() {
                Adt2::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
                    h137
                }
            })? {
                f14(match v2.clone() {
                    Adt2::C0(_, _, h138, _, _, _, _, _, _, _, _) => {
                        h138
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h139, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h139
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h140, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h140
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h141, _, _, _, _, _, _, _, _) => {
                        h141
                    }
                })?
            } else {
                false
            } {
                f14(match v2.clone() {
                    Adt2::C0(_, _, _, h142, _, _, _, _, _, _, _) => {
                        h142
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h143, _, _, _, _, _, _, _, _, _, _, _) => {
                        h143
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h144, _, _, _, _, _, _, _, _, _, _) => {
                        h144
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h145, _, _, _, _, _, _, _) => {
                        h145
                    }
                })?
            } else {
                false
            } {
                f14(match v2.clone() {
                    Adt2::C0(_, _, _, _, h146, _, _, _, _, _, _) => {
                        h146
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h147, _, _, _, _, _, _, _, _, _) => {
                        h147
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h148, _, _, _, _, _, _, _, _) => {
                        h148
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h149, _, _, _, _, _, _) => {
                        h149
                    }
                })?
            } else {
                false
            } {
                let a151 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h150, _, _, _, _, _, _, _) => {
                        h150
                    }
                };
                let a152 = 4294967295u64;
                nat_le(a151, a152)
            } else {
                false
            } {
                if v4 {
                    !f15(match v2.clone() {
                        Adt2::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                            h153
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h154, _, _, _, _, _, _, _, _) => {
                            h154
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h155, _, _, _, _, _, _, _) => {
                            h155
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h156, _, _, _, _, _, _) => {
                            h156
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f11(v2, 1u64), f11(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h157, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h158, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h158
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h159, _, _, _, _, _, _, _, _, _, _, _) => {
                        h159
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h160, _, _, _, _, _, _, _, _, _) => {
                        h160
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a161 = v0;
            let a162 = 62u64;
            if nat_eq(a161, a162) {
                let a164 = match v1.clone() {
                    Adt1::C0(h163, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                };
                let a165 = 2147483648u64;
                if if if if if nat_lt(a164, a165) {
                    let a167 = match v1.clone() {
                        Adt1::C0(_, h166, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h166
                        }
                    };
                    let a168 = 2147483648u64;
                    nat_lt(a167, a168)
                } else {
                    false
                } {
                    let a171 = match v1.clone() {
                        Adt1::C0(h169, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h169
                        }
                    };
                    let a172 = match v2.clone() {
                        Adt2::C0(_, h170, _, _, _, _, _, _, _, _, _) => {
                            h170
                        }
                    };
                    nat_lt(a171, a172)
                } else {
                    false
                } {
                    let a175 = match v1.clone() {
                        Adt1::C0(_, h173, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h173
                        }
                    };
                    let a176 = match v2.clone() {
                        Adt2::C0(_, h174, _, _, _, _, _, _, _, _, _) => {
                            h174
                        }
                    };
                    nat_lt(a175, a176)
                } else {
                    false
                } {
                    let a179 = match v2.clone() {
                        Adt2::C0(_, _, h177, _, _, _, _, _, _, _, _) => {
                            h177
                        }
                    };
                    let a180 = match v3.clone() {
                        Adt2::C0(_, _, h178, _, _, _, _, _, _, _, _) => {
                            h178
                        }
                    };
                    if if if nat_eq(a179, a180) {
                        let a183 = match v2.clone() {
                            Adt2::C0(_, _, _, h181, _, _, _, _, _, _, _) => {
                                h181
                            }
                        };
                        let a184 = match v3.clone() {
                            Adt2::C0(_, _, _, h182, _, _, _, _, _, _, _) => {
                                h182
                            }
                        };
                        nat_eq(a183, a184)
                    } else {
                        false
                    } {
                        let a187 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h185, _, _, _, _, _, _) => {
                                h185
                            }
                        };
                        let a188 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h186, _, _, _, _, _, _) => {
                                h186
                            }
                        };
                        nat_eq(a187, a188)
                    } else {
                        false
                    } {
                        let a197 = match v3.clone() {
                            Adt2::C0(_, h189, _, _, _, _, _, _, _, _, _) => {
                                h189
                            }
                        };
                        let a198 = {
                            let a195 = {
                                let a192 = match v2.clone() {
                                    Adt2::C0(_, h190, _, _, _, _, _, _, _, _, _) => {
                                        h190
                                    }
                                };
                                let a193 = match v1.clone() {
                                    Adt1::C0(h191, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h191
                                    }
                                };
                                nat_add(a192, a193)?
                            };
                            let a196 = match v1.clone() {
                                Adt1::C0(_, h194, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h194
                                }
                            };
                            nat_add(a195, a196)?
                        };
                        nat_eq(a197, a198)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f11(v2, 1u64), f11(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h199, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h199
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a200 = v0;
                let a201 = 63u64;
                if nat_eq(a200, a201) {
                    if if if if if f16(match v2.clone() {
                        Adt2::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                            h202
                        }
                    }, match v1.clone() {
                        Adt1::C0(h203, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h203
                        }
                    })? {
                        f16(match v2.clone() {
                            Adt2::C0(_, _, h204, _, _, _, _, _, _, _, _) => {
                                h204
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h205, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h205
                            }
                        })?
                    } else {
                        false
                    } {
                        f16(match v2.clone() {
                            Adt2::C0(_, _, _, h206, _, _, _, _, _, _, _) => {
                                h206
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h207, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h207
                            }
                        })?
                    } else {
                        false
                    } {
                        f16(match v2.clone() {
                            Adt2::C0(_, _, _, _, h208, _, _, _, _, _, _) => {
                                h208
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h209, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h209
                            }
                        })?
                    } else {
                        false
                    } {
                        let a212 = match v2.clone() {
                            Adt2::C0(_, h210, _, _, _, _, _, _, _, _, _) => {
                                h210
                            }
                        };
                        let a213 = match v3.clone() {
                            Adt2::C0(_, h211, _, _, _, _, _, _, _, _, _) => {
                                h211
                            }
                        };
                        if if if nat_eq(a212, a213) {
                            let a216 = match v2.clone() {
                                Adt2::C0(_, _, h214, _, _, _, _, _, _, _, _) => {
                                    h214
                                }
                            };
                            let a217 = match v3.clone() {
                                Adt2::C0(_, _, h215, _, _, _, _, _, _, _, _) => {
                                    h215
                                }
                            };
                            nat_eq(a216, a217)
                        } else {
                            false
                        } {
                            let a220 = match v2.clone() {
                                Adt2::C0(_, _, _, h218, _, _, _, _, _, _, _) => {
                                    h218
                                }
                            };
                            let a221 = match v3.clone() {
                                Adt2::C0(_, _, _, h219, _, _, _, _, _, _, _) => {
                                    h219
                                }
                            };
                            nat_eq(a220, a221)
                        } else {
                            false
                        } {
                            let a224 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h222, _, _, _, _, _, _) => {
                                    h222
                                }
                            };
                            let a225 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h223, _, _, _, _, _, _) => {
                                    h223
                                }
                            };
                            nat_eq(a224, a225)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f11(v2, 1u64), f11(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h226, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h226
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h227, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h227
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h228, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h228
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h229, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h229
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
    let m230 = v3;
    match m230 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a231 = v0;
                let a232 = 4u64;
                nat_eq(a231, a232)
            };
            let v7: bool = {
                let a234 = 0u64;
                let a235 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h233, _, _, _, _, _, _, _, _, _, _, _) => {
                        h233
                    }
                };
                nat_lt(a234, a235)
            };
            let v8: u64 = f10(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h236, _, _, _, _, _, _, _, _, _) => {
                    h236
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h237, _, _, _, _, _, _, _, _) => {
                    h237
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h238, _, _, _, _, _, _, _) => {
                    h238
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h239, _, _, _, _, _, _) => {
                    h239
                }
            }, 4u64, match v1.clone() {
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
            });
            if if if if if if if if if if if if v6 {
                let a244 = match v2.clone() {
                    Adt2::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                        h243
                    }
                };
                let a245 = 0u64;
                if nat_eq(a244, a245) {
                    let a247 = match v5.clone() {
                        Adt2::C0(h246, _, _, _, _, _, _, _, _, _, _) => {
                            h246
                        }
                    };
                    let a248 = 0u64;
                    nat_eq(a247, a248)
                } else {
                    false
                }
            } else {
                let a250 = match v2.clone() {
                    Adt2::C0(h249, _, _, _, _, _, _, _, _, _, _) => {
                        h249
                    }
                };
                let a251 = 0u64;
                if if nat_eq(a250, a251) {
                    true
                } else {
                    let a253 = match v2.clone() {
                        Adt2::C0(h252, _, _, _, _, _, _, _, _, _, _) => {
                            h252
                        }
                    };
                    let a254 = 26u64;
                    nat_eq(a253, a254)
                } {
                    let a256 = match v5.clone() {
                        Adt2::C0(h255, _, _, _, _, _, _, _, _, _, _) => {
                            h255
                        }
                    };
                    let a257 = 0u64;
                    if nat_eq(a256, a257) {
                        true
                    } else {
                        let a259 = match v5.clone() {
                            Adt2::C0(h258, _, _, _, _, _, _, _, _, _, _) => {
                                h258
                            }
                        };
                        let a260 = 26u64;
                        nat_eq(a259, a260)
                    }
                } else {
                    false
                }
            } {
                let a263 = match v2.clone() {
                    Adt2::C0(h261, _, _, _, _, _, _, _, _, _, _) => {
                        h261
                    }
                };
                let a264 = match v4.clone() {
                    Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                        h262
                    }
                };
                nat_eq(a263, a264)
            } else {
                false
            } {
                f17(v2.clone(), v4.clone())
            } else {
                false
            } {
                f18(v2.clone())?
            } else {
                false
            } {
                f18(v4.clone())?
            } else {
                false
            } {
                let a266 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h265, _, _, _, _, _) => {
                        h265
                    }
                };
                let a267 = 4u64;
                nat_eq(a266, a267)
            } else {
                false
            } {
                let a268 = v8;
                let a269 = f10(v2.clone(), 1u64)?;
                nat_le(a268, a269)
            } else {
                false
            } {
                let a271 = match v1.clone() {
                    Adt1::C0(_, _, _, h270, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h270
                    }
                };
                let a272 = 1073741824u64;
                nat_lt(a271, a272)
            } else {
                false
            } {
                let a274 = match v1.clone() {
                    Adt1::C0(h273, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h273
                    }
                };
                let a275 = 4294967295u64;
                if if if if nat_le(a274, a275) {
                    let a277 = match v1.clone() {
                        Adt1::C0(_, h276, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h276
                        }
                    };
                    let a278 = 4294967295u64;
                    nat_le(a277, a278)
                } else {
                    false
                } {
                    let a280 = match v1.clone() {
                        Adt1::C0(_, _, h279, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h279
                        }
                    };
                    let a281 = 4294967295u64;
                    nat_le(a280, a281)
                } else {
                    false
                } {
                    let a283 = match v1.clone() {
                        Adt1::C0(_, _, _, h282, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h282
                        }
                    };
                    let a284 = 4294967295u64;
                    nat_le(a283, a284)
                } else {
                    false
                } {
                    let a286 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h285, _, _, _, _, _, _, _, _, _, _, _) => {
                            h285
                        }
                    };
                    let a287 = 4294967295u64;
                    nat_le(a286, a287)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a290 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h288) => {
                            h288
                        }
                    };
                    let a291 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h289) => {
                            h289
                        }
                    };
                    nat_eq(a290, a291)
                } else {
                    true
                }
            } else {
                false
            } {
                f19(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h292, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h292
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h293, _) => {
                        h293
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f10(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a294 = v10;
                    let a295 = v8;
                    nat_add(a294, a295)?
                }, 4u64, f11(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h296, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h296
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m297 = v3;
    match m297 {
        None => {
            f20(v0, v1, match v2.clone() {
                Adt2::C0(h298, _, _, _, _, _, _, _, _, _, _) => {
                    h298
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h299, _, _, _, _, _, _, _, _, _, _) => {
                    h299
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h300, _, _, _, _, _, _, _, _, _) => {
                    h300
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h301, _, _, _, _, _, _, _, _) => {
                    h301
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h302, _, _, _, _, _, _, _) => {
                    h302
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h303, _, _, _, _, _, _) => {
                    h303
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h304, _, _, _, _, _, _, _, _, _) => {
                    h304
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h305, _, _, _, _, _, _, _, _) => {
                    h305
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h306, _, _, _, _, _, _, _) => {
                    h306
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h307, _, _, _, _, _, _) => {
                    h307
                }
            })
        }
        Some(v5) => {
            f20(v0, v1, match v2.clone() {
                Adt2::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                    h308
                }
            }, match v5.clone() {
                Adt2::C0(h309, _, _, _, _, _, _, _, _, _, _) => {
                    h309
                }
            }, match v4.clone() {
                Adt2::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
                    h310
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h311, _, _, _, _, _, _, _, _, _) => {
                    h311
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                    h312
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h313, _, _, _, _, _, _, _) => {
                    h313
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h314, _, _, _, _, _, _) => {
                    h314
                }
            }, match v5.clone() {
                Adt2::C0(_, h315, _, _, _, _, _, _, _, _, _) => {
                    h315
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h316, _, _, _, _, _, _, _, _) => {
                    h316
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h317, _, _, _, _, _, _, _) => {
                    h317
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h318, _, _, _, _, _, _) => {
                    h318
                }
            }, match v4.clone() {
                Adt2::C0(_, h319, _, _, _, _, _, _, _, _, _) => {
                    h319
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h320, _, _, _, _, _, _, _, _) => {
                    h320
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h321, _, _, _, _, _, _, _) => {
                    h321
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h322, _, _, _, _, _, _) => {
                    h322
                }
            })
        }
    }
}

pub fn f8(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m323 = v3;
    match m323 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f21(v0, v1)?, f21(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f22(v0, v1, v2.clone())?, f21(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f23(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f24(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f25(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f24(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f26(v0, v1, v2, v16, v17, v18)
        }
    }
}

pub fn f9(v0: u64) -> Adt7 {
    let a324 = v0;
    let a325 = 0u64;
    if nat_eq(a324, a325) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a326 = v0;
        let a327 = 1u64;
        if nat_eq(a326, a327) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a328 = v0;
            let a329 = 2u64;
            if nat_eq(a328, a329) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a330 = v0;
                let a331 = 3u64;
                if nat_eq(a330, a331) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a332 = v0;
                    let a333 = 6u64;
                    if nat_eq(a332, a333) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a334 = v0;
                        let a335 = 7u64;
                        if nat_eq(a334, a335) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a336 = v0;
                            let a337 = 8u64;
                            if nat_eq(a336, a337) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a338 = v0;
                                let a339 = 9u64;
                                if nat_eq(a338, a339) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a340 = v0;
                                    let a341 = 10u64;
                                    if nat_eq(a340, a341) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a342 = v0;
                                        let a343 = 11u64;
                                        if nat_eq(a342, a343) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a344 = v0;
                                            let a345 = 12u64;
                                            if nat_eq(a344, a345) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a346 = v0;
                                                let a347 = 13u64;
                                                if nat_eq(a346, a347) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a348 = v0;
                                                    let a349 = 14u64;
                                                    if nat_eq(a348, a349) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a350 = v0;
                                                        let a351 = 15u64;
                                                        if nat_eq(a350, a351) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a352 = v0;
                                                            let a353 = 16u64;
                                                            if nat_eq(a352, a353) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a354 = v0;
                                                                let a355 = 17u64;
                                                                if nat_eq(a354, a355) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a356 = v0;
                                                                    let a357 = 18u64;
                                                                    if nat_eq(a356, a357) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a358 = v0;
                                                                        let a359 = 19u64;
                                                                        if nat_eq(a358, a359) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a360 = v0;
                                                                            let a361 = 20u64;
                                                                            if nat_eq(a360, a361) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a362 = v0;
                                                                                let a363 = 21u64;
                                                                                if nat_eq(a362, a363) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a364 = v0;
                                                                                    let a365 = 22u64;
                                                                                    if nat_eq(a364, a365) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a366 = v0;
                                                                                        let a367 = 23u64;
                                                                                        if nat_eq(a366, a367) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a368 = v0;
                                                                                            let a369 = 24u64;
                                                                                            if nat_eq(a368, a369) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a370 = v0;
                                                                                                let a371 = 25u64;
                                                                                                if nat_eq(a370, a371) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a372 = v0;
                                                                                                    let a373 = 26u64;
                                                                                                    if nat_eq(a372, a373) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a374 = v0;
                                                                                                        let a375 = 27u64;
                                                                                                        if nat_eq(a374, a375) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a376 = v0;
                                                                                                            let a377 = 28u64;
                                                                                                            if nat_eq(a376, a377) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a378 = v0;
                                                                                                                let a379 = 29u64;
                                                                                                                if nat_eq(a378, a379) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a380 = v0;
                                                                                                                    let a381 = 30u64;
                                                                                                                    if nat_eq(a380, a381) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a382 = v0;
                                                                                                                        let a383 = 34u64;
                                                                                                                        if nat_eq(a382, a383) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a384 = v0;
                                                                                                                            let a385 = 35u64;
                                                                                                                            if nat_eq(a384, a385) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a386 = v0;
                                                                                                                                let a387 = 39u64;
                                                                                                                                if nat_eq(a386, a387) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a388 = v0;
                                                                                                                                    let a389 = 40u64;
                                                                                                                                    if nat_eq(a388, a389) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a390 = v0;
                                                                                                                                        let a391 = 41u64;
                                                                                                                                        if nat_eq(a390, a391) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a392 = v0;
                                                                                                                                            let a393 = 42u64;
                                                                                                                                            if nat_eq(a392, a393) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a394 = v0;
                                                                                                                                                let a395 = 43u64;
                                                                                                                                                if nat_eq(a394, a395) {
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

pub fn f10(v0: Adt2, v1: u64) -> R<u64> {
    let a407 = {
        let a401 = {
            let a397 = match v0.clone() {
                Adt2::C0(_, h396, _, _, _, _, _, _, _, _, _) => {
                    h396
                }
            };
            let a398 = v1;
            let a399 = 0u64;
            nat_quot(a397, a398, a399)
        };
        let a402 = match v0.clone() {
            Adt2::C0(_, _, h400, _, _, _, _, _, _, _, _) => {
                h400
            }
        };
        nat_mul(a401, a402)?
    };
    let a408 = {
        let a405 = match v0.clone() {
            Adt2::C0(_, _, _, h403, _, _, _, _, _, _, _) => {
                h403
            }
        };
        let a406 = match v0 {
            Adt2::C0(_, _, _, _, h404, _, _, _, _, _, _) => {
                h404
            }
        };
        nat_mul(a405, a406)?
    };
    nat_mul(a407, a408)
}

pub fn f11(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a410 = match v0.clone() {
            Adt2::C0(_, h409, _, _, _, _, _, _, _, _, _) => {
                h409
            }
        };
        let a411 = v1;
        let a412 = 0u64;
        nat_quot(a410, a411, a412)
    }, match v0.clone() {
        Adt2::C0(_, _, h413, _, _, _, _, _, _, _, _) => {
            h413
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h414, _, _, _, _, _, _, _) => {
            h414
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h415, _, _, _, _, _, _) => {
            h415
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h416, _, _, _, _, _) => {
            h416
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h417, _, _, _, _) => {
            h417
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h418, _, _, _) => {
            h418
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h419, _, _) => {
            h419
        }
    })
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f15(v4, v5, v6, v7) {
        f15(v0, v1, v2, v3)
    } else {
        let a423 = {
            let a420 = v0;
            let a421 = v4;
            let a422 = 1u64;
            nat_rem(a420, a421, a422)
        };
        let a424 = 0u64;
        if if if nat_eq(a423, a424) {
            let a428 = {
                let a425 = v1;
                let a426 = v5;
                let a427 = 1u64;
                nat_rem(a425, a426, a427)
            };
            let a429 = 0u64;
            nat_eq(a428, a429)
        } else {
            false
        } {
            let a433 = {
                let a430 = v2;
                let a431 = v6;
                let a432 = 1u64;
                nat_rem(a430, a431, a432)
            };
            let a434 = 0u64;
            nat_eq(a433, a434)
        } else {
            false
        } {
            let a438 = {
                let a435 = v3;
                let a436 = v7;
                let a437 = 1u64;
                nat_rem(a435, a436, a437)
            };
            let a439 = 0u64;
            nat_eq(a438, a439)
        } else {
            false
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a440 = v0;
    let a441 = v1;
    if nat_eq(a440, a441) {
        let a444 = v4;
        let a445 = {
            let a442 = v2;
            let a443 = v3;
            nat_add(a442, a443)?
        };
        Ok(nat_eq(a444, a445))
    } else {
        let a446 = v2;
        let a447 = v3;
        if nat_eq(a446, a447) {
            let a448 = v4;
            let a449 = v2;
            Ok(nat_eq(a448, a449))
        } else {
            Ok(false)
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a452 = {
            let a450 = v0;
            let a451 = f27(v1);
            nat_add(a450, a451)?
        };
        let a453 = f27(v2);
        nat_add(a452, a453)?
    };
    let v5: u64 = {
        let a454 = f28(v1);
        let a455 = f28(v2);
        nat_add(a454, a455)?
    };
    let a456 = v1;
    let a457 = 4294967295u64;
    if if nat_le(a456, a457) {
        let a458 = v2;
        let a459 = 4294967295u64;
        nat_le(a458, a459)
    } else {
        false
    } {
        let a460 = v5;
        let a461 = v4;
        if nat_le(a460, a461) {
            let a464 = v3;
            let a465 = {
                let a462 = v4;
                let a463 = v5;
                nat_sub(a462, a463)
            };
            Ok(nat_eq(a464, a465))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a466 = v0;
    let a467 = 0u64;
    if if if nat_eq(a466, a467) {
        true
    } else {
        let a468 = v1;
        let a469 = 0u64;
        nat_eq(a468, a469)
    } {
        true
    } else {
        let a470 = v2;
        let a471 = 0u64;
        nat_eq(a470, a471)
    } {
        true
    } else {
        let a472 = v3;
        let a473 = 0u64;
        nat_eq(a472, a473)
    }
}

pub fn f16(v0: u64, v1: u64) -> R<bool> {
    let a474 = v1;
    let a475 = 4294967295u64;
    if nat_le(a474, a475) {
        let a478 = {
            let a476 = f27(v1);
            let a477 = f28(v1);
            nat_add(a476, a477)?
        };
        let a479 = v0;
        Ok(nat_lt(a478, a479))
    } else {
        Ok(false)
    }
}

pub fn f17(v0: Adt2, v1: Adt2) -> bool {
    let a482 = match v0.clone() {
        Adt2::C0(_, h480, _, _, _, _, _, _, _, _, _) => {
            h480
        }
    };
    let a483 = match v1.clone() {
        Adt2::C0(_, h481, _, _, _, _, _, _, _, _, _) => {
            h481
        }
    };
    if if if nat_eq(a482, a483) {
        let a486 = match v0.clone() {
            Adt2::C0(_, _, h484, _, _, _, _, _, _, _, _) => {
                h484
            }
        };
        let a487 = match v1.clone() {
            Adt2::C0(_, _, h485, _, _, _, _, _, _, _, _) => {
                h485
            }
        };
        nat_eq(a486, a487)
    } else {
        false
    } {
        let a490 = match v0.clone() {
            Adt2::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
                h488
            }
        };
        let a491 = match v1.clone() {
            Adt2::C0(_, _, _, h489, _, _, _, _, _, _, _) => {
                h489
            }
        };
        nat_eq(a490, a491)
    } else {
        false
    } {
        let a494 = match v0 {
            Adt2::C0(_, _, _, _, h492, _, _, _, _, _, _) => {
                h492
            }
        };
        let a495 = match v1 {
            Adt2::C0(_, _, _, _, h493, _, _, _, _, _, _) => {
                h493
            }
        };
        nat_eq(a494, a495)
    } else {
        false
    }
}

pub fn f18(v0: Adt2) -> R<bool> {
    let a497 = match v0.clone() {
        Adt2::C0(_, h496, _, _, _, _, _, _, _, _, _) => {
            h496
        }
    };
    let a498 = 1u64;
    if if if if nat_eq(a497, a498) {
        true
    } else {
        let a500 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h499, _, _, _, _, _) => {
                h499
            }
        };
        let a501 = 4u64;
        nat_eq(a500, a501)
    } {
        let a503 = match v0.clone() {
            Adt2::C0(_, _, h502, _, _, _, _, _, _, _, _) => {
                h502
            }
        };
        let a504 = 1u64;
        if nat_eq(a503, a504) {
            true
        } else {
            let a509 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h505, _, _, _, _) => {
                    h505
                }
            };
            let a510 = {
                let a507 = match v0.clone() {
                    Adt2::C0(_, h506, _, _, _, _, _, _, _, _, _) => {
                        h506
                    }
                };
                let a508 = 4u64;
                nat_mul(a507, a508)?
            };
            nat_eq(a509, a510)
        }
    } else {
        false
    } {
        let a512 = match v0.clone() {
            Adt2::C0(_, _, _, h511, _, _, _, _, _, _, _) => {
                h511
            }
        };
        let a513 = 1u64;
        if nat_eq(a512, a513) {
            true
        } else {
            let a521 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h514, _, _, _) => {
                    h514
                }
            };
            let a522 = {
                let a519 = {
                    let a517 = match v0.clone() {
                        Adt2::C0(_, h515, _, _, _, _, _, _, _, _, _) => {
                            h515
                        }
                    };
                    let a518 = match v0.clone() {
                        Adt2::C0(_, _, h516, _, _, _, _, _, _, _, _) => {
                            h516
                        }
                    };
                    nat_mul(a517, a518)?
                };
                let a520 = 4u64;
                nat_mul(a519, a520)?
            };
            nat_eq(a521, a522)
        }
    } else {
        false
    } {
        let a524 = match v0.clone() {
            Adt2::C0(_, _, _, _, h523, _, _, _, _, _, _) => {
                h523
            }
        };
        let a525 = 1u64;
        if nat_eq(a524, a525) {
            Ok(true)
        } else {
            let a536 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h526, _, _) => {
                    h526
                }
            };
            let a537 = {
                let a534 = {
                    let a532 = {
                        let a529 = match v0.clone() {
                            Adt2::C0(_, h527, _, _, _, _, _, _, _, _, _) => {
                                h527
                            }
                        };
                        let a530 = match v0.clone() {
                            Adt2::C0(_, _, h528, _, _, _, _, _, _, _, _) => {
                                h528
                            }
                        };
                        nat_mul(a529, a530)?
                    };
                    let a533 = match v0 {
                        Adt2::C0(_, _, _, h531, _, _, _, _, _, _, _) => {
                            h531
                        }
                    };
                    nat_mul(a532, a533)?
                };
                let a535 = 4u64;
                nat_mul(a534, a535)?
            };
            Ok(nat_eq(a536, a537))
        }
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f29(v0.clone(), false)?;
    let v6: u64 = f29(v0, true)?;
    let a538 = v6;
    let a539 = v1;
    if if nat_le(a538, a539) {
        let a546 = {
            let a544 = {
                let a540 = v1;
                let a541 = v5;
                nat_add(a540, a541)?
            };
            let a545 = {
                let a542 = v3;
                let a543 = 0u64;
                if nat_eq(a542, a543) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a544, a545)?
        };
        let a547 = v2;
        nat_le(a546, a547)
    } else {
        false
    } {
        if v4 {
            let a552 = {
                let a550 = {
                    let a548 = v1;
                    let a549 = v5;
                    nat_add(a548, a549)?
                };
                let a551 = v6;
                nat_sub(a550, a551)
            };
            let a553 = v2;
            Ok(nat_lt(a552, a553))
        } else {
            let a558 = {
                let a556 = {
                    let a554 = v1;
                    let a555 = v5;
                    nat_add(a554, a555)?
                };
                let a557 = v6;
                nat_sub(a556, a557)
            };
            let a559 = v2;
            Ok(nat_le(a558, a559))
        }
    } else {
        Ok(false)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m560 = f30(v0, v1);
    match m560 {
        None => {
            if v5 {
                let m561 = f31(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m561 {
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
                let a562 = 4u64;
                let a563 = v28;
                nat_le(a562, a563)
            };
            let a564 = v2;
            let a565 = 0u64;
            if if if if if nat_eq(a564, a565) {
                let a566 = v4;
                let a567 = 0u64;
                nat_eq(a566, a567)
            } else {
                false
            } {
                let a568 = v6;
                let a569 = v14;
                if nat_eq(a568, a569) {
                    let a570 = v7;
                    let a571 = v15;
                    nat_eq(a570, a571)
                } else {
                    false
                }
            } else {
                false
            } {
                let a572 = v8;
                let a573 = v16;
                if nat_eq(a572, a573) {
                    let a574 = v9;
                    let a575 = v17;
                    nat_eq(a574, a575)
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
                        let a576 = v3;
                        let a577 = 0u64;
                        nat_eq(a576, a577)
                    } else {
                        false
                    } {
                        f12(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a582 = {
                        let a578 = v6;
                        let a579 = v7;
                        nat_mul(a578, a579)?
                    };
                    let a583 = {
                        let a580 = v8;
                        let a581 = v9;
                        nat_mul(a580, a581)?
                    };
                    nat_mul(a582, a583)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f21(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a585 = v0;
        let a586 = match v1.clone() {
            Adt3::C0(h584, _, _, _, _, _, _, _) => {
                h584
            }
        };
        let a587 = 0u64;
        nat_quot(a585, a586, a587)
    };
    let v3: u64 = {
        let a589 = v2;
        let a590 = match v1.clone() {
            Adt3::C0(_, h588, _, _, _, _, _, _) => {
                h588
            }
        };
        let a591 = 0u64;
        nat_quot(a589, a590, a591)
    };
    f32({
        let a593 = v0;
        let a594 = match v1.clone() {
            Adt3::C0(h592, _, _, _, _, _, _, _) => {
                h592
            }
        };
        let a595 = 0u64;
        nat_rem(a593, a594, a595)
    }, {
        let a597 = v2;
        let a598 = match v1.clone() {
            Adt3::C0(_, h596, _, _, _, _, _, _) => {
                h596
            }
        };
        let a599 = 0u64;
        nat_rem(a597, a598, a599)
    }, {
        let a601 = v3;
        let a602 = match v1.clone() {
            Adt3::C0(_, _, h600, _, _, _, _, _) => {
                h600
            }
        };
        let a603 = 0u64;
        nat_rem(a601, a602, a603)
    }, {
        let a605 = v3;
        let a606 = match v1.clone() {
            Adt3::C0(_, _, h604, _, _, _, _, _) => {
                h604
            }
        };
        let a607 = 0u64;
        nat_quot(a605, a606, a607)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h608, _, _, _) => {
            h608
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h609, _, _) => {
            h609
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h610, _) => {
            h610
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h611) => {
            h611
        }
    })
}

pub fn f22(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a613 = v0;
        let a614 = match v2.clone() {
            Adt3::C0(h612, _, _, _, _, _, _, _) => {
                h612
            }
        };
        let a615 = 0u64;
        nat_quot(a613, a614, a615)
    };
    let v4: u64 = {
        let a617 = v3;
        let a618 = match v2.clone() {
            Adt3::C0(_, h616, _, _, _, _, _, _) => {
                h616
            }
        };
        let a619 = 0u64;
        nat_quot(a617, a618, a619)
    };
    f33({
        let a621 = v0;
        let a622 = match v2.clone() {
            Adt3::C0(h620, _, _, _, _, _, _, _) => {
                h620
            }
        };
        let a623 = 0u64;
        nat_rem(a621, a622, a623)
    }, {
        let a625 = v3;
        let a626 = match v2.clone() {
            Adt3::C0(_, h624, _, _, _, _, _, _) => {
                h624
            }
        };
        let a627 = 0u64;
        nat_rem(a625, a626, a627)
    }, {
        let a629 = v4;
        let a630 = match v2.clone() {
            Adt3::C0(_, _, h628, _, _, _, _, _) => {
                h628
            }
        };
        let a631 = 0u64;
        nat_rem(a629, a630, a631)
    }, {
        let a633 = v4;
        let a634 = match v2 {
            Adt3::C0(_, _, h632, _, _, _, _, _) => {
                h632
            }
        };
        let a635 = 0u64;
        nat_quot(a633, a634, a635)
    }, match v1.clone() {
        Adt3::C0(h636, _, _, _, _, _, _, _) => {
            h636
        }
    }, match v1.clone() {
        Adt3::C0(_, h637, _, _, _, _, _, _) => {
            h637
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h638, _, _, _, _, _) => {
            h638
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h639, _, _, _, _) => {
            h639
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h640, _, _, _) => {
            h640
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h641, _, _) => {
            h641
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h642, _) => {
            h642
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h643) => {
            h643
        }
    })
}

pub fn f23(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a645 = v0;
        let a646 = match v3.clone() {
            Adt3::C0(h644, _, _, _, _, _, _, _) => {
                h644
            }
        };
        let a647 = 0u64;
        nat_quot(a645, a646, a647)
    };
    let v6: u64 = {
        let a649 = v5;
        let a650 = match v3.clone() {
            Adt3::C0(_, h648, _, _, _, _, _, _) => {
                h648
            }
        };
        let a651 = 0u64;
        nat_quot(a649, a650, a651)
    };
    let v7: u64 = {
        let a653 = v0;
        let a654 = match v3.clone() {
            Adt3::C0(h652, _, _, _, _, _, _, _) => {
                h652
            }
        };
        let a655 = 0u64;
        nat_rem(a653, a654, a655)
    };
    let v8: u64 = {
        let a657 = v5;
        let a658 = match v3.clone() {
            Adt3::C0(_, h656, _, _, _, _, _, _) => {
                h656
            }
        };
        let a659 = 0u64;
        nat_rem(a657, a658, a659)
    };
    let v9: u64 = {
        let a661 = v6;
        let a662 = match v3.clone() {
            Adt3::C0(_, _, h660, _, _, _, _, _) => {
                h660
            }
        };
        let a663 = 0u64;
        nat_rem(a661, a662, a663)
    };
    let v10: u64 = {
        let a665 = v6;
        let a666 = match v3.clone() {
            Adt3::C0(_, _, h664, _, _, _, _, _) => {
                h664
            }
        };
        let a667 = 0u64;
        nat_quot(a665, a666, a667)
    };
    let v11: u64 = {
        let a668 = v4;
        let a669 = 0u64;
        if nat_eq(a668, a669) {
            match v1.clone() {
                Adt3::C0(h670, _, _, _, _, _, _, _) => {
                    h670
                }
            }
        } else {
            let a671 = v4;
            let a672 = 1u64;
            if nat_eq(a671, a672) {
                match v1.clone() {
                    Adt3::C0(_, h673, _, _, _, _, _, _) => {
                        h673
                    }
                }
            } else {
                let a674 = v4;
                let a675 = 2u64;
                if nat_eq(a674, a675) {
                    match v1.clone() {
                        Adt3::C0(_, _, h676, _, _, _, _, _) => {
                            h676
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h677, _, _, _, _) => {
                            h677
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a684 = v11;
        let a685 = {
            let a678 = v4;
            let a679 = 0u64;
            if nat_eq(a678, a679) {
                v7
            } else {
                let a680 = v4;
                let a681 = 1u64;
                if nat_eq(a680, a681) {
                    v8
                } else {
                    let a682 = v4;
                    let a683 = 2u64;
                    if nat_eq(a682, a683) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a684, a685)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f32(if if v12 {
        let a686 = v4;
        let a687 = 0u64;
        nat_eq(a686, a687)
    } else {
        false
    } {
        let a688 = v7;
        let a689 = v11;
        nat_sub(a688, a689)
    } else {
        v7
    }, if if v12 {
        let a690 = v4;
        let a691 = 1u64;
        nat_eq(a690, a691)
    } else {
        false
    } {
        let a692 = v8;
        let a693 = v11;
        nat_sub(a692, a693)
    } else {
        v8
    }, if if v12 {
        let a694 = v4;
        let a695 = 2u64;
        nat_eq(a694, a695)
    } else {
        false
    } {
        let a696 = v9;
        let a697 = v11;
        nat_sub(a696, a697)
    } else {
        v9
    }, if if v12 {
        let a698 = v4;
        let a699 = 3u64;
        nat_eq(a698, a699)
    } else {
        false
    } {
        let a700 = v10;
        let a701 = v11;
        nat_sub(a700, a701)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h702, _, _, _) => {
            h702
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h703, _, _) => {
            h703
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h704, _) => {
            h704
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h705) => {
            h705
        }
    })?, f21(v0, v3)?))
}

pub fn f24(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f34(v0, v2.clone(), 0u64);
    let v9: u64 = f34(v0, v2.clone(), 1u64);
    let v10: u64 = f34(v0, v2.clone(), 2u64);
    let v11: u64 = f34(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f35(v1.clone(), f36(v8, v3, match v1.clone() {
            Adt3::C0(h706, _, _, _, _, _, _, _) => {
                h706
            }
        })?, f36(v9, v4, match v1.clone() {
            Adt3::C0(_, h707, _, _, _, _, _, _) => {
                h707
            }
        })?, f36(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h708, _, _, _, _, _) => {
                h708
            }
        })?, f36(v11, v6, match v1 {
            Adt3::C0(_, _, _, h709, _, _, _, _) => {
                h709
            }
        })?)?, f21(v0, v2)?))
    } else {
        if if if if f37(v8, v3, match v1.clone() {
            Adt3::C0(h710, _, _, _, _, _, _, _) => {
                h710
            }
        })? {
            f37(v9, v4, match v1.clone() {
                Adt3::C0(_, h711, _, _, _, _, _, _) => {
                    h711
                }
            })?
        } else {
            false
        } {
            f37(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h712, _, _, _, _, _) => {
                    h712
                }
            })?
        } else {
            false
        } {
            f37(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h713, _, _, _, _) => {
                    h713
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f35(v1, f38(v8, v3)?, f38(v9, v4)?, f38(v10, v5)?, f38(v11, v6)?)?, f21(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f21(v0, v2)?))
        }
    }
}

pub fn f25(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f34(v0, v2.clone(), 0u64);
    let v5: u64 = f34(v0, v2.clone(), 1u64);
    let v6: u64 = f34(v0, v2.clone(), 2u64);
    let v7: u64 = f34(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f35(v1.clone(), f39(v4, v3, match v1 {
        Adt3::C0(h714, _, _, _, _, _, _, _) => {
            h714
        }
    })?, v5, v6, v7)?, f21(v0, v2)?))
}

pub fn f26(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a715 = v0;
    let a716 = v3;
    if nat_lt(a715, a716) {
        Ok(Adt5::C1(false, {
            let a717 = v0;
            let a718 = 4u64;
            nat_mul(a717, a718)?
        }, {
            let a719 = v0;
            let a720 = 4u64;
            nat_mul(a719, a720)?
        }))
    } else {
        let v6: u64 = {
            let a721 = v0;
            let a722 = v3;
            nat_sub(a721, a722)
        };
        let v7: u64 = f21(v6, v1)?;
        let v8: u64 = f40(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f27(v0: u64) -> u64 {
    let a723 = v0;
    let a724 = 2147483648u64;
    if nat_lt(a723, a724) {
        v0
    } else {
        0u64
    }
}

pub fn f28(v0: u64) -> u64 {
    let a725 = v0;
    let a726 = 2147483648u64;
    if nat_lt(a725, a726) {
        0u64
    } else {
        let a727 = 4294967296u64;
        let a728 = v0;
        nat_sub(a727, a728)
    }
}

pub fn f29(v0: Adt3, v1: bool) -> R<u64> {
    f41(v0.clone(), {
        let a730 = match v0.clone() {
            Adt3::C0(h729, _, _, _, _, _, _, _) => {
                h729
            }
        };
        let a731 = 1u64;
        nat_sub(a730, a731)
    }, {
        let a733 = match v0.clone() {
            Adt3::C0(_, h732, _, _, _, _, _, _) => {
                h732
            }
        };
        let a734 = 1u64;
        nat_sub(a733, a734)
    }, {
        let a736 = match v0.clone() {
            Adt3::C0(_, _, h735, _, _, _, _, _) => {
                h735
            }
        };
        let a737 = 1u64;
        nat_sub(a736, a737)
    }, {
        let a739 = match v0 {
            Adt3::C0(_, _, _, h738, _, _, _, _) => {
                h738
            }
        };
        let a740 = 1u64;
        nat_sub(a739, a740)
    }, v1)
}

pub fn f30(v0: u64, v1: u64) -> Option<u64> {
    let a741 = v0;
    let a742 = 2u64;
    if nat_eq(a741, a742) {
        Some(0u64)
    } else {
        let a743 = v0;
        let a744 = 5u64;
        if nat_eq(a743, a744) {
            Some(1u64)
        } else {
            let a745 = v0;
            let a746 = 6u64;
            if nat_eq(a745, a746) {
                Some(2u64)
            } else {
                let a747 = v0;
                let a748 = 7u64;
                if nat_eq(a747, a748) {
                    Some(3u64)
                } else {
                    let a749 = v0;
                    let a750 = 9u64;
                    if nat_eq(a749, a750) {
                        Some(4u64)
                    } else {
                        let a751 = v0;
                        let a752 = 8u64;
                        if nat_eq(a751, a752) {
                            Some(5u64)
                        } else {
                            let a753 = v0;
                            let a754 = 89u64;
                            if nat_eq(a753, a754) {
                                let a755 = v1;
                                let a756 = 2u64;
                                if nat_eq(a755, a756) {
                                    Some(6u64)
                                } else {
                                    let a757 = v1;
                                    let a758 = 0u64;
                                    if nat_eq(a757, a758) {
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

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a759 = v0;
    let a760 = 28u64;
    if if if if nat_eq(a759, a760) {
        let a761 = v1;
        let a762 = 0u64;
        nat_eq(a761, a762)
    } else {
        false
    } {
        let a763 = v2;
        let a764 = 0u64;
        nat_eq(a763, a764)
    } else {
        false
    } {
        let a765 = v3;
        let a766 = 0u64;
        nat_eq(a765, a766)
    } else {
        false
    } {
        let a767 = v4;
        let a768 = v8;
        if if if if if if if if nat_eq(a767, a768) {
            let a769 = v5;
            let a770 = v12;
            nat_eq(a769, a770)
        } else {
            false
        } {
            let a771 = v9;
            let a772 = v13;
            nat_eq(a771, a772)
        } else {
            false
        } {
            let a773 = v10;
            let a774 = v14;
            nat_eq(a773, a774)
        } else {
            false
        } {
            let a775 = v11;
            let a776 = v15;
            nat_eq(a775, a776)
        } else {
            false
        } {
            let a777 = 0u64;
            let a778 = v6;
            nat_lt(a777, a778)
        } else {
            false
        } {
            let a779 = 0u64;
            let a780 = v7;
            nat_lt(a779, a780)
        } else {
            false
        } {
            let a784 = {
                let a781 = v10;
                let a782 = v6;
                let a783 = 1u64;
                nat_rem(a781, a782, a783)
            };
            let a785 = 0u64;
            if nat_eq(a784, a785) {
                let a789 = {
                    let a786 = v11;
                    let a787 = v7;
                    let a788 = 1u64;
                    nat_rem(a786, a787, a788)
                };
                let a790 = 0u64;
                nat_eq(a789, a790)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a791 = v10;
                let a792 = v6;
                let a793 = 0u64;
                nat_quot(a791, a792, a793)
            }, {
                let a794 = v11;
                let a795 = v7;
                let a796 = 0u64;
                nat_quot(a794, a795, a796)
            }, {
                let a797 = v5;
                let a798 = v4;
                nat_mul(a797, a798)?
            }, {
                let a799 = v4;
                let a800 = v9;
                nat_mul(a799, a800)?
            }, {
                let a801 = v5;
                let a802 = v9;
                nat_mul(a801, a802)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a815 = {
        let a811 = {
            let a807 = {
                let a803 = v0;
                let a804 = v4;
                nat_mul(a803, a804)?
            };
            let a808 = {
                let a805 = v1;
                let a806 = v5;
                nat_mul(a805, a806)?
            };
            nat_add(a807, a808)?
        };
        let a812 = {
            let a809 = v2;
            let a810 = v6;
            nat_mul(a809, a810)?
        };
        nat_add(a811, a812)?
    };
    let a816 = {
        let a813 = v3;
        let a814 = v7;
        nat_mul(a813, a814)?
    };
    nat_add(a815, a816)
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f32({
        let a817 = v0;
        let a818 = v4;
        let a819 = 0u64;
        nat_rem(a817, a818, a819)
    }, {
        let a820 = v1;
        let a821 = v5;
        let a822 = 0u64;
        nat_rem(a820, a821, a822)
    }, {
        let a823 = v2;
        let a824 = v6;
        let a825 = 0u64;
        nat_rem(a823, a824, a825)
    }, {
        let a826 = v3;
        let a827 = v7;
        let a828 = 0u64;
        nat_rem(a826, a827, a828)
    }, v8, v9, v10, v11)
}

pub fn f34(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a830 = v0;
        let a831 = match v1.clone() {
            Adt3::C0(h829, _, _, _, _, _, _, _) => {
                h829
            }
        };
        let a832 = 0u64;
        nat_quot(a830, a831, a832)
    };
    let v4: u64 = {
        let a834 = v3;
        let a835 = match v1.clone() {
            Adt3::C0(_, h833, _, _, _, _, _, _) => {
                h833
            }
        };
        let a836 = 0u64;
        nat_quot(a834, a835, a836)
    };
    let a837 = v2;
    let a838 = 0u64;
    if nat_eq(a837, a838) {
        let a840 = v0;
        let a841 = match v1 {
            Adt3::C0(h839, _, _, _, _, _, _, _) => {
                h839
            }
        };
        let a842 = 0u64;
        nat_rem(a840, a841, a842)
    } else {
        let a843 = v2;
        let a844 = 1u64;
        if nat_eq(a843, a844) {
            let a846 = v3;
            let a847 = match v1 {
                Adt3::C0(_, h845, _, _, _, _, _, _) => {
                    h845
                }
            };
            let a848 = 0u64;
            nat_rem(a846, a847, a848)
        } else {
            let a849 = v2;
            let a850 = 2u64;
            if nat_eq(a849, a850) {
                let a852 = v4;
                let a853 = match v1 {
                    Adt3::C0(_, _, h851, _, _, _, _, _) => {
                        h851
                    }
                };
                let a854 = 0u64;
                nat_rem(a852, a853, a854)
            } else {
                let a856 = v4;
                let a857 = match v1 {
                    Adt3::C0(_, _, h855, _, _, _, _, _) => {
                        h855
                    }
                };
                let a858 = 0u64;
                nat_quot(a856, a857, a858)
            }
        }
    }
}

pub fn f35(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f32(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h859, _, _, _) => {
            h859
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h860, _, _) => {
            h860
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h861, _) => {
            h861
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h862) => {
            h862
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a863 = v1;
    let a864 = 2147483648u64;
    if nat_lt(a863, a864) {
        let a875 = {
            let a873 = {
                let a868 = {
                    let a865 = v0;
                    let a866 = v2;
                    let a867 = 0u64;
                    nat_rem(a865, a866, a867)
                };
                let a869 = v2;
                nat_add(a868, a869)?
            };
            let a874 = {
                let a870 = v1;
                let a871 = v2;
                let a872 = 0u64;
                nat_rem(a870, a871, a872)
            };
            nat_sub(a873, a874)
        };
        let a876 = v2;
        let a877 = 0u64;
        Ok(nat_rem(a875, a876, a877))
    } else {
        let a880 = {
            let a878 = v0;
            let a879 = f28(v1);
            nat_add(a878, a879)?
        };
        let a881 = v2;
        let a882 = 0u64;
        Ok(nat_rem(a880, a881, a882))
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a885 = f27(v1);
    let a886 = {
        let a883 = v0;
        let a884 = f28(v1);
        nat_add(a883, a884)?
    };
    if nat_le(a885, a886) {
        let a887 = f38(v0, v1)?;
        let a888 = v2;
        Ok(nat_lt(a887, a888))
    } else {
        Ok(false)
    }
}

pub fn f38(v0: u64, v1: u64) -> R<u64> {
    let a891 = {
        let a889 = v0;
        let a890 = f28(v1);
        nat_add(a889, a890)?
    };
    let a892 = f27(v1);
    Ok(nat_sub(a891, a892))
}

pub fn f39(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a893 = v0;
    let a894 = v1;
    if nat_lt(a893, a894) {
        let a895 = v1;
        let a896 = v0;
        Ok(nat_sub(a895, a896))
    } else {
        let a899 = v0;
        let a900 = {
            let a897 = v1;
            let a898 = v2;
            nat_add(a897, a898)?
        };
        if nat_lt(a899, a900) {
            let a901 = v0;
            let a902 = v1;
            Ok(nat_sub(a901, a902))
        } else {
            let a909 = {
                let a905 = 2u64;
                let a906 = {
                    let a903 = v2;
                    let a904 = 1u64;
                    nat_sub(a903, a904)
                };
                nat_mul(a905, a906)?
            };
            let a910 = {
                let a907 = v0;
                let a908 = v1;
                nat_sub(a907, a908)
            };
            Ok(nat_sub(a909, a910))
        }
    }
}

pub fn f40(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a913 = {
        let a911 = v2;
        let a912 = f41(v1.clone(), f34(v0, v1.clone(), 0u64), f34(v0, v1.clone(), 1u64), f34(v0, v1.clone(), 2u64), f34(v0, v1.clone(), 3u64), false)?;
        nat_add(a911, a912)?
    };
    let a914 = f41(v1.clone(), f34(v0, v1.clone(), 0u64), f34(v0, v1.clone(), 1u64), f34(v0, v1.clone(), 2u64), f34(v0, v1, 3u64), true)?;
    Ok(nat_sub(a913, a914))
}

pub fn f41(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a935 = {
        let a929 = {
            let a923 = {
                let a917 = v1;
                let a918 = if v5 {
                    f28(match v0.clone() {
                        Adt3::C0(_, _, _, _, h915, _, _, _) => {
                            h915
                        }
                    })
                } else {
                    f27(match v0.clone() {
                        Adt3::C0(_, _, _, _, h916, _, _, _) => {
                            h916
                        }
                    })
                };
                nat_mul(a917, a918)?
            };
            let a924 = {
                let a921 = v2;
                let a922 = if v5 {
                    f28(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h919, _, _) => {
                            h919
                        }
                    })
                } else {
                    f27(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h920, _, _) => {
                            h920
                        }
                    })
                };
                nat_mul(a921, a922)?
            };
            nat_add(a923, a924)?
        };
        let a930 = {
            let a927 = v3;
            let a928 = if v5 {
                f28(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h925, _) => {
                        h925
                    }
                })
            } else {
                f27(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h926, _) => {
                        h926
                    }
                })
            };
            nat_mul(a927, a928)?
        };
        nat_add(a929, a930)?
    };
    let a936 = {
        let a933 = v4;
        let a934 = if v5 {
            f28(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h931) => {
                    h931
                }
            })
        } else {
            f27(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h932) => {
                    h932
                }
            })
        };
        nat_mul(a933, a934)?
    };
    nat_add(a935, a936)
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
