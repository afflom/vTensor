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
    C6(u64, u64, u64, u64, u64, u64),
    C7(u64, Adt3, Adt3, Adt4, u64),
    C8(Adt4),
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
    C23(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
    C6(u64, u64, u64, bool),
    C7(Adt7, Adt8, Adt8, Adt9, u64, u64),
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
    C0(u64),
    C1(u64),
    C2(u64, u64, bool),
    C3(u64, u64, bool),
    C4(u64, u64, u64, u64),
    C5(u64),
    C6,
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64),
    C1(u64),
    C2,
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(Adt11),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, _, _, _, _, _, _, _, v13) => {
            let a2 = v1;
            let a3 = 64u64;
            if nat_eq(a2, a3) {
                Ok(Adt5::C0(f1(v2, v13)?))
            } else {
                let m4 = v3;
                match m4 {
                    None => {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                    Some(v14) => {
                        let a5 = v1;
                        let a6 = 3u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt5::C0(f2(v14, v4, v5, v13)?))
                        } else {
                            f3(v1, v2, v14, v4, v13)
                        }
                    }
                }
            }
        }
        Adt0::C1(v15, v16, v17, v18) => {
            f4(v15, v16, v17, v18)
        }
        Adt0::C2(v19, v20, v21, v22) => {
            f5(v19, v20, v21, v22)
        }
        Adt0::C3(v23, v24, v25, v26, v27) => {
            f6(v23, v24, v25, v26, v27)
        }
        Adt0::C4(v28, v29, v30, v31, v32, v33) => {
            f7(v28, v29, v30, v31, v32, v33)
        }
        Adt0::C5(v34, v35, v36, v37, v38, v39) => {
            f8(v34, v35, v36, v37, v38, v39)
        }
        Adt0::C6(v40, v41, v42, v43, v44, v45) => {
            f9(v40, v41, v42, v43, v44, v45)
        }
        Adt0::C7(v46, v47, v48, v49, v50) => {
            f10(v46, v47, v48, v49, v50)
        }
        Adt0::C8(v51) => {
            Ok(f11(v51))
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
    let v2: u64 = f12(v1.clone(), 1u64)?;
    let a8 = match v1.clone() {
        Adt2::C0(h7, _, _, _, _, _, _, _, _, _, _) => {
            h7
        }
    };
    let a9 = 0u64;
    if if if if if if if if nat_eq(a8, a9) {
        let a11 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h10, _, _, _, _, _) => {
                h10
            }
        };
        let a12 = 4u64;
        nat_eq(a11, a12)
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(h13, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h13
            }
        })
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(_, h14, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h14
            }
        })
    } else {
        false
    } {
        let a16 = 0u64;
        let a17 = match v0.clone() {
            Adt1::C0(_, _, h15, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h15
            }
        };
        nat_lt(a16, a17)
    } else {
        false
    } {
        let a19 = match v0.clone() {
            Adt1::C0(_, _, h18, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h18
            }
        };
        let a20 = 2139095040u64;
        nat_lt(a19, a20)
    } else {
        false
    } {
        let a21 = v2;
        let a22 = 9223372036854775808u64;
        nat_lt(a21, a22)
    } else {
        false
    } {
        let a24 = f14(v2)?;
        let a25 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h23, _) => {
                h23
            }
        };
        nat_le(a24, a25)
    } else {
        false
    } {
        Ok(Adt6::C4(f15(v2)?, 4u64, f16(v1.clone(), 1u64), f16(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h26, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        }, match v0.clone() {
            Adt1::C0(_, h27, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h27
            }
        }, match v0 {
            Adt1::C0(_, _, h28, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h28
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m29 = v1;
    match m29 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m30 = v2;
            match m30 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a32 = match v0.clone() {
                        Adt2::C0(h31, _, _, _, _, _, _, _, _, _, _) => {
                            h31
                        }
                    };
                    let a33 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a32, a33) {
                        let a35 = match v4.clone() {
                            Adt2::C0(h34, _, _, _, _, _, _, _, _, _, _) => {
                                h34
                            }
                        };
                        let a36 = 0u64;
                        nat_eq(a35, a36)
                    } else {
                        false
                    } {
                        let a38 = match v5.clone() {
                            Adt2::C0(h37, _, _, _, _, _, _, _, _, _, _) => {
                                h37
                            }
                        };
                        let a39 = 26u64;
                        nat_eq(a38, a39)
                    } else {
                        false
                    } {
                        let a41 = match v3.clone() {
                            Adt2::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                                h40
                            }
                        };
                        let a42 = 0u64;
                        nat_eq(a41, a42)
                    } else {
                        false
                    } {
                        f17(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a45 = match v0.clone() {
                            Adt2::C0(_, h43, _, _, _, _, _, _, _, _, _) => {
                                h43
                            }
                        };
                        let a46 = match v4.clone() {
                            Adt2::C0(_, h44, _, _, _, _, _, _, _, _, _) => {
                                h44
                            }
                        };
                        nat_eq(a45, a46)
                    } else {
                        false
                    } {
                        let a49 = match v0.clone() {
                            Adt2::C0(_, _, h47, _, _, _, _, _, _, _, _) => {
                                h47
                            }
                        };
                        let a50 = match v5.clone() {
                            Adt2::C0(_, h48, _, _, _, _, _, _, _, _, _) => {
                                h48
                            }
                        };
                        nat_eq(a49, a50)
                    } else {
                        false
                    } {
                        let a53 = match v0.clone() {
                            Adt2::C0(_, _, _, h51, _, _, _, _, _, _, _) => {
                                h51
                            }
                        };
                        let a54 = match v5.clone() {
                            Adt2::C0(_, _, h52, _, _, _, _, _, _, _, _) => {
                                h52
                            }
                        };
                        nat_eq(a53, a54)
                    } else {
                        false
                    } {
                        let a56 = 0u64;
                        let a57 = match v5.clone() {
                            Adt2::C0(_, _, _, h55, _, _, _, _, _, _, _) => {
                                h55
                            }
                        };
                        nat_lt(a56, a57)
                    } else {
                        false
                    } {
                        let a59 = 0u64;
                        let a60 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h58, _, _, _, _, _, _) => {
                                h58
                            }
                        };
                        nat_lt(a59, a60)
                    } else {
                        false
                    } {
                        let a62 = 4u64;
                        let a63 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h61, _, _, _, _, _) => {
                                h61
                            }
                        };
                        nat_le(a62, a63)
                    } else {
                        false
                    } {
                        let a65 = 4u64;
                        let a66 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h64, _, _, _, _, _) => {
                                h64
                            }
                        };
                        nat_le(a65, a66)
                    } else {
                        false
                    } {
                        let a68 = 4u64;
                        let a69 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h67, _, _, _, _, _) => {
                                h67
                            }
                        };
                        nat_le(a68, a69)
                    } else {
                        false
                    } {
                        let a71 = 4u64;
                        let a72 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h70, _, _, _, _, _) => {
                                h70
                            }
                        };
                        nat_le(a71, a72)
                    } else {
                        false
                    } {
                        f18(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f18(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f18(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f19(match v0.clone() {
                            Adt2::C0(_, h73, _, _, _, _, _, _, _, _, _) => {
                                h73
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h74, _, _, _, _, _, _, _, _) => {
                                h74
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h75, _, _, _, _, _, _, _) => {
                                h75
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                                h76
                            }
                        })?, 4u64, f16(v0, 1u64), f16(v3, 1u64), Adt4::C22(f16(v4, 1u64), f16(v5, 1u64))))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt5> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h77, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h77
        }
    };
    Ok(Adt5::C0({
        let a78 = v0;
        let a79 = 39u64;
        if nat_eq(a78, a79) {
            f20(v2, v3, v4)?
        } else {
            let a80 = v0;
            let a81 = 77u64;
            if nat_eq(a80, a81) {
                f21(v2, v4)?
            } else {
                let a82 = v0;
                let a83 = 40u64;
                if nat_eq(a82, a83) {
                    f22(v2, v3, v4)?
                } else {
                    if f23(v0) {
                        f24(v0, v2, {
                            let m84 = v3;
                            match m84 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a85 = v0;
                        let a86 = 19u64;
                        if nat_eq(a85, a86) {
                            f25(v2, v4)?
                        } else {
                            let a87 = v0;
                            let a88 = 21u64;
                            if nat_eq(a87, a88) {
                                f26(v5, v2, v3, v4)?
                            } else {
                                let a89 = v0;
                                let a90 = 61u64;
                                if if if nat_eq(a89, a90) {
                                    true
                                } else {
                                    let a91 = v0;
                                    let a92 = 62u64;
                                    nat_eq(a91, a92)
                                } {
                                    true
                                } else {
                                    let a93 = v0;
                                    let a94 = 63u64;
                                    nat_eq(a93, a94)
                                } {
                                    f27(v0, v1, v2, v4)?
                                } else {
                                    let a95 = v0;
                                    let a96 = 4u64;
                                    if if nat_eq(a95, a96) {
                                        true
                                    } else {
                                        let a97 = v0;
                                        let a98 = 32u64;
                                        nat_eq(a97, a98)
                                    } {
                                        f28(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a99 = v0;
                                        let a100 = 20u64;
                                        if nat_eq(a99, a100) {
                                            f29(v2, v4)?
                                        } else {
                                            let a101 = v0;
                                            let a102 = 42u64;
                                            if nat_eq(a101, a102) {
                                                f30(4u64, v2, v4)?
                                            } else {
                                                let a103 = v0;
                                                let a104 = 69u64;
                                                if nat_eq(a103, a104) {
                                                    let a105 = v5;
                                                    let a106 = 4u64;
                                                    if nat_lt(a105, a106) {
                                                        f30(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a107 = v0;
                                                    let a108 = 70u64;
                                                    if nat_eq(a107, a108) {
                                                        f31(v5, v2, v4)?
                                                    } else {
                                                        let a109 = v0;
                                                        let a110 = 75u64;
                                                        if if nat_eq(a109, a110) {
                                                            true
                                                        } else {
                                                            let a111 = v0;
                                                            let a112 = 76u64;
                                                            nat_eq(a111, a112)
                                                        } {
                                                            f32({
                                                                let a113 = v0;
                                                                let a114 = 76u64;
                                                                nat_eq(a113, a114)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a115 = v0;
                                                            let a116 = 31u64;
                                                            if if if nat_eq(a115, a116) {
                                                                true
                                                            } else {
                                                                let a117 = v0;
                                                                let a118 = 47u64;
                                                                nat_eq(a117, a118)
                                                            } {
                                                                true
                                                            } else {
                                                                let a119 = v0;
                                                                let a120 = 68u64;
                                                                nat_eq(a119, a120)
                                                            } {
                                                                f33(v0, v1, v2, v4)?
                                                            } else {
                                                                let a121 = v0;
                                                                let a122 = 15u64;
                                                                if nat_eq(a121, a122) {
                                                                    f34(v2, v4)?
                                                                } else {
                                                                    let a123 = v0;
                                                                    let a124 = 17u64;
                                                                    if nat_eq(a123, a124) {
                                                                        f35(v2, v4)?
                                                                    } else {
                                                                        let a125 = v0;
                                                                        let a126 = 18u64;
                                                                        if nat_eq(a125, a126) {
                                                                            f36(v2, v3, v4)?
                                                                        } else {
                                                                            let a127 = v0;
                                                                            let a128 = 13u64;
                                                                            if if if nat_eq(a127, a128) {
                                                                                true
                                                                            } else {
                                                                                let a129 = v0;
                                                                                let a130 = 14u64;
                                                                                nat_eq(a129, a130)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a131 = v0;
                                                                                let a132 = 16u64;
                                                                                nat_eq(a131, a132)
                                                                            } {
                                                                                f37(v0, v2, v4)?
                                                                            } else {
                                                                                let m133 = f38(v0, v5);
                                                                                match m133 {
                                                                                    None => {
                                                                                        f39(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f40(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f4(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m134 = v3;
    match m134 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f41(v0, v1)?, f41(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f43(v0, v1, v2.clone())?, f41(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f44(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f45(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f46(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f45(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f47(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f48(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f49(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f41(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f50(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f41(v0, v1)?, f41(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f51(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f41(v0, v1)?, f41(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f41(v0, v1)?, f41(v0, v2)?))
        }
        Adt4::C15 => {
            f52(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f53(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f54(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f41(f42(v0, match v2.clone() {
                Adt3::C0(h135, _, _, _, _, _, _, _) => {
                    h135
                }
            }, match v2.clone() {
                Adt3::C0(_, h136, _, _, _, _, _, _) => {
                    h136
                }
            })?, v1)?, f41(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f5(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f55(match v1.clone() {
        Adt3::C0(h137, _, _, _, _, _, _, _) => {
            h137
        }
    })?;
    let v5: u64 = {
        let a138 = v0;
        let a139 = v4;
        let a140 = 0u64;
        nat_quot(a138, a139, a140)
    };
    let v6: u64 = {
        let a141 = v0;
        let a142 = v4;
        let a143 = 0u64;
        nat_rem(a141, a142, a143)
    };
    let a144 = v6;
    let a145 = 0u64;
    if nat_eq(a144, a145) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a146 = v6;
        let a147 = 1u64;
        if nat_eq(a146, a147) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a150 = {
                let a148 = v6;
                let a149 = 1u64;
                nat_add(a148, a149)?
            };
            let a151 = v4;
            if nat_eq(a150, a151) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f41(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a152 = v6;
                    let a153 = 2u64;
                    nat_sub(a152, a153)
                };
                let v8: u64 = {
                    let a154 = v7;
                    let a155 = 3u64;
                    let a156 = 0u64;
                    nat_quot(a154, a155, a156)
                };
                let v9: u64 = {
                    let a157 = v7;
                    let a158 = 3u64;
                    let a159 = 0u64;
                    nat_rem(a157, a158, a159)
                };
                let v10: u64 = f41({
                    let a163 = {
                        let a161 = v5;
                        let a162 = match v1.clone() {
                            Adt3::C0(h160, _, _, _, _, _, _, _) => {
                                h160
                            }
                        };
                        nat_mul(a161, a162)?
                    };
                    let a164 = v8;
                    nat_add(a163, a164)?
                }, v1)?;
                let a165 = v9;
                let a166 = 0u64;
                if nat_eq(a165, a166) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a167 = v9;
                    let a168 = 1u64;
                    if nat_eq(a167, a168) {
                        let a169 = v3;
                        let a170 = 2u64;
                        if nat_eq(a169, a170) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a171 = v3;
                        let a172 = 2u64;
                        if if nat_eq(a171, a172) {
                            true
                        } else {
                            let a173 = v3;
                            let a174 = 3u64;
                            nat_eq(a173, a174)
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

pub fn f6(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a176 = 2u64;
        let a177 = match v1.clone() {
            Adt3::C0(_, h175, _, _, _, _, _, _) => {
                h175
            }
        };
        nat_mul(a176, a177)?
    };
    let a178 = v0;
    let a179 = v5;
    if nat_lt(a178, a179) {
        let a183 = {
            let a180 = v0;
            let a181 = 2u64;
            let a182 = 0u64;
            nat_rem(a180, a181, a182)
        };
        let a184 = 0u64;
        if nat_eq(a183, a184) {
            Ok(Adt5::C4(19u64, 4u64, f41({
                let a185 = v0;
                let a186 = 2u64;
                let a187 = 0u64;
                nat_quot(a185, a186, a187)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a188 = v4;
            let a189 = 2147483648u64;
            if if nat_lt(a188, a189) {
                let a191 = v4;
                let a192 = match v2 {
                    Adt3::C0(_, h190, _, _, _, _, _, _) => {
                        h190
                    }
                };
                nat_lt(a191, a192)
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
            let a195 = match v2.clone() {
                Adt3::C0(h193, _, _, _, _, _, _, _) => {
                    h193
                }
            };
            let a196 = match v2.clone() {
                Adt3::C0(_, h194, _, _, _, _, _, _) => {
                    h194
                }
            };
            nat_mul(a195, a196)?
        };
        let a199 = {
            let a197 = v0;
            let a198 = v5;
            nat_sub(a197, a198)
        };
        let a200 = v6;
        if nat_lt(a199, a200) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f41({
                let a201 = v0;
                let a202 = v5;
                nat_sub(a201, a202)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a205 = {
                    let a203 = v0;
                    let a204 = v5;
                    nat_sub(a203, a204)
                };
                let a206 = v6;
                nat_sub(a205, a206)
            };
            let v8: u64 = {
                let a210 = 1u64;
                let a211 = {
                    let a208 = 2u64;
                    let a209 = match v1.clone() {
                        Adt3::C0(h207, _, _, _, _, _, _, _) => {
                            h207
                        }
                    };
                    nat_mul(a208, a209)?
                };
                nat_add(a210, a211)?
            };
            let v9: u64 = {
                let a212 = v7;
                let a213 = v8;
                let a214 = 0u64;
                nat_quot(a212, a213, a214)
            };
            let v10: u64 = {
                let a215 = v7;
                let a216 = v8;
                let a217 = 0u64;
                nat_rem(a215, a216, a217)
            };
            let a218 = v10;
            let a219 = 0u64;
            if nat_eq(a218, a219) {
                Ok(Adt5::C4(19u64, 4u64, f41(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a223 = {
                    let a220 = v10;
                    let a221 = 2u64;
                    let a222 = 0u64;
                    nat_rem(a220, a221, a222)
                };
                let a224 = 1u64;
                if nat_eq(a223, a224) {
                    Ok(Adt5::C4(6u64, 0u64, f41({
                        let a233 = {
                            let a226 = v9;
                            let a227 = match v1.clone() {
                                Adt3::C0(h225, _, _, _, _, _, _, _) => {
                                    h225
                                }
                            };
                            nat_mul(a226, a227)?
                        };
                        let a234 = {
                            let a230 = {
                                let a228 = v10;
                                let a229 = 1u64;
                                nat_sub(a228, a229)
                            };
                            let a231 = 2u64;
                            let a232 = 0u64;
                            nat_quot(a230, a231, a232)
                        };
                        nat_add(a233, a234)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f41({
                        let a243 = {
                            let a236 = v4;
                            let a237 = match v1.clone() {
                                Adt3::C0(h235, _, _, _, _, _, _, _) => {
                                    h235
                                }
                            };
                            nat_mul(a236, a237)?
                        };
                        let a244 = {
                            let a240 = {
                                let a238 = v10;
                                let a239 = 1u64;
                                nat_sub(a238, a239)
                            };
                            let a241 = 2u64;
                            let a242 = 0u64;
                            nat_quot(a240, a241, a242)
                        };
                        nat_add(a243, a244)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f41({
                        let a253 = {
                            let a246 = v4;
                            let a247 = match v1 {
                                Adt3::C0(h245, _, _, _, _, _, _, _) => {
                                    h245
                                }
                            };
                            nat_mul(a246, a247)?
                        };
                        let a254 = {
                            let a250 = {
                                let a248 = v10;
                                let a249 = 1u64;
                                nat_sub(a248, a249)
                            };
                            let a251 = 2u64;
                            let a252 = 0u64;
                            nat_quot(a250, a251, a252)
                        };
                        nat_add(a253, a254)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a255 = v1;
    let a256 = f56(v2.clone(), v4.clone())?;
    if nat_lt(a255, a256) {
        let a259 = v1;
        let a260 = {
            let a257 = 2u64;
            let a258 = f57(v4.clone())?;
            nat_mul(a257, a258)?
        };
        if nat_lt(a259, a260) {
            let a264 = {
                let a261 = v1;
                let a262 = 2u64;
                let a263 = 0u64;
                nat_rem(a261, a262, a263)
            };
            let a265 = 0u64;
            if nat_eq(a264, a265) {
                f58({
                    let a266 = v1;
                    let a267 = 2u64;
                    let a268 = 0u64;
                    nat_quot(a266, a267, a268)
                }, v4)
            } else {
                if f59(match v2 {
                    Adt3::C0(_, h269, _, _, _, _, _, _) => {
                        h269
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a272 = v1;
                let a273 = {
                    let a270 = 2u64;
                    let a271 = f57(v4.clone())?;
                    nat_mul(a270, a271)?
                };
                nat_sub(a272, a273)
            };
            let v7: u64 = {
                let a277 = v6;
                let a278 = {
                    let a275 = 1u64;
                    let a276 = match v2.clone() {
                        Adt3::C0(h274, _, _, _, _, _, _, _) => {
                            h274
                        }
                    };
                    nat_add(a275, a276)?
                };
                let a279 = 0u64;
                nat_quot(a277, a278, a279)
            };
            let v8: u64 = {
                let a283 = v6;
                let a284 = {
                    let a281 = 1u64;
                    let a282 = match v2.clone() {
                        Adt3::C0(h280, _, _, _, _, _, _, _) => {
                            h280
                        }
                    };
                    nat_add(a281, a282)?
                };
                let a285 = 0u64;
                nat_rem(a283, a284, a285)
            };
            let a286 = v8;
            let a287 = 0u64;
            if nat_eq(a286, a287) {
                f58(v7, v4)
            } else {
                f60(v0, v7, {
                    let a288 = v8;
                    let a289 = 1u64;
                    nat_sub(a288, a289)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f8(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a292 = match v4.clone() {
            Adt3::C0(h290, _, _, _, _, _, _, _) => {
                h290
            }
        };
        let a293 = match v4.clone() {
            Adt3::C0(_, h291, _, _, _, _, _, _) => {
                h291
            }
        };
        nat_mul(a292, a293)?
    };
    let v7: u64 = {
        let a294 = 2u64;
        let a295 = v6;
        nat_mul(a294, a295)?
    };
    let v8: u64 = f61(match v3.clone() {
        Adt3::C0(_, h296, _, _, _, _, _, _) => {
            h296
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h297, _, _, _, _, _) => {
            h297
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h298, _, _, _, _) => {
            h298
        }
    });
    let a299 = v0;
    let a300 = v7;
    if nat_lt(a299, a300) {
        let a304 = {
            let a301 = v0;
            let a302 = 2u64;
            let a303 = 0u64;
            nat_rem(a301, a302, a303)
        };
        let a305 = 0u64;
        if nat_eq(a304, a305) {
            Ok(Adt5::C4(19u64, 5u64, f41({
                let a306 = v0;
                let a307 = 2u64;
                let a308 = 0u64;
                nat_quot(a306, a307, a308)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f62(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a309 = v0;
            let a310 = v7;
            nat_sub(a309, a310)
        };
        let v10: u64 = {
            let a314 = v9;
            let a315 = {
                let a312 = 1u64;
                let a313 = match v1.clone() {
                    Adt3::C0(h311, _, _, _, _, _, _, _) => {
                        h311
                    }
                };
                nat_add(a312, a313)?
            };
            let a316 = 0u64;
            nat_quot(a314, a315, a316)
        };
        let v11: u64 = {
            let a320 = v9;
            let a321 = {
                let a318 = 1u64;
                let a319 = match v1.clone() {
                    Adt3::C0(h317, _, _, _, _, _, _, _) => {
                        h317
                    }
                };
                nat_add(a318, a319)?
            };
            let a322 = 0u64;
            nat_rem(a320, a321, a322)
        };
        let a323 = v11;
        let a324 = 0u64;
        if nat_eq(a323, a324) {
            Ok(Adt5::C4(19u64, 5u64, f41({
                let a325 = v10;
                let a326 = v6;
                let a327 = 0u64;
                nat_rem(a325, a326, a327)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f62(v5, v8) {
                let v12: u64 = {
                    let a333 = {
                        let a329 = v10;
                        let a330 = match v1.clone() {
                            Adt3::C0(h328, _, _, _, _, _, _, _) => {
                                h328
                            }
                        };
                        nat_mul(a329, a330)?
                    };
                    let a334 = {
                        let a331 = v11;
                        let a332 = 1u64;
                        nat_sub(a331, a332)
                    };
                    nat_add(a333, a334)?
                };
                let a335 = v5;
                let a336 = 4294967295u64;
                if nat_eq(a335, a336) {
                    Ok(Adt5::C1(false, f41(v12, v1)?, f41(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f41(v12, v1)?, 4u64, f41({
                        let a342 = {
                            let a338 = v5;
                            let a339 = match v3.clone() {
                                Adt3::C0(h337, _, _, _, _, _, _, _) => {
                                    h337
                                }
                            };
                            nat_mul(a338, a339)?
                        };
                        let a343 = {
                            let a340 = v11;
                            let a341 = 1u64;
                            nat_sub(a340, a341)
                        };
                        nat_add(a342, a343)?
                    }, v3)?, 3u64, f41(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a344 = v0;
    let a345 = f15(v1)?;
    if nat_lt(a344, a345) {
        let a346 = v0;
        let a347 = 4u64;
        if nat_lt(a346, a347) {
            let a348 = v0;
            let a349 = 0u64;
            if nat_eq(a348, a349) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a350 = v0;
                let a351 = 1u64;
                if nat_eq(a350, a351) {
                    let a352 = v5;
                    let a353 = 0u64;
                    if nat_eq(a352, a353) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a354 = v0;
                    let a355 = 2u64;
                    if nat_eq(a354, a355) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a356 = v0;
            let a357 = 4u64;
            if if nat_eq(a356, a357) {
                let a358 = f63(v5)?;
                let a359 = v1;
                !nat_eq(a358, a359)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a362 = {
                        let a360 = v0;
                        let a361 = 4u64;
                        nat_sub(a360, a361)
                    };
                    let a363 = 3u64;
                    let a364 = 0u64;
                    nat_quot(a362, a363, a364)
                };
                let v7: u64 = {
                    let a367 = {
                        let a365 = v0;
                        let a366 = 4u64;
                        nat_sub(a365, a366)
                    };
                    let a368 = 3u64;
                    let a369 = 0u64;
                    nat_rem(a367, a368, a369)
                };
                let a370 = v7;
                let a371 = 0u64;
                if nat_eq(a370, a371) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a372 = v7;
                    let a373 = 1u64;
                    if nat_eq(a372, a373) {
                        Ok(Adt5::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt5::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f14(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f10(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt5> {
    Ok(f64(v3.clone(), {
        let m374 = v3.clone();
        match m374 {
            Adt4::C0 => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f5(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f6(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f7(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f4(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f8(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v49, v50, v51, v52) => {
                f9(v0, v52, v49, v50, v51, v4)?
            }
        }
    }))
}

pub fn f11(v0: Adt4) -> Adt5 {
    let m375 = v0;
    match m375 {
        Adt4::C0 => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C1 => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C2(_, _) => {
            Adt5::C6(3u64, 0u64, 0u64, false)
        }
        Adt4::C3(_, _, _, _, _) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C4(_) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C5(_, _, _, _) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C6(_, _, _) => {
            Adt5::C6(3u64, 0u64, 0u64, false)
        }
        Adt4::C7(_) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C8(_) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C9(v18, v19) => {
            Adt5::C6(0u64, v18, v19, true)
        }
        Adt4::C10(_, v21, _) => {
            Adt5::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false)
        }
        Adt4::C11(_, _) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C12(_, _, _) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C13(_, _, _, _) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C14(_, _, _) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C15 => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C16(_, _) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C17 => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C18(_, _) => {
            Adt5::C6(3u64, 0u64, 0u64, false)
        }
        Adt4::C19(_, _) => {
            Adt5::C6(3u64, 0u64, 0u64, false)
        }
        Adt4::C20(_, _) => {
            Adt5::C6(3u64, 0u64, 0u64, false)
        }
        Adt4::C21 => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
        Adt4::C22(_, _) => {
            Adt5::C6(7u64, 0u64, 0u64, false)
        }
        Adt4::C23(_, _, _, _) => {
            Adt5::C6(0u64, 0u64, 0u64, false)
        }
    }
}

pub fn f12(v0: Adt2, v1: u64) -> R<u64> {
    let a387 = {
        let a381 = {
            let a377 = match v0.clone() {
                Adt2::C0(_, h376, _, _, _, _, _, _, _, _, _) => {
                    h376
                }
            };
            let a378 = v1;
            let a379 = 0u64;
            nat_quot(a377, a378, a379)
        };
        let a382 = match v0.clone() {
            Adt2::C0(_, _, h380, _, _, _, _, _, _, _, _) => {
                h380
            }
        };
        nat_mul(a381, a382)?
    };
    let a388 = {
        let a385 = match v0.clone() {
            Adt2::C0(_, _, _, h383, _, _, _, _, _, _, _) => {
                h383
            }
        };
        let a386 = match v0 {
            Adt2::C0(_, _, _, _, h384, _, _, _, _, _, _) => {
                h384
            }
        };
        nat_mul(a385, a386)?
    };
    nat_mul(a387, a388)
}

pub fn f13(v0: u64) -> bool {
    let a389 = v0;
    let a390 = 4294967296u64;
    if nat_lt(a389, a390) {
        let a394 = {
            let a391 = v0;
            let a392 = 2147483648u64;
            let a393 = 0u64;
            nat_rem(a391, a392, a393)
        };
        let a395 = 2139095040u64;
        nat_lt(a394, a395)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a396 = 4u64;
    let a397 = v0;
    nat_mul(a396, a397)
}

pub fn f15(v0: u64) -> R<u64> {
    let a398 = v0;
    let a399 = 0u64;
    if nat_eq(a398, a399) {
        Ok(0u64)
    } else {
        let a402 = 4u64;
        let a403 = {
            let a400 = 3u64;
            let a401 = v0;
            nat_mul(a400, a401)?
        };
        nat_add(a402, a403)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a405 = match v0.clone() {
            Adt2::C0(_, h404, _, _, _, _, _, _, _, _, _) => {
                h404
            }
        };
        let a406 = v1;
        let a407 = 0u64;
        nat_quot(a405, a406, a407)
    }, match v0.clone() {
        Adt2::C0(_, _, h408, _, _, _, _, _, _, _, _) => {
            h408
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h409, _, _, _, _, _, _, _) => {
            h409
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h410, _, _, _, _, _, _) => {
            h410
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h411, _, _, _, _, _) => {
            h411
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h412, _, _, _, _) => {
            h412
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h413, _, _, _) => {
            h413
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h414, _, _) => {
            h414
        }
    })
}

pub fn f17(v0: Adt2, v1: Adt2) -> bool {
    let a417 = match v0.clone() {
        Adt2::C0(_, h415, _, _, _, _, _, _, _, _, _) => {
            h415
        }
    };
    let a418 = match v1.clone() {
        Adt2::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
            h416
        }
    };
    if if if nat_eq(a417, a418) {
        let a421 = match v0.clone() {
            Adt2::C0(_, _, h419, _, _, _, _, _, _, _, _) => {
                h419
            }
        };
        let a422 = match v1.clone() {
            Adt2::C0(_, _, h420, _, _, _, _, _, _, _, _) => {
                h420
            }
        };
        nat_eq(a421, a422)
    } else {
        false
    } {
        let a425 = match v0.clone() {
            Adt2::C0(_, _, _, h423, _, _, _, _, _, _, _) => {
                h423
            }
        };
        let a426 = match v1.clone() {
            Adt2::C0(_, _, _, h424, _, _, _, _, _, _, _) => {
                h424
            }
        };
        nat_eq(a425, a426)
    } else {
        false
    } {
        let a429 = match v0 {
            Adt2::C0(_, _, _, _, h427, _, _, _, _, _, _) => {
                h427
            }
        };
        let a430 = match v1 {
            Adt2::C0(_, _, _, _, h428, _, _, _, _, _, _) => {
                h428
            }
        };
        nat_eq(a429, a430)
    } else {
        false
    }
}

pub fn f18(v0: Adt2, v1: Adt2) -> R<bool> {
    let a432 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h431, _) => {
            h431
        }
    };
    let a433 = 0u64;
    if nat_eq(a432, a433) {
        Ok(true)
    } else {
        let a435 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h434, _) => {
                h434
            }
        };
        let a436 = 0u64;
        if nat_eq(a435, a436) {
            Ok(true)
        } else {
            let a438 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h437) => {
                    h437
                }
            };
            let a439 = 0u64;
            if nat_eq(a438, a439) {
                Ok(true)
            } else {
                let a441 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h440) => {
                        h440
                    }
                };
                let a442 = 0u64;
                if nat_eq(a441, a442) {
                    Ok(true)
                } else {
                    let a448 = {
                        let a445 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h443) => {
                                h443
                            }
                        };
                        let a446 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h444, _) => {
                                h444
                            }
                        };
                        nat_add(a445, a446)?
                    };
                    let a449 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h447) => {
                            h447
                        }
                    };
                    if nat_le(a448, a449) {
                        Ok(true)
                    } else {
                        let a455 = {
                            let a452 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h450) => {
                                    h450
                                }
                            };
                            let a453 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h451, _) => {
                                    h451
                                }
                            };
                            nat_add(a452, a453)?
                        };
                        let a456 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h454) => {
                                h454
                            }
                        };
                        Ok(nat_le(a455, a456))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a457 = v0;
    let a458 = 0u64;
    if if if if nat_eq(a457, a458) {
        true
    } else {
        let a459 = v1;
        let a460 = 0u64;
        nat_eq(a459, a460)
    } {
        true
    } else {
        let a461 = v2;
        let a462 = 0u64;
        nat_eq(a461, a462)
    } {
        true
    } else {
        let a463 = v3;
        let a464 = 0u64;
        nat_eq(a463, a464)
    } {
        Ok(0u64)
    } else {
        let a477 = {
            let a467 = 2u64;
            let a468 = {
                let a465 = v1;
                let a466 = v2;
                nat_mul(a465, a466)?
            };
            nat_mul(a467, a468)?
        };
        let a478 = {
            let a475 = {
                let a471 = {
                    let a469 = v1;
                    let a470 = v2;
                    nat_mul(a469, a470)?
                };
                let a472 = v3;
                nat_mul(a471, a472)?
            };
            let a476 = {
                let a473 = 1u64;
                let a474 = v0;
                nat_add(a473, a474)?
            };
            nat_mul(a475, a476)?
        };
        nat_add(a477, a478)
    }
}

pub fn f20(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m479 = v1;
    match m479 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f65(v0, v3, v2)
        }
    }
}

pub fn f21(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a481 = match v0.clone() {
        Adt2::C0(h480, _, _, _, _, _, _, _, _, _, _) => {
            h480
        }
    };
    let a482 = 1u64;
    if if nat_eq(a481, a482) {
        true
    } else {
        let a484 = match v0.clone() {
            Adt2::C0(h483, _, _, _, _, _, _, _, _, _, _) => {
                h483
            }
        };
        let a485 = 30u64;
        nat_eq(a484, a485)
    } {
        let a487 = match v1.clone() {
            Adt2::C0(h486, _, _, _, _, _, _, _, _, _, _) => {
                h486
            }
        };
        let a488 = 1u64;
        if if if if if if if if if if if nat_eq(a487, a488) {
            let a491 = match v0.clone() {
                Adt2::C0(_, h489, _, _, _, _, _, _, _, _, _) => {
                    h489
                }
            };
            let a492 = match v1.clone() {
                Adt2::C0(_, h490, _, _, _, _, _, _, _, _, _) => {
                    h490
                }
            };
            nat_eq(a491, a492)
        } else {
            false
        } {
            let a494 = 0u64;
            let a495 = match v1.clone() {
                Adt2::C0(_, _, h493, _, _, _, _, _, _, _, _) => {
                    h493
                }
            };
            nat_lt(a494, a495)
        } else {
            false
        } {
            let a498 = match v1.clone() {
                Adt2::C0(_, _, h496, _, _, _, _, _, _, _, _) => {
                    h496
                }
            };
            let a499 = match v1.clone() {
                Adt2::C0(_, _, _, h497, _, _, _, _, _, _, _) => {
                    h497
                }
            };
            nat_eq(a498, a499)
        } else {
            false
        } {
            let a501 = match v1.clone() {
                Adt2::C0(_, _, _, _, h500, _, _, _, _, _, _) => {
                    h500
                }
            };
            let a502 = 1u64;
            nat_eq(a501, a502)
        } else {
            false
        } {
            let a510 = match v0.clone() {
                Adt2::C0(_, _, h503, _, _, _, _, _, _, _, _) => {
                    h503
                }
            };
            let a511 = {
                let a508 = {
                    let a506 = match v1.clone() {
                        Adt2::C0(_, _, h504, _, _, _, _, _, _, _, _) => {
                            h504
                        }
                    };
                    let a507 = match v1.clone() {
                        Adt2::C0(_, _, h505, _, _, _, _, _, _, _, _) => {
                            h505
                        }
                    };
                    nat_add(a506, a507)?
                };
                let a509 = 1u64;
                nat_sub(a508, a509)
            };
            nat_eq(a510, a511)
        } else {
            false
        } {
            let a513 = 0u64;
            let a514 = match v0.clone() {
                Adt2::C0(_, _, _, h512, _, _, _, _, _, _, _) => {
                    h512
                }
            };
            nat_lt(a513, a514)
        } else {
            false
        } {
            let a516 = 0u64;
            let a517 = match v0.clone() {
                Adt2::C0(_, _, _, _, h515, _, _, _, _, _, _) => {
                    h515
                }
            };
            nat_lt(a516, a517)
        } else {
            false
        } {
            let a519 = 2u64;
            let a520 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h518, _, _, _, _, _) => {
                    h518
                }
            };
            nat_le(a519, a520)
        } else {
            false
        } {
            let a522 = 2u64;
            let a523 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h521, _, _, _, _, _) => {
                    h521
                }
            };
            nat_le(a522, a523)
        } else {
            false
        } {
            f18(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f12(v1.clone(), 1u64)?, 2u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f22(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m524 = v1;
    match m524 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a526 = match v0.clone() {
                Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                    h525
                }
            };
            let a527 = 0u64;
            if if if if if if if if if if if if if nat_eq(a526, a527) {
                true
            } else {
                let a529 = match v0.clone() {
                    Adt2::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                        h528
                    }
                };
                let a530 = 1u64;
                nat_eq(a529, a530)
            } {
                let a532 = match v3.clone() {
                    Adt2::C0(h531, _, _, _, _, _, _, _, _, _, _) => {
                        h531
                    }
                };
                let a533 = 26u64;
                nat_eq(a532, a533)
            } else {
                false
            } {
                let a535 = match v2.clone() {
                    Adt2::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                        h534
                    }
                };
                let a536 = 0u64;
                nat_eq(a535, a536)
            } else {
                false
            } {
                let a539 = match v0.clone() {
                    Adt2::C0(_, h537, _, _, _, _, _, _, _, _, _) => {
                        h537
                    }
                };
                let a540 = match v2.clone() {
                    Adt2::C0(_, h538, _, _, _, _, _, _, _, _, _) => {
                        h538
                    }
                };
                nat_eq(a539, a540)
            } else {
                false
            } {
                let a543 = match v3.clone() {
                    Adt2::C0(_, h541, _, _, _, _, _, _, _, _, _) => {
                        h541
                    }
                };
                let a544 = match v0.clone() {
                    Adt2::C0(_, _, h542, _, _, _, _, _, _, _, _) => {
                        h542
                    }
                };
                nat_le(a543, a544)
            } else {
                false
            } {
                let a546 = match v0.clone() {
                    Adt2::C0(_, _, _, h545, _, _, _, _, _, _, _) => {
                        h545
                    }
                };
                let a547 = 1u64;
                nat_eq(a546, a547)
            } else {
                false
            } {
                let a549 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h548, _, _, _, _, _, _) => {
                        h548
                    }
                };
                let a550 = 1u64;
                nat_eq(a549, a550)
            } else {
                false
            } {
                let a552 = match v2.clone() {
                    Adt2::C0(_, _, _, h551, _, _, _, _, _, _, _) => {
                        h551
                    }
                };
                let a553 = 1u64;
                nat_eq(a552, a553)
            } else {
                false
            } {
                let a555 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h554, _, _, _, _, _, _) => {
                        h554
                    }
                };
                let a556 = 1u64;
                nat_eq(a555, a556)
            } else {
                false
            } {
                let a558 = match v3.clone() {
                    Adt2::C0(_, _, h557, _, _, _, _, _, _, _, _) => {
                        h557
                    }
                };
                let a559 = 1u64;
                nat_eq(a558, a559)
            } else {
                false
            } {
                let a561 = match v3.clone() {
                    Adt2::C0(_, _, _, h560, _, _, _, _, _, _, _) => {
                        h560
                    }
                };
                let a562 = 1u64;
                nat_eq(a561, a562)
            } else {
                false
            } {
                let a564 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h563, _, _, _, _, _, _) => {
                        h563
                    }
                };
                let a565 = 1u64;
                nat_eq(a564, a565)
            } else {
                false
            } {
                if f66(match v2.clone() {
                    Adt2::C0(_, h566, _, _, _, _, _, _, _, _, _) => {
                        h566
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h567, _, _, _, _, _, _, _, _) => {
                        h567
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h568, _, _, _, _, _, _, _) => {
                        h568
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h569, _, _, _, _, _, _) => {
                        h569
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a571 = match v3.clone() {
                        Adt2::C0(_, h570, _, _, _, _, _, _, _, _, _) => {
                            h570
                        }
                    };
                    let a572 = 0u64;
                    if nat_eq(a571, a572) {
                        Ok(Adt6::C4(f12(v2.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a574 = match v0.clone() {
                            Adt2::C0(_, h573, _, _, _, _, _, _, _, _, _) => {
                                h573
                            }
                        };
                        let a575 = 2147483647u64;
                        if if if if if if nat_le(a574, a575) {
                            let a577 = match v3.clone() {
                                Adt2::C0(_, h576, _, _, _, _, _, _, _, _, _) => {
                                    h576
                                }
                            };
                            let a578 = 2147483647u64;
                            nat_le(a577, a578)
                        } else {
                            false
                        } {
                            let a580 = 4u64;
                            let a581 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h579, _, _, _, _, _) => {
                                    h579
                                }
                            };
                            nat_le(a580, a581)
                        } else {
                            false
                        } {
                            let a587 = {
                                let a584 = match v2.clone() {
                                    Adt2::C0(_, h582, _, _, _, _, _, _, _, _, _) => {
                                        h582
                                    }
                                };
                                let a585 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h583, _, _, _, _, _) => {
                                        h583
                                    }
                                };
                                nat_mul(a584, a585)?
                            };
                            let a588 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h586, _, _, _, _) => {
                                    h586
                                }
                            };
                            nat_le(a587, a588)
                        } else {
                            false
                        } {
                            f18(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f18(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a602 = {
                                    let a592 = {
                                        let a590 = 2u64;
                                        let a591 = match v3.clone() {
                                            Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
                                                h589
                                            }
                                        };
                                        nat_mul(a590, a591)?
                                    };
                                    let a593 = f12(v2.clone(), 1u64)?;
                                    nat_add(a592, a593)?
                                };
                                let a603 = {
                                    let a600 = match v3.clone() {
                                        Adt2::C0(_, h594, _, _, _, _, _, _, _, _, _) => {
                                            h594
                                        }
                                    };
                                    let a601 = {
                                        let a598 = 1u64;
                                        let a599 = {
                                            let a596 = 2u64;
                                            let a597 = match v0.clone() {
                                                Adt2::C0(_, h595, _, _, _, _, _, _, _, _, _) => {
                                                    h595
                                                }
                                            };
                                            nat_mul(a596, a597)?
                                        };
                                        nat_add(a598, a599)?
                                    };
                                    nat_mul(a600, a601)?
                                };
                                nat_add(a602, a603)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h604, _, _, _, _, _, _, _, _, _) => {
                                    h604
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h605, _, _, _, _, _, _, _, _, _) => {
                                    h605
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h606, _, _, _, _, _) => {
                                    h606
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h607, _, _, _, _) => {
                                    h607
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h608, _, _, _) => {
                                    h608
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h609, _, _) => {
                                    h609
                                }
                            }), f16(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h610, _, _, _, _, _, _, _, _, _, _) => {
                                    h610
                                }
                            }, f16(v3, 1u64))))
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

pub fn f23(v0: u64) -> bool {
    let a611 = v0;
    let a612 = 1u64;
    if if nat_eq(a611, a612) {
        true
    } else {
        let a613 = v0;
        let a614 = 33u64;
        nat_eq(a613, a614)
    } {
        true
    } else {
        let a615 = v0;
        let a616 = 34u64;
        nat_eq(a615, a616)
    }
}

pub fn f24(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a619 = match v1.clone() {
        Adt2::C0(h617, _, _, _, _, _, _, _, _, _, _) => {
            h617
        }
    };
    let a620 = match v3.clone() {
        Adt2::C0(h618, _, _, _, _, _, _, _, _, _, _) => {
            h618
        }
    };
    if nat_eq(a619, a620) {
        f67(v0, v1, v2, v3)
    } else {
        if if if if f68(match v1.clone() {
            Adt2::C0(h621, _, _, _, _, _, _, _, _, _, _) => {
                h621
            }
        }) {
            f68(match v3.clone() {
                Adt2::C0(h622, _, _, _, _, _, _, _, _, _, _) => {
                    h622
                }
            })
        } else {
            false
        } {
            let a623 = v0;
            let a624 = 33u64;
            if nat_eq(a623, a624) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a625 = f12(v1.clone(), 1u64)?;
            let a626 = f12(v3.clone(), 1u64)?;
            nat_eq(a625, a626)
        } else {
            false
        } {
            let m628 = f69(match v3.clone() {
                Adt2::C0(h627, _, _, _, _, _, _, _, _, _, _) => {
                    h627
                }
            });
            match m628 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f12(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h629) => {
                            h629
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                            h630
                        }
                    }, match v3 {
                        Adt2::C0(h631, _, _, _, _, _, _, _, _, _, _) => {
                            h631
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a633 = match v0.clone() {
        Adt2::C0(h632, _, _, _, _, _, _, _, _, _, _) => {
            h632
        }
    };
    let a634 = 0u64;
    if if if if nat_eq(a633, a634) {
        true
    } else {
        let a636 = match v0.clone() {
            Adt2::C0(h635, _, _, _, _, _, _, _, _, _, _) => {
                h635
            }
        };
        let a637 = 1u64;
        nat_eq(a636, a637)
    } {
        true
    } else {
        let a639 = match v0.clone() {
            Adt2::C0(h638, _, _, _, _, _, _, _, _, _, _) => {
                h638
            }
        };
        let a640 = 25u64;
        if nat_eq(a639, a640) {
            true
        } else {
            let a642 = match v0.clone() {
                Adt2::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
                    h641
                }
            };
            let a643 = 26u64;
            nat_eq(a642, a643)
        }
    } {
        true
    } else {
        let a645 = match v0.clone() {
            Adt2::C0(h644, _, _, _, _, _, _, _, _, _, _) => {
                h644
            }
        };
        let a646 = 30u64;
        nat_eq(a645, a646)
    } {
        let a649 = match v0.clone() {
            Adt2::C0(h647, _, _, _, _, _, _, _, _, _, _) => {
                h647
            }
        };
        let a650 = match v1.clone() {
            Adt2::C0(h648, _, _, _, _, _, _, _, _, _, _) => {
                h648
            }
        };
        if if nat_eq(a649, a650) {
            f70(match v1.clone() {
                Adt2::C0(_, h651, _, _, _, _, _, _, _, _, _) => {
                    h651
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h652, _, _, _, _, _, _, _, _) => {
                    h652
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h653, _, _, _, _, _, _, _) => {
                    h653
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h654, _, _, _, _, _, _) => {
                    h654
                }
            }, match v0.clone() {
                Adt2::C0(_, h655, _, _, _, _, _, _, _, _, _) => {
                    h655
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h656, _, _, _, _, _, _, _, _) => {
                    h656
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h657, _, _, _, _, _, _, _) => {
                    h657
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h658, _, _, _, _, _, _) => {
                    h658
                }
            })
        } else {
            false
        } {
            let m660 = f69(match v0.clone() {
                Adt2::C0(h659, _, _, _, _, _, _, _, _, _, _) => {
                    h659
                }
            });
            match m660 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h661) => {
                            h661
                        }
                    }, f16(v0, 1u64), f16(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f26(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m662 = v2;
    match m662 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a663 = v0;
            let a664 = 4u64;
            if if if nat_lt(a663, a664) {
                let a667 = match v1.clone() {
                    Adt2::C0(h665, _, _, _, _, _, _, _, _, _, _) => {
                        h665
                    }
                };
                let a668 = match v4.clone() {
                    Adt2::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
                        h666
                    }
                };
                if nat_eq(a667, a668) {
                    let a671 = match v1.clone() {
                        Adt2::C0(h669, _, _, _, _, _, _, _, _, _, _) => {
                            h669
                        }
                    };
                    let a672 = match v3.clone() {
                        Adt2::C0(h670, _, _, _, _, _, _, _, _, _, _) => {
                            h670
                        }
                    };
                    nat_eq(a671, a672)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f71(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h673, _, _, _, _, _, _, _, _, _) => {
                        h673
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h674, _, _, _, _, _, _, _, _, _) => {
                        h674
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h675, _, _, _, _, _, _, _, _, _) => {
                        h675
                    }
                })? {
                    f71(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h676, _, _, _, _, _, _, _, _) => {
                            h676
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                            h677
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h678, _, _, _, _, _, _, _, _) => {
                            h678
                        }
                    })?
                } else {
                    false
                } {
                    f71(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h679, _, _, _, _, _, _, _) => {
                            h679
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h680, _, _, _, _, _, _, _) => {
                            h680
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h681, _, _, _, _, _, _, _) => {
                            h681
                        }
                    })?
                } else {
                    false
                } {
                    f71(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h682, _, _, _, _, _, _) => {
                            h682
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h683, _, _, _, _, _, _) => {
                            h683
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h684, _, _, _, _, _, _) => {
                            h684
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m686 = f69(match v1.clone() {
                    Adt2::C0(h685, _, _, _, _, _, _, _, _, _, _) => {
                        h685
                    }
                });
                match m686 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h687, _) => {
                                h687
                            }
                        };
                        let a692 = {
                            let a689 = match v1.clone() {
                                Adt2::C0(_, h688, _, _, _, _, _, _, _, _, _) => {
                                    h688
                                }
                            };
                            let a690 = v6;
                            let a691 = 1u64;
                            nat_rem(a689, a690, a691)
                        };
                        let a693 = 0u64;
                        if if if nat_eq(a692, a693) {
                            let a698 = {
                                let a695 = match v4.clone() {
                                    Adt2::C0(_, h694, _, _, _, _, _, _, _, _, _) => {
                                        h694
                                    }
                                };
                                let a696 = v6;
                                let a697 = 1u64;
                                nat_rem(a695, a696, a697)
                            };
                            let a699 = 0u64;
                            nat_eq(a698, a699)
                        } else {
                            false
                        } {
                            let a704 = {
                                let a701 = match v3.clone() {
                                    Adt2::C0(_, h700, _, _, _, _, _, _, _, _, _) => {
                                        h700
                                    }
                                };
                                let a702 = v6;
                                let a703 = 1u64;
                                nat_rem(a701, a702, a703)
                            };
                            let a705 = 0u64;
                            nat_eq(a704, a705)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h706) => {
                                    h706
                                }
                            }, f16(v1, v6), f16(v3, v6), Adt4::C2(v0, f16(v4, v6))))
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

pub fn f27(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a708 = match v2.clone() {
        Adt2::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
            h707
        }
    };
    let a709 = 0u64;
    if if nat_eq(a708, a709) {
        let a711 = match v3.clone() {
            Adt2::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
                h710
            }
        };
        let a712 = 0u64;
        nat_eq(a711, a712)
    } else {
        false
    } {
        let a713 = v0;
        let a714 = 61u64;
        if nat_eq(a713, a714) {
            let v4: bool = {
                let a716 = 0u64;
                let a717 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h715, _, _, _, _, _, _, _) => {
                        h715
                    }
                };
                nat_lt(a716, a717)
            };
            if if if if if if f72(match v2.clone() {
                Adt2::C0(_, h718, _, _, _, _, _, _, _, _, _) => {
                    h718
                }
            }, match v1.clone() {
                Adt1::C0(h719, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h719
                }
            }, match v1.clone() {
                Adt1::C0(_, h720, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h720
                }
            }, match v3.clone() {
                Adt2::C0(_, h721, _, _, _, _, _, _, _, _, _) => {
                    h721
                }
            })? {
                f72(match v2.clone() {
                    Adt2::C0(_, _, h722, _, _, _, _, _, _, _, _) => {
                        h722
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h723, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h723
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h724, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h724
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h725, _, _, _, _, _, _, _, _) => {
                        h725
                    }
                })?
            } else {
                false
            } {
                f72(match v2.clone() {
                    Adt2::C0(_, _, _, h726, _, _, _, _, _, _, _) => {
                        h726
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h727, _, _, _, _, _, _, _, _, _, _, _) => {
                        h727
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h728, _, _, _, _, _, _, _, _, _, _) => {
                        h728
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h729, _, _, _, _, _, _, _) => {
                        h729
                    }
                })?
            } else {
                false
            } {
                f72(match v2.clone() {
                    Adt2::C0(_, _, _, _, h730, _, _, _, _, _, _) => {
                        h730
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h731, _, _, _, _, _, _, _, _, _) => {
                        h731
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h732, _, _, _, _, _, _, _, _) => {
                        h732
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h733, _, _, _, _, _, _) => {
                        h733
                    }
                })?
            } else {
                false
            } {
                let a735 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h734, _, _, _, _, _, _, _) => {
                        h734
                    }
                };
                let a736 = 4294967295u64;
                nat_le(a735, a736)
            } else {
                false
            } {
                if v4 {
                    !f66(match v2.clone() {
                        Adt2::C0(_, h737, _, _, _, _, _, _, _, _, _) => {
                            h737
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h738, _, _, _, _, _, _, _, _) => {
                            h738
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h739, _, _, _, _, _, _, _) => {
                            h739
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h740, _, _, _, _, _, _) => {
                            h740
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h741, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h741
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h742, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h742
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h743, _, _, _, _, _, _, _, _, _, _, _) => {
                        h743
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h744, _, _, _, _, _, _, _, _, _) => {
                        h744
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a745 = v0;
            let a746 = 62u64;
            if nat_eq(a745, a746) {
                let a748 = match v1.clone() {
                    Adt1::C0(h747, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h747
                    }
                };
                let a749 = 2147483648u64;
                if if if if if nat_lt(a748, a749) {
                    let a751 = match v1.clone() {
                        Adt1::C0(_, h750, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h750
                        }
                    };
                    let a752 = 2147483648u64;
                    nat_lt(a751, a752)
                } else {
                    false
                } {
                    let a755 = match v1.clone() {
                        Adt1::C0(h753, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h753
                        }
                    };
                    let a756 = match v2.clone() {
                        Adt2::C0(_, h754, _, _, _, _, _, _, _, _, _) => {
                            h754
                        }
                    };
                    nat_lt(a755, a756)
                } else {
                    false
                } {
                    let a759 = match v1.clone() {
                        Adt1::C0(_, h757, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h757
                        }
                    };
                    let a760 = match v2.clone() {
                        Adt2::C0(_, h758, _, _, _, _, _, _, _, _, _) => {
                            h758
                        }
                    };
                    nat_lt(a759, a760)
                } else {
                    false
                } {
                    let a763 = match v2.clone() {
                        Adt2::C0(_, _, h761, _, _, _, _, _, _, _, _) => {
                            h761
                        }
                    };
                    let a764 = match v3.clone() {
                        Adt2::C0(_, _, h762, _, _, _, _, _, _, _, _) => {
                            h762
                        }
                    };
                    if if if nat_eq(a763, a764) {
                        let a767 = match v2.clone() {
                            Adt2::C0(_, _, _, h765, _, _, _, _, _, _, _) => {
                                h765
                            }
                        };
                        let a768 = match v3.clone() {
                            Adt2::C0(_, _, _, h766, _, _, _, _, _, _, _) => {
                                h766
                            }
                        };
                        nat_eq(a767, a768)
                    } else {
                        false
                    } {
                        let a771 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h769, _, _, _, _, _, _) => {
                                h769
                            }
                        };
                        let a772 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h770, _, _, _, _, _, _) => {
                                h770
                            }
                        };
                        nat_eq(a771, a772)
                    } else {
                        false
                    } {
                        let a781 = match v3.clone() {
                            Adt2::C0(_, h773, _, _, _, _, _, _, _, _, _) => {
                                h773
                            }
                        };
                        let a782 = {
                            let a779 = {
                                let a776 = match v2.clone() {
                                    Adt2::C0(_, h774, _, _, _, _, _, _, _, _, _) => {
                                        h774
                                    }
                                };
                                let a777 = match v1.clone() {
                                    Adt1::C0(h775, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h775
                                    }
                                };
                                nat_add(a776, a777)?
                            };
                            let a780 = match v1.clone() {
                                Adt1::C0(_, h778, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h778
                                }
                            };
                            nat_add(a779, a780)?
                        };
                        nat_eq(a781, a782)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h783, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h783
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a784 = v0;
                let a785 = 63u64;
                if nat_eq(a784, a785) {
                    if if if if if f73(match v2.clone() {
                        Adt2::C0(_, h786, _, _, _, _, _, _, _, _, _) => {
                            h786
                        }
                    }, match v1.clone() {
                        Adt1::C0(h787, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h787
                        }
                    })? {
                        f73(match v2.clone() {
                            Adt2::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                                h788
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h789, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h789
                            }
                        })?
                    } else {
                        false
                    } {
                        f73(match v2.clone() {
                            Adt2::C0(_, _, _, h790, _, _, _, _, _, _, _) => {
                                h790
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h791, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h791
                            }
                        })?
                    } else {
                        false
                    } {
                        f73(match v2.clone() {
                            Adt2::C0(_, _, _, _, h792, _, _, _, _, _, _) => {
                                h792
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h793, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h793
                            }
                        })?
                    } else {
                        false
                    } {
                        let a796 = match v2.clone() {
                            Adt2::C0(_, h794, _, _, _, _, _, _, _, _, _) => {
                                h794
                            }
                        };
                        let a797 = match v3.clone() {
                            Adt2::C0(_, h795, _, _, _, _, _, _, _, _, _) => {
                                h795
                            }
                        };
                        if if if nat_eq(a796, a797) {
                            let a800 = match v2.clone() {
                                Adt2::C0(_, _, h798, _, _, _, _, _, _, _, _) => {
                                    h798
                                }
                            };
                            let a801 = match v3.clone() {
                                Adt2::C0(_, _, h799, _, _, _, _, _, _, _, _) => {
                                    h799
                                }
                            };
                            nat_eq(a800, a801)
                        } else {
                            false
                        } {
                            let a804 = match v2.clone() {
                                Adt2::C0(_, _, _, h802, _, _, _, _, _, _, _) => {
                                    h802
                                }
                            };
                            let a805 = match v3.clone() {
                                Adt2::C0(_, _, _, h803, _, _, _, _, _, _, _) => {
                                    h803
                                }
                            };
                            nat_eq(a804, a805)
                        } else {
                            false
                        } {
                            let a808 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h806, _, _, _, _, _, _) => {
                                    h806
                                }
                            };
                            let a809 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h807, _, _, _, _, _, _) => {
                                    h807
                                }
                            };
                            nat_eq(a808, a809)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h810, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h810
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h811, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h811
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h812, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h812
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h813, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h813
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

pub fn f28(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m814 = v3;
    match m814 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a815 = v0;
                let a816 = 4u64;
                nat_eq(a815, a816)
            };
            let v7: bool = {
                let a818 = 0u64;
                let a819 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h817, _, _, _, _, _, _, _, _, _, _, _) => {
                        h817
                    }
                };
                nat_lt(a818, a819)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h820, _, _, _, _, _, _, _, _, _) => {
                    h820
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h821, _, _, _, _, _, _, _, _) => {
                    h821
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h822, _, _, _, _, _, _, _) => {
                    h822
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h823, _, _, _, _, _, _) => {
                    h823
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h824, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h824
                }
            }, match v1.clone() {
                Adt1::C0(_, h825, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h825
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h826, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h826
                }
            });
            if if if if if if if if if if if if v6 {
                let a828 = match v2.clone() {
                    Adt2::C0(h827, _, _, _, _, _, _, _, _, _, _) => {
                        h827
                    }
                };
                let a829 = 0u64;
                if nat_eq(a828, a829) {
                    let a831 = match v5.clone() {
                        Adt2::C0(h830, _, _, _, _, _, _, _, _, _, _) => {
                            h830
                        }
                    };
                    let a832 = 0u64;
                    nat_eq(a831, a832)
                } else {
                    false
                }
            } else {
                let a834 = match v2.clone() {
                    Adt2::C0(h833, _, _, _, _, _, _, _, _, _, _) => {
                        h833
                    }
                };
                let a835 = 0u64;
                if if nat_eq(a834, a835) {
                    true
                } else {
                    let a837 = match v2.clone() {
                        Adt2::C0(h836, _, _, _, _, _, _, _, _, _, _) => {
                            h836
                        }
                    };
                    let a838 = 26u64;
                    nat_eq(a837, a838)
                } {
                    let a840 = match v5.clone() {
                        Adt2::C0(h839, _, _, _, _, _, _, _, _, _, _) => {
                            h839
                        }
                    };
                    let a841 = 0u64;
                    if nat_eq(a840, a841) {
                        true
                    } else {
                        let a843 = match v5.clone() {
                            Adt2::C0(h842, _, _, _, _, _, _, _, _, _, _) => {
                                h842
                            }
                        };
                        let a844 = 26u64;
                        nat_eq(a843, a844)
                    }
                } else {
                    false
                }
            } {
                let a847 = match v2.clone() {
                    Adt2::C0(h845, _, _, _, _, _, _, _, _, _, _) => {
                        h845
                    }
                };
                let a848 = match v4.clone() {
                    Adt2::C0(h846, _, _, _, _, _, _, _, _, _, _) => {
                        h846
                    }
                };
                nat_eq(a847, a848)
            } else {
                false
            } {
                f17(v2.clone(), v4.clone())
            } else {
                false
            } {
                f74(v2.clone())?
            } else {
                false
            } {
                f74(v4.clone())?
            } else {
                false
            } {
                let a850 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h849, _, _, _, _, _) => {
                        h849
                    }
                };
                let a851 = 4u64;
                nat_eq(a850, a851)
            } else {
                false
            } {
                let a852 = v8;
                let a853 = f12(v2.clone(), 1u64)?;
                nat_le(a852, a853)
            } else {
                false
            } {
                let a855 = match v1.clone() {
                    Adt1::C0(_, _, _, h854, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h854
                    }
                };
                let a856 = 1073741824u64;
                nat_lt(a855, a856)
            } else {
                false
            } {
                let a858 = match v1.clone() {
                    Adt1::C0(h857, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h857
                    }
                };
                let a859 = 4294967295u64;
                if if if if nat_le(a858, a859) {
                    let a861 = match v1.clone() {
                        Adt1::C0(_, h860, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h860
                        }
                    };
                    let a862 = 4294967295u64;
                    nat_le(a861, a862)
                } else {
                    false
                } {
                    let a864 = match v1.clone() {
                        Adt1::C0(_, _, h863, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h863
                        }
                    };
                    let a865 = 4294967295u64;
                    nat_le(a864, a865)
                } else {
                    false
                } {
                    let a867 = match v1.clone() {
                        Adt1::C0(_, _, _, h866, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h866
                        }
                    };
                    let a868 = 4294967295u64;
                    nat_le(a867, a868)
                } else {
                    false
                } {
                    let a870 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h869, _, _, _, _, _, _, _, _, _, _, _) => {
                            h869
                        }
                    };
                    let a871 = 4294967295u64;
                    nat_le(a870, a871)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a874 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h872) => {
                            h872
                        }
                    };
                    let a875 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h873) => {
                            h873
                        }
                    };
                    nat_eq(a874, a875)
                } else {
                    true
                }
            } else {
                false
            } {
                f75(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h876, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h876
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h877, _) => {
                        h877
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f12(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a878 = v10;
                    let a879 = v8;
                    nat_add(a878, a879)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h880, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h880
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f29(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a882 = match v0.clone() {
        Adt2::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
            h881
        }
    };
    let a883 = 0u64;
    if if if nat_eq(a882, a883) {
        let a885 = match v1.clone() {
            Adt2::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
                h884
            }
        };
        let a886 = 0u64;
        nat_eq(a885, a886)
    } else {
        false
    } {
        f70(match v0.clone() {
            Adt2::C0(_, h887, _, _, _, _, _, _, _, _, _) => {
                h887
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h888, _, _, _, _, _, _, _, _) => {
                h888
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h889, _, _, _, _, _, _, _) => {
                h889
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h890, _, _, _, _, _, _) => {
                h890
            }
        }, match v1.clone() {
            Adt2::C0(_, h891, _, _, _, _, _, _, _, _, _) => {
                h891
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h892, _, _, _, _, _, _, _, _) => {
                h892
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h893, _, _, _, _, _, _, _) => {
                h893
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h894, _, _, _, _, _, _) => {
                h894
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a895 = v2;
            let a896 = f12(v0.clone(), 1u64)?;
            nat_add(a895, a896)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f30(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a898 = match v1.clone() {
        Adt2::C0(h897, _, _, _, _, _, _, _, _, _, _) => {
            h897
        }
    };
    let a899 = 0u64;
    if if if nat_eq(a898, a899) {
        true
    } else {
        let a901 = match v1.clone() {
            Adt2::C0(h900, _, _, _, _, _, _, _, _, _, _) => {
                h900
            }
        };
        let a902 = 1u64;
        nat_eq(a901, a902)
    } {
        true
    } else {
        let a904 = match v1.clone() {
            Adt2::C0(h903, _, _, _, _, _, _, _, _, _, _) => {
                h903
            }
        };
        let a905 = 30u64;
        nat_eq(a904, a905)
    } {
        let a908 = match v1.clone() {
            Adt2::C0(h906, _, _, _, _, _, _, _, _, _, _) => {
                h906
            }
        };
        let a909 = match v2.clone() {
            Adt2::C0(h907, _, _, _, _, _, _, _, _, _, _) => {
                h907
            }
        };
        if if if if if nat_eq(a908, a909) {
            let a912 = match v1.clone() {
                Adt2::C0(_, h910, _, _, _, _, _, _, _, _, _) => {
                    h910
                }
            };
            let a913 = match v2.clone() {
                Adt2::C0(_, h911, _, _, _, _, _, _, _, _, _) => {
                    h911
                }
            };
            nat_eq(a912, a913)
        } else {
            false
        } {
            let a916 = match v1.clone() {
                Adt2::C0(_, _, _, h914, _, _, _, _, _, _, _) => {
                    h914
                }
            };
            let a917 = match v2.clone() {
                Adt2::C0(_, _, _, h915, _, _, _, _, _, _, _) => {
                    h915
                }
            };
            nat_eq(a916, a917)
        } else {
            false
        } {
            let a920 = match v1.clone() {
                Adt2::C0(_, _, _, _, h918, _, _, _, _, _, _) => {
                    h918
                }
            };
            let a921 = match v2.clone() {
                Adt2::C0(_, _, _, _, h919, _, _, _, _, _, _) => {
                    h919
                }
            };
            nat_eq(a920, a921)
        } else {
            false
        } {
            let a922 = v0;
            let a923 = 4u64;
            if nat_eq(a922, a923) {
                let a925 = match v1.clone() {
                    Adt2::C0(_, _, h924, _, _, _, _, _, _, _, _) => {
                        h924
                    }
                };
                let a926 = 1u64;
                if nat_eq(a925, a926) {
                    let a929 = match v1.clone() {
                        Adt2::C0(_, h927, _, _, _, _, _, _, _, _, _) => {
                            h927
                        }
                    };
                    let a930 = match v2.clone() {
                        Adt2::C0(_, _, h928, _, _, _, _, _, _, _, _) => {
                            h928
                        }
                    };
                    nat_eq(a929, a930)
                } else {
                    false
                }
            } else {
                let a931 = v0;
                let a932 = 4u64;
                if if nat_lt(a931, a932) {
                    let a935 = match v1.clone() {
                        Adt2::C0(_, h933, _, _, _, _, _, _, _, _, _) => {
                            h933
                        }
                    };
                    let a936 = match v1.clone() {
                        Adt2::C0(_, _, h934, _, _, _, _, _, _, _, _) => {
                            h934
                        }
                    };
                    nat_eq(a935, a936)
                } else {
                    false
                } {
                    let a939 = match v1.clone() {
                        Adt2::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                            h937
                        }
                    };
                    let a940 = match v2.clone() {
                        Adt2::C0(_, _, h938, _, _, _, _, _, _, _, _) => {
                            h938
                        }
                    };
                    nat_eq(a939, a940)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m942 = f69(match v1.clone() {
                Adt2::C0(h941, _, _, _, _, _, _, _, _, _, _) => {
                    h941
                }
            });
            match m942 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h943) => {
                            h943
                        }
                    }, f16(v1, 1u64), f16(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f31(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f76(match v1.clone() {
        Adt2::C0(h944, _, _, _, _, _, _, _, _, _, _) => {
            h944
        }
    }, v0) {
        let a947 = match v1.clone() {
            Adt2::C0(h945, _, _, _, _, _, _, _, _, _, _) => {
                h945
            }
        };
        let a948 = match v2.clone() {
            Adt2::C0(h946, _, _, _, _, _, _, _, _, _, _) => {
                h946
            }
        };
        nat_eq(a947, a948)
    } else {
        false
    } {
        let a951 = match v1.clone() {
            Adt2::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        let a952 = match v2.clone() {
            Adt2::C0(_, h950, _, _, _, _, _, _, _, _, _) => {
                h950
            }
        };
        if if if nat_eq(a951, a952) {
            let a955 = match v1.clone() {
                Adt2::C0(_, _, h953, _, _, _, _, _, _, _, _) => {
                    h953
                }
            };
            let a956 = match v2.clone() {
                Adt2::C0(_, _, h954, _, _, _, _, _, _, _, _) => {
                    h954
                }
            };
            nat_eq(a955, a956)
        } else {
            false
        } {
            let a959 = match v1.clone() {
                Adt2::C0(_, _, _, h957, _, _, _, _, _, _, _) => {
                    h957
                }
            };
            let a960 = match v2.clone() {
                Adt2::C0(_, _, _, h958, _, _, _, _, _, _, _) => {
                    h958
                }
            };
            nat_eq(a959, a960)
        } else {
            false
        } {
            let a963 = match v1.clone() {
                Adt2::C0(_, _, _, _, h961, _, _, _, _, _, _) => {
                    h961
                }
            };
            let a964 = match v2.clone() {
                Adt2::C0(_, _, _, _, h962, _, _, _, _, _, _) => {
                    h962
                }
            };
            nat_eq(a963, a964)
        } else {
            false
        }
    } else {
        false
    } {
        let m966 = f69(match v1.clone() {
            Adt2::C0(h965, _, _, _, _, _, _, _, _, _, _) => {
                h965
            }
        });
        match m966 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f12(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h967) => {
                        h967
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h968, _, _, _, _, _, _, _, _, _, _) => {
                        h968
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f32(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h969, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h969
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h970, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h970
            }
        }
    };
    let a971 = 0u64;
    let a972 = v4;
    if if nat_lt(a971, a972) {
        let a973 = v4;
        let a974 = 2147483648u64;
        nat_lt(a973, a974)
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
        let v7: u64 = f77(match v5.clone() {
            Adt2::C0(_, _, h975, _, _, _, _, _, _, _, _) => {
                h975
            }
        }, v4)?;
        let v8: u64 = f77(match v5.clone() {
            Adt2::C0(_, _, _, h976, _, _, _, _, _, _, _) => {
                h976
            }
        }, v4)?;
        let a978 = match v5.clone() {
            Adt2::C0(h977, _, _, _, _, _, _, _, _, _, _) => {
                h977
            }
        };
        let a979 = 0u64;
        if if if if if nat_eq(a978, a979) {
            let a981 = match v6.clone() {
                Adt2::C0(h980, _, _, _, _, _, _, _, _, _, _) => {
                    h980
                }
            };
            let a982 = 0u64;
            nat_eq(a981, a982)
        } else {
            false
        } {
            let a984 = match v5.clone() {
                Adt2::C0(_, _, _, _, h983, _, _, _, _, _, _) => {
                    h983
                }
            };
            let a985 = 1u64;
            nat_eq(a984, a985)
        } else {
            false
        } {
            let a988 = match v5.clone() {
                Adt2::C0(_, h986, _, _, _, _, _, _, _, _, _) => {
                    h986
                }
            };
            let a989 = match v6.clone() {
                Adt2::C0(_, h987, _, _, _, _, _, _, _, _, _) => {
                    h987
                }
            };
            nat_eq(a988, a989)
        } else {
            false
        } {
            if v0 {
                let a991 = match v5.clone() {
                    Adt2::C0(_, _, h990, _, _, _, _, _, _, _, _) => {
                        h990
                    }
                };
                let a992 = 2147483648u64;
                if if if nat_lt(a991, a992) {
                    let a994 = match v5.clone() {
                        Adt2::C0(_, _, _, h993, _, _, _, _, _, _, _) => {
                            h993
                        }
                    };
                    let a995 = 2147483648u64;
                    nat_lt(a994, a995)
                } else {
                    false
                } {
                    let a998 = {
                        let a996 = v7;
                        let a997 = v8;
                        nat_mul(a996, a997)?
                    };
                    let a999 = 2147483648u64;
                    nat_le(a998, a999)
                } else {
                    false
                } {
                    f78(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1001 = match v1.clone() {
                    Adt1::C0(h1000, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1000
                    }
                };
                let a1002 = v7;
                if if if if nat_eq(a1001, a1002) {
                    let a1004 = match v1 {
                        Adt1::C0(_, h1003, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1003
                        }
                    };
                    let a1005 = v8;
                    nat_eq(a1004, a1005)
                } else {
                    false
                } {
                    f79(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1007 = match v6.clone() {
                        Adt2::C0(_, _, h1006, _, _, _, _, _, _, _, _) => {
                            h1006
                        }
                    };
                    let a1008 = v4;
                    if nat_eq(a1007, a1008) {
                        let a1010 = match v6.clone() {
                            Adt2::C0(_, _, _, h1009, _, _, _, _, _, _, _) => {
                                h1009
                            }
                        };
                        let a1011 = v4;
                        nat_eq(a1010, a1011)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1015 = match v6 {
                        Adt2::C0(_, _, _, _, h1012, _, _, _, _, _, _) => {
                            h1012
                        }
                    };
                    let a1016 = {
                        let a1013 = v7;
                        let a1014 = v8;
                        nat_mul(a1013, a1014)?
                    };
                    nat_eq(a1015, a1016)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f33(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a1018 = match v2.clone() {
        Adt2::C0(h1017, _, _, _, _, _, _, _, _, _, _) => {
            h1017
        }
    };
    let a1019 = 0u64;
    if if if if if nat_eq(a1018, a1019) {
        true
    } else {
        let a1021 = match v2.clone() {
            Adt2::C0(h1020, _, _, _, _, _, _, _, _, _, _) => {
                h1020
            }
        };
        let a1022 = 1u64;
        nat_eq(a1021, a1022)
    } {
        true
    } else {
        let a1024 = match v2.clone() {
            Adt2::C0(h1023, _, _, _, _, _, _, _, _, _, _) => {
                h1023
            }
        };
        let a1025 = 30u64;
        nat_eq(a1024, a1025)
    } {
        let a1028 = match v2.clone() {
            Adt2::C0(h1026, _, _, _, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1029 = match v3.clone() {
            Adt2::C0(h1027, _, _, _, _, _, _, _, _, _, _) => {
                h1027
            }
        };
        nat_eq(a1028, a1029)
    } else {
        false
    } {
        if if f17(v2.clone(), v3.clone()) {
            let a1031 = match v1.clone() {
                Adt1::C0(h1030, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1030
                }
            };
            let a1032 = 4294967296u64;
            nat_lt(a1031, a1032)
        } else {
            false
        } {
            let a1034 = match v1.clone() {
                Adt1::C0(_, h1033, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1033
                }
            };
            let a1035 = 4294967296u64;
            nat_lt(a1034, a1035)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f12(v3.clone(), 1u64)?, {
            let a1037 = match v2.clone() {
                Adt2::C0(h1036, _, _, _, _, _, _, _, _, _, _) => {
                    h1036
                }
            };
            let a1038 = 0u64;
            if nat_eq(a1037, a1038) {
                4u64
            } else {
                2u64
            }
        }, f16(v2.clone(), 1u64), f16(v3, 1u64), Adt4::C13({
            let a1039 = v0;
            let a1040 = 31u64;
            if nat_eq(a1039, a1040) {
                0u64
            } else {
                let a1041 = v0;
                let a1042 = 47u64;
                if nat_eq(a1041, a1042) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1043, _, _, _, _, _, _, _, _, _, _) => {
                h1043
            }
        }, match v1.clone() {
            Adt1::C0(h1044, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1044
            }
        }, match v1 {
            Adt1::C0(_, h1045, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1045
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f34(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1047 = match v0.clone() {
        Adt2::C0(h1046, _, _, _, _, _, _, _, _, _, _) => {
            h1046
        }
    };
    let a1048 = 0u64;
    if if if nat_eq(a1047, a1048) {
        let a1050 = match v1.clone() {
            Adt2::C0(h1049, _, _, _, _, _, _, _, _, _, _) => {
                h1049
            }
        };
        let a1051 = 0u64;
        nat_eq(a1050, a1051)
    } else {
        false
    } {
        f17(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f12(v1.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f35(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1053 = match v0.clone() {
        Adt2::C0(h1052, _, _, _, _, _, _, _, _, _, _) => {
            h1052
        }
    };
    let a1054 = 0u64;
    if if nat_eq(a1053, a1054) {
        let a1056 = match v1.clone() {
            Adt2::C0(h1055, _, _, _, _, _, _, _, _, _, _) => {
                h1055
            }
        };
        let a1057 = 26u64;
        nat_eq(a1056, a1057)
    } else {
        false
    } {
        let a1059 = match v0.clone() {
            Adt2::C0(_, _, _, h1058, _, _, _, _, _, _, _) => {
                h1058
            }
        };
        let a1060 = 1u64;
        if if if if if if if nat_eq(a1059, a1060) {
            let a1062 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1061, _, _, _, _, _, _) => {
                    h1061
                }
            };
            let a1063 = 1u64;
            nat_eq(a1062, a1063)
        } else {
            false
        } {
            let a1065 = match v0.clone() {
                Adt2::C0(_, h1064, _, _, _, _, _, _, _, _, _) => {
                    h1064
                }
            };
            let a1066 = 2147483647u64;
            nat_le(a1065, a1066)
        } else {
            false
        } {
            let a1069 = match v1.clone() {
                Adt2::C0(_, h1067, _, _, _, _, _, _, _, _, _) => {
                    h1067
                }
            };
            let a1070 = match v0.clone() {
                Adt2::C0(_, _, h1068, _, _, _, _, _, _, _, _) => {
                    h1068
                }
            };
            nat_eq(a1069, a1070)
        } else {
            false
        } {
            let a1072 = match v1.clone() {
                Adt2::C0(_, _, h1071, _, _, _, _, _, _, _, _) => {
                    h1071
                }
            };
            let a1073 = 1u64;
            nat_eq(a1072, a1073)
        } else {
            false
        } {
            let a1075 = match v1.clone() {
                Adt2::C0(_, _, _, h1074, _, _, _, _, _, _, _) => {
                    h1074
                }
            };
            let a1076 = 1u64;
            nat_eq(a1075, a1076)
        } else {
            false
        } {
            let a1078 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1077, _, _, _, _, _, _) => {
                    h1077
                }
            };
            let a1079 = 1u64;
            nat_eq(a1078, a1079)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1082 = match v0.clone() {
                    Adt2::C0(_, _, h1080, _, _, _, _, _, _, _, _) => {
                        h1080
                    }
                };
                let a1083 = f55(match v0.clone() {
                    Adt2::C0(_, h1081, _, _, _, _, _, _, _, _, _) => {
                        h1081
                    }
                })?;
                nat_mul(a1082, a1083)?
            }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f36(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1084 = v1;
    match m1084 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f57(f16(v0.clone(), 1u64))?;
            let a1086 = match v0.clone() {
                Adt2::C0(h1085, _, _, _, _, _, _, _, _, _, _) => {
                    h1085
                }
            };
            let a1087 = 26u64;
            if if if nat_eq(a1086, a1087) {
                let a1089 = match v3.clone() {
                    Adt2::C0(h1088, _, _, _, _, _, _, _, _, _, _) => {
                        h1088
                    }
                };
                let a1090 = 26u64;
                nat_eq(a1089, a1090)
            } else {
                false
            } {
                let a1092 = match v2.clone() {
                    Adt2::C0(h1091, _, _, _, _, _, _, _, _, _, _) => {
                        h1091
                    }
                };
                let a1093 = 27u64;
                nat_eq(a1092, a1093)
            } else {
                false
            } {
                if if if if if if f17(v0.clone(), v3.clone()) {
                    let a1095 = match v2.clone() {
                        Adt2::C0(_, h1094, _, _, _, _, _, _, _, _, _) => {
                            h1094
                        }
                    };
                    let a1096 = 1u64;
                    nat_eq(a1095, a1096)
                } else {
                    false
                } {
                    let a1098 = match v2.clone() {
                        Adt2::C0(_, _, h1097, _, _, _, _, _, _, _, _) => {
                            h1097
                        }
                    };
                    let a1099 = 1u64;
                    nat_eq(a1098, a1099)
                } else {
                    false
                } {
                    let a1101 = match v2.clone() {
                        Adt2::C0(_, _, _, h1100, _, _, _, _, _, _, _) => {
                            h1100
                        }
                    };
                    let a1102 = 1u64;
                    nat_eq(a1101, a1102)
                } else {
                    false
                } {
                    let a1104 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1103, _, _, _, _, _, _) => {
                            h1103
                        }
                    };
                    let a1105 = 1u64;
                    nat_eq(a1104, a1105)
                } else {
                    false
                } {
                    let a1106 = v4;
                    let a1107 = 9223372036854775808u64;
                    nat_lt(a1106, a1107)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1110 = 2u64;
                        let a1111 = {
                            let a1108 = v4;
                            let a1109 = 2u64;
                            nat_mul(a1108, a1109)?
                        };
                        nat_add(a1110, a1111)?
                    }, 8u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C18(v4, f16(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f37(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1112, _, _, _, _, _, _, _, _, _, _) => {
            h1112
        }
    };
    let a1113 = v3;
    let a1114 = 0u64;
    if if nat_eq(a1113, a1114) {
        true
    } else {
        let a1115 = v0;
        let a1116 = 13u64;
        if nat_eq(a1115, a1116) {
            let a1117 = v3;
            let a1118 = 1u64;
            if nat_eq(a1117, a1118) {
                true
            } else {
                let a1119 = v3;
                let a1120 = 30u64;
                nat_eq(a1119, a1120)
            }
        } else {
            false
        }
    } {
        let a1122 = v3;
        let a1123 = match v2.clone() {
            Adt2::C0(h1121, _, _, _, _, _, _, _, _, _, _) => {
                h1121
            }
        };
        if if nat_eq(a1122, a1123) {
            let a1124 = v0;
            let a1125 = 13u64;
            if nat_eq(a1124, a1125) {
                let a1127 = match v2.clone() {
                    Adt2::C0(_, h1126, _, _, _, _, _, _, _, _, _) => {
                        h1126
                    }
                };
                let a1128 = 1u64;
                if if if nat_eq(a1127, a1128) {
                    let a1130 = match v2.clone() {
                        Adt2::C0(_, _, h1129, _, _, _, _, _, _, _, _) => {
                            h1129
                        }
                    };
                    let a1131 = 1u64;
                    nat_eq(a1130, a1131)
                } else {
                    false
                } {
                    let a1133 = match v2.clone() {
                        Adt2::C0(_, _, _, h1132, _, _, _, _, _, _, _) => {
                            h1132
                        }
                    };
                    let a1134 = 1u64;
                    nat_eq(a1133, a1134)
                } else {
                    false
                } {
                    let a1136 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1135, _, _, _, _, _, _) => {
                            h1135
                        }
                    };
                    let a1137 = 1u64;
                    nat_eq(a1136, a1137)
                } else {
                    false
                }
            } else {
                let a1139 = match v2.clone() {
                    Adt2::C0(_, h1138, _, _, _, _, _, _, _, _, _) => {
                        h1138
                    }
                };
                let a1140 = 1u64;
                if if if nat_eq(a1139, a1140) {
                    let a1143 = match v1.clone() {
                        Adt2::C0(_, _, h1141, _, _, _, _, _, _, _, _) => {
                            h1141
                        }
                    };
                    let a1144 = match v2.clone() {
                        Adt2::C0(_, _, h1142, _, _, _, _, _, _, _, _) => {
                            h1142
                        }
                    };
                    nat_eq(a1143, a1144)
                } else {
                    false
                } {
                    let a1147 = match v1.clone() {
                        Adt2::C0(_, _, _, h1145, _, _, _, _, _, _, _) => {
                            h1145
                        }
                    };
                    let a1148 = match v2.clone() {
                        Adt2::C0(_, _, _, h1146, _, _, _, _, _, _, _) => {
                            h1146
                        }
                    };
                    nat_eq(a1147, a1148)
                } else {
                    false
                } {
                    let a1151 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1149, _, _, _, _, _, _) => {
                            h1149
                        }
                    };
                    let a1152 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1150, _, _, _, _, _, _) => {
                            h1150
                        }
                    };
                    nat_eq(a1151, a1152)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f16(v1, 1u64);
            Ok(Adt6::C4(f80(v4.clone(), v0, v3)?, {
                let a1153 = v3;
                let a1154 = 0u64;
                if nat_eq(a1153, a1154) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f16(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f38(v0: u64, v1: u64) -> Option<u64> {
    let a1155 = v0;
    let a1156 = 8u64;
    if nat_eq(a1155, a1156) {
        Some(5u64)
    } else {
        let a1157 = v0;
        let a1158 = 9u64;
        if nat_eq(a1157, a1158) {
            Some(6u64)
        } else {
            let a1159 = v0;
            let a1160 = 89u64;
            if nat_eq(a1159, a1160) {
                let a1161 = v1;
                let a1162 = 0u64;
                if nat_eq(a1161, a1162) {
                    Some(0u64)
                } else {
                    let a1163 = v1;
                    let a1164 = 1u64;
                    if nat_eq(a1163, a1164) {
                        Some(1u64)
                    } else {
                        let a1165 = v1;
                        let a1166 = 2u64;
                        if nat_eq(a1165, a1166) {
                            Some(2u64)
                        } else {
                            let a1167 = v1;
                            let a1168 = 3u64;
                            if nat_eq(a1167, a1168) {
                                Some(3u64)
                            } else {
                                let a1169 = v1;
                                let a1170 = 6u64;
                                if nat_eq(a1169, a1170) {
                                    Some(4u64)
                                } else {
                                    let a1171 = v1;
                                    let a1172 = 12u64;
                                    if nat_eq(a1171, a1172) {
                                        Some(7u64)
                                    } else {
                                        let a1173 = v1;
                                        let a1174 = 11u64;
                                        if nat_eq(a1173, a1174) {
                                            Some(8u64)
                                        } else {
                                            let a1175 = v1;
                                            let a1176 = 18u64;
                                            if nat_eq(a1175, a1176) {
                                                Some(9u64)
                                            } else {
                                                let a1177 = v1;
                                                let a1178 = 19u64;
                                                if nat_eq(a1177, a1178) {
                                                    Some(10u64)
                                                } else {
                                                    let a1179 = v1;
                                                    let a1180 = 20u64;
                                                    if nat_eq(a1179, a1180) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1181 = v1;
                                                        let a1182 = 21u64;
                                                        if nat_eq(a1181, a1182) {
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

pub fn f39(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1183 = f81(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1183 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v5, v6, {
                let m1184 = v3;
                match m1184 {
                    None => {
                        f16(v2, 1u64)
                    }
                    Some(v12) => {
                        f16(v12, 1u64)
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

pub fn f40(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1186 = match v1.clone() {
        Adt2::C0(h1185, _, _, _, _, _, _, _, _, _, _) => {
            h1185
        }
    };
    let a1187 = 0u64;
    if if if if if nat_eq(a1186, a1187) {
        true
    } else {
        let a1189 = match v1.clone() {
            Adt2::C0(h1188, _, _, _, _, _, _, _, _, _, _) => {
                h1188
            }
        };
        let a1190 = 1u64;
        nat_eq(a1189, a1190)
    } {
        true
    } else {
        let a1192 = match v1.clone() {
            Adt2::C0(h1191, _, _, _, _, _, _, _, _, _, _) => {
                h1191
            }
        };
        let a1193 = 30u64;
        nat_eq(a1192, a1193)
    } {
        let a1196 = match v1.clone() {
            Adt2::C0(h1194, _, _, _, _, _, _, _, _, _, _) => {
                h1194
            }
        };
        let a1197 = match v2.clone() {
            Adt2::C0(h1195, _, _, _, _, _, _, _, _, _, _) => {
                h1195
            }
        };
        if nat_eq(a1196, a1197) {
            true
        } else {
            let a1199 = match v2.clone() {
                Adt2::C0(h1198, _, _, _, _, _, _, _, _, _, _) => {
                    h1198
                }
            };
            let a1200 = 0u64;
            nat_eq(a1199, a1200)
        }
    } else {
        false
    } {
        f17(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f12(v2.clone(), 1u64)?, {
            let a1202 = match v2.clone() {
                Adt2::C0(h1201, _, _, _, _, _, _, _, _, _, _) => {
                    h1201
                }
            };
            let a1203 = 0u64;
            if nat_eq(a1202, a1203) {
                4u64
            } else {
                2u64
            }
        }, f16(v1.clone(), 1u64), f16(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1204, _, _, _, _, _, _, _, _, _, _) => {
                h1204
            }
        }, match v2 {
            Adt2::C0(h1205, _, _, _, _, _, _, _, _, _, _) => {
                h1205
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f41(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1207 = v0;
        let a1208 = match v1.clone() {
            Adt3::C0(h1206, _, _, _, _, _, _, _) => {
                h1206
            }
        };
        let a1209 = 0u64;
        nat_quot(a1207, a1208, a1209)
    };
    let v3: u64 = {
        let a1211 = v2;
        let a1212 = match v1.clone() {
            Adt3::C0(_, h1210, _, _, _, _, _, _) => {
                h1210
            }
        };
        let a1213 = 0u64;
        nat_quot(a1211, a1212, a1213)
    };
    f82({
        let a1215 = v0;
        let a1216 = match v1.clone() {
            Adt3::C0(h1214, _, _, _, _, _, _, _) => {
                h1214
            }
        };
        let a1217 = 0u64;
        nat_rem(a1215, a1216, a1217)
    }, {
        let a1219 = v2;
        let a1220 = match v1.clone() {
            Adt3::C0(_, h1218, _, _, _, _, _, _) => {
                h1218
            }
        };
        let a1221 = 0u64;
        nat_rem(a1219, a1220, a1221)
    }, {
        let a1223 = v3;
        let a1224 = match v1.clone() {
            Adt3::C0(_, _, h1222, _, _, _, _, _) => {
                h1222
            }
        };
        let a1225 = 0u64;
        nat_rem(a1223, a1224, a1225)
    }, {
        let a1227 = v3;
        let a1228 = match v1.clone() {
            Adt3::C0(_, _, h1226, _, _, _, _, _) => {
                h1226
            }
        };
        let a1229 = 0u64;
        nat_quot(a1227, a1228, a1229)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1230, _, _, _) => {
            h1230
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1231, _, _) => {
            h1231
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1232, _) => {
            h1232
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1233) => {
            h1233
        }
    })
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1251 = {
        let a1246 = f83(v2, {
            let a1237 = {
                let a1234 = v0;
                let a1235 = v1;
                let a1236 = 0u64;
                nat_quot(a1234, a1235, a1236)
            };
            let a1238 = v2;
            let a1239 = 0u64;
            nat_rem(a1237, a1238, a1239)
        }, {
            let a1243 = {
                let a1240 = v0;
                let a1241 = v1;
                let a1242 = 0u64;
                nat_quot(a1240, a1241, a1242)
            };
            let a1244 = v2;
            let a1245 = 0u64;
            nat_quot(a1243, a1244, a1245)
        })?;
        let a1247 = v1;
        nat_mul(a1246, a1247)?
    };
    let a1252 = {
        let a1248 = v0;
        let a1249 = v1;
        let a1250 = 0u64;
        nat_rem(a1248, a1249, a1250)
    };
    nat_add(a1251, a1252)
}

pub fn f43(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1254 = v0;
        let a1255 = match v2.clone() {
            Adt3::C0(h1253, _, _, _, _, _, _, _) => {
                h1253
            }
        };
        let a1256 = 0u64;
        nat_quot(a1254, a1255, a1256)
    };
    let v4: u64 = {
        let a1258 = v3;
        let a1259 = match v2.clone() {
            Adt3::C0(_, h1257, _, _, _, _, _, _) => {
                h1257
            }
        };
        let a1260 = 0u64;
        nat_quot(a1258, a1259, a1260)
    };
    f84({
        let a1262 = v0;
        let a1263 = match v2.clone() {
            Adt3::C0(h1261, _, _, _, _, _, _, _) => {
                h1261
            }
        };
        let a1264 = 0u64;
        nat_rem(a1262, a1263, a1264)
    }, {
        let a1266 = v3;
        let a1267 = match v2.clone() {
            Adt3::C0(_, h1265, _, _, _, _, _, _) => {
                h1265
            }
        };
        let a1268 = 0u64;
        nat_rem(a1266, a1267, a1268)
    }, {
        let a1270 = v4;
        let a1271 = match v2.clone() {
            Adt3::C0(_, _, h1269, _, _, _, _, _) => {
                h1269
            }
        };
        let a1272 = 0u64;
        nat_rem(a1270, a1271, a1272)
    }, {
        let a1274 = v4;
        let a1275 = match v2 {
            Adt3::C0(_, _, h1273, _, _, _, _, _) => {
                h1273
            }
        };
        let a1276 = 0u64;
        nat_quot(a1274, a1275, a1276)
    }, match v1.clone() {
        Adt3::C0(h1277, _, _, _, _, _, _, _) => {
            h1277
        }
    }, match v1.clone() {
        Adt3::C0(_, h1278, _, _, _, _, _, _) => {
            h1278
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1279, _, _, _, _, _) => {
            h1279
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1280, _, _, _, _) => {
            h1280
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1281, _, _, _) => {
            h1281
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1282, _, _) => {
            h1282
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1283, _) => {
            h1283
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1284) => {
            h1284
        }
    })
}

pub fn f44(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1286 = v0;
        let a1287 = match v3.clone() {
            Adt3::C0(h1285, _, _, _, _, _, _, _) => {
                h1285
            }
        };
        let a1288 = 0u64;
        nat_quot(a1286, a1287, a1288)
    };
    let v6: u64 = {
        let a1290 = v5;
        let a1291 = match v3.clone() {
            Adt3::C0(_, h1289, _, _, _, _, _, _) => {
                h1289
            }
        };
        let a1292 = 0u64;
        nat_quot(a1290, a1291, a1292)
    };
    let v7: u64 = {
        let a1294 = v0;
        let a1295 = match v3.clone() {
            Adt3::C0(h1293, _, _, _, _, _, _, _) => {
                h1293
            }
        };
        let a1296 = 0u64;
        nat_rem(a1294, a1295, a1296)
    };
    let v8: u64 = {
        let a1298 = v5;
        let a1299 = match v3.clone() {
            Adt3::C0(_, h1297, _, _, _, _, _, _) => {
                h1297
            }
        };
        let a1300 = 0u64;
        nat_rem(a1298, a1299, a1300)
    };
    let v9: u64 = {
        let a1302 = v6;
        let a1303 = match v3.clone() {
            Adt3::C0(_, _, h1301, _, _, _, _, _) => {
                h1301
            }
        };
        let a1304 = 0u64;
        nat_rem(a1302, a1303, a1304)
    };
    let v10: u64 = {
        let a1306 = v6;
        let a1307 = match v3.clone() {
            Adt3::C0(_, _, h1305, _, _, _, _, _) => {
                h1305
            }
        };
        let a1308 = 0u64;
        nat_quot(a1306, a1307, a1308)
    };
    let v11: u64 = {
        let a1309 = v4;
        let a1310 = 0u64;
        if nat_eq(a1309, a1310) {
            match v1.clone() {
                Adt3::C0(h1311, _, _, _, _, _, _, _) => {
                    h1311
                }
            }
        } else {
            let a1312 = v4;
            let a1313 = 1u64;
            if nat_eq(a1312, a1313) {
                match v1.clone() {
                    Adt3::C0(_, h1314, _, _, _, _, _, _) => {
                        h1314
                    }
                }
            } else {
                let a1315 = v4;
                let a1316 = 2u64;
                if nat_eq(a1315, a1316) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1317, _, _, _, _, _) => {
                            h1317
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1318, _, _, _, _) => {
                            h1318
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1325 = v11;
        let a1326 = {
            let a1319 = v4;
            let a1320 = 0u64;
            if nat_eq(a1319, a1320) {
                v7
            } else {
                let a1321 = v4;
                let a1322 = 1u64;
                if nat_eq(a1321, a1322) {
                    v8
                } else {
                    let a1323 = v4;
                    let a1324 = 2u64;
                    if nat_eq(a1323, a1324) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1325, a1326)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f82(if if v12 {
        let a1327 = v4;
        let a1328 = 0u64;
        nat_eq(a1327, a1328)
    } else {
        false
    } {
        let a1329 = v7;
        let a1330 = v11;
        nat_sub(a1329, a1330)
    } else {
        v7
    }, if if v12 {
        let a1331 = v4;
        let a1332 = 1u64;
        nat_eq(a1331, a1332)
    } else {
        false
    } {
        let a1333 = v8;
        let a1334 = v11;
        nat_sub(a1333, a1334)
    } else {
        v8
    }, if if v12 {
        let a1335 = v4;
        let a1336 = 2u64;
        nat_eq(a1335, a1336)
    } else {
        false
    } {
        let a1337 = v9;
        let a1338 = v11;
        nat_sub(a1337, a1338)
    } else {
        v9
    }, if if v12 {
        let a1339 = v4;
        let a1340 = 3u64;
        nat_eq(a1339, a1340)
    } else {
        false
    } {
        let a1341 = v10;
        let a1342 = v11;
        nat_sub(a1341, a1342)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1343, _, _, _) => {
            h1343
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1344, _, _) => {
            h1344
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1345, _) => {
            h1345
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1346) => {
            h1346
        }
    })?, f41(v0, v3)?))
}

pub fn f45(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f85(v0, v2.clone(), 0u64);
    let v9: u64 = f85(v0, v2.clone(), 1u64);
    let v10: u64 = f85(v0, v2.clone(), 2u64);
    let v11: u64 = f85(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f86(v1.clone(), f87(v8, v3, match v1.clone() {
            Adt3::C0(h1347, _, _, _, _, _, _, _) => {
                h1347
            }
        })?, f87(v9, v4, match v1.clone() {
            Adt3::C0(_, h1348, _, _, _, _, _, _) => {
                h1348
            }
        })?, f87(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1349, _, _, _, _, _) => {
                h1349
            }
        })?, f87(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1350, _, _, _, _) => {
                h1350
            }
        })?)?, f41(v0, v2)?))
    } else {
        if if if if f88(v8, v3, match v1.clone() {
            Adt3::C0(h1351, _, _, _, _, _, _, _) => {
                h1351
            }
        })? {
            f88(v9, v4, match v1.clone() {
                Adt3::C0(_, h1352, _, _, _, _, _, _) => {
                    h1352
                }
            })?
        } else {
            false
        } {
            f88(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1353, _, _, _, _, _) => {
                    h1353
                }
            })?
        } else {
            false
        } {
            f88(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1354, _, _, _, _) => {
                    h1354
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f86(v1, f89(v8, v3)?, f89(v9, v4)?, f89(v10, v5)?, f89(v11, v6)?)?, f41(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f41(v0, v2)?))
        }
    }
}

pub fn f46(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f85(v0, v2.clone(), 0u64);
    let v5: u64 = f85(v0, v2.clone(), 1u64);
    let v6: u64 = f85(v0, v2.clone(), 2u64);
    let v7: u64 = f85(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f86(v1.clone(), f90(v4, v3, match v1 {
        Adt3::C0(h1355, _, _, _, _, _, _, _) => {
            h1355
        }
    })?, v5, v6, v7)?, f41(v0, v2)?))
}

pub fn f47(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1356 = v0;
    let a1357 = v3;
    if nat_lt(a1356, a1357) {
        Ok(Adt5::C1(false, {
            let a1358 = v0;
            let a1359 = 4u64;
            nat_mul(a1358, a1359)?
        }, {
            let a1360 = v0;
            let a1361 = 4u64;
            nat_mul(a1360, a1361)?
        }))
    } else {
        let v6: u64 = {
            let a1362 = v0;
            let a1363 = v3;
            nat_sub(a1362, a1363)
        };
        let v7: u64 = f41(v6, v1)?;
        let v8: u64 = f91(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f48(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1364 = v0;
    let a1365 = v3;
    if nat_lt(a1364, a1365) {
        Ok(Adt5::C1(true, 0u64, f41(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1366 = v0;
            let a1367 = v3;
            nat_sub(a1366, a1367)
        };
        let v5: u64 = f43(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f41(v4, v1)?, 3u64, v5))
    }
}

pub fn f49(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1369 = v1;
        let a1370 = match v3.clone() {
            Adt3::C0(h1368, _, _, _, _, _, _, _) => {
                h1368
            }
        };
        let a1371 = 0u64;
        nat_rem(a1369, a1370, a1371)
    };
    let v5: u64 = {
        let a1377 = {
            let a1373 = v1;
            let a1374 = match v3.clone() {
                Adt3::C0(h1372, _, _, _, _, _, _, _) => {
                    h1372
                }
            };
            let a1375 = 0u64;
            nat_quot(a1373, a1374, a1375)
        };
        let a1378 = match v3.clone() {
            Adt3::C0(_, h1376, _, _, _, _, _, _) => {
                h1376
            }
        };
        let a1379 = 0u64;
        nat_rem(a1377, a1378, a1379)
    };
    let v6: bool = f92(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f43(v1, v2, v3.clone())?
    } else {
        0u64
    }, f41(v1, v3)?))
}

pub fn f50(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1380 = v3;
        let a1381 = 8u64;
        nat_add(a1380, a1381)?
    }, 0u64, f41(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f43(v0, v5, v2.clone())?
    }, 3u64, f41(v0, v2)?))
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1383 = v0;
        let a1384 = match v2.clone() {
            Adt3::C0(h1382, _, _, _, _, _, _, _) => {
                h1382
            }
        };
        let a1385 = 0u64;
        nat_rem(a1383, a1384, a1385)
    };
    let v7: u64 = {
        let a1387 = v0;
        let a1388 = match v2.clone() {
            Adt3::C0(h1386, _, _, _, _, _, _, _) => {
                h1386
            }
        };
        let a1389 = 0u64;
        nat_quot(a1387, a1388, a1389)
    };
    let v8: u64 = {
        let a1391 = v7;
        let a1392 = match v2.clone() {
            Adt3::C0(_, h1390, _, _, _, _, _, _) => {
                h1390
            }
        };
        let a1393 = 0u64;
        nat_rem(a1391, a1392, a1393)
    };
    let v9: u64 = {
        let a1395 = v7;
        let a1396 = match v2.clone() {
            Adt3::C0(_, h1394, _, _, _, _, _, _) => {
                h1394
            }
        };
        let a1397 = 0u64;
        nat_quot(a1395, a1396, a1397)
    };
    let v10: u64 = {
        let a1399 = v9;
        let a1400 = match v2.clone() {
            Adt3::C0(_, _, h1398, _, _, _, _, _) => {
                h1398
            }
        };
        let a1401 = 0u64;
        nat_rem(a1399, a1400, a1401)
    };
    let v11: u64 = {
        let a1403 = v9;
        let a1404 = match v2.clone() {
            Adt3::C0(_, _, h1402, _, _, _, _, _) => {
                h1402
            }
        };
        let a1405 = 0u64;
        nat_quot(a1403, a1404, a1405)
    };
    let v12: u64 = f41(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1414 = {
                let a1409 = {
                    let a1406 = v10;
                    let a1407 = v4;
                    let a1408 = 0u64;
                    nat_quot(a1406, a1407, a1408)
                };
                let a1410 = v5;
                nat_mul(a1409, a1410)?
            };
            let a1415 = {
                let a1411 = v8;
                let a1412 = v4;
                let a1413 = 0u64;
                nat_quot(a1411, a1412, a1413)
            };
            nat_add(a1414, a1415)?
        };
        let v14: u64 = {
            let a1435 = v6;
            let a1436 = {
                let a1433 = match v1.clone() {
                    Adt3::C0(h1416, _, _, _, _, _, _, _) => {
                        h1416
                    }
                };
                let a1434 = {
                    let a1431 = {
                        let a1417 = v8;
                        let a1418 = v4;
                        let a1419 = 0u64;
                        nat_rem(a1417, a1418, a1419)
                    };
                    let a1432 = {
                        let a1429 = match v1.clone() {
                            Adt3::C0(_, h1420, _, _, _, _, _, _) => {
                                h1420
                            }
                        };
                        let a1430 = {
                            let a1427 = {
                                let a1421 = v10;
                                let a1422 = v4;
                                let a1423 = 0u64;
                                nat_rem(a1421, a1422, a1423)
                            };
                            let a1428 = {
                                let a1425 = match v1.clone() {
                                    Adt3::C0(_, _, h1424, _, _, _, _, _) => {
                                        h1424
                                    }
                                };
                                let a1426 = v13;
                                nat_mul(a1425, a1426)?
                            };
                            nat_add(a1427, a1428)?
                        };
                        nat_mul(a1429, a1430)?
                    };
                    nat_add(a1431, a1432)?
                };
                nat_mul(a1433, a1434)?
            };
            nat_add(a1435, a1436)?
        };
        Ok(Adt5::C1(false, f41(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1442 = {
                let a1440 = {
                    let a1437 = v11;
                    let a1438 = v5;
                    let a1439 = 0u64;
                    nat_rem(a1437, a1438, a1439)
                };
                let a1441 = v4;
                nat_mul(a1440, a1441)?
            };
            let a1443 = v8;
            nat_add(a1442, a1443)?
        };
        let v16: u64 = {
            let a1449 = {
                let a1447 = {
                    let a1444 = v11;
                    let a1445 = v5;
                    let a1446 = 0u64;
                    nat_quot(a1444, a1445, a1446)
                };
                let a1448 = v4;
                nat_mul(a1447, a1448)?
            };
            let a1450 = v10;
            nat_add(a1449, a1450)?
        };
        let a1452 = v15;
        let a1453 = match v1.clone() {
            Adt3::C0(_, h1451, _, _, _, _, _, _) => {
                h1451
            }
        };
        if if nat_lt(a1452, a1453) {
            let a1455 = v16;
            let a1456 = match v1.clone() {
                Adt3::C0(_, _, h1454, _, _, _, _, _) => {
                    h1454
                }
            };
            nat_lt(a1455, a1456)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f41({
                let a1465 = v6;
                let a1466 = {
                    let a1463 = match v1.clone() {
                        Adt3::C0(h1457, _, _, _, _, _, _, _) => {
                            h1457
                        }
                    };
                    let a1464 = {
                        let a1461 = v15;
                        let a1462 = {
                            let a1459 = match v1.clone() {
                                Adt3::C0(_, h1458, _, _, _, _, _, _) => {
                                    h1458
                                }
                            };
                            let a1460 = v16;
                            nat_mul(a1459, a1460)?
                        };
                        nat_add(a1461, a1462)?
                    };
                    nat_mul(a1463, a1464)?
                };
                nat_add(a1465, a1466)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f41(v0, v2.clone())?;
    let v4: u64 = f41(v0, v1.clone())?;
    let a1471 = {
        let a1468 = v0;
        let a1469 = match v1 {
            Adt3::C0(h1467, _, _, _, _, _, _, _) => {
                h1467
            }
        };
        let a1470 = 0u64;
        nat_rem(a1468, a1469, a1470)
    };
    let a1472 = 0u64;
    if nat_eq(a1471, a1472) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f41({
            let a1473 = v0;
            let a1474 = 1u64;
            nat_sub(a1473, a1474)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1475 = v3;
    let a1476 = 13u64;
    if nat_eq(a1475, a1476) {
        f93(v0, v1, v4)
    } else {
        let v5: u64 = f94(match v1.clone() {
            Adt3::C0(h1477, _, _, _, _, _, _, _) => {
                h1477
            }
        }, {
            let a1478 = v3;
            let a1479 = 16u64;
            nat_eq(a1478, a1479)
        })?;
        let v6: u64 = {
            let a1480 = v0;
            let a1481 = v5;
            let a1482 = 0u64;
            nat_quot(a1480, a1481, a1482)
        };
        let v7: u64 = {
            let a1483 = v0;
            let a1484 = v5;
            let a1485 = 0u64;
            nat_rem(a1483, a1484, a1485)
        };
        let v8: u64 = {
            let a1489 = 1u64;
            let a1490 = {
                let a1487 = match v1.clone() {
                    Adt3::C0(h1486, _, _, _, _, _, _, _) => {
                        h1486
                    }
                };
                let a1488 = 2u64;
                nat_mul(a1487, a1488)?
            };
            nat_add(a1489, a1490)?
        };
        let a1491 = v7;
        let a1492 = v8;
        if nat_lt(a1491, a1492) {
            f95(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f41(v6, v2)?;
            let a1493 = v3;
            let a1494 = 14u64;
            if nat_eq(a1493, a1494) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1495 = v7;
                let a1496 = v8;
                if nat_eq(a1495, a1496) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1499 = v7;
                    let a1500 = {
                        let a1497 = v8;
                        let a1498 = 1u64;
                        nat_add(a1497, a1498)?
                    };
                    if nat_eq(a1499, a1500) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1501, _, _, _, _, _, _, _) => {
                                h1501
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

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1502 = v0;
    let a1503 = 0u64;
    if nat_eq(a1502, a1503) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1510 = {
            let a1504 = v0;
            let a1505 = 1u64;
            nat_add(a1504, a1505)?
        };
        let a1511 = {
            let a1508 = 2u64;
            let a1509 = {
                let a1506 = v3;
                let a1507 = 2u64;
                nat_mul(a1506, a1507)?
            };
            nat_add(a1508, a1509)?
        };
        if nat_eq(a1510, a1511) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1512 = v0;
                let a1513 = 1u64;
                nat_sub(a1512, a1513)
            };
            let v5: u64 = {
                let a1514 = v4;
                let a1515 = 2u64;
                let a1516 = 0u64;
                nat_quot(a1514, a1515, a1516)
            };
            let a1520 = {
                let a1517 = v4;
                let a1518 = 2u64;
                let a1519 = 0u64;
                nat_rem(a1517, a1518, a1519)
            };
            let a1521 = 0u64;
            if nat_eq(a1520, a1521) {
                Ok(Adt5::C4(17u64, 0u64, f41(v5, v1)?, 4u64, f41(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f55(v0: u64) -> R<u64> {
    let a1524 = 3u64;
    let a1525 = {
        let a1522 = v0;
        let a1523 = 3u64;
        nat_mul(a1522, a1523)?
    };
    nat_add(a1524, a1525)
}

pub fn f56(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1533 = {
        let a1526 = 2u64;
        let a1527 = f57(v1.clone())?;
        nat_mul(a1526, a1527)?
    };
    let a1534 = {
        let a1531 = f57(v1)?;
        let a1532 = {
            let a1529 = 1u64;
            let a1530 = match v0 {
                Adt3::C0(h1528, _, _, _, _, _, _, _) => {
                    h1528
                }
            };
            nat_add(a1529, a1530)?
        };
        nat_mul(a1531, a1532)?
    };
    nat_add(a1533, a1534)
}

pub fn f57(v0: Adt3) -> R<u64> {
    if f66(match v0.clone() {
        Adt3::C0(h1535, _, _, _, _, _, _, _) => {
            h1535
        }
    }, match v0.clone() {
        Adt3::C0(_, h1536, _, _, _, _, _, _) => {
            h1536
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1537, _, _, _, _, _) => {
            h1537
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1538, _, _, _, _) => {
            h1538
        }
    }) {
        Ok(0u64)
    } else {
        let a1547 = match v0.clone() {
            Adt3::C0(h1539, _, _, _, _, _, _, _) => {
                h1539
            }
        };
        let a1548 = {
            let a1545 = match v0.clone() {
                Adt3::C0(_, h1540, _, _, _, _, _, _) => {
                    h1540
                }
            };
            let a1546 = {
                let a1543 = match v0.clone() {
                    Adt3::C0(_, _, h1541, _, _, _, _, _) => {
                        h1541
                    }
                };
                let a1544 = match v0 {
                    Adt3::C0(_, _, _, h1542, _, _, _, _) => {
                        h1542
                    }
                };
                nat_mul(a1543, a1544)?
            };
            nat_mul(a1545, a1546)?
        };
        nat_mul(a1547, a1548)
    }
}

pub fn f58(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f41(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f59(v0: u64, v1: u64) -> bool {
    let a1549 = v1;
    let a1550 = 2147483647u64;
    if nat_le(a1549, a1550) {
        let a1551 = v1;
        let a1552 = v0;
        nat_lt(a1551, a1552)
    } else {
        false
    }
}

pub fn f60(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1553 = f69(v0);
    match m1553 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a1555 = v2;
                let a1556 = match v7.clone() {
                    Adt11::C0(_, h1554, _) => {
                        h1554
                    }
                };
                nat_mul(a1555, a1556)?
            };
            let m1569 = f96(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1557, _, _, _, _, _, _, _) => {
                    h1557
                }
            }, match v5.clone() {
                Adt3::C0(_, h1558, _, _, _, _, _, _) => {
                    h1558
                }
            }, match v5 {
                Adt3::C0(_, _, h1559, _, _, _, _, _) => {
                    h1559
                }
            }, {
                let a1562 = match v3.clone() {
                    Adt3::C0(h1560, _, _, _, _, _, _, _) => {
                        h1560
                    }
                };
                let a1563 = match v7.clone() {
                    Adt11::C0(_, h1561, _) => {
                        h1561
                    }
                };
                nat_mul(a1562, a1563)?
            }, match v3.clone() {
                Adt3::C0(_, h1564, _, _, _, _, _, _) => {
                    h1564
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1565, _, _, _) => {
                    h1565
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1566, _, _) => {
                    h1566
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1567, _) => {
                    h1567
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1568) => {
                    h1568
                }
            })?;
            match m1569 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1570 = v0;
                    let a1571 = 0u64;
                    if if nat_eq(a1570, a1571) {
                        true
                    } else {
                        let a1572 = v0;
                        let a1573 = 26u64;
                        nat_eq(a1572, a1573)
                    } {
                        Ok(Adt5::C1(false, v9, f41({
                            let a1577 = {
                                let a1575 = v1;
                                let a1576 = match v4.clone() {
                                    Adt3::C0(h1574, _, _, _, _, _, _, _) => {
                                        h1574
                                    }
                                };
                                nat_mul(a1575, a1576)?
                            };
                            let a1578 = v8;
                            nat_add(a1577, a1578)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h1579) => {
                                h1579
                            }
                        }, f41({
                            let a1583 = {
                                let a1581 = v1;
                                let a1582 = match v4.clone() {
                                    Adt3::C0(h1580, _, _, _, _, _, _, _) => {
                                        h1580
                                    }
                                };
                                nat_mul(a1581, a1582)?
                            };
                            let a1584 = v8;
                            nat_add(a1583, a1584)?
                        }, v4.clone())?, {
                            let a1591 = {
                                let a1589 = {
                                    let a1586 = match v7 {
                                        Adt11::C0(_, h1585, _) => {
                                            h1585
                                        }
                                    };
                                    let a1587 = 1u64;
                                    nat_sub(a1586, a1587)
                                };
                                let a1590 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1588, _, _, _) => {
                                        h1588
                                    }
                                };
                                nat_mul(a1589, a1590)?
                            };
                            let a1592 = 4u64;
                            nat_add(a1591, a1592)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1593, _, _, _) => {
                                h1593
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1594 = v1;
    let a1595 = 0u64;
    if if nat_eq(a1594, a1595) {
        true
    } else {
        let a1596 = v2;
        let a1597 = 0u64;
        nat_eq(a1596, a1597)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f62(v0: u64, v1: u64) -> bool {
    let a1598 = v0;
    let a1599 = 4294967295u64;
    if nat_eq(a1598, a1599) {
        true
    } else {
        let a1600 = v0;
        let a1601 = 2147483648u64;
        if nat_lt(a1600, a1601) {
            let a1602 = v0;
            let a1603 = v1;
            nat_lt(a1602, a1603)
        } else {
            false
        }
    }
}

pub fn f63(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a1604 = v0;
        let a1605 = 8388608u64;
        let a1606 = 0u64;
        nat_quot(a1604, a1605, a1606)
    };
    let a1607 = v0;
    let a1608 = 2147483648u64;
    if if if nat_lt(a1607, a1608) {
        let a1609 = 127u64;
        let a1610 = v1;
        nat_le(a1609, a1610)
    } else {
        false
    } {
        let a1611 = v1;
        let a1612 = 190u64;
        nat_lt(a1611, a1612)
    } else {
        false
    } {
        let v2: u64 = {
            let a1616 = 8388608u64;
            let a1617 = {
                let a1613 = v0;
                let a1614 = 8388608u64;
                let a1615 = 0u64;
                nat_rem(a1613, a1614, a1615)
            };
            nat_add(a1616, a1617)?
        };
        let a1618 = v1;
        let a1619 = 150u64;
        if nat_lt(a1618, a1619) {
            let v3: u64 = f97({
                let a1620 = 150u64;
                let a1621 = v1;
                nat_sub(a1620, a1621)
            });
            let a1625 = {
                let a1622 = v2;
                let a1623 = v3;
                let a1624 = 0u64;
                nat_rem(a1622, a1623, a1624)
            };
            let a1626 = 0u64;
            if nat_eq(a1625, a1626) {
                let a1627 = v2;
                let a1628 = v3;
                let a1629 = 0u64;
                Ok(nat_quot(a1627, a1628, a1629))
            } else {
                Ok(0u64)
            }
        } else {
            let a1632 = f97({
                let a1630 = v1;
                let a1631 = 150u64;
                nat_sub(a1630, a1631)
            });
            let a1633 = v2;
            nat_mul(a1632, a1633)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f64(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1634 = v1;
    match m1634 {
        Adt5::C0(v2) => {
            Adt5::C0(v2)
        }
        Adt5::C1(v3, v4, v5) => {
            Adt5::C1(v3, v4, v5)
        }
        Adt5::C2(v6, v7, v8, v9, v10, v11) => {
            Adt5::C2(v6, v7, v8, v9, v10, v11)
        }
        Adt5::C3(v12, v13, v14, v15) => {
            f98(f99(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f100(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f101(v0.clone(), v16);
            f98(f102(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
        }
        Adt5::C5(v24, v25, v26, v27, v28, v29) => {
            Adt5::C5(v24, v25, v26, v27, v28, v29)
        }
        Adt5::C6(v30, v31, v32, v33) => {
            Adt5::C6(v30, v31, v32, v33)
        }
        Adt5::C7(v34, v35, v36, v37, v38, v39) => {
            Adt5::C7(v34, v35, v36, v37, v38, v39)
        }
    }
}

pub fn f65(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f103(match v0.clone() {
        Adt2::C0(h1635, _, _, _, _, _, _, _, _, _, _) => {
            h1635
        }
    }, match v0.clone() {
        Adt2::C0(_, h1636, _, _, _, _, _, _, _, _, _) => {
            h1636
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1637, _, _, _, _, _, _, _, _) => {
            h1637
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1638, _, _, _, _, _, _, _) => {
            h1638
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1639, _, _, _, _, _, _) => {
            h1639
        }
    }, match v1.clone() {
        Adt2::C0(h1640, _, _, _, _, _, _, _, _, _, _) => {
            h1640
        }
    }, match v1.clone() {
        Adt2::C0(_, h1641, _, _, _, _, _, _, _, _, _) => {
            h1641
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1642, _, _, _, _, _, _, _, _) => {
            h1642
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1643, _, _, _, _, _, _, _) => {
            h1643
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1644, _, _, _, _, _, _) => {
            h1644
        }
    }, match v2.clone() {
        Adt2::C0(h1645, _, _, _, _, _, _, _, _, _, _) => {
            h1645
        }
    }, match v2.clone() {
        Adt2::C0(_, h1646, _, _, _, _, _, _, _, _, _) => {
            h1646
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1647, _, _, _, _, _, _, _, _) => {
            h1647
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1648, _, _, _, _, _, _, _) => {
            h1648
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1649, _, _, _, _, _, _) => {
            h1649
        }
    }) {
        let m1651 = f69(match v0.clone() {
            Adt2::C0(h1650, _, _, _, _, _, _, _, _, _, _) => {
                h1650
            }
        });
        match m1651 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1657 = {
                    let a1654 = match v0.clone() {
                        Adt2::C0(_, h1652, _, _, _, _, _, _, _, _, _) => {
                            h1652
                        }
                    };
                    let a1655 = match v3.clone() {
                        Adt11::C0(_, h1653, _) => {
                            h1653
                        }
                    };
                    let a1656 = 0u64;
                    nat_rem(a1654, a1655, a1656)
                };
                let a1658 = 0u64;
                if if nat_eq(a1657, a1658) {
                    if f104(match v0.clone() {
                        Adt2::C0(_, h1659, _, _, _, _, _, _, _, _, _) => {
                            h1659
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1660, _, _, _, _, _, _, _, _, _) => {
                            h1660
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1661, _, _, _, _, _, _, _, _) => {
                            h1661
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1662, _, _, _, _, _, _, _) => {
                            h1662
                        }
                    }) {
                        true
                    } else {
                        let a1664 = 4u64;
                        let a1665 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1663, _, _, _, _, _) => {
                                h1663
                            }
                        };
                        if if nat_le(a1664, a1665) {
                            let a1667 = match v3.clone() {
                                Adt11::C0(_, h1666, _) => {
                                    h1666
                                }
                            };
                            let a1668 = 1u64;
                            if nat_eq(a1667, a1668) {
                                true
                            } else {
                                let a1671 = match v3.clone() {
                                    Adt11::C0(_, _, h1669) => {
                                        h1669
                                    }
                                };
                                let a1672 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1670, _, _, _, _, _) => {
                                        h1670
                                    }
                                };
                                nat_le(a1671, a1672)
                            }
                        } else {
                            false
                        } {
                            if f18(v0.clone(), v2.clone())? {
                                f18(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f16(v0.clone(), match v3 {
                        Adt11::C0(_, h1673, _) => {
                            h1673
                        }
                    });
                    Ok(Adt6::C4(if f104(match v0.clone() {
                        Adt2::C0(_, h1674, _, _, _, _, _, _, _, _, _) => {
                            h1674
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1675, _, _, _, _, _, _, _, _, _) => {
                            h1675
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1676, _, _, _, _, _, _, _, _) => {
                            h1676
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1677, _, _, _, _, _, _, _) => {
                            h1677
                        }
                    }) {
                        0u64
                    } else {
                        f56(v4.clone(), f16(v1.clone(), 1u64))?
                    }, 4u64, v4, f16(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1678, _, _, _, _, _, _, _, _, _, _) => {
                            h1678
                        }
                    }, f16(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1679 = v0;
    let a1680 = 0u64;
    if if if nat_eq(a1679, a1680) {
        true
    } else {
        let a1681 = v1;
        let a1682 = 0u64;
        nat_eq(a1681, a1682)
    } {
        true
    } else {
        let a1683 = v2;
        let a1684 = 0u64;
        nat_eq(a1683, a1684)
    } {
        true
    } else {
        let a1685 = v3;
        let a1686 = 0u64;
        nat_eq(a1685, a1686)
    }
}

pub fn f67(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1689 = match v1.clone() {
        Adt2::C0(h1687, _, _, _, _, _, _, _, _, _, _) => {
            h1687
        }
    };
    let a1690 = match v3.clone() {
        Adt2::C0(h1688, _, _, _, _, _, _, _, _, _, _) => {
            h1688
        }
    };
    if if nat_eq(a1689, a1690) {
        let a1691 = v0;
        let a1692 = 33u64;
        if nat_eq(a1691, a1692) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1694 = f69(match v1.clone() {
            Adt2::C0(h1693, _, _, _, _, _, _, _, _, _, _) => {
                h1693
            }
        });
        match m1694 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1695, _) => {
                        h1695
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a1700 = {
                    let a1697 = match v1.clone() {
                        Adt2::C0(_, h1696, _, _, _, _, _, _, _, _, _) => {
                            h1696
                        }
                    };
                    let a1698 = v5;
                    let a1699 = 1u64;
                    nat_rem(a1697, a1698, a1699)
                };
                let a1701 = 0u64;
                if if if nat_eq(a1700, a1701) {
                    let a1706 = {
                        let a1703 = match v3.clone() {
                            Adt2::C0(_, h1702, _, _, _, _, _, _, _, _, _) => {
                                h1702
                            }
                        };
                        let a1704 = v5;
                        let a1705 = 1u64;
                        nat_rem(a1703, a1704, a1705)
                    };
                    let a1707 = 0u64;
                    nat_eq(a1706, a1707)
                } else {
                    false
                } {
                    let a1708 = v6;
                    let a1709 = f12(v3.clone(), v5)?;
                    nat_eq(a1708, a1709)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1710) => {
                            h1710
                        }
                    }, f16(v1, v5), f16(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f68(v0: u64) -> bool {
    let a1711 = v0;
    let a1712 = 0u64;
    if if nat_eq(a1711, a1712) {
        true
    } else {
        let a1713 = v0;
        let a1714 = 1u64;
        nat_eq(a1713, a1714)
    } {
        true
    } else {
        let a1715 = v0;
        let a1716 = 30u64;
        nat_eq(a1715, a1716)
    }
}

pub fn f69(v0: u64) -> Adt10 {
    let a1717 = v0;
    let a1718 = 0u64;
    if nat_eq(a1717, a1718) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1719 = v0;
        let a1720 = 1u64;
        if nat_eq(a1719, a1720) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1721 = v0;
            let a1722 = 2u64;
            if nat_eq(a1721, a1722) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1723 = v0;
                let a1724 = 3u64;
                if nat_eq(a1723, a1724) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1725 = v0;
                    let a1726 = 6u64;
                    if nat_eq(a1725, a1726) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1727 = v0;
                        let a1728 = 7u64;
                        if nat_eq(a1727, a1728) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1729 = v0;
                            let a1730 = 8u64;
                            if nat_eq(a1729, a1730) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1731 = v0;
                                let a1732 = 9u64;
                                if nat_eq(a1731, a1732) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1733 = v0;
                                    let a1734 = 10u64;
                                    if nat_eq(a1733, a1734) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1735 = v0;
                                        let a1736 = 11u64;
                                        if nat_eq(a1735, a1736) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1737 = v0;
                                            let a1738 = 12u64;
                                            if nat_eq(a1737, a1738) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1739 = v0;
                                                let a1740 = 13u64;
                                                if nat_eq(a1739, a1740) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1741 = v0;
                                                    let a1742 = 14u64;
                                                    if nat_eq(a1741, a1742) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1743 = v0;
                                                        let a1744 = 15u64;
                                                        if nat_eq(a1743, a1744) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1745 = v0;
                                                            let a1746 = 16u64;
                                                            if nat_eq(a1745, a1746) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1747 = v0;
                                                                let a1748 = 17u64;
                                                                if nat_eq(a1747, a1748) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1749 = v0;
                                                                    let a1750 = 18u64;
                                                                    if nat_eq(a1749, a1750) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1751 = v0;
                                                                        let a1752 = 19u64;
                                                                        if nat_eq(a1751, a1752) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1753 = v0;
                                                                            let a1754 = 20u64;
                                                                            if nat_eq(a1753, a1754) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1755 = v0;
                                                                                let a1756 = 21u64;
                                                                                if nat_eq(a1755, a1756) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1757 = v0;
                                                                                    let a1758 = 22u64;
                                                                                    if nat_eq(a1757, a1758) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1759 = v0;
                                                                                        let a1760 = 23u64;
                                                                                        if nat_eq(a1759, a1760) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1761 = v0;
                                                                                            let a1762 = 24u64;
                                                                                            if nat_eq(a1761, a1762) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1763 = v0;
                                                                                                let a1764 = 25u64;
                                                                                                if nat_eq(a1763, a1764) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1765 = v0;
                                                                                                    let a1766 = 26u64;
                                                                                                    if nat_eq(a1765, a1766) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1767 = v0;
                                                                                                        let a1768 = 27u64;
                                                                                                        if nat_eq(a1767, a1768) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1769 = v0;
                                                                                                            let a1770 = 28u64;
                                                                                                            if nat_eq(a1769, a1770) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1771 = v0;
                                                                                                                let a1772 = 29u64;
                                                                                                                if nat_eq(a1771, a1772) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1773 = v0;
                                                                                                                    let a1774 = 30u64;
                                                                                                                    if nat_eq(a1773, a1774) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1775 = v0;
                                                                                                                        let a1776 = 34u64;
                                                                                                                        if nat_eq(a1775, a1776) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1777 = v0;
                                                                                                                            let a1778 = 35u64;
                                                                                                                            if nat_eq(a1777, a1778) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1779 = v0;
                                                                                                                                let a1780 = 39u64;
                                                                                                                                if nat_eq(a1779, a1780) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1781 = v0;
                                                                                                                                    let a1782 = 40u64;
                                                                                                                                    if nat_eq(a1781, a1782) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1783 = v0;
                                                                                                                                        let a1784 = 41u64;
                                                                                                                                        if nat_eq(a1783, a1784) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1785 = v0;
                                                                                                                                            let a1786 = 42u64;
                                                                                                                                            if nat_eq(a1785, a1786) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1787 = v0;
                                                                                                                                                let a1788 = 43u64;
                                                                                                                                                if nat_eq(a1787, a1788) {
                                                                                                                                                    Adt10::C1(Adt11::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt10::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f66(v4, v5, v6, v7) {
        f66(v0, v1, v2, v3)
    } else {
        let a1792 = {
            let a1789 = v0;
            let a1790 = v4;
            let a1791 = 1u64;
            nat_rem(a1789, a1790, a1791)
        };
        let a1793 = 0u64;
        if if if nat_eq(a1792, a1793) {
            let a1797 = {
                let a1794 = v1;
                let a1795 = v5;
                let a1796 = 1u64;
                nat_rem(a1794, a1795, a1796)
            };
            let a1798 = 0u64;
            nat_eq(a1797, a1798)
        } else {
            false
        } {
            let a1802 = {
                let a1799 = v2;
                let a1800 = v6;
                let a1801 = 1u64;
                nat_rem(a1799, a1800, a1801)
            };
            let a1803 = 0u64;
            nat_eq(a1802, a1803)
        } else {
            false
        } {
            let a1807 = {
                let a1804 = v3;
                let a1805 = v7;
                let a1806 = 1u64;
                nat_rem(a1804, a1805, a1806)
            };
            let a1808 = 0u64;
            nat_eq(a1807, a1808)
        } else {
            false
        }
    }
}

pub fn f71(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1809 = v0;
    let a1810 = v1;
    if nat_eq(a1809, a1810) {
        let a1813 = v4;
        let a1814 = {
            let a1811 = v2;
            let a1812 = v3;
            nat_add(a1811, a1812)?
        };
        Ok(nat_eq(a1813, a1814))
    } else {
        let a1815 = v2;
        let a1816 = v3;
        if nat_eq(a1815, a1816) {
            let a1817 = v4;
            let a1818 = v2;
            Ok(nat_eq(a1817, a1818))
        } else {
            Ok(false)
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1821 = {
            let a1819 = v0;
            let a1820 = f105(v1);
            nat_add(a1819, a1820)?
        };
        let a1822 = f105(v2);
        nat_add(a1821, a1822)?
    };
    let v5: u64 = {
        let a1823 = f106(v1);
        let a1824 = f106(v2);
        nat_add(a1823, a1824)?
    };
    let a1825 = v1;
    let a1826 = 4294967295u64;
    if if nat_le(a1825, a1826) {
        let a1827 = v2;
        let a1828 = 4294967295u64;
        nat_le(a1827, a1828)
    } else {
        false
    } {
        let a1829 = v5;
        let a1830 = v4;
        if nat_le(a1829, a1830) {
            let a1833 = v3;
            let a1834 = {
                let a1831 = v4;
                let a1832 = v5;
                nat_sub(a1831, a1832)
            };
            Ok(nat_eq(a1833, a1834))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f73(v0: u64, v1: u64) -> R<bool> {
    let a1835 = v1;
    let a1836 = 4294967295u64;
    if nat_le(a1835, a1836) {
        let a1839 = {
            let a1837 = f105(v1);
            let a1838 = f106(v1);
            nat_add(a1837, a1838)?
        };
        let a1840 = v0;
        Ok(nat_lt(a1839, a1840))
    } else {
        Ok(false)
    }
}

pub fn f74(v0: Adt2) -> R<bool> {
    let a1842 = match v0.clone() {
        Adt2::C0(_, h1841, _, _, _, _, _, _, _, _, _) => {
            h1841
        }
    };
    let a1843 = 1u64;
    if if if if nat_eq(a1842, a1843) {
        true
    } else {
        let a1845 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1844, _, _, _, _, _) => {
                h1844
            }
        };
        let a1846 = 4u64;
        nat_eq(a1845, a1846)
    } {
        let a1848 = match v0.clone() {
            Adt2::C0(_, _, h1847, _, _, _, _, _, _, _, _) => {
                h1847
            }
        };
        let a1849 = 1u64;
        if nat_eq(a1848, a1849) {
            true
        } else {
            let a1854 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1850, _, _, _, _) => {
                    h1850
                }
            };
            let a1855 = {
                let a1852 = match v0.clone() {
                    Adt2::C0(_, h1851, _, _, _, _, _, _, _, _, _) => {
                        h1851
                    }
                };
                let a1853 = 4u64;
                nat_mul(a1852, a1853)?
            };
            nat_eq(a1854, a1855)
        }
    } else {
        false
    } {
        let a1857 = match v0.clone() {
            Adt2::C0(_, _, _, h1856, _, _, _, _, _, _, _) => {
                h1856
            }
        };
        let a1858 = 1u64;
        if nat_eq(a1857, a1858) {
            true
        } else {
            let a1866 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1859, _, _, _) => {
                    h1859
                }
            };
            let a1867 = {
                let a1864 = {
                    let a1862 = match v0.clone() {
                        Adt2::C0(_, h1860, _, _, _, _, _, _, _, _, _) => {
                            h1860
                        }
                    };
                    let a1863 = match v0.clone() {
                        Adt2::C0(_, _, h1861, _, _, _, _, _, _, _, _) => {
                            h1861
                        }
                    };
                    nat_mul(a1862, a1863)?
                };
                let a1865 = 4u64;
                nat_mul(a1864, a1865)?
            };
            nat_eq(a1866, a1867)
        }
    } else {
        false
    } {
        let a1869 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1868, _, _, _, _, _, _) => {
                h1868
            }
        };
        let a1870 = 1u64;
        if nat_eq(a1869, a1870) {
            Ok(true)
        } else {
            let a1881 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1871, _, _) => {
                    h1871
                }
            };
            let a1882 = {
                let a1879 = {
                    let a1877 = {
                        let a1874 = match v0.clone() {
                            Adt2::C0(_, h1872, _, _, _, _, _, _, _, _, _) => {
                                h1872
                            }
                        };
                        let a1875 = match v0.clone() {
                            Adt2::C0(_, _, h1873, _, _, _, _, _, _, _, _) => {
                                h1873
                            }
                        };
                        nat_mul(a1874, a1875)?
                    };
                    let a1878 = match v0 {
                        Adt2::C0(_, _, _, h1876, _, _, _, _, _, _, _) => {
                            h1876
                        }
                    };
                    nat_mul(a1877, a1878)?
                };
                let a1880 = 4u64;
                nat_mul(a1879, a1880)?
            };
            Ok(nat_eq(a1881, a1882))
        }
    } else {
        Ok(false)
    }
}

pub fn f75(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f107(v0.clone(), false)?;
    let v6: u64 = f107(v0, true)?;
    let a1883 = v6;
    let a1884 = v1;
    if if nat_le(a1883, a1884) {
        let a1891 = {
            let a1889 = {
                let a1885 = v1;
                let a1886 = v5;
                nat_add(a1885, a1886)?
            };
            let a1890 = {
                let a1887 = v3;
                let a1888 = 0u64;
                if nat_eq(a1887, a1888) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1889, a1890)?
        };
        let a1892 = v2;
        nat_le(a1891, a1892)
    } else {
        false
    } {
        if v4 {
            let a1897 = {
                let a1895 = {
                    let a1893 = v1;
                    let a1894 = v5;
                    nat_add(a1893, a1894)?
                };
                let a1896 = v6;
                nat_sub(a1895, a1896)
            };
            let a1898 = v2;
            Ok(nat_lt(a1897, a1898))
        } else {
            let a1903 = {
                let a1901 = {
                    let a1899 = v1;
                    let a1900 = v5;
                    nat_add(a1899, a1900)?
                };
                let a1902 = v6;
                nat_sub(a1901, a1902)
            };
            let a1904 = v2;
            Ok(nat_le(a1903, a1904))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1905 = v0;
    let a1906 = 0u64;
    if if nat_eq(a1905, a1906) {
        true
    } else {
        let a1907 = v0;
        let a1908 = 1u64;
        nat_eq(a1907, a1908)
    } {
        let a1909 = v1;
        let a1910 = 4294967296u64;
        nat_lt(a1909, a1910)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64) -> R<u64> {
    let a1915 = {
        let a1913 = v0;
        let a1914 = {
            let a1911 = v1;
            let a1912 = 1u64;
            nat_sub(a1911, a1912)
        };
        nat_add(a1913, a1914)?
    };
    let a1916 = v1;
    let a1917 = 0u64;
    Ok(nat_quot(a1915, a1916, a1917))
}

pub fn f78(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f66(match v0.clone() {
        Adt2::C0(_, h1918, _, _, _, _, _, _, _, _, _) => {
            h1918
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1919, _, _, _, _, _, _, _, _) => {
            h1919
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1920, _, _, _, _, _, _, _) => {
            h1920
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1921, _, _, _, _, _, _) => {
            h1921
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1924 = match v1.clone() {
                Adt2::C0(_, h1922, _, _, _, _, _, _, _, _, _) => {
                    h1922
                }
            };
            let a1925 = match v1.clone() {
                Adt2::C0(_, _, h1923, _, _, _, _, _, _, _, _) => {
                    h1923
                }
            };
            nat_mul(a1924, a1925)?
        };
        let v5: u64 = {
            let a1927 = v4;
            let a1928 = match v1.clone() {
                Adt2::C0(_, _, _, h1926, _, _, _, _, _, _, _) => {
                    h1926
                }
            };
            nat_mul(a1927, a1928)?
        };
        let a1941 = {
            let a1939 = {
                let a1930 = match v0.clone() {
                    Adt2::C0(_, h1929, _, _, _, _, _, _, _, _, _) => {
                        h1929
                    }
                };
                let a1931 = 1u64;
                nat_sub(a1930, a1931)
            };
            let a1940 = {
                let a1937 = f108(match v0.clone() {
                    Adt2::C0(_, _, h1932, _, _, _, _, _, _, _, _) => {
                        h1932
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1933, _, _, _, _, _, _, _, _, _) => {
                        h1933
                    }
                })?;
                let a1938 = f108(match v0 {
                    Adt2::C0(_, _, _, h1934, _, _, _, _, _, _, _) => {
                        h1934
                    }
                }, v2, {
                    let a1935 = v3;
                    let a1936 = v5;
                    nat_mul(a1935, a1936)?
                }, v4)?;
                nat_add(a1937, a1938)?
            };
            nat_add(a1939, a1940)?
        };
        let a1942 = f12(v1, 1u64)?;
        Ok(nat_lt(a1941, a1942))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1943 = v0;
    let a1944 = 2147483648u64;
    if if if if nat_lt(a1943, a1944) {
        let a1945 = v1;
        let a1946 = 2147483648u64;
        nat_lt(a1945, a1946)
    } else {
        false
    } {
        let a1949 = {
            let a1947 = v0;
            let a1948 = v1;
            nat_mul(a1947, a1948)?
        };
        let a1950 = 2147483648u64;
        nat_lt(a1949, a1950)
    } else {
        false
    } {
        let a1955 = {
            let a1953 = {
                let a1951 = v0;
                let a1952 = 1u64;
                nat_sub(a1951, a1952)
            };
            let a1954 = v2;
            nat_mul(a1953, a1954)?
        };
        let a1956 = 2147483648u64;
        nat_lt(a1955, a1956)
    } else {
        false
    } {
        let a1961 = {
            let a1959 = {
                let a1957 = v1;
                let a1958 = 1u64;
                nat_sub(a1957, a1958)
            };
            let a1960 = v2;
            nat_mul(a1959, a1960)?
        };
        let a1962 = 2147483648u64;
        Ok(nat_lt(a1961, a1962))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1963 = v1;
    let a1964 = 13u64;
    if if nat_eq(a1963, a1964) {
        f66(match v0.clone() {
            Adt3::C0(h1965, _, _, _, _, _, _, _) => {
                h1965
            }
        }, match v0.clone() {
            Adt3::C0(_, h1966, _, _, _, _, _, _) => {
                h1966
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1967, _, _, _, _, _) => {
                h1967
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1968, _, _, _, _) => {
                h1968
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1974 = match v0.clone() {
                Adt3::C0(_, h1969, _, _, _, _, _, _) => {
                    h1969
                }
            };
            let a1975 = {
                let a1972 = match v0.clone() {
                    Adt3::C0(_, _, h1970, _, _, _, _, _) => {
                        h1970
                    }
                };
                let a1973 = match v0.clone() {
                    Adt3::C0(_, _, _, h1971, _, _, _, _) => {
                        h1971
                    }
                };
                nat_mul(a1972, a1973)?
            };
            nat_mul(a1974, a1975)?
        };
        let a1976 = v1;
        let a1977 = 13u64;
        if nat_eq(a1976, a1977) {
            let a1986 = 2u64;
            let a1987 = if f109(v0.clone(), v2) {
                let a1979 = v3;
                let a1980 = f94(match v0 {
                    Adt3::C0(h1978, _, _, _, _, _, _, _) => {
                        h1978
                    }
                }, false)?;
                nat_mul(a1979, a1980)?
            } else {
                let a1984 = v3;
                let a1985 = {
                    let a1982 = match v0 {
                        Adt3::C0(h1981, _, _, _, _, _, _, _) => {
                            h1981
                        }
                    };
                    let a1983 = 2u64;
                    nat_mul(a1982, a1983)?
                };
                nat_mul(a1984, a1985)?
            };
            nat_add(a1986, a1987)
        } else {
            let a1991 = v3;
            let a1992 = f94(match v0 {
                Adt3::C0(h1988, _, _, _, _, _, _, _) => {
                    h1988
                }
            }, {
                let a1989 = v1;
                let a1990 = 16u64;
                nat_eq(a1989, a1990)
            })?;
            nat_mul(a1991, a1992)
        }
    }
}

pub fn f81(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1993 = v3;
    match m1993 {
        None => {
            f110(v0, v1, match v2.clone() {
                Adt2::C0(h1994, _, _, _, _, _, _, _, _, _, _) => {
                    h1994
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1995, _, _, _, _, _, _, _, _, _, _) => {
                    h1995
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1996, _, _, _, _, _, _, _, _, _) => {
                    h1996
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1997, _, _, _, _, _, _, _, _) => {
                    h1997
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1998, _, _, _, _, _, _, _) => {
                    h1998
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1999, _, _, _, _, _, _) => {
                    h1999
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h2000, _, _, _, _, _, _, _, _, _) => {
                    h2000
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2001, _, _, _, _, _, _, _, _) => {
                    h2001
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2002, _, _, _, _, _, _, _) => {
                    h2002
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2003, _, _, _, _, _, _) => {
                    h2003
                }
            })
        }
        Some(v5) => {
            f110(v0, v1, match v2.clone() {
                Adt2::C0(h2004, _, _, _, _, _, _, _, _, _, _) => {
                    h2004
                }
            }, match v5.clone() {
                Adt2::C0(h2005, _, _, _, _, _, _, _, _, _, _) => {
                    h2005
                }
            }, match v4.clone() {
                Adt2::C0(h2006, _, _, _, _, _, _, _, _, _, _) => {
                    h2006
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h2007, _, _, _, _, _, _, _, _, _) => {
                    h2007
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2008, _, _, _, _, _, _, _, _) => {
                    h2008
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2009, _, _, _, _, _, _, _) => {
                    h2009
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2010, _, _, _, _, _, _) => {
                    h2010
                }
            }, match v5.clone() {
                Adt2::C0(_, h2011, _, _, _, _, _, _, _, _, _) => {
                    h2011
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h2012, _, _, _, _, _, _, _, _) => {
                    h2012
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h2013, _, _, _, _, _, _, _) => {
                    h2013
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h2014, _, _, _, _, _, _) => {
                    h2014
                }
            }, match v4.clone() {
                Adt2::C0(_, h2015, _, _, _, _, _, _, _, _, _) => {
                    h2015
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2016, _, _, _, _, _, _, _, _) => {
                    h2016
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2017, _, _, _, _, _, _, _) => {
                    h2017
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2018, _, _, _, _, _, _) => {
                    h2018
                }
            })
        }
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2031 = {
        let a2027 = {
            let a2023 = {
                let a2019 = v0;
                let a2020 = v4;
                nat_mul(a2019, a2020)?
            };
            let a2024 = {
                let a2021 = v1;
                let a2022 = v5;
                nat_mul(a2021, a2022)?
            };
            nat_add(a2023, a2024)?
        };
        let a2028 = {
            let a2025 = v2;
            let a2026 = v6;
            nat_mul(a2025, a2026)?
        };
        nat_add(a2027, a2028)?
    };
    let a2032 = {
        let a2029 = v3;
        let a2030 = v7;
        nat_mul(a2029, a2030)?
    };
    nat_add(a2031, a2032)
}

pub fn f83(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2037 = {
        let a2035 = {
            let a2033 = v0;
            let a2034 = v1;
            nat_sub(a2033, a2034)
        };
        let a2036 = 1u64;
        nat_sub(a2035, a2036)
    };
    let a2038 = v2;
    nat_add(a2037, a2038)
}

pub fn f84(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f82({
        let a2039 = v0;
        let a2040 = v4;
        let a2041 = 0u64;
        nat_rem(a2039, a2040, a2041)
    }, {
        let a2042 = v1;
        let a2043 = v5;
        let a2044 = 0u64;
        nat_rem(a2042, a2043, a2044)
    }, {
        let a2045 = v2;
        let a2046 = v6;
        let a2047 = 0u64;
        nat_rem(a2045, a2046, a2047)
    }, {
        let a2048 = v3;
        let a2049 = v7;
        let a2050 = 0u64;
        nat_rem(a2048, a2049, a2050)
    }, v8, v9, v10, v11)
}

pub fn f85(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2052 = v0;
        let a2053 = match v1.clone() {
            Adt3::C0(h2051, _, _, _, _, _, _, _) => {
                h2051
            }
        };
        let a2054 = 0u64;
        nat_quot(a2052, a2053, a2054)
    };
    let v4: u64 = {
        let a2056 = v3;
        let a2057 = match v1.clone() {
            Adt3::C0(_, h2055, _, _, _, _, _, _) => {
                h2055
            }
        };
        let a2058 = 0u64;
        nat_quot(a2056, a2057, a2058)
    };
    let a2059 = v2;
    let a2060 = 0u64;
    if nat_eq(a2059, a2060) {
        let a2062 = v0;
        let a2063 = match v1 {
            Adt3::C0(h2061, _, _, _, _, _, _, _) => {
                h2061
            }
        };
        let a2064 = 0u64;
        nat_rem(a2062, a2063, a2064)
    } else {
        let a2065 = v2;
        let a2066 = 1u64;
        if nat_eq(a2065, a2066) {
            let a2068 = v3;
            let a2069 = match v1 {
                Adt3::C0(_, h2067, _, _, _, _, _, _) => {
                    h2067
                }
            };
            let a2070 = 0u64;
            nat_rem(a2068, a2069, a2070)
        } else {
            let a2071 = v2;
            let a2072 = 2u64;
            if nat_eq(a2071, a2072) {
                let a2074 = v4;
                let a2075 = match v1 {
                    Adt3::C0(_, _, h2073, _, _, _, _, _) => {
                        h2073
                    }
                };
                let a2076 = 0u64;
                nat_rem(a2074, a2075, a2076)
            } else {
                let a2078 = v4;
                let a2079 = match v1 {
                    Adt3::C0(_, _, h2077, _, _, _, _, _) => {
                        h2077
                    }
                };
                let a2080 = 0u64;
                nat_quot(a2078, a2079, a2080)
            }
        }
    }
}

pub fn f86(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f82(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2081, _, _, _) => {
            h2081
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2082, _, _) => {
            h2082
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2083, _) => {
            h2083
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2084) => {
            h2084
        }
    })
}

pub fn f87(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2085 = v1;
    let a2086 = 2147483648u64;
    if nat_lt(a2085, a2086) {
        let a2097 = {
            let a2095 = {
                let a2090 = {
                    let a2087 = v0;
                    let a2088 = v2;
                    let a2089 = 0u64;
                    nat_rem(a2087, a2088, a2089)
                };
                let a2091 = v2;
                nat_add(a2090, a2091)?
            };
            let a2096 = {
                let a2092 = v1;
                let a2093 = v2;
                let a2094 = 0u64;
                nat_rem(a2092, a2093, a2094)
            };
            nat_sub(a2095, a2096)
        };
        let a2098 = v2;
        let a2099 = 0u64;
        Ok(nat_rem(a2097, a2098, a2099))
    } else {
        let a2102 = {
            let a2100 = v0;
            let a2101 = f106(v1);
            nat_add(a2100, a2101)?
        };
        let a2103 = v2;
        let a2104 = 0u64;
        Ok(nat_rem(a2102, a2103, a2104))
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2107 = f105(v1);
    let a2108 = {
        let a2105 = v0;
        let a2106 = f106(v1);
        nat_add(a2105, a2106)?
    };
    if nat_le(a2107, a2108) {
        let a2109 = f89(v0, v1)?;
        let a2110 = v2;
        Ok(nat_lt(a2109, a2110))
    } else {
        Ok(false)
    }
}

pub fn f89(v0: u64, v1: u64) -> R<u64> {
    let a2113 = {
        let a2111 = v0;
        let a2112 = f106(v1);
        nat_add(a2111, a2112)?
    };
    let a2114 = f105(v1);
    Ok(nat_sub(a2113, a2114))
}

pub fn f90(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2115 = v0;
    let a2116 = v1;
    if nat_lt(a2115, a2116) {
        let a2117 = v1;
        let a2118 = v0;
        Ok(nat_sub(a2117, a2118))
    } else {
        let a2121 = v0;
        let a2122 = {
            let a2119 = v1;
            let a2120 = v2;
            nat_add(a2119, a2120)?
        };
        if nat_lt(a2121, a2122) {
            let a2123 = v0;
            let a2124 = v1;
            Ok(nat_sub(a2123, a2124))
        } else {
            let a2131 = {
                let a2127 = 2u64;
                let a2128 = {
                    let a2125 = v2;
                    let a2126 = 1u64;
                    nat_sub(a2125, a2126)
                };
                nat_mul(a2127, a2128)?
            };
            let a2132 = {
                let a2129 = v0;
                let a2130 = v1;
                nat_sub(a2129, a2130)
            };
            Ok(nat_sub(a2131, a2132))
        }
    }
}

pub fn f91(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2135 = {
        let a2133 = v2;
        let a2134 = f111(v1.clone(), f85(v0, v1.clone(), 0u64), f85(v0, v1.clone(), 1u64), f85(v0, v1.clone(), 2u64), f85(v0, v1.clone(), 3u64), false)?;
        nat_add(a2133, a2134)?
    };
    let a2136 = f111(v1.clone(), f85(v0, v1.clone(), 0u64), f85(v0, v1.clone(), 1u64), f85(v0, v1.clone(), 2u64), f85(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2135, a2136))
}

pub fn f92(v0: u64, v1: u64, v2: u64) -> bool {
    let a2137 = v0;
    let a2138 = 0u64;
    if nat_eq(a2137, a2138) {
        let a2139 = v2;
        let a2140 = v1;
        nat_le(a2139, a2140)
    } else {
        let a2141 = v0;
        let a2142 = 1u64;
        if nat_eq(a2141, a2142) {
            let a2143 = v2;
            let a2144 = v1;
            nat_lt(a2143, a2144)
        } else {
            let a2145 = v0;
            let a2146 = 2u64;
            if nat_eq(a2145, a2146) {
                let a2147 = v1;
                let a2148 = v2;
                nat_le(a2147, a2148)
            } else {
                let a2149 = v0;
                let a2150 = 3u64;
                if nat_eq(a2149, a2150) {
                    let a2151 = v1;
                    let a2152 = v2;
                    nat_lt(a2151, a2152)
                } else {
                    let a2153 = v1;
                    let a2154 = v2;
                    nat_eq(a2153, a2154)
                }
            }
        }
    }
}

pub fn f93(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2155 = v0;
    let a2156 = 0u64;
    if nat_eq(a2155, a2156) {
        Ok(f112(0u64))
    } else {
        let a2159 = {
            let a2157 = v0;
            let a2158 = 1u64;
            nat_add(a2157, a2158)?
        };
        let a2160 = f80(v1.clone(), 13u64, v2)?;
        if nat_eq(a2159, a2160) {
            Ok(Adt5::C4({
                let a2161 = v2;
                let a2162 = 0u64;
                if nat_eq(a2161, a2162) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2163 = v0;
                let a2164 = 1u64;
                nat_sub(a2163, a2164)
            };
            if f109(v1.clone(), v2) {
                let v4: u64 = f94(match v1.clone() {
                    Adt3::C0(h2165, _, _, _, _, _, _, _) => {
                        h2165
                    }
                }, false)?;
                let v5: u64 = {
                    let a2166 = v3;
                    let a2167 = v4;
                    let a2168 = 0u64;
                    nat_quot(a2166, a2167, a2168)
                };
                let v6: u64 = {
                    let a2169 = v3;
                    let a2170 = v4;
                    let a2171 = 0u64;
                    nat_rem(a2169, a2170, a2171)
                };
                let a2174 = {
                    let a2172 = v6;
                    let a2173 = 1u64;
                    nat_add(a2172, a2173)?
                };
                let a2175 = v4;
                if nat_eq(a2174, a2175) {
                    Ok(Adt5::C4({
                        let a2176 = v2;
                        let a2177 = 0u64;
                        if nat_eq(a2176, a2177) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f95(v6, v5, v1, v2)
                }
            } else {
                f113({
                    let a2178 = v3;
                    let a2179 = 2u64;
                    let a2180 = 0u64;
                    nat_quot(a2178, a2179, a2180)
                }, {
                    let a2181 = v3;
                    let a2182 = 2u64;
                    let a2183 = 0u64;
                    nat_rem(a2181, a2182, a2183)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f94(v0: u64, v1: bool) -> R<u64> {
    let a2186 = {
        let a2184 = v0;
        let a2185 = 2u64;
        nat_mul(a2184, a2185)?
    };
    let a2187 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2186, a2187)
}

pub fn f95(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2188 = v0;
    let a2189 = 0u64;
    if nat_eq(a2188, a2189) {
        Ok(f112(1u64))
    } else {
        f113({
            let a2198 = {
                let a2191 = v1;
                let a2192 = match v2.clone() {
                    Adt3::C0(h2190, _, _, _, _, _, _, _) => {
                        h2190
                    }
                };
                nat_mul(a2191, a2192)?
            };
            let a2199 = {
                let a2195 = {
                    let a2193 = v0;
                    let a2194 = 1u64;
                    nat_sub(a2193, a2194)
                };
                let a2196 = 2u64;
                let a2197 = 0u64;
                nat_quot(a2195, a2196, a2197)
            };
            nat_add(a2198, a2199)?
        }, {
            let a2202 = {
                let a2200 = v0;
                let a2201 = 1u64;
                nat_sub(a2200, a2201)
            };
            let a2203 = 2u64;
            let a2204 = 0u64;
            nat_rem(a2202, a2203, a2204)
        }, v2, v3, 1u64)
    }
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2205 = f69(v0);
    match m2205 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f114(v0) {
                let a2206 = v1;
                let a2207 = v7;
                nat_lt(a2206, a2207)
            } else {
                false
            } {
                f59(v8, v2)
            } else {
                false
            } {
                let a2208 = v3;
                let a2209 = f115(v4, v5, v6)?;
                nat_lt(a2208, a2209)
            } else {
                false
            } {
                let a2214 = {
                    let a2211 = v7;
                    let a2212 = match v13.clone() {
                        Adt11::C0(_, h2210, _) => {
                            h2210
                        }
                    };
                    let a2213 = 0u64;
                    nat_rem(a2211, a2212, a2213)
                };
                let a2215 = 0u64;
                nat_eq(a2214, a2215)
            } else {
                false
            } {
                Ok(Some({
                    let a2244 = {
                        let a2234 = {
                            let a2224 = {
                                let a2220 = {
                                    let a2217 = v1;
                                    let a2218 = match v13 {
                                        Adt11::C0(_, h2216, _) => {
                                            h2216
                                        }
                                    };
                                    let a2219 = 0u64;
                                    nat_quot(a2217, a2218, a2219)
                                };
                                let a2221 = v9;
                                nat_mul(a2220, a2221)?
                            };
                            let a2225 = {
                                let a2222 = v2;
                                let a2223 = v10;
                                nat_mul(a2222, a2223)?
                            };
                            nat_add(a2224, a2225)?
                        };
                        let a2235 = {
                            let a2232 = {
                                let a2229 = {
                                    let a2226 = v3;
                                    let a2227 = v4;
                                    let a2228 = 0u64;
                                    nat_quot(a2226, a2227, a2228)
                                };
                                let a2230 = v5;
                                let a2231 = 0u64;
                                nat_rem(a2229, a2230, a2231)
                            };
                            let a2233 = v11;
                            nat_mul(a2232, a2233)?
                        };
                        nat_add(a2234, a2235)?
                    };
                    let a2245 = {
                        let a2242 = {
                            let a2239 = {
                                let a2236 = v3;
                                let a2237 = v4;
                                let a2238 = 0u64;
                                nat_quot(a2236, a2237, a2238)
                            };
                            let a2240 = v5;
                            let a2241 = 0u64;
                            nat_quot(a2239, a2240, a2241)
                        };
                        let a2243 = v12;
                        nat_mul(a2242, a2243)?
                    };
                    nat_add(a2244, a2245)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f97(v0: u64) -> u64 {
    let a2246 = v0;
    let a2247 = 40u64;
    if nat_lt(a2246, a2247) {
        let a2248 = v0;
        let a2249 = 20u64;
        if nat_lt(a2248, a2249) {
            let a2250 = v0;
            let a2251 = 10u64;
            if nat_lt(a2250, a2251) {
                let a2252 = v0;
                let a2253 = 5u64;
                if nat_lt(a2252, a2253) {
                    let a2254 = v0;
                    let a2255 = 2u64;
                    if nat_lt(a2254, a2255) {
                        let a2256 = v0;
                        let a2257 = 1u64;
                        if nat_lt(a2256, a2257) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2258 = v0;
                        let a2259 = 3u64;
                        if nat_lt(a2258, a2259) {
                            4u64
                        } else {
                            let a2260 = v0;
                            let a2261 = 4u64;
                            if nat_lt(a2260, a2261) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2262 = v0;
                    let a2263 = 7u64;
                    if nat_lt(a2262, a2263) {
                        let a2264 = v0;
                        let a2265 = 6u64;
                        if nat_lt(a2264, a2265) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2266 = v0;
                        let a2267 = 8u64;
                        if nat_lt(a2266, a2267) {
                            128u64
                        } else {
                            let a2268 = v0;
                            let a2269 = 9u64;
                            if nat_lt(a2268, a2269) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2270 = v0;
                let a2271 = 15u64;
                if nat_lt(a2270, a2271) {
                    let a2272 = v0;
                    let a2273 = 12u64;
                    if nat_lt(a2272, a2273) {
                        let a2274 = v0;
                        let a2275 = 11u64;
                        if nat_lt(a2274, a2275) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2276 = v0;
                        let a2277 = 13u64;
                        if nat_lt(a2276, a2277) {
                            4096u64
                        } else {
                            let a2278 = v0;
                            let a2279 = 14u64;
                            if nat_lt(a2278, a2279) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2280 = v0;
                    let a2281 = 17u64;
                    if nat_lt(a2280, a2281) {
                        let a2282 = v0;
                        let a2283 = 16u64;
                        if nat_lt(a2282, a2283) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2284 = v0;
                        let a2285 = 18u64;
                        if nat_lt(a2284, a2285) {
                            131072u64
                        } else {
                            let a2286 = v0;
                            let a2287 = 19u64;
                            if nat_lt(a2286, a2287) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2288 = v0;
            let a2289 = 30u64;
            if nat_lt(a2288, a2289) {
                let a2290 = v0;
                let a2291 = 25u64;
                if nat_lt(a2290, a2291) {
                    let a2292 = v0;
                    let a2293 = 22u64;
                    if nat_lt(a2292, a2293) {
                        let a2294 = v0;
                        let a2295 = 21u64;
                        if nat_lt(a2294, a2295) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2296 = v0;
                        let a2297 = 23u64;
                        if nat_lt(a2296, a2297) {
                            4194304u64
                        } else {
                            let a2298 = v0;
                            let a2299 = 24u64;
                            if nat_lt(a2298, a2299) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2300 = v0;
                    let a2301 = 27u64;
                    if nat_lt(a2300, a2301) {
                        let a2302 = v0;
                        let a2303 = 26u64;
                        if nat_lt(a2302, a2303) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2304 = v0;
                        let a2305 = 28u64;
                        if nat_lt(a2304, a2305) {
                            134217728u64
                        } else {
                            let a2306 = v0;
                            let a2307 = 29u64;
                            if nat_lt(a2306, a2307) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2308 = v0;
                let a2309 = 35u64;
                if nat_lt(a2308, a2309) {
                    let a2310 = v0;
                    let a2311 = 32u64;
                    if nat_lt(a2310, a2311) {
                        let a2312 = v0;
                        let a2313 = 31u64;
                        if nat_lt(a2312, a2313) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2314 = v0;
                        let a2315 = 33u64;
                        if nat_lt(a2314, a2315) {
                            4294967296u64
                        } else {
                            let a2316 = v0;
                            let a2317 = 34u64;
                            if nat_lt(a2316, a2317) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2318 = v0;
                    let a2319 = 37u64;
                    if nat_lt(a2318, a2319) {
                        let a2320 = v0;
                        let a2321 = 36u64;
                        if nat_lt(a2320, a2321) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2322 = v0;
                        let a2323 = 38u64;
                        if nat_lt(a2322, a2323) {
                            137438953472u64
                        } else {
                            let a2324 = v0;
                            let a2325 = 39u64;
                            if nat_lt(a2324, a2325) {
                                274877906944u64
                            } else {
                                549755813888u64
                            }
                        }
                    }
                }
            }
        }
    } else {
        0u64
    }
}

pub fn f98(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a2326 = v7;
        let a2327 = 6u64;
        if nat_eq(a2326, a2327) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f116(v3, v4), f116(v5, v6), f117(v7, v8), f118(v1), f118(v9))
}

pub fn f99(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m2328 = v0;
    match m2328 {
        Adt4::C0 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C1 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C2(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C3(_, _, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C4(_) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C5(_, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C6(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C7(_) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C8(_) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C9(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C10(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C11(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C12(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C13(v30, v31, v32, v33) => {
            Adt7::C4(v30, v31, v32, v33)
        }
        Adt4::C14(_, _, _) => {
            Adt7::C3(v1, v2, true)
        }
        Adt4::C15 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C16(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C17 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C18(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C19(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C20(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C21 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C22(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C23(_, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
    }
}

pub fn f100(v0: Adt4) -> u64 {
    let m2329 = v0;
    match m2329 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1 => {
            0u64
        }
        Adt4::C2(_, _) => {
            0u64
        }
        Adt4::C3(_, _, _, _, _) => {
            0u64
        }
        Adt4::C4(_) => {
            0u64
        }
        Adt4::C5(_, _, _, _) => {
            0u64
        }
        Adt4::C6(_, _, _) => {
            0u64
        }
        Adt4::C7(_) => {
            0u64
        }
        Adt4::C8(_) => {
            0u64
        }
        Adt4::C9(_, _) => {
            0u64
        }
        Adt4::C10(_, _, _) => {
            0u64
        }
        Adt4::C11(_, _) => {
            0u64
        }
        Adt4::C12(_, _, _) => {
            0u64
        }
        Adt4::C13(_, _, _, _) => {
            0u64
        }
        Adt4::C14(v32, _, _) => {
            v32
        }
        Adt4::C15 => {
            0u64
        }
        Adt4::C16(_, _) => {
            0u64
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(_, _) => {
            0u64
        }
        Adt4::C20(_, _) => {
            0u64
        }
        Adt4::C21 => {
            0u64
        }
        Adt4::C22(_, _) => {
            0u64
        }
        Adt4::C23(_, _, _, _) => {
            0u64
        }
    }
}

pub fn f101(v0: Adt4, v1: u64) -> u64 {
    let m2330 = v0;
    match m2330 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1 => {
            0u64
        }
        Adt4::C2(_, _) => {
            0u64
        }
        Adt4::C3(_, _, _, _, _) => {
            0u64
        }
        Adt4::C4(_) => {
            0u64
        }
        Adt4::C5(_, _, _, _) => {
            0u64
        }
        Adt4::C6(_, _, _) => {
            0u64
        }
        Adt4::C7(_) => {
            0u64
        }
        Adt4::C8(_) => {
            0u64
        }
        Adt4::C9(_, _) => {
            0u64
        }
        Adt4::C10(_, _, _) => {
            0u64
        }
        Adt4::C11(_, _) => {
            0u64
        }
        Adt4::C12(_, _, _) => {
            0u64
        }
        Adt4::C13(_, _, _, _) => {
            0u64
        }
        Adt4::C14(_, _, _) => {
            0u64
        }
        Adt4::C15 => {
            0u64
        }
        Adt4::C16(_, v37) => {
            v37
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v40, _) => {
            let a2331 = v1;
            let a2332 = 6u64;
            if nat_eq(a2331, a2332) {
                v40
            } else {
                0u64
            }
        }
        Adt4::C20(_, _) => {
            0u64
        }
        Adt4::C21 => {
            0u64
        }
        Adt4::C22(_, _) => {
            0u64
        }
        Adt4::C23(_, _, _, _) => {
            0u64
        }
    }
}

pub fn f102(v0: Adt4, v1: u64) -> Adt7 {
    let a2333 = v1;
    let a2334 = 4u64;
    if nat_lt(a2333, a2334) {
        Adt7::C0(v1)
    } else {
        let a2335 = v1;
        let a2336 = 4u64;
        if nat_eq(a2335, a2336) {
            Adt7::C1(0u64)
        } else {
            let a2337 = v1;
            let a2338 = 5u64;
            if nat_eq(a2337, a2338) {
                Adt7::C1(3u64)
            } else {
                let a2339 = v1;
                let a2340 = 6u64;
                if nat_eq(a2339, a2340) {
                    Adt7::C2(f101(v0, v1), 0u64, false)
                } else {
                    let a2341 = v1;
                    let a2342 = 7u64;
                    if nat_eq(a2341, a2342) {
                        Adt7::C2(0u64, f101(v0, v1), false)
                    } else {
                        let a2343 = v1;
                        let a2344 = 16u64;
                        if nat_lt(a2343, a2344) {
                            Adt7::C1({
                                let a2345 = v1;
                                let a2346 = 8u64;
                                nat_sub(a2345, a2346)
                            })
                        } else {
                            let a2347 = v1;
                            let a2348 = 20u64;
                            if nat_lt(a2347, a2348) {
                                Adt7::C5({
                                    let a2349 = v1;
                                    let a2350 = 16u64;
                                    nat_sub(a2349, a2350)
                                })
                            } else {
                                let a2351 = v1;
                                let a2352 = 20u64;
                                if nat_eq(a2351, a2352) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    Adt7::C6
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f119(v0, v1, v6, v7, v8) {
        let a2353 = v5;
        let a2354 = 26u64;
        nat_eq(a2353, a2354)
    } else {
        false
    } {
        let a2355 = v3;
        let a2356 = v7;
        nat_eq(a2355, a2356)
    } else {
        false
    } {
        let a2357 = v4;
        let a2358 = v8;
        nat_eq(a2357, a2358)
    } else {
        false
    } {
        let a2359 = v9;
        let a2360 = 1u64;
        nat_eq(a2359, a2360)
    } else {
        false
    } {
        let a2361 = v10;
        let a2362 = f120(v0);
        nat_eq(a2361, a2362)
    } else {
        false
    } {
        let a2363 = v11;
        let a2364 = v1;
        nat_eq(a2363, a2364)
    } else {
        false
    } {
        let a2365 = v12;
        let a2366 = v6;
        nat_eq(a2365, a2366)
    } else {
        false
    } {
        let a2367 = v13;
        let a2368 = v7;
        nat_eq(a2367, a2368)
    } else {
        false
    } {
        let a2369 = v14;
        let a2370 = v8;
        nat_eq(a2369, a2370)
    } else {
        false
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2371 = v0;
    let a2372 = 0u64;
    if if if nat_eq(a2371, a2372) {
        true
    } else {
        let a2373 = v1;
        let a2374 = 0u64;
        nat_eq(a2373, a2374)
    } {
        true
    } else {
        let a2375 = v2;
        let a2376 = 0u64;
        nat_eq(a2375, a2376)
    } {
        true
    } else {
        let a2377 = v3;
        let a2378 = 0u64;
        nat_eq(a2377, a2378)
    }
}

pub fn f105(v0: u64) -> u64 {
    let a2379 = v0;
    let a2380 = 2147483648u64;
    if nat_lt(a2379, a2380) {
        v0
    } else {
        0u64
    }
}

pub fn f106(v0: u64) -> u64 {
    let a2381 = v0;
    let a2382 = 2147483648u64;
    if nat_lt(a2381, a2382) {
        0u64
    } else {
        let a2383 = 4294967296u64;
        let a2384 = v0;
        nat_sub(a2383, a2384)
    }
}

pub fn f107(v0: Adt3, v1: bool) -> R<u64> {
    f111(v0.clone(), {
        let a2386 = match v0.clone() {
            Adt3::C0(h2385, _, _, _, _, _, _, _) => {
                h2385
            }
        };
        let a2387 = 1u64;
        nat_sub(a2386, a2387)
    }, {
        let a2389 = match v0.clone() {
            Adt3::C0(_, h2388, _, _, _, _, _, _) => {
                h2388
            }
        };
        let a2390 = 1u64;
        nat_sub(a2389, a2390)
    }, {
        let a2392 = match v0.clone() {
            Adt3::C0(_, _, h2391, _, _, _, _, _) => {
                h2391
            }
        };
        let a2393 = 1u64;
        nat_sub(a2392, a2393)
    }, {
        let a2395 = match v0 {
            Adt3::C0(_, _, _, h2394, _, _, _, _) => {
                h2394
            }
        };
        let a2396 = 1u64;
        nat_sub(a2395, a2396)
    }, v1)
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2397 = v0;
    let a2398 = 0u64;
    if nat_eq(a2397, a2398) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2399 = v0;
            let a2400 = 1u64;
            nat_sub(a2399, a2400)
        };
        let v5: u64 = {
            let a2401 = v4;
            let a2402 = v1;
            let a2403 = 0u64;
            nat_quot(a2401, a2402, a2403)
        };
        let v6: u64 = {
            let a2411 = {
                let a2404 = v5;
                let a2405 = v2;
                nat_mul(a2404, a2405)?
            };
            let a2412 = {
                let a2409 = {
                    let a2406 = v4;
                    let a2407 = v1;
                    let a2408 = 0u64;
                    nat_rem(a2406, a2407, a2408)
                };
                let a2410 = v3;
                nat_mul(a2409, a2410)?
            };
            nat_add(a2411, a2412)?
        };
        let a2413 = v5;
        let a2414 = 0u64;
        if nat_eq(a2413, a2414) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2423 = {
                    let a2417 = {
                        let a2415 = v5;
                        let a2416 = 1u64;
                        nat_sub(a2415, a2416)
                    };
                    let a2418 = v2;
                    nat_mul(a2417, a2418)?
                };
                let a2424 = {
                    let a2421 = {
                        let a2419 = v1;
                        let a2420 = 1u64;
                        nat_sub(a2419, a2420)
                    };
                    let a2422 = v3;
                    nat_mul(a2421, a2422)?
                };
                nat_add(a2423, a2424)?
            };
            let a2425 = v6;
            let a2426 = v7;
            if nat_lt(a2425, a2426) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f109(v0: Adt3, v1: u64) -> bool {
    let a2427 = v1;
    let a2428 = 0u64;
    if !nat_eq(a2427, a2428) {
        true
    } else {
        let a2430 = match v0 {
            Adt3::C0(_, _, _, _, h2429, _, _, _) => {
                h2429
            }
        };
        let a2431 = 4u64;
        nat_eq(a2430, a2431)
    }
}

pub fn f110(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2432 = f121(v0, v1);
    match m2432 {
        None => {
            if v5 {
                let m2433 = f122(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2433 {
                    Adt12::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt12::C1 => {
                        Ok(Adt6::C1)
                    }
                    Adt12::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt6::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a2434 = 4u64;
                let a2435 = v28;
                nat_le(a2434, a2435)
            };
            let a2436 = v2;
            let a2437 = 0u64;
            if if if if if nat_eq(a2436, a2437) {
                let a2438 = v4;
                let a2439 = 0u64;
                nat_eq(a2438, a2439)
            } else {
                false
            } {
                let a2440 = v6;
                let a2441 = v14;
                if nat_eq(a2440, a2441) {
                    let a2442 = v7;
                    let a2443 = v15;
                    nat_eq(a2442, a2443)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2444 = v8;
                let a2445 = v16;
                if nat_eq(a2444, a2445) {
                    let a2446 = v9;
                    let a2447 = v17;
                    nat_eq(a2446, a2447)
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
                        let a2448 = v3;
                        let a2449 = 0u64;
                        nat_eq(a2448, a2449)
                    } else {
                        false
                    } {
                        f70(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2454 = {
                        let a2450 = v6;
                        let a2451 = v7;
                        nat_mul(a2450, a2451)?
                    };
                    let a2455 = {
                        let a2452 = v8;
                        let a2453 = v9;
                        nat_mul(a2452, a2453)?
                    };
                    nat_mul(a2454, a2455)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f111(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2476 = {
        let a2470 = {
            let a2464 = {
                let a2458 = v1;
                let a2459 = if v5 {
                    f106(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2456, _, _, _) => {
                            h2456
                        }
                    })
                } else {
                    f105(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2457, _, _, _) => {
                            h2457
                        }
                    })
                };
                nat_mul(a2458, a2459)?
            };
            let a2465 = {
                let a2462 = v2;
                let a2463 = if v5 {
                    f106(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2460, _, _) => {
                            h2460
                        }
                    })
                } else {
                    f105(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2461, _, _) => {
                            h2461
                        }
                    })
                };
                nat_mul(a2462, a2463)?
            };
            nat_add(a2464, a2465)?
        };
        let a2471 = {
            let a2468 = v3;
            let a2469 = if v5 {
                f106(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2466, _) => {
                        h2466
                    }
                })
            } else {
                f105(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2467, _) => {
                        h2467
                    }
                })
            };
            nat_mul(a2468, a2469)?
        };
        nat_add(a2470, a2471)?
    };
    let a2477 = {
        let a2474 = v4;
        let a2475 = if v5 {
            f106(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2472) => {
                    h2472
                }
            })
        } else {
            f105(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2473) => {
                    h2473
                }
            })
        };
        nat_mul(a2474, a2475)?
    };
    nat_add(a2476, a2477)
}

pub fn f112(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f113(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2478 = v1;
    let a2479 = 0u64;
    if nat_eq(a2478, a2479) {
        Ok(Adt5::C4({
            let a2480 = v3;
            let a2481 = 0u64;
            if nat_eq(a2480, a2481) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f41(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2482 = v3;
            let a2483 = 0u64;
            if nat_eq(a2482, a2483) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f114(v0: u64) -> bool {
    let a2484 = v0;
    let a2485 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2484, a2485) {
        true
    } else {
        let a2486 = v0;
        let a2487 = 1u64;
        nat_eq(a2486, a2487)
    } {
        true
    } else {
        let a2488 = v0;
        let a2489 = 2u64;
        nat_eq(a2488, a2489)
    } {
        true
    } else {
        let a2490 = v0;
        let a2491 = 3u64;
        nat_eq(a2490, a2491)
    } {
        true
    } else {
        let a2492 = v0;
        let a2493 = 6u64;
        nat_eq(a2492, a2493)
    } {
        true
    } else {
        let a2494 = v0;
        let a2495 = 7u64;
        nat_eq(a2494, a2495)
    } {
        true
    } else {
        let a2496 = v0;
        let a2497 = 8u64;
        nat_eq(a2496, a2497)
    } {
        true
    } else {
        let a2498 = v0;
        let a2499 = 10u64;
        nat_eq(a2498, a2499)
    } {
        true
    } else {
        let a2500 = v0;
        let a2501 = 11u64;
        nat_eq(a2500, a2501)
    } {
        true
    } else {
        let a2502 = v0;
        let a2503 = 12u64;
        nat_eq(a2502, a2503)
    } {
        true
    } else {
        let a2504 = v0;
        let a2505 = 13u64;
        nat_eq(a2504, a2505)
    } {
        true
    } else {
        let a2506 = v0;
        let a2507 = 14u64;
        nat_eq(a2506, a2507)
    } {
        true
    } else {
        let a2508 = v0;
        let a2509 = 16u64;
        nat_eq(a2508, a2509)
    } {
        true
    } else {
        let a2510 = v0;
        let a2511 = 17u64;
        nat_eq(a2510, a2511)
    } {
        true
    } else {
        let a2512 = v0;
        let a2513 = 18u64;
        nat_eq(a2512, a2513)
    } {
        true
    } else {
        let a2514 = v0;
        let a2515 = 19u64;
        nat_eq(a2514, a2515)
    } {
        true
    } else {
        let a2516 = v0;
        let a2517 = 20u64;
        nat_eq(a2516, a2517)
    } {
        true
    } else {
        let a2518 = v0;
        let a2519 = 21u64;
        nat_eq(a2518, a2519)
    } {
        true
    } else {
        let a2520 = v0;
        let a2521 = 22u64;
        nat_eq(a2520, a2521)
    } {
        true
    } else {
        let a2522 = v0;
        let a2523 = 23u64;
        nat_eq(a2522, a2523)
    } {
        true
    } else {
        let a2524 = v0;
        let a2525 = 26u64;
        nat_eq(a2524, a2525)
    } {
        true
    } else {
        let a2526 = v0;
        let a2527 = 29u64;
        nat_eq(a2526, a2527)
    } {
        true
    } else {
        let a2528 = v0;
        let a2529 = 30u64;
        nat_eq(a2528, a2529)
    } {
        true
    } else {
        let a2530 = v0;
        let a2531 = 34u64;
        nat_eq(a2530, a2531)
    } {
        true
    } else {
        let a2532 = v0;
        let a2533 = 35u64;
        nat_eq(a2532, a2533)
    } {
        true
    } else {
        let a2534 = v0;
        let a2535 = 39u64;
        nat_eq(a2534, a2535)
    } {
        true
    } else {
        let a2536 = v0;
        let a2537 = 40u64;
        nat_eq(a2536, a2537)
    } {
        true
    } else {
        let a2538 = v0;
        let a2539 = 41u64;
        nat_eq(a2538, a2539)
    } {
        true
    } else {
        let a2540 = v0;
        let a2541 = 42u64;
        nat_eq(a2540, a2541)
    } {
        true
    } else {
        let a2542 = v0;
        let a2543 = 43u64;
        nat_eq(a2542, a2543)
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2544 = v0;
    let a2545 = 0u64;
    if if if nat_eq(a2544, a2545) {
        true
    } else {
        let a2546 = v1;
        let a2547 = 0u64;
        nat_eq(a2546, a2547)
    } {
        true
    } else {
        let a2548 = v2;
        let a2549 = 0u64;
        nat_eq(a2548, a2549)
    } {
        Ok(0u64)
    } else {
        let a2552 = {
            let a2550 = v0;
            let a2551 = v1;
            nat_mul(a2550, a2551)?
        };
        let a2553 = v2;
        nat_mul(a2552, a2553)
    }
}

pub fn f116(v0: u64, v1: u64) -> Adt8 {
    let a2554 = v0;
    let a2555 = 0u64;
    if nat_eq(a2554, a2555) {
        Adt8::C0(0u64, v1)
    } else {
        let a2556 = v0;
        let a2557 = 1u64;
        if nat_eq(a2556, a2557) {
            Adt8::C1(v1)
        } else {
            let a2558 = v0;
            let a2559 = 2u64;
            if nat_eq(a2558, a2559) {
                Adt8::C2(v1)
            } else {
                let a2560 = v0;
                let a2561 = 3u64;
                if nat_eq(a2560, a2561) {
                    Adt8::C3(v1)
                } else {
                    let a2562 = v0;
                    let a2563 = 13u64;
                    if nat_lt(a2562, a2563) {
                        Adt8::C0({
                            let a2564 = v0;
                            let a2565 = 3u64;
                            nat_sub(a2564, a2565)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f117(v0: u64, v1: u64) -> Adt9 {
    let a2566 = v0;
    let a2567 = 1u64;
    if nat_eq(a2566, a2567) {
        Adt9::C0(v1)
    } else {
        let a2568 = v0;
        let a2569 = 3u64;
        if if nat_eq(a2568, a2569) {
            true
        } else {
            let a2570 = v0;
            let a2571 = 6u64;
            nat_eq(a2570, a2571)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f118(v0: u64) -> u64 {
    let m2572 = f69(v0);
    match m2572 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2574 = match v1.clone() {
                Adt11::C0(_, h2573, _) => {
                    h2573
                }
            };
            let a2575 = 1u64;
            if if nat_eq(a2574, a2575) {
                let a2577 = match v1.clone() {
                    Adt11::C0(_, _, h2576) => {
                        h2576
                    }
                };
                let a2578 = 1u64;
                if if if nat_eq(a2577, a2578) {
                    true
                } else {
                    let a2580 = match v1.clone() {
                        Adt11::C0(_, _, h2579) => {
                            h2579
                        }
                    };
                    let a2581 = 2u64;
                    nat_eq(a2580, a2581)
                } {
                    true
                } else {
                    let a2583 = match v1.clone() {
                        Adt11::C0(_, _, h2582) => {
                            h2582
                        }
                    };
                    let a2584 = 4u64;
                    nat_eq(a2583, a2584)
                } {
                    true
                } else {
                    let a2586 = match v1.clone() {
                        Adt11::C0(_, _, h2585) => {
                            h2585
                        }
                    };
                    let a2587 = 8u64;
                    nat_eq(a2586, a2587)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2588) => {
                        h2588
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f119(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f104(v1, v2, v3, v4) {
        f123(v0)
    } else {
        f114(v0)
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2589 = v0;
    let a2590 = 26u64;
    if nat_eq(a2589, a2590) {
        26u64
    } else {
        0u64
    }
}

pub fn f121(v0: u64, v1: u64) -> Option<u64> {
    let a2591 = v0;
    let a2592 = 2u64;
    if nat_eq(a2591, a2592) {
        Some(0u64)
    } else {
        let a2593 = v0;
        let a2594 = 5u64;
        if nat_eq(a2593, a2594) {
            Some(1u64)
        } else {
            let a2595 = v0;
            let a2596 = 6u64;
            if nat_eq(a2595, a2596) {
                Some(2u64)
            } else {
                let a2597 = v0;
                let a2598 = 7u64;
                if nat_eq(a2597, a2598) {
                    Some(3u64)
                } else {
                    let a2599 = v0;
                    let a2600 = 9u64;
                    if nat_eq(a2599, a2600) {
                        Some(4u64)
                    } else {
                        let a2601 = v0;
                        let a2602 = 8u64;
                        if nat_eq(a2601, a2602) {
                            Some(5u64)
                        } else {
                            let a2603 = v0;
                            let a2604 = 89u64;
                            if nat_eq(a2603, a2604) {
                                let a2605 = v1;
                                let a2606 = 2u64;
                                if nat_eq(a2605, a2606) {
                                    Some(6u64)
                                } else {
                                    let a2607 = v1;
                                    let a2608 = 0u64;
                                    if nat_eq(a2607, a2608) {
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

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a2609 = v0;
    let a2610 = 28u64;
    if if if if nat_eq(a2609, a2610) {
        let a2611 = v1;
        let a2612 = 0u64;
        nat_eq(a2611, a2612)
    } else {
        false
    } {
        let a2613 = v2;
        let a2614 = 0u64;
        nat_eq(a2613, a2614)
    } else {
        false
    } {
        let a2615 = v3;
        let a2616 = 0u64;
        nat_eq(a2615, a2616)
    } else {
        false
    } {
        let a2617 = v4;
        let a2618 = v8;
        if if if if if if if if nat_eq(a2617, a2618) {
            let a2619 = v5;
            let a2620 = v12;
            nat_eq(a2619, a2620)
        } else {
            false
        } {
            let a2621 = v9;
            let a2622 = v13;
            nat_eq(a2621, a2622)
        } else {
            false
        } {
            let a2623 = v10;
            let a2624 = v14;
            nat_eq(a2623, a2624)
        } else {
            false
        } {
            let a2625 = v11;
            let a2626 = v15;
            nat_eq(a2625, a2626)
        } else {
            false
        } {
            let a2627 = 0u64;
            let a2628 = v6;
            nat_lt(a2627, a2628)
        } else {
            false
        } {
            let a2629 = 0u64;
            let a2630 = v7;
            nat_lt(a2629, a2630)
        } else {
            false
        } {
            let a2634 = {
                let a2631 = v10;
                let a2632 = v6;
                let a2633 = 1u64;
                nat_rem(a2631, a2632, a2633)
            };
            let a2635 = 0u64;
            if nat_eq(a2634, a2635) {
                let a2639 = {
                    let a2636 = v11;
                    let a2637 = v7;
                    let a2638 = 1u64;
                    nat_rem(a2636, a2637, a2638)
                };
                let a2640 = 0u64;
                nat_eq(a2639, a2640)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a2641 = v10;
                let a2642 = v6;
                let a2643 = 0u64;
                nat_quot(a2641, a2642, a2643)
            }, {
                let a2644 = v11;
                let a2645 = v7;
                let a2646 = 0u64;
                nat_quot(a2644, a2645, a2646)
            }, {
                let a2647 = v5;
                let a2648 = v4;
                nat_mul(a2647, a2648)?
            }, {
                let a2649 = v4;
                let a2650 = v9;
                nat_mul(a2649, a2650)?
            }, {
                let a2651 = v5;
                let a2652 = v9;
                nat_mul(a2651, a2652)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f123(v0: u64) -> bool {
    let m2653 = f69(v0);
    match m2653 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}
