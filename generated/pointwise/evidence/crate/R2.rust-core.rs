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
    C0(u64, Adt1, Option<Adt2>, Option<Adt2>, Option<Adt2>, Adt2),
    C1(u64, Adt3, Adt3, Adt4, u64),
    C2(Adt4),
    C3(bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
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
    C27(Adt5),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6, Adt6, Adt6, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt8),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
    C6(u64, u64, u64, bool),
    C7(Adt9, Adt10, Adt10, Adt11, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
    C5(u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64),
    C1(u64),
    C2(u64, u64, bool),
    C3(u64, u64, bool),
    C4(u64, u64, u64, u64),
    C5(u64),
    C6,
    C7(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64),
    C1(u64),
    C2,
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(Adt13),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt7> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6) => {
            let a2 = v1;
            let a3 = 64u64;
            if nat_eq(a2, a3) {
                Ok(Adt7::C0(f1(v2, v6)?))
            } else {
                let m4 = v3;
                match m4 {
                    None => {
                        Ok(Adt7::C0(Adt8::C1))
                    }
                    Some(v7) => {
                        let a5 = v1;
                        let a6 = 78u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt7::C0(f2(v2, v7, v4, v5, v6)?))
                        } else {
                            let a7 = v1;
                            let a8 = 23u64;
                            if if if nat_eq(a7, a8) {
                                true
                            } else {
                                let a9 = v1;
                                let a10 = 26u64;
                                nat_eq(a9, a10)
                            } {
                                true
                            } else {
                                let a11 = v1;
                                let a12 = 25u64;
                                nat_eq(a11, a12)
                            } {
                                Ok(Adt7::C0(Adt8::C5(64u64)))
                            } else {
                                let a13 = v1;
                                let a14 = 24u64;
                                if nat_eq(a13, a14) {
                                    Ok(Adt7::C0(f3(v2, v7, v6)?))
                                } else {
                                    let a15 = v1;
                                    let a16 = 27u64;
                                    if nat_eq(a15, a16) {
                                        Ok(Adt7::C0(f4(v2, v7, v6)?))
                                    } else {
                                        let a17 = v1;
                                        let a18 = 3u64;
                                        if nat_eq(a17, a18) {
                                            Ok(Adt7::C0(f5(v7, v4, v5, v6)?))
                                        } else {
                                            f6(v1, v2, v7, v4, v6)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v8, v9, v10, v11, v12) => {
            f7(v8, v9, v10, v11, v12)
        }
        Adt0::C2(v13) => {
            f8(v13)
        }
        Adt0::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26) => {
            Ok(f9(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26))
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt8> {
    let v2: u64 = f10(v1.clone(), 1u64)?;
    let a20 = match v1.clone() {
        Adt2::C0(h19, _, _, _, _, _, _, _, _, _, _) => {
            h19
        }
    };
    let a21 = 0u64;
    if if if if if if if if nat_eq(a20, a21) {
        let a23 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h22, _, _, _, _, _) => {
                h22
            }
        };
        let a24 = 4u64;
        nat_eq(a23, a24)
    } else {
        false
    } {
        f11(match v0.clone() {
            Adt1::C0(h25, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        })
    } else {
        false
    } {
        f11(match v0.clone() {
            Adt1::C0(_, h26, _, _, _, _, _, _, _, _) => {
                h26
            }
        })
    } else {
        false
    } {
        let a28 = 0u64;
        let a29 = match v0.clone() {
            Adt1::C0(_, _, h27, _, _, _, _, _, _, _) => {
                h27
            }
        };
        nat_lt(a28, a29)
    } else {
        false
    } {
        let a31 = match v0.clone() {
            Adt1::C0(_, _, h30, _, _, _, _, _, _, _) => {
                h30
            }
        };
        let a32 = 2139095040u64;
        nat_lt(a31, a32)
    } else {
        false
    } {
        let a33 = v2;
        let a34 = 9223372036854775808u64;
        nat_lt(a33, a34)
    } else {
        false
    } {
        let a36 = f12(v2)?;
        let a37 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h35, _) => {
                h35
            }
        };
        nat_le(a36, a37)
    } else {
        false
    } {
        Ok(Adt8::C4(f13(v2)?, 4u64, f14(v1.clone(), 1u64), f14(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h38, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        }, match v0.clone() {
            Adt1::C0(_, h39, _, _, _, _, _, _, _, _) => {
                h39
            }
        }, match v0 {
            Adt1::C0(_, _, h40, _, _, _, _, _, _, _) => {
                h40
            }
        }, v2)))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let m41 = v2;
    match m41 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v5) => {
            let m42 = v3;
            match m42 {
                None => {
                    Ok(Adt8::C1)
                }
                Some(v6) => {
                    let a44 = match v1.clone() {
                        Adt2::C0(h43, _, _, _, _, _, _, _, _, _, _) => {
                            h43
                        }
                    };
                    let a45 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a44, a45) {
                        let a47 = match v5.clone() {
                            Adt2::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                                h46
                            }
                        };
                        let a48 = 0u64;
                        nat_eq(a47, a48)
                    } else {
                        false
                    } {
                        let a50 = match v6.clone() {
                            Adt2::C0(h49, _, _, _, _, _, _, _, _, _, _) => {
                                h49
                            }
                        };
                        let a51 = 0u64;
                        nat_eq(a50, a51)
                    } else {
                        false
                    } {
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
                        f15(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f15(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f16(v1.clone())?
                    } else {
                        false
                    } {
                        f16(v5.clone())?
                    } else {
                        false
                    } {
                        f16(v6)?
                    } else {
                        false
                    } {
                        f16(v4.clone())?
                    } else {
                        false
                    } {
                        let a60 = match v1.clone() {
                            Adt2::C0(_, h55, _, _, _, _, _, _, _, _, _) => {
                                h55
                            }
                        };
                        let a61 = {
                            let a58 = match v5.clone() {
                                Adt2::C0(_, h56, _, _, _, _, _, _, _, _, _) => {
                                    h56
                                }
                            };
                            let a59 = match v5.clone() {
                                Adt2::C0(_, h57, _, _, _, _, _, _, _, _, _) => {
                                    h57
                                }
                            };
                            nat_mul(a58, a59)?
                        };
                        nat_eq(a60, a61)
                    } else {
                        false
                    } {
                        let a67 = match v1.clone() {
                            Adt2::C0(_, _, h62, _, _, _, _, _, _, _, _) => {
                                h62
                            }
                        };
                        let a68 = {
                            let a65 = match v5.clone() {
                                Adt2::C0(_, _, h63, _, _, _, _, _, _, _, _) => {
                                    h63
                                }
                            };
                            let a66 = match v5.clone() {
                                Adt2::C0(_, _, _, h64, _, _, _, _, _, _, _) => {
                                    h64
                                }
                            };
                            nat_mul(a65, a66)?
                        };
                        nat_eq(a67, a68)
                    } else {
                        false
                    } {
                        let a71 = match v1.clone() {
                            Adt2::C0(_, _, _, h69, _, _, _, _, _, _, _) => {
                                h69
                            }
                        };
                        let a72 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h70, _, _, _, _, _, _) => {
                                h70
                            }
                        };
                        nat_eq(a71, a72)
                    } else {
                        false
                    } {
                        let a74 = match v0.clone() {
                            Adt1::C0(h73, _, _, _, _, _, _, _, _, _) => {
                                h73
                            }
                        };
                        let a75 = 4294967296u64;
                        nat_lt(a74, a75)
                    } else {
                        false
                    } {
                        let a77 = match v0.clone() {
                            Adt1::C0(h76, _, _, _, _, _, _, _, _, _) => {
                                h76
                            }
                        };
                        let a78 = 0u64;
                        if nat_eq(a77, a78) {
                            if f17(v1.clone(), v4.clone())? {
                                true
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
                            let a85 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h83) => {
                                    h83
                                }
                            };
                            let a86 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h84) => {
                                    h84
                                }
                            };
                            nat_eq(a85, a86)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a88 = match v4.clone() {
                                Adt2::C0(_, h87, _, _, _, _, _, _, _, _, _) => {
                                    h87
                                }
                            };
                            let a89 = 0u64;
                            if if if nat_eq(a88, a89) {
                                true
                            } else {
                                let a91 = match v4.clone() {
                                    Adt2::C0(_, _, h90, _, _, _, _, _, _, _, _) => {
                                        h90
                                    }
                                };
                                let a92 = 0u64;
                                nat_eq(a91, a92)
                            } {
                                true
                            } else {
                                let a94 = match v4.clone() {
                                    Adt2::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
                                        h93
                                    }
                                };
                                let a95 = 0u64;
                                nat_eq(a94, a95)
                            } {
                                true
                            } else {
                                let a97 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h96, _, _, _, _, _, _) => {
                                        h96
                                    }
                                };
                                let a98 = 0u64;
                                nat_eq(a97, a98)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                                h99
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a101 = match v0 {
                                Adt1::C0(h100, _, _, _, _, _, _, _, _, _) => {
                                    h100
                                }
                            };
                            let a102 = 0u64;
                            if nat_eq(a101, a102) {
                                f10(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f18(v8, match v5.clone() {
                                Adt2::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                                    h103
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                                    h104
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h105, _, _, _, _, _, _) => {
                                    h105
                                }
                            })?
                        };
                        Ok(Adt8::C4(f19(v9, v8, v10)?, 4u64, f14(v1, 1u64), f14(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt8::C1)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h106, _, _, _, _, _, _, _, _, _, _) => {
            h106
        }
    };
    let v4: u64 = f20(v3);
    let a108 = match v1.clone() {
        Adt2::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
            h107
        }
    };
    let a109 = 0u64;
    if if if if if if if if nat_eq(a108, a109) {
        true
    } else {
        let a111 = match v1.clone() {
            Adt2::C0(h110, _, _, _, _, _, _, _, _, _, _) => {
                h110
            }
        };
        let a112 = 30u64;
        nat_eq(a111, a112)
    } {
        let a115 = match v1.clone() {
            Adt2::C0(h113, _, _, _, _, _, _, _, _, _, _) => {
                h113
            }
        };
        let a116 = match v2.clone() {
            Adt2::C0(h114, _, _, _, _, _, _, _, _, _, _) => {
                h114
            }
        };
        nat_eq(a115, a116)
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        f21(match v0.clone() {
            Adt1::C0(h117, _, _, _, _, _, _, _, _, _) => {
                h117
            }
        })
    } else {
        false
    } {
        f22(v1.clone(), v4)
    } else {
        false
    } {
        f22(v2.clone(), v4)
    } else {
        false
    } {
        if f17(v1.clone(), v2.clone())? {
            true
        } else {
            f23(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f24(match v1.clone() {
            Adt2::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h119, _, _, _, _, _, _, _, _) => {
                h119
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h120, _, _, _, _, _, _, _) => {
                h120
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h121, _, _, _, _, _, _) => {
                h121
            }
        }) {
            0u64
        } else {
            let a127 = match v1.clone() {
                Adt2::C0(_, _, h122, _, _, _, _, _, _, _, _) => {
                    h122
                }
            };
            let a128 = {
                let a125 = match v1.clone() {
                    Adt2::C0(_, _, _, h123, _, _, _, _, _, _, _) => {
                        h123
                    }
                };
                let a126 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                        h124
                    }
                };
                nat_mul(a125, a126)?
            };
            nat_mul(a127, a128)?
        };
        let v6: u64 = {
            let a129 = v5;
            let a130 = 0u64;
            if if nat_eq(a129, a130) {
                true
            } else {
                f25(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a132 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h131, _) => {
                        h131
                    }
                };
                let a133 = v4;
                let a134 = 0u64;
                nat_quot(a132, a133, a134)
            }
        };
        Ok(Adt8::C4(f26(v3, match v1.clone() {
            Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }, v5, v6)?, v4, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h136, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        }, v5, f27(match v1 {
            Adt2::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let a139 = match v1.clone() {
        Adt2::C0(h138, _, _, _, _, _, _, _, _, _, _) => {
            h138
        }
    };
    let a140 = 0u64;
    if if if if if nat_eq(a139, a140) {
        let a142 = match v2.clone() {
            Adt2::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                h141
            }
        };
        let a143 = 0u64;
        nat_eq(a142, a143)
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        f28(match v0.clone() {
            Adt1::C0(h144, _, _, _, _, _, _, _, _, _) => {
                h144
            }
        })
    } else {
        false
    } {
        if f17(v1.clone(), v2.clone())? {
            true
        } else {
            f23(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f14(v1, 1u64);
        let v4: u64 = {
            let a150 = match v3.clone() {
                Adt3::C0(_, h145, _, _, _, _, _, _) => {
                    h145
                }
            };
            let a151 = {
                let a148 = match v3.clone() {
                    Adt3::C0(_, _, h146, _, _, _, _, _) => {
                        h146
                    }
                };
                let a149 = match v3.clone() {
                    Adt3::C0(_, _, _, h147, _, _, _, _) => {
                        h147
                    }
                };
                nat_mul(a148, a149)?
            };
            nat_mul(a150, a151)?
        };
        Ok(Adt8::C4(f29(match v3.clone() {
            Adt3::C0(h152, _, _, _, _, _, _, _) => {
                h152
            }
        }, v4)?, 4u64, v3, f14(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h153, _, _, _, _, _, _, _, _, _) => {
                h153
            }
        })))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f5(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt8> {
    let m154 = v1;
    match m154 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v4) => {
            let m155 = v2;
            match m155 {
                None => {
                    Ok(Adt8::C1)
                }
                Some(v5) => {
                    let a157 = match v0.clone() {
                        Adt2::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                            h156
                        }
                    };
                    let a158 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a157, a158) {
                        let a160 = match v4.clone() {
                            Adt2::C0(h159, _, _, _, _, _, _, _, _, _, _) => {
                                h159
                            }
                        };
                        let a161 = 0u64;
                        nat_eq(a160, a161)
                    } else {
                        false
                    } {
                        let a163 = match v5.clone() {
                            Adt2::C0(h162, _, _, _, _, _, _, _, _, _, _) => {
                                h162
                            }
                        };
                        let a164 = 26u64;
                        nat_eq(a163, a164)
                    } else {
                        false
                    } {
                        let a166 = match v3.clone() {
                            Adt2::C0(h165, _, _, _, _, _, _, _, _, _, _) => {
                                h165
                            }
                        };
                        let a167 = 0u64;
                        nat_eq(a166, a167)
                    } else {
                        false
                    } {
                        f15(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a170 = match v0.clone() {
                            Adt2::C0(_, h168, _, _, _, _, _, _, _, _, _) => {
                                h168
                            }
                        };
                        let a171 = match v4.clone() {
                            Adt2::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
                                h169
                            }
                        };
                        nat_eq(a170, a171)
                    } else {
                        false
                    } {
                        let a174 = match v0.clone() {
                            Adt2::C0(_, _, h172, _, _, _, _, _, _, _, _) => {
                                h172
                            }
                        };
                        let a175 = match v5.clone() {
                            Adt2::C0(_, h173, _, _, _, _, _, _, _, _, _) => {
                                h173
                            }
                        };
                        nat_eq(a174, a175)
                    } else {
                        false
                    } {
                        let a178 = match v0.clone() {
                            Adt2::C0(_, _, _, h176, _, _, _, _, _, _, _) => {
                                h176
                            }
                        };
                        let a179 = match v5.clone() {
                            Adt2::C0(_, _, h177, _, _, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        nat_eq(a178, a179)
                    } else {
                        false
                    } {
                        let a181 = 0u64;
                        let a182 = match v5.clone() {
                            Adt2::C0(_, _, _, h180, _, _, _, _, _, _, _) => {
                                h180
                            }
                        };
                        nat_lt(a181, a182)
                    } else {
                        false
                    } {
                        let a184 = 0u64;
                        let a185 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h183, _, _, _, _, _, _) => {
                                h183
                            }
                        };
                        nat_lt(a184, a185)
                    } else {
                        false
                    } {
                        let a187 = 4u64;
                        let a188 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h186, _, _, _, _, _) => {
                                h186
                            }
                        };
                        nat_le(a187, a188)
                    } else {
                        false
                    } {
                        let a190 = 4u64;
                        let a191 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h189, _, _, _, _, _) => {
                                h189
                            }
                        };
                        nat_le(a190, a191)
                    } else {
                        false
                    } {
                        let a193 = 4u64;
                        let a194 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h192, _, _, _, _, _) => {
                                h192
                            }
                        };
                        nat_le(a193, a194)
                    } else {
                        false
                    } {
                        let a196 = 4u64;
                        let a197 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h195, _, _, _, _, _) => {
                                h195
                            }
                        };
                        nat_le(a196, a197)
                    } else {
                        false
                    } {
                        f17(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f17(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f17(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt8::C4(f30(match v0.clone() {
                            Adt2::C0(_, h198, _, _, _, _, _, _, _, _, _) => {
                                h198
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h199, _, _, _, _, _, _, _, _) => {
                                h199
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h200, _, _, _, _, _, _, _) => {
                                h200
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h201, _, _, _, _, _, _) => {
                                h201
                            }
                        })?, 4u64, f14(v0, 1u64), f14(v3, 1u64), Adt4::C22(f14(v4, 1u64), f14(v5, 1u64))))
                    } else {
                        Ok(Adt8::C1)
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt7> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h202, _, _, _, _, _, _, _, _, _) => {
            h202
        }
    };
    Ok(Adt7::C0({
        let a203 = v0;
        let a204 = 49u64;
        if nat_eq(a203, a204) {
            f31(v1, v2, v3, v4)?
        } else {
            let a205 = v0;
            let a206 = 51u64;
            if nat_eq(a205, a206) {
                f32(v1, v2, v3, v4)?
            } else {
                let a207 = v0;
                let a208 = 39u64;
                if nat_eq(a207, a208) {
                    f33(v2, v3, v4)?
                } else {
                    let a209 = v0;
                    let a210 = 77u64;
                    if nat_eq(a209, a210) {
                        f34(v2, v4)?
                    } else {
                        let a211 = v0;
                        let a212 = 40u64;
                        if nat_eq(a211, a212) {
                            f35(v2, v3, v4)?
                        } else {
                            if f36(v0) {
                                f37(v0, v2, {
                                    let m213 = v3;
                                    match m213 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }, v4)?
                            } else {
                                let a214 = v0;
                                let a215 = 19u64;
                                if nat_eq(a214, a215) {
                                    f38(v2, v4)?
                                } else {
                                    let a216 = v0;
                                    let a217 = 21u64;
                                    if nat_eq(a216, a217) {
                                        f39(v5, v2, v3, v4)?
                                    } else {
                                        let a218 = v0;
                                        let a219 = 61u64;
                                        if if if nat_eq(a218, a219) {
                                            true
                                        } else {
                                            let a220 = v0;
                                            let a221 = 62u64;
                                            nat_eq(a220, a221)
                                        } {
                                            true
                                        } else {
                                            let a222 = v0;
                                            let a223 = 63u64;
                                            nat_eq(a222, a223)
                                        } {
                                            f40(v0, v1, v2, v4)?
                                        } else {
                                            let a224 = v0;
                                            let a225 = 4u64;
                                            if if nat_eq(a224, a225) {
                                                true
                                            } else {
                                                let a226 = v0;
                                                let a227 = 32u64;
                                                nat_eq(a226, a227)
                                            } {
                                                f41(v0, v1, v2, v3, v4)?
                                            } else {
                                                let a228 = v0;
                                                let a229 = 20u64;
                                                if nat_eq(a228, a229) {
                                                    f42(v2, v4)?
                                                } else {
                                                    let a230 = v0;
                                                    let a231 = 42u64;
                                                    if nat_eq(a230, a231) {
                                                        f43(4u64, v2, v4)?
                                                    } else {
                                                        let a232 = v0;
                                                        let a233 = 69u64;
                                                        if nat_eq(a232, a233) {
                                                            let a234 = v5;
                                                            let a235 = 4u64;
                                                            if nat_lt(a234, a235) {
                                                                f43(v5, v2, v4)?
                                                            } else {
                                                                Adt8::C1
                                                            }
                                                        } else {
                                                            let a236 = v0;
                                                            let a237 = 70u64;
                                                            if nat_eq(a236, a237) {
                                                                f44(v5, v2, v4)?
                                                            } else {
                                                                let a238 = v0;
                                                                let a239 = 75u64;
                                                                if if nat_eq(a238, a239) {
                                                                    true
                                                                } else {
                                                                    let a240 = v0;
                                                                    let a241 = 76u64;
                                                                    nat_eq(a240, a241)
                                                                } {
                                                                    f45({
                                                                        let a242 = v0;
                                                                        let a243 = 76u64;
                                                                        nat_eq(a242, a243)
                                                                    }, v1, v2, v4)?
                                                                } else {
                                                                    let a244 = v0;
                                                                    let a245 = 31u64;
                                                                    if if if nat_eq(a244, a245) {
                                                                        true
                                                                    } else {
                                                                        let a246 = v0;
                                                                        let a247 = 47u64;
                                                                        nat_eq(a246, a247)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a248 = v0;
                                                                        let a249 = 68u64;
                                                                        nat_eq(a248, a249)
                                                                    } {
                                                                        f46(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a250 = v0;
                                                                        let a251 = 15u64;
                                                                        if nat_eq(a250, a251) {
                                                                            f47(v2, v4)?
                                                                        } else {
                                                                            let a252 = v0;
                                                                            let a253 = 17u64;
                                                                            if nat_eq(a252, a253) {
                                                                                f48(v2, v4)?
                                                                            } else {
                                                                                let a254 = v0;
                                                                                let a255 = 18u64;
                                                                                if nat_eq(a254, a255) {
                                                                                    f49(v2, v3, v4)?
                                                                                } else {
                                                                                    let a256 = v0;
                                                                                    let a257 = 13u64;
                                                                                    if if if nat_eq(a256, a257) {
                                                                                        true
                                                                                    } else {
                                                                                        let a258 = v0;
                                                                                        let a259 = 14u64;
                                                                                        nat_eq(a258, a259)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a260 = v0;
                                                                                        let a261 = 16u64;
                                                                                        nat_eq(a260, a261)
                                                                                    } {
                                                                                        f50(v0, v2, v4)?
                                                                                    } else {
                                                                                        let m262 = f51(v0, v5)?;
                                                                                        match m262 {
                                                                                            None => {
                                                                                                f52(v0, v5, v2, v3, v4)?
                                                                                            }
                                                                                            Some(v7) => {
                                                                                                f53(v7, v2, v4)?
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt7> {
    Ok(f54(v3.clone(), {
        let m263 = v3.clone();
        match m263 {
            Adt4::C0 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f56(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f57(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f58(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f59(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v58, v59, v60, v61) => {
                f62(v0, v61, v58, v59, v60, v4)?
            }
            Adt4::C24(_) => {
                Adt7::C0(Adt8::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C26(v53, v54, v55, v56) => {
                let a264 = v53;
                let a265 = 64u64;
                if if if nat_eq(a264, a265) {
                    true
                } else {
                    let a266 = v53;
                    let a267 = 65u64;
                    nat_eq(a266, a267)
                } {
                    true
                } else {
                    let a268 = v53;
                    let a269 = 66u64;
                    nat_eq(a268, a269)
                } {
                    Adt7::C0(Adt8::C5(64u64))
                } else {
                    let a270 = v53;
                    let a271 = 0u64;
                    if if nat_eq(a270, a271) {
                        true
                    } else {
                        let a272 = v53;
                        let a273 = 30u64;
                        nat_eq(a272, a273)
                    } {
                        f61(v0, v1, v2, v53, v54, v55, v56, v4)?
                    } else {
                        Adt7::C0(Adt8::C1)
                    }
                }
            }
            Adt4::C27(v52) => {
                f60(v0, v1, v2, v52)?
            }
        }
    }))
}

pub fn f8(v0: Adt4) -> R<Adt7> {
    let m274 = v0;
    match m274 {
        Adt4::C0 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C1 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C2(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C3(_, _, _, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C4(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C5(_, _, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C6(_, _, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C7(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C8(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C9(v18, v19) => {
            Ok(Adt7::C6(0u64, v18, v19, true))
        }
        Adt4::C10(_, v21, _) => {
            Ok(Adt7::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false))
        }
        Adt4::C11(_, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C12(_, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C13(_, _, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C14(_, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C15 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C16(_, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C17 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C18(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C19(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C20(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C21 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C22(_, _) => {
            Ok(Adt7::C6(7u64, 0u64, 0u64, false))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt7::C6(0u64, 0u64, 0u64, false))
        }
        Adt4::C24(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C25(v45, _, v47) => {
            Ok(Adt7::C6({
                let a279 = {
                    let a275 = 0u64;
                    let a276 = v45;
                    if nat_lt(a275, a276) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a280 = {
                    let a277 = 0u64;
                    let a278 = v47;
                    if nat_lt(a277, a278) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a279, a280)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v49, _, _, _) => {
            Ok(Adt7::C6({
                let a281 = v49;
                let a282 = 66u64;
                if nat_eq(a281, a282) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false))
        }
        Adt4::C27(_) => {
            Ok(Adt7::C6(2u64, 0u64, 0u64, false))
        }
    }
}

pub fn f9(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt7 {
    let a283 = v1;
    let a284 = 5u64;
    if if if if nat_lt(a283, a284) {
        let a285 = v5;
        let a286 = 5u64;
        nat_lt(a285, a286)
    } else {
        false
    } {
        let a287 = v7;
        let a288 = 5u64;
        nat_lt(a287, a288)
    } else {
        false
    } {
        let a289 = v9;
        let a290 = 2u64;
        if nat_lt(a289, a290) {
            true
        } else {
            let a291 = v9;
            let a292 = 3u64;
            nat_eq(a291, a292)
        }
    } else {
        false
    } {
        Adt7::C7(f63(v0, v1, v2, v3, v4), f64(v5, v6), f64(v7, v8), f65({
            let a293 = v9;
            let a294 = 0u64;
            if nat_eq(a293, a294) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt7::C0(Adt8::C1)
    }
}

pub fn f10(v0: Adt2, v1: u64) -> R<u64> {
    let a306 = {
        let a300 = {
            let a296 = match v0.clone() {
                Adt2::C0(_, h295, _, _, _, _, _, _, _, _, _) => {
                    h295
                }
            };
            let a297 = v1;
            let a298 = 0u64;
            nat_quot(a296, a297, a298)
        };
        let a301 = match v0.clone() {
            Adt2::C0(_, _, h299, _, _, _, _, _, _, _, _) => {
                h299
            }
        };
        nat_mul(a300, a301)?
    };
    let a307 = {
        let a304 = match v0.clone() {
            Adt2::C0(_, _, _, h302, _, _, _, _, _, _, _) => {
                h302
            }
        };
        let a305 = match v0 {
            Adt2::C0(_, _, _, _, h303, _, _, _, _, _, _) => {
                h303
            }
        };
        nat_mul(a304, a305)?
    };
    nat_mul(a306, a307)
}

pub fn f11(v0: u64) -> bool {
    let a308 = v0;
    let a309 = 4294967296u64;
    if nat_lt(a308, a309) {
        let a313 = {
            let a310 = v0;
            let a311 = 2147483648u64;
            let a312 = 0u64;
            nat_rem(a310, a311, a312)
        };
        let a314 = 2139095040u64;
        nat_lt(a313, a314)
    } else {
        false
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a315 = 4u64;
    let a316 = v0;
    nat_mul(a315, a316)
}

pub fn f13(v0: u64) -> R<u64> {
    let a317 = v0;
    let a318 = 0u64;
    if nat_eq(a317, a318) {
        Ok(0u64)
    } else {
        let a321 = 4u64;
        let a322 = {
            let a319 = 3u64;
            let a320 = v0;
            nat_mul(a319, a320)?
        };
        nat_add(a321, a322)
    }
}

pub fn f14(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a324 = match v0.clone() {
            Adt2::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a325 = v1;
        let a326 = 0u64;
        nat_quot(a324, a325, a326)
    }, match v0.clone() {
        Adt2::C0(_, _, h327, _, _, _, _, _, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h328, _, _, _, _, _, _, _) => {
            h328
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h329, _, _, _, _, _, _) => {
            h329
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h330, _, _, _, _, _) => {
            h330
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h331, _, _, _, _) => {
            h331
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h332, _, _, _) => {
            h332
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h333, _, _) => {
            h333
        }
    })
}

pub fn f15(v0: Adt2, v1: Adt2) -> bool {
    let a336 = match v0.clone() {
        Adt2::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
            h334
        }
    };
    let a337 = match v1.clone() {
        Adt2::C0(_, h335, _, _, _, _, _, _, _, _, _) => {
            h335
        }
    };
    if if if nat_eq(a336, a337) {
        let a340 = match v0.clone() {
            Adt2::C0(_, _, h338, _, _, _, _, _, _, _, _) => {
                h338
            }
        };
        let a341 = match v1.clone() {
            Adt2::C0(_, _, h339, _, _, _, _, _, _, _, _) => {
                h339
            }
        };
        nat_eq(a340, a341)
    } else {
        false
    } {
        let a344 = match v0.clone() {
            Adt2::C0(_, _, _, h342, _, _, _, _, _, _, _) => {
                h342
            }
        };
        let a345 = match v1.clone() {
            Adt2::C0(_, _, _, h343, _, _, _, _, _, _, _) => {
                h343
            }
        };
        nat_eq(a344, a345)
    } else {
        false
    } {
        let a348 = match v0 {
            Adt2::C0(_, _, _, _, h346, _, _, _, _, _, _) => {
                h346
            }
        };
        let a349 = match v1 {
            Adt2::C0(_, _, _, _, h347, _, _, _, _, _, _) => {
                h347
            }
        };
        nat_eq(a348, a349)
    } else {
        false
    }
}

pub fn f16(v0: Adt2) -> R<bool> {
    let a351 = match v0.clone() {
        Adt2::C0(_, h350, _, _, _, _, _, _, _, _, _) => {
            h350
        }
    };
    let a352 = 1u64;
    if if if if nat_eq(a351, a352) {
        true
    } else {
        let a354 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h353, _, _, _, _, _) => {
                h353
            }
        };
        let a355 = 4u64;
        nat_eq(a354, a355)
    } {
        let a357 = match v0.clone() {
            Adt2::C0(_, _, h356, _, _, _, _, _, _, _, _) => {
                h356
            }
        };
        let a358 = 1u64;
        if nat_eq(a357, a358) {
            true
        } else {
            let a363 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h359, _, _, _, _) => {
                    h359
                }
            };
            let a364 = {
                let a361 = match v0.clone() {
                    Adt2::C0(_, h360, _, _, _, _, _, _, _, _, _) => {
                        h360
                    }
                };
                let a362 = 4u64;
                nat_mul(a361, a362)?
            };
            nat_eq(a363, a364)
        }
    } else {
        false
    } {
        let a366 = match v0.clone() {
            Adt2::C0(_, _, _, h365, _, _, _, _, _, _, _) => {
                h365
            }
        };
        let a367 = 1u64;
        if nat_eq(a366, a367) {
            true
        } else {
            let a375 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h368, _, _, _) => {
                    h368
                }
            };
            let a376 = {
                let a373 = {
                    let a371 = match v0.clone() {
                        Adt2::C0(_, h369, _, _, _, _, _, _, _, _, _) => {
                            h369
                        }
                    };
                    let a372 = match v0.clone() {
                        Adt2::C0(_, _, h370, _, _, _, _, _, _, _, _) => {
                            h370
                        }
                    };
                    nat_mul(a371, a372)?
                };
                let a374 = 4u64;
                nat_mul(a373, a374)?
            };
            nat_eq(a375, a376)
        }
    } else {
        false
    } {
        let a378 = match v0.clone() {
            Adt2::C0(_, _, _, _, h377, _, _, _, _, _, _) => {
                h377
            }
        };
        let a379 = 1u64;
        if nat_eq(a378, a379) {
            Ok(true)
        } else {
            let a390 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h380, _, _) => {
                    h380
                }
            };
            let a391 = {
                let a388 = {
                    let a386 = {
                        let a383 = match v0.clone() {
                            Adt2::C0(_, h381, _, _, _, _, _, _, _, _, _) => {
                                h381
                            }
                        };
                        let a384 = match v0.clone() {
                            Adt2::C0(_, _, h382, _, _, _, _, _, _, _, _) => {
                                h382
                            }
                        };
                        nat_mul(a383, a384)?
                    };
                    let a387 = match v0 {
                        Adt2::C0(_, _, _, h385, _, _, _, _, _, _, _) => {
                            h385
                        }
                    };
                    nat_mul(a386, a387)?
                };
                let a389 = 4u64;
                nat_mul(a388, a389)?
            };
            Ok(nat_eq(a390, a391))
        }
    } else {
        Ok(false)
    }
}

pub fn f17(v0: Adt2, v1: Adt2) -> R<bool> {
    let a393 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h392, _) => {
            h392
        }
    };
    let a394 = 0u64;
    if nat_eq(a393, a394) {
        Ok(true)
    } else {
        let a396 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h395, _) => {
                h395
            }
        };
        let a397 = 0u64;
        if nat_eq(a396, a397) {
            Ok(true)
        } else {
            let a399 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h398) => {
                    h398
                }
            };
            let a400 = 0u64;
            if nat_eq(a399, a400) {
                Ok(true)
            } else {
                let a402 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h401) => {
                        h401
                    }
                };
                let a403 = 0u64;
                if nat_eq(a402, a403) {
                    Ok(true)
                } else {
                    let a409 = {
                        let a406 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h404) => {
                                h404
                            }
                        };
                        let a407 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h405, _) => {
                                h405
                            }
                        };
                        nat_add(a406, a407)?
                    };
                    let a410 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h408) => {
                            h408
                        }
                    };
                    if nat_le(a409, a410) {
                        Ok(true)
                    } else {
                        let a416 = {
                            let a413 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h411) => {
                                    h411
                                }
                            };
                            let a414 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h412, _) => {
                                    h412
                                }
                            };
                            nat_add(a413, a414)?
                        };
                        let a417 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h415) => {
                                h415
                            }
                        };
                        Ok(nat_le(a416, a417))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a422 = {
        let a418 = v0;
        let a419 = v1;
        nat_mul(a418, a419)?
    };
    let a423 = {
        let a420 = v2;
        let a421 = f66(v3);
        nat_mul(a420, a421)?
    };
    nat_mul(a422, a423)
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a426 = v0;
    let a427 = {
        let a424 = f67(v1)?;
        let a425 = v2;
        nat_mul(a424, a425)?
    };
    nat_add(a426, a427)
}

pub fn f20(v0: u64) -> u64 {
    let a428 = v0;
    let a429 = 30u64;
    if nat_eq(a428, a429) {
        2u64
    } else {
        4u64
    }
}

pub fn f21(v0: u64) -> bool {
    let a430 = v0;
    let a431 = 2139095040u64;
    if nat_le(a430, a431) {
        true
    } else {
        let a432 = v0;
        let a433 = 2147483648u64;
        nat_eq(a432, a433)
    }
}

pub fn f22(v0: Adt2, v1: u64) -> bool {
    let a435 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h434, _, _, _, _, _) => {
            h434
        }
    };
    let a436 = v1;
    if if nat_eq(a435, a436) {
        let a441 = {
            let a438 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h437) => {
                    h437
                }
            };
            let a439 = v1;
            let a440 = 0u64;
            nat_rem(a438, a439, a440)
        };
        let a442 = 0u64;
        nat_eq(a441, a442)
    } else {
        false
    } {
        f68(match v0.clone() {
            Adt2::C0(_, _, h443, _, _, _, _, _, _, _, _) => {
                h443
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h444, _, _, _, _, _, _, _) => {
                h444
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h445, _, _, _, _, _, _) => {
                h445
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h446, _, _, _, _) => {
                h446
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h447, _, _, _) => {
                h447
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h448, _, _) => {
                h448
            }
        }, v1)
    } else {
        false
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a451 = match v0.clone() {
            Adt2::C0(h449, _, _, _, _, _, _, _, _, _, _) => {
                h449
            }
        };
        let a452 = match v1.clone() {
            Adt2::C0(h450, _, _, _, _, _, _, _, _, _, _) => {
                h450
            }
        };
        nat_eq(a451, a452)
    } else {
        false
    } {
        let a455 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h453) => {
                h453
            }
        };
        let a456 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h454) => {
                h454
            }
        };
        nat_eq(a455, a456)
    } else {
        false
    } {
        let a459 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h457, _, _, _, _, _) => {
                h457
            }
        };
        let a460 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h458, _, _, _, _, _) => {
                h458
            }
        };
        nat_eq(a459, a460)
    } else {
        false
    } {
        let a463 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h461, _, _, _, _) => {
                h461
            }
        };
        let a464 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h462, _, _, _, _) => {
                h462
            }
        };
        nat_eq(a463, a464)
    } else {
        false
    } {
        let a467 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h465, _, _, _) => {
                h465
            }
        };
        let a468 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h466, _, _, _) => {
                h466
            }
        };
        nat_eq(a467, a468)
    } else {
        false
    } {
        let a471 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h469, _, _) => {
                h469
            }
        };
        let a472 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h470, _, _) => {
                h470
            }
        };
        nat_eq(a471, a472)
    } else {
        false
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a473 = v0;
    let a474 = 0u64;
    if if if nat_eq(a473, a474) {
        true
    } else {
        let a475 = v1;
        let a476 = 0u64;
        nat_eq(a475, a476)
    } {
        true
    } else {
        let a477 = v2;
        let a478 = 0u64;
        nat_eq(a477, a478)
    } {
        true
    } else {
        let a479 = v3;
        let a480 = 0u64;
        nat_eq(a479, a480)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> R<bool> {
    f69({
        let a482 = match v0.clone() {
            Adt2::C0(_, h481, _, _, _, _, _, _, _, _, _) => {
                h481
            }
        };
        let a483 = v1;
        nat_mul(a482, a483)?
    }, match v0.clone() {
        Adt2::C0(_, _, h484, _, _, _, _, _, _, _, _) => {
            h484
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h485, _, _, _, _, _, _, _) => {
            h485
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h486, _, _, _, _, _, _) => {
            h486
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h487, _, _, _, _) => {
            h487
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h488, _, _, _) => {
            h488
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h489, _, _) => {
            h489
        }
    })
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a490 = v1;
    let a491 = 0u64;
    if if nat_eq(a490, a491) {
        true
    } else {
        let a492 = v2;
        let a493 = 0u64;
        nat_eq(a492, a493)
    } {
        Ok(0u64)
    } else {
        let a500 = {
            let a494 = f70(v1, v2, v3)?;
            let a495 = f71(v0, v1, v2)?;
            nat_add(a494, a495)?
        };
        let a501 = {
            let a498 = f72(v0);
            let a499 = {
                let a496 = v1;
                let a497 = v2;
                nat_mul(a496, a497)?
            };
            nat_mul(a498, a499)?
        };
        nat_add(a500, a501)
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a502 = v0;
    let a503 = 0u64;
    if if nat_eq(a502, a503) {
        true
    } else {
        let a504 = v1;
        let a505 = 0u64;
        nat_eq(a504, a505)
    } {
        Ok(3u64)
    } else {
        let a508 = {
            let a506 = 3u64;
            let a507 = v1;
            nat_add(a506, a507)?
        };
        let a509 = v2;
        nat_add(a508, a509)
    }
}

pub fn f28(v0: u64) -> bool {
    let a510 = v0;
    let a511 = 2139095040u64;
    if nat_le(a510, a511) {
        true
    } else {
        let a512 = v0;
        let a513 = 2147483648u64;
        nat_eq(a512, a513)
    }
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let a514 = v0;
    let a515 = 0u64;
    if nat_eq(a514, a515) {
        Ok(0u64)
    } else {
        let a516 = v1;
        let a517 = f73(v0)?;
        nat_mul(a516, a517)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a518 = v0;
    let a519 = 0u64;
    if if if if nat_eq(a518, a519) {
        true
    } else {
        let a520 = v1;
        let a521 = 0u64;
        nat_eq(a520, a521)
    } {
        true
    } else {
        let a522 = v2;
        let a523 = 0u64;
        nat_eq(a522, a523)
    } {
        true
    } else {
        let a524 = v3;
        let a525 = 0u64;
        nat_eq(a524, a525)
    } {
        Ok(0u64)
    } else {
        let a538 = {
            let a528 = 2u64;
            let a529 = {
                let a526 = v1;
                let a527 = v2;
                nat_mul(a526, a527)?
            };
            nat_mul(a528, a529)?
        };
        let a539 = {
            let a536 = {
                let a532 = {
                    let a530 = v1;
                    let a531 = v2;
                    nat_mul(a530, a531)?
                };
                let a533 = v3;
                nat_mul(a532, a533)?
            };
            let a537 = {
                let a534 = 1u64;
                let a535 = v0;
                nat_add(a534, a535)?
            };
            nat_mul(a536, a537)?
        };
        nat_add(a538, a539)
    }
}

pub fn f31(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt8> {
    let m540 = v2;
    match m540 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a542 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h541, _, _, _) => {
                        h541
                    }
                };
                let a543 = 1u64;
                nat_eq(a542, a543)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h544, _, _, _, _, _, _, _, _, _) => {
                    h544
                }
            }, match v0.clone() {
                Adt1::C0(h545, _, _, _, _, _, _, _, _, _) => {
                    h545
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h546, _, _, _, _, _, _, _) => {
                    h546
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h547, _, _, _, _, _) => {
                    h547
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h548, _, _, _, _, _, _, _, _) => {
                        h548
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h549, _, _, _, _, _, _, _, _) => {
                        h549
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h550, _, _, _, _, _, _) => {
                        h550
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h551, _, _, _, _) => {
                        h551
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h552, _, _, _, _, _, _, _) => {
                        h552
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h553, _, _, _, _, _, _, _, _) => {
                        h553
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h554, _, _, _, _, _, _) => {
                        h554
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h555, _, _, _, _, _, _, _) => {
                        h555
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h556, _, _, _, _, _, _, _, _) => {
                        h556
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h557, _, _, _, _, _, _, _, _, _, _) => {
                    h557
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h558, _, _, _, _, _, _, _, _, _, _) => {
                    h558
                }
            };
            let v14: u64 = {
                let a559 = v12;
                let a560 = 0u64;
                if nat_eq(a559, a560) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a562 = match v4.clone() {
                    Adt2::C0(_, h561, _, _, _, _, _, _, _, _, _) => {
                        h561
                    }
                };
                let a563 = v14;
                nat_mul(a562, a563)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h564, _, _, _) => {
                        h564
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h565, _, _, _, _) => {
                        h565
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h566, _, _) => {
                        h566
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h567, _, _, _) => {
                        h567
                    }
                }
            };
            let a569 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h568, _, _, _) => {
                    h568
                }
            };
            let a570 = 1u64;
            if if if if if if if if if if nat_le(a569, a570) {
                let a571 = v12;
                let a572 = 0u64;
                if nat_eq(a571, a572) {
                    true
                } else {
                    let a573 = v12;
                    let a574 = 1u64;
                    if nat_eq(a573, a574) {
                        let a575 = v13;
                        let a576 = 1u64;
                        nat_eq(a575, a576)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a577 = v13;
                let a578 = 0u64;
                if nat_eq(a577, a578) {
                    true
                } else {
                    let a579 = v13;
                    let a580 = 1u64;
                    nat_eq(a579, a580)
                }
            } else {
                false
            } {
                let a582 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h581, _, _, _, _, _) => {
                        h581
                    }
                };
                let a583 = v14;
                nat_eq(a582, a583)
            } else {
                false
            } {
                let a591 = match v3.clone() {
                    Adt2::C0(_, h584, _, _, _, _, _, _, _, _, _) => {
                        h584
                    }
                };
                let a592 = {
                    let a589 = v9;
                    let a590 = {
                        let a587 = match v6.clone() {
                            Adt6::C0(h585, _, _, _) => {
                                h585
                            }
                        };
                        let a588 = match v7.clone() {
                            Adt6::C0(h586, _, _, _) => {
                                h586
                            }
                        };
                        nat_mul(a587, a588)?
                    };
                    nat_mul(a589, a590)?
                };
                nat_eq(a591, a592)
            } else {
                false
            } {
                let a595 = match v3.clone() {
                    Adt2::C0(_, _, h593, _, _, _, _, _, _, _, _) => {
                        h593
                    }
                };
                let a596 = f74(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h594, _, _, _, _, _, _, _, _, _) => {
                        h594
                    }
                })?;
                nat_eq(a595, a596)
            } else {
                false
            } {
                let a598 = 0u64;
                let a599 = match v3.clone() {
                    Adt2::C0(_, _, h597, _, _, _, _, _, _, _, _) => {
                        h597
                    }
                };
                nat_lt(a598, a599)
            } else {
                false
            } {
                if v5 {
                    let a601 = match v1 {
                        Adt2::C0(_, _, _, h600, _, _, _, _, _, _, _) => {
                            h600
                        }
                    };
                    let a602 = v9;
                    if if if nat_eq(a601, a602) {
                        let a604 = match v3.clone() {
                            Adt2::C0(_, _, _, h603, _, _, _, _, _, _, _) => {
                                h603
                            }
                        };
                        let a605 = f74(v7.clone(), v11)?;
                        nat_eq(a604, a605)
                    } else {
                        false
                    } {
                        let a607 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h606, _, _, _, _, _, _) => {
                                h606
                            }
                        };
                        let a608 = v10;
                        nat_eq(a607, a608)
                    } else {
                        false
                    } {
                        let a610 = 0u64;
                        let a611 = match v3.clone() {
                            Adt2::C0(_, _, _, h609, _, _, _, _, _, _, _) => {
                                h609
                            }
                        };
                        nat_lt(a610, a611)
                    } else {
                        false
                    }
                } else {
                    let a613 = match v1 {
                        Adt2::C0(_, _, h612, _, _, _, _, _, _, _, _) => {
                            h612
                        }
                    };
                    let a614 = v9;
                    if if if nat_eq(a613, a614) {
                        let a616 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h615, _, _, _, _, _, _) => {
                                h615
                            }
                        };
                        let a617 = 1u64;
                        nat_eq(a616, a617)
                    } else {
                        false
                    } {
                        let a619 = match v3.clone() {
                            Adt2::C0(_, _, _, h618, _, _, _, _, _, _, _) => {
                                h618
                            }
                        };
                        let a620 = v10;
                        nat_eq(a619, a620)
                    } else {
                        false
                    } {
                        let a622 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h621, _, _, _, _, _, _) => {
                                h621
                            }
                        };
                        let a623 = 1u64;
                        nat_eq(a622, a623)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f24(match v4.clone() {
                    Adt2::C0(_, h624, _, _, _, _, _, _, _, _, _) => {
                        h624
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a644 = {
                        let a641 = {
                            let a639 = {
                                let a633 = {
                                    let a627 = {
                                        let a625 = v10;
                                        let a626 = 1u64;
                                        nat_sub(a625, a626)
                                    };
                                    let a628 = v17;
                                    nat_mul(a627, a628)?
                                };
                                let a634 = {
                                    let a631 = {
                                        let a629 = v9;
                                        let a630 = 1u64;
                                        nat_sub(a629, a630)
                                    };
                                    let a632 = v16;
                                    nat_mul(a631, a632)?
                                };
                                nat_add(a633, a634)?
                            };
                            let a640 = {
                                let a637 = {
                                    let a635 = v11;
                                    let a636 = 1u64;
                                    nat_sub(a635, a636)
                                };
                                let a638 = v15;
                                nat_mul(a637, a638)?
                            };
                            nat_add(a639, a640)?
                        };
                        let a642 = v15;
                        nat_add(a641, a642)?
                    };
                    let a645 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h643, _) => {
                            h643
                        }
                    };
                    nat_le(a644, a645)
                }
            } else {
                false
            } {
                f17(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt8::C4(f10(v3.clone(), 1u64)?, {
                    let a646 = v13;
                    let a647 = 0u64;
                    if nat_eq(a646, a647) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h648, _, _, _, _, _, _, _, _, _) => {
                        h648
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f14(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h649, _, _, _, _, _, _, _, _, _) => {
                            h649
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h650, _, _, _, _, _, _, _, _) => {
                            h650
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h651, _, _, _, _, _, _, _) => {
                            h651
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h652, _, _, _, _, _) => {
                            h652
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h653, _, _, _, _) => {
                            h653
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h654, _, _, _) => {
                            h654
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f32(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt8> {
    let m655 = v2;
    match m655 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h656, _, _, _, _, _, _, _, _, _) => {
                    h656
                }
            }, match v0.clone() {
                Adt1::C0(h657, _, _, _, _, _, _, _, _, _) => {
                    h657
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h658, _, _, _, _, _, _) => {
                    h658
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h659, _, _, _) => {
                    h659
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
                    h660
                }
            }, match v0.clone() {
                Adt1::C0(_, h661, _, _, _, _, _, _, _, _) => {
                    h661
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h662, _, _, _, _, _) => {
                    h662
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h663, _, _) => {
                    h663
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h664, _, _, _, _, _, _, _) => {
                    h664
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h665, _, _, _, _, _, _, _) => {
                    h665
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h666, _, _, _, _) => {
                    h666
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h667, _) => {
                    h667
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h668) => {
                    h668
                }
            };
            let v9: u64 = f74(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h669, _, _, _, _, _, _, _) => {
                    h669
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h670, _, _, _, _, _, _, _, _, _, _) => {
                    h670
                }
            };
            let v11: u64 = {
                let a678 = v8;
                let a679 = {
                    let a676 = match v5.clone() {
                        Adt6::C0(h671, _, _, _) => {
                            h671
                        }
                    };
                    let a677 = {
                        let a674 = match v6.clone() {
                            Adt6::C0(h672, _, _, _) => {
                                h672
                            }
                        };
                        let a675 = match v7.clone() {
                            Adt6::C0(h673, _, _, _) => {
                                h673
                            }
                        };
                        nat_mul(a674, a675)?
                    };
                    nat_mul(a676, a677)?
                };
                nat_mul(a678, a679)?
            };
            let a681 = match v4.clone() {
                Adt2::C0(h680, _, _, _, _, _, _, _, _, _, _) => {
                    h680
                }
            };
            let a682 = 0u64;
            if if if if if if if if if if if if if nat_eq(a681, a682) {
                let a683 = v10;
                let a684 = 0u64;
                if nat_eq(a683, a684) {
                    true
                } else {
                    let a685 = v10;
                    let a686 = 1u64;
                    nat_eq(a685, a686)
                }
            } else {
                false
            } {
                let a687 = 0u64;
                let a688 = v8;
                nat_lt(a687, a688)
            } else {
                false
            } {
                let a689 = v8;
                let a690 = 2147483648u64;
                nat_lt(a689, a690)
            } else {
                false
            } {
                let a692 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h691, _, _, _, _, _) => {
                        h691
                    }
                };
                let a693 = 4u64;
                nat_eq(a692, a693)
            } else {
                false
            } {
                let a694 = 0u64;
                let a695 = v9;
                nat_lt(a694, a695)
            } else {
                false
            } {
                let a697 = 0u64;
                let a698 = match v3.clone() {
                    Adt2::C0(_, _, h696, _, _, _, _, _, _, _, _) => {
                        h696
                    }
                };
                nat_lt(a697, a698)
            } else {
                false
            } {
                let a700 = 0u64;
                let a701 = match v3.clone() {
                    Adt2::C0(_, _, _, h699, _, _, _, _, _, _, _) => {
                        h699
                    }
                };
                nat_lt(a700, a701)
            } else {
                false
            } {
                let a703 = match v3.clone() {
                    Adt2::C0(_, h702, _, _, _, _, _, _, _, _, _) => {
                        h702
                    }
                };
                let a704 = v11;
                nat_eq(a703, a704)
            } else {
                false
            } {
                let a707 = match v3.clone() {
                    Adt2::C0(_, _, h705, _, _, _, _, _, _, _, _) => {
                        h705
                    }
                };
                let a708 = f74(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h706, _, _, _, _, _, _, _, _, _) => {
                        h706
                    }
                })?;
                nat_eq(a707, a708)
            } else {
                false
            } {
                let a711 = match v3.clone() {
                    Adt2::C0(_, _, _, h709, _, _, _, _, _, _, _) => {
                        h709
                    }
                };
                let a712 = f74(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h710, _, _, _, _, _, _, _, _) => {
                        h710
                    }
                })?;
                nat_eq(a711, a712)
            } else {
                false
            } {
                let a720 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h713, _, _, _, _, _, _) => {
                        h713
                    }
                };
                let a721 = {
                    let a718 = v9;
                    let a719 = {
                        let a715 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h714, _, _, _, _, _, _) => {
                                h714
                            }
                        };
                        let a716 = v8;
                        let a717 = 0u64;
                        nat_quot(a715, a716, a717)
                    };
                    nat_mul(a718, a719)?
                };
                nat_eq(a720, a721)
            } else {
                false
            } {
                f17(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt8::C4(f10(v3.clone(), 1u64)?, {
                    let a722 = v10;
                    let a723 = 0u64;
                    if nat_eq(a722, a723) {
                        4u64
                    } else {
                        2u64
                    }
                }, f14(v4.clone(), 1u64), f14(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h724, _, _) => {
                        h724
                    }
                }, {
                    let a726 = v8;
                    let a727 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h725, _, _) => {
                            h725
                        }
                    };
                    nat_mul(a726, a727)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f33(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt8> {
    let m728 = v1;
    match m728 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v3) => {
            f75(v0, v3, v2)
        }
    }
}

