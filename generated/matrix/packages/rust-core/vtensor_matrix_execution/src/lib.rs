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
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64),
    C1(bool, u64, u64, u64),
    C2,
    C3,
    C4(u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(Adt3),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: Adt0, v9: Adt0, v10: Adt0, v11: u64, v12: u64, v13: u64) -> R<Adt1> {
    let v14: u64 = {
        let a1 = v13;
        let a2 = 30u64;
        if nat_eq(a1, a2) {
            f1(v11)
        } else {
            0u64
        }
    };
    let v15: Adt0 = {
        let a3 = 0u64;
        let a4 = v14;
        if nat_lt(a3, a4) {
            Adt0::C0(match v8.clone() {
                Adt0::C0(_, h5, _, _) => {
                    h5
                }
            }, 0u64, match v8.clone() {
                Adt0::C0(_, _, h6, _) => {
                    h6
                }
            }, match v8.clone() {
                Adt0::C0(_, _, _, h7) => {
                    h7
                }
            })
        } else {
            f2(v13, v8.clone())
        }
    };
    let m8 = f3(v0, v1, v2, v3, v4, v5, v6, v7, v15, f2(v13, v9), v10)?;
    match m8 {
        Adt1::C0(v20, v21) => {
            Ok(Adt1::C0(v20, v21))
        }
        Adt1::C1(v16, v17, v18, v19) => {
            if v16 {
                Ok(Adt1::C1(true, v17, v18, v19))
            } else {
                let a9 = v17;
                let a10 = 0u64;
                if if nat_eq(a9, a10) {
                    let a11 = 0u64;
                    let a12 = v14;
                    nat_lt(a11, a12)
                } else {
                    false
                } {
                    f4(v18, v19, v2, v11, v14, match v8 {
                        Adt0::C0(h13, _, _, _) => {
                            h13
                        }
                    })
                } else {
                    Ok(f5(v17, v18, v19, {
                        let a14 = v17;
                        let a15 = 0u64;
                        if nat_eq(a14, a15) {
                            v11
                        } else {
                            v12
                        }
                    }, f6(v11)))
                }
            }
        }
        Adt1::C2 => {
            Ok(Adt1::C2)
        }
        Adt1::C3 => {
            Ok(Adt1::C3)
        }
        Adt1::C4(v22, v23, v24, v25, v26, v27) => {
            Ok(Adt1::C4(v22, v23, v24, v25, v26, v27))
        }
        Adt1::C5(v28, v29, v30, v31, v32, v33) => {
            Ok(Adt1::C5(v28, v29, v30, v31, v32, v33))
        }
    }
}

pub fn f1(v0: u64) -> u64 {
    if f7(v0) {
        let m16 = f8(v0);
        match m16 {
            Adt2::C0 => {
                0u64
            }
            Adt2::C1(v1) => {
                let a18 = 1u64;
                let a19 = match v1.clone() {
                    Adt3::C0(_, h17, _) => {
                        h17
                    }
                };
                if if nat_lt(a18, a19) {
                    true
                } else {
                    let a20 = v0;
                    let a21 = 43u64;
                    nat_eq(a20, a21)
                } {
                    match v1 {
                        Adt3::C0(_, h22, _) => {
                            h22
                        }
                    }
                } else {
                    0u64
                }
            }
        }
    } else {
        0u64
    }
}

