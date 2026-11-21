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
    C24(u64),
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
                        let a6 = 27u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt5::C0(f2(v2, v14, v13)?))
                        } else {
                            let a7 = v1;
                            let a8 = 3u64;
                            if nat_eq(a7, a8) {
                                Ok(Adt5::C0(f3(v14, v4, v5, v13)?))
                            } else {
                                f4(v1, v2, v14, v4, v13)
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v15, v16, v17, v18) => {
            f5(v15, v16, v17, v18)
        }
        Adt0::C2(v19, v20, v21, v22) => {
            f6(v19, v20, v21, v22)
        }
        Adt0::C3(v23, v24, v25, v26, v27) => {
            f7(v23, v24, v25, v26, v27)
        }
        Adt0::C4(v28, v29, v30, v31, v32, v33) => {
            f8(v28, v29, v30, v31, v32, v33)
        }
        Adt0::C5(v34, v35, v36, v37, v38, v39) => {
            f9(v34, v35, v36, v37, v38, v39)
        }
        Adt0::C6(v40, v41, v42, v43, v44, v45) => {
            f10(v40, v41, v42, v43, v44, v45)
        }
        Adt0::C7(v46, v47, v48, v49, v50) => {
            f11(v46, v47, v48, v49, v50)
        }
        Adt0::C8(v51) => {
            Ok(f12(v51))
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
    let v2: u64 = f13(v1.clone(), 1u64)?;
    let a10 = match v1.clone() {
        Adt2::C0(h9, _, _, _, _, _, _, _, _, _, _) => {
            h9
        }
    };
    let a11 = 0u64;
    if if if if if if if if nat_eq(a10, a11) {
        let a13 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h12, _, _, _, _, _) => {
                h12
            }
        };
        let a14 = 4u64;
        nat_eq(a13, a14)
    } else {
        false
    } {
        f14(match v0.clone() {
            Adt1::C0(h15, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h15
            }
        })
    } else {
        false
    } {
        f14(match v0.clone() {
            Adt1::C0(_, h16, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h16
            }
        })
    } else {
        false
    } {
        let a18 = 0u64;
        let a19 = match v0.clone() {
            Adt1::C0(_, _, h17, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h17
            }
        };
        nat_lt(a18, a19)
    } else {
        false
    } {
        let a21 = match v0.clone() {
            Adt1::C0(_, _, h20, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h20
            }
        };
        let a22 = 2139095040u64;
        nat_lt(a21, a22)
    } else {
        false
    } {
        let a23 = v2;
        let a24 = 9223372036854775808u64;
        nat_lt(a23, a24)
    } else {
        false
    } {
        let a26 = f15(v2)?;
        let a27 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h25, _) => {
                h25
            }
        };
        nat_le(a26, a27)
    } else {
        false
    } {
        Ok(Adt6::C4(f16(v2)?, 4u64, f17(v1.clone(), 1u64), f17(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h28, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h28
            }
        }, match v0.clone() {
            Adt1::C0(_, h29, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h29
            }
        }, match v0 {
            Adt1::C0(_, _, h30, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h30
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a32 = match v1.clone() {
        Adt2::C0(h31, _, _, _, _, _, _, _, _, _, _) => {
            h31
        }
    };
    let a33 = 0u64;
    if if if if if nat_eq(a32, a33) {
        let a35 = match v2.clone() {
            Adt2::C0(h34, _, _, _, _, _, _, _, _, _, _) => {
                h34
            }
        };
        let a36 = 0u64;
        nat_eq(a35, a36)
    } else {
        false
    } {
        f18(v1.clone(), v2.clone())
    } else {
        false
    } {
        f19(match v0.clone() {
            Adt1::C0(h37, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h37
            }
        })
    } else {
        false
    } {
        if f20(v1.clone(), v2.clone())? {
            true
        } else {
            f21(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f17(v1, 1u64);
        let v4: u64 = {
            let a43 = match v3.clone() {
                Adt3::C0(_, h38, _, _, _, _, _, _) => {
                    h38
                }
            };
            let a44 = {
                let a41 = match v3.clone() {
                    Adt3::C0(_, _, h39, _, _, _, _, _) => {
                        h39
                    }
                };
                let a42 = match v3.clone() {
                    Adt3::C0(_, _, _, h40, _, _, _, _) => {
                        h40
                    }
                };
                nat_mul(a41, a42)?
            };
            nat_mul(a43, a44)?
        };
        Ok(Adt6::C4(f22(match v3.clone() {
            Adt3::C0(h45, _, _, _, _, _, _, _) => {
                h45
            }
        }, v4)?, 4u64, v3, f17(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h46, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h46
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f3(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m47 = v1;
    match m47 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m48 = v2;
            match m48 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a50 = match v0.clone() {
                        Adt2::C0(h49, _, _, _, _, _, _, _, _, _, _) => {
                            h49
                        }
                    };
                    let a51 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a50, a51) {
                        let a53 = match v4.clone() {
                            Adt2::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
                                h52
                            }
                        };
                        let a54 = 0u64;
                        nat_eq(a53, a54)
                    } else {
                        false
                    } {
                        let a56 = match v5.clone() {
                            Adt2::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
                                h55
                            }
                        };
                        let a57 = 26u64;
                        nat_eq(a56, a57)
                    } else {
                        false
                    } {
                        let a59 = match v3.clone() {
                            Adt2::C0(h58, _, _, _, _, _, _, _, _, _, _) => {
                                h58
                            }
                        };
                        let a60 = 0u64;
                        nat_eq(a59, a60)
                    } else {
                        false
                    } {
                        f18(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a63 = match v0.clone() {
                            Adt2::C0(_, h61, _, _, _, _, _, _, _, _, _) => {
                                h61
                            }
                        };
                        let a64 = match v4.clone() {
                            Adt2::C0(_, h62, _, _, _, _, _, _, _, _, _) => {
                                h62
                            }
                        };
                        nat_eq(a63, a64)
                    } else {
                        false
                    } {
                        let a67 = match v0.clone() {
                            Adt2::C0(_, _, h65, _, _, _, _, _, _, _, _) => {
                                h65
                            }
                        };
                        let a68 = match v5.clone() {
                            Adt2::C0(_, h66, _, _, _, _, _, _, _, _, _) => {
                                h66
                            }
                        };
                        nat_eq(a67, a68)
                    } else {
                        false
                    } {
                        let a71 = match v0.clone() {
                            Adt2::C0(_, _, _, h69, _, _, _, _, _, _, _) => {
                                h69
                            }
                        };
                        let a72 = match v5.clone() {
                            Adt2::C0(_, _, h70, _, _, _, _, _, _, _, _) => {
                                h70
                            }
                        };
                        nat_eq(a71, a72)
                    } else {
                        false
                    } {
                        let a74 = 0u64;
                        let a75 = match v5.clone() {
                            Adt2::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                                h73
                            }
                        };
                        nat_lt(a74, a75)
                    } else {
                        false
                    } {
                        let a77 = 0u64;
                        let a78 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                                h76
                            }
                        };
                        nat_lt(a77, a78)
                    } else {
                        false
                    } {
                        let a80 = 4u64;
                        let a81 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h79, _, _, _, _, _) => {
                                h79
                            }
                        };
                        nat_le(a80, a81)
                    } else {
                        false
                    } {
                        let a83 = 4u64;
                        let a84 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h82, _, _, _, _, _) => {
                                h82
                            }
                        };
                        nat_le(a83, a84)
                    } else {
                        false
                    } {
                        let a86 = 4u64;
                        let a87 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h85, _, _, _, _, _) => {
                                h85
                            }
                        };
                        nat_le(a86, a87)
                    } else {
                        false
                    } {
                        let a89 = 4u64;
                        let a90 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h88, _, _, _, _, _) => {
                                h88
                            }
                        };
                        nat_le(a89, a90)
                    } else {
                        false
                    } {
                        f20(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f20(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f20(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f23(match v0.clone() {
                            Adt2::C0(_, h91, _, _, _, _, _, _, _, _, _) => {
                                h91
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h92, _, _, _, _, _, _, _, _) => {
                                h92
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
                                h93
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h94, _, _, _, _, _, _) => {
                                h94
                            }
                        })?, 4u64, f17(v0, 1u64), f17(v3, 1u64), Adt4::C22(f17(v4, 1u64), f17(v5, 1u64))))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt5> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h95, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h95
        }
    };
    Ok(Adt5::C0({
        let a96 = v0;
        let a97 = 39u64;
        if nat_eq(a96, a97) {
            f24(v2, v3, v4)?
        } else {
            let a98 = v0;
            let a99 = 77u64;
            if nat_eq(a98, a99) {
                f25(v2, v4)?
            } else {
                let a100 = v0;
                let a101 = 40u64;
                if nat_eq(a100, a101) {
                    f26(v2, v3, v4)?
                } else {
                    if f27(v0) {
                        f28(v0, v2, {
                            let m102 = v3;
                            match m102 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a103 = v0;
                        let a104 = 19u64;
                        if nat_eq(a103, a104) {
                            f29(v2, v4)?
                        } else {
                            let a105 = v0;
                            let a106 = 21u64;
                            if nat_eq(a105, a106) {
                                f30(v5, v2, v3, v4)?
                            } else {
                                let a107 = v0;
                                let a108 = 61u64;
                                if if if nat_eq(a107, a108) {
                                    true
                                } else {
                                    let a109 = v0;
                                    let a110 = 62u64;
                                    nat_eq(a109, a110)
                                } {
                                    true
                                } else {
                                    let a111 = v0;
                                    let a112 = 63u64;
                                    nat_eq(a111, a112)
                                } {
                                    f31(v0, v1, v2, v4)?
                                } else {
                                    let a113 = v0;
                                    let a114 = 4u64;
                                    if if nat_eq(a113, a114) {
                                        true
                                    } else {
                                        let a115 = v0;
                                        let a116 = 32u64;
                                        nat_eq(a115, a116)
                                    } {
                                        f32(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a117 = v0;
                                        let a118 = 20u64;
                                        if nat_eq(a117, a118) {
                                            f33(v2, v4)?
                                        } else {
                                            let a119 = v0;
                                            let a120 = 42u64;
                                            if nat_eq(a119, a120) {
                                                f34(4u64, v2, v4)?
                                            } else {
                                                let a121 = v0;
                                                let a122 = 69u64;
                                                if nat_eq(a121, a122) {
                                                    let a123 = v5;
                                                    let a124 = 4u64;
                                                    if nat_lt(a123, a124) {
                                                        f34(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a125 = v0;
                                                    let a126 = 70u64;
                                                    if nat_eq(a125, a126) {
                                                        f35(v5, v2, v4)?
                                                    } else {
                                                        let a127 = v0;
                                                        let a128 = 75u64;
                                                        if if nat_eq(a127, a128) {
                                                            true
                                                        } else {
                                                            let a129 = v0;
                                                            let a130 = 76u64;
                                                            nat_eq(a129, a130)
                                                        } {
                                                            f36({
                                                                let a131 = v0;
                                                                let a132 = 76u64;
                                                                nat_eq(a131, a132)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a133 = v0;
                                                            let a134 = 31u64;
                                                            if if if nat_eq(a133, a134) {
                                                                true
                                                            } else {
                                                                let a135 = v0;
                                                                let a136 = 47u64;
                                                                nat_eq(a135, a136)
                                                            } {
                                                                true
                                                            } else {
                                                                let a137 = v0;
                                                                let a138 = 68u64;
                                                                nat_eq(a137, a138)
                                                            } {
                                                                f37(v0, v1, v2, v4)?
                                                            } else {
                                                                let a139 = v0;
                                                                let a140 = 15u64;
                                                                if nat_eq(a139, a140) {
                                                                    f38(v2, v4)?
                                                                } else {
                                                                    let a141 = v0;
                                                                    let a142 = 17u64;
                                                                    if nat_eq(a141, a142) {
                                                                        f39(v2, v4)?
                                                                    } else {
                                                                        let a143 = v0;
                                                                        let a144 = 18u64;
                                                                        if nat_eq(a143, a144) {
                                                                            f40(v2, v3, v4)?
                                                                        } else {
                                                                            let a145 = v0;
                                                                            let a146 = 13u64;
                                                                            if if if nat_eq(a145, a146) {
                                                                                true
                                                                            } else {
                                                                                let a147 = v0;
                                                                                let a148 = 14u64;
                                                                                nat_eq(a147, a148)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a149 = v0;
                                                                                let a150 = 16u64;
                                                                                nat_eq(a149, a150)
                                                                            } {
                                                                                f41(v0, v2, v4)?
                                                                            } else {
                                                                                let m151 = f42(v0, v5);
                                                                                match m151 {
                                                                                    None => {
                                                                                        f43(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f44(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f5(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m152 = v3;
    match m152 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f47(v0, v1, v2.clone())?, f45(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f48(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f49(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f50(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f49(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f51(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f52(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f53(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f45(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f54(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f55(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C15 => {
            f56(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f57(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f58(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f45(f46(v0, match v2.clone() {
                Adt3::C0(h153, _, _, _, _, _, _, _) => {
                    h153
                }
            }, match v2.clone() {
                Adt3::C0(_, h154, _, _, _, _, _, _) => {
                    h154
                }
            })?, v1)?, f45(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f6(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f59(match v1.clone() {
        Adt3::C0(h155, _, _, _, _, _, _, _) => {
            h155
        }
    })?;
    let v5: u64 = {
        let a156 = v0;
        let a157 = v4;
        let a158 = 0u64;
        nat_quot(a156, a157, a158)
    };
    let v6: u64 = {
        let a159 = v0;
        let a160 = v4;
        let a161 = 0u64;
        nat_rem(a159, a160, a161)
    };
    let a162 = v6;
    let a163 = 0u64;
    if nat_eq(a162, a163) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a164 = v6;
        let a165 = 1u64;
        if nat_eq(a164, a165) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a168 = {
                let a166 = v6;
                let a167 = 1u64;
                nat_add(a166, a167)?
            };
            let a169 = v4;
            if nat_eq(a168, a169) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f45(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a170 = v6;
                    let a171 = 2u64;
                    nat_sub(a170, a171)
                };
                let v8: u64 = {
                    let a172 = v7;
                    let a173 = 3u64;
                    let a174 = 0u64;
                    nat_quot(a172, a173, a174)
                };
                let v9: u64 = {
                    let a175 = v7;
                    let a176 = 3u64;
                    let a177 = 0u64;
                    nat_rem(a175, a176, a177)
                };
                let v10: u64 = f45({
                    let a181 = {
                        let a179 = v5;
                        let a180 = match v1.clone() {
                            Adt3::C0(h178, _, _, _, _, _, _, _) => {
                                h178
                            }
                        };
                        nat_mul(a179, a180)?
                    };
                    let a182 = v8;
                    nat_add(a181, a182)?
                }, v1)?;
                let a183 = v9;
                let a184 = 0u64;
                if nat_eq(a183, a184) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a185 = v9;
                    let a186 = 1u64;
                    if nat_eq(a185, a186) {
                        let a187 = v3;
                        let a188 = 2u64;
                        if nat_eq(a187, a188) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a189 = v3;
                        let a190 = 2u64;
                        if if nat_eq(a189, a190) {
                            true
                        } else {
                            let a191 = v3;
                            let a192 = 3u64;
                            nat_eq(a191, a192)
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

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a194 = 2u64;
        let a195 = match v1.clone() {
            Adt3::C0(_, h193, _, _, _, _, _, _) => {
                h193
            }
        };
        nat_mul(a194, a195)?
    };
    let a196 = v0;
    let a197 = v5;
    if nat_lt(a196, a197) {
        let a201 = {
            let a198 = v0;
            let a199 = 2u64;
            let a200 = 0u64;
            nat_rem(a198, a199, a200)
        };
        let a202 = 0u64;
        if nat_eq(a201, a202) {
            Ok(Adt5::C4(19u64, 4u64, f45({
                let a203 = v0;
                let a204 = 2u64;
                let a205 = 0u64;
                nat_quot(a203, a204, a205)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a206 = v4;
            let a207 = 2147483648u64;
            if if nat_lt(a206, a207) {
                let a209 = v4;
                let a210 = match v2 {
                    Adt3::C0(_, h208, _, _, _, _, _, _) => {
                        h208
                    }
                };
                nat_lt(a209, a210)
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
            let a213 = match v2.clone() {
                Adt3::C0(h211, _, _, _, _, _, _, _) => {
                    h211
                }
            };
            let a214 = match v2.clone() {
                Adt3::C0(_, h212, _, _, _, _, _, _) => {
                    h212
                }
            };
            nat_mul(a213, a214)?
        };
        let a217 = {
            let a215 = v0;
            let a216 = v5;
            nat_sub(a215, a216)
        };
        let a218 = v6;
        if nat_lt(a217, a218) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f45({
                let a219 = v0;
                let a220 = v5;
                nat_sub(a219, a220)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a223 = {
                    let a221 = v0;
                    let a222 = v5;
                    nat_sub(a221, a222)
                };
                let a224 = v6;
                nat_sub(a223, a224)
            };
            let v8: u64 = {
                let a228 = 1u64;
                let a229 = {
                    let a226 = 2u64;
                    let a227 = match v1.clone() {
                        Adt3::C0(h225, _, _, _, _, _, _, _) => {
                            h225
                        }
                    };
                    nat_mul(a226, a227)?
                };
                nat_add(a228, a229)?
            };
            let v9: u64 = {
                let a230 = v7;
                let a231 = v8;
                let a232 = 0u64;
                nat_quot(a230, a231, a232)
            };
            let v10: u64 = {
                let a233 = v7;
                let a234 = v8;
                let a235 = 0u64;
                nat_rem(a233, a234, a235)
            };
            let a236 = v10;
            let a237 = 0u64;
            if nat_eq(a236, a237) {
                Ok(Adt5::C4(19u64, 4u64, f45(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a241 = {
                    let a238 = v10;
                    let a239 = 2u64;
                    let a240 = 0u64;
                    nat_rem(a238, a239, a240)
                };
                let a242 = 1u64;
                if nat_eq(a241, a242) {
                    Ok(Adt5::C4(6u64, 0u64, f45({
                        let a251 = {
                            let a244 = v9;
                            let a245 = match v1.clone() {
                                Adt3::C0(h243, _, _, _, _, _, _, _) => {
                                    h243
                                }
                            };
                            nat_mul(a244, a245)?
                        };
                        let a252 = {
                            let a248 = {
                                let a246 = v10;
                                let a247 = 1u64;
                                nat_sub(a246, a247)
                            };
                            let a249 = 2u64;
                            let a250 = 0u64;
                            nat_quot(a248, a249, a250)
                        };
                        nat_add(a251, a252)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f45({
                        let a261 = {
                            let a254 = v4;
                            let a255 = match v1.clone() {
                                Adt3::C0(h253, _, _, _, _, _, _, _) => {
                                    h253
                                }
                            };
                            nat_mul(a254, a255)?
                        };
                        let a262 = {
                            let a258 = {
                                let a256 = v10;
                                let a257 = 1u64;
                                nat_sub(a256, a257)
                            };
                            let a259 = 2u64;
                            let a260 = 0u64;
                            nat_quot(a258, a259, a260)
                        };
                        nat_add(a261, a262)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f45({
                        let a271 = {
                            let a264 = v4;
                            let a265 = match v1 {
                                Adt3::C0(h263, _, _, _, _, _, _, _) => {
                                    h263
                                }
                            };
                            nat_mul(a264, a265)?
                        };
                        let a272 = {
                            let a268 = {
                                let a266 = v10;
                                let a267 = 1u64;
                                nat_sub(a266, a267)
                            };
                            let a269 = 2u64;
                            let a270 = 0u64;
                            nat_quot(a268, a269, a270)
                        };
                        nat_add(a271, a272)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a273 = v1;
    let a274 = f60(v2.clone(), v4.clone())?;
    if nat_lt(a273, a274) {
        let a277 = v1;
        let a278 = {
            let a275 = 2u64;
            let a276 = f61(v4.clone())?;
            nat_mul(a275, a276)?
        };
        if nat_lt(a277, a278) {
            let a282 = {
                let a279 = v1;
                let a280 = 2u64;
                let a281 = 0u64;
                nat_rem(a279, a280, a281)
            };
            let a283 = 0u64;
            if nat_eq(a282, a283) {
                f62({
                    let a284 = v1;
                    let a285 = 2u64;
                    let a286 = 0u64;
                    nat_quot(a284, a285, a286)
                }, v4)
            } else {
                if f63(match v2 {
                    Adt3::C0(_, h287, _, _, _, _, _, _) => {
                        h287
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a290 = v1;
                let a291 = {
                    let a288 = 2u64;
                    let a289 = f61(v4.clone())?;
                    nat_mul(a288, a289)?
                };
                nat_sub(a290, a291)
            };
            let v7: u64 = {
                let a295 = v6;
                let a296 = {
                    let a293 = 1u64;
                    let a294 = match v2.clone() {
                        Adt3::C0(h292, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    nat_add(a293, a294)?
                };
                let a297 = 0u64;
                nat_quot(a295, a296, a297)
            };
            let v8: u64 = {
                let a301 = v6;
                let a302 = {
                    let a299 = 1u64;
                    let a300 = match v2.clone() {
                        Adt3::C0(h298, _, _, _, _, _, _, _) => {
                            h298
                        }
                    };
                    nat_add(a299, a300)?
                };
                let a303 = 0u64;
                nat_rem(a301, a302, a303)
            };
            let a304 = v8;
            let a305 = 0u64;
            if nat_eq(a304, a305) {
                f62(v7, v4)
            } else {
                f64(v0, v7, {
                    let a306 = v8;
                    let a307 = 1u64;
                    nat_sub(a306, a307)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f9(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a310 = match v4.clone() {
            Adt3::C0(h308, _, _, _, _, _, _, _) => {
                h308
            }
        };
        let a311 = match v4.clone() {
            Adt3::C0(_, h309, _, _, _, _, _, _) => {
                h309
            }
        };
        nat_mul(a310, a311)?
    };
    let v7: u64 = {
        let a312 = 2u64;
        let a313 = v6;
        nat_mul(a312, a313)?
    };
    let v8: u64 = f65(match v3.clone() {
        Adt3::C0(_, h314, _, _, _, _, _, _) => {
            h314
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h315, _, _, _, _, _) => {
            h315
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h316, _, _, _, _) => {
            h316
        }
    });
    let a317 = v0;
    let a318 = v7;
    if nat_lt(a317, a318) {
        let a322 = {
            let a319 = v0;
            let a320 = 2u64;
            let a321 = 0u64;
            nat_rem(a319, a320, a321)
        };
        let a323 = 0u64;
        if nat_eq(a322, a323) {
            Ok(Adt5::C4(19u64, 5u64, f45({
                let a324 = v0;
                let a325 = 2u64;
                let a326 = 0u64;
                nat_quot(a324, a325, a326)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f66(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a327 = v0;
            let a328 = v7;
            nat_sub(a327, a328)
        };
        let v10: u64 = {
            let a332 = v9;
            let a333 = {
                let a330 = 1u64;
                let a331 = match v1.clone() {
                    Adt3::C0(h329, _, _, _, _, _, _, _) => {
                        h329
                    }
                };
                nat_add(a330, a331)?
            };
            let a334 = 0u64;
            nat_quot(a332, a333, a334)
        };
        let v11: u64 = {
            let a338 = v9;
            let a339 = {
                let a336 = 1u64;
                let a337 = match v1.clone() {
                    Adt3::C0(h335, _, _, _, _, _, _, _) => {
                        h335
                    }
                };
                nat_add(a336, a337)?
            };
            let a340 = 0u64;
            nat_rem(a338, a339, a340)
        };
        let a341 = v11;
        let a342 = 0u64;
        if nat_eq(a341, a342) {
            Ok(Adt5::C4(19u64, 5u64, f45({
                let a343 = v10;
                let a344 = v6;
                let a345 = 0u64;
                nat_rem(a343, a344, a345)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f66(v5, v8) {
                let v12: u64 = {
                    let a351 = {
                        let a347 = v10;
                        let a348 = match v1.clone() {
                            Adt3::C0(h346, _, _, _, _, _, _, _) => {
                                h346
                            }
                        };
                        nat_mul(a347, a348)?
                    };
                    let a352 = {
                        let a349 = v11;
                        let a350 = 1u64;
                        nat_sub(a349, a350)
                    };
                    nat_add(a351, a352)?
                };
                let a353 = v5;
                let a354 = 4294967295u64;
                if nat_eq(a353, a354) {
                    Ok(Adt5::C1(false, f45(v12, v1)?, f45(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f45(v12, v1)?, 4u64, f45({
                        let a360 = {
                            let a356 = v5;
                            let a357 = match v3.clone() {
                                Adt3::C0(h355, _, _, _, _, _, _, _) => {
                                    h355
                                }
                            };
                            nat_mul(a356, a357)?
                        };
                        let a361 = {
                            let a358 = v11;
                            let a359 = 1u64;
                            nat_sub(a358, a359)
                        };
                        nat_add(a360, a361)?
                    }, v3)?, 3u64, f45(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a362 = v0;
    let a363 = f16(v1)?;
    if nat_lt(a362, a363) {
        let a364 = v0;
        let a365 = 4u64;
        if nat_lt(a364, a365) {
            let a366 = v0;
            let a367 = 0u64;
            if nat_eq(a366, a367) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a368 = v0;
                let a369 = 1u64;
                if nat_eq(a368, a369) {
                    let a370 = v5;
                    let a371 = 0u64;
                    if nat_eq(a370, a371) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a372 = v0;
                    let a373 = 2u64;
                    if nat_eq(a372, a373) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a374 = v0;
            let a375 = 4u64;
            if if nat_eq(a374, a375) {
                let a376 = f67(v5)?;
                let a377 = v1;
                !nat_eq(a376, a377)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a380 = {
                        let a378 = v0;
                        let a379 = 4u64;
                        nat_sub(a378, a379)
                    };
                    let a381 = 3u64;
                    let a382 = 0u64;
                    nat_quot(a380, a381, a382)
                };
                let v7: u64 = {
                    let a385 = {
                        let a383 = v0;
                        let a384 = 4u64;
                        nat_sub(a383, a384)
                    };
                    let a386 = 3u64;
                    let a387 = 0u64;
                    nat_rem(a385, a386, a387)
                };
                let a388 = v7;
                let a389 = 0u64;
                if nat_eq(a388, a389) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a390 = v7;
                    let a391 = 1u64;
                    if nat_eq(a390, a391) {
                        Ok(Adt5::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt5::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f15(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f11(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt5> {
    Ok(f68(v3.clone(), {
        let m392 = v3.clone();
        match m392 {
            Adt4::C0 => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f6(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f7(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f8(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f5(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f9(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v50, v51, v52, v53) => {
                f10(v0, v53, v50, v51, v52, v4)?
            }
            Adt4::C24(v49) => {
                f69(v0, v1, v2, v49, v4)?
            }
        }
    }))
}

pub fn f12(v0: Adt4) -> Adt5 {
    let m393 = v0;
    match m393 {
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
        Adt4::C24(_) => {
            Adt5::C6(1u64, 0u64, 0u64, false)
        }
    }
}

pub fn f13(v0: Adt2, v1: u64) -> R<u64> {
    let a405 = {
        let a399 = {
            let a395 = match v0.clone() {
                Adt2::C0(_, h394, _, _, _, _, _, _, _, _, _) => {
                    h394
                }
            };
            let a396 = v1;
            let a397 = 0u64;
            nat_quot(a395, a396, a397)
        };
        let a400 = match v0.clone() {
            Adt2::C0(_, _, h398, _, _, _, _, _, _, _, _) => {
                h398
            }
        };
        nat_mul(a399, a400)?
    };
    let a406 = {
        let a403 = match v0.clone() {
            Adt2::C0(_, _, _, h401, _, _, _, _, _, _, _) => {
                h401
            }
        };
        let a404 = match v0 {
            Adt2::C0(_, _, _, _, h402, _, _, _, _, _, _) => {
                h402
            }
        };
        nat_mul(a403, a404)?
    };
    nat_mul(a405, a406)
}

pub fn f14(v0: u64) -> bool {
    let a407 = v0;
    let a408 = 4294967296u64;
    if nat_lt(a407, a408) {
        let a412 = {
            let a409 = v0;
            let a410 = 2147483648u64;
            let a411 = 0u64;
            nat_rem(a409, a410, a411)
        };
        let a413 = 2139095040u64;
        nat_lt(a412, a413)
    } else {
        false
    }
}

pub fn f15(v0: u64) -> R<u64> {
    let a414 = 4u64;
    let a415 = v0;
    nat_mul(a414, a415)
}

pub fn f16(v0: u64) -> R<u64> {
    let a416 = v0;
    let a417 = 0u64;
    if nat_eq(a416, a417) {
        Ok(0u64)
    } else {
        let a420 = 4u64;
        let a421 = {
            let a418 = 3u64;
            let a419 = v0;
            nat_mul(a418, a419)?
        };
        nat_add(a420, a421)
    }
}

pub fn f17(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a423 = match v0.clone() {
            Adt2::C0(_, h422, _, _, _, _, _, _, _, _, _) => {
                h422
            }
        };
        let a424 = v1;
        let a425 = 0u64;
        nat_quot(a423, a424, a425)
    }, match v0.clone() {
        Adt2::C0(_, _, h426, _, _, _, _, _, _, _, _) => {
            h426
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h427, _, _, _, _, _, _, _) => {
            h427
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h428, _, _, _, _, _, _) => {
            h428
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h429, _, _, _, _, _) => {
            h429
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h430, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h431, _, _, _) => {
            h431
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h432, _, _) => {
            h432
        }
    })
}

pub fn f18(v0: Adt2, v1: Adt2) -> bool {
    let a435 = match v0.clone() {
        Adt2::C0(_, h433, _, _, _, _, _, _, _, _, _) => {
            h433
        }
    };
    let a436 = match v1.clone() {
        Adt2::C0(_, h434, _, _, _, _, _, _, _, _, _) => {
            h434
        }
    };
    if if if nat_eq(a435, a436) {
        let a439 = match v0.clone() {
            Adt2::C0(_, _, h437, _, _, _, _, _, _, _, _) => {
                h437
            }
        };
        let a440 = match v1.clone() {
            Adt2::C0(_, _, h438, _, _, _, _, _, _, _, _) => {
                h438
            }
        };
        nat_eq(a439, a440)
    } else {
        false
    } {
        let a443 = match v0.clone() {
            Adt2::C0(_, _, _, h441, _, _, _, _, _, _, _) => {
                h441
            }
        };
        let a444 = match v1.clone() {
            Adt2::C0(_, _, _, h442, _, _, _, _, _, _, _) => {
                h442
            }
        };
        nat_eq(a443, a444)
    } else {
        false
    } {
        let a447 = match v0 {
            Adt2::C0(_, _, _, _, h445, _, _, _, _, _, _) => {
                h445
            }
        };
        let a448 = match v1 {
            Adt2::C0(_, _, _, _, h446, _, _, _, _, _, _) => {
                h446
            }
        };
        nat_eq(a447, a448)
    } else {
        false
    }
}

pub fn f19(v0: u64) -> bool {
    let a449 = v0;
    let a450 = 2139095040u64;
    if nat_le(a449, a450) {
        true
    } else {
        let a451 = v0;
        let a452 = 2147483648u64;
        nat_eq(a451, a452)
    }
}

pub fn f20(v0: Adt2, v1: Adt2) -> R<bool> {
    let a454 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h453, _) => {
            h453
        }
    };
    let a455 = 0u64;
    if nat_eq(a454, a455) {
        Ok(true)
    } else {
        let a457 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h456, _) => {
                h456
            }
        };
        let a458 = 0u64;
        if nat_eq(a457, a458) {
            Ok(true)
        } else {
            let a460 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h459) => {
                    h459
                }
            };
            let a461 = 0u64;
            if nat_eq(a460, a461) {
                Ok(true)
            } else {
                let a463 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h462) => {
                        h462
                    }
                };
                let a464 = 0u64;
                if nat_eq(a463, a464) {
                    Ok(true)
                } else {
                    let a470 = {
                        let a467 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h465) => {
                                h465
                            }
                        };
                        let a468 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h466, _) => {
                                h466
                            }
                        };
                        nat_add(a467, a468)?
                    };
                    let a471 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h469) => {
                            h469
                        }
                    };
                    if nat_le(a470, a471) {
                        Ok(true)
                    } else {
                        let a477 = {
                            let a474 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h472) => {
                                    h472
                                }
                            };
                            let a475 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h473, _) => {
                                    h473
                                }
                            };
                            nat_add(a474, a475)?
                        };
                        let a478 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h476) => {
                                h476
                            }
                        };
                        Ok(nat_le(a477, a478))
                    }
                }
            }
        }
    }
}

pub fn f21(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f18(v0.clone(), v1.clone()) {
        let a481 = match v0.clone() {
            Adt2::C0(h479, _, _, _, _, _, _, _, _, _, _) => {
                h479
            }
        };
        let a482 = match v1.clone() {
            Adt2::C0(h480, _, _, _, _, _, _, _, _, _, _) => {
                h480
            }
        };
        nat_eq(a481, a482)
    } else {
        false
    } {
        let a485 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h483) => {
                h483
            }
        };
        let a486 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h484) => {
                h484
            }
        };
        nat_eq(a485, a486)
    } else {
        false
    } {
        let a489 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h487, _, _, _, _, _) => {
                h487
            }
        };
        let a490 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h488, _, _, _, _, _) => {
                h488
            }
        };
        nat_eq(a489, a490)
    } else {
        false
    } {
        let a493 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h491, _, _, _, _) => {
                h491
            }
        };
        let a494 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h492, _, _, _, _) => {
                h492
            }
        };
        nat_eq(a493, a494)
    } else {
        false
    } {
        let a497 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h495, _, _, _) => {
                h495
            }
        };
        let a498 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h496, _, _, _) => {
                h496
            }
        };
        nat_eq(a497, a498)
    } else {
        false
    } {
        let a501 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h499, _, _) => {
                h499
            }
        };
        let a502 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h500, _, _) => {
                h500
            }
        };
        nat_eq(a501, a502)
    } else {
        false
    }
}

pub fn f22(v0: u64, v1: u64) -> R<u64> {
    let a503 = v0;
    let a504 = 0u64;
    if nat_eq(a503, a504) {
        Ok(0u64)
    } else {
        let a505 = v1;
        let a506 = f70(v0)?;
        nat_mul(a505, a506)
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a507 = v0;
    let a508 = 0u64;
    if if if if nat_eq(a507, a508) {
        true
    } else {
        let a509 = v1;
        let a510 = 0u64;
        nat_eq(a509, a510)
    } {
        true
    } else {
        let a511 = v2;
        let a512 = 0u64;
        nat_eq(a511, a512)
    } {
        true
    } else {
        let a513 = v3;
        let a514 = 0u64;
        nat_eq(a513, a514)
    } {
        Ok(0u64)
    } else {
        let a527 = {
            let a517 = 2u64;
            let a518 = {
                let a515 = v1;
                let a516 = v2;
                nat_mul(a515, a516)?
            };
            nat_mul(a517, a518)?
        };
        let a528 = {
            let a525 = {
                let a521 = {
                    let a519 = v1;
                    let a520 = v2;
                    nat_mul(a519, a520)?
                };
                let a522 = v3;
                nat_mul(a521, a522)?
            };
            let a526 = {
                let a523 = 1u64;
                let a524 = v0;
                nat_add(a523, a524)?
            };
            nat_mul(a525, a526)?
        };
        nat_add(a527, a528)
    }
}

pub fn f24(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m529 = v1;
    match m529 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f71(v0, v3, v2)
        }
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a531 = match v0.clone() {
        Adt2::C0(h530, _, _, _, _, _, _, _, _, _, _) => {
            h530
        }
    };
    let a532 = 1u64;
    if if nat_eq(a531, a532) {
        true
    } else {
        let a534 = match v0.clone() {
            Adt2::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
                h533
            }
        };
        let a535 = 30u64;
        nat_eq(a534, a535)
    } {
        let a537 = match v1.clone() {
            Adt2::C0(h536, _, _, _, _, _, _, _, _, _, _) => {
                h536
            }
        };
        let a538 = 1u64;
        if if if if if if if if if if if nat_eq(a537, a538) {
            let a541 = match v0.clone() {
                Adt2::C0(_, h539, _, _, _, _, _, _, _, _, _) => {
                    h539
                }
            };
            let a542 = match v1.clone() {
                Adt2::C0(_, h540, _, _, _, _, _, _, _, _, _) => {
                    h540
                }
            };
            nat_eq(a541, a542)
        } else {
            false
        } {
            let a544 = 0u64;
            let a545 = match v1.clone() {
                Adt2::C0(_, _, h543, _, _, _, _, _, _, _, _) => {
                    h543
                }
            };
            nat_lt(a544, a545)
        } else {
            false
        } {
            let a548 = match v1.clone() {
                Adt2::C0(_, _, h546, _, _, _, _, _, _, _, _) => {
                    h546
                }
            };
            let a549 = match v1.clone() {
                Adt2::C0(_, _, _, h547, _, _, _, _, _, _, _) => {
                    h547
                }
            };
            nat_eq(a548, a549)
        } else {
            false
        } {
            let a551 = match v1.clone() {
                Adt2::C0(_, _, _, _, h550, _, _, _, _, _, _) => {
                    h550
                }
            };
            let a552 = 1u64;
            nat_eq(a551, a552)
        } else {
            false
        } {
            let a560 = match v0.clone() {
                Adt2::C0(_, _, h553, _, _, _, _, _, _, _, _) => {
                    h553
                }
            };
            let a561 = {
                let a558 = {
                    let a556 = match v1.clone() {
                        Adt2::C0(_, _, h554, _, _, _, _, _, _, _, _) => {
                            h554
                        }
                    };
                    let a557 = match v1.clone() {
                        Adt2::C0(_, _, h555, _, _, _, _, _, _, _, _) => {
                            h555
                        }
                    };
                    nat_add(a556, a557)?
                };
                let a559 = 1u64;
                nat_sub(a558, a559)
            };
            nat_eq(a560, a561)
        } else {
            false
        } {
            let a563 = 0u64;
            let a564 = match v0.clone() {
                Adt2::C0(_, _, _, h562, _, _, _, _, _, _, _) => {
                    h562
                }
            };
            nat_lt(a563, a564)
        } else {
            false
        } {
            let a566 = 0u64;
            let a567 = match v0.clone() {
                Adt2::C0(_, _, _, _, h565, _, _, _, _, _, _) => {
                    h565
                }
            };
            nat_lt(a566, a567)
        } else {
            false
        } {
            let a569 = 2u64;
            let a570 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h568, _, _, _, _, _) => {
                    h568
                }
            };
            nat_le(a569, a570)
        } else {
            false
        } {
            let a572 = 2u64;
            let a573 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h571, _, _, _, _, _) => {
                    h571
                }
            };
            nat_le(a572, a573)
        } else {
            false
        } {
            f20(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f13(v1.clone(), 1u64)?, 2u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f26(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m574 = v1;
    match m574 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a576 = match v0.clone() {
                Adt2::C0(h575, _, _, _, _, _, _, _, _, _, _) => {
                    h575
                }
            };
            let a577 = 0u64;
            if if if if if if if if if if if if if nat_eq(a576, a577) {
                true
            } else {
                let a579 = match v0.clone() {
                    Adt2::C0(h578, _, _, _, _, _, _, _, _, _, _) => {
                        h578
                    }
                };
                let a580 = 1u64;
                nat_eq(a579, a580)
            } {
                let a582 = match v3.clone() {
                    Adt2::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                        h581
                    }
                };
                let a583 = 26u64;
                nat_eq(a582, a583)
            } else {
                false
            } {
                let a585 = match v2.clone() {
                    Adt2::C0(h584, _, _, _, _, _, _, _, _, _, _) => {
                        h584
                    }
                };
                let a586 = 0u64;
                nat_eq(a585, a586)
            } else {
                false
            } {
                let a589 = match v0.clone() {
                    Adt2::C0(_, h587, _, _, _, _, _, _, _, _, _) => {
                        h587
                    }
                };
                let a590 = match v2.clone() {
                    Adt2::C0(_, h588, _, _, _, _, _, _, _, _, _) => {
                        h588
                    }
                };
                nat_eq(a589, a590)
            } else {
                false
            } {
                let a593 = match v3.clone() {
                    Adt2::C0(_, h591, _, _, _, _, _, _, _, _, _) => {
                        h591
                    }
                };
                let a594 = match v0.clone() {
                    Adt2::C0(_, _, h592, _, _, _, _, _, _, _, _) => {
                        h592
                    }
                };
                nat_le(a593, a594)
            } else {
                false
            } {
                let a596 = match v0.clone() {
                    Adt2::C0(_, _, _, h595, _, _, _, _, _, _, _) => {
                        h595
                    }
                };
                let a597 = 1u64;
                nat_eq(a596, a597)
            } else {
                false
            } {
                let a599 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h598, _, _, _, _, _, _) => {
                        h598
                    }
                };
                let a600 = 1u64;
                nat_eq(a599, a600)
            } else {
                false
            } {
                let a602 = match v2.clone() {
                    Adt2::C0(_, _, _, h601, _, _, _, _, _, _, _) => {
                        h601
                    }
                };
                let a603 = 1u64;
                nat_eq(a602, a603)
            } else {
                false
            } {
                let a605 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h604, _, _, _, _, _, _) => {
                        h604
                    }
                };
                let a606 = 1u64;
                nat_eq(a605, a606)
            } else {
                false
            } {
                let a608 = match v3.clone() {
                    Adt2::C0(_, _, h607, _, _, _, _, _, _, _, _) => {
                        h607
                    }
                };
                let a609 = 1u64;
                nat_eq(a608, a609)
            } else {
                false
            } {
                let a611 = match v3.clone() {
                    Adt2::C0(_, _, _, h610, _, _, _, _, _, _, _) => {
                        h610
                    }
                };
                let a612 = 1u64;
                nat_eq(a611, a612)
            } else {
                false
            } {
                let a614 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h613, _, _, _, _, _, _) => {
                        h613
                    }
                };
                let a615 = 1u64;
                nat_eq(a614, a615)
            } else {
                false
            } {
                if f72(match v2.clone() {
                    Adt2::C0(_, h616, _, _, _, _, _, _, _, _, _) => {
                        h616
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h617, _, _, _, _, _, _, _, _) => {
                        h617
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h618, _, _, _, _, _, _, _) => {
                        h618
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h619, _, _, _, _, _, _) => {
                        h619
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f17(v0, 1u64), f17(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a621 = match v3.clone() {
                        Adt2::C0(_, h620, _, _, _, _, _, _, _, _, _) => {
                            h620
                        }
                    };
                    let a622 = 0u64;
                    if nat_eq(a621, a622) {
                        Ok(Adt6::C4(f13(v2.clone(), 1u64)?, 4u64, f17(v0, 1u64), f17(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a624 = match v0.clone() {
                            Adt2::C0(_, h623, _, _, _, _, _, _, _, _, _) => {
                                h623
                            }
                        };
                        let a625 = 2147483647u64;
                        if if if if if if nat_le(a624, a625) {
                            let a627 = match v3.clone() {
                                Adt2::C0(_, h626, _, _, _, _, _, _, _, _, _) => {
                                    h626
                                }
                            };
                            let a628 = 2147483647u64;
                            nat_le(a627, a628)
                        } else {
                            false
                        } {
                            let a630 = 4u64;
                            let a631 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h629, _, _, _, _, _) => {
                                    h629
                                }
                            };
                            nat_le(a630, a631)
                        } else {
                            false
                        } {
                            let a637 = {
                                let a634 = match v2.clone() {
                                    Adt2::C0(_, h632, _, _, _, _, _, _, _, _, _) => {
                                        h632
                                    }
                                };
                                let a635 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h633, _, _, _, _, _) => {
                                        h633
                                    }
                                };
                                nat_mul(a634, a635)?
                            };
                            let a638 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h636, _, _, _, _) => {
                                    h636
                                }
                            };
                            nat_le(a637, a638)
                        } else {
                            false
                        } {
                            f20(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f20(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a652 = {
                                    let a642 = {
                                        let a640 = 2u64;
                                        let a641 = match v3.clone() {
                                            Adt2::C0(_, h639, _, _, _, _, _, _, _, _, _) => {
                                                h639
                                            }
                                        };
                                        nat_mul(a640, a641)?
                                    };
                                    let a643 = f13(v2.clone(), 1u64)?;
                                    nat_add(a642, a643)?
                                };
                                let a653 = {
                                    let a650 = match v3.clone() {
                                        Adt2::C0(_, h644, _, _, _, _, _, _, _, _, _) => {
                                            h644
                                        }
                                    };
                                    let a651 = {
                                        let a648 = 1u64;
                                        let a649 = {
                                            let a646 = 2u64;
                                            let a647 = match v0.clone() {
                                                Adt2::C0(_, h645, _, _, _, _, _, _, _, _, _) => {
                                                    h645
                                                }
                                            };
                                            nat_mul(a646, a647)?
                                        };
                                        nat_add(a648, a649)?
                                    };
                                    nat_mul(a650, a651)?
                                };
                                nat_add(a652, a653)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h654, _, _, _, _, _, _, _, _, _) => {
                                    h654
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h655, _, _, _, _, _, _, _, _, _) => {
                                    h655
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h656, _, _, _, _, _) => {
                                    h656
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h657, _, _, _, _) => {
                                    h657
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h658, _, _, _) => {
                                    h658
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h659, _, _) => {
                                    h659
                                }
                            }), f17(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h660, _, _, _, _, _, _, _, _, _, _) => {
                                    h660
                                }
                            }, f17(v3, 1u64))))
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

pub fn f27(v0: u64) -> bool {
    let a661 = v0;
    let a662 = 1u64;
    if if nat_eq(a661, a662) {
        true
    } else {
        let a663 = v0;
        let a664 = 33u64;
        nat_eq(a663, a664)
    } {
        true
    } else {
        let a665 = v0;
        let a666 = 34u64;
        nat_eq(a665, a666)
    }
}

pub fn f28(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a669 = match v1.clone() {
        Adt2::C0(h667, _, _, _, _, _, _, _, _, _, _) => {
            h667
        }
    };
    let a670 = match v3.clone() {
        Adt2::C0(h668, _, _, _, _, _, _, _, _, _, _) => {
            h668
        }
    };
    if nat_eq(a669, a670) {
        f73(v0, v1, v2, v3)
    } else {
        if if if if f74(match v1.clone() {
            Adt2::C0(h671, _, _, _, _, _, _, _, _, _, _) => {
                h671
            }
        }) {
            f74(match v3.clone() {
                Adt2::C0(h672, _, _, _, _, _, _, _, _, _, _) => {
                    h672
                }
            })
        } else {
            false
        } {
            let a673 = v0;
            let a674 = 33u64;
            if nat_eq(a673, a674) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a675 = f13(v1.clone(), 1u64)?;
            let a676 = f13(v3.clone(), 1u64)?;
            nat_eq(a675, a676)
        } else {
            false
        } {
            let m678 = f75(match v3.clone() {
                Adt2::C0(h677, _, _, _, _, _, _, _, _, _, _) => {
                    h677
                }
            });
            match m678 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f13(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h679) => {
                            h679
                        }
                    }, f17(v1.clone(), 1u64), f17(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h680, _, _, _, _, _, _, _, _, _, _) => {
                            h680
                        }
                    }, match v3 {
                        Adt2::C0(h681, _, _, _, _, _, _, _, _, _, _) => {
                            h681
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f29(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a683 = match v0.clone() {
        Adt2::C0(h682, _, _, _, _, _, _, _, _, _, _) => {
            h682
        }
    };
    let a684 = 0u64;
    if if if if nat_eq(a683, a684) {
        true
    } else {
        let a686 = match v0.clone() {
            Adt2::C0(h685, _, _, _, _, _, _, _, _, _, _) => {
                h685
            }
        };
        let a687 = 1u64;
        nat_eq(a686, a687)
    } {
        true
    } else {
        let a689 = match v0.clone() {
            Adt2::C0(h688, _, _, _, _, _, _, _, _, _, _) => {
                h688
            }
        };
        let a690 = 25u64;
        if nat_eq(a689, a690) {
            true
        } else {
            let a692 = match v0.clone() {
                Adt2::C0(h691, _, _, _, _, _, _, _, _, _, _) => {
                    h691
                }
            };
            let a693 = 26u64;
            nat_eq(a692, a693)
        }
    } {
        true
    } else {
        let a695 = match v0.clone() {
            Adt2::C0(h694, _, _, _, _, _, _, _, _, _, _) => {
                h694
            }
        };
        let a696 = 30u64;
        nat_eq(a695, a696)
    } {
        let a699 = match v0.clone() {
            Adt2::C0(h697, _, _, _, _, _, _, _, _, _, _) => {
                h697
            }
        };
        let a700 = match v1.clone() {
            Adt2::C0(h698, _, _, _, _, _, _, _, _, _, _) => {
                h698
            }
        };
        if if nat_eq(a699, a700) {
            f76(match v1.clone() {
                Adt2::C0(_, h701, _, _, _, _, _, _, _, _, _) => {
                    h701
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h702, _, _, _, _, _, _, _, _) => {
                    h702
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h703, _, _, _, _, _, _, _) => {
                    h703
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h704, _, _, _, _, _, _) => {
                    h704
                }
            }, match v0.clone() {
                Adt2::C0(_, h705, _, _, _, _, _, _, _, _, _) => {
                    h705
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h706, _, _, _, _, _, _, _, _) => {
                    h706
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h707, _, _, _, _, _, _, _) => {
                    h707
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h708, _, _, _, _, _, _) => {
                    h708
                }
            })
        } else {
            false
        } {
            let m710 = f75(match v0.clone() {
                Adt2::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
                    h709
                }
            });
            match m710 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f13(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h711) => {
                            h711
                        }
                    }, f17(v0, 1u64), f17(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f30(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m712 = v2;
    match m712 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a713 = v0;
            let a714 = 4u64;
            if if if nat_lt(a713, a714) {
                let a717 = match v1.clone() {
                    Adt2::C0(h715, _, _, _, _, _, _, _, _, _, _) => {
                        h715
                    }
                };
                let a718 = match v4.clone() {
                    Adt2::C0(h716, _, _, _, _, _, _, _, _, _, _) => {
                        h716
                    }
                };
                if nat_eq(a717, a718) {
                    let a721 = match v1.clone() {
                        Adt2::C0(h719, _, _, _, _, _, _, _, _, _, _) => {
                            h719
                        }
                    };
                    let a722 = match v3.clone() {
                        Adt2::C0(h720, _, _, _, _, _, _, _, _, _, _) => {
                            h720
                        }
                    };
                    nat_eq(a721, a722)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f77(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h723, _, _, _, _, _, _, _, _, _) => {
                        h723
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h724, _, _, _, _, _, _, _, _, _) => {
                        h724
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h725, _, _, _, _, _, _, _, _, _) => {
                        h725
                    }
                })? {
                    f77(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h726, _, _, _, _, _, _, _, _) => {
                            h726
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h727, _, _, _, _, _, _, _, _) => {
                            h727
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h728, _, _, _, _, _, _, _, _) => {
                            h728
                        }
                    })?
                } else {
                    false
                } {
                    f77(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h729, _, _, _, _, _, _, _) => {
                            h729
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h730, _, _, _, _, _, _, _) => {
                            h730
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h731, _, _, _, _, _, _, _) => {
                            h731
                        }
                    })?
                } else {
                    false
                } {
                    f77(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h732, _, _, _, _, _, _) => {
                            h732
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h733, _, _, _, _, _, _) => {
                            h733
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h734, _, _, _, _, _, _) => {
                            h734
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m736 = f75(match v1.clone() {
                    Adt2::C0(h735, _, _, _, _, _, _, _, _, _, _) => {
                        h735
                    }
                });
                match m736 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h737, _) => {
                                h737
                            }
                        };
                        let a742 = {
                            let a739 = match v1.clone() {
                                Adt2::C0(_, h738, _, _, _, _, _, _, _, _, _) => {
                                    h738
                                }
                            };
                            let a740 = v6;
                            let a741 = 1u64;
                            nat_rem(a739, a740, a741)
                        };
                        let a743 = 0u64;
                        if if if nat_eq(a742, a743) {
                            let a748 = {
                                let a745 = match v4.clone() {
                                    Adt2::C0(_, h744, _, _, _, _, _, _, _, _, _) => {
                                        h744
                                    }
                                };
                                let a746 = v6;
                                let a747 = 1u64;
                                nat_rem(a745, a746, a747)
                            };
                            let a749 = 0u64;
                            nat_eq(a748, a749)
                        } else {
                            false
                        } {
                            let a754 = {
                                let a751 = match v3.clone() {
                                    Adt2::C0(_, h750, _, _, _, _, _, _, _, _, _) => {
                                        h750
                                    }
                                };
                                let a752 = v6;
                                let a753 = 1u64;
                                nat_rem(a751, a752, a753)
                            };
                            let a755 = 0u64;
                            nat_eq(a754, a755)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f13(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h756) => {
                                    h756
                                }
                            }, f17(v1, v6), f17(v3, v6), Adt4::C2(v0, f17(v4, v6))))
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

pub fn f31(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a758 = match v2.clone() {
        Adt2::C0(h757, _, _, _, _, _, _, _, _, _, _) => {
            h757
        }
    };
    let a759 = 0u64;
    if if nat_eq(a758, a759) {
        let a761 = match v3.clone() {
            Adt2::C0(h760, _, _, _, _, _, _, _, _, _, _) => {
                h760
            }
        };
        let a762 = 0u64;
        nat_eq(a761, a762)
    } else {
        false
    } {
        let a763 = v0;
        let a764 = 61u64;
        if nat_eq(a763, a764) {
            let v4: bool = {
                let a766 = 0u64;
                let a767 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h765, _, _, _, _, _, _, _) => {
                        h765
                    }
                };
                nat_lt(a766, a767)
            };
            if if if if if if f78(match v2.clone() {
                Adt2::C0(_, h768, _, _, _, _, _, _, _, _, _) => {
                    h768
                }
            }, match v1.clone() {
                Adt1::C0(h769, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h769
                }
            }, match v1.clone() {
                Adt1::C0(_, h770, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h770
                }
            }, match v3.clone() {
                Adt2::C0(_, h771, _, _, _, _, _, _, _, _, _) => {
                    h771
                }
            })? {
                f78(match v2.clone() {
                    Adt2::C0(_, _, h772, _, _, _, _, _, _, _, _) => {
                        h772
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h773, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h773
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h774, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h774
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h775, _, _, _, _, _, _, _, _) => {
                        h775
                    }
                })?
            } else {
                false
            } {
                f78(match v2.clone() {
                    Adt2::C0(_, _, _, h776, _, _, _, _, _, _, _) => {
                        h776
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h777, _, _, _, _, _, _, _, _, _, _, _) => {
                        h777
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h778, _, _, _, _, _, _, _, _, _, _) => {
                        h778
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
                        h779
                    }
                })?
            } else {
                false
            } {
                f78(match v2.clone() {
                    Adt2::C0(_, _, _, _, h780, _, _, _, _, _, _) => {
                        h780
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h781, _, _, _, _, _, _, _, _, _) => {
                        h781
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h782, _, _, _, _, _, _, _, _) => {
                        h782
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h783, _, _, _, _, _, _) => {
                        h783
                    }
                })?
            } else {
                false
            } {
                let a785 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h784, _, _, _, _, _, _, _) => {
                        h784
                    }
                };
                let a786 = 4294967295u64;
                nat_le(a785, a786)
            } else {
                false
            } {
                if v4 {
                    !f72(match v2.clone() {
                        Adt2::C0(_, h787, _, _, _, _, _, _, _, _, _) => {
                            h787
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                            h788
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h789, _, _, _, _, _, _, _) => {
                            h789
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h790, _, _, _, _, _, _) => {
                            h790
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h791, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h791
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h792, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h792
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h793, _, _, _, _, _, _, _, _, _, _, _) => {
                        h793
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h794, _, _, _, _, _, _, _, _, _) => {
                        h794
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a795 = v0;
            let a796 = 62u64;
            if nat_eq(a795, a796) {
                let a798 = match v1.clone() {
                    Adt1::C0(h797, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h797
                    }
                };
                let a799 = 2147483648u64;
                if if if if if nat_lt(a798, a799) {
                    let a801 = match v1.clone() {
                        Adt1::C0(_, h800, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h800
                        }
                    };
                    let a802 = 2147483648u64;
                    nat_lt(a801, a802)
                } else {
                    false
                } {
                    let a805 = match v1.clone() {
                        Adt1::C0(h803, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h803
                        }
                    };
                    let a806 = match v2.clone() {
                        Adt2::C0(_, h804, _, _, _, _, _, _, _, _, _) => {
                            h804
                        }
                    };
                    nat_lt(a805, a806)
                } else {
                    false
                } {
                    let a809 = match v1.clone() {
                        Adt1::C0(_, h807, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h807
                        }
                    };
                    let a810 = match v2.clone() {
                        Adt2::C0(_, h808, _, _, _, _, _, _, _, _, _) => {
                            h808
                        }
                    };
                    nat_lt(a809, a810)
                } else {
                    false
                } {
                    let a813 = match v2.clone() {
                        Adt2::C0(_, _, h811, _, _, _, _, _, _, _, _) => {
                            h811
                        }
                    };
                    let a814 = match v3.clone() {
                        Adt2::C0(_, _, h812, _, _, _, _, _, _, _, _) => {
                            h812
                        }
                    };
                    if if if nat_eq(a813, a814) {
                        let a817 = match v2.clone() {
                            Adt2::C0(_, _, _, h815, _, _, _, _, _, _, _) => {
                                h815
                            }
                        };
                        let a818 = match v3.clone() {
                            Adt2::C0(_, _, _, h816, _, _, _, _, _, _, _) => {
                                h816
                            }
                        };
                        nat_eq(a817, a818)
                    } else {
                        false
                    } {
                        let a821 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h819, _, _, _, _, _, _) => {
                                h819
                            }
                        };
                        let a822 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h820, _, _, _, _, _, _) => {
                                h820
                            }
                        };
                        nat_eq(a821, a822)
                    } else {
                        false
                    } {
                        let a831 = match v3.clone() {
                            Adt2::C0(_, h823, _, _, _, _, _, _, _, _, _) => {
                                h823
                            }
                        };
                        let a832 = {
                            let a829 = {
                                let a826 = match v2.clone() {
                                    Adt2::C0(_, h824, _, _, _, _, _, _, _, _, _) => {
                                        h824
                                    }
                                };
                                let a827 = match v1.clone() {
                                    Adt1::C0(h825, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h825
                                    }
                                };
                                nat_add(a826, a827)?
                            };
                            let a830 = match v1.clone() {
                                Adt1::C0(_, h828, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h828
                                }
                            };
                            nat_add(a829, a830)?
                        };
                        nat_eq(a831, a832)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h833, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h833
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a834 = v0;
                let a835 = 63u64;
                if nat_eq(a834, a835) {
                    if if if if if f79(match v2.clone() {
                        Adt2::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                            h836
                        }
                    }, match v1.clone() {
                        Adt1::C0(h837, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h837
                        }
                    })? {
                        f79(match v2.clone() {
                            Adt2::C0(_, _, h838, _, _, _, _, _, _, _, _) => {
                                h838
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h839, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h839
                            }
                        })?
                    } else {
                        false
                    } {
                        f79(match v2.clone() {
                            Adt2::C0(_, _, _, h840, _, _, _, _, _, _, _) => {
                                h840
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h841, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h841
                            }
                        })?
                    } else {
                        false
                    } {
                        f79(match v2.clone() {
                            Adt2::C0(_, _, _, _, h842, _, _, _, _, _, _) => {
                                h842
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h843, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h843
                            }
                        })?
                    } else {
                        false
                    } {
                        let a846 = match v2.clone() {
                            Adt2::C0(_, h844, _, _, _, _, _, _, _, _, _) => {
                                h844
                            }
                        };
                        let a847 = match v3.clone() {
                            Adt2::C0(_, h845, _, _, _, _, _, _, _, _, _) => {
                                h845
                            }
                        };
                        if if if nat_eq(a846, a847) {
                            let a850 = match v2.clone() {
                                Adt2::C0(_, _, h848, _, _, _, _, _, _, _, _) => {
                                    h848
                                }
                            };
                            let a851 = match v3.clone() {
                                Adt2::C0(_, _, h849, _, _, _, _, _, _, _, _) => {
                                    h849
                                }
                            };
                            nat_eq(a850, a851)
                        } else {
                            false
                        } {
                            let a854 = match v2.clone() {
                                Adt2::C0(_, _, _, h852, _, _, _, _, _, _, _) => {
                                    h852
                                }
                            };
                            let a855 = match v3.clone() {
                                Adt2::C0(_, _, _, h853, _, _, _, _, _, _, _) => {
                                    h853
                                }
                            };
                            nat_eq(a854, a855)
                        } else {
                            false
                        } {
                            let a858 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h856, _, _, _, _, _, _) => {
                                    h856
                                }
                            };
                            let a859 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h857, _, _, _, _, _, _) => {
                                    h857
                                }
                            };
                            nat_eq(a858, a859)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h860, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h860
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h861, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h861
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h862, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h862
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h863, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h863
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

pub fn f32(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m864 = v3;
    match m864 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a865 = v0;
                let a866 = 4u64;
                nat_eq(a865, a866)
            };
            let v7: bool = {
                let a868 = 0u64;
                let a869 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h867, _, _, _, _, _, _, _, _, _, _, _) => {
                        h867
                    }
                };
                nat_lt(a868, a869)
            };
            let v8: u64 = f13(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h870, _, _, _, _, _, _, _, _, _) => {
                    h870
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h871, _, _, _, _, _, _, _, _) => {
                    h871
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h872, _, _, _, _, _, _, _) => {
                    h872
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h873, _, _, _, _, _, _) => {
                    h873
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h874, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h874
                }
            }, match v1.clone() {
                Adt1::C0(_, h875, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h875
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h876, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h876
                }
            });
            if if if if if if if if if if if if v6 {
                let a878 = match v2.clone() {
                    Adt2::C0(h877, _, _, _, _, _, _, _, _, _, _) => {
                        h877
                    }
                };
                let a879 = 0u64;
                if nat_eq(a878, a879) {
                    let a881 = match v5.clone() {
                        Adt2::C0(h880, _, _, _, _, _, _, _, _, _, _) => {
                            h880
                        }
                    };
                    let a882 = 0u64;
                    nat_eq(a881, a882)
                } else {
                    false
                }
            } else {
                let a884 = match v2.clone() {
                    Adt2::C0(h883, _, _, _, _, _, _, _, _, _, _) => {
                        h883
                    }
                };
                let a885 = 0u64;
                if if nat_eq(a884, a885) {
                    true
                } else {
                    let a887 = match v2.clone() {
                        Adt2::C0(h886, _, _, _, _, _, _, _, _, _, _) => {
                            h886
                        }
                    };
                    let a888 = 26u64;
                    nat_eq(a887, a888)
                } {
                    let a890 = match v5.clone() {
                        Adt2::C0(h889, _, _, _, _, _, _, _, _, _, _) => {
                            h889
                        }
                    };
                    let a891 = 0u64;
                    if nat_eq(a890, a891) {
                        true
                    } else {
                        let a893 = match v5.clone() {
                            Adt2::C0(h892, _, _, _, _, _, _, _, _, _, _) => {
                                h892
                            }
                        };
                        let a894 = 26u64;
                        nat_eq(a893, a894)
                    }
                } else {
                    false
                }
            } {
                let a897 = match v2.clone() {
                    Adt2::C0(h895, _, _, _, _, _, _, _, _, _, _) => {
                        h895
                    }
                };
                let a898 = match v4.clone() {
                    Adt2::C0(h896, _, _, _, _, _, _, _, _, _, _) => {
                        h896
                    }
                };
                nat_eq(a897, a898)
            } else {
                false
            } {
                f18(v2.clone(), v4.clone())
            } else {
                false
            } {
                f80(v2.clone())?
            } else {
                false
            } {
                f80(v4.clone())?
            } else {
                false
            } {
                let a900 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h899, _, _, _, _, _) => {
                        h899
                    }
                };
                let a901 = 4u64;
                nat_eq(a900, a901)
            } else {
                false
            } {
                let a902 = v8;
                let a903 = f13(v2.clone(), 1u64)?;
                nat_le(a902, a903)
            } else {
                false
            } {
                let a905 = match v1.clone() {
                    Adt1::C0(_, _, _, h904, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h904
                    }
                };
                let a906 = 1073741824u64;
                nat_lt(a905, a906)
            } else {
                false
            } {
                let a908 = match v1.clone() {
                    Adt1::C0(h907, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h907
                    }
                };
                let a909 = 4294967295u64;
                if if if if nat_le(a908, a909) {
                    let a911 = match v1.clone() {
                        Adt1::C0(_, h910, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h910
                        }
                    };
                    let a912 = 4294967295u64;
                    nat_le(a911, a912)
                } else {
                    false
                } {
                    let a914 = match v1.clone() {
                        Adt1::C0(_, _, h913, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h913
                        }
                    };
                    let a915 = 4294967295u64;
                    nat_le(a914, a915)
                } else {
                    false
                } {
                    let a917 = match v1.clone() {
                        Adt1::C0(_, _, _, h916, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h916
                        }
                    };
                    let a918 = 4294967295u64;
                    nat_le(a917, a918)
                } else {
                    false
                } {
                    let a920 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h919, _, _, _, _, _, _, _, _, _, _, _) => {
                            h919
                        }
                    };
                    let a921 = 4294967295u64;
                    nat_le(a920, a921)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a924 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h922) => {
                            h922
                        }
                    };
                    let a925 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h923) => {
                            h923
                        }
                    };
                    nat_eq(a924, a925)
                } else {
                    true
                }
            } else {
                false
            } {
                f81(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h926, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h926
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h927, _) => {
                        h927
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f13(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a928 = v10;
                    let a929 = v8;
                    nat_add(a928, a929)?
                }, 4u64, f17(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h930, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h930
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a932 = match v0.clone() {
        Adt2::C0(h931, _, _, _, _, _, _, _, _, _, _) => {
            h931
        }
    };
    let a933 = 0u64;
    if if if nat_eq(a932, a933) {
        let a935 = match v1.clone() {
            Adt2::C0(h934, _, _, _, _, _, _, _, _, _, _) => {
                h934
            }
        };
        let a936 = 0u64;
        nat_eq(a935, a936)
    } else {
        false
    } {
        f76(match v0.clone() {
            Adt2::C0(_, h937, _, _, _, _, _, _, _, _, _) => {
                h937
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h938, _, _, _, _, _, _, _, _) => {
                h938
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h939, _, _, _, _, _, _, _) => {
                h939
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h940, _, _, _, _, _, _) => {
                h940
            }
        }, match v1.clone() {
            Adt2::C0(_, h941, _, _, _, _, _, _, _, _, _) => {
                h941
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h942, _, _, _, _, _, _, _, _) => {
                h942
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h943, _, _, _, _, _, _, _) => {
                h943
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h944, _, _, _, _, _, _) => {
                h944
            }
        })
    } else {
        false
    } {
        let v2: u64 = f13(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a945 = v2;
            let a946 = f13(v0.clone(), 1u64)?;
            nat_add(a945, a946)?
        }, 4u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f34(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a948 = match v1.clone() {
        Adt2::C0(h947, _, _, _, _, _, _, _, _, _, _) => {
            h947
        }
    };
    let a949 = 0u64;
    if if if nat_eq(a948, a949) {
        true
    } else {
        let a951 = match v1.clone() {
            Adt2::C0(h950, _, _, _, _, _, _, _, _, _, _) => {
                h950
            }
        };
        let a952 = 1u64;
        nat_eq(a951, a952)
    } {
        true
    } else {
        let a954 = match v1.clone() {
            Adt2::C0(h953, _, _, _, _, _, _, _, _, _, _) => {
                h953
            }
        };
        let a955 = 30u64;
        nat_eq(a954, a955)
    } {
        let a958 = match v1.clone() {
            Adt2::C0(h956, _, _, _, _, _, _, _, _, _, _) => {
                h956
            }
        };
        let a959 = match v2.clone() {
            Adt2::C0(h957, _, _, _, _, _, _, _, _, _, _) => {
                h957
            }
        };
        if if if if if nat_eq(a958, a959) {
            let a962 = match v1.clone() {
                Adt2::C0(_, h960, _, _, _, _, _, _, _, _, _) => {
                    h960
                }
            };
            let a963 = match v2.clone() {
                Adt2::C0(_, h961, _, _, _, _, _, _, _, _, _) => {
                    h961
                }
            };
            nat_eq(a962, a963)
        } else {
            false
        } {
            let a966 = match v1.clone() {
                Adt2::C0(_, _, _, h964, _, _, _, _, _, _, _) => {
                    h964
                }
            };
            let a967 = match v2.clone() {
                Adt2::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                    h965
                }
            };
            nat_eq(a966, a967)
        } else {
            false
        } {
            let a970 = match v1.clone() {
                Adt2::C0(_, _, _, _, h968, _, _, _, _, _, _) => {
                    h968
                }
            };
            let a971 = match v2.clone() {
                Adt2::C0(_, _, _, _, h969, _, _, _, _, _, _) => {
                    h969
                }
            };
            nat_eq(a970, a971)
        } else {
            false
        } {
            let a972 = v0;
            let a973 = 4u64;
            if nat_eq(a972, a973) {
                let a975 = match v1.clone() {
                    Adt2::C0(_, _, h974, _, _, _, _, _, _, _, _) => {
                        h974
                    }
                };
                let a976 = 1u64;
                if nat_eq(a975, a976) {
                    let a979 = match v1.clone() {
                        Adt2::C0(_, h977, _, _, _, _, _, _, _, _, _) => {
                            h977
                        }
                    };
                    let a980 = match v2.clone() {
                        Adt2::C0(_, _, h978, _, _, _, _, _, _, _, _) => {
                            h978
                        }
                    };
                    nat_eq(a979, a980)
                } else {
                    false
                }
            } else {
                let a981 = v0;
                let a982 = 4u64;
                if if nat_lt(a981, a982) {
                    let a985 = match v1.clone() {
                        Adt2::C0(_, h983, _, _, _, _, _, _, _, _, _) => {
                            h983
                        }
                    };
                    let a986 = match v1.clone() {
                        Adt2::C0(_, _, h984, _, _, _, _, _, _, _, _) => {
                            h984
                        }
                    };
                    nat_eq(a985, a986)
                } else {
                    false
                } {
                    let a989 = match v1.clone() {
                        Adt2::C0(_, _, h987, _, _, _, _, _, _, _, _) => {
                            h987
                        }
                    };
                    let a990 = match v2.clone() {
                        Adt2::C0(_, _, h988, _, _, _, _, _, _, _, _) => {
                            h988
                        }
                    };
                    nat_eq(a989, a990)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m992 = f75(match v1.clone() {
                Adt2::C0(h991, _, _, _, _, _, _, _, _, _, _) => {
                    h991
                }
            });
            match m992 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f13(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h993) => {
                            h993
                        }
                    }, f17(v1, 1u64), f17(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f35(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f82(match v1.clone() {
        Adt2::C0(h994, _, _, _, _, _, _, _, _, _, _) => {
            h994
        }
    }, v0) {
        let a997 = match v1.clone() {
            Adt2::C0(h995, _, _, _, _, _, _, _, _, _, _) => {
                h995
            }
        };
        let a998 = match v2.clone() {
            Adt2::C0(h996, _, _, _, _, _, _, _, _, _, _) => {
                h996
            }
        };
        nat_eq(a997, a998)
    } else {
        false
    } {
        let a1001 = match v1.clone() {
            Adt2::C0(_, h999, _, _, _, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1002 = match v2.clone() {
            Adt2::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
                h1000
            }
        };
        if if if nat_eq(a1001, a1002) {
            let a1005 = match v1.clone() {
                Adt2::C0(_, _, h1003, _, _, _, _, _, _, _, _) => {
                    h1003
                }
            };
            let a1006 = match v2.clone() {
                Adt2::C0(_, _, h1004, _, _, _, _, _, _, _, _) => {
                    h1004
                }
            };
            nat_eq(a1005, a1006)
        } else {
            false
        } {
            let a1009 = match v1.clone() {
                Adt2::C0(_, _, _, h1007, _, _, _, _, _, _, _) => {
                    h1007
                }
            };
            let a1010 = match v2.clone() {
                Adt2::C0(_, _, _, h1008, _, _, _, _, _, _, _) => {
                    h1008
                }
            };
            nat_eq(a1009, a1010)
        } else {
            false
        } {
            let a1013 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1011, _, _, _, _, _, _) => {
                    h1011
                }
            };
            let a1014 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1012, _, _, _, _, _, _) => {
                    h1012
                }
            };
            nat_eq(a1013, a1014)
        } else {
            false
        }
    } else {
        false
    } {
        let m1016 = f75(match v1.clone() {
            Adt2::C0(h1015, _, _, _, _, _, _, _, _, _, _) => {
                h1015
            }
        });
        match m1016 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f13(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h1017) => {
                        h1017
                    }
                }, f17(v1.clone(), 1u64), f17(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1018, _, _, _, _, _, _, _, _, _, _) => {
                        h1018
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f36(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1019, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1019
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1020, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1020
            }
        }
    };
    let a1021 = 0u64;
    let a1022 = v4;
    if if nat_lt(a1021, a1022) {
        let a1023 = v4;
        let a1024 = 2147483648u64;
        nat_lt(a1023, a1024)
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
        let v7: u64 = f83(match v5.clone() {
            Adt2::C0(_, _, h1025, _, _, _, _, _, _, _, _) => {
                h1025
            }
        }, v4)?;
        let v8: u64 = f83(match v5.clone() {
            Adt2::C0(_, _, _, h1026, _, _, _, _, _, _, _) => {
                h1026
            }
        }, v4)?;
        let a1028 = match v5.clone() {
            Adt2::C0(h1027, _, _, _, _, _, _, _, _, _, _) => {
                h1027
            }
        };
        let a1029 = 0u64;
        if if if if if nat_eq(a1028, a1029) {
            let a1031 = match v6.clone() {
                Adt2::C0(h1030, _, _, _, _, _, _, _, _, _, _) => {
                    h1030
                }
            };
            let a1032 = 0u64;
            nat_eq(a1031, a1032)
        } else {
            false
        } {
            let a1034 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1033, _, _, _, _, _, _) => {
                    h1033
                }
            };
            let a1035 = 1u64;
            nat_eq(a1034, a1035)
        } else {
            false
        } {
            let a1038 = match v5.clone() {
                Adt2::C0(_, h1036, _, _, _, _, _, _, _, _, _) => {
                    h1036
                }
            };
            let a1039 = match v6.clone() {
                Adt2::C0(_, h1037, _, _, _, _, _, _, _, _, _) => {
                    h1037
                }
            };
            nat_eq(a1038, a1039)
        } else {
            false
        } {
            if v0 {
                let a1041 = match v5.clone() {
                    Adt2::C0(_, _, h1040, _, _, _, _, _, _, _, _) => {
                        h1040
                    }
                };
                let a1042 = 2147483648u64;
                if if if nat_lt(a1041, a1042) {
                    let a1044 = match v5.clone() {
                        Adt2::C0(_, _, _, h1043, _, _, _, _, _, _, _) => {
                            h1043
                        }
                    };
                    let a1045 = 2147483648u64;
                    nat_lt(a1044, a1045)
                } else {
                    false
                } {
                    let a1048 = {
                        let a1046 = v7;
                        let a1047 = v8;
                        nat_mul(a1046, a1047)?
                    };
                    let a1049 = 2147483648u64;
                    nat_le(a1048, a1049)
                } else {
                    false
                } {
                    f84(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1051 = match v1.clone() {
                    Adt1::C0(h1050, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1050
                    }
                };
                let a1052 = v7;
                if if if if nat_eq(a1051, a1052) {
                    let a1054 = match v1 {
                        Adt1::C0(_, h1053, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1053
                        }
                    };
                    let a1055 = v8;
                    nat_eq(a1054, a1055)
                } else {
                    false
                } {
                    f85(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1057 = match v6.clone() {
                        Adt2::C0(_, _, h1056, _, _, _, _, _, _, _, _) => {
                            h1056
                        }
                    };
                    let a1058 = v4;
                    if nat_eq(a1057, a1058) {
                        let a1060 = match v6.clone() {
                            Adt2::C0(_, _, _, h1059, _, _, _, _, _, _, _) => {
                                h1059
                            }
                        };
                        let a1061 = v4;
                        nat_eq(a1060, a1061)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1065 = match v6 {
                        Adt2::C0(_, _, _, _, h1062, _, _, _, _, _, _) => {
                            h1062
                        }
                    };
                    let a1066 = {
                        let a1063 = v7;
                        let a1064 = v8;
                        nat_mul(a1063, a1064)?
                    };
                    nat_eq(a1065, a1066)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f37(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a1068 = match v2.clone() {
        Adt2::C0(h1067, _, _, _, _, _, _, _, _, _, _) => {
            h1067
        }
    };
    let a1069 = 0u64;
    if if if if if nat_eq(a1068, a1069) {
        true
    } else {
        let a1071 = match v2.clone() {
            Adt2::C0(h1070, _, _, _, _, _, _, _, _, _, _) => {
                h1070
            }
        };
        let a1072 = 1u64;
        nat_eq(a1071, a1072)
    } {
        true
    } else {
        let a1074 = match v2.clone() {
            Adt2::C0(h1073, _, _, _, _, _, _, _, _, _, _) => {
                h1073
            }
        };
        let a1075 = 30u64;
        nat_eq(a1074, a1075)
    } {
        let a1078 = match v2.clone() {
            Adt2::C0(h1076, _, _, _, _, _, _, _, _, _, _) => {
                h1076
            }
        };
        let a1079 = match v3.clone() {
            Adt2::C0(h1077, _, _, _, _, _, _, _, _, _, _) => {
                h1077
            }
        };
        nat_eq(a1078, a1079)
    } else {
        false
    } {
        if if f18(v2.clone(), v3.clone()) {
            let a1081 = match v1.clone() {
                Adt1::C0(h1080, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1080
                }
            };
            let a1082 = 4294967296u64;
            nat_lt(a1081, a1082)
        } else {
            false
        } {
            let a1084 = match v1.clone() {
                Adt1::C0(_, h1083, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1083
                }
            };
            let a1085 = 4294967296u64;
            nat_lt(a1084, a1085)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f13(v3.clone(), 1u64)?, {
            let a1087 = match v2.clone() {
                Adt2::C0(h1086, _, _, _, _, _, _, _, _, _, _) => {
                    h1086
                }
            };
            let a1088 = 0u64;
            if nat_eq(a1087, a1088) {
                4u64
            } else {
                2u64
            }
        }, f17(v2.clone(), 1u64), f17(v3, 1u64), Adt4::C13({
            let a1089 = v0;
            let a1090 = 31u64;
            if nat_eq(a1089, a1090) {
                0u64
            } else {
                let a1091 = v0;
                let a1092 = 47u64;
                if nat_eq(a1091, a1092) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1093, _, _, _, _, _, _, _, _, _, _) => {
                h1093
            }
        }, match v1.clone() {
            Adt1::C0(h1094, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1094
            }
        }, match v1 {
            Adt1::C0(_, h1095, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1095
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f38(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1097 = match v0.clone() {
        Adt2::C0(h1096, _, _, _, _, _, _, _, _, _, _) => {
            h1096
        }
    };
    let a1098 = 0u64;
    if if if nat_eq(a1097, a1098) {
        let a1100 = match v1.clone() {
            Adt2::C0(h1099, _, _, _, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1101 = 0u64;
        nat_eq(a1100, a1101)
    } else {
        false
    } {
        f18(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f13(v1.clone(), 1u64)?, 4u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f39(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1103 = match v0.clone() {
        Adt2::C0(h1102, _, _, _, _, _, _, _, _, _, _) => {
            h1102
        }
    };
    let a1104 = 0u64;
    if if nat_eq(a1103, a1104) {
        let a1106 = match v1.clone() {
            Adt2::C0(h1105, _, _, _, _, _, _, _, _, _, _) => {
                h1105
            }
        };
        let a1107 = 26u64;
        nat_eq(a1106, a1107)
    } else {
        false
    } {
        let a1109 = match v0.clone() {
            Adt2::C0(_, _, _, h1108, _, _, _, _, _, _, _) => {
                h1108
            }
        };
        let a1110 = 1u64;
        if if if if if if if nat_eq(a1109, a1110) {
            let a1112 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1111, _, _, _, _, _, _) => {
                    h1111
                }
            };
            let a1113 = 1u64;
            nat_eq(a1112, a1113)
        } else {
            false
        } {
            let a1115 = match v0.clone() {
                Adt2::C0(_, h1114, _, _, _, _, _, _, _, _, _) => {
                    h1114
                }
            };
            let a1116 = 2147483647u64;
            nat_le(a1115, a1116)
        } else {
            false
        } {
            let a1119 = match v1.clone() {
                Adt2::C0(_, h1117, _, _, _, _, _, _, _, _, _) => {
                    h1117
                }
            };
            let a1120 = match v0.clone() {
                Adt2::C0(_, _, h1118, _, _, _, _, _, _, _, _) => {
                    h1118
                }
            };
            nat_eq(a1119, a1120)
        } else {
            false
        } {
            let a1122 = match v1.clone() {
                Adt2::C0(_, _, h1121, _, _, _, _, _, _, _, _) => {
                    h1121
                }
            };
            let a1123 = 1u64;
            nat_eq(a1122, a1123)
        } else {
            false
        } {
            let a1125 = match v1.clone() {
                Adt2::C0(_, _, _, h1124, _, _, _, _, _, _, _) => {
                    h1124
                }
            };
            let a1126 = 1u64;
            nat_eq(a1125, a1126)
        } else {
            false
        } {
            let a1128 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1127, _, _, _, _, _, _) => {
                    h1127
                }
            };
            let a1129 = 1u64;
            nat_eq(a1128, a1129)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1132 = match v0.clone() {
                    Adt2::C0(_, _, h1130, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                };
                let a1133 = f59(match v0.clone() {
                    Adt2::C0(_, h1131, _, _, _, _, _, _, _, _, _) => {
                        h1131
                    }
                })?;
                nat_mul(a1132, a1133)?
            }, 4u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f40(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1134 = v1;
    match m1134 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f61(f17(v0.clone(), 1u64))?;
            let a1136 = match v0.clone() {
                Adt2::C0(h1135, _, _, _, _, _, _, _, _, _, _) => {
                    h1135
                }
            };
            let a1137 = 26u64;
            if if if nat_eq(a1136, a1137) {
                let a1139 = match v3.clone() {
                    Adt2::C0(h1138, _, _, _, _, _, _, _, _, _, _) => {
                        h1138
                    }
                };
                let a1140 = 26u64;
                nat_eq(a1139, a1140)
            } else {
                false
            } {
                let a1142 = match v2.clone() {
                    Adt2::C0(h1141, _, _, _, _, _, _, _, _, _, _) => {
                        h1141
                    }
                };
                let a1143 = 27u64;
                nat_eq(a1142, a1143)
            } else {
                false
            } {
                if if if if if if f18(v0.clone(), v3.clone()) {
                    let a1145 = match v2.clone() {
                        Adt2::C0(_, h1144, _, _, _, _, _, _, _, _, _) => {
                            h1144
                        }
                    };
                    let a1146 = 1u64;
                    nat_eq(a1145, a1146)
                } else {
                    false
                } {
                    let a1148 = match v2.clone() {
                        Adt2::C0(_, _, h1147, _, _, _, _, _, _, _, _) => {
                            h1147
                        }
                    };
                    let a1149 = 1u64;
                    nat_eq(a1148, a1149)
                } else {
                    false
                } {
                    let a1151 = match v2.clone() {
                        Adt2::C0(_, _, _, h1150, _, _, _, _, _, _, _) => {
                            h1150
                        }
                    };
                    let a1152 = 1u64;
                    nat_eq(a1151, a1152)
                } else {
                    false
                } {
                    let a1154 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1153, _, _, _, _, _, _) => {
                            h1153
                        }
                    };
                    let a1155 = 1u64;
                    nat_eq(a1154, a1155)
                } else {
                    false
                } {
                    let a1156 = v4;
                    let a1157 = 9223372036854775808u64;
                    nat_lt(a1156, a1157)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1160 = 2u64;
                        let a1161 = {
                            let a1158 = v4;
                            let a1159 = 2u64;
                            nat_mul(a1158, a1159)?
                        };
                        nat_add(a1160, a1161)?
                    }, 8u64, f17(v0, 1u64), f17(v2, 1u64), Adt4::C18(v4, f17(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f41(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1162, _, _, _, _, _, _, _, _, _, _) => {
            h1162
        }
    };
    let a1163 = v3;
    let a1164 = 0u64;
    if if nat_eq(a1163, a1164) {
        true
    } else {
        let a1165 = v0;
        let a1166 = 13u64;
        if nat_eq(a1165, a1166) {
            let a1167 = v3;
            let a1168 = 1u64;
            if nat_eq(a1167, a1168) {
                true
            } else {
                let a1169 = v3;
                let a1170 = 30u64;
                nat_eq(a1169, a1170)
            }
        } else {
            false
        }
    } {
        let a1172 = v3;
        let a1173 = match v2.clone() {
            Adt2::C0(h1171, _, _, _, _, _, _, _, _, _, _) => {
                h1171
            }
        };
        if if nat_eq(a1172, a1173) {
            let a1174 = v0;
            let a1175 = 13u64;
            if nat_eq(a1174, a1175) {
                let a1177 = match v2.clone() {
                    Adt2::C0(_, h1176, _, _, _, _, _, _, _, _, _) => {
                        h1176
                    }
                };
                let a1178 = 1u64;
                if if if nat_eq(a1177, a1178) {
                    let a1180 = match v2.clone() {
                        Adt2::C0(_, _, h1179, _, _, _, _, _, _, _, _) => {
                            h1179
                        }
                    };
                    let a1181 = 1u64;
                    nat_eq(a1180, a1181)
                } else {
                    false
                } {
                    let a1183 = match v2.clone() {
                        Adt2::C0(_, _, _, h1182, _, _, _, _, _, _, _) => {
                            h1182
                        }
                    };
                    let a1184 = 1u64;
                    nat_eq(a1183, a1184)
                } else {
                    false
                } {
                    let a1186 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1185, _, _, _, _, _, _) => {
                            h1185
                        }
                    };
                    let a1187 = 1u64;
                    nat_eq(a1186, a1187)
                } else {
                    false
                }
            } else {
                let a1189 = match v2.clone() {
                    Adt2::C0(_, h1188, _, _, _, _, _, _, _, _, _) => {
                        h1188
                    }
                };
                let a1190 = 1u64;
                if if if nat_eq(a1189, a1190) {
                    let a1193 = match v1.clone() {
                        Adt2::C0(_, _, h1191, _, _, _, _, _, _, _, _) => {
                            h1191
                        }
                    };
                    let a1194 = match v2.clone() {
                        Adt2::C0(_, _, h1192, _, _, _, _, _, _, _, _) => {
                            h1192
                        }
                    };
                    nat_eq(a1193, a1194)
                } else {
                    false
                } {
                    let a1197 = match v1.clone() {
                        Adt2::C0(_, _, _, h1195, _, _, _, _, _, _, _) => {
                            h1195
                        }
                    };
                    let a1198 = match v2.clone() {
                        Adt2::C0(_, _, _, h1196, _, _, _, _, _, _, _) => {
                            h1196
                        }
                    };
                    nat_eq(a1197, a1198)
                } else {
                    false
                } {
                    let a1201 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1199, _, _, _, _, _, _) => {
                            h1199
                        }
                    };
                    let a1202 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1200, _, _, _, _, _, _) => {
                            h1200
                        }
                    };
                    nat_eq(a1201, a1202)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f17(v1, 1u64);
            Ok(Adt6::C4(f86(v4.clone(), v0, v3)?, {
                let a1203 = v3;
                let a1204 = 0u64;
                if nat_eq(a1203, a1204) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f17(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f42(v0: u64, v1: u64) -> Option<u64> {
    let a1205 = v0;
    let a1206 = 8u64;
    if nat_eq(a1205, a1206) {
        Some(5u64)
    } else {
        let a1207 = v0;
        let a1208 = 9u64;
        if nat_eq(a1207, a1208) {
            Some(6u64)
        } else {
            let a1209 = v0;
            let a1210 = 89u64;
            if nat_eq(a1209, a1210) {
                let a1211 = v1;
                let a1212 = 0u64;
                if nat_eq(a1211, a1212) {
                    Some(0u64)
                } else {
                    let a1213 = v1;
                    let a1214 = 1u64;
                    if nat_eq(a1213, a1214) {
                        Some(1u64)
                    } else {
                        let a1215 = v1;
                        let a1216 = 2u64;
                        if nat_eq(a1215, a1216) {
                            Some(2u64)
                        } else {
                            let a1217 = v1;
                            let a1218 = 3u64;
                            if nat_eq(a1217, a1218) {
                                Some(3u64)
                            } else {
                                let a1219 = v1;
                                let a1220 = 6u64;
                                if nat_eq(a1219, a1220) {
                                    Some(4u64)
                                } else {
                                    let a1221 = v1;
                                    let a1222 = 12u64;
                                    if nat_eq(a1221, a1222) {
                                        Some(7u64)
                                    } else {
                                        let a1223 = v1;
                                        let a1224 = 11u64;
                                        if nat_eq(a1223, a1224) {
                                            Some(8u64)
                                        } else {
                                            let a1225 = v1;
                                            let a1226 = 18u64;
                                            if nat_eq(a1225, a1226) {
                                                Some(9u64)
                                            } else {
                                                let a1227 = v1;
                                                let a1228 = 19u64;
                                                if nat_eq(a1227, a1228) {
                                                    Some(10u64)
                                                } else {
                                                    let a1229 = v1;
                                                    let a1230 = 20u64;
                                                    if nat_eq(a1229, a1230) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1231 = v1;
                                                        let a1232 = 21u64;
                                                        if nat_eq(a1231, a1232) {
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

pub fn f43(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1233 = f87(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1233 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f17(v2.clone(), 1u64), f17(v4, 1u64), Adt4::C10(v5, v6, {
                let m1234 = v3;
                match m1234 {
                    None => {
                        f17(v2, 1u64)
                    }
                    Some(v12) => {
                        f17(v12, 1u64)
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

pub fn f44(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1236 = match v1.clone() {
        Adt2::C0(h1235, _, _, _, _, _, _, _, _, _, _) => {
            h1235
        }
    };
    let a1237 = 0u64;
    if if if if if nat_eq(a1236, a1237) {
        true
    } else {
        let a1239 = match v1.clone() {
            Adt2::C0(h1238, _, _, _, _, _, _, _, _, _, _) => {
                h1238
            }
        };
        let a1240 = 1u64;
        nat_eq(a1239, a1240)
    } {
        true
    } else {
        let a1242 = match v1.clone() {
            Adt2::C0(h1241, _, _, _, _, _, _, _, _, _, _) => {
                h1241
            }
        };
        let a1243 = 30u64;
        nat_eq(a1242, a1243)
    } {
        let a1246 = match v1.clone() {
            Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                h1244
            }
        };
        let a1247 = match v2.clone() {
            Adt2::C0(h1245, _, _, _, _, _, _, _, _, _, _) => {
                h1245
            }
        };
        if nat_eq(a1246, a1247) {
            true
        } else {
            let a1249 = match v2.clone() {
                Adt2::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
                    h1248
                }
            };
            let a1250 = 0u64;
            nat_eq(a1249, a1250)
        }
    } else {
        false
    } {
        f18(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f13(v2.clone(), 1u64)?, {
            let a1252 = match v2.clone() {
                Adt2::C0(h1251, _, _, _, _, _, _, _, _, _, _) => {
                    h1251
                }
            };
            let a1253 = 0u64;
            if nat_eq(a1252, a1253) {
                4u64
            } else {
                2u64
            }
        }, f17(v1.clone(), 1u64), f17(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                h1254
            }
        }, match v2 {
            Adt2::C0(h1255, _, _, _, _, _, _, _, _, _, _) => {
                h1255
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f45(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1257 = v0;
        let a1258 = match v1.clone() {
            Adt3::C0(h1256, _, _, _, _, _, _, _) => {
                h1256
            }
        };
        let a1259 = 0u64;
        nat_quot(a1257, a1258, a1259)
    };
    let v3: u64 = {
        let a1261 = v2;
        let a1262 = match v1.clone() {
            Adt3::C0(_, h1260, _, _, _, _, _, _) => {
                h1260
            }
        };
        let a1263 = 0u64;
        nat_quot(a1261, a1262, a1263)
    };
    f88({
        let a1265 = v0;
        let a1266 = match v1.clone() {
            Adt3::C0(h1264, _, _, _, _, _, _, _) => {
                h1264
            }
        };
        let a1267 = 0u64;
        nat_rem(a1265, a1266, a1267)
    }, {
        let a1269 = v2;
        let a1270 = match v1.clone() {
            Adt3::C0(_, h1268, _, _, _, _, _, _) => {
                h1268
            }
        };
        let a1271 = 0u64;
        nat_rem(a1269, a1270, a1271)
    }, {
        let a1273 = v3;
        let a1274 = match v1.clone() {
            Adt3::C0(_, _, h1272, _, _, _, _, _) => {
                h1272
            }
        };
        let a1275 = 0u64;
        nat_rem(a1273, a1274, a1275)
    }, {
        let a1277 = v3;
        let a1278 = match v1.clone() {
            Adt3::C0(_, _, h1276, _, _, _, _, _) => {
                h1276
            }
        };
        let a1279 = 0u64;
        nat_quot(a1277, a1278, a1279)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1280, _, _, _) => {
            h1280
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1281, _, _) => {
            h1281
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1282, _) => {
            h1282
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1283) => {
            h1283
        }
    })
}

pub fn f46(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1301 = {
        let a1296 = f89(v2, {
            let a1287 = {
                let a1284 = v0;
                let a1285 = v1;
                let a1286 = 0u64;
                nat_quot(a1284, a1285, a1286)
            };
            let a1288 = v2;
            let a1289 = 0u64;
            nat_rem(a1287, a1288, a1289)
        }, {
            let a1293 = {
                let a1290 = v0;
                let a1291 = v1;
                let a1292 = 0u64;
                nat_quot(a1290, a1291, a1292)
            };
            let a1294 = v2;
            let a1295 = 0u64;
            nat_quot(a1293, a1294, a1295)
        })?;
        let a1297 = v1;
        nat_mul(a1296, a1297)?
    };
    let a1302 = {
        let a1298 = v0;
        let a1299 = v1;
        let a1300 = 0u64;
        nat_rem(a1298, a1299, a1300)
    };
    nat_add(a1301, a1302)
}

pub fn f47(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1304 = v0;
        let a1305 = match v2.clone() {
            Adt3::C0(h1303, _, _, _, _, _, _, _) => {
                h1303
            }
        };
        let a1306 = 0u64;
        nat_quot(a1304, a1305, a1306)
    };
    let v4: u64 = {
        let a1308 = v3;
        let a1309 = match v2.clone() {
            Adt3::C0(_, h1307, _, _, _, _, _, _) => {
                h1307
            }
        };
        let a1310 = 0u64;
        nat_quot(a1308, a1309, a1310)
    };
    f90({
        let a1312 = v0;
        let a1313 = match v2.clone() {
            Adt3::C0(h1311, _, _, _, _, _, _, _) => {
                h1311
            }
        };
        let a1314 = 0u64;
        nat_rem(a1312, a1313, a1314)
    }, {
        let a1316 = v3;
        let a1317 = match v2.clone() {
            Adt3::C0(_, h1315, _, _, _, _, _, _) => {
                h1315
            }
        };
        let a1318 = 0u64;
        nat_rem(a1316, a1317, a1318)
    }, {
        let a1320 = v4;
        let a1321 = match v2.clone() {
            Adt3::C0(_, _, h1319, _, _, _, _, _) => {
                h1319
            }
        };
        let a1322 = 0u64;
        nat_rem(a1320, a1321, a1322)
    }, {
        let a1324 = v4;
        let a1325 = match v2 {
            Adt3::C0(_, _, h1323, _, _, _, _, _) => {
                h1323
            }
        };
        let a1326 = 0u64;
        nat_quot(a1324, a1325, a1326)
    }, match v1.clone() {
        Adt3::C0(h1327, _, _, _, _, _, _, _) => {
            h1327
        }
    }, match v1.clone() {
        Adt3::C0(_, h1328, _, _, _, _, _, _) => {
            h1328
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1329, _, _, _, _, _) => {
            h1329
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1330, _, _, _, _) => {
            h1330
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1331, _, _, _) => {
            h1331
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1332, _, _) => {
            h1332
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1333, _) => {
            h1333
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1334) => {
            h1334
        }
    })
}

pub fn f48(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1336 = v0;
        let a1337 = match v3.clone() {
            Adt3::C0(h1335, _, _, _, _, _, _, _) => {
                h1335
            }
        };
        let a1338 = 0u64;
        nat_quot(a1336, a1337, a1338)
    };
    let v6: u64 = {
        let a1340 = v5;
        let a1341 = match v3.clone() {
            Adt3::C0(_, h1339, _, _, _, _, _, _) => {
                h1339
            }
        };
        let a1342 = 0u64;
        nat_quot(a1340, a1341, a1342)
    };
    let v7: u64 = {
        let a1344 = v0;
        let a1345 = match v3.clone() {
            Adt3::C0(h1343, _, _, _, _, _, _, _) => {
                h1343
            }
        };
        let a1346 = 0u64;
        nat_rem(a1344, a1345, a1346)
    };
    let v8: u64 = {
        let a1348 = v5;
        let a1349 = match v3.clone() {
            Adt3::C0(_, h1347, _, _, _, _, _, _) => {
                h1347
            }
        };
        let a1350 = 0u64;
        nat_rem(a1348, a1349, a1350)
    };
    let v9: u64 = {
        let a1352 = v6;
        let a1353 = match v3.clone() {
            Adt3::C0(_, _, h1351, _, _, _, _, _) => {
                h1351
            }
        };
        let a1354 = 0u64;
        nat_rem(a1352, a1353, a1354)
    };
    let v10: u64 = {
        let a1356 = v6;
        let a1357 = match v3.clone() {
            Adt3::C0(_, _, h1355, _, _, _, _, _) => {
                h1355
            }
        };
        let a1358 = 0u64;
        nat_quot(a1356, a1357, a1358)
    };
    let v11: u64 = {
        let a1359 = v4;
        let a1360 = 0u64;
        if nat_eq(a1359, a1360) {
            match v1.clone() {
                Adt3::C0(h1361, _, _, _, _, _, _, _) => {
                    h1361
                }
            }
        } else {
            let a1362 = v4;
            let a1363 = 1u64;
            if nat_eq(a1362, a1363) {
                match v1.clone() {
                    Adt3::C0(_, h1364, _, _, _, _, _, _) => {
                        h1364
                    }
                }
            } else {
                let a1365 = v4;
                let a1366 = 2u64;
                if nat_eq(a1365, a1366) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1367, _, _, _, _, _) => {
                            h1367
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1368, _, _, _, _) => {
                            h1368
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1375 = v11;
        let a1376 = {
            let a1369 = v4;
            let a1370 = 0u64;
            if nat_eq(a1369, a1370) {
                v7
            } else {
                let a1371 = v4;
                let a1372 = 1u64;
                if nat_eq(a1371, a1372) {
                    v8
                } else {
                    let a1373 = v4;
                    let a1374 = 2u64;
                    if nat_eq(a1373, a1374) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1375, a1376)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f88(if if v12 {
        let a1377 = v4;
        let a1378 = 0u64;
        nat_eq(a1377, a1378)
    } else {
        false
    } {
        let a1379 = v7;
        let a1380 = v11;
        nat_sub(a1379, a1380)
    } else {
        v7
    }, if if v12 {
        let a1381 = v4;
        let a1382 = 1u64;
        nat_eq(a1381, a1382)
    } else {
        false
    } {
        let a1383 = v8;
        let a1384 = v11;
        nat_sub(a1383, a1384)
    } else {
        v8
    }, if if v12 {
        let a1385 = v4;
        let a1386 = 2u64;
        nat_eq(a1385, a1386)
    } else {
        false
    } {
        let a1387 = v9;
        let a1388 = v11;
        nat_sub(a1387, a1388)
    } else {
        v9
    }, if if v12 {
        let a1389 = v4;
        let a1390 = 3u64;
        nat_eq(a1389, a1390)
    } else {
        false
    } {
        let a1391 = v10;
        let a1392 = v11;
        nat_sub(a1391, a1392)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1393, _, _, _) => {
            h1393
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1394, _, _) => {
            h1394
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1395, _) => {
            h1395
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1396) => {
            h1396
        }
    })?, f45(v0, v3)?))
}

pub fn f49(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f91(v0, v2.clone(), 0u64);
    let v9: u64 = f91(v0, v2.clone(), 1u64);
    let v10: u64 = f91(v0, v2.clone(), 2u64);
    let v11: u64 = f91(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f92(v1.clone(), f93(v8, v3, match v1.clone() {
            Adt3::C0(h1397, _, _, _, _, _, _, _) => {
                h1397
            }
        })?, f93(v9, v4, match v1.clone() {
            Adt3::C0(_, h1398, _, _, _, _, _, _) => {
                h1398
            }
        })?, f93(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1399, _, _, _, _, _) => {
                h1399
            }
        })?, f93(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1400, _, _, _, _) => {
                h1400
            }
        })?)?, f45(v0, v2)?))
    } else {
        if if if if f94(v8, v3, match v1.clone() {
            Adt3::C0(h1401, _, _, _, _, _, _, _) => {
                h1401
            }
        })? {
            f94(v9, v4, match v1.clone() {
                Adt3::C0(_, h1402, _, _, _, _, _, _) => {
                    h1402
                }
            })?
        } else {
            false
        } {
            f94(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1403, _, _, _, _, _) => {
                    h1403
                }
            })?
        } else {
            false
        } {
            f94(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1404, _, _, _, _) => {
                    h1404
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f92(v1, f95(v8, v3)?, f95(v9, v4)?, f95(v10, v5)?, f95(v11, v6)?)?, f45(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f45(v0, v2)?))
        }
    }
}

pub fn f50(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f91(v0, v2.clone(), 0u64);
    let v5: u64 = f91(v0, v2.clone(), 1u64);
    let v6: u64 = f91(v0, v2.clone(), 2u64);
    let v7: u64 = f91(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f92(v1.clone(), f96(v4, v3, match v1 {
        Adt3::C0(h1405, _, _, _, _, _, _, _) => {
            h1405
        }
    })?, v5, v6, v7)?, f45(v0, v2)?))
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1406 = v0;
    let a1407 = v3;
    if nat_lt(a1406, a1407) {
        Ok(Adt5::C1(false, {
            let a1408 = v0;
            let a1409 = 4u64;
            nat_mul(a1408, a1409)?
        }, {
            let a1410 = v0;
            let a1411 = 4u64;
            nat_mul(a1410, a1411)?
        }))
    } else {
        let v6: u64 = {
            let a1412 = v0;
            let a1413 = v3;
            nat_sub(a1412, a1413)
        };
        let v7: u64 = f45(v6, v1)?;
        let v8: u64 = f97(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1414 = v0;
    let a1415 = v3;
    if nat_lt(a1414, a1415) {
        Ok(Adt5::C1(true, 0u64, f45(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1416 = v0;
            let a1417 = v3;
            nat_sub(a1416, a1417)
        };
        let v5: u64 = f47(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f45(v4, v1)?, 3u64, v5))
    }
}

pub fn f53(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1419 = v1;
        let a1420 = match v3.clone() {
            Adt3::C0(h1418, _, _, _, _, _, _, _) => {
                h1418
            }
        };
        let a1421 = 0u64;
        nat_rem(a1419, a1420, a1421)
    };
    let v5: u64 = {
        let a1427 = {
            let a1423 = v1;
            let a1424 = match v3.clone() {
                Adt3::C0(h1422, _, _, _, _, _, _, _) => {
                    h1422
                }
            };
            let a1425 = 0u64;
            nat_quot(a1423, a1424, a1425)
        };
        let a1428 = match v3.clone() {
            Adt3::C0(_, h1426, _, _, _, _, _, _) => {
                h1426
            }
        };
        let a1429 = 0u64;
        nat_rem(a1427, a1428, a1429)
    };
    let v6: bool = f98(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f47(v1, v2, v3.clone())?
    } else {
        0u64
    }, f45(v1, v3)?))
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1430 = v3;
        let a1431 = 8u64;
        nat_add(a1430, a1431)?
    }, 0u64, f45(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f47(v0, v5, v2.clone())?
    }, 3u64, f45(v0, v2)?))
}

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1433 = v0;
        let a1434 = match v2.clone() {
            Adt3::C0(h1432, _, _, _, _, _, _, _) => {
                h1432
            }
        };
        let a1435 = 0u64;
        nat_rem(a1433, a1434, a1435)
    };
    let v7: u64 = {
        let a1437 = v0;
        let a1438 = match v2.clone() {
            Adt3::C0(h1436, _, _, _, _, _, _, _) => {
                h1436
            }
        };
        let a1439 = 0u64;
        nat_quot(a1437, a1438, a1439)
    };
    let v8: u64 = {
        let a1441 = v7;
        let a1442 = match v2.clone() {
            Adt3::C0(_, h1440, _, _, _, _, _, _) => {
                h1440
            }
        };
        let a1443 = 0u64;
        nat_rem(a1441, a1442, a1443)
    };
    let v9: u64 = {
        let a1445 = v7;
        let a1446 = match v2.clone() {
            Adt3::C0(_, h1444, _, _, _, _, _, _) => {
                h1444
            }
        };
        let a1447 = 0u64;
        nat_quot(a1445, a1446, a1447)
    };
    let v10: u64 = {
        let a1449 = v9;
        let a1450 = match v2.clone() {
            Adt3::C0(_, _, h1448, _, _, _, _, _) => {
                h1448
            }
        };
        let a1451 = 0u64;
        nat_rem(a1449, a1450, a1451)
    };
    let v11: u64 = {
        let a1453 = v9;
        let a1454 = match v2.clone() {
            Adt3::C0(_, _, h1452, _, _, _, _, _) => {
                h1452
            }
        };
        let a1455 = 0u64;
        nat_quot(a1453, a1454, a1455)
    };
    let v12: u64 = f45(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1464 = {
                let a1459 = {
                    let a1456 = v10;
                    let a1457 = v4;
                    let a1458 = 0u64;
                    nat_quot(a1456, a1457, a1458)
                };
                let a1460 = v5;
                nat_mul(a1459, a1460)?
            };
            let a1465 = {
                let a1461 = v8;
                let a1462 = v4;
                let a1463 = 0u64;
                nat_quot(a1461, a1462, a1463)
            };
            nat_add(a1464, a1465)?
        };
        let v14: u64 = {
            let a1485 = v6;
            let a1486 = {
                let a1483 = match v1.clone() {
                    Adt3::C0(h1466, _, _, _, _, _, _, _) => {
                        h1466
                    }
                };
                let a1484 = {
                    let a1481 = {
                        let a1467 = v8;
                        let a1468 = v4;
                        let a1469 = 0u64;
                        nat_rem(a1467, a1468, a1469)
                    };
                    let a1482 = {
                        let a1479 = match v1.clone() {
                            Adt3::C0(_, h1470, _, _, _, _, _, _) => {
                                h1470
                            }
                        };
                        let a1480 = {
                            let a1477 = {
                                let a1471 = v10;
                                let a1472 = v4;
                                let a1473 = 0u64;
                                nat_rem(a1471, a1472, a1473)
                            };
                            let a1478 = {
                                let a1475 = match v1.clone() {
                                    Adt3::C0(_, _, h1474, _, _, _, _, _) => {
                                        h1474
                                    }
                                };
                                let a1476 = v13;
                                nat_mul(a1475, a1476)?
                            };
                            nat_add(a1477, a1478)?
                        };
                        nat_mul(a1479, a1480)?
                    };
                    nat_add(a1481, a1482)?
                };
                nat_mul(a1483, a1484)?
            };
            nat_add(a1485, a1486)?
        };
        Ok(Adt5::C1(false, f45(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1492 = {
                let a1490 = {
                    let a1487 = v11;
                    let a1488 = v5;
                    let a1489 = 0u64;
                    nat_rem(a1487, a1488, a1489)
                };
                let a1491 = v4;
                nat_mul(a1490, a1491)?
            };
            let a1493 = v8;
            nat_add(a1492, a1493)?
        };
        let v16: u64 = {
            let a1499 = {
                let a1497 = {
                    let a1494 = v11;
                    let a1495 = v5;
                    let a1496 = 0u64;
                    nat_quot(a1494, a1495, a1496)
                };
                let a1498 = v4;
                nat_mul(a1497, a1498)?
            };
            let a1500 = v10;
            nat_add(a1499, a1500)?
        };
        let a1502 = v15;
        let a1503 = match v1.clone() {
            Adt3::C0(_, h1501, _, _, _, _, _, _) => {
                h1501
            }
        };
        if if nat_lt(a1502, a1503) {
            let a1505 = v16;
            let a1506 = match v1.clone() {
                Adt3::C0(_, _, h1504, _, _, _, _, _) => {
                    h1504
                }
            };
            nat_lt(a1505, a1506)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f45({
                let a1515 = v6;
                let a1516 = {
                    let a1513 = match v1.clone() {
                        Adt3::C0(h1507, _, _, _, _, _, _, _) => {
                            h1507
                        }
                    };
                    let a1514 = {
                        let a1511 = v15;
                        let a1512 = {
                            let a1509 = match v1.clone() {
                                Adt3::C0(_, h1508, _, _, _, _, _, _) => {
                                    h1508
                                }
                            };
                            let a1510 = v16;
                            nat_mul(a1509, a1510)?
                        };
                        nat_add(a1511, a1512)?
                    };
                    nat_mul(a1513, a1514)?
                };
                nat_add(a1515, a1516)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f45(v0, v2.clone())?;
    let v4: u64 = f45(v0, v1.clone())?;
    let a1521 = {
        let a1518 = v0;
        let a1519 = match v1 {
            Adt3::C0(h1517, _, _, _, _, _, _, _) => {
                h1517
            }
        };
        let a1520 = 0u64;
        nat_rem(a1518, a1519, a1520)
    };
    let a1522 = 0u64;
    if nat_eq(a1521, a1522) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f45({
            let a1523 = v0;
            let a1524 = 1u64;
            nat_sub(a1523, a1524)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1525 = v3;
    let a1526 = 13u64;
    if nat_eq(a1525, a1526) {
        f99(v0, v1, v4)
    } else {
        let v5: u64 = f100(match v1.clone() {
            Adt3::C0(h1527, _, _, _, _, _, _, _) => {
                h1527
            }
        }, {
            let a1528 = v3;
            let a1529 = 16u64;
            nat_eq(a1528, a1529)
        })?;
        let v6: u64 = {
            let a1530 = v0;
            let a1531 = v5;
            let a1532 = 0u64;
            nat_quot(a1530, a1531, a1532)
        };
        let v7: u64 = {
            let a1533 = v0;
            let a1534 = v5;
            let a1535 = 0u64;
            nat_rem(a1533, a1534, a1535)
        };
        let v8: u64 = {
            let a1539 = 1u64;
            let a1540 = {
                let a1537 = match v1.clone() {
                    Adt3::C0(h1536, _, _, _, _, _, _, _) => {
                        h1536
                    }
                };
                let a1538 = 2u64;
                nat_mul(a1537, a1538)?
            };
            nat_add(a1539, a1540)?
        };
        let a1541 = v7;
        let a1542 = v8;
        if nat_lt(a1541, a1542) {
            f101(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f45(v6, v2)?;
            let a1543 = v3;
            let a1544 = 14u64;
            if nat_eq(a1543, a1544) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1545 = v7;
                let a1546 = v8;
                if nat_eq(a1545, a1546) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1549 = v7;
                    let a1550 = {
                        let a1547 = v8;
                        let a1548 = 1u64;
                        nat_add(a1547, a1548)?
                    };
                    if nat_eq(a1549, a1550) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1551, _, _, _, _, _, _, _) => {
                                h1551
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

pub fn f58(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1552 = v0;
    let a1553 = 0u64;
    if nat_eq(a1552, a1553) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1560 = {
            let a1554 = v0;
            let a1555 = 1u64;
            nat_add(a1554, a1555)?
        };
        let a1561 = {
            let a1558 = 2u64;
            let a1559 = {
                let a1556 = v3;
                let a1557 = 2u64;
                nat_mul(a1556, a1557)?
            };
            nat_add(a1558, a1559)?
        };
        if nat_eq(a1560, a1561) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1562 = v0;
                let a1563 = 1u64;
                nat_sub(a1562, a1563)
            };
            let v5: u64 = {
                let a1564 = v4;
                let a1565 = 2u64;
                let a1566 = 0u64;
                nat_quot(a1564, a1565, a1566)
            };
            let a1570 = {
                let a1567 = v4;
                let a1568 = 2u64;
                let a1569 = 0u64;
                nat_rem(a1567, a1568, a1569)
            };
            let a1571 = 0u64;
            if nat_eq(a1570, a1571) {
                Ok(Adt5::C4(17u64, 0u64, f45(v5, v1)?, 4u64, f45(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f59(v0: u64) -> R<u64> {
    let a1574 = 3u64;
    let a1575 = {
        let a1572 = v0;
        let a1573 = 3u64;
        nat_mul(a1572, a1573)?
    };
    nat_add(a1574, a1575)
}

pub fn f60(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1583 = {
        let a1576 = 2u64;
        let a1577 = f61(v1.clone())?;
        nat_mul(a1576, a1577)?
    };
    let a1584 = {
        let a1581 = f61(v1)?;
        let a1582 = {
            let a1579 = 1u64;
            let a1580 = match v0 {
                Adt3::C0(h1578, _, _, _, _, _, _, _) => {
                    h1578
                }
            };
            nat_add(a1579, a1580)?
        };
        nat_mul(a1581, a1582)?
    };
    nat_add(a1583, a1584)
}

pub fn f61(v0: Adt3) -> R<u64> {
    if f72(match v0.clone() {
        Adt3::C0(h1585, _, _, _, _, _, _, _) => {
            h1585
        }
    }, match v0.clone() {
        Adt3::C0(_, h1586, _, _, _, _, _, _) => {
            h1586
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1587, _, _, _, _, _) => {
            h1587
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1588, _, _, _, _) => {
            h1588
        }
    }) {
        Ok(0u64)
    } else {
        let a1597 = match v0.clone() {
            Adt3::C0(h1589, _, _, _, _, _, _, _) => {
                h1589
            }
        };
        let a1598 = {
            let a1595 = match v0.clone() {
                Adt3::C0(_, h1590, _, _, _, _, _, _) => {
                    h1590
                }
            };
            let a1596 = {
                let a1593 = match v0.clone() {
                    Adt3::C0(_, _, h1591, _, _, _, _, _) => {
                        h1591
                    }
                };
                let a1594 = match v0 {
                    Adt3::C0(_, _, _, h1592, _, _, _, _) => {
                        h1592
                    }
                };
                nat_mul(a1593, a1594)?
            };
            nat_mul(a1595, a1596)?
        };
        nat_mul(a1597, a1598)
    }
}

pub fn f62(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f45(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f63(v0: u64, v1: u64) -> bool {
    let a1599 = v1;
    let a1600 = 2147483647u64;
    if nat_le(a1599, a1600) {
        let a1601 = v1;
        let a1602 = v0;
        nat_lt(a1601, a1602)
    } else {
        false
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1603 = f75(v0);
    match m1603 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a1605 = v2;
                let a1606 = match v7.clone() {
                    Adt11::C0(_, h1604, _) => {
                        h1604
                    }
                };
                nat_mul(a1605, a1606)?
            };
            let m1619 = f102(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1607, _, _, _, _, _, _, _) => {
                    h1607
                }
            }, match v5.clone() {
                Adt3::C0(_, h1608, _, _, _, _, _, _) => {
                    h1608
                }
            }, match v5 {
                Adt3::C0(_, _, h1609, _, _, _, _, _) => {
                    h1609
                }
            }, {
                let a1612 = match v3.clone() {
                    Adt3::C0(h1610, _, _, _, _, _, _, _) => {
                        h1610
                    }
                };
                let a1613 = match v7.clone() {
                    Adt11::C0(_, h1611, _) => {
                        h1611
                    }
                };
                nat_mul(a1612, a1613)?
            }, match v3.clone() {
                Adt3::C0(_, h1614, _, _, _, _, _, _) => {
                    h1614
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1615, _, _, _) => {
                    h1615
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1616, _, _) => {
                    h1616
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1617, _) => {
                    h1617
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1618) => {
                    h1618
                }
            })?;
            match m1619 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1620 = v0;
                    let a1621 = 0u64;
                    if if nat_eq(a1620, a1621) {
                        true
                    } else {
                        let a1622 = v0;
                        let a1623 = 26u64;
                        nat_eq(a1622, a1623)
                    } {
                        Ok(Adt5::C1(false, v9, f45({
                            let a1627 = {
                                let a1625 = v1;
                                let a1626 = match v4.clone() {
                                    Adt3::C0(h1624, _, _, _, _, _, _, _) => {
                                        h1624
                                    }
                                };
                                nat_mul(a1625, a1626)?
                            };
                            let a1628 = v8;
                            nat_add(a1627, a1628)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h1629) => {
                                h1629
                            }
                        }, f45({
                            let a1633 = {
                                let a1631 = v1;
                                let a1632 = match v4.clone() {
                                    Adt3::C0(h1630, _, _, _, _, _, _, _) => {
                                        h1630
                                    }
                                };
                                nat_mul(a1631, a1632)?
                            };
                            let a1634 = v8;
                            nat_add(a1633, a1634)?
                        }, v4.clone())?, {
                            let a1641 = {
                                let a1639 = {
                                    let a1636 = match v7 {
                                        Adt11::C0(_, h1635, _) => {
                                            h1635
                                        }
                                    };
                                    let a1637 = 1u64;
                                    nat_sub(a1636, a1637)
                                };
                                let a1640 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1638, _, _, _) => {
                                        h1638
                                    }
                                };
                                nat_mul(a1639, a1640)?
                            };
                            let a1642 = 4u64;
                            nat_add(a1641, a1642)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1643, _, _, _) => {
                                h1643
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1644 = v1;
    let a1645 = 0u64;
    if if nat_eq(a1644, a1645) {
        true
    } else {
        let a1646 = v2;
        let a1647 = 0u64;
        nat_eq(a1646, a1647)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f66(v0: u64, v1: u64) -> bool {
    let a1648 = v0;
    let a1649 = 4294967295u64;
    if nat_eq(a1648, a1649) {
        true
    } else {
        let a1650 = v0;
        let a1651 = 2147483648u64;
        if nat_lt(a1650, a1651) {
            let a1652 = v0;
            let a1653 = v1;
            nat_lt(a1652, a1653)
        } else {
            false
        }
    }
}

pub fn f67(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a1654 = v0;
        let a1655 = 8388608u64;
        let a1656 = 0u64;
        nat_quot(a1654, a1655, a1656)
    };
    let a1657 = v0;
    let a1658 = 2147483648u64;
    if if if nat_lt(a1657, a1658) {
        let a1659 = 127u64;
        let a1660 = v1;
        nat_le(a1659, a1660)
    } else {
        false
    } {
        let a1661 = v1;
        let a1662 = 190u64;
        nat_lt(a1661, a1662)
    } else {
        false
    } {
        let v2: u64 = {
            let a1666 = 8388608u64;
            let a1667 = {
                let a1663 = v0;
                let a1664 = 8388608u64;
                let a1665 = 0u64;
                nat_rem(a1663, a1664, a1665)
            };
            nat_add(a1666, a1667)?
        };
        let a1668 = v1;
        let a1669 = 150u64;
        if nat_lt(a1668, a1669) {
            let v3: u64 = f103({
                let a1670 = 150u64;
                let a1671 = v1;
                nat_sub(a1670, a1671)
            });
            let a1675 = {
                let a1672 = v2;
                let a1673 = v3;
                let a1674 = 0u64;
                nat_rem(a1672, a1673, a1674)
            };
            let a1676 = 0u64;
            if nat_eq(a1675, a1676) {
                let a1677 = v2;
                let a1678 = v3;
                let a1679 = 0u64;
                Ok(nat_quot(a1677, a1678, a1679))
            } else {
                Ok(0u64)
            }
        } else {
            let a1682 = f103({
                let a1680 = v1;
                let a1681 = 150u64;
                nat_sub(a1680, a1681)
            });
            let a1683 = v2;
            nat_mul(a1682, a1683)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f68(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1684 = v1;
    match m1684 {
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
            f104(f105(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f106(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f107(v0.clone(), v16);
            f104(f108(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
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

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = f70(match v1.clone() {
        Adt3::C0(h1685, _, _, _, _, _, _, _) => {
            h1685
        }
    })?;
    let v6: u64 = {
        let a1691 = match v1.clone() {
            Adt3::C0(_, h1686, _, _, _, _, _, _) => {
                h1686
            }
        };
        let a1692 = {
            let a1689 = match v1.clone() {
                Adt3::C0(_, _, h1687, _, _, _, _, _) => {
                    h1687
                }
            };
            let a1690 = match v1.clone() {
                Adt3::C0(_, _, _, h1688, _, _, _, _) => {
                    h1688
                }
            };
            nat_mul(a1689, a1690)?
        };
        nat_mul(a1691, a1692)?
    };
    let a1694 = v0;
    let a1695 = f22(match v1.clone() {
        Adt3::C0(h1693, _, _, _, _, _, _, _) => {
            h1693
        }
    }, v6)?;
    if nat_lt(a1694, a1695) {
        f109({
            let a1696 = v0;
            let a1697 = v5;
            let a1698 = 0u64;
            nat_rem(a1696, a1697, a1698)
        }, {
            let a1699 = v0;
            let a1700 = v5;
            let a1701 = 0u64;
            nat_quot(a1699, a1700, a1701)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f70(v0: u64) -> R<u64> {
    let a1704 = 5u64;
    let a1705 = {
        let a1702 = 4u64;
        let a1703 = v0;
        nat_mul(a1702, a1703)?
    };
    nat_add(a1704, a1705)
}

pub fn f71(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f110(match v0.clone() {
        Adt2::C0(h1706, _, _, _, _, _, _, _, _, _, _) => {
            h1706
        }
    }, match v0.clone() {
        Adt2::C0(_, h1707, _, _, _, _, _, _, _, _, _) => {
            h1707
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1708, _, _, _, _, _, _, _, _) => {
            h1708
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1709, _, _, _, _, _, _, _) => {
            h1709
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1710, _, _, _, _, _, _) => {
            h1710
        }
    }, match v1.clone() {
        Adt2::C0(h1711, _, _, _, _, _, _, _, _, _, _) => {
            h1711
        }
    }, match v1.clone() {
        Adt2::C0(_, h1712, _, _, _, _, _, _, _, _, _) => {
            h1712
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1713, _, _, _, _, _, _, _, _) => {
            h1713
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1714, _, _, _, _, _, _, _) => {
            h1714
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1715, _, _, _, _, _, _) => {
            h1715
        }
    }, match v2.clone() {
        Adt2::C0(h1716, _, _, _, _, _, _, _, _, _, _) => {
            h1716
        }
    }, match v2.clone() {
        Adt2::C0(_, h1717, _, _, _, _, _, _, _, _, _) => {
            h1717
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1718, _, _, _, _, _, _, _, _) => {
            h1718
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1719, _, _, _, _, _, _, _) => {
            h1719
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1720, _, _, _, _, _, _) => {
            h1720
        }
    }) {
        let m1722 = f75(match v0.clone() {
            Adt2::C0(h1721, _, _, _, _, _, _, _, _, _, _) => {
                h1721
            }
        });
        match m1722 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1728 = {
                    let a1725 = match v0.clone() {
                        Adt2::C0(_, h1723, _, _, _, _, _, _, _, _, _) => {
                            h1723
                        }
                    };
                    let a1726 = match v3.clone() {
                        Adt11::C0(_, h1724, _) => {
                            h1724
                        }
                    };
                    let a1727 = 0u64;
                    nat_rem(a1725, a1726, a1727)
                };
                let a1729 = 0u64;
                if if nat_eq(a1728, a1729) {
                    if f111(match v0.clone() {
                        Adt2::C0(_, h1730, _, _, _, _, _, _, _, _, _) => {
                            h1730
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1731, _, _, _, _, _, _, _, _, _) => {
                            h1731
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1732, _, _, _, _, _, _, _, _) => {
                            h1732
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1733, _, _, _, _, _, _, _) => {
                            h1733
                        }
                    }) {
                        true
                    } else {
                        let a1735 = 4u64;
                        let a1736 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1734, _, _, _, _, _) => {
                                h1734
                            }
                        };
                        if if nat_le(a1735, a1736) {
                            let a1738 = match v3.clone() {
                                Adt11::C0(_, h1737, _) => {
                                    h1737
                                }
                            };
                            let a1739 = 1u64;
                            if nat_eq(a1738, a1739) {
                                true
                            } else {
                                let a1742 = match v3.clone() {
                                    Adt11::C0(_, _, h1740) => {
                                        h1740
                                    }
                                };
                                let a1743 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1741, _, _, _, _, _) => {
                                        h1741
                                    }
                                };
                                nat_le(a1742, a1743)
                            }
                        } else {
                            false
                        } {
                            if f20(v0.clone(), v2.clone())? {
                                f20(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f17(v0.clone(), match v3 {
                        Adt11::C0(_, h1744, _) => {
                            h1744
                        }
                    });
                    Ok(Adt6::C4(if f111(match v0.clone() {
                        Adt2::C0(_, h1745, _, _, _, _, _, _, _, _, _) => {
                            h1745
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1746, _, _, _, _, _, _, _, _, _) => {
                            h1746
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1747, _, _, _, _, _, _, _, _) => {
                            h1747
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1748, _, _, _, _, _, _, _) => {
                            h1748
                        }
                    }) {
                        0u64
                    } else {
                        f60(v4.clone(), f17(v1.clone(), 1u64))?
                    }, 4u64, v4, f17(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1749, _, _, _, _, _, _, _, _, _, _) => {
                            h1749
                        }
                    }, f17(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1750 = v0;
    let a1751 = 0u64;
    if if if nat_eq(a1750, a1751) {
        true
    } else {
        let a1752 = v1;
        let a1753 = 0u64;
        nat_eq(a1752, a1753)
    } {
        true
    } else {
        let a1754 = v2;
        let a1755 = 0u64;
        nat_eq(a1754, a1755)
    } {
        true
    } else {
        let a1756 = v3;
        let a1757 = 0u64;
        nat_eq(a1756, a1757)
    }
}

pub fn f73(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1760 = match v1.clone() {
        Adt2::C0(h1758, _, _, _, _, _, _, _, _, _, _) => {
            h1758
        }
    };
    let a1761 = match v3.clone() {
        Adt2::C0(h1759, _, _, _, _, _, _, _, _, _, _) => {
            h1759
        }
    };
    if if nat_eq(a1760, a1761) {
        let a1762 = v0;
        let a1763 = 33u64;
        if nat_eq(a1762, a1763) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1765 = f75(match v1.clone() {
            Adt2::C0(h1764, _, _, _, _, _, _, _, _, _, _) => {
                h1764
            }
        });
        match m1765 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1766, _) => {
                        h1766
                    }
                };
                let v6: u64 = f13(v1.clone(), v5)?;
                let a1771 = {
                    let a1768 = match v1.clone() {
                        Adt2::C0(_, h1767, _, _, _, _, _, _, _, _, _) => {
                            h1767
                        }
                    };
                    let a1769 = v5;
                    let a1770 = 1u64;
                    nat_rem(a1768, a1769, a1770)
                };
                let a1772 = 0u64;
                if if if nat_eq(a1771, a1772) {
                    let a1777 = {
                        let a1774 = match v3.clone() {
                            Adt2::C0(_, h1773, _, _, _, _, _, _, _, _, _) => {
                                h1773
                            }
                        };
                        let a1775 = v5;
                        let a1776 = 1u64;
                        nat_rem(a1774, a1775, a1776)
                    };
                    let a1778 = 0u64;
                    nat_eq(a1777, a1778)
                } else {
                    false
                } {
                    let a1779 = v6;
                    let a1780 = f13(v3.clone(), v5)?;
                    nat_eq(a1779, a1780)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1781) => {
                            h1781
                        }
                    }, f17(v1, v5), f17(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f74(v0: u64) -> bool {
    let a1782 = v0;
    let a1783 = 0u64;
    if if nat_eq(a1782, a1783) {
        true
    } else {
        let a1784 = v0;
        let a1785 = 1u64;
        nat_eq(a1784, a1785)
    } {
        true
    } else {
        let a1786 = v0;
        let a1787 = 30u64;
        nat_eq(a1786, a1787)
    }
}

pub fn f75(v0: u64) -> Adt10 {
    let a1788 = v0;
    let a1789 = 0u64;
    if nat_eq(a1788, a1789) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1790 = v0;
        let a1791 = 1u64;
        if nat_eq(a1790, a1791) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1792 = v0;
            let a1793 = 2u64;
            if nat_eq(a1792, a1793) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1794 = v0;
                let a1795 = 3u64;
                if nat_eq(a1794, a1795) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1796 = v0;
                    let a1797 = 6u64;
                    if nat_eq(a1796, a1797) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1798 = v0;
                        let a1799 = 7u64;
                        if nat_eq(a1798, a1799) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1800 = v0;
                            let a1801 = 8u64;
                            if nat_eq(a1800, a1801) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1802 = v0;
                                let a1803 = 9u64;
                                if nat_eq(a1802, a1803) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1804 = v0;
                                    let a1805 = 10u64;
                                    if nat_eq(a1804, a1805) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1806 = v0;
                                        let a1807 = 11u64;
                                        if nat_eq(a1806, a1807) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1808 = v0;
                                            let a1809 = 12u64;
                                            if nat_eq(a1808, a1809) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1810 = v0;
                                                let a1811 = 13u64;
                                                if nat_eq(a1810, a1811) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1812 = v0;
                                                    let a1813 = 14u64;
                                                    if nat_eq(a1812, a1813) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1814 = v0;
                                                        let a1815 = 15u64;
                                                        if nat_eq(a1814, a1815) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1816 = v0;
                                                            let a1817 = 16u64;
                                                            if nat_eq(a1816, a1817) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1818 = v0;
                                                                let a1819 = 17u64;
                                                                if nat_eq(a1818, a1819) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1820 = v0;
                                                                    let a1821 = 18u64;
                                                                    if nat_eq(a1820, a1821) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1822 = v0;
                                                                        let a1823 = 19u64;
                                                                        if nat_eq(a1822, a1823) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1824 = v0;
                                                                            let a1825 = 20u64;
                                                                            if nat_eq(a1824, a1825) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1826 = v0;
                                                                                let a1827 = 21u64;
                                                                                if nat_eq(a1826, a1827) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1828 = v0;
                                                                                    let a1829 = 22u64;
                                                                                    if nat_eq(a1828, a1829) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1830 = v0;
                                                                                        let a1831 = 23u64;
                                                                                        if nat_eq(a1830, a1831) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1832 = v0;
                                                                                            let a1833 = 24u64;
                                                                                            if nat_eq(a1832, a1833) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1834 = v0;
                                                                                                let a1835 = 25u64;
                                                                                                if nat_eq(a1834, a1835) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1836 = v0;
                                                                                                    let a1837 = 26u64;
                                                                                                    if nat_eq(a1836, a1837) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1838 = v0;
                                                                                                        let a1839 = 27u64;
                                                                                                        if nat_eq(a1838, a1839) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1840 = v0;
                                                                                                            let a1841 = 28u64;
                                                                                                            if nat_eq(a1840, a1841) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1842 = v0;
                                                                                                                let a1843 = 29u64;
                                                                                                                if nat_eq(a1842, a1843) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1844 = v0;
                                                                                                                    let a1845 = 30u64;
                                                                                                                    if nat_eq(a1844, a1845) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1846 = v0;
                                                                                                                        let a1847 = 34u64;
                                                                                                                        if nat_eq(a1846, a1847) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1848 = v0;
                                                                                                                            let a1849 = 35u64;
                                                                                                                            if nat_eq(a1848, a1849) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1850 = v0;
                                                                                                                                let a1851 = 39u64;
                                                                                                                                if nat_eq(a1850, a1851) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1852 = v0;
                                                                                                                                    let a1853 = 40u64;
                                                                                                                                    if nat_eq(a1852, a1853) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1854 = v0;
                                                                                                                                        let a1855 = 41u64;
                                                                                                                                        if nat_eq(a1854, a1855) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1856 = v0;
                                                                                                                                            let a1857 = 42u64;
                                                                                                                                            if nat_eq(a1856, a1857) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1858 = v0;
                                                                                                                                                let a1859 = 43u64;
                                                                                                                                                if nat_eq(a1858, a1859) {
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

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f72(v4, v5, v6, v7) {
        f72(v0, v1, v2, v3)
    } else {
        let a1863 = {
            let a1860 = v0;
            let a1861 = v4;
            let a1862 = 1u64;
            nat_rem(a1860, a1861, a1862)
        };
        let a1864 = 0u64;
        if if if nat_eq(a1863, a1864) {
            let a1868 = {
                let a1865 = v1;
                let a1866 = v5;
                let a1867 = 1u64;
                nat_rem(a1865, a1866, a1867)
            };
            let a1869 = 0u64;
            nat_eq(a1868, a1869)
        } else {
            false
        } {
            let a1873 = {
                let a1870 = v2;
                let a1871 = v6;
                let a1872 = 1u64;
                nat_rem(a1870, a1871, a1872)
            };
            let a1874 = 0u64;
            nat_eq(a1873, a1874)
        } else {
            false
        } {
            let a1878 = {
                let a1875 = v3;
                let a1876 = v7;
                let a1877 = 1u64;
                nat_rem(a1875, a1876, a1877)
            };
            let a1879 = 0u64;
            nat_eq(a1878, a1879)
        } else {
            false
        }
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1880 = v0;
    let a1881 = v1;
    if nat_eq(a1880, a1881) {
        let a1884 = v4;
        let a1885 = {
            let a1882 = v2;
            let a1883 = v3;
            nat_add(a1882, a1883)?
        };
        Ok(nat_eq(a1884, a1885))
    } else {
        let a1886 = v2;
        let a1887 = v3;
        if nat_eq(a1886, a1887) {
            let a1888 = v4;
            let a1889 = v2;
            Ok(nat_eq(a1888, a1889))
        } else {
            Ok(false)
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1892 = {
            let a1890 = v0;
            let a1891 = f112(v1);
            nat_add(a1890, a1891)?
        };
        let a1893 = f112(v2);
        nat_add(a1892, a1893)?
    };
    let v5: u64 = {
        let a1894 = f113(v1);
        let a1895 = f113(v2);
        nat_add(a1894, a1895)?
    };
    let a1896 = v1;
    let a1897 = 4294967295u64;
    if if nat_le(a1896, a1897) {
        let a1898 = v2;
        let a1899 = 4294967295u64;
        nat_le(a1898, a1899)
    } else {
        false
    } {
        let a1900 = v5;
        let a1901 = v4;
        if nat_le(a1900, a1901) {
            let a1904 = v3;
            let a1905 = {
                let a1902 = v4;
                let a1903 = v5;
                nat_sub(a1902, a1903)
            };
            Ok(nat_eq(a1904, a1905))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f79(v0: u64, v1: u64) -> R<bool> {
    let a1906 = v1;
    let a1907 = 4294967295u64;
    if nat_le(a1906, a1907) {
        let a1910 = {
            let a1908 = f112(v1);
            let a1909 = f113(v1);
            nat_add(a1908, a1909)?
        };
        let a1911 = v0;
        Ok(nat_lt(a1910, a1911))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: Adt2) -> R<bool> {
    let a1913 = match v0.clone() {
        Adt2::C0(_, h1912, _, _, _, _, _, _, _, _, _) => {
            h1912
        }
    };
    let a1914 = 1u64;
    if if if if nat_eq(a1913, a1914) {
        true
    } else {
        let a1916 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1915, _, _, _, _, _) => {
                h1915
            }
        };
        let a1917 = 4u64;
        nat_eq(a1916, a1917)
    } {
        let a1919 = match v0.clone() {
            Adt2::C0(_, _, h1918, _, _, _, _, _, _, _, _) => {
                h1918
            }
        };
        let a1920 = 1u64;
        if nat_eq(a1919, a1920) {
            true
        } else {
            let a1925 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1921, _, _, _, _) => {
                    h1921
                }
            };
            let a1926 = {
                let a1923 = match v0.clone() {
                    Adt2::C0(_, h1922, _, _, _, _, _, _, _, _, _) => {
                        h1922
                    }
                };
                let a1924 = 4u64;
                nat_mul(a1923, a1924)?
            };
            nat_eq(a1925, a1926)
        }
    } else {
        false
    } {
        let a1928 = match v0.clone() {
            Adt2::C0(_, _, _, h1927, _, _, _, _, _, _, _) => {
                h1927
            }
        };
        let a1929 = 1u64;
        if nat_eq(a1928, a1929) {
            true
        } else {
            let a1937 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1930, _, _, _) => {
                    h1930
                }
            };
            let a1938 = {
                let a1935 = {
                    let a1933 = match v0.clone() {
                        Adt2::C0(_, h1931, _, _, _, _, _, _, _, _, _) => {
                            h1931
                        }
                    };
                    let a1934 = match v0.clone() {
                        Adt2::C0(_, _, h1932, _, _, _, _, _, _, _, _) => {
                            h1932
                        }
                    };
                    nat_mul(a1933, a1934)?
                };
                let a1936 = 4u64;
                nat_mul(a1935, a1936)?
            };
            nat_eq(a1937, a1938)
        }
    } else {
        false
    } {
        let a1940 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1939, _, _, _, _, _, _) => {
                h1939
            }
        };
        let a1941 = 1u64;
        if nat_eq(a1940, a1941) {
            Ok(true)
        } else {
            let a1952 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1942, _, _) => {
                    h1942
                }
            };
            let a1953 = {
                let a1950 = {
                    let a1948 = {
                        let a1945 = match v0.clone() {
                            Adt2::C0(_, h1943, _, _, _, _, _, _, _, _, _) => {
                                h1943
                            }
                        };
                        let a1946 = match v0.clone() {
                            Adt2::C0(_, _, h1944, _, _, _, _, _, _, _, _) => {
                                h1944
                            }
                        };
                        nat_mul(a1945, a1946)?
                    };
                    let a1949 = match v0 {
                        Adt2::C0(_, _, _, h1947, _, _, _, _, _, _, _) => {
                            h1947
                        }
                    };
                    nat_mul(a1948, a1949)?
                };
                let a1951 = 4u64;
                nat_mul(a1950, a1951)?
            };
            Ok(nat_eq(a1952, a1953))
        }
    } else {
        Ok(false)
    }
}

pub fn f81(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f114(v0.clone(), false)?;
    let v6: u64 = f114(v0, true)?;
    let a1954 = v6;
    let a1955 = v1;
    if if nat_le(a1954, a1955) {
        let a1962 = {
            let a1960 = {
                let a1956 = v1;
                let a1957 = v5;
                nat_add(a1956, a1957)?
            };
            let a1961 = {
                let a1958 = v3;
                let a1959 = 0u64;
                if nat_eq(a1958, a1959) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1960, a1961)?
        };
        let a1963 = v2;
        nat_le(a1962, a1963)
    } else {
        false
    } {
        if v4 {
            let a1968 = {
                let a1966 = {
                    let a1964 = v1;
                    let a1965 = v5;
                    nat_add(a1964, a1965)?
                };
                let a1967 = v6;
                nat_sub(a1966, a1967)
            };
            let a1969 = v2;
            Ok(nat_lt(a1968, a1969))
        } else {
            let a1974 = {
                let a1972 = {
                    let a1970 = v1;
                    let a1971 = v5;
                    nat_add(a1970, a1971)?
                };
                let a1973 = v6;
                nat_sub(a1972, a1973)
            };
            let a1975 = v2;
            Ok(nat_le(a1974, a1975))
        }
    } else {
        Ok(false)
    }
}

pub fn f82(v0: u64, v1: u64) -> bool {
    let a1976 = v0;
    let a1977 = 0u64;
    if if nat_eq(a1976, a1977) {
        true
    } else {
        let a1978 = v0;
        let a1979 = 1u64;
        nat_eq(a1978, a1979)
    } {
        let a1980 = v1;
        let a1981 = 4294967296u64;
        nat_lt(a1980, a1981)
    } else {
        false
    }
}

pub fn f83(v0: u64, v1: u64) -> R<u64> {
    let a1986 = {
        let a1984 = v0;
        let a1985 = {
            let a1982 = v1;
            let a1983 = 1u64;
            nat_sub(a1982, a1983)
        };
        nat_add(a1984, a1985)?
    };
    let a1987 = v1;
    let a1988 = 0u64;
    Ok(nat_quot(a1986, a1987, a1988))
}

pub fn f84(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f72(match v0.clone() {
        Adt2::C0(_, h1989, _, _, _, _, _, _, _, _, _) => {
            h1989
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1990, _, _, _, _, _, _, _, _) => {
            h1990
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1991, _, _, _, _, _, _, _) => {
            h1991
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1992, _, _, _, _, _, _) => {
            h1992
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1995 = match v1.clone() {
                Adt2::C0(_, h1993, _, _, _, _, _, _, _, _, _) => {
                    h1993
                }
            };
            let a1996 = match v1.clone() {
                Adt2::C0(_, _, h1994, _, _, _, _, _, _, _, _) => {
                    h1994
                }
            };
            nat_mul(a1995, a1996)?
        };
        let v5: u64 = {
            let a1998 = v4;
            let a1999 = match v1.clone() {
                Adt2::C0(_, _, _, h1997, _, _, _, _, _, _, _) => {
                    h1997
                }
            };
            nat_mul(a1998, a1999)?
        };
        let a2012 = {
            let a2010 = {
                let a2001 = match v0.clone() {
                    Adt2::C0(_, h2000, _, _, _, _, _, _, _, _, _) => {
                        h2000
                    }
                };
                let a2002 = 1u64;
                nat_sub(a2001, a2002)
            };
            let a2011 = {
                let a2008 = f115(match v0.clone() {
                    Adt2::C0(_, _, h2003, _, _, _, _, _, _, _, _) => {
                        h2003
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2004, _, _, _, _, _, _, _, _, _) => {
                        h2004
                    }
                })?;
                let a2009 = f115(match v0 {
                    Adt2::C0(_, _, _, h2005, _, _, _, _, _, _, _) => {
                        h2005
                    }
                }, v2, {
                    let a2006 = v3;
                    let a2007 = v5;
                    nat_mul(a2006, a2007)?
                }, v4)?;
                nat_add(a2008, a2009)?
            };
            nat_add(a2010, a2011)?
        };
        let a2013 = f13(v1, 1u64)?;
        Ok(nat_lt(a2012, a2013))
    }
}

pub fn f85(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2014 = v0;
    let a2015 = 2147483648u64;
    if if if if nat_lt(a2014, a2015) {
        let a2016 = v1;
        let a2017 = 2147483648u64;
        nat_lt(a2016, a2017)
    } else {
        false
    } {
        let a2020 = {
            let a2018 = v0;
            let a2019 = v1;
            nat_mul(a2018, a2019)?
        };
        let a2021 = 2147483648u64;
        nat_lt(a2020, a2021)
    } else {
        false
    } {
        let a2026 = {
            let a2024 = {
                let a2022 = v0;
                let a2023 = 1u64;
                nat_sub(a2022, a2023)
            };
            let a2025 = v2;
            nat_mul(a2024, a2025)?
        };
        let a2027 = 2147483648u64;
        nat_lt(a2026, a2027)
    } else {
        false
    } {
        let a2032 = {
            let a2030 = {
                let a2028 = v1;
                let a2029 = 1u64;
                nat_sub(a2028, a2029)
            };
            let a2031 = v2;
            nat_mul(a2030, a2031)?
        };
        let a2033 = 2147483648u64;
        Ok(nat_lt(a2032, a2033))
    } else {
        Ok(false)
    }
}

pub fn f86(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2034 = v1;
    let a2035 = 13u64;
    if if nat_eq(a2034, a2035) {
        f72(match v0.clone() {
            Adt3::C0(h2036, _, _, _, _, _, _, _) => {
                h2036
            }
        }, match v0.clone() {
            Adt3::C0(_, h2037, _, _, _, _, _, _) => {
                h2037
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2038, _, _, _, _, _) => {
                h2038
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2039, _, _, _, _) => {
                h2039
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2045 = match v0.clone() {
                Adt3::C0(_, h2040, _, _, _, _, _, _) => {
                    h2040
                }
            };
            let a2046 = {
                let a2043 = match v0.clone() {
                    Adt3::C0(_, _, h2041, _, _, _, _, _) => {
                        h2041
                    }
                };
                let a2044 = match v0.clone() {
                    Adt3::C0(_, _, _, h2042, _, _, _, _) => {
                        h2042
                    }
                };
                nat_mul(a2043, a2044)?
            };
            nat_mul(a2045, a2046)?
        };
        let a2047 = v1;
        let a2048 = 13u64;
        if nat_eq(a2047, a2048) {
            let a2057 = 2u64;
            let a2058 = if f116(v0.clone(), v2) {
                let a2050 = v3;
                let a2051 = f100(match v0 {
                    Adt3::C0(h2049, _, _, _, _, _, _, _) => {
                        h2049
                    }
                }, false)?;
                nat_mul(a2050, a2051)?
            } else {
                let a2055 = v3;
                let a2056 = {
                    let a2053 = match v0 {
                        Adt3::C0(h2052, _, _, _, _, _, _, _) => {
                            h2052
                        }
                    };
                    let a2054 = 2u64;
                    nat_mul(a2053, a2054)?
                };
                nat_mul(a2055, a2056)?
            };
            nat_add(a2057, a2058)
        } else {
            let a2062 = v3;
            let a2063 = f100(match v0 {
                Adt3::C0(h2059, _, _, _, _, _, _, _) => {
                    h2059
                }
            }, {
                let a2060 = v1;
                let a2061 = 16u64;
                nat_eq(a2060, a2061)
            })?;
            nat_mul(a2062, a2063)
        }
    }
}

pub fn f87(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m2064 = v3;
    match m2064 {
        None => {
            f117(v0, v1, match v2.clone() {
                Adt2::C0(h2065, _, _, _, _, _, _, _, _, _, _) => {
                    h2065
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h2066, _, _, _, _, _, _, _, _, _, _) => {
                    h2066
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h2067, _, _, _, _, _, _, _, _, _) => {
                    h2067
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2068, _, _, _, _, _, _, _, _) => {
                    h2068
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2069, _, _, _, _, _, _, _) => {
                    h2069
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2070, _, _, _, _, _, _) => {
                    h2070
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h2071, _, _, _, _, _, _, _, _, _) => {
                    h2071
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2072, _, _, _, _, _, _, _, _) => {
                    h2072
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2073, _, _, _, _, _, _, _) => {
                    h2073
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2074, _, _, _, _, _, _) => {
                    h2074
                }
            })
        }
        Some(v5) => {
            f117(v0, v1, match v2.clone() {
                Adt2::C0(h2075, _, _, _, _, _, _, _, _, _, _) => {
                    h2075
                }
            }, match v5.clone() {
                Adt2::C0(h2076, _, _, _, _, _, _, _, _, _, _) => {
                    h2076
                }
            }, match v4.clone() {
                Adt2::C0(h2077, _, _, _, _, _, _, _, _, _, _) => {
                    h2077
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h2078, _, _, _, _, _, _, _, _, _) => {
                    h2078
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2079, _, _, _, _, _, _, _, _) => {
                    h2079
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2080, _, _, _, _, _, _, _) => {
                    h2080
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2081, _, _, _, _, _, _) => {
                    h2081
                }
            }, match v5.clone() {
                Adt2::C0(_, h2082, _, _, _, _, _, _, _, _, _) => {
                    h2082
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h2083, _, _, _, _, _, _, _, _) => {
                    h2083
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h2084, _, _, _, _, _, _, _) => {
                    h2084
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h2085, _, _, _, _, _, _) => {
                    h2085
                }
            }, match v4.clone() {
                Adt2::C0(_, h2086, _, _, _, _, _, _, _, _, _) => {
                    h2086
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2087, _, _, _, _, _, _, _, _) => {
                    h2087
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2088, _, _, _, _, _, _, _) => {
                    h2088
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2089, _, _, _, _, _, _) => {
                    h2089
                }
            })
        }
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2102 = {
        let a2098 = {
            let a2094 = {
                let a2090 = v0;
                let a2091 = v4;
                nat_mul(a2090, a2091)?
            };
            let a2095 = {
                let a2092 = v1;
                let a2093 = v5;
                nat_mul(a2092, a2093)?
            };
            nat_add(a2094, a2095)?
        };
        let a2099 = {
            let a2096 = v2;
            let a2097 = v6;
            nat_mul(a2096, a2097)?
        };
        nat_add(a2098, a2099)?
    };
    let a2103 = {
        let a2100 = v3;
        let a2101 = v7;
        nat_mul(a2100, a2101)?
    };
    nat_add(a2102, a2103)
}

pub fn f89(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2108 = {
        let a2106 = {
            let a2104 = v0;
            let a2105 = v1;
            nat_sub(a2104, a2105)
        };
        let a2107 = 1u64;
        nat_sub(a2106, a2107)
    };
    let a2109 = v2;
    nat_add(a2108, a2109)
}

pub fn f90(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f88({
        let a2110 = v0;
        let a2111 = v4;
        let a2112 = 0u64;
        nat_rem(a2110, a2111, a2112)
    }, {
        let a2113 = v1;
        let a2114 = v5;
        let a2115 = 0u64;
        nat_rem(a2113, a2114, a2115)
    }, {
        let a2116 = v2;
        let a2117 = v6;
        let a2118 = 0u64;
        nat_rem(a2116, a2117, a2118)
    }, {
        let a2119 = v3;
        let a2120 = v7;
        let a2121 = 0u64;
        nat_rem(a2119, a2120, a2121)
    }, v8, v9, v10, v11)
}

pub fn f91(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2123 = v0;
        let a2124 = match v1.clone() {
            Adt3::C0(h2122, _, _, _, _, _, _, _) => {
                h2122
            }
        };
        let a2125 = 0u64;
        nat_quot(a2123, a2124, a2125)
    };
    let v4: u64 = {
        let a2127 = v3;
        let a2128 = match v1.clone() {
            Adt3::C0(_, h2126, _, _, _, _, _, _) => {
                h2126
            }
        };
        let a2129 = 0u64;
        nat_quot(a2127, a2128, a2129)
    };
    let a2130 = v2;
    let a2131 = 0u64;
    if nat_eq(a2130, a2131) {
        let a2133 = v0;
        let a2134 = match v1 {
            Adt3::C0(h2132, _, _, _, _, _, _, _) => {
                h2132
            }
        };
        let a2135 = 0u64;
        nat_rem(a2133, a2134, a2135)
    } else {
        let a2136 = v2;
        let a2137 = 1u64;
        if nat_eq(a2136, a2137) {
            let a2139 = v3;
            let a2140 = match v1 {
                Adt3::C0(_, h2138, _, _, _, _, _, _) => {
                    h2138
                }
            };
            let a2141 = 0u64;
            nat_rem(a2139, a2140, a2141)
        } else {
            let a2142 = v2;
            let a2143 = 2u64;
            if nat_eq(a2142, a2143) {
                let a2145 = v4;
                let a2146 = match v1 {
                    Adt3::C0(_, _, h2144, _, _, _, _, _) => {
                        h2144
                    }
                };
                let a2147 = 0u64;
                nat_rem(a2145, a2146, a2147)
            } else {
                let a2149 = v4;
                let a2150 = match v1 {
                    Adt3::C0(_, _, h2148, _, _, _, _, _) => {
                        h2148
                    }
                };
                let a2151 = 0u64;
                nat_quot(a2149, a2150, a2151)
            }
        }
    }
}

pub fn f92(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f88(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2152, _, _, _) => {
            h2152
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2153, _, _) => {
            h2153
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2154, _) => {
            h2154
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2155) => {
            h2155
        }
    })
}

pub fn f93(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2156 = v1;
    let a2157 = 2147483648u64;
    if nat_lt(a2156, a2157) {
        let a2168 = {
            let a2166 = {
                let a2161 = {
                    let a2158 = v0;
                    let a2159 = v2;
                    let a2160 = 0u64;
                    nat_rem(a2158, a2159, a2160)
                };
                let a2162 = v2;
                nat_add(a2161, a2162)?
            };
            let a2167 = {
                let a2163 = v1;
                let a2164 = v2;
                let a2165 = 0u64;
                nat_rem(a2163, a2164, a2165)
            };
            nat_sub(a2166, a2167)
        };
        let a2169 = v2;
        let a2170 = 0u64;
        Ok(nat_rem(a2168, a2169, a2170))
    } else {
        let a2173 = {
            let a2171 = v0;
            let a2172 = f113(v1);
            nat_add(a2171, a2172)?
        };
        let a2174 = v2;
        let a2175 = 0u64;
        Ok(nat_rem(a2173, a2174, a2175))
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2178 = f112(v1);
    let a2179 = {
        let a2176 = v0;
        let a2177 = f113(v1);
        nat_add(a2176, a2177)?
    };
    if nat_le(a2178, a2179) {
        let a2180 = f95(v0, v1)?;
        let a2181 = v2;
        Ok(nat_lt(a2180, a2181))
    } else {
        Ok(false)
    }
}

pub fn f95(v0: u64, v1: u64) -> R<u64> {
    let a2184 = {
        let a2182 = v0;
        let a2183 = f113(v1);
        nat_add(a2182, a2183)?
    };
    let a2185 = f112(v1);
    Ok(nat_sub(a2184, a2185))
}

pub fn f96(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2186 = v0;
    let a2187 = v1;
    if nat_lt(a2186, a2187) {
        let a2188 = v1;
        let a2189 = v0;
        Ok(nat_sub(a2188, a2189))
    } else {
        let a2192 = v0;
        let a2193 = {
            let a2190 = v1;
            let a2191 = v2;
            nat_add(a2190, a2191)?
        };
        if nat_lt(a2192, a2193) {
            let a2194 = v0;
            let a2195 = v1;
            Ok(nat_sub(a2194, a2195))
        } else {
            let a2202 = {
                let a2198 = 2u64;
                let a2199 = {
                    let a2196 = v2;
                    let a2197 = 1u64;
                    nat_sub(a2196, a2197)
                };
                nat_mul(a2198, a2199)?
            };
            let a2203 = {
                let a2200 = v0;
                let a2201 = v1;
                nat_sub(a2200, a2201)
            };
            Ok(nat_sub(a2202, a2203))
        }
    }
}

pub fn f97(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2206 = {
        let a2204 = v2;
        let a2205 = f118(v1.clone(), f91(v0, v1.clone(), 0u64), f91(v0, v1.clone(), 1u64), f91(v0, v1.clone(), 2u64), f91(v0, v1.clone(), 3u64), false)?;
        nat_add(a2204, a2205)?
    };
    let a2207 = f118(v1.clone(), f91(v0, v1.clone(), 0u64), f91(v0, v1.clone(), 1u64), f91(v0, v1.clone(), 2u64), f91(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2206, a2207))
}

pub fn f98(v0: u64, v1: u64, v2: u64) -> bool {
    let a2208 = v0;
    let a2209 = 0u64;
    if nat_eq(a2208, a2209) {
        let a2210 = v2;
        let a2211 = v1;
        nat_le(a2210, a2211)
    } else {
        let a2212 = v0;
        let a2213 = 1u64;
        if nat_eq(a2212, a2213) {
            let a2214 = v2;
            let a2215 = v1;
            nat_lt(a2214, a2215)
        } else {
            let a2216 = v0;
            let a2217 = 2u64;
            if nat_eq(a2216, a2217) {
                let a2218 = v1;
                let a2219 = v2;
                nat_le(a2218, a2219)
            } else {
                let a2220 = v0;
                let a2221 = 3u64;
                if nat_eq(a2220, a2221) {
                    let a2222 = v1;
                    let a2223 = v2;
                    nat_lt(a2222, a2223)
                } else {
                    let a2224 = v1;
                    let a2225 = v2;
                    nat_eq(a2224, a2225)
                }
            }
        }
    }
}

pub fn f99(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2226 = v0;
    let a2227 = 0u64;
    if nat_eq(a2226, a2227) {
        Ok(f119(0u64))
    } else {
        let a2230 = {
            let a2228 = v0;
            let a2229 = 1u64;
            nat_add(a2228, a2229)?
        };
        let a2231 = f86(v1.clone(), 13u64, v2)?;
        if nat_eq(a2230, a2231) {
            Ok(Adt5::C4({
                let a2232 = v2;
                let a2233 = 0u64;
                if nat_eq(a2232, a2233) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2234 = v0;
                let a2235 = 1u64;
                nat_sub(a2234, a2235)
            };
            if f116(v1.clone(), v2) {
                let v4: u64 = f100(match v1.clone() {
                    Adt3::C0(h2236, _, _, _, _, _, _, _) => {
                        h2236
                    }
                }, false)?;
                let v5: u64 = {
                    let a2237 = v3;
                    let a2238 = v4;
                    let a2239 = 0u64;
                    nat_quot(a2237, a2238, a2239)
                };
                let v6: u64 = {
                    let a2240 = v3;
                    let a2241 = v4;
                    let a2242 = 0u64;
                    nat_rem(a2240, a2241, a2242)
                };
                let a2245 = {
                    let a2243 = v6;
                    let a2244 = 1u64;
                    nat_add(a2243, a2244)?
                };
                let a2246 = v4;
                if nat_eq(a2245, a2246) {
                    Ok(Adt5::C4({
                        let a2247 = v2;
                        let a2248 = 0u64;
                        if nat_eq(a2247, a2248) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f101(v6, v5, v1, v2)
                }
            } else {
                f120({
                    let a2249 = v3;
                    let a2250 = 2u64;
                    let a2251 = 0u64;
                    nat_quot(a2249, a2250, a2251)
                }, {
                    let a2252 = v3;
                    let a2253 = 2u64;
                    let a2254 = 0u64;
                    nat_rem(a2252, a2253, a2254)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f100(v0: u64, v1: bool) -> R<u64> {
    let a2257 = {
        let a2255 = v0;
        let a2256 = 2u64;
        nat_mul(a2255, a2256)?
    };
    let a2258 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2257, a2258)
}

pub fn f101(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2259 = v0;
    let a2260 = 0u64;
    if nat_eq(a2259, a2260) {
        Ok(f119(1u64))
    } else {
        f120({
            let a2269 = {
                let a2262 = v1;
                let a2263 = match v2.clone() {
                    Adt3::C0(h2261, _, _, _, _, _, _, _) => {
                        h2261
                    }
                };
                nat_mul(a2262, a2263)?
            };
            let a2270 = {
                let a2266 = {
                    let a2264 = v0;
                    let a2265 = 1u64;
                    nat_sub(a2264, a2265)
                };
                let a2267 = 2u64;
                let a2268 = 0u64;
                nat_quot(a2266, a2267, a2268)
            };
            nat_add(a2269, a2270)?
        }, {
            let a2273 = {
                let a2271 = v0;
                let a2272 = 1u64;
                nat_sub(a2271, a2272)
            };
            let a2274 = 2u64;
            let a2275 = 0u64;
            nat_rem(a2273, a2274, a2275)
        }, v2, v3, 1u64)
    }
}

pub fn f102(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2276 = f75(v0);
    match m2276 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f121(v0) {
                let a2277 = v1;
                let a2278 = v7;
                nat_lt(a2277, a2278)
            } else {
                false
            } {
                f63(v8, v2)
            } else {
                false
            } {
                let a2279 = v3;
                let a2280 = f122(v4, v5, v6)?;
                nat_lt(a2279, a2280)
            } else {
                false
            } {
                let a2285 = {
                    let a2282 = v7;
                    let a2283 = match v13.clone() {
                        Adt11::C0(_, h2281, _) => {
                            h2281
                        }
                    };
                    let a2284 = 0u64;
                    nat_rem(a2282, a2283, a2284)
                };
                let a2286 = 0u64;
                nat_eq(a2285, a2286)
            } else {
                false
            } {
                Ok(Some({
                    let a2315 = {
                        let a2305 = {
                            let a2295 = {
                                let a2291 = {
                                    let a2288 = v1;
                                    let a2289 = match v13 {
                                        Adt11::C0(_, h2287, _) => {
                                            h2287
                                        }
                                    };
                                    let a2290 = 0u64;
                                    nat_quot(a2288, a2289, a2290)
                                };
                                let a2292 = v9;
                                nat_mul(a2291, a2292)?
                            };
                            let a2296 = {
                                let a2293 = v2;
                                let a2294 = v10;
                                nat_mul(a2293, a2294)?
                            };
                            nat_add(a2295, a2296)?
                        };
                        let a2306 = {
                            let a2303 = {
                                let a2300 = {
                                    let a2297 = v3;
                                    let a2298 = v4;
                                    let a2299 = 0u64;
                                    nat_quot(a2297, a2298, a2299)
                                };
                                let a2301 = v5;
                                let a2302 = 0u64;
                                nat_rem(a2300, a2301, a2302)
                            };
                            let a2304 = v11;
                            nat_mul(a2303, a2304)?
                        };
                        nat_add(a2305, a2306)?
                    };
                    let a2316 = {
                        let a2313 = {
                            let a2310 = {
                                let a2307 = v3;
                                let a2308 = v4;
                                let a2309 = 0u64;
                                nat_quot(a2307, a2308, a2309)
                            };
                            let a2311 = v5;
                            let a2312 = 0u64;
                            nat_quot(a2310, a2311, a2312)
                        };
                        let a2314 = v12;
                        nat_mul(a2313, a2314)?
                    };
                    nat_add(a2315, a2316)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f103(v0: u64) -> u64 {
    let a2317 = v0;
    let a2318 = 40u64;
    if nat_lt(a2317, a2318) {
        let a2319 = v0;
        let a2320 = 20u64;
        if nat_lt(a2319, a2320) {
            let a2321 = v0;
            let a2322 = 10u64;
            if nat_lt(a2321, a2322) {
                let a2323 = v0;
                let a2324 = 5u64;
                if nat_lt(a2323, a2324) {
                    let a2325 = v0;
                    let a2326 = 2u64;
                    if nat_lt(a2325, a2326) {
                        let a2327 = v0;
                        let a2328 = 1u64;
                        if nat_lt(a2327, a2328) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2329 = v0;
                        let a2330 = 3u64;
                        if nat_lt(a2329, a2330) {
                            4u64
                        } else {
                            let a2331 = v0;
                            let a2332 = 4u64;
                            if nat_lt(a2331, a2332) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2333 = v0;
                    let a2334 = 7u64;
                    if nat_lt(a2333, a2334) {
                        let a2335 = v0;
                        let a2336 = 6u64;
                        if nat_lt(a2335, a2336) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2337 = v0;
                        let a2338 = 8u64;
                        if nat_lt(a2337, a2338) {
                            128u64
                        } else {
                            let a2339 = v0;
                            let a2340 = 9u64;
                            if nat_lt(a2339, a2340) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2341 = v0;
                let a2342 = 15u64;
                if nat_lt(a2341, a2342) {
                    let a2343 = v0;
                    let a2344 = 12u64;
                    if nat_lt(a2343, a2344) {
                        let a2345 = v0;
                        let a2346 = 11u64;
                        if nat_lt(a2345, a2346) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2347 = v0;
                        let a2348 = 13u64;
                        if nat_lt(a2347, a2348) {
                            4096u64
                        } else {
                            let a2349 = v0;
                            let a2350 = 14u64;
                            if nat_lt(a2349, a2350) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2351 = v0;
                    let a2352 = 17u64;
                    if nat_lt(a2351, a2352) {
                        let a2353 = v0;
                        let a2354 = 16u64;
                        if nat_lt(a2353, a2354) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2355 = v0;
                        let a2356 = 18u64;
                        if nat_lt(a2355, a2356) {
                            131072u64
                        } else {
                            let a2357 = v0;
                            let a2358 = 19u64;
                            if nat_lt(a2357, a2358) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2359 = v0;
            let a2360 = 30u64;
            if nat_lt(a2359, a2360) {
                let a2361 = v0;
                let a2362 = 25u64;
                if nat_lt(a2361, a2362) {
                    let a2363 = v0;
                    let a2364 = 22u64;
                    if nat_lt(a2363, a2364) {
                        let a2365 = v0;
                        let a2366 = 21u64;
                        if nat_lt(a2365, a2366) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2367 = v0;
                        let a2368 = 23u64;
                        if nat_lt(a2367, a2368) {
                            4194304u64
                        } else {
                            let a2369 = v0;
                            let a2370 = 24u64;
                            if nat_lt(a2369, a2370) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2371 = v0;
                    let a2372 = 27u64;
                    if nat_lt(a2371, a2372) {
                        let a2373 = v0;
                        let a2374 = 26u64;
                        if nat_lt(a2373, a2374) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2375 = v0;
                        let a2376 = 28u64;
                        if nat_lt(a2375, a2376) {
                            134217728u64
                        } else {
                            let a2377 = v0;
                            let a2378 = 29u64;
                            if nat_lt(a2377, a2378) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2379 = v0;
                let a2380 = 35u64;
                if nat_lt(a2379, a2380) {
                    let a2381 = v0;
                    let a2382 = 32u64;
                    if nat_lt(a2381, a2382) {
                        let a2383 = v0;
                        let a2384 = 31u64;
                        if nat_lt(a2383, a2384) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2385 = v0;
                        let a2386 = 33u64;
                        if nat_lt(a2385, a2386) {
                            4294967296u64
                        } else {
                            let a2387 = v0;
                            let a2388 = 34u64;
                            if nat_lt(a2387, a2388) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2389 = v0;
                    let a2390 = 37u64;
                    if nat_lt(a2389, a2390) {
                        let a2391 = v0;
                        let a2392 = 36u64;
                        if nat_lt(a2391, a2392) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2393 = v0;
                        let a2394 = 38u64;
                        if nat_lt(a2393, a2394) {
                            137438953472u64
                        } else {
                            let a2395 = v0;
                            let a2396 = 39u64;
                            if nat_lt(a2395, a2396) {
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

pub fn f104(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a2397 = v7;
        let a2398 = 6u64;
        if nat_eq(a2397, a2398) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f123(v3, v4), f123(v5, v6), f124(v7, v8), f125(v1), f125(v9))
}

pub fn f105(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m2399 = v0;
    match m2399 {
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
        Adt4::C24(_) => {
            Adt7::C2(v1, v2, true)
        }
    }
}

pub fn f106(v0: Adt4) -> u64 {
    let m2400 = v0;
    match m2400 {
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
        Adt4::C24(_) => {
            0u64
        }
    }
}

pub fn f107(v0: Adt4, v1: u64) -> u64 {
    let m2401 = v0;
    match m2401 {
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
            let a2402 = v1;
            let a2403 = 6u64;
            if nat_eq(a2402, a2403) {
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
        Adt4::C24(_) => {
            0u64
        }
    }
}

pub fn f108(v0: Adt4, v1: u64) -> Adt7 {
    let a2404 = v1;
    let a2405 = 4u64;
    if nat_lt(a2404, a2405) {
        Adt7::C0(v1)
    } else {
        let a2406 = v1;
        let a2407 = 4u64;
        if nat_eq(a2406, a2407) {
            Adt7::C1(0u64)
        } else {
            let a2408 = v1;
            let a2409 = 5u64;
            if nat_eq(a2408, a2409) {
                Adt7::C1(3u64)
            } else {
                let a2410 = v1;
                let a2411 = 6u64;
                if nat_eq(a2410, a2411) {
                    Adt7::C2(f107(v0, v1), 0u64, false)
                } else {
                    let a2412 = v1;
                    let a2413 = 7u64;
                    if nat_eq(a2412, a2413) {
                        Adt7::C2(0u64, f107(v0, v1), false)
                    } else {
                        let a2414 = v1;
                        let a2415 = 16u64;
                        if nat_lt(a2414, a2415) {
                            Adt7::C1({
                                let a2416 = v1;
                                let a2417 = 8u64;
                                nat_sub(a2416, a2417)
                            })
                        } else {
                            let a2418 = v1;
                            let a2419 = 20u64;
                            if nat_lt(a2418, a2419) {
                                Adt7::C5({
                                    let a2420 = v1;
                                    let a2421 = 16u64;
                                    nat_sub(a2420, a2421)
                                })
                            } else {
                                let a2422 = v1;
                                let a2423 = 20u64;
                                if nat_eq(a2422, a2423) {
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

pub fn f109(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2424, _, _, _, _, _, _, _) => {
            h2424
        }
    };
    let v7: u64 = {
        let a2427 = 1u64;
        let a2428 = {
            let a2425 = 3u64;
            let a2426 = v6;
            nat_mul(a2425, a2426)?
        };
        nat_add(a2427, a2428)?
    };
    let a2429 = v0;
    let a2430 = 0u64;
    if nat_eq(a2429, a2430) {
        Ok(f119(0u64))
    } else {
        let a2431 = v0;
        let a2432 = v7;
        if nat_lt(a2431, a2432) {
            let v8: u64 = {
                let a2433 = v0;
                let a2434 = 1u64;
                nat_sub(a2433, a2434)
            };
            let v9: u64 = {
                let a2440 = {
                    let a2435 = v1;
                    let a2436 = v6;
                    nat_mul(a2435, a2436)?
                };
                let a2441 = {
                    let a2437 = v8;
                    let a2438 = 3u64;
                    let a2439 = 0u64;
                    nat_quot(a2437, a2438, a2439)
                };
                nat_add(a2440, a2441)?
            };
            let v10: u64 = f45(v9, v2)?;
            let v11: u64 = {
                let a2442 = v8;
                let a2443 = 3u64;
                let a2444 = 0u64;
                nat_rem(a2442, a2443, a2444)
            };
            let a2445 = v11;
            let a2446 = 0u64;
            if nat_eq(a2445, a2446) {
                Ok(Adt5::C4(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a2447 = v11;
                let a2448 = 1u64;
                if nat_eq(a2447, a2448) {
                    Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a2449 = v0;
            let a2450 = v7;
            if nat_eq(a2449, a2450) {
                Ok(Adt5::C4(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2453 = v0;
                let a2454 = {
                    let a2451 = v7;
                    let a2452 = 1u64;
                    nat_add(a2451, a2452)?
                };
                if nat_eq(a2453, a2454) {
                    Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2457 = v0;
                    let a2458 = {
                        let a2455 = v7;
                        let a2456 = 2u64;
                        nat_add(a2455, a2456)?
                    };
                    if nat_eq(a2457, a2458) {
                        Ok(Adt5::C4(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a2461 = v0;
                        let a2462 = {
                            let a2459 = v7;
                            let a2460 = 3u64;
                            nat_add(a2459, a2460)?
                        };
                        if nat_eq(a2461, a2462) {
                            let a2463 = v5;
                            let a2464 = 1u64;
                            if if nat_eq(a2463, a2464) {
                                true
                            } else {
                                let a2465 = v5;
                                let a2466 = 2u64;
                                nat_eq(a2465, a2466)
                            } {
                                Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a2473 = {
                                    let a2467 = v1;
                                    let a2468 = v6;
                                    nat_mul(a2467, a2468)?
                                };
                                let a2474 = {
                                    let a2471 = v0;
                                    let a2472 = {
                                        let a2469 = v7;
                                        let a2470 = 4u64;
                                        nat_add(a2469, a2470)?
                                    };
                                    nat_sub(a2471, a2472)
                                };
                                nat_add(a2473, a2474)?
                            };
                            Ok(Adt5::C4(10u64, 0u64, f45(v12, v2)?, 1u64, 2u64, 3u64, f45(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f110(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f126(v0, v1, v6, v7, v8) {
        let a2475 = v5;
        let a2476 = 26u64;
        nat_eq(a2475, a2476)
    } else {
        false
    } {
        let a2477 = v3;
        let a2478 = v7;
        nat_eq(a2477, a2478)
    } else {
        false
    } {
        let a2479 = v4;
        let a2480 = v8;
        nat_eq(a2479, a2480)
    } else {
        false
    } {
        let a2481 = v9;
        let a2482 = 1u64;
        nat_eq(a2481, a2482)
    } else {
        false
    } {
        let a2483 = v10;
        let a2484 = f127(v0);
        nat_eq(a2483, a2484)
    } else {
        false
    } {
        let a2485 = v11;
        let a2486 = v1;
        nat_eq(a2485, a2486)
    } else {
        false
    } {
        let a2487 = v12;
        let a2488 = v6;
        nat_eq(a2487, a2488)
    } else {
        false
    } {
        let a2489 = v13;
        let a2490 = v7;
        nat_eq(a2489, a2490)
    } else {
        false
    } {
        let a2491 = v14;
        let a2492 = v8;
        nat_eq(a2491, a2492)
    } else {
        false
    }
}

pub fn f111(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2493 = v0;
    let a2494 = 0u64;
    if if if nat_eq(a2493, a2494) {
        true
    } else {
        let a2495 = v1;
        let a2496 = 0u64;
        nat_eq(a2495, a2496)
    } {
        true
    } else {
        let a2497 = v2;
        let a2498 = 0u64;
        nat_eq(a2497, a2498)
    } {
        true
    } else {
        let a2499 = v3;
        let a2500 = 0u64;
        nat_eq(a2499, a2500)
    }
}

pub fn f112(v0: u64) -> u64 {
    let a2501 = v0;
    let a2502 = 2147483648u64;
    if nat_lt(a2501, a2502) {
        v0
    } else {
        0u64
    }
}

pub fn f113(v0: u64) -> u64 {
    let a2503 = v0;
    let a2504 = 2147483648u64;
    if nat_lt(a2503, a2504) {
        0u64
    } else {
        let a2505 = 4294967296u64;
        let a2506 = v0;
        nat_sub(a2505, a2506)
    }
}

pub fn f114(v0: Adt3, v1: bool) -> R<u64> {
    f118(v0.clone(), {
        let a2508 = match v0.clone() {
            Adt3::C0(h2507, _, _, _, _, _, _, _) => {
                h2507
            }
        };
        let a2509 = 1u64;
        nat_sub(a2508, a2509)
    }, {
        let a2511 = match v0.clone() {
            Adt3::C0(_, h2510, _, _, _, _, _, _) => {
                h2510
            }
        };
        let a2512 = 1u64;
        nat_sub(a2511, a2512)
    }, {
        let a2514 = match v0.clone() {
            Adt3::C0(_, _, h2513, _, _, _, _, _) => {
                h2513
            }
        };
        let a2515 = 1u64;
        nat_sub(a2514, a2515)
    }, {
        let a2517 = match v0 {
            Adt3::C0(_, _, _, h2516, _, _, _, _) => {
                h2516
            }
        };
        let a2518 = 1u64;
        nat_sub(a2517, a2518)
    }, v1)
}

pub fn f115(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2519 = v0;
    let a2520 = 0u64;
    if nat_eq(a2519, a2520) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2521 = v0;
            let a2522 = 1u64;
            nat_sub(a2521, a2522)
        };
        let v5: u64 = {
            let a2523 = v4;
            let a2524 = v1;
            let a2525 = 0u64;
            nat_quot(a2523, a2524, a2525)
        };
        let v6: u64 = {
            let a2533 = {
                let a2526 = v5;
                let a2527 = v2;
                nat_mul(a2526, a2527)?
            };
            let a2534 = {
                let a2531 = {
                    let a2528 = v4;
                    let a2529 = v1;
                    let a2530 = 0u64;
                    nat_rem(a2528, a2529, a2530)
                };
                let a2532 = v3;
                nat_mul(a2531, a2532)?
            };
            nat_add(a2533, a2534)?
        };
        let a2535 = v5;
        let a2536 = 0u64;
        if nat_eq(a2535, a2536) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2545 = {
                    let a2539 = {
                        let a2537 = v5;
                        let a2538 = 1u64;
                        nat_sub(a2537, a2538)
                    };
                    let a2540 = v2;
                    nat_mul(a2539, a2540)?
                };
                let a2546 = {
                    let a2543 = {
                        let a2541 = v1;
                        let a2542 = 1u64;
                        nat_sub(a2541, a2542)
                    };
                    let a2544 = v3;
                    nat_mul(a2543, a2544)?
                };
                nat_add(a2545, a2546)?
            };
            let a2547 = v6;
            let a2548 = v7;
            if nat_lt(a2547, a2548) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f116(v0: Adt3, v1: u64) -> bool {
    let a2549 = v1;
    let a2550 = 0u64;
    if !nat_eq(a2549, a2550) {
        true
    } else {
        let a2552 = match v0 {
            Adt3::C0(_, _, _, _, h2551, _, _, _) => {
                h2551
            }
        };
        let a2553 = 4u64;
        nat_eq(a2552, a2553)
    }
}

pub fn f117(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2554 = f128(v0, v1);
    match m2554 {
        None => {
            if v5 {
                let m2555 = f129(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2555 {
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
                let a2556 = 4u64;
                let a2557 = v28;
                nat_le(a2556, a2557)
            };
            let a2558 = v2;
            let a2559 = 0u64;
            if if if if if nat_eq(a2558, a2559) {
                let a2560 = v4;
                let a2561 = 0u64;
                nat_eq(a2560, a2561)
            } else {
                false
            } {
                let a2562 = v6;
                let a2563 = v14;
                if nat_eq(a2562, a2563) {
                    let a2564 = v7;
                    let a2565 = v15;
                    nat_eq(a2564, a2565)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2566 = v8;
                let a2567 = v16;
                if nat_eq(a2566, a2567) {
                    let a2568 = v9;
                    let a2569 = v17;
                    nat_eq(a2568, a2569)
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
                        let a2570 = v3;
                        let a2571 = 0u64;
                        nat_eq(a2570, a2571)
                    } else {
                        false
                    } {
                        f76(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2576 = {
                        let a2572 = v6;
                        let a2573 = v7;
                        nat_mul(a2572, a2573)?
                    };
                    let a2577 = {
                        let a2574 = v8;
                        let a2575 = v9;
                        nat_mul(a2574, a2575)?
                    };
                    nat_mul(a2576, a2577)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f118(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2598 = {
        let a2592 = {
            let a2586 = {
                let a2580 = v1;
                let a2581 = if v5 {
                    f113(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2578, _, _, _) => {
                            h2578
                        }
                    })
                } else {
                    f112(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2579, _, _, _) => {
                            h2579
                        }
                    })
                };
                nat_mul(a2580, a2581)?
            };
            let a2587 = {
                let a2584 = v2;
                let a2585 = if v5 {
                    f113(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2582, _, _) => {
                            h2582
                        }
                    })
                } else {
                    f112(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2583, _, _) => {
                            h2583
                        }
                    })
                };
                nat_mul(a2584, a2585)?
            };
            nat_add(a2586, a2587)?
        };
        let a2593 = {
            let a2590 = v3;
            let a2591 = if v5 {
                f113(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2588, _) => {
                        h2588
                    }
                })
            } else {
                f112(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2589, _) => {
                        h2589
                    }
                })
            };
            nat_mul(a2590, a2591)?
        };
        nat_add(a2592, a2593)?
    };
    let a2599 = {
        let a2596 = v4;
        let a2597 = if v5 {
            f113(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2594) => {
                    h2594
                }
            })
        } else {
            f112(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2595) => {
                    h2595
                }
            })
        };
        nat_mul(a2596, a2597)?
    };
    nat_add(a2598, a2599)
}

pub fn f119(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f120(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2600 = v1;
    let a2601 = 0u64;
    if nat_eq(a2600, a2601) {
        Ok(Adt5::C4({
            let a2602 = v3;
            let a2603 = 0u64;
            if nat_eq(a2602, a2603) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f45(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2604 = v3;
            let a2605 = 0u64;
            if nat_eq(a2604, a2605) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f121(v0: u64) -> bool {
    let a2606 = v0;
    let a2607 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2606, a2607) {
        true
    } else {
        let a2608 = v0;
        let a2609 = 1u64;
        nat_eq(a2608, a2609)
    } {
        true
    } else {
        let a2610 = v0;
        let a2611 = 2u64;
        nat_eq(a2610, a2611)
    } {
        true
    } else {
        let a2612 = v0;
        let a2613 = 3u64;
        nat_eq(a2612, a2613)
    } {
        true
    } else {
        let a2614 = v0;
        let a2615 = 6u64;
        nat_eq(a2614, a2615)
    } {
        true
    } else {
        let a2616 = v0;
        let a2617 = 7u64;
        nat_eq(a2616, a2617)
    } {
        true
    } else {
        let a2618 = v0;
        let a2619 = 8u64;
        nat_eq(a2618, a2619)
    } {
        true
    } else {
        let a2620 = v0;
        let a2621 = 10u64;
        nat_eq(a2620, a2621)
    } {
        true
    } else {
        let a2622 = v0;
        let a2623 = 11u64;
        nat_eq(a2622, a2623)
    } {
        true
    } else {
        let a2624 = v0;
        let a2625 = 12u64;
        nat_eq(a2624, a2625)
    } {
        true
    } else {
        let a2626 = v0;
        let a2627 = 13u64;
        nat_eq(a2626, a2627)
    } {
        true
    } else {
        let a2628 = v0;
        let a2629 = 14u64;
        nat_eq(a2628, a2629)
    } {
        true
    } else {
        let a2630 = v0;
        let a2631 = 16u64;
        nat_eq(a2630, a2631)
    } {
        true
    } else {
        let a2632 = v0;
        let a2633 = 17u64;
        nat_eq(a2632, a2633)
    } {
        true
    } else {
        let a2634 = v0;
        let a2635 = 18u64;
        nat_eq(a2634, a2635)
    } {
        true
    } else {
        let a2636 = v0;
        let a2637 = 19u64;
        nat_eq(a2636, a2637)
    } {
        true
    } else {
        let a2638 = v0;
        let a2639 = 20u64;
        nat_eq(a2638, a2639)
    } {
        true
    } else {
        let a2640 = v0;
        let a2641 = 21u64;
        nat_eq(a2640, a2641)
    } {
        true
    } else {
        let a2642 = v0;
        let a2643 = 22u64;
        nat_eq(a2642, a2643)
    } {
        true
    } else {
        let a2644 = v0;
        let a2645 = 23u64;
        nat_eq(a2644, a2645)
    } {
        true
    } else {
        let a2646 = v0;
        let a2647 = 26u64;
        nat_eq(a2646, a2647)
    } {
        true
    } else {
        let a2648 = v0;
        let a2649 = 29u64;
        nat_eq(a2648, a2649)
    } {
        true
    } else {
        let a2650 = v0;
        let a2651 = 30u64;
        nat_eq(a2650, a2651)
    } {
        true
    } else {
        let a2652 = v0;
        let a2653 = 34u64;
        nat_eq(a2652, a2653)
    } {
        true
    } else {
        let a2654 = v0;
        let a2655 = 35u64;
        nat_eq(a2654, a2655)
    } {
        true
    } else {
        let a2656 = v0;
        let a2657 = 39u64;
        nat_eq(a2656, a2657)
    } {
        true
    } else {
        let a2658 = v0;
        let a2659 = 40u64;
        nat_eq(a2658, a2659)
    } {
        true
    } else {
        let a2660 = v0;
        let a2661 = 41u64;
        nat_eq(a2660, a2661)
    } {
        true
    } else {
        let a2662 = v0;
        let a2663 = 42u64;
        nat_eq(a2662, a2663)
    } {
        true
    } else {
        let a2664 = v0;
        let a2665 = 43u64;
        nat_eq(a2664, a2665)
    }
}

pub fn f122(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2666 = v0;
    let a2667 = 0u64;
    if if if nat_eq(a2666, a2667) {
        true
    } else {
        let a2668 = v1;
        let a2669 = 0u64;
        nat_eq(a2668, a2669)
    } {
        true
    } else {
        let a2670 = v2;
        let a2671 = 0u64;
        nat_eq(a2670, a2671)
    } {
        Ok(0u64)
    } else {
        let a2674 = {
            let a2672 = v0;
            let a2673 = v1;
            nat_mul(a2672, a2673)?
        };
        let a2675 = v2;
        nat_mul(a2674, a2675)
    }
}

pub fn f123(v0: u64, v1: u64) -> Adt8 {
    let a2676 = v0;
    let a2677 = 0u64;
    if nat_eq(a2676, a2677) {
        Adt8::C0(0u64, v1)
    } else {
        let a2678 = v0;
        let a2679 = 1u64;
        if nat_eq(a2678, a2679) {
            Adt8::C1(v1)
        } else {
            let a2680 = v0;
            let a2681 = 2u64;
            if nat_eq(a2680, a2681) {
                Adt8::C2(v1)
            } else {
                let a2682 = v0;
                let a2683 = 3u64;
                if nat_eq(a2682, a2683) {
                    Adt8::C3(v1)
                } else {
                    let a2684 = v0;
                    let a2685 = 13u64;
                    if nat_lt(a2684, a2685) {
                        Adt8::C0({
                            let a2686 = v0;
                            let a2687 = 3u64;
                            nat_sub(a2686, a2687)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f124(v0: u64, v1: u64) -> Adt9 {
    let a2688 = v0;
    let a2689 = 1u64;
    if nat_eq(a2688, a2689) {
        Adt9::C0(v1)
    } else {
        let a2690 = v0;
        let a2691 = 3u64;
        if if nat_eq(a2690, a2691) {
            true
        } else {
            let a2692 = v0;
            let a2693 = 6u64;
            nat_eq(a2692, a2693)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f125(v0: u64) -> u64 {
    let m2694 = f75(v0);
    match m2694 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2696 = match v1.clone() {
                Adt11::C0(_, h2695, _) => {
                    h2695
                }
            };
            let a2697 = 1u64;
            if if nat_eq(a2696, a2697) {
                let a2699 = match v1.clone() {
                    Adt11::C0(_, _, h2698) => {
                        h2698
                    }
                };
                let a2700 = 1u64;
                if if if nat_eq(a2699, a2700) {
                    true
                } else {
                    let a2702 = match v1.clone() {
                        Adt11::C0(_, _, h2701) => {
                            h2701
                        }
                    };
                    let a2703 = 2u64;
                    nat_eq(a2702, a2703)
                } {
                    true
                } else {
                    let a2705 = match v1.clone() {
                        Adt11::C0(_, _, h2704) => {
                            h2704
                        }
                    };
                    let a2706 = 4u64;
                    nat_eq(a2705, a2706)
                } {
                    true
                } else {
                    let a2708 = match v1.clone() {
                        Adt11::C0(_, _, h2707) => {
                            h2707
                        }
                    };
                    let a2709 = 8u64;
                    nat_eq(a2708, a2709)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2710) => {
                        h2710
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f111(v1, v2, v3, v4) {
        f130(v0)
    } else {
        f121(v0)
    }
}

pub fn f127(v0: u64) -> u64 {
    let a2711 = v0;
    let a2712 = 26u64;
    if nat_eq(a2711, a2712) {
        26u64
    } else {
        0u64
    }
}

pub fn f128(v0: u64, v1: u64) -> Option<u64> {
    let a2713 = v0;
    let a2714 = 2u64;
    if nat_eq(a2713, a2714) {
        Some(0u64)
    } else {
        let a2715 = v0;
        let a2716 = 5u64;
        if nat_eq(a2715, a2716) {
            Some(1u64)
        } else {
            let a2717 = v0;
            let a2718 = 6u64;
            if nat_eq(a2717, a2718) {
                Some(2u64)
            } else {
                let a2719 = v0;
                let a2720 = 7u64;
                if nat_eq(a2719, a2720) {
                    Some(3u64)
                } else {
                    let a2721 = v0;
                    let a2722 = 9u64;
                    if nat_eq(a2721, a2722) {
                        Some(4u64)
                    } else {
                        let a2723 = v0;
                        let a2724 = 8u64;
                        if nat_eq(a2723, a2724) {
                            Some(5u64)
                        } else {
                            let a2725 = v0;
                            let a2726 = 89u64;
                            if nat_eq(a2725, a2726) {
                                let a2727 = v1;
                                let a2728 = 2u64;
                                if nat_eq(a2727, a2728) {
                                    Some(6u64)
                                } else {
                                    let a2729 = v1;
                                    let a2730 = 0u64;
                                    if nat_eq(a2729, a2730) {
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

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a2731 = v0;
    let a2732 = 28u64;
    if if if if nat_eq(a2731, a2732) {
        let a2733 = v1;
        let a2734 = 0u64;
        nat_eq(a2733, a2734)
    } else {
        false
    } {
        let a2735 = v2;
        let a2736 = 0u64;
        nat_eq(a2735, a2736)
    } else {
        false
    } {
        let a2737 = v3;
        let a2738 = 0u64;
        nat_eq(a2737, a2738)
    } else {
        false
    } {
        let a2739 = v4;
        let a2740 = v8;
        if if if if if if if if nat_eq(a2739, a2740) {
            let a2741 = v5;
            let a2742 = v12;
            nat_eq(a2741, a2742)
        } else {
            false
        } {
            let a2743 = v9;
            let a2744 = v13;
            nat_eq(a2743, a2744)
        } else {
            false
        } {
            let a2745 = v10;
            let a2746 = v14;
            nat_eq(a2745, a2746)
        } else {
            false
        } {
            let a2747 = v11;
            let a2748 = v15;
            nat_eq(a2747, a2748)
        } else {
            false
        } {
            let a2749 = 0u64;
            let a2750 = v6;
            nat_lt(a2749, a2750)
        } else {
            false
        } {
            let a2751 = 0u64;
            let a2752 = v7;
            nat_lt(a2751, a2752)
        } else {
            false
        } {
            let a2756 = {
                let a2753 = v10;
                let a2754 = v6;
                let a2755 = 1u64;
                nat_rem(a2753, a2754, a2755)
            };
            let a2757 = 0u64;
            if nat_eq(a2756, a2757) {
                let a2761 = {
                    let a2758 = v11;
                    let a2759 = v7;
                    let a2760 = 1u64;
                    nat_rem(a2758, a2759, a2760)
                };
                let a2762 = 0u64;
                nat_eq(a2761, a2762)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a2763 = v10;
                let a2764 = v6;
                let a2765 = 0u64;
                nat_quot(a2763, a2764, a2765)
            }, {
                let a2766 = v11;
                let a2767 = v7;
                let a2768 = 0u64;
                nat_quot(a2766, a2767, a2768)
            }, {
                let a2769 = v5;
                let a2770 = v4;
                nat_mul(a2769, a2770)?
            }, {
                let a2771 = v4;
                let a2772 = v9;
                nat_mul(a2771, a2772)?
            }, {
                let a2773 = v5;
                let a2774 = v9;
                nat_mul(a2773, a2774)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f130(v0: u64) -> bool {
    let m2775 = f75(v0);
    match m2775 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}
