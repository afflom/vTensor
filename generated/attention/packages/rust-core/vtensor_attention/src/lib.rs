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
    C10(Adt5, u64, u64, u64, u64),
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
    C0(Adt1, Adt1, Adt1, Adt1, Adt1),
}

#[derive(Clone)]
pub enum Adt6 {
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
    C10(Adt5),
    C11(u64, u64, u64, u64, u64),
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

pub fn f0(v0: Adt0) -> R<Adt6> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47) => {
            f3(v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, None::<Adt1>, v47)
        }
        Adt0::C1(v76, v77, v78, v79) => {
            f9(v76, v77, v78, v79)
        }
        Adt0::C2(v64, v65, v66, v67, v68, v69, v70, v71) => {
            f7(v64, v65, v66, v67, v68, v69, v70, v71)
        }
        Adt0::C3(v72, v73, v74, v75) => {
            f8(v72, v73, v74, v75)
        }
        Adt0::C4(v48, v49, v50, v51, v52, v53, v54, v55) => {
            f4(v48, v49, v50, v51, v52, v53, v54, v55)
        }
        Adt0::C5(v56, v57, v58, v59) => {
            f5(v56, v57, v58, v59)
        }
        Adt0::C6(v60, v61, v62, v63) => {
            f6(v60, v61, v62, v63)
        }
        Adt0::C7(v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36) => {
            f2(v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36)
        }
        Adt0::C8(v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) => {
            let m8 = v19.clone();
            match m8 {
                None => {
                    f2(v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v20)
                }
                Some(_) => {
                    let m9 = v14;
                    match m9 {
                        None => {
                            Ok(Adt6::C0)
                        }
                        Some(v22) => {
                            let m10 = v15;
                            match m10 {
                                None => {
                                    Ok(Adt6::C0)
                                }
                                Some(v23) => {
                                    let m11 = v16;
                                    match m11 {
                                        None => {
                                            Ok(Adt6::C0)
                                        }
                                        Some(v24) => {
                                            f3(v8, v9, v11, v12, v13, v22, v23, v24, v17, v18, v19, v20)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C9(v6, v7) => {
            let a2 = v6;
            let a3 = 2147483648u64;
            if nat_lt(a2, a3) {
                let a4 = v6;
                let a5 = v7;
                if nat_lt(a4, a5) {
                    Ok(Adt6::C9(v6))
                } else {
                    Ok(Adt6::C0)
                }
            } else {
                let a6 = v6;
                let a7 = 4294967296u64;
                if nat_lt(a6, a7) {
                    Ok(Adt6::C8)
                } else {
                    Ok(Adt6::C0)
                }
            }
        }
        Adt0::C10(v1, v2, v3, v4, v5) => {
            f1(v1, v2, v3, v4, v5)
        }
    }
}

pub fn f1(v0: Adt5, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt6> {
    let a15 = v1;
    let a16 = {
        let m14 = match v0.clone() {
            Adt5::C0(h13, _, _, _, _) => {
                h13
            }
        };
        match m14 {
            Adt1::C0(_, _, _, h12, _, _, _, _, _, _, _) => {
                h12
            }
        }
    };
    if if if if nat_lt(a15, a16) {
        let a20 = v2;
        let a21 = {
            let m19 = match v0.clone() {
                Adt5::C0(h18, _, _, _, _) => {
                    h18
                }
            };
            match m19 {
                Adt1::C0(_, _, h17, _, _, _, _, _, _, _, _) => {
                    h17
                }
            }
        };
        nat_lt(a20, a21)
    } else {
        false
    } {
        let a25 = v3;
        let a26 = {
            let m24 = match v0.clone() {
                Adt5::C0(h23, _, _, _, _) => {
                    h23
                }
            };
            match m24 {
                Adt1::C0(_, _, _, _, h22, _, _, _, _, _, _) => {
                    h22
                }
            }
        };
        nat_lt(a25, a26)
    } else {
        false
    } {
        let a30 = v4;
        let a31 = {
            let m29 = match v0.clone() {
                Adt5::C0(_, h28, _, _, _) => {
                    h28
                }
            };
            match m29 {
                Adt1::C0(_, _, _, h27, _, _, _, _, _, _, _) => {
                    h27
                }
            }
        };
        nat_lt(a30, a31)
    } else {
        false
    } {
        Ok(Adt6::C11(f10(match v0.clone() {
            Adt5::C0(h32, _, _, _, _) => {
                h32
            }
        }, v2, v1, v3)?, f10(match v0.clone() {
            Adt5::C0(_, h33, _, _, _) => {
                h33
            }
        }, 0u64, v4, v3)?, f10(match v0.clone() {
            Adt5::C0(_, _, h34, _, _) => {
                h34
            }
        }, v1, 0u64, v3)?, f10(match v0.clone() {
            Adt5::C0(_, _, _, h35, _) => {
                h35
            }
        }, v1, 0u64, {
            let a39 = v3;
            let a40 = {
                let m38 = match v0.clone() {
                    Adt5::C0(_, _, _, h37, _) => {
                        h37
                    }
                };
                match m38 {
                    Adt1::C0(_, _, _, _, h36, _, _, _, _, _, _) => {
                        h36
                    }
                }
            };
            let a41 = 0u64;
            nat_rem(a39, a40, a41)
        })?, f10(match v0 {
            Adt5::C0(_, _, _, _, h42) => {
                h42
            }
        }, v1, 0u64, v3)?))
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f2(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: u64, v6: Option<Adt1>, v7: Option<Adt1>, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt6> {
    let m43 = v6;
    match m43 {
        None => {
            Ok(Adt6::C0)
        }
        Some(v12) => {
            let a44 = v0;
            let a45 = 43u64;
            if nat_eq(a44, a45) {
                let m46 = v9;
                match m46 {
                    None => {
                        let m47 = v10;
                        match m47 {
                            None => {
                                f7(v0, v1, v2, v3, v12, v7, v8, v11)
                            }
                            Some(_) => {
                                Ok(Adt6::C0)
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Adt6::C0)
                    }
                }
            } else {
                let m48 = v7;
                match m48 {
                    None => {
                        Ok(Adt6::C0)
                    }
                    Some(v15) => {
                        let a49 = v0;
                        let a50 = 84u64;
                        if nat_eq(a49, a50) {
                            f11(v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let a51 = v0;
                            let a52 = 88u64;
                            if nat_eq(a51, a52) {
                                f4(v0, v1, v12, v15, v8, v9, v10, v11)
                            } else {
                                let m53 = v8;
                                match m53 {
                                    None => {
                                        Ok(Adt6::C0)
                                    }
                                    Some(v16) => {
                                        f3(v0, v1, v3, v4, v5, v12, v15, v16, v9, v10, None::<Adt1>, v11)
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

pub fn f3(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt6> {
    let m54 = f12(v10, v8.clone(), v5.clone(), v6.clone(), v7.clone(), v11.clone())?;
    match m54 {
        None => {
            Ok(Adt6::C0)
        }
        Some(v12) => {
            let a55 = v0;
            let a56 = 71u64;
            if if if if if if if if if if if if if if if if if if nat_eq(a55, a56) {
                !v1
            } else {
                false
            } {
                f13(v2, match v5.clone() {
                    Adt1::C0(_, _, _, h57, _, _, _, _, _, _, _) => {
                        h57
                    }
                }, v8.clone())
            } else {
                false
            } {
                let a58 = v4;
                let a59 = 0u64;
                if if nat_eq(a58, a59) {
                    true
                } else {
                    let a60 = v4;
                    let a61 = 10u64;
                    nat_eq(a60, a61)
                } {
                    true
                } else {
                    let a62 = v4;
                    let a63 = 15u64;
                    nat_eq(a62, a63)
                }
            } else {
                false
            } {
                let a65 = match v5.clone() {
                    Adt1::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                        h64
                    }
                };
                let a66 = 0u64;
                nat_eq(a65, a66)
            } else {
                false
            } {
                let a68 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h67, _, _, _, _, _) => {
                        h67
                    }
                };
                let a69 = 4u64;
                nat_eq(a68, a69)
            } else {
                false
            } {
                f14(v11.clone())?
            } else {
                false
            } {
                f15(v6.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                f15(v7.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                let a72 = match v5.clone() {
                    Adt1::C0(_, h70, _, _, _, _, _, _, _, _, _) => {
                        h70
                    }
                };
                let a73 = match v6.clone() {
                    Adt1::C0(_, h71, _, _, _, _, _, _, _, _, _) => {
                        h71
                    }
                };
                nat_eq(a72, a73)
            } else {
                false
            } {
                let a76 = match v6.clone() {
                    Adt1::C0(_, _, h74, _, _, _, _, _, _, _, _) => {
                        h74
                    }
                };
                let a77 = match v7.clone() {
                    Adt1::C0(_, _, h75, _, _, _, _, _, _, _, _) => {
                        h75
                    }
                };
                nat_eq(a76, a77)
            } else {
                false
            } {
                let a80 = match v11.clone() {
                    Adt1::C0(_, h78, _, _, _, _, _, _, _, _, _) => {
                        h78
                    }
                };
                let a81 = match v7.clone() {
                    Adt1::C0(_, h79, _, _, _, _, _, _, _, _, _) => {
                        h79
                    }
                };
                nat_eq(a80, a81)
            } else {
                false
            } {
                let a84 = match v11.clone() {
                    Adt1::C0(_, _, h82, _, _, _, _, _, _, _, _) => {
                        h82
                    }
                };
                let a85 = match v5.clone() {
                    Adt1::C0(_, _, _, h83, _, _, _, _, _, _, _) => {
                        h83
                    }
                };
                nat_eq(a84, a85)
            } else {
                false
            } {
                let a88 = match v11.clone() {
                    Adt1::C0(_, _, _, h86, _, _, _, _, _, _, _) => {
                        h86
                    }
                };
                let a89 = match v5.clone() {
                    Adt1::C0(_, _, h87, _, _, _, _, _, _, _, _) => {
                        h87
                    }
                };
                nat_eq(a88, a89)
            } else {
                false
            } {
                let a92 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h90, _, _, _, _, _, _) => {
                        h90
                    }
                };
                let a93 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h91, _, _, _, _, _, _) => {
                        h91
                    }
                };
                nat_eq(a92, a93)
            } else {
                false
            } {
                f16(v5.clone(), v11.clone())?
            } else {
                false
            } {
                f17(v8.clone(), v5.clone(), v12, v11.clone())?
            } else {
                false
            } {
                f18(v9.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                Ok(Adt6::C1(Adt2::C0(v5, v6, v7, v8, v9, v11, v12, f19(v2), f20(v3))))
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f4(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt6> {
    let v8: u64 = {
        let a95 = match v2.clone() {
            Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                h94
            }
        };
        let a96 = 1u64;
        if nat_eq(a95, a96) {
            2u64
        } else {
            4u64
        }
    };
    let a97 = v0;
    let a98 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a97, a98) {
        !v1
    } else {
        false
    } {
        let a100 = match v2.clone() {
            Adt1::C0(h99, _, _, _, _, _, _, _, _, _, _) => {
                h99
            }
        };
        let a101 = 0u64;
        if nat_eq(a100, a101) {
            true
        } else {
            let a103 = match v2.clone() {
                Adt1::C0(h102, _, _, _, _, _, _, _, _, _, _) => {
                    h102
                }
            };
            let a104 = 1u64;
            nat_eq(a103, a104)
        }
    } else {
        false
    } {
        let a106 = match v3.clone() {
            Adt1::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
                h105
            }
        };
        let a107 = 26u64;
        nat_eq(a106, a107)
    } else {
        false
    } {
        let a110 = match v7.clone() {
            Adt1::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                h108
            }
        };
        let a111 = match v2.clone() {
            Adt1::C0(h109, _, _, _, _, _, _, _, _, _, _) => {
                h109
            }
        };
        nat_eq(a110, a111)
    } else {
        false
    } {
        let a113 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h112, _, _, _, _, _) => {
                h112
            }
        };
        let a114 = v8;
        nat_eq(a113, a114)
    } else {
        false
    } {
        let a116 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h115, _, _, _, _, _) => {
                h115
            }
        };
        let a117 = v8;
        nat_eq(a116, a117)
    } else {
        false
    } {
        let a120 = match v2.clone() {
            Adt1::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        };
        let a121 = match v7.clone() {
            Adt1::C0(_, h119, _, _, _, _, _, _, _, _, _) => {
                h119
            }
        };
        nat_eq(a120, a121)
    } else {
        false
    } {
        f21(v3.clone(), v7.clone())
    } else {
        false
    } {
        f22({
            let a123 = v8;
            let a124 = match v7.clone() {
                Adt1::C0(_, h122, _, _, _, _, _, _, _, _, _) => {
                    h122
                }
            };
            nat_mul(a123, a124)?
        }, match v7.clone() {
            Adt1::C0(_, _, h125, _, _, _, _, _, _, _, _) => {
                h125
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h126, _, _, _, _, _, _, _) => {
                h126
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h127, _, _, _, _, _, _) => {
                h127
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h128, _, _, _, _) => {
                h128
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h129, _, _, _) => {
                h129
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h130, _, _) => {
                h130
            }
        })?
    } else {
        false
    } {
        f16(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f16(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f23(match v3.clone() {
            Adt1::C0(_, _, h131, _, _, _, _, _, _, _, _) => {
                h131
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h132, _, _, _, _, _, _, _, _) => {
                h132
            }
        })
    } else {
        false
    } {
        f23(match v3.clone() {
            Adt1::C0(_, _, _, h133, _, _, _, _, _, _, _) => {
                h133
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h134, _, _, _, _, _, _, _) => {
                h134
            }
        })
    } else {
        false
    } {
        f23(match v3.clone() {
            Adt1::C0(_, _, _, _, h135, _, _, _, _, _, _) => {
                h135
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h136, _, _, _, _, _, _) => {
                h136
            }
        })
    } else {
        false
    } {
        let m137 = v4.clone();
        match m137 {
            None => {
                true
            }
            Some(v9) => {
                let a139 = match v9.clone() {
                    Adt1::C0(h138, _, _, _, _, _, _, _, _, _, _) => {
                        h138
                    }
                };
                let a140 = 0u64;
                if if nat_eq(a139, a140) {
                    f21(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f16(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m141 = v5.clone();
        match m141 {
            None => {
                true
            }
            Some(v10) => {
                let a143 = match v10.clone() {
                    Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
                        h142
                    }
                };
                let a144 = 26u64;
                if if if if nat_eq(a143, a144) {
                    let a146 = match v10.clone() {
                        Adt1::C0(_, _, _, h145, _, _, _, _, _, _, _) => {
                            h145
                        }
                    };
                    let a147 = 1u64;
                    nat_eq(a146, a147)
                } else {
                    false
                } {
                    let a149 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                            h148
                        }
                    };
                    let a150 = 1u64;
                    nat_eq(a149, a150)
                } else {
                    false
                } {
                    f16(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m151 = v4.clone();
                    match m151 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a154 = match v11 {
                                Adt1::C0(_, h152, _, _, _, _, _, _, _, _, _) => {
                                    h152
                                }
                            };
                            let a155 = match v10 {
                                Adt1::C0(_, _, h153, _, _, _, _, _, _, _, _) => {
                                    h153
                                }
                            };
                            nat_eq(a154, a155)
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
        let m156 = v6.clone();
        match m156 {
            None => {
                true
            }
            Some(v12) => {
                let a158 = match v12.clone() {
                    Adt1::C0(h157, _, _, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                };
                let a159 = 26u64;
                if if nat_eq(a158, a159) {
                    f21(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f16(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt6::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a160 = v8;
            let a161 = 2u64;
            if nat_eq(a160, a161) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m162 = v5;
            match m162 {
                None => {
                    let m163 = v4;
                    match m163 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h164, _, _, _, _, _, _, _, _, _) => {
                                    h164
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h165, _, _, _, _, _, _, _, _, _) => {
                                    h165
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h166, _, _, _, _, _, _, _, _) => {
                            h166
                        }
                    }
                }
            }
        })))
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f5(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let v4: Adt1 = match v0.clone() {
        Adt4::C0(h167, _, _, _, _, _, _, _, _) => {
            h167
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h168, _, _, _, _, _, _, _) => {
            h168
        }
    };
    let a170 = v1;
    let a171 = match v5.clone() {
        Adt1::C0(_, _, h169, _, _, _, _, _, _, _, _) => {
            h169
        }
    };
    if if if nat_lt(a170, a171) {
        let a173 = v2;
        let a174 = match v5.clone() {
            Adt1::C0(_, _, _, h172, _, _, _, _, _, _, _) => {
                h172
            }
        };
        nat_lt(a173, a174)
    } else {
        false
    } {
        let a176 = v3;
        let a177 = match v5.clone() {
            Adt1::C0(_, _, _, _, h175, _, _, _, _, _, _) => {
                h175
            }
        };
        nat_lt(a176, a177)
    } else {
        false
    } {
        Ok(Adt6::C6(f10(v4.clone(), {
            let a179 = v1;
            let a180 = match v4.clone() {
                Adt1::C0(_, _, h178, _, _, _, _, _, _, _, _) => {
                    h178
                }
            };
            let a181 = 0u64;
            nat_rem(a179, a180, a181)
        }, {
            let a183 = v2;
            let a184 = match v4.clone() {
                Adt1::C0(_, _, _, h182, _, _, _, _, _, _, _) => {
                    h182
                }
            };
            let a185 = 0u64;
            nat_rem(a183, a184, a185)
        }, {
            let a187 = v3;
            let a188 = match v4 {
                Adt1::C0(_, _, _, _, h186, _, _, _, _, _, _) => {
                    h186
                }
            };
            let a189 = 0u64;
            nat_rem(a187, a188, a189)
        })?, f10(v5, v1, v2, v3)?, {
            let m191 = match v0.clone() {
                Adt4::C0(_, _, h190, _, _, _, _, _, _) => {
                    h190
                }
            };
            match m191 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f10(v6, v1, v2, v3)?
                }
            }
        }, {
            let m193 = match v0.clone() {
                Adt4::C0(_, _, _, _, h192, _, _, _, _) => {
                    h192
                }
            };
            match m193 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f10(v7, v1, v2, v3)?
                }
            }
        }, f10(match v0 {
            Adt4::C0(_, _, _, _, _, h194, _, _, _) => {
                h194
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f6(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let a195 = v1;
    let a196 = 0u64;
    if nat_eq(a195, a196) {
        let a200 = v2;
        let a201 = {
            let m199 = match v0.clone() {
                Adt4::C0(_, h198, _, _, _, _, _, _, _) => {
                    h198
                }
            };
            match m199 {
                Adt1::C0(_, h197, _, _, _, _, _, _, _, _, _) => {
                    h197
                }
            }
        };
        if nat_lt(a200, a201) {
            Ok(Adt6::C7({
                let a205 = v2;
                let a206 = {
                    let m204 = match v0 {
                        Adt4::C0(_, h203, _, _, _, _, _, _, _) => {
                            h203
                        }
                    };
                    match m204 {
                        Adt1::C0(_, _, _, _, _, h202, _, _, _, _, _) => {
                            h202
                        }
                    }
                };
                nat_mul(a205, a206)?
            }))
        } else {
            Ok(Adt6::C8)
        }
    } else {
        let a207 = v1;
        let a208 = 1u64;
        if nat_eq(a207, a208) {
            let m210 = match v0 {
                Adt4::C0(_, _, _, _, h209, _, _, _, _) => {
                    h209
                }
            };
            match m210 {
                None => {
                    Ok(Adt6::C8)
                }
                Some(v4) => {
                    let a212 = v2;
                    let a213 = match v4.clone() {
                        Adt1::C0(_, h211, _, _, _, _, _, _, _, _, _) => {
                            h211
                        }
                    };
                    if nat_lt(a212, a213) {
                        Ok(Adt6::C7({
                            let a215 = v2;
                            let a216 = match v4 {
                                Adt1::C0(_, _, _, _, _, h214, _, _, _, _, _) => {
                                    h214
                                }
                            };
                            nat_mul(a215, a216)?
                        }))
                    } else {
                        Ok(Adt6::C8)
                    }
                }
            }
        } else {
            let a217 = v1;
            let a218 = 2u64;
            if nat_eq(a217, a218) {
                let a219 = v2;
                let a220 = 2147483648u64;
                if if nat_lt(a219, a220) {
                    let a222 = v2;
                    let a223 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h221) => {
                            h221
                        }
                    };
                    nat_lt(a222, a223)
                } else {
                    false
                } {
                    let m225 = match v0 {
                        Adt4::C0(_, _, h224, _, _, _, _, _, _) => {
                            h224
                        }
                    };
                    match m225 {
                        None => {
                            Ok(Adt6::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt6::C7({
                                let a227 = v2;
                                let a228 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h226, _, _, _, _, _) => {
                                        h226
                                    }
                                };
                                nat_mul(a227, a228)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt6::C8)
                }
            } else {
                let a229 = v1;
                let a230 = 3u64;
                if nat_eq(a229, a230) {
                    let m232 = match v0 {
                        Adt4::C0(_, _, _, h231, _, _, _, _, _) => {
                            h231
                        }
                    };
                    match m232 {
                        None => {
                            Ok(Adt6::C8)
                        }
                        Some(v6) => {
                            let a234 = v2;
                            let a235 = match v6.clone() {
                                Adt1::C0(_, _, h233, _, _, _, _, _, _, _, _) => {
                                    h233
                                }
                            };
                            if if nat_lt(a234, a235) {
                                let a237 = v3;
                                let a238 = match v6.clone() {
                                    Adt1::C0(_, h236, _, _, _, _, _, _, _, _, _) => {
                                        h236
                                    }
                                };
                                nat_lt(a237, a238)
                            } else {
                                false
                            } {
                                Ok(Adt6::C7({
                                    let a245 = {
                                        let a240 = v2;
                                        let a241 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h239, _, _, _, _) => {
                                                h239
                                            }
                                        };
                                        nat_mul(a240, a241)?
                                    };
                                    let a246 = {
                                        let a243 = v3;
                                        let a244 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h242, _, _, _, _, _) => {
                                                h242
                                            }
                                        };
                                        nat_mul(a243, a244)?
                                    };
                                    nat_add(a245, a246)?
                                }))
                            } else {
                                Ok(Adt6::C8)
                            }
                        }
                    }
                } else {
                    let a247 = v1;
                    let a248 = 4u64;
                    if nat_eq(a247, a248) {
                        let a249 = v2;
                        let a250 = 2147483648u64;
                        if if nat_lt(a249, a250) {
                            let a254 = v2;
                            let a255 = {
                                let m253 = match v0.clone() {
                                    Adt4::C0(h252, _, _, _, _, _, _, _, _) => {
                                        h252
                                    }
                                };
                                match m253 {
                                    Adt1::C0(_, h251, _, _, _, _, _, _, _, _, _) => {
                                        h251
                                    }
                                }
                            };
                            nat_lt(a254, a255)
                        } else {
                            false
                        } {
                            Ok(Adt6::C7({
                                let a257 = v2;
                                let a258 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h256, _, _) => {
                                        h256
                                    }
                                };
                                nat_mul(a257, a258)?
                            }))
                        } else {
                            Ok(Adt6::C8)
                        }
                    } else {
                        Ok(Adt6::C0)
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt6> {
    if v1 {
        Ok(Adt6::C0)
    } else {
        let a259 = v0;
        let a260 = 43u64;
        if if if if if if if if nat_eq(a259, a260) {
            let a262 = match v4.clone() {
                Adt1::C0(h261, _, _, _, _, _, _, _, _, _, _) => {
                    h261
                }
            };
            let a263 = 0u64;
            if nat_eq(a262, a263) {
                f24(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a265 = match v7.clone() {
                Adt1::C0(h264, _, _, _, _, _, _, _, _, _, _) => {
                    h264
                }
            };
            let a266 = 0u64;
            if nat_eq(a265, a266) {
                f24(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f25(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f16(v4.clone(), v7.clone())? {
                true
            } else {
                f26(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f27(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f28(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f13(v3, match v4.clone() {
                Adt1::C0(_, _, _, h267, _, _, _, _, _, _, _) => {
                    h267
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt6::C3(Adt3::C0(v4, v5, v7, v2, f19(v3))))
        } else {
            Ok(Adt6::C0)
        }
    }
}

pub fn f8(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h268, _, _, _, _) => {
            h268
        }
    };
    let a270 = v1;
    let a271 = match v4.clone() {
        Adt1::C0(_, _, h269, _, _, _, _, _, _, _, _) => {
            h269
        }
    };
    if if if nat_lt(a270, a271) {
        let a273 = v2;
        let a274 = match v4.clone() {
            Adt1::C0(_, _, _, h272, _, _, _, _, _, _, _) => {
                h272
            }
        };
        nat_lt(a273, a274)
    } else {
        false
    } {
        let a276 = v3;
        let a277 = match v4.clone() {
            Adt1::C0(_, _, _, _, h275, _, _, _, _, _, _) => {
                h275
            }
        };
        nat_lt(a276, a277)
    } else {
        false
    } {
        let v6: u64 = {
            let m279 = match v0.clone() {
                Adt3::C0(_, h278, _, _, _) => {
                    h278
                }
            };
            match m279 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f10(v5.clone(), v1, {
                        let a281 = v2;
                        let a282 = match v5.clone() {
                            Adt1::C0(_, _, _, h280, _, _, _, _, _, _, _) => {
                                h280
                            }
                        };
                        let a283 = 0u64;
                        nat_rem(a281, a282, a283)
                    }, {
                        let a285 = v3;
                        let a286 = match v5 {
                            Adt1::C0(_, _, _, _, h284, _, _, _, _, _, _) => {
                                h284
                            }
                        };
                        let a287 = 0u64;
                        nat_rem(a285, a286, a287)
                    })?
                }
            }
        };
        Ok(Adt6::C4(f10(v4, v1, v2, v3)?, v6, {
            let a288 = v2;
            let a289 = 4u64;
            nat_mul(a288, a289)?
        }, f10(match v0 {
            Adt3::C0(_, _, h290, _, _) => {
                h290
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f9(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let a294 = v1;
    let a295 = {
        let m293 = match v0.clone() {
            Adt2::C0(h292, _, _, _, _, _, _, _, _) => {
                h292
            }
        };
        match m293 {
            Adt1::C0(_, _, h291, _, _, _, _, _, _, _, _) => {
                h291
            }
        }
    };
    if if if nat_lt(a294, a295) {
        let a299 = v2;
        let a300 = {
            let m298 = match v0.clone() {
                Adt2::C0(h297, _, _, _, _, _, _, _, _) => {
                    h297
                }
            };
            match m298 {
                Adt1::C0(_, _, _, h296, _, _, _, _, _, _, _) => {
                    h296
                }
            }
        };
        nat_lt(a299, a300)
    } else {
        false
    } {
        let a304 = v3;
        let a305 = {
            let m303 = match v0.clone() {
                Adt2::C0(h302, _, _, _, _, _, _, _, _) => {
                    h302
                }
            };
            match m303 {
                Adt1::C0(_, _, _, _, h301, _, _, _, _, _, _) => {
                    h301
                }
            }
        };
        nat_lt(a304, a305)
    } else {
        false
    } {
        Ok(Adt6::C2(f10(match v0.clone() {
            Adt2::C0(h306, _, _, _, _, _, _, _, _) => {
                h306
            }
        }, v1, v2, v3)?, f10(match v0.clone() {
            Adt2::C0(_, h307, _, _, _, _, _, _, _) => {
                h307
            }
        }, 0u64, {
            let a317 = v2;
            let a318 = {
                let a314 = {
                    let m310 = match v0.clone() {
                        Adt2::C0(h309, _, _, _, _, _, _, _, _) => {
                            h309
                        }
                    };
                    match m310 {
                        Adt1::C0(_, _, _, h308, _, _, _, _, _, _, _) => {
                            h308
                        }
                    }
                };
                let a315 = {
                    let m313 = match v0.clone() {
                        Adt2::C0(_, h312, _, _, _, _, _, _, _) => {
                            h312
                        }
                    };
                    match m313 {
                        Adt1::C0(_, _, _, h311, _, _, _, _, _, _, _) => {
                            h311
                        }
                    }
                };
                let a316 = 0u64;
                nat_quot(a314, a315, a316)
            };
            let a319 = 0u64;
            nat_quot(a317, a318, a319)
        }, {
            let a329 = v3;
            let a330 = {
                let a326 = {
                    let m322 = match v0.clone() {
                        Adt2::C0(h321, _, _, _, _, _, _, _, _) => {
                            h321
                        }
                    };
                    match m322 {
                        Adt1::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                            h320
                        }
                    }
                };
                let a327 = {
                    let m325 = match v0.clone() {
                        Adt2::C0(_, h324, _, _, _, _, _, _, _) => {
                            h324
                        }
                    };
                    match m325 {
                        Adt1::C0(_, _, _, _, h323, _, _, _, _, _, _) => {
                            h323
                        }
                    }
                };
                let a328 = 0u64;
                nat_quot(a326, a327, a328)
            };
            let a331 = 0u64;
            nat_quot(a329, a330, a331)
        })?, f10(match v0.clone() {
            Adt2::C0(_, _, h332, _, _, _, _, _, _) => {
                h332
            }
        }, 0u64, {
            let a342 = v2;
            let a343 = {
                let a339 = {
                    let m335 = match v0.clone() {
                        Adt2::C0(h334, _, _, _, _, _, _, _, _) => {
                            h334
                        }
                    };
                    match m335 {
                        Adt1::C0(_, _, _, h333, _, _, _, _, _, _, _) => {
                            h333
                        }
                    }
                };
                let a340 = {
                    let m338 = match v0.clone() {
                        Adt2::C0(_, _, h337, _, _, _, _, _, _) => {
                            h337
                        }
                    };
                    match m338 {
                        Adt1::C0(_, _, _, h336, _, _, _, _, _, _, _) => {
                            h336
                        }
                    }
                };
                let a341 = 0u64;
                nat_quot(a339, a340, a341)
            };
            let a344 = 0u64;
            nat_quot(a342, a343, a344)
        }, {
            let a354 = v3;
            let a355 = {
                let a351 = {
                    let m347 = match v0.clone() {
                        Adt2::C0(h346, _, _, _, _, _, _, _, _) => {
                            h346
                        }
                    };
                    match m347 {
                        Adt1::C0(_, _, _, _, h345, _, _, _, _, _, _) => {
                            h345
                        }
                    }
                };
                let a352 = {
                    let m350 = match v0.clone() {
                        Adt2::C0(_, _, h349, _, _, _, _, _, _) => {
                            h349
                        }
                    };
                    match m350 {
                        Adt1::C0(_, _, _, _, h348, _, _, _, _, _, _) => {
                            h348
                        }
                    }
                };
                let a353 = 0u64;
                nat_quot(a351, a352, a353)
            };
            let a356 = 0u64;
            nat_quot(a354, a355, a356)
        })?, {
            let m358 = match v0.clone() {
                Adt2::C0(_, _, _, h357, _, _, _, _, _) => {
                    h357
                }
            };
            match m358 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f10(v4.clone(), v1, {
                        let a360 = v2;
                        let a361 = match v4.clone() {
                            Adt1::C0(_, _, _, h359, _, _, _, _, _, _, _) => {
                                h359
                            }
                        };
                        let a362 = 0u64;
                        nat_rem(a360, a361, a362)
                    }, {
                        let a364 = v3;
                        let a365 = match v4 {
                            Adt1::C0(_, _, _, _, h363, _, _, _, _, _, _) => {
                                h363
                            }
                        };
                        let a366 = 0u64;
                        nat_rem(a364, a365, a366)
                    })?
                }
            }
        }, {
            let a367 = v2;
            let a368 = 4u64;
            nat_mul(a367, a368)?
        }, f10(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h369, _, _, _) => {
                h369
            }
        }, v2, v1, v3)?, {
            let m372 = match v0.clone() {
                Adt2::C0(_, h371, _, _, _, _, _, _, _) => {
                    h371
                }
            };
            match m372 {
                Adt1::C0(_, _, _, _, _, _, h370, _, _, _, _) => {
                    h370
                }
            }
        }, {
            let m375 = match v0.clone() {
                Adt2::C0(_, _, h374, _, _, _, _, _, _) => {
                    h374
                }
            };
            match m375 {
                Adt1::C0(_, _, _, _, _, _, h373, _, _, _, _) => {
                    h373
                }
            }
        }, f29({
            let m378 = match v0 {
                Adt2::C0(_, h377, _, _, _, _, _, _, _) => {
                    h377
                }
            };
            match m378 {
                Adt1::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                    h376
                }
            }
        })))
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f10(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a390 = {
        let a385 = {
            let a380 = v1;
            let a381 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h379, _, _, _, _) => {
                    h379
                }
            };
            nat_mul(a380, a381)?
        };
        let a386 = {
            let a383 = v2;
            let a384 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h382, _, _, _) => {
                    h382
                }
            };
            nat_mul(a383, a384)?
        };
        nat_add(a385, a386)?
    };
    let a391 = {
        let a388 = v3;
        let a389 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h387, _, _) => {
                h387
            }
        };
        nat_mul(a388, a389)?
    };
    nat_add(a390, a391)
}

pub fn f11(v0: bool, v1: Adt1, v2: Adt1, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Adt1) -> R<Adt6> {
    let v8: Adt1 = {
        let m392 = v3.clone();
        match m392 {
            None => {
                v1.clone()
            }
            Some(v7) => {
                v7
            }
        }
    };
    let v10: Adt1 = {
        let m393 = v4.clone();
        match m393 {
            None => {
                v1.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if v0 {
        false
    } else {
        let m394 = v5;
        match m394 {
            None => {
                let m395 = v3;
                match m395 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m396 = v4;
                        match m396 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        }
                    }
                }
            }
            Some(_) => {
                false
            }
        }
    } {
        let a398 = match v1.clone() {
            Adt1::C0(h397, _, _, _, _, _, _, _, _, _, _) => {
                h397
            }
        };
        let a399 = 0u64;
        if nat_eq(a398, a399) {
            let a401 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h400, _, _, _, _, _) => {
                    h400
                }
            };
            let a402 = 4u64;
            nat_eq(a401, a402)
        } else {
            false
        }
    } else {
        false
    } {
        let a404 = match v8.clone() {
            Adt1::C0(h403, _, _, _, _, _, _, _, _, _, _) => {
                h403
            }
        };
        let a405 = 0u64;
        if nat_eq(a404, a405) {
            let a407 = match v8.clone() {
                Adt1::C0(_, _, _, _, _, h406, _, _, _, _, _) => {
                    h406
                }
            };
            let a408 = 4u64;
            nat_eq(a407, a408)
        } else {
            false
        }
    } else {
        false
    } {
        let a410 = match v10.clone() {
            Adt1::C0(h409, _, _, _, _, _, _, _, _, _, _) => {
                h409
            }
        };
        let a411 = 1u64;
        if nat_eq(a410, a411) {
            let a413 = match v10.clone() {
                Adt1::C0(_, _, _, _, _, h412, _, _, _, _, _) => {
                    h412
                }
            };
            let a414 = 2u64;
            nat_eq(a413, a414)
        } else {
            false
        }
    } else {
        false
    } {
        f14(v6.clone())?
    } else {
        false
    } {
        f30(match v2.clone() {
            Adt1::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
                h415
            }
        }, match v2.clone() {
            Adt1::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
                h416
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h417, _, _, _, _, _) => {
                h417
            }
        })
    } else {
        false
    } {
        let a419 = 0u64;
        let a420 = match v1.clone() {
            Adt1::C0(_, h418, _, _, _, _, _, _, _, _, _) => {
                h418
            }
        };
        nat_lt(a419, a420)
    } else {
        false
    } {
        let a422 = 0u64;
        let a423 = match v1.clone() {
            Adt1::C0(_, _, h421, _, _, _, _, _, _, _, _) => {
                h421
            }
        };
        nat_lt(a422, a423)
    } else {
        false
    } {
        let a425 = 0u64;
        let a426 = match v1.clone() {
            Adt1::C0(_, _, _, h424, _, _, _, _, _, _, _) => {
                h424
            }
        };
        nat_lt(a425, a426)
    } else {
        false
    } {
        let a428 = 0u64;
        let a429 = match v1.clone() {
            Adt1::C0(_, _, _, _, h427, _, _, _, _, _, _) => {
                h427
            }
        };
        nat_lt(a428, a429)
    } else {
        false
    } {
        let a431 = 0u64;
        let a432 = match v2.clone() {
            Adt1::C0(_, _, _, h430, _, _, _, _, _, _, _) => {
                h430
            }
        };
        nat_lt(a431, a432)
    } else {
        false
    } {
        let a435 = match v2.clone() {
            Adt1::C0(_, h433, _, _, _, _, _, _, _, _, _) => {
                h433
            }
        };
        let a436 = match v1.clone() {
            Adt1::C0(_, h434, _, _, _, _, _, _, _, _, _) => {
                h434
            }
        };
        nat_eq(a435, a436)
    } else {
        false
    } {
        let a438 = match v2.clone() {
            Adt1::C0(_, _, h437, _, _, _, _, _, _, _, _) => {
                h437
            }
        };
        let a439 = 1u64;
        nat_eq(a438, a439)
    } else {
        false
    } {
        let a442 = match v2.clone() {
            Adt1::C0(_, _, _, _, h440, _, _, _, _, _, _) => {
                h440
            }
        };
        let a443 = match v1.clone() {
            Adt1::C0(_, _, _, _, h441, _, _, _, _, _, _) => {
                h441
            }
        };
        nat_eq(a442, a443)
    } else {
        false
    } {
        let a446 = match v8.clone() {
            Adt1::C0(_, h444, _, _, _, _, _, _, _, _, _) => {
                h444
            }
        };
        let a447 = match v1.clone() {
            Adt1::C0(_, _, h445, _, _, _, _, _, _, _, _) => {
                h445
            }
        };
        nat_eq(a446, a447)
    } else {
        false
    } {
        let a450 = match v8.clone() {
            Adt1::C0(_, _, h448, _, _, _, _, _, _, _, _) => {
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
        let a453 = match v8.clone() {
            Adt1::C0(_, _, _, h452, _, _, _, _, _, _, _) => {
                h452
            }
        };
        let a454 = 1u64;
        nat_eq(a453, a454)
    } else {
        false
    } {
        let a457 = match v8.clone() {
            Adt1::C0(_, _, _, _, h455, _, _, _, _, _, _) => {
                h455
            }
        };
        let a458 = match v1.clone() {
            Adt1::C0(_, _, _, _, h456, _, _, _, _, _, _) => {
                h456
            }
        };
        nat_eq(a457, a458)
    } else {
        false
    } {
        let a461 = match v10.clone() {
            Adt1::C0(_, h459, _, _, _, _, _, _, _, _, _) => {
                h459
            }
        };
        let a462 = match v2.clone() {
            Adt1::C0(_, _, _, h460, _, _, _, _, _, _, _) => {
                h460
            }
        };
        nat_eq(a461, a462)
    } else {
        false
    } {
        let a465 = match v10.clone() {
            Adt1::C0(_, _, h463, _, _, _, _, _, _, _, _) => {
                h463
            }
        };
        let a466 = match v1.clone() {
            Adt1::C0(_, _, _, h464, _, _, _, _, _, _, _) => {
                h464
            }
        };
        nat_eq(a465, a466)
    } else {
        false
    } {
        let a468 = match v10.clone() {
            Adt1::C0(_, _, _, h467, _, _, _, _, _, _, _) => {
                h467
            }
        };
        let a469 = 1u64;
        nat_eq(a468, a469)
    } else {
        false
    } {
        f23(match v1.clone() {
            Adt1::C0(_, _, _, _, h470, _, _, _, _, _, _) => {
                h470
            }
        }, match v10.clone() {
            Adt1::C0(_, _, _, _, h471, _, _, _, _, _, _) => {
                h471
            }
        })
    } else {
        false
    } {
        let a474 = match v6.clone() {
            Adt1::C0(_, h472, _, _, _, _, _, _, _, _, _) => {
                h472
            }
        };
        let a475 = match v2.clone() {
            Adt1::C0(_, _, _, h473, _, _, _, _, _, _, _) => {
                h473
            }
        };
        nat_eq(a474, a475)
    } else {
        false
    } {
        let a478 = match v6.clone() {
            Adt1::C0(_, _, h476, _, _, _, _, _, _, _, _) => {
                h476
            }
        };
        let a479 = match v1.clone() {
            Adt1::C0(_, _, _, h477, _, _, _, _, _, _, _) => {
                h477
            }
        };
        nat_eq(a478, a479)
    } else {
        false
    } {
        let a481 = match v6.clone() {
            Adt1::C0(_, _, _, h480, _, _, _, _, _, _, _) => {
                h480
            }
        };
        let a482 = 1u64;
        nat_eq(a481, a482)
    } else {
        false
    } {
        let a485 = match v6.clone() {
            Adt1::C0(_, _, _, _, h483, _, _, _, _, _, _) => {
                h483
            }
        };
        let a486 = match v1.clone() {
            Adt1::C0(_, _, _, _, h484, _, _, _, _, _, _) => {
                h484
            }
        };
        nat_eq(a485, a486)
    } else {
        false
    } {
        f16(v1.clone(), v6.clone())?
    } else {
        false
    } {
        f16(v2.clone(), v6.clone())?
    } else {
        false
    } {
        f16(v8.clone(), v6.clone())?
    } else {
        false
    } {
        f16(v10.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt6::C10(Adt5::C0(v1, v2, v8, v10, v6)))
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f12(v0: Option<Adt1>, v1: Option<Adt1>, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Option<u64>> {
    let m487 = v0;
    match m487 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h488, _, _, _, _, _, _, _, _) => {
                    h488
                }
            }))
        }
        Some(v6) => {
            let m489 = v1;
            match m489 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a491 = match v6.clone() {
                        Adt1::C0(h490, _, _, _, _, _, _, _, _, _, _) => {
                            h490
                        }
                    };
                    let a492 = 26u64;
                    if if if if if if if if if nat_eq(a491, a492) {
                        f24(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a495 = match v6.clone() {
                            Adt1::C0(_, h493, _, _, _, _, _, _, _, _, _) => {
                                h493
                            }
                        };
                        let a496 = match v7 {
                            Adt1::C0(_, h494, _, _, _, _, _, _, _, _, _) => {
                                h494
                            }
                        };
                        nat_eq(a495, a496)
                    } else {
                        false
                    } {
                        let a499 = match v6.clone() {
                            Adt1::C0(_, _, h497, _, _, _, _, _, _, _, _) => {
                                h497
                            }
                        };
                        let a500 = match v2 {
                            Adt1::C0(_, _, _, _, h498, _, _, _, _, _, _) => {
                                h498
                            }
                        };
                        nat_eq(a499, a500)
                    } else {
                        false
                    } {
                        let a502 = match v6.clone() {
                            Adt1::C0(_, _, _, h501, _, _, _, _, _, _, _) => {
                                h501
                            }
                        };
                        let a503 = 1u64;
                        nat_eq(a502, a503)
                    } else {
                        false
                    } {
                        let a505 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h504, _, _, _, _, _, _) => {
                                h504
                            }
                        };
                        let a506 = 1u64;
                        nat_eq(a505, a506)
                    } else {
                        false
                    } {
                        let a508 = match v3 {
                            Adt1::C0(_, _, _, _, h507, _, _, _, _, _, _) => {
                                h507
                            }
                        };
                        let a509 = 1u64;
                        nat_eq(a508, a509)
                    } else {
                        false
                    } {
                        let a511 = match v4 {
                            Adt1::C0(_, _, _, _, h510, _, _, _, _, _, _) => {
                                h510
                            }
                        };
                        let a512 = 1u64;
                        nat_eq(a511, a512)
                    } else {
                        false
                    } {
                        f16(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h513, _, _, _, _, _, _, _, _, _) => {
                                h513
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

pub fn f13(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a514 = v0;
    let a515 = 0u64;
    if if nat_eq(a514, a515) {
        true
    } else {
        let a516 = 2147483648u64;
        let a517 = v0;
        if nat_le(a516, a517) {
            let a518 = v0;
            let a519 = 4286578688u64;
            nat_lt(a518, a519)
        } else {
            false
        }
    } {
        true
    } else {
        let a520 = v0;
        let a521 = 2139095040u64;
        if if nat_lt(a520, a521) {
            let a522 = v1;
            let a523 = 4294967295u64;
            nat_le(a522, a523)
        } else {
            false
        } {
            let m524 = v2;
            match m524 {
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

pub fn f14(v0: Adt1) -> R<bool> {
    let a526 = match v0.clone() {
        Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
            h525
        }
    };
    let a527 = 0u64;
    if if if if nat_eq(a526, a527) {
        let a529 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h528, _, _, _, _, _) => {
                h528
            }
        };
        let a530 = 4u64;
        nat_eq(a529, a530)
    } else {
        false
    } {
        let a536 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h531, _, _, _, _) => {
                h531
            }
        };
        let a537 = {
            let a534 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h532, _, _, _, _, _) => {
                    h532
                }
            };
            let a535 = match v0.clone() {
                Adt1::C0(_, h533, _, _, _, _, _, _, _, _, _) => {
                    h533
                }
            };
            nat_mul(a534, a535)?
        };
        nat_eq(a536, a537)
    } else {
        false
    } {
        let a543 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h538, _, _, _) => {
                h538
            }
        };
        let a544 = {
            let a541 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h539, _, _, _, _) => {
                    h539
                }
            };
            let a542 = match v0.clone() {
                Adt1::C0(_, _, h540, _, _, _, _, _, _, _, _) => {
                    h540
                }
            };
            nat_mul(a541, a542)?
        };
        nat_eq(a543, a544)
    } else {
        false
    } {
        let a550 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h545, _, _) => {
                h545
            }
        };
        let a551 = {
            let a548 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h546, _, _, _) => {
                    h546
                }
            };
            let a549 = match v0 {
                Adt1::C0(_, _, _, h547, _, _, _, _, _, _, _) => {
                    h547
                }
            };
            nat_mul(a548, a549)?
        };
        Ok(nat_eq(a550, a551))
    } else {
        Ok(false)
    }
}

pub fn f15(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f30(match v0.clone() {
        Adt1::C0(h552, _, _, _, _, _, _, _, _, _, _) => {
            h552
        }
    }, match v0.clone() {
        Adt1::C0(_, h553, _, _, _, _, _, _, _, _, _) => {
            h553
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h554, _, _, _, _, _) => {
            h554
        }
    }) {
        f23(match v1.clone() {
            Adt1::C0(_, _, _, h555, _, _, _, _, _, _, _) => {
                h555
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h556, _, _, _, _, _, _, _) => {
                h556
            }
        })
    } else {
        false
    } {
        f23(match v1 {
            Adt1::C0(_, _, _, _, h557, _, _, _, _, _, _) => {
                h557
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h558, _, _, _, _, _, _) => {
                h558
            }
        })
    } else {
        false
    } {
        f16(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f16(v0: Adt1, v1: Adt1) -> R<bool> {
    let a560 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h559, _) => {
            h559
        }
    };
    let a561 = 0u64;
    if nat_eq(a560, a561) {
        Ok(true)
    } else {
        let a563 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h562, _) => {
                h562
            }
        };
        let a564 = 0u64;
        if nat_eq(a563, a564) {
            Ok(true)
        } else {
            let a566 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h565) => {
                    h565
                }
            };
            let a567 = 0u64;
            if nat_eq(a566, a567) {
                Ok(true)
            } else {
                let a569 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h568) => {
                        h568
                    }
                };
                let a570 = 0u64;
                if nat_eq(a569, a570) {
                    Ok(true)
                } else {
                    let a576 = {
                        let a573 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h571) => {
                                h571
                            }
                        };
                        let a574 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h572, _) => {
                                h572
                            }
                        };
                        nat_add(a573, a574)?
                    };
                    let a577 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h575) => {
                            h575
                        }
                    };
                    if nat_le(a576, a577) {
                        Ok(true)
                    } else {
                        let a583 = {
                            let a580 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h578) => {
                                    h578
                                }
                            };
                            let a581 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h579, _) => {
                                    h579
                                }
                            };
                            nat_add(a580, a581)?
                        };
                        let a584 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h582) => {
                                h582
                            }
                        };
                        Ok(nat_le(a583, a584))
                    }
                }
            }
        }
    }
}

pub fn f17(v0: Option<Adt1>, v1: Adt1, v2: u64, v3: Adt1) -> R<bool> {
    let m585 = v0;
    match m585 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a587 = match v4.clone() {
                Adt1::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                    h586
                }
            };
            let a588 = 1u64;
            if if if if if if nat_eq(a587, a588) {
                let a590 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h589, _, _, _, _, _) => {
                        h589
                    }
                };
                let a591 = 2u64;
                nat_eq(a590, a591)
            } else {
                false
            } {
                let a593 = v2;
                let a594 = match v4.clone() {
                    Adt1::C0(_, h592, _, _, _, _, _, _, _, _, _) => {
                        h592
                    }
                };
                nat_le(a593, a594)
            } else {
                false
            } {
                let a597 = match v1.clone() {
                    Adt1::C0(_, _, h595, _, _, _, _, _, _, _, _) => {
                        h595
                    }
                };
                let a598 = match v4.clone() {
                    Adt1::C0(_, _, h596, _, _, _, _, _, _, _, _) => {
                        h596
                    }
                };
                nat_le(a597, a598)
            } else {
                false
            } {
                f23(match v1.clone() {
                    Adt1::C0(_, _, _, h599, _, _, _, _, _, _, _) => {
                        h599
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h600, _, _, _, _, _, _, _) => {
                        h600
                    }
                })
            } else {
                false
            } {
                f23(match v1 {
                    Adt1::C0(_, _, _, _, h601, _, _, _, _, _, _) => {
                        h601
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h602, _, _, _, _, _, _) => {
                        h602
                    }
                })
            } else {
                false
            } {
                f16(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f18(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m603 = v0;
    match m603 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a605 = match v3.clone() {
                Adt1::C0(h604, _, _, _, _, _, _, _, _, _, _) => {
                    h604
                }
            };
            let a606 = 0u64;
            if if if if if if nat_eq(a605, a606) {
                let a608 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h607, _, _, _, _, _) => {
                        h607
                    }
                };
                let a609 = 4u64;
                nat_eq(a608, a609)
            } else {
                false
            } {
                let a612 = match v3.clone() {
                    Adt1::C0(_, h610, _, _, _, _, _, _, _, _, _) => {
                        h610
                    }
                };
                let a613 = match v1 {
                    Adt1::C0(_, _, _, h611, _, _, _, _, _, _, _) => {
                        h611
                    }
                };
                nat_eq(a612, a613)
            } else {
                false
            } {
                let a615 = match v3.clone() {
                    Adt1::C0(_, _, h614, _, _, _, _, _, _, _, _) => {
                        h614
                    }
                };
                let a616 = 1u64;
                nat_eq(a615, a616)
            } else {
                false
            } {
                let a618 = match v3.clone() {
                    Adt1::C0(_, _, _, h617, _, _, _, _, _, _, _) => {
                        h617
                    }
                };
                let a619 = 1u64;
                nat_eq(a618, a619)
            } else {
                false
            } {
                let a621 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h620, _, _, _, _, _, _) => {
                        h620
                    }
                };
                let a622 = 1u64;
                nat_eq(a621, a622)
            } else {
                false
            } {
                f16(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f19(v0: u64) -> Option<u64> {
    let a623 = 0u64;
    let a624 = v0;
    if if nat_lt(a623, a624) {
        let a625 = v0;
        let a626 = 2139095040u64;
        nat_lt(a625, a626)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f20(v0: u64) -> Option<u64> {
    let a627 = v0;
    let a628 = 0u64;
    if if nat_eq(a627, a628) {
        true
    } else {
        let a629 = v0;
        let a630 = 2147483648u64;
        nat_eq(a629, a630)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> bool {
    let a633 = match v0.clone() {
        Adt1::C0(_, _, h631, _, _, _, _, _, _, _, _) => {
            h631
        }
    };
    let a634 = match v1.clone() {
        Adt1::C0(_, _, h632, _, _, _, _, _, _, _, _) => {
            h632
        }
    };
    if if nat_eq(a633, a634) {
        let a637 = match v0.clone() {
            Adt1::C0(_, _, _, h635, _, _, _, _, _, _, _) => {
                h635
            }
        };
        let a638 = match v1.clone() {
            Adt1::C0(_, _, _, h636, _, _, _, _, _, _, _) => {
                h636
            }
        };
        nat_eq(a637, a638)
    } else {
        false
    } {
        let a641 = match v0 {
            Adt1::C0(_, _, _, _, h639, _, _, _, _, _, _) => {
                h639
            }
        };
        let a642 = match v1 {
            Adt1::C0(_, _, _, _, h640, _, _, _, _, _, _) => {
                h640
            }
        };
        nat_eq(a641, a642)
    } else {
        false
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f31(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f31(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f31(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f31(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f31(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f31(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f23(v0: u64, v1: u64) -> bool {
    let a643 = 0u64;
    let a644 = v1;
    if nat_lt(a643, a644) {
        let a648 = {
            let a645 = v0;
            let a646 = v1;
            let a647 = 0u64;
            nat_rem(a645, a646, a647)
        };
        let a649 = 0u64;
        nat_eq(a648, a649)
    } else {
        false
    }
}

pub fn f24(v0: Adt1, v1: u64) -> R<bool> {
    let a651 = match v0.clone() {
        Adt1::C0(_, h650, _, _, _, _, _, _, _, _, _) => {
            h650
        }
    };
    let a652 = 1u64;
    if if if if nat_eq(a651, a652) {
        true
    } else {
        let a654 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h653, _, _, _, _, _) => {
                h653
            }
        };
        let a655 = v1;
        nat_eq(a654, a655)
    } {
        let a657 = match v0.clone() {
            Adt1::C0(_, _, h656, _, _, _, _, _, _, _, _) => {
                h656
            }
        };
        let a658 = 1u64;
        if nat_eq(a657, a658) {
            true
        } else {
            let a663 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h659, _, _, _, _) => {
                    h659
                }
            };
            let a664 = {
                let a661 = v1;
                let a662 = match v0.clone() {
                    Adt1::C0(_, h660, _, _, _, _, _, _, _, _, _) => {
                        h660
                    }
                };
                nat_mul(a661, a662)?
            };
            nat_eq(a663, a664)
        }
    } else {
        false
    } {
        let a666 = match v0.clone() {
            Adt1::C0(_, _, _, h665, _, _, _, _, _, _, _) => {
                h665
            }
        };
        let a667 = 1u64;
        if nat_eq(a666, a667) {
            true
        } else {
            let a675 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h668, _, _, _) => {
                    h668
                }
            };
            let a676 = {
                let a673 = {
                    let a670 = v1;
                    let a671 = match v0.clone() {
                        Adt1::C0(_, h669, _, _, _, _, _, _, _, _, _) => {
                            h669
                        }
                    };
                    nat_mul(a670, a671)?
                };
                let a674 = match v0.clone() {
                    Adt1::C0(_, _, h672, _, _, _, _, _, _, _, _) => {
                        h672
                    }
                };
                nat_mul(a673, a674)?
            };
            nat_eq(a675, a676)
        }
    } else {
        false
    } {
        let a678 = match v0.clone() {
            Adt1::C0(_, _, _, _, h677, _, _, _, _, _, _) => {
                h677
            }
        };
        let a679 = 1u64;
        if nat_eq(a678, a679) {
            Ok(true)
        } else {
            let a690 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h680, _, _) => {
                    h680
                }
            };
            let a691 = {
                let a688 = {
                    let a685 = {
                        let a682 = v1;
                        let a683 = match v0.clone() {
                            Adt1::C0(_, h681, _, _, _, _, _, _, _, _, _) => {
                                h681
                            }
                        };
                        nat_mul(a682, a683)?
                    };
                    let a686 = match v0.clone() {
                        Adt1::C0(_, _, h684, _, _, _, _, _, _, _, _) => {
                            h684
                        }
                    };
                    nat_mul(a685, a686)?
                };
                let a689 = match v0 {
                    Adt1::C0(_, _, _, h687, _, _, _, _, _, _, _) => {
                        h687
                    }
                };
                nat_mul(a688, a689)?
            };
            Ok(nat_eq(a690, a691))
        }
    } else {
        Ok(false)
    }
}

pub fn f25(v0: Adt1, v1: Adt1) -> bool {
    let a694 = match v0.clone() {
        Adt1::C0(_, h692, _, _, _, _, _, _, _, _, _) => {
            h692
        }
    };
    let a695 = match v1.clone() {
        Adt1::C0(_, h693, _, _, _, _, _, _, _, _, _) => {
            h693
        }
    };
    if if if nat_eq(a694, a695) {
        let a698 = match v0.clone() {
            Adt1::C0(_, _, h696, _, _, _, _, _, _, _, _) => {
                h696
            }
        };
        let a699 = match v1.clone() {
            Adt1::C0(_, _, h697, _, _, _, _, _, _, _, _) => {
                h697
            }
        };
        nat_eq(a698, a699)
    } else {
        false
    } {
        let a702 = match v0.clone() {
            Adt1::C0(_, _, _, h700, _, _, _, _, _, _, _) => {
                h700
            }
        };
        let a703 = match v1.clone() {
            Adt1::C0(_, _, _, h701, _, _, _, _, _, _, _) => {
                h701
            }
        };
        nat_eq(a702, a703)
    } else {
        false
    } {
        let a706 = match v0 {
            Adt1::C0(_, _, _, _, h704, _, _, _, _, _, _) => {
                h704
            }
        };
        let a707 = match v1 {
            Adt1::C0(_, _, _, _, h705, _, _, _, _, _, _) => {
                h705
            }
        };
        nat_eq(a706, a707)
    } else {
        false
    }
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a710 = match v0.clone() {
            Adt1::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                h708
            }
        };
        let a711 = match v1.clone() {
            Adt1::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
                h709
            }
        };
        nat_eq(a710, a711)
    } else {
        false
    } {
        let a714 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h712) => {
                h712
            }
        };
        let a715 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h713) => {
                h713
            }
        };
        nat_eq(a714, a715)
    } else {
        false
    } {
        let a718 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h716, _, _, _, _, _) => {
                h716
            }
        };
        let a719 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h717, _, _, _, _, _) => {
                h717
            }
        };
        nat_eq(a718, a719)
    } else {
        false
    } {
        let a722 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h720, _, _, _, _) => {
                h720
            }
        };
        let a723 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h721, _, _, _, _) => {
                h721
            }
        };
        nat_eq(a722, a723)
    } else {
        false
    } {
        let a726 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h724, _, _, _) => {
                h724
            }
        };
        let a727 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h725, _, _, _) => {
                h725
            }
        };
        nat_eq(a726, a727)
    } else {
        false
    } {
        let a730 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h728, _, _) => {
                h728
            }
        };
        let a731 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h729, _, _) => {
                h729
            }
        };
        nat_eq(a730, a731)
    } else {
        false
    }
}

