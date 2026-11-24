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
    C0(u64, bool, Adt1, Adt1, Adt1, bool),
    C1(Adt2, u64),
    C2(Adt2, u64, u64),
    C3(u64, bool, Adt1, Adt1, Adt1, Adt1, bool),
    C4(Adt2, u64, u64),
    C5(u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, Option<Adt3>, Option<u64>, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Adt2),
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
    C5(u64),
    C6(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(u64, u64, u64, u64),
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

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt4> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v6, v7, v8, v9, v10, v11) => {
            f2(v6, v7, v8.clone(), v9.clone(), v10, f3(v6, v8, v9, v11))
        }
        Adt0::C1(v12, v13) => {
            let m4 = match v12.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3, _, _) => {
                    h3
                }
            };
            match m4 {
                None => {
                    f4(v12, v13)
                }
                Some(v14) => {
                    let a6 = v13;
                    let a7 = match v12.clone() {
                        Adt2::C0(_, _, _, _, _, h5, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h5
                        }
                    };
                    if nat_lt(a6, a7) {
                        Ok(Adt4::C4({
                            let a22 = {
                                let a13 = {
                                    let a9 = v13;
                                    let a10 = match v12.clone() {
                                        Adt2::C0(_, _, _, h8, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h8
                                        }
                                    };
                                    let a11 = 0u64;
                                    nat_rem(a9, a10, a11)
                                };
                                let a14 = match v14.clone() {
                                    Adt3::C0(_, _, h12, _) => {
                                        h12
                                    }
                                };
                                nat_mul(a13, a14)?
                            };
                            let a23 = {
                                let a20 = {
                                    let a16 = v13;
                                    let a17 = match v12 {
                                        Adt2::C0(_, _, _, h15, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h15
                                        }
                                    };
                                    let a18 = 0u64;
                                    nat_quot(a16, a17, a18)
                                };
                                let a21 = match v14 {
                                    Adt3::C0(_, _, _, h19) => {
                                        h19
                                    }
                                };
                                nat_mul(a20, a21)?
                            };
                            nat_add(a22, a23)?
                        }))
                    } else {
                        Ok(Adt4::C0)
                    }
                }
            }
        }
        Adt0::C2(v25, v26, v27) => {
            let a25 = v27;
            let a26 = match v25.clone() {
                Adt2::C0(_, _, h24, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h24
                }
            };
            if nat_lt(a25, a26) {
                Ok(Adt4::C3({
                    let a30 = v26;
                    let a31 = {
                        let a28 = v27;
                        let a29 = match v25 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h27, _, _, _, _, _, _, _, _, _, _, _) => {
                                h27
                            }
                        };
                        nat_mul(a28, a29)?
                    };
                    nat_add(a30, a31)?
                }))
            } else {
                Ok(Adt4::C0)
            }
        }
        Adt0::C3(v15, v16, v17, v18, v19, v20, v21) => {
            f5(v15, v16, v17.clone(), v18.clone(), v19, v20, f3(v15, v17, v18, v21))
        }
        Adt0::C4(v22, v23, v24) => {
            f6(v22, v23, v24)
        }
        Adt0::C5(v1) => {
            let m2 = f1(v1);
            match m2 {
                Adt5::C0 => {
                    Ok(Adt4::C0)
                }
                Adt5::C1(v2, v3, v4, v5) => {
                    Ok(Adt4::C6(v2, v3, v4, v5))
                }
            }
        }
    }
}

pub fn f1(v0: u64) -> Adt5 {
    let a32 = v0;
    let a33 = 0u64;
    if if if if nat_eq(a32, a33) {
        true
    } else {
        let a34 = v0;
        let a35 = 1u64;
        nat_eq(a34, a35)
    } {
        true
    } else {
        let a36 = v0;
        let a37 = 30u64;
        nat_eq(a36, a37)
    } {
        true
    } else {
        let a38 = v0;
        let a39 = 43u64;
        nat_eq(a38, a39)
    } {
        Adt5::C1(2u64, 1u64, 0u64, 2u64)
    } else {
        Adt5::C0
    }
}