pub fn f34(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a730 = match v0.clone() {
        Adt2::C0(h729, _, _, _, _, _, _, _, _, _, _) => {
            h729
        }
    };
    let a731 = 1u64;
    if if nat_eq(a730, a731) {
        true
    } else {
        let a733 = match v0.clone() {
            Adt2::C0(h732, _, _, _, _, _, _, _, _, _, _) => {
                h732
            }
        };
        let a734 = 30u64;
        nat_eq(a733, a734)
    } {
        let a736 = match v1.clone() {
            Adt2::C0(h735, _, _, _, _, _, _, _, _, _, _) => {
                h735
            }
        };
        let a737 = 1u64;
        if if if if if if if if if if if nat_eq(a736, a737) {
            let a740 = match v0.clone() {
                Adt2::C0(_, h738, _, _, _, _, _, _, _, _, _) => {
                    h738
                }
            };
            let a741 = match v1.clone() {
                Adt2::C0(_, h739, _, _, _, _, _, _, _, _, _) => {
                    h739
                }
            };
            nat_eq(a740, a741)
        } else {
            false
        } {
            let a743 = 0u64;
            let a744 = match v1.clone() {
                Adt2::C0(_, _, h742, _, _, _, _, _, _, _, _) => {
                    h742
                }
            };
            nat_lt(a743, a744)
        } else {
            false
        } {
            let a747 = match v1.clone() {
                Adt2::C0(_, _, h745, _, _, _, _, _, _, _, _) => {
                    h745
                }
            };
            let a748 = match v1.clone() {
                Adt2::C0(_, _, _, h746, _, _, _, _, _, _, _) => {
                    h746
                }
            };
            nat_eq(a747, a748)
        } else {
            false
        } {
            let a750 = match v1.clone() {
                Adt2::C0(_, _, _, _, h749, _, _, _, _, _, _) => {
                    h749
                }
            };
            let a751 = 1u64;
            nat_eq(a750, a751)
        } else {
            false
        } {
            let a759 = match v0.clone() {
                Adt2::C0(_, _, h752, _, _, _, _, _, _, _, _) => {
                    h752
                }
            };
            let a760 = {
                let a757 = {
                    let a755 = match v1.clone() {
                        Adt2::C0(_, _, h753, _, _, _, _, _, _, _, _) => {
                            h753
                        }
                    };
                    let a756 = match v1.clone() {
                        Adt2::C0(_, _, h754, _, _, _, _, _, _, _, _) => {
                            h754
                        }
                    };
                    nat_add(a755, a756)?
                };
                let a758 = 1u64;
                nat_sub(a757, a758)
            };
            nat_eq(a759, a760)
        } else {
            false
        } {
            let a762 = 0u64;
            let a763 = match v0.clone() {
                Adt2::C0(_, _, _, h761, _, _, _, _, _, _, _) => {
                    h761
                }
            };
            nat_lt(a762, a763)
        } else {
            false
        } {
            let a765 = 0u64;
            let a766 = match v0.clone() {
                Adt2::C0(_, _, _, _, h764, _, _, _, _, _, _) => {
                    h764
                }
            };
            nat_lt(a765, a766)
        } else {
            false
        } {
            let a768 = 2u64;
            let a769 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h767, _, _, _, _, _) => {
                    h767
                }
            };
            nat_le(a768, a769)
        } else {
            false
        } {
            let a771 = 2u64;
            let a772 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h770, _, _, _, _, _) => {
                    h770
                }
            };
            nat_le(a771, a772)
        } else {
            false
        } {
            f17(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt8::C4(f10(v1.clone(), 1u64)?, 2u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f35(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt8> {
    let m773 = v1;
    match m773 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v3) => {
            f76(v0, v3, v2)
        }
    }
}

pub fn f36(v0: u64) -> bool {
    let a774 = v0;
    let a775 = 1u64;
    if if nat_eq(a774, a775) {
        true
    } else {
        let a776 = v0;
        let a777 = 33u64;
        nat_eq(a776, a777)
    } {
        true
    } else {
        let a778 = v0;
        let a779 = 34u64;
        nat_eq(a778, a779)
    }
}

pub fn f37(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt8> {
    let a782 = match v1.clone() {
        Adt2::C0(h780, _, _, _, _, _, _, _, _, _, _) => {
            h780
        }
    };
    let a783 = match v3.clone() {
        Adt2::C0(h781, _, _, _, _, _, _, _, _, _, _) => {
            h781
        }
    };
    if nat_eq(a782, a783) {
        f77(v0, v1, v2, v3)
    } else {
        if if if if f78(match v1.clone() {
            Adt2::C0(h784, _, _, _, _, _, _, _, _, _, _) => {
                h784
            }
        }) {
            f78(match v3.clone() {
                Adt2::C0(h785, _, _, _, _, _, _, _, _, _, _) => {
                    h785
                }
            })
        } else {
            false
        } {
            let a786 = v0;
            let a787 = 33u64;
            if nat_eq(a786, a787) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a788 = f10(v1.clone(), 1u64)?;
            let a789 = f10(v3.clone(), 1u64)?;
            nat_eq(a788, a789)
        } else {
            false
        } {
            let m791 = f79(match v3.clone() {
                Adt2::C0(h790, _, _, _, _, _, _, _, _, _, _) => {
                    h790
                }
            });
            match m791 {
                Adt12::C0 => {
                    Ok(Adt8::C0)
                }
                Adt12::C1(v4) => {
                    Ok(Adt8::C4(f10(v3.clone(), 1u64)?, match v4 {
                        Adt13::C0(_, _, h792) => {
                            h792
                        }
                    }, f14(v1.clone(), 1u64), f14(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h793, _, _, _, _, _, _, _, _, _, _) => {
                            h793
                        }
                    }, match v3 {
                        Adt2::C0(h794, _, _, _, _, _, _, _, _, _, _) => {
                            h794
                        }
                    })))
                }
            }
        } else {
            Ok(Adt8::C1)
        }
    }
}

pub fn f38(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a796 = match v0.clone() {
        Adt2::C0(h795, _, _, _, _, _, _, _, _, _, _) => {
            h795
        }
    };
    let a797 = 0u64;
    if if if if nat_eq(a796, a797) {
        true
    } else {
        let a799 = match v0.clone() {
            Adt2::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                h798
            }
        };
        let a800 = 1u64;
        nat_eq(a799, a800)
    } {
        true
    } else {
        let a802 = match v0.clone() {
            Adt2::C0(h801, _, _, _, _, _, _, _, _, _, _) => {
                h801
            }
        };
        let a803 = 25u64;
        if nat_eq(a802, a803) {
            true
        } else {
            let a805 = match v0.clone() {
                Adt2::C0(h804, _, _, _, _, _, _, _, _, _, _) => {
                    h804
                }
            };
            let a806 = 26u64;
            nat_eq(a805, a806)
        }
    } {
        true
    } else {
        let a808 = match v0.clone() {
            Adt2::C0(h807, _, _, _, _, _, _, _, _, _, _) => {
                h807
            }
        };
        let a809 = 30u64;
        nat_eq(a808, a809)
    } {
        let a812 = match v0.clone() {
            Adt2::C0(h810, _, _, _, _, _, _, _, _, _, _) => {
                h810
            }
        };
        let a813 = match v1.clone() {
            Adt2::C0(h811, _, _, _, _, _, _, _, _, _, _) => {
                h811
            }
        };
        if if nat_eq(a812, a813) {
            f80(match v1.clone() {
                Adt2::C0(_, h814, _, _, _, _, _, _, _, _, _) => {
                    h814
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h815, _, _, _, _, _, _, _, _) => {
                    h815
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h816, _, _, _, _, _, _, _) => {
                    h816
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h817, _, _, _, _, _, _) => {
                    h817
                }
            }, match v0.clone() {
                Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                    h818
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h819, _, _, _, _, _, _, _, _) => {
                    h819
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h820, _, _, _, _, _, _, _) => {
                    h820
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h821, _, _, _, _, _, _) => {
                    h821
                }
            })
        } else {
            false
        } {
            let m823 = f79(match v0.clone() {
                Adt2::C0(h822, _, _, _, _, _, _, _, _, _, _) => {
                    h822
                }
            });
            match m823 {
                Adt12::C0 => {
                    Ok(Adt8::C0)
                }
                Adt12::C1(v2) => {
                    Ok(Adt8::C4(f10(v1.clone(), 1u64)?, match v2 {
                        Adt13::C0(_, _, h824) => {
                            h824
                        }
                    }, f14(v0, 1u64), f14(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f39(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt8> {
    let m825 = v2;
    match m825 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v4) => {
            let a826 = v0;
            let a827 = 4u64;
            if if if nat_lt(a826, a827) {
                let a830 = match v1.clone() {
                    Adt2::C0(h828, _, _, _, _, _, _, _, _, _, _) => {
                        h828
                    }
                };
                let a831 = match v4.clone() {
                    Adt2::C0(h829, _, _, _, _, _, _, _, _, _, _) => {
                        h829
                    }
                };
                if nat_eq(a830, a831) {
                    let a834 = match v1.clone() {
                        Adt2::C0(h832, _, _, _, _, _, _, _, _, _, _) => {
                            h832
                        }
                    };
                    let a835 = match v3.clone() {
                        Adt2::C0(h833, _, _, _, _, _, _, _, _, _, _) => {
                            h833
                        }
                    };
                    nat_eq(a834, a835)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f81(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                        h836
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h837, _, _, _, _, _, _, _, _, _) => {
                        h837
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h838, _, _, _, _, _, _, _, _, _) => {
                        h838
                    }
                })? {
                    f81(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h839, _, _, _, _, _, _, _, _) => {
                            h839
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h840, _, _, _, _, _, _, _, _) => {
                            h840
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h841, _, _, _, _, _, _, _, _) => {
                            h841
                        }
                    })?
                } else {
                    false
                } {
                    f81(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h842, _, _, _, _, _, _, _) => {
                            h842
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h843, _, _, _, _, _, _, _) => {
                            h843
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h844, _, _, _, _, _, _, _) => {
                            h844
                        }
                    })?
                } else {
                    false
                } {
                    f81(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h845, _, _, _, _, _, _) => {
                            h845
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h846, _, _, _, _, _, _) => {
                            h846
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h847, _, _, _, _, _, _) => {
                            h847
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m849 = f79(match v1.clone() {
                    Adt2::C0(h848, _, _, _, _, _, _, _, _, _, _) => {
                        h848
                    }
                });
                match m849 {
                    Adt12::C0 => {
                        Ok(Adt8::C0)
                    }
                    Adt12::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt13::C0(_, h850, _) => {
                                h850
                            }
                        };
                        let a855 = {
                            let a852 = match v1.clone() {
                                Adt2::C0(_, h851, _, _, _, _, _, _, _, _, _) => {
                                    h851
                                }
                            };
                            let a853 = v6;
                            let a854 = 1u64;
                            nat_rem(a852, a853, a854)
                        };
                        let a856 = 0u64;
                        if if if nat_eq(a855, a856) {
                            let a861 = {
                                let a858 = match v4.clone() {
                                    Adt2::C0(_, h857, _, _, _, _, _, _, _, _, _) => {
                                        h857
                                    }
                                };
                                let a859 = v6;
                                let a860 = 1u64;
                                nat_rem(a858, a859, a860)
                            };
                            let a862 = 0u64;
                            nat_eq(a861, a862)
                        } else {
                            false
                        } {
                            let a867 = {
                                let a864 = match v3.clone() {
                                    Adt2::C0(_, h863, _, _, _, _, _, _, _, _, _) => {
                                        h863
                                    }
                                };
                                let a865 = v6;
                                let a866 = 1u64;
                                nat_rem(a864, a865, a866)
                            };
                            let a868 = 0u64;
                            nat_eq(a867, a868)
                        } else {
                            false
                        } {
                            Ok(Adt8::C4(f10(v3.clone(), v6)?, match v5 {
                                Adt13::C0(_, _, h869) => {
                                    h869
                                }
                            }, f14(v1, v6), f14(v3, v6), Adt4::C2(v0, f14(v4, v6))))
                        } else {
                            Ok(Adt8::C1)
                        }
                    }
                }
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f40(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt8> {
    let a871 = match v2.clone() {
        Adt2::C0(h870, _, _, _, _, _, _, _, _, _, _) => {
            h870
        }
    };
    let a872 = 0u64;
    if if nat_eq(a871, a872) {
        let a874 = match v3.clone() {
            Adt2::C0(h873, _, _, _, _, _, _, _, _, _, _) => {
                h873
            }
        };
        let a875 = 0u64;
        nat_eq(a874, a875)
    } else {
        false
    } {
        let a876 = v0;
        let a877 = 61u64;
        if nat_eq(a876, a877) {
            let v4: bool = {
                let a879 = 0u64;
                let a880 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h878, _) => {
                        h878
                    }
                };
                nat_lt(a879, a880)
            };
            if if if if if if f82(match v2.clone() {
                Adt2::C0(_, h881, _, _, _, _, _, _, _, _, _) => {
                    h881
                }
            }, match v1.clone() {
                Adt1::C0(h882, _, _, _, _, _, _, _, _, _) => {
                    h882
                }
            }, match v1.clone() {
                Adt1::C0(_, h883, _, _, _, _, _, _, _, _) => {
                    h883
                }
            }, match v3.clone() {
                Adt2::C0(_, h884, _, _, _, _, _, _, _, _, _) => {
                    h884
                }
            })? {
                f82(match v2.clone() {
                    Adt2::C0(_, _, h885, _, _, _, _, _, _, _, _) => {
                        h885
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h886, _, _, _, _, _, _, _) => {
                        h886
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h887, _, _, _, _, _, _) => {
                        h887
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h888, _, _, _, _, _, _, _, _) => {
                        h888
                    }
                })?
            } else {
                false
            } {
                f82(match v2.clone() {
                    Adt2::C0(_, _, _, h889, _, _, _, _, _, _, _) => {
                        h889
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h890, _, _, _, _, _) => {
                        h890
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h891, _, _, _, _) => {
                        h891
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h892, _, _, _, _, _, _, _) => {
                        h892
                    }
                })?
            } else {
                false
            } {
                f82(match v2.clone() {
                    Adt2::C0(_, _, _, _, h893, _, _, _, _, _, _) => {
                        h893
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h894, _, _, _) => {
                        h894
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h895, _, _) => {
                        h895
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h896, _, _, _, _, _, _) => {
                        h896
                    }
                })?
            } else {
                false
            } {
                let a898 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h897, _) => {
                        h897
                    }
                };
                let a899 = 4294967295u64;
                nat_le(a898, a899)
            } else {
                false
            } {
                if v4 {
                    !f24(match v2.clone() {
                        Adt2::C0(_, h900, _, _, _, _, _, _, _, _, _) => {
                            h900
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h901, _, _, _, _, _, _, _, _) => {
                            h901
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h902, _, _, _, _, _, _, _) => {
                            h902
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h903, _, _, _, _, _, _) => {
                            h903
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h904, _, _, _, _, _, _, _, _, _) => {
                        h904
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h905, _, _, _, _, _, _, _) => {
                        h905
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h906, _, _, _, _, _) => {
                        h906
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h907, _, _, _) => {
                        h907
                    }
                }, v4)))
            } else {
                Ok(Adt8::C1)
            }
        } else {
            let a908 = v0;
            let a909 = 62u64;
            if nat_eq(a908, a909) {
                let a911 = match v1.clone() {
                    Adt1::C0(h910, _, _, _, _, _, _, _, _, _) => {
                        h910
                    }
                };
                let a912 = 2147483648u64;
                if if if if if nat_lt(a911, a912) {
                    let a914 = match v1.clone() {
                        Adt1::C0(_, h913, _, _, _, _, _, _, _, _) => {
                            h913
                        }
                    };
                    let a915 = 2147483648u64;
                    nat_lt(a914, a915)
                } else {
                    false
                } {
                    let a918 = match v1.clone() {
                        Adt1::C0(h916, _, _, _, _, _, _, _, _, _) => {
                            h916
                        }
                    };
                    let a919 = match v2.clone() {
                        Adt2::C0(_, h917, _, _, _, _, _, _, _, _, _) => {
                            h917
                        }
                    };
                    nat_lt(a918, a919)
                } else {
                    false
                } {
                    let a922 = match v1.clone() {
                        Adt1::C0(_, h920, _, _, _, _, _, _, _, _) => {
                            h920
                        }
                    };
                    let a923 = match v2.clone() {
                        Adt2::C0(_, h921, _, _, _, _, _, _, _, _, _) => {
                            h921
                        }
                    };
                    nat_lt(a922, a923)
                } else {
                    false
                } {
                    let a926 = match v2.clone() {
                        Adt2::C0(_, _, h924, _, _, _, _, _, _, _, _) => {
                            h924
                        }
                    };
                    let a927 = match v3.clone() {
                        Adt2::C0(_, _, h925, _, _, _, _, _, _, _, _) => {
                            h925
                        }
                    };
                    if if if nat_eq(a926, a927) {
                        let a930 = match v2.clone() {
                            Adt2::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                                h928
                            }
                        };
                        let a931 = match v3.clone() {
                            Adt2::C0(_, _, _, h929, _, _, _, _, _, _, _) => {
                                h929
                            }
                        };
                        nat_eq(a930, a931)
                    } else {
                        false
                    } {
                        let a934 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h932, _, _, _, _, _, _) => {
                                h932
                            }
                        };
                        let a935 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h933, _, _, _, _, _, _) => {
                                h933
                            }
                        };
                        nat_eq(a934, a935)
                    } else {
                        false
                    } {
                        let a944 = match v3.clone() {
                            Adt2::C0(_, h936, _, _, _, _, _, _, _, _, _) => {
                                h936
                            }
                        };
                        let a945 = {
                            let a942 = {
                                let a939 = match v2.clone() {
                                    Adt2::C0(_, h937, _, _, _, _, _, _, _, _, _) => {
                                        h937
                                    }
                                };
                                let a940 = match v1.clone() {
                                    Adt1::C0(h938, _, _, _, _, _, _, _, _, _) => {
                                        h938
                                    }
                                };
                                nat_add(a939, a940)?
                            };
                            let a943 = match v1.clone() {
                                Adt1::C0(_, h941, _, _, _, _, _, _, _, _) => {
                                    h941
                                }
                            };
                            nat_add(a942, a943)?
                        };
                        nat_eq(a944, a945)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h946, _, _, _, _, _, _, _, _, _) => {
                            h946
                        }
                    })))
                } else {
                    Ok(Adt8::C1)
                }
            } else {
                let a947 = v0;
                let a948 = 63u64;
                if nat_eq(a947, a948) {
                    if if if if if f83(match v2.clone() {
                        Adt2::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                            h949
                        }
                    }, match v1.clone() {
                        Adt1::C0(h950, _, _, _, _, _, _, _, _, _) => {
                            h950
                        }
                    })? {
                        f83(match v2.clone() {
                            Adt2::C0(_, _, h951, _, _, _, _, _, _, _, _) => {
                                h951
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h952, _, _, _, _, _, _, _, _) => {
                                h952
                            }
                        })?
                    } else {
                        false
                    } {
                        f83(match v2.clone() {
                            Adt2::C0(_, _, _, h953, _, _, _, _, _, _, _) => {
                                h953
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h954, _, _, _, _, _, _, _) => {
                                h954
                            }
                        })?
                    } else {
                        false
                    } {
                        f83(match v2.clone() {
                            Adt2::C0(_, _, _, _, h955, _, _, _, _, _, _) => {
                                h955
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h956, _, _, _, _, _, _) => {
                                h956
                            }
                        })?
                    } else {
                        false
                    } {
                        let a959 = match v2.clone() {
                            Adt2::C0(_, h957, _, _, _, _, _, _, _, _, _) => {
                                h957
                            }
                        };
                        let a960 = match v3.clone() {
                            Adt2::C0(_, h958, _, _, _, _, _, _, _, _, _) => {
                                h958
                            }
                        };
                        if if if nat_eq(a959, a960) {
                            let a963 = match v2.clone() {
                                Adt2::C0(_, _, h961, _, _, _, _, _, _, _, _) => {
                                    h961
                                }
                            };
                            let a964 = match v3.clone() {
                                Adt2::C0(_, _, h962, _, _, _, _, _, _, _, _) => {
                                    h962
                                }
                            };
                            nat_eq(a963, a964)
                        } else {
                            false
                        } {
                            let a967 = match v2.clone() {
                                Adt2::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                                    h965
                                }
                            };
                            let a968 = match v3.clone() {
                                Adt2::C0(_, _, _, h966, _, _, _, _, _, _, _) => {
                                    h966
                                }
                            };
                            nat_eq(a967, a968)
                        } else {
                            false
                        } {
                            let a971 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h969, _, _, _, _, _, _) => {
                                    h969
                                }
                            };
                            let a972 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h970, _, _, _, _, _, _) => {
                                    h970
                                }
                            };
                            nat_eq(a971, a972)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h973, _, _, _, _, _, _, _, _, _) => {
                                h973
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h974, _, _, _, _, _, _, _, _) => {
                                h974
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h975, _, _, _, _, _, _, _) => {
                                h975
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h976, _, _, _, _, _, _) => {
                                h976
                            }
                        })))
                    } else {
                        Ok(Adt8::C1)
                    }
                } else {
                    Ok(Adt8::C0)
                }
            }
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f41(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let m977 = v3;
    match m977 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a978 = v0;
                let a979 = 4u64;
                nat_eq(a978, a979)
            };
            let v7: bool = {
                let a981 = 0u64;
                let a982 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h980, _, _, _, _, _) => {
                        h980
                    }
                };
                nat_lt(a981, a982)
            };
            let v8: u64 = f10(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h983, _, _, _, _, _, _, _, _, _) => {
                    h983
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h984, _, _, _, _, _, _, _, _) => {
                    h984
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h985, _, _, _, _, _, _, _) => {
                    h985
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h986, _, _, _, _, _, _) => {
                    h986
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h987, _, _, _, _, _, _, _, _, _) => {
                    h987
                }
            }, match v1.clone() {
                Adt1::C0(_, h988, _, _, _, _, _, _, _, _) => {
                    h988
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h989, _, _, _, _, _, _, _) => {
                    h989
                }
            });
            if if if if if if if if if if if if v6 {
                let a991 = match v2.clone() {
                    Adt2::C0(h990, _, _, _, _, _, _, _, _, _, _) => {
                        h990
                    }
                };
                let a992 = 0u64;
                if nat_eq(a991, a992) {
                    let a994 = match v5.clone() {
                        Adt2::C0(h993, _, _, _, _, _, _, _, _, _, _) => {
                            h993
                        }
                    };
                    let a995 = 0u64;
                    nat_eq(a994, a995)
                } else {
                    false
                }
            } else {
                let a997 = match v2.clone() {
                    Adt2::C0(h996, _, _, _, _, _, _, _, _, _, _) => {
                        h996
                    }
                };
                let a998 = 0u64;
                if if nat_eq(a997, a998) {
                    true
                } else {
                    let a1000 = match v2.clone() {
                        Adt2::C0(h999, _, _, _, _, _, _, _, _, _, _) => {
                            h999
                        }
                    };
                    let a1001 = 26u64;
                    nat_eq(a1000, a1001)
                } {
                    let a1003 = match v5.clone() {
                        Adt2::C0(h1002, _, _, _, _, _, _, _, _, _, _) => {
                            h1002
                        }
                    };
                    let a1004 = 0u64;
                    if nat_eq(a1003, a1004) {
                        true
                    } else {
                        let a1006 = match v5.clone() {
                            Adt2::C0(h1005, _, _, _, _, _, _, _, _, _, _) => {
                                h1005
                            }
                        };
                        let a1007 = 26u64;
                        nat_eq(a1006, a1007)
                    }
                } else {
                    false
                }
            } {
                let a1010 = match v2.clone() {
                    Adt2::C0(h1008, _, _, _, _, _, _, _, _, _, _) => {
                        h1008
                    }
                };
                let a1011 = match v4.clone() {
                    Adt2::C0(h1009, _, _, _, _, _, _, _, _, _, _) => {
                        h1009
                    }
                };
                nat_eq(a1010, a1011)
            } else {
                false
            } {
                f15(v2.clone(), v4.clone())
            } else {
                false
            } {
                f16(v2.clone())?
            } else {
                false
            } {
                f16(v4.clone())?
            } else {
                false
            } {
                let a1013 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1012, _, _, _, _, _) => {
                        h1012
                    }
                };
                let a1014 = 4u64;
                nat_eq(a1013, a1014)
            } else {
                false
            } {
                let a1015 = v8;
                let a1016 = f10(v2.clone(), 1u64)?;
                nat_le(a1015, a1016)
            } else {
                false
            } {
                let a1018 = match v1.clone() {
                    Adt1::C0(_, _, _, h1017, _, _, _, _, _, _) => {
                        h1017
                    }
                };
                let a1019 = 1073741824u64;
                nat_lt(a1018, a1019)
            } else {
                false
            } {
                let a1021 = match v1.clone() {
                    Adt1::C0(h1020, _, _, _, _, _, _, _, _, _) => {
                        h1020
                    }
                };
                let a1022 = 4294967295u64;
                if if if if nat_le(a1021, a1022) {
                    let a1024 = match v1.clone() {
                        Adt1::C0(_, h1023, _, _, _, _, _, _, _, _) => {
                            h1023
                        }
                    };
                    let a1025 = 4294967295u64;
                    nat_le(a1024, a1025)
                } else {
                    false
                } {
                    let a1027 = match v1.clone() {
                        Adt1::C0(_, _, h1026, _, _, _, _, _, _, _) => {
                            h1026
                        }
                    };
                    let a1028 = 4294967295u64;
                    nat_le(a1027, a1028)
                } else {
                    false
                } {
                    let a1030 = match v1.clone() {
                        Adt1::C0(_, _, _, h1029, _, _, _, _, _, _) => {
                            h1029
                        }
                    };
                    let a1031 = 4294967295u64;
                    nat_le(a1030, a1031)
                } else {
                    false
                } {
                    let a1033 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1032, _, _, _, _, _) => {
                            h1032
                        }
                    };
                    let a1034 = 4294967295u64;
                    nat_le(a1033, a1034)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1037 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1035) => {
                            h1035
                        }
                    };
                    let a1038 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1036) => {
                            h1036
                        }
                    };
                    nat_eq(a1037, a1038)
                } else {
                    true
                }
            } else {
                false
            } {
                f84(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1039, _, _, _, _, _, _) => {
                        h1039
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1040, _) => {
                        h1040
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f10(v2, 1u64)?
                };
                Ok(Adt8::C4({
                    let a1041 = v10;
                    let a1042 = v8;
                    nat_add(a1041, a1042)?
                }, 4u64, f14(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1043, _, _, _, _, _, _) => {
                        h1043
                    }
                }, v6)))
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f42(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a1045 = match v0.clone() {
        Adt2::C0(h1044, _, _, _, _, _, _, _, _, _, _) => {
            h1044
        }
    };
    let a1046 = 0u64;
    if if if nat_eq(a1045, a1046) {
        let a1048 = match v1.clone() {
            Adt2::C0(h1047, _, _, _, _, _, _, _, _, _, _) => {
                h1047
            }
        };
        let a1049 = 0u64;
        nat_eq(a1048, a1049)
    } else {
        false
    } {
        f80(match v0.clone() {
            Adt2::C0(_, h1050, _, _, _, _, _, _, _, _, _) => {
                h1050
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1051, _, _, _, _, _, _, _, _) => {
                h1051
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1052, _, _, _, _, _, _, _) => {
                h1052
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1053, _, _, _, _, _, _) => {
                h1053
            }
        }, match v1.clone() {
            Adt2::C0(_, h1054, _, _, _, _, _, _, _, _, _) => {
                h1054
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1055, _, _, _, _, _, _, _, _) => {
                h1055
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1056, _, _, _, _, _, _, _) => {
                h1056
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1057, _, _, _, _, _, _) => {
                h1057
            }
        })
    } else {
        false
    } {
        let v2: u64 = f10(v1.clone(), 1u64)?;
        Ok(Adt8::C4({
            let a1058 = v2;
            let a1059 = f10(v0.clone(), 1u64)?;
            nat_add(a1058, a1059)?
        }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f43(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let a1061 = match v1.clone() {
        Adt2::C0(h1060, _, _, _, _, _, _, _, _, _, _) => {
            h1060
        }
    };
    let a1062 = 0u64;
    if if if nat_eq(a1061, a1062) {
        true
    } else {
        let a1064 = match v1.clone() {
            Adt2::C0(h1063, _, _, _, _, _, _, _, _, _, _) => {
                h1063
            }
        };
        let a1065 = 1u64;
        nat_eq(a1064, a1065)
    } {
        true
    } else {
        let a1067 = match v1.clone() {
            Adt2::C0(h1066, _, _, _, _, _, _, _, _, _, _) => {
                h1066
            }
        };
        let a1068 = 30u64;
        nat_eq(a1067, a1068)
    } {
        let a1071 = match v1.clone() {
            Adt2::C0(h1069, _, _, _, _, _, _, _, _, _, _) => {
                h1069
            }
        };
        let a1072 = match v2.clone() {
            Adt2::C0(h1070, _, _, _, _, _, _, _, _, _, _) => {
                h1070
            }
        };
        if if if if if nat_eq(a1071, a1072) {
            let a1075 = match v1.clone() {
                Adt2::C0(_, h1073, _, _, _, _, _, _, _, _, _) => {
                    h1073
                }
            };
            let a1076 = match v2.clone() {
                Adt2::C0(_, h1074, _, _, _, _, _, _, _, _, _) => {
                    h1074
                }
            };
            nat_eq(a1075, a1076)
        } else {
            false
        } {
            let a1079 = match v1.clone() {
                Adt2::C0(_, _, _, h1077, _, _, _, _, _, _, _) => {
                    h1077
                }
            };
            let a1080 = match v2.clone() {
                Adt2::C0(_, _, _, h1078, _, _, _, _, _, _, _) => {
                    h1078
                }
            };
            nat_eq(a1079, a1080)
        } else {
            false
        } {
            let a1083 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1081, _, _, _, _, _, _) => {
                    h1081
                }
            };
            let a1084 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1082, _, _, _, _, _, _) => {
                    h1082
                }
            };
            nat_eq(a1083, a1084)
        } else {
            false
        } {
            let a1085 = v0;
            let a1086 = 4u64;
            if nat_eq(a1085, a1086) {
                let a1088 = match v1.clone() {
                    Adt2::C0(_, _, h1087, _, _, _, _, _, _, _, _) => {
                        h1087
                    }
                };
                let a1089 = 1u64;
                if nat_eq(a1088, a1089) {
                    let a1092 = match v1.clone() {
                        Adt2::C0(_, h1090, _, _, _, _, _, _, _, _, _) => {
                            h1090
                        }
                    };
                    let a1093 = match v2.clone() {
                        Adt2::C0(_, _, h1091, _, _, _, _, _, _, _, _) => {
                            h1091
                        }
                    };
                    nat_eq(a1092, a1093)
                } else {
                    false
                }
            } else {
                let a1094 = v0;
                let a1095 = 4u64;
                if if nat_lt(a1094, a1095) {
                    let a1098 = match v1.clone() {
                        Adt2::C0(_, h1096, _, _, _, _, _, _, _, _, _) => {
                            h1096
                        }
                    };
                    let a1099 = match v1.clone() {
                        Adt2::C0(_, _, h1097, _, _, _, _, _, _, _, _) => {
                            h1097
                        }
                    };
                    nat_eq(a1098, a1099)
                } else {
                    false
                } {
                    let a1102 = match v1.clone() {
                        Adt2::C0(_, _, h1100, _, _, _, _, _, _, _, _) => {
                            h1100
                        }
                    };
                    let a1103 = match v2.clone() {
                        Adt2::C0(_, _, h1101, _, _, _, _, _, _, _, _) => {
                            h1101
                        }
                    };
                    nat_eq(a1102, a1103)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1105 = f79(match v1.clone() {
                Adt2::C0(h1104, _, _, _, _, _, _, _, _, _, _) => {
                    h1104
                }
            });
            match m1105 {
                Adt12::C0 => {
                    Ok(Adt8::C0)
                }
                Adt12::C1(v3) => {
                    Ok(Adt8::C4(f10(v2.clone(), 1u64)?, match v3 {
                        Adt13::C0(_, _, h1106) => {
                            h1106
                        }
                    }, f14(v1, 1u64), f14(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f44(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    if if if f85(match v1.clone() {
        Adt2::C0(h1107, _, _, _, _, _, _, _, _, _, _) => {
            h1107
        }
    }, v0) {
        let a1110 = match v1.clone() {
            Adt2::C0(h1108, _, _, _, _, _, _, _, _, _, _) => {
                h1108
            }
        };
        let a1111 = match v2.clone() {
            Adt2::C0(h1109, _, _, _, _, _, _, _, _, _, _) => {
                h1109
            }
        };
        nat_eq(a1110, a1111)
    } else {
        false
    } {
        let a1114 = match v1.clone() {
            Adt2::C0(_, h1112, _, _, _, _, _, _, _, _, _) => {
                h1112
            }
        };
        let a1115 = match v2.clone() {
            Adt2::C0(_, h1113, _, _, _, _, _, _, _, _, _) => {
                h1113
            }
        };
        if if if nat_eq(a1114, a1115) {
            let a1118 = match v1.clone() {
                Adt2::C0(_, _, h1116, _, _, _, _, _, _, _, _) => {
                    h1116
                }
            };
            let a1119 = match v2.clone() {
                Adt2::C0(_, _, h1117, _, _, _, _, _, _, _, _) => {
                    h1117
                }
            };
            nat_eq(a1118, a1119)
        } else {
            false
        } {
            let a1122 = match v1.clone() {
                Adt2::C0(_, _, _, h1120, _, _, _, _, _, _, _) => {
                    h1120
                }
            };
            let a1123 = match v2.clone() {
                Adt2::C0(_, _, _, h1121, _, _, _, _, _, _, _) => {
                    h1121
                }
            };
            nat_eq(a1122, a1123)
        } else {
            false
        } {
            let a1126 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1124, _, _, _, _, _, _) => {
                    h1124
                }
            };
            let a1127 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1125, _, _, _, _, _, _) => {
                    h1125
                }
            };
            nat_eq(a1126, a1127)
        } else {
            false
        }
    } else {
        false
    } {
        let m1129 = f79(match v1.clone() {
            Adt2::C0(h1128, _, _, _, _, _, _, _, _, _, _) => {
                h1128
            }
        });
        match m1129 {
            Adt12::C0 => {
                Ok(Adt8::C0)
            }
            Adt12::C1(v3) => {
                Ok(Adt8::C4(f10(v2.clone(), 1u64)?, match v3 {
                    Adt13::C0(_, _, h1130) => {
                        h1130
                    }
                }, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1131, _, _, _, _, _, _, _, _, _, _) => {
                        h1131
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f45(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt8> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1132, _, _, _, _, _, _, _, _, _) => {
                h1132
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1133, _, _, _, _, _, _, _) => {
                h1133
            }
        }
    };
    let a1134 = 0u64;
    let a1135 = v4;
    if if nat_lt(a1134, a1135) {
        let a1136 = v4;
        let a1137 = 2147483648u64;
        nat_lt(a1136, a1137)
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
        let v7: u64 = f86(match v5.clone() {
            Adt2::C0(_, _, h1138, _, _, _, _, _, _, _, _) => {
                h1138
            }
        }, v4)?;
        let v8: u64 = f86(match v5.clone() {
            Adt2::C0(_, _, _, h1139, _, _, _, _, _, _, _) => {
                h1139
            }
        }, v4)?;
        let a1141 = match v5.clone() {
            Adt2::C0(h1140, _, _, _, _, _, _, _, _, _, _) => {
                h1140
            }
        };
        let a1142 = 0u64;
        if if if if if nat_eq(a1141, a1142) {
            let a1144 = match v6.clone() {
                Adt2::C0(h1143, _, _, _, _, _, _, _, _, _, _) => {
                    h1143
                }
            };
            let a1145 = 0u64;
            nat_eq(a1144, a1145)
        } else {
            false
        } {
            let a1147 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1146, _, _, _, _, _, _) => {
                    h1146
                }
            };
            let a1148 = 1u64;
            nat_eq(a1147, a1148)
        } else {
            false
        } {
            let a1151 = match v5.clone() {
                Adt2::C0(_, h1149, _, _, _, _, _, _, _, _, _) => {
                    h1149
                }
            };
            let a1152 = match v6.clone() {
                Adt2::C0(_, h1150, _, _, _, _, _, _, _, _, _) => {
                    h1150
                }
            };
            nat_eq(a1151, a1152)
        } else {
            false
        } {
            if v0 {
                let a1154 = match v5.clone() {
                    Adt2::C0(_, _, h1153, _, _, _, _, _, _, _, _) => {
                        h1153
                    }
                };
                let a1155 = 2147483648u64;
                if if if nat_lt(a1154, a1155) {
                    let a1157 = match v5.clone() {
                        Adt2::C0(_, _, _, h1156, _, _, _, _, _, _, _) => {
                            h1156
                        }
                    };
                    let a1158 = 2147483648u64;
                    nat_lt(a1157, a1158)
                } else {
                    false
                } {
                    let a1161 = {
                        let a1159 = v7;
                        let a1160 = v8;
                        nat_mul(a1159, a1160)?
                    };
                    let a1162 = 2147483648u64;
                    nat_le(a1161, a1162)
                } else {
                    false
                } {
                    f87(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1164 = match v1.clone() {
                    Adt1::C0(h1163, _, _, _, _, _, _, _, _, _) => {
                        h1163
                    }
                };
                let a1165 = v7;
                if if if if nat_eq(a1164, a1165) {
                    let a1167 = match v1 {
                        Adt1::C0(_, h1166, _, _, _, _, _, _, _, _) => {
                            h1166
                        }
                    };
                    let a1168 = v8;
                    nat_eq(a1167, a1168)
                } else {
                    false
                } {
                    f88(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1170 = match v6.clone() {
                        Adt2::C0(_, _, h1169, _, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    let a1171 = v4;
                    if nat_eq(a1170, a1171) {
                        let a1173 = match v6.clone() {
                            Adt2::C0(_, _, _, h1172, _, _, _, _, _, _, _) => {
                                h1172
                            }
                        };
                        let a1174 = v4;
                        nat_eq(a1173, a1174)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1178 = match v6 {
                        Adt2::C0(_, _, _, _, h1175, _, _, _, _, _, _) => {
                            h1175
                        }
                    };
                    let a1179 = {
                        let a1176 = v7;
                        let a1177 = v8;
                        nat_mul(a1176, a1177)?
                    };
                    nat_eq(a1178, a1179)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f46(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt8> {
    let a1181 = match v2.clone() {
        Adt2::C0(h1180, _, _, _, _, _, _, _, _, _, _) => {
            h1180
        }
    };
    let a1182 = 0u64;
    if if if if if nat_eq(a1181, a1182) {
        true
    } else {
        let a1184 = match v2.clone() {
            Adt2::C0(h1183, _, _, _, _, _, _, _, _, _, _) => {
                h1183
            }
        };
        let a1185 = 1u64;
        nat_eq(a1184, a1185)
    } {
        true
    } else {
        let a1187 = match v2.clone() {
            Adt2::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
                h1186
            }
        };
        let a1188 = 30u64;
        nat_eq(a1187, a1188)
    } {
        let a1191 = match v2.clone() {
            Adt2::C0(h1189, _, _, _, _, _, _, _, _, _, _) => {
                h1189
            }
        };
        let a1192 = match v3.clone() {
            Adt2::C0(h1190, _, _, _, _, _, _, _, _, _, _) => {
                h1190
            }
        };
        nat_eq(a1191, a1192)
    } else {
        false
    } {
        if if f15(v2.clone(), v3.clone()) {
            let a1194 = match v1.clone() {
                Adt1::C0(h1193, _, _, _, _, _, _, _, _, _) => {
                    h1193
                }
            };
            let a1195 = 4294967296u64;
            nat_lt(a1194, a1195)
        } else {
            false
        } {
            let a1197 = match v1.clone() {
                Adt1::C0(_, h1196, _, _, _, _, _, _, _, _) => {
                    h1196
                }
            };
            let a1198 = 4294967296u64;
            nat_lt(a1197, a1198)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt8::C4(f10(v3.clone(), 1u64)?, {
            let a1200 = match v2.clone() {
                Adt2::C0(h1199, _, _, _, _, _, _, _, _, _, _) => {
                    h1199
                }
            };
            let a1201 = 0u64;
            if nat_eq(a1200, a1201) {
                4u64
            } else {
                2u64
            }
        }, f14(v2.clone(), 1u64), f14(v3, 1u64), Adt4::C13({
            let a1202 = v0;
            let a1203 = 31u64;
            if nat_eq(a1202, a1203) {
                0u64
            } else {
                let a1204 = v0;
                let a1205 = 47u64;
                if nat_eq(a1204, a1205) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                h1206
            }
        }, match v1.clone() {
            Adt1::C0(h1207, _, _, _, _, _, _, _, _, _) => {
                h1207
            }
        }, match v1 {
            Adt1::C0(_, h1208, _, _, _, _, _, _, _, _) => {
                h1208
            }
        })))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a1210 = match v0.clone() {
        Adt2::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
            h1209
        }
    };
    let a1211 = 0u64;
    if if if nat_eq(a1210, a1211) {
        let a1213 = match v1.clone() {
            Adt2::C0(h1212, _, _, _, _, _, _, _, _, _, _) => {
                h1212
            }
        };
        let a1214 = 0u64;
        nat_eq(a1213, a1214)
    } else {
        false
    } {
        f15(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt8::C4(f10(v1.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f48(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a1216 = match v0.clone() {
        Adt2::C0(h1215, _, _, _, _, _, _, _, _, _, _) => {
            h1215
        }
    };
    let a1217 = 0u64;
    if if nat_eq(a1216, a1217) {
        let a1219 = match v1.clone() {
            Adt2::C0(h1218, _, _, _, _, _, _, _, _, _, _) => {
                h1218
            }
        };
        let a1220 = 26u64;
        nat_eq(a1219, a1220)
    } else {
        false
    } {
        let a1222 = match v0.clone() {
            Adt2::C0(_, _, _, h1221, _, _, _, _, _, _, _) => {
                h1221
            }
        };
        let a1223 = 1u64;
        if if if if if if if nat_eq(a1222, a1223) {
            let a1225 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1224, _, _, _, _, _, _) => {
                    h1224
                }
            };
            let a1226 = 1u64;
            nat_eq(a1225, a1226)
        } else {
            false
        } {
            let a1228 = match v0.clone() {
                Adt2::C0(_, h1227, _, _, _, _, _, _, _, _, _) => {
                    h1227
                }
            };
            let a1229 = 2147483647u64;
            nat_le(a1228, a1229)
        } else {
            false
        } {
            let a1232 = match v1.clone() {
                Adt2::C0(_, h1230, _, _, _, _, _, _, _, _, _) => {
                    h1230
                }
            };
            let a1233 = match v0.clone() {
                Adt2::C0(_, _, h1231, _, _, _, _, _, _, _, _) => {
                    h1231
                }
            };
            nat_eq(a1232, a1233)
        } else {
            false
        } {
            let a1235 = match v1.clone() {
                Adt2::C0(_, _, h1234, _, _, _, _, _, _, _, _) => {
                    h1234
                }
            };
            let a1236 = 1u64;
            nat_eq(a1235, a1236)
        } else {
            false
        } {
            let a1238 = match v1.clone() {
                Adt2::C0(_, _, _, h1237, _, _, _, _, _, _, _) => {
                    h1237
                }
            };
            let a1239 = 1u64;
            nat_eq(a1238, a1239)
        } else {
            false
        } {
            let a1241 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1240, _, _, _, _, _, _) => {
                    h1240
                }
            };
            let a1242 = 1u64;
            nat_eq(a1241, a1242)
        } else {
            false
        } {
            Ok(Adt8::C4({
                let a1245 = match v0.clone() {
                    Adt2::C0(_, _, h1243, _, _, _, _, _, _, _, _) => {
                        h1243
                    }
                };
                let a1246 = f89(match v0.clone() {
                    Adt2::C0(_, h1244, _, _, _, _, _, _, _, _, _) => {
                        h1244
                    }
                })?;
                nat_mul(a1245, a1246)?
            }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f49(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt8> {
    let m1247 = v1;
    match m1247 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v3) => {
            let v4: u64 = f90(f14(v0.clone(), 1u64))?;
            let a1249 = match v0.clone() {
                Adt2::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
                    h1248
                }
            };
            let a1250 = 26u64;
            if if if nat_eq(a1249, a1250) {
                let a1252 = match v3.clone() {
                    Adt2::C0(h1251, _, _, _, _, _, _, _, _, _, _) => {
                        h1251
                    }
                };
                let a1253 = 26u64;
                nat_eq(a1252, a1253)
            } else {
                false
            } {
                let a1255 = match v2.clone() {
                    Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                        h1254
                    }
                };
                let a1256 = 27u64;
                nat_eq(a1255, a1256)
            } else {
                false
            } {
                if if if if if if f15(v0.clone(), v3.clone()) {
                    let a1258 = match v2.clone() {
                        Adt2::C0(_, h1257, _, _, _, _, _, _, _, _, _) => {
                            h1257
                        }
                    };
                    let a1259 = 1u64;
                    nat_eq(a1258, a1259)
                } else {
                    false
                } {
                    let a1261 = match v2.clone() {
                        Adt2::C0(_, _, h1260, _, _, _, _, _, _, _, _) => {
                            h1260
                        }
                    };
                    let a1262 = 1u64;
                    nat_eq(a1261, a1262)
                } else {
                    false
                } {
                    let a1264 = match v2.clone() {
                        Adt2::C0(_, _, _, h1263, _, _, _, _, _, _, _) => {
                            h1263
                        }
                    };
                    let a1265 = 1u64;
                    nat_eq(a1264, a1265)
                } else {
                    false
                } {
                    let a1267 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1266, _, _, _, _, _, _) => {
                            h1266
                        }
                    };
                    let a1268 = 1u64;
                    nat_eq(a1267, a1268)
                } else {
                    false
                } {
                    let a1269 = v4;
                    let a1270 = 9223372036854775808u64;
                    nat_lt(a1269, a1270)
                } else {
                    false
                } {
                    Ok(Adt8::C4({
                        let a1273 = 2u64;
                        let a1274 = {
                            let a1271 = v4;
                            let a1272 = 2u64;
                            nat_mul(a1271, a1272)?
                        };
                        nat_add(a1273, a1274)?
                    }, 8u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C18(v4, f14(v3, 1u64))))
                } else {
                    Ok(Adt8::C1)
                }
            } else {
                Ok(Adt8::C0)
            }
        }
    }
}

pub fn f50(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1275, _, _, _, _, _, _, _, _, _, _) => {
            h1275
        }
    };
    let a1276 = v3;
    let a1277 = 0u64;
    if if nat_eq(a1276, a1277) {
        true
    } else {
        let a1278 = v0;
        let a1279 = 13u64;
        if nat_eq(a1278, a1279) {
            let a1280 = v3;
            let a1281 = 1u64;
            if nat_eq(a1280, a1281) {
                true
            } else {
                let a1282 = v3;
                let a1283 = 30u64;
                nat_eq(a1282, a1283)
            }
        } else {
            false
        }
    } {
        let a1285 = v3;
        let a1286 = match v2.clone() {
            Adt2::C0(h1284, _, _, _, _, _, _, _, _, _, _) => {
                h1284
            }
        };
        if if nat_eq(a1285, a1286) {
            let a1287 = v0;
            let a1288 = 13u64;
            if nat_eq(a1287, a1288) {
                let a1290 = match v2.clone() {
                    Adt2::C0(_, h1289, _, _, _, _, _, _, _, _, _) => {
                        h1289
                    }
                };
                let a1291 = 1u64;
                if if if nat_eq(a1290, a1291) {
                    let a1293 = match v2.clone() {
                        Adt2::C0(_, _, h1292, _, _, _, _, _, _, _, _) => {
                            h1292
                        }
                    };
                    let a1294 = 1u64;
                    nat_eq(a1293, a1294)
                } else {
                    false
                } {
                    let a1296 = match v2.clone() {
                        Adt2::C0(_, _, _, h1295, _, _, _, _, _, _, _) => {
                            h1295
                        }
                    };
                    let a1297 = 1u64;
                    nat_eq(a1296, a1297)
                } else {
                    false
                } {
                    let a1299 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1298, _, _, _, _, _, _) => {
                            h1298
                        }
                    };
                    let a1300 = 1u64;
                    nat_eq(a1299, a1300)
                } else {
                    false
                }
            } else {
                let a1302 = match v2.clone() {
                    Adt2::C0(_, h1301, _, _, _, _, _, _, _, _, _) => {
                        h1301
                    }
                };
                let a1303 = 1u64;
                if if if nat_eq(a1302, a1303) {
                    let a1306 = match v1.clone() {
                        Adt2::C0(_, _, h1304, _, _, _, _, _, _, _, _) => {
                            h1304
                        }
                    };
                    let a1307 = match v2.clone() {
                        Adt2::C0(_, _, h1305, _, _, _, _, _, _, _, _) => {
                            h1305
                        }
                    };
                    nat_eq(a1306, a1307)
                } else {
                    false
                } {
                    let a1310 = match v1.clone() {
                        Adt2::C0(_, _, _, h1308, _, _, _, _, _, _, _) => {
                            h1308
                        }
                    };
                    let a1311 = match v2.clone() {
                        Adt2::C0(_, _, _, h1309, _, _, _, _, _, _, _) => {
                            h1309
                        }
                    };
                    nat_eq(a1310, a1311)
                } else {
                    false
                } {
                    let a1314 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1312, _, _, _, _, _, _) => {
                            h1312
                        }
                    };
                    let a1315 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1313, _, _, _, _, _, _) => {
                            h1313
                        }
                    };
                    nat_eq(a1314, a1315)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f14(v1, 1u64);
            Ok(Adt8::C4(f91(v4.clone(), v0, v3)?, {
                let a1316 = v3;
                let a1317 = 0u64;
                if nat_eq(a1316, a1317) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f14(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f51(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1318 = v0;
    let a1319 = 89u64;
    if if nat_eq(a1318, a1319) {
        let a1320 = v1;
        let a1321 = 4u64;
        nat_eq(a1320, a1321)
    } else {
        false
    } {
        Ok(Some(19u64))
    } else {
        let a1322 = 10u64;
        let a1323 = v0;
        if if nat_le(a1322, a1323) {
            let a1324 = v0;
            let a1325 = 12u64;
            nat_le(a1324, a1325)
        } else {
            false
        } {
            Ok(Some({
                let a1326 = v0;
                let a1327 = 3u64;
                nat_add(a1326, a1327)?
            }))
        } else {
            let a1328 = v0;
            let a1329 = 89u64;
            if if nat_eq(a1328, a1329) {
                let a1330 = v1;
                let a1331 = 7u64;
                nat_eq(a1330, a1331)
            } else {
                false
            } {
                Ok(Some(17u64))
            } else {
                let a1332 = v0;
                let a1333 = 89u64;
                if if nat_eq(a1332, a1333) {
                    let a1334 = v1;
                    let a1335 = 10u64;
                    nat_eq(a1334, a1335)
                } else {
                    false
                } {
                    Ok(Some(18u64))
                } else {
                    let a1336 = v0;
                    let a1337 = 89u64;
                    if if nat_eq(a1336, a1337) {
                        let a1338 = v1;
                        let a1339 = 13u64;
                        nat_eq(a1338, a1339)
                    } else {
                        false
                    } {
                        Ok(Some(16u64))
                    } else {
                        let a1340 = v0;
                        let a1341 = 8u64;
                        if nat_eq(a1340, a1341) {
                            Ok(Some(5u64))
                        } else {
                            let a1342 = v0;
                            let a1343 = 9u64;
                            if nat_eq(a1342, a1343) {
                                Ok(Some(6u64))
                            } else {
                                let a1344 = v0;
                                let a1345 = 89u64;
                                if nat_eq(a1344, a1345) {
                                    let a1346 = v1;
                                    let a1347 = 0u64;
                                    if nat_eq(a1346, a1347) {
                                        Ok(Some(0u64))
                                    } else {
                                        let a1348 = v1;
                                        let a1349 = 1u64;
                                        if nat_eq(a1348, a1349) {
                                            Ok(Some(1u64))
                                        } else {
                                            let a1350 = v1;
                                            let a1351 = 2u64;
                                            if nat_eq(a1350, a1351) {
                                                Ok(Some(2u64))
                                            } else {
                                                let a1352 = v1;
                                                let a1353 = 3u64;
                                                if nat_eq(a1352, a1353) {
                                                    Ok(Some(3u64))
                                                } else {
                                                    let a1354 = v1;
                                                    let a1355 = 6u64;
                                                    if nat_eq(a1354, a1355) {
                                                        Ok(Some(4u64))
                                                    } else {
                                                        let a1356 = v1;
                                                        let a1357 = 12u64;
                                                        if nat_eq(a1356, a1357) {
                                                            Ok(Some(7u64))
                                                        } else {
                                                            let a1358 = v1;
                                                            let a1359 = 11u64;
                                                            if nat_eq(a1358, a1359) {
                                                                Ok(Some(8u64))
                                                            } else {
                                                                let a1360 = v1;
                                                                let a1361 = 18u64;
                                                                if nat_eq(a1360, a1361) {
                                                                    Ok(Some(9u64))
                                                                } else {
                                                                    let a1362 = v1;
                                                                    let a1363 = 19u64;
                                                                    if nat_eq(a1362, a1363) {
                                                                        Ok(Some(10u64))
                                                                    } else {
                                                                        let a1364 = v1;
                                                                        let a1365 = 20u64;
                                                                        if nat_eq(a1364, a1365) {
                                                                            Ok(Some(11u64))
                                                                        } else {
                                                                            let a1366 = v1;
                                                                            let a1367 = 21u64;
                                                                            if nat_eq(a1366, a1367) {
                                                                                Ok(Some(12u64))
                                                                            } else {
                                                                                Ok(None::<u64>)
                                                                            }
                                                                        }
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
                                    Ok(None::<u64>)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f52(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let m1368 = f92(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1368 {
        Adt8::C0 => {
            Ok(Adt8::C0)
        }
        Adt8::C1 => {
            let a1369 = v0;
            let a1370 = 2u64;
            if if if if nat_eq(a1369, a1370) {
                true
            } else {
                let a1371 = v0;
                let a1372 = 5u64;
                nat_eq(a1371, a1372)
            } {
                true
            } else {
                let a1373 = v0;
                let a1374 = 6u64;
                nat_eq(a1373, a1374)
            } {
                true
            } else {
                let a1375 = v0;
                let a1376 = 7u64;
                nat_eq(a1375, a1376)
            } {
                Ok(Adt8::C5(67u64))
            } else {
                Ok(Adt8::C1)
            }
        }
        Adt8::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt8::C4(v12, 4u64, f14(v2.clone(), 1u64), f14(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1377 = v3;
                    match m1377 {
                        None => {
                            f14(v2, 1u64)
                        }
                        Some(v13) => {
                            f14(v13, 1u64)
                        }
                    }
                })))
            } else {
                Ok(Adt8::C5(67u64))
            }
        }
        Adt8::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) => {
            Ok(Adt8::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23))
        }
        Adt8::C4(v24, v25, v26, v27, v28) => {
            Ok(Adt8::C4(v24, v25, v26, v27, v28))
        }
        Adt8::C5(v5) => {
            Ok(Adt8::C5(v5))
        }
    }
}

pub fn f53(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let a1379 = match v1.clone() {
        Adt2::C0(h1378, _, _, _, _, _, _, _, _, _, _) => {
            h1378
        }
    };
    let a1380 = 0u64;
    if if if if if nat_eq(a1379, a1380) {
        true
    } else {
        let a1382 = match v1.clone() {
            Adt2::C0(h1381, _, _, _, _, _, _, _, _, _, _) => {
                h1381
            }
        };
        let a1383 = 1u64;
        nat_eq(a1382, a1383)
    } {
        true
    } else {
        let a1385 = match v1.clone() {
            Adt2::C0(h1384, _, _, _, _, _, _, _, _, _, _) => {
                h1384
            }
        };
        let a1386 = 30u64;
        nat_eq(a1385, a1386)
    } {
        let a1389 = match v1.clone() {
            Adt2::C0(h1387, _, _, _, _, _, _, _, _, _, _) => {
                h1387
            }
        };
        let a1390 = match v2.clone() {
            Adt2::C0(h1388, _, _, _, _, _, _, _, _, _, _) => {
                h1388
            }
        };
        if nat_eq(a1389, a1390) {
            true
        } else {
            let a1392 = match v2.clone() {
                Adt2::C0(h1391, _, _, _, _, _, _, _, _, _, _) => {
                    h1391
                }
            };
            let a1393 = 0u64;
            nat_eq(a1392, a1393)
        }
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt8::C4(f10(v2.clone(), 1u64)?, {
            let a1395 = match v2.clone() {
                Adt2::C0(h1394, _, _, _, _, _, _, _, _, _, _) => {
                    h1394
                }
            };
            let a1396 = 0u64;
            if nat_eq(a1395, a1396) {
                4u64
            } else {
                2u64
            }
        }, f14(v1.clone(), 1u64), f14(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1397, _, _, _, _, _, _, _, _, _, _) => {
                h1397
            }
        }, match v2 {
            Adt2::C0(h1398, _, _, _, _, _, _, _, _, _, _) => {
                h1398
            }
        })))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f54(v0: Adt4, v1: Adt7) -> Adt7 {
    let m1399 = v1;
    match m1399 {
        Adt7::C0(v2) => {
            Adt7::C0(v2)
        }
        Adt7::C1(v3, v4, v5) => {
            Adt7::C1(v3, v4, v5)
        }
        Adt7::C2(v6, v7, v8, v9, v10, v11) => {
            Adt7::C2(v6, v7, v8, v9, v10, v11)
        }
        Adt7::C3(v12, v13, v14, v15) => {
            f93(f94(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f95(v0), 3u64, v15)
        }
        Adt7::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f96(v0.clone(), v16);
            f93(f97(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
        }
        Adt7::C5(v24, v25, v26, v27, v28, v29) => {
            Adt7::C5(v24, v25, v26, v27, v28, v29)
        }
        Adt7::C6(v30, v31, v32, v33) => {
            Adt7::C6(v30, v31, v32, v33)
        }
        Adt7::C7(v34, v35, v36, v37, v38, v39) => {
            Adt7::C7(v34, v35, v36, v37, v38, v39)
        }
    }
}

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt7> {
    let m1400 = v3;
    match m1400 {
        Adt4::C0 => {
            Ok(Adt7::C1(false, f98(v0, v1)?, f98(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt7::C1(false, f100(v0, v1, v2.clone())?, f98(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f101(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f102(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f103(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f102(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f104(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f105(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f106(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt7::C1(true, 0u64, f98(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f107(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt7::C3(v28, v29, f98(v0, v1)?, f98(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f108(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt7::C3(v34, v34, f98(v0, v1)?, f98(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt7::C3(v38, v39, f98(v0, v1)?, f98(v0, v2)?))
        }
        Adt4::C15 => {
            f109(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f110(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C18(v42, v43) => {
            f111(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C21 => {
            Ok(Adt7::C1(false, f98(f99(v0, match v2.clone() {
                Adt3::C0(h1401, _, _, _, _, _, _, _) => {
                    h1401
                }
            }, match v2.clone() {
                Adt3::C0(_, h1402, _, _, _, _, _, _) => {
                    h1402
                }
            })?, v1)?, f98(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt7::C0(Adt8::C5(65u64)))
        }
        Adt4::C25(v48, v49, v50) => {
            f112(v0, v48, v49, v50)
        }
        Adt4::C26(v52, _, _, _) => {
            let a1403 = v52;
            let a1404 = 64u64;
            if if if nat_eq(a1403, a1404) {
                true
            } else {
                let a1405 = v52;
                let a1406 = 65u64;
                nat_eq(a1405, a1406)
            } {
                true
            } else {
                let a1407 = v52;
                let a1408 = 66u64;
                nat_eq(a1407, a1408)
            } {
                Ok(Adt7::C0(Adt8::C5(64u64)))
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
        Adt4::C27(v51) => {
            f60(v0, v1, v2, v51)
        }
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let v4: u64 = f89(match v1.clone() {
        Adt3::C0(h1409, _, _, _, _, _, _, _) => {
            h1409
        }
    })?;
    let v5: u64 = {
        let a1410 = v0;
        let a1411 = v4;
        let a1412 = 0u64;
        nat_quot(a1410, a1411, a1412)
    };
    let v6: u64 = {
        let a1413 = v0;
        let a1414 = v4;
        let a1415 = 0u64;
        nat_rem(a1413, a1414, a1415)
    };
    let a1416 = v6;
    let a1417 = 0u64;
    if nat_eq(a1416, a1417) {
        Ok(Adt7::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1418 = v6;
        let a1419 = 1u64;
        if nat_eq(a1418, a1419) {
            Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1422 = {
                let a1420 = v6;
                let a1421 = 1u64;
                nat_add(a1420, a1421)?
            };
            let a1423 = v4;
            if nat_eq(a1422, a1423) {
                Ok(Adt7::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f98(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1424 = v6;
                    let a1425 = 2u64;
                    nat_sub(a1424, a1425)
                };
                let v8: u64 = {
                    let a1426 = v7;
                    let a1427 = 3u64;
                    let a1428 = 0u64;
                    nat_quot(a1426, a1427, a1428)
                };
                let v9: u64 = {
                    let a1429 = v7;
                    let a1430 = 3u64;
                    let a1431 = 0u64;
                    nat_rem(a1429, a1430, a1431)
                };
                let v10: u64 = f98({
                    let a1435 = {
                        let a1433 = v5;
                        let a1434 = match v1.clone() {
                            Adt3::C0(h1432, _, _, _, _, _, _, _) => {
                                h1432
                            }
                        };
                        nat_mul(a1433, a1434)?
                    };
                    let a1436 = v8;
                    nat_add(a1435, a1436)?
                }, v1)?;
                let a1437 = v9;
                let a1438 = 0u64;
                if nat_eq(a1437, a1438) {
                    Ok(Adt7::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1439 = v9;
                    let a1440 = 1u64;
                    if nat_eq(a1439, a1440) {
                        let a1441 = v3;
                        let a1442 = 2u64;
                        if nat_eq(a1441, a1442) {
                            Ok(Adt7::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt7::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1443 = v3;
                        let a1444 = 2u64;
                        if if nat_eq(a1443, a1444) {
                            true
                        } else {
                            let a1445 = v3;
                            let a1446 = 3u64;
                            nat_eq(a1445, a1446)
                        } {
                            Ok(Adt7::C4(19u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt7::C4(19u64, 2u64, v8, 2u64, 0u64, 1u64, 1u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt7> {
    let v5: u64 = {
        let a1448 = 2u64;
        let a1449 = match v1.clone() {
            Adt3::C0(_, h1447, _, _, _, _, _, _) => {
                h1447
            }
        };
        nat_mul(a1448, a1449)?
    };
    let a1450 = v0;
    let a1451 = v5;
    if nat_lt(a1450, a1451) {
        let a1455 = {
            let a1452 = v0;
            let a1453 = 2u64;
            let a1454 = 0u64;
            nat_rem(a1452, a1453, a1454)
        };
        let a1456 = 0u64;
        if nat_eq(a1455, a1456) {
            Ok(Adt7::C4(19u64, 4u64, f98({
                let a1457 = v0;
                let a1458 = 2u64;
                let a1459 = 0u64;
                nat_quot(a1457, a1458, a1459)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1460 = v4;
            let a1461 = 2147483648u64;
            if if nat_lt(a1460, a1461) {
                let a1463 = v4;
                let a1464 = match v2 {
                    Adt3::C0(_, h1462, _, _, _, _, _, _) => {
                        h1462
                    }
                };
                nat_lt(a1463, a1464)
            } else {
                false
            } {
                Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
    } else {
        let v6: u64 = {
            let a1467 = match v2.clone() {
                Adt3::C0(h1465, _, _, _, _, _, _, _) => {
                    h1465
                }
            };
            let a1468 = match v2.clone() {
                Adt3::C0(_, h1466, _, _, _, _, _, _) => {
                    h1466
                }
            };
            nat_mul(a1467, a1468)?
        };
        let a1471 = {
            let a1469 = v0;
            let a1470 = v5;
            nat_sub(a1469, a1470)
        };
        let a1472 = v6;
        if nat_lt(a1471, a1472) {
            Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f98({
                let a1473 = v0;
                let a1474 = v5;
                nat_sub(a1473, a1474)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1477 = {
                    let a1475 = v0;
                    let a1476 = v5;
                    nat_sub(a1475, a1476)
                };
                let a1478 = v6;
                nat_sub(a1477, a1478)
            };
            let v8: u64 = {
                let a1482 = 1u64;
                let a1483 = {
                    let a1480 = 2u64;
                    let a1481 = match v1.clone() {
                        Adt3::C0(h1479, _, _, _, _, _, _, _) => {
                            h1479
                        }
                    };
                    nat_mul(a1480, a1481)?
                };
                nat_add(a1482, a1483)?
            };
            let v9: u64 = {
                let a1484 = v7;
                let a1485 = v8;
                let a1486 = 0u64;
                nat_quot(a1484, a1485, a1486)
            };
            let v10: u64 = {
                let a1487 = v7;
                let a1488 = v8;
                let a1489 = 0u64;
                nat_rem(a1487, a1488, a1489)
            };
            let a1490 = v10;
            let a1491 = 0u64;
            if nat_eq(a1490, a1491) {
                Ok(Adt7::C4(19u64, 4u64, f98(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1495 = {
                    let a1492 = v10;
                    let a1493 = 2u64;
                    let a1494 = 0u64;
                    nat_rem(a1492, a1493, a1494)
                };
                let a1496 = 1u64;
                if nat_eq(a1495, a1496) {
                    Ok(Adt7::C4(6u64, 0u64, f98({
                        let a1505 = {
                            let a1498 = v9;
                            let a1499 = match v1.clone() {
                                Adt3::C0(h1497, _, _, _, _, _, _, _) => {
                                    h1497
                                }
                            };
                            nat_mul(a1498, a1499)?
                        };
                        let a1506 = {
                            let a1502 = {
                                let a1500 = v10;
                                let a1501 = 1u64;
                                nat_sub(a1500, a1501)
                            };
                            let a1503 = 2u64;
                            let a1504 = 0u64;
                            nat_quot(a1502, a1503, a1504)
                        };
                        nat_add(a1505, a1506)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt7::C4(4u64, 3u64, f98({
                        let a1515 = {
                            let a1508 = v4;
                            let a1509 = match v1.clone() {
                                Adt3::C0(h1507, _, _, _, _, _, _, _) => {
                                    h1507
                                }
                            };
                            nat_mul(a1508, a1509)?
                        };
                        let a1516 = {
                            let a1512 = {
                                let a1510 = v10;
                                let a1511 = 1u64;
                                nat_sub(a1510, a1511)
                            };
                            let a1513 = 2u64;
                            let a1514 = 0u64;
                            nat_quot(a1512, a1513, a1514)
                        };
                        nat_add(a1515, a1516)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f98({
                        let a1525 = {
                            let a1518 = v4;
                            let a1519 = match v1 {
                                Adt3::C0(h1517, _, _, _, _, _, _, _) => {
                                    h1517
                                }
                            };
                            nat_mul(a1518, a1519)?
                        };
                        let a1526 = {
                            let a1522 = {
                                let a1520 = v10;
                                let a1521 = 1u64;
                                nat_sub(a1520, a1521)
                            };
                            let a1523 = 2u64;
                            let a1524 = 0u64;
                            nat_quot(a1522, a1523, a1524)
                        };
                        nat_add(a1525, a1526)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f58(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt7> {
    let a1527 = v1;
    let a1528 = f113(v2.clone(), v4.clone())?;
    if nat_lt(a1527, a1528) {
        let a1531 = v1;
        let a1532 = {
            let a1529 = 2u64;
            let a1530 = f90(v4.clone())?;
            nat_mul(a1529, a1530)?
        };
        if nat_lt(a1531, a1532) {
            let a1536 = {
                let a1533 = v1;
                let a1534 = 2u64;
                let a1535 = 0u64;
                nat_rem(a1533, a1534, a1535)
            };
            let a1537 = 0u64;
            if nat_eq(a1536, a1537) {
                f114({
                    let a1538 = v1;
                    let a1539 = 2u64;
                    let a1540 = 0u64;
                    nat_quot(a1538, a1539, a1540)
                }, v4)
            } else {
                if f115(match v2 {
                    Adt3::C0(_, h1541, _, _, _, _, _, _) => {
                        h1541
                    }
                }, v5) {
                    Ok(Adt7::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt7::C0(Adt8::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1544 = v1;
                let a1545 = {
                    let a1542 = 2u64;
                    let a1543 = f90(v4.clone())?;
                    nat_mul(a1542, a1543)?
                };
                nat_sub(a1544, a1545)
            };
            let v7: u64 = {
                let a1549 = v6;
                let a1550 = {
                    let a1547 = 1u64;
                    let a1548 = match v2.clone() {
                        Adt3::C0(h1546, _, _, _, _, _, _, _) => {
                            h1546
                        }
                    };
                    nat_add(a1547, a1548)?
                };
                let a1551 = 0u64;
                nat_quot(a1549, a1550, a1551)
            };
            let v8: u64 = {
                let a1555 = v6;
                let a1556 = {
                    let a1553 = 1u64;
                    let a1554 = match v2.clone() {
                        Adt3::C0(h1552, _, _, _, _, _, _, _) => {
                            h1552
                        }
                    };
                    nat_add(a1553, a1554)?
                };
                let a1557 = 0u64;
                nat_rem(a1555, a1556, a1557)
            };
            let a1558 = v8;
            let a1559 = 0u64;
            if nat_eq(a1558, a1559) {
                f114(v7, v4)
            } else {
                f116(v0, v7, {
                    let a1560 = v8;
                    let a1561 = 1u64;
                    nat_sub(a1560, a1561)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt7> {
    let v6: u64 = {
        let a1564 = match v4.clone() {
            Adt3::C0(h1562, _, _, _, _, _, _, _) => {
                h1562
            }
        };
        let a1565 = match v4.clone() {
            Adt3::C0(_, h1563, _, _, _, _, _, _) => {
                h1563
            }
        };
        nat_mul(a1564, a1565)?
    };
    let v7: u64 = {
        let a1566 = 2u64;
        let a1567 = v6;
        nat_mul(a1566, a1567)?
    };
    let v8: u64 = f117(match v3.clone() {
        Adt3::C0(_, h1568, _, _, _, _, _, _) => {
            h1568
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1569, _, _, _, _, _) => {
            h1569
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1570, _, _, _, _) => {
            h1570
        }
    });
    let a1571 = v0;
    let a1572 = v7;
    if nat_lt(a1571, a1572) {
        let a1576 = {
            let a1573 = v0;
            let a1574 = 2u64;
            let a1575 = 0u64;
            nat_rem(a1573, a1574, a1575)
        };
        let a1577 = 0u64;
        if nat_eq(a1576, a1577) {
            Ok(Adt7::C4(19u64, 5u64, f98({
                let a1578 = v0;
                let a1579 = 2u64;
                let a1580 = 0u64;
                nat_quot(a1578, a1579, a1580)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f118(v5, v8) {
                Ok(Adt7::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1581 = v0;
            let a1582 = v7;
            nat_sub(a1581, a1582)
        };
        let v10: u64 = {
            let a1586 = v9;
            let a1587 = {
                let a1584 = 1u64;
                let a1585 = match v1.clone() {
                    Adt3::C0(h1583, _, _, _, _, _, _, _) => {
                        h1583
                    }
                };
                nat_add(a1584, a1585)?
            };
            let a1588 = 0u64;
            nat_quot(a1586, a1587, a1588)
        };
        let v11: u64 = {
            let a1592 = v9;
            let a1593 = {
                let a1590 = 1u64;
                let a1591 = match v1.clone() {
                    Adt3::C0(h1589, _, _, _, _, _, _, _) => {
                        h1589
                    }
                };
                nat_add(a1590, a1591)?
            };
            let a1594 = 0u64;
            nat_rem(a1592, a1593, a1594)
        };
        let a1595 = v11;
        let a1596 = 0u64;
        if nat_eq(a1595, a1596) {
            Ok(Adt7::C4(19u64, 5u64, f98({
                let a1597 = v10;
                let a1598 = v6;
                let a1599 = 0u64;
                nat_rem(a1597, a1598, a1599)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f118(v5, v8) {
                let v12: u64 = {
                    let a1605 = {
                        let a1601 = v10;
                        let a1602 = match v1.clone() {
                            Adt3::C0(h1600, _, _, _, _, _, _, _) => {
                                h1600
                            }
                        };
                        nat_mul(a1601, a1602)?
                    };
                    let a1606 = {
                        let a1603 = v11;
                        let a1604 = 1u64;
                        nat_sub(a1603, a1604)
                    };
                    nat_add(a1605, a1606)?
                };
                let a1607 = v5;
                let a1608 = 4294967295u64;
                if nat_eq(a1607, a1608) {
                    Ok(Adt7::C1(false, f98(v12, v1)?, f98(v12, v2)?))
                } else {
                    Ok(Adt7::C4(8u64, 0u64, f98(v12, v1)?, 4u64, f98({
                        let a1614 = {
                            let a1610 = v5;
                            let a1611 = match v3.clone() {
                                Adt3::C0(h1609, _, _, _, _, _, _, _) => {
                                    h1609
                                }
                            };
                            nat_mul(a1610, a1611)?
                        };
                        let a1615 = {
                            let a1612 = v11;
                            let a1613 = 1u64;
                            nat_sub(a1612, a1613)
                        };
                        nat_add(a1614, a1615)?
                    }, v3)?, 3u64, f98(v12, v2)?))
                }
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
    }
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt7> {
    let v4: u64 = f119(v0, v2.clone(), 0u64);
    let v5: u64 = f119(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a1617 = v5;
        let a1618 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1616, _) => {
                h1616
            }
        };
        let a1619 = 0u64;
        nat_rem(a1617, a1618, a1619)
    };
    let v7: u64 = {
        let a1621 = v5;
        let a1622 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1620, _) => {
                h1620
            }
        };
        let a1623 = 0u64;
        nat_quot(a1621, a1622, a1623)
    };
    let v8: Option<u64> = f120(match v3.clone() {
        Adt5::C0(h1624, _, _, _, _, _, _, _) => {
            h1624
        }
    }, match v1.clone() {
        Adt3::C0(h1625, _, _, _, _, _, _, _) => {
            h1625
        }
    }, f119(v0, v2.clone(), 1u64), {
        let a1629 = v4;
        let a1630 = {
            let m1628 = match v3.clone() {
                Adt5::C0(h1627, _, _, _, _, _, _, _) => {
                    h1627
                }
            };
            match m1628 {
                Adt6::C0(h1626, _, _, _) => {
                    h1626
                }
            }
        };
        let a1631 = 0u64;
        nat_rem(a1629, a1630, a1631)
    })?;
    let v9: u64 = {
        let a1635 = v4;
        let a1636 = {
            let m1634 = match v3.clone() {
                Adt5::C0(h1633, _, _, _, _, _, _, _) => {
                    h1633
                }
            };
            match m1634 {
                Adt6::C0(h1632, _, _, _) => {
                    h1632
                }
            }
        };
        let a1637 = 0u64;
        nat_quot(a1635, a1636, a1637)
    };
    let v10: Option<u64> = f120(match v3.clone() {
        Adt5::C0(_, h1638, _, _, _, _, _, _) => {
            h1638
        }
    }, match v1.clone() {
        Adt3::C0(_, h1639, _, _, _, _, _, _) => {
            h1639
        }
    }, f119(v0, v2.clone(), 2u64), {
        let a1643 = v9;
        let a1644 = {
            let m1642 = match v3.clone() {
                Adt5::C0(_, h1641, _, _, _, _, _, _) => {
                    h1641
                }
            };
            match m1642 {
                Adt6::C0(h1640, _, _, _) => {
                    h1640
                }
            }
        };
        let a1645 = 0u64;
        nat_rem(a1643, a1644, a1645)
    })?;
    let v11: u64 = {
        let a1649 = v9;
        let a1650 = {
            let m1648 = match v3.clone() {
                Adt5::C0(_, h1647, _, _, _, _, _, _) => {
                    h1647
                }
            };
            match m1648 {
                Adt6::C0(h1646, _, _, _) => {
                    h1646
                }
            }
        };
        let a1651 = 0u64;
        nat_quot(a1649, a1650, a1651)
    };
    let v12: Option<u64> = f120(match v3.clone() {
        Adt5::C0(_, _, h1652, _, _, _, _, _) => {
            h1652
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1653, _, _, _, _, _) => {
            h1653
        }
    }, v6, {
        let a1657 = v11;
        let a1658 = {
            let m1656 = match v3.clone() {
                Adt5::C0(_, _, h1655, _, _, _, _, _) => {
                    h1655
                }
            };
            match m1656 {
                Adt6::C0(h1654, _, _, _) => {
                    h1654
                }
            }
        };
        let a1659 = 0u64;
        nat_rem(a1657, a1658, a1659)
    })?;
    let v13: u64 = {
        let a1663 = v11;
        let a1664 = {
            let m1662 = match v3.clone() {
                Adt5::C0(_, _, h1661, _, _, _, _, _) => {
                    h1661
                }
            };
            match m1662 {
                Adt6::C0(h1660, _, _, _) => {
                    h1660
                }
            }
        };
        let a1665 = 0u64;
        nat_quot(a1663, a1664, a1665)
    };
    let v17: Adt10 = {
        let m1666 = v8;
        match m1666 {
            None => {
                Adt10::C2(0u64)
            }
            Some(v14) => {
                let m1667 = v10;
                match m1667 {
                    None => {
                        Adt10::C2(0u64)
                    }
                    Some(v15) => {
                        let m1668 = v12;
                        match m1668 {
                            None => {
                                Adt10::C2(0u64)
                            }
                            Some(v16) => {
                                Adt10::C0(1u64, {
                                    let a1677 = {
                                        let a1672 = f121(v1, v14, v15, v16, 0u64)?;
                                        let a1673 = {
                                            let a1670 = v7;
                                            let a1671 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h1669, _, _, _) => {
                                                    h1669
                                                }
                                            };
                                            nat_mul(a1670, a1671)?
                                        };
                                        nat_add(a1672, a1673)?
                                    };
                                    let a1678 = {
                                        let a1675 = v13;
                                        let a1676 = match v3.clone() {
                                            Adt5::C0(_, _, _, h1674, _, _, _, _) => {
                                                h1674
                                            }
                                        };
                                        nat_mul(a1675, a1676)?
                                    };
                                    nat_add(a1677, a1678)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt7::C7(Adt9::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h1679, _, _) => {
            h1679
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h1680) => {
            h1680
        }
    }, true), v17, Adt10::C2(0u64), Adt11::C1(f98(v0, v2)?), {
        let a1682 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h1681, _, _) => {
                h1681
            }
        };
        let a1683 = 0u64;
        if nat_eq(a1682, a1683) {
            4u64
        } else {
            2u64
        }
    }, {
        let a1685 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h1684) => {
                h1684
            }
        };
        let a1686 = 0u64;
        if nat_eq(a1685, a1686) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt7> {
    let v8: u64 = {
        let a1689 = v6;
        let a1690 = {
            let a1687 = 3u64;
            let a1688 = v5;
            nat_add(a1687, a1688)?
        };
        nat_sub(a1689, a1690)
    };
    let a1692 = v0;
    let a1693 = f26(v3, match v1.clone() {
        Adt3::C0(h1691, _, _, _, _, _, _, _) => {
            h1691
        }
    }, v5, v8)?;
    if nat_lt(a1692, a1693) {
        let v9: u64 = f70(match v1.clone() {
            Adt3::C0(h1694, _, _, _, _, _, _, _) => {
                h1694
            }
        }, v5, v8)?;
        let v10: u64 = f71(v3, match v1.clone() {
            Adt3::C0(h1695, _, _, _, _, _, _, _) => {
                h1695
            }
        }, v5)?;
        let a1696 = v0;
        let a1697 = v9;
        if nat_lt(a1696, a1697) {
            f122(v0, v2, f20(v3), {
                let a1698 = 3u64;
                let a1699 = v5;
                nat_add(a1698, a1699)?
            }, v7)
        } else {
            let a1702 = v0;
            let a1703 = {
                let a1700 = v9;
                let a1701 = v10;
                nat_add(a1700, a1701)?
            };
            if nat_lt(a1702, a1703) {
                let v11: u64 = {
                    let a1704 = v0;
                    let a1705 = v9;
                    nat_sub(a1704, a1705)
                };
                let v12: u64 = f123(v3, match v1.clone() {
                    Adt3::C0(h1706, _, _, _, _, _, _, _) => {
                        h1706
                    }
                })?;
                f124({
                    let a1707 = v11;
                    let a1708 = v12;
                    let a1709 = 0u64;
                    nat_rem(a1707, a1708, a1709)
                }, {
                    let a1710 = v11;
                    let a1711 = v12;
                    let a1712 = 0u64;
                    nat_quot(a1710, a1711, a1712)
                }, v1, v3, v4, v7)
            } else {
                f125({
                    let a1715 = v0;
                    let a1716 = {
                        let a1713 = v9;
                        let a1714 = v10;
                        nat_add(a1713, a1714)?
                    };
                    nat_sub(a1715, a1716)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt7> {
    let a1717 = v0;
    let a1718 = f13(v1)?;
    if nat_lt(a1717, a1718) {
        let a1719 = v0;
        let a1720 = 4u64;
        if nat_lt(a1719, a1720) {
            let a1721 = v0;
            let a1722 = 0u64;
            if nat_eq(a1721, a1722) {
                Ok(Adt7::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1723 = v0;
                let a1724 = 1u64;
                if nat_eq(a1723, a1724) {
                    let a1725 = v5;
                    let a1726 = 0u64;
                    if nat_eq(a1725, a1726) {
                        Ok(Adt7::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt7::C0(Adt8::C1))
                    }
                } else {
                    let a1727 = v0;
                    let a1728 = 2u64;
                    if nat_eq(a1727, a1728) {
                        Ok(Adt7::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt7::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1729 = v0;
            let a1730 = 4u64;
            if if nat_eq(a1729, a1730) {
                let a1731 = f126(v5)?;
                let a1732 = v1;
                !nat_eq(a1731, a1732)
            } else {
                false
            } {
                Ok(Adt7::C0(Adt8::C1))
            } else {
                let v6: u64 = {
                    let a1735 = {
                        let a1733 = v0;
                        let a1734 = 4u64;
                        nat_sub(a1733, a1734)
                    };
                    let a1736 = 3u64;
                    let a1737 = 0u64;
                    nat_quot(a1735, a1736, a1737)
                };
                let v7: u64 = {
                    let a1740 = {
                        let a1738 = v0;
                        let a1739 = 4u64;
                        nat_sub(a1738, a1739)
                    };
                    let a1741 = 3u64;
                    let a1742 = 0u64;
                    nat_rem(a1740, a1741, a1742)
                };
                let a1743 = v7;
                let a1744 = 0u64;
                if nat_eq(a1743, a1744) {
                    Ok(Adt7::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1745 = v7;
                    let a1746 = 1u64;
                    if nat_eq(a1745, a1746) {
                        Ok(Adt7::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt7::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f12(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f63(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt9 {
    let a1747 = v1;
    let a1748 = 0u64;
    if nat_eq(a1747, a1748) {
        if v0 {
            Adt9::C2(v3, v4, false)
        } else {
            Adt9::C0(v2)
        }
    } else {
        let a1749 = v1;
        let a1750 = 1u64;
        if nat_eq(a1749, a1750) {
            Adt9::C1(v2)
        } else {
            let a1751 = v1;
            let a1752 = 2u64;
            if nat_eq(a1751, a1752) {
                Adt9::C5(v2)
            } else {
                let a1753 = v1;
                let a1754 = 3u64;
                if nat_eq(a1753, a1754) {
                    Adt9::C3(v2, v2, false)
                } else {
                    let a1755 = v1;
                    let a1756 = 4u64;
                    if nat_eq(a1755, a1756) {
                        Adt9::C2(v3, v4, false)
                    } else {
                        Adt9::C6
                    }
                }
            }
        }
    }
}

pub fn f64(v0: u64, v1: u64) -> Adt10 {
    let a1757 = v0;
    let a1758 = 0u64;
    if nat_eq(a1757, a1758) {
        Adt10::C0(0u64, v1)
    } else {
        let a1759 = v0;
        let a1760 = 1u64;
        if nat_eq(a1759, a1760) {
            Adt10::C1(v1)
        } else {
            let a1761 = v0;
            let a1762 = 2u64;
            if nat_eq(a1761, a1762) {
                Adt10::C2(v1)
            } else {
                let a1763 = v0;
                let a1764 = 3u64;
                if nat_eq(a1763, a1764) {
                    Adt10::C3(v1)
                } else {
                    let a1765 = v0;
                    let a1766 = 13u64;
                    if nat_lt(a1765, a1766) {
                        Adt10::C0({
                            let a1767 = v0;
                            let a1768 = 3u64;
                            nat_sub(a1767, a1768)
                        }, v1)
                    } else {
                        Adt10::C4
                    }
                }
            }
        }
    }
}

pub fn f65(v0: u64, v1: u64) -> Adt11 {
    let a1769 = v0;
    let a1770 = 1u64;
    if nat_eq(a1769, a1770) {
        Adt11::C0(v1)
    } else {
        let a1771 = v0;
        let a1772 = 3u64;
        if if nat_eq(a1771, a1772) {
            true
        } else {
            let a1773 = v0;
            let a1774 = 6u64;
            nat_eq(a1773, a1774)
        } {
            Adt11::C1(v1)
        } else {
            Adt11::C2
        }
    }
}

pub fn f66(v0: u64) -> u64 {
    let v1: u64 = {
        let a1775 = v0;
        let a1776 = 4294967296u64;
        let a1777 = 0u64;
        nat_rem(a1775, a1776, a1777)
    };
    let a1778 = v1;
    let a1779 = 2147483648u64;
    if nat_lt(a1778, a1779) {
        v1
    } else {
        0u64
    }
}

pub fn f67(v0: u64) -> R<u64> {
    let a1782 = 2u64;
    let a1783 = {
        let a1780 = 2u64;
        let a1781 = v0;
        nat_mul(a1780, a1781)?
    };
    nat_add(a1782, a1783)
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f127(v0, v3, v6) {
        f127(v1, v4, v6)
    } else {
        false
    } {
        f127(v2, v5, v6)
    } else {
        false
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f128(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f128(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f128(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f128(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f128(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f128(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1784 = v2;
    let a1785 = 0u64;
    if nat_eq(a1784, a1785) {
        Ok(0u64)
    } else {
        let a1788 = 2u64;
        let a1789 = {
            let a1786 = v0;
            let a1787 = v1;
            nat_mul(a1786, a1787)?
        };
        nat_mul(a1788, a1789)
    }
}

pub fn f71(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1790 = v2;
    let a1791 = f123(v0, v1)?;
    nat_mul(a1790, a1791)
}

pub fn f72(v0: u64) -> u64 {
    let a1792 = v0;
    let a1793 = 30u64;
    if nat_eq(a1792, a1793) {
        3u64
    } else {
        1u64
    }
}

pub fn f73(v0: u64) -> R<u64> {
    let a1796 = 5u64;
    let a1797 = {
        let a1794 = 4u64;
        let a1795 = v0;
        nat_mul(a1794, a1795)?
    };
    nat_add(a1796, a1797)
}

pub fn f74(v0: Adt6, v1: u64) -> R<u64> {
    let a1799 = 0u64;
    let a1800 = match v0.clone() {
        Adt6::C0(h1798, _, _, _) => {
            h1798
        }
    };
    if if if if if nat_lt(a1799, a1800) {
        let a1802 = 0u64;
        let a1803 = match v0.clone() {
            Adt6::C0(_, h1801, _, _) => {
                h1801
            }
        };
        nat_lt(a1802, a1803)
    } else {
        false
    } {
        let a1805 = 0u64;
        let a1806 = match v0.clone() {
            Adt6::C0(_, _, _, h1804) => {
                h1804
            }
        };
        nat_lt(a1805, a1806)
    } else {
        false
    } {
        let a1808 = match v0.clone() {
            Adt6::C0(_, h1807, _, _) => {
                h1807
            }
        };
        let a1809 = 2147483648u64;
        nat_lt(a1808, a1809)
    } else {
        false
    } {
        let a1811 = match v0.clone() {
            Adt6::C0(_, _, _, h1810) => {
                h1810
            }
        };
        let a1812 = 2147483648u64;
        nat_lt(a1811, a1812)
    } else {
        false
    } {
        let v2: u64 = {
            let a1816 = v1;
            let a1817 = {
                let a1814 = 2u64;
                let a1815 = f129(match v0.clone() {
                    Adt6::C0(_, _, h1813, _) => {
                        h1813
                    }
                });
                nat_mul(a1814, a1815)?
            };
            nat_add(a1816, a1817)?
        };
        let v3: u64 = {
            let a1829 = {
                let a1824 = 1u64;
                let a1825 = {
                    let a1822 = match v0.clone() {
                        Adt6::C0(_, _, _, h1818) => {
                            h1818
                        }
                    };
                    let a1823 = {
                        let a1820 = match v0.clone() {
                            Adt6::C0(h1819, _, _, _) => {
                                h1819
                            }
                        };
                        let a1821 = 1u64;
                        nat_sub(a1820, a1821)
                    };
                    nat_mul(a1822, a1823)?
                };
                nat_add(a1824, a1825)?
            };
            let a1830 = {
                let a1827 = 2u64;
                let a1828 = f130(match v0.clone() {
                    Adt6::C0(_, _, h1826, _) => {
                        h1826
                    }
                });
                nat_mul(a1827, a1828)?
            };
            nat_add(a1829, a1830)?
        };
        let a1831 = v3;
        let a1832 = v2;
        if nat_le(a1831, a1832) {
            let a1839 = 1u64;
            let a1840 = {
                let a1836 = {
                    let a1833 = v2;
                    let a1834 = v3;
                    nat_sub(a1833, a1834)
                };
                let a1837 = match v0 {
                    Adt6::C0(_, h1835, _, _) => {
                        h1835
                    }
                };
                let a1838 = 0u64;
                nat_quot(a1836, a1837, a1838)
            };
            nat_add(a1839, a1840)
        } else {
            let a1844 = {
                let a1841 = v3;
                let a1842 = v2;
                nat_sub(a1841, a1842)
            };
            let a1845 = match v0 {
                Adt6::C0(_, h1843, _, _) => {
                    h1843
                }
            };
            if nat_lt(a1844, a1845) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f75(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt8> {
    if f131(match v0.clone() {
        Adt2::C0(h1846, _, _, _, _, _, _, _, _, _, _) => {
            h1846
        }
    }, match v0.clone() {
        Adt2::C0(_, h1847, _, _, _, _, _, _, _, _, _) => {
            h1847
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1848, _, _, _, _, _, _, _, _) => {
            h1848
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1849, _, _, _, _, _, _, _) => {
            h1849
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1850, _, _, _, _, _, _) => {
            h1850
        }
    }, match v1.clone() {
        Adt2::C0(h1851, _, _, _, _, _, _, _, _, _, _) => {
            h1851
        }
    }, match v1.clone() {
        Adt2::C0(_, h1852, _, _, _, _, _, _, _, _, _) => {
            h1852
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1853, _, _, _, _, _, _, _, _) => {
            h1853
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1854, _, _, _, _, _, _, _) => {
            h1854
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1855, _, _, _, _, _, _) => {
            h1855
        }
    }, match v2.clone() {
        Adt2::C0(h1856, _, _, _, _, _, _, _, _, _, _) => {
            h1856
        }
    }, match v2.clone() {
        Adt2::C0(_, h1857, _, _, _, _, _, _, _, _, _) => {
            h1857
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1858, _, _, _, _, _, _, _, _) => {
            h1858
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1859, _, _, _, _, _, _, _) => {
            h1859
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1860, _, _, _, _, _, _) => {
            h1860
        }
    }) {
        let m1862 = f79(match v0.clone() {
            Adt2::C0(h1861, _, _, _, _, _, _, _, _, _, _) => {
                h1861
            }
        });
        match m1862 {
            Adt12::C0 => {
                Ok(Adt8::C0)
            }
            Adt12::C1(v3) => {
                let a1868 = {
                    let a1865 = match v0.clone() {
                        Adt2::C0(_, h1863, _, _, _, _, _, _, _, _, _) => {
                            h1863
                        }
                    };
                    let a1866 = match v3.clone() {
                        Adt13::C0(_, h1864, _) => {
                            h1864
                        }
                    };
                    let a1867 = 0u64;
                    nat_rem(a1865, a1866, a1867)
                };
                let a1869 = 0u64;
                if if nat_eq(a1868, a1869) {
                    if f132(match v0.clone() {
                        Adt2::C0(_, h1870, _, _, _, _, _, _, _, _, _) => {
                            h1870
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1871, _, _, _, _, _, _, _, _, _) => {
                            h1871
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1872, _, _, _, _, _, _, _, _) => {
                            h1872
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1873, _, _, _, _, _, _, _) => {
                            h1873
                        }
                    }) {
                        true
                    } else {
                        let a1875 = 4u64;
                        let a1876 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1874, _, _, _, _, _) => {
                                h1874
                            }
                        };
                        if if nat_le(a1875, a1876) {
                            let a1878 = match v3.clone() {
                                Adt13::C0(_, h1877, _) => {
                                    h1877
                                }
                            };
                            let a1879 = 1u64;
                            if nat_eq(a1878, a1879) {
                                true
                            } else {
                                let a1882 = match v3.clone() {
                                    Adt13::C0(_, _, h1880) => {
                                        h1880
                                    }
                                };
                                let a1883 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1881, _, _, _, _, _) => {
                                        h1881
                                    }
                                };
                                nat_le(a1882, a1883)
                            }
                        } else {
                            false
                        } {
                            if f17(v0.clone(), v2.clone())? {
                                f17(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f14(v0.clone(), match v3 {
                        Adt13::C0(_, h1884, _) => {
                            h1884
                        }
                    });
                    Ok(Adt8::C4(if f132(match v0.clone() {
                        Adt2::C0(_, h1885, _, _, _, _, _, _, _, _, _) => {
                            h1885
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1886, _, _, _, _, _, _, _, _, _) => {
                            h1886
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1887, _, _, _, _, _, _, _, _) => {
                            h1887
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1888, _, _, _, _, _, _, _) => {
                            h1888
                        }
                    }) {
                        0u64
                    } else {
                        f113(v4.clone(), f14(v1.clone(), 1u64))?
                    }, 4u64, v4, f14(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1889, _, _, _, _, _, _, _, _, _, _) => {
                            h1889
                        }
                    }, f14(v1, 1u64))))
                } else {
                    Ok(Adt8::C1)
                }
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f76(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt8> {
    if if if if if if if if if if if if f78(match v0.clone() {
        Adt2::C0(h1890, _, _, _, _, _, _, _, _, _, _) => {
            h1890
        }
    }) {
        let a1892 = match v1.clone() {
            Adt2::C0(h1891, _, _, _, _, _, _, _, _, _, _) => {
                h1891
            }
        };
        let a1893 = 26u64;
        nat_eq(a1892, a1893)
    } else {
        false
    } {
        let a1895 = match v2.clone() {
            Adt2::C0(h1894, _, _, _, _, _, _, _, _, _, _) => {
                h1894
            }
        };
        let a1896 = 0u64;
        nat_eq(a1895, a1896)
    } else {
        false
    } {
        let a1899 = match v0.clone() {
            Adt2::C0(_, h1897, _, _, _, _, _, _, _, _, _) => {
                h1897
            }
        };
        let a1900 = match v2.clone() {
            Adt2::C0(_, h1898, _, _, _, _, _, _, _, _, _) => {
                h1898
            }
        };
        nat_eq(a1899, a1900)
    } else {
        false
    } {
        let a1903 = match v1.clone() {
            Adt2::C0(_, h1901, _, _, _, _, _, _, _, _, _) => {
                h1901
            }
        };
        let a1904 = match v0.clone() {
            Adt2::C0(_, _, h1902, _, _, _, _, _, _, _, _) => {
                h1902
            }
        };
        nat_le(a1903, a1904)
    } else {
        false
    } {
        let a1906 = match v0.clone() {
            Adt2::C0(_, _, _, h1905, _, _, _, _, _, _, _) => {
                h1905
            }
        };
        let a1907 = 1u64;
        nat_eq(a1906, a1907)
    } else {
        false
    } {
        let a1909 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1908, _, _, _, _, _, _) => {
                h1908
            }
        };
        let a1910 = 1u64;
        nat_eq(a1909, a1910)
    } else {
        false
    } {
        let a1912 = match v2.clone() {
            Adt2::C0(_, _, _, h1911, _, _, _, _, _, _, _) => {
                h1911
            }
        };
        let a1913 = 1u64;
        nat_eq(a1912, a1913)
    } else {
        false
    } {
        let a1915 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1914, _, _, _, _, _, _) => {
                h1914
            }
        };
        let a1916 = 1u64;
        nat_eq(a1915, a1916)
    } else {
        false
    } {
        let a1918 = match v1.clone() {
            Adt2::C0(_, _, h1917, _, _, _, _, _, _, _, _) => {
                h1917
            }
        };
        let a1919 = 1u64;
        nat_eq(a1918, a1919)
    } else {
        false
    } {
        let a1921 = match v1.clone() {
            Adt2::C0(_, _, _, h1920, _, _, _, _, _, _, _) => {
                h1920
            }
        };
        let a1922 = 1u64;
        nat_eq(a1921, a1922)
    } else {
        false
    } {
        let a1924 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1923, _, _, _, _, _, _) => {
                h1923
            }
        };
        let a1925 = 1u64;
        nat_eq(a1924, a1925)
    } else {
        false
    } {
        if f24(match v2.clone() {
            Adt2::C0(_, h1926, _, _, _, _, _, _, _, _, _) => {
                h1926
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h1927, _, _, _, _, _, _, _, _) => {
                h1927
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h1928, _, _, _, _, _, _, _) => {
                h1928
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h1929, _, _, _, _, _, _) => {
                h1929
            }
        }) {
            Ok(Adt8::C4(0u64, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a1931 = match v1.clone() {
                Adt2::C0(_, h1930, _, _, _, _, _, _, _, _, _) => {
                    h1930
                }
            };
            let a1932 = 0u64;
            if nat_eq(a1931, a1932) {
                Ok(Adt8::C4(f10(v2.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a1934 = match v0.clone() {
                    Adt2::C0(_, h1933, _, _, _, _, _, _, _, _, _) => {
                        h1933
                    }
                };
                let a1935 = 2147483647u64;
                if if if if if if nat_le(a1934, a1935) {
                    let a1937 = match v1.clone() {
                        Adt2::C0(_, h1936, _, _, _, _, _, _, _, _, _) => {
                            h1936
                        }
                    };
                    let a1938 = 2147483647u64;
                    nat_le(a1937, a1938)
                } else {
                    false
                } {
                    let a1940 = 4u64;
                    let a1941 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h1939, _, _, _, _, _) => {
                            h1939
                        }
                    };
                    nat_le(a1940, a1941)
                } else {
                    false
                } {
                    let a1947 = {
                        let a1944 = match v2.clone() {
                            Adt2::C0(_, h1942, _, _, _, _, _, _, _, _, _) => {
                                h1942
                            }
                        };
                        let a1945 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1943, _, _, _, _, _) => {
                                h1943
                            }
                        };
                        nat_mul(a1944, a1945)?
                    };
                    let a1948 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1946, _, _, _, _) => {
                            h1946
                        }
                    };
                    nat_le(a1947, a1948)
                } else {
                    false
                } {
                    f17(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f17(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt8::C4({
                        let a1962 = {
                            let a1952 = {
                                let a1950 = 2u64;
                                let a1951 = match v1.clone() {
                                    Adt2::C0(_, h1949, _, _, _, _, _, _, _, _, _) => {
                                        h1949
                                    }
                                };
                                nat_mul(a1950, a1951)?
                            };
                            let a1953 = f10(v2.clone(), 1u64)?;
                            nat_add(a1952, a1953)?
                        };
                        let a1963 = {
                            let a1960 = match v1.clone() {
                                Adt2::C0(_, h1954, _, _, _, _, _, _, _, _, _) => {
                                    h1954
                                }
                            };
                            let a1961 = {
                                let a1958 = 1u64;
                                let a1959 = {
                                    let a1956 = 2u64;
                                    let a1957 = match v0.clone() {
                                        Adt2::C0(_, h1955, _, _, _, _, _, _, _, _, _) => {
                                            h1955
                                        }
                                    };
                                    nat_mul(a1956, a1957)?
                                };
                                nat_add(a1958, a1959)?
                            };
                            nat_mul(a1960, a1961)?
                        };
                        nat_add(a1962, a1963)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h1964, _, _, _, _, _, _, _, _, _) => {
                            h1964
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1965, _, _, _, _, _, _, _, _, _) => {
                            h1965
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h1966, _, _, _, _, _) => {
                            h1966
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1967, _, _, _, _) => {
                            h1967
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h1968, _, _, _) => {
                            h1968
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1969, _, _) => {
                            h1969
                        }
                    }), f14(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h1970, _, _, _, _, _, _, _, _, _, _) => {
                            h1970
                        }
                    }, f14(v1, 1u64))))
                } else {
                    Ok(Adt8::C1)
                }
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f77(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt8> {
    let a1973 = match v1.clone() {
        Adt2::C0(h1971, _, _, _, _, _, _, _, _, _, _) => {
            h1971
        }
    };
    let a1974 = match v3.clone() {
        Adt2::C0(h1972, _, _, _, _, _, _, _, _, _, _) => {
            h1972
        }
    };
    if if nat_eq(a1973, a1974) {
        let a1975 = v0;
        let a1976 = 33u64;
        if nat_eq(a1975, a1976) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1978 = f79(match v1.clone() {
            Adt2::C0(h1977, _, _, _, _, _, _, _, _, _, _) => {
                h1977
            }
        });
        match m1978 {
            Adt12::C0 => {
                Ok(Adt8::C0)
            }
            Adt12::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt13::C0(_, h1979, _) => {
                        h1979
                    }
                };
                let v6: u64 = f10(v1.clone(), v5)?;
                let a1984 = {
                    let a1981 = match v1.clone() {
                        Adt2::C0(_, h1980, _, _, _, _, _, _, _, _, _) => {
                            h1980
                        }
                    };
                    let a1982 = v5;
                    let a1983 = 1u64;
                    nat_rem(a1981, a1982, a1983)
                };
                let a1985 = 0u64;
                if if if nat_eq(a1984, a1985) {
                    let a1990 = {
                        let a1987 = match v3.clone() {
                            Adt2::C0(_, h1986, _, _, _, _, _, _, _, _, _) => {
                                h1986
                            }
                        };
                        let a1988 = v5;
                        let a1989 = 1u64;
                        nat_rem(a1987, a1988, a1989)
                    };
                    let a1991 = 0u64;
                    nat_eq(a1990, a1991)
                } else {
                    false
                } {
                    let a1992 = v6;
                    let a1993 = f10(v3.clone(), v5)?;
                    nat_eq(a1992, a1993)
                } else {
                    false
                } {
                    Ok(Adt8::C4(v6, match v4 {
                        Adt13::C0(_, _, h1994) => {
                            h1994
                        }
                    }, f14(v1, v5), f14(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt8::C1)
                }
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f78(v0: u64) -> bool {
    let a1995 = v0;
    let a1996 = 0u64;
    if if nat_eq(a1995, a1996) {
        true
    } else {
        let a1997 = v0;
        let a1998 = 1u64;
        nat_eq(a1997, a1998)
    } {
        true
    } else {
        let a1999 = v0;
        let a2000 = 30u64;
        nat_eq(a1999, a2000)
    }
}

pub fn f79(v0: u64) -> Adt12 {
    let a2001 = v0;
    let a2002 = 0u64;
    if nat_eq(a2001, a2002) {
        Adt12::C1(Adt13::C0(0u64, 1u64, 4u64))
    } else {
        let a2003 = v0;
        let a2004 = 1u64;
        if nat_eq(a2003, a2004) {
            Adt12::C1(Adt13::C0(1u64, 1u64, 2u64))
        } else {
            let a2005 = v0;
            let a2006 = 2u64;
            if nat_eq(a2005, a2006) {
                Adt12::C1(Adt13::C0(2u64, 32u64, 18u64))
            } else {
                let a2007 = v0;
                let a2008 = 3u64;
                if nat_eq(a2007, a2008) {
                    Adt12::C1(Adt13::C0(3u64, 32u64, 20u64))
                } else {
                    let a2009 = v0;
                    let a2010 = 6u64;
                    if nat_eq(a2009, a2010) {
                        Adt12::C1(Adt13::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2011 = v0;
                        let a2012 = 7u64;
                        if nat_eq(a2011, a2012) {
                            Adt12::C1(Adt13::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2013 = v0;
                            let a2014 = 8u64;
                            if nat_eq(a2013, a2014) {
                                Adt12::C1(Adt13::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2015 = v0;
                                let a2016 = 9u64;
                                if nat_eq(a2015, a2016) {
                                    Adt12::C1(Adt13::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2017 = v0;
                                    let a2018 = 10u64;
                                    if nat_eq(a2017, a2018) {
                                        Adt12::C1(Adt13::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2019 = v0;
                                        let a2020 = 11u64;
                                        if nat_eq(a2019, a2020) {
                                            Adt12::C1(Adt13::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2021 = v0;
                                            let a2022 = 12u64;
                                            if nat_eq(a2021, a2022) {
                                                Adt12::C1(Adt13::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2023 = v0;
                                                let a2024 = 13u64;
                                                if nat_eq(a2023, a2024) {
                                                    Adt12::C1(Adt13::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2025 = v0;
                                                    let a2026 = 14u64;
                                                    if nat_eq(a2025, a2026) {
                                                        Adt12::C1(Adt13::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2027 = v0;
                                                        let a2028 = 15u64;
                                                        if nat_eq(a2027, a2028) {
                                                            Adt12::C1(Adt13::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2029 = v0;
                                                            let a2030 = 16u64;
                                                            if nat_eq(a2029, a2030) {
                                                                Adt12::C1(Adt13::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2031 = v0;
                                                                let a2032 = 17u64;
                                                                if nat_eq(a2031, a2032) {
                                                                    Adt12::C1(Adt13::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2033 = v0;
                                                                    let a2034 = 18u64;
                                                                    if nat_eq(a2033, a2034) {
                                                                        Adt12::C1(Adt13::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2035 = v0;
                                                                        let a2036 = 19u64;
                                                                        if nat_eq(a2035, a2036) {
                                                                            Adt12::C1(Adt13::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2037 = v0;
                                                                            let a2038 = 20u64;
                                                                            if nat_eq(a2037, a2038) {
                                                                                Adt12::C1(Adt13::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2039 = v0;
                                                                                let a2040 = 21u64;
                                                                                if nat_eq(a2039, a2040) {
                                                                                    Adt12::C1(Adt13::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2041 = v0;
                                                                                    let a2042 = 22u64;
                                                                                    if nat_eq(a2041, a2042) {
                                                                                        Adt12::C1(Adt13::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2043 = v0;
                                                                                        let a2044 = 23u64;
                                                                                        if nat_eq(a2043, a2044) {
                                                                                            Adt12::C1(Adt13::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2045 = v0;
                                                                                            let a2046 = 24u64;
                                                                                            if nat_eq(a2045, a2046) {
                                                                                                Adt12::C1(Adt13::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2047 = v0;
                                                                                                let a2048 = 25u64;
                                                                                                if nat_eq(a2047, a2048) {
                                                                                                    Adt12::C1(Adt13::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2049 = v0;
                                                                                                    let a2050 = 26u64;
                                                                                                    if nat_eq(a2049, a2050) {
                                                                                                        Adt12::C1(Adt13::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2051 = v0;
                                                                                                        let a2052 = 27u64;
                                                                                                        if nat_eq(a2051, a2052) {
                                                                                                            Adt12::C1(Adt13::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2053 = v0;
                                                                                                            let a2054 = 28u64;
                                                                                                            if nat_eq(a2053, a2054) {
                                                                                                                Adt12::C1(Adt13::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2055 = v0;
                                                                                                                let a2056 = 29u64;
                                                                                                                if nat_eq(a2055, a2056) {
                                                                                                                    Adt12::C1(Adt13::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2057 = v0;
                                                                                                                    let a2058 = 30u64;
                                                                                                                    if nat_eq(a2057, a2058) {
                                                                                                                        Adt12::C1(Adt13::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2059 = v0;
                                                                                                                        let a2060 = 34u64;
                                                                                                                        if nat_eq(a2059, a2060) {
                                                                                                                            Adt12::C1(Adt13::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2061 = v0;
                                                                                                                            let a2062 = 35u64;
                                                                                                                            if nat_eq(a2061, a2062) {
                                                                                                                                Adt12::C1(Adt13::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2063 = v0;
                                                                                                                                let a2064 = 39u64;
                                                                                                                                if nat_eq(a2063, a2064) {
                                                                                                                                    Adt12::C1(Adt13::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2065 = v0;
                                                                                                                                    let a2066 = 40u64;
                                                                                                                                    if nat_eq(a2065, a2066) {
                                                                                                                                        Adt12::C1(Adt13::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2067 = v0;
                                                                                                                                        let a2068 = 41u64;
                                                                                                                                        if nat_eq(a2067, a2068) {
                                                                                                                                            Adt12::C1(Adt13::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2069 = v0;
                                                                                                                                            let a2070 = 42u64;
                                                                                                                                            if nat_eq(a2069, a2070) {
                                                                                                                                                Adt12::C1(Adt13::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2071 = v0;
                                                                                                                                                let a2072 = 43u64;
                                                                                                                                                if nat_eq(a2071, a2072) {
                                                                                                                                                    Adt12::C1(Adt13::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt12::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f24(v4, v5, v6, v7) {
        f24(v0, v1, v2, v3)
    } else {
        let a2076 = {
            let a2073 = v0;
            let a2074 = v4;
            let a2075 = 1u64;
            nat_rem(a2073, a2074, a2075)
        };
        let a2077 = 0u64;
        if if if nat_eq(a2076, a2077) {
            let a2081 = {
                let a2078 = v1;
                let a2079 = v5;
                let a2080 = 1u64;
                nat_rem(a2078, a2079, a2080)
            };
            let a2082 = 0u64;
            nat_eq(a2081, a2082)
        } else {
            false
        } {
            let a2086 = {
                let a2083 = v2;
                let a2084 = v6;
                let a2085 = 1u64;
                nat_rem(a2083, a2084, a2085)
            };
            let a2087 = 0u64;
            nat_eq(a2086, a2087)
        } else {
            false
        } {
            let a2091 = {
                let a2088 = v3;
                let a2089 = v7;
                let a2090 = 1u64;
                nat_rem(a2088, a2089, a2090)
            };
            let a2092 = 0u64;
            nat_eq(a2091, a2092)
        } else {
            false
        }
    }
}

pub fn f81(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2093 = v0;
    let a2094 = v1;
    if nat_eq(a2093, a2094) {
        let a2097 = v4;
        let a2098 = {
            let a2095 = v2;
            let a2096 = v3;
            nat_add(a2095, a2096)?
        };
        Ok(nat_eq(a2097, a2098))
    } else {
        let a2099 = v2;
        let a2100 = v3;
        if nat_eq(a2099, a2100) {
            let a2101 = v4;
            let a2102 = v2;
            Ok(nat_eq(a2101, a2102))
        } else {
            Ok(false)
        }
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2105 = {
            let a2103 = v0;
            let a2104 = f129(v1);
            nat_add(a2103, a2104)?
        };
        let a2106 = f129(v2);
        nat_add(a2105, a2106)?
    };
    let v5: u64 = {
        let a2107 = f130(v1);
        let a2108 = f130(v2);
        nat_add(a2107, a2108)?
    };
    let a2109 = v1;
    let a2110 = 4294967295u64;
    if if nat_le(a2109, a2110) {
        let a2111 = v2;
        let a2112 = 4294967295u64;
        nat_le(a2111, a2112)
    } else {
        false
    } {
        let a2113 = v5;
        let a2114 = v4;
        if nat_le(a2113, a2114) {
            let a2117 = v3;
            let a2118 = {
                let a2115 = v4;
                let a2116 = v5;
                nat_sub(a2115, a2116)
            };
            Ok(nat_eq(a2117, a2118))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f83(v0: u64, v1: u64) -> R<bool> {
    let a2119 = v1;
    let a2120 = 4294967295u64;
    if nat_le(a2119, a2120) {
        let a2123 = {
            let a2121 = f129(v1);
            let a2122 = f130(v1);
            nat_add(a2121, a2122)?
        };
        let a2124 = v0;
        Ok(nat_lt(a2123, a2124))
    } else {
        Ok(false)
    }
}

pub fn f84(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f133(v0.clone(), false)?;
    let v6: u64 = f133(v0, true)?;
    let a2125 = v6;
    let a2126 = v1;
    if if nat_le(a2125, a2126) {
        let a2133 = {
            let a2131 = {
                let a2127 = v1;
                let a2128 = v5;
                nat_add(a2127, a2128)?
            };
            let a2132 = {
                let a2129 = v3;
                let a2130 = 0u64;
                if nat_eq(a2129, a2130) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2131, a2132)?
        };
        let a2134 = v2;
        nat_le(a2133, a2134)
    } else {
        false
    } {
        if v4 {
            let a2139 = {
                let a2137 = {
                    let a2135 = v1;
                    let a2136 = v5;
                    nat_add(a2135, a2136)?
                };
                let a2138 = v6;
                nat_sub(a2137, a2138)
            };
            let a2140 = v2;
            Ok(nat_lt(a2139, a2140))
        } else {
            let a2145 = {
                let a2143 = {
                    let a2141 = v1;
                    let a2142 = v5;
                    nat_add(a2141, a2142)?
                };
                let a2144 = v6;
                nat_sub(a2143, a2144)
            };
            let a2146 = v2;
            Ok(nat_le(a2145, a2146))
        }
    } else {
        Ok(false)
    }
}

pub fn f85(v0: u64, v1: u64) -> bool {
    let a2147 = v0;
    let a2148 = 0u64;
    if if nat_eq(a2147, a2148) {
        true
    } else {
        let a2149 = v0;
        let a2150 = 1u64;
        nat_eq(a2149, a2150)
    } {
        let a2151 = v1;
        let a2152 = 4294967296u64;
        nat_lt(a2151, a2152)
    } else {
        false
    }
}

pub fn f86(v0: u64, v1: u64) -> R<u64> {
    let a2157 = {
        let a2155 = v0;
        let a2156 = {
            let a2153 = v1;
            let a2154 = 1u64;
            nat_sub(a2153, a2154)
        };
        nat_add(a2155, a2156)?
    };
    let a2158 = v1;
    let a2159 = 0u64;
    Ok(nat_quot(a2157, a2158, a2159))
}

pub fn f87(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f24(match v0.clone() {
        Adt2::C0(_, h2160, _, _, _, _, _, _, _, _, _) => {
            h2160
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2161, _, _, _, _, _, _, _, _) => {
            h2161
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2162, _, _, _, _, _, _, _) => {
            h2162
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2163, _, _, _, _, _, _) => {
            h2163
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2166 = match v1.clone() {
                Adt2::C0(_, h2164, _, _, _, _, _, _, _, _, _) => {
                    h2164
                }
            };
            let a2167 = match v1.clone() {
                Adt2::C0(_, _, h2165, _, _, _, _, _, _, _, _) => {
                    h2165
                }
            };
            nat_mul(a2166, a2167)?
        };
        let v5: u64 = {
            let a2169 = v4;
            let a2170 = match v1.clone() {
                Adt2::C0(_, _, _, h2168, _, _, _, _, _, _, _) => {
                    h2168
                }
            };
            nat_mul(a2169, a2170)?
        };
        let a2183 = {
            let a2181 = {
                let a2172 = match v0.clone() {
                    Adt2::C0(_, h2171, _, _, _, _, _, _, _, _, _) => {
                        h2171
                    }
                };
                let a2173 = 1u64;
                nat_sub(a2172, a2173)
            };
            let a2182 = {
                let a2179 = f134(match v0.clone() {
                    Adt2::C0(_, _, h2174, _, _, _, _, _, _, _, _) => {
                        h2174
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2175, _, _, _, _, _, _, _, _, _) => {
                        h2175
                    }
                })?;
                let a2180 = f134(match v0 {
                    Adt2::C0(_, _, _, h2176, _, _, _, _, _, _, _) => {
                        h2176
                    }
                }, v2, {
                    let a2177 = v3;
                    let a2178 = v5;
                    nat_mul(a2177, a2178)?
                }, v4)?;
                nat_add(a2179, a2180)?
            };
            nat_add(a2181, a2182)?
        };
        let a2184 = f10(v1, 1u64)?;
        Ok(nat_lt(a2183, a2184))
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2185 = v0;
    let a2186 = 2147483648u64;
    if if if if nat_lt(a2185, a2186) {
        let a2187 = v1;
        let a2188 = 2147483648u64;
        nat_lt(a2187, a2188)
    } else {
        false
    } {
        let a2191 = {
            let a2189 = v0;
            let a2190 = v1;
            nat_mul(a2189, a2190)?
        };
        let a2192 = 2147483648u64;
        nat_lt(a2191, a2192)
    } else {
        false
    } {
        let a2197 = {
            let a2195 = {
                let a2193 = v0;
                let a2194 = 1u64;
                nat_sub(a2193, a2194)
            };
            let a2196 = v2;
            nat_mul(a2195, a2196)?
        };
        let a2198 = 2147483648u64;
        nat_lt(a2197, a2198)
    } else {
        false
    } {
        let a2203 = {
            let a2201 = {
                let a2199 = v1;
                let a2200 = 1u64;
                nat_sub(a2199, a2200)
            };
            let a2202 = v2;
            nat_mul(a2201, a2202)?
        };
        let a2204 = 2147483648u64;
        Ok(nat_lt(a2203, a2204))
    } else {
        Ok(false)
    }
}

pub fn f89(v0: u64) -> R<u64> {
    let a2207 = 3u64;
    let a2208 = {
        let a2205 = v0;
        let a2206 = 3u64;
        nat_mul(a2205, a2206)?
    };
    nat_add(a2207, a2208)
}

pub fn f90(v0: Adt3) -> R<u64> {
    if f24(match v0.clone() {
        Adt3::C0(h2209, _, _, _, _, _, _, _) => {
            h2209
        }
    }, match v0.clone() {
        Adt3::C0(_, h2210, _, _, _, _, _, _) => {
            h2210
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h2211, _, _, _, _, _) => {
            h2211
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h2212, _, _, _, _) => {
            h2212
        }
    }) {
        Ok(0u64)
    } else {
        let a2221 = match v0.clone() {
            Adt3::C0(h2213, _, _, _, _, _, _, _) => {
                h2213
            }
        };
        let a2222 = {
            let a2219 = match v0.clone() {
                Adt3::C0(_, h2214, _, _, _, _, _, _) => {
                    h2214
                }
            };
            let a2220 = {
                let a2217 = match v0.clone() {
                    Adt3::C0(_, _, h2215, _, _, _, _, _) => {
                        h2215
                    }
                };
                let a2218 = match v0 {
                    Adt3::C0(_, _, _, h2216, _, _, _, _) => {
                        h2216
                    }
                };
                nat_mul(a2217, a2218)?
            };
            nat_mul(a2219, a2220)?
        };
        nat_mul(a2221, a2222)
    }
}

pub fn f91(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2223 = v1;
    let a2224 = 13u64;
    if if nat_eq(a2223, a2224) {
        f24(match v0.clone() {
            Adt3::C0(h2225, _, _, _, _, _, _, _) => {
                h2225
            }
        }, match v0.clone() {
            Adt3::C0(_, h2226, _, _, _, _, _, _) => {
                h2226
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2227, _, _, _, _, _) => {
                h2227
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2228, _, _, _, _) => {
                h2228
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2234 = match v0.clone() {
                Adt3::C0(_, h2229, _, _, _, _, _, _) => {
                    h2229
                }
            };
            let a2235 = {
                let a2232 = match v0.clone() {
                    Adt3::C0(_, _, h2230, _, _, _, _, _) => {
                        h2230
                    }
                };
                let a2233 = match v0.clone() {
                    Adt3::C0(_, _, _, h2231, _, _, _, _) => {
                        h2231
                    }
                };
                nat_mul(a2232, a2233)?
            };
            nat_mul(a2234, a2235)?
        };
        let a2236 = v1;
        let a2237 = 13u64;
        if nat_eq(a2236, a2237) {
            let a2246 = 2u64;
            let a2247 = if f135(v0.clone(), v2) {
                let a2239 = v3;
                let a2240 = f136(match v0 {
                    Adt3::C0(h2238, _, _, _, _, _, _, _) => {
                        h2238
                    }
                }, false)?;
                nat_mul(a2239, a2240)?
            } else {
                let a2244 = v3;
                let a2245 = {
                    let a2242 = match v0 {
                        Adt3::C0(h2241, _, _, _, _, _, _, _) => {
                            h2241
                        }
                    };
                    let a2243 = 2u64;
                    nat_mul(a2242, a2243)?
                };
                nat_mul(a2244, a2245)?
            };
            nat_add(a2246, a2247)
        } else {
            let a2251 = v3;
            let a2252 = f136(match v0 {
                Adt3::C0(h2248, _, _, _, _, _, _, _) => {
                    h2248
                }
            }, {
                let a2249 = v1;
                let a2250 = 16u64;
                nat_eq(a2249, a2250)
            })?;
            nat_mul(a2251, a2252)
        }
    }
}

pub fn f92(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let a2253 = v0;
    let a2254 = 30u64;
    if if nat_eq(a2253, a2254) {
        let a2257 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2255, _, _, _, _) => {
                h2255
            }
        };
        let a2258 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2256, _, _, _, _, _) => {
                h2256
            }
        };
        nat_lt(a2257, a2258)
    } else {
        false
    } {
        Ok(Adt8::C1)
    } else {
        let m2259 = v3;
        match m2259 {
            None => {
                f137(v0, v1, match v2.clone() {
                    Adt2::C0(h2260, _, _, _, _, _, _, _, _, _, _) => {
                        h2260
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2261, _, _, _, _, _, _, _, _, _, _) => {
                        h2261
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2262, _, _, _, _, _, _, _, _, _) => {
                        h2262
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2263, _, _, _, _, _, _, _, _) => {
                        h2263
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2264, _, _, _, _, _, _, _) => {
                        h2264
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2265, _, _, _, _, _, _) => {
                        h2265
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2266, _, _, _, _, _, _, _, _, _) => {
                        h2266
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2267, _, _, _, _, _, _, _, _) => {
                        h2267
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2268, _, _, _, _, _, _, _) => {
                        h2268
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2269, _, _, _, _, _, _) => {
                        h2269
                    }
                })
            }
            Some(v5) => {
                f137(v0, v1, match v2.clone() {
                    Adt2::C0(h2270, _, _, _, _, _, _, _, _, _, _) => {
                        h2270
                    }
                }, match v5.clone() {
                    Adt2::C0(h2271, _, _, _, _, _, _, _, _, _, _) => {
                        h2271
                    }
                }, match v4.clone() {
                    Adt2::C0(h2272, _, _, _, _, _, _, _, _, _, _) => {
                        h2272
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h2273, _, _, _, _, _, _, _, _, _) => {
                        h2273
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2274, _, _, _, _, _, _, _, _) => {
                        h2274
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2275, _, _, _, _, _, _, _) => {
                        h2275
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2276, _, _, _, _, _, _) => {
                        h2276
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h2277, _, _, _, _, _, _, _, _, _) => {
                        h2277
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h2278, _, _, _, _, _, _, _, _) => {
                        h2278
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h2279, _, _, _, _, _, _, _) => {
                        h2279
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h2280, _, _, _, _, _, _) => {
                        h2280
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2281, _, _, _, _, _, _, _, _, _) => {
                        h2281
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2282, _, _, _, _, _, _, _, _) => {
                        h2282
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2283, _, _, _, _, _, _, _) => {
                        h2283
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2284, _, _, _, _, _, _) => {
                        h2284
                    }
                })
            }
        }
    }
}

pub fn f93(v0: Adt9, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt7 {
    let v9: u64 = {
        let a2285 = v7;
        let a2286 = 6u64;
        if nat_eq(a2285, a2286) {
            27u64
        } else {
            v2
        }
    };
    Adt7::C7(v0, f64(v3, v4), f64(v5, v6), f65(v7, v8), f138(v1), f138(v9))
}

pub fn f94(v0: Adt4, v1: u64, v2: u64) -> Adt9 {
    let m2287 = v0;
    match m2287 {
        Adt4::C0 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C1 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C2(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C3(_, _, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C4(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C5(_, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C6(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C7(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C8(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C9(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C10(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C11(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C12(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C13(v30, v31, v32, v33) => {
            Adt9::C4(v30, v31, v32, v33)
        }
        Adt4::C14(v34, _, _) => {
            let a2288 = 13u64;
            let a2289 = v34;
            if nat_le(a2288, a2289) {
                Adt9::C7(v34, v1, v2)
            } else {
                Adt9::C3(v1, v2, true)
            }
        }
        Adt4::C15 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C16(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C17 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C18(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C19(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C20(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C21 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C22(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C23(_, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C24(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C25(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C26(_, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C27(v50) => {
            Adt9::C2(match v50.clone() {
                Adt5::C0(_, _, _, _, _, h2290, _, _) => {
                    h2290
                }
            }, match v50 {
                Adt5::C0(_, _, _, _, _, _, _, h2291) => {
                    h2291
                }
            }, true)
        }
    }
}

pub fn f95(v0: Adt4) -> u64 {
    let m2292 = v0;
    match m2292 {
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
        Adt4::C27(_) => {
            0u64
        }
    }
}

pub fn f96(v0: Adt4, v1: u64) -> u64 {
    let m2293 = v0;
    match m2293 {
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
            let a2294 = v1;
            let a2295 = 6u64;
            if nat_eq(a2294, a2295) {
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
        Adt4::C26(v50, _, _, _) => {
            let a2297 = v1;
            let a2298 = 6u64;
            if if if nat_eq(a2297, a2298) {
                true
            } else {
                let a2299 = v1;
                let a2300 = 7u64;
                nat_eq(a2299, a2300)
            } {
                let a2301 = v50;
                let a2302 = 30u64;
                nat_eq(a2301, a2302)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v49) => {
            match v49 {
                Adt5::C0(_, _, _, _, _, _, _, h2296) => {
                    h2296
                }
            }
        }
    }
}

pub fn f97(v0: Adt4, v1: u64) -> Adt9 {
    let a2303 = v1;
    let a2304 = 4u64;
    if nat_lt(a2303, a2304) {
        Adt9::C0(v1)
    } else {
        let a2305 = v1;
        let a2306 = 4u64;
        if nat_eq(a2305, a2306) {
            Adt9::C1(0u64)
        } else {
            let a2307 = v1;
            let a2308 = 5u64;
            if nat_eq(a2307, a2308) {
                Adt9::C1(3u64)
            } else {
                let a2309 = v1;
                let a2310 = 6u64;
                if nat_eq(a2309, a2310) {
                    Adt9::C2(f96(v0, v1), 0u64, false)
                } else {
                    let a2311 = v1;
                    let a2312 = 7u64;
                    if nat_eq(a2311, a2312) {
                        Adt9::C2(0u64, f96(v0, v1), false)
                    } else {
                        let a2313 = v1;
                        let a2314 = 16u64;
                        if nat_lt(a2313, a2314) {
                            Adt9::C1({
                                let a2315 = v1;
                                let a2316 = 8u64;
                                nat_sub(a2315, a2316)
                            })
                        } else {
                            let a2317 = v1;
                            let a2318 = 20u64;
                            if nat_lt(a2317, a2318) {
                                Adt9::C5({
                                    let a2319 = v1;
                                    let a2320 = 16u64;
                                    nat_sub(a2319, a2320)
                                })
                            } else {
                                let a2321 = v1;
                                let a2322 = 20u64;
                                if nat_eq(a2321, a2322) {
                                    Adt9::C3(0u64, 0u64, false)
                                } else {
                                    let a2323 = v1;
                                    let a2324 = 21u64;
                                    if nat_eq(a2323, a2324) {
                                        Adt9::C0(4u64)
                                    } else {
                                        let a2325 = v1;
                                        let a2326 = 22u64;
                                        if nat_eq(a2325, a2326) {
                                            Adt9::C0(5u64)
                                        } else {
                                            Adt9::C6
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f98(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2328 = v0;
        let a2329 = match v1.clone() {
            Adt3::C0(h2327, _, _, _, _, _, _, _) => {
                h2327
            }
        };
        let a2330 = 0u64;
        nat_quot(a2328, a2329, a2330)
    };
    let v3: u64 = {
        let a2332 = v2;
        let a2333 = match v1.clone() {
            Adt3::C0(_, h2331, _, _, _, _, _, _) => {
                h2331
            }
        };
        let a2334 = 0u64;
        nat_quot(a2332, a2333, a2334)
    };
    f139({
        let a2336 = v0;
        let a2337 = match v1.clone() {
            Adt3::C0(h2335, _, _, _, _, _, _, _) => {
                h2335
            }
        };
        let a2338 = 0u64;
        nat_rem(a2336, a2337, a2338)
    }, {
        let a2340 = v2;
        let a2341 = match v1.clone() {
            Adt3::C0(_, h2339, _, _, _, _, _, _) => {
                h2339
            }
        };
        let a2342 = 0u64;
        nat_rem(a2340, a2341, a2342)
    }, {
        let a2344 = v3;
        let a2345 = match v1.clone() {
            Adt3::C0(_, _, h2343, _, _, _, _, _) => {
                h2343
            }
        };
        let a2346 = 0u64;
        nat_rem(a2344, a2345, a2346)
    }, {
        let a2348 = v3;
        let a2349 = match v1.clone() {
            Adt3::C0(_, _, h2347, _, _, _, _, _) => {
                h2347
            }
        };
        let a2350 = 0u64;
        nat_quot(a2348, a2349, a2350)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2351, _, _, _) => {
            h2351
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2352, _, _) => {
            h2352
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2353, _) => {
            h2353
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2354) => {
            h2354
        }
    })
}

pub fn f99(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2372 = {
        let a2367 = f140(v2, {
            let a2358 = {
                let a2355 = v0;
                let a2356 = v1;
                let a2357 = 0u64;
                nat_quot(a2355, a2356, a2357)
            };
            let a2359 = v2;
            let a2360 = 0u64;
            nat_rem(a2358, a2359, a2360)
        }, {
            let a2364 = {
                let a2361 = v0;
                let a2362 = v1;
                let a2363 = 0u64;
                nat_quot(a2361, a2362, a2363)
            };
            let a2365 = v2;
            let a2366 = 0u64;
            nat_quot(a2364, a2365, a2366)
        })?;
        let a2368 = v1;
        nat_mul(a2367, a2368)?
    };
    let a2373 = {
        let a2369 = v0;
        let a2370 = v1;
        let a2371 = 0u64;
        nat_rem(a2369, a2370, a2371)
    };
    nat_add(a2372, a2373)
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2375 = v0;
        let a2376 = match v2.clone() {
            Adt3::C0(h2374, _, _, _, _, _, _, _) => {
                h2374
            }
        };
        let a2377 = 0u64;
        nat_quot(a2375, a2376, a2377)
    };
    let v4: u64 = {
        let a2379 = v3;
        let a2380 = match v2.clone() {
            Adt3::C0(_, h2378, _, _, _, _, _, _) => {
                h2378
            }
        };
        let a2381 = 0u64;
        nat_quot(a2379, a2380, a2381)
    };
    f141({
        let a2383 = v0;
        let a2384 = match v2.clone() {
            Adt3::C0(h2382, _, _, _, _, _, _, _) => {
                h2382
            }
        };
        let a2385 = 0u64;
        nat_rem(a2383, a2384, a2385)
    }, {
        let a2387 = v3;
        let a2388 = match v2.clone() {
            Adt3::C0(_, h2386, _, _, _, _, _, _) => {
                h2386
            }
        };
        let a2389 = 0u64;
        nat_rem(a2387, a2388, a2389)
    }, {
        let a2391 = v4;
        let a2392 = match v2.clone() {
            Adt3::C0(_, _, h2390, _, _, _, _, _) => {
                h2390
            }
        };
        let a2393 = 0u64;
        nat_rem(a2391, a2392, a2393)
    }, {
        let a2395 = v4;
        let a2396 = match v2 {
            Adt3::C0(_, _, h2394, _, _, _, _, _) => {
                h2394
            }
        };
        let a2397 = 0u64;
        nat_quot(a2395, a2396, a2397)
    }, match v1.clone() {
        Adt3::C0(h2398, _, _, _, _, _, _, _) => {
            h2398
        }
    }, match v1.clone() {
        Adt3::C0(_, h2399, _, _, _, _, _, _) => {
            h2399
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2400, _, _, _, _, _) => {
            h2400
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2401, _, _, _, _) => {
            h2401
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2402, _, _, _) => {
            h2402
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2403, _, _) => {
            h2403
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2404, _) => {
            h2404
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2405) => {
            h2405
        }
    })
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt7> {
    let v5: u64 = {
        let a2407 = v0;
        let a2408 = match v3.clone() {
            Adt3::C0(h2406, _, _, _, _, _, _, _) => {
                h2406
            }
        };
        let a2409 = 0u64;
        nat_quot(a2407, a2408, a2409)
    };
    let v6: u64 = {
        let a2411 = v5;
        let a2412 = match v3.clone() {
            Adt3::C0(_, h2410, _, _, _, _, _, _) => {
                h2410
            }
        };
        let a2413 = 0u64;
        nat_quot(a2411, a2412, a2413)
    };
    let v7: u64 = {
        let a2415 = v0;
        let a2416 = match v3.clone() {
            Adt3::C0(h2414, _, _, _, _, _, _, _) => {
                h2414
            }
        };
        let a2417 = 0u64;
        nat_rem(a2415, a2416, a2417)
    };
    let v8: u64 = {
        let a2419 = v5;
        let a2420 = match v3.clone() {
            Adt3::C0(_, h2418, _, _, _, _, _, _) => {
                h2418
            }
        };
        let a2421 = 0u64;
        nat_rem(a2419, a2420, a2421)
    };
    let v9: u64 = {
        let a2423 = v6;
        let a2424 = match v3.clone() {
            Adt3::C0(_, _, h2422, _, _, _, _, _) => {
                h2422
            }
        };
        let a2425 = 0u64;
        nat_rem(a2423, a2424, a2425)
    };
    let v10: u64 = {
        let a2427 = v6;
        let a2428 = match v3.clone() {
            Adt3::C0(_, _, h2426, _, _, _, _, _) => {
                h2426
            }
        };
        let a2429 = 0u64;
        nat_quot(a2427, a2428, a2429)
    };
    let v11: u64 = {
        let a2430 = v4;
        let a2431 = 0u64;
        if nat_eq(a2430, a2431) {
            match v1.clone() {
                Adt3::C0(h2432, _, _, _, _, _, _, _) => {
                    h2432
                }
            }
        } else {
            let a2433 = v4;
            let a2434 = 1u64;
            if nat_eq(a2433, a2434) {
                match v1.clone() {
                    Adt3::C0(_, h2435, _, _, _, _, _, _) => {
                        h2435
                    }
                }
            } else {
                let a2436 = v4;
                let a2437 = 2u64;
                if nat_eq(a2436, a2437) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2438, _, _, _, _, _) => {
                            h2438
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2439, _, _, _, _) => {
                            h2439
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2446 = v11;
        let a2447 = {
            let a2440 = v4;
            let a2441 = 0u64;
            if nat_eq(a2440, a2441) {
                v7
            } else {
                let a2442 = v4;
                let a2443 = 1u64;
                if nat_eq(a2442, a2443) {
                    v8
                } else {
                    let a2444 = v4;
                    let a2445 = 2u64;
                    if nat_eq(a2444, a2445) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2446, a2447)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt7::C1(v12, f139(if if v12 {
        let a2448 = v4;
        let a2449 = 0u64;
        nat_eq(a2448, a2449)
    } else {
        false
    } {
        let a2450 = v7;
        let a2451 = v11;
        nat_sub(a2450, a2451)
    } else {
        v7
    }, if if v12 {
        let a2452 = v4;
        let a2453 = 1u64;
        nat_eq(a2452, a2453)
    } else {
        false
    } {
        let a2454 = v8;
        let a2455 = v11;
        nat_sub(a2454, a2455)
    } else {
        v8
    }, if if v12 {
        let a2456 = v4;
        let a2457 = 2u64;
        nat_eq(a2456, a2457)
    } else {
        false
    } {
        let a2458 = v9;
        let a2459 = v11;
        nat_sub(a2458, a2459)
    } else {
        v9
    }, if if v12 {
        let a2460 = v4;
        let a2461 = 3u64;
        nat_eq(a2460, a2461)
    } else {
        false
    } {
        let a2462 = v10;
        let a2463 = v11;
        nat_sub(a2462, a2463)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2464, _, _, _) => {
            h2464
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2465, _, _) => {
            h2465
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2466, _) => {
            h2466
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2467) => {
            h2467
        }
    })?, f98(v0, v3)?))
}

pub fn f102(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt7> {
    let v8: u64 = f119(v0, v2.clone(), 0u64);
    let v9: u64 = f119(v0, v2.clone(), 1u64);
    let v10: u64 = f119(v0, v2.clone(), 2u64);
    let v11: u64 = f119(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt7::C1(false, f121(v1.clone(), f142(v8, v3, match v1.clone() {
            Adt3::C0(h2468, _, _, _, _, _, _, _) => {
                h2468
            }
        })?, f142(v9, v4, match v1.clone() {
            Adt3::C0(_, h2469, _, _, _, _, _, _) => {
                h2469
            }
        })?, f142(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2470, _, _, _, _, _) => {
                h2470
            }
        })?, f142(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2471, _, _, _, _) => {
                h2471
            }
        })?)?, f98(v0, v2)?))
    } else {
        if if if if f143(v8, v3, match v1.clone() {
            Adt3::C0(h2472, _, _, _, _, _, _, _) => {
                h2472
            }
        })? {
            f143(v9, v4, match v1.clone() {
                Adt3::C0(_, h2473, _, _, _, _, _, _) => {
                    h2473
                }
            })?
        } else {
            false
        } {
            f143(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2474, _, _, _, _, _) => {
                    h2474
                }
            })?
        } else {
            false
        } {
            f143(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2475, _, _, _, _) => {
                    h2475
                }
            })?
        } else {
            false
        } {
            Ok(Adt7::C1(false, f121(v1, f144(v8, v3)?, f144(v9, v4)?, f144(v10, v5)?, f144(v11, v6)?)?, f98(v0, v2)?))
        } else {
            Ok(Adt7::C1(true, 0u64, f98(v0, v2)?))
        }
    }
}

pub fn f103(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let v4: u64 = f119(v0, v2.clone(), 0u64);
    let v5: u64 = f119(v0, v2.clone(), 1u64);
    let v6: u64 = f119(v0, v2.clone(), 2u64);
    let v7: u64 = f119(v0, v2.clone(), 3u64);
    Ok(Adt7::C1(false, f121(v1.clone(), f145(v4, v3, match v1 {
        Adt3::C0(h2476, _, _, _, _, _, _, _) => {
            h2476
        }
    })?, v5, v6, v7)?, f98(v0, v2)?))
}

pub fn f104(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt7> {
    let a2477 = v0;
    let a2478 = v3;
    if nat_lt(a2477, a2478) {
        Ok(Adt7::C1(false, {
            let a2479 = v0;
            let a2480 = 4u64;
            nat_mul(a2479, a2480)?
        }, {
            let a2481 = v0;
            let a2482 = 4u64;
            nat_mul(a2481, a2482)?
        }))
    } else {
        let v6: u64 = {
            let a2483 = v0;
            let a2484 = v3;
            nat_sub(a2483, a2484)
        };
        let v7: u64 = f98(v6, v1)?;
        let v8: u64 = f146(v6, v2, v4)?;
        if v5 {
            Ok(Adt7::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt7::C1(true, v7, v8))
        }
    }
}

pub fn f105(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let a2485 = v0;
    let a2486 = v3;
    if nat_lt(a2485, a2486) {
        Ok(Adt7::C1(true, 0u64, f98(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2487 = v0;
            let a2488 = v3;
            nat_sub(a2487, a2488)
        };
        let v5: u64 = f100(v4, v2, v1.clone())?;
        Ok(Adt7::C4(8u64, 3u64, v5, 0u64, f98(v4, v1)?, 3u64, v5))
    }
}

pub fn f106(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt7> {
    let v4: u64 = {
        let a2490 = v1;
        let a2491 = match v3.clone() {
            Adt3::C0(h2489, _, _, _, _, _, _, _) => {
                h2489
            }
        };
        let a2492 = 0u64;
        nat_rem(a2490, a2491, a2492)
    };
    let v5: u64 = {
        let a2498 = {
            let a2494 = v1;
            let a2495 = match v3.clone() {
                Adt3::C0(h2493, _, _, _, _, _, _, _) => {
                    h2493
                }
            };
            let a2496 = 0u64;
            nat_quot(a2494, a2495, a2496)
        };
        let a2499 = match v3.clone() {
            Adt3::C0(_, h2497, _, _, _, _, _, _) => {
                h2497
            }
        };
        let a2500 = 0u64;
        nat_rem(a2498, a2499, a2500)
    };
    let v6: bool = f147(v0, v4, v5);
    Ok(Adt7::C1(!v6, if v6 {
        f100(v1, v2, v3.clone())?
    } else {
        0u64
    }, f98(v1, v3)?))
}

pub fn f107(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt7> {
    Ok(Adt7::C4({
        let a2501 = v3;
        let a2502 = 8u64;
        nat_add(a2501, a2502)?
    }, 0u64, f98(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f100(v0, v5, v2.clone())?
    }, 3u64, f98(v0, v2)?))
}

pub fn f108(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt7> {
    let v6: u64 = {
        let a2504 = v0;
        let a2505 = match v2.clone() {
            Adt3::C0(h2503, _, _, _, _, _, _, _) => {
                h2503
            }
        };
        let a2506 = 0u64;
        nat_rem(a2504, a2505, a2506)
    };
    let v7: u64 = {
        let a2508 = v0;
        let a2509 = match v2.clone() {
            Adt3::C0(h2507, _, _, _, _, _, _, _) => {
                h2507
            }
        };
        let a2510 = 0u64;
        nat_quot(a2508, a2509, a2510)
    };
    let v8: u64 = {
        let a2512 = v7;
        let a2513 = match v2.clone() {
            Adt3::C0(_, h2511, _, _, _, _, _, _) => {
                h2511
            }
        };
        let a2514 = 0u64;
        nat_rem(a2512, a2513, a2514)
    };
    let v9: u64 = {
        let a2516 = v7;
        let a2517 = match v2.clone() {
            Adt3::C0(_, h2515, _, _, _, _, _, _) => {
                h2515
            }
        };
        let a2518 = 0u64;
        nat_quot(a2516, a2517, a2518)
    };
    let v10: u64 = {
        let a2520 = v9;
        let a2521 = match v2.clone() {
            Adt3::C0(_, _, h2519, _, _, _, _, _) => {
                h2519
            }
        };
        let a2522 = 0u64;
        nat_rem(a2520, a2521, a2522)
    };
    let v11: u64 = {
        let a2524 = v9;
        let a2525 = match v2.clone() {
            Adt3::C0(_, _, h2523, _, _, _, _, _) => {
                h2523
            }
        };
        let a2526 = 0u64;
        nat_quot(a2524, a2525, a2526)
    };
    let v12: u64 = f98(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2535 = {
                let a2530 = {
                    let a2527 = v10;
                    let a2528 = v4;
                    let a2529 = 0u64;
                    nat_quot(a2527, a2528, a2529)
                };
                let a2531 = v5;
                nat_mul(a2530, a2531)?
            };
            let a2536 = {
                let a2532 = v8;
                let a2533 = v4;
                let a2534 = 0u64;
                nat_quot(a2532, a2533, a2534)
            };
            nat_add(a2535, a2536)?
        };
        let v14: u64 = {
            let a2556 = v6;
            let a2557 = {
                let a2554 = match v1.clone() {
                    Adt3::C0(h2537, _, _, _, _, _, _, _) => {
                        h2537
                    }
                };
                let a2555 = {
                    let a2552 = {
                        let a2538 = v8;
                        let a2539 = v4;
                        let a2540 = 0u64;
                        nat_rem(a2538, a2539, a2540)
                    };
                    let a2553 = {
                        let a2550 = match v1.clone() {
                            Adt3::C0(_, h2541, _, _, _, _, _, _) => {
                                h2541
                            }
                        };
                        let a2551 = {
                            let a2548 = {
                                let a2542 = v10;
                                let a2543 = v4;
                                let a2544 = 0u64;
                                nat_rem(a2542, a2543, a2544)
                            };
                            let a2549 = {
                                let a2546 = match v1.clone() {
                                    Adt3::C0(_, _, h2545, _, _, _, _, _) => {
                                        h2545
                                    }
                                };
                                let a2547 = v13;
                                nat_mul(a2546, a2547)?
                            };
                            nat_add(a2548, a2549)?
                        };
                        nat_mul(a2550, a2551)?
                    };
                    nat_add(a2552, a2553)?
                };
                nat_mul(a2554, a2555)?
            };
            nat_add(a2556, a2557)?
        };
        Ok(Adt7::C1(false, f98(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2563 = {
                let a2561 = {
                    let a2558 = v11;
                    let a2559 = v5;
                    let a2560 = 0u64;
                    nat_rem(a2558, a2559, a2560)
                };
                let a2562 = v4;
                nat_mul(a2561, a2562)?
            };
            let a2564 = v8;
            nat_add(a2563, a2564)?
        };
        let v16: u64 = {
            let a2570 = {
                let a2568 = {
                    let a2565 = v11;
                    let a2566 = v5;
                    let a2567 = 0u64;
                    nat_quot(a2565, a2566, a2567)
                };
                let a2569 = v4;
                nat_mul(a2568, a2569)?
            };
            let a2571 = v10;
            nat_add(a2570, a2571)?
        };
        let a2573 = v15;
        let a2574 = match v1.clone() {
            Adt3::C0(_, h2572, _, _, _, _, _, _) => {
                h2572
            }
        };
        if if nat_lt(a2573, a2574) {
            let a2576 = v16;
            let a2577 = match v1.clone() {
                Adt3::C0(_, _, h2575, _, _, _, _, _) => {
                    h2575
                }
            };
            nat_lt(a2576, a2577)
        } else {
            false
        } {
            Ok(Adt7::C1(false, f98({
                let a2586 = v6;
                let a2587 = {
                    let a2584 = match v1.clone() {
                        Adt3::C0(h2578, _, _, _, _, _, _, _) => {
                            h2578
                        }
                    };
                    let a2585 = {
                        let a2582 = v15;
                        let a2583 = {
                            let a2580 = match v1.clone() {
                                Adt3::C0(_, h2579, _, _, _, _, _, _) => {
                                    h2579
                                }
                            };
                            let a2581 = v16;
                            nat_mul(a2580, a2581)?
                        };
                        nat_add(a2582, a2583)?
                    };
                    nat_mul(a2584, a2585)?
                };
                nat_add(a2586, a2587)?
            }, v1)?, v12))
        } else {
            Ok(Adt7::C1(true, 0u64, v12))
        }
    }
}

pub fn f109(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt7> {
    let v3: u64 = f98(v0, v2.clone())?;
    let v4: u64 = f98(v0, v1.clone())?;
    let a2592 = {
        let a2589 = v0;
        let a2590 = match v1 {
            Adt3::C0(h2588, _, _, _, _, _, _, _) => {
                h2588
            }
        };
        let a2591 = 0u64;
        nat_rem(a2589, a2590, a2591)
    };
    let a2593 = 0u64;
    if nat_eq(a2592, a2593) {
        Ok(Adt7::C1(false, v4, v3))
    } else {
        Ok(Adt7::C4(8u64, 3u64, f98({
            let a2594 = v0;
            let a2595 = 1u64;
            nat_sub(a2594, a2595)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt7> {
    let a2596 = v3;
    let a2597 = 13u64;
    if nat_eq(a2596, a2597) {
        f148(v0, v1, v4)
    } else {
        let v5: u64 = f136(match v1.clone() {
            Adt3::C0(h2598, _, _, _, _, _, _, _) => {
                h2598
            }
        }, {
            let a2599 = v3;
            let a2600 = 16u64;
            nat_eq(a2599, a2600)
        })?;
        let v6: u64 = {
            let a2601 = v0;
            let a2602 = v5;
            let a2603 = 0u64;
            nat_quot(a2601, a2602, a2603)
        };
        let v7: u64 = {
            let a2604 = v0;
            let a2605 = v5;
            let a2606 = 0u64;
            nat_rem(a2604, a2605, a2606)
        };
        let v8: u64 = {
            let a2610 = 1u64;
            let a2611 = {
                let a2608 = match v1.clone() {
                    Adt3::C0(h2607, _, _, _, _, _, _, _) => {
                        h2607
                    }
                };
                let a2609 = 2u64;
                nat_mul(a2608, a2609)?
            };
            nat_add(a2610, a2611)?
        };
        let a2612 = v7;
        let a2613 = v8;
        if nat_lt(a2612, a2613) {
            f149(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f98(v6, v2)?;
            let a2614 = v3;
            let a2615 = 14u64;
            if nat_eq(a2614, a2615) {
                Ok(Adt7::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2616 = v7;
                let a2617 = v8;
                if nat_eq(a2616, a2617) {
                    Ok(Adt7::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2620 = v7;
                    let a2621 = {
                        let a2618 = v8;
                        let a2619 = 1u64;
                        nat_add(a2618, a2619)?
                    };
                    if nat_eq(a2620, a2621) {
                        Ok(Adt7::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2622, _, _, _, _, _, _, _) => {
                                h2622
                            }
                        }, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(Adt7::C4(5u64, 1u64, 1u64, 1u64, 2u64, 3u64, v9))
                    }
                }
            }
        }
    }
}

pub fn f111(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let a2623 = v0;
    let a2624 = 0u64;
    if nat_eq(a2623, a2624) {
        Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2631 = {
            let a2625 = v0;
            let a2626 = 1u64;
            nat_add(a2625, a2626)?
        };
        let a2632 = {
            let a2629 = 2u64;
            let a2630 = {
                let a2627 = v3;
                let a2628 = 2u64;
                nat_mul(a2627, a2628)?
            };
            nat_add(a2629, a2630)?
        };
        if nat_eq(a2631, a2632) {
            Ok(Adt7::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2633 = v0;
                let a2634 = 1u64;
                nat_sub(a2633, a2634)
            };
            let v5: u64 = {
                let a2635 = v4;
                let a2636 = 2u64;
                let a2637 = 0u64;
                nat_quot(a2635, a2636, a2637)
            };
            let a2641 = {
                let a2638 = v4;
                let a2639 = 2u64;
                let a2640 = 0u64;
                nat_rem(a2638, a2639, a2640)
            };
            let a2642 = 0u64;
            if nat_eq(a2641, a2642) {
                Ok(Adt7::C4(17u64, 0u64, f98(v5, v1)?, 4u64, f98(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt7::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f112(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a2643 = v0;
    let a2644 = f19(v1, v2, v3)?;
    if nat_lt(a2643, a2644) {
        let a2645 = v0;
        let a2646 = v1;
        if nat_lt(a2645, a2646) {
            Ok(Adt7::C1(false, {
                let a2647 = 4u64;
                let a2648 = v0;
                nat_mul(a2647, a2648)?
            }, {
                let a2649 = 4u64;
                let a2650 = v0;
                nat_mul(a2649, a2650)?
            }))
        } else {
            let v4: u64 = {
                let a2651 = v0;
                let a2652 = v1;
                nat_sub(a2651, a2652)
            };
            let v5: u64 = f150(v4, v2)?;
            let a2653 = v5;
            let a2654 = 2u64;
            if nat_lt(a2653, a2654) {
                Ok(Adt7::C4(6u64, {
                    let a2655 = v5;
                    let a2656 = 0u64;
                    if nat_eq(a2655, a2656) {
                        4u64
                    } else {
                        5u64
                    }
                }, f151(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f152(v4, v2)?;
                Ok(Adt7::C4(8u64, 3u64, v6, 1u64, if f153(v4, v2)? {
                    1u64
                } else {
                    0u64
                }, 3u64, v6))
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f113(v0: Adt3, v1: Adt3) -> R<u64> {
    let a2664 = {
        let a2657 = 2u64;
        let a2658 = f90(v1.clone())?;
        nat_mul(a2657, a2658)?
    };
    let a2665 = {
        let a2662 = f90(v1)?;
        let a2663 = {
            let a2660 = 1u64;
            let a2661 = match v0 {
                Adt3::C0(h2659, _, _, _, _, _, _, _) => {
                    h2659
                }
            };
            nat_add(a2660, a2661)?
        };
        nat_mul(a2662, a2663)?
    };
    nat_add(a2664, a2665)
}

pub fn f114(v0: u64, v1: Adt3) -> R<Adt7> {
    Ok(Adt7::C4(19u64, 4u64, f98(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f115(v0: u64, v1: u64) -> bool {
    let a2666 = v1;
    let a2667 = 2147483647u64;
    if nat_le(a2666, a2667) {
        let a2668 = v1;
        let a2669 = v0;
        nat_lt(a2668, a2669)
    } else {
        false
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt7> {
    let m2670 = f79(v0);
    match m2670 {
        Adt12::C0 => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt12::C1(v7) => {
            let v8: u64 = {
                let a2672 = v2;
                let a2673 = match v7.clone() {
                    Adt13::C0(_, h2671, _) => {
                        h2671
                    }
                };
                nat_mul(a2672, a2673)?
            };
            let m2686 = f154(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2674, _, _, _, _, _, _, _) => {
                    h2674
                }
            }, match v5.clone() {
                Adt3::C0(_, h2675, _, _, _, _, _, _) => {
                    h2675
                }
            }, match v5 {
                Adt3::C0(_, _, h2676, _, _, _, _, _) => {
                    h2676
                }
            }, {
                let a2679 = match v3.clone() {
                    Adt3::C0(h2677, _, _, _, _, _, _, _) => {
                        h2677
                    }
                };
                let a2680 = match v7.clone() {
                    Adt13::C0(_, h2678, _) => {
                        h2678
                    }
                };
                nat_mul(a2679, a2680)?
            }, match v3.clone() {
                Adt3::C0(_, h2681, _, _, _, _, _, _) => {
                    h2681
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2682, _, _, _) => {
                    h2682
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2683, _, _) => {
                    h2683
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2684, _) => {
                    h2684
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2685) => {
                    h2685
                }
            })?;
            match m2686 {
                None => {
                    Ok(Adt7::C0(Adt8::C1))
                }
                Some(v9) => {
                    let a2687 = v0;
                    let a2688 = 0u64;
                    if if nat_eq(a2687, a2688) {
                        true
                    } else {
                        let a2689 = v0;
                        let a2690 = 26u64;
                        nat_eq(a2689, a2690)
                    } {
                        Ok(Adt7::C1(false, v9, f98({
                            let a2694 = {
                                let a2692 = v1;
                                let a2693 = match v4.clone() {
                                    Adt3::C0(h2691, _, _, _, _, _, _, _) => {
                                        h2691
                                    }
                                };
                                nat_mul(a2692, a2693)?
                            };
                            let a2695 = v8;
                            nat_add(a2694, a2695)?
                        }, v4)?))
                    } else {
                        Ok(Adt7::C5(v0, v9, match v7.clone() {
                            Adt13::C0(_, _, h2696) => {
                                h2696
                            }
                        }, f98({
                            let a2700 = {
                                let a2698 = v1;
                                let a2699 = match v4.clone() {
                                    Adt3::C0(h2697, _, _, _, _, _, _, _) => {
                                        h2697
                                    }
                                };
                                nat_mul(a2698, a2699)?
                            };
                            let a2701 = v8;
                            nat_add(a2700, a2701)?
                        }, v4.clone())?, {
                            let a2708 = {
                                let a2706 = {
                                    let a2703 = match v7 {
                                        Adt13::C0(_, h2702, _) => {
                                            h2702
                                        }
                                    };
                                    let a2704 = 1u64;
                                    nat_sub(a2703, a2704)
                                };
                                let a2707 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2705, _, _, _) => {
                                        h2705
                                    }
                                };
                                nat_mul(a2706, a2707)?
                            };
                            let a2709 = 4u64;
                            nat_add(a2708, a2709)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2710, _, _, _) => {
                                h2710
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f117(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2711 = v1;
    let a2712 = 0u64;
    if if nat_eq(a2711, a2712) {
        true
    } else {
        let a2713 = v2;
        let a2714 = 0u64;
        nat_eq(a2713, a2714)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f118(v0: u64, v1: u64) -> bool {
    let a2715 = v0;
    let a2716 = 4294967295u64;
    if nat_eq(a2715, a2716) {
        true
    } else {
        let a2717 = v0;
        let a2718 = 2147483648u64;
        if nat_lt(a2717, a2718) {
            let a2719 = v0;
            let a2720 = v1;
            nat_lt(a2719, a2720)
        } else {
            false
        }
    }
}

pub fn f119(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2722 = v0;
        let a2723 = match v1.clone() {
            Adt3::C0(h2721, _, _, _, _, _, _, _) => {
                h2721
            }
        };
        let a2724 = 0u64;
        nat_quot(a2722, a2723, a2724)
    };
    let v4: u64 = {
        let a2726 = v3;
        let a2727 = match v1.clone() {
            Adt3::C0(_, h2725, _, _, _, _, _, _) => {
                h2725
            }
        };
        let a2728 = 0u64;
        nat_quot(a2726, a2727, a2728)
    };
    let a2729 = v2;
    let a2730 = 0u64;
    if nat_eq(a2729, a2730) {
        let a2732 = v0;
        let a2733 = match v1 {
            Adt3::C0(h2731, _, _, _, _, _, _, _) => {
                h2731
            }
        };
        let a2734 = 0u64;
        nat_rem(a2732, a2733, a2734)
    } else {
        let a2735 = v2;
        let a2736 = 1u64;
        if nat_eq(a2735, a2736) {
            let a2738 = v3;
            let a2739 = match v1 {
                Adt3::C0(_, h2737, _, _, _, _, _, _) => {
                    h2737
                }
            };
            let a2740 = 0u64;
            nat_rem(a2738, a2739, a2740)
        } else {
            let a2741 = v2;
            let a2742 = 2u64;
            if nat_eq(a2741, a2742) {
                let a2744 = v4;
                let a2745 = match v1 {
                    Adt3::C0(_, _, h2743, _, _, _, _, _) => {
                        h2743
                    }
                };
                let a2746 = 0u64;
                nat_rem(a2744, a2745, a2746)
            } else {
                let a2748 = v4;
                let a2749 = match v1 {
                    Adt3::C0(_, _, h2747, _, _, _, _, _) => {
                        h2747
                    }
                };
                let a2750 = 0u64;
                nat_quot(a2748, a2749, a2750)
            }
        }
    }
}

pub fn f120(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2760 = {
            let a2757 = {
                let a2752 = v2;
                let a2753 = match v0.clone() {
                    Adt6::C0(_, h2751, _, _) => {
                        h2751
                    }
                };
                nat_mul(a2752, a2753)?
            };
            let a2758 = {
                let a2755 = v3;
                let a2756 = match v0.clone() {
                    Adt6::C0(_, _, _, h2754) => {
                        h2754
                    }
                };
                nat_mul(a2755, a2756)?
            };
            nat_add(a2757, a2758)?
        };
        let a2761 = f130(match v0.clone() {
            Adt6::C0(_, _, h2759, _) => {
                h2759
            }
        });
        nat_add(a2760, a2761)?
    };
    let v5: u64 = f129(match v0 {
        Adt6::C0(_, _, h2762, _) => {
            h2762
        }
    });
    let a2763 = v5;
    let a2764 = v4;
    if if nat_le(a2763, a2764) {
        let a2767 = {
            let a2765 = v4;
            let a2766 = v5;
            nat_sub(a2765, a2766)
        };
        let a2768 = v1;
        nat_lt(a2767, a2768)
    } else {
        false
    } {
        Ok(Some({
            let a2769 = v4;
            let a2770 = v5;
            nat_sub(a2769, a2770)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f121(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f139(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2771, _, _, _) => {
            h2771
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2772, _, _) => {
            h2772
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2773, _) => {
            h2773
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2774) => {
            h2774
        }
    })
}

pub fn f122(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt7> {
    let v5: u64 = {
        let a2781 = v3;
        let a2782 = {
            let a2778 = f98({
                let a2775 = v0;
                let a2776 = 2u64;
                let a2777 = 0u64;
                nat_quot(a2775, a2776, a2777)
            }, v1)?;
            let a2779 = v2;
            let a2780 = 0u64;
            nat_quot(a2778, a2779, a2780)
        };
        nat_add(a2781, a2782)?
    };
    let a2786 = {
        let a2783 = v0;
        let a2784 = 2u64;
        let a2785 = 0u64;
        nat_rem(a2783, a2784, a2785)
    };
    let a2787 = 0u64;
    if nat_eq(a2786, a2787) {
        Ok(Adt7::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2788 = v4;
        let a2789 = 0u64;
        if nat_eq(a2788, a2789) {
            Ok(Adt7::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt7::C0(Adt8::C1))
        }
    }
}

pub fn f123(v0: u64, v1: u64) -> R<u64> {
    let a2792 = 8u64;
    let a2793 = {
        let a2790 = f155(v0);
        let a2791 = v1;
        nat_mul(a2790, a2791)?
    };
    nat_add(a2792, a2793)
}

pub fn f124(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt7> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2794, _, _, _, _, _, _, _) => {
            h2794
        }
    };
    let v7: u64 = f155(v3);
    let v8: u64 = {
        let a2797 = 1u64;
        let a2798 = {
            let a2795 = v6;
            let a2796 = v7;
            nat_mul(a2795, a2796)?
        };
        nat_add(a2797, a2798)?
    };
    let a2799 = v0;
    let a2800 = 0u64;
    if nat_eq(a2799, a2800) {
        Ok(f156(0u64))
    } else {
        let a2801 = v0;
        let a2802 = v8;
        if nat_lt(a2801, a2802) {
            let v9: u64 = {
                let a2803 = v0;
                let a2804 = 1u64;
                nat_sub(a2803, a2804)
            };
            let v10: u64 = {
                let a2805 = v9;
                let a2806 = v7;
                let a2807 = 0u64;
                nat_rem(a2805, a2806, a2807)
            };
            let v11: u64 = f98({
                let a2813 = {
                    let a2808 = v1;
                    let a2809 = v6;
                    nat_mul(a2808, a2809)?
                };
                let a2814 = {
                    let a2810 = v9;
                    let a2811 = v7;
                    let a2812 = 0u64;
                    nat_quot(a2810, a2811, a2812)
                };
                nat_add(a2813, a2814)?
            }, v2)?;
            let a2815 = v3;
            let a2816 = 30u64;
            if nat_eq(a2815, a2816) {
                let a2817 = v10;
                let a2818 = 0u64;
                if nat_eq(a2817, a2818) {
                    Ok(Adt7::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2819 = v10;
                    let a2820 = 1u64;
                    if nat_eq(a2819, a2820) {
                        Ok(Adt7::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2821 = v10;
                        let a2822 = 2u64;
                        if nat_eq(a2821, a2822) {
                            Ok(Adt7::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt7::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2823 = v10;
                let a2824 = 0u64;
                if nat_eq(a2823, a2824) {
                    Ok(Adt7::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2825 = v10;
                    let a2826 = 1u64;
                    if nat_eq(a2825, a2826) {
                        Ok(Adt7::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt7::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2827 = v0;
                let a2828 = v8;
                nat_sub(a2827, a2828)
            };
            let a2829 = v12;
            let a2830 = 0u64;
            if nat_eq(a2829, a2830) {
                Ok(Adt7::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2831 = v12;
                let a2832 = 1u64;
                if nat_eq(a2831, a2832) {
                    Ok(Adt7::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2833 = v12;
                    let a2834 = 2u64;
                    if nat_eq(a2833, a2834) {
                        Ok(Adt7::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2835 = v12;
                        let a2836 = 3u64;
                        if nat_eq(a2835, a2836) {
                            Ok(Adt7::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2837 = v12;
                            let a2838 = 4u64;
                            if nat_eq(a2837, a2838) {
                                Ok(Adt7::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2839 = v12;
                                let a2840 = 5u64;
                                if nat_eq(a2839, a2840) {
                                    Ok(Adt7::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2841 = v12;
                                    let a2842 = 6u64;
                                    if nat_eq(a2841, a2842) {
                                        if f157(v5) {
                                            Ok(Adt7::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2843 = 3u64;
                                                let a2844 = v1;
                                                nat_add(a2843, a2844)?
                                            }))
                                        } else {
                                            Ok(Adt7::C0(Adt8::C1))
                                        }
                                    } else {
                                        Ok(Adt7::C0(Adt8::C1))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f125(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let v4: u64 = {
        let a2845 = v0;
        let a2846 = f72(v3);
        let a2847 = 0u64;
        nat_quot(a2845, a2846, a2847)
    };
    let v5: u64 = {
        let a2852 = 3u64;
        let a2853 = {
            let a2849 = v4;
            let a2850 = match v1.clone() {
                Adt3::C0(h2848, _, _, _, _, _, _, _) => {
                    h2848
                }
            };
            let a2851 = 0u64;
            nat_quot(a2849, a2850, a2851)
        };
        nat_add(a2852, a2853)?
    };
    let v6: u64 = f98(v4, v1)?;
    let v7: u64 = f98(v4, v2)?;
    let a2854 = v3;
    let a2855 = 30u64;
    if nat_eq(a2854, a2855) {
        let a2859 = {
            let a2856 = v0;
            let a2857 = 3u64;
            let a2858 = 0u64;
            nat_rem(a2856, a2857, a2858)
        };
        let a2860 = 0u64;
        if nat_eq(a2859, a2860) {
            Ok(Adt7::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2864 = {
                let a2861 = v0;
                let a2862 = 3u64;
                let a2863 = 0u64;
                nat_rem(a2861, a2862, a2863)
            };
            let a2865 = 1u64;
            if nat_eq(a2864, a2865) {
                Ok(Adt7::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt7::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt7::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f126(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a2866 = v0;
        let a2867 = 8388608u64;
        let a2868 = 0u64;
        nat_quot(a2866, a2867, a2868)
    };
    let a2869 = v0;
    let a2870 = 2147483648u64;
    if if if nat_lt(a2869, a2870) {
        let a2871 = 127u64;
        let a2872 = v1;
        nat_le(a2871, a2872)
    } else {
        false
    } {
        let a2873 = v1;
        let a2874 = 190u64;
        nat_lt(a2873, a2874)
    } else {
        false
    } {
        let v2: u64 = {
            let a2878 = 8388608u64;
            let a2879 = {
                let a2875 = v0;
                let a2876 = 8388608u64;
                let a2877 = 0u64;
                nat_rem(a2875, a2876, a2877)
            };
            nat_add(a2878, a2879)?
        };
        let a2880 = v1;
        let a2881 = 150u64;
        if nat_lt(a2880, a2881) {
            let v3: u64 = f158({
                let a2882 = 150u64;
                let a2883 = v1;
                nat_sub(a2882, a2883)
            });
            let a2887 = {
                let a2884 = v2;
                let a2885 = v3;
                let a2886 = 0u64;
                nat_rem(a2884, a2885, a2886)
            };
            let a2888 = 0u64;
            if nat_eq(a2887, a2888) {
                let a2889 = v2;
                let a2890 = v3;
                let a2891 = 0u64;
                Ok(nat_quot(a2889, a2890, a2891))
            } else {
                Ok(0u64)
            }
        } else {
            let a2894 = f158({
                let a2892 = v1;
                let a2893 = 150u64;
                nat_sub(a2892, a2893)
            });
            let a2895 = v2;
            nat_mul(a2894, a2895)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f127(v0: u64, v1: u64, v2: u64) -> bool {
    let a2896 = v0;
    let a2897 = 1u64;
    if nat_le(a2896, a2897) {
        true
    } else {
        let a2901 = {
            let a2898 = v1;
            let a2899 = v2;
            let a2900 = 1u64;
            nat_rem(a2898, a2899, a2900)
        };
        let a2902 = 0u64;
        nat_eq(a2901, a2902)
    }
}

pub fn f128(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2903 = v1;
    let a2904 = 1u64;
    if if if nat_le(a2903, a2904) {
        true
    } else {
        let a2905 = v0;
        let a2906 = v2;
        nat_le(a2905, a2906)
    } {
        let a2907 = v3;
        let a2908 = 1u64;
        if nat_le(a2907, a2908) {
            true
        } else {
            let a2915 = {
                let a2913 = v0;
                let a2914 = {
                    let a2911 = {
                        let a2909 = v1;
                        let a2910 = 1u64;
                        nat_sub(a2909, a2910)
                    };
                    let a2912 = v2;
                    nat_mul(a2911, a2912)?
                };
                nat_add(a2913, a2914)?
            };
            let a2916 = v4;
            nat_le(a2915, a2916)
        }
    } else {
        false
    } {
        let a2917 = v5;
        let a2918 = 1u64;
        if nat_le(a2917, a2918) {
            Ok(true)
        } else {
            let a2931 = {
                let a2929 = {
                    let a2923 = v0;
                    let a2924 = {
                        let a2921 = {
                            let a2919 = v1;
                            let a2920 = 1u64;
                            nat_sub(a2919, a2920)
                        };
                        let a2922 = v2;
                        nat_mul(a2921, a2922)?
                    };
                    nat_add(a2923, a2924)?
                };
                let a2930 = {
                    let a2927 = {
                        let a2925 = v3;
                        let a2926 = 1u64;
                        nat_sub(a2925, a2926)
                    };
                    let a2928 = v4;
                    nat_mul(a2927, a2928)?
                };
                nat_add(a2929, a2930)?
            };
            let a2932 = v6;
            Ok(nat_le(a2931, a2932))
        }
    } else {
        Ok(false)
    }
}

pub fn f129(v0: u64) -> u64 {
    let a2933 = v0;
    let a2934 = 2147483648u64;
    if nat_lt(a2933, a2934) {
        v0
    } else {
        0u64
    }
}

pub fn f130(v0: u64) -> u64 {
    let a2935 = v0;
    let a2936 = 2147483648u64;
    if nat_lt(a2935, a2936) {
        0u64
    } else {
        let a2937 = 4294967296u64;
        let a2938 = v0;
        nat_sub(a2937, a2938)
    }
}

pub fn f131(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f159(v0, v1, v6, v7, v8) {
        let a2939 = v5;
        let a2940 = 26u64;
        nat_eq(a2939, a2940)
    } else {
        false
    } {
        let a2941 = v3;
        let a2942 = v7;
        nat_eq(a2941, a2942)
    } else {
        false
    } {
        let a2943 = v4;
        let a2944 = v8;
        nat_eq(a2943, a2944)
    } else {
        false
    } {
        let a2945 = v9;
        let a2946 = 1u64;
        nat_eq(a2945, a2946)
    } else {
        false
    } {
        let a2947 = v10;
        let a2948 = f160(v0);
        nat_eq(a2947, a2948)
    } else {
        false
    } {
        let a2949 = v11;
        let a2950 = v1;
        nat_eq(a2949, a2950)
    } else {
        false
    } {
        let a2951 = v12;
        let a2952 = v6;
        nat_eq(a2951, a2952)
    } else {
        false
    } {
        let a2953 = v13;
        let a2954 = v7;
        nat_eq(a2953, a2954)
    } else {
        false
    } {
        let a2955 = v14;
        let a2956 = v8;
        nat_eq(a2955, a2956)
    } else {
        false
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2957 = v0;
    let a2958 = 0u64;
    if if if nat_eq(a2957, a2958) {
        true
    } else {
        let a2959 = v1;
        let a2960 = 0u64;
        nat_eq(a2959, a2960)
    } {
        true
    } else {
        let a2961 = v2;
        let a2962 = 0u64;
        nat_eq(a2961, a2962)
    } {
        true
    } else {
        let a2963 = v3;
        let a2964 = 0u64;
        nat_eq(a2963, a2964)
    }
}

pub fn f133(v0: Adt3, v1: bool) -> R<u64> {
    f161(v0.clone(), {
        let a2966 = match v0.clone() {
            Adt3::C0(h2965, _, _, _, _, _, _, _) => {
                h2965
            }
        };
        let a2967 = 1u64;
        nat_sub(a2966, a2967)
    }, {
        let a2969 = match v0.clone() {
            Adt3::C0(_, h2968, _, _, _, _, _, _) => {
                h2968
            }
        };
        let a2970 = 1u64;
        nat_sub(a2969, a2970)
    }, {
        let a2972 = match v0.clone() {
            Adt3::C0(_, _, h2971, _, _, _, _, _) => {
                h2971
            }
        };
        let a2973 = 1u64;
        nat_sub(a2972, a2973)
    }, {
        let a2975 = match v0 {
            Adt3::C0(_, _, _, h2974, _, _, _, _) => {
                h2974
            }
        };
        let a2976 = 1u64;
        nat_sub(a2975, a2976)
    }, v1)
}

pub fn f134(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2977 = v0;
    let a2978 = 0u64;
    if nat_eq(a2977, a2978) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2979 = v0;
            let a2980 = 1u64;
            nat_sub(a2979, a2980)
        };
        let v5: u64 = {
            let a2981 = v4;
            let a2982 = v1;
            let a2983 = 0u64;
            nat_quot(a2981, a2982, a2983)
        };
        let v6: u64 = {
            let a2991 = {
                let a2984 = v5;
                let a2985 = v2;
                nat_mul(a2984, a2985)?
            };
            let a2992 = {
                let a2989 = {
                    let a2986 = v4;
                    let a2987 = v1;
                    let a2988 = 0u64;
                    nat_rem(a2986, a2987, a2988)
                };
                let a2990 = v3;
                nat_mul(a2989, a2990)?
            };
            nat_add(a2991, a2992)?
        };
        let a2993 = v5;
        let a2994 = 0u64;
        if nat_eq(a2993, a2994) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a3003 = {
                    let a2997 = {
                        let a2995 = v5;
                        let a2996 = 1u64;
                        nat_sub(a2995, a2996)
                    };
                    let a2998 = v2;
                    nat_mul(a2997, a2998)?
                };
                let a3004 = {
                    let a3001 = {
                        let a2999 = v1;
                        let a3000 = 1u64;
                        nat_sub(a2999, a3000)
                    };
                    let a3002 = v3;
                    nat_mul(a3001, a3002)?
                };
                nat_add(a3003, a3004)?
            };
            let a3005 = v6;
            let a3006 = v7;
            if nat_lt(a3005, a3006) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f135(v0: Adt3, v1: u64) -> bool {
    let a3007 = v1;
    let a3008 = 0u64;
    if !nat_eq(a3007, a3008) {
        true
    } else {
        let a3010 = match v0 {
            Adt3::C0(_, _, _, _, h3009, _, _, _) => {
                h3009
            }
        };
        let a3011 = 4u64;
        nat_eq(a3010, a3011)
    }
}

pub fn f136(v0: u64, v1: bool) -> R<u64> {
    let a3014 = {
        let a3012 = v0;
        let a3013 = 2u64;
        nat_mul(a3012, a3013)?
    };
    let a3015 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a3014, a3015)
}

pub fn f137(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt8> {
    let m3016 = f162(v0, v1);
    match m3016 {
        None => {
            if v5 {
                let m3017 = f163(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m3017 {
                    Adt14::C0 => {
                        Ok(Adt8::C0)
                    }
                    Adt14::C1 => {
                        Ok(Adt8::C1)
                    }
                    Adt14::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt8::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt8::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a3018 = 4u64;
                let a3019 = v28;
                nat_le(a3018, a3019)
            };
            let a3020 = v2;
            let a3021 = 0u64;
            if if if if if nat_eq(a3020, a3021) {
                let a3022 = v4;
                let a3023 = 0u64;
                nat_eq(a3022, a3023)
            } else {
                false
            } {
                let a3024 = v6;
                let a3025 = v14;
                if nat_eq(a3024, a3025) {
                    let a3026 = v7;
                    let a3027 = v15;
                    nat_eq(a3026, a3027)
                } else {
                    false
                }
            } else {
                false
            } {
                let a3028 = v8;
                let a3029 = v16;
                if nat_eq(a3028, a3029) {
                    let a3030 = v9;
                    let a3031 = v17;
                    nat_eq(a3030, a3031)
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
                        let a3032 = v3;
                        let a3033 = 0u64;
                        nat_eq(a3032, a3033)
                    } else {
                        false
                    } {
                        f80(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt8::C2(v28, v29, v6, v7, v8, v9, {
                    let a3038 = {
                        let a3034 = v6;
                        let a3035 = v7;
                        nat_mul(a3034, a3035)?
                    };
                    let a3039 = {
                        let a3036 = v8;
                        let a3037 = v9;
                        nat_mul(a3036, a3037)?
                    };
                    nat_mul(a3038, a3039)?
                }))
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f138(v0: u64) -> u64 {
    let m3040 = f79(v0);
    match m3040 {
        Adt12::C0 => {
            0u64
        }
        Adt12::C1(v1) => {
            let a3042 = match v1.clone() {
                Adt13::C0(_, h3041, _) => {
                    h3041
                }
            };
            let a3043 = 1u64;
            if if nat_eq(a3042, a3043) {
                let a3045 = match v1.clone() {
                    Adt13::C0(_, _, h3044) => {
                        h3044
                    }
                };
                let a3046 = 1u64;
                if if if nat_eq(a3045, a3046) {
                    true
                } else {
                    let a3048 = match v1.clone() {
                        Adt13::C0(_, _, h3047) => {
                            h3047
                        }
                    };
                    let a3049 = 2u64;
                    nat_eq(a3048, a3049)
                } {
                    true
                } else {
                    let a3051 = match v1.clone() {
                        Adt13::C0(_, _, h3050) => {
                            h3050
                        }
                    };
                    let a3052 = 4u64;
                    nat_eq(a3051, a3052)
                } {
                    true
                } else {
                    let a3054 = match v1.clone() {
                        Adt13::C0(_, _, h3053) => {
                            h3053
                        }
                    };
                    let a3055 = 8u64;
                    nat_eq(a3054, a3055)
                }
            } else {
                false
            } {
                match v1 {
                    Adt13::C0(_, _, h3056) => {
                        h3056
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f139(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3069 = {
        let a3065 = {
            let a3061 = {
                let a3057 = v0;
                let a3058 = v4;
                nat_mul(a3057, a3058)?
            };
            let a3062 = {
                let a3059 = v1;
                let a3060 = v5;
                nat_mul(a3059, a3060)?
            };
            nat_add(a3061, a3062)?
        };
        let a3066 = {
            let a3063 = v2;
            let a3064 = v6;
            nat_mul(a3063, a3064)?
        };
        nat_add(a3065, a3066)?
    };
    let a3070 = {
        let a3067 = v3;
        let a3068 = v7;
        nat_mul(a3067, a3068)?
    };
    nat_add(a3069, a3070)
}

pub fn f140(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3075 = {
        let a3073 = {
            let a3071 = v0;
            let a3072 = v1;
            nat_sub(a3071, a3072)
        };
        let a3074 = 1u64;
        nat_sub(a3073, a3074)
    };
    let a3076 = v2;
    nat_add(a3075, a3076)
}

pub fn f141(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f139({
        let a3077 = v0;
        let a3078 = v4;
        let a3079 = 0u64;
        nat_rem(a3077, a3078, a3079)
    }, {
        let a3080 = v1;
        let a3081 = v5;
        let a3082 = 0u64;
        nat_rem(a3080, a3081, a3082)
    }, {
        let a3083 = v2;
        let a3084 = v6;
        let a3085 = 0u64;
        nat_rem(a3083, a3084, a3085)
    }, {
        let a3086 = v3;
        let a3087 = v7;
        let a3088 = 0u64;
        nat_rem(a3086, a3087, a3088)
    }, v8, v9, v10, v11)
}

pub fn f142(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3089 = v1;
    let a3090 = 2147483648u64;
    if nat_lt(a3089, a3090) {
        let a3101 = {
            let a3099 = {
                let a3094 = {
                    let a3091 = v0;
                    let a3092 = v2;
                    let a3093 = 0u64;
                    nat_rem(a3091, a3092, a3093)
                };
                let a3095 = v2;
                nat_add(a3094, a3095)?
            };
            let a3100 = {
                let a3096 = v1;
                let a3097 = v2;
                let a3098 = 0u64;
                nat_rem(a3096, a3097, a3098)
            };
            nat_sub(a3099, a3100)
        };
        let a3102 = v2;
        let a3103 = 0u64;
        Ok(nat_rem(a3101, a3102, a3103))
    } else {
        let a3106 = {
            let a3104 = v0;
            let a3105 = f130(v1);
            nat_add(a3104, a3105)?
        };
        let a3107 = v2;
        let a3108 = 0u64;
        Ok(nat_rem(a3106, a3107, a3108))
    }
}

pub fn f143(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3111 = f129(v1);
    let a3112 = {
        let a3109 = v0;
        let a3110 = f130(v1);
        nat_add(a3109, a3110)?
    };
    if nat_le(a3111, a3112) {
        let a3113 = f144(v0, v1)?;
        let a3114 = v2;
        Ok(nat_lt(a3113, a3114))
    } else {
        Ok(false)
    }
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a3117 = {
        let a3115 = v0;
        let a3116 = f130(v1);
        nat_add(a3115, a3116)?
    };
    let a3118 = f129(v1);
    Ok(nat_sub(a3117, a3118))
}

pub fn f145(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3119 = v0;
    let a3120 = v1;
    if nat_lt(a3119, a3120) {
        let a3121 = v1;
        let a3122 = v0;
        Ok(nat_sub(a3121, a3122))
    } else {
        let a3125 = v0;
        let a3126 = {
            let a3123 = v1;
            let a3124 = v2;
            nat_add(a3123, a3124)?
        };
        if nat_lt(a3125, a3126) {
            let a3127 = v0;
            let a3128 = v1;
            Ok(nat_sub(a3127, a3128))
        } else {
            let a3135 = {
                let a3131 = 2u64;
                let a3132 = {
                    let a3129 = v2;
                    let a3130 = 1u64;
                    nat_sub(a3129, a3130)
                };
                nat_mul(a3131, a3132)?
            };
            let a3136 = {
                let a3133 = v0;
                let a3134 = v1;
                nat_sub(a3133, a3134)
            };
            Ok(nat_sub(a3135, a3136))
        }
    }
}

pub fn f146(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3139 = {
        let a3137 = v2;
        let a3138 = f161(v1.clone(), f119(v0, v1.clone(), 0u64), f119(v0, v1.clone(), 1u64), f119(v0, v1.clone(), 2u64), f119(v0, v1.clone(), 3u64), false)?;
        nat_add(a3137, a3138)?
    };
    let a3140 = f161(v1.clone(), f119(v0, v1.clone(), 0u64), f119(v0, v1.clone(), 1u64), f119(v0, v1.clone(), 2u64), f119(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3139, a3140))
}

pub fn f147(v0: u64, v1: u64, v2: u64) -> bool {
    let a3141 = v0;
    let a3142 = 0u64;
    if nat_eq(a3141, a3142) {
        let a3143 = v2;
        let a3144 = v1;
        nat_le(a3143, a3144)
    } else {
        let a3145 = v0;
        let a3146 = 1u64;
        if nat_eq(a3145, a3146) {
            let a3147 = v2;
            let a3148 = v1;
            nat_lt(a3147, a3148)
        } else {
            let a3149 = v0;
            let a3150 = 2u64;
            if nat_eq(a3149, a3150) {
                let a3151 = v1;
                let a3152 = v2;
                nat_le(a3151, a3152)
            } else {
                let a3153 = v0;
                let a3154 = 3u64;
                if nat_eq(a3153, a3154) {
                    let a3155 = v1;
                    let a3156 = v2;
                    nat_lt(a3155, a3156)
                } else {
                    let a3157 = v1;
                    let a3158 = v2;
                    nat_eq(a3157, a3158)
                }
            }
        }
    }
}

pub fn f148(v0: u64, v1: Adt3, v2: u64) -> R<Adt7> {
    let a3159 = v0;
    let a3160 = 0u64;
    if nat_eq(a3159, a3160) {
        Ok(f156(0u64))
    } else {
        let a3163 = {
            let a3161 = v0;
            let a3162 = 1u64;
            nat_add(a3161, a3162)?
        };
        let a3164 = f91(v1.clone(), 13u64, v2)?;
        if nat_eq(a3163, a3164) {
            Ok(Adt7::C4({
                let a3165 = v2;
                let a3166 = 0u64;
                if nat_eq(a3165, a3166) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3167 = v0;
                let a3168 = 1u64;
                nat_sub(a3167, a3168)
            };
            if f135(v1.clone(), v2) {
                let v4: u64 = f136(match v1.clone() {
                    Adt3::C0(h3169, _, _, _, _, _, _, _) => {
                        h3169
                    }
                }, false)?;
                let v5: u64 = {
                    let a3170 = v3;
                    let a3171 = v4;
                    let a3172 = 0u64;
                    nat_quot(a3170, a3171, a3172)
                };
                let v6: u64 = {
                    let a3173 = v3;
                    let a3174 = v4;
                    let a3175 = 0u64;
                    nat_rem(a3173, a3174, a3175)
                };
                let a3178 = {
                    let a3176 = v6;
                    let a3177 = 1u64;
                    nat_add(a3176, a3177)?
                };
                let a3179 = v4;
                if nat_eq(a3178, a3179) {
                    Ok(Adt7::C4({
                        let a3180 = v2;
                        let a3181 = 0u64;
                        if nat_eq(a3180, a3181) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f149(v6, v5, v1, v2)
                }
            } else {
                f164({
                    let a3182 = v3;
                    let a3183 = 2u64;
                    let a3184 = 0u64;
                    nat_quot(a3182, a3183, a3184)
                }, {
                    let a3185 = v3;
                    let a3186 = 2u64;
                    let a3187 = 0u64;
                    nat_rem(a3185, a3186, a3187)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f149(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt7> {
    let a3188 = v0;
    let a3189 = 0u64;
    if nat_eq(a3188, a3189) {
        Ok(f156(1u64))
    } else {
        f164({
            let a3198 = {
                let a3191 = v1;
                let a3192 = match v2.clone() {
                    Adt3::C0(h3190, _, _, _, _, _, _, _) => {
                        h3190
                    }
                };
                nat_mul(a3191, a3192)?
            };
            let a3199 = {
                let a3195 = {
                    let a3193 = v0;
                    let a3194 = 1u64;
                    nat_sub(a3193, a3194)
                };
                let a3196 = 2u64;
                let a3197 = 0u64;
                nat_quot(a3195, a3196, a3197)
            };
            nat_add(a3198, a3199)?
        }, {
            let a3202 = {
                let a3200 = v0;
                let a3201 = 1u64;
                nat_sub(a3200, a3201)
            };
            let a3203 = 2u64;
            let a3204 = 0u64;
            nat_rem(a3202, a3203, a3204)
        }, v2, v3, 1u64)
    }
}

pub fn f150(v0: u64, v1: u64) -> R<u64> {
    let a3205 = v0;
    let a3206 = f67(v1)?;
    let a3207 = 0u64;
    Ok(nat_rem(a3205, a3206, a3207))
}

pub fn f151(v0: u64, v1: u64) -> R<u64> {
    let a3208 = 4u64;
    let a3209 = f165(v0, v1)?;
    nat_mul(a3208, a3209)
}

pub fn f152(v0: u64, v1: u64) -> R<u64> {
    let a3210 = 4u64;
    let a3211 = f166(v0, v1)?;
    nat_mul(a3210, a3211)
}

pub fn f153(v0: u64, v1: u64) -> R<bool> {
    let a3217 = {
        let a3214 = {
            let a3212 = f150(v0, v1)?;
            let a3213 = 2u64;
            nat_sub(a3212, a3213)
        };
        let a3215 = 2u64;
        let a3216 = 0u64;
        nat_rem(a3214, a3215, a3216)
    };
    let a3218 = 0u64;
    Ok(nat_eq(a3217, a3218))
}

pub fn f154(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3219 = f79(v0);
    match m3219 {
        Adt12::C0 => {
            Ok(None::<u64>)
        }
        Adt12::C1(v13) => {
            if if if if if f167(v0) {
                let a3220 = v1;
                let a3221 = v7;
                nat_lt(a3220, a3221)
            } else {
                false
            } {
                f115(v8, v2)
            } else {
                false
            } {
                let a3222 = v3;
                let a3223 = f168(v4, v5, v6)?;
                nat_lt(a3222, a3223)
            } else {
                false
            } {
                let a3228 = {
                    let a3225 = v7;
                    let a3226 = match v13.clone() {
                        Adt13::C0(_, h3224, _) => {
                            h3224
                        }
                    };
                    let a3227 = 0u64;
                    nat_rem(a3225, a3226, a3227)
                };
                let a3229 = 0u64;
                nat_eq(a3228, a3229)
            } else {
                false
            } {
                Ok(Some({
                    let a3258 = {
                        let a3248 = {
                            let a3238 = {
                                let a3234 = {
                                    let a3231 = v1;
                                    let a3232 = match v13 {
                                        Adt13::C0(_, h3230, _) => {
                                            h3230
                                        }
                                    };
                                    let a3233 = 0u64;
                                    nat_quot(a3231, a3232, a3233)
                                };
                                let a3235 = v9;
                                nat_mul(a3234, a3235)?
                            };
                            let a3239 = {
                                let a3236 = v2;
                                let a3237 = v10;
                                nat_mul(a3236, a3237)?
                            };
                            nat_add(a3238, a3239)?
                        };
                        let a3249 = {
                            let a3246 = {
                                let a3243 = {
                                    let a3240 = v3;
                                    let a3241 = v4;
                                    let a3242 = 0u64;
                                    nat_quot(a3240, a3241, a3242)
                                };
                                let a3244 = v5;
                                let a3245 = 0u64;
                                nat_rem(a3243, a3244, a3245)
                            };
                            let a3247 = v11;
                            nat_mul(a3246, a3247)?
                        };
                        nat_add(a3248, a3249)?
                    };
                    let a3259 = {
                        let a3256 = {
                            let a3253 = {
                                let a3250 = v3;
                                let a3251 = v4;
                                let a3252 = 0u64;
                                nat_quot(a3250, a3251, a3252)
                            };
                            let a3254 = v5;
                            let a3255 = 0u64;
                            nat_quot(a3253, a3254, a3255)
                        };
                        let a3257 = v12;
                        nat_mul(a3256, a3257)?
                    };
                    nat_add(a3258, a3259)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f155(v0: u64) -> u64 {
    let a3260 = v0;
    let a3261 = 30u64;
    if nat_eq(a3260, a3261) {
        4u64
    } else {
        3u64
    }
}

pub fn f156(v0: u64) -> Adt7 {
    Adt7::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f157(v0: u64) -> bool {
    let a3262 = 0u64;
    let a3263 = v0;
    if nat_lt(a3262, a3263) {
        let a3264 = v0;
        let a3265 = 2139095040u64;
        nat_le(a3264, a3265)
    } else {
        false
    }
}

pub fn f158(v0: u64) -> u64 {
    let a3266 = v0;
    let a3267 = 40u64;
    if nat_lt(a3266, a3267) {
        let a3268 = v0;
        let a3269 = 20u64;
        if nat_lt(a3268, a3269) {
            let a3270 = v0;
            let a3271 = 10u64;
            if nat_lt(a3270, a3271) {
                let a3272 = v0;
                let a3273 = 5u64;
                if nat_lt(a3272, a3273) {
                    let a3274 = v0;
                    let a3275 = 2u64;
                    if nat_lt(a3274, a3275) {
                        let a3276 = v0;
                        let a3277 = 1u64;
                        if nat_lt(a3276, a3277) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a3278 = v0;
                        let a3279 = 3u64;
                        if nat_lt(a3278, a3279) {
                            4u64
                        } else {
                            let a3280 = v0;
                            let a3281 = 4u64;
                            if nat_lt(a3280, a3281) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a3282 = v0;
                    let a3283 = 7u64;
                    if nat_lt(a3282, a3283) {
                        let a3284 = v0;
                        let a3285 = 6u64;
                        if nat_lt(a3284, a3285) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a3286 = v0;
                        let a3287 = 8u64;
                        if nat_lt(a3286, a3287) {
                            128u64
                        } else {
                            let a3288 = v0;
                            let a3289 = 9u64;
                            if nat_lt(a3288, a3289) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a3290 = v0;
                let a3291 = 15u64;
                if nat_lt(a3290, a3291) {
                    let a3292 = v0;
                    let a3293 = 12u64;
                    if nat_lt(a3292, a3293) {
                        let a3294 = v0;
                        let a3295 = 11u64;
                        if nat_lt(a3294, a3295) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a3296 = v0;
                        let a3297 = 13u64;
                        if nat_lt(a3296, a3297) {
                            4096u64
                        } else {
                            let a3298 = v0;
                            let a3299 = 14u64;
                            if nat_lt(a3298, a3299) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a3300 = v0;
                    let a3301 = 17u64;
                    if nat_lt(a3300, a3301) {
                        let a3302 = v0;
                        let a3303 = 16u64;
                        if nat_lt(a3302, a3303) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a3304 = v0;
                        let a3305 = 18u64;
                        if nat_lt(a3304, a3305) {
                            131072u64
                        } else {
                            let a3306 = v0;
                            let a3307 = 19u64;
                            if nat_lt(a3306, a3307) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a3308 = v0;
            let a3309 = 30u64;
            if nat_lt(a3308, a3309) {
                let a3310 = v0;
                let a3311 = 25u64;
                if nat_lt(a3310, a3311) {
                    let a3312 = v0;
                    let a3313 = 22u64;
                    if nat_lt(a3312, a3313) {
                        let a3314 = v0;
                        let a3315 = 21u64;
                        if nat_lt(a3314, a3315) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a3316 = v0;
                        let a3317 = 23u64;
                        if nat_lt(a3316, a3317) {
                            4194304u64
                        } else {
                            let a3318 = v0;
                            let a3319 = 24u64;
                            if nat_lt(a3318, a3319) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a3320 = v0;
                    let a3321 = 27u64;
                    if nat_lt(a3320, a3321) {
                        let a3322 = v0;
                        let a3323 = 26u64;
                        if nat_lt(a3322, a3323) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a3324 = v0;
                        let a3325 = 28u64;
                        if nat_lt(a3324, a3325) {
                            134217728u64
                        } else {
                            let a3326 = v0;
                            let a3327 = 29u64;
                            if nat_lt(a3326, a3327) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a3328 = v0;
                let a3329 = 35u64;
                if nat_lt(a3328, a3329) {
                    let a3330 = v0;
                    let a3331 = 32u64;
                    if nat_lt(a3330, a3331) {
                        let a3332 = v0;
                        let a3333 = 31u64;
                        if nat_lt(a3332, a3333) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3334 = v0;
                        let a3335 = 33u64;
                        if nat_lt(a3334, a3335) {
                            4294967296u64
                        } else {
                            let a3336 = v0;
                            let a3337 = 34u64;
                            if nat_lt(a3336, a3337) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3338 = v0;
                    let a3339 = 37u64;
                    if nat_lt(a3338, a3339) {
                        let a3340 = v0;
                        let a3341 = 36u64;
                        if nat_lt(a3340, a3341) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3342 = v0;
                        let a3343 = 38u64;
                        if nat_lt(a3342, a3343) {
                            137438953472u64
                        } else {
                            let a3344 = v0;
                            let a3345 = 39u64;
                            if nat_lt(a3344, a3345) {
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

pub fn f159(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f132(v1, v2, v3, v4) {
        f169(v0)
    } else {
        f167(v0)
    }
}

pub fn f160(v0: u64) -> u64 {
    let a3346 = v0;
    let a3347 = 26u64;
    if nat_eq(a3346, a3347) {
        26u64
    } else {
        0u64
    }
}

pub fn f161(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3368 = {
        let a3362 = {
            let a3356 = {
                let a3350 = v1;
                let a3351 = if v5 {
                    f130(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3348, _, _, _) => {
                            h3348
                        }
                    })
                } else {
                    f129(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3349, _, _, _) => {
                            h3349
                        }
                    })
                };
                nat_mul(a3350, a3351)?
            };
            let a3357 = {
                let a3354 = v2;
                let a3355 = if v5 {
                    f130(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3352, _, _) => {
                            h3352
                        }
                    })
                } else {
                    f129(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3353, _, _) => {
                            h3353
                        }
                    })
                };
                nat_mul(a3354, a3355)?
            };
            nat_add(a3356, a3357)?
        };
        let a3363 = {
            let a3360 = v3;
            let a3361 = if v5 {
                f130(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3358, _) => {
                        h3358
                    }
                })
            } else {
                f129(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3359, _) => {
                        h3359
                    }
                })
            };
            nat_mul(a3360, a3361)?
        };
        nat_add(a3362, a3363)?
    };
    let a3369 = {
        let a3366 = v4;
        let a3367 = if v5 {
            f130(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3364) => {
                    h3364
                }
            })
        } else {
            f129(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3365) => {
                    h3365
                }
            })
        };
        nat_mul(a3366, a3367)?
    };
    nat_add(a3368, a3369)
}

pub fn f162(v0: u64, v1: u64) -> Option<u64> {
    let a3370 = v0;
    let a3371 = 2u64;
    if nat_eq(a3370, a3371) {
        Some(0u64)
    } else {
        let a3372 = v0;
        let a3373 = 5u64;
        if nat_eq(a3372, a3373) {
            Some(1u64)
        } else {
            let a3374 = v0;
            let a3375 = 6u64;
            if nat_eq(a3374, a3375) {
                Some(2u64)
            } else {
                let a3376 = v0;
                let a3377 = 7u64;
                if nat_eq(a3376, a3377) {
                    Some(3u64)
                } else {
                    let a3378 = v0;
                    let a3379 = 9u64;
                    if nat_eq(a3378, a3379) {
                        Some(4u64)
                    } else {
                        let a3380 = v0;
                        let a3381 = 8u64;
                        if nat_eq(a3380, a3381) {
                            Some(5u64)
                        } else {
                            let a3382 = v0;
                            let a3383 = 89u64;
                            if nat_eq(a3382, a3383) {
                                let a3384 = v1;
                                let a3385 = 2u64;
                                if nat_eq(a3384, a3385) {
                                    Some(6u64)
                                } else {
                                    let a3386 = v1;
                                    let a3387 = 0u64;
                                    if nat_eq(a3386, a3387) {
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

pub fn f163(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt14> {
    let a3388 = v0;
    let a3389 = 30u64;
    if nat_eq(a3388, a3389) {
        let a3390 = v2;
        let a3391 = 0u64;
        if if if nat_eq(a3390, a3391) {
            true
        } else {
            let a3392 = v2;
            let a3393 = 1u64;
            nat_eq(a3392, a3393)
        } {
            let a3394 = v1;
            let a3395 = 0u64;
            if nat_eq(a3394, a3395) {
                true
            } else {
                f170(v1, v4)
            }
        } else {
            false
        } {
            f171(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt14::C0)
        }
    } else {
        if if if f172(v1) {
            f172(v2)
        } else {
            false
        } {
            let a3396 = v1;
            let a3397 = 0u64;
            if if nat_eq(a3396, a3397) {
                true
            } else {
                let a3398 = v2;
                let a3399 = 0u64;
                nat_eq(a3398, a3399)
            } {
                true
            } else {
                let a3400 = v1;
                let a3401 = v2;
                nat_eq(a3400, a3401)
            }
        } else {
            false
        } {
            f171(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt14::C0)
        }
    }
}

pub fn f164(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt7> {
    let a3402 = v1;
    let a3403 = 0u64;
    if nat_eq(a3402, a3403) {
        Ok(Adt7::C4({
            let a3404 = v3;
            let a3405 = 0u64;
            if nat_eq(a3404, a3405) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f98(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt7::C4({
            let a3406 = v3;
            let a3407 = 0u64;
            if nat_eq(a3406, a3407) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f165(v0: u64, v1: u64) -> R<u64> {
    let a3408 = v0;
    let a3409 = f67(v1)?;
    let a3410 = 0u64;
    Ok(nat_quot(a3408, a3409, a3410))
}

pub fn f166(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f165(v0, v1)?;
    let v3: u64 = {
        let a3413 = {
            let a3411 = f150(v0, v1)?;
            let a3412 = 2u64;
            nat_sub(a3411, a3412)
        };
        let a3414 = 2u64;
        let a3415 = 0u64;
        nat_quot(a3413, a3414, a3415)
    };
    let v4: u64 = {
        let a3416 = v2;
        let a3417 = v1;
        nat_mul(a3416, a3417)?
    };
    if f153(v0, v1)? {
        let a3418 = v4;
        let a3419 = v3;
        nat_add(a3418, a3419)
    } else {
        let a3431 = {
            let a3427 = v4;
            let a3428 = {
                let a3425 = {
                    let a3420 = v1;
                    let a3421 = 1u64;
                    nat_sub(a3420, a3421)
                };
                let a3426 = {
                    let a3422 = v2;
                    let a3423 = v1;
                    let a3424 = 0u64;
                    nat_rem(a3422, a3423, a3424)
                };
                nat_mul(a3425, a3426)?
            };
            nat_sub(a3427, a3428)
        };
        let a3432 = {
            let a3429 = v3;
            let a3430 = v1;
            nat_mul(a3429, a3430)?
        };
        nat_add(a3431, a3432)
    }
}

pub fn f167(v0: u64) -> bool {
    let a3433 = v0;
    let a3434 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3433, a3434) {
        true
    } else {
        let a3435 = v0;
        let a3436 = 1u64;
        nat_eq(a3435, a3436)
    } {
        true
    } else {
        let a3437 = v0;
        let a3438 = 2u64;
        nat_eq(a3437, a3438)
    } {
        true
    } else {
        let a3439 = v0;
        let a3440 = 3u64;
        nat_eq(a3439, a3440)
    } {
        true
    } else {
        let a3441 = v0;
        let a3442 = 6u64;
        nat_eq(a3441, a3442)
    } {
        true
    } else {
        let a3443 = v0;
        let a3444 = 7u64;
        nat_eq(a3443, a3444)
    } {
        true
    } else {
        let a3445 = v0;
        let a3446 = 8u64;
        nat_eq(a3445, a3446)
    } {
        true
    } else {
        let a3447 = v0;
        let a3448 = 10u64;
        nat_eq(a3447, a3448)
    } {
        true
    } else {
        let a3449 = v0;
        let a3450 = 11u64;
        nat_eq(a3449, a3450)
    } {
        true
    } else {
        let a3451 = v0;
        let a3452 = 12u64;
        nat_eq(a3451, a3452)
    } {
        true
    } else {
        let a3453 = v0;
        let a3454 = 13u64;
        nat_eq(a3453, a3454)
    } {
        true
    } else {
        let a3455 = v0;
        let a3456 = 14u64;
        nat_eq(a3455, a3456)
    } {
        true
    } else {
        let a3457 = v0;
        let a3458 = 16u64;
        nat_eq(a3457, a3458)
    } {
        true
    } else {
        let a3459 = v0;
        let a3460 = 17u64;
        nat_eq(a3459, a3460)
    } {
        true
    } else {
        let a3461 = v0;
        let a3462 = 18u64;
        nat_eq(a3461, a3462)
    } {
        true
    } else {
        let a3463 = v0;
        let a3464 = 19u64;
        nat_eq(a3463, a3464)
    } {
        true
    } else {
        let a3465 = v0;
        let a3466 = 20u64;
        nat_eq(a3465, a3466)
    } {
        true
    } else {
        let a3467 = v0;
        let a3468 = 21u64;
        nat_eq(a3467, a3468)
    } {
        true
    } else {
        let a3469 = v0;
        let a3470 = 22u64;
        nat_eq(a3469, a3470)
    } {
        true
    } else {
        let a3471 = v0;
        let a3472 = 23u64;
        nat_eq(a3471, a3472)
    } {
        true
    } else {
        let a3473 = v0;
        let a3474 = 26u64;
        nat_eq(a3473, a3474)
    } {
        true
    } else {
        let a3475 = v0;
        let a3476 = 29u64;
        nat_eq(a3475, a3476)
    } {
        true
    } else {
        let a3477 = v0;
        let a3478 = 30u64;
        nat_eq(a3477, a3478)
    } {
        true
    } else {
        let a3479 = v0;
        let a3480 = 34u64;
        nat_eq(a3479, a3480)
    } {
        true
    } else {
        let a3481 = v0;
        let a3482 = 35u64;
        nat_eq(a3481, a3482)
    } {
        true
    } else {
        let a3483 = v0;
        let a3484 = 39u64;
        nat_eq(a3483, a3484)
    } {
        true
    } else {
        let a3485 = v0;
        let a3486 = 40u64;
        nat_eq(a3485, a3486)
    } {
        true
    } else {
        let a3487 = v0;
        let a3488 = 41u64;
        nat_eq(a3487, a3488)
    } {
        true
    } else {
        let a3489 = v0;
        let a3490 = 42u64;
        nat_eq(a3489, a3490)
    } {
        true
    } else {
        let a3491 = v0;
        let a3492 = 43u64;
        nat_eq(a3491, a3492)
    }
}

pub fn f168(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3493 = v0;
    let a3494 = 0u64;
    if if if nat_eq(a3493, a3494) {
        true
    } else {
        let a3495 = v1;
        let a3496 = 0u64;
        nat_eq(a3495, a3496)
    } {
        true
    } else {
        let a3497 = v2;
        let a3498 = 0u64;
        nat_eq(a3497, a3498)
    } {
        Ok(0u64)
    } else {
        let a3501 = {
            let a3499 = v0;
            let a3500 = v1;
            nat_mul(a3499, a3500)?
        };
        let a3502 = v2;
        nat_mul(a3501, a3502)
    }
}

pub fn f169(v0: u64) -> bool {
    let m3503 = f79(v0);
    match m3503 {
        Adt12::C0 => {
            false
        }
        Adt12::C1(_) => {
            true
        }
    }
}

pub fn f170(v0: u64, v1: u64) -> bool {
    let a3504 = v0;
    let a3505 = 1u64;
    if nat_eq(a3504, a3505) {
        true
    } else {
        let v2: u64 = f173(v0);
        let a3506 = 0u64;
        let a3507 = v2;
        if nat_lt(a3506, a3507) {
            let a3511 = {
                let a3508 = v1;
                let a3509 = v2;
                let a3510 = 1u64;
                nat_rem(a3508, a3509, a3510)
            };
            let a3512 = 0u64;
            nat_eq(a3511, a3512)
        } else {
            false
        }
    }
}

pub fn f171(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt14> {
    let a3513 = v0;
    let a3514 = 28u64;
    if if if if nat_eq(a3513, a3514) {
        let a3515 = v1;
        let a3516 = 0u64;
        nat_eq(a3515, a3516)
    } else {
        false
    } {
        let a3517 = v2;
        let a3518 = 0u64;
        nat_eq(a3517, a3518)
    } else {
        false
    } {
        let a3519 = v3;
        let a3520 = 0u64;
        nat_eq(a3519, a3520)
    } else {
        false
    } {
        let a3521 = v4;
        let a3522 = v8;
        if if if if if if if if nat_eq(a3521, a3522) {
            let a3523 = v5;
            let a3524 = v12;
            nat_eq(a3523, a3524)
        } else {
            false
        } {
            let a3525 = v9;
            let a3526 = v13;
            nat_eq(a3525, a3526)
        } else {
            false
        } {
            let a3527 = v10;
            let a3528 = v14;
            nat_eq(a3527, a3528)
        } else {
            false
        } {
            let a3529 = v11;
            let a3530 = v15;
            nat_eq(a3529, a3530)
        } else {
            false
        } {
            let a3531 = 0u64;
            let a3532 = v6;
            nat_lt(a3531, a3532)
        } else {
            false
        } {
            let a3533 = 0u64;
            let a3534 = v7;
            nat_lt(a3533, a3534)
        } else {
            false
        } {
            let a3538 = {
                let a3535 = v10;
                let a3536 = v6;
                let a3537 = 1u64;
                nat_rem(a3535, a3536, a3537)
            };
            let a3539 = 0u64;
            if nat_eq(a3538, a3539) {
                let a3543 = {
                    let a3540 = v11;
                    let a3541 = v7;
                    let a3542 = 1u64;
                    nat_rem(a3540, a3541, a3542)
                };
                let a3544 = 0u64;
                nat_eq(a3543, a3544)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt14::C2(v5, v4, v9, v10, v11, {
                let a3545 = v10;
                let a3546 = v6;
                let a3547 = 0u64;
                nat_quot(a3545, a3546, a3547)
            }, {
                let a3548 = v11;
                let a3549 = v7;
                let a3550 = 0u64;
                nat_quot(a3548, a3549, a3550)
            }, {
                let a3551 = v5;
                let a3552 = v4;
                nat_mul(a3551, a3552)?
            }, {
                let a3553 = v4;
                let a3554 = v9;
                nat_mul(a3553, a3554)?
            }, {
                let a3555 = v5;
                let a3556 = v9;
                nat_mul(a3555, a3556)?
            }))
        } else {
            Ok(Adt14::C1)
        }
    } else {
        Ok(Adt14::C0)
    }
}

pub fn f172(v0: u64) -> bool {
    let a3557 = v0;
    let a3558 = 0u64;
    if if nat_eq(a3557, a3558) {
        true
    } else {
        let a3559 = v0;
        let a3560 = 1u64;
        nat_eq(a3559, a3560)
    } {
        true
    } else {
        let a3561 = v0;
        let a3562 = 30u64;
        nat_eq(a3561, a3562)
    }
}

pub fn f173(v0: u64) -> u64 {
    if f167(v0) {
        let m3563 = f79(v0);
        match m3563 {
            Adt12::C0 => {
                0u64
            }
            Adt12::C1(v1) => {
                let a3565 = 1u64;
                let a3566 = match v1.clone() {
                    Adt13::C0(_, h3564, _) => {
                        h3564
                    }
                };
                if if nat_lt(a3565, a3566) {
                    true
                } else {
                    let a3567 = v0;
                    let a3568 = 43u64;
                    nat_eq(a3567, a3568)
                } {
                    match v1 {
                        Adt13::C0(_, h3569, _) => {
                            h3569
                        }
                    }
                } else {
                    0u64
                }
            }
        }
    } else {
        0u64
    }
}