pub fn f27(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m732 = v0;
    match m732 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a734 = match v3.clone() {
                Adt1::C0(h733, _, _, _, _, _, _, _, _, _, _) => {
                    h733
                }
            };
            let a735 = 0u64;
            if if if if if if if nat_eq(a734, a735) {
                true
            } else {
                let a737 = match v3.clone() {
                    Adt1::C0(h736, _, _, _, _, _, _, _, _, _, _) => {
                        h736
                    }
                };
                let a738 = 1u64;
                nat_eq(a737, a738)
            } {
                f24(v3.clone(), {
                    let a740 = match v3.clone() {
                        Adt1::C0(h739, _, _, _, _, _, _, _, _, _, _) => {
                            h739
                        }
                    };
                    let a741 = 0u64;
                    if nat_eq(a740, a741) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a744 = match v3.clone() {
                    Adt1::C0(_, h742, _, _, _, _, _, _, _, _, _) => {
                        h742
                    }
                };
                let a745 = match v1.clone() {
                    Adt1::C0(_, h743, _, _, _, _, _, _, _, _, _) => {
                        h743
                    }
                };
                nat_eq(a744, a745)
            } else {
                false
            } {
                let a748 = match v1.clone() {
                    Adt1::C0(_, _, h746, _, _, _, _, _, _, _, _) => {
                        h746
                    }
                };
                let a749 = match v3.clone() {
                    Adt1::C0(_, _, h747, _, _, _, _, _, _, _, _) => {
                        h747
                    }
                };
                nat_le(a748, a749)
            } else {
                false
            } {
                f23(match v1.clone() {
                    Adt1::C0(_, _, _, h750, _, _, _, _, _, _, _) => {
                        h750
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h751, _, _, _, _, _, _, _) => {
                        h751
                    }
                })
            } else {
                false
            } {
                f23(match v1 {
                    Adt1::C0(_, _, _, _, h752, _, _, _, _, _, _) => {
                        h752
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h753, _, _, _, _, _, _) => {
                        h753
                    }
                })
            } else {
                false
            } {
                f16(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f28(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m754 = v0;
    match m754 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a756 = match v3.clone() {
                Adt1::C0(h755, _, _, _, _, _, _, _, _, _, _) => {
                    h755
                }
            };
            let a757 = 0u64;
            if if if if if if nat_eq(a756, a757) {
                let a759 = match v3.clone() {
                    Adt1::C0(_, h758, _, _, _, _, _, _, _, _, _) => {
                        h758
                    }
                };
                let a760 = 1u64;
                if nat_eq(a759, a760) {
                    true
                } else {
                    let a762 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h761, _, _, _, _, _) => {
                            h761
                        }
                    };
                    let a763 = 4u64;
                    nat_eq(a762, a763)
                }
            } else {
                false
            } {
                let a766 = match v3.clone() {
                    Adt1::C0(_, h764, _, _, _, _, _, _, _, _, _) => {
                        h764
                    }
                };
                let a767 = match v1 {
                    Adt1::C0(_, _, _, h765, _, _, _, _, _, _, _) => {
                        h765
                    }
                };
                nat_eq(a766, a767)
            } else {
                false
            } {
                let a769 = match v3.clone() {
                    Adt1::C0(_, _, h768, _, _, _, _, _, _, _, _) => {
                        h768
                    }
                };
                let a770 = 1u64;
                nat_eq(a769, a770)
            } else {
                false
            } {
                let a772 = match v3.clone() {
                    Adt1::C0(_, _, _, h771, _, _, _, _, _, _, _) => {
                        h771
                    }
                };
                let a773 = 1u64;
                nat_eq(a772, a773)
            } else {
                false
            } {
                let a775 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h774, _, _, _, _, _, _) => {
                        h774
                    }
                };
                let a776 = 1u64;
                nat_eq(a775, a776)
            } else {
                false
            } {
                f16(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f32(v0) {
        8u64
    } else {
        if f33(v0) {
            9u64
        } else {
            f34(v0)
        }
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> bool {
    let m777 = f35(v0);
    match m777 {
        Adt7::C0 => {
            false
        }
        Adt7::C1(v3) => {
            let a778 = v0;
            let a779 = 0u64;
            if if if if if if nat_eq(a778, a779) {
                true
            } else {
                let a780 = v0;
                let a781 = 1u64;
                nat_eq(a780, a781)
            } {
                true
            } else {
                let a782 = v0;
                let a783 = 30u64;
                nat_eq(a782, a783)
            } {
                true
            } else {
                let a784 = v0;
                let a785 = 43u64;
                nat_eq(a784, a785)
            } {
                true
            } else {
                f36(v0)
            } {
                let a787 = v2;
                let a788 = match v3.clone() {
                    Adt8::C0(_, _, h786) => {
                        h786
                    }
                };
                nat_eq(a787, a788)
            } else {
                false
            } {
                f23(v1, match v3 {
                    Adt8::C0(_, h789, _) => {
                        h789
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a790 = v1;
    let a791 = 1u64;
    if if if nat_le(a790, a791) {
        true
    } else {
        let a792 = v0;
        let a793 = v2;
        nat_le(a792, a793)
    } {
        let a794 = v3;
        let a795 = 1u64;
        if nat_le(a794, a795) {
            true
        } else {
            let a802 = {
                let a800 = v0;
                let a801 = {
                    let a798 = {
                        let a796 = v1;
                        let a797 = 1u64;
                        nat_sub(a796, a797)
                    };
                    let a799 = v2;
                    nat_mul(a798, a799)?
                };
                nat_add(a800, a801)?
            };
            let a803 = v4;
            nat_le(a802, a803)
        }
    } else {
        false
    } {
        let a804 = v5;
        let a805 = 1u64;
        if nat_le(a804, a805) {
            Ok(true)
        } else {
            let a818 = {
                let a816 = {
                    let a810 = v0;
                    let a811 = {
                        let a808 = {
                            let a806 = v1;
                            let a807 = 1u64;
                            nat_sub(a806, a807)
                        };
                        let a809 = v2;
                        nat_mul(a808, a809)?
                    };
                    nat_add(a810, a811)?
                };
                let a817 = {
                    let a814 = {
                        let a812 = v3;
                        let a813 = 1u64;
                        nat_sub(a812, a813)
                    };
                    let a815 = v4;
                    nat_mul(a814, a815)?
                };
                nat_add(a816, a817)?
            };
            let a819 = v6;
            Ok(nat_le(a818, a819))
        }
    } else {
        Ok(false)
    }
}

pub fn f32(v0: u64) -> bool {
    let a820 = v0;
    let a821 = 2u64;
    if if if if nat_eq(a820, a821) {
        true
    } else {
        let a822 = v0;
        let a823 = 6u64;
        nat_eq(a822, a823)
    } {
        true
    } else {
        let a824 = v0;
        let a825 = 8u64;
        nat_eq(a824, a825)
    } {
        true
    } else {
        let a826 = v0;
        let a827 = 20u64;
        nat_eq(a826, a827)
    } {
        true
    } else {
        f37(v0)
    }
}

pub fn f33(v0: u64) -> bool {
    let a828 = v0;
    let a829 = 3u64;
    if nat_eq(a828, a829) {
        true
    } else {
        let a830 = v0;
        let a831 = 7u64;
        nat_eq(a830, a831)
    }
}

pub fn f34(v0: u64) -> u64 {
    let a832 = v0;
    let a833 = 30u64;
    if nat_eq(a832, a833) {
        30u64
    } else {
        0u64
    }
}

pub fn f35(v0: u64) -> Adt7 {
    let a834 = v0;
    let a835 = 0u64;
    if nat_eq(a834, a835) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a836 = v0;
        let a837 = 1u64;
        if nat_eq(a836, a837) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a838 = v0;
            let a839 = 2u64;
            if nat_eq(a838, a839) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a840 = v0;
                let a841 = 3u64;
                if nat_eq(a840, a841) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a842 = v0;
                    let a843 = 6u64;
                    if nat_eq(a842, a843) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a844 = v0;
                        let a845 = 7u64;
                        if nat_eq(a844, a845) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a846 = v0;
                            let a847 = 8u64;
                            if nat_eq(a846, a847) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a848 = v0;
                                let a849 = 9u64;
                                if nat_eq(a848, a849) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a850 = v0;
                                    let a851 = 10u64;
                                    if nat_eq(a850, a851) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a852 = v0;
                                        let a853 = 11u64;
                                        if nat_eq(a852, a853) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a854 = v0;
                                            let a855 = 12u64;
                                            if nat_eq(a854, a855) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a856 = v0;
                                                let a857 = 13u64;
                                                if nat_eq(a856, a857) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a858 = v0;
                                                    let a859 = 14u64;
                                                    if nat_eq(a858, a859) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a860 = v0;
                                                        let a861 = 15u64;
                                                        if nat_eq(a860, a861) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a862 = v0;
                                                            let a863 = 16u64;
                                                            if nat_eq(a862, a863) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a864 = v0;
                                                                let a865 = 17u64;
                                                                if nat_eq(a864, a865) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a866 = v0;
                                                                    let a867 = 18u64;
                                                                    if nat_eq(a866, a867) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a868 = v0;
                                                                        let a869 = 19u64;
                                                                        if nat_eq(a868, a869) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a870 = v0;
                                                                            let a871 = 20u64;
                                                                            if nat_eq(a870, a871) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a872 = v0;
                                                                                let a873 = 21u64;
                                                                                if nat_eq(a872, a873) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a874 = v0;
                                                                                    let a875 = 22u64;
                                                                                    if nat_eq(a874, a875) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a876 = v0;
                                                                                        let a877 = 23u64;
                                                                                        if nat_eq(a876, a877) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a878 = v0;
                                                                                            let a879 = 24u64;
                                                                                            if nat_eq(a878, a879) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a880 = v0;
                                                                                                let a881 = 25u64;
                                                                                                if nat_eq(a880, a881) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a882 = v0;
                                                                                                    let a883 = 26u64;
                                                                                                    if nat_eq(a882, a883) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a884 = v0;
                                                                                                        let a885 = 27u64;
                                                                                                        if nat_eq(a884, a885) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a886 = v0;
                                                                                                            let a887 = 28u64;
                                                                                                            if nat_eq(a886, a887) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a888 = v0;
                                                                                                                let a889 = 29u64;
                                                                                                                if nat_eq(a888, a889) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a890 = v0;
                                                                                                                    let a891 = 30u64;
                                                                                                                    if nat_eq(a890, a891) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a892 = v0;
                                                                                                                        let a893 = 34u64;
                                                                                                                        if nat_eq(a892, a893) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a894 = v0;
                                                                                                                            let a895 = 35u64;
                                                                                                                            if nat_eq(a894, a895) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a896 = v0;
                                                                                                                                let a897 = 39u64;
                                                                                                                                if nat_eq(a896, a897) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a898 = v0;
                                                                                                                                    let a899 = 40u64;
                                                                                                                                    if nat_eq(a898, a899) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a900 = v0;
                                                                                                                                        let a901 = 41u64;
                                                                                                                                        if nat_eq(a900, a901) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a902 = v0;
                                                                                                                                            let a903 = 42u64;
                                                                                                                                            if nat_eq(a902, a903) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a904 = v0;
                                                                                                                                                let a905 = 43u64;
                                                                                                                                                if nat_eq(a904, a905) {
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

pub fn f36(v0: u64) -> bool {
    if f32(v0) {
        true
    } else {
        f33(v0)
    }
}

pub fn f37(v0: u64) -> bool {
    let a906 = v0;
    let a907 = 41u64;
    if nat_eq(a906, a907) {
        true
    } else {
        let a908 = v0;
        let a909 = 42u64;
        nat_eq(a908, a909)
    }
}

pub fn run(p0: Adt0) -> R<Adt6> {
    f0(p0)
}