pub fn f2(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: bool) -> R<Adt4> {
    if v1 {
        Ok(Adt4::C0)
    } else {
        let m41 = f7(match v2.clone() {
            Adt1::C0(h40, _, _, _, _, _, _, _, _) => {
                h40
            }
        });
        match m41 {
            Adt6::C0 => {
                Ok(Adt4::C0)
            }
            Adt6::C1(v6) => {
                f8(v0, v2, v3, v4, match v6.clone() {
                    Adt7::C0(_, h42, _) => {
                        h42
                    }
                }, match v6 {
                    Adt7::C0(_, _, h43) => {
                        h43
                    }
                }, v5)
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: bool) -> bool {
    let a45 = match v2.clone() {
        Adt1::C0(h44, _, _, _, _, _, _, _, _) => {
            h44
        }
    };
    let a46 = 30u64;
    if if nat_eq(a45, a46) {
        let a47 = v0;
        let a48 = 28u64;
        if if nat_eq(a47, a48) {
            let a50 = match v1.clone() {
                Adt1::C0(h49, _, _, _, _, _, _, _, _) => {
                    h49
                }
            };
            let a51 = 1u64;
            nat_eq(a50, a51)
        } else {
            false
        } {
            true
        } else {
            let a52 = v0;
            let a53 = 29u64;
            if nat_eq(a52, a53) {
                let a55 = match v1.clone() {
                    Adt1::C0(h54, _, _, _, _, _, _, _, _) => {
                        h54
                    }
                };
                let a56 = 30u64;
                nat_eq(a55, a56)
            } else {
                false
            }
        }
    } else {
        false
    } {
        true
    } else {
        if v3 {
            !f9(match v1 {
                Adt1::C0(h57, _, _, _, _, _, _, _, _) => {
                    h57
                }
            }, match v2.clone() {
                Adt1::C0(h58, _, _, _, _, _, _, _, _) => {
                    h58
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, _, h59, _, _, _) => {
                    h59
                }
            })
        } else {
            false
        }
    }
}

pub fn f4(v0: Adt2, v1: u64) -> R<Adt4> {
    let a61 = v1;
    let a62 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h60, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h60
        }
    };
    if nat_lt(a61, a62) {
        let v2: u64 = {
            let a64 = v1;
            let a65 = match v0.clone() {
                Adt2::C0(_, _, _, h63, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h63
                }
            };
            let a66 = 0u64;
            nat_rem(a64, a65, a66)
        };
        let v3: u64 = {
            let a68 = v1;
            let a69 = match v0.clone() {
                Adt2::C0(_, _, _, h67, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h67
                }
            };
            let a70 = 0u64;
            nat_quot(a68, a69, a70)
        };
        let v4: u64 = {
            let a72 = v3;
            let a73 = match v0.clone() {
                Adt2::C0(_, _, _, _, h71, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h71
                }
            };
            let a74 = 0u64;
            nat_rem(a72, a73, a74)
        };
        let v5: u64 = {
            let a76 = v3;
            let a77 = match v0.clone() {
                Adt2::C0(_, _, _, _, h75, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            };
            let a78 = 0u64;
            nat_quot(a76, a77, a78)
        };
        Ok(Adt4::C2({
            let a93 = {
                let a84 = {
                    let a80 = v4;
                    let a81 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h79, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h79
                        }
                    };
                    let a82 = 0u64;
                    nat_quot(a80, a81, a82)
                };
                let a85 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h83, _, _, _, _, _, _, _, _, _, _) => {
                        h83
                    }
                };
                nat_mul(a84, a85)?
            };
            let a94 = {
                let a91 = {
                    let a87 = v5;
                    let a88 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h86, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h86
                        }
                    };
                    let a89 = 0u64;
                    nat_quot(a87, a88, a89)
                };
                let a92 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h90, _, _, _, _, _, _, _, _, _) => {
                        h90
                    }
                };
                nat_mul(a91, a92)?
            };
            nat_add(a93, a94)?
        }, {
            let a106 = {
                let a101 = {
                    let a96 = v2;
                    let a97 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h95, _, _, _, _, _, _, _, _) => {
                            h95
                        }
                    };
                    nat_mul(a96, a97)?
                };
                let a102 = {
                    let a99 = v4;
                    let a100 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h98, _, _, _, _, _, _, _) => {
                            h98
                        }
                    };
                    nat_mul(a99, a100)?
                };
                nat_add(a101, a102)?
            };
            let a107 = {
                let a104 = v5;
                let a105 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h103, _, _, _, _, _, _) => {
                        h103
                    }
                };
                nat_mul(a104, a105)?
            };
            nat_add(a106, a107)?
        }, {
            let a119 = {
                let a114 = {
                    let a109 = v2;
                    let a110 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h108, _, _, _, _, _) => {
                            h108
                        }
                    };
                    nat_mul(a109, a110)?
                };
                let a115 = {
                    let a112 = v4;
                    let a113 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h111, _, _, _, _) => {
                            h111
                        }
                    };
                    nat_mul(a112, a113)?
                };
                nat_add(a114, a115)?
            };
            let a120 = {
                let a117 = v5;
                let a118 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h116, _, _, _) => {
                        h116
                    }
                };
                nat_mul(a117, a118)?
            };
            nat_add(a119, a120)?
        }))
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f5(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: bool) -> R<Adt4> {
    let a121 = v0;
    let a122 = 29u64;
    if if if if if if if if if if if if if if if if if if if if nat_eq(a121, a122) {
        !v1
    } else {
        false
    } {
        let a124 = match v4.clone() {
            Adt1::C0(h123, _, _, _, _, _, _, _, _) => {
                h123
            }
        };
        let a125 = 26u64;
        nat_eq(a124, a125)
    } else {
        false
    } {
        let a127 = match v3.clone() {
            Adt1::C0(h126, _, _, _, _, _, _, _, _) => {
                h126
            }
        };
        let a128 = 0u64;
        if if nat_eq(a127, a128) {
            true
        } else {
            if v6 {
                f10(match v3.clone() {
                    Adt1::C0(h129, _, _, _, _, _, _, _, _) => {
                        h129
                    }
                })
            } else {
                false
            }
        } {
            true
        } else {
            f9(match v2.clone() {
                Adt1::C0(h130, _, _, _, _, _, _, _, _) => {
                    h130
                }
            }, match v3.clone() {
                Adt1::C0(h131, _, _, _, _, _, _, _, _) => {
                    h131
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h132, _, _, _) => {
                    h132
                }
            })
        }
    } else {
        false
    } {
        let a134 = match v5.clone() {
            Adt1::C0(h133, _, _, _, _, _, _, _, _) => {
                h133
            }
        };
        let a135 = 0u64;
        nat_eq(a134, a135)
    } else {
        false
    } {
        let a137 = match v2.clone() {
            Adt1::C0(_, _, _, _, h136, _, _, _, _) => {
                h136
            }
        };
        let a138 = 1u64;
        nat_eq(a137, a138)
    } else {
        false
    } {
        let a140 = match v3.clone() {
            Adt1::C0(_, _, _, _, h139, _, _, _, _) => {
                h139
            }
        };
        let a141 = 1u64;
        nat_eq(a140, a141)
    } else {
        false
    } {
        let a143 = match v4.clone() {
            Adt1::C0(_, _, _, _, h142, _, _, _, _) => {
                h142
            }
        };
        let a144 = 1u64;
        nat_eq(a143, a144)
    } else {
        false
    } {
        let a146 = match v5.clone() {
            Adt1::C0(_, _, _, _, h145, _, _, _, _) => {
                h145
            }
        };
        let a147 = 1u64;
        nat_eq(a146, a147)
    } else {
        false
    } {
        let a149 = match v4.clone() {
            Adt1::C0(_, _, _, h148, _, _, _, _, _) => {
                h148
            }
        };
        let a150 = 1u64;
        nat_eq(a149, a150)
    } else {
        false
    } {
        let a153 = match v4.clone() {
            Adt1::C0(_, _, h151, _, _, _, _, _, _) => {
                h151
            }
        };
        let a154 = match v3.clone() {
            Adt1::C0(_, _, _, h152, _, _, _, _, _) => {
                h152
            }
        };
        nat_eq(a153, a154)
    } else {
        false
    } {
        let a157 = match v2.clone() {
            Adt1::C0(_, h155, _, _, _, _, _, _, _) => {
                h155
            }
        };
        let a158 = match v3.clone() {
            Adt1::C0(_, h156, _, _, _, _, _, _, _) => {
                h156
            }
        };
        nat_eq(a157, a158)
    } else {
        false
    } {
        let a160 = 0u64;
        let a161 = match v3.clone() {
            Adt1::C0(_, _, h159, _, _, _, _, _, _) => {
                h159
            }
        };
        nat_lt(a160, a161)
    } else {
        false
    } {
        let a167 = {
            let a164 = match v4.clone() {
                Adt1::C0(_, h162, _, _, _, _, _, _, _) => {
                    h162
                }
            };
            let a165 = match v3.clone() {
                Adt1::C0(_, _, h163, _, _, _, _, _, _) => {
                    h163
                }
            };
            let a166 = 0u64;
            nat_rem(a164, a165, a166)
        };
        let a168 = 0u64;
        nat_eq(a167, a168)
    } else {
        false
    } {
        let a171 = match v5.clone() {
            Adt1::C0(_, h169, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a172 = match v2.clone() {
            Adt1::C0(_, _, h170, _, _, _, _, _, _) => {
                h170
            }
        };
        nat_eq(a171, a172)
    } else {
        false
    } {
        let a175 = match v5.clone() {
            Adt1::C0(_, _, h173, _, _, _, _, _, _) => {
                h173
            }
        };
        let a176 = match v4.clone() {
            Adt1::C0(_, h174, _, _, _, _, _, _, _) => {
                h174
            }
        };
        nat_eq(a175, a176)
    } else {
        false
    } {
        let a179 = match v5.clone() {
            Adt1::C0(_, _, _, h177, _, _, _, _, _) => {
                h177
            }
        };
        let a180 = match v3.clone() {
            Adt1::C0(_, _, _, h178, _, _, _, _, _) => {
                h178
            }
        };
        nat_eq(a179, a180)
    } else {
        false
    } {
        let a183 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h181, _, _, _) => {
                h181
            }
        };
        let a184 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h182, _, _) => {
                h182
            }
        };
        nat_le(a183, a184)
    } else {
        false
    } {
        let a189 = {
            let a186 = match v5.clone() {
                Adt1::C0(_, h185, _, _, _, _, _, _, _) => {
                    h185
                }
            };
            let a187 = 4u64;
            nat_mul(a186, a187)?
        };
        let a190 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, h188, _, _) => {
                h188
            }
        };
        nat_le(a189, a190)
    } else {
        false
    } {
        let a196 = {
            let a193 = match v5.clone() {
                Adt1::C0(_, _, h191, _, _, _, _, _, _) => {
                    h191
                }
            };
            let a194 = match v5.clone() {
                Adt1::C0(_, _, _, _, _, _, h192, _, _) => {
                    h192
                }
            };
            nat_mul(a193, a194)?
        };
        let a197 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h195, _) => {
                h195
            }
        };
        nat_le(a196, a197)
    } else {
        false
    } {
        let m199 = f7(match v2.clone() {
            Adt1::C0(h198, _, _, _, _, _, _, _, _) => {
                h198
            }
        });
        match m199 {
            Adt6::C0 => {
                Ok(Adt4::C0)
            }
            Adt6::C1(v7) => {
                if f11(v2.clone(), v3.clone(), v5.clone(), true, v6, match v7.clone() {
                    Adt7::C0(_, h200, _) => {
                        h200
                    }
                }, match v7.clone() {
                    Adt7::C0(_, _, h201) => {
                        h201
                    }
                }) {
                    Ok(Adt4::C1(f12(v2.clone(), v3.clone(), v5, match v7.clone() {
                        Adt7::C0(_, h202, _) => {
                            h202
                        }
                    }, match v7 {
                        Adt7::C0(_, _, h203) => {
                            h203
                        }
                    }, match v2.clone() {
                        Adt1::C0(_, _, h204, _, _, _, _, _, _) => {
                            h204
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, h205, _, _, _, _, _, _, _) => {
                            h205
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h206, _, _, _, _, _, _) => {
                            h206
                        }
                    }, 1u64, 1u64, 1u64, Some(Adt3::C0(match v2 {
                        Adt1::C0(_, _, _, h207, _, _, _, _, _) => {
                            h207
                        }
                    }, match v3 {
                        Adt1::C0(_, _, h208, _, _, _, _, _, _) => {
                            h208
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, _, h209, _, _, _) => {
                            h209
                        }
                    }, match v4 {
                        Adt1::C0(_, _, _, _, _, _, h210, _, _) => {
                            h210
                        }
                    })), v6)?))
                } else {
                    Ok(Adt4::C0)
                }
            }
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f6(v0: Adt2, v1: u64, v2: u64) -> R<Adt4> {
    let m212 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h211, _, _) => {
            h211
        }
    };
    match m212 {
        None => {
            Ok(Adt4::C0)
        }
        Some(v3) => {
            let a214 = v1;
            let a215 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h213, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h213
                }
            };
            if nat_lt(a214, a215) {
                let v4: u64 = {
                    let a217 = v1;
                    let a218 = match v0.clone() {
                        Adt2::C0(_, _, _, h216, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h216
                        }
                    };
                    let a219 = 0u64;
                    nat_rem(a217, a218, a219)
                };
                let v5: u64 = {
                    let a221 = v1;
                    let a222 = match v0.clone() {
                        Adt2::C0(_, _, _, h220, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h220
                        }
                    };
                    let a223 = 0u64;
                    nat_quot(a221, a222, a223)
                };
                let v6: u64 = {
                    let a230 = {
                        let a225 = v4;
                        let a226 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h224, _, _, _, _, _) => {
                                h224
                            }
                        };
                        nat_mul(a225, a226)?
                    };
                    let a231 = {
                        let a228 = v5;
                        let a229 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h227, _, _, _, _) => {
                                h227
                            }
                        };
                        nat_mul(a228, a229)?
                    };
                    nat_add(a230, a231)?
                };
                let a232 = v2;
                let a233 = 4294967295u64;
                if nat_eq(a232, a233) {
                    Ok(Adt4::C5(v6))
                } else {
                    let a235 = v2;
                    let a236 = match v3.clone() {
                        Adt3::C0(h234, _, _, _) => {
                            h234
                        }
                    };
                    if nat_lt(a235, a236) {
                        Ok(Adt4::C2({
                            let a238 = v2;
                            let a239 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h237, _, _, _, _, _, _, _, _, _, _) => {
                                    h237
                                }
                            };
                            nat_mul(a238, a239)?
                        }, {
                            let a250 = {
                                let a245 = {
                                    let a241 = v4;
                                    let a242 = match v3 {
                                        Adt3::C0(_, h240, _, _) => {
                                            h240
                                        }
                                    };
                                    let a243 = 0u64;
                                    nat_rem(a241, a242, a243)
                                };
                                let a246 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h244, _, _, _, _, _, _, _, _) => {
                                        h244
                                    }
                                };
                                nat_mul(a245, a246)?
                            };
                            let a251 = {
                                let a248 = v5;
                                let a249 = match v0 {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h247, _, _, _, _, _, _, _) => {
                                        h247
                                    }
                                };
                                nat_mul(a248, a249)?
                            };
                            nat_add(a250, a251)?
                        }, v6))
                    } else {
                        Ok(Adt4::C0)
                    }
                }
            } else {
                Ok(Adt4::C0)
            }
        }
    }
}

