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
    C0(u64, bool, u64, u64, u64, Adt1, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Adt1),
    C1(Adt2, u64, u64, u64),
    C2(u64, bool, u64, u64, Adt1, Option<Adt1>, Option<Adt1>, Adt1),
    C3(Adt3, u64, u64, u64),
    C4(u64, bool, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Option<Adt1>, Adt1),
    C5(Adt4, u64, u64, u64),
    C6(Adt4, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt1, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Adt1, Option<u64>, Option<u64>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt1, Option<Adt1>, Adt1, u64, Option<u64>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt1, Adt1, Option<Adt1>, Option<Adt1>, Option<Adt1>, Adt1, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Adt2),
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C3(Adt3),
    C4(u64, u64, u64, u64),
    C5(Adt4),
    C6(u64, u64, u64, u64, u64),
    C7(u64),
    C8,
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

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            f1(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
        }
        Adt0::C1(v40, v41, v42, v43) => {
            f7(v40, v41, v42, v43)
        }
        Adt0::C2(v28, v29, v30, v31, v32, v33, v34, v35) => {
            f5(v28, v29, v30, v31, v32, v33, v34, v35)
        }
        Adt0::C3(v36, v37, v38, v39) => {
            f6(v36, v37, v38, v39)
        }
        Adt0::C4(v12, v13, v14, v15, v16, v17, v18, v19) => {
            f2(v12, v13, v14, v15, v16, v17, v18, v19)
        }
        Adt0::C5(v20, v21, v22, v23) => {
            f3(v20, v21, v22, v23)
        }
        Adt0::C6(v24, v25, v26, v27) => {
            f4(v24, v25, v26, v27)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Adt1) -> R<Adt5> {
    let a2 = v0;
    let a3 = 71u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a2, a3) {
        !v1
    } else {
        false
    } {
        f8(v2, match v5.clone() {
            Adt1::C0(_, _, _, h4, _, _, _, _, _, _, _) => {
                h4
            }
        }, v8.clone())
    } else {
        false
    } {
        let a5 = v4;
        let a6 = 0u64;
        if if nat_eq(a5, a6) {
            true
        } else {
            let a7 = v4;
            let a8 = 10u64;
            nat_eq(a7, a8)
        } {
            true
        } else {
            let a9 = v4;
            let a10 = 15u64;
            nat_eq(a9, a10)
        }
    } else {
        false
    } {
        let a12 = match v5.clone() {
            Adt1::C0(h11, _, _, _, _, _, _, _, _, _, _) => {
                h11
            }
        };
        let a13 = 0u64;
        nat_eq(a12, a13)
    } else {
        false
    } {
        let a15 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, h14, _, _, _, _, _) => {
                h14
            }
        };
        let a16 = 4u64;
        nat_eq(a15, a16)
    } else {
        false
    } {
        f9(v10.clone())?
    } else {
        false
    } {
        f10(v6.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        f10(v7.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        let a19 = match v5.clone() {
            Adt1::C0(_, h17, _, _, _, _, _, _, _, _, _) => {
                h17
            }
        };
        let a20 = match v6.clone() {
            Adt1::C0(_, h18, _, _, _, _, _, _, _, _, _) => {
                h18
            }
        };
        nat_eq(a19, a20)
    } else {
        false
    } {
        let a23 = match v6.clone() {
            Adt1::C0(_, _, h21, _, _, _, _, _, _, _, _) => {
                h21
            }
        };
        let a24 = match v7.clone() {
            Adt1::C0(_, _, h22, _, _, _, _, _, _, _, _) => {
                h22
            }
        };
        nat_eq(a23, a24)
    } else {
        false
    } {
        let a27 = match v10.clone() {
            Adt1::C0(_, h25, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        };
        let a28 = match v7.clone() {
            Adt1::C0(_, h26, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        };
        nat_eq(a27, a28)
    } else {
        false
    } {
        let a31 = match v10.clone() {
            Adt1::C0(_, _, h29, _, _, _, _, _, _, _, _) => {
                h29
            }
        };
        let a32 = match v5.clone() {
            Adt1::C0(_, _, _, h30, _, _, _, _, _, _, _) => {
                h30
            }
        };
        nat_eq(a31, a32)
    } else {
        false
    } {
        let a35 = match v10.clone() {
            Adt1::C0(_, _, _, h33, _, _, _, _, _, _, _) => {
                h33
            }
        };
        let a36 = match v5.clone() {
            Adt1::C0(_, _, h34, _, _, _, _, _, _, _, _) => {
                h34
            }
        };
        nat_eq(a35, a36)
    } else {
        false
    } {
        let a39 = match v10.clone() {
            Adt1::C0(_, _, _, _, h37, _, _, _, _, _, _) => {
                h37
            }
        };
        let a40 = match v5.clone() {
            Adt1::C0(_, _, _, _, h38, _, _, _, _, _, _) => {
                h38
            }
        };
        nat_eq(a39, a40)
    } else {
        false
    } {
        f11(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f12(v8.clone(), v5.clone(), v6.clone(), v10.clone())?
    } else {
        false
    } {
        f13(v9.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        Ok(Adt5::C1(Adt2::C0(v5, v6, v7, v8, v9, v10, f14(v2), f15(v3))))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f2(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt5> {
    let v8: u64 = {
        let a42 = match v2.clone() {
            Adt1::C0(h41, _, _, _, _, _, _, _, _, _, _) => {
                h41
            }
        };
        let a43 = 1u64;
        if nat_eq(a42, a43) {
            2u64
        } else {
            4u64
        }
    };
    let a44 = v0;
    let a45 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a44, a45) {
        !v1
    } else {
        false
    } {
        let a47 = match v2.clone() {
            Adt1::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                h46
            }
        };
        let a48 = 0u64;
        if nat_eq(a47, a48) {
            true
        } else {
            let a50 = match v2.clone() {
                Adt1::C0(h49, _, _, _, _, _, _, _, _, _, _) => {
                    h49
                }
            };
            let a51 = 1u64;
            nat_eq(a50, a51)
        }
    } else {
        false
    } {
        let a53 = match v3.clone() {
            Adt1::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
                h52
            }
        };
        let a54 = 26u64;
        nat_eq(a53, a54)
    } else {
        false
    } {
        let a57 = match v7.clone() {
            Adt1::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
                h55
            }
        };
        let a58 = match v2.clone() {
            Adt1::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        };
        nat_eq(a57, a58)
    } else {
        false
    } {
        let a60 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h59, _, _, _, _, _) => {
                h59
            }
        };
        let a61 = v8;
        nat_eq(a60, a61)
    } else {
        false
    } {
        let a63 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h62, _, _, _, _, _) => {
                h62
            }
        };
        let a64 = v8;
        nat_eq(a63, a64)
    } else {
        false
    } {
        let a67 = match v2.clone() {
            Adt1::C0(_, h65, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        };
        let a68 = match v7.clone() {
            Adt1::C0(_, h66, _, _, _, _, _, _, _, _, _) => {
                h66
            }
        };
        nat_eq(a67, a68)
    } else {
        false
    } {
        f16(v3.clone(), v7.clone())
    } else {
        false
    } {
        f17(v7.clone(), v8)?
    } else {
        false
    } {
        f11(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f11(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, h69, _, _, _, _, _, _, _, _) => {
                h69
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h70, _, _, _, _, _, _, _, _) => {
                h70
            }
        })
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, _, h71, _, _, _, _, _, _, _) => {
                h71
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h72, _, _, _, _, _, _, _) => {
                h72
            }
        })
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, _, _, h73, _, _, _, _, _, _) => {
                h73
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
                h74
            }
        })
    } else {
        false
    } {
        let m75 = v4.clone();
        match m75 {
            None => {
                true
            }
            Some(v9) => {
                let a77 = match v9.clone() {
                    Adt1::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                        h76
                    }
                };
                let a78 = 0u64;
                if if nat_eq(a77, a78) {
                    f16(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f11(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m79 = v5.clone();
        match m79 {
            None => {
                true
            }
            Some(v10) => {
                let a81 = match v10.clone() {
                    Adt1::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                        h80
                    }
                };
                let a82 = 26u64;
                if if if if nat_eq(a81, a82) {
                    let a84 = match v10.clone() {
                        Adt1::C0(_, _, _, h83, _, _, _, _, _, _, _) => {
                            h83
                        }
                    };
                    let a85 = 1u64;
                    nat_eq(a84, a85)
                } else {
                    false
                } {
                    let a87 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h86, _, _, _, _, _, _) => {
                            h86
                        }
                    };
                    let a88 = 1u64;
                    nat_eq(a87, a88)
                } else {
                    false
                } {
                    f11(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m89 = v4.clone();
                    match m89 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a92 = match v11 {
                                Adt1::C0(_, h90, _, _, _, _, _, _, _, _, _) => {
                                    h90
                                }
                            };
                            let a93 = match v10 {
                                Adt1::C0(_, _, h91, _, _, _, _, _, _, _, _) => {
                                    h91
                                }
                            };
                            nat_eq(a92, a93)
                        }
                    }
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m94 = v6.clone();
        match m94 {
            None => {
                true
            }
            Some(v12) => {
                let a96 = match v12.clone() {
                    Adt1::C0(h95, _, _, _, _, _, _, _, _, _, _) => {
                        h95
                    }
                };
                let a97 = 26u64;
                if if nat_eq(a96, a97) {
                    f16(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f11(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt5::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a98 = v8;
            let a99 = 2u64;
            if nat_eq(a98, a99) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m100 = v5;
            match m100 {
                None => {
                    let m101 = v4;
                    match m101 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h102, _, _, _, _, _, _, _, _, _) => {
                                    h102
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h103, _, _, _, _, _, _, _, _, _) => {
                                    h103
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h104, _, _, _, _, _, _, _, _) => {
                            h104
                        }
                    }
                }
            }
        })))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f3(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: Adt1 = match v0.clone() {
        Adt4::C0(h105, _, _, _, _, _, _, _, _) => {
            h105
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h106, _, _, _, _, _, _, _) => {
            h106
        }
    };
    let a108 = v1;
    let a109 = match v5.clone() {
        Adt1::C0(_, _, h107, _, _, _, _, _, _, _, _) => {
            h107
        }
    };
    if if if nat_lt(a108, a109) {
        let a111 = v2;
        let a112 = match v5.clone() {
            Adt1::C0(_, _, _, h110, _, _, _, _, _, _, _) => {
                h110
            }
        };
        nat_lt(a111, a112)
    } else {
        false
    } {
        let a114 = v3;
        let a115 = match v5.clone() {
            Adt1::C0(_, _, _, _, h113, _, _, _, _, _, _) => {
                h113
            }
        };
        nat_lt(a114, a115)
    } else {
        false
    } {
        Ok(Adt5::C6(f19(v4.clone(), {
            let a117 = v1;
            let a118 = match v4.clone() {
                Adt1::C0(_, _, h116, _, _, _, _, _, _, _, _) => {
                    h116
                }
            };
            let a119 = 0u64;
            nat_rem(a117, a118, a119)
        }, {
            let a121 = v2;
            let a122 = match v4.clone() {
                Adt1::C0(_, _, _, h120, _, _, _, _, _, _, _) => {
                    h120
                }
            };
            let a123 = 0u64;
            nat_rem(a121, a122, a123)
        }, {
            let a125 = v3;
            let a126 = match v4 {
                Adt1::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                    h124
                }
            };
            let a127 = 0u64;
            nat_rem(a125, a126, a127)
        })?, f19(v5, v1, v2, v3)?, {
            let m129 = match v0.clone() {
                Adt4::C0(_, _, h128, _, _, _, _, _, _) => {
                    h128
                }
            };
            match m129 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f19(v6, v1, v2, v3)?
                }
            }
        }, {
            let m131 = match v0.clone() {
                Adt4::C0(_, _, _, _, h130, _, _, _, _) => {
                    h130
                }
            };
            match m131 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f19(v7, v1, v2, v3)?
                }
            }
        }, f19(match v0 {
            Adt4::C0(_, _, _, _, _, h132, _, _, _) => {
                h132
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f4(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a133 = v1;
    let a134 = 0u64;
    if nat_eq(a133, a134) {
        let a138 = v2;
        let a139 = {
            let m137 = match v0.clone() {
                Adt4::C0(_, h136, _, _, _, _, _, _, _) => {
                    h136
                }
            };
            match m137 {
                Adt1::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                    h135
                }
            }
        };
        if nat_lt(a138, a139) {
            Ok(Adt5::C7({
                let a143 = v2;
                let a144 = {
                    let m142 = match v0 {
                        Adt4::C0(_, h141, _, _, _, _, _, _, _) => {
                            h141
                        }
                    };
                    match m142 {
                        Adt1::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
                            h140
                        }
                    }
                };
                nat_mul(a143, a144)?
            }))
        } else {
            Ok(Adt5::C8)
        }
    } else {
        let a145 = v1;
        let a146 = 1u64;
        if nat_eq(a145, a146) {
            let m148 = match v0 {
                Adt4::C0(_, _, _, _, h147, _, _, _, _) => {
                    h147
                }
            };
            match m148 {
                None => {
                    Ok(Adt5::C8)
                }
                Some(v4) => {
                    let a150 = v2;
                    let a151 = match v4.clone() {
                        Adt1::C0(_, h149, _, _, _, _, _, _, _, _, _) => {
                            h149
                        }
                    };
                    if nat_lt(a150, a151) {
                        Ok(Adt5::C7({
                            let a153 = v2;
                            let a154 = match v4 {
                                Adt1::C0(_, _, _, _, _, h152, _, _, _, _, _) => {
                                    h152
                                }
                            };
                            nat_mul(a153, a154)?
                        }))
                    } else {
                        Ok(Adt5::C8)
                    }
                }
            }
        } else {
            let a155 = v1;
            let a156 = 2u64;
            if nat_eq(a155, a156) {
                let a157 = v2;
                let a158 = 2147483648u64;
                if if nat_lt(a157, a158) {
                    let a160 = v2;
                    let a161 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h159) => {
                            h159
                        }
                    };
                    nat_lt(a160, a161)
                } else {
                    false
                } {
                    let m163 = match v0 {
                        Adt4::C0(_, _, h162, _, _, _, _, _, _) => {
                            h162
                        }
                    };
                    match m163 {
                        None => {
                            Ok(Adt5::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt5::C7({
                                let a165 = v2;
                                let a166 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h164, _, _, _, _, _) => {
                                        h164
                                    }
                                };
                                nat_mul(a165, a166)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt5::C8)
                }
            } else {
                let a167 = v1;
                let a168 = 3u64;
                if nat_eq(a167, a168) {
                    let m170 = match v0 {
                        Adt4::C0(_, _, _, h169, _, _, _, _, _) => {
                            h169
                        }
                    };
                    match m170 {
                        None => {
                            Ok(Adt5::C8)
                        }
                        Some(v6) => {
                            let a172 = v2;
                            let a173 = match v6.clone() {
                                Adt1::C0(_, _, h171, _, _, _, _, _, _, _, _) => {
                                    h171
                                }
                            };
                            if if nat_lt(a172, a173) {
                                let a175 = v3;
                                let a176 = match v6.clone() {
                                    Adt1::C0(_, h174, _, _, _, _, _, _, _, _, _) => {
                                        h174
                                    }
                                };
                                nat_lt(a175, a176)
                            } else {
                                false
                            } {
                                Ok(Adt5::C7({
                                    let a183 = {
                                        let a178 = v2;
                                        let a179 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h177, _, _, _, _) => {
                                                h177
                                            }
                                        };
                                        nat_mul(a178, a179)?
                                    };
                                    let a184 = {
                                        let a181 = v3;
                                        let a182 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h180, _, _, _, _, _) => {
                                                h180
                                            }
                                        };
                                        nat_mul(a181, a182)?
                                    };
                                    nat_add(a183, a184)?
                                }))
                            } else {
                                Ok(Adt5::C8)
                            }
                        }
                    }
                } else {
                    let a185 = v1;
                    let a186 = 4u64;
                    if nat_eq(a185, a186) {
                        let a187 = v2;
                        let a188 = 2147483648u64;
                        if if nat_lt(a187, a188) {
                            let a192 = v2;
                            let a193 = {
                                let m191 = match v0.clone() {
                                    Adt4::C0(h190, _, _, _, _, _, _, _, _) => {
                                        h190
                                    }
                                };
                                match m191 {
                                    Adt1::C0(_, h189, _, _, _, _, _, _, _, _, _) => {
                                        h189
                                    }
                                }
                            };
                            nat_lt(a192, a193)
                        } else {
                            false
                        } {
                            Ok(Adt5::C7({
                                let a195 = v2;
                                let a196 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h194, _, _) => {
                                        h194
                                    }
                                };
                                nat_mul(a195, a196)?
                            }))
                        } else {
                            Ok(Adt5::C8)
                        }
                    } else {
                        Ok(Adt5::C0)
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt5> {
    if v1 {
        Ok(Adt5::C0)
    } else {
        let a197 = v0;
        let a198 = 43u64;
        if if if if if if if if nat_eq(a197, a198) {
            let a200 = match v4.clone() {
                Adt1::C0(h199, _, _, _, _, _, _, _, _, _, _) => {
                    h199
                }
            };
            let a201 = 0u64;
            if nat_eq(a200, a201) {
                f17(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a203 = match v7.clone() {
                Adt1::C0(h202, _, _, _, _, _, _, _, _, _, _) => {
                    h202
                }
            };
            let a204 = 0u64;
            if nat_eq(a203, a204) {
                f17(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f20(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f11(v4.clone(), v7.clone())? {
                true
            } else {
                f21(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f22(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f23(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f8(v3, match v4.clone() {
                Adt1::C0(_, _, _, h205, _, _, _, _, _, _, _) => {
                    h205
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt5::C3(Adt3::C0(v4, v5, v7, v2, f14(v3))))
        } else {
            Ok(Adt5::C0)
        }
    }
}

pub fn f6(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h206, _, _, _, _) => {
            h206
        }
    };
    let a208 = v1;
    let a209 = match v4.clone() {
        Adt1::C0(_, _, h207, _, _, _, _, _, _, _, _) => {
            h207
        }
    };
    if if if nat_lt(a208, a209) {
        let a211 = v2;
        let a212 = match v4.clone() {
            Adt1::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                h210
            }
        };
        nat_lt(a211, a212)
    } else {
        false
    } {
        let a214 = v3;
        let a215 = match v4.clone() {
            Adt1::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                h213
            }
        };
        nat_lt(a214, a215)
    } else {
        false
    } {
        let v6: u64 = {
            let m217 = match v0.clone() {
                Adt3::C0(_, h216, _, _, _) => {
                    h216
                }
            };
            match m217 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f19(v5.clone(), v1, {
                        let a219 = v2;
                        let a220 = match v5.clone() {
                            Adt1::C0(_, _, _, h218, _, _, _, _, _, _, _) => {
                                h218
                            }
                        };
                        let a221 = 0u64;
                        nat_rem(a219, a220, a221)
                    }, {
                        let a223 = v3;
                        let a224 = match v5 {
                            Adt1::C0(_, _, _, _, h222, _, _, _, _, _, _) => {
                                h222
                            }
                        };
                        let a225 = 0u64;
                        nat_rem(a223, a224, a225)
                    })?
                }
            }
        };
        Ok(Adt5::C4(f19(v4, v1, v2, v3)?, v6, {
            let a226 = v2;
            let a227 = 4u64;
            nat_mul(a226, a227)?
        }, f19(match v0 {
            Adt3::C0(_, _, h228, _, _) => {
                h228
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f7(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a232 = v1;
    let a233 = {
        let m231 = match v0.clone() {
            Adt2::C0(h230, _, _, _, _, _, _, _) => {
                h230
            }
        };
        match m231 {
            Adt1::C0(_, _, h229, _, _, _, _, _, _, _, _) => {
                h229
            }
        }
    };
    if if if nat_lt(a232, a233) {
        let a237 = v2;
        let a238 = {
            let m236 = match v0.clone() {
                Adt2::C0(h235, _, _, _, _, _, _, _) => {
                    h235
                }
            };
            match m236 {
                Adt1::C0(_, _, _, h234, _, _, _, _, _, _, _) => {
                    h234
                }
            }
        };
        nat_lt(a237, a238)
    } else {
        false
    } {
        let a242 = v3;
        let a243 = {
            let m241 = match v0.clone() {
                Adt2::C0(h240, _, _, _, _, _, _, _) => {
                    h240
                }
            };
            match m241 {
                Adt1::C0(_, _, _, _, h239, _, _, _, _, _, _) => {
                    h239
                }
            }
        };
        nat_lt(a242, a243)
    } else {
        false
    } {
        Ok(Adt5::C2(f19(match v0.clone() {
            Adt2::C0(h244, _, _, _, _, _, _, _) => {
                h244
            }
        }, v1, v2, v3)?, f19(match v0.clone() {
            Adt2::C0(_, h245, _, _, _, _, _, _) => {
                h245
            }
        }, 0u64, {
            let a255 = v2;
            let a256 = {
                let a252 = {
                    let m248 = match v0.clone() {
                        Adt2::C0(h247, _, _, _, _, _, _, _) => {
                            h247
                        }
                    };
                    match m248 {
                        Adt1::C0(_, _, _, h246, _, _, _, _, _, _, _) => {
                            h246
                        }
                    }
                };
                let a253 = {
                    let m251 = match v0.clone() {
                        Adt2::C0(_, h250, _, _, _, _, _, _) => {
                            h250
                        }
                    };
                    match m251 {
                        Adt1::C0(_, _, _, h249, _, _, _, _, _, _, _) => {
                            h249
                        }
                    }
                };
                let a254 = 0u64;
                nat_quot(a252, a253, a254)
            };
            let a257 = 0u64;
            nat_quot(a255, a256, a257)
        }, {
            let a267 = v3;
            let a268 = {
                let a264 = {
                    let m260 = match v0.clone() {
                        Adt2::C0(h259, _, _, _, _, _, _, _) => {
                            h259
                        }
                    };
                    match m260 {
                        Adt1::C0(_, _, _, _, h258, _, _, _, _, _, _) => {
                            h258
                        }
                    }
                };
                let a265 = {
                    let m263 = match v0.clone() {
                        Adt2::C0(_, h262, _, _, _, _, _, _) => {
                            h262
                        }
                    };
                    match m263 {
                        Adt1::C0(_, _, _, _, h261, _, _, _, _, _, _) => {
                            h261
                        }
                    }
                };
                let a266 = 0u64;
                nat_quot(a264, a265, a266)
            };
            let a269 = 0u64;
            nat_quot(a267, a268, a269)
        })?, f19(match v0.clone() {
            Adt2::C0(_, _, h270, _, _, _, _, _) => {
                h270
            }
        }, 0u64, {
            let a280 = v2;
            let a281 = {
                let a277 = {
                    let m273 = match v0.clone() {
                        Adt2::C0(h272, _, _, _, _, _, _, _) => {
                            h272
                        }
                    };
                    match m273 {
                        Adt1::C0(_, _, _, h271, _, _, _, _, _, _, _) => {
                            h271
                        }
                    }
                };
                let a278 = {
                    let m276 = match v0.clone() {
                        Adt2::C0(_, _, h275, _, _, _, _, _) => {
                            h275
                        }
                    };
                    match m276 {
                        Adt1::C0(_, _, _, h274, _, _, _, _, _, _, _) => {
                            h274
                        }
                    }
                };
                let a279 = 0u64;
                nat_quot(a277, a278, a279)
            };
            let a282 = 0u64;
            nat_quot(a280, a281, a282)
        }, {
            let a292 = v3;
            let a293 = {
                let a289 = {
                    let m285 = match v0.clone() {
                        Adt2::C0(h284, _, _, _, _, _, _, _) => {
                            h284
                        }
                    };
                    match m285 {
                        Adt1::C0(_, _, _, _, h283, _, _, _, _, _, _) => {
                            h283
                        }
                    }
                };
                let a290 = {
                    let m288 = match v0.clone() {
                        Adt2::C0(_, _, h287, _, _, _, _, _) => {
                            h287
                        }
                    };
                    match m288 {
                        Adt1::C0(_, _, _, _, h286, _, _, _, _, _, _) => {
                            h286
                        }
                    }
                };
                let a291 = 0u64;
                nat_quot(a289, a290, a291)
            };
            let a294 = 0u64;
            nat_quot(a292, a293, a294)
        })?, {
            let m296 = match v0.clone() {
                Adt2::C0(_, _, _, h295, _, _, _, _) => {
                    h295
                }
            };
            match m296 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f19(v4.clone(), v1, {
                        let a298 = v2;
                        let a299 = match v4.clone() {
                            Adt1::C0(_, _, _, h297, _, _, _, _, _, _, _) => {
                                h297
                            }
                        };
                        let a300 = 0u64;
                        nat_rem(a298, a299, a300)
                    }, {
                        let a302 = v3;
                        let a303 = match v4 {
                            Adt1::C0(_, _, _, _, h301, _, _, _, _, _, _) => {
                                h301
                            }
                        };
                        let a304 = 0u64;
                        nat_rem(a302, a303, a304)
                    })?
                }
            }
        }, {
            let a305 = v2;
            let a306 = 4u64;
            nat_mul(a305, a306)?
        }, f19(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h307, _, _) => {
                h307
            }
        }, v2, v1, v3)?, {
            let m310 = match v0.clone() {
                Adt2::C0(_, h309, _, _, _, _, _, _) => {
                    h309
                }
            };
            match m310 {
                Adt1::C0(_, _, _, _, _, _, h308, _, _, _, _) => {
                    h308
                }
            }
        }, {
            let m313 = match v0.clone() {
                Adt2::C0(_, _, h312, _, _, _, _, _) => {
                    h312
                }
            };
            match m313 {
                Adt1::C0(_, _, _, _, _, _, h311, _, _, _, _) => {
                    h311
                }
            }
        }, f24({
            let m316 = match v0 {
                Adt2::C0(_, h315, _, _, _, _, _, _) => {
                    h315
                }
            };
            match m316 {
                Adt1::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
                    h314
                }
            }
        })))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f8(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a317 = v0;
    let a318 = 0u64;
    if if nat_eq(a317, a318) {
        true
    } else {
        let a319 = 2147483648u64;
        let a320 = v0;
        if nat_le(a319, a320) {
            let a321 = v0;
            let a322 = 4286578688u64;
            nat_lt(a321, a322)
        } else {
            false
        }
    } {
        true
    } else {
        let a323 = v0;
        let a324 = 2139095040u64;
        if if nat_lt(a323, a324) {
            let a325 = v1;
            let a326 = 4294967295u64;
            nat_le(a325, a326)
        } else {
            false
        } {
            let m327 = v2;
            match m327 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            false
        }
    }
}

pub fn f9(v0: Adt1) -> R<bool> {
    let a329 = match v0.clone() {
        Adt1::C0(h328, _, _, _, _, _, _, _, _, _, _) => {
            h328
        }
    };
    let a330 = 0u64;
    if if if if nat_eq(a329, a330) {
        let a332 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h331, _, _, _, _, _) => {
                h331
            }
        };
        let a333 = 4u64;
        nat_eq(a332, a333)
    } else {
        false
    } {
        let a339 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h334, _, _, _, _) => {
                h334
            }
        };
        let a340 = {
            let a337 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h335, _, _, _, _, _) => {
                    h335
                }
            };
            let a338 = match v0.clone() {
                Adt1::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                    h336
                }
            };
            nat_mul(a337, a338)?
        };
        nat_eq(a339, a340)
    } else {
        false
    } {
        let a346 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h341, _, _, _) => {
                h341
            }
        };
        let a347 = {
            let a344 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h342, _, _, _, _) => {
                    h342
                }
            };
            let a345 = match v0.clone() {
                Adt1::C0(_, _, h343, _, _, _, _, _, _, _, _) => {
                    h343
                }
            };
            nat_mul(a344, a345)?
        };
        nat_eq(a346, a347)
    } else {
        false
    } {
        let a353 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h348, _, _) => {
                h348
            }
        };
        let a354 = {
            let a351 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h349, _, _, _) => {
                    h349
                }
            };
            let a352 = match v0 {
                Adt1::C0(_, _, _, h350, _, _, _, _, _, _, _) => {
                    h350
                }
            };
            nat_mul(a351, a352)?
        };
        Ok(nat_eq(a353, a354))
    } else {
        Ok(false)
    }
}

