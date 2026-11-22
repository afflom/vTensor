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
    C25(u64, u64, u64),
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
                        let a6 = 78u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt5::C0(f2(v2, v14, v4, v5, v13)?))
                        } else {
                            let a7 = v1;
                            let a8 = 27u64;
                            if nat_eq(a7, a8) {
                                Ok(Adt5::C0(f3(v2, v14, v13)?))
                            } else {
                                let a9 = v1;
                                let a10 = 3u64;
                                if nat_eq(a9, a10) {
                                    Ok(Adt5::C0(f4(v14, v4, v5, v13)?))
                                } else {
                                    f5(v1, v2, v14, v4, v13)
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v15, v16, v17, v18) => {
            f6(v15, v16, v17, v18)
        }
        Adt0::C2(v19, v20, v21, v22) => {
            f7(v19, v20, v21, v22)
        }
        Adt0::C3(v23, v24, v25, v26, v27) => {
            f8(v23, v24, v25, v26, v27)
        }
        Adt0::C4(v28, v29, v30, v31, v32, v33) => {
            f9(v28, v29, v30, v31, v32, v33)
        }
        Adt0::C5(v34, v35, v36, v37, v38, v39) => {
            f10(v34, v35, v36, v37, v38, v39)
        }
        Adt0::C6(v40, v41, v42, v43, v44, v45) => {
            f11(v40, v41, v42, v43, v44, v45)
        }
        Adt0::C7(v46, v47, v48, v49, v50) => {
            f12(v46, v47, v48, v49, v50)
        }
        Adt0::C8(v51) => {
            f13(v51)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
    let v2: u64 = f14(v1.clone(), 1u64)?;
    let a12 = match v1.clone() {
        Adt2::C0(h11, _, _, _, _, _, _, _, _, _, _) => {
            h11
        }
    };
    let a13 = 0u64;
    if if if if if if if if nat_eq(a12, a13) {
        let a15 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h14, _, _, _, _, _) => {
                h14
            }
        };
        let a16 = 4u64;
        nat_eq(a15, a16)
    } else {
        false
    } {
        f15(match v0.clone() {
            Adt1::C0(h17, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h17
            }
        })
    } else {
        false
    } {
        f15(match v0.clone() {
            Adt1::C0(_, h18, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h18
            }
        })
    } else {
        false
    } {
        let a20 = 0u64;
        let a21 = match v0.clone() {
            Adt1::C0(_, _, h19, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h19
            }
        };
        nat_lt(a20, a21)
    } else {
        false
    } {
        let a23 = match v0.clone() {
            Adt1::C0(_, _, h22, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h22
            }
        };
        let a24 = 2139095040u64;
        nat_lt(a23, a24)
    } else {
        false
    } {
        let a25 = v2;
        let a26 = 9223372036854775808u64;
        nat_lt(a25, a26)
    } else {
        false
    } {
        let a28 = f16(v2)?;
        let a29 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h27, _) => {
                h27
            }
        };
        nat_le(a28, a29)
    } else {
        false
    } {
        Ok(Adt6::C4(f17(v2)?, 4u64, f18(v1.clone(), 1u64), f18(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h30, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h30
            }
        }, match v0.clone() {
            Adt1::C0(_, h31, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h31
            }
        }, match v0 {
            Adt1::C0(_, _, h32, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h32
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m33 = v2;
    match m33 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let m34 = v3;
            match m34 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v6) => {
                    let a36 = match v1.clone() {
                        Adt2::C0(h35, _, _, _, _, _, _, _, _, _, _) => {
                            h35
                        }
                    };
                    let a37 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a36, a37) {
                        let a39 = match v5.clone() {
                            Adt2::C0(h38, _, _, _, _, _, _, _, _, _, _) => {
                                h38
                            }
                        };
                        let a40 = 0u64;
                        nat_eq(a39, a40)
                    } else {
                        false
                    } {
                        let a42 = match v6.clone() {
                            Adt2::C0(h41, _, _, _, _, _, _, _, _, _, _) => {
                                h41
                            }
                        };
                        let a43 = 0u64;
                        nat_eq(a42, a43)
                    } else {
                        false
                    } {
                        let a45 = match v4.clone() {
                            Adt2::C0(h44, _, _, _, _, _, _, _, _, _, _) => {
                                h44
                            }
                        };
                        let a46 = 0u64;
                        nat_eq(a45, a46)
                    } else {
                        false
                    } {
                        f19(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f19(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f20(v1.clone())?
                    } else {
                        false
                    } {
                        f20(v5.clone())?
                    } else {
                        false
                    } {
                        f20(v6)?
                    } else {
                        false
                    } {
                        f20(v4.clone())?
                    } else {
                        false
                    } {
                        let a52 = match v1.clone() {
                            Adt2::C0(_, h47, _, _, _, _, _, _, _, _, _) => {
                                h47
                            }
                        };
                        let a53 = {
                            let a50 = match v5.clone() {
                                Adt2::C0(_, h48, _, _, _, _, _, _, _, _, _) => {
                                    h48
                                }
                            };
                            let a51 = match v5.clone() {
                                Adt2::C0(_, h49, _, _, _, _, _, _, _, _, _) => {
                                    h49
                                }
                            };
                            nat_mul(a50, a51)?
                        };
                        nat_eq(a52, a53)
                    } else {
                        false
                    } {
                        let a59 = match v1.clone() {
                            Adt2::C0(_, _, h54, _, _, _, _, _, _, _, _) => {
                                h54
                            }
                        };
                        let a60 = {
                            let a57 = match v5.clone() {
                                Adt2::C0(_, _, h55, _, _, _, _, _, _, _, _) => {
                                    h55
                                }
                            };
                            let a58 = match v5.clone() {
                                Adt2::C0(_, _, _, h56, _, _, _, _, _, _, _) => {
                                    h56
                                }
                            };
                            nat_mul(a57, a58)?
                        };
                        nat_eq(a59, a60)
                    } else {
                        false
                    } {
                        let a63 = match v1.clone() {
                            Adt2::C0(_, _, _, h61, _, _, _, _, _, _, _) => {
                                h61
                            }
                        };
                        let a64 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h62, _, _, _, _, _, _) => {
                                h62
                            }
                        };
                        nat_eq(a63, a64)
                    } else {
                        false
                    } {
                        let a66 = match v0.clone() {
                            Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h65
                            }
                        };
                        let a67 = 4294967296u64;
                        nat_lt(a66, a67)
                    } else {
                        false
                    } {
                        let a69 = match v0.clone() {
                            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h68
                            }
                        };
                        let a70 = 0u64;
                        if nat_eq(a69, a70) {
                            if f21(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a73 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h71) => {
                                        h71
                                    }
                                };
                                let a74 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h72) => {
                                        h72
                                    }
                                };
                                nat_eq(a73, a74)
                            }
                        } else {
                            let a77 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h75) => {
                                    h75
                                }
                            };
                            let a78 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h76) => {
                                    h76
                                }
                            };
                            nat_eq(a77, a78)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a80 = match v4.clone() {
                                Adt2::C0(_, h79, _, _, _, _, _, _, _, _, _) => {
                                    h79
                                }
                            };
                            let a81 = 0u64;
                            if if if nat_eq(a80, a81) {
                                true
                            } else {
                                let a83 = match v4.clone() {
                                    Adt2::C0(_, _, h82, _, _, _, _, _, _, _, _) => {
                                        h82
                                    }
                                };
                                let a84 = 0u64;
                                nat_eq(a83, a84)
                            } {
                                true
                            } else {
                                let a86 = match v4.clone() {
                                    Adt2::C0(_, _, _, h85, _, _, _, _, _, _, _) => {
                                        h85
                                    }
                                };
                                let a87 = 0u64;
                                nat_eq(a86, a87)
                            } {
                                true
                            } else {
                                let a89 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h88, _, _, _, _, _, _) => {
                                        h88
                                    }
                                };
                                let a90 = 0u64;
                                nat_eq(a89, a90)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h91, _, _, _, _, _, _, _, _, _) => {
                                h91
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a93 = match v0 {
                                Adt1::C0(h92, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h92
                                }
                            };
                            let a94 = 0u64;
                            if nat_eq(a93, a94) {
                                f14(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f22(v8, match v5.clone() {
                                Adt2::C0(_, _, h95, _, _, _, _, _, _, _, _) => {
                                    h95
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h96, _, _, _, _, _, _, _) => {
                                    h96
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h97, _, _, _, _, _, _) => {
                                    h97
                                }
                            })?
                        };
                        Ok(Adt6::C4(f23(v9, v8, v10)?, 4u64, f18(v1, 1u64), f18(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a99 = match v1.clone() {
        Adt2::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
            h98
        }
    };
    let a100 = 0u64;
    if if if if if nat_eq(a99, a100) {
        let a102 = match v2.clone() {
            Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                h101
            }
        };
        let a103 = 0u64;
        nat_eq(a102, a103)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f24(match v0.clone() {
            Adt1::C0(h104, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h104
            }
        })
    } else {
        false
    } {
        if f21(v1.clone(), v2.clone())? {
            true
        } else {
            f25(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f18(v1, 1u64);
        let v4: u64 = {
            let a110 = match v3.clone() {
                Adt3::C0(_, h105, _, _, _, _, _, _) => {
                    h105
                }
            };
            let a111 = {
                let a108 = match v3.clone() {
                    Adt3::C0(_, _, h106, _, _, _, _, _) => {
                        h106
                    }
                };
                let a109 = match v3.clone() {
                    Adt3::C0(_, _, _, h107, _, _, _, _) => {
                        h107
                    }
                };
                nat_mul(a108, a109)?
            };
            nat_mul(a110, a111)?
        };
        Ok(Adt6::C4(f26(match v3.clone() {
            Adt3::C0(h112, _, _, _, _, _, _, _) => {
                h112
            }
        }, v4)?, 4u64, v3, f18(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h113, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h113
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f4(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m114 = v1;
    match m114 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m115 = v2;
            match m115 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a117 = match v0.clone() {
                        Adt2::C0(h116, _, _, _, _, _, _, _, _, _, _) => {
                            h116
                        }
                    };
                    let a118 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a117, a118) {
                        let a120 = match v4.clone() {
                            Adt2::C0(h119, _, _, _, _, _, _, _, _, _, _) => {
                                h119
                            }
                        };
                        let a121 = 0u64;
                        nat_eq(a120, a121)
                    } else {
                        false
                    } {
                        let a123 = match v5.clone() {
                            Adt2::C0(h122, _, _, _, _, _, _, _, _, _, _) => {
                                h122
                            }
                        };
                        let a124 = 26u64;
                        nat_eq(a123, a124)
                    } else {
                        false
                    } {
                        let a126 = match v3.clone() {
                            Adt2::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
                                h125
                            }
                        };
                        let a127 = 0u64;
                        nat_eq(a126, a127)
                    } else {
                        false
                    } {
                        f19(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a130 = match v0.clone() {
                            Adt2::C0(_, h128, _, _, _, _, _, _, _, _, _) => {
                                h128
                            }
                        };
                        let a131 = match v4.clone() {
                            Adt2::C0(_, h129, _, _, _, _, _, _, _, _, _) => {
                                h129
                            }
                        };
                        nat_eq(a130, a131)
                    } else {
                        false
                    } {
                        let a134 = match v0.clone() {
                            Adt2::C0(_, _, h132, _, _, _, _, _, _, _, _) => {
                                h132
                            }
                        };
                        let a135 = match v5.clone() {
                            Adt2::C0(_, h133, _, _, _, _, _, _, _, _, _) => {
                                h133
                            }
                        };
                        nat_eq(a134, a135)
                    } else {
                        false
                    } {
                        let a138 = match v0.clone() {
                            Adt2::C0(_, _, _, h136, _, _, _, _, _, _, _) => {
                                h136
                            }
                        };
                        let a139 = match v5.clone() {
                            Adt2::C0(_, _, h137, _, _, _, _, _, _, _, _) => {
                                h137
                            }
                        };
                        nat_eq(a138, a139)
                    } else {
                        false
                    } {
                        let a141 = 0u64;
                        let a142 = match v5.clone() {
                            Adt2::C0(_, _, _, h140, _, _, _, _, _, _, _) => {
                                h140
                            }
                        };
                        nat_lt(a141, a142)
                    } else {
                        false
                    } {
                        let a144 = 0u64;
                        let a145 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h143, _, _, _, _, _, _) => {
                                h143
                            }
                        };
                        nat_lt(a144, a145)
                    } else {
                        false
                    } {
                        let a147 = 4u64;
                        let a148 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h146, _, _, _, _, _) => {
                                h146
                            }
                        };
                        nat_le(a147, a148)
                    } else {
                        false
                    } {
                        let a150 = 4u64;
                        let a151 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h149, _, _, _, _, _) => {
                                h149
                            }
                        };
                        nat_le(a150, a151)
                    } else {
                        false
                    } {
                        let a153 = 4u64;
                        let a154 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h152, _, _, _, _, _) => {
                                h152
                            }
                        };
                        nat_le(a153, a154)
                    } else {
                        false
                    } {
                        let a156 = 4u64;
                        let a157 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h155, _, _, _, _, _) => {
                                h155
                            }
                        };
                        nat_le(a156, a157)
                    } else {
                        false
                    } {
                        f21(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f21(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f21(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f27(match v0.clone() {
                            Adt2::C0(_, h158, _, _, _, _, _, _, _, _, _) => {
                                h158
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h159, _, _, _, _, _, _, _, _) => {
                                h159
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h160, _, _, _, _, _, _, _) => {
                                h160
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h161, _, _, _, _, _, _) => {
                                h161
                            }
                        })?, 4u64, f18(v0, 1u64), f18(v3, 1u64), Adt4::C22(f18(v4, 1u64), f18(v5, 1u64))))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt5> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h162, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h162
        }
    };
    Ok(Adt5::C0({
        let a163 = v0;
        let a164 = 39u64;
        if nat_eq(a163, a164) {
            f28(v2, v3, v4)?
        } else {
            let a165 = v0;
            let a166 = 77u64;
            if nat_eq(a165, a166) {
                f29(v2, v4)?
            } else {
                let a167 = v0;
                let a168 = 40u64;
                if nat_eq(a167, a168) {
                    f30(v2, v3, v4)?
                } else {
                    if f31(v0) {
                        f32(v0, v2, {
                            let m169 = v3;
                            match m169 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a170 = v0;
                        let a171 = 19u64;
                        if nat_eq(a170, a171) {
                            f33(v2, v4)?
                        } else {
                            let a172 = v0;
                            let a173 = 21u64;
                            if nat_eq(a172, a173) {
                                f34(v5, v2, v3, v4)?
                            } else {
                                let a174 = v0;
                                let a175 = 61u64;
                                if if if nat_eq(a174, a175) {
                                    true
                                } else {
                                    let a176 = v0;
                                    let a177 = 62u64;
                                    nat_eq(a176, a177)
                                } {
                                    true
                                } else {
                                    let a178 = v0;
                                    let a179 = 63u64;
                                    nat_eq(a178, a179)
                                } {
                                    f35(v0, v1, v2, v4)?
                                } else {
                                    let a180 = v0;
                                    let a181 = 4u64;
                                    if if nat_eq(a180, a181) {
                                        true
                                    } else {
                                        let a182 = v0;
                                        let a183 = 32u64;
                                        nat_eq(a182, a183)
                                    } {
                                        f36(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a184 = v0;
                                        let a185 = 20u64;
                                        if nat_eq(a184, a185) {
                                            f37(v2, v4)?
                                        } else {
                                            let a186 = v0;
                                            let a187 = 42u64;
                                            if nat_eq(a186, a187) {
                                                f38(4u64, v2, v4)?
                                            } else {
                                                let a188 = v0;
                                                let a189 = 69u64;
                                                if nat_eq(a188, a189) {
                                                    let a190 = v5;
                                                    let a191 = 4u64;
                                                    if nat_lt(a190, a191) {
                                                        f38(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a192 = v0;
                                                    let a193 = 70u64;
                                                    if nat_eq(a192, a193) {
                                                        f39(v5, v2, v4)?
                                                    } else {
                                                        let a194 = v0;
                                                        let a195 = 75u64;
                                                        if if nat_eq(a194, a195) {
                                                            true
                                                        } else {
                                                            let a196 = v0;
                                                            let a197 = 76u64;
                                                            nat_eq(a196, a197)
                                                        } {
                                                            f40({
                                                                let a198 = v0;
                                                                let a199 = 76u64;
                                                                nat_eq(a198, a199)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a200 = v0;
                                                            let a201 = 31u64;
                                                            if if if nat_eq(a200, a201) {
                                                                true
                                                            } else {
                                                                let a202 = v0;
                                                                let a203 = 47u64;
                                                                nat_eq(a202, a203)
                                                            } {
                                                                true
                                                            } else {
                                                                let a204 = v0;
                                                                let a205 = 68u64;
                                                                nat_eq(a204, a205)
                                                            } {
                                                                f41(v0, v1, v2, v4)?
                                                            } else {
                                                                let a206 = v0;
                                                                let a207 = 15u64;
                                                                if nat_eq(a206, a207) {
                                                                    f42(v2, v4)?
                                                                } else {
                                                                    let a208 = v0;
                                                                    let a209 = 17u64;
                                                                    if nat_eq(a208, a209) {
                                                                        f43(v2, v4)?
                                                                    } else {
                                                                        let a210 = v0;
                                                                        let a211 = 18u64;
                                                                        if nat_eq(a210, a211) {
                                                                            f44(v2, v3, v4)?
                                                                        } else {
                                                                            let a212 = v0;
                                                                            let a213 = 13u64;
                                                                            if if if nat_eq(a212, a213) {
                                                                                true
                                                                            } else {
                                                                                let a214 = v0;
                                                                                let a215 = 14u64;
                                                                                nat_eq(a214, a215)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a216 = v0;
                                                                                let a217 = 16u64;
                                                                                nat_eq(a216, a217)
                                                                            } {
                                                                                f45(v0, v2, v4)?
                                                                            } else {
                                                                                let m218 = f46(v0, v5);
                                                                                match m218 {
                                                                                    None => {
                                                                                        f47(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f48(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f6(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m219 = v3;
    match m219 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f51(v0, v1, v2.clone())?, f49(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f52(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f53(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f54(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f53(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f55(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f56(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f57(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f49(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f58(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f59(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C15 => {
            f60(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f61(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f62(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f49(f50(v0, match v2.clone() {
                Adt3::C0(h220, _, _, _, _, _, _, _) => {
                    h220
                }
            }, match v2.clone() {
                Adt3::C0(_, h221, _, _, _, _, _, _) => {
                    h221
                }
            })?, v1)?, f49(v0, v2)?))
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
        Adt4::C25(v48, v49, v50) => {
            f63(v0, v48, v49, v50)
        }
    }
}

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f64(match v1.clone() {
        Adt3::C0(h222, _, _, _, _, _, _, _) => {
            h222
        }
    })?;
    let v5: u64 = {
        let a223 = v0;
        let a224 = v4;
        let a225 = 0u64;
        nat_quot(a223, a224, a225)
    };
    let v6: u64 = {
        let a226 = v0;
        let a227 = v4;
        let a228 = 0u64;
        nat_rem(a226, a227, a228)
    };
    let a229 = v6;
    let a230 = 0u64;
    if nat_eq(a229, a230) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a231 = v6;
        let a232 = 1u64;
        if nat_eq(a231, a232) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a235 = {
                let a233 = v6;
                let a234 = 1u64;
                nat_add(a233, a234)?
            };
            let a236 = v4;
            if nat_eq(a235, a236) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f49(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a237 = v6;
                    let a238 = 2u64;
                    nat_sub(a237, a238)
                };
                let v8: u64 = {
                    let a239 = v7;
                    let a240 = 3u64;
                    let a241 = 0u64;
                    nat_quot(a239, a240, a241)
                };
                let v9: u64 = {
                    let a242 = v7;
                    let a243 = 3u64;
                    let a244 = 0u64;
                    nat_rem(a242, a243, a244)
                };
                let v10: u64 = f49({
                    let a248 = {
                        let a246 = v5;
                        let a247 = match v1.clone() {
                            Adt3::C0(h245, _, _, _, _, _, _, _) => {
                                h245
                            }
                        };
                        nat_mul(a246, a247)?
                    };
                    let a249 = v8;
                    nat_add(a248, a249)?
                }, v1)?;
                let a250 = v9;
                let a251 = 0u64;
                if nat_eq(a250, a251) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a252 = v9;
                    let a253 = 1u64;
                    if nat_eq(a252, a253) {
                        let a254 = v3;
                        let a255 = 2u64;
                        if nat_eq(a254, a255) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a256 = v3;
                        let a257 = 2u64;
                        if if nat_eq(a256, a257) {
                            true
                        } else {
                            let a258 = v3;
                            let a259 = 3u64;
                            nat_eq(a258, a259)
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

pub fn f8(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a261 = 2u64;
        let a262 = match v1.clone() {
            Adt3::C0(_, h260, _, _, _, _, _, _) => {
                h260
            }
        };
        nat_mul(a261, a262)?
    };
    let a263 = v0;
    let a264 = v5;
    if nat_lt(a263, a264) {
        let a268 = {
            let a265 = v0;
            let a266 = 2u64;
            let a267 = 0u64;
            nat_rem(a265, a266, a267)
        };
        let a269 = 0u64;
        if nat_eq(a268, a269) {
            Ok(Adt5::C4(19u64, 4u64, f49({
                let a270 = v0;
                let a271 = 2u64;
                let a272 = 0u64;
                nat_quot(a270, a271, a272)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a273 = v4;
            let a274 = 2147483648u64;
            if if nat_lt(a273, a274) {
                let a276 = v4;
                let a277 = match v2 {
                    Adt3::C0(_, h275, _, _, _, _, _, _) => {
                        h275
                    }
                };
                nat_lt(a276, a277)
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
            let a280 = match v2.clone() {
                Adt3::C0(h278, _, _, _, _, _, _, _) => {
                    h278
                }
            };
            let a281 = match v2.clone() {
                Adt3::C0(_, h279, _, _, _, _, _, _) => {
                    h279
                }
            };
            nat_mul(a280, a281)?
        };
        let a284 = {
            let a282 = v0;
            let a283 = v5;
            nat_sub(a282, a283)
        };
        let a285 = v6;
        if nat_lt(a284, a285) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f49({
                let a286 = v0;
                let a287 = v5;
                nat_sub(a286, a287)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a290 = {
                    let a288 = v0;
                    let a289 = v5;
                    nat_sub(a288, a289)
                };
                let a291 = v6;
                nat_sub(a290, a291)
            };
            let v8: u64 = {
                let a295 = 1u64;
                let a296 = {
                    let a293 = 2u64;
                    let a294 = match v1.clone() {
                        Adt3::C0(h292, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    nat_mul(a293, a294)?
                };
                nat_add(a295, a296)?
            };
            let v9: u64 = {
                let a297 = v7;
                let a298 = v8;
                let a299 = 0u64;
                nat_quot(a297, a298, a299)
            };
            let v10: u64 = {
                let a300 = v7;
                let a301 = v8;
                let a302 = 0u64;
                nat_rem(a300, a301, a302)
            };
            let a303 = v10;
            let a304 = 0u64;
            if nat_eq(a303, a304) {
                Ok(Adt5::C4(19u64, 4u64, f49(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a308 = {
                    let a305 = v10;
                    let a306 = 2u64;
                    let a307 = 0u64;
                    nat_rem(a305, a306, a307)
                };
                let a309 = 1u64;
                if nat_eq(a308, a309) {
                    Ok(Adt5::C4(6u64, 0u64, f49({
                        let a318 = {
                            let a311 = v9;
                            let a312 = match v1.clone() {
                                Adt3::C0(h310, _, _, _, _, _, _, _) => {
                                    h310
                                }
                            };
                            nat_mul(a311, a312)?
                        };
                        let a319 = {
                            let a315 = {
                                let a313 = v10;
                                let a314 = 1u64;
                                nat_sub(a313, a314)
                            };
                            let a316 = 2u64;
                            let a317 = 0u64;
                            nat_quot(a315, a316, a317)
                        };
                        nat_add(a318, a319)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f49({
                        let a328 = {
                            let a321 = v4;
                            let a322 = match v1.clone() {
                                Adt3::C0(h320, _, _, _, _, _, _, _) => {
                                    h320
                                }
                            };
                            nat_mul(a321, a322)?
                        };
                        let a329 = {
                            let a325 = {
                                let a323 = v10;
                                let a324 = 1u64;
                                nat_sub(a323, a324)
                            };
                            let a326 = 2u64;
                            let a327 = 0u64;
                            nat_quot(a325, a326, a327)
                        };
                        nat_add(a328, a329)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f49({
                        let a338 = {
                            let a331 = v4;
                            let a332 = match v1 {
                                Adt3::C0(h330, _, _, _, _, _, _, _) => {
                                    h330
                                }
                            };
                            nat_mul(a331, a332)?
                        };
                        let a339 = {
                            let a335 = {
                                let a333 = v10;
                                let a334 = 1u64;
                                nat_sub(a333, a334)
                            };
                            let a336 = 2u64;
                            let a337 = 0u64;
                            nat_quot(a335, a336, a337)
                        };
                        nat_add(a338, a339)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a340 = v1;
    let a341 = f65(v2.clone(), v4.clone())?;
    if nat_lt(a340, a341) {
        let a344 = v1;
        let a345 = {
            let a342 = 2u64;
            let a343 = f66(v4.clone())?;
            nat_mul(a342, a343)?
        };
        if nat_lt(a344, a345) {
            let a349 = {
                let a346 = v1;
                let a347 = 2u64;
                let a348 = 0u64;
                nat_rem(a346, a347, a348)
            };
            let a350 = 0u64;
            if nat_eq(a349, a350) {
                f67({
                    let a351 = v1;
                    let a352 = 2u64;
                    let a353 = 0u64;
                    nat_quot(a351, a352, a353)
                }, v4)
            } else {
                if f68(match v2 {
                    Adt3::C0(_, h354, _, _, _, _, _, _) => {
                        h354
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a357 = v1;
                let a358 = {
                    let a355 = 2u64;
                    let a356 = f66(v4.clone())?;
                    nat_mul(a355, a356)?
                };
                nat_sub(a357, a358)
            };
            let v7: u64 = {
                let a362 = v6;
                let a363 = {
                    let a360 = 1u64;
                    let a361 = match v2.clone() {
                        Adt3::C0(h359, _, _, _, _, _, _, _) => {
                            h359
                        }
                    };
                    nat_add(a360, a361)?
                };
                let a364 = 0u64;
                nat_quot(a362, a363, a364)
            };
            let v8: u64 = {
                let a368 = v6;
                let a369 = {
                    let a366 = 1u64;
                    let a367 = match v2.clone() {
                        Adt3::C0(h365, _, _, _, _, _, _, _) => {
                            h365
                        }
                    };
                    nat_add(a366, a367)?
                };
                let a370 = 0u64;
                nat_rem(a368, a369, a370)
            };
            let a371 = v8;
            let a372 = 0u64;
            if nat_eq(a371, a372) {
                f67(v7, v4)
            } else {
                f69(v0, v7, {
                    let a373 = v8;
                    let a374 = 1u64;
                    nat_sub(a373, a374)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f10(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a377 = match v4.clone() {
            Adt3::C0(h375, _, _, _, _, _, _, _) => {
                h375
            }
        };
        let a378 = match v4.clone() {
            Adt3::C0(_, h376, _, _, _, _, _, _) => {
                h376
            }
        };
        nat_mul(a377, a378)?
    };
    let v7: u64 = {
        let a379 = 2u64;
        let a380 = v6;
        nat_mul(a379, a380)?
    };
    let v8: u64 = f70(match v3.clone() {
        Adt3::C0(_, h381, _, _, _, _, _, _) => {
            h381
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h382, _, _, _, _, _) => {
            h382
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h383, _, _, _, _) => {
            h383
        }
    });
    let a384 = v0;
    let a385 = v7;
    if nat_lt(a384, a385) {
        let a389 = {
            let a386 = v0;
            let a387 = 2u64;
            let a388 = 0u64;
            nat_rem(a386, a387, a388)
        };
        let a390 = 0u64;
        if nat_eq(a389, a390) {
            Ok(Adt5::C4(19u64, 5u64, f49({
                let a391 = v0;
                let a392 = 2u64;
                let a393 = 0u64;
                nat_quot(a391, a392, a393)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f71(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a394 = v0;
            let a395 = v7;
            nat_sub(a394, a395)
        };
        let v10: u64 = {
            let a399 = v9;
            let a400 = {
                let a397 = 1u64;
                let a398 = match v1.clone() {
                    Adt3::C0(h396, _, _, _, _, _, _, _) => {
                        h396
                    }
                };
                nat_add(a397, a398)?
            };
            let a401 = 0u64;
            nat_quot(a399, a400, a401)
        };
        let v11: u64 = {
            let a405 = v9;
            let a406 = {
                let a403 = 1u64;
                let a404 = match v1.clone() {
                    Adt3::C0(h402, _, _, _, _, _, _, _) => {
                        h402
                    }
                };
                nat_add(a403, a404)?
            };
            let a407 = 0u64;
            nat_rem(a405, a406, a407)
        };
        let a408 = v11;
        let a409 = 0u64;
        if nat_eq(a408, a409) {
            Ok(Adt5::C4(19u64, 5u64, f49({
                let a410 = v10;
                let a411 = v6;
                let a412 = 0u64;
                nat_rem(a410, a411, a412)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f71(v5, v8) {
                let v12: u64 = {
                    let a418 = {
                        let a414 = v10;
                        let a415 = match v1.clone() {
                            Adt3::C0(h413, _, _, _, _, _, _, _) => {
                                h413
                            }
                        };
                        nat_mul(a414, a415)?
                    };
                    let a419 = {
                        let a416 = v11;
                        let a417 = 1u64;
                        nat_sub(a416, a417)
                    };
                    nat_add(a418, a419)?
                };
                let a420 = v5;
                let a421 = 4294967295u64;
                if nat_eq(a420, a421) {
                    Ok(Adt5::C1(false, f49(v12, v1)?, f49(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f49(v12, v1)?, 4u64, f49({
                        let a427 = {
                            let a423 = v5;
                            let a424 = match v3.clone() {
                                Adt3::C0(h422, _, _, _, _, _, _, _) => {
                                    h422
                                }
                            };
                            nat_mul(a423, a424)?
                        };
                        let a428 = {
                            let a425 = v11;
                            let a426 = 1u64;
                            nat_sub(a425, a426)
                        };
                        nat_add(a427, a428)?
                    }, v3)?, 3u64, f49(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a429 = v0;
    let a430 = f17(v1)?;
    if nat_lt(a429, a430) {
        let a431 = v0;
        let a432 = 4u64;
        if nat_lt(a431, a432) {
            let a433 = v0;
            let a434 = 0u64;
            if nat_eq(a433, a434) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a435 = v0;
                let a436 = 1u64;
                if nat_eq(a435, a436) {
                    let a437 = v5;
                    let a438 = 0u64;
                    if nat_eq(a437, a438) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a439 = v0;
                    let a440 = 2u64;
                    if nat_eq(a439, a440) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a441 = v0;
            let a442 = 4u64;
            if if nat_eq(a441, a442) {
                let a443 = f72(v5)?;
                let a444 = v1;
                !nat_eq(a443, a444)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a447 = {
                        let a445 = v0;
                        let a446 = 4u64;
                        nat_sub(a445, a446)
                    };
                    let a448 = 3u64;
                    let a449 = 0u64;
                    nat_quot(a447, a448, a449)
                };
                let v7: u64 = {
                    let a452 = {
                        let a450 = v0;
                        let a451 = 4u64;
                        nat_sub(a450, a451)
                    };
                    let a453 = 3u64;
                    let a454 = 0u64;
                    nat_rem(a452, a453, a454)
                };
                let a455 = v7;
                let a456 = 0u64;
                if nat_eq(a455, a456) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a457 = v7;
                    let a458 = 1u64;
                    if nat_eq(a457, a458) {
                        Ok(Adt5::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt5::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f16(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f12(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt5> {
    Ok(f73(v3.clone(), {
        let m459 = v3.clone();
        match m459 {
            Adt4::C0 => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f7(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f8(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f9(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f6(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f10(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v53, v54, v55, v56) => {
                f11(v0, v56, v53, v54, v55, v4)?
            }
            Adt4::C24(v52) => {
                f74(v0, v1, v2, v52, v4)?
            }
            Adt4::C25(_, _, _) => {
                f6(v0, v1, v2, v3)?
            }
        }
    }))
}

pub fn f13(v0: Adt4) -> R<Adt5> {
    let m460 = v0;
    match m460 {
        Adt4::C0 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C1 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C2(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C3(_, _, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C4(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C5(_, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C6(_, _, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C7(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C8(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C9(v18, v19) => {
            Ok(Adt5::C6(0u64, v18, v19, true))
        }
        Adt4::C10(_, v21, _) => {
            Ok(Adt5::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false))
        }
        Adt4::C11(_, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C12(_, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C13(_, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C14(_, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C15 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C16(_, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C17 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C18(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C21 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C6(7u64, 0u64, 0u64, false))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C6(0u64, 0u64, 0u64, false))
        }
        Adt4::C24(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C25(v45, _, v47) => {
            Ok(Adt5::C6({
                let a465 = {
                    let a461 = 0u64;
                    let a462 = v45;
                    if nat_lt(a461, a462) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a466 = {
                    let a463 = 0u64;
                    let a464 = v47;
                    if nat_lt(a463, a464) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a465, a466)?
            }, 0u64, 0u64, false))
        }
    }
}

pub fn f14(v0: Adt2, v1: u64) -> R<u64> {
    let a478 = {
        let a472 = {
            let a468 = match v0.clone() {
                Adt2::C0(_, h467, _, _, _, _, _, _, _, _, _) => {
                    h467
                }
            };
            let a469 = v1;
            let a470 = 0u64;
            nat_quot(a468, a469, a470)
        };
        let a473 = match v0.clone() {
            Adt2::C0(_, _, h471, _, _, _, _, _, _, _, _) => {
                h471
            }
        };
        nat_mul(a472, a473)?
    };
    let a479 = {
        let a476 = match v0.clone() {
            Adt2::C0(_, _, _, h474, _, _, _, _, _, _, _) => {
                h474
            }
        };
        let a477 = match v0 {
            Adt2::C0(_, _, _, _, h475, _, _, _, _, _, _) => {
                h475
            }
        };
        nat_mul(a476, a477)?
    };
    nat_mul(a478, a479)
}

pub fn f15(v0: u64) -> bool {
    let a480 = v0;
    let a481 = 4294967296u64;
    if nat_lt(a480, a481) {
        let a485 = {
            let a482 = v0;
            let a483 = 2147483648u64;
            let a484 = 0u64;
            nat_rem(a482, a483, a484)
        };
        let a486 = 2139095040u64;
        nat_lt(a485, a486)
    } else {
        false
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let a487 = 4u64;
    let a488 = v0;
    nat_mul(a487, a488)
}

pub fn f17(v0: u64) -> R<u64> {
    let a489 = v0;
    let a490 = 0u64;
    if nat_eq(a489, a490) {
        Ok(0u64)
    } else {
        let a493 = 4u64;
        let a494 = {
            let a491 = 3u64;
            let a492 = v0;
            nat_mul(a491, a492)?
        };
        nat_add(a493, a494)
    }
}

pub fn f18(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a496 = match v0.clone() {
            Adt2::C0(_, h495, _, _, _, _, _, _, _, _, _) => {
                h495
            }
        };
        let a497 = v1;
        let a498 = 0u64;
        nat_quot(a496, a497, a498)
    }, match v0.clone() {
        Adt2::C0(_, _, h499, _, _, _, _, _, _, _, _) => {
            h499
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h500, _, _, _, _, _, _, _) => {
            h500
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h501, _, _, _, _, _, _) => {
            h501
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h502, _, _, _, _, _) => {
            h502
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h503, _, _, _, _) => {
            h503
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h504, _, _, _) => {
            h504
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h505, _, _) => {
            h505
        }
    })
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a508 = match v0.clone() {
        Adt2::C0(_, h506, _, _, _, _, _, _, _, _, _) => {
            h506
        }
    };
    let a509 = match v1.clone() {
        Adt2::C0(_, h507, _, _, _, _, _, _, _, _, _) => {
            h507
        }
    };
    if if if nat_eq(a508, a509) {
        let a512 = match v0.clone() {
            Adt2::C0(_, _, h510, _, _, _, _, _, _, _, _) => {
                h510
            }
        };
        let a513 = match v1.clone() {
            Adt2::C0(_, _, h511, _, _, _, _, _, _, _, _) => {
                h511
            }
        };
        nat_eq(a512, a513)
    } else {
        false
    } {
        let a516 = match v0.clone() {
            Adt2::C0(_, _, _, h514, _, _, _, _, _, _, _) => {
                h514
            }
        };
        let a517 = match v1.clone() {
            Adt2::C0(_, _, _, h515, _, _, _, _, _, _, _) => {
                h515
            }
        };
        nat_eq(a516, a517)
    } else {
        false
    } {
        let a520 = match v0 {
            Adt2::C0(_, _, _, _, h518, _, _, _, _, _, _) => {
                h518
            }
        };
        let a521 = match v1 {
            Adt2::C0(_, _, _, _, h519, _, _, _, _, _, _) => {
                h519
            }
        };
        nat_eq(a520, a521)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a523 = match v0.clone() {
        Adt2::C0(_, h522, _, _, _, _, _, _, _, _, _) => {
            h522
        }
    };
    let a524 = 1u64;
    if if if if nat_eq(a523, a524) {
        true
    } else {
        let a526 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h525, _, _, _, _, _) => {
                h525
            }
        };
        let a527 = 4u64;
        nat_eq(a526, a527)
    } {
        let a529 = match v0.clone() {
            Adt2::C0(_, _, h528, _, _, _, _, _, _, _, _) => {
                h528
            }
        };
        let a530 = 1u64;
        if nat_eq(a529, a530) {
            true
        } else {
            let a535 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h531, _, _, _, _) => {
                    h531
                }
            };
            let a536 = {
                let a533 = match v0.clone() {
                    Adt2::C0(_, h532, _, _, _, _, _, _, _, _, _) => {
                        h532
                    }
                };
                let a534 = 4u64;
                nat_mul(a533, a534)?
            };
            nat_eq(a535, a536)
        }
    } else {
        false
    } {
        let a538 = match v0.clone() {
            Adt2::C0(_, _, _, h537, _, _, _, _, _, _, _) => {
                h537
            }
        };
        let a539 = 1u64;
        if nat_eq(a538, a539) {
            true
        } else {
            let a547 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h540, _, _, _) => {
                    h540
                }
            };
            let a548 = {
                let a545 = {
                    let a543 = match v0.clone() {
                        Adt2::C0(_, h541, _, _, _, _, _, _, _, _, _) => {
                            h541
                        }
                    };
                    let a544 = match v0.clone() {
                        Adt2::C0(_, _, h542, _, _, _, _, _, _, _, _) => {
                            h542
                        }
                    };
                    nat_mul(a543, a544)?
                };
                let a546 = 4u64;
                nat_mul(a545, a546)?
            };
            nat_eq(a547, a548)
        }
    } else {
        false
    } {
        let a550 = match v0.clone() {
            Adt2::C0(_, _, _, _, h549, _, _, _, _, _, _) => {
                h549
            }
        };
        let a551 = 1u64;
        if nat_eq(a550, a551) {
            Ok(true)
        } else {
            let a562 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h552, _, _) => {
                    h552
                }
            };
            let a563 = {
                let a560 = {
                    let a558 = {
                        let a555 = match v0.clone() {
                            Adt2::C0(_, h553, _, _, _, _, _, _, _, _, _) => {
                                h553
                            }
                        };
                        let a556 = match v0.clone() {
                            Adt2::C0(_, _, h554, _, _, _, _, _, _, _, _) => {
                                h554
                            }
                        };
                        nat_mul(a555, a556)?
                    };
                    let a559 = match v0 {
                        Adt2::C0(_, _, _, h557, _, _, _, _, _, _, _) => {
                            h557
                        }
                    };
                    nat_mul(a558, a559)?
                };
                let a561 = 4u64;
                nat_mul(a560, a561)?
            };
            Ok(nat_eq(a562, a563))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt2, v1: Adt2) -> R<bool> {
    let a565 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h564, _) => {
            h564
        }
    };
    let a566 = 0u64;
    if nat_eq(a565, a566) {
        Ok(true)
    } else {
        let a568 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h567, _) => {
                h567
            }
        };
        let a569 = 0u64;
        if nat_eq(a568, a569) {
            Ok(true)
        } else {
            let a571 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h570) => {
                    h570
                }
            };
            let a572 = 0u64;
            if nat_eq(a571, a572) {
                Ok(true)
            } else {
                let a574 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h573) => {
                        h573
                    }
                };
                let a575 = 0u64;
                if nat_eq(a574, a575) {
                    Ok(true)
                } else {
                    let a581 = {
                        let a578 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h576) => {
                                h576
                            }
                        };
                        let a579 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h577, _) => {
                                h577
                            }
                        };
                        nat_add(a578, a579)?
                    };
                    let a582 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h580) => {
                            h580
                        }
                    };
                    if nat_le(a581, a582) {
                        Ok(true)
                    } else {
                        let a588 = {
                            let a585 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h583) => {
                                    h583
                                }
                            };
                            let a586 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h584, _) => {
                                    h584
                                }
                            };
                            nat_add(a585, a586)?
                        };
                        let a589 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h587) => {
                                h587
                            }
                        };
                        Ok(nat_le(a588, a589))
                    }
                }
            }
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a594 = {
        let a590 = v0;
        let a591 = v1;
        nat_mul(a590, a591)?
    };
    let a595 = {
        let a592 = v2;
        let a593 = f75(v3);
        nat_mul(a592, a593)?
    };
    nat_mul(a594, a595)
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a598 = v0;
    let a599 = {
        let a596 = f76(v1)?;
        let a597 = v2;
        nat_mul(a596, a597)?
    };
    nat_add(a598, a599)
}

pub fn f24(v0: u64) -> bool {
    let a600 = v0;
    let a601 = 2139095040u64;
    if nat_le(a600, a601) {
        true
    } else {
        let a602 = v0;
        let a603 = 2147483648u64;
        nat_eq(a602, a603)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a606 = match v0.clone() {
            Adt2::C0(h604, _, _, _, _, _, _, _, _, _, _) => {
                h604
            }
        };
        let a607 = match v1.clone() {
            Adt2::C0(h605, _, _, _, _, _, _, _, _, _, _) => {
                h605
            }
        };
        nat_eq(a606, a607)
    } else {
        false
    } {
        let a610 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h608) => {
                h608
            }
        };
        let a611 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h609) => {
                h609
            }
        };
        nat_eq(a610, a611)
    } else {
        false
    } {
        let a614 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h612, _, _, _, _, _) => {
                h612
            }
        };
        let a615 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h613, _, _, _, _, _) => {
                h613
            }
        };
        nat_eq(a614, a615)
    } else {
        false
    } {
        let a618 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h616, _, _, _, _) => {
                h616
            }
        };
        let a619 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h617, _, _, _, _) => {
                h617
            }
        };
        nat_eq(a618, a619)
    } else {
        false
    } {
        let a622 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h620, _, _, _) => {
                h620
            }
        };
        let a623 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h621, _, _, _) => {
                h621
            }
        };
        nat_eq(a622, a623)
    } else {
        false
    } {
        let a626 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h624, _, _) => {
                h624
            }
        };
        let a627 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h625, _, _) => {
                h625
            }
        };
        nat_eq(a626, a627)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64) -> R<u64> {
    let a628 = v0;
    let a629 = 0u64;
    if nat_eq(a628, a629) {
        Ok(0u64)
    } else {
        let a630 = v1;
        let a631 = f77(v0)?;
        nat_mul(a630, a631)
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a632 = v0;
    let a633 = 0u64;
    if if if if nat_eq(a632, a633) {
        true
    } else {
        let a634 = v1;
        let a635 = 0u64;
        nat_eq(a634, a635)
    } {
        true
    } else {
        let a636 = v2;
        let a637 = 0u64;
        nat_eq(a636, a637)
    } {
        true
    } else {
        let a638 = v3;
        let a639 = 0u64;
        nat_eq(a638, a639)
    } {
        Ok(0u64)
    } else {
        let a652 = {
            let a642 = 2u64;
            let a643 = {
                let a640 = v1;
                let a641 = v2;
                nat_mul(a640, a641)?
            };
            nat_mul(a642, a643)?
        };
        let a653 = {
            let a650 = {
                let a646 = {
                    let a644 = v1;
                    let a645 = v2;
                    nat_mul(a644, a645)?
                };
                let a647 = v3;
                nat_mul(a646, a647)?
            };
            let a651 = {
                let a648 = 1u64;
                let a649 = v0;
                nat_add(a648, a649)?
            };
            nat_mul(a650, a651)?
        };
        nat_add(a652, a653)
    }
}

pub fn f28(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m654 = v1;
    match m654 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f78(v0, v3, v2)
        }
    }
}

pub fn f29(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a656 = match v0.clone() {
        Adt2::C0(h655, _, _, _, _, _, _, _, _, _, _) => {
            h655
        }
    };
    let a657 = 1u64;
    if if nat_eq(a656, a657) {
        true
    } else {
        let a659 = match v0.clone() {
            Adt2::C0(h658, _, _, _, _, _, _, _, _, _, _) => {
                h658
            }
        };
        let a660 = 30u64;
        nat_eq(a659, a660)
    } {
        let a662 = match v1.clone() {
            Adt2::C0(h661, _, _, _, _, _, _, _, _, _, _) => {
                h661
            }
        };
        let a663 = 1u64;
        if if if if if if if if if if if nat_eq(a662, a663) {
            let a666 = match v0.clone() {
                Adt2::C0(_, h664, _, _, _, _, _, _, _, _, _) => {
                    h664
                }
            };
            let a667 = match v1.clone() {
                Adt2::C0(_, h665, _, _, _, _, _, _, _, _, _) => {
                    h665
                }
            };
            nat_eq(a666, a667)
        } else {
            false
        } {
            let a669 = 0u64;
            let a670 = match v1.clone() {
                Adt2::C0(_, _, h668, _, _, _, _, _, _, _, _) => {
                    h668
                }
            };
            nat_lt(a669, a670)
        } else {
            false
        } {
            let a673 = match v1.clone() {
                Adt2::C0(_, _, h671, _, _, _, _, _, _, _, _) => {
                    h671
                }
            };
            let a674 = match v1.clone() {
                Adt2::C0(_, _, _, h672, _, _, _, _, _, _, _) => {
                    h672
                }
            };
            nat_eq(a673, a674)
        } else {
            false
        } {
            let a676 = match v1.clone() {
                Adt2::C0(_, _, _, _, h675, _, _, _, _, _, _) => {
                    h675
                }
            };
            let a677 = 1u64;
            nat_eq(a676, a677)
        } else {
            false
        } {
            let a685 = match v0.clone() {
                Adt2::C0(_, _, h678, _, _, _, _, _, _, _, _) => {
                    h678
                }
            };
            let a686 = {
                let a683 = {
                    let a681 = match v1.clone() {
                        Adt2::C0(_, _, h679, _, _, _, _, _, _, _, _) => {
                            h679
                        }
                    };
                    let a682 = match v1.clone() {
                        Adt2::C0(_, _, h680, _, _, _, _, _, _, _, _) => {
                            h680
                        }
                    };
                    nat_add(a681, a682)?
                };
                let a684 = 1u64;
                nat_sub(a683, a684)
            };
            nat_eq(a685, a686)
        } else {
            false
        } {
            let a688 = 0u64;
            let a689 = match v0.clone() {
                Adt2::C0(_, _, _, h687, _, _, _, _, _, _, _) => {
                    h687
                }
            };
            nat_lt(a688, a689)
        } else {
            false
        } {
            let a691 = 0u64;
            let a692 = match v0.clone() {
                Adt2::C0(_, _, _, _, h690, _, _, _, _, _, _) => {
                    h690
                }
            };
            nat_lt(a691, a692)
        } else {
            false
        } {
            let a694 = 2u64;
            let a695 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h693, _, _, _, _, _) => {
                    h693
                }
            };
            nat_le(a694, a695)
        } else {
            false
        } {
            let a697 = 2u64;
            let a698 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h696, _, _, _, _, _) => {
                    h696
                }
            };
            nat_le(a697, a698)
        } else {
            false
        } {
            f21(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f14(v1.clone(), 1u64)?, 2u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f30(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m699 = v1;
    match m699 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a701 = match v0.clone() {
                Adt2::C0(h700, _, _, _, _, _, _, _, _, _, _) => {
                    h700
                }
            };
            let a702 = 0u64;
            if if if if if if if if if if if if if nat_eq(a701, a702) {
                true
            } else {
                let a704 = match v0.clone() {
                    Adt2::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
                        h703
                    }
                };
                let a705 = 1u64;
                nat_eq(a704, a705)
            } {
                let a707 = match v3.clone() {
                    Adt2::C0(h706, _, _, _, _, _, _, _, _, _, _) => {
                        h706
                    }
                };
                let a708 = 26u64;
                nat_eq(a707, a708)
            } else {
                false
            } {
                let a710 = match v2.clone() {
                    Adt2::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
                        h709
                    }
                };
                let a711 = 0u64;
                nat_eq(a710, a711)
            } else {
                false
            } {
                let a714 = match v0.clone() {
                    Adt2::C0(_, h712, _, _, _, _, _, _, _, _, _) => {
                        h712
                    }
                };
                let a715 = match v2.clone() {
                    Adt2::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                        h713
                    }
                };
                nat_eq(a714, a715)
            } else {
                false
            } {
                let a718 = match v3.clone() {
                    Adt2::C0(_, h716, _, _, _, _, _, _, _, _, _) => {
                        h716
                    }
                };
                let a719 = match v0.clone() {
                    Adt2::C0(_, _, h717, _, _, _, _, _, _, _, _) => {
                        h717
                    }
                };
                nat_le(a718, a719)
            } else {
                false
            } {
                let a721 = match v0.clone() {
                    Adt2::C0(_, _, _, h720, _, _, _, _, _, _, _) => {
                        h720
                    }
                };
                let a722 = 1u64;
                nat_eq(a721, a722)
            } else {
                false
            } {
                let a724 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h723, _, _, _, _, _, _) => {
                        h723
                    }
                };
                let a725 = 1u64;
                nat_eq(a724, a725)
            } else {
                false
            } {
                let a727 = match v2.clone() {
                    Adt2::C0(_, _, _, h726, _, _, _, _, _, _, _) => {
                        h726
                    }
                };
                let a728 = 1u64;
                nat_eq(a727, a728)
            } else {
                false
            } {
                let a730 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h729, _, _, _, _, _, _) => {
                        h729
                    }
                };
                let a731 = 1u64;
                nat_eq(a730, a731)
            } else {
                false
            } {
                let a733 = match v3.clone() {
                    Adt2::C0(_, _, h732, _, _, _, _, _, _, _, _) => {
                        h732
                    }
                };
                let a734 = 1u64;
                nat_eq(a733, a734)
            } else {
                false
            } {
                let a736 = match v3.clone() {
                    Adt2::C0(_, _, _, h735, _, _, _, _, _, _, _) => {
                        h735
                    }
                };
                let a737 = 1u64;
                nat_eq(a736, a737)
            } else {
                false
            } {
                let a739 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h738, _, _, _, _, _, _) => {
                        h738
                    }
                };
                let a740 = 1u64;
                nat_eq(a739, a740)
            } else {
                false
            } {
                if f79(match v2.clone() {
                    Adt2::C0(_, h741, _, _, _, _, _, _, _, _, _) => {
                        h741
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h742, _, _, _, _, _, _, _, _) => {
                        h742
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h743, _, _, _, _, _, _, _) => {
                        h743
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h744, _, _, _, _, _, _) => {
                        h744
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a746 = match v3.clone() {
                        Adt2::C0(_, h745, _, _, _, _, _, _, _, _, _) => {
                            h745
                        }
                    };
                    let a747 = 0u64;
                    if nat_eq(a746, a747) {
                        Ok(Adt6::C4(f14(v2.clone(), 1u64)?, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a749 = match v0.clone() {
                            Adt2::C0(_, h748, _, _, _, _, _, _, _, _, _) => {
                                h748
                            }
                        };
                        let a750 = 2147483647u64;
                        if if if if if if nat_le(a749, a750) {
                            let a752 = match v3.clone() {
                                Adt2::C0(_, h751, _, _, _, _, _, _, _, _, _) => {
                                    h751
                                }
                            };
                            let a753 = 2147483647u64;
                            nat_le(a752, a753)
                        } else {
                            false
                        } {
                            let a755 = 4u64;
                            let a756 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h754, _, _, _, _, _) => {
                                    h754
                                }
                            };
                            nat_le(a755, a756)
                        } else {
                            false
                        } {
                            let a762 = {
                                let a759 = match v2.clone() {
                                    Adt2::C0(_, h757, _, _, _, _, _, _, _, _, _) => {
                                        h757
                                    }
                                };
                                let a760 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h758, _, _, _, _, _) => {
                                        h758
                                    }
                                };
                                nat_mul(a759, a760)?
                            };
                            let a763 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h761, _, _, _, _) => {
                                    h761
                                }
                            };
                            nat_le(a762, a763)
                        } else {
                            false
                        } {
                            f21(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f21(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a777 = {
                                    let a767 = {
                                        let a765 = 2u64;
                                        let a766 = match v3.clone() {
                                            Adt2::C0(_, h764, _, _, _, _, _, _, _, _, _) => {
                                                h764
                                            }
                                        };
                                        nat_mul(a765, a766)?
                                    };
                                    let a768 = f14(v2.clone(), 1u64)?;
                                    nat_add(a767, a768)?
                                };
                                let a778 = {
                                    let a775 = match v3.clone() {
                                        Adt2::C0(_, h769, _, _, _, _, _, _, _, _, _) => {
                                            h769
                                        }
                                    };
                                    let a776 = {
                                        let a773 = 1u64;
                                        let a774 = {
                                            let a771 = 2u64;
                                            let a772 = match v0.clone() {
                                                Adt2::C0(_, h770, _, _, _, _, _, _, _, _, _) => {
                                                    h770
                                                }
                                            };
                                            nat_mul(a771, a772)?
                                        };
                                        nat_add(a773, a774)?
                                    };
                                    nat_mul(a775, a776)?
                                };
                                nat_add(a777, a778)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h779, _, _, _, _, _, _, _, _, _) => {
                                    h779
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h780, _, _, _, _, _, _, _, _, _) => {
                                    h780
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h781, _, _, _, _, _) => {
                                    h781
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h782, _, _, _, _) => {
                                    h782
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h783, _, _, _) => {
                                    h783
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h784, _, _) => {
                                    h784
                                }
                            }), f18(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h785, _, _, _, _, _, _, _, _, _, _) => {
                                    h785
                                }
                            }, f18(v3, 1u64))))
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

pub fn f31(v0: u64) -> bool {
    let a786 = v0;
    let a787 = 1u64;
    if if nat_eq(a786, a787) {
        true
    } else {
        let a788 = v0;
        let a789 = 33u64;
        nat_eq(a788, a789)
    } {
        true
    } else {
        let a790 = v0;
        let a791 = 34u64;
        nat_eq(a790, a791)
    }
}

pub fn f32(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a794 = match v1.clone() {
        Adt2::C0(h792, _, _, _, _, _, _, _, _, _, _) => {
            h792
        }
    };
    let a795 = match v3.clone() {
        Adt2::C0(h793, _, _, _, _, _, _, _, _, _, _) => {
            h793
        }
    };
    if nat_eq(a794, a795) {
        f80(v0, v1, v2, v3)
    } else {
        if if if if f81(match v1.clone() {
            Adt2::C0(h796, _, _, _, _, _, _, _, _, _, _) => {
                h796
            }
        }) {
            f81(match v3.clone() {
                Adt2::C0(h797, _, _, _, _, _, _, _, _, _, _) => {
                    h797
                }
            })
        } else {
            false
        } {
            let a798 = v0;
            let a799 = 33u64;
            if nat_eq(a798, a799) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a800 = f14(v1.clone(), 1u64)?;
            let a801 = f14(v3.clone(), 1u64)?;
            nat_eq(a800, a801)
        } else {
            false
        } {
            let m803 = f82(match v3.clone() {
                Adt2::C0(h802, _, _, _, _, _, _, _, _, _, _) => {
                    h802
                }
            });
            match m803 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f14(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h804) => {
                            h804
                        }
                    }, f18(v1.clone(), 1u64), f18(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h805, _, _, _, _, _, _, _, _, _, _) => {
                            h805
                        }
                    }, match v3 {
                        Adt2::C0(h806, _, _, _, _, _, _, _, _, _, _) => {
                            h806
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a808 = match v0.clone() {
        Adt2::C0(h807, _, _, _, _, _, _, _, _, _, _) => {
            h807
        }
    };
    let a809 = 0u64;
    if if if if nat_eq(a808, a809) {
        true
    } else {
        let a811 = match v0.clone() {
            Adt2::C0(h810, _, _, _, _, _, _, _, _, _, _) => {
                h810
            }
        };
        let a812 = 1u64;
        nat_eq(a811, a812)
    } {
        true
    } else {
        let a814 = match v0.clone() {
            Adt2::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
                h813
            }
        };
        let a815 = 25u64;
        if nat_eq(a814, a815) {
            true
        } else {
            let a817 = match v0.clone() {
                Adt2::C0(h816, _, _, _, _, _, _, _, _, _, _) => {
                    h816
                }
            };
            let a818 = 26u64;
            nat_eq(a817, a818)
        }
    } {
        true
    } else {
        let a820 = match v0.clone() {
            Adt2::C0(h819, _, _, _, _, _, _, _, _, _, _) => {
                h819
            }
        };
        let a821 = 30u64;
        nat_eq(a820, a821)
    } {
        let a824 = match v0.clone() {
            Adt2::C0(h822, _, _, _, _, _, _, _, _, _, _) => {
                h822
            }
        };
        let a825 = match v1.clone() {
            Adt2::C0(h823, _, _, _, _, _, _, _, _, _, _) => {
                h823
            }
        };
        if if nat_eq(a824, a825) {
            f83(match v1.clone() {
                Adt2::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                    h826
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h827, _, _, _, _, _, _, _, _) => {
                    h827
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h828, _, _, _, _, _, _, _) => {
                    h828
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h829, _, _, _, _, _, _) => {
                    h829
                }
            }, match v0.clone() {
                Adt2::C0(_, h830, _, _, _, _, _, _, _, _, _) => {
                    h830
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h831, _, _, _, _, _, _, _, _) => {
                    h831
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h832, _, _, _, _, _, _, _) => {
                    h832
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h833, _, _, _, _, _, _) => {
                    h833
                }
            })
        } else {
            false
        } {
            let m835 = f82(match v0.clone() {
                Adt2::C0(h834, _, _, _, _, _, _, _, _, _, _) => {
                    h834
                }
            });
            match m835 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f14(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h836) => {
                            h836
                        }
                    }, f18(v0, 1u64), f18(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f34(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m837 = v2;
    match m837 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a838 = v0;
            let a839 = 4u64;
            if if if nat_lt(a838, a839) {
                let a842 = match v1.clone() {
                    Adt2::C0(h840, _, _, _, _, _, _, _, _, _, _) => {
                        h840
                    }
                };
                let a843 = match v4.clone() {
                    Adt2::C0(h841, _, _, _, _, _, _, _, _, _, _) => {
                        h841
                    }
                };
                if nat_eq(a842, a843) {
                    let a846 = match v1.clone() {
                        Adt2::C0(h844, _, _, _, _, _, _, _, _, _, _) => {
                            h844
                        }
                    };
                    let a847 = match v3.clone() {
                        Adt2::C0(h845, _, _, _, _, _, _, _, _, _, _) => {
                            h845
                        }
                    };
                    nat_eq(a846, a847)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f84(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                        h848
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h849, _, _, _, _, _, _, _, _, _) => {
                        h849
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h850, _, _, _, _, _, _, _, _, _) => {
                        h850
                    }
                })? {
                    f84(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h851, _, _, _, _, _, _, _, _) => {
                            h851
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h852, _, _, _, _, _, _, _, _) => {
                            h852
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h853, _, _, _, _, _, _, _, _) => {
                            h853
                        }
                    })?
                } else {
                    false
                } {
                    f84(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h854, _, _, _, _, _, _, _) => {
                            h854
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h855, _, _, _, _, _, _, _) => {
                            h855
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h856, _, _, _, _, _, _, _) => {
                            h856
                        }
                    })?
                } else {
                    false
                } {
                    f84(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h857, _, _, _, _, _, _) => {
                            h857
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h858, _, _, _, _, _, _) => {
                            h858
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h859, _, _, _, _, _, _) => {
                            h859
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m861 = f82(match v1.clone() {
                    Adt2::C0(h860, _, _, _, _, _, _, _, _, _, _) => {
                        h860
                    }
                });
                match m861 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h862, _) => {
                                h862
                            }
                        };
                        let a867 = {
                            let a864 = match v1.clone() {
                                Adt2::C0(_, h863, _, _, _, _, _, _, _, _, _) => {
                                    h863
                                }
                            };
                            let a865 = v6;
                            let a866 = 1u64;
                            nat_rem(a864, a865, a866)
                        };
                        let a868 = 0u64;
                        if if if nat_eq(a867, a868) {
                            let a873 = {
                                let a870 = match v4.clone() {
                                    Adt2::C0(_, h869, _, _, _, _, _, _, _, _, _) => {
                                        h869
                                    }
                                };
                                let a871 = v6;
                                let a872 = 1u64;
                                nat_rem(a870, a871, a872)
                            };
                            let a874 = 0u64;
                            nat_eq(a873, a874)
                        } else {
                            false
                        } {
                            let a879 = {
                                let a876 = match v3.clone() {
                                    Adt2::C0(_, h875, _, _, _, _, _, _, _, _, _) => {
                                        h875
                                    }
                                };
                                let a877 = v6;
                                let a878 = 1u64;
                                nat_rem(a876, a877, a878)
                            };
                            let a880 = 0u64;
                            nat_eq(a879, a880)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f14(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h881) => {
                                    h881
                                }
                            }, f18(v1, v6), f18(v3, v6), Adt4::C2(v0, f18(v4, v6))))
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

pub fn f35(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a883 = match v2.clone() {
        Adt2::C0(h882, _, _, _, _, _, _, _, _, _, _) => {
            h882
        }
    };
    let a884 = 0u64;
    if if nat_eq(a883, a884) {
        let a886 = match v3.clone() {
            Adt2::C0(h885, _, _, _, _, _, _, _, _, _, _) => {
                h885
            }
        };
        let a887 = 0u64;
        nat_eq(a886, a887)
    } else {
        false
    } {
        let a888 = v0;
        let a889 = 61u64;
        if nat_eq(a888, a889) {
            let v4: bool = {
                let a891 = 0u64;
                let a892 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h890, _, _, _, _, _, _, _) => {
                        h890
                    }
                };
                nat_lt(a891, a892)
            };
            if if if if if if f85(match v2.clone() {
                Adt2::C0(_, h893, _, _, _, _, _, _, _, _, _) => {
                    h893
                }
            }, match v1.clone() {
                Adt1::C0(h894, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h894
                }
            }, match v1.clone() {
                Adt1::C0(_, h895, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h895
                }
            }, match v3.clone() {
                Adt2::C0(_, h896, _, _, _, _, _, _, _, _, _) => {
                    h896
                }
            })? {
                f85(match v2.clone() {
                    Adt2::C0(_, _, h897, _, _, _, _, _, _, _, _) => {
                        h897
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h898, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h898
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h899, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h899
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h900, _, _, _, _, _, _, _, _) => {
                        h900
                    }
                })?
            } else {
                false
            } {
                f85(match v2.clone() {
                    Adt2::C0(_, _, _, h901, _, _, _, _, _, _, _) => {
                        h901
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h902, _, _, _, _, _, _, _, _, _, _, _) => {
                        h902
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h903, _, _, _, _, _, _, _, _, _, _) => {
                        h903
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h904, _, _, _, _, _, _, _) => {
                        h904
                    }
                })?
            } else {
                false
            } {
                f85(match v2.clone() {
                    Adt2::C0(_, _, _, _, h905, _, _, _, _, _, _) => {
                        h905
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h906, _, _, _, _, _, _, _, _, _) => {
                        h906
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h907, _, _, _, _, _, _, _, _) => {
                        h907
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h908, _, _, _, _, _, _) => {
                        h908
                    }
                })?
            } else {
                false
            } {
                let a910 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h909, _, _, _, _, _, _, _) => {
                        h909
                    }
                };
                let a911 = 4294967295u64;
                nat_le(a910, a911)
            } else {
                false
            } {
                if v4 {
                    !f79(match v2.clone() {
                        Adt2::C0(_, h912, _, _, _, _, _, _, _, _, _) => {
                            h912
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h913, _, _, _, _, _, _, _, _) => {
                            h913
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h914, _, _, _, _, _, _, _) => {
                            h914
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h915, _, _, _, _, _, _) => {
                            h915
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h916, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h916
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h917, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h917
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h918, _, _, _, _, _, _, _, _, _, _, _) => {
                        h918
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h919, _, _, _, _, _, _, _, _, _) => {
                        h919
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a920 = v0;
            let a921 = 62u64;
            if nat_eq(a920, a921) {
                let a923 = match v1.clone() {
                    Adt1::C0(h922, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h922
                    }
                };
                let a924 = 2147483648u64;
                if if if if if nat_lt(a923, a924) {
                    let a926 = match v1.clone() {
                        Adt1::C0(_, h925, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h925
                        }
                    };
                    let a927 = 2147483648u64;
                    nat_lt(a926, a927)
                } else {
                    false
                } {
                    let a930 = match v1.clone() {
                        Adt1::C0(h928, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h928
                        }
                    };
                    let a931 = match v2.clone() {
                        Adt2::C0(_, h929, _, _, _, _, _, _, _, _, _) => {
                            h929
                        }
                    };
                    nat_lt(a930, a931)
                } else {
                    false
                } {
                    let a934 = match v1.clone() {
                        Adt1::C0(_, h932, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h932
                        }
                    };
                    let a935 = match v2.clone() {
                        Adt2::C0(_, h933, _, _, _, _, _, _, _, _, _) => {
                            h933
                        }
                    };
                    nat_lt(a934, a935)
                } else {
                    false
                } {
                    let a938 = match v2.clone() {
                        Adt2::C0(_, _, h936, _, _, _, _, _, _, _, _) => {
                            h936
                        }
                    };
                    let a939 = match v3.clone() {
                        Adt2::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                            h937
                        }
                    };
                    if if if nat_eq(a938, a939) {
                        let a942 = match v2.clone() {
                            Adt2::C0(_, _, _, h940, _, _, _, _, _, _, _) => {
                                h940
                            }
                        };
                        let a943 = match v3.clone() {
                            Adt2::C0(_, _, _, h941, _, _, _, _, _, _, _) => {
                                h941
                            }
                        };
                        nat_eq(a942, a943)
                    } else {
                        false
                    } {
                        let a946 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h944, _, _, _, _, _, _) => {
                                h944
                            }
                        };
                        let a947 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h945, _, _, _, _, _, _) => {
                                h945
                            }
                        };
                        nat_eq(a946, a947)
                    } else {
                        false
                    } {
                        let a956 = match v3.clone() {
                            Adt2::C0(_, h948, _, _, _, _, _, _, _, _, _) => {
                                h948
                            }
                        };
                        let a957 = {
                            let a954 = {
                                let a951 = match v2.clone() {
                                    Adt2::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                                        h949
                                    }
                                };
                                let a952 = match v1.clone() {
                                    Adt1::C0(h950, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h950
                                    }
                                };
                                nat_add(a951, a952)?
                            };
                            let a955 = match v1.clone() {
                                Adt1::C0(_, h953, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h953
                                }
                            };
                            nat_add(a954, a955)?
                        };
                        nat_eq(a956, a957)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h958, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h958
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a959 = v0;
                let a960 = 63u64;
                if nat_eq(a959, a960) {
                    if if if if if f86(match v2.clone() {
                        Adt2::C0(_, h961, _, _, _, _, _, _, _, _, _) => {
                            h961
                        }
                    }, match v1.clone() {
                        Adt1::C0(h962, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h962
                        }
                    })? {
                        f86(match v2.clone() {
                            Adt2::C0(_, _, h963, _, _, _, _, _, _, _, _) => {
                                h963
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h964, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h964
                            }
                        })?
                    } else {
                        false
                    } {
                        f86(match v2.clone() {
                            Adt2::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                                h965
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h966, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h966
                            }
                        })?
                    } else {
                        false
                    } {
                        f86(match v2.clone() {
                            Adt2::C0(_, _, _, _, h967, _, _, _, _, _, _) => {
                                h967
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h968, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h968
                            }
                        })?
                    } else {
                        false
                    } {
                        let a971 = match v2.clone() {
                            Adt2::C0(_, h969, _, _, _, _, _, _, _, _, _) => {
                                h969
                            }
                        };
                        let a972 = match v3.clone() {
                            Adt2::C0(_, h970, _, _, _, _, _, _, _, _, _) => {
                                h970
                            }
                        };
                        if if if nat_eq(a971, a972) {
                            let a975 = match v2.clone() {
                                Adt2::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                                    h973
                                }
                            };
                            let a976 = match v3.clone() {
                                Adt2::C0(_, _, h974, _, _, _, _, _, _, _, _) => {
                                    h974
                                }
                            };
                            nat_eq(a975, a976)
                        } else {
                            false
                        } {
                            let a979 = match v2.clone() {
                                Adt2::C0(_, _, _, h977, _, _, _, _, _, _, _) => {
                                    h977
                                }
                            };
                            let a980 = match v3.clone() {
                                Adt2::C0(_, _, _, h978, _, _, _, _, _, _, _) => {
                                    h978
                                }
                            };
                            nat_eq(a979, a980)
                        } else {
                            false
                        } {
                            let a983 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h981, _, _, _, _, _, _) => {
                                    h981
                                }
                            };
                            let a984 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h982, _, _, _, _, _, _) => {
                                    h982
                                }
                            };
                            nat_eq(a983, a984)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h985, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h985
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h986, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h986
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h987, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h987
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h988, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h988
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

pub fn f36(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m989 = v3;
    match m989 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a990 = v0;
                let a991 = 4u64;
                nat_eq(a990, a991)
            };
            let v7: bool = {
                let a993 = 0u64;
                let a994 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h992, _, _, _, _, _, _, _, _, _, _, _) => {
                        h992
                    }
                };
                nat_lt(a993, a994)
            };
            let v8: u64 = f14(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h995, _, _, _, _, _, _, _, _, _) => {
                    h995
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h996, _, _, _, _, _, _, _, _) => {
                    h996
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h997, _, _, _, _, _, _, _) => {
                    h997
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h998, _, _, _, _, _, _) => {
                    h998
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h999, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h999
                }
            }, match v1.clone() {
                Adt1::C0(_, h1000, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1000
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1001, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1001
                }
            });
            if if if if if if if if if if if if v6 {
                let a1003 = match v2.clone() {
                    Adt2::C0(h1002, _, _, _, _, _, _, _, _, _, _) => {
                        h1002
                    }
                };
                let a1004 = 0u64;
                if nat_eq(a1003, a1004) {
                    let a1006 = match v5.clone() {
                        Adt2::C0(h1005, _, _, _, _, _, _, _, _, _, _) => {
                            h1005
                        }
                    };
                    let a1007 = 0u64;
                    nat_eq(a1006, a1007)
                } else {
                    false
                }
            } else {
                let a1009 = match v2.clone() {
                    Adt2::C0(h1008, _, _, _, _, _, _, _, _, _, _) => {
                        h1008
                    }
                };
                let a1010 = 0u64;
                if if nat_eq(a1009, a1010) {
                    true
                } else {
                    let a1012 = match v2.clone() {
                        Adt2::C0(h1011, _, _, _, _, _, _, _, _, _, _) => {
                            h1011
                        }
                    };
                    let a1013 = 26u64;
                    nat_eq(a1012, a1013)
                } {
                    let a1015 = match v5.clone() {
                        Adt2::C0(h1014, _, _, _, _, _, _, _, _, _, _) => {
                            h1014
                        }
                    };
                    let a1016 = 0u64;
                    if nat_eq(a1015, a1016) {
                        true
                    } else {
                        let a1018 = match v5.clone() {
                            Adt2::C0(h1017, _, _, _, _, _, _, _, _, _, _) => {
                                h1017
                            }
                        };
                        let a1019 = 26u64;
                        nat_eq(a1018, a1019)
                    }
                } else {
                    false
                }
            } {
                let a1022 = match v2.clone() {
                    Adt2::C0(h1020, _, _, _, _, _, _, _, _, _, _) => {
                        h1020
                    }
                };
                let a1023 = match v4.clone() {
                    Adt2::C0(h1021, _, _, _, _, _, _, _, _, _, _) => {
                        h1021
                    }
                };
                nat_eq(a1022, a1023)
            } else {
                false
            } {
                f19(v2.clone(), v4.clone())
            } else {
                false
            } {
                f20(v2.clone())?
            } else {
                false
            } {
                f20(v4.clone())?
            } else {
                false
            } {
                let a1025 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1024, _, _, _, _, _) => {
                        h1024
                    }
                };
                let a1026 = 4u64;
                nat_eq(a1025, a1026)
            } else {
                false
            } {
                let a1027 = v8;
                let a1028 = f14(v2.clone(), 1u64)?;
                nat_le(a1027, a1028)
            } else {
                false
            } {
                let a1030 = match v1.clone() {
                    Adt1::C0(_, _, _, h1029, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1029
                    }
                };
                let a1031 = 1073741824u64;
                nat_lt(a1030, a1031)
            } else {
                false
            } {
                let a1033 = match v1.clone() {
                    Adt1::C0(h1032, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1032
                    }
                };
                let a1034 = 4294967295u64;
                if if if if nat_le(a1033, a1034) {
                    let a1036 = match v1.clone() {
                        Adt1::C0(_, h1035, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1035
                        }
                    };
                    let a1037 = 4294967295u64;
                    nat_le(a1036, a1037)
                } else {
                    false
                } {
                    let a1039 = match v1.clone() {
                        Adt1::C0(_, _, h1038, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1038
                        }
                    };
                    let a1040 = 4294967295u64;
                    nat_le(a1039, a1040)
                } else {
                    false
                } {
                    let a1042 = match v1.clone() {
                        Adt1::C0(_, _, _, h1041, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1041
                        }
                    };
                    let a1043 = 4294967295u64;
                    nat_le(a1042, a1043)
                } else {
                    false
                } {
                    let a1045 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1044, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1044
                        }
                    };
                    let a1046 = 4294967295u64;
                    nat_le(a1045, a1046)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1049 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1047) => {
                            h1047
                        }
                    };
                    let a1050 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1048) => {
                            h1048
                        }
                    };
                    nat_eq(a1049, a1050)
                } else {
                    true
                }
            } else {
                false
            } {
                f87(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1051, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1051
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1052, _) => {
                        h1052
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f14(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a1053 = v10;
                    let a1054 = v8;
                    nat_add(a1053, a1054)?
                }, 4u64, f18(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1055, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1055
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f37(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1057 = match v0.clone() {
        Adt2::C0(h1056, _, _, _, _, _, _, _, _, _, _) => {
            h1056
        }
    };
    let a1058 = 0u64;
    if if if nat_eq(a1057, a1058) {
        let a1060 = match v1.clone() {
            Adt2::C0(h1059, _, _, _, _, _, _, _, _, _, _) => {
                h1059
            }
        };
        let a1061 = 0u64;
        nat_eq(a1060, a1061)
    } else {
        false
    } {
        f83(match v0.clone() {
            Adt2::C0(_, h1062, _, _, _, _, _, _, _, _, _) => {
                h1062
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1063, _, _, _, _, _, _, _, _) => {
                h1063
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1064, _, _, _, _, _, _, _) => {
                h1064
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1065, _, _, _, _, _, _) => {
                h1065
            }
        }, match v1.clone() {
            Adt2::C0(_, h1066, _, _, _, _, _, _, _, _, _) => {
                h1066
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1067, _, _, _, _, _, _, _, _) => {
                h1067
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1068, _, _, _, _, _, _, _) => {
                h1068
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1069, _, _, _, _, _, _) => {
                h1069
            }
        })
    } else {
        false
    } {
        let v2: u64 = f14(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a1070 = v2;
            let a1071 = f14(v0.clone(), 1u64)?;
            nat_add(a1070, a1071)?
        }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f38(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1073 = match v1.clone() {
        Adt2::C0(h1072, _, _, _, _, _, _, _, _, _, _) => {
            h1072
        }
    };
    let a1074 = 0u64;
    if if if nat_eq(a1073, a1074) {
        true
    } else {
        let a1076 = match v1.clone() {
            Adt2::C0(h1075, _, _, _, _, _, _, _, _, _, _) => {
                h1075
            }
        };
        let a1077 = 1u64;
        nat_eq(a1076, a1077)
    } {
        true
    } else {
        let a1079 = match v1.clone() {
            Adt2::C0(h1078, _, _, _, _, _, _, _, _, _, _) => {
                h1078
            }
        };
        let a1080 = 30u64;
        nat_eq(a1079, a1080)
    } {
        let a1083 = match v1.clone() {
            Adt2::C0(h1081, _, _, _, _, _, _, _, _, _, _) => {
                h1081
            }
        };
        let a1084 = match v2.clone() {
            Adt2::C0(h1082, _, _, _, _, _, _, _, _, _, _) => {
                h1082
            }
        };
        if if if if if nat_eq(a1083, a1084) {
            let a1087 = match v1.clone() {
                Adt2::C0(_, h1085, _, _, _, _, _, _, _, _, _) => {
                    h1085
                }
            };
            let a1088 = match v2.clone() {
                Adt2::C0(_, h1086, _, _, _, _, _, _, _, _, _) => {
                    h1086
                }
            };
            nat_eq(a1087, a1088)
        } else {
            false
        } {
            let a1091 = match v1.clone() {
                Adt2::C0(_, _, _, h1089, _, _, _, _, _, _, _) => {
                    h1089
                }
            };
            let a1092 = match v2.clone() {
                Adt2::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
                    h1090
                }
            };
            nat_eq(a1091, a1092)
        } else {
            false
        } {
            let a1095 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1093, _, _, _, _, _, _) => {
                    h1093
                }
            };
            let a1096 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1094, _, _, _, _, _, _) => {
                    h1094
                }
            };
            nat_eq(a1095, a1096)
        } else {
            false
        } {
            let a1097 = v0;
            let a1098 = 4u64;
            if nat_eq(a1097, a1098) {
                let a1100 = match v1.clone() {
                    Adt2::C0(_, _, h1099, _, _, _, _, _, _, _, _) => {
                        h1099
                    }
                };
                let a1101 = 1u64;
                if nat_eq(a1100, a1101) {
                    let a1104 = match v1.clone() {
                        Adt2::C0(_, h1102, _, _, _, _, _, _, _, _, _) => {
                            h1102
                        }
                    };
                    let a1105 = match v2.clone() {
                        Adt2::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                            h1103
                        }
                    };
                    nat_eq(a1104, a1105)
                } else {
                    false
                }
            } else {
                let a1106 = v0;
                let a1107 = 4u64;
                if if nat_lt(a1106, a1107) {
                    let a1110 = match v1.clone() {
                        Adt2::C0(_, h1108, _, _, _, _, _, _, _, _, _) => {
                            h1108
                        }
                    };
                    let a1111 = match v1.clone() {
                        Adt2::C0(_, _, h1109, _, _, _, _, _, _, _, _) => {
                            h1109
                        }
                    };
                    nat_eq(a1110, a1111)
                } else {
                    false
                } {
                    let a1114 = match v1.clone() {
                        Adt2::C0(_, _, h1112, _, _, _, _, _, _, _, _) => {
                            h1112
                        }
                    };
                    let a1115 = match v2.clone() {
                        Adt2::C0(_, _, h1113, _, _, _, _, _, _, _, _) => {
                            h1113
                        }
                    };
                    nat_eq(a1114, a1115)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1117 = f82(match v1.clone() {
                Adt2::C0(h1116, _, _, _, _, _, _, _, _, _, _) => {
                    h1116
                }
            });
            match m1117 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f14(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h1118) => {
                            h1118
                        }
                    }, f18(v1, 1u64), f18(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f39(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f88(match v1.clone() {
        Adt2::C0(h1119, _, _, _, _, _, _, _, _, _, _) => {
            h1119
        }
    }, v0) {
        let a1122 = match v1.clone() {
            Adt2::C0(h1120, _, _, _, _, _, _, _, _, _, _) => {
                h1120
            }
        };
        let a1123 = match v2.clone() {
            Adt2::C0(h1121, _, _, _, _, _, _, _, _, _, _) => {
                h1121
            }
        };
        nat_eq(a1122, a1123)
    } else {
        false
    } {
        let a1126 = match v1.clone() {
            Adt2::C0(_, h1124, _, _, _, _, _, _, _, _, _) => {
                h1124
            }
        };
        let a1127 = match v2.clone() {
            Adt2::C0(_, h1125, _, _, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        if if if nat_eq(a1126, a1127) {
            let a1130 = match v1.clone() {
                Adt2::C0(_, _, h1128, _, _, _, _, _, _, _, _) => {
                    h1128
                }
            };
            let a1131 = match v2.clone() {
                Adt2::C0(_, _, h1129, _, _, _, _, _, _, _, _) => {
                    h1129
                }
            };
            nat_eq(a1130, a1131)
        } else {
            false
        } {
            let a1134 = match v1.clone() {
                Adt2::C0(_, _, _, h1132, _, _, _, _, _, _, _) => {
                    h1132
                }
            };
            let a1135 = match v2.clone() {
                Adt2::C0(_, _, _, h1133, _, _, _, _, _, _, _) => {
                    h1133
                }
            };
            nat_eq(a1134, a1135)
        } else {
            false
        } {
            let a1138 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1136, _, _, _, _, _, _) => {
                    h1136
                }
            };
            let a1139 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1137, _, _, _, _, _, _) => {
                    h1137
                }
            };
            nat_eq(a1138, a1139)
        } else {
            false
        }
    } else {
        false
    } {
        let m1141 = f82(match v1.clone() {
            Adt2::C0(h1140, _, _, _, _, _, _, _, _, _, _) => {
                h1140
            }
        });
        match m1141 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f14(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h1142) => {
                        h1142
                    }
                }, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1143, _, _, _, _, _, _, _, _, _, _) => {
                        h1143
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f40(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1144, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1144
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1145, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1145
            }
        }
    };
    let a1146 = 0u64;
    let a1147 = v4;
    if if nat_lt(a1146, a1147) {
        let a1148 = v4;
        let a1149 = 2147483648u64;
        nat_lt(a1148, a1149)
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
        let v7: u64 = f89(match v5.clone() {
            Adt2::C0(_, _, h1150, _, _, _, _, _, _, _, _) => {
                h1150
            }
        }, v4)?;
        let v8: u64 = f89(match v5.clone() {
            Adt2::C0(_, _, _, h1151, _, _, _, _, _, _, _) => {
                h1151
            }
        }, v4)?;
        let a1153 = match v5.clone() {
            Adt2::C0(h1152, _, _, _, _, _, _, _, _, _, _) => {
                h1152
            }
        };
        let a1154 = 0u64;
        if if if if if nat_eq(a1153, a1154) {
            let a1156 = match v6.clone() {
                Adt2::C0(h1155, _, _, _, _, _, _, _, _, _, _) => {
                    h1155
                }
            };
            let a1157 = 0u64;
            nat_eq(a1156, a1157)
        } else {
            false
        } {
            let a1159 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1158, _, _, _, _, _, _) => {
                    h1158
                }
            };
            let a1160 = 1u64;
            nat_eq(a1159, a1160)
        } else {
            false
        } {
            let a1163 = match v5.clone() {
                Adt2::C0(_, h1161, _, _, _, _, _, _, _, _, _) => {
                    h1161
                }
            };
            let a1164 = match v6.clone() {
                Adt2::C0(_, h1162, _, _, _, _, _, _, _, _, _) => {
                    h1162
                }
            };
            nat_eq(a1163, a1164)
        } else {
            false
        } {
            if v0 {
                let a1166 = match v5.clone() {
                    Adt2::C0(_, _, h1165, _, _, _, _, _, _, _, _) => {
                        h1165
                    }
                };
                let a1167 = 2147483648u64;
                if if if nat_lt(a1166, a1167) {
                    let a1169 = match v5.clone() {
                        Adt2::C0(_, _, _, h1168, _, _, _, _, _, _, _) => {
                            h1168
                        }
                    };
                    let a1170 = 2147483648u64;
                    nat_lt(a1169, a1170)
                } else {
                    false
                } {
                    let a1173 = {
                        let a1171 = v7;
                        let a1172 = v8;
                        nat_mul(a1171, a1172)?
                    };
                    let a1174 = 2147483648u64;
                    nat_le(a1173, a1174)
                } else {
                    false
                } {
                    f90(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1176 = match v1.clone() {
                    Adt1::C0(h1175, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1175
                    }
                };
                let a1177 = v7;
                if if if if nat_eq(a1176, a1177) {
                    let a1179 = match v1 {
                        Adt1::C0(_, h1178, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1178
                        }
                    };
                    let a1180 = v8;
                    nat_eq(a1179, a1180)
                } else {
                    false
                } {
                    f91(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1182 = match v6.clone() {
                        Adt2::C0(_, _, h1181, _, _, _, _, _, _, _, _) => {
                            h1181
                        }
                    };
                    let a1183 = v4;
                    if nat_eq(a1182, a1183) {
                        let a1185 = match v6.clone() {
                            Adt2::C0(_, _, _, h1184, _, _, _, _, _, _, _) => {
                                h1184
                            }
                        };
                        let a1186 = v4;
                        nat_eq(a1185, a1186)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1190 = match v6 {
                        Adt2::C0(_, _, _, _, h1187, _, _, _, _, _, _) => {
                            h1187
                        }
                    };
                    let a1191 = {
                        let a1188 = v7;
                        let a1189 = v8;
                        nat_mul(a1188, a1189)?
                    };
                    nat_eq(a1190, a1191)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f41(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a1193 = match v2.clone() {
        Adt2::C0(h1192, _, _, _, _, _, _, _, _, _, _) => {
            h1192
        }
    };
    let a1194 = 0u64;
    if if if if if nat_eq(a1193, a1194) {
        true
    } else {
        let a1196 = match v2.clone() {
            Adt2::C0(h1195, _, _, _, _, _, _, _, _, _, _) => {
                h1195
            }
        };
        let a1197 = 1u64;
        nat_eq(a1196, a1197)
    } {
        true
    } else {
        let a1199 = match v2.clone() {
            Adt2::C0(h1198, _, _, _, _, _, _, _, _, _, _) => {
                h1198
            }
        };
        let a1200 = 30u64;
        nat_eq(a1199, a1200)
    } {
        let a1203 = match v2.clone() {
            Adt2::C0(h1201, _, _, _, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        let a1204 = match v3.clone() {
            Adt2::C0(h1202, _, _, _, _, _, _, _, _, _, _) => {
                h1202
            }
        };
        nat_eq(a1203, a1204)
    } else {
        false
    } {
        if if f19(v2.clone(), v3.clone()) {
            let a1206 = match v1.clone() {
                Adt1::C0(h1205, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1205
                }
            };
            let a1207 = 4294967296u64;
            nat_lt(a1206, a1207)
        } else {
            false
        } {
            let a1209 = match v1.clone() {
                Adt1::C0(_, h1208, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1208
                }
            };
            let a1210 = 4294967296u64;
            nat_lt(a1209, a1210)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f14(v3.clone(), 1u64)?, {
            let a1212 = match v2.clone() {
                Adt2::C0(h1211, _, _, _, _, _, _, _, _, _, _) => {
                    h1211
                }
            };
            let a1213 = 0u64;
            if nat_eq(a1212, a1213) {
                4u64
            } else {
                2u64
            }
        }, f18(v2.clone(), 1u64), f18(v3, 1u64), Adt4::C13({
            let a1214 = v0;
            let a1215 = 31u64;
            if nat_eq(a1214, a1215) {
                0u64
            } else {
                let a1216 = v0;
                let a1217 = 47u64;
                if nat_eq(a1216, a1217) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1218, _, _, _, _, _, _, _, _, _, _) => {
                h1218
            }
        }, match v1.clone() {
            Adt1::C0(h1219, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1219
            }
        }, match v1 {
            Adt1::C0(_, h1220, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1220
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f42(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1222 = match v0.clone() {
        Adt2::C0(h1221, _, _, _, _, _, _, _, _, _, _) => {
            h1221
        }
    };
    let a1223 = 0u64;
    if if if nat_eq(a1222, a1223) {
        let a1225 = match v1.clone() {
            Adt2::C0(h1224, _, _, _, _, _, _, _, _, _, _) => {
                h1224
            }
        };
        let a1226 = 0u64;
        nat_eq(a1225, a1226)
    } else {
        false
    } {
        f19(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f14(v1.clone(), 1u64)?, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f43(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1228 = match v0.clone() {
        Adt2::C0(h1227, _, _, _, _, _, _, _, _, _, _) => {
            h1227
        }
    };
    let a1229 = 0u64;
    if if nat_eq(a1228, a1229) {
        let a1231 = match v1.clone() {
            Adt2::C0(h1230, _, _, _, _, _, _, _, _, _, _) => {
                h1230
            }
        };
        let a1232 = 26u64;
        nat_eq(a1231, a1232)
    } else {
        false
    } {
        let a1234 = match v0.clone() {
            Adt2::C0(_, _, _, h1233, _, _, _, _, _, _, _) => {
                h1233
            }
        };
        let a1235 = 1u64;
        if if if if if if if nat_eq(a1234, a1235) {
            let a1237 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1236, _, _, _, _, _, _) => {
                    h1236
                }
            };
            let a1238 = 1u64;
            nat_eq(a1237, a1238)
        } else {
            false
        } {
            let a1240 = match v0.clone() {
                Adt2::C0(_, h1239, _, _, _, _, _, _, _, _, _) => {
                    h1239
                }
            };
            let a1241 = 2147483647u64;
            nat_le(a1240, a1241)
        } else {
            false
        } {
            let a1244 = match v1.clone() {
                Adt2::C0(_, h1242, _, _, _, _, _, _, _, _, _) => {
                    h1242
                }
            };
            let a1245 = match v0.clone() {
                Adt2::C0(_, _, h1243, _, _, _, _, _, _, _, _) => {
                    h1243
                }
            };
            nat_eq(a1244, a1245)
        } else {
            false
        } {
            let a1247 = match v1.clone() {
                Adt2::C0(_, _, h1246, _, _, _, _, _, _, _, _) => {
                    h1246
                }
            };
            let a1248 = 1u64;
            nat_eq(a1247, a1248)
        } else {
            false
        } {
            let a1250 = match v1.clone() {
                Adt2::C0(_, _, _, h1249, _, _, _, _, _, _, _) => {
                    h1249
                }
            };
            let a1251 = 1u64;
            nat_eq(a1250, a1251)
        } else {
            false
        } {
            let a1253 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1252, _, _, _, _, _, _) => {
                    h1252
                }
            };
            let a1254 = 1u64;
            nat_eq(a1253, a1254)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1257 = match v0.clone() {
                    Adt2::C0(_, _, h1255, _, _, _, _, _, _, _, _) => {
                        h1255
                    }
                };
                let a1258 = f64(match v0.clone() {
                    Adt2::C0(_, h1256, _, _, _, _, _, _, _, _, _) => {
                        h1256
                    }
                })?;
                nat_mul(a1257, a1258)?
            }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f44(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1259 = v1;
    match m1259 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f66(f18(v0.clone(), 1u64))?;
            let a1261 = match v0.clone() {
                Adt2::C0(h1260, _, _, _, _, _, _, _, _, _, _) => {
                    h1260
                }
            };
            let a1262 = 26u64;
            if if if nat_eq(a1261, a1262) {
                let a1264 = match v3.clone() {
                    Adt2::C0(h1263, _, _, _, _, _, _, _, _, _, _) => {
                        h1263
                    }
                };
                let a1265 = 26u64;
                nat_eq(a1264, a1265)
            } else {
                false
            } {
                let a1267 = match v2.clone() {
                    Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
                        h1266
                    }
                };
                let a1268 = 27u64;
                nat_eq(a1267, a1268)
            } else {
                false
            } {
                if if if if if if f19(v0.clone(), v3.clone()) {
                    let a1270 = match v2.clone() {
                        Adt2::C0(_, h1269, _, _, _, _, _, _, _, _, _) => {
                            h1269
                        }
                    };
                    let a1271 = 1u64;
                    nat_eq(a1270, a1271)
                } else {
                    false
                } {
                    let a1273 = match v2.clone() {
                        Adt2::C0(_, _, h1272, _, _, _, _, _, _, _, _) => {
                            h1272
                        }
                    };
                    let a1274 = 1u64;
                    nat_eq(a1273, a1274)
                } else {
                    false
                } {
                    let a1276 = match v2.clone() {
                        Adt2::C0(_, _, _, h1275, _, _, _, _, _, _, _) => {
                            h1275
                        }
                    };
                    let a1277 = 1u64;
                    nat_eq(a1276, a1277)
                } else {
                    false
                } {
                    let a1279 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1278, _, _, _, _, _, _) => {
                            h1278
                        }
                    };
                    let a1280 = 1u64;
                    nat_eq(a1279, a1280)
                } else {
                    false
                } {
                    let a1281 = v4;
                    let a1282 = 9223372036854775808u64;
                    nat_lt(a1281, a1282)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1285 = 2u64;
                        let a1286 = {
                            let a1283 = v4;
                            let a1284 = 2u64;
                            nat_mul(a1283, a1284)?
                        };
                        nat_add(a1285, a1286)?
                    }, 8u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C18(v4, f18(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f45(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1287, _, _, _, _, _, _, _, _, _, _) => {
            h1287
        }
    };
    let a1288 = v3;
    let a1289 = 0u64;
    if if nat_eq(a1288, a1289) {
        true
    } else {
        let a1290 = v0;
        let a1291 = 13u64;
        if nat_eq(a1290, a1291) {
            let a1292 = v3;
            let a1293 = 1u64;
            if nat_eq(a1292, a1293) {
                true
            } else {
                let a1294 = v3;
                let a1295 = 30u64;
                nat_eq(a1294, a1295)
            }
        } else {
            false
        }
    } {
        let a1297 = v3;
        let a1298 = match v2.clone() {
            Adt2::C0(h1296, _, _, _, _, _, _, _, _, _, _) => {
                h1296
            }
        };
        if if nat_eq(a1297, a1298) {
            let a1299 = v0;
            let a1300 = 13u64;
            if nat_eq(a1299, a1300) {
                let a1302 = match v2.clone() {
                    Adt2::C0(_, h1301, _, _, _, _, _, _, _, _, _) => {
                        h1301
                    }
                };
                let a1303 = 1u64;
                if if if nat_eq(a1302, a1303) {
                    let a1305 = match v2.clone() {
                        Adt2::C0(_, _, h1304, _, _, _, _, _, _, _, _) => {
                            h1304
                        }
                    };
                    let a1306 = 1u64;
                    nat_eq(a1305, a1306)
                } else {
                    false
                } {
                    let a1308 = match v2.clone() {
                        Adt2::C0(_, _, _, h1307, _, _, _, _, _, _, _) => {
                            h1307
                        }
                    };
                    let a1309 = 1u64;
                    nat_eq(a1308, a1309)
                } else {
                    false
                } {
                    let a1311 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1310, _, _, _, _, _, _) => {
                            h1310
                        }
                    };
                    let a1312 = 1u64;
                    nat_eq(a1311, a1312)
                } else {
                    false
                }
            } else {
                let a1314 = match v2.clone() {
                    Adt2::C0(_, h1313, _, _, _, _, _, _, _, _, _) => {
                        h1313
                    }
                };
                let a1315 = 1u64;
                if if if nat_eq(a1314, a1315) {
                    let a1318 = match v1.clone() {
                        Adt2::C0(_, _, h1316, _, _, _, _, _, _, _, _) => {
                            h1316
                        }
                    };
                    let a1319 = match v2.clone() {
                        Adt2::C0(_, _, h1317, _, _, _, _, _, _, _, _) => {
                            h1317
                        }
                    };
                    nat_eq(a1318, a1319)
                } else {
                    false
                } {
                    let a1322 = match v1.clone() {
                        Adt2::C0(_, _, _, h1320, _, _, _, _, _, _, _) => {
                            h1320
                        }
                    };
                    let a1323 = match v2.clone() {
                        Adt2::C0(_, _, _, h1321, _, _, _, _, _, _, _) => {
                            h1321
                        }
                    };
                    nat_eq(a1322, a1323)
                } else {
                    false
                } {
                    let a1326 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1324, _, _, _, _, _, _) => {
                            h1324
                        }
                    };
                    let a1327 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1325, _, _, _, _, _, _) => {
                            h1325
                        }
                    };
                    nat_eq(a1326, a1327)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f18(v1, 1u64);
            Ok(Adt6::C4(f92(v4.clone(), v0, v3)?, {
                let a1328 = v3;
                let a1329 = 0u64;
                if nat_eq(a1328, a1329) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f18(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f46(v0: u64, v1: u64) -> Option<u64> {
    let a1330 = v0;
    let a1331 = 8u64;
    if nat_eq(a1330, a1331) {
        Some(5u64)
    } else {
        let a1332 = v0;
        let a1333 = 9u64;
        if nat_eq(a1332, a1333) {
            Some(6u64)
        } else {
            let a1334 = v0;
            let a1335 = 89u64;
            if nat_eq(a1334, a1335) {
                let a1336 = v1;
                let a1337 = 0u64;
                if nat_eq(a1336, a1337) {
                    Some(0u64)
                } else {
                    let a1338 = v1;
                    let a1339 = 1u64;
                    if nat_eq(a1338, a1339) {
                        Some(1u64)
                    } else {
                        let a1340 = v1;
                        let a1341 = 2u64;
                        if nat_eq(a1340, a1341) {
                            Some(2u64)
                        } else {
                            let a1342 = v1;
                            let a1343 = 3u64;
                            if nat_eq(a1342, a1343) {
                                Some(3u64)
                            } else {
                                let a1344 = v1;
                                let a1345 = 6u64;
                                if nat_eq(a1344, a1345) {
                                    Some(4u64)
                                } else {
                                    let a1346 = v1;
                                    let a1347 = 12u64;
                                    if nat_eq(a1346, a1347) {
                                        Some(7u64)
                                    } else {
                                        let a1348 = v1;
                                        let a1349 = 11u64;
                                        if nat_eq(a1348, a1349) {
                                            Some(8u64)
                                        } else {
                                            let a1350 = v1;
                                            let a1351 = 18u64;
                                            if nat_eq(a1350, a1351) {
                                                Some(9u64)
                                            } else {
                                                let a1352 = v1;
                                                let a1353 = 19u64;
                                                if nat_eq(a1352, a1353) {
                                                    Some(10u64)
                                                } else {
                                                    let a1354 = v1;
                                                    let a1355 = 20u64;
                                                    if nat_eq(a1354, a1355) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1356 = v1;
                                                        let a1357 = 21u64;
                                                        if nat_eq(a1356, a1357) {
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

pub fn f47(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1358 = f93(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1358 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f18(v2.clone(), 1u64), f18(v4, 1u64), Adt4::C10(v5, v6, {
                let m1359 = v3;
                match m1359 {
                    None => {
                        f18(v2, 1u64)
                    }
                    Some(v12) => {
                        f18(v12, 1u64)
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

pub fn f48(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1361 = match v1.clone() {
        Adt2::C0(h1360, _, _, _, _, _, _, _, _, _, _) => {
            h1360
        }
    };
    let a1362 = 0u64;
    if if if if if nat_eq(a1361, a1362) {
        true
    } else {
        let a1364 = match v1.clone() {
            Adt2::C0(h1363, _, _, _, _, _, _, _, _, _, _) => {
                h1363
            }
        };
        let a1365 = 1u64;
        nat_eq(a1364, a1365)
    } {
        true
    } else {
        let a1367 = match v1.clone() {
            Adt2::C0(h1366, _, _, _, _, _, _, _, _, _, _) => {
                h1366
            }
        };
        let a1368 = 30u64;
        nat_eq(a1367, a1368)
    } {
        let a1371 = match v1.clone() {
            Adt2::C0(h1369, _, _, _, _, _, _, _, _, _, _) => {
                h1369
            }
        };
        let a1372 = match v2.clone() {
            Adt2::C0(h1370, _, _, _, _, _, _, _, _, _, _) => {
                h1370
            }
        };
        if nat_eq(a1371, a1372) {
            true
        } else {
            let a1374 = match v2.clone() {
                Adt2::C0(h1373, _, _, _, _, _, _, _, _, _, _) => {
                    h1373
                }
            };
            let a1375 = 0u64;
            nat_eq(a1374, a1375)
        }
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f14(v2.clone(), 1u64)?, {
            let a1377 = match v2.clone() {
                Adt2::C0(h1376, _, _, _, _, _, _, _, _, _, _) => {
                    h1376
                }
            };
            let a1378 = 0u64;
            if nat_eq(a1377, a1378) {
                4u64
            } else {
                2u64
            }
        }, f18(v1.clone(), 1u64), f18(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1379, _, _, _, _, _, _, _, _, _, _) => {
                h1379
            }
        }, match v2 {
            Adt2::C0(h1380, _, _, _, _, _, _, _, _, _, _) => {
                h1380
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f49(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1382 = v0;
        let a1383 = match v1.clone() {
            Adt3::C0(h1381, _, _, _, _, _, _, _) => {
                h1381
            }
        };
        let a1384 = 0u64;
        nat_quot(a1382, a1383, a1384)
    };
    let v3: u64 = {
        let a1386 = v2;
        let a1387 = match v1.clone() {
            Adt3::C0(_, h1385, _, _, _, _, _, _) => {
                h1385
            }
        };
        let a1388 = 0u64;
        nat_quot(a1386, a1387, a1388)
    };
    f94({
        let a1390 = v0;
        let a1391 = match v1.clone() {
            Adt3::C0(h1389, _, _, _, _, _, _, _) => {
                h1389
            }
        };
        let a1392 = 0u64;
        nat_rem(a1390, a1391, a1392)
    }, {
        let a1394 = v2;
        let a1395 = match v1.clone() {
            Adt3::C0(_, h1393, _, _, _, _, _, _) => {
                h1393
            }
        };
        let a1396 = 0u64;
        nat_rem(a1394, a1395, a1396)
    }, {
        let a1398 = v3;
        let a1399 = match v1.clone() {
            Adt3::C0(_, _, h1397, _, _, _, _, _) => {
                h1397
            }
        };
        let a1400 = 0u64;
        nat_rem(a1398, a1399, a1400)
    }, {
        let a1402 = v3;
        let a1403 = match v1.clone() {
            Adt3::C0(_, _, h1401, _, _, _, _, _) => {
                h1401
            }
        };
        let a1404 = 0u64;
        nat_quot(a1402, a1403, a1404)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1405, _, _, _) => {
            h1405
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1406, _, _) => {
            h1406
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1407, _) => {
            h1407
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1408) => {
            h1408
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1426 = {
        let a1421 = f95(v2, {
            let a1412 = {
                let a1409 = v0;
                let a1410 = v1;
                let a1411 = 0u64;
                nat_quot(a1409, a1410, a1411)
            };
            let a1413 = v2;
            let a1414 = 0u64;
            nat_rem(a1412, a1413, a1414)
        }, {
            let a1418 = {
                let a1415 = v0;
                let a1416 = v1;
                let a1417 = 0u64;
                nat_quot(a1415, a1416, a1417)
            };
            let a1419 = v2;
            let a1420 = 0u64;
            nat_quot(a1418, a1419, a1420)
        })?;
        let a1422 = v1;
        nat_mul(a1421, a1422)?
    };
    let a1427 = {
        let a1423 = v0;
        let a1424 = v1;
        let a1425 = 0u64;
        nat_rem(a1423, a1424, a1425)
    };
    nat_add(a1426, a1427)
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1429 = v0;
        let a1430 = match v2.clone() {
            Adt3::C0(h1428, _, _, _, _, _, _, _) => {
                h1428
            }
        };
        let a1431 = 0u64;
        nat_quot(a1429, a1430, a1431)
    };
    let v4: u64 = {
        let a1433 = v3;
        let a1434 = match v2.clone() {
            Adt3::C0(_, h1432, _, _, _, _, _, _) => {
                h1432
            }
        };
        let a1435 = 0u64;
        nat_quot(a1433, a1434, a1435)
    };
    f96({
        let a1437 = v0;
        let a1438 = match v2.clone() {
            Adt3::C0(h1436, _, _, _, _, _, _, _) => {
                h1436
            }
        };
        let a1439 = 0u64;
        nat_rem(a1437, a1438, a1439)
    }, {
        let a1441 = v3;
        let a1442 = match v2.clone() {
            Adt3::C0(_, h1440, _, _, _, _, _, _) => {
                h1440
            }
        };
        let a1443 = 0u64;
        nat_rem(a1441, a1442, a1443)
    }, {
        let a1445 = v4;
        let a1446 = match v2.clone() {
            Adt3::C0(_, _, h1444, _, _, _, _, _) => {
                h1444
            }
        };
        let a1447 = 0u64;
        nat_rem(a1445, a1446, a1447)
    }, {
        let a1449 = v4;
        let a1450 = match v2 {
            Adt3::C0(_, _, h1448, _, _, _, _, _) => {
                h1448
            }
        };
        let a1451 = 0u64;
        nat_quot(a1449, a1450, a1451)
    }, match v1.clone() {
        Adt3::C0(h1452, _, _, _, _, _, _, _) => {
            h1452
        }
    }, match v1.clone() {
        Adt3::C0(_, h1453, _, _, _, _, _, _) => {
            h1453
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1454, _, _, _, _, _) => {
            h1454
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1455, _, _, _, _) => {
            h1455
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1456, _, _, _) => {
            h1456
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1457, _, _) => {
            h1457
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1458, _) => {
            h1458
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1459) => {
            h1459
        }
    })
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1461 = v0;
        let a1462 = match v3.clone() {
            Adt3::C0(h1460, _, _, _, _, _, _, _) => {
                h1460
            }
        };
        let a1463 = 0u64;
        nat_quot(a1461, a1462, a1463)
    };
    let v6: u64 = {
        let a1465 = v5;
        let a1466 = match v3.clone() {
            Adt3::C0(_, h1464, _, _, _, _, _, _) => {
                h1464
            }
        };
        let a1467 = 0u64;
        nat_quot(a1465, a1466, a1467)
    };
    let v7: u64 = {
        let a1469 = v0;
        let a1470 = match v3.clone() {
            Adt3::C0(h1468, _, _, _, _, _, _, _) => {
                h1468
            }
        };
        let a1471 = 0u64;
        nat_rem(a1469, a1470, a1471)
    };
    let v8: u64 = {
        let a1473 = v5;
        let a1474 = match v3.clone() {
            Adt3::C0(_, h1472, _, _, _, _, _, _) => {
                h1472
            }
        };
        let a1475 = 0u64;
        nat_rem(a1473, a1474, a1475)
    };
    let v9: u64 = {
        let a1477 = v6;
        let a1478 = match v3.clone() {
            Adt3::C0(_, _, h1476, _, _, _, _, _) => {
                h1476
            }
        };
        let a1479 = 0u64;
        nat_rem(a1477, a1478, a1479)
    };
    let v10: u64 = {
        let a1481 = v6;
        let a1482 = match v3.clone() {
            Adt3::C0(_, _, h1480, _, _, _, _, _) => {
                h1480
            }
        };
        let a1483 = 0u64;
        nat_quot(a1481, a1482, a1483)
    };
    let v11: u64 = {
        let a1484 = v4;
        let a1485 = 0u64;
        if nat_eq(a1484, a1485) {
            match v1.clone() {
                Adt3::C0(h1486, _, _, _, _, _, _, _) => {
                    h1486
                }
            }
        } else {
            let a1487 = v4;
            let a1488 = 1u64;
            if nat_eq(a1487, a1488) {
                match v1.clone() {
                    Adt3::C0(_, h1489, _, _, _, _, _, _) => {
                        h1489
                    }
                }
            } else {
                let a1490 = v4;
                let a1491 = 2u64;
                if nat_eq(a1490, a1491) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1492, _, _, _, _, _) => {
                            h1492
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1493, _, _, _, _) => {
                            h1493
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1500 = v11;
        let a1501 = {
            let a1494 = v4;
            let a1495 = 0u64;
            if nat_eq(a1494, a1495) {
                v7
            } else {
                let a1496 = v4;
                let a1497 = 1u64;
                if nat_eq(a1496, a1497) {
                    v8
                } else {
                    let a1498 = v4;
                    let a1499 = 2u64;
                    if nat_eq(a1498, a1499) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1500, a1501)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f94(if if v12 {
        let a1502 = v4;
        let a1503 = 0u64;
        nat_eq(a1502, a1503)
    } else {
        false
    } {
        let a1504 = v7;
        let a1505 = v11;
        nat_sub(a1504, a1505)
    } else {
        v7
    }, if if v12 {
        let a1506 = v4;
        let a1507 = 1u64;
        nat_eq(a1506, a1507)
    } else {
        false
    } {
        let a1508 = v8;
        let a1509 = v11;
        nat_sub(a1508, a1509)
    } else {
        v8
    }, if if v12 {
        let a1510 = v4;
        let a1511 = 2u64;
        nat_eq(a1510, a1511)
    } else {
        false
    } {
        let a1512 = v9;
        let a1513 = v11;
        nat_sub(a1512, a1513)
    } else {
        v9
    }, if if v12 {
        let a1514 = v4;
        let a1515 = 3u64;
        nat_eq(a1514, a1515)
    } else {
        false
    } {
        let a1516 = v10;
        let a1517 = v11;
        nat_sub(a1516, a1517)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1518, _, _, _) => {
            h1518
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1519, _, _) => {
            h1519
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1520, _) => {
            h1520
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1521) => {
            h1521
        }
    })?, f49(v0, v3)?))
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f97(v0, v2.clone(), 0u64);
    let v9: u64 = f97(v0, v2.clone(), 1u64);
    let v10: u64 = f97(v0, v2.clone(), 2u64);
    let v11: u64 = f97(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f98(v1.clone(), f99(v8, v3, match v1.clone() {
            Adt3::C0(h1522, _, _, _, _, _, _, _) => {
                h1522
            }
        })?, f99(v9, v4, match v1.clone() {
            Adt3::C0(_, h1523, _, _, _, _, _, _) => {
                h1523
            }
        })?, f99(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1524, _, _, _, _, _) => {
                h1524
            }
        })?, f99(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1525, _, _, _, _) => {
                h1525
            }
        })?)?, f49(v0, v2)?))
    } else {
        if if if if f100(v8, v3, match v1.clone() {
            Adt3::C0(h1526, _, _, _, _, _, _, _) => {
                h1526
            }
        })? {
            f100(v9, v4, match v1.clone() {
                Adt3::C0(_, h1527, _, _, _, _, _, _) => {
                    h1527
                }
            })?
        } else {
            false
        } {
            f100(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1528, _, _, _, _, _) => {
                    h1528
                }
            })?
        } else {
            false
        } {
            f100(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1529, _, _, _, _) => {
                    h1529
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f98(v1, f101(v8, v3)?, f101(v9, v4)?, f101(v10, v5)?, f101(v11, v6)?)?, f49(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f49(v0, v2)?))
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f97(v0, v2.clone(), 0u64);
    let v5: u64 = f97(v0, v2.clone(), 1u64);
    let v6: u64 = f97(v0, v2.clone(), 2u64);
    let v7: u64 = f97(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f98(v1.clone(), f102(v4, v3, match v1 {
        Adt3::C0(h1530, _, _, _, _, _, _, _) => {
            h1530
        }
    })?, v5, v6, v7)?, f49(v0, v2)?))
}

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1531 = v0;
    let a1532 = v3;
    if nat_lt(a1531, a1532) {
        Ok(Adt5::C1(false, {
            let a1533 = v0;
            let a1534 = 4u64;
            nat_mul(a1533, a1534)?
        }, {
            let a1535 = v0;
            let a1536 = 4u64;
            nat_mul(a1535, a1536)?
        }))
    } else {
        let v6: u64 = {
            let a1537 = v0;
            let a1538 = v3;
            nat_sub(a1537, a1538)
        };
        let v7: u64 = f49(v6, v1)?;
        let v8: u64 = f103(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1539 = v0;
    let a1540 = v3;
    if nat_lt(a1539, a1540) {
        Ok(Adt5::C1(true, 0u64, f49(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1541 = v0;
            let a1542 = v3;
            nat_sub(a1541, a1542)
        };
        let v5: u64 = f51(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f49(v4, v1)?, 3u64, v5))
    }
}

pub fn f57(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1544 = v1;
        let a1545 = match v3.clone() {
            Adt3::C0(h1543, _, _, _, _, _, _, _) => {
                h1543
            }
        };
        let a1546 = 0u64;
        nat_rem(a1544, a1545, a1546)
    };
    let v5: u64 = {
        let a1552 = {
            let a1548 = v1;
            let a1549 = match v3.clone() {
                Adt3::C0(h1547, _, _, _, _, _, _, _) => {
                    h1547
                }
            };
            let a1550 = 0u64;
            nat_quot(a1548, a1549, a1550)
        };
        let a1553 = match v3.clone() {
            Adt3::C0(_, h1551, _, _, _, _, _, _) => {
                h1551
            }
        };
        let a1554 = 0u64;
        nat_rem(a1552, a1553, a1554)
    };
    let v6: bool = f104(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f51(v1, v2, v3.clone())?
    } else {
        0u64
    }, f49(v1, v3)?))
}

pub fn f58(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1555 = v3;
        let a1556 = 8u64;
        nat_add(a1555, a1556)?
    }, 0u64, f49(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f51(v0, v5, v2.clone())?
    }, 3u64, f49(v0, v2)?))
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1558 = v0;
        let a1559 = match v2.clone() {
            Adt3::C0(h1557, _, _, _, _, _, _, _) => {
                h1557
            }
        };
        let a1560 = 0u64;
        nat_rem(a1558, a1559, a1560)
    };
    let v7: u64 = {
        let a1562 = v0;
        let a1563 = match v2.clone() {
            Adt3::C0(h1561, _, _, _, _, _, _, _) => {
                h1561
            }
        };
        let a1564 = 0u64;
        nat_quot(a1562, a1563, a1564)
    };
    let v8: u64 = {
        let a1566 = v7;
        let a1567 = match v2.clone() {
            Adt3::C0(_, h1565, _, _, _, _, _, _) => {
                h1565
            }
        };
        let a1568 = 0u64;
        nat_rem(a1566, a1567, a1568)
    };
    let v9: u64 = {
        let a1570 = v7;
        let a1571 = match v2.clone() {
            Adt3::C0(_, h1569, _, _, _, _, _, _) => {
                h1569
            }
        };
        let a1572 = 0u64;
        nat_quot(a1570, a1571, a1572)
    };
    let v10: u64 = {
        let a1574 = v9;
        let a1575 = match v2.clone() {
            Adt3::C0(_, _, h1573, _, _, _, _, _) => {
                h1573
            }
        };
        let a1576 = 0u64;
        nat_rem(a1574, a1575, a1576)
    };
    let v11: u64 = {
        let a1578 = v9;
        let a1579 = match v2.clone() {
            Adt3::C0(_, _, h1577, _, _, _, _, _) => {
                h1577
            }
        };
        let a1580 = 0u64;
        nat_quot(a1578, a1579, a1580)
    };
    let v12: u64 = f49(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1589 = {
                let a1584 = {
                    let a1581 = v10;
                    let a1582 = v4;
                    let a1583 = 0u64;
                    nat_quot(a1581, a1582, a1583)
                };
                let a1585 = v5;
                nat_mul(a1584, a1585)?
            };
            let a1590 = {
                let a1586 = v8;
                let a1587 = v4;
                let a1588 = 0u64;
                nat_quot(a1586, a1587, a1588)
            };
            nat_add(a1589, a1590)?
        };
        let v14: u64 = {
            let a1610 = v6;
            let a1611 = {
                let a1608 = match v1.clone() {
                    Adt3::C0(h1591, _, _, _, _, _, _, _) => {
                        h1591
                    }
                };
                let a1609 = {
                    let a1606 = {
                        let a1592 = v8;
                        let a1593 = v4;
                        let a1594 = 0u64;
                        nat_rem(a1592, a1593, a1594)
                    };
                    let a1607 = {
                        let a1604 = match v1.clone() {
                            Adt3::C0(_, h1595, _, _, _, _, _, _) => {
                                h1595
                            }
                        };
                        let a1605 = {
                            let a1602 = {
                                let a1596 = v10;
                                let a1597 = v4;
                                let a1598 = 0u64;
                                nat_rem(a1596, a1597, a1598)
                            };
                            let a1603 = {
                                let a1600 = match v1.clone() {
                                    Adt3::C0(_, _, h1599, _, _, _, _, _) => {
                                        h1599
                                    }
                                };
                                let a1601 = v13;
                                nat_mul(a1600, a1601)?
                            };
                            nat_add(a1602, a1603)?
                        };
                        nat_mul(a1604, a1605)?
                    };
                    nat_add(a1606, a1607)?
                };
                nat_mul(a1608, a1609)?
            };
            nat_add(a1610, a1611)?
        };
        Ok(Adt5::C1(false, f49(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1617 = {
                let a1615 = {
                    let a1612 = v11;
                    let a1613 = v5;
                    let a1614 = 0u64;
                    nat_rem(a1612, a1613, a1614)
                };
                let a1616 = v4;
                nat_mul(a1615, a1616)?
            };
            let a1618 = v8;
            nat_add(a1617, a1618)?
        };
        let v16: u64 = {
            let a1624 = {
                let a1622 = {
                    let a1619 = v11;
                    let a1620 = v5;
                    let a1621 = 0u64;
                    nat_quot(a1619, a1620, a1621)
                };
                let a1623 = v4;
                nat_mul(a1622, a1623)?
            };
            let a1625 = v10;
            nat_add(a1624, a1625)?
        };
        let a1627 = v15;
        let a1628 = match v1.clone() {
            Adt3::C0(_, h1626, _, _, _, _, _, _) => {
                h1626
            }
        };
        if if nat_lt(a1627, a1628) {
            let a1630 = v16;
            let a1631 = match v1.clone() {
                Adt3::C0(_, _, h1629, _, _, _, _, _) => {
                    h1629
                }
            };
            nat_lt(a1630, a1631)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f49({
                let a1640 = v6;
                let a1641 = {
                    let a1638 = match v1.clone() {
                        Adt3::C0(h1632, _, _, _, _, _, _, _) => {
                            h1632
                        }
                    };
                    let a1639 = {
                        let a1636 = v15;
                        let a1637 = {
                            let a1634 = match v1.clone() {
                                Adt3::C0(_, h1633, _, _, _, _, _, _) => {
                                    h1633
                                }
                            };
                            let a1635 = v16;
                            nat_mul(a1634, a1635)?
                        };
                        nat_add(a1636, a1637)?
                    };
                    nat_mul(a1638, a1639)?
                };
                nat_add(a1640, a1641)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f49(v0, v2.clone())?;
    let v4: u64 = f49(v0, v1.clone())?;
    let a1646 = {
        let a1643 = v0;
        let a1644 = match v1 {
            Adt3::C0(h1642, _, _, _, _, _, _, _) => {
                h1642
            }
        };
        let a1645 = 0u64;
        nat_rem(a1643, a1644, a1645)
    };
    let a1647 = 0u64;
    if nat_eq(a1646, a1647) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f49({
            let a1648 = v0;
            let a1649 = 1u64;
            nat_sub(a1648, a1649)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1650 = v3;
    let a1651 = 13u64;
    if nat_eq(a1650, a1651) {
        f105(v0, v1, v4)
    } else {
        let v5: u64 = f106(match v1.clone() {
            Adt3::C0(h1652, _, _, _, _, _, _, _) => {
                h1652
            }
        }, {
            let a1653 = v3;
            let a1654 = 16u64;
            nat_eq(a1653, a1654)
        })?;
        let v6: u64 = {
            let a1655 = v0;
            let a1656 = v5;
            let a1657 = 0u64;
            nat_quot(a1655, a1656, a1657)
        };
        let v7: u64 = {
            let a1658 = v0;
            let a1659 = v5;
            let a1660 = 0u64;
            nat_rem(a1658, a1659, a1660)
        };
        let v8: u64 = {
            let a1664 = 1u64;
            let a1665 = {
                let a1662 = match v1.clone() {
                    Adt3::C0(h1661, _, _, _, _, _, _, _) => {
                        h1661
                    }
                };
                let a1663 = 2u64;
                nat_mul(a1662, a1663)?
            };
            nat_add(a1664, a1665)?
        };
        let a1666 = v7;
        let a1667 = v8;
        if nat_lt(a1666, a1667) {
            f107(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f49(v6, v2)?;
            let a1668 = v3;
            let a1669 = 14u64;
            if nat_eq(a1668, a1669) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1670 = v7;
                let a1671 = v8;
                if nat_eq(a1670, a1671) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1674 = v7;
                    let a1675 = {
                        let a1672 = v8;
                        let a1673 = 1u64;
                        nat_add(a1672, a1673)?
                    };
                    if nat_eq(a1674, a1675) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1676, _, _, _, _, _, _, _) => {
                                h1676
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

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1677 = v0;
    let a1678 = 0u64;
    if nat_eq(a1677, a1678) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1685 = {
            let a1679 = v0;
            let a1680 = 1u64;
            nat_add(a1679, a1680)?
        };
        let a1686 = {
            let a1683 = 2u64;
            let a1684 = {
                let a1681 = v3;
                let a1682 = 2u64;
                nat_mul(a1681, a1682)?
            };
            nat_add(a1683, a1684)?
        };
        if nat_eq(a1685, a1686) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1687 = v0;
                let a1688 = 1u64;
                nat_sub(a1687, a1688)
            };
            let v5: u64 = {
                let a1689 = v4;
                let a1690 = 2u64;
                let a1691 = 0u64;
                nat_quot(a1689, a1690, a1691)
            };
            let a1695 = {
                let a1692 = v4;
                let a1693 = 2u64;
                let a1694 = 0u64;
                nat_rem(a1692, a1693, a1694)
            };
            let a1696 = 0u64;
            if nat_eq(a1695, a1696) {
                Ok(Adt5::C4(17u64, 0u64, f49(v5, v1)?, 4u64, f49(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a1697 = v0;
    let a1698 = f23(v1, v2, v3)?;
    if nat_lt(a1697, a1698) {
        let a1699 = v0;
        let a1700 = v1;
        if nat_lt(a1699, a1700) {
            Ok(Adt5::C1(false, {
                let a1701 = 4u64;
                let a1702 = v0;
                nat_mul(a1701, a1702)?
            }, {
                let a1703 = 4u64;
                let a1704 = v0;
                nat_mul(a1703, a1704)?
            }))
        } else {
            let v4: u64 = {
                let a1705 = v0;
                let a1706 = v1;
                nat_sub(a1705, a1706)
            };
            let v5: u64 = f108(v4, v2)?;
            let a1707 = v5;
            let a1708 = 2u64;
            if nat_lt(a1707, a1708) {
                Ok(Adt5::C4(6u64, {
                    let a1709 = v5;
                    let a1710 = 0u64;
                    if nat_eq(a1709, a1710) {
                        4u64
                    } else {
                        5u64
                    }
                }, f109(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f110(v4, v2)?;
                Ok(Adt5::C4(8u64, 3u64, v6, 1u64, if f111(v4, v2)? {
                    1u64
                } else {
                    0u64
                }, 3u64, v6))
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f64(v0: u64) -> R<u64> {
    let a1713 = 3u64;
    let a1714 = {
        let a1711 = v0;
        let a1712 = 3u64;
        nat_mul(a1711, a1712)?
    };
    nat_add(a1713, a1714)
}

pub fn f65(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1722 = {
        let a1715 = 2u64;
        let a1716 = f66(v1.clone())?;
        nat_mul(a1715, a1716)?
    };
    let a1723 = {
        let a1720 = f66(v1)?;
        let a1721 = {
            let a1718 = 1u64;
            let a1719 = match v0 {
                Adt3::C0(h1717, _, _, _, _, _, _, _) => {
                    h1717
                }
            };
            nat_add(a1718, a1719)?
        };
        nat_mul(a1720, a1721)?
    };
    nat_add(a1722, a1723)
}

pub fn f66(v0: Adt3) -> R<u64> {
    if f79(match v0.clone() {
        Adt3::C0(h1724, _, _, _, _, _, _, _) => {
            h1724
        }
    }, match v0.clone() {
        Adt3::C0(_, h1725, _, _, _, _, _, _) => {
            h1725
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1726, _, _, _, _, _) => {
            h1726
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1727, _, _, _, _) => {
            h1727
        }
    }) {
        Ok(0u64)
    } else {
        let a1736 = match v0.clone() {
            Adt3::C0(h1728, _, _, _, _, _, _, _) => {
                h1728
            }
        };
        let a1737 = {
            let a1734 = match v0.clone() {
                Adt3::C0(_, h1729, _, _, _, _, _, _) => {
                    h1729
                }
            };
            let a1735 = {
                let a1732 = match v0.clone() {
                    Adt3::C0(_, _, h1730, _, _, _, _, _) => {
                        h1730
                    }
                };
                let a1733 = match v0 {
                    Adt3::C0(_, _, _, h1731, _, _, _, _) => {
                        h1731
                    }
                };
                nat_mul(a1732, a1733)?
            };
            nat_mul(a1734, a1735)?
        };
        nat_mul(a1736, a1737)
    }
}

pub fn f67(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f49(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f68(v0: u64, v1: u64) -> bool {
    let a1738 = v1;
    let a1739 = 2147483647u64;
    if nat_le(a1738, a1739) {
        let a1740 = v1;
        let a1741 = v0;
        nat_lt(a1740, a1741)
    } else {
        false
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1742 = f82(v0);
    match m1742 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a1744 = v2;
                let a1745 = match v7.clone() {
                    Adt11::C0(_, h1743, _) => {
                        h1743
                    }
                };
                nat_mul(a1744, a1745)?
            };
            let m1758 = f112(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1746, _, _, _, _, _, _, _) => {
                    h1746
                }
            }, match v5.clone() {
                Adt3::C0(_, h1747, _, _, _, _, _, _) => {
                    h1747
                }
            }, match v5 {
                Adt3::C0(_, _, h1748, _, _, _, _, _) => {
                    h1748
                }
            }, {
                let a1751 = match v3.clone() {
                    Adt3::C0(h1749, _, _, _, _, _, _, _) => {
                        h1749
                    }
                };
                let a1752 = match v7.clone() {
                    Adt11::C0(_, h1750, _) => {
                        h1750
                    }
                };
                nat_mul(a1751, a1752)?
            }, match v3.clone() {
                Adt3::C0(_, h1753, _, _, _, _, _, _) => {
                    h1753
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1754, _, _, _) => {
                    h1754
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1755, _, _) => {
                    h1755
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1756, _) => {
                    h1756
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1757) => {
                    h1757
                }
            })?;
            match m1758 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1759 = v0;
                    let a1760 = 0u64;
                    if if nat_eq(a1759, a1760) {
                        true
                    } else {
                        let a1761 = v0;
                        let a1762 = 26u64;
                        nat_eq(a1761, a1762)
                    } {
                        Ok(Adt5::C1(false, v9, f49({
                            let a1766 = {
                                let a1764 = v1;
                                let a1765 = match v4.clone() {
                                    Adt3::C0(h1763, _, _, _, _, _, _, _) => {
                                        h1763
                                    }
                                };
                                nat_mul(a1764, a1765)?
                            };
                            let a1767 = v8;
                            nat_add(a1766, a1767)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h1768) => {
                                h1768
                            }
                        }, f49({
                            let a1772 = {
                                let a1770 = v1;
                                let a1771 = match v4.clone() {
                                    Adt3::C0(h1769, _, _, _, _, _, _, _) => {
                                        h1769
                                    }
                                };
                                nat_mul(a1770, a1771)?
                            };
                            let a1773 = v8;
                            nat_add(a1772, a1773)?
                        }, v4.clone())?, {
                            let a1780 = {
                                let a1778 = {
                                    let a1775 = match v7 {
                                        Adt11::C0(_, h1774, _) => {
                                            h1774
                                        }
                                    };
                                    let a1776 = 1u64;
                                    nat_sub(a1775, a1776)
                                };
                                let a1779 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1777, _, _, _) => {
                                        h1777
                                    }
                                };
                                nat_mul(a1778, a1779)?
                            };
                            let a1781 = 4u64;
                            nat_add(a1780, a1781)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1782, _, _, _) => {
                                h1782
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1783 = v1;
    let a1784 = 0u64;
    if if nat_eq(a1783, a1784) {
        true
    } else {
        let a1785 = v2;
        let a1786 = 0u64;
        nat_eq(a1785, a1786)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f71(v0: u64, v1: u64) -> bool {
    let a1787 = v0;
    let a1788 = 4294967295u64;
    if nat_eq(a1787, a1788) {
        true
    } else {
        let a1789 = v0;
        let a1790 = 2147483648u64;
        if nat_lt(a1789, a1790) {
            let a1791 = v0;
            let a1792 = v1;
            nat_lt(a1791, a1792)
        } else {
            false
        }
    }
}

pub fn f72(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a1793 = v0;
        let a1794 = 8388608u64;
        let a1795 = 0u64;
        nat_quot(a1793, a1794, a1795)
    };
    let a1796 = v0;
    let a1797 = 2147483648u64;
    if if if nat_lt(a1796, a1797) {
        let a1798 = 127u64;
        let a1799 = v1;
        nat_le(a1798, a1799)
    } else {
        false
    } {
        let a1800 = v1;
        let a1801 = 190u64;
        nat_lt(a1800, a1801)
    } else {
        false
    } {
        let v2: u64 = {
            let a1805 = 8388608u64;
            let a1806 = {
                let a1802 = v0;
                let a1803 = 8388608u64;
                let a1804 = 0u64;
                nat_rem(a1802, a1803, a1804)
            };
            nat_add(a1805, a1806)?
        };
        let a1807 = v1;
        let a1808 = 150u64;
        if nat_lt(a1807, a1808) {
            let v3: u64 = f113({
                let a1809 = 150u64;
                let a1810 = v1;
                nat_sub(a1809, a1810)
            });
            let a1814 = {
                let a1811 = v2;
                let a1812 = v3;
                let a1813 = 0u64;
                nat_rem(a1811, a1812, a1813)
            };
            let a1815 = 0u64;
            if nat_eq(a1814, a1815) {
                let a1816 = v2;
                let a1817 = v3;
                let a1818 = 0u64;
                Ok(nat_quot(a1816, a1817, a1818))
            } else {
                Ok(0u64)
            }
        } else {
            let a1821 = f113({
                let a1819 = v1;
                let a1820 = 150u64;
                nat_sub(a1819, a1820)
            });
            let a1822 = v2;
            nat_mul(a1821, a1822)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f73(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1823 = v1;
    match m1823 {
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
            f114(f115(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f116(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f117(v0.clone(), v16);
            f114(f118(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
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

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = f77(match v1.clone() {
        Adt3::C0(h1824, _, _, _, _, _, _, _) => {
            h1824
        }
    })?;
    let v6: u64 = {
        let a1830 = match v1.clone() {
            Adt3::C0(_, h1825, _, _, _, _, _, _) => {
                h1825
            }
        };
        let a1831 = {
            let a1828 = match v1.clone() {
                Adt3::C0(_, _, h1826, _, _, _, _, _) => {
                    h1826
                }
            };
            let a1829 = match v1.clone() {
                Adt3::C0(_, _, _, h1827, _, _, _, _) => {
                    h1827
                }
            };
            nat_mul(a1828, a1829)?
        };
        nat_mul(a1830, a1831)?
    };
    let a1833 = v0;
    let a1834 = f26(match v1.clone() {
        Adt3::C0(h1832, _, _, _, _, _, _, _) => {
            h1832
        }
    }, v6)?;
    if nat_lt(a1833, a1834) {
        f119({
            let a1835 = v0;
            let a1836 = v5;
            let a1837 = 0u64;
            nat_rem(a1835, a1836, a1837)
        }, {
            let a1838 = v0;
            let a1839 = v5;
            let a1840 = 0u64;
            nat_quot(a1838, a1839, a1840)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f75(v0: u64) -> u64 {
    let v1: u64 = {
        let a1841 = v0;
        let a1842 = 4294967296u64;
        let a1843 = 0u64;
        nat_rem(a1841, a1842, a1843)
    };
    let a1844 = v1;
    let a1845 = 2147483648u64;
    if nat_lt(a1844, a1845) {
        v1
    } else {
        0u64
    }
}

pub fn f76(v0: u64) -> R<u64> {
    let a1848 = 2u64;
    let a1849 = {
        let a1846 = 2u64;
        let a1847 = v0;
        nat_mul(a1846, a1847)?
    };
    nat_add(a1848, a1849)
}

pub fn f77(v0: u64) -> R<u64> {
    let a1852 = 5u64;
    let a1853 = {
        let a1850 = 4u64;
        let a1851 = v0;
        nat_mul(a1850, a1851)?
    };
    nat_add(a1852, a1853)
}

pub fn f78(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f120(match v0.clone() {
        Adt2::C0(h1854, _, _, _, _, _, _, _, _, _, _) => {
            h1854
        }
    }, match v0.clone() {
        Adt2::C0(_, h1855, _, _, _, _, _, _, _, _, _) => {
            h1855
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1856, _, _, _, _, _, _, _, _) => {
            h1856
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1857, _, _, _, _, _, _, _) => {
            h1857
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1858, _, _, _, _, _, _) => {
            h1858
        }
    }, match v1.clone() {
        Adt2::C0(h1859, _, _, _, _, _, _, _, _, _, _) => {
            h1859
        }
    }, match v1.clone() {
        Adt2::C0(_, h1860, _, _, _, _, _, _, _, _, _) => {
            h1860
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1861, _, _, _, _, _, _, _, _) => {
            h1861
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1862, _, _, _, _, _, _, _) => {
            h1862
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1863, _, _, _, _, _, _) => {
            h1863
        }
    }, match v2.clone() {
        Adt2::C0(h1864, _, _, _, _, _, _, _, _, _, _) => {
            h1864
        }
    }, match v2.clone() {
        Adt2::C0(_, h1865, _, _, _, _, _, _, _, _, _) => {
            h1865
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1866, _, _, _, _, _, _, _, _) => {
            h1866
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1867, _, _, _, _, _, _, _) => {
            h1867
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1868, _, _, _, _, _, _) => {
            h1868
        }
    }) {
        let m1870 = f82(match v0.clone() {
            Adt2::C0(h1869, _, _, _, _, _, _, _, _, _, _) => {
                h1869
            }
        });
        match m1870 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1876 = {
                    let a1873 = match v0.clone() {
                        Adt2::C0(_, h1871, _, _, _, _, _, _, _, _, _) => {
                            h1871
                        }
                    };
                    let a1874 = match v3.clone() {
                        Adt11::C0(_, h1872, _) => {
                            h1872
                        }
                    };
                    let a1875 = 0u64;
                    nat_rem(a1873, a1874, a1875)
                };
                let a1877 = 0u64;
                if if nat_eq(a1876, a1877) {
                    if f121(match v0.clone() {
                        Adt2::C0(_, h1878, _, _, _, _, _, _, _, _, _) => {
                            h1878
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1879, _, _, _, _, _, _, _, _, _) => {
                            h1879
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1880, _, _, _, _, _, _, _, _) => {
                            h1880
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1881, _, _, _, _, _, _, _) => {
                            h1881
                        }
                    }) {
                        true
                    } else {
                        let a1883 = 4u64;
                        let a1884 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1882, _, _, _, _, _) => {
                                h1882
                            }
                        };
                        if if nat_le(a1883, a1884) {
                            let a1886 = match v3.clone() {
                                Adt11::C0(_, h1885, _) => {
                                    h1885
                                }
                            };
                            let a1887 = 1u64;
                            if nat_eq(a1886, a1887) {
                                true
                            } else {
                                let a1890 = match v3.clone() {
                                    Adt11::C0(_, _, h1888) => {
                                        h1888
                                    }
                                };
                                let a1891 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1889, _, _, _, _, _) => {
                                        h1889
                                    }
                                };
                                nat_le(a1890, a1891)
                            }
                        } else {
                            false
                        } {
                            if f21(v0.clone(), v2.clone())? {
                                f21(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f18(v0.clone(), match v3 {
                        Adt11::C0(_, h1892, _) => {
                            h1892
                        }
                    });
                    Ok(Adt6::C4(if f121(match v0.clone() {
                        Adt2::C0(_, h1893, _, _, _, _, _, _, _, _, _) => {
                            h1893
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1894, _, _, _, _, _, _, _, _, _) => {
                            h1894
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1895, _, _, _, _, _, _, _, _) => {
                            h1895
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1896, _, _, _, _, _, _, _) => {
                            h1896
                        }
                    }) {
                        0u64
                    } else {
                        f65(v4.clone(), f18(v1.clone(), 1u64))?
                    }, 4u64, v4, f18(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1897, _, _, _, _, _, _, _, _, _, _) => {
                            h1897
                        }
                    }, f18(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1898 = v0;
    let a1899 = 0u64;
    if if if nat_eq(a1898, a1899) {
        true
    } else {
        let a1900 = v1;
        let a1901 = 0u64;
        nat_eq(a1900, a1901)
    } {
        true
    } else {
        let a1902 = v2;
        let a1903 = 0u64;
        nat_eq(a1902, a1903)
    } {
        true
    } else {
        let a1904 = v3;
        let a1905 = 0u64;
        nat_eq(a1904, a1905)
    }
}

pub fn f80(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1908 = match v1.clone() {
        Adt2::C0(h1906, _, _, _, _, _, _, _, _, _, _) => {
            h1906
        }
    };
    let a1909 = match v3.clone() {
        Adt2::C0(h1907, _, _, _, _, _, _, _, _, _, _) => {
            h1907
        }
    };
    if if nat_eq(a1908, a1909) {
        let a1910 = v0;
        let a1911 = 33u64;
        if nat_eq(a1910, a1911) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1913 = f82(match v1.clone() {
            Adt2::C0(h1912, _, _, _, _, _, _, _, _, _, _) => {
                h1912
            }
        });
        match m1913 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1914, _) => {
                        h1914
                    }
                };
                let v6: u64 = f14(v1.clone(), v5)?;
                let a1919 = {
                    let a1916 = match v1.clone() {
                        Adt2::C0(_, h1915, _, _, _, _, _, _, _, _, _) => {
                            h1915
                        }
                    };
                    let a1917 = v5;
                    let a1918 = 1u64;
                    nat_rem(a1916, a1917, a1918)
                };
                let a1920 = 0u64;
                if if if nat_eq(a1919, a1920) {
                    let a1925 = {
                        let a1922 = match v3.clone() {
                            Adt2::C0(_, h1921, _, _, _, _, _, _, _, _, _) => {
                                h1921
                            }
                        };
                        let a1923 = v5;
                        let a1924 = 1u64;
                        nat_rem(a1922, a1923, a1924)
                    };
                    let a1926 = 0u64;
                    nat_eq(a1925, a1926)
                } else {
                    false
                } {
                    let a1927 = v6;
                    let a1928 = f14(v3.clone(), v5)?;
                    nat_eq(a1927, a1928)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1929) => {
                            h1929
                        }
                    }, f18(v1, v5), f18(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f81(v0: u64) -> bool {
    let a1930 = v0;
    let a1931 = 0u64;
    if if nat_eq(a1930, a1931) {
        true
    } else {
        let a1932 = v0;
        let a1933 = 1u64;
        nat_eq(a1932, a1933)
    } {
        true
    } else {
        let a1934 = v0;
        let a1935 = 30u64;
        nat_eq(a1934, a1935)
    }
}

pub fn f82(v0: u64) -> Adt10 {
    let a1936 = v0;
    let a1937 = 0u64;
    if nat_eq(a1936, a1937) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1938 = v0;
        let a1939 = 1u64;
        if nat_eq(a1938, a1939) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1940 = v0;
            let a1941 = 2u64;
            if nat_eq(a1940, a1941) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1942 = v0;
                let a1943 = 3u64;
                if nat_eq(a1942, a1943) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1944 = v0;
                    let a1945 = 6u64;
                    if nat_eq(a1944, a1945) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1946 = v0;
                        let a1947 = 7u64;
                        if nat_eq(a1946, a1947) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1948 = v0;
                            let a1949 = 8u64;
                            if nat_eq(a1948, a1949) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1950 = v0;
                                let a1951 = 9u64;
                                if nat_eq(a1950, a1951) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1952 = v0;
                                    let a1953 = 10u64;
                                    if nat_eq(a1952, a1953) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1954 = v0;
                                        let a1955 = 11u64;
                                        if nat_eq(a1954, a1955) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1956 = v0;
                                            let a1957 = 12u64;
                                            if nat_eq(a1956, a1957) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1958 = v0;
                                                let a1959 = 13u64;
                                                if nat_eq(a1958, a1959) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1960 = v0;
                                                    let a1961 = 14u64;
                                                    if nat_eq(a1960, a1961) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1962 = v0;
                                                        let a1963 = 15u64;
                                                        if nat_eq(a1962, a1963) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1964 = v0;
                                                            let a1965 = 16u64;
                                                            if nat_eq(a1964, a1965) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1966 = v0;
                                                                let a1967 = 17u64;
                                                                if nat_eq(a1966, a1967) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1968 = v0;
                                                                    let a1969 = 18u64;
                                                                    if nat_eq(a1968, a1969) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1970 = v0;
                                                                        let a1971 = 19u64;
                                                                        if nat_eq(a1970, a1971) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1972 = v0;
                                                                            let a1973 = 20u64;
                                                                            if nat_eq(a1972, a1973) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1974 = v0;
                                                                                let a1975 = 21u64;
                                                                                if nat_eq(a1974, a1975) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1976 = v0;
                                                                                    let a1977 = 22u64;
                                                                                    if nat_eq(a1976, a1977) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1978 = v0;
                                                                                        let a1979 = 23u64;
                                                                                        if nat_eq(a1978, a1979) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1980 = v0;
                                                                                            let a1981 = 24u64;
                                                                                            if nat_eq(a1980, a1981) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1982 = v0;
                                                                                                let a1983 = 25u64;
                                                                                                if nat_eq(a1982, a1983) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1984 = v0;
                                                                                                    let a1985 = 26u64;
                                                                                                    if nat_eq(a1984, a1985) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1986 = v0;
                                                                                                        let a1987 = 27u64;
                                                                                                        if nat_eq(a1986, a1987) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1988 = v0;
                                                                                                            let a1989 = 28u64;
                                                                                                            if nat_eq(a1988, a1989) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1990 = v0;
                                                                                                                let a1991 = 29u64;
                                                                                                                if nat_eq(a1990, a1991) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1992 = v0;
                                                                                                                    let a1993 = 30u64;
                                                                                                                    if nat_eq(a1992, a1993) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1994 = v0;
                                                                                                                        let a1995 = 34u64;
                                                                                                                        if nat_eq(a1994, a1995) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1996 = v0;
                                                                                                                            let a1997 = 35u64;
                                                                                                                            if nat_eq(a1996, a1997) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1998 = v0;
                                                                                                                                let a1999 = 39u64;
                                                                                                                                if nat_eq(a1998, a1999) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2000 = v0;
                                                                                                                                    let a2001 = 40u64;
                                                                                                                                    if nat_eq(a2000, a2001) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2002 = v0;
                                                                                                                                        let a2003 = 41u64;
                                                                                                                                        if nat_eq(a2002, a2003) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2004 = v0;
                                                                                                                                            let a2005 = 42u64;
                                                                                                                                            if nat_eq(a2004, a2005) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2006 = v0;
                                                                                                                                                let a2007 = 43u64;
                                                                                                                                                if nat_eq(a2006, a2007) {
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

pub fn f83(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f79(v4, v5, v6, v7) {
        f79(v0, v1, v2, v3)
    } else {
        let a2011 = {
            let a2008 = v0;
            let a2009 = v4;
            let a2010 = 1u64;
            nat_rem(a2008, a2009, a2010)
        };
        let a2012 = 0u64;
        if if if nat_eq(a2011, a2012) {
            let a2016 = {
                let a2013 = v1;
                let a2014 = v5;
                let a2015 = 1u64;
                nat_rem(a2013, a2014, a2015)
            };
            let a2017 = 0u64;
            nat_eq(a2016, a2017)
        } else {
            false
        } {
            let a2021 = {
                let a2018 = v2;
                let a2019 = v6;
                let a2020 = 1u64;
                nat_rem(a2018, a2019, a2020)
            };
            let a2022 = 0u64;
            nat_eq(a2021, a2022)
        } else {
            false
        } {
            let a2026 = {
                let a2023 = v3;
                let a2024 = v7;
                let a2025 = 1u64;
                nat_rem(a2023, a2024, a2025)
            };
            let a2027 = 0u64;
            nat_eq(a2026, a2027)
        } else {
            false
        }
    }
}

pub fn f84(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2028 = v0;
    let a2029 = v1;
    if nat_eq(a2028, a2029) {
        let a2032 = v4;
        let a2033 = {
            let a2030 = v2;
            let a2031 = v3;
            nat_add(a2030, a2031)?
        };
        Ok(nat_eq(a2032, a2033))
    } else {
        let a2034 = v2;
        let a2035 = v3;
        if nat_eq(a2034, a2035) {
            let a2036 = v4;
            let a2037 = v2;
            Ok(nat_eq(a2036, a2037))
        } else {
            Ok(false)
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2040 = {
            let a2038 = v0;
            let a2039 = f122(v1);
            nat_add(a2038, a2039)?
        };
        let a2041 = f122(v2);
        nat_add(a2040, a2041)?
    };
    let v5: u64 = {
        let a2042 = f123(v1);
        let a2043 = f123(v2);
        nat_add(a2042, a2043)?
    };
    let a2044 = v1;
    let a2045 = 4294967295u64;
    if if nat_le(a2044, a2045) {
        let a2046 = v2;
        let a2047 = 4294967295u64;
        nat_le(a2046, a2047)
    } else {
        false
    } {
        let a2048 = v5;
        let a2049 = v4;
        if nat_le(a2048, a2049) {
            let a2052 = v3;
            let a2053 = {
                let a2050 = v4;
                let a2051 = v5;
                nat_sub(a2050, a2051)
            };
            Ok(nat_eq(a2052, a2053))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f86(v0: u64, v1: u64) -> R<bool> {
    let a2054 = v1;
    let a2055 = 4294967295u64;
    if nat_le(a2054, a2055) {
        let a2058 = {
            let a2056 = f122(v1);
            let a2057 = f123(v1);
            nat_add(a2056, a2057)?
        };
        let a2059 = v0;
        Ok(nat_lt(a2058, a2059))
    } else {
        Ok(false)
    }
}

pub fn f87(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f124(v0.clone(), false)?;
    let v6: u64 = f124(v0, true)?;
    let a2060 = v6;
    let a2061 = v1;
    if if nat_le(a2060, a2061) {
        let a2068 = {
            let a2066 = {
                let a2062 = v1;
                let a2063 = v5;
                nat_add(a2062, a2063)?
            };
            let a2067 = {
                let a2064 = v3;
                let a2065 = 0u64;
                if nat_eq(a2064, a2065) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2066, a2067)?
        };
        let a2069 = v2;
        nat_le(a2068, a2069)
    } else {
        false
    } {
        if v4 {
            let a2074 = {
                let a2072 = {
                    let a2070 = v1;
                    let a2071 = v5;
                    nat_add(a2070, a2071)?
                };
                let a2073 = v6;
                nat_sub(a2072, a2073)
            };
            let a2075 = v2;
            Ok(nat_lt(a2074, a2075))
        } else {
            let a2080 = {
                let a2078 = {
                    let a2076 = v1;
                    let a2077 = v5;
                    nat_add(a2076, a2077)?
                };
                let a2079 = v6;
                nat_sub(a2078, a2079)
            };
            let a2081 = v2;
            Ok(nat_le(a2080, a2081))
        }
    } else {
        Ok(false)
    }
}

pub fn f88(v0: u64, v1: u64) -> bool {
    let a2082 = v0;
    let a2083 = 0u64;
    if if nat_eq(a2082, a2083) {
        true
    } else {
        let a2084 = v0;
        let a2085 = 1u64;
        nat_eq(a2084, a2085)
    } {
        let a2086 = v1;
        let a2087 = 4294967296u64;
        nat_lt(a2086, a2087)
    } else {
        false
    }
}

pub fn f89(v0: u64, v1: u64) -> R<u64> {
    let a2092 = {
        let a2090 = v0;
        let a2091 = {
            let a2088 = v1;
            let a2089 = 1u64;
            nat_sub(a2088, a2089)
        };
        nat_add(a2090, a2091)?
    };
    let a2093 = v1;
    let a2094 = 0u64;
    Ok(nat_quot(a2092, a2093, a2094))
}

pub fn f90(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f79(match v0.clone() {
        Adt2::C0(_, h2095, _, _, _, _, _, _, _, _, _) => {
            h2095
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2096, _, _, _, _, _, _, _, _) => {
            h2096
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2097, _, _, _, _, _, _, _) => {
            h2097
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2098, _, _, _, _, _, _) => {
            h2098
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2101 = match v1.clone() {
                Adt2::C0(_, h2099, _, _, _, _, _, _, _, _, _) => {
                    h2099
                }
            };
            let a2102 = match v1.clone() {
                Adt2::C0(_, _, h2100, _, _, _, _, _, _, _, _) => {
                    h2100
                }
            };
            nat_mul(a2101, a2102)?
        };
        let v5: u64 = {
            let a2104 = v4;
            let a2105 = match v1.clone() {
                Adt2::C0(_, _, _, h2103, _, _, _, _, _, _, _) => {
                    h2103
                }
            };
            nat_mul(a2104, a2105)?
        };
        let a2118 = {
            let a2116 = {
                let a2107 = match v0.clone() {
                    Adt2::C0(_, h2106, _, _, _, _, _, _, _, _, _) => {
                        h2106
                    }
                };
                let a2108 = 1u64;
                nat_sub(a2107, a2108)
            };
            let a2117 = {
                let a2114 = f125(match v0.clone() {
                    Adt2::C0(_, _, h2109, _, _, _, _, _, _, _, _) => {
                        h2109
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2110, _, _, _, _, _, _, _, _, _) => {
                        h2110
                    }
                })?;
                let a2115 = f125(match v0 {
                    Adt2::C0(_, _, _, h2111, _, _, _, _, _, _, _) => {
                        h2111
                    }
                }, v2, {
                    let a2112 = v3;
                    let a2113 = v5;
                    nat_mul(a2112, a2113)?
                }, v4)?;
                nat_add(a2114, a2115)?
            };
            nat_add(a2116, a2117)?
        };
        let a2119 = f14(v1, 1u64)?;
        Ok(nat_lt(a2118, a2119))
    }
}

pub fn f91(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2120 = v0;
    let a2121 = 2147483648u64;
    if if if if nat_lt(a2120, a2121) {
        let a2122 = v1;
        let a2123 = 2147483648u64;
        nat_lt(a2122, a2123)
    } else {
        false
    } {
        let a2126 = {
            let a2124 = v0;
            let a2125 = v1;
            nat_mul(a2124, a2125)?
        };
        let a2127 = 2147483648u64;
        nat_lt(a2126, a2127)
    } else {
        false
    } {
        let a2132 = {
            let a2130 = {
                let a2128 = v0;
                let a2129 = 1u64;
                nat_sub(a2128, a2129)
            };
            let a2131 = v2;
            nat_mul(a2130, a2131)?
        };
        let a2133 = 2147483648u64;
        nat_lt(a2132, a2133)
    } else {
        false
    } {
        let a2138 = {
            let a2136 = {
                let a2134 = v1;
                let a2135 = 1u64;
                nat_sub(a2134, a2135)
            };
            let a2137 = v2;
            nat_mul(a2136, a2137)?
        };
        let a2139 = 2147483648u64;
        Ok(nat_lt(a2138, a2139))
    } else {
        Ok(false)
    }
}

pub fn f92(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2140 = v1;
    let a2141 = 13u64;
    if if nat_eq(a2140, a2141) {
        f79(match v0.clone() {
            Adt3::C0(h2142, _, _, _, _, _, _, _) => {
                h2142
            }
        }, match v0.clone() {
            Adt3::C0(_, h2143, _, _, _, _, _, _) => {
                h2143
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2144, _, _, _, _, _) => {
                h2144
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2145, _, _, _, _) => {
                h2145
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2151 = match v0.clone() {
                Adt3::C0(_, h2146, _, _, _, _, _, _) => {
                    h2146
                }
            };
            let a2152 = {
                let a2149 = match v0.clone() {
                    Adt3::C0(_, _, h2147, _, _, _, _, _) => {
                        h2147
                    }
                };
                let a2150 = match v0.clone() {
                    Adt3::C0(_, _, _, h2148, _, _, _, _) => {
                        h2148
                    }
                };
                nat_mul(a2149, a2150)?
            };
            nat_mul(a2151, a2152)?
        };
        let a2153 = v1;
        let a2154 = 13u64;
        if nat_eq(a2153, a2154) {
            let a2163 = 2u64;
            let a2164 = if f126(v0.clone(), v2) {
                let a2156 = v3;
                let a2157 = f106(match v0 {
                    Adt3::C0(h2155, _, _, _, _, _, _, _) => {
                        h2155
                    }
                }, false)?;
                nat_mul(a2156, a2157)?
            } else {
                let a2161 = v3;
                let a2162 = {
                    let a2159 = match v0 {
                        Adt3::C0(h2158, _, _, _, _, _, _, _) => {
                            h2158
                        }
                    };
                    let a2160 = 2u64;
                    nat_mul(a2159, a2160)?
                };
                nat_mul(a2161, a2162)?
            };
            nat_add(a2163, a2164)
        } else {
            let a2168 = v3;
            let a2169 = f106(match v0 {
                Adt3::C0(h2165, _, _, _, _, _, _, _) => {
                    h2165
                }
            }, {
                let a2166 = v1;
                let a2167 = 16u64;
                nat_eq(a2166, a2167)
            })?;
            nat_mul(a2168, a2169)
        }
    }
}

pub fn f93(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m2170 = v3;
    match m2170 {
        None => {
            f127(v0, v1, match v2.clone() {
                Adt2::C0(h2171, _, _, _, _, _, _, _, _, _, _) => {
                    h2171
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h2172, _, _, _, _, _, _, _, _, _, _) => {
                    h2172
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h2173, _, _, _, _, _, _, _, _, _) => {
                    h2173
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2174, _, _, _, _, _, _, _, _) => {
                    h2174
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2175, _, _, _, _, _, _, _) => {
                    h2175
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2176, _, _, _, _, _, _) => {
                    h2176
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h2177, _, _, _, _, _, _, _, _, _) => {
                    h2177
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2178, _, _, _, _, _, _, _, _) => {
                    h2178
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2179, _, _, _, _, _, _, _) => {
                    h2179
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2180, _, _, _, _, _, _) => {
                    h2180
                }
            })
        }
        Some(v5) => {
            f127(v0, v1, match v2.clone() {
                Adt2::C0(h2181, _, _, _, _, _, _, _, _, _, _) => {
                    h2181
                }
            }, match v5.clone() {
                Adt2::C0(h2182, _, _, _, _, _, _, _, _, _, _) => {
                    h2182
                }
            }, match v4.clone() {
                Adt2::C0(h2183, _, _, _, _, _, _, _, _, _, _) => {
                    h2183
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h2184, _, _, _, _, _, _, _, _, _) => {
                    h2184
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2185, _, _, _, _, _, _, _, _) => {
                    h2185
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2186, _, _, _, _, _, _, _) => {
                    h2186
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2187, _, _, _, _, _, _) => {
                    h2187
                }
            }, match v5.clone() {
                Adt2::C0(_, h2188, _, _, _, _, _, _, _, _, _) => {
                    h2188
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h2189, _, _, _, _, _, _, _, _) => {
                    h2189
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h2190, _, _, _, _, _, _, _) => {
                    h2190
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h2191, _, _, _, _, _, _) => {
                    h2191
                }
            }, match v4.clone() {
                Adt2::C0(_, h2192, _, _, _, _, _, _, _, _, _) => {
                    h2192
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2193, _, _, _, _, _, _, _, _) => {
                    h2193
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2194, _, _, _, _, _, _, _) => {
                    h2194
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2195, _, _, _, _, _, _) => {
                    h2195
                }
            })
        }
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2208 = {
        let a2204 = {
            let a2200 = {
                let a2196 = v0;
                let a2197 = v4;
                nat_mul(a2196, a2197)?
            };
            let a2201 = {
                let a2198 = v1;
                let a2199 = v5;
                nat_mul(a2198, a2199)?
            };
            nat_add(a2200, a2201)?
        };
        let a2205 = {
            let a2202 = v2;
            let a2203 = v6;
            nat_mul(a2202, a2203)?
        };
        nat_add(a2204, a2205)?
    };
    let a2209 = {
        let a2206 = v3;
        let a2207 = v7;
        nat_mul(a2206, a2207)?
    };
    nat_add(a2208, a2209)
}

pub fn f95(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2214 = {
        let a2212 = {
            let a2210 = v0;
            let a2211 = v1;
            nat_sub(a2210, a2211)
        };
        let a2213 = 1u64;
        nat_sub(a2212, a2213)
    };
    let a2215 = v2;
    nat_add(a2214, a2215)
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f94({
        let a2216 = v0;
        let a2217 = v4;
        let a2218 = 0u64;
        nat_rem(a2216, a2217, a2218)
    }, {
        let a2219 = v1;
        let a2220 = v5;
        let a2221 = 0u64;
        nat_rem(a2219, a2220, a2221)
    }, {
        let a2222 = v2;
        let a2223 = v6;
        let a2224 = 0u64;
        nat_rem(a2222, a2223, a2224)
    }, {
        let a2225 = v3;
        let a2226 = v7;
        let a2227 = 0u64;
        nat_rem(a2225, a2226, a2227)
    }, v8, v9, v10, v11)
}

pub fn f97(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2229 = v0;
        let a2230 = match v1.clone() {
            Adt3::C0(h2228, _, _, _, _, _, _, _) => {
                h2228
            }
        };
        let a2231 = 0u64;
        nat_quot(a2229, a2230, a2231)
    };
    let v4: u64 = {
        let a2233 = v3;
        let a2234 = match v1.clone() {
            Adt3::C0(_, h2232, _, _, _, _, _, _) => {
                h2232
            }
        };
        let a2235 = 0u64;
        nat_quot(a2233, a2234, a2235)
    };
    let a2236 = v2;
    let a2237 = 0u64;
    if nat_eq(a2236, a2237) {
        let a2239 = v0;
        let a2240 = match v1 {
            Adt3::C0(h2238, _, _, _, _, _, _, _) => {
                h2238
            }
        };
        let a2241 = 0u64;
        nat_rem(a2239, a2240, a2241)
    } else {
        let a2242 = v2;
        let a2243 = 1u64;
        if nat_eq(a2242, a2243) {
            let a2245 = v3;
            let a2246 = match v1 {
                Adt3::C0(_, h2244, _, _, _, _, _, _) => {
                    h2244
                }
            };
            let a2247 = 0u64;
            nat_rem(a2245, a2246, a2247)
        } else {
            let a2248 = v2;
            let a2249 = 2u64;
            if nat_eq(a2248, a2249) {
                let a2251 = v4;
                let a2252 = match v1 {
                    Adt3::C0(_, _, h2250, _, _, _, _, _) => {
                        h2250
                    }
                };
                let a2253 = 0u64;
                nat_rem(a2251, a2252, a2253)
            } else {
                let a2255 = v4;
                let a2256 = match v1 {
                    Adt3::C0(_, _, h2254, _, _, _, _, _) => {
                        h2254
                    }
                };
                let a2257 = 0u64;
                nat_quot(a2255, a2256, a2257)
            }
        }
    }
}

pub fn f98(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f94(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2258, _, _, _) => {
            h2258
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2259, _, _) => {
            h2259
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2260, _) => {
            h2260
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2261) => {
            h2261
        }
    })
}

pub fn f99(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2262 = v1;
    let a2263 = 2147483648u64;
    if nat_lt(a2262, a2263) {
        let a2274 = {
            let a2272 = {
                let a2267 = {
                    let a2264 = v0;
                    let a2265 = v2;
                    let a2266 = 0u64;
                    nat_rem(a2264, a2265, a2266)
                };
                let a2268 = v2;
                nat_add(a2267, a2268)?
            };
            let a2273 = {
                let a2269 = v1;
                let a2270 = v2;
                let a2271 = 0u64;
                nat_rem(a2269, a2270, a2271)
            };
            nat_sub(a2272, a2273)
        };
        let a2275 = v2;
        let a2276 = 0u64;
        Ok(nat_rem(a2274, a2275, a2276))
    } else {
        let a2279 = {
            let a2277 = v0;
            let a2278 = f123(v1);
            nat_add(a2277, a2278)?
        };
        let a2280 = v2;
        let a2281 = 0u64;
        Ok(nat_rem(a2279, a2280, a2281))
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2284 = f122(v1);
    let a2285 = {
        let a2282 = v0;
        let a2283 = f123(v1);
        nat_add(a2282, a2283)?
    };
    if nat_le(a2284, a2285) {
        let a2286 = f101(v0, v1)?;
        let a2287 = v2;
        Ok(nat_lt(a2286, a2287))
    } else {
        Ok(false)
    }
}

pub fn f101(v0: u64, v1: u64) -> R<u64> {
    let a2290 = {
        let a2288 = v0;
        let a2289 = f123(v1);
        nat_add(a2288, a2289)?
    };
    let a2291 = f122(v1);
    Ok(nat_sub(a2290, a2291))
}

pub fn f102(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2292 = v0;
    let a2293 = v1;
    if nat_lt(a2292, a2293) {
        let a2294 = v1;
        let a2295 = v0;
        Ok(nat_sub(a2294, a2295))
    } else {
        let a2298 = v0;
        let a2299 = {
            let a2296 = v1;
            let a2297 = v2;
            nat_add(a2296, a2297)?
        };
        if nat_lt(a2298, a2299) {
            let a2300 = v0;
            let a2301 = v1;
            Ok(nat_sub(a2300, a2301))
        } else {
            let a2308 = {
                let a2304 = 2u64;
                let a2305 = {
                    let a2302 = v2;
                    let a2303 = 1u64;
                    nat_sub(a2302, a2303)
                };
                nat_mul(a2304, a2305)?
            };
            let a2309 = {
                let a2306 = v0;
                let a2307 = v1;
                nat_sub(a2306, a2307)
            };
            Ok(nat_sub(a2308, a2309))
        }
    }
}

pub fn f103(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2312 = {
        let a2310 = v2;
        let a2311 = f128(v1.clone(), f97(v0, v1.clone(), 0u64), f97(v0, v1.clone(), 1u64), f97(v0, v1.clone(), 2u64), f97(v0, v1.clone(), 3u64), false)?;
        nat_add(a2310, a2311)?
    };
    let a2313 = f128(v1.clone(), f97(v0, v1.clone(), 0u64), f97(v0, v1.clone(), 1u64), f97(v0, v1.clone(), 2u64), f97(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2312, a2313))
}

pub fn f104(v0: u64, v1: u64, v2: u64) -> bool {
    let a2314 = v0;
    let a2315 = 0u64;
    if nat_eq(a2314, a2315) {
        let a2316 = v2;
        let a2317 = v1;
        nat_le(a2316, a2317)
    } else {
        let a2318 = v0;
        let a2319 = 1u64;
        if nat_eq(a2318, a2319) {
            let a2320 = v2;
            let a2321 = v1;
            nat_lt(a2320, a2321)
        } else {
            let a2322 = v0;
            let a2323 = 2u64;
            if nat_eq(a2322, a2323) {
                let a2324 = v1;
                let a2325 = v2;
                nat_le(a2324, a2325)
            } else {
                let a2326 = v0;
                let a2327 = 3u64;
                if nat_eq(a2326, a2327) {
                    let a2328 = v1;
                    let a2329 = v2;
                    nat_lt(a2328, a2329)
                } else {
                    let a2330 = v1;
                    let a2331 = v2;
                    nat_eq(a2330, a2331)
                }
            }
        }
    }
}

pub fn f105(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2332 = v0;
    let a2333 = 0u64;
    if nat_eq(a2332, a2333) {
        Ok(f129(0u64))
    } else {
        let a2336 = {
            let a2334 = v0;
            let a2335 = 1u64;
            nat_add(a2334, a2335)?
        };
        let a2337 = f92(v1.clone(), 13u64, v2)?;
        if nat_eq(a2336, a2337) {
            Ok(Adt5::C4({
                let a2338 = v2;
                let a2339 = 0u64;
                if nat_eq(a2338, a2339) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2340 = v0;
                let a2341 = 1u64;
                nat_sub(a2340, a2341)
            };
            if f126(v1.clone(), v2) {
                let v4: u64 = f106(match v1.clone() {
                    Adt3::C0(h2342, _, _, _, _, _, _, _) => {
                        h2342
                    }
                }, false)?;
                let v5: u64 = {
                    let a2343 = v3;
                    let a2344 = v4;
                    let a2345 = 0u64;
                    nat_quot(a2343, a2344, a2345)
                };
                let v6: u64 = {
                    let a2346 = v3;
                    let a2347 = v4;
                    let a2348 = 0u64;
                    nat_rem(a2346, a2347, a2348)
                };
                let a2351 = {
                    let a2349 = v6;
                    let a2350 = 1u64;
                    nat_add(a2349, a2350)?
                };
                let a2352 = v4;
                if nat_eq(a2351, a2352) {
                    Ok(Adt5::C4({
                        let a2353 = v2;
                        let a2354 = 0u64;
                        if nat_eq(a2353, a2354) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f107(v6, v5, v1, v2)
                }
            } else {
                f130({
                    let a2355 = v3;
                    let a2356 = 2u64;
                    let a2357 = 0u64;
                    nat_quot(a2355, a2356, a2357)
                }, {
                    let a2358 = v3;
                    let a2359 = 2u64;
                    let a2360 = 0u64;
                    nat_rem(a2358, a2359, a2360)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f106(v0: u64, v1: bool) -> R<u64> {
    let a2363 = {
        let a2361 = v0;
        let a2362 = 2u64;
        nat_mul(a2361, a2362)?
    };
    let a2364 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2363, a2364)
}

pub fn f107(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2365 = v0;
    let a2366 = 0u64;
    if nat_eq(a2365, a2366) {
        Ok(f129(1u64))
    } else {
        f130({
            let a2375 = {
                let a2368 = v1;
                let a2369 = match v2.clone() {
                    Adt3::C0(h2367, _, _, _, _, _, _, _) => {
                        h2367
                    }
                };
                nat_mul(a2368, a2369)?
            };
            let a2376 = {
                let a2372 = {
                    let a2370 = v0;
                    let a2371 = 1u64;
                    nat_sub(a2370, a2371)
                };
                let a2373 = 2u64;
                let a2374 = 0u64;
                nat_quot(a2372, a2373, a2374)
            };
            nat_add(a2375, a2376)?
        }, {
            let a2379 = {
                let a2377 = v0;
                let a2378 = 1u64;
                nat_sub(a2377, a2378)
            };
            let a2380 = 2u64;
            let a2381 = 0u64;
            nat_rem(a2379, a2380, a2381)
        }, v2, v3, 1u64)
    }
}

pub fn f108(v0: u64, v1: u64) -> R<u64> {
    let a2382 = v0;
    let a2383 = f76(v1)?;
    let a2384 = 0u64;
    Ok(nat_rem(a2382, a2383, a2384))
}

pub fn f109(v0: u64, v1: u64) -> R<u64> {
    let a2385 = 4u64;
    let a2386 = f131(v0, v1)?;
    nat_mul(a2385, a2386)
}

pub fn f110(v0: u64, v1: u64) -> R<u64> {
    let a2387 = 4u64;
    let a2388 = f132(v0, v1)?;
    nat_mul(a2387, a2388)
}

pub fn f111(v0: u64, v1: u64) -> R<bool> {
    let a2394 = {
        let a2391 = {
            let a2389 = f108(v0, v1)?;
            let a2390 = 2u64;
            nat_sub(a2389, a2390)
        };
        let a2392 = 2u64;
        let a2393 = 0u64;
        nat_rem(a2391, a2392, a2393)
    };
    let a2395 = 0u64;
    Ok(nat_eq(a2394, a2395))
}

pub fn f112(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2396 = f82(v0);
    match m2396 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f133(v0) {
                let a2397 = v1;
                let a2398 = v7;
                nat_lt(a2397, a2398)
            } else {
                false
            } {
                f68(v8, v2)
            } else {
                false
            } {
                let a2399 = v3;
                let a2400 = f134(v4, v5, v6)?;
                nat_lt(a2399, a2400)
            } else {
                false
            } {
                let a2405 = {
                    let a2402 = v7;
                    let a2403 = match v13.clone() {
                        Adt11::C0(_, h2401, _) => {
                            h2401
                        }
                    };
                    let a2404 = 0u64;
                    nat_rem(a2402, a2403, a2404)
                };
                let a2406 = 0u64;
                nat_eq(a2405, a2406)
            } else {
                false
            } {
                Ok(Some({
                    let a2435 = {
                        let a2425 = {
                            let a2415 = {
                                let a2411 = {
                                    let a2408 = v1;
                                    let a2409 = match v13 {
                                        Adt11::C0(_, h2407, _) => {
                                            h2407
                                        }
                                    };
                                    let a2410 = 0u64;
                                    nat_quot(a2408, a2409, a2410)
                                };
                                let a2412 = v9;
                                nat_mul(a2411, a2412)?
                            };
                            let a2416 = {
                                let a2413 = v2;
                                let a2414 = v10;
                                nat_mul(a2413, a2414)?
                            };
                            nat_add(a2415, a2416)?
                        };
                        let a2426 = {
                            let a2423 = {
                                let a2420 = {
                                    let a2417 = v3;
                                    let a2418 = v4;
                                    let a2419 = 0u64;
                                    nat_quot(a2417, a2418, a2419)
                                };
                                let a2421 = v5;
                                let a2422 = 0u64;
                                nat_rem(a2420, a2421, a2422)
                            };
                            let a2424 = v11;
                            nat_mul(a2423, a2424)?
                        };
                        nat_add(a2425, a2426)?
                    };
                    let a2436 = {
                        let a2433 = {
                            let a2430 = {
                                let a2427 = v3;
                                let a2428 = v4;
                                let a2429 = 0u64;
                                nat_quot(a2427, a2428, a2429)
                            };
                            let a2431 = v5;
                            let a2432 = 0u64;
                            nat_quot(a2430, a2431, a2432)
                        };
                        let a2434 = v12;
                        nat_mul(a2433, a2434)?
                    };
                    nat_add(a2435, a2436)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f113(v0: u64) -> u64 {
    let a2437 = v0;
    let a2438 = 40u64;
    if nat_lt(a2437, a2438) {
        let a2439 = v0;
        let a2440 = 20u64;
        if nat_lt(a2439, a2440) {
            let a2441 = v0;
            let a2442 = 10u64;
            if nat_lt(a2441, a2442) {
                let a2443 = v0;
                let a2444 = 5u64;
                if nat_lt(a2443, a2444) {
                    let a2445 = v0;
                    let a2446 = 2u64;
                    if nat_lt(a2445, a2446) {
                        let a2447 = v0;
                        let a2448 = 1u64;
                        if nat_lt(a2447, a2448) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2449 = v0;
                        let a2450 = 3u64;
                        if nat_lt(a2449, a2450) {
                            4u64
                        } else {
                            let a2451 = v0;
                            let a2452 = 4u64;
                            if nat_lt(a2451, a2452) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2453 = v0;
                    let a2454 = 7u64;
                    if nat_lt(a2453, a2454) {
                        let a2455 = v0;
                        let a2456 = 6u64;
                        if nat_lt(a2455, a2456) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2457 = v0;
                        let a2458 = 8u64;
                        if nat_lt(a2457, a2458) {
                            128u64
                        } else {
                            let a2459 = v0;
                            let a2460 = 9u64;
                            if nat_lt(a2459, a2460) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2461 = v0;
                let a2462 = 15u64;
                if nat_lt(a2461, a2462) {
                    let a2463 = v0;
                    let a2464 = 12u64;
                    if nat_lt(a2463, a2464) {
                        let a2465 = v0;
                        let a2466 = 11u64;
                        if nat_lt(a2465, a2466) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2467 = v0;
                        let a2468 = 13u64;
                        if nat_lt(a2467, a2468) {
                            4096u64
                        } else {
                            let a2469 = v0;
                            let a2470 = 14u64;
                            if nat_lt(a2469, a2470) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2471 = v0;
                    let a2472 = 17u64;
                    if nat_lt(a2471, a2472) {
                        let a2473 = v0;
                        let a2474 = 16u64;
                        if nat_lt(a2473, a2474) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2475 = v0;
                        let a2476 = 18u64;
                        if nat_lt(a2475, a2476) {
                            131072u64
                        } else {
                            let a2477 = v0;
                            let a2478 = 19u64;
                            if nat_lt(a2477, a2478) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2479 = v0;
            let a2480 = 30u64;
            if nat_lt(a2479, a2480) {
                let a2481 = v0;
                let a2482 = 25u64;
                if nat_lt(a2481, a2482) {
                    let a2483 = v0;
                    let a2484 = 22u64;
                    if nat_lt(a2483, a2484) {
                        let a2485 = v0;
                        let a2486 = 21u64;
                        if nat_lt(a2485, a2486) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2487 = v0;
                        let a2488 = 23u64;
                        if nat_lt(a2487, a2488) {
                            4194304u64
                        } else {
                            let a2489 = v0;
                            let a2490 = 24u64;
                            if nat_lt(a2489, a2490) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2491 = v0;
                    let a2492 = 27u64;
                    if nat_lt(a2491, a2492) {
                        let a2493 = v0;
                        let a2494 = 26u64;
                        if nat_lt(a2493, a2494) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2495 = v0;
                        let a2496 = 28u64;
                        if nat_lt(a2495, a2496) {
                            134217728u64
                        } else {
                            let a2497 = v0;
                            let a2498 = 29u64;
                            if nat_lt(a2497, a2498) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2499 = v0;
                let a2500 = 35u64;
                if nat_lt(a2499, a2500) {
                    let a2501 = v0;
                    let a2502 = 32u64;
                    if nat_lt(a2501, a2502) {
                        let a2503 = v0;
                        let a2504 = 31u64;
                        if nat_lt(a2503, a2504) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2505 = v0;
                        let a2506 = 33u64;
                        if nat_lt(a2505, a2506) {
                            4294967296u64
                        } else {
                            let a2507 = v0;
                            let a2508 = 34u64;
                            if nat_lt(a2507, a2508) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2509 = v0;
                    let a2510 = 37u64;
                    if nat_lt(a2509, a2510) {
                        let a2511 = v0;
                        let a2512 = 36u64;
                        if nat_lt(a2511, a2512) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2513 = v0;
                        let a2514 = 38u64;
                        if nat_lt(a2513, a2514) {
                            137438953472u64
                        } else {
                            let a2515 = v0;
                            let a2516 = 39u64;
                            if nat_lt(a2515, a2516) {
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

pub fn f114(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a2517 = v7;
        let a2518 = 6u64;
        if nat_eq(a2517, a2518) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f135(v3, v4), f135(v5, v6), f136(v7, v8), f137(v1), f137(v9))
}

pub fn f115(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m2519 = v0;
    match m2519 {
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
        Adt4::C25(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
    }
}

pub fn f116(v0: Adt4) -> u64 {
    let m2520 = v0;
    match m2520 {
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
        Adt4::C25(_, _, _) => {
            0u64
        }
    }
}

pub fn f117(v0: Adt4, v1: u64) -> u64 {
    let m2521 = v0;
    match m2521 {
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
            let a2522 = v1;
            let a2523 = 6u64;
            if nat_eq(a2522, a2523) {
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
        Adt4::C25(_, _, _) => {
            0u64
        }
    }
}

pub fn f118(v0: Adt4, v1: u64) -> Adt7 {
    let a2524 = v1;
    let a2525 = 4u64;
    if nat_lt(a2524, a2525) {
        Adt7::C0(v1)
    } else {
        let a2526 = v1;
        let a2527 = 4u64;
        if nat_eq(a2526, a2527) {
            Adt7::C1(0u64)
        } else {
            let a2528 = v1;
            let a2529 = 5u64;
            if nat_eq(a2528, a2529) {
                Adt7::C1(3u64)
            } else {
                let a2530 = v1;
                let a2531 = 6u64;
                if nat_eq(a2530, a2531) {
                    Adt7::C2(f117(v0, v1), 0u64, false)
                } else {
                    let a2532 = v1;
                    let a2533 = 7u64;
                    if nat_eq(a2532, a2533) {
                        Adt7::C2(0u64, f117(v0, v1), false)
                    } else {
                        let a2534 = v1;
                        let a2535 = 16u64;
                        if nat_lt(a2534, a2535) {
                            Adt7::C1({
                                let a2536 = v1;
                                let a2537 = 8u64;
                                nat_sub(a2536, a2537)
                            })
                        } else {
                            let a2538 = v1;
                            let a2539 = 20u64;
                            if nat_lt(a2538, a2539) {
                                Adt7::C5({
                                    let a2540 = v1;
                                    let a2541 = 16u64;
                                    nat_sub(a2540, a2541)
                                })
                            } else {
                                let a2542 = v1;
                                let a2543 = 20u64;
                                if nat_eq(a2542, a2543) {
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

pub fn f119(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2544, _, _, _, _, _, _, _) => {
            h2544
        }
    };
    let v7: u64 = {
        let a2547 = 1u64;
        let a2548 = {
            let a2545 = 3u64;
            let a2546 = v6;
            nat_mul(a2545, a2546)?
        };
        nat_add(a2547, a2548)?
    };
    let a2549 = v0;
    let a2550 = 0u64;
    if nat_eq(a2549, a2550) {
        Ok(f129(0u64))
    } else {
        let a2551 = v0;
        let a2552 = v7;
        if nat_lt(a2551, a2552) {
            let v8: u64 = {
                let a2553 = v0;
                let a2554 = 1u64;
                nat_sub(a2553, a2554)
            };
            let v9: u64 = {
                let a2560 = {
                    let a2555 = v1;
                    let a2556 = v6;
                    nat_mul(a2555, a2556)?
                };
                let a2561 = {
                    let a2557 = v8;
                    let a2558 = 3u64;
                    let a2559 = 0u64;
                    nat_quot(a2557, a2558, a2559)
                };
                nat_add(a2560, a2561)?
            };
            let v10: u64 = f49(v9, v2)?;
            let v11: u64 = {
                let a2562 = v8;
                let a2563 = 3u64;
                let a2564 = 0u64;
                nat_rem(a2562, a2563, a2564)
            };
            let a2565 = v11;
            let a2566 = 0u64;
            if nat_eq(a2565, a2566) {
                Ok(Adt5::C4(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a2567 = v11;
                let a2568 = 1u64;
                if nat_eq(a2567, a2568) {
                    Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a2569 = v0;
            let a2570 = v7;
            if nat_eq(a2569, a2570) {
                Ok(Adt5::C4(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2573 = v0;
                let a2574 = {
                    let a2571 = v7;
                    let a2572 = 1u64;
                    nat_add(a2571, a2572)?
                };
                if nat_eq(a2573, a2574) {
                    Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2577 = v0;
                    let a2578 = {
                        let a2575 = v7;
                        let a2576 = 2u64;
                        nat_add(a2575, a2576)?
                    };
                    if nat_eq(a2577, a2578) {
                        Ok(Adt5::C4(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a2581 = v0;
                        let a2582 = {
                            let a2579 = v7;
                            let a2580 = 3u64;
                            nat_add(a2579, a2580)?
                        };
                        if nat_eq(a2581, a2582) {
                            let a2583 = v5;
                            let a2584 = 1u64;
                            if if nat_eq(a2583, a2584) {
                                true
                            } else {
                                let a2585 = v5;
                                let a2586 = 2u64;
                                nat_eq(a2585, a2586)
                            } {
                                Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a2593 = {
                                    let a2587 = v1;
                                    let a2588 = v6;
                                    nat_mul(a2587, a2588)?
                                };
                                let a2594 = {
                                    let a2591 = v0;
                                    let a2592 = {
                                        let a2589 = v7;
                                        let a2590 = 4u64;
                                        nat_add(a2589, a2590)?
                                    };
                                    nat_sub(a2591, a2592)
                                };
                                nat_add(a2593, a2594)?
                            };
                            Ok(Adt5::C4(10u64, 0u64, f49(v12, v2)?, 1u64, 2u64, 3u64, f49(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f120(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f138(v0, v1, v6, v7, v8) {
        let a2595 = v5;
        let a2596 = 26u64;
        nat_eq(a2595, a2596)
    } else {
        false
    } {
        let a2597 = v3;
        let a2598 = v7;
        nat_eq(a2597, a2598)
    } else {
        false
    } {
        let a2599 = v4;
        let a2600 = v8;
        nat_eq(a2599, a2600)
    } else {
        false
    } {
        let a2601 = v9;
        let a2602 = 1u64;
        nat_eq(a2601, a2602)
    } else {
        false
    } {
        let a2603 = v10;
        let a2604 = f139(v0);
        nat_eq(a2603, a2604)
    } else {
        false
    } {
        let a2605 = v11;
        let a2606 = v1;
        nat_eq(a2605, a2606)
    } else {
        false
    } {
        let a2607 = v12;
        let a2608 = v6;
        nat_eq(a2607, a2608)
    } else {
        false
    } {
        let a2609 = v13;
        let a2610 = v7;
        nat_eq(a2609, a2610)
    } else {
        false
    } {
        let a2611 = v14;
        let a2612 = v8;
        nat_eq(a2611, a2612)
    } else {
        false
    }
}

pub fn f121(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2613 = v0;
    let a2614 = 0u64;
    if if if nat_eq(a2613, a2614) {
        true
    } else {
        let a2615 = v1;
        let a2616 = 0u64;
        nat_eq(a2615, a2616)
    } {
        true
    } else {
        let a2617 = v2;
        let a2618 = 0u64;
        nat_eq(a2617, a2618)
    } {
        true
    } else {
        let a2619 = v3;
        let a2620 = 0u64;
        nat_eq(a2619, a2620)
    }
}

pub fn f122(v0: u64) -> u64 {
    let a2621 = v0;
    let a2622 = 2147483648u64;
    if nat_lt(a2621, a2622) {
        v0
    } else {
        0u64
    }
}

pub fn f123(v0: u64) -> u64 {
    let a2623 = v0;
    let a2624 = 2147483648u64;
    if nat_lt(a2623, a2624) {
        0u64
    } else {
        let a2625 = 4294967296u64;
        let a2626 = v0;
        nat_sub(a2625, a2626)
    }
}

pub fn f124(v0: Adt3, v1: bool) -> R<u64> {
    f128(v0.clone(), {
        let a2628 = match v0.clone() {
            Adt3::C0(h2627, _, _, _, _, _, _, _) => {
                h2627
            }
        };
        let a2629 = 1u64;
        nat_sub(a2628, a2629)
    }, {
        let a2631 = match v0.clone() {
            Adt3::C0(_, h2630, _, _, _, _, _, _) => {
                h2630
            }
        };
        let a2632 = 1u64;
        nat_sub(a2631, a2632)
    }, {
        let a2634 = match v0.clone() {
            Adt3::C0(_, _, h2633, _, _, _, _, _) => {
                h2633
            }
        };
        let a2635 = 1u64;
        nat_sub(a2634, a2635)
    }, {
        let a2637 = match v0 {
            Adt3::C0(_, _, _, h2636, _, _, _, _) => {
                h2636
            }
        };
        let a2638 = 1u64;
        nat_sub(a2637, a2638)
    }, v1)
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2639 = v0;
    let a2640 = 0u64;
    if nat_eq(a2639, a2640) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2641 = v0;
            let a2642 = 1u64;
            nat_sub(a2641, a2642)
        };
        let v5: u64 = {
            let a2643 = v4;
            let a2644 = v1;
            let a2645 = 0u64;
            nat_quot(a2643, a2644, a2645)
        };
        let v6: u64 = {
            let a2653 = {
                let a2646 = v5;
                let a2647 = v2;
                nat_mul(a2646, a2647)?
            };
            let a2654 = {
                let a2651 = {
                    let a2648 = v4;
                    let a2649 = v1;
                    let a2650 = 0u64;
                    nat_rem(a2648, a2649, a2650)
                };
                let a2652 = v3;
                nat_mul(a2651, a2652)?
            };
            nat_add(a2653, a2654)?
        };
        let a2655 = v5;
        let a2656 = 0u64;
        if nat_eq(a2655, a2656) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2665 = {
                    let a2659 = {
                        let a2657 = v5;
                        let a2658 = 1u64;
                        nat_sub(a2657, a2658)
                    };
                    let a2660 = v2;
                    nat_mul(a2659, a2660)?
                };
                let a2666 = {
                    let a2663 = {
                        let a2661 = v1;
                        let a2662 = 1u64;
                        nat_sub(a2661, a2662)
                    };
                    let a2664 = v3;
                    nat_mul(a2663, a2664)?
                };
                nat_add(a2665, a2666)?
            };
            let a2667 = v6;
            let a2668 = v7;
            if nat_lt(a2667, a2668) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f126(v0: Adt3, v1: u64) -> bool {
    let a2669 = v1;
    let a2670 = 0u64;
    if !nat_eq(a2669, a2670) {
        true
    } else {
        let a2672 = match v0 {
            Adt3::C0(_, _, _, _, h2671, _, _, _) => {
                h2671
            }
        };
        let a2673 = 4u64;
        nat_eq(a2672, a2673)
    }
}

pub fn f127(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2674 = f140(v0, v1);
    match m2674 {
        None => {
            if v5 {
                let m2675 = f141(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2675 {
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
                let a2676 = 4u64;
                let a2677 = v28;
                nat_le(a2676, a2677)
            };
            let a2678 = v2;
            let a2679 = 0u64;
            if if if if if nat_eq(a2678, a2679) {
                let a2680 = v4;
                let a2681 = 0u64;
                nat_eq(a2680, a2681)
            } else {
                false
            } {
                let a2682 = v6;
                let a2683 = v14;
                if nat_eq(a2682, a2683) {
                    let a2684 = v7;
                    let a2685 = v15;
                    nat_eq(a2684, a2685)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2686 = v8;
                let a2687 = v16;
                if nat_eq(a2686, a2687) {
                    let a2688 = v9;
                    let a2689 = v17;
                    nat_eq(a2688, a2689)
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
                        let a2690 = v3;
                        let a2691 = 0u64;
                        nat_eq(a2690, a2691)
                    } else {
                        false
                    } {
                        f83(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2696 = {
                        let a2692 = v6;
                        let a2693 = v7;
                        nat_mul(a2692, a2693)?
                    };
                    let a2697 = {
                        let a2694 = v8;
                        let a2695 = v9;
                        nat_mul(a2694, a2695)?
                    };
                    nat_mul(a2696, a2697)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f128(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2718 = {
        let a2712 = {
            let a2706 = {
                let a2700 = v1;
                let a2701 = if v5 {
                    f123(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2698, _, _, _) => {
                            h2698
                        }
                    })
                } else {
                    f122(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2699, _, _, _) => {
                            h2699
                        }
                    })
                };
                nat_mul(a2700, a2701)?
            };
            let a2707 = {
                let a2704 = v2;
                let a2705 = if v5 {
                    f123(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2702, _, _) => {
                            h2702
                        }
                    })
                } else {
                    f122(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2703, _, _) => {
                            h2703
                        }
                    })
                };
                nat_mul(a2704, a2705)?
            };
            nat_add(a2706, a2707)?
        };
        let a2713 = {
            let a2710 = v3;
            let a2711 = if v5 {
                f123(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2708, _) => {
                        h2708
                    }
                })
            } else {
                f122(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2709, _) => {
                        h2709
                    }
                })
            };
            nat_mul(a2710, a2711)?
        };
        nat_add(a2712, a2713)?
    };
    let a2719 = {
        let a2716 = v4;
        let a2717 = if v5 {
            f123(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2714) => {
                    h2714
                }
            })
        } else {
            f122(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2715) => {
                    h2715
                }
            })
        };
        nat_mul(a2716, a2717)?
    };
    nat_add(a2718, a2719)
}

pub fn f129(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f130(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2720 = v1;
    let a2721 = 0u64;
    if nat_eq(a2720, a2721) {
        Ok(Adt5::C4({
            let a2722 = v3;
            let a2723 = 0u64;
            if nat_eq(a2722, a2723) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f49(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2724 = v3;
            let a2725 = 0u64;
            if nat_eq(a2724, a2725) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f131(v0: u64, v1: u64) -> R<u64> {
    let a2726 = v0;
    let a2727 = f76(v1)?;
    let a2728 = 0u64;
    Ok(nat_quot(a2726, a2727, a2728))
}

pub fn f132(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f131(v0, v1)?;
    let v3: u64 = {
        let a2731 = {
            let a2729 = f108(v0, v1)?;
            let a2730 = 2u64;
            nat_sub(a2729, a2730)
        };
        let a2732 = 2u64;
        let a2733 = 0u64;
        nat_quot(a2731, a2732, a2733)
    };
    let v4: u64 = {
        let a2734 = v2;
        let a2735 = v1;
        nat_mul(a2734, a2735)?
    };
    if f111(v0, v1)? {
        let a2736 = v4;
        let a2737 = v3;
        nat_add(a2736, a2737)
    } else {
        let a2749 = {
            let a2745 = v4;
            let a2746 = {
                let a2743 = {
                    let a2738 = v1;
                    let a2739 = 1u64;
                    nat_sub(a2738, a2739)
                };
                let a2744 = {
                    let a2740 = v2;
                    let a2741 = v1;
                    let a2742 = 0u64;
                    nat_rem(a2740, a2741, a2742)
                };
                nat_mul(a2743, a2744)?
            };
            nat_sub(a2745, a2746)
        };
        let a2750 = {
            let a2747 = v3;
            let a2748 = v1;
            nat_mul(a2747, a2748)?
        };
        nat_add(a2749, a2750)
    }
}

pub fn f133(v0: u64) -> bool {
    let a2751 = v0;
    let a2752 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2751, a2752) {
        true
    } else {
        let a2753 = v0;
        let a2754 = 1u64;
        nat_eq(a2753, a2754)
    } {
        true
    } else {
        let a2755 = v0;
        let a2756 = 2u64;
        nat_eq(a2755, a2756)
    } {
        true
    } else {
        let a2757 = v0;
        let a2758 = 3u64;
        nat_eq(a2757, a2758)
    } {
        true
    } else {
        let a2759 = v0;
        let a2760 = 6u64;
        nat_eq(a2759, a2760)
    } {
        true
    } else {
        let a2761 = v0;
        let a2762 = 7u64;
        nat_eq(a2761, a2762)
    } {
        true
    } else {
        let a2763 = v0;
        let a2764 = 8u64;
        nat_eq(a2763, a2764)
    } {
        true
    } else {
        let a2765 = v0;
        let a2766 = 10u64;
        nat_eq(a2765, a2766)
    } {
        true
    } else {
        let a2767 = v0;
        let a2768 = 11u64;
        nat_eq(a2767, a2768)
    } {
        true
    } else {
        let a2769 = v0;
        let a2770 = 12u64;
        nat_eq(a2769, a2770)
    } {
        true
    } else {
        let a2771 = v0;
        let a2772 = 13u64;
        nat_eq(a2771, a2772)
    } {
        true
    } else {
        let a2773 = v0;
        let a2774 = 14u64;
        nat_eq(a2773, a2774)
    } {
        true
    } else {
        let a2775 = v0;
        let a2776 = 16u64;
        nat_eq(a2775, a2776)
    } {
        true
    } else {
        let a2777 = v0;
        let a2778 = 17u64;
        nat_eq(a2777, a2778)
    } {
        true
    } else {
        let a2779 = v0;
        let a2780 = 18u64;
        nat_eq(a2779, a2780)
    } {
        true
    } else {
        let a2781 = v0;
        let a2782 = 19u64;
        nat_eq(a2781, a2782)
    } {
        true
    } else {
        let a2783 = v0;
        let a2784 = 20u64;
        nat_eq(a2783, a2784)
    } {
        true
    } else {
        let a2785 = v0;
        let a2786 = 21u64;
        nat_eq(a2785, a2786)
    } {
        true
    } else {
        let a2787 = v0;
        let a2788 = 22u64;
        nat_eq(a2787, a2788)
    } {
        true
    } else {
        let a2789 = v0;
        let a2790 = 23u64;
        nat_eq(a2789, a2790)
    } {
        true
    } else {
        let a2791 = v0;
        let a2792 = 26u64;
        nat_eq(a2791, a2792)
    } {
        true
    } else {
        let a2793 = v0;
        let a2794 = 29u64;
        nat_eq(a2793, a2794)
    } {
        true
    } else {
        let a2795 = v0;
        let a2796 = 30u64;
        nat_eq(a2795, a2796)
    } {
        true
    } else {
        let a2797 = v0;
        let a2798 = 34u64;
        nat_eq(a2797, a2798)
    } {
        true
    } else {
        let a2799 = v0;
        let a2800 = 35u64;
        nat_eq(a2799, a2800)
    } {
        true
    } else {
        let a2801 = v0;
        let a2802 = 39u64;
        nat_eq(a2801, a2802)
    } {
        true
    } else {
        let a2803 = v0;
        let a2804 = 40u64;
        nat_eq(a2803, a2804)
    } {
        true
    } else {
        let a2805 = v0;
        let a2806 = 41u64;
        nat_eq(a2805, a2806)
    } {
        true
    } else {
        let a2807 = v0;
        let a2808 = 42u64;
        nat_eq(a2807, a2808)
    } {
        true
    } else {
        let a2809 = v0;
        let a2810 = 43u64;
        nat_eq(a2809, a2810)
    }
}

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2811 = v0;
    let a2812 = 0u64;
    if if if nat_eq(a2811, a2812) {
        true
    } else {
        let a2813 = v1;
        let a2814 = 0u64;
        nat_eq(a2813, a2814)
    } {
        true
    } else {
        let a2815 = v2;
        let a2816 = 0u64;
        nat_eq(a2815, a2816)
    } {
        Ok(0u64)
    } else {
        let a2819 = {
            let a2817 = v0;
            let a2818 = v1;
            nat_mul(a2817, a2818)?
        };
        let a2820 = v2;
        nat_mul(a2819, a2820)
    }
}

pub fn f135(v0: u64, v1: u64) -> Adt8 {
    let a2821 = v0;
    let a2822 = 0u64;
    if nat_eq(a2821, a2822) {
        Adt8::C0(0u64, v1)
    } else {
        let a2823 = v0;
        let a2824 = 1u64;
        if nat_eq(a2823, a2824) {
            Adt8::C1(v1)
        } else {
            let a2825 = v0;
            let a2826 = 2u64;
            if nat_eq(a2825, a2826) {
                Adt8::C2(v1)
            } else {
                let a2827 = v0;
                let a2828 = 3u64;
                if nat_eq(a2827, a2828) {
                    Adt8::C3(v1)
                } else {
                    let a2829 = v0;
                    let a2830 = 13u64;
                    if nat_lt(a2829, a2830) {
                        Adt8::C0({
                            let a2831 = v0;
                            let a2832 = 3u64;
                            nat_sub(a2831, a2832)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f136(v0: u64, v1: u64) -> Adt9 {
    let a2833 = v0;
    let a2834 = 1u64;
    if nat_eq(a2833, a2834) {
        Adt9::C0(v1)
    } else {
        let a2835 = v0;
        let a2836 = 3u64;
        if if nat_eq(a2835, a2836) {
            true
        } else {
            let a2837 = v0;
            let a2838 = 6u64;
            nat_eq(a2837, a2838)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f137(v0: u64) -> u64 {
    let m2839 = f82(v0);
    match m2839 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2841 = match v1.clone() {
                Adt11::C0(_, h2840, _) => {
                    h2840
                }
            };
            let a2842 = 1u64;
            if if nat_eq(a2841, a2842) {
                let a2844 = match v1.clone() {
                    Adt11::C0(_, _, h2843) => {
                        h2843
                    }
                };
                let a2845 = 1u64;
                if if if nat_eq(a2844, a2845) {
                    true
                } else {
                    let a2847 = match v1.clone() {
                        Adt11::C0(_, _, h2846) => {
                            h2846
                        }
                    };
                    let a2848 = 2u64;
                    nat_eq(a2847, a2848)
                } {
                    true
                } else {
                    let a2850 = match v1.clone() {
                        Adt11::C0(_, _, h2849) => {
                            h2849
                        }
                    };
                    let a2851 = 4u64;
                    nat_eq(a2850, a2851)
                } {
                    true
                } else {
                    let a2853 = match v1.clone() {
                        Adt11::C0(_, _, h2852) => {
                            h2852
                        }
                    };
                    let a2854 = 8u64;
                    nat_eq(a2853, a2854)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2855) => {
                        h2855
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f121(v1, v2, v3, v4) {
        f142(v0)
    } else {
        f133(v0)
    }
}

pub fn f139(v0: u64) -> u64 {
    let a2856 = v0;
    let a2857 = 26u64;
    if nat_eq(a2856, a2857) {
        26u64
    } else {
        0u64
    }
}

pub fn f140(v0: u64, v1: u64) -> Option<u64> {
    let a2858 = v0;
    let a2859 = 2u64;
    if nat_eq(a2858, a2859) {
        Some(0u64)
    } else {
        let a2860 = v0;
        let a2861 = 5u64;
        if nat_eq(a2860, a2861) {
            Some(1u64)
        } else {
            let a2862 = v0;
            let a2863 = 6u64;
            if nat_eq(a2862, a2863) {
                Some(2u64)
            } else {
                let a2864 = v0;
                let a2865 = 7u64;
                if nat_eq(a2864, a2865) {
                    Some(3u64)
                } else {
                    let a2866 = v0;
                    let a2867 = 9u64;
                    if nat_eq(a2866, a2867) {
                        Some(4u64)
                    } else {
                        let a2868 = v0;
                        let a2869 = 8u64;
                        if nat_eq(a2868, a2869) {
                            Some(5u64)
                        } else {
                            let a2870 = v0;
                            let a2871 = 89u64;
                            if nat_eq(a2870, a2871) {
                                let a2872 = v1;
                                let a2873 = 2u64;
                                if nat_eq(a2872, a2873) {
                                    Some(6u64)
                                } else {
                                    let a2874 = v1;
                                    let a2875 = 0u64;
                                    if nat_eq(a2874, a2875) {
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

pub fn f141(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a2876 = v0;
    let a2877 = 28u64;
    if if if if nat_eq(a2876, a2877) {
        let a2878 = v1;
        let a2879 = 0u64;
        nat_eq(a2878, a2879)
    } else {
        false
    } {
        let a2880 = v2;
        let a2881 = 0u64;
        nat_eq(a2880, a2881)
    } else {
        false
    } {
        let a2882 = v3;
        let a2883 = 0u64;
        nat_eq(a2882, a2883)
    } else {
        false
    } {
        let a2884 = v4;
        let a2885 = v8;
        if if if if if if if if nat_eq(a2884, a2885) {
            let a2886 = v5;
            let a2887 = v12;
            nat_eq(a2886, a2887)
        } else {
            false
        } {
            let a2888 = v9;
            let a2889 = v13;
            nat_eq(a2888, a2889)
        } else {
            false
        } {
            let a2890 = v10;
            let a2891 = v14;
            nat_eq(a2890, a2891)
        } else {
            false
        } {
            let a2892 = v11;
            let a2893 = v15;
            nat_eq(a2892, a2893)
        } else {
            false
        } {
            let a2894 = 0u64;
            let a2895 = v6;
            nat_lt(a2894, a2895)
        } else {
            false
        } {
            let a2896 = 0u64;
            let a2897 = v7;
            nat_lt(a2896, a2897)
        } else {
            false
        } {
            let a2901 = {
                let a2898 = v10;
                let a2899 = v6;
                let a2900 = 1u64;
                nat_rem(a2898, a2899, a2900)
            };
            let a2902 = 0u64;
            if nat_eq(a2901, a2902) {
                let a2906 = {
                    let a2903 = v11;
                    let a2904 = v7;
                    let a2905 = 1u64;
                    nat_rem(a2903, a2904, a2905)
                };
                let a2907 = 0u64;
                nat_eq(a2906, a2907)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a2908 = v10;
                let a2909 = v6;
                let a2910 = 0u64;
                nat_quot(a2908, a2909, a2910)
            }, {
                let a2911 = v11;
                let a2912 = v7;
                let a2913 = 0u64;
                nat_quot(a2911, a2912, a2913)
            }, {
                let a2914 = v5;
                let a2915 = v4;
                nat_mul(a2914, a2915)?
            }, {
                let a2916 = v4;
                let a2917 = v9;
                nat_mul(a2916, a2917)?
            }, {
                let a2918 = v5;
                let a2919 = v9;
                nat_mul(a2918, a2919)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f142(v0: u64) -> bool {
    let m2920 = f82(v0);
    match m2920 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}
