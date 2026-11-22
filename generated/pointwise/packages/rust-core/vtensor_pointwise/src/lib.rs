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
    C1(u64, Adt3, Adt3, Adt4, u64),
    C2(Adt4),
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
    C26(u64, u64, u64, u64),
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
    C5(u64),
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
                            let a8 = 23u64;
                            if nat_eq(a7, a8) {
                                Ok(Adt5::C0(Adt6::C5(64u64)))
                            } else {
                                let a9 = v1;
                                let a10 = 24u64;
                                if nat_eq(a9, a10) {
                                    Ok(Adt5::C0(f3(v2, v14, v13)?))
                                } else {
                                    let a11 = v1;
                                    let a12 = 27u64;
                                    if nat_eq(a11, a12) {
                                        Ok(Adt5::C0(f4(v2, v14, v13)?))
                                    } else {
                                        let a13 = v1;
                                        let a14 = 3u64;
                                        if nat_eq(a13, a14) {
                                            Ok(Adt5::C0(f5(v14, v4, v5, v13)?))
                                        } else {
                                            f6(v1, v2, v14, v4, v13)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v15, v16, v17, v18, v19) => {
            f7(v15, v16, v17, v18, v19)
        }
        Adt0::C2(v20) => {
            f8(v20)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
    let v2: u64 = f9(v1.clone(), 1u64)?;
    let a16 = match v1.clone() {
        Adt2::C0(h15, _, _, _, _, _, _, _, _, _, _) => {
            h15
        }
    };
    let a17 = 0u64;
    if if if if if if if if nat_eq(a16, a17) {
        let a19 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h18, _, _, _, _, _) => {
                h18
            }
        };
        let a20 = 4u64;
        nat_eq(a19, a20)
    } else {
        false
    } {
        f10(match v0.clone() {
            Adt1::C0(h21, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h21
            }
        })
    } else {
        false
    } {
        f10(match v0.clone() {
            Adt1::C0(_, h22, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h22
            }
        })
    } else {
        false
    } {
        let a24 = 0u64;
        let a25 = match v0.clone() {
            Adt1::C0(_, _, h23, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h23
            }
        };
        nat_lt(a24, a25)
    } else {
        false
    } {
        let a27 = match v0.clone() {
            Adt1::C0(_, _, h26, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        };
        let a28 = 2139095040u64;
        nat_lt(a27, a28)
    } else {
        false
    } {
        let a29 = v2;
        let a30 = 9223372036854775808u64;
        nat_lt(a29, a30)
    } else {
        false
    } {
        let a32 = f11(v2)?;
        let a33 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h31, _) => {
                h31
            }
        };
        nat_le(a32, a33)
    } else {
        false
    } {
        Ok(Adt6::C4(f12(v2)?, 4u64, f13(v1.clone(), 1u64), f13(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h34, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h34
            }
        }, match v0.clone() {
            Adt1::C0(_, h35, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h35
            }
        }, match v0 {
            Adt1::C0(_, _, h36, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h36
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m37 = v2;
    match m37 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let m38 = v3;
            match m38 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v6) => {
                    let a40 = match v1.clone() {
                        Adt2::C0(h39, _, _, _, _, _, _, _, _, _, _) => {
                            h39
                        }
                    };
                    let a41 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a40, a41) {
                        let a43 = match v5.clone() {
                            Adt2::C0(h42, _, _, _, _, _, _, _, _, _, _) => {
                                h42
                            }
                        };
                        let a44 = 0u64;
                        nat_eq(a43, a44)
                    } else {
                        false
                    } {
                        let a46 = match v6.clone() {
                            Adt2::C0(h45, _, _, _, _, _, _, _, _, _, _) => {
                                h45
                            }
                        };
                        let a47 = 0u64;
                        nat_eq(a46, a47)
                    } else {
                        false
                    } {
                        let a49 = match v4.clone() {
                            Adt2::C0(h48, _, _, _, _, _, _, _, _, _, _) => {
                                h48
                            }
                        };
                        let a50 = 0u64;
                        nat_eq(a49, a50)
                    } else {
                        false
                    } {
                        f14(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f14(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f15(v1.clone())?
                    } else {
                        false
                    } {
                        f15(v5.clone())?
                    } else {
                        false
                    } {
                        f15(v6)?
                    } else {
                        false
                    } {
                        f15(v4.clone())?
                    } else {
                        false
                    } {
                        let a56 = match v1.clone() {
                            Adt2::C0(_, h51, _, _, _, _, _, _, _, _, _) => {
                                h51
                            }
                        };
                        let a57 = {
                            let a54 = match v5.clone() {
                                Adt2::C0(_, h52, _, _, _, _, _, _, _, _, _) => {
                                    h52
                                }
                            };
                            let a55 = match v5.clone() {
                                Adt2::C0(_, h53, _, _, _, _, _, _, _, _, _) => {
                                    h53
                                }
                            };
                            nat_mul(a54, a55)?
                        };
                        nat_eq(a56, a57)
                    } else {
                        false
                    } {
                        let a63 = match v1.clone() {
                            Adt2::C0(_, _, h58, _, _, _, _, _, _, _, _) => {
                                h58
                            }
                        };
                        let a64 = {
                            let a61 = match v5.clone() {
                                Adt2::C0(_, _, h59, _, _, _, _, _, _, _, _) => {
                                    h59
                                }
                            };
                            let a62 = match v5.clone() {
                                Adt2::C0(_, _, _, h60, _, _, _, _, _, _, _) => {
                                    h60
                                }
                            };
                            nat_mul(a61, a62)?
                        };
                        nat_eq(a63, a64)
                    } else {
                        false
                    } {
                        let a67 = match v1.clone() {
                            Adt2::C0(_, _, _, h65, _, _, _, _, _, _, _) => {
                                h65
                            }
                        };
                        let a68 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h66, _, _, _, _, _, _) => {
                                h66
                            }
                        };
                        nat_eq(a67, a68)
                    } else {
                        false
                    } {
                        let a70 = match v0.clone() {
                            Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h69
                            }
                        };
                        let a71 = 4294967296u64;
                        nat_lt(a70, a71)
                    } else {
                        false
                    } {
                        let a73 = match v0.clone() {
                            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h72
                            }
                        };
                        let a74 = 0u64;
                        if nat_eq(a73, a74) {
                            if f16(v1.clone(), v4.clone())? {
                                true
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
                            let a81 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h79) => {
                                    h79
                                }
                            };
                            let a82 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h80) => {
                                    h80
                                }
                            };
                            nat_eq(a81, a82)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a84 = match v4.clone() {
                                Adt2::C0(_, h83, _, _, _, _, _, _, _, _, _) => {
                                    h83
                                }
                            };
                            let a85 = 0u64;
                            if if if nat_eq(a84, a85) {
                                true
                            } else {
                                let a87 = match v4.clone() {
                                    Adt2::C0(_, _, h86, _, _, _, _, _, _, _, _) => {
                                        h86
                                    }
                                };
                                let a88 = 0u64;
                                nat_eq(a87, a88)
                            } {
                                true
                            } else {
                                let a90 = match v4.clone() {
                                    Adt2::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                                        h89
                                    }
                                };
                                let a91 = 0u64;
                                nat_eq(a90, a91)
                            } {
                                true
                            } else {
                                let a93 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h92, _, _, _, _, _, _) => {
                                        h92
                                    }
                                };
                                let a94 = 0u64;
                                nat_eq(a93, a94)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h95, _, _, _, _, _, _, _, _, _) => {
                                h95
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a97 = match v0 {
                                Adt1::C0(h96, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h96
                                }
                            };
                            let a98 = 0u64;
                            if nat_eq(a97, a98) {
                                f9(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f17(v8, match v5.clone() {
                                Adt2::C0(_, _, h99, _, _, _, _, _, _, _, _) => {
                                    h99
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h100, _, _, _, _, _, _, _) => {
                                    h100
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h101, _, _, _, _, _, _) => {
                                    h101
                                }
                            })?
                        };
                        Ok(Adt6::C4(f18(v9, v8, v10)?, 4u64, f13(v1, 1u64), f13(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h102, _, _, _, _, _, _, _, _, _, _) => {
            h102
        }
    };
    let v4: u64 = f19(v3);
    let a104 = match v1.clone() {
        Adt2::C0(h103, _, _, _, _, _, _, _, _, _, _) => {
            h103
        }
    };
    let a105 = 0u64;
    if if if if if if if if nat_eq(a104, a105) {
        true
    } else {
        let a107 = match v1.clone() {
            Adt2::C0(h106, _, _, _, _, _, _, _, _, _, _) => {
                h106
            }
        };
        let a108 = 30u64;
        nat_eq(a107, a108)
    } {
        let a111 = match v1.clone() {
            Adt2::C0(h109, _, _, _, _, _, _, _, _, _, _) => {
                h109
            }
        };
        let a112 = match v2.clone() {
            Adt2::C0(h110, _, _, _, _, _, _, _, _, _, _) => {
                h110
            }
        };
        nat_eq(a111, a112)
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        f20(match v0.clone() {
            Adt1::C0(h113, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h113
            }
        })
    } else {
        false
    } {
        f21(v1.clone(), v4)
    } else {
        false
    } {
        f21(v2.clone(), v4)
    } else {
        false
    } {
        if f16(v1.clone(), v2.clone())? {
            true
        } else {
            f22(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f23(match v1.clone() {
            Adt2::C0(_, h114, _, _, _, _, _, _, _, _, _) => {
                h114
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h115, _, _, _, _, _, _, _, _) => {
                h115
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h116, _, _, _, _, _, _, _) => {
                h116
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h117, _, _, _, _, _, _) => {
                h117
            }
        }) {
            0u64
        } else {
            let a123 = match v1.clone() {
                Adt2::C0(_, _, h118, _, _, _, _, _, _, _, _) => {
                    h118
                }
            };
            let a124 = {
                let a121 = match v1.clone() {
                    Adt2::C0(_, _, _, h119, _, _, _, _, _, _, _) => {
                        h119
                    }
                };
                let a122 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h120, _, _, _, _, _, _) => {
                        h120
                    }
                };
                nat_mul(a121, a122)?
            };
            nat_mul(a123, a124)?
        };
        let v6: u64 = {
            let a125 = v5;
            let a126 = 0u64;
            if if nat_eq(a125, a126) {
                true
            } else {
                f24(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a128 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h127, _) => {
                        h127
                    }
                };
                let a129 = v4;
                let a130 = 0u64;
                nat_quot(a128, a129, a130)
            }
        };
        Ok(Adt6::C4(f25(v3, match v1.clone() {
            Adt2::C0(_, h131, _, _, _, _, _, _, _, _, _) => {
                h131
            }
        }, v5, v6)?, v4, f13(v1.clone(), 1u64), f13(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h132, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h132
            }
        }, v5, f26(match v1 {
            Adt2::C0(_, h133, _, _, _, _, _, _, _, _, _) => {
                h133
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a135 = match v1.clone() {
        Adt2::C0(h134, _, _, _, _, _, _, _, _, _, _) => {
            h134
        }
    };
    let a136 = 0u64;
    if if if if if nat_eq(a135, a136) {
        let a138 = match v2.clone() {
            Adt2::C0(h137, _, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        };
        let a139 = 0u64;
        nat_eq(a138, a139)
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        f27(match v0.clone() {
            Adt1::C0(h140, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h140
            }
        })
    } else {
        false
    } {
        if f16(v1.clone(), v2.clone())? {
            true
        } else {
            f22(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f13(v1, 1u64);
        let v4: u64 = {
            let a146 = match v3.clone() {
                Adt3::C0(_, h141, _, _, _, _, _, _) => {
                    h141
                }
            };
            let a147 = {
                let a144 = match v3.clone() {
                    Adt3::C0(_, _, h142, _, _, _, _, _) => {
                        h142
                    }
                };
                let a145 = match v3.clone() {
                    Adt3::C0(_, _, _, h143, _, _, _, _) => {
                        h143
                    }
                };
                nat_mul(a144, a145)?
            };
            nat_mul(a146, a147)?
        };
        Ok(Adt6::C4(f28(match v3.clone() {
            Adt3::C0(h148, _, _, _, _, _, _, _) => {
                h148
            }
        }, v4)?, 4u64, v3, f13(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h149
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f5(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m150 = v1;
    match m150 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m151 = v2;
            match m151 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a153 = match v0.clone() {
                        Adt2::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
                            h152
                        }
                    };
                    let a154 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a153, a154) {
                        let a156 = match v4.clone() {
                            Adt2::C0(h155, _, _, _, _, _, _, _, _, _, _) => {
                                h155
                            }
                        };
                        let a157 = 0u64;
                        nat_eq(a156, a157)
                    } else {
                        false
                    } {
                        let a159 = match v5.clone() {
                            Adt2::C0(h158, _, _, _, _, _, _, _, _, _, _) => {
                                h158
                            }
                        };
                        let a160 = 26u64;
                        nat_eq(a159, a160)
                    } else {
                        false
                    } {
                        let a162 = match v3.clone() {
                            Adt2::C0(h161, _, _, _, _, _, _, _, _, _, _) => {
                                h161
                            }
                        };
                        let a163 = 0u64;
                        nat_eq(a162, a163)
                    } else {
                        false
                    } {
                        f14(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a166 = match v0.clone() {
                            Adt2::C0(_, h164, _, _, _, _, _, _, _, _, _) => {
                                h164
                            }
                        };
                        let a167 = match v4.clone() {
                            Adt2::C0(_, h165, _, _, _, _, _, _, _, _, _) => {
                                h165
                            }
                        };
                        nat_eq(a166, a167)
                    } else {
                        false
                    } {
                        let a170 = match v0.clone() {
                            Adt2::C0(_, _, h168, _, _, _, _, _, _, _, _) => {
                                h168
                            }
                        };
                        let a171 = match v5.clone() {
                            Adt2::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
                                h169
                            }
                        };
                        nat_eq(a170, a171)
                    } else {
                        false
                    } {
                        let a174 = match v0.clone() {
                            Adt2::C0(_, _, _, h172, _, _, _, _, _, _, _) => {
                                h172
                            }
                        };
                        let a175 = match v5.clone() {
                            Adt2::C0(_, _, h173, _, _, _, _, _, _, _, _) => {
                                h173
                            }
                        };
                        nat_eq(a174, a175)
                    } else {
                        false
                    } {
                        let a177 = 0u64;
                        let a178 = match v5.clone() {
                            Adt2::C0(_, _, _, h176, _, _, _, _, _, _, _) => {
                                h176
                            }
                        };
                        nat_lt(a177, a178)
                    } else {
                        false
                    } {
                        let a180 = 0u64;
                        let a181 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h179, _, _, _, _, _, _) => {
                                h179
                            }
                        };
                        nat_lt(a180, a181)
                    } else {
                        false
                    } {
                        let a183 = 4u64;
                        let a184 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h182, _, _, _, _, _) => {
                                h182
                            }
                        };
                        nat_le(a183, a184)
                    } else {
                        false
                    } {
                        let a186 = 4u64;
                        let a187 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h185, _, _, _, _, _) => {
                                h185
                            }
                        };
                        nat_le(a186, a187)
                    } else {
                        false
                    } {
                        let a189 = 4u64;
                        let a190 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h188, _, _, _, _, _) => {
                                h188
                            }
                        };
                        nat_le(a189, a190)
                    } else {
                        false
                    } {
                        let a192 = 4u64;
                        let a193 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h191, _, _, _, _, _) => {
                                h191
                            }
                        };
                        nat_le(a192, a193)
                    } else {
                        false
                    } {
                        f16(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f16(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f16(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f29(match v0.clone() {
                            Adt2::C0(_, h194, _, _, _, _, _, _, _, _, _) => {
                                h194
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h195, _, _, _, _, _, _, _, _) => {
                                h195
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h196, _, _, _, _, _, _, _) => {
                                h196
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h197, _, _, _, _, _, _) => {
                                h197
                            }
                        })?, 4u64, f13(v0, 1u64), f13(v3, 1u64), Adt4::C22(f13(v4, 1u64), f13(v5, 1u64))))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt5> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h198, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h198
        }
    };
    Ok(Adt5::C0({
        let a199 = v0;
        let a200 = 39u64;
        if nat_eq(a199, a200) {
            f30(v2, v3, v4)?
        } else {
            let a201 = v0;
            let a202 = 77u64;
            if nat_eq(a201, a202) {
                f31(v2, v4)?
            } else {
                let a203 = v0;
                let a204 = 40u64;
                if nat_eq(a203, a204) {
                    f32(v2, v3, v4)?
                } else {
                    if f33(v0) {
                        f34(v0, v2, {
                            let m205 = v3;
                            match m205 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a206 = v0;
                        let a207 = 19u64;
                        if nat_eq(a206, a207) {
                            f35(v2, v4)?
                        } else {
                            let a208 = v0;
                            let a209 = 21u64;
                            if nat_eq(a208, a209) {
                                f36(v5, v2, v3, v4)?
                            } else {
                                let a210 = v0;
                                let a211 = 61u64;
                                if if if nat_eq(a210, a211) {
                                    true
                                } else {
                                    let a212 = v0;
                                    let a213 = 62u64;
                                    nat_eq(a212, a213)
                                } {
                                    true
                                } else {
                                    let a214 = v0;
                                    let a215 = 63u64;
                                    nat_eq(a214, a215)
                                } {
                                    f37(v0, v1, v2, v4)?
                                } else {
                                    let a216 = v0;
                                    let a217 = 4u64;
                                    if if nat_eq(a216, a217) {
                                        true
                                    } else {
                                        let a218 = v0;
                                        let a219 = 32u64;
                                        nat_eq(a218, a219)
                                    } {
                                        f38(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a220 = v0;
                                        let a221 = 20u64;
                                        if nat_eq(a220, a221) {
                                            f39(v2, v4)?
                                        } else {
                                            let a222 = v0;
                                            let a223 = 42u64;
                                            if nat_eq(a222, a223) {
                                                f40(4u64, v2, v4)?
                                            } else {
                                                let a224 = v0;
                                                let a225 = 69u64;
                                                if nat_eq(a224, a225) {
                                                    let a226 = v5;
                                                    let a227 = 4u64;
                                                    if nat_lt(a226, a227) {
                                                        f40(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a228 = v0;
                                                    let a229 = 70u64;
                                                    if nat_eq(a228, a229) {
                                                        f41(v5, v2, v4)?
                                                    } else {
                                                        let a230 = v0;
                                                        let a231 = 75u64;
                                                        if if nat_eq(a230, a231) {
                                                            true
                                                        } else {
                                                            let a232 = v0;
                                                            let a233 = 76u64;
                                                            nat_eq(a232, a233)
                                                        } {
                                                            f42({
                                                                let a234 = v0;
                                                                let a235 = 76u64;
                                                                nat_eq(a234, a235)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a236 = v0;
                                                            let a237 = 31u64;
                                                            if if if nat_eq(a236, a237) {
                                                                true
                                                            } else {
                                                                let a238 = v0;
                                                                let a239 = 47u64;
                                                                nat_eq(a238, a239)
                                                            } {
                                                                true
                                                            } else {
                                                                let a240 = v0;
                                                                let a241 = 68u64;
                                                                nat_eq(a240, a241)
                                                            } {
                                                                f43(v0, v1, v2, v4)?
                                                            } else {
                                                                let a242 = v0;
                                                                let a243 = 15u64;
                                                                if nat_eq(a242, a243) {
                                                                    f44(v2, v4)?
                                                                } else {
                                                                    let a244 = v0;
                                                                    let a245 = 17u64;
                                                                    if nat_eq(a244, a245) {
                                                                        f45(v2, v4)?
                                                                    } else {
                                                                        let a246 = v0;
                                                                        let a247 = 18u64;
                                                                        if nat_eq(a246, a247) {
                                                                            f46(v2, v3, v4)?
                                                                        } else {
                                                                            let a248 = v0;
                                                                            let a249 = 13u64;
                                                                            if if if nat_eq(a248, a249) {
                                                                                true
                                                                            } else {
                                                                                let a250 = v0;
                                                                                let a251 = 14u64;
                                                                                nat_eq(a250, a251)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a252 = v0;
                                                                                let a253 = 16u64;
                                                                                nat_eq(a252, a253)
                                                                            } {
                                                                                f47(v0, v2, v4)?
                                                                            } else {
                                                                                let m254 = f48(v0, v5);
                                                                                match m254 {
                                                                                    None => {
                                                                                        f49(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f50(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt5> {
    Ok(f51(v3.clone(), {
        let m255 = v3.clone();
        match m255 {
            Adt4::C0 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f53(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f54(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f55(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f56(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v57, v58, v59, v60) => {
                f58(v0, v60, v57, v58, v59, v4)?
            }
            Adt4::C24(_) => {
                Adt5::C0(Adt6::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C26(v52, v53, v54, v55) => {
                let a256 = v52;
                let a257 = 64u64;
                if nat_eq(a256, a257) {
                    Adt5::C0(Adt6::C5(64u64))
                } else {
                    let a258 = v52;
                    let a259 = 0u64;
                    if if nat_eq(a258, a259) {
                        true
                    } else {
                        let a260 = v52;
                        let a261 = 30u64;
                        nat_eq(a260, a261)
                    } {
                        f57(v0, v1, v2, v52, v53, v54, v55, v4)?
                    } else {
                        Adt5::C0(Adt6::C1)
                    }
                }
            }
        }
    }))
}

pub fn f8(v0: Adt4) -> R<Adt5> {
    let m262 = v0;
    match m262 {
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
                let a267 = {
                    let a263 = 0u64;
                    let a264 = v45;
                    if nat_lt(a263, a264) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a268 = {
                    let a265 = 0u64;
                    let a266 = v47;
                    if nat_lt(a265, a266) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a267, a268)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(_, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
    }
}

pub fn f9(v0: Adt2, v1: u64) -> R<u64> {
    let a280 = {
        let a274 = {
            let a270 = match v0.clone() {
                Adt2::C0(_, h269, _, _, _, _, _, _, _, _, _) => {
                    h269
                }
            };
            let a271 = v1;
            let a272 = 0u64;
            nat_quot(a270, a271, a272)
        };
        let a275 = match v0.clone() {
            Adt2::C0(_, _, h273, _, _, _, _, _, _, _, _) => {
                h273
            }
        };
        nat_mul(a274, a275)?
    };
    let a281 = {
        let a278 = match v0.clone() {
            Adt2::C0(_, _, _, h276, _, _, _, _, _, _, _) => {
                h276
            }
        };
        let a279 = match v0 {
            Adt2::C0(_, _, _, _, h277, _, _, _, _, _, _) => {
                h277
            }
        };
        nat_mul(a278, a279)?
    };
    nat_mul(a280, a281)
}

pub fn f10(v0: u64) -> bool {
    let a282 = v0;
    let a283 = 4294967296u64;
    if nat_lt(a282, a283) {
        let a287 = {
            let a284 = v0;
            let a285 = 2147483648u64;
            let a286 = 0u64;
            nat_rem(a284, a285, a286)
        };
        let a288 = 2139095040u64;
        nat_lt(a287, a288)
    } else {
        false
    }
}

pub fn f11(v0: u64) -> R<u64> {
    let a289 = 4u64;
    let a290 = v0;
    nat_mul(a289, a290)
}

pub fn f12(v0: u64) -> R<u64> {
    let a291 = v0;
    let a292 = 0u64;
    if nat_eq(a291, a292) {
        Ok(0u64)
    } else {
        let a295 = 4u64;
        let a296 = {
            let a293 = 3u64;
            let a294 = v0;
            nat_mul(a293, a294)?
        };
        nat_add(a295, a296)
    }
}

pub fn f13(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a298 = match v0.clone() {
            Adt2::C0(_, h297, _, _, _, _, _, _, _, _, _) => {
                h297
            }
        };
        let a299 = v1;
        let a300 = 0u64;
        nat_quot(a298, a299, a300)
    }, match v0.clone() {
        Adt2::C0(_, _, h301, _, _, _, _, _, _, _, _) => {
            h301
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h302, _, _, _, _, _, _, _) => {
            h302
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h303, _, _, _, _, _, _) => {
            h303
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h304, _, _, _, _, _) => {
            h304
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h305, _, _, _, _) => {
            h305
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h306, _, _, _) => {
            h306
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h307, _, _) => {
            h307
        }
    })
}

pub fn f14(v0: Adt2, v1: Adt2) -> bool {
    let a310 = match v0.clone() {
        Adt2::C0(_, h308, _, _, _, _, _, _, _, _, _) => {
            h308
        }
    };
    let a311 = match v1.clone() {
        Adt2::C0(_, h309, _, _, _, _, _, _, _, _, _) => {
            h309
        }
    };
    if if if nat_eq(a310, a311) {
        let a314 = match v0.clone() {
            Adt2::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                h312
            }
        };
        let a315 = match v1.clone() {
            Adt2::C0(_, _, h313, _, _, _, _, _, _, _, _) => {
                h313
            }
        };
        nat_eq(a314, a315)
    } else {
        false
    } {
        let a318 = match v0.clone() {
            Adt2::C0(_, _, _, h316, _, _, _, _, _, _, _) => {
                h316
            }
        };
        let a319 = match v1.clone() {
            Adt2::C0(_, _, _, h317, _, _, _, _, _, _, _) => {
                h317
            }
        };
        nat_eq(a318, a319)
    } else {
        false
    } {
        let a322 = match v0 {
            Adt2::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                h320
            }
        };
        let a323 = match v1 {
            Adt2::C0(_, _, _, _, h321, _, _, _, _, _, _) => {
                h321
            }
        };
        nat_eq(a322, a323)
    } else {
        false
    }
}

pub fn f15(v0: Adt2) -> R<bool> {
    let a325 = match v0.clone() {
        Adt2::C0(_, h324, _, _, _, _, _, _, _, _, _) => {
            h324
        }
    };
    let a326 = 1u64;
    if if if if nat_eq(a325, a326) {
        true
    } else {
        let a328 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h327, _, _, _, _, _) => {
                h327
            }
        };
        let a329 = 4u64;
        nat_eq(a328, a329)
    } {
        let a331 = match v0.clone() {
            Adt2::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        let a332 = 1u64;
        if nat_eq(a331, a332) {
            true
        } else {
            let a337 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h333, _, _, _, _) => {
                    h333
                }
            };
            let a338 = {
                let a335 = match v0.clone() {
                    Adt2::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                        h334
                    }
                };
                let a336 = 4u64;
                nat_mul(a335, a336)?
            };
            nat_eq(a337, a338)
        }
    } else {
        false
    } {
        let a340 = match v0.clone() {
            Adt2::C0(_, _, _, h339, _, _, _, _, _, _, _) => {
                h339
            }
        };
        let a341 = 1u64;
        if nat_eq(a340, a341) {
            true
        } else {
            let a349 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h342, _, _, _) => {
                    h342
                }
            };
            let a350 = {
                let a347 = {
                    let a345 = match v0.clone() {
                        Adt2::C0(_, h343, _, _, _, _, _, _, _, _, _) => {
                            h343
                        }
                    };
                    let a346 = match v0.clone() {
                        Adt2::C0(_, _, h344, _, _, _, _, _, _, _, _) => {
                            h344
                        }
                    };
                    nat_mul(a345, a346)?
                };
                let a348 = 4u64;
                nat_mul(a347, a348)?
            };
            nat_eq(a349, a350)
        }
    } else {
        false
    } {
        let a352 = match v0.clone() {
            Adt2::C0(_, _, _, _, h351, _, _, _, _, _, _) => {
                h351
            }
        };
        let a353 = 1u64;
        if nat_eq(a352, a353) {
            Ok(true)
        } else {
            let a364 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h354, _, _) => {
                    h354
                }
            };
            let a365 = {
                let a362 = {
                    let a360 = {
                        let a357 = match v0.clone() {
                            Adt2::C0(_, h355, _, _, _, _, _, _, _, _, _) => {
                                h355
                            }
                        };
                        let a358 = match v0.clone() {
                            Adt2::C0(_, _, h356, _, _, _, _, _, _, _, _) => {
                                h356
                            }
                        };
                        nat_mul(a357, a358)?
                    };
                    let a361 = match v0 {
                        Adt2::C0(_, _, _, h359, _, _, _, _, _, _, _) => {
                            h359
                        }
                    };
                    nat_mul(a360, a361)?
                };
                let a363 = 4u64;
                nat_mul(a362, a363)?
            };
            Ok(nat_eq(a364, a365))
        }
    } else {
        Ok(false)
    }
}

pub fn f16(v0: Adt2, v1: Adt2) -> R<bool> {
    let a367 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h366, _) => {
            h366
        }
    };
    let a368 = 0u64;
    if nat_eq(a367, a368) {
        Ok(true)
    } else {
        let a370 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h369, _) => {
                h369
            }
        };
        let a371 = 0u64;
        if nat_eq(a370, a371) {
            Ok(true)
        } else {
            let a373 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h372) => {
                    h372
                }
            };
            let a374 = 0u64;
            if nat_eq(a373, a374) {
                Ok(true)
            } else {
                let a376 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h375) => {
                        h375
                    }
                };
                let a377 = 0u64;
                if nat_eq(a376, a377) {
                    Ok(true)
                } else {
                    let a383 = {
                        let a380 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h378) => {
                                h378
                            }
                        };
                        let a381 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h379, _) => {
                                h379
                            }
                        };
                        nat_add(a380, a381)?
                    };
                    let a384 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h382) => {
                            h382
                        }
                    };
                    if nat_le(a383, a384) {
                        Ok(true)
                    } else {
                        let a390 = {
                            let a387 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h385) => {
                                    h385
                                }
                            };
                            let a388 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h386, _) => {
                                    h386
                                }
                            };
                            nat_add(a387, a388)?
                        };
                        let a391 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h389) => {
                                h389
                            }
                        };
                        Ok(nat_le(a390, a391))
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a396 = {
        let a392 = v0;
        let a393 = v1;
        nat_mul(a392, a393)?
    };
    let a397 = {
        let a394 = v2;
        let a395 = f59(v3);
        nat_mul(a394, a395)?
    };
    nat_mul(a396, a397)
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a400 = v0;
    let a401 = {
        let a398 = f60(v1)?;
        let a399 = v2;
        nat_mul(a398, a399)?
    };
    nat_add(a400, a401)
}

pub fn f19(v0: u64) -> u64 {
    let a402 = v0;
    let a403 = 30u64;
    if nat_eq(a402, a403) {
        2u64
    } else {
        4u64
    }
}

pub fn f20(v0: u64) -> bool {
    let a404 = v0;
    let a405 = 2139095040u64;
    if nat_le(a404, a405) {
        true
    } else {
        let a406 = v0;
        let a407 = 2147483648u64;
        nat_eq(a406, a407)
    }
}

pub fn f21(v0: Adt2, v1: u64) -> bool {
    let a409 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h408, _, _, _, _, _) => {
            h408
        }
    };
    let a410 = v1;
    if if nat_eq(a409, a410) {
        let a415 = {
            let a412 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h411) => {
                    h411
                }
            };
            let a413 = v1;
            let a414 = 0u64;
            nat_rem(a412, a413, a414)
        };
        let a416 = 0u64;
        nat_eq(a415, a416)
    } else {
        false
    } {
        f61(match v0.clone() {
            Adt2::C0(_, _, h417, _, _, _, _, _, _, _, _) => {
                h417
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h418, _, _, _, _, _, _, _) => {
                h418
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h419, _, _, _, _, _, _) => {
                h419
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h420, _, _, _, _) => {
                h420
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h421, _, _, _) => {
                h421
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h422, _, _) => {
                h422
            }
        }, v1)
    } else {
        false
    }
}

pub fn f22(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a425 = match v0.clone() {
            Adt2::C0(h423, _, _, _, _, _, _, _, _, _, _) => {
                h423
            }
        };
        let a426 = match v1.clone() {
            Adt2::C0(h424, _, _, _, _, _, _, _, _, _, _) => {
                h424
            }
        };
        nat_eq(a425, a426)
    } else {
        false
    } {
        let a429 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h427) => {
                h427
            }
        };
        let a430 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h428) => {
                h428
            }
        };
        nat_eq(a429, a430)
    } else {
        false
    } {
        let a433 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h431, _, _, _, _, _) => {
                h431
            }
        };
        let a434 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h432, _, _, _, _, _) => {
                h432
            }
        };
        nat_eq(a433, a434)
    } else {
        false
    } {
        let a437 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h435, _, _, _, _) => {
                h435
            }
        };
        let a438 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h436, _, _, _, _) => {
                h436
            }
        };
        nat_eq(a437, a438)
    } else {
        false
    } {
        let a441 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h439, _, _, _) => {
                h439
            }
        };
        let a442 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h440, _, _, _) => {
                h440
            }
        };
        nat_eq(a441, a442)
    } else {
        false
    } {
        let a445 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h443, _, _) => {
                h443
            }
        };
        let a446 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h444, _, _) => {
                h444
            }
        };
        nat_eq(a445, a446)
    } else {
        false
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a447 = v0;
    let a448 = 0u64;
    if if if nat_eq(a447, a448) {
        true
    } else {
        let a449 = v1;
        let a450 = 0u64;
        nat_eq(a449, a450)
    } {
        true
    } else {
        let a451 = v2;
        let a452 = 0u64;
        nat_eq(a451, a452)
    } {
        true
    } else {
        let a453 = v3;
        let a454 = 0u64;
        nat_eq(a453, a454)
    }
}

