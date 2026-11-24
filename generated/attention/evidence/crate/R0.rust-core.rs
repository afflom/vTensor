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
                if if nat_eq(a312, a313) {
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
                }
            } else {
                false
            } {
                let a319 = match v5.clone() {
                    Adt1::C0(h318, _, _, _, _, _, _, _, _, _, _) => {
                        h318
                    }
                };
                let a320 = 0u64;
                nat_eq(a319, a320)
            } else {
                false
            } {
                let a322 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, h321, _, _, _, _, _) => {
                        h321
                    }
                };
                let a323 = 4u64;
                nat_eq(a322, a323)
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
                let a326 = match v5.clone() {
                    Adt1::C0(_, h324, _, _, _, _, _, _, _, _, _) => {
                        h324
                    }
                };
                let a327 = match v6.clone() {
                    Adt1::C0(_, h325, _, _, _, _, _, _, _, _, _) => {
                        h325
                    }
                };
                nat_eq(a326, a327)
            } else {
                false
            } {
                let a330 = match v6.clone() {
                    Adt1::C0(_, _, h328, _, _, _, _, _, _, _, _) => {
                        h328
                    }
                };
                let a331 = match v7.clone() {
                    Adt1::C0(_, _, h329, _, _, _, _, _, _, _, _) => {
                        h329
                    }
                };
                nat_eq(a330, a331)
            } else {
                false
            } {
                let a334 = match v11.clone() {
                    Adt1::C0(_, h332, _, _, _, _, _, _, _, _, _) => {
                        h332
                    }
                };
                let a335 = match v7.clone() {
                    Adt1::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                        h333
                    }
                };
                nat_eq(a334, a335)
            } else {
                false
            } {
                let a338 = match v11.clone() {
                    Adt1::C0(_, _, h336, _, _, _, _, _, _, _, _) => {
                        h336
                    }
                };
                let a339 = match v5.clone() {
                    Adt1::C0(_, _, _, h337, _, _, _, _, _, _, _) => {
                        h337
                    }
                };
                nat_eq(a338, a339)
            } else {
                false
            } {
                let a342 = match v11.clone() {
                    Adt1::C0(_, _, _, h340, _, _, _, _, _, _, _) => {
                        h340
                    }
                };
                let a343 = match v5.clone() {
                    Adt1::C0(_, _, h341, _, _, _, _, _, _, _, _) => {
                        h341
                    }
                };
                nat_eq(a342, a343)
            } else {
                false
            } {
                let a346 = match v11.clone() {
                    Adt1::C0(_, _, _, _, h344, _, _, _, _, _, _) => {
                        h344
                    }
                };
                let a347 = match v5.clone() {
                    Adt1::C0(_, _, _, _, h345, _, _, _, _, _, _) => {
                        h345
                    }
                };
                nat_eq(a346, a347)
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
        let a349 = match v2.clone() {
            Adt1::C0(h348, _, _, _, _, _, _, _, _, _, _) => {
                h348
            }
        };
        let a350 = 1u64;
        if nat_eq(a349, a350) {
            2u64
        } else {
            4u64
        }
    };
    let a351 = v0;
    let a352 = 88u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a351, a352) {
        !v1
    } else {
        false
    } {
        let a354 = match v2.clone() {
            Adt1::C0(h353, _, _, _, _, _, _, _, _, _, _) => {
                h353
            }
        };
        let a355 = 0u64;
        if nat_eq(a354, a355) {
            true
        } else {
            let a357 = match v2.clone() {
                Adt1::C0(h356, _, _, _, _, _, _, _, _, _, _) => {
                    h356
                }
            };
            let a358 = 1u64;
            nat_eq(a357, a358)
        }
    } else {
        false
    } {
        let a360 = match v3.clone() {
            Adt1::C0(h359, _, _, _, _, _, _, _, _, _, _) => {
                h359
            }
        };
        let a361 = 26u64;
        nat_eq(a360, a361)
    } else {
        false
    } {
        let a364 = match v7.clone() {
            Adt1::C0(h362, _, _, _, _, _, _, _, _, _, _) => {
                h362
            }
        };
        let a365 = match v2.clone() {
            Adt1::C0(h363, _, _, _, _, _, _, _, _, _, _) => {
                h363
            }
        };
        nat_eq(a364, a365)
    } else {
        false
    } {
        let a367 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h366, _, _, _, _, _) => {
                h366
            }
        };
        let a368 = v8;
        nat_eq(a367, a368)
    } else {
        false
    } {
        let a370 = match v7.clone() {
            Adt1::C0(_, _, _, _, _, h369, _, _, _, _, _) => {
                h369
            }
        };
        let a371 = v8;
        nat_eq(a370, a371)
    } else {
        false
    } {
        let a374 = match v2.clone() {
            Adt1::C0(_, h372, _, _, _, _, _, _, _, _, _) => {
                h372
            }
        };
        let a375 = match v7.clone() {
            Adt1::C0(_, h373, _, _, _, _, _, _, _, _, _) => {
                h373
            }
        };
        nat_eq(a374, a375)
    } else {
        false
    } {
        f35(v3.clone(), v7.clone())
    } else {
        false
    } {
        f36({
            let a377 = v8;
            let a378 = match v7.clone() {
                Adt1::C0(_, h376, _, _, _, _, _, _, _, _, _) => {
                    h376
                }
            };
            nat_mul(a377, a378)?
        }, match v7.clone() {
            Adt1::C0(_, _, h379, _, _, _, _, _, _, _, _) => {
                h379
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, h380, _, _, _, _, _, _, _) => {
                h380
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, h381, _, _, _, _, _, _) => {
                h381
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, h382, _, _, _, _) => {
                h382
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h383, _, _, _) => {
                h383
            }
        }, match v7.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h384, _, _) => {
                h384
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
            Adt1::C0(_, _, h385, _, _, _, _, _, _, _, _) => {
                h385
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h386, _, _, _, _, _, _, _, _) => {
                h386
            }
        })
    } else {
        false
    } {
        f17(match v3.clone() {
            Adt1::C0(_, _, _, h387, _, _, _, _, _, _, _) => {
                h387
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h388, _, _, _, _, _, _, _) => {
                h388
            }
        })
    } else {
        false
    } {
        f17(match v3.clone() {
            Adt1::C0(_, _, _, _, h389, _, _, _, _, _, _) => {
                h389
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h390, _, _, _, _, _, _) => {
                h390
            }
        })
    } else {
        false
    } {
        let m391 = v4.clone();
        match m391 {
            None => {
                true
            }
            Some(v9) => {
                let a393 = match v9.clone() {
                    Adt1::C0(h392, _, _, _, _, _, _, _, _, _, _) => {
                        h392
                    }
                };
                let a394 = 0u64;
                if if nat_eq(a393, a394) {
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
        let m395 = v5.clone();
        match m395 {
            None => {
                true
            }
            Some(v10) => {
                let a397 = match v10.clone() {
                    Adt1::C0(h396, _, _, _, _, _, _, _, _, _, _) => {
                        h396
                    }
                };
                let a398 = 26u64;
                if if if if nat_eq(a397, a398) {
                    let a400 = match v10.clone() {
                        Adt1::C0(_, _, _, h399, _, _, _, _, _, _, _) => {
                            h399
                        }
                    };
                    let a401 = 1u64;
                    nat_eq(a400, a401)
                } else {
                    false
                } {
                    let a403 = match v10.clone() {
                        Adt1::C0(_, _, _, _, h402, _, _, _, _, _, _) => {
                            h402
                        }
                    };
                    let a404 = 1u64;
                    nat_eq(a403, a404)
                } else {
                    false
                } {
                    f23(v10.clone(), v7.clone())?
                } else {
                    false
                } {
                    let m405 = v4.clone();
                    match m405 {
                        None => {
                            true
                        }
                        Some(v11) => {
                            let a408 = match v11 {
                                Adt1::C0(_, h406, _, _, _, _, _, _, _, _, _) => {
                                    h406
                                }
                            };
                            let a409 = match v10 {
                                Adt1::C0(_, _, h407, _, _, _, _, _, _, _, _) => {
                                    h407
                                }
                            };
                            nat_eq(a408, a409)
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
        let m410 = v6.clone();
        match m410 {
            None => {
                true
            }
            Some(v12) => {
                let a412 = match v12.clone() {
                    Adt1::C0(h411, _, _, _, _, _, _, _, _, _, _) => {
                        h411
                    }
                };
                let a413 = 26u64;
                if if nat_eq(a412, a413) {
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
            let a414 = v8;
            let a415 = 2u64;
            if nat_eq(a414, a415) {
                64512u64
            } else {
                4286578688u64
            }
        }, {
            let m416 = v5;
            match m416 {
                None => {
                    let m417 = v4;
                    match m417 {
                        None => {
                            match v2 {
                                Adt1::C0(_, h418, _, _, _, _, _, _, _, _, _) => {
                                    h418
                                }
                            }
                        }
                        Some(v13) => {
                            match v13 {
                                Adt1::C0(_, h419, _, _, _, _, _, _, _, _, _) => {
                                    h419
                                }
                            }
                        }
                    }
                }
                Some(v14) => {
                    match v14 {
                        Adt1::C0(_, _, h420, _, _, _, _, _, _, _, _) => {
                            h420
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
        Adt4::C0(h421, _, _, _, _, _, _, _, _) => {
            h421
        }
    };
    let v5: Adt1 = match v0.clone() {
        Adt4::C0(_, h422, _, _, _, _, _, _, _) => {
            h422
        }
    };
    let a424 = v1;
    let a425 = match v5.clone() {
        Adt1::C0(_, _, h423, _, _, _, _, _, _, _, _) => {
            h423
        }
    };
    if if if nat_lt(a424, a425) {
        let a427 = v2;
        let a428 = match v5.clone() {
            Adt1::C0(_, _, _, h426, _, _, _, _, _, _, _) => {
                h426
            }
        };
        nat_lt(a427, a428)
    } else {
        false
    } {
        let a430 = v3;
        let a431 = match v5.clone() {
            Adt1::C0(_, _, _, _, h429, _, _, _, _, _, _) => {
                h429
            }
        };
        nat_lt(a430, a431)
    } else {
        false
    } {
        Ok(Adt7::C6(f16(v4.clone(), {
            let a433 = v1;
            let a434 = match v4.clone() {
                Adt1::C0(_, _, h432, _, _, _, _, _, _, _, _) => {
                    h432
                }
            };
            let a435 = 0u64;
            nat_rem(a433, a434, a435)
        }, {
            let a437 = v2;
            let a438 = match v4.clone() {
                Adt1::C0(_, _, _, h436, _, _, _, _, _, _, _) => {
                    h436
                }
            };
            let a439 = 0u64;
            nat_rem(a437, a438, a439)
        }, {
            let a441 = v3;
            let a442 = match v4 {
                Adt1::C0(_, _, _, _, h440, _, _, _, _, _, _) => {
                    h440
                }
            };
            let a443 = 0u64;
            nat_rem(a441, a442, a443)
        })?, f16(v5, v1, v2, v3)?, {
            let m445 = match v0.clone() {
                Adt4::C0(_, _, h444, _, _, _, _, _, _) => {
                    h444
                }
            };
            match m445 {
                None => {
                    0u64
                }
                Some(v6) => {
                    f16(v6, v1, v2, v3)?
                }
            }
        }, {
            let m447 = match v0.clone() {
                Adt4::C0(_, _, _, _, h446, _, _, _, _) => {
                    h446
                }
            };
            match m447 {
                None => {
                    0u64
                }
                Some(v7) => {
                    f16(v7, v1, v2, v3)?
                }
            }
        }, f16(match v0 {
            Adt4::C0(_, _, _, _, _, h448, _, _, _) => {
                h448
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f12(v0: Adt4, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a449 = v1;
    let a450 = 0u64;
    if nat_eq(a449, a450) {
        let a454 = v2;
        let a455 = {
            let m453 = match v0.clone() {
                Adt4::C0(_, h452, _, _, _, _, _, _, _) => {
                    h452
                }
            };
            match m453 {
                Adt1::C0(_, h451, _, _, _, _, _, _, _, _, _) => {
                    h451
                }
            }
        };
        if nat_lt(a454, a455) {
            Ok(Adt7::C7({
                let a459 = v2;
                let a460 = {
                    let m458 = match v0 {
                        Adt4::C0(_, h457, _, _, _, _, _, _, _) => {
                            h457
                        }
                    };
                    match m458 {
                        Adt1::C0(_, _, _, _, _, h456, _, _, _, _, _) => {
                            h456
                        }
                    }
                };
                nat_mul(a459, a460)?
            }))
        } else {
            Ok(Adt7::C8)
        }
    } else {
        let a461 = v1;
        let a462 = 1u64;
        if nat_eq(a461, a462) {
            let m464 = match v0 {
                Adt4::C0(_, _, _, _, h463, _, _, _, _) => {
                    h463
                }
            };
            match m464 {
                None => {
                    Ok(Adt7::C8)
                }
                Some(v4) => {
                    let a466 = v2;
                    let a467 = match v4.clone() {
                        Adt1::C0(_, h465, _, _, _, _, _, _, _, _, _) => {
                            h465
                        }
                    };
                    if nat_lt(a466, a467) {
                        Ok(Adt7::C7({
                            let a469 = v2;
                            let a470 = match v4 {
                                Adt1::C0(_, _, _, _, _, h468, _, _, _, _, _) => {
                                    h468
                                }
                            };
                            nat_mul(a469, a470)?
                        }))
                    } else {
                        Ok(Adt7::C8)
                    }
                }
            }
        } else {
            let a471 = v1;
            let a472 = 2u64;
            if nat_eq(a471, a472) {
                let a473 = v2;
                let a474 = 2147483648u64;
                if if nat_lt(a473, a474) {
                    let a476 = v2;
                    let a477 = match v0.clone() {
                        Adt4::C0(_, _, _, _, _, _, _, _, h475) => {
                            h475
                        }
                    };
                    nat_lt(a476, a477)
                } else {
                    false
                } {
                    let m479 = match v0 {
                        Adt4::C0(_, _, h478, _, _, _, _, _, _) => {
                            h478
                        }
                    };
                    match m479 {
                        None => {
                            Ok(Adt7::C7(0u64))
                        }
                        Some(v5) => {
                            Ok(Adt7::C7({
                                let a481 = v2;
                                let a482 = match v5 {
                                    Adt1::C0(_, _, _, _, _, h480, _, _, _, _, _) => {
                                        h480
                                    }
                                };
                                nat_mul(a481, a482)?
                            }))
                        }
                    }
                } else {
                    Ok(Adt7::C8)
                }
            } else {
                let a483 = v1;
                let a484 = 3u64;
                if nat_eq(a483, a484) {
                    let m486 = match v0 {
                        Adt4::C0(_, _, _, h485, _, _, _, _, _) => {
                            h485
                        }
                    };
                    match m486 {
                        None => {
                            Ok(Adt7::C8)
                        }
                        Some(v6) => {
                            let a488 = v2;
                            let a489 = match v6.clone() {
                                Adt1::C0(_, _, h487, _, _, _, _, _, _, _, _) => {
                                    h487
                                }
                            };
                            if if nat_lt(a488, a489) {
                                let a491 = v3;
                                let a492 = match v6.clone() {
                                    Adt1::C0(_, h490, _, _, _, _, _, _, _, _, _) => {
                                        h490
                                    }
                                };
                                nat_lt(a491, a492)
                            } else {
                                false
                            } {
                                Ok(Adt7::C7({
                                    let a499 = {
                                        let a494 = v2;
                                        let a495 = match v6.clone() {
                                            Adt1::C0(_, _, _, _, _, _, h493, _, _, _, _) => {
                                                h493
                                            }
                                        };
                                        nat_mul(a494, a495)?
                                    };
                                    let a500 = {
                                        let a497 = v3;
                                        let a498 = match v6 {
                                            Adt1::C0(_, _, _, _, _, h496, _, _, _, _, _) => {
                                                h496
                                            }
                                        };
                                        nat_mul(a497, a498)?
                                    };
                                    nat_add(a499, a500)?
                                }))
                            } else {
                                Ok(Adt7::C8)
                            }
                        }
                    }
                } else {
                    let a501 = v1;
                    let a502 = 4u64;
                    if nat_eq(a501, a502) {
                        let a503 = v2;
                        let a504 = 2147483648u64;
                        if if nat_lt(a503, a504) {
                            let a508 = v2;
                            let a509 = {
                                let m507 = match v0.clone() {
                                    Adt4::C0(h506, _, _, _, _, _, _, _, _) => {
                                        h506
                                    }
                                };
                                match m507 {
                                    Adt1::C0(_, h505, _, _, _, _, _, _, _, _, _) => {
                                        h505
                                    }
                                }
                            };
                            nat_lt(a508, a509)
                        } else {
                            false
                        } {
                            Ok(Adt7::C7({
                                let a511 = v2;
                                let a512 = match v0 {
                                    Adt4::C0(_, _, _, _, _, _, h510, _, _) => {
                                        h510
                                    }
                                };
                                nat_mul(a511, a512)?
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
        let a513 = v0;
        let a514 = 43u64;
        if if if if if if if if nat_eq(a513, a514) {
            let a516 = match v4.clone() {
                Adt1::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
                    h515
                }
            };
            let a517 = 0u64;
            if nat_eq(a516, a517) {
                f20(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a519 = match v7.clone() {
                Adt1::C0(h518, _, _, _, _, _, _, _, _, _, _) => {
                    h518
                }
            };
            let a520 = 0u64;
            if nat_eq(a519, a520) {
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
                Adt1::C0(_, _, _, h521, _, _, _, _, _, _, _) => {
                    h521
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
        Adt3::C0(h522, _, _, _, _) => {
            h522
        }
    };
    let a524 = v1;
    let a525 = match v4.clone() {
        Adt1::C0(_, _, h523, _, _, _, _, _, _, _, _) => {
            h523
        }
    };
    if if if nat_lt(a524, a525) {
        let a527 = v2;
        let a528 = match v4.clone() {
            Adt1::C0(_, _, _, h526, _, _, _, _, _, _, _) => {
                h526
            }
        };
        nat_lt(a527, a528)
    } else {
        false
    } {
        let a530 = v3;
        let a531 = match v4.clone() {
            Adt1::C0(_, _, _, _, h529, _, _, _, _, _, _) => {
                h529
            }
        };
        nat_lt(a530, a531)
    } else {
        false
    } {
        let v6: u64 = {
            let m533 = match v0.clone() {
                Adt3::C0(_, h532, _, _, _) => {
                    h532
                }
            };
            match m533 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f16(v5.clone(), v1, {
                        let a535 = v2;
                        let a536 = match v5.clone() {
                            Adt1::C0(_, _, _, h534, _, _, _, _, _, _, _) => {
                                h534
                            }
                        };
                        let a537 = 0u64;
                        nat_rem(a535, a536, a537)
                    }, {
                        let a539 = v3;
                        let a540 = match v5 {
                            Adt1::C0(_, _, _, _, h538, _, _, _, _, _, _) => {
                                h538
                            }
                        };
                        let a541 = 0u64;
                        nat_rem(a539, a540, a541)
                    })?
                }
            }
        };
        Ok(Adt7::C4(f16(v4, v1, v2, v3)?, v6, {
            let a542 = v2;
            let a543 = 4u64;
            nat_mul(a542, a543)?
        }, f16(match v0 {
            Adt3::C0(_, _, h544, _, _) => {
                h544
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f15(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a548 = v1;
    let a549 = {
        let m547 = match v0.clone() {
            Adt2::C0(h546, _, _, _, _, _, _, _, _) => {
                h546
            }
        };
        match m547 {
            Adt1::C0(_, _, h545, _, _, _, _, _, _, _, _) => {
                h545
            }
        }
    };
    if if if nat_lt(a548, a549) {
        let a553 = v2;
        let a554 = {
            let m552 = match v0.clone() {
                Adt2::C0(h551, _, _, _, _, _, _, _, _) => {
                    h551
                }
            };
            match m552 {
                Adt1::C0(_, _, _, h550, _, _, _, _, _, _, _) => {
                    h550
                }
            }
        };
        nat_lt(a553, a554)
    } else {
        false
    } {
        let a558 = v3;
        let a559 = {
            let m557 = match v0.clone() {
                Adt2::C0(h556, _, _, _, _, _, _, _, _) => {
                    h556
                }
            };
            match m557 {
                Adt1::C0(_, _, _, _, h555, _, _, _, _, _, _) => {
                    h555
                }
            }
        };
        nat_lt(a558, a559)
    } else {
        false
    } {
        Ok(Adt7::C2(f16(match v0.clone() {
            Adt2::C0(h560, _, _, _, _, _, _, _, _) => {
                h560
            }
        }, v1, v2, v3)?, f16(match v0.clone() {
            Adt2::C0(_, h561, _, _, _, _, _, _, _) => {
                h561
            }
        }, 0u64, {
            let a571 = v2;
            let a572 = {
                let a568 = {
                    let m564 = match v0.clone() {
                        Adt2::C0(h563, _, _, _, _, _, _, _, _) => {
                            h563
                        }
                    };
                    match m564 {
                        Adt1::C0(_, _, _, h562, _, _, _, _, _, _, _) => {
                            h562
                        }
                    }
                };
                let a569 = {
                    let m567 = match v0.clone() {
                        Adt2::C0(_, h566, _, _, _, _, _, _, _) => {
                            h566
                        }
                    };
                    match m567 {
                        Adt1::C0(_, _, _, h565, _, _, _, _, _, _, _) => {
                            h565
                        }
                    }
                };
                let a570 = 0u64;
                nat_quot(a568, a569, a570)
            };
            let a573 = 0u64;
            nat_quot(a571, a572, a573)
        }, {
            let a583 = v3;
            let a584 = {
                let a580 = {
                    let m576 = match v0.clone() {
                        Adt2::C0(h575, _, _, _, _, _, _, _, _) => {
                            h575
                        }
                    };
                    match m576 {
                        Adt1::C0(_, _, _, _, h574, _, _, _, _, _, _) => {
                            h574
                        }
                    }
                };
                let a581 = {
                    let m579 = match v0.clone() {
                        Adt2::C0(_, h578, _, _, _, _, _, _, _) => {
                            h578
                        }
                    };
                    match m579 {
                        Adt1::C0(_, _, _, _, h577, _, _, _, _, _, _) => {
                            h577
                        }
                    }
                };
                let a582 = 0u64;
                nat_quot(a580, a581, a582)
            };
            let a585 = 0u64;
            nat_quot(a583, a584, a585)
        })?, f16(match v0.clone() {
            Adt2::C0(_, _, h586, _, _, _, _, _, _) => {
                h586
            }
        }, 0u64, {
            let a596 = v2;
            let a597 = {
                let a593 = {
                    let m589 = match v0.clone() {
                        Adt2::C0(h588, _, _, _, _, _, _, _, _) => {
                            h588
                        }
                    };
                    match m589 {
                        Adt1::C0(_, _, _, h587, _, _, _, _, _, _, _) => {
                            h587
                        }
                    }
                };
                let a594 = {
                    let m592 = match v0.clone() {
                        Adt2::C0(_, _, h591, _, _, _, _, _, _) => {
                            h591
                        }
                    };
                    match m592 {
                        Adt1::C0(_, _, _, h590, _, _, _, _, _, _, _) => {
                            h590
                        }
                    }
                };
                let a595 = 0u64;
                nat_quot(a593, a594, a595)
            };
            let a598 = 0u64;
            nat_quot(a596, a597, a598)
        }, {
            let a608 = v3;
            let a609 = {
                let a605 = {
                    let m601 = match v0.clone() {
                        Adt2::C0(h600, _, _, _, _, _, _, _, _) => {
                            h600
                        }
                    };
                    match m601 {
                        Adt1::C0(_, _, _, _, h599, _, _, _, _, _, _) => {
                            h599
                        }
                    }
                };
                let a606 = {
                    let m604 = match v0.clone() {
                        Adt2::C0(_, _, h603, _, _, _, _, _, _) => {
                            h603
                        }
                    };
                    match m604 {
                        Adt1::C0(_, _, _, _, h602, _, _, _, _, _, _) => {
                            h602
                        }
                    }
                };
                let a607 = 0u64;
                nat_quot(a605, a606, a607)
            };
            let a610 = 0u64;
            nat_quot(a608, a609, a610)
        })?, {
            let m612 = match v0.clone() {
                Adt2::C0(_, _, _, h611, _, _, _, _, _) => {
                    h611
                }
            };
            match m612 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f16(v4.clone(), v1, {
                        let a614 = v2;
                        let a615 = match v4.clone() {
                            Adt1::C0(_, _, _, h613, _, _, _, _, _, _, _) => {
                                h613
                            }
                        };
                        let a616 = 0u64;
                        nat_rem(a614, a615, a616)
                    }, {
                        let a618 = v3;
                        let a619 = match v4 {
                            Adt1::C0(_, _, _, _, h617, _, _, _, _, _, _) => {
                                h617
                            }
                        };
                        let a620 = 0u64;
                        nat_rem(a618, a619, a620)
                    })?
                }
            }
        }, {
            let a621 = v2;
            let a622 = 4u64;
            nat_mul(a621, a622)?
        }, f16(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h623, _, _, _) => {
                h623
            }
        }, v2, v1, v3)?, {
            let m626 = match v0.clone() {
                Adt2::C0(_, h625, _, _, _, _, _, _, _) => {
                    h625
                }
            };
            match m626 {
                Adt1::C0(_, _, _, _, _, _, h624, _, _, _, _) => {
                    h624
                }
            }
        }, {
            let m629 = match v0.clone() {
                Adt2::C0(_, _, h628, _, _, _, _, _, _) => {
                    h628
                }
            };
            match m629 {
                Adt1::C0(_, _, _, _, _, _, h627, _, _, _, _) => {
                    h627
                }
            }
        }, f40({
            let m632 = match v0 {
                Adt2::C0(_, h631, _, _, _, _, _, _, _) => {
                    h631
                }
            };
            match m632 {
                Adt1::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                    h630
                }
            }
        })))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f16(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a644 = {
        let a639 = {
            let a634 = v1;
            let a635 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h633, _, _, _, _) => {
                    h633
                }
            };
            nat_mul(a634, a635)?
        };
        let a640 = {
            let a637 = v2;
            let a638 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h636, _, _, _) => {
                    h636
                }
            };
            nat_mul(a637, a638)?
        };
        nat_add(a639, a640)?
    };
    let a645 = {
        let a642 = v3;
        let a643 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h641, _, _) => {
                h641
            }
        };
        nat_mul(a642, a643)?
    };
    nat_add(a644, a645)
}

pub fn f17(v0: u64, v1: u64) -> bool {
    let a646 = 0u64;
    let a647 = v1;
    if nat_lt(a646, a647) {
        let a651 = {
            let a648 = v0;
            let a649 = v1;
            let a650 = 0u64;
            nat_rem(a648, a649, a650)
        };
        let a652 = 0u64;
        nat_eq(a651, a652)
    } else {
        false
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> R<bool> {
    let a654 = match v0.clone() {
        Adt1::C0(h653, _, _, _, _, _, _, _, _, _, _) => {
            h653
        }
    };
    let a655 = 0u64;
    if if nat_eq(a654, a655) {
        let a657 = match v0.clone() {
            Adt1::C0(_, h656, _, _, _, _, _, _, _, _, _) => {
                h656
            }
        };
        let a658 = 1u64;
        if nat_eq(a657, a658) {
            true
        } else {
            let a660 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h659, _, _, _, _, _) => {
                    h659
                }
            };
            let a661 = 4u64;
            nat_eq(a660, a661)
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
    let a663 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h662, _, _, _, _, _) => {
            h662
        }
    };
    let a664 = 4u64;
    if nat_eq(a663, a664) {
        let a669 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h665, _, _, _, _) => {
                h665
            }
        };
        let a670 = {
            let a667 = 4u64;
            let a668 = match v0 {
                Adt1::C0(_, h666, _, _, _, _, _, _, _, _, _) => {
                    h666
                }
            };
            nat_mul(a667, a668)?
        };
        Ok(nat_eq(a669, a670))
    } else {
        Ok(false)
    }
}

pub fn f20(v0: Adt1, v1: u64) -> R<bool> {
    let a672 = match v0.clone() {
        Adt1::C0(_, h671, _, _, _, _, _, _, _, _, _) => {
            h671
        }
    };
    let a673 = 1u64;
    if if if if nat_eq(a672, a673) {
        true
    } else {
        let a675 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h674, _, _, _, _, _) => {
                h674
            }
        };
        let a676 = v1;
        nat_eq(a675, a676)
    } {
        let a678 = match v0.clone() {
            Adt1::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                h677
            }
        };
        let a679 = 1u64;
        if nat_eq(a678, a679) {
            true
        } else {
            let a684 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h680, _, _, _, _) => {
                    h680
                }
            };
            let a685 = {
                let a682 = v1;
                let a683 = match v0.clone() {
                    Adt1::C0(_, h681, _, _, _, _, _, _, _, _, _) => {
                        h681
                    }
                };
                nat_mul(a682, a683)?
            };
            nat_eq(a684, a685)
        }
    } else {
        false
    } {
        let a687 = match v0.clone() {
            Adt1::C0(_, _, _, h686, _, _, _, _, _, _, _) => {
                h686
            }
        };
        let a688 = 1u64;
        if nat_eq(a687, a688) {
            true
        } else {
            let a696 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h689, _, _, _) => {
                    h689
                }
            };
            let a697 = {
                let a694 = {
                    let a691 = v1;
                    let a692 = match v0.clone() {
                        Adt1::C0(_, h690, _, _, _, _, _, _, _, _, _) => {
                            h690
                        }
                    };
                    nat_mul(a691, a692)?
                };
                let a695 = match v0.clone() {
                    Adt1::C0(_, _, h693, _, _, _, _, _, _, _, _) => {
                        h693
                    }
                };
                nat_mul(a694, a695)?
            };
            nat_eq(a696, a697)
        }
    } else {
        false
    } {
        let a699 = match v0.clone() {
            Adt1::C0(_, _, _, _, h698, _, _, _, _, _, _) => {
                h698
            }
        };
        let a700 = 1u64;
        if nat_eq(a699, a700) {
            Ok(true)
        } else {
            let a711 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h701, _, _) => {
                    h701
                }
            };
            let a712 = {
                let a709 = {
                    let a706 = {
                        let a703 = v1;
                        let a704 = match v0.clone() {
                            Adt1::C0(_, h702, _, _, _, _, _, _, _, _, _) => {
                                h702
                            }
                        };
                        nat_mul(a703, a704)?
                    };
                    let a707 = match v0.clone() {
                        Adt1::C0(_, _, h705, _, _, _, _, _, _, _, _) => {
                            h705
                        }
                    };
                    nat_mul(a706, a707)?
                };
                let a710 = match v0 {
                    Adt1::C0(_, _, _, h708, _, _, _, _, _, _, _) => {
                        h708
                    }
                };
                nat_mul(a709, a710)?
            };
            Ok(nat_eq(a711, a712))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    let a714 = match v0.clone() {
        Adt1::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
            h713
        }
    };
    let a715 = v1;
    if if if nat_eq(a714, a715) {
        let a717 = match v0.clone() {
            Adt1::C0(_, _, h716, _, _, _, _, _, _, _, _) => {
                h716
            }
        };
        let a718 = v2;
        nat_eq(a717, a718)
    } else {
        false
    } {
        let a720 = match v0.clone() {
            Adt1::C0(_, _, _, h719, _, _, _, _, _, _, _) => {
                h719
            }
        };
        let a721 = v3;
        nat_eq(a720, a721)
    } else {
        false
    } {
        let a723 = match v0 {
            Adt1::C0(_, _, _, _, h722, _, _, _, _, _, _) => {
                h722
            }
        };
        let a724 = v4;
        nat_eq(a723, a724)
    } else {
        false
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> bool {
    let a727 = match v0.clone() {
        Adt1::C0(_, h725, _, _, _, _, _, _, _, _, _) => {
            h725
        }
    };
    let a728 = match v1.clone() {
        Adt1::C0(_, h726, _, _, _, _, _, _, _, _, _) => {
            h726
        }
    };
    if if if nat_eq(a727, a728) {
        let a731 = match v0.clone() {
            Adt1::C0(_, _, h729, _, _, _, _, _, _, _, _) => {
                h729
            }
        };
        let a732 = match v1.clone() {
            Adt1::C0(_, _, h730, _, _, _, _, _, _, _, _) => {
                h730
            }
        };
        nat_eq(a731, a732)
    } else {
        false
    } {
        let a735 = match v0.clone() {
            Adt1::C0(_, _, _, h733, _, _, _, _, _, _, _) => {
                h733
            }
        };
        let a736 = match v1.clone() {
            Adt1::C0(_, _, _, h734, _, _, _, _, _, _, _) => {
                h734
            }
        };
        nat_eq(a735, a736)
    } else {
        false
    } {
        let a739 = match v0 {
            Adt1::C0(_, _, _, _, h737, _, _, _, _, _, _) => {
                h737
            }
        };
        let a740 = match v1 {
            Adt1::C0(_, _, _, _, h738, _, _, _, _, _, _) => {
                h738
            }
        };
        nat_eq(a739, a740)
    } else {
        false
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> R<bool> {
    let a742 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h741, _) => {
            h741
        }
    };
    let a743 = 0u64;
    if nat_eq(a742, a743) {
        Ok(true)
    } else {
        let a745 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h744, _) => {
                h744
            }
        };
        let a746 = 0u64;
        if nat_eq(a745, a746) {
            Ok(true)
        } else {
            let a748 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h747) => {
                    h747
                }
            };
            let a749 = 0u64;
            if nat_eq(a748, a749) {
                Ok(true)
            } else {
                let a751 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h750) => {
                        h750
                    }
                };
                let a752 = 0u64;
                if nat_eq(a751, a752) {
                    Ok(true)
                } else {
                    let a758 = {
                        let a755 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h753) => {
                                h753
                            }
                        };
                        let a756 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h754, _) => {
                                h754
                            }
                        };
                        nat_add(a755, a756)?
                    };
                    let a759 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h757) => {
                            h757
                        }
                    };
                    if nat_le(a758, a759) {
                        Ok(true)
                    } else {
                        let a765 = {
                            let a762 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h760) => {
                                    h760
                                }
                            };
                            let a763 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h761, _) => {
                                    h761
                                }
                            };
                            nat_add(a762, a763)?
                        };
                        let a766 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h764) => {
                                h764
                            }
                        };
                        Ok(nat_le(a765, a766))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h767, _, _, _, _, _, _, _, _, _) => {
            h767
        }
    };
    let v5: u64 = match v0.clone() {
        Adt1::C0(_, _, h768, _, _, _, _, _, _, _, _) => {
            h768
        }
    };
    let v6: u64 = match v0.clone() {
        Adt1::C0(_, _, _, h769, _, _, _, _, _, _, _) => {
            h769
        }
    };
    let v7: u64 = match v0 {
        Adt1::C0(_, _, _, _, h770, _, _, _, _, _, _) => {
            h770
        }
    };
    let a791 = 4u64;
    let a792 = {
        let a789 = {
            let a775 = {
                let a771 = v4;
                let a772 = v5;
                nat_mul(a771, a772)?
            };
            let a776 = {
                let a773 = v6;
                let a774 = v7;
                nat_mul(a773, a774)?
            };
            nat_mul(a775, a776)?
        };
        let a790 = {
            let a787 = {
                let a777 = v4;
                let a778 = v4;
                nat_mul(a777, a778)?
            };
            let a788 = {
                let a785 = v1;
                let a786 = {
                    let a783 = v5;
                    let a784 = {
                        let a781 = v2;
                        let a782 = {
                            let a779 = v7;
                            let a780 = v3;
                            nat_mul(a779, a780)?
                        };
                        nat_add(a781, a782)?
                    };
                    nat_mul(a783, a784)?
                };
                nat_add(a785, a786)?
            };
            nat_mul(a787, a788)?
        };
        nat_add(a789, a790)?
    };
    nat_mul(a791, a792)
}

pub fn f25(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: Adt1, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt7> {
    let v8: u64 = match v3.clone() {
        Adt1::C0(_, h793, _, _, _, _, _, _, _, _, _) => {
            h793
        }
    };
    let v9: u64 = match v3.clone() {
        Adt1::C0(_, _, h794, _, _, _, _, _, _, _, _) => {
            h794
        }
    };
    let v10: u64 = match v3.clone() {
        Adt1::C0(_, _, _, h795, _, _, _, _, _, _, _) => {
            h795
        }
    };
    let v11: u64 = match v3.clone() {
        Adt1::C0(_, _, _, _, h796, _, _, _, _, _, _) => {
            h796
        }
    };
    let a797 = 0u64;
    let a798 = v8;
    if if if if if if if if if if if if if if if if if if if if if if if nat_lt(a797, a798) {
        let a799 = 0u64;
        let a800 = v9;
        nat_lt(a799, a800)
    } else {
        false
    } {
        let a801 = 0u64;
        let a802 = v11;
        nat_lt(a801, a802)
    } else {
        false
    } {
        let a803 = 0u64;
        let a804 = v0;
        nat_lt(a803, a804)
    } else {
        false
    } {
        let a805 = v0;
        let a806 = 2147483647u64;
        nat_le(a805, a806)
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
        let a808 = match v4.clone() {
            Adt1::C0(_, h807, _, _, _, _, _, _, _, _, _) => {
                h807
            }
        };
        let a809 = 1u64;
        if nat_eq(a808, a809) {
            true
        } else {
            let a811 = match v4.clone() {
                Adt1::C0(_, h810, _, _, _, _, _, _, _, _, _) => {
                    h810
                }
            };
            let a812 = v8;
            nat_eq(a811, a812)
        }
    } else {
        false
    } {
        let a815 = match v4.clone() {
            Adt1::C0(_, h813, _, _, _, _, _, _, _, _, _) => {
                h813
            }
        };
        let a816 = match v4.clone() {
            Adt1::C0(_, h814, _, _, _, _, _, _, _, _, _) => {
                h814
            }
        };
        if if if nat_eq(a815, a816) {
            let a818 = match v4.clone() {
                Adt1::C0(_, _, h817, _, _, _, _, _, _, _, _) => {
                    h817
                }
            };
            let a819 = v9;
            nat_eq(a818, a819)
        } else {
            false
        } {
            let a821 = match v4.clone() {
                Adt1::C0(_, _, _, h820, _, _, _, _, _, _, _) => {
                    h820
                }
            };
            let a822 = v10;
            nat_eq(a821, a822)
        } else {
            false
        } {
            let a824 = match v4.clone() {
                Adt1::C0(_, _, _, _, h823, _, _, _, _, _, _) => {
                    h823
                }
            };
            let a825 = v11;
            nat_eq(a824, a825)
        } else {
            false
        }
    } else {
        false
    } {
        let a827 = match v5.clone() {
            Adt1::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                h826
            }
        };
        let a828 = 1u64;
        if if if nat_eq(a827, a828) {
            let a830 = match v5.clone() {
                Adt1::C0(_, _, h829, _, _, _, _, _, _, _, _) => {
                    h829
                }
            };
            let a831 = v9;
            nat_eq(a830, a831)
        } else {
            false
        } {
            let a833 = match v5.clone() {
                Adt1::C0(_, _, _, h832, _, _, _, _, _, _, _) => {
                    h832
                }
            };
            let a834 = v10;
            nat_eq(a833, a834)
        } else {
            false
        } {
            let a836 = match v5.clone() {
                Adt1::C0(_, _, _, _, h835, _, _, _, _, _, _) => {
                    h835
                }
            };
            let a837 = v11;
            nat_eq(a836, a837)
        } else {
            false
        }
    } else {
        false
    } {
        let a839 = match v6.clone() {
            Adt1::C0(_, h838, _, _, _, _, _, _, _, _, _) => {
                h838
            }
        };
        let a840 = v8;
        if if if nat_eq(a839, a840) {
            let a842 = match v6.clone() {
                Adt1::C0(_, _, h841, _, _, _, _, _, _, _, _) => {
                    h841
                }
            };
            let a843 = v8;
            nat_eq(a842, a843)
        } else {
            false
        } {
            let a845 = match v6.clone() {
                Adt1::C0(_, _, _, h844, _, _, _, _, _, _, _) => {
                    h844
                }
            };
            let a846 = v9;
            nat_eq(a845, a846)
        } else {
            false
        } {
            let a848 = match v6.clone() {
                Adt1::C0(_, _, _, _, h847, _, _, _, _, _, _) => {
                    h847
                }
            };
            let a849 = v11;
            nat_eq(a848, a849)
        } else {
            false
        }
    } else {
        false
    } {
        let a853 = match v7.clone() {
            Adt1::C0(_, h850, _, _, _, _, _, _, _, _, _) => {
                h850
            }
        };
        let a854 = {
            let a851 = v8;
            let a852 = v9;
            nat_mul(a851, a852)?
        };
        if if if nat_eq(a853, a854) {
            let a864 = match v7.clone() {
                Adt1::C0(_, _, h855, _, _, _, _, _, _, _, _) => {
                    h855
                }
            };
            let a865 = {
                let a862 = {
                    let a856 = v10;
                    let a857 = v11;
                    nat_mul(a856, a857)?
                };
                let a863 = {
                    let a860 = {
                        let a858 = v0;
                        let a859 = v8;
                        nat_mul(a858, a859)?
                    };
                    let a861 = v11;
                    nat_mul(a860, a861)?
                };
                nat_add(a862, a863)?
            };
            nat_eq(a864, a865)
        } else {
            false
        } {
            let a867 = match v7.clone() {
                Adt1::C0(_, _, _, h866, _, _, _, _, _, _, _) => {
                    h866
                }
            };
            let a868 = 1u64;
            nat_eq(a867, a868)
        } else {
            false
        } {
            let a870 = match v7.clone() {
                Adt1::C0(_, _, _, _, h869, _, _, _, _, _, _) => {
                    h869
                }
            };
            let a871 = 1u64;
            nat_eq(a870, a871)
        } else {
            false
        }
    } else {
        false
    } {
        let a873 = match v7.clone() {
            Adt1::C0(h872, _, _, _, _, _, _, _, _, _, _) => {
                h872
            }
        };
        let a874 = 0u64;
        nat_eq(a873, a874)
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
            let a875 = v9;
            let a876 = v11;
            nat_mul(a875, a876)?
        };
        let v13: u64 = {
            let a877 = v8;
            let a878 = v8;
            nat_mul(a877, a878)?
        };
        let v14: u64 = {
            let a879 = v10;
            let a880 = 0u64;
            if if nat_eq(a879, a880) {
                let a881 = v0;
                let a882 = 1u64;
                nat_eq(a881, a882)
            } else {
                false
            } {
                1u64
            } else {
                let a883 = v0;
                let a884 = v10;
                if nat_lt(a883, a884) {
                    v0
                } else {
                    v10
                }
            }
        };
        let v15: u64 = {
            let a891 = v12;
            let a892 = {
                let a889 = {
                    let a885 = v10;
                    let a886 = v8;
                    nat_mul(a885, a886)?
                };
                let a890 = {
                    let a887 = v14;
                    let a888 = v13;
                    nat_mul(a887, a888)?
                };
                nat_add(a889, a890)?
            };
            nat_mul(a891, a892)?
        };
        Ok(Adt7::C12(Adt6::C0(v1, v2, v3, v4, v5, v6, v0, v12, v13, {
            let a895 = {
                let a893 = v13;
                let a894 = v8;
                nat_add(a893, a894)?
            };
            let a896 = v8;
            nat_add(a895, a896)?
        }, v15, false)))
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f26(v0: bool, v1: Adt1, v2: Adt1, v3: Option<Adt1>, v4: Option<Adt1>, v5: Option<Adt1>, v6: Adt1) -> R<Adt7> {
    let v8: Adt1 = {
        let m897 = v3.clone();
        match m897 {
            None => {
                v1.clone()
            }
            Some(v7) => {
                v7
            }
        }
    };
    let v10: Adt1 = {
        let m898 = v4.clone();
        match m898 {
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
        let m899 = v5;
        match m899 {
            None => {
                let m900 = v3;
                match m900 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m901 = v4;
                        match m901 {
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
        let a903 = match v1.clone() {
            Adt1::C0(h902, _, _, _, _, _, _, _, _, _, _) => {
                h902
            }
        };
        let a904 = 0u64;
        if nat_eq(a903, a904) {
            let a906 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h905, _, _, _, _, _) => {
                    h905
                }
            };
            let a907 = 4u64;
            nat_eq(a906, a907)
        } else {
            false
        }
    } else {
        false
    } {
        let a909 = match v8.clone() {
            Adt1::C0(h908, _, _, _, _, _, _, _, _, _, _) => {
                h908
            }
        };
        let a910 = 0u64;
        if nat_eq(a909, a910) {
            let a912 = match v8.clone() {
                Adt1::C0(_, _, _, _, _, h911, _, _, _, _, _) => {
                    h911
                }
            };
            let a913 = 4u64;
            nat_eq(a912, a913)
        } else {
            false
        }
    } else {
        false
    } {
        let a915 = match v10.clone() {
            Adt1::C0(h914, _, _, _, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a916 = 1u64;
        if nat_eq(a915, a916) {
            let a918 = match v10.clone() {
                Adt1::C0(_, _, _, _, _, h917, _, _, _, _, _) => {
                    h917
                }
            };
            let a919 = 2u64;
            nat_eq(a918, a919)
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
            Adt1::C0(h920, _, _, _, _, _, _, _, _, _, _) => {
                h920
            }
        }, match v2.clone() {
            Adt1::C0(_, h921, _, _, _, _, _, _, _, _, _) => {
                h921
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h922, _, _, _, _, _) => {
                h922
            }
        })
    } else {
        false
    } {
        let a924 = 0u64;
        let a925 = match v1.clone() {
            Adt1::C0(_, h923, _, _, _, _, _, _, _, _, _) => {
                h923
            }
        };
        nat_lt(a924, a925)
    } else {
        false
    } {
        let a927 = 0u64;
        let a928 = match v1.clone() {
            Adt1::C0(_, _, h926, _, _, _, _, _, _, _, _) => {
                h926
            }
        };
        nat_lt(a927, a928)
    } else {
        false
    } {
        let a930 = 0u64;
        let a931 = match v1.clone() {
            Adt1::C0(_, _, _, h929, _, _, _, _, _, _, _) => {
                h929
            }
        };
        nat_lt(a930, a931)
    } else {
        false
    } {
        let a933 = 0u64;
        let a934 = match v1.clone() {
            Adt1::C0(_, _, _, _, h932, _, _, _, _, _, _) => {
                h932
            }
        };
        nat_lt(a933, a934)
    } else {
        false
    } {
        let a936 = 0u64;
        let a937 = match v2.clone() {
            Adt1::C0(_, _, _, h935, _, _, _, _, _, _, _) => {
                h935
            }
        };
        nat_lt(a936, a937)
    } else {
        false
    } {
        let a940 = match v2.clone() {
            Adt1::C0(_, h938, _, _, _, _, _, _, _, _, _) => {
                h938
            }
        };
        let a941 = match v1.clone() {
            Adt1::C0(_, h939, _, _, _, _, _, _, _, _, _) => {
                h939
            }
        };
        nat_eq(a940, a941)
    } else {
        false
    } {
        let a943 = match v2.clone() {
            Adt1::C0(_, _, h942, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        let a944 = 1u64;
        nat_eq(a943, a944)
    } else {
        false
    } {
        let a947 = match v2.clone() {
            Adt1::C0(_, _, _, _, h945, _, _, _, _, _, _) => {
                h945
            }
        };
        let a948 = match v1.clone() {
            Adt1::C0(_, _, _, _, h946, _, _, _, _, _, _) => {
                h946
            }
        };
        nat_eq(a947, a948)
    } else {
        false
    } {
        let a951 = match v8.clone() {
            Adt1::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        let a952 = match v1.clone() {
            Adt1::C0(_, _, h950, _, _, _, _, _, _, _, _) => {
                h950
            }
        };
        nat_eq(a951, a952)
    } else {
        false
    } {
        let a955 = match v8.clone() {
            Adt1::C0(_, _, h953, _, _, _, _, _, _, _, _) => {
                h953
            }
        };
        let a956 = match v1.clone() {
            Adt1::C0(_, _, _, h954, _, _, _, _, _, _, _) => {
                h954
            }
        };
        nat_eq(a955, a956)
    } else {
        false
    } {
        let a958 = match v8.clone() {
            Adt1::C0(_, _, _, h957, _, _, _, _, _, _, _) => {
                h957
            }
        };
        let a959 = 1u64;
        nat_eq(a958, a959)
    } else {
        false
    } {
        let a962 = match v8.clone() {
            Adt1::C0(_, _, _, _, h960, _, _, _, _, _, _) => {
                h960
            }
        };
        let a963 = match v1.clone() {
            Adt1::C0(_, _, _, _, h961, _, _, _, _, _, _) => {
                h961
            }
        };
        nat_eq(a962, a963)
    } else {
        false
    } {
        let a966 = match v10.clone() {
            Adt1::C0(_, h964, _, _, _, _, _, _, _, _, _) => {
                h964
            }
        };
        let a967 = match v2.clone() {
            Adt1::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                h965
            }
        };
        nat_eq(a966, a967)
    } else {
        false
    } {
        let a970 = match v10.clone() {
            Adt1::C0(_, _, h968, _, _, _, _, _, _, _, _) => {
                h968
            }
        };
        let a971 = match v1.clone() {
            Adt1::C0(_, _, _, h969, _, _, _, _, _, _, _) => {
                h969
            }
        };
        nat_eq(a970, a971)
    } else {
        false
    } {
        let a973 = match v10.clone() {
            Adt1::C0(_, _, _, h972, _, _, _, _, _, _, _) => {
                h972
            }
        };
        let a974 = 1u64;
        nat_eq(a973, a974)
    } else {
        false
    } {
        f17(match v1.clone() {
            Adt1::C0(_, _, _, _, h975, _, _, _, _, _, _) => {
                h975
            }
        }, match v10.clone() {
            Adt1::C0(_, _, _, _, h976, _, _, _, _, _, _) => {
                h976
            }
        })
    } else {
        false
    } {
        let a979 = match v6.clone() {
            Adt1::C0(_, h977, _, _, _, _, _, _, _, _, _) => {
                h977
            }
        };
        let a980 = match v2.clone() {
            Adt1::C0(_, _, _, h978, _, _, _, _, _, _, _) => {
                h978
            }
        };
        nat_eq(a979, a980)
    } else {
        false
    } {
        let a983 = match v6.clone() {
            Adt1::C0(_, _, h981, _, _, _, _, _, _, _, _) => {
                h981
            }
        };
        let a984 = match v1.clone() {
            Adt1::C0(_, _, _, h982, _, _, _, _, _, _, _) => {
                h982
            }
        };
        nat_eq(a983, a984)
    } else {
        false
    } {
        let a986 = match v6.clone() {
            Adt1::C0(_, _, _, h985, _, _, _, _, _, _, _) => {
                h985
            }
        };
        let a987 = 1u64;
        nat_eq(a986, a987)
    } else {
        false
    } {
        let a990 = match v6.clone() {
            Adt1::C0(_, _, _, _, h988, _, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = match v1.clone() {
            Adt1::C0(_, _, _, _, h989, _, _, _, _, _, _) => {
                h989
            }
        };
        nat_eq(a990, a991)
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
    let m992 = v0;
    match m992 {
        None => {
            Ok(Some(match v3 {
                Adt1::C0(_, _, h993, _, _, _, _, _, _, _, _) => {
                    h993
                }
            }))
        }
        Some(v6) => {
            let m994 = v1;
            match m994 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v7) => {
                    let a996 = match v6.clone() {
                        Adt1::C0(h995, _, _, _, _, _, _, _, _, _, _) => {
                            h995
                        }
                    };
                    let a997 = 26u64;
                    if if if if if if if if if nat_eq(a996, a997) {
                        f20(v6.clone(), 4u64)?
                    } else {
                        false
                    } {
                        let a1000 = match v6.clone() {
                            Adt1::C0(_, h998, _, _, _, _, _, _, _, _, _) => {
                                h998
                            }
                        };
                        let a1001 = match v7 {
                            Adt1::C0(_, h999, _, _, _, _, _, _, _, _, _) => {
                                h999
                            }
                        };
                        nat_eq(a1000, a1001)
                    } else {
                        false
                    } {
                        let a1004 = match v6.clone() {
                            Adt1::C0(_, _, h1002, _, _, _, _, _, _, _, _) => {
                                h1002
                            }
                        };
                        let a1005 = match v2 {
                            Adt1::C0(_, _, _, _, h1003, _, _, _, _, _, _) => {
                                h1003
                            }
                        };
                        nat_eq(a1004, a1005)
                    } else {
                        false
                    } {
                        let a1007 = match v6.clone() {
                            Adt1::C0(_, _, _, h1006, _, _, _, _, _, _, _) => {
                                h1006
                            }
                        };
                        let a1008 = 1u64;
                        nat_eq(a1007, a1008)
                    } else {
                        false
                    } {
                        let a1010 = match v6.clone() {
                            Adt1::C0(_, _, _, _, h1009, _, _, _, _, _, _) => {
                                h1009
                            }
                        };
                        let a1011 = 1u64;
                        nat_eq(a1010, a1011)
                    } else {
                        false
                    } {
                        let a1013 = match v3 {
                            Adt1::C0(_, _, _, _, h1012, _, _, _, _, _, _) => {
                                h1012
                            }
                        };
                        let a1014 = 1u64;
                        nat_eq(a1013, a1014)
                    } else {
                        false
                    } {
                        let a1016 = match v4 {
                            Adt1::C0(_, _, _, _, h1015, _, _, _, _, _, _) => {
                                h1015
                            }
                        };
                        let a1017 = 1u64;
                        nat_eq(a1016, a1017)
                    } else {
                        false
                    } {
                        f23(v6.clone(), v5)?
                    } else {
                        false
                    } {
                        Ok(Some(match v6 {
                            Adt1::C0(_, h1018, _, _, _, _, _, _, _, _, _) => {
                                h1018
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
    let a1019 = v0;
    let a1020 = 0u64;
    if if nat_eq(a1019, a1020) {
        true
    } else {
        let a1021 = 2147483648u64;
        let a1022 = v0;
        if nat_le(a1021, a1022) {
            let a1023 = v0;
            let a1024 = 4286578688u64;
            nat_lt(a1023, a1024)
        } else {
            false
        }
    } {
        true
    } else {
        let a1025 = v0;
        let a1026 = 2139095040u64;
        if if nat_lt(a1025, a1026) {
            let a1027 = v1;
            let a1028 = 4294967295u64;
            nat_le(a1027, a1028)
        } else {
            false
        } {
            let m1029 = v2;
            match m1029 {
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
    let a1031 = match v0.clone() {
        Adt1::C0(h1030, _, _, _, _, _, _, _, _, _, _) => {
            h1030
        }
    };
    let a1032 = 0u64;
    if if if if nat_eq(a1031, a1032) {
        let a1034 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1033, _, _, _, _, _) => {
                h1033
            }
        };
        let a1035 = 4u64;
        nat_eq(a1034, a1035)
    } else {
        false
    } {
        let a1041 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1036, _, _, _, _) => {
                h1036
            }
        };
        let a1042 = {
            let a1039 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1037, _, _, _, _, _) => {
                    h1037
                }
            };
            let a1040 = match v0.clone() {
                Adt1::C0(_, h1038, _, _, _, _, _, _, _, _, _) => {
                    h1038
                }
            };
            nat_mul(a1039, a1040)?
        };
        nat_eq(a1041, a1042)
    } else {
        false
    } {
        let a1048 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1043, _, _, _) => {
                h1043
            }
        };
        let a1049 = {
            let a1046 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1044, _, _, _, _) => {
                    h1044
                }
            };
            let a1047 = match v0.clone() {
                Adt1::C0(_, _, h1045, _, _, _, _, _, _, _, _) => {
                    h1045
                }
            };
            nat_mul(a1046, a1047)?
        };
        nat_eq(a1048, a1049)
    } else {
        false
    } {
        let a1055 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h1050, _, _) => {
                h1050
            }
        };
        let a1056 = {
            let a1053 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1051, _, _, _) => {
                    h1051
                }
            };
            let a1054 = match v0 {
                Adt1::C0(_, _, _, h1052, _, _, _, _, _, _, _) => {
                    h1052
                }
            };
            nat_mul(a1053, a1054)?
        };
        Ok(nat_eq(a1055, a1056))
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f42(match v0.clone() {
        Adt1::C0(h1057, _, _, _, _, _, _, _, _, _, _) => {
            h1057
        }
    }, match v0.clone() {
        Adt1::C0(_, h1058, _, _, _, _, _, _, _, _, _) => {
            h1058
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1059, _, _, _, _, _) => {
            h1059
        }
    }) {
        f17(match v1.clone() {
            Adt1::C0(_, _, _, h1060, _, _, _, _, _, _, _) => {
                h1060
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h1061, _, _, _, _, _, _, _) => {
                h1061
            }
        })
    } else {
        false
    } {
        f17(match v1 {
            Adt1::C0(_, _, _, _, h1062, _, _, _, _, _, _) => {
                h1062
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h1063, _, _, _, _, _, _) => {
                h1063
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
    let m1064 = v0;
    match m1064 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a1066 = match v4.clone() {
                Adt1::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
                    h1065
                }
            };
            let a1067 = 1u64;
            if if if if if if nat_eq(a1066, a1067) {
                let a1069 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h1068, _, _, _, _, _) => {
                        h1068
                    }
                };
                let a1070 = 2u64;
                nat_eq(a1069, a1070)
            } else {
                false
            } {
                let a1072 = v2;
                let a1073 = match v4.clone() {
                    Adt1::C0(_, h1071, _, _, _, _, _, _, _, _, _) => {
                        h1071
                    }
                };
                nat_le(a1072, a1073)
            } else {
                false
            } {
                let a1076 = match v1.clone() {
                    Adt1::C0(_, _, h1074, _, _, _, _, _, _, _, _) => {
                        h1074
                    }
                };
                let a1077 = match v4.clone() {
                    Adt1::C0(_, _, h1075, _, _, _, _, _, _, _, _) => {
                        h1075
                    }
                };
                nat_le(a1076, a1077)
            } else {
                false
            } {
                f17(match v1.clone() {
                    Adt1::C0(_, _, _, h1078, _, _, _, _, _, _, _) => {
                        h1078
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h1079, _, _, _, _, _, _, _) => {
                        h1079
                    }
                })
            } else {
                false
            } {
                f17(match v1 {
                    Adt1::C0(_, _, _, _, h1080, _, _, _, _, _, _) => {
                        h1080
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h1081, _, _, _, _, _, _) => {
                        h1081
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
    let m1082 = v0;
    match m1082 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1084 = match v3.clone() {
                Adt1::C0(h1083, _, _, _, _, _, _, _, _, _, _) => {
                    h1083
                }
            };
            let a1085 = 0u64;
            if if if if if if nat_eq(a1084, a1085) {
                let a1087 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h1086, _, _, _, _, _) => {
                        h1086
                    }
                };
                let a1088 = 4u64;
                nat_eq(a1087, a1088)
            } else {
                false
            } {
                let a1091 = match v3.clone() {
                    Adt1::C0(_, h1089, _, _, _, _, _, _, _, _, _) => {
                        h1089
                    }
                };
                let a1092 = match v1 {
                    Adt1::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
                        h1090
                    }
                };
                nat_eq(a1091, a1092)
            } else {
                false
            } {
                let a1094 = match v3.clone() {
                    Adt1::C0(_, _, h1093, _, _, _, _, _, _, _, _) => {
                        h1093
                    }
                };
                let a1095 = 1u64;
                nat_eq(a1094, a1095)
            } else {
                false
            } {
                let a1097 = match v3.clone() {
                    Adt1::C0(_, _, _, h1096, _, _, _, _, _, _, _) => {
                        h1096
                    }
                };
                let a1098 = 1u64;
                nat_eq(a1097, a1098)
            } else {
                false
            } {
                let a1100 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1099, _, _, _, _, _, _) => {
                        h1099
                    }
                };
                let a1101 = 1u64;
                nat_eq(a1100, a1101)
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
    let a1102 = 0u64;
    let a1103 = v0;
    if if nat_lt(a1102, a1103) {
        let a1104 = v0;
        let a1105 = 2139095040u64;
        nat_lt(a1104, a1105)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f34(v0: u64) -> Option<u64> {
    let a1106 = v0;
    let a1107 = 0u64;
    if if nat_eq(a1106, a1107) {
        true
    } else {
        let a1108 = v0;
        let a1109 = 2147483648u64;
        nat_eq(a1108, a1109)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f35(v0: Adt1, v1: Adt1) -> bool {
    let a1112 = match v0.clone() {
        Adt1::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
            h1110
        }
    };
    let a1113 = match v1.clone() {
        Adt1::C0(_, _, h1111, _, _, _, _, _, _, _, _) => {
            h1111
        }
    };
    if if nat_eq(a1112, a1113) {
        let a1116 = match v0.clone() {
            Adt1::C0(_, _, _, h1114, _, _, _, _, _, _, _) => {
                h1114
            }
        };
        let a1117 = match v1.clone() {
            Adt1::C0(_, _, _, h1115, _, _, _, _, _, _, _) => {
                h1115
            }
        };
        nat_eq(a1116, a1117)
    } else {
        false
    } {
        let a1120 = match v0 {
            Adt1::C0(_, _, _, _, h1118, _, _, _, _, _, _) => {
                h1118
            }
        };
        let a1121 = match v1 {
            Adt1::C0(_, _, _, _, h1119, _, _, _, _, _, _) => {
                h1119
            }
        };
        nat_eq(a1120, a1121)
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
        let a1124 = match v0.clone() {
            Adt1::C0(h1122, _, _, _, _, _, _, _, _, _, _) => {
                h1122
            }
        };
        let a1125 = match v1.clone() {
            Adt1::C0(h1123, _, _, _, _, _, _, _, _, _, _) => {
                h1123
            }
        };
        nat_eq(a1124, a1125)
    } else {
        false
    } {
        let a1128 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1126) => {
                h1126
            }
        };
        let a1129 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1127) => {
                h1127
            }
        };
        nat_eq(a1128, a1129)
    } else {
        false
    } {
        let a1132 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1130, _, _, _, _, _) => {
                h1130
            }
        };
        let a1133 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1131, _, _, _, _, _) => {
                h1131
            }
        };
        nat_eq(a1132, a1133)
    } else {
        false
    } {
        let a1136 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1134, _, _, _, _) => {
                h1134
            }
        };
        let a1137 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1135, _, _, _, _) => {
                h1135
            }
        };
        nat_eq(a1136, a1137)
    } else {
        false
    } {
        let a1140 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1138, _, _, _) => {
                h1138
            }
        };
        let a1141 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1139, _, _, _) => {
                h1139
            }
        };
        nat_eq(a1140, a1141)
    } else {
        false
    } {
        let a1144 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1142, _, _) => {
                h1142
            }
        };
        let a1145 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1143, _, _) => {
                h1143
            }
        };
        nat_eq(a1144, a1145)
    } else {
        false
    }
}

