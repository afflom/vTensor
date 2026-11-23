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
    C8(u64, bool, u64, u64, u64, u64, Option<Adt1>, Option<Adt1>, Option<Adt1>, Option<Adt1>, Option<Adt1>, Option<Adt1>, Adt1),
    C9(u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt1, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Adt1, u64, Option<u64>, Option<u64>),
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
    C9(u64),
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
        Adt0::C0(v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42) => {
            f2(v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, None::<Adt1>, v42)
        }
        Adt0::C1(v71, v72, v73, v74) => {
            f8(v71, v72, v73, v74)
        }
        Adt0::C2(v59, v60, v61, v62, v63, v64, v65, v66) => {
            f6(v59, v60, v61, v62, v63, v64, v65, v66)
        }
        Adt0::C3(v67, v68, v69, v70) => {
            f7(v67, v68, v69, v70)
        }
        Adt0::C4(v43, v44, v45, v46, v47, v48, v49, v50) => {
            f3(v43, v44, v45, v46, v47, v48, v49, v50)
        }
        Adt0::C5(v51, v52, v53, v54) => {
            f4(v51, v52, v53, v54)
        }
        Adt0::C6(v55, v56, v57, v58) => {
            f5(v55, v56, v57, v58)
        }
        Adt0::C7(v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) => {
            f1(v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)
        }
        Adt0::C8(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
            let m8 = v14.clone();
            match m8 {
                None => {
                    f1(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v15)
                }
                Some(_) => {
                    let m9 = v9;
                    match m9 {
                        None => {
                            Ok(Adt5::C0)
                        }
                        Some(v17) => {
                            let m10 = v10;
                            match m10 {
                                None => {
                                    Ok(Adt5::C0)
                                }
                                Some(v18) => {
                                    let m11 = v11;
                                    match m11 {
                                        None => {
                                            Ok(Adt5::C0)
                                        }
                                        Some(v19) => {
                                            f2(v3, v4, v6, v7, v8, v17, v18, v19, v12, v13, v14, v15)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C9(v1, v2) => {
            let a2 = v1;
            let a3 = 2147483648u64;
            if nat_lt(a2, a3) {
                let a4 = v1;
                let a5 = v2;
                if nat_lt(a4, a5) {
                    Ok(Adt5::C9(v1))
                } else {
                    Ok(Adt5::C0)
                }
            } else {
                let a6 = v1;
                let a7 = 4294967296u64;
                if nat_lt(a6, a7) {
                    Ok(Adt5::C8)
                } else {
                    Ok(Adt5::C0)
                }
            }
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: u64, v6: Option<Adt1>, v7: Option<Adt1>, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt5> {
    let m12 = v6;
    match m12 {
        None => {
            Ok(Adt5::C0)
        }
        Some(v12) => {
            let a13 = v0;
            let a14 = 43u64;
            if nat_eq(a13, a14) {
                let m15 = v9;
                match m15 {
                    None => {
                        let m16 = v10;
                        match m16 {
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
                let m17 = v7;
                match m17 {
                    None => {
                        Ok(Adt5::C0)
                    }
                    Some(v15) => {
                        let a18 = v0;
                        let a19 = 88u64;
                        if nat_eq(a18, a19) {
                            f3(v0, v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let m20 = v8;
                            match m20 {
                                None => {
                                    Ok(Adt5::C0)
                                }
                                Some(v16) => {
                                    f2(v0, v1, v3, v4, v5, v12, v15, v16, v9, v10, None::<Adt1>, v11)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt5> {
    let m21 = f9(v10, v8.clone(), v5.clone(), v6.clone(), v7.clone(), v11.clone())?;
    match m21 {
        None => {
            Ok(Adt5::C0)
        }
        Some(v12) => {
            let a22 = v0;
            let a23 = 71u64;
            if if if if if if if if if if if if if if if if if if nat_eq(a22, a23) {
                !v1
            } else {
                false
            } {
                f10(v2, match v5.clone() {
                    Adt1::C0(_, _, _, h24, _, _, _, _, _, _, _) => {
                        h24
                    }
                }, v8.clone())
            } else {
                false
            } {
                let a25 = v4;
                let a26 = 0u64;
                if if nat_eq(a25, a26) {
                    true
                } else {
                    let a27 = v4;
                    let a28 = 10u64;
                    nat_eq(a27, a28)
                } {
                    true
                } else {
                    let a29 = v4;
                    let a30 = 15u64;
                    nat_eq(a29, a30)
                }
            } else {
                false
            } {
                let a32 = match v5.clone() {
                    Adt1::C0(h31, _, _, _, _, _, _, _, _, _, _) => {
                        h31
                    }
                };
                let a33 = 0u64;
                nat_eq(a32, a33)
            } else {
                false
            } {
                let a35 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h34, _, _, _, _, _) => {
                        h34
                    }
                };
                let a36 = 4u64;
                nat_eq(a35, a36)
            } else {
                false
            } {
                f11(v11.clone())?
            } else {
                false
            } {
                f12(v6.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                f12(v7.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                let a39 = match v5.clone() {
                    Adt1::C0(_, h37, _, _, _, _, _, _, _, _, _) => {
                        h37
                    }
                };
                let a40 = match v6.clone() {
                    Adt1::C0(_, h38, _, _, _, _, _, _, _, _, _) => {
                        h38
                    }
                };
                nat_eq(a39, a40)
            } else {
                false
            } {
                let a43 = match v6.clone() {
                    Adt1::C0(_, _, h41, _, _, _, _, _, _, _, _) => {
                        h41
                    }
                };
                let a44 = match v7.clone() {
                    Adt1::C0(_, _, h42, _, _, _, _, _, _, _, _) => {
                        h42
                    }
                };
                nat_eq(a43, a44)
            } else {
                false
            } {
                let a47 = match v11.clone() {
                    Adt1::C0(_, h45, _, _, _, _, _, _, _, _, _) => {
                        h45
                    }
                };
                let a48 = match v7.clone() {
                    Adt1::C0(_, h46, _, _, _, _, _, _, _, _, _) => {
                        h46
                    }
                };
                nat_eq(a47, a48)
            } else {
                false
            } {
                let a51 = match v11.clone() {
                    Adt1::C0(_, _, h49, _, _, _, _, _, _, _, _) => {
                        h49
                    }
                };
                let a52 = match v5.clone() {
                    Adt1::C0(_, _, _, h50, _, _, _, _, _, _, _) => {
                        h50
                    }
                };
                nat_eq(a51, a52)
            } else {
                false
            } {
                let a55 = match v11.clone() {
                    Adt1::C0(_, _, _, h53, _, _, _, _, _, _, _) => {
                        h53
                    }
                };
                let a56 = match v5.clone() {
                    Adt1::C0(_, _, h54, _, _, _, _, _, _, _, _) => {
                        h54
                    }
                };
                nat_eq(a55, a56)
            } else {
                false
            } {
                let a59 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h57, _, _, _, _, _, _) => {
                        h57
                    }
                };
                let a60 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h58, _, _, _, _, _, _) => {
                        h58
                    }
                };
                nat_eq(a59, a60)
            } else {
                false
            } {
                f13(v5.clone(), v11.clone())?
            } else {
                false
            } {
                f14(v8.clone(), v5.clone(), v12, v11.clone())?
            } else {
                false
            } {
                f15(v9.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                Ok(Adt5::C1(Adt2::C0(v5, v6, v7, v8, v9, v11, v12, f16(v2), f17(v3))))
            } else {
                Ok(Adt5::C0)
            }
        }
    }
}

pub fn f3(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt5> {
    let v8: u64 = {
        let a62 = match v2.clone() {
            Adt1::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        };
        let a63 = 1u64;
        if nat_eq(a62, a63) {
            2u64
        } else {
            4u64
        }
    };
    let a64 = v0;
    let a65 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a64, a65) {
        !v1
    } else {
        false
    } {
        let a67 = match v2.clone() {
            Adt1::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                h66
            }
        };
        let a68 = 0u64;
        if nat_eq(a67, a68) {
            true
        } else {
            let a70 = match v2.clone() {
                Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                    h69
                }
            };
            let a71 = 1u64;
            nat_eq(a70, a71)
        }
    } else {
        false
    } {
        let a73 = match v3.clone() {
            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        };
        let a74 = 26u64;
        nat_eq(a73, a74)
    } else {
        false
    } {
        let a77 = match v7.clone() {
            Adt1::C0(h75, _, _, _, _, _, _, _, _, _, _) => {
                h75
            }
        };
        let a78 = match v2.clone() {
            Adt1::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                h76
            }
        };
        nat_eq(a77, a78)
    } else {
        false
    } {
        let a80 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h79, _, _, _, _, _) => {
                h79
            }
        };
        let a81 = v8;
        nat_eq(a80, a81)
    } else {
        false
    } {
        let a83 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h82, _, _, _, _, _) => {
                h82
            }
        };
        let a84 = v8;
        nat_eq(a83, a84)
    } else {
        false
    } {
        let a87 = match v2.clone() {
            Adt1::C0(_, h85, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        };
        let a88 = match v7.clone() {
            Adt1::C0(_, h86, _, _, _, _, _, _, _, _, _) => {
                h86
            }
        };
        nat_eq(a87, a88)
    } else {
        false
    } {
        f18(v3.clone(), v7.clone())
    } else {
        false
    } {
        f19({
            let a90 = v8;
            let a91 = match v7.clone() {
                Adt1::C0(_, h89, _, _, _, _, _, _, _, _, _) => {
                    h89
                }
            };
            nat_mul(a90, a91)?
        }, match v7.clone() {
            Adt1::C0(_, _, h92, _, _, _, _, _, _, _, _) => {
                h92
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
                h93
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h94, _, _, _, _, _, _) => {
                h94
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h95, _, _, _, _) => {
                h95
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h96, _, _, _) => {
                h96
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h97, _, _) => {
                h97
            }
        })?
    } else {
        false
    } {
        f13(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f13(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f20(match v3.clone() {
            Adt1::C0(_, _, h98, _, _, _, _, _, _, _, _) => {
                h98
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h99, _, _, _, _, _, _, _, _) => {
                h99
            }
        })
    } else {
        false
    } {
        f20(match v3.clone() {
            Adt1::C0(_, _, _, h100, _, _, _, _, _, _, _) => {
                h100
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h101, _, _, _, _, _, _, _) => {
                h101
            }
        })
    } else {
        false
    } {
        f20(match v3.clone() {
            Adt1::C0(_, _, _, _, h102, _, _, _, _, _, _) => {
                h102
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h103, _, _, _, _, _, _) => {
                h103
            }
        })
    } else {
        false
    } {
        let m104 = v4.clone();
        match m104 {
            None => {
                true
            }
            Some(v9) => {
                let a106 = match v9.clone() {
                    Adt1::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
                        h105
                    }
                };
                let a107 = 0u64;
                if if nat_eq(a106, a107) {
                    f18(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f13(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m108 = v5.clone();
        match m108 {
            None => {
                true
            }
            Some(v10) => {
                let a110 = match v10.clone() {
                    Adt1::C0(h109, _, _, _, _, _, _, _, _, _, _) => {
                        h109
                    }
                };
                let a111 = 26u64;
                if if if if nat_eq(a110, a111) {
                    let a113 = match v10.clone() {
                        Adt1::C0(_, _, _, h112, _, _, _, _, _, _, _) => {
                            h112
                        }
                    };
                    let a114 = 1u64;
                    nat_eq(a113, a114)
                } else {
                    false
                } {
                    let a116 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h115, _, _, _, _, _, _) => {
                            h115
                        }
                    };
                    let a117 = 1u64;
                    nat_eq(a116, a117)
                } else {
                    false
                } {
                    f13(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m118 = v4.clone();
                    match m118 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a121 = match v11 {
                                Adt1::C0(_, h119, _, _, _, _, _, _, _, _, _) => {
                                    h119
                                }
                            };
                            let a122 = match v10 {
                                Adt1::C0(_, _, h120, _, _, _, _, _, _, _, _) => {
                                    h120
                                }
                            };
                            nat_eq(a121, a122)
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
        let m123 = v6.clone();
        match m123 {
            None => {
                true
            }
            Some(v12) => {
                let a125 = match v12.clone() {
                    Adt1::C0(h124, _, _, _, _, _, _, _, _, _, _) => {
                        h124
                    }
                };
                let a126 = 26u64;
                if if nat_eq(a125, a126) {
                    f18(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f13(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt5::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a127 = v8;
            let a128 = 2u64;
            if nat_eq(a127, a128) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m129 = v5;
            match m129 {
                None => {
                    let m130 = v4;
                    match m130 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h131, _, _, _, _, _, _, _, _, _) => {
                                    h131
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h132, _, _, _, _, _, _, _, _, _) => {
                                    h132
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h133, _, _, _, _, _, _, _, _) => {
                            h133
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
        Adt4::C0(h134, _, _, _, _, _, _, _, _) => {
            h134
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h135, _, _, _, _, _, _, _) => {
            h135
        }
    };
    let a137 = v1;
    let a138 = match v5.clone() {
        Adt1::C0(_, _, h136, _, _, _, _, _, _, _, _) => {
            h136
        }
    };
    if if if nat_lt(a137, a138) {
        let a140 = v2;
        let a141 = match v5.clone() {
            Adt1::C0(_, _, _, h139, _, _, _, _, _, _, _) => {
                h139
            }
        };
        nat_lt(a140, a141)
    } else {
        false
    } {
        let a143 = v3;
        let a144 = match v5.clone() {
            Adt1::C0(_, _, _, _, h142, _, _, _, _, _, _) => {
                h142
            }
        };
        nat_lt(a143, a144)
    } else {
        false
    } {
        Ok(Adt5::C6(f21(v4.clone(), {
            let a146 = v1;
            let a147 = match v4.clone() {
                Adt1::C0(_, _, h145, _, _, _, _, _, _, _, _) => {
                    h145
                }
            };
            let a148 = 0u64;
            nat_rem(a146, a147, a148)
        }, {
            let a150 = v2;
            let a151 = match v4.clone() {
                Adt1::C0(_, _, _, h149, _, _, _, _, _, _, _) => {
                    h149
                }
            };
            let a152 = 0u64;
            nat_rem(a150, a151, a152)
        }, {
            let a154 = v3;
            let a155 = match v4 {
                Adt1::C0(_, _, _, _, h153, _, _, _, _, _, _) => {
                    h153
                }
            };
            let a156 = 0u64;
            nat_rem(a154, a155, a156)
        })?, f21(v5, v1, v2, v3)?, {
            let m158 = match v0.clone() {
                Adt4::C0(_, _, h157, _, _, _, _, _, _) => {
                    h157
                }
            };
            match m158 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f21(v6, v1, v2, v3)?
                }
            }
        }, {
            let m160 = match v0.clone() {
                Adt4::C0(_, _, _, _, h159, _, _, _, _) => {
                    h159
                }
            };
            match m160 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f21(v7, v1, v2, v3)?
                }
            }
        }, f21(match v0 {
            Adt4::C0(_, _, _, _, _, h161, _, _, _) => {
                h161
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f5(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a162 = v1;
    let a163 = 0u64;
    if nat_eq(a162, a163) {
        let a167 = v2;
        let a168 = {
            let m166 = match v0.clone() {
                Adt4::C0(_, h165, _, _, _, _, _, _, _) => {
                    h165
                }
            };
            match m166 {
                Adt1::C0(_, h164, _, _, _, _, _, _, _, _, _) => {
                    h164
                }
            }
        };
        if nat_lt(a167, a168) {
            Ok(Adt5::C7({
                let a172 = v2;
                let a173 = {
                    let m171 = match v0 {
                        Adt4::C0(_, h170, _, _, _, _, _, _, _) => {
                            h170
                        }
                    };
                    match m171 {
                        Adt1::C0(_, _, _, _, _, h169, _, _, _, _, _) => {
                            h169
                        }
                    }
                };
                nat_mul(a172, a173)?
            }))
        } else {
            Ok(Adt5::C8)
        }
    } else {
        let a174 = v1;
        let a175 = 1u64;
        if nat_eq(a174, a175) {
            let m177 = match v0 {
                Adt4::C0(_, _, _, _, h176, _, _, _, _) => {
                    h176
                }
            };
            match m177 {
                None => {
                    Ok(Adt5::C8)
                }
                Some(v4) => {
                    let a179 = v2;
                    let a180 = match v4.clone() {
                        Adt1::C0(_, h178, _, _, _, _, _, _, _, _, _) => {
                            h178
                        }
                    };
                    if nat_lt(a179, a180) {
                        Ok(Adt5::C7({
                            let a182 = v2;
                            let a183 = match v4 {
                                Adt1::C0(_, _, _, _, _, h181, _, _, _, _, _) => {
                                    h181
                                }
                            };
                            nat_mul(a182, a183)?
                        }))
                    } else {
                        Ok(Adt5::C8)
                    }
                }
            }
        } else {
            let a184 = v1;
            let a185 = 2u64;
            if nat_eq(a184, a185) {
                let a186 = v2;
                let a187 = 2147483648u64;
                if if nat_lt(a186, a187) {
                    let a189 = v2;
                    let a190 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h188) => {
                            h188
                        }
                    };
                    nat_lt(a189, a190)
                } else {
                    false
                } {
                    let m192 = match v0 {
                        Adt4::C0(_, _, h191, _, _, _, _, _, _) => {
                            h191
                        }
                    };
                    match m192 {
                        None => {
                            Ok(Adt5::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt5::C7({
                                let a194 = v2;
                                let a195 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h193, _, _, _, _, _) => {
                                        h193
                                    }
                                };
                                nat_mul(a194, a195)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt5::C8)
                }
            } else {
                let a196 = v1;
                let a197 = 3u64;
                if nat_eq(a196, a197) {
                    let m199 = match v0 {
                        Adt4::C0(_, _, _, h198, _, _, _, _, _) => {
                            h198
                        }
                    };
                    match m199 {
                        None => {
                            Ok(Adt5::C8)
                        }
                        Some(v6) => {
                            let a201 = v2;
                            let a202 = match v6.clone() {
                                Adt1::C0(_, _, h200, _, _, _, _, _, _, _, _) => {
                                    h200
                                }
                            };
                            if if nat_lt(a201, a202) {
                                let a204 = v3;
                                let a205 = match v6.clone() {
                                    Adt1::C0(_, h203, _, _, _, _, _, _, _, _, _) => {
                                        h203
                                    }
                                };
                                nat_lt(a204, a205)
                            } else {
                                false
                            } {
                                Ok(Adt5::C7({
                                    let a212 = {
                                        let a207 = v2;
                                        let a208 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h206, _, _, _, _) => {
                                                h206
                                            }
                                        };
                                        nat_mul(a207, a208)?
                                    };
                                    let a213 = {
                                        let a210 = v3;
                                        let a211 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h209, _, _, _, _, _) => {
                                                h209
                                            }
                                        };
                                        nat_mul(a210, a211)?
                                    };
                                    nat_add(a212, a213)?
                                }))
                            } else {
                                Ok(Adt5::C8)
                            }
                        }
                    }
                } else {
                    let a214 = v1;
                    let a215 = 4u64;
                    if nat_eq(a214, a215) {
                        let a216 = v2;
                        let a217 = 2147483648u64;
                        if if nat_lt(a216, a217) {
                            let a221 = v2;
                            let a222 = {
                                let m220 = match v0.clone() {
                                    Adt4::C0(h219, _, _, _, _, _, _, _, _) => {
                                        h219
                                    }
                                };
                                match m220 {
                                    Adt1::C0(_, h218, _, _, _, _, _, _, _, _, _) => {
                                        h218
                                    }
                                }
                            };
                            nat_lt(a221, a222)
                        } else {
                            false
                        } {
                            Ok(Adt5::C7({
                                let a224 = v2;
                                let a225 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h223, _, _) => {
                                        h223
                                    }
                                };
                                nat_mul(a224, a225)?
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
        let a226 = v0;
        let a227 = 43u64;
        if if if if if if if if nat_eq(a226, a227) {
            let a229 = match v4.clone() {
                Adt1::C0(h228, _, _, _, _, _, _, _, _, _, _) => {
                    h228
                }
            };
            let a230 = 0u64;
            if nat_eq(a229, a230) {
                f22(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a232 = match v7.clone() {
                Adt1::C0(h231, _, _, _, _, _, _, _, _, _, _) => {
                    h231
                }
            };
            let a233 = 0u64;
            if nat_eq(a232, a233) {
                f22(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f23(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f13(v4.clone(), v7.clone())? {
                true
            } else {
                f24(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f25(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f26(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f10(v3, match v4.clone() {
                Adt1::C0(_, _, _, h234, _, _, _, _, _, _, _) => {
                    h234
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt5::C3(Adt3::C0(v4, v5, v7, v2, f16(v3))))
        } else {
            Ok(Adt5::C0)
        }
    }
}

pub fn f7(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h235, _, _, _, _) => {
            h235
        }
    };
    let a237 = v1;
    let a238 = match v4.clone() {
        Adt1::C0(_, _, h236, _, _, _, _, _, _, _, _) => {
            h236
        }
    };
    if if if nat_lt(a237, a238) {
        let a240 = v2;
        let a241 = match v4.clone() {
            Adt1::C0(_, _, _, h239, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_lt(a240, a241)
    } else {
        false
    } {
        let a243 = v3;
        let a244 = match v4.clone() {
            Adt1::C0(_, _, _, _, h242, _, _, _, _, _, _) => {
                h242
            }
        };
        nat_lt(a243, a244)
    } else {
        false
    } {
        let v6: u64 = {
            let m246 = match v0.clone() {
                Adt3::C0(_, h245, _, _, _) => {
                    h245
                }
            };
            match m246 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f21(v5.clone(), v1, {
                        let a248 = v2;
                        let a249 = match v5.clone() {
                            Adt1::C0(_, _, _, h247, _, _, _, _, _, _, _) => {
                                h247
                            }
                        };
                        let a250 = 0u64;
                        nat_rem(a248, a249, a250)
                    }, {
                        let a252 = v3;
                        let a253 = match v5 {
                            Adt1::C0(_, _, _, _, h251, _, _, _, _, _, _) => {
                                h251
                            }
                        };
                        let a254 = 0u64;
                        nat_rem(a252, a253, a254)
                    })?
                }
            }
        };
        Ok(Adt5::C4(f21(v4, v1, v2, v3)?, v6, {
            let a255 = v2;
            let a256 = 4u64;
            nat_mul(a255, a256)?
        }, f21(match v0 {
            Adt3::C0(_, _, h257, _, _) => {
                h257
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f8(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a261 = v1;
    let a262 = {
        let m260 = match v0.clone() {
            Adt2::C0(h259, _, _, _, _, _, _, _, _) => {
                h259
            }
        };
        match m260 {
            Adt1::C0(_, _, h258, _, _, _, _, _, _, _, _) => {
                h258
            }
        }
    };
    if if if nat_lt(a261, a262) {
        let a266 = v2;
        let a267 = {
            let m265 = match v0.clone() {
                Adt2::C0(h264, _, _, _, _, _, _, _, _) => {
                    h264
                }
            };
            match m265 {
                Adt1::C0(_, _, _, h263, _, _, _, _, _, _, _) => {
                    h263
                }
            }
        };
        nat_lt(a266, a267)
    } else {
        false
    } {
        let a271 = v3;
        let a272 = {
            let m270 = match v0.clone() {
                Adt2::C0(h269, _, _, _, _, _, _, _, _) => {
                    h269
                }
            };
            match m270 {
                Adt1::C0(_, _, _, _, h268, _, _, _, _, _, _) => {
                    h268
                }
            }
        };
        nat_lt(a271, a272)
    } else {
        false
    } {
        Ok(Adt5::C2(f21(match v0.clone() {
            Adt2::C0(h273, _, _, _, _, _, _, _, _) => {
                h273
            }
        }, v1, v2, v3)?, f21(match v0.clone() {
            Adt2::C0(_, h274, _, _, _, _, _, _, _) => {
                h274
            }
        }, 0u64, {
            let a284 = v2;
            let a285 = {
                let a281 = {
                    let m277 = match v0.clone() {
                        Adt2::C0(h276, _, _, _, _, _, _, _, _) => {
                            h276
                        }
                    };
                    match m277 {
                        Adt1::C0(_, _, _, h275, _, _, _, _, _, _, _) => {
                            h275
                        }
                    }
                };
                let a282 = {
                    let m280 = match v0.clone() {
                        Adt2::C0(_, h279, _, _, _, _, _, _, _) => {
                            h279
                        }
                    };
                    match m280 {
                        Adt1::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                            h278
                        }
                    }
                };
                let a283 = 0u64;
                nat_quot(a281, a282, a283)
            };
            let a286 = 0u64;
            nat_quot(a284, a285, a286)
        }, {
            let a296 = v3;
            let a297 = {
                let a293 = {
                    let m289 = match v0.clone() {
                        Adt2::C0(h288, _, _, _, _, _, _, _, _) => {
                            h288
                        }
                    };
                    match m289 {
                        Adt1::C0(_, _, _, _, h287, _, _, _, _, _, _) => {
                            h287
                        }
                    }
                };
                let a294 = {
                    let m292 = match v0.clone() {
                        Adt2::C0(_, h291, _, _, _, _, _, _, _) => {
                            h291
                        }
                    };
                    match m292 {
                        Adt1::C0(_, _, _, _, h290, _, _, _, _, _, _) => {
                            h290
                        }
                    }
                };
                let a295 = 0u64;
                nat_quot(a293, a294, a295)
            };
            let a298 = 0u64;
            nat_quot(a296, a297, a298)
        })?, f21(match v0.clone() {
            Adt2::C0(_, _, h299, _, _, _, _, _, _) => {
                h299
            }
        }, 0u64, {
            let a309 = v2;
            let a310 = {
                let a306 = {
                    let m302 = match v0.clone() {
                        Adt2::C0(h301, _, _, _, _, _, _, _, _) => {
                            h301
                        }
                    };
                    match m302 {
                        Adt1::C0(_, _, _, h300, _, _, _, _, _, _, _) => {
                            h300
                        }
                    }
                };
                let a307 = {
                    let m305 = match v0.clone() {
                        Adt2::C0(_, _, h304, _, _, _, _, _, _) => {
                            h304
                        }
                    };
                    match m305 {
                        Adt1::C0(_, _, _, h303, _, _, _, _, _, _, _) => {
                            h303
                        }
                    }
                };
                let a308 = 0u64;
                nat_quot(a306, a307, a308)
            };
            let a311 = 0u64;
            nat_quot(a309, a310, a311)
        }, {
            let a321 = v3;
            let a322 = {
                let a318 = {
                    let m314 = match v0.clone() {
                        Adt2::C0(h313, _, _, _, _, _, _, _, _) => {
                            h313
                        }
                    };
                    match m314 {
                        Adt1::C0(_, _, _, _, h312, _, _, _, _, _, _) => {
                            h312
                        }
                    }
                };
                let a319 = {
                    let m317 = match v0.clone() {
                        Adt2::C0(_, _, h316, _, _, _, _, _, _) => {
                            h316
                        }
                    };
                    match m317 {
                        Adt1::C0(_, _, _, _, h315, _, _, _, _, _, _) => {
                            h315
                        }
                    }
                };
                let a320 = 0u64;
                nat_quot(a318, a319, a320)
            };
            let a323 = 0u64;
            nat_quot(a321, a322, a323)
        })?, {
            let m325 = match v0.clone() {
                Adt2::C0(_, _, _, h324, _, _, _, _, _) => {
                    h324
                }
            };
            match m325 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f21(v4.clone(), v1, {
                        let a327 = v2;
                        let a328 = match v4.clone() {
                            Adt1::C0(_, _, _, h326, _, _, _, _, _, _, _) => {
                                h326
                            }
                        };
                        let a329 = 0u64;
                        nat_rem(a327, a328, a329)
                    }, {
                        let a331 = v3;
                        let a332 = match v4 {
                            Adt1::C0(_, _, _, _, h330, _, _, _, _, _, _) => {
                                h330
                            }
                        };
                        let a333 = 0u64;
                        nat_rem(a331, a332, a333)
                    })?
                }
            }
        }, {
            let a334 = v2;
            let a335 = 4u64;
            nat_mul(a334, a335)?
        }, f21(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h336, _, _, _) => {
                h336
            }
        }, v2, v1, v3)?, {
            let m339 = match v0.clone() {
                Adt2::C0(_, h338, _, _, _, _, _, _, _) => {
                    h338
                }
            };
            match m339 {
                Adt1::C0(_, _, _, _, _, _, h337, _, _, _, _) => {
                    h337
                }
            }
        }, {
            let m342 = match v0.clone() {
                Adt2::C0(_, _, h341, _, _, _, _, _, _) => {
                    h341
                }
            };
            match m342 {
                Adt1::C0(_, _, _, _, _, _, h340, _, _, _, _) => {
                    h340
                }
            }
        }, f27({
            let m345 = match v0 {
                Adt2::C0(_, h344, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            match m345 {
                Adt1::C0(h343, _, _, _, _, _, _, _, _, _, _) => {
                    h343
                }
            }
        })))
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f9(v0: Option<Adt1>, v1: Option<Adt1>, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Option<u64>> {
    let m346 = v0;
    match m346 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h347, _, _, _, _, _, _, _, _) => {
                    h347
                }
            }))
        }
        Some(v6) => {
            let m348 = v1;
            match m348 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a350 = match v6.clone() {
                        Adt1::C0(h349, _, _, _, _, _, _, _, _, _, _) => {
                            h349
                        }
                    };
                    let a351 = 26u64;
                    if if if if if if if if if nat_eq(a350, a351) {
                        f22(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a354 = match v6.clone() {
                            Adt1::C0(_, h352, _, _, _, _, _, _, _, _, _) => {
                                h352
                            }
                        };
                        let a355 = match v7 {
                            Adt1::C0(_, h353, _, _, _, _, _, _, _, _, _) => {
                                h353
                            }
                        };
                        nat_eq(a354, a355)
                    } else {
                        false
                    } {
                        let a358 = match v6.clone() {
                            Adt1::C0(_, _, h356, _, _, _, _, _, _, _, _) => {
                                h356
                            }
                        };
                        let a359 = match v2 {
                            Adt1::C0(_, _, _, _, h357, _, _, _, _, _, _) => {
                                h357
                            }
                        };
                        nat_eq(a358, a359)
                    } else {
                        false
                    } {
                        let a361 = match v6.clone() {
                            Adt1::C0(_, _, _, h360, _, _, _, _, _, _, _) => {
                                h360
                            }
                        };
                        let a362 = 1u64;
                        nat_eq(a361, a362)
                    } else {
                        false
                    } {
                        let a364 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h363, _, _, _, _, _, _) => {
                                h363
                            }
                        };
                        let a365 = 1u64;
                        nat_eq(a364, a365)
                    } else {
                        false
                    } {
                        let a367 = match v3 {
                            Adt1::C0(_, _, _, _, h366, _, _, _, _, _, _) => {
                                h366
                            }
                        };
                        let a368 = 1u64;
                        nat_eq(a367, a368)
                    } else {
                        false
                    } {
                        let a370 = match v4 {
                            Adt1::C0(_, _, _, _, h369, _, _, _, _, _, _) => {
                                h369
                            }
                        };
                        let a371 = 1u64;
                        nat_eq(a370, a371)
                    } else {
                        false
                    } {
                        f13(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h372, _, _, _, _, _, _, _, _, _) => {
                                h372
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a373 = v0;
    let a374 = 0u64;
    if if nat_eq(a373, a374) {
        true
    } else {
        let a375 = 2147483648u64;
        let a376 = v0;
        if nat_le(a375, a376) {
            let a377 = v0;
            let a378 = 4286578688u64;
            nat_lt(a377, a378)
        } else {
            false
        }
    } {
        true
    } else {
        let a379 = v0;
        let a380 = 2139095040u64;
        if if nat_lt(a379, a380) {
            let a381 = v1;
            let a382 = 4294967295u64;
            nat_le(a381, a382)
        } else {
            false
        } {
            let m383 = v2;
            match m383 {
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

pub fn f11(v0: Adt1) -> R<bool> {
    let a385 = match v0.clone() {
        Adt1::C0(h384, _, _, _, _, _, _, _, _, _, _) => {
            h384
        }
    };
    let a386 = 0u64;
    if if if if nat_eq(a385, a386) {
        let a388 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h387, _, _, _, _, _) => {
                h387
            }
        };
        let a389 = 4u64;
        nat_eq(a388, a389)
    } else {
        false
    } {
        let a395 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h390, _, _, _, _) => {
                h390
            }
        };
        let a396 = {
            let a393 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h391, _, _, _, _, _) => {
                    h391
                }
            };
            let a394 = match v0.clone() {
                Adt1::C0(_, h392, _, _, _, _, _, _, _, _, _) => {
                    h392
                }
            };
            nat_mul(a393, a394)?
        };
        nat_eq(a395, a396)
    } else {
        false
    } {
        let a402 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h397, _, _, _) => {
                h397
            }
        };
        let a403 = {
            let a400 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h398, _, _, _, _) => {
                    h398
                }
            };
            let a401 = match v0.clone() {
                Adt1::C0(_, _, h399, _, _, _, _, _, _, _, _) => {
                    h399
                }
            };
            nat_mul(a400, a401)?
        };
        nat_eq(a402, a403)
    } else {
        false
    } {
        let a409 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h404, _, _) => {
                h404
            }
        };
        let a410 = {
            let a407 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h405, _, _, _) => {
                    h405
                }
            };
            let a408 = match v0 {
                Adt1::C0(_, _, _, h406, _, _, _, _, _, _, _) => {
                    h406
                }
            };
            nat_mul(a407, a408)?
        };
        Ok(nat_eq(a409, a410))
    } else {
        Ok(false)
    }
}

pub fn f12(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f28(match v0.clone() {
        Adt1::C0(h411, _, _, _, _, _, _, _, _, _, _) => {
            h411
        }
    }, match v0.clone() {
        Adt1::C0(_, h412, _, _, _, _, _, _, _, _, _) => {
            h412
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h413, _, _, _, _, _) => {
            h413
        }
    }) {
        f20(match v1.clone() {
            Adt1::C0(_, _, _, h414, _, _, _, _, _, _, _) => {
                h414
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h415, _, _, _, _, _, _, _) => {
                h415
            }
        })
    } else {
        false
    } {
        f20(match v1 {
            Adt1::C0(_, _, _, _, h416, _, _, _, _, _, _) => {
                h416
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h417, _, _, _, _, _, _) => {
                h417
            }
        })
    } else {
        false
    } {
        f13(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f13(v0: Adt1, v1: Adt1) -> R<bool> {
    let a419 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h418, _) => {
            h418
        }
    };
    let a420 = 0u64;
    if nat_eq(a419, a420) {
        Ok(true)
    } else {
        let a422 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h421, _) => {
                h421
            }
        };
        let a423 = 0u64;
        if nat_eq(a422, a423) {
            Ok(true)
        } else {
            let a425 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h424) => {
                    h424
                }
            };
            let a426 = 0u64;
            if nat_eq(a425, a426) {
                Ok(true)
            } else {
                let a428 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h427) => {
                        h427
                    }
                };
                let a429 = 0u64;
                if nat_eq(a428, a429) {
                    Ok(true)
                } else {
                    let a435 = {
                        let a432 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h430) => {
                                h430
                            }
                        };
                        let a433 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h431, _) => {
                                h431
                            }
                        };
                        nat_add(a432, a433)?
                    };
                    let a436 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h434) => {
                            h434
                        }
                    };
                    if nat_le(a435, a436) {
                        Ok(true)
                    } else {
                        let a442 = {
                            let a439 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h437) => {
                                    h437
                                }
                            };
                            let a440 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h438, _) => {
                                    h438
                                }
                            };
                            nat_add(a439, a440)?
                        };
                        let a443 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h441) => {
                                h441
                            }
                        };
                        Ok(nat_le(a442, a443))
                    }
                }
            }
        }
    }
}

pub fn f14(v0: Option<Adt1>, v1: Adt1, v2: u64, v3: Adt1) -> R<bool> {
    let m444 = v0;
    match m444 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a446 = match v4.clone() {
                Adt1::C0(h445, _, _, _, _, _, _, _, _, _, _) => {
                    h445
                }
            };
            let a447 = 1u64;
            if if if if if if nat_eq(a446, a447) {
                let a449 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h448, _, _, _, _, _) => {
                        h448
                    }
                };
                let a450 = 2u64;
                nat_eq(a449, a450)
            } else {
                false
            } {
                let a452 = v2;
                let a453 = match v4.clone() {
                    Adt1::C0(_, h451, _, _, _, _, _, _, _, _, _) => {
                        h451
                    }
                };
                nat_le(a452, a453)
            } else {
                false
            } {
                let a456 = match v1.clone() {
                    Adt1::C0(_, _, h454, _, _, _, _, _, _, _, _) => {
                        h454
                    }
                };
                let a457 = match v4.clone() {
                    Adt1::C0(_, _, h455, _, _, _, _, _, _, _, _) => {
                        h455
                    }
                };
                nat_le(a456, a457)
            } else {
                false
            } {
                f20(match v1.clone() {
                    Adt1::C0(_, _, _, h458, _, _, _, _, _, _, _) => {
                        h458
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h459, _, _, _, _, _, _, _) => {
                        h459
                    }
                })
            } else {
                false
            } {
                f20(match v1 {
                    Adt1::C0(_, _, _, _, h460, _, _, _, _, _, _) => {
                        h460
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h461, _, _, _, _, _, _) => {
                        h461
                    }
                })
            } else {
                false
            } {
                f13(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f15(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m462 = v0;
    match m462 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a464 = match v3.clone() {
                Adt1::C0(h463, _, _, _, _, _, _, _, _, _, _) => {
                    h463
                }
            };
            let a465 = 0u64;
            if if if if if if nat_eq(a464, a465) {
                let a467 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h466, _, _, _, _, _) => {
                        h466
                    }
                };
                let a468 = 4u64;
                nat_eq(a467, a468)
            } else {
                false
            } {
                let a471 = match v3.clone() {
                    Adt1::C0(_, h469, _, _, _, _, _, _, _, _, _) => {
                        h469
                    }
                };
                let a472 = match v1 {
                    Adt1::C0(_, _, _, h470, _, _, _, _, _, _, _) => {
                        h470
                    }
                };
                nat_eq(a471, a472)
            } else {
                false
            } {
                let a474 = match v3.clone() {
                    Adt1::C0(_, _, h473, _, _, _, _, _, _, _, _) => {
                        h473
                    }
                };
                let a475 = 1u64;
                nat_eq(a474, a475)
            } else {
                false
            } {
                let a477 = match v3.clone() {
                    Adt1::C0(_, _, _, h476, _, _, _, _, _, _, _) => {
                        h476
                    }
                };
                let a478 = 1u64;
                nat_eq(a477, a478)
            } else {
                false
            } {
                let a480 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h479, _, _, _, _, _, _) => {
                        h479
                    }
                };
                let a481 = 1u64;
                nat_eq(a480, a481)
            } else {
                false
            } {
                f13(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f16(v0: u64) -> Option<u64> {
    let a482 = 0u64;
    let a483 = v0;
    if if nat_lt(a482, a483) {
        let a484 = v0;
        let a485 = 2139095040u64;
        nat_lt(a484, a485)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f17(v0: u64) -> Option<u64> {
    let a486 = v0;
    let a487 = 0u64;
    if if nat_eq(a486, a487) {
        true
    } else {
        let a488 = v0;
        let a489 = 2147483648u64;
        nat_eq(a488, a489)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> bool {
    let a492 = match v0.clone() {
        Adt1::C0(_, _, h490, _, _, _, _, _, _, _, _) => {
            h490
        }
    };
    let a493 = match v1.clone() {
        Adt1::C0(_, _, h491, _, _, _, _, _, _, _, _) => {
            h491
        }
    };
    if if nat_eq(a492, a493) {
        let a496 = match v0.clone() {
            Adt1::C0(_, _, _, h494, _, _, _, _, _, _, _) => {
                h494
            }
        };
        let a497 = match v1.clone() {
            Adt1::C0(_, _, _, h495, _, _, _, _, _, _, _) => {
                h495
            }
        };
        nat_eq(a496, a497)
    } else {
        false
    } {
        let a500 = match v0 {
            Adt1::C0(_, _, _, _, h498, _, _, _, _, _, _) => {
                h498
            }
        };
        let a501 = match v1 {
            Adt1::C0(_, _, _, _, h499, _, _, _, _, _, _) => {
                h499
            }
        };
        nat_eq(a500, a501)
    } else {
        false
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f29(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f29(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f29(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f29(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f29(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f29(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f20(v0: u64, v1: u64) -> bool {
    let a502 = 0u64;
    let a503 = v1;
    if nat_lt(a502, a503) {
        let a507 = {
            let a504 = v0;
            let a505 = v1;
            let a506 = 0u64;
            nat_rem(a504, a505, a506)
        };
        let a508 = 0u64;
        nat_eq(a507, a508)
    } else {
        false
    }
}

pub fn f21(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a520 = {
        let a515 = {
            let a510 = v1;
            let a511 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h509, _, _, _, _) => {
                    h509
                }
            };
            nat_mul(a510, a511)?
        };
        let a516 = {
            let a513 = v2;
            let a514 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h512, _, _, _) => {
                    h512
                }
            };
            nat_mul(a513, a514)?
        };
        nat_add(a515, a516)?
    };
    let a521 = {
        let a518 = v3;
        let a519 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h517, _, _) => {
                h517
            }
        };
        nat_mul(a518, a519)?
    };
    nat_add(a520, a521)
}

pub fn f22(v0: Adt1, v1: u64) -> R<bool> {
    let a523 = match v0.clone() {
        Adt1::C0(_, h522, _, _, _, _, _, _, _, _, _) => {
            h522
        }
    };
    let a524 = 1u64;
    if if if if nat_eq(a523, a524) {
        true
    } else {
        let a526 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h525, _, _, _, _, _) => {
                h525
            }
        };
        let a527 = v1;
        nat_eq(a526, a527)
    } {
        let a529 = match v0.clone() {
            Adt1::C0(_, _, h528, _, _, _, _, _, _, _, _) => {
                h528
            }
        };
        let a530 = 1u64;
        if nat_eq(a529, a530) {
            true
        } else {
            let a535 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h531, _, _, _, _) => {
                    h531
                }
            };
            let a536 = {
                let a533 = v1;
                let a534 = match v0.clone() {
                    Adt1::C0(_, h532, _, _, _, _, _, _, _, _, _) => {
                        h532
                    }
                };
                nat_mul(a533, a534)?
            };
            nat_eq(a535, a536)
        }
    } else {
        false
    } {
        let a538 = match v0.clone() {
            Adt1::C0(_, _, _, h537, _, _, _, _, _, _, _) => {
                h537
            }
        };
        let a539 = 1u64;
        if nat_eq(a538, a539) {
            true
        } else {
            let a547 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h540, _, _, _) => {
                    h540
                }
            };
            let a548 = {
                let a545 = {
                    let a542 = v1;
                    let a543 = match v0.clone() {
                        Adt1::C0(_, h541, _, _, _, _, _, _, _, _, _) => {
                            h541
                        }
                    };
                    nat_mul(a542, a543)?
                };
                let a546 = match v0.clone() {
                    Adt1::C0(_, _, h544, _, _, _, _, _, _, _, _) => {
                        h544
                    }
                };
                nat_mul(a545, a546)?
            };
            nat_eq(a547, a548)
        }
    } else {
        false
    } {
        let a550 = match v0.clone() {
            Adt1::C0(_, _, _, _, h549, _, _, _, _, _, _) => {
                h549
            }
        };
        let a551 = 1u64;
        if nat_eq(a550, a551) {
            Ok(true)
        } else {
            let a562 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h552, _, _) => {
                    h552
                }
            };
            let a563 = {
                let a560 = {
                    let a557 = {
                        let a554 = v1;
                        let a555 = match v0.clone() {
                            Adt1::C0(_, h553, _, _, _, _, _, _, _, _, _) => {
                                h553
                            }
                        };
                        nat_mul(a554, a555)?
                    };
                    let a558 = match v0.clone() {
                        Adt1::C0(_, _, h556, _, _, _, _, _, _, _, _) => {
                            h556
                        }
                    };
                    nat_mul(a557, a558)?
                };
                let a561 = match v0 {
                    Adt1::C0(_, _, _, h559, _, _, _, _, _, _, _) => {
                        h559
                    }
                };
                nat_mul(a560, a561)?
            };
            Ok(nat_eq(a562, a563))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    let a566 = match v0.clone() {
        Adt1::C0(_, h564, _, _, _, _, _, _, _, _, _) => {
            h564
        }
    };
    let a567 = match v1.clone() {
        Adt1::C0(_, h565, _, _, _, _, _, _, _, _, _) => {
            h565
        }
    };
    if if if nat_eq(a566, a567) {
        let a570 = match v0.clone() {
            Adt1::C0(_, _, h568, _, _, _, _, _, _, _, _) => {
                h568
            }
        };
        let a571 = match v1.clone() {
            Adt1::C0(_, _, h569, _, _, _, _, _, _, _, _) => {
                h569
            }
        };
        nat_eq(a570, a571)
    } else {
        false
    } {
        let a574 = match v0.clone() {
            Adt1::C0(_, _, _, h572, _, _, _, _, _, _, _) => {
                h572
            }
        };
        let a575 = match v1.clone() {
            Adt1::C0(_, _, _, h573, _, _, _, _, _, _, _) => {
                h573
            }
        };
        nat_eq(a574, a575)
    } else {
        false
    } {
        let a578 = match v0 {
            Adt1::C0(_, _, _, _, h576, _, _, _, _, _, _) => {
                h576
            }
        };
        let a579 = match v1 {
            Adt1::C0(_, _, _, _, h577, _, _, _, _, _, _) => {
                h577
            }
        };
        nat_eq(a578, a579)
    } else {
        false
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f23(v0.clone(), v1.clone()) {
        let a582 = match v0.clone() {
            Adt1::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                h580
            }
        };
        let a583 = match v1.clone() {
            Adt1::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                h581
            }
        };
        nat_eq(a582, a583)
    } else {
        false
    } {
        let a586 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h584) => {
                h584
            }
        };
        let a587 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h585) => {
                h585
            }
        };
        nat_eq(a586, a587)
    } else {
        false
    } {
        let a590 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h588, _, _, _, _, _) => {
                h588
            }
        };
        let a591 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h589, _, _, _, _, _) => {
                h589
            }
        };
        nat_eq(a590, a591)
    } else {
        false
    } {
        let a594 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h592, _, _, _, _) => {
                h592
            }
        };
        let a595 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h593, _, _, _, _) => {
                h593
            }
        };
        nat_eq(a594, a595)
    } else {
        false
    } {
        let a598 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h596, _, _, _) => {
                h596
            }
        };
        let a599 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h597, _, _, _) => {
                h597
            }
        };
        nat_eq(a598, a599)
    } else {
        false
    } {
        let a602 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h600, _, _) => {
                h600
            }
        };
        let a603 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h601, _, _) => {
                h601
            }
        };
        nat_eq(a602, a603)
    } else {
        false
    }
}