pub fn f10(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f25(match v0.clone() {
        Adt1::C0(h355, _, _, _, _, _, _, _, _, _, _) => {
            h355
        }
    }, match v0.clone() {
        Adt1::C0(_, h356, _, _, _, _, _, _, _, _, _) => {
            h356
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h357, _, _, _, _, _) => {
            h357
        }
    }) {
        f18(match v1.clone() {
            Adt1::C0(_, _, _, h358, _, _, _, _, _, _, _) => {
                h358
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h359, _, _, _, _, _, _, _) => {
                h359
            }
        })
    } else {
        false
    } {
        f18(match v1 {
            Adt1::C0(_, _, _, _, h360, _, _, _, _, _, _) => {
                h360
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h361, _, _, _, _, _, _) => {
                h361
            }
        })
    } else {
        false
    } {
        f11(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f11(v0: Adt1, v1: Adt1) -> R<bool> {
    let a363 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h362, _) => {
            h362
        }
    };
    let a364 = 0u64;
    if nat_eq(a363, a364) {
        Ok(true)
    } else {
        let a366 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h365, _) => {
                h365
            }
        };
        let a367 = 0u64;
        if nat_eq(a366, a367) {
            Ok(true)
        } else {
            let a369 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h368) => {
                    h368
                }
            };
            let a370 = 0u64;
            if nat_eq(a369, a370) {
                Ok(true)
            } else {
                let a372 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h371) => {
                        h371
                    }
                };
                let a373 = 0u64;
                if nat_eq(a372, a373) {
                    Ok(true)
                } else {
                    let a379 = {
                        let a376 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h374) => {
                                h374
                            }
                        };
                        let a377 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h375, _) => {
                                h375
                            }
                        };
                        nat_add(a376, a377)?
                    };
                    let a380 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h378) => {
                            h378
                        }
                    };
                    if nat_le(a379, a380) {
                        Ok(true)
                    } else {
                        let a386 = {
                            let a383 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h381) => {
                                    h381
                                }
                            };
                            let a384 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h382, _) => {
                                    h382
                                }
                            };
                            nat_add(a383, a384)?
                        };
                        let a387 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h385) => {
                                h385
                            }
                        };
                        Ok(nat_le(a386, a387))
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m388 = v0;
    match m388 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a390 = match v4.clone() {
                Adt1::C0(h389, _, _, _, _, _, _, _, _, _, _) => {
                    h389
                }
            };
            let a391 = 1u64;
            if if if if if if nat_eq(a390, a391) {
                let a393 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h392, _, _, _, _, _) => {
                        h392
                    }
                };
                let a394 = 2u64;
                nat_eq(a393, a394)
            } else {
                false
            } {
                let a397 = match v2 {
                    Adt1::C0(_, _, h395, _, _, _, _, _, _, _, _) => {
                        h395
                    }
                };
                let a398 = match v4.clone() {
                    Adt1::C0(_, h396, _, _, _, _, _, _, _, _, _) => {
                        h396
                    }
                };
                nat_le(a397, a398)
            } else {
                false
            } {
                let a401 = match v1.clone() {
                    Adt1::C0(_, _, h399, _, _, _, _, _, _, _, _) => {
                        h399
                    }
                };
                let a402 = match v4.clone() {
                    Adt1::C0(_, _, h400, _, _, _, _, _, _, _, _) => {
                        h400
                    }
                };
                nat_le(a401, a402)
            } else {
                false
            } {
                f18(match v1.clone() {
                    Adt1::C0(_, _, _, h403, _, _, _, _, _, _, _) => {
                        h403
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h404, _, _, _, _, _, _, _) => {
                        h404
                    }
                })
            } else {
                false
            } {
                f18(match v1 {
                    Adt1::C0(_, _, _, _, h405, _, _, _, _, _, _) => {
                        h405
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h406, _, _, _, _, _, _) => {
                        h406
                    }
                })
            } else {
                false
            } {
                f11(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f13(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m407 = v0;
    match m407 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a409 = match v3.clone() {
                Adt1::C0(h408, _, _, _, _, _, _, _, _, _, _) => {
                    h408
                }
            };
            let a410 = 0u64;
            if if if if if if nat_eq(a409, a410) {
                let a412 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h411, _, _, _, _, _) => {
                        h411
                    }
                };
                let a413 = 4u64;
                nat_eq(a412, a413)
            } else {
                false
            } {
                let a416 = match v3.clone() {
                    Adt1::C0(_, h414, _, _, _, _, _, _, _, _, _) => {
                        h414
                    }
                };
                let a417 = match v1 {
                    Adt1::C0(_, _, _, h415, _, _, _, _, _, _, _) => {
                        h415
                    }
                };
                nat_eq(a416, a417)
            } else {
                false
            } {
                let a419 = match v3.clone() {
                    Adt1::C0(_, _, h418, _, _, _, _, _, _, _, _) => {
                        h418
                    }
                };
                let a420 = 1u64;
                nat_eq(a419, a420)
            } else {
                false
            } {
                let a422 = match v3.clone() {
                    Adt1::C0(_, _, _, h421, _, _, _, _, _, _, _) => {
                        h421
                    }
                };
                let a423 = 1u64;
                nat_eq(a422, a423)
            } else {
                false
            } {
                let a425 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h424, _, _, _, _, _, _) => {
                        h424
                    }
                };
                let a426 = 1u64;
                nat_eq(a425, a426)
            } else {
                false
            } {
                f11(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f14(v0: u64) -> Option<u64> {
    let a427 = 0u64;
    let a428 = v0;
    if if nat_lt(a427, a428) {
        let a429 = v0;
        let a430 = 2139095040u64;
        nat_lt(a429, a430)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f15(v0: u64) -> Option<u64> {
    let a431 = v0;
    let a432 = 0u64;
    if if nat_eq(a431, a432) {
        true
    } else {
        let a433 = v0;
        let a434 = 2147483648u64;
        nat_eq(a433, a434)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f16(v0: Adt1, v1: Adt1) -> bool {
    let a437 = match v0.clone() {
        Adt1::C0(_, _, h435, _, _, _, _, _, _, _, _) => {
            h435
        }
    };
    let a438 = match v1.clone() {
        Adt1::C0(_, _, h436, _, _, _, _, _, _, _, _) => {
            h436
        }
    };
    if if nat_eq(a437, a438) {
        let a441 = match v0.clone() {
            Adt1::C0(_, _, _, h439, _, _, _, _, _, _, _) => {
                h439
            }
        };
        let a442 = match v1.clone() {
            Adt1::C0(_, _, _, h440, _, _, _, _, _, _, _) => {
                h440
            }
        };
        nat_eq(a441, a442)
    } else {
        false
    } {
        let a445 = match v0 {
            Adt1::C0(_, _, _, _, h443, _, _, _, _, _, _) => {
                h443
            }
        };
        let a446 = match v1 {
            Adt1::C0(_, _, _, _, h444, _, _, _, _, _, _) => {
                h444
            }
        };
        nat_eq(a445, a446)
    } else {
        false
    }
}

pub fn f17(v0: Adt1, v1: u64) -> R<bool> {
    let a448 = match v0.clone() {
        Adt1::C0(_, h447, _, _, _, _, _, _, _, _, _) => {
            h447
        }
    };
    let a449 = 1u64;
    if if if if nat_eq(a448, a449) {
        true
    } else {
        let a451 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h450, _, _, _, _, _) => {
                h450
            }
        };
        let a452 = v1;
        nat_eq(a451, a452)
    } {
        let a454 = match v0.clone() {
            Adt1::C0(_, _, h453, _, _, _, _, _, _, _, _) => {
                h453
            }
        };
        let a455 = 1u64;
        if nat_eq(a454, a455) {
            true
        } else {
            let a460 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h456, _, _, _, _) => {
                    h456
                }
            };
            let a461 = {
                let a458 = v1;
                let a459 = match v0.clone() {
                    Adt1::C0(_, h457, _, _, _, _, _, _, _, _, _) => {
                        h457
                    }
                };
                nat_mul(a458, a459)?
            };
            nat_eq(a460, a461)
        }
    } else {
        false
    } {
        let a463 = match v0.clone() {
            Adt1::C0(_, _, _, h462, _, _, _, _, _, _, _) => {
                h462
            }
        };
        let a464 = 1u64;
        if nat_eq(a463, a464) {
            true
        } else {
            let a472 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h465, _, _, _) => {
                    h465
                }
            };
            let a473 = {
                let a470 = {
                    let a467 = v1;
                    let a468 = match v0.clone() {
                        Adt1::C0(_, h466, _, _, _, _, _, _, _, _, _) => {
                            h466
                        }
                    };
                    nat_mul(a467, a468)?
                };
                let a471 = match v0.clone() {
                    Adt1::C0(_, _, h469, _, _, _, _, _, _, _, _) => {
                        h469
                    }
                };
                nat_mul(a470, a471)?
            };
            nat_eq(a472, a473)
        }
    } else {
        false
    } {
        let a475 = match v0.clone() {
            Adt1::C0(_, _, _, _, h474, _, _, _, _, _, _) => {
                h474
            }
        };
        let a476 = 1u64;
        if nat_eq(a475, a476) {
            Ok(true)
        } else {
            let a487 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h477, _, _) => {
                    h477
                }
            };
            let a488 = {
                let a485 = {
                    let a482 = {
                        let a479 = v1;
                        let a480 = match v0.clone() {
                            Adt1::C0(_, h478, _, _, _, _, _, _, _, _, _) => {
                                h478
                            }
                        };
                        nat_mul(a479, a480)?
                    };
                    let a483 = match v0.clone() {
                        Adt1::C0(_, _, h481, _, _, _, _, _, _, _, _) => {
                            h481
                        }
                    };
                    nat_mul(a482, a483)?
                };
                let a486 = match v0 {
                    Adt1::C0(_, _, _, h484, _, _, _, _, _, _, _) => {
                        h484
                    }
                };
                nat_mul(a485, a486)?
            };
            Ok(nat_eq(a487, a488))
        }
    } else {
        Ok(false)
    }
}

