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
    C0(u64),
    C1(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64),
}

pub fn f0(v0: Adt0, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v19) => {
            f1(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt0::C1(v20, v21, v22, v23) => {
            let a2 = v20;
            let a3 = 66u64;
            if nat_eq(a2, a3) {
                f2(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f3(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt1> {
    f4(v0, Adt2::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt2::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt1> {
    f5(v0, Adt2::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt2::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt2::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt1> {
    let a4 = v0;
    let a5 = 64u64;
    if nat_eq(a4, a5) {
        f6(v1, Adt2::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt2::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6 = v0;
        let a7 = 65u64;
        if nat_eq(a6, a7) {
            f7(v1, Adt2::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt2::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt1::C0)
        }
    }
}

pub fn f4(v0: u64, v1: Adt2, v2: Adt2, v3: u64, v4: u64) -> R<Adt1> {
    let v5: u64 = f8(match v1.clone() {
        Adt2::C0(h8, _, _, _, _, _, _, _) => {
            h8
        }
    })?;
    let v6: u64 = {
        let a14 = match v1.clone() {
            Adt2::C0(_, h9, _, _, _, _, _, _) => {
                h9
            }
        };
        let a15 = {
            let a12 = match v1.clone() {
                Adt2::C0(_, _, h10, _, _, _, _, _) => {
                    h10
                }
            };
            let a13 = match v1.clone() {
                Adt2::C0(_, _, _, h11, _, _, _, _) => {
                    h11
                }
            };
            nat_mul(a12, a13)?
        };
        nat_mul(a14, a15)?
    };
    let a17 = v0;
    let a18 = f9(match v1.clone() {
        Adt2::C0(h16, _, _, _, _, _, _, _) => {
            h16
        }
    }, v6)?;
    if nat_lt(a17, a18) {
        f10({
            let a19 = v0;
            let a20 = v5;
            let a21 = 0u64;
            nat_rem(a19, a20, a21)
        }, {
            let a22 = v0;
            let a23 = v5;
            let a24 = 0u64;
            nat_quot(a22, a23, a24)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt1> {
    let v8: u64 = {
        let a25 = v6;
        let a26 = 6u64;
        nat_sub(a25, a26)
    };
    let v9: Adt2 = f11(v3);
    let a28 = v0;
    let a29 = f12(match v1.clone() {
        Adt2::C0(h27, _, _, _, _, _, _, _) => {
            h27
        }
    }, v5, v8)?;
    if nat_lt(a28, a29) {
        let v10: u64 = f13(match v1.clone() {
            Adt2::C0(h30, _, _, _, _, _, _, _) => {
                h30
            }
        }, v5, v8)?;
        let a31 = v0;
        let a32 = v10;
        if nat_lt(a31, a32) {
            f14(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a33 = v0;
                let a34 = v10;
                nat_sub(a33, a34)
            };
            let v12: u64 = f15(match v1.clone() {
                Adt2::C0(h35, _, _, _, _, _, _, _) => {
                    h35
                }
            })?;
            f16({
                let a36 = v11;
                let a37 = v12;
                let a38 = 0u64;
                nat_rem(a36, a37, a38)
            }, {
                let a39 = v11;
                let a40 = v12;
                let a41 = 0u64;
                nat_quot(a39, a40, a41)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f6(v0: u64, v1: Adt2, v2: Adt2, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt1> {
    let v7: u64 = {
        let a42 = v5;
        let a43 = 6u64;
        nat_sub(a42, a43)
    };
    let v8: bool = f17(match v1.clone() {
        Adt2::C0(_, _, _, _, h44, _, _, _) => {
            h44
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h45, _, _, _) => {
            h45
        }
    });
    let a47 = v0;
    let a48 = f18(v8, match v1.clone() {
        Adt2::C0(h46, _, _, _, _, _, _, _) => {
            h46
        }
    }, v4, v7)?;
    if nat_lt(a47, a48) {
        let v9: u64 = f13(match v1.clone() {
            Adt2::C0(h49, _, _, _, _, _, _, _) => {
                h49
            }
        }, v4, v7)?;
        let a50 = v0;
        let a51 = v9;
        if nat_lt(a50, a51) {
            f14(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a52 = v0;
                let a53 = v9;
                nat_sub(a52, a53)
            };
            let v11: u64 = f19(v8, match v1.clone() {
                Adt2::C0(h54, _, _, _, _, _, _, _) => {
                    h54
                }
            })?;
            f20({
                let a55 = v10;
                let a56 = v11;
                let a57 = 0u64;
                nat_rem(a55, a56, a57)
            }, {
                let a58 = v10;
                let a59 = v11;
                let a60 = 0u64;
                nat_quot(a58, a59, a60)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f7(v0: u64, v1: Adt2, v2: Adt2, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt1> {
    let v7: u64 = {
        let a61 = v5;
        let a62 = 6u64;
        nat_sub(a61, a62)
    };
    let v8: Adt2 = f11(v2);
    let v9: u64 = f21(match v1.clone() {
        Adt2::C0(h63, _, _, _, _, _, _, _) => {
            h63
        }
    }, match v1.clone() {
        Adt2::C0(_, h64, _, _, _, _, _, _) => {
            h64
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h65, _, _, _, _, _) => {
            h65
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h66, _, _, _, _) => {
            h66
        }
    }, v7)?;
    let a71 = v0;
    let a72 = f22(match v1.clone() {
        Adt2::C0(h67, _, _, _, _, _, _, _) => {
            h67
        }
    }, match v1.clone() {
        Adt2::C0(_, h68, _, _, _, _, _, _) => {
            h68
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h69, _, _, _, _, _) => {
            h69
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h70, _, _, _, _) => {
            h70
        }
    }, v4, v7)?;
    if nat_lt(a71, a72) {
        let a73 = v0;
        let a74 = v9;
        if nat_lt(a73, a74) {
            f14(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt3 = f23({
                let a75 = v0;
                let a76 = v9;
                nat_sub(a75, a76)
            }, match v1.clone() {
                Adt2::C0(h77, _, _, _, _, _, _, _) => {
                    h77
                }
            }, match v1.clone() {
                Adt2::C0(_, h78, _, _, _, _, _, _) => {
                    h78
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h79, _, _, _, _, _) => {
                    h79
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h80, _, _, _, _) => {
                    h80
                }
            }, v4)?;
            f24(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f8(v0: u64) -> R<u64> {
    let a83 = 5u64;
    let a84 = {
        let a81 = 4u64;
        let a82 = v0;
        nat_mul(a81, a82)?
    };
    nat_add(a83, a84)
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    let a85 = v0;
    let a86 = 0u64;
    if nat_eq(a85, a86) {
        Ok(0u64)
    } else {
        let a87 = v1;
        let a88 = f8(v0)?;
        nat_mul(a87, a88)
    }
}

pub fn f10(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = match v2.clone() {
        Adt2::C0(h89, _, _, _, _, _, _, _) => {
            h89
        }
    };
    let v7: u64 = {
        let a92 = 1u64;
        let a93 = {
            let a90 = 3u64;
            let a91 = v6;
            nat_mul(a90, a91)?
        };
        nat_add(a92, a93)?
    };
    let a94 = v0;
    let a95 = 0u64;
    if nat_eq(a94, a95) {
        Ok(f25(0u64))
    } else {
        let a96 = v0;
        let a97 = v7;
        if nat_lt(a96, a97) {
            let v8: u64 = {
                let a98 = v0;
                let a99 = 1u64;
                nat_sub(a98, a99)
            };
            let v9: u64 = {
                let a105 = {
                    let a100 = v1;
                    let a101 = v6;
                    nat_mul(a100, a101)?
                };
                let a106 = {
                    let a102 = v8;
                    let a103 = 3u64;
                    let a104 = 0u64;
                    nat_quot(a102, a103, a104)
                };
                nat_add(a105, a106)?
            };
            let v10: u64 = f26(v9, v2)?;
            let v11: u64 = {
                let a107 = v8;
                let a108 = 3u64;
                let a109 = 0u64;
                nat_rem(a107, a108, a109)
            };
            let a110 = v11;
            let a111 = 0u64;
            if nat_eq(a110, a111) {
                Ok(f27(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a112 = v11;
                let a113 = 1u64;
                if nat_eq(a112, a113) {
                    Ok(f27(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f27(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a114 = v0;
            let a115 = v7;
            if nat_eq(a114, a115) {
                Ok(f27(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a118 = v0;
                let a119 = {
                    let a116 = v7;
                    let a117 = 1u64;
                    nat_add(a116, a117)?
                };
                if nat_eq(a118, a119) {
                    Ok(f27(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a122 = v0;
                    let a123 = {
                        let a120 = v7;
                        let a121 = 2u64;
                        nat_add(a120, a121)?
                    };
                    if nat_eq(a122, a123) {
                        Ok(f27(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a126 = v0;
                        let a127 = {
                            let a124 = v7;
                            let a125 = 3u64;
                            nat_add(a124, a125)?
                        };
                        if nat_eq(a126, a127) {
                            let a128 = v5;
                            let a129 = 1u64;
                            if if nat_eq(a128, a129) {
                                true
                            } else {
                                let a130 = v5;
                                let a131 = 2u64;
                                nat_eq(a130, a131)
                            } {
                                Ok(f27(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f27(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a138 = {
                                    let a132 = v1;
                                    let a133 = v6;
                                    nat_mul(a132, a133)?
                                };
                                let a139 = {
                                    let a136 = v0;
                                    let a137 = {
                                        let a134 = v7;
                                        let a135 = 4u64;
                                        nat_add(a134, a135)?
                                    };
                                    nat_sub(a136, a137)
                                };
                                nat_add(a138, a139)?
                            };
                            Ok(f27(10u64, 0u64, f26(v12, v2)?, 1u64, 2u64, 3u64, f26(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f11(v0: Adt2) -> Adt2 {
    Adt2::C0(match v0.clone() {
        Adt2::C0(h140, _, _, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt2::C0(_, h141, _, _, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h142, _, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h143, _, _, _, _) => {
            h143
        }
    }, 4u64, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h144, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h145, _) => {
            h145
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h146) => {
            h146
        }
    })
}

pub fn f12(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a147 = v0;
    let a148 = 0u64;
    if if nat_eq(a147, a148) {
        true
    } else {
        let a149 = v1;
        let a150 = 0u64;
        nat_eq(a149, a150)
    } {
        Ok(0u64)
    } else {
        let a153 = f13(v0, v1, v2)?;
        let a154 = {
            let a151 = v1;
            let a152 = f15(v0)?;
            nat_mul(a151, a152)?
        };
        nat_add(a153, a154)
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a155 = v2;
    let a156 = 0u64;
    if nat_eq(a155, a156) {
        Ok(0u64)
    } else {
        let a159 = 2u64;
        let a160 = {
            let a157 = v0;
            let a158 = v1;
            nat_mul(a157, a158)?
        };
        nat_mul(a159, a160)
    }
}

pub fn f14(v0: u64, v1: Adt2, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let v5: u64 = {
        let a167 = v3;
        let a168 = {
            let a164 = f26({
                let a161 = v0;
                let a162 = 2u64;
                let a163 = 0u64;
                nat_quot(a161, a162, a163)
            }, v1)?;
            let a165 = v2;
            let a166 = 0u64;
            nat_quot(a164, a165, a166)
        };
        nat_add(a167, a168)?
    };
    let a172 = {
        let a169 = v0;
        let a170 = 2u64;
        let a171 = 0u64;
        nat_rem(a169, a170, a171)
    };
    let a173 = 0u64;
    if nat_eq(a172, a173) {
        Ok(f27(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a174 = v4;
        let a175 = 0u64;
        if nat_eq(a174, a175) {
            Ok(f27(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt1::C0)
        }
    }
}

pub fn f15(v0: u64) -> R<u64> {
    let a178 = 13u64;
    let a179 = {
        let a176 = 9u64;
        let a177 = v0;
        nat_mul(a176, a177)?
    };
    nat_add(a178, a179)
}

pub fn f16(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: Adt2, v5: u64) -> R<Adt1> {
    let v6: u64 = f28(match v2.clone() {
        Adt2::C0(h180, _, _, _, _, _, _, _) => {
            h180
        }
    })?;
    let v7: u64 = f29(match v2.clone() {
        Adt2::C0(h181, _, _, _, _, _, _, _) => {
            h181
        }
    })?;
    let a182 = v0;
    let a183 = 2u64;
    if nat_lt(a182, a183) {
        Ok(f25(v0))
    } else {
        let a184 = v0;
        let a185 = v6;
        if nat_lt(a184, a185) {
            f30({
                let a186 = v0;
                let a187 = 2u64;
                nat_sub(a186, a187)
            }, v1, v2, v3)
        } else {
            let a188 = v0;
            let a189 = v7;
            if nat_lt(a188, a189) {
                Ok(f31({
                    let a190 = v0;
                    let a191 = v6;
                    nat_sub(a190, a191)
                }, match v2 {
                    Adt2::C0(h192, _, _, _, _, _, _, _) => {
                        h192
                    }
                }, v5))
            } else {
                let a194 = v0;
                let a195 = f15(match v2.clone() {
                    Adt2::C0(h193, _, _, _, _, _, _, _) => {
                        h193
                    }
                })?;
                if nat_lt(a194, a195) {
                    f32({
                        let a196 = v0;
                        let a197 = v7;
                        nat_sub(a196, a197)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64) -> bool {
    let a198 = v0;
    let a199 = 4u64;
    if nat_eq(a198, a199) {
        let a200 = v1;
        let a201 = 4u64;
        nat_eq(a200, a201)
    } else {
        false
    }
}

pub fn f18(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a202 = v1;
    let a203 = 0u64;
    if if nat_eq(a202, a203) {
        true
    } else {
        let a204 = v2;
        let a205 = 0u64;
        nat_eq(a204, a205)
    } {
        Ok(0u64)
    } else {
        let a208 = f13(v1, v2, v3)?;
        let a209 = {
            let a206 = v2;
            let a207 = f19(v0, v1)?;
            nat_mul(a206, a207)?
        };
        nat_add(a208, a209)
    }
}

pub fn f19(v0: bool, v1: u64) -> R<u64> {
    let a214 = {
        let a212 = {
            let a210 = f33(v0, v1)?;
            let a211 = f34(v0, v1)?;
            nat_add(a210, a211)?
        };
        let a213 = f35(v0);
        nat_add(a212, a213)?
    };
    let a215 = v1;
    nat_add(a214, a215)
}

pub fn f20(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: u64, v5: bool) -> R<Adt1> {
    let v6: u64 = f33(v5, match v2.clone() {
        Adt2::C0(h216, _, _, _, _, _, _, _) => {
            h216
        }
    })?;
    let v7: u64 = {
        let a218 = v6;
        let a219 = f34(v5, match v2.clone() {
            Adt2::C0(h217, _, _, _, _, _, _, _) => {
                h217
            }
        })?;
        nat_add(a218, a219)?
    };
    let v8: u64 = {
        let a220 = v7;
        let a221 = f35(v5);
        nat_add(a220, a221)?
    };
    let a222 = v0;
    let a223 = v6;
    if nat_lt(a222, a223) {
        f36(v0, v1, v2, v5)
    } else {
        let a224 = v0;
        let a225 = v7;
        if nat_lt(a224, a225) {
            f37({
                let a226 = v0;
                let a227 = v6;
                nat_sub(a226, a227)
            }, v1, v2, v3, v5)
        } else {
            let a228 = v0;
            let a229 = v8;
            if nat_lt(a228, a229) {
                Ok(f38({
                    let a230 = v0;
                    let a231 = v7;
                    nat_sub(a230, a231)
                }, match v2 {
                    Adt2::C0(h232, _, _, _, _, _, _, _) => {
                        h232
                    }
                }, v4, v5))
            } else {
                let a236 = v0;
                let a237 = {
                    let a234 = v8;
                    let a235 = match v2.clone() {
                        Adt2::C0(h233, _, _, _, _, _, _, _) => {
                            h233
                        }
                    };
                    nat_add(a234, a235)?
                };
                if nat_lt(a236, a237) {
                    let v9: u64 = f26({
                        let a243 = {
                            let a239 = v1;
                            let a240 = match v2 {
                                Adt2::C0(h238, _, _, _, _, _, _, _) => {
                                    h238
                                }
                            };
                            nat_mul(a239, a240)?
                        };
                        let a244 = {
                            let a241 = v0;
                            let a242 = v8;
                            nat_sub(a241, a242)
                        };
                        nat_add(a243, a244)?
                    }, v3)?;
                    Ok(f27(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a245 = v4;
    let a246 = 0u64;
    if nat_eq(a245, a246) {
        Ok(0u64)
    } else {
        let a253 = 2u64;
        let a254 = {
            let a251 = {
                let a247 = v0;
                let a248 = v1;
                nat_mul(a247, a248)?
            };
            let a252 = {
                let a249 = v2;
                let a250 = v3;
                nat_mul(a249, a250)?
            };
            nat_mul(a251, a252)?
        };
        nat_mul(a253, a254)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a255 = v0;
    let a256 = 0u64;
    if if if if nat_eq(a255, a256) {
        true
    } else {
        let a257 = v1;
        let a258 = 0u64;
        nat_eq(a257, a258)
    } {
        true
    } else {
        let a259 = v2;
        let a260 = 0u64;
        nat_eq(a259, a260)
    } {
        true
    } else {
        let a261 = v3;
        let a262 = 0u64;
        nat_eq(a261, a262)
    } {
        Ok(0u64)
    } else {
        let a263 = f21(v0, v1, v2, v3, v5)?;
        let a264 = f39(v0, v1, v2, v3, v4)?;
        nat_add(a263, a264)
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt3> {
    let v6: u64 = f40(v3, v5)?;
    let v7: u64 = f41(v1, v2, v6)?;
    let v8: u64 = {
        let a267 = {
            let a265 = f42(v3, v5)?;
            let a266 = v4;
            nat_mul(a265, a266)?
        };
        let a268 = v7;
        nat_mul(a267, a268)?
    };
    let a269 = v0;
    let a270 = v8;
    if nat_lt(a269, a270) {
        let v9: u64 = {
            let a271 = v0;
            let a272 = v7;
            let a273 = 0u64;
            nat_quot(a271, a272, a273)
        };
        Ok(Adt3::C0({
            let a277 = {
                let a274 = v9;
                let a275 = v4;
                let a276 = 0u64;
                nat_quot(a274, a275, a276)
            };
            let a278 = v6;
            nat_mul(a277, a278)?
        }, v6, {
            let a279 = v9;
            let a280 = v4;
            let a281 = 0u64;
            nat_rem(a279, a280, a281)
        }, {
            let a282 = v0;
            let a283 = v7;
            let a284 = 0u64;
            nat_rem(a282, a283, a284)
        }))
    } else {
        let v10: u64 = {
            let a285 = v0;
            let a286 = v8;
            nat_sub(a285, a286)
        };
        let v11: u64 = f41(v1, v2, f43(v3, v5)?)?;
        Ok(Adt3::C0({
            let a287 = f42(v3, v5)?;
            let a288 = v6;
            nat_mul(a287, a288)?
        }, f43(v3, v5)?, {
            let a289 = v10;
            let a290 = v11;
            let a291 = 0u64;
            nat_quot(a289, a290, a291)
        }, {
            let a292 = v10;
            let a293 = v11;
            let a294 = 0u64;
            nat_rem(a292, a293, a294)
        }))
    }
}

pub fn f24(v0: Adt3, v1: Adt2, v2: Adt2, v3: u64) -> R<Adt1> {
    let v4: u64 = match v0.clone() {
        Adt3::C0(_, _, _, h295) => {
            h295
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(h296, _, _, _, _, _, _, _) => {
            h296
        }
    };
    let v6: u64 = {
        let a299 = match v0.clone() {
            Adt3::C0(_, h297, _, _) => {
                h297
            }
        };
        let a300 = match v1.clone() {
            Adt2::C0(_, h298, _, _, _, _, _, _) => {
                h298
            }
        };
        nat_mul(a299, a300)?
    };
    let v7: u64 = {
        let a301 = v6;
        let a302 = v5;
        nat_mul(a301, a302)?
    };
    let v8: u64 = {
        let a305 = {
            let a303 = 2u64;
            let a304 = v5;
            nat_mul(a303, a304)?
        };
        let a306 = 2u64;
        nat_add(a305, a306)?
    };
    let v9: u64 = {
        let a309 = {
            let a307 = 4u64;
            let a308 = v5;
            nat_mul(a307, a308)?
        };
        let a310 = 2u64;
        nat_add(a309, a310)?
    };
    let v10: u64 = {
        let a313 = 1u64;
        let a314 = {
            let a311 = v6;
            let a312 = v8;
            nat_mul(a311, a312)?
        };
        nat_add(a313, a314)?
    };
    let v11: u64 = {
        let a315 = v10;
        let a316 = 4u64;
        nat_add(a315, a316)?
    };
    let v12: u64 = {
        let a319 = v11;
        let a320 = {
            let a317 = v6;
            let a318 = v9;
            nat_mul(a317, a318)?
        };
        nat_add(a319, a320)?
    };
    let v13: u64 = {
        let a321 = v12;
        let a322 = 6u64;
        nat_add(a321, a322)?
    };
    let a323 = v4;
    let a324 = 0u64;
    if nat_eq(a323, a324) {
        Ok(f25(0u64))
    } else {
        let a325 = v4;
        let a326 = v10;
        if nat_lt(a325, a326) {
            let v14: u64 = {
                let a327 = v4;
                let a328 = 1u64;
                nat_sub(a327, a328)
            };
            f44(false, {
                let a329 = v14;
                let a330 = v8;
                let a331 = 0u64;
                nat_rem(a329, a330, a331)
            }, {
                let a332 = v14;
                let a333 = v8;
                let a334 = 0u64;
                nat_quot(a332, a333, a334)
            }, v0, v1, v2)
        } else {
            let a335 = v4;
            let a336 = v11;
            if nat_lt(a335, a336) {
                Ok(f45({
                    let a337 = v4;
                    let a338 = v10;
                    nat_sub(a337, a338)
                }, v7))
            } else {
                let a339 = v4;
                let a340 = v12;
                if nat_lt(a339, a340) {
                    let v15: u64 = {
                        let a341 = v4;
                        let a342 = v11;
                        nat_sub(a341, a342)
                    };
                    f44(true, {
                        let a343 = v15;
                        let a344 = v9;
                        let a345 = 0u64;
                        nat_rem(a343, a344, a345)
                    }, {
                        let a346 = v15;
                        let a347 = v9;
                        let a348 = 0u64;
                        nat_quot(a346, a347, a348)
                    }, v0, v1, v2)
                } else {
                    let a349 = v4;
                    let a350 = v13;
                    if nat_lt(a349, a350) {
                        Ok(f46({
                            let a351 = v4;
                            let a352 = v12;
                            nat_sub(a351, a352)
                        }, v7, v3))
                    } else {
                        let a355 = v4;
                        let a356 = {
                            let a353 = v13;
                            let a354 = v7;
                            nat_add(a353, a354)?
                        };
                        if nat_lt(a355, a356) {
                            let v16: u64 = {
                                let a357 = v4;
                                let a358 = v13;
                                nat_sub(a357, a358)
                            };
                            let v17: u64 = {
                                let a359 = v16;
                                let a360 = v5;
                                let a361 = 0u64;
                                nat_quot(a359, a360, a361)
                            };
                            let v18: u64 = {
                                let a367 = {
                                    let a362 = f47(v17, v0, v1)?;
                                    let a363 = v5;
                                    nat_mul(a362, a363)?
                                };
                                let a368 = {
                                    let a364 = v16;
                                    let a365 = v5;
                                    let a366 = 0u64;
                                    nat_rem(a364, a365, a366)
                                };
                                nat_add(a367, a368)?
                            };
                            let v19: u64 = f26(v18, v2)?;
                            Ok(f27(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt1::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> Adt1 {
    f27(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f26(v0: u64, v1: Adt2) -> R<u64> {
    let v2: u64 = {
        let a370 = v0;
        let a371 = match v1.clone() {
            Adt2::C0(h369, _, _, _, _, _, _, _) => {
                h369
            }
        };
        let a372 = 0u64;
        nat_quot(a370, a371, a372)
    };
    let v3: u64 = {
        let a374 = v2;
        let a375 = match v1.clone() {
            Adt2::C0(_, h373, _, _, _, _, _, _) => {
                h373
            }
        };
        let a376 = 0u64;
        nat_quot(a374, a375, a376)
    };
    f48({
        let a378 = v0;
        let a379 = match v1.clone() {
            Adt2::C0(h377, _, _, _, _, _, _, _) => {
                h377
            }
        };
        let a380 = 0u64;
        nat_rem(a378, a379, a380)
    }, {
        let a382 = v2;
        let a383 = match v1.clone() {
            Adt2::C0(_, h381, _, _, _, _, _, _) => {
                h381
            }
        };
        let a384 = 0u64;
        nat_rem(a382, a383, a384)
    }, {
        let a386 = v3;
        let a387 = match v1.clone() {
            Adt2::C0(_, _, h385, _, _, _, _, _) => {
                h385
            }
        };
        let a388 = 0u64;
        nat_rem(a386, a387, a388)
    }, {
        let a390 = v3;
        let a391 = match v1.clone() {
            Adt2::C0(_, _, h389, _, _, _, _, _) => {
                h389
            }
        };
        let a392 = 0u64;
        nat_quot(a390, a391, a392)
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h393, _, _, _) => {
            h393
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, h394, _, _) => {
            h394
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h395, _) => {
            h395
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, _, h396) => {
            h396
        }
    })
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt1 {
    let a397 = v0;
    let a398 = 4u64;
    if if if nat_le(a397, a398) {
        true
    } else {
        let a399 = 9u64;
        let a400 = v0;
        if nat_le(a399, a400) {
            let a401 = v0;
            let a402 = 12u64;
            nat_le(a401, a402)
        } else {
            false
        }
    } {
        true
    } else {
        let a403 = v0;
        let a404 = 16u64;
        if if nat_eq(a403, a404) {
            true
        } else {
            let a405 = v0;
            let a406 = 19u64;
            nat_eq(a405, a406)
        } {
            true
        } else {
            let a407 = v0;
            let a408 = 21u64;
            if nat_eq(a407, a408) {
                true
            } else {
                let a409 = v0;
                let a410 = 22u64;
                nat_eq(a409, a410)
            }
        }
    } {
        Adt1::C1({
            let a411 = v0;
            let a412 = 4u64;
            if if nat_lt(a411, a412) {
                true
            } else {
                let a413 = 21u64;
                let a414 = v0;
                nat_le(a413, a414)
            } {
                0u64
            } else {
                let a415 = v0;
                let a416 = 16u64;
                if if nat_eq(a415, a416) {
                    true
                } else {
                    let a417 = v0;
                    let a418 = 19u64;
                    nat_eq(a417, a418)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a419 = v0;
            let a420 = 4u64;
            if nat_lt(a419, a420) {
                v0
            } else {
                let a421 = v0;
                let a422 = 4u64;
                if if nat_eq(a421, a422) {
                    true
                } else {
                    let a423 = v0;
                    let a424 = 16u64;
                    nat_eq(a423, a424)
                } {
                    0u64
                } else {
                    let a425 = v0;
                    let a426 = 19u64;
                    if nat_eq(a425, a426) {
                        3u64
                    } else {
                        let a427 = 21u64;
                        let a428 = v0;
                        if nat_le(a427, a428) {
                            let a429 = v0;
                            let a430 = 17u64;
                            nat_sub(a429, a430)
                        } else {
                            let a431 = v0;
                            let a432 = 8u64;
                            nat_sub(a431, a432)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt1::C0
    }
}

pub fn f28(v0: u64) -> R<u64> {
    let a435 = 2u64;
    let a436 = {
        let a433 = 6u64;
        let a434 = v0;
        nat_mul(a433, a434)?
    };
    nat_add(a435, a436)
}

pub fn f29(v0: u64) -> R<u64> {
    let a437 = f28(v0)?;
    let a438 = 11u64;
    nat_add(a437, a438)
}

pub fn f30(v0: u64, v1: u64, v2: Adt2, v3: Adt2) -> R<Adt1> {
    let v4: u64 = {
        let a439 = v0;
        let a440 = 6u64;
        let a441 = 0u64;
        nat_rem(a439, a440, a441)
    };
    let v5: u64 = {
        let a448 = {
            let a443 = v1;
            let a444 = match v2.clone() {
                Adt2::C0(h442, _, _, _, _, _, _, _) => {
                    h442
                }
            };
            nat_mul(a443, a444)?
        };
        let a449 = {
            let a445 = v0;
            let a446 = 6u64;
            let a447 = 0u64;
            nat_quot(a445, a446, a447)
        };
        nat_add(a448, a449)?
    };
    let v6: u64 = f26(v5, v3)?;
    let v7: u64 = f26(v5, v2)?;
    let a450 = v4;
    let a451 = 0u64;
    if nat_eq(a450, a451) {
        Ok(f27(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a452 = v4;
        let a453 = 1u64;
        if nat_eq(a452, a453) {
            Ok(f27(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a454 = v4;
            let a455 = 2u64;
            if nat_eq(a454, a455) {
                Ok(f27(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a456 = v4;
                let a457 = 3u64;
                if nat_eq(a456, a457) {
                    Ok(f27(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a458 = v4;
                    let a459 = 4u64;
                    if nat_eq(a458, a459) {
                        Ok(f27(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f27(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64) -> Adt1 {
    let a460 = v0;
    let a461 = 0u64;
    if nat_eq(a460, a461) {
        f27(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a462 = v0;
        let a463 = 1u64;
        if nat_eq(a462, a463) {
            f27(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a464 = v0;
            let a465 = 2u64;
            if nat_eq(a464, a465) {
                f27(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a466 = v0;
                let a467 = 3u64;
                if nat_eq(a466, a467) {
                    f27(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a468 = v0;
                    let a469 = 4u64;
                    if nat_eq(a468, a469) {
                        f27(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a470 = v0;
                        let a471 = 5u64;
                        if nat_eq(a470, a471) {
                            f27(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a472 = v0;
                            let a473 = 6u64;
                            if nat_eq(a472, a473) {
                                f27(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a474 = v0;
                                let a475 = 7u64;
                                if nat_eq(a474, a475) {
                                    f27(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a476 = v0;
                                    let a477 = 8u64;
                                    if nat_eq(a476, a477) {
                                        f27(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a478 = v0;
                                        let a479 = 9u64;
                                        if nat_eq(a478, a479) {
                                            Adt1::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a480 = v0;
                                            let a481 = 10u64;
                                            if nat_eq(a480, a481) {
                                                f27(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
                                            } else {
                                                Adt1::C0
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f32(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: Adt2) -> R<Adt1> {
    let v5: u64 = {
        let a482 = v0;
        let a483 = 3u64;
        let a484 = 0u64;
        nat_rem(a482, a483, a484)
    };
    let v6: u64 = {
        let a491 = {
            let a486 = v1;
            let a487 = match v2.clone() {
                Adt2::C0(h485, _, _, _, _, _, _, _) => {
                    h485
                }
            };
            nat_mul(a486, a487)?
        };
        let a492 = {
            let a488 = v0;
            let a489 = 3u64;
            let a490 = 0u64;
            nat_quot(a488, a489, a490)
        };
        nat_add(a491, a492)?
    };
    let v7: u64 = f26(v6, v3)?;
    let v8: u64 = f26(v6, v2)?;
    let v9: u64 = f26(v6, v4)?;
    let a493 = v5;
    let a494 = 0u64;
    if nat_eq(a493, a494) {
        Ok(f27(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a495 = v5;
        let a496 = 1u64;
        if nat_eq(a495, a496) {
            Ok(f27(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f27(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f33(v0: bool, v1: u64) -> R<u64> {
    let a499 = if v0 {
        5u64
    } else {
        4u64
    };
    let a500 = {
        let a497 = f49(v0);
        let a498 = v1;
        nat_mul(a497, a498)?
    };
    nat_add(a499, a500)
}

pub fn f34(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a508 = {
            let a501 = 13u64;
            let a502 = f50(v1);
            nat_mul(a501, a502)?
        };
        let a509 = {
            let a506 = 4u64;
            let a507 = {
                let a503 = v1;
                let a504 = 4u64;
                let a505 = 0u64;
                nat_rem(a503, a504, a505)
            };
            nat_mul(a506, a507)?
        };
        nat_add(a508, a509)
    } else {
        let a510 = 3u64;
        let a511 = v1;
        nat_mul(a510, a511)
    }
}

pub fn f35(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f36(v0: u64, v1: u64, v2: Adt2, v3: bool) -> R<Adt1> {
    let v4: u64 = match v2.clone() {
        Adt2::C0(h512, _, _, _, _, _, _, _) => {
            h512
        }
    };
    let v5: u64 = {
        let a515 = 1u64;
        let a516 = {
            let a513 = v4;
            let a514 = f49(v3);
            nat_mul(a513, a514)?
        };
        nat_add(a515, a516)?
    };
    let a517 = v0;
    let a518 = 0u64;
    if nat_eq(a517, a518) {
        Ok(f25(0u64))
    } else {
        let a519 = v0;
        let a520 = v5;
        if nat_lt(a519, a520) {
            let v6: u64 = {
                let a521 = v0;
                let a522 = 1u64;
                nat_sub(a521, a522)
            };
            let v7: u64 = f26({
                let a528 = {
                    let a523 = v1;
                    let a524 = v4;
                    nat_mul(a523, a524)?
                };
                let a529 = {
                    let a525 = v6;
                    let a526 = f49(v3);
                    let a527 = 0u64;
                    nat_quot(a525, a526, a527)
                };
                nat_add(a528, a529)?
            }, v2)?;
            if v3 {
                let a533 = {
                    let a530 = v6;
                    let a531 = 2u64;
                    let a532 = 0u64;
                    nat_rem(a530, a531, a532)
                };
                let a534 = 0u64;
                if nat_eq(a533, a534) {
                    Ok(f27(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f27(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f27(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a535 = v0;
                let a536 = v5;
                nat_sub(a535, a536)
            };
            let a537 = v8;
            let a538 = 0u64;
            if nat_eq(a537, a538) {
                Ok(f27(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a539 = v8;
                    let a540 = 1u64;
                    nat_eq(a539, a540)
                } else {
                    false
                } {
                    Ok(f27(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a541 = v8;
                    let a542 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a541, a542) {
                        Ok(f27(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a543 = v8;
                        let a544 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a543, a544) {
                            Ok(f25(0u64))
                        } else {
                            Ok(Adt1::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f37(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: bool) -> R<Adt1> {
    let v5: u64 = if v4 {
        let a546 = 13u64;
        let a547 = f50(match v2.clone() {
            Adt2::C0(h545, _, _, _, _, _, _, _) => {
                h545
            }
        });
        nat_mul(a546, a547)?
    } else {
        0u64
    };
    let a548 = v0;
    let a549 = v5;
    if nat_lt(a548, a549) {
        f51({
            let a550 = v0;
            let a551 = 13u64;
            let a552 = 0u64;
            nat_rem(a550, a551, a552)
        }, v1, {
            let a553 = v0;
            let a554 = 13u64;
            let a555 = 0u64;
            nat_quot(a553, a554, a555)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a556 = v0;
            let a557 = v5;
            nat_sub(a556, a557)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f52({
            let a558 = v6;
            let a559 = v7;
            let a560 = 0u64;
            nat_rem(a558, a559, a560)
        }, {
            let a570 = {
                let a565 = {
                    let a562 = v1;
                    let a563 = match v2.clone() {
                        Adt2::C0(h561, _, _, _, _, _, _, _) => {
                            h561
                        }
                    };
                    nat_mul(a562, a563)?
                };
                let a566 = if v4 {
                    f53(match v2.clone() {
                        Adt2::C0(h564, _, _, _, _, _, _, _) => {
                            h564
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a565, a566)?
            };
            let a571 = {
                let a567 = v6;
                let a568 = v7;
                let a569 = 0u64;
                nat_quot(a567, a568, a569)
            };
            nat_add(a570, a571)?
        }, v2, v3, v4)
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt1 {
    let a572 = v0;
    let a573 = 0u64;
    if nat_eq(a572, a573) {
        f27(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a574 = v0;
        let a575 = 1u64;
        if nat_eq(a574, a575) {
            f27(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a576 = v0;
                let a577 = 2u64;
                nat_eq(a576, a577)
            } else {
                false
            } {
                f27(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a578 = v0;
                let a579 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a578, a579) {
                    f27(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a580 = v0;
                    let a581 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a580, a581) {
                        f27(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a582 = v0;
                        let a583 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a582, a583) {
                            f27(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt1::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a588 = v3;
    let a589 = {
        let a586 = {
            let a584 = f42(v2, v4)?;
            let a585 = f41(v0, v1, f40(v2, v4)?)?;
            nat_mul(a584, a585)?
        };
        let a587 = f41(v0, v1, f43(v2, v4)?)?;
        nat_add(a586, a587)?
    };
    nat_mul(a588, a589)
}

pub fn f40(v0: u64, v1: u64) -> R<u64> {
    let a594 = {
        let a592 = {
            let a590 = v0;
            let a591 = v1;
            nat_add(a590, a591)?
        };
        let a593 = 1u64;
        nat_sub(a592, a593)
    };
    let a595 = v1;
    let a596 = 0u64;
    Ok(nat_quot(a594, a595, a596))
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a597 = v2;
    let a598 = 0u64;
    if nat_eq(a597, a598) {
        Ok(0u64)
    } else {
        let a603 = 11u64;
        let a604 = {
            let a601 = {
                let a599 = v1;
                let a600 = v2;
                nat_mul(a599, a600)?
            };
            let a602 = f54(v0)?;
            nat_mul(a601, a602)?
        };
        nat_add(a603, a604)
    }
}

pub fn f42(v0: u64, v1: u64) -> R<u64> {
    let a605 = v0;
    let a606 = f40(v0, v1)?;
    let a607 = 0u64;
    Ok(nat_quot(a605, a606, a607))
}

pub fn f43(v0: u64, v1: u64) -> R<u64> {
    let a608 = v0;
    let a609 = f40(v0, v1)?;
    let a610 = 0u64;
    Ok(nat_rem(a608, a609, a610))
}

pub fn f44(v0: bool, v1: u64, v2: u64, v3: Adt3, v4: Adt2, v5: Adt2) -> R<Adt1> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a614 = 1u64;
        let a615 = {
            let a612 = match v4.clone() {
                Adt2::C0(h611, _, _, _, _, _, _, _) => {
                    h611
                }
            };
            let a613 = v6;
            nat_mul(a612, a613)?
        };
        nat_add(a614, a615)?
    };
    let a616 = v1;
    let a617 = 0u64;
    if nat_eq(a616, a617) {
        Ok(f25(1u64))
    } else {
        let a618 = v1;
        let a619 = v7;
        if nat_lt(a618, a619) {
            let v8: u64 = {
                let a620 = v1;
                let a621 = 1u64;
                nat_sub(a620, a621)
            };
            let v9: u64 = {
                let a628 = {
                    let a623 = f47(v2, v3, v4.clone())?;
                    let a624 = match v4.clone() {
                        Adt2::C0(h622, _, _, _, _, _, _, _) => {
                            h622
                        }
                    };
                    nat_mul(a623, a624)?
                };
                let a629 = {
                    let a625 = v8;
                    let a626 = v6;
                    let a627 = 0u64;
                    nat_quot(a625, a626, a627)
                };
                nat_add(a628, a629)?
            };
            let v10: u64 = f26(v9, v4)?;
            let v11: u64 = f26(v9, v5)?;
            let v12: u64 = {
                let a630 = v8;
                let a631 = v6;
                let a632 = 0u64;
                nat_rem(a630, a631, a632)
            };
            if v0 {
                let a633 = v12;
                let a634 = 0u64;
                if nat_eq(a633, a634) {
                    Ok(f27(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a635 = v12;
                    let a636 = 1u64;
                    if nat_eq(a635, a636) {
                        Ok(f27(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a637 = v12;
                        let a638 = 2u64;
                        if nat_eq(a637, a638) {
                            Ok(f27(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f27(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a639 = v12;
                let a640 = 0u64;
                if nat_eq(a639, a640) {
                    Ok(f27(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f27(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a641 = v1;
            let a642 = v7;
            if nat_eq(a641, a642) {
                Ok(f27(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt1::C0)
            }
        }
    }
}

pub fn f45(v0: u64, v1: u64) -> Adt1 {
    let a643 = v0;
    let a644 = 0u64;
    if nat_eq(a643, a644) {
        f27(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a645 = v0;
        let a646 = 1u64;
        if nat_eq(a645, a646) {
            f27(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a647 = v0;
            let a648 = 2u64;
            if nat_eq(a647, a648) {
                f27(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a649 = v0;
                let a650 = 3u64;
                if nat_eq(a649, a650) {
                    f25(0u64)
                } else {
                    Adt1::C0
                }
            }
        }
    }
}

pub fn f46(v0: u64, v1: u64, v2: u64) -> Adt1 {
    let a651 = v0;
    let a652 = 0u64;
    if nat_eq(a651, a652) {
        f27(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a653 = v0;
        let a654 = 1u64;
        if nat_eq(a653, a654) {
            f27(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a655 = v0;
            let a656 = 2u64;
            if nat_eq(a655, a656) {
                f27(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a657 = v0;
                let a658 = 3u64;
                if nat_eq(a657, a658) {
                    f27(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a659 = v0;
                    let a660 = 4u64;
                    if nat_eq(a659, a660) {
                        f27(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a661 = v0;
                        let a662 = 5u64;
                        if nat_eq(a661, a662) {
                            f27(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt1::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f47(v0: u64, v1: Adt3, v2: Adt2) -> R<u64> {
    let a673 = {
        let a671 = {
            let a668 = {
                let a665 = match v1.clone() {
                    Adt3::C0(_, _, h663, _) => {
                        h663
                    }
                };
                let a666 = match v2.clone() {
                    Adt2::C0(_, _, h664, _, _, _, _, _) => {
                        h664
                    }
                };
                nat_mul(a665, a666)?
            };
            let a669 = match v1 {
                Adt3::C0(h667, _, _, _) => {
                    h667
                }
            };
            nat_add(a668, a669)?
        };
        let a672 = match v2 {
            Adt2::C0(_, h670, _, _, _, _, _, _) => {
                h670
            }
        };
        nat_mul(a671, a672)?
    };
    let a674 = v0;
    nat_add(a673, a674)
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a687 = {
        let a683 = {
            let a679 = {
                let a675 = v0;
                let a676 = v4;
                nat_mul(a675, a676)?
            };
            let a680 = {
                let a677 = v1;
                let a678 = v5;
                nat_mul(a677, a678)?
            };
            nat_add(a679, a680)?
        };
        let a684 = {
            let a681 = v2;
            let a682 = v6;
            nat_mul(a681, a682)?
        };
        nat_add(a683, a684)?
    };
    let a688 = {
        let a685 = v3;
        let a686 = v7;
        nat_mul(a685, a686)?
    };
    nat_add(a687, a688)
}

pub fn f49(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f50(v0: u64) -> u64 {
    let a689 = v0;
    let a690 = 4u64;
    let a691 = 0u64;
    nat_quot(a689, a690, a691)
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: Adt2, v4: Adt2) -> R<Adt1> {
    let v5: u64 = {
        let a699 = {
            let a697 = {
                let a693 = v1;
                let a694 = match v3.clone() {
                    Adt2::C0(h692, _, _, _, _, _, _, _) => {
                        h692
                    }
                };
                nat_mul(a693, a694)?
            };
            let a698 = {
                let a695 = v2;
                let a696 = 4u64;
                nat_mul(a695, a696)?
            };
            nat_add(a697, a698)?
        };
        let a700 = f55(v0);
        nat_add(a699, a700)?
    };
    let v6: u64 = f26(v5, v3)?;
    let v7: u64 = f26(v5, v4)?;
    let a701 = v0;
    let a702 = 0u64;
    if if if if nat_eq(a701, a702) {
        true
    } else {
        let a703 = v0;
        let a704 = 2u64;
        nat_eq(a703, a704)
    } {
        true
    } else {
        let a705 = v0;
        let a706 = 5u64;
        nat_eq(a705, a706)
    } {
        true
    } else {
        let a707 = v0;
        let a708 = 7u64;
        nat_eq(a707, a708)
    } {
        Ok(f27(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a709 = v0;
        let a710 = 1u64;
        if if if if nat_eq(a709, a710) {
            true
        } else {
            let a711 = v0;
            let a712 = 3u64;
            nat_eq(a711, a712)
        } {
            true
        } else {
            let a713 = v0;
            let a714 = 6u64;
            nat_eq(a713, a714)
        } {
            true
        } else {
            let a715 = v0;
            let a716 = 8u64;
            nat_eq(a715, a716)
        } {
            Ok(f27(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a717 = v0;
                let a718 = 1u64;
                if nat_eq(a717, a718) {
                    4u64
                } else {
                    let a719 = v0;
                    let a720 = 6u64;
                    if nat_eq(a719, a720) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a721 = v0;
            let a722 = 4u64;
            if nat_eq(a721, a722) {
                Ok(f27(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a723 = v0;
                let a724 = 9u64;
                if nat_eq(a723, a724) {
                    Ok(f27(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a725 = v0;
                    let a726 = 10u64;
                    if nat_eq(a725, a726) {
                        Ok(f27(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a727 = v0;
                        let a728 = 11u64;
                        if nat_eq(a727, a728) {
                            Ok(f27(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a729 = v0;
                            let a730 = 12u64;
                            if nat_eq(a729, a730) {
                                Ok(f27(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                            } else {
                                Ok(Adt1::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f52(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: bool) -> R<Adt1> {
    let v5: u64 = f26(v1, v2)?;
    let v6: u64 = f26(v1, v3)?;
    let a731 = v0;
    let a732 = 0u64;
    if nat_eq(a731, a732) {
        Ok(f27(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a733 = v0;
        let a734 = 1u64;
        if nat_eq(a733, a734) {
            Ok(f27(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a735 = v0;
                let a736 = 2u64;
                if nat_eq(a735, a736) {
                    Ok(f27(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a737 = v0;
                    let a738 = 3u64;
                    if nat_eq(a737, a738) {
                        Ok(f27(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt1::C0)
                    }
                }
            } else {
                let a739 = v0;
                let a740 = 2u64;
                if nat_eq(a739, a740) {
                    Ok(f27(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    }
}

pub fn f53(v0: u64) -> R<u64> {
    let a741 = 4u64;
    let a742 = f50(v0);
    nat_mul(a741, a742)
}

pub fn f54(v0: u64) -> R<u64> {
    let a745 = {
        let a743 = 7u64;
        let a744 = v0;
        nat_mul(a743, a744)?
    };
    let a746 = 4u64;
    nat_add(a745, a746)
}

pub fn f55(v0: u64) -> u64 {
    let a747 = v0;
    let a748 = 5u64;
    if nat_lt(a747, a748) {
        let a749 = v0;
        let a750 = 2u64;
        if nat_lt(a749, a750) {
            0u64
        } else {
            1u64
        }
    } else {
        let a751 = v0;
        let a752 = 7u64;
        if nat_lt(a751, a752) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64, p10: u64, p11: u64, p12: u64, p13: u64, p14: u64, p15: u64, p16: u64, p17: u64, p18: u64) -> R<Adt1> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18)
}
