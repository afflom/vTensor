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
    let a17 = v0;
    let a18 = 0u64;
    if if if if nat_eq(a17, a18) {
        true
    } else {
        let a19 = v0;
        let a20 = 1u64;
        nat_eq(a19, a20)
    } {
        true
    } else {
        let a21 = v0;
        let a22 = 30u64;
        nat_eq(a21, a22)
    } {
        true
    } else {
        let a23 = v0;
        let a24 = 43u64;
        nat_eq(a23, a24)
    } {
        Adt7::C15(2u64, 1u64, 0u64, 2u64)
    } else {
        Adt7::C0
    }
}

pub fn f2(v0: Adt5, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt7> {
    let a28 = v1;
    let a29 = {
        let m27 = match v0.clone() {
            Adt5::C0(h26, _, _, _, _) => {
                h26
            }
        };
        match m27 {
            Adt1::C0(_, _, _, h25, _, _, _, _, _, _, _) => {
                h25
            }
        }
    };
    if if if if nat_lt(a28, a29) {
        let a33 = v2;
        let a34 = {
            let m32 = match v0.clone() {
                Adt5::C0(h31, _, _, _, _) => {
                    h31
                }
            };
            match m32 {
                Adt1::C0(_, _, h30, _, _, _, _, _, _, _, _) => {
                    h30
                }
            }
        };
        nat_lt(a33, a34)
    } else {
        false
    } {
        let a38 = v3;
        let a39 = {
            let m37 = match v0.clone() {
                Adt5::C0(h36, _, _, _, _) => {
                    h36
                }
            };
            match m37 {
                Adt1::C0(_, _, _, _, h35, _, _, _, _, _, _) => {
                    h35
                }
            }
        };
        nat_lt(a38, a39)
    } else {
        false
    } {
        let a43 = v4;
        let a44 = {
            let m42 = match v0.clone() {
                Adt5::C0(_, h41, _, _, _) => {
                    h41
                }
            };
            match m42 {
                Adt1::C0(_, _, _, h40, _, _, _, _, _, _, _) => {
                    h40
                }
            }
        };
        nat_lt(a43, a44)
    } else {
        false
    } {
        Ok(Adt7::C11(f17(match v0.clone() {
            Adt5::C0(h45, _, _, _, _) => {
                h45
            }
        }, v2, v1, v3)?, f17(match v0.clone() {
            Adt5::C0(_, h46, _, _, _) => {
                h46
            }
        }, 0u64, v4, v3)?, f17(match v0.clone() {
            Adt5::C0(_, _, h47, _, _) => {
                h47
            }
        }, v1, 0u64, v3)?, f17(match v0.clone() {
            Adt5::C0(_, _, _, h48, _) => {
                h48
            }
        }, v1, 0u64, {
            let a52 = v3;
            let a53 = {
                let m51 = match v0.clone() {
                    Adt5::C0(_, _, _, h50, _) => {
                        h50
                    }
                };
                match m51 {
                    Adt1::C0(_, _, _, _, h49, _, _, _, _, _, _) => {
                        h49
                    }
                }
            };
            let a54 = 0u64;
            nat_rem(a52, a53, a54)
        })?, f17(match v0 {
            Adt5::C0(_, _, _, _, h55) => {
                h55
            }
        }, v1, 0u64, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f3(v0: u64, v1: bool, v2: u64, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1, v8: Adt1, v9: Adt1, v10: Adt1) -> R<Adt7> {
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, h56, _, _, _, _, _, _, _, _, _) => {
            h56
        }
    };
    let v12: u64 = match v4.clone() {
        Adt1::C0(_, h57, _, _, _, _, _, _, _, _, _) => {
            h57
        }
    };
    let v13: u64 = match v4.clone() {
        Adt1::C0(_, _, h58, _, _, _, _, _, _, _, _) => {
            h58
        }
    };
    let v14: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h59, _, _, _, _, _, _, _) => {
            h59
        }
    };
    let v15: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h60, _, _, _, _, _, _) => {
            h60
        }
    };
    let v16: u64 = {
        let a61 = v11;
        let a62 = v12;
        nat_mul(a61, a62)?
    };
    let v17: u64 = {
        let a63 = v13;
        let a64 = v15;
        nat_mul(a63, a64)?
    };
    let v18: u64 = {
        let a67 = {
            let a65 = v14;
            let a66 = v12;
            nat_mul(a65, a66)?
        };
        let a68 = v17;
        nat_mul(a67, a68)?
    };
    let a69 = v0;
    let a70 = 74u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a69, a70) {
        !v1
    } else {
        false
    } {
        let a71 = 0u64;
        let a72 = v2;
        nat_lt(a71, a72)
    } else {
        false
    } {
        let a73 = v2;
        let a74 = 2147483647u64;
        nat_le(a73, a74)
    } else {
        false
    } {
        let a75 = 0u64;
        let a76 = v11;
        nat_lt(a75, a76)
    } else {
        false
    } {
        let a77 = 0u64;
        let a78 = v12;
        nat_lt(a77, a78)
    } else {
        false
    } {
        let a79 = 0u64;
        let a80 = v13;
        nat_lt(a79, a80)
    } else {
        false
    } {
        let a81 = 0u64;
        let a82 = v15;
        nat_lt(a81, a82)
    } else {
        false
    } {
        let a84 = 0u64;
        let a85 = match v3.clone() {
            Adt1::C0(_, _, _, _, h83, _, _, _, _, _, _) => {
                h83
            }
        };
        nat_lt(a84, a85)
    } else {
        false
    } {
        f18(v13, match v7.clone() {
            Adt1::C0(_, _, h86, _, _, _, _, _, _, _, _) => {
                h86
            }
        })
    } else {
        false
    } {
        f19(v3.clone(), v10.clone())?
    } else {
        false
    } {
        f19(v4.clone(), v10.clone())?
    } else {
        false
    } {
        f19(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f19(v6.clone(), v10.clone())?
    } else {
        false
    } {
        f19(v7.clone(), v10.clone())?
    } else {
        false
    } {
        f19(v8.clone(), v10.clone())?
    } else {
        false
    } {
        f20(v4.clone())?
    } else {
        false
    } {
        f20(v7.clone())?
    } else {
        false
    } {
        f20(v8.clone())?
    } else {
        false
    } {
        f21(v3.clone(), 4u64)?
    } else {
        false
    } {
        f21(v5.clone(), 4u64)?
    } else {
        false
    } {
        f21(v6.clone(), 4u64)?
    } else {
        false
    } {
        f21(v10.clone(), 4u64)?
    } else {
        false
    } {
        let a88 = match v3.clone() {
            Adt1::C0(_, _, h87, _, _, _, _, _, _, _, _) => {
                h87
            }
        };
        let a89 = v12;
        nat_eq(a88, a89)
    } else {
        false
    } {
        let a91 = match v3.clone() {
            Adt1::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                h90
            }
        };
        let a92 = v13;
        nat_eq(a91, a92)
    } else {
        false
    } {
        f22(v5.clone(), v13, v14, v15, 1u64)
    } else {
        false
    } {
        let a94 = match v6.clone() {
            Adt1::C0(_, _, h93, _, _, _, _, _, _, _, _) => {
                h93
            }
        };
        let a95 = v13;
        nat_eq(a94, a95)
    } else {
        false
    } {
        let a97 = match v6.clone() {
            Adt1::C0(_, _, _, h96, _, _, _, _, _, _, _) => {
                h96
            }
        };
        let a98 = 1u64;
        nat_eq(a97, a98)
    } else {
        false
    } {
        let a100 = match v6.clone() {
            Adt1::C0(_, _, _, _, h99, _, _, _, _, _, _) => {
                h99
            }
        };
        let a101 = 1u64;
        nat_eq(a100, a101)
    } else {
        false
    } {
        let a103 = match v6.clone() {
            Adt1::C0(_, h102, _, _, _, _, _, _, _, _, _) => {
                h102
            }
        };
        let a104 = 1u64;
        if nat_eq(a103, a104) {
            true
        } else {
            let a106 = match v6.clone() {
                Adt1::C0(_, h105, _, _, _, _, _, _, _, _, _) => {
                    h105
                }
            };
            let a107 = v11;
            if nat_eq(a106, a107) {
                let a108 = v2;
                let a109 = 1u64;
                nat_eq(a108, a109)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a111 = match v7.clone() {
            Adt1::C0(_, h110, _, _, _, _, _, _, _, _, _) => {
                h110
            }
        };
        let a112 = v11;
        nat_eq(a111, a112)
    } else {
        false
    } {
        let a114 = match v7.clone() {
            Adt1::C0(_, _, _, h113, _, _, _, _, _, _, _) => {
                h113
            }
        };
        let a115 = v14;
        nat_eq(a114, a115)
    } else {
        false
    } {
        let a117 = match v7.clone() {
            Adt1::C0(_, _, _, _, h116, _, _, _, _, _, _) => {
                h116
            }
        };
        let a118 = v15;
        nat_eq(a117, a118)
    } else {
        false
    } {
        f23(v7.clone(), v8.clone())
    } else {
        false
    } {
        f22(v9.clone(), v15, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        let a120 = match v9.clone() {
            Adt1::C0(h119, _, _, _, _, _, _, _, _, _, _) => {
                h119
            }
        };
        let a121 = 26u64;
        nat_eq(a120, a121)
    } else {
        false
    } {
        let a123 = match v9.clone() {
            Adt1::C0(_, _, _, _, _, h122, _, _, _, _, _) => {
                h122
            }
        };
        let a124 = 4u64;
        nat_eq(a123, a124)
    } else {
        false
    } {
        f24(v9, v10.clone())?
    } else {
        false
    } {
        let a126 = match v10.clone() {
            Adt1::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
                h125
            }
        };
        let a127 = 0u64;
        nat_eq(a126, a127)
    } else {
        false
    } {
        f22(v10, {
            let a132 = v18;
            let a133 = {
                let a130 = v2;
                let a131 = {
                    let a128 = v16;
                    let a129 = v17;
                    nat_mul(a128, a129)?
                };
                nat_mul(a130, a131)?
            };
            nat_add(a132, a133)?
        }, 1u64, 1u64, 1u64)
    } else {
        false
    } {
        Ok(Adt7::C12(Adt6::C0(v7, v8, v4, v6, v5, v3, v2, v17, v16, {
            let a136 = {
                let a134 = v16;
                let a135 = v11;
                nat_add(a134, a135)?
            };
            let a137 = v12;
            nat_add(a136, a137)?
        }, {
            let a144 = v18;
            let a145 = {
                let a142 = {
                    let a138 = v2;
                    let a139 = v14;
                    if nat_lt(a138, a139) {
                        v2
                    } else {
                        v14
                    }
                };
                let a143 = {
                    let a140 = v16;
                    let a141 = v17;
                    nat_mul(a140, a141)?
                };
                nat_mul(a142, a143)?
            };
            nat_add(a144, a145)?
        }, true)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f4(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt6::C0(_, _, _, _, _, h146, _, _, _, _, _, _) => {
            h146
        }
    };
    let v5: Adt1 = match v0 {
        Adt6::C0(_, _, h147, _, _, _, _, _, _, _, _, _) => {
            h147
        }
    };
    let a149 = v1;
    let a150 = match v5.clone() {
        Adt1::C0(_, _, h148, _, _, _, _, _, _, _, _) => {
            h148
        }
    };
    if if if if nat_lt(a149, a150) {
        let a152 = v2;
        let a153 = match v5 {
            Adt1::C0(_, _, _, _, h151, _, _, _, _, _, _) => {
                h151
            }
        };
        nat_lt(a152, a153)
    } else {
        false
    } {
        let a155 = v3;
        let a156 = match v4.clone() {
            Adt1::C0(_, _, _, _, h154, _, _, _, _, _, _) => {
                h154
            }
        };
        nat_lt(a155, a156)
    } else {
        false
    } {
        let a157 = v3;
        let a158 = 2147483648u64;
        nat_lt(a157, a158)
    } else {
        false
    } {
        Ok(Adt7::C13(f17(v4, 0u64, v1, v3)?, None::<u64>))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f5(v0: Adt6, v1: u64, v2: u64) -> R<Adt7> {
    let v3: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h159, _, _, _, _, _, _, _, _, _) => {
            h159
        }
    };
    let a161 = v1;
    let a162 = match v3.clone() {
        Adt1::C0(_, _, h160, _, _, _, _, _, _, _, _) => {
            h160
        }
    };
    if if nat_lt(a161, a162) {
        let a164 = v2;
        let a165 = match v3.clone() {
            Adt1::C0(_, _, _, _, h163, _, _, _, _, _, _) => {
                h163
            }
        };
        nat_lt(a164, a165)
    } else {
        false
    } {
        Ok(Adt7::C13(f17(match v0.clone() {
            Adt6::C0(_, _, _, _, _, h166, _, _, _, _, _, _) => {
                h166
            }
        }, 0u64, v1, v2)?, {
            let a168 = match v3.clone() {
                Adt1::C0(_, _, _, h167, _, _, _, _, _, _, _) => {
                    h167
                }
            };
            let a169 = 0u64;
            if if nat_eq(a168, a169) {
                let a171 = match v0 {
                    Adt6::C0(_, _, _, _, _, _, h170, _, _, _, _, _) => {
                        h170
                    }
                };
                let a172 = 1u64;
                nat_eq(a171, a172)
            } else {
                false
            } {
                Some(f25(v3, v1, v2, 0u64)?)
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
        Adt6::C0(_, _, h173, _, _, _, _, _, _, _, _, _) => {
            h173
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(h174, _, _, _, _, _, _, _, _, _, _, _) => {
            h174
        }
    };
    let v6: u64 = match v4.clone() {
        Adt1::C0(_, h175, _, _, _, _, _, _, _, _, _) => {
            h175
        }
    };
    let v7: u64 = match v4.clone() {
        Adt1::C0(_, _, h176, _, _, _, _, _, _, _, _) => {
            h176
        }
    };
    let v8: u64 = match v4.clone() {
        Adt1::C0(_, _, _, h177, _, _, _, _, _, _, _) => {
            h177
        }
    };
    let v9: u64 = match v4.clone() {
        Adt1::C0(_, _, _, _, h178, _, _, _, _, _, _) => {
            h178
        }
    };
    let a179 = v1;
    let a180 = v7;
    if if if nat_lt(a179, a180) {
        let a181 = v2;
        let a182 = v9;
        nat_lt(a181, a182)
    } else {
        false
    } {
        let a183 = v3;
        let a184 = v8;
        nat_lt(a183, a184)
    } else {
        false
    } {
        let v10: u64 = {
            let a189 = v1;
            let a190 = {
                let a186 = v7;
                let a187 = match v5.clone() {
                    Adt1::C0(_, _, h185, _, _, _, _, _, _, _, _) => {
                        h185
                    }
                };
                let a188 = 0u64;
                nat_quot(a186, a187, a188)
            };
            let a191 = 0u64;
            nat_quot(a189, a190, a191)
        };
        let v11: u64 = {
            let a194 = v8;
            let a195 = {
                let a192 = v3;
                let a193 = 1u64;
                nat_add(a192, a193)?
            };
            nat_sub(a194, a195)
        };
        Ok(Adt7::C14(f17(v5, v10, v3, v2)?, f17(match v0.clone() {
            Adt6::C0(_, h196, _, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        }, v10, v3, v2)?, f17(v4, v1, v3, v2)?, f17(match v0.clone() {
            Adt6::C0(_, _, _, h197, _, _, _, _, _, _, _, _) => {
                h197
            }
        }, v1, 0u64, 0u64)?, {
            let a201 = f17(match v0.clone() {
                Adt6::C0(_, _, _, _, h198, _, _, _, _, _, _, _) => {
                    h198
                }
            }, v3, v2, 0u64)?;
            let a202 = {
                let a199 = 4u64;
                let a200 = v1;
                nat_mul(a199, a200)?
            };
            nat_add(a201, a202)?
        }, {
            let a213 = 4u64;
            let a214 = {
                let a211 = v6;
                let a212 = {
                    let a209 = v1;
                    let a210 = {
                        let a207 = v7;
                        let a208 = {
                            let a205 = v3;
                            let a206 = {
                                let a203 = v8;
                                let a204 = v2;
                                nat_mul(a203, a204)?
                            };
                            nat_add(a205, a206)?
                        };
                        nat_mul(a207, a208)?
                    };
                    nat_add(a209, a210)?
                };
                nat_mul(a211, a212)?
            };
            nat_mul(a213, a214)?
        }, {
            let a216 = v11;
            let a217 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, h215, _, _, _, _, _) => {
                    h215
                }
            };
            if nat_lt(a216, a217) {
                Some({
                    let a236 = 4u64;
                    let a237 = {
                        let a234 = {
                            let a221 = {
                                let a218 = v6;
                                let a219 = v8;
                                nat_mul(a218, a219)?
                            };
                            let a222 = match v0.clone() {
                                Adt6::C0(_, _, _, _, _, _, _, h220, _, _, _, _) => {
                                    h220
                                }
                            };
                            nat_mul(a221, a222)?
                        };
                        let a235 = {
                            let a232 = match v0 {
                                Adt6::C0(_, _, _, _, _, _, _, _, h223, _, _, _) => {
                                    h223
                                }
                            };
                            let a233 = {
                                let a230 = v1;
                                let a231 = {
                                    let a228 = v7;
                                    let a229 = {
                                        let a226 = v2;
                                        let a227 = {
                                            let a224 = v9;
                                            let a225 = v11;
                                            nat_mul(a224, a225)?
                                        };
                                        nat_add(a226, a227)?
                                    };
                                    nat_mul(a228, a229)?
                                };
                                nat_add(a230, a231)?
                            };
                            nat_mul(a232, a233)?
                        };
                        nat_add(a234, a235)?
                    };
                    nat_mul(a236, a237)?
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
        Adt6::C0(h238, _, _, _, _, _, _, _, _, _, _, _) => {
            h238
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt6::C0(_, h239, _, _, _, _, _, _, _, _, _, _) => {
            h239
        }
    };
    let v6: Adt1 = match v0.clone() {
        Adt6::C0(_, _, h240, _, _, _, _, _, _, _, _, _) => {
            h240
        }
    };
    let a242 = v1;
    let a243 = match v6.clone() {
        Adt1::C0(_, _, h241, _, _, _, _, _, _, _, _) => {
            h241
        }
    };
    if if if nat_lt(a242, a243) {
        let a245 = v2;
        let a246 = match v6.clone() {
            Adt1::C0(_, _, _, _, h244, _, _, _, _, _, _) => {
                h244
            }
        };
        nat_lt(a245, a246)
    } else {
        false
    } {
        let a248 = v3;
        let a249 = match v6.clone() {
            Adt1::C0(_, _, _, h247, _, _, _, _, _, _, _) => {
                h247
            }
        };
        nat_lt(a248, a249)
    } else {
        false
    } {
        let v7: u64 = {
            let a253 = match v6.clone() {
                Adt1::C0(_, _, _, h250, _, _, _, _, _, _, _) => {
                    h250
                }
            };
            let a254 = {
                let a251 = v3;
                let a252 = 1u64;
                nat_add(a251, a252)?
            };
            nat_sub(a253, a254)
        };
        Ok(Adt7::C14(f17(v4.clone(), {
            let a256 = v1;
            let a257 = match v4.clone() {
                Adt1::C0(_, _, h255, _, _, _, _, _, _, _, _) => {
                    h255
                }
            };
            let a258 = 0u64;
            nat_rem(a256, a257, a258)
        }, v3, {
            let a264 = v2;
            let a265 = {
                let a261 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h259, _, _, _, _, _, _) => {
                        h259
                    }
                };
                let a262 = match v4 {
                    Adt1::C0(_, _, _, _, h260, _, _, _, _, _, _) => {
                        h260
                    }
                };
                let a263 = 0u64;
                nat_quot(a261, a262, a263)
            };
            let a266 = 0u64;
            nat_quot(a264, a265, a266)
        })?, f17(v5.clone(), {
            let a268 = v1;
            let a269 = match v5.clone() {
                Adt1::C0(_, _, h267, _, _, _, _, _, _, _, _) => {
                    h267
                }
            };
            let a270 = 0u64;
            nat_rem(a268, a269, a270)
        }, v3, {
            let a276 = v2;
            let a277 = {
                let a273 = match v6.clone() {
                    Adt1::C0(_, _, _, _, h271, _, _, _, _, _, _) => {
                        h271
                    }
                };
                let a274 = match v5 {
                    Adt1::C0(_, _, _, _, h272, _, _, _, _, _, _) => {
                        h272
                    }
                };
                let a275 = 0u64;
                nat_quot(a273, a274, a275)
            };
            let a278 = 0u64;
            nat_quot(a276, a277, a278)
        })?, f17(v6.clone(), v1, v3, v2)?, f17(match v0.clone() {
            Adt6::C0(_, _, _, h279, _, _, _, _, _, _, _, _) => {
                h279
            }
        }, v1, v3, v2)?, f17(match v0.clone() {
            Adt6::C0(_, _, _, _, h280, _, _, _, _, _, _, _) => {
                h280
            }
        }, v1, v3, v2)?, {
            let a294 = 4u64;
            let a295 = {
                let a292 = match v6.clone() {
                    Adt1::C0(_, h281, _, _, _, _, _, _, _, _, _) => {
                        h281
                    }
                };
                let a293 = {
                    let a290 = v1;
                    let a291 = {
                        let a288 = match v6.clone() {
                            Adt1::C0(_, _, h282, _, _, _, _, _, _, _, _) => {
                                h282
                            }
                        };
                        let a289 = {
                            let a286 = v3;
                            let a287 = {
                                let a284 = match v6.clone() {
                                    Adt1::C0(_, _, _, h283, _, _, _, _, _, _, _) => {
                                        h283
                                    }
                                };
                                let a285 = v2;
                                nat_mul(a284, a285)?
                            };
                            nat_add(a286, a287)?
                        };
                        nat_mul(a288, a289)?
                    };
                    nat_add(a290, a291)?
                };
                nat_mul(a292, a293)?
            };
            nat_mul(a294, a295)?
        }, {
            let a297 = v7;
            let a298 = match v0 {
                Adt6::C0(_, _, _, _, _, _, h296, _, _, _, _, _) => {
                    h296
                }
            };
            if nat_lt(a297, a298) {
                Some(f25(v6, v1, v2, v7)?)
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
        let m299 = v2;
        match m299 {
            None => {
                Ok(Adt7::C0)
            }
            Some(v9) => {
                let m300 = v3;
                match m300 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v10) => {
                        let m301 = v4;
                        match m301 {
                            None => {
                                Ok(Adt7::C0)
                            }
                            Some(v11) => {
                                let m302 = v5;
                                match m302 {
                                    None => {
                                        Ok(Adt7::C0)
                                    }
                                    Some(v12) => {
                                        let m303 = v6;
                                        match m303 {
                                            None => {
                                                Ok(Adt7::C0)
                                            }
                                            Some(v13) => {
                                                let m304 = v7;
                                                match m304 {
                                                    None => {
                                                        Ok(Adt7::C0)
                                                    }
                                                    Some(v14) => {
                                                        f26(v1, v9, v10, v11, v12, v13, v14, v8)
                                                    }
                                                }
                                            }
                                        }
                                    }
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
    let m305 = v6;
    match m305 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a306 = v0;
            let a307 = 43u64;
            if nat_eq(a306, a307) {
                let m308 = v9;
                match m308 {
                    None => {
                        let m309 = v10;
                        match m309 {
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
                let m310 = v7;
                match m310 {
                    None => {
                        Ok(Adt7::C0)
                    }
                    Some(v15) => {
                        let a311 = v0;
                        let a312 = 84u64;
                        if nat_eq(a311, a312) {
                            f27(v1, v12, v15, v8, v9, v10, v11)
                        } else {
                            let a313 = v0;
                            let a314 = 88u64;
                            if nat_eq(a313, a314) {
                                f11(v0, v1, v12, v15, v8, v9, v10, v11)
                            } else {
                                let m315 = v8;
                                match m315 {
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
    let m316 = f28(v10, v8.clone(), v5.clone(), v6.clone(), v7.clone(), v11.clone())?;
    match m316 {
        None => {
            Ok(Adt7::C0)
        }
        Some(v12) => {
            let a317 = v0;
            let a318 = 71u64;
            if if if if if if if if if if if if if if if if if if nat_eq(a317, a318) {
                !v1
            } else {
                false
            } {
                f29(v2, match v5.clone() {
                    Adt1::C0(_, _, _, h319, _, _, _, _, _, _, _) => {
                        h319
                    }
                }, v8.clone())
            } else {
                false
            } {
                let a320 = v4;
                let a321 = 0u64;
                if if if nat_eq(a320, a321) {
                    true
                } else {
                    let a322 = v4;
                    let a323 = 10u64;
                    nat_eq(a322, a323)
                } {
                    true
                } else {
                    let a324 = v4;
                    let a325 = 15u64;
                    nat_eq(a324, a325)
                } {
                    true
                } else {
                    let a326 = v4;
                    let a327 = 20u64;
                    nat_eq(a326, a327)
                }
            } else {
                false
            } {
                let a329 = match v5.clone() {
                    Adt1::C0(h328, _, _, _, _, _, _, _, _, _, _) => {
                        h328
                    }
                };
                let a330 = 0u64;
                nat_eq(a329, a330)
            } else {
                false
            } {
                let a332 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h331, _, _, _, _, _) => {
                        h331
                    }
                };
                let a333 = 4u64;
                nat_eq(a332, a333)
            } else {
                false
            } {
                f30(v11.clone())?
            } else {
                false
            } {
                f31(v6.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                f31(v7.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                let a336 = match v5.clone() {
                    Adt1::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                        h334
                    }
                };
                let a337 = match v6.clone() {
                    Adt1::C0(_, h335, _, _, _, _, _, _, _, _, _) => {
                        h335
                    }
                };
                nat_eq(a336, a337)
            } else {
                false
            } {
                let a340 = match v6.clone() {
                    Adt1::C0(_, _, h338, _, _, _, _, _, _, _, _) => {
                        h338
                    }
                };
                let a341 = match v7.clone() {
                    Adt1::C0(_, _, h339, _, _, _, _, _, _, _, _) => {
                        h339
                    }
                };
                nat_eq(a340, a341)
            } else {
                false
            } {
                let a344 = match v11.clone() {
                    Adt1::C0(_, h342, _, _, _, _, _, _, _, _, _) => {
                        h342
                    }
                };
                let a345 = match v7.clone() {
                    Adt1::C0(_, h343, _, _, _, _, _, _, _, _, _) => {
                        h343
                    }
                };
                nat_eq(a344, a345)
            } else {
                false
            } {
                let a348 = match v11.clone() {
                    Adt1::C0(_, _, h346, _, _, _, _, _, _, _, _) => {
                        h346
                    }
                };
                let a349 = match v5.clone() {
                    Adt1::C0(_, _, _, h347, _, _, _, _, _, _, _) => {
                        h347
                    }
                };
                nat_eq(a348, a349)
            } else {
                false
            } {
                let a352 = match v11.clone() {
                    Adt1::C0(_, _, _, h350, _, _, _, _, _, _, _) => {
                        h350
                    }
                };
                let a353 = match v5.clone() {
                    Adt1::C0(_, _, h351, _, _, _, _, _, _, _, _) => {
                        h351
                    }
                };
                nat_eq(a352, a353)
            } else {
                false
            } {
                let a356 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h354, _, _, _, _, _, _) => {
                        h354
                    }
                };
                let a357 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h355, _, _, _, _, _, _) => {
                        h355
                    }
                };
                nat_eq(a356, a357)
            } else {
                false
            } {
                f24(v5.clone(), v11.clone())?
            } else {
                false
            } {
                f32(v8.clone(), v5.clone(), v12, v11.clone())?
            } else {
                false
            } {
                f33(v9.clone(), v5.clone(), v11.clone())?
            } else {
                false
            } {
                Ok(Adt7::C1(Adt2::C0(v5, v6, v7, v8, v9, v11, v12, f34(v2), f35(v3))))
            } else {
                Ok(Adt7::C0)
            }
        }
    }
}

pub fn f11(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Option<Adt1>, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt7> {
    let v8: u64 = {
        let a359 = match v2.clone() {
            Adt1::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                h358
            }
        };
        let a360 = 1u64;
        if nat_eq(a359, a360) {
            2u64
        } else {
            4u64
        }
    };
    let a361 = v0;
    let a362 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a361, a362) {
        !v1
    } else {
        false
    } {
        let a364 = match v2.clone() {
            Adt1::C0(h363, _, _, _, _, _, _, _, _, _, _) => {
                h363
            }
        };
        let a365 = 0u64;
        if nat_eq(a364, a365) {
            true
        } else {
            let a367 = match v2.clone() {
                Adt1::C0(h366, _, _, _, _, _, _, _, _, _, _) => {
                    h366
                }
            };
            let a368 = 1u64;
            nat_eq(a367, a368)
        }
    } else {
        false
    } {
        let a370 = match v3.clone() {
            Adt1::C0(h369, _, _, _, _, _, _, _, _, _, _) => {
                h369
            }
        };
        let a371 = 26u64;
        nat_eq(a370, a371)
    } else {
        false
    } {
        let a374 = match v7.clone() {
            Adt1::C0(h372, _, _, _, _, _, _, _, _, _, _) => {
                h372
            }
        };
        let a375 = match v2.clone() {
            Adt1::C0(h373, _, _, _, _, _, _, _, _, _, _) => {
                h373
            }
        };
        nat_eq(a374, a375)
    } else {
        false
    } {
        let a377 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h376, _, _, _, _, _) => {
                h376
            }
        };
        let a378 = v8;
        nat_eq(a377, a378)
    } else {
        false
    } {
        let a380 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h379, _, _, _, _, _) => {
                h379
            }
        };
        let a381 = v8;
        nat_eq(a380, a381)
    } else {
        false
    } {
        let a384 = match v2.clone() {
            Adt1::C0(_, h382, _, _, _, _, _, _, _, _, _) => {
                h382
            }
        };
        let a385 = match v7.clone() {
            Adt1::C0(_, h383, _, _, _, _, _, _, _, _, _) => {
                h383
            }
        };
        nat_eq(a384, a385)
    } else {
        false
    } {
        f36(v3.clone(), v7.clone())
    } else {
        false
    } {
        f37({
            let a387 = v8;
            let a388 = match v7.clone() {
                Adt1::C0(_, h386, _, _, _, _, _, _, _, _, _) => {
                    h386
                }
            };
            nat_mul(a387, a388)?
        }, match v7.clone() {
            Adt1::C0(_, _, h389, _, _, _, _, _, _, _, _) => {
                h389
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h390, _, _, _, _, _, _, _) => {
                h390
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h391, _, _, _, _, _, _) => {
                h391
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h392, _, _, _, _) => {
                h392
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h393, _, _, _) => {
                h393
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h394, _, _) => {
                h394
            }
        })?
    } else {
        false
    } {
        f24(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f24(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, h395, _, _, _, _, _, _, _, _) => {
                h395
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h396, _, _, _, _, _, _, _, _) => {
                h396
            }
        })
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, _, h397, _, _, _, _, _, _, _) => {
                h397
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h398, _, _, _, _, _, _, _) => {
                h398
            }
        })
    } else {
        false
    } {
        f18(match v3.clone() {
            Adt1::C0(_, _, _, _, h399, _, _, _, _, _, _) => {
                h399
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h400, _, _, _, _, _, _) => {
                h400
            }
        })
    } else {
        false
    } {
        let m401 = v4.clone();
        match m401 {
            None => {
                true
            }
            Some(v9) => {
                let a403 = match v9.clone() {
                    Adt1::C0(h402, _, _, _, _, _, _, _, _, _, _) => {
                        h402
                    }
                };
                let a404 = 0u64;
                if if nat_eq(a403, a404) {
                    f36(v3.clone(), v9.clone())
                } else {
                    false
                } {
                    f24(v9, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        let m405 = v5.clone();
        match m405 {
            None => {
                true
            }
            Some(v10) => {
                let a407 = match v10.clone() {
                    Adt1::C0(h406, _, _, _, _, _, _, _, _, _, _) => {
                        h406
                    }
                };
                let a408 = 26u64;
                if if if if nat_eq(a407, a408) {
                    let a410 = match v10.clone() {
                        Adt1::C0(_, _, _, h409, _, _, _, _, _, _, _) => {
                            h409
                        }
                    };
                    let a411 = 1u64;
                    nat_eq(a410, a411)
                } else {
                    false
                } {
                    let a413 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h412, _, _, _, _, _, _) => {
                            h412
                        }
                    };
                    let a414 = 1u64;
                    nat_eq(a413, a414)
                } else {
                    false
                } {
                    f24(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m415 = v4.clone();
                    match m415 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a418 = match v11 {
                                Adt1::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
                                    h416
                                }
                            };
                            let a419 = match v10 {
                                Adt1::C0(_, _, h417, _, _, _, _, _, _, _, _) => {
                                    h417
                                }
                            };
                            nat_eq(a418, a419)
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
        let m420 = v6.clone();
        match m420 {
            None => {
                true
            }
            Some(v12) => {
                let a422 = match v12.clone() {
                    Adt1::C0(h421, _, _, _, _, _, _, _, _, _, _) => {
                        h421
                    }
                };
                let a423 = 26u64;
                if if nat_eq(a422, a423) {
                    f36(v3.clone(), v12.clone())
                } else {
                    false
                } {
                    f24(v12, v7.clone())?
                } else {
                    false
                }
            }
        }
    } else {
        false
    } {
        Ok(Adt7::C5(Adt4::C0(v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, {
            let a424 = v8;
            let a425 = 2u64;
            if nat_eq(a424, a425) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m426 = v5;
            match m426 {
                None => {
                    let m427 = v4;
                    match m427 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h428, _, _, _, _, _, _, _, _, _) => {
                                    h428
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h429, _, _, _, _, _, _, _, _, _) => {
                                    h429
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h430, _, _, _, _, _, _, _, _) => {
                            h430
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
        Adt4::C0(h431, _, _, _, _, _, _, _, _) => {
            h431
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h432, _, _, _, _, _, _, _) => {
            h432
        }
    };
    let a434 = v1;
    let a435 = match v5.clone() {
        Adt1::C0(_, _, h433, _, _, _, _, _, _, _, _) => {
            h433
        }
    };
    if if if nat_lt(a434, a435) {
        let a437 = v2;
        let a438 = match v5.clone() {
            Adt1::C0(_, _, _, h436, _, _, _, _, _, _, _) => {
                h436
            }
        };
        nat_lt(a437, a438)
    } else {
        false
    } {
        let a440 = v3;
        let a441 = match v5.clone() {
            Adt1::C0(_, _, _, _, h439, _, _, _, _, _, _) => {
                h439
            }
        };
        nat_lt(a440, a441)
    } else {
        false
    } {
        Ok(Adt7::C6(f17(v4.clone(), {
            let a443 = v1;
            let a444 = match v4.clone() {
                Adt1::C0(_, _, h442, _, _, _, _, _, _, _, _) => {
                    h442
                }
            };
            let a445 = 0u64;
            nat_rem(a443, a444, a445)
        }, {
            let a447 = v2;
            let a448 = match v4.clone() {
                Adt1::C0(_, _, _, h446, _, _, _, _, _, _, _) => {
                    h446
                }
            };
            let a449 = 0u64;
            nat_rem(a447, a448, a449)
        }, {
            let a451 = v3;
            let a452 = match v4 {
                Adt1::C0(_, _, _, _, h450, _, _, _, _, _, _) => {
                    h450
                }
            };
            let a453 = 0u64;
            nat_rem(a451, a452, a453)
        })?, f17(v5, v1, v2, v3)?, {
            let m455 = match v0.clone() {
                Adt4::C0(_, _, h454, _, _, _, _, _, _) => {
                    h454
                }
            };
            match m455 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f17(v6, v1, v2, v3)?
                }
            }
        }, {
            let m457 = match v0.clone() {
                Adt4::C0(_, _, _, _, h456, _, _, _, _) => {
                    h456
                }
            };
            match m457 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f17(v7, v1, v2, v3)?
                }
            }
        }, f17(match v0 {
            Adt4::C0(_, _, _, _, _, h458, _, _, _) => {
                h458
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f13(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a459 = v1;
    let a460 = 0u64;
    if nat_eq(a459, a460) {
        let a464 = v2;
        let a465 = {
            let m463 = match v0.clone() {
                Adt4::C0(_, h462, _, _, _, _, _, _, _) => {
                    h462
                }
            };
            match m463 {
                Adt1::C0(_, h461, _, _, _, _, _, _, _, _, _) => {
                    h461
                }
            }
        };
        if nat_lt(a464, a465) {
            Ok(Adt7::C7({
                let a469 = v2;
                let a470 = {
                    let m468 = match v0 {
                        Adt4::C0(_, h467, _, _, _, _, _, _, _) => {
                            h467
                        }
                    };
                    match m468 {
                        Adt1::C0(_, _, _, _, _, h466, _, _, _, _, _) => {
                            h466
                        }
                    }
                };
                nat_mul(a469, a470)?
            }))
        } else {
            Ok(Adt7::C8)
        }
    } else {
        let a471 = v1;
        let a472 = 1u64;
        if nat_eq(a471, a472) {
            let m474 = match v0 {
                Adt4::C0(_, _, _, _, h473, _, _, _, _) => {
                    h473
                }
            };
            match m474 {
                None => {
                    Ok(Adt7::C8)
                }
                Some(v4) => {
                    let a476 = v2;
                    let a477 = match v4.clone() {
                        Adt1::C0(_, h475, _, _, _, _, _, _, _, _, _) => {
                            h475
                        }
                    };
                    if nat_lt(a476, a477) {
                        Ok(Adt7::C7({
                            let a479 = v2;
                            let a480 = match v4 {
                                Adt1::C0(_, _, _, _, _, h478, _, _, _, _, _) => {
                                    h478
                                }
                            };
                            nat_mul(a479, a480)?
                        }))
                    } else {
                        Ok(Adt7::C8)
                    }
                }
            }
        } else {
            let a481 = v1;
            let a482 = 2u64;
            if nat_eq(a481, a482) {
                let a483 = v2;
                let a484 = 2147483648u64;
                if if nat_lt(a483, a484) {
                    let a486 = v2;
                    let a487 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h485) => {
                            h485
                        }
                    };
                    nat_lt(a486, a487)
                } else {
                    false
                } {
                    let m489 = match v0 {
                        Adt4::C0(_, _, h488, _, _, _, _, _, _) => {
                            h488
                        }
                    };
                    match m489 {
                        None => {
                            Ok(Adt7::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt7::C7({
                                let a491 = v2;
                                let a492 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h490, _, _, _, _, _) => {
                                        h490
                                    }
                                };
                                nat_mul(a491, a492)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt7::C8)
                }
            } else {
                let a493 = v1;
                let a494 = 3u64;
                if nat_eq(a493, a494) {
                    let m496 = match v0 {
                        Adt4::C0(_, _, _, h495, _, _, _, _, _) => {
                            h495
                        }
                    };
                    match m496 {
                        None => {
                            Ok(Adt7::C8)
                        }
                        Some(v6) => {
                            let a498 = v2;
                            let a499 = match v6.clone() {
                                Adt1::C0(_, _, h497, _, _, _, _, _, _, _, _) => {
                                    h497
                                }
                            };
                            if if nat_lt(a498, a499) {
                                let a501 = v3;
                                let a502 = match v6.clone() {
                                    Adt1::C0(_, h500, _, _, _, _, _, _, _, _, _) => {
                                        h500
                                    }
                                };
                                nat_lt(a501, a502)
                            } else {
                                false
                            } {
                                Ok(Adt7::C7({
                                    let a509 = {
                                        let a504 = v2;
                                        let a505 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h503, _, _, _, _) => {
                                                h503
                                            }
                                        };
                                        nat_mul(a504, a505)?
                                    };
                                    let a510 = {
                                        let a507 = v3;
                                        let a508 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h506, _, _, _, _, _) => {
                                                h506
                                            }
                                        };
                                        nat_mul(a507, a508)?
                                    };
                                    nat_add(a509, a510)?
                                }))
                            } else {
                                Ok(Adt7::C8)
                            }
                        }
                    }
                } else {
                    let a511 = v1;
                    let a512 = 4u64;
                    if nat_eq(a511, a512) {
                        let a513 = v2;
                        let a514 = 2147483648u64;
                        if if nat_lt(a513, a514) {
                            let a518 = v2;
                            let a519 = {
                                let m517 = match v0.clone() {
                                    Adt4::C0(h516, _, _, _, _, _, _, _, _) => {
                                        h516
                                    }
                                };
                                match m517 {
                                    Adt1::C0(_, h515, _, _, _, _, _, _, _, _, _) => {
                                        h515
                                    }
                                }
                            };
                            nat_lt(a518, a519)
                        } else {
                            false
                        } {
                            Ok(Adt7::C7({
                                let a521 = v2;
                                let a522 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h520, _, _) => {
                                        h520
                                    }
                                };
                                nat_mul(a521, a522)?
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
        let a523 = v0;
        let a524 = 43u64;
        if if if if if if if if nat_eq(a523, a524) {
            let a526 = match v4.clone() {
                Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                    h525
                }
            };
            let a527 = 0u64;
            if nat_eq(a526, a527) {
                f21(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a529 = match v7.clone() {
                Adt1::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                    h528
                }
            };
            let a530 = 0u64;
            if nat_eq(a529, a530) {
                f21(v7.clone(), 4u64)?
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
            if f24(v4.clone(), v7.clone())? {
                true
            } else {
                f38(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f39(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f40(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f29(v3, match v4.clone() {
                Adt1::C0(_, _, _, h531, _, _, _, _, _, _, _) => {
                    h531
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt7::C3(Adt3::C0(v4, v5, v7, v2, f34(v3))))
        } else {
            Ok(Adt7::C0)
        }
    }
}

pub fn f15(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h532, _, _, _, _) => {
            h532
        }
    };
    let a534 = v1;
    let a535 = match v4.clone() {
        Adt1::C0(_, _, h533, _, _, _, _, _, _, _, _) => {
            h533
        }
    };
    if if if nat_lt(a534, a535) {
        let a537 = v2;
        let a538 = match v4.clone() {
            Adt1::C0(_, _, _, h536, _, _, _, _, _, _, _) => {
                h536
            }
        };
        nat_lt(a537, a538)
    } else {
        false
    } {
        let a540 = v3;
        let a541 = match v4.clone() {
            Adt1::C0(_, _, _, _, h539, _, _, _, _, _, _) => {
                h539
            }
        };
        nat_lt(a540, a541)
    } else {
        false
    } {
        let v6: u64 = {
            let m543 = match v0.clone() {
                Adt3::C0(_, h542, _, _, _) => {
                    h542
                }
            };
            match m543 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f17(v5.clone(), v1, {
                        let a545 = v2;
                        let a546 = match v5.clone() {
                            Adt1::C0(_, _, _, h544, _, _, _, _, _, _, _) => {
                                h544
                            }
                        };
                        let a547 = 0u64;
                        nat_rem(a545, a546, a547)
                    }, {
                        let a549 = v3;
                        let a550 = match v5 {
                            Adt1::C0(_, _, _, _, h548, _, _, _, _, _, _) => {
                                h548
                            }
                        };
                        let a551 = 0u64;
                        nat_rem(a549, a550, a551)
                    })?
                }
            }
        };
        Ok(Adt7::C4(f17(v4, v1, v2, v3)?, v6, {
            let a552 = v2;
            let a553 = 4u64;
            nat_mul(a552, a553)?
        }, f17(match v0 {
            Adt3::C0(_, _, h554, _, _) => {
                h554
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f16(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a558 = v1;
    let a559 = {
        let m557 = match v0.clone() {
            Adt2::C0(h556, _, _, _, _, _, _, _, _) => {
                h556
            }
        };
        match m557 {
            Adt1::C0(_, _, h555, _, _, _, _, _, _, _, _) => {
                h555
            }
        }
    };
    if if if nat_lt(a558, a559) {
        let a563 = v2;
        let a564 = {
            let m562 = match v0.clone() {
                Adt2::C0(h561, _, _, _, _, _, _, _, _) => {
                    h561
                }
            };
            match m562 {
                Adt1::C0(_, _, _, h560, _, _, _, _, _, _, _) => {
                    h560
                }
            }
        };
        nat_lt(a563, a564)
    } else {
        false
    } {
        let a568 = v3;
        let a569 = {
            let m567 = match v0.clone() {
                Adt2::C0(h566, _, _, _, _, _, _, _, _) => {
                    h566
                }
            };
            match m567 {
                Adt1::C0(_, _, _, _, h565, _, _, _, _, _, _) => {
                    h565
                }
            }
        };
        nat_lt(a568, a569)
    } else {
        false
    } {
        Ok(Adt7::C2(f17(match v0.clone() {
            Adt2::C0(h570, _, _, _, _, _, _, _, _) => {
                h570
            }
        }, v1, v2, v3)?, f17(match v0.clone() {
            Adt2::C0(_, h571, _, _, _, _, _, _, _) => {
                h571
            }
        }, 0u64, {
            let a581 = v2;
            let a582 = {
                let a578 = {
                    let m574 = match v0.clone() {
                        Adt2::C0(h573, _, _, _, _, _, _, _, _) => {
                            h573
                        }
                    };
                    match m574 {
                        Adt1::C0(_, _, _, h572, _, _, _, _, _, _, _) => {
                            h572
                        }
                    }
                };
                let a579 = {
                    let m577 = match v0.clone() {
                        Adt2::C0(_, h576, _, _, _, _, _, _, _) => {
                            h576
                        }
                    };
                    match m577 {
                        Adt1::C0(_, _, _, h575, _, _, _, _, _, _, _) => {
                            h575
                        }
                    }
                };
                let a580 = 0u64;
                nat_quot(a578, a579, a580)
            };
            let a583 = 0u64;
            nat_quot(a581, a582, a583)
        }, {
            let a593 = v3;
            let a594 = {
                let a590 = {
                    let m586 = match v0.clone() {
                        Adt2::C0(h585, _, _, _, _, _, _, _, _) => {
                            h585
                        }
                    };
                    match m586 {
                        Adt1::C0(_, _, _, _, h584, _, _, _, _, _, _) => {
                            h584
                        }
                    }
                };
                let a591 = {
                    let m589 = match v0.clone() {
                        Adt2::C0(_, h588, _, _, _, _, _, _, _) => {
                            h588
                        }
                    };
                    match m589 {
                        Adt1::C0(_, _, _, _, h587, _, _, _, _, _, _) => {
                            h587
                        }
                    }
                };
                let a592 = 0u64;
                nat_quot(a590, a591, a592)
            };
            let a595 = 0u64;
            nat_quot(a593, a594, a595)
        })?, f17(match v0.clone() {
            Adt2::C0(_, _, h596, _, _, _, _, _, _) => {
                h596
            }
        }, 0u64, {
            let a606 = v2;
            let a607 = {
                let a603 = {
                    let m599 = match v0.clone() {
                        Adt2::C0(h598, _, _, _, _, _, _, _, _) => {
                            h598
                        }
                    };
                    match m599 {
                        Adt1::C0(_, _, _, h597, _, _, _, _, _, _, _) => {
                            h597
                        }
                    }
                };
                let a604 = {
                    let m602 = match v0.clone() {
                        Adt2::C0(_, _, h601, _, _, _, _, _, _) => {
                            h601
                        }
                    };
                    match m602 {
                        Adt1::C0(_, _, _, h600, _, _, _, _, _, _, _) => {
                            h600
                        }
                    }
                };
                let a605 = 0u64;
                nat_quot(a603, a604, a605)
            };
            let a608 = 0u64;
            nat_quot(a606, a607, a608)
        }, {
            let a618 = v3;
            let a619 = {
                let a615 = {
                    let m611 = match v0.clone() {
                        Adt2::C0(h610, _, _, _, _, _, _, _, _) => {
                            h610
                        }
                    };
                    match m611 {
                        Adt1::C0(_, _, _, _, h609, _, _, _, _, _, _) => {
                            h609
                        }
                    }
                };
                let a616 = {
                    let m614 = match v0.clone() {
                        Adt2::C0(_, _, h613, _, _, _, _, _, _) => {
                            h613
                        }
                    };
                    match m614 {
                        Adt1::C0(_, _, _, _, h612, _, _, _, _, _, _) => {
                            h612
                        }
                    }
                };
                let a617 = 0u64;
                nat_quot(a615, a616, a617)
            };
            let a620 = 0u64;
            nat_quot(a618, a619, a620)
        })?, {
            let m622 = match v0.clone() {
                Adt2::C0(_, _, _, h621, _, _, _, _, _) => {
                    h621
                }
            };
            match m622 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f17(v4.clone(), v1, {
                        let a624 = v2;
                        let a625 = match v4.clone() {
                            Adt1::C0(_, _, _, h623, _, _, _, _, _, _, _) => {
                                h623
                            }
                        };
                        let a626 = 0u64;
                        nat_rem(a624, a625, a626)
                    }, {
                        let a628 = v3;
                        let a629 = match v4 {
                            Adt1::C0(_, _, _, _, h627, _, _, _, _, _, _) => {
                                h627
                            }
                        };
                        let a630 = 0u64;
                        nat_rem(a628, a629, a630)
                    })?
                }
            }
        }, {
            let a631 = v2;
            let a632 = 4u64;
            nat_mul(a631, a632)?
        }, f17(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h633, _, _, _) => {
                h633
            }
        }, v2, v1, v3)?, {
            let m636 = match v0.clone() {
                Adt2::C0(_, h635, _, _, _, _, _, _, _) => {
                    h635
                }
            };
            match m636 {
                Adt1::C0(_, _, _, _, _, _, h634, _, _, _, _) => {
                    h634
                }
            }
        }, {
            let m639 = match v0.clone() {
                Adt2::C0(_, _, h638, _, _, _, _, _, _) => {
                    h638
                }
            };
            match m639 {
                Adt1::C0(_, _, _, _, _, _, h637, _, _, _, _) => {
                    h637
                }
            }
        }, f41({
            let m642 = match v0 {
                Adt2::C0(_, h641, _, _, _, _, _, _, _) => {
                    h641
                }
            };
            match m642 {
                Adt1::C0(h640, _, _, _, _, _, _, _, _, _, _) => {
                    h640
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f17(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a654 = {
        let a649 = {
            let a644 = v1;
            let a645 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h643, _, _, _, _) => {
                    h643
                }
            };
            nat_mul(a644, a645)?
        };
        let a650 = {
            let a647 = v2;
            let a648 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h646, _, _, _) => {
                    h646
                }
            };
            nat_mul(a647, a648)?
        };
        nat_add(a649, a650)?
    };
    let a655 = {
        let a652 = v3;
        let a653 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h651, _, _) => {
                h651
            }
        };
        nat_mul(a652, a653)?
    };
    nat_add(a654, a655)
}

pub fn f18(v0: u64, v1: u64) -> bool {
    let a656 = 0u64;
    let a657 = v1;
    if nat_lt(a656, a657) {
        let a661 = {
            let a658 = v0;
            let a659 = v1;
            let a660 = 0u64;
            nat_rem(a658, a659, a660)
        };
        let a662 = 0u64;
        nat_eq(a661, a662)
    } else {
        false
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> R<bool> {
    let a664 = match v0.clone() {
        Adt1::C0(h663, _, _, _, _, _, _, _, _, _, _) => {
            h663
        }
    };
    let a665 = 0u64;
    if if nat_eq(a664, a665) {
        let a667 = match v0.clone() {
            Adt1::C0(_, h666, _, _, _, _, _, _, _, _, _) => {
                h666
            }
        };
        let a668 = 1u64;
        if nat_eq(a667, a668) {
            true
        } else {
            let a670 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h669, _, _, _, _, _) => {
                    h669
                }
            };
            let a671 = 4u64;
            nat_eq(a670, a671)
        }
    } else {
        false
    } {
        f24(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f20(v0: Adt1) -> R<bool> {
    let a673 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h672, _, _, _, _, _) => {
            h672
        }
    };
    let a674 = 4u64;
    if nat_eq(a673, a674) {
        let a679 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h675, _, _, _, _) => {
                h675
            }
        };
        let a680 = {
            let a677 = 4u64;
            let a678 = match v0 {
                Adt1::C0(_, h676, _, _, _, _, _, _, _, _, _) => {
                    h676
                }
            };
            nat_mul(a677, a678)?
        };
        Ok(nat_eq(a679, a680))
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: u64) -> R<bool> {
    let a682 = match v0.clone() {
        Adt1::C0(_, h681, _, _, _, _, _, _, _, _, _) => {
            h681
        }
    };
    let a683 = 1u64;
    if if if if nat_eq(a682, a683) {
        true
    } else {
        let a685 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h684, _, _, _, _, _) => {
                h684
            }
        };
        let a686 = v1;
        nat_eq(a685, a686)
    } {
        let a688 = match v0.clone() {
            Adt1::C0(_, _, h687, _, _, _, _, _, _, _, _) => {
                h687
            }
        };
        let a689 = 1u64;
        if nat_eq(a688, a689) {
            true
        } else {
            let a694 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h690, _, _, _, _) => {
                    h690
                }
            };
            let a695 = {
                let a692 = v1;
                let a693 = match v0.clone() {
                    Adt1::C0(_, h691, _, _, _, _, _, _, _, _, _) => {
                        h691
                    }
                };
                nat_mul(a692, a693)?
            };
            nat_eq(a694, a695)
        }
    } else {
        false
    } {
        let a697 = match v0.clone() {
            Adt1::C0(_, _, _, h696, _, _, _, _, _, _, _) => {
                h696
            }
        };
        let a698 = 1u64;
        if nat_eq(a697, a698) {
            true
        } else {
            let a706 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h699, _, _, _) => {
                    h699
                }
            };
            let a707 = {
                let a704 = {
                    let a701 = v1;
                    let a702 = match v0.clone() {
                        Adt1::C0(_, h700, _, _, _, _, _, _, _, _, _) => {
                            h700
                        }
                    };
                    nat_mul(a701, a702)?
                };
                let a705 = match v0.clone() {
                    Adt1::C0(_, _, h703, _, _, _, _, _, _, _, _) => {
                        h703
                    }
                };
                nat_mul(a704, a705)?
            };
            nat_eq(a706, a707)
        }
    } else {
        false
    } {
        let a709 = match v0.clone() {
            Adt1::C0(_, _, _, _, h708, _, _, _, _, _, _) => {
                h708
            }
        };
        let a710 = 1u64;
        if nat_eq(a709, a710) {
            Ok(true)
        } else {
            let a721 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h711, _, _) => {
                    h711
                }
            };
            let a722 = {
                let a719 = {
                    let a716 = {
                        let a713 = v1;
                        let a714 = match v0.clone() {
                            Adt1::C0(_, h712, _, _, _, _, _, _, _, _, _) => {
                                h712
                            }
                        };
                        nat_mul(a713, a714)?
                    };
                    let a717 = match v0.clone() {
                        Adt1::C0(_, _, h715, _, _, _, _, _, _, _, _) => {
                            h715
                        }
                    };
                    nat_mul(a716, a717)?
                };
                let a720 = match v0 {
                    Adt1::C0(_, _, _, h718, _, _, _, _, _, _, _) => {
                        h718
                    }
                };
                nat_mul(a719, a720)?
            };
            Ok(nat_eq(a721, a722))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    let a724 = match v0.clone() {
        Adt1::C0(_, h723, _, _, _, _, _, _, _, _, _) => {
            h723
        }
    };
    let a725 = v1;
    if if if nat_eq(a724, a725) {
        let a727 = match v0.clone() {
            Adt1::C0(_, _, h726, _, _, _, _, _, _, _, _) => {
                h726
            }
        };
        let a728 = v2;
        nat_eq(a727, a728)
    } else {
        false
    } {
        let a730 = match v0.clone() {
            Adt1::C0(_, _, _, h729, _, _, _, _, _, _, _) => {
                h729
            }
        };
        let a731 = v3;
        nat_eq(a730, a731)
    } else {
        false
    } {
        let a733 = match v0 {
            Adt1::C0(_, _, _, _, h732, _, _, _, _, _, _) => {
                h732
            }
        };
        let a734 = v4;
        nat_eq(a733, a734)
    } else {
        false
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    let a737 = match v0.clone() {
        Adt1::C0(_, h735, _, _, _, _, _, _, _, _, _) => {
            h735
        }
    };
    let a738 = match v1.clone() {
        Adt1::C0(_, h736, _, _, _, _, _, _, _, _, _) => {
            h736
        }
    };
    if if if nat_eq(a737, a738) {
        let a741 = match v0.clone() {
            Adt1::C0(_, _, h739, _, _, _, _, _, _, _, _) => {
                h739
            }
        };
        let a742 = match v1.clone() {
            Adt1::C0(_, _, h740, _, _, _, _, _, _, _, _) => {
                h740
            }
        };
        nat_eq(a741, a742)
    } else {
        false
    } {
        let a745 = match v0.clone() {
            Adt1::C0(_, _, _, h743, _, _, _, _, _, _, _) => {
                h743
            }
        };
        let a746 = match v1.clone() {
            Adt1::C0(_, _, _, h744, _, _, _, _, _, _, _) => {
                h744
            }
        };
        nat_eq(a745, a746)
    } else {
        false
    } {
        let a749 = match v0 {
            Adt1::C0(_, _, _, _, h747, _, _, _, _, _, _) => {
                h747
            }
        };
        let a750 = match v1 {
            Adt1::C0(_, _, _, _, h748, _, _, _, _, _, _) => {
                h748
            }
        };
        nat_eq(a749, a750)
    } else {
        false
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> R<bool> {
    let a752 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h751, _) => {
            h751
        }
    };
    let a753 = 0u64;
    if nat_eq(a752, a753) {
        Ok(true)
    } else {
        let a755 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h754, _) => {
                h754
            }
        };
        let a756 = 0u64;
        if nat_eq(a755, a756) {
            Ok(true)
        } else {
            let a758 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h757) => {
                    h757
                }
            };
            let a759 = 0u64;
            if nat_eq(a758, a759) {
                Ok(true)
            } else {
                let a761 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h760) => {
                        h760
                    }
                };
                let a762 = 0u64;
                if nat_eq(a761, a762) {
                    Ok(true)
                } else {
                    let a768 = {
                        let a765 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h763) => {
                                h763
                            }
                        };
                        let a766 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h764, _) => {
                                h764
                            }
                        };
                        nat_add(a765, a766)?
                    };
                    let a769 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h767) => {
                            h767
                        }
                    };
                    if nat_le(a768, a769) {
                        Ok(true)
                    } else {
                        let a775 = {
                            let a772 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h770) => {
                                    h770
                                }
                            };
                            let a773 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h771, _) => {
                                    h771
                                }
                            };
                            nat_add(a772, a773)?
                        };
                        let a776 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h774) => {
                                h774
                            }
                        };
                        Ok(nat_le(a775, a776))
                    }
                }
            }
        }
    }
}

pub fn f25(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h777, _, _, _, _, _, _, _, _, _) => {
            h777
        }
    };
    let v5: u64 = match v0.clone() {
        Adt1::C0(_, _, h778, _, _, _, _, _, _, _, _) => {
            h778
        }
    };
    let v6: u64 = match v0.clone() {
        Adt1::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
            h779
        }
    };
    let v7: u64 = match v0 {
        Adt1::C0(_, _, _, _, h780, _, _, _, _, _, _) => {
            h780
        }
    };
    let a801 = 4u64;
    let a802 = {
        let a799 = {
            let a785 = {
                let a781 = v4;
                let a782 = v5;
                nat_mul(a781, a782)?
            };
            let a786 = {
                let a783 = v6;
                let a784 = v7;
                nat_mul(a783, a784)?
            };
            nat_mul(a785, a786)?
        };
        let a800 = {
            let a797 = {
                let a787 = v4;
                let a788 = v4;
                nat_mul(a787, a788)?
            };
            let a798 = {
                let a795 = v1;
                let a796 = {
                    let a793 = v5;
                    let a794 = {
                        let a791 = v2;
                        let a792 = {
                            let a789 = v7;
                            let a790 = v3;
                            nat_mul(a789, a790)?
                        };
                        nat_add(a791, a792)?
                    };
                    nat_mul(a793, a794)?
                };
                nat_add(a795, a796)?
            };
            nat_mul(a797, a798)?
        };
        nat_add(a799, a800)?
    };
    nat_mul(a801, a802)
}

pub fn f26(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt7> {
    let v8: u64 = match v3.clone() {
        Adt1::C0(_, h803, _, _, _, _, _, _, _, _, _) => {
            h803
        }
    };
    let v9: u64 = match v3.clone() {
        Adt1::C0(_, _, h804, _, _, _, _, _, _, _, _) => {
            h804
        }
    };
    let v10: u64 = match v3.clone() {
        Adt1::C0(_, _, _, h805, _, _, _, _, _, _, _) => {
            h805
        }
    };
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, _, _, _, h806, _, _, _, _, _, _) => {
            h806
        }
    };
    let a807 = 0u64;
    let a808 = v8;
    if if if if if if if if if if if if if if if if if if if if if if if nat_lt(a807, a808) {
        let a809 = 0u64;
        let a810 = v9;
        nat_lt(a809, a810)
    } else {
        false
    } {
        let a811 = 0u64;
        let a812 = v11;
        nat_lt(a811, a812)
    } else {
        false
    } {
        let a813 = 0u64;
        let a814 = v0;
        nat_lt(a813, a814)
    } else {
        false
    } {
        let a815 = v0;
        let a816 = 2147483647u64;
        nat_le(a815, a816)
    } else {
        false
    } {
        f19(v1.clone(), v7.clone())?
    } else {
        false
    } {
        f19(v2.clone(), v7.clone())?
    } else {
        false
    } {
        f19(v3.clone(), v7.clone())?
    } else {
        false
    } {
        f19(v4.clone(), v7.clone())?
    } else {
        false
    } {
        f19(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f19(v6.clone(), v7.clone())?
    } else {
        false
    } {
        f42(v1.clone(), v3.clone())
    } else {
        false
    } {
        f42(v2.clone(), v3.clone())
    } else {
        false
    } {
        let a818 = match v4.clone() {
            Adt1::C0(_, h817, _, _, _, _, _, _, _, _, _) => {
                h817
            }
        };
        let a819 = 1u64;
        if nat_eq(a818, a819) {
            true
        } else {
            let a821 = match v4.clone() {
                Adt1::C0(_, h820, _, _, _, _, _, _, _, _, _) => {
                    h820
                }
            };
            let a822 = v8;
            nat_eq(a821, a822)
        }
    } else {
        false
    } {
        let a825 = match v4.clone() {
            Adt1::C0(_, h823, _, _, _, _, _, _, _, _, _) => {
                h823
            }
        };
        let a826 = match v4.clone() {
            Adt1::C0(_, h824, _, _, _, _, _, _, _, _, _) => {
                h824
            }
        };
        if if if nat_eq(a825, a826) {
            let a828 = match v4.clone() {
                Adt1::C0(_, _, h827, _, _, _, _, _, _, _, _) => {
                    h827
                }
            };
            let a829 = v9;
            nat_eq(a828, a829)
        } else {
            false
        } {
            let a831 = match v4.clone() {
                Adt1::C0(_, _, _, h830, _, _, _, _, _, _, _) => {
                    h830
                }
            };
            let a832 = v10;
            nat_eq(a831, a832)
        } else {
            false
        } {
            let a834 = match v4.clone() {
                Adt1::C0(_, _, _, _, h833, _, _, _, _, _, _) => {
                    h833
                }
            };
            let a835 = v11;
            nat_eq(a834, a835)
        } else {
            false
        }
    } else {
        false
    } {
        let a837 = match v5.clone() {
            Adt1::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                h836
            }
        };
        let a838 = 1u64;
        if if if nat_eq(a837, a838) {
            let a840 = match v5.clone() {
                Adt1::C0(_, _, h839, _, _, _, _, _, _, _, _) => {
                    h839
                }
            };
            let a841 = v9;
            nat_eq(a840, a841)
        } else {
            false
        } {
            let a843 = match v5.clone() {
                Adt1::C0(_, _, _, h842, _, _, _, _, _, _, _) => {
                    h842
                }
            };
            let a844 = v10;
            nat_eq(a843, a844)
        } else {
            false
        } {
            let a846 = match v5.clone() {
                Adt1::C0(_, _, _, _, h845, _, _, _, _, _, _) => {
                    h845
                }
            };
            let a847 = v11;
            nat_eq(a846, a847)
        } else {
            false
        }
    } else {
        false
    } {
        let a849 = match v6.clone() {
            Adt1::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a850 = v8;
        if if if nat_eq(a849, a850) {
            let a852 = match v6.clone() {
                Adt1::C0(_, _, h851, _, _, _, _, _, _, _, _) => {
                    h851
                }
            };
            let a853 = v8;
            nat_eq(a852, a853)
        } else {
            false
        } {
            let a855 = match v6.clone() {
                Adt1::C0(_, _, _, h854, _, _, _, _, _, _, _) => {
                    h854
                }
            };
            let a856 = v9;
            nat_eq(a855, a856)
        } else {
            false
        } {
            let a858 = match v6.clone() {
                Adt1::C0(_, _, _, _, h857, _, _, _, _, _, _) => {
                    h857
                }
            };
            let a859 = v11;
            nat_eq(a858, a859)
        } else {
            false
        }
    } else {
        false
    } {
        let a863 = match v7.clone() {
            Adt1::C0(_, h860, _, _, _, _, _, _, _, _, _) => {
                h860
            }
        };
        let a864 = {
            let a861 = v8;
            let a862 = v9;
            nat_mul(a861, a862)?
        };
        if if if nat_eq(a863, a864) {
            let a874 = match v7.clone() {
                Adt1::C0(_, _, h865, _, _, _, _, _, _, _, _) => {
                    h865
                }
            };
            let a875 = {
                let a872 = {
                    let a866 = v10;
                    let a867 = v11;
                    nat_mul(a866, a867)?
                };
                let a873 = {
                    let a870 = {
                        let a868 = v0;
                        let a869 = v8;
                        nat_mul(a868, a869)?
                    };
                    let a871 = v11;
                    nat_mul(a870, a871)?
                };
                nat_add(a872, a873)?
            };
            nat_eq(a874, a875)
        } else {
            false
        } {
            let a877 = match v7.clone() {
                Adt1::C0(_, _, _, h876, _, _, _, _, _, _, _) => {
                    h876
                }
            };
            let a878 = 1u64;
            nat_eq(a877, a878)
        } else {
            false
        } {
            let a880 = match v7.clone() {
                Adt1::C0(_, _, _, _, h879, _, _, _, _, _, _) => {
                    h879
                }
            };
            let a881 = 1u64;
            nat_eq(a880, a881)
        } else {
            false
        }
    } else {
        false
    } {
        let a883 = match v7.clone() {
            Adt1::C0(h882, _, _, _, _, _, _, _, _, _, _) => {
                h882
            }
        };
        let a884 = 0u64;
        nat_eq(a883, a884)
    } else {
        false
    } {
        f21(v4.clone(), 4u64)?
    } else {
        false
    } {
        f21(v5.clone(), 4u64)?
    } else {
        false
    } {
        f21(v6.clone(), 4u64)?
    } else {
        false
    } {
        f21(v7, 4u64)?
    } else {
        false
    } {
        let v12: u64 = {
            let a885 = v9;
            let a886 = v11;
            nat_mul(a885, a886)?
        };
        let v13: u64 = {
            let a887 = v8;
            let a888 = v8;
            nat_mul(a887, a888)?
        };
        let v14: u64 = {
            let a889 = v10;
            let a890 = 0u64;
            if if nat_eq(a889, a890) {
                let a891 = v0;
                let a892 = 1u64;
                nat_eq(a891, a892)
            } else {
                false
            } {
                1u64
            } else {
                let a893 = v0;
                let a894 = v10;
                if nat_lt(a893, a894) {
                    v0
                } else {
                    v10
                }
            }
        };
        let v15: u64 = {
            let a901 = v12;
            let a902 = {
                let a899 = {
                    let a895 = v10;
                    let a896 = v8;
                    nat_mul(a895, a896)?
                };
                let a900 = {
                    let a897 = v14;
                    let a898 = v13;
                    nat_mul(a897, a898)?
                };
                nat_add(a899, a900)?
            };
            nat_mul(a901, a902)?
        };
        Ok(Adt7::C12(Adt6::C0(v1, v2, v3, v4, v5, v6, v0, v12, v13, {
            let a905 = {
                let a903 = v13;
                let a904 = v8;
                nat_add(a903, a904)?
            };
            let a906 = v8;
            nat_add(a905, a906)?
        }, v15, false)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f27(v0: bool, v1: Adt1, v2: Adt1, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Adt1) -> R<Adt7> {
    let v8: Adt1 = {
        let m907 = v3.clone();
        match m907 {
            None => {
                v1.clone()
            }
            Some(v7) => {
                v7
            }
        }
    };
    let v10: Adt1 = {
        let m908 = v4.clone();
        match m908 {
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
        let m909 = v5;
        match m909 {
            None => {
                let m910 = v3;
                match m910 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m911 = v4;
                        match m911 {
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
        let a913 = match v1.clone() {
            Adt1::C0(h912, _, _, _, _, _, _, _, _, _, _) => {
                h912
            }
        };
        let a914 = 0u64;
        if nat_eq(a913, a914) {
            let a916 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h915, _, _, _, _, _) => {
                    h915
                }
            };
            let a917 = 4u64;
            nat_eq(a916, a917)
        } else {
            false
        }
    } else {
        false
    } {
        let a919 = match v8.clone() {
            Adt1::C0(h918, _, _, _, _, _, _, _, _, _, _) => {
                h918
            }
        };
        let a920 = 0u64;
        if nat_eq(a919, a920) {
            let a922 = match v8.clone() {
                Adt1::C0(_, _, _, _, _, h921, _, _, _, _, _) => {
                    h921
                }
            };
            let a923 = 4u64;
            nat_eq(a922, a923)
        } else {
            false
        }
    } else {
        false
    } {
        let a925 = match v10.clone() {
            Adt1::C0(h924, _, _, _, _, _, _, _, _, _, _) => {
                h924
            }
        };
        let a926 = 1u64;
        if nat_eq(a925, a926) {
            let a928 = match v10.clone() {
                Adt1::C0(_, _, _, _, _, h927, _, _, _, _, _) => {
                    h927
                }
            };
            let a929 = 2u64;
            nat_eq(a928, a929)
        } else {
            false
        }
    } else {
        false
    } {
        f30(v6.clone())?
    } else {
        false
    } {
        f43(match v2.clone() {
            Adt1::C0(h930, _, _, _, _, _, _, _, _, _, _) => {
                h930
            }
        }, match v2.clone() {
            Adt1::C0(_, h931, _, _, _, _, _, _, _, _, _) => {
                h931
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h932, _, _, _, _, _) => {
                h932
            }
        })
    } else {
        false
    } {
        let a934 = 0u64;
        let a935 = match v1.clone() {
            Adt1::C0(_, h933, _, _, _, _, _, _, _, _, _) => {
                h933
            }
        };
        nat_lt(a934, a935)
    } else {
        false
    } {
        let a937 = 0u64;
        let a938 = match v1.clone() {
            Adt1::C0(_, _, h936, _, _, _, _, _, _, _, _) => {
                h936
            }
        };
        nat_lt(a937, a938)
    } else {
        false
    } {
        let a940 = 0u64;
        let a941 = match v1.clone() {
            Adt1::C0(_, _, _, h939, _, _, _, _, _, _, _) => {
                h939
            }
        };
        nat_lt(a940, a941)
    } else {
        false
    } {
        let a943 = 0u64;
        let a944 = match v1.clone() {
            Adt1::C0(_, _, _, _, h942, _, _, _, _, _, _) => {
                h942
            }
        };
        nat_lt(a943, a944)
    } else {
        false
    } {
        let a946 = 0u64;
        let a947 = match v2.clone() {
            Adt1::C0(_, _, _, h945, _, _, _, _, _, _, _) => {
                h945
            }
        };
        nat_lt(a946, a947)
    } else {
        false
    } {
        let a950 = match v2.clone() {
            Adt1::C0(_, h948, _, _, _, _, _, _, _, _, _) => {
                h948
            }
        };
        let a951 = match v1.clone() {
            Adt1::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        nat_eq(a950, a951)
    } else {
        false
    } {
        let a953 = match v2.clone() {
            Adt1::C0(_, _, h952, _, _, _, _, _, _, _, _) => {
                h952
            }
        };
        let a954 = 1u64;
        nat_eq(a953, a954)
    } else {
        false
    } {
        let a957 = match v2.clone() {
            Adt1::C0(_, _, _, _, h955, _, _, _, _, _, _) => {
                h955
            }
        };
        let a958 = match v1.clone() {
            Adt1::C0(_, _, _, _, h956, _, _, _, _, _, _) => {
                h956
            }
        };
        nat_eq(a957, a958)
    } else {
        false
    } {
        let a961 = match v8.clone() {
            Adt1::C0(_, h959, _, _, _, _, _, _, _, _, _) => {
                h959
            }
        };
        let a962 = match v1.clone() {
            Adt1::C0(_, _, h960, _, _, _, _, _, _, _, _) => {
                h960
            }
        };
        nat_eq(a961, a962)
    } else {
        false
    } {
        let a965 = match v8.clone() {
            Adt1::C0(_, _, h963, _, _, _, _, _, _, _, _) => {
                h963
            }
        };
        let a966 = match v1.clone() {
            Adt1::C0(_, _, _, h964, _, _, _, _, _, _, _) => {
                h964
            }
        };
        nat_eq(a965, a966)
    } else {
        false
    } {
        let a968 = match v8.clone() {
            Adt1::C0(_, _, _, h967, _, _, _, _, _, _, _) => {
                h967
            }
        };
        let a969 = 1u64;
        nat_eq(a968, a969)
    } else {
        false
    } {
        let a972 = match v8.clone() {
            Adt1::C0(_, _, _, _, h970, _, _, _, _, _, _) => {
                h970
            }
        };
        let a973 = match v1.clone() {
            Adt1::C0(_, _, _, _, h971, _, _, _, _, _, _) => {
                h971
            }
        };
        nat_eq(a972, a973)
    } else {
        false
    } {
        let a976 = match v10.clone() {
            Adt1::C0(_, h974, _, _, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a977 = match v2.clone() {
            Adt1::C0(_, _, _, h975, _, _, _, _, _, _, _) => {
                h975
            }
        };
        nat_eq(a976, a977)
    } else {
        false
    } {
        let a980 = match v10.clone() {
            Adt1::C0(_, _, h978, _, _, _, _, _, _, _, _) => {
                h978
            }
        };
        let a981 = match v1.clone() {
            Adt1::C0(_, _, _, h979, _, _, _, _, _, _, _) => {
                h979
            }
        };
        nat_eq(a980, a981)
    } else {
        false
    } {
        let a983 = match v10.clone() {
            Adt1::C0(_, _, _, h982, _, _, _, _, _, _, _) => {
                h982
            }
        };
        let a984 = 1u64;
        nat_eq(a983, a984)
    } else {
        false
    } {
        f18(match v1.clone() {
            Adt1::C0(_, _, _, _, h985, _, _, _, _, _, _) => {
                h985
            }
        }, match v10.clone() {
            Adt1::C0(_, _, _, _, h986, _, _, _, _, _, _) => {
                h986
            }
        })
    } else {
        false
    } {
        let a989 = match v6.clone() {
            Adt1::C0(_, h987, _, _, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a990 = match v2.clone() {
            Adt1::C0(_, _, _, h988, _, _, _, _, _, _, _) => {
                h988
            }
        };
        nat_eq(a989, a990)
    } else {
        false
    } {
        let a993 = match v6.clone() {
            Adt1::C0(_, _, h991, _, _, _, _, _, _, _, _) => {
                h991
            }
        };
        let a994 = match v1.clone() {
            Adt1::C0(_, _, _, h992, _, _, _, _, _, _, _) => {
                h992
            }
        };
        nat_eq(a993, a994)
    } else {
        false
    } {
        let a996 = match v6.clone() {
            Adt1::C0(_, _, _, h995, _, _, _, _, _, _, _) => {
                h995
            }
        };
        let a997 = 1u64;
        nat_eq(a996, a997)
    } else {
        false
    } {
        let a1000 = match v6.clone() {
            Adt1::C0(_, _, _, _, h998, _, _, _, _, _, _) => {
                h998
            }
        };
        let a1001 = match v1.clone() {
            Adt1::C0(_, _, _, _, h999, _, _, _, _, _, _) => {
                h999
            }
        };
        nat_eq(a1000, a1001)
    } else {
        false
    } {
        f24(v1.clone(), v6.clone())?
    } else {
        false
    } {
        f24(v2.clone(), v6.clone())?
    } else {
        false
    } {
        f24(v8.clone(), v6.clone())?
    } else {
        false
    } {
        f24(v10.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt7::C10(Adt5::C0(v1, v2, v8, v10, v6)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f28(v0: Option<Adt1>, v1: Option<Adt1>, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Option<u64>> {
    let m1002 = v0;
    match m1002 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h1003, _, _, _, _, _, _, _, _) => {
                    h1003
                }
            }))
        }
        Some(v6) => {
            let m1004 = v1;
            match m1004 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a1006 = match v6.clone() {
                        Adt1::C0(h1005, _, _, _, _, _, _, _, _, _, _) => {
                            h1005
                        }
                    };
                    let a1007 = 26u64;
                    if if if if if if if if if nat_eq(a1006, a1007) {
                        f21(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a1010 = match v6.clone() {
                            Adt1::C0(_, h1008, _, _, _, _, _, _, _, _, _) => {
                                h1008
                            }
                        };
                        let a1011 = match v7 {
                            Adt1::C0(_, h1009, _, _, _, _, _, _, _, _, _) => {
                                h1009
                            }
                        };
                        nat_eq(a1010, a1011)
                    } else {
                        false
                    } {
                        let a1014 = match v6.clone() {
                            Adt1::C0(_, _, h1012, _, _, _, _, _, _, _, _) => {
                                h1012
                            }
                        };
                        let a1015 = match v2 {
                            Adt1::C0(_, _, _, _, h1013, _, _, _, _, _, _) => {
                                h1013
                            }
                        };
                        nat_eq(a1014, a1015)
                    } else {
                        false
                    } {
                        let a1017 = match v6.clone() {
                            Adt1::C0(_, _, _, h1016, _, _, _, _, _, _, _) => {
                                h1016
                            }
                        };
                        let a1018 = 1u64;
                        nat_eq(a1017, a1018)
                    } else {
                        false
                    } {
                        let a1020 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h1019, _, _, _, _, _, _) => {
                                h1019
                            }
                        };
                        let a1021 = 1u64;
                        nat_eq(a1020, a1021)
                    } else {
                        false
                    } {
                        let a1023 = match v3 {
                            Adt1::C0(_, _, _, _, h1022, _, _, _, _, _, _) => {
                                h1022
                            }
                        };
                        let a1024 = 1u64;
                        nat_eq(a1023, a1024)
                    } else {
                        false
                    } {
                        let a1026 = match v4 {
                            Adt1::C0(_, _, _, _, h1025, _, _, _, _, _, _) => {
                                h1025
                            }
                        };
                        let a1027 = 1u64;
                        nat_eq(a1026, a1027)
                    } else {
                        false
                    } {
                        f24(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h1028, _, _, _, _, _, _, _, _, _) => {
                                h1028
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

pub fn f29(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a1029 = v0;
    let a1030 = 0u64;
    if if nat_eq(a1029, a1030) {
        true
    } else {
        let a1031 = 2147483648u64;
        let a1032 = v0;
        if nat_le(a1031, a1032) {
            let a1033 = v0;
            let a1034 = 4286578688u64;
            nat_lt(a1033, a1034)
        } else {
            false
        }
    } {
        true
    } else {
        let a1035 = v0;
        let a1036 = 2139095040u64;
        if if nat_lt(a1035, a1036) {
            let a1037 = v1;
            let a1038 = 4294967295u64;
            nat_le(a1037, a1038)
        } else {
            false
        } {
            let m1039 = v2;
            match m1039 {
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

pub fn f30(v0: Adt1) -> R<bool> {
    let a1041 = match v0.clone() {
        Adt1::C0(h1040, _, _, _, _, _, _, _, _, _, _) => {
            h1040
        }
    };
    let a1042 = 0u64;
    if if if if nat_eq(a1041, a1042) {
        let a1044 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1043, _, _, _, _, _) => {
                h1043
            }
        };
        let a1045 = 4u64;
        nat_eq(a1044, a1045)
    } else {
        false
    } {
        let a1051 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1046, _, _, _, _) => {
                h1046
            }
        };
        let a1052 = {
            let a1049 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1047, _, _, _, _, _) => {
                    h1047
                }
            };
            let a1050 = match v0.clone() {
                Adt1::C0(_, h1048, _, _, _, _, _, _, _, _, _) => {
                    h1048
                }
            };
            nat_mul(a1049, a1050)?
        };
        nat_eq(a1051, a1052)
    } else {
        false
    } {
        let a1058 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1053, _, _, _) => {
                h1053
            }
        };
        let a1059 = {
            let a1056 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1054, _, _, _, _) => {
                    h1054
                }
            };
            let a1057 = match v0.clone() {
                Adt1::C0(_, _, h1055, _, _, _, _, _, _, _, _) => {
                    h1055
                }
            };
            nat_mul(a1056, a1057)?
        };
        nat_eq(a1058, a1059)
    } else {
        false
    } {
        let a1065 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1060, _, _) => {
                h1060
            }
        };
        let a1066 = {
            let a1063 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1061, _, _, _) => {
                    h1061
                }
            };
            let a1064 = match v0 {
                Adt1::C0(_, _, _, h1062, _, _, _, _, _, _, _) => {
                    h1062
                }
            };
            nat_mul(a1063, a1064)?
        };
        Ok(nat_eq(a1065, a1066))
    } else {
        Ok(false)
    }
}

pub fn f31(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f43(match v0.clone() {
        Adt1::C0(h1067, _, _, _, _, _, _, _, _, _, _) => {
            h1067
        }
    }, match v0.clone() {
        Adt1::C0(_, h1068, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1069, _, _, _, _, _) => {
            h1069
        }
    }) {
        f18(match v1.clone() {
            Adt1::C0(_, _, _, h1070, _, _, _, _, _, _, _) => {
                h1070
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h1071, _, _, _, _, _, _, _) => {
                h1071
            }
        })
    } else {
        false
    } {
        f18(match v1 {
            Adt1::C0(_, _, _, _, h1072, _, _, _, _, _, _) => {
                h1072
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h1073, _, _, _, _, _, _) => {
                h1073
            }
        })
    } else {
        false
    } {
        f24(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f32(v0: Option<Adt1>, v1: Adt1, v2: u64, v3: Adt1) -> R<bool> {
    let m1074 = v0;
    match m1074 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a1076 = match v4.clone() {
                Adt1::C0(h1075, _, _, _, _, _, _, _, _, _, _) => {
                    h1075
                }
            };
            let a1077 = 1u64;
            if if if if if if nat_eq(a1076, a1077) {
                let a1079 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h1078, _, _, _, _, _) => {
                        h1078
                    }
                };
                let a1080 = 2u64;
                nat_eq(a1079, a1080)
            } else {
                false
            } {
                let a1082 = v2;
                let a1083 = match v4.clone() {
                    Adt1::C0(_, h1081, _, _, _, _, _, _, _, _, _) => {
                        h1081
                    }
                };
                nat_le(a1082, a1083)
            } else {
                false
            } {
                let a1086 = match v1.clone() {
                    Adt1::C0(_, _, h1084, _, _, _, _, _, _, _, _) => {
                        h1084
                    }
                };
                let a1087 = match v4.clone() {
                    Adt1::C0(_, _, h1085, _, _, _, _, _, _, _, _) => {
                        h1085
                    }
                };
                nat_le(a1086, a1087)
            } else {
                false
            } {
                f18(match v1.clone() {
                    Adt1::C0(_, _, _, h1088, _, _, _, _, _, _, _) => {
                        h1088
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h1089, _, _, _, _, _, _, _) => {
                        h1089
                    }
                })
            } else {
                false
            } {
                f18(match v1 {
                    Adt1::C0(_, _, _, _, h1090, _, _, _, _, _, _) => {
                        h1090
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h1091, _, _, _, _, _, _) => {
                        h1091
                    }
                })
            } else {
                false
            } {
                f24(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f33(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1092 = v0;
    match m1092 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1094 = match v3.clone() {
                Adt1::C0(h1093, _, _, _, _, _, _, _, _, _, _) => {
                    h1093
                }
            };
            let a1095 = 0u64;
            if if if if if if nat_eq(a1094, a1095) {
                let a1097 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h1096, _, _, _, _, _) => {
                        h1096
                    }
                };
                let a1098 = 4u64;
                nat_eq(a1097, a1098)
            } else {
                false
            } {
                let a1101 = match v3.clone() {
                    Adt1::C0(_, h1099, _, _, _, _, _, _, _, _, _) => {
                        h1099
                    }
                };
                let a1102 = match v1 {
                    Adt1::C0(_, _, _, h1100, _, _, _, _, _, _, _) => {
                        h1100
                    }
                };
                nat_eq(a1101, a1102)
            } else {
                false
            } {
                let a1104 = match v3.clone() {
                    Adt1::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                        h1103
                    }
                };
                let a1105 = 1u64;
                nat_eq(a1104, a1105)
            } else {
                false
            } {
                let a1107 = match v3.clone() {
                    Adt1::C0(_, _, _, h1106, _, _, _, _, _, _, _) => {
                        h1106
                    }
                };
                let a1108 = 1u64;
                nat_eq(a1107, a1108)
            } else {
                false
            } {
                let a1110 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1109, _, _, _, _, _, _) => {
                        h1109
                    }
                };
                let a1111 = 1u64;
                nat_eq(a1110, a1111)
            } else {
                false
            } {
                f24(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f34(v0: u64) -> Option<u64> {
    let a1112 = 0u64;
    let a1113 = v0;
    if if nat_lt(a1112, a1113) {
        let a1114 = v0;
        let a1115 = 2139095040u64;
        nat_lt(a1114, a1115)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f35(v0: u64) -> Option<u64> {
    let a1116 = v0;
    let a1117 = 0u64;
    if if nat_eq(a1116, a1117) {
        true
    } else {
        let a1118 = v0;
        let a1119 = 2147483648u64;
        nat_eq(a1118, a1119)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> bool {
    let a1122 = match v0.clone() {
        Adt1::C0(_, _, h1120, _, _, _, _, _, _, _, _) => {
            h1120
        }
    };
    let a1123 = match v1.clone() {
        Adt1::C0(_, _, h1121, _, _, _, _, _, _, _, _) => {
            h1121
        }
    };
    if if nat_eq(a1122, a1123) {
        let a1126 = match v0.clone() {
            Adt1::C0(_, _, _, h1124, _, _, _, _, _, _, _) => {
                h1124
            }
        };
        let a1127 = match v1.clone() {
            Adt1::C0(_, _, _, h1125, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        nat_eq(a1126, a1127)
    } else {
        false
    } {
        let a1130 = match v0 {
            Adt1::C0(_, _, _, _, h1128, _, _, _, _, _, _) => {
                h1128
            }
        };
        let a1131 = match v1 {
            Adt1::C0(_, _, _, _, h1129, _, _, _, _, _, _) => {
                h1129
            }
        };
        nat_eq(a1130, a1131)
    } else {
        false
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f44(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f44(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f44(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f44(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f44(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f44(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f38(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f23(v0.clone(), v1.clone()) {
        let a1134 = match v0.clone() {
            Adt1::C0(h1132, _, _, _, _, _, _, _, _, _, _) => {
                h1132
            }
        };
        let a1135 = match v1.clone() {
            Adt1::C0(h1133, _, _, _, _, _, _, _, _, _, _) => {
                h1133
            }
        };
        nat_eq(a1134, a1135)
    } else {
        false
    } {
        let a1138 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1136) => {
                h1136
            }
        };
        let a1139 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1137) => {
                h1137
            }
        };
        nat_eq(a1138, a1139)
    } else {
        false
    } {
        let a1142 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1140, _, _, _, _, _) => {
                h1140
            }
        };
        let a1143 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1141, _, _, _, _, _) => {
                h1141
            }
        };
        nat_eq(a1142, a1143)
    } else {
        false
    } {
        let a1146 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1144, _, _, _, _) => {
                h1144
            }
        };
        let a1147 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1145, _, _, _, _) => {
                h1145
            }
        };
        nat_eq(a1146, a1147)
    } else {
        false
    } {
        let a1150 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1148, _, _, _) => {
                h1148
            }
        };
        let a1151 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1149, _, _, _) => {
                h1149
            }
        };
        nat_eq(a1150, a1151)
    } else {
        false
    } {
        let a1154 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1152, _, _) => {
                h1152
            }
        };
        let a1155 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1153, _, _) => {
                h1153
            }
        };
        nat_eq(a1154, a1155)
    } else {
        false
    }
}

pub fn f39(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1156 = v0;
    match m1156 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1158 = match v3.clone() {
                Adt1::C0(h1157, _, _, _, _, _, _, _, _, _, _) => {
                    h1157
                }
            };
            let a1159 = 0u64;
            if if if if if if if nat_eq(a1158, a1159) {
                true
            } else {
                let a1161 = match v3.clone() {
                    Adt1::C0(h1160, _, _, _, _, _, _, _, _, _, _) => {
                        h1160
                    }
                };
                let a1162 = 1u64;
                nat_eq(a1161, a1162)
            } {
                f21(v3.clone(), {
                    let a1164 = match v3.clone() {
                        Adt1::C0(h1163, _, _, _, _, _, _, _, _, _, _) => {
                            h1163
                        }
                    };
                    let a1165 = 0u64;
                    if nat_eq(a1164, a1165) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a1168 = match v3.clone() {
                    Adt1::C0(_, h1166, _, _, _, _, _, _, _, _, _) => {
                        h1166
                    }
                };
                let a1169 = match v1.clone() {
                    Adt1::C0(_, h1167, _, _, _, _, _, _, _, _, _) => {
                        h1167
                    }
                };
                nat_eq(a1168, a1169)
            } else {
                false
            } {
                let a1172 = match v1.clone() {
                    Adt1::C0(_, _, h1170, _, _, _, _, _, _, _, _) => {
                        h1170
                    }
                };
                let a1173 = match v3.clone() {
                    Adt1::C0(_, _, h1171, _, _, _, _, _, _, _, _) => {
                        h1171
                    }
                };
                nat_le(a1172, a1173)
            } else {
                false
            } {
                f18(match v1.clone() {
                    Adt1::C0(_, _, _, h1174, _, _, _, _, _, _, _) => {
                        h1174
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h1175, _, _, _, _, _, _, _) => {
                        h1175
                    }
                })
            } else {
                false
            } {
                f18(match v1 {
                    Adt1::C0(_, _, _, _, h1176, _, _, _, _, _, _) => {
                        h1176
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h1177, _, _, _, _, _, _) => {
                        h1177
                    }
                })
            } else {
                false
            } {
                f24(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f40(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1178 = v0;
    match m1178 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1180 = match v3.clone() {
                Adt1::C0(h1179, _, _, _, _, _, _, _, _, _, _) => {
                    h1179
                }
            };
            let a1181 = 0u64;
            if if if if if if nat_eq(a1180, a1181) {
                let a1183 = match v3.clone() {
                    Adt1::C0(_, h1182, _, _, _, _, _, _, _, _, _) => {
                        h1182
                    }
                };
                let a1184 = 1u64;
                if nat_eq(a1183, a1184) {
                    true
                } else {
                    let a1186 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h1185, _, _, _, _, _) => {
                            h1185
                        }
                    };
                    let a1187 = 4u64;
                    nat_eq(a1186, a1187)
                }
            } else {
                false
            } {
                let a1190 = match v3.clone() {
                    Adt1::C0(_, h1188, _, _, _, _, _, _, _, _, _) => {
                        h1188
                    }
                };
                let a1191 = match v1 {
                    Adt1::C0(_, _, _, h1189, _, _, _, _, _, _, _) => {
                        h1189
                    }
                };
                nat_eq(a1190, a1191)
            } else {
                false
            } {
                let a1193 = match v3.clone() {
                    Adt1::C0(_, _, h1192, _, _, _, _, _, _, _, _) => {
                        h1192
                    }
                };
                let a1194 = 1u64;
                nat_eq(a1193, a1194)
            } else {
                false
            } {
                let a1196 = match v3.clone() {
                    Adt1::C0(_, _, _, h1195, _, _, _, _, _, _, _) => {
                        h1195
                    }
                };
                let a1197 = 1u64;
                nat_eq(a1196, a1197)
            } else {
                false
            } {
                let a1199 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1198, _, _, _, _, _, _) => {
                        h1198
                    }
                };
                let a1200 = 1u64;
                nat_eq(a1199, a1200)
            } else {
                false
            } {
                f24(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f41(v0: u64) -> u64 {
    if f45(v0) {
        15u64
    } else {
        if if f46(v0) {
            true
        } else {
            f47(v0)
        } {
            8u64
        } else {
            if f48(v0) {
                9u64
            } else {
                f49(v0)
            }
        }
    }
}

pub fn f42(v0: Adt1, v1: Adt1) -> bool {
    let a1203 = match v0.clone() {
        Adt1::C0(_, h1201, _, _, _, _, _, _, _, _, _) => {
            h1201
        }
    };
    let a1204 = match v1.clone() {
        Adt1::C0(_, h1202, _, _, _, _, _, _, _, _, _) => {
            h1202
        }
    };
    if if if nat_eq(a1203, a1204) {
        let a1207 = match v0.clone() {
            Adt1::C0(_, _, _, h1205, _, _, _, _, _, _, _) => {
                h1205
            }
        };
        let a1208 = match v1.clone() {
            Adt1::C0(_, _, _, h1206, _, _, _, _, _, _, _) => {
                h1206
            }
        };
        nat_eq(a1207, a1208)
    } else {
        false
    } {
        let a1210 = 0u64;
        let a1211 = match v0.clone() {
            Adt1::C0(_, _, h1209, _, _, _, _, _, _, _, _) => {
                h1209
            }
        };
        nat_lt(a1210, a1211)
    } else {
        false
    } {
        f18(match v1 {
            Adt1::C0(_, _, _, _, h1212, _, _, _, _, _, _) => {
                h1212
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, h1213, _, _, _, _, _, _) => {
                h1213
            }
        })
    } else {
        false
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> bool {
    let m1214 = f50(v0);
    match m1214 {
        Adt8::C0 => {
            false
        }
        Adt8::C1(v3) => {
            let a1215 = v0;
            let a1216 = 0u64;
            if if if if if if nat_eq(a1215, a1216) {
                true
            } else {
                let a1217 = v0;
                let a1218 = 1u64;
                nat_eq(a1217, a1218)
            } {
                true
            } else {
                let a1219 = v0;
                let a1220 = 30u64;
                nat_eq(a1219, a1220)
            } {
                true
            } else {
                let a1221 = v0;
                let a1222 = 43u64;
                nat_eq(a1221, a1222)
            } {
                true
            } else {
                f51(v0)
            } {
                let a1224 = v2;
                let a1225 = match v3.clone() {
                    Adt9::C0(_, _, h1223) => {
                        h1223
                    }
                };
                nat_eq(a1224, a1225)
            } else {
                false
            } {
                f18(v1, match v3 {
                    Adt9::C0(_, h1226, _) => {
                        h1226
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1227 = v1;
    let a1228 = 1u64;
    if if if nat_le(a1227, a1228) {
        true
    } else {
        let a1229 = v0;
        let a1230 = v2;
        nat_le(a1229, a1230)
    } {
        let a1231 = v3;
        let a1232 = 1u64;
        if nat_le(a1231, a1232) {
            true
        } else {
            let a1239 = {
                let a1237 = v0;
                let a1238 = {
                    let a1235 = {
                        let a1233 = v1;
                        let a1234 = 1u64;
                        nat_sub(a1233, a1234)
                    };
                    let a1236 = v2;
                    nat_mul(a1235, a1236)?
                };
                nat_add(a1237, a1238)?
            };
            let a1240 = v4;
            nat_le(a1239, a1240)
        }
    } else {
        false
    } {
        let a1241 = v5;
        let a1242 = 1u64;
        if nat_le(a1241, a1242) {
            Ok(true)
        } else {
            let a1255 = {
                let a1253 = {
                    let a1247 = v0;
                    let a1248 = {
                        let a1245 = {
                            let a1243 = v1;
                            let a1244 = 1u64;
                            nat_sub(a1243, a1244)
                        };
                        let a1246 = v2;
                        nat_mul(a1245, a1246)?
                    };
                    nat_add(a1247, a1248)?
                };
                let a1254 = {
                    let a1251 = {
                        let a1249 = v3;
                        let a1250 = 1u64;
                        nat_sub(a1249, a1250)
                    };
                    let a1252 = v4;
                    nat_mul(a1251, a1252)?
                };
                nat_add(a1253, a1254)?
            };
            let a1256 = v6;
            Ok(nat_le(a1255, a1256))
        }
    } else {
        Ok(false)
    }
}

pub fn f45(v0: u64) -> bool {
    let a1257 = 10u64;
    let a1258 = v0;
    if if if if if nat_le(a1257, a1258) {
        let a1259 = v0;
        let a1260 = 14u64;
        nat_le(a1259, a1260)
    } else {
        false
    } {
        true
    } else {
        let a1261 = 16u64;
        let a1262 = v0;
        if nat_le(a1261, a1262) {
            let a1263 = v0;
            let a1264 = 19u64;
            nat_le(a1263, a1264)
        } else {
            false
        }
    } {
        true
    } else {
        let a1265 = 21u64;
        let a1266 = v0;
        if nat_le(a1265, a1266) {
            let a1267 = v0;
            let a1268 = 23u64;
            nat_le(a1267, a1268)
        } else {
            false
        }
    } {
        true
    } else {
        let a1269 = 34u64;
        let a1270 = v0;
        if nat_le(a1269, a1270) {
            let a1271 = v0;
            let a1272 = 35u64;
            nat_le(a1271, a1272)
        } else {
            false
        }
    } {
        true
    } else {
        let a1273 = v0;
        let a1274 = 29u64;
        nat_eq(a1273, a1274)
    }
}

pub fn f46(v0: u64) -> bool {
    let a1275 = v0;
    let a1276 = 2u64;
    if if if if nat_eq(a1275, a1276) {
        true
    } else {
        let a1277 = v0;
        let a1278 = 6u64;
        nat_eq(a1277, a1278)
    } {
        true
    } else {
        let a1279 = v0;
        let a1280 = 8u64;
        nat_eq(a1279, a1280)
    } {
        true
    } else {
        let a1281 = v0;
        let a1282 = 20u64;
        nat_eq(a1281, a1282)
    } {
        true
    } else {
        f52(v0)
    }
}

pub fn f47(v0: u64) -> bool {
    let a1283 = v0;
    let a1284 = 39u64;
    if nat_eq(a1283, a1284) {
        true
    } else {
        let a1285 = v0;
        let a1286 = 40u64;
        nat_eq(a1285, a1286)
    }
}

pub fn f48(v0: u64) -> bool {
    let a1287 = v0;
    let a1288 = 3u64;
    if nat_eq(a1287, a1288) {
        true
    } else {
        let a1289 = v0;
        let a1290 = 7u64;
        nat_eq(a1289, a1290)
    }
}

pub fn f49(v0: u64) -> u64 {
    let a1291 = v0;
    let a1292 = 30u64;
    if nat_eq(a1291, a1292) {
        30u64
    } else {
        0u64
    }
}

pub fn f50(v0: u64) -> Adt8 {
    let a1293 = v0;
    let a1294 = 0u64;
    if nat_eq(a1293, a1294) {
        Adt8::C1(Adt9::C0(0u64, 1u64, 4u64))
    } else {
        let a1295 = v0;
        let a1296 = 1u64;
        if nat_eq(a1295, a1296) {
            Adt8::C1(Adt9::C0(1u64, 1u64, 2u64))
        } else {
            let a1297 = v0;
            let a1298 = 2u64;
            if nat_eq(a1297, a1298) {
                Adt8::C1(Adt9::C0(2u64, 32u64, 18u64))
            } else {
                let a1299 = v0;
                let a1300 = 3u64;
                if nat_eq(a1299, a1300) {
                    Adt8::C1(Adt9::C0(3u64, 32u64, 20u64))
                } else {
                    let a1301 = v0;
                    let a1302 = 6u64;
                    if nat_eq(a1301, a1302) {
                        Adt8::C1(Adt9::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1303 = v0;
                        let a1304 = 7u64;
                        if nat_eq(a1303, a1304) {
                            Adt8::C1(Adt9::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1305 = v0;
                            let a1306 = 8u64;
                            if nat_eq(a1305, a1306) {
                                Adt8::C1(Adt9::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1307 = v0;
                                let a1308 = 9u64;
                                if nat_eq(a1307, a1308) {
                                    Adt8::C1(Adt9::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1309 = v0;
                                    let a1310 = 10u64;
                                    if nat_eq(a1309, a1310) {
                                        Adt8::C1(Adt9::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1311 = v0;
                                        let a1312 = 11u64;
                                        if nat_eq(a1311, a1312) {
                                            Adt8::C1(Adt9::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1313 = v0;
                                            let a1314 = 12u64;
                                            if nat_eq(a1313, a1314) {
                                                Adt8::C1(Adt9::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1315 = v0;
                                                let a1316 = 13u64;
                                                if nat_eq(a1315, a1316) {
                                                    Adt8::C1(Adt9::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1317 = v0;
                                                    let a1318 = 14u64;
                                                    if nat_eq(a1317, a1318) {
                                                        Adt8::C1(Adt9::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1319 = v0;
                                                        let a1320 = 15u64;
                                                        if nat_eq(a1319, a1320) {
                                                            Adt8::C1(Adt9::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1321 = v0;
                                                            let a1322 = 16u64;
                                                            if nat_eq(a1321, a1322) {
                                                                Adt8::C1(Adt9::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1323 = v0;
                                                                let a1324 = 17u64;
                                                                if nat_eq(a1323, a1324) {
                                                                    Adt8::C1(Adt9::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1325 = v0;
                                                                    let a1326 = 18u64;
                                                                    if nat_eq(a1325, a1326) {
                                                                        Adt8::C1(Adt9::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1327 = v0;
                                                                        let a1328 = 19u64;
                                                                        if nat_eq(a1327, a1328) {
                                                                            Adt8::C1(Adt9::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1329 = v0;
                                                                            let a1330 = 20u64;
                                                                            if nat_eq(a1329, a1330) {
                                                                                Adt8::C1(Adt9::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1331 = v0;
                                                                                let a1332 = 21u64;
                                                                                if nat_eq(a1331, a1332) {
                                                                                    Adt8::C1(Adt9::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1333 = v0;
                                                                                    let a1334 = 22u64;
                                                                                    if nat_eq(a1333, a1334) {
                                                                                        Adt8::C1(Adt9::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1335 = v0;
                                                                                        let a1336 = 23u64;
                                                                                        if nat_eq(a1335, a1336) {
                                                                                            Adt8::C1(Adt9::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1337 = v0;
                                                                                            let a1338 = 24u64;
                                                                                            if nat_eq(a1337, a1338) {
                                                                                                Adt8::C1(Adt9::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1339 = v0;
                                                                                                let a1340 = 25u64;
                                                                                                if nat_eq(a1339, a1340) {
                                                                                                    Adt8::C1(Adt9::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1341 = v0;
                                                                                                    let a1342 = 26u64;
                                                                                                    if nat_eq(a1341, a1342) {
                                                                                                        Adt8::C1(Adt9::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1343 = v0;
                                                                                                        let a1344 = 27u64;
                                                                                                        if nat_eq(a1343, a1344) {
                                                                                                            Adt8::C1(Adt9::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1345 = v0;
                                                                                                            let a1346 = 28u64;
                                                                                                            if nat_eq(a1345, a1346) {
                                                                                                                Adt8::C1(Adt9::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1347 = v0;
                                                                                                                let a1348 = 29u64;
                                                                                                                if nat_eq(a1347, a1348) {
                                                                                                                    Adt8::C1(Adt9::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1349 = v0;
                                                                                                                    let a1350 = 30u64;
                                                                                                                    if nat_eq(a1349, a1350) {
                                                                                                                        Adt8::C1(Adt9::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1351 = v0;
                                                                                                                        let a1352 = 34u64;
                                                                                                                        if nat_eq(a1351, a1352) {
                                                                                                                            Adt8::C1(Adt9::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1353 = v0;
                                                                                                                            let a1354 = 35u64;
                                                                                                                            if nat_eq(a1353, a1354) {
                                                                                                                                Adt8::C1(Adt9::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1355 = v0;
                                                                                                                                let a1356 = 39u64;
                                                                                                                                if nat_eq(a1355, a1356) {
                                                                                                                                    Adt8::C1(Adt9::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1357 = v0;
                                                                                                                                    let a1358 = 40u64;
                                                                                                                                    if nat_eq(a1357, a1358) {
                                                                                                                                        Adt8::C1(Adt9::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1359 = v0;
                                                                                                                                        let a1360 = 41u64;
                                                                                                                                        if nat_eq(a1359, a1360) {
                                                                                                                                            Adt8::C1(Adt9::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1361 = v0;
                                                                                                                                            let a1362 = 42u64;
                                                                                                                                            if nat_eq(a1361, a1362) {
                                                                                                                                                Adt8::C1(Adt9::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1363 = v0;
                                                                                                                                                let a1364 = 43u64;
                                                                                                                                                if nat_eq(a1363, a1364) {
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

pub fn f51(v0: u64) -> bool {
    if if f53(v0) {
        true
    } else {
        f47(v0)
    } {
        true
    } else {
        f45(v0)
    }
}

pub fn f52(v0: u64) -> bool {
    let a1365 = v0;
    let a1366 = 41u64;
    if nat_eq(a1365, a1366) {
        true
    } else {
        let a1367 = v0;
        let a1368 = 42u64;
        nat_eq(a1367, a1368)
    }
}

pub fn f53(v0: u64) -> bool {
    if f46(v0) {
        true
    } else {
        f48(v0)
    }
}