pub fn f18(v0: u64, v1: u64) -> bool {
    let a489 = 0u64;
    let a490 = v1;
    if nat_lt(a489, a490) {
        let a494 = {
            let a491 = v0;
            let a492 = v1;
            let a493 = 0u64;
            nat_rem(a491, a492, a493)
        };
        let a495 = 0u64;
        nat_eq(a494, a495)
    } else {
        false
    }
}

pub fn f19(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a507 = {
        let a502 = {
            let a497 = v1;
            let a498 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h496, _, _, _, _) => {
                    h496
                }
            };
            nat_mul(a497, a498)?
        };
        let a503 = {
            let a500 = v2;
            let a501 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h499, _, _, _) => {
                    h499
                }
            };
            nat_mul(a500, a501)?
        };
        nat_add(a502, a503)?
    };
    let a508 = {
        let a505 = v3;
        let a506 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h504, _, _) => {
                h504
            }
        };
        nat_mul(a505, a506)?
    };
    nat_add(a507, a508)
}

pub fn f20(v0: Adt1, v1: Adt1) -> bool {
    let a511 = match v0.clone() {
        Adt1::C0(_, h509, _, _, _, _, _, _, _, _, _) => {
            h509
        }
    };
    let a512 = match v1.clone() {
        Adt1::C0(_, h510, _, _, _, _, _, _, _, _, _) => {
            h510
        }
    };
    if if if nat_eq(a511, a512) {
        let a515 = match v0.clone() {
            Adt1::C0(_, _, h513, _, _, _, _, _, _, _, _) => {
                h513
            }
        };
        let a516 = match v1.clone() {
            Adt1::C0(_, _, h514, _, _, _, _, _, _, _, _) => {
                h514
            }
        };
        nat_eq(a515, a516)
    } else {
        false
    } {
        let a519 = match v0.clone() {
            Adt1::C0(_, _, _, h517, _, _, _, _, _, _, _) => {
                h517
            }
        };
        let a520 = match v1.clone() {
            Adt1::C0(_, _, _, h518, _, _, _, _, _, _, _) => {
                h518
            }
        };
        nat_eq(a519, a520)
    } else {
        false
    } {
        let a523 = match v0 {
            Adt1::C0(_, _, _, _, h521, _, _, _, _, _, _) => {
                h521
            }
        };
        let a524 = match v1 {
            Adt1::C0(_, _, _, _, h522, _, _, _, _, _, _) => {
                h522
            }
        };
        nat_eq(a523, a524)
    } else {
        false
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f20(v0.clone(), v1.clone()) {
        let a527 = match v0.clone() {
            Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                h525
            }
        };
        let a528 = match v1.clone() {
            Adt1::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                h526
            }
        };
        nat_eq(a527, a528)
    } else {
        false
    } {
        let a531 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h529) => {
                h529
            }
        };
        let a532 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h530) => {
                h530
            }
        };
        nat_eq(a531, a532)
    } else {
        false
    } {
        let a535 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h533, _, _, _, _, _) => {
                h533
            }
        };
        let a536 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h534, _, _, _, _, _) => {
                h534
            }
        };
        nat_eq(a535, a536)
    } else {
        false
    } {
        let a539 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h537, _, _, _, _) => {
                h537
            }
        };
        let a540 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h538, _, _, _, _) => {
                h538
            }
        };
        nat_eq(a539, a540)
    } else {
        false
    } {
        let a543 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h541, _, _, _) => {
                h541
            }
        };
        let a544 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h542, _, _, _) => {
                h542
            }
        };
        nat_eq(a543, a544)
    } else {
        false
    } {
        let a547 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h545, _, _) => {
                h545
            }
        };
        let a548 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h546, _, _) => {
                h546
            }
        };
        nat_eq(a547, a548)
    } else {
        false
    }
}

