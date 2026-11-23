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
    C7(u64, bool, u64, u64, u64, u64, Option<Adt1>, Option<Adt1>, Option<Adt1>, Option<Adt1>, Option<Adt1>, Adt1),
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
        Adt0::C0(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) => {
            f2(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
        }
        Adt0::C1(v52, v53, v54, v55) => {
            f8(v52, v53, v54, v55)
        }
        Adt0::C2(v40, v41, v42, v43, v44, v45, v46, v47) => {
            f6(v40, v41, v42, v43, v44, v45, v46, v47)
        }
        Adt0::C3(v48, v49, v50, v51) => {
            f7(v48, v49, v50, v51)
        }
        Adt0::C4(v24, v25, v26, v27, v28, v29, v30, v31) => {
            f3(v24, v25, v26, v27, v28, v29, v30, v31)
        }
        Adt0::C5(v32, v33, v34, v35) => {
            f4(v32, v33, v34, v35)
        }
        Adt0::C6(v36, v37, v38, v39) => {
            f5(v36, v37, v38, v39)
        }
        Adt0::C7(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
            f1(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: u64, v6: Option<Adt1>, v7: Option<Adt1>, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt5> {
    let m2 = v6;
    match m2 {
        None => {
            Ok(Adt5::C0)
        }
        Some(v12) => {
            let a3 = v0;
            let a4 = 43u64;
            if nat_eq(a3, a4) {
                let m5 = v9;
                match m5 {
                    None => {
                        let m6 = v10;
                        match m6 {
                            None => {
                                f6(v0, v1, v2, v3, v12, v7, v8, v11)
                            }
                            Some(_) => {
                                Ok(Adt5::C0)
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Adt5::C0)
                    }
                }
            } else {
                let m7 = v7;
                match m7 {
                    None => {
                        Ok(Adt5::C0)
                    }
                    Some(v15) => {
                        let a8 = v0;
                        let a9 = 88u64;
                        if nat_eq(a8, a9) {
                            f3(v0, v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let m10 = v8;
                            match m10 {
                                None => {
                                    Ok(Adt5::C0)
                                }
                                Some(v16) => {
                                    f2(v0, v1, v3, v4, v5, v12, v15, v16, v9, v10, v11)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Adt1) -> R<Adt5> {
    let a11 = v0;
    let a12 = 71u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a11, a12) {
        !v1
    } else {
        false
    } {
        f9(v2, match v5.clone() {
            Adt1::C0(_, _, _, h13, _, _, _, _, _, _, _) => {
                h13
            }
        }, v8.clone())
    } else {
        false
    } {
        let a14 = v4;
        let a15 = 0u64;
        if if nat_eq(a14, a15) {
            true
        } else {
            let a16 = v4;
            let a17 = 10u64;
            nat_eq(a16, a17)
        } {
            true
        } else {
            let a18 = v4;
            let a19 = 15u64;
            nat_eq(a18, a19)
        }
    } else {
        false
    } {
        let a21 = match v5.clone() {
            Adt1::C0(h20, _, _, _, _, _, _, _, _, _, _) => {
                h20
            }
        };
        let a22 = 0u64;
        nat_eq(a21, a22)
    } else {
        false
    } {
        let a24 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, h23, _, _, _, _, _) => {
                h23
            }
        };
        let a25 = 4u64;
        nat_eq(a24, a25)
    } else {
        false
    } {
        f10(v10.clone())?
    } else {
        false
    } {
        f11(v6.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        f11(v7.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        let a28 = match v5.clone() {
            Adt1::C0(_, h26, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        };
        let a29 = match v6.clone() {
            Adt1::C0(_, h27, _, _, _, _, _, _, _, _, _) => {
                h27
            }
        };
        nat_eq(a28, a29)
    } else {
        false
    } {
        let a32 = match v6.clone() {
            Adt1::C0(_, _, h30, _, _, _, _, _, _, _, _) => {
                h30
            }
        };
        let a33 = match v7.clone() {
            Adt1::C0(_, _, h31, _, _, _, _, _, _, _, _) => {
                h31
            }
        };
        nat_eq(a32, a33)
    } else {
        false
    } {
        let a36 = match v10.clone() {
            Adt1::C0(_, h34, _, _, _, _, _, _, _, _, _) => {
                h34
            }
        };
        let a37 = match v7.clone() {
            Adt1::C0(_, h35, _, _, _, _, _, _, _, _, _) => {
                h35
            }
        };
        nat_eq(a36, a37)
    } else {
        false
    } {
        let a40 = match v10.clone() {
            Adt1::C0(_, _, h38, _, _, _, _, _, _, _, _) => {
                h38
            }
        };
        let a41 = match v5.clone() {
            Adt1::C0(_, _, _, h39, _, _, _, _, _, _, _) => {
                h39
            }
        };
        nat_eq(a40, a41)
    } else {
        false
    } {
        let a44 = match v10.clone() {
            Adt1::C0(_, _, _, h42, _, _, _, _, _, _, _) => {
                h42
            }
        };
        let a45 = match v5.clone() {
            Adt1::C0(_, _, h43, _, _, _, _, _, _, _, _) => {
                h43
            }
        };
        nat_eq(a44, a45)
    } else {
        false
    } {
        let a48 = match v10.clone() {
            Adt1::C0(_, _, _, _, h46, _, _, _, _, _, _) => {
                h46
            }
        };
        let a49 = match v5.clone() {
            Adt1::C0(_, _, _, _, h47, _, _, _, _, _, _) => {
                h47
            }
        };
        nat_eq(a48, a49)
    } else {
        false
    } {
        f12(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f13(v8.clone(), v5.clone(), v6.clone(), v10.clone())?
    } else {
        false
    } {
        f14(v9.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        Ok(Adt5::C1(Adt2::C0(v5, v6, v7, v8, v9, v10, f15(v2), f16(v3))))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f3(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt5> {
    let v8: u64 = {
        let a51 = match v2.clone() {
            Adt1::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                h50
            }
        };
        let a52 = 1u64;
        if nat_eq(a51, a52) {
            2u64
        } else {
            4u64
        }
    };
    let a53 = v0;
    let a54 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a53, a54) {
        !v1
    } else {
        false
    } {
        let a56 = match v2.clone() {
            Adt1::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
                h55
            }
        };
        let a57 = 0u64;
        if nat_eq(a56, a57) {
            true
        } else {
            let a59 = match v2.clone() {
                Adt1::C0(h58, _, _, _, _, _, _, _, _, _, _) => {
                    h58
                }
            };
            let a60 = 1u64;
            nat_eq(a59, a60)
        }
    } else {
        false
    } {
        let a62 = match v3.clone() {
            Adt1::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        };
        let a63 = 26u64;
        nat_eq(a62, a63)
    } else {
        false
    } {
        let a66 = match v7.clone() {
            Adt1::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        let a67 = match v2.clone() {
            Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        };
        nat_eq(a66, a67)
    } else {
        false
    } {
        let a69 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h68, _, _, _, _, _) => {
                h68
            }
        };
        let a70 = v8;
        nat_eq(a69, a70)
    } else {
        false
    } {
        let a72 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h71, _, _, _, _, _) => {
                h71
            }
        };
        let a73 = v8;
        nat_eq(a72, a73)
    } else {
        false
    } {
        let a76 = match v2.clone() {
            Adt1::C0(_, h74, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        };
        let a77 = match v7.clone() {
            Adt1::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
                h75
            }
        };
        nat_eq(a76, a77)
    } else {
        false
    } {
        f17(v3.clone(), v7.clone())
    } else {
        false
    } {
        f18({
            let a79 = v8;
            let a80 = match v7.clone() {
                Adt1::C0(_, h78, _, _, _, _, _, _, _, _, _) => {
                    h78
                }
            };
            nat_mul(a79, a80)?
        }, match v7.clone() {
            Adt1::C0(_, _, h81, _, _, _, _, _, _, _, _) => {
                h81
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h82, _, _, _, _, _, _, _) => {
                h82
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h83, _, _, _, _, _, _) => {
                h83
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h84, _, _, _, _) => {
                h84
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h85, _, _, _) => {
                h85
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h86, _, _) => {
                h86
            }
        })?
    } else {
        false
    } {
        f12(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f12(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f19(match v3.clone() {
            Adt1::C0(_, _, h87, _, _, _, _, _, _, _, _) => {
                h87
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h88, _, _, _, _, _, _, _, _) => {
                h88
            }
        })
    } else {
        false
    } {
        f19(match v3.clone() {
            Adt1::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                h89
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                h90
            }
        })
    } else {
        false
    } {
        f19(match v3.clone() {
            Adt1::C0(_, _, _, _, h91, _, _, _, _, _, _) => {
                h91
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h92, _, _, _, _, _, _) => {
                h92
            }
        })
    } else {
        false
    } {
        let m93 = v4.clone();
        match m93 {
            None => {
                true
            }
            Some(v9) => {
                let a95 = match v9.clone() {
                    Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                        h94
                    }
                };
                let a96 = 0u64;
                if if nat_eq(a95, a96) {
                    f17(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f12(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m97 = v5.clone();
        match m97 {
            None => {
                true
            }
            Some(v10) => {
                let a99 = match v10.clone() {
                    Adt1::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                        h98
                    }
                };
                let a100 = 26u64;
                if if if if nat_eq(a99, a100) {
                    let a102 = match v10.clone() {
                        Adt1::C0(_, _, _, h101, _, _, _, _, _, _, _) => {
                            h101
                        }
                    };
                    let a103 = 1u64;
                    nat_eq(a102, a103)
                } else {
                    false
                } {
                    let a105 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h104, _, _, _, _, _, _) => {
                            h104
                        }
                    };
                    let a106 = 1u64;
                    nat_eq(a105, a106)
                } else {
                    false
                } {
                    f12(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m107 = v4.clone();
                    match m107 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a110 = match v11 {
                                Adt1::C0(_, h108, _, _, _, _, _, _, _, _, _) => {
                                    h108
                                }
                            };
                            let a111 = match v10 {
                                Adt1::C0(_, _, h109, _, _, _, _, _, _, _, _) => {
                                    h109
                                }
                            };
                            nat_eq(a110, a111)
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
        let m112 = v6.clone();
        match m112 {
            None => {
                true
            }
            Some(v12) => {
                let a114 = match v12.clone() {
                    Adt1::C0(h113, _, _, _, _, _, _, _, _, _, _) => {
                        h113
                    }
                };
                let a115 = 26u64;
                if if nat_eq(a114, a115) {
                    f17(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f12(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt5::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a116 = v8;
            let a117 = 2u64;
            if nat_eq(a116, a117) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m118 = v5;
            match m118 {
                None => {
                    let m119 = v4;
                    match m119 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h120, _, _, _, _, _, _, _, _, _) => {
                                    h120
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h121, _, _, _, _, _, _, _, _, _) => {
                                    h121
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h122, _, _, _, _, _, _, _, _) => {
                            h122
                        }
                    }
                }
            }
        })))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f4(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: Adt1 = match v0.clone() {
        Adt4::C0(h123, _, _, _, _, _, _, _, _) => {
            h123
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h124, _, _, _, _, _, _, _) => {
            h124
        }
    };
    let a126 = v1;
    let a127 = match v5.clone() {
        Adt1::C0(_, _, h125, _, _, _, _, _, _, _, _) => {
            h125
        }
    };
    if if if nat_lt(a126, a127) {
        let a129 = v2;
        let a130 = match v5.clone() {
            Adt1::C0(_, _, _, h128, _, _, _, _, _, _, _) => {
                h128
            }
        };
        nat_lt(a129, a130)
    } else {
        false
    } {
        let a132 = v3;
        let a133 = match v5.clone() {
            Adt1::C0(_, _, _, _, h131, _, _, _, _, _, _) => {
                h131
            }
        };
        nat_lt(a132, a133)
    } else {
        false
    } {
        Ok(Adt5::C6(f20(v4.clone(), {
            let a135 = v1;
            let a136 = match v4.clone() {
                Adt1::C0(_, _, h134, _, _, _, _, _, _, _, _) => {
                    h134
                }
            };
            let a137 = 0u64;
            nat_rem(a135, a136, a137)
        }, {
            let a139 = v2;
            let a140 = match v4.clone() {
                Adt1::C0(_, _, _, h138, _, _, _, _, _, _, _) => {
                    h138
                }
            };
            let a141 = 0u64;
            nat_rem(a139, a140, a141)
        }, {
            let a143 = v3;
            let a144 = match v4 {
                Adt1::C0(_, _, _, _, h142, _, _, _, _, _, _) => {
                    h142
                }
            };
            let a145 = 0u64;
            nat_rem(a143, a144, a145)
        })?, f20(v5, v1, v2, v3)?, {
            let m147 = match v0.clone() {
                Adt4::C0(_, _, h146, _, _, _, _, _, _) => {
                    h146
                }
            };
            match m147 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f20(v6, v1, v2, v3)?
                }
            }
        }, {
            let m149 = match v0.clone() {
                Adt4::C0(_, _, _, _, h148, _, _, _, _) => {
                    h148
                }
            };
            match m149 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f20(v7, v1, v2, v3)?
                }
            }
        }, f20(match v0 {
            Adt4::C0(_, _, _, _, _, h150, _, _, _) => {
                h150
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f5(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a151 = v1;
    let a152 = 0u64;
    if nat_eq(a151, a152) {
        let a156 = v2;
        let a157 = {
            let m155 = match v0.clone() {
                Adt4::C0(_, h154, _, _, _, _, _, _, _) => {
                    h154
                }
            };
            match m155 {
                Adt1::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                    h153
                }
            }
        };
        if nat_lt(a156, a157) {
            Ok(Adt5::C7({
                let a161 = v2;
                let a162 = {
                    let m160 = match v0 {
                        Adt4::C0(_, h159, _, _, _, _, _, _, _) => {
                            h159
                        }
                    };
                    match m160 {
                        Adt1::C0(_, _, _, _, _, h158, _, _, _, _, _) => {
                            h158
                        }
                    }
                };
                nat_mul(a161, a162)?
            }))
        } else {
            Ok(Adt5::C8)
        }
    } else {
        let a163 = v1;
        let a164 = 1u64;
        if nat_eq(a163, a164) {
            let m166 = match v0 {
                Adt4::C0(_, _, _, _, h165, _, _, _, _) => {
                    h165
                }
            };
            match m166 {
                None => {
                    Ok(Adt5::C8)
                }
                Some(v4) => {
                    let a168 = v2;
                    let a169 = match v4.clone() {
                        Adt1::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
                            h167
                        }
                    };
                    if nat_lt(a168, a169) {
                        Ok(Adt5::C7({
                            let a171 = v2;
                            let a172 = match v4 {
                                Adt1::C0(_, _, _, _, _, h170, _, _, _, _, _) => {
                                    h170
                                }
                            };
                            nat_mul(a171, a172)?
                        }))
                    } else {
                        Ok(Adt5::C8)
                    }
                }
            }
        } else {
            let a173 = v1;
            let a174 = 2u64;
            if nat_eq(a173, a174) {
                let a175 = v2;
                let a176 = 2147483648u64;
                if if nat_lt(a175, a176) {
                    let a178 = v2;
                    let a179 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h177) => {
                            h177
                        }
                    };
                    nat_lt(a178, a179)
                } else {
                    false
                } {
                    let m181 = match v0 {
                        Adt4::C0(_, _, h180, _, _, _, _, _, _) => {
                            h180
                        }
                    };
                    match m181 {
                        None => {
                            Ok(Adt5::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt5::C7({
                                let a183 = v2;
                                let a184 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h182, _, _, _, _, _) => {
                                        h182
                                    }
                                };
                                nat_mul(a183, a184)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt5::C8)
                }
            } else {
                let a185 = v1;
                let a186 = 3u64;
                if nat_eq(a185, a186) {
                    let m188 = match v0 {
                        Adt4::C0(_, _, _, h187, _, _, _, _, _) => {
                            h187
                        }
                    };
                    match m188 {
                        None => {
                            Ok(Adt5::C8)
                        }
                        Some(v6) => {
                            let a190 = v2;
                            let a191 = match v6.clone() {
                                Adt1::C0(_, _, h189, _, _, _, _, _, _, _, _) => {
                                    h189
                                }
                            };
                            if if nat_lt(a190, a191) {
                                let a193 = v3;
                                let a194 = match v6.clone() {
                                    Adt1::C0(_, h192, _, _, _, _, _, _, _, _, _) => {
                                        h192
                                    }
                                };
                                nat_lt(a193, a194)
                            } else {
                                false
                            } {
                                Ok(Adt5::C7({
                                    let a201 = {
                                        let a196 = v2;
                                        let a197 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h195, _, _, _, _) => {
                                                h195
                                            }
                                        };
                                        nat_mul(a196, a197)?
                                    };
                                    let a202 = {
                                        let a199 = v3;
                                        let a200 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h198, _, _, _, _, _) => {
                                                h198
                                            }
                                        };
                                        nat_mul(a199, a200)?
                                    };
                                    nat_add(a201, a202)?
                                }))
                            } else {
                                Ok(Adt5::C8)
                            }
                        }
                    }
                } else {
                    let a203 = v1;
                    let a204 = 4u64;
                    if nat_eq(a203, a204) {
                        let a205 = v2;
                        let a206 = 2147483648u64;
                        if if nat_lt(a205, a206) {
                            let a210 = v2;
                            let a211 = {
                                let m209 = match v0.clone() {
                                    Adt4::C0(h208, _, _, _, _, _, _, _, _) => {
                                        h208
                                    }
                                };
                                match m209 {
                                    Adt1::C0(_, h207, _, _, _, _, _, _, _, _, _) => {
                                        h207
                                    }
                                }
                            };
                            nat_lt(a210, a211)
                        } else {
                            false
                        } {
                            Ok(Adt5::C7({
                                let a213 = v2;
                                let a214 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h212, _, _) => {
                                        h212
                                    }
                                };
                                nat_mul(a213, a214)?
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

pub fn f6(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt5> {
    if v1 {
        Ok(Adt5::C0)
    } else {
        let a215 = v0;
        let a216 = 43u64;
        if if if if if if if if nat_eq(a215, a216) {
            let a218 = match v4.clone() {
                Adt1::C0(h217, _, _, _, _, _, _, _, _, _, _) => {
                    h217
                }
            };
            let a219 = 0u64;
            if nat_eq(a218, a219) {
                f21(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a221 = match v7.clone() {
                Adt1::C0(h220, _, _, _, _, _, _, _, _, _, _) => {
                    h220
                }
            };
            let a222 = 0u64;
            if nat_eq(a221, a222) {
                f21(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f22(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f12(v4.clone(), v7.clone())? {
                true
            } else {
                f23(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f24(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f25(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f9(v3, match v4.clone() {
                Adt1::C0(_, _, _, h223, _, _, _, _, _, _, _) => {
                    h223
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt5::C3(Adt3::C0(v4, v5, v7, v2, f15(v3))))
        } else {
            Ok(Adt5::C0)
        }
    }
}

pub fn f7(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h224, _, _, _, _) => {
            h224
        }
    };
    let a226 = v1;
    let a227 = match v4.clone() {
        Adt1::C0(_, _, h225, _, _, _, _, _, _, _, _) => {
            h225
        }
    };
    if if if nat_lt(a226, a227) {
        let a229 = v2;
        let a230 = match v4.clone() {
            Adt1::C0(_, _, _, h228, _, _, _, _, _, _, _) => {
                h228
            }
        };
        nat_lt(a229, a230)
    } else {
        false
    } {
        let a232 = v3;
        let a233 = match v4.clone() {
            Adt1::C0(_, _, _, _, h231, _, _, _, _, _, _) => {
                h231
            }
        };
        nat_lt(a232, a233)
    } else {
        false
    } {
        let v6: u64 = {
            let m235 = match v0.clone() {
                Adt3::C0(_, h234, _, _, _) => {
                    h234
                }
            };
            match m235 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f20(v5.clone(), v1, {
                        let a237 = v2;
                        let a238 = match v5.clone() {
                            Adt1::C0(_, _, _, h236, _, _, _, _, _, _, _) => {
                                h236
                            }
                        };
                        let a239 = 0u64;
                        nat_rem(a237, a238, a239)
                    }, {
                        let a241 = v3;
                        let a242 = match v5 {
                            Adt1::C0(_, _, _, _, h240, _, _, _, _, _, _) => {
                                h240
                            }
                        };
                        let a243 = 0u64;
                        nat_rem(a241, a242, a243)
                    })?
                }
            }
        };
        Ok(Adt5::C4(f20(v4, v1, v2, v3)?, v6, {
            let a244 = v2;
            let a245 = 4u64;
            nat_mul(a244, a245)?
        }, f20(match v0 {
            Adt3::C0(_, _, h246, _, _) => {
                h246
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f8(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a250 = v1;
    let a251 = {
        let m249 = match v0.clone() {
            Adt2::C0(h248, _, _, _, _, _, _, _) => {
                h248
            }
        };
        match m249 {
            Adt1::C0(_, _, h247, _, _, _, _, _, _, _, _) => {
                h247
            }
        }
    };
    if if if nat_lt(a250, a251) {
        let a255 = v2;
        let a256 = {
            let m254 = match v0.clone() {
                Adt2::C0(h253, _, _, _, _, _, _, _) => {
                    h253
                }
            };
            match m254 {
                Adt1::C0(_, _, _, h252, _, _, _, _, _, _, _) => {
                    h252
                }
            }
        };
        nat_lt(a255, a256)
    } else {
        false
    } {
        let a260 = v3;
        let a261 = {
            let m259 = match v0.clone() {
                Adt2::C0(h258, _, _, _, _, _, _, _) => {
                    h258
                }
            };
            match m259 {
                Adt1::C0(_, _, _, _, h257, _, _, _, _, _, _) => {
                    h257
                }
            }
        };
        nat_lt(a260, a261)
    } else {
        false
    } {
        Ok(Adt5::C2(f20(match v0.clone() {
            Adt2::C0(h262, _, _, _, _, _, _, _) => {
                h262
            }
        }, v1, v2, v3)?, f20(match v0.clone() {
            Adt2::C0(_, h263, _, _, _, _, _, _) => {
                h263
            }
        }, 0u64, {
            let a273 = v2;
            let a274 = {
                let a270 = {
                    let m266 = match v0.clone() {
                        Adt2::C0(h265, _, _, _, _, _, _, _) => {
                            h265
                        }
                    };
                    match m266 {
                        Adt1::C0(_, _, _, h264, _, _, _, _, _, _, _) => {
                            h264
                        }
                    }
                };
                let a271 = {
                    let m269 = match v0.clone() {
                        Adt2::C0(_, h268, _, _, _, _, _, _) => {
                            h268
                        }
                    };
                    match m269 {
                        Adt1::C0(_, _, _, h267, _, _, _, _, _, _, _) => {
                            h267
                        }
                    }
                };
                let a272 = 0u64;
                nat_quot(a270, a271, a272)
            };
            let a275 = 0u64;
            nat_quot(a273, a274, a275)
        }, {
            let a285 = v3;
            let a286 = {
                let a282 = {
                    let m278 = match v0.clone() {
                        Adt2::C0(h277, _, _, _, _, _, _, _) => {
                            h277
                        }
                    };
                    match m278 {
                        Adt1::C0(_, _, _, _, h276, _, _, _, _, _, _) => {
                            h276
                        }
                    }
                };
                let a283 = {
                    let m281 = match v0.clone() {
                        Adt2::C0(_, h280, _, _, _, _, _, _) => {
                            h280
                        }
                    };
                    match m281 {
                        Adt1::C0(_, _, _, _, h279, _, _, _, _, _, _) => {
                            h279
                        }
                    }
                };
                let a284 = 0u64;
                nat_quot(a282, a283, a284)
            };
            let a287 = 0u64;
            nat_quot(a285, a286, a287)
        })?, f20(match v0.clone() {
            Adt2::C0(_, _, h288, _, _, _, _, _) => {
                h288
            }
        }, 0u64, {
            let a298 = v2;
            let a299 = {
                let a295 = {
                    let m291 = match v0.clone() {
                        Adt2::C0(h290, _, _, _, _, _, _, _) => {
                            h290
                        }
                    };
                    match m291 {
                        Adt1::C0(_, _, _, h289, _, _, _, _, _, _, _) => {
                            h289
                        }
                    }
                };
                let a296 = {
                    let m294 = match v0.clone() {
                        Adt2::C0(_, _, h293, _, _, _, _, _) => {
                            h293
                        }
                    };
                    match m294 {
                        Adt1::C0(_, _, _, h292, _, _, _, _, _, _, _) => {
                            h292
                        }
                    }
                };
                let a297 = 0u64;
                nat_quot(a295, a296, a297)
            };
            let a300 = 0u64;
            nat_quot(a298, a299, a300)
        }, {
            let a310 = v3;
            let a311 = {
                let a307 = {
                    let m303 = match v0.clone() {
                        Adt2::C0(h302, _, _, _, _, _, _, _) => {
                            h302
                        }
                    };
                    match m303 {
                        Adt1::C0(_, _, _, _, h301, _, _, _, _, _, _) => {
                            h301
                        }
                    }
                };
                let a308 = {
                    let m306 = match v0.clone() {
                        Adt2::C0(_, _, h305, _, _, _, _, _) => {
                            h305
                        }
                    };
                    match m306 {
                        Adt1::C0(_, _, _, _, h304, _, _, _, _, _, _) => {
                            h304
                        }
                    }
                };
                let a309 = 0u64;
                nat_quot(a307, a308, a309)
            };
            let a312 = 0u64;
            nat_quot(a310, a311, a312)
        })?, {
            let m314 = match v0.clone() {
                Adt2::C0(_, _, _, h313, _, _, _, _) => {
                    h313
                }
            };
            match m314 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f20(v4.clone(), v1, {
                        let a316 = v2;
                        let a317 = match v4.clone() {
                            Adt1::C0(_, _, _, h315, _, _, _, _, _, _, _) => {
                                h315
                            }
                        };
                        let a318 = 0u64;
                        nat_rem(a316, a317, a318)
                    }, {
                        let a320 = v3;
                        let a321 = match v4 {
                            Adt1::C0(_, _, _, _, h319, _, _, _, _, _, _) => {
                                h319
                            }
                        };
                        let a322 = 0u64;
                        nat_rem(a320, a321, a322)
                    })?
                }
            }
        }, {
            let a323 = v2;
            let a324 = 4u64;
            nat_mul(a323, a324)?
        }, f20(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h325, _, _) => {
                h325
            }
        }, v2, v1, v3)?, {
            let m328 = match v0.clone() {
                Adt2::C0(_, h327, _, _, _, _, _, _) => {
                    h327
                }
            };
            match m328 {
                Adt1::C0(_, _, _, _, _, _, h326, _, _, _, _) => {
                    h326
                }
            }
        }, {
            let m331 = match v0.clone() {
                Adt2::C0(_, _, h330, _, _, _, _, _) => {
                    h330
                }
            };
            match m331 {
                Adt1::C0(_, _, _, _, _, _, h329, _, _, _, _) => {
                    h329
                }
            }
        }, f26({
            let m334 = match v0 {
                Adt2::C0(_, h333, _, _, _, _, _, _) => {
                    h333
                }
            };
            match m334 {
                Adt1::C0(h332, _, _, _, _, _, _, _, _, _, _) => {
                    h332
                }
            }
        })))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f9(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a335 = v0;
    let a336 = 0u64;
    if if nat_eq(a335, a336) {
        true
    } else {
        let a337 = 2147483648u64;
        let a338 = v0;
        if nat_le(a337, a338) {
            let a339 = v0;
            let a340 = 4286578688u64;
            nat_lt(a339, a340)
        } else {
            false
        }
    } {
        true
    } else {
        let a341 = v0;
        let a342 = 2139095040u64;
        if if nat_lt(a341, a342) {
            let a343 = v1;
            let a344 = 4294967295u64;
            nat_le(a343, a344)
        } else {
            false
        } {
            let m345 = v2;
            match m345 {
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

pub fn f10(v0: Adt1) -> R<bool> {
    let a347 = match v0.clone() {
        Adt1::C0(h346, _, _, _, _, _, _, _, _, _, _) => {
            h346
        }
    };
    let a348 = 0u64;
    if if if if nat_eq(a347, a348) {
        let a350 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h349, _, _, _, _, _) => {
                h349
            }
        };
        let a351 = 4u64;
        nat_eq(a350, a351)
    } else {
        false
    } {
        let a357 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h352, _, _, _, _) => {
                h352
            }
        };
        let a358 = {
            let a355 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h353, _, _, _, _, _) => {
                    h353
                }
            };
            let a356 = match v0.clone() {
                Adt1::C0(_, h354, _, _, _, _, _, _, _, _, _) => {
                    h354
                }
            };
            nat_mul(a355, a356)?
        };
        nat_eq(a357, a358)
    } else {
        false
    } {
        let a364 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h359, _, _, _) => {
                h359
            }
        };
        let a365 = {
            let a362 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h360, _, _, _, _) => {
                    h360
                }
            };
            let a363 = match v0.clone() {
                Adt1::C0(_, _, h361, _, _, _, _, _, _, _, _) => {
                    h361
                }
            };
            nat_mul(a362, a363)?
        };
        nat_eq(a364, a365)
    } else {
        false
    } {
        let a371 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h366, _, _) => {
                h366
            }
        };
        let a372 = {
            let a369 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h367, _, _, _) => {
                    h367
                }
            };
            let a370 = match v0 {
                Adt1::C0(_, _, _, h368, _, _, _, _, _, _, _) => {
                    h368
                }
            };
            nat_mul(a369, a370)?
        };
        Ok(nat_eq(a371, a372))
    } else {
        Ok(false)
    }
}

pub fn f11(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f27(match v0.clone() {
        Adt1::C0(h373, _, _, _, _, _, _, _, _, _, _) => {
            h373
        }
    }, match v0.clone() {
        Adt1::C0(_, h374, _, _, _, _, _, _, _, _, _) => {
            h374
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h375, _, _, _, _, _) => {
            h375
        }
    }) {
        f19(match v1.clone() {
            Adt1::C0(_, _, _, h376, _, _, _, _, _, _, _) => {
                h376
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h377, _, _, _, _, _, _, _) => {
                h377
            }
        })
    } else {
        false
    } {
        f19(match v1 {
            Adt1::C0(_, _, _, _, h378, _, _, _, _, _, _) => {
                h378
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h379, _, _, _, _, _, _) => {
                h379
            }
        })
    } else {
        false
    } {
        f12(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f12(v0: Adt1, v1: Adt1) -> R<bool> {
    let a381 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h380, _) => {
            h380
        }
    };
    let a382 = 0u64;
    if nat_eq(a381, a382) {
        Ok(true)
    } else {
        let a384 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h383, _) => {
                h383
            }
        };
        let a385 = 0u64;
        if nat_eq(a384, a385) {
            Ok(true)
        } else {
            let a387 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h386) => {
                    h386
                }
            };
            let a388 = 0u64;
            if nat_eq(a387, a388) {
                Ok(true)
            } else {
                let a390 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h389) => {
                        h389
                    }
                };
                let a391 = 0u64;
                if nat_eq(a390, a391) {
                    Ok(true)
                } else {
                    let a397 = {
                        let a394 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h392) => {
                                h392
                            }
                        };
                        let a395 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h393, _) => {
                                h393
                            }
                        };
                        nat_add(a394, a395)?
                    };
                    let a398 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h396) => {
                            h396
                        }
                    };
                    if nat_le(a397, a398) {
                        Ok(true)
                    } else {
                        let a404 = {
                            let a401 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h399) => {
                                    h399
                                }
                            };
                            let a402 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h400, _) => {
                                    h400
                                }
                            };
                            nat_add(a401, a402)?
                        };
                        let a405 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h403) => {
                                h403
                            }
                        };
                        Ok(nat_le(a404, a405))
                    }
                }
            }
        }
    }
}

pub fn f13(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m406 = v0;
    match m406 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a408 = match v4.clone() {
                Adt1::C0(h407, _, _, _, _, _, _, _, _, _, _) => {
                    h407
                }
            };
            let a409 = 1u64;
            if if if if if if nat_eq(a408, a409) {
                let a411 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h410, _, _, _, _, _) => {
                        h410
                    }
                };
                let a412 = 2u64;
                nat_eq(a411, a412)
            } else {
                false
            } {
                let a415 = match v2 {
                    Adt1::C0(_, _, h413, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                };
                let a416 = match v4.clone() {
                    Adt1::C0(_, h414, _, _, _, _, _, _, _, _, _) => {
                        h414
                    }
                };
                nat_le(a415, a416)
            } else {
                false
            } {
                let a419 = match v1.clone() {
                    Adt1::C0(_, _, h417, _, _, _, _, _, _, _, _) => {
                        h417
                    }
                };
                let a420 = match v4.clone() {
                    Adt1::C0(_, _, h418, _, _, _, _, _, _, _, _) => {
                        h418
                    }
                };
                nat_le(a419, a420)
            } else {
                false
            } {
                f19(match v1.clone() {
                    Adt1::C0(_, _, _, h421, _, _, _, _, _, _, _) => {
                        h421
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h422, _, _, _, _, _, _, _) => {
                        h422
                    }
                })
            } else {
                false
            } {
                f19(match v1 {
                    Adt1::C0(_, _, _, _, h423, _, _, _, _, _, _) => {
                        h423
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h424, _, _, _, _, _, _) => {
                        h424
                    }
                })
            } else {
                false
            } {
                f12(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f14(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m425 = v0;
    match m425 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a427 = match v3.clone() {
                Adt1::C0(h426, _, _, _, _, _, _, _, _, _, _) => {
                    h426
                }
            };
            let a428 = 0u64;
            if if if if if if nat_eq(a427, a428) {
                let a430 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h429, _, _, _, _, _) => {
                        h429
                    }
                };
                let a431 = 4u64;
                nat_eq(a430, a431)
            } else {
                false
            } {
                let a434 = match v3.clone() {
                    Adt1::C0(_, h432, _, _, _, _, _, _, _, _, _) => {
                        h432
                    }
                };
                let a435 = match v1 {
                    Adt1::C0(_, _, _, h433, _, _, _, _, _, _, _) => {
                        h433
                    }
                };
                nat_eq(a434, a435)
            } else {
                false
            } {
                let a437 = match v3.clone() {
                    Adt1::C0(_, _, h436, _, _, _, _, _, _, _, _) => {
                        h436
                    }
                };
                let a438 = 1u64;
                nat_eq(a437, a438)
            } else {
                false
            } {
                let a440 = match v3.clone() {
                    Adt1::C0(_, _, _, h439, _, _, _, _, _, _, _) => {
                        h439
                    }
                };
                let a441 = 1u64;
                nat_eq(a440, a441)
            } else {
                false
            } {
                let a443 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h442, _, _, _, _, _, _) => {
                        h442
                    }
                };
                let a444 = 1u64;
                nat_eq(a443, a444)
            } else {
                false
            } {
                f12(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f15(v0: u64) -> Option<u64> {
    let a445 = 0u64;
    let a446 = v0;
    if if nat_lt(a445, a446) {
        let a447 = v0;
        let a448 = 2139095040u64;
        nat_lt(a447, a448)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f16(v0: u64) -> Option<u64> {
    let a449 = v0;
    let a450 = 0u64;
    if if nat_eq(a449, a450) {
        true
    } else {
        let a451 = v0;
        let a452 = 2147483648u64;
        nat_eq(a451, a452)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f17(v0: Adt1, v1: Adt1) -> bool {
    let a455 = match v0.clone() {
        Adt1::C0(_, _, h453, _, _, _, _, _, _, _, _) => {
            h453
        }
    };
    let a456 = match v1.clone() {
        Adt1::C0(_, _, h454, _, _, _, _, _, _, _, _) => {
            h454
        }
    };
    if if nat_eq(a455, a456) {
        let a459 = match v0.clone() {
            Adt1::C0(_, _, _, h457, _, _, _, _, _, _, _) => {
                h457
            }
        };
        let a460 = match v1.clone() {
            Adt1::C0(_, _, _, h458, _, _, _, _, _, _, _) => {
                h458
            }
        };
        nat_eq(a459, a460)
    } else {
        false
    } {
        let a463 = match v0 {
            Adt1::C0(_, _, _, _, h461, _, _, _, _, _, _) => {
                h461
            }
        };
        let a464 = match v1 {
            Adt1::C0(_, _, _, _, h462, _, _, _, _, _, _) => {
                h462
            }
        };
        nat_eq(a463, a464)
    } else {
        false
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f28(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f28(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f28(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f28(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f28(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f28(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f19(v0: u64, v1: u64) -> bool {
    let a465 = 0u64;
    let a466 = v1;
    if nat_lt(a465, a466) {
        let a470 = {
            let a467 = v0;
            let a468 = v1;
            let a469 = 0u64;
            nat_rem(a467, a468, a469)
        };
        let a471 = 0u64;
        nat_eq(a470, a471)
    } else {
        false
    }
}

pub fn f20(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a483 = {
        let a478 = {
            let a473 = v1;
            let a474 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h472, _, _, _, _) => {
                    h472
                }
            };
            nat_mul(a473, a474)?
        };
        let a479 = {
            let a476 = v2;
            let a477 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h475, _, _, _) => {
                    h475
                }
            };
            nat_mul(a476, a477)?
        };
        nat_add(a478, a479)?
    };
    let a484 = {
        let a481 = v3;
        let a482 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h480, _, _) => {
                h480
            }
        };
        nat_mul(a481, a482)?
    };
    nat_add(a483, a484)
}

pub fn f21(v0: Adt1, v1: u64) -> R<bool> {
    let a486 = match v0.clone() {
        Adt1::C0(_, h485, _, _, _, _, _, _, _, _, _) => {
            h485
        }
    };
    let a487 = 1u64;
    if if if if nat_eq(a486, a487) {
        true
    } else {
        let a489 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h488, _, _, _, _, _) => {
                h488
            }
        };
        let a490 = v1;
        nat_eq(a489, a490)
    } {
        let a492 = match v0.clone() {
            Adt1::C0(_, _, h491, _, _, _, _, _, _, _, _) => {
                h491
            }
        };
        let a493 = 1u64;
        if nat_eq(a492, a493) {
            true
        } else {
            let a498 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h494, _, _, _, _) => {
                    h494
                }
            };
            let a499 = {
                let a496 = v1;
                let a497 = match v0.clone() {
                    Adt1::C0(_, h495, _, _, _, _, _, _, _, _, _) => {
                        h495
                    }
                };
                nat_mul(a496, a497)?
            };
            nat_eq(a498, a499)
        }
    } else {
        false
    } {
        let a501 = match v0.clone() {
            Adt1::C0(_, _, _, h500, _, _, _, _, _, _, _) => {
                h500
            }
        };
        let a502 = 1u64;
        if nat_eq(a501, a502) {
            true
        } else {
            let a510 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h503, _, _, _) => {
                    h503
                }
            };
            let a511 = {
                let a508 = {
                    let a505 = v1;
                    let a506 = match v0.clone() {
                        Adt1::C0(_, h504, _, _, _, _, _, _, _, _, _) => {
                            h504
                        }
                    };
                    nat_mul(a505, a506)?
                };
                let a509 = match v0.clone() {
                    Adt1::C0(_, _, h507, _, _, _, _, _, _, _, _) => {
                        h507
                    }
                };
                nat_mul(a508, a509)?
            };
            nat_eq(a510, a511)
        }
    } else {
        false
    } {
        let a513 = match v0.clone() {
            Adt1::C0(_, _, _, _, h512, _, _, _, _, _, _) => {
                h512
            }
        };
        let a514 = 1u64;
        if nat_eq(a513, a514) {
            Ok(true)
        } else {
            let a525 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h515, _, _) => {
                    h515
                }
            };
            let a526 = {
                let a523 = {
                    let a520 = {
                        let a517 = v1;
                        let a518 = match v0.clone() {
                            Adt1::C0(_, h516, _, _, _, _, _, _, _, _, _) => {
                                h516
                            }
                        };
                        nat_mul(a517, a518)?
                    };
                    let a521 = match v0.clone() {
                        Adt1::C0(_, _, h519, _, _, _, _, _, _, _, _) => {
                            h519
                        }
                    };
                    nat_mul(a520, a521)?
                };
                let a524 = match v0 {
                    Adt1::C0(_, _, _, h522, _, _, _, _, _, _, _) => {
                        h522
                    }
                };
                nat_mul(a523, a524)?
            };
            Ok(nat_eq(a525, a526))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> bool {
    let a529 = match v0.clone() {
        Adt1::C0(_, h527, _, _, _, _, _, _, _, _, _) => {
            h527
        }
    };
    let a530 = match v1.clone() {
        Adt1::C0(_, h528, _, _, _, _, _, _, _, _, _) => {
            h528
        }
    };
    if if if nat_eq(a529, a530) {
        let a533 = match v0.clone() {
            Adt1::C0(_, _, h531, _, _, _, _, _, _, _, _) => {
                h531
            }
        };
        let a534 = match v1.clone() {
            Adt1::C0(_, _, h532, _, _, _, _, _, _, _, _) => {
                h532
            }
        };
        nat_eq(a533, a534)
    } else {
        false
    } {
        let a537 = match v0.clone() {
            Adt1::C0(_, _, _, h535, _, _, _, _, _, _, _) => {
                h535
            }
        };
        let a538 = match v1.clone() {
            Adt1::C0(_, _, _, h536, _, _, _, _, _, _, _) => {
                h536
            }
        };
        nat_eq(a537, a538)
    } else {
        false
    } {
        let a541 = match v0 {
            Adt1::C0(_, _, _, _, h539, _, _, _, _, _, _) => {
                h539
            }
        };
        let a542 = match v1 {
            Adt1::C0(_, _, _, _, h540, _, _, _, _, _, _) => {
                h540
            }
        };
        nat_eq(a541, a542)
    } else {
        false
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f22(v0.clone(), v1.clone()) {
        let a545 = match v0.clone() {
            Adt1::C0(h543, _, _, _, _, _, _, _, _, _, _) => {
                h543
            }
        };
        let a546 = match v1.clone() {
            Adt1::C0(h544, _, _, _, _, _, _, _, _, _, _) => {
                h544
            }
        };
        nat_eq(a545, a546)
    } else {
        false
    } {
        let a549 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h547) => {
                h547
            }
        };
        let a550 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h548) => {
                h548
            }
        };
        nat_eq(a549, a550)
    } else {
        false
    } {
        let a553 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h551, _, _, _, _, _) => {
                h551
            }
        };
        let a554 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h552, _, _, _, _, _) => {
                h552
            }
        };
        nat_eq(a553, a554)
    } else {
        false
    } {
        let a557 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h555, _, _, _, _) => {
                h555
            }
        };
        let a558 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h556, _, _, _, _) => {
                h556
            }
        };
        nat_eq(a557, a558)
    } else {
        false
    } {
        let a561 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h559, _, _, _) => {
                h559
            }
        };
        let a562 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h560, _, _, _) => {
                h560
            }
        };
        nat_eq(a561, a562)
    } else {
        false
    } {
        let a565 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h563, _, _) => {
                h563
            }
        };
        let a566 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h564, _, _) => {
                h564
            }
        };
        nat_eq(a565, a566)
    } else {
        false
    }
}

