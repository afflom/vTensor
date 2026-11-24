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
    C11(Adt6, u64, u64),
    C12(Adt6, u64, u64, u64),
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
    C0(Adt1, Adt1, Adt1, Adt1, Adt1, Adt1, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
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
    C12(Adt6),
    C13(u64, Option<u64>),
    C14(u64, u64, u64, u64, u64, u64, Option<u64>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt7> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54) => {
            f6(v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, None::<Adt1>, v54)
        }
        Adt0::C1(v83, v84, v85, v86) => {
            f12(v83, v84, v85, v86)
        }
        Adt0::C2(v71, v72, v73, v74, v75, v76, v77, v78) => {
            f10(v71, v72, v73, v74, v75, v76, v77, v78)
        }
        Adt0::C3(v79, v80, v81, v82) => {
            f11(v79, v80, v81, v82)
        }
        Adt0::C4(v55, v56, v57, v58, v59, v60, v61, v62) => {
            f7(v55, v56, v57, v58, v59, v60, v61, v62)
        }
        Adt0::C5(v63, v64, v65, v66) => {
            f8(v63, v64, v65, v66)
        }
        Adt0::C6(v67, v68, v69, v70) => {
            f9(v67, v68, v69, v70)
        }
        Adt0::C7(v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43) => {
            f5(v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43)
        }
        Adt0::C8(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
            let a8 = v15;
            let a9 = 83u64;
            if nat_eq(a8, a9) {
                f4(v16, v17, v21, v22, v23, v24, v25, v26, v27)
            } else {
                let m10 = v26.clone();
                match m10 {
                    None => {
                        f5(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v27)
                    }
                    Some(_) => {
                        let m11 = v21;
                        match m11 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v29) => {
                                let m12 = v22;
                                match m12 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v30) => {
                                        let m13 = v23;
                                        match m13 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v31) => {
                                                f6(v15, v16, v18, v19, v20, v29, v30, v31, v24, v25, v26, v27)
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
        Adt0::C9(v6, v7) => {
            let a2 = v6;
            let a3 = 2147483648u64;
            if nat_lt(a2, a3) {
                let a4 = v6;
                let a5 = v7;
                if nat_lt(a4, a5) {
                    Ok(Adt7::C9(v6))
                } else {
                    Ok(Adt7::C0)
                }
            } else {
                let a6 = v6;
                let a7 = 4294967296u64;
                if nat_lt(a6, a7) {
                    Ok(Adt7::C8)
                } else {
                    Ok(Adt7::C0)
                }
            }
        }
        Adt0::C10(v1, v2, v3, v4, v5) => {
            f1(v1, v2, v3, v4, v5)
        }
        Adt0::C11(v8, v9, v10) => {
            f2(v8, v9, v10)
        }
        Adt0::C12(v11, v12, v13, v14) => {
            f3(v11, v12, v13, v14)
        }
    }
}

pub fn f1(v0: Adt5, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt7> {
    let a17 = v1;
    let a18 = {
        let m16 = match v0.clone() {
            Adt5::C0(h15, _, _, _, _) => {
                h15
            }
        };
        match m16 {
            Adt1::C0(_, _, _, h14, _, _, _, _, _, _, _) => {
                h14
            }
        }
    };
    if if if if nat_lt(a17, a18) {
        let a22 = v2;
        let a23 = {
            let m21 = match v0.clone() {
                Adt5::C0(h20, _, _, _, _) => {
                    h20
                }
            };
            match m21 {
                Adt1::C0(_, _, h19, _, _, _, _, _, _, _, _) => {
                    h19
                }
            }
        };
        nat_lt(a22, a23)
    } else {
        false
    } {
        let a27 = v3;
        let a28 = {
            let m26 = match v0.clone() {
                Adt5::C0(h25, _, _, _, _) => {
                    h25
                }
            };
            match m26 {
                Adt1::C0(_, _, _, _, h24, _, _, _, _, _, _) => {
                    h24
                }
            }
        };
        nat_lt(a27, a28)
    } else {
        false
    } {
        let a32 = v4;
        let a33 = {
            let m31 = match v0.clone() {
                Adt5::C0(_, h30, _, _, _) => {
                    h30
                }
            };
            match m31 {
                Adt1::C0(_, _, _, h29, _, _, _, _, _, _, _) => {
                    h29
                }
            }
        };
        nat_lt(a32, a33)
    } else {
        false
    } {
        Ok(Adt7::C11(f13(match v0.clone() {
            Adt5::C0(h34, _, _, _, _) => {
                h34
            }
        }, v2, v1, v3)?, f13(match v0.clone() {
            Adt5::C0(_, h35, _, _, _) => {
                h35
            }
        }, 0u64, v4, v3)?, f13(match v0.clone() {
            Adt5::C0(_, _, h36, _, _) => {
                h36
            }
        }, v1, 0u64, v3)?, f13(match v0.clone() {
            Adt5::C0(_, _, _, h37, _) => {
                h37
            }
        }, v1, 0u64, {
            let a41 = v3;
            let a42 = {
                let m40 = match v0.clone() {
                    Adt5::C0(_, _, _, h39, _) => {
                        h39
                    }
                };
                match m40 {
                    Adt1::C0(_, _, _, _, h38, _, _, _, _, _, _) => {
                        h38
                    }
                }
            };
            let a43 = 0u64;
            nat_rem(a41, a42, a43)
        })?, f13(match v0 {
            Adt5::C0(_, _, _, _, h44) => {
                h44
            }
        }, v1, 0u64, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f2(v0: Adt6, v1: u64, v2: u64) -> R<Adt7> {
    let v3: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h45, _, _, _, _, _, _, _, _) => {
            h45
        }
    };
    let a47 = v1;
    let a48 = match v3.clone() {
        Adt1::C0(_, _, h46, _, _, _, _, _, _, _, _) => {
            h46
        }
    };
    if if nat_lt(a47, a48) {
        let a50 = v2;
        let a51 = match v3.clone() {
            Adt1::C0(_, _, _, _, h49, _, _, _, _, _, _) => {
                h49
            }
        };
        nat_lt(a50, a51)
    } else {
        false
    } {
        Ok(Adt7::C13(f13(match v0.clone() {
            Adt6::C0(_, _, _, _, _, h52, _, _, _, _, _) => {
                h52
            }
        }, 0u64, v1, v2)?, {
            let a54 = match v3.clone() {
                Adt1::C0(_, _, _, h53, _, _, _, _, _, _, _) => {
                    h53
                }
            };
            let a55 = 0u64;
            if if nat_eq(a54, a55) {
                let a57 = match v0 {
                    Adt6::C0(_, _, _, _, _, _, h56, _, _, _, _) => {
                        h56
                    }
                };
                let a58 = 1u64;
                nat_eq(a57, a58)
            } else {
                false
            } {
                Some(f14(v3, v1, v2, 0u64)?)
            } else {
                None::<u64>
            }
        }))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f3(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
            h59
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(_, h60, _, _, _, _, _, _, _, _, _) => {
            h60
        }
    };
    let v6: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h61, _, _, _, _, _, _, _, _) => {
            h61
        }
    };
    let a63 = v1;
    let a64 = match v6.clone() {
        Adt1::C0(_, _, h62, _, _, _, _, _, _, _, _) => {
            h62
        }
    };
    if if if nat_lt(a63, a64) {
        let a66 = v2;
        let a67 = match v6.clone() {
            Adt1::C0(_, _, _, _, h65, _, _, _, _, _, _) => {
                h65
            }
        };
        nat_lt(a66, a67)
    } else {
        false
    } {
        let a69 = v3;
        let a70 = match v6.clone() {
            Adt1::C0(_, _, _, h68, _, _, _, _, _, _, _) => {
                h68
            }
        };
        nat_lt(a69, a70)
    } else {
        false
    } {
        let v7: u64 = {
            let a74 = match v6.clone() {
                Adt1::C0(_, _, _, h71, _, _, _, _, _, _, _) => {
                    h71
                }
            };
            let a75 = {
                let a72 = v3;
                let a73 = 1u64;
                nat_add(a72, a73)?
            };
            nat_sub(a74, a75)
        };
        Ok(Adt7::C14(f13(v4.clone(), {
            let a77 = v1;
            let a78 = match v4.clone() {
                Adt1::C0(_, _, h76, _, _, _, _, _, _, _, _) => {
                    h76
                }
            };
            let a79 = 0u64;
            nat_rem(a77, a78, a79)
        }, v3, {
            let a85 = v2;
            let a86 = {
                let a82 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                        h80
                    }
                };
                let a83 = match v4 {
                    Adt1::C0(_, _, _, _, h81, _, _, _, _, _, _) => {
                        h81
                    }
                };
                let a84 = 0u64;
                nat_quot(a82, a83, a84)
            };
            let a87 = 0u64;
            nat_quot(a85, a86, a87)
        })?, f13(v5.clone(), {
            let a89 = v1;
            let a90 = match v5.clone() {
                Adt1::C0(_, _, h88, _, _, _, _, _, _, _, _) => {
                    h88
                }
            };
            let a91 = 0u64;
            nat_rem(a89, a90, a91)
        }, v3, {
            let a97 = v2;
            let a98 = {
                let a94 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h92, _, _, _, _, _, _) => {
                        h92
                    }
                };
                let a95 = match v5 {
                    Adt1::C0(_, _, _, _, h93, _, _, _, _, _, _) => {
                        h93
                    }
                };
                let a96 = 0u64;
                nat_quot(a94, a95, a96)
            };
            let a99 = 0u64;
            nat_quot(a97, a98, a99)
        })?, f13(v6.clone(), v1, v3, v2)?, f13(match v0.clone() {
            Adt6::C0(_, _, _, h100, _, _, _, _, _, _, _) => {
                h100
            }
        }, v1, v3, v2)?, f13(match v0.clone() {
            Adt6::C0(_, _, _, _, h101, _, _, _, _, _, _) => {
                h101
            }
        }, v1, v3, v2)?, {
            let a115 = 4u64;
            let a116 = {
                let a113 = match v6.clone() {
                    Adt1::C0(_, h102, _, _, _, _, _, _, _, _, _) => {
                        h102
                    }
                };
                let a114 = {
                    let a111 = v1;
                    let a112 = {
                        let a109 = match v6.clone() {
                            Adt1::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                                h103
                            }
                        };
                        let a110 = {
                            let a107 = v3;
                            let a108 = {
                                let a105 = match v6.clone() {
                                    Adt1::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                                        h104
                                    }
                                };
                                let a106 = v2;
                                nat_mul(a105, a106)?
                            };
                            nat_add(a107, a108)?
                        };
                        nat_mul(a109, a110)?
                    };
                    nat_add(a111, a112)?
                };
                nat_mul(a113, a114)?
            };
            nat_mul(a115, a116)?
        }, {
            let a118 = v7;
            let a119 = match v0 {
                Adt6::C0(_, _, _, _, _, _, h117, _, _, _, _) => {
                    h117
                }
            };
            if nat_lt(a118, a119) {
                Some(f14(v6, v1, v2, v7)?)
            } else {
                None::<u64>
            }
        }))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f4(v0: bool, v1: u64, v2: Option<Adt1>, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Option<Adt1>, v8: Adt1) -> R<Adt7> {
    if v0 {
        Ok(Adt7::C0)
    } else {
        let m120 = v2;
        match m120 {
            None => {
                Ok(Adt7::C0)
            }
            Some(v9) => {
                let m121 = v3;
                match m121 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v10) => {
                        let m122 = v4;
                        match m122 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v11) => {
                                let m123 = v5;
                                match m123 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v12) => {
                                        let m124 = v6;
                                        match m124 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v13) => {
                                                let m125 = v7;
                                                match m125 {
                                                    None => {
                                                        Ok(Adt7::C0)
                                                    }
                                                    Some(v14) => {
                                                        f15(v1, v9, v10, v11, v12, v13, v14, v8)
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

pub fn f5(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: u64, v6: Option<Adt1>, v7: Option<Adt1>, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt7> {
    let m126 = v6;
    match m126 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a127 = v0;
            let a128 = 43u64;
            if nat_eq(a127, a128) {
                let m129 = v9;
                match m129 {
                    None => {
                        let m130 = v10;
                        match m130 {
                            None => {
                                f10(v0, v1, v2, v3, v12, v7, v8, v11)
                            }
                            Some(_) => {
                                Ok(Adt7::C0)
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Adt7::C0)
                    }
                }
            } else {
                let m131 = v7;
                match m131 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v15) => {
                        let a132 = v0;
                        let a133 = 84u64;
                        if nat_eq(a132, a133) {
                            f16(v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let a134 = v0;
                            let a135 = 88u64;
                            if nat_eq(a134, a135) {
                                f7(v0, v1, v12, v15, v8, v9, v10, v11)
                            } else {
                                let m136 = v8;
                                match m136 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v16) => {
                                        f6(v0, v1, v3, v4, v5, v12, v15, v16, v9, v10, None::<Adt1>, v11)
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

pub fn f6(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt7> {
    let m137 = f17(v10, v8.clone(), v5.clone(), v6.clone(), v7.clone(), v11.clone())?;
    match m137 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a138 = v0;
            let a139 = 71u64;
            if if if if if if if if if if if if if if if if if if nat_eq(a138, a139) {
                !v1
            } else {
                false
            } {
                f18(v2, match v5.clone() {
                    Adt1::C0(_, _, _, h140, _, _, _, _, _, _, _) => {
                        h140
                    }
                }, v8.clone())
            } else {
                false
            } {
                let a141 = v4;
                let a142 = 0u64;
                if if nat_eq(a141, a142) {
                    true
                } else {
                    let a143 = v4;
                    let a144 = 10u64;
                    nat_eq(a143, a144)
                } {
                    true
                } else {
                    let a145 = v4;
                    let a146 = 15u64;
                    nat_eq(a145, a146)
                }
            } else {
                false
            } {
                let a148 = match v5.clone() {
                    Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _) => {
                        h147
                    }
                };
                let a149 = 0u64;
                nat_eq(a148, a149)
            } else {
                false
            } {
                let a151 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h150, _, _, _, _, _) => {
                        h150
                    }
                };
                let a152 = 4u64;
                nat_eq(a151, a152)
            } else {
                false
            } {
                f19(v11.clone())?
            } else {
                false
            } {
                f20(v6.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                f20(v7.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                let a155 = match v5.clone() {
                    Adt1::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                        h153
                    }
                };
                let a156 = match v6.clone() {
                    Adt1::C0(_, h154, _, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                };
                nat_eq(a155, a156)
            } else {
                false
            } {
                let a159 = match v6.clone() {
                    Adt1::C0(_, _, h157, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                };
                let a160 = match v7.clone() {
                    Adt1::C0(_, _, h158, _, _, _, _, _, _, _, _) => {
                        h158
                    }
                };
                nat_eq(a159, a160)
            } else {
                false
            } {
                let a163 = match v11.clone() {
                    Adt1::C0(_, h161, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                };
                let a164 = match v7.clone() {
                    Adt1::C0(_, h162, _, _, _, _, _, _, _, _, _) => {
                        h162
                    }
                };
                nat_eq(a163, a164)
            } else {
                false
            } {
                let a167 = match v11.clone() {
                    Adt1::C0(_, _, h165, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                };
                let a168 = match v5.clone() {
                    Adt1::C0(_, _, _, h166, _, _, _, _, _, _, _) => {
                        h166
                    }
                };
                nat_eq(a167, a168)
            } else {
                false
            } {
                let a171 = match v11.clone() {
                    Adt1::C0(_, _, _, h169, _, _, _, _, _, _, _) => {
                        h169
                    }
                };
                let a172 = match v5.clone() {
                    Adt1::C0(_, _, h170, _, _, _, _, _, _, _, _) => {
                        h170
                    }
                };
                nat_eq(a171, a172)
            } else {
                false
            } {
                let a175 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h173, _, _, _, _, _, _) => {
                        h173
                    }
                };
                let a176 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h174, _, _, _, _, _, _) => {
                        h174
                    }
                };
                nat_eq(a175, a176)
            } else {
                false
            } {
                f21(v5.clone(), v11.clone())?
            } else {
                false
            } {
                f22(v8.clone(), v5.clone(), v12, v11.clone())?
            } else {
                false
            } {
                f23(v9.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                Ok(Adt7::C1(Adt2::C0(v5, v6, v7, v8, v9, v11, v12, f24(v2), f25(v3))))
            } else {
                Ok(Adt7::C0)
            }
        }
    }
}

pub fn f7(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    let v8: u64 = {
        let a178 = match v2.clone() {
            Adt1::C0(h177, _, _, _, _, _, _, _, _, _, _) => {
                h177
            }
        };
        let a179 = 1u64;
        if nat_eq(a178, a179) {
            2u64
        } else {
            4u64
        }
    };
    let a180 = v0;
    let a181 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a180, a181) {
        !v1
    } else {
        false
    } {
        let a183 = match v2.clone() {
            Adt1::C0(h182, _, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        let a184 = 0u64;
        if nat_eq(a183, a184) {
            true
        } else {
            let a186 = match v2.clone() {
                Adt1::C0(h185, _, _, _, _, _, _, _, _, _, _) => {
                    h185
                }
            };
            let a187 = 1u64;
            nat_eq(a186, a187)
        }
    } else {
        false
    } {
        let a189 = match v3.clone() {
            Adt1::C0(h188, _, _, _, _, _, _, _, _, _, _) => {
                h188
            }
        };
        let a190 = 26u64;
        nat_eq(a189, a190)
    } else {
        false
    } {
        let a193 = match v7.clone() {
            Adt1::C0(h191, _, _, _, _, _, _, _, _, _, _) => {
                h191
            }
        };
        let a194 = match v2.clone() {
            Adt1::C0(h192, _, _, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        nat_eq(a193, a194)
    } else {
        false
    } {
        let a196 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h195, _, _, _, _, _) => {
                h195
            }
        };
        let a197 = v8;
        nat_eq(a196, a197)
    } else {
        false
    } {
        let a199 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h198, _, _, _, _, _) => {
                h198
            }
        };
        let a200 = v8;
        nat_eq(a199, a200)
    } else {
        false
    } {
        let a203 = match v2.clone() {
            Adt1::C0(_, h201, _, _, _, _, _, _, _, _, _) => {
                h201
            }
        };
        let a204 = match v7.clone() {
            Adt1::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                h202
            }
        };
        nat_eq(a203, a204)
    } else {
        false
    } {
        f26(v3.clone(), v7.clone())
    } else {
        false
    } {
        f27({
            let a206 = v8;
            let a207 = match v7.clone() {
                Adt1::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
                    h205
                }
            };
            nat_mul(a206, a207)?
        }, match v7.clone() {
            Adt1::C0(_, _, h208, _, _, _, _, _, _, _, _) => {
                h208
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h209, _, _, _, _, _, _, _) => {
                h209
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h210, _, _, _, _, _, _) => {
                h210
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h211, _, _, _, _) => {
                h211
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h212, _, _, _) => {
                h212
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h213, _, _) => {
                h213
            }
        })?
    } else {
        false
    } {
        f21(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f21(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f28(match v3.clone() {
            Adt1::C0(_, _, h214, _, _, _, _, _, _, _, _) => {
                h214
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h215, _, _, _, _, _, _, _, _) => {
                h215
            }
        })
    } else {
        false
    } {
        f28(match v3.clone() {
            Adt1::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                h216
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h217, _, _, _, _, _, _, _) => {
                h217
            }
        })
    } else {
        false
    } {
        f28(match v3.clone() {
            Adt1::C0(_, _, _, _, h218, _, _, _, _, _, _) => {
                h218
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h219, _, _, _, _, _, _) => {
                h219
            }
        })
    } else {
        false
    } {
        let m220 = v4.clone();
        match m220 {
            None => {
                true
            }
            Some(v9) => {
                let a222 = match v9.clone() {
                    Adt1::C0(h221, _, _, _, _, _, _, _, _, _, _) => {
                        h221
                    }
                };
                let a223 = 0u64;
                if if nat_eq(a222, a223) {
                    f26(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f21(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m224 = v5.clone();
        match m224 {
            None => {
                true
            }
            Some(v10) => {
                let a226 = match v10.clone() {
                    Adt1::C0(h225, _, _, _, _, _, _, _, _, _, _) => {
                        h225
                    }
                };
                let a227 = 26u64;
                if if if if nat_eq(a226, a227) {
                    let a229 = match v10.clone() {
                        Adt1::C0(_, _, _, h228, _, _, _, _, _, _, _) => {
                            h228
                        }
                    };
                    let a230 = 1u64;
                    nat_eq(a229, a230)
                } else {
                    false
                } {
                    let a232 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h231, _, _, _, _, _, _) => {
                            h231
                        }
                    };
                    let a233 = 1u64;
                    nat_eq(a232, a233)
                } else {
                    false
                } {
                    f21(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m234 = v4.clone();
                    match m234 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a237 = match v11 {
                                Adt1::C0(_, h235, _, _, _, _, _, _, _, _, _) => {
                                    h235
                                }
                            };
                            let a238 = match v10 {
                                Adt1::C0(_, _, h236, _, _, _, _, _, _, _, _) => {
                                    h236
                                }
                            };
                            nat_eq(a237, a238)
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
        let m239 = v6.clone();
        match m239 {
            None => {
                true
            }
            Some(v12) => {
                let a241 = match v12.clone() {
                    Adt1::C0(h240, _, _, _, _, _, _, _, _, _, _) => {
                        h240
                    }
                };
                let a242 = 26u64;
                if if nat_eq(a241, a242) {
                    f26(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f21(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt7::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a243 = v8;
            let a244 = 2u64;
            if nat_eq(a243, a244) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m245 = v5;
            match m245 {
                None => {
                    let m246 = v4;
                    match m246 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                                    h247
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h248, _, _, _, _, _, _, _, _, _) => {
                                    h248
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h249, _, _, _, _, _, _, _, _) => {
                            h249
                        }
                    }
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f8(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt4::C0(h250, _, _, _, _, _, _, _, _) => {
            h250
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h251, _, _, _, _, _, _, _) => {
            h251
        }
    };
    let a253 = v1;
    let a254 = match v5.clone() {
        Adt1::C0(_, _, h252, _, _, _, _, _, _, _, _) => {
            h252
        }
    };
    if if if nat_lt(a253, a254) {
        let a256 = v2;
        let a257 = match v5.clone() {
            Adt1::C0(_, _, _, h255, _, _, _, _, _, _, _) => {
                h255
            }
        };
        nat_lt(a256, a257)
    } else {
        false
    } {
        let a259 = v3;
        let a260 = match v5.clone() {
            Adt1::C0(_, _, _, _, h258, _, _, _, _, _, _) => {
                h258
            }
        };
        nat_lt(a259, a260)
    } else {
        false
    } {
        Ok(Adt7::C6(f13(v4.clone(), {
            let a262 = v1;
            let a263 = match v4.clone() {
                Adt1::C0(_, _, h261, _, _, _, _, _, _, _, _) => {
                    h261
                }
            };
            let a264 = 0u64;
            nat_rem(a262, a263, a264)
        }, {
            let a266 = v2;
            let a267 = match v4.clone() {
                Adt1::C0(_, _, _, h265, _, _, _, _, _, _, _) => {
                    h265
                }
            };
            let a268 = 0u64;
            nat_rem(a266, a267, a268)
        }, {
            let a270 = v3;
            let a271 = match v4 {
                Adt1::C0(_, _, _, _, h269, _, _, _, _, _, _) => {
                    h269
                }
            };
            let a272 = 0u64;
            nat_rem(a270, a271, a272)
        })?, f13(v5, v1, v2, v3)?, {
            let m274 = match v0.clone() {
                Adt4::C0(_, _, h273, _, _, _, _, _, _) => {
                    h273
                }
            };
            match m274 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f13(v6, v1, v2, v3)?
                }
            }
        }, {
            let m276 = match v0.clone() {
                Adt4::C0(_, _, _, _, h275, _, _, _, _) => {
                    h275
                }
            };
            match m276 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f13(v7, v1, v2, v3)?
                }
            }
        }, f13(match v0 {
            Adt4::C0(_, _, _, _, _, h277, _, _, _) => {
                h277
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f9(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a278 = v1;
    let a279 = 0u64;
    if nat_eq(a278, a279) {
        let a283 = v2;
        let a284 = {
            let m282 = match v0.clone() {
                Adt4::C0(_, h281, _, _, _, _, _, _, _) => {
                    h281
                }
            };
            match m282 {
                Adt1::C0(_, h280, _, _, _, _, _, _, _, _, _) => {
                    h280
                }
            }
        };
        if nat_lt(a283, a284) {
            Ok(Adt7::C7({
                let a288 = v2;
                let a289 = {
                    let m287 = match v0 {
                        Adt4::C0(_, h286, _, _, _, _, _, _, _) => {
                            h286
                        }
                    };
                    match m287 {
                        Adt1::C0(_, _, _, _, _, h285, _, _, _, _, _) => {
                            h285
                        }
                    }
                };
                nat_mul(a288, a289)?
            }))
        } else {
            Ok(Adt7::C8)
        }
    } else {
        let a290 = v1;
        let a291 = 1u64;
        if nat_eq(a290, a291) {
            let m293 = match v0 {
                Adt4::C0(_, _, _, _, h292, _, _, _, _) => {
                    h292
                }
            };
            match m293 {
                None => {
                    Ok(Adt7::C8)
                }
                Some(v4) => {
                    let a295 = v2;
                    let a296 = match v4.clone() {
                        Adt1::C0(_, h294, _, _, _, _, _, _, _, _, _) => {
                            h294
                        }
                    };
                    if nat_lt(a295, a296) {
                        Ok(Adt7::C7({
                            let a298 = v2;
                            let a299 = match v4 {
                                Adt1::C0(_, _, _, _, _, h297, _, _, _, _, _) => {
                                    h297
                                }
                            };
                            nat_mul(a298, a299)?
                        }))
                    } else {
                        Ok(Adt7::C8)
                    }
                }
            }
        } else {
            let a300 = v1;
            let a301 = 2u64;
            if nat_eq(a300, a301) {
                let a302 = v2;
                let a303 = 2147483648u64;
                if if nat_lt(a302, a303) {
                    let a305 = v2;
                    let a306 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h304) => {
                            h304
                        }
                    };
                    nat_lt(a305, a306)
                } else {
                    false
                } {
                    let m308 = match v0 {
                        Adt4::C0(_, _, h307, _, _, _, _, _, _) => {
                            h307
                        }
                    };
                    match m308 {
                        None => {
                            Ok(Adt7::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt7::C7({
                                let a310 = v2;
                                let a311 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h309, _, _, _, _, _) => {
                                        h309
                                    }
                                };
                                nat_mul(a310, a311)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt7::C8)
                }
            } else {
                let a312 = v1;
                let a313 = 3u64;
                if nat_eq(a312, a313) {
                    let m315 = match v0 {
                        Adt4::C0(_, _, _, h314, _, _, _, _, _) => {
                            h314
                        }
                    };
                    match m315 {
                        None => {
                            Ok(Adt7::C8)
                        }
                        Some(v6) => {
                            let a317 = v2;
                            let a318 = match v6.clone() {
                                Adt1::C0(_, _, h316, _, _, _, _, _, _, _, _) => {
                                    h316
                                }
                            };
                            if if nat_lt(a317, a318) {
                                let a320 = v3;
                                let a321 = match v6.clone() {
                                    Adt1::C0(_, h319, _, _, _, _, _, _, _, _, _) => {
                                        h319
                                    }
                                };
                                nat_lt(a320, a321)
                            } else {
                                false
                            } {
                                Ok(Adt7::C7({
                                    let a328 = {
                                        let a323 = v2;
                                        let a324 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h322, _, _, _, _) => {
                                                h322
                                            }
                                        };
                                        nat_mul(a323, a324)?
                                    };
                                    let a329 = {
                                        let a326 = v3;
                                        let a327 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h325, _, _, _, _, _) => {
                                                h325
                                            }
                                        };
                                        nat_mul(a326, a327)?
                                    };
                                    nat_add(a328, a329)?
                                }))
                            } else {
                                Ok(Adt7::C8)
                            }
                        }
                    }
                } else {
                    let a330 = v1;
                    let a331 = 4u64;
                    if nat_eq(a330, a331) {
                        let a332 = v2;
                        let a333 = 2147483648u64;
                        if if nat_lt(a332, a333) {
                            let a337 = v2;
                            let a338 = {
                                let m336 = match v0.clone() {
                                    Adt4::C0(h335, _, _, _, _, _, _, _, _) => {
                                        h335
                                    }
                                };
                                match m336 {
                                    Adt1::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                                        h334
                                    }
                                }
                            };
                            nat_lt(a337, a338)
                        } else {
                            false
                        } {
                            Ok(Adt7::C7({
                                let a340 = v2;
                                let a341 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h339, _, _) => {
                                        h339
                                    }
                                };
                                nat_mul(a340, a341)?
                            }))
                        } else {
                            Ok(Adt7::C8)
                        }
                    } else {
                        Ok(Adt7::C0)
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    if v1 {
        Ok(Adt7::C0)
    } else {
        let a342 = v0;
        let a343 = 43u64;
        if if if if if if if if nat_eq(a342, a343) {
            let a345 = match v4.clone() {
                Adt1::C0(h344, _, _, _, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            let a346 = 0u64;
            if nat_eq(a345, a346) {
                f29(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a348 = match v7.clone() {
                Adt1::C0(h347, _, _, _, _, _, _, _, _, _, _) => {
                    h347
                }
            };
            let a349 = 0u64;
            if nat_eq(a348, a349) {
                f29(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f30(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f21(v4.clone(), v7.clone())? {
                true
            } else {
                f31(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f32(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f33(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f18(v3, match v4.clone() {
                Adt1::C0(_, _, _, h350, _, _, _, _, _, _, _) => {
                    h350
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt7::C3(Adt3::C0(v4, v5, v7, v2, f24(v3))))
        } else {
            Ok(Adt7::C0)
        }
    }
}

pub fn f11(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h351, _, _, _, _) => {
            h351
        }
    };
    let a353 = v1;
    let a354 = match v4.clone() {
        Adt1::C0(_, _, h352, _, _, _, _, _, _, _, _) => {
            h352
        }
    };
    if if if nat_lt(a353, a354) {
        let a356 = v2;
        let a357 = match v4.clone() {
            Adt1::C0(_, _, _, h355, _, _, _, _, _, _, _) => {
                h355
            }
        };
        nat_lt(a356, a357)
    } else {
        false
    } {
        let a359 = v3;
        let a360 = match v4.clone() {
            Adt1::C0(_, _, _, _, h358, _, _, _, _, _, _) => {
                h358
            }
        };
        nat_lt(a359, a360)
    } else {
        false
    } {
        let v6: u64 = {
            let m362 = match v0.clone() {
                Adt3::C0(_, h361, _, _, _) => {
                    h361
                }
            };
            match m362 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f13(v5.clone(), v1, {
                        let a364 = v2;
                        let a365 = match v5.clone() {
                            Adt1::C0(_, _, _, h363, _, _, _, _, _, _, _) => {
                                h363
                            }
                        };
                        let a366 = 0u64;
                        nat_rem(a364, a365, a366)
                    }, {
                        let a368 = v3;
                        let a369 = match v5 {
                            Adt1::C0(_, _, _, _, h367, _, _, _, _, _, _) => {
                                h367
                            }
                        };
                        let a370 = 0u64;
                        nat_rem(a368, a369, a370)
                    })?
                }
            }
        };
        Ok(Adt7::C4(f13(v4, v1, v2, v3)?, v6, {
            let a371 = v2;
            let a372 = 4u64;
            nat_mul(a371, a372)?
        }, f13(match v0 {
            Adt3::C0(_, _, h373, _, _) => {
                h373
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f12(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a377 = v1;
    let a378 = {
        let m376 = match v0.clone() {
            Adt2::C0(h375, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        match m376 {
            Adt1::C0(_, _, h374, _, _, _, _, _, _, _, _) => {
                h374
            }
        }
    };
    if if if nat_lt(a377, a378) {
        let a382 = v2;
        let a383 = {
            let m381 = match v0.clone() {
                Adt2::C0(h380, _, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            match m381 {
                Adt1::C0(_, _, _, h379, _, _, _, _, _, _, _) => {
                    h379
                }
            }
        };
        nat_lt(a382, a383)
    } else {
        false
    } {
        let a387 = v3;
        let a388 = {
            let m386 = match v0.clone() {
                Adt2::C0(h385, _, _, _, _, _, _, _, _) => {
                    h385
                }
            };
            match m386 {
                Adt1::C0(_, _, _, _, h384, _, _, _, _, _, _) => {
                    h384
                }
            }
        };
        nat_lt(a387, a388)
    } else {
        false
    } {
        Ok(Adt7::C2(f13(match v0.clone() {
            Adt2::C0(h389, _, _, _, _, _, _, _, _) => {
                h389
            }
        }, v1, v2, v3)?, f13(match v0.clone() {
            Adt2::C0(_, h390, _, _, _, _, _, _, _) => {
                h390
            }
        }, 0u64, {
            let a400 = v2;
            let a401 = {
                let a397 = {
                    let m393 = match v0.clone() {
                        Adt2::C0(h392, _, _, _, _, _, _, _, _) => {
                            h392
                        }
                    };
                    match m393 {
                        Adt1::C0(_, _, _, h391, _, _, _, _, _, _, _) => {
                            h391
                        }
                    }
                };
                let a398 = {
                    let m396 = match v0.clone() {
                        Adt2::C0(_, h395, _, _, _, _, _, _, _) => {
                            h395
                        }
                    };
                    match m396 {
                        Adt1::C0(_, _, _, h394, _, _, _, _, _, _, _) => {
                            h394
                        }
                    }
                };
                let a399 = 0u64;
                nat_quot(a397, a398, a399)
            };
            let a402 = 0u64;
            nat_quot(a400, a401, a402)
        }, {
            let a412 = v3;
            let a413 = {
                let a409 = {
                    let m405 = match v0.clone() {
                        Adt2::C0(h404, _, _, _, _, _, _, _, _) => {
                            h404
                        }
                    };
                    match m405 {
                        Adt1::C0(_, _, _, _, h403, _, _, _, _, _, _) => {
                            h403
                        }
                    }
                };
                let a410 = {
                    let m408 = match v0.clone() {
                        Adt2::C0(_, h407, _, _, _, _, _, _, _) => {
                            h407
                        }
                    };
                    match m408 {
                        Adt1::C0(_, _, _, _, h406, _, _, _, _, _, _) => {
                            h406
                        }
                    }
                };
                let a411 = 0u64;
                nat_quot(a409, a410, a411)
            };
            let a414 = 0u64;
            nat_quot(a412, a413, a414)
        })?, f13(match v0.clone() {
            Adt2::C0(_, _, h415, _, _, _, _, _, _) => {
                h415
            }
        }, 0u64, {
            let a425 = v2;
            let a426 = {
                let a422 = {
                    let m418 = match v0.clone() {
                        Adt2::C0(h417, _, _, _, _, _, _, _, _) => {
                            h417
                        }
                    };
                    match m418 {
                        Adt1::C0(_, _, _, h416, _, _, _, _, _, _, _) => {
                            h416
                        }
                    }
                };
                let a423 = {
                    let m421 = match v0.clone() {
                        Adt2::C0(_, _, h420, _, _, _, _, _, _) => {
                            h420
                        }
                    };
                    match m421 {
                        Adt1::C0(_, _, _, h419, _, _, _, _, _, _, _) => {
                            h419
                        }
                    }
                };
                let a424 = 0u64;
                nat_quot(a422, a423, a424)
            };
            let a427 = 0u64;
            nat_quot(a425, a426, a427)
        }, {
            let a437 = v3;
            let a438 = {
                let a434 = {
                    let m430 = match v0.clone() {
                        Adt2::C0(h429, _, _, _, _, _, _, _, _) => {
                            h429
                        }
                    };
                    match m430 {
                        Adt1::C0(_, _, _, _, h428, _, _, _, _, _, _) => {
                            h428
                        }
                    }
                };
                let a435 = {
                    let m433 = match v0.clone() {
                        Adt2::C0(_, _, h432, _, _, _, _, _, _) => {
                            h432
                        }
                    };
                    match m433 {
                        Adt1::C0(_, _, _, _, h431, _, _, _, _, _, _) => {
                            h431
                        }
                    }
                };
                let a436 = 0u64;
                nat_quot(a434, a435, a436)
            };
            let a439 = 0u64;
            nat_quot(a437, a438, a439)
        })?, {
            let m441 = match v0.clone() {
                Adt2::C0(_, _, _, h440, _, _, _, _, _) => {
                    h440
                }
            };
            match m441 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f13(v4.clone(), v1, {
                        let a443 = v2;
                        let a444 = match v4.clone() {
                            Adt1::C0(_, _, _, h442, _, _, _, _, _, _, _) => {
                                h442
                            }
                        };
                        let a445 = 0u64;
                        nat_rem(a443, a444, a445)
                    }, {
                        let a447 = v3;
                        let a448 = match v4 {
                            Adt1::C0(_, _, _, _, h446, _, _, _, _, _, _) => {
                                h446
                            }
                        };
                        let a449 = 0u64;
                        nat_rem(a447, a448, a449)
                    })?
                }
            }
        }, {
            let a450 = v2;
            let a451 = 4u64;
            nat_mul(a450, a451)?
        }, f13(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h452, _, _, _) => {
                h452
            }
        }, v2, v1, v3)?, {
            let m455 = match v0.clone() {
                Adt2::C0(_, h454, _, _, _, _, _, _, _) => {
                    h454
                }
            };
            match m455 {
                Adt1::C0(_, _, _, _, _, _, h453, _, _, _, _) => {
                    h453
                }
            }
        }, {
            let m458 = match v0.clone() {
                Adt2::C0(_, _, h457, _, _, _, _, _, _) => {
                    h457
                }
            };
            match m458 {
                Adt1::C0(_, _, _, _, _, _, h456, _, _, _, _) => {
                    h456
                }
            }
        }, f34({
            let m461 = match v0 {
                Adt2::C0(_, h460, _, _, _, _, _, _, _) => {
                    h460
                }
            };
            match m461 {
                Adt1::C0(h459, _, _, _, _, _, _, _, _, _, _) => {
                    h459
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f13(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a473 = {
        let a468 = {
            let a463 = v1;
            let a464 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h462, _, _, _, _) => {
                    h462
                }
            };
            nat_mul(a463, a464)?
        };
        let a469 = {
            let a466 = v2;
            let a467 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h465, _, _, _) => {
                    h465
                }
            };
            nat_mul(a466, a467)?
        };
        nat_add(a468, a469)?
    };
    let a474 = {
        let a471 = v3;
        let a472 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h470, _, _) => {
                h470
            }
        };
        nat_mul(a471, a472)?
    };
    nat_add(a473, a474)
}

pub fn f14(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h475, _, _, _, _, _, _, _, _, _) => {
            h475
        }
    };
    let v5: u64 = match v0.clone() {
        Adt1::C0(_, _, h476, _, _, _, _, _, _, _, _) => {
            h476
        }
    };
    let v6: u64 = match v0.clone() {
        Adt1::C0(_, _, _, h477, _, _, _, _, _, _, _) => {
            h477
        }
    };
    let v7: u64 = match v0 {
        Adt1::C0(_, _, _, _, h478, _, _, _, _, _, _) => {
            h478
        }
    };
    let a499 = 4u64;
    let a500 = {
        let a497 = {
            let a483 = {
                let a479 = v4;
                let a480 = v5;
                nat_mul(a479, a480)?
            };
            let a484 = {
                let a481 = v6;
                let a482 = v7;
                nat_mul(a481, a482)?
            };
            nat_mul(a483, a484)?
        };
        let a498 = {
            let a495 = {
                let a485 = v4;
                let a486 = v4;
                nat_mul(a485, a486)?
            };
            let a496 = {
                let a493 = v1;
                let a494 = {
                    let a491 = v5;
                    let a492 = {
                        let a489 = v2;
                        let a490 = {
                            let a487 = v7;
                            let a488 = v3;
                            nat_mul(a487, a488)?
                        };
                        nat_add(a489, a490)?
                    };
                    nat_mul(a491, a492)?
                };
                nat_add(a493, a494)?
            };
            nat_mul(a495, a496)?
        };
        nat_add(a497, a498)?
    };
    nat_mul(a499, a500)
}

pub fn f15(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt7> {
    let v8: u64 = match v3.clone() {
        Adt1::C0(_, h501, _, _, _, _, _, _, _, _, _) => {
            h501
        }
    };
    let v9: u64 = match v3.clone() {
        Adt1::C0(_, _, h502, _, _, _, _, _, _, _, _) => {
            h502
        }
    };
    let v10: u64 = match v3.clone() {
        Adt1::C0(_, _, _, h503, _, _, _, _, _, _, _) => {
            h503
        }
    };
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, _, _, _, h504, _, _, _, _, _, _) => {
            h504
        }
    };
    let a505 = 0u64;
    let a506 = v8;
    if if if if if if if if if if if if if if if if if if if if if if if nat_lt(a505, a506) {
        let a507 = 0u64;
        let a508 = v9;
        nat_lt(a507, a508)
    } else {
        false
    } {
        let a509 = 0u64;
        let a510 = v11;
        nat_lt(a509, a510)
    } else {
        false
    } {
        let a511 = 0u64;
        let a512 = v0;
        nat_lt(a511, a512)
    } else {
        false
    } {
        let a513 = v0;
        let a514 = 2147483647u64;
        nat_le(a513, a514)
    } else {
        false
    } {
        f35(v1.clone(), v7.clone())?
    } else {
        false
    } {
        f35(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f35(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f35(v4.clone(), v7.clone())?
    } else {
        false
    } {
        f35(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f35(v6.clone(), v7.clone())?
    } else {
        false
    } {
        f36(v1.clone(), v3.clone())
    } else {
        false
    } {
        f36(v2.clone(), v3.clone())
    } else {
        false
    } {
        let a516 = match v4.clone() {
            Adt1::C0(_, h515, _, _, _, _, _, _, _, _, _) => {
                h515
            }
        };
        let a517 = 1u64;
        if nat_eq(a516, a517) {
            true
        } else {
            let a519 = match v4.clone() {
                Adt1::C0(_, h518, _, _, _, _, _, _, _, _, _) => {
                    h518
                }
            };
            let a520 = v8;
            nat_eq(a519, a520)
        }
    } else {
        false
    } {
        let a523 = match v4.clone() {
            Adt1::C0(_, h521, _, _, _, _, _, _, _, _, _) => {
                h521
            }
        };
        let a524 = match v4.clone() {
            Adt1::C0(_, h522, _, _, _, _, _, _, _, _, _) => {
                h522
            }
        };
        if if if nat_eq(a523, a524) {
            let a526 = match v4.clone() {
                Adt1::C0(_, _, h525, _, _, _, _, _, _, _, _) => {
                    h525
                }
            };
            let a527 = v9;
            nat_eq(a526, a527)
        } else {
            false
        } {
            let a529 = match v4.clone() {
                Adt1::C0(_, _, _, h528, _, _, _, _, _, _, _) => {
                    h528
                }
            };
            let a530 = v10;
            nat_eq(a529, a530)
        } else {
            false
        } {
            let a532 = match v4.clone() {
                Adt1::C0(_, _, _, _, h531, _, _, _, _, _, _) => {
                    h531
                }
            };
            let a533 = v11;
            nat_eq(a532, a533)
        } else {
            false
        }
    } else {
        false
    } {
        let a535 = match v5.clone() {
            Adt1::C0(_, h534, _, _, _, _, _, _, _, _, _) => {
                h534
            }
        };
        let a536 = 1u64;
        if if if nat_eq(a535, a536) {
            let a538 = match v5.clone() {
                Adt1::C0(_, _, h537, _, _, _, _, _, _, _, _) => {
                    h537
                }
            };
            let a539 = v9;
            nat_eq(a538, a539)
        } else {
            false
        } {
            let a541 = match v5.clone() {
                Adt1::C0(_, _, _, h540, _, _, _, _, _, _, _) => {
                    h540
                }
            };
            let a542 = v10;
            nat_eq(a541, a542)
        } else {
            false
        } {
            let a544 = match v5.clone() {
                Adt1::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
                    h543
                }
            };
            let a545 = v11;
            nat_eq(a544, a545)
        } else {
            false
        }
    } else {
        false
    } {
        let a547 = match v6.clone() {
            Adt1::C0(_, h546, _, _, _, _, _, _, _, _, _) => {
                h546
            }
        };
        let a548 = v8;
        if if if nat_eq(a547, a548) {
            let a550 = match v6.clone() {
                Adt1::C0(_, _, h549, _, _, _, _, _, _, _, _) => {
                    h549
                }
            };
            let a551 = v8;
            nat_eq(a550, a551)
        } else {
            false
        } {
            let a553 = match v6.clone() {
                Adt1::C0(_, _, _, h552, _, _, _, _, _, _, _) => {
                    h552
                }
            };
            let a554 = v9;
            nat_eq(a553, a554)
        } else {
            false
        } {
            let a556 = match v6.clone() {
                Adt1::C0(_, _, _, _, h555, _, _, _, _, _, _) => {
                    h555
                }
            };
            let a557 = v11;
            nat_eq(a556, a557)
        } else {
            false
        }
    } else {
        false
    } {
        let a561 = match v7.clone() {
            Adt1::C0(_, h558, _, _, _, _, _, _, _, _, _) => {
                h558
            }
        };
        let a562 = {
            let a559 = v8;
            let a560 = v9;
            nat_mul(a559, a560)?
        };
        if if if nat_eq(a561, a562) {
            let a572 = match v7.clone() {
                Adt1::C0(_, _, h563, _, _, _, _, _, _, _, _) => {
                    h563
                }
            };
            let a573 = {
                let a570 = {
                    let a564 = v10;
                    let a565 = v11;
                    nat_mul(a564, a565)?
                };
                let a571 = {
                    let a568 = {
                        let a566 = v0;
                        let a567 = v8;
                        nat_mul(a566, a567)?
                    };
                    let a569 = v11;
                    nat_mul(a568, a569)?
                };
                nat_add(a570, a571)?
            };
            nat_eq(a572, a573)
        } else {
            false
        } {
            let a575 = match v7.clone() {
                Adt1::C0(_, _, _, h574, _, _, _, _, _, _, _) => {
                    h574
                }
            };
            let a576 = 1u64;
            nat_eq(a575, a576)
        } else {
            false
        } {
            let a578 = match v7.clone() {
                Adt1::C0(_, _, _, _, h577, _, _, _, _, _, _) => {
                    h577
                }
            };
            let a579 = 1u64;
            nat_eq(a578, a579)
        } else {
            false
        }
    } else {
        false
    } {
        let a581 = match v7.clone() {
            Adt1::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                h580
            }
        };
        let a582 = 0u64;
        nat_eq(a581, a582)
    } else {
        false
    } {
        f29(v4.clone(), 4u64)?
    } else {
        false
    } {
        f29(v5.clone(), 4u64)?
    } else {
        false
    } {
        f29(v6.clone(), 4u64)?
    } else {
        false
    } {
        f29(v7, 4u64)?
    } else {
        false
    } {
        let v12: u64 = {
            let a583 = v9;
            let a584 = v11;
            nat_mul(a583, a584)?
        };
        let v13: u64 = {
            let a585 = v8;
            let a586 = v8;
            nat_mul(a585, a586)?
        };
        let v14: u64 = {
            let a587 = v10;
            let a588 = 0u64;
            if if nat_eq(a587, a588) {
                let a589 = v0;
                let a590 = 1u64;
                nat_eq(a589, a590)
            } else {
                false
            } {
                1u64
            } else {
                let a591 = v0;
                let a592 = v10;
                if nat_lt(a591, a592) {
                    v0
                } else {
                    v10
                }
            }
        };
        let v15: u64 = {
            let a599 = v12;
            let a600 = {
                let a597 = {
                    let a593 = v10;
                    let a594 = v8;
                    nat_mul(a593, a594)?
                };
                let a598 = {
                    let a595 = v14;
                    let a596 = v13;
                    nat_mul(a595, a596)?
                };
                nat_add(a597, a598)?
            };
            nat_mul(a599, a600)?
        };
        Ok(Adt7::C12(Adt6::C0(v1, v2, v3, v4, v5, v6, v0, v12, v13, {
            let a603 = {
                let a601 = v13;
                let a602 = v8;
                nat_add(a601, a602)?
            };
            let a604 = v8;
            nat_add(a603, a604)?
        }, v15)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f16(v0: bool, v1: Adt1, v2: Adt1, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Adt1) -> R<Adt7> {
    let v8: Adt1 = {
        let m605 = v3.clone();
        match m605 {
            None => {
                v1.clone()
            }
            Some(v7) => {
                v7
            }
        }
    };
    let v10: Adt1 = {
        let m606 = v4.clone();
        match m606 {
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
        let m607 = v5;
        match m607 {
            None => {
                let m608 = v3;
                match m608 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m609 = v4;
                        match m609 {
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
        let a611 = match v1.clone() {
            Adt1::C0(h610, _, _, _, _, _, _, _, _, _, _) => {
                h610
            }
        };
        let a612 = 0u64;
        if nat_eq(a611, a612) {
            let a614 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h613, _, _, _, _, _) => {
                    h613
                }
            };
            let a615 = 4u64;
            nat_eq(a614, a615)
        } else {
            false
        }
    } else {
        false
    } {
        let a617 = match v8.clone() {
            Adt1::C0(h616, _, _, _, _, _, _, _, _, _, _) => {
                h616
            }
        };
        let a618 = 0u64;
        if nat_eq(a617, a618) {
            let a620 = match v8.clone() {
                Adt1::C0(_, _, _, _, _, h619, _, _, _, _, _) => {
                    h619
                }
            };
            let a621 = 4u64;
            nat_eq(a620, a621)
        } else {
            false
        }
    } else {
        false
    } {
        let a623 = match v10.clone() {
            Adt1::C0(h622, _, _, _, _, _, _, _, _, _, _) => {
                h622
            }
        };
        let a624 = 1u64;
        if nat_eq(a623, a624) {
            let a626 = match v10.clone() {
                Adt1::C0(_, _, _, _, _, h625, _, _, _, _, _) => {
                    h625
                }
            };
            let a627 = 2u64;
            nat_eq(a626, a627)
        } else {
            false
        }
    } else {
        false
    } {
        f19(v6.clone())?
    } else {
        false
    } {
        f37(match v2.clone() {
            Adt1::C0(h628, _, _, _, _, _, _, _, _, _, _) => {
                h628
            }
        }, match v2.clone() {
            Adt1::C0(_, h629, _, _, _, _, _, _, _, _, _) => {
                h629
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h630, _, _, _, _, _) => {
                h630
            }
        })
    } else {
        false
    } {
        let a632 = 0u64;
        let a633 = match v1.clone() {
            Adt1::C0(_, h631, _, _, _, _, _, _, _, _, _) => {
                h631
            }
        };
        nat_lt(a632, a633)
    } else {
        false
    } {
        let a635 = 0u64;
        let a636 = match v1.clone() {
            Adt1::C0(_, _, h634, _, _, _, _, _, _, _, _) => {
                h634
            }
        };
        nat_lt(a635, a636)
    } else {
        false
    } {
        let a638 = 0u64;
        let a639 = match v1.clone() {
            Adt1::C0(_, _, _, h637, _, _, _, _, _, _, _) => {
                h637
            }
        };
        nat_lt(a638, a639)
    } else {
        false
    } {
        let a641 = 0u64;
        let a642 = match v1.clone() {
            Adt1::C0(_, _, _, _, h640, _, _, _, _, _, _) => {
                h640
            }
        };
        nat_lt(a641, a642)
    } else {
        false
    } {
        let a644 = 0u64;
        let a645 = match v2.clone() {
            Adt1::C0(_, _, _, h643, _, _, _, _, _, _, _) => {
                h643
            }
        };
        nat_lt(a644, a645)
    } else {
        false
    } {
        let a648 = match v2.clone() {
            Adt1::C0(_, h646, _, _, _, _, _, _, _, _, _) => {
                h646
            }
        };
        let a649 = match v1.clone() {
            Adt1::C0(_, h647, _, _, _, _, _, _, _, _, _) => {
                h647
            }
        };
        nat_eq(a648, a649)
    } else {
        false
    } {
        let a651 = match v2.clone() {
            Adt1::C0(_, _, h650, _, _, _, _, _, _, _, _) => {
                h650
            }
        };
        let a652 = 1u64;
        nat_eq(a651, a652)
    } else {
        false
    } {
        let a655 = match v2.clone() {
            Adt1::C0(_, _, _, _, h653, _, _, _, _, _, _) => {
                h653
            }
        };
        let a656 = match v1.clone() {
            Adt1::C0(_, _, _, _, h654, _, _, _, _, _, _) => {
                h654
            }
        };
        nat_eq(a655, a656)
    } else {
        false
    } {
        let a659 = match v8.clone() {
            Adt1::C0(_, h657, _, _, _, _, _, _, _, _, _) => {
                h657
            }
        };
        let a660 = match v1.clone() {
            Adt1::C0(_, _, h658, _, _, _, _, _, _, _, _) => {
                h658
            }
        };
        nat_eq(a659, a660)
    } else {
        false
    } {
        let a663 = match v8.clone() {
            Adt1::C0(_, _, h661, _, _, _, _, _, _, _, _) => {
                h661
            }
        };
        let a664 = match v1.clone() {
            Adt1::C0(_, _, _, h662, _, _, _, _, _, _, _) => {
                h662
            }
        };
        nat_eq(a663, a664)
    } else {
        false
    } {
        let a666 = match v8.clone() {
            Adt1::C0(_, _, _, h665, _, _, _, _, _, _, _) => {
                h665
            }
        };
        let a667 = 1u64;
        nat_eq(a666, a667)
    } else {
        false
    } {
        let a670 = match v8.clone() {
            Adt1::C0(_, _, _, _, h668, _, _, _, _, _, _) => {
                h668
            }
        };
        let a671 = match v1.clone() {
            Adt1::C0(_, _, _, _, h669, _, _, _, _, _, _) => {
                h669
            }
        };
        nat_eq(a670, a671)
    } else {
        false
    } {
        let a674 = match v10.clone() {
            Adt1::C0(_, h672, _, _, _, _, _, _, _, _, _) => {
                h672
            }
        };
        let a675 = match v2.clone() {
            Adt1::C0(_, _, _, h673, _, _, _, _, _, _, _) => {
                h673
            }
        };
        nat_eq(a674, a675)
    } else {
        false
    } {
        let a678 = match v10.clone() {
            Adt1::C0(_, _, h676, _, _, _, _, _, _, _, _) => {
                h676
            }
        };
        let a679 = match v1.clone() {
            Adt1::C0(_, _, _, h677, _, _, _, _, _, _, _) => {
                h677
            }
        };
        nat_eq(a678, a679)
    } else {
        false
    } {
        let a681 = match v10.clone() {
            Adt1::C0(_, _, _, h680, _, _, _, _, _, _, _) => {
                h680
            }
        };
        let a682 = 1u64;
        nat_eq(a681, a682)
    } else {
        false
    } {
        f28(match v1.clone() {
            Adt1::C0(_, _, _, _, h683, _, _, _, _, _, _) => {
                h683
            }
        }, match v10.clone() {
            Adt1::C0(_, _, _, _, h684, _, _, _, _, _, _) => {
                h684
            }
        })
    } else {
        false
    } {
        let a687 = match v6.clone() {
            Adt1::C0(_, h685, _, _, _, _, _, _, _, _, _) => {
                h685
            }
        };
        let a688 = match v2.clone() {
            Adt1::C0(_, _, _, h686, _, _, _, _, _, _, _) => {
                h686
            }
        };
        nat_eq(a687, a688)
    } else {
        false
    } {
        let a691 = match v6.clone() {
            Adt1::C0(_, _, h689, _, _, _, _, _, _, _, _) => {
                h689
            }
        };
        let a692 = match v1.clone() {
            Adt1::C0(_, _, _, h690, _, _, _, _, _, _, _) => {
                h690
            }
        };
        nat_eq(a691, a692)
    } else {
        false
    } {
        let a694 = match v6.clone() {
            Adt1::C0(_, _, _, h693, _, _, _, _, _, _, _) => {
                h693
            }
        };
        let a695 = 1u64;
        nat_eq(a694, a695)
    } else {
        false
    } {
        let a698 = match v6.clone() {
            Adt1::C0(_, _, _, _, h696, _, _, _, _, _, _) => {
                h696
            }
        };
        let a699 = match v1.clone() {
            Adt1::C0(_, _, _, _, h697, _, _, _, _, _, _) => {
                h697
            }
        };
        nat_eq(a698, a699)
    } else {
        false
    } {
        f21(v1.clone(), v6.clone())?
    } else {
        false
    } {
        f21(v2.clone(), v6.clone())?
    } else {
        false
    } {
        f21(v8.clone(), v6.clone())?
    } else {
        false
    } {
        f21(v10.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt7::C10(Adt5::C0(v1, v2, v8, v10, v6)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f17(v0: Option<Adt1>, v1: Option<Adt1>, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Option<u64>> {
    let m700 = v0;
    match m700 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h701, _, _, _, _, _, _, _, _) => {
                    h701
                }
            }))
        }
        Some(v6) => {
            let m702 = v1;
            match m702 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a704 = match v6.clone() {
                        Adt1::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
                            h703
                        }
                    };
                    let a705 = 26u64;
                    if if if if if if if if if nat_eq(a704, a705) {
                        f29(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a708 = match v6.clone() {
                            Adt1::C0(_, h706, _, _, _, _, _, _, _, _, _) => {
                                h706
                            }
                        };
                        let a709 = match v7 {
                            Adt1::C0(_, h707, _, _, _, _, _, _, _, _, _) => {
                                h707
                            }
                        };
                        nat_eq(a708, a709)
                    } else {
                        false
                    } {
                        let a712 = match v6.clone() {
                            Adt1::C0(_, _, h710, _, _, _, _, _, _, _, _) => {
                                h710
                            }
                        };
                        let a713 = match v2 {
                            Adt1::C0(_, _, _, _, h711, _, _, _, _, _, _) => {
                                h711
                            }
                        };
                        nat_eq(a712, a713)
                    } else {
                        false
                    } {
                        let a715 = match v6.clone() {
                            Adt1::C0(_, _, _, h714, _, _, _, _, _, _, _) => {
                                h714
                            }
                        };
                        let a716 = 1u64;
                        nat_eq(a715, a716)
                    } else {
                        false
                    } {
                        let a718 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h717, _, _, _, _, _, _) => {
                                h717
                            }
                        };
                        let a719 = 1u64;
                        nat_eq(a718, a719)
                    } else {
                        false
                    } {
                        let a721 = match v3 {
                            Adt1::C0(_, _, _, _, h720, _, _, _, _, _, _) => {
                                h720
                            }
                        };
                        let a722 = 1u64;
                        nat_eq(a721, a722)
                    } else {
                        false
                    } {
                        let a724 = match v4 {
                            Adt1::C0(_, _, _, _, h723, _, _, _, _, _, _) => {
                                h723
                            }
                        };
                        let a725 = 1u64;
                        nat_eq(a724, a725)
                    } else {
                        false
                    } {
                        f21(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h726, _, _, _, _, _, _, _, _, _) => {
                                h726
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

pub fn f18(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a727 = v0;
    let a728 = 0u64;
    if if nat_eq(a727, a728) {
        true
    } else {
        let a729 = 2147483648u64;
        let a730 = v0;
        if nat_le(a729, a730) {
            let a731 = v0;
            let a732 = 4286578688u64;
            nat_lt(a731, a732)
        } else {
            false
        }
    } {
        true
    } else {
        let a733 = v0;
        let a734 = 2139095040u64;
        if if nat_lt(a733, a734) {
            let a735 = v1;
            let a736 = 4294967295u64;
            nat_le(a735, a736)
        } else {
            false
        } {
            let m737 = v2;
            match m737 {
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

pub fn f19(v0: Adt1) -> R<bool> {
    let a739 = match v0.clone() {
        Adt1::C0(h738, _, _, _, _, _, _, _, _, _, _) => {
            h738
        }
    };
    let a740 = 0u64;
    if if if if nat_eq(a739, a740) {
        let a742 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h741, _, _, _, _, _) => {
                h741
            }
        };
        let a743 = 4u64;
        nat_eq(a742, a743)
    } else {
        false
    } {
        let a749 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h744, _, _, _, _) => {
                h744
            }
        };
        let a750 = {
            let a747 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h745, _, _, _, _, _) => {
                    h745
                }
            };
            let a748 = match v0.clone() {
                Adt1::C0(_, h746, _, _, _, _, _, _, _, _, _) => {
                    h746
                }
            };
            nat_mul(a747, a748)?
        };
        nat_eq(a749, a750)
    } else {
        false
    } {
        let a756 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h751, _, _, _) => {
                h751
            }
        };
        let a757 = {
            let a754 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h752, _, _, _, _) => {
                    h752
                }
            };
            let a755 = match v0.clone() {
                Adt1::C0(_, _, h753, _, _, _, _, _, _, _, _) => {
                    h753
                }
            };
            nat_mul(a754, a755)?
        };
        nat_eq(a756, a757)
    } else {
        false
    } {
        let a763 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h758, _, _) => {
                h758
            }
        };
        let a764 = {
            let a761 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h759, _, _, _) => {
                    h759
                }
            };
            let a762 = match v0 {
                Adt1::C0(_, _, _, h760, _, _, _, _, _, _, _) => {
                    h760
                }
            };
            nat_mul(a761, a762)?
        };
        Ok(nat_eq(a763, a764))
    } else {
        Ok(false)
    }
}

pub fn f20(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f37(match v0.clone() {
        Adt1::C0(h765, _, _, _, _, _, _, _, _, _, _) => {
            h765
        }
    }, match v0.clone() {
        Adt1::C0(_, h766, _, _, _, _, _, _, _, _, _) => {
            h766
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h767, _, _, _, _, _) => {
            h767
        }
    }) {
        f28(match v1.clone() {
            Adt1::C0(_, _, _, h768, _, _, _, _, _, _, _) => {
                h768
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h769, _, _, _, _, _, _, _) => {
                h769
            }
        })
    } else {
        false
    } {
        f28(match v1 {
            Adt1::C0(_, _, _, _, h770, _, _, _, _, _, _) => {
                h770
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h771, _, _, _, _, _, _) => {
                h771
            }
        })
    } else {
        false
    } {
        f21(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> R<bool> {
    let a773 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h772, _) => {
            h772
        }
    };
    let a774 = 0u64;
    if nat_eq(a773, a774) {
        Ok(true)
    } else {
        let a776 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h775, _) => {
                h775
            }
        };
        let a777 = 0u64;
        if nat_eq(a776, a777) {
            Ok(true)
        } else {
            let a779 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h778) => {
                    h778
                }
            };
            let a780 = 0u64;
            if nat_eq(a779, a780) {
                Ok(true)
            } else {
                let a782 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h781) => {
                        h781
                    }
                };
                let a783 = 0u64;
                if nat_eq(a782, a783) {
                    Ok(true)
                } else {
                    let a789 = {
                        let a786 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h784) => {
                                h784
                            }
                        };
                        let a787 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h785, _) => {
                                h785
                            }
                        };
                        nat_add(a786, a787)?
                    };
                    let a790 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h788) => {
                            h788
                        }
                    };
                    if nat_le(a789, a790) {
                        Ok(true)
                    } else {
                        let a796 = {
                            let a793 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h791) => {
                                    h791
                                }
                            };
                            let a794 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h792, _) => {
                                    h792
                                }
                            };
                            nat_add(a793, a794)?
                        };
                        let a797 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h795) => {
                                h795
                            }
                        };
                        Ok(nat_le(a796, a797))
                    }
                }
            }
        }
    }
}

pub fn f22(v0: Option<Adt1>, v1: Adt1, v2: u64, v3: Adt1) -> R<bool> {
    let m798 = v0;
    match m798 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a800 = match v4.clone() {
                Adt1::C0(h799, _, _, _, _, _, _, _, _, _, _) => {
                    h799
                }
            };
            let a801 = 1u64;
            if if if if if if nat_eq(a800, a801) {
                let a803 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h802, _, _, _, _, _) => {
                        h802
                    }
                };
                let a804 = 2u64;
                nat_eq(a803, a804)
            } else {
                false
            } {
                let a806 = v2;
                let a807 = match v4.clone() {
                    Adt1::C0(_, h805, _, _, _, _, _, _, _, _, _) => {
                        h805
                    }
                };
                nat_le(a806, a807)
            } else {
                false
            } {
                let a810 = match v1.clone() {
                    Adt1::C0(_, _, h808, _, _, _, _, _, _, _, _) => {
                        h808
                    }
                };
                let a811 = match v4.clone() {
                    Adt1::C0(_, _, h809, _, _, _, _, _, _, _, _) => {
                        h809
                    }
                };
                nat_le(a810, a811)
            } else {
                false
            } {
                f28(match v1.clone() {
                    Adt1::C0(_, _, _, h812, _, _, _, _, _, _, _) => {
                        h812
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h813, _, _, _, _, _, _, _) => {
                        h813
                    }
                })
            } else {
                false
            } {
                f28(match v1 {
                    Adt1::C0(_, _, _, _, h814, _, _, _, _, _, _) => {
                        h814
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h815, _, _, _, _, _, _) => {
                        h815
                    }
                })
            } else {
                false
            } {
                f21(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f23(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m816 = v0;
    match m816 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a818 = match v3.clone() {
                Adt1::C0(h817, _, _, _, _, _, _, _, _, _, _) => {
                    h817
                }
            };
            let a819 = 0u64;
            if if if if if if nat_eq(a818, a819) {
                let a821 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h820, _, _, _, _, _) => {
                        h820
                    }
                };
                let a822 = 4u64;
                nat_eq(a821, a822)
            } else {
                false
            } {
                let a825 = match v3.clone() {
                    Adt1::C0(_, h823, _, _, _, _, _, _, _, _, _) => {
                        h823
                    }
                };
                let a826 = match v1 {
                    Adt1::C0(_, _, _, h824, _, _, _, _, _, _, _) => {
                        h824
                    }
                };
                nat_eq(a825, a826)
            } else {
                false
            } {
                let a828 = match v3.clone() {
                    Adt1::C0(_, _, h827, _, _, _, _, _, _, _, _) => {
                        h827
                    }
                };
                let a829 = 1u64;
                nat_eq(a828, a829)
            } else {
                false
            } {
                let a831 = match v3.clone() {
                    Adt1::C0(_, _, _, h830, _, _, _, _, _, _, _) => {
                        h830
                    }
                };
                let a832 = 1u64;
                nat_eq(a831, a832)
            } else {
                false
            } {
                let a834 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h833, _, _, _, _, _, _) => {
                        h833
                    }
                };
                let a835 = 1u64;
                nat_eq(a834, a835)
            } else {
                false
            } {
                f21(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f24(v0: u64) -> Option<u64> {
    let a836 = 0u64;
    let a837 = v0;
    if if nat_lt(a836, a837) {
        let a838 = v0;
        let a839 = 2139095040u64;
        nat_lt(a838, a839)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f25(v0: u64) -> Option<u64> {
    let a840 = v0;
    let a841 = 0u64;
    if if nat_eq(a840, a841) {
        true
    } else {
        let a842 = v0;
        let a843 = 2147483648u64;
        nat_eq(a842, a843)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    let a846 = match v0.clone() {
        Adt1::C0(_, _, h844, _, _, _, _, _, _, _, _) => {
            h844
        }
    };
    let a847 = match v1.clone() {
        Adt1::C0(_, _, h845, _, _, _, _, _, _, _, _) => {
            h845
        }
    };
    if if nat_eq(a846, a847) {
        let a850 = match v0.clone() {
            Adt1::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = match v1.clone() {
            Adt1::C0(_, _, _, h849, _, _, _, _, _, _, _) => {
                h849
            }
        };
        nat_eq(a850, a851)
    } else {
        false
    } {
        let a854 = match v0 {
            Adt1::C0(_, _, _, _, h852, _, _, _, _, _, _) => {
                h852
            }
        };
        let a855 = match v1 {
            Adt1::C0(_, _, _, _, h853, _, _, _, _, _, _) => {
                h853
            }
        };
        nat_eq(a854, a855)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f38(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f38(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f38(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f38(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f38(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f38(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f28(v0: u64, v1: u64) -> bool {
    let a856 = 0u64;
    let a857 = v1;
    if nat_lt(a856, a857) {
        let a861 = {
            let a858 = v0;
            let a859 = v1;
            let a860 = 0u64;
            nat_rem(a858, a859, a860)
        };
        let a862 = 0u64;
        nat_eq(a861, a862)
    } else {
        false
    }
}

pub fn f29(v0: Adt1, v1: u64) -> R<bool> {
    let a864 = match v0.clone() {
        Adt1::C0(_, h863, _, _, _, _, _, _, _, _, _) => {
            h863
        }
    };
    let a865 = 1u64;
    if if if if nat_eq(a864, a865) {
        true
    } else {
        let a867 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h866, _, _, _, _, _) => {
                h866
            }
        };
        let a868 = v1;
        nat_eq(a867, a868)
    } {
        let a870 = match v0.clone() {
            Adt1::C0(_, _, h869, _, _, _, _, _, _, _, _) => {
                h869
            }
        };
        let a871 = 1u64;
        if nat_eq(a870, a871) {
            true
        } else {
            let a876 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h872, _, _, _, _) => {
                    h872
                }
            };
            let a877 = {
                let a874 = v1;
                let a875 = match v0.clone() {
                    Adt1::C0(_, h873, _, _, _, _, _, _, _, _, _) => {
                        h873
                    }
                };
                nat_mul(a874, a875)?
            };
            nat_eq(a876, a877)
        }
    } else {
        false
    } {
        let a879 = match v0.clone() {
            Adt1::C0(_, _, _, h878, _, _, _, _, _, _, _) => {
                h878
            }
        };
        let a880 = 1u64;
        if nat_eq(a879, a880) {
            true
        } else {
            let a888 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h881, _, _, _) => {
                    h881
                }
            };
            let a889 = {
                let a886 = {
                    let a883 = v1;
                    let a884 = match v0.clone() {
                        Adt1::C0(_, h882, _, _, _, _, _, _, _, _, _) => {
                            h882
                        }
                    };
                    nat_mul(a883, a884)?
                };
                let a887 = match v0.clone() {
                    Adt1::C0(_, _, h885, _, _, _, _, _, _, _, _) => {
                        h885
                    }
                };
                nat_mul(a886, a887)?
            };
            nat_eq(a888, a889)
        }
    } else {
        false
    } {
        let a891 = match v0.clone() {
            Adt1::C0(_, _, _, _, h890, _, _, _, _, _, _) => {
                h890
            }
        };
        let a892 = 1u64;
        if nat_eq(a891, a892) {
            Ok(true)
        } else {
            let a903 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h893, _, _) => {
                    h893
                }
            };
            let a904 = {
                let a901 = {
                    let a898 = {
                        let a895 = v1;
                        let a896 = match v0.clone() {
                            Adt1::C0(_, h894, _, _, _, _, _, _, _, _, _) => {
                                h894
                            }
                        };
                        nat_mul(a895, a896)?
                    };
                    let a899 = match v0.clone() {
                        Adt1::C0(_, _, h897, _, _, _, _, _, _, _, _) => {
                            h897
                        }
                    };
                    nat_mul(a898, a899)?
                };
                let a902 = match v0 {
                    Adt1::C0(_, _, _, h900, _, _, _, _, _, _, _) => {
                        h900
                    }
                };
                nat_mul(a901, a902)?
            };
            Ok(nat_eq(a903, a904))
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Adt1, v1: Adt1) -> bool {
    let a907 = match v0.clone() {
        Adt1::C0(_, h905, _, _, _, _, _, _, _, _, _) => {
            h905
        }
    };
    let a908 = match v1.clone() {
        Adt1::C0(_, h906, _, _, _, _, _, _, _, _, _) => {
            h906
        }
    };
    if if if nat_eq(a907, a908) {
        let a911 = match v0.clone() {
            Adt1::C0(_, _, h909, _, _, _, _, _, _, _, _) => {
                h909
            }
        };
        let a912 = match v1.clone() {
            Adt1::C0(_, _, h910, _, _, _, _, _, _, _, _) => {
                h910
            }
        };
        nat_eq(a911, a912)
    } else {
        false
    } {
        let a915 = match v0.clone() {
            Adt1::C0(_, _, _, h913, _, _, _, _, _, _, _) => {
                h913
            }
        };
        let a916 = match v1.clone() {
            Adt1::C0(_, _, _, h914, _, _, _, _, _, _, _) => {
                h914
            }
        };
        nat_eq(a915, a916)
    } else {
        false
    } {
        let a919 = match v0 {
            Adt1::C0(_, _, _, _, h917, _, _, _, _, _, _) => {
                h917
            }
        };
        let a920 = match v1 {
            Adt1::C0(_, _, _, _, h918, _, _, _, _, _, _) => {
                h918
            }
        };
        nat_eq(a919, a920)
    } else {
        false
    }
}

pub fn f31(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f30(v0.clone(), v1.clone()) {
        let a923 = match v0.clone() {
            Adt1::C0(h921, _, _, _, _, _, _, _, _, _, _) => {
                h921
            }
        };
        let a924 = match v1.clone() {
            Adt1::C0(h922, _, _, _, _, _, _, _, _, _, _) => {
                h922
            }
        };
        nat_eq(a923, a924)
    } else {
        false
    } {
        let a927 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h925) => {
                h925
            }
        };
        let a928 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h926) => {
                h926
            }
        };
        nat_eq(a927, a928)
    } else {
        false
    } {
        let a931 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h929, _, _, _, _, _) => {
                h929
            }
        };
        let a932 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h930, _, _, _, _, _) => {
                h930
            }
        };
        nat_eq(a931, a932)
    } else {
        false
    } {
        let a935 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h933, _, _, _, _) => {
                h933
            }
        };
        let a936 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h934, _, _, _, _) => {
                h934
            }
        };
        nat_eq(a935, a936)
    } else {
        false
    } {
        let a939 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h937, _, _, _) => {
                h937
            }
        };
        let a940 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h938, _, _, _) => {
                h938
            }
        };
        nat_eq(a939, a940)
    } else {
        false
    } {
        let a943 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h941, _, _) => {
                h941
            }
        };
        let a944 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h942, _, _) => {
                h942
            }
        };
        nat_eq(a943, a944)
    } else {
        false
    }
}

pub fn f32(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m945 = v0;
    match m945 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a947 = match v3.clone() {
                Adt1::C0(h946, _, _, _, _, _, _, _, _, _, _) => {
                    h946
                }
            };
            let a948 = 0u64;
            if if if if if if if nat_eq(a947, a948) {
                true
            } else {
                let a950 = match v3.clone() {
                    Adt1::C0(h949, _, _, _, _, _, _, _, _, _, _) => {
                        h949
                    }
                };
                let a951 = 1u64;
                nat_eq(a950, a951)
            } {
                f29(v3.clone(), {
                    let a953 = match v3.clone() {
                        Adt1::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
                            h952
                        }
                    };
                    let a954 = 0u64;
                    if nat_eq(a953, a954) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a957 = match v3.clone() {
                    Adt1::C0(_, h955, _, _, _, _, _, _, _, _, _) => {
                        h955
                    }
                };
                let a958 = match v1.clone() {
                    Adt1::C0(_, h956, _, _, _, _, _, _, _, _, _) => {
                        h956
                    }
                };
                nat_eq(a957, a958)
            } else {
                false
            } {
                let a961 = match v1.clone() {
                    Adt1::C0(_, _, h959, _, _, _, _, _, _, _, _) => {
                        h959
                    }
                };
                let a962 = match v3.clone() {
                    Adt1::C0(_, _, h960, _, _, _, _, _, _, _, _) => {
                        h960
                    }
                };
                nat_le(a961, a962)
            } else {
                false
            } {
                f28(match v1.clone() {
                    Adt1::C0(_, _, _, h963, _, _, _, _, _, _, _) => {
                        h963
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h964, _, _, _, _, _, _, _) => {
                        h964
                    }
                })
            } else {
                false
            } {
                f28(match v1 {
                    Adt1::C0(_, _, _, _, h965, _, _, _, _, _, _) => {
                        h965
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h966, _, _, _, _, _, _) => {
                        h966
                    }
                })
            } else {
                false
            } {
                f21(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f33(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m967 = v0;
    match m967 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a969 = match v3.clone() {
                Adt1::C0(h968, _, _, _, _, _, _, _, _, _, _) => {
                    h968
                }
            };
            let a970 = 0u64;
            if if if if if if nat_eq(a969, a970) {
                let a972 = match v3.clone() {
                    Adt1::C0(_, h971, _, _, _, _, _, _, _, _, _) => {
                        h971
                    }
                };
                let a973 = 1u64;
                if nat_eq(a972, a973) {
                    true
                } else {
                    let a975 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h974, _, _, _, _, _) => {
                            h974
                        }
                    };
                    let a976 = 4u64;
                    nat_eq(a975, a976)
                }
            } else {
                false
            } {
                let a979 = match v3.clone() {
                    Adt1::C0(_, h977, _, _, _, _, _, _, _, _, _) => {
                        h977
                    }
                };
                let a980 = match v1 {
                    Adt1::C0(_, _, _, h978, _, _, _, _, _, _, _) => {
                        h978
                    }
                };
                nat_eq(a979, a980)
            } else {
                false
            } {
                let a982 = match v3.clone() {
                    Adt1::C0(_, _, h981, _, _, _, _, _, _, _, _) => {
                        h981
                    }
                };
                let a983 = 1u64;
                nat_eq(a982, a983)
            } else {
                false
            } {
                let a985 = match v3.clone() {
                    Adt1::C0(_, _, _, h984, _, _, _, _, _, _, _) => {
                        h984
                    }
                };
                let a986 = 1u64;
                nat_eq(a985, a986)
            } else {
                false
            } {
                let a988 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h987, _, _, _, _, _, _) => {
                        h987
                    }
                };
                let a989 = 1u64;
                nat_eq(a988, a989)
            } else {
                false
            } {
                f21(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f34(v0: u64) -> u64 {
    if f39(v0) {
        8u64
    } else {
        if f40(v0) {
            9u64
        } else {
            f41(v0)
        }
    }
}

pub fn f35(v0: Adt1, v1: Adt1) -> R<bool> {
    let a991 = match v0.clone() {
        Adt1::C0(h990, _, _, _, _, _, _, _, _, _, _) => {
            h990
        }
    };
    let a992 = 0u64;
    if if nat_eq(a991, a992) {
        let a994 = match v0.clone() {
            Adt1::C0(_, h993, _, _, _, _, _, _, _, _, _) => {
                h993
            }
        };
        let a995 = 1u64;
        if nat_eq(a994, a995) {
            true
        } else {
            let a997 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h996, _, _, _, _, _) => {
                    h996
                }
            };
            let a998 = 4u64;
            nat_eq(a997, a998)
        }
    } else {
        false
    } {
        f21(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> bool {
    let a1001 = match v0.clone() {
        Adt1::C0(_, h999, _, _, _, _, _, _, _, _, _) => {
            h999
        }
    };
    let a1002 = match v1.clone() {
        Adt1::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
            h1000
        }
    };
    if if if nat_eq(a1001, a1002) {
        let a1005 = match v0.clone() {
            Adt1::C0(_, _, _, h1003, _, _, _, _, _, _, _) => {
                h1003
            }
        };
        let a1006 = match v1.clone() {
            Adt1::C0(_, _, _, h1004, _, _, _, _, _, _, _) => {
                h1004
            }
        };
        nat_eq(a1005, a1006)
    } else {
        false
    } {
        let a1008 = 0u64;
        let a1009 = match v0.clone() {
            Adt1::C0(_, _, h1007, _, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        nat_lt(a1008, a1009)
    } else {
        false
    } {
        f28(match v1 {
            Adt1::C0(_, _, _, _, h1010, _, _, _, _, _, _) => {
                h1010
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, h1011, _, _, _, _, _, _) => {
                h1011
            }
        })
    } else {
        false
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> bool {
    let m1012 = f42(v0);
    match m1012 {
        Adt8::C0 => {
            false
        }
        Adt8::C1(v3) => {
            let a1013 = v0;
            let a1014 = 0u64;
            if if if if if if nat_eq(a1013, a1014) {
                true
            } else {
                let a1015 = v0;
                let a1016 = 1u64;
                nat_eq(a1015, a1016)
            } {
                true
            } else {
                let a1017 = v0;
                let a1018 = 30u64;
                nat_eq(a1017, a1018)
            } {
                true
            } else {
                let a1019 = v0;
                let a1020 = 43u64;
                nat_eq(a1019, a1020)
            } {
                true
            } else {
                f43(v0)
            } {
                let a1022 = v2;
                let a1023 = match v3.clone() {
                    Adt9::C0(_, _, h1021) => {
                        h1021
                    }
                };
                nat_eq(a1022, a1023)
            } else {
                false
            } {
                f28(v1, match v3 {
                    Adt9::C0(_, h1024, _) => {
                        h1024
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1025 = v1;
    let a1026 = 1u64;
    if if if nat_le(a1025, a1026) {
        true
    } else {
        let a1027 = v0;
        let a1028 = v2;
        nat_le(a1027, a1028)
    } {
        let a1029 = v3;
        let a1030 = 1u64;
        if nat_le(a1029, a1030) {
            true
        } else {
            let a1037 = {
                let a1035 = v0;
                let a1036 = {
                    let a1033 = {
                        let a1031 = v1;
                        let a1032 = 1u64;
                        nat_sub(a1031, a1032)
                    };
                    let a1034 = v2;
                    nat_mul(a1033, a1034)?
                };
                nat_add(a1035, a1036)?
            };
            let a1038 = v4;
            nat_le(a1037, a1038)
        }
    } else {
        false
    } {
        let a1039 = v5;
        let a1040 = 1u64;
        if nat_le(a1039, a1040) {
            Ok(true)
        } else {
            let a1053 = {
                let a1051 = {
                    let a1045 = v0;
                    let a1046 = {
                        let a1043 = {
                            let a1041 = v1;
                            let a1042 = 1u64;
                            nat_sub(a1041, a1042)
                        };
                        let a1044 = v2;
                        nat_mul(a1043, a1044)?
                    };
                    nat_add(a1045, a1046)?
                };
                let a1052 = {
                    let a1049 = {
                        let a1047 = v3;
                        let a1048 = 1u64;
                        nat_sub(a1047, a1048)
                    };
                    let a1050 = v4;
                    nat_mul(a1049, a1050)?
                };
                nat_add(a1051, a1052)?
            };
            let a1054 = v6;
            Ok(nat_le(a1053, a1054))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: u64) -> bool {
    let a1055 = v0;
    let a1056 = 2u64;
    if if if if nat_eq(a1055, a1056) {
        true
    } else {
        let a1057 = v0;
        let a1058 = 6u64;
        nat_eq(a1057, a1058)
    } {
        true
    } else {
        let a1059 = v0;
        let a1060 = 8u64;
        nat_eq(a1059, a1060)
    } {
        true
    } else {
        let a1061 = v0;
        let a1062 = 20u64;
        nat_eq(a1061, a1062)
    } {
        true
    } else {
        f44(v0)
    }
}

pub fn f40(v0: u64) -> bool {
    let a1063 = v0;
    let a1064 = 3u64;
    if nat_eq(a1063, a1064) {
        true
    } else {
        let a1065 = v0;
        let a1066 = 7u64;
        nat_eq(a1065, a1066)
    }
}

pub fn f41(v0: u64) -> u64 {
    let a1067 = v0;
    let a1068 = 30u64;
    if nat_eq(a1067, a1068) {
        30u64
    } else {
        0u64
    }
}

pub fn f42(v0: u64) -> Adt8 {
    let a1069 = v0;
    let a1070 = 0u64;
    if nat_eq(a1069, a1070) {
        Adt8::C1(Adt9::C0(0u64, 1u64, 4u64))
    } else {
        let a1071 = v0;
        let a1072 = 1u64;
        if nat_eq(a1071, a1072) {
            Adt8::C1(Adt9::C0(1u64, 1u64, 2u64))
        } else {
            let a1073 = v0;
            let a1074 = 2u64;
            if nat_eq(a1073, a1074) {
                Adt8::C1(Adt9::C0(2u64, 32u64, 18u64))
            } else {
                let a1075 = v0;
                let a1076 = 3u64;
                if nat_eq(a1075, a1076) {
                    Adt8::C1(Adt9::C0(3u64, 32u64, 20u64))
                } else {
                    let a1077 = v0;
                    let a1078 = 6u64;
                    if nat_eq(a1077, a1078) {
                        Adt8::C1(Adt9::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1079 = v0;
                        let a1080 = 7u64;
                        if nat_eq(a1079, a1080) {
                            Adt8::C1(Adt9::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1081 = v0;
                            let a1082 = 8u64;
                            if nat_eq(a1081, a1082) {
                                Adt8::C1(Adt9::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1083 = v0;
                                let a1084 = 9u64;
                                if nat_eq(a1083, a1084) {
                                    Adt8::C1(Adt9::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1085 = v0;
                                    let a1086 = 10u64;
                                    if nat_eq(a1085, a1086) {
                                        Adt8::C1(Adt9::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1087 = v0;
                                        let a1088 = 11u64;
                                        if nat_eq(a1087, a1088) {
                                            Adt8::C1(Adt9::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1089 = v0;
                                            let a1090 = 12u64;
                                            if nat_eq(a1089, a1090) {
                                                Adt8::C1(Adt9::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1091 = v0;
                                                let a1092 = 13u64;
                                                if nat_eq(a1091, a1092) {
                                                    Adt8::C1(Adt9::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1093 = v0;
                                                    let a1094 = 14u64;
                                                    if nat_eq(a1093, a1094) {
                                                        Adt8::C1(Adt9::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1095 = v0;
                                                        let a1096 = 15u64;
                                                        if nat_eq(a1095, a1096) {
                                                            Adt8::C1(Adt9::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1097 = v0;
                                                            let a1098 = 16u64;
                                                            if nat_eq(a1097, a1098) {
                                                                Adt8::C1(Adt9::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1099 = v0;
                                                                let a1100 = 17u64;
                                                                if nat_eq(a1099, a1100) {
                                                                    Adt8::C1(Adt9::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1101 = v0;
                                                                    let a1102 = 18u64;
                                                                    if nat_eq(a1101, a1102) {
                                                                        Adt8::C1(Adt9::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1103 = v0;
                                                                        let a1104 = 19u64;
                                                                        if nat_eq(a1103, a1104) {
                                                                            Adt8::C1(Adt9::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1105 = v0;
                                                                            let a1106 = 20u64;
                                                                            if nat_eq(a1105, a1106) {
                                                                                Adt8::C1(Adt9::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1107 = v0;
                                                                                let a1108 = 21u64;
                                                                                if nat_eq(a1107, a1108) {
                                                                                    Adt8::C1(Adt9::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1109 = v0;
                                                                                    let a1110 = 22u64;
                                                                                    if nat_eq(a1109, a1110) {
                                                                                        Adt8::C1(Adt9::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1111 = v0;
                                                                                        let a1112 = 23u64;
                                                                                        if nat_eq(a1111, a1112) {
                                                                                            Adt8::C1(Adt9::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1113 = v0;
                                                                                            let a1114 = 24u64;
                                                                                            if nat_eq(a1113, a1114) {
                                                                                                Adt8::C1(Adt9::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1115 = v0;
                                                                                                let a1116 = 25u64;
                                                                                                if nat_eq(a1115, a1116) {
                                                                                                    Adt8::C1(Adt9::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1117 = v0;
                                                                                                    let a1118 = 26u64;
                                                                                                    if nat_eq(a1117, a1118) {
                                                                                                        Adt8::C1(Adt9::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1119 = v0;
                                                                                                        let a1120 = 27u64;
                                                                                                        if nat_eq(a1119, a1120) {
                                                                                                            Adt8::C1(Adt9::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1121 = v0;
                                                                                                            let a1122 = 28u64;
                                                                                                            if nat_eq(a1121, a1122) {
                                                                                                                Adt8::C1(Adt9::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1123 = v0;
                                                                                                                let a1124 = 29u64;
                                                                                                                if nat_eq(a1123, a1124) {
                                                                                                                    Adt8::C1(Adt9::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1125 = v0;
                                                                                                                    let a1126 = 30u64;
                                                                                                                    if nat_eq(a1125, a1126) {
                                                                                                                        Adt8::C1(Adt9::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1127 = v0;
                                                                                                                        let a1128 = 34u64;
                                                                                                                        if nat_eq(a1127, a1128) {
                                                                                                                            Adt8::C1(Adt9::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1129 = v0;
                                                                                                                            let a1130 = 35u64;
                                                                                                                            if nat_eq(a1129, a1130) {
                                                                                                                                Adt8::C1(Adt9::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1131 = v0;
                                                                                                                                let a1132 = 39u64;
                                                                                                                                if nat_eq(a1131, a1132) {
                                                                                                                                    Adt8::C1(Adt9::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1133 = v0;
                                                                                                                                    let a1134 = 40u64;
                                                                                                                                    if nat_eq(a1133, a1134) {
                                                                                                                                        Adt8::C1(Adt9::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1135 = v0;
                                                                                                                                        let a1136 = 41u64;
                                                                                                                                        if nat_eq(a1135, a1136) {
                                                                                                                                            Adt8::C1(Adt9::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1137 = v0;
                                                                                                                                            let a1138 = 42u64;
                                                                                                                                            if nat_eq(a1137, a1138) {
                                                                                                                                                Adt8::C1(Adt9::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1139 = v0;
                                                                                                                                                let a1140 = 43u64;
                                                                                                                                                if nat_eq(a1139, a1140) {
                                                                                                                                                    Adt8::C1(Adt9::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt8::C0
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

pub fn f43(v0: u64) -> bool {
    if f39(v0) {
        true
    } else {
        f40(v0)
    }
}

pub fn f44(v0: u64) -> bool {
    let a1141 = v0;
    let a1142 = 41u64;
    if nat_eq(a1141, a1142) {
        true
    } else {
        let a1143 = v0;
        let a1144 = 42u64;
        nat_eq(a1143, a1144)
    }
}

pub fn run(p0: Adt0) -> R<Adt7> {
    f0(p0)
}
