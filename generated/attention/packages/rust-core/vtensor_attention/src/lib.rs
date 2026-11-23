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
        f17({
            let a70 = v8;
            let a71 = match v7.clone() {
                Adt1::C0(_, h69, _, _, _, _, _, _, _, _, _) => {
                    h69
                }
            };
            nat_mul(a70, a71)?
        }, match v7.clone() {
            Adt1::C0(_, _, h72, _, _, _, _, _, _, _, _) => {
                h72
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                h73
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
                h74
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h75, _, _, _, _) => {
                h75
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h76, _, _, _) => {
                h76
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h77, _, _) => {
                h77
            }
        })?
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
            Adt1::C0(_, _, h78, _, _, _, _, _, _, _, _) => {
                h78
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h79, _, _, _, _, _, _, _, _) => {
                h79
            }
        })
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, _, h80, _, _, _, _, _, _, _) => {
                h80
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h81, _, _, _, _, _, _, _) => {
                h81
            }
        })
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, _, _, h82, _, _, _, _, _, _) => {
                h82
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h83, _, _, _, _, _, _) => {
                h83
            }
        })
    } else {
        false
    } {
        let m84 = v4.clone();
        match m84 {
            None => {
                true
            }
            Some(v9) => {
                let a86 = match v9.clone() {
                    Adt1::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                        h85
                    }
                };
                let a87 = 0u64;
                if if nat_eq(a86, a87) {
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
        let m88 = v5.clone();
        match m88 {
            None => {
                true
            }
            Some(v10) => {
                let a90 = match v10.clone() {
                    Adt1::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                        h89
                    }
                };
                let a91 = 26u64;
                if if if if nat_eq(a90, a91) {
                    let a93 = match v10.clone() {
                        Adt1::C0(_, _, _, h92, _, _, _, _, _, _, _) => {
                            h92
                        }
                    };
                    let a94 = 1u64;
                    nat_eq(a93, a94)
                } else {
                    false
                } {
                    let a96 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h95, _, _, _, _, _, _) => {
                            h95
                        }
                    };
                    let a97 = 1u64;
                    nat_eq(a96, a97)
                } else {
                    false
                } {
                    f11(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m98 = v4.clone();
                    match m98 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a101 = match v11 {
                                Adt1::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                                    h99
                                }
                            };
                            let a102 = match v10 {
                                Adt1::C0(_, _, h100, _, _, _, _, _, _, _, _) => {
                                    h100
                                }
                            };
                            nat_eq(a101, a102)
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
        let m103 = v6.clone();
        match m103 {
            None => {
                true
            }
            Some(v12) => {
                let a105 = match v12.clone() {
                    Adt1::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                };
                let a106 = 26u64;
                if if nat_eq(a105, a106) {
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
            let a107 = v8;
            let a108 = 2u64;
            if nat_eq(a107, a108) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m109 = v5;
            match m109 {
                None => {
                    let m110 = v4;
                    match m110 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h111, _, _, _, _, _, _, _, _, _) => {
                                    h111
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
                                    h112
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
                            h113
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
        Adt4::C0(h114, _, _, _, _, _, _, _, _) => {
            h114
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h115, _, _, _, _, _, _, _) => {
            h115
        }
    };
    let a117 = v1;
    let a118 = match v5.clone() {
        Adt1::C0(_, _, h116, _, _, _, _, _, _, _, _) => {
            h116
        }
    };
    if if if nat_lt(a117, a118) {
        let a120 = v2;
        let a121 = match v5.clone() {
            Adt1::C0(_, _, _, h119, _, _, _, _, _, _, _) => {
                h119
            }
        };
        nat_lt(a120, a121)
    } else {
        false
    } {
        let a123 = v3;
        let a124 = match v5.clone() {
            Adt1::C0(_, _, _, _, h122, _, _, _, _, _, _) => {
                h122
            }
        };
        nat_lt(a123, a124)
    } else {
        false
    } {
        Ok(Adt5::C6(f19(v4.clone(), {
            let a126 = v1;
            let a127 = match v4.clone() {
                Adt1::C0(_, _, h125, _, _, _, _, _, _, _, _) => {
                    h125
                }
            };
            let a128 = 0u64;
            nat_rem(a126, a127, a128)
        }, {
            let a130 = v2;
            let a131 = match v4.clone() {
                Adt1::C0(_, _, _, h129, _, _, _, _, _, _, _) => {
                    h129
                }
            };
            let a132 = 0u64;
            nat_rem(a130, a131, a132)
        }, {
            let a134 = v3;
            let a135 = match v4 {
                Adt1::C0(_, _, _, _, h133, _, _, _, _, _, _) => {
                    h133
                }
            };
            let a136 = 0u64;
            nat_rem(a134, a135, a136)
        })?, f19(v5, v1, v2, v3)?, {
            let m138 = match v0.clone() {
                Adt4::C0(_, _, h137, _, _, _, _, _, _) => {
                    h137
                }
            };
            match m138 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f19(v6, v1, v2, v3)?
                }
            }
        }, {
            let m140 = match v0.clone() {
                Adt4::C0(_, _, _, _, h139, _, _, _, _) => {
                    h139
                }
            };
            match m140 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f19(v7, v1, v2, v3)?
                }
            }
        }, f19(match v0 {
            Adt4::C0(_, _, _, _, _, h141, _, _, _) => {
                h141
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f4(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a142 = v1;
    let a143 = 0u64;
    if nat_eq(a142, a143) {
        let a147 = v2;
        let a148 = {
            let m146 = match v0.clone() {
                Adt4::C0(_, h145, _, _, _, _, _, _, _) => {
                    h145
                }
            };
            match m146 {
                Adt1::C0(_, h144, _, _, _, _, _, _, _, _, _) => {
                    h144
                }
            }
        };
        if nat_lt(a147, a148) {
            Ok(Adt5::C7({
                let a152 = v2;
                let a153 = {
                    let m151 = match v0 {
                        Adt4::C0(_, h150, _, _, _, _, _, _, _) => {
                            h150
                        }
                    };
                    match m151 {
                        Adt1::C0(_, _, _, _, _, h149, _, _, _, _, _) => {
                            h149
                        }
                    }
                };
                nat_mul(a152, a153)?
            }))
        } else {
            Ok(Adt5::C8)
        }
    } else {
        let a154 = v1;
        let a155 = 1u64;
        if nat_eq(a154, a155) {
            let m157 = match v0 {
                Adt4::C0(_, _, _, _, h156, _, _, _, _) => {
                    h156
                }
            };
            match m157 {
                None => {
                    Ok(Adt5::C8)
                }
                Some(v4) => {
                    let a159 = v2;
                    let a160 = match v4.clone() {
                        Adt1::C0(_, h158, _, _, _, _, _, _, _, _, _) => {
                            h158
                        }
                    };
                    if nat_lt(a159, a160) {
                        Ok(Adt5::C7({
                            let a162 = v2;
                            let a163 = match v4 {
                                Adt1::C0(_, _, _, _, _, h161, _, _, _, _, _) => {
                                    h161
                                }
                            };
                            nat_mul(a162, a163)?
                        }))
                    } else {
                        Ok(Adt5::C8)
                    }
                }
            }
        } else {
            let a164 = v1;
            let a165 = 2u64;
            if nat_eq(a164, a165) {
                let a166 = v2;
                let a167 = 2147483648u64;
                if if nat_lt(a166, a167) {
                    let a169 = v2;
                    let a170 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h168) => {
                            h168
                        }
                    };
                    nat_lt(a169, a170)
                } else {
                    false
                } {
                    let m172 = match v0 {
                        Adt4::C0(_, _, h171, _, _, _, _, _, _) => {
                            h171
                        }
                    };
                    match m172 {
                        None => {
                            Ok(Adt5::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt5::C7({
                                let a174 = v2;
                                let a175 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h173, _, _, _, _, _) => {
                                        h173
                                    }
                                };
                                nat_mul(a174, a175)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt5::C8)
                }
            } else {
                let a176 = v1;
                let a177 = 3u64;
                if nat_eq(a176, a177) {
                    let m179 = match v0 {
                        Adt4::C0(_, _, _, h178, _, _, _, _, _) => {
                            h178
                        }
                    };
                    match m179 {
                        None => {
                            Ok(Adt5::C8)
                        }
                        Some(v6) => {
                            let a181 = v2;
                            let a182 = match v6.clone() {
                                Adt1::C0(_, _, h180, _, _, _, _, _, _, _, _) => {
                                    h180
                                }
                            };
                            if if nat_lt(a181, a182) {
                                let a184 = v3;
                                let a185 = match v6.clone() {
                                    Adt1::C0(_, h183, _, _, _, _, _, _, _, _, _) => {
                                        h183
                                    }
                                };
                                nat_lt(a184, a185)
                            } else {
                                false
                            } {
                                Ok(Adt5::C7({
                                    let a192 = {
                                        let a187 = v2;
                                        let a188 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h186, _, _, _, _) => {
                                                h186
                                            }
                                        };
                                        nat_mul(a187, a188)?
                                    };
                                    let a193 = {
                                        let a190 = v3;
                                        let a191 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h189, _, _, _, _, _) => {
                                                h189
                                            }
                                        };
                                        nat_mul(a190, a191)?
                                    };
                                    nat_add(a192, a193)?
                                }))
                            } else {
                                Ok(Adt5::C8)
                            }
                        }
                    }
                } else {
                    let a194 = v1;
                    let a195 = 4u64;
                    if nat_eq(a194, a195) {
                        let a196 = v2;
                        let a197 = 2147483648u64;
                        if if nat_lt(a196, a197) {
                            let a201 = v2;
                            let a202 = {
                                let m200 = match v0.clone() {
                                    Adt4::C0(h199, _, _, _, _, _, _, _, _) => {
                                        h199
                                    }
                                };
                                match m200 {
                                    Adt1::C0(_, h198, _, _, _, _, _, _, _, _, _) => {
                                        h198
                                    }
                                }
                            };
                            nat_lt(a201, a202)
                        } else {
                            false
                        } {
                            Ok(Adt5::C7({
                                let a204 = v2;
                                let a205 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h203, _, _) => {
                                        h203
                                    }
                                };
                                nat_mul(a204, a205)?
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
        let a206 = v0;
        let a207 = 43u64;
        if if if if if if if if nat_eq(a206, a207) {
            let a209 = match v4.clone() {
                Adt1::C0(h208, _, _, _, _, _, _, _, _, _, _) => {
                    h208
                }
            };
            let a210 = 0u64;
            if nat_eq(a209, a210) {
                f20(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a212 = match v7.clone() {
                Adt1::C0(h211, _, _, _, _, _, _, _, _, _, _) => {
                    h211
                }
            };
            let a213 = 0u64;
            if nat_eq(a212, a213) {
                f20(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f21(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f11(v4.clone(), v7.clone())? {
                true
            } else {
                f22(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f23(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f24(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f8(v3, match v4.clone() {
                Adt1::C0(_, _, _, h214, _, _, _, _, _, _, _) => {
                    h214
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
        Adt3::C0(h215, _, _, _, _) => {
            h215
        }
    };
    let a217 = v1;
    let a218 = match v4.clone() {
        Adt1::C0(_, _, h216, _, _, _, _, _, _, _, _) => {
            h216
        }
    };
    if if if nat_lt(a217, a218) {
        let a220 = v2;
        let a221 = match v4.clone() {
            Adt1::C0(_, _, _, h219, _, _, _, _, _, _, _) => {
                h219
            }
        };
        nat_lt(a220, a221)
    } else {
        false
    } {
        let a223 = v3;
        let a224 = match v4.clone() {
            Adt1::C0(_, _, _, _, h222, _, _, _, _, _, _) => {
                h222
            }
        };
        nat_lt(a223, a224)
    } else {
        false
    } {
        let v6: u64 = {
            let m226 = match v0.clone() {
                Adt3::C0(_, h225, _, _, _) => {
                    h225
                }
            };
            match m226 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f19(v5.clone(), v1, {
                        let a228 = v2;
                        let a229 = match v5.clone() {
                            Adt1::C0(_, _, _, h227, _, _, _, _, _, _, _) => {
                                h227
                            }
                        };
                        let a230 = 0u64;
                        nat_rem(a228, a229, a230)
                    }, {
                        let a232 = v3;
                        let a233 = match v5 {
                            Adt1::C0(_, _, _, _, h231, _, _, _, _, _, _) => {
                                h231
                            }
                        };
                        let a234 = 0u64;
                        nat_rem(a232, a233, a234)
                    })?
                }
            }
        };
        Ok(Adt5::C4(f19(v4, v1, v2, v3)?, v6, {
            let a235 = v2;
            let a236 = 4u64;
            nat_mul(a235, a236)?
        }, f19(match v0 {
            Adt3::C0(_, _, h237, _, _) => {
                h237
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f7(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a241 = v1;
    let a242 = {
        let m240 = match v0.clone() {
            Adt2::C0(h239, _, _, _, _, _, _, _) => {
                h239
            }
        };
        match m240 {
            Adt1::C0(_, _, h238, _, _, _, _, _, _, _, _) => {
                h238
            }
        }
    };
    if if if nat_lt(a241, a242) {
        let a246 = v2;
        let a247 = {
            let m245 = match v0.clone() {
                Adt2::C0(h244, _, _, _, _, _, _, _) => {
                    h244
                }
            };
            match m245 {
                Adt1::C0(_, _, _, h243, _, _, _, _, _, _, _) => {
                    h243
                }
            }
        };
        nat_lt(a246, a247)
    } else {
        false
    } {
        let a251 = v3;
        let a252 = {
            let m250 = match v0.clone() {
                Adt2::C0(h249, _, _, _, _, _, _, _) => {
                    h249
                }
            };
            match m250 {
                Adt1::C0(_, _, _, _, h248, _, _, _, _, _, _) => {
                    h248
                }
            }
        };
        nat_lt(a251, a252)
    } else {
        false
    } {
        Ok(Adt5::C2(f19(match v0.clone() {
            Adt2::C0(h253, _, _, _, _, _, _, _) => {
                h253
            }
        }, v1, v2, v3)?, f19(match v0.clone() {
            Adt2::C0(_, h254, _, _, _, _, _, _) => {
                h254
            }
        }, 0u64, {
            let a264 = v2;
            let a265 = {
                let a261 = {
                    let m257 = match v0.clone() {
                        Adt2::C0(h256, _, _, _, _, _, _, _) => {
                            h256
                        }
                    };
                    match m257 {
                        Adt1::C0(_, _, _, h255, _, _, _, _, _, _, _) => {
                            h255
                        }
                    }
                };
                let a262 = {
                    let m260 = match v0.clone() {
                        Adt2::C0(_, h259, _, _, _, _, _, _) => {
                            h259
                        }
                    };
                    match m260 {
                        Adt1::C0(_, _, _, h258, _, _, _, _, _, _, _) => {
                            h258
                        }
                    }
                };
                let a263 = 0u64;
                nat_quot(a261, a262, a263)
            };
            let a266 = 0u64;
            nat_quot(a264, a265, a266)
        }, {
            let a276 = v3;
            let a277 = {
                let a273 = {
                    let m269 = match v0.clone() {
                        Adt2::C0(h268, _, _, _, _, _, _, _) => {
                            h268
                        }
                    };
                    match m269 {
                        Adt1::C0(_, _, _, _, h267, _, _, _, _, _, _) => {
                            h267
                        }
                    }
                };
                let a274 = {
                    let m272 = match v0.clone() {
                        Adt2::C0(_, h271, _, _, _, _, _, _) => {
                            h271
                        }
                    };
                    match m272 {
                        Adt1::C0(_, _, _, _, h270, _, _, _, _, _, _) => {
                            h270
                        }
                    }
                };
                let a275 = 0u64;
                nat_quot(a273, a274, a275)
            };
            let a278 = 0u64;
            nat_quot(a276, a277, a278)
        })?, f19(match v0.clone() {
            Adt2::C0(_, _, h279, _, _, _, _, _) => {
                h279
            }
        }, 0u64, {
            let a289 = v2;
            let a290 = {
                let a286 = {
                    let m282 = match v0.clone() {
                        Adt2::C0(h281, _, _, _, _, _, _, _) => {
                            h281
                        }
                    };
                    match m282 {
                        Adt1::C0(_, _, _, h280, _, _, _, _, _, _, _) => {
                            h280
                        }
                    }
                };
                let a287 = {
                    let m285 = match v0.clone() {
                        Adt2::C0(_, _, h284, _, _, _, _, _) => {
                            h284
                        }
                    };
                    match m285 {
                        Adt1::C0(_, _, _, h283, _, _, _, _, _, _, _) => {
                            h283
                        }
                    }
                };
                let a288 = 0u64;
                nat_quot(a286, a287, a288)
            };
            let a291 = 0u64;
            nat_quot(a289, a290, a291)
        }, {
            let a301 = v3;
            let a302 = {
                let a298 = {
                    let m294 = match v0.clone() {
                        Adt2::C0(h293, _, _, _, _, _, _, _) => {
                            h293
                        }
                    };
                    match m294 {
                        Adt1::C0(_, _, _, _, h292, _, _, _, _, _, _) => {
                            h292
                        }
                    }
                };
                let a299 = {
                    let m297 = match v0.clone() {
                        Adt2::C0(_, _, h296, _, _, _, _, _) => {
                            h296
                        }
                    };
                    match m297 {
                        Adt1::C0(_, _, _, _, h295, _, _, _, _, _, _) => {
                            h295
                        }
                    }
                };
                let a300 = 0u64;
                nat_quot(a298, a299, a300)
            };
            let a303 = 0u64;
            nat_quot(a301, a302, a303)
        })?, {
            let m305 = match v0.clone() {
                Adt2::C0(_, _, _, h304, _, _, _, _) => {
                    h304
                }
            };
            match m305 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f19(v4.clone(), v1, {
                        let a307 = v2;
                        let a308 = match v4.clone() {
                            Adt1::C0(_, _, _, h306, _, _, _, _, _, _, _) => {
                                h306
                            }
                        };
                        let a309 = 0u64;
                        nat_rem(a307, a308, a309)
                    }, {
                        let a311 = v3;
                        let a312 = match v4 {
                            Adt1::C0(_, _, _, _, h310, _, _, _, _, _, _) => {
                                h310
                            }
                        };
                        let a313 = 0u64;
                        nat_rem(a311, a312, a313)
                    })?
                }
            }
        }, {
            let a314 = v2;
            let a315 = 4u64;
            nat_mul(a314, a315)?
        }, f19(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h316, _, _) => {
                h316
            }
        }, v2, v1, v3)?, {
            let m319 = match v0.clone() {
                Adt2::C0(_, h318, _, _, _, _, _, _) => {
                    h318
                }
            };
            match m319 {
                Adt1::C0(_, _, _, _, _, _, h317, _, _, _, _) => {
                    h317
                }
            }
        }, {
            let m322 = match v0.clone() {
                Adt2::C0(_, _, h321, _, _, _, _, _) => {
                    h321
                }
            };
            match m322 {
                Adt1::C0(_, _, _, _, _, _, h320, _, _, _, _) => {
                    h320
                }
            }
        }, f25({
            let m325 = match v0 {
                Adt2::C0(_, h324, _, _, _, _, _, _) => {
                    h324
                }
            };
            match m325 {
                Adt1::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
                    h323
                }
            }
        })))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f8(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a326 = v0;
    let a327 = 0u64;
    if if nat_eq(a326, a327) {
        true
    } else {
        let a328 = 2147483648u64;
        let a329 = v0;
        if nat_le(a328, a329) {
            let a330 = v0;
            let a331 = 4286578688u64;
            nat_lt(a330, a331)
        } else {
            false
        }
    } {
        true
    } else {
        let a332 = v0;
        let a333 = 2139095040u64;
        if if nat_lt(a332, a333) {
            let a334 = v1;
            let a335 = 4294967295u64;
            nat_le(a334, a335)
        } else {
            false
        } {
            let m336 = v2;
            match m336 {
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
    let a338 = match v0.clone() {
        Adt1::C0(h337, _, _, _, _, _, _, _, _, _, _) => {
            h337
        }
    };
    let a339 = 0u64;
    if if if if nat_eq(a338, a339) {
        let a341 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h340, _, _, _, _, _) => {
                h340
            }
        };
        let a342 = 4u64;
        nat_eq(a341, a342)
    } else {
        false
    } {
        let a348 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h343, _, _, _, _) => {
                h343
            }
        };
        let a349 = {
            let a346 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h344, _, _, _, _, _) => {
                    h344
                }
            };
            let a347 = match v0.clone() {
                Adt1::C0(_, h345, _, _, _, _, _, _, _, _, _) => {
                    h345
                }
            };
            nat_mul(a346, a347)?
        };
        nat_eq(a348, a349)
    } else {
        false
    } {
        let a355 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h350, _, _, _) => {
                h350
            }
        };
        let a356 = {
            let a353 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h351, _, _, _, _) => {
                    h351
                }
            };
            let a354 = match v0.clone() {
                Adt1::C0(_, _, h352, _, _, _, _, _, _, _, _) => {
                    h352
                }
            };
            nat_mul(a353, a354)?
        };
        nat_eq(a355, a356)
    } else {
        false
    } {
        let a362 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h357, _, _) => {
                h357
            }
        };
        let a363 = {
            let a360 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h358, _, _, _) => {
                    h358
                }
            };
            let a361 = match v0 {
                Adt1::C0(_, _, _, h359, _, _, _, _, _, _, _) => {
                    h359
                }
            };
            nat_mul(a360, a361)?
        };
        Ok(nat_eq(a362, a363))
    } else {
        Ok(false)
    }
}

pub fn f10(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f26(match v0.clone() {
        Adt1::C0(h364, _, _, _, _, _, _, _, _, _, _) => {
            h364
        }
    }, match v0.clone() {
        Adt1::C0(_, h365, _, _, _, _, _, _, _, _, _) => {
            h365
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h366, _, _, _, _, _) => {
            h366
        }
    }) {
        f18(match v1.clone() {
            Adt1::C0(_, _, _, h367, _, _, _, _, _, _, _) => {
                h367
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h368, _, _, _, _, _, _, _) => {
                h368
            }
        })
    } else {
        false
    } {
        f18(match v1 {
            Adt1::C0(_, _, _, _, h369, _, _, _, _, _, _) => {
                h369
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h370, _, _, _, _, _, _) => {
                h370
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
    let a372 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h371, _) => {
            h371
        }
    };
    let a373 = 0u64;
    if nat_eq(a372, a373) {
        Ok(true)
    } else {
        let a375 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h374, _) => {
                h374
            }
        };
        let a376 = 0u64;
        if nat_eq(a375, a376) {
            Ok(true)
        } else {
            let a378 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h377) => {
                    h377
                }
            };
            let a379 = 0u64;
            if nat_eq(a378, a379) {
                Ok(true)
            } else {
                let a381 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h380) => {
                        h380
                    }
                };
                let a382 = 0u64;
                if nat_eq(a381, a382) {
                    Ok(true)
                } else {
                    let a388 = {
                        let a385 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h383) => {
                                h383
                            }
                        };
                        let a386 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h384, _) => {
                                h384
                            }
                        };
                        nat_add(a385, a386)?
                    };
                    let a389 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h387) => {
                            h387
                        }
                    };
                    if nat_le(a388, a389) {
                        Ok(true)
                    } else {
                        let a395 = {
                            let a392 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h390) => {
                                    h390
                                }
                            };
                            let a393 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h391, _) => {
                                    h391
                                }
                            };
                            nat_add(a392, a393)?
                        };
                        let a396 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h394) => {
                                h394
                            }
                        };
                        Ok(nat_le(a395, a396))
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m397 = v0;
    match m397 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a399 = match v4.clone() {
                Adt1::C0(h398, _, _, _, _, _, _, _, _, _, _) => {
                    h398
                }
            };
            let a400 = 1u64;
            if if if if if if nat_eq(a399, a400) {
                let a402 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h401, _, _, _, _, _) => {
                        h401
                    }
                };
                let a403 = 2u64;
                nat_eq(a402, a403)
            } else {
                false
            } {
                let a406 = match v2 {
                    Adt1::C0(_, _, h404, _, _, _, _, _, _, _, _) => {
                        h404
                    }
                };
                let a407 = match v4.clone() {
                    Adt1::C0(_, h405, _, _, _, _, _, _, _, _, _) => {
                        h405
                    }
                };
                nat_le(a406, a407)
            } else {
                false
            } {
                let a410 = match v1.clone() {
                    Adt1::C0(_, _, h408, _, _, _, _, _, _, _, _) => {
                        h408
                    }
                };
                let a411 = match v4.clone() {
                    Adt1::C0(_, _, h409, _, _, _, _, _, _, _, _) => {
                        h409
                    }
                };
                nat_le(a410, a411)
            } else {
                false
            } {
                f18(match v1.clone() {
                    Adt1::C0(_, _, _, h412, _, _, _, _, _, _, _) => {
                        h412
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h413, _, _, _, _, _, _, _) => {
                        h413
                    }
                })
            } else {
                false
            } {
                f18(match v1 {
                    Adt1::C0(_, _, _, _, h414, _, _, _, _, _, _) => {
                        h414
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h415, _, _, _, _, _, _) => {
                        h415
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
    let m416 = v0;
    match m416 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a418 = match v3.clone() {
                Adt1::C0(h417, _, _, _, _, _, _, _, _, _, _) => {
                    h417
                }
            };
            let a419 = 0u64;
            if if if if if if nat_eq(a418, a419) {
                let a421 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h420, _, _, _, _, _) => {
                        h420
                    }
                };
                let a422 = 4u64;
                nat_eq(a421, a422)
            } else {
                false
            } {
                let a425 = match v3.clone() {
                    Adt1::C0(_, h423, _, _, _, _, _, _, _, _, _) => {
                        h423
                    }
                };
                let a426 = match v1 {
                    Adt1::C0(_, _, _, h424, _, _, _, _, _, _, _) => {
                        h424
                    }
                };
                nat_eq(a425, a426)
            } else {
                false
            } {
                let a428 = match v3.clone() {
                    Adt1::C0(_, _, h427, _, _, _, _, _, _, _, _) => {
                        h427
                    }
                };
                let a429 = 1u64;
                nat_eq(a428, a429)
            } else {
                false
            } {
                let a431 = match v3.clone() {
                    Adt1::C0(_, _, _, h430, _, _, _, _, _, _, _) => {
                        h430
                    }
                };
                let a432 = 1u64;
                nat_eq(a431, a432)
            } else {
                false
            } {
                let a434 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h433, _, _, _, _, _, _) => {
                        h433
                    }
                };
                let a435 = 1u64;
                nat_eq(a434, a435)
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
    let a436 = 0u64;
    let a437 = v0;
    if if nat_lt(a436, a437) {
        let a438 = v0;
        let a439 = 2139095040u64;
        nat_lt(a438, a439)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f15(v0: u64) -> Option<u64> {
    let a440 = v0;
    let a441 = 0u64;
    if if nat_eq(a440, a441) {
        true
    } else {
        let a442 = v0;
        let a443 = 2147483648u64;
        nat_eq(a442, a443)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f16(v0: Adt1, v1: Adt1) -> bool {
    let a446 = match v0.clone() {
        Adt1::C0(_, _, h444, _, _, _, _, _, _, _, _) => {
            h444
        }
    };
    let a447 = match v1.clone() {
        Adt1::C0(_, _, h445, _, _, _, _, _, _, _, _) => {
            h445
        }
    };
    if if nat_eq(a446, a447) {
        let a450 = match v0.clone() {
            Adt1::C0(_, _, _, h448, _, _, _, _, _, _, _) => {
                h448
            }
        };
        let a451 = match v1.clone() {
            Adt1::C0(_, _, _, h449, _, _, _, _, _, _, _) => {
                h449
            }
        };
        nat_eq(a450, a451)
    } else {
        false
    } {
        let a454 = match v0 {
            Adt1::C0(_, _, _, _, h452, _, _, _, _, _, _) => {
                h452
            }
        };
        let a455 = match v1 {
            Adt1::C0(_, _, _, _, h453, _, _, _, _, _, _) => {
                h453
            }
        };
        nat_eq(a454, a455)
    } else {
        false
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f27(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f27(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f27(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f27(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f27(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f27(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f18(v0: u64, v1: u64) -> bool {
    let a456 = 0u64;
    let a457 = v1;
    if nat_lt(a456, a457) {
        let a461 = {
            let a458 = v0;
            let a459 = v1;
            let a460 = 0u64;
            nat_rem(a458, a459, a460)
        };
        let a462 = 0u64;
        nat_eq(a461, a462)
    } else {
        false
    }
}

pub fn f19(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a474 = {
        let a469 = {
            let a464 = v1;
            let a465 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h463, _, _, _, _) => {
                    h463
                }
            };
            nat_mul(a464, a465)?
        };
        let a470 = {
            let a467 = v2;
            let a468 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h466, _, _, _) => {
                    h466
                }
            };
            nat_mul(a467, a468)?
        };
        nat_add(a469, a470)?
    };
    let a475 = {
        let a472 = v3;
        let a473 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h471, _, _) => {
                h471
            }
        };
        nat_mul(a472, a473)?
    };
    nat_add(a474, a475)
}

pub fn f20(v0: Adt1, v1: u64) -> R<bool> {
    let a477 = match v0.clone() {
        Adt1::C0(_, h476, _, _, _, _, _, _, _, _, _) => {
            h476
        }
    };
    let a478 = 1u64;
    if if if if nat_eq(a477, a478) {
        true
    } else {
        let a480 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h479, _, _, _, _, _) => {
                h479
            }
        };
        let a481 = v1;
        nat_eq(a480, a481)
    } {
        let a483 = match v0.clone() {
            Adt1::C0(_, _, h482, _, _, _, _, _, _, _, _) => {
                h482
            }
        };
        let a484 = 1u64;
        if nat_eq(a483, a484) {
            true
        } else {
            let a489 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h485, _, _, _, _) => {
                    h485
                }
            };
            let a490 = {
                let a487 = v1;
                let a488 = match v0.clone() {
                    Adt1::C0(_, h486, _, _, _, _, _, _, _, _, _) => {
                        h486
                    }
                };
                nat_mul(a487, a488)?
            };
            nat_eq(a489, a490)
        }
    } else {
        false
    } {
        let a492 = match v0.clone() {
            Adt1::C0(_, _, _, h491, _, _, _, _, _, _, _) => {
                h491
            }
        };
        let a493 = 1u64;
        if nat_eq(a492, a493) {
            true
        } else {
            let a501 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h494, _, _, _) => {
                    h494
                }
            };
            let a502 = {
                let a499 = {
                    let a496 = v1;
                    let a497 = match v0.clone() {
                        Adt1::C0(_, h495, _, _, _, _, _, _, _, _, _) => {
                            h495
                        }
                    };
                    nat_mul(a496, a497)?
                };
                let a500 = match v0.clone() {
                    Adt1::C0(_, _, h498, _, _, _, _, _, _, _, _) => {
                        h498
                    }
                };
                nat_mul(a499, a500)?
            };
            nat_eq(a501, a502)
        }
    } else {
        false
    } {
        let a504 = match v0.clone() {
            Adt1::C0(_, _, _, _, h503, _, _, _, _, _, _) => {
                h503
            }
        };
        let a505 = 1u64;
        if nat_eq(a504, a505) {
            Ok(true)
        } else {
            let a516 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h506, _, _) => {
                    h506
                }
            };
            let a517 = {
                let a514 = {
                    let a511 = {
                        let a508 = v1;
                        let a509 = match v0.clone() {
                            Adt1::C0(_, h507, _, _, _, _, _, _, _, _, _) => {
                                h507
                            }
                        };
                        nat_mul(a508, a509)?
                    };
                    let a512 = match v0.clone() {
                        Adt1::C0(_, _, h510, _, _, _, _, _, _, _, _) => {
                            h510
                        }
                    };
                    nat_mul(a511, a512)?
                };
                let a515 = match v0 {
                    Adt1::C0(_, _, _, h513, _, _, _, _, _, _, _) => {
                        h513
                    }
                };
                nat_mul(a514, a515)?
            };
            Ok(nat_eq(a516, a517))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> bool {
    let a520 = match v0.clone() {
        Adt1::C0(_, h518, _, _, _, _, _, _, _, _, _) => {
            h518
        }
    };
    let a521 = match v1.clone() {
        Adt1::C0(_, h519, _, _, _, _, _, _, _, _, _) => {
            h519
        }
    };
    if if if nat_eq(a520, a521) {
        let a524 = match v0.clone() {
            Adt1::C0(_, _, h522, _, _, _, _, _, _, _, _) => {
                h522
            }
        };
        let a525 = match v1.clone() {
            Adt1::C0(_, _, h523, _, _, _, _, _, _, _, _) => {
                h523
            }
        };
        nat_eq(a524, a525)
    } else {
        false
    } {
        let a528 = match v0.clone() {
            Adt1::C0(_, _, _, h526, _, _, _, _, _, _, _) => {
                h526
            }
        };
        let a529 = match v1.clone() {
            Adt1::C0(_, _, _, h527, _, _, _, _, _, _, _) => {
                h527
            }
        };
        nat_eq(a528, a529)
    } else {
        false
    } {
        let a532 = match v0 {
            Adt1::C0(_, _, _, _, h530, _, _, _, _, _, _) => {
                h530
            }
        };
        let a533 = match v1 {
            Adt1::C0(_, _, _, _, h531, _, _, _, _, _, _) => {
                h531
            }
        };
        nat_eq(a532, a533)
    } else {
        false
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f21(v0.clone(), v1.clone()) {
        let a536 = match v0.clone() {
            Adt1::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                h534
            }
        };
        let a537 = match v1.clone() {
            Adt1::C0(h535, _, _, _, _, _, _, _, _, _, _) => {
                h535
            }
        };
        nat_eq(a536, a537)
    } else {
        false
    } {
        let a540 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h538) => {
                h538
            }
        };
        let a541 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h539) => {
                h539
            }
        };
        nat_eq(a540, a541)
    } else {
        false
    } {
        let a544 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h542, _, _, _, _, _) => {
                h542
            }
        };
        let a545 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h543, _, _, _, _, _) => {
                h543
            }
        };
        nat_eq(a544, a545)
    } else {
        false
    } {
        let a548 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h546, _, _, _, _) => {
                h546
            }
        };
        let a549 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h547, _, _, _, _) => {
                h547
            }
        };
        nat_eq(a548, a549)
    } else {
        false
    } {
        let a552 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h550, _, _, _) => {
                h550
            }
        };
        let a553 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h551, _, _, _) => {
                h551
            }
        };
        nat_eq(a552, a553)
    } else {
        false
    } {
        let a556 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h554, _, _) => {
                h554
            }
        };
        let a557 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h555, _, _) => {
                h555
            }
        };
        nat_eq(a556, a557)
    } else {
        false
    }
}