pub fn f24(v0: Adt2, v1: u64) -> R<bool> {
    f62({
        let a456 = match v0.clone() {
            Adt2::C0(_, h455, _, _, _, _, _, _, _, _, _) => {
                h455
            }
        };
        let a457 = v1;
        nat_mul(a456, a457)?
    }, match v0.clone() {
        Adt2::C0(_, _, h458, _, _, _, _, _, _, _, _) => {
            h458
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h459, _, _, _, _, _, _, _) => {
            h459
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h460, _, _, _, _, _, _) => {
            h460
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h461, _, _, _, _) => {
            h461
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h462, _, _, _) => {
            h462
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h463, _, _) => {
            h463
        }
    })
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a464 = v1;
    let a465 = 0u64;
    if if nat_eq(a464, a465) {
        true
    } else {
        let a466 = v2;
        let a467 = 0u64;
        nat_eq(a466, a467)
    } {
        Ok(0u64)
    } else {
        let a474 = {
            let a468 = f63(v1, v2, v3)?;
            let a469 = f64(v0, v1, v2)?;
            nat_add(a468, a469)?
        };
        let a475 = {
            let a472 = f65(v0);
            let a473 = {
                let a470 = v1;
                let a471 = v2;
                nat_mul(a470, a471)?
            };
            nat_mul(a472, a473)?
        };
        nat_add(a474, a475)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a476 = v0;
    let a477 = 0u64;
    if if nat_eq(a476, a477) {
        true
    } else {
        let a478 = v1;
        let a479 = 0u64;
        nat_eq(a478, a479)
    } {
        Ok(3u64)
    } else {
        let a482 = {
            let a480 = 3u64;
            let a481 = v1;
            nat_add(a480, a481)?
        };
        let a483 = v2;
        nat_add(a482, a483)
    }
}

pub fn f27(v0: u64) -> bool {
    let a484 = v0;
    let a485 = 2139095040u64;
    if nat_le(a484, a485) {
        true
    } else {
        let a486 = v0;
        let a487 = 2147483648u64;
        nat_eq(a486, a487)
    }
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let a488 = v0;
    let a489 = 0u64;
    if nat_eq(a488, a489) {
        Ok(0u64)
    } else {
        let a490 = v1;
        let a491 = f66(v0)?;
        nat_mul(a490, a491)
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a492 = v0;
    let a493 = 0u64;
    if if if if nat_eq(a492, a493) {
        true
    } else {
        let a494 = v1;
        let a495 = 0u64;
        nat_eq(a494, a495)
    } {
        true
    } else {
        let a496 = v2;
        let a497 = 0u64;
        nat_eq(a496, a497)
    } {
        true
    } else {
        let a498 = v3;
        let a499 = 0u64;
        nat_eq(a498, a499)
    } {
        Ok(0u64)
    } else {
        let a512 = {
            let a502 = 2u64;
            let a503 = {
                let a500 = v1;
                let a501 = v2;
                nat_mul(a500, a501)?
            };
            nat_mul(a502, a503)?
        };
        let a513 = {
            let a510 = {
                let a506 = {
                    let a504 = v1;
                    let a505 = v2;
                    nat_mul(a504, a505)?
                };
                let a507 = v3;
                nat_mul(a506, a507)?
            };
            let a511 = {
                let a508 = 1u64;
                let a509 = v0;
                nat_add(a508, a509)?
            };
            nat_mul(a510, a511)?
        };
        nat_add(a512, a513)
    }
}

pub fn f30(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m514 = v1;
    match m514 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f67(v0, v3, v2)
        }
    }
}

pub fn f31(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a516 = match v0.clone() {
        Adt2::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
            h515
        }
    };
    let a517 = 1u64;
    if if nat_eq(a516, a517) {
        true
    } else {
        let a519 = match v0.clone() {
            Adt2::C0(h518, _, _, _, _, _, _, _, _, _, _) => {
                h518
            }
        };
        let a520 = 30u64;
        nat_eq(a519, a520)
    } {
        let a522 = match v1.clone() {
            Adt2::C0(h521, _, _, _, _, _, _, _, _, _, _) => {
                h521
            }
        };
        let a523 = 1u64;
        if if if if if if if if if if if nat_eq(a522, a523) {
            let a526 = match v0.clone() {
                Adt2::C0(_, h524, _, _, _, _, _, _, _, _, _) => {
                    h524
                }
            };
            let a527 = match v1.clone() {
                Adt2::C0(_, h525, _, _, _, _, _, _, _, _, _) => {
                    h525
                }
            };
            nat_eq(a526, a527)
        } else {
            false
        } {
            let a529 = 0u64;
            let a530 = match v1.clone() {
                Adt2::C0(_, _, h528, _, _, _, _, _, _, _, _) => {
                    h528
                }
            };
            nat_lt(a529, a530)
        } else {
            false
        } {
            let a533 = match v1.clone() {
                Adt2::C0(_, _, h531, _, _, _, _, _, _, _, _) => {
                    h531
                }
            };
            let a534 = match v1.clone() {
                Adt2::C0(_, _, _, h532, _, _, _, _, _, _, _) => {
                    h532
                }
            };
            nat_eq(a533, a534)
        } else {
            false
        } {
            let a536 = match v1.clone() {
                Adt2::C0(_, _, _, _, h535, _, _, _, _, _, _) => {
                    h535
                }
            };
            let a537 = 1u64;
            nat_eq(a536, a537)
        } else {
            false
        } {
            let a545 = match v0.clone() {
                Adt2::C0(_, _, h538, _, _, _, _, _, _, _, _) => {
                    h538
                }
            };
            let a546 = {
                let a543 = {
                    let a541 = match v1.clone() {
                        Adt2::C0(_, _, h539, _, _, _, _, _, _, _, _) => {
                            h539
                        }
                    };
                    let a542 = match v1.clone() {
                        Adt2::C0(_, _, h540, _, _, _, _, _, _, _, _) => {
                            h540
                        }
                    };
                    nat_add(a541, a542)?
                };
                let a544 = 1u64;
                nat_sub(a543, a544)
            };
            nat_eq(a545, a546)
        } else {
            false
        } {
            let a548 = 0u64;
            let a549 = match v0.clone() {
                Adt2::C0(_, _, _, h547, _, _, _, _, _, _, _) => {
                    h547
                }
            };
            nat_lt(a548, a549)
        } else {
            false
        } {
            let a551 = 0u64;
            let a552 = match v0.clone() {
                Adt2::C0(_, _, _, _, h550, _, _, _, _, _, _) => {
                    h550
                }
            };
            nat_lt(a551, a552)
        } else {
            false
        } {
            let a554 = 2u64;
            let a555 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h553, _, _, _, _, _) => {
                    h553
                }
            };
            nat_le(a554, a555)
        } else {
            false
        } {
            let a557 = 2u64;
            let a558 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h556, _, _, _, _, _) => {
                    h556
                }
            };
            nat_le(a557, a558)
        } else {
            false
        } {
            f16(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f9(v1.clone(), 1u64)?, 2u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f32(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m559 = v1;
    match m559 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a561 = match v0.clone() {
                Adt2::C0(h560, _, _, _, _, _, _, _, _, _, _) => {
                    h560
                }
            };
            let a562 = 0u64;
            if if if if if if if if if if if if if nat_eq(a561, a562) {
                true
            } else {
                let a564 = match v0.clone() {
                    Adt2::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
                        h563
                    }
                };
                let a565 = 1u64;
                nat_eq(a564, a565)
            } {
                let a567 = match v3.clone() {
                    Adt2::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
                        h566
                    }
                };
                let a568 = 26u64;
                nat_eq(a567, a568)
            } else {
                false
            } {
                let a570 = match v2.clone() {
                    Adt2::C0(h569, _, _, _, _, _, _, _, _, _, _) => {
                        h569
                    }
                };
                let a571 = 0u64;
                nat_eq(a570, a571)
            } else {
                false
            } {
                let a574 = match v0.clone() {
                    Adt2::C0(_, h572, _, _, _, _, _, _, _, _, _) => {
                        h572
                    }
                };
                let a575 = match v2.clone() {
                    Adt2::C0(_, h573, _, _, _, _, _, _, _, _, _) => {
                        h573
                    }
                };
                nat_eq(a574, a575)
            } else {
                false
            } {
                let a578 = match v3.clone() {
                    Adt2::C0(_, h576, _, _, _, _, _, _, _, _, _) => {
                        h576
                    }
                };
                let a579 = match v0.clone() {
                    Adt2::C0(_, _, h577, _, _, _, _, _, _, _, _) => {
                        h577
                    }
                };
                nat_le(a578, a579)
            } else {
                false
            } {
                let a581 = match v0.clone() {
                    Adt2::C0(_, _, _, h580, _, _, _, _, _, _, _) => {
                        h580
                    }
                };
                let a582 = 1u64;
                nat_eq(a581, a582)
            } else {
                false
            } {
                let a584 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h583, _, _, _, _, _, _) => {
                        h583
                    }
                };
                let a585 = 1u64;
                nat_eq(a584, a585)
            } else {
                false
            } {
                let a587 = match v2.clone() {
                    Adt2::C0(_, _, _, h586, _, _, _, _, _, _, _) => {
                        h586
                    }
                };
                let a588 = 1u64;
                nat_eq(a587, a588)
            } else {
                false
            } {
                let a590 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h589, _, _, _, _, _, _) => {
                        h589
                    }
                };
                let a591 = 1u64;
                nat_eq(a590, a591)
            } else {
                false
            } {
                let a593 = match v3.clone() {
                    Adt2::C0(_, _, h592, _, _, _, _, _, _, _, _) => {
                        h592
                    }
                };
                let a594 = 1u64;
                nat_eq(a593, a594)
            } else {
                false
            } {
                let a596 = match v3.clone() {
                    Adt2::C0(_, _, _, h595, _, _, _, _, _, _, _) => {
                        h595
                    }
                };
                let a597 = 1u64;
                nat_eq(a596, a597)
            } else {
                false
            } {
                let a599 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h598, _, _, _, _, _, _) => {
                        h598
                    }
                };
                let a600 = 1u64;
                nat_eq(a599, a600)
            } else {
                false
            } {
                if f23(match v2.clone() {
                    Adt2::C0(_, h601, _, _, _, _, _, _, _, _, _) => {
                        h601
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h602, _, _, _, _, _, _, _, _) => {
                        h602
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h603, _, _, _, _, _, _, _) => {
                        h603
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h604, _, _, _, _, _, _) => {
                        h604
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a606 = match v3.clone() {
                        Adt2::C0(_, h605, _, _, _, _, _, _, _, _, _) => {
                            h605
                        }
                    };
                    let a607 = 0u64;
                    if nat_eq(a606, a607) {
                        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a609 = match v0.clone() {
                            Adt2::C0(_, h608, _, _, _, _, _, _, _, _, _) => {
                                h608
                            }
                        };
                        let a610 = 2147483647u64;
                        if if if if if if nat_le(a609, a610) {
                            let a612 = match v3.clone() {
                                Adt2::C0(_, h611, _, _, _, _, _, _, _, _, _) => {
                                    h611
                                }
                            };
                            let a613 = 2147483647u64;
                            nat_le(a612, a613)
                        } else {
                            false
                        } {
                            let a615 = 4u64;
                            let a616 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h614, _, _, _, _, _) => {
                                    h614
                                }
                            };
                            nat_le(a615, a616)
                        } else {
                            false
                        } {
                            let a622 = {
                                let a619 = match v2.clone() {
                                    Adt2::C0(_, h617, _, _, _, _, _, _, _, _, _) => {
                                        h617
                                    }
                                };
                                let a620 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h618, _, _, _, _, _) => {
                                        h618
                                    }
                                };
                                nat_mul(a619, a620)?
                            };
                            let a623 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h621, _, _, _, _) => {
                                    h621
                                }
                            };
                            nat_le(a622, a623)
                        } else {
                            false
                        } {
                            f16(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f16(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a637 = {
                                    let a627 = {
                                        let a625 = 2u64;
                                        let a626 = match v3.clone() {
                                            Adt2::C0(_, h624, _, _, _, _, _, _, _, _, _) => {
                                                h624
                                            }
                                        };
                                        nat_mul(a625, a626)?
                                    };
                                    let a628 = f9(v2.clone(), 1u64)?;
                                    nat_add(a627, a628)?
                                };
                                let a638 = {
                                    let a635 = match v3.clone() {
                                        Adt2::C0(_, h629, _, _, _, _, _, _, _, _, _) => {
                                            h629
                                        }
                                    };
                                    let a636 = {
                                        let a633 = 1u64;
                                        let a634 = {
                                            let a631 = 2u64;
                                            let a632 = match v0.clone() {
                                                Adt2::C0(_, h630, _, _, _, _, _, _, _, _, _) => {
                                                    h630
                                                }
                                            };
                                            nat_mul(a631, a632)?
                                        };
                                        nat_add(a633, a634)?
                                    };
                                    nat_mul(a635, a636)?
                                };
                                nat_add(a637, a638)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h639, _, _, _, _, _, _, _, _, _) => {
                                    h639
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h640, _, _, _, _, _, _, _, _, _) => {
                                    h640
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h641, _, _, _, _, _) => {
                                    h641
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h642, _, _, _, _) => {
                                    h642
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h643, _, _, _) => {
                                    h643
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h644, _, _) => {
                                    h644
                                }
                            }), f13(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h645, _, _, _, _, _, _, _, _, _, _) => {
                                    h645
                                }
                            }, f13(v3, 1u64))))
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

pub fn f33(v0: u64) -> bool {
    let a646 = v0;
    let a647 = 1u64;
    if if nat_eq(a646, a647) {
        true
    } else {
        let a648 = v0;
        let a649 = 33u64;
        nat_eq(a648, a649)
    } {
        true
    } else {
        let a650 = v0;
        let a651 = 34u64;
        nat_eq(a650, a651)
    }
}

pub fn f34(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a654 = match v1.clone() {
        Adt2::C0(h652, _, _, _, _, _, _, _, _, _, _) => {
            h652
        }
    };
    let a655 = match v3.clone() {
        Adt2::C0(h653, _, _, _, _, _, _, _, _, _, _) => {
            h653
        }
    };
    if nat_eq(a654, a655) {
        f68(v0, v1, v2, v3)
    } else {
        if if if if f69(match v1.clone() {
            Adt2::C0(h656, _, _, _, _, _, _, _, _, _, _) => {
                h656
            }
        }) {
            f69(match v3.clone() {
                Adt2::C0(h657, _, _, _, _, _, _, _, _, _, _) => {
                    h657
                }
            })
        } else {
            false
        } {
            let a658 = v0;
            let a659 = 33u64;
            if nat_eq(a658, a659) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a660 = f9(v1.clone(), 1u64)?;
            let a661 = f9(v3.clone(), 1u64)?;
            nat_eq(a660, a661)
        } else {
            false
        } {
            let m663 = f70(match v3.clone() {
                Adt2::C0(h662, _, _, _, _, _, _, _, _, _, _) => {
                    h662
                }
            });
            match m663 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h664) => {
                            h664
                        }
                    }, f13(v1.clone(), 1u64), f13(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h665, _, _, _, _, _, _, _, _, _, _) => {
                            h665
                        }
                    }, match v3 {
                        Adt2::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
                            h666
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f35(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a668 = match v0.clone() {
        Adt2::C0(h667, _, _, _, _, _, _, _, _, _, _) => {
            h667
        }
    };
    let a669 = 0u64;
    if if if if nat_eq(a668, a669) {
        true
    } else {
        let a671 = match v0.clone() {
            Adt2::C0(h670, _, _, _, _, _, _, _, _, _, _) => {
                h670
            }
        };
        let a672 = 1u64;
        nat_eq(a671, a672)
    } {
        true
    } else {
        let a674 = match v0.clone() {
            Adt2::C0(h673, _, _, _, _, _, _, _, _, _, _) => {
                h673
            }
        };
        let a675 = 25u64;
        if nat_eq(a674, a675) {
            true
        } else {
            let a677 = match v0.clone() {
                Adt2::C0(h676, _, _, _, _, _, _, _, _, _, _) => {
                    h676
                }
            };
            let a678 = 26u64;
            nat_eq(a677, a678)
        }
    } {
        true
    } else {
        let a680 = match v0.clone() {
            Adt2::C0(h679, _, _, _, _, _, _, _, _, _, _) => {
                h679
            }
        };
        let a681 = 30u64;
        nat_eq(a680, a681)
    } {
        let a684 = match v0.clone() {
            Adt2::C0(h682, _, _, _, _, _, _, _, _, _, _) => {
                h682
            }
        };
        let a685 = match v1.clone() {
            Adt2::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                h683
            }
        };
        if if nat_eq(a684, a685) {
            f71(match v1.clone() {
                Adt2::C0(_, h686, _, _, _, _, _, _, _, _, _) => {
                    h686
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h687, _, _, _, _, _, _, _, _) => {
                    h687
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h688, _, _, _, _, _, _, _) => {
                    h688
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h689, _, _, _, _, _, _) => {
                    h689
                }
            }, match v0.clone() {
                Adt2::C0(_, h690, _, _, _, _, _, _, _, _, _) => {
                    h690
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h691, _, _, _, _, _, _, _, _) => {
                    h691
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h692, _, _, _, _, _, _, _) => {
                    h692
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h693, _, _, _, _, _, _) => {
                    h693
                }
            })
        } else {
            false
        } {
            let m695 = f70(match v0.clone() {
                Adt2::C0(h694, _, _, _, _, _, _, _, _, _, _) => {
                    h694
                }
            });
            match m695 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f9(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h696) => {
                            h696
                        }
                    }, f13(v0, 1u64), f13(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f36(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m697 = v2;
    match m697 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a698 = v0;
            let a699 = 4u64;
            if if if nat_lt(a698, a699) {
                let a702 = match v1.clone() {
                    Adt2::C0(h700, _, _, _, _, _, _, _, _, _, _) => {
                        h700
                    }
                };
                let a703 = match v4.clone() {
                    Adt2::C0(h701, _, _, _, _, _, _, _, _, _, _) => {
                        h701
                    }
                };
                if nat_eq(a702, a703) {
                    let a706 = match v1.clone() {
                        Adt2::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
                            h704
                        }
                    };
                    let a707 = match v3.clone() {
                        Adt2::C0(h705, _, _, _, _, _, _, _, _, _, _) => {
                            h705
                        }
                    };
                    nat_eq(a706, a707)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f72(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h708, _, _, _, _, _, _, _, _, _) => {
                        h708
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h709, _, _, _, _, _, _, _, _, _) => {
                        h709
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h710, _, _, _, _, _, _, _, _, _) => {
                        h710
                    }
                })? {
                    f72(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h711, _, _, _, _, _, _, _, _) => {
                            h711
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h712, _, _, _, _, _, _, _, _) => {
                            h712
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h713, _, _, _, _, _, _, _, _) => {
                            h713
                        }
                    })?
                } else {
                    false
                } {
                    f72(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h714, _, _, _, _, _, _, _) => {
                            h714
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h715, _, _, _, _, _, _, _) => {
                            h715
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h716, _, _, _, _, _, _, _) => {
                            h716
                        }
                    })?
                } else {
                    false
                } {
                    f72(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h717, _, _, _, _, _, _) => {
                            h717
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h718, _, _, _, _, _, _) => {
                            h718
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h719, _, _, _, _, _, _) => {
                            h719
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m721 = f70(match v1.clone() {
                    Adt2::C0(h720, _, _, _, _, _, _, _, _, _, _) => {
                        h720
                    }
                });
                match m721 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h722, _) => {
                                h722
                            }
                        };
                        let a727 = {
                            let a724 = match v1.clone() {
                                Adt2::C0(_, h723, _, _, _, _, _, _, _, _, _) => {
                                    h723
                                }
                            };
                            let a725 = v6;
                            let a726 = 1u64;
                            nat_rem(a724, a725, a726)
                        };
                        let a728 = 0u64;
                        if if if nat_eq(a727, a728) {
                            let a733 = {
                                let a730 = match v4.clone() {
                                    Adt2::C0(_, h729, _, _, _, _, _, _, _, _, _) => {
                                        h729
                                    }
                                };
                                let a731 = v6;
                                let a732 = 1u64;
                                nat_rem(a730, a731, a732)
                            };
                            let a734 = 0u64;
                            nat_eq(a733, a734)
                        } else {
                            false
                        } {
                            let a739 = {
                                let a736 = match v3.clone() {
                                    Adt2::C0(_, h735, _, _, _, _, _, _, _, _, _) => {
                                        h735
                                    }
                                };
                                let a737 = v6;
                                let a738 = 1u64;
                                nat_rem(a736, a737, a738)
                            };
                            let a740 = 0u64;
                            nat_eq(a739, a740)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f9(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h741) => {
                                    h741
                                }
                            }, f13(v1, v6), f13(v3, v6), Adt4::C2(v0, f13(v4, v6))))
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