pub fn f25(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m604 = v0;
    match m604 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a606 = match v3.clone() {
                Adt1::C0(h605, _, _, _, _, _, _, _, _, _, _) => {
                    h605
                }
            };
            let a607 = 0u64;
            if if if if if if if nat_eq(a606, a607) {
                true
            } else {
                let a609 = match v3.clone() {
                    Adt1::C0(h608, _, _, _, _, _, _, _, _, _, _) => {
                        h608
                    }
                };
                let a610 = 1u64;
                nat_eq(a609, a610)
            } {
                f22(v3.clone(), {
                    let a612 = match v3.clone() {
                        Adt1::C0(h611, _, _, _, _, _, _, _, _, _, _) => {
                            h611
                        }
                    };
                    let a613 = 0u64;
                    if nat_eq(a612, a613) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a616 = match v3.clone() {
                    Adt1::C0(_, h614, _, _, _, _, _, _, _, _, _) => {
                        h614
                    }
                };
                let a617 = match v1.clone() {
                    Adt1::C0(_, h615, _, _, _, _, _, _, _, _, _) => {
                        h615
                    }
                };
                nat_eq(a616, a617)
            } else {
                false
            } {
                let a620 = match v1.clone() {
                    Adt1::C0(_, _, h618, _, _, _, _, _, _, _, _) => {
                        h618
                    }
                };
                let a621 = match v3.clone() {
                    Adt1::C0(_, _, h619, _, _, _, _, _, _, _, _) => {
                        h619
                    }
                };
                nat_le(a620, a621)
            } else {
                false
            } {
                f20(match v1.clone() {
                    Adt1::C0(_, _, _, h622, _, _, _, _, _, _, _) => {
                        h622
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h623, _, _, _, _, _, _, _) => {
                        h623
                    }
                })
            } else {
                false
            } {
                f20(match v1 {
                    Adt1::C0(_, _, _, _, h624, _, _, _, _, _, _) => {
                        h624
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h625, _, _, _, _, _, _) => {
                        h625
                    }
                })
            } else {
                false
            } {
                f13(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f26(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m626 = v0;
    match m626 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a628 = match v3.clone() {
                Adt1::C0(h627, _, _, _, _, _, _, _, _, _, _) => {
                    h627
                }
            };
            let a629 = 0u64;
            if if if if if if nat_eq(a628, a629) {
                let a631 = match v3.clone() {
                    Adt1::C0(_, h630, _, _, _, _, _, _, _, _, _) => {
                        h630
                    }
                };
                let a632 = 1u64;
                if nat_eq(a631, a632) {
                    true
                } else {
                    let a634 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h633, _, _, _, _, _) => {
                            h633
                        }
                    };
                    let a635 = 4u64;
                    nat_eq(a634, a635)
                }
            } else {
                false
            } {
                let a638 = match v3.clone() {
                    Adt1::C0(_, h636, _, _, _, _, _, _, _, _, _) => {
                        h636
                    }
                };
                let a639 = match v1 {
                    Adt1::C0(_, _, _, h637, _, _, _, _, _, _, _) => {
                        h637
                    }
                };
                nat_eq(a638, a639)
            } else {
                false
            } {
                let a641 = match v3.clone() {
                    Adt1::C0(_, _, h640, _, _, _, _, _, _, _, _) => {
                        h640
                    }
                };
                let a642 = 1u64;
                nat_eq(a641, a642)
            } else {
                false
            } {
                let a644 = match v3.clone() {
                    Adt1::C0(_, _, _, h643, _, _, _, _, _, _, _) => {
                        h643
                    }
                };
                let a645 = 1u64;
                nat_eq(a644, a645)
            } else {
                false
            } {
                let a647 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h646, _, _, _, _, _, _) => {
                        h646
                    }
                };
                let a648 = 1u64;
                nat_eq(a647, a648)
            } else {
                false
            } {
                f13(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f27(v0: u64) -> u64 {
    if f30(v0) {
        8u64
    } else {
        if f31(v0) {
            9u64
        } else {
            f32(v0)
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> bool {
    let m649 = f33(v0);
    match m649 {
        Adt6::C0 => {
            false
        }
        Adt6::C1(v3) => {
            let a650 = v0;
            let a651 = 0u64;
            if if if if if if nat_eq(a650, a651) {
                true
            } else {
                let a652 = v0;
                let a653 = 1u64;
                nat_eq(a652, a653)
            } {
                true
            } else {
                let a654 = v0;
                let a655 = 30u64;
                nat_eq(a654, a655)
            } {
                true
            } else {
                let a656 = v0;
                let a657 = 43u64;
                nat_eq(a656, a657)
            } {
                true
            } else {
                f34(v0)
            } {
                let a659 = v2;
                let a660 = match v3.clone() {
                    Adt7::C0(_, _, h658) => {
                        h658
                    }
                };
                nat_eq(a659, a660)
            } else {
                false
            } {
                f20(v1, match v3 {
                    Adt7::C0(_, h661, _) => {
                        h661
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a662 = v1;
    let a663 = 1u64;
    if if if nat_le(a662, a663) {
        true
    } else {
        let a664 = v0;
        let a665 = v2;
        nat_le(a664, a665)
    } {
        let a666 = v3;
        let a667 = 1u64;
        if nat_le(a666, a667) {
            true
        } else {
            let a674 = {
                let a672 = v0;
                let a673 = {
                    let a670 = {
                        let a668 = v1;
                        let a669 = 1u64;
                        nat_sub(a668, a669)
                    };
                    let a671 = v2;
                    nat_mul(a670, a671)?
                };
                nat_add(a672, a673)?
            };
            let a675 = v4;
            nat_le(a674, a675)
        }
    } else {
        false
    } {
        let a676 = v5;
        let a677 = 1u64;
        if nat_le(a676, a677) {
            Ok(true)
        } else {
            let a690 = {
                let a688 = {
                    let a682 = v0;
                    let a683 = {
                        let a680 = {
                            let a678 = v1;
                            let a679 = 1u64;
                            nat_sub(a678, a679)
                        };
                        let a681 = v2;
                        nat_mul(a680, a681)?
                    };
                    nat_add(a682, a683)?
                };
                let a689 = {
                    let a686 = {
                        let a684 = v3;
                        let a685 = 1u64;
                        nat_sub(a684, a685)
                    };
                    let a687 = v4;
                    nat_mul(a686, a687)?
                };
                nat_add(a688, a689)?
            };
            let a691 = v6;
            Ok(nat_le(a690, a691))
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: u64) -> bool {
    let a692 = v0;
    let a693 = 2u64;
    if if if if nat_eq(a692, a693) {
        true
    } else {
        let a694 = v0;
        let a695 = 6u64;
        nat_eq(a694, a695)
    } {
        true
    } else {
        let a696 = v0;
        let a697 = 8u64;
        nat_eq(a696, a697)
    } {
        true
    } else {
        let a698 = v0;
        let a699 = 20u64;
        nat_eq(a698, a699)
    } {
        true
    } else {
        f35(v0)
    }
}

pub fn f31(v0: u64) -> bool {
    let a700 = v0;
    let a701 = 3u64;
    if nat_eq(a700, a701) {
        true
    } else {
        let a702 = v0;
        let a703 = 7u64;
        nat_eq(a702, a703)
    }
}

pub fn f32(v0: u64) -> u64 {
    let a704 = v0;
    let a705 = 30u64;
    if nat_eq(a704, a705) {
        30u64
    } else {
        0u64
    }
}

pub fn f33(v0: u64) -> Adt6 {
    let a706 = v0;
    let a707 = 0u64;
    if nat_eq(a706, a707) {
        Adt6::C1(Adt7::C0(0u64, 1u64, 4u64))
    } else {
        let a708 = v0;
        let a709 = 1u64;
        if nat_eq(a708, a709) {
            Adt6::C1(Adt7::C0(1u64, 1u64, 2u64))
        } else {
            let a710 = v0;
            let a711 = 2u64;
            if nat_eq(a710, a711) {
                Adt6::C1(Adt7::C0(2u64, 32u64, 18u64))
            } else {
                let a712 = v0;
                let a713 = 3u64;
                if nat_eq(a712, a713) {
                    Adt6::C1(Adt7::C0(3u64, 32u64, 20u64))
                } else {
                    let a714 = v0;
                    let a715 = 6u64;
                    if nat_eq(a714, a715) {
                        Adt6::C1(Adt7::C0(6u64, 32u64, 22u64))
                    } else {
                        let a716 = v0;
                        let a717 = 7u64;
                        if nat_eq(a716, a717) {
                            Adt6::C1(Adt7::C0(7u64, 32u64, 24u64))
                        } else {
                            let a718 = v0;
                            let a719 = 8u64;
                            if nat_eq(a718, a719) {
                                Adt6::C1(Adt7::C0(8u64, 32u64, 34u64))
                            } else {
                                let a720 = v0;
                                let a721 = 9u64;
                                if nat_eq(a720, a721) {
                                    Adt6::C1(Adt7::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a722 = v0;
                                    let a723 = 10u64;
                                    if nat_eq(a722, a723) {
                                        Adt6::C1(Adt7::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a724 = v0;
                                        let a725 = 11u64;
                                        if nat_eq(a724, a725) {
                                            Adt6::C1(Adt7::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a726 = v0;
                                            let a727 = 12u64;
                                            if nat_eq(a726, a727) {
                                                Adt6::C1(Adt7::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a728 = v0;
                                                let a729 = 13u64;
                                                if nat_eq(a728, a729) {
                                                    Adt6::C1(Adt7::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a730 = v0;
                                                    let a731 = 14u64;
                                                    if nat_eq(a730, a731) {
                                                        Adt6::C1(Adt7::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a732 = v0;
                                                        let a733 = 15u64;
                                                        if nat_eq(a732, a733) {
                                                            Adt6::C1(Adt7::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a734 = v0;
                                                            let a735 = 16u64;
                                                            if nat_eq(a734, a735) {
                                                                Adt6::C1(Adt7::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a736 = v0;
                                                                let a737 = 17u64;
                                                                if nat_eq(a736, a737) {
                                                                    Adt6::C1(Adt7::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a738 = v0;
                                                                    let a739 = 18u64;
                                                                    if nat_eq(a738, a739) {
                                                                        Adt6::C1(Adt7::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a740 = v0;
                                                                        let a741 = 19u64;
                                                                        if nat_eq(a740, a741) {
                                                                            Adt6::C1(Adt7::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a742 = v0;
                                                                            let a743 = 20u64;
                                                                            if nat_eq(a742, a743) {
                                                                                Adt6::C1(Adt7::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a744 = v0;
                                                                                let a745 = 21u64;
                                                                                if nat_eq(a744, a745) {
                                                                                    Adt6::C1(Adt7::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a746 = v0;
                                                                                    let a747 = 22u64;
                                                                                    if nat_eq(a746, a747) {
                                                                                        Adt6::C1(Adt7::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a748 = v0;
                                                                                        let a749 = 23u64;
                                                                                        if nat_eq(a748, a749) {
                                                                                            Adt6::C1(Adt7::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a750 = v0;
                                                                                            let a751 = 24u64;
                                                                                            if nat_eq(a750, a751) {
                                                                                                Adt6::C1(Adt7::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a752 = v0;
                                                                                                let a753 = 25u64;
                                                                                                if nat_eq(a752, a753) {
                                                                                                    Adt6::C1(Adt7::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a754 = v0;
                                                                                                    let a755 = 26u64;
                                                                                                    if nat_eq(a754, a755) {
                                                                                                        Adt6::C1(Adt7::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a756 = v0;
                                                                                                        let a757 = 27u64;
                                                                                                        if nat_eq(a756, a757) {
                                                                                                            Adt6::C1(Adt7::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a758 = v0;
                                                                                                            let a759 = 28u64;
                                                                                                            if nat_eq(a758, a759) {
                                                                                                                Adt6::C1(Adt7::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a760 = v0;
                                                                                                                let a761 = 29u64;
                                                                                                                if nat_eq(a760, a761) {
                                                                                                                    Adt6::C1(Adt7::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a762 = v0;
                                                                                                                    let a763 = 30u64;
                                                                                                                    if nat_eq(a762, a763) {
                                                                                                                        Adt6::C1(Adt7::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a764 = v0;
                                                                                                                        let a765 = 34u64;
                                                                                                                        if nat_eq(a764, a765) {
                                                                                                                            Adt6::C1(Adt7::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a766 = v0;
                                                                                                                            let a767 = 35u64;
                                                                                                                            if nat_eq(a766, a767) {
                                                                                                                                Adt6::C1(Adt7::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a768 = v0;
                                                                                                                                let a769 = 39u64;
                                                                                                                                if nat_eq(a768, a769) {
                                                                                                                                    Adt6::C1(Adt7::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a770 = v0;
                                                                                                                                    let a771 = 40u64;
                                                                                                                                    if nat_eq(a770, a771) {
                                                                                                                                        Adt6::C1(Adt7::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a772 = v0;
                                                                                                                                        let a773 = 41u64;
                                                                                                                                        if nat_eq(a772, a773) {
                                                                                                                                            Adt6::C1(Adt7::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a774 = v0;
                                                                                                                                            let a775 = 42u64;
                                                                                                                                            if nat_eq(a774, a775) {
                                                                                                                                                Adt6::C1(Adt7::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a776 = v0;
                                                                                                                                                let a777 = 43u64;
                                                                                                                                                if nat_eq(a776, a777) {
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

pub fn f34(v0: u64) -> bool {
    if f30(v0) {
        true
    } else {
        f31(v0)
    }
}

pub fn f35(v0: u64) -> bool {
    let a778 = v0;
    let a779 = 41u64;
    if nat_eq(a778, a779) {
        true
    } else {
        let a780 = v0;
        let a781 = 42u64;
        nat_eq(a780, a781)
    }
}
