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
    C1(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(Adt10),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64, u64),
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
        Adt8::C1(v1, v2, v3, v4) => {
            Adt7::C15(v1, v2, v3, v4)
        }
    }
}

pub fn f2(v0: Adt5, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt7> {
    let a21 = v1;
    let a22 = {
        let m20 = match v0.clone() {
            Adt5::C0(h19, _, _, _, _) => {
                h19
            }
        };
        match m20 {
            Adt1::C0(_, _, _, h18, _, _, _, _, _, _, _) => {
                h18
            }
        }
    };
    if if if if nat_lt(a21, a22) {
        let a26 = v2;
        let a27 = {
            let m25 = match v0.clone() {
                Adt5::C0(h24, _, _, _, _) => {
                    h24
                }
            };
            match m25 {
                Adt1::C0(_, _, h23, _, _, _, _, _, _, _, _) => {
                    h23
                }
            }
        };
        nat_lt(a26, a27)
    } else {
        false
    } {
        let a31 = v3;
        let a32 = {
            let m30 = match v0.clone() {
                Adt5::C0(h29, _, _, _, _) => {
                    h29
                }
            };
            match m30 {
                Adt1::C0(_, _, _, _, h28, _, _, _, _, _, _) => {
                    h28
                }
            }
        };
        nat_lt(a31, a32)
    } else {
        false
    } {
        let a36 = v4;
        let a37 = {
            let m35 = match v0.clone() {
                Adt5::C0(_, h34, _, _, _) => {
                    h34
                }
            };
            match m35 {
                Adt1::C0(_, _, _, h33, _, _, _, _, _, _, _) => {
                    h33
                }
            }
        };
        nat_lt(a36, a37)
    } else {
        false
    } {
        Ok(Adt7::C11(f18(match v0.clone() {
            Adt5::C0(h38, _, _, _, _) => {
                h38
            }
        }, v2, v1, v3)?, f18(match v0.clone() {
            Adt5::C0(_, h39, _, _, _) => {
                h39
            }
        }, 0u64, v4, v3)?, f18(match v0.clone() {
            Adt5::C0(_, _, h40, _, _) => {
                h40
            }
        }, v1, 0u64, v3)?, f18(match v0.clone() {
            Adt5::C0(_, _, _, h41, _) => {
                h41
            }
        }, v1, 0u64, {
            let a45 = v3;
            let a46 = {
                let m44 = match v0.clone() {
                    Adt5::C0(_, _, _, h43, _) => {
                        h43
                    }
                };
                match m44 {
                    Adt1::C0(_, _, _, _, h42, _, _, _, _, _, _) => {
                        h42
                    }
                }
            };
            let a47 = 0u64;
            nat_rem(a45, a46, a47)
        })?, f18(match v0 {
            Adt5::C0(_, _, _, _, h48) => {
                h48
            }
        }, v1, 0u64, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f3(v0: u64, v1: bool, v2: u64, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1, v8: Adt1, v9: Adt1, v10: Adt1) -> R<Adt7> {
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, h49, _, _, _, _, _, _, _, _, _) => {
            h49
        }
    };
    let v12: u64 = match v4.clone() {
        Adt1::C0(_, h50, _, _, _, _, _, _, _, _, _) => {
            h50
        }
    };
    let v13: u64 = match v4.clone() {
        Adt1::C0(_, _, h51, _, _, _, _, _, _, _, _) => {
            h51
        }
    };
    let v14: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h52, _, _, _, _, _, _, _) => {
            h52
        }
    };
    let v15: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h53, _, _, _, _, _, _) => {
            h53
        }
    };
    let v16: u64 = {
        let a54 = v11;
        let a55 = v12;
        nat_mul(a54, a55)?
    };
    let v17: u64 = {
        let a56 = v13;
        let a57 = v15;
        nat_mul(a56, a57)?
    };
    let v18: u64 = {
        let a60 = {
            let a58 = v14;
            let a59 = v12;
            nat_mul(a58, a59)?
        };
        let a61 = v17;
        nat_mul(a60, a61)?
    };
    let a62 = v0;
    let a63 = 74u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a62, a63) {
        !v1
    } else {
        false
    } {
        let a64 = 0u64;
        let a65 = v2;
        nat_lt(a64, a65)
    } else {
        false
    } {
        let a66 = v2;
        let a67 = 2147483647u64;
        nat_le(a66, a67)
    } else {
        false
    } {
        let a68 = 0u64;
        let a69 = v11;
        nat_lt(a68, a69)
    } else {
        false
    } {
        let a70 = 0u64;
        let a71 = v12;
        nat_lt(a70, a71)
    } else {
        false
    } {
        let a72 = 0u64;
        let a73 = v13;
        nat_lt(a72, a73)
    } else {
        false
    } {
        let a74 = 0u64;
        let a75 = v15;
        nat_lt(a74, a75)
    } else {
        false
    } {
        let a77 = 0u64;
        let a78 = match v3.clone() {
            Adt1::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                h76
            }
        };
        nat_lt(a77, a78)
    } else {
        false
    } {
        f19(v13, match v7.clone() {
            Adt1::C0(_, _, h79, _, _, _, _, _, _, _, _) => {
                h79
            }
        })
    } else {
        false
    } {
        f20(v3.clone(), v10.clone())?
    } else {
        false
    } {
        f20(v4.clone(), v10.clone())?
    } else {
        false
    } {
        f20(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f20(v6.clone(), v10.clone())?
    } else {
        false
    } {
        f20(v7.clone(), v10.clone())?
    } else {
        false
    } {
        f20(v8.clone(), v10.clone())?
    } else {
        false
    } {
        f21(v4.clone())?
    } else {
        false
    } {
        f21(v7.clone())?
    } else {
        false
    } {
        f21(v8.clone())?
    } else {
        false
    } {
        f22(v3.clone(), 4u64)?
    } else {
        false
    } {
        f22(v5.clone(), 4u64)?
    } else {
        false
    } {
        f22(v6.clone(), 4u64)?
    } else {
        false
    } {
        f22(v10.clone(), 4u64)?
    } else {
        false
    } {
        let a81 = match v3.clone() {
            Adt1::C0(_, _, h80, _, _, _, _, _, _, _, _) => {
                h80
            }
        };
        let a82 = v12;
        nat_eq(a81, a82)
    } else {
        false
    } {
        let a84 = match v3.clone() {
            Adt1::C0(_, _, _, h83, _, _, _, _, _, _, _) => {
                h83
            }
        };
        let a85 = v13;
        nat_eq(a84, a85)
    } else {
        false
    } {
        f23(v5.clone(), v13, v14, v15, 1u64)
    } else {
        false
    } {
        let a87 = match v6.clone() {
            Adt1::C0(_, _, h86, _, _, _, _, _, _, _, _) => {
                h86
            }
        };
        let a88 = v13;
        nat_eq(a87, a88)
    } else {
        false
    } {
        let a90 = match v6.clone() {
            Adt1::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                h89
            }
        };
        let a91 = 1u64;
        nat_eq(a90, a91)
    } else {
        false
    } {
        let a93 = match v6.clone() {
            Adt1::C0(_, _, _, _, h92, _, _, _, _, _, _) => {
                h92
            }
        };
        let a94 = 1u64;
        nat_eq(a93, a94)
    } else {
        false
    } {
        let a96 = match v6.clone() {
            Adt1::C0(_, h95, _, _, _, _, _, _, _, _, _) => {
                h95
            }
        };
        let a97 = 1u64;
        if nat_eq(a96, a97) {
            true
        } else {
            let a99 = match v6.clone() {
                Adt1::C0(_, h98, _, _, _, _, _, _, _, _, _) => {
                    h98
                }
            };
            let a100 = v11;
            if nat_eq(a99, a100) {
                let a101 = v2;
                let a102 = 1u64;
                nat_eq(a101, a102)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a104 = match v7.clone() {
            Adt1::C0(_, h103, _, _, _, _, _, _, _, _, _) => {
                h103
            }
        };
        let a105 = v11;
        nat_eq(a104, a105)
    } else {
        false
    } {
        let a107 = match v7.clone() {
            Adt1::C0(_, _, _, h106, _, _, _, _, _, _, _) => {
                h106
            }
        };
        let a108 = v14;
        nat_eq(a107, a108)
    } else {
        false
    } {
        let a110 = match v7.clone() {
            Adt1::C0(_, _, _, _, h109, _, _, _, _, _, _) => {
                h109
            }
        };
        let a111 = v15;
        nat_eq(a110, a111)
    } else {
        false
    } {
        f24(v7.clone(), v8.clone())
    } else {
        false
    } {
        f23(v9.clone(), v15, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        let a113 = match v9.clone() {
            Adt1::C0(h112, _, _, _, _, _, _, _, _, _, _) => {
                h112
            }
        };
        let a114 = 26u64;
        nat_eq(a113, a114)
    } else {
        false
    } {
        let a116 = match v9.clone() {
            Adt1::C0(_, _, _, _, _, h115, _, _, _, _, _) => {
                h115
            }
        };
        let a117 = 4u64;
        nat_eq(a116, a117)
    } else {
        false
    } {
        f25(v9, v10.clone())?
    } else {
        false
    } {
        let a119 = match v10.clone() {
            Adt1::C0(h118, _, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        };
        let a120 = 0u64;
        nat_eq(a119, a120)
    } else {
        false
    } {
        f23(v10, {
            let a125 = v18;
            let a126 = {
                let a123 = v2;
                let a124 = {
                    let a121 = v16;
                    let a122 = v17;
                    nat_mul(a121, a122)?
                };
                nat_mul(a123, a124)?
            };
            nat_add(a125, a126)?
        }, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        Ok(Adt7::C12(Adt6::C0(v7, v8, v4, v6, v5, v3, v2, v17, v16, {
            let a129 = {
                let a127 = v16;
                let a128 = v11;
                nat_add(a127, a128)?
            };
            let a130 = v12;
            nat_add(a129, a130)?
        }, {
            let a137 = v18;
            let a138 = {
                let a135 = {
                    let a131 = v2;
                    let a132 = v14;
                    if nat_lt(a131, a132) {
                        v2
                    } else {
                        v14
                    }
                };
                let a136 = {
                    let a133 = v16;
                    let a134 = v17;
                    nat_mul(a133, a134)?
                };
                nat_mul(a135, a136)?
            };
            nat_add(a137, a138)?
        }, true)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f4(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(_, _, _, _, _, h139, _, _, _, _, _, _) => {
            h139
        }
    };
    let v5: Adt1 = match v0 {
        Adt6::C0(_, _, h140, _, _, _, _, _, _, _, _, _) => {
            h140
        }
    };
    let a142 = v1;
    let a143 = match v5.clone() {
        Adt1::C0(_, _, h141, _, _, _, _, _, _, _, _) => {
            h141
        }
    };
    if if if if nat_lt(a142, a143) {
        let a145 = v2;
        let a146 = match v5 {
            Adt1::C0(_, _, _, _, h144, _, _, _, _, _, _) => {
                h144
            }
        };
        nat_lt(a145, a146)
    } else {
        false
    } {
        let a148 = v3;
        let a149 = match v4.clone() {
            Adt1::C0(_, _, _, _, h147, _, _, _, _, _, _) => {
                h147
            }
        };
        nat_lt(a148, a149)
    } else {
        false
    } {
        let a150 = v3;
        let a151 = 2147483648u64;
        nat_lt(a150, a151)
    } else {
        false
    } {
        Ok(Adt7::C13(f18(v4, 0u64, v1, v3)?, None::<u64>))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f5(v0: Adt6, v1: u64, v2: u64) -> R<Adt7> {
    let v3: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h152, _, _, _, _, _, _, _, _, _) => {
            h152
        }
    };
    let a154 = v1;
    let a155 = match v3.clone() {
        Adt1::C0(_, _, h153, _, _, _, _, _, _, _, _) => {
            h153
        }
    };
    if if nat_lt(a154, a155) {
        let a157 = v2;
        let a158 = match v3.clone() {
            Adt1::C0(_, _, _, _, h156, _, _, _, _, _, _) => {
                h156
            }
        };
        nat_lt(a157, a158)
    } else {
        false
    } {
        Ok(Adt7::C13(f18(match v0.clone() {
            Adt6::C0(_, _, _, _, _, h159, _, _, _, _, _, _) => {
                h159
            }
        }, 0u64, v1, v2)?, {
            let a161 = match v3.clone() {
                Adt1::C0(_, _, _, h160, _, _, _, _, _, _, _) => {
                    h160
                }
            };
            let a162 = 0u64;
            if if nat_eq(a161, a162) {
                let a164 = match v0 {
                    Adt6::C0(_, _, _, _, _, _, h163, _, _, _, _, _) => {
                        h163
                    }
                };
                let a165 = 1u64;
                nat_eq(a164, a165)
            } else {
                false
            } {
                Some(f26(v3, v1, v2, 0u64)?)
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
        Adt6::C0(_, _, h166, _, _, _, _, _, _, _, _, _) => {
            h166
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(h167, _, _, _, _, _, _, _, _, _, _, _) => {
            h167
        }
    };
    let v6: u64 = match v4.clone() {
        Adt1::C0(_, h168, _, _, _, _, _, _, _, _, _) => {
            h168
        }
    };
    let v7: u64 = match v4.clone() {
        Adt1::C0(_, _, h169, _, _, _, _, _, _, _, _) => {
            h169
        }
    };
    let v8: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h170, _, _, _, _, _, _, _) => {
            h170
        }
    };
    let v9: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h171, _, _, _, _, _, _) => {
            h171
        }
    };
    let a172 = v1;
    let a173 = v7;
    if if if nat_lt(a172, a173) {
        let a174 = v2;
        let a175 = v9;
        nat_lt(a174, a175)
    } else {
        false
    } {
        let a176 = v3;
        let a177 = v8;
        nat_lt(a176, a177)
    } else {
        false
    } {
        let v10: u64 = {
            let a182 = v1;
            let a183 = {
                let a179 = v7;
                let a180 = match v5.clone() {
                    Adt1::C0(_, _, h178, _, _, _, _, _, _, _, _) => {
                        h178
                    }
                };
                let a181 = 0u64;
                nat_quot(a179, a180, a181)
            };
            let a184 = 0u64;
            nat_quot(a182, a183, a184)
        };
        let v11: u64 = {
            let a187 = v8;
            let a188 = {
                let a185 = v3;
                let a186 = 1u64;
                nat_add(a185, a186)?
            };
            nat_sub(a187, a188)
        };
        Ok(Adt7::C14(f18(v5, v10, v3, v2)?, f18(match v0.clone() {
            Adt6::C0(_, h189, _, _, _, _, _, _, _, _, _, _) => {
                h189
            }
        }, v10, v3, v2)?, f18(v4, v1, v3, v2)?, f18(match v0.clone() {
            Adt6::C0(_, _, _, h190, _, _, _, _, _, _, _, _) => {
                h190
            }
        }, v1, 0u64, 0u64)?, {
            let a194 = f18(match v0.clone() {
                Adt6::C0(_, _, _, _, h191, _, _, _, _, _, _, _) => {
                    h191
                }
            }, v3, v2, 0u64)?;
            let a195 = {
                let a192 = 4u64;
                let a193 = v1;
                nat_mul(a192, a193)?
            };
            nat_add(a194, a195)?
        }, {
            let a206 = 4u64;
            let a207 = {
                let a204 = v6;
                let a205 = {
                    let a202 = v1;
                    let a203 = {
                        let a200 = v7;
                        let a201 = {
                            let a198 = v3;
                            let a199 = {
                                let a196 = v8;
                                let a197 = v2;
                                nat_mul(a196, a197)?
                            };
                            nat_add(a198, a199)?
                        };
                        nat_mul(a200, a201)?
                    };
                    nat_add(a202, a203)?
                };
                nat_mul(a204, a205)?
            };
            nat_mul(a206, a207)?
        }, {
            let a209 = v11;
            let a210 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, h208, _, _, _, _, _) => {
                    h208
                }
            };
            if nat_lt(a209, a210) {
                Some({
                    let a229 = 4u64;
                    let a230 = {
                        let a227 = {
                            let a214 = {
                                let a211 = v6;
                                let a212 = v8;
                                nat_mul(a211, a212)?
                            };
                            let a215 = match v0.clone() {
                                Adt6::C0(_, _, _, _, _, _, _, h213, _, _, _, _) => {
                                    h213
                                }
                            };
                            nat_mul(a214, a215)?
                        };
                        let a228 = {
                            let a225 = match v0 {
                                Adt6::C0(_, _, _, _, _, _, _, _, h216, _, _, _) => {
                                    h216
                                }
                            };
                            let a226 = {
                                let a223 = v1;
                                let a224 = {
                                    let a221 = v7;
                                    let a222 = {
                                        let a219 = v2;
                                        let a220 = {
                                            let a217 = v9;
                                            let a218 = v11;
                                            nat_mul(a217, a218)?
                                        };
                                        nat_add(a219, a220)?
                                    };
                                    nat_mul(a221, a222)?
                                };
                                nat_add(a223, a224)?
                            };
                            nat_mul(a225, a226)?
                        };
                        nat_add(a227, a228)?
                    };
                    nat_mul(a229, a230)?
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
        Adt6::C0(h231, _, _, _, _, _, _, _, _, _, _, _) => {
            h231
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(_, h232, _, _, _, _, _, _, _, _, _, _) => {
            h232
        }
    };
    let v6: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h233, _, _, _, _, _, _, _, _, _) => {
            h233
        }
    };
    let a235 = v1;
    let a236 = match v6.clone() {
        Adt1::C0(_, _, h234, _, _, _, _, _, _, _, _) => {
            h234
        }
    };
    if if if nat_lt(a235, a236) {
        let a238 = v2;
        let a239 = match v6.clone() {
            Adt1::C0(_, _, _, _, h237, _, _, _, _, _, _) => {
                h237
            }
        };
        nat_lt(a238, a239)
    } else {
        false
    } {
        let a241 = v3;
        let a242 = match v6.clone() {
            Adt1::C0(_, _, _, h240, _, _, _, _, _, _, _) => {
                h240
            }
        };
        nat_lt(a241, a242)
    } else {
        false
    } {
        let v7: u64 = {
            let a246 = match v6.clone() {
                Adt1::C0(_, _, _, h243, _, _, _, _, _, _, _) => {
                    h243
                }
            };
            let a247 = {
                let a244 = v3;
                let a245 = 1u64;
                nat_add(a244, a245)?
            };
            nat_sub(a246, a247)
        };
        Ok(Adt7::C14(f18(v4.clone(), {
            let a249 = v1;
            let a250 = match v4.clone() {
                Adt1::C0(_, _, h248, _, _, _, _, _, _, _, _) => {
                    h248
                }
            };
            let a251 = 0u64;
            nat_rem(a249, a250, a251)
        }, v3, {
            let a257 = v2;
            let a258 = {
                let a254 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h252, _, _, _, _, _, _) => {
                        h252
                    }
                };
                let a255 = match v4 {
                    Adt1::C0(_, _, _, _, h253, _, _, _, _, _, _) => {
                        h253
                    }
                };
                let a256 = 0u64;
                nat_quot(a254, a255, a256)
            };
            let a259 = 0u64;
            nat_quot(a257, a258, a259)
        })?, f18(v5.clone(), {
            let a261 = v1;
            let a262 = match v5.clone() {
                Adt1::C0(_, _, h260, _, _, _, _, _, _, _, _) => {
                    h260
                }
            };
            let a263 = 0u64;
            nat_rem(a261, a262, a263)
        }, v3, {
            let a269 = v2;
            let a270 = {
                let a266 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                        h264
                    }
                };
                let a267 = match v5 {
                    Adt1::C0(_, _, _, _, h265, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a268 = 0u64;
                nat_quot(a266, a267, a268)
            };
            let a271 = 0u64;
            nat_quot(a269, a270, a271)
        })?, f18(v6.clone(), v1, v3, v2)?, f18(match v0.clone() {
            Adt6::C0(_, _, _, h272, _, _, _, _, _, _, _, _) => {
                h272
            }
        }, v1, v3, v2)?, f18(match v0.clone() {
            Adt6::C0(_, _, _, _, h273, _, _, _, _, _, _, _) => {
                h273
            }
        }, v1, v3, v2)?, {
            let a287 = 4u64;
            let a288 = {
                let a285 = match v6.clone() {
                    Adt1::C0(_, h274, _, _, _, _, _, _, _, _, _) => {
                        h274
                    }
                };
                let a286 = {
                    let a283 = v1;
                    let a284 = {
                        let a281 = match v6.clone() {
                            Adt1::C0(_, _, h275, _, _, _, _, _, _, _, _) => {
                                h275
                            }
                        };
                        let a282 = {
                            let a279 = v3;
                            let a280 = {
                                let a277 = match v6.clone() {
                                    Adt1::C0(_, _, _, h276, _, _, _, _, _, _, _) => {
                                        h276
                                    }
                                };
                                let a278 = v2;
                                nat_mul(a277, a278)?
                            };
                            nat_add(a279, a280)?
                        };
                        nat_mul(a281, a282)?
                    };
                    nat_add(a283, a284)?
                };
                nat_mul(a285, a286)?
            };
            nat_mul(a287, a288)?
        }, {
            let a290 = v7;
            let a291 = match v0 {
                Adt6::C0(_, _, _, _, _, _, h289, _, _, _, _, _) => {
                    h289
                }
            };
            if nat_lt(a290, a291) {
                Some(f26(v6, v1, v2, v7)?)
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
        let m292 = v2;
        match m292 {
            None => {
                Ok(Adt7::C0)
            }
            Some(v9) => {
                let m293 = v3;
                match m293 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v10) => {
                        let m294 = v4;
                        match m294 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v11) => {
                                let m295 = v5;
                                match m295 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v12) => {
                                        let m296 = v6;
                                        match m296 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v13) => {
                                                let m297 = v7;
                                                match m297 {
                                                    None => {
                                                        Ok(Adt7::C0)
                                                    }
                                                    Some(v14) => {
                                                        f27(v1, v9, v10, v11, v12, v13, v14, v8)
                                                    }
                                                }
                                            }
                                        }
                                    }
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
    let m298 = v6;
    match m298 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a299 = v0;
            let a300 = 43u64;
            if nat_eq(a299, a300) {
                let m301 = v9;
                match m301 {
                    None => {
                        let m302 = v10;
                        match m302 {
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
                let m303 = v7;
                match m303 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v15) => {
                        let a304 = v0;
                        let a305 = 84u64;
                        if nat_eq(a304, a305) {
                            f28(v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let a306 = v0;
                            let a307 = 88u64;
                            if nat_eq(a306, a307) {
                                f11(v0, v1, v12, v15, v8, v9, v10, v11)
                            } else {
                                let m308 = v8;
                                match m308 {
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
    let m309 = f29(v10, v8.clone(), v5.clone(), v6.clone(), v7.clone(), v11.clone())?;
    match m309 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a310 = v0;
            let a311 = 71u64;
            if if if if if if if if if if if if if if if if if if nat_eq(a310, a311) {
                !v1
            } else {
                false
            } {
                f30(v2, match v5.clone() {
                    Adt1::C0(_, _, _, h312, _, _, _, _, _, _, _) => {
                        h312
                    }
                }, v8.clone())
            } else {
                false
            } {
                let a313 = v4;
                let a314 = 0u64;
                if if if nat_eq(a313, a314) {
                    true
                } else {
                    let a315 = v4;
                    let a316 = 10u64;
                    nat_eq(a315, a316)
                } {
                    true
                } else {
                    let a317 = v4;
                    let a318 = 15u64;
                    nat_eq(a317, a318)
                } {
                    true
                } else {
                    let a319 = v4;
                    let a320 = 20u64;
                    nat_eq(a319, a320)
                }
            } else {
                false
            } {
                let a322 = match v5.clone() {
                    Adt1::C0(h321, _, _, _, _, _, _, _, _, _, _) => {
                        h321
                    }
                };
                let a323 = 0u64;
                nat_eq(a322, a323)
            } else {
                false
            } {
                let a325 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h324, _, _, _, _, _) => {
                        h324
                    }
                };
                let a326 = 4u64;
                nat_eq(a325, a326)
            } else {
                false
            } {
                f31(v11.clone())?
            } else {
                false
            } {
                f32(v6.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                f32(v7.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                let a329 = match v5.clone() {
                    Adt1::C0(_, h327, _, _, _, _, _, _, _, _, _) => {
                        h327
                    }
                };
                let a330 = match v6.clone() {
                    Adt1::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
                        h328
                    }
                };
                nat_eq(a329, a330)
            } else {
                false
            } {
                let a333 = match v6.clone() {
                    Adt1::C0(_, _, h331, _, _, _, _, _, _, _, _) => {
                        h331
                    }
                };
                let a334 = match v7.clone() {
                    Adt1::C0(_, _, h332, _, _, _, _, _, _, _, _) => {
                        h332
                    }
                };
                nat_eq(a333, a334)
            } else {
                false
            } {
                let a337 = match v11.clone() {
                    Adt1::C0(_, h335, _, _, _, _, _, _, _, _, _) => {
                        h335
                    }
                };
                let a338 = match v7.clone() {
                    Adt1::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                        h336
                    }
                };
                nat_eq(a337, a338)
            } else {
                false
            } {
                let a341 = match v11.clone() {
                    Adt1::C0(_, _, h339, _, _, _, _, _, _, _, _) => {
                        h339
                    }
                };
                let a342 = match v5.clone() {
                    Adt1::C0(_, _, _, h340, _, _, _, _, _, _, _) => {
                        h340
                    }
                };
                nat_eq(a341, a342)
            } else {
                false
            } {
                let a345 = match v11.clone() {
                    Adt1::C0(_, _, _, h343, _, _, _, _, _, _, _) => {
                        h343
                    }
                };
                let a346 = match v5.clone() {
                    Adt1::C0(_, _, h344, _, _, _, _, _, _, _, _) => {
                        h344
                    }
                };
                nat_eq(a345, a346)
            } else {
                false
            } {
                let a349 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h347, _, _, _, _, _, _) => {
                        h347
                    }
                };
                let a350 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h348, _, _, _, _, _, _) => {
                        h348
                    }
                };
                nat_eq(a349, a350)
            } else {
                false
            } {
                f25(v5.clone(), v11.clone())?
            } else {
                false
            } {
                f33(v8.clone(), v5.clone(), v12, v11.clone())?
            } else {
                false
            } {
                f34(v9.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                Ok(Adt7::C1(Adt2::C0(v5, v6, v7, v8, v9, v11, v12, f35(v2), f36(v3))))
            } else {
                Ok(Adt7::C0)
            }
        }
    }
}

pub fn f11(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    let v8: u64 = {
        let a352 = match v2.clone() {
            Adt1::C0(h351, _, _, _, _, _, _, _, _, _, _) => {
                h351
            }
        };
        let a353 = 1u64;
        if nat_eq(a352, a353) {
            2u64
        } else {
            4u64
        }
    };
    let a354 = v0;
    let a355 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a354, a355) {
        !v1
    } else {
        false
    } {
        let a357 = match v2.clone() {
            Adt1::C0(h356, _, _, _, _, _, _, _, _, _, _) => {
                h356
            }
        };
        let a358 = 0u64;
        if nat_eq(a357, a358) {
            true
        } else {
            let a360 = match v2.clone() {
                Adt1::C0(h359, _, _, _, _, _, _, _, _, _, _) => {
                    h359
                }
            };
            let a361 = 1u64;
            nat_eq(a360, a361)
        }
    } else {
        false
    } {
        let a363 = match v3.clone() {
            Adt1::C0(h362, _, _, _, _, _, _, _, _, _, _) => {
                h362
            }
        };
        let a364 = 26u64;
        nat_eq(a363, a364)
    } else {
        false
    } {
        let a367 = match v7.clone() {
            Adt1::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
                h365
            }
        };
        let a368 = match v2.clone() {
            Adt1::C0(h366, _, _, _, _, _, _, _, _, _, _) => {
                h366
            }
        };
        nat_eq(a367, a368)
    } else {
        false
    } {
        let a370 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h369, _, _, _, _, _) => {
                h369
            }
        };
        let a371 = v8;
        nat_eq(a370, a371)
    } else {
        false
    } {
        let a373 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h372, _, _, _, _, _) => {
                h372
            }
        };
        let a374 = v8;
        nat_eq(a373, a374)
    } else {
        false
    } {
        let a377 = match v2.clone() {
            Adt1::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        let a378 = match v7.clone() {
            Adt1::C0(_, h376, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        };
        nat_eq(a377, a378)
    } else {
        false
    } {
        f37(v3.clone(), v7.clone())
    } else {
        false
    } {
        f38({
            let a380 = v8;
            let a381 = match v7.clone() {
                Adt1::C0(_, h379, _, _, _, _, _, _, _, _, _) => {
                    h379
                }
            };
            nat_mul(a380, a381)?
        }, match v7.clone() {
            Adt1::C0(_, _, h382, _, _, _, _, _, _, _, _) => {
                h382
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h383, _, _, _, _, _, _, _) => {
                h383
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h384, _, _, _, _, _, _) => {
                h384
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h385, _, _, _, _) => {
                h385
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h386, _, _, _) => {
                h386
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h387, _, _) => {
                h387
            }
        })?
    } else {
        false
    } {
        f25(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f25(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f19(match v3.clone() {
            Adt1::C0(_, _, h388, _, _, _, _, _, _, _, _) => {
                h388
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h389, _, _, _, _, _, _, _, _) => {
                h389
            }
        })
    } else {
        false
    } {
        f19(match v3.clone() {
            Adt1::C0(_, _, _, h390, _, _, _, _, _, _, _) => {
                h390
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h391, _, _, _, _, _, _, _) => {
                h391
            }
        })
    } else {
        false
    } {
        f19(match v3.clone() {
            Adt1::C0(_, _, _, _, h392, _, _, _, _, _, _) => {
                h392
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h393, _, _, _, _, _, _) => {
                h393
            }
        })
    } else {
        false
    } {
        let m394 = v4.clone();
        match m394 {
            None => {
                true
            }
            Some(v9) => {
                let a396 = match v9.clone() {
                    Adt1::C0(h395, _, _, _, _, _, _, _, _, _, _) => {
                        h395
                    }
                };
                let a397 = 0u64;
                if if nat_eq(a396, a397) {
                    f37(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f25(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m398 = v5.clone();
        match m398 {
            None => {
                true
            }
            Some(v10) => {
                let a400 = match v10.clone() {
                    Adt1::C0(h399, _, _, _, _, _, _, _, _, _, _) => {
                        h399
                    }
                };
                let a401 = 26u64;
                if if if if nat_eq(a400, a401) {
                    let a403 = match v10.clone() {
                        Adt1::C0(_, _, _, h402, _, _, _, _, _, _, _) => {
                            h402
                        }
                    };
                    let a404 = 1u64;
                    nat_eq(a403, a404)
                } else {
                    false
                } {
                    let a406 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h405, _, _, _, _, _, _) => {
                            h405
                        }
                    };
                    let a407 = 1u64;
                    nat_eq(a406, a407)
                } else {
                    false
                } {
                    f25(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m408 = v4.clone();
                    match m408 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a411 = match v11 {
                                Adt1::C0(_, h409, _, _, _, _, _, _, _, _, _) => {
                                    h409
                                }
                            };
                            let a412 = match v10 {
                                Adt1::C0(_, _, h410, _, _, _, _, _, _, _, _) => {
                                    h410
                                }
                            };
                            nat_eq(a411, a412)
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
        let m413 = v6.clone();
        match m413 {
            None => {
                true
            }
            Some(v12) => {
                let a415 = match v12.clone() {
                    Adt1::C0(h414, _, _, _, _, _, _, _, _, _, _) => {
                        h414
                    }
                };
                let a416 = 26u64;
                if if nat_eq(a415, a416) {
                    f37(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f25(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt7::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a417 = v8;
            let a418 = 2u64;
            if nat_eq(a417, a418) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m419 = v5;
            match m419 {
                None => {
                    let m420 = v4;
                    match m420 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h421, _, _, _, _, _, _, _, _, _) => {
                                    h421
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h422, _, _, _, _, _, _, _, _, _) => {
                                    h422
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h423, _, _, _, _, _, _, _, _) => {
                            h423
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
        Adt4::C0(h424, _, _, _, _, _, _, _, _) => {
            h424
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h425, _, _, _, _, _, _, _) => {
            h425
        }
    };
    let a427 = v1;
    let a428 = match v5.clone() {
        Adt1::C0(_, _, h426, _, _, _, _, _, _, _, _) => {
            h426
        }
    };
    if if if nat_lt(a427, a428) {
        let a430 = v2;
        let a431 = match v5.clone() {
            Adt1::C0(_, _, _, h429, _, _, _, _, _, _, _) => {
                h429
            }
        };
        nat_lt(a430, a431)
    } else {
        false
    } {
        let a433 = v3;
        let a434 = match v5.clone() {
            Adt1::C0(_, _, _, _, h432, _, _, _, _, _, _) => {
                h432
            }
        };
        nat_lt(a433, a434)
    } else {
        false
    } {
        Ok(Adt7::C6(f18(v4.clone(), {
            let a436 = v1;
            let a437 = match v4.clone() {
                Adt1::C0(_, _, h435, _, _, _, _, _, _, _, _) => {
                    h435
                }
            };
            let a438 = 0u64;
            nat_rem(a436, a437, a438)
        }, {
            let a440 = v2;
            let a441 = match v4.clone() {
                Adt1::C0(_, _, _, h439, _, _, _, _, _, _, _) => {
                    h439
                }
            };
            let a442 = 0u64;
            nat_rem(a440, a441, a442)
        }, {
            let a444 = v3;
            let a445 = match v4 {
                Adt1::C0(_, _, _, _, h443, _, _, _, _, _, _) => {
                    h443
                }
            };
            let a446 = 0u64;
            nat_rem(a444, a445, a446)
        })?, f18(v5, v1, v2, v3)?, {
            let m448 = match v0.clone() {
                Adt4::C0(_, _, h447, _, _, _, _, _, _) => {
                    h447
                }
            };
            match m448 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f18(v6, v1, v2, v3)?
                }
            }
        }, {
            let m450 = match v0.clone() {
                Adt4::C0(_, _, _, _, h449, _, _, _, _) => {
                    h449
                }
            };
            match m450 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f18(v7, v1, v2, v3)?
                }
            }
        }, f18(match v0 {
            Adt4::C0(_, _, _, _, _, h451, _, _, _) => {
                h451
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f13(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a452 = v1;
    let a453 = 0u64;
    if nat_eq(a452, a453) {
        let a457 = v2;
        let a458 = {
            let m456 = match v0.clone() {
                Adt4::C0(_, h455, _, _, _, _, _, _, _) => {
                    h455
                }
            };
            match m456 {
                Adt1::C0(_, h454, _, _, _, _, _, _, _, _, _) => {
                    h454
                }
            }
        };
        if nat_lt(a457, a458) {
            Ok(Adt7::C7({
                let a462 = v2;
                let a463 = {
                    let m461 = match v0 {
                        Adt4::C0(_, h460, _, _, _, _, _, _, _) => {
                            h460
                        }
                    };
                    match m461 {
                        Adt1::C0(_, _, _, _, _, h459, _, _, _, _, _) => {
                            h459
                        }
                    }
                };
                nat_mul(a462, a463)?
            }))
        } else {
            Ok(Adt7::C8)
        }
    } else {
        let a464 = v1;
        let a465 = 1u64;
        if nat_eq(a464, a465) {
            let m467 = match v0 {
                Adt4::C0(_, _, _, _, h466, _, _, _, _) => {
                    h466
                }
            };
            match m467 {
                None => {
                    Ok(Adt7::C8)
                }
                Some(v4) => {
                    let a469 = v2;
                    let a470 = match v4.clone() {
                        Adt1::C0(_, h468, _, _, _, _, _, _, _, _, _) => {
                            h468
                        }
                    };
                    if nat_lt(a469, a470) {
                        Ok(Adt7::C7({
                            let a472 = v2;
                            let a473 = match v4 {
                                Adt1::C0(_, _, _, _, _, h471, _, _, _, _, _) => {
                                    h471
                                }
                            };
                            nat_mul(a472, a473)?
                        }))
                    } else {
                        Ok(Adt7::C8)
                    }
                }
            }
        } else {
            let a474 = v1;
            let a475 = 2u64;
            if nat_eq(a474, a475) {
                let a476 = v2;
                let a477 = 2147483648u64;
                if if nat_lt(a476, a477) {
                    let a479 = v2;
                    let a480 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h478) => {
                            h478
                        }
                    };
                    nat_lt(a479, a480)
                } else {
                    false
                } {
                    let m482 = match v0 {
                        Adt4::C0(_, _, h481, _, _, _, _, _, _) => {
                            h481
                        }
                    };
                    match m482 {
                        None => {
                            Ok(Adt7::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt7::C7({
                                let a484 = v2;
                                let a485 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h483, _, _, _, _, _) => {
                                        h483
                                    }
                                };
                                nat_mul(a484, a485)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt7::C8)
                }
            } else {
                let a486 = v1;
                let a487 = 3u64;
                if nat_eq(a486, a487) {
                    let m489 = match v0 {
                        Adt4::C0(_, _, _, h488, _, _, _, _, _) => {
                            h488
                        }
                    };
                    match m489 {
                        None => {
                            Ok(Adt7::C8)
                        }
                        Some(v6) => {
                            let a491 = v2;
                            let a492 = match v6.clone() {
                                Adt1::C0(_, _, h490, _, _, _, _, _, _, _, _) => {
                                    h490
                                }
                            };
                            if if nat_lt(a491, a492) {
                                let a494 = v3;
                                let a495 = match v6.clone() {
                                    Adt1::C0(_, h493, _, _, _, _, _, _, _, _, _) => {
                                        h493
                                    }
                                };
                                nat_lt(a494, a495)
                            } else {
                                false
                            } {
                                Ok(Adt7::C7({
                                    let a502 = {
                                        let a497 = v2;
                                        let a498 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h496, _, _, _, _) => {
                                                h496
                                            }
                                        };
                                        nat_mul(a497, a498)?
                                    };
                                    let a503 = {
                                        let a500 = v3;
                                        let a501 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h499, _, _, _, _, _) => {
                                                h499
                                            }
                                        };
                                        nat_mul(a500, a501)?
                                    };
                                    nat_add(a502, a503)?
                                }))
                            } else {
                                Ok(Adt7::C8)
                            }
                        }
                    }
                } else {
                    let a504 = v1;
                    let a505 = 4u64;
                    if nat_eq(a504, a505) {
                        let a506 = v2;
                        let a507 = 2147483648u64;
                        if if nat_lt(a506, a507) {
                            let a511 = v2;
                            let a512 = {
                                let m510 = match v0.clone() {
                                    Adt4::C0(h509, _, _, _, _, _, _, _, _) => {
                                        h509
                                    }
                                };
                                match m510 {
                                    Adt1::C0(_, h508, _, _, _, _, _, _, _, _, _) => {
                                        h508
                                    }
                                }
                            };
                            nat_lt(a511, a512)
                        } else {
                            false
                        } {
                            Ok(Adt7::C7({
                                let a514 = v2;
                                let a515 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h513, _, _) => {
                                        h513
                                    }
                                };
                                nat_mul(a514, a515)?
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
        let a516 = v0;
        let a517 = 43u64;
        if if if if if if if if nat_eq(a516, a517) {
            let a519 = match v4.clone() {
                Adt1::C0(h518, _, _, _, _, _, _, _, _, _, _) => {
                    h518
                }
            };
            let a520 = 0u64;
            if nat_eq(a519, a520) {
                f22(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a522 = match v7.clone() {
                Adt1::C0(h521, _, _, _, _, _, _, _, _, _, _) => {
                    h521
                }
            };
            let a523 = 0u64;
            if nat_eq(a522, a523) {
                f22(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f24(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f25(v4.clone(), v7.clone())? {
                true
            } else {
                f39(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f40(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f41(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f30(v3, match v4.clone() {
                Adt1::C0(_, _, _, h524, _, _, _, _, _, _, _) => {
                    h524
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt7::C3(Adt3::C0(v4, v5, v7, v2, f35(v3))))
        } else {
            Ok(Adt7::C0)
        }
    }
}

pub fn f15(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h525, _, _, _, _) => {
            h525
        }
    };
    let a527 = v1;
    let a528 = match v4.clone() {
        Adt1::C0(_, _, h526, _, _, _, _, _, _, _, _) => {
            h526
        }
    };
    if if if nat_lt(a527, a528) {
        let a530 = v2;
        let a531 = match v4.clone() {
            Adt1::C0(_, _, _, h529, _, _, _, _, _, _, _) => {
                h529
            }
        };
        nat_lt(a530, a531)
    } else {
        false
    } {
        let a533 = v3;
        let a534 = match v4.clone() {
            Adt1::C0(_, _, _, _, h532, _, _, _, _, _, _) => {
                h532
            }
        };
        nat_lt(a533, a534)
    } else {
        false
    } {
        let v6: u64 = {
            let m536 = match v0.clone() {
                Adt3::C0(_, h535, _, _, _) => {
                    h535
                }
            };
            match m536 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f18(v5.clone(), v1, {
                        let a538 = v2;
                        let a539 = match v5.clone() {
                            Adt1::C0(_, _, _, h537, _, _, _, _, _, _, _) => {
                                h537
                            }
                        };
                        let a540 = 0u64;
                        nat_rem(a538, a539, a540)
                    }, {
                        let a542 = v3;
                        let a543 = match v5 {
                            Adt1::C0(_, _, _, _, h541, _, _, _, _, _, _) => {
                                h541
                            }
                        };
                        let a544 = 0u64;
                        nat_rem(a542, a543, a544)
                    })?
                }
            }
        };
        Ok(Adt7::C4(f18(v4, v1, v2, v3)?, v6, {
            let a545 = v2;
            let a546 = 4u64;
            nat_mul(a545, a546)?
        }, f18(match v0 {
            Adt3::C0(_, _, h547, _, _) => {
                h547
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f16(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a551 = v1;
    let a552 = {
        let m550 = match v0.clone() {
            Adt2::C0(h549, _, _, _, _, _, _, _, _) => {
                h549
            }
        };
        match m550 {
            Adt1::C0(_, _, h548, _, _, _, _, _, _, _, _) => {
                h548
            }
        }
    };
    if if if nat_lt(a551, a552) {
        let a556 = v2;
        let a557 = {
            let m555 = match v0.clone() {
                Adt2::C0(h554, _, _, _, _, _, _, _, _) => {
                    h554
                }
            };
            match m555 {
                Adt1::C0(_, _, _, h553, _, _, _, _, _, _, _) => {
                    h553
                }
            }
        };
        nat_lt(a556, a557)
    } else {
        false
    } {
        let a561 = v3;
        let a562 = {
            let m560 = match v0.clone() {
                Adt2::C0(h559, _, _, _, _, _, _, _, _) => {
                    h559
                }
            };
            match m560 {
                Adt1::C0(_, _, _, _, h558, _, _, _, _, _, _) => {
                    h558
                }
            }
        };
        nat_lt(a561, a562)
    } else {
        false
    } {
        Ok(Adt7::C2(f18(match v0.clone() {
            Adt2::C0(h563, _, _, _, _, _, _, _, _) => {
                h563
            }
        }, v1, v2, v3)?, f18(match v0.clone() {
            Adt2::C0(_, h564, _, _, _, _, _, _, _) => {
                h564
            }
        }, 0u64, {
            let a574 = v2;
            let a575 = {
                let a571 = {
                    let m567 = match v0.clone() {
                        Adt2::C0(h566, _, _, _, _, _, _, _, _) => {
                            h566
                        }
                    };
                    match m567 {
                        Adt1::C0(_, _, _, h565, _, _, _, _, _, _, _) => {
                            h565
                        }
                    }
                };
                let a572 = {
                    let m570 = match v0.clone() {
                        Adt2::C0(_, h569, _, _, _, _, _, _, _) => {
                            h569
                        }
                    };
                    match m570 {
                        Adt1::C0(_, _, _, h568, _, _, _, _, _, _, _) => {
                            h568
                        }
                    }
                };
                let a573 = 0u64;
                nat_quot(a571, a572, a573)
            };
            let a576 = 0u64;
            nat_quot(a574, a575, a576)
        }, {
            let a586 = v3;
            let a587 = {
                let a583 = {
                    let m579 = match v0.clone() {
                        Adt2::C0(h578, _, _, _, _, _, _, _, _) => {
                            h578
                        }
                    };
                    match m579 {
                        Adt1::C0(_, _, _, _, h577, _, _, _, _, _, _) => {
                            h577
                        }
                    }
                };
                let a584 = {
                    let m582 = match v0.clone() {
                        Adt2::C0(_, h581, _, _, _, _, _, _, _) => {
                            h581
                        }
                    };
                    match m582 {
                        Adt1::C0(_, _, _, _, h580, _, _, _, _, _, _) => {
                            h580
                        }
                    }
                };
                let a585 = 0u64;
                nat_quot(a583, a584, a585)
            };
            let a588 = 0u64;
            nat_quot(a586, a587, a588)
        })?, f18(match v0.clone() {
            Adt2::C0(_, _, h589, _, _, _, _, _, _) => {
                h589
            }
        }, 0u64, {
            let a599 = v2;
            let a600 = {
                let a596 = {
                    let m592 = match v0.clone() {
                        Adt2::C0(h591, _, _, _, _, _, _, _, _) => {
                            h591
                        }
                    };
                    match m592 {
                        Adt1::C0(_, _, _, h590, _, _, _, _, _, _, _) => {
                            h590
                        }
                    }
                };
                let a597 = {
                    let m595 = match v0.clone() {
                        Adt2::C0(_, _, h594, _, _, _, _, _, _) => {
                            h594
                        }
                    };
                    match m595 {
                        Adt1::C0(_, _, _, h593, _, _, _, _, _, _, _) => {
                            h593
                        }
                    }
                };
                let a598 = 0u64;
                nat_quot(a596, a597, a598)
            };
            let a601 = 0u64;
            nat_quot(a599, a600, a601)
        }, {
            let a611 = v3;
            let a612 = {
                let a608 = {
                    let m604 = match v0.clone() {
                        Adt2::C0(h603, _, _, _, _, _, _, _, _) => {
                            h603
                        }
                    };
                    match m604 {
                        Adt1::C0(_, _, _, _, h602, _, _, _, _, _, _) => {
                            h602
                        }
                    }
                };
                let a609 = {
                    let m607 = match v0.clone() {
                        Adt2::C0(_, _, h606, _, _, _, _, _, _) => {
                            h606
                        }
                    };
                    match m607 {
                        Adt1::C0(_, _, _, _, h605, _, _, _, _, _, _) => {
                            h605
                        }
                    }
                };
                let a610 = 0u64;
                nat_quot(a608, a609, a610)
            };
            let a613 = 0u64;
            nat_quot(a611, a612, a613)
        })?, {
            let m615 = match v0.clone() {
                Adt2::C0(_, _, _, h614, _, _, _, _, _) => {
                    h614
                }
            };
            match m615 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f18(v4.clone(), v1, {
                        let a617 = v2;
                        let a618 = match v4.clone() {
                            Adt1::C0(_, _, _, h616, _, _, _, _, _, _, _) => {
                                h616
                            }
                        };
                        let a619 = 0u64;
                        nat_rem(a617, a618, a619)
                    }, {
                        let a621 = v3;
                        let a622 = match v4 {
                            Adt1::C0(_, _, _, _, h620, _, _, _, _, _, _) => {
                                h620
                            }
                        };
                        let a623 = 0u64;
                        nat_rem(a621, a622, a623)
                    })?
                }
            }
        }, {
            let a624 = v2;
            let a625 = 4u64;
            nat_mul(a624, a625)?
        }, f18(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h626, _, _, _) => {
                h626
            }
        }, v2, v1, v3)?, {
            let m629 = match v0.clone() {
                Adt2::C0(_, h628, _, _, _, _, _, _, _) => {
                    h628
                }
            };
            match m629 {
                Adt1::C0(_, _, _, _, _, _, h627, _, _, _, _) => {
                    h627
                }
            }
        }, {
            let m632 = match v0.clone() {
                Adt2::C0(_, _, h631, _, _, _, _, _, _) => {
                    h631
                }
            };
            match m632 {
                Adt1::C0(_, _, _, _, _, _, h630, _, _, _, _) => {
                    h630
                }
            }
        }, f42({
            let m635 = match v0 {
                Adt2::C0(_, h634, _, _, _, _, _, _, _) => {
                    h634
                }
            };
            match m635 {
                Adt1::C0(h633, _, _, _, _, _, _, _, _, _, _) => {
                    h633
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f17(v0: u64) -> Adt8 {
    let a636 = v0;
    let a637 = 0u64;
    if if if if nat_eq(a636, a637) {
        true
    } else {
        let a638 = v0;
        let a639 = 1u64;
        nat_eq(a638, a639)
    } {
        true
    } else {
        let a640 = v0;
        let a641 = 30u64;
        nat_eq(a640, a641)
    } {
        true
    } else {
        let a642 = v0;
        let a643 = 43u64;
        nat_eq(a642, a643)
    } {
        Adt8::C1(2u64, 1u64, 0u64, 2u64)
    } else {
        Adt8::C0
    }
}

pub fn f18(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a655 = {
        let a650 = {
            let a645 = v1;
            let a646 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h644, _, _, _, _) => {
                    h644
                }
            };
            nat_mul(a645, a646)?
        };
        let a651 = {
            let a648 = v2;
            let a649 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h647, _, _, _) => {
                    h647
                }
            };
            nat_mul(a648, a649)?
        };
        nat_add(a650, a651)?
    };
    let a656 = {
        let a653 = v3;
        let a654 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h652, _, _) => {
                h652
            }
        };
        nat_mul(a653, a654)?
    };
    nat_add(a655, a656)
}

pub fn f19(v0: u64, v1: u64) -> bool {
    let a657 = 0u64;
    let a658 = v1;
    if nat_lt(a657, a658) {
        let a662 = {
            let a659 = v0;
            let a660 = v1;
            let a661 = 0u64;
            nat_rem(a659, a660, a661)
        };
        let a663 = 0u64;
        nat_eq(a662, a663)
    } else {
        false
    }
}

pub fn f20(v0: Adt1, v1: Adt1) -> R<bool> {
    let a665 = match v0.clone() {
        Adt1::C0(h664, _, _, _, _, _, _, _, _, _, _) => {
            h664
        }
    };
    let a666 = 0u64;
    if if nat_eq(a665, a666) {
        let a668 = match v0.clone() {
            Adt1::C0(_, h667, _, _, _, _, _, _, _, _, _) => {
                h667
            }
        };
        let a669 = 1u64;
        if nat_eq(a668, a669) {
            true
        } else {
            let a671 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h670, _, _, _, _, _) => {
                    h670
                }
            };
            let a672 = 4u64;
            nat_eq(a671, a672)
        }
    } else {
        false
    } {
        f25(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1) -> R<bool> {
    let a674 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h673, _, _, _, _, _) => {
            h673
        }
    };
    let a675 = 4u64;
    if nat_eq(a674, a675) {
        let a680 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h676, _, _, _, _) => {
                h676
            }
        };
        let a681 = {
            let a678 = 4u64;
            let a679 = match v0 {
                Adt1::C0(_, h677, _, _, _, _, _, _, _, _, _) => {
                    h677
                }
            };
            nat_mul(a678, a679)?
        };
        Ok(nat_eq(a680, a681))
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt1, v1: u64) -> R<bool> {
    let a683 = match v0.clone() {
        Adt1::C0(_, h682, _, _, _, _, _, _, _, _, _) => {
            h682
        }
    };
    let a684 = 1u64;
    if if if if nat_eq(a683, a684) {
        true
    } else {
        let a686 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h685, _, _, _, _, _) => {
                h685
            }
        };
        let a687 = v1;
        nat_eq(a686, a687)
    } {
        let a689 = match v0.clone() {
            Adt1::C0(_, _, h688, _, _, _, _, _, _, _, _) => {
                h688
            }
        };
        let a690 = 1u64;
        if nat_eq(a689, a690) {
            true
        } else {
            let a695 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h691, _, _, _, _) => {
                    h691
                }
            };
            let a696 = {
                let a693 = v1;
                let a694 = match v0.clone() {
                    Adt1::C0(_, h692, _, _, _, _, _, _, _, _, _) => {
                        h692
                    }
                };
                nat_mul(a693, a694)?
            };
            nat_eq(a695, a696)
        }
    } else {
        false
    } {
        let a698 = match v0.clone() {
            Adt1::C0(_, _, _, h697, _, _, _, _, _, _, _) => {
                h697
            }
        };
        let a699 = 1u64;
        if nat_eq(a698, a699) {
            true
        } else {
            let a707 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h700, _, _, _) => {
                    h700
                }
            };
            let a708 = {
                let a705 = {
                    let a702 = v1;
                    let a703 = match v0.clone() {
                        Adt1::C0(_, h701, _, _, _, _, _, _, _, _, _) => {
                            h701
                        }
                    };
                    nat_mul(a702, a703)?
                };
                let a706 = match v0.clone() {
                    Adt1::C0(_, _, h704, _, _, _, _, _, _, _, _) => {
                        h704
                    }
                };
                nat_mul(a705, a706)?
            };
            nat_eq(a707, a708)
        }
    } else {
        false
    } {
        let a710 = match v0.clone() {
            Adt1::C0(_, _, _, _, h709, _, _, _, _, _, _) => {
                h709
            }
        };
        let a711 = 1u64;
        if nat_eq(a710, a711) {
            Ok(true)
        } else {
            let a722 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h712, _, _) => {
                    h712
                }
            };
            let a723 = {
                let a720 = {
                    let a717 = {
                        let a714 = v1;
                        let a715 = match v0.clone() {
                            Adt1::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                                h713
                            }
                        };
                        nat_mul(a714, a715)?
                    };
                    let a718 = match v0.clone() {
                        Adt1::C0(_, _, h716, _, _, _, _, _, _, _, _) => {
                            h716
                        }
                    };
                    nat_mul(a717, a718)?
                };
                let a721 = match v0 {
                    Adt1::C0(_, _, _, h719, _, _, _, _, _, _, _) => {
                        h719
                    }
                };
                nat_mul(a720, a721)?
            };
            Ok(nat_eq(a722, a723))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    let a725 = match v0.clone() {
        Adt1::C0(_, h724, _, _, _, _, _, _, _, _, _) => {
            h724
        }
    };
    let a726 = v1;
    if if if nat_eq(a725, a726) {
        let a728 = match v0.clone() {
            Adt1::C0(_, _, h727, _, _, _, _, _, _, _, _) => {
                h727
            }
        };
        let a729 = v2;
        nat_eq(a728, a729)
    } else {
        false
    } {
        let a731 = match v0.clone() {
            Adt1::C0(_, _, _, h730, _, _, _, _, _, _, _) => {
                h730
            }
        };
        let a732 = v3;
        nat_eq(a731, a732)
    } else {
        false
    } {
        let a734 = match v0 {
            Adt1::C0(_, _, _, _, h733, _, _, _, _, _, _) => {
                h733
            }
        };
        let a735 = v4;
        nat_eq(a734, a735)
    } else {
        false
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    let a738 = match v0.clone() {
        Adt1::C0(_, h736, _, _, _, _, _, _, _, _, _) => {
            h736
        }
    };
    let a739 = match v1.clone() {
        Adt1::C0(_, h737, _, _, _, _, _, _, _, _, _) => {
            h737
        }
    };
    if if if nat_eq(a738, a739) {
        let a742 = match v0.clone() {
            Adt1::C0(_, _, h740, _, _, _, _, _, _, _, _) => {
                h740
            }
        };
        let a743 = match v1.clone() {
            Adt1::C0(_, _, h741, _, _, _, _, _, _, _, _) => {
                h741
            }
        };
        nat_eq(a742, a743)
    } else {
        false
    } {
        let a746 = match v0.clone() {
            Adt1::C0(_, _, _, h744, _, _, _, _, _, _, _) => {
                h744
            }
        };
        let a747 = match v1.clone() {
            Adt1::C0(_, _, _, h745, _, _, _, _, _, _, _) => {
                h745
            }
        };
        nat_eq(a746, a747)
    } else {
        false
    } {
        let a750 = match v0 {
            Adt1::C0(_, _, _, _, h748, _, _, _, _, _, _) => {
                h748
            }
        };
        let a751 = match v1 {
            Adt1::C0(_, _, _, _, h749, _, _, _, _, _, _) => {
                h749
            }
        };
        nat_eq(a750, a751)
    } else {
        false
    }
}

pub fn f25(v0: Adt1, v1: Adt1) -> R<bool> {
    let a753 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h752, _) => {
            h752
        }
    };
    let a754 = 0u64;
    if nat_eq(a753, a754) {
        Ok(true)
    } else {
        let a756 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h755, _) => {
                h755
            }
        };
        let a757 = 0u64;
        if nat_eq(a756, a757) {
            Ok(true)
        } else {
            let a759 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h758) => {
                    h758
                }
            };
            let a760 = 0u64;
            if nat_eq(a759, a760) {
                Ok(true)
            } else {
                let a762 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h761) => {
                        h761
                    }
                };
                let a763 = 0u64;
                if nat_eq(a762, a763) {
                    Ok(true)
                } else {
                    let a769 = {
                        let a766 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h764) => {
                                h764
                            }
                        };
                        let a767 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h765, _) => {
                                h765
                            }
                        };
                        nat_add(a766, a767)?
                    };
                    let a770 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h768) => {
                            h768
                        }
                    };
                    if nat_le(a769, a770) {
                        Ok(true)
                    } else {
                        let a776 = {
                            let a773 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h771) => {
                                    h771
                                }
                            };
                            let a774 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h772, _) => {
                                    h772
                                }
                            };
                            nat_add(a773, a774)?
                        };
                        let a777 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h775) => {
                                h775
                            }
                        };
                        Ok(nat_le(a776, a777))
                    }
                }
            }
        }
    }
}

pub fn f26(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h778, _, _, _, _, _, _, _, _, _) => {
            h778
        }
    };
    let v5: u64 = match v0.clone() {
        Adt1::C0(_, _, h779, _, _, _, _, _, _, _, _) => {
            h779
        }
    };
    let v6: u64 = match v0.clone() {
        Adt1::C0(_, _, _, h780, _, _, _, _, _, _, _) => {
            h780
        }
    };
    let v7: u64 = match v0 {
        Adt1::C0(_, _, _, _, h781, _, _, _, _, _, _) => {
            h781
        }
    };
    let a802 = 4u64;
    let a803 = {
        let a800 = {
            let a786 = {
                let a782 = v4;
                let a783 = v5;
                nat_mul(a782, a783)?
            };
            let a787 = {
                let a784 = v6;
                let a785 = v7;
                nat_mul(a784, a785)?
            };
            nat_mul(a786, a787)?
        };
        let a801 = {
            let a798 = {
                let a788 = v4;
                let a789 = v4;
                nat_mul(a788, a789)?
            };
            let a799 = {
                let a796 = v1;
                let a797 = {
                    let a794 = v5;
                    let a795 = {
                        let a792 = v2;
                        let a793 = {
                            let a790 = v7;
                            let a791 = v3;
                            nat_mul(a790, a791)?
                        };
                        nat_add(a792, a793)?
                    };
                    nat_mul(a794, a795)?
                };
                nat_add(a796, a797)?
            };
            nat_mul(a798, a799)?
        };
        nat_add(a800, a801)?
    };
    nat_mul(a802, a803)
}

pub fn f27(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt7> {
    let v8: u64 = match v3.clone() {
        Adt1::C0(_, h804, _, _, _, _, _, _, _, _, _) => {
            h804
        }
    };
    let v9: u64 = match v3.clone() {
        Adt1::C0(_, _, h805, _, _, _, _, _, _, _, _) => {
            h805
        }
    };
    let v10: u64 = match v3.clone() {
        Adt1::C0(_, _, _, h806, _, _, _, _, _, _, _) => {
            h806
        }
    };
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, _, _, _, h807, _, _, _, _, _, _) => {
            h807
        }
    };
    let a808 = 0u64;
    let a809 = v8;
    if if if if if if if if if if if if if if if if if if if if if if if nat_lt(a808, a809) {
        let a810 = 0u64;
        let a811 = v9;
        nat_lt(a810, a811)
    } else {
        false
    } {
        let a812 = 0u64;
        let a813 = v11;
        nat_lt(a812, a813)
    } else {
        false
    } {
        let a814 = 0u64;
        let a815 = v0;
        nat_lt(a814, a815)
    } else {
        false
    } {
        let a816 = v0;
        let a817 = 2147483647u64;
        nat_le(a816, a817)
    } else {
        false
    } {
        f20(v1.clone(), v7.clone())?
    } else {
        false
    } {
        f20(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f20(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f20(v4.clone(), v7.clone())?
    } else {
        false
    } {
        f20(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f20(v6.clone(), v7.clone())?
    } else {
        false
    } {
        f43(v1.clone(), v3.clone())
    } else {
        false
    } {
        f43(v2.clone(), v3.clone())
    } else {
        false
    } {
        let a819 = match v4.clone() {
            Adt1::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                h818
            }
        };
        let a820 = 1u64;
        if nat_eq(a819, a820) {
            true
        } else {
            let a822 = match v4.clone() {
                Adt1::C0(_, h821, _, _, _, _, _, _, _, _, _) => {
                    h821
                }
            };
            let a823 = v8;
            nat_eq(a822, a823)
        }
    } else {
        false
    } {
        let a826 = match v4.clone() {
            Adt1::C0(_, h824, _, _, _, _, _, _, _, _, _) => {
                h824
            }
        };
        let a827 = match v4.clone() {
            Adt1::C0(_, h825, _, _, _, _, _, _, _, _, _) => {
                h825
            }
        };
        if if if nat_eq(a826, a827) {
            let a829 = match v4.clone() {
                Adt1::C0(_, _, h828, _, _, _, _, _, _, _, _) => {
                    h828
                }
            };
            let a830 = v9;
            nat_eq(a829, a830)
        } else {
            false
        } {
            let a832 = match v4.clone() {
                Adt1::C0(_, _, _, h831, _, _, _, _, _, _, _) => {
                    h831
                }
            };
            let a833 = v10;
            nat_eq(a832, a833)
        } else {
            false
        } {
            let a835 = match v4.clone() {
                Adt1::C0(_, _, _, _, h834, _, _, _, _, _, _) => {
                    h834
                }
            };
            let a836 = v11;
            nat_eq(a835, a836)
        } else {
            false
        }
    } else {
        false
    } {
        let a838 = match v5.clone() {
            Adt1::C0(_, h837, _, _, _, _, _, _, _, _, _) => {
                h837
            }
        };
        let a839 = 1u64;
        if if if nat_eq(a838, a839) {
            let a841 = match v5.clone() {
                Adt1::C0(_, _, h840, _, _, _, _, _, _, _, _) => {
                    h840
                }
            };
            let a842 = v9;
            nat_eq(a841, a842)
        } else {
            false
        } {
            let a844 = match v5.clone() {
                Adt1::C0(_, _, _, h843, _, _, _, _, _, _, _) => {
                    h843
                }
            };
            let a845 = v10;
            nat_eq(a844, a845)
        } else {
            false
        } {
            let a847 = match v5.clone() {
                Adt1::C0(_, _, _, _, h846, _, _, _, _, _, _) => {
                    h846
                }
            };
            let a848 = v11;
            nat_eq(a847, a848)
        } else {
            false
        }
    } else {
        false
    } {
        let a850 = match v6.clone() {
            Adt1::C0(_, h849, _, _, _, _, _, _, _, _, _) => {
                h849
            }
        };
        let a851 = v8;
        if if if nat_eq(a850, a851) {
            let a853 = match v6.clone() {
                Adt1::C0(_, _, h852, _, _, _, _, _, _, _, _) => {
                    h852
                }
            };
            let a854 = v8;
            nat_eq(a853, a854)
        } else {
            false
        } {
            let a856 = match v6.clone() {
                Adt1::C0(_, _, _, h855, _, _, _, _, _, _, _) => {
                    h855
                }
            };
            let a857 = v9;
            nat_eq(a856, a857)
        } else {
            false
        } {
            let a859 = match v6.clone() {
                Adt1::C0(_, _, _, _, h858, _, _, _, _, _, _) => {
                    h858
                }
            };
            let a860 = v11;
            nat_eq(a859, a860)
        } else {
            false
        }
    } else {
        false
    } {
        let a864 = match v7.clone() {
            Adt1::C0(_, h861, _, _, _, _, _, _, _, _, _) => {
                h861
            }
        };
        let a865 = {
            let a862 = v8;
            let a863 = v9;
            nat_mul(a862, a863)?
        };
        if if if nat_eq(a864, a865) {
            let a875 = match v7.clone() {
                Adt1::C0(_, _, h866, _, _, _, _, _, _, _, _) => {
                    h866
                }
            };
            let a876 = {
                let a873 = {
                    let a867 = v10;
                    let a868 = v11;
                    nat_mul(a867, a868)?
                };
                let a874 = {
                    let a871 = {
                        let a869 = v0;
                        let a870 = v8;
                        nat_mul(a869, a870)?
                    };
                    let a872 = v11;
                    nat_mul(a871, a872)?
                };
                nat_add(a873, a874)?
            };
            nat_eq(a875, a876)
        } else {
            false
        } {
            let a878 = match v7.clone() {
                Adt1::C0(_, _, _, h877, _, _, _, _, _, _, _) => {
                    h877
                }
            };
            let a879 = 1u64;
            nat_eq(a878, a879)
        } else {
            false
        } {
            let a881 = match v7.clone() {
                Adt1::C0(_, _, _, _, h880, _, _, _, _, _, _) => {
                    h880
                }
            };
            let a882 = 1u64;
            nat_eq(a881, a882)
        } else {
            false
        }
    } else {
        false
    } {
        let a884 = match v7.clone() {
            Adt1::C0(h883, _, _, _, _, _, _, _, _, _, _) => {
                h883
            }
        };
        let a885 = 0u64;
        nat_eq(a884, a885)
    } else {
        false
    } {
        f22(v4.clone(), 4u64)?
    } else {
        false
    } {
        f22(v5.clone(), 4u64)?
    } else {
        false
    } {
        f22(v6.clone(), 4u64)?
    } else {
        false
    } {
        f22(v7, 4u64)?
    } else {
        false
    } {
        let v12: u64 = {
            let a886 = v9;
            let a887 = v11;
            nat_mul(a886, a887)?
        };
        let v13: u64 = {
            let a888 = v8;
            let a889 = v8;
            nat_mul(a888, a889)?
        };
        let v14: u64 = {
            let a890 = v10;
            let a891 = 0u64;
            if if nat_eq(a890, a891) {
                let a892 = v0;
                let a893 = 1u64;
                nat_eq(a892, a893)
            } else {
                false
            } {
                1u64
            } else {
                let a894 = v0;
                let a895 = v10;
                if nat_lt(a894, a895) {
                    v0
                } else {
                    v10
                }
            }
        };
        let v15: u64 = {
            let a902 = v12;
            let a903 = {
                let a900 = {
                    let a896 = v10;
                    let a897 = v8;
                    nat_mul(a896, a897)?
                };
                let a901 = {
                    let a898 = v14;
                    let a899 = v13;
                    nat_mul(a898, a899)?
                };
                nat_add(a900, a901)?
            };
            nat_mul(a902, a903)?
        };
        Ok(Adt7::C12(Adt6::C0(v1, v2, v3, v4, v5, v6, v0, v12, v13, {
            let a906 = {
                let a904 = v13;
                let a905 = v8;
                nat_add(a904, a905)?
            };
            let a907 = v8;
            nat_add(a906, a907)?
        }, v15, false)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f28(v0: bool, v1: Adt1, v2: Adt1, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Adt1) -> R<Adt7> {
    let v8: Adt1 = {
        let m908 = v3.clone();
        match m908 {
            None => {
                v1.clone()
            }
            Some(v7) => {
                v7
            }
        }
    };
    let v10: Adt1 = {
        let m909 = v4.clone();
        match m909 {
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
        let m910 = v5;
        match m910 {
            None => {
                let m911 = v3;
                match m911 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m912 = v4;
                        match m912 {
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
        let a914 = match v1.clone() {
            Adt1::C0(h913, _, _, _, _, _, _, _, _, _, _) => {
                h913
            }
        };
        let a915 = 0u64;
        if nat_eq(a914, a915) {
            let a917 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h916, _, _, _, _, _) => {
                    h916
                }
            };
            let a918 = 4u64;
            nat_eq(a917, a918)
        } else {
            false
        }
    } else {
        false
    } {
        let a920 = match v8.clone() {
            Adt1::C0(h919, _, _, _, _, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = 0u64;
        if nat_eq(a920, a921) {
            let a923 = match v8.clone() {
                Adt1::C0(_, _, _, _, _, h922, _, _, _, _, _) => {
                    h922
                }
            };
            let a924 = 4u64;
            nat_eq(a923, a924)
        } else {
            false
        }
    } else {
        false
    } {
        let a926 = match v10.clone() {
            Adt1::C0(h925, _, _, _, _, _, _, _, _, _, _) => {
                h925
            }
        };
        let a927 = 1u64;
        if nat_eq(a926, a927) {
            let a929 = match v10.clone() {
                Adt1::C0(_, _, _, _, _, h928, _, _, _, _, _) => {
                    h928
                }
            };
            let a930 = 2u64;
            nat_eq(a929, a930)
        } else {
            false
        }
    } else {
        false
    } {
        f31(v6.clone())?
    } else {
        false
    } {
        f44(match v2.clone() {
            Adt1::C0(h931, _, _, _, _, _, _, _, _, _, _) => {
                h931
            }
        }, match v2.clone() {
            Adt1::C0(_, h932, _, _, _, _, _, _, _, _, _) => {
                h932
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h933, _, _, _, _, _) => {
                h933
            }
        })
    } else {
        false
    } {
        let a935 = 0u64;
        let a936 = match v1.clone() {
            Adt1::C0(_, h934, _, _, _, _, _, _, _, _, _) => {
                h934
            }
        };
        nat_lt(a935, a936)
    } else {
        false
    } {
        let a938 = 0u64;
        let a939 = match v1.clone() {
            Adt1::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                h937
            }
        };
        nat_lt(a938, a939)
    } else {
        false
    } {
        let a941 = 0u64;
        let a942 = match v1.clone() {
            Adt1::C0(_, _, _, h940, _, _, _, _, _, _, _) => {
                h940
            }
        };
        nat_lt(a941, a942)
    } else {
        false
    } {
        let a944 = 0u64;
        let a945 = match v1.clone() {
            Adt1::C0(_, _, _, _, h943, _, _, _, _, _, _) => {
                h943
            }
        };
        nat_lt(a944, a945)
    } else {
        false
    } {
        let a947 = 0u64;
        let a948 = match v2.clone() {
            Adt1::C0(_, _, _, h946, _, _, _, _, _, _, _) => {
                h946
            }
        };
        nat_lt(a947, a948)
    } else {
        false
    } {
        let a951 = match v2.clone() {
            Adt1::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        let a952 = match v1.clone() {
            Adt1::C0(_, h950, _, _, _, _, _, _, _, _, _) => {
                h950
            }
        };
        nat_eq(a951, a952)
    } else {
        false
    } {
        let a954 = match v2.clone() {
            Adt1::C0(_, _, h953, _, _, _, _, _, _, _, _) => {
                h953
            }
        };
        let a955 = 1u64;
        nat_eq(a954, a955)
    } else {
        false
    } {
        let a958 = match v2.clone() {
            Adt1::C0(_, _, _, _, h956, _, _, _, _, _, _) => {
                h956
            }
        };
        let a959 = match v1.clone() {
            Adt1::C0(_, _, _, _, h957, _, _, _, _, _, _) => {
                h957
            }
        };
        nat_eq(a958, a959)
    } else {
        false
    } {
        let a962 = match v8.clone() {
            Adt1::C0(_, h960, _, _, _, _, _, _, _, _, _) => {
                h960
            }
        };
        let a963 = match v1.clone() {
            Adt1::C0(_, _, h961, _, _, _, _, _, _, _, _) => {
                h961
            }
        };
        nat_eq(a962, a963)
    } else {
        false
    } {
        let a966 = match v8.clone() {
            Adt1::C0(_, _, h964, _, _, _, _, _, _, _, _) => {
                h964
            }
        };
        let a967 = match v1.clone() {
            Adt1::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                h965
            }
        };
        nat_eq(a966, a967)
    } else {
        false
    } {
        let a969 = match v8.clone() {
            Adt1::C0(_, _, _, h968, _, _, _, _, _, _, _) => {
                h968
            }
        };
        let a970 = 1u64;
        nat_eq(a969, a970)
    } else {
        false
    } {
        let a973 = match v8.clone() {
            Adt1::C0(_, _, _, _, h971, _, _, _, _, _, _) => {
                h971
            }
        };
        let a974 = match v1.clone() {
            Adt1::C0(_, _, _, _, h972, _, _, _, _, _, _) => {
                h972
            }
        };
        nat_eq(a973, a974)
    } else {
        false
    } {
        let a977 = match v10.clone() {
            Adt1::C0(_, h975, _, _, _, _, _, _, _, _, _) => {
                h975
            }
        };
        let a978 = match v2.clone() {
            Adt1::C0(_, _, _, h976, _, _, _, _, _, _, _) => {
                h976
            }
        };
        nat_eq(a977, a978)
    } else {
        false
    } {
        let a981 = match v10.clone() {
            Adt1::C0(_, _, h979, _, _, _, _, _, _, _, _) => {
                h979
            }
        };
        let a982 = match v1.clone() {
            Adt1::C0(_, _, _, h980, _, _, _, _, _, _, _) => {
                h980
            }
        };
        nat_eq(a981, a982)
    } else {
        false
    } {
        let a984 = match v10.clone() {
            Adt1::C0(_, _, _, h983, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a985 = 1u64;
        nat_eq(a984, a985)
    } else {
        false
    } {
        f19(match v1.clone() {
            Adt1::C0(_, _, _, _, h986, _, _, _, _, _, _) => {
                h986
            }
        }, match v10.clone() {
            Adt1::C0(_, _, _, _, h987, _, _, _, _, _, _) => {
                h987
            }
        })
    } else {
        false
    } {
        let a990 = match v6.clone() {
            Adt1::C0(_, h988, _, _, _, _, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = match v2.clone() {
            Adt1::C0(_, _, _, h989, _, _, _, _, _, _, _) => {
                h989
            }
        };
        nat_eq(a990, a991)
    } else {
        false
    } {
        let a994 = match v6.clone() {
            Adt1::C0(_, _, h992, _, _, _, _, _, _, _, _) => {
                h992
            }
        };
        let a995 = match v1.clone() {
            Adt1::C0(_, _, _, h993, _, _, _, _, _, _, _) => {
                h993
            }
        };
        nat_eq(a994, a995)
    } else {
        false
    } {
        let a997 = match v6.clone() {
            Adt1::C0(_, _, _, h996, _, _, _, _, _, _, _) => {
                h996
            }
        };
        let a998 = 1u64;
        nat_eq(a997, a998)
    } else {
        false
    } {
        let a1001 = match v6.clone() {
            Adt1::C0(_, _, _, _, h999, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1002 = match v1.clone() {
            Adt1::C0(_, _, _, _, h1000, _, _, _, _, _, _) => {
                h1000
            }
        };
        nat_eq(a1001, a1002)
    } else {
        false
    } {
        f25(v1.clone(), v6.clone())?
    } else {
        false
    } {
        f25(v2.clone(), v6.clone())?
    } else {
        false
    } {
        f25(v8.clone(), v6.clone())?
    } else {
        false
    } {
        f25(v10.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt7::C10(Adt5::C0(v1, v2, v8, v10, v6)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f29(v0: Option<Adt1>, v1: Option<Adt1>, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Option<u64>> {
    let m1003 = v0;
    match m1003 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h1004, _, _, _, _, _, _, _, _) => {
                    h1004
                }
            }))
        }
        Some(v6) => {
            let m1005 = v1;
            match m1005 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a1007 = match v6.clone() {
                        Adt1::C0(h1006, _, _, _, _, _, _, _, _, _, _) => {
                            h1006
                        }
                    };
                    let a1008 = 26u64;
                    if if if if if if if if if nat_eq(a1007, a1008) {
                        f22(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a1011 = match v6.clone() {
                            Adt1::C0(_, h1009, _, _, _, _, _, _, _, _, _) => {
                                h1009
                            }
                        };
                        let a1012 = match v7 {
                            Adt1::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                                h1010
                            }
                        };
                        nat_eq(a1011, a1012)
                    } else {
                        false
                    } {
                        let a1015 = match v6.clone() {
                            Adt1::C0(_, _, h1013, _, _, _, _, _, _, _, _) => {
                                h1013
                            }
                        };
                        let a1016 = match v2 {
                            Adt1::C0(_, _, _, _, h1014, _, _, _, _, _, _) => {
                                h1014
                            }
                        };
                        nat_eq(a1015, a1016)
                    } else {
                        false
                    } {
                        let a1018 = match v6.clone() {
                            Adt1::C0(_, _, _, h1017, _, _, _, _, _, _, _) => {
                                h1017
                            }
                        };
                        let a1019 = 1u64;
                        nat_eq(a1018, a1019)
                    } else {
                        false
                    } {
                        let a1021 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h1020, _, _, _, _, _, _) => {
                                h1020
                            }
                        };
                        let a1022 = 1u64;
                        nat_eq(a1021, a1022)
                    } else {
                        false
                    } {
                        let a1024 = match v3 {
                            Adt1::C0(_, _, _, _, h1023, _, _, _, _, _, _) => {
                                h1023
                            }
                        };
                        let a1025 = 1u64;
                        nat_eq(a1024, a1025)
                    } else {
                        false
                    } {
                        let a1027 = match v4 {
                            Adt1::C0(_, _, _, _, h1026, _, _, _, _, _, _) => {
                                h1026
                            }
                        };
                        let a1028 = 1u64;
                        nat_eq(a1027, a1028)
                    } else {
                        false
                    } {
                        f25(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h1029, _, _, _, _, _, _, _, _, _) => {
                                h1029
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

pub fn f30(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a1030 = v0;
    let a1031 = 0u64;
    if if nat_eq(a1030, a1031) {
        true
    } else {
        let a1032 = 2147483648u64;
        let a1033 = v0;
        if nat_le(a1032, a1033) {
            let a1034 = v0;
            let a1035 = 4286578688u64;
            nat_lt(a1034, a1035)
        } else {
            false
        }
    } {
        true
    } else {
        let a1036 = v0;
        let a1037 = 2139095040u64;
        if if nat_lt(a1036, a1037) {
            let a1038 = v1;
            let a1039 = 4294967295u64;
            nat_le(a1038, a1039)
        } else {
            false
        } {
            let m1040 = v2;
            match m1040 {
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

pub fn f31(v0: Adt1) -> R<bool> {
    let a1042 = match v0.clone() {
        Adt1::C0(h1041, _, _, _, _, _, _, _, _, _, _) => {
            h1041
        }
    };
    let a1043 = 0u64;
    if if if if nat_eq(a1042, a1043) {
        let a1045 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1044, _, _, _, _, _) => {
                h1044
            }
        };
        let a1046 = 4u64;
        nat_eq(a1045, a1046)
    } else {
        false
    } {
        let a1052 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1047, _, _, _, _) => {
                h1047
            }
        };
        let a1053 = {
            let a1050 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1048, _, _, _, _, _) => {
                    h1048
                }
            };
            let a1051 = match v0.clone() {
                Adt1::C0(_, h1049, _, _, _, _, _, _, _, _, _) => {
                    h1049
                }
            };
            nat_mul(a1050, a1051)?
        };
        nat_eq(a1052, a1053)
    } else {
        false
    } {
        let a1059 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1054, _, _, _) => {
                h1054
            }
        };
        let a1060 = {
            let a1057 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1055, _, _, _, _) => {
                    h1055
                }
            };
            let a1058 = match v0.clone() {
                Adt1::C0(_, _, h1056, _, _, _, _, _, _, _, _) => {
                    h1056
                }
            };
            nat_mul(a1057, a1058)?
        };
        nat_eq(a1059, a1060)
    } else {
        false
    } {
        let a1066 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1061, _, _) => {
                h1061
            }
        };
        let a1067 = {
            let a1064 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1062, _, _, _) => {
                    h1062
                }
            };
            let a1065 = match v0 {
                Adt1::C0(_, _, _, h1063, _, _, _, _, _, _, _) => {
                    h1063
                }
            };
            nat_mul(a1064, a1065)?
        };
        Ok(nat_eq(a1066, a1067))
    } else {
        Ok(false)
    }
}

pub fn f32(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f44(match v0.clone() {
        Adt1::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    }, match v0.clone() {
        Adt1::C0(_, h1069, _, _, _, _, _, _, _, _, _) => {
            h1069
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1070, _, _, _, _, _) => {
            h1070
        }
    }) {
        f19(match v1.clone() {
            Adt1::C0(_, _, _, h1071, _, _, _, _, _, _, _) => {
                h1071
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h1072, _, _, _, _, _, _, _) => {
                h1072
            }
        })
    } else {
        false
    } {
        f19(match v1 {
            Adt1::C0(_, _, _, _, h1073, _, _, _, _, _, _) => {
                h1073
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h1074, _, _, _, _, _, _) => {
                h1074
            }
        })
    } else {
        false
    } {
        f25(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f33(v0: Option<Adt1>, v1: Adt1, v2: u64, v3: Adt1) -> R<bool> {
    let m1075 = v0;
    match m1075 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a1077 = match v4.clone() {
                Adt1::C0(h1076, _, _, _, _, _, _, _, _, _, _) => {
                    h1076
                }
            };
            let a1078 = 1u64;
            if if if if if if nat_eq(a1077, a1078) {
                let a1080 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h1079, _, _, _, _, _) => {
                        h1079
                    }
                };
                let a1081 = 2u64;
                nat_eq(a1080, a1081)
            } else {
                false
            } {
                let a1083 = v2;
                let a1084 = match v4.clone() {
                    Adt1::C0(_, h1082, _, _, _, _, _, _, _, _, _) => {
                        h1082
                    }
                };
                nat_le(a1083, a1084)
            } else {
                false
            } {
                let a1087 = match v1.clone() {
                    Adt1::C0(_, _, h1085, _, _, _, _, _, _, _, _) => {
                        h1085
                    }
                };
                let a1088 = match v4.clone() {
                    Adt1::C0(_, _, h1086, _, _, _, _, _, _, _, _) => {
                        h1086
                    }
                };
                nat_le(a1087, a1088)
            } else {
                false
            } {
                f19(match v1.clone() {
                    Adt1::C0(_, _, _, h1089, _, _, _, _, _, _, _) => {
                        h1089
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
                        h1090
                    }
                })
            } else {
                false
            } {
                f19(match v1 {
                    Adt1::C0(_, _, _, _, h1091, _, _, _, _, _, _) => {
                        h1091
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h1092, _, _, _, _, _, _) => {
                        h1092
                    }
                })
            } else {
                false
            } {
                f25(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f34(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1093 = v0;
    match m1093 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1095 = match v3.clone() {
                Adt1::C0(h1094, _, _, _, _, _, _, _, _, _, _) => {
                    h1094
                }
            };
            let a1096 = 0u64;
            if if if if if if nat_eq(a1095, a1096) {
                let a1098 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h1097, _, _, _, _, _) => {
                        h1097
                    }
                };
                let a1099 = 4u64;
                nat_eq(a1098, a1099)
            } else {
                false
            } {
                let a1102 = match v3.clone() {
                    Adt1::C0(_, h1100, _, _, _, _, _, _, _, _, _) => {
                        h1100
                    }
                };
                let a1103 = match v1 {
                    Adt1::C0(_, _, _, h1101, _, _, _, _, _, _, _) => {
                        h1101
                    }
                };
                nat_eq(a1102, a1103)
            } else {
                false
            } {
                let a1105 = match v3.clone() {
                    Adt1::C0(_, _, h1104, _, _, _, _, _, _, _, _) => {
                        h1104
                    }
                };
                let a1106 = 1u64;
                nat_eq(a1105, a1106)
            } else {
                false
            } {
                let a1108 = match v3.clone() {
                    Adt1::C0(_, _, _, h1107, _, _, _, _, _, _, _) => {
                        h1107
                    }
                };
                let a1109 = 1u64;
                nat_eq(a1108, a1109)
            } else {
                false
            } {
                let a1111 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1110, _, _, _, _, _, _) => {
                        h1110
                    }
                };
                let a1112 = 1u64;
                nat_eq(a1111, a1112)
            } else {
                false
            } {
                f25(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f35(v0: u64) -> Option<u64> {
    let a1113 = 0u64;
    let a1114 = v0;
    if if nat_lt(a1113, a1114) {
        let a1115 = v0;
        let a1116 = 2139095040u64;
        nat_lt(a1115, a1116)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f36(v0: u64) -> Option<u64> {
    let a1117 = v0;
    let a1118 = 0u64;
    if if nat_eq(a1117, a1118) {
        true
    } else {
        let a1119 = v0;
        let a1120 = 2147483648u64;
        nat_eq(a1119, a1120)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f37(v0: Adt1, v1: Adt1) -> bool {
    let a1123 = match v0.clone() {
        Adt1::C0(_, _, h1121, _, _, _, _, _, _, _, _) => {
            h1121
        }
    };
    let a1124 = match v1.clone() {
        Adt1::C0(_, _, h1122, _, _, _, _, _, _, _, _) => {
            h1122
        }
    };
    if if nat_eq(a1123, a1124) {
        let a1127 = match v0.clone() {
            Adt1::C0(_, _, _, h1125, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        let a1128 = match v1.clone() {
            Adt1::C0(_, _, _, h1126, _, _, _, _, _, _, _) => {
                h1126
            }
        };
        nat_eq(a1127, a1128)
    } else {
        false
    } {
        let a1131 = match v0 {
            Adt1::C0(_, _, _, _, h1129, _, _, _, _, _, _) => {
                h1129
            }
        };
        let a1132 = match v1 {
            Adt1::C0(_, _, _, _, h1130, _, _, _, _, _, _) => {
                h1130
            }
        };
        nat_eq(a1131, a1132)
    } else {
        false
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f45(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f45(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f45(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f45(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f45(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f45(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f39(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a1135 = match v0.clone() {
            Adt1::C0(h1133, _, _, _, _, _, _, _, _, _, _) => {
                h1133
            }
        };
        let a1136 = match v1.clone() {
            Adt1::C0(h1134, _, _, _, _, _, _, _, _, _, _) => {
                h1134
            }
        };
        nat_eq(a1135, a1136)
    } else {
        false
    } {
        let a1139 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1137) => {
                h1137
            }
        };
        let a1140 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1138) => {
                h1138
            }
        };
        nat_eq(a1139, a1140)
    } else {
        false
    } {
        let a1143 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1141, _, _, _, _, _) => {
                h1141
            }
        };
        let a1144 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1142, _, _, _, _, _) => {
                h1142
            }
        };
        nat_eq(a1143, a1144)
    } else {
        false
    } {
        let a1147 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1145, _, _, _, _) => {
                h1145
            }
        };
        let a1148 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1146, _, _, _, _) => {
                h1146
            }
        };
        nat_eq(a1147, a1148)
    } else {
        false
    } {
        let a1151 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1149, _, _, _) => {
                h1149
            }
        };
        let a1152 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1150, _, _, _) => {
                h1150
            }
        };
        nat_eq(a1151, a1152)
    } else {
        false
    } {
        let a1155 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1153, _, _) => {
                h1153
            }
        };
        let a1156 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1154, _, _) => {
                h1154
            }
        };
        nat_eq(a1155, a1156)
    } else {
        false
    }
}

pub fn f40(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1157 = v0;
    match m1157 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1159 = match v3.clone() {
                Adt1::C0(h1158, _, _, _, _, _, _, _, _, _, _) => {
                    h1158
                }
            };
            let a1160 = 0u64;
            if if if if if if if nat_eq(a1159, a1160) {
                true
            } else {
                let a1162 = match v3.clone() {
                    Adt1::C0(h1161, _, _, _, _, _, _, _, _, _, _) => {
                        h1161
                    }
                };
                let a1163 = 1u64;
                nat_eq(a1162, a1163)
            } {
                f22(v3.clone(), {
                    let a1165 = match v3.clone() {
                        Adt1::C0(h1164, _, _, _, _, _, _, _, _, _, _) => {
                            h1164
                        }
                    };
                    let a1166 = 0u64;
                    if nat_eq(a1165, a1166) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a1169 = match v3.clone() {
                    Adt1::C0(_, h1167, _, _, _, _, _, _, _, _, _) => {
                        h1167
                    }
                };
                let a1170 = match v1.clone() {
                    Adt1::C0(_, h1168, _, _, _, _, _, _, _, _, _) => {
                        h1168
                    }
                };
                nat_eq(a1169, a1170)
            } else {
                false
            } {
                let a1173 = match v1.clone() {
                    Adt1::C0(_, _, h1171, _, _, _, _, _, _, _, _) => {
                        h1171
                    }
                };
                let a1174 = match v3.clone() {
                    Adt1::C0(_, _, h1172, _, _, _, _, _, _, _, _) => {
                        h1172
                    }
                };
                nat_le(a1173, a1174)
            } else {
                false
            } {
                f19(match v1.clone() {
                    Adt1::C0(_, _, _, h1175, _, _, _, _, _, _, _) => {
                        h1175
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h1176, _, _, _, _, _, _, _) => {
                        h1176
                    }
                })
            } else {
                false
            } {
                f19(match v1 {
                    Adt1::C0(_, _, _, _, h1177, _, _, _, _, _, _) => {
                        h1177
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h1178, _, _, _, _, _, _) => {
                        h1178
                    }
                })
            } else {
                false
            } {
                f25(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f41(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1179 = v0;
    match m1179 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1181 = match v3.clone() {
                Adt1::C0(h1180, _, _, _, _, _, _, _, _, _, _) => {
                    h1180
                }
            };
            let a1182 = 0u64;
            if if if if if if nat_eq(a1181, a1182) {
                let a1184 = match v3.clone() {
                    Adt1::C0(_, h1183, _, _, _, _, _, _, _, _, _) => {
                        h1183
                    }
                };
                let a1185 = 1u64;
                if nat_eq(a1184, a1185) {
                    true
                } else {
                    let a1187 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h1186, _, _, _, _, _) => {
                            h1186
                        }
                    };
                    let a1188 = 4u64;
                    nat_eq(a1187, a1188)
                }
            } else {
                false
            } {
                let a1191 = match v3.clone() {
                    Adt1::C0(_, h1189, _, _, _, _, _, _, _, _, _) => {
                        h1189
                    }
                };
                let a1192 = match v1 {
                    Adt1::C0(_, _, _, h1190, _, _, _, _, _, _, _) => {
                        h1190
                    }
                };
                nat_eq(a1191, a1192)
            } else {
                false
            } {
                let a1194 = match v3.clone() {
                    Adt1::C0(_, _, h1193, _, _, _, _, _, _, _, _) => {
                        h1193
                    }
                };
                let a1195 = 1u64;
                nat_eq(a1194, a1195)
            } else {
                false
            } {
                let a1197 = match v3.clone() {
                    Adt1::C0(_, _, _, h1196, _, _, _, _, _, _, _) => {
                        h1196
                    }
                };
                let a1198 = 1u64;
                nat_eq(a1197, a1198)
            } else {
                false
            } {
                let a1200 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1199, _, _, _, _, _, _) => {
                        h1199
                    }
                };
                let a1201 = 1u64;
                nat_eq(a1200, a1201)
            } else {
                false
            } {
                f25(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f42(v0: u64) -> u64 {
    if f46(v0) {
        15u64
    } else {
        if if f47(v0) {
            true
        } else {
            f48(v0)
        } {
            8u64
        } else {
            if f49(v0) {
                9u64
            } else {
                f50(v0)
            }
        }
    }
}

pub fn f43(v0: Adt1, v1: Adt1) -> bool {
    let a1204 = match v0.clone() {
        Adt1::C0(_, h1202, _, _, _, _, _, _, _, _, _) => {
            h1202
        }
    };
    let a1205 = match v1.clone() {
        Adt1::C0(_, h1203, _, _, _, _, _, _, _, _, _) => {
            h1203
        }
    };
    if if if nat_eq(a1204, a1205) {
        let a1208 = match v0.clone() {
            Adt1::C0(_, _, _, h1206, _, _, _, _, _, _, _) => {
                h1206
            }
        };
        let a1209 = match v1.clone() {
            Adt1::C0(_, _, _, h1207, _, _, _, _, _, _, _) => {
                h1207
            }
        };
        nat_eq(a1208, a1209)
    } else {
        false
    } {
        let a1211 = 0u64;
        let a1212 = match v0.clone() {
            Adt1::C0(_, _, h1210, _, _, _, _, _, _, _, _) => {
                h1210
            }
        };
        nat_lt(a1211, a1212)
    } else {
        false
    } {
        f19(match v1 {
            Adt1::C0(_, _, _, _, h1213, _, _, _, _, _, _) => {
                h1213
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, h1214, _, _, _, _, _, _) => {
                h1214
            }
        })
    } else {
        false
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> bool {
    let m1215 = f51(v0);
    match m1215 {
        Adt9::C0 => {
            false
        }
        Adt9::C1(v3) => {
            let a1216 = v0;
            let a1217 = 0u64;
            if if if if if if nat_eq(a1216, a1217) {
                true
            } else {
                let a1218 = v0;
                let a1219 = 1u64;
                nat_eq(a1218, a1219)
            } {
                true
            } else {
                let a1220 = v0;
                let a1221 = 30u64;
                nat_eq(a1220, a1221)
            } {
                true
            } else {
                let a1222 = v0;
                let a1223 = 43u64;
                nat_eq(a1222, a1223)
            } {
                true
            } else {
                f52(v0)
            } {
                let a1225 = v2;
                let a1226 = match v3.clone() {
                    Adt10::C0(_, _, h1224) => {
                        h1224
                    }
                };
                nat_eq(a1225, a1226)
            } else {
                false
            } {
                f19(v1, match v3 {
                    Adt10::C0(_, h1227, _) => {
                        h1227
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1228 = v1;
    let a1229 = 1u64;
    if if if nat_le(a1228, a1229) {
        true
    } else {
        let a1230 = v0;
        let a1231 = v2;
        nat_le(a1230, a1231)
    } {
        let a1232 = v3;
        let a1233 = 1u64;
        if nat_le(a1232, a1233) {
            true
        } else {
            let a1240 = {
                let a1238 = v0;
                let a1239 = {
                    let a1236 = {
                        let a1234 = v1;
                        let a1235 = 1u64;
                        nat_sub(a1234, a1235)
                    };
                    let a1237 = v2;
                    nat_mul(a1236, a1237)?
                };
                nat_add(a1238, a1239)?
            };
            let a1241 = v4;
            nat_le(a1240, a1241)
        }
    } else {
        false
    } {
        let a1242 = v5;
        let a1243 = 1u64;
        if nat_le(a1242, a1243) {
            Ok(true)
        } else {
            let a1256 = {
                let a1254 = {
                    let a1248 = v0;
                    let a1249 = {
                        let a1246 = {
                            let a1244 = v1;
                            let a1245 = 1u64;
                            nat_sub(a1244, a1245)
                        };
                        let a1247 = v2;
                        nat_mul(a1246, a1247)?
                    };
                    nat_add(a1248, a1249)?
                };
                let a1255 = {
                    let a1252 = {
                        let a1250 = v3;
                        let a1251 = 1u64;
                        nat_sub(a1250, a1251)
                    };
                    let a1253 = v4;
                    nat_mul(a1252, a1253)?
                };
                nat_add(a1254, a1255)?
            };
            let a1257 = v6;
            Ok(nat_le(a1256, a1257))
        }
    } else {
        Ok(false)
    }
}

pub fn f46(v0: u64) -> bool {
    let a1258 = 10u64;
    let a1259 = v0;
    if if if if if nat_le(a1258, a1259) {
        let a1260 = v0;
        let a1261 = 14u64;
        nat_le(a1260, a1261)
    } else {
        false
    } {
        true
    } else {
        let a1262 = 16u64;
        let a1263 = v0;
        if nat_le(a1262, a1263) {
            let a1264 = v0;
            let a1265 = 19u64;
            nat_le(a1264, a1265)
        } else {
            false
        }
    } {
        true
    } else {
        let a1266 = 21u64;
        let a1267 = v0;
        if nat_le(a1266, a1267) {
            let a1268 = v0;
            let a1269 = 23u64;
            nat_le(a1268, a1269)
        } else {
            false
        }
    } {
        true
    } else {
        let a1270 = 34u64;
        let a1271 = v0;
        if nat_le(a1270, a1271) {
            let a1272 = v0;
            let a1273 = 35u64;
            nat_le(a1272, a1273)
        } else {
            false
        }
    } {
        true
    } else {
        let a1274 = v0;
        let a1275 = 29u64;
        nat_eq(a1274, a1275)
    }
}

pub fn f47(v0: u64) -> bool {
    let a1276 = v0;
    let a1277 = 2u64;
    if if if if nat_eq(a1276, a1277) {
        true
    } else {
        let a1278 = v0;
        let a1279 = 6u64;
        nat_eq(a1278, a1279)
    } {
        true
    } else {
        let a1280 = v0;
        let a1281 = 8u64;
        nat_eq(a1280, a1281)
    } {
        true
    } else {
        let a1282 = v0;
        let a1283 = 20u64;
        nat_eq(a1282, a1283)
    } {
        true
    } else {
        f53(v0)
    }
}

pub fn f48(v0: u64) -> bool {
    let a1284 = v0;
    let a1285 = 39u64;
    if nat_eq(a1284, a1285) {
        true
    } else {
        let a1286 = v0;
        let a1287 = 40u64;
        nat_eq(a1286, a1287)
    }
}

pub fn f49(v0: u64) -> bool {
    let a1288 = v0;
    let a1289 = 3u64;
    if nat_eq(a1288, a1289) {
        true
    } else {
        let a1290 = v0;
        let a1291 = 7u64;
        nat_eq(a1290, a1291)
    }
}

pub fn f50(v0: u64) -> u64 {
    let a1292 = v0;
    let a1293 = 30u64;
    if nat_eq(a1292, a1293) {
        30u64
    } else {
        0u64
    }
}

pub fn f51(v0: u64) -> Adt9 {
    let a1294 = v0;
    let a1295 = 0u64;
    if nat_eq(a1294, a1295) {
        Adt9::C1(Adt10::C0(0u64, 1u64, 4u64))
    } else {
        let a1296 = v0;
        let a1297 = 1u64;
        if nat_eq(a1296, a1297) {
            Adt9::C1(Adt10::C0(1u64, 1u64, 2u64))
        } else {
            let a1298 = v0;
            let a1299 = 2u64;
            if nat_eq(a1298, a1299) {
                Adt9::C1(Adt10::C0(2u64, 32u64, 18u64))
            } else {
                let a1300 = v0;
                let a1301 = 3u64;
                if nat_eq(a1300, a1301) {
                    Adt9::C1(Adt10::C0(3u64, 32u64, 20u64))
                } else {
                    let a1302 = v0;
                    let a1303 = 6u64;
                    if nat_eq(a1302, a1303) {
                        Adt9::C1(Adt10::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1304 = v0;
                        let a1305 = 7u64;
                        if nat_eq(a1304, a1305) {
                            Adt9::C1(Adt10::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1306 = v0;
                            let a1307 = 8u64;
                            if nat_eq(a1306, a1307) {
                                Adt9::C1(Adt10::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1308 = v0;
                                let a1309 = 9u64;
                                if nat_eq(a1308, a1309) {
                                    Adt9::C1(Adt10::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1310 = v0;
                                    let a1311 = 10u64;
                                    if nat_eq(a1310, a1311) {
                                        Adt9::C1(Adt10::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1312 = v0;
                                        let a1313 = 11u64;
                                        if nat_eq(a1312, a1313) {
                                            Adt9::C1(Adt10::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1314 = v0;
                                            let a1315 = 12u64;
                                            if nat_eq(a1314, a1315) {
                                                Adt9::C1(Adt10::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1316 = v0;
                                                let a1317 = 13u64;
                                                if nat_eq(a1316, a1317) {
                                                    Adt9::C1(Adt10::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1318 = v0;
                                                    let a1319 = 14u64;
                                                    if nat_eq(a1318, a1319) {
                                                        Adt9::C1(Adt10::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1320 = v0;
                                                        let a1321 = 15u64;
                                                        if nat_eq(a1320, a1321) {
                                                            Adt9::C1(Adt10::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1322 = v0;
                                                            let a1323 = 16u64;
                                                            if nat_eq(a1322, a1323) {
                                                                Adt9::C1(Adt10::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1324 = v0;
                                                                let a1325 = 17u64;
                                                                if nat_eq(a1324, a1325) {
                                                                    Adt9::C1(Adt10::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1326 = v0;
                                                                    let a1327 = 18u64;
                                                                    if nat_eq(a1326, a1327) {
                                                                        Adt9::C1(Adt10::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1328 = v0;
                                                                        let a1329 = 19u64;
                                                                        if nat_eq(a1328, a1329) {
                                                                            Adt9::C1(Adt10::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1330 = v0;
                                                                            let a1331 = 20u64;
                                                                            if nat_eq(a1330, a1331) {
                                                                                Adt9::C1(Adt10::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1332 = v0;
                                                                                let a1333 = 21u64;
                                                                                if nat_eq(a1332, a1333) {
                                                                                    Adt9::C1(Adt10::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1334 = v0;
                                                                                    let a1335 = 22u64;
                                                                                    if nat_eq(a1334, a1335) {
                                                                                        Adt9::C1(Adt10::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1336 = v0;
                                                                                        let a1337 = 23u64;
                                                                                        if nat_eq(a1336, a1337) {
                                                                                            Adt9::C1(Adt10::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1338 = v0;
                                                                                            let a1339 = 24u64;
                                                                                            if nat_eq(a1338, a1339) {
                                                                                                Adt9::C1(Adt10::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1340 = v0;
                                                                                                let a1341 = 25u64;
                                                                                                if nat_eq(a1340, a1341) {
                                                                                                    Adt9::C1(Adt10::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1342 = v0;
                                                                                                    let a1343 = 26u64;
                                                                                                    if nat_eq(a1342, a1343) {
                                                                                                        Adt9::C1(Adt10::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1344 = v0;
                                                                                                        let a1345 = 27u64;
                                                                                                        if nat_eq(a1344, a1345) {
                                                                                                            Adt9::C1(Adt10::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1346 = v0;
                                                                                                            let a1347 = 28u64;
                                                                                                            if nat_eq(a1346, a1347) {
                                                                                                                Adt9::C1(Adt10::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1348 = v0;
                                                                                                                let a1349 = 29u64;
                                                                                                                if nat_eq(a1348, a1349) {
                                                                                                                    Adt9::C1(Adt10::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1350 = v0;
                                                                                                                    let a1351 = 30u64;
                                                                                                                    if nat_eq(a1350, a1351) {
                                                                                                                        Adt9::C1(Adt10::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1352 = v0;
                                                                                                                        let a1353 = 34u64;
                                                                                                                        if nat_eq(a1352, a1353) {
                                                                                                                            Adt9::C1(Adt10::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1354 = v0;
                                                                                                                            let a1355 = 35u64;
                                                                                                                            if nat_eq(a1354, a1355) {
                                                                                                                                Adt9::C1(Adt10::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1356 = v0;
                                                                                                                                let a1357 = 39u64;
                                                                                                                                if nat_eq(a1356, a1357) {
                                                                                                                                    Adt9::C1(Adt10::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1358 = v0;
                                                                                                                                    let a1359 = 40u64;
                                                                                                                                    if nat_eq(a1358, a1359) {
                                                                                                                                        Adt9::C1(Adt10::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1360 = v0;
                                                                                                                                        let a1361 = 41u64;
                                                                                                                                        if nat_eq(a1360, a1361) {
                                                                                                                                            Adt9::C1(Adt10::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1362 = v0;
                                                                                                                                            let a1363 = 42u64;
                                                                                                                                            if nat_eq(a1362, a1363) {
                                                                                                                                                Adt9::C1(Adt10::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1364 = v0;
                                                                                                                                                let a1365 = 43u64;
                                                                                                                                                if nat_eq(a1364, a1365) {
                                                                                                                                                    Adt9::C1(Adt10::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt9::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f52(v0: u64) -> bool {
    if if f54(v0) {
        true
    } else {
        f48(v0)
    } {
        true
    } else {
        f46(v0)
    }
}

pub fn f53(v0: u64) -> bool {
    let a1366 = v0;
    let a1367 = 41u64;
    if nat_eq(a1366, a1367) {
        true
    } else {
        let a1368 = v0;
        let a1369 = 42u64;
        nat_eq(a1368, a1369)
    }
}

pub fn f54(v0: u64) -> bool {
    if f47(v0) {
        true
    } else {
        f49(v0)
    }
}