pub fn f23(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m558 = v0;
    match m558 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a560 = match v3.clone() {
                Adt1::C0(h559, _, _, _, _, _, _, _, _, _, _) => {
                    h559
                }
            };
            let a561 = 0u64;
            if if if if if if if nat_eq(a560, a561) {
                true
            } else {
                let a563 = match v3.clone() {
                    Adt1::C0(h562, _, _, _, _, _, _, _, _, _, _) => {
                        h562
                    }
                };
                let a564 = 1u64;
                nat_eq(a563, a564)
            } {
                f20(v3.clone(), {
                    let a566 = match v3.clone() {
                        Adt1::C0(h565, _, _, _, _, _, _, _, _, _, _) => {
                            h565
                        }
                    };
                    let a567 = 0u64;
                    if nat_eq(a566, a567) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a570 = match v3.clone() {
                    Adt1::C0(_, h568, _, _, _, _, _, _, _, _, _) => {
                        h568
                    }
                };
                let a571 = match v1.clone() {
                    Adt1::C0(_, h569, _, _, _, _, _, _, _, _, _) => {
                        h569
                    }
                };
                nat_eq(a570, a571)
            } else {
                false
            } {
                let a574 = match v1.clone() {
                    Adt1::C0(_, _, h572, _, _, _, _, _, _, _, _) => {
                        h572
                    }
                };
                let a575 = match v3.clone() {
                    Adt1::C0(_, _, h573, _, _, _, _, _, _, _, _) => {
                        h573
                    }
                };
                nat_le(a574, a575)
            } else {
                false
            } {
                f18(match v1.clone() {
                    Adt1::C0(_, _, _, h576, _, _, _, _, _, _, _) => {
                        h576
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h577, _, _, _, _, _, _, _) => {
                        h577
                    }
                })
            } else {
                false
            } {
                f18(match v1 {
                    Adt1::C0(_, _, _, _, h578, _, _, _, _, _, _) => {
                        h578
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h579, _, _, _, _, _, _) => {
                        h579
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

pub fn f24(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m580 = v0;
    match m580 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a582 = match v3.clone() {
                Adt1::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                    h581
                }
            };
            let a583 = 0u64;
            if if if if if if nat_eq(a582, a583) {
                let a585 = match v3.clone() {
                    Adt1::C0(_, h584, _, _, _, _, _, _, _, _, _) => {
                        h584
                    }
                };
                let a586 = 1u64;
                if nat_eq(a585, a586) {
                    true
                } else {
                    let a588 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h587, _, _, _, _, _) => {
                            h587
                        }
                    };
                    let a589 = 4u64;
                    nat_eq(a588, a589)
                }
            } else {
                false
            } {
                let a592 = match v3.clone() {
                    Adt1::C0(_, h590, _, _, _, _, _, _, _, _, _) => {
                        h590
                    }
                };
                let a593 = match v1 {
                    Adt1::C0(_, _, _, h591, _, _, _, _, _, _, _) => {
                        h591
                    }
                };
                nat_eq(a592, a593)
            } else {
                false
            } {
                let a595 = match v3.clone() {
                    Adt1::C0(_, _, h594, _, _, _, _, _, _, _, _) => {
                        h594
                    }
                };
                let a596 = 1u64;
                nat_eq(a595, a596)
            } else {
                false
            } {
                let a598 = match v3.clone() {
                    Adt1::C0(_, _, _, h597, _, _, _, _, _, _, _) => {
                        h597
                    }
                };
                let a599 = 1u64;
                nat_eq(a598, a599)
            } else {
                false
            } {
                let a601 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h600, _, _, _, _, _, _) => {
                        h600
                    }
                };
                let a602 = 1u64;
                nat_eq(a601, a602)
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

pub fn f25(v0: u64) -> u64 {
    if f28(v0) {
        8u64
    } else {
        if f29(v0) {
            9u64
        } else {
            f30(v0)
        }
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> bool {
    let m603 = f31(v0);
    match m603 {
        Adt6::C0 => {
            false
        }
        Adt6::C1(v3) => {
            let a604 = v0;
            let a605 = 0u64;
            if if if if if nat_eq(a604, a605) {
                true
            } else {
                let a606 = v0;
                let a607 = 1u64;
                nat_eq(a606, a607)
            } {
                true
            } else {
                let a608 = v0;
                let a609 = 30u64;
                nat_eq(a608, a609)
            } {
                true
            } else {
                f32(v0)
            } {
                let a611 = v2;
                let a612 = match v3.clone() {
                    Adt7::C0(_, _, h610) => {
                        h610
                    }
                };
                nat_eq(a611, a612)
            } else {
                false
            } {
                f18(v1, match v3 {
                    Adt7::C0(_, h613, _) => {
                        h613
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a614 = v1;
    let a615 = 1u64;
    if if if nat_le(a614, a615) {
        true
    } else {
        let a616 = v0;
        let a617 = v2;
        nat_le(a616, a617)
    } {
        let a618 = v3;
        let a619 = 1u64;
        if nat_le(a618, a619) {
            true
        } else {
            let a626 = {
                let a624 = v0;
                let a625 = {
                    let a622 = {
                        let a620 = v1;
                        let a621 = 1u64;
                        nat_sub(a620, a621)
                    };
                    let a623 = v2;
                    nat_mul(a622, a623)?
                };
                nat_add(a624, a625)?
            };
            let a627 = v4;
            nat_le(a626, a627)
        }
    } else {
        false
    } {
        let a628 = v5;
        let a629 = 1u64;
        if nat_le(a628, a629) {
            Ok(true)
        } else {
            let a642 = {
                let a640 = {
                    let a634 = v0;
                    let a635 = {
                        let a632 = {
                            let a630 = v1;
                            let a631 = 1u64;
                            nat_sub(a630, a631)
                        };
                        let a633 = v2;
                        nat_mul(a632, a633)?
                    };
                    nat_add(a634, a635)?
                };
                let a641 = {
                    let a638 = {
                        let a636 = v3;
                        let a637 = 1u64;
                        nat_sub(a636, a637)
                    };
                    let a639 = v4;
                    nat_mul(a638, a639)?
                };
                nat_add(a640, a641)?
            };
            let a643 = v6;
            Ok(nat_le(a642, a643))
        }
    } else {
        Ok(false)
    }
}

pub fn f28(v0: u64) -> bool {
    let a644 = v0;
    let a645 = 2u64;
    if if if nat_eq(a644, a645) {
        true
    } else {
        let a646 = v0;
        let a647 = 6u64;
        nat_eq(a646, a647)
    } {
        true
    } else {
        let a648 = v0;
        let a649 = 8u64;
        nat_eq(a648, a649)
    } {
        true
    } else {
        let a650 = v0;
        let a651 = 20u64;
        nat_eq(a650, a651)
    }
}

pub fn f29(v0: u64) -> bool {
    let a652 = v0;
    let a653 = 3u64;
    if nat_eq(a652, a653) {
        true
    } else {
        let a654 = v0;
        let a655 = 7u64;
        nat_eq(a654, a655)
    }
}

pub fn f30(v0: u64) -> u64 {
    let a656 = v0;
    let a657 = 30u64;
    if nat_eq(a656, a657) {
        30u64
    } else {
        0u64
    }
}

pub fn f31(v0: u64) -> Adt6 {
    let a658 = v0;
    let a659 = 0u64;
    if nat_eq(a658, a659) {
        Adt6::C1(Adt7::C0(0u64, 1u64, 4u64))
    } else {
        let a660 = v0;
        let a661 = 1u64;
        if nat_eq(a660, a661) {
            Adt6::C1(Adt7::C0(1u64, 1u64, 2u64))
        } else {
            let a662 = v0;
            let a663 = 2u64;
            if nat_eq(a662, a663) {
                Adt6::C1(Adt7::C0(2u64, 32u64, 18u64))
            } else {
                let a664 = v0;
                let a665 = 3u64;
                if nat_eq(a664, a665) {
                    Adt6::C1(Adt7::C0(3u64, 32u64, 20u64))
                } else {
                    let a666 = v0;
                    let a667 = 6u64;
                    if nat_eq(a666, a667) {
                        Adt6::C1(Adt7::C0(6u64, 32u64, 22u64))
                    } else {
                        let a668 = v0;
                        let a669 = 7u64;
                        if nat_eq(a668, a669) {
                            Adt6::C1(Adt7::C0(7u64, 32u64, 24u64))
                        } else {
                            let a670 = v0;
                            let a671 = 8u64;
                            if nat_eq(a670, a671) {
                                Adt6::C1(Adt7::C0(8u64, 32u64, 34u64))
                            } else {
                                let a672 = v0;
                                let a673 = 9u64;
                                if nat_eq(a672, a673) {
                                    Adt6::C1(Adt7::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a674 = v0;
                                    let a675 = 10u64;
                                    if nat_eq(a674, a675) {
                                        Adt6::C1(Adt7::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a676 = v0;
                                        let a677 = 11u64;
                                        if nat_eq(a676, a677) {
                                            Adt6::C1(Adt7::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a678 = v0;
                                            let a679 = 12u64;
                                            if nat_eq(a678, a679) {
                                                Adt6::C1(Adt7::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a680 = v0;
                                                let a681 = 13u64;
                                                if nat_eq(a680, a681) {
                                                    Adt6::C1(Adt7::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a682 = v0;
                                                    let a683 = 14u64;
                                                    if nat_eq(a682, a683) {
                                                        Adt6::C1(Adt7::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a684 = v0;
                                                        let a685 = 15u64;
                                                        if nat_eq(a684, a685) {
                                                            Adt6::C1(Adt7::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a686 = v0;
                                                            let a687 = 16u64;
                                                            if nat_eq(a686, a687) {
                                                                Adt6::C1(Adt7::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a688 = v0;
                                                                let a689 = 17u64;
                                                                if nat_eq(a688, a689) {
                                                                    Adt6::C1(Adt7::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a690 = v0;
                                                                    let a691 = 18u64;
                                                                    if nat_eq(a690, a691) {
                                                                        Adt6::C1(Adt7::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a692 = v0;
                                                                        let a693 = 19u64;
                                                                        if nat_eq(a692, a693) {
                                                                            Adt6::C1(Adt7::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a694 = v0;
                                                                            let a695 = 20u64;
                                                                            if nat_eq(a694, a695) {
                                                                                Adt6::C1(Adt7::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a696 = v0;
                                                                                let a697 = 21u64;
                                                                                if nat_eq(a696, a697) {
                                                                                    Adt6::C1(Adt7::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a698 = v0;
                                                                                    let a699 = 22u64;
                                                                                    if nat_eq(a698, a699) {
                                                                                        Adt6::C1(Adt7::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a700 = v0;
                                                                                        let a701 = 23u64;
                                                                                        if nat_eq(a700, a701) {
                                                                                            Adt6::C1(Adt7::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a702 = v0;
                                                                                            let a703 = 24u64;
                                                                                            if nat_eq(a702, a703) {
                                                                                                Adt6::C1(Adt7::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a704 = v0;
                                                                                                let a705 = 25u64;
                                                                                                if nat_eq(a704, a705) {
                                                                                                    Adt6::C1(Adt7::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a706 = v0;
                                                                                                    let a707 = 26u64;
                                                                                                    if nat_eq(a706, a707) {
                                                                                                        Adt6::C1(Adt7::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a708 = v0;
                                                                                                        let a709 = 27u64;
                                                                                                        if nat_eq(a708, a709) {
                                                                                                            Adt6::C1(Adt7::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a710 = v0;
                                                                                                            let a711 = 28u64;
                                                                                                            if nat_eq(a710, a711) {
                                                                                                                Adt6::C1(Adt7::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a712 = v0;
                                                                                                                let a713 = 29u64;
                                                                                                                if nat_eq(a712, a713) {
                                                                                                                    Adt6::C1(Adt7::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a714 = v0;
                                                                                                                    let a715 = 30u64;
                                                                                                                    if nat_eq(a714, a715) {
                                                                                                                        Adt6::C1(Adt7::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a716 = v0;
                                                                                                                        let a717 = 34u64;
                                                                                                                        if nat_eq(a716, a717) {
                                                                                                                            Adt6::C1(Adt7::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a718 = v0;
                                                                                                                            let a719 = 35u64;
                                                                                                                            if nat_eq(a718, a719) {
                                                                                                                                Adt6::C1(Adt7::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a720 = v0;
                                                                                                                                let a721 = 39u64;
                                                                                                                                if nat_eq(a720, a721) {
                                                                                                                                    Adt6::C1(Adt7::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a722 = v0;
                                                                                                                                    let a723 = 40u64;
                                                                                                                                    if nat_eq(a722, a723) {
                                                                                                                                        Adt6::C1(Adt7::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a724 = v0;
                                                                                                                                        let a725 = 41u64;
                                                                                                                                        if nat_eq(a724, a725) {
                                                                                                                                            Adt6::C1(Adt7::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a726 = v0;
                                                                                                                                            let a727 = 42u64;
                                                                                                                                            if nat_eq(a726, a727) {
                                                                                                                                                Adt6::C1(Adt7::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a728 = v0;
                                                                                                                                                let a729 = 43u64;
                                                                                                                                                if nat_eq(a728, a729) {
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

pub fn f32(v0: u64) -> bool {
    if f28(v0) {
        true
    } else {
        f29(v0)
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
