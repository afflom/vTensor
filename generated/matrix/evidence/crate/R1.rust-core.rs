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
    C5(u64, u64, u64, u64, u64, u64, u64),
    C6(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(Adt4),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: Adt0, v9: Adt0, v10: Adt0, v11: u64, v12: u64, v13: u64) -> R<Adt1> {
    let v14: bool = {
        let a1 = v13;
        let a2 = 30u64;
        nat_eq(a1, a2)
    };
    let m3 = f1(v0, v1, v2, v3, v4, v5, v6, v7, if v14 {
        f2(v8.clone())
    } else {
        v8.clone()
    }, if v14 {
        f2(v9.clone())
    } else {
        v9.clone()
    }, v10)?;
    match m3 {
        Adt1::C0(v19, v20) => {
            Ok(Adt1::C0(v19, v20))
        }
        Adt1::C1(v15, v16, v17, v18) => {
            if v15 {
                Ok(Adt1::C1(true, v16, v17, v18))
            } else {
                if v14 {
                    let a4 = v16;
                    let a5 = 0u64;
                    if nat_eq(a4, a5) {
                        f3(v16, v17, v18, {
                            let a6 = v17;
                            let a7 = v2;
                            let a8 = 0u64;
                            nat_quot(a6, a7, a8)
                        }, v2, {
                            let a14 = {
                                let a12 = {
                                    let a9 = v17;
                                    let a10 = v2;
                                    let a11 = 0u64;
                                    nat_rem(a9, a10, a11)
                                };
                                let a13 = 1u64;
                                nat_add(a12, a13)?
                            };
                            let a15 = v2;
                            nat_eq(a14, a15)
                        }, v11, match v8 {
                            Adt0::C0(h16, _, _, _) => {
                                h16
                            }
                        })
                    } else {
                        f3(v16, v17, v18, {
                            let a21 = {
                                let a19 = v17;
                                let a20 = {
                                    let a17 = v1;
                                    let a18 = v2;
                                    nat_mul(a17, a18)?
                                };
                                nat_sub(a19, a20)
                            };
                            let a22 = v3;
                            let a23 = 0u64;
                            nat_rem(a21, a22, a23)
                        }, 1u64, true, v12, match v9 {
                            Adt0::C0(h24, _, _, _) => {
                                h24
                            }
                        })
                    }
                } else {
                    Ok(f4(v16, v17, v18, {
                        let a25 = v16;
                        let a26 = 0u64;
                        if nat_eq(a25, a26) {
                            v11
                        } else {
                            v12
                        }
                    }, f5(v11)))
                }
            }
        }
        Adt1::C2 => {
            let a27 = v13;
            let a28 = 28u64;
            if nat_eq(a27, a28) {
                let m29 = f6(v11);
                match m29 {
                    Adt2::C0 => {
                        Ok(Adt1::C2)
                    }
                    Adt2::C1(v21, v22, v23, v24) => {
                        Ok(Adt1::C6(v21, v22, v23, v24))
                    }
                }
            } else {
                Ok(Adt1::C2)
            }
        }
        Adt1::C3 => {
            Ok(Adt1::C3)
        }
        Adt1::C4(v29, v30, v31, v32, v33, v34) => {
            Ok(Adt1::C4(v29, v30, v31, v32, v33, v34))
        }
        Adt1::C5(v35, v36, v37, v38, v39, v40, v41) => {
            Ok(Adt1::C5(v35, v36, v37, v38, v39, v40, v41))
        }
        Adt1::C6(v25, v26, v27, v28) => {
            Ok(Adt1::C6(v25, v26, v27, v28))
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: Adt0, v9: Adt0, v10: Adt0) -> R<Adt1> {
    if f7(v1, v3, v4, v5) {
        let a30 = v0;
        let a31 = 0u64;
        if nat_eq(a30, a31) {
            Ok(Adt1::C0(1u64, 0u64))
        } else {
            Ok(Adt1::C3)
        }
    } else {
        let a32 = 0u64;
        let a33 = v6;
        if if nat_lt(a32, a33) {
            let a34 = 0u64;
            let a35 = v7;
            nat_lt(a34, a35)
        } else {
            false
        } {
            let v11: u64 = {
                let a36 = v1;
                let a37 = v2;
                nat_mul(a36, a37)?
            };
            let v12: u64 = {
                let a38 = v2;
                let a39 = v3;
                nat_mul(a38, a39)?
            };
            let v13: u64 = {
                let a40 = v1;
                let a41 = v3;
                nat_mul(a40, a41)?
            };
            let v14: u64 = {
                let a46 = {
                    let a44 = {
                        let a42 = v11;
                        let a43 = v12;
                        nat_add(a42, a43)?
                    };
                    let a45 = 1u64;
                    nat_add(a44, a45)?
                };
                let a47 = v13;
                nat_add(a46, a47)?
            };
            let v15: u64 = {
                let a52 = 1u64;
                let a53 = {
                    let a50 = {
                        let a48 = v4;
                        let a49 = v5;
                        nat_mul(a48, a49)?
                    };
                    let a51 = v14;
                    nat_mul(a50, a51)?
                };
                nat_add(a52, a53)?
            };
            let a54 = v0;
            let a55 = 0u64;
            if nat_eq(a54, a55) {
                Ok(Adt1::C0(v15, {
                    let a58 = {
                        let a56 = v11;
                        let a57 = v12;
                        nat_add(a56, a57)?
                    };
                    let a59 = v13;
                    nat_add(a58, a59)?
                }))
            } else {
                let a60 = v0;
                let a61 = v15;
                if nat_lt(a60, a61) {
                    let v16: u64 = {
                        let a62 = v0;
                        let a63 = 1u64;
                        nat_sub(a62, a63)
                    };
                    let v17: u64 = {
                        let a64 = v16;
                        let a65 = v14;
                        let a66 = 0u64;
                        nat_quot(a64, a65, a66)
                    };
                    let v18: u64 = {
                        let a67 = v16;
                        let a68 = v14;
                        let a69 = 0u64;
                        nat_rem(a67, a68, a69)
                    };
                    let v19: u64 = {
                        let a70 = v17;
                        let a71 = v4;
                        let a72 = 0u64;
                        nat_rem(a70, a71, a72)
                    };
                    let v20: u64 = {
                        let a73 = v17;
                        let a74 = v4;
                        let a75 = 0u64;
                        nat_quot(a73, a74, a75)
                    };
                    let a76 = v18;
                    let a77 = v11;
                    if nat_lt(a76, a77) {
                        Ok(Adt1::C1(false, 0u64, v18, f8({
                            let a78 = v18;
                            let a79 = v2;
                            let a80 = 0u64;
                            nat_rem(a78, a79, a80)
                        }, {
                            let a81 = v18;
                            let a82 = v2;
                            let a83 = 0u64;
                            nat_quot(a81, a82, a83)
                        }, {
                            let a84 = v19;
                            let a85 = v6;
                            let a86 = 0u64;
                            nat_quot(a84, a85, a86)
                        }, {
                            let a87 = v20;
                            let a88 = v7;
                            let a89 = 0u64;
                            nat_quot(a87, a88, a89)
                        }, v8)?))
                    } else {
                        let a92 = v18;
                        let a93 = {
                            let a90 = v11;
                            let a91 = v12;
                            nat_add(a90, a91)?
                        };
                        if nat_lt(a92, a93) {
                            let v21: u64 = {
                                let a94 = v18;
                                let a95 = v11;
                                nat_sub(a94, a95)
                            };
                            Ok(Adt1::C1(false, 1u64, v18, f8({
                                let a96 = v21;
                                let a97 = v3;
                                let a98 = 0u64;
                                nat_quot(a96, a97, a98)
                            }, {
                                let a99 = v21;
                                let a100 = v3;
                                let a101 = 0u64;
                                nat_rem(a99, a100, a101)
                            }, v19, v20, v9)?))
                        } else {
                            let a104 = v18;
                            let a105 = {
                                let a102 = v11;
                                let a103 = v12;
                                nat_add(a102, a103)?
                            };
                            if nat_eq(a104, a105) {
                                Ok(Adt1::C2)
                            } else {
                                let v22: u64 = {
                                    let a110 = {
                                        let a108 = v18;
                                        let a109 = {
                                            let a106 = v11;
                                            let a107 = v12;
                                            nat_add(a106, a107)?
                                        };
                                        nat_sub(a108, a109)
                                    };
                                    let a111 = 1u64;
                                    nat_sub(a110, a111)
                                };
                                Ok(Adt1::C1(true, 2u64, {
                                    let a114 = {
                                        let a112 = v11;
                                        let a113 = v12;
                                        nat_add(a112, a113)?
                                    };
                                    let a115 = v22;
                                    nat_add(a114, a115)?
                                }, f8({
                                    let a116 = v22;
                                    let a117 = v3;
                                    let a118 = 0u64;
                                    nat_quot(a116, a117, a118)
                                }, {
                                    let a119 = v22;
                                    let a120 = v3;
                                    let a121 = 0u64;
                                    nat_rem(a119, a120, a121)
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

pub fn f2(v0: Adt0) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(_, h122, _, _) => {
            h122
        }
    }, 0u64, match v0.clone() {
        Adt0::C0(_, _, h123, _) => {
            h123
        }
    }, match v0 {
        Adt0::C0(_, _, _, h124) => {
            h124
        }
    })
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64) -> R<Adt1> {
    let m125 = f9(v6);
    match m125 {
        Adt3::C0 => {
            Ok(Adt1::C3)
        }
        Adt3::C1(v8) => {
            let v9: u64 = match v8.clone() {
                Adt4::C0(_, h126, _) => {
                    h126
                }
            };
            let v10: u64 = {
                let a127 = v3;
                let a128 = v9;
                let a129 = 0u64;
                nat_quot(a127, a128, a129)
            };
            let v11: u64 = if v5 {
                let a132 = {
                    let a130 = v9;
                    let a131 = 1u64;
                    nat_sub(a130, a131)
                };
                let a133 = v4;
                nat_mul(a132, a133)?
            } else {
                0u64
            };
            Ok(Adt1::C5(v0, v1, {
                let a136 = v2;
                let a137 = {
                    let a134 = v10;
                    let a135 = v7;
                    nat_mul(a134, a135)?
                };
                nat_add(a136, a137)?
            }, v6, match v8 {
                Adt4::C0(_, _, h138) => {
                    h138
                }
            }, {
                let a139 = v4;
                let a140 = 4u64;
                nat_mul(a139, a140)?
            }, v11))
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt1 {
    let a141 = v3;
    let a142 = 0u64;
    if if nat_eq(a141, a142) {
        let a143 = v4;
        let a144 = 0u64;
        nat_eq(a143, a144)
    } else {
        false
    } {
        Adt1::C1(false, v0, v1, v2)
    } else {
        Adt1::C4(v0, v1, v2, v3, v4, {
            let a145 = v3;
            let a146 = 43u64;
            if nat_eq(a145, a146) {
                1u64
            } else {
                let a147 = v3;
                let a148 = 0u64;
                if nat_eq(a147, a148) {
                    4u64
                } else {
                    2u64
                }
            }
        })
    }
}

pub fn f5(v0: u64) -> u64 {
    let a149 = v0;
    let a150 = 30u64;
    if nat_eq(a149, a150) {
        30u64
    } else {
        0u64
    }
}

pub fn f6(v0: u64) -> Adt2 {
    let a151 = v0;
    let a152 = 0u64;
    if if if if nat_eq(a151, a152) {
        true
    } else {
        let a153 = v0;
        let a154 = 1u64;
        nat_eq(a153, a154)
    } {
        true
    } else {
        let a155 = v0;
        let a156 = 30u64;
        nat_eq(a155, a156)
    } {
        true
    } else {
        let a157 = v0;
        let a158 = 43u64;
        nat_eq(a157, a158)
    } {
        Adt2::C1(2u64, 1u64, 0u64, 2u64)
    } else {
        Adt2::C0
    }
}

pub fn f7(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a159 = v0;
    let a160 = 0u64;
    if if if nat_eq(a159, a160) {
        true
    } else {
        let a161 = v1;
        let a162 = 0u64;
        nat_eq(a161, a162)
    } {
        true
    } else {
        let a163 = v2;
        let a164 = 0u64;
        nat_eq(a163, a164)
    } {
        true
    } else {
        let a165 = v3;
        let a166 = 0u64;
        nat_eq(a165, a166)
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt0) -> R<u64> {
    let a183 = {
        let a178 = {
            let a173 = {
                let a168 = v0;
                let a169 = match v4.clone() {
                    Adt0::C0(h167, _, _, _) => {
                        h167
                    }
                };
                nat_mul(a168, a169)?
            };
            let a174 = {
                let a171 = v1;
                let a172 = match v4.clone() {
                    Adt0::C0(_, h170, _, _) => {
                        h170
                    }
                };
                nat_mul(a171, a172)?
            };
            nat_add(a173, a174)?
        };
        let a179 = {
            let a176 = v2;
            let a177 = match v4.clone() {
                Adt0::C0(_, _, h175, _) => {
                    h175
                }
            };
            nat_mul(a176, a177)?
        };
        nat_add(a178, a179)?
    };
    let a184 = {
        let a181 = v3;
        let a182 = match v4 {
            Adt0::C0(_, _, _, h180) => {
                h180
            }
        };
        nat_mul(a181, a182)?
    };
    nat_add(a183, a184)
}

pub fn f9(v0: u64) -> Adt3 {
    let a185 = v0;
    let a186 = 0u64;
    if nat_eq(a185, a186) {
        Adt3::C1(Adt4::C0(0u64, 1u64, 4u64))
    } else {
        let a187 = v0;
        let a188 = 1u64;
        if nat_eq(a187, a188) {
            Adt3::C1(Adt4::C0(1u64, 1u64, 2u64))
        } else {
            let a189 = v0;
            let a190 = 2u64;
            if nat_eq(a189, a190) {
                Adt3::C1(Adt4::C0(2u64, 32u64, 18u64))
            } else {
                let a191 = v0;
                let a192 = 3u64;
                if nat_eq(a191, a192) {
                    Adt3::C1(Adt4::C0(3u64, 32u64, 20u64))
                } else {
                    let a193 = v0;
                    let a194 = 6u64;
                    if nat_eq(a193, a194) {
                        Adt3::C1(Adt4::C0(6u64, 32u64, 22u64))
                    } else {
                        let a195 = v0;
                        let a196 = 7u64;
                        if nat_eq(a195, a196) {
                            Adt3::C1(Adt4::C0(7u64, 32u64, 24u64))
                        } else {
                            let a197 = v0;
                            let a198 = 8u64;
                            if nat_eq(a197, a198) {
                                Adt3::C1(Adt4::C0(8u64, 32u64, 34u64))
                            } else {
                                let a199 = v0;
                                let a200 = 9u64;
                                if nat_eq(a199, a200) {
                                    Adt3::C1(Adt4::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a201 = v0;
                                    let a202 = 10u64;
                                    if nat_eq(a201, a202) {
                                        Adt3::C1(Adt4::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a203 = v0;
                                        let a204 = 11u64;
                                        if nat_eq(a203, a204) {
                                            Adt3::C1(Adt4::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a205 = v0;
                                            let a206 = 12u64;
                                            if nat_eq(a205, a206) {
                                                Adt3::C1(Adt4::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a207 = v0;
                                                let a208 = 13u64;
                                                if nat_eq(a207, a208) {
                                                    Adt3::C1(Adt4::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a209 = v0;
                                                    let a210 = 14u64;
                                                    if nat_eq(a209, a210) {
                                                        Adt3::C1(Adt4::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a211 = v0;
                                                        let a212 = 15u64;
                                                        if nat_eq(a211, a212) {
                                                            Adt3::C1(Adt4::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a213 = v0;
                                                            let a214 = 16u64;
                                                            if nat_eq(a213, a214) {
                                                                Adt3::C1(Adt4::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a215 = v0;
                                                                let a216 = 17u64;
                                                                if nat_eq(a215, a216) {
                                                                    Adt3::C1(Adt4::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a217 = v0;
                                                                    let a218 = 18u64;
                                                                    if nat_eq(a217, a218) {
                                                                        Adt3::C1(Adt4::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a219 = v0;
                                                                        let a220 = 19u64;
                                                                        if nat_eq(a219, a220) {
                                                                            Adt3::C1(Adt4::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a221 = v0;
                                                                            let a222 = 20u64;
                                                                            if nat_eq(a221, a222) {
                                                                                Adt3::C1(Adt4::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a223 = v0;
                                                                                let a224 = 21u64;
                                                                                if nat_eq(a223, a224) {
                                                                                    Adt3::C1(Adt4::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a225 = v0;
                                                                                    let a226 = 22u64;
                                                                                    if nat_eq(a225, a226) {
                                                                                        Adt3::C1(Adt4::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a227 = v0;
                                                                                        let a228 = 23u64;
                                                                                        if nat_eq(a227, a228) {
                                                                                            Adt3::C1(Adt4::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a229 = v0;
                                                                                            let a230 = 24u64;
                                                                                            if nat_eq(a229, a230) {
                                                                                                Adt3::C1(Adt4::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a231 = v0;
                                                                                                let a232 = 25u64;
                                                                                                if nat_eq(a231, a232) {
                                                                                                    Adt3::C1(Adt4::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a233 = v0;
                                                                                                    let a234 = 26u64;
                                                                                                    if nat_eq(a233, a234) {
                                                                                                        Adt3::C1(Adt4::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a235 = v0;
                                                                                                        let a236 = 27u64;
                                                                                                        if nat_eq(a235, a236) {
                                                                                                            Adt3::C1(Adt4::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a237 = v0;
                                                                                                            let a238 = 28u64;
                                                                                                            if nat_eq(a237, a238) {
                                                                                                                Adt3::C1(Adt4::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a239 = v0;
                                                                                                                let a240 = 29u64;
                                                                                                                if nat_eq(a239, a240) {
                                                                                                                    Adt3::C1(Adt4::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a241 = v0;
                                                                                                                    let a242 = 30u64;
                                                                                                                    if nat_eq(a241, a242) {
                                                                                                                        Adt3::C1(Adt4::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a243 = v0;
                                                                                                                        let a244 = 34u64;
                                                                                                                        if nat_eq(a243, a244) {
                                                                                                                            Adt3::C1(Adt4::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a245 = v0;
                                                                                                                            let a246 = 35u64;
                                                                                                                            if nat_eq(a245, a246) {
                                                                                                                                Adt3::C1(Adt4::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a247 = v0;
                                                                                                                                let a248 = 39u64;
                                                                                                                                if nat_eq(a247, a248) {
                                                                                                                                    Adt3::C1(Adt4::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a249 = v0;
                                                                                                                                    let a250 = 40u64;
                                                                                                                                    if nat_eq(a249, a250) {
                                                                                                                                        Adt3::C1(Adt4::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a251 = v0;
                                                                                                                                        let a252 = 41u64;
                                                                                                                                        if nat_eq(a251, a252) {
                                                                                                                                            Adt3::C1(Adt4::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a253 = v0;
                                                                                                                                            let a254 = 42u64;
                                                                                                                                            if nat_eq(a253, a254) {
                                                                                                                                                Adt3::C1(Adt4::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a255 = v0;
                                                                                                                                                let a256 = 43u64;
                                                                                                                                                if nat_eq(a255, a256) {
                                                                                                                                                    Adt3::C1(Adt4::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt3::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
