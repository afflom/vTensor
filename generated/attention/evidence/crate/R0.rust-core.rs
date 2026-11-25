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
    C15(u64),
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
    C15(u64, u64, u64, u64),
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

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt7> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70) => {
            f10(v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, None::<Adt1>, v70)
        }
        Adt0::C1(v99, v100, v101, v102) => {
            f16(v99, v100, v101, v102)
        }
        Adt0::C2(v87, v88, v89, v90, v91, v92, v93, v94) => {
            f14(v87, v88, v89, v90, v91, v92, v93, v94)
        }
        Adt0::C3(v95, v96, v97, v98) => {
            f15(v95, v96, v97, v98)
        }
        Adt0::C4(v71, v72, v73, v74, v75, v76, v77, v78) => {
            f11(v71, v72, v73, v74, v75, v76, v77, v78)
        }
        Adt0::C5(v79, v80, v81, v82) => {
            f12(v79, v80, v81, v82)
        }
        Adt0::C6(v83, v84, v85, v86) => {
            f13(v83, v84, v85, v86)
        }
        Adt0::C7(v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59) => {
            f9(v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59)
        }
        Adt0::C8(v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43) => {
            let a11 = v31;
            let a12 = 83u64;
            if nat_eq(a11, a12) {
                f8(v32, v33, v37, v38, v39, v40, v41, v42, v43)
            } else {
                let m13 = v42.clone();
                match m13 {
                    None => {
                        f9(v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v43)
                    }
                    Some(_) => {
                        let m14 = v37;
                        match m14 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v45) => {
                                let m15 = v38;
                                match m15 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v46) => {
                                        let m16 = v39;
                                        match m16 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v47) => {
                                                f10(v31, v32, v34, v35, v36, v45, v46, v47, v40, v41, v42, v43)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C9(v7, v8) => {
            let a2 = v7;
            let a3 = 2147483648u64;
            if nat_lt(a2, a3) {
                let a4 = v7;
                let a5 = v8;
                if nat_lt(a4, a5) {
                    Ok(Adt7::C9(v7))
                } else {
                    Ok(Adt7::C0)
                }
            } else {
                let a6 = v7;
                let a7 = 4294967296u64;
                if nat_lt(a6, a7) {
                    Ok(Adt7::C8)
                } else {
                    Ok(Adt7::C0)
                }
            }
        }
        Adt0::C10(v2, v3, v4, v5, v6) => {
            f2(v2, v3, v4, v5, v6)
        }
        Adt0::C11(v24, v25, v26) => {
            if match v24.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h9) => {
                    h9
                }
            } {
                Ok(Adt7::C0)
            } else {
                f5(v24, v25, v26)
            }
        }
        Adt0::C12(v27, v28, v29, v30) => {
            if match v27.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h10) => {
                    h10
                }
            } {
                f6(v27, v28, v29, v30)
            } else {
                f7(v27, v28, v29, v30)
            }
        }
        Adt0::C13(v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) => {
            f3(v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
        }
        Adt0::C14(v20, v21, v22, v23) => {
            if match v20.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h8) => {
                    h8
                }
            } {
                f4(v20, v21, v22, v23)
            } else {
                Ok(Adt7::C0)
            }
        }
        Adt0::C15(v1) => {
            Ok(f1(v1))
        }
    }
}

pub fn f1(v0: u64) -> Adt7 {
    let m17 = f17(v0);
    match m17 {
        Adt8::C0 => {
            Adt7::C0
        }
        Adt8::C1(_) => {
            if f18(v0) {
                Adt7::C0
            } else {
                let m18 = f19(0u64);
                match m18 {
                    Adt10::C0 => {
                        Adt7::C0
                    }
                    Adt10::C1(v2, v3, v4, v5) => {
                        Adt7::C15(v2, v3, v4, v5)
                    }
                }
            }
        }
    }
}