pub fn f2(v0: u64, v1: Adt0) -> Adt0 {
    let a23 = v0;
    let a24 = 30u64;
    if nat_eq(a23, a24) {
        Adt0::C0(match v1.clone() {
            Adt0::C0(_, h25, _, _) => {
                h25
            }
        }, match v1.clone() {
            Adt0::C0(h26, _, _, _) => {
                h26
            }
        }, match v1.clone() {
            Adt0::C0(_, _, h27, _) => {
                h27
            }
        }, match v1 {
            Adt0::C0(_, _, _, h28) => {
                h28
            }
        })
    } else {
        v1
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: Adt0, v9: Adt0, v10: Adt0) -> R<Adt1> {
    if f9(v1, v3, v4, v5) {
        let a29 = v0;
        let a30 = 0u64;
        if nat_eq(a29, a30) {
            Ok(Adt1::C0(1u64, 0u64))
        } else {
            Ok(Adt1::C3)
        }
    } else {
        let a31 = 0u64;
        let a32 = v6;
        if if nat_lt(a31, a32) {
            let a33 = 0u64;
            let a34 = v7;
            nat_lt(a33, a34)
        } else {
            false
        } {
            let v11: u64 = {
                let a35 = v1;
                let a36 = v2;
                nat_mul(a35, a36)?
            };
            let v12: u64 = {
                let a37 = v2;
                let a38 = v3;
                nat_mul(a37, a38)?
            };
            let v13: u64 = {
                let a39 = v1;
                let a40 = v3;
                nat_mul(a39, a40)?
            };
            let v14: u64 = {
                let a45 = {
                    let a43 = {
                        let a41 = v11;
                        let a42 = v12;
                        nat_add(a41, a42)?
                    };
                    let a44 = 1u64;
                    nat_add(a43, a44)?
                };
                let a46 = v13;
                nat_add(a45, a46)?
            };
            let v15: u64 = {
                let a51 = 1u64;
                let a52 = {
                    let a49 = {
                        let a47 = v4;
                        let a48 = v5;
                        nat_mul(a47, a48)?
                    };
                    let a50 = v14;
                    nat_mul(a49, a50)?
                };
                nat_add(a51, a52)?
            };
            let a53 = v0;
            let a54 = 0u64;
            if nat_eq(a53, a54) {
                Ok(Adt1::C0(v15, {
                    let a57 = {
                        let a55 = v11;
                        let a56 = v12;
                        nat_add(a55, a56)?
                    };
                    let a58 = v13;
                    nat_add(a57, a58)?
                }))
            } else {
                let a59 = v0;
                let a60 = v15;
                if nat_lt(a59, a60) {
                    let v16: u64 = {
                        let a61 = v0;
                        let a62 = 1u64;
                        nat_sub(a61, a62)
                    };
                    let v17: u64 = {
                        let a63 = v16;
                        let a64 = v14;
                        let a65 = 0u64;
                        nat_quot(a63, a64, a65)
                    };
                    let v18: u64 = {
                        let a66 = v16;
                        let a67 = v14;
                        let a68 = 0u64;
                        nat_rem(a66, a67, a68)
                    };
                    let v19: u64 = {
                        let a69 = v17;
                        let a70 = v4;
                        let a71 = 0u64;
                        nat_rem(a69, a70, a71)
                    };
                    let v20: u64 = {
                        let a72 = v17;
                        let a73 = v4;
                        let a74 = 0u64;
                        nat_quot(a72, a73, a74)
                    };
                    let a75 = v18;
                    let a76 = v11;
                    if nat_lt(a75, a76) {
                        Ok(Adt1::C1(false, 0u64, v18, f10({
                            let a77 = v18;
                            let a78 = v2;
                            let a79 = 0u64;
                            nat_rem(a77, a78, a79)
                        }, {
                            let a80 = v18;
                            let a81 = v2;
                            let a82 = 0u64;
                            nat_quot(a80, a81, a82)
                        }, {
                            let a83 = v19;
                            let a84 = v6;
                            let a85 = 0u64;
                            nat_quot(a83, a84, a85)
                        }, {
                            let a86 = v20;
                            let a87 = v7;
                            let a88 = 0u64;
                            nat_quot(a86, a87, a88)
                        }, v8)?))
                    } else {
                        let a91 = v18;
                        let a92 = {
                            let a89 = v11;
                            let a90 = v12;
                            nat_add(a89, a90)?
                        };
                        if nat_lt(a91, a92) {
                            let v21: u64 = {
                                let a93 = v18;
                                let a94 = v11;
                                nat_sub(a93, a94)
                            };
                            Ok(Adt1::C1(false, 1u64, v18, f10({
                                let a95 = v21;
                                let a96 = v3;
                                let a97 = 0u64;
                                nat_quot(a95, a96, a97)
                            }, {
                                let a98 = v21;
                                let a99 = v3;
                                let a100 = 0u64;
                                nat_rem(a98, a99, a100)
                            }, v19, v20, v9)?))
                        } else {
                            let a103 = v18;
                            let a104 = {
                                let a101 = v11;
                                let a102 = v12;
                                nat_add(a101, a102)?
                            };
                            if nat_eq(a103, a104) {
                                Ok(Adt1::C2)
                            } else {
                                let v22: u64 = {
                                    let a109 = {
                                        let a107 = v18;
                                        let a108 = {
                                            let a105 = v11;
                                            let a106 = v12;
                                            nat_add(a105, a106)?
                                        };
                                        nat_sub(a107, a108)
                                    };
                                    let a110 = 1u64;
                                    nat_sub(a109, a110)
                                };
                                Ok(Adt1::C1(true, 2u64, {
                                    let a113 = {
                                        let a111 = v11;
                                        let a112 = v12;
                                        nat_add(a111, a112)?
                                    };
                                    let a114 = v22;
                                    nat_add(a113, a114)?
                                }, f10({
                                    let a115 = v22;
                                    let a116 = v3;
                                    let a117 = 0u64;
                                    nat_quot(a115, a116, a117)
                                }, {
                                    let a118 = v22;
                                    let a119 = v3;
                                    let a120 = 0u64;
                                    nat_rem(a118, a119, a120)
                                }, v19, v20, v10)?))
                            }
                        }
                    }
                } else {
                    Ok(Adt1::C3)
                }
            }
        } else {
            Ok(Adt1::C3)
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let m121 = f8(v3);
    match m121 {
        Adt2::C0 => {
            Ok(Adt1::C3)
        }
        Adt2::C1(v6) => {
            let v7: u64 = {
                let a122 = v0;
                let a123 = v2;
                let a124 = 0u64;
                nat_quot(a122, a123, a124)
            };
            let v8: u64 = {
                let a125 = v7;
                let a126 = v4;
                let a127 = 0u64;
                nat_quot(a125, a126, a127)
            };
            let v9: u64 = {
                let a133 = {
                    let a131 = {
                        let a128 = v0;
                        let a129 = v2;
                        let a130 = 0u64;
                        nat_rem(a128, a129, a130)
                    };
                    let a132 = 1u64;
                    nat_add(a131, a132)?
                };
                let a134 = v2;
                if nat_eq(a133, a134) {
                    let a137 = {
                        let a135 = v4;
                        let a136 = 1u64;
                        nat_sub(a135, a136)
                    };
                    let a138 = v2;
                    nat_mul(a137, a138)?
                } else {
                    0u64
                }
            };
            Ok(Adt1::C5(v0, {
                let a141 = v1;
                let a142 = {
                    let a139 = v8;
                    let a140 = v5;
                    nat_mul(a139, a140)?
                };
                nat_add(a141, a142)?
            }, v3, match v6 {
                Adt3::C0(_, _, h143) => {
                    h143
                }
            }, {
                let a144 = v2;
                let a145 = 4u64;
                nat_mul(a144, a145)?
            }, v9))
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt1 {
    let a146 = v3;
    let a147 = 0u64;
    if if nat_eq(a146, a147) {
        let a148 = v4;
        let a149 = 0u64;
        nat_eq(a148, a149)
    } else {
        false
    } {
        Adt1::C1(false, v0, v1, v2)
    } else {
        Adt1::C4(v0, v1, v2, v3, v4, {
            let a150 = v3;
            let a151 = 43u64;
            if nat_eq(a150, a151) {
                1u64
            } else {
                let a152 = v3;
                let a153 = 0u64;
                if nat_eq(a152, a153) {
                    4u64
                } else {
                    2u64
                }
            }
        })
    }
}

pub fn f6(v0: u64) -> u64 {
    let a154 = v0;
    let a155 = 30u64;
    if nat_eq(a154, a155) {
        30u64
    } else {
        0u64
    }
}

pub fn f7(v0: u64) -> bool {
    let a156 = v0;
    let a157 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a156, a157) {
        true
    } else {
        let a158 = v0;
        let a159 = 1u64;
        nat_eq(a158, a159)
    } {
        true
    } else {
        let a160 = v0;
        let a161 = 2u64;
        nat_eq(a160, a161)
    } {
        true
    } else {
        let a162 = v0;
        let a163 = 3u64;
        nat_eq(a162, a163)
    } {
        true
    } else {
        let a164 = v0;
        let a165 = 6u64;
        nat_eq(a164, a165)
    } {
        true
    } else {
        let a166 = v0;
        let a167 = 7u64;
        nat_eq(a166, a167)
    } {
        true
    } else {
        let a168 = v0;
        let a169 = 8u64;
        nat_eq(a168, a169)
    } {
        true
    } else {
        let a170 = v0;
        let a171 = 10u64;
        nat_eq(a170, a171)
    } {
        true
    } else {
        let a172 = v0;
        let a173 = 11u64;
        nat_eq(a172, a173)
    } {
        true
    } else {
        let a174 = v0;
        let a175 = 12u64;
        nat_eq(a174, a175)
    } {
        true
    } else {
        let a176 = v0;
        let a177 = 13u64;
        nat_eq(a176, a177)
    } {
        true
    } else {
        let a178 = v0;
        let a179 = 14u64;
        nat_eq(a178, a179)
    } {
        true
    } else {
        let a180 = v0;
        let a181 = 16u64;
        nat_eq(a180, a181)
    } {
        true
    } else {
        let a182 = v0;
        let a183 = 17u64;
        nat_eq(a182, a183)
    } {
        true
    } else {
        let a184 = v0;
        let a185 = 18u64;
        nat_eq(a184, a185)
    } {
        true
    } else {
        let a186 = v0;
        let a187 = 19u64;
        nat_eq(a186, a187)
    } {
        true
    } else {
        let a188 = v0;
        let a189 = 20u64;
        nat_eq(a188, a189)
    } {
        true
    } else {
        let a190 = v0;
        let a191 = 21u64;
        nat_eq(a190, a191)
    } {
        true
    } else {
        let a192 = v0;
        let a193 = 22u64;
        nat_eq(a192, a193)
    } {
        true
    } else {
        let a194 = v0;
        let a195 = 23u64;
        nat_eq(a194, a195)
    } {
        true
    } else {
        let a196 = v0;
        let a197 = 26u64;
        nat_eq(a196, a197)
    } {
        true
    } else {
        let a198 = v0;
        let a199 = 29u64;
        nat_eq(a198, a199)
    } {
        true
    } else {
        let a200 = v0;
        let a201 = 30u64;
        nat_eq(a200, a201)
    } {
        true
    } else {
        let a202 = v0;
        let a203 = 34u64;
        nat_eq(a202, a203)
    } {
        true
    } else {
        let a204 = v0;
        let a205 = 35u64;
        nat_eq(a204, a205)
    } {
        true
    } else {
        let a206 = v0;
        let a207 = 39u64;
        nat_eq(a206, a207)
    } {
        true
    } else {
        let a208 = v0;
        let a209 = 40u64;
        nat_eq(a208, a209)
    } {
        true
    } else {
        let a210 = v0;
        let a211 = 41u64;
        nat_eq(a210, a211)
    } {
        true
    } else {
        let a212 = v0;
        let a213 = 42u64;
        nat_eq(a212, a213)
    } {
        true
    } else {
        let a214 = v0;
        let a215 = 43u64;
        nat_eq(a214, a215)
    }
}

pub fn f8(v0: u64) -> Adt2 {
    let a216 = v0;
    let a217 = 0u64;
    if nat_eq(a216, a217) {
        Adt2::C1(Adt3::C0(0u64, 1u64, 4u64))
    } else {
        let a218 = v0;
        let a219 = 1u64;
        if nat_eq(a218, a219) {
            Adt2::C1(Adt3::C0(1u64, 1u64, 2u64))
        } else {
            let a220 = v0;
            let a221 = 2u64;
            if nat_eq(a220, a221) {
                Adt2::C1(Adt3::C0(2u64, 32u64, 18u64))
            } else {
                let a222 = v0;
                let a223 = 3u64;
                if nat_eq(a222, a223) {
                    Adt2::C1(Adt3::C0(3u64, 32u64, 20u64))
                } else {
                    let a224 = v0;
                    let a225 = 6u64;
                    if nat_eq(a224, a225) {
                        Adt2::C1(Adt3::C0(6u64, 32u64, 22u64))
                    } else {
                        let a226 = v0;
                        let a227 = 7u64;
                        if nat_eq(a226, a227) {
                            Adt2::C1(Adt3::C0(7u64, 32u64, 24u64))
                        } else {
                            let a228 = v0;
                            let a229 = 8u64;
                            if nat_eq(a228, a229) {
                                Adt2::C1(Adt3::C0(8u64, 32u64, 34u64))
                            } else {
                                let a230 = v0;
                                let a231 = 9u64;
                                if nat_eq(a230, a231) {
                                    Adt2::C1(Adt3::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a232 = v0;
                                    let a233 = 10u64;
                                    if nat_eq(a232, a233) {
                                        Adt2::C1(Adt3::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a234 = v0;
                                        let a235 = 11u64;
                                        if nat_eq(a234, a235) {
                                            Adt2::C1(Adt3::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a236 = v0;
                                            let a237 = 12u64;
                                            if nat_eq(a236, a237) {
                                                Adt2::C1(Adt3::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a238 = v0;
                                                let a239 = 13u64;
                                                if nat_eq(a238, a239) {
                                                    Adt2::C1(Adt3::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a240 = v0;
                                                    let a241 = 14u64;
                                                    if nat_eq(a240, a241) {
                                                        Adt2::C1(Adt3::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a242 = v0;
                                                        let a243 = 15u64;
                                                        if nat_eq(a242, a243) {
                                                            Adt2::C1(Adt3::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a244 = v0;
                                                            let a245 = 16u64;
                                                            if nat_eq(a244, a245) {
                                                                Adt2::C1(Adt3::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a246 = v0;
                                                                let a247 = 17u64;
                                                                if nat_eq(a246, a247) {
                                                                    Adt2::C1(Adt3::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a248 = v0;
                                                                    let a249 = 18u64;
                                                                    if nat_eq(a248, a249) {
                                                                        Adt2::C1(Adt3::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a250 = v0;
                                                                        let a251 = 19u64;
                                                                        if nat_eq(a250, a251) {
                                                                            Adt2::C1(Adt3::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a252 = v0;
                                                                            let a253 = 20u64;
                                                                            if nat_eq(a252, a253) {
                                                                                Adt2::C1(Adt3::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a254 = v0;
                                                                                let a255 = 21u64;
                                                                                if nat_eq(a254, a255) {
                                                                                    Adt2::C1(Adt3::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a256 = v0;
                                                                                    let a257 = 22u64;
                                                                                    if nat_eq(a256, a257) {
                                                                                        Adt2::C1(Adt3::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a258 = v0;
                                                                                        let a259 = 23u64;
                                                                                        if nat_eq(a258, a259) {
                                                                                            Adt2::C1(Adt3::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a260 = v0;
                                                                                            let a261 = 24u64;
                                                                                            if nat_eq(a260, a261) {
                                                                                                Adt2::C1(Adt3::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a262 = v0;
                                                                                                let a263 = 25u64;
                                                                                                if nat_eq(a262, a263) {
                                                                                                    Adt2::C1(Adt3::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a264 = v0;
                                                                                                    let a265 = 26u64;
                                                                                                    if nat_eq(a264, a265) {
                                                                                                        Adt2::C1(Adt3::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a266 = v0;
                                                                                                        let a267 = 27u64;
                                                                                                        if nat_eq(a266, a267) {
                                                                                                            Adt2::C1(Adt3::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a268 = v0;
                                                                                                            let a269 = 28u64;
                                                                                                            if nat_eq(a268, a269) {
                                                                                                                Adt2::C1(Adt3::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a270 = v0;
                                                                                                                let a271 = 29u64;
                                                                                                                if nat_eq(a270, a271) {
                                                                                                                    Adt2::C1(Adt3::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a272 = v0;
                                                                                                                    let a273 = 30u64;
                                                                                                                    if nat_eq(a272, a273) {
                                                                                                                        Adt2::C1(Adt3::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a274 = v0;
                                                                                                                        let a275 = 34u64;
                                                                                                                        if nat_eq(a274, a275) {
                                                                                                                            Adt2::C1(Adt3::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a276 = v0;
                                                                                                                            let a277 = 35u64;
                                                                                                                            if nat_eq(a276, a277) {
                                                                                                                                Adt2::C1(Adt3::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a278 = v0;
                                                                                                                                let a279 = 39u64;
                                                                                                                                if nat_eq(a278, a279) {
                                                                                                                                    Adt2::C1(Adt3::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a280 = v0;
                                                                                                                                    let a281 = 40u64;
                                                                                                                                    if nat_eq(a280, a281) {
                                                                                                                                        Adt2::C1(Adt3::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a282 = v0;
                                                                                                                                        let a283 = 41u64;
                                                                                                                                        if nat_eq(a282, a283) {
                                                                                                                                            Adt2::C1(Adt3::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a284 = v0;
                                                                                                                                            let a285 = 42u64;
                                                                                                                                            if nat_eq(a284, a285) {
                                                                                                                                                Adt2::C1(Adt3::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a286 = v0;
                                                                                                                                                let a287 = 43u64;
                                                                                                                                                if nat_eq(a286, a287) {
                                                                                                                                                    Adt2::C1(Adt3::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt2::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a288 = v0;
    let a289 = 0u64;
    if if if nat_eq(a288, a289) {
        true
    } else {
        let a290 = v1;
        let a291 = 0u64;
        nat_eq(a290, a291)
    } {
        true
    } else {
        let a292 = v2;
        let a293 = 0u64;
        nat_eq(a292, a293)
    } {
        true
    } else {
        let a294 = v3;
        let a295 = 0u64;
        nat_eq(a294, a295)
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt0) -> R<u64> {
    let a312 = {
        let a307 = {
            let a302 = {
                let a297 = v0;
                let a298 = match v4.clone() {
                    Adt0::C0(h296, _, _, _) => {
                        h296
                    }
                };
                nat_mul(a297, a298)?
            };
            let a303 = {
                let a300 = v1;
                let a301 = match v4.clone() {
                    Adt0::C0(_, h299, _, _) => {
                        h299
                    }
                };
                nat_mul(a300, a301)?
            };
            nat_add(a302, a303)?
        };
        let a308 = {
            let a305 = v2;
            let a306 = match v4.clone() {
                Adt0::C0(_, _, h304, _) => {
                    h304
                }
            };
            nat_mul(a305, a306)?
        };
        nat_add(a307, a308)?
    };
    let a313 = {
        let a310 = v3;
        let a311 = match v4 {
            Adt0::C0(_, _, _, h309) => {
                h309
            }
        };
        nat_mul(a310, a311)?
    };
    nat_add(a312, a313)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: Adt0, p9: Adt0, p10: Adt0, p11: u64, p12: u64, p13: u64) -> R<Adt1> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
}
