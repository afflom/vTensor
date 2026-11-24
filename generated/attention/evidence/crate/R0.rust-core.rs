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
    C13(u64, bool, u64, Adt1, Adt1, Adt1, Adt1, Adt1, Adt1, Adt1, Adt1),
    C14(Adt6, u64, u64, u64),
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
    C0(Adt1, Adt1, Adt1, Adt1, Adt1, Adt1, u64, u64, u64, u64, u64, bool),
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
        Adt0::C0(v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69) => {
            f9(v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, None::<Adt1>, v69)
        }
        Adt0::C1(v98, v99, v100, v101) => {
            f15(v98, v99, v100, v101)
        }
        Adt0::C2(v86, v87, v88, v89, v90, v91, v92, v93) => {
            f13(v86, v87, v88, v89, v90, v91, v92, v93)
        }
        Adt0::C3(v94, v95, v96, v97) => {
            f14(v94, v95, v96, v97)
        }
        Adt0::C4(v70, v71, v72, v73, v74, v75, v76, v77) => {
            f10(v70, v71, v72, v73, v74, v75, v76, v77)
        }
        Adt0::C5(v78, v79, v80, v81) => {
            f11(v78, v79, v80, v81)
        }
        Adt0::C6(v82, v83, v84, v85) => {
            f12(v82, v83, v84, v85)
        }
        Adt0::C7(v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58) => {
            f8(v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58)
        }
        Adt0::C8(v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42) => {
            let a11 = v30;
            let a12 = 83u64;
            if nat_eq(a11, a12) {
                f7(v31, v32, v36, v37, v38, v39, v40, v41, v42)
            } else {
                let m13 = v41.clone();
                match m13 {
                    None => {
                        f8(v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v42)
                    }
                    Some(_) => {
                        let m14 = v36;
                        match m14 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v44) => {
                                let m15 = v37;
                                match m15 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v45) => {
                                        let m16 = v38;
                                        match m16 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v46) => {
                                                f9(v30, v31, v33, v34, v35, v44, v45, v46, v39, v40, v41, v42)
                                            }
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
        Adt0::C11(v23, v24, v25) => {
            if match v23.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h9) => {
                    h9
                }
            } {
                Ok(Adt7::C0)
            } else {
                f4(v23, v24, v25)
            }
        }
        Adt0::C12(v26, v27, v28, v29) => {
            if match v26.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h10) => {
                    h10
                }
            } {
                f5(v26, v27, v28, v29)
            } else {
                f6(v26, v27, v28, v29)
            }
        }
        Adt0::C13(v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) => {
            f2(v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
        }
        Adt0::C14(v19, v20, v21, v22) => {
            if match v19.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h8) => {
                    h8
                }
            } {
                f3(v19, v20, v21, v22)
            } else {
                Ok(Adt7::C0)
            }
        }
    }
}