pub fn f37(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a743 = match v2.clone() {
        Adt2::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
            h742
        }
    };
    let a744 = 0u64;
    if if nat_eq(a743, a744) {
        let a746 = match v3.clone() {
            Adt2::C0(h745, _, _, _, _, _, _, _, _, _, _) => {
                h745
            }
        };
        let a747 = 0u64;
        nat_eq(a746, a747)
    } else {
        false
    } {
        let a748 = v0;
        let a749 = 61u64;
        if nat_eq(a748, a749) {
            let v4: bool = {
                let a751 = 0u64;
                let a752 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h750, _, _, _, _, _, _, _) => {
                        h750
                    }
                };
                nat_lt(a751, a752)
            };
            if if if if if if f73(match v2.clone() {
                Adt2::C0(_, h753, _, _, _, _, _, _, _, _, _) => {
                    h753
                }
            }, match v1.clone() {
                Adt1::C0(h754, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h754
                }
            }, match v1.clone() {
                Adt1::C0(_, h755, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h755
                }
            }, match v3.clone() {
                Adt2::C0(_, h756, _, _, _, _, _, _, _, _, _) => {
                    h756
                }
            })? {
                f73(match v2.clone() {
                    Adt2::C0(_, _, h757, _, _, _, _, _, _, _, _) => {
                        h757
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h758, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h758
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h759, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h759
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h760, _, _, _, _, _, _, _, _) => {
                        h760
                    }
                })?
            } else {
                false
            } {
                f73(match v2.clone() {
                    Adt2::C0(_, _, _, h761, _, _, _, _, _, _, _) => {
                        h761
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h762, _, _, _, _, _, _, _, _, _, _, _) => {
                        h762
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h763, _, _, _, _, _, _, _, _, _, _) => {
                        h763
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h764, _, _, _, _, _, _, _) => {
                        h764
                    }
                })?
            } else {
                false
            } {
                f73(match v2.clone() {
                    Adt2::C0(_, _, _, _, h765, _, _, _, _, _, _) => {
                        h765
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h766, _, _, _, _, _, _, _, _, _) => {
                        h766
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h767, _, _, _, _, _, _, _, _) => {
                        h767
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h768, _, _, _, _, _, _) => {
                        h768
                    }
                })?
            } else {
                false
            } {
                let a770 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h769, _, _, _, _, _, _, _) => {
                        h769
                    }
                };
                let a771 = 4294967295u64;
                nat_le(a770, a771)
            } else {
                false
            } {
                if v4 {
                    !f23(match v2.clone() {
                        Adt2::C0(_, h772, _, _, _, _, _, _, _, _, _) => {
                            h772
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h773, _, _, _, _, _, _, _, _) => {
                            h773
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h774, _, _, _, _, _, _, _) => {
                            h774
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h775, _, _, _, _, _, _) => {
                            h775
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h776, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h776
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h777, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h777
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h778, _, _, _, _, _, _, _, _, _, _, _) => {
                        h778
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h779, _, _, _, _, _, _, _, _, _) => {
                        h779
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a780 = v0;
            let a781 = 62u64;
            if nat_eq(a780, a781) {
                let a783 = match v1.clone() {
                    Adt1::C0(h782, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h782
                    }
                };
                let a784 = 2147483648u64;
                if if if if if nat_lt(a783, a784) {
                    let a786 = match v1.clone() {
                        Adt1::C0(_, h785, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h785
                        }
                    };
                    let a787 = 2147483648u64;
                    nat_lt(a786, a787)
                } else {
                    false
                } {
                    let a790 = match v1.clone() {
                        Adt1::C0(h788, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h788
                        }
                    };
                    let a791 = match v2.clone() {
                        Adt2::C0(_, h789, _, _, _, _, _, _, _, _, _) => {
                            h789
                        }
                    };
                    nat_lt(a790, a791)
                } else {
                    false
                } {
                    let a794 = match v1.clone() {
                        Adt1::C0(_, h792, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h792
                        }
                    };
                    let a795 = match v2.clone() {
                        Adt2::C0(_, h793, _, _, _, _, _, _, _, _, _) => {
                            h793
                        }
                    };
                    nat_lt(a794, a795)
                } else {
                    false
                } {
                    let a798 = match v2.clone() {
                        Adt2::C0(_, _, h796, _, _, _, _, _, _, _, _) => {
                            h796
                        }
                    };
                    let a799 = match v3.clone() {
                        Adt2::C0(_, _, h797, _, _, _, _, _, _, _, _) => {
                            h797
                        }
                    };
                    if if if nat_eq(a798, a799) {
                        let a802 = match v2.clone() {
                            Adt2::C0(_, _, _, h800, _, _, _, _, _, _, _) => {
                                h800
                            }
                        };
                        let a803 = match v3.clone() {
                            Adt2::C0(_, _, _, h801, _, _, _, _, _, _, _) => {
                                h801
                            }
                        };
                        nat_eq(a802, a803)
                    } else {
                        false
                    } {
                        let a806 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h804, _, _, _, _, _, _) => {
                                h804
                            }
                        };
                        let a807 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h805, _, _, _, _, _, _) => {
                                h805
                            }
                        };
                        nat_eq(a806, a807)
                    } else {
                        false
                    } {
                        let a816 = match v3.clone() {
                            Adt2::C0(_, h808, _, _, _, _, _, _, _, _, _) => {
                                h808
                            }
                        };
                        let a817 = {
                            let a814 = {
                                let a811 = match v2.clone() {
                                    Adt2::C0(_, h809, _, _, _, _, _, _, _, _, _) => {
                                        h809
                                    }
                                };
                                let a812 = match v1.clone() {
                                    Adt1::C0(h810, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h810
                                    }
                                };
                                nat_add(a811, a812)?
                            };
                            let a815 = match v1.clone() {
                                Adt1::C0(_, h813, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h813
                                }
                            };
                            nat_add(a814, a815)?
                        };
                        nat_eq(a816, a817)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h818, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h818
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a819 = v0;
                let a820 = 63u64;
                if nat_eq(a819, a820) {
                    if if if if if f74(match v2.clone() {
                        Adt2::C0(_, h821, _, _, _, _, _, _, _, _, _) => {
                            h821
                        }
                    }, match v1.clone() {
                        Adt1::C0(h822, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h822
                        }
                    })? {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, h823, _, _, _, _, _, _, _, _) => {
                                h823
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h824, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h824
                            }
                        })?
                    } else {
                        false
                    } {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, _, h825, _, _, _, _, _, _, _) => {
                                h825
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h826, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h826
                            }
                        })?
                    } else {
                        false
                    } {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, _, _, h827, _, _, _, _, _, _) => {
                                h827
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h828, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h828
                            }
                        })?
                    } else {
                        false
                    } {
                        let a831 = match v2.clone() {
                            Adt2::C0(_, h829, _, _, _, _, _, _, _, _, _) => {
                                h829
                            }
                        };
                        let a832 = match v3.clone() {
                            Adt2::C0(_, h830, _, _, _, _, _, _, _, _, _) => {
                                h830
                            }
                        };
                        if if if nat_eq(a831, a832) {
                            let a835 = match v2.clone() {
                                Adt2::C0(_, _, h833, _, _, _, _, _, _, _, _) => {
                                    h833
                                }
                            };
                            let a836 = match v3.clone() {
                                Adt2::C0(_, _, h834, _, _, _, _, _, _, _, _) => {
                                    h834
                                }
                            };
                            nat_eq(a835, a836)
                        } else {
                            false
                        } {
                            let a839 = match v2.clone() {
                                Adt2::C0(_, _, _, h837, _, _, _, _, _, _, _) => {
                                    h837
                                }
                            };
                            let a840 = match v3.clone() {
                                Adt2::C0(_, _, _, h838, _, _, _, _, _, _, _) => {
                                    h838
                                }
                            };
                            nat_eq(a839, a840)
                        } else {
                            false
                        } {
                            let a843 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h841, _, _, _, _, _, _) => {
                                    h841
                                }
                            };
                            let a844 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h842, _, _, _, _, _, _) => {
                                    h842
                                }
                            };
                            nat_eq(a843, a844)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h845, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h845
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h846, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h846
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h847, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h847
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h848, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h848
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

