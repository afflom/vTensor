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
    C0(u64, Adt1, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Adt2),
    C1(u64, Adt3, Adt3, Adt4),
    C2(u64, Adt3, Adt3, u64),
    C3(u64, Adt3, Adt3, Adt3, u64),
    C4(u64, u64, Adt3, Adt3, Adt3, u64),
    C5(u64, Adt3, Adt3, Adt3, Adt3, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2(u64, Adt3),
    C3(u64, u64, u64, u64, bool),
    C4(u64),
    C5(u64, u64, u64, u64),
    C6(u64, u64, bool),
    C7(u64),
    C8(u64),
    C9(u64, u64),
    C10(u64, bool, Adt3),
    C11(u64, u64),
    C12(bool, u64, u64),
    C13(u64, u64, u64, u64),
    C14(u64, u64, u64),
    C15,
    C16(u64, u64),
    C17,
    C18(u64, Adt3),
    C19(u64, Adt3),
    C20(u64, Adt3),
    C21,
    C22(Adt3, Adt3),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
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

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, _, _, _, _, _, _, _, v13) => {
            let m2 = v3;
            match m2 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v14) => {
                    let a3 = v1;
                    let a4 = 3u64;
                    if nat_eq(a3, a4) {
                        Ok(Adt5::C0(f1(v14, v4, v5, v13)?))
                    } else {
                        f2(v1, v2, v14, v4, v13)
                    }
                }
            }
        }
        Adt0::C1(v15, v16, v17, v18) => {
            f3(v15, v16, v17, v18)
        }
        Adt0::C2(v19, v20, v21, v22) => {
            f4(v19, v20, v21, v22)
        }
        Adt0::C3(v23, v24, v25, v26, v27) => {
            f5(v23, v24, v25, v26, v27)
        }
        Adt0::C4(v28, v29, v30, v31, v32, v33) => {
            f6(v28, v29, v30, v31, v32, v33)
        }
        Adt0::C5(v34, v35, v36, v37, v38, v39) => {
            f7(v34, v35, v36, v37, v38, v39)
        }
    }
}