pub fn f1(v0: Adt5, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt7> {
    let a20 = v1;
    let a21 = {
        let m19 = match v0.clone() {
            Adt5::C0(h18, _, _, _, _) => {
                h18
            }
        };
        match m19 {
            Adt1::C0(_, _, _, h17, _, _, _, _, _, _, _) => {
                h17
            }
        }
    };
    if if if if nat_lt(a20, a21) {
        let a25 = v2;
        let a26 = {
            let m24 = match v0.clone() {
                Adt5::C0(h23, _, _, _, _) => {
                    h23
                }
            };
            match m24 {
                Adt1::C0(_, _, h22, _, _, _, _, _, _, _, _) => {
                    h22
                }
            }
        };
        nat_lt(a25, a26)
    } else {
        false
    } {
        let a30 = v3;
        let a31 = {
            let m29 = match v0.clone() {
                Adt5::C0(h28, _, _, _, _) => {
                    h28
                }
            };
            match m29 {
                Adt1::C0(_, _, _, _, h27, _, _, _, _, _, _) => {
                    h27
                }
            }
        };
        nat_lt(a30, a31)
    } else {
        false
    } {
        let a35 = v4;
        let a36 = {
            let m34 = match v0.clone() {
                Adt5::C0(_, h33, _, _, _) => {
                    h33
                }
            };
            match m34 {
                Adt1::C0(_, _, _, h32, _, _, _, _, _, _, _) => {
                    h32
                }
            }
        };
        nat_lt(a35, a36)
    } else {
        false
    } {
        Ok(Adt7::C11(f16(match v0.clone() {
            Adt5::C0(h37, _, _, _, _) => {
                h37
            }
        }, v2, v1, v3)?, f16(match v0.clone() {
            Adt5::C0(_, h38, _, _, _) => {
                h38
            }
        }, 0u64, v4, v3)?, f16(match v0.clone() {
            Adt5::C0(_, _, h39, _, _) => {
                h39
            }
        }, v1, 0u64, v3)?, f16(match v0.clone() {
            Adt5::C0(_, _, _, h40, _) => {
                h40
            }
        }, v1, 0u64, {
            let a44 = v3;
            let a45 = {
                let m43 = match v0.clone() {
                    Adt5::C0(_, _, _, h42, _) => {
                        h42
                    }
                };
                match m43 {
                    Adt1::C0(_, _, _, _, h41, _, _, _, _, _, _) => {
                        h41
                    }
                }
            };
            let a46 = 0u64;
            nat_rem(a44, a45, a46)
        })?, f16(match v0 {
            Adt5::C0(_, _, _, _, h47) => {
                h47
            }
        }, v1, 0u64, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f2(v0: u64, v1: bool, v2: u64, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1, v8: Adt1, v9: Adt1, v10: Adt1) -> R<Adt7> {
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, h48, _, _, _, _, _, _, _, _, _) => {
            h48
        }
    };
    let v12: u64 = match v4.clone() {
        Adt1::C0(_, h49, _, _, _, _, _, _, _, _, _) => {
            h49
        }
    };
    let v13: u64 = match v4.clone() {
        Adt1::C0(_, _, h50, _, _, _, _, _, _, _, _) => {
            h50
        }
    };
    let v14: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h51, _, _, _, _, _, _, _) => {
            h51
        }
    };
    let v15: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h52, _, _, _, _, _, _) => {
            h52
        }
    };
    let v16: u64 = {
        let a53 = v11;
        let a54 = v12;
        nat_mul(a53, a54)?
    };
    let v17: u64 = {
        let a55 = v13;
        let a56 = v15;
        nat_mul(a55, a56)?
    };
    let v18: u64 = {
        let a59 = {
            let a57 = v14;
            let a58 = v12;
            nat_mul(a57, a58)?
        };
        let a60 = v17;
        nat_mul(a59, a60)?
    };
    let a61 = v0;
    let a62 = 74u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a61, a62) {
        !v1
    } else {
        false
    } {
        let a63 = 0u64;
        let a64 = v2;
        nat_lt(a63, a64)
    } else {
        false
    } {
        let a65 = v2;
        let a66 = 2147483647u64;
        nat_le(a65, a66)
    } else {
        false
    } {
        let a67 = 0u64;
        let a68 = v11;
        nat_lt(a67, a68)
    } else {
        false
    } {
        let a69 = 0u64;
        let a70 = v12;
        nat_lt(a69, a70)
    } else {
        false
    } {
        let a71 = 0u64;
        let a72 = v13;
        nat_lt(a71, a72)
    } else {
        false
    } {
        let a73 = 0u64;
        let a74 = v15;
        nat_lt(a73, a74)
    } else {
        false
    } {
        let a76 = 0u64;
        let a77 = match v3.clone() {
            Adt1::C0(_, _, _, _, h75, _, _, _, _, _, _) => {
                h75
            }
        };
        nat_lt(a76, a77)
    } else {
        false
    } {
        f17(v13, match v7.clone() {
            Adt1::C0(_, _, h78, _, _, _, _, _, _, _, _) => {
                h78
            }
        })
    } else {
        false
    } {
        f18(v3.clone(), v10.clone())?
    } else {
        false
    } {
        f18(v4.clone(), v10.clone())?
    } else {
        false
    } {
        f18(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f18(v6.clone(), v10.clone())?
    } else {
        false
    } {
        f18(v7.clone(), v10.clone())?
    } else {
        false
    } {
        f18(v8.clone(), v10.clone())?
    } else {
        false
    } {
        f19(v4.clone())?
    } else {
        false
    } {
        f19(v7.clone())?
    } else {
        false
    } {
        f19(v8.clone())?
    } else {
        false
    } {
        f20(v3.clone(), 4u64)?
    } else {
        false
    } {
        f20(v5.clone(), 4u64)?
    } else {
        false
    } {
        f20(v6.clone(), 4u64)?
    } else {
        false
    } {
        f20(v10.clone(), 4u64)?
    } else {
        false
    } {
        let a80 = match v3.clone() {
            Adt1::C0(_, _, h79, _, _, _, _, _, _, _, _) => {
                h79
            }
        };
        let a81 = v12;
        nat_eq(a80, a81)
    } else {
        false
    } {
        let a83 = match v3.clone() {
            Adt1::C0(_, _, _, h82, _, _, _, _, _, _, _) => {
                h82
            }
        };
        let a84 = v13;
        nat_eq(a83, a84)
    } else {
        false
    } {
        f21(v5.clone(), v13, v14, v15, 1u64)
    } else {
        false
    } {
        let a86 = match v6.clone() {
            Adt1::C0(_, _, h85, _, _, _, _, _, _, _, _) => {
                h85
            }
        };
        let a87 = v13;
        nat_eq(a86, a87)
    } else {
        false
    } {
        let a89 = match v6.clone() {
            Adt1::C0(_, _, _, h88, _, _, _, _, _, _, _) => {
                h88
            }
        };
        let a90 = 1u64;
        nat_eq(a89, a90)
    } else {
        false
    } {
        let a92 = match v6.clone() {
            Adt1::C0(_, _, _, _, h91, _, _, _, _, _, _) => {
                h91
            }
        };
        let a93 = 1u64;
        nat_eq(a92, a93)
    } else {
        false
    } {
        let a95 = match v6.clone() {
            Adt1::C0(_, h94, _, _, _, _, _, _, _, _, _) => {
                h94
            }
        };
        let a96 = 1u64;
        if nat_eq(a95, a96) {
            true
        } else {
            let a98 = match v6.clone() {
                Adt1::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                    h97
                }
            };
            let a99 = v11;
            if nat_eq(a98, a99) {
                let a100 = v2;
                let a101 = 1u64;
                nat_eq(a100, a101)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a103 = match v7.clone() {
            Adt1::C0(_, h102, _, _, _, _, _, _, _, _, _) => {
                h102
            }
        };
        let a104 = v11;
        nat_eq(a103, a104)
    } else {
        false
    } {
        let a106 = match v7.clone() {
            Adt1::C0(_, _, _, h105, _, _, _, _, _, _, _) => {
                h105
            }
        };
        let a107 = v14;
        nat_eq(a106, a107)
    } else {
        false
    } {
        let a109 = match v7.clone() {
            Adt1::C0(_, _, _, _, h108, _, _, _, _, _, _) => {
                h108
            }
        };
        let a110 = v15;
        nat_eq(a109, a110)
    } else {
        false
    } {
        f22(v7.clone(), v8.clone())
    } else {
        false
    } {
        f21(v9.clone(), v15, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        let a112 = match v9.clone() {
            Adt1::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
                h111
            }
        };
        let a113 = 26u64;
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a115 = match v9.clone() {
            Adt1::C0(_, _, _, _, _, h114, _, _, _, _, _) => {
                h114
            }
        };
        let a116 = 4u64;
        nat_eq(a115, a116)
    } else {
        false
    } {
        f23(v9, v10.clone())?
    } else {
        false
    } {
        let a118 = match v10.clone() {
            Adt1::C0(h117, _, _, _, _, _, _, _, _, _, _) => {
                h117
            }
        };
        let a119 = 0u64;
        nat_eq(a118, a119)
    } else {
        false
    } {
        f21(v10, {
            let a124 = v18;
            let a125 = {
                let a122 = v2;
                let a123 = {
                    let a120 = v16;
                    let a121 = v17;
                    nat_mul(a120, a121)?
                };
                nat_mul(a122, a123)?
            };
            nat_add(a124, a125)?
        }, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        Ok(Adt7::C12(Adt6::C0(v7, v8, v4, v6, v5, v3, v2, v17, v16, {
            let a128 = {
                let a126 = v16;
                let a127 = v11;
                nat_add(a126, a127)?
            };
            let a129 = v12;
            nat_add(a128, a129)?
        }, {
            let a136 = v18;
            let a137 = {
                let a134 = {
                    let a130 = v2;
                    let a131 = v14;
                    if nat_lt(a130, a131) {
                        v2
                    } else {
                        v14
                    }
                };
                let a135 = {
                    let a132 = v16;
                    let a133 = v17;
                    nat_mul(a132, a133)?
                };
                nat_mul(a134, a135)?
            };
            nat_add(a136, a137)?
        }, true)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f3(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(_, _, _, _, _, h138, _, _, _, _, _, _) => {
            h138
        }
    };
    let v5: Adt1 = match v0 {
        Adt6::C0(_, _, h139, _, _, _, _, _, _, _, _, _) => {
            h139
        }
    };
    let a141 = v1;
    let a142 = match v5.clone() {
        Adt1::C0(_, _, h140, _, _, _, _, _, _, _, _) => {
            h140
        }
    };
    if if if if nat_lt(a141, a142) {
        let a144 = v2;
        let a145 = match v5 {
            Adt1::C0(_, _, _, _, h143, _, _, _, _, _, _) => {
                h143
            }
        };
        nat_lt(a144, a145)
    } else {
        false
    } {
        let a147 = v3;
        let a148 = match v4.clone() {
            Adt1::C0(_, _, _, _, h146, _, _, _, _, _, _) => {
                h146
            }
        };
        nat_lt(a147, a148)
    } else {
        false
    } {
        let a149 = v3;
        let a150 = 2147483648u64;
        nat_lt(a149, a150)
    } else {
        false
    } {
        Ok(Adt7::C13(f16(v4, 0u64, v1, v3)?, None::<u64>))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f4(v0: Adt6, v1: u64, v2: u64) -> R<Adt7> {
    let v3: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h151, _, _, _, _, _, _, _, _, _) => {
            h151
        }
    };
    let a153 = v1;
    let a154 = match v3.clone() {
        Adt1::C0(_, _, h152, _, _, _, _, _, _, _, _) => {
            h152
        }
    };
    if if nat_lt(a153, a154) {
        let a156 = v2;
        let a157 = match v3.clone() {
            Adt1::C0(_, _, _, _, h155, _, _, _, _, _, _) => {
                h155
            }
        };
        nat_lt(a156, a157)
    } else {
        false
    } {
        Ok(Adt7::C13(f16(match v0.clone() {
            Adt6::C0(_, _, _, _, _, h158, _, _, _, _, _, _) => {
                h158
            }
        }, 0u64, v1, v2)?, {
            let a160 = match v3.clone() {
                Adt1::C0(_, _, _, h159, _, _, _, _, _, _, _) => {
                    h159
                }
            };
            let a161 = 0u64;
            if if nat_eq(a160, a161) {
                let a163 = match v0 {
                    Adt6::C0(_, _, _, _, _, _, h162, _, _, _, _, _) => {
                        h162
                    }
                };
                let a164 = 1u64;
                nat_eq(a163, a164)
            } else {
                false
            } {
                Some(f24(v3, v1, v2, 0u64)?)
            } else {
                None::<u64>
            }
        }))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f5(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h165, _, _, _, _, _, _, _, _, _) => {
            h165
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(h166, _, _, _, _, _, _, _, _, _, _, _) => {
            h166
        }
    };
    let v6: u64 = match v4.clone() {
        Adt1::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
            h167
        }
    };
    let v7: u64 = match v4.clone() {
        Adt1::C0(_, _, h168, _, _, _, _, _, _, _, _) => {
            h168
        }
    };
    let v8: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h169, _, _, _, _, _, _, _) => {
            h169
        }
    };
    let v9: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h170, _, _, _, _, _, _) => {
            h170
        }
    };
    let a171 = v1;
    let a172 = v7;
    if if if nat_lt(a171, a172) {
        let a173 = v2;
        let a174 = v9;
        nat_lt(a173, a174)
    } else {
        false
    } {
        let a175 = v3;
        let a176 = v8;
        nat_lt(a175, a176)
    } else {
        false
    } {
        let v10: u64 = {
            let a181 = v1;
            let a182 = {
                let a178 = v7;
                let a179 = match v5.clone() {
                    Adt1::C0(_, _, h177, _, _, _, _, _, _, _, _) => {
                        h177
                    }
                };
                let a180 = 0u64;
                nat_quot(a178, a179, a180)
            };
            let a183 = 0u64;
            nat_quot(a181, a182, a183)
        };
        let v11: u64 = {
            let a186 = v8;
            let a187 = {
                let a184 = v3;
                let a185 = 1u64;
                nat_add(a184, a185)?
            };
            nat_sub(a186, a187)
        };
        Ok(Adt7::C14(f16(v5, v10, v3, v2)?, f16(match v0.clone() {
            Adt6::C0(_, h188, _, _, _, _, _, _, _, _, _, _) => {
                h188
            }
        }, v10, v3, v2)?, f16(v4, v1, v3, v2)?, f16(match v0.clone() {
            Adt6::C0(_, _, _, h189, _, _, _, _, _, _, _, _) => {
                h189
            }
        }, v1, 0u64, 0u64)?, {
            let a193 = f16(match v0.clone() {
                Adt6::C0(_, _, _, _, h190, _, _, _, _, _, _, _) => {
                    h190
                }
            }, v3, v2, 0u64)?;
            let a194 = {
                let a191 = 4u64;
                let a192 = v1;
                nat_mul(a191, a192)?
            };
            nat_add(a193, a194)?
        }, {
            let a205 = 4u64;
            let a206 = {
                let a203 = v6;
                let a204 = {
                    let a201 = v1;
                    let a202 = {
                        let a199 = v7;
                        let a200 = {
                            let a197 = v3;
                            let a198 = {
                                let a195 = v8;
                                let a196 = v2;
                                nat_mul(a195, a196)?
                            };
                            nat_add(a197, a198)?
                        };
                        nat_mul(a199, a200)?
                    };
                    nat_add(a201, a202)?
                };
                nat_mul(a203, a204)?
            };
            nat_mul(a205, a206)?
        }, {
            let a208 = v11;
            let a209 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, h207, _, _, _, _, _) => {
                    h207
                }
            };
            if nat_lt(a208, a209) {
                Some({
                    let a228 = 4u64;
                    let a229 = {
                        let a226 = {
                            let a213 = {
                                let a210 = v6;
                                let a211 = v8;
                                nat_mul(a210, a211)?
                            };
                            let a214 = match v0.clone() {
                                Adt6::C0(_, _, _, _, _, _, _, h212, _, _, _, _) => {
                                    h212
                                }
                            };
                            nat_mul(a213, a214)?
                        };
                        let a227 = {
                            let a224 = match v0 {
                                Adt6::C0(_, _, _, _, _, _, _, _, h215, _, _, _) => {
                                    h215
                                }
                            };
                            let a225 = {
                                let a222 = v1;
                                let a223 = {
                                    let a220 = v7;
                                    let a221 = {
                                        let a218 = v2;
                                        let a219 = {
                                            let a216 = v9;
                                            let a217 = v11;
                                            nat_mul(a216, a217)?
                                        };
                                        nat_add(a218, a219)?
                                    };
                                    nat_mul(a220, a221)?
                                };
                                nat_add(a222, a223)?
                            };
                            nat_mul(a224, a225)?
                        };
                        nat_add(a226, a227)?
                    };
                    nat_mul(a228, a229)?
                })
            } else {
                None::<u64>
            }
        }))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f6(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(h230, _, _, _, _, _, _, _, _, _, _, _) => {
            h230
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(_, h231, _, _, _, _, _, _, _, _, _, _) => {
            h231
        }
    };
    let v6: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h232, _, _, _, _, _, _, _, _, _) => {
            h232
        }
    };
    let a234 = v1;
    let a235 = match v6.clone() {
        Adt1::C0(_, _, h233, _, _, _, _, _, _, _, _) => {
            h233
        }
    };
    if if if nat_lt(a234, a235) {
        let a237 = v2;
        let a238 = match v6.clone() {
            Adt1::C0(_, _, _, _, h236, _, _, _, _, _, _) => {
                h236
            }
        };
        nat_lt(a237, a238)
    } else {
        false
    } {
        let a240 = v3;
        let a241 = match v6.clone() {
            Adt1::C0(_, _, _, h239, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_lt(a240, a241)
    } else {
        false
    } {
        let v7: u64 = {
            let a245 = match v6.clone() {
                Adt1::C0(_, _, _, h242, _, _, _, _, _, _, _) => {
                    h242
                }
            };
            let a246 = {
                let a243 = v3;
                let a244 = 1u64;
                nat_add(a243, a244)?
            };
            nat_sub(a245, a246)
        };
        Ok(Adt7::C14(f16(v4.clone(), {
            let a248 = v1;
            let a249 = match v4.clone() {
                Adt1::C0(_, _, h247, _, _, _, _, _, _, _, _) => {
                    h247
                }
            };
            let a250 = 0u64;
            nat_rem(a248, a249, a250)
        }, v3, {
            let a256 = v2;
            let a257 = {
                let a253 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h251, _, _, _, _, _, _) => {
                        h251
                    }
                };
                let a254 = match v4 {
                    Adt1::C0(_, _, _, _, h252, _, _, _, _, _, _) => {
                        h252
                    }
                };
                let a255 = 0u64;
                nat_quot(a253, a254, a255)
            };
            let a258 = 0u64;
            nat_quot(a256, a257, a258)
        })?, f16(v5.clone(), {
            let a260 = v1;
            let a261 = match v5.clone() {
                Adt1::C0(_, _, h259, _, _, _, _, _, _, _, _) => {
                    h259
                }
            };
            let a262 = 0u64;
            nat_rem(a260, a261, a262)
        }, v3, {
            let a268 = v2;
            let a269 = {
                let a265 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h263, _, _, _, _, _, _) => {
                        h263
                    }
                };
                let a266 = match v5 {
                    Adt1::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                        h264
                    }
                };
                let a267 = 0u64;
                nat_quot(a265, a266, a267)
            };
            let a270 = 0u64;
            nat_quot(a268, a269, a270)
        })?, f16(v6.clone(), v1, v3, v2)?, f16(match v0.clone() {
            Adt6::C0(_, _, _, h271, _, _, _, _, _, _, _, _) => {
                h271
            }
        }, v1, v3, v2)?, f16(match v0.clone() {
            Adt6::C0(_, _, _, _, h272, _, _, _, _, _, _, _) => {
                h272
            }
        }, v1, v3, v2)?, {
            let a286 = 4u64;
            let a287 = {
                let a284 = match v6.clone() {
                    Adt1::C0(_, h273, _, _, _, _, _, _, _, _, _) => {
                        h273
                    }
                };
                let a285 = {
                    let a282 = v1;
                    let a283 = {
                        let a280 = match v6.clone() {
                            Adt1::C0(_, _, h274, _, _, _, _, _, _, _, _) => {
                                h274
                            }
                        };
                        let a281 = {
                            let a278 = v3;
                            let a279 = {
                                let a276 = match v6.clone() {
                                    Adt1::C0(_, _, _, h275, _, _, _, _, _, _, _) => {
                                        h275
                                    }
                                };
                                let a277 = v2;
                                nat_mul(a276, a277)?
                            };
                            nat_add(a278, a279)?
                        };
                        nat_mul(a280, a281)?
                    };
                    nat_add(a282, a283)?
                };
                nat_mul(a284, a285)?
            };
            nat_mul(a286, a287)?
        }, {
            let a289 = v7;
            let a290 = match v0 {
                Adt6::C0(_, _, _, _, _, _, h288, _, _, _, _, _) => {
                    h288
                }
            };
            if nat_lt(a289, a290) {
                Some(f24(v6, v1, v2, v7)?)
            } else {
                None::<u64>
            }
        }))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f7(v0: bool, v1: u64, v2: Option<Adt1>, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Option<Adt1>, v8: Adt1) -> R<Adt7> {
    if v0 {
        Ok(Adt7::C0)
    } else {
        let m291 = v2;
        match m291 {
            None => {
                Ok(Adt7::C0)
            }
            Some(v9) => {
                let m292 = v3;
                match m292 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v10) => {
                        let m293 = v4;
                        match m293 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v11) => {
                                let m294 = v5;
                                match m294 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v12) => {
                                        let m295 = v6;
                                        match m295 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v13) => {
                                                let m296 = v7;
                                                match m296 {
                                                    None => {
                                                        Ok(Adt7::C0)
                                                    }
                                                    Some(v14) => {
                                                        f25(v1, v9, v10, v11, v12, v13, v14, v8)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: u64, v6: Option<Adt1>, v7: Option<Adt1>, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt7> {
    let m297 = v6;
    match m297 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a298 = v0;
            let a299 = 43u64;
            if nat_eq(a298, a299) {
                let m300 = v9;
                match m300 {
                    None => {
                        let m301 = v10;
                        match m301 {
                            None => {
                                f13(v0, v1, v2, v3, v12, v7, v8, v11)
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
                let m302 = v7;
                match m302 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v15) => {
                        let a303 = v0;
                        let a304 = 84u64;
                        if nat_eq(a303, a304) {
                            f26(v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let a305 = v0;
                            let a306 = 88u64;
                            if nat_eq(a305, a306) {
                                f10(v0, v1, v12, v15, v8, v9, v10, v11)
                            } else {
                                let m307 = v8;
                                match m307 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v16) => {
                                        f9(v0, v1, v3, v4, v5, v12, v15, v16, v9, v10, None::<Adt1>, v11)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt7> {
    let m308 = f27(v10, v8.clone(), v5.clone(), v6.clone(), v7.clone(), v11.clone())?;
    match m308 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a309 = v0;
            let a310 = 71u64;
            if if if if if if if if if if if if if if if if if if nat_eq(a309, a310) {
                !v1
            } else {
                false
            } {
                f28(v2, match v5.clone() {
                    Adt1::C0(_, _, _, h311, _, _, _, _, _, _, _) => {
                        h311
                    }
                }, v8.clone())
            } else {
                false
            } {
                let a312 = v4;
                let a313 = 0u64;
                if if if nat_eq(a312, a313) {
                    true
                } else {
                    let a314 = v4;
                    let a315 = 10u64;
                    nat_eq(a314, a315)
                } {
                    true
                } else {
                    let a316 = v4;
                    let a317 = 15u64;
                    nat_eq(a316, a317)
                } {
                    true
                } else {
                    let a318 = v4;
                    let a319 = 20u64;
                    nat_eq(a318, a319)
                }
            } else {
                false
            } {
                let a321 = match v5.clone() {
                    Adt1::C0(h320, _, _, _, _, _, _, _, _, _, _) => {
                        h320
                    }
                };
                let a322 = 0u64;
                nat_eq(a321, a322)
            } else {
                false
            } {
                let a324 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h323, _, _, _, _, _) => {
                        h323
                    }
                };
                let a325 = 4u64;
                nat_eq(a324, a325)
            } else {
                false
            } {
                f29(v11.clone())?
            } else {
                false
            } {
                f30(v6.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                f30(v7.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                let a328 = match v5.clone() {
                    Adt1::C0(_, h326, _, _, _, _, _, _, _, _, _) => {
                        h326
                    }
                };
                let a329 = match v6.clone() {
                    Adt1::C0(_, h327, _, _, _, _, _, _, _, _, _) => {
                        h327
                    }
                };
                nat_eq(a328, a329)
            } else {
                false
            } {
                let a332 = match v6.clone() {
                    Adt1::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                        h330
                    }
                };
                let a333 = match v7.clone() {
                    Adt1::C0(_, _, h331, _, _, _, _, _, _, _, _) => {
                        h331
                    }
                };
                nat_eq(a332, a333)
            } else {
                false
            } {
                let a336 = match v11.clone() {
                    Adt1::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                        h334
                    }
                };
                let a337 = match v7.clone() {
                    Adt1::C0(_, h335, _, _, _, _, _, _, _, _, _) => {
                        h335
                    }
                };
                nat_eq(a336, a337)
            } else {
                false
            } {
                let a340 = match v11.clone() {
                    Adt1::C0(_, _, h338, _, _, _, _, _, _, _, _) => {
                        h338
                    }
                };
                let a341 = match v5.clone() {
                    Adt1::C0(_, _, _, h339, _, _, _, _, _, _, _) => {
                        h339
                    }
                };
                nat_eq(a340, a341)
            } else {
                false
            } {
                let a344 = match v11.clone() {
                    Adt1::C0(_, _, _, h342, _, _, _, _, _, _, _) => {
                        h342
                    }
                };
                let a345 = match v5.clone() {
                    Adt1::C0(_, _, h343, _, _, _, _, _, _, _, _) => {
                        h343
                    }
                };
                nat_eq(a344, a345)
            } else {
                false
            } {
                let a348 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h346, _, _, _, _, _, _) => {
                        h346
                    }
                };
                let a349 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h347, _, _, _, _, _, _) => {
                        h347
                    }
                };
                nat_eq(a348, a349)
            } else {
                false
            } {
                f23(v5.clone(), v11.clone())?
            } else {
                false
            } {
                f31(v8.clone(), v5.clone(), v12, v11.clone())?
            } else {
                false
            } {
                f32(v9.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                Ok(Adt7::C1(Adt2::C0(v5, v6, v7, v8, v9, v11, v12, f33(v2), f34(v3))))
            } else {
                Ok(Adt7::C0)
            }
        }
    }
}

pub fn f10(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    let v8: u64 = {
        let a351 = match v2.clone() {
            Adt1::C0(h350, _, _, _, _, _, _, _, _, _, _) => {
                h350
            }
        };
        let a352 = 1u64;
        if nat_eq(a351, a352) {
            2u64
        } else {
            4u64
        }
    };
    let a353 = v0;
    let a354 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a353, a354) {
        !v1
    } else {
        false
    } {
        let a356 = match v2.clone() {
            Adt1::C0(h355, _, _, _, _, _, _, _, _, _, _) => {
                h355
            }
        };
        let a357 = 0u64;
        if nat_eq(a356, a357) {
            true
        } else {
            let a359 = match v2.clone() {
                Adt1::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                    h358
                }
            };
            let a360 = 1u64;
            nat_eq(a359, a360)
        }
    } else {
        false
    } {
        let a362 = match v3.clone() {
            Adt1::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
                h361
            }
        };
        let a363 = 26u64;
        nat_eq(a362, a363)
    } else {
        false
    } {
        let a366 = match v7.clone() {
            Adt1::C0(h364, _, _, _, _, _, _, _, _, _, _) => {
                h364
            }
        };
        let a367 = match v2.clone() {
            Adt1::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
                h365
            }
        };
        nat_eq(a366, a367)
    } else {
        false
    } {
        let a369 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h368, _, _, _, _, _) => {
                h368
            }
        };
        let a370 = v8;
        nat_eq(a369, a370)
    } else {
        false
    } {
        let a372 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h371, _, _, _, _, _) => {
                h371
            }
        };
        let a373 = v8;
        nat_eq(a372, a373)
    } else {
        false
    } {
        let a376 = match v2.clone() {
            Adt1::C0(_, h374, _, _, _, _, _, _, _, _, _) => {
                h374
            }
        };
        let a377 = match v7.clone() {
            Adt1::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        nat_eq(a376, a377)
    } else {
        false
    } {
        f35(v3.clone(), v7.clone())
    } else {
        false
    } {
        f36({
            let a379 = v8;
            let a380 = match v7.clone() {
                Adt1::C0(_, h378, _, _, _, _, _, _, _, _, _) => {
                    h378
                }
            };
            nat_mul(a379, a380)?
        }, match v7.clone() {
            Adt1::C0(_, _, h381, _, _, _, _, _, _, _, _) => {
                h381
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h382, _, _, _, _, _, _, _) => {
                h382
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h383, _, _, _, _, _, _) => {
                h383
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h384, _, _, _, _) => {
                h384
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h385, _, _, _) => {
                h385
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h386, _, _) => {
                h386
            }
        })?
    } else {
        false
    } {
        f23(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f23(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f17(match v3.clone() {
            Adt1::C0(_, _, h387, _, _, _, _, _, _, _, _) => {
                h387
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h388, _, _, _, _, _, _, _, _) => {
                h388
            }
        })
    } else {
        false
    } {
        f17(match v3.clone() {
            Adt1::C0(_, _, _, h389, _, _, _, _, _, _, _) => {
                h389
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h390, _, _, _, _, _, _, _) => {
                h390
            }
        })
    } else {
        false
    } {
        f17(match v3.clone() {
            Adt1::C0(_, _, _, _, h391, _, _, _, _, _, _) => {
                h391
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h392, _, _, _, _, _, _) => {
                h392
            }
        })
    } else {
        false
    } {
        let m393 = v4.clone();
        match m393 {
            None => {
                true
            }
            Some(v9) => {
                let a395 = match v9.clone() {
                    Adt1::C0(h394, _, _, _, _, _, _, _, _, _, _) => {
                        h394
                    }
                };
                let a396 = 0u64;
                if if nat_eq(a395, a396) {
                    f35(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f23(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m397 = v5.clone();
        match m397 {
            None => {
                true
            }
            Some(v10) => {
                let a399 = match v10.clone() {
                    Adt1::C0(h398, _, _, _, _, _, _, _, _, _, _) => {
                        h398
                    }
                };
                let a400 = 26u64;
                if if if if nat_eq(a399, a400) {
                    let a402 = match v10.clone() {
                        Adt1::C0(_, _, _, h401, _, _, _, _, _, _, _) => {
                            h401
                        }
                    };
                    let a403 = 1u64;
                    nat_eq(a402, a403)
                } else {
                    false
                } {
                    let a405 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h404, _, _, _, _, _, _) => {
                            h404
                        }
                    };
                    let a406 = 1u64;
                    nat_eq(a405, a406)
                } else {
                    false
                } {
                    f23(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m407 = v4.clone();
                    match m407 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a410 = match v11 {
                                Adt1::C0(_, h408, _, _, _, _, _, _, _, _, _) => {
                                    h408
                                }
                            };
                            let a411 = match v10 {
                                Adt1::C0(_, _, h409, _, _, _, _, _, _, _, _) => {
                                    h409
                                }
                            };
                            nat_eq(a410, a411)
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
        let m412 = v6.clone();
        match m412 {
            None => {
                true
            }
            Some(v12) => {
                let a414 = match v12.clone() {
                    Adt1::C0(h413, _, _, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                };
                let a415 = 26u64;
                if if nat_eq(a414, a415) {
                    f35(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f23(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt7::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a416 = v8;
            let a417 = 2u64;
            if nat_eq(a416, a417) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m418 = v5;
            match m418 {
                None => {
                    let m419 = v4;
                    match m419 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h420, _, _, _, _, _, _, _, _, _) => {
                                    h420
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h421, _, _, _, _, _, _, _, _, _) => {
                                    h421
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h422, _, _, _, _, _, _, _, _) => {
                            h422
                        }
                    }
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f11(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt4::C0(h423, _, _, _, _, _, _, _, _) => {
            h423
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h424, _, _, _, _, _, _, _) => {
            h424
        }
    };
    let a426 = v1;
    let a427 = match v5.clone() {
        Adt1::C0(_, _, h425, _, _, _, _, _, _, _, _) => {
            h425
        }
    };
    if if if nat_lt(a426, a427) {
        let a429 = v2;
        let a430 = match v5.clone() {
            Adt1::C0(_, _, _, h428, _, _, _, _, _, _, _) => {
                h428
            }
        };
        nat_lt(a429, a430)
    } else {
        false
    } {
        let a432 = v3;
        let a433 = match v5.clone() {
            Adt1::C0(_, _, _, _, h431, _, _, _, _, _, _) => {
                h431
            }
        };
        nat_lt(a432, a433)
    } else {
        false
    } {
        Ok(Adt7::C6(f16(v4.clone(), {
            let a435 = v1;
            let a436 = match v4.clone() {
                Adt1::C0(_, _, h434, _, _, _, _, _, _, _, _) => {
                    h434
                }
            };
            let a437 = 0u64;
            nat_rem(a435, a436, a437)
        }, {
            let a439 = v2;
            let a440 = match v4.clone() {
                Adt1::C0(_, _, _, h438, _, _, _, _, _, _, _) => {
                    h438
                }
            };
            let a441 = 0u64;
            nat_rem(a439, a440, a441)
        }, {
            let a443 = v3;
            let a444 = match v4 {
                Adt1::C0(_, _, _, _, h442, _, _, _, _, _, _) => {
                    h442
                }
            };
            let a445 = 0u64;
            nat_rem(a443, a444, a445)
        })?, f16(v5, v1, v2, v3)?, {
            let m447 = match v0.clone() {
                Adt4::C0(_, _, h446, _, _, _, _, _, _) => {
                    h446
                }
            };
            match m447 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f16(v6, v1, v2, v3)?
                }
            }
        }, {
            let m449 = match v0.clone() {
                Adt4::C0(_, _, _, _, h448, _, _, _, _) => {
                    h448
                }
            };
            match m449 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f16(v7, v1, v2, v3)?
                }
            }
        }, f16(match v0 {
            Adt4::C0(_, _, _, _, _, h450, _, _, _) => {
                h450
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f12(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a451 = v1;
    let a452 = 0u64;
    if nat_eq(a451, a452) {
        let a456 = v2;
        let a457 = {
            let m455 = match v0.clone() {
                Adt4::C0(_, h454, _, _, _, _, _, _, _) => {
                    h454
                }
            };
            match m455 {
                Adt1::C0(_, h453, _, _, _, _, _, _, _, _, _) => {
                    h453
                }
            }
        };
        if nat_lt(a456, a457) {
            Ok(Adt7::C7({
                let a461 = v2;
                let a462 = {
                    let m460 = match v0 {
                        Adt4::C0(_, h459, _, _, _, _, _, _, _) => {
                            h459
                        }
                    };
                    match m460 {
                        Adt1::C0(_, _, _, _, _, h458, _, _, _, _, _) => {
                            h458
                        }
                    }
                };
                nat_mul(a461, a462)?
            }))
        } else {
            Ok(Adt7::C8)
        }
    } else {
        let a463 = v1;
        let a464 = 1u64;
        if nat_eq(a463, a464) {
            let m466 = match v0 {
                Adt4::C0(_, _, _, _, h465, _, _, _, _) => {
                    h465
                }
            };
            match m466 {
                None => {
                    Ok(Adt7::C8)
                }
                Some(v4) => {
                    let a468 = v2;
                    let a469 = match v4.clone() {
                        Adt1::C0(_, h467, _, _, _, _, _, _, _, _, _) => {
                            h467
                        }
                    };
                    if nat_lt(a468, a469) {
                        Ok(Adt7::C7({
                            let a471 = v2;
                            let a472 = match v4 {
                                Adt1::C0(_, _, _, _, _, h470, _, _, _, _, _) => {
                                    h470
                                }
                            };
                            nat_mul(a471, a472)?
                        }))
                    } else {
                        Ok(Adt7::C8)
                    }
                }
            }
        } else {
            let a473 = v1;
            let a474 = 2u64;
            if nat_eq(a473, a474) {
                let a475 = v2;
                let a476 = 2147483648u64;
                if if nat_lt(a475, a476) {
                    let a478 = v2;
                    let a479 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h477) => {
                            h477
                        }
                    };
                    nat_lt(a478, a479)
                } else {
                    false
                } {
                    let m481 = match v0 {
                        Adt4::C0(_, _, h480, _, _, _, _, _, _) => {
                            h480
                        }
                    };
                    match m481 {
                        None => {
                            Ok(Adt7::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt7::C7({
                                let a483 = v2;
                                let a484 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h482, _, _, _, _, _) => {
                                        h482
                                    }
                                };
                                nat_mul(a483, a484)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt7::C8)
                }
            } else {
                let a485 = v1;
                let a486 = 3u64;
                if nat_eq(a485, a486) {
                    let m488 = match v0 {
                        Adt4::C0(_, _, _, h487, _, _, _, _, _) => {
                            h487
                        }
                    };
                    match m488 {
                        None => {
                            Ok(Adt7::C8)
                        }
                        Some(v6) => {
                            let a490 = v2;
                            let a491 = match v6.clone() {
                                Adt1::C0(_, _, h489, _, _, _, _, _, _, _, _) => {
                                    h489
                                }
                            };
                            if if nat_lt(a490, a491) {
                                let a493 = v3;
                                let a494 = match v6.clone() {
                                    Adt1::C0(_, h492, _, _, _, _, _, _, _, _, _) => {
                                        h492
                                    }
                                };
                                nat_lt(a493, a494)
                            } else {
                                false
                            } {
                                Ok(Adt7::C7({
                                    let a501 = {
                                        let a496 = v2;
                                        let a497 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h495, _, _, _, _) => {
                                                h495
                                            }
                                        };
                                        nat_mul(a496, a497)?
                                    };
                                    let a502 = {
                                        let a499 = v3;
                                        let a500 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h498, _, _, _, _, _) => {
                                                h498
                                            }
                                        };
                                        nat_mul(a499, a500)?
                                    };
                                    nat_add(a501, a502)?
                                }))
                            } else {
                                Ok(Adt7::C8)
                            }
                        }
                    }
                } else {
                    let a503 = v1;
                    let a504 = 4u64;
                    if nat_eq(a503, a504) {
                        let a505 = v2;
                        let a506 = 2147483648u64;
                        if if nat_lt(a505, a506) {
                            let a510 = v2;
                            let a511 = {
                                let m509 = match v0.clone() {
                                    Adt4::C0(h508, _, _, _, _, _, _, _, _) => {
                                        h508
                                    }
                                };
                                match m509 {
                                    Adt1::C0(_, h507, _, _, _, _, _, _, _, _, _) => {
                                        h507
                                    }
                                }
                            };
                            nat_lt(a510, a511)
                        } else {
                            false
                        } {
                            Ok(Adt7::C7({
                                let a513 = v2;
                                let a514 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h512, _, _) => {
                                        h512
                                    }
                                };
                                nat_mul(a513, a514)?
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

pub fn f13(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    if v1 {
        Ok(Adt7::C0)
    } else {
        let a515 = v0;
        let a516 = 43u64;
        if if if if if if if if nat_eq(a515, a516) {
            let a518 = match v4.clone() {
                Adt1::C0(h517, _, _, _, _, _, _, _, _, _, _) => {
                    h517
                }
            };
            let a519 = 0u64;
            if nat_eq(a518, a519) {
                f20(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a521 = match v7.clone() {
                Adt1::C0(h520, _, _, _, _, _, _, _, _, _, _) => {
                    h520
                }
            };
            let a522 = 0u64;
            if nat_eq(a521, a522) {
                f20(v7.clone(), 4u64)?
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
            if f23(v4.clone(), v7.clone())? {
                true
            } else {
                f37(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f38(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f39(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f28(v3, match v4.clone() {
                Adt1::C0(_, _, _, h523, _, _, _, _, _, _, _) => {
                    h523
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt7::C3(Adt3::C0(v4, v5, v7, v2, f33(v3))))
        } else {
            Ok(Adt7::C0)
        }
    }
}

pub fn f14(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h524, _, _, _, _) => {
            h524
        }
    };
    let a526 = v1;
    let a527 = match v4.clone() {
        Adt1::C0(_, _, h525, _, _, _, _, _, _, _, _) => {
            h525
        }
    };
    if if if nat_lt(a526, a527) {
        let a529 = v2;
        let a530 = match v4.clone() {
            Adt1::C0(_, _, _, h528, _, _, _, _, _, _, _) => {
                h528
            }
        };
        nat_lt(a529, a530)
    } else {
        false
    } {
        let a532 = v3;
        let a533 = match v4.clone() {
            Adt1::C0(_, _, _, _, h531, _, _, _, _, _, _) => {
                h531
            }
        };
        nat_lt(a532, a533)
    } else {
        false
    } {
        let v6: u64 = {
            let m535 = match v0.clone() {
                Adt3::C0(_, h534, _, _, _) => {
                    h534
                }
            };
            match m535 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f16(v5.clone(), v1, {
                        let a537 = v2;
                        let a538 = match v5.clone() {
                            Adt1::C0(_, _, _, h536, _, _, _, _, _, _, _) => {
                                h536
                            }
                        };
                        let a539 = 0u64;
                        nat_rem(a537, a538, a539)
                    }, {
                        let a541 = v3;
                        let a542 = match v5 {
                            Adt1::C0(_, _, _, _, h540, _, _, _, _, _, _) => {
                                h540
                            }
                        };
                        let a543 = 0u64;
                        nat_rem(a541, a542, a543)
                    })?
                }
            }
        };
        Ok(Adt7::C4(f16(v4, v1, v2, v3)?, v6, {
            let a544 = v2;
            let a545 = 4u64;
            nat_mul(a544, a545)?
        }, f16(match v0 {
            Adt3::C0(_, _, h546, _, _) => {
                h546
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f15(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a550 = v1;
    let a551 = {
        let m549 = match v0.clone() {
            Adt2::C0(h548, _, _, _, _, _, _, _, _) => {
                h548
            }
        };
        match m549 {
            Adt1::C0(_, _, h547, _, _, _, _, _, _, _, _) => {
                h547
            }
        }
    };
    if if if nat_lt(a550, a551) {
        let a555 = v2;
        let a556 = {
            let m554 = match v0.clone() {
                Adt2::C0(h553, _, _, _, _, _, _, _, _) => {
                    h553
                }
            };
            match m554 {
                Adt1::C0(_, _, _, h552, _, _, _, _, _, _, _) => {
                    h552
                }
            }
        };
        nat_lt(a555, a556)
    } else {
        false
    } {
        let a560 = v3;
        let a561 = {
            let m559 = match v0.clone() {
                Adt2::C0(h558, _, _, _, _, _, _, _, _) => {
                    h558
                }
            };
            match m559 {
                Adt1::C0(_, _, _, _, h557, _, _, _, _, _, _) => {
                    h557
                }
            }
        };
        nat_lt(a560, a561)
    } else {
        false
    } {
        Ok(Adt7::C2(f16(match v0.clone() {
            Adt2::C0(h562, _, _, _, _, _, _, _, _) => {
                h562
            }
        }, v1, v2, v3)?, f16(match v0.clone() {
            Adt2::C0(_, h563, _, _, _, _, _, _, _) => {
                h563
            }
        }, 0u64, {
            let a573 = v2;
            let a574 = {
                let a570 = {
                    let m566 = match v0.clone() {
                        Adt2::C0(h565, _, _, _, _, _, _, _, _) => {
                            h565
                        }
                    };
                    match m566 {
                        Adt1::C0(_, _, _, h564, _, _, _, _, _, _, _) => {
                            h564
                        }
                    }
                };
                let a571 = {
                    let m569 = match v0.clone() {
                        Adt2::C0(_, h568, _, _, _, _, _, _, _) => {
                            h568
                        }
                    };
                    match m569 {
                        Adt1::C0(_, _, _, h567, _, _, _, _, _, _, _) => {
                            h567
                        }
                    }
                };
                let a572 = 0u64;
                nat_quot(a570, a571, a572)
            };
            let a575 = 0u64;
            nat_quot(a573, a574, a575)
        }, {
            let a585 = v3;
            let a586 = {
                let a582 = {
                    let m578 = match v0.clone() {
                        Adt2::C0(h577, _, _, _, _, _, _, _, _) => {
                            h577
                        }
                    };
                    match m578 {
                        Adt1::C0(_, _, _, _, h576, _, _, _, _, _, _) => {
                            h576
                        }
                    }
                };
                let a583 = {
                    let m581 = match v0.clone() {
                        Adt2::C0(_, h580, _, _, _, _, _, _, _) => {
                            h580
                        }
                    };
                    match m581 {
                        Adt1::C0(_, _, _, _, h579, _, _, _, _, _, _) => {
                            h579
                        }
                    }
                };
                let a584 = 0u64;
                nat_quot(a582, a583, a584)
            };
            let a587 = 0u64;
            nat_quot(a585, a586, a587)
        })?, f16(match v0.clone() {
            Adt2::C0(_, _, h588, _, _, _, _, _, _) => {
                h588
            }
        }, 0u64, {
            let a598 = v2;
            let a599 = {
                let a595 = {
                    let m591 = match v0.clone() {
                        Adt2::C0(h590, _, _, _, _, _, _, _, _) => {
                            h590
                        }
                    };
                    match m591 {
                        Adt1::C0(_, _, _, h589, _, _, _, _, _, _, _) => {
                            h589
                        }
                    }
                };
                let a596 = {
                    let m594 = match v0.clone() {
                        Adt2::C0(_, _, h593, _, _, _, _, _, _) => {
                            h593
                        }
                    };
                    match m594 {
                        Adt1::C0(_, _, _, h592, _, _, _, _, _, _, _) => {
                            h592
                        }
                    }
                };
                let a597 = 0u64;
                nat_quot(a595, a596, a597)
            };
            let a600 = 0u64;
            nat_quot(a598, a599, a600)
        }, {
            let a610 = v3;
            let a611 = {
                let a607 = {
                    let m603 = match v0.clone() {
                        Adt2::C0(h602, _, _, _, _, _, _, _, _) => {
                            h602
                        }
                    };
                    match m603 {
                        Adt1::C0(_, _, _, _, h601, _, _, _, _, _, _) => {
                            h601
                        }
                    }
                };
                let a608 = {
                    let m606 = match v0.clone() {
                        Adt2::C0(_, _, h605, _, _, _, _, _, _) => {
                            h605
                        }
                    };
                    match m606 {
                        Adt1::C0(_, _, _, _, h604, _, _, _, _, _, _) => {
                            h604
                        }
                    }
                };
                let a609 = 0u64;
                nat_quot(a607, a608, a609)
            };
            let a612 = 0u64;
            nat_quot(a610, a611, a612)
        })?, {
            let m614 = match v0.clone() {
                Adt2::C0(_, _, _, h613, _, _, _, _, _) => {
                    h613
                }
            };
            match m614 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f16(v4.clone(), v1, {
                        let a616 = v2;
                        let a617 = match v4.clone() {
                            Adt1::C0(_, _, _, h615, _, _, _, _, _, _, _) => {
                                h615
                            }
                        };
                        let a618 = 0u64;
                        nat_rem(a616, a617, a618)
                    }, {
                        let a620 = v3;
                        let a621 = match v4 {
                            Adt1::C0(_, _, _, _, h619, _, _, _, _, _, _) => {
                                h619
                            }
                        };
                        let a622 = 0u64;
                        nat_rem(a620, a621, a622)
                    })?
                }
            }
        }, {
            let a623 = v2;
            let a624 = 4u64;
            nat_mul(a623, a624)?
        }, f16(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h625, _, _, _) => {
                h625
            }
        }, v2, v1, v3)?, {
            let m628 = match v0.clone() {
                Adt2::C0(_, h627, _, _, _, _, _, _, _) => {
                    h627
                }
            };
            match m628 {
                Adt1::C0(_, _, _, _, _, _, h626, _, _, _, _) => {
                    h626
                }
            }
        }, {
            let m631 = match v0.clone() {
                Adt2::C0(_, _, h630, _, _, _, _, _, _) => {
                    h630
                }
            };
            match m631 {
                Adt1::C0(_, _, _, _, _, _, h629, _, _, _, _) => {
                    h629
                }
            }
        }, f40({
            let m634 = match v0 {
                Adt2::C0(_, h633, _, _, _, _, _, _, _) => {
                    h633
                }
            };
            match m634 {
                Adt1::C0(h632, _, _, _, _, _, _, _, _, _, _) => {
                    h632
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f16(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a646 = {
        let a641 = {
            let a636 = v1;
            let a637 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h635, _, _, _, _) => {
                    h635
                }
            };
            nat_mul(a636, a637)?
        };
        let a642 = {
            let a639 = v2;
            let a640 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h638, _, _, _) => {
                    h638
                }
            };
            nat_mul(a639, a640)?
        };
        nat_add(a641, a642)?
    };
    let a647 = {
        let a644 = v3;
        let a645 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h643, _, _) => {
                h643
            }
        };
        nat_mul(a644, a645)?
    };
    nat_add(a646, a647)
}

pub fn f17(v0: u64, v1: u64) -> bool {
    let a648 = 0u64;
    let a649 = v1;
    if nat_lt(a648, a649) {
        let a653 = {
            let a650 = v0;
            let a651 = v1;
            let a652 = 0u64;
            nat_rem(a650, a651, a652)
        };
        let a654 = 0u64;
        nat_eq(a653, a654)
    } else {
        false
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> R<bool> {
    let a656 = match v0.clone() {
        Adt1::C0(h655, _, _, _, _, _, _, _, _, _, _) => {
            h655
        }
    };
    let a657 = 0u64;
    if if nat_eq(a656, a657) {
        let a659 = match v0.clone() {
            Adt1::C0(_, h658, _, _, _, _, _, _, _, _, _) => {
                h658
            }
        };
        let a660 = 1u64;
        if nat_eq(a659, a660) {
            true
        } else {
            let a662 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h661, _, _, _, _, _) => {
                    h661
                }
            };
            let a663 = 4u64;
            nat_eq(a662, a663)
        }
    } else {
        false
    } {
        f23(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt1) -> R<bool> {
    let a665 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h664, _, _, _, _, _) => {
            h664
        }
    };
    let a666 = 4u64;
    if nat_eq(a665, a666) {
        let a671 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h667, _, _, _, _) => {
                h667
            }
        };
        let a672 = {
            let a669 = 4u64;
            let a670 = match v0 {
                Adt1::C0(_, h668, _, _, _, _, _, _, _, _, _) => {
                    h668
                }
            };
            nat_mul(a669, a670)?
        };
        Ok(nat_eq(a671, a672))
    } else {
        Ok(false)
    }
}

pub fn f20(v0: Adt1, v1: u64) -> R<bool> {
    let a674 = match v0.clone() {
        Adt1::C0(_, h673, _, _, _, _, _, _, _, _, _) => {
            h673
        }
    };
    let a675 = 1u64;
    if if if if nat_eq(a674, a675) {
        true
    } else {
        let a677 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h676, _, _, _, _, _) => {
                h676
            }
        };
        let a678 = v1;
        nat_eq(a677, a678)
    } {
        let a680 = match v0.clone() {
            Adt1::C0(_, _, h679, _, _, _, _, _, _, _, _) => {
                h679
            }
        };
        let a681 = 1u64;
        if nat_eq(a680, a681) {
            true
        } else {
            let a686 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h682, _, _, _, _) => {
                    h682
                }
            };
            let a687 = {
                let a684 = v1;
                let a685 = match v0.clone() {
                    Adt1::C0(_, h683, _, _, _, _, _, _, _, _, _) => {
                        h683
                    }
                };
                nat_mul(a684, a685)?
            };
            nat_eq(a686, a687)
        }
    } else {
        false
    } {
        let a689 = match v0.clone() {
            Adt1::C0(_, _, _, h688, _, _, _, _, _, _, _) => {
                h688
            }
        };
        let a690 = 1u64;
        if nat_eq(a689, a690) {
            true
        } else {
            let a698 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h691, _, _, _) => {
                    h691
                }
            };
            let a699 = {
                let a696 = {
                    let a693 = v1;
                    let a694 = match v0.clone() {
                        Adt1::C0(_, h692, _, _, _, _, _, _, _, _, _) => {
                            h692
                        }
                    };
                    nat_mul(a693, a694)?
                };
                let a697 = match v0.clone() {
                    Adt1::C0(_, _, h695, _, _, _, _, _, _, _, _) => {
                        h695
                    }
                };
                nat_mul(a696, a697)?
            };
            nat_eq(a698, a699)
        }
    } else {
        false
    } {
        let a701 = match v0.clone() {
            Adt1::C0(_, _, _, _, h700, _, _, _, _, _, _) => {
                h700
            }
        };
        let a702 = 1u64;
        if nat_eq(a701, a702) {
            Ok(true)
        } else {
            let a713 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h703, _, _) => {
                    h703
                }
            };
            let a714 = {
                let a711 = {
                    let a708 = {
                        let a705 = v1;
                        let a706 = match v0.clone() {
                            Adt1::C0(_, h704, _, _, _, _, _, _, _, _, _) => {
                                h704
                            }
                        };
                        nat_mul(a705, a706)?
                    };
                    let a709 = match v0.clone() {
                        Adt1::C0(_, _, h707, _, _, _, _, _, _, _, _) => {
                            h707
                        }
                    };
                    nat_mul(a708, a709)?
                };
                let a712 = match v0 {
                    Adt1::C0(_, _, _, h710, _, _, _, _, _, _, _) => {
                        h710
                    }
                };
                nat_mul(a711, a712)?
            };
            Ok(nat_eq(a713, a714))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    let a716 = match v0.clone() {
        Adt1::C0(_, h715, _, _, _, _, _, _, _, _, _) => {
            h715
        }
    };
    let a717 = v1;
    if if if nat_eq(a716, a717) {
        let a719 = match v0.clone() {
            Adt1::C0(_, _, h718, _, _, _, _, _, _, _, _) => {
                h718
            }
        };
        let a720 = v2;
        nat_eq(a719, a720)
    } else {
        false
    } {
        let a722 = match v0.clone() {
            Adt1::C0(_, _, _, h721, _, _, _, _, _, _, _) => {
                h721
            }
        };
        let a723 = v3;
        nat_eq(a722, a723)
    } else {
        false
    } {
        let a725 = match v0 {
            Adt1::C0(_, _, _, _, h724, _, _, _, _, _, _) => {
                h724
            }
        };
        let a726 = v4;
        nat_eq(a725, a726)
    } else {
        false
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> bool {
    let a729 = match v0.clone() {
        Adt1::C0(_, h727, _, _, _, _, _, _, _, _, _) => {
            h727
        }
    };
    let a730 = match v1.clone() {
        Adt1::C0(_, h728, _, _, _, _, _, _, _, _, _) => {
            h728
        }
    };
    if if if nat_eq(a729, a730) {
        let a733 = match v0.clone() {
            Adt1::C0(_, _, h731, _, _, _, _, _, _, _, _) => {
                h731
            }
        };
        let a734 = match v1.clone() {
            Adt1::C0(_, _, h732, _, _, _, _, _, _, _, _) => {
                h732
            }
        };
        nat_eq(a733, a734)
    } else {
        false
    } {
        let a737 = match v0.clone() {
            Adt1::C0(_, _, _, h735, _, _, _, _, _, _, _) => {
                h735
            }
        };
        let a738 = match v1.clone() {
            Adt1::C0(_, _, _, h736, _, _, _, _, _, _, _) => {
                h736
            }
        };
        nat_eq(a737, a738)
    } else {
        false
    } {
        let a741 = match v0 {
            Adt1::C0(_, _, _, _, h739, _, _, _, _, _, _) => {
                h739
            }
        };
        let a742 = match v1 {
            Adt1::C0(_, _, _, _, h740, _, _, _, _, _, _) => {
                h740
            }
        };
        nat_eq(a741, a742)
    } else {
        false
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> R<bool> {
    let a744 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h743, _) => {
            h743
        }
    };
    let a745 = 0u64;
    if nat_eq(a744, a745) {
        Ok(true)
    } else {
        let a747 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h746, _) => {
                h746
            }
        };
        let a748 = 0u64;
        if nat_eq(a747, a748) {
            Ok(true)
        } else {
            let a750 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h749) => {
                    h749
                }
            };
            let a751 = 0u64;
            if nat_eq(a750, a751) {
                Ok(true)
            } else {
                let a753 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h752) => {
                        h752
                    }
                };
                let a754 = 0u64;
                if nat_eq(a753, a754) {
                    Ok(true)
                } else {
                    let a760 = {
                        let a757 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h755) => {
                                h755
                            }
                        };
                        let a758 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h756, _) => {
                                h756
                            }
                        };
                        nat_add(a757, a758)?
                    };
                    let a761 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h759) => {
                            h759
                        }
                    };
                    if nat_le(a760, a761) {
                        Ok(true)
                    } else {
                        let a767 = {
                            let a764 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h762) => {
                                    h762
                                }
                            };
                            let a765 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h763, _) => {
                                    h763
                                }
                            };
                            nat_add(a764, a765)?
                        };
                        let a768 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h766) => {
                                h766
                            }
                        };
                        Ok(nat_le(a767, a768))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h769, _, _, _, _, _, _, _, _, _) => {
            h769
        }
    };
    let v5: u64 = match v0.clone() {
        Adt1::C0(_, _, h770, _, _, _, _, _, _, _, _) => {
            h770
        }
    };
    let v6: u64 = match v0.clone() {
        Adt1::C0(_, _, _, h771, _, _, _, _, _, _, _) => {
            h771
        }
    };
    let v7: u64 = match v0 {
        Adt1::C0(_, _, _, _, h772, _, _, _, _, _, _) => {
            h772
        }
    };
    let a793 = 4u64;
    let a794 = {
        let a791 = {
            let a777 = {
                let a773 = v4;
                let a774 = v5;
                nat_mul(a773, a774)?
            };
            let a778 = {
                let a775 = v6;
                let a776 = v7;
                nat_mul(a775, a776)?
            };
            nat_mul(a777, a778)?
        };
        let a792 = {
            let a789 = {
                let a779 = v4;
                let a780 = v4;
                nat_mul(a779, a780)?
            };
            let a790 = {
                let a787 = v1;
                let a788 = {
                    let a785 = v5;
                    let a786 = {
                        let a783 = v2;
                        let a784 = {
                            let a781 = v7;
                            let a782 = v3;
                            nat_mul(a781, a782)?
                        };
                        nat_add(a783, a784)?
                    };
                    nat_mul(a785, a786)?
                };
                nat_add(a787, a788)?
            };
            nat_mul(a789, a790)?
        };
        nat_add(a791, a792)?
    };
    nat_mul(a793, a794)
}

pub fn f25(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt7> {
    let v8: u64 = match v3.clone() {
        Adt1::C0(_, h795, _, _, _, _, _, _, _, _, _) => {
            h795
        }
    };
    let v9: u64 = match v3.clone() {
        Adt1::C0(_, _, h796, _, _, _, _, _, _, _, _) => {
            h796
        }
    };
    let v10: u64 = match v3.clone() {
        Adt1::C0(_, _, _, h797, _, _, _, _, _, _, _) => {
            h797
        }
    };
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, _, _, _, h798, _, _, _, _, _, _) => {
            h798
        }
    };
    let a799 = 0u64;
    let a800 = v8;
    if if if if if if if if if if if if if if if if if if if if if if if nat_lt(a799, a800) {
        let a801 = 0u64;
        let a802 = v9;
        nat_lt(a801, a802)
    } else {
        false
    } {
        let a803 = 0u64;
        let a804 = v11;
        nat_lt(a803, a804)
    } else {
        false
    } {
        let a805 = 0u64;
        let a806 = v0;
        nat_lt(a805, a806)
    } else {
        false
    } {
        let a807 = v0;
        let a808 = 2147483647u64;
        nat_le(a807, a808)
    } else {
        false
    } {
        f18(v1.clone(), v7.clone())?
    } else {
        false
    } {
        f18(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f18(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f18(v4.clone(), v7.clone())?
    } else {
        false
    } {
        f18(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f18(v6.clone(), v7.clone())?
    } else {
        false
    } {
        f41(v1.clone(), v3.clone())
    } else {
        false
    } {
        f41(v2.clone(), v3.clone())
    } else {
        false
    } {
        let a810 = match v4.clone() {
            Adt1::C0(_, h809, _, _, _, _, _, _, _, _, _) => {
                h809
            }
        };
        let a811 = 1u64;
        if nat_eq(a810, a811) {
            true
        } else {
            let a813 = match v4.clone() {
                Adt1::C0(_, h812, _, _, _, _, _, _, _, _, _) => {
                    h812
                }
            };
            let a814 = v8;
            nat_eq(a813, a814)
        }
    } else {
        false
    } {
        let a817 = match v4.clone() {
            Adt1::C0(_, h815, _, _, _, _, _, _, _, _, _) => {
                h815
            }
        };
        let a818 = match v4.clone() {
            Adt1::C0(_, h816, _, _, _, _, _, _, _, _, _) => {
                h816
            }
        };
        if if if nat_eq(a817, a818) {
            let a820 = match v4.clone() {
                Adt1::C0(_, _, h819, _, _, _, _, _, _, _, _) => {
                    h819
                }
            };
            let a821 = v9;
            nat_eq(a820, a821)
        } else {
            false
        } {
            let a823 = match v4.clone() {
                Adt1::C0(_, _, _, h822, _, _, _, _, _, _, _) => {
                    h822
                }
            };
            let a824 = v10;
            nat_eq(a823, a824)
        } else {
            false
        } {
            let a826 = match v4.clone() {
                Adt1::C0(_, _, _, _, h825, _, _, _, _, _, _) => {
                    h825
                }
            };
            let a827 = v11;
            nat_eq(a826, a827)
        } else {
            false
        }
    } else {
        false
    } {
        let a829 = match v5.clone() {
            Adt1::C0(_, h828, _, _, _, _, _, _, _, _, _) => {
                h828
            }
        };
        let a830 = 1u64;
        if if if nat_eq(a829, a830) {
            let a832 = match v5.clone() {
                Adt1::C0(_, _, h831, _, _, _, _, _, _, _, _) => {
                    h831
                }
            };
            let a833 = v9;
            nat_eq(a832, a833)
        } else {
            false
        } {
            let a835 = match v5.clone() {
                Adt1::C0(_, _, _, h834, _, _, _, _, _, _, _) => {
                    h834
                }
            };
            let a836 = v10;
            nat_eq(a835, a836)
        } else {
            false
        } {
            let a838 = match v5.clone() {
                Adt1::C0(_, _, _, _, h837, _, _, _, _, _, _) => {
                    h837
                }
            };
            let a839 = v11;
            nat_eq(a838, a839)
        } else {
            false
        }
    } else {
        false
    } {
        let a841 = match v6.clone() {
            Adt1::C0(_, h840, _, _, _, _, _, _, _, _, _) => {
                h840
            }
        };
        let a842 = v8;
        if if if nat_eq(a841, a842) {
            let a844 = match v6.clone() {
                Adt1::C0(_, _, h843, _, _, _, _, _, _, _, _) => {
                    h843
                }
            };
            let a845 = v8;
            nat_eq(a844, a845)
        } else {
            false
        } {
            let a847 = match v6.clone() {
                Adt1::C0(_, _, _, h846, _, _, _, _, _, _, _) => {
                    h846
                }
            };
            let a848 = v9;
            nat_eq(a847, a848)
        } else {
            false
        } {
            let a850 = match v6.clone() {
                Adt1::C0(_, _, _, _, h849, _, _, _, _, _, _) => {
                    h849
                }
            };
            let a851 = v11;
            nat_eq(a850, a851)
        } else {
            false
        }
    } else {
        false
    } {
        let a855 = match v7.clone() {
            Adt1::C0(_, h852, _, _, _, _, _, _, _, _, _) => {
                h852
            }
        };
        let a856 = {
            let a853 = v8;
            let a854 = v9;
            nat_mul(a853, a854)?
        };
        if if if nat_eq(a855, a856) {
            let a866 = match v7.clone() {
                Adt1::C0(_, _, h857, _, _, _, _, _, _, _, _) => {
                    h857
                }
            };
            let a867 = {
                let a864 = {
                    let a858 = v10;
                    let a859 = v11;
                    nat_mul(a858, a859)?
                };
                let a865 = {
                    let a862 = {
                        let a860 = v0;
                        let a861 = v8;
                        nat_mul(a860, a861)?
                    };
                    let a863 = v11;
                    nat_mul(a862, a863)?
                };
                nat_add(a864, a865)?
            };
            nat_eq(a866, a867)
        } else {
            false
        } {
            let a869 = match v7.clone() {
                Adt1::C0(_, _, _, h868, _, _, _, _, _, _, _) => {
                    h868
                }
            };
            let a870 = 1u64;
            nat_eq(a869, a870)
        } else {
            false
        } {
            let a872 = match v7.clone() {
                Adt1::C0(_, _, _, _, h871, _, _, _, _, _, _) => {
                    h871
                }
            };
            let a873 = 1u64;
            nat_eq(a872, a873)
        } else {
            false
        }
    } else {
        false
    } {
        let a875 = match v7.clone() {
            Adt1::C0(h874, _, _, _, _, _, _, _, _, _, _) => {
                h874
            }
        };
        let a876 = 0u64;
        nat_eq(a875, a876)
    } else {
        false
    } {
        f20(v4.clone(), 4u64)?
    } else {
        false
    } {
        f20(v5.clone(), 4u64)?
    } else {
        false
    } {
        f20(v6.clone(), 4u64)?
    } else {
        false
    } {
        f20(v7, 4u64)?
    } else {
        false
    } {
        let v12: u64 = {
            let a877 = v9;
            let a878 = v11;
            nat_mul(a877, a878)?
        };
        let v13: u64 = {
            let a879 = v8;
            let a880 = v8;
            nat_mul(a879, a880)?
        };
        let v14: u64 = {
            let a881 = v10;
            let a882 = 0u64;
            if if nat_eq(a881, a882) {
                let a883 = v0;
                let a884 = 1u64;
                nat_eq(a883, a884)
            } else {
                false
            } {
                1u64
            } else {
                let a885 = v0;
                let a886 = v10;
                if nat_lt(a885, a886) {
                    v0
                } else {
                    v10
                }
            }
        };
        let v15: u64 = {
            let a893 = v12;
            let a894 = {
                let a891 = {
                    let a887 = v10;
                    let a888 = v8;
                    nat_mul(a887, a888)?
                };
                let a892 = {
                    let a889 = v14;
                    let a890 = v13;
                    nat_mul(a889, a890)?
                };
                nat_add(a891, a892)?
            };
            nat_mul(a893, a894)?
        };
        Ok(Adt7::C12(Adt6::C0(v1, v2, v3, v4, v5, v6, v0, v12, v13, {
            let a897 = {
                let a895 = v13;
                let a896 = v8;
                nat_add(a895, a896)?
            };
            let a898 = v8;
            nat_add(a897, a898)?
        }, v15, false)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f26(v0: bool, v1: Adt1, v2: Adt1, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Adt1) -> R<Adt7> {
    let v8: Adt1 = {
        let m899 = v3.clone();
        match m899 {
            None => {
                v1.clone()
            }
            Some(v7) => {
                v7
            }
        }
    };
    let v10: Adt1 = {
        let m900 = v4.clone();
        match m900 {
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
        let m901 = v5;
        match m901 {
            None => {
                let m902 = v3;
                match m902 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m903 = v4;
                        match m903 {
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
        let a905 = match v1.clone() {
            Adt1::C0(h904, _, _, _, _, _, _, _, _, _, _) => {
                h904
            }
        };
        let a906 = 0u64;
        if nat_eq(a905, a906) {
            let a908 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h907, _, _, _, _, _) => {
                    h907
                }
            };
            let a909 = 4u64;
            nat_eq(a908, a909)
        } else {
            false
        }
    } else {
        false
    } {
        let a911 = match v8.clone() {
            Adt1::C0(h910, _, _, _, _, _, _, _, _, _, _) => {
                h910
            }
        };
        let a912 = 0u64;
        if nat_eq(a911, a912) {
            let a914 = match v8.clone() {
                Adt1::C0(_, _, _, _, _, h913, _, _, _, _, _) => {
                    h913
                }
            };
            let a915 = 4u64;
            nat_eq(a914, a915)
        } else {
            false
        }
    } else {
        false
    } {
        let a917 = match v10.clone() {
            Adt1::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a918 = 1u64;
        if nat_eq(a917, a918) {
            let a920 = match v10.clone() {
                Adt1::C0(_, _, _, _, _, h919, _, _, _, _, _) => {
                    h919
                }
            };
            let a921 = 2u64;
            nat_eq(a920, a921)
        } else {
            false
        }
    } else {
        false
    } {
        f29(v6.clone())?
    } else {
        false
    } {
        f42(match v2.clone() {
            Adt1::C0(h922, _, _, _, _, _, _, _, _, _, _) => {
                h922
            }
        }, match v2.clone() {
            Adt1::C0(_, h923, _, _, _, _, _, _, _, _, _) => {
                h923
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h924, _, _, _, _, _) => {
                h924
            }
        })
    } else {
        false
    } {
        let a926 = 0u64;
        let a927 = match v1.clone() {
            Adt1::C0(_, h925, _, _, _, _, _, _, _, _, _) => {
                h925
            }
        };
        nat_lt(a926, a927)
    } else {
        false
    } {
        let a929 = 0u64;
        let a930 = match v1.clone() {
            Adt1::C0(_, _, h928, _, _, _, _, _, _, _, _) => {
                h928
            }
        };
        nat_lt(a929, a930)
    } else {
        false
    } {
        let a932 = 0u64;
        let a933 = match v1.clone() {
            Adt1::C0(_, _, _, h931, _, _, _, _, _, _, _) => {
                h931
            }
        };
        nat_lt(a932, a933)
    } else {
        false
    } {
        let a935 = 0u64;
        let a936 = match v1.clone() {
            Adt1::C0(_, _, _, _, h934, _, _, _, _, _, _) => {
                h934
            }
        };
        nat_lt(a935, a936)
    } else {
        false
    } {
        let a938 = 0u64;
        let a939 = match v2.clone() {
            Adt1::C0(_, _, _, h937, _, _, _, _, _, _, _) => {
                h937
            }
        };
        nat_lt(a938, a939)
    } else {
        false
    } {
        let a942 = match v2.clone() {
            Adt1::C0(_, h940, _, _, _, _, _, _, _, _, _) => {
                h940
            }
        };
        let a943 = match v1.clone() {
            Adt1::C0(_, h941, _, _, _, _, _, _, _, _, _) => {
                h941
            }
        };
        nat_eq(a942, a943)
    } else {
        false
    } {
        let a945 = match v2.clone() {
            Adt1::C0(_, _, h944, _, _, _, _, _, _, _, _) => {
                h944
            }
        };
        let a946 = 1u64;
        nat_eq(a945, a946)
    } else {
        false
    } {
        let a949 = match v2.clone() {
            Adt1::C0(_, _, _, _, h947, _, _, _, _, _, _) => {
                h947
            }
        };
        let a950 = match v1.clone() {
            Adt1::C0(_, _, _, _, h948, _, _, _, _, _, _) => {
                h948
            }
        };
        nat_eq(a949, a950)
    } else {
        false
    } {
        let a953 = match v8.clone() {
            Adt1::C0(_, h951, _, _, _, _, _, _, _, _, _) => {
                h951
            }
        };
        let a954 = match v1.clone() {
            Adt1::C0(_, _, h952, _, _, _, _, _, _, _, _) => {
                h952
            }
        };
        nat_eq(a953, a954)
    } else {
        false
    } {
        let a957 = match v8.clone() {
            Adt1::C0(_, _, h955, _, _, _, _, _, _, _, _) => {
                h955
            }
        };
        let a958 = match v1.clone() {
            Adt1::C0(_, _, _, h956, _, _, _, _, _, _, _) => {
                h956
            }
        };
        nat_eq(a957, a958)
    } else {
        false
    } {
        let a960 = match v8.clone() {
            Adt1::C0(_, _, _, h959, _, _, _, _, _, _, _) => {
                h959
            }
        };
        let a961 = 1u64;
        nat_eq(a960, a961)
    } else {
        false
    } {
        let a964 = match v8.clone() {
            Adt1::C0(_, _, _, _, h962, _, _, _, _, _, _) => {
                h962
            }
        };
        let a965 = match v1.clone() {
            Adt1::C0(_, _, _, _, h963, _, _, _, _, _, _) => {
                h963
            }
        };
        nat_eq(a964, a965)
    } else {
        false
    } {
        let a968 = match v10.clone() {
            Adt1::C0(_, h966, _, _, _, _, _, _, _, _, _) => {
                h966
            }
        };
        let a969 = match v2.clone() {
            Adt1::C0(_, _, _, h967, _, _, _, _, _, _, _) => {
                h967
            }
        };
        nat_eq(a968, a969)
    } else {
        false
    } {
        let a972 = match v10.clone() {
            Adt1::C0(_, _, h970, _, _, _, _, _, _, _, _) => {
                h970
            }
        };
        let a973 = match v1.clone() {
            Adt1::C0(_, _, _, h971, _, _, _, _, _, _, _) => {
                h971
            }
        };
        nat_eq(a972, a973)
    } else {
        false
    } {
        let a975 = match v10.clone() {
            Adt1::C0(_, _, _, h974, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a976 = 1u64;
        nat_eq(a975, a976)
    } else {
        false
    } {
        f17(match v1.clone() {
            Adt1::C0(_, _, _, _, h977, _, _, _, _, _, _) => {
                h977
            }
        }, match v10.clone() {
            Adt1::C0(_, _, _, _, h978, _, _, _, _, _, _) => {
                h978
            }
        })
    } else {
        false
    } {
        let a981 = match v6.clone() {
            Adt1::C0(_, h979, _, _, _, _, _, _, _, _, _) => {
                h979
            }
        };
        let a982 = match v2.clone() {
            Adt1::C0(_, _, _, h980, _, _, _, _, _, _, _) => {
                h980
            }
        };
        nat_eq(a981, a982)
    } else {
        false
    } {
        let a985 = match v6.clone() {
            Adt1::C0(_, _, h983, _, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a986 = match v1.clone() {
            Adt1::C0(_, _, _, h984, _, _, _, _, _, _, _) => {
                h984
            }
        };
        nat_eq(a985, a986)
    } else {
        false
    } {
        let a988 = match v6.clone() {
            Adt1::C0(_, _, _, h987, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a989 = 1u64;
        nat_eq(a988, a989)
    } else {
        false
    } {
        let a992 = match v6.clone() {
            Adt1::C0(_, _, _, _, h990, _, _, _, _, _, _) => {
                h990
            }
        };
        let a993 = match v1.clone() {
            Adt1::C0(_, _, _, _, h991, _, _, _, _, _, _) => {
                h991
            }
        };
        nat_eq(a992, a993)
    } else {
        false
    } {
        f23(v1.clone(), v6.clone())?
    } else {
        false
    } {
        f23(v2.clone(), v6.clone())?
    } else {
        false
    } {
        f23(v8.clone(), v6.clone())?
    } else {
        false
    } {
        f23(v10.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt7::C10(Adt5::C0(v1, v2, v8, v10, v6)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f27(v0: Option<Adt1>, v1: Option<Adt1>, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Option<u64>> {
    let m994 = v0;
    match m994 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h995, _, _, _, _, _, _, _, _) => {
                    h995
                }
            }))
        }
        Some(v6) => {
            let m996 = v1;
            match m996 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a998 = match v6.clone() {
                        Adt1::C0(h997, _, _, _, _, _, _, _, _, _, _) => {
                            h997
                        }
                    };
                    let a999 = 26u64;
                    if if if if if if if if if nat_eq(a998, a999) {
                        f20(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a1002 = match v6.clone() {
                            Adt1::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
                                h1000
                            }
                        };
                        let a1003 = match v7 {
                            Adt1::C0(_, h1001, _, _, _, _, _, _, _, _, _) => {
                                h1001
                            }
                        };
                        nat_eq(a1002, a1003)
                    } else {
                        false
                    } {
                        let a1006 = match v6.clone() {
                            Adt1::C0(_, _, h1004, _, _, _, _, _, _, _, _) => {
                                h1004
                            }
                        };
                        let a1007 = match v2 {
                            Adt1::C0(_, _, _, _, h1005, _, _, _, _, _, _) => {
                                h1005
                            }
                        };
                        nat_eq(a1006, a1007)
                    } else {
                        false
                    } {
                        let a1009 = match v6.clone() {
                            Adt1::C0(_, _, _, h1008, _, _, _, _, _, _, _) => {
                                h1008
                            }
                        };
                        let a1010 = 1u64;
                        nat_eq(a1009, a1010)
                    } else {
                        false
                    } {
                        let a1012 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h1011, _, _, _, _, _, _) => {
                                h1011
                            }
                        };
                        let a1013 = 1u64;
                        nat_eq(a1012, a1013)
                    } else {
                        false
                    } {
                        let a1015 = match v3 {
                            Adt1::C0(_, _, _, _, h1014, _, _, _, _, _, _) => {
                                h1014
                            }
                        };
                        let a1016 = 1u64;
                        nat_eq(a1015, a1016)
                    } else {
                        false
                    } {
                        let a1018 = match v4 {
                            Adt1::C0(_, _, _, _, h1017, _, _, _, _, _, _) => {
                                h1017
                            }
                        };
                        let a1019 = 1u64;
                        nat_eq(a1018, a1019)
                    } else {
                        false
                    } {
                        f23(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h1020, _, _, _, _, _, _, _, _, _) => {
                                h1020
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

pub fn f28(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a1021 = v0;
    let a1022 = 0u64;
    if if nat_eq(a1021, a1022) {
        true
    } else {
        let a1023 = 2147483648u64;
        let a1024 = v0;
        if nat_le(a1023, a1024) {
            let a1025 = v0;
            let a1026 = 4286578688u64;
            nat_lt(a1025, a1026)
        } else {
            false
        }
    } {
        true
    } else {
        let a1027 = v0;
        let a1028 = 2139095040u64;
        if if nat_lt(a1027, a1028) {
            let a1029 = v1;
            let a1030 = 4294967295u64;
            nat_le(a1029, a1030)
        } else {
            false
        } {
            let m1031 = v2;
            match m1031 {
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

pub fn f29(v0: Adt1) -> R<bool> {
    let a1033 = match v0.clone() {
        Adt1::C0(h1032, _, _, _, _, _, _, _, _, _, _) => {
            h1032
        }
    };
    let a1034 = 0u64;
    if if if if nat_eq(a1033, a1034) {
        let a1036 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1035, _, _, _, _, _) => {
                h1035
            }
        };
        let a1037 = 4u64;
        nat_eq(a1036, a1037)
    } else {
        false
    } {
        let a1043 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1038, _, _, _, _) => {
                h1038
            }
        };
        let a1044 = {
            let a1041 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1039, _, _, _, _, _) => {
                    h1039
                }
            };
            let a1042 = match v0.clone() {
                Adt1::C0(_, h1040, _, _, _, _, _, _, _, _, _) => {
                    h1040
                }
            };
            nat_mul(a1041, a1042)?
        };
        nat_eq(a1043, a1044)
    } else {
        false
    } {
        let a1050 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1045, _, _, _) => {
                h1045
            }
        };
        let a1051 = {
            let a1048 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1046, _, _, _, _) => {
                    h1046
                }
            };
            let a1049 = match v0.clone() {
                Adt1::C0(_, _, h1047, _, _, _, _, _, _, _, _) => {
                    h1047
                }
            };
            nat_mul(a1048, a1049)?
        };
        nat_eq(a1050, a1051)
    } else {
        false
    } {
        let a1057 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1052, _, _) => {
                h1052
            }
        };
        let a1058 = {
            let a1055 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1053, _, _, _) => {
                    h1053
                }
            };
            let a1056 = match v0 {
                Adt1::C0(_, _, _, h1054, _, _, _, _, _, _, _) => {
                    h1054
                }
            };
            nat_mul(a1055, a1056)?
        };
        Ok(nat_eq(a1057, a1058))
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f42(match v0.clone() {
        Adt1::C0(h1059, _, _, _, _, _, _, _, _, _, _) => {
            h1059
        }
    }, match v0.clone() {
        Adt1::C0(_, h1060, _, _, _, _, _, _, _, _, _) => {
            h1060
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1061, _, _, _, _, _) => {
            h1061
        }
    }) {
        f17(match v1.clone() {
            Adt1::C0(_, _, _, h1062, _, _, _, _, _, _, _) => {
                h1062
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h1063, _, _, _, _, _, _, _) => {
                h1063
            }
        })
    } else {
        false
    } {
        f17(match v1 {
            Adt1::C0(_, _, _, _, h1064, _, _, _, _, _, _) => {
                h1064
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h1065, _, _, _, _, _, _) => {
                h1065
            }
        })
    } else {
        false
    } {
        f23(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f31(v0: Option<Adt1>, v1: Adt1, v2: u64, v3: Adt1) -> R<bool> {
    let m1066 = v0;
    match m1066 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a1068 = match v4.clone() {
                Adt1::C0(h1067, _, _, _, _, _, _, _, _, _, _) => {
                    h1067
                }
            };
            let a1069 = 1u64;
            if if if if if if nat_eq(a1068, a1069) {
                let a1071 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h1070, _, _, _, _, _) => {
                        h1070
                    }
                };
                let a1072 = 2u64;
                nat_eq(a1071, a1072)
            } else {
                false
            } {
                let a1074 = v2;
                let a1075 = match v4.clone() {
                    Adt1::C0(_, h1073, _, _, _, _, _, _, _, _, _) => {
                        h1073
                    }
                };
                nat_le(a1074, a1075)
            } else {
                false
            } {
                let a1078 = match v1.clone() {
                    Adt1::C0(_, _, h1076, _, _, _, _, _, _, _, _) => {
                        h1076
                    }
                };
                let a1079 = match v4.clone() {
                    Adt1::C0(_, _, h1077, _, _, _, _, _, _, _, _) => {
                        h1077
                    }
                };
                nat_le(a1078, a1079)
            } else {
                false
            } {
                f17(match v1.clone() {
                    Adt1::C0(_, _, _, h1080, _, _, _, _, _, _, _) => {
                        h1080
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h1081, _, _, _, _, _, _, _) => {
                        h1081
                    }
                })
            } else {
                false
            } {
                f17(match v1 {
                    Adt1::C0(_, _, _, _, h1082, _, _, _, _, _, _) => {
                        h1082
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h1083, _, _, _, _, _, _) => {
                        h1083
                    }
                })
            } else {
                false
            } {
                f23(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f32(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1084 = v0;
    match m1084 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1086 = match v3.clone() {
                Adt1::C0(h1085, _, _, _, _, _, _, _, _, _, _) => {
                    h1085
                }
            };
            let a1087 = 0u64;
            if if if if if if nat_eq(a1086, a1087) {
                let a1089 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h1088, _, _, _, _, _) => {
                        h1088
                    }
                };
                let a1090 = 4u64;
                nat_eq(a1089, a1090)
            } else {
                false
            } {
                let a1093 = match v3.clone() {
                    Adt1::C0(_, h1091, _, _, _, _, _, _, _, _, _) => {
                        h1091
                    }
                };
                let a1094 = match v1 {
                    Adt1::C0(_, _, _, h1092, _, _, _, _, _, _, _) => {
                        h1092
                    }
                };
                nat_eq(a1093, a1094)
            } else {
                false
            } {
                let a1096 = match v3.clone() {
                    Adt1::C0(_, _, h1095, _, _, _, _, _, _, _, _) => {
                        h1095
                    }
                };
                let a1097 = 1u64;
                nat_eq(a1096, a1097)
            } else {
                false
            } {
                let a1099 = match v3.clone() {
                    Adt1::C0(_, _, _, h1098, _, _, _, _, _, _, _) => {
                        h1098
                    }
                };
                let a1100 = 1u64;
                nat_eq(a1099, a1100)
            } else {
                false
            } {
                let a1102 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1101, _, _, _, _, _, _) => {
                        h1101
                    }
                };
                let a1103 = 1u64;
                nat_eq(a1102, a1103)
            } else {
                false
            } {
                f23(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f33(v0: u64) -> Option<u64> {
    let a1104 = 0u64;
    let a1105 = v0;
    if if nat_lt(a1104, a1105) {
        let a1106 = v0;
        let a1107 = 2139095040u64;
        nat_lt(a1106, a1107)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f34(v0: u64) -> Option<u64> {
    let a1108 = v0;
    let a1109 = 0u64;
    if if nat_eq(a1108, a1109) {
        true
    } else {
        let a1110 = v0;
        let a1111 = 2147483648u64;
        nat_eq(a1110, a1111)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f35(v0: Adt1, v1: Adt1) -> bool {
    let a1114 = match v0.clone() {
        Adt1::C0(_, _, h1112, _, _, _, _, _, _, _, _) => {
            h1112
        }
    };
    let a1115 = match v1.clone() {
        Adt1::C0(_, _, h1113, _, _, _, _, _, _, _, _) => {
            h1113
        }
    };
    if if nat_eq(a1114, a1115) {
        let a1118 = match v0.clone() {
            Adt1::C0(_, _, _, h1116, _, _, _, _, _, _, _) => {
                h1116
            }
        };
        let a1119 = match v1.clone() {
            Adt1::C0(_, _, _, h1117, _, _, _, _, _, _, _) => {
                h1117
            }
        };
        nat_eq(a1118, a1119)
    } else {
        false
    } {
        let a1122 = match v0 {
            Adt1::C0(_, _, _, _, h1120, _, _, _, _, _, _) => {
                h1120
            }
        };
        let a1123 = match v1 {
            Adt1::C0(_, _, _, _, h1121, _, _, _, _, _, _) => {
                h1121
            }
        };
        nat_eq(a1122, a1123)
    } else {
        false
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f43(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f43(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f43(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f43(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f43(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f43(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f37(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f22(v0.clone(), v1.clone()) {
        let a1126 = match v0.clone() {
            Adt1::C0(h1124, _, _, _, _, _, _, _, _, _, _) => {
                h1124
            }
        };
        let a1127 = match v1.clone() {
            Adt1::C0(h1125, _, _, _, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        nat_eq(a1126, a1127)
    } else {
        false
    } {
        let a1130 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1128) => {
                h1128
            }
        };
        let a1131 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1129) => {
                h1129
            }
        };
        nat_eq(a1130, a1131)
    } else {
        false
    } {
        let a1134 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1132, _, _, _, _, _) => {
                h1132
            }
        };
        let a1135 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1133, _, _, _, _, _) => {
                h1133
            }
        };
        nat_eq(a1134, a1135)
    } else {
        false
    } {
        let a1138 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1136, _, _, _, _) => {
                h1136
            }
        };
        let a1139 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1137, _, _, _, _) => {
                h1137
            }
        };
        nat_eq(a1138, a1139)
    } else {
        false
    } {
        let a1142 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1140, _, _, _) => {
                h1140
            }
        };
        let a1143 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1141, _, _, _) => {
                h1141
            }
        };
        nat_eq(a1142, a1143)
    } else {
        false
    } {
        let a1146 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1144, _, _) => {
                h1144
            }
        };
        let a1147 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1145, _, _) => {
                h1145
            }
        };
        nat_eq(a1146, a1147)
    } else {
        false
    }
}

pub fn f38(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1148 = v0;
    match m1148 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1150 = match v3.clone() {
                Adt1::C0(h1149, _, _, _, _, _, _, _, _, _, _) => {
                    h1149
                }
            };
            let a1151 = 0u64;
            if if if if if if if nat_eq(a1150, a1151) {
                true
            } else {
                let a1153 = match v3.clone() {
                    Adt1::C0(h1152, _, _, _, _, _, _, _, _, _, _) => {
                        h1152
                    }
                };
                let a1154 = 1u64;
                nat_eq(a1153, a1154)
            } {
                f20(v3.clone(), {
                    let a1156 = match v3.clone() {
                        Adt1::C0(h1155, _, _, _, _, _, _, _, _, _, _) => {
                            h1155
                        }
                    };
                    let a1157 = 0u64;
                    if nat_eq(a1156, a1157) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a1160 = match v3.clone() {
                    Adt1::C0(_, h1158, _, _, _, _, _, _, _, _, _) => {
                        h1158
                    }
                };
                let a1161 = match v1.clone() {
                    Adt1::C0(_, h1159, _, _, _, _, _, _, _, _, _) => {
                        h1159
                    }
                };
                nat_eq(a1160, a1161)
            } else {
                false
            } {
                let a1164 = match v1.clone() {
                    Adt1::C0(_, _, h1162, _, _, _, _, _, _, _, _) => {
                        h1162
                    }
                };
                let a1165 = match v3.clone() {
                    Adt1::C0(_, _, h1163, _, _, _, _, _, _, _, _) => {
                        h1163
                    }
                };
                nat_le(a1164, a1165)
            } else {
                false
            } {
                f17(match v1.clone() {
                    Adt1::C0(_, _, _, h1166, _, _, _, _, _, _, _) => {
                        h1166
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h1167, _, _, _, _, _, _, _) => {
                        h1167
                    }
                })
            } else {
                false
            } {
                f17(match v1 {
                    Adt1::C0(_, _, _, _, h1168, _, _, _, _, _, _) => {
                        h1168
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h1169, _, _, _, _, _, _) => {
                        h1169
                    }
                })
            } else {
                false
            } {
                f23(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f39(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1170 = v0;
    match m1170 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1172 = match v3.clone() {
                Adt1::C0(h1171, _, _, _, _, _, _, _, _, _, _) => {
                    h1171
                }
            };
            let a1173 = 0u64;
            if if if if if if nat_eq(a1172, a1173) {
                let a1175 = match v3.clone() {
                    Adt1::C0(_, h1174, _, _, _, _, _, _, _, _, _) => {
                        h1174
                    }
                };
                let a1176 = 1u64;
                if nat_eq(a1175, a1176) {
                    true
                } else {
                    let a1178 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h1177, _, _, _, _, _) => {
                            h1177
                        }
                    };
                    let a1179 = 4u64;
                    nat_eq(a1178, a1179)
                }
            } else {
                false
            } {
                let a1182 = match v3.clone() {
                    Adt1::C0(_, h1180, _, _, _, _, _, _, _, _, _) => {
                        h1180
                    }
                };
                let a1183 = match v1 {
                    Adt1::C0(_, _, _, h1181, _, _, _, _, _, _, _) => {
                        h1181
                    }
                };
                nat_eq(a1182, a1183)
            } else {
                false
            } {
                let a1185 = match v3.clone() {
                    Adt1::C0(_, _, h1184, _, _, _, _, _, _, _, _) => {
                        h1184
                    }
                };
                let a1186 = 1u64;
                nat_eq(a1185, a1186)
            } else {
                false
            } {
                let a1188 = match v3.clone() {
                    Adt1::C0(_, _, _, h1187, _, _, _, _, _, _, _) => {
                        h1187
                    }
                };
                let a1189 = 1u64;
                nat_eq(a1188, a1189)
            } else {
                false
            } {
                let a1191 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1190, _, _, _, _, _, _) => {
                        h1190
                    }
                };
                let a1192 = 1u64;
                nat_eq(a1191, a1192)
            } else {
                false
            } {
                f23(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f40(v0: u64) -> u64 {
    if f44(v0) {
        15u64
    } else {
        if if f45(v0) {
            true
        } else {
            f46(v0)
        } {
            8u64
        } else {
            if f47(v0) {
                9u64
            } else {
                f48(v0)
            }
        }
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    let a1195 = match v0.clone() {
        Adt1::C0(_, h1193, _, _, _, _, _, _, _, _, _) => {
            h1193
        }
    };
    let a1196 = match v1.clone() {
        Adt1::C0(_, h1194, _, _, _, _, _, _, _, _, _) => {
            h1194
        }
    };
    if if if nat_eq(a1195, a1196) {
        let a1199 = match v0.clone() {
            Adt1::C0(_, _, _, h1197, _, _, _, _, _, _, _) => {
                h1197
            }
        };
        let a1200 = match v1.clone() {
            Adt1::C0(_, _, _, h1198, _, _, _, _, _, _, _) => {
                h1198
            }
        };
        nat_eq(a1199, a1200)
    } else {
        false
    } {
        let a1202 = 0u64;
        let a1203 = match v0.clone() {
            Adt1::C0(_, _, h1201, _, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        nat_lt(a1202, a1203)
    } else {
        false
    } {
        f17(match v1 {
            Adt1::C0(_, _, _, _, h1204, _, _, _, _, _, _) => {
                h1204
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, h1205, _, _, _, _, _, _) => {
                h1205
            }
        })
    } else {
        false
    }
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> bool {
    let m1206 = f49(v0);
    match m1206 {
        Adt8::C0 => {
            false
        }
        Adt8::C1(v3) => {
            let a1207 = v0;
            let a1208 = 0u64;
            if if if if if if nat_eq(a1207, a1208) {
                true
            } else {
                let a1209 = v0;
                let a1210 = 1u64;
                nat_eq(a1209, a1210)
            } {
                true
            } else {
                let a1211 = v0;
                let a1212 = 30u64;
                nat_eq(a1211, a1212)
            } {
                true
            } else {
                let a1213 = v0;
                let a1214 = 43u64;
                nat_eq(a1213, a1214)
            } {
                true
            } else {
                f50(v0)
            } {
                let a1216 = v2;
                let a1217 = match v3.clone() {
                    Adt9::C0(_, _, h1215) => {
                        h1215
                    }
                };
                nat_eq(a1216, a1217)
            } else {
                false
            } {
                f17(v1, match v3 {
                    Adt9::C0(_, h1218, _) => {
                        h1218
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1219 = v1;
    let a1220 = 1u64;
    if if if nat_le(a1219, a1220) {
        true
    } else {
        let a1221 = v0;
        let a1222 = v2;
        nat_le(a1221, a1222)
    } {
        let a1223 = v3;
        let a1224 = 1u64;
        if nat_le(a1223, a1224) {
            true
        } else {
            let a1231 = {
                let a1229 = v0;
                let a1230 = {
                    let a1227 = {
                        let a1225 = v1;
                        let a1226 = 1u64;
                        nat_sub(a1225, a1226)
                    };
                    let a1228 = v2;
                    nat_mul(a1227, a1228)?
                };
                nat_add(a1229, a1230)?
            };
            let a1232 = v4;
            nat_le(a1231, a1232)
        }
    } else {
        false
    } {
        let a1233 = v5;
        let a1234 = 1u64;
        if nat_le(a1233, a1234) {
            Ok(true)
        } else {
            let a1247 = {
                let a1245 = {
                    let a1239 = v0;
                    let a1240 = {
                        let a1237 = {
                            let a1235 = v1;
                            let a1236 = 1u64;
                            nat_sub(a1235, a1236)
                        };
                        let a1238 = v2;
                        nat_mul(a1237, a1238)?
                    };
                    nat_add(a1239, a1240)?
                };
                let a1246 = {
                    let a1243 = {
                        let a1241 = v3;
                        let a1242 = 1u64;
                        nat_sub(a1241, a1242)
                    };
                    let a1244 = v4;
                    nat_mul(a1243, a1244)?
                };
                nat_add(a1245, a1246)?
            };
            let a1248 = v6;
            Ok(nat_le(a1247, a1248))
        }
    } else {
        Ok(false)
    }
}

pub fn f44(v0: u64) -> bool {
    let a1249 = 10u64;
    let a1250 = v0;
    if if if if if nat_le(a1249, a1250) {
        let a1251 = v0;
        let a1252 = 14u64;
        nat_le(a1251, a1252)
    } else {
        false
    } {
        true
    } else {
        let a1253 = 16u64;
        let a1254 = v0;
        if nat_le(a1253, a1254) {
            let a1255 = v0;
            let a1256 = 19u64;
            nat_le(a1255, a1256)
        } else {
            false
        }
    } {
        true
    } else {
        let a1257 = 21u64;
        let a1258 = v0;
        if nat_le(a1257, a1258) {
            let a1259 = v0;
            let a1260 = 23u64;
            nat_le(a1259, a1260)
        } else {
            false
        }
    } {
        true
    } else {
        let a1261 = 34u64;
        let a1262 = v0;
        if nat_le(a1261, a1262) {
            let a1263 = v0;
            let a1264 = 35u64;
            nat_le(a1263, a1264)
        } else {
            false
        }
    } {
        true
    } else {
        let a1265 = v0;
        let a1266 = 29u64;
        nat_eq(a1265, a1266)
    }
}

pub fn f45(v0: u64) -> bool {
    let a1267 = v0;
    let a1268 = 2u64;
    if if if if nat_eq(a1267, a1268) {
        true
    } else {
        let a1269 = v0;
        let a1270 = 6u64;
        nat_eq(a1269, a1270)
    } {
        true
    } else {
        let a1271 = v0;
        let a1272 = 8u64;
        nat_eq(a1271, a1272)
    } {
        true
    } else {
        let a1273 = v0;
        let a1274 = 20u64;
        nat_eq(a1273, a1274)
    } {
        true
    } else {
        f51(v0)
    }
}

pub fn f46(v0: u64) -> bool {
    let a1275 = v0;
    let a1276 = 39u64;
    if nat_eq(a1275, a1276) {
        true
    } else {
        let a1277 = v0;
        let a1278 = 40u64;
        nat_eq(a1277, a1278)
    }
}

pub fn f47(v0: u64) -> bool {
    let a1279 = v0;
    let a1280 = 3u64;
    if nat_eq(a1279, a1280) {
        true
    } else {
        let a1281 = v0;
        let a1282 = 7u64;
        nat_eq(a1281, a1282)
    }
}

pub fn f48(v0: u64) -> u64 {
    let a1283 = v0;
    let a1284 = 30u64;
    if nat_eq(a1283, a1284) {
        30u64
    } else {
        0u64
    }
}

pub fn f49(v0: u64) -> Adt8 {
    let a1285 = v0;
    let a1286 = 0u64;
    if nat_eq(a1285, a1286) {
        Adt8::C1(Adt9::C0(0u64, 1u64, 4u64))
    } else {
        let a1287 = v0;
        let a1288 = 1u64;
        if nat_eq(a1287, a1288) {
            Adt8::C1(Adt9::C0(1u64, 1u64, 2u64))
        } else {
            let a1289 = v0;
            let a1290 = 2u64;
            if nat_eq(a1289, a1290) {
                Adt8::C1(Adt9::C0(2u64, 32u64, 18u64))
            } else {
                let a1291 = v0;
                let a1292 = 3u64;
                if nat_eq(a1291, a1292) {
                    Adt8::C1(Adt9::C0(3u64, 32u64, 20u64))
                } else {
                    let a1293 = v0;
                    let a1294 = 6u64;
                    if nat_eq(a1293, a1294) {
                        Adt8::C1(Adt9::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1295 = v0;
                        let a1296 = 7u64;
                        if nat_eq(a1295, a1296) {
                            Adt8::C1(Adt9::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1297 = v0;
                            let a1298 = 8u64;
                            if nat_eq(a1297, a1298) {
                                Adt8::C1(Adt9::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1299 = v0;
                                let a1300 = 9u64;
                                if nat_eq(a1299, a1300) {
                                    Adt8::C1(Adt9::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1301 = v0;
                                    let a1302 = 10u64;
                                    if nat_eq(a1301, a1302) {
                                        Adt8::C1(Adt9::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1303 = v0;
                                        let a1304 = 11u64;
                                        if nat_eq(a1303, a1304) {
                                            Adt8::C1(Adt9::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1305 = v0;
                                            let a1306 = 12u64;
                                            if nat_eq(a1305, a1306) {
                                                Adt8::C1(Adt9::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1307 = v0;
                                                let a1308 = 13u64;
                                                if nat_eq(a1307, a1308) {
                                                    Adt8::C1(Adt9::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1309 = v0;
                                                    let a1310 = 14u64;
                                                    if nat_eq(a1309, a1310) {
                                                        Adt8::C1(Adt9::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1311 = v0;
                                                        let a1312 = 15u64;
                                                        if nat_eq(a1311, a1312) {
                                                            Adt8::C1(Adt9::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1313 = v0;
                                                            let a1314 = 16u64;
                                                            if nat_eq(a1313, a1314) {
                                                                Adt8::C1(Adt9::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1315 = v0;
                                                                let a1316 = 17u64;
                                                                if nat_eq(a1315, a1316) {
                                                                    Adt8::C1(Adt9::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1317 = v0;
                                                                    let a1318 = 18u64;
                                                                    if nat_eq(a1317, a1318) {
                                                                        Adt8::C1(Adt9::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1319 = v0;
                                                                        let a1320 = 19u64;
                                                                        if nat_eq(a1319, a1320) {
                                                                            Adt8::C1(Adt9::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1321 = v0;
                                                                            let a1322 = 20u64;
                                                                            if nat_eq(a1321, a1322) {
                                                                                Adt8::C1(Adt9::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1323 = v0;
                                                                                let a1324 = 21u64;
                                                                                if nat_eq(a1323, a1324) {
                                                                                    Adt8::C1(Adt9::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1325 = v0;
                                                                                    let a1326 = 22u64;
                                                                                    if nat_eq(a1325, a1326) {
                                                                                        Adt8::C1(Adt9::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1327 = v0;
                                                                                        let a1328 = 23u64;
                                                                                        if nat_eq(a1327, a1328) {
                                                                                            Adt8::C1(Adt9::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1329 = v0;
                                                                                            let a1330 = 24u64;
                                                                                            if nat_eq(a1329, a1330) {
                                                                                                Adt8::C1(Adt9::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1331 = v0;
                                                                                                let a1332 = 25u64;
                                                                                                if nat_eq(a1331, a1332) {
                                                                                                    Adt8::C1(Adt9::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1333 = v0;
                                                                                                    let a1334 = 26u64;
                                                                                                    if nat_eq(a1333, a1334) {
                                                                                                        Adt8::C1(Adt9::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1335 = v0;
                                                                                                        let a1336 = 27u64;
                                                                                                        if nat_eq(a1335, a1336) {
                                                                                                            Adt8::C1(Adt9::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1337 = v0;
                                                                                                            let a1338 = 28u64;
                                                                                                            if nat_eq(a1337, a1338) {
                                                                                                                Adt8::C1(Adt9::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1339 = v0;
                                                                                                                let a1340 = 29u64;
                                                                                                                if nat_eq(a1339, a1340) {
                                                                                                                    Adt8::C1(Adt9::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1341 = v0;
                                                                                                                    let a1342 = 30u64;
                                                                                                                    if nat_eq(a1341, a1342) {
                                                                                                                        Adt8::C1(Adt9::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1343 = v0;
                                                                                                                        let a1344 = 34u64;
                                                                                                                        if nat_eq(a1343, a1344) {
                                                                                                                            Adt8::C1(Adt9::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1345 = v0;
                                                                                                                            let a1346 = 35u64;
                                                                                                                            if nat_eq(a1345, a1346) {
                                                                                                                                Adt8::C1(Adt9::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1347 = v0;
                                                                                                                                let a1348 = 39u64;
                                                                                                                                if nat_eq(a1347, a1348) {
                                                                                                                                    Adt8::C1(Adt9::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1349 = v0;
                                                                                                                                    let a1350 = 40u64;
                                                                                                                                    if nat_eq(a1349, a1350) {
                                                                                                                                        Adt8::C1(Adt9::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1351 = v0;
                                                                                                                                        let a1352 = 41u64;
                                                                                                                                        if nat_eq(a1351, a1352) {
                                                                                                                                            Adt8::C1(Adt9::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1353 = v0;
                                                                                                                                            let a1354 = 42u64;
                                                                                                                                            if nat_eq(a1353, a1354) {
                                                                                                                                                Adt8::C1(Adt9::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1355 = v0;
                                                                                                                                                let a1356 = 43u64;
                                                                                                                                                if nat_eq(a1355, a1356) {
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

pub fn f50(v0: u64) -> bool {
    if if f52(v0) {
        true
    } else {
        f46(v0)
    } {
        true
    } else {
        f44(v0)
    }
}

pub fn f51(v0: u64) -> bool {
    let a1357 = v0;
    let a1358 = 41u64;
    if nat_eq(a1357, a1358) {
        true
    } else {
        let a1359 = v0;
        let a1360 = 42u64;
        nat_eq(a1359, a1360)
    }
}

pub fn f52(v0: u64) -> bool {
    if f45(v0) {
        true
    } else {
        f47(v0)
    }
}