pub fn f22(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m549 = v0;
    match m549 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a551 = match v3.clone() {
                Adt1::C0(h550, _, _, _, _, _, _, _, _, _, _) => {
                    h550
                }
            };
            let a552 = 0u64;
            if if if if if if if nat_eq(a551, a552) {
                true
            } else {
                let a554 = match v3.clone() {
                    Adt1::C0(h553, _, _, _, _, _, _, _, _, _, _) => {
                        h553
                    }
                };
                let a555 = 1u64;
                nat_eq(a554, a555)
            } {
                f17(v3.clone(), {
                    let a557 = match v3.clone() {
                        Adt1::C0(h556, _, _, _, _, _, _, _, _, _, _) => {
                            h556
                        }
                    };
                    let a558 = 0u64;
                    if nat_eq(a557, a558) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a561 = match v3.clone() {
                    Adt1::C0(_, h559, _, _, _, _, _, _, _, _, _) => {
                        h559
                    }
                };
                let a562 = match v1.clone() {
                    Adt1::C0(_, h560, _, _, _, _, _, _, _, _, _) => {
                        h560
                    }
                };
                nat_eq(a561, a562)
            } else {
                false
            } {
                let a565 = match v1.clone() {
                    Adt1::C0(_, _, h563, _, _, _, _, _, _, _, _) => {
                        h563
                    }
                };
                let a566 = match v3.clone() {
                    Adt1::C0(_, _, h564, _, _, _, _, _, _, _, _) => {
                        h564
                    }
                };
                nat_le(a565, a566)
            } else {
                false
            } {
                f18(match v1.clone() {
                    Adt1::C0(_, _, _, h567, _, _, _, _, _, _, _) => {
                        h567
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h568, _, _, _, _, _, _, _) => {
                        h568
                    }
                })
            } else {
                false
            } {
                f18(match v1 {
                    Adt1::C0(_, _, _, _, h569, _, _, _, _, _, _) => {
                        h569
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h570, _, _, _, _, _, _) => {
                        h570
                    }
                })
            } else {
                false
            } {
                f11(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f23(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m571 = v0;
    match m571 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a573 = match v3.clone() {
                Adt1::C0(h572, _, _, _, _, _, _, _, _, _, _) => {
                    h572
                }
            };
            let a574 = 0u64;
            if if if if if if nat_eq(a573, a574) {
                let a576 = match v3.clone() {
                    Adt1::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
                        h575
                    }
                };
                let a577 = 1u64;
                if nat_eq(a576, a577) {
                    true
                } else {
                    let a579 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h578, _, _, _, _, _) => {
                            h578
                        }
                    };
                    let a580 = 4u64;
                    nat_eq(a579, a580)
                }
            } else {
                false
            } {
                let a583 = match v3.clone() {
                    Adt1::C0(_, h581, _, _, _, _, _, _, _, _, _) => {
                        h581
                    }
                };
                let a584 = match v1 {
                    Adt1::C0(_, _, _, h582, _, _, _, _, _, _, _) => {
                        h582
                    }
                };
                nat_eq(a583, a584)
            } else {
                false
            } {
                let a586 = match v3.clone() {
                    Adt1::C0(_, _, h585, _, _, _, _, _, _, _, _) => {
                        h585
                    }
                };
                let a587 = 1u64;
                nat_eq(a586, a587)
            } else {
                false
            } {
                let a589 = match v3.clone() {
                    Adt1::C0(_, _, _, h588, _, _, _, _, _, _, _) => {
                        h588
                    }
                };
                let a590 = 1u64;
                nat_eq(a589, a590)
            } else {
                false
            } {
                let a592 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h591, _, _, _, _, _, _) => {
                        h591
                    }
                };
                let a593 = 1u64;
                nat_eq(a592, a593)
            } else {
                false
            } {
                f11(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f24(v0: u64) -> u64 {
    if f26(v0) {
        8u64
    } else {
        f27(v0)
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> bool {
    let m594 = f28(v0);
    match m594 {
        Adt6::C0 => {
            false
        }
        Adt6::C1(v3) => {
            let a595 = v0;
            let a596 = 0u64;
            if if if if if nat_eq(a595, a596) {
                true
            } else {
                let a597 = v0;
                let a598 = 1u64;
                nat_eq(a597, a598)
            } {
                true
            } else {
                let a599 = v0;
                let a600 = 30u64;
                nat_eq(a599, a600)
            } {
                true
            } else {
                f26(v0)
            } {
                let a602 = v2;
                let a603 = match v3.clone() {
                    Adt7::C0(_, _, h601) => {
                        h601
                    }
                };
                nat_eq(a602, a603)
            } else {
                false
            } {
                f18(v1, match v3 {
                    Adt7::C0(_, h604, _) => {
                        h604
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f26(v0: u64) -> bool {
    let a605 = v0;
    let a606 = 2u64;
    if nat_eq(a605, a606) {
        true
    } else {
        let a607 = v0;
        let a608 = 8u64;
        nat_eq(a607, a608)
    }
}

pub fn f27(v0: u64) -> u64 {
    let a609 = v0;
    let a610 = 30u64;
    if nat_eq(a609, a610) {
        30u64
    } else {
        0u64
    }
}

pub fn f28(v0: u64) -> Adt6 {
    let a611 = v0;
    let a612 = 0u64;
    if nat_eq(a611, a612) {
        Adt6::C1(Adt7::C0(0u64, 1u64, 4u64))
    } else {
        let a613 = v0;
        let a614 = 1u64;
        if nat_eq(a613, a614) {
            Adt6::C1(Adt7::C0(1u64, 1u64, 2u64))
        } else {
            let a615 = v0;
            let a616 = 2u64;
            if nat_eq(a615, a616) {
                Adt6::C1(Adt7::C0(2u64, 32u64, 18u64))
            } else {
                let a617 = v0;
                let a618 = 3u64;
                if nat_eq(a617, a618) {
                    Adt6::C1(Adt7::C0(3u64, 32u64, 20u64))
                } else {
                    let a619 = v0;
                    let a620 = 6u64;
                    if nat_eq(a619, a620) {
                        Adt6::C1(Adt7::C0(6u64, 32u64, 22u64))
                    } else {
                        let a621 = v0;
                        let a622 = 7u64;
                        if nat_eq(a621, a622) {
                            Adt6::C1(Adt7::C0(7u64, 32u64, 24u64))
                        } else {
                            let a623 = v0;
                            let a624 = 8u64;
                            if nat_eq(a623, a624) {
                                Adt6::C1(Adt7::C0(8u64, 32u64, 34u64))
                            } else {
                                let a625 = v0;
                                let a626 = 9u64;
                                if nat_eq(a625, a626) {
                                    Adt6::C1(Adt7::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a627 = v0;
                                    let a628 = 10u64;
                                    if nat_eq(a627, a628) {
                                        Adt6::C1(Adt7::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a629 = v0;
                                        let a630 = 11u64;
                                        if nat_eq(a629, a630) {
                                            Adt6::C1(Adt7::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a631 = v0;
                                            let a632 = 12u64;
                                            if nat_eq(a631, a632) {
                                                Adt6::C1(Adt7::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a633 = v0;
                                                let a634 = 13u64;
                                                if nat_eq(a633, a634) {
                                                    Adt6::C1(Adt7::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a635 = v0;
                                                    let a636 = 14u64;
                                                    if nat_eq(a635, a636) {
                                                        Adt6::C1(Adt7::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a637 = v0;
                                                        let a638 = 15u64;
                                                        if nat_eq(a637, a638) {
                                                            Adt6::C1(Adt7::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a639 = v0;
                                                            let a640 = 16u64;
                                                            if nat_eq(a639, a640) {
                                                                Adt6::C1(Adt7::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a641 = v0;
                                                                let a642 = 17u64;
                                                                if nat_eq(a641, a642) {
                                                                    Adt6::C1(Adt7::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a643 = v0;
                                                                    let a644 = 18u64;
                                                                    if nat_eq(a643, a644) {
                                                                        Adt6::C1(Adt7::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a645 = v0;
                                                                        let a646 = 19u64;
                                                                        if nat_eq(a645, a646) {
                                                                            Adt6::C1(Adt7::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a647 = v0;
                                                                            let a648 = 20u64;
                                                                            if nat_eq(a647, a648) {
                                                                                Adt6::C1(Adt7::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a649 = v0;
                                                                                let a650 = 21u64;
                                                                                if nat_eq(a649, a650) {
                                                                                    Adt6::C1(Adt7::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a651 = v0;
                                                                                    let a652 = 22u64;
                                                                                    if nat_eq(a651, a652) {
                                                                                        Adt6::C1(Adt7::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a653 = v0;
                                                                                        let a654 = 23u64;
                                                                                        if nat_eq(a653, a654) {
                                                                                            Adt6::C1(Adt7::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a655 = v0;
                                                                                            let a656 = 24u64;
                                                                                            if nat_eq(a655, a656) {
                                                                                                Adt6::C1(Adt7::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a657 = v0;
                                                                                                let a658 = 25u64;
                                                                                                if nat_eq(a657, a658) {
                                                                                                    Adt6::C1(Adt7::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a659 = v0;
                                                                                                    let a660 = 26u64;
                                                                                                    if nat_eq(a659, a660) {
                                                                                                        Adt6::C1(Adt7::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a661 = v0;
                                                                                                        let a662 = 27u64;
                                                                                                        if nat_eq(a661, a662) {
                                                                                                            Adt6::C1(Adt7::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a663 = v0;
                                                                                                            let a664 = 28u64;
                                                                                                            if nat_eq(a663, a664) {
                                                                                                                Adt6::C1(Adt7::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a665 = v0;
                                                                                                                let a666 = 29u64;
                                                                                                                if nat_eq(a665, a666) {
                                                                                                                    Adt6::C1(Adt7::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a667 = v0;
                                                                                                                    let a668 = 30u64;
                                                                                                                    if nat_eq(a667, a668) {
                                                                                                                        Adt6::C1(Adt7::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a669 = v0;
                                                                                                                        let a670 = 34u64;
                                                                                                                        if nat_eq(a669, a670) {
                                                                                                                            Adt6::C1(Adt7::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a671 = v0;
                                                                                                                            let a672 = 35u64;
                                                                                                                            if nat_eq(a671, a672) {
                                                                                                                                Adt6::C1(Adt7::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a673 = v0;
                                                                                                                                let a674 = 39u64;
                                                                                                                                if nat_eq(a673, a674) {
                                                                                                                                    Adt6::C1(Adt7::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a675 = v0;
                                                                                                                                    let a676 = 40u64;
                                                                                                                                    if nat_eq(a675, a676) {
                                                                                                                                        Adt6::C1(Adt7::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a677 = v0;
                                                                                                                                        let a678 = 41u64;
                                                                                                                                        if nat_eq(a677, a678) {
                                                                                                                                            Adt6::C1(Adt7::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a679 = v0;
                                                                                                                                            let a680 = 42u64;
                                                                                                                                            if nat_eq(a679, a680) {
                                                                                                                                                Adt6::C1(Adt7::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a681 = v0;
                                                                                                                                                let a682 = 43u64;
                                                                                                                                                if nat_eq(a681, a682) {
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