pub fn f7(v0: u64) -> Adt6 {
    let a252 = v0;
    let a253 = 0u64;
    if nat_eq(a252, a253) {
        Adt6::C1(Adt7::C0(0u64, 1u64, 4u64))
    } else {
        let a254 = v0;
        let a255 = 1u64;
        if nat_eq(a254, a255) {
            Adt6::C1(Adt7::C0(1u64, 1u64, 2u64))
        } else {
            let a256 = v0;
            let a257 = 2u64;
            if nat_eq(a256, a257) {
                Adt6::C1(Adt7::C0(2u64, 32u64, 18u64))
            } else {
                let a258 = v0;
                let a259 = 3u64;
                if nat_eq(a258, a259) {
                    Adt6::C1(Adt7::C0(3u64, 32u64, 20u64))
                } else {
                    let a260 = v0;
                    let a261 = 6u64;
                    if nat_eq(a260, a261) {
                        Adt6::C1(Adt7::C0(6u64, 32u64, 22u64))
                    } else {
                        let a262 = v0;
                        let a263 = 7u64;
                        if nat_eq(a262, a263) {
                            Adt6::C1(Adt7::C0(7u64, 32u64, 24u64))
                        } else {
                            let a264 = v0;
                            let a265 = 8u64;
                            if nat_eq(a264, a265) {
                                Adt6::C1(Adt7::C0(8u64, 32u64, 34u64))
                            } else {
                                let a266 = v0;
                                let a267 = 9u64;
                                if nat_eq(a266, a267) {
                                    Adt6::C1(Adt7::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a268 = v0;
                                    let a269 = 10u64;
                                    if nat_eq(a268, a269) {
                                        Adt6::C1(Adt7::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a270 = v0;
                                        let a271 = 11u64;
                                        if nat_eq(a270, a271) {
                                            Adt6::C1(Adt7::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a272 = v0;
                                            let a273 = 12u64;
                                            if nat_eq(a272, a273) {
                                                Adt6::C1(Adt7::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a274 = v0;
                                                let a275 = 13u64;
                                                if nat_eq(a274, a275) {
                                                    Adt6::C1(Adt7::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a276 = v0;
                                                    let a277 = 14u64;
                                                    if nat_eq(a276, a277) {
                                                        Adt6::C1(Adt7::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a278 = v0;
                                                        let a279 = 15u64;
                                                        if nat_eq(a278, a279) {
                                                            Adt6::C1(Adt7::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a280 = v0;
                                                            let a281 = 16u64;
                                                            if nat_eq(a280, a281) {
                                                                Adt6::C1(Adt7::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a282 = v0;
                                                                let a283 = 17u64;
                                                                if nat_eq(a282, a283) {
                                                                    Adt6::C1(Adt7::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a284 = v0;
                                                                    let a285 = 18u64;
                                                                    if nat_eq(a284, a285) {
                                                                        Adt6::C1(Adt7::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a286 = v0;
                                                                        let a287 = 19u64;
                                                                        if nat_eq(a286, a287) {
                                                                            Adt6::C1(Adt7::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a288 = v0;
                                                                            let a289 = 20u64;
                                                                            if nat_eq(a288, a289) {
                                                                                Adt6::C1(Adt7::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a290 = v0;
                                                                                let a291 = 21u64;
                                                                                if nat_eq(a290, a291) {
                                                                                    Adt6::C1(Adt7::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a292 = v0;
                                                                                    let a293 = 22u64;
                                                                                    if nat_eq(a292, a293) {
                                                                                        Adt6::C1(Adt7::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a294 = v0;
                                                                                        let a295 = 23u64;
                                                                                        if nat_eq(a294, a295) {
                                                                                            Adt6::C1(Adt7::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a296 = v0;
                                                                                            let a297 = 24u64;
                                                                                            if nat_eq(a296, a297) {
                                                                                                Adt6::C1(Adt7::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a298 = v0;
                                                                                                let a299 = 25u64;
                                                                                                if nat_eq(a298, a299) {
                                                                                                    Adt6::C1(Adt7::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a300 = v0;
                                                                                                    let a301 = 26u64;
                                                                                                    if nat_eq(a300, a301) {
                                                                                                        Adt6::C1(Adt7::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a302 = v0;
                                                                                                        let a303 = 27u64;
                                                                                                        if nat_eq(a302, a303) {
                                                                                                            Adt6::C1(Adt7::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a304 = v0;
                                                                                                            let a305 = 28u64;
                                                                                                            if nat_eq(a304, a305) {
                                                                                                                Adt6::C1(Adt7::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a306 = v0;
                                                                                                                let a307 = 29u64;
                                                                                                                if nat_eq(a306, a307) {
                                                                                                                    Adt6::C1(Adt7::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a308 = v0;
                                                                                                                    let a309 = 30u64;
                                                                                                                    if nat_eq(a308, a309) {
                                                                                                                        Adt6::C1(Adt7::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a310 = v0;
                                                                                                                        let a311 = 34u64;
                                                                                                                        if nat_eq(a310, a311) {
                                                                                                                            Adt6::C1(Adt7::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a312 = v0;
                                                                                                                            let a313 = 35u64;
                                                                                                                            if nat_eq(a312, a313) {
                                                                                                                                Adt6::C1(Adt7::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a314 = v0;
                                                                                                                                let a315 = 39u64;
                                                                                                                                if nat_eq(a314, a315) {
                                                                                                                                    Adt6::C1(Adt7::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a316 = v0;
                                                                                                                                    let a317 = 40u64;
                                                                                                                                    if nat_eq(a316, a317) {
                                                                                                                                        Adt6::C1(Adt7::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a318 = v0;
                                                                                                                                        let a319 = 41u64;
                                                                                                                                        if nat_eq(a318, a319) {
                                                                                                                                            Adt6::C1(Adt7::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a320 = v0;
                                                                                                                                            let a321 = 42u64;
                                                                                                                                            if nat_eq(a320, a321) {
                                                                                                                                                Adt6::C1(Adt7::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a322 = v0;
                                                                                                                                                let a323 = 43u64;
                                                                                                                                                if nat_eq(a322, a323) {
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

pub fn f8(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64, v5: u64, v6: bool) -> R<Adt4> {
    if f11(v1.clone(), v2.clone(), v3.clone(), v6, v6, v4, v5) {
        let m337 = f13(v0, 0u64, 0u64, match v3.clone() {
            Adt1::C0(h324, _, _, _, _, _, _, _, _) => {
                h324
            }
        }, match v1.clone() {
            Adt1::C0(_, h325, _, _, _, _, _, _, _) => {
                h325
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h326, _, _, _, _, _, _) => {
                h326
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h327, _, _, _, _, _) => {
                h327
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h328, _, _, _, _) => {
                h328
            }
        }, match v2.clone() {
            Adt1::C0(_, h329, _, _, _, _, _, _, _) => {
                h329
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h330, _, _, _, _, _, _) => {
                h330
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h331, _, _, _, _, _) => {
                h331
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h332, _, _, _, _) => {
                h332
            }
        }, match v3.clone() {
            Adt1::C0(_, h333, _, _, _, _, _, _, _) => {
                h333
            }
        }, match v3.clone() {
            Adt1::C0(_, _, h334, _, _, _, _, _, _) => {
                h334
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h335, _, _, _, _, _) => {
                h335
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h336, _, _, _, _) => {
                h336
            }
        })?;
        match m337 {
            Adt8::C0 => {
                Ok(Adt4::C0)
            }
            Adt8::C1 => {
                Ok(Adt4::C0)
            }
            Adt8::C2(v7, _, v9, v10, v11, v12, v13, _, _, _) => {
                Ok(Adt4::C1(f12(v1, v2, v3, v4, v5, v7, v9, v10, v11, v12, v13, None::<Adt3>, v6)?))
            }
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64) -> bool {
    if f14(v0) {
        let a338 = v1;
        let a339 = 15u64;
        if nat_eq(a338, a339) {
            let a340 = v2;
            let a341 = 292u64;
            nat_eq(a340, a341)
        } else {
            false
        }
    } else {
        if f15(v0) {
            let a342 = v1;
            let a343 = 9u64;
            if nat_eq(a342, a343) {
                let a344 = v2;
                let a345 = 36u64;
                nat_eq(a344, a345)
            } else {
                false
            }
        } else {
            if if f16(v0) {
                true
            } else {
                f17(v0)
            } {
                let a346 = v1;
                let a347 = 8u64;
                if nat_eq(a346, a347) {
                    let a348 = v2;
                    let a349 = 34u64;
                    nat_eq(a348, a349)
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f10(v0: u64) -> bool {
    let a350 = v0;
    let a351 = 0u64;
    if if nat_eq(a350, a351) {
        true
    } else {
        let a352 = v0;
        let a353 = 1u64;
        nat_eq(a352, a353)
    } {
        true
    } else {
        let a354 = v0;
        let a355 = 30u64;
        nat_eq(a354, a355)
    }
}

pub fn f11(v0: Adt1, v1: Adt1, v2: Adt1, v3: bool, v4: bool, v5: u64, v6: u64) -> bool {
    if if if if if if if if f18(match v0.clone() {
        Adt1::C0(h356, _, _, _, _, _, _, _, _) => {
            h356
        }
    }) {
        true
    } else {
        if v3 {
            if f10(match v0.clone() {
                Adt1::C0(h357, _, _, _, _, _, _, _, _) => {
                    h357
                }
            }) {
                true
            } else {
                let a359 = match v0.clone() {
                    Adt1::C0(h358, _, _, _, _, _, _, _, _) => {
                        h358
                    }
                };
                let a360 = 43u64;
                nat_eq(a359, a360)
            }
        } else {
            false
        }
    } {
        let a365 = {
            let a362 = match v0.clone() {
                Adt1::C0(_, h361, _, _, _, _, _, _, _) => {
                    h361
                }
            };
            let a363 = v5;
            let a364 = 0u64;
            nat_rem(a362, a363, a364)
        };
        let a366 = 0u64;
        nat_eq(a365, a366)
    } else {
        false
    } {
        let a368 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h367, _, _, _) => {
                h367
            }
        };
        let a369 = v6;
        nat_eq(a368, a369)
    } else {
        false
    } {
        if f19(match v1.clone() {
            Adt1::C0(h370, _, _, _, _, _, _, _, _) => {
                h370
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, h371, _, _, _) => {
                h371
            }
        }, v4) {
            true
        } else {
            f9(match v0 {
                Adt1::C0(h372, _, _, _, _, _, _, _, _) => {
                    h372
                }
            }, match v1.clone() {
                Adt1::C0(h373, _, _, _, _, _, _, _, _) => {
                    h373
                }
            }, match v1 {
                Adt1::C0(_, _, _, _, _, h374, _, _, _) => {
                    h374
                }
            })
        }
    } else {
        false
    } {
        let a376 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h375, _, _, _) => {
                h375
            }
        };
        let a377 = 4u64;
        nat_eq(a376, a377)
    } else {
        false
    } {
        let a380 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h378, _, _, _) => {
                h378
            }
        };
        let a381 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h379, _, _) => {
                h379
            }
        };
        nat_le(a380, a381)
    } else {
        false
    } {
        let a384 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h382, _, _) => {
                h382
            }
        };
        let a385 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h383, _) => {
                h383
            }
        };
        nat_le(a384, a385)
    } else {
        false
    } {
        let a388 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h386, _) => {
                h386
            }
        };
        let a389 = match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h387) => {
                h387
            }
        };
        nat_le(a388, a389)
    } else {
        false
    }
}

pub fn f12(v0: Adt1, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: Option<Adt3>, v12: bool) -> R<Adt2> {
    Ok(Adt2::C0(if v12 {
        match v0.clone() {
            Adt1::C0(_, h390, _, _, _, _, _, _, _) => {
                h390
            }
        }
    } else {
        let a393 = match v0.clone() {
            Adt1::C0(_, h391, _, _, _, _, _, _, _) => {
                h391
            }
        };
        let a394 = f20(match v0.clone() {
            Adt1::C0(h392, _, _, _, _, _, _, _, _) => {
                h392
            }
        }, v3);
        let a395 = 0u64;
        nat_quot(a393, a394, a395)
    }, {
        let a400 = {
            let a397 = match v0.clone() {
                Adt1::C0(_, h396, _, _, _, _, _, _, _) => {
                    h396
                }
            };
            let a398 = v3;
            let a399 = 0u64;
            nat_quot(a397, a398, a399)
        };
        let a401 = v4;
        nat_mul(a400, a401)?
    }, v5, v6, v7, {
        let a404 = v6;
        let a405 = {
            let a402 = v7;
            let a403 = v8;
            nat_mul(a402, a403)?
        };
        nat_mul(a404, a405)?
    }, {
        let a410 = v5;
        let a411 = {
            let a408 = v6;
            let a409 = {
                let a406 = v7;
                let a407 = v8;
                nat_mul(a406, a407)?
            };
            nat_mul(a408, a409)?
        };
        nat_mul(a410, a411)?
    }, v9, v10, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h412, _, _) => {
            h412
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h413, _) => {
            h413
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h414) => {
            h414
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h415, _, _) => {
            h415
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h416, _) => {
            h416
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h417) => {
            h417
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h418, _, _) => {
            h418
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h419, _) => {
            h419
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h420) => {
            h420
        }
    }, v11, if v12 {
        Some(0u64)
    } else {
        if if f10(match v0.clone() {
            Adt1::C0(h421, _, _, _, _, _, _, _, _) => {
                h421
            }
        }) {
            true
        } else {
            let a423 = match v0.clone() {
                Adt1::C0(h422, _, _, _, _, _, _, _, _) => {
                    h422
                }
            };
            let a424 = 43u64;
            nat_eq(a423, a424)
        } {
            Some(f21(match v0.clone() {
                Adt1::C0(h425, _, _, _, _, _, _, _, _) => {
                    h425
                }
            }))
        } else {
            None::<u64>
        }
    }, match v0 {
        Adt1::C0(h426, _, _, _, _, _, _, _, _) => {
            h426
        }
    }))
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt8> {
    let a427 = v0;
    let a428 = 28u64;
    if if if if nat_eq(a427, a428) {
        let a429 = v1;
        let a430 = 0u64;
        nat_eq(a429, a430)
    } else {
        false
    } {
        let a431 = v2;
        let a432 = 0u64;
        nat_eq(a431, a432)
    } else {
        false
    } {
        let a433 = v3;
        let a434 = 0u64;
        nat_eq(a433, a434)
    } else {
        false
    } {
        let a435 = v4;
        let a436 = v8;
        if if if if if if if if nat_eq(a435, a436) {
            let a437 = v5;
            let a438 = v12;
            nat_eq(a437, a438)
        } else {
            false
        } {
            let a439 = v9;
            let a440 = v13;
            nat_eq(a439, a440)
        } else {
            false
        } {
            let a441 = v10;
            let a442 = v14;
            nat_eq(a441, a442)
        } else {
            false
        } {
            let a443 = v11;
            let a444 = v15;
            nat_eq(a443, a444)
        } else {
            false
        } {
            let a445 = 0u64;
            let a446 = v6;
            nat_lt(a445, a446)
        } else {
            false
        } {
            let a447 = 0u64;
            let a448 = v7;
            nat_lt(a447, a448)
        } else {
            false
        } {
            let a452 = {
                let a449 = v10;
                let a450 = v6;
                let a451 = 1u64;
                nat_rem(a449, a450, a451)
            };
            let a453 = 0u64;
            if nat_eq(a452, a453) {
                let a457 = {
                    let a454 = v11;
                    let a455 = v7;
                    let a456 = 1u64;
                    nat_rem(a454, a455, a456)
                };
                let a458 = 0u64;
                nat_eq(a457, a458)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt8::C2(v5, v4, v9, v10, v11, {
                let a459 = v10;
                let a460 = v6;
                let a461 = 0u64;
                nat_quot(a459, a460, a461)
            }, {
                let a462 = v11;
                let a463 = v7;
                let a464 = 0u64;
                nat_quot(a462, a463, a464)
            }, {
                let a465 = v5;
                let a466 = v4;
                nat_mul(a465, a466)?
            }, {
                let a467 = v4;
                let a468 = v9;
                nat_mul(a467, a468)?
            }, {
                let a469 = v5;
                let a470 = v9;
                nat_mul(a469, a470)?
            }))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f14(v0: u64) -> bool {
    let a471 = 10u64;
    let a472 = v0;
    if if if if if nat_le(a471, a472) {
        let a473 = v0;
        let a474 = 14u64;
        nat_le(a473, a474)
    } else {
        false
    } {
        true
    } else {
        let a475 = 16u64;
        let a476 = v0;
        if nat_le(a475, a476) {
            let a477 = v0;
            let a478 = 19u64;
            nat_le(a477, a478)
        } else {
            false
        }
    } {
        true
    } else {
        let a479 = 21u64;
        let a480 = v0;
        if nat_le(a479, a480) {
            let a481 = v0;
            let a482 = 23u64;
            nat_le(a481, a482)
        } else {
            false
        }
    } {
        true
    } else {
        let a483 = 34u64;
        let a484 = v0;
        if nat_le(a483, a484) {
            let a485 = v0;
            let a486 = 35u64;
            nat_le(a485, a486)
        } else {
            false
        }
    } {
        true
    } else {
        let a487 = v0;
        let a488 = 29u64;
        nat_eq(a487, a488)
    }
}

pub fn f15(v0: u64) -> bool {
    let a489 = v0;
    let a490 = 3u64;
    if nat_eq(a489, a490) {
        true
    } else {
        let a491 = v0;
        let a492 = 7u64;
        nat_eq(a491, a492)
    }
}

pub fn f16(v0: u64) -> bool {
    let a493 = v0;
    let a494 = 2u64;
    if if if if nat_eq(a493, a494) {
        true
    } else {
        let a495 = v0;
        let a496 = 6u64;
        nat_eq(a495, a496)
    } {
        true
    } else {
        let a497 = v0;
        let a498 = 8u64;
        nat_eq(a497, a498)
    } {
        true
    } else {
        let a499 = v0;
        let a500 = 20u64;
        nat_eq(a499, a500)
    } {
        true
    } else {
        f22(v0)
    }
}

pub fn f17(v0: u64) -> bool {
    let a501 = v0;
    let a502 = 39u64;
    if nat_eq(a501, a502) {
        true
    } else {
        let a503 = v0;
        let a504 = 40u64;
        nat_eq(a503, a504)
    }
}

pub fn f18(v0: u64) -> bool {
    if if f23(v0) {
        true
    } else {
        f17(v0)
    } {
        true
    } else {
        f14(v0)
    }
}

pub fn f19(v0: u64, v1: u64, v2: bool) -> bool {
    let a505 = v0;
    let a506 = 0u64;
    if if nat_eq(a505, a506) {
        let a507 = v1;
        let a508 = 4u64;
        nat_eq(a507, a508)
    } else {
        false
    } {
        true
    } else {
        let a509 = v0;
        let a510 = 1u64;
        if if nat_eq(a509, a510) {
            true
        } else {
            if v2 {
                let a511 = v0;
                let a512 = 30u64;
                nat_eq(a511, a512)
            } else {
                false
            }
        } {
            let a513 = v1;
            let a514 = 2u64;
            nat_eq(a513, a514)
        } else {
            false
        }
    }
}

pub fn f20(v0: u64, v1: u64) -> u64 {
    if if f17(v0) {
        true
    } else {
        f22(v0)
    } {
        32u64
    } else {
        v1
    }
}

pub fn f21(v0: u64) -> u64 {
    let a515 = v0;
    let a516 = 30u64;
    if nat_eq(a515, a516) {
        30u64
    } else {
        0u64
    }
}

pub fn f22(v0: u64) -> bool {
    let a517 = v0;
    let a518 = 41u64;
    if nat_eq(a517, a518) {
        true
    } else {
        let a519 = v0;
        let a520 = 42u64;
        nat_eq(a519, a520)
    }
}

pub fn f23(v0: u64) -> bool {
    if f16(v0) {
        true
    } else {
        f15(v0)
    }
}