pub fn f38(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m849 = v3;
    match m849 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a850 = v0;
                let a851 = 4u64;
                nat_eq(a850, a851)
            };
            let v7: bool = {
                let a853 = 0u64;
                let a854 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h852, _, _, _, _, _, _, _, _, _, _, _) => {
                        h852
                    }
                };
                nat_lt(a853, a854)
            };
            let v8: u64 = f9(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h855, _, _, _, _, _, _, _, _, _) => {
                    h855
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h856, _, _, _, _, _, _, _, _) => {
                    h856
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h857, _, _, _, _, _, _, _) => {
                    h857
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h858, _, _, _, _, _, _) => {
                    h858
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h859, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h859
                }
            }, match v1.clone() {
                Adt1::C0(_, h860, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h860
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h861, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h861
                }
            });
            if if if if if if if if if if if if v6 {
                let a863 = match v2.clone() {
                    Adt2::C0(h862, _, _, _, _, _, _, _, _, _, _) => {
                        h862
                    }
                };
                let a864 = 0u64;
                if nat_eq(a863, a864) {
                    let a866 = match v5.clone() {
                        Adt2::C0(h865, _, _, _, _, _, _, _, _, _, _) => {
                            h865
                        }
                    };
                    let a867 = 0u64;
                    nat_eq(a866, a867)
                } else {
                    false
                }
            } else {
                let a869 = match v2.clone() {
                    Adt2::C0(h868, _, _, _, _, _, _, _, _, _, _) => {
                        h868
                    }
                };
                let a870 = 0u64;
                if if nat_eq(a869, a870) {
                    true
                } else {
                    let a872 = match v2.clone() {
                        Adt2::C0(h871, _, _, _, _, _, _, _, _, _, _) => {
                            h871
                        }
                    };
                    let a873 = 26u64;
                    nat_eq(a872, a873)
                } {
                    let a875 = match v5.clone() {
                        Adt2::C0(h874, _, _, _, _, _, _, _, _, _, _) => {
                            h874
                        }
                    };
                    let a876 = 0u64;
                    if nat_eq(a875, a876) {
                        true
                    } else {
                        let a878 = match v5.clone() {
                            Adt2::C0(h877, _, _, _, _, _, _, _, _, _, _) => {
                                h877
                            }
                        };
                        let a879 = 26u64;
                        nat_eq(a878, a879)
                    }
                } else {
                    false
                }
            } {
                let a882 = match v2.clone() {
                    Adt2::C0(h880, _, _, _, _, _, _, _, _, _, _) => {
                        h880
                    }
                };
                let a883 = match v4.clone() {
                    Adt2::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
                        h881
                    }
                };
                nat_eq(a882, a883)
            } else {
                false
            } {
                f14(v2.clone(), v4.clone())
            } else {
                false
            } {
                f15(v2.clone())?
            } else {
                false
            } {
                f15(v4.clone())?
            } else {
                false
            } {
                let a885 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h884, _, _, _, _, _) => {
                        h884
                    }
                };
                let a886 = 4u64;
                nat_eq(a885, a886)
            } else {
                false
            } {
                let a887 = v8;
                let a888 = f9(v2.clone(), 1u64)?;
                nat_le(a887, a888)
            } else {
                false
            } {
                let a890 = match v1.clone() {
                    Adt1::C0(_, _, _, h889, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h889
                    }
                };
                let a891 = 1073741824u64;
                nat_lt(a890, a891)
            } else {
                false
            } {
                let a893 = match v1.clone() {
                    Adt1::C0(h892, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h892
                    }
                };
                let a894 = 4294967295u64;
                if if if if nat_le(a893, a894) {
                    let a896 = match v1.clone() {
                        Adt1::C0(_, h895, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h895
                        }
                    };
                    let a897 = 4294967295u64;
                    nat_le(a896, a897)
                } else {
                    false
                } {
                    let a899 = match v1.clone() {
                        Adt1::C0(_, _, h898, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h898
                        }
                    };
                    let a900 = 4294967295u64;
                    nat_le(a899, a900)
                } else {
                    false
                } {
                    let a902 = match v1.clone() {
                        Adt1::C0(_, _, _, h901, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h901
                        }
                    };
                    let a903 = 4294967295u64;
                    nat_le(a902, a903)
                } else {
                    false
                } {
                    let a905 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h904, _, _, _, _, _, _, _, _, _, _, _) => {
                            h904
                        }
                    };
                    let a906 = 4294967295u64;
                    nat_le(a905, a906)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a909 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h907) => {
                            h907
                        }
                    };
                    let a910 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h908) => {
                            h908
                        }
                    };
                    nat_eq(a909, a910)
                } else {
                    true
                }
            } else {
                false
            } {
                f75(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h911, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h911
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h912, _) => {
                        h912
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f9(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a913 = v10;
                    let a914 = v8;
                    nat_add(a913, a914)?
                }, 4u64, f13(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h915, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h915
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f39(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a917 = match v0.clone() {
        Adt2::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
            h916
        }
    };
    let a918 = 0u64;
    if if if nat_eq(a917, a918) {
        let a920 = match v1.clone() {
            Adt2::C0(h919, _, _, _, _, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = 0u64;
        nat_eq(a920, a921)
    } else {
        false
    } {
        f71(match v0.clone() {
            Adt2::C0(_, h922, _, _, _, _, _, _, _, _, _) => {
                h922
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h923, _, _, _, _, _, _, _, _) => {
                h923
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h924, _, _, _, _, _, _, _) => {
                h924
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h925, _, _, _, _, _, _) => {
                h925
            }
        }, match v1.clone() {
            Adt2::C0(_, h926, _, _, _, _, _, _, _, _, _) => {
                h926
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h927, _, _, _, _, _, _, _, _) => {
                h927
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                h928
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h929, _, _, _, _, _, _) => {
                h929
            }
        })
    } else {
        false
    } {
        let v2: u64 = f9(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a930 = v2;
            let a931 = f9(v0.clone(), 1u64)?;
            nat_add(a930, a931)?
        }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f40(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a933 = match v1.clone() {
        Adt2::C0(h932, _, _, _, _, _, _, _, _, _, _) => {
            h932
        }
    };
    let a934 = 0u64;
    if if if nat_eq(a933, a934) {
        true
    } else {
        let a936 = match v1.clone() {
            Adt2::C0(h935, _, _, _, _, _, _, _, _, _, _) => {
                h935
            }
        };
        let a937 = 1u64;
        nat_eq(a936, a937)
    } {
        true
    } else {
        let a939 = match v1.clone() {
            Adt2::C0(h938, _, _, _, _, _, _, _, _, _, _) => {
                h938
            }
        };
        let a940 = 30u64;
        nat_eq(a939, a940)
    } {
        let a943 = match v1.clone() {
            Adt2::C0(h941, _, _, _, _, _, _, _, _, _, _) => {
                h941
            }
        };
        let a944 = match v2.clone() {
            Adt2::C0(h942, _, _, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        if if if if if nat_eq(a943, a944) {
            let a947 = match v1.clone() {
                Adt2::C0(_, h945, _, _, _, _, _, _, _, _, _) => {
                    h945
                }
            };
            let a948 = match v2.clone() {
                Adt2::C0(_, h946, _, _, _, _, _, _, _, _, _) => {
                    h946
                }
            };
            nat_eq(a947, a948)
        } else {
            false
        } {
            let a951 = match v1.clone() {
                Adt2::C0(_, _, _, h949, _, _, _, _, _, _, _) => {
                    h949
                }
            };
            let a952 = match v2.clone() {
                Adt2::C0(_, _, _, h950, _, _, _, _, _, _, _) => {
                    h950
                }
            };
            nat_eq(a951, a952)
        } else {
            false
        } {
            let a955 = match v1.clone() {
                Adt2::C0(_, _, _, _, h953, _, _, _, _, _, _) => {
                    h953
                }
            };
            let a956 = match v2.clone() {
                Adt2::C0(_, _, _, _, h954, _, _, _, _, _, _) => {
                    h954
                }
            };
            nat_eq(a955, a956)
        } else {
            false
        } {
            let a957 = v0;
            let a958 = 4u64;
            if nat_eq(a957, a958) {
                let a960 = match v1.clone() {
                    Adt2::C0(_, _, h959, _, _, _, _, _, _, _, _) => {
                        h959
                    }
                };
                let a961 = 1u64;
                if nat_eq(a960, a961) {
                    let a964 = match v1.clone() {
                        Adt2::C0(_, h962, _, _, _, _, _, _, _, _, _) => {
                            h962
                        }
                    };
                    let a965 = match v2.clone() {
                        Adt2::C0(_, _, h963, _, _, _, _, _, _, _, _) => {
                            h963
                        }
                    };
                    nat_eq(a964, a965)
                } else {
                    false
                }
            } else {
                let a966 = v0;
                let a967 = 4u64;
                if if nat_lt(a966, a967) {
                    let a970 = match v1.clone() {
                        Adt2::C0(_, h968, _, _, _, _, _, _, _, _, _) => {
                            h968
                        }
                    };
                    let a971 = match v1.clone() {
                        Adt2::C0(_, _, h969, _, _, _, _, _, _, _, _) => {
                            h969
                        }
                    };
                    nat_eq(a970, a971)
                } else {
                    false
                } {
                    let a974 = match v1.clone() {
                        Adt2::C0(_, _, h972, _, _, _, _, _, _, _, _) => {
                            h972
                        }
                    };
                    let a975 = match v2.clone() {
                        Adt2::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                            h973
                        }
                    };
                    nat_eq(a974, a975)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m977 = f70(match v1.clone() {
                Adt2::C0(h976, _, _, _, _, _, _, _, _, _, _) => {
                    h976
                }
            });
            match m977 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h978) => {
                            h978
                        }
                    }, f13(v1, 1u64), f13(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f41(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f76(match v1.clone() {
        Adt2::C0(h979, _, _, _, _, _, _, _, _, _, _) => {
            h979
        }
    }, v0) {
        let a982 = match v1.clone() {
            Adt2::C0(h980, _, _, _, _, _, _, _, _, _, _) => {
                h980
            }
        };
        let a983 = match v2.clone() {
            Adt2::C0(h981, _, _, _, _, _, _, _, _, _, _) => {
                h981
            }
        };
        nat_eq(a982, a983)
    } else {
        false
    } {
        let a986 = match v1.clone() {
            Adt2::C0(_, h984, _, _, _, _, _, _, _, _, _) => {
                h984
            }
        };
        let a987 = match v2.clone() {
            Adt2::C0(_, h985, _, _, _, _, _, _, _, _, _) => {
                h985
            }
        };
        if if if nat_eq(a986, a987) {
            let a990 = match v1.clone() {
                Adt2::C0(_, _, h988, _, _, _, _, _, _, _, _) => {
                    h988
                }
            };
            let a991 = match v2.clone() {
                Adt2::C0(_, _, h989, _, _, _, _, _, _, _, _) => {
                    h989
                }
            };
            nat_eq(a990, a991)
        } else {
            false
        } {
            let a994 = match v1.clone() {
                Adt2::C0(_, _, _, h992, _, _, _, _, _, _, _) => {
                    h992
                }
            };
            let a995 = match v2.clone() {
                Adt2::C0(_, _, _, h993, _, _, _, _, _, _, _) => {
                    h993
                }
            };
            nat_eq(a994, a995)
        } else {
            false
        } {
            let a998 = match v1.clone() {
                Adt2::C0(_, _, _, _, h996, _, _, _, _, _, _) => {
                    h996
                }
            };
            let a999 = match v2.clone() {
                Adt2::C0(_, _, _, _, h997, _, _, _, _, _, _) => {
                    h997
                }
            };
            nat_eq(a998, a999)
        } else {
            false
        }
    } else {
        false
    } {
        let m1001 = f70(match v1.clone() {
            Adt2::C0(h1000, _, _, _, _, _, _, _, _, _, _) => {
                h1000
            }
        });
        match m1001 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h1002) => {
                        h1002
                    }
                }, f13(v1.clone(), 1u64), f13(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1003, _, _, _, _, _, _, _, _, _, _) => {
                        h1003
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f42(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1004, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1004
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1005, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1005
            }
        }
    };
    let a1006 = 0u64;
    let a1007 = v4;
    if if nat_lt(a1006, a1007) {
        let a1008 = v4;
        let a1009 = 2147483648u64;
        nat_lt(a1008, a1009)
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
            Adt2::C0(_, _, h1010, _, _, _, _, _, _, _, _) => {
                h1010
            }
        }, v4)?;
        let v8: u64 = f77(match v5.clone() {
            Adt2::C0(_, _, _, h1011, _, _, _, _, _, _, _) => {
                h1011
            }
        }, v4)?;
        let a1013 = match v5.clone() {
            Adt2::C0(h1012, _, _, _, _, _, _, _, _, _, _) => {
                h1012
            }
        };
        let a1014 = 0u64;
        if if if if if nat_eq(a1013, a1014) {
            let a1016 = match v6.clone() {
                Adt2::C0(h1015, _, _, _, _, _, _, _, _, _, _) => {
                    h1015
                }
            };
            let a1017 = 0u64;
            nat_eq(a1016, a1017)
        } else {
            false
        } {
            let a1019 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1018, _, _, _, _, _, _) => {
                    h1018
                }
            };
            let a1020 = 1u64;
            nat_eq(a1019, a1020)
        } else {
            false
        } {
            let a1023 = match v5.clone() {
                Adt2::C0(_, h1021, _, _, _, _, _, _, _, _, _) => {
                    h1021
                }
            };
            let a1024 = match v6.clone() {
                Adt2::C0(_, h1022, _, _, _, _, _, _, _, _, _) => {
                    h1022
                }
            };
            nat_eq(a1023, a1024)
        } else {
            false
        } {
            if v0 {
                let a1026 = match v5.clone() {
                    Adt2::C0(_, _, h1025, _, _, _, _, _, _, _, _) => {
                        h1025
                    }
                };
                let a1027 = 2147483648u64;
                if if if nat_lt(a1026, a1027) {
                    let a1029 = match v5.clone() {
                        Adt2::C0(_, _, _, h1028, _, _, _, _, _, _, _) => {
                            h1028
                        }
                    };
                    let a1030 = 2147483648u64;
                    nat_lt(a1029, a1030)
                } else {
                    false
                } {
                    let a1033 = {
                        let a1031 = v7;
                        let a1032 = v8;
                        nat_mul(a1031, a1032)?
                    };
                    let a1034 = 2147483648u64;
                    nat_le(a1033, a1034)
                } else {
                    false
                } {
                    f78(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1036 = match v1.clone() {
                    Adt1::C0(h1035, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1035
                    }
                };
                let a1037 = v7;
                if if if if nat_eq(a1036, a1037) {
                    let a1039 = match v1 {
                        Adt1::C0(_, h1038, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1038
                        }
                    };
                    let a1040 = v8;
                    nat_eq(a1039, a1040)
                } else {
                    false
                } {
                    f79(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1042 = match v6.clone() {
                        Adt2::C0(_, _, h1041, _, _, _, _, _, _, _, _) => {
                            h1041
                        }
                    };
                    let a1043 = v4;
                    if nat_eq(a1042, a1043) {
                        let a1045 = match v6.clone() {
                            Adt2::C0(_, _, _, h1044, _, _, _, _, _, _, _) => {
                                h1044
                            }
                        };
                        let a1046 = v4;
                        nat_eq(a1045, a1046)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1050 = match v6 {
                        Adt2::C0(_, _, _, _, h1047, _, _, _, _, _, _) => {
                            h1047
                        }
                    };
                    let a1051 = {
                        let a1048 = v7;
                        let a1049 = v8;
                        nat_mul(a1048, a1049)?
                    };
                    nat_eq(a1050, a1051)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f43(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a1053 = match v2.clone() {
        Adt2::C0(h1052, _, _, _, _, _, _, _, _, _, _) => {
            h1052
        }
    };
    let a1054 = 0u64;
    if if if if if nat_eq(a1053, a1054) {
        true
    } else {
        let a1056 = match v2.clone() {
            Adt2::C0(h1055, _, _, _, _, _, _, _, _, _, _) => {
                h1055
            }
        };
        let a1057 = 1u64;
        nat_eq(a1056, a1057)
    } {
        true
    } else {
        let a1059 = match v2.clone() {
            Adt2::C0(h1058, _, _, _, _, _, _, _, _, _, _) => {
                h1058
            }
        };
        let a1060 = 30u64;
        nat_eq(a1059, a1060)
    } {
        let a1063 = match v2.clone() {
            Adt2::C0(h1061, _, _, _, _, _, _, _, _, _, _) => {
                h1061
            }
        };
        let a1064 = match v3.clone() {
            Adt2::C0(h1062, _, _, _, _, _, _, _, _, _, _) => {
                h1062
            }
        };
        nat_eq(a1063, a1064)
    } else {
        false
    } {
        if if f14(v2.clone(), v3.clone()) {
            let a1066 = match v1.clone() {
                Adt1::C0(h1065, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1065
                }
            };
            let a1067 = 4294967296u64;
            nat_lt(a1066, a1067)
        } else {
            false
        } {
            let a1069 = match v1.clone() {
                Adt1::C0(_, h1068, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1068
                }
            };
            let a1070 = 4294967296u64;
            nat_lt(a1069, a1070)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, {
            let a1072 = match v2.clone() {
                Adt2::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                    h1071
                }
            };
            let a1073 = 0u64;
            if nat_eq(a1072, a1073) {
                4u64
            } else {
                2u64
            }
        }, f13(v2.clone(), 1u64), f13(v3, 1u64), Adt4::C13({
            let a1074 = v0;
            let a1075 = 31u64;
            if nat_eq(a1074, a1075) {
                0u64
            } else {
                let a1076 = v0;
                let a1077 = 47u64;
                if nat_eq(a1076, a1077) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1078, _, _, _, _, _, _, _, _, _, _) => {
                h1078
            }
        }, match v1.clone() {
            Adt1::C0(h1079, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1079
            }
        }, match v1 {
            Adt1::C0(_, h1080, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1080
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f44(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1082 = match v0.clone() {
        Adt2::C0(h1081, _, _, _, _, _, _, _, _, _, _) => {
            h1081
        }
    };
    let a1083 = 0u64;
    if if if nat_eq(a1082, a1083) {
        let a1085 = match v1.clone() {
            Adt2::C0(h1084, _, _, _, _, _, _, _, _, _, _) => {
                h1084
            }
        };
        let a1086 = 0u64;
        nat_eq(a1085, a1086)
    } else {
        false
    } {
        f14(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v1.clone(), 1u64)?, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f45(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1088 = match v0.clone() {
        Adt2::C0(h1087, _, _, _, _, _, _, _, _, _, _) => {
            h1087
        }
    };
    let a1089 = 0u64;
    if if nat_eq(a1088, a1089) {
        let a1091 = match v1.clone() {
            Adt2::C0(h1090, _, _, _, _, _, _, _, _, _, _) => {
                h1090
            }
        };
        let a1092 = 26u64;
        nat_eq(a1091, a1092)
    } else {
        false
    } {
        let a1094 = match v0.clone() {
            Adt2::C0(_, _, _, h1093, _, _, _, _, _, _, _) => {
                h1093
            }
        };
        let a1095 = 1u64;
        if if if if if if if nat_eq(a1094, a1095) {
            let a1097 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1096, _, _, _, _, _, _) => {
                    h1096
                }
            };
            let a1098 = 1u64;
            nat_eq(a1097, a1098)
        } else {
            false
        } {
            let a1100 = match v0.clone() {
                Adt2::C0(_, h1099, _, _, _, _, _, _, _, _, _) => {
                    h1099
                }
            };
            let a1101 = 2147483647u64;
            nat_le(a1100, a1101)
        } else {
            false
        } {
            let a1104 = match v1.clone() {
                Adt2::C0(_, h1102, _, _, _, _, _, _, _, _, _) => {
                    h1102
                }
            };
            let a1105 = match v0.clone() {
                Adt2::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                    h1103
                }
            };
            nat_eq(a1104, a1105)
        } else {
            false
        } {
            let a1107 = match v1.clone() {
                Adt2::C0(_, _, h1106, _, _, _, _, _, _, _, _) => {
                    h1106
                }
            };
            let a1108 = 1u64;
            nat_eq(a1107, a1108)
        } else {
            false
        } {
            let a1110 = match v1.clone() {
                Adt2::C0(_, _, _, h1109, _, _, _, _, _, _, _) => {
                    h1109
                }
            };
            let a1111 = 1u64;
            nat_eq(a1110, a1111)
        } else {
            false
        } {
            let a1113 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1112, _, _, _, _, _, _) => {
                    h1112
                }
            };
            let a1114 = 1u64;
            nat_eq(a1113, a1114)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1117 = match v0.clone() {
                    Adt2::C0(_, _, h1115, _, _, _, _, _, _, _, _) => {
                        h1115
                    }
                };
                let a1118 = f80(match v0.clone() {
                    Adt2::C0(_, h1116, _, _, _, _, _, _, _, _, _) => {
                        h1116
                    }
                })?;
                nat_mul(a1117, a1118)?
            }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f46(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1119 = v1;
    match m1119 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f81(f13(v0.clone(), 1u64))?;
            let a1121 = match v0.clone() {
                Adt2::C0(h1120, _, _, _, _, _, _, _, _, _, _) => {
                    h1120
                }
            };
            let a1122 = 26u64;
            if if if nat_eq(a1121, a1122) {
                let a1124 = match v3.clone() {
                    Adt2::C0(h1123, _, _, _, _, _, _, _, _, _, _) => {
                        h1123
                    }
                };
                let a1125 = 26u64;
                nat_eq(a1124, a1125)
            } else {
                false
            } {
                let a1127 = match v2.clone() {
                    Adt2::C0(h1126, _, _, _, _, _, _, _, _, _, _) => {
                        h1126
                    }
                };
                let a1128 = 27u64;
                nat_eq(a1127, a1128)
            } else {
                false
            } {
                if if if if if if f14(v0.clone(), v3.clone()) {
                    let a1130 = match v2.clone() {
                        Adt2::C0(_, h1129, _, _, _, _, _, _, _, _, _) => {
                            h1129
                        }
                    };
                    let a1131 = 1u64;
                    nat_eq(a1130, a1131)
                } else {
                    false
                } {
                    let a1133 = match v2.clone() {
                        Adt2::C0(_, _, h1132, _, _, _, _, _, _, _, _) => {
                            h1132
                        }
                    };
                    let a1134 = 1u64;
                    nat_eq(a1133, a1134)
                } else {
                    false
                } {
                    let a1136 = match v2.clone() {
                        Adt2::C0(_, _, _, h1135, _, _, _, _, _, _, _) => {
                            h1135
                        }
                    };
                    let a1137 = 1u64;
                    nat_eq(a1136, a1137)
                } else {
                    false
                } {
                    let a1139 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1138, _, _, _, _, _, _) => {
                            h1138
                        }
                    };
                    let a1140 = 1u64;
                    nat_eq(a1139, a1140)
                } else {
                    false
                } {
                    let a1141 = v4;
                    let a1142 = 9223372036854775808u64;
                    nat_lt(a1141, a1142)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1145 = 2u64;
                        let a1146 = {
                            let a1143 = v4;
                            let a1144 = 2u64;
                            nat_mul(a1143, a1144)?
                        };
                        nat_add(a1145, a1146)?
                    }, 8u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C18(v4, f13(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f47(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1147, _, _, _, _, _, _, _, _, _, _) => {
            h1147
        }
    };
    let a1148 = v3;
    let a1149 = 0u64;
    if if nat_eq(a1148, a1149) {
        true
    } else {
        let a1150 = v0;
        let a1151 = 13u64;
        if nat_eq(a1150, a1151) {
            let a1152 = v3;
            let a1153 = 1u64;
            if nat_eq(a1152, a1153) {
                true
            } else {
                let a1154 = v3;
                let a1155 = 30u64;
                nat_eq(a1154, a1155)
            }
        } else {
            false
        }
    } {
        let a1157 = v3;
        let a1158 = match v2.clone() {
            Adt2::C0(h1156, _, _, _, _, _, _, _, _, _, _) => {
                h1156
            }
        };
        if if nat_eq(a1157, a1158) {
            let a1159 = v0;
            let a1160 = 13u64;
            if nat_eq(a1159, a1160) {
                let a1162 = match v2.clone() {
                    Adt2::C0(_, h1161, _, _, _, _, _, _, _, _, _) => {
                        h1161
                    }
                };
                let a1163 = 1u64;
                if if if nat_eq(a1162, a1163) {
                    let a1165 = match v2.clone() {
                        Adt2::C0(_, _, h1164, _, _, _, _, _, _, _, _) => {
                            h1164
                        }
                    };
                    let a1166 = 1u64;
                    nat_eq(a1165, a1166)
                } else {
                    false
                } {
                    let a1168 = match v2.clone() {
                        Adt2::C0(_, _, _, h1167, _, _, _, _, _, _, _) => {
                            h1167
                        }
                    };
                    let a1169 = 1u64;
                    nat_eq(a1168, a1169)
                } else {
                    false
                } {
                    let a1171 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1170, _, _, _, _, _, _) => {
                            h1170
                        }
                    };
                    let a1172 = 1u64;
                    nat_eq(a1171, a1172)
                } else {
                    false
                }
            } else {
                let a1174 = match v2.clone() {
                    Adt2::C0(_, h1173, _, _, _, _, _, _, _, _, _) => {
                        h1173
                    }
                };
                let a1175 = 1u64;
                if if if nat_eq(a1174, a1175) {
                    let a1178 = match v1.clone() {
                        Adt2::C0(_, _, h1176, _, _, _, _, _, _, _, _) => {
                            h1176
                        }
                    };
                    let a1179 = match v2.clone() {
                        Adt2::C0(_, _, h1177, _, _, _, _, _, _, _, _) => {
                            h1177
                        }
                    };
                    nat_eq(a1178, a1179)
                } else {
                    false
                } {
                    let a1182 = match v1.clone() {
                        Adt2::C0(_, _, _, h1180, _, _, _, _, _, _, _) => {
                            h1180
                        }
                    };
                    let a1183 = match v2.clone() {
                        Adt2::C0(_, _, _, h1181, _, _, _, _, _, _, _) => {
                            h1181
                        }
                    };
                    nat_eq(a1182, a1183)
                } else {
                    false
                } {
                    let a1186 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1184, _, _, _, _, _, _) => {
                            h1184
                        }
                    };
                    let a1187 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1185, _, _, _, _, _, _) => {
                            h1185
                        }
                    };
                    nat_eq(a1186, a1187)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f13(v1, 1u64);
            Ok(Adt6::C4(f82(v4.clone(), v0, v3)?, {
                let a1188 = v3;
                let a1189 = 0u64;
                if nat_eq(a1188, a1189) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f13(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f48(v0: u64, v1: u64) -> Option<u64> {
    let a1190 = v0;
    let a1191 = 8u64;
    if nat_eq(a1190, a1191) {
        Some(5u64)
    } else {
        let a1192 = v0;
        let a1193 = 9u64;
        if nat_eq(a1192, a1193) {
            Some(6u64)
        } else {
            let a1194 = v0;
            let a1195 = 89u64;
            if nat_eq(a1194, a1195) {
                let a1196 = v1;
                let a1197 = 0u64;
                if nat_eq(a1196, a1197) {
                    Some(0u64)
                } else {
                    let a1198 = v1;
                    let a1199 = 1u64;
                    if nat_eq(a1198, a1199) {
                        Some(1u64)
                    } else {
                        let a1200 = v1;
                        let a1201 = 2u64;
                        if nat_eq(a1200, a1201) {
                            Some(2u64)
                        } else {
                            let a1202 = v1;
                            let a1203 = 3u64;
                            if nat_eq(a1202, a1203) {
                                Some(3u64)
                            } else {
                                let a1204 = v1;
                                let a1205 = 6u64;
                                if nat_eq(a1204, a1205) {
                                    Some(4u64)
                                } else {
                                    let a1206 = v1;
                                    let a1207 = 12u64;
                                    if nat_eq(a1206, a1207) {
                                        Some(7u64)
                                    } else {
                                        let a1208 = v1;
                                        let a1209 = 11u64;
                                        if nat_eq(a1208, a1209) {
                                            Some(8u64)
                                        } else {
                                            let a1210 = v1;
                                            let a1211 = 18u64;
                                            if nat_eq(a1210, a1211) {
                                                Some(9u64)
                                            } else {
                                                let a1212 = v1;
                                                let a1213 = 19u64;
                                                if nat_eq(a1212, a1213) {
                                                    Some(10u64)
                                                } else {
                                                    let a1214 = v1;
                                                    let a1215 = 20u64;
                                                    if nat_eq(a1214, a1215) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1216 = v1;
                                                        let a1217 = 21u64;
                                                        if nat_eq(a1216, a1217) {
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

pub fn f49(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1218 = f83(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1218 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            Ok(Adt6::C4(v12, 4u64, f13(v2.clone(), 1u64), f13(v4, 1u64), Adt4::C10(v6, v7, {
                let m1219 = v3;
                match m1219 {
                    None => {
                        f13(v2, 1u64)
                    }
                    Some(v13) => {
                        f13(v13, 1u64)
                    }
                }
            })))
        }
        Adt6::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) => {
            Ok(Adt6::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23))
        }
        Adt6::C4(v24, v25, v26, v27, v28) => {
            Ok(Adt6::C4(v24, v25, v26, v27, v28))
        }
        Adt6::C5(v5) => {
            Ok(Adt6::C5(v5))
        }
    }
}

pub fn f50(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1221 = match v1.clone() {
        Adt2::C0(h1220, _, _, _, _, _, _, _, _, _, _) => {
            h1220
        }
    };
    let a1222 = 0u64;
    if if if if if nat_eq(a1221, a1222) {
        true
    } else {
        let a1224 = match v1.clone() {
            Adt2::C0(h1223, _, _, _, _, _, _, _, _, _, _) => {
                h1223
            }
        };
        let a1225 = 1u64;
        nat_eq(a1224, a1225)
    } {
        true
    } else {
        let a1227 = match v1.clone() {
            Adt2::C0(h1226, _, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1228 = 30u64;
        nat_eq(a1227, a1228)
    } {
        let a1231 = match v1.clone() {
            Adt2::C0(h1229, _, _, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1232 = match v2.clone() {
            Adt2::C0(h1230, _, _, _, _, _, _, _, _, _, _) => {
                h1230
            }
        };
        if nat_eq(a1231, a1232) {
            true
        } else {
            let a1234 = match v2.clone() {
                Adt2::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
                    h1233
                }
            };
            let a1235 = 0u64;
            nat_eq(a1234, a1235)
        }
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, {
            let a1237 = match v2.clone() {
                Adt2::C0(h1236, _, _, _, _, _, _, _, _, _, _) => {
                    h1236
                }
            };
            let a1238 = 0u64;
            if nat_eq(a1237, a1238) {
                4u64
            } else {
                2u64
            }
        }, f13(v1.clone(), 1u64), f13(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1239, _, _, _, _, _, _, _, _, _, _) => {
                h1239
            }
        }, match v2 {
            Adt2::C0(h1240, _, _, _, _, _, _, _, _, _, _) => {
                h1240
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1241 = v1;
    match m1241 {
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
            f84(f85(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f86(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f87(v0.clone(), v16);
            f84(f88(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
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

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m1242 = v3;
    match m1242 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f91(v0, v1, v2.clone())?, f89(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f92(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f93(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f94(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f93(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f95(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f96(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f97(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f98(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f99(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C15 => {
            f100(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f101(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f102(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f89(f90(v0, match v2.clone() {
                Adt3::C0(h1243, _, _, _, _, _, _, _) => {
                    h1243
                }
            }, match v2.clone() {
                Adt3::C0(_, h1244, _, _, _, _, _, _) => {
                    h1244
                }
            })?, v1)?, f89(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt5::C0(Adt6::C5(65u64)))
        }
        Adt4::C25(v48, v49, v50) => {
            f103(v0, v48, v49, v50)
        }
        Adt4::C26(v51, _, _, _) => {
            let a1245 = v51;
            let a1246 = 64u64;
            if nat_eq(a1245, a1246) {
                Ok(Adt5::C0(Adt6::C5(64u64)))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f80(match v1.clone() {
        Adt3::C0(h1247, _, _, _, _, _, _, _) => {
            h1247
        }
    })?;
    let v5: u64 = {
        let a1248 = v0;
        let a1249 = v4;
        let a1250 = 0u64;
        nat_quot(a1248, a1249, a1250)
    };
    let v6: u64 = {
        let a1251 = v0;
        let a1252 = v4;
        let a1253 = 0u64;
        nat_rem(a1251, a1252, a1253)
    };
    let a1254 = v6;
    let a1255 = 0u64;
    if nat_eq(a1254, a1255) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1256 = v6;
        let a1257 = 1u64;
        if nat_eq(a1256, a1257) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1260 = {
                let a1258 = v6;
                let a1259 = 1u64;
                nat_add(a1258, a1259)?
            };
            let a1261 = v4;
            if nat_eq(a1260, a1261) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f89(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1262 = v6;
                    let a1263 = 2u64;
                    nat_sub(a1262, a1263)
                };
                let v8: u64 = {
                    let a1264 = v7;
                    let a1265 = 3u64;
                    let a1266 = 0u64;
                    nat_quot(a1264, a1265, a1266)
                };
                let v9: u64 = {
                    let a1267 = v7;
                    let a1268 = 3u64;
                    let a1269 = 0u64;
                    nat_rem(a1267, a1268, a1269)
                };
                let v10: u64 = f89({
                    let a1273 = {
                        let a1271 = v5;
                        let a1272 = match v1.clone() {
                            Adt3::C0(h1270, _, _, _, _, _, _, _) => {
                                h1270
                            }
                        };
                        nat_mul(a1271, a1272)?
                    };
                    let a1274 = v8;
                    nat_add(a1273, a1274)?
                }, v1)?;
                let a1275 = v9;
                let a1276 = 0u64;
                if nat_eq(a1275, a1276) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1277 = v9;
                    let a1278 = 1u64;
                    if nat_eq(a1277, a1278) {
                        let a1279 = v3;
                        let a1280 = 2u64;
                        if nat_eq(a1279, a1280) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1281 = v3;
                        let a1282 = 2u64;
                        if if nat_eq(a1281, a1282) {
                            true
                        } else {
                            let a1283 = v3;
                            let a1284 = 3u64;
                            nat_eq(a1283, a1284)
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

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1286 = 2u64;
        let a1287 = match v1.clone() {
            Adt3::C0(_, h1285, _, _, _, _, _, _) => {
                h1285
            }
        };
        nat_mul(a1286, a1287)?
    };
    let a1288 = v0;
    let a1289 = v5;
    if nat_lt(a1288, a1289) {
        let a1293 = {
            let a1290 = v0;
            let a1291 = 2u64;
            let a1292 = 0u64;
            nat_rem(a1290, a1291, a1292)
        };
        let a1294 = 0u64;
        if nat_eq(a1293, a1294) {
            Ok(Adt5::C4(19u64, 4u64, f89({
                let a1295 = v0;
                let a1296 = 2u64;
                let a1297 = 0u64;
                nat_quot(a1295, a1296, a1297)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1298 = v4;
            let a1299 = 2147483648u64;
            if if nat_lt(a1298, a1299) {
                let a1301 = v4;
                let a1302 = match v2 {
                    Adt3::C0(_, h1300, _, _, _, _, _, _) => {
                        h1300
                    }
                };
                nat_lt(a1301, a1302)
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
            let a1305 = match v2.clone() {
                Adt3::C0(h1303, _, _, _, _, _, _, _) => {
                    h1303
                }
            };
            let a1306 = match v2.clone() {
                Adt3::C0(_, h1304, _, _, _, _, _, _) => {
                    h1304
                }
            };
            nat_mul(a1305, a1306)?
        };
        let a1309 = {
            let a1307 = v0;
            let a1308 = v5;
            nat_sub(a1307, a1308)
        };
        let a1310 = v6;
        if nat_lt(a1309, a1310) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f89({
                let a1311 = v0;
                let a1312 = v5;
                nat_sub(a1311, a1312)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1315 = {
                    let a1313 = v0;
                    let a1314 = v5;
                    nat_sub(a1313, a1314)
                };
                let a1316 = v6;
                nat_sub(a1315, a1316)
            };
            let v8: u64 = {
                let a1320 = 1u64;
                let a1321 = {
                    let a1318 = 2u64;
                    let a1319 = match v1.clone() {
                        Adt3::C0(h1317, _, _, _, _, _, _, _) => {
                            h1317
                        }
                    };
                    nat_mul(a1318, a1319)?
                };
                nat_add(a1320, a1321)?
            };
            let v9: u64 = {
                let a1322 = v7;
                let a1323 = v8;
                let a1324 = 0u64;
                nat_quot(a1322, a1323, a1324)
            };
            let v10: u64 = {
                let a1325 = v7;
                let a1326 = v8;
                let a1327 = 0u64;
                nat_rem(a1325, a1326, a1327)
            };
            let a1328 = v10;
            let a1329 = 0u64;
            if nat_eq(a1328, a1329) {
                Ok(Adt5::C4(19u64, 4u64, f89(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1333 = {
                    let a1330 = v10;
                    let a1331 = 2u64;
                    let a1332 = 0u64;
                    nat_rem(a1330, a1331, a1332)
                };
                let a1334 = 1u64;
                if nat_eq(a1333, a1334) {
                    Ok(Adt5::C4(6u64, 0u64, f89({
                        let a1343 = {
                            let a1336 = v9;
                            let a1337 = match v1.clone() {
                                Adt3::C0(h1335, _, _, _, _, _, _, _) => {
                                    h1335
                                }
                            };
                            nat_mul(a1336, a1337)?
                        };
                        let a1344 = {
                            let a1340 = {
                                let a1338 = v10;
                                let a1339 = 1u64;
                                nat_sub(a1338, a1339)
                            };
                            let a1341 = 2u64;
                            let a1342 = 0u64;
                            nat_quot(a1340, a1341, a1342)
                        };
                        nat_add(a1343, a1344)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f89({
                        let a1353 = {
                            let a1346 = v4;
                            let a1347 = match v1.clone() {
                                Adt3::C0(h1345, _, _, _, _, _, _, _) => {
                                    h1345
                                }
                            };
                            nat_mul(a1346, a1347)?
                        };
                        let a1354 = {
                            let a1350 = {
                                let a1348 = v10;
                                let a1349 = 1u64;
                                nat_sub(a1348, a1349)
                            };
                            let a1351 = 2u64;
                            let a1352 = 0u64;
                            nat_quot(a1350, a1351, a1352)
                        };
                        nat_add(a1353, a1354)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f89({
                        let a1363 = {
                            let a1356 = v4;
                            let a1357 = match v1 {
                                Adt3::C0(h1355, _, _, _, _, _, _, _) => {
                                    h1355
                                }
                            };
                            nat_mul(a1356, a1357)?
                        };
                        let a1364 = {
                            let a1360 = {
                                let a1358 = v10;
                                let a1359 = 1u64;
                                nat_sub(a1358, a1359)
                            };
                            let a1361 = 2u64;
                            let a1362 = 0u64;
                            nat_quot(a1360, a1361, a1362)
                        };
                        nat_add(a1363, a1364)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1365 = v1;
    let a1366 = f104(v2.clone(), v4.clone())?;
    if nat_lt(a1365, a1366) {
        let a1369 = v1;
        let a1370 = {
            let a1367 = 2u64;
            let a1368 = f81(v4.clone())?;
            nat_mul(a1367, a1368)?
        };
        if nat_lt(a1369, a1370) {
            let a1374 = {
                let a1371 = v1;
                let a1372 = 2u64;
                let a1373 = 0u64;
                nat_rem(a1371, a1372, a1373)
            };
            let a1375 = 0u64;
            if nat_eq(a1374, a1375) {
                f105({
                    let a1376 = v1;
                    let a1377 = 2u64;
                    let a1378 = 0u64;
                    nat_quot(a1376, a1377, a1378)
                }, v4)
            } else {
                if f106(match v2 {
                    Adt3::C0(_, h1379, _, _, _, _, _, _) => {
                        h1379
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1382 = v1;
                let a1383 = {
                    let a1380 = 2u64;
                    let a1381 = f81(v4.clone())?;
                    nat_mul(a1380, a1381)?
                };
                nat_sub(a1382, a1383)
            };
            let v7: u64 = {
                let a1387 = v6;
                let a1388 = {
                    let a1385 = 1u64;
                    let a1386 = match v2.clone() {
                        Adt3::C0(h1384, _, _, _, _, _, _, _) => {
                            h1384
                        }
                    };
                    nat_add(a1385, a1386)?
                };
                let a1389 = 0u64;
                nat_quot(a1387, a1388, a1389)
            };
            let v8: u64 = {
                let a1393 = v6;
                let a1394 = {
                    let a1391 = 1u64;
                    let a1392 = match v2.clone() {
                        Adt3::C0(h1390, _, _, _, _, _, _, _) => {
                            h1390
                        }
                    };
                    nat_add(a1391, a1392)?
                };
                let a1395 = 0u64;
                nat_rem(a1393, a1394, a1395)
            };
            let a1396 = v8;
            let a1397 = 0u64;
            if nat_eq(a1396, a1397) {
                f105(v7, v4)
            } else {
                f107(v0, v7, {
                    let a1398 = v8;
                    let a1399 = 1u64;
                    nat_sub(a1398, a1399)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1402 = match v4.clone() {
            Adt3::C0(h1400, _, _, _, _, _, _, _) => {
                h1400
            }
        };
        let a1403 = match v4.clone() {
            Adt3::C0(_, h1401, _, _, _, _, _, _) => {
                h1401
            }
        };
        nat_mul(a1402, a1403)?
    };
    let v7: u64 = {
        let a1404 = 2u64;
        let a1405 = v6;
        nat_mul(a1404, a1405)?
    };
    let v8: u64 = f108(match v3.clone() {
        Adt3::C0(_, h1406, _, _, _, _, _, _) => {
            h1406
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1407, _, _, _, _, _) => {
            h1407
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1408, _, _, _, _) => {
            h1408
        }
    });
    let a1409 = v0;
    let a1410 = v7;
    if nat_lt(a1409, a1410) {
        let a1414 = {
            let a1411 = v0;
            let a1412 = 2u64;
            let a1413 = 0u64;
            nat_rem(a1411, a1412, a1413)
        };
        let a1415 = 0u64;
        if nat_eq(a1414, a1415) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1416 = v0;
                let a1417 = 2u64;
                let a1418 = 0u64;
                nat_quot(a1416, a1417, a1418)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1419 = v0;
            let a1420 = v7;
            nat_sub(a1419, a1420)
        };
        let v10: u64 = {
            let a1424 = v9;
            let a1425 = {
                let a1422 = 1u64;
                let a1423 = match v1.clone() {
                    Adt3::C0(h1421, _, _, _, _, _, _, _) => {
                        h1421
                    }
                };
                nat_add(a1422, a1423)?
            };
            let a1426 = 0u64;
            nat_quot(a1424, a1425, a1426)
        };
        let v11: u64 = {
            let a1430 = v9;
            let a1431 = {
                let a1428 = 1u64;
                let a1429 = match v1.clone() {
                    Adt3::C0(h1427, _, _, _, _, _, _, _) => {
                        h1427
                    }
                };
                nat_add(a1428, a1429)?
            };
            let a1432 = 0u64;
            nat_rem(a1430, a1431, a1432)
        };
        let a1433 = v11;
        let a1434 = 0u64;
        if nat_eq(a1433, a1434) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1435 = v10;
                let a1436 = v6;
                let a1437 = 0u64;
                nat_rem(a1435, a1436, a1437)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                let v12: u64 = {
                    let a1443 = {
                        let a1439 = v10;
                        let a1440 = match v1.clone() {
                            Adt3::C0(h1438, _, _, _, _, _, _, _) => {
                                h1438
                            }
                        };
                        nat_mul(a1439, a1440)?
                    };
                    let a1444 = {
                        let a1441 = v11;
                        let a1442 = 1u64;
                        nat_sub(a1441, a1442)
                    };
                    nat_add(a1443, a1444)?
                };
                let a1445 = v5;
                let a1446 = 4294967295u64;
                if nat_eq(a1445, a1446) {
                    Ok(Adt5::C1(false, f89(v12, v1)?, f89(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f89(v12, v1)?, 4u64, f89({
                        let a1452 = {
                            let a1448 = v5;
                            let a1449 = match v3.clone() {
                                Adt3::C0(h1447, _, _, _, _, _, _, _) => {
                                    h1447
                                }
                            };
                            nat_mul(a1448, a1449)?
                        };
                        let a1453 = {
                            let a1450 = v11;
                            let a1451 = 1u64;
                            nat_sub(a1450, a1451)
                        };
                        nat_add(a1452, a1453)?
                    }, v3)?, 3u64, f89(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt5> {
    let v8: u64 = {
        let a1456 = v6;
        let a1457 = {
            let a1454 = 3u64;
            let a1455 = v5;
            nat_add(a1454, a1455)?
        };
        nat_sub(a1456, a1457)
    };
    let a1459 = v0;
    let a1460 = f25(v3, match v1.clone() {
        Adt3::C0(h1458, _, _, _, _, _, _, _) => {
            h1458
        }
    }, v5, v8)?;
    if nat_lt(a1459, a1460) {
        let v9: u64 = f63(match v1.clone() {
            Adt3::C0(h1461, _, _, _, _, _, _, _) => {
                h1461
            }
        }, v5, v8)?;
        let v10: u64 = f64(v3, match v1.clone() {
            Adt3::C0(h1462, _, _, _, _, _, _, _) => {
                h1462
            }
        }, v5)?;
        let a1463 = v0;
        let a1464 = v9;
        if nat_lt(a1463, a1464) {
            f110(v0, v2, f19(v3), {
                let a1465 = 3u64;
                let a1466 = v5;
                nat_add(a1465, a1466)?
            }, v7)
        } else {
            let a1469 = v0;
            let a1470 = {
                let a1467 = v9;
                let a1468 = v10;
                nat_add(a1467, a1468)?
            };
            if nat_lt(a1469, a1470) {
                let v11: u64 = {
                    let a1471 = v0;
                    let a1472 = v9;
                    nat_sub(a1471, a1472)
                };
                let v12: u64 = f111(v3, match v1.clone() {
                    Adt3::C0(h1473, _, _, _, _, _, _, _) => {
                        h1473
                    }
                })?;
                f112({
                    let a1474 = v11;
                    let a1475 = v12;
                    let a1476 = 0u64;
                    nat_rem(a1474, a1475, a1476)
                }, {
                    let a1477 = v11;
                    let a1478 = v12;
                    let a1479 = 0u64;
                    nat_quot(a1477, a1478, a1479)
                }, v1, v3, v4, v7)
            } else {
                f113({
                    let a1482 = v0;
                    let a1483 = {
                        let a1480 = v9;
                        let a1481 = v10;
                        nat_add(a1480, a1481)?
                    };
                    nat_sub(a1482, a1483)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1484 = v0;
    let a1485 = f12(v1)?;
    if nat_lt(a1484, a1485) {
        let a1486 = v0;
        let a1487 = 4u64;
        if nat_lt(a1486, a1487) {
            let a1488 = v0;
            let a1489 = 0u64;
            if nat_eq(a1488, a1489) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1490 = v0;
                let a1491 = 1u64;
                if nat_eq(a1490, a1491) {
                    let a1492 = v5;
                    let a1493 = 0u64;
                    if nat_eq(a1492, a1493) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1494 = v0;
                    let a1495 = 2u64;
                    if nat_eq(a1494, a1495) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1496 = v0;
            let a1497 = 4u64;
            if if nat_eq(a1496, a1497) {
                let a1498 = f114(v5)?;
                let a1499 = v1;
                !nat_eq(a1498, a1499)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1502 = {
                        let a1500 = v0;
                        let a1501 = 4u64;
                        nat_sub(a1500, a1501)
                    };
                    let a1503 = 3u64;
                    let a1504 = 0u64;
                    nat_quot(a1502, a1503, a1504)
                };
                let v7: u64 = {
                    let a1507 = {
                        let a1505 = v0;
                        let a1506 = 4u64;
                        nat_sub(a1505, a1506)
                    };
                    let a1508 = 3u64;
                    let a1509 = 0u64;
                    nat_rem(a1507, a1508, a1509)
                };
                let a1510 = v7;
                let a1511 = 0u64;
                if nat_eq(a1510, a1511) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1512 = v7;
                    let a1513 = 1u64;
                    if nat_eq(a1512, a1513) {
                        Ok(Adt5::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt5::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f11(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f59(v0: u64) -> u64 {
    let v1: u64 = {
        let a1514 = v0;
        let a1515 = 4294967296u64;
        let a1516 = 0u64;
        nat_rem(a1514, a1515, a1516)
    };
    let a1517 = v1;
    let a1518 = 2147483648u64;
    if nat_lt(a1517, a1518) {
        v1
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> R<u64> {
    let a1521 = 2u64;
    let a1522 = {
        let a1519 = 2u64;
        let a1520 = v0;
        nat_mul(a1519, a1520)?
    };
    nat_add(a1521, a1522)
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f115(v0, v3, v6) {
        f115(v1, v4, v6)
    } else {
        false
    } {
        f115(v2, v5, v6)
    } else {
        false
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f116(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f116(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f116(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f116(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f116(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f116(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1523 = v2;
    let a1524 = 0u64;
    if nat_eq(a1523, a1524) {
        Ok(0u64)
    } else {
        let a1527 = 2u64;
        let a1528 = {
            let a1525 = v0;
            let a1526 = v1;
            nat_mul(a1525, a1526)?
        };
        nat_mul(a1527, a1528)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1529 = v2;
    let a1530 = f111(v0, v1)?;
    nat_mul(a1529, a1530)
}

pub fn f65(v0: u64) -> u64 {
    let a1531 = v0;
    let a1532 = 30u64;
    if nat_eq(a1531, a1532) {
        3u64
    } else {
        1u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1535 = 5u64;
    let a1536 = {
        let a1533 = 4u64;
        let a1534 = v0;
        nat_mul(a1533, a1534)?
    };
    nat_add(a1535, a1536)
}

pub fn f67(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f117(match v0.clone() {
        Adt2::C0(h1537, _, _, _, _, _, _, _, _, _, _) => {
            h1537
        }
    }, match v0.clone() {
        Adt2::C0(_, h1538, _, _, _, _, _, _, _, _, _) => {
            h1538
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1539, _, _, _, _, _, _, _, _) => {
            h1539
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1540, _, _, _, _, _, _, _) => {
            h1540
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1541, _, _, _, _, _, _) => {
            h1541
        }
    }, match v1.clone() {
        Adt2::C0(h1542, _, _, _, _, _, _, _, _, _, _) => {
            h1542
        }
    }, match v1.clone() {
        Adt2::C0(_, h1543, _, _, _, _, _, _, _, _, _) => {
            h1543
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1544, _, _, _, _, _, _, _, _) => {
            h1544
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1545, _, _, _, _, _, _, _) => {
            h1545
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1546, _, _, _, _, _, _) => {
            h1546
        }
    }, match v2.clone() {
        Adt2::C0(h1547, _, _, _, _, _, _, _, _, _, _) => {
            h1547
        }
    }, match v2.clone() {
        Adt2::C0(_, h1548, _, _, _, _, _, _, _, _, _) => {
            h1548
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1549, _, _, _, _, _, _, _, _) => {
            h1549
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1550, _, _, _, _, _, _, _) => {
            h1550
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1551, _, _, _, _, _, _) => {
            h1551
        }
    }) {
        let m1553 = f70(match v0.clone() {
            Adt2::C0(h1552, _, _, _, _, _, _, _, _, _, _) => {
                h1552
            }
        });
        match m1553 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1559 = {
                    let a1556 = match v0.clone() {
                        Adt2::C0(_, h1554, _, _, _, _, _, _, _, _, _) => {
                            h1554
                        }
                    };
                    let a1557 = match v3.clone() {
                        Adt11::C0(_, h1555, _) => {
                            h1555
                        }
                    };
                    let a1558 = 0u64;
                    nat_rem(a1556, a1557, a1558)
                };
                let a1560 = 0u64;
                if if nat_eq(a1559, a1560) {
                    if f118(match v0.clone() {
                        Adt2::C0(_, h1561, _, _, _, _, _, _, _, _, _) => {
                            h1561
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1562, _, _, _, _, _, _, _, _, _) => {
                            h1562
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1563, _, _, _, _, _, _, _, _) => {
                            h1563
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1564, _, _, _, _, _, _, _) => {
                            h1564
                        }
                    }) {
                        true
                    } else {
                        let a1566 = 4u64;
                        let a1567 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1565, _, _, _, _, _) => {
                                h1565
                            }
                        };
                        if if nat_le(a1566, a1567) {
                            let a1569 = match v3.clone() {
                                Adt11::C0(_, h1568, _) => {
                                    h1568
                                }
                            };
                            let a1570 = 1u64;
                            if nat_eq(a1569, a1570) {
                                true
                            } else {
                                let a1573 = match v3.clone() {
                                    Adt11::C0(_, _, h1571) => {
                                        h1571
                                    }
                                };
                                let a1574 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1572, _, _, _, _, _) => {
                                        h1572
                                    }
                                };
                                nat_le(a1573, a1574)
                            }
                        } else {
                            false
                        } {
                            if f16(v0.clone(), v2.clone())? {
                                f16(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f13(v0.clone(), match v3 {
                        Adt11::C0(_, h1575, _) => {
                            h1575
                        }
                    });
                    Ok(Adt6::C4(if f118(match v0.clone() {
                        Adt2::C0(_, h1576, _, _, _, _, _, _, _, _, _) => {
                            h1576
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1577, _, _, _, _, _, _, _, _, _) => {
                            h1577
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1578, _, _, _, _, _, _, _, _) => {
                            h1578
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1579, _, _, _, _, _, _, _) => {
                            h1579
                        }
                    }) {
                        0u64
                    } else {
                        f104(v4.clone(), f13(v1.clone(), 1u64))?
                    }, 4u64, v4, f13(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1580, _, _, _, _, _, _, _, _, _, _) => {
                            h1580
                        }
                    }, f13(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f68(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1583 = match v1.clone() {
        Adt2::C0(h1581, _, _, _, _, _, _, _, _, _, _) => {
            h1581
        }
    };
    let a1584 = match v3.clone() {
        Adt2::C0(h1582, _, _, _, _, _, _, _, _, _, _) => {
            h1582
        }
    };
    if if nat_eq(a1583, a1584) {
        let a1585 = v0;
        let a1586 = 33u64;
        if nat_eq(a1585, a1586) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1588 = f70(match v1.clone() {
            Adt2::C0(h1587, _, _, _, _, _, _, _, _, _, _) => {
                h1587
            }
        });
        match m1588 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1589, _) => {
                        h1589
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a1594 = {
                    let a1591 = match v1.clone() {
                        Adt2::C0(_, h1590, _, _, _, _, _, _, _, _, _) => {
                            h1590
                        }
                    };
                    let a1592 = v5;
                    let a1593 = 1u64;
                    nat_rem(a1591, a1592, a1593)
                };
                let a1595 = 0u64;
                if if if nat_eq(a1594, a1595) {
                    let a1600 = {
                        let a1597 = match v3.clone() {
                            Adt2::C0(_, h1596, _, _, _, _, _, _, _, _, _) => {
                                h1596
                            }
                        };
                        let a1598 = v5;
                        let a1599 = 1u64;
                        nat_rem(a1597, a1598, a1599)
                    };
                    let a1601 = 0u64;
                    nat_eq(a1600, a1601)
                } else {
                    false
                } {
                    let a1602 = v6;
                    let a1603 = f9(v3.clone(), v5)?;
                    nat_eq(a1602, a1603)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1604) => {
                            h1604
                        }
                    }, f13(v1, v5), f13(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f69(v0: u64) -> bool {
    let a1605 = v0;
    let a1606 = 0u64;
    if if nat_eq(a1605, a1606) {
        true
    } else {
        let a1607 = v0;
        let a1608 = 1u64;
        nat_eq(a1607, a1608)
    } {
        true
    } else {
        let a1609 = v0;
        let a1610 = 30u64;
        nat_eq(a1609, a1610)
    }
}

pub fn f70(v0: u64) -> Adt10 {
    let a1611 = v0;
    let a1612 = 0u64;
    if nat_eq(a1611, a1612) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1613 = v0;
        let a1614 = 1u64;
        if nat_eq(a1613, a1614) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1615 = v0;
            let a1616 = 2u64;
            if nat_eq(a1615, a1616) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1617 = v0;
                let a1618 = 3u64;
                if nat_eq(a1617, a1618) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1619 = v0;
                    let a1620 = 6u64;
                    if nat_eq(a1619, a1620) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1621 = v0;
                        let a1622 = 7u64;
                        if nat_eq(a1621, a1622) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1623 = v0;
                            let a1624 = 8u64;
                            if nat_eq(a1623, a1624) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1625 = v0;
                                let a1626 = 9u64;
                                if nat_eq(a1625, a1626) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1627 = v0;
                                    let a1628 = 10u64;
                                    if nat_eq(a1627, a1628) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1629 = v0;
                                        let a1630 = 11u64;
                                        if nat_eq(a1629, a1630) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1631 = v0;
                                            let a1632 = 12u64;
                                            if nat_eq(a1631, a1632) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1633 = v0;
                                                let a1634 = 13u64;
                                                if nat_eq(a1633, a1634) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1635 = v0;
                                                    let a1636 = 14u64;
                                                    if nat_eq(a1635, a1636) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1637 = v0;
                                                        let a1638 = 15u64;
                                                        if nat_eq(a1637, a1638) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1639 = v0;
                                                            let a1640 = 16u64;
                                                            if nat_eq(a1639, a1640) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1641 = v0;
                                                                let a1642 = 17u64;
                                                                if nat_eq(a1641, a1642) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1643 = v0;
                                                                    let a1644 = 18u64;
                                                                    if nat_eq(a1643, a1644) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1645 = v0;
                                                                        let a1646 = 19u64;
                                                                        if nat_eq(a1645, a1646) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1647 = v0;
                                                                            let a1648 = 20u64;
                                                                            if nat_eq(a1647, a1648) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1649 = v0;
                                                                                let a1650 = 21u64;
                                                                                if nat_eq(a1649, a1650) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1651 = v0;
                                                                                    let a1652 = 22u64;
                                                                                    if nat_eq(a1651, a1652) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1653 = v0;
                                                                                        let a1654 = 23u64;
                                                                                        if nat_eq(a1653, a1654) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1655 = v0;
                                                                                            let a1656 = 24u64;
                                                                                            if nat_eq(a1655, a1656) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1657 = v0;
                                                                                                let a1658 = 25u64;
                                                                                                if nat_eq(a1657, a1658) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1659 = v0;
                                                                                                    let a1660 = 26u64;
                                                                                                    if nat_eq(a1659, a1660) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1661 = v0;
                                                                                                        let a1662 = 27u64;
                                                                                                        if nat_eq(a1661, a1662) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1663 = v0;
                                                                                                            let a1664 = 28u64;
                                                                                                            if nat_eq(a1663, a1664) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1665 = v0;
                                                                                                                let a1666 = 29u64;
                                                                                                                if nat_eq(a1665, a1666) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1667 = v0;
                                                                                                                    let a1668 = 30u64;
                                                                                                                    if nat_eq(a1667, a1668) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1669 = v0;
                                                                                                                        let a1670 = 34u64;
                                                                                                                        if nat_eq(a1669, a1670) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1671 = v0;
                                                                                                                            let a1672 = 35u64;
                                                                                                                            if nat_eq(a1671, a1672) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1673 = v0;
                                                                                                                                let a1674 = 39u64;
                                                                                                                                if nat_eq(a1673, a1674) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1675 = v0;
                                                                                                                                    let a1676 = 40u64;
                                                                                                                                    if nat_eq(a1675, a1676) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1677 = v0;
                                                                                                                                        let a1678 = 41u64;
                                                                                                                                        if nat_eq(a1677, a1678) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1679 = v0;
                                                                                                                                            let a1680 = 42u64;
                                                                                                                                            if nat_eq(a1679, a1680) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1681 = v0;
                                                                                                                                                let a1682 = 43u64;
                                                                                                                                                if nat_eq(a1681, a1682) {
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

pub fn f71(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f23(v4, v5, v6, v7) {
        f23(v0, v1, v2, v3)
    } else {
        let a1686 = {
            let a1683 = v0;
            let a1684 = v4;
            let a1685 = 1u64;
            nat_rem(a1683, a1684, a1685)
        };
        let a1687 = 0u64;
        if if if nat_eq(a1686, a1687) {
            let a1691 = {
                let a1688 = v1;
                let a1689 = v5;
                let a1690 = 1u64;
                nat_rem(a1688, a1689, a1690)
            };
            let a1692 = 0u64;
            nat_eq(a1691, a1692)
        } else {
            false
        } {
            let a1696 = {
                let a1693 = v2;
                let a1694 = v6;
                let a1695 = 1u64;
                nat_rem(a1693, a1694, a1695)
            };
            let a1697 = 0u64;
            nat_eq(a1696, a1697)
        } else {
            false
        } {
            let a1701 = {
                let a1698 = v3;
                let a1699 = v7;
                let a1700 = 1u64;
                nat_rem(a1698, a1699, a1700)
            };
            let a1702 = 0u64;
            nat_eq(a1701, a1702)
        } else {
            false
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1703 = v0;
    let a1704 = v1;
    if nat_eq(a1703, a1704) {
        let a1707 = v4;
        let a1708 = {
            let a1705 = v2;
            let a1706 = v3;
            nat_add(a1705, a1706)?
        };
        Ok(nat_eq(a1707, a1708))
    } else {
        let a1709 = v2;
        let a1710 = v3;
        if nat_eq(a1709, a1710) {
            let a1711 = v4;
            let a1712 = v2;
            Ok(nat_eq(a1711, a1712))
        } else {
            Ok(false)
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1715 = {
            let a1713 = v0;
            let a1714 = f119(v1);
            nat_add(a1713, a1714)?
        };
        let a1716 = f119(v2);
        nat_add(a1715, a1716)?
    };
    let v5: u64 = {
        let a1717 = f120(v1);
        let a1718 = f120(v2);
        nat_add(a1717, a1718)?
    };
    let a1719 = v1;
    let a1720 = 4294967295u64;
    if if nat_le(a1719, a1720) {
        let a1721 = v2;
        let a1722 = 4294967295u64;
        nat_le(a1721, a1722)
    } else {
        false
    } {
        let a1723 = v5;
        let a1724 = v4;
        if nat_le(a1723, a1724) {
            let a1727 = v3;
            let a1728 = {
                let a1725 = v4;
                let a1726 = v5;
                nat_sub(a1725, a1726)
            };
            Ok(nat_eq(a1727, a1728))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f74(v0: u64, v1: u64) -> R<bool> {
    let a1729 = v1;
    let a1730 = 4294967295u64;
    if nat_le(a1729, a1730) {
        let a1733 = {
            let a1731 = f119(v1);
            let a1732 = f120(v1);
            nat_add(a1731, a1732)?
        };
        let a1734 = v0;
        Ok(nat_lt(a1733, a1734))
    } else {
        Ok(false)
    }
}

pub fn f75(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f121(v0.clone(), false)?;
    let v6: u64 = f121(v0, true)?;
    let a1735 = v6;
    let a1736 = v1;
    if if nat_le(a1735, a1736) {
        let a1743 = {
            let a1741 = {
                let a1737 = v1;
                let a1738 = v5;
                nat_add(a1737, a1738)?
            };
            let a1742 = {
                let a1739 = v3;
                let a1740 = 0u64;
                if nat_eq(a1739, a1740) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1741, a1742)?
        };
        let a1744 = v2;
        nat_le(a1743, a1744)
    } else {
        false
    } {
        if v4 {
            let a1749 = {
                let a1747 = {
                    let a1745 = v1;
                    let a1746 = v5;
                    nat_add(a1745, a1746)?
                };
                let a1748 = v6;
                nat_sub(a1747, a1748)
            };
            let a1750 = v2;
            Ok(nat_lt(a1749, a1750))
        } else {
            let a1755 = {
                let a1753 = {
                    let a1751 = v1;
                    let a1752 = v5;
                    nat_add(a1751, a1752)?
                };
                let a1754 = v6;
                nat_sub(a1753, a1754)
            };
            let a1756 = v2;
            Ok(nat_le(a1755, a1756))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1757 = v0;
    let a1758 = 0u64;
    if if nat_eq(a1757, a1758) {
        true
    } else {
        let a1759 = v0;
        let a1760 = 1u64;
        nat_eq(a1759, a1760)
    } {
        let a1761 = v1;
        let a1762 = 4294967296u64;
        nat_lt(a1761, a1762)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64) -> R<u64> {
    let a1767 = {
        let a1765 = v0;
        let a1766 = {
            let a1763 = v1;
            let a1764 = 1u64;
            nat_sub(a1763, a1764)
        };
        nat_add(a1765, a1766)?
    };
    let a1768 = v1;
    let a1769 = 0u64;
    Ok(nat_quot(a1767, a1768, a1769))
}

pub fn f78(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1770, _, _, _, _, _, _, _, _, _) => {
            h1770
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1771, _, _, _, _, _, _, _, _) => {
            h1771
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1772, _, _, _, _, _, _, _) => {
            h1772
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1773, _, _, _, _, _, _) => {
            h1773
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1776 = match v1.clone() {
                Adt2::C0(_, h1774, _, _, _, _, _, _, _, _, _) => {
                    h1774
                }
            };
            let a1777 = match v1.clone() {
                Adt2::C0(_, _, h1775, _, _, _, _, _, _, _, _) => {
                    h1775
                }
            };
            nat_mul(a1776, a1777)?
        };
        let v5: u64 = {
            let a1779 = v4;
            let a1780 = match v1.clone() {
                Adt2::C0(_, _, _, h1778, _, _, _, _, _, _, _) => {
                    h1778
                }
            };
            nat_mul(a1779, a1780)?
        };
        let a1793 = {
            let a1791 = {
                let a1782 = match v0.clone() {
                    Adt2::C0(_, h1781, _, _, _, _, _, _, _, _, _) => {
                        h1781
                    }
                };
                let a1783 = 1u64;
                nat_sub(a1782, a1783)
            };
            let a1792 = {
                let a1789 = f122(match v0.clone() {
                    Adt2::C0(_, _, h1784, _, _, _, _, _, _, _, _) => {
                        h1784
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1785, _, _, _, _, _, _, _, _, _) => {
                        h1785
                    }
                })?;
                let a1790 = f122(match v0 {
                    Adt2::C0(_, _, _, h1786, _, _, _, _, _, _, _) => {
                        h1786
                    }
                }, v2, {
                    let a1787 = v3;
                    let a1788 = v5;
                    nat_mul(a1787, a1788)?
                }, v4)?;
                nat_add(a1789, a1790)?
            };
            nat_add(a1791, a1792)?
        };
        let a1794 = f9(v1, 1u64)?;
        Ok(nat_lt(a1793, a1794))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1795 = v0;
    let a1796 = 2147483648u64;
    if if if if nat_lt(a1795, a1796) {
        let a1797 = v1;
        let a1798 = 2147483648u64;
        nat_lt(a1797, a1798)
    } else {
        false
    } {
        let a1801 = {
            let a1799 = v0;
            let a1800 = v1;
            nat_mul(a1799, a1800)?
        };
        let a1802 = 2147483648u64;
        nat_lt(a1801, a1802)
    } else {
        false
    } {
        let a1807 = {
            let a1805 = {
                let a1803 = v0;
                let a1804 = 1u64;
                nat_sub(a1803, a1804)
            };
            let a1806 = v2;
            nat_mul(a1805, a1806)?
        };
        let a1808 = 2147483648u64;
        nat_lt(a1807, a1808)
    } else {
        false
    } {
        let a1813 = {
            let a1811 = {
                let a1809 = v1;
                let a1810 = 1u64;
                nat_sub(a1809, a1810)
            };
            let a1812 = v2;
            nat_mul(a1811, a1812)?
        };
        let a1814 = 2147483648u64;
        Ok(nat_lt(a1813, a1814))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a1817 = 3u64;
    let a1818 = {
        let a1815 = v0;
        let a1816 = 3u64;
        nat_mul(a1815, a1816)?
    };
    nat_add(a1817, a1818)
}

pub fn f81(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
        Adt3::C0(h1819, _, _, _, _, _, _, _) => {
            h1819
        }
    }, match v0.clone() {
        Adt3::C0(_, h1820, _, _, _, _, _, _) => {
            h1820
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1821, _, _, _, _, _) => {
            h1821
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1822, _, _, _, _) => {
            h1822
        }
    }) {
        Ok(0u64)
    } else {
        let a1831 = match v0.clone() {
            Adt3::C0(h1823, _, _, _, _, _, _, _) => {
                h1823
            }
        };
        let a1832 = {
            let a1829 = match v0.clone() {
                Adt3::C0(_, h1824, _, _, _, _, _, _) => {
                    h1824
                }
            };
            let a1830 = {
                let a1827 = match v0.clone() {
                    Adt3::C0(_, _, h1825, _, _, _, _, _) => {
                        h1825
                    }
                };
                let a1828 = match v0 {
                    Adt3::C0(_, _, _, h1826, _, _, _, _) => {
                        h1826
                    }
                };
                nat_mul(a1827, a1828)?
            };
            nat_mul(a1829, a1830)?
        };
        nat_mul(a1831, a1832)
    }
}

pub fn f82(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1833 = v1;
    let a1834 = 13u64;
    if if nat_eq(a1833, a1834) {
        f23(match v0.clone() {
            Adt3::C0(h1835, _, _, _, _, _, _, _) => {
                h1835
            }
        }, match v0.clone() {
            Adt3::C0(_, h1836, _, _, _, _, _, _) => {
                h1836
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1837, _, _, _, _, _) => {
                h1837
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1838, _, _, _, _) => {
                h1838
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1844 = match v0.clone() {
                Adt3::C0(_, h1839, _, _, _, _, _, _) => {
                    h1839
                }
            };
            let a1845 = {
                let a1842 = match v0.clone() {
                    Adt3::C0(_, _, h1840, _, _, _, _, _) => {
                        h1840
                    }
                };
                let a1843 = match v0.clone() {
                    Adt3::C0(_, _, _, h1841, _, _, _, _) => {
                        h1841
                    }
                };
                nat_mul(a1842, a1843)?
            };
            nat_mul(a1844, a1845)?
        };
        let a1846 = v1;
        let a1847 = 13u64;
        if nat_eq(a1846, a1847) {
            let a1856 = 2u64;
            let a1857 = if f123(v0.clone(), v2) {
                let a1849 = v3;
                let a1850 = f124(match v0 {
                    Adt3::C0(h1848, _, _, _, _, _, _, _) => {
                        h1848
                    }
                }, false)?;
                nat_mul(a1849, a1850)?
            } else {
                let a1854 = v3;
                let a1855 = {
                    let a1852 = match v0 {
                        Adt3::C0(h1851, _, _, _, _, _, _, _) => {
                            h1851
                        }
                    };
                    let a1853 = 2u64;
                    nat_mul(a1852, a1853)?
                };
                nat_mul(a1854, a1855)?
            };
            nat_add(a1856, a1857)
        } else {
            let a1861 = v3;
            let a1862 = f124(match v0 {
                Adt3::C0(h1858, _, _, _, _, _, _, _) => {
                    h1858
                }
            }, {
                let a1859 = v1;
                let a1860 = 16u64;
                nat_eq(a1859, a1860)
            })?;
            nat_mul(a1861, a1862)
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1863 = v3;
    match m1863 {
        None => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1864, _, _, _, _, _, _, _, _, _, _) => {
                    h1864
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1865, _, _, _, _, _, _, _, _, _, _) => {
                    h1865
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1866, _, _, _, _, _, _, _, _, _) => {
                    h1866
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1867, _, _, _, _, _, _, _, _) => {
                    h1867
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1868, _, _, _, _, _, _, _) => {
                    h1868
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1869, _, _, _, _, _, _) => {
                    h1869
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1870, _, _, _, _, _, _, _, _, _) => {
                    h1870
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1871, _, _, _, _, _, _, _, _) => {
                    h1871
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1872, _, _, _, _, _, _, _) => {
                    h1872
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1873, _, _, _, _, _, _) => {
                    h1873
                }
            })
        }
        Some(v5) => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1874, _, _, _, _, _, _, _, _, _, _) => {
                    h1874
                }
            }, match v5.clone() {
                Adt2::C0(h1875, _, _, _, _, _, _, _, _, _, _) => {
                    h1875
                }
            }, match v4.clone() {
                Adt2::C0(h1876, _, _, _, _, _, _, _, _, _, _) => {
                    h1876
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1877, _, _, _, _, _, _, _, _, _) => {
                    h1877
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1878, _, _, _, _, _, _, _, _) => {
                    h1878
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1879, _, _, _, _, _, _, _) => {
                    h1879
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1880, _, _, _, _, _, _) => {
                    h1880
                }
            }, match v5.clone() {
                Adt2::C0(_, h1881, _, _, _, _, _, _, _, _, _) => {
                    h1881
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1882, _, _, _, _, _, _, _, _) => {
                    h1882
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1883, _, _, _, _, _, _, _) => {
                    h1883
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1884, _, _, _, _, _, _) => {
                    h1884
                }
            }, match v4.clone() {
                Adt2::C0(_, h1885, _, _, _, _, _, _, _, _, _) => {
                    h1885
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1886, _, _, _, _, _, _, _, _) => {
                    h1886
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1887, _, _, _, _, _, _, _) => {
                    h1887
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1888, _, _, _, _, _, _) => {
                    h1888
                }
            })
        }
    }
}

pub fn f84(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1889 = v7;
        let a1890 = 6u64;
        if nat_eq(a1889, a1890) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f126(v3, v4), f126(v5, v6), f127(v7, v8), f128(v1), f128(v9))
}

pub fn f85(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1891 = v0;
    match m1891 {
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
        Adt4::C26(_, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
    }
}

pub fn f86(v0: Adt4) -> u64 {
    let m1892 = v0;
    match m1892 {
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
        Adt4::C26(_, _, _, _) => {
            0u64
        }
    }
}

pub fn f87(v0: Adt4, v1: u64) -> u64 {
    let m1893 = v0;
    match m1893 {
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
            let a1894 = v1;
            let a1895 = 6u64;
            if nat_eq(a1894, a1895) {
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
        Adt4::C26(v49, _, _, _) => {
            let a1896 = v1;
            let a1897 = 6u64;
            if if if nat_eq(a1896, a1897) {
                true
            } else {
                let a1898 = v1;
                let a1899 = 7u64;
                nat_eq(a1898, a1899)
            } {
                let a1900 = v49;
                let a1901 = 30u64;
                nat_eq(a1900, a1901)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
    }
}

pub fn f88(v0: Adt4, v1: u64) -> Adt7 {
    let a1902 = v1;
    let a1903 = 4u64;
    if nat_lt(a1902, a1903) {
        Adt7::C0(v1)
    } else {
        let a1904 = v1;
        let a1905 = 4u64;
        if nat_eq(a1904, a1905) {
            Adt7::C1(0u64)
        } else {
            let a1906 = v1;
            let a1907 = 5u64;
            if nat_eq(a1906, a1907) {
                Adt7::C1(3u64)
            } else {
                let a1908 = v1;
                let a1909 = 6u64;
                if nat_eq(a1908, a1909) {
                    Adt7::C2(f87(v0, v1), 0u64, false)
                } else {
                    let a1910 = v1;
                    let a1911 = 7u64;
                    if nat_eq(a1910, a1911) {
                        Adt7::C2(0u64, f87(v0, v1), false)
                    } else {
                        let a1912 = v1;
                        let a1913 = 16u64;
                        if nat_lt(a1912, a1913) {
                            Adt7::C1({
                                let a1914 = v1;
                                let a1915 = 8u64;
                                nat_sub(a1914, a1915)
                            })
                        } else {
                            let a1916 = v1;
                            let a1917 = 20u64;
                            if nat_lt(a1916, a1917) {
                                Adt7::C5({
                                    let a1918 = v1;
                                    let a1919 = 16u64;
                                    nat_sub(a1918, a1919)
                                })
                            } else {
                                let a1920 = v1;
                                let a1921 = 20u64;
                                if nat_eq(a1920, a1921) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1922 = v1;
                                    let a1923 = 21u64;
                                    if nat_eq(a1922, a1923) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a1924 = v1;
                                        let a1925 = 22u64;
                                        if nat_eq(a1924, a1925) {
                                            Adt7::C0(5u64)
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
    }
}

pub fn f89(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1927 = v0;
        let a1928 = match v1.clone() {
            Adt3::C0(h1926, _, _, _, _, _, _, _) => {
                h1926
            }
        };
        let a1929 = 0u64;
        nat_quot(a1927, a1928, a1929)
    };
    let v3: u64 = {
        let a1931 = v2;
        let a1932 = match v1.clone() {
            Adt3::C0(_, h1930, _, _, _, _, _, _) => {
                h1930
            }
        };
        let a1933 = 0u64;
        nat_quot(a1931, a1932, a1933)
    };
    f129({
        let a1935 = v0;
        let a1936 = match v1.clone() {
            Adt3::C0(h1934, _, _, _, _, _, _, _) => {
                h1934
            }
        };
        let a1937 = 0u64;
        nat_rem(a1935, a1936, a1937)
    }, {
        let a1939 = v2;
        let a1940 = match v1.clone() {
            Adt3::C0(_, h1938, _, _, _, _, _, _) => {
                h1938
            }
        };
        let a1941 = 0u64;
        nat_rem(a1939, a1940, a1941)
    }, {
        let a1943 = v3;
        let a1944 = match v1.clone() {
            Adt3::C0(_, _, h1942, _, _, _, _, _) => {
                h1942
            }
        };
        let a1945 = 0u64;
        nat_rem(a1943, a1944, a1945)
    }, {
        let a1947 = v3;
        let a1948 = match v1.clone() {
            Adt3::C0(_, _, h1946, _, _, _, _, _) => {
                h1946
            }
        };
        let a1949 = 0u64;
        nat_quot(a1947, a1948, a1949)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1950, _, _, _) => {
            h1950
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1951, _, _) => {
            h1951
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1952, _) => {
            h1952
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1953) => {
            h1953
        }
    })
}

pub fn f90(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1971 = {
        let a1966 = f130(v2, {
            let a1957 = {
                let a1954 = v0;
                let a1955 = v1;
                let a1956 = 0u64;
                nat_quot(a1954, a1955, a1956)
            };
            let a1958 = v2;
            let a1959 = 0u64;
            nat_rem(a1957, a1958, a1959)
        }, {
            let a1963 = {
                let a1960 = v0;
                let a1961 = v1;
                let a1962 = 0u64;
                nat_quot(a1960, a1961, a1962)
            };
            let a1964 = v2;
            let a1965 = 0u64;
            nat_quot(a1963, a1964, a1965)
        })?;
        let a1967 = v1;
        nat_mul(a1966, a1967)?
    };
    let a1972 = {
        let a1968 = v0;
        let a1969 = v1;
        let a1970 = 0u64;
        nat_rem(a1968, a1969, a1970)
    };
    nat_add(a1971, a1972)
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1974 = v0;
        let a1975 = match v2.clone() {
            Adt3::C0(h1973, _, _, _, _, _, _, _) => {
                h1973
            }
        };
        let a1976 = 0u64;
        nat_quot(a1974, a1975, a1976)
    };
    let v4: u64 = {
        let a1978 = v3;
        let a1979 = match v2.clone() {
            Adt3::C0(_, h1977, _, _, _, _, _, _) => {
                h1977
            }
        };
        let a1980 = 0u64;
        nat_quot(a1978, a1979, a1980)
    };
    f131({
        let a1982 = v0;
        let a1983 = match v2.clone() {
            Adt3::C0(h1981, _, _, _, _, _, _, _) => {
                h1981
            }
        };
        let a1984 = 0u64;
        nat_rem(a1982, a1983, a1984)
    }, {
        let a1986 = v3;
        let a1987 = match v2.clone() {
            Adt3::C0(_, h1985, _, _, _, _, _, _) => {
                h1985
            }
        };
        let a1988 = 0u64;
        nat_rem(a1986, a1987, a1988)
    }, {
        let a1990 = v4;
        let a1991 = match v2.clone() {
            Adt3::C0(_, _, h1989, _, _, _, _, _) => {
                h1989
            }
        };
        let a1992 = 0u64;
        nat_rem(a1990, a1991, a1992)
    }, {
        let a1994 = v4;
        let a1995 = match v2 {
            Adt3::C0(_, _, h1993, _, _, _, _, _) => {
                h1993
            }
        };
        let a1996 = 0u64;
        nat_quot(a1994, a1995, a1996)
    }, match v1.clone() {
        Adt3::C0(h1997, _, _, _, _, _, _, _) => {
            h1997
        }
    }, match v1.clone() {
        Adt3::C0(_, h1998, _, _, _, _, _, _) => {
            h1998
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1999, _, _, _, _, _) => {
            h1999
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2000, _, _, _, _) => {
            h2000
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2001, _, _, _) => {
            h2001
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2002, _, _) => {
            h2002
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2003, _) => {
            h2003
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2004) => {
            h2004
        }
    })
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2006 = v0;
        let a2007 = match v3.clone() {
            Adt3::C0(h2005, _, _, _, _, _, _, _) => {
                h2005
            }
        };
        let a2008 = 0u64;
        nat_quot(a2006, a2007, a2008)
    };
    let v6: u64 = {
        let a2010 = v5;
        let a2011 = match v3.clone() {
            Adt3::C0(_, h2009, _, _, _, _, _, _) => {
                h2009
            }
        };
        let a2012 = 0u64;
        nat_quot(a2010, a2011, a2012)
    };
    let v7: u64 = {
        let a2014 = v0;
        let a2015 = match v3.clone() {
            Adt3::C0(h2013, _, _, _, _, _, _, _) => {
                h2013
            }
        };
        let a2016 = 0u64;
        nat_rem(a2014, a2015, a2016)
    };
    let v8: u64 = {
        let a2018 = v5;
        let a2019 = match v3.clone() {
            Adt3::C0(_, h2017, _, _, _, _, _, _) => {
                h2017
            }
        };
        let a2020 = 0u64;
        nat_rem(a2018, a2019, a2020)
    };
    let v9: u64 = {
        let a2022 = v6;
        let a2023 = match v3.clone() {
            Adt3::C0(_, _, h2021, _, _, _, _, _) => {
                h2021
            }
        };
        let a2024 = 0u64;
        nat_rem(a2022, a2023, a2024)
    };
    let v10: u64 = {
        let a2026 = v6;
        let a2027 = match v3.clone() {
            Adt3::C0(_, _, h2025, _, _, _, _, _) => {
                h2025
            }
        };
        let a2028 = 0u64;
        nat_quot(a2026, a2027, a2028)
    };
    let v11: u64 = {
        let a2029 = v4;
        let a2030 = 0u64;
        if nat_eq(a2029, a2030) {
            match v1.clone() {
                Adt3::C0(h2031, _, _, _, _, _, _, _) => {
                    h2031
                }
            }
        } else {
            let a2032 = v4;
            let a2033 = 1u64;
            if nat_eq(a2032, a2033) {
                match v1.clone() {
                    Adt3::C0(_, h2034, _, _, _, _, _, _) => {
                        h2034
                    }
                }
            } else {
                let a2035 = v4;
                let a2036 = 2u64;
                if nat_eq(a2035, a2036) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2037, _, _, _, _, _) => {
                            h2037
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2038, _, _, _, _) => {
                            h2038
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2045 = v11;
        let a2046 = {
            let a2039 = v4;
            let a2040 = 0u64;
            if nat_eq(a2039, a2040) {
                v7
            } else {
                let a2041 = v4;
                let a2042 = 1u64;
                if nat_eq(a2041, a2042) {
                    v8
                } else {
                    let a2043 = v4;
                    let a2044 = 2u64;
                    if nat_eq(a2043, a2044) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2045, a2046)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f129(if if v12 {
        let a2047 = v4;
        let a2048 = 0u64;
        nat_eq(a2047, a2048)
    } else {
        false
    } {
        let a2049 = v7;
        let a2050 = v11;
        nat_sub(a2049, a2050)
    } else {
        v7
    }, if if v12 {
        let a2051 = v4;
        let a2052 = 1u64;
        nat_eq(a2051, a2052)
    } else {
        false
    } {
        let a2053 = v8;
        let a2054 = v11;
        nat_sub(a2053, a2054)
    } else {
        v8
    }, if if v12 {
        let a2055 = v4;
        let a2056 = 2u64;
        nat_eq(a2055, a2056)
    } else {
        false
    } {
        let a2057 = v9;
        let a2058 = v11;
        nat_sub(a2057, a2058)
    } else {
        v9
    }, if if v12 {
        let a2059 = v4;
        let a2060 = 3u64;
        nat_eq(a2059, a2060)
    } else {
        false
    } {
        let a2061 = v10;
        let a2062 = v11;
        nat_sub(a2061, a2062)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2063, _, _, _) => {
            h2063
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2064, _, _) => {
            h2064
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2065, _) => {
            h2065
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2066) => {
            h2066
        }
    })?, f89(v0, v3)?))
}

pub fn f93(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f132(v0, v2.clone(), 0u64);
    let v9: u64 = f132(v0, v2.clone(), 1u64);
    let v10: u64 = f132(v0, v2.clone(), 2u64);
    let v11: u64 = f132(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f133(v1.clone(), f134(v8, v3, match v1.clone() {
            Adt3::C0(h2067, _, _, _, _, _, _, _) => {
                h2067
            }
        })?, f134(v9, v4, match v1.clone() {
            Adt3::C0(_, h2068, _, _, _, _, _, _) => {
                h2068
            }
        })?, f134(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2069, _, _, _, _, _) => {
                h2069
            }
        })?, f134(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2070, _, _, _, _) => {
                h2070
            }
        })?)?, f89(v0, v2)?))
    } else {
        if if if if f135(v8, v3, match v1.clone() {
            Adt3::C0(h2071, _, _, _, _, _, _, _) => {
                h2071
            }
        })? {
            f135(v9, v4, match v1.clone() {
                Adt3::C0(_, h2072, _, _, _, _, _, _) => {
                    h2072
                }
            })?
        } else {
            false
        } {
            f135(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2073, _, _, _, _, _) => {
                    h2073
                }
            })?
        } else {
            false
        } {
            f135(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2074, _, _, _, _) => {
                    h2074
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f133(v1, f136(v8, v3)?, f136(v9, v4)?, f136(v10, v5)?, f136(v11, v6)?)?, f89(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
        }
    }
}

pub fn f94(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f132(v0, v2.clone(), 0u64);
    let v5: u64 = f132(v0, v2.clone(), 1u64);
    let v6: u64 = f132(v0, v2.clone(), 2u64);
    let v7: u64 = f132(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f133(v1.clone(), f137(v4, v3, match v1 {
        Adt3::C0(h2075, _, _, _, _, _, _, _) => {
            h2075
        }
    })?, v5, v6, v7)?, f89(v0, v2)?))
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2076 = v0;
    let a2077 = v3;
    if nat_lt(a2076, a2077) {
        Ok(Adt5::C1(false, {
            let a2078 = v0;
            let a2079 = 4u64;
            nat_mul(a2078, a2079)?
        }, {
            let a2080 = v0;
            let a2081 = 4u64;
            nat_mul(a2080, a2081)?
        }))
    } else {
        let v6: u64 = {
            let a2082 = v0;
            let a2083 = v3;
            nat_sub(a2082, a2083)
        };
        let v7: u64 = f89(v6, v1)?;
        let v8: u64 = f138(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f96(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2084 = v0;
    let a2085 = v3;
    if nat_lt(a2084, a2085) {
        Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2086 = v0;
            let a2087 = v3;
            nat_sub(a2086, a2087)
        };
        let v5: u64 = f91(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f89(v4, v1)?, 3u64, v5))
    }
}

pub fn f97(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2089 = v1;
        let a2090 = match v3.clone() {
            Adt3::C0(h2088, _, _, _, _, _, _, _) => {
                h2088
            }
        };
        let a2091 = 0u64;
        nat_rem(a2089, a2090, a2091)
    };
    let v5: u64 = {
        let a2097 = {
            let a2093 = v1;
            let a2094 = match v3.clone() {
                Adt3::C0(h2092, _, _, _, _, _, _, _) => {
                    h2092
                }
            };
            let a2095 = 0u64;
            nat_quot(a2093, a2094, a2095)
        };
        let a2098 = match v3.clone() {
            Adt3::C0(_, h2096, _, _, _, _, _, _) => {
                h2096
            }
        };
        let a2099 = 0u64;
        nat_rem(a2097, a2098, a2099)
    };
    let v6: bool = f139(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f91(v1, v2, v3.clone())?
    } else {
        0u64
    }, f89(v1, v3)?))
}

pub fn f98(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a2100 = v3;
        let a2101 = 8u64;
        nat_add(a2100, a2101)?
    }, 0u64, f89(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f91(v0, v5, v2.clone())?
    }, 3u64, f89(v0, v2)?))
}

pub fn f99(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a2103 = v0;
        let a2104 = match v2.clone() {
            Adt3::C0(h2102, _, _, _, _, _, _, _) => {
                h2102
            }
        };
        let a2105 = 0u64;
        nat_rem(a2103, a2104, a2105)
    };
    let v7: u64 = {
        let a2107 = v0;
        let a2108 = match v2.clone() {
            Adt3::C0(h2106, _, _, _, _, _, _, _) => {
                h2106
            }
        };
        let a2109 = 0u64;
        nat_quot(a2107, a2108, a2109)
    };
    let v8: u64 = {
        let a2111 = v7;
        let a2112 = match v2.clone() {
            Adt3::C0(_, h2110, _, _, _, _, _, _) => {
                h2110
            }
        };
        let a2113 = 0u64;
        nat_rem(a2111, a2112, a2113)
    };
    let v9: u64 = {
        let a2115 = v7;
        let a2116 = match v2.clone() {
            Adt3::C0(_, h2114, _, _, _, _, _, _) => {
                h2114
            }
        };
        let a2117 = 0u64;
        nat_quot(a2115, a2116, a2117)
    };
    let v10: u64 = {
        let a2119 = v9;
        let a2120 = match v2.clone() {
            Adt3::C0(_, _, h2118, _, _, _, _, _) => {
                h2118
            }
        };
        let a2121 = 0u64;
        nat_rem(a2119, a2120, a2121)
    };
    let v11: u64 = {
        let a2123 = v9;
        let a2124 = match v2.clone() {
            Adt3::C0(_, _, h2122, _, _, _, _, _) => {
                h2122
            }
        };
        let a2125 = 0u64;
        nat_quot(a2123, a2124, a2125)
    };
    let v12: u64 = f89(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2134 = {
                let a2129 = {
                    let a2126 = v10;
                    let a2127 = v4;
                    let a2128 = 0u64;
                    nat_quot(a2126, a2127, a2128)
                };
                let a2130 = v5;
                nat_mul(a2129, a2130)?
            };
            let a2135 = {
                let a2131 = v8;
                let a2132 = v4;
                let a2133 = 0u64;
                nat_quot(a2131, a2132, a2133)
            };
            nat_add(a2134, a2135)?
        };
        let v14: u64 = {
            let a2155 = v6;
            let a2156 = {
                let a2153 = match v1.clone() {
                    Adt3::C0(h2136, _, _, _, _, _, _, _) => {
                        h2136
                    }
                };
                let a2154 = {
                    let a2151 = {
                        let a2137 = v8;
                        let a2138 = v4;
                        let a2139 = 0u64;
                        nat_rem(a2137, a2138, a2139)
                    };
                    let a2152 = {
                        let a2149 = match v1.clone() {
                            Adt3::C0(_, h2140, _, _, _, _, _, _) => {
                                h2140
                            }
                        };
                        let a2150 = {
                            let a2147 = {
                                let a2141 = v10;
                                let a2142 = v4;
                                let a2143 = 0u64;
                                nat_rem(a2141, a2142, a2143)
                            };
                            let a2148 = {
                                let a2145 = match v1.clone() {
                                    Adt3::C0(_, _, h2144, _, _, _, _, _) => {
                                        h2144
                                    }
                                };
                                let a2146 = v13;
                                nat_mul(a2145, a2146)?
                            };
                            nat_add(a2147, a2148)?
                        };
                        nat_mul(a2149, a2150)?
                    };
                    nat_add(a2151, a2152)?
                };
                nat_mul(a2153, a2154)?
            };
            nat_add(a2155, a2156)?
        };
        Ok(Adt5::C1(false, f89(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2162 = {
                let a2160 = {
                    let a2157 = v11;
                    let a2158 = v5;
                    let a2159 = 0u64;
                    nat_rem(a2157, a2158, a2159)
                };
                let a2161 = v4;
                nat_mul(a2160, a2161)?
            };
            let a2163 = v8;
            nat_add(a2162, a2163)?
        };
        let v16: u64 = {
            let a2169 = {
                let a2167 = {
                    let a2164 = v11;
                    let a2165 = v5;
                    let a2166 = 0u64;
                    nat_quot(a2164, a2165, a2166)
                };
                let a2168 = v4;
                nat_mul(a2167, a2168)?
            };
            let a2170 = v10;
            nat_add(a2169, a2170)?
        };
        let a2172 = v15;
        let a2173 = match v1.clone() {
            Adt3::C0(_, h2171, _, _, _, _, _, _) => {
                h2171
            }
        };
        if if nat_lt(a2172, a2173) {
            let a2175 = v16;
            let a2176 = match v1.clone() {
                Adt3::C0(_, _, h2174, _, _, _, _, _) => {
                    h2174
                }
            };
            nat_lt(a2175, a2176)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f89({
                let a2185 = v6;
                let a2186 = {
                    let a2183 = match v1.clone() {
                        Adt3::C0(h2177, _, _, _, _, _, _, _) => {
                            h2177
                        }
                    };
                    let a2184 = {
                        let a2181 = v15;
                        let a2182 = {
                            let a2179 = match v1.clone() {
                                Adt3::C0(_, h2178, _, _, _, _, _, _) => {
                                    h2178
                                }
                            };
                            let a2180 = v16;
                            nat_mul(a2179, a2180)?
                        };
                        nat_add(a2181, a2182)?
                    };
                    nat_mul(a2183, a2184)?
                };
                nat_add(a2185, a2186)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f89(v0, v2.clone())?;
    let v4: u64 = f89(v0, v1.clone())?;
    let a2191 = {
        let a2188 = v0;
        let a2189 = match v1 {
            Adt3::C0(h2187, _, _, _, _, _, _, _) => {
                h2187
            }
        };
        let a2190 = 0u64;
        nat_rem(a2188, a2189, a2190)
    };
    let a2192 = 0u64;
    if nat_eq(a2191, a2192) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f89({
            let a2193 = v0;
            let a2194 = 1u64;
            nat_sub(a2193, a2194)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2195 = v3;
    let a2196 = 13u64;
    if nat_eq(a2195, a2196) {
        f140(v0, v1, v4)
    } else {
        let v5: u64 = f124(match v1.clone() {
            Adt3::C0(h2197, _, _, _, _, _, _, _) => {
                h2197
            }
        }, {
            let a2198 = v3;
            let a2199 = 16u64;
            nat_eq(a2198, a2199)
        })?;
        let v6: u64 = {
            let a2200 = v0;
            let a2201 = v5;
            let a2202 = 0u64;
            nat_quot(a2200, a2201, a2202)
        };
        let v7: u64 = {
            let a2203 = v0;
            let a2204 = v5;
            let a2205 = 0u64;
            nat_rem(a2203, a2204, a2205)
        };
        let v8: u64 = {
            let a2209 = 1u64;
            let a2210 = {
                let a2207 = match v1.clone() {
                    Adt3::C0(h2206, _, _, _, _, _, _, _) => {
                        h2206
                    }
                };
                let a2208 = 2u64;
                nat_mul(a2207, a2208)?
            };
            nat_add(a2209, a2210)?
        };
        let a2211 = v7;
        let a2212 = v8;
        if nat_lt(a2211, a2212) {
            f141(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f89(v6, v2)?;
            let a2213 = v3;
            let a2214 = 14u64;
            if nat_eq(a2213, a2214) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2215 = v7;
                let a2216 = v8;
                if nat_eq(a2215, a2216) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2219 = v7;
                    let a2220 = {
                        let a2217 = v8;
                        let a2218 = 1u64;
                        nat_add(a2217, a2218)?
                    };
                    if nat_eq(a2219, a2220) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2221, _, _, _, _, _, _, _) => {
                                h2221
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

pub fn f102(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2222 = v0;
    let a2223 = 0u64;
    if nat_eq(a2222, a2223) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2230 = {
            let a2224 = v0;
            let a2225 = 1u64;
            nat_add(a2224, a2225)?
        };
        let a2231 = {
            let a2228 = 2u64;
            let a2229 = {
                let a2226 = v3;
                let a2227 = 2u64;
                nat_mul(a2226, a2227)?
            };
            nat_add(a2228, a2229)?
        };
        if nat_eq(a2230, a2231) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2232 = v0;
                let a2233 = 1u64;
                nat_sub(a2232, a2233)
            };
            let v5: u64 = {
                let a2234 = v4;
                let a2235 = 2u64;
                let a2236 = 0u64;
                nat_quot(a2234, a2235, a2236)
            };
            let a2240 = {
                let a2237 = v4;
                let a2238 = 2u64;
                let a2239 = 0u64;
                nat_rem(a2237, a2238, a2239)
            };
            let a2241 = 0u64;
            if nat_eq(a2240, a2241) {
                Ok(Adt5::C4(17u64, 0u64, f89(v5, v1)?, 4u64, f89(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2242 = v0;
    let a2243 = f18(v1, v2, v3)?;
    if nat_lt(a2242, a2243) {
        let a2244 = v0;
        let a2245 = v1;
        if nat_lt(a2244, a2245) {
            Ok(Adt5::C1(false, {
                let a2246 = 4u64;
                let a2247 = v0;
                nat_mul(a2246, a2247)?
            }, {
                let a2248 = 4u64;
                let a2249 = v0;
                nat_mul(a2248, a2249)?
            }))
        } else {
            let v4: u64 = {
                let a2250 = v0;
                let a2251 = v1;
                nat_sub(a2250, a2251)
            };
            let v5: u64 = f142(v4, v2)?;
            let a2252 = v5;
            let a2253 = 2u64;
            if nat_lt(a2252, a2253) {
                Ok(Adt5::C4(6u64, {
                    let a2254 = v5;
                    let a2255 = 0u64;
                    if nat_eq(a2254, a2255) {
                        4u64
                    } else {
                        5u64
                    }
                }, f143(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f144(v4, v2)?;
                Ok(Adt5::C4(8u64, 3u64, v6, 1u64, if f145(v4, v2)? {
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

pub fn f104(v0: Adt3, v1: Adt3) -> R<u64> {
    let a2263 = {
        let a2256 = 2u64;
        let a2257 = f81(v1.clone())?;
        nat_mul(a2256, a2257)?
    };
    let a2264 = {
        let a2261 = f81(v1)?;
        let a2262 = {
            let a2259 = 1u64;
            let a2260 = match v0 {
                Adt3::C0(h2258, _, _, _, _, _, _, _) => {
                    h2258
                }
            };
            nat_add(a2259, a2260)?
        };
        nat_mul(a2261, a2262)?
    };
    nat_add(a2263, a2264)
}

pub fn f105(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f89(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f106(v0: u64, v1: u64) -> bool {
    let a2265 = v1;
    let a2266 = 2147483647u64;
    if nat_le(a2265, a2266) {
        let a2267 = v1;
        let a2268 = v0;
        nat_lt(a2267, a2268)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2269 = f70(v0);
    match m2269 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2271 = v2;
                let a2272 = match v7.clone() {
                    Adt11::C0(_, h2270, _) => {
                        h2270
                    }
                };
                nat_mul(a2271, a2272)?
            };
            let m2285 = f146(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2273, _, _, _, _, _, _, _) => {
                    h2273
                }
            }, match v5.clone() {
                Adt3::C0(_, h2274, _, _, _, _, _, _) => {
                    h2274
                }
            }, match v5 {
                Adt3::C0(_, _, h2275, _, _, _, _, _) => {
                    h2275
                }
            }, {
                let a2278 = match v3.clone() {
                    Adt3::C0(h2276, _, _, _, _, _, _, _) => {
                        h2276
                    }
                };
                let a2279 = match v7.clone() {
                    Adt11::C0(_, h2277, _) => {
                        h2277
                    }
                };
                nat_mul(a2278, a2279)?
            }, match v3.clone() {
                Adt3::C0(_, h2280, _, _, _, _, _, _) => {
                    h2280
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2281, _, _, _) => {
                    h2281
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2282, _, _) => {
                    h2282
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2283, _) => {
                    h2283
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2284) => {
                    h2284
                }
            })?;
            match m2285 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2286 = v0;
                    let a2287 = 0u64;
                    if if nat_eq(a2286, a2287) {
                        true
                    } else {
                        let a2288 = v0;
                        let a2289 = 26u64;
                        nat_eq(a2288, a2289)
                    } {
                        Ok(Adt5::C1(false, v9, f89({
                            let a2293 = {
                                let a2291 = v1;
                                let a2292 = match v4.clone() {
                                    Adt3::C0(h2290, _, _, _, _, _, _, _) => {
                                        h2290
                                    }
                                };
                                nat_mul(a2291, a2292)?
                            };
                            let a2294 = v8;
                            nat_add(a2293, a2294)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2295) => {
                                h2295
                            }
                        }, f89({
                            let a2299 = {
                                let a2297 = v1;
                                let a2298 = match v4.clone() {
                                    Adt3::C0(h2296, _, _, _, _, _, _, _) => {
                                        h2296
                                    }
                                };
                                nat_mul(a2297, a2298)?
                            };
                            let a2300 = v8;
                            nat_add(a2299, a2300)?
                        }, v4.clone())?, {
                            let a2307 = {
                                let a2305 = {
                                    let a2302 = match v7 {
                                        Adt11::C0(_, h2301, _) => {
                                            h2301
                                        }
                                    };
                                    let a2303 = 1u64;
                                    nat_sub(a2302, a2303)
                                };
                                let a2306 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2304, _, _, _) => {
                                        h2304
                                    }
                                };
                                nat_mul(a2305, a2306)?
                            };
                            let a2308 = 4u64;
                            nat_add(a2307, a2308)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2309, _, _, _) => {
                                h2309
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2310 = v1;
    let a2311 = 0u64;
    if if nat_eq(a2310, a2311) {
        true
    } else {
        let a2312 = v2;
        let a2313 = 0u64;
        nat_eq(a2312, a2313)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f109(v0: u64, v1: u64) -> bool {
    let a2314 = v0;
    let a2315 = 4294967295u64;
    if nat_eq(a2314, a2315) {
        true
    } else {
        let a2316 = v0;
        let a2317 = 2147483648u64;
        if nat_lt(a2316, a2317) {
            let a2318 = v0;
            let a2319 = v1;
            nat_lt(a2318, a2319)
        } else {
            false
        }
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2326 = v3;
        let a2327 = {
            let a2323 = f89({
                let a2320 = v0;
                let a2321 = 2u64;
                let a2322 = 0u64;
                nat_quot(a2320, a2321, a2322)
            }, v1)?;
            let a2324 = v2;
            let a2325 = 0u64;
            nat_quot(a2323, a2324, a2325)
        };
        nat_add(a2326, a2327)?
    };
    let a2331 = {
        let a2328 = v0;
        let a2329 = 2u64;
        let a2330 = 0u64;
        nat_rem(a2328, a2329, a2330)
    };
    let a2332 = 0u64;
    if nat_eq(a2331, a2332) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2333 = v4;
        let a2334 = 0u64;
        if nat_eq(a2333, a2334) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f111(v0: u64, v1: u64) -> R<u64> {
    let a2337 = 8u64;
    let a2338 = {
        let a2335 = f147(v0);
        let a2336 = v1;
        nat_mul(a2335, a2336)?
    };
    nat_add(a2337, a2338)
}

pub fn f112(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2339, _, _, _, _, _, _, _) => {
            h2339
        }
    };
    let v7: u64 = f147(v3);
    let v8: u64 = {
        let a2342 = 1u64;
        let a2343 = {
            let a2340 = v6;
            let a2341 = v7;
            nat_mul(a2340, a2341)?
        };
        nat_add(a2342, a2343)?
    };
    let a2344 = v0;
    let a2345 = 0u64;
    if nat_eq(a2344, a2345) {
        Ok(f148(0u64))
    } else {
        let a2346 = v0;
        let a2347 = v8;
        if nat_lt(a2346, a2347) {
            let v9: u64 = {
                let a2348 = v0;
                let a2349 = 1u64;
                nat_sub(a2348, a2349)
            };
            let v10: u64 = {
                let a2350 = v9;
                let a2351 = v7;
                let a2352 = 0u64;
                nat_rem(a2350, a2351, a2352)
            };
            let v11: u64 = f89({
                let a2358 = {
                    let a2353 = v1;
                    let a2354 = v6;
                    nat_mul(a2353, a2354)?
                };
                let a2359 = {
                    let a2355 = v9;
                    let a2356 = v7;
                    let a2357 = 0u64;
                    nat_quot(a2355, a2356, a2357)
                };
                nat_add(a2358, a2359)?
            }, v2)?;
            let a2360 = v3;
            let a2361 = 30u64;
            if nat_eq(a2360, a2361) {
                let a2362 = v10;
                let a2363 = 0u64;
                if nat_eq(a2362, a2363) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2364 = v10;
                    let a2365 = 1u64;
                    if nat_eq(a2364, a2365) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2366 = v10;
                        let a2367 = 2u64;
                        if nat_eq(a2366, a2367) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2368 = v10;
                let a2369 = 0u64;
                if nat_eq(a2368, a2369) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2370 = v10;
                    let a2371 = 1u64;
                    if nat_eq(a2370, a2371) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2372 = v0;
                let a2373 = v8;
                nat_sub(a2372, a2373)
            };
            let a2374 = v12;
            let a2375 = 0u64;
            if nat_eq(a2374, a2375) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2376 = v12;
                let a2377 = 1u64;
                if nat_eq(a2376, a2377) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2378 = v12;
                    let a2379 = 2u64;
                    if nat_eq(a2378, a2379) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2380 = v12;
                        let a2381 = 3u64;
                        if nat_eq(a2380, a2381) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2382 = v12;
                            let a2383 = 4u64;
                            if nat_eq(a2382, a2383) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2384 = v12;
                                let a2385 = 5u64;
                                if nat_eq(a2384, a2385) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2386 = v12;
                                    let a2387 = 6u64;
                                    if nat_eq(a2386, a2387) {
                                        if f149(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2388 = 3u64;
                                                let a2389 = v1;
                                                nat_add(a2388, a2389)?
                                            }))
                                        } else {
                                            Ok(Adt5::C0(Adt6::C1))
                                        }
                                    } else {
                                        Ok(Adt5::C0(Adt6::C1))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f113(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a2390 = v0;
        let a2391 = f65(v3);
        let a2392 = 0u64;
        nat_quot(a2390, a2391, a2392)
    };
    let v5: u64 = {
        let a2397 = 3u64;
        let a2398 = {
            let a2394 = v4;
            let a2395 = match v1.clone() {
                Adt3::C0(h2393, _, _, _, _, _, _, _) => {
                    h2393
                }
            };
            let a2396 = 0u64;
            nat_quot(a2394, a2395, a2396)
        };
        nat_add(a2397, a2398)?
    };
    let v6: u64 = f89(v4, v1)?;
    let v7: u64 = f89(v4, v2)?;
    let a2399 = v3;
    let a2400 = 30u64;
    if nat_eq(a2399, a2400) {
        let a2404 = {
            let a2401 = v0;
            let a2402 = 3u64;
            let a2403 = 0u64;
            nat_rem(a2401, a2402, a2403)
        };
        let a2405 = 0u64;
        if nat_eq(a2404, a2405) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2409 = {
                let a2406 = v0;
                let a2407 = 3u64;
                let a2408 = 0u64;
                nat_rem(a2406, a2407, a2408)
            };
            let a2410 = 1u64;
            if nat_eq(a2409, a2410) {
                Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt5::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt5::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f114(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a2411 = v0;
        let a2412 = 8388608u64;
        let a2413 = 0u64;
        nat_quot(a2411, a2412, a2413)
    };
    let a2414 = v0;
    let a2415 = 2147483648u64;
    if if if nat_lt(a2414, a2415) {
        let a2416 = 127u64;
        let a2417 = v1;
        nat_le(a2416, a2417)
    } else {
        false
    } {
        let a2418 = v1;
        let a2419 = 190u64;
        nat_lt(a2418, a2419)
    } else {
        false
    } {
        let v2: u64 = {
            let a2423 = 8388608u64;
            let a2424 = {
                let a2420 = v0;
                let a2421 = 8388608u64;
                let a2422 = 0u64;
                nat_rem(a2420, a2421, a2422)
            };
            nat_add(a2423, a2424)?
        };
        let a2425 = v1;
        let a2426 = 150u64;
        if nat_lt(a2425, a2426) {
            let v3: u64 = f150({
                let a2427 = 150u64;
                let a2428 = v1;
                nat_sub(a2427, a2428)
            });
            let a2432 = {
                let a2429 = v2;
                let a2430 = v3;
                let a2431 = 0u64;
                nat_rem(a2429, a2430, a2431)
            };
            let a2433 = 0u64;
            if nat_eq(a2432, a2433) {
                let a2434 = v2;
                let a2435 = v3;
                let a2436 = 0u64;
                Ok(nat_quot(a2434, a2435, a2436))
            } else {
                Ok(0u64)
            }
        } else {
            let a2439 = f150({
                let a2437 = v1;
                let a2438 = 150u64;
                nat_sub(a2437, a2438)
            });
            let a2440 = v2;
            nat_mul(a2439, a2440)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> bool {
    let a2441 = v0;
    let a2442 = 1u64;
    if nat_le(a2441, a2442) {
        true
    } else {
        let a2446 = {
            let a2443 = v1;
            let a2444 = v2;
            let a2445 = 1u64;
            nat_rem(a2443, a2444, a2445)
        };
        let a2447 = 0u64;
        nat_eq(a2446, a2447)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2448 = v1;
    let a2449 = 1u64;
    if if if nat_le(a2448, a2449) {
        true
    } else {
        let a2450 = v0;
        let a2451 = v2;
        nat_le(a2450, a2451)
    } {
        let a2452 = v3;
        let a2453 = 1u64;
        if nat_le(a2452, a2453) {
            true
        } else {
            let a2460 = {
                let a2458 = v0;
                let a2459 = {
                    let a2456 = {
                        let a2454 = v1;
                        let a2455 = 1u64;
                        nat_sub(a2454, a2455)
                    };
                    let a2457 = v2;
                    nat_mul(a2456, a2457)?
                };
                nat_add(a2458, a2459)?
            };
            let a2461 = v4;
            nat_le(a2460, a2461)
        }
    } else {
        false
    } {
        let a2462 = v5;
        let a2463 = 1u64;
        if nat_le(a2462, a2463) {
            Ok(true)
        } else {
            let a2476 = {
                let a2474 = {
                    let a2468 = v0;
                    let a2469 = {
                        let a2466 = {
                            let a2464 = v1;
                            let a2465 = 1u64;
                            nat_sub(a2464, a2465)
                        };
                        let a2467 = v2;
                        nat_mul(a2466, a2467)?
                    };
                    nat_add(a2468, a2469)?
                };
                let a2475 = {
                    let a2472 = {
                        let a2470 = v3;
                        let a2471 = 1u64;
                        nat_sub(a2470, a2471)
                    };
                    let a2473 = v4;
                    nat_mul(a2472, a2473)?
                };
                nat_add(a2474, a2475)?
            };
            let a2477 = v6;
            Ok(nat_le(a2476, a2477))
        }
    } else {
        Ok(false)
    }
}

pub fn f117(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f151(v0, v1, v6, v7, v8) {
        let a2478 = v5;
        let a2479 = 26u64;
        nat_eq(a2478, a2479)
    } else {
        false
    } {
        let a2480 = v3;
        let a2481 = v7;
        nat_eq(a2480, a2481)
    } else {
        false
    } {
        let a2482 = v4;
        let a2483 = v8;
        nat_eq(a2482, a2483)
    } else {
        false
    } {
        let a2484 = v9;
        let a2485 = 1u64;
        nat_eq(a2484, a2485)
    } else {
        false
    } {
        let a2486 = v10;
        let a2487 = f152(v0);
        nat_eq(a2486, a2487)
    } else {
        false
    } {
        let a2488 = v11;
        let a2489 = v1;
        nat_eq(a2488, a2489)
    } else {
        false
    } {
        let a2490 = v12;
        let a2491 = v6;
        nat_eq(a2490, a2491)
    } else {
        false
    } {
        let a2492 = v13;
        let a2493 = v7;
        nat_eq(a2492, a2493)
    } else {
        false
    } {
        let a2494 = v14;
        let a2495 = v8;
        nat_eq(a2494, a2495)
    } else {
        false
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2496 = v0;
    let a2497 = 0u64;
    if if if nat_eq(a2496, a2497) {
        true
    } else {
        let a2498 = v1;
        let a2499 = 0u64;
        nat_eq(a2498, a2499)
    } {
        true
    } else {
        let a2500 = v2;
        let a2501 = 0u64;
        nat_eq(a2500, a2501)
    } {
        true
    } else {
        let a2502 = v3;
        let a2503 = 0u64;
        nat_eq(a2502, a2503)
    }
}

pub fn f119(v0: u64) -> u64 {
    let a2504 = v0;
    let a2505 = 2147483648u64;
    if nat_lt(a2504, a2505) {
        v0
    } else {
        0u64
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2506 = v0;
    let a2507 = 2147483648u64;
    if nat_lt(a2506, a2507) {
        0u64
    } else {
        let a2508 = 4294967296u64;
        let a2509 = v0;
        nat_sub(a2508, a2509)
    }
}

pub fn f121(v0: Adt3, v1: bool) -> R<u64> {
    f153(v0.clone(), {
        let a2511 = match v0.clone() {
            Adt3::C0(h2510, _, _, _, _, _, _, _) => {
                h2510
            }
        };
        let a2512 = 1u64;
        nat_sub(a2511, a2512)
    }, {
        let a2514 = match v0.clone() {
            Adt3::C0(_, h2513, _, _, _, _, _, _) => {
                h2513
            }
        };
        let a2515 = 1u64;
        nat_sub(a2514, a2515)
    }, {
        let a2517 = match v0.clone() {
            Adt3::C0(_, _, h2516, _, _, _, _, _) => {
                h2516
            }
        };
        let a2518 = 1u64;
        nat_sub(a2517, a2518)
    }, {
        let a2520 = match v0 {
            Adt3::C0(_, _, _, h2519, _, _, _, _) => {
                h2519
            }
        };
        let a2521 = 1u64;
        nat_sub(a2520, a2521)
    }, v1)
}

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2522 = v0;
    let a2523 = 0u64;
    if nat_eq(a2522, a2523) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2524 = v0;
            let a2525 = 1u64;
            nat_sub(a2524, a2525)
        };
        let v5: u64 = {
            let a2526 = v4;
            let a2527 = v1;
            let a2528 = 0u64;
            nat_quot(a2526, a2527, a2528)
        };
        let v6: u64 = {
            let a2536 = {
                let a2529 = v5;
                let a2530 = v2;
                nat_mul(a2529, a2530)?
            };
            let a2537 = {
                let a2534 = {
                    let a2531 = v4;
                    let a2532 = v1;
                    let a2533 = 0u64;
                    nat_rem(a2531, a2532, a2533)
                };
                let a2535 = v3;
                nat_mul(a2534, a2535)?
            };
            nat_add(a2536, a2537)?
        };
        let a2538 = v5;
        let a2539 = 0u64;
        if nat_eq(a2538, a2539) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2548 = {
                    let a2542 = {
                        let a2540 = v5;
                        let a2541 = 1u64;
                        nat_sub(a2540, a2541)
                    };
                    let a2543 = v2;
                    nat_mul(a2542, a2543)?
                };
                let a2549 = {
                    let a2546 = {
                        let a2544 = v1;
                        let a2545 = 1u64;
                        nat_sub(a2544, a2545)
                    };
                    let a2547 = v3;
                    nat_mul(a2546, a2547)?
                };
                nat_add(a2548, a2549)?
            };
            let a2550 = v6;
            let a2551 = v7;
            if nat_lt(a2550, a2551) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f123(v0: Adt3, v1: u64) -> bool {
    let a2552 = v1;
    let a2553 = 0u64;
    if !nat_eq(a2552, a2553) {
        true
    } else {
        let a2555 = match v0 {
            Adt3::C0(_, _, _, _, h2554, _, _, _) => {
                h2554
            }
        };
        let a2556 = 4u64;
        nat_eq(a2555, a2556)
    }
}

pub fn f124(v0: u64, v1: bool) -> R<u64> {
    let a2559 = {
        let a2557 = v0;
        let a2558 = 2u64;
        nat_mul(a2557, a2558)?
    };
    let a2560 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2559, a2560)
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2561 = f154(v0, v1);
    match m2561 {
        None => {
            if v5 {
                let m2562 = f155(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2562 {
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
                let a2563 = 4u64;
                let a2564 = v28;
                nat_le(a2563, a2564)
            };
            let a2565 = v2;
            let a2566 = 0u64;
            if if if if if nat_eq(a2565, a2566) {
                let a2567 = v4;
                let a2568 = 0u64;
                nat_eq(a2567, a2568)
            } else {
                false
            } {
                let a2569 = v6;
                let a2570 = v14;
                if nat_eq(a2569, a2570) {
                    let a2571 = v7;
                    let a2572 = v15;
                    nat_eq(a2571, a2572)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2573 = v8;
                let a2574 = v16;
                if nat_eq(a2573, a2574) {
                    let a2575 = v9;
                    let a2576 = v17;
                    nat_eq(a2575, a2576)
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
                        let a2577 = v3;
                        let a2578 = 0u64;
                        nat_eq(a2577, a2578)
                    } else {
                        false
                    } {
                        f71(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2583 = {
                        let a2579 = v6;
                        let a2580 = v7;
                        nat_mul(a2579, a2580)?
                    };
                    let a2584 = {
                        let a2581 = v8;
                        let a2582 = v9;
                        nat_mul(a2581, a2582)?
                    };
                    nat_mul(a2583, a2584)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64) -> Adt8 {
    let a2585 = v0;
    let a2586 = 0u64;
    if nat_eq(a2585, a2586) {
        Adt8::C0(0u64, v1)
    } else {
        let a2587 = v0;
        let a2588 = 1u64;
        if nat_eq(a2587, a2588) {
            Adt8::C1(v1)
        } else {
            let a2589 = v0;
            let a2590 = 2u64;
            if nat_eq(a2589, a2590) {
                Adt8::C2(v1)
            } else {
                let a2591 = v0;
                let a2592 = 3u64;
                if nat_eq(a2591, a2592) {
                    Adt8::C3(v1)
                } else {
                    let a2593 = v0;
                    let a2594 = 13u64;
                    if nat_lt(a2593, a2594) {
                        Adt8::C0({
                            let a2595 = v0;
                            let a2596 = 3u64;
                            nat_sub(a2595, a2596)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f127(v0: u64, v1: u64) -> Adt9 {
    let a2597 = v0;
    let a2598 = 1u64;
    if nat_eq(a2597, a2598) {
        Adt9::C0(v1)
    } else {
        let a2599 = v0;
        let a2600 = 3u64;
        if if nat_eq(a2599, a2600) {
            true
        } else {
            let a2601 = v0;
            let a2602 = 6u64;
            nat_eq(a2601, a2602)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f128(v0: u64) -> u64 {
    let m2603 = f70(v0);
    match m2603 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2605 = match v1.clone() {
                Adt11::C0(_, h2604, _) => {
                    h2604
                }
            };
            let a2606 = 1u64;
            if if nat_eq(a2605, a2606) {
                let a2608 = match v1.clone() {
                    Adt11::C0(_, _, h2607) => {
                        h2607
                    }
                };
                let a2609 = 1u64;
                if if if nat_eq(a2608, a2609) {
                    true
                } else {
                    let a2611 = match v1.clone() {
                        Adt11::C0(_, _, h2610) => {
                            h2610
                        }
                    };
                    let a2612 = 2u64;
                    nat_eq(a2611, a2612)
                } {
                    true
                } else {
                    let a2614 = match v1.clone() {
                        Adt11::C0(_, _, h2613) => {
                            h2613
                        }
                    };
                    let a2615 = 4u64;
                    nat_eq(a2614, a2615)
                } {
                    true
                } else {
                    let a2617 = match v1.clone() {
                        Adt11::C0(_, _, h2616) => {
                            h2616
                        }
                    };
                    let a2618 = 8u64;
                    nat_eq(a2617, a2618)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2619) => {
                        h2619
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2632 = {
        let a2628 = {
            let a2624 = {
                let a2620 = v0;
                let a2621 = v4;
                nat_mul(a2620, a2621)?
            };
            let a2625 = {
                let a2622 = v1;
                let a2623 = v5;
                nat_mul(a2622, a2623)?
            };
            nat_add(a2624, a2625)?
        };
        let a2629 = {
            let a2626 = v2;
            let a2627 = v6;
            nat_mul(a2626, a2627)?
        };
        nat_add(a2628, a2629)?
    };
    let a2633 = {
        let a2630 = v3;
        let a2631 = v7;
        nat_mul(a2630, a2631)?
    };
    nat_add(a2632, a2633)
}

pub fn f130(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2638 = {
        let a2636 = {
            let a2634 = v0;
            let a2635 = v1;
            nat_sub(a2634, a2635)
        };
        let a2637 = 1u64;
        nat_sub(a2636, a2637)
    };
    let a2639 = v2;
    nat_add(a2638, a2639)
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f129({
        let a2640 = v0;
        let a2641 = v4;
        let a2642 = 0u64;
        nat_rem(a2640, a2641, a2642)
    }, {
        let a2643 = v1;
        let a2644 = v5;
        let a2645 = 0u64;
        nat_rem(a2643, a2644, a2645)
    }, {
        let a2646 = v2;
        let a2647 = v6;
        let a2648 = 0u64;
        nat_rem(a2646, a2647, a2648)
    }, {
        let a2649 = v3;
        let a2650 = v7;
        let a2651 = 0u64;
        nat_rem(a2649, a2650, a2651)
    }, v8, v9, v10, v11)
}

pub fn f132(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2653 = v0;
        let a2654 = match v1.clone() {
            Adt3::C0(h2652, _, _, _, _, _, _, _) => {
                h2652
            }
        };
        let a2655 = 0u64;
        nat_quot(a2653, a2654, a2655)
    };
    let v4: u64 = {
        let a2657 = v3;
        let a2658 = match v1.clone() {
            Adt3::C0(_, h2656, _, _, _, _, _, _) => {
                h2656
            }
        };
        let a2659 = 0u64;
        nat_quot(a2657, a2658, a2659)
    };
    let a2660 = v2;
    let a2661 = 0u64;
    if nat_eq(a2660, a2661) {
        let a2663 = v0;
        let a2664 = match v1 {
            Adt3::C0(h2662, _, _, _, _, _, _, _) => {
                h2662
            }
        };
        let a2665 = 0u64;
        nat_rem(a2663, a2664, a2665)
    } else {
        let a2666 = v2;
        let a2667 = 1u64;
        if nat_eq(a2666, a2667) {
            let a2669 = v3;
            let a2670 = match v1 {
                Adt3::C0(_, h2668, _, _, _, _, _, _) => {
                    h2668
                }
            };
            let a2671 = 0u64;
            nat_rem(a2669, a2670, a2671)
        } else {
            let a2672 = v2;
            let a2673 = 2u64;
            if nat_eq(a2672, a2673) {
                let a2675 = v4;
                let a2676 = match v1 {
                    Adt3::C0(_, _, h2674, _, _, _, _, _) => {
                        h2674
                    }
                };
                let a2677 = 0u64;
                nat_rem(a2675, a2676, a2677)
            } else {
                let a2679 = v4;
                let a2680 = match v1 {
                    Adt3::C0(_, _, h2678, _, _, _, _, _) => {
                        h2678
                    }
                };
                let a2681 = 0u64;
                nat_quot(a2679, a2680, a2681)
            }
        }
    }
}

pub fn f133(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f129(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2682, _, _, _) => {
            h2682
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2683, _, _) => {
            h2683
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2684, _) => {
            h2684
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2685) => {
            h2685
        }
    })
}

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2686 = v1;
    let a2687 = 2147483648u64;
    if nat_lt(a2686, a2687) {
        let a2698 = {
            let a2696 = {
                let a2691 = {
                    let a2688 = v0;
                    let a2689 = v2;
                    let a2690 = 0u64;
                    nat_rem(a2688, a2689, a2690)
                };
                let a2692 = v2;
                nat_add(a2691, a2692)?
            };
            let a2697 = {
                let a2693 = v1;
                let a2694 = v2;
                let a2695 = 0u64;
                nat_rem(a2693, a2694, a2695)
            };
            nat_sub(a2696, a2697)
        };
        let a2699 = v2;
        let a2700 = 0u64;
        Ok(nat_rem(a2698, a2699, a2700))
    } else {
        let a2703 = {
            let a2701 = v0;
            let a2702 = f120(v1);
            nat_add(a2701, a2702)?
        };
        let a2704 = v2;
        let a2705 = 0u64;
        Ok(nat_rem(a2703, a2704, a2705))
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2708 = f119(v1);
    let a2709 = {
        let a2706 = v0;
        let a2707 = f120(v1);
        nat_add(a2706, a2707)?
    };
    if nat_le(a2708, a2709) {
        let a2710 = f136(v0, v1)?;
        let a2711 = v2;
        Ok(nat_lt(a2710, a2711))
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a2714 = {
        let a2712 = v0;
        let a2713 = f120(v1);
        nat_add(a2712, a2713)?
    };
    let a2715 = f119(v1);
    Ok(nat_sub(a2714, a2715))
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2716 = v0;
    let a2717 = v1;
    if nat_lt(a2716, a2717) {
        let a2718 = v1;
        let a2719 = v0;
        Ok(nat_sub(a2718, a2719))
    } else {
        let a2722 = v0;
        let a2723 = {
            let a2720 = v1;
            let a2721 = v2;
            nat_add(a2720, a2721)?
        };
        if nat_lt(a2722, a2723) {
            let a2724 = v0;
            let a2725 = v1;
            Ok(nat_sub(a2724, a2725))
        } else {
            let a2732 = {
                let a2728 = 2u64;
                let a2729 = {
                    let a2726 = v2;
                    let a2727 = 1u64;
                    nat_sub(a2726, a2727)
                };
                nat_mul(a2728, a2729)?
            };
            let a2733 = {
                let a2730 = v0;
                let a2731 = v1;
                nat_sub(a2730, a2731)
            };
            Ok(nat_sub(a2732, a2733))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2736 = {
        let a2734 = v2;
        let a2735 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1.clone(), 3u64), false)?;
        nat_add(a2734, a2735)?
    };
    let a2737 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2736, a2737))
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> bool {
    let a2738 = v0;
    let a2739 = 0u64;
    if nat_eq(a2738, a2739) {
        let a2740 = v2;
        let a2741 = v1;
        nat_le(a2740, a2741)
    } else {
        let a2742 = v0;
        let a2743 = 1u64;
        if nat_eq(a2742, a2743) {
            let a2744 = v2;
            let a2745 = v1;
            nat_lt(a2744, a2745)
        } else {
            let a2746 = v0;
            let a2747 = 2u64;
            if nat_eq(a2746, a2747) {
                let a2748 = v1;
                let a2749 = v2;
                nat_le(a2748, a2749)
            } else {
                let a2750 = v0;
                let a2751 = 3u64;
                if nat_eq(a2750, a2751) {
                    let a2752 = v1;
                    let a2753 = v2;
                    nat_lt(a2752, a2753)
                } else {
                    let a2754 = v1;
                    let a2755 = v2;
                    nat_eq(a2754, a2755)
                }
            }
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2756 = v0;
    let a2757 = 0u64;
    if nat_eq(a2756, a2757) {
        Ok(f148(0u64))
    } else {
        let a2760 = {
            let a2758 = v0;
            let a2759 = 1u64;
            nat_add(a2758, a2759)?
        };
        let a2761 = f82(v1.clone(), 13u64, v2)?;
        if nat_eq(a2760, a2761) {
            Ok(Adt5::C4({
                let a2762 = v2;
                let a2763 = 0u64;
                if nat_eq(a2762, a2763) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2764 = v0;
                let a2765 = 1u64;
                nat_sub(a2764, a2765)
            };
            if f123(v1.clone(), v2) {
                let v4: u64 = f124(match v1.clone() {
                    Adt3::C0(h2766, _, _, _, _, _, _, _) => {
                        h2766
                    }
                }, false)?;
                let v5: u64 = {
                    let a2767 = v3;
                    let a2768 = v4;
                    let a2769 = 0u64;
                    nat_quot(a2767, a2768, a2769)
                };
                let v6: u64 = {
                    let a2770 = v3;
                    let a2771 = v4;
                    let a2772 = 0u64;
                    nat_rem(a2770, a2771, a2772)
                };
                let a2775 = {
                    let a2773 = v6;
                    let a2774 = 1u64;
                    nat_add(a2773, a2774)?
                };
                let a2776 = v4;
                if nat_eq(a2775, a2776) {
                    Ok(Adt5::C4({
                        let a2777 = v2;
                        let a2778 = 0u64;
                        if nat_eq(a2777, a2778) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f141(v6, v5, v1, v2)
                }
            } else {
                f156({
                    let a2779 = v3;
                    let a2780 = 2u64;
                    let a2781 = 0u64;
                    nat_quot(a2779, a2780, a2781)
                }, {
                    let a2782 = v3;
                    let a2783 = 2u64;
                    let a2784 = 0u64;
                    nat_rem(a2782, a2783, a2784)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f141(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2785 = v0;
    let a2786 = 0u64;
    if nat_eq(a2785, a2786) {
        Ok(f148(1u64))
    } else {
        f156({
            let a2795 = {
                let a2788 = v1;
                let a2789 = match v2.clone() {
                    Adt3::C0(h2787, _, _, _, _, _, _, _) => {
                        h2787
                    }
                };
                nat_mul(a2788, a2789)?
            };
            let a2796 = {
                let a2792 = {
                    let a2790 = v0;
                    let a2791 = 1u64;
                    nat_sub(a2790, a2791)
                };
                let a2793 = 2u64;
                let a2794 = 0u64;
                nat_quot(a2792, a2793, a2794)
            };
            nat_add(a2795, a2796)?
        }, {
            let a2799 = {
                let a2797 = v0;
                let a2798 = 1u64;
                nat_sub(a2797, a2798)
            };
            let a2800 = 2u64;
            let a2801 = 0u64;
            nat_rem(a2799, a2800, a2801)
        }, v2, v3, 1u64)
    }
}

pub fn f142(v0: u64, v1: u64) -> R<u64> {
    let a2802 = v0;
    let a2803 = f60(v1)?;
    let a2804 = 0u64;
    Ok(nat_rem(a2802, a2803, a2804))
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2805 = 4u64;
    let a2806 = f157(v0, v1)?;
    nat_mul(a2805, a2806)
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2807 = 4u64;
    let a2808 = f158(v0, v1)?;
    nat_mul(a2807, a2808)
}

pub fn f145(v0: u64, v1: u64) -> R<bool> {
    let a2814 = {
        let a2811 = {
            let a2809 = f142(v0, v1)?;
            let a2810 = 2u64;
            nat_sub(a2809, a2810)
        };
        let a2812 = 2u64;
        let a2813 = 0u64;
        nat_rem(a2811, a2812, a2813)
    };
    let a2815 = 0u64;
    Ok(nat_eq(a2814, a2815))
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2816 = f70(v0);
    match m2816 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f159(v0) {
                let a2817 = v1;
                let a2818 = v7;
                nat_lt(a2817, a2818)
            } else {
                false
            } {
                f106(v8, v2)
            } else {
                false
            } {
                let a2819 = v3;
                let a2820 = f160(v4, v5, v6)?;
                nat_lt(a2819, a2820)
            } else {
                false
            } {
                let a2825 = {
                    let a2822 = v7;
                    let a2823 = match v13.clone() {
                        Adt11::C0(_, h2821, _) => {
                            h2821
                        }
                    };
                    let a2824 = 0u64;
                    nat_rem(a2822, a2823, a2824)
                };
                let a2826 = 0u64;
                nat_eq(a2825, a2826)
            } else {
                false
            } {
                Ok(Some({
                    let a2855 = {
                        let a2845 = {
                            let a2835 = {
                                let a2831 = {
                                    let a2828 = v1;
                                    let a2829 = match v13 {
                                        Adt11::C0(_, h2827, _) => {
                                            h2827
                                        }
                                    };
                                    let a2830 = 0u64;
                                    nat_quot(a2828, a2829, a2830)
                                };
                                let a2832 = v9;
                                nat_mul(a2831, a2832)?
                            };
                            let a2836 = {
                                let a2833 = v2;
                                let a2834 = v10;
                                nat_mul(a2833, a2834)?
                            };
                            nat_add(a2835, a2836)?
                        };
                        let a2846 = {
                            let a2843 = {
                                let a2840 = {
                                    let a2837 = v3;
                                    let a2838 = v4;
                                    let a2839 = 0u64;
                                    nat_quot(a2837, a2838, a2839)
                                };
                                let a2841 = v5;
                                let a2842 = 0u64;
                                nat_rem(a2840, a2841, a2842)
                            };
                            let a2844 = v11;
                            nat_mul(a2843, a2844)?
                        };
                        nat_add(a2845, a2846)?
                    };
                    let a2856 = {
                        let a2853 = {
                            let a2850 = {
                                let a2847 = v3;
                                let a2848 = v4;
                                let a2849 = 0u64;
                                nat_quot(a2847, a2848, a2849)
                            };
                            let a2851 = v5;
                            let a2852 = 0u64;
                            nat_quot(a2850, a2851, a2852)
                        };
                        let a2854 = v12;
                        nat_mul(a2853, a2854)?
                    };
                    nat_add(a2855, a2856)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f147(v0: u64) -> u64 {
    let a2857 = v0;
    let a2858 = 30u64;
    if nat_eq(a2857, a2858) {
        4u64
    } else {
        3u64
    }
}

pub fn f148(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f149(v0: u64) -> bool {
    let a2859 = 0u64;
    let a2860 = v0;
    if nat_lt(a2859, a2860) {
        let a2861 = v0;
        let a2862 = 2139095040u64;
        nat_le(a2861, a2862)
    } else {
        false
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2863 = v0;
    let a2864 = 40u64;
    if nat_lt(a2863, a2864) {
        let a2865 = v0;
        let a2866 = 20u64;
        if nat_lt(a2865, a2866) {
            let a2867 = v0;
            let a2868 = 10u64;
            if nat_lt(a2867, a2868) {
                let a2869 = v0;
                let a2870 = 5u64;
                if nat_lt(a2869, a2870) {
                    let a2871 = v0;
                    let a2872 = 2u64;
                    if nat_lt(a2871, a2872) {
                        let a2873 = v0;
                        let a2874 = 1u64;
                        if nat_lt(a2873, a2874) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2875 = v0;
                        let a2876 = 3u64;
                        if nat_lt(a2875, a2876) {
                            4u64
                        } else {
                            let a2877 = v0;
                            let a2878 = 4u64;
                            if nat_lt(a2877, a2878) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2879 = v0;
                    let a2880 = 7u64;
                    if nat_lt(a2879, a2880) {
                        let a2881 = v0;
                        let a2882 = 6u64;
                        if nat_lt(a2881, a2882) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2883 = v0;
                        let a2884 = 8u64;
                        if nat_lt(a2883, a2884) {
                            128u64
                        } else {
                            let a2885 = v0;
                            let a2886 = 9u64;
                            if nat_lt(a2885, a2886) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2887 = v0;
                let a2888 = 15u64;
                if nat_lt(a2887, a2888) {
                    let a2889 = v0;
                    let a2890 = 12u64;
                    if nat_lt(a2889, a2890) {
                        let a2891 = v0;
                        let a2892 = 11u64;
                        if nat_lt(a2891, a2892) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2893 = v0;
                        let a2894 = 13u64;
                        if nat_lt(a2893, a2894) {
                            4096u64
                        } else {
                            let a2895 = v0;
                            let a2896 = 14u64;
                            if nat_lt(a2895, a2896) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2897 = v0;
                    let a2898 = 17u64;
                    if nat_lt(a2897, a2898) {
                        let a2899 = v0;
                        let a2900 = 16u64;
                        if nat_lt(a2899, a2900) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2901 = v0;
                        let a2902 = 18u64;
                        if nat_lt(a2901, a2902) {
                            131072u64
                        } else {
                            let a2903 = v0;
                            let a2904 = 19u64;
                            if nat_lt(a2903, a2904) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2905 = v0;
            let a2906 = 30u64;
            if nat_lt(a2905, a2906) {
                let a2907 = v0;
                let a2908 = 25u64;
                if nat_lt(a2907, a2908) {
                    let a2909 = v0;
                    let a2910 = 22u64;
                    if nat_lt(a2909, a2910) {
                        let a2911 = v0;
                        let a2912 = 21u64;
                        if nat_lt(a2911, a2912) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2913 = v0;
                        let a2914 = 23u64;
                        if nat_lt(a2913, a2914) {
                            4194304u64
                        } else {
                            let a2915 = v0;
                            let a2916 = 24u64;
                            if nat_lt(a2915, a2916) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2917 = v0;
                    let a2918 = 27u64;
                    if nat_lt(a2917, a2918) {
                        let a2919 = v0;
                        let a2920 = 26u64;
                        if nat_lt(a2919, a2920) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2921 = v0;
                        let a2922 = 28u64;
                        if nat_lt(a2921, a2922) {
                            134217728u64
                        } else {
                            let a2923 = v0;
                            let a2924 = 29u64;
                            if nat_lt(a2923, a2924) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2925 = v0;
                let a2926 = 35u64;
                if nat_lt(a2925, a2926) {
                    let a2927 = v0;
                    let a2928 = 32u64;
                    if nat_lt(a2927, a2928) {
                        let a2929 = v0;
                        let a2930 = 31u64;
                        if nat_lt(a2929, a2930) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2931 = v0;
                        let a2932 = 33u64;
                        if nat_lt(a2931, a2932) {
                            4294967296u64
                        } else {
                            let a2933 = v0;
                            let a2934 = 34u64;
                            if nat_lt(a2933, a2934) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2935 = v0;
                    let a2936 = 37u64;
                    if nat_lt(a2935, a2936) {
                        let a2937 = v0;
                        let a2938 = 36u64;
                        if nat_lt(a2937, a2938) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2939 = v0;
                        let a2940 = 38u64;
                        if nat_lt(a2939, a2940) {
                            137438953472u64
                        } else {
                            let a2941 = v0;
                            let a2942 = 39u64;
                            if nat_lt(a2941, a2942) {
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

pub fn f151(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f118(v1, v2, v3, v4) {
        f161(v0)
    } else {
        f159(v0)
    }
}

pub fn f152(v0: u64) -> u64 {
    let a2943 = v0;
    let a2944 = 26u64;
    if nat_eq(a2943, a2944) {
        26u64
    } else {
        0u64
    }
}

pub fn f153(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2965 = {
        let a2959 = {
            let a2953 = {
                let a2947 = v1;
                let a2948 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2945, _, _, _) => {
                            h2945
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2946, _, _, _) => {
                            h2946
                        }
                    })
                };
                nat_mul(a2947, a2948)?
            };
            let a2954 = {
                let a2951 = v2;
                let a2952 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2949, _, _) => {
                            h2949
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2950, _, _) => {
                            h2950
                        }
                    })
                };
                nat_mul(a2951, a2952)?
            };
            nat_add(a2953, a2954)?
        };
        let a2960 = {
            let a2957 = v3;
            let a2958 = if v5 {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2955, _) => {
                        h2955
                    }
                })
            } else {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2956, _) => {
                        h2956
                    }
                })
            };
            nat_mul(a2957, a2958)?
        };
        nat_add(a2959, a2960)?
    };
    let a2966 = {
        let a2963 = v4;
        let a2964 = if v5 {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2961) => {
                    h2961
                }
            })
        } else {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2962) => {
                    h2962
                }
            })
        };
        nat_mul(a2963, a2964)?
    };
    nat_add(a2965, a2966)
}

pub fn f154(v0: u64, v1: u64) -> Option<u64> {
    let a2967 = v0;
    let a2968 = 2u64;
    if nat_eq(a2967, a2968) {
        Some(0u64)
    } else {
        let a2969 = v0;
        let a2970 = 5u64;
        if nat_eq(a2969, a2970) {
            Some(1u64)
        } else {
            let a2971 = v0;
            let a2972 = 6u64;
            if nat_eq(a2971, a2972) {
                Some(2u64)
            } else {
                let a2973 = v0;
                let a2974 = 7u64;
                if nat_eq(a2973, a2974) {
                    Some(3u64)
                } else {
                    let a2975 = v0;
                    let a2976 = 9u64;
                    if nat_eq(a2975, a2976) {
                        Some(4u64)
                    } else {
                        let a2977 = v0;
                        let a2978 = 8u64;
                        if nat_eq(a2977, a2978) {
                            Some(5u64)
                        } else {
                            let a2979 = v0;
                            let a2980 = 89u64;
                            if nat_eq(a2979, a2980) {
                                let a2981 = v1;
                                let a2982 = 2u64;
                                if nat_eq(a2981, a2982) {
                                    Some(6u64)
                                } else {
                                    let a2983 = v1;
                                    let a2984 = 0u64;
                                    if nat_eq(a2983, a2984) {
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

pub fn f155(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a2985 = v0;
    let a2986 = 28u64;
    if if if if nat_eq(a2985, a2986) {
        let a2987 = v1;
        let a2988 = 0u64;
        nat_eq(a2987, a2988)
    } else {
        false
    } {
        let a2989 = v2;
        let a2990 = 0u64;
        nat_eq(a2989, a2990)
    } else {
        false
    } {
        let a2991 = v3;
        let a2992 = 0u64;
        nat_eq(a2991, a2992)
    } else {
        false
    } {
        let a2993 = v4;
        let a2994 = v8;
        if if if if if if if if nat_eq(a2993, a2994) {
            let a2995 = v5;
            let a2996 = v12;
            nat_eq(a2995, a2996)
        } else {
            false
        } {
            let a2997 = v9;
            let a2998 = v13;
            nat_eq(a2997, a2998)
        } else {
            false
        } {
            let a2999 = v10;
            let a3000 = v14;
            nat_eq(a2999, a3000)
        } else {
            false
        } {
            let a3001 = v11;
            let a3002 = v15;
            nat_eq(a3001, a3002)
        } else {
            false
        } {
            let a3003 = 0u64;
            let a3004 = v6;
            nat_lt(a3003, a3004)
        } else {
            false
        } {
            let a3005 = 0u64;
            let a3006 = v7;
            nat_lt(a3005, a3006)
        } else {
            false
        } {
            let a3010 = {
                let a3007 = v10;
                let a3008 = v6;
                let a3009 = 1u64;
                nat_rem(a3007, a3008, a3009)
            };
            let a3011 = 0u64;
            if nat_eq(a3010, a3011) {
                let a3015 = {
                    let a3012 = v11;
                    let a3013 = v7;
                    let a3014 = 1u64;
                    nat_rem(a3012, a3013, a3014)
                };
                let a3016 = 0u64;
                nat_eq(a3015, a3016)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3017 = v10;
                let a3018 = v6;
                let a3019 = 0u64;
                nat_quot(a3017, a3018, a3019)
            }, {
                let a3020 = v11;
                let a3021 = v7;
                let a3022 = 0u64;
                nat_quot(a3020, a3021, a3022)
            }, {
                let a3023 = v5;
                let a3024 = v4;
                nat_mul(a3023, a3024)?
            }, {
                let a3025 = v4;
                let a3026 = v9;
                nat_mul(a3025, a3026)?
            }, {
                let a3027 = v5;
                let a3028 = v9;
                nat_mul(a3027, a3028)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f156(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3029 = v1;
    let a3030 = 0u64;
    if nat_eq(a3029, a3030) {
        Ok(Adt5::C4({
            let a3031 = v3;
            let a3032 = 0u64;
            if nat_eq(a3031, a3032) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f89(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3033 = v3;
            let a3034 = 0u64;
            if nat_eq(a3033, a3034) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    let a3035 = v0;
    let a3036 = f60(v1)?;
    let a3037 = 0u64;
    Ok(nat_quot(a3035, a3036, a3037))
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f157(v0, v1)?;
    let v3: u64 = {
        let a3040 = {
            let a3038 = f142(v0, v1)?;
            let a3039 = 2u64;
            nat_sub(a3038, a3039)
        };
        let a3041 = 2u64;
        let a3042 = 0u64;
        nat_quot(a3040, a3041, a3042)
    };
    let v4: u64 = {
        let a3043 = v2;
        let a3044 = v1;
        nat_mul(a3043, a3044)?
    };
    if f145(v0, v1)? {
        let a3045 = v4;
        let a3046 = v3;
        nat_add(a3045, a3046)
    } else {
        let a3058 = {
            let a3054 = v4;
            let a3055 = {
                let a3052 = {
                    let a3047 = v1;
                    let a3048 = 1u64;
                    nat_sub(a3047, a3048)
                };
                let a3053 = {
                    let a3049 = v2;
                    let a3050 = v1;
                    let a3051 = 0u64;
                    nat_rem(a3049, a3050, a3051)
                };
                nat_mul(a3052, a3053)?
            };
            nat_sub(a3054, a3055)
        };
        let a3059 = {
            let a3056 = v3;
            let a3057 = v1;
            nat_mul(a3056, a3057)?
        };
        nat_add(a3058, a3059)
    }
}

pub fn f159(v0: u64) -> bool {
    let a3060 = v0;
    let a3061 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3060, a3061) {
        true
    } else {
        let a3062 = v0;
        let a3063 = 1u64;
        nat_eq(a3062, a3063)
    } {
        true
    } else {
        let a3064 = v0;
        let a3065 = 2u64;
        nat_eq(a3064, a3065)
    } {
        true
    } else {
        let a3066 = v0;
        let a3067 = 3u64;
        nat_eq(a3066, a3067)
    } {
        true
    } else {
        let a3068 = v0;
        let a3069 = 6u64;
        nat_eq(a3068, a3069)
    } {
        true
    } else {
        let a3070 = v0;
        let a3071 = 7u64;
        nat_eq(a3070, a3071)
    } {
        true
    } else {
        let a3072 = v0;
        let a3073 = 8u64;
        nat_eq(a3072, a3073)
    } {
        true
    } else {
        let a3074 = v0;
        let a3075 = 10u64;
        nat_eq(a3074, a3075)
    } {
        true
    } else {
        let a3076 = v0;
        let a3077 = 11u64;
        nat_eq(a3076, a3077)
    } {
        true
    } else {
        let a3078 = v0;
        let a3079 = 12u64;
        nat_eq(a3078, a3079)
    } {
        true
    } else {
        let a3080 = v0;
        let a3081 = 13u64;
        nat_eq(a3080, a3081)
    } {
        true
    } else {
        let a3082 = v0;
        let a3083 = 14u64;
        nat_eq(a3082, a3083)
    } {
        true
    } else {
        let a3084 = v0;
        let a3085 = 16u64;
        nat_eq(a3084, a3085)
    } {
        true
    } else {
        let a3086 = v0;
        let a3087 = 17u64;
        nat_eq(a3086, a3087)
    } {
        true
    } else {
        let a3088 = v0;
        let a3089 = 18u64;
        nat_eq(a3088, a3089)
    } {
        true
    } else {
        let a3090 = v0;
        let a3091 = 19u64;
        nat_eq(a3090, a3091)
    } {
        true
    } else {
        let a3092 = v0;
        let a3093 = 20u64;
        nat_eq(a3092, a3093)
    } {
        true
    } else {
        let a3094 = v0;
        let a3095 = 21u64;
        nat_eq(a3094, a3095)
    } {
        true
    } else {
        let a3096 = v0;
        let a3097 = 22u64;
        nat_eq(a3096, a3097)
    } {
        true
    } else {
        let a3098 = v0;
        let a3099 = 23u64;
        nat_eq(a3098, a3099)
    } {
        true
    } else {
        let a3100 = v0;
        let a3101 = 26u64;
        nat_eq(a3100, a3101)
    } {
        true
    } else {
        let a3102 = v0;
        let a3103 = 29u64;
        nat_eq(a3102, a3103)
    } {
        true
    } else {
        let a3104 = v0;
        let a3105 = 30u64;
        nat_eq(a3104, a3105)
    } {
        true
    } else {
        let a3106 = v0;
        let a3107 = 34u64;
        nat_eq(a3106, a3107)
    } {
        true
    } else {
        let a3108 = v0;
        let a3109 = 35u64;
        nat_eq(a3108, a3109)
    } {
        true
    } else {
        let a3110 = v0;
        let a3111 = 39u64;
        nat_eq(a3110, a3111)
    } {
        true
    } else {
        let a3112 = v0;
        let a3113 = 40u64;
        nat_eq(a3112, a3113)
    } {
        true
    } else {
        let a3114 = v0;
        let a3115 = 41u64;
        nat_eq(a3114, a3115)
    } {
        true
    } else {
        let a3116 = v0;
        let a3117 = 42u64;
        nat_eq(a3116, a3117)
    } {
        true
    } else {
        let a3118 = v0;
        let a3119 = 43u64;
        nat_eq(a3118, a3119)
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3120 = v0;
    let a3121 = 0u64;
    if if if nat_eq(a3120, a3121) {
        true
    } else {
        let a3122 = v1;
        let a3123 = 0u64;
        nat_eq(a3122, a3123)
    } {
        true
    } else {
        let a3124 = v2;
        let a3125 = 0u64;
        nat_eq(a3124, a3125)
    } {
        Ok(0u64)
    } else {
        let a3128 = {
            let a3126 = v0;
            let a3127 = v1;
            nat_mul(a3126, a3127)?
        };
        let a3129 = v2;
        nat_mul(a3128, a3129)
    }
}

pub fn f161(v0: u64) -> bool {
    let m3130 = f70(v0);
    match m3130 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