pub fn f1(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m5 = v1;
    match m5 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m6 = v2;
            match m6 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a8 = match v0.clone() {
                        Adt2::C0(h7, _, _, _, _, _, _, _, _, _, _) => {
                            h7
                        }
                    };
                    let a9 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a8, a9) {
                        let a11 = match v4.clone() {
                            Adt2::C0(h10, _, _, _, _, _, _, _, _, _, _) => {
                                h10
                            }
                        };
                        let a12 = 0u64;
                        nat_eq(a11, a12)
                    } else {
                        false
                    } {
                        let a14 = match v5.clone() {
                            Adt2::C0(h13, _, _, _, _, _, _, _, _, _, _) => {
                                h13
                            }
                        };
                        let a15 = 26u64;
                        nat_eq(a14, a15)
                    } else {
                        false
                    } {
                        let a17 = match v3.clone() {
                            Adt2::C0(h16, _, _, _, _, _, _, _, _, _, _) => {
                                h16
                            }
                        };
                        let a18 = 0u64;
                        nat_eq(a17, a18)
                    } else {
                        false
                    } {
                        f8(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a21 = match v0.clone() {
                            Adt2::C0(_, h19, _, _, _, _, _, _, _, _, _) => {
                                h19
                            }
                        };
                        let a22 = match v4.clone() {
                            Adt2::C0(_, h20, _, _, _, _, _, _, _, _, _) => {
                                h20
                            }
                        };
                        nat_eq(a21, a22)
                    } else {
                        false
                    } {
                        let a25 = match v0.clone() {
                            Adt2::C0(_, _, h23, _, _, _, _, _, _, _, _) => {
                                h23
                            }
                        };
                        let a26 = match v5.clone() {
                            Adt2::C0(_, h24, _, _, _, _, _, _, _, _, _) => {
                                h24
                            }
                        };
                        nat_eq(a25, a26)
                    } else {
                        false
                    } {
                        let a29 = match v0.clone() {
                            Adt2::C0(_, _, _, h27, _, _, _, _, _, _, _) => {
                                h27
                            }
                        };
                        let a30 = match v5.clone() {
                            Adt2::C0(_, _, h28, _, _, _, _, _, _, _, _) => {
                                h28
                            }
                        };
                        nat_eq(a29, a30)
                    } else {
                        false
                    } {
                        let a32 = 0u64;
                        let a33 = match v5.clone() {
                            Adt2::C0(_, _, _, h31, _, _, _, _, _, _, _) => {
                                h31
                            }
                        };
                        nat_lt(a32, a33)
                    } else {
                        false
                    } {
                        let a35 = 0u64;
                        let a36 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h34, _, _, _, _, _, _) => {
                                h34
                            }
                        };
                        nat_lt(a35, a36)
                    } else {
                        false
                    } {
                        let a38 = 4u64;
                        let a39 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h37, _, _, _, _, _) => {
                                h37
                            }
                        };
                        nat_le(a38, a39)
                    } else {
                        false
                    } {
                        let a41 = 4u64;
                        let a42 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h40, _, _, _, _, _) => {
                                h40
                            }
                        };
                        nat_le(a41, a42)
                    } else {
                        false
                    } {
                        let a44 = 4u64;
                        let a45 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h43, _, _, _, _, _) => {
                                h43
                            }
                        };
                        nat_le(a44, a45)
                    } else {
                        false
                    } {
                        let a47 = 4u64;
                        let a48 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h46, _, _, _, _, _) => {
                                h46
                            }
                        };
                        nat_le(a47, a48)
                    } else {
                        false
                    } {
                        f9(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f9(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f9(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f10(match v0.clone() {
                            Adt2::C0(_, h49, _, _, _, _, _, _, _, _, _) => {
                                h49
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h50, _, _, _, _, _, _, _, _) => {
                                h50
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h51, _, _, _, _, _, _, _) => {
                                h51
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h52, _, _, _, _, _, _) => {
                                h52
                            }
                        })?, 4u64, f11(v0, 1u64), f11(v3, 1u64), Adt4::C22(f11(v4, 1u64), f11(v5, 1u64))))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt5> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h53, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h53
        }
    };
    Ok(Adt5::C0({
        let a54 = v0;
        let a55 = 39u64;
        if nat_eq(a54, a55) {
            f12(v2, v3, v4)?
        } else {
            let a56 = v0;
            let a57 = 77u64;
            if nat_eq(a56, a57) {
                f13(v2, v4)?
            } else {
                let a58 = v0;
                let a59 = 40u64;
                if nat_eq(a58, a59) {
                    f14(v2, v3, v4)?
                } else {
                    if f15(v0) {
                        f16(v0, v2, {
                            let m60 = v3;
                            match m60 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a61 = v0;
                        let a62 = 19u64;
                        if nat_eq(a61, a62) {
                            f17(v2, v4)?
                        } else {
                            let a63 = v0;
                            let a64 = 21u64;
                            if nat_eq(a63, a64) {
                                f18(v5, v2, v3, v4)?
                            } else {
                                let a65 = v0;
                                let a66 = 61u64;
                                if if if nat_eq(a65, a66) {
                                    true
                                } else {
                                    let a67 = v0;
                                    let a68 = 62u64;
                                    nat_eq(a67, a68)
                                } {
                                    true
                                } else {
                                    let a69 = v0;
                                    let a70 = 63u64;
                                    nat_eq(a69, a70)
                                } {
                                    f19(v0, v1, v2, v4)?
                                } else {
                                    let a71 = v0;
                                    let a72 = 4u64;
                                    if if nat_eq(a71, a72) {
                                        true
                                    } else {
                                        let a73 = v0;
                                        let a74 = 32u64;
                                        nat_eq(a73, a74)
                                    } {
                                        f20(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a75 = v0;
                                        let a76 = 20u64;
                                        if nat_eq(a75, a76) {
                                            f21(v2, v4)?
                                        } else {
                                            let a77 = v0;
                                            let a78 = 42u64;
                                            if nat_eq(a77, a78) {
                                                f22(4u64, v2, v4)?
                                            } else {
                                                let a79 = v0;
                                                let a80 = 69u64;
                                                if nat_eq(a79, a80) {
                                                    let a81 = v5;
                                                    let a82 = 4u64;
                                                    if nat_lt(a81, a82) {
                                                        f22(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a83 = v0;
                                                    let a84 = 70u64;
                                                    if nat_eq(a83, a84) {
                                                        f23(v5, v2, v4)?
                                                    } else {
                                                        let a85 = v0;
                                                        let a86 = 75u64;
                                                        if if nat_eq(a85, a86) {
                                                            true
                                                        } else {
                                                            let a87 = v0;
                                                            let a88 = 76u64;
                                                            nat_eq(a87, a88)
                                                        } {
                                                            f24({
                                                                let a89 = v0;
                                                                let a90 = 76u64;
                                                                nat_eq(a89, a90)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a91 = v0;
                                                            let a92 = 31u64;
                                                            if if if nat_eq(a91, a92) {
                                                                true
                                                            } else {
                                                                let a93 = v0;
                                                                let a94 = 47u64;
                                                                nat_eq(a93, a94)
                                                            } {
                                                                true
                                                            } else {
                                                                let a95 = v0;
                                                                let a96 = 68u64;
                                                                nat_eq(a95, a96)
                                                            } {
                                                                f25(v0, v1, v2, v4)?
                                                            } else {
                                                                let a97 = v0;
                                                                let a98 = 15u64;
                                                                if nat_eq(a97, a98) {
                                                                    f26(v2, v4)?
                                                                } else {
                                                                    let a99 = v0;
                                                                    let a100 = 17u64;
                                                                    if nat_eq(a99, a100) {
                                                                        f27(v2, v4)?
                                                                    } else {
                                                                        let a101 = v0;
                                                                        let a102 = 18u64;
                                                                        if nat_eq(a101, a102) {
                                                                            f28(v2, v3, v4)?
                                                                        } else {
                                                                            let a103 = v0;
                                                                            let a104 = 13u64;
                                                                            if if if nat_eq(a103, a104) {
                                                                                true
                                                                            } else {
                                                                                let a105 = v0;
                                                                                let a106 = 14u64;
                                                                                nat_eq(a105, a106)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a107 = v0;
                                                                                let a108 = 16u64;
                                                                                nat_eq(a107, a108)
                                                                            } {
                                                                                f29(v0, v2, v4)?
                                                                            } else {
                                                                                let m109 = f30(v0, v5);
                                                                                match m109 {
                                                                                    None => {
                                                                                        f31(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f32(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }))
}

pub fn f3(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m110 = v3;
    match m110 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f33(v0, v1)?, f33(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f35(v0, v1, v2.clone())?, f33(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f36(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f37(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f38(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f37(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f39(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f40(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f41(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f33(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f42(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f33(v0, v1)?, f33(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f43(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f33(v0, v1)?, f33(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f33(v0, v1)?, f33(v0, v2)?))
        }
        Adt4::C15 => {
            f44(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f45(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f46(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f33(f34(v0, match v2.clone() {
                Adt3::C0(h111, _, _, _, _, _, _, _) => {
                    h111
                }
            }, match v2.clone() {
                Adt3::C0(_, h112, _, _, _, _, _, _) => {
                    h112
                }
            })?, v1)?, f33(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f4(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f47(match v1.clone() {
        Adt3::C0(h113, _, _, _, _, _, _, _) => {
            h113
        }
    })?;
    let v5: u64 = {
        let a114 = v0;
        let a115 = v4;
        let a116 = 0u64;
        nat_quot(a114, a115, a116)
    };
    let v6: u64 = {
        let a117 = v0;
        let a118 = v4;
        let a119 = 0u64;
        nat_rem(a117, a118, a119)
    };
    let a120 = v6;
    let a121 = 0u64;
    if nat_eq(a120, a121) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a122 = v6;
        let a123 = 1u64;
        if nat_eq(a122, a123) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a126 = {
                let a124 = v6;
                let a125 = 1u64;
                nat_add(a124, a125)?
            };
            let a127 = v4;
            if nat_eq(a126, a127) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f33(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a128 = v6;
                    let a129 = 2u64;
                    nat_sub(a128, a129)
                };
                let v8: u64 = {
                    let a130 = v7;
                    let a131 = 3u64;
                    let a132 = 0u64;
                    nat_quot(a130, a131, a132)
                };
                let v9: u64 = {
                    let a133 = v7;
                    let a134 = 3u64;
                    let a135 = 0u64;
                    nat_rem(a133, a134, a135)
                };
                let v10: u64 = f33({
                    let a139 = {
                        let a137 = v5;
                        let a138 = match v1.clone() {
                            Adt3::C0(h136, _, _, _, _, _, _, _) => {
                                h136
                            }
                        };
                        nat_mul(a137, a138)?
                    };
                    let a140 = v8;
                    nat_add(a139, a140)?
                }, v1)?;
                let a141 = v9;
                let a142 = 0u64;
                if nat_eq(a141, a142) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a143 = v9;
                    let a144 = 1u64;
                    if nat_eq(a143, a144) {
                        let a145 = v3;
                        let a146 = 2u64;
                        if nat_eq(a145, a146) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a147 = v3;
                        let a148 = 2u64;
                        if if nat_eq(a147, a148) {
                            true
                        } else {
                            let a149 = v3;
                            let a150 = 3u64;
                            nat_eq(a149, a150)
                        } {
                            Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(19u64, 2u64, v8, 2u64, 0u64, 1u64, 1u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a152 = 2u64;
        let a153 = match v1.clone() {
            Adt3::C0(_, h151, _, _, _, _, _, _) => {
                h151
            }
        };
        nat_mul(a152, a153)?
    };
    let a154 = v0;
    let a155 = v5;
    if nat_lt(a154, a155) {
        let a159 = {
            let a156 = v0;
            let a157 = 2u64;
            let a158 = 0u64;
            nat_rem(a156, a157, a158)
        };
        let a160 = 0u64;
        if nat_eq(a159, a160) {
            Ok(Adt5::C4(19u64, 4u64, f33({
                let a161 = v0;
                let a162 = 2u64;
                let a163 = 0u64;
                nat_quot(a161, a162, a163)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a164 = v4;
            let a165 = 2147483648u64;
            if if nat_lt(a164, a165) {
                let a167 = v4;
                let a168 = match v2 {
                    Adt3::C0(_, h166, _, _, _, _, _, _) => {
                        h166
                    }
                };
                nat_lt(a167, a168)
            } else {
                false
            } {
                Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v6: u64 = {
            let a171 = match v2.clone() {
                Adt3::C0(h169, _, _, _, _, _, _, _) => {
                    h169
                }
            };
            let a172 = match v2.clone() {
                Adt3::C0(_, h170, _, _, _, _, _, _) => {
                    h170
                }
            };
            nat_mul(a171, a172)?
        };
        let a175 = {
            let a173 = v0;
            let a174 = v5;
            nat_sub(a173, a174)
        };
        let a176 = v6;
        if nat_lt(a175, a176) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f33({
                let a177 = v0;
                let a178 = v5;
                nat_sub(a177, a178)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a181 = {
                    let a179 = v0;
                    let a180 = v5;
                    nat_sub(a179, a180)
                };
                let a182 = v6;
                nat_sub(a181, a182)
            };
            let v8: u64 = {
                let a186 = 1u64;
                let a187 = {
                    let a184 = 2u64;
                    let a185 = match v1.clone() {
                        Adt3::C0(h183, _, _, _, _, _, _, _) => {
                            h183
                        }
                    };
                    nat_mul(a184, a185)?
                };
                nat_add(a186, a187)?
            };
            let v9: u64 = {
                let a188 = v7;
                let a189 = v8;
                let a190 = 0u64;
                nat_quot(a188, a189, a190)
            };
            let v10: u64 = {
                let a191 = v7;
                let a192 = v8;
                let a193 = 0u64;
                nat_rem(a191, a192, a193)
            };
            let a194 = v10;
            let a195 = 0u64;
            if nat_eq(a194, a195) {
                Ok(Adt5::C4(19u64, 4u64, f33(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a199 = {
                    let a196 = v10;
                    let a197 = 2u64;
                    let a198 = 0u64;
                    nat_rem(a196, a197, a198)
                };
                let a200 = 1u64;
                if nat_eq(a199, a200) {
                    Ok(Adt5::C4(6u64, 0u64, f33({
                        let a209 = {
                            let a202 = v9;
                            let a203 = match v1.clone() {
                                Adt3::C0(h201, _, _, _, _, _, _, _) => {
                                    h201
                                }
                            };
                            nat_mul(a202, a203)?
                        };
                        let a210 = {
                            let a206 = {
                                let a204 = v10;
                                let a205 = 1u64;
                                nat_sub(a204, a205)
                            };
                            let a207 = 2u64;
                            let a208 = 0u64;
                            nat_quot(a206, a207, a208)
                        };
                        nat_add(a209, a210)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f33({
                        let a219 = {
                            let a212 = v4;
                            let a213 = match v1.clone() {
                                Adt3::C0(h211, _, _, _, _, _, _, _) => {
                                    h211
                                }
                            };
                            nat_mul(a212, a213)?
                        };
                        let a220 = {
                            let a216 = {
                                let a214 = v10;
                                let a215 = 1u64;
                                nat_sub(a214, a215)
                            };
                            let a217 = 2u64;
                            let a218 = 0u64;
                            nat_quot(a216, a217, a218)
                        };
                        nat_add(a219, a220)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f33({
                        let a229 = {
                            let a222 = v4;
                            let a223 = match v1 {
                                Adt3::C0(h221, _, _, _, _, _, _, _) => {
                                    h221
                                }
                            };
                            nat_mul(a222, a223)?
                        };
                        let a230 = {
                            let a226 = {
                                let a224 = v10;
                                let a225 = 1u64;
                                nat_sub(a224, a225)
                            };
                            let a227 = 2u64;
                            let a228 = 0u64;
                            nat_quot(a226, a227, a228)
                        };
                        nat_add(a229, a230)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a231 = v1;
    let a232 = f48(v2.clone(), v4.clone())?;
    if nat_lt(a231, a232) {
        let a235 = v1;
        let a236 = {
            let a233 = 2u64;
            let a234 = f49(v4.clone())?;
            nat_mul(a233, a234)?
        };
        if nat_lt(a235, a236) {
            let a240 = {
                let a237 = v1;
                let a238 = 2u64;
                let a239 = 0u64;
                nat_rem(a237, a238, a239)
            };
            let a241 = 0u64;
            if nat_eq(a240, a241) {
                f50({
                    let a242 = v1;
                    let a243 = 2u64;
                    let a244 = 0u64;
                    nat_quot(a242, a243, a244)
                }, v4)
            } else {
                if f51(match v2 {
                    Adt3::C0(_, h245, _, _, _, _, _, _) => {
                        h245
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a248 = v1;
                let a249 = {
                    let a246 = 2u64;
                    let a247 = f49(v4.clone())?;
                    nat_mul(a246, a247)?
                };
                nat_sub(a248, a249)
            };
            let v7: u64 = {
                let a253 = v6;
                let a254 = {
                    let a251 = 1u64;
                    let a252 = match v2.clone() {
                        Adt3::C0(h250, _, _, _, _, _, _, _) => {
                            h250
                        }
                    };
                    nat_add(a251, a252)?
                };
                let a255 = 0u64;
                nat_quot(a253, a254, a255)
            };
            let v8: u64 = {
                let a259 = v6;
                let a260 = {
                    let a257 = 1u64;
                    let a258 = match v2.clone() {
                        Adt3::C0(h256, _, _, _, _, _, _, _) => {
                            h256
                        }
                    };
                    nat_add(a257, a258)?
                };
                let a261 = 0u64;
                nat_rem(a259, a260, a261)
            };
            let a262 = v8;
            let a263 = 0u64;
            if nat_eq(a262, a263) {
                f50(v7, v4)
            } else {
                f52(v0, v7, {
                    let a264 = v8;
                    let a265 = 1u64;
                    nat_sub(a264, a265)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a268 = match v4.clone() {
            Adt3::C0(h266, _, _, _, _, _, _, _) => {
                h266
            }
        };
        let a269 = match v4.clone() {
            Adt3::C0(_, h267, _, _, _, _, _, _) => {
                h267
            }
        };
        nat_mul(a268, a269)?
    };
    let v7: u64 = {
        let a270 = 2u64;
        let a271 = v6;
        nat_mul(a270, a271)?
    };
    let v8: u64 = f53(match v3.clone() {
        Adt3::C0(_, h272, _, _, _, _, _, _) => {
            h272
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h273, _, _, _, _, _) => {
            h273
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h274, _, _, _, _) => {
            h274
        }
    });
    let a275 = v0;
    let a276 = v7;
    if nat_lt(a275, a276) {
        let a280 = {
            let a277 = v0;
            let a278 = 2u64;
            let a279 = 0u64;
            nat_rem(a277, a278, a279)
        };
        let a281 = 0u64;
        if nat_eq(a280, a281) {
            Ok(Adt5::C4(19u64, 5u64, f33({
                let a282 = v0;
                let a283 = 2u64;
                let a284 = 0u64;
                nat_quot(a282, a283, a284)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f54(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a285 = v0;
            let a286 = v7;
            nat_sub(a285, a286)
        };
        let v10: u64 = {
            let a290 = v9;
            let a291 = {
                let a288 = 1u64;
                let a289 = match v1.clone() {
                    Adt3::C0(h287, _, _, _, _, _, _, _) => {
                        h287
                    }
                };
                nat_add(a288, a289)?
            };
            let a292 = 0u64;
            nat_quot(a290, a291, a292)
        };
        let v11: u64 = {
            let a296 = v9;
            let a297 = {
                let a294 = 1u64;
                let a295 = match v1.clone() {
                    Adt3::C0(h293, _, _, _, _, _, _, _) => {
                        h293
                    }
                };
                nat_add(a294, a295)?
            };
            let a298 = 0u64;
            nat_rem(a296, a297, a298)
        };
        let a299 = v11;
        let a300 = 0u64;
        if nat_eq(a299, a300) {
            Ok(Adt5::C4(19u64, 5u64, f33({
                let a301 = v10;
                let a302 = v6;
                let a303 = 0u64;
                nat_rem(a301, a302, a303)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f54(v5, v8) {
                let v12: u64 = {
                    let a309 = {
                        let a305 = v10;
                        let a306 = match v1.clone() {
                            Adt3::C0(h304, _, _, _, _, _, _, _) => {
                                h304
                            }
                        };
                        nat_mul(a305, a306)?
                    };
                    let a310 = {
                        let a307 = v11;
                        let a308 = 1u64;
                        nat_sub(a307, a308)
                    };
                    nat_add(a309, a310)?
                };
                let a311 = v5;
                let a312 = 4294967295u64;
                if nat_eq(a311, a312) {
                    Ok(Adt5::C1(false, f33(v12, v1)?, f33(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f33(v12, v1)?, 4u64, f33({
                        let a318 = {
                            let a314 = v5;
                            let a315 = match v3.clone() {
                                Adt3::C0(h313, _, _, _, _, _, _, _) => {
                                    h313
                                }
                            };
                            nat_mul(a314, a315)?
                        };
                        let a319 = {
                            let a316 = v11;
                            let a317 = 1u64;
                            nat_sub(a316, a317)
                        };
                        nat_add(a318, a319)?
                    }, v3)?, 3u64, f33(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f8(v0: Adt2, v1: Adt2) -> bool {
    let a322 = match v0.clone() {
        Adt2::C0(_, h320, _, _, _, _, _, _, _, _, _) => {
            h320
        }
    };
    let a323 = match v1.clone() {
        Adt2::C0(_, h321, _, _, _, _, _, _, _, _, _) => {
            h321
        }
    };
    if if if nat_eq(a322, a323) {
        let a326 = match v0.clone() {
            Adt2::C0(_, _, h324, _, _, _, _, _, _, _, _) => {
                h324
            }
        };
        let a327 = match v1.clone() {
            Adt2::C0(_, _, h325, _, _, _, _, _, _, _, _) => {
                h325
            }
        };
        nat_eq(a326, a327)
    } else {
        false
    } {
        let a330 = match v0.clone() {
            Adt2::C0(_, _, _, h328, _, _, _, _, _, _, _) => {
                h328
            }
        };
        let a331 = match v1.clone() {
            Adt2::C0(_, _, _, h329, _, _, _, _, _, _, _) => {
                h329
            }
        };
        nat_eq(a330, a331)
    } else {
        false
    } {
        let a334 = match v0 {
            Adt2::C0(_, _, _, _, h332, _, _, _, _, _, _) => {
                h332
            }
        };
        let a335 = match v1 {
            Adt2::C0(_, _, _, _, h333, _, _, _, _, _, _) => {
                h333
            }
        };
        nat_eq(a334, a335)
    } else {
        false
    }
}

pub fn f9(v0: Adt2, v1: Adt2) -> R<bool> {
    let a337 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h336, _) => {
            h336
        }
    };
    let a338 = 0u64;
    if nat_eq(a337, a338) {
        Ok(true)
    } else {
        let a340 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h339, _) => {
                h339
            }
        };
        let a341 = 0u64;
        if nat_eq(a340, a341) {
            Ok(true)
        } else {
            let a343 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h342) => {
                    h342
                }
            };
            let a344 = 0u64;
            if nat_eq(a343, a344) {
                Ok(true)
            } else {
                let a346 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h345) => {
                        h345
                    }
                };
                let a347 = 0u64;
                if nat_eq(a346, a347) {
                    Ok(true)
                } else {
                    let a353 = {
                        let a350 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h348) => {
                                h348
                            }
                        };
                        let a351 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h349, _) => {
                                h349
                            }
                        };
                        nat_add(a350, a351)?
                    };
                    let a354 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h352) => {
                            h352
                        }
                    };
                    if nat_le(a353, a354) {
                        Ok(true)
                    } else {
                        let a360 = {
                            let a357 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h355) => {
                                    h355
                                }
                            };
                            let a358 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h356, _) => {
                                    h356
                                }
                            };
                            nat_add(a357, a358)?
                        };
                        let a361 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h359) => {
                                h359
                            }
                        };
                        Ok(nat_le(a360, a361))
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a362 = v0;
    let a363 = 0u64;
    if if if if nat_eq(a362, a363) {
        true
    } else {
        let a364 = v1;
        let a365 = 0u64;
        nat_eq(a364, a365)
    } {
        true
    } else {
        let a366 = v2;
        let a367 = 0u64;
        nat_eq(a366, a367)
    } {
        true
    } else {
        let a368 = v3;
        let a369 = 0u64;
        nat_eq(a368, a369)
    } {
        Ok(0u64)
    } else {
        let a382 = {
            let a372 = 2u64;
            let a373 = {
                let a370 = v1;
                let a371 = v2;
                nat_mul(a370, a371)?
            };
            nat_mul(a372, a373)?
        };
        let a383 = {
            let a380 = {
                let a376 = {
                    let a374 = v1;
                    let a375 = v2;
                    nat_mul(a374, a375)?
                };
                let a377 = v3;
                nat_mul(a376, a377)?
            };
            let a381 = {
                let a378 = 1u64;
                let a379 = v0;
                nat_add(a378, a379)?
            };
            nat_mul(a380, a381)?
        };
        nat_add(a382, a383)
    }
}

pub fn f11(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a385 = match v0.clone() {
            Adt2::C0(_, h384, _, _, _, _, _, _, _, _, _) => {
                h384
            }
        };
        let a386 = v1;
        let a387 = 0u64;
        nat_quot(a385, a386, a387)
    }, match v0.clone() {
        Adt2::C0(_, _, h388, _, _, _, _, _, _, _, _) => {
            h388
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h389, _, _, _, _, _, _, _) => {
            h389
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h390, _, _, _, _, _, _) => {
            h390
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h391, _, _, _, _, _) => {
            h391
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h392, _, _, _, _) => {
            h392
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h393, _, _, _) => {
            h393
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h394, _, _) => {
            h394
        }
    })
}

pub fn f12(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m395 = v1;
    match m395 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f55(v0, v3, v2)
        }
    }
}

pub fn f13(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a397 = match v0.clone() {
        Adt2::C0(h396, _, _, _, _, _, _, _, _, _, _) => {
            h396
        }
    };
    let a398 = 1u64;
    if if nat_eq(a397, a398) {
        true
    } else {
        let a400 = match v0.clone() {
            Adt2::C0(h399, _, _, _, _, _, _, _, _, _, _) => {
                h399
            }
        };
        let a401 = 30u64;
        nat_eq(a400, a401)
    } {
        let a403 = match v1.clone() {
            Adt2::C0(h402, _, _, _, _, _, _, _, _, _, _) => {
                h402
            }
        };
        let a404 = 1u64;
        if if if if if if if if if if if nat_eq(a403, a404) {
            let a407 = match v0.clone() {
                Adt2::C0(_, h405, _, _, _, _, _, _, _, _, _) => {
                    h405
                }
            };
            let a408 = match v1.clone() {
                Adt2::C0(_, h406, _, _, _, _, _, _, _, _, _) => {
                    h406
                }
            };
            nat_eq(a407, a408)
        } else {
            false
        } {
            let a410 = 0u64;
            let a411 = match v1.clone() {
                Adt2::C0(_, _, h409, _, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            nat_lt(a410, a411)
        } else {
            false
        } {
            let a414 = match v1.clone() {
                Adt2::C0(_, _, h412, _, _, _, _, _, _, _, _) => {
                    h412
                }
            };
            let a415 = match v1.clone() {
                Adt2::C0(_, _, _, h413, _, _, _, _, _, _, _) => {
                    h413
                }
            };
            nat_eq(a414, a415)
        } else {
            false
        } {
            let a417 = match v1.clone() {
                Adt2::C0(_, _, _, _, h416, _, _, _, _, _, _) => {
                    h416
                }
            };
            let a418 = 1u64;
            nat_eq(a417, a418)
        } else {
            false
        } {
            let a426 = match v0.clone() {
                Adt2::C0(_, _, h419, _, _, _, _, _, _, _, _) => {
                    h419
                }
            };
            let a427 = {
                let a424 = {
                    let a422 = match v1.clone() {
                        Adt2::C0(_, _, h420, _, _, _, _, _, _, _, _) => {
                            h420
                        }
                    };
                    let a423 = match v1.clone() {
                        Adt2::C0(_, _, h421, _, _, _, _, _, _, _, _) => {
                            h421
                        }
                    };
                    nat_add(a422, a423)?
                };
                let a425 = 1u64;
                nat_sub(a424, a425)
            };
            nat_eq(a426, a427)
        } else {
            false
        } {
            let a429 = 0u64;
            let a430 = match v0.clone() {
                Adt2::C0(_, _, _, h428, _, _, _, _, _, _, _) => {
                    h428
                }
            };
            nat_lt(a429, a430)
        } else {
            false
        } {
            let a432 = 0u64;
            let a433 = match v0.clone() {
                Adt2::C0(_, _, _, _, h431, _, _, _, _, _, _) => {
                    h431
                }
            };
            nat_lt(a432, a433)
        } else {
            false
        } {
            let a435 = 2u64;
            let a436 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h434, _, _, _, _, _) => {
                    h434
                }
            };
            nat_le(a435, a436)
        } else {
            false
        } {
            let a438 = 2u64;
            let a439 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h437, _, _, _, _, _) => {
                    h437
                }
            };
            nat_le(a438, a439)
        } else {
            false
        } {
            f9(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f56(v1.clone(), 1u64)?, 2u64, f11(v0, 1u64), f11(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f14(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m440 = v1;
    match m440 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a442 = match v0.clone() {
                Adt2::C0(h441, _, _, _, _, _, _, _, _, _, _) => {
                    h441
                }
            };
            let a443 = 0u64;
            if if if if if if if if if if if if if nat_eq(a442, a443) {
                true
            } else {
                let a445 = match v0.clone() {
                    Adt2::C0(h444, _, _, _, _, _, _, _, _, _, _) => {
                        h444
                    }
                };
                let a446 = 1u64;
                nat_eq(a445, a446)
            } {
                let a448 = match v3.clone() {
                    Adt2::C0(h447, _, _, _, _, _, _, _, _, _, _) => {
                        h447
                    }
                };
                let a449 = 26u64;
                nat_eq(a448, a449)
            } else {
                false
            } {
                let a451 = match v2.clone() {
                    Adt2::C0(h450, _, _, _, _, _, _, _, _, _, _) => {
                        h450
                    }
                };
                let a452 = 0u64;
                nat_eq(a451, a452)
            } else {
                false
            } {
                let a455 = match v0.clone() {
                    Adt2::C0(_, h453, _, _, _, _, _, _, _, _, _) => {
                        h453
                    }
                };
                let a456 = match v2.clone() {
                    Adt2::C0(_, h454, _, _, _, _, _, _, _, _, _) => {
                        h454
                    }
                };
                nat_eq(a455, a456)
            } else {
                false
            } {
                let a459 = match v3.clone() {
                    Adt2::C0(_, h457, _, _, _, _, _, _, _, _, _) => {
                        h457
                    }
                };
                let a460 = match v0.clone() {
                    Adt2::C0(_, _, h458, _, _, _, _, _, _, _, _) => {
                        h458
                    }
                };
                nat_le(a459, a460)
            } else {
                false
            } {
                let a462 = match v0.clone() {
                    Adt2::C0(_, _, _, h461, _, _, _, _, _, _, _) => {
                        h461
                    }
                };
                let a463 = 1u64;
                nat_eq(a462, a463)
            } else {
                false
            } {
                let a465 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h464, _, _, _, _, _, _) => {
                        h464
                    }
                };
                let a466 = 1u64;
                nat_eq(a465, a466)
            } else {
                false
            } {
                let a468 = match v2.clone() {
                    Adt2::C0(_, _, _, h467, _, _, _, _, _, _, _) => {
                        h467
                    }
                };
                let a469 = 1u64;
                nat_eq(a468, a469)
            } else {
                false
            } {
                let a471 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h470, _, _, _, _, _, _) => {
                        h470
                    }
                };
                let a472 = 1u64;
                nat_eq(a471, a472)
            } else {
                false
            } {
                let a474 = match v3.clone() {
                    Adt2::C0(_, _, h473, _, _, _, _, _, _, _, _) => {
                        h473
                    }
                };
                let a475 = 1u64;
                nat_eq(a474, a475)
            } else {
                false
            } {
                let a477 = match v3.clone() {
                    Adt2::C0(_, _, _, h476, _, _, _, _, _, _, _) => {
                        h476
                    }
                };
                let a478 = 1u64;
                nat_eq(a477, a478)
            } else {
                false
            } {
                let a480 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h479, _, _, _, _, _, _) => {
                        h479
                    }
                };
                let a481 = 1u64;
                nat_eq(a480, a481)
            } else {
                false
            } {
                if f57(match v2.clone() {
                    Adt2::C0(_, h482, _, _, _, _, _, _, _, _, _) => {
                        h482
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h483, _, _, _, _, _, _, _, _) => {
                        h483
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h484, _, _, _, _, _, _, _) => {
                        h484
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h485, _, _, _, _, _, _) => {
                        h485
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f11(v0, 1u64), f11(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a487 = match v3.clone() {
                        Adt2::C0(_, h486, _, _, _, _, _, _, _, _, _) => {
                            h486
                        }
                    };
                    let a488 = 0u64;
                    if nat_eq(a487, a488) {
                        Ok(Adt6::C4(f56(v2.clone(), 1u64)?, 4u64, f11(v0, 1u64), f11(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a490 = match v0.clone() {
                            Adt2::C0(_, h489, _, _, _, _, _, _, _, _, _) => {
                                h489
                            }
                        };
                        let a491 = 2147483647u64;
                        if if if if if if nat_le(a490, a491) {
                            let a493 = match v3.clone() {
                                Adt2::C0(_, h492, _, _, _, _, _, _, _, _, _) => {
                                    h492
                                }
                            };
                            let a494 = 2147483647u64;
                            nat_le(a493, a494)
                        } else {
                            false
                        } {
                            let a496 = 4u64;
                            let a497 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h495, _, _, _, _, _) => {
                                    h495
                                }
                            };
                            nat_le(a496, a497)
                        } else {
                            false
                        } {
                            let a503 = {
                                let a500 = match v2.clone() {
                                    Adt2::C0(_, h498, _, _, _, _, _, _, _, _, _) => {
                                        h498
                                    }
                                };
                                let a501 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h499, _, _, _, _, _) => {
                                        h499
                                    }
                                };
                                nat_mul(a500, a501)?
                            };
                            let a504 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h502, _, _, _, _) => {
                                    h502
                                }
                            };
                            nat_le(a503, a504)
                        } else {
                            false
                        } {
                            f9(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f9(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a518 = {
                                    let a508 = {
                                        let a506 = 2u64;
                                        let a507 = match v3.clone() {
                                            Adt2::C0(_, h505, _, _, _, _, _, _, _, _, _) => {
                                                h505
                                            }
                                        };
                                        nat_mul(a506, a507)?
                                    };
                                    let a509 = f56(v2.clone(), 1u64)?;
                                    nat_add(a508, a509)?
                                };
                                let a519 = {
                                    let a516 = match v3.clone() {
                                        Adt2::C0(_, h510, _, _, _, _, _, _, _, _, _) => {
                                            h510
                                        }
                                    };
                                    let a517 = {
                                        let a514 = 1u64;
                                        let a515 = {
                                            let a512 = 2u64;
                                            let a513 = match v0.clone() {
                                                Adt2::C0(_, h511, _, _, _, _, _, _, _, _, _) => {
                                                    h511
                                                }
                                            };
                                            nat_mul(a512, a513)?
                                        };
                                        nat_add(a514, a515)?
                                    };
                                    nat_mul(a516, a517)?
                                };
                                nat_add(a518, a519)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h520, _, _, _, _, _, _, _, _, _) => {
                                    h520
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h521, _, _, _, _, _, _, _, _, _) => {
                                    h521
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h522, _, _, _, _, _) => {
                                    h522
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h523, _, _, _, _) => {
                                    h523
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h524, _, _, _) => {
                                    h524
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h525, _, _) => {
                                    h525
                                }
                            }), f11(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                                    h526
                                }
                            }, f11(v3, 1u64))))
                        } else {
                            Ok(Adt6::C1)
                        }
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f15(v0: u64) -> bool {
    let a527 = v0;
    let a528 = 1u64;
    if if nat_eq(a527, a528) {
        true
    } else {
        let a529 = v0;
        let a530 = 33u64;
        nat_eq(a529, a530)
    } {
        true
    } else {
        let a531 = v0;
        let a532 = 34u64;
        nat_eq(a531, a532)
    }
}

pub fn f16(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a535 = match v1.clone() {
        Adt2::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
            h533
        }
    };
    let a536 = match v3.clone() {
        Adt2::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
            h534
        }
    };
    if nat_eq(a535, a536) {
        f58(v0, v1, v2, v3)
    } else {
        if if if if f59(match v1.clone() {
            Adt2::C0(h537, _, _, _, _, _, _, _, _, _, _) => {
                h537
            }
        }) {
            f59(match v3.clone() {
                Adt2::C0(h538, _, _, _, _, _, _, _, _, _, _) => {
                    h538
                }
            })
        } else {
            false
        } {
            let a539 = v0;
            let a540 = 33u64;
            if nat_eq(a539, a540) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a541 = f56(v1.clone(), 1u64)?;
            let a542 = f56(v3.clone(), 1u64)?;
            nat_eq(a541, a542)
        } else {
            false
        } {
            let m544 = f60(match v3.clone() {
                Adt2::C0(h543, _, _, _, _, _, _, _, _, _, _) => {
                    h543
                }
            });
            match m544 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f56(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h545) => {
                            h545
                        }
                    }, f11(v1.clone(), 1u64), f11(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h546, _, _, _, _, _, _, _, _, _, _) => {
                            h546
                        }
                    }, match v3 {
                        Adt2::C0(h547, _, _, _, _, _, _, _, _, _, _) => {
                            h547
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f17(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a549 = match v0.clone() {
        Adt2::C0(h548, _, _, _, _, _, _, _, _, _, _) => {
            h548
        }
    };
    let a550 = 0u64;
    if if if if nat_eq(a549, a550) {
        true
    } else {
        let a552 = match v0.clone() {
            Adt2::C0(h551, _, _, _, _, _, _, _, _, _, _) => {
                h551
            }
        };
        let a553 = 1u64;
        nat_eq(a552, a553)
    } {
        true
    } else {
        let a555 = match v0.clone() {
            Adt2::C0(h554, _, _, _, _, _, _, _, _, _, _) => {
                h554
            }
        };
        let a556 = 25u64;
        if nat_eq(a555, a556) {
            true
        } else {
            let a558 = match v0.clone() {
                Adt2::C0(h557, _, _, _, _, _, _, _, _, _, _) => {
                    h557
                }
            };
            let a559 = 26u64;
            nat_eq(a558, a559)
        }
    } {
        true
    } else {
        let a561 = match v0.clone() {
            Adt2::C0(h560, _, _, _, _, _, _, _, _, _, _) => {
                h560
            }
        };
        let a562 = 30u64;
        nat_eq(a561, a562)
    } {
        let a565 = match v0.clone() {
            Adt2::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
                h563
            }
        };
        let a566 = match v1.clone() {
            Adt2::C0(h564, _, _, _, _, _, _, _, _, _, _) => {
                h564
            }
        };
        if if nat_eq(a565, a566) {
            f61(match v1.clone() {
                Adt2::C0(_, h567, _, _, _, _, _, _, _, _, _) => {
                    h567
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h568, _, _, _, _, _, _, _, _) => {
                    h568
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h569, _, _, _, _, _, _, _) => {
                    h569
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h570, _, _, _, _, _, _) => {
                    h570
                }
            }, match v0.clone() {
                Adt2::C0(_, h571, _, _, _, _, _, _, _, _, _) => {
                    h571
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h572, _, _, _, _, _, _, _, _) => {
                    h572
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h573, _, _, _, _, _, _, _) => {
                    h573
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h574, _, _, _, _, _, _) => {
                    h574
                }
            })
        } else {
            false
        } {
            let m576 = f60(match v0.clone() {
                Adt2::C0(h575, _, _, _, _, _, _, _, _, _, _) => {
                    h575
                }
            });
            match m576 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f56(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h577) => {
                            h577
                        }
                    }, f11(v0, 1u64), f11(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f18(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m578 = v2;
    match m578 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a579 = v0;
            let a580 = 4u64;
            if if if nat_lt(a579, a580) {
                let a583 = match v1.clone() {
                    Adt2::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                        h581
                    }
                };
                let a584 = match v4.clone() {
                    Adt2::C0(h582, _, _, _, _, _, _, _, _, _, _) => {
                        h582
                    }
                };
                if nat_eq(a583, a584) {
                    let a587 = match v1.clone() {
                        Adt2::C0(h585, _, _, _, _, _, _, _, _, _, _) => {
                            h585
                        }
                    };
                    let a588 = match v3.clone() {
                        Adt2::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                            h586
                        }
                    };
                    nat_eq(a587, a588)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f62(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
                        h589
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h590, _, _, _, _, _, _, _, _, _) => {
                        h590
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h591, _, _, _, _, _, _, _, _, _) => {
                        h591
                    }
                })? {
                    f62(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h592, _, _, _, _, _, _, _, _) => {
                            h592
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h593, _, _, _, _, _, _, _, _) => {
                            h593
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h594, _, _, _, _, _, _, _, _) => {
                            h594
                        }
                    })?
                } else {
                    false
                } {
                    f62(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h595, _, _, _, _, _, _, _) => {
                            h595
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h596, _, _, _, _, _, _, _) => {
                            h596
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h597, _, _, _, _, _, _, _) => {
                            h597
                        }
                    })?
                } else {
                    false
                } {
                    f62(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h598, _, _, _, _, _, _) => {
                            h598
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h599, _, _, _, _, _, _) => {
                            h599
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h600, _, _, _, _, _, _) => {
                            h600
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m602 = f60(match v1.clone() {
                    Adt2::C0(h601, _, _, _, _, _, _, _, _, _, _) => {
                        h601
                    }
                });
                match m602 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h603, _) => {
                                h603
                            }
                        };
                        let a608 = {
                            let a605 = match v1.clone() {
                                Adt2::C0(_, h604, _, _, _, _, _, _, _, _, _) => {
                                    h604
                                }
                            };
                            let a606 = v6;
                            let a607 = 1u64;
                            nat_rem(a605, a606, a607)
                        };
                        let a609 = 0u64;
                        if if if nat_eq(a608, a609) {
                            let a614 = {
                                let a611 = match v4.clone() {
                                    Adt2::C0(_, h610, _, _, _, _, _, _, _, _, _) => {
                                        h610
                                    }
                                };
                                let a612 = v6;
                                let a613 = 1u64;
                                nat_rem(a611, a612, a613)
                            };
                            let a615 = 0u64;
                            nat_eq(a614, a615)
                        } else {
                            false
                        } {
                            let a620 = {
                                let a617 = match v3.clone() {
                                    Adt2::C0(_, h616, _, _, _, _, _, _, _, _, _) => {
                                        h616
                                    }
                                };
                                let a618 = v6;
                                let a619 = 1u64;
                                nat_rem(a617, a618, a619)
                            };
                            let a621 = 0u64;
                            nat_eq(a620, a621)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f56(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h622) => {
                                    h622
                                }
                            }, f11(v1, v6), f11(v3, v6), Adt4::C2(v0, f11(v4, v6))))
                        } else {
                            Ok(Adt6::C1)
                        }
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f19(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a624 = match v2.clone() {
        Adt2::C0(h623, _, _, _, _, _, _, _, _, _, _) => {
            h623
        }
    };
    let a625 = 0u64;
    if if nat_eq(a624, a625) {
        let a627 = match v3.clone() {
            Adt2::C0(h626, _, _, _, _, _, _, _, _, _, _) => {
                h626
            }
        };
        let a628 = 0u64;
        nat_eq(a627, a628)
    } else {
        false
    } {
        let a629 = v0;
        let a630 = 61u64;
        if nat_eq(a629, a630) {
            let v4: bool = {
                let a632 = 0u64;
                let a633 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h631, _, _, _, _, _, _, _) => {
                        h631
                    }
                };
                nat_lt(a632, a633)
            };
            if if if if if if f63(match v2.clone() {
                Adt2::C0(_, h634, _, _, _, _, _, _, _, _, _) => {
                    h634
                }
            }, match v1.clone() {
                Adt1::C0(h635, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h635
                }
            }, match v1.clone() {
                Adt1::C0(_, h636, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h636
                }
            }, match v3.clone() {
                Adt2::C0(_, h637, _, _, _, _, _, _, _, _, _) => {
                    h637
                }
            })? {
                f63(match v2.clone() {
                    Adt2::C0(_, _, h638, _, _, _, _, _, _, _, _) => {
                        h638
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h639, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h639
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h640, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h640
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h641, _, _, _, _, _, _, _, _) => {
                        h641
                    }
                })?
            } else {
                false
            } {
                f63(match v2.clone() {
                    Adt2::C0(_, _, _, h642, _, _, _, _, _, _, _) => {
                        h642
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h643, _, _, _, _, _, _, _, _, _, _, _) => {
                        h643
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h644, _, _, _, _, _, _, _, _, _, _) => {
                        h644
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h645, _, _, _, _, _, _, _) => {
                        h645
                    }
                })?
            } else {
                false
            } {
                f63(match v2.clone() {
                    Adt2::C0(_, _, _, _, h646, _, _, _, _, _, _) => {
                        h646
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h647, _, _, _, _, _, _, _, _, _) => {
                        h647
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h648, _, _, _, _, _, _, _, _) => {
                        h648
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h649, _, _, _, _, _, _) => {
                        h649
                    }
                })?
            } else {
                false
            } {
                let a651 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h650, _, _, _, _, _, _, _) => {
                        h650
                    }
                };
                let a652 = 4294967295u64;
                nat_le(a651, a652)
            } else {
                false
            } {
                if v4 {
                    !f57(match v2.clone() {
                        Adt2::C0(_, h653, _, _, _, _, _, _, _, _, _) => {
                            h653
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h654, _, _, _, _, _, _, _, _) => {
                            h654
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h655, _, _, _, _, _, _, _) => {
                            h655
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h656, _, _, _, _, _, _) => {
                            h656
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f56(v3.clone(), 1u64)?, 4u64, f11(v2, 1u64), f11(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h657, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h657
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h658, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h658
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h659, _, _, _, _, _, _, _, _, _, _, _) => {
                        h659
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h660, _, _, _, _, _, _, _, _, _) => {
                        h660
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a661 = v0;
            let a662 = 62u64;
            if nat_eq(a661, a662) {
                let a664 = match v1.clone() {
                    Adt1::C0(h663, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h663
                    }
                };
                let a665 = 2147483648u64;
                if if if if if nat_lt(a664, a665) {
                    let a667 = match v1.clone() {
                        Adt1::C0(_, h666, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h666
                        }
                    };
                    let a668 = 2147483648u64;
                    nat_lt(a667, a668)
                } else {
                    false
                } {
                    let a671 = match v1.clone() {
                        Adt1::C0(h669, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h669
                        }
                    };
                    let a672 = match v2.clone() {
                        Adt2::C0(_, h670, _, _, _, _, _, _, _, _, _) => {
                            h670
                        }
                    };
                    nat_lt(a671, a672)
                } else {
                    false
                } {
                    let a675 = match v1.clone() {
                        Adt1::C0(_, h673, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h673
                        }
                    };
                    let a676 = match v2.clone() {
                        Adt2::C0(_, h674, _, _, _, _, _, _, _, _, _) => {
                            h674
                        }
                    };
                    nat_lt(a675, a676)
                } else {
                    false
                } {
                    let a679 = match v2.clone() {
                        Adt2::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                            h677
                        }
                    };
                    let a680 = match v3.clone() {
                        Adt2::C0(_, _, h678, _, _, _, _, _, _, _, _) => {
                            h678
                        }
                    };
                    if if if nat_eq(a679, a680) {
                        let a683 = match v2.clone() {
                            Adt2::C0(_, _, _, h681, _, _, _, _, _, _, _) => {
                                h681
                            }
                        };
                        let a684 = match v3.clone() {
                            Adt2::C0(_, _, _, h682, _, _, _, _, _, _, _) => {
                                h682
                            }
                        };
                        nat_eq(a683, a684)
                    } else {
                        false
                    } {
                        let a687 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h685, _, _, _, _, _, _) => {
                                h685
                            }
                        };
                        let a688 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h686, _, _, _, _, _, _) => {
                                h686
                            }
                        };
                        nat_eq(a687, a688)
                    } else {
                        false
                    } {
                        let a697 = match v3.clone() {
                            Adt2::C0(_, h689, _, _, _, _, _, _, _, _, _) => {
                                h689
                            }
                        };
                        let a698 = {
                            let a695 = {
                                let a692 = match v2.clone() {
                                    Adt2::C0(_, h690, _, _, _, _, _, _, _, _, _) => {
                                        h690
                                    }
                                };
                                let a693 = match v1.clone() {
                                    Adt1::C0(h691, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h691
                                    }
                                };
                                nat_add(a692, a693)?
                            };
                            let a696 = match v1.clone() {
                                Adt1::C0(_, h694, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h694
                                }
                            };
                            nat_add(a695, a696)?
                        };
                        nat_eq(a697, a698)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f56(v3.clone(), 1u64)?, 4u64, f11(v2, 1u64), f11(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h699, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h699
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a700 = v0;
                let a701 = 63u64;
                if nat_eq(a700, a701) {
                    if if if if if f64(match v2.clone() {
                        Adt2::C0(_, h702, _, _, _, _, _, _, _, _, _) => {
                            h702
                        }
                    }, match v1.clone() {
                        Adt1::C0(h703, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h703
                        }
                    })? {
                        f64(match v2.clone() {
                            Adt2::C0(_, _, h704, _, _, _, _, _, _, _, _) => {
                                h704
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h705, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h705
                            }
                        })?
                    } else {
                        false
                    } {
                        f64(match v2.clone() {
                            Adt2::C0(_, _, _, h706, _, _, _, _, _, _, _) => {
                                h706
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h707, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h707
                            }
                        })?
                    } else {
                        false
                    } {
                        f64(match v2.clone() {
                            Adt2::C0(_, _, _, _, h708, _, _, _, _, _, _) => {
                                h708
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h709, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h709
                            }
                        })?
                    } else {
                        false
                    } {
                        let a712 = match v2.clone() {
                            Adt2::C0(_, h710, _, _, _, _, _, _, _, _, _) => {
                                h710
                            }
                        };
                        let a713 = match v3.clone() {
                            Adt2::C0(_, h711, _, _, _, _, _, _, _, _, _) => {
                                h711
                            }
                        };
                        if if if nat_eq(a712, a713) {
                            let a716 = match v2.clone() {
                                Adt2::C0(_, _, h714, _, _, _, _, _, _, _, _) => {
                                    h714
                                }
                            };
                            let a717 = match v3.clone() {
                                Adt2::C0(_, _, h715, _, _, _, _, _, _, _, _) => {
                                    h715
                                }
                            };
                            nat_eq(a716, a717)
                        } else {
                            false
                        } {
                            let a720 = match v2.clone() {
                                Adt2::C0(_, _, _, h718, _, _, _, _, _, _, _) => {
                                    h718
                                }
                            };
                            let a721 = match v3.clone() {
                                Adt2::C0(_, _, _, h719, _, _, _, _, _, _, _) => {
                                    h719
                                }
                            };
                            nat_eq(a720, a721)
                        } else {
                            false
                        } {
                            let a724 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h722, _, _, _, _, _, _) => {
                                    h722
                                }
                            };
                            let a725 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h723, _, _, _, _, _, _) => {
                                    h723
                                }
                            };
                            nat_eq(a724, a725)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f56(v3.clone(), 1u64)?, 4u64, f11(v2, 1u64), f11(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h726, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h726
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h727, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h727
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h728, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h728
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h729, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h729
                            }
                        })))
                    } else {
                        Ok(Adt6::C1)
                    }
                } else {
                    Ok(Adt6::C0)
                }
            }
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f20(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m730 = v3;
    match m730 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a731 = v0;
                let a732 = 4u64;
                nat_eq(a731, a732)
            };
            let v7: bool = {
                let a734 = 0u64;
                let a735 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h733, _, _, _, _, _, _, _, _, _, _, _) => {
                        h733
                    }
                };
                nat_lt(a734, a735)
            };
            let v8: u64 = f56(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h736, _, _, _, _, _, _, _, _, _) => {
                    h736
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h737, _, _, _, _, _, _, _, _) => {
                    h737
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h738, _, _, _, _, _, _, _) => {
                    h738
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h739, _, _, _, _, _, _) => {
                    h739
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h740, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h740
                }
            }, match v1.clone() {
                Adt1::C0(_, h741, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h741
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h742, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h742
                }
            });
            if if if if if if if if if if if if v6 {
                let a744 = match v2.clone() {
                    Adt2::C0(h743, _, _, _, _, _, _, _, _, _, _) => {
                        h743
                    }
                };
                let a745 = 0u64;
                if nat_eq(a744, a745) {
                    let a747 = match v5.clone() {
                        Adt2::C0(h746, _, _, _, _, _, _, _, _, _, _) => {
                            h746
                        }
                    };
                    let a748 = 0u64;
                    nat_eq(a747, a748)
                } else {
                    false
                }
            } else {
                let a750 = match v2.clone() {
                    Adt2::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
                        h749
                    }
                };
                let a751 = 0u64;
                if if nat_eq(a750, a751) {
                    true
                } else {
                    let a753 = match v2.clone() {
                        Adt2::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
                            h752
                        }
                    };
                    let a754 = 26u64;
                    nat_eq(a753, a754)
                } {
                    let a756 = match v5.clone() {
                        Adt2::C0(h755, _, _, _, _, _, _, _, _, _, _) => {
                            h755
                        }
                    };
                    let a757 = 0u64;
                    if nat_eq(a756, a757) {
                        true
                    } else {
                        let a759 = match v5.clone() {
                            Adt2::C0(h758, _, _, _, _, _, _, _, _, _, _) => {
                                h758
                            }
                        };
                        let a760 = 26u64;
                        nat_eq(a759, a760)
                    }
                } else {
                    false
                }
            } {
                let a763 = match v2.clone() {
                    Adt2::C0(h761, _, _, _, _, _, _, _, _, _, _) => {
                        h761
                    }
                };
                let a764 = match v4.clone() {
                    Adt2::C0(h762, _, _, _, _, _, _, _, _, _, _) => {
                        h762
                    }
                };
                nat_eq(a763, a764)
            } else {
                false
            } {
                f8(v2.clone(), v4.clone())
            } else {
                false
            } {
                f65(v2.clone())?
            } else {
                false
            } {
                f65(v4.clone())?
            } else {
                false
            } {
                let a766 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h765, _, _, _, _, _) => {
                        h765
                    }
                };
                let a767 = 4u64;
                nat_eq(a766, a767)
            } else {
                false
            } {
                let a768 = v8;
                let a769 = f56(v2.clone(), 1u64)?;
                nat_le(a768, a769)
            } else {
                false
            } {
                let a771 = match v1.clone() {
                    Adt1::C0(_, _, _, h770, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h770
                    }
                };
                let a772 = 1073741824u64;
                nat_lt(a771, a772)
            } else {
                false
            } {
                let a774 = match v1.clone() {
                    Adt1::C0(h773, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h773
                    }
                };
                let a775 = 4294967295u64;
                if if if if nat_le(a774, a775) {
                    let a777 = match v1.clone() {
                        Adt1::C0(_, h776, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h776
                        }
                    };
                    let a778 = 4294967295u64;
                    nat_le(a777, a778)
                } else {
                    false
                } {
                    let a780 = match v1.clone() {
                        Adt1::C0(_, _, h779, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h779
                        }
                    };
                    let a781 = 4294967295u64;
                    nat_le(a780, a781)
                } else {
                    false
                } {
                    let a783 = match v1.clone() {
                        Adt1::C0(_, _, _, h782, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h782
                        }
                    };
                    let a784 = 4294967295u64;
                    nat_le(a783, a784)
                } else {
                    false
                } {
                    let a786 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h785, _, _, _, _, _, _, _, _, _, _, _) => {
                            h785
                        }
                    };
                    let a787 = 4294967295u64;
                    nat_le(a786, a787)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a790 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h788) => {
                            h788
                        }
                    };
                    let a791 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h789) => {
                            h789
                        }
                    };
                    nat_eq(a790, a791)
                } else {
                    true
                }
            } else {
                false
            } {
                f66(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h792, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h792
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h793, _) => {
                        h793
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f56(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a794 = v10;
                    let a795 = v8;
                    nat_add(a794, a795)?
                }, 4u64, f11(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h796, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h796
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f21(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a798 = match v0.clone() {
        Adt2::C0(h797, _, _, _, _, _, _, _, _, _, _) => {
            h797
        }
    };
    let a799 = 0u64;
    if if if nat_eq(a798, a799) {
        let a801 = match v1.clone() {
            Adt2::C0(h800, _, _, _, _, _, _, _, _, _, _) => {
                h800
            }
        };
        let a802 = 0u64;
        nat_eq(a801, a802)
    } else {
        false
    } {
        f61(match v0.clone() {
            Adt2::C0(_, h803, _, _, _, _, _, _, _, _, _) => {
                h803
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h804, _, _, _, _, _, _, _, _) => {
                h804
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h805, _, _, _, _, _, _, _) => {
                h805
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h806, _, _, _, _, _, _) => {
                h806
            }
        }, match v1.clone() {
            Adt2::C0(_, h807, _, _, _, _, _, _, _, _, _) => {
                h807
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h808, _, _, _, _, _, _, _, _) => {
                h808
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h809, _, _, _, _, _, _, _) => {
                h809
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h810, _, _, _, _, _, _) => {
                h810
            }
        })
    } else {
        false
    } {
        let v2: u64 = f56(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a811 = v2;
            let a812 = f56(v0.clone(), 1u64)?;
            nat_add(a811, a812)?
        }, 4u64, f11(v0, 1u64), f11(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f22(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a814 = match v1.clone() {
        Adt2::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
            h813
        }
    };
    let a815 = 0u64;
    if if if nat_eq(a814, a815) {
        true
    } else {
        let a817 = match v1.clone() {
            Adt2::C0(h816, _, _, _, _, _, _, _, _, _, _) => {
                h816
            }
        };
        let a818 = 1u64;
        nat_eq(a817, a818)
    } {
        true
    } else {
        let a820 = match v1.clone() {
            Adt2::C0(h819, _, _, _, _, _, _, _, _, _, _) => {
                h819
            }
        };
        let a821 = 30u64;
        nat_eq(a820, a821)
    } {
        let a824 = match v1.clone() {
            Adt2::C0(h822, _, _, _, _, _, _, _, _, _, _) => {
                h822
            }
        };
        let a825 = match v2.clone() {
            Adt2::C0(h823, _, _, _, _, _, _, _, _, _, _) => {
                h823
            }
        };
        if if if if if nat_eq(a824, a825) {
            let a828 = match v1.clone() {
                Adt2::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                    h826
                }
            };
            let a829 = match v2.clone() {
                Adt2::C0(_, h827, _, _, _, _, _, _, _, _, _) => {
                    h827
                }
            };
            nat_eq(a828, a829)
        } else {
            false
        } {
            let a832 = match v1.clone() {
                Adt2::C0(_, _, _, h830, _, _, _, _, _, _, _) => {
                    h830
                }
            };
            let a833 = match v2.clone() {
                Adt2::C0(_, _, _, h831, _, _, _, _, _, _, _) => {
                    h831
                }
            };
            nat_eq(a832, a833)
        } else {
            false
        } {
            let a836 = match v1.clone() {
                Adt2::C0(_, _, _, _, h834, _, _, _, _, _, _) => {
                    h834
                }
            };
            let a837 = match v2.clone() {
                Adt2::C0(_, _, _, _, h835, _, _, _, _, _, _) => {
                    h835
                }
            };
            nat_eq(a836, a837)
        } else {
            false
        } {
            let a838 = v0;
            let a839 = 4u64;
            if nat_eq(a838, a839) {
                let a841 = match v1.clone() {
                    Adt2::C0(_, _, h840, _, _, _, _, _, _, _, _) => {
                        h840
                    }
                };
                let a842 = 1u64;
                if nat_eq(a841, a842) {
                    let a845 = match v1.clone() {
                        Adt2::C0(_, h843, _, _, _, _, _, _, _, _, _) => {
                            h843
                        }
                    };
                    let a846 = match v2.clone() {
                        Adt2::C0(_, _, h844, _, _, _, _, _, _, _, _) => {
                            h844
                        }
                    };
                    nat_eq(a845, a846)
                } else {
                    false
                }
            } else {
                let a847 = v0;
                let a848 = 4u64;
                if if nat_lt(a847, a848) {
                    let a851 = match v1.clone() {
                        Adt2::C0(_, h849, _, _, _, _, _, _, _, _, _) => {
                            h849
                        }
                    };
                    let a852 = match v1.clone() {
                        Adt2::C0(_, _, h850, _, _, _, _, _, _, _, _) => {
                            h850
                        }
                    };
                    nat_eq(a851, a852)
                } else {
                    false
                } {
                    let a855 = match v1.clone() {
                        Adt2::C0(_, _, h853, _, _, _, _, _, _, _, _) => {
                            h853
                        }
                    };
                    let a856 = match v2.clone() {
                        Adt2::C0(_, _, h854, _, _, _, _, _, _, _, _) => {
                            h854
                        }
                    };
                    nat_eq(a855, a856)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m858 = f60(match v1.clone() {
                Adt2::C0(h857, _, _, _, _, _, _, _, _, _, _) => {
                    h857
                }
            });
            match m858 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f56(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h859) => {
                            h859
                        }
                    }, f11(v1, 1u64), f11(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f23(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f67(match v1.clone() {
        Adt2::C0(h860, _, _, _, _, _, _, _, _, _, _) => {
            h860
        }
    }, v0) {
        let a863 = match v1.clone() {
            Adt2::C0(h861, _, _, _, _, _, _, _, _, _, _) => {
                h861
            }
        };
        let a864 = match v2.clone() {
            Adt2::C0(h862, _, _, _, _, _, _, _, _, _, _) => {
                h862
            }
        };
        nat_eq(a863, a864)
    } else {
        false
    } {
        let a867 = match v1.clone() {
            Adt2::C0(_, h865, _, _, _, _, _, _, _, _, _) => {
                h865
            }
        };
        let a868 = match v2.clone() {
            Adt2::C0(_, h866, _, _, _, _, _, _, _, _, _) => {
                h866
            }
        };
        if if if nat_eq(a867, a868) {
            let a871 = match v1.clone() {
                Adt2::C0(_, _, h869, _, _, _, _, _, _, _, _) => {
                    h869
                }
            };
            let a872 = match v2.clone() {
                Adt2::C0(_, _, h870, _, _, _, _, _, _, _, _) => {
                    h870
                }
            };
            nat_eq(a871, a872)
        } else {
            false
        } {
            let a875 = match v1.clone() {
                Adt2::C0(_, _, _, h873, _, _, _, _, _, _, _) => {
                    h873
                }
            };
            let a876 = match v2.clone() {
                Adt2::C0(_, _, _, h874, _, _, _, _, _, _, _) => {
                    h874
                }
            };
            nat_eq(a875, a876)
        } else {
            false
        } {
            let a879 = match v1.clone() {
                Adt2::C0(_, _, _, _, h877, _, _, _, _, _, _) => {
                    h877
                }
            };
            let a880 = match v2.clone() {
                Adt2::C0(_, _, _, _, h878, _, _, _, _, _, _) => {
                    h878
                }
            };
            nat_eq(a879, a880)
        } else {
            false
        }
    } else {
        false
    } {
        let m882 = f60(match v1.clone() {
            Adt2::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
                h881
            }
        });
        match m882 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f56(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h883) => {
                        h883
                    }
                }, f11(v1.clone(), 1u64), f11(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
                        h884
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f24(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h885, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h885
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h886, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h886
            }
        }
    };
    let a887 = 0u64;
    let a888 = v4;
    if if nat_lt(a887, a888) {
        let a889 = v4;
        let a890 = 2147483648u64;
        nat_lt(a889, a890)
    } else {
        false
    } {
        let v5: Adt2 = if v0 {
            v3.clone()
        } else {
            v2.clone()
        };
        let v6: Adt2 = if v0 {
            v2.clone()
        } else {
            v3.clone()
        };
        let v7: u64 = f68(match v5.clone() {
            Adt2::C0(_, _, h891, _, _, _, _, _, _, _, _) => {
                h891
            }
        }, v4)?;
        let v8: u64 = f68(match v5.clone() {
            Adt2::C0(_, _, _, h892, _, _, _, _, _, _, _) => {
                h892
            }
        }, v4)?;
        let a894 = match v5.clone() {
            Adt2::C0(h893, _, _, _, _, _, _, _, _, _, _) => {
                h893
            }
        };
        let a895 = 0u64;
        if if if if if nat_eq(a894, a895) {
            let a897 = match v6.clone() {
                Adt2::C0(h896, _, _, _, _, _, _, _, _, _, _) => {
                    h896
                }
            };
            let a898 = 0u64;
            nat_eq(a897, a898)
        } else {
            false
        } {
            let a900 = match v5.clone() {
                Adt2::C0(_, _, _, _, h899, _, _, _, _, _, _) => {
                    h899
                }
            };
            let a901 = 1u64;
            nat_eq(a900, a901)
        } else {
            false
        } {
            let a904 = match v5.clone() {
                Adt2::C0(_, h902, _, _, _, _, _, _, _, _, _) => {
                    h902
                }
            };
            let a905 = match v6.clone() {
                Adt2::C0(_, h903, _, _, _, _, _, _, _, _, _) => {
                    h903
                }
            };
            nat_eq(a904, a905)
        } else {
            false
        } {
            if v0 {
                let a907 = match v5.clone() {
                    Adt2::C0(_, _, h906, _, _, _, _, _, _, _, _) => {
                        h906
                    }
                };
                let a908 = 2147483648u64;
                if if if nat_lt(a907, a908) {
                    let a910 = match v5.clone() {
                        Adt2::C0(_, _, _, h909, _, _, _, _, _, _, _) => {
                            h909
                        }
                    };
                    let a911 = 2147483648u64;
                    nat_lt(a910, a911)
                } else {
                    false
                } {
                    let a914 = {
                        let a912 = v7;
                        let a913 = v8;
                        nat_mul(a912, a913)?
                    };
                    let a915 = 2147483648u64;
                    nat_le(a914, a915)
                } else {
                    false
                } {
                    f69(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a917 = match v1.clone() {
                    Adt1::C0(h916, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h916
                    }
                };
                let a918 = v7;
                if if if if nat_eq(a917, a918) {
                    let a920 = match v1 {
                        Adt1::C0(_, h919, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h919
                        }
                    };
                    let a921 = v8;
                    nat_eq(a920, a921)
                } else {
                    false
                } {
                    f70(v7, v8, v4)?
                } else {
                    false
                } {
                    let a923 = match v6.clone() {
                        Adt2::C0(_, _, h922, _, _, _, _, _, _, _, _) => {
                            h922
                        }
                    };
                    let a924 = v4;
                    if nat_eq(a923, a924) {
                        let a926 = match v6.clone() {
                            Adt2::C0(_, _, _, h925, _, _, _, _, _, _, _) => {
                                h925
                            }
                        };
                        let a927 = v4;
                        nat_eq(a926, a927)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a931 = match v6 {
                        Adt2::C0(_, _, _, _, h928, _, _, _, _, _, _) => {
                            h928
                        }
                    };
                    let a932 = {
                        let a929 = v7;
                        let a930 = v8;
                        nat_mul(a929, a930)?
                    };
                    nat_eq(a931, a932)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f56(v3.clone(), 1u64)?, 4u64, f11(v2, 1u64), f11(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f25(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a934 = match v2.clone() {
        Adt2::C0(h933, _, _, _, _, _, _, _, _, _, _) => {
            h933
        }
    };
    let a935 = 0u64;
    if if if if if nat_eq(a934, a935) {
        true
    } else {
        let a937 = match v2.clone() {
            Adt2::C0(h936, _, _, _, _, _, _, _, _, _, _) => {
                h936
            }
        };
        let a938 = 1u64;
        nat_eq(a937, a938)
    } {
        true
    } else {
        let a940 = match v2.clone() {
            Adt2::C0(h939, _, _, _, _, _, _, _, _, _, _) => {
                h939
            }
        };
        let a941 = 30u64;
        nat_eq(a940, a941)
    } {
        let a944 = match v2.clone() {
            Adt2::C0(h942, _, _, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        let a945 = match v3.clone() {
            Adt2::C0(h943, _, _, _, _, _, _, _, _, _, _) => {
                h943
            }
        };
        nat_eq(a944, a945)
    } else {
        false
    } {
        if if f8(v2.clone(), v3.clone()) {
            let a947 = match v1.clone() {
                Adt1::C0(h946, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h946
                }
            };
            let a948 = 4294967296u64;
            nat_lt(a947, a948)
        } else {
            false
        } {
            let a950 = match v1.clone() {
                Adt1::C0(_, h949, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h949
                }
            };
            let a951 = 4294967296u64;
            nat_lt(a950, a951)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f56(v3.clone(), 1u64)?, {
            let a953 = match v2.clone() {
                Adt2::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
                    h952
                }
            };
            let a954 = 0u64;
            if nat_eq(a953, a954) {
                4u64
            } else {
                2u64
            }
        }, f11(v2.clone(), 1u64), f11(v3, 1u64), Adt4::C13({
            let a955 = v0;
            let a956 = 31u64;
            if nat_eq(a955, a956) {
                0u64
            } else {
                let a957 = v0;
                let a958 = 47u64;
                if nat_eq(a957, a958) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h959, _, _, _, _, _, _, _, _, _, _) => {
                h959
            }
        }, match v1.clone() {
            Adt1::C0(h960, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h960
            }
        }, match v1 {
            Adt1::C0(_, h961, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h961
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a963 = match v0.clone() {
        Adt2::C0(h962, _, _, _, _, _, _, _, _, _, _) => {
            h962
        }
    };
    let a964 = 0u64;
    if if if nat_eq(a963, a964) {
        let a966 = match v1.clone() {
            Adt2::C0(h965, _, _, _, _, _, _, _, _, _, _) => {
                h965
            }
        };
        let a967 = 0u64;
        nat_eq(a966, a967)
    } else {
        false
    } {
        f8(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f56(v1.clone(), 1u64)?, 4u64, f11(v0, 1u64), f11(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a969 = match v0.clone() {
        Adt2::C0(h968, _, _, _, _, _, _, _, _, _, _) => {
            h968
        }
    };
    let a970 = 0u64;
    if if nat_eq(a969, a970) {
        let a972 = match v1.clone() {
            Adt2::C0(h971, _, _, _, _, _, _, _, _, _, _) => {
                h971
            }
        };
        let a973 = 26u64;
        nat_eq(a972, a973)
    } else {
        false
    } {
        let a975 = match v0.clone() {
            Adt2::C0(_, _, _, h974, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a976 = 1u64;
        if if if if if if if nat_eq(a975, a976) {
            let a978 = match v0.clone() {
                Adt2::C0(_, _, _, _, h977, _, _, _, _, _, _) => {
                    h977
                }
            };
            let a979 = 1u64;
            nat_eq(a978, a979)
        } else {
            false
        } {
            let a981 = match v0.clone() {
                Adt2::C0(_, h980, _, _, _, _, _, _, _, _, _) => {
                    h980
                }
            };
            let a982 = 2147483647u64;
            nat_le(a981, a982)
        } else {
            false
        } {
            let a985 = match v1.clone() {
                Adt2::C0(_, h983, _, _, _, _, _, _, _, _, _) => {
                    h983
                }
            };
            let a986 = match v0.clone() {
                Adt2::C0(_, _, h984, _, _, _, _, _, _, _, _) => {
                    h984
                }
            };
            nat_eq(a985, a986)
        } else {
            false
        } {
            let a988 = match v1.clone() {
                Adt2::C0(_, _, h987, _, _, _, _, _, _, _, _) => {
                    h987
                }
            };
            let a989 = 1u64;
            nat_eq(a988, a989)
        } else {
            false
        } {
            let a991 = match v1.clone() {
                Adt2::C0(_, _, _, h990, _, _, _, _, _, _, _) => {
                    h990
                }
            };
            let a992 = 1u64;
            nat_eq(a991, a992)
        } else {
            false
        } {
            let a994 = match v1.clone() {
                Adt2::C0(_, _, _, _, h993, _, _, _, _, _, _) => {
                    h993
                }
            };
            let a995 = 1u64;
            nat_eq(a994, a995)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a998 = match v0.clone() {
                    Adt2::C0(_, _, h996, _, _, _, _, _, _, _, _) => {
                        h996
                    }
                };
                let a999 = f47(match v0.clone() {
                    Adt2::C0(_, h997, _, _, _, _, _, _, _, _, _) => {
                        h997
                    }
                })?;
                nat_mul(a998, a999)?
            }, 4u64, f11(v0, 1u64), f11(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f28(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1000 = v1;
    match m1000 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f49(f11(v0.clone(), 1u64))?;
            let a1002 = match v0.clone() {
                Adt2::C0(h1001, _, _, _, _, _, _, _, _, _, _) => {
                    h1001
                }
            };
            let a1003 = 26u64;
            if if if nat_eq(a1002, a1003) {
                let a1005 = match v3.clone() {
                    Adt2::C0(h1004, _, _, _, _, _, _, _, _, _, _) => {
                        h1004
                    }
                };
                let a1006 = 26u64;
                nat_eq(a1005, a1006)
            } else {
                false
            } {
                let a1008 = match v2.clone() {
                    Adt2::C0(h1007, _, _, _, _, _, _, _, _, _, _) => {
                        h1007
                    }
                };
                let a1009 = 27u64;
                nat_eq(a1008, a1009)
            } else {
                false
            } {
                if if if if if if f8(v0.clone(), v3.clone()) {
                    let a1011 = match v2.clone() {
                        Adt2::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                            h1010
                        }
                    };
                    let a1012 = 1u64;
                    nat_eq(a1011, a1012)
                } else {
                    false
                } {
                    let a1014 = match v2.clone() {
                        Adt2::C0(_, _, h1013, _, _, _, _, _, _, _, _) => {
                            h1013
                        }
                    };
                    let a1015 = 1u64;
                    nat_eq(a1014, a1015)
                } else {
                    false
                } {
                    let a1017 = match v2.clone() {
                        Adt2::C0(_, _, _, h1016, _, _, _, _, _, _, _) => {
                            h1016
                        }
                    };
                    let a1018 = 1u64;
                    nat_eq(a1017, a1018)
                } else {
                    false
                } {
                    let a1020 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1019, _, _, _, _, _, _) => {
                            h1019
                        }
                    };
                    let a1021 = 1u64;
                    nat_eq(a1020, a1021)
                } else {
                    false
                } {
                    let a1022 = v4;
                    let a1023 = 9223372036854775808u64;
                    nat_lt(a1022, a1023)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1026 = 2u64;
                        let a1027 = {
                            let a1024 = v4;
                            let a1025 = 2u64;
                            nat_mul(a1024, a1025)?
                        };
                        nat_add(a1026, a1027)?
                    }, 8u64, f11(v0, 1u64), f11(v2, 1u64), Adt4::C18(v4, f11(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f29(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1028, _, _, _, _, _, _, _, _, _, _) => {
            h1028
        }
    };
    let a1029 = v3;
    let a1030 = 0u64;
    if if nat_eq(a1029, a1030) {
        true
    } else {
        let a1031 = v0;
        let a1032 = 13u64;
        if nat_eq(a1031, a1032) {
            let a1033 = v3;
            let a1034 = 1u64;
            if nat_eq(a1033, a1034) {
                true
            } else {
                let a1035 = v3;
                let a1036 = 30u64;
                nat_eq(a1035, a1036)
            }
        } else {
            false
        }
    } {
        let a1038 = v3;
        let a1039 = match v2.clone() {
            Adt2::C0(h1037, _, _, _, _, _, _, _, _, _, _) => {
                h1037
            }
        };
        if if nat_eq(a1038, a1039) {
            let a1040 = v0;
            let a1041 = 13u64;
            if nat_eq(a1040, a1041) {
                let a1043 = match v2.clone() {
                    Adt2::C0(_, h1042, _, _, _, _, _, _, _, _, _) => {
                        h1042
                    }
                };
                let a1044 = 1u64;
                if if if nat_eq(a1043, a1044) {
                    let a1046 = match v2.clone() {
                        Adt2::C0(_, _, h1045, _, _, _, _, _, _, _, _) => {
                            h1045
                        }
                    };
                    let a1047 = 1u64;
                    nat_eq(a1046, a1047)
                } else {
                    false
                } {
                    let a1049 = match v2.clone() {
                        Adt2::C0(_, _, _, h1048, _, _, _, _, _, _, _) => {
                            h1048
                        }
                    };
                    let a1050 = 1u64;
                    nat_eq(a1049, a1050)
                } else {
                    false
                } {
                    let a1052 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1051, _, _, _, _, _, _) => {
                            h1051
                        }
                    };
                    let a1053 = 1u64;
                    nat_eq(a1052, a1053)
                } else {
                    false
                }
            } else {
                let a1055 = match v2.clone() {
                    Adt2::C0(_, h1054, _, _, _, _, _, _, _, _, _) => {
                        h1054
                    }
                };
                let a1056 = 1u64;
                if if if nat_eq(a1055, a1056) {
                    let a1059 = match v1.clone() {
                        Adt2::C0(_, _, h1057, _, _, _, _, _, _, _, _) => {
                            h1057
                        }
                    };
                    let a1060 = match v2.clone() {
                        Adt2::C0(_, _, h1058, _, _, _, _, _, _, _, _) => {
                            h1058
                        }
                    };
                    nat_eq(a1059, a1060)
                } else {
                    false
                } {
                    let a1063 = match v1.clone() {
                        Adt2::C0(_, _, _, h1061, _, _, _, _, _, _, _) => {
                            h1061
                        }
                    };
                    let a1064 = match v2.clone() {
                        Adt2::C0(_, _, _, h1062, _, _, _, _, _, _, _) => {
                            h1062
                        }
                    };
                    nat_eq(a1063, a1064)
                } else {
                    false
                } {
                    let a1067 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1065, _, _, _, _, _, _) => {
                            h1065
                        }
                    };
                    let a1068 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1066, _, _, _, _, _, _) => {
                            h1066
                        }
                    };
                    nat_eq(a1067, a1068)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f11(v1, 1u64);
            Ok(Adt6::C4(f71(v4.clone(), v0, v3)?, {
                let a1069 = v3;
                let a1070 = 0u64;
                if nat_eq(a1069, a1070) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f11(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f30(v0: u64, v1: u64) -> Option<u64> {
    let a1071 = v0;
    let a1072 = 8u64;
    if nat_eq(a1071, a1072) {
        Some(5u64)
    } else {
        let a1073 = v0;
        let a1074 = 9u64;
        if nat_eq(a1073, a1074) {
            Some(6u64)
        } else {
            let a1075 = v0;
            let a1076 = 89u64;
            if nat_eq(a1075, a1076) {
                let a1077 = v1;
                let a1078 = 0u64;
                if nat_eq(a1077, a1078) {
                    Some(0u64)
                } else {
                    let a1079 = v1;
                    let a1080 = 1u64;
                    if nat_eq(a1079, a1080) {
                        Some(1u64)
                    } else {
                        let a1081 = v1;
                        let a1082 = 2u64;
                        if nat_eq(a1081, a1082) {
                            Some(2u64)
                        } else {
                            let a1083 = v1;
                            let a1084 = 3u64;
                            if nat_eq(a1083, a1084) {
                                Some(3u64)
                            } else {
                                let a1085 = v1;
                                let a1086 = 6u64;
                                if nat_eq(a1085, a1086) {
                                    Some(4u64)
                                } else {
                                    let a1087 = v1;
                                    let a1088 = 12u64;
                                    if nat_eq(a1087, a1088) {
                                        Some(7u64)
                                    } else {
                                        let a1089 = v1;
                                        let a1090 = 11u64;
                                        if nat_eq(a1089, a1090) {
                                            Some(8u64)
                                        } else {
                                            let a1091 = v1;
                                            let a1092 = 18u64;
                                            if nat_eq(a1091, a1092) {
                                                Some(9u64)
                                            } else {
                                                let a1093 = v1;
                                                let a1094 = 19u64;
                                                if nat_eq(a1093, a1094) {
                                                    Some(10u64)
                                                } else {
                                                    let a1095 = v1;
                                                    let a1096 = 20u64;
                                                    if nat_eq(a1095, a1096) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1097 = v1;
                                                        let a1098 = 21u64;
                                                        if nat_eq(a1097, a1098) {
                                                            Some(12u64)
                                                        } else {
                                                            None::<u64>
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f31(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1099 = f72(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1099 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f11(v2.clone(), 1u64), f11(v4, 1u64), Adt4::C10(v5, v6, {
                let m1100 = v3;
                match m1100 {
                    None => {
                        f11(v2, 1u64)
                    }
                    Some(v12) => {
                        f11(v12, 1u64)
                    }
                }
            })))
        }
        Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
            Ok(Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22))
        }
        Adt6::C4(v23, v24, v25, v26, v27) => {
            Ok(Adt6::C4(v23, v24, v25, v26, v27))
        }
    }
}

pub fn f32(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1102 = match v1.clone() {
        Adt2::C0(h1101, _, _, _, _, _, _, _, _, _, _) => {
            h1101
        }
    };
    let a1103 = 0u64;
    if if if if if nat_eq(a1102, a1103) {
        true
    } else {
        let a1105 = match v1.clone() {
            Adt2::C0(h1104, _, _, _, _, _, _, _, _, _, _) => {
                h1104
            }
        };
        let a1106 = 1u64;
        nat_eq(a1105, a1106)
    } {
        true
    } else {
        let a1108 = match v1.clone() {
            Adt2::C0(h1107, _, _, _, _, _, _, _, _, _, _) => {
                h1107
            }
        };
        let a1109 = 30u64;
        nat_eq(a1108, a1109)
    } {
        let a1112 = match v1.clone() {
            Adt2::C0(h1110, _, _, _, _, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1113 = match v2.clone() {
            Adt2::C0(h1111, _, _, _, _, _, _, _, _, _, _) => {
                h1111
            }
        };
        if nat_eq(a1112, a1113) {
            true
        } else {
            let a1115 = match v2.clone() {
                Adt2::C0(h1114, _, _, _, _, _, _, _, _, _, _) => {
                    h1114
                }
            };
            let a1116 = 0u64;
            nat_eq(a1115, a1116)
        }
    } else {
        false
    } {
        f8(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f56(v2.clone(), 1u64)?, {
            let a1118 = match v2.clone() {
                Adt2::C0(h1117, _, _, _, _, _, _, _, _, _, _) => {
                    h1117
                }
            };
            let a1119 = 0u64;
            if nat_eq(a1118, a1119) {
                4u64
            } else {
                2u64
            }
        }, f11(v1.clone(), 1u64), f11(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1120, _, _, _, _, _, _, _, _, _, _) => {
                h1120
            }
        }, match v2 {
            Adt2::C0(h1121, _, _, _, _, _, _, _, _, _, _) => {
                h1121
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f33(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1123 = v0;
        let a1124 = match v1.clone() {
            Adt3::C0(h1122, _, _, _, _, _, _, _) => {
                h1122
            }
        };
        let a1125 = 0u64;
        nat_quot(a1123, a1124, a1125)
    };
    let v3: u64 = {
        let a1127 = v2;
        let a1128 = match v1.clone() {
            Adt3::C0(_, h1126, _, _, _, _, _, _) => {
                h1126
            }
        };
        let a1129 = 0u64;
        nat_quot(a1127, a1128, a1129)
    };
    f73({
        let a1131 = v0;
        let a1132 = match v1.clone() {
            Adt3::C0(h1130, _, _, _, _, _, _, _) => {
                h1130
            }
        };
        let a1133 = 0u64;
        nat_rem(a1131, a1132, a1133)
    }, {
        let a1135 = v2;
        let a1136 = match v1.clone() {
            Adt3::C0(_, h1134, _, _, _, _, _, _) => {
                h1134
            }
        };
        let a1137 = 0u64;
        nat_rem(a1135, a1136, a1137)
    }, {
        let a1139 = v3;
        let a1140 = match v1.clone() {
            Adt3::C0(_, _, h1138, _, _, _, _, _) => {
                h1138
            }
        };
        let a1141 = 0u64;
        nat_rem(a1139, a1140, a1141)
    }, {
        let a1143 = v3;
        let a1144 = match v1.clone() {
            Adt3::C0(_, _, h1142, _, _, _, _, _) => {
                h1142
            }
        };
        let a1145 = 0u64;
        nat_quot(a1143, a1144, a1145)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1146, _, _, _) => {
            h1146
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1147, _, _) => {
            h1147
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1148, _) => {
            h1148
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1149) => {
            h1149
        }
    })
}

pub fn f34(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1167 = {
        let a1162 = f74(v2, {
            let a1153 = {
                let a1150 = v0;
                let a1151 = v1;
                let a1152 = 0u64;
                nat_quot(a1150, a1151, a1152)
            };
            let a1154 = v2;
            let a1155 = 0u64;
            nat_rem(a1153, a1154, a1155)
        }, {
            let a1159 = {
                let a1156 = v0;
                let a1157 = v1;
                let a1158 = 0u64;
                nat_quot(a1156, a1157, a1158)
            };
            let a1160 = v2;
            let a1161 = 0u64;
            nat_quot(a1159, a1160, a1161)
        })?;
        let a1163 = v1;
        nat_mul(a1162, a1163)?
    };
    let a1168 = {
        let a1164 = v0;
        let a1165 = v1;
        let a1166 = 0u64;
        nat_rem(a1164, a1165, a1166)
    };
    nat_add(a1167, a1168)
}

pub fn f35(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1170 = v0;
        let a1171 = match v2.clone() {
            Adt3::C0(h1169, _, _, _, _, _, _, _) => {
                h1169
            }
        };
        let a1172 = 0u64;
        nat_quot(a1170, a1171, a1172)
    };
    let v4: u64 = {
        let a1174 = v3;
        let a1175 = match v2.clone() {
            Adt3::C0(_, h1173, _, _, _, _, _, _) => {
                h1173
            }
        };
        let a1176 = 0u64;
        nat_quot(a1174, a1175, a1176)
    };
    f75({
        let a1178 = v0;
        let a1179 = match v2.clone() {
            Adt3::C0(h1177, _, _, _, _, _, _, _) => {
                h1177
            }
        };
        let a1180 = 0u64;
        nat_rem(a1178, a1179, a1180)
    }, {
        let a1182 = v3;
        let a1183 = match v2.clone() {
            Adt3::C0(_, h1181, _, _, _, _, _, _) => {
                h1181
            }
        };
        let a1184 = 0u64;
        nat_rem(a1182, a1183, a1184)
    }, {
        let a1186 = v4;
        let a1187 = match v2.clone() {
            Adt3::C0(_, _, h1185, _, _, _, _, _) => {
                h1185
            }
        };
        let a1188 = 0u64;
        nat_rem(a1186, a1187, a1188)
    }, {
        let a1190 = v4;
        let a1191 = match v2 {
            Adt3::C0(_, _, h1189, _, _, _, _, _) => {
                h1189
            }
        };
        let a1192 = 0u64;
        nat_quot(a1190, a1191, a1192)
    }, match v1.clone() {
        Adt3::C0(h1193, _, _, _, _, _, _, _) => {
            h1193
        }
    }, match v1.clone() {
        Adt3::C0(_, h1194, _, _, _, _, _, _) => {
            h1194
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1195, _, _, _, _, _) => {
            h1195
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1196, _, _, _, _) => {
            h1196
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1197, _, _, _) => {
            h1197
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1198, _, _) => {
            h1198
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1199, _) => {
            h1199
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1200) => {
            h1200
        }
    })
}

pub fn f36(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1202 = v0;
        let a1203 = match v3.clone() {
            Adt3::C0(h1201, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        let a1204 = 0u64;
        nat_quot(a1202, a1203, a1204)
    };
    let v6: u64 = {
        let a1206 = v5;
        let a1207 = match v3.clone() {
            Adt3::C0(_, h1205, _, _, _, _, _, _) => {
                h1205
            }
        };
        let a1208 = 0u64;
        nat_quot(a1206, a1207, a1208)
    };
    let v7: u64 = {
        let a1210 = v0;
        let a1211 = match v3.clone() {
            Adt3::C0(h1209, _, _, _, _, _, _, _) => {
                h1209
            }
        };
        let a1212 = 0u64;
        nat_rem(a1210, a1211, a1212)
    };
    let v8: u64 = {
        let a1214 = v5;
        let a1215 = match v3.clone() {
            Adt3::C0(_, h1213, _, _, _, _, _, _) => {
                h1213
            }
        };
        let a1216 = 0u64;
        nat_rem(a1214, a1215, a1216)
    };
    let v9: u64 = {
        let a1218 = v6;
        let a1219 = match v3.clone() {
            Adt3::C0(_, _, h1217, _, _, _, _, _) => {
                h1217
            }
        };
        let a1220 = 0u64;
        nat_rem(a1218, a1219, a1220)
    };
    let v10: u64 = {
        let a1222 = v6;
        let a1223 = match v3.clone() {
            Adt3::C0(_, _, h1221, _, _, _, _, _) => {
                h1221
            }
        };
        let a1224 = 0u64;
        nat_quot(a1222, a1223, a1224)
    };
    let v11: u64 = {
        let a1225 = v4;
        let a1226 = 0u64;
        if nat_eq(a1225, a1226) {
            match v1.clone() {
                Adt3::C0(h1227, _, _, _, _, _, _, _) => {
                    h1227
                }
            }
        } else {
            let a1228 = v4;
            let a1229 = 1u64;
            if nat_eq(a1228, a1229) {
                match v1.clone() {
                    Adt3::C0(_, h1230, _, _, _, _, _, _) => {
                        h1230
                    }
                }
            } else {
                let a1231 = v4;
                let a1232 = 2u64;
                if nat_eq(a1231, a1232) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1233, _, _, _, _, _) => {
                            h1233
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1234, _, _, _, _) => {
                            h1234
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1241 = v11;
        let a1242 = {
            let a1235 = v4;
            let a1236 = 0u64;
            if nat_eq(a1235, a1236) {
                v7
            } else {
                let a1237 = v4;
                let a1238 = 1u64;
                if nat_eq(a1237, a1238) {
                    v8
                } else {
                    let a1239 = v4;
                    let a1240 = 2u64;
                    if nat_eq(a1239, a1240) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1241, a1242)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f73(if if v12 {
        let a1243 = v4;
        let a1244 = 0u64;
        nat_eq(a1243, a1244)
    } else {
        false
    } {
        let a1245 = v7;
        let a1246 = v11;
        nat_sub(a1245, a1246)
    } else {
        v7
    }, if if v12 {
        let a1247 = v4;
        let a1248 = 1u64;
        nat_eq(a1247, a1248)
    } else {
        false
    } {
        let a1249 = v8;
        let a1250 = v11;
        nat_sub(a1249, a1250)
    } else {
        v8
    }, if if v12 {
        let a1251 = v4;
        let a1252 = 2u64;
        nat_eq(a1251, a1252)
    } else {
        false
    } {
        let a1253 = v9;
        let a1254 = v11;
        nat_sub(a1253, a1254)
    } else {
        v9
    }, if if v12 {
        let a1255 = v4;
        let a1256 = 3u64;
        nat_eq(a1255, a1256)
    } else {
        false
    } {
        let a1257 = v10;
        let a1258 = v11;
        nat_sub(a1257, a1258)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1259, _, _, _) => {
            h1259
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1260, _, _) => {
            h1260
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1261, _) => {
            h1261
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1262) => {
            h1262
        }
    })?, f33(v0, v3)?))
}

pub fn f37(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f76(v0, v2.clone(), 0u64);
    let v9: u64 = f76(v0, v2.clone(), 1u64);
    let v10: u64 = f76(v0, v2.clone(), 2u64);
    let v11: u64 = f76(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f77(v1.clone(), f78(v8, v3, match v1.clone() {
            Adt3::C0(h1263, _, _, _, _, _, _, _) => {
                h1263
            }
        })?, f78(v9, v4, match v1.clone() {
            Adt3::C0(_, h1264, _, _, _, _, _, _) => {
                h1264
            }
        })?, f78(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1265, _, _, _, _, _) => {
                h1265
            }
        })?, f78(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1266, _, _, _, _) => {
                h1266
            }
        })?)?, f33(v0, v2)?))
    } else {
        if if if if f79(v8, v3, match v1.clone() {
            Adt3::C0(h1267, _, _, _, _, _, _, _) => {
                h1267
            }
        })? {
            f79(v9, v4, match v1.clone() {
                Adt3::C0(_, h1268, _, _, _, _, _, _) => {
                    h1268
                }
            })?
        } else {
            false
        } {
            f79(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1269, _, _, _, _, _) => {
                    h1269
                }
            })?
        } else {
            false
        } {
            f79(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1270, _, _, _, _) => {
                    h1270
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f77(v1, f80(v8, v3)?, f80(v9, v4)?, f80(v10, v5)?, f80(v11, v6)?)?, f33(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f33(v0, v2)?))
        }
    }
}

pub fn f38(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f76(v0, v2.clone(), 0u64);
    let v5: u64 = f76(v0, v2.clone(), 1u64);
    let v6: u64 = f76(v0, v2.clone(), 2u64);
    let v7: u64 = f76(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f77(v1.clone(), f81(v4, v3, match v1 {
        Adt3::C0(h1271, _, _, _, _, _, _, _) => {
            h1271
        }
    })?, v5, v6, v7)?, f33(v0, v2)?))
}

pub fn f39(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1272 = v0;
    let a1273 = v3;
    if nat_lt(a1272, a1273) {
        Ok(Adt5::C1(false, {
            let a1274 = v0;
            let a1275 = 4u64;
            nat_mul(a1274, a1275)?
        }, {
            let a1276 = v0;
            let a1277 = 4u64;
            nat_mul(a1276, a1277)?
        }))
    } else {
        let v6: u64 = {
            let a1278 = v0;
            let a1279 = v3;
            nat_sub(a1278, a1279)
        };
        let v7: u64 = f33(v6, v1)?;
        let v8: u64 = f82(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f40(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1280 = v0;
    let a1281 = v3;
    if nat_lt(a1280, a1281) {
        Ok(Adt5::C1(true, 0u64, f33(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1282 = v0;
            let a1283 = v3;
            nat_sub(a1282, a1283)
        };
        let v5: u64 = f35(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f33(v4, v1)?, 3u64, v5))
    }
}

pub fn f41(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1285 = v1;
        let a1286 = match v3.clone() {
            Adt3::C0(h1284, _, _, _, _, _, _, _) => {
                h1284
            }
        };
        let a1287 = 0u64;
        nat_rem(a1285, a1286, a1287)
    };
    let v5: u64 = {
        let a1293 = {
            let a1289 = v1;
            let a1290 = match v3.clone() {
                Adt3::C0(h1288, _, _, _, _, _, _, _) => {
                    h1288
                }
            };
            let a1291 = 0u64;
            nat_quot(a1289, a1290, a1291)
        };
        let a1294 = match v3.clone() {
            Adt3::C0(_, h1292, _, _, _, _, _, _) => {
                h1292
            }
        };
        let a1295 = 0u64;
        nat_rem(a1293, a1294, a1295)
    };
    let v6: bool = f83(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f35(v1, v2, v3.clone())?
    } else {
        0u64
    }, f33(v1, v3)?))
}

pub fn f42(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1296 = v3;
        let a1297 = 8u64;
        nat_add(a1296, a1297)?
    }, 0u64, f33(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f35(v0, v5, v2.clone())?
    }, 3u64, f33(v0, v2)?))
}

pub fn f43(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1299 = v0;
        let a1300 = match v2.clone() {
            Adt3::C0(h1298, _, _, _, _, _, _, _) => {
                h1298
            }
        };
        let a1301 = 0u64;
        nat_rem(a1299, a1300, a1301)
    };
    let v7: u64 = {
        let a1303 = v0;
        let a1304 = match v2.clone() {
            Adt3::C0(h1302, _, _, _, _, _, _, _) => {
                h1302
            }
        };
        let a1305 = 0u64;
        nat_quot(a1303, a1304, a1305)
    };
    let v8: u64 = {
        let a1307 = v7;
        let a1308 = match v2.clone() {
            Adt3::C0(_, h1306, _, _, _, _, _, _) => {
                h1306
            }
        };
        let a1309 = 0u64;
        nat_rem(a1307, a1308, a1309)
    };
    let v9: u64 = {
        let a1311 = v7;
        let a1312 = match v2.clone() {
            Adt3::C0(_, h1310, _, _, _, _, _, _) => {
                h1310
            }
        };
        let a1313 = 0u64;
        nat_quot(a1311, a1312, a1313)
    };
    let v10: u64 = {
        let a1315 = v9;
        let a1316 = match v2.clone() {
            Adt3::C0(_, _, h1314, _, _, _, _, _) => {
                h1314
            }
        };
        let a1317 = 0u64;
        nat_rem(a1315, a1316, a1317)
    };
    let v11: u64 = {
        let a1319 = v9;
        let a1320 = match v2.clone() {
            Adt3::C0(_, _, h1318, _, _, _, _, _) => {
                h1318
            }
        };
        let a1321 = 0u64;
        nat_quot(a1319, a1320, a1321)
    };
    let v12: u64 = f33(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1330 = {
                let a1325 = {
                    let a1322 = v10;
                    let a1323 = v4;
                    let a1324 = 0u64;
                    nat_quot(a1322, a1323, a1324)
                };
                let a1326 = v5;
                nat_mul(a1325, a1326)?
            };
            let a1331 = {
                let a1327 = v8;
                let a1328 = v4;
                let a1329 = 0u64;
                nat_quot(a1327, a1328, a1329)
            };
            nat_add(a1330, a1331)?
        };
        let v14: u64 = {
            let a1351 = v6;
            let a1352 = {
                let a1349 = match v1.clone() {
                    Adt3::C0(h1332, _, _, _, _, _, _, _) => {
                        h1332
                    }
                };
                let a1350 = {
                    let a1347 = {
                        let a1333 = v8;
                        let a1334 = v4;
                        let a1335 = 0u64;
                        nat_rem(a1333, a1334, a1335)
                    };
                    let a1348 = {
                        let a1345 = match v1.clone() {
                            Adt3::C0(_, h1336, _, _, _, _, _, _) => {
                                h1336
                            }
                        };
                        let a1346 = {
                            let a1343 = {
                                let a1337 = v10;
                                let a1338 = v4;
                                let a1339 = 0u64;
                                nat_rem(a1337, a1338, a1339)
                            };
                            let a1344 = {
                                let a1341 = match v1.clone() {
                                    Adt3::C0(_, _, h1340, _, _, _, _, _) => {
                                        h1340
                                    }
                                };
                                let a1342 = v13;
                                nat_mul(a1341, a1342)?
                            };
                            nat_add(a1343, a1344)?
                        };
                        nat_mul(a1345, a1346)?
                    };
                    nat_add(a1347, a1348)?
                };
                nat_mul(a1349, a1350)?
            };
            nat_add(a1351, a1352)?
        };
        Ok(Adt5::C1(false, f33(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1358 = {
                let a1356 = {
                    let a1353 = v11;
                    let a1354 = v5;
                    let a1355 = 0u64;
                    nat_rem(a1353, a1354, a1355)
                };
                let a1357 = v4;
                nat_mul(a1356, a1357)?
            };
            let a1359 = v8;
            nat_add(a1358, a1359)?
        };
        let v16: u64 = {
            let a1365 = {
                let a1363 = {
                    let a1360 = v11;
                    let a1361 = v5;
                    let a1362 = 0u64;
                    nat_quot(a1360, a1361, a1362)
                };
                let a1364 = v4;
                nat_mul(a1363, a1364)?
            };
            let a1366 = v10;
            nat_add(a1365, a1366)?
        };
        let a1368 = v15;
        let a1369 = match v1.clone() {
            Adt3::C0(_, h1367, _, _, _, _, _, _) => {
                h1367
            }
        };
        if if nat_lt(a1368, a1369) {
            let a1371 = v16;
            let a1372 = match v1.clone() {
                Adt3::C0(_, _, h1370, _, _, _, _, _) => {
                    h1370
                }
            };
            nat_lt(a1371, a1372)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f33({
                let a1381 = v6;
                let a1382 = {
                    let a1379 = match v1.clone() {
                        Adt3::C0(h1373, _, _, _, _, _, _, _) => {
                            h1373
                        }
                    };
                    let a1380 = {
                        let a1377 = v15;
                        let a1378 = {
                            let a1375 = match v1.clone() {
                                Adt3::C0(_, h1374, _, _, _, _, _, _) => {
                                    h1374
                                }
                            };
                            let a1376 = v16;
                            nat_mul(a1375, a1376)?
                        };
                        nat_add(a1377, a1378)?
                    };
                    nat_mul(a1379, a1380)?
                };
                nat_add(a1381, a1382)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f44(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f33(v0, v2.clone())?;
    let v4: u64 = f33(v0, v1.clone())?;
    let a1387 = {
        let a1384 = v0;
        let a1385 = match v1 {
            Adt3::C0(h1383, _, _, _, _, _, _, _) => {
                h1383
            }
        };
        let a1386 = 0u64;
        nat_rem(a1384, a1385, a1386)
    };
    let a1388 = 0u64;
    if nat_eq(a1387, a1388) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f33({
            let a1389 = v0;
            let a1390 = 1u64;
            nat_sub(a1389, a1390)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f45(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1391 = v3;
    let a1392 = 13u64;
    if nat_eq(a1391, a1392) {
        f84(v0, v1, v4)
    } else {
        let v5: u64 = f85(match v1.clone() {
            Adt3::C0(h1393, _, _, _, _, _, _, _) => {
                h1393
            }
        }, {
            let a1394 = v3;
            let a1395 = 16u64;
            nat_eq(a1394, a1395)
        })?;
        let v6: u64 = {
            let a1396 = v0;
            let a1397 = v5;
            let a1398 = 0u64;
            nat_quot(a1396, a1397, a1398)
        };
        let v7: u64 = {
            let a1399 = v0;
            let a1400 = v5;
            let a1401 = 0u64;
            nat_rem(a1399, a1400, a1401)
        };
        let v8: u64 = {
            let a1405 = 1u64;
            let a1406 = {
                let a1403 = match v1.clone() {
                    Adt3::C0(h1402, _, _, _, _, _, _, _) => {
                        h1402
                    }
                };
                let a1404 = 2u64;
                nat_mul(a1403, a1404)?
            };
            nat_add(a1405, a1406)?
        };
        let a1407 = v7;
        let a1408 = v8;
        if nat_lt(a1407, a1408) {
            f86(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f33(v6, v2)?;
            let a1409 = v3;
            let a1410 = 14u64;
            if nat_eq(a1409, a1410) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1411 = v7;
                let a1412 = v8;
                if nat_eq(a1411, a1412) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1415 = v7;
                    let a1416 = {
                        let a1413 = v8;
                        let a1414 = 1u64;
                        nat_add(a1413, a1414)?
                    };
                    if nat_eq(a1415, a1416) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1417, _, _, _, _, _, _, _) => {
                                h1417
                            }
                        }, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(5u64, 1u64, 1u64, 1u64, 2u64, 3u64, v9))
                    }
                }
            }
        }
    }
}

pub fn f46(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1418 = v0;
    let a1419 = 0u64;
    if nat_eq(a1418, a1419) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1426 = {
            let a1420 = v0;
            let a1421 = 1u64;
            nat_add(a1420, a1421)?
        };
        let a1427 = {
            let a1424 = 2u64;
            let a1425 = {
                let a1422 = v3;
                let a1423 = 2u64;
                nat_mul(a1422, a1423)?
            };
            nat_add(a1424, a1425)?
        };
        if nat_eq(a1426, a1427) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1428 = v0;
                let a1429 = 1u64;
                nat_sub(a1428, a1429)
            };
            let v5: u64 = {
                let a1430 = v4;
                let a1431 = 2u64;
                let a1432 = 0u64;
                nat_quot(a1430, a1431, a1432)
            };
            let a1436 = {
                let a1433 = v4;
                let a1434 = 2u64;
                let a1435 = 0u64;
                nat_rem(a1433, a1434, a1435)
            };
            let a1437 = 0u64;
            if nat_eq(a1436, a1437) {
                Ok(Adt5::C4(17u64, 0u64, f33(v5, v1)?, 4u64, f33(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f47(v0: u64) -> R<u64> {
    let a1440 = 3u64;
    let a1441 = {
        let a1438 = v0;
        let a1439 = 3u64;
        nat_mul(a1438, a1439)?
    };
    nat_add(a1440, a1441)
}

pub fn f48(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1449 = {
        let a1442 = 2u64;
        let a1443 = f49(v1.clone())?;
        nat_mul(a1442, a1443)?
    };
    let a1450 = {
        let a1447 = f49(v1)?;
        let a1448 = {
            let a1445 = 1u64;
            let a1446 = match v0 {
                Adt3::C0(h1444, _, _, _, _, _, _, _) => {
                    h1444
                }
            };
            nat_add(a1445, a1446)?
        };
        nat_mul(a1447, a1448)?
    };
    nat_add(a1449, a1450)
}

pub fn f49(v0: Adt3) -> R<u64> {
    if f57(match v0.clone() {
        Adt3::C0(h1451, _, _, _, _, _, _, _) => {
            h1451
        }
    }, match v0.clone() {
        Adt3::C0(_, h1452, _, _, _, _, _, _) => {
            h1452
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1453, _, _, _, _, _) => {
            h1453
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1454, _, _, _, _) => {
            h1454
        }
    }) {
        Ok(0u64)
    } else {
        let a1463 = match v0.clone() {
            Adt3::C0(h1455, _, _, _, _, _, _, _) => {
                h1455
            }
        };
        let a1464 = {
            let a1461 = match v0.clone() {
                Adt3::C0(_, h1456, _, _, _, _, _, _) => {
                    h1456
                }
            };
            let a1462 = {
                let a1459 = match v0.clone() {
                    Adt3::C0(_, _, h1457, _, _, _, _, _) => {
                        h1457
                    }
                };
                let a1460 = match v0 {
                    Adt3::C0(_, _, _, h1458, _, _, _, _) => {
                        h1458
                    }
                };
                nat_mul(a1459, a1460)?
            };
            nat_mul(a1461, a1462)?
        };
        nat_mul(a1463, a1464)
    }
}

pub fn f50(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f33(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f51(v0: u64, v1: u64) -> bool {
    let a1465 = v1;
    let a1466 = 2147483647u64;
    if nat_le(a1465, a1466) {
        let a1467 = v1;
        let a1468 = v0;
        nat_lt(a1467, a1468)
    } else {
        false
    }
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1469 = f60(v0);
    match m1469 {
        Adt7::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt7::C1(v7) => {
            let v8: u64 = {
                let a1471 = v2;
                let a1472 = match v7.clone() {
                    Adt8::C0(_, h1470, _) => {
                        h1470
                    }
                };
                nat_mul(a1471, a1472)?
            };
            let m1485 = f87(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1473, _, _, _, _, _, _, _) => {
                    h1473
                }
            }, match v5.clone() {
                Adt3::C0(_, h1474, _, _, _, _, _, _) => {
                    h1474
                }
            }, match v5 {
                Adt3::C0(_, _, h1475, _, _, _, _, _) => {
                    h1475
                }
            }, {
                let a1478 = match v3.clone() {
                    Adt3::C0(h1476, _, _, _, _, _, _, _) => {
                        h1476
                    }
                };
                let a1479 = match v7.clone() {
                    Adt8::C0(_, h1477, _) => {
                        h1477
                    }
                };
                nat_mul(a1478, a1479)?
            }, match v3.clone() {
                Adt3::C0(_, h1480, _, _, _, _, _, _) => {
                    h1480
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1481, _, _, _) => {
                    h1481
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1482, _, _) => {
                    h1482
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1483, _) => {
                    h1483
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1484) => {
                    h1484
                }
            })?;
            match m1485 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1486 = v0;
                    let a1487 = 0u64;
                    if if nat_eq(a1486, a1487) {
                        true
                    } else {
                        let a1488 = v0;
                        let a1489 = 26u64;
                        nat_eq(a1488, a1489)
                    } {
                        Ok(Adt5::C1(false, v9, f33({
                            let a1493 = {
                                let a1491 = v1;
                                let a1492 = match v4.clone() {
                                    Adt3::C0(h1490, _, _, _, _, _, _, _) => {
                                        h1490
                                    }
                                };
                                nat_mul(a1491, a1492)?
                            };
                            let a1494 = v8;
                            nat_add(a1493, a1494)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt8::C0(_, _, h1495) => {
                                h1495
                            }
                        }, f33({
                            let a1499 = {
                                let a1497 = v1;
                                let a1498 = match v4.clone() {
                                    Adt3::C0(h1496, _, _, _, _, _, _, _) => {
                                        h1496
                                    }
                                };
                                nat_mul(a1497, a1498)?
                            };
                            let a1500 = v8;
                            nat_add(a1499, a1500)?
                        }, v4.clone())?, {
                            let a1507 = {
                                let a1505 = {
                                    let a1502 = match v7 {
                                        Adt8::C0(_, h1501, _) => {
                                            h1501
                                        }
                                    };
                                    let a1503 = 1u64;
                                    nat_sub(a1502, a1503)
                                };
                                let a1506 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1504, _, _, _) => {
                                        h1504
                                    }
                                };
                                nat_mul(a1505, a1506)?
                            };
                            let a1508 = 4u64;
                            nat_add(a1507, a1508)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1509, _, _, _) => {
                                h1509
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1510 = v1;
    let a1511 = 0u64;
    if if nat_eq(a1510, a1511) {
        true
    } else {
        let a1512 = v2;
        let a1513 = 0u64;
        nat_eq(a1512, a1513)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f54(v0: u64, v1: u64) -> bool {
    let a1514 = v0;
    let a1515 = 4294967295u64;
    if nat_eq(a1514, a1515) {
        true
    } else {
        let a1516 = v0;
        let a1517 = 2147483648u64;
        if nat_lt(a1516, a1517) {
            let a1518 = v0;
            let a1519 = v1;
            nat_lt(a1518, a1519)
        } else {
            false
        }
    }
}

pub fn f55(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f88(match v0.clone() {
        Adt2::C0(h1520, _, _, _, _, _, _, _, _, _, _) => {
            h1520
        }
    }, match v0.clone() {
        Adt2::C0(_, h1521, _, _, _, _, _, _, _, _, _) => {
            h1521
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1522, _, _, _, _, _, _, _, _) => {
            h1522
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1523, _, _, _, _, _, _, _) => {
            h1523
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1524, _, _, _, _, _, _) => {
            h1524
        }
    }, match v1.clone() {
        Adt2::C0(h1525, _, _, _, _, _, _, _, _, _, _) => {
            h1525
        }
    }, match v1.clone() {
        Adt2::C0(_, h1526, _, _, _, _, _, _, _, _, _) => {
            h1526
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1527, _, _, _, _, _, _, _, _) => {
            h1527
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1528, _, _, _, _, _, _, _) => {
            h1528
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1529, _, _, _, _, _, _) => {
            h1529
        }
    }, match v2.clone() {
        Adt2::C0(h1530, _, _, _, _, _, _, _, _, _, _) => {
            h1530
        }
    }, match v2.clone() {
        Adt2::C0(_, h1531, _, _, _, _, _, _, _, _, _) => {
            h1531
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1532, _, _, _, _, _, _, _, _) => {
            h1532
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1533, _, _, _, _, _, _, _) => {
            h1533
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1534, _, _, _, _, _, _) => {
            h1534
        }
    }) {
        let m1536 = f60(match v0.clone() {
            Adt2::C0(h1535, _, _, _, _, _, _, _, _, _, _) => {
                h1535
            }
        });
        match m1536 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                let a1542 = {
                    let a1539 = match v0.clone() {
                        Adt2::C0(_, h1537, _, _, _, _, _, _, _, _, _) => {
                            h1537
                        }
                    };
                    let a1540 = match v3.clone() {
                        Adt8::C0(_, h1538, _) => {
                            h1538
                        }
                    };
                    let a1541 = 0u64;
                    nat_rem(a1539, a1540, a1541)
                };
                let a1543 = 0u64;
                if if nat_eq(a1542, a1543) {
                    if f89(match v0.clone() {
                        Adt2::C0(_, h1544, _, _, _, _, _, _, _, _, _) => {
                            h1544
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1545, _, _, _, _, _, _, _, _, _) => {
                            h1545
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1546, _, _, _, _, _, _, _, _) => {
                            h1546
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1547, _, _, _, _, _, _, _) => {
                            h1547
                        }
                    }) {
                        true
                    } else {
                        let a1549 = 4u64;
                        let a1550 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1548, _, _, _, _, _) => {
                                h1548
                            }
                        };
                        if if nat_le(a1549, a1550) {
                            let a1552 = match v3.clone() {
                                Adt8::C0(_, h1551, _) => {
                                    h1551
                                }
                            };
                            let a1553 = 1u64;
                            if nat_eq(a1552, a1553) {
                                true
                            } else {
                                let a1556 = match v3.clone() {
                                    Adt8::C0(_, _, h1554) => {
                                        h1554
                                    }
                                };
                                let a1557 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1555, _, _, _, _, _) => {
                                        h1555
                                    }
                                };
                                nat_le(a1556, a1557)
                            }
                        } else {
                            false
                        } {
                            if f9(v0.clone(), v2.clone())? {
                                f9(v1.clone(), v2.clone())?
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                } else {
                    false
                } {
                    let v4: Adt3 = f11(v0.clone(), match v3 {
                        Adt8::C0(_, h1558, _) => {
                            h1558
                        }
                    });
                    Ok(Adt6::C4(if f89(match v0.clone() {
                        Adt2::C0(_, h1559, _, _, _, _, _, _, _, _, _) => {
                            h1559
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1560, _, _, _, _, _, _, _, _, _) => {
                            h1560
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1561, _, _, _, _, _, _, _, _) => {
                            h1561
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1562, _, _, _, _, _, _, _) => {
                            h1562
                        }
                    }) {
                        0u64
                    } else {
                        f48(v4.clone(), f11(v1.clone(), 1u64))?
                    }, 4u64, v4, f11(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1563, _, _, _, _, _, _, _, _, _, _) => {
                            h1563
                        }
                    }, f11(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f56(v0: Adt2, v1: u64) -> R<u64> {
    let a1575 = {
        let a1569 = {
            let a1565 = match v0.clone() {
                Adt2::C0(_, h1564, _, _, _, _, _, _, _, _, _) => {
                    h1564
                }
            };
            let a1566 = v1;
            let a1567 = 0u64;
            nat_quot(a1565, a1566, a1567)
        };
        let a1570 = match v0.clone() {
            Adt2::C0(_, _, h1568, _, _, _, _, _, _, _, _) => {
                h1568
            }
        };
        nat_mul(a1569, a1570)?
    };
    let a1576 = {
        let a1573 = match v0.clone() {
            Adt2::C0(_, _, _, h1571, _, _, _, _, _, _, _) => {
                h1571
            }
        };
        let a1574 = match v0 {
            Adt2::C0(_, _, _, _, h1572, _, _, _, _, _, _) => {
                h1572
            }
        };
        nat_mul(a1573, a1574)?
    };
    nat_mul(a1575, a1576)
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1577 = v0;
    let a1578 = 0u64;
    if if if nat_eq(a1577, a1578) {
        true
    } else {
        let a1579 = v1;
        let a1580 = 0u64;
        nat_eq(a1579, a1580)
    } {
        true
    } else {
        let a1581 = v2;
        let a1582 = 0u64;
        nat_eq(a1581, a1582)
    } {
        true
    } else {
        let a1583 = v3;
        let a1584 = 0u64;
        nat_eq(a1583, a1584)
    }
}

pub fn f58(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1587 = match v1.clone() {
        Adt2::C0(h1585, _, _, _, _, _, _, _, _, _, _) => {
            h1585
        }
    };
    let a1588 = match v3.clone() {
        Adt2::C0(h1586, _, _, _, _, _, _, _, _, _, _) => {
            h1586
        }
    };
    if if nat_eq(a1587, a1588) {
        let a1589 = v0;
        let a1590 = 33u64;
        if nat_eq(a1589, a1590) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1592 = f60(match v1.clone() {
            Adt2::C0(h1591, _, _, _, _, _, _, _, _, _, _) => {
                h1591
            }
        });
        match m1592 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h1593, _) => {
                        h1593
                    }
                };
                let v6: u64 = f56(v1.clone(), v5)?;
                let a1598 = {
                    let a1595 = match v1.clone() {
                        Adt2::C0(_, h1594, _, _, _, _, _, _, _, _, _) => {
                            h1594
                        }
                    };
                    let a1596 = v5;
                    let a1597 = 1u64;
                    nat_rem(a1595, a1596, a1597)
                };
                let a1599 = 0u64;
                if if if nat_eq(a1598, a1599) {
                    let a1604 = {
                        let a1601 = match v3.clone() {
                            Adt2::C0(_, h1600, _, _, _, _, _, _, _, _, _) => {
                                h1600
                            }
                        };
                        let a1602 = v5;
                        let a1603 = 1u64;
                        nat_rem(a1601, a1602, a1603)
                    };
                    let a1605 = 0u64;
                    nat_eq(a1604, a1605)
                } else {
                    false
                } {
                    let a1606 = v6;
                    let a1607 = f56(v3.clone(), v5)?;
                    nat_eq(a1606, a1607)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h1608) => {
                            h1608
                        }
                    }, f11(v1, v5), f11(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f59(v0: u64) -> bool {
    let a1609 = v0;
    let a1610 = 0u64;
    if if nat_eq(a1609, a1610) {
        true
    } else {
        let a1611 = v0;
        let a1612 = 1u64;
        nat_eq(a1611, a1612)
    } {
        true
    } else {
        let a1613 = v0;
        let a1614 = 30u64;
        nat_eq(a1613, a1614)
    }
}

pub fn f60(v0: u64) -> Adt7 {
    let a1615 = v0;
    let a1616 = 0u64;
    if nat_eq(a1615, a1616) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a1617 = v0;
        let a1618 = 1u64;
        if nat_eq(a1617, a1618) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a1619 = v0;
            let a1620 = 2u64;
            if nat_eq(a1619, a1620) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a1621 = v0;
                let a1622 = 3u64;
                if nat_eq(a1621, a1622) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a1623 = v0;
                    let a1624 = 6u64;
                    if nat_eq(a1623, a1624) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1625 = v0;
                        let a1626 = 7u64;
                        if nat_eq(a1625, a1626) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1627 = v0;
                            let a1628 = 8u64;
                            if nat_eq(a1627, a1628) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1629 = v0;
                                let a1630 = 9u64;
                                if nat_eq(a1629, a1630) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1631 = v0;
                                    let a1632 = 10u64;
                                    if nat_eq(a1631, a1632) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1633 = v0;
                                        let a1634 = 11u64;
                                        if nat_eq(a1633, a1634) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1635 = v0;
                                            let a1636 = 12u64;
                                            if nat_eq(a1635, a1636) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1637 = v0;
                                                let a1638 = 13u64;
                                                if nat_eq(a1637, a1638) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1639 = v0;
                                                    let a1640 = 14u64;
                                                    if nat_eq(a1639, a1640) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1641 = v0;
                                                        let a1642 = 15u64;
                                                        if nat_eq(a1641, a1642) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1643 = v0;
                                                            let a1644 = 16u64;
                                                            if nat_eq(a1643, a1644) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1645 = v0;
                                                                let a1646 = 17u64;
                                                                if nat_eq(a1645, a1646) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1647 = v0;
                                                                    let a1648 = 18u64;
                                                                    if nat_eq(a1647, a1648) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1649 = v0;
                                                                        let a1650 = 19u64;
                                                                        if nat_eq(a1649, a1650) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1651 = v0;
                                                                            let a1652 = 20u64;
                                                                            if nat_eq(a1651, a1652) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1653 = v0;
                                                                                let a1654 = 21u64;
                                                                                if nat_eq(a1653, a1654) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1655 = v0;
                                                                                    let a1656 = 22u64;
                                                                                    if nat_eq(a1655, a1656) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1657 = v0;
                                                                                        let a1658 = 23u64;
                                                                                        if nat_eq(a1657, a1658) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1659 = v0;
                                                                                            let a1660 = 24u64;
                                                                                            if nat_eq(a1659, a1660) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1661 = v0;
                                                                                                let a1662 = 25u64;
                                                                                                if nat_eq(a1661, a1662) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1663 = v0;
                                                                                                    let a1664 = 26u64;
                                                                                                    if nat_eq(a1663, a1664) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1665 = v0;
                                                                                                        let a1666 = 27u64;
                                                                                                        if nat_eq(a1665, a1666) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1667 = v0;
                                                                                                            let a1668 = 28u64;
                                                                                                            if nat_eq(a1667, a1668) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1669 = v0;
                                                                                                                let a1670 = 29u64;
                                                                                                                if nat_eq(a1669, a1670) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1671 = v0;
                                                                                                                    let a1672 = 30u64;
                                                                                                                    if nat_eq(a1671, a1672) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1673 = v0;
                                                                                                                        let a1674 = 34u64;
                                                                                                                        if nat_eq(a1673, a1674) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1675 = v0;
                                                                                                                            let a1676 = 35u64;
                                                                                                                            if nat_eq(a1675, a1676) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1677 = v0;
                                                                                                                                let a1678 = 39u64;
                                                                                                                                if nat_eq(a1677, a1678) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1679 = v0;
                                                                                                                                    let a1680 = 40u64;
                                                                                                                                    if nat_eq(a1679, a1680) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1681 = v0;
                                                                                                                                        let a1682 = 41u64;
                                                                                                                                        if nat_eq(a1681, a1682) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1683 = v0;
                                                                                                                                            let a1684 = 42u64;
                                                                                                                                            if nat_eq(a1683, a1684) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1685 = v0;
                                                                                                                                                let a1686 = 43u64;
                                                                                                                                                if nat_eq(a1685, a1686) {
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

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f57(v4, v5, v6, v7) {
        f57(v0, v1, v2, v3)
    } else {
        let a1690 = {
            let a1687 = v0;
            let a1688 = v4;
            let a1689 = 1u64;
            nat_rem(a1687, a1688, a1689)
        };
        let a1691 = 0u64;
        if if if nat_eq(a1690, a1691) {
            let a1695 = {
                let a1692 = v1;
                let a1693 = v5;
                let a1694 = 1u64;
                nat_rem(a1692, a1693, a1694)
            };
            let a1696 = 0u64;
            nat_eq(a1695, a1696)
        } else {
            false
        } {
            let a1700 = {
                let a1697 = v2;
                let a1698 = v6;
                let a1699 = 1u64;
                nat_rem(a1697, a1698, a1699)
            };
            let a1701 = 0u64;
            nat_eq(a1700, a1701)
        } else {
            false
        } {
            let a1705 = {
                let a1702 = v3;
                let a1703 = v7;
                let a1704 = 1u64;
                nat_rem(a1702, a1703, a1704)
            };
            let a1706 = 0u64;
            nat_eq(a1705, a1706)
        } else {
            false
        }
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1707 = v0;
    let a1708 = v1;
    if nat_eq(a1707, a1708) {
        let a1711 = v4;
        let a1712 = {
            let a1709 = v2;
            let a1710 = v3;
            nat_add(a1709, a1710)?
        };
        Ok(nat_eq(a1711, a1712))
    } else {
        let a1713 = v2;
        let a1714 = v3;
        if nat_eq(a1713, a1714) {
            let a1715 = v4;
            let a1716 = v2;
            Ok(nat_eq(a1715, a1716))
        } else {
            Ok(false)
        }
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1719 = {
            let a1717 = v0;
            let a1718 = f90(v1);
            nat_add(a1717, a1718)?
        };
        let a1720 = f90(v2);
        nat_add(a1719, a1720)?
    };
    let v5: u64 = {
        let a1721 = f91(v1);
        let a1722 = f91(v2);
        nat_add(a1721, a1722)?
    };
    let a1723 = v1;
    let a1724 = 4294967295u64;
    if if nat_le(a1723, a1724) {
        let a1725 = v2;
        let a1726 = 4294967295u64;
        nat_le(a1725, a1726)
    } else {
        false
    } {
        let a1727 = v5;
        let a1728 = v4;
        if nat_le(a1727, a1728) {
            let a1731 = v3;
            let a1732 = {
                let a1729 = v4;
                let a1730 = v5;
                nat_sub(a1729, a1730)
            };
            Ok(nat_eq(a1731, a1732))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f64(v0: u64, v1: u64) -> R<bool> {
    let a1733 = v1;
    let a1734 = 4294967295u64;
    if nat_le(a1733, a1734) {
        let a1737 = {
            let a1735 = f90(v1);
            let a1736 = f91(v1);
            nat_add(a1735, a1736)?
        };
        let a1738 = v0;
        Ok(nat_lt(a1737, a1738))
    } else {
        Ok(false)
    }
}

pub fn f65(v0: Adt2) -> R<bool> {
    let a1740 = match v0.clone() {
        Adt2::C0(_, h1739, _, _, _, _, _, _, _, _, _) => {
            h1739
        }
    };
    let a1741 = 1u64;
    if if if if nat_eq(a1740, a1741) {
        true
    } else {
        let a1743 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1742, _, _, _, _, _) => {
                h1742
            }
        };
        let a1744 = 4u64;
        nat_eq(a1743, a1744)
    } {
        let a1746 = match v0.clone() {
            Adt2::C0(_, _, h1745, _, _, _, _, _, _, _, _) => {
                h1745
            }
        };
        let a1747 = 1u64;
        if nat_eq(a1746, a1747) {
            true
        } else {
            let a1752 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1748, _, _, _, _) => {
                    h1748
                }
            };
            let a1753 = {
                let a1750 = match v0.clone() {
                    Adt2::C0(_, h1749, _, _, _, _, _, _, _, _, _) => {
                        h1749
                    }
                };
                let a1751 = 4u64;
                nat_mul(a1750, a1751)?
            };
            nat_eq(a1752, a1753)
        }
    } else {
        false
    } {
        let a1755 = match v0.clone() {
            Adt2::C0(_, _, _, h1754, _, _, _, _, _, _, _) => {
                h1754
            }
        };
        let a1756 = 1u64;
        if nat_eq(a1755, a1756) {
            true
        } else {
            let a1764 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1757, _, _, _) => {
                    h1757
                }
            };
            let a1765 = {
                let a1762 = {
                    let a1760 = match v0.clone() {
                        Adt2::C0(_, h1758, _, _, _, _, _, _, _, _, _) => {
                            h1758
                        }
                    };
                    let a1761 = match v0.clone() {
                        Adt2::C0(_, _, h1759, _, _, _, _, _, _, _, _) => {
                            h1759
                        }
                    };
                    nat_mul(a1760, a1761)?
                };
                let a1763 = 4u64;
                nat_mul(a1762, a1763)?
            };
            nat_eq(a1764, a1765)
        }
    } else {
        false
    } {
        let a1767 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1766, _, _, _, _, _, _) => {
                h1766
            }
        };
        let a1768 = 1u64;
        if nat_eq(a1767, a1768) {
            Ok(true)
        } else {
            let a1779 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1769, _, _) => {
                    h1769
                }
            };
            let a1780 = {
                let a1777 = {
                    let a1775 = {
                        let a1772 = match v0.clone() {
                            Adt2::C0(_, h1770, _, _, _, _, _, _, _, _, _) => {
                                h1770
                            }
                        };
                        let a1773 = match v0.clone() {
                            Adt2::C0(_, _, h1771, _, _, _, _, _, _, _, _) => {
                                h1771
                            }
                        };
                        nat_mul(a1772, a1773)?
                    };
                    let a1776 = match v0 {
                        Adt2::C0(_, _, _, h1774, _, _, _, _, _, _, _) => {
                            h1774
                        }
                    };
                    nat_mul(a1775, a1776)?
                };
                let a1778 = 4u64;
                nat_mul(a1777, a1778)?
            };
            Ok(nat_eq(a1779, a1780))
        }
    } else {
        Ok(false)
    }
}

pub fn f66(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f92(v0.clone(), false)?;
    let v6: u64 = f92(v0, true)?;
    let a1781 = v6;
    let a1782 = v1;
    if if nat_le(a1781, a1782) {
        let a1789 = {
            let a1787 = {
                let a1783 = v1;
                let a1784 = v5;
                nat_add(a1783, a1784)?
            };
            let a1788 = {
                let a1785 = v3;
                let a1786 = 0u64;
                if nat_eq(a1785, a1786) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1787, a1788)?
        };
        let a1790 = v2;
        nat_le(a1789, a1790)
    } else {
        false
    } {
        if v4 {
            let a1795 = {
                let a1793 = {
                    let a1791 = v1;
                    let a1792 = v5;
                    nat_add(a1791, a1792)?
                };
                let a1794 = v6;
                nat_sub(a1793, a1794)
            };
            let a1796 = v2;
            Ok(nat_lt(a1795, a1796))
        } else {
            let a1801 = {
                let a1799 = {
                    let a1797 = v1;
                    let a1798 = v5;
                    nat_add(a1797, a1798)?
                };
                let a1800 = v6;
                nat_sub(a1799, a1800)
            };
            let a1802 = v2;
            Ok(nat_le(a1801, a1802))
        }
    } else {
        Ok(false)
    }
}

pub fn f67(v0: u64, v1: u64) -> bool {
    let a1803 = v0;
    let a1804 = 0u64;
    if if nat_eq(a1803, a1804) {
        true
    } else {
        let a1805 = v0;
        let a1806 = 1u64;
        nat_eq(a1805, a1806)
    } {
        let a1807 = v1;
        let a1808 = 4294967296u64;
        nat_lt(a1807, a1808)
    } else {
        false
    }
}

pub fn f68(v0: u64, v1: u64) -> R<u64> {
    let a1813 = {
        let a1811 = v0;
        let a1812 = {
            let a1809 = v1;
            let a1810 = 1u64;
            nat_sub(a1809, a1810)
        };
        nat_add(a1811, a1812)?
    };
    let a1814 = v1;
    let a1815 = 0u64;
    Ok(nat_quot(a1813, a1814, a1815))
}

pub fn f69(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f57(match v0.clone() {
        Adt2::C0(_, h1816, _, _, _, _, _, _, _, _, _) => {
            h1816
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1817, _, _, _, _, _, _, _, _) => {
            h1817
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1818, _, _, _, _, _, _, _) => {
            h1818
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1819, _, _, _, _, _, _) => {
            h1819
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1822 = match v1.clone() {
                Adt2::C0(_, h1820, _, _, _, _, _, _, _, _, _) => {
                    h1820
                }
            };
            let a1823 = match v1.clone() {
                Adt2::C0(_, _, h1821, _, _, _, _, _, _, _, _) => {
                    h1821
                }
            };
            nat_mul(a1822, a1823)?
        };
        let v5: u64 = {
            let a1825 = v4;
            let a1826 = match v1.clone() {
                Adt2::C0(_, _, _, h1824, _, _, _, _, _, _, _) => {
                    h1824
                }
            };
            nat_mul(a1825, a1826)?
        };
        let a1839 = {
            let a1837 = {
                let a1828 = match v0.clone() {
                    Adt2::C0(_, h1827, _, _, _, _, _, _, _, _, _) => {
                        h1827
                    }
                };
                let a1829 = 1u64;
                nat_sub(a1828, a1829)
            };
            let a1838 = {
                let a1835 = f93(match v0.clone() {
                    Adt2::C0(_, _, h1830, _, _, _, _, _, _, _, _) => {
                        h1830
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1831, _, _, _, _, _, _, _, _, _) => {
                        h1831
                    }
                })?;
                let a1836 = f93(match v0 {
                    Adt2::C0(_, _, _, h1832, _, _, _, _, _, _, _) => {
                        h1832
                    }
                }, v2, {
                    let a1833 = v3;
                    let a1834 = v5;
                    nat_mul(a1833, a1834)?
                }, v4)?;
                nat_add(a1835, a1836)?
            };
            nat_add(a1837, a1838)?
        };
        let a1840 = f56(v1, 1u64)?;
        Ok(nat_lt(a1839, a1840))
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1841 = v0;
    let a1842 = 2147483648u64;
    if if if if nat_lt(a1841, a1842) {
        let a1843 = v1;
        let a1844 = 2147483648u64;
        nat_lt(a1843, a1844)
    } else {
        false
    } {
        let a1847 = {
            let a1845 = v0;
            let a1846 = v1;
            nat_mul(a1845, a1846)?
        };
        let a1848 = 2147483648u64;
        nat_lt(a1847, a1848)
    } else {
        false
    } {
        let a1853 = {
            let a1851 = {
                let a1849 = v0;
                let a1850 = 1u64;
                nat_sub(a1849, a1850)
            };
            let a1852 = v2;
            nat_mul(a1851, a1852)?
        };
        let a1854 = 2147483648u64;
        nat_lt(a1853, a1854)
    } else {
        false
    } {
        let a1859 = {
            let a1857 = {
                let a1855 = v1;
                let a1856 = 1u64;
                nat_sub(a1855, a1856)
            };
            let a1858 = v2;
            nat_mul(a1857, a1858)?
        };
        let a1860 = 2147483648u64;
        Ok(nat_lt(a1859, a1860))
    } else {
        Ok(false)
    }
}

pub fn f71(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1861 = v1;
    let a1862 = 13u64;
    if if nat_eq(a1861, a1862) {
        f57(match v0.clone() {
            Adt3::C0(h1863, _, _, _, _, _, _, _) => {
                h1863
            }
        }, match v0.clone() {
            Adt3::C0(_, h1864, _, _, _, _, _, _) => {
                h1864
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1865, _, _, _, _, _) => {
                h1865
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1866, _, _, _, _) => {
                h1866
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1872 = match v0.clone() {
                Adt3::C0(_, h1867, _, _, _, _, _, _) => {
                    h1867
                }
            };
            let a1873 = {
                let a1870 = match v0.clone() {
                    Adt3::C0(_, _, h1868, _, _, _, _, _) => {
                        h1868
                    }
                };
                let a1871 = match v0.clone() {
                    Adt3::C0(_, _, _, h1869, _, _, _, _) => {
                        h1869
                    }
                };
                nat_mul(a1870, a1871)?
            };
            nat_mul(a1872, a1873)?
        };
        let a1874 = v1;
        let a1875 = 13u64;
        if nat_eq(a1874, a1875) {
            let a1884 = 2u64;
            let a1885 = if f94(v0.clone(), v2) {
                let a1877 = v3;
                let a1878 = f85(match v0 {
                    Adt3::C0(h1876, _, _, _, _, _, _, _) => {
                        h1876
                    }
                }, false)?;
                nat_mul(a1877, a1878)?
            } else {
                let a1882 = v3;
                let a1883 = {
                    let a1880 = match v0 {
                        Adt3::C0(h1879, _, _, _, _, _, _, _) => {
                            h1879
                        }
                    };
                    let a1881 = 2u64;
                    nat_mul(a1880, a1881)?
                };
                nat_mul(a1882, a1883)?
            };
            nat_add(a1884, a1885)
        } else {
            let a1889 = v3;
            let a1890 = f85(match v0 {
                Adt3::C0(h1886, _, _, _, _, _, _, _) => {
                    h1886
                }
            }, {
                let a1887 = v1;
                let a1888 = 16u64;
                nat_eq(a1887, a1888)
            })?;
            nat_mul(a1889, a1890)
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1891 = v3;
    match m1891 {
        None => {
            f95(v0, v1, match v2.clone() {
                Adt2::C0(h1892, _, _, _, _, _, _, _, _, _, _) => {
                    h1892
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1893, _, _, _, _, _, _, _, _, _, _) => {
                    h1893
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1894, _, _, _, _, _, _, _, _, _) => {
                    h1894
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1895, _, _, _, _, _, _, _, _) => {
                    h1895
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1896, _, _, _, _, _, _, _) => {
                    h1896
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1897, _, _, _, _, _, _) => {
                    h1897
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1898, _, _, _, _, _, _, _, _, _) => {
                    h1898
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1899, _, _, _, _, _, _, _, _) => {
                    h1899
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1900, _, _, _, _, _, _, _) => {
                    h1900
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1901, _, _, _, _, _, _) => {
                    h1901
                }
            })
        }
        Some(v5) => {
            f95(v0, v1, match v2.clone() {
                Adt2::C0(h1902, _, _, _, _, _, _, _, _, _, _) => {
                    h1902
                }
            }, match v5.clone() {
                Adt2::C0(h1903, _, _, _, _, _, _, _, _, _, _) => {
                    h1903
                }
            }, match v4.clone() {
                Adt2::C0(h1904, _, _, _, _, _, _, _, _, _, _) => {
                    h1904
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1905, _, _, _, _, _, _, _, _, _) => {
                    h1905
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1906, _, _, _, _, _, _, _, _) => {
                    h1906
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1907, _, _, _, _, _, _, _) => {
                    h1907
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1908, _, _, _, _, _, _) => {
                    h1908
                }
            }, match v5.clone() {
                Adt2::C0(_, h1909, _, _, _, _, _, _, _, _, _) => {
                    h1909
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1910, _, _, _, _, _, _, _, _) => {
                    h1910
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1911, _, _, _, _, _, _, _) => {
                    h1911
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1912, _, _, _, _, _, _) => {
                    h1912
                }
            }, match v4.clone() {
                Adt2::C0(_, h1913, _, _, _, _, _, _, _, _, _) => {
                    h1913
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1914, _, _, _, _, _, _, _, _) => {
                    h1914
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1915, _, _, _, _, _, _, _) => {
                    h1915
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1916, _, _, _, _, _, _) => {
                    h1916
                }
            })
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1929 = {
        let a1925 = {
            let a1921 = {
                let a1917 = v0;
                let a1918 = v4;
                nat_mul(a1917, a1918)?
            };
            let a1922 = {
                let a1919 = v1;
                let a1920 = v5;
                nat_mul(a1919, a1920)?
            };
            nat_add(a1921, a1922)?
        };
        let a1926 = {
            let a1923 = v2;
            let a1924 = v6;
            nat_mul(a1923, a1924)?
        };
        nat_add(a1925, a1926)?
    };
    let a1930 = {
        let a1927 = v3;
        let a1928 = v7;
        nat_mul(a1927, a1928)?
    };
    nat_add(a1929, a1930)
}

pub fn f74(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1935 = {
        let a1933 = {
            let a1931 = v0;
            let a1932 = v1;
            nat_sub(a1931, a1932)
        };
        let a1934 = 1u64;
        nat_sub(a1933, a1934)
    };
    let a1936 = v2;
    nat_add(a1935, a1936)
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f73({
        let a1937 = v0;
        let a1938 = v4;
        let a1939 = 0u64;
        nat_rem(a1937, a1938, a1939)
    }, {
        let a1940 = v1;
        let a1941 = v5;
        let a1942 = 0u64;
        nat_rem(a1940, a1941, a1942)
    }, {
        let a1943 = v2;
        let a1944 = v6;
        let a1945 = 0u64;
        nat_rem(a1943, a1944, a1945)
    }, {
        let a1946 = v3;
        let a1947 = v7;
        let a1948 = 0u64;
        nat_rem(a1946, a1947, a1948)
    }, v8, v9, v10, v11)
}

pub fn f76(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1950 = v0;
        let a1951 = match v1.clone() {
            Adt3::C0(h1949, _, _, _, _, _, _, _) => {
                h1949
            }
        };
        let a1952 = 0u64;
        nat_quot(a1950, a1951, a1952)
    };
    let v4: u64 = {
        let a1954 = v3;
        let a1955 = match v1.clone() {
            Adt3::C0(_, h1953, _, _, _, _, _, _) => {
                h1953
            }
        };
        let a1956 = 0u64;
        nat_quot(a1954, a1955, a1956)
    };
    let a1957 = v2;
    let a1958 = 0u64;
    if nat_eq(a1957, a1958) {
        let a1960 = v0;
        let a1961 = match v1 {
            Adt3::C0(h1959, _, _, _, _, _, _, _) => {
                h1959
            }
        };
        let a1962 = 0u64;
        nat_rem(a1960, a1961, a1962)
    } else {
        let a1963 = v2;
        let a1964 = 1u64;
        if nat_eq(a1963, a1964) {
            let a1966 = v3;
            let a1967 = match v1 {
                Adt3::C0(_, h1965, _, _, _, _, _, _) => {
                    h1965
                }
            };
            let a1968 = 0u64;
            nat_rem(a1966, a1967, a1968)
        } else {
            let a1969 = v2;
            let a1970 = 2u64;
            if nat_eq(a1969, a1970) {
                let a1972 = v4;
                let a1973 = match v1 {
                    Adt3::C0(_, _, h1971, _, _, _, _, _) => {
                        h1971
                    }
                };
                let a1974 = 0u64;
                nat_rem(a1972, a1973, a1974)
            } else {
                let a1976 = v4;
                let a1977 = match v1 {
                    Adt3::C0(_, _, h1975, _, _, _, _, _) => {
                        h1975
                    }
                };
                let a1978 = 0u64;
                nat_quot(a1976, a1977, a1978)
            }
        }
    }
}

pub fn f77(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f73(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1979, _, _, _) => {
            h1979
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1980, _, _) => {
            h1980
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1981, _) => {
            h1981
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1982) => {
            h1982
        }
    })
}

pub fn f78(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1983 = v1;
    let a1984 = 2147483648u64;
    if nat_lt(a1983, a1984) {
        let a1995 = {
            let a1993 = {
                let a1988 = {
                    let a1985 = v0;
                    let a1986 = v2;
                    let a1987 = 0u64;
                    nat_rem(a1985, a1986, a1987)
                };
                let a1989 = v2;
                nat_add(a1988, a1989)?
            };
            let a1994 = {
                let a1990 = v1;
                let a1991 = v2;
                let a1992 = 0u64;
                nat_rem(a1990, a1991, a1992)
            };
            nat_sub(a1993, a1994)
        };
        let a1996 = v2;
        let a1997 = 0u64;
        Ok(nat_rem(a1995, a1996, a1997))
    } else {
        let a2000 = {
            let a1998 = v0;
            let a1999 = f91(v1);
            nat_add(a1998, a1999)?
        };
        let a2001 = v2;
        let a2002 = 0u64;
        Ok(nat_rem(a2000, a2001, a2002))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2005 = f90(v1);
    let a2006 = {
        let a2003 = v0;
        let a2004 = f91(v1);
        nat_add(a2003, a2004)?
    };
    if nat_le(a2005, a2006) {
        let a2007 = f80(v0, v1)?;
        let a2008 = v2;
        Ok(nat_lt(a2007, a2008))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64, v1: u64) -> R<u64> {
    let a2011 = {
        let a2009 = v0;
        let a2010 = f91(v1);
        nat_add(a2009, a2010)?
    };
    let a2012 = f90(v1);
    Ok(nat_sub(a2011, a2012))
}

pub fn f81(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2013 = v0;
    let a2014 = v1;
    if nat_lt(a2013, a2014) {
        let a2015 = v1;
        let a2016 = v0;
        Ok(nat_sub(a2015, a2016))
    } else {
        let a2019 = v0;
        let a2020 = {
            let a2017 = v1;
            let a2018 = v2;
            nat_add(a2017, a2018)?
        };
        if nat_lt(a2019, a2020) {
            let a2021 = v0;
            let a2022 = v1;
            Ok(nat_sub(a2021, a2022))
        } else {
            let a2029 = {
                let a2025 = 2u64;
                let a2026 = {
                    let a2023 = v2;
                    let a2024 = 1u64;
                    nat_sub(a2023, a2024)
                };
                nat_mul(a2025, a2026)?
            };
            let a2030 = {
                let a2027 = v0;
                let a2028 = v1;
                nat_sub(a2027, a2028)
            };
            Ok(nat_sub(a2029, a2030))
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2033 = {
        let a2031 = v2;
        let a2032 = f96(v1.clone(), f76(v0, v1.clone(), 0u64), f76(v0, v1.clone(), 1u64), f76(v0, v1.clone(), 2u64), f76(v0, v1.clone(), 3u64), false)?;
        nat_add(a2031, a2032)?
    };
    let a2034 = f96(v1.clone(), f76(v0, v1.clone(), 0u64), f76(v0, v1.clone(), 1u64), f76(v0, v1.clone(), 2u64), f76(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2033, a2034))
}

pub fn f83(v0: u64, v1: u64, v2: u64) -> bool {
    let a2035 = v0;
    let a2036 = 0u64;
    if nat_eq(a2035, a2036) {
        let a2037 = v2;
        let a2038 = v1;
        nat_le(a2037, a2038)
    } else {
        let a2039 = v0;
        let a2040 = 1u64;
        if nat_eq(a2039, a2040) {
            let a2041 = v2;
            let a2042 = v1;
            nat_lt(a2041, a2042)
        } else {
            let a2043 = v0;
            let a2044 = 2u64;
            if nat_eq(a2043, a2044) {
                let a2045 = v1;
                let a2046 = v2;
                nat_le(a2045, a2046)
            } else {
                let a2047 = v0;
                let a2048 = 3u64;
                if nat_eq(a2047, a2048) {
                    let a2049 = v1;
                    let a2050 = v2;
                    nat_lt(a2049, a2050)
                } else {
                    let a2051 = v1;
                    let a2052 = v2;
                    nat_eq(a2051, a2052)
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2053 = v0;
    let a2054 = 0u64;
    if nat_eq(a2053, a2054) {
        Ok(f97(0u64))
    } else {
        let a2057 = {
            let a2055 = v0;
            let a2056 = 1u64;
            nat_add(a2055, a2056)?
        };
        let a2058 = f71(v1.clone(), 13u64, v2)?;
        if nat_eq(a2057, a2058) {
            Ok(Adt5::C4({
                let a2059 = v2;
                let a2060 = 0u64;
                if nat_eq(a2059, a2060) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2061 = v0;
                let a2062 = 1u64;
                nat_sub(a2061, a2062)
            };
            if f94(v1.clone(), v2) {
                let v4: u64 = f85(match v1.clone() {
                    Adt3::C0(h2063, _, _, _, _, _, _, _) => {
                        h2063
                    }
                }, false)?;
                let v5: u64 = {
                    let a2064 = v3;
                    let a2065 = v4;
                    let a2066 = 0u64;
                    nat_quot(a2064, a2065, a2066)
                };
                let v6: u64 = {
                    let a2067 = v3;
                    let a2068 = v4;
                    let a2069 = 0u64;
                    nat_rem(a2067, a2068, a2069)
                };
                let a2072 = {
                    let a2070 = v6;
                    let a2071 = 1u64;
                    nat_add(a2070, a2071)?
                };
                let a2073 = v4;
                if nat_eq(a2072, a2073) {
                    Ok(Adt5::C4({
                        let a2074 = v2;
                        let a2075 = 0u64;
                        if nat_eq(a2074, a2075) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f86(v6, v5, v1, v2)
                }
            } else {
                f98({
                    let a2076 = v3;
                    let a2077 = 2u64;
                    let a2078 = 0u64;
                    nat_quot(a2076, a2077, a2078)
                }, {
                    let a2079 = v3;
                    let a2080 = 2u64;
                    let a2081 = 0u64;
                    nat_rem(a2079, a2080, a2081)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f85(v0: u64, v1: bool) -> R<u64> {
    let a2084 = {
        let a2082 = v0;
        let a2083 = 2u64;
        nat_mul(a2082, a2083)?
    };
    let a2085 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2084, a2085)
}

pub fn f86(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2086 = v0;
    let a2087 = 0u64;
    if nat_eq(a2086, a2087) {
        Ok(f97(1u64))
    } else {
        f98({
            let a2096 = {
                let a2089 = v1;
                let a2090 = match v2.clone() {
                    Adt3::C0(h2088, _, _, _, _, _, _, _) => {
                        h2088
                    }
                };
                nat_mul(a2089, a2090)?
            };
            let a2097 = {
                let a2093 = {
                    let a2091 = v0;
                    let a2092 = 1u64;
                    nat_sub(a2091, a2092)
                };
                let a2094 = 2u64;
                let a2095 = 0u64;
                nat_quot(a2093, a2094, a2095)
            };
            nat_add(a2096, a2097)?
        }, {
            let a2100 = {
                let a2098 = v0;
                let a2099 = 1u64;
                nat_sub(a2098, a2099)
            };
            let a2101 = 2u64;
            let a2102 = 0u64;
            nat_rem(a2100, a2101, a2102)
        }, v2, v3, 1u64)
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2103 = f60(v0);
    match m2103 {
        Adt7::C0 => {
            Ok(None::<u64>)
        }
        Adt7::C1(v13) => {
            if if if if if f99(v0) {
                let a2104 = v1;
                let a2105 = v7;
                nat_lt(a2104, a2105)
            } else {
                false
            } {
                f51(v8, v2)
            } else {
                false
            } {
                let a2106 = v3;
                let a2107 = f100(v4, v5, v6)?;
                nat_lt(a2106, a2107)
            } else {
                false
            } {
                let a2112 = {
                    let a2109 = v7;
                    let a2110 = match v13.clone() {
                        Adt8::C0(_, h2108, _) => {
                            h2108
                        }
                    };
                    let a2111 = 0u64;
                    nat_rem(a2109, a2110, a2111)
                };
                let a2113 = 0u64;
                nat_eq(a2112, a2113)
            } else {
                false
            } {
                Ok(Some({
                    let a2142 = {
                        let a2132 = {
                            let a2122 = {
                                let a2118 = {
                                    let a2115 = v1;
                                    let a2116 = match v13 {
                                        Adt8::C0(_, h2114, _) => {
                                            h2114
                                        }
                                    };
                                    let a2117 = 0u64;
                                    nat_quot(a2115, a2116, a2117)
                                };
                                let a2119 = v9;
                                nat_mul(a2118, a2119)?
                            };
                            let a2123 = {
                                let a2120 = v2;
                                let a2121 = v10;
                                nat_mul(a2120, a2121)?
                            };
                            nat_add(a2122, a2123)?
                        };
                        let a2133 = {
                            let a2130 = {
                                let a2127 = {
                                    let a2124 = v3;
                                    let a2125 = v4;
                                    let a2126 = 0u64;
                                    nat_quot(a2124, a2125, a2126)
                                };
                                let a2128 = v5;
                                let a2129 = 0u64;
                                nat_rem(a2127, a2128, a2129)
                            };
                            let a2131 = v11;
                            nat_mul(a2130, a2131)?
                        };
                        nat_add(a2132, a2133)?
                    };
                    let a2143 = {
                        let a2140 = {
                            let a2137 = {
                                let a2134 = v3;
                                let a2135 = v4;
                                let a2136 = 0u64;
                                nat_quot(a2134, a2135, a2136)
                            };
                            let a2138 = v5;
                            let a2139 = 0u64;
                            nat_quot(a2137, a2138, a2139)
                        };
                        let a2141 = v12;
                        nat_mul(a2140, a2141)?
                    };
                    nat_add(a2142, a2143)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f88(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f101(v0, v1, v6, v7, v8) {
        let a2144 = v5;
        let a2145 = 26u64;
        nat_eq(a2144, a2145)
    } else {
        false
    } {
        let a2146 = v3;
        let a2147 = v7;
        nat_eq(a2146, a2147)
    } else {
        false
    } {
        let a2148 = v4;
        let a2149 = v8;
        nat_eq(a2148, a2149)
    } else {
        false
    } {
        let a2150 = v9;
        let a2151 = 1u64;
        nat_eq(a2150, a2151)
    } else {
        false
    } {
        let a2152 = v10;
        let a2153 = f102(v0);
        nat_eq(a2152, a2153)
    } else {
        false
    } {
        let a2154 = v11;
        let a2155 = v1;
        nat_eq(a2154, a2155)
    } else {
        false
    } {
        let a2156 = v12;
        let a2157 = v6;
        nat_eq(a2156, a2157)
    } else {
        false
    } {
        let a2158 = v13;
        let a2159 = v7;
        nat_eq(a2158, a2159)
    } else {
        false
    } {
        let a2160 = v14;
        let a2161 = v8;
        nat_eq(a2160, a2161)
    } else {
        false
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2162 = v0;
    let a2163 = 0u64;
    if if if nat_eq(a2162, a2163) {
        true
    } else {
        let a2164 = v1;
        let a2165 = 0u64;
        nat_eq(a2164, a2165)
    } {
        true
    } else {
        let a2166 = v2;
        let a2167 = 0u64;
        nat_eq(a2166, a2167)
    } {
        true
    } else {
        let a2168 = v3;
        let a2169 = 0u64;
        nat_eq(a2168, a2169)
    }
}

pub fn f90(v0: u64) -> u64 {
    let a2170 = v0;
    let a2171 = 2147483648u64;
    if nat_lt(a2170, a2171) {
        v0
    } else {
        0u64
    }
}

pub fn f91(v0: u64) -> u64 {
    let a2172 = v0;
    let a2173 = 2147483648u64;
    if nat_lt(a2172, a2173) {
        0u64
    } else {
        let a2174 = 4294967296u64;
        let a2175 = v0;
        nat_sub(a2174, a2175)
    }
}

pub fn f92(v0: Adt3, v1: bool) -> R<u64> {
    f96(v0.clone(), {
        let a2177 = match v0.clone() {
            Adt3::C0(h2176, _, _, _, _, _, _, _) => {
                h2176
            }
        };
        let a2178 = 1u64;
        nat_sub(a2177, a2178)
    }, {
        let a2180 = match v0.clone() {
            Adt3::C0(_, h2179, _, _, _, _, _, _) => {
                h2179
            }
        };
        let a2181 = 1u64;
        nat_sub(a2180, a2181)
    }, {
        let a2183 = match v0.clone() {
            Adt3::C0(_, _, h2182, _, _, _, _, _) => {
                h2182
            }
        };
        let a2184 = 1u64;
        nat_sub(a2183, a2184)
    }, {
        let a2186 = match v0 {
            Adt3::C0(_, _, _, h2185, _, _, _, _) => {
                h2185
            }
        };
        let a2187 = 1u64;
        nat_sub(a2186, a2187)
    }, v1)
}

pub fn f93(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2188 = v0;
    let a2189 = 0u64;
    if nat_eq(a2188, a2189) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2190 = v0;
            let a2191 = 1u64;
            nat_sub(a2190, a2191)
        };
        let v5: u64 = {
            let a2192 = v4;
            let a2193 = v1;
            let a2194 = 0u64;
            nat_quot(a2192, a2193, a2194)
        };
        let v6: u64 = {
            let a2202 = {
                let a2195 = v5;
                let a2196 = v2;
                nat_mul(a2195, a2196)?
            };
            let a2203 = {
                let a2200 = {
                    let a2197 = v4;
                    let a2198 = v1;
                    let a2199 = 0u64;
                    nat_rem(a2197, a2198, a2199)
                };
                let a2201 = v3;
                nat_mul(a2200, a2201)?
            };
            nat_add(a2202, a2203)?
        };
        let a2204 = v5;
        let a2205 = 0u64;
        if nat_eq(a2204, a2205) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2214 = {
                    let a2208 = {
                        let a2206 = v5;
                        let a2207 = 1u64;
                        nat_sub(a2206, a2207)
                    };
                    let a2209 = v2;
                    nat_mul(a2208, a2209)?
                };
                let a2215 = {
                    let a2212 = {
                        let a2210 = v1;
                        let a2211 = 1u64;
                        nat_sub(a2210, a2211)
                    };
                    let a2213 = v3;
                    nat_mul(a2212, a2213)?
                };
                nat_add(a2214, a2215)?
            };
            let a2216 = v6;
            let a2217 = v7;
            if nat_lt(a2216, a2217) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f94(v0: Adt3, v1: u64) -> bool {
    let a2218 = v1;
    let a2219 = 0u64;
    if !nat_eq(a2218, a2219) {
        true
    } else {
        let a2221 = match v0 {
            Adt3::C0(_, _, _, _, h2220, _, _, _) => {
                h2220
            }
        };
        let a2222 = 4u64;
        nat_eq(a2221, a2222)
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2223 = f103(v0, v1);
    match m2223 {
        None => {
            if v5 {
                let m2224 = f104(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2224 {
                    Adt9::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt9::C1 => {
                        Ok(Adt6::C1)
                    }
                    Adt9::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt6::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a2225 = 4u64;
                let a2226 = v28;
                nat_le(a2225, a2226)
            };
            let a2227 = v2;
            let a2228 = 0u64;
            if if if if if nat_eq(a2227, a2228) {
                let a2229 = v4;
                let a2230 = 0u64;
                nat_eq(a2229, a2230)
            } else {
                false
            } {
                let a2231 = v6;
                let a2232 = v14;
                if nat_eq(a2231, a2232) {
                    let a2233 = v7;
                    let a2234 = v15;
                    nat_eq(a2233, a2234)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2235 = v8;
                let a2236 = v16;
                if nat_eq(a2235, a2236) {
                    let a2237 = v9;
                    let a2238 = v17;
                    nat_eq(a2237, a2238)
                } else {
                    false
                }
            } else {
                false
            } {
                if v29 {
                    true
                } else {
                    if if v5 {
                        let a2239 = v3;
                        let a2240 = 0u64;
                        nat_eq(a2239, a2240)
                    } else {
                        false
                    } {
                        f61(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2245 = {
                        let a2241 = v6;
                        let a2242 = v7;
                        nat_mul(a2241, a2242)?
                    };
                    let a2246 = {
                        let a2243 = v8;
                        let a2244 = v9;
                        nat_mul(a2243, a2244)?
                    };
                    nat_mul(a2245, a2246)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f96(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2267 = {
        let a2261 = {
            let a2255 = {
                let a2249 = v1;
                let a2250 = if v5 {
                    f91(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2247, _, _, _) => {
                            h2247
                        }
                    })
                } else {
                    f90(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2248, _, _, _) => {
                            h2248
                        }
                    })
                };
                nat_mul(a2249, a2250)?
            };
            let a2256 = {
                let a2253 = v2;
                let a2254 = if v5 {
                    f91(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2251, _, _) => {
                            h2251
                        }
                    })
                } else {
                    f90(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2252, _, _) => {
                            h2252
                        }
                    })
                };
                nat_mul(a2253, a2254)?
            };
            nat_add(a2255, a2256)?
        };
        let a2262 = {
            let a2259 = v3;
            let a2260 = if v5 {
                f91(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2257, _) => {
                        h2257
                    }
                })
            } else {
                f90(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2258, _) => {
                        h2258
                    }
                })
            };
            nat_mul(a2259, a2260)?
        };
        nat_add(a2261, a2262)?
    };
    let a2268 = {
        let a2265 = v4;
        let a2266 = if v5 {
            f91(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2263) => {
                    h2263
                }
            })
        } else {
            f90(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2264) => {
                    h2264
                }
            })
        };
        nat_mul(a2265, a2266)?
    };
    nat_add(a2267, a2268)
}

pub fn f97(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f98(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2269 = v1;
    let a2270 = 0u64;
    if nat_eq(a2269, a2270) {
        Ok(Adt5::C4({
            let a2271 = v3;
            let a2272 = 0u64;
            if nat_eq(a2271, a2272) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f33(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2273 = v3;
            let a2274 = 0u64;
            if nat_eq(a2273, a2274) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f99(v0: u64) -> bool {
    let a2275 = v0;
    let a2276 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2275, a2276) {
        true
    } else {
        let a2277 = v0;
        let a2278 = 1u64;
        nat_eq(a2277, a2278)
    } {
        true
    } else {
        let a2279 = v0;
        let a2280 = 2u64;
        nat_eq(a2279, a2280)
    } {
        true
    } else {
        let a2281 = v0;
        let a2282 = 3u64;
        nat_eq(a2281, a2282)
    } {
        true
    } else {
        let a2283 = v0;
        let a2284 = 6u64;
        nat_eq(a2283, a2284)
    } {
        true
    } else {
        let a2285 = v0;
        let a2286 = 7u64;
        nat_eq(a2285, a2286)
    } {
        true
    } else {
        let a2287 = v0;
        let a2288 = 8u64;
        nat_eq(a2287, a2288)
    } {
        true
    } else {
        let a2289 = v0;
        let a2290 = 10u64;
        nat_eq(a2289, a2290)
    } {
        true
    } else {
        let a2291 = v0;
        let a2292 = 11u64;
        nat_eq(a2291, a2292)
    } {
        true
    } else {
        let a2293 = v0;
        let a2294 = 12u64;
        nat_eq(a2293, a2294)
    } {
        true
    } else {
        let a2295 = v0;
        let a2296 = 13u64;
        nat_eq(a2295, a2296)
    } {
        true
    } else {
        let a2297 = v0;
        let a2298 = 14u64;
        nat_eq(a2297, a2298)
    } {
        true
    } else {
        let a2299 = v0;
        let a2300 = 16u64;
        nat_eq(a2299, a2300)
    } {
        true
    } else {
        let a2301 = v0;
        let a2302 = 17u64;
        nat_eq(a2301, a2302)
    } {
        true
    } else {
        let a2303 = v0;
        let a2304 = 18u64;
        nat_eq(a2303, a2304)
    } {
        true
    } else {
        let a2305 = v0;
        let a2306 = 19u64;
        nat_eq(a2305, a2306)
    } {
        true
    } else {
        let a2307 = v0;
        let a2308 = 20u64;
        nat_eq(a2307, a2308)
    } {
        true
    } else {
        let a2309 = v0;
        let a2310 = 21u64;
        nat_eq(a2309, a2310)
    } {
        true
    } else {
        let a2311 = v0;
        let a2312 = 22u64;
        nat_eq(a2311, a2312)
    } {
        true
    } else {
        let a2313 = v0;
        let a2314 = 23u64;
        nat_eq(a2313, a2314)
    } {
        true
    } else {
        let a2315 = v0;
        let a2316 = 26u64;
        nat_eq(a2315, a2316)
    } {
        true
    } else {
        let a2317 = v0;
        let a2318 = 29u64;
        nat_eq(a2317, a2318)
    } {
        true
    } else {
        let a2319 = v0;
        let a2320 = 30u64;
        nat_eq(a2319, a2320)
    } {
        true
    } else {
        let a2321 = v0;
        let a2322 = 34u64;
        nat_eq(a2321, a2322)
    } {
        true
    } else {
        let a2323 = v0;
        let a2324 = 35u64;
        nat_eq(a2323, a2324)
    } {
        true
    } else {
        let a2325 = v0;
        let a2326 = 39u64;
        nat_eq(a2325, a2326)
    } {
        true
    } else {
        let a2327 = v0;
        let a2328 = 40u64;
        nat_eq(a2327, a2328)
    } {
        true
    } else {
        let a2329 = v0;
        let a2330 = 41u64;
        nat_eq(a2329, a2330)
    } {
        true
    } else {
        let a2331 = v0;
        let a2332 = 42u64;
        nat_eq(a2331, a2332)
    } {
        true
    } else {
        let a2333 = v0;
        let a2334 = 43u64;
        nat_eq(a2333, a2334)
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2335 = v0;
    let a2336 = 0u64;
    if if if nat_eq(a2335, a2336) {
        true
    } else {
        let a2337 = v1;
        let a2338 = 0u64;
        nat_eq(a2337, a2338)
    } {
        true
    } else {
        let a2339 = v2;
        let a2340 = 0u64;
        nat_eq(a2339, a2340)
    } {
        Ok(0u64)
    } else {
        let a2343 = {
            let a2341 = v0;
            let a2342 = v1;
            nat_mul(a2341, a2342)?
        };
        let a2344 = v2;
        nat_mul(a2343, a2344)
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f89(v1, v2, v3, v4) {
        f105(v0)
    } else {
        f99(v0)
    }
}

pub fn f102(v0: u64) -> u64 {
    let a2345 = v0;
    let a2346 = 26u64;
    if nat_eq(a2345, a2346) {
        26u64
    } else {
        0u64
    }
}

pub fn f103(v0: u64, v1: u64) -> Option<u64> {
    let a2347 = v0;
    let a2348 = 2u64;
    if nat_eq(a2347, a2348) {
        Some(0u64)
    } else {
        let a2349 = v0;
        let a2350 = 5u64;
        if nat_eq(a2349, a2350) {
            Some(1u64)
        } else {
            let a2351 = v0;
            let a2352 = 6u64;
            if nat_eq(a2351, a2352) {
                Some(2u64)
            } else {
                let a2353 = v0;
                let a2354 = 7u64;
                if nat_eq(a2353, a2354) {
                    Some(3u64)
                } else {
                    let a2355 = v0;
                    let a2356 = 9u64;
                    if nat_eq(a2355, a2356) {
                        Some(4u64)
                    } else {
                        let a2357 = v0;
                        let a2358 = 8u64;
                        if nat_eq(a2357, a2358) {
                            Some(5u64)
                        } else {
                            let a2359 = v0;
                            let a2360 = 89u64;
                            if nat_eq(a2359, a2360) {
                                let a2361 = v1;
                                let a2362 = 2u64;
                                if nat_eq(a2361, a2362) {
                                    Some(6u64)
                                } else {
                                    let a2363 = v1;
                                    let a2364 = 0u64;
                                    if nat_eq(a2363, a2364) {
                                        Some(7u64)
                                    } else {
                                        None::<u64>
                                    }
                                }
                            } else {
                                None::<u64>
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a2365 = v0;
    let a2366 = 28u64;
    if if if if nat_eq(a2365, a2366) {
        let a2367 = v1;
        let a2368 = 0u64;
        nat_eq(a2367, a2368)
    } else {
        false
    } {
        let a2369 = v2;
        let a2370 = 0u64;
        nat_eq(a2369, a2370)
    } else {
        false
    } {
        let a2371 = v3;
        let a2372 = 0u64;
        nat_eq(a2371, a2372)
    } else {
        false
    } {
        let a2373 = v4;
        let a2374 = v8;
        if if if if if if if if nat_eq(a2373, a2374) {
            let a2375 = v5;
            let a2376 = v12;
            nat_eq(a2375, a2376)
        } else {
            false
        } {
            let a2377 = v9;
            let a2378 = v13;
            nat_eq(a2377, a2378)
        } else {
            false
        } {
            let a2379 = v10;
            let a2380 = v14;
            nat_eq(a2379, a2380)
        } else {
            false
        } {
            let a2381 = v11;
            let a2382 = v15;
            nat_eq(a2381, a2382)
        } else {
            false
        } {
            let a2383 = 0u64;
            let a2384 = v6;
            nat_lt(a2383, a2384)
        } else {
            false
        } {
            let a2385 = 0u64;
            let a2386 = v7;
            nat_lt(a2385, a2386)
        } else {
            false
        } {
            let a2390 = {
                let a2387 = v10;
                let a2388 = v6;
                let a2389 = 1u64;
                nat_rem(a2387, a2388, a2389)
            };
            let a2391 = 0u64;
            if nat_eq(a2390, a2391) {
                let a2395 = {
                    let a2392 = v11;
                    let a2393 = v7;
                    let a2394 = 1u64;
                    nat_rem(a2392, a2393, a2394)
                };
                let a2396 = 0u64;
                nat_eq(a2395, a2396)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a2397 = v10;
                let a2398 = v6;
                let a2399 = 0u64;
                nat_quot(a2397, a2398, a2399)
            }, {
                let a2400 = v11;
                let a2401 = v7;
                let a2402 = 0u64;
                nat_quot(a2400, a2401, a2402)
            }, {
                let a2403 = v5;
                let a2404 = v4;
                nat_mul(a2403, a2404)?
            }, {
                let a2405 = v4;
                let a2406 = v9;
                nat_mul(a2405, a2406)?
            }, {
                let a2407 = v5;
                let a2408 = v9;
                nat_mul(a2407, a2408)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f105(v0: u64) -> bool {
    let m2409 = f60(v0);
    match m2409 {
        Adt7::C0 => {
            false
        }
        Adt7::C1(_) => {
            true
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