pub fn f24(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m567 = v0;
    match m567 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a569 = match v3.clone() {
                Adt1::C0(h568, _, _, _, _, _, _, _, _, _, _) => {
                    h568
                }
            };
            let a570 = 0u64;
            if if if if if if if nat_eq(a569, a570) {
                true
            } else {
                let a572 = match v3.clone() {
                    Adt1::C0(h571, _, _, _, _, _, _, _, _, _, _) => {
                        h571
                    }
                };
                let a573 = 1u64;
                nat_eq(a572, a573)
            } {
                f21(v3.clone(), {
                    let a575 = match v3.clone() {
                        Adt1::C0(h574, _, _, _, _, _, _, _, _, _, _) => {
                            h574
                        }
                    };
                    let a576 = 0u64;
                    if nat_eq(a575, a576) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a579 = match v3.clone() {
                    Adt1::C0(_, h577, _, _, _, _, _, _, _, _, _) => {
                        h577
                    }
                };
                let a580 = match v1.clone() {
                    Adt1::C0(_, h578, _, _, _, _, _, _, _, _, _) => {
                        h578
                    }
                };
                nat_eq(a579, a580)
            } else {
                false
            } {
                let a583 = match v1.clone() {
                    Adt1::C0(_, _, h581, _, _, _, _, _, _, _, _) => {
                        h581
                    }
                };
                let a584 = match v3.clone() {
                    Adt1::C0(_, _, h582, _, _, _, _, _, _, _, _) => {
                        h582
                    }
                };
                nat_le(a583, a584)
            } else {
                false
            } {
                f19(match v1.clone() {
                    Adt1::C0(_, _, _, h585, _, _, _, _, _, _, _) => {
                        h585
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h586, _, _, _, _, _, _, _) => {
                        h586
                    }
                })
            } else {
                false
            } {
                f19(match v1 {
                    Adt1::C0(_, _, _, _, h587, _, _, _, _, _, _) => {
                        h587
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h588, _, _, _, _, _, _) => {
                        h588
                    }
                })
            } else {
                false
            } {
                f12(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f25(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m589 = v0;
    match m589 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a591 = match v3.clone() {
                Adt1::C0(h590, _, _, _, _, _, _, _, _, _, _) => {
                    h590
                }
            };
            let a592 = 0u64;
            if if if if if if nat_eq(a591, a592) {
                let a594 = match v3.clone() {
                    Adt1::C0(_, h593, _, _, _, _, _, _, _, _, _) => {
                        h593
                    }
                };
                let a595 = 1u64;
                if nat_eq(a594, a595) {
                    true
                } else {
                    let a597 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h596, _, _, _, _, _) => {
                            h596
                        }
                    };
                    let a598 = 4u64;
                    nat_eq(a597, a598)
                }
            } else {
                false
            } {
                let a601 = match v3.clone() {
                    Adt1::C0(_, h599, _, _, _, _, _, _, _, _, _) => {
                        h599
                    }
                };
                let a602 = match v1 {
                    Adt1::C0(_, _, _, h600, _, _, _, _, _, _, _) => {
                        h600
                    }
                };
                nat_eq(a601, a602)
            } else {
                false
            } {
                let a604 = match v3.clone() {
                    Adt1::C0(_, _, h603, _, _, _, _, _, _, _, _) => {
                        h603
                    }
                };
                let a605 = 1u64;
                nat_eq(a604, a605)
            } else {
                false
            } {
                let a607 = match v3.clone() {
                    Adt1::C0(_, _, _, h606, _, _, _, _, _, _, _) => {
                        h606
                    }
                };
                let a608 = 1u64;
                nat_eq(a607, a608)
            } else {
                false
            } {
                let a610 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h609, _, _, _, _, _, _) => {
                        h609
                    }
                };
                let a611 = 1u64;
                nat_eq(a610, a611)
            } else {
                false
            } {
                f12(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f26(v0: u64) -> u64 {
    if f29(v0) {
        8u64
    } else {
        if f30(v0) {
            9u64
        } else {
            f31(v0)
        }
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> bool {
    let m612 = f32(v0);
    match m612 {
        Adt6::C0 => {
            false
        }
        Adt6::C1(v3) => {
            let a613 = v0;
            let a614 = 0u64;
            if if if if if nat_eq(a613, a614) {
                true
            } else {
                let a615 = v0;
                let a616 = 1u64;
                nat_eq(a615, a616)
            } {
                true
            } else {
                let a617 = v0;
                let a618 = 30u64;
                nat_eq(a617, a618)
            } {
                true
            } else {
                f33(v0)
            } {
                let a620 = v2;
                let a621 = match v3.clone() {
                    Adt7::C0(_, _, h619) => {
                        h619
                    }
                };
                nat_eq(a620, a621)
            } else {
                false
            } {
                f19(v1, match v3 {
                    Adt7::C0(_, h622, _) => {
                        h622
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a623 = v1;
    let a624 = 1u64;
    if if if nat_le(a623, a624) {
        true
    } else {
        let a625 = v0;
        let a626 = v2;
        nat_le(a625, a626)
    } {
        let a627 = v3;
        let a628 = 1u64;
        if nat_le(a627, a628) {
            true
        } else {
            let a635 = {
                let a633 = v0;
                let a634 = {
                    let a631 = {
                        let a629 = v1;
                        let a630 = 1u64;
                        nat_sub(a629, a630)
                    };
                    let a632 = v2;
                    nat_mul(a631, a632)?
                };
                nat_add(a633, a634)?
            };
            let a636 = v4;
            nat_le(a635, a636)
        }
    } else {
        false
    } {
        let a637 = v5;
        let a638 = 1u64;
        if nat_le(a637, a638) {
            Ok(true)
        } else {
            let a651 = {
                let a649 = {
                    let a643 = v0;
                    let a644 = {
                        let a641 = {
                            let a639 = v1;
                            let a640 = 1u64;
                            nat_sub(a639, a640)
                        };
                        let a642 = v2;
                        nat_mul(a641, a642)?
                    };
                    nat_add(a643, a644)?
                };
                let a650 = {
                    let a647 = {
                        let a645 = v3;
                        let a646 = 1u64;
                        nat_sub(a645, a646)
                    };
                    let a648 = v4;
                    nat_mul(a647, a648)?
                };
                nat_add(a649, a650)?
            };
            let a652 = v6;
            Ok(nat_le(a651, a652))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: u64) -> bool {
    let a653 = v0;
    let a654 = 2u64;
    if if if nat_eq(a653, a654) {
        true
    } else {
        let a655 = v0;
        let a656 = 6u64;
        nat_eq(a655, a656)
    } {
        true
    } else {
        let a657 = v0;
        let a658 = 8u64;
        nat_eq(a657, a658)
    } {
        true
    } else {
        let a659 = v0;
        let a660 = 20u64;
        nat_eq(a659, a660)
    }
}

pub fn f30(v0: u64) -> bool {
    let a661 = v0;
    let a662 = 3u64;
    if nat_eq(a661, a662) {
        true
    } else {
        let a663 = v0;
        let a664 = 7u64;
        nat_eq(a663, a664)
    }
}

pub fn f31(v0: u64) -> u64 {
    let a665 = v0;
    let a666 = 30u64;
    if nat_eq(a665, a666) {
        30u64
    } else {
        0u64
    }
}

pub fn f32(v0: u64) -> Adt6 {
    let a667 = v0;
    let a668 = 0u64;
    if nat_eq(a667, a668) {
        Adt6::C1(Adt7::C0(0u64, 1u64, 4u64))
    } else {
        let a669 = v0;
        let a670 = 1u64;
        if nat_eq(a669, a670) {
            Adt6::C1(Adt7::C0(1u64, 1u64, 2u64))
        } else {
            let a671 = v0;
            let a672 = 2u64;
            if nat_eq(a671, a672) {
                Adt6::C1(Adt7::C0(2u64, 32u64, 18u64))
            } else {
                let a673 = v0;
                let a674 = 3u64;
                if nat_eq(a673, a674) {
                    Adt6::C1(Adt7::C0(3u64, 32u64, 20u64))
                } else {
                    let a675 = v0;
                    let a676 = 6u64;
                    if nat_eq(a675, a676) {
                        Adt6::C1(Adt7::C0(6u64, 32u64, 22u64))
                    } else {
                        let a677 = v0;
                        let a678 = 7u64;
                        if nat_eq(a677, a678) {
                            Adt6::C1(Adt7::C0(7u64, 32u64, 24u64))
                        } else {
                            let a679 = v0;
                            let a680 = 8u64;
                            if nat_eq(a679, a680) {
                                Adt6::C1(Adt7::C0(8u64, 32u64, 34u64))
                            } else {
                                let a681 = v0;
                                let a682 = 9u64;
                                if nat_eq(a681, a682) {
                                    Adt6::C1(Adt7::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a683 = v0;
                                    let a684 = 10u64;
                                    if nat_eq(a683, a684) {
                                        Adt6::C1(Adt7::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a685 = v0;
                                        let a686 = 11u64;
                                        if nat_eq(a685, a686) {
                                            Adt6::C1(Adt7::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a687 = v0;
                                            let a688 = 12u64;
                                            if nat_eq(a687, a688) {
                                                Adt6::C1(Adt7::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a689 = v0;
                                                let a690 = 13u64;
                                                if nat_eq(a689, a690) {
                                                    Adt6::C1(Adt7::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a691 = v0;
                                                    let a692 = 14u64;
                                                    if nat_eq(a691, a692) {
                                                        Adt6::C1(Adt7::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a693 = v0;
                                                        let a694 = 15u64;
                                                        if nat_eq(a693, a694) {
                                                            Adt6::C1(Adt7::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a695 = v0;
                                                            let a696 = 16u64;
                                                            if nat_eq(a695, a696) {
                                                                Adt6::C1(Adt7::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a697 = v0;
                                                                let a698 = 17u64;
                                                                if nat_eq(a697, a698) {
                                                                    Adt6::C1(Adt7::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a699 = v0;
                                                                    let a700 = 18u64;
                                                                    if nat_eq(a699, a700) {
                                                                        Adt6::C1(Adt7::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a701 = v0;
                                                                        let a702 = 19u64;
                                                                        if nat_eq(a701, a702) {
                                                                            Adt6::C1(Adt7::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a703 = v0;
                                                                            let a704 = 20u64;
                                                                            if nat_eq(a703, a704) {
                                                                                Adt6::C1(Adt7::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a705 = v0;
                                                                                let a706 = 21u64;
                                                                                if nat_eq(a705, a706) {
                                                                                    Adt6::C1(Adt7::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a707 = v0;
                                                                                    let a708 = 22u64;
                                                                                    if nat_eq(a707, a708) {
                                                                                        Adt6::C1(Adt7::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a709 = v0;
                                                                                        let a710 = 23u64;
                                                                                        if nat_eq(a709, a710) {
                                                                                            Adt6::C1(Adt7::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a711 = v0;
                                                                                            let a712 = 24u64;
                                                                                            if nat_eq(a711, a712) {
                                                                                                Adt6::C1(Adt7::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a713 = v0;
                                                                                                let a714 = 25u64;
                                                                                                if nat_eq(a713, a714) {
                                                                                                    Adt6::C1(Adt7::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a715 = v0;
                                                                                                    let a716 = 26u64;
                                                                                                    if nat_eq(a715, a716) {
                                                                                                        Adt6::C1(Adt7::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a717 = v0;
                                                                                                        let a718 = 27u64;
                                                                                                        if nat_eq(a717, a718) {
                                                                                                            Adt6::C1(Adt7::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a719 = v0;
                                                                                                            let a720 = 28u64;
                                                                                                            if nat_eq(a719, a720) {
                                                                                                                Adt6::C1(Adt7::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a721 = v0;
                                                                                                                let a722 = 29u64;
                                                                                                                if nat_eq(a721, a722) {
                                                                                                                    Adt6::C1(Adt7::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a723 = v0;
                                                                                                                    let a724 = 30u64;
                                                                                                                    if nat_eq(a723, a724) {
                                                                                                                        Adt6::C1(Adt7::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a725 = v0;
                                                                                                                        let a726 = 34u64;
                                                                                                                        if nat_eq(a725, a726) {
                                                                                                                            Adt6::C1(Adt7::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a727 = v0;
                                                                                                                            let a728 = 35u64;
                                                                                                                            if nat_eq(a727, a728) {
                                                                                                                                Adt6::C1(Adt7::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a729 = v0;
                                                                                                                                let a730 = 39u64;
                                                                                                                                if nat_eq(a729, a730) {
                                                                                                                                    Adt6::C1(Adt7::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a731 = v0;
                                                                                                                                    let a732 = 40u64;
                                                                                                                                    if nat_eq(a731, a732) {
                                                                                                                                        Adt6::C1(Adt7::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a733 = v0;
                                                                                                                                        let a734 = 41u64;
                                                                                                                                        if nat_eq(a733, a734) {
                                                                                                                                            Adt6::C1(Adt7::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a735 = v0;
                                                                                                                                            let a736 = 42u64;
                                                                                                                                            if nat_eq(a735, a736) {
                                                                                                                                                Adt6::C1(Adt7::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a737 = v0;
                                                                                                                                                let a738 = 43u64;
                                                                                                                                                if nat_eq(a737, a738) {
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

pub fn f33(v0: u64) -> bool {
    if f29(v0) {
        true
    } else {
        f30(v0)
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
