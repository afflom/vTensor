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
            let a16 = v13;
            let a17 = 28u64;
            if nat_eq(a16, a17) {
                let m18 = f7(v11);
                match m18 {
                    Adt2::C0 => {
                        Ok(Adt1::C2)
                    }
                    Adt2::C1(v22, v23, v24, v25) => {
                        Ok(Adt1::C6(v22, v23, v24, v25))
                    }
                }
            } else {
                Ok(Adt1::C2)
            }
        }
        Adt1::C3 => {
            Ok(Adt1::C3)
        }
        Adt1::C4(v30, v31, v32, v33, v34, v35) => {
            Ok(Adt1::C4(v30, v31, v32, v33, v34, v35))
        }
        Adt1::C5(v36, v37, v38, v39, v40, v41) => {
            Ok(Adt1::C5(v36, v37, v38, v39, v40, v41))
        }
        Adt1::C6(v26, v27, v28, v29) => {
            Ok(Adt1::C6(v26, v27, v28, v29))
        }
    }
}

pub fn f1(v0: u64) -> u64 {
    if f8(v0) {
        let m19 = f9(v0);
        match m19 {
            Adt3::C0 => {
                0u64
            }
            Adt3::C1(v1) => {
                let a21 = 1u64;
                let a22 = match v1.clone() {
                    Adt4::C0(_, h20, _) => {
                        h20
                    }
                };
                if if nat_lt(a21, a22) {
                    true
                } else {
                    let a23 = v0;
                    let a24 = 43u64;
                    nat_eq(a23, a24)
                } {
                    match v1 {
                        Adt4::C0(_, h25, _) => {
                            h25
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
    let a26 = v0;
    let a27 = 30u64;
    if nat_eq(a26, a27) {
        Adt0::C0(match v1.clone() {
            Adt0::C0(_, h28, _, _) => {
                h28
            }
        }, match v1.clone() {
            Adt0::C0(h29, _, _, _) => {
                h29
            }
        }, match v1.clone() {
            Adt0::C0(_, _, h30, _) => {
                h30
            }
        }, match v1 {
            Adt0::C0(_, _, _, h31) => {
                h31
            }
        })
    } else {
        v1
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: Adt0, v9: Adt0, v10: Adt0) -> R<Adt1> {
    if f10(v1, v3, v4, v5) {
        let a32 = v0;
        let a33 = 0u64;
        if nat_eq(a32, a33) {
            Ok(Adt1::C0(1u64, 0u64))
        } else {
            Ok(Adt1::C3)
        }
    } else {
        let a34 = 0u64;
        let a35 = v6;
        if if nat_lt(a34, a35) {
            let a36 = 0u64;
            let a37 = v7;
            nat_lt(a36, a37)
        } else {
            false
        } {
            let v11: u64 = {
                let a38 = v1;
                let a39 = v2;
                nat_mul(a38, a39)?
            };
            let v12: u64 = {
                let a40 = v2;
                let a41 = v3;
                nat_mul(a40, a41)?
            };
            let v13: u64 = {
                let a42 = v1;
                let a43 = v3;
                nat_mul(a42, a43)?
            };
            let v14: u64 = {
                let a48 = {
                    let a46 = {
                        let a44 = v11;
                        let a45 = v12;
                        nat_add(a44, a45)?
                    };
                    let a47 = 1u64;
                    nat_add(a46, a47)?
                };
                let a49 = v13;
                nat_add(a48, a49)?
            };
            let v15: u64 = {
                let a54 = 1u64;
                let a55 = {
                    let a52 = {
                        let a50 = v4;
                        let a51 = v5;
                        nat_mul(a50, a51)?
                    };
                    let a53 = v14;
                    nat_mul(a52, a53)?
                };
                nat_add(a54, a55)?
            };
            let a56 = v0;
            let a57 = 0u64;
            if nat_eq(a56, a57) {
                Ok(Adt1::C0(v15, {
                    let a60 = {
                        let a58 = v11;
                        let a59 = v12;
                        nat_add(a58, a59)?
                    };
                    let a61 = v13;
                    nat_add(a60, a61)?
                }))
            } else {
                let a62 = v0;
                let a63 = v15;
                if nat_lt(a62, a63) {
                    let v16: u64 = {
                        let a64 = v0;
                        let a65 = 1u64;
                        nat_sub(a64, a65)
                    };
                    let v17: u64 = {
                        let a66 = v16;
                        let a67 = v14;
                        let a68 = 0u64;
                        nat_quot(a66, a67, a68)
                    };
                    let v18: u64 = {
                        let a69 = v16;
                        let a70 = v14;
                        let a71 = 0u64;
                        nat_rem(a69, a70, a71)
                    };
                    let v19: u64 = {
                        let a72 = v17;
                        let a73 = v4;
                        let a74 = 0u64;
                        nat_rem(a72, a73, a74)
                    };
                    let v20: u64 = {
                        let a75 = v17;
                        let a76 = v4;
                        let a77 = 0u64;
                        nat_quot(a75, a76, a77)
                    };
                    let a78 = v18;
                    let a79 = v11;
                    if nat_lt(a78, a79) {
                        Ok(Adt1::C1(false, 0u64, v18, f11({
                            let a80 = v18;
                            let a81 = v2;
                            let a82 = 0u64;
                            nat_rem(a80, a81, a82)
                        }, {
                            let a83 = v18;
                            let a84 = v2;
                            let a85 = 0u64;
                            nat_quot(a83, a84, a85)
                        }, {
                            let a86 = v19;
                            let a87 = v6;
                            let a88 = 0u64;
                            nat_quot(a86, a87, a88)
                        }, {
                            let a89 = v20;
                            let a90 = v7;
                            let a91 = 0u64;
                            nat_quot(a89, a90, a91)
                        }, v8)?))
                    } else {
                        let a94 = v18;
                        let a95 = {
                            let a92 = v11;
                            let a93 = v12;
                            nat_add(a92, a93)?
                        };
                        if nat_lt(a94, a95) {
                            let v21: u64 = {
                                let a96 = v18;
                                let a97 = v11;
                                nat_sub(a96, a97)
                            };
                            Ok(Adt1::C1(false, 1u64, v18, f11({
                                let a98 = v21;
                                let a99 = v3;
                                let a100 = 0u64;
                                nat_quot(a98, a99, a100)
                            }, {
                                let a101 = v21;
                                let a102 = v3;
                                let a103 = 0u64;
                                nat_rem(a101, a102, a103)
                            }, v19, v20, v9)?))
                        } else {
                            let a106 = v18;
                            let a107 = {
                                let a104 = v11;
                                let a105 = v12;
                                nat_add(a104, a105)?
                            };
                            if nat_eq(a106, a107) {
                                Ok(Adt1::C2)
                            } else {
                                let v22: u64 = {
                                    let a112 = {
                                        let a110 = v18;
                                        let a111 = {
                                            let a108 = v11;
                                            let a109 = v12;
                                            nat_add(a108, a109)?
                                        };
                                        nat_sub(a110, a111)
                                    };
                                    let a113 = 1u64;
                                    nat_sub(a112, a113)
                                };
                                Ok(Adt1::C1(true, 2u64, {
                                    let a116 = {
                                        let a114 = v11;
                                        let a115 = v12;
                                        nat_add(a114, a115)?
                                    };
                                    let a117 = v22;
                                    nat_add(a116, a117)?
                                }, f11({
                                    let a118 = v22;
                                    let a119 = v3;
                                    let a120 = 0u64;
                                    nat_quot(a118, a119, a120)
                                }, {
                                    let a121 = v22;
                                    let a122 = v3;
                                    let a123 = 0u64;
                                    nat_rem(a121, a122, a123)
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
    let m124 = f9(v3);
    match m124 {
        Adt3::C0 => {
            Ok(Adt1::C3)
        }
        Adt3::C1(v6) => {
            let v7: u64 = {
                let a125 = v0;
                let a126 = v2;
                let a127 = 0u64;
                nat_quot(a125, a126, a127)
            };
            let v8: u64 = {
                let a128 = v7;
                let a129 = v4;
                let a130 = 0u64;
                nat_quot(a128, a129, a130)
            };
            let v9: u64 = {
                let a136 = {
                    let a134 = {
                        let a131 = v0;
                        let a132 = v2;
                        let a133 = 0u64;
                        nat_rem(a131, a132, a133)
                    };
                    let a135 = 1u64;
                    nat_add(a134, a135)?
                };
                let a137 = v2;
                if nat_eq(a136, a137) {
                    let a140 = {
                        let a138 = v4;
                        let a139 = 1u64;
                        nat_sub(a138, a139)
                    };
                    let a141 = v2;
                    nat_mul(a140, a141)?
                } else {
                    0u64
                }
            };
            Ok(Adt1::C5(v0, {
                let a144 = v1;
                let a145 = {
                    let a142 = v8;
                    let a143 = v5;
                    nat_mul(a142, a143)?
                };
                nat_add(a144, a145)?
            }, v3, match v6 {
                Adt4::C0(_, _, h146) => {
                    h146
                }
            }, {
                let a147 = v2;
                let a148 = 4u64;
                nat_mul(a147, a148)?
            }, v9))
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt1 {
    let a149 = v3;
    let a150 = 0u64;
    if if nat_eq(a149, a150) {
        let a151 = v4;
        let a152 = 0u64;
        nat_eq(a151, a152)
    } else {
        false
    } {
        Adt1::C1(false, v0, v1, v2)
    } else {
        Adt1::C4(v0, v1, v2, v3, v4, {
            let a153 = v3;
            let a154 = 43u64;
            if nat_eq(a153, a154) {
                1u64
            } else {
                let a155 = v3;
                let a156 = 0u64;
                if nat_eq(a155, a156) {
                    4u64
                } else {
                    2u64
                }
            }
        })
    }
}

pub fn f6(v0: u64) -> u64 {
    let a157 = v0;
    let a158 = 30u64;
    if nat_eq(a157, a158) {
        30u64
    } else {
        0u64
    }
}

pub fn f7(v0: u64) -> Adt2 {
    let a159 = v0;
    let a160 = 0u64;
    if if if if nat_eq(a159, a160) {
        true
    } else {
        let a161 = v0;
        let a162 = 1u64;
        nat_eq(a161, a162)
    } {
        true
    } else {
        let a163 = v0;
        let a164 = 30u64;
        nat_eq(a163, a164)
    } {
        true
    } else {
        let a165 = v0;
        let a166 = 43u64;
        nat_eq(a165, a166)
    } {
        Adt2::C1(2u64, 1u64, 0u64, 2u64)
    } else {
        Adt2::C0
    }
}

pub fn f8(v0: u64) -> bool {
    let a167 = v0;
    let a168 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a167, a168) {
        true
    } else {
        let a169 = v0;
        let a170 = 1u64;
        nat_eq(a169, a170)
    } {
        true
    } else {
        let a171 = v0;
        let a172 = 2u64;
        nat_eq(a171, a172)
    } {
        true
    } else {
        let a173 = v0;
        let a174 = 3u64;
        nat_eq(a173, a174)
    } {
        true
    } else {
        let a175 = v0;
        let a176 = 6u64;
        nat_eq(a175, a176)
    } {
        true
    } else {
        let a177 = v0;
        let a178 = 7u64;
        nat_eq(a177, a178)
    } {
        true
    } else {
        let a179 = v0;
        let a180 = 8u64;
        nat_eq(a179, a180)
    } {
        true
    } else {
        let a181 = v0;
        let a182 = 10u64;
        nat_eq(a181, a182)
    } {
        true
    } else {
        let a183 = v0;
        let a184 = 11u64;
        nat_eq(a183, a184)
    } {
        true
    } else {
        let a185 = v0;
        let a186 = 12u64;
        nat_eq(a185, a186)
    } {
        true
    } else {
        let a187 = v0;
        let a188 = 13u64;
        nat_eq(a187, a188)
    } {
        true
    } else {
        let a189 = v0;
        let a190 = 14u64;
        nat_eq(a189, a190)
    } {
        true
    } else {
        let a191 = v0;
        let a192 = 16u64;
        nat_eq(a191, a192)
    } {
        true
    } else {
        let a193 = v0;
        let a194 = 17u64;
        nat_eq(a193, a194)
    } {
        true
    } else {
        let a195 = v0;
        let a196 = 18u64;
        nat_eq(a195, a196)
    } {
        true
    } else {
        let a197 = v0;
        let a198 = 19u64;
        nat_eq(a197, a198)
    } {
        true
    } else {
        let a199 = v0;
        let a200 = 20u64;
        nat_eq(a199, a200)
    } {
        true
    } else {
        let a201 = v0;
        let a202 = 21u64;
        nat_eq(a201, a202)
    } {
        true
    } else {
        let a203 = v0;
        let a204 = 22u64;
        nat_eq(a203, a204)
    } {
        true
    } else {
        let a205 = v0;
        let a206 = 23u64;
        nat_eq(a205, a206)
    } {
        true
    } else {
        let a207 = v0;
        let a208 = 26u64;
        nat_eq(a207, a208)
    } {
        true
    } else {
        let a209 = v0;
        let a210 = 29u64;
        nat_eq(a209, a210)
    } {
        true
    } else {
        let a211 = v0;
        let a212 = 30u64;
        nat_eq(a211, a212)
    } {
        true
    } else {
        let a213 = v0;
        let a214 = 34u64;
        nat_eq(a213, a214)
    } {
        true
    } else {
        let a215 = v0;
        let a216 = 35u64;
        nat_eq(a215, a216)
    } {
        true
    } else {
        let a217 = v0;
        let a218 = 39u64;
        nat_eq(a217, a218)
    } {
        true
    } else {
        let a219 = v0;
        let a220 = 40u64;
        nat_eq(a219, a220)
    } {
        true
    } else {
        let a221 = v0;
        let a222 = 41u64;
        nat_eq(a221, a222)
    } {
        true
    } else {
        let a223 = v0;
        let a224 = 42u64;
        nat_eq(a223, a224)
    } {
        true
    } else {
        let a225 = v0;
        let a226 = 43u64;
        nat_eq(a225, a226)
    }
}

pub fn f9(v0: u64) -> Adt3 {
    let a227 = v0;
    let a228 = 0u64;
    if nat_eq(a227, a228) {
        Adt3::C1(Adt4::C0(0u64, 1u64, 4u64))
    } else {
        let a229 = v0;
        let a230 = 1u64;
        if nat_eq(a229, a230) {
            Adt3::C1(Adt4::C0(1u64, 1u64, 2u64))
        } else {
            let a231 = v0;
            let a232 = 2u64;
            if nat_eq(a231, a232) {
                Adt3::C1(Adt4::C0(2u64, 32u64, 18u64))
            } else {
                let a233 = v0;
                let a234 = 3u64;
                if nat_eq(a233, a234) {
                    Adt3::C1(Adt4::C0(3u64, 32u64, 20u64))
                } else {
                    let a235 = v0;
                    let a236 = 6u64;
                    if nat_eq(a235, a236) {
                        Adt3::C1(Adt4::C0(6u64, 32u64, 22u64))
                    } else {
                        let a237 = v0;
                        let a238 = 7u64;
                        if nat_eq(a237, a238) {
                            Adt3::C1(Adt4::C0(7u64, 32u64, 24u64))
                        } else {
                            let a239 = v0;
                            let a240 = 8u64;
                            if nat_eq(a239, a240) {
                                Adt3::C1(Adt4::C0(8u64, 32u64, 34u64))
                            } else {
                                let a241 = v0;
                                let a242 = 9u64;
                                if nat_eq(a241, a242) {
                                    Adt3::C1(Adt4::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a243 = v0;
                                    let a244 = 10u64;
                                    if nat_eq(a243, a244) {
                                        Adt3::C1(Adt4::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a245 = v0;
                                        let a246 = 11u64;
                                        if nat_eq(a245, a246) {
                                            Adt3::C1(Adt4::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a247 = v0;
                                            let a248 = 12u64;
                                            if nat_eq(a247, a248) {
                                                Adt3::C1(Adt4::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a249 = v0;
                                                let a250 = 13u64;
                                                if nat_eq(a249, a250) {
                                                    Adt3::C1(Adt4::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a251 = v0;
                                                    let a252 = 14u64;
                                                    if nat_eq(a251, a252) {
                                                        Adt3::C1(Adt4::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a253 = v0;
                                                        let a254 = 15u64;
                                                        if nat_eq(a253, a254) {
                                                            Adt3::C1(Adt4::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a255 = v0;
                                                            let a256 = 16u64;
                                                            if nat_eq(a255, a256) {
                                                                Adt3::C1(Adt4::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a257 = v0;
                                                                let a258 = 17u64;
                                                                if nat_eq(a257, a258) {
                                                                    Adt3::C1(Adt4::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a259 = v0;
                                                                    let a260 = 18u64;
                                                                    if nat_eq(a259, a260) {
                                                                        Adt3::C1(Adt4::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a261 = v0;
                                                                        let a262 = 19u64;
                                                                        if nat_eq(a261, a262) {
                                                                            Adt3::C1(Adt4::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a263 = v0;
                                                                            let a264 = 20u64;
                                                                            if nat_eq(a263, a264) {
                                                                                Adt3::C1(Adt4::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a265 = v0;
                                                                                let a266 = 21u64;
                                                                                if nat_eq(a265, a266) {
                                                                                    Adt3::C1(Adt4::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a267 = v0;
                                                                                    let a268 = 22u64;
                                                                                    if nat_eq(a267, a268) {
                                                                                        Adt3::C1(Adt4::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a269 = v0;
                                                                                        let a270 = 23u64;
                                                                                        if nat_eq(a269, a270) {
                                                                                            Adt3::C1(Adt4::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a271 = v0;
                                                                                            let a272 = 24u64;
                                                                                            if nat_eq(a271, a272) {
                                                                                                Adt3::C1(Adt4::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a273 = v0;
                                                                                                let a274 = 25u64;
                                                                                                if nat_eq(a273, a274) {
                                                                                                    Adt3::C1(Adt4::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a275 = v0;
                                                                                                    let a276 = 26u64;
                                                                                                    if nat_eq(a275, a276) {
                                                                                                        Adt3::C1(Adt4::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a277 = v0;
                                                                                                        let a278 = 27u64;
                                                                                                        if nat_eq(a277, a278) {
                                                                                                            Adt3::C1(Adt4::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a279 = v0;
                                                                                                            let a280 = 28u64;
                                                                                                            if nat_eq(a279, a280) {
                                                                                                                Adt3::C1(Adt4::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a281 = v0;
                                                                                                                let a282 = 29u64;
                                                                                                                if nat_eq(a281, a282) {
                                                                                                                    Adt3::C1(Adt4::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a283 = v0;
                                                                                                                    let a284 = 30u64;
                                                                                                                    if nat_eq(a283, a284) {
                                                                                                                        Adt3::C1(Adt4::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a285 = v0;
                                                                                                                        let a286 = 34u64;
                                                                                                                        if nat_eq(a285, a286) {
                                                                                                                            Adt3::C1(Adt4::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a287 = v0;
                                                                                                                            let a288 = 35u64;
                                                                                                                            if nat_eq(a287, a288) {
                                                                                                                                Adt3::C1(Adt4::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a289 = v0;
                                                                                                                                let a290 = 39u64;
                                                                                                                                if nat_eq(a289, a290) {
                                                                                                                                    Adt3::C1(Adt4::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a291 = v0;
                                                                                                                                    let a292 = 40u64;
                                                                                                                                    if nat_eq(a291, a292) {
                                                                                                                                        Adt3::C1(Adt4::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a293 = v0;
                                                                                                                                        let a294 = 41u64;
                                                                                                                                        if nat_eq(a293, a294) {
                                                                                                                                            Adt3::C1(Adt4::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a295 = v0;
                                                                                                                                            let a296 = 42u64;
                                                                                                                                            if nat_eq(a295, a296) {
                                                                                                                                                Adt3::C1(Adt4::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a297 = v0;
                                                                                                                                                let a298 = 43u64;
                                                                                                                                                if nat_eq(a297, a298) {
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

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a299 = v0;
    let a300 = 0u64;
    if if if nat_eq(a299, a300) {
        true
    } else {
        let a301 = v1;
        let a302 = 0u64;
        nat_eq(a301, a302)
    } {
        true
    } else {
        let a303 = v2;
        let a304 = 0u64;
        nat_eq(a303, a304)
    } {
        true
    } else {
        let a305 = v3;
        let a306 = 0u64;
        nat_eq(a305, a306)
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt0) -> R<u64> {
    let a323 = {
        let a318 = {
            let a313 = {
                let a308 = v0;
                let a309 = match v4.clone() {
                    Adt0::C0(h307, _, _, _) => {
                        h307
                    }
                };
                nat_mul(a308, a309)?
            };
            let a314 = {
                let a311 = v1;
                let a312 = match v4.clone() {
                    Adt0::C0(_, h310, _, _) => {
                        h310
                    }
                };
                nat_mul(a311, a312)?
            };
            nat_add(a313, a314)?
        };
        let a319 = {
            let a316 = v2;
            let a317 = match v4.clone() {
                Adt0::C0(_, _, h315, _) => {
                    h315
                }
            };
            nat_mul(a316, a317)?
        };
        nat_add(a318, a319)?
    };
    let a324 = {
        let a321 = v3;
        let a322 = match v4 {
            Adt0::C0(_, _, _, h320) => {
                h320
            }
        };
        nat_mul(a321, a322)?
    };
    nat_add(a323, a324)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: Adt0, p9: Adt0, p10: Adt0, p11: u64, p12: u64, p13: u64) -> R<Adt1> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
}