pub fn f2(v0: Adt5, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt7> {
    let a22 = v1;
    let a23 = {
        let m21 = match v0.clone() {
            Adt5::C0(h20, _, _, _, _) => {
                h20
            }
        };
        match m21 {
            Adt1::C0(_, _, _, h19, _, _, _, _, _, _, _) => {
                h19
            }
        }
    };
    if if if if nat_lt(a22, a23) {
        let a27 = v2;
        let a28 = {
            let m26 = match v0.clone() {
                Adt5::C0(h25, _, _, _, _) => {
                    h25
                }
            };
            match m26 {
                Adt1::C0(_, _, h24, _, _, _, _, _, _, _, _) => {
                    h24
                }
            }
        };
        nat_lt(a27, a28)
    } else {
        false
    } {
        let a32 = v3;
        let a33 = {
            let m31 = match v0.clone() {
                Adt5::C0(h30, _, _, _, _) => {
                    h30
                }
            };
            match m31 {
                Adt1::C0(_, _, _, _, h29, _, _, _, _, _, _) => {
                    h29
                }
            }
        };
        nat_lt(a32, a33)
    } else {
        false
    } {
        let a37 = v4;
        let a38 = {
            let m36 = match v0.clone() {
                Adt5::C0(_, h35, _, _, _) => {
                    h35
                }
            };
            match m36 {
                Adt1::C0(_, _, _, h34, _, _, _, _, _, _, _) => {
                    h34
                }
            }
        };
        nat_lt(a37, a38)
    } else {
        false
    } {
        Ok(Adt7::C11(f20(match v0.clone() {
            Adt5::C0(h39, _, _, _, _) => {
                h39
            }
        }, v2, v1, v3)?, f20(match v0.clone() {
            Adt5::C0(_, h40, _, _, _) => {
                h40
            }
        }, 0u64, v4, v3)?, f20(match v0.clone() {
            Adt5::C0(_, _, h41, _, _) => {
                h41
            }
        }, v1, 0u64, v3)?, f20(match v0.clone() {
            Adt5::C0(_, _, _, h42, _) => {
                h42
            }
        }, v1, 0u64, {
            let a46 = v3;
            let a47 = {
                let m45 = match v0.clone() {
                    Adt5::C0(_, _, _, h44, _) => {
                        h44
                    }
                };
                match m45 {
                    Adt1::C0(_, _, _, _, h43, _, _, _, _, _, _) => {
                        h43
                    }
                }
            };
            let a48 = 0u64;
            nat_rem(a46, a47, a48)
        })?, f20(match v0 {
            Adt5::C0(_, _, _, _, h49) => {
                h49
            }
        }, v1, 0u64, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f3(v0: u64, v1: bool, v2: u64, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1, v8: Adt1, v9: Adt1, v10: Adt1) -> R<Adt7> {
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, h50, _, _, _, _, _, _, _, _, _) => {
            h50
        }
    };
    let v12: u64 = match v4.clone() {
        Adt1::C0(_, h51, _, _, _, _, _, _, _, _, _) => {
            h51
        }
    };
    let v13: u64 = match v4.clone() {
        Adt1::C0(_, _, h52, _, _, _, _, _, _, _, _) => {
            h52
        }
    };
    let v14: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h53, _, _, _, _, _, _, _) => {
            h53
        }
    };
    let v15: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h54, _, _, _, _, _, _) => {
            h54
        }
    };
    let v16: u64 = {
        let a55 = v11;
        let a56 = v12;
        nat_mul(a55, a56)?
    };
    let v17: u64 = {
        let a57 = v13;
        let a58 = v15;
        nat_mul(a57, a58)?
    };
    let v18: u64 = {
        let a61 = {
            let a59 = v14;
            let a60 = v12;
            nat_mul(a59, a60)?
        };
        let a62 = v17;
        nat_mul(a61, a62)?
    };
    let a63 = v0;
    let a64 = 74u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a63, a64) {
        !v1
    } else {
        false
    } {
        let a65 = 0u64;
        let a66 = v2;
        nat_lt(a65, a66)
    } else {
        false
    } {
        let a67 = v2;
        let a68 = 2147483647u64;
        nat_le(a67, a68)
    } else {
        false
    } {
        let a69 = 0u64;
        let a70 = v11;
        nat_lt(a69, a70)
    } else {
        false
    } {
        let a71 = 0u64;
        let a72 = v12;
        nat_lt(a71, a72)
    } else {
        false
    } {
        let a73 = 0u64;
        let a74 = v13;
        nat_lt(a73, a74)
    } else {
        false
    } {
        let a75 = 0u64;
        let a76 = v15;
        nat_lt(a75, a76)
    } else {
        false
    } {
        let a78 = 0u64;
        let a79 = match v3.clone() {
            Adt1::C0(_, _, _, _, h77, _, _, _, _, _, _) => {
                h77
            }
        };
        nat_lt(a78, a79)
    } else {
        false
    } {
        f21(v13, match v7.clone() {
            Adt1::C0(_, _, h80, _, _, _, _, _, _, _, _) => {
                h80
            }
        })
    } else {
        false
    } {
        f22(v3.clone(), v10.clone())?
    } else {
        false
    } {
        f22(v4.clone(), v10.clone())?
    } else {
        false
    } {
        f22(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f22(v6.clone(), v10.clone())?
    } else {
        false
    } {
        f22(v7.clone(), v10.clone())?
    } else {
        false
    } {
        f22(v8.clone(), v10.clone())?
    } else {
        false
    } {
        f23(v4.clone())?
    } else {
        false
    } {
        f23(v7.clone())?
    } else {
        false
    } {
        f23(v8.clone())?
    } else {
        false
    } {
        f24(v3.clone(), 4u64)?
    } else {
        false
    } {
        f24(v5.clone(), 4u64)?
    } else {
        false
    } {
        f24(v6.clone(), 4u64)?
    } else {
        false
    } {
        f24(v10.clone(), 4u64)?
    } else {
        false
    } {
        let a82 = match v3.clone() {
            Adt1::C0(_, _, h81, _, _, _, _, _, _, _, _) => {
                h81
            }
        };
        let a83 = v12;
        nat_eq(a82, a83)
    } else {
        false
    } {
        let a85 = match v3.clone() {
            Adt1::C0(_, _, _, h84, _, _, _, _, _, _, _) => {
                h84
            }
        };
        let a86 = v13;
        nat_eq(a85, a86)
    } else {
        false
    } {
        f25(v5.clone(), v13, v14, v15, 1u64)
    } else {
        false
    } {
        let a88 = match v6.clone() {
            Adt1::C0(_, _, h87, _, _, _, _, _, _, _, _) => {
                h87
            }
        };
        let a89 = v13;
        nat_eq(a88, a89)
    } else {
        false
    } {
        let a91 = match v6.clone() {
            Adt1::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                h90
            }
        };
        let a92 = 1u64;
        nat_eq(a91, a92)
    } else {
        false
    } {
        let a94 = match v6.clone() {
            Adt1::C0(_, _, _, _, h93, _, _, _, _, _, _) => {
                h93
            }
        };
        let a95 = 1u64;
        nat_eq(a94, a95)
    } else {
        false
    } {
        let a97 = match v6.clone() {
            Adt1::C0(_, h96, _, _, _, _, _, _, _, _, _) => {
                h96
            }
        };
        let a98 = 1u64;
        if nat_eq(a97, a98) {
            true
        } else {
            let a100 = match v6.clone() {
                Adt1::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                    h99
                }
            };
            let a101 = v11;
            if nat_eq(a100, a101) {
                let a102 = v2;
                let a103 = 1u64;
                nat_eq(a102, a103)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a105 = match v7.clone() {
            Adt1::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                h104
            }
        };
        let a106 = v11;
        nat_eq(a105, a106)
    } else {
        false
    } {
        let a108 = match v7.clone() {
            Adt1::C0(_, _, _, h107, _, _, _, _, _, _, _) => {
                h107
            }
        };
        let a109 = v14;
        nat_eq(a108, a109)
    } else {
        false
    } {
        let a111 = match v7.clone() {
            Adt1::C0(_, _, _, _, h110, _, _, _, _, _, _) => {
                h110
            }
        };
        let a112 = v15;
        nat_eq(a111, a112)
    } else {
        false
    } {
        f26(v7.clone(), v8.clone())
    } else {
        false
    } {
        f25(v9.clone(), v15, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        let a114 = match v9.clone() {
            Adt1::C0(h113, _, _, _, _, _, _, _, _, _, _) => {
                h113
            }
        };
        let a115 = 26u64;
        nat_eq(a114, a115)
    } else {
        false
    } {
        let a117 = match v9.clone() {
            Adt1::C0(_, _, _, _, _, h116, _, _, _, _, _) => {
                h116
            }
        };
        let a118 = 4u64;
        nat_eq(a117, a118)
    } else {
        false
    } {
        f27(v9, v10.clone())?
    } else {
        false
    } {
        let a120 = match v10.clone() {
            Adt1::C0(h119, _, _, _, _, _, _, _, _, _, _) => {
                h119
            }
        };
        let a121 = 0u64;
        nat_eq(a120, a121)
    } else {
        false
    } {
        f25(v10, {
            let a126 = v18;
            let a127 = {
                let a124 = v2;
                let a125 = {
                    let a122 = v16;
                    let a123 = v17;
                    nat_mul(a122, a123)?
                };
                nat_mul(a124, a125)?
            };
            nat_add(a126, a127)?
        }, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        Ok(Adt7::C12(Adt6::C0(v7, v8, v4, v6, v5, v3, v2, v17, v16, {
            let a130 = {
                let a128 = v16;
                let a129 = v11;
                nat_add(a128, a129)?
            };
            let a131 = v12;
            nat_add(a130, a131)?
        }, {
            let a138 = v18;
            let a139 = {
                let a136 = {
                    let a132 = v2;
                    let a133 = v14;
                    if nat_lt(a132, a133) {
                        v2
                    } else {
                        v14
                    }
                };
                let a137 = {
                    let a134 = v16;
                    let a135 = v17;
                    nat_mul(a134, a135)?
                };
                nat_mul(a136, a137)?
            };
            nat_add(a138, a139)?
        }, true)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f4(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(_, _, _, _, _, h140, _, _, _, _, _, _) => {
            h140
        }
    };
    let v5: Adt1 = match v0 {
        Adt6::C0(_, _, h141, _, _, _, _, _, _, _, _, _) => {
            h141
        }
    };
    let a143 = v1;
    let a144 = match v5.clone() {
        Adt1::C0(_, _, h142, _, _, _, _, _, _, _, _) => {
            h142
        }
    };
    if if if if nat_lt(a143, a144) {
        let a146 = v2;
        let a147 = match v5 {
            Adt1::C0(_, _, _, _, h145, _, _, _, _, _, _) => {
                h145
            }
        };
        nat_lt(a146, a147)
    } else {
        false
    } {
        let a149 = v3;
        let a150 = match v4.clone() {
            Adt1::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                h148
            }
        };
        nat_lt(a149, a150)
    } else {
        false
    } {
        let a151 = v3;
        let a152 = 2147483648u64;
        nat_lt(a151, a152)
    } else {
        false
    } {
        Ok(Adt7::C13(f20(v4, 0u64, v1, v3)?, None::<u64>))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f5(v0: Adt6, v1: u64, v2: u64) -> R<Adt7> {
    let v3: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h153, _, _, _, _, _, _, _, _, _) => {
            h153
        }
    };
    let a155 = v1;
    let a156 = match v3.clone() {
        Adt1::C0(_, _, h154, _, _, _, _, _, _, _, _) => {
            h154
        }
    };
    if if nat_lt(a155, a156) {
        let a158 = v2;
        let a159 = match v3.clone() {
            Adt1::C0(_, _, _, _, h157, _, _, _, _, _, _) => {
                h157
            }
        };
        nat_lt(a158, a159)
    } else {
        false
    } {
        Ok(Adt7::C13(f20(match v0.clone() {
            Adt6::C0(_, _, _, _, _, h160, _, _, _, _, _, _) => {
                h160
            }
        }, 0u64, v1, v2)?, {
            let a162 = match v3.clone() {
                Adt1::C0(_, _, _, h161, _, _, _, _, _, _, _) => {
                    h161
                }
            };
            let a163 = 0u64;
            if if nat_eq(a162, a163) {
                let a165 = match v0 {
                    Adt6::C0(_, _, _, _, _, _, h164, _, _, _, _, _) => {
                        h164
                    }
                };
                let a166 = 1u64;
                nat_eq(a165, a166)
            } else {
                false
            } {
                Some(f28(v3, v1, v2, 0u64)?)
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
        Adt6::C0(_, _, h167, _, _, _, _, _, _, _, _, _) => {
            h167
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(h168, _, _, _, _, _, _, _, _, _, _, _) => {
            h168
        }
    };
    let v6: u64 = match v4.clone() {
        Adt1::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
            h169
        }
    };
    let v7: u64 = match v4.clone() {
        Adt1::C0(_, _, h170, _, _, _, _, _, _, _, _) => {
            h170
        }
    };
    let v8: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h171, _, _, _, _, _, _, _) => {
            h171
        }
    };
    let v9: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h172, _, _, _, _, _, _) => {
            h172
        }
    };
    let a173 = v1;
    let a174 = v7;
    if if if nat_lt(a173, a174) {
        let a175 = v2;
        let a176 = v9;
        nat_lt(a175, a176)
    } else {
        false
    } {
        let a177 = v3;
        let a178 = v8;
        nat_lt(a177, a178)
    } else {
        false
    } {
        let v10: u64 = {
            let a183 = v1;
            let a184 = {
                let a180 = v7;
                let a181 = match v5.clone() {
                    Adt1::C0(_, _, h179, _, _, _, _, _, _, _, _) => {
                        h179
                    }
                };
                let a182 = 0u64;
                nat_quot(a180, a181, a182)
            };
            let a185 = 0u64;
            nat_quot(a183, a184, a185)
        };
        let v11: u64 = {
            let a188 = v8;
            let a189 = {
                let a186 = v3;
                let a187 = 1u64;
                nat_add(a186, a187)?
            };
            nat_sub(a188, a189)
        };
        Ok(Adt7::C14(f20(v5, v10, v3, v2)?, f20(match v0.clone() {
            Adt6::C0(_, h190, _, _, _, _, _, _, _, _, _, _) => {
                h190
            }
        }, v10, v3, v2)?, f20(v4, v1, v3, v2)?, f20(match v0.clone() {
            Adt6::C0(_, _, _, h191, _, _, _, _, _, _, _, _) => {
                h191
            }
        }, v1, 0u64, 0u64)?, {
            let a195 = f20(match v0.clone() {
                Adt6::C0(_, _, _, _, h192, _, _, _, _, _, _, _) => {
                    h192
                }
            }, v3, v2, 0u64)?;
            let a196 = {
                let a193 = 4u64;
                let a194 = v1;
                nat_mul(a193, a194)?
            };
            nat_add(a195, a196)?
        }, {
            let a207 = 4u64;
            let a208 = {
                let a205 = v6;
                let a206 = {
                    let a203 = v1;
                    let a204 = {
                        let a201 = v7;
                        let a202 = {
                            let a199 = v3;
                            let a200 = {
                                let a197 = v8;
                                let a198 = v2;
                                nat_mul(a197, a198)?
                            };
                            nat_add(a199, a200)?
                        };
                        nat_mul(a201, a202)?
                    };
                    nat_add(a203, a204)?
                };
                nat_mul(a205, a206)?
            };
            nat_mul(a207, a208)?
        }, {
            let a210 = v11;
            let a211 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, h209, _, _, _, _, _) => {
                    h209
                }
            };
            if nat_lt(a210, a211) {
                Some({
                    let a230 = 4u64;
                    let a231 = {
                        let a228 = {
                            let a215 = {
                                let a212 = v6;
                                let a213 = v8;
                                nat_mul(a212, a213)?
                            };
                            let a216 = match v0.clone() {
                                Adt6::C0(_, _, _, _, _, _, _, h214, _, _, _, _) => {
                                    h214
                                }
                            };
                            nat_mul(a215, a216)?
                        };
                        let a229 = {
                            let a226 = match v0 {
                                Adt6::C0(_, _, _, _, _, _, _, _, h217, _, _, _) => {
                                    h217
                                }
                            };
                            let a227 = {
                                let a224 = v1;
                                let a225 = {
                                    let a222 = v7;
                                    let a223 = {
                                        let a220 = v2;
                                        let a221 = {
                                            let a218 = v9;
                                            let a219 = v11;
                                            nat_mul(a218, a219)?
                                        };
                                        nat_add(a220, a221)?
                                    };
                                    nat_mul(a222, a223)?
                                };
                                nat_add(a224, a225)?
                            };
                            nat_mul(a226, a227)?
                        };
                        nat_add(a228, a229)?
                    };
                    nat_mul(a230, a231)?
                })
            } else {
                None::<u64>
            }
        }))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f7(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(h232, _, _, _, _, _, _, _, _, _, _, _) => {
            h232
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(_, h233, _, _, _, _, _, _, _, _, _, _) => {
            h233
        }
    };
    let v6: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h234, _, _, _, _, _, _, _, _, _) => {
            h234
        }
    };
    let a236 = v1;
    let a237 = match v6.clone() {
        Adt1::C0(_, _, h235, _, _, _, _, _, _, _, _) => {
            h235
        }
    };
    if if if nat_lt(a236, a237) {
        let a239 = v2;
        let a240 = match v6.clone() {
            Adt1::C0(_, _, _, _, h238, _, _, _, _, _, _) => {
                h238
            }
        };
        nat_lt(a239, a240)
    } else {
        false
    } {
        let a242 = v3;
        let a243 = match v6.clone() {
            Adt1::C0(_, _, _, h241, _, _, _, _, _, _, _) => {
                h241
            }
        };
        nat_lt(a242, a243)
    } else {
        false
    } {
        let v7: u64 = {
            let a247 = match v6.clone() {
                Adt1::C0(_, _, _, h244, _, _, _, _, _, _, _) => {
                    h244
                }
            };
            let a248 = {
                let a245 = v3;
                let a246 = 1u64;
                nat_add(a245, a246)?
            };
            nat_sub(a247, a248)
        };
        Ok(Adt7::C14(f20(v4.clone(), {
            let a250 = v1;
            let a251 = match v4.clone() {
                Adt1::C0(_, _, h249, _, _, _, _, _, _, _, _) => {
                    h249
                }
            };
            let a252 = 0u64;
            nat_rem(a250, a251, a252)
        }, v3, {
            let a258 = v2;
            let a259 = {
                let a255 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h253, _, _, _, _, _, _) => {
                        h253
                    }
                };
                let a256 = match v4 {
                    Adt1::C0(_, _, _, _, h254, _, _, _, _, _, _) => {
                        h254
                    }
                };
                let a257 = 0u64;
                nat_quot(a255, a256, a257)
            };
            let a260 = 0u64;
            nat_quot(a258, a259, a260)
        })?, f20(v5.clone(), {
            let a262 = v1;
            let a263 = match v5.clone() {
                Adt1::C0(_, _, h261, _, _, _, _, _, _, _, _) => {
                    h261
                }
            };
            let a264 = 0u64;
            nat_rem(a262, a263, a264)
        }, v3, {
            let a270 = v2;
            let a271 = {
                let a267 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h265, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a268 = match v5 {
                    Adt1::C0(_, _, _, _, h266, _, _, _, _, _, _) => {
                        h266
                    }
                };
                let a269 = 0u64;
                nat_quot(a267, a268, a269)
            };
            let a272 = 0u64;
            nat_quot(a270, a271, a272)
        })?, f20(v6.clone(), v1, v3, v2)?, f20(match v0.clone() {
            Adt6::C0(_, _, _, h273, _, _, _, _, _, _, _, _) => {
                h273
            }
        }, v1, v3, v2)?, f20(match v0.clone() {
            Adt6::C0(_, _, _, _, h274, _, _, _, _, _, _, _) => {
                h274
            }
        }, v1, v3, v2)?, {
            let a288 = 4u64;
            let a289 = {
                let a286 = match v6.clone() {
                    Adt1::C0(_, h275, _, _, _, _, _, _, _, _, _) => {
                        h275
                    }
                };
                let a287 = {
                    let a284 = v1;
                    let a285 = {
                        let a282 = match v6.clone() {
                            Adt1::C0(_, _, h276, _, _, _, _, _, _, _, _) => {
                                h276
                            }
                        };
                        let a283 = {
                            let a280 = v3;
                            let a281 = {
                                let a278 = match v6.clone() {
                                    Adt1::C0(_, _, _, h277, _, _, _, _, _, _, _) => {
                                        h277
                                    }
                                };
                                let a279 = v2;
                                nat_mul(a278, a279)?
                            };
                            nat_add(a280, a281)?
                        };
                        nat_mul(a282, a283)?
                    };
                    nat_add(a284, a285)?
                };
                nat_mul(a286, a287)?
            };
            nat_mul(a288, a289)?
        }, {
            let a291 = v7;
            let a292 = match v0 {
                Adt6::C0(_, _, _, _, _, _, h290, _, _, _, _, _) => {
                    h290
                }
            };
            if nat_lt(a291, a292) {
                Some(f28(v6, v1, v2, v7)?)
            } else {
                None::<u64>
            }
        }))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f8(v0: bool, v1: u64, v2: Option<Adt1>, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Option<Adt1>, v8: Adt1) -> R<Adt7> {
    if v0 {
        Ok(Adt7::C0)
    } else {
        let m293 = v2;
        match m293 {
            None => {
                Ok(Adt7::C0)
            }
            Some(v9) => {
                let m294 = v3;
                match m294 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v10) => {
                        let m295 = v4;
                        match m295 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v11) => {
                                let m296 = v5;
                                match m296 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v12) => {
                                        let m297 = v6;
                                        match m297 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v13) => {
                                                let m298 = v7;
                                                match m298 {
                                                    None => {
                                                        Ok(Adt7::C0)
                                                    }
                                                    Some(v14) => {
                                                        f29(v1, v9, v10, v11, v12, v13, v14, v8)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: u64, v6: Option<Adt1>, v7: Option<Adt1>, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt7> {
    let m299 = v6;
    match m299 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a300 = v0;
            let a301 = 43u64;
            if nat_eq(a300, a301) {
                let m302 = v9;
                match m302 {
                    None => {
                        let m303 = v10;
                        match m303 {
                            None => {
                                f14(v0, v1, v2, v3, v12, v7, v8, v11)
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
                let m304 = v7;
                match m304 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v15) => {
                        let a305 = v0;
                        let a306 = 84u64;
                        if nat_eq(a305, a306) {
                            f30(v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let a307 = v0;
                            let a308 = 88u64;
                            if nat_eq(a307, a308) {
                                f11(v0, v1, v12, v15, v8, v9, v10, v11)
                            } else {
                                let m309 = v8;
                                match m309 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v16) => {
                                        f10(v0, v1, v3, v4, v5, v12, v15, v16, v9, v10, None::<Adt1>, v11)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Option<Adt1>, v11: Adt1) -> R<Adt7> {
    let m310 = f31(v10, v8.clone(), v5.clone(), v6.clone(), v7.clone(), v11.clone())?;
    match m310 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a311 = v0;
            let a312 = 71u64;
            if if if if if if if if if if if if if if if if if if nat_eq(a311, a312) {
                !v1
            } else {
                false
            } {
                f32(v2, match v5.clone() {
                    Adt1::C0(_, _, _, h313, _, _, _, _, _, _, _) => {
                        h313
                    }
                }, v8.clone())
            } else {
                false
            } {
                let a314 = v4;
                let a315 = 0u64;
                if if if nat_eq(a314, a315) {
                    true
                } else {
                    let a316 = v4;
                    let a317 = 10u64;
                    nat_eq(a316, a317)
                } {
                    true
                } else {
                    let a318 = v4;
                    let a319 = 15u64;
                    nat_eq(a318, a319)
                } {
                    true
                } else {
                    let a320 = v4;
                    let a321 = 20u64;
                    nat_eq(a320, a321)
                }
            } else {
                false
            } {
                let a323 = match v5.clone() {
                    Adt1::C0(h322, _, _, _, _, _, _, _, _, _, _) => {
                        h322
                    }
                };
                let a324 = 0u64;
                nat_eq(a323, a324)
            } else {
                false
            } {
                let a326 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h325, _, _, _, _, _) => {
                        h325
                    }
                };
                let a327 = 4u64;
                nat_eq(a326, a327)
            } else {
                false
            } {
                f33(v11.clone())?
            } else {
                false
            } {
                f34(v6.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                f34(v7.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                let a330 = match v5.clone() {
                    Adt1::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
                        h328
                    }
                };
                let a331 = match v6.clone() {
                    Adt1::C0(_, h329, _, _, _, _, _, _, _, _, _) => {
                        h329
                    }
                };
                nat_eq(a330, a331)
            } else {
                false
            } {
                let a334 = match v6.clone() {
                    Adt1::C0(_, _, h332, _, _, _, _, _, _, _, _) => {
                        h332
                    }
                };
                let a335 = match v7.clone() {
                    Adt1::C0(_, _, h333, _, _, _, _, _, _, _, _) => {
                        h333
                    }
                };
                nat_eq(a334, a335)
            } else {
                false
            } {
                let a338 = match v11.clone() {
                    Adt1::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                        h336
                    }
                };
                let a339 = match v7.clone() {
                    Adt1::C0(_, h337, _, _, _, _, _, _, _, _, _) => {
                        h337
                    }
                };
                nat_eq(a338, a339)
            } else {
                false
            } {
                let a342 = match v11.clone() {
                    Adt1::C0(_, _, h340, _, _, _, _, _, _, _, _) => {
                        h340
                    }
                };
                let a343 = match v5.clone() {
                    Adt1::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                        h341
                    }
                };
                nat_eq(a342, a343)
            } else {
                false
            } {
                let a346 = match v11.clone() {
                    Adt1::C0(_, _, _, h344, _, _, _, _, _, _, _) => {
                        h344
                    }
                };
                let a347 = match v5.clone() {
                    Adt1::C0(_, _, h345, _, _, _, _, _, _, _, _) => {
                        h345
                    }
                };
                nat_eq(a346, a347)
            } else {
                false
            } {
                let a350 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h348, _, _, _, _, _, _) => {
                        h348
                    }
                };
                let a351 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h349, _, _, _, _, _, _) => {
                        h349
                    }
                };
                nat_eq(a350, a351)
            } else {
                false
            } {
                f27(v5.clone(), v11.clone())?
            } else {
                false
            } {
                f35(v8.clone(), v5.clone(), v12, v11.clone())?
            } else {
                false
            } {
                f36(v9.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                Ok(Adt7::C1(Adt2::C0(v5, v6, v7, v8, v9, v11, v12, f37(v2), f38(v3))))
            } else {
                Ok(Adt7::C0)
            }
        }
    }
}

pub fn f11(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    let v8: u64 = {
        let a353 = match v2.clone() {
            Adt1::C0(h352, _, _, _, _, _, _, _, _, _, _) => {
                h352
            }
        };
        let a354 = 1u64;
        if nat_eq(a353, a354) {
            2u64
        } else {
            4u64
        }
    };
    let a355 = v0;
    let a356 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a355, a356) {
        !v1
    } else {
        false
    } {
        let a358 = match v2.clone() {
            Adt1::C0(h357, _, _, _, _, _, _, _, _, _, _) => {
                h357
            }
        };
        let a359 = 0u64;
        if nat_eq(a358, a359) {
            true
        } else {
            let a361 = match v2.clone() {
                Adt1::C0(h360, _, _, _, _, _, _, _, _, _, _) => {
                    h360
                }
            };
            let a362 = 1u64;
            nat_eq(a361, a362)
        }
    } else {
        false
    } {
        let a364 = match v3.clone() {
            Adt1::C0(h363, _, _, _, _, _, _, _, _, _, _) => {
                h363
            }
        };
        let a365 = 26u64;
        nat_eq(a364, a365)
    } else {
        false
    } {
        let a368 = match v7.clone() {
            Adt1::C0(h366, _, _, _, _, _, _, _, _, _, _) => {
                h366
            }
        };
        let a369 = match v2.clone() {
            Adt1::C0(h367, _, _, _, _, _, _, _, _, _, _) => {
                h367
            }
        };
        nat_eq(a368, a369)
    } else {
        false
    } {
        let a371 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h370, _, _, _, _, _) => {
                h370
            }
        };
        let a372 = v8;
        nat_eq(a371, a372)
    } else {
        false
    } {
        let a374 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h373, _, _, _, _, _) => {
                h373
            }
        };
        let a375 = v8;
        nat_eq(a374, a375)
    } else {
        false
    } {
        let a378 = match v2.clone() {
            Adt1::C0(_, h376, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        };
        let a379 = match v7.clone() {
            Adt1::C0(_, h377, _, _, _, _, _, _, _, _, _) => {
                h377
            }
        };
        nat_eq(a378, a379)
    } else {
        false
    } {
        f39(v3.clone(), v7.clone())
    } else {
        false
    } {
        f40({
            let a381 = v8;
            let a382 = match v7.clone() {
                Adt1::C0(_, h380, _, _, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            nat_mul(a381, a382)?
        }, match v7.clone() {
            Adt1::C0(_, _, h383, _, _, _, _, _, _, _, _) => {
                h383
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h384, _, _, _, _, _, _, _) => {
                h384
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h385, _, _, _, _, _, _) => {
                h385
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h386, _, _, _, _) => {
                h386
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h387, _, _, _) => {
                h387
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h388, _, _) => {
                h388
            }
        })?
    } else {
        false
    } {
        f27(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f27(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f21(match v3.clone() {
            Adt1::C0(_, _, h389, _, _, _, _, _, _, _, _) => {
                h389
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h390, _, _, _, _, _, _, _, _) => {
                h390
            }
        })
    } else {
        false
    } {
        f21(match v3.clone() {
            Adt1::C0(_, _, _, h391, _, _, _, _, _, _, _) => {
                h391
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h392, _, _, _, _, _, _, _) => {
                h392
            }
        })
    } else {
        false
    } {
        f21(match v3.clone() {
            Adt1::C0(_, _, _, _, h393, _, _, _, _, _, _) => {
                h393
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h394, _, _, _, _, _, _) => {
                h394
            }
        })
    } else {
        false
    } {
        let m395 = v4.clone();
        match m395 {
            None => {
                true
            }
            Some(v9) => {
                let a397 = match v9.clone() {
                    Adt1::C0(h396, _, _, _, _, _, _, _, _, _, _) => {
                        h396
                    }
                };
                let a398 = 0u64;
                if if nat_eq(a397, a398) {
                    f39(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f27(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m399 = v5.clone();
        match m399 {
            None => {
                true
            }
            Some(v10) => {
                let a401 = match v10.clone() {
                    Adt1::C0(h400, _, _, _, _, _, _, _, _, _, _) => {
                        h400
                    }
                };
                let a402 = 26u64;
                if if if if nat_eq(a401, a402) {
                    let a404 = match v10.clone() {
                        Adt1::C0(_, _, _, h403, _, _, _, _, _, _, _) => {
                            h403
                        }
                    };
                    let a405 = 1u64;
                    nat_eq(a404, a405)
                } else {
                    false
                } {
                    let a407 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h406, _, _, _, _, _, _) => {
                            h406
                        }
                    };
                    let a408 = 1u64;
                    nat_eq(a407, a408)
                } else {
                    false
                } {
                    f27(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m409 = v4.clone();
                    match m409 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a412 = match v11 {
                                Adt1::C0(_, h410, _, _, _, _, _, _, _, _, _) => {
                                    h410
                                }
                            };
                            let a413 = match v10 {
                                Adt1::C0(_, _, h411, _, _, _, _, _, _, _, _) => {
                                    h411
                                }
                            };
                            nat_eq(a412, a413)
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
        let m414 = v6.clone();
        match m414 {
            None => {
                true
            }
            Some(v12) => {
                let a416 = match v12.clone() {
                    Adt1::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
                        h415
                    }
                };
                let a417 = 26u64;
                if if nat_eq(a416, a417) {
                    f39(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f27(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt7::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a418 = v8;
            let a419 = 2u64;
            if nat_eq(a418, a419) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m420 = v5;
            match m420 {
                None => {
                    let m421 = v4;
                    match m421 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h422, _, _, _, _, _, _, _, _, _) => {
                                    h422
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h423, _, _, _, _, _, _, _, _, _) => {
                                    h423
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h424, _, _, _, _, _, _, _, _) => {
                            h424
                        }
                    }
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f12(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt4::C0(h425, _, _, _, _, _, _, _, _) => {
            h425
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h426, _, _, _, _, _, _, _) => {
            h426
        }
    };
    let a428 = v1;
    let a429 = match v5.clone() {
        Adt1::C0(_, _, h427, _, _, _, _, _, _, _, _) => {
            h427
        }
    };
    if if if nat_lt(a428, a429) {
        let a431 = v2;
        let a432 = match v5.clone() {
            Adt1::C0(_, _, _, h430, _, _, _, _, _, _, _) => {
                h430
            }
        };
        nat_lt(a431, a432)
    } else {
        false
    } {
        let a434 = v3;
        let a435 = match v5.clone() {
            Adt1::C0(_, _, _, _, h433, _, _, _, _, _, _) => {
                h433
            }
        };
        nat_lt(a434, a435)
    } else {
        false
    } {
        Ok(Adt7::C6(f20(v4.clone(), {
            let a437 = v1;
            let a438 = match v4.clone() {
                Adt1::C0(_, _, h436, _, _, _, _, _, _, _, _) => {
                    h436
                }
            };
            let a439 = 0u64;
            nat_rem(a437, a438, a439)
        }, {
            let a441 = v2;
            let a442 = match v4.clone() {
                Adt1::C0(_, _, _, h440, _, _, _, _, _, _, _) => {
                    h440
                }
            };
            let a443 = 0u64;
            nat_rem(a441, a442, a443)
        }, {
            let a445 = v3;
            let a446 = match v4 {
                Adt1::C0(_, _, _, _, h444, _, _, _, _, _, _) => {
                    h444
                }
            };
            let a447 = 0u64;
            nat_rem(a445, a446, a447)
        })?, f20(v5, v1, v2, v3)?, {
            let m449 = match v0.clone() {
                Adt4::C0(_, _, h448, _, _, _, _, _, _) => {
                    h448
                }
            };
            match m449 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f20(v6, v1, v2, v3)?
                }
            }
        }, {
            let m451 = match v0.clone() {
                Adt4::C0(_, _, _, _, h450, _, _, _, _) => {
                    h450
                }
            };
            match m451 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f20(v7, v1, v2, v3)?
                }
            }
        }, f20(match v0 {
            Adt4::C0(_, _, _, _, _, h452, _, _, _) => {
                h452
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f13(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a453 = v1;
    let a454 = 0u64;
    if nat_eq(a453, a454) {
        let a458 = v2;
        let a459 = {
            let m457 = match v0.clone() {
                Adt4::C0(_, h456, _, _, _, _, _, _, _) => {
                    h456
                }
            };
            match m457 {
                Adt1::C0(_, h455, _, _, _, _, _, _, _, _, _) => {
                    h455
                }
            }
        };
        if nat_lt(a458, a459) {
            Ok(Adt7::C7({
                let a463 = v2;
                let a464 = {
                    let m462 = match v0 {
                        Adt4::C0(_, h461, _, _, _, _, _, _, _) => {
                            h461
                        }
                    };
                    match m462 {
                        Adt1::C0(_, _, _, _, _, h460, _, _, _, _, _) => {
                            h460
                        }
                    }
                };
                nat_mul(a463, a464)?
            }))
        } else {
            Ok(Adt7::C8)
        }
    } else {
        let a465 = v1;
        let a466 = 1u64;
        if nat_eq(a465, a466) {
            let m468 = match v0 {
                Adt4::C0(_, _, _, _, h467, _, _, _, _) => {
                    h467
                }
            };
            match m468 {
                None => {
                    Ok(Adt7::C8)
                }
                Some(v4) => {
                    let a470 = v2;
                    let a471 = match v4.clone() {
                        Adt1::C0(_, h469, _, _, _, _, _, _, _, _, _) => {
                            h469
                        }
                    };
                    if nat_lt(a470, a471) {
                        Ok(Adt7::C7({
                            let a473 = v2;
                            let a474 = match v4 {
                                Adt1::C0(_, _, _, _, _, h472, _, _, _, _, _) => {
                                    h472
                                }
                            };
                            nat_mul(a473, a474)?
                        }))
                    } else {
                        Ok(Adt7::C8)
                    }
                }
            }
        } else {
            let a475 = v1;
            let a476 = 2u64;
            if nat_eq(a475, a476) {
                let a477 = v2;
                let a478 = 2147483648u64;
                if if nat_lt(a477, a478) {
                    let a480 = v2;
                    let a481 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h479) => {
                            h479
                        }
                    };
                    nat_lt(a480, a481)
                } else {
                    false
                } {
                    let m483 = match v0 {
                        Adt4::C0(_, _, h482, _, _, _, _, _, _) => {
                            h482
                        }
                    };
                    match m483 {
                        None => {
                            Ok(Adt7::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt7::C7({
                                let a485 = v2;
                                let a486 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h484, _, _, _, _, _) => {
                                        h484
                                    }
                                };
                                nat_mul(a485, a486)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt7::C8)
                }
            } else {
                let a487 = v1;
                let a488 = 3u64;
                if nat_eq(a487, a488) {
                    let m490 = match v0 {
                        Adt4::C0(_, _, _, h489, _, _, _, _, _) => {
                            h489
                        }
                    };
                    match m490 {
                        None => {
                            Ok(Adt7::C8)
                        }
                        Some(v6) => {
                            let a492 = v2;
                            let a493 = match v6.clone() {
                                Adt1::C0(_, _, h491, _, _, _, _, _, _, _, _) => {
                                    h491
                                }
                            };
                            if if nat_lt(a492, a493) {
                                let a495 = v3;
                                let a496 = match v6.clone() {
                                    Adt1::C0(_, h494, _, _, _, _, _, _, _, _, _) => {
                                        h494
                                    }
                                };
                                nat_lt(a495, a496)
                            } else {
                                false
                            } {
                                Ok(Adt7::C7({
                                    let a503 = {
                                        let a498 = v2;
                                        let a499 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h497, _, _, _, _) => {
                                                h497
                                            }
                                        };
                                        nat_mul(a498, a499)?
                                    };
                                    let a504 = {
                                        let a501 = v3;
                                        let a502 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h500, _, _, _, _, _) => {
                                                h500
                                            }
                                        };
                                        nat_mul(a501, a502)?
                                    };
                                    nat_add(a503, a504)?
                                }))
                            } else {
                                Ok(Adt7::C8)
                            }
                        }
                    }
                } else {
                    let a505 = v1;
                    let a506 = 4u64;
                    if nat_eq(a505, a506) {
                        let a507 = v2;
                        let a508 = 2147483648u64;
                        if if nat_lt(a507, a508) {
                            let a512 = v2;
                            let a513 = {
                                let m511 = match v0.clone() {
                                    Adt4::C0(h510, _, _, _, _, _, _, _, _) => {
                                        h510
                                    }
                                };
                                match m511 {
                                    Adt1::C0(_, h509, _, _, _, _, _, _, _, _, _) => {
                                        h509
                                    }
                                }
                            };
                            nat_lt(a512, a513)
                        } else {
                            false
                        } {
                            Ok(Adt7::C7({
                                let a515 = v2;
                                let a516 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h514, _, _) => {
                                        h514
                                    }
                                };
                                nat_mul(a515, a516)?
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

pub fn f14(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    if v1 {
        Ok(Adt7::C0)
    } else {
        let a517 = v0;
        let a518 = 43u64;
        if if if if if if if if nat_eq(a517, a518) {
            let a520 = match v4.clone() {
                Adt1::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
                    h519
                }
            };
            let a521 = 0u64;
            if nat_eq(a520, a521) {
                f24(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a523 = match v7.clone() {
                Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                    h522
                }
            };
            let a524 = 0u64;
            if nat_eq(a523, a524) {
                f24(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f26(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f27(v4.clone(), v7.clone())? {
                true
            } else {
                f41(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f42(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f43(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f32(v3, match v4.clone() {
                Adt1::C0(_, _, _, h525, _, _, _, _, _, _, _) => {
                    h525
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt7::C3(Adt3::C0(v4, v5, v7, v2, f37(v3))))
        } else {
            Ok(Adt7::C0)
        }
    }
}

pub fn f15(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h526, _, _, _, _) => {
            h526
        }
    };
    let a528 = v1;
    let a529 = match v4.clone() {
        Adt1::C0(_, _, h527, _, _, _, _, _, _, _, _) => {
            h527
        }
    };
    if if if nat_lt(a528, a529) {
        let a531 = v2;
        let a532 = match v4.clone() {
            Adt1::C0(_, _, _, h530, _, _, _, _, _, _, _) => {
                h530
            }
        };
        nat_lt(a531, a532)
    } else {
        false
    } {
        let a534 = v3;
        let a535 = match v4.clone() {
            Adt1::C0(_, _, _, _, h533, _, _, _, _, _, _) => {
                h533
            }
        };
        nat_lt(a534, a535)
    } else {
        false
    } {
        let v6: u64 = {
            let m537 = match v0.clone() {
                Adt3::C0(_, h536, _, _, _) => {
                    h536
                }
            };
            match m537 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f20(v5.clone(), v1, {
                        let a539 = v2;
                        let a540 = match v5.clone() {
                            Adt1::C0(_, _, _, h538, _, _, _, _, _, _, _) => {
                                h538
                            }
                        };
                        let a541 = 0u64;
                        nat_rem(a539, a540, a541)
                    }, {
                        let a543 = v3;
                        let a544 = match v5 {
                            Adt1::C0(_, _, _, _, h542, _, _, _, _, _, _) => {
                                h542
                            }
                        };
                        let a545 = 0u64;
                        nat_rem(a543, a544, a545)
                    })?
                }
            }
        };
        Ok(Adt7::C4(f20(v4, v1, v2, v3)?, v6, {
            let a546 = v2;
            let a547 = 4u64;
            nat_mul(a546, a547)?
        }, f20(match v0 {
            Adt3::C0(_, _, h548, _, _) => {
                h548
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f16(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a552 = v1;
    let a553 = {
        let m551 = match v0.clone() {
            Adt2::C0(h550, _, _, _, _, _, _, _, _) => {
                h550
            }
        };
        match m551 {
            Adt1::C0(_, _, h549, _, _, _, _, _, _, _, _) => {
                h549
            }
        }
    };
    if if if nat_lt(a552, a553) {
        let a557 = v2;
        let a558 = {
            let m556 = match v0.clone() {
                Adt2::C0(h555, _, _, _, _, _, _, _, _) => {
                    h555
                }
            };
            match m556 {
                Adt1::C0(_, _, _, h554, _, _, _, _, _, _, _) => {
                    h554
                }
            }
        };
        nat_lt(a557, a558)
    } else {
        false
    } {
        let a562 = v3;
        let a563 = {
            let m561 = match v0.clone() {
                Adt2::C0(h560, _, _, _, _, _, _, _, _) => {
                    h560
                }
            };
            match m561 {
                Adt1::C0(_, _, _, _, h559, _, _, _, _, _, _) => {
                    h559
                }
            }
        };
        nat_lt(a562, a563)
    } else {
        false
    } {
        Ok(Adt7::C2(f20(match v0.clone() {
            Adt2::C0(h564, _, _, _, _, _, _, _, _) => {
                h564
            }
        }, v1, v2, v3)?, f20(match v0.clone() {
            Adt2::C0(_, h565, _, _, _, _, _, _, _) => {
                h565
            }
        }, 0u64, {
            let a575 = v2;
            let a576 = {
                let a572 = {
                    let m568 = match v0.clone() {
                        Adt2::C0(h567, _, _, _, _, _, _, _, _) => {
                            h567
                        }
                    };
                    match m568 {
                        Adt1::C0(_, _, _, h566, _, _, _, _, _, _, _) => {
                            h566
                        }
                    }
                };
                let a573 = {
                    let m571 = match v0.clone() {
                        Adt2::C0(_, h570, _, _, _, _, _, _, _) => {
                            h570
                        }
                    };
                    match m571 {
                        Adt1::C0(_, _, _, h569, _, _, _, _, _, _, _) => {
                            h569
                        }
                    }
                };
                let a574 = 0u64;
                nat_quot(a572, a573, a574)
            };
            let a577 = 0u64;
            nat_quot(a575, a576, a577)
        }, {
            let a587 = v3;
            let a588 = {
                let a584 = {
                    let m580 = match v0.clone() {
                        Adt2::C0(h579, _, _, _, _, _, _, _, _) => {
                            h579
                        }
                    };
                    match m580 {
                        Adt1::C0(_, _, _, _, h578, _, _, _, _, _, _) => {
                            h578
                        }
                    }
                };
                let a585 = {
                    let m583 = match v0.clone() {
                        Adt2::C0(_, h582, _, _, _, _, _, _, _) => {
                            h582
                        }
                    };
                    match m583 {
                        Adt1::C0(_, _, _, _, h581, _, _, _, _, _, _) => {
                            h581
                        }
                    }
                };
                let a586 = 0u64;
                nat_quot(a584, a585, a586)
            };
            let a589 = 0u64;
            nat_quot(a587, a588, a589)
        })?, f20(match v0.clone() {
            Adt2::C0(_, _, h590, _, _, _, _, _, _) => {
                h590
            }
        }, 0u64, {
            let a600 = v2;
            let a601 = {
                let a597 = {
                    let m593 = match v0.clone() {
                        Adt2::C0(h592, _, _, _, _, _, _, _, _) => {
                            h592
                        }
                    };
                    match m593 {
                        Adt1::C0(_, _, _, h591, _, _, _, _, _, _, _) => {
                            h591
                        }
                    }
                };
                let a598 = {
                    let m596 = match v0.clone() {
                        Adt2::C0(_, _, h595, _, _, _, _, _, _) => {
                            h595
                        }
                    };
                    match m596 {
                        Adt1::C0(_, _, _, h594, _, _, _, _, _, _, _) => {
                            h594
                        }
                    }
                };
                let a599 = 0u64;
                nat_quot(a597, a598, a599)
            };
            let a602 = 0u64;
            nat_quot(a600, a601, a602)
        }, {
            let a612 = v3;
            let a613 = {
                let a609 = {
                    let m605 = match v0.clone() {
                        Adt2::C0(h604, _, _, _, _, _, _, _, _) => {
                            h604
                        }
                    };
                    match m605 {
                        Adt1::C0(_, _, _, _, h603, _, _, _, _, _, _) => {
                            h603
                        }
                    }
                };
                let a610 = {
                    let m608 = match v0.clone() {
                        Adt2::C0(_, _, h607, _, _, _, _, _, _) => {
                            h607
                        }
                    };
                    match m608 {
                        Adt1::C0(_, _, _, _, h606, _, _, _, _, _, _) => {
                            h606
                        }
                    }
                };
                let a611 = 0u64;
                nat_quot(a609, a610, a611)
            };
            let a614 = 0u64;
            nat_quot(a612, a613, a614)
        })?, {
            let m616 = match v0.clone() {
                Adt2::C0(_, _, _, h615, _, _, _, _, _) => {
                    h615
                }
            };
            match m616 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f20(v4.clone(), v1, {
                        let a618 = v2;
                        let a619 = match v4.clone() {
                            Adt1::C0(_, _, _, h617, _, _, _, _, _, _, _) => {
                                h617
                            }
                        };
                        let a620 = 0u64;
                        nat_rem(a618, a619, a620)
                    }, {
                        let a622 = v3;
                        let a623 = match v4 {
                            Adt1::C0(_, _, _, _, h621, _, _, _, _, _, _) => {
                                h621
                            }
                        };
                        let a624 = 0u64;
                        nat_rem(a622, a623, a624)
                    })?
                }
            }
        }, {
            let a625 = v2;
            let a626 = 4u64;
            nat_mul(a625, a626)?
        }, f20(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h627, _, _, _) => {
                h627
            }
        }, v2, v1, v3)?, {
            let m630 = match v0.clone() {
                Adt2::C0(_, h629, _, _, _, _, _, _, _) => {
                    h629
                }
            };
            match m630 {
                Adt1::C0(_, _, _, _, _, _, h628, _, _, _, _) => {
                    h628
                }
            }
        }, {
            let m633 = match v0.clone() {
                Adt2::C0(_, _, h632, _, _, _, _, _, _) => {
                    h632
                }
            };
            match m633 {
                Adt1::C0(_, _, _, _, _, _, h631, _, _, _, _) => {
                    h631
                }
            }
        }, f44({
            let m636 = match v0 {
                Adt2::C0(_, h635, _, _, _, _, _, _, _) => {
                    h635
                }
            };
            match m636 {
                Adt1::C0(h634, _, _, _, _, _, _, _, _, _, _) => {
                    h634
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f17(v0: u64) -> Adt8 {
    let a637 = v0;
    let a638 = 0u64;
    if nat_eq(a637, a638) {
        Adt8::C1(Adt9::C0(0u64, 1u64, 4u64))
    } else {
        let a639 = v0;
        let a640 = 1u64;
        if nat_eq(a639, a640) {
            Adt8::C1(Adt9::C0(1u64, 1u64, 2u64))
        } else {
            let a641 = v0;
            let a642 = 2u64;
            if nat_eq(a641, a642) {
                Adt8::C1(Adt9::C0(2u64, 32u64, 18u64))
            } else {
                let a643 = v0;
                let a644 = 3u64;
                if nat_eq(a643, a644) {
                    Adt8::C1(Adt9::C0(3u64, 32u64, 20u64))
                } else {
                    let a645 = v0;
                    let a646 = 6u64;
                    if nat_eq(a645, a646) {
                        Adt8::C1(Adt9::C0(6u64, 32u64, 22u64))
                    } else {
                        let a647 = v0;
                        let a648 = 7u64;
                        if nat_eq(a647, a648) {
                            Adt8::C1(Adt9::C0(7u64, 32u64, 24u64))
                        } else {
                            let a649 = v0;
                            let a650 = 8u64;
                            if nat_eq(a649, a650) {
                                Adt8::C1(Adt9::C0(8u64, 32u64, 34u64))
                            } else {
                                let a651 = v0;
                                let a652 = 9u64;
                                if nat_eq(a651, a652) {
                                    Adt8::C1(Adt9::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a653 = v0;
                                    let a654 = 10u64;
                                    if nat_eq(a653, a654) {
                                        Adt8::C1(Adt9::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a655 = v0;
                                        let a656 = 11u64;
                                        if nat_eq(a655, a656) {
                                            Adt8::C1(Adt9::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a657 = v0;
                                            let a658 = 12u64;
                                            if nat_eq(a657, a658) {
                                                Adt8::C1(Adt9::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a659 = v0;
                                                let a660 = 13u64;
                                                if nat_eq(a659, a660) {
                                                    Adt8::C1(Adt9::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a661 = v0;
                                                    let a662 = 14u64;
                                                    if nat_eq(a661, a662) {
                                                        Adt8::C1(Adt9::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a663 = v0;
                                                        let a664 = 15u64;
                                                        if nat_eq(a663, a664) {
                                                            Adt8::C1(Adt9::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a665 = v0;
                                                            let a666 = 16u64;
                                                            if nat_eq(a665, a666) {
                                                                Adt8::C1(Adt9::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a667 = v0;
                                                                let a668 = 17u64;
                                                                if nat_eq(a667, a668) {
                                                                    Adt8::C1(Adt9::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a669 = v0;
                                                                    let a670 = 18u64;
                                                                    if nat_eq(a669, a670) {
                                                                        Adt8::C1(Adt9::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a671 = v0;
                                                                        let a672 = 19u64;
                                                                        if nat_eq(a671, a672) {
                                                                            Adt8::C1(Adt9::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a673 = v0;
                                                                            let a674 = 20u64;
                                                                            if nat_eq(a673, a674) {
                                                                                Adt8::C1(Adt9::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a675 = v0;
                                                                                let a676 = 21u64;
                                                                                if nat_eq(a675, a676) {
                                                                                    Adt8::C1(Adt9::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a677 = v0;
                                                                                    let a678 = 22u64;
                                                                                    if nat_eq(a677, a678) {
                                                                                        Adt8::C1(Adt9::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a679 = v0;
                                                                                        let a680 = 23u64;
                                                                                        if nat_eq(a679, a680) {
                                                                                            Adt8::C1(Adt9::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a681 = v0;
                                                                                            let a682 = 24u64;
                                                                                            if nat_eq(a681, a682) {
                                                                                                Adt8::C1(Adt9::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a683 = v0;
                                                                                                let a684 = 25u64;
                                                                                                if nat_eq(a683, a684) {
                                                                                                    Adt8::C1(Adt9::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a685 = v0;
                                                                                                    let a686 = 26u64;
                                                                                                    if nat_eq(a685, a686) {
                                                                                                        Adt8::C1(Adt9::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a687 = v0;
                                                                                                        let a688 = 27u64;
                                                                                                        if nat_eq(a687, a688) {
                                                                                                            Adt8::C1(Adt9::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a689 = v0;
                                                                                                            let a690 = 28u64;
                                                                                                            if nat_eq(a689, a690) {
                                                                                                                Adt8::C1(Adt9::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a691 = v0;
                                                                                                                let a692 = 29u64;
                                                                                                                if nat_eq(a691, a692) {
                                                                                                                    Adt8::C1(Adt9::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a693 = v0;
                                                                                                                    let a694 = 30u64;
                                                                                                                    if nat_eq(a693, a694) {
                                                                                                                        Adt8::C1(Adt9::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a695 = v0;
                                                                                                                        let a696 = 34u64;
                                                                                                                        if nat_eq(a695, a696) {
                                                                                                                            Adt8::C1(Adt9::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a697 = v0;
                                                                                                                            let a698 = 35u64;
                                                                                                                            if nat_eq(a697, a698) {
                                                                                                                                Adt8::C1(Adt9::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a699 = v0;
                                                                                                                                let a700 = 39u64;
                                                                                                                                if nat_eq(a699, a700) {
                                                                                                                                    Adt8::C1(Adt9::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a701 = v0;
                                                                                                                                    let a702 = 40u64;
                                                                                                                                    if nat_eq(a701, a702) {
                                                                                                                                        Adt8::C1(Adt9::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a703 = v0;
                                                                                                                                        let a704 = 41u64;
                                                                                                                                        if nat_eq(a703, a704) {
                                                                                                                                            Adt8::C1(Adt9::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a705 = v0;
                                                                                                                                            let a706 = 42u64;
                                                                                                                                            if nat_eq(a705, a706) {
                                                                                                                                                Adt8::C1(Adt9::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a707 = v0;
                                                                                                                                                let a708 = 43u64;
                                                                                                                                                if nat_eq(a707, a708) {
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

pub fn f18(v0: u64) -> bool {
    if if f45(v0) {
        true
    } else {
        f46(v0)
    } {
        true
    } else {
        f47(v0)
    }
}

pub fn f19(v0: u64) -> Adt10 {
    let a709 = v0;
    let a710 = 0u64;
    if if if if nat_eq(a709, a710) {
        true
    } else {
        let a711 = v0;
        let a712 = 1u64;
        nat_eq(a711, a712)
    } {
        true
    } else {
        let a713 = v0;
        let a714 = 30u64;
        nat_eq(a713, a714)
    } {
        true
    } else {
        let a715 = v0;
        let a716 = 43u64;
        nat_eq(a715, a716)
    } {
        Adt10::C1(2u64, 1u64, 0u64, 2u64)
    } else {
        Adt10::C0
    }
}

pub fn f20(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a728 = {
        let a723 = {
            let a718 = v1;
            let a719 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h717, _, _, _, _) => {
                    h717
                }
            };
            nat_mul(a718, a719)?
        };
        let a724 = {
            let a721 = v2;
            let a722 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h720, _, _, _) => {
                    h720
                }
            };
            nat_mul(a721, a722)?
        };
        nat_add(a723, a724)?
    };
    let a729 = {
        let a726 = v3;
        let a727 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h725, _, _) => {
                h725
            }
        };
        nat_mul(a726, a727)?
    };
    nat_add(a728, a729)
}

pub fn f21(v0: u64, v1: u64) -> bool {
    let a730 = 0u64;
    let a731 = v1;
    if nat_lt(a730, a731) {
        let a735 = {
            let a732 = v0;
            let a733 = v1;
            let a734 = 0u64;
            nat_rem(a732, a733, a734)
        };
        let a736 = 0u64;
        nat_eq(a735, a736)
    } else {
        false
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> R<bool> {
    let a738 = match v0.clone() {
        Adt1::C0(h737, _, _, _, _, _, _, _, _, _, _) => {
            h737
        }
    };
    let a739 = 0u64;
    if if nat_eq(a738, a739) {
        let a741 = match v0.clone() {
            Adt1::C0(_, h740, _, _, _, _, _, _, _, _, _) => {
                h740
            }
        };
        let a742 = 1u64;
        if nat_eq(a741, a742) {
            true
        } else {
            let a744 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h743, _, _, _, _, _) => {
                    h743
                }
            };
            let a745 = 4u64;
            nat_eq(a744, a745)
        }
    } else {
        false
    } {
        f27(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt1) -> R<bool> {
    let a747 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h746, _, _, _, _, _) => {
            h746
        }
    };
    let a748 = 4u64;
    if nat_eq(a747, a748) {
        let a753 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h749, _, _, _, _) => {
                h749
            }
        };
        let a754 = {
            let a751 = 4u64;
            let a752 = match v0 {
                Adt1::C0(_, h750, _, _, _, _, _, _, _, _, _) => {
                    h750
                }
            };
            nat_mul(a751, a752)?
        };
        Ok(nat_eq(a753, a754))
    } else {
        Ok(false)
    }
}

pub fn f24(v0: Adt1, v1: u64) -> R<bool> {
    let a756 = match v0.clone() {
        Adt1::C0(_, h755, _, _, _, _, _, _, _, _, _) => {
            h755
        }
    };
    let a757 = 1u64;
    if if if if nat_eq(a756, a757) {
        true
    } else {
        let a759 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h758, _, _, _, _, _) => {
                h758
            }
        };
        let a760 = v1;
        nat_eq(a759, a760)
    } {
        let a762 = match v0.clone() {
            Adt1::C0(_, _, h761, _, _, _, _, _, _, _, _) => {
                h761
            }
        };
        let a763 = 1u64;
        if nat_eq(a762, a763) {
            true
        } else {
            let a768 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h764, _, _, _, _) => {
                    h764
                }
            };
            let a769 = {
                let a766 = v1;
                let a767 = match v0.clone() {
                    Adt1::C0(_, h765, _, _, _, _, _, _, _, _, _) => {
                        h765
                    }
                };
                nat_mul(a766, a767)?
            };
            nat_eq(a768, a769)
        }
    } else {
        false
    } {
        let a771 = match v0.clone() {
            Adt1::C0(_, _, _, h770, _, _, _, _, _, _, _) => {
                h770
            }
        };
        let a772 = 1u64;
        if nat_eq(a771, a772) {
            true
        } else {
            let a780 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h773, _, _, _) => {
                    h773
                }
            };
            let a781 = {
                let a778 = {
                    let a775 = v1;
                    let a776 = match v0.clone() {
                        Adt1::C0(_, h774, _, _, _, _, _, _, _, _, _) => {
                            h774
                        }
                    };
                    nat_mul(a775, a776)?
                };
                let a779 = match v0.clone() {
                    Adt1::C0(_, _, h777, _, _, _, _, _, _, _, _) => {
                        h777
                    }
                };
                nat_mul(a778, a779)?
            };
            nat_eq(a780, a781)
        }
    } else {
        false
    } {
        let a783 = match v0.clone() {
            Adt1::C0(_, _, _, _, h782, _, _, _, _, _, _) => {
                h782
            }
        };
        let a784 = 1u64;
        if nat_eq(a783, a784) {
            Ok(true)
        } else {
            let a795 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h785, _, _) => {
                    h785
                }
            };
            let a796 = {
                let a793 = {
                    let a790 = {
                        let a787 = v1;
                        let a788 = match v0.clone() {
                            Adt1::C0(_, h786, _, _, _, _, _, _, _, _, _) => {
                                h786
                            }
                        };
                        nat_mul(a787, a788)?
                    };
                    let a791 = match v0.clone() {
                        Adt1::C0(_, _, h789, _, _, _, _, _, _, _, _) => {
                            h789
                        }
                    };
                    nat_mul(a790, a791)?
                };
                let a794 = match v0 {
                    Adt1::C0(_, _, _, h792, _, _, _, _, _, _, _) => {
                        h792
                    }
                };
                nat_mul(a793, a794)?
            };
            Ok(nat_eq(a795, a796))
        }
    } else {
        Ok(false)
    }
}

pub fn f25(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    let a798 = match v0.clone() {
        Adt1::C0(_, h797, _, _, _, _, _, _, _, _, _) => {
            h797
        }
    };
    let a799 = v1;
    if if if nat_eq(a798, a799) {
        let a801 = match v0.clone() {
            Adt1::C0(_, _, h800, _, _, _, _, _, _, _, _) => {
                h800
            }
        };
        let a802 = v2;
        nat_eq(a801, a802)
    } else {
        false
    } {
        let a804 = match v0.clone() {
            Adt1::C0(_, _, _, h803, _, _, _, _, _, _, _) => {
                h803
            }
        };
        let a805 = v3;
        nat_eq(a804, a805)
    } else {
        false
    } {
        let a807 = match v0 {
            Adt1::C0(_, _, _, _, h806, _, _, _, _, _, _) => {
                h806
            }
        };
        let a808 = v4;
        nat_eq(a807, a808)
    } else {
        false
    }
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    let a811 = match v0.clone() {
        Adt1::C0(_, h809, _, _, _, _, _, _, _, _, _) => {
            h809
        }
    };
    let a812 = match v1.clone() {
        Adt1::C0(_, h810, _, _, _, _, _, _, _, _, _) => {
            h810
        }
    };
    if if if nat_eq(a811, a812) {
        let a815 = match v0.clone() {
            Adt1::C0(_, _, h813, _, _, _, _, _, _, _, _) => {
                h813
            }
        };
        let a816 = match v1.clone() {
            Adt1::C0(_, _, h814, _, _, _, _, _, _, _, _) => {
                h814
            }
        };
        nat_eq(a815, a816)
    } else {
        false
    } {
        let a819 = match v0.clone() {
            Adt1::C0(_, _, _, h817, _, _, _, _, _, _, _) => {
                h817
            }
        };
        let a820 = match v1.clone() {
            Adt1::C0(_, _, _, h818, _, _, _, _, _, _, _) => {
                h818
            }
        };
        nat_eq(a819, a820)
    } else {
        false
    } {
        let a823 = match v0 {
            Adt1::C0(_, _, _, _, h821, _, _, _, _, _, _) => {
                h821
            }
        };
        let a824 = match v1 {
            Adt1::C0(_, _, _, _, h822, _, _, _, _, _, _) => {
                h822
            }
        };
        nat_eq(a823, a824)
    } else {
        false
    }
}

pub fn f27(v0: Adt1, v1: Adt1) -> R<bool> {
    let a826 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h825, _) => {
            h825
        }
    };
    let a827 = 0u64;
    if nat_eq(a826, a827) {
        Ok(true)
    } else {
        let a829 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h828, _) => {
                h828
            }
        };
        let a830 = 0u64;
        if nat_eq(a829, a830) {
            Ok(true)
        } else {
            let a832 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h831) => {
                    h831
                }
            };
            let a833 = 0u64;
            if nat_eq(a832, a833) {
                Ok(true)
            } else {
                let a835 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h834) => {
                        h834
                    }
                };
                let a836 = 0u64;
                if nat_eq(a835, a836) {
                    Ok(true)
                } else {
                    let a842 = {
                        let a839 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h837) => {
                                h837
                            }
                        };
                        let a840 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h838, _) => {
                                h838
                            }
                        };
                        nat_add(a839, a840)?
                    };
                    let a843 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h841) => {
                            h841
                        }
                    };
                    if nat_le(a842, a843) {
                        Ok(true)
                    } else {
                        let a849 = {
                            let a846 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h844) => {
                                    h844
                                }
                            };
                            let a847 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h845, _) => {
                                    h845
                                }
                            };
                            nat_add(a846, a847)?
                        };
                        let a850 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h848) => {
                                h848
                            }
                        };
                        Ok(nat_le(a849, a850))
                    }
                }
            }
        }
    }
}

pub fn f28(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h851, _, _, _, _, _, _, _, _, _) => {
            h851
        }
    };
    let v5: u64 = match v0.clone() {
        Adt1::C0(_, _, h852, _, _, _, _, _, _, _, _) => {
            h852
        }
    };
    let v6: u64 = match v0.clone() {
        Adt1::C0(_, _, _, h853, _, _, _, _, _, _, _) => {
            h853
        }
    };
    let v7: u64 = match v0 {
        Adt1::C0(_, _, _, _, h854, _, _, _, _, _, _) => {
            h854
        }
    };
    let a875 = 4u64;
    let a876 = {
        let a873 = {
            let a859 = {
                let a855 = v4;
                let a856 = v5;
                nat_mul(a855, a856)?
            };
            let a860 = {
                let a857 = v6;
                let a858 = v7;
                nat_mul(a857, a858)?
            };
            nat_mul(a859, a860)?
        };
        let a874 = {
            let a871 = {
                let a861 = v4;
                let a862 = v4;
                nat_mul(a861, a862)?
            };
            let a872 = {
                let a869 = v1;
                let a870 = {
                    let a867 = v5;
                    let a868 = {
                        let a865 = v2;
                        let a866 = {
                            let a863 = v7;
                            let a864 = v3;
                            nat_mul(a863, a864)?
                        };
                        nat_add(a865, a866)?
                    };
                    nat_mul(a867, a868)?
                };
                nat_add(a869, a870)?
            };
            nat_mul(a871, a872)?
        };
        nat_add(a873, a874)?
    };
    nat_mul(a875, a876)
}

pub fn f29(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt7> {
    let v8: u64 = match v3.clone() {
        Adt1::C0(_, h877, _, _, _, _, _, _, _, _, _) => {
            h877
        }
    };
    let v9: u64 = match v3.clone() {
        Adt1::C0(_, _, h878, _, _, _, _, _, _, _, _) => {
            h878
        }
    };
    let v10: u64 = match v3.clone() {
        Adt1::C0(_, _, _, h879, _, _, _, _, _, _, _) => {
            h879
        }
    };
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, _, _, _, h880, _, _, _, _, _, _) => {
            h880
        }
    };
    let a881 = 0u64;
    let a882 = v8;
    if if if if if if if if if if if if if if if if if if if if if if if nat_lt(a881, a882) {
        let a883 = 0u64;
        let a884 = v9;
        nat_lt(a883, a884)
    } else {
        false
    } {
        let a885 = 0u64;
        let a886 = v11;
        nat_lt(a885, a886)
    } else {
        false
    } {
        let a887 = 0u64;
        let a888 = v0;
        nat_lt(a887, a888)
    } else {
        false
    } {
        let a889 = v0;
        let a890 = 2147483647u64;
        nat_le(a889, a890)
    } else {
        false
    } {
        f22(v1.clone(), v7.clone())?
    } else {
        false
    } {
        f22(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f22(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f22(v4.clone(), v7.clone())?
    } else {
        false
    } {
        f22(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f22(v6.clone(), v7.clone())?
    } else {
        false
    } {
        f48(v1.clone(), v3.clone())
    } else {
        false
    } {
        f48(v2.clone(), v3.clone())
    } else {
        false
    } {
        let a892 = match v4.clone() {
            Adt1::C0(_, h891, _, _, _, _, _, _, _, _, _) => {
                h891
            }
        };
        let a893 = 1u64;
        if nat_eq(a892, a893) {
            true
        } else {
            let a895 = match v4.clone() {
                Adt1::C0(_, h894, _, _, _, _, _, _, _, _, _) => {
                    h894
                }
            };
            let a896 = v8;
            nat_eq(a895, a896)
        }
    } else {
        false
    } {
        let a899 = match v4.clone() {
            Adt1::C0(_, h897, _, _, _, _, _, _, _, _, _) => {
                h897
            }
        };
        let a900 = match v4.clone() {
            Adt1::C0(_, h898, _, _, _, _, _, _, _, _, _) => {
                h898
            }
        };
        if if if nat_eq(a899, a900) {
            let a902 = match v4.clone() {
                Adt1::C0(_, _, h901, _, _, _, _, _, _, _, _) => {
                    h901
                }
            };
            let a903 = v9;
            nat_eq(a902, a903)
        } else {
            false
        } {
            let a905 = match v4.clone() {
                Adt1::C0(_, _, _, h904, _, _, _, _, _, _, _) => {
                    h904
                }
            };
            let a906 = v10;
            nat_eq(a905, a906)
        } else {
            false
        } {
            let a908 = match v4.clone() {
                Adt1::C0(_, _, _, _, h907, _, _, _, _, _, _) => {
                    h907
                }
            };
            let a909 = v11;
            nat_eq(a908, a909)
        } else {
            false
        }
    } else {
        false
    } {
        let a911 = match v5.clone() {
            Adt1::C0(_, h910, _, _, _, _, _, _, _, _, _) => {
                h910
            }
        };
        let a912 = 1u64;
        if if if nat_eq(a911, a912) {
            let a914 = match v5.clone() {
                Adt1::C0(_, _, h913, _, _, _, _, _, _, _, _) => {
                    h913
                }
            };
            let a915 = v9;
            nat_eq(a914, a915)
        } else {
            false
        } {
            let a917 = match v5.clone() {
                Adt1::C0(_, _, _, h916, _, _, _, _, _, _, _) => {
                    h916
                }
            };
            let a918 = v10;
            nat_eq(a917, a918)
        } else {
            false
        } {
            let a920 = match v5.clone() {
                Adt1::C0(_, _, _, _, h919, _, _, _, _, _, _) => {
                    h919
                }
            };
            let a921 = v11;
            nat_eq(a920, a921)
        } else {
            false
        }
    } else {
        false
    } {
        let a923 = match v6.clone() {
            Adt1::C0(_, h922, _, _, _, _, _, _, _, _, _) => {
                h922
            }
        };
        let a924 = v8;
        if if if nat_eq(a923, a924) {
            let a926 = match v6.clone() {
                Adt1::C0(_, _, h925, _, _, _, _, _, _, _, _) => {
                    h925
                }
            };
            let a927 = v8;
            nat_eq(a926, a927)
        } else {
            false
        } {
            let a929 = match v6.clone() {
                Adt1::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                    h928
                }
            };
            let a930 = v9;
            nat_eq(a929, a930)
        } else {
            false
        } {
            let a932 = match v6.clone() {
                Adt1::C0(_, _, _, _, h931, _, _, _, _, _, _) => {
                    h931
                }
            };
            let a933 = v11;
            nat_eq(a932, a933)
        } else {
            false
        }
    } else {
        false
    } {
        let a937 = match v7.clone() {
            Adt1::C0(_, h934, _, _, _, _, _, _, _, _, _) => {
                h934
            }
        };
        let a938 = {
            let a935 = v8;
            let a936 = v9;
            nat_mul(a935, a936)?
        };
        if if if nat_eq(a937, a938) {
            let a948 = match v7.clone() {
                Adt1::C0(_, _, h939, _, _, _, _, _, _, _, _) => {
                    h939
                }
            };
            let a949 = {
                let a946 = {
                    let a940 = v10;
                    let a941 = v11;
                    nat_mul(a940, a941)?
                };
                let a947 = {
                    let a944 = {
                        let a942 = v0;
                        let a943 = v8;
                        nat_mul(a942, a943)?
                    };
                    let a945 = v11;
                    nat_mul(a944, a945)?
                };
                nat_add(a946, a947)?
            };
            nat_eq(a948, a949)
        } else {
            false
        } {
            let a951 = match v7.clone() {
                Adt1::C0(_, _, _, h950, _, _, _, _, _, _, _) => {
                    h950
                }
            };
            let a952 = 1u64;
            nat_eq(a951, a952)
        } else {
            false
        } {
            let a954 = match v7.clone() {
                Adt1::C0(_, _, _, _, h953, _, _, _, _, _, _) => {
                    h953
                }
            };
            let a955 = 1u64;
            nat_eq(a954, a955)
        } else {
            false
        }
    } else {
        false
    } {
        let a957 = match v7.clone() {
            Adt1::C0(h956, _, _, _, _, _, _, _, _, _, _) => {
                h956
            }
        };
        let a958 = 0u64;
        nat_eq(a957, a958)
    } else {
        false
    } {
        f24(v4.clone(), 4u64)?
    } else {
        false
    } {
        f24(v5.clone(), 4u64)?
    } else {
        false
    } {
        f24(v6.clone(), 4u64)?
    } else {
        false
    } {
        f24(v7, 4u64)?
    } else {
        false
    } {
        let v12: u64 = {
            let a959 = v9;
            let a960 = v11;
            nat_mul(a959, a960)?
        };
        let v13: u64 = {
            let a961 = v8;
            let a962 = v8;
            nat_mul(a961, a962)?
        };
        let v14: u64 = {
            let a963 = v10;
            let a964 = 0u64;
            if if nat_eq(a963, a964) {
                let a965 = v0;
                let a966 = 1u64;
                nat_eq(a965, a966)
            } else {
                false
            } {
                1u64
            } else {
                let a967 = v0;
                let a968 = v10;
                if nat_lt(a967, a968) {
                    v0
                } else {
                    v10
                }
            }
        };
        let v15: u64 = {
            let a975 = v12;
            let a976 = {
                let a973 = {
                    let a969 = v10;
                    let a970 = v8;
                    nat_mul(a969, a970)?
                };
                let a974 = {
                    let a971 = v14;
                    let a972 = v13;
                    nat_mul(a971, a972)?
                };
                nat_add(a973, a974)?
            };
            nat_mul(a975, a976)?
        };
        Ok(Adt7::C12(Adt6::C0(v1, v2, v3, v4, v5, v6, v0, v12, v13, {
            let a979 = {
                let a977 = v13;
                let a978 = v8;
                nat_add(a977, a978)?
            };
            let a980 = v8;
            nat_add(a979, a980)?
        }, v15, false)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f30(v0: bool, v1: Adt1, v2: Adt1, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Adt1) -> R<Adt7> {
    let v8: Adt1 = {
        let m981 = v3.clone();
        match m981 {
            None => {
                v1.clone()
            }
            Some(v7) => {
                v7
            }
        }
    };
    let v10: Adt1 = {
        let m982 = v4.clone();
        match m982 {
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
        let m983 = v5;
        match m983 {
            None => {
                let m984 = v3;
                match m984 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m985 = v4;
                        match m985 {
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
        let a987 = match v1.clone() {
            Adt1::C0(h986, _, _, _, _, _, _, _, _, _, _) => {
                h986
            }
        };
        let a988 = 0u64;
        if nat_eq(a987, a988) {
            let a990 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h989, _, _, _, _, _) => {
                    h989
                }
            };
            let a991 = 4u64;
            nat_eq(a990, a991)
        } else {
            false
        }
    } else {
        false
    } {
        let a993 = match v8.clone() {
            Adt1::C0(h992, _, _, _, _, _, _, _, _, _, _) => {
                h992
            }
        };
        let a994 = 0u64;
        if nat_eq(a993, a994) {
            let a996 = match v8.clone() {
                Adt1::C0(_, _, _, _, _, h995, _, _, _, _, _) => {
                    h995
                }
            };
            let a997 = 4u64;
            nat_eq(a996, a997)
        } else {
            false
        }
    } else {
        false
    } {
        let a999 = match v10.clone() {
            Adt1::C0(h998, _, _, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        let a1000 = 1u64;
        if nat_eq(a999, a1000) {
            let a1002 = match v10.clone() {
                Adt1::C0(_, _, _, _, _, h1001, _, _, _, _, _) => {
                    h1001
                }
            };
            let a1003 = 2u64;
            nat_eq(a1002, a1003)
        } else {
            false
        }
    } else {
        false
    } {
        f33(v6.clone())?
    } else {
        false
    } {
        f49(match v2.clone() {
            Adt1::C0(h1004, _, _, _, _, _, _, _, _, _, _) => {
                h1004
            }
        }, match v2.clone() {
            Adt1::C0(_, h1005, _, _, _, _, _, _, _, _, _) => {
                h1005
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h1006, _, _, _, _, _) => {
                h1006
            }
        })
    } else {
        false
    } {
        let a1008 = 0u64;
        let a1009 = match v1.clone() {
            Adt1::C0(_, h1007, _, _, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        nat_lt(a1008, a1009)
    } else {
        false
    } {
        let a1011 = 0u64;
        let a1012 = match v1.clone() {
            Adt1::C0(_, _, h1010, _, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        nat_lt(a1011, a1012)
    } else {
        false
    } {
        let a1014 = 0u64;
        let a1015 = match v1.clone() {
            Adt1::C0(_, _, _, h1013, _, _, _, _, _, _, _) => {
                h1013
            }
        };
        nat_lt(a1014, a1015)
    } else {
        false
    } {
        let a1017 = 0u64;
        let a1018 = match v1.clone() {
            Adt1::C0(_, _, _, _, h1016, _, _, _, _, _, _) => {
                h1016
            }
        };
        nat_lt(a1017, a1018)
    } else {
        false
    } {
        let a1020 = 0u64;
        let a1021 = match v2.clone() {
            Adt1::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
                h1019
            }
        };
        nat_lt(a1020, a1021)
    } else {
        false
    } {
        let a1024 = match v2.clone() {
            Adt1::C0(_, h1022, _, _, _, _, _, _, _, _, _) => {
                h1022
            }
        };
        let a1025 = match v1.clone() {
            Adt1::C0(_, h1023, _, _, _, _, _, _, _, _, _) => {
                h1023
            }
        };
        nat_eq(a1024, a1025)
    } else {
        false
    } {
        let a1027 = match v2.clone() {
            Adt1::C0(_, _, h1026, _, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1028 = 1u64;
        nat_eq(a1027, a1028)
    } else {
        false
    } {
        let a1031 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1029, _, _, _, _, _, _) => {
                h1029
            }
        };
        let a1032 = match v1.clone() {
            Adt1::C0(_, _, _, _, h1030, _, _, _, _, _, _) => {
                h1030
            }
        };
        nat_eq(a1031, a1032)
    } else {
        false
    } {
        let a1035 = match v8.clone() {
            Adt1::C0(_, h1033, _, _, _, _, _, _, _, _, _) => {
                h1033
            }
        };
        let a1036 = match v1.clone() {
            Adt1::C0(_, _, h1034, _, _, _, _, _, _, _, _) => {
                h1034
            }
        };
        nat_eq(a1035, a1036)
    } else {
        false
    } {
        let a1039 = match v8.clone() {
            Adt1::C0(_, _, h1037, _, _, _, _, _, _, _, _) => {
                h1037
            }
        };
        let a1040 = match v1.clone() {
            Adt1::C0(_, _, _, h1038, _, _, _, _, _, _, _) => {
                h1038
            }
        };
        nat_eq(a1039, a1040)
    } else {
        false
    } {
        let a1042 = match v8.clone() {
            Adt1::C0(_, _, _, h1041, _, _, _, _, _, _, _) => {
                h1041
            }
        };
        let a1043 = 1u64;
        nat_eq(a1042, a1043)
    } else {
        false
    } {
        let a1046 = match v8.clone() {
            Adt1::C0(_, _, _, _, h1044, _, _, _, _, _, _) => {
                h1044
            }
        };
        let a1047 = match v1.clone() {
            Adt1::C0(_, _, _, _, h1045, _, _, _, _, _, _) => {
                h1045
            }
        };
        nat_eq(a1046, a1047)
    } else {
        false
    } {
        let a1050 = match v10.clone() {
            Adt1::C0(_, h1048, _, _, _, _, _, _, _, _, _) => {
                h1048
            }
        };
        let a1051 = match v2.clone() {
            Adt1::C0(_, _, _, h1049, _, _, _, _, _, _, _) => {
                h1049
            }
        };
        nat_eq(a1050, a1051)
    } else {
        false
    } {
        let a1054 = match v10.clone() {
            Adt1::C0(_, _, h1052, _, _, _, _, _, _, _, _) => {
                h1052
            }
        };
        let a1055 = match v1.clone() {
            Adt1::C0(_, _, _, h1053, _, _, _, _, _, _, _) => {
                h1053
            }
        };
        nat_eq(a1054, a1055)
    } else {
        false
    } {
        let a1057 = match v10.clone() {
            Adt1::C0(_, _, _, h1056, _, _, _, _, _, _, _) => {
                h1056
            }
        };
        let a1058 = 1u64;
        nat_eq(a1057, a1058)
    } else {
        false
    } {
        f21(match v1.clone() {
            Adt1::C0(_, _, _, _, h1059, _, _, _, _, _, _) => {
                h1059
            }
        }, match v10.clone() {
            Adt1::C0(_, _, _, _, h1060, _, _, _, _, _, _) => {
                h1060
            }
        })
    } else {
        false
    } {
        let a1063 = match v6.clone() {
            Adt1::C0(_, h1061, _, _, _, _, _, _, _, _, _) => {
                h1061
            }
        };
        let a1064 = match v2.clone() {
            Adt1::C0(_, _, _, h1062, _, _, _, _, _, _, _) => {
                h1062
            }
        };
        nat_eq(a1063, a1064)
    } else {
        false
    } {
        let a1067 = match v6.clone() {
            Adt1::C0(_, _, h1065, _, _, _, _, _, _, _, _) => {
                h1065
            }
        };
        let a1068 = match v1.clone() {
            Adt1::C0(_, _, _, h1066, _, _, _, _, _, _, _) => {
                h1066
            }
        };
        nat_eq(a1067, a1068)
    } else {
        false
    } {
        let a1070 = match v6.clone() {
            Adt1::C0(_, _, _, h1069, _, _, _, _, _, _, _) => {
                h1069
            }
        };
        let a1071 = 1u64;
        nat_eq(a1070, a1071)
    } else {
        false
    } {
        let a1074 = match v6.clone() {
            Adt1::C0(_, _, _, _, h1072, _, _, _, _, _, _) => {
                h1072
            }
        };
        let a1075 = match v1.clone() {
            Adt1::C0(_, _, _, _, h1073, _, _, _, _, _, _) => {
                h1073
            }
        };
        nat_eq(a1074, a1075)
    } else {
        false
    } {
        f27(v1.clone(), v6.clone())?
    } else {
        false
    } {
        f27(v2.clone(), v6.clone())?
    } else {
        false
    } {
        f27(v8.clone(), v6.clone())?
    } else {
        false
    } {
        f27(v10.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt7::C10(Adt5::C0(v1, v2, v8, v10, v6)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f31(v0: Option<Adt1>, v1: Option<Adt1>, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Option<u64>> {
    let m1076 = v0;
    match m1076 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h1077, _, _, _, _, _, _, _, _) => {
                    h1077
                }
            }))
        }
        Some(v6) => {
            let m1078 = v1;
            match m1078 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a1080 = match v6.clone() {
                        Adt1::C0(h1079, _, _, _, _, _, _, _, _, _, _) => {
                            h1079
                        }
                    };
                    let a1081 = 26u64;
                    if if if if if if if if if nat_eq(a1080, a1081) {
                        f24(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a1084 = match v6.clone() {
                            Adt1::C0(_, h1082, _, _, _, _, _, _, _, _, _) => {
                                h1082
                            }
                        };
                        let a1085 = match v7 {
                            Adt1::C0(_, h1083, _, _, _, _, _, _, _, _, _) => {
                                h1083
                            }
                        };
                        nat_eq(a1084, a1085)
                    } else {
                        false
                    } {
                        let a1088 = match v6.clone() {
                            Adt1::C0(_, _, h1086, _, _, _, _, _, _, _, _) => {
                                h1086
                            }
                        };
                        let a1089 = match v2 {
                            Adt1::C0(_, _, _, _, h1087, _, _, _, _, _, _) => {
                                h1087
                            }
                        };
                        nat_eq(a1088, a1089)
                    } else {
                        false
                    } {
                        let a1091 = match v6.clone() {
                            Adt1::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
                                h1090
                            }
                        };
                        let a1092 = 1u64;
                        nat_eq(a1091, a1092)
                    } else {
                        false
                    } {
                        let a1094 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h1093, _, _, _, _, _, _) => {
                                h1093
                            }
                        };
                        let a1095 = 1u64;
                        nat_eq(a1094, a1095)
                    } else {
                        false
                    } {
                        let a1097 = match v3 {
                            Adt1::C0(_, _, _, _, h1096, _, _, _, _, _, _) => {
                                h1096
                            }
                        };
                        let a1098 = 1u64;
                        nat_eq(a1097, a1098)
                    } else {
                        false
                    } {
                        let a1100 = match v4 {
                            Adt1::C0(_, _, _, _, h1099, _, _, _, _, _, _) => {
                                h1099
                            }
                        };
                        let a1101 = 1u64;
                        nat_eq(a1100, a1101)
                    } else {
                        false
                    } {
                        f27(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h1102, _, _, _, _, _, _, _, _, _) => {
                                h1102
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

pub fn f32(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a1103 = v0;
    let a1104 = 0u64;
    if if nat_eq(a1103, a1104) {
        true
    } else {
        let a1105 = 2147483648u64;
        let a1106 = v0;
        if nat_le(a1105, a1106) {
            let a1107 = v0;
            let a1108 = 4286578688u64;
            nat_lt(a1107, a1108)
        } else {
            false
        }
    } {
        true
    } else {
        let a1109 = v0;
        let a1110 = 2139095040u64;
        if if nat_lt(a1109, a1110) {
            let a1111 = v1;
            let a1112 = 4294967295u64;
            nat_le(a1111, a1112)
        } else {
            false
        } {
            let m1113 = v2;
            match m1113 {
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

pub fn f33(v0: Adt1) -> R<bool> {
    let a1115 = match v0.clone() {
        Adt1::C0(h1114, _, _, _, _, _, _, _, _, _, _) => {
            h1114
        }
    };
    let a1116 = 0u64;
    if if if if nat_eq(a1115, a1116) {
        let a1118 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1117, _, _, _, _, _) => {
                h1117
            }
        };
        let a1119 = 4u64;
        nat_eq(a1118, a1119)
    } else {
        false
    } {
        let a1125 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1120, _, _, _, _) => {
                h1120
            }
        };
        let a1126 = {
            let a1123 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1121, _, _, _, _, _) => {
                    h1121
                }
            };
            let a1124 = match v0.clone() {
                Adt1::C0(_, h1122, _, _, _, _, _, _, _, _, _) => {
                    h1122
                }
            };
            nat_mul(a1123, a1124)?
        };
        nat_eq(a1125, a1126)
    } else {
        false
    } {
        let a1132 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1127, _, _, _) => {
                h1127
            }
        };
        let a1133 = {
            let a1130 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1128, _, _, _, _) => {
                    h1128
                }
            };
            let a1131 = match v0.clone() {
                Adt1::C0(_, _, h1129, _, _, _, _, _, _, _, _) => {
                    h1129
                }
            };
            nat_mul(a1130, a1131)?
        };
        nat_eq(a1132, a1133)
    } else {
        false
    } {
        let a1139 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1134, _, _) => {
                h1134
            }
        };
        let a1140 = {
            let a1137 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1135, _, _, _) => {
                    h1135
                }
            };
            let a1138 = match v0 {
                Adt1::C0(_, _, _, h1136, _, _, _, _, _, _, _) => {
                    h1136
                }
            };
            nat_mul(a1137, a1138)?
        };
        Ok(nat_eq(a1139, a1140))
    } else {
        Ok(false)
    }
}

pub fn f34(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f50(match v0.clone() {
        Adt1::C0(h1141, _, _, _, _, _, _, _, _, _, _) => {
            h1141
        }
    }, match v0.clone() {
        Adt1::C0(_, h1142, _, _, _, _, _, _, _, _, _) => {
            h1142
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1143, _, _, _, _, _) => {
            h1143
        }
    }) {
        f21(match v1.clone() {
            Adt1::C0(_, _, _, h1144, _, _, _, _, _, _, _) => {
                h1144
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h1145, _, _, _, _, _, _, _) => {
                h1145
            }
        })
    } else {
        false
    } {
        f21(match v1 {
            Adt1::C0(_, _, _, _, h1146, _, _, _, _, _, _) => {
                h1146
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h1147, _, _, _, _, _, _) => {
                h1147
            }
        })
    } else {
        false
    } {
        f27(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f35(v0: Option<Adt1>, v1: Adt1, v2: u64, v3: Adt1) -> R<bool> {
    let m1148 = v0;
    match m1148 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a1150 = match v4.clone() {
                Adt1::C0(h1149, _, _, _, _, _, _, _, _, _, _) => {
                    h1149
                }
            };
            let a1151 = 1u64;
            if if if if if if nat_eq(a1150, a1151) {
                let a1153 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h1152, _, _, _, _, _) => {
                        h1152
                    }
                };
                let a1154 = 2u64;
                nat_eq(a1153, a1154)
            } else {
                false
            } {
                let a1156 = v2;
                let a1157 = match v4.clone() {
                    Adt1::C0(_, h1155, _, _, _, _, _, _, _, _, _) => {
                        h1155
                    }
                };
                nat_le(a1156, a1157)
            } else {
                false
            } {
                let a1160 = match v1.clone() {
                    Adt1::C0(_, _, h1158, _, _, _, _, _, _, _, _) => {
                        h1158
                    }
                };
                let a1161 = match v4.clone() {
                    Adt1::C0(_, _, h1159, _, _, _, _, _, _, _, _) => {
                        h1159
                    }
                };
                nat_le(a1160, a1161)
            } else {
                false
            } {
                f21(match v1.clone() {
                    Adt1::C0(_, _, _, h1162, _, _, _, _, _, _, _) => {
                        h1162
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h1163, _, _, _, _, _, _, _) => {
                        h1163
                    }
                })
            } else {
                false
            } {
                f21(match v1 {
                    Adt1::C0(_, _, _, _, h1164, _, _, _, _, _, _) => {
                        h1164
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h1165, _, _, _, _, _, _) => {
                        h1165
                    }
                })
            } else {
                false
            } {
                f27(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f36(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1166 = v0;
    match m1166 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1168 = match v3.clone() {
                Adt1::C0(h1167, _, _, _, _, _, _, _, _, _, _) => {
                    h1167
                }
            };
            let a1169 = 0u64;
            if if if if if if nat_eq(a1168, a1169) {
                let a1171 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h1170, _, _, _, _, _) => {
                        h1170
                    }
                };
                let a1172 = 4u64;
                nat_eq(a1171, a1172)
            } else {
                false
            } {
                let a1175 = match v3.clone() {
                    Adt1::C0(_, h1173, _, _, _, _, _, _, _, _, _) => {
                        h1173
                    }
                };
                let a1176 = match v1 {
                    Adt1::C0(_, _, _, h1174, _, _, _, _, _, _, _) => {
                        h1174
                    }
                };
                nat_eq(a1175, a1176)
            } else {
                false
            } {
                let a1178 = match v3.clone() {
                    Adt1::C0(_, _, h1177, _, _, _, _, _, _, _, _) => {
                        h1177
                    }
                };
                let a1179 = 1u64;
                nat_eq(a1178, a1179)
            } else {
                false
            } {
                let a1181 = match v3.clone() {
                    Adt1::C0(_, _, _, h1180, _, _, _, _, _, _, _) => {
                        h1180
                    }
                };
                let a1182 = 1u64;
                nat_eq(a1181, a1182)
            } else {
                false
            } {
                let a1184 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1183, _, _, _, _, _, _) => {
                        h1183
                    }
                };
                let a1185 = 1u64;
                nat_eq(a1184, a1185)
            } else {
                false
            } {
                f27(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f37(v0: u64) -> Option<u64> {
    let a1186 = 0u64;
    let a1187 = v0;
    if if nat_lt(a1186, a1187) {
        let a1188 = v0;
        let a1189 = 2139095040u64;
        nat_lt(a1188, a1189)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f38(v0: u64) -> Option<u64> {
    let a1190 = v0;
    let a1191 = 0u64;
    if if nat_eq(a1190, a1191) {
        true
    } else {
        let a1192 = v0;
        let a1193 = 2147483648u64;
        nat_eq(a1192, a1193)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f39(v0: Adt1, v1: Adt1) -> bool {
    let a1196 = match v0.clone() {
        Adt1::C0(_, _, h1194, _, _, _, _, _, _, _, _) => {
            h1194
        }
    };
    let a1197 = match v1.clone() {
        Adt1::C0(_, _, h1195, _, _, _, _, _, _, _, _) => {
            h1195
        }
    };
    if if nat_eq(a1196, a1197) {
        let a1200 = match v0.clone() {
            Adt1::C0(_, _, _, h1198, _, _, _, _, _, _, _) => {
                h1198
            }
        };
        let a1201 = match v1.clone() {
            Adt1::C0(_, _, _, h1199, _, _, _, _, _, _, _) => {
                h1199
            }
        };
        nat_eq(a1200, a1201)
    } else {
        false
    } {
        let a1204 = match v0 {
            Adt1::C0(_, _, _, _, h1202, _, _, _, _, _, _) => {
                h1202
            }
        };
        let a1205 = match v1 {
            Adt1::C0(_, _, _, _, h1203, _, _, _, _, _, _) => {
                h1203
            }
        };
        nat_eq(a1204, a1205)
    } else {
        false
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f51(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f51(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f51(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f51(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f51(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f51(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f26(v0.clone(), v1.clone()) {
        let a1208 = match v0.clone() {
            Adt1::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                h1206
            }
        };
        let a1209 = match v1.clone() {
            Adt1::C0(h1207, _, _, _, _, _, _, _, _, _, _) => {
                h1207
            }
        };
        nat_eq(a1208, a1209)
    } else {
        false
    } {
        let a1212 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1210) => {
                h1210
            }
        };
        let a1213 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1211) => {
                h1211
            }
        };
        nat_eq(a1212, a1213)
    } else {
        false
    } {
        let a1216 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1214, _, _, _, _, _) => {
                h1214
            }
        };
        let a1217 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1215, _, _, _, _, _) => {
                h1215
            }
        };
        nat_eq(a1216, a1217)
    } else {
        false
    } {
        let a1220 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1218, _, _, _, _) => {
                h1218
            }
        };
        let a1221 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1219, _, _, _, _) => {
                h1219
            }
        };
        nat_eq(a1220, a1221)
    } else {
        false
    } {
        let a1224 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1222, _, _, _) => {
                h1222
            }
        };
        let a1225 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1223, _, _, _) => {
                h1223
            }
        };
        nat_eq(a1224, a1225)
    } else {
        false
    } {
        let a1228 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1226, _, _) => {
                h1226
            }
        };
        let a1229 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1227, _, _) => {
                h1227
            }
        };
        nat_eq(a1228, a1229)
    } else {
        false
    }
}

pub fn f42(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1230 = v0;
    match m1230 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1232 = match v3.clone() {
                Adt1::C0(h1231, _, _, _, _, _, _, _, _, _, _) => {
                    h1231
                }
            };
            let a1233 = 0u64;
            if if if if if if if nat_eq(a1232, a1233) {
                true
            } else {
                let a1235 = match v3.clone() {
                    Adt1::C0(h1234, _, _, _, _, _, _, _, _, _, _) => {
                        h1234
                    }
                };
                let a1236 = 1u64;
                nat_eq(a1235, a1236)
            } {
                f24(v3.clone(), {
                    let a1238 = match v3.clone() {
                        Adt1::C0(h1237, _, _, _, _, _, _, _, _, _, _) => {
                            h1237
                        }
                    };
                    let a1239 = 0u64;
                    if nat_eq(a1238, a1239) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a1242 = match v3.clone() {
                    Adt1::C0(_, h1240, _, _, _, _, _, _, _, _, _) => {
                        h1240
                    }
                };
                let a1243 = match v1.clone() {
                    Adt1::C0(_, h1241, _, _, _, _, _, _, _, _, _) => {
                        h1241
                    }
                };
                nat_eq(a1242, a1243)
            } else {
                false
            } {
                let a1246 = match v1.clone() {
                    Adt1::C0(_, _, h1244, _, _, _, _, _, _, _, _) => {
                        h1244
                    }
                };
                let a1247 = match v3.clone() {
                    Adt1::C0(_, _, h1245, _, _, _, _, _, _, _, _) => {
                        h1245
                    }
                };
                nat_le(a1246, a1247)
            } else {
                false
            } {
                f21(match v1.clone() {
                    Adt1::C0(_, _, _, h1248, _, _, _, _, _, _, _) => {
                        h1248
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h1249, _, _, _, _, _, _, _) => {
                        h1249
                    }
                })
            } else {
                false
            } {
                f21(match v1 {
                    Adt1::C0(_, _, _, _, h1250, _, _, _, _, _, _) => {
                        h1250
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h1251, _, _, _, _, _, _) => {
                        h1251
                    }
                })
            } else {
                false
            } {
                f27(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f43(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1252 = v0;
    match m1252 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1254 = match v3.clone() {
                Adt1::C0(h1253, _, _, _, _, _, _, _, _, _, _) => {
                    h1253
                }
            };
            let a1255 = 0u64;
            if if if if if if nat_eq(a1254, a1255) {
                let a1257 = match v3.clone() {
                    Adt1::C0(_, h1256, _, _, _, _, _, _, _, _, _) => {
                        h1256
                    }
                };
                let a1258 = 1u64;
                if nat_eq(a1257, a1258) {
                    true
                } else {
                    let a1260 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h1259, _, _, _, _, _) => {
                            h1259
                        }
                    };
                    let a1261 = 4u64;
                    nat_eq(a1260, a1261)
                }
            } else {
                false
            } {
                let a1264 = match v3.clone() {
                    Adt1::C0(_, h1262, _, _, _, _, _, _, _, _, _) => {
                        h1262
                    }
                };
                let a1265 = match v1 {
                    Adt1::C0(_, _, _, h1263, _, _, _, _, _, _, _) => {
                        h1263
                    }
                };
                nat_eq(a1264, a1265)
            } else {
                false
            } {
                let a1267 = match v3.clone() {
                    Adt1::C0(_, _, h1266, _, _, _, _, _, _, _, _) => {
                        h1266
                    }
                };
                let a1268 = 1u64;
                nat_eq(a1267, a1268)
            } else {
                false
            } {
                let a1270 = match v3.clone() {
                    Adt1::C0(_, _, _, h1269, _, _, _, _, _, _, _) => {
                        h1269
                    }
                };
                let a1271 = 1u64;
                nat_eq(a1270, a1271)
            } else {
                false
            } {
                let a1273 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1272, _, _, _, _, _, _) => {
                        h1272
                    }
                };
                let a1274 = 1u64;
                nat_eq(a1273, a1274)
            } else {
                false
            } {
                f27(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f44(v0: u64) -> u64 {
    if f47(v0) {
        15u64
    } else {
        if if f52(v0) {
            true
        } else {
            f46(v0)
        } {
            8u64
        } else {
            if f53(v0) {
                9u64
            } else {
                f54(v0)
            }
        }
    }
}

pub fn f45(v0: u64) -> bool {
    if f52(v0) {
        true
    } else {
        f53(v0)
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
    let a1279 = 10u64;
    let a1280 = v0;
    if if if if if nat_le(a1279, a1280) {
        let a1281 = v0;
        let a1282 = 14u64;
        nat_le(a1281, a1282)
    } else {
        false
    } {
        true
    } else {
        let a1283 = 16u64;
        let a1284 = v0;
        if nat_le(a1283, a1284) {
            let a1285 = v0;
            let a1286 = 19u64;
            nat_le(a1285, a1286)
        } else {
            false
        }
    } {
        true
    } else {
        let a1287 = 21u64;
        let a1288 = v0;
        if nat_le(a1287, a1288) {
            let a1289 = v0;
            let a1290 = 23u64;
            nat_le(a1289, a1290)
        } else {
            false
        }
    } {
        true
    } else {
        let a1291 = 34u64;
        let a1292 = v0;
        if nat_le(a1291, a1292) {
            let a1293 = v0;
            let a1294 = 35u64;
            nat_le(a1293, a1294)
        } else {
            false
        }
    } {
        true
    } else {
        let a1295 = v0;
        let a1296 = 29u64;
        nat_eq(a1295, a1296)
    }
}

pub fn f48(v0: Adt1, v1: Adt1) -> bool {
    let a1299 = match v0.clone() {
        Adt1::C0(_, h1297, _, _, _, _, _, _, _, _, _) => {
            h1297
        }
    };
    let a1300 = match v1.clone() {
        Adt1::C0(_, h1298, _, _, _, _, _, _, _, _, _) => {
            h1298
        }
    };
    if if if nat_eq(a1299, a1300) {
        let a1303 = match v0.clone() {
            Adt1::C0(_, _, _, h1301, _, _, _, _, _, _, _) => {
                h1301
            }
        };
        let a1304 = match v1.clone() {
            Adt1::C0(_, _, _, h1302, _, _, _, _, _, _, _) => {
                h1302
            }
        };
        nat_eq(a1303, a1304)
    } else {
        false
    } {
        let a1306 = 0u64;
        let a1307 = match v0.clone() {
            Adt1::C0(_, _, h1305, _, _, _, _, _, _, _, _) => {
                h1305
            }
        };
        nat_lt(a1306, a1307)
    } else {
        false
    } {
        f21(match v1 {
            Adt1::C0(_, _, _, _, h1308, _, _, _, _, _, _) => {
                h1308
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, h1309, _, _, _, _, _, _) => {
                h1309
            }
        })
    } else {
        false
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> bool {
    let m1310 = f17(v0);
    match m1310 {
        Adt8::C0 => {
            false
        }
        Adt8::C1(v3) => {
            let a1311 = v0;
            let a1312 = 0u64;
            if if if if if if nat_eq(a1311, a1312) {
                true
            } else {
                let a1313 = v0;
                let a1314 = 1u64;
                nat_eq(a1313, a1314)
            } {
                true
            } else {
                let a1315 = v0;
                let a1316 = 30u64;
                nat_eq(a1315, a1316)
            } {
                true
            } else {
                let a1317 = v0;
                let a1318 = 43u64;
                nat_eq(a1317, a1318)
            } {
                true
            } else {
                f18(v0)
            } {
                let a1320 = v2;
                let a1321 = match v3.clone() {
                    Adt9::C0(_, _, h1319) => {
                        h1319
                    }
                };
                nat_eq(a1320, a1321)
            } else {
                false
            } {
                f21(v1, match v3 {
                    Adt9::C0(_, h1322, _) => {
                        h1322
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> bool {
    let m1323 = f17(v0);
    match m1323 {
        Adt8::C0 => {
            false
        }
        Adt8::C1(v3) => {
            let a1325 = v2;
            let a1326 = match v3.clone() {
                Adt9::C0(_, _, h1324) => {
                    h1324
                }
            };
            if nat_eq(a1325, a1326) {
                f21(v1, match v3 {
                    Adt9::C0(_, h1327, _) => {
                        h1327
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1328 = v1;
    let a1329 = 1u64;
    if if if nat_le(a1328, a1329) {
        true
    } else {
        let a1330 = v0;
        let a1331 = v2;
        nat_le(a1330, a1331)
    } {
        let a1332 = v3;
        let a1333 = 1u64;
        if nat_le(a1332, a1333) {
            true
        } else {
            let a1340 = {
                let a1338 = v0;
                let a1339 = {
                    let a1336 = {
                        let a1334 = v1;
                        let a1335 = 1u64;
                        nat_sub(a1334, a1335)
                    };
                    let a1337 = v2;
                    nat_mul(a1336, a1337)?
                };
                nat_add(a1338, a1339)?
            };
            let a1341 = v4;
            nat_le(a1340, a1341)
        }
    } else {
        false
    } {
        let a1342 = v5;
        let a1343 = 1u64;
        if nat_le(a1342, a1343) {
            Ok(true)
        } else {
            let a1356 = {
                let a1354 = {
                    let a1348 = v0;
                    let a1349 = {
                        let a1346 = {
                            let a1344 = v1;
                            let a1345 = 1u64;
                            nat_sub(a1344, a1345)
                        };
                        let a1347 = v2;
                        nat_mul(a1346, a1347)?
                    };
                    nat_add(a1348, a1349)?
                };
                let a1355 = {
                    let a1352 = {
                        let a1350 = v3;
                        let a1351 = 1u64;
                        nat_sub(a1350, a1351)
                    };
                    let a1353 = v4;
                    nat_mul(a1352, a1353)?
                };
                nat_add(a1354, a1355)?
            };
            let a1357 = v6;
            Ok(nat_le(a1356, a1357))
        }
    } else {
        Ok(false)
    }
}

pub fn f52(v0: u64) -> bool {
    let a1358 = v0;
    let a1359 = 2u64;
    if if if if nat_eq(a1358, a1359) {
        true
    } else {
        let a1360 = v0;
        let a1361 = 6u64;
        nat_eq(a1360, a1361)
    } {
        true
    } else {
        let a1362 = v0;
        let a1363 = 8u64;
        nat_eq(a1362, a1363)
    } {
        true
    } else {
        let a1364 = v0;
        let a1365 = 20u64;
        nat_eq(a1364, a1365)
    } {
        true
    } else {
        f55(v0)
    }
}

pub fn f53(v0: u64) -> bool {
    let a1366 = v0;
    let a1367 = 3u64;
    if nat_eq(a1366, a1367) {
        true
    } else {
        let a1368 = v0;
        let a1369 = 7u64;
        nat_eq(a1368, a1369)
    }
}

pub fn f54(v0: u64) -> u64 {
    let a1370 = v0;
    let a1371 = 30u64;
    if nat_eq(a1370, a1371) {
        30u64
    } else {
        0u64
    }
}

pub fn f55(v0: u64) -> bool {
    let a1372 = v0;
    let a1373 = 41u64;
    if nat_eq(a1372, a1373) {
        true
    } else {
        let a1374 = v0;
        let a1375 = 42u64;
        nat_eq(a1374, a1375)
    }
}