pub fn f38(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m1146 = v0;
    match m1146 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1148 = match v3.clone() {
                Adt1::C0(h1147, _, _, _, _, _, _, _, _, _, _) => {
                    h1147
                }
            };
            let a1149 = 0u64;
            if if if if if if if nat_eq(a1148, a1149) {
                true
            } else {
                let a1151 = match v3.clone() {
                    Adt1::C0(h1150, _, _, _, _, _, _, _, _, _, _) => {
                        h1150
                    }
                };
                let a1152 = 1u64;
                nat_eq(a1151, a1152)
            } {
                f20(v3.clone(), {
                    let a1154 = match v3.clone() {
                        Adt1::C0(h1153, _, _, _, _, _, _, _, _, _, _) => {
                            h1153
                        }
                    };
                    let a1155 = 0u64;
                    if nat_eq(a1154, a1155) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a1158 = match v3.clone() {
                    Adt1::C0(_, h1156, _, _, _, _, _, _, _, _, _) => {
                        h1156
                    }
                };
                let a1159 = match v1.clone() {
                    Adt1::C0(_, h1157, _, _, _, _, _, _, _, _, _) => {
                        h1157
                    }
                };
                nat_eq(a1158, a1159)
            } else {
                false
            } {
                let a1162 = match v1.clone() {
                    Adt1::C0(_, _, h1160, _, _, _, _, _, _, _, _) => {
                        h1160
                    }
                };
                let a1163 = match v3.clone() {
                    Adt1::C0(_, _, h1161, _, _, _, _, _, _, _, _) => {
                        h1161
                    }
                };
                nat_le(a1162, a1163)
            } else {
                false
            } {
                f17(match v1.clone() {
                    Adt1::C0(_, _, _, h1164, _, _, _, _, _, _, _) => {
                        h1164
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h1165, _, _, _, _, _, _, _) => {
                        h1165
                    }
                })
            } else {
                false
            } {
                f17(match v1 {
                    Adt1::C0(_, _, _, _, h1166, _, _, _, _, _, _) => {
                        h1166
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h1167, _, _, _, _, _, _) => {
                        h1167
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
    let m1168 = v0;
    match m1168 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a1170 = match v3.clone() {
                Adt1::C0(h1169, _, _, _, _, _, _, _, _, _, _) => {
                    h1169
                }
            };
            let a1171 = 0u64;
            if if if if if if nat_eq(a1170, a1171) {
                let a1173 = match v3.clone() {
                    Adt1::C0(_, h1172, _, _, _, _, _, _, _, _, _) => {
                        h1172
                    }
                };
                let a1174 = 1u64;
                if nat_eq(a1173, a1174) {
                    true
                } else {
                    let a1176 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h1175, _, _, _, _, _) => {
                            h1175
                        }
                    };
                    let a1177 = 4u64;
                    nat_eq(a1176, a1177)
                }
            } else {
                false
            } {
                let a1180 = match v3.clone() {
                    Adt1::C0(_, h1178, _, _, _, _, _, _, _, _, _) => {
                        h1178
                    }
                };
                let a1181 = match v1 {
                    Adt1::C0(_, _, _, h1179, _, _, _, _, _, _, _) => {
                        h1179
                    }
                };
                nat_eq(a1180, a1181)
            } else {
                false
            } {
                let a1183 = match v3.clone() {
                    Adt1::C0(_, _, h1182, _, _, _, _, _, _, _, _) => {
                        h1182
                    }
                };
                let a1184 = 1u64;
                nat_eq(a1183, a1184)
            } else {
                false
            } {
                let a1186 = match v3.clone() {
                    Adt1::C0(_, _, _, h1185, _, _, _, _, _, _, _) => {
                        h1185
                    }
                };
                let a1187 = 1u64;
                nat_eq(a1186, a1187)
            } else {
                false
            } {
                let a1189 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h1188, _, _, _, _, _, _) => {
                        h1188
                    }
                };
                let a1190 = 1u64;
                nat_eq(a1189, a1190)
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
        8u64
    } else {
        if f45(v0) {
            9u64
        } else {
            f46(v0)
        }
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    let a1193 = match v0.clone() {
        Adt1::C0(_, h1191, _, _, _, _, _, _, _, _, _) => {
            h1191
        }
    };
    let a1194 = match v1.clone() {
        Adt1::C0(_, h1192, _, _, _, _, _, _, _, _, _) => {
            h1192
        }
    };
    if if if nat_eq(a1193, a1194) {
        let a1197 = match v0.clone() {
            Adt1::C0(_, _, _, h1195, _, _, _, _, _, _, _) => {
                h1195
            }
        };
        let a1198 = match v1.clone() {
            Adt1::C0(_, _, _, h1196, _, _, _, _, _, _, _) => {
                h1196
            }
        };
        nat_eq(a1197, a1198)
    } else {
        false
    } {
        let a1200 = 0u64;
        let a1201 = match v0.clone() {
            Adt1::C0(_, _, h1199, _, _, _, _, _, _, _, _) => {
                h1199
            }
        };
        nat_lt(a1200, a1201)
    } else {
        false
    } {
        f17(match v1 {
            Adt1::C0(_, _, _, _, h1202, _, _, _, _, _, _) => {
                h1202
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, h1203, _, _, _, _, _, _) => {
                h1203
            }
        })
    } else {
        false
    }
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> bool {
    let m1204 = f47(v0);
    match m1204 {
        Adt8::C0 => {
            false
        }
        Adt8::C1(v3) => {
            let a1205 = v0;
            let a1206 = 0u64;
            if if if if if if nat_eq(a1205, a1206) {
                true
            } else {
                let a1207 = v0;
                let a1208 = 1u64;
                nat_eq(a1207, a1208)
            } {
                true
            } else {
                let a1209 = v0;
                let a1210 = 30u64;
                nat_eq(a1209, a1210)
            } {
                true
            } else {
                let a1211 = v0;
                let a1212 = 43u64;
                nat_eq(a1211, a1212)
            } {
                true
            } else {
                f48(v0)
            } {
                let a1214 = v2;
                let a1215 = match v3.clone() {
                    Adt9::C0(_, _, h1213) => {
                        h1213
                    }
                };
                nat_eq(a1214, a1215)
            } else {
                false
            } {
                f17(v1, match v3 {
                    Adt9::C0(_, h1216, _) => {
                        h1216
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1217 = v1;
    let a1218 = 1u64;
    if if if nat_le(a1217, a1218) {
        true
    } else {
        let a1219 = v0;
        let a1220 = v2;
        nat_le(a1219, a1220)
    } {
        let a1221 = v3;
        let a1222 = 1u64;
        if nat_le(a1221, a1222) {
            true
        } else {
            let a1229 = {
                let a1227 = v0;
                let a1228 = {
                    let a1225 = {
                        let a1223 = v1;
                        let a1224 = 1u64;
                        nat_sub(a1223, a1224)
                    };
                    let a1226 = v2;
                    nat_mul(a1225, a1226)?
                };
                nat_add(a1227, a1228)?
            };
            let a1230 = v4;
            nat_le(a1229, a1230)
        }
    } else {
        false
    } {
        let a1231 = v5;
        let a1232 = 1u64;
        if nat_le(a1231, a1232) {
            Ok(true)
        } else {
            let a1245 = {
                let a1243 = {
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
                let a1244 = {
                    let a1241 = {
                        let a1239 = v3;
                        let a1240 = 1u64;
                        nat_sub(a1239, a1240)
                    };
                    let a1242 = v4;
                    nat_mul(a1241, a1242)?
                };
                nat_add(a1243, a1244)?
            };
            let a1246 = v6;
            Ok(nat_le(a1245, a1246))
        }
    } else {
        Ok(false)
    }
}

pub fn f44(v0: u64) -> bool {
    let a1247 = v0;
    let a1248 = 2u64;
    if if if if nat_eq(a1247, a1248) {
        true
    } else {
        let a1249 = v0;
        let a1250 = 6u64;
        nat_eq(a1249, a1250)
    } {
        true
    } else {
        let a1251 = v0;
        let a1252 = 8u64;
        nat_eq(a1251, a1252)
    } {
        true
    } else {
        let a1253 = v0;
        let a1254 = 20u64;
        nat_eq(a1253, a1254)
    } {
        true
    } else {
        f49(v0)
    }
}

pub fn f45(v0: u64) -> bool {
    let a1255 = v0;
    let a1256 = 3u64;
    if nat_eq(a1255, a1256) {
        true
    } else {
        let a1257 = v0;
        let a1258 = 7u64;
        nat_eq(a1257, a1258)
    }
}

pub fn f46(v0: u64) -> u64 {
    let a1259 = v0;
    let a1260 = 30u64;
    if nat_eq(a1259, a1260) {
        30u64
    } else {
        0u64
    }
}

pub fn f47(v0: u64) -> Adt8 {
    let a1261 = v0;
    let a1262 = 0u64;
    if nat_eq(a1261, a1262) {
        Adt8::C1(Adt9::C0(0u64, 1u64, 4u64))
    } else {
        let a1263 = v0;
        let a1264 = 1u64;
        if nat_eq(a1263, a1264) {
            Adt8::C1(Adt9::C0(1u64, 1u64, 2u64))
        } else {
            let a1265 = v0;
            let a1266 = 2u64;
            if nat_eq(a1265, a1266) {
                Adt8::C1(Adt9::C0(2u64, 32u64, 18u64))
            } else {
                let a1267 = v0;
                let a1268 = 3u64;
                if nat_eq(a1267, a1268) {
                    Adt8::C1(Adt9::C0(3u64, 32u64, 20u64))
                } else {
                    let a1269 = v0;
                    let a1270 = 6u64;
                    if nat_eq(a1269, a1270) {
                        Adt8::C1(Adt9::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1271 = v0;
                        let a1272 = 7u64;
                        if nat_eq(a1271, a1272) {
                            Adt8::C1(Adt9::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1273 = v0;
                            let a1274 = 8u64;
                            if nat_eq(a1273, a1274) {
                                Adt8::C1(Adt9::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1275 = v0;
                                let a1276 = 9u64;
                                if nat_eq(a1275, a1276) {
                                    Adt8::C1(Adt9::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1277 = v0;
                                    let a1278 = 10u64;
                                    if nat_eq(a1277, a1278) {
                                        Adt8::C1(Adt9::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1279 = v0;
                                        let a1280 = 11u64;
                                        if nat_eq(a1279, a1280) {
                                            Adt8::C1(Adt9::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1281 = v0;
                                            let a1282 = 12u64;
                                            if nat_eq(a1281, a1282) {
                                                Adt8::C1(Adt9::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1283 = v0;
                                                let a1284 = 13u64;
                                                if nat_eq(a1283, a1284) {
                                                    Adt8::C1(Adt9::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1285 = v0;
                                                    let a1286 = 14u64;
                                                    if nat_eq(a1285, a1286) {
                                                        Adt8::C1(Adt9::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1287 = v0;
                                                        let a1288 = 15u64;
                                                        if nat_eq(a1287, a1288) {
                                                            Adt8::C1(Adt9::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1289 = v0;
                                                            let a1290 = 16u64;
                                                            if nat_eq(a1289, a1290) {
                                                                Adt8::C1(Adt9::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1291 = v0;
                                                                let a1292 = 17u64;
                                                                if nat_eq(a1291, a1292) {
                                                                    Adt8::C1(Adt9::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1293 = v0;
                                                                    let a1294 = 18u64;
                                                                    if nat_eq(a1293, a1294) {
                                                                        Adt8::C1(Adt9::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1295 = v0;
                                                                        let a1296 = 19u64;
                                                                        if nat_eq(a1295, a1296) {
                                                                            Adt8::C1(Adt9::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1297 = v0;
                                                                            let a1298 = 20u64;
                                                                            if nat_eq(a1297, a1298) {
                                                                                Adt8::C1(Adt9::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1299 = v0;
                                                                                let a1300 = 21u64;
                                                                                if nat_eq(a1299, a1300) {
                                                                                    Adt8::C1(Adt9::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1301 = v0;
                                                                                    let a1302 = 22u64;
                                                                                    if nat_eq(a1301, a1302) {
                                                                                        Adt8::C1(Adt9::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1303 = v0;
                                                                                        let a1304 = 23u64;
                                                                                        if nat_eq(a1303, a1304) {
                                                                                            Adt8::C1(Adt9::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1305 = v0;
                                                                                            let a1306 = 24u64;
                                                                                            if nat_eq(a1305, a1306) {
                                                                                                Adt8::C1(Adt9::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1307 = v0;
                                                                                                let a1308 = 25u64;
                                                                                                if nat_eq(a1307, a1308) {
                                                                                                    Adt8::C1(Adt9::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1309 = v0;
                                                                                                    let a1310 = 26u64;
                                                                                                    if nat_eq(a1309, a1310) {
                                                                                                        Adt8::C1(Adt9::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1311 = v0;
                                                                                                        let a1312 = 27u64;
                                                                                                        if nat_eq(a1311, a1312) {
                                                                                                            Adt8::C1(Adt9::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1313 = v0;
                                                                                                            let a1314 = 28u64;
                                                                                                            if nat_eq(a1313, a1314) {
                                                                                                                Adt8::C1(Adt9::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1315 = v0;
                                                                                                                let a1316 = 29u64;
                                                                                                                if nat_eq(a1315, a1316) {
                                                                                                                    Adt8::C1(Adt9::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1317 = v0;
                                                                                                                    let a1318 = 30u64;
                                                                                                                    if nat_eq(a1317, a1318) {
                                                                                                                        Adt8::C1(Adt9::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1319 = v0;
                                                                                                                        let a1320 = 34u64;
                                                                                                                        if nat_eq(a1319, a1320) {
                                                                                                                            Adt8::C1(Adt9::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1321 = v0;
                                                                                                                            let a1322 = 35u64;
                                                                                                                            if nat_eq(a1321, a1322) {
                                                                                                                                Adt8::C1(Adt9::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1323 = v0;
                                                                                                                                let a1324 = 39u64;
                                                                                                                                if nat_eq(a1323, a1324) {
                                                                                                                                    Adt8::C1(Adt9::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1325 = v0;
                                                                                                                                    let a1326 = 40u64;
                                                                                                                                    if nat_eq(a1325, a1326) {
                                                                                                                                        Adt8::C1(Adt9::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1327 = v0;
                                                                                                                                        let a1328 = 41u64;
                                                                                                                                        if nat_eq(a1327, a1328) {
                                                                                                                                            Adt8::C1(Adt9::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1329 = v0;
                                                                                                                                            let a1330 = 42u64;
                                                                                                                                            if nat_eq(a1329, a1330) {
                                                                                                                                                Adt8::C1(Adt9::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1331 = v0;
                                                                                                                                                let a1332 = 43u64;
                                                                                                                                                if nat_eq(a1331, a1332) {
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

pub fn f48(v0: u64) -> bool {
    if f44(v0) {
        true
    } else {
        f45(v0)
    }
}

pub fn f49(v0: u64) -> bool {
    let a1333 = v0;
    let a1334 = 41u64;
    if nat_eq(a1333, a1334) {
        true
    } else {
        let a1335 = v0;
        let a1336 = 42u64;
        nat_eq(a1335, a1336)
    }
}
