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
    C4(Adt7, u64, u64, u64),
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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, Adt3, Adt3, Adt3, Adt6, Adt6),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt9),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
    C6(u64, u64, u64, bool),
    C7(Adt10, Adt11, Adt11, Adt12, u64, u64),
    C8(Option<u64>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
    C5(u64),
    C6(Adt7),
}

#[derive(Clone)]
pub enum Adt10 {
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
pub enum Adt11 {
    C0(u64, u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64),
    C1(u64),
    C2,
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1(Adt14),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt8> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6) => {
            let a2 = v1;
            let a3 = 64u64;
            if nat_eq(a2, a3) {
                Ok(Adt8::C0(f1(v2, v6)?))
            } else {
                let m4 = v3;
                match m4 {
                    None => {
                        Ok(Adt8::C0(Adt9::C1))
                    }
                    Some(v7) => {
                        let a5 = v1;
                        let a6 = 53u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt8::C0(f2(v2, v7, v4, v6)?))
                        } else {
                            let a7 = v1;
                            let a8 = 78u64;
                            if nat_eq(a7, a8) {
                                Ok(Adt8::C0(f3(v2, v7, v4, v5, v6)?))
                            } else {
                                let a9 = v1;
                                let a10 = 23u64;
                                if if if nat_eq(a9, a10) {
                                    true
                                } else {
                                    let a11 = v1;
                                    let a12 = 26u64;
                                    nat_eq(a11, a12)
                                } {
                                    true
                                } else {
                                    let a13 = v1;
                                    let a14 = 25u64;
                                    nat_eq(a13, a14)
                                } {
                                    Ok(Adt8::C0(Adt9::C5(64u64)))
                                } else {
                                    let a15 = v1;
                                    let a16 = 24u64;
                                    if nat_eq(a15, a16) {
                                        Ok(Adt8::C0(f4(v2, v7, v6)?))
                                    } else {
                                        let a17 = v1;
                                        let a18 = 27u64;
                                        if nat_eq(a17, a18) {
                                            Ok(Adt8::C0(f5(v2, v7, v6)?))
                                        } else {
                                            let a19 = v1;
                                            let a20 = 3u64;
                                            if nat_eq(a19, a20) {
                                                Ok(Adt8::C0(f6(v7, v4, v5, v6)?))
                                            } else {
                                                f7(v1, v2, v7, v4, v6)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v12, v13, v14, v15, v16) => {
            f9(v12, v13, v14, v15, v16)
        }
        Adt0::C2(v17) => {
            f10(v17)
        }
        Adt0::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) => {
            Ok(f11(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30))
        }
        Adt0::C4(v8, v9, v10, v11) => {
            f8(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt9> {
    let v2: u64 = f12(v1.clone(), 1u64)?;
    let a22 = match v1.clone() {
        Adt2::C0(h21, _, _, _, _, _, _, _, _, _, _) => {
            h21
        }
    };
    let a23 = 0u64;
    if if if if if if if if nat_eq(a22, a23) {
        let a25 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h24, _, _, _, _, _) => {
                h24
            }
        };
        let a26 = 4u64;
        nat_eq(a25, a26)
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(h27, _, _, _, _, _, _, _, _, _) => {
                h27
            }
        })
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(_, h28, _, _, _, _, _, _, _, _) => {
                h28
            }
        })
    } else {
        false
    } {
        let a30 = 0u64;
        let a31 = match v0.clone() {
            Adt1::C0(_, _, h29, _, _, _, _, _, _, _) => {
                h29
            }
        };
        nat_lt(a30, a31)
    } else {
        false
    } {
        let a33 = match v0.clone() {
            Adt1::C0(_, _, h32, _, _, _, _, _, _, _) => {
                h32
            }
        };
        let a34 = 2139095040u64;
        nat_lt(a33, a34)
    } else {
        false
    } {
        let a35 = v2;
        let a36 = 9223372036854775808u64;
        nat_lt(a35, a36)
    } else {
        false
    } {
        let a38 = f14(v2)?;
        let a39 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h37, _) => {
                h37
            }
        };
        nat_le(a38, a39)
    } else {
        false
    } {
        Ok(Adt9::C4(f15(v2)?, 4u64, f16(v1.clone(), 1u64), f16(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h40, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        }, match v0.clone() {
            Adt1::C0(_, h41, _, _, _, _, _, _, _, _) => {
                h41
            }
        }, match v0 {
            Adt1::C0(_, _, h42, _, _, _, _, _, _, _) => {
                h42
            }
        }, v2)))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m43 = v2;
    match m43 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h44, _, _, _, _, _, _, _, _, _) => {
                    h44
                }
            }, match v0.clone() {
                Adt1::C0(h45, _, _, _, _, _, _, _, _, _) => {
                    h45
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h46, _, _, _, _, _, _, _) => {
                    h46
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h47, _, _, _, _, _) => {
                    h47
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h48, _, _, _, _, _, _, _, _) => {
                    h48
                }
            }, match v0.clone() {
                Adt1::C0(_, h49, _, _, _, _, _, _, _, _) => {
                    h49
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h50, _, _, _, _, _, _) => {
                    h50
                }
            }, match v0 {
                Adt1::C0(_, _, _, _, _, h51, _, _, _, _) => {
                    h51
                }
            });
            let v7: u64 = f17(v5.clone(), match v4.clone() {
                Adt2::C0(_, h52, _, _, _, _, _, _, _, _, _) => {
                    h52
                }
            })?;
            let v8: u64 = f17(v6.clone(), match v4.clone() {
                Adt2::C0(_, _, h53, _, _, _, _, _, _, _, _) => {
                    h53
                }
            })?;
            let v9: u64 = {
                let a57 = {
                    let a54 = v7;
                    let a55 = v8;
                    nat_mul(a54, a55)?
                };
                let a58 = match v4.clone() {
                    Adt2::C0(_, _, _, _, h56, _, _, _, _, _, _) => {
                        h56
                    }
                };
                nat_mul(a57, a58)?
            };
            let v10: u64 = match v1.clone() {
                Adt2::C0(_, _, _, _, h59, _, _, _, _, _, _) => {
                    h59
                }
            };
            let v11: u64 = {
                let a65 = {
                    let a62 = match v5.clone() {
                        Adt6::C0(h60, _, _, _) => {
                            h60
                        }
                    };
                    let a63 = match v6.clone() {
                        Adt6::C0(h61, _, _, _) => {
                            h61
                        }
                    };
                    nat_mul(a62, a63)?
                };
                let a66 = match v1.clone() {
                    Adt2::C0(_, _, _, h64, _, _, _, _, _, _, _) => {
                        h64
                    }
                };
                nat_mul(a65, a66)?
            };
            let v12: u64 = {
                let a67 = v9;
                let a68 = 32u64;
                if nat_lt(a67, a68) {
                    v9
                } else {
                    32u64
                }
            };
            let a70 = match v1.clone() {
                Adt2::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                    h69
                }
            };
            let a71 = 0u64;
            if if if if if if if if if if if if if if nat_eq(a70, a71) {
                true
            } else {
                let a73 = match v1.clone() {
                    Adt2::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                        h72
                    }
                };
                let a74 = 1u64;
                nat_eq(a73, a74)
            } {
                let a76 = match v4.clone() {
                    Adt2::C0(h75, _, _, _, _, _, _, _, _, _, _) => {
                        h75
                    }
                };
                let a77 = 0u64;
                nat_eq(a76, a77)
            } else {
                false
            } {
                let a79 = match v3.clone() {
                    Adt2::C0(h78, _, _, _, _, _, _, _, _, _, _) => {
                        h78
                    }
                };
                let a80 = 0u64;
                nat_eq(a79, a80)
            } else {
                false
            } {
                let a83 = match v1.clone() {
                    Adt2::C0(_, _, _, h81, _, _, _, _, _, _, _) => {
                        h81
                    }
                };
                let a84 = match v4.clone() {
                    Adt2::C0(_, _, _, h82, _, _, _, _, _, _, _) => {
                        h82
                    }
                };
                nat_eq(a83, a84)
            } else {
                false
            } {
                let a86 = match v3.clone() {
                    Adt2::C0(_, h85, _, _, _, _, _, _, _, _, _) => {
                        h85
                    }
                };
                let a87 = v7;
                nat_eq(a86, a87)
            } else {
                false
            } {
                let a89 = match v3.clone() {
                    Adt2::C0(_, _, h88, _, _, _, _, _, _, _, _) => {
                        h88
                    }
                };
                let a90 = v8;
                nat_eq(a89, a90)
            } else {
                false
            } {
                let a92 = match v3.clone() {
                    Adt2::C0(_, _, _, h91, _, _, _, _, _, _, _) => {
                        h91
                    }
                };
                let a93 = v10;
                nat_eq(a92, a93)
            } else {
                false
            } {
                let a96 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h94, _, _, _, _, _, _) => {
                        h94
                    }
                };
                let a97 = match v4.clone() {
                    Adt2::C0(_, _, _, _, h95, _, _, _, _, _, _) => {
                        h95
                    }
                };
                nat_eq(a96, a97)
            } else {
                false
            } {
                let a98 = 0u64;
                let a99 = v11;
                nat_lt(a98, a99)
            } else {
                false
            } {
                let a100 = 0u64;
                let a101 = v10;
                nat_lt(a100, a101)
            } else {
                false
            } {
                let a102 = 0u64;
                let a103 = v9;
                nat_lt(a102, a103)
            } else {
                false
            } {
                f18(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt9::C6(Adt7::C0(v9, v11, v10, {
                    let a104 = v11;
                    let a105 = v10;
                    nat_mul(a104, a105)?
                }, v12, {
                    let a106 = v12;
                    let a107 = v11;
                    nat_mul(a106, a107)?
                }, {
                    let a108 = v12;
                    let a109 = v10;
                    nat_mul(a108, a109)?
                }, match v1.clone() {
                    Adt2::C0(h110, _, _, _, _, _, _, _, _, _, _) => {
                        h110
                    }
                }, f16(v1, 1u64), f16(v4, 1u64), f16(v3, 1u64), v5, v6)))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let m111 = v2;
    match m111 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v5) => {
            let m112 = v3;
            match m112 {
                None => {
                    Ok(Adt9::C1)
                }
                Some(v6) => {
                    let a114 = match v1.clone() {
                        Adt2::C0(h113, _, _, _, _, _, _, _, _, _, _) => {
                            h113
                        }
                    };
                    let a115 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a114, a115) {
                        let a117 = match v5.clone() {
                            Adt2::C0(h116, _, _, _, _, _, _, _, _, _, _) => {
                                h116
                            }
                        };
                        let a118 = 0u64;
                        nat_eq(a117, a118)
                    } else {
                        false
                    } {
                        let a120 = match v6.clone() {
                            Adt2::C0(h119, _, _, _, _, _, _, _, _, _, _) => {
                                h119
                            }
                        };
                        let a121 = 0u64;
                        nat_eq(a120, a121)
                    } else {
                        false
                    } {
                        let a123 = match v4.clone() {
                            Adt2::C0(h122, _, _, _, _, _, _, _, _, _, _) => {
                                h122
                            }
                        };
                        let a124 = 0u64;
                        nat_eq(a123, a124)
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
                        let a130 = match v1.clone() {
                            Adt2::C0(_, h125, _, _, _, _, _, _, _, _, _) => {
                                h125
                            }
                        };
                        let a131 = {
                            let a128 = match v5.clone() {
                                Adt2::C0(_, h126, _, _, _, _, _, _, _, _, _) => {
                                    h126
                                }
                            };
                            let a129 = match v5.clone() {
                                Adt2::C0(_, h127, _, _, _, _, _, _, _, _, _) => {
                                    h127
                                }
                            };
                            nat_mul(a128, a129)?
                        };
                        nat_eq(a130, a131)
                    } else {
                        false
                    } {
                        let a137 = match v1.clone() {
                            Adt2::C0(_, _, h132, _, _, _, _, _, _, _, _) => {
                                h132
                            }
                        };
                        let a138 = {
                            let a135 = match v5.clone() {
                                Adt2::C0(_, _, h133, _, _, _, _, _, _, _, _) => {
                                    h133
                                }
                            };
                            let a136 = match v5.clone() {
                                Adt2::C0(_, _, _, h134, _, _, _, _, _, _, _) => {
                                    h134
                                }
                            };
                            nat_mul(a135, a136)?
                        };
                        nat_eq(a137, a138)
                    } else {
                        false
                    } {
                        let a141 = match v1.clone() {
                            Adt2::C0(_, _, _, h139, _, _, _, _, _, _, _) => {
                                h139
                            }
                        };
                        let a142 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h140, _, _, _, _, _, _) => {
                                h140
                            }
                        };
                        nat_eq(a141, a142)
                    } else {
                        false
                    } {
                        let a144 = match v0.clone() {
                            Adt1::C0(h143, _, _, _, _, _, _, _, _, _) => {
                                h143
                            }
                        };
                        let a145 = 4294967296u64;
                        nat_lt(a144, a145)
                    } else {
                        false
                    } {
                        let a147 = match v0.clone() {
                            Adt1::C0(h146, _, _, _, _, _, _, _, _, _) => {
                                h146
                            }
                        };
                        let a148 = 0u64;
                        if nat_eq(a147, a148) {
                            if f18(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a151 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h149) => {
                                        h149
                                    }
                                };
                                let a152 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h150) => {
                                        h150
                                    }
                                };
                                nat_eq(a151, a152)
                            }
                        } else {
                            let a155 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h153) => {
                                    h153
                                }
                            };
                            let a156 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h154) => {
                                    h154
                                }
                            };
                            nat_eq(a155, a156)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a158 = match v4.clone() {
                                Adt2::C0(_, h157, _, _, _, _, _, _, _, _, _) => {
                                    h157
                                }
                            };
                            let a159 = 0u64;
                            if if if nat_eq(a158, a159) {
                                true
                            } else {
                                let a161 = match v4.clone() {
                                    Adt2::C0(_, _, h160, _, _, _, _, _, _, _, _) => {
                                        h160
                                    }
                                };
                                let a162 = 0u64;
                                nat_eq(a161, a162)
                            } {
                                true
                            } else {
                                let a164 = match v4.clone() {
                                    Adt2::C0(_, _, _, h163, _, _, _, _, _, _, _) => {
                                        h163
                                    }
                                };
                                let a165 = 0u64;
                                nat_eq(a164, a165)
                            } {
                                true
                            } else {
                                let a167 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h166, _, _, _, _, _, _) => {
                                        h166
                                    }
                                };
                                let a168 = 0u64;
                                nat_eq(a167, a168)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
                                h169
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a171 = match v0 {
                                Adt1::C0(h170, _, _, _, _, _, _, _, _, _) => {
                                    h170
                                }
                            };
                            let a172 = 0u64;
                            if nat_eq(a171, a172) {
                                f12(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f21(v8, match v5.clone() {
                                Adt2::C0(_, _, h173, _, _, _, _, _, _, _, _) => {
                                    h173
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h174, _, _, _, _, _, _, _) => {
                                    h174
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h175, _, _, _, _, _, _) => {
                                    h175
                                }
                            })?
                        };
                        Ok(Adt9::C4(f22(v9, v8, v10)?, 4u64, f16(v1, 1u64), f16(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt9::C1)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt9> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h176, _, _, _, _, _, _, _, _, _, _) => {
            h176
        }
    };
    let v4: u64 = f23(v3);
    let a178 = match v1.clone() {
        Adt2::C0(h177, _, _, _, _, _, _, _, _, _, _) => {
            h177
        }
    };
    let a179 = 0u64;
    if if if if if if if if nat_eq(a178, a179) {
        true
    } else {
        let a181 = match v1.clone() {
            Adt2::C0(h180, _, _, _, _, _, _, _, _, _, _) => {
                h180
            }
        };
        let a182 = 30u64;
        nat_eq(a181, a182)
    } {
        let a185 = match v1.clone() {
            Adt2::C0(h183, _, _, _, _, _, _, _, _, _, _) => {
                h183
            }
        };
        let a186 = match v2.clone() {
            Adt2::C0(h184, _, _, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        nat_eq(a185, a186)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f24(match v0.clone() {
            Adt1::C0(h187, _, _, _, _, _, _, _, _, _) => {
                h187
            }
        })
    } else {
        false
    } {
        f25(v1.clone(), v4)
    } else {
        false
    } {
        f25(v2.clone(), v4)
    } else {
        false
    } {
        if f18(v1.clone(), v2.clone())? {
            true
        } else {
            f26(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f27(match v1.clone() {
            Adt2::C0(_, h188, _, _, _, _, _, _, _, _, _) => {
                h188
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h189, _, _, _, _, _, _, _, _) => {
                h189
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h190, _, _, _, _, _, _, _) => {
                h190
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h191, _, _, _, _, _, _) => {
                h191
            }
        }) {
            0u64
        } else {
            let a197 = match v1.clone() {
                Adt2::C0(_, _, h192, _, _, _, _, _, _, _, _) => {
                    h192
                }
            };
            let a198 = {
                let a195 = match v1.clone() {
                    Adt2::C0(_, _, _, h193, _, _, _, _, _, _, _) => {
                        h193
                    }
                };
                let a196 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h194, _, _, _, _, _, _) => {
                        h194
                    }
                };
                nat_mul(a195, a196)?
            };
            nat_mul(a197, a198)?
        };
        let v6: u64 = {
            let a199 = v5;
            let a200 = 0u64;
            if if nat_eq(a199, a200) {
                true
            } else {
                f28(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a202 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h201, _) => {
                        h201
                    }
                };
                let a203 = v4;
                let a204 = 0u64;
                nat_quot(a202, a203, a204)
            }
        };
        Ok(Adt9::C4(f29(v3, match v1.clone() {
            Adt2::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
                h205
            }
        }, v5, v6)?, v4, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h206, _, _, _, _, _, _, _, _, _) => {
                h206
            }
        }, v5, f30(match v1 {
            Adt2::C0(_, h207, _, _, _, _, _, _, _, _, _) => {
                h207
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt9> {
    let a209 = match v1.clone() {
        Adt2::C0(h208, _, _, _, _, _, _, _, _, _, _) => {
            h208
        }
    };
    let a210 = 0u64;
    if if if if if nat_eq(a209, a210) {
        let a212 = match v2.clone() {
            Adt2::C0(h211, _, _, _, _, _, _, _, _, _, _) => {
                h211
            }
        };
        let a213 = 0u64;
        nat_eq(a212, a213)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f31(match v0.clone() {
            Adt1::C0(h214, _, _, _, _, _, _, _, _, _) => {
                h214
            }
        })
    } else {
        false
    } {
        if f18(v1.clone(), v2.clone())? {
            true
        } else {
            f26(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f16(v1, 1u64);
        let v4: u64 = {
            let a220 = match v3.clone() {
                Adt3::C0(_, h215, _, _, _, _, _, _) => {
                    h215
                }
            };
            let a221 = {
                let a218 = match v3.clone() {
                    Adt3::C0(_, _, h216, _, _, _, _, _) => {
                        h216
                    }
                };
                let a219 = match v3.clone() {
                    Adt3::C0(_, _, _, h217, _, _, _, _) => {
                        h217
                    }
                };
                nat_mul(a218, a219)?
            };
            nat_mul(a220, a221)?
        };
        Ok(Adt9::C4(f32(match v3.clone() {
            Adt3::C0(h222, _, _, _, _, _, _, _) => {
                h222
            }
        }, v4)?, 4u64, v3, f16(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h223, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        })))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f6(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m224 = v1;
    match m224 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let m225 = v2;
            match m225 {
                None => {
                    Ok(Adt9::C1)
                }
                Some(v5) => {
                    let a227 = match v0.clone() {
                        Adt2::C0(h226, _, _, _, _, _, _, _, _, _, _) => {
                            h226
                        }
                    };
                    let a228 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a227, a228) {
                        let a230 = match v4.clone() {
                            Adt2::C0(h229, _, _, _, _, _, _, _, _, _, _) => {
                                h229
                            }
                        };
                        let a231 = 0u64;
                        nat_eq(a230, a231)
                    } else {
                        false
                    } {
                        let a233 = match v5.clone() {
                            Adt2::C0(h232, _, _, _, _, _, _, _, _, _, _) => {
                                h232
                            }
                        };
                        let a234 = 26u64;
                        nat_eq(a233, a234)
                    } else {
                        false
                    } {
                        let a236 = match v3.clone() {
                            Adt2::C0(h235, _, _, _, _, _, _, _, _, _, _) => {
                                h235
                            }
                        };
                        let a237 = 0u64;
                        nat_eq(a236, a237)
                    } else {
                        false
                    } {
                        f19(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a240 = match v0.clone() {
                            Adt2::C0(_, h238, _, _, _, _, _, _, _, _, _) => {
                                h238
                            }
                        };
                        let a241 = match v4.clone() {
                            Adt2::C0(_, h239, _, _, _, _, _, _, _, _, _) => {
                                h239
                            }
                        };
                        nat_eq(a240, a241)
                    } else {
                        false
                    } {
                        let a244 = match v0.clone() {
                            Adt2::C0(_, _, h242, _, _, _, _, _, _, _, _) => {
                                h242
                            }
                        };
                        let a245 = match v5.clone() {
                            Adt2::C0(_, h243, _, _, _, _, _, _, _, _, _) => {
                                h243
                            }
                        };
                        nat_eq(a244, a245)
                    } else {
                        false
                    } {
                        let a248 = match v0.clone() {
                            Adt2::C0(_, _, _, h246, _, _, _, _, _, _, _) => {
                                h246
                            }
                        };
                        let a249 = match v5.clone() {
                            Adt2::C0(_, _, h247, _, _, _, _, _, _, _, _) => {
                                h247
                            }
                        };
                        nat_eq(a248, a249)
                    } else {
                        false
                    } {
                        let a251 = 0u64;
                        let a252 = match v5.clone() {
                            Adt2::C0(_, _, _, h250, _, _, _, _, _, _, _) => {
                                h250
                            }
                        };
                        nat_lt(a251, a252)
                    } else {
                        false
                    } {
                        let a254 = 0u64;
                        let a255 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h253, _, _, _, _, _, _) => {
                                h253
                            }
                        };
                        nat_lt(a254, a255)
                    } else {
                        false
                    } {
                        let a257 = 4u64;
                        let a258 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h256, _, _, _, _, _) => {
                                h256
                            }
                        };
                        nat_le(a257, a258)
                    } else {
                        false
                    } {
                        let a260 = 4u64;
                        let a261 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h259, _, _, _, _, _) => {
                                h259
                            }
                        };
                        nat_le(a260, a261)
                    } else {
                        false
                    } {
                        let a263 = 4u64;
                        let a264 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h262, _, _, _, _, _) => {
                                h262
                            }
                        };
                        nat_le(a263, a264)
                    } else {
                        false
                    } {
                        let a266 = 4u64;
                        let a267 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h265, _, _, _, _, _) => {
                                h265
                            }
                        };
                        nat_le(a266, a267)
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
                        Ok(Adt9::C4(f33(match v0.clone() {
                            Adt2::C0(_, h268, _, _, _, _, _, _, _, _, _) => {
                                h268
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h269, _, _, _, _, _, _, _, _) => {
                                h269
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h270, _, _, _, _, _, _, _) => {
                                h270
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h271, _, _, _, _, _, _) => {
                                h271
                            }
                        })?, 4u64, f16(v0, 1u64), f16(v3, 1u64), Adt4::C22(f16(v4, 1u64), f16(v5, 1u64))))
                    } else {
                        Ok(Adt9::C1)
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h272, _, _, _, _, _, _, _, _, _) => {
            h272
        }
    };
    Ok(Adt8::C0({
        let a273 = v0;
        let a274 = 49u64;
        if nat_eq(a273, a274) {
            f34(v1, v2, v3, v4)?
        } else {
            let a275 = v0;
            let a276 = 51u64;
            if nat_eq(a275, a276) {
                f35(v1, v2, v3, v4)?
            } else {
                let a277 = v0;
                let a278 = 39u64;
                if nat_eq(a277, a278) {
                    f36(v2, v3, v4)?
                } else {
                    let a279 = v0;
                    let a280 = 77u64;
                    if nat_eq(a279, a280) {
                        f37(v2, v4)?
                    } else {
                        let a281 = v0;
                        let a282 = 40u64;
                        if nat_eq(a281, a282) {
                            f38(v2, v3, v4)?
                        } else {
                            if f39(v0) {
                                f40(v0, v2, {
                                    let m283 = v3;
                                    match m283 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }, v4)?
                            } else {
                                let a284 = v0;
                                let a285 = 19u64;
                                if nat_eq(a284, a285) {
                                    f41(v2, v4)?
                                } else {
                                    let a286 = v0;
                                    let a287 = 21u64;
                                    if nat_eq(a286, a287) {
                                        f42(v5, v2, v3, v4)?
                                    } else {
                                        let a288 = v0;
                                        let a289 = 61u64;
                                        if if if nat_eq(a288, a289) {
                                            true
                                        } else {
                                            let a290 = v0;
                                            let a291 = 62u64;
                                            nat_eq(a290, a291)
                                        } {
                                            true
                                        } else {
                                            let a292 = v0;
                                            let a293 = 63u64;
                                            nat_eq(a292, a293)
                                        } {
                                            f43(v0, v1, v2, v4)?
                                        } else {
                                            let a294 = v0;
                                            let a295 = 4u64;
                                            if if nat_eq(a294, a295) {
                                                true
                                            } else {
                                                let a296 = v0;
                                                let a297 = 32u64;
                                                nat_eq(a296, a297)
                                            } {
                                                f44(v0, v1, v2, v3, v4)?
                                            } else {
                                                let a298 = v0;
                                                let a299 = 20u64;
                                                if nat_eq(a298, a299) {
                                                    f45(v2, v4)?
                                                } else {
                                                    let a300 = v0;
                                                    let a301 = 42u64;
                                                    if nat_eq(a300, a301) {
                                                        f46(4u64, v2, v4)?
                                                    } else {
                                                        let a302 = v0;
                                                        let a303 = 69u64;
                                                        if nat_eq(a302, a303) {
                                                            let a304 = v5;
                                                            let a305 = 4u64;
                                                            if nat_lt(a304, a305) {
                                                                f46(v5, v2, v4)?
                                                            } else {
                                                                Adt9::C1
                                                            }
                                                        } else {
                                                            let a306 = v0;
                                                            let a307 = 70u64;
                                                            if nat_eq(a306, a307) {
                                                                f47(v5, v2, v4)?
                                                            } else {
                                                                let a308 = v0;
                                                                let a309 = 75u64;
                                                                if if nat_eq(a308, a309) {
                                                                    true
                                                                } else {
                                                                    let a310 = v0;
                                                                    let a311 = 76u64;
                                                                    nat_eq(a310, a311)
                                                                } {
                                                                    f48({
                                                                        let a312 = v0;
                                                                        let a313 = 76u64;
                                                                        nat_eq(a312, a313)
                                                                    }, v1, v2, v4)?
                                                                } else {
                                                                    let a314 = v0;
                                                                    let a315 = 31u64;
                                                                    if if if nat_eq(a314, a315) {
                                                                        true
                                                                    } else {
                                                                        let a316 = v0;
                                                                        let a317 = 47u64;
                                                                        nat_eq(a316, a317)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a318 = v0;
                                                                        let a319 = 68u64;
                                                                        nat_eq(a318, a319)
                                                                    } {
                                                                        f49(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a320 = v0;
                                                                        let a321 = 15u64;
                                                                        if nat_eq(a320, a321) {
                                                                            f50(v2, v4)?
                                                                        } else {
                                                                            let a322 = v0;
                                                                            let a323 = 17u64;
                                                                            if nat_eq(a322, a323) {
                                                                                f51(v2, v4)?
                                                                            } else {
                                                                                let a324 = v0;
                                                                                let a325 = 18u64;
                                                                                if nat_eq(a324, a325) {
                                                                                    f52(v2, v3, v4)?
                                                                                } else {
                                                                                    let a326 = v0;
                                                                                    let a327 = 13u64;
                                                                                    if if if nat_eq(a326, a327) {
                                                                                        true
                                                                                    } else {
                                                                                        let a328 = v0;
                                                                                        let a329 = 14u64;
                                                                                        nat_eq(a328, a329)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a330 = v0;
                                                                                        let a331 = 16u64;
                                                                                        nat_eq(a330, a331)
                                                                                    } {
                                                                                        f53(v0, v2, v4)?
                                                                                    } else {
                                                                                        let m332 = f54(v0, v5)?;
                                                                                        match m332 {
                                                                                            None => {
                                                                                                f55(v0, v5, v2, v3, v4)?
                                                                                            }
                                                                                            Some(v7) => {
                                                                                                f56(v7, v2, v4)?
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f8(v0: Adt7, v1: u64, v2: u64, v3: u64) -> R<Adt8> {
    let a333 = v1;
    let a334 = 0u64;
    if nat_eq(a333, a334) {
        let a336 = v3;
        let a337 = match v0.clone() {
            Adt7::C0(_, _, _, h335, _, _, _, _, _, _, _, _, _) => {
                h335
            }
        };
        if nat_lt(a336, a337) {
            let v4: Adt3 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, h338, _, _, _, _) => {
                    h338
                }
            };
            let v5: u64 = {
                let a340 = v3;
                let a341 = match v0.clone() {
                    Adt7::C0(_, _, h339, _, _, _, _, _, _, _, _, _, _) => {
                        h339
                    }
                };
                let a342 = 0u64;
                nat_quot(a340, a341, a342)
            };
            Ok(Adt8::C8(Some(f57(v4.clone(), f58(v5, v4.clone(), 0u64), f58(v5, v4.clone(), 1u64), f58(v5, v4, 2u64), {
                let a344 = v3;
                let a345 = match v0 {
                    Adt7::C0(_, _, h343, _, _, _, _, _, _, _, _, _, _) => {
                        h343
                    }
                };
                let a346 = 0u64;
                nat_rem(a344, a345, a346)
            })?)))
        } else {
            Ok(Adt8::C0(Adt9::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h347, _, _) => {
                h347
            }
        };
        let v7: u64 = {
            let a348 = v1;
            let a349 = 1u64;
            if nat_eq(a348, a349) {
                match v0.clone() {
                    Adt7::C0(_, h350, _, _, _, _, _, _, _, _, _, _, _) => {
                        h350
                    }
                }
            } else {
                match v0.clone() {
                    Adt7::C0(_, _, h351, _, _, _, _, _, _, _, _, _, _) => {
                        h351
                    }
                }
            }
        };
        let v8: u64 = {
            let a355 = v2;
            let a356 = {
                let a352 = v3;
                let a353 = v7;
                let a354 = 0u64;
                nat_quot(a352, a353, a354)
            };
            nat_add(a355, a356)?
        };
        let v9: u64 = {
            let a357 = v3;
            let a358 = v7;
            let a359 = 0u64;
            nat_rem(a357, a358, a359)
        };
        let v10: u64 = {
            let a361 = v8;
            let a362 = match v6.clone() {
                Adt3::C0(h360, _, _, _, _, _, _, _) => {
                    h360
                }
            };
            let a363 = 0u64;
            nat_rem(a361, a362, a363)
        };
        let v11: u64 = {
            let a365 = v8;
            let a366 = match v6.clone() {
                Adt3::C0(h364, _, _, _, _, _, _, _) => {
                    h364
                }
            };
            let a367 = 0u64;
            nat_quot(a365, a366, a367)
        };
        let v12: u64 = {
            let a369 = v11;
            let a370 = match v6.clone() {
                Adt3::C0(_, h368, _, _, _, _, _, _) => {
                    h368
                }
            };
            let a371 = 0u64;
            nat_rem(a369, a370, a371)
        };
        let v13: u64 = {
            let a373 = v11;
            let a374 = match v6.clone() {
                Adt3::C0(_, h372, _, _, _, _, _, _) => {
                    h372
                }
            };
            let a375 = 0u64;
            nat_quot(a373, a374, a375)
        };
        let a377 = v8;
        let a378 = match v0.clone() {
            Adt7::C0(h376, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        };
        if nat_lt(a377, a378) {
            let a379 = v1;
            let a380 = 1u64;
            if nat_eq(a379, a380) {
                let v14: Adt3 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, h381, _, _, _, _) => {
                        h381
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h382, _, _, _) => {
                        h382
                    }
                };
                let m385 = f59(match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h383, _) => {
                        h383
                    }
                }, match v15.clone() {
                    Adt3::C0(h384, _, _, _, _, _, _, _) => {
                        h384
                    }
                }, v10, f58(v9, v14.clone(), 0u64))?;
                match m385 {
                    None => {
                        Ok(Adt8::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m388 = f59(match v0 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h386) => {
                                h386
                            }
                        }, match v15.clone() {
                            Adt3::C0(_, h387, _, _, _, _, _, _) => {
                                h387
                            }
                        }, v12, f58(v9, v14.clone(), 1u64))?;
                        match m388 {
                            None => {
                                Ok(Adt8::C8(None::<u64>))
                            }
                            Some(v17) => {
                                Ok(Adt8::C8(Some(f57(v15, v16, v17, f58(v9, v14, 2u64), v13)?)))
                            }
                        }
                    }
                }
            } else {
                let a389 = v1;
                let a390 = 2u64;
                if nat_eq(a389, a390) {
                    Ok(Adt8::C8(Some(f57(v6, v10, v12, v9, v13)?)))
                } else {
                    Ok(Adt8::C0(Adt9::C1))
                }
            }
        } else {
            Ok(Adt8::C0(Adt9::C1))
        }
    }
}

pub fn f9(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt8> {
    Ok(f60(v3.clone(), {
        let m391 = v3.clone();
        match m391 {
            Adt4::C0 => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f62(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f63(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f64(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f65(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v58, v59, v60, v61) => {
                f68(v0, v61, v58, v59, v60, v4)?
            }
            Adt4::C24(_) => {
                Adt8::C0(Adt9::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f61(v0, v1, v2, v3)?
            }
            Adt4::C26(v53, v54, v55, v56) => {
                let a392 = v53;
                let a393 = 64u64;
                if if if nat_eq(a392, a393) {
                    true
                } else {
                    let a394 = v53;
                    let a395 = 65u64;
                    nat_eq(a394, a395)
                } {
                    true
                } else {
                    let a396 = v53;
                    let a397 = 66u64;
                    nat_eq(a396, a397)
                } {
                    Adt8::C0(Adt9::C5(64u64))
                } else {
                    let a398 = v53;
                    let a399 = 0u64;
                    if if nat_eq(a398, a399) {
                        true
                    } else {
                        let a400 = v53;
                        let a401 = 30u64;
                        nat_eq(a400, a401)
                    } {
                        f67(v0, v1, v2, v53, v54, v55, v56, v4)?
                    } else {
                        Adt8::C0(Adt9::C1)
                    }
                }
            }
            Adt4::C27(v52) => {
                f66(v0, v1, v2, v52)?
            }
        }
    }))
}

pub fn f10(v0: Adt4) -> R<Adt8> {
    let m402 = v0;
    match m402 {
        Adt4::C0 => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C1 => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C2(_, _) => {
            Ok(Adt8::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C3(_, _, _, _, _) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C4(_) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C5(_, _, _, _) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C6(_, _, _) => {
            Ok(Adt8::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C7(_) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C8(_) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C9(v18, v19) => {
            Ok(Adt8::C6(0u64, v18, v19, true))
        }
        Adt4::C10(_, v21, _) => {
            Ok(Adt8::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false))
        }
        Adt4::C11(_, _) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C12(_, _, _) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C13(_, _, _, _) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C14(_, _, _) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C15 => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C16(_, _) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C17 => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C18(_, _) => {
            Ok(Adt8::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C19(_, _) => {
            Ok(Adt8::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C20(_, _) => {
            Ok(Adt8::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C21 => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C22(_, _) => {
            Ok(Adt8::C6(7u64, 0u64, 0u64, false))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt8::C6(0u64, 0u64, 0u64, false))
        }
        Adt4::C24(_) => {
            Ok(Adt8::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C25(v45, _, v47) => {
            Ok(Adt8::C6({
                let a407 = {
                    let a403 = 0u64;
                    let a404 = v45;
                    if nat_lt(a403, a404) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a408 = {
                    let a405 = 0u64;
                    let a406 = v47;
                    if nat_lt(a405, a406) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a407, a408)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v49, _, _, _) => {
            Ok(Adt8::C6({
                let a409 = v49;
                let a410 = 66u64;
                if nat_eq(a409, a410) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false))
        }
        Adt4::C27(_) => {
            Ok(Adt8::C6(2u64, 0u64, 0u64, false))
        }
    }
}

pub fn f11(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt8 {
    let a411 = v1;
    let a412 = 5u64;
    if if if if nat_lt(a411, a412) {
        let a413 = v5;
        let a414 = 5u64;
        nat_lt(a413, a414)
    } else {
        false
    } {
        let a415 = v7;
        let a416 = 5u64;
        nat_lt(a415, a416)
    } else {
        false
    } {
        let a417 = v9;
        let a418 = 2u64;
        if nat_lt(a417, a418) {
            true
        } else {
            let a419 = v9;
            let a420 = 3u64;
            nat_eq(a419, a420)
        }
    } else {
        false
    } {
        Adt8::C7(f69(v0, v1, v2, v3, v4), f70(v5, v6), f70(v7, v8), f71({
            let a421 = v9;
            let a422 = 0u64;
            if nat_eq(a421, a422) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt8::C0(Adt9::C1)
    }
}

pub fn f12(v0: Adt2, v1: u64) -> R<u64> {
    let a434 = {
        let a428 = {
            let a424 = match v0.clone() {
                Adt2::C0(_, h423, _, _, _, _, _, _, _, _, _) => {
                    h423
                }
            };
            let a425 = v1;
            let a426 = 0u64;
            nat_quot(a424, a425, a426)
        };
        let a429 = match v0.clone() {
            Adt2::C0(_, _, h427, _, _, _, _, _, _, _, _) => {
                h427
            }
        };
        nat_mul(a428, a429)?
    };
    let a435 = {
        let a432 = match v0.clone() {
            Adt2::C0(_, _, _, h430, _, _, _, _, _, _, _) => {
                h430
            }
        };
        let a433 = match v0 {
            Adt2::C0(_, _, _, _, h431, _, _, _, _, _, _) => {
                h431
            }
        };
        nat_mul(a432, a433)?
    };
    nat_mul(a434, a435)
}

pub fn f13(v0: u64) -> bool {
    let a436 = v0;
    let a437 = 4294967296u64;
    if nat_lt(a436, a437) {
        let a441 = {
            let a438 = v0;
            let a439 = 2147483648u64;
            let a440 = 0u64;
            nat_rem(a438, a439, a440)
        };
        let a442 = 2139095040u64;
        nat_lt(a441, a442)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a443 = 4u64;
    let a444 = v0;
    nat_mul(a443, a444)
}

pub fn f15(v0: u64) -> R<u64> {
    let a445 = v0;
    let a446 = 0u64;
    if nat_eq(a445, a446) {
        Ok(0u64)
    } else {
        let a449 = 4u64;
        let a450 = {
            let a447 = 3u64;
            let a448 = v0;
            nat_mul(a447, a448)?
        };
        nat_add(a449, a450)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a452 = match v0.clone() {
            Adt2::C0(_, h451, _, _, _, _, _, _, _, _, _) => {
                h451
            }
        };
        let a453 = v1;
        let a454 = 0u64;
        nat_quot(a452, a453, a454)
    }, match v0.clone() {
        Adt2::C0(_, _, h455, _, _, _, _, _, _, _, _) => {
            h455
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h456, _, _, _, _, _, _, _) => {
            h456
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h457, _, _, _, _, _, _) => {
            h457
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h458, _, _, _, _, _) => {
            h458
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h459, _, _, _, _) => {
            h459
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h460, _, _, _) => {
            h460
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h461, _, _) => {
            h461
        }
    })
}

pub fn f17(v0: Adt6, v1: u64) -> R<u64> {
    let a463 = 0u64;
    let a464 = match v0.clone() {
        Adt6::C0(h462, _, _, _) => {
            h462
        }
    };
    if if if if if nat_lt(a463, a464) {
        let a466 = 0u64;
        let a467 = match v0.clone() {
            Adt6::C0(_, h465, _, _) => {
                h465
            }
        };
        nat_lt(a466, a467)
    } else {
        false
    } {
        let a469 = 0u64;
        let a470 = match v0.clone() {
            Adt6::C0(_, _, _, h468) => {
                h468
            }
        };
        nat_lt(a469, a470)
    } else {
        false
    } {
        let a472 = match v0.clone() {
            Adt6::C0(_, h471, _, _) => {
                h471
            }
        };
        let a473 = 2147483648u64;
        nat_lt(a472, a473)
    } else {
        false
    } {
        let a475 = match v0.clone() {
            Adt6::C0(_, _, _, h474) => {
                h474
            }
        };
        let a476 = 2147483648u64;
        nat_lt(a475, a476)
    } else {
        false
    } {
        let v2: u64 = {
            let a480 = v1;
            let a481 = {
                let a478 = 2u64;
                let a479 = f72(match v0.clone() {
                    Adt6::C0(_, _, h477, _) => {
                        h477
                    }
                });
                nat_mul(a478, a479)?
            };
            nat_add(a480, a481)?
        };
        let v3: u64 = {
            let a493 = {
                let a488 = 1u64;
                let a489 = {
                    let a486 = match v0.clone() {
                        Adt6::C0(_, _, _, h482) => {
                            h482
                        }
                    };
                    let a487 = {
                        let a484 = match v0.clone() {
                            Adt6::C0(h483, _, _, _) => {
                                h483
                            }
                        };
                        let a485 = 1u64;
                        nat_sub(a484, a485)
                    };
                    nat_mul(a486, a487)?
                };
                nat_add(a488, a489)?
            };
            let a494 = {
                let a491 = 2u64;
                let a492 = f73(match v0.clone() {
                    Adt6::C0(_, _, h490, _) => {
                        h490
                    }
                });
                nat_mul(a491, a492)?
            };
            nat_add(a493, a494)?
        };
        let a495 = v3;
        let a496 = v2;
        if nat_le(a495, a496) {
            let a503 = 1u64;
            let a504 = {
                let a500 = {
                    let a497 = v2;
                    let a498 = v3;
                    nat_sub(a497, a498)
                };
                let a501 = match v0 {
                    Adt6::C0(_, h499, _, _) => {
                        h499
                    }
                };
                let a502 = 0u64;
                nat_quot(a500, a501, a502)
            };
            nat_add(a503, a504)
        } else {
            let a508 = {
                let a505 = v3;
                let a506 = v2;
                nat_sub(a505, a506)
            };
            let a509 = match v0 {
                Adt6::C0(_, h507, _, _) => {
                    h507
                }
            };
            if nat_lt(a508, a509) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f18(v0: Adt2, v1: Adt2) -> R<bool> {
    let a511 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h510, _) => {
            h510
        }
    };
    let a512 = 0u64;
    if nat_eq(a511, a512) {
        Ok(true)
    } else {
        let a514 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h513, _) => {
                h513
            }
        };
        let a515 = 0u64;
        if nat_eq(a514, a515) {
            Ok(true)
        } else {
            let a517 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h516) => {
                    h516
                }
            };
            let a518 = 0u64;
            if nat_eq(a517, a518) {
                Ok(true)
            } else {
                let a520 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h519) => {
                        h519
                    }
                };
                let a521 = 0u64;
                if nat_eq(a520, a521) {
                    Ok(true)
                } else {
                    let a527 = {
                        let a524 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h522) => {
                                h522
                            }
                        };
                        let a525 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h523, _) => {
                                h523
                            }
                        };
                        nat_add(a524, a525)?
                    };
                    let a528 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h526) => {
                            h526
                        }
                    };
                    if nat_le(a527, a528) {
                        Ok(true)
                    } else {
                        let a534 = {
                            let a531 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h529) => {
                                    h529
                                }
                            };
                            let a532 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h530, _) => {
                                    h530
                                }
                            };
                            nat_add(a531, a532)?
                        };
                        let a535 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h533) => {
                                h533
                            }
                        };
                        Ok(nat_le(a534, a535))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a538 = match v0.clone() {
        Adt2::C0(_, h536, _, _, _, _, _, _, _, _, _) => {
            h536
        }
    };
    let a539 = match v1.clone() {
        Adt2::C0(_, h537, _, _, _, _, _, _, _, _, _) => {
            h537
        }
    };
    if if if nat_eq(a538, a539) {
        let a542 = match v0.clone() {
            Adt2::C0(_, _, h540, _, _, _, _, _, _, _, _) => {
                h540
            }
        };
        let a543 = match v1.clone() {
            Adt2::C0(_, _, h541, _, _, _, _, _, _, _, _) => {
                h541
            }
        };
        nat_eq(a542, a543)
    } else {
        false
    } {
        let a546 = match v0.clone() {
            Adt2::C0(_, _, _, h544, _, _, _, _, _, _, _) => {
                h544
            }
        };
        let a547 = match v1.clone() {
            Adt2::C0(_, _, _, h545, _, _, _, _, _, _, _) => {
                h545
            }
        };
        nat_eq(a546, a547)
    } else {
        false
    } {
        let a550 = match v0 {
            Adt2::C0(_, _, _, _, h548, _, _, _, _, _, _) => {
                h548
            }
        };
        let a551 = match v1 {
            Adt2::C0(_, _, _, _, h549, _, _, _, _, _, _) => {
                h549
            }
        };
        nat_eq(a550, a551)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a553 = match v0.clone() {
        Adt2::C0(_, h552, _, _, _, _, _, _, _, _, _) => {
            h552
        }
    };
    let a554 = 1u64;
    if if if if nat_eq(a553, a554) {
        true
    } else {
        let a556 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h555, _, _, _, _, _) => {
                h555
            }
        };
        let a557 = 4u64;
        nat_eq(a556, a557)
    } {
        let a559 = match v0.clone() {
            Adt2::C0(_, _, h558, _, _, _, _, _, _, _, _) => {
                h558
            }
        };
        let a560 = 1u64;
        if nat_eq(a559, a560) {
            true
        } else {
            let a565 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h561, _, _, _, _) => {
                    h561
                }
            };
            let a566 = {
                let a563 = match v0.clone() {
                    Adt2::C0(_, h562, _, _, _, _, _, _, _, _, _) => {
                        h562
                    }
                };
                let a564 = 4u64;
                nat_mul(a563, a564)?
            };
            nat_eq(a565, a566)
        }
    } else {
        false
    } {
        let a568 = match v0.clone() {
            Adt2::C0(_, _, _, h567, _, _, _, _, _, _, _) => {
                h567
            }
        };
        let a569 = 1u64;
        if nat_eq(a568, a569) {
            true
        } else {
            let a577 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h570, _, _, _) => {
                    h570
                }
            };
            let a578 = {
                let a575 = {
                    let a573 = match v0.clone() {
                        Adt2::C0(_, h571, _, _, _, _, _, _, _, _, _) => {
                            h571
                        }
                    };
                    let a574 = match v0.clone() {
                        Adt2::C0(_, _, h572, _, _, _, _, _, _, _, _) => {
                            h572
                        }
                    };
                    nat_mul(a573, a574)?
                };
                let a576 = 4u64;
                nat_mul(a575, a576)?
            };
            nat_eq(a577, a578)
        }
    } else {
        false
    } {
        let a580 = match v0.clone() {
            Adt2::C0(_, _, _, _, h579, _, _, _, _, _, _) => {
                h579
            }
        };
        let a581 = 1u64;
        if nat_eq(a580, a581) {
            Ok(true)
        } else {
            let a592 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h582, _, _) => {
                    h582
                }
            };
            let a593 = {
                let a590 = {
                    let a588 = {
                        let a585 = match v0.clone() {
                            Adt2::C0(_, h583, _, _, _, _, _, _, _, _, _) => {
                                h583
                            }
                        };
                        let a586 = match v0.clone() {
                            Adt2::C0(_, _, h584, _, _, _, _, _, _, _, _) => {
                                h584
                            }
                        };
                        nat_mul(a585, a586)?
                    };
                    let a589 = match v0 {
                        Adt2::C0(_, _, _, h587, _, _, _, _, _, _, _) => {
                            h587
                        }
                    };
                    nat_mul(a588, a589)?
                };
                let a591 = 4u64;
                nat_mul(a590, a591)?
            };
            Ok(nat_eq(a592, a593))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a598 = {
        let a594 = v0;
        let a595 = v1;
        nat_mul(a594, a595)?
    };
    let a599 = {
        let a596 = v2;
        let a597 = f74(v3);
        nat_mul(a596, a597)?
    };
    nat_mul(a598, a599)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a602 = v0;
    let a603 = {
        let a600 = f75(v1)?;
        let a601 = v2;
        nat_mul(a600, a601)?
    };
    nat_add(a602, a603)
}

pub fn f23(v0: u64) -> u64 {
    let a604 = v0;
    let a605 = 30u64;
    if nat_eq(a604, a605) {
        2u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64) -> bool {
    let a606 = v0;
    let a607 = 2139095040u64;
    if nat_le(a606, a607) {
        true
    } else {
        let a608 = v0;
        let a609 = 2147483648u64;
        nat_eq(a608, a609)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> bool {
    let a611 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h610, _, _, _, _, _) => {
            h610
        }
    };
    let a612 = v1;
    if if nat_eq(a611, a612) {
        let a617 = {
            let a614 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h613) => {
                    h613
                }
            };
            let a615 = v1;
            let a616 = 0u64;
            nat_rem(a614, a615, a616)
        };
        let a618 = 0u64;
        nat_eq(a617, a618)
    } else {
        false
    } {
        f76(match v0.clone() {
            Adt2::C0(_, _, h619, _, _, _, _, _, _, _, _) => {
                h619
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h620, _, _, _, _, _, _, _) => {
                h620
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h621, _, _, _, _, _, _) => {
                h621
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h622, _, _, _, _) => {
                h622
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h623, _, _, _) => {
                h623
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h624, _, _) => {
                h624
            }
        }, v1)
    } else {
        false
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a627 = match v0.clone() {
            Adt2::C0(h625, _, _, _, _, _, _, _, _, _, _) => {
                h625
            }
        };
        let a628 = match v1.clone() {
            Adt2::C0(h626, _, _, _, _, _, _, _, _, _, _) => {
                h626
            }
        };
        nat_eq(a627, a628)
    } else {
        false
    } {
        let a631 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h629) => {
                h629
            }
        };
        let a632 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h630) => {
                h630
            }
        };
        nat_eq(a631, a632)
    } else {
        false
    } {
        let a635 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h633, _, _, _, _, _) => {
                h633
            }
        };
        let a636 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h634, _, _, _, _, _) => {
                h634
            }
        };
        nat_eq(a635, a636)
    } else {
        false
    } {
        let a639 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h637, _, _, _, _) => {
                h637
            }
        };
        let a640 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h638, _, _, _, _) => {
                h638
            }
        };
        nat_eq(a639, a640)
    } else {
        false
    } {
        let a643 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h641, _, _, _) => {
                h641
            }
        };
        let a644 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h642, _, _, _) => {
                h642
            }
        };
        nat_eq(a643, a644)
    } else {
        false
    } {
        let a647 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h645, _, _) => {
                h645
            }
        };
        let a648 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h646, _, _) => {
                h646
            }
        };
        nat_eq(a647, a648)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a649 = v0;
    let a650 = 0u64;
    if if if nat_eq(a649, a650) {
        true
    } else {
        let a651 = v1;
        let a652 = 0u64;
        nat_eq(a651, a652)
    } {
        true
    } else {
        let a653 = v2;
        let a654 = 0u64;
        nat_eq(a653, a654)
    } {
        true
    } else {
        let a655 = v3;
        let a656 = 0u64;
        nat_eq(a655, a656)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<bool> {
    f77({
        let a658 = match v0.clone() {
            Adt2::C0(_, h657, _, _, _, _, _, _, _, _, _) => {
                h657
            }
        };
        let a659 = v1;
        nat_mul(a658, a659)?
    }, match v0.clone() {
        Adt2::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
            h660
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h661, _, _, _, _, _, _, _) => {
            h661
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h662, _, _, _, _, _, _) => {
            h662
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h663, _, _, _, _) => {
            h663
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h664, _, _, _) => {
            h664
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h665, _, _) => {
            h665
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a666 = v1;
    let a667 = 0u64;
    if if nat_eq(a666, a667) {
        true
    } else {
        let a668 = v2;
        let a669 = 0u64;
        nat_eq(a668, a669)
    } {
        Ok(0u64)
    } else {
        let a676 = {
            let a670 = f78(v1, v2, v3)?;
            let a671 = f79(v0, v1, v2)?;
            nat_add(a670, a671)?
        };
        let a677 = {
            let a674 = f80(v0);
            let a675 = {
                let a672 = v1;
                let a673 = v2;
                nat_mul(a672, a673)?
            };
            nat_mul(a674, a675)?
        };
        nat_add(a676, a677)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a678 = v0;
    let a679 = 0u64;
    if if nat_eq(a678, a679) {
        true
    } else {
        let a680 = v1;
        let a681 = 0u64;
        nat_eq(a680, a681)
    } {
        Ok(3u64)
    } else {
        let a684 = {
            let a682 = 3u64;
            let a683 = v1;
            nat_add(a682, a683)?
        };
        let a685 = v2;
        nat_add(a684, a685)
    }
}

pub fn f31(v0: u64) -> bool {
    let a686 = v0;
    let a687 = 2139095040u64;
    if nat_le(a686, a687) {
        true
    } else {
        let a688 = v0;
        let a689 = 2147483648u64;
        nat_eq(a688, a689)
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a690 = v0;
    let a691 = 0u64;
    if nat_eq(a690, a691) {
        Ok(0u64)
    } else {
        let a692 = v1;
        let a693 = f81(v0)?;
        nat_mul(a692, a693)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a694 = v0;
    let a695 = 0u64;
    if if if if nat_eq(a694, a695) {
        true
    } else {
        let a696 = v1;
        let a697 = 0u64;
        nat_eq(a696, a697)
    } {
        true
    } else {
        let a698 = v2;
        let a699 = 0u64;
        nat_eq(a698, a699)
    } {
        true
    } else {
        let a700 = v3;
        let a701 = 0u64;
        nat_eq(a700, a701)
    } {
        Ok(0u64)
    } else {
        let a714 = {
            let a704 = 2u64;
            let a705 = {
                let a702 = v1;
                let a703 = v2;
                nat_mul(a702, a703)?
            };
            nat_mul(a704, a705)?
        };
        let a715 = {
            let a712 = {
                let a708 = {
                    let a706 = v1;
                    let a707 = v2;
                    nat_mul(a706, a707)?
                };
                let a709 = v3;
                nat_mul(a708, a709)?
            };
            let a713 = {
                let a710 = 1u64;
                let a711 = v0;
                nat_add(a710, a711)?
            };
            nat_mul(a712, a713)?
        };
        nat_add(a714, a715)
    }
}

pub fn f34(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m716 = v2;
    match m716 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a718 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h717, _, _, _) => {
                        h717
                    }
                };
                let a719 = 1u64;
                nat_eq(a718, a719)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h720, _, _, _, _, _, _, _, _, _) => {
                    h720
                }
            }, match v0.clone() {
                Adt1::C0(h721, _, _, _, _, _, _, _, _, _) => {
                    h721
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h722, _, _, _, _, _, _, _) => {
                    h722
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h723, _, _, _, _, _) => {
                    h723
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h724, _, _, _, _, _, _, _, _) => {
                        h724
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h725, _, _, _, _, _, _, _, _) => {
                        h725
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h726, _, _, _, _, _, _) => {
                        h726
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h727, _, _, _, _) => {
                        h727
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h728, _, _, _, _, _, _, _) => {
                        h728
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h729, _, _, _, _, _, _, _, _) => {
                        h729
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h730, _, _, _, _, _, _) => {
                        h730
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h731, _, _, _, _, _, _, _) => {
                        h731
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h732, _, _, _, _, _, _, _, _) => {
                        h732
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h733, _, _, _, _, _, _, _, _, _, _) => {
                    h733
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h734, _, _, _, _, _, _, _, _, _, _) => {
                    h734
                }
            };
            let v14: u64 = {
                let a735 = v12;
                let a736 = 0u64;
                if nat_eq(a735, a736) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a738 = match v4.clone() {
                    Adt2::C0(_, h737, _, _, _, _, _, _, _, _, _) => {
                        h737
                    }
                };
                let a739 = v14;
                nat_mul(a738, a739)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h740, _, _, _) => {
                        h740
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h741, _, _, _, _) => {
                        h741
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h742, _, _) => {
                        h742
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h743, _, _, _) => {
                        h743
                    }
                }
            };
            let a745 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h744, _, _, _) => {
                    h744
                }
            };
            let a746 = 1u64;
            if if if if if if if if if if nat_le(a745, a746) {
                let a747 = v12;
                let a748 = 0u64;
                if nat_eq(a747, a748) {
                    true
                } else {
                    let a749 = v12;
                    let a750 = 1u64;
                    if nat_eq(a749, a750) {
                        let a751 = v13;
                        let a752 = 1u64;
                        nat_eq(a751, a752)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a753 = v13;
                let a754 = 0u64;
                if nat_eq(a753, a754) {
                    true
                } else {
                    let a755 = v13;
                    let a756 = 1u64;
                    nat_eq(a755, a756)
                }
            } else {
                false
            } {
                let a758 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h757, _, _, _, _, _) => {
                        h757
                    }
                };
                let a759 = v14;
                nat_eq(a758, a759)
            } else {
                false
            } {
                let a767 = match v3.clone() {
                    Adt2::C0(_, h760, _, _, _, _, _, _, _, _, _) => {
                        h760
                    }
                };
                let a768 = {
                    let a765 = v9;
                    let a766 = {
                        let a763 = match v6.clone() {
                            Adt6::C0(h761, _, _, _) => {
                                h761
                            }
                        };
                        let a764 = match v7.clone() {
                            Adt6::C0(h762, _, _, _) => {
                                h762
                            }
                        };
                        nat_mul(a763, a764)?
                    };
                    nat_mul(a765, a766)?
                };
                nat_eq(a767, a768)
            } else {
                false
            } {
                let a771 = match v3.clone() {
                    Adt2::C0(_, _, h769, _, _, _, _, _, _, _, _) => {
                        h769
                    }
                };
                let a772 = f17(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h770, _, _, _, _, _, _, _, _, _) => {
                        h770
                    }
                })?;
                nat_eq(a771, a772)
            } else {
                false
            } {
                let a774 = 0u64;
                let a775 = match v3.clone() {
                    Adt2::C0(_, _, h773, _, _, _, _, _, _, _, _) => {
                        h773
                    }
                };
                nat_lt(a774, a775)
            } else {
                false
            } {
                if v5 {
                    let a777 = match v1 {
                        Adt2::C0(_, _, _, h776, _, _, _, _, _, _, _) => {
                            h776
                        }
                    };
                    let a778 = v9;
                    if if if nat_eq(a777, a778) {
                        let a780 = match v3.clone() {
                            Adt2::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
                                h779
                            }
                        };
                        let a781 = f17(v7.clone(), v11)?;
                        nat_eq(a780, a781)
                    } else {
                        false
                    } {
                        let a783 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h782, _, _, _, _, _, _) => {
                                h782
                            }
                        };
                        let a784 = v10;
                        nat_eq(a783, a784)
                    } else {
                        false
                    } {
                        let a786 = 0u64;
                        let a787 = match v3.clone() {
                            Adt2::C0(_, _, _, h785, _, _, _, _, _, _, _) => {
                                h785
                            }
                        };
                        nat_lt(a786, a787)
                    } else {
                        false
                    }
                } else {
                    let a789 = match v1 {
                        Adt2::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                            h788
                        }
                    };
                    let a790 = v9;
                    if if if nat_eq(a789, a790) {
                        let a792 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h791, _, _, _, _, _, _) => {
                                h791
                            }
                        };
                        let a793 = 1u64;
                        nat_eq(a792, a793)
                    } else {
                        false
                    } {
                        let a795 = match v3.clone() {
                            Adt2::C0(_, _, _, h794, _, _, _, _, _, _, _) => {
                                h794
                            }
                        };
                        let a796 = v10;
                        nat_eq(a795, a796)
                    } else {
                        false
                    } {
                        let a798 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h797, _, _, _, _, _, _) => {
                                h797
                            }
                        };
                        let a799 = 1u64;
                        nat_eq(a798, a799)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f27(match v4.clone() {
                    Adt2::C0(_, h800, _, _, _, _, _, _, _, _, _) => {
                        h800
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a820 = {
                        let a817 = {
                            let a815 = {
                                let a809 = {
                                    let a803 = {
                                        let a801 = v10;
                                        let a802 = 1u64;
                                        nat_sub(a801, a802)
                                    };
                                    let a804 = v17;
                                    nat_mul(a803, a804)?
                                };
                                let a810 = {
                                    let a807 = {
                                        let a805 = v9;
                                        let a806 = 1u64;
                                        nat_sub(a805, a806)
                                    };
                                    let a808 = v16;
                                    nat_mul(a807, a808)?
                                };
                                nat_add(a809, a810)?
                            };
                            let a816 = {
                                let a813 = {
                                    let a811 = v11;
                                    let a812 = 1u64;
                                    nat_sub(a811, a812)
                                };
                                let a814 = v15;
                                nat_mul(a813, a814)?
                            };
                            nat_add(a815, a816)?
                        };
                        let a818 = v15;
                        nat_add(a817, a818)?
                    };
                    let a821 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h819, _) => {
                            h819
                        }
                    };
                    nat_le(a820, a821)
                }
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt9::C4(f12(v3.clone(), 1u64)?, {
                    let a822 = v13;
                    let a823 = 0u64;
                    if nat_eq(a822, a823) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h824, _, _, _, _, _, _, _, _, _) => {
                        h824
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f16(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h825, _, _, _, _, _, _, _, _, _) => {
                            h825
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h826, _, _, _, _, _, _, _, _) => {
                            h826
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h827, _, _, _, _, _, _, _) => {
                            h827
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h828, _, _, _, _, _) => {
                            h828
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h829, _, _, _, _) => {
                            h829
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h830, _, _, _) => {
                            h830
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f35(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m831 = v2;
    match m831 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h832, _, _, _, _, _, _, _, _, _) => {
                    h832
                }
            }, match v0.clone() {
                Adt1::C0(h833, _, _, _, _, _, _, _, _, _) => {
                    h833
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h834, _, _, _, _, _, _) => {
                    h834
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h835, _, _, _) => {
                    h835
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h836, _, _, _, _, _, _, _, _) => {
                    h836
                }
            }, match v0.clone() {
                Adt1::C0(_, h837, _, _, _, _, _, _, _, _) => {
                    h837
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h838, _, _, _, _, _) => {
                    h838
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h839, _, _) => {
                    h839
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h840, _, _, _, _, _, _, _) => {
                    h840
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h841, _, _, _, _, _, _, _) => {
                    h841
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h842, _, _, _, _) => {
                    h842
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h843, _) => {
                    h843
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h844) => {
                    h844
                }
            };
            let v9: u64 = f17(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h845, _, _, _, _, _, _, _) => {
                    h845
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h846, _, _, _, _, _, _, _, _, _, _) => {
                    h846
                }
            };
            let v11: u64 = {
                let a854 = v8;
                let a855 = {
                    let a852 = match v5.clone() {
                        Adt6::C0(h847, _, _, _) => {
                            h847
                        }
                    };
                    let a853 = {
                        let a850 = match v6.clone() {
                            Adt6::C0(h848, _, _, _) => {
                                h848
                            }
                        };
                        let a851 = match v7.clone() {
                            Adt6::C0(h849, _, _, _) => {
                                h849
                            }
                        };
                        nat_mul(a850, a851)?
                    };
                    nat_mul(a852, a853)?
                };
                nat_mul(a854, a855)?
            };
            let a857 = match v4.clone() {
                Adt2::C0(h856, _, _, _, _, _, _, _, _, _, _) => {
                    h856
                }
            };
            let a858 = 0u64;
            if if if if if if if if if if if if if nat_eq(a857, a858) {
                let a859 = v10;
                let a860 = 0u64;
                if nat_eq(a859, a860) {
                    true
                } else {
                    let a861 = v10;
                    let a862 = 1u64;
                    nat_eq(a861, a862)
                }
            } else {
                false
            } {
                let a863 = 0u64;
                let a864 = v8;
                nat_lt(a863, a864)
            } else {
                false
            } {
                let a865 = v8;
                let a866 = 2147483648u64;
                nat_lt(a865, a866)
            } else {
                false
            } {
                let a868 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h867, _, _, _, _, _) => {
                        h867
                    }
                };
                let a869 = 4u64;
                nat_eq(a868, a869)
            } else {
                false
            } {
                let a870 = 0u64;
                let a871 = v9;
                nat_lt(a870, a871)
            } else {
                false
            } {
                let a873 = 0u64;
                let a874 = match v3.clone() {
                    Adt2::C0(_, _, h872, _, _, _, _, _, _, _, _) => {
                        h872
                    }
                };
                nat_lt(a873, a874)
            } else {
                false
            } {
                let a876 = 0u64;
                let a877 = match v3.clone() {
                    Adt2::C0(_, _, _, h875, _, _, _, _, _, _, _) => {
                        h875
                    }
                };
                nat_lt(a876, a877)
            } else {
                false
            } {
                let a879 = match v3.clone() {
                    Adt2::C0(_, h878, _, _, _, _, _, _, _, _, _) => {
                        h878
                    }
                };
                let a880 = v11;
                nat_eq(a879, a880)
            } else {
                false
            } {
                let a883 = match v3.clone() {
                    Adt2::C0(_, _, h881, _, _, _, _, _, _, _, _) => {
                        h881
                    }
                };
                let a884 = f17(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h882, _, _, _, _, _, _, _, _, _) => {
                        h882
                    }
                })?;
                nat_eq(a883, a884)
            } else {
                false
            } {
                let a887 = match v3.clone() {
                    Adt2::C0(_, _, _, h885, _, _, _, _, _, _, _) => {
                        h885
                    }
                };
                let a888 = f17(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h886, _, _, _, _, _, _, _, _) => {
                        h886
                    }
                })?;
                nat_eq(a887, a888)
            } else {
                false
            } {
                let a896 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h889, _, _, _, _, _, _) => {
                        h889
                    }
                };
                let a897 = {
                    let a894 = v9;
                    let a895 = {
                        let a891 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h890, _, _, _, _, _, _) => {
                                h890
                            }
                        };
                        let a892 = v8;
                        let a893 = 0u64;
                        nat_quot(a891, a892, a893)
                    };
                    nat_mul(a894, a895)?
                };
                nat_eq(a896, a897)
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt9::C4(f12(v3.clone(), 1u64)?, {
                    let a898 = v10;
                    let a899 = 0u64;
                    if nat_eq(a898, a899) {
                        4u64
                    } else {
                        2u64
                    }
                }, f16(v4.clone(), 1u64), f16(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h900, _, _) => {
                        h900
                    }
                }, {
                    let a902 = v8;
                    let a903 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h901, _, _) => {
                            h901
                        }
                    };
                    nat_mul(a902, a903)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f36(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt9> {
    let m904 = v1;
    match m904 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v3) => {
            f82(v0, v3, v2)
        }
    }
}

pub fn f37(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a906 = match v0.clone() {
        Adt2::C0(h905, _, _, _, _, _, _, _, _, _, _) => {
            h905
        }
    };
    let a907 = 1u64;
    if if nat_eq(a906, a907) {
        true
    } else {
        let a909 = match v0.clone() {
            Adt2::C0(h908, _, _, _, _, _, _, _, _, _, _) => {
                h908
            }
        };
        let a910 = 30u64;
        nat_eq(a909, a910)
    } {
        let a912 = match v1.clone() {
            Adt2::C0(h911, _, _, _, _, _, _, _, _, _, _) => {
                h911
            }
        };
        let a913 = 1u64;
        if if if if if if if if if if if nat_eq(a912, a913) {
            let a916 = match v0.clone() {
                Adt2::C0(_, h914, _, _, _, _, _, _, _, _, _) => {
                    h914
                }
            };
            let a917 = match v1.clone() {
                Adt2::C0(_, h915, _, _, _, _, _, _, _, _, _) => {
                    h915
                }
            };
            nat_eq(a916, a917)
        } else {
            false
        } {
            let a919 = 0u64;
            let a920 = match v1.clone() {
                Adt2::C0(_, _, h918, _, _, _, _, _, _, _, _) => {
                    h918
                }
            };
            nat_lt(a919, a920)
        } else {
            false
        } {
            let a923 = match v1.clone() {
                Adt2::C0(_, _, h921, _, _, _, _, _, _, _, _) => {
                    h921
                }
            };
            let a924 = match v1.clone() {
                Adt2::C0(_, _, _, h922, _, _, _, _, _, _, _) => {
                    h922
                }
            };
            nat_eq(a923, a924)
        } else {
            false
        } {
            let a926 = match v1.clone() {
                Adt2::C0(_, _, _, _, h925, _, _, _, _, _, _) => {
                    h925
                }
            };
            let a927 = 1u64;
            nat_eq(a926, a927)
        } else {
            false
        } {
            let a935 = match v0.clone() {
                Adt2::C0(_, _, h928, _, _, _, _, _, _, _, _) => {
                    h928
                }
            };
            let a936 = {
                let a933 = {
                    let a931 = match v1.clone() {
                        Adt2::C0(_, _, h929, _, _, _, _, _, _, _, _) => {
                            h929
                        }
                    };
                    let a932 = match v1.clone() {
                        Adt2::C0(_, _, h930, _, _, _, _, _, _, _, _) => {
                            h930
                        }
                    };
                    nat_add(a931, a932)?
                };
                let a934 = 1u64;
                nat_sub(a933, a934)
            };
            nat_eq(a935, a936)
        } else {
            false
        } {
            let a938 = 0u64;
            let a939 = match v0.clone() {
                Adt2::C0(_, _, _, h937, _, _, _, _, _, _, _) => {
                    h937
                }
            };
            nat_lt(a938, a939)
        } else {
            false
        } {
            let a941 = 0u64;
            let a942 = match v0.clone() {
                Adt2::C0(_, _, _, _, h940, _, _, _, _, _, _) => {
                    h940
                }
            };
            nat_lt(a941, a942)
        } else {
            false
        } {
            let a944 = 2u64;
            let a945 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h943, _, _, _, _, _) => {
                    h943
                }
            };
            nat_le(a944, a945)
        } else {
            false
        } {
            let a947 = 2u64;
            let a948 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h946, _, _, _, _, _) => {
                    h946
                }
            };
            nat_le(a947, a948)
        } else {
            false
        } {
            f18(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt9::C4(f12(v1.clone(), 1u64)?, 2u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f38(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt9> {
    let m949 = v1;
    match m949 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v3) => {
            f83(v0, v3, v2)
        }
    }
}

pub fn f39(v0: u64) -> bool {
    let a950 = v0;
    let a951 = 1u64;
    if if nat_eq(a950, a951) {
        true
    } else {
        let a952 = v0;
        let a953 = 33u64;
        nat_eq(a952, a953)
    } {
        true
    } else {
        let a954 = v0;
        let a955 = 34u64;
        nat_eq(a954, a955)
    }
}

pub fn f40(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt9> {
    let a958 = match v1.clone() {
        Adt2::C0(h956, _, _, _, _, _, _, _, _, _, _) => {
            h956
        }
    };
    let a959 = match v3.clone() {
        Adt2::C0(h957, _, _, _, _, _, _, _, _, _, _) => {
            h957
        }
    };
    if nat_eq(a958, a959) {
        f84(v0, v1, v2, v3)
    } else {
        if if if if f85(match v1.clone() {
            Adt2::C0(h960, _, _, _, _, _, _, _, _, _, _) => {
                h960
            }
        }) {
            f85(match v3.clone() {
                Adt2::C0(h961, _, _, _, _, _, _, _, _, _, _) => {
                    h961
                }
            })
        } else {
            false
        } {
            let a962 = v0;
            let a963 = 33u64;
            if nat_eq(a962, a963) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a964 = f12(v1.clone(), 1u64)?;
            let a965 = f12(v3.clone(), 1u64)?;
            nat_eq(a964, a965)
        } else {
            false
        } {
            let m967 = f86(match v3.clone() {
                Adt2::C0(h966, _, _, _, _, _, _, _, _, _, _) => {
                    h966
                }
            });
            match m967 {
                Adt13::C0 => {
                    Ok(Adt9::C0)
                }
                Adt13::C1(v4) => {
                    Ok(Adt9::C4(f12(v3.clone(), 1u64)?, match v4 {
                        Adt14::C0(_, _, h968) => {
                            h968
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h969, _, _, _, _, _, _, _, _, _, _) => {
                            h969
                        }
                    }, match v3 {
                        Adt2::C0(h970, _, _, _, _, _, _, _, _, _, _) => {
                            h970
                        }
                    })))
                }
            }
        } else {
            Ok(Adt9::C1)
        }
    }
}

pub fn f41(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a972 = match v0.clone() {
        Adt2::C0(h971, _, _, _, _, _, _, _, _, _, _) => {
            h971
        }
    };
    let a973 = 0u64;
    if if if if nat_eq(a972, a973) {
        true
    } else {
        let a975 = match v0.clone() {
            Adt2::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a976 = 1u64;
        nat_eq(a975, a976)
    } {
        true
    } else {
        let a978 = match v0.clone() {
            Adt2::C0(h977, _, _, _, _, _, _, _, _, _, _) => {
                h977
            }
        };
        let a979 = 25u64;
        if nat_eq(a978, a979) {
            true
        } else {
            let a981 = match v0.clone() {
                Adt2::C0(h980, _, _, _, _, _, _, _, _, _, _) => {
                    h980
                }
            };
            let a982 = 26u64;
            nat_eq(a981, a982)
        }
    } {
        true
    } else {
        let a984 = match v0.clone() {
            Adt2::C0(h983, _, _, _, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a985 = 30u64;
        nat_eq(a984, a985)
    } {
        let a988 = match v0.clone() {
            Adt2::C0(h986, _, _, _, _, _, _, _, _, _, _) => {
                h986
            }
        };
        let a989 = match v1.clone() {
            Adt2::C0(h987, _, _, _, _, _, _, _, _, _, _) => {
                h987
            }
        };
        if if nat_eq(a988, a989) {
            f87(match v1.clone() {
                Adt2::C0(_, h990, _, _, _, _, _, _, _, _, _) => {
                    h990
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h991, _, _, _, _, _, _, _, _) => {
                    h991
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h992, _, _, _, _, _, _, _) => {
                    h992
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h993, _, _, _, _, _, _) => {
                    h993
                }
            }, match v0.clone() {
                Adt2::C0(_, h994, _, _, _, _, _, _, _, _, _) => {
                    h994
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h995, _, _, _, _, _, _, _, _) => {
                    h995
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h996, _, _, _, _, _, _, _) => {
                    h996
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h997, _, _, _, _, _, _) => {
                    h997
                }
            })
        } else {
            false
        } {
            let m999 = f86(match v0.clone() {
                Adt2::C0(h998, _, _, _, _, _, _, _, _, _, _) => {
                    h998
                }
            });
            match m999 {
                Adt13::C0 => {
                    Ok(Adt9::C0)
                }
                Adt13::C1(v2) => {
                    Ok(Adt9::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt14::C0(_, _, h1000) => {
                            h1000
                        }
                    }, f16(v0, 1u64), f16(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f42(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m1001 = v2;
    match m1001 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let a1002 = v0;
            let a1003 = 4u64;
            if if if nat_lt(a1002, a1003) {
                let a1006 = match v1.clone() {
                    Adt2::C0(h1004, _, _, _, _, _, _, _, _, _, _) => {
                        h1004
                    }
                };
                let a1007 = match v4.clone() {
                    Adt2::C0(h1005, _, _, _, _, _, _, _, _, _, _) => {
                        h1005
                    }
                };
                if nat_eq(a1006, a1007) {
                    let a1010 = match v1.clone() {
                        Adt2::C0(h1008, _, _, _, _, _, _, _, _, _, _) => {
                            h1008
                        }
                    };
                    let a1011 = match v3.clone() {
                        Adt2::C0(h1009, _, _, _, _, _, _, _, _, _, _) => {
                            h1009
                        }
                    };
                    nat_eq(a1010, a1011)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f88(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1012, _, _, _, _, _, _, _, _, _) => {
                        h1012
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1013, _, _, _, _, _, _, _, _, _) => {
                        h1013
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1014, _, _, _, _, _, _, _, _, _) => {
                        h1014
                    }
                })? {
                    f88(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1015, _, _, _, _, _, _, _, _) => {
                            h1015
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1016, _, _, _, _, _, _, _, _) => {
                            h1016
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1017, _, _, _, _, _, _, _, _) => {
                            h1017
                        }
                    })?
                } else {
                    false
                } {
                    f88(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1018, _, _, _, _, _, _, _) => {
                            h1018
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
                            h1019
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1020, _, _, _, _, _, _, _) => {
                            h1020
                        }
                    })?
                } else {
                    false
                } {
                    f88(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1021, _, _, _, _, _, _) => {
                            h1021
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1022, _, _, _, _, _, _) => {
                            h1022
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1023, _, _, _, _, _, _) => {
                            h1023
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1025 = f86(match v1.clone() {
                    Adt2::C0(h1024, _, _, _, _, _, _, _, _, _, _) => {
                        h1024
                    }
                });
                match m1025 {
                    Adt13::C0 => {
                        Ok(Adt9::C0)
                    }
                    Adt13::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt14::C0(_, h1026, _) => {
                                h1026
                            }
                        };
                        let a1031 = {
                            let a1028 = match v1.clone() {
                                Adt2::C0(_, h1027, _, _, _, _, _, _, _, _, _) => {
                                    h1027
                                }
                            };
                            let a1029 = v6;
                            let a1030 = 1u64;
                            nat_rem(a1028, a1029, a1030)
                        };
                        let a1032 = 0u64;
                        if if if nat_eq(a1031, a1032) {
                            let a1037 = {
                                let a1034 = match v4.clone() {
                                    Adt2::C0(_, h1033, _, _, _, _, _, _, _, _, _) => {
                                        h1033
                                    }
                                };
                                let a1035 = v6;
                                let a1036 = 1u64;
                                nat_rem(a1034, a1035, a1036)
                            };
                            let a1038 = 0u64;
                            nat_eq(a1037, a1038)
                        } else {
                            false
                        } {
                            let a1043 = {
                                let a1040 = match v3.clone() {
                                    Adt2::C0(_, h1039, _, _, _, _, _, _, _, _, _) => {
                                        h1039
                                    }
                                };
                                let a1041 = v6;
                                let a1042 = 1u64;
                                nat_rem(a1040, a1041, a1042)
                            };
                            let a1044 = 0u64;
                            nat_eq(a1043, a1044)
                        } else {
                            false
                        } {
                            Ok(Adt9::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt14::C0(_, _, h1045) => {
                                    h1045
                                }
                            }, f16(v1, v6), f16(v3, v6), Adt4::C2(v0, f16(v4, v6))))
                        } else {
                            Ok(Adt9::C1)
                        }
                    }
                }
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f43(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt9> {
    let a1047 = match v2.clone() {
        Adt2::C0(h1046, _, _, _, _, _, _, _, _, _, _) => {
            h1046
        }
    };
    let a1048 = 0u64;
    if if nat_eq(a1047, a1048) {
        let a1050 = match v3.clone() {
            Adt2::C0(h1049, _, _, _, _, _, _, _, _, _, _) => {
                h1049
            }
        };
        let a1051 = 0u64;
        nat_eq(a1050, a1051)
    } else {
        false
    } {
        let a1052 = v0;
        let a1053 = 61u64;
        if nat_eq(a1052, a1053) {
            let v4: bool = {
                let a1055 = 0u64;
                let a1056 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1054, _) => {
                        h1054
                    }
                };
                nat_lt(a1055, a1056)
            };
            if if if if if if f89(match v2.clone() {
                Adt2::C0(_, h1057, _, _, _, _, _, _, _, _, _) => {
                    h1057
                }
            }, match v1.clone() {
                Adt1::C0(h1058, _, _, _, _, _, _, _, _, _) => {
                    h1058
                }
            }, match v1.clone() {
                Adt1::C0(_, h1059, _, _, _, _, _, _, _, _) => {
                    h1059
                }
            }, match v3.clone() {
                Adt2::C0(_, h1060, _, _, _, _, _, _, _, _, _) => {
                    h1060
                }
            })? {
                f89(match v2.clone() {
                    Adt2::C0(_, _, h1061, _, _, _, _, _, _, _, _) => {
                        h1061
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1062, _, _, _, _, _, _, _) => {
                        h1062
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1063, _, _, _, _, _, _) => {
                        h1063
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1064, _, _, _, _, _, _, _, _) => {
                        h1064
                    }
                })?
            } else {
                false
            } {
                f89(match v2.clone() {
                    Adt2::C0(_, _, _, h1065, _, _, _, _, _, _, _) => {
                        h1065
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1066, _, _, _, _, _) => {
                        h1066
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1067, _, _, _, _) => {
                        h1067
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1068, _, _, _, _, _, _, _) => {
                        h1068
                    }
                })?
            } else {
                false
            } {
                f89(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1069, _, _, _, _, _, _) => {
                        h1069
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1070, _, _, _) => {
                        h1070
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1071, _, _) => {
                        h1071
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1072, _, _, _, _, _, _) => {
                        h1072
                    }
                })?
            } else {
                false
            } {
                let a1074 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1073, _) => {
                        h1073
                    }
                };
                let a1075 = 4294967295u64;
                nat_le(a1074, a1075)
            } else {
                false
            } {
                if v4 {
                    !f27(match v2.clone() {
                        Adt2::C0(_, h1076, _, _, _, _, _, _, _, _, _) => {
                            h1076
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1077, _, _, _, _, _, _, _, _) => {
                            h1077
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1078, _, _, _, _, _, _, _) => {
                            h1078
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1079, _, _, _, _, _, _) => {
                            h1079
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt9::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1080, _, _, _, _, _, _, _, _, _) => {
                        h1080
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1081, _, _, _, _, _, _, _) => {
                        h1081
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1082, _, _, _, _, _) => {
                        h1082
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1083, _, _, _) => {
                        h1083
                    }
                }, v4)))
            } else {
                Ok(Adt9::C1)
            }
        } else {
            let a1084 = v0;
            let a1085 = 62u64;
            if nat_eq(a1084, a1085) {
                let a1087 = match v1.clone() {
                    Adt1::C0(h1086, _, _, _, _, _, _, _, _, _) => {
                        h1086
                    }
                };
                let a1088 = 2147483648u64;
                if if if if if nat_lt(a1087, a1088) {
                    let a1090 = match v1.clone() {
                        Adt1::C0(_, h1089, _, _, _, _, _, _, _, _) => {
                            h1089
                        }
                    };
                    let a1091 = 2147483648u64;
                    nat_lt(a1090, a1091)
                } else {
                    false
                } {
                    let a1094 = match v1.clone() {
                        Adt1::C0(h1092, _, _, _, _, _, _, _, _, _) => {
                            h1092
                        }
                    };
                    let a1095 = match v2.clone() {
                        Adt2::C0(_, h1093, _, _, _, _, _, _, _, _, _) => {
                            h1093
                        }
                    };
                    nat_lt(a1094, a1095)
                } else {
                    false
                } {
                    let a1098 = match v1.clone() {
                        Adt1::C0(_, h1096, _, _, _, _, _, _, _, _) => {
                            h1096
                        }
                    };
                    let a1099 = match v2.clone() {
                        Adt2::C0(_, h1097, _, _, _, _, _, _, _, _, _) => {
                            h1097
                        }
                    };
                    nat_lt(a1098, a1099)
                } else {
                    false
                } {
                    let a1102 = match v2.clone() {
                        Adt2::C0(_, _, h1100, _, _, _, _, _, _, _, _) => {
                            h1100
                        }
                    };
                    let a1103 = match v3.clone() {
                        Adt2::C0(_, _, h1101, _, _, _, _, _, _, _, _) => {
                            h1101
                        }
                    };
                    if if if nat_eq(a1102, a1103) {
                        let a1106 = match v2.clone() {
                            Adt2::C0(_, _, _, h1104, _, _, _, _, _, _, _) => {
                                h1104
                            }
                        };
                        let a1107 = match v3.clone() {
                            Adt2::C0(_, _, _, h1105, _, _, _, _, _, _, _) => {
                                h1105
                            }
                        };
                        nat_eq(a1106, a1107)
                    } else {
                        false
                    } {
                        let a1110 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1108, _, _, _, _, _, _) => {
                                h1108
                            }
                        };
                        let a1111 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1109, _, _, _, _, _, _) => {
                                h1109
                            }
                        };
                        nat_eq(a1110, a1111)
                    } else {
                        false
                    } {
                        let a1120 = match v3.clone() {
                            Adt2::C0(_, h1112, _, _, _, _, _, _, _, _, _) => {
                                h1112
                            }
                        };
                        let a1121 = {
                            let a1118 = {
                                let a1115 = match v2.clone() {
                                    Adt2::C0(_, h1113, _, _, _, _, _, _, _, _, _) => {
                                        h1113
                                    }
                                };
                                let a1116 = match v1.clone() {
                                    Adt1::C0(h1114, _, _, _, _, _, _, _, _, _) => {
                                        h1114
                                    }
                                };
                                nat_add(a1115, a1116)?
                            };
                            let a1119 = match v1.clone() {
                                Adt1::C0(_, h1117, _, _, _, _, _, _, _, _) => {
                                    h1117
                                }
                            };
                            nat_add(a1118, a1119)?
                        };
                        nat_eq(a1120, a1121)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt9::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1122, _, _, _, _, _, _, _, _, _) => {
                            h1122
                        }
                    })))
                } else {
                    Ok(Adt9::C1)
                }
            } else {
                let a1123 = v0;
                let a1124 = 63u64;
                if nat_eq(a1123, a1124) {
                    if if if if if f90(match v2.clone() {
                        Adt2::C0(_, h1125, _, _, _, _, _, _, _, _, _) => {
                            h1125
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1126, _, _, _, _, _, _, _, _, _) => {
                            h1126
                        }
                    })? {
                        f90(match v2.clone() {
                            Adt2::C0(_, _, h1127, _, _, _, _, _, _, _, _) => {
                                h1127
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1128, _, _, _, _, _, _, _, _) => {
                                h1128
                            }
                        })?
                    } else {
                        false
                    } {
                        f90(match v2.clone() {
                            Adt2::C0(_, _, _, h1129, _, _, _, _, _, _, _) => {
                                h1129
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1130, _, _, _, _, _, _, _) => {
                                h1130
                            }
                        })?
                    } else {
                        false
                    } {
                        f90(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1131, _, _, _, _, _, _) => {
                                h1131
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1132, _, _, _, _, _, _) => {
                                h1132
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1135 = match v2.clone() {
                            Adt2::C0(_, h1133, _, _, _, _, _, _, _, _, _) => {
                                h1133
                            }
                        };
                        let a1136 = match v3.clone() {
                            Adt2::C0(_, h1134, _, _, _, _, _, _, _, _, _) => {
                                h1134
                            }
                        };
                        if if if nat_eq(a1135, a1136) {
                            let a1139 = match v2.clone() {
                                Adt2::C0(_, _, h1137, _, _, _, _, _, _, _, _) => {
                                    h1137
                                }
                            };
                            let a1140 = match v3.clone() {
                                Adt2::C0(_, _, h1138, _, _, _, _, _, _, _, _) => {
                                    h1138
                                }
                            };
                            nat_eq(a1139, a1140)
                        } else {
                            false
                        } {
                            let a1143 = match v2.clone() {
                                Adt2::C0(_, _, _, h1141, _, _, _, _, _, _, _) => {
                                    h1141
                                }
                            };
                            let a1144 = match v3.clone() {
                                Adt2::C0(_, _, _, h1142, _, _, _, _, _, _, _) => {
                                    h1142
                                }
                            };
                            nat_eq(a1143, a1144)
                        } else {
                            false
                        } {
                            let a1147 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1145, _, _, _, _, _, _) => {
                                    h1145
                                }
                            };
                            let a1148 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1146, _, _, _, _, _, _) => {
                                    h1146
                                }
                            };
                            nat_eq(a1147, a1148)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1149, _, _, _, _, _, _, _, _, _) => {
                                h1149
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1150, _, _, _, _, _, _, _, _) => {
                                h1150
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1151, _, _, _, _, _, _, _) => {
                                h1151
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1152, _, _, _, _, _, _) => {
                                h1152
                            }
                        })))
                    } else {
                        Ok(Adt9::C1)
                    }
                } else {
                    Ok(Adt9::C0)
                }
            }
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f44(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let m1153 = v3;
    match m1153 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1154 = v0;
                let a1155 = 4u64;
                nat_eq(a1154, a1155)
            };
            let v7: bool = {
                let a1157 = 0u64;
                let a1158 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1156, _, _, _, _, _) => {
                        h1156
                    }
                };
                nat_lt(a1157, a1158)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1159, _, _, _, _, _, _, _, _, _) => {
                    h1159
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1160, _, _, _, _, _, _, _, _) => {
                    h1160
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1161, _, _, _, _, _, _, _) => {
                    h1161
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1162, _, _, _, _, _, _) => {
                    h1162
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1163, _, _, _, _, _, _, _, _, _) => {
                    h1163
                }
            }, match v1.clone() {
                Adt1::C0(_, h1164, _, _, _, _, _, _, _, _) => {
                    h1164
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1165, _, _, _, _, _, _, _) => {
                    h1165
                }
            });
            if if if if if if if if if if if if v6 {
                let a1167 = match v2.clone() {
                    Adt2::C0(h1166, _, _, _, _, _, _, _, _, _, _) => {
                        h1166
                    }
                };
                let a1168 = 0u64;
                if nat_eq(a1167, a1168) {
                    let a1170 = match v5.clone() {
                        Adt2::C0(h1169, _, _, _, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    let a1171 = 0u64;
                    nat_eq(a1170, a1171)
                } else {
                    false
                }
            } else {
                let a1173 = match v2.clone() {
                    Adt2::C0(h1172, _, _, _, _, _, _, _, _, _, _) => {
                        h1172
                    }
                };
                let a1174 = 0u64;
                if if nat_eq(a1173, a1174) {
                    true
                } else {
                    let a1176 = match v2.clone() {
                        Adt2::C0(h1175, _, _, _, _, _, _, _, _, _, _) => {
                            h1175
                        }
                    };
                    let a1177 = 26u64;
                    nat_eq(a1176, a1177)
                } {
                    let a1179 = match v5.clone() {
                        Adt2::C0(h1178, _, _, _, _, _, _, _, _, _, _) => {
                            h1178
                        }
                    };
                    let a1180 = 0u64;
                    if nat_eq(a1179, a1180) {
                        true
                    } else {
                        let a1182 = match v5.clone() {
                            Adt2::C0(h1181, _, _, _, _, _, _, _, _, _, _) => {
                                h1181
                            }
                        };
                        let a1183 = 26u64;
                        nat_eq(a1182, a1183)
                    }
                } else {
                    false
                }
            } {
                let a1186 = match v2.clone() {
                    Adt2::C0(h1184, _, _, _, _, _, _, _, _, _, _) => {
                        h1184
                    }
                };
                let a1187 = match v4.clone() {
                    Adt2::C0(h1185, _, _, _, _, _, _, _, _, _, _) => {
                        h1185
                    }
                };
                nat_eq(a1186, a1187)
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
                let a1189 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1188, _, _, _, _, _) => {
                        h1188
                    }
                };
                let a1190 = 4u64;
                nat_eq(a1189, a1190)
            } else {
                false
            } {
                let a1191 = v8;
                let a1192 = f12(v2.clone(), 1u64)?;
                nat_le(a1191, a1192)
            } else {
                false
            } {
                let a1194 = match v1.clone() {
                    Adt1::C0(_, _, _, h1193, _, _, _, _, _, _) => {
                        h1193
                    }
                };
                let a1195 = 1073741824u64;
                nat_lt(a1194, a1195)
            } else {
                false
            } {
                let a1197 = match v1.clone() {
                    Adt1::C0(h1196, _, _, _, _, _, _, _, _, _) => {
                        h1196
                    }
                };
                let a1198 = 4294967295u64;
                if if if if nat_le(a1197, a1198) {
                    let a1200 = match v1.clone() {
                        Adt1::C0(_, h1199, _, _, _, _, _, _, _, _) => {
                            h1199
                        }
                    };
                    let a1201 = 4294967295u64;
                    nat_le(a1200, a1201)
                } else {
                    false
                } {
                    let a1203 = match v1.clone() {
                        Adt1::C0(_, _, h1202, _, _, _, _, _, _, _) => {
                            h1202
                        }
                    };
                    let a1204 = 4294967295u64;
                    nat_le(a1203, a1204)
                } else {
                    false
                } {
                    let a1206 = match v1.clone() {
                        Adt1::C0(_, _, _, h1205, _, _, _, _, _, _) => {
                            h1205
                        }
                    };
                    let a1207 = 4294967295u64;
                    nat_le(a1206, a1207)
                } else {
                    false
                } {
                    let a1209 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1208, _, _, _, _, _) => {
                            h1208
                        }
                    };
                    let a1210 = 4294967295u64;
                    nat_le(a1209, a1210)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1213 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1211) => {
                            h1211
                        }
                    };
                    let a1214 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1212) => {
                            h1212
                        }
                    };
                    nat_eq(a1213, a1214)
                } else {
                    true
                }
            } else {
                false
            } {
                f91(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1215, _, _, _, _, _, _) => {
                        h1215
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1216, _) => {
                        h1216
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
                Ok(Adt9::C4({
                    let a1217 = v10;
                    let a1218 = v8;
                    nat_add(a1217, a1218)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1219, _, _, _, _, _, _) => {
                        h1219
                    }
                }, v6)))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f45(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a1221 = match v0.clone() {
        Adt2::C0(h1220, _, _, _, _, _, _, _, _, _, _) => {
            h1220
        }
    };
    let a1222 = 0u64;
    if if if nat_eq(a1221, a1222) {
        let a1224 = match v1.clone() {
            Adt2::C0(h1223, _, _, _, _, _, _, _, _, _, _) => {
                h1223
            }
        };
        let a1225 = 0u64;
        nat_eq(a1224, a1225)
    } else {
        false
    } {
        f87(match v0.clone() {
            Adt2::C0(_, h1226, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1227, _, _, _, _, _, _, _, _) => {
                h1227
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1228, _, _, _, _, _, _, _) => {
                h1228
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1229, _, _, _, _, _, _) => {
                h1229
            }
        }, match v1.clone() {
            Adt2::C0(_, h1230, _, _, _, _, _, _, _, _, _) => {
                h1230
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1231, _, _, _, _, _, _, _, _) => {
                h1231
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1232, _, _, _, _, _, _, _) => {
                h1232
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1233, _, _, _, _, _, _) => {
                h1233
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt9::C4({
            let a1234 = v2;
            let a1235 = f12(v0.clone(), 1u64)?;
            nat_add(a1234, a1235)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f46(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt9> {
    let a1237 = match v1.clone() {
        Adt2::C0(h1236, _, _, _, _, _, _, _, _, _, _) => {
            h1236
        }
    };
    let a1238 = 0u64;
    if if if nat_eq(a1237, a1238) {
        true
    } else {
        let a1240 = match v1.clone() {
            Adt2::C0(h1239, _, _, _, _, _, _, _, _, _, _) => {
                h1239
            }
        };
        let a1241 = 1u64;
        nat_eq(a1240, a1241)
    } {
        true
    } else {
        let a1243 = match v1.clone() {
            Adt2::C0(h1242, _, _, _, _, _, _, _, _, _, _) => {
                h1242
            }
        };
        let a1244 = 30u64;
        nat_eq(a1243, a1244)
    } {
        let a1247 = match v1.clone() {
            Adt2::C0(h1245, _, _, _, _, _, _, _, _, _, _) => {
                h1245
            }
        };
        let a1248 = match v2.clone() {
            Adt2::C0(h1246, _, _, _, _, _, _, _, _, _, _) => {
                h1246
            }
        };
        if if if if if nat_eq(a1247, a1248) {
            let a1251 = match v1.clone() {
                Adt2::C0(_, h1249, _, _, _, _, _, _, _, _, _) => {
                    h1249
                }
            };
            let a1252 = match v2.clone() {
                Adt2::C0(_, h1250, _, _, _, _, _, _, _, _, _) => {
                    h1250
                }
            };
            nat_eq(a1251, a1252)
        } else {
            false
        } {
            let a1255 = match v1.clone() {
                Adt2::C0(_, _, _, h1253, _, _, _, _, _, _, _) => {
                    h1253
                }
            };
            let a1256 = match v2.clone() {
                Adt2::C0(_, _, _, h1254, _, _, _, _, _, _, _) => {
                    h1254
                }
            };
            nat_eq(a1255, a1256)
        } else {
            false
        } {
            let a1259 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1257, _, _, _, _, _, _) => {
                    h1257
                }
            };
            let a1260 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1258, _, _, _, _, _, _) => {
                    h1258
                }
            };
            nat_eq(a1259, a1260)
        } else {
            false
        } {
            let a1261 = v0;
            let a1262 = 4u64;
            if nat_eq(a1261, a1262) {
                let a1264 = match v1.clone() {
                    Adt2::C0(_, _, h1263, _, _, _, _, _, _, _, _) => {
                        h1263
                    }
                };
                let a1265 = 1u64;
                if nat_eq(a1264, a1265) {
                    let a1268 = match v1.clone() {
                        Adt2::C0(_, h1266, _, _, _, _, _, _, _, _, _) => {
                            h1266
                        }
                    };
                    let a1269 = match v2.clone() {
                        Adt2::C0(_, _, h1267, _, _, _, _, _, _, _, _) => {
                            h1267
                        }
                    };
                    nat_eq(a1268, a1269)
                } else {
                    false
                }
            } else {
                let a1270 = v0;
                let a1271 = 4u64;
                if if nat_lt(a1270, a1271) {
                    let a1274 = match v1.clone() {
                        Adt2::C0(_, h1272, _, _, _, _, _, _, _, _, _) => {
                            h1272
                        }
                    };
                    let a1275 = match v1.clone() {
                        Adt2::C0(_, _, h1273, _, _, _, _, _, _, _, _) => {
                            h1273
                        }
                    };
                    nat_eq(a1274, a1275)
                } else {
                    false
                } {
                    let a1278 = match v1.clone() {
                        Adt2::C0(_, _, h1276, _, _, _, _, _, _, _, _) => {
                            h1276
                        }
                    };
                    let a1279 = match v2.clone() {
                        Adt2::C0(_, _, h1277, _, _, _, _, _, _, _, _) => {
                            h1277
                        }
                    };
                    nat_eq(a1278, a1279)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1281 = f86(match v1.clone() {
                Adt2::C0(h1280, _, _, _, _, _, _, _, _, _, _) => {
                    h1280
                }
            });
            match m1281 {
                Adt13::C0 => {
                    Ok(Adt9::C0)
                }
                Adt13::C1(v3) => {
                    Ok(Adt9::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt14::C0(_, _, h1282) => {
                            h1282
                        }
                    }, f16(v1, 1u64), f16(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f47(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt9> {
    if if if f92(match v1.clone() {
        Adt2::C0(h1283, _, _, _, _, _, _, _, _, _, _) => {
            h1283
        }
    }, v0) {
        let a1286 = match v1.clone() {
            Adt2::C0(h1284, _, _, _, _, _, _, _, _, _, _) => {
                h1284
            }
        };
        let a1287 = match v2.clone() {
            Adt2::C0(h1285, _, _, _, _, _, _, _, _, _, _) => {
                h1285
            }
        };
        nat_eq(a1286, a1287)
    } else {
        false
    } {
        let a1290 = match v1.clone() {
            Adt2::C0(_, h1288, _, _, _, _, _, _, _, _, _) => {
                h1288
            }
        };
        let a1291 = match v2.clone() {
            Adt2::C0(_, h1289, _, _, _, _, _, _, _, _, _) => {
                h1289
            }
        };
        if if if nat_eq(a1290, a1291) {
            let a1294 = match v1.clone() {
                Adt2::C0(_, _, h1292, _, _, _, _, _, _, _, _) => {
                    h1292
                }
            };
            let a1295 = match v2.clone() {
                Adt2::C0(_, _, h1293, _, _, _, _, _, _, _, _) => {
                    h1293
                }
            };
            nat_eq(a1294, a1295)
        } else {
            false
        } {
            let a1298 = match v1.clone() {
                Adt2::C0(_, _, _, h1296, _, _, _, _, _, _, _) => {
                    h1296
                }
            };
            let a1299 = match v2.clone() {
                Adt2::C0(_, _, _, h1297, _, _, _, _, _, _, _) => {
                    h1297
                }
            };
            nat_eq(a1298, a1299)
        } else {
            false
        } {
            let a1302 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1300, _, _, _, _, _, _) => {
                    h1300
                }
            };
            let a1303 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1301, _, _, _, _, _, _) => {
                    h1301
                }
            };
            nat_eq(a1302, a1303)
        } else {
            false
        }
    } else {
        false
    } {
        let m1305 = f86(match v1.clone() {
            Adt2::C0(h1304, _, _, _, _, _, _, _, _, _, _) => {
                h1304
            }
        });
        match m1305 {
            Adt13::C0 => {
                Ok(Adt9::C0)
            }
            Adt13::C1(v3) => {
                Ok(Adt9::C4(f12(v2.clone(), 1u64)?, match v3 {
                    Adt14::C0(_, _, h1306) => {
                        h1306
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1307, _, _, _, _, _, _, _, _, _, _) => {
                        h1307
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f48(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt9> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1308, _, _, _, _, _, _, _, _, _) => {
                h1308
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1309, _, _, _, _, _, _, _) => {
                h1309
            }
        }
    };
    let a1310 = 0u64;
    let a1311 = v4;
    if if nat_lt(a1310, a1311) {
        let a1312 = v4;
        let a1313 = 2147483648u64;
        nat_lt(a1312, a1313)
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
        let v7: u64 = f93(match v5.clone() {
            Adt2::C0(_, _, h1314, _, _, _, _, _, _, _, _) => {
                h1314
            }
        }, v4)?;
        let v8: u64 = f93(match v5.clone() {
            Adt2::C0(_, _, _, h1315, _, _, _, _, _, _, _) => {
                h1315
            }
        }, v4)?;
        let a1317 = match v5.clone() {
            Adt2::C0(h1316, _, _, _, _, _, _, _, _, _, _) => {
                h1316
            }
        };
        let a1318 = 0u64;
        if if if if if nat_eq(a1317, a1318) {
            let a1320 = match v6.clone() {
                Adt2::C0(h1319, _, _, _, _, _, _, _, _, _, _) => {
                    h1319
                }
            };
            let a1321 = 0u64;
            nat_eq(a1320, a1321)
        } else {
            false
        } {
            let a1323 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1322, _, _, _, _, _, _) => {
                    h1322
                }
            };
            let a1324 = 1u64;
            nat_eq(a1323, a1324)
        } else {
            false
        } {
            let a1327 = match v5.clone() {
                Adt2::C0(_, h1325, _, _, _, _, _, _, _, _, _) => {
                    h1325
                }
            };
            let a1328 = match v6.clone() {
                Adt2::C0(_, h1326, _, _, _, _, _, _, _, _, _) => {
                    h1326
                }
            };
            nat_eq(a1327, a1328)
        } else {
            false
        } {
            if v0 {
                let a1330 = match v5.clone() {
                    Adt2::C0(_, _, h1329, _, _, _, _, _, _, _, _) => {
                        h1329
                    }
                };
                let a1331 = 2147483648u64;
                if if if nat_lt(a1330, a1331) {
                    let a1333 = match v5.clone() {
                        Adt2::C0(_, _, _, h1332, _, _, _, _, _, _, _) => {
                            h1332
                        }
                    };
                    let a1334 = 2147483648u64;
                    nat_lt(a1333, a1334)
                } else {
                    false
                } {
                    let a1337 = {
                        let a1335 = v7;
                        let a1336 = v8;
                        nat_mul(a1335, a1336)?
                    };
                    let a1338 = 2147483648u64;
                    nat_le(a1337, a1338)
                } else {
                    false
                } {
                    f94(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1340 = match v1.clone() {
                    Adt1::C0(h1339, _, _, _, _, _, _, _, _, _) => {
                        h1339
                    }
                };
                let a1341 = v7;
                if if if if nat_eq(a1340, a1341) {
                    let a1343 = match v1 {
                        Adt1::C0(_, h1342, _, _, _, _, _, _, _, _) => {
                            h1342
                        }
                    };
                    let a1344 = v8;
                    nat_eq(a1343, a1344)
                } else {
                    false
                } {
                    f95(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1346 = match v6.clone() {
                        Adt2::C0(_, _, h1345, _, _, _, _, _, _, _, _) => {
                            h1345
                        }
                    };
                    let a1347 = v4;
                    if nat_eq(a1346, a1347) {
                        let a1349 = match v6.clone() {
                            Adt2::C0(_, _, _, h1348, _, _, _, _, _, _, _) => {
                                h1348
                            }
                        };
                        let a1350 = v4;
                        nat_eq(a1349, a1350)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1354 = match v6 {
                        Adt2::C0(_, _, _, _, h1351, _, _, _, _, _, _) => {
                            h1351
                        }
                    };
                    let a1355 = {
                        let a1352 = v7;
                        let a1353 = v8;
                        nat_mul(a1352, a1353)?
                    };
                    nat_eq(a1354, a1355)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt9::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f49(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt9> {
    let a1357 = match v2.clone() {
        Adt2::C0(h1356, _, _, _, _, _, _, _, _, _, _) => {
            h1356
        }
    };
    let a1358 = 0u64;
    if if if if if nat_eq(a1357, a1358) {
        true
    } else {
        let a1360 = match v2.clone() {
            Adt2::C0(h1359, _, _, _, _, _, _, _, _, _, _) => {
                h1359
            }
        };
        let a1361 = 1u64;
        nat_eq(a1360, a1361)
    } {
        true
    } else {
        let a1363 = match v2.clone() {
            Adt2::C0(h1362, _, _, _, _, _, _, _, _, _, _) => {
                h1362
            }
        };
        let a1364 = 30u64;
        nat_eq(a1363, a1364)
    } {
        let a1367 = match v2.clone() {
            Adt2::C0(h1365, _, _, _, _, _, _, _, _, _, _) => {
                h1365
            }
        };
        let a1368 = match v3.clone() {
            Adt2::C0(h1366, _, _, _, _, _, _, _, _, _, _) => {
                h1366
            }
        };
        nat_eq(a1367, a1368)
    } else {
        false
    } {
        if if f19(v2.clone(), v3.clone()) {
            let a1370 = match v1.clone() {
                Adt1::C0(h1369, _, _, _, _, _, _, _, _, _) => {
                    h1369
                }
            };
            let a1371 = 4294967296u64;
            nat_lt(a1370, a1371)
        } else {
            false
        } {
            let a1373 = match v1.clone() {
                Adt1::C0(_, h1372, _, _, _, _, _, _, _, _) => {
                    h1372
                }
            };
            let a1374 = 4294967296u64;
            nat_lt(a1373, a1374)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt9::C4(f12(v3.clone(), 1u64)?, {
            let a1376 = match v2.clone() {
                Adt2::C0(h1375, _, _, _, _, _, _, _, _, _, _) => {
                    h1375
                }
            };
            let a1377 = 0u64;
            if nat_eq(a1376, a1377) {
                4u64
            } else {
                2u64
            }
        }, f16(v2.clone(), 1u64), f16(v3, 1u64), Adt4::C13({
            let a1378 = v0;
            let a1379 = 31u64;
            if nat_eq(a1378, a1379) {
                0u64
            } else {
                let a1380 = v0;
                let a1381 = 47u64;
                if nat_eq(a1380, a1381) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1382, _, _, _, _, _, _, _, _, _, _) => {
                h1382
            }
        }, match v1.clone() {
            Adt1::C0(h1383, _, _, _, _, _, _, _, _, _) => {
                h1383
            }
        }, match v1 {
            Adt1::C0(_, h1384, _, _, _, _, _, _, _, _) => {
                h1384
            }
        })))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f50(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a1386 = match v0.clone() {
        Adt2::C0(h1385, _, _, _, _, _, _, _, _, _, _) => {
            h1385
        }
    };
    let a1387 = 0u64;
    if if if nat_eq(a1386, a1387) {
        let a1389 = match v1.clone() {
            Adt2::C0(h1388, _, _, _, _, _, _, _, _, _, _) => {
                h1388
            }
        };
        let a1390 = 0u64;
        nat_eq(a1389, a1390)
    } else {
        false
    } {
        f19(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt9::C4(f12(v1.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f51(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a1392 = match v0.clone() {
        Adt2::C0(h1391, _, _, _, _, _, _, _, _, _, _) => {
            h1391
        }
    };
    let a1393 = 0u64;
    if if nat_eq(a1392, a1393) {
        let a1395 = match v1.clone() {
            Adt2::C0(h1394, _, _, _, _, _, _, _, _, _, _) => {
                h1394
            }
        };
        let a1396 = 26u64;
        nat_eq(a1395, a1396)
    } else {
        false
    } {
        let a1398 = match v0.clone() {
            Adt2::C0(_, _, _, h1397, _, _, _, _, _, _, _) => {
                h1397
            }
        };
        let a1399 = 1u64;
        if if if if if if if nat_eq(a1398, a1399) {
            let a1401 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1400, _, _, _, _, _, _) => {
                    h1400
                }
            };
            let a1402 = 1u64;
            nat_eq(a1401, a1402)
        } else {
            false
        } {
            let a1404 = match v0.clone() {
                Adt2::C0(_, h1403, _, _, _, _, _, _, _, _, _) => {
                    h1403
                }
            };
            let a1405 = 2147483647u64;
            nat_le(a1404, a1405)
        } else {
            false
        } {
            let a1408 = match v1.clone() {
                Adt2::C0(_, h1406, _, _, _, _, _, _, _, _, _) => {
                    h1406
                }
            };
            let a1409 = match v0.clone() {
                Adt2::C0(_, _, h1407, _, _, _, _, _, _, _, _) => {
                    h1407
                }
            };
            nat_eq(a1408, a1409)
        } else {
            false
        } {
            let a1411 = match v1.clone() {
                Adt2::C0(_, _, h1410, _, _, _, _, _, _, _, _) => {
                    h1410
                }
            };
            let a1412 = 1u64;
            nat_eq(a1411, a1412)
        } else {
            false
        } {
            let a1414 = match v1.clone() {
                Adt2::C0(_, _, _, h1413, _, _, _, _, _, _, _) => {
                    h1413
                }
            };
            let a1415 = 1u64;
            nat_eq(a1414, a1415)
        } else {
            false
        } {
            let a1417 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1416, _, _, _, _, _, _) => {
                    h1416
                }
            };
            let a1418 = 1u64;
            nat_eq(a1417, a1418)
        } else {
            false
        } {
            Ok(Adt9::C4({
                let a1421 = match v0.clone() {
                    Adt2::C0(_, _, h1419, _, _, _, _, _, _, _, _) => {
                        h1419
                    }
                };
                let a1422 = f96(match v0.clone() {
                    Adt2::C0(_, h1420, _, _, _, _, _, _, _, _, _) => {
                        h1420
                    }
                })?;
                nat_mul(a1421, a1422)?
            }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f52(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt9> {
    let m1423 = v1;
    match m1423 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v3) => {
            let v4: u64 = f97(f16(v0.clone(), 1u64))?;
            let a1425 = match v0.clone() {
                Adt2::C0(h1424, _, _, _, _, _, _, _, _, _, _) => {
                    h1424
                }
            };
            let a1426 = 26u64;
            if if if nat_eq(a1425, a1426) {
                let a1428 = match v3.clone() {
                    Adt2::C0(h1427, _, _, _, _, _, _, _, _, _, _) => {
                        h1427
                    }
                };
                let a1429 = 26u64;
                nat_eq(a1428, a1429)
            } else {
                false
            } {
                let a1431 = match v2.clone() {
                    Adt2::C0(h1430, _, _, _, _, _, _, _, _, _, _) => {
                        h1430
                    }
                };
                let a1432 = 27u64;
                nat_eq(a1431, a1432)
            } else {
                false
            } {
                if if if if if if f19(v0.clone(), v3.clone()) {
                    let a1434 = match v2.clone() {
                        Adt2::C0(_, h1433, _, _, _, _, _, _, _, _, _) => {
                            h1433
                        }
                    };
                    let a1435 = 1u64;
                    nat_eq(a1434, a1435)
                } else {
                    false
                } {
                    let a1437 = match v2.clone() {
                        Adt2::C0(_, _, h1436, _, _, _, _, _, _, _, _) => {
                            h1436
                        }
                    };
                    let a1438 = 1u64;
                    nat_eq(a1437, a1438)
                } else {
                    false
                } {
                    let a1440 = match v2.clone() {
                        Adt2::C0(_, _, _, h1439, _, _, _, _, _, _, _) => {
                            h1439
                        }
                    };
                    let a1441 = 1u64;
                    nat_eq(a1440, a1441)
                } else {
                    false
                } {
                    let a1443 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1442, _, _, _, _, _, _) => {
                            h1442
                        }
                    };
                    let a1444 = 1u64;
                    nat_eq(a1443, a1444)
                } else {
                    false
                } {
                    let a1445 = v4;
                    let a1446 = 9223372036854775808u64;
                    nat_lt(a1445, a1446)
                } else {
                    false
                } {
                    Ok(Adt9::C4({
                        let a1449 = 2u64;
                        let a1450 = {
                            let a1447 = v4;
                            let a1448 = 2u64;
                            nat_mul(a1447, a1448)?
                        };
                        nat_add(a1449, a1450)?
                    }, 8u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C18(v4, f16(v3, 1u64))))
                } else {
                    Ok(Adt9::C1)
                }
            } else {
                Ok(Adt9::C0)
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt9> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1451, _, _, _, _, _, _, _, _, _, _) => {
            h1451
        }
    };
    let a1452 = v3;
    let a1453 = 0u64;
    if if nat_eq(a1452, a1453) {
        true
    } else {
        let a1454 = v0;
        let a1455 = 13u64;
        if nat_eq(a1454, a1455) {
            let a1456 = v3;
            let a1457 = 1u64;
            if nat_eq(a1456, a1457) {
                true
            } else {
                let a1458 = v3;
                let a1459 = 30u64;
                nat_eq(a1458, a1459)
            }
        } else {
            false
        }
    } {
        let a1461 = v3;
        let a1462 = match v2.clone() {
            Adt2::C0(h1460, _, _, _, _, _, _, _, _, _, _) => {
                h1460
            }
        };
        if if nat_eq(a1461, a1462) {
            let a1463 = v0;
            let a1464 = 13u64;
            if nat_eq(a1463, a1464) {
                let a1466 = match v2.clone() {
                    Adt2::C0(_, h1465, _, _, _, _, _, _, _, _, _) => {
                        h1465
                    }
                };
                let a1467 = 1u64;
                if if if nat_eq(a1466, a1467) {
                    let a1469 = match v2.clone() {
                        Adt2::C0(_, _, h1468, _, _, _, _, _, _, _, _) => {
                            h1468
                        }
                    };
                    let a1470 = 1u64;
                    nat_eq(a1469, a1470)
                } else {
                    false
                } {
                    let a1472 = match v2.clone() {
                        Adt2::C0(_, _, _, h1471, _, _, _, _, _, _, _) => {
                            h1471
                        }
                    };
                    let a1473 = 1u64;
                    nat_eq(a1472, a1473)
                } else {
                    false
                } {
                    let a1475 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1474, _, _, _, _, _, _) => {
                            h1474
                        }
                    };
                    let a1476 = 1u64;
                    nat_eq(a1475, a1476)
                } else {
                    false
                }
            } else {
                let a1478 = match v2.clone() {
                    Adt2::C0(_, h1477, _, _, _, _, _, _, _, _, _) => {
                        h1477
                    }
                };
                let a1479 = 1u64;
                if if if nat_eq(a1478, a1479) {
                    let a1482 = match v1.clone() {
                        Adt2::C0(_, _, h1480, _, _, _, _, _, _, _, _) => {
                            h1480
                        }
                    };
                    let a1483 = match v2.clone() {
                        Adt2::C0(_, _, h1481, _, _, _, _, _, _, _, _) => {
                            h1481
                        }
                    };
                    nat_eq(a1482, a1483)
                } else {
                    false
                } {
                    let a1486 = match v1.clone() {
                        Adt2::C0(_, _, _, h1484, _, _, _, _, _, _, _) => {
                            h1484
                        }
                    };
                    let a1487 = match v2.clone() {
                        Adt2::C0(_, _, _, h1485, _, _, _, _, _, _, _) => {
                            h1485
                        }
                    };
                    nat_eq(a1486, a1487)
                } else {
                    false
                } {
                    let a1490 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1488, _, _, _, _, _, _) => {
                            h1488
                        }
                    };
                    let a1491 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1489, _, _, _, _, _, _) => {
                            h1489
                        }
                    };
                    nat_eq(a1490, a1491)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f16(v1, 1u64);
            Ok(Adt9::C4(f98(v4.clone(), v0, v3)?, {
                let a1492 = v3;
                let a1493 = 0u64;
                if nat_eq(a1492, a1493) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f16(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f54(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1494 = v0;
    let a1495 = 89u64;
    if if nat_eq(a1494, a1495) {
        let a1496 = v1;
        let a1497 = 4u64;
        nat_eq(a1496, a1497)
    } else {
        false
    } {
        Ok(Some(19u64))
    } else {
        let a1498 = 10u64;
        let a1499 = v0;
        if if nat_le(a1498, a1499) {
            let a1500 = v0;
            let a1501 = 12u64;
            nat_le(a1500, a1501)
        } else {
            false
        } {
            Ok(Some({
                let a1502 = v0;
                let a1503 = 3u64;
                nat_add(a1502, a1503)?
            }))
        } else {
            let a1504 = v0;
            let a1505 = 89u64;
            if if nat_eq(a1504, a1505) {
                let a1506 = v1;
                let a1507 = 7u64;
                nat_eq(a1506, a1507)
            } else {
                false
            } {
                Ok(Some(17u64))
            } else {
                let a1508 = v0;
                let a1509 = 89u64;
                if if nat_eq(a1508, a1509) {
                    let a1510 = v1;
                    let a1511 = 10u64;
                    nat_eq(a1510, a1511)
                } else {
                    false
                } {
                    Ok(Some(18u64))
                } else {
                    let a1512 = v0;
                    let a1513 = 89u64;
                    if if nat_eq(a1512, a1513) {
                        let a1514 = v1;
                        let a1515 = 13u64;
                        nat_eq(a1514, a1515)
                    } else {
                        false
                    } {
                        Ok(Some(16u64))
                    } else {
                        let a1516 = v0;
                        let a1517 = 8u64;
                        if nat_eq(a1516, a1517) {
                            Ok(Some(5u64))
                        } else {
                            let a1518 = v0;
                            let a1519 = 9u64;
                            if nat_eq(a1518, a1519) {
                                Ok(Some(6u64))
                            } else {
                                let a1520 = v0;
                                let a1521 = 89u64;
                                if nat_eq(a1520, a1521) {
                                    let a1522 = v1;
                                    let a1523 = 0u64;
                                    if nat_eq(a1522, a1523) {
                                        Ok(Some(0u64))
                                    } else {
                                        let a1524 = v1;
                                        let a1525 = 1u64;
                                        if nat_eq(a1524, a1525) {
                                            Ok(Some(1u64))
                                        } else {
                                            let a1526 = v1;
                                            let a1527 = 2u64;
                                            if nat_eq(a1526, a1527) {
                                                Ok(Some(2u64))
                                            } else {
                                                let a1528 = v1;
                                                let a1529 = 3u64;
                                                if nat_eq(a1528, a1529) {
                                                    Ok(Some(3u64))
                                                } else {
                                                    let a1530 = v1;
                                                    let a1531 = 6u64;
                                                    if nat_eq(a1530, a1531) {
                                                        Ok(Some(4u64))
                                                    } else {
                                                        let a1532 = v1;
                                                        let a1533 = 12u64;
                                                        if nat_eq(a1532, a1533) {
                                                            Ok(Some(7u64))
                                                        } else {
                                                            let a1534 = v1;
                                                            let a1535 = 11u64;
                                                            if nat_eq(a1534, a1535) {
                                                                Ok(Some(8u64))
                                                            } else {
                                                                let a1536 = v1;
                                                                let a1537 = 18u64;
                                                                if nat_eq(a1536, a1537) {
                                                                    Ok(Some(9u64))
                                                                } else {
                                                                    let a1538 = v1;
                                                                    let a1539 = 19u64;
                                                                    if nat_eq(a1538, a1539) {
                                                                        Ok(Some(10u64))
                                                                    } else {
                                                                        let a1540 = v1;
                                                                        let a1541 = 20u64;
                                                                        if nat_eq(a1540, a1541) {
                                                                            Ok(Some(11u64))
                                                                        } else {
                                                                            let a1542 = v1;
                                                                            let a1543 = 21u64;
                                                                            if nat_eq(a1542, a1543) {
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

pub fn f55(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let m1544 = f99(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1544 {
        Adt9::C0 => {
            Ok(Adt9::C0)
        }
        Adt9::C1 => {
            let a1545 = v0;
            let a1546 = 2u64;
            if if if if nat_eq(a1545, a1546) {
                true
            } else {
                let a1547 = v0;
                let a1548 = 5u64;
                nat_eq(a1547, a1548)
            } {
                true
            } else {
                let a1549 = v0;
                let a1550 = 6u64;
                nat_eq(a1549, a1550)
            } {
                true
            } else {
                let a1551 = v0;
                let a1552 = 7u64;
                nat_eq(a1551, a1552)
            } {
                Ok(Adt9::C5(67u64))
            } else {
                Ok(Adt9::C1)
            }
        }
        Adt9::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt9::C4(v13, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1553 = v3;
                    match m1553 {
                        None => {
                            f16(v2, 1u64)
                        }
                        Some(v14) => {
                            f16(v14, 1u64)
                        }
                    }
                })))
            } else {
                Ok(Adt9::C5(67u64))
            }
        }
        Adt9::C3(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24) => {
            Ok(Adt9::C3(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24))
        }
        Adt9::C4(v25, v26, v27, v28, v29) => {
            Ok(Adt9::C4(v25, v26, v27, v28, v29))
        }
        Adt9::C5(v6) => {
            Ok(Adt9::C5(v6))
        }
        Adt9::C6(v5) => {
            Ok(Adt9::C6(v5))
        }
    }
}

pub fn f56(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt9> {
    let a1555 = match v1.clone() {
        Adt2::C0(h1554, _, _, _, _, _, _, _, _, _, _) => {
            h1554
        }
    };
    let a1556 = 0u64;
    if if if if if nat_eq(a1555, a1556) {
        true
    } else {
        let a1558 = match v1.clone() {
            Adt2::C0(h1557, _, _, _, _, _, _, _, _, _, _) => {
                h1557
            }
        };
        let a1559 = 1u64;
        nat_eq(a1558, a1559)
    } {
        true
    } else {
        let a1561 = match v1.clone() {
            Adt2::C0(h1560, _, _, _, _, _, _, _, _, _, _) => {
                h1560
            }
        };
        let a1562 = 30u64;
        nat_eq(a1561, a1562)
    } {
        let a1565 = match v1.clone() {
            Adt2::C0(h1563, _, _, _, _, _, _, _, _, _, _) => {
                h1563
            }
        };
        let a1566 = match v2.clone() {
            Adt2::C0(h1564, _, _, _, _, _, _, _, _, _, _) => {
                h1564
            }
        };
        if nat_eq(a1565, a1566) {
            true
        } else {
            let a1568 = match v2.clone() {
                Adt2::C0(h1567, _, _, _, _, _, _, _, _, _, _) => {
                    h1567
                }
            };
            let a1569 = 0u64;
            nat_eq(a1568, a1569)
        }
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt9::C4(f12(v2.clone(), 1u64)?, {
            let a1571 = match v2.clone() {
                Adt2::C0(h1570, _, _, _, _, _, _, _, _, _, _) => {
                    h1570
                }
            };
            let a1572 = 0u64;
            if nat_eq(a1571, a1572) {
                4u64
            } else {
                2u64
            }
        }, f16(v1.clone(), 1u64), f16(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1573, _, _, _, _, _, _, _, _, _, _) => {
                h1573
            }
        }, match v2 {
            Adt2::C0(h1574, _, _, _, _, _, _, _, _, _, _) => {
                h1574
            }
        })))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f57(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f100(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1575, _, _, _) => {
            h1575
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1576, _, _) => {
            h1576
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1577, _) => {
            h1577
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1578) => {
            h1578
        }
    })
}

pub fn f58(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1580 = v0;
        let a1581 = match v1.clone() {
            Adt3::C0(h1579, _, _, _, _, _, _, _) => {
                h1579
            }
        };
        let a1582 = 0u64;
        nat_quot(a1580, a1581, a1582)
    };
    let v4: u64 = {
        let a1584 = v3;
        let a1585 = match v1.clone() {
            Adt3::C0(_, h1583, _, _, _, _, _, _) => {
                h1583
            }
        };
        let a1586 = 0u64;
        nat_quot(a1584, a1585, a1586)
    };
    let a1587 = v2;
    let a1588 = 0u64;
    if nat_eq(a1587, a1588) {
        let a1590 = v0;
        let a1591 = match v1 {
            Adt3::C0(h1589, _, _, _, _, _, _, _) => {
                h1589
            }
        };
        let a1592 = 0u64;
        nat_rem(a1590, a1591, a1592)
    } else {
        let a1593 = v2;
        let a1594 = 1u64;
        if nat_eq(a1593, a1594) {
            let a1596 = v3;
            let a1597 = match v1 {
                Adt3::C0(_, h1595, _, _, _, _, _, _) => {
                    h1595
                }
            };
            let a1598 = 0u64;
            nat_rem(a1596, a1597, a1598)
        } else {
            let a1599 = v2;
            let a1600 = 2u64;
            if nat_eq(a1599, a1600) {
                let a1602 = v4;
                let a1603 = match v1 {
                    Adt3::C0(_, _, h1601, _, _, _, _, _) => {
                        h1601
                    }
                };
                let a1604 = 0u64;
                nat_rem(a1602, a1603, a1604)
            } else {
                let a1606 = v4;
                let a1607 = match v1 {
                    Adt3::C0(_, _, h1605, _, _, _, _, _) => {
                        h1605
                    }
                };
                let a1608 = 0u64;
                nat_quot(a1606, a1607, a1608)
            }
        }
    }
}

pub fn f59(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1618 = {
            let a1615 = {
                let a1610 = v2;
                let a1611 = match v0.clone() {
                    Adt6::C0(_, h1609, _, _) => {
                        h1609
                    }
                };
                nat_mul(a1610, a1611)?
            };
            let a1616 = {
                let a1613 = v3;
                let a1614 = match v0.clone() {
                    Adt6::C0(_, _, _, h1612) => {
                        h1612
                    }
                };
                nat_mul(a1613, a1614)?
            };
            nat_add(a1615, a1616)?
        };
        let a1619 = f73(match v0.clone() {
            Adt6::C0(_, _, h1617, _) => {
                h1617
            }
        });
        nat_add(a1618, a1619)?
    };
    let v5: u64 = f72(match v0 {
        Adt6::C0(_, _, h1620, _) => {
            h1620
        }
    });
    let a1621 = v5;
    let a1622 = v4;
    if if nat_le(a1621, a1622) {
        let a1625 = {
            let a1623 = v4;
            let a1624 = v5;
            nat_sub(a1623, a1624)
        };
        let a1626 = v1;
        nat_lt(a1625, a1626)
    } else {
        false
    } {
        Ok(Some({
            let a1627 = v4;
            let a1628 = v5;
            nat_sub(a1627, a1628)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f60(v0: Adt4, v1: Adt8) -> Adt8 {
    let m1629 = v1;
    match m1629 {
        Adt8::C0(v3) => {
            Adt8::C0(v3)
        }
        Adt8::C1(v4, v5, v6) => {
            Adt8::C1(v4, v5, v6)
        }
        Adt8::C2(v7, v8, v9, v10, v11, v12) => {
            Adt8::C2(v7, v8, v9, v10, v11, v12)
        }
        Adt8::C3(v13, v14, v15, v16) => {
            f101(f102(v0.clone(), v13, v14), v13, v14, 0u64, v15, 2u64, f103(v0), 3u64, v16)
        }
        Adt8::C4(v17, v18, v19, v20, v21, v22, v23) => {
            let v24: u64 = f104(v0.clone(), v17);
            f101(f105(v0, v17), v24, v24, v18, v19, v20, v21, v22, v23)
        }
        Adt8::C5(v25, v26, v27, v28, v29, v30) => {
            Adt8::C5(v25, v26, v27, v28, v29, v30)
        }
        Adt8::C6(v31, v32, v33, v34) => {
            Adt8::C6(v31, v32, v33, v34)
        }
        Adt8::C7(v35, v36, v37, v38, v39, v40) => {
            Adt8::C7(v35, v36, v37, v38, v39, v40)
        }
        Adt8::C8(v2) => {
            Adt8::C8(v2)
        }
    }
}

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt8> {
    let m1630 = v3;
    match m1630 {
        Adt4::C0 => {
            Ok(Adt8::C1(false, f106(v0, v1)?, f106(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt8::C1(false, f108(v0, v1, v2.clone())?, f106(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f109(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f110(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f111(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f110(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f112(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f113(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f114(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt8::C1(true, 0u64, f106(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f115(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt8::C3(v28, v29, f106(v0, v1)?, f106(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f116(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt8::C3(v34, v34, f106(v0, v1)?, f106(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt8::C3(v38, v39, f106(v0, v1)?, f106(v0, v2)?))
        }
        Adt4::C15 => {
            f117(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f118(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt4::C18(v42, v43) => {
            f119(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt4::C21 => {
            Ok(Adt8::C1(false, f106(f107(v0, match v2.clone() {
                Adt3::C0(h1631, _, _, _, _, _, _, _) => {
                    h1631
                }
            }, match v2.clone() {
                Adt3::C0(_, h1632, _, _, _, _, _, _) => {
                    h1632
                }
            })?, v1)?, f106(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt8::C0(Adt9::C5(65u64)))
        }
        Adt4::C25(v48, v49, v50) => {
            f120(v0, v48, v49, v50)
        }
        Adt4::C26(v52, _, _, _) => {
            let a1633 = v52;
            let a1634 = 64u64;
            if if if nat_eq(a1633, a1634) {
                true
            } else {
                let a1635 = v52;
                let a1636 = 65u64;
                nat_eq(a1635, a1636)
            } {
                true
            } else {
                let a1637 = v52;
                let a1638 = 66u64;
                nat_eq(a1637, a1638)
            } {
                Ok(Adt8::C0(Adt9::C5(64u64)))
            } else {
                Ok(Adt8::C0(Adt9::C1))
            }
        }
        Adt4::C27(v51) => {
            f66(v0, v1, v2, v51)
        }
    }
}

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt8> {
    let v4: u64 = f96(match v1.clone() {
        Adt3::C0(h1639, _, _, _, _, _, _, _) => {
            h1639
        }
    })?;
    let v5: u64 = {
        let a1640 = v0;
        let a1641 = v4;
        let a1642 = 0u64;
        nat_quot(a1640, a1641, a1642)
    };
    let v6: u64 = {
        let a1643 = v0;
        let a1644 = v4;
        let a1645 = 0u64;
        nat_rem(a1643, a1644, a1645)
    };
    let a1646 = v6;
    let a1647 = 0u64;
    if nat_eq(a1646, a1647) {
        Ok(Adt8::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1648 = v6;
        let a1649 = 1u64;
        if nat_eq(a1648, a1649) {
            Ok(Adt8::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1652 = {
                let a1650 = v6;
                let a1651 = 1u64;
                nat_add(a1650, a1651)?
            };
            let a1653 = v4;
            if nat_eq(a1652, a1653) {
                Ok(Adt8::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f106(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1654 = v6;
                    let a1655 = 2u64;
                    nat_sub(a1654, a1655)
                };
                let v8: u64 = {
                    let a1656 = v7;
                    let a1657 = 3u64;
                    let a1658 = 0u64;
                    nat_quot(a1656, a1657, a1658)
                };
                let v9: u64 = {
                    let a1659 = v7;
                    let a1660 = 3u64;
                    let a1661 = 0u64;
                    nat_rem(a1659, a1660, a1661)
                };
                let v10: u64 = f106({
                    let a1665 = {
                        let a1663 = v5;
                        let a1664 = match v1.clone() {
                            Adt3::C0(h1662, _, _, _, _, _, _, _) => {
                                h1662
                            }
                        };
                        nat_mul(a1663, a1664)?
                    };
                    let a1666 = v8;
                    nat_add(a1665, a1666)?
                }, v1)?;
                let a1667 = v9;
                let a1668 = 0u64;
                if nat_eq(a1667, a1668) {
                    Ok(Adt8::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1669 = v9;
                    let a1670 = 1u64;
                    if nat_eq(a1669, a1670) {
                        let a1671 = v3;
                        let a1672 = 2u64;
                        if nat_eq(a1671, a1672) {
                            Ok(Adt8::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt8::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1673 = v3;
                        let a1674 = 2u64;
                        if if nat_eq(a1673, a1674) {
                            true
                        } else {
                            let a1675 = v3;
                            let a1676 = 3u64;
                            nat_eq(a1675, a1676)
                        } {
                            Ok(Adt8::C4(19u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt8::C4(19u64, 2u64, v8, 2u64, 0u64, 1u64, 1u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt8> {
    let v5: u64 = {
        let a1678 = 2u64;
        let a1679 = match v1.clone() {
            Adt3::C0(_, h1677, _, _, _, _, _, _) => {
                h1677
            }
        };
        nat_mul(a1678, a1679)?
    };
    let a1680 = v0;
    let a1681 = v5;
    if nat_lt(a1680, a1681) {
        let a1685 = {
            let a1682 = v0;
            let a1683 = 2u64;
            let a1684 = 0u64;
            nat_rem(a1682, a1683, a1684)
        };
        let a1686 = 0u64;
        if nat_eq(a1685, a1686) {
            Ok(Adt8::C4(19u64, 4u64, f106({
                let a1687 = v0;
                let a1688 = 2u64;
                let a1689 = 0u64;
                nat_quot(a1687, a1688, a1689)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1690 = v4;
            let a1691 = 2147483648u64;
            if if nat_lt(a1690, a1691) {
                let a1693 = v4;
                let a1694 = match v2 {
                    Adt3::C0(_, h1692, _, _, _, _, _, _) => {
                        h1692
                    }
                };
                nat_lt(a1693, a1694)
            } else {
                false
            } {
                Ok(Adt8::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt8::C0(Adt9::C1))
            }
        }
    } else {
        let v6: u64 = {
            let a1697 = match v2.clone() {
                Adt3::C0(h1695, _, _, _, _, _, _, _) => {
                    h1695
                }
            };
            let a1698 = match v2.clone() {
                Adt3::C0(_, h1696, _, _, _, _, _, _) => {
                    h1696
                }
            };
            nat_mul(a1697, a1698)?
        };
        let a1701 = {
            let a1699 = v0;
            let a1700 = v5;
            nat_sub(a1699, a1700)
        };
        let a1702 = v6;
        if nat_lt(a1701, a1702) {
            Ok(Adt8::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f106({
                let a1703 = v0;
                let a1704 = v5;
                nat_sub(a1703, a1704)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1707 = {
                    let a1705 = v0;
                    let a1706 = v5;
                    nat_sub(a1705, a1706)
                };
                let a1708 = v6;
                nat_sub(a1707, a1708)
            };
            let v8: u64 = {
                let a1712 = 1u64;
                let a1713 = {
                    let a1710 = 2u64;
                    let a1711 = match v1.clone() {
                        Adt3::C0(h1709, _, _, _, _, _, _, _) => {
                            h1709
                        }
                    };
                    nat_mul(a1710, a1711)?
                };
                nat_add(a1712, a1713)?
            };
            let v9: u64 = {
                let a1714 = v7;
                let a1715 = v8;
                let a1716 = 0u64;
                nat_quot(a1714, a1715, a1716)
            };
            let v10: u64 = {
                let a1717 = v7;
                let a1718 = v8;
                let a1719 = 0u64;
                nat_rem(a1717, a1718, a1719)
            };
            let a1720 = v10;
            let a1721 = 0u64;
            if nat_eq(a1720, a1721) {
                Ok(Adt8::C4(19u64, 4u64, f106(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1725 = {
                    let a1722 = v10;
                    let a1723 = 2u64;
                    let a1724 = 0u64;
                    nat_rem(a1722, a1723, a1724)
                };
                let a1726 = 1u64;
                if nat_eq(a1725, a1726) {
                    Ok(Adt8::C4(6u64, 0u64, f106({
                        let a1735 = {
                            let a1728 = v9;
                            let a1729 = match v1.clone() {
                                Adt3::C0(h1727, _, _, _, _, _, _, _) => {
                                    h1727
                                }
                            };
                            nat_mul(a1728, a1729)?
                        };
                        let a1736 = {
                            let a1732 = {
                                let a1730 = v10;
                                let a1731 = 1u64;
                                nat_sub(a1730, a1731)
                            };
                            let a1733 = 2u64;
                            let a1734 = 0u64;
                            nat_quot(a1732, a1733, a1734)
                        };
                        nat_add(a1735, a1736)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt8::C4(4u64, 3u64, f106({
                        let a1745 = {
                            let a1738 = v4;
                            let a1739 = match v1.clone() {
                                Adt3::C0(h1737, _, _, _, _, _, _, _) => {
                                    h1737
                                }
                            };
                            nat_mul(a1738, a1739)?
                        };
                        let a1746 = {
                            let a1742 = {
                                let a1740 = v10;
                                let a1741 = 1u64;
                                nat_sub(a1740, a1741)
                            };
                            let a1743 = 2u64;
                            let a1744 = 0u64;
                            nat_quot(a1742, a1743, a1744)
                        };
                        nat_add(a1745, a1746)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f106({
                        let a1755 = {
                            let a1748 = v4;
                            let a1749 = match v1 {
                                Adt3::C0(h1747, _, _, _, _, _, _, _) => {
                                    h1747
                                }
                            };
                            nat_mul(a1748, a1749)?
                        };
                        let a1756 = {
                            let a1752 = {
                                let a1750 = v10;
                                let a1751 = 1u64;
                                nat_sub(a1750, a1751)
                            };
                            let a1753 = 2u64;
                            let a1754 = 0u64;
                            nat_quot(a1752, a1753, a1754)
                        };
                        nat_add(a1755, a1756)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f64(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt8> {
    let a1757 = v1;
    let a1758 = f121(v2.clone(), v4.clone())?;
    if nat_lt(a1757, a1758) {
        let a1761 = v1;
        let a1762 = {
            let a1759 = 2u64;
            let a1760 = f97(v4.clone())?;
            nat_mul(a1759, a1760)?
        };
        if nat_lt(a1761, a1762) {
            let a1766 = {
                let a1763 = v1;
                let a1764 = 2u64;
                let a1765 = 0u64;
                nat_rem(a1763, a1764, a1765)
            };
            let a1767 = 0u64;
            if nat_eq(a1766, a1767) {
                f122({
                    let a1768 = v1;
                    let a1769 = 2u64;
                    let a1770 = 0u64;
                    nat_quot(a1768, a1769, a1770)
                }, v4)
            } else {
                if f123(match v2 {
                    Adt3::C0(_, h1771, _, _, _, _, _, _) => {
                        h1771
                    }
                }, v5) {
                    Ok(Adt8::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt8::C0(Adt9::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1774 = v1;
                let a1775 = {
                    let a1772 = 2u64;
                    let a1773 = f97(v4.clone())?;
                    nat_mul(a1772, a1773)?
                };
                nat_sub(a1774, a1775)
            };
            let v7: u64 = {
                let a1779 = v6;
                let a1780 = {
                    let a1777 = 1u64;
                    let a1778 = match v2.clone() {
                        Adt3::C0(h1776, _, _, _, _, _, _, _) => {
                            h1776
                        }
                    };
                    nat_add(a1777, a1778)?
                };
                let a1781 = 0u64;
                nat_quot(a1779, a1780, a1781)
            };
            let v8: u64 = {
                let a1785 = v6;
                let a1786 = {
                    let a1783 = 1u64;
                    let a1784 = match v2.clone() {
                        Adt3::C0(h1782, _, _, _, _, _, _, _) => {
                            h1782
                        }
                    };
                    nat_add(a1783, a1784)?
                };
                let a1787 = 0u64;
                nat_rem(a1785, a1786, a1787)
            };
            let a1788 = v8;
            let a1789 = 0u64;
            if nat_eq(a1788, a1789) {
                f122(v7, v4)
            } else {
                f124(v0, v7, {
                    let a1790 = v8;
                    let a1791 = 1u64;
                    nat_sub(a1790, a1791)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt8::C0(Adt9::C1))
    }
}

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt8> {
    let v6: u64 = {
        let a1794 = match v4.clone() {
            Adt3::C0(h1792, _, _, _, _, _, _, _) => {
                h1792
            }
        };
        let a1795 = match v4.clone() {
            Adt3::C0(_, h1793, _, _, _, _, _, _) => {
                h1793
            }
        };
        nat_mul(a1794, a1795)?
    };
    let v7: u64 = {
        let a1796 = 2u64;
        let a1797 = v6;
        nat_mul(a1796, a1797)?
    };
    let v8: u64 = f125(match v3.clone() {
        Adt3::C0(_, h1798, _, _, _, _, _, _) => {
            h1798
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1799, _, _, _, _, _) => {
            h1799
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1800, _, _, _, _) => {
            h1800
        }
    });
    let a1801 = v0;
    let a1802 = v7;
    if nat_lt(a1801, a1802) {
        let a1806 = {
            let a1803 = v0;
            let a1804 = 2u64;
            let a1805 = 0u64;
            nat_rem(a1803, a1804, a1805)
        };
        let a1807 = 0u64;
        if nat_eq(a1806, a1807) {
            Ok(Adt8::C4(19u64, 5u64, f106({
                let a1808 = v0;
                let a1809 = 2u64;
                let a1810 = 0u64;
                nat_quot(a1808, a1809, a1810)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f126(v5, v8) {
                Ok(Adt8::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt8::C0(Adt9::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1811 = v0;
            let a1812 = v7;
            nat_sub(a1811, a1812)
        };
        let v10: u64 = {
            let a1816 = v9;
            let a1817 = {
                let a1814 = 1u64;
                let a1815 = match v1.clone() {
                    Adt3::C0(h1813, _, _, _, _, _, _, _) => {
                        h1813
                    }
                };
                nat_add(a1814, a1815)?
            };
            let a1818 = 0u64;
            nat_quot(a1816, a1817, a1818)
        };
        let v11: u64 = {
            let a1822 = v9;
            let a1823 = {
                let a1820 = 1u64;
                let a1821 = match v1.clone() {
                    Adt3::C0(h1819, _, _, _, _, _, _, _) => {
                        h1819
                    }
                };
                nat_add(a1820, a1821)?
            };
            let a1824 = 0u64;
            nat_rem(a1822, a1823, a1824)
        };
        let a1825 = v11;
        let a1826 = 0u64;
        if nat_eq(a1825, a1826) {
            Ok(Adt8::C4(19u64, 5u64, f106({
                let a1827 = v10;
                let a1828 = v6;
                let a1829 = 0u64;
                nat_rem(a1827, a1828, a1829)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f126(v5, v8) {
                let v12: u64 = {
                    let a1835 = {
                        let a1831 = v10;
                        let a1832 = match v1.clone() {
                            Adt3::C0(h1830, _, _, _, _, _, _, _) => {
                                h1830
                            }
                        };
                        nat_mul(a1831, a1832)?
                    };
                    let a1836 = {
                        let a1833 = v11;
                        let a1834 = 1u64;
                        nat_sub(a1833, a1834)
                    };
                    nat_add(a1835, a1836)?
                };
                let a1837 = v5;
                let a1838 = 4294967295u64;
                if nat_eq(a1837, a1838) {
                    Ok(Adt8::C1(false, f106(v12, v1)?, f106(v12, v2)?))
                } else {
                    Ok(Adt8::C4(8u64, 0u64, f106(v12, v1)?, 4u64, f106({
                        let a1844 = {
                            let a1840 = v5;
                            let a1841 = match v3.clone() {
                                Adt3::C0(h1839, _, _, _, _, _, _, _) => {
                                    h1839
                                }
                            };
                            nat_mul(a1840, a1841)?
                        };
                        let a1845 = {
                            let a1842 = v11;
                            let a1843 = 1u64;
                            nat_sub(a1842, a1843)
                        };
                        nat_add(a1844, a1845)?
                    }, v3)?, 3u64, f106(v12, v2)?))
                }
            } else {
                Ok(Adt8::C0(Adt9::C1))
            }
        }
    }
}

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt8> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a1847 = v5;
        let a1848 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1846, _) => {
                h1846
            }
        };
        let a1849 = 0u64;
        nat_rem(a1847, a1848, a1849)
    };
    let v7: u64 = {
        let a1851 = v5;
        let a1852 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1850, _) => {
                h1850
            }
        };
        let a1853 = 0u64;
        nat_quot(a1851, a1852, a1853)
    };
    let v8: Option<u64> = f59(match v3.clone() {
        Adt5::C0(h1854, _, _, _, _, _, _, _) => {
            h1854
        }
    }, match v1.clone() {
        Adt3::C0(h1855, _, _, _, _, _, _, _) => {
            h1855
        }
    }, f58(v0, v2.clone(), 1u64), {
        let a1859 = v4;
        let a1860 = {
            let m1858 = match v3.clone() {
                Adt5::C0(h1857, _, _, _, _, _, _, _) => {
                    h1857
                }
            };
            match m1858 {
                Adt6::C0(h1856, _, _, _) => {
                    h1856
                }
            }
        };
        let a1861 = 0u64;
        nat_rem(a1859, a1860, a1861)
    })?;
    let v9: u64 = {
        let a1865 = v4;
        let a1866 = {
            let m1864 = match v3.clone() {
                Adt5::C0(h1863, _, _, _, _, _, _, _) => {
                    h1863
                }
            };
            match m1864 {
                Adt6::C0(h1862, _, _, _) => {
                    h1862
                }
            }
        };
        let a1867 = 0u64;
        nat_quot(a1865, a1866, a1867)
    };
    let v10: Option<u64> = f59(match v3.clone() {
        Adt5::C0(_, h1868, _, _, _, _, _, _) => {
            h1868
        }
    }, match v1.clone() {
        Adt3::C0(_, h1869, _, _, _, _, _, _) => {
            h1869
        }
    }, f58(v0, v2.clone(), 2u64), {
        let a1873 = v9;
        let a1874 = {
            let m1872 = match v3.clone() {
                Adt5::C0(_, h1871, _, _, _, _, _, _) => {
                    h1871
                }
            };
            match m1872 {
                Adt6::C0(h1870, _, _, _) => {
                    h1870
                }
            }
        };
        let a1875 = 0u64;
        nat_rem(a1873, a1874, a1875)
    })?;
    let v11: u64 = {
        let a1879 = v9;
        let a1880 = {
            let m1878 = match v3.clone() {
                Adt5::C0(_, h1877, _, _, _, _, _, _) => {
                    h1877
                }
            };
            match m1878 {
                Adt6::C0(h1876, _, _, _) => {
                    h1876
                }
            }
        };
        let a1881 = 0u64;
        nat_quot(a1879, a1880, a1881)
    };
    let v12: Option<u64> = f59(match v3.clone() {
        Adt5::C0(_, _, h1882, _, _, _, _, _) => {
            h1882
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1883, _, _, _, _, _) => {
            h1883
        }
    }, v6, {
        let a1887 = v11;
        let a1888 = {
            let m1886 = match v3.clone() {
                Adt5::C0(_, _, h1885, _, _, _, _, _) => {
                    h1885
                }
            };
            match m1886 {
                Adt6::C0(h1884, _, _, _) => {
                    h1884
                }
            }
        };
        let a1889 = 0u64;
        nat_rem(a1887, a1888, a1889)
    })?;
    let v13: u64 = {
        let a1893 = v11;
        let a1894 = {
            let m1892 = match v3.clone() {
                Adt5::C0(_, _, h1891, _, _, _, _, _) => {
                    h1891
                }
            };
            match m1892 {
                Adt6::C0(h1890, _, _, _) => {
                    h1890
                }
            }
        };
        let a1895 = 0u64;
        nat_quot(a1893, a1894, a1895)
    };
    let v17: Adt11 = {
        let m1896 = v8;
        match m1896 {
            None => {
                Adt11::C2(0u64)
            }
            Some(v14) => {
                let m1897 = v10;
                match m1897 {
                    None => {
                        Adt11::C2(0u64)
                    }
                    Some(v15) => {
                        let m1898 = v12;
                        match m1898 {
                            None => {
                                Adt11::C2(0u64)
                            }
                            Some(v16) => {
                                Adt11::C0(1u64, {
                                    let a1907 = {
                                        let a1902 = f57(v1, v14, v15, v16, 0u64)?;
                                        let a1903 = {
                                            let a1900 = v7;
                                            let a1901 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h1899, _, _, _) => {
                                                    h1899
                                                }
                                            };
                                            nat_mul(a1900, a1901)?
                                        };
                                        nat_add(a1902, a1903)?
                                    };
                                    let a1908 = {
                                        let a1905 = v13;
                                        let a1906 = match v3.clone() {
                                            Adt5::C0(_, _, _, h1904, _, _, _, _) => {
                                                h1904
                                            }
                                        };
                                        nat_mul(a1905, a1906)?
                                    };
                                    nat_add(a1907, a1908)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt8::C7(Adt10::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h1909, _, _) => {
            h1909
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h1910) => {
            h1910
        }
    }, true), v17, Adt11::C2(0u64), Adt12::C1(f106(v0, v2)?), {
        let a1912 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h1911, _, _) => {
                h1911
            }
        };
        let a1913 = 0u64;
        if nat_eq(a1912, a1913) {
            4u64
        } else {
            2u64
        }
    }, {
        let a1915 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h1914) => {
                h1914
            }
        };
        let a1916 = 0u64;
        if nat_eq(a1915, a1916) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt8> {
    let v8: u64 = {
        let a1919 = v6;
        let a1920 = {
            let a1917 = 3u64;
            let a1918 = v5;
            nat_add(a1917, a1918)?
        };
        nat_sub(a1919, a1920)
    };
    let a1922 = v0;
    let a1923 = f29(v3, match v1.clone() {
        Adt3::C0(h1921, _, _, _, _, _, _, _) => {
            h1921
        }
    }, v5, v8)?;
    if nat_lt(a1922, a1923) {
        let v9: u64 = f78(match v1.clone() {
            Adt3::C0(h1924, _, _, _, _, _, _, _) => {
                h1924
            }
        }, v5, v8)?;
        let v10: u64 = f79(v3, match v1.clone() {
            Adt3::C0(h1925, _, _, _, _, _, _, _) => {
                h1925
            }
        }, v5)?;
        let a1926 = v0;
        let a1927 = v9;
        if nat_lt(a1926, a1927) {
            f127(v0, v2, f23(v3), {
                let a1928 = 3u64;
                let a1929 = v5;
                nat_add(a1928, a1929)?
            }, v7)
        } else {
            let a1932 = v0;
            let a1933 = {
                let a1930 = v9;
                let a1931 = v10;
                nat_add(a1930, a1931)?
            };
            if nat_lt(a1932, a1933) {
                let v11: u64 = {
                    let a1934 = v0;
                    let a1935 = v9;
                    nat_sub(a1934, a1935)
                };
                let v12: u64 = f128(v3, match v1.clone() {
                    Adt3::C0(h1936, _, _, _, _, _, _, _) => {
                        h1936
                    }
                })?;
                f129({
                    let a1937 = v11;
                    let a1938 = v12;
                    let a1939 = 0u64;
                    nat_rem(a1937, a1938, a1939)
                }, {
                    let a1940 = v11;
                    let a1941 = v12;
                    let a1942 = 0u64;
                    nat_quot(a1940, a1941, a1942)
                }, v1, v3, v4, v7)
            } else {
                f130({
                    let a1945 = v0;
                    let a1946 = {
                        let a1943 = v9;
                        let a1944 = v10;
                        nat_add(a1943, a1944)?
                    };
                    nat_sub(a1945, a1946)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt8::C0(Adt9::C1))
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt8> {
    let a1947 = v0;
    let a1948 = f15(v1)?;
    if nat_lt(a1947, a1948) {
        let a1949 = v0;
        let a1950 = 4u64;
        if nat_lt(a1949, a1950) {
            let a1951 = v0;
            let a1952 = 0u64;
            if nat_eq(a1951, a1952) {
                Ok(Adt8::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1953 = v0;
                let a1954 = 1u64;
                if nat_eq(a1953, a1954) {
                    let a1955 = v5;
                    let a1956 = 0u64;
                    if nat_eq(a1955, a1956) {
                        Ok(Adt8::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt8::C0(Adt9::C1))
                    }
                } else {
                    let a1957 = v0;
                    let a1958 = 2u64;
                    if nat_eq(a1957, a1958) {
                        Ok(Adt8::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt8::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1959 = v0;
            let a1960 = 4u64;
            if if nat_eq(a1959, a1960) {
                let a1961 = f131(v5)?;
                let a1962 = v1;
                !nat_eq(a1961, a1962)
            } else {
                false
            } {
                Ok(Adt8::C0(Adt9::C1))
            } else {
                let v6: u64 = {
                    let a1965 = {
                        let a1963 = v0;
                        let a1964 = 4u64;
                        nat_sub(a1963, a1964)
                    };
                    let a1966 = 3u64;
                    let a1967 = 0u64;
                    nat_quot(a1965, a1966, a1967)
                };
                let v7: u64 = {
                    let a1970 = {
                        let a1968 = v0;
                        let a1969 = 4u64;
                        nat_sub(a1968, a1969)
                    };
                    let a1971 = 3u64;
                    let a1972 = 0u64;
                    nat_rem(a1970, a1971, a1972)
                };
                let a1973 = v7;
                let a1974 = 0u64;
                if nat_eq(a1973, a1974) {
                    Ok(Adt8::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1975 = v7;
                    let a1976 = 1u64;
                    if nat_eq(a1975, a1976) {
                        Ok(Adt8::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt8::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f14(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt8::C0(Adt9::C1))
    }
}

pub fn f69(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt10 {
    let a1977 = v1;
    let a1978 = 0u64;
    if nat_eq(a1977, a1978) {
        if v0 {
            Adt10::C2(v3, v4, false)
        } else {
            Adt10::C0(v2)
        }
    } else {
        let a1979 = v1;
        let a1980 = 1u64;
        if nat_eq(a1979, a1980) {
            Adt10::C1(v2)
        } else {
            let a1981 = v1;
            let a1982 = 2u64;
            if nat_eq(a1981, a1982) {
                Adt10::C5(v2)
            } else {
                let a1983 = v1;
                let a1984 = 3u64;
                if nat_eq(a1983, a1984) {
                    Adt10::C3(v2, v2, false)
                } else {
                    let a1985 = v1;
                    let a1986 = 4u64;
                    if nat_eq(a1985, a1986) {
                        Adt10::C2(v3, v4, false)
                    } else {
                        Adt10::C6
                    }
                }
            }
        }
    }
}

pub fn f70(v0: u64, v1: u64) -> Adt11 {
    let a1987 = v0;
    let a1988 = 0u64;
    if nat_eq(a1987, a1988) {
        Adt11::C0(0u64, v1)
    } else {
        let a1989 = v0;
        let a1990 = 1u64;
        if nat_eq(a1989, a1990) {
            Adt11::C1(v1)
        } else {
            let a1991 = v0;
            let a1992 = 2u64;
            if nat_eq(a1991, a1992) {
                Adt11::C2(v1)
            } else {
                let a1993 = v0;
                let a1994 = 3u64;
                if nat_eq(a1993, a1994) {
                    Adt11::C3(v1)
                } else {
                    let a1995 = v0;
                    let a1996 = 13u64;
                    if nat_lt(a1995, a1996) {
                        Adt11::C0({
                            let a1997 = v0;
                            let a1998 = 3u64;
                            nat_sub(a1997, a1998)
                        }, v1)
                    } else {
                        Adt11::C4
                    }
                }
            }
        }
    }
}

pub fn f71(v0: u64, v1: u64) -> Adt12 {
    let a1999 = v0;
    let a2000 = 1u64;
    if nat_eq(a1999, a2000) {
        Adt12::C0(v1)
    } else {
        let a2001 = v0;
        let a2002 = 3u64;
        if if nat_eq(a2001, a2002) {
            true
        } else {
            let a2003 = v0;
            let a2004 = 6u64;
            nat_eq(a2003, a2004)
        } {
            Adt12::C1(v1)
        } else {
            Adt12::C2
        }
    }
}

pub fn f72(v0: u64) -> u64 {
    let a2005 = v0;
    let a2006 = 2147483648u64;
    if nat_lt(a2005, a2006) {
        v0
    } else {
        0u64
    }
}

pub fn f73(v0: u64) -> u64 {
    let a2007 = v0;
    let a2008 = 2147483648u64;
    if nat_lt(a2007, a2008) {
        0u64
    } else {
        let a2009 = 4294967296u64;
        let a2010 = v0;
        nat_sub(a2009, a2010)
    }
}

pub fn f74(v0: u64) -> u64 {
    let v1: u64 = {
        let a2011 = v0;
        let a2012 = 4294967296u64;
        let a2013 = 0u64;
        nat_rem(a2011, a2012, a2013)
    };
    let a2014 = v1;
    let a2015 = 2147483648u64;
    if nat_lt(a2014, a2015) {
        v1
    } else {
        0u64
    }
}

pub fn f75(v0: u64) -> R<u64> {
    let a2018 = 2u64;
    let a2019 = {
        let a2016 = 2u64;
        let a2017 = v0;
        nat_mul(a2016, a2017)?
    };
    nat_add(a2018, a2019)
}

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f132(v0, v3, v6) {
        f132(v1, v4, v6)
    } else {
        false
    } {
        f132(v2, v5, v6)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f133(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f133(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f133(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f133(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f133(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f133(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2020 = v2;
    let a2021 = 0u64;
    if nat_eq(a2020, a2021) {
        Ok(0u64)
    } else {
        let a2024 = 2u64;
        let a2025 = {
            let a2022 = v0;
            let a2023 = v1;
            nat_mul(a2022, a2023)?
        };
        nat_mul(a2024, a2025)
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2026 = v2;
    let a2027 = f128(v0, v1)?;
    nat_mul(a2026, a2027)
}

pub fn f80(v0: u64) -> u64 {
    let a2028 = v0;
    let a2029 = 30u64;
    if nat_eq(a2028, a2029) {
        3u64
    } else {
        1u64
    }
}

pub fn f81(v0: u64) -> R<u64> {
    let a2032 = 5u64;
    let a2033 = {
        let a2030 = 4u64;
        let a2031 = v0;
        nat_mul(a2030, a2031)?
    };
    nat_add(a2032, a2033)
}

pub fn f82(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt9> {
    if f134(match v0.clone() {
        Adt2::C0(h2034, _, _, _, _, _, _, _, _, _, _) => {
            h2034
        }
    }, match v0.clone() {
        Adt2::C0(_, h2035, _, _, _, _, _, _, _, _, _) => {
            h2035
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2036, _, _, _, _, _, _, _, _) => {
            h2036
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2037, _, _, _, _, _, _, _) => {
            h2037
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2038, _, _, _, _, _, _) => {
            h2038
        }
    }, match v1.clone() {
        Adt2::C0(h2039, _, _, _, _, _, _, _, _, _, _) => {
            h2039
        }
    }, match v1.clone() {
        Adt2::C0(_, h2040, _, _, _, _, _, _, _, _, _) => {
            h2040
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2041, _, _, _, _, _, _, _, _) => {
            h2041
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2042, _, _, _, _, _, _, _) => {
            h2042
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2043, _, _, _, _, _, _) => {
            h2043
        }
    }, match v2.clone() {
        Adt2::C0(h2044, _, _, _, _, _, _, _, _, _, _) => {
            h2044
        }
    }, match v2.clone() {
        Adt2::C0(_, h2045, _, _, _, _, _, _, _, _, _) => {
            h2045
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2046, _, _, _, _, _, _, _, _) => {
            h2046
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2047, _, _, _, _, _, _, _) => {
            h2047
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2048, _, _, _, _, _, _) => {
            h2048
        }
    }) {
        let m2050 = f86(match v0.clone() {
            Adt2::C0(h2049, _, _, _, _, _, _, _, _, _, _) => {
                h2049
            }
        });
        match m2050 {
            Adt13::C0 => {
                Ok(Adt9::C0)
            }
            Adt13::C1(v3) => {
                let a2056 = {
                    let a2053 = match v0.clone() {
                        Adt2::C0(_, h2051, _, _, _, _, _, _, _, _, _) => {
                            h2051
                        }
                    };
                    let a2054 = match v3.clone() {
                        Adt14::C0(_, h2052, _) => {
                            h2052
                        }
                    };
                    let a2055 = 0u64;
                    nat_rem(a2053, a2054, a2055)
                };
                let a2057 = 0u64;
                if if nat_eq(a2056, a2057) {
                    if f135(match v0.clone() {
                        Adt2::C0(_, h2058, _, _, _, _, _, _, _, _, _) => {
                            h2058
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2059, _, _, _, _, _, _, _, _, _) => {
                            h2059
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2060, _, _, _, _, _, _, _, _) => {
                            h2060
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2061, _, _, _, _, _, _, _) => {
                            h2061
                        }
                    }) {
                        true
                    } else {
                        let a2063 = 4u64;
                        let a2064 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2062, _, _, _, _, _) => {
                                h2062
                            }
                        };
                        if if nat_le(a2063, a2064) {
                            let a2066 = match v3.clone() {
                                Adt14::C0(_, h2065, _) => {
                                    h2065
                                }
                            };
                            let a2067 = 1u64;
                            if nat_eq(a2066, a2067) {
                                true
                            } else {
                                let a2070 = match v3.clone() {
                                    Adt14::C0(_, _, h2068) => {
                                        h2068
                                    }
                                };
                                let a2071 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2069, _, _, _, _, _) => {
                                        h2069
                                    }
                                };
                                nat_le(a2070, a2071)
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
                        Adt14::C0(_, h2072, _) => {
                            h2072
                        }
                    });
                    Ok(Adt9::C4(if f135(match v0.clone() {
                        Adt2::C0(_, h2073, _, _, _, _, _, _, _, _, _) => {
                            h2073
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2074, _, _, _, _, _, _, _, _, _) => {
                            h2074
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2075, _, _, _, _, _, _, _, _) => {
                            h2075
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2076, _, _, _, _, _, _, _) => {
                            h2076
                        }
                    }) {
                        0u64
                    } else {
                        f121(v4.clone(), f16(v1.clone(), 1u64))?
                    }, 4u64, v4, f16(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2077, _, _, _, _, _, _, _, _, _, _) => {
                            h2077
                        }
                    }, f16(v1, 1u64))))
                } else {
                    Ok(Adt9::C1)
                }
            }
        }
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f83(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt9> {
    if if if if if if if if if if if if f85(match v0.clone() {
        Adt2::C0(h2078, _, _, _, _, _, _, _, _, _, _) => {
            h2078
        }
    }) {
        let a2080 = match v1.clone() {
            Adt2::C0(h2079, _, _, _, _, _, _, _, _, _, _) => {
                h2079
            }
        };
        let a2081 = 26u64;
        nat_eq(a2080, a2081)
    } else {
        false
    } {
        let a2083 = match v2.clone() {
            Adt2::C0(h2082, _, _, _, _, _, _, _, _, _, _) => {
                h2082
            }
        };
        let a2084 = 0u64;
        nat_eq(a2083, a2084)
    } else {
        false
    } {
        let a2087 = match v0.clone() {
            Adt2::C0(_, h2085, _, _, _, _, _, _, _, _, _) => {
                h2085
            }
        };
        let a2088 = match v2.clone() {
            Adt2::C0(_, h2086, _, _, _, _, _, _, _, _, _) => {
                h2086
            }
        };
        nat_eq(a2087, a2088)
    } else {
        false
    } {
        let a2091 = match v1.clone() {
            Adt2::C0(_, h2089, _, _, _, _, _, _, _, _, _) => {
                h2089
            }
        };
        let a2092 = match v0.clone() {
            Adt2::C0(_, _, h2090, _, _, _, _, _, _, _, _) => {
                h2090
            }
        };
        nat_le(a2091, a2092)
    } else {
        false
    } {
        let a2094 = match v0.clone() {
            Adt2::C0(_, _, _, h2093, _, _, _, _, _, _, _) => {
                h2093
            }
        };
        let a2095 = 1u64;
        nat_eq(a2094, a2095)
    } else {
        false
    } {
        let a2097 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2096, _, _, _, _, _, _) => {
                h2096
            }
        };
        let a2098 = 1u64;
        nat_eq(a2097, a2098)
    } else {
        false
    } {
        let a2100 = match v2.clone() {
            Adt2::C0(_, _, _, h2099, _, _, _, _, _, _, _) => {
                h2099
            }
        };
        let a2101 = 1u64;
        nat_eq(a2100, a2101)
    } else {
        false
    } {
        let a2103 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2102, _, _, _, _, _, _) => {
                h2102
            }
        };
        let a2104 = 1u64;
        nat_eq(a2103, a2104)
    } else {
        false
    } {
        let a2106 = match v1.clone() {
            Adt2::C0(_, _, h2105, _, _, _, _, _, _, _, _) => {
                h2105
            }
        };
        let a2107 = 1u64;
        nat_eq(a2106, a2107)
    } else {
        false
    } {
        let a2109 = match v1.clone() {
            Adt2::C0(_, _, _, h2108, _, _, _, _, _, _, _) => {
                h2108
            }
        };
        let a2110 = 1u64;
        nat_eq(a2109, a2110)
    } else {
        false
    } {
        let a2112 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2111, _, _, _, _, _, _) => {
                h2111
            }
        };
        let a2113 = 1u64;
        nat_eq(a2112, a2113)
    } else {
        false
    } {
        if f27(match v2.clone() {
            Adt2::C0(_, h2114, _, _, _, _, _, _, _, _, _) => {
                h2114
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2115, _, _, _, _, _, _, _, _) => {
                h2115
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2116, _, _, _, _, _, _, _) => {
                h2116
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2117, _, _, _, _, _, _) => {
                h2117
            }
        }) {
            Ok(Adt9::C4(0u64, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2119 = match v1.clone() {
                Adt2::C0(_, h2118, _, _, _, _, _, _, _, _, _) => {
                    h2118
                }
            };
            let a2120 = 0u64;
            if nat_eq(a2119, a2120) {
                Ok(Adt9::C4(f12(v2.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2122 = match v0.clone() {
                    Adt2::C0(_, h2121, _, _, _, _, _, _, _, _, _) => {
                        h2121
                    }
                };
                let a2123 = 2147483647u64;
                if if if if if if nat_le(a2122, a2123) {
                    let a2125 = match v1.clone() {
                        Adt2::C0(_, h2124, _, _, _, _, _, _, _, _, _) => {
                            h2124
                        }
                    };
                    let a2126 = 2147483647u64;
                    nat_le(a2125, a2126)
                } else {
                    false
                } {
                    let a2128 = 4u64;
                    let a2129 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2127, _, _, _, _, _) => {
                            h2127
                        }
                    };
                    nat_le(a2128, a2129)
                } else {
                    false
                } {
                    let a2135 = {
                        let a2132 = match v2.clone() {
                            Adt2::C0(_, h2130, _, _, _, _, _, _, _, _, _) => {
                                h2130
                            }
                        };
                        let a2133 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2131, _, _, _, _, _) => {
                                h2131
                            }
                        };
                        nat_mul(a2132, a2133)?
                    };
                    let a2136 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2134, _, _, _, _) => {
                            h2134
                        }
                    };
                    nat_le(a2135, a2136)
                } else {
                    false
                } {
                    f18(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f18(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt9::C4({
                        let a2150 = {
                            let a2140 = {
                                let a2138 = 2u64;
                                let a2139 = match v1.clone() {
                                    Adt2::C0(_, h2137, _, _, _, _, _, _, _, _, _) => {
                                        h2137
                                    }
                                };
                                nat_mul(a2138, a2139)?
                            };
                            let a2141 = f12(v2.clone(), 1u64)?;
                            nat_add(a2140, a2141)?
                        };
                        let a2151 = {
                            let a2148 = match v1.clone() {
                                Adt2::C0(_, h2142, _, _, _, _, _, _, _, _, _) => {
                                    h2142
                                }
                            };
                            let a2149 = {
                                let a2146 = 1u64;
                                let a2147 = {
                                    let a2144 = 2u64;
                                    let a2145 = match v0.clone() {
                                        Adt2::C0(_, h2143, _, _, _, _, _, _, _, _, _) => {
                                            h2143
                                        }
                                    };
                                    nat_mul(a2144, a2145)?
                                };
                                nat_add(a2146, a2147)?
                            };
                            nat_mul(a2148, a2149)?
                        };
                        nat_add(a2150, a2151)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2152, _, _, _, _, _, _, _, _, _) => {
                            h2152
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2153, _, _, _, _, _, _, _, _, _) => {
                            h2153
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2154, _, _, _, _, _) => {
                            h2154
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2155, _, _, _, _) => {
                            h2155
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2156, _, _, _) => {
                            h2156
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2157, _, _) => {
                            h2157
                        }
                    }), f16(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2158, _, _, _, _, _, _, _, _, _, _) => {
                            h2158
                        }
                    }, f16(v1, 1u64))))
                } else {
                    Ok(Adt9::C1)
                }
            }
        }
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f84(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt9> {
    let a2161 = match v1.clone() {
        Adt2::C0(h2159, _, _, _, _, _, _, _, _, _, _) => {
            h2159
        }
    };
    let a2162 = match v3.clone() {
        Adt2::C0(h2160, _, _, _, _, _, _, _, _, _, _) => {
            h2160
        }
    };
    if if nat_eq(a2161, a2162) {
        let a2163 = v0;
        let a2164 = 33u64;
        if nat_eq(a2163, a2164) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2166 = f86(match v1.clone() {
            Adt2::C0(h2165, _, _, _, _, _, _, _, _, _, _) => {
                h2165
            }
        });
        match m2166 {
            Adt13::C0 => {
                Ok(Adt9::C0)
            }
            Adt13::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt14::C0(_, h2167, _) => {
                        h2167
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a2172 = {
                    let a2169 = match v1.clone() {
                        Adt2::C0(_, h2168, _, _, _, _, _, _, _, _, _) => {
                            h2168
                        }
                    };
                    let a2170 = v5;
                    let a2171 = 1u64;
                    nat_rem(a2169, a2170, a2171)
                };
                let a2173 = 0u64;
                if if if nat_eq(a2172, a2173) {
                    let a2178 = {
                        let a2175 = match v3.clone() {
                            Adt2::C0(_, h2174, _, _, _, _, _, _, _, _, _) => {
                                h2174
                            }
                        };
                        let a2176 = v5;
                        let a2177 = 1u64;
                        nat_rem(a2175, a2176, a2177)
                    };
                    let a2179 = 0u64;
                    nat_eq(a2178, a2179)
                } else {
                    false
                } {
                    let a2180 = v6;
                    let a2181 = f12(v3.clone(), v5)?;
                    nat_eq(a2180, a2181)
                } else {
                    false
                } {
                    Ok(Adt9::C4(v6, match v4 {
                        Adt14::C0(_, _, h2182) => {
                            h2182
                        }
                    }, f16(v1, v5), f16(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt9::C1)
                }
            }
        }
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f85(v0: u64) -> bool {
    let a2183 = v0;
    let a2184 = 0u64;
    if if nat_eq(a2183, a2184) {
        true
    } else {
        let a2185 = v0;
        let a2186 = 1u64;
        nat_eq(a2185, a2186)
    } {
        true
    } else {
        let a2187 = v0;
        let a2188 = 30u64;
        nat_eq(a2187, a2188)
    }
}

pub fn f86(v0: u64) -> Adt13 {
    let a2189 = v0;
    let a2190 = 0u64;
    if nat_eq(a2189, a2190) {
        Adt13::C1(Adt14::C0(0u64, 1u64, 4u64))
    } else {
        let a2191 = v0;
        let a2192 = 1u64;
        if nat_eq(a2191, a2192) {
            Adt13::C1(Adt14::C0(1u64, 1u64, 2u64))
        } else {
            let a2193 = v0;
            let a2194 = 2u64;
            if nat_eq(a2193, a2194) {
                Adt13::C1(Adt14::C0(2u64, 32u64, 18u64))
            } else {
                let a2195 = v0;
                let a2196 = 3u64;
                if nat_eq(a2195, a2196) {
                    Adt13::C1(Adt14::C0(3u64, 32u64, 20u64))
                } else {
                    let a2197 = v0;
                    let a2198 = 6u64;
                    if nat_eq(a2197, a2198) {
                        Adt13::C1(Adt14::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2199 = v0;
                        let a2200 = 7u64;
                        if nat_eq(a2199, a2200) {
                            Adt13::C1(Adt14::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2201 = v0;
                            let a2202 = 8u64;
                            if nat_eq(a2201, a2202) {
                                Adt13::C1(Adt14::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2203 = v0;
                                let a2204 = 9u64;
                                if nat_eq(a2203, a2204) {
                                    Adt13::C1(Adt14::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2205 = v0;
                                    let a2206 = 10u64;
                                    if nat_eq(a2205, a2206) {
                                        Adt13::C1(Adt14::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2207 = v0;
                                        let a2208 = 11u64;
                                        if nat_eq(a2207, a2208) {
                                            Adt13::C1(Adt14::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2209 = v0;
                                            let a2210 = 12u64;
                                            if nat_eq(a2209, a2210) {
                                                Adt13::C1(Adt14::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2211 = v0;
                                                let a2212 = 13u64;
                                                if nat_eq(a2211, a2212) {
                                                    Adt13::C1(Adt14::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2213 = v0;
                                                    let a2214 = 14u64;
                                                    if nat_eq(a2213, a2214) {
                                                        Adt13::C1(Adt14::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2215 = v0;
                                                        let a2216 = 15u64;
                                                        if nat_eq(a2215, a2216) {
                                                            Adt13::C1(Adt14::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2217 = v0;
                                                            let a2218 = 16u64;
                                                            if nat_eq(a2217, a2218) {
                                                                Adt13::C1(Adt14::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2219 = v0;
                                                                let a2220 = 17u64;
                                                                if nat_eq(a2219, a2220) {
                                                                    Adt13::C1(Adt14::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2221 = v0;
                                                                    let a2222 = 18u64;
                                                                    if nat_eq(a2221, a2222) {
                                                                        Adt13::C1(Adt14::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2223 = v0;
                                                                        let a2224 = 19u64;
                                                                        if nat_eq(a2223, a2224) {
                                                                            Adt13::C1(Adt14::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2225 = v0;
                                                                            let a2226 = 20u64;
                                                                            if nat_eq(a2225, a2226) {
                                                                                Adt13::C1(Adt14::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2227 = v0;
                                                                                let a2228 = 21u64;
                                                                                if nat_eq(a2227, a2228) {
                                                                                    Adt13::C1(Adt14::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2229 = v0;
                                                                                    let a2230 = 22u64;
                                                                                    if nat_eq(a2229, a2230) {
                                                                                        Adt13::C1(Adt14::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2231 = v0;
                                                                                        let a2232 = 23u64;
                                                                                        if nat_eq(a2231, a2232) {
                                                                                            Adt13::C1(Adt14::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2233 = v0;
                                                                                            let a2234 = 24u64;
                                                                                            if nat_eq(a2233, a2234) {
                                                                                                Adt13::C1(Adt14::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2235 = v0;
                                                                                                let a2236 = 25u64;
                                                                                                if nat_eq(a2235, a2236) {
                                                                                                    Adt13::C1(Adt14::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2237 = v0;
                                                                                                    let a2238 = 26u64;
                                                                                                    if nat_eq(a2237, a2238) {
                                                                                                        Adt13::C1(Adt14::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2239 = v0;
                                                                                                        let a2240 = 27u64;
                                                                                                        if nat_eq(a2239, a2240) {
                                                                                                            Adt13::C1(Adt14::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2241 = v0;
                                                                                                            let a2242 = 28u64;
                                                                                                            if nat_eq(a2241, a2242) {
                                                                                                                Adt13::C1(Adt14::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2243 = v0;
                                                                                                                let a2244 = 29u64;
                                                                                                                if nat_eq(a2243, a2244) {
                                                                                                                    Adt13::C1(Adt14::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2245 = v0;
                                                                                                                    let a2246 = 30u64;
                                                                                                                    if nat_eq(a2245, a2246) {
                                                                                                                        Adt13::C1(Adt14::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2247 = v0;
                                                                                                                        let a2248 = 34u64;
                                                                                                                        if nat_eq(a2247, a2248) {
                                                                                                                            Adt13::C1(Adt14::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2249 = v0;
                                                                                                                            let a2250 = 35u64;
                                                                                                                            if nat_eq(a2249, a2250) {
                                                                                                                                Adt13::C1(Adt14::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2251 = v0;
                                                                                                                                let a2252 = 39u64;
                                                                                                                                if nat_eq(a2251, a2252) {
                                                                                                                                    Adt13::C1(Adt14::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2253 = v0;
                                                                                                                                    let a2254 = 40u64;
                                                                                                                                    if nat_eq(a2253, a2254) {
                                                                                                                                        Adt13::C1(Adt14::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2255 = v0;
                                                                                                                                        let a2256 = 41u64;
                                                                                                                                        if nat_eq(a2255, a2256) {
                                                                                                                                            Adt13::C1(Adt14::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2257 = v0;
                                                                                                                                            let a2258 = 42u64;
                                                                                                                                            if nat_eq(a2257, a2258) {
                                                                                                                                                Adt13::C1(Adt14::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2259 = v0;
                                                                                                                                                let a2260 = 43u64;
                                                                                                                                                if nat_eq(a2259, a2260) {
                                                                                                                                                    Adt13::C1(Adt14::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt13::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f27(v4, v5, v6, v7) {
        f27(v0, v1, v2, v3)
    } else {
        let a2264 = {
            let a2261 = v0;
            let a2262 = v4;
            let a2263 = 1u64;
            nat_rem(a2261, a2262, a2263)
        };
        let a2265 = 0u64;
        if if if nat_eq(a2264, a2265) {
            let a2269 = {
                let a2266 = v1;
                let a2267 = v5;
                let a2268 = 1u64;
                nat_rem(a2266, a2267, a2268)
            };
            let a2270 = 0u64;
            nat_eq(a2269, a2270)
        } else {
            false
        } {
            let a2274 = {
                let a2271 = v2;
                let a2272 = v6;
                let a2273 = 1u64;
                nat_rem(a2271, a2272, a2273)
            };
            let a2275 = 0u64;
            nat_eq(a2274, a2275)
        } else {
            false
        } {
            let a2279 = {
                let a2276 = v3;
                let a2277 = v7;
                let a2278 = 1u64;
                nat_rem(a2276, a2277, a2278)
            };
            let a2280 = 0u64;
            nat_eq(a2279, a2280)
        } else {
            false
        }
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2281 = v0;
    let a2282 = v1;
    if nat_eq(a2281, a2282) {
        let a2285 = v4;
        let a2286 = {
            let a2283 = v2;
            let a2284 = v3;
            nat_add(a2283, a2284)?
        };
        Ok(nat_eq(a2285, a2286))
    } else {
        let a2287 = v2;
        let a2288 = v3;
        if nat_eq(a2287, a2288) {
            let a2289 = v4;
            let a2290 = v2;
            Ok(nat_eq(a2289, a2290))
        } else {
            Ok(false)
        }
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2293 = {
            let a2291 = v0;
            let a2292 = f72(v1);
            nat_add(a2291, a2292)?
        };
        let a2294 = f72(v2);
        nat_add(a2293, a2294)?
    };
    let v5: u64 = {
        let a2295 = f73(v1);
        let a2296 = f73(v2);
        nat_add(a2295, a2296)?
    };
    let a2297 = v1;
    let a2298 = 4294967295u64;
    if if nat_le(a2297, a2298) {
        let a2299 = v2;
        let a2300 = 4294967295u64;
        nat_le(a2299, a2300)
    } else {
        false
    } {
        let a2301 = v5;
        let a2302 = v4;
        if nat_le(a2301, a2302) {
            let a2305 = v3;
            let a2306 = {
                let a2303 = v4;
                let a2304 = v5;
                nat_sub(a2303, a2304)
            };
            Ok(nat_eq(a2305, a2306))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f90(v0: u64, v1: u64) -> R<bool> {
    let a2307 = v1;
    let a2308 = 4294967295u64;
    if nat_le(a2307, a2308) {
        let a2311 = {
            let a2309 = f72(v1);
            let a2310 = f73(v1);
            nat_add(a2309, a2310)?
        };
        let a2312 = v0;
        Ok(nat_lt(a2311, a2312))
    } else {
        Ok(false)
    }
}

pub fn f91(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f136(v0.clone(), false)?;
    let v6: u64 = f136(v0, true)?;
    let a2313 = v6;
    let a2314 = v1;
    if if nat_le(a2313, a2314) {
        let a2321 = {
            let a2319 = {
                let a2315 = v1;
                let a2316 = v5;
                nat_add(a2315, a2316)?
            };
            let a2320 = {
                let a2317 = v3;
                let a2318 = 0u64;
                if nat_eq(a2317, a2318) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2319, a2320)?
        };
        let a2322 = v2;
        nat_le(a2321, a2322)
    } else {
        false
    } {
        if v4 {
            let a2327 = {
                let a2325 = {
                    let a2323 = v1;
                    let a2324 = v5;
                    nat_add(a2323, a2324)?
                };
                let a2326 = v6;
                nat_sub(a2325, a2326)
            };
            let a2328 = v2;
            Ok(nat_lt(a2327, a2328))
        } else {
            let a2333 = {
                let a2331 = {
                    let a2329 = v1;
                    let a2330 = v5;
                    nat_add(a2329, a2330)?
                };
                let a2332 = v6;
                nat_sub(a2331, a2332)
            };
            let a2334 = v2;
            Ok(nat_le(a2333, a2334))
        }
    } else {
        Ok(false)
    }
}

pub fn f92(v0: u64, v1: u64) -> bool {
    let a2335 = v0;
    let a2336 = 0u64;
    if if nat_eq(a2335, a2336) {
        true
    } else {
        let a2337 = v0;
        let a2338 = 1u64;
        nat_eq(a2337, a2338)
    } {
        let a2339 = v1;
        let a2340 = 4294967296u64;
        nat_lt(a2339, a2340)
    } else {
        false
    }
}

pub fn f93(v0: u64, v1: u64) -> R<u64> {
    let a2345 = {
        let a2343 = v0;
        let a2344 = {
            let a2341 = v1;
            let a2342 = 1u64;
            nat_sub(a2341, a2342)
        };
        nat_add(a2343, a2344)?
    };
    let a2346 = v1;
    let a2347 = 0u64;
    Ok(nat_quot(a2345, a2346, a2347))
}

pub fn f94(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f27(match v0.clone() {
        Adt2::C0(_, h2348, _, _, _, _, _, _, _, _, _) => {
            h2348
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2349, _, _, _, _, _, _, _, _) => {
            h2349
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2350, _, _, _, _, _, _, _) => {
            h2350
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2351, _, _, _, _, _, _) => {
            h2351
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2354 = match v1.clone() {
                Adt2::C0(_, h2352, _, _, _, _, _, _, _, _, _) => {
                    h2352
                }
            };
            let a2355 = match v1.clone() {
                Adt2::C0(_, _, h2353, _, _, _, _, _, _, _, _) => {
                    h2353
                }
            };
            nat_mul(a2354, a2355)?
        };
        let v5: u64 = {
            let a2357 = v4;
            let a2358 = match v1.clone() {
                Adt2::C0(_, _, _, h2356, _, _, _, _, _, _, _) => {
                    h2356
                }
            };
            nat_mul(a2357, a2358)?
        };
        let a2371 = {
            let a2369 = {
                let a2360 = match v0.clone() {
                    Adt2::C0(_, h2359, _, _, _, _, _, _, _, _, _) => {
                        h2359
                    }
                };
                let a2361 = 1u64;
                nat_sub(a2360, a2361)
            };
            let a2370 = {
                let a2367 = f137(match v0.clone() {
                    Adt2::C0(_, _, h2362, _, _, _, _, _, _, _, _) => {
                        h2362
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2363, _, _, _, _, _, _, _, _, _) => {
                        h2363
                    }
                })?;
                let a2368 = f137(match v0 {
                    Adt2::C0(_, _, _, h2364, _, _, _, _, _, _, _) => {
                        h2364
                    }
                }, v2, {
                    let a2365 = v3;
                    let a2366 = v5;
                    nat_mul(a2365, a2366)?
                }, v4)?;
                nat_add(a2367, a2368)?
            };
            nat_add(a2369, a2370)?
        };
        let a2372 = f12(v1, 1u64)?;
        Ok(nat_lt(a2371, a2372))
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2373 = v0;
    let a2374 = 2147483648u64;
    if if if if nat_lt(a2373, a2374) {
        let a2375 = v1;
        let a2376 = 2147483648u64;
        nat_lt(a2375, a2376)
    } else {
        false
    } {
        let a2379 = {
            let a2377 = v0;
            let a2378 = v1;
            nat_mul(a2377, a2378)?
        };
        let a2380 = 2147483648u64;
        nat_lt(a2379, a2380)
    } else {
        false
    } {
        let a2385 = {
            let a2383 = {
                let a2381 = v0;
                let a2382 = 1u64;
                nat_sub(a2381, a2382)
            };
            let a2384 = v2;
            nat_mul(a2383, a2384)?
        };
        let a2386 = 2147483648u64;
        nat_lt(a2385, a2386)
    } else {
        false
    } {
        let a2391 = {
            let a2389 = {
                let a2387 = v1;
                let a2388 = 1u64;
                nat_sub(a2387, a2388)
            };
            let a2390 = v2;
            nat_mul(a2389, a2390)?
        };
        let a2392 = 2147483648u64;
        Ok(nat_lt(a2391, a2392))
    } else {
        Ok(false)
    }
}

pub fn f96(v0: u64) -> R<u64> {
    let a2395 = 3u64;
    let a2396 = {
        let a2393 = v0;
        let a2394 = 3u64;
        nat_mul(a2393, a2394)?
    };
    nat_add(a2395, a2396)
}

pub fn f97(v0: Adt3) -> R<u64> {
    if f27(match v0.clone() {
        Adt3::C0(h2397, _, _, _, _, _, _, _) => {
            h2397
        }
    }, match v0.clone() {
        Adt3::C0(_, h2398, _, _, _, _, _, _) => {
            h2398
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h2399, _, _, _, _, _) => {
            h2399
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h2400, _, _, _, _) => {
            h2400
        }
    }) {
        Ok(0u64)
    } else {
        let a2409 = match v0.clone() {
            Adt3::C0(h2401, _, _, _, _, _, _, _) => {
                h2401
            }
        };
        let a2410 = {
            let a2407 = match v0.clone() {
                Adt3::C0(_, h2402, _, _, _, _, _, _) => {
                    h2402
                }
            };
            let a2408 = {
                let a2405 = match v0.clone() {
                    Adt3::C0(_, _, h2403, _, _, _, _, _) => {
                        h2403
                    }
                };
                let a2406 = match v0 {
                    Adt3::C0(_, _, _, h2404, _, _, _, _) => {
                        h2404
                    }
                };
                nat_mul(a2405, a2406)?
            };
            nat_mul(a2407, a2408)?
        };
        nat_mul(a2409, a2410)
    }
}

pub fn f98(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2411 = v1;
    let a2412 = 13u64;
    if if nat_eq(a2411, a2412) {
        f27(match v0.clone() {
            Adt3::C0(h2413, _, _, _, _, _, _, _) => {
                h2413
            }
        }, match v0.clone() {
            Adt3::C0(_, h2414, _, _, _, _, _, _) => {
                h2414
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2415, _, _, _, _, _) => {
                h2415
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2416, _, _, _, _) => {
                h2416
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2422 = match v0.clone() {
                Adt3::C0(_, h2417, _, _, _, _, _, _) => {
                    h2417
                }
            };
            let a2423 = {
                let a2420 = match v0.clone() {
                    Adt3::C0(_, _, h2418, _, _, _, _, _) => {
                        h2418
                    }
                };
                let a2421 = match v0.clone() {
                    Adt3::C0(_, _, _, h2419, _, _, _, _) => {
                        h2419
                    }
                };
                nat_mul(a2420, a2421)?
            };
            nat_mul(a2422, a2423)?
        };
        let a2424 = v1;
        let a2425 = 13u64;
        if nat_eq(a2424, a2425) {
            let a2434 = 2u64;
            let a2435 = if f138(v0.clone(), v2) {
                let a2427 = v3;
                let a2428 = f139(match v0 {
                    Adt3::C0(h2426, _, _, _, _, _, _, _) => {
                        h2426
                    }
                }, false)?;
                nat_mul(a2427, a2428)?
            } else {
                let a2432 = v3;
                let a2433 = {
                    let a2430 = match v0 {
                        Adt3::C0(h2429, _, _, _, _, _, _, _) => {
                            h2429
                        }
                    };
                    let a2431 = 2u64;
                    nat_mul(a2430, a2431)?
                };
                nat_mul(a2432, a2433)?
            };
            nat_add(a2434, a2435)
        } else {
            let a2439 = v3;
            let a2440 = f139(match v0 {
                Adt3::C0(h2436, _, _, _, _, _, _, _) => {
                    h2436
                }
            }, {
                let a2437 = v1;
                let a2438 = 16u64;
                nat_eq(a2437, a2438)
            })?;
            nat_mul(a2439, a2440)
        }
    }
}

pub fn f99(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a2441 = v0;
    let a2442 = 30u64;
    if if nat_eq(a2441, a2442) {
        let a2445 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2443, _, _, _, _) => {
                h2443
            }
        };
        let a2446 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2444, _, _, _, _, _) => {
                h2444
            }
        };
        nat_lt(a2445, a2446)
    } else {
        false
    } {
        Ok(Adt9::C1)
    } else {
        let m2447 = v3;
        match m2447 {
            None => {
                f140(v0, v1, match v2.clone() {
                    Adt2::C0(h2448, _, _, _, _, _, _, _, _, _, _) => {
                        h2448
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2449, _, _, _, _, _, _, _, _, _, _) => {
                        h2449
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2450, _, _, _, _, _, _, _, _, _) => {
                        h2450
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2451, _, _, _, _, _, _, _, _) => {
                        h2451
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2452, _, _, _, _, _, _, _) => {
                        h2452
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2453, _, _, _, _, _, _) => {
                        h2453
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2454, _, _, _, _, _, _, _, _, _) => {
                        h2454
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2455, _, _, _, _, _, _, _, _) => {
                        h2455
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2456, _, _, _, _, _, _, _) => {
                        h2456
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2457, _, _, _, _, _, _) => {
                        h2457
                    }
                })
            }
            Some(v5) => {
                f140(v0, v1, match v2.clone() {
                    Adt2::C0(h2458, _, _, _, _, _, _, _, _, _, _) => {
                        h2458
                    }
                }, match v5.clone() {
                    Adt2::C0(h2459, _, _, _, _, _, _, _, _, _, _) => {
                        h2459
                    }
                }, match v4.clone() {
                    Adt2::C0(h2460, _, _, _, _, _, _, _, _, _, _) => {
                        h2460
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h2461, _, _, _, _, _, _, _, _, _) => {
                        h2461
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2462, _, _, _, _, _, _, _, _) => {
                        h2462
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2463, _, _, _, _, _, _, _) => {
                        h2463
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2464, _, _, _, _, _, _) => {
                        h2464
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h2465, _, _, _, _, _, _, _, _, _) => {
                        h2465
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h2466, _, _, _, _, _, _, _, _) => {
                        h2466
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h2467, _, _, _, _, _, _, _) => {
                        h2467
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h2468, _, _, _, _, _, _) => {
                        h2468
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2469, _, _, _, _, _, _, _, _, _) => {
                        h2469
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2470, _, _, _, _, _, _, _, _) => {
                        h2470
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2471, _, _, _, _, _, _, _) => {
                        h2471
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2472, _, _, _, _, _, _) => {
                        h2472
                    }
                })
            }
        }
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2485 = {
        let a2481 = {
            let a2477 = {
                let a2473 = v0;
                let a2474 = v4;
                nat_mul(a2473, a2474)?
            };
            let a2478 = {
                let a2475 = v1;
                let a2476 = v5;
                nat_mul(a2475, a2476)?
            };
            nat_add(a2477, a2478)?
        };
        let a2482 = {
            let a2479 = v2;
            let a2480 = v6;
            nat_mul(a2479, a2480)?
        };
        nat_add(a2481, a2482)?
    };
    let a2486 = {
        let a2483 = v3;
        let a2484 = v7;
        nat_mul(a2483, a2484)?
    };
    nat_add(a2485, a2486)
}

pub fn f101(v0: Adt10, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt8 {
    let v9: u64 = {
        let a2487 = v7;
        let a2488 = 6u64;
        if nat_eq(a2487, a2488) {
            27u64
        } else {
            v2
        }
    };
    Adt8::C7(v0, f70(v3, v4), f70(v5, v6), f71(v7, v8), f141(v1), f141(v9))
}

pub fn f102(v0: Adt4, v1: u64, v2: u64) -> Adt10 {
    let m2489 = v0;
    match m2489 {
        Adt4::C0 => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C1 => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C2(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C3(_, _, _, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C4(_) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C5(_, _, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C6(_, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C7(_) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C8(_) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C9(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C10(_, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C11(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C12(_, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C13(v30, v31, v32, v33) => {
            Adt10::C4(v30, v31, v32, v33)
        }
        Adt4::C14(v34, _, _) => {
            let a2490 = 13u64;
            let a2491 = v34;
            if nat_le(a2490, a2491) {
                Adt10::C7(v34, v1, v2)
            } else {
                Adt10::C3(v1, v2, true)
            }
        }
        Adt4::C15 => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C16(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C17 => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C18(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C19(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C20(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C21 => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C22(_, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C23(_, _, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C24(_) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C25(_, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C26(_, _, _, _) => {
            Adt10::C2(v1, v2, true)
        }
        Adt4::C27(v50) => {
            Adt10::C2(match v50.clone() {
                Adt5::C0(_, _, _, _, _, h2492, _, _) => {
                    h2492
                }
            }, match v50 {
                Adt5::C0(_, _, _, _, _, _, _, h2493) => {
                    h2493
                }
            }, true)
        }
    }
}

pub fn f103(v0: Adt4) -> u64 {
    let m2494 = v0;
    match m2494 {
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

pub fn f104(v0: Adt4, v1: u64) -> u64 {
    let m2495 = v0;
    match m2495 {
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
            let a2496 = v1;
            let a2497 = 6u64;
            if nat_eq(a2496, a2497) {
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
            let a2499 = v1;
            let a2500 = 6u64;
            if if if nat_eq(a2499, a2500) {
                true
            } else {
                let a2501 = v1;
                let a2502 = 7u64;
                nat_eq(a2501, a2502)
            } {
                let a2503 = v50;
                let a2504 = 30u64;
                nat_eq(a2503, a2504)
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
                Adt5::C0(_, _, _, _, _, _, _, h2498) => {
                    h2498
                }
            }
        }
    }
}

pub fn f105(v0: Adt4, v1: u64) -> Adt10 {
    let a2505 = v1;
    let a2506 = 4u64;
    if nat_lt(a2505, a2506) {
        Adt10::C0(v1)
    } else {
        let a2507 = v1;
        let a2508 = 4u64;
        if nat_eq(a2507, a2508) {
            Adt10::C1(0u64)
        } else {
            let a2509 = v1;
            let a2510 = 5u64;
            if nat_eq(a2509, a2510) {
                Adt10::C1(3u64)
            } else {
                let a2511 = v1;
                let a2512 = 6u64;
                if nat_eq(a2511, a2512) {
                    Adt10::C2(f104(v0, v1), 0u64, false)
                } else {
                    let a2513 = v1;
                    let a2514 = 7u64;
                    if nat_eq(a2513, a2514) {
                        Adt10::C2(0u64, f104(v0, v1), false)
                    } else {
                        let a2515 = v1;
                        let a2516 = 16u64;
                        if nat_lt(a2515, a2516) {
                            Adt10::C1({
                                let a2517 = v1;
                                let a2518 = 8u64;
                                nat_sub(a2517, a2518)
                            })
                        } else {
                            let a2519 = v1;
                            let a2520 = 20u64;
                            if nat_lt(a2519, a2520) {
                                Adt10::C5({
                                    let a2521 = v1;
                                    let a2522 = 16u64;
                                    nat_sub(a2521, a2522)
                                })
                            } else {
                                let a2523 = v1;
                                let a2524 = 20u64;
                                if nat_eq(a2523, a2524) {
                                    Adt10::C3(0u64, 0u64, false)
                                } else {
                                    let a2525 = v1;
                                    let a2526 = 21u64;
                                    if nat_eq(a2525, a2526) {
                                        Adt10::C0(4u64)
                                    } else {
                                        let a2527 = v1;
                                        let a2528 = 22u64;
                                        if nat_eq(a2527, a2528) {
                                            Adt10::C0(5u64)
                                        } else {
                                            Adt10::C6
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f106(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2530 = v0;
        let a2531 = match v1.clone() {
            Adt3::C0(h2529, _, _, _, _, _, _, _) => {
                h2529
            }
        };
        let a2532 = 0u64;
        nat_quot(a2530, a2531, a2532)
    };
    let v3: u64 = {
        let a2534 = v2;
        let a2535 = match v1.clone() {
            Adt3::C0(_, h2533, _, _, _, _, _, _) => {
                h2533
            }
        };
        let a2536 = 0u64;
        nat_quot(a2534, a2535, a2536)
    };
    f100({
        let a2538 = v0;
        let a2539 = match v1.clone() {
            Adt3::C0(h2537, _, _, _, _, _, _, _) => {
                h2537
            }
        };
        let a2540 = 0u64;
        nat_rem(a2538, a2539, a2540)
    }, {
        let a2542 = v2;
        let a2543 = match v1.clone() {
            Adt3::C0(_, h2541, _, _, _, _, _, _) => {
                h2541
            }
        };
        let a2544 = 0u64;
        nat_rem(a2542, a2543, a2544)
    }, {
        let a2546 = v3;
        let a2547 = match v1.clone() {
            Adt3::C0(_, _, h2545, _, _, _, _, _) => {
                h2545
            }
        };
        let a2548 = 0u64;
        nat_rem(a2546, a2547, a2548)
    }, {
        let a2550 = v3;
        let a2551 = match v1.clone() {
            Adt3::C0(_, _, h2549, _, _, _, _, _) => {
                h2549
            }
        };
        let a2552 = 0u64;
        nat_quot(a2550, a2551, a2552)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2553, _, _, _) => {
            h2553
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2554, _, _) => {
            h2554
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2555, _) => {
            h2555
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2556) => {
            h2556
        }
    })
}

pub fn f107(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2574 = {
        let a2569 = f142(v2, {
            let a2560 = {
                let a2557 = v0;
                let a2558 = v1;
                let a2559 = 0u64;
                nat_quot(a2557, a2558, a2559)
            };
            let a2561 = v2;
            let a2562 = 0u64;
            nat_rem(a2560, a2561, a2562)
        }, {
            let a2566 = {
                let a2563 = v0;
                let a2564 = v1;
                let a2565 = 0u64;
                nat_quot(a2563, a2564, a2565)
            };
            let a2567 = v2;
            let a2568 = 0u64;
            nat_quot(a2566, a2567, a2568)
        })?;
        let a2570 = v1;
        nat_mul(a2569, a2570)?
    };
    let a2575 = {
        let a2571 = v0;
        let a2572 = v1;
        let a2573 = 0u64;
        nat_rem(a2571, a2572, a2573)
    };
    nat_add(a2574, a2575)
}

pub fn f108(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2577 = v0;
        let a2578 = match v2.clone() {
            Adt3::C0(h2576, _, _, _, _, _, _, _) => {
                h2576
            }
        };
        let a2579 = 0u64;
        nat_quot(a2577, a2578, a2579)
    };
    let v4: u64 = {
        let a2581 = v3;
        let a2582 = match v2.clone() {
            Adt3::C0(_, h2580, _, _, _, _, _, _) => {
                h2580
            }
        };
        let a2583 = 0u64;
        nat_quot(a2581, a2582, a2583)
    };
    f143({
        let a2585 = v0;
        let a2586 = match v2.clone() {
            Adt3::C0(h2584, _, _, _, _, _, _, _) => {
                h2584
            }
        };
        let a2587 = 0u64;
        nat_rem(a2585, a2586, a2587)
    }, {
        let a2589 = v3;
        let a2590 = match v2.clone() {
            Adt3::C0(_, h2588, _, _, _, _, _, _) => {
                h2588
            }
        };
        let a2591 = 0u64;
        nat_rem(a2589, a2590, a2591)
    }, {
        let a2593 = v4;
        let a2594 = match v2.clone() {
            Adt3::C0(_, _, h2592, _, _, _, _, _) => {
                h2592
            }
        };
        let a2595 = 0u64;
        nat_rem(a2593, a2594, a2595)
    }, {
        let a2597 = v4;
        let a2598 = match v2 {
            Adt3::C0(_, _, h2596, _, _, _, _, _) => {
                h2596
            }
        };
        let a2599 = 0u64;
        nat_quot(a2597, a2598, a2599)
    }, match v1.clone() {
        Adt3::C0(h2600, _, _, _, _, _, _, _) => {
            h2600
        }
    }, match v1.clone() {
        Adt3::C0(_, h2601, _, _, _, _, _, _) => {
            h2601
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2602, _, _, _, _, _) => {
            h2602
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2603, _, _, _, _) => {
            h2603
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2604, _, _, _) => {
            h2604
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2605, _, _) => {
            h2605
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2606, _) => {
            h2606
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2607) => {
            h2607
        }
    })
}

pub fn f109(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt8> {
    let v5: u64 = {
        let a2609 = v0;
        let a2610 = match v3.clone() {
            Adt3::C0(h2608, _, _, _, _, _, _, _) => {
                h2608
            }
        };
        let a2611 = 0u64;
        nat_quot(a2609, a2610, a2611)
    };
    let v6: u64 = {
        let a2613 = v5;
        let a2614 = match v3.clone() {
            Adt3::C0(_, h2612, _, _, _, _, _, _) => {
                h2612
            }
        };
        let a2615 = 0u64;
        nat_quot(a2613, a2614, a2615)
    };
    let v7: u64 = {
        let a2617 = v0;
        let a2618 = match v3.clone() {
            Adt3::C0(h2616, _, _, _, _, _, _, _) => {
                h2616
            }
        };
        let a2619 = 0u64;
        nat_rem(a2617, a2618, a2619)
    };
    let v8: u64 = {
        let a2621 = v5;
        let a2622 = match v3.clone() {
            Adt3::C0(_, h2620, _, _, _, _, _, _) => {
                h2620
            }
        };
        let a2623 = 0u64;
        nat_rem(a2621, a2622, a2623)
    };
    let v9: u64 = {
        let a2625 = v6;
        let a2626 = match v3.clone() {
            Adt3::C0(_, _, h2624, _, _, _, _, _) => {
                h2624
            }
        };
        let a2627 = 0u64;
        nat_rem(a2625, a2626, a2627)
    };
    let v10: u64 = {
        let a2629 = v6;
        let a2630 = match v3.clone() {
            Adt3::C0(_, _, h2628, _, _, _, _, _) => {
                h2628
            }
        };
        let a2631 = 0u64;
        nat_quot(a2629, a2630, a2631)
    };
    let v11: u64 = {
        let a2632 = v4;
        let a2633 = 0u64;
        if nat_eq(a2632, a2633) {
            match v1.clone() {
                Adt3::C0(h2634, _, _, _, _, _, _, _) => {
                    h2634
                }
            }
        } else {
            let a2635 = v4;
            let a2636 = 1u64;
            if nat_eq(a2635, a2636) {
                match v1.clone() {
                    Adt3::C0(_, h2637, _, _, _, _, _, _) => {
                        h2637
                    }
                }
            } else {
                let a2638 = v4;
                let a2639 = 2u64;
                if nat_eq(a2638, a2639) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2640, _, _, _, _, _) => {
                            h2640
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2641, _, _, _, _) => {
                            h2641
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2648 = v11;
        let a2649 = {
            let a2642 = v4;
            let a2643 = 0u64;
            if nat_eq(a2642, a2643) {
                v7
            } else {
                let a2644 = v4;
                let a2645 = 1u64;
                if nat_eq(a2644, a2645) {
                    v8
                } else {
                    let a2646 = v4;
                    let a2647 = 2u64;
                    if nat_eq(a2646, a2647) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2648, a2649)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt8::C1(v12, f100(if if v12 {
        let a2650 = v4;
        let a2651 = 0u64;
        nat_eq(a2650, a2651)
    } else {
        false
    } {
        let a2652 = v7;
        let a2653 = v11;
        nat_sub(a2652, a2653)
    } else {
        v7
    }, if if v12 {
        let a2654 = v4;
        let a2655 = 1u64;
        nat_eq(a2654, a2655)
    } else {
        false
    } {
        let a2656 = v8;
        let a2657 = v11;
        nat_sub(a2656, a2657)
    } else {
        v8
    }, if if v12 {
        let a2658 = v4;
        let a2659 = 2u64;
        nat_eq(a2658, a2659)
    } else {
        false
    } {
        let a2660 = v9;
        let a2661 = v11;
        nat_sub(a2660, a2661)
    } else {
        v9
    }, if if v12 {
        let a2662 = v4;
        let a2663 = 3u64;
        nat_eq(a2662, a2663)
    } else {
        false
    } {
        let a2664 = v10;
        let a2665 = v11;
        nat_sub(a2664, a2665)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2666, _, _, _) => {
            h2666
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2667, _, _) => {
            h2667
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2668, _) => {
            h2668
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2669) => {
            h2669
        }
    })?, f106(v0, v3)?))
}

pub fn f110(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt8> {
    let v8: u64 = f58(v0, v2.clone(), 0u64);
    let v9: u64 = f58(v0, v2.clone(), 1u64);
    let v10: u64 = f58(v0, v2.clone(), 2u64);
    let v11: u64 = f58(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt8::C1(false, f57(v1.clone(), f144(v8, v3, match v1.clone() {
            Adt3::C0(h2670, _, _, _, _, _, _, _) => {
                h2670
            }
        })?, f144(v9, v4, match v1.clone() {
            Adt3::C0(_, h2671, _, _, _, _, _, _) => {
                h2671
            }
        })?, f144(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2672, _, _, _, _, _) => {
                h2672
            }
        })?, f144(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2673, _, _, _, _) => {
                h2673
            }
        })?)?, f106(v0, v2)?))
    } else {
        if if if if f145(v8, v3, match v1.clone() {
            Adt3::C0(h2674, _, _, _, _, _, _, _) => {
                h2674
            }
        })? {
            f145(v9, v4, match v1.clone() {
                Adt3::C0(_, h2675, _, _, _, _, _, _) => {
                    h2675
                }
            })?
        } else {
            false
        } {
            f145(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2676, _, _, _, _, _) => {
                    h2676
                }
            })?
        } else {
            false
        } {
            f145(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2677, _, _, _, _) => {
                    h2677
                }
            })?
        } else {
            false
        } {
            Ok(Adt8::C1(false, f57(v1, f146(v8, v3)?, f146(v9, v4)?, f146(v10, v5)?, f146(v11, v6)?)?, f106(v0, v2)?))
        } else {
            Ok(Adt8::C1(true, 0u64, f106(v0, v2)?))
        }
    }
}

pub fn f111(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt8> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 1u64);
    let v6: u64 = f58(v0, v2.clone(), 2u64);
    let v7: u64 = f58(v0, v2.clone(), 3u64);
    Ok(Adt8::C1(false, f57(v1.clone(), f147(v4, v3, match v1 {
        Adt3::C0(h2678, _, _, _, _, _, _, _) => {
            h2678
        }
    })?, v5, v6, v7)?, f106(v0, v2)?))
}

pub fn f112(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt8> {
    let a2679 = v0;
    let a2680 = v3;
    if nat_lt(a2679, a2680) {
        Ok(Adt8::C1(false, {
            let a2681 = v0;
            let a2682 = 4u64;
            nat_mul(a2681, a2682)?
        }, {
            let a2683 = v0;
            let a2684 = 4u64;
            nat_mul(a2683, a2684)?
        }))
    } else {
        let v6: u64 = {
            let a2685 = v0;
            let a2686 = v3;
            nat_sub(a2685, a2686)
        };
        let v7: u64 = f106(v6, v1)?;
        let v8: u64 = f148(v6, v2, v4)?;
        if v5 {
            Ok(Adt8::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt8::C1(true, v7, v8))
        }
    }
}

pub fn f113(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt8> {
    let a2687 = v0;
    let a2688 = v3;
    if nat_lt(a2687, a2688) {
        Ok(Adt8::C1(true, 0u64, f106(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2689 = v0;
            let a2690 = v3;
            nat_sub(a2689, a2690)
        };
        let v5: u64 = f108(v4, v2, v1.clone())?;
        Ok(Adt8::C4(8u64, 3u64, v5, 0u64, f106(v4, v1)?, 3u64, v5))
    }
}

pub fn f114(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt8> {
    let v4: u64 = {
        let a2692 = v1;
        let a2693 = match v3.clone() {
            Adt3::C0(h2691, _, _, _, _, _, _, _) => {
                h2691
            }
        };
        let a2694 = 0u64;
        nat_rem(a2692, a2693, a2694)
    };
    let v5: u64 = {
        let a2700 = {
            let a2696 = v1;
            let a2697 = match v3.clone() {
                Adt3::C0(h2695, _, _, _, _, _, _, _) => {
                    h2695
                }
            };
            let a2698 = 0u64;
            nat_quot(a2696, a2697, a2698)
        };
        let a2701 = match v3.clone() {
            Adt3::C0(_, h2699, _, _, _, _, _, _) => {
                h2699
            }
        };
        let a2702 = 0u64;
        nat_rem(a2700, a2701, a2702)
    };
    let v6: bool = f149(v0, v4, v5);
    Ok(Adt8::C1(!v6, if v6 {
        f108(v1, v2, v3.clone())?
    } else {
        0u64
    }, f106(v1, v3)?))
}

pub fn f115(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt8> {
    Ok(Adt8::C4({
        let a2703 = v3;
        let a2704 = 8u64;
        nat_add(a2703, a2704)?
    }, 0u64, f106(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f108(v0, v5, v2.clone())?
    }, 3u64, f106(v0, v2)?))
}

pub fn f116(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt8> {
    let v6: u64 = {
        let a2706 = v0;
        let a2707 = match v2.clone() {
            Adt3::C0(h2705, _, _, _, _, _, _, _) => {
                h2705
            }
        };
        let a2708 = 0u64;
        nat_rem(a2706, a2707, a2708)
    };
    let v7: u64 = {
        let a2710 = v0;
        let a2711 = match v2.clone() {
            Adt3::C0(h2709, _, _, _, _, _, _, _) => {
                h2709
            }
        };
        let a2712 = 0u64;
        nat_quot(a2710, a2711, a2712)
    };
    let v8: u64 = {
        let a2714 = v7;
        let a2715 = match v2.clone() {
            Adt3::C0(_, h2713, _, _, _, _, _, _) => {
                h2713
            }
        };
        let a2716 = 0u64;
        nat_rem(a2714, a2715, a2716)
    };
    let v9: u64 = {
        let a2718 = v7;
        let a2719 = match v2.clone() {
            Adt3::C0(_, h2717, _, _, _, _, _, _) => {
                h2717
            }
        };
        let a2720 = 0u64;
        nat_quot(a2718, a2719, a2720)
    };
    let v10: u64 = {
        let a2722 = v9;
        let a2723 = match v2.clone() {
            Adt3::C0(_, _, h2721, _, _, _, _, _) => {
                h2721
            }
        };
        let a2724 = 0u64;
        nat_rem(a2722, a2723, a2724)
    };
    let v11: u64 = {
        let a2726 = v9;
        let a2727 = match v2.clone() {
            Adt3::C0(_, _, h2725, _, _, _, _, _) => {
                h2725
            }
        };
        let a2728 = 0u64;
        nat_quot(a2726, a2727, a2728)
    };
    let v12: u64 = f106(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2737 = {
                let a2732 = {
                    let a2729 = v10;
                    let a2730 = v4;
                    let a2731 = 0u64;
                    nat_quot(a2729, a2730, a2731)
                };
                let a2733 = v5;
                nat_mul(a2732, a2733)?
            };
            let a2738 = {
                let a2734 = v8;
                let a2735 = v4;
                let a2736 = 0u64;
                nat_quot(a2734, a2735, a2736)
            };
            nat_add(a2737, a2738)?
        };
        let v14: u64 = {
            let a2758 = v6;
            let a2759 = {
                let a2756 = match v1.clone() {
                    Adt3::C0(h2739, _, _, _, _, _, _, _) => {
                        h2739
                    }
                };
                let a2757 = {
                    let a2754 = {
                        let a2740 = v8;
                        let a2741 = v4;
                        let a2742 = 0u64;
                        nat_rem(a2740, a2741, a2742)
                    };
                    let a2755 = {
                        let a2752 = match v1.clone() {
                            Adt3::C0(_, h2743, _, _, _, _, _, _) => {
                                h2743
                            }
                        };
                        let a2753 = {
                            let a2750 = {
                                let a2744 = v10;
                                let a2745 = v4;
                                let a2746 = 0u64;
                                nat_rem(a2744, a2745, a2746)
                            };
                            let a2751 = {
                                let a2748 = match v1.clone() {
                                    Adt3::C0(_, _, h2747, _, _, _, _, _) => {
                                        h2747
                                    }
                                };
                                let a2749 = v13;
                                nat_mul(a2748, a2749)?
                            };
                            nat_add(a2750, a2751)?
                        };
                        nat_mul(a2752, a2753)?
                    };
                    nat_add(a2754, a2755)?
                };
                nat_mul(a2756, a2757)?
            };
            nat_add(a2758, a2759)?
        };
        Ok(Adt8::C1(false, f106(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2765 = {
                let a2763 = {
                    let a2760 = v11;
                    let a2761 = v5;
                    let a2762 = 0u64;
                    nat_rem(a2760, a2761, a2762)
                };
                let a2764 = v4;
                nat_mul(a2763, a2764)?
            };
            let a2766 = v8;
            nat_add(a2765, a2766)?
        };
        let v16: u64 = {
            let a2772 = {
                let a2770 = {
                    let a2767 = v11;
                    let a2768 = v5;
                    let a2769 = 0u64;
                    nat_quot(a2767, a2768, a2769)
                };
                let a2771 = v4;
                nat_mul(a2770, a2771)?
            };
            let a2773 = v10;
            nat_add(a2772, a2773)?
        };
        let a2775 = v15;
        let a2776 = match v1.clone() {
            Adt3::C0(_, h2774, _, _, _, _, _, _) => {
                h2774
            }
        };
        if if nat_lt(a2775, a2776) {
            let a2778 = v16;
            let a2779 = match v1.clone() {
                Adt3::C0(_, _, h2777, _, _, _, _, _) => {
                    h2777
                }
            };
            nat_lt(a2778, a2779)
        } else {
            false
        } {
            Ok(Adt8::C1(false, f106({
                let a2788 = v6;
                let a2789 = {
                    let a2786 = match v1.clone() {
                        Adt3::C0(h2780, _, _, _, _, _, _, _) => {
                            h2780
                        }
                    };
                    let a2787 = {
                        let a2784 = v15;
                        let a2785 = {
                            let a2782 = match v1.clone() {
                                Adt3::C0(_, h2781, _, _, _, _, _, _) => {
                                    h2781
                                }
                            };
                            let a2783 = v16;
                            nat_mul(a2782, a2783)?
                        };
                        nat_add(a2784, a2785)?
                    };
                    nat_mul(a2786, a2787)?
                };
                nat_add(a2788, a2789)?
            }, v1)?, v12))
        } else {
            Ok(Adt8::C1(true, 0u64, v12))
        }
    }
}

pub fn f117(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt8> {
    let v3: u64 = f106(v0, v2.clone())?;
    let v4: u64 = f106(v0, v1.clone())?;
    let a2794 = {
        let a2791 = v0;
        let a2792 = match v1 {
            Adt3::C0(h2790, _, _, _, _, _, _, _) => {
                h2790
            }
        };
        let a2793 = 0u64;
        nat_rem(a2791, a2792, a2793)
    };
    let a2795 = 0u64;
    if nat_eq(a2794, a2795) {
        Ok(Adt8::C1(false, v4, v3))
    } else {
        Ok(Adt8::C4(8u64, 3u64, f106({
            let a2796 = v0;
            let a2797 = 1u64;
            nat_sub(a2796, a2797)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f118(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt8> {
    let a2798 = v3;
    let a2799 = 13u64;
    if nat_eq(a2798, a2799) {
        f150(v0, v1, v4)
    } else {
        let v5: u64 = f139(match v1.clone() {
            Adt3::C0(h2800, _, _, _, _, _, _, _) => {
                h2800
            }
        }, {
            let a2801 = v3;
            let a2802 = 16u64;
            nat_eq(a2801, a2802)
        })?;
        let v6: u64 = {
            let a2803 = v0;
            let a2804 = v5;
            let a2805 = 0u64;
            nat_quot(a2803, a2804, a2805)
        };
        let v7: u64 = {
            let a2806 = v0;
            let a2807 = v5;
            let a2808 = 0u64;
            nat_rem(a2806, a2807, a2808)
        };
        let v8: u64 = {
            let a2812 = 1u64;
            let a2813 = {
                let a2810 = match v1.clone() {
                    Adt3::C0(h2809, _, _, _, _, _, _, _) => {
                        h2809
                    }
                };
                let a2811 = 2u64;
                nat_mul(a2810, a2811)?
            };
            nat_add(a2812, a2813)?
        };
        let a2814 = v7;
        let a2815 = v8;
        if nat_lt(a2814, a2815) {
            f151(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f106(v6, v2)?;
            let a2816 = v3;
            let a2817 = 14u64;
            if nat_eq(a2816, a2817) {
                Ok(Adt8::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2818 = v7;
                let a2819 = v8;
                if nat_eq(a2818, a2819) {
                    Ok(Adt8::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2822 = v7;
                    let a2823 = {
                        let a2820 = v8;
                        let a2821 = 1u64;
                        nat_add(a2820, a2821)?
                    };
                    if nat_eq(a2822, a2823) {
                        Ok(Adt8::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2824, _, _, _, _, _, _, _) => {
                                h2824
                            }
                        }, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(Adt8::C4(5u64, 1u64, 1u64, 1u64, 2u64, 3u64, v9))
                    }
                }
            }
        }
    }
}

pub fn f119(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt8> {
    let a2825 = v0;
    let a2826 = 0u64;
    if nat_eq(a2825, a2826) {
        Ok(Adt8::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2833 = {
            let a2827 = v0;
            let a2828 = 1u64;
            nat_add(a2827, a2828)?
        };
        let a2834 = {
            let a2831 = 2u64;
            let a2832 = {
                let a2829 = v3;
                let a2830 = 2u64;
                nat_mul(a2829, a2830)?
            };
            nat_add(a2831, a2832)?
        };
        if nat_eq(a2833, a2834) {
            Ok(Adt8::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2835 = v0;
                let a2836 = 1u64;
                nat_sub(a2835, a2836)
            };
            let v5: u64 = {
                let a2837 = v4;
                let a2838 = 2u64;
                let a2839 = 0u64;
                nat_quot(a2837, a2838, a2839)
            };
            let a2843 = {
                let a2840 = v4;
                let a2841 = 2u64;
                let a2842 = 0u64;
                nat_rem(a2840, a2841, a2842)
            };
            let a2844 = 0u64;
            if nat_eq(a2843, a2844) {
                Ok(Adt8::C4(17u64, 0u64, f106(v5, v1)?, 4u64, f106(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt8::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f120(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt8> {
    let a2845 = v0;
    let a2846 = f22(v1, v2, v3)?;
    if nat_lt(a2845, a2846) {
        let a2847 = v0;
        let a2848 = v1;
        if nat_lt(a2847, a2848) {
            Ok(Adt8::C1(false, {
                let a2849 = 4u64;
                let a2850 = v0;
                nat_mul(a2849, a2850)?
            }, {
                let a2851 = 4u64;
                let a2852 = v0;
                nat_mul(a2851, a2852)?
            }))
        } else {
            let v4: u64 = {
                let a2853 = v0;
                let a2854 = v1;
                nat_sub(a2853, a2854)
            };
            let v5: u64 = f152(v4, v2)?;
            let a2855 = v5;
            let a2856 = 2u64;
            if nat_lt(a2855, a2856) {
                Ok(Adt8::C4(6u64, {
                    let a2857 = v5;
                    let a2858 = 0u64;
                    if nat_eq(a2857, a2858) {
                        4u64
                    } else {
                        5u64
                    }
                }, f153(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f154(v4, v2)?;
                Ok(Adt8::C4(8u64, 3u64, v6, 1u64, if f155(v4, v2)? {
                    1u64
                } else {
                    0u64
                }, 3u64, v6))
            }
        }
    } else {
        Ok(Adt8::C0(Adt9::C1))
    }
}

pub fn f121(v0: Adt3, v1: Adt3) -> R<u64> {
    let a2866 = {
        let a2859 = 2u64;
        let a2860 = f97(v1.clone())?;
        nat_mul(a2859, a2860)?
    };
    let a2867 = {
        let a2864 = f97(v1)?;
        let a2865 = {
            let a2862 = 1u64;
            let a2863 = match v0 {
                Adt3::C0(h2861, _, _, _, _, _, _, _) => {
                    h2861
                }
            };
            nat_add(a2862, a2863)?
        };
        nat_mul(a2864, a2865)?
    };
    nat_add(a2866, a2867)
}

pub fn f122(v0: u64, v1: Adt3) -> R<Adt8> {
    Ok(Adt8::C4(19u64, 4u64, f106(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f123(v0: u64, v1: u64) -> bool {
    let a2868 = v1;
    let a2869 = 2147483647u64;
    if nat_le(a2868, a2869) {
        let a2870 = v1;
        let a2871 = v0;
        nat_lt(a2870, a2871)
    } else {
        false
    }
}

pub fn f124(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt8> {
    let m2872 = f86(v0);
    match m2872 {
        Adt13::C0 => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt13::C1(v7) => {
            let v8: u64 = {
                let a2874 = v2;
                let a2875 = match v7.clone() {
                    Adt14::C0(_, h2873, _) => {
                        h2873
                    }
                };
                nat_mul(a2874, a2875)?
            };
            let m2888 = f156(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2876, _, _, _, _, _, _, _) => {
                    h2876
                }
            }, match v5.clone() {
                Adt3::C0(_, h2877, _, _, _, _, _, _) => {
                    h2877
                }
            }, match v5 {
                Adt3::C0(_, _, h2878, _, _, _, _, _) => {
                    h2878
                }
            }, {
                let a2881 = match v3.clone() {
                    Adt3::C0(h2879, _, _, _, _, _, _, _) => {
                        h2879
                    }
                };
                let a2882 = match v7.clone() {
                    Adt14::C0(_, h2880, _) => {
                        h2880
                    }
                };
                nat_mul(a2881, a2882)?
            }, match v3.clone() {
                Adt3::C0(_, h2883, _, _, _, _, _, _) => {
                    h2883
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2884, _, _, _) => {
                    h2884
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2885, _, _) => {
                    h2885
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2886, _) => {
                    h2886
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2887) => {
                    h2887
                }
            })?;
            match m2888 {
                None => {
                    Ok(Adt8::C0(Adt9::C1))
                }
                Some(v9) => {
                    let a2889 = v0;
                    let a2890 = 0u64;
                    if if nat_eq(a2889, a2890) {
                        true
                    } else {
                        let a2891 = v0;
                        let a2892 = 26u64;
                        nat_eq(a2891, a2892)
                    } {
                        Ok(Adt8::C1(false, v9, f106({
                            let a2896 = {
                                let a2894 = v1;
                                let a2895 = match v4.clone() {
                                    Adt3::C0(h2893, _, _, _, _, _, _, _) => {
                                        h2893
                                    }
                                };
                                nat_mul(a2894, a2895)?
                            };
                            let a2897 = v8;
                            nat_add(a2896, a2897)?
                        }, v4)?))
                    } else {
                        Ok(Adt8::C5(v0, v9, match v7.clone() {
                            Adt14::C0(_, _, h2898) => {
                                h2898
                            }
                        }, f106({
                            let a2902 = {
                                let a2900 = v1;
                                let a2901 = match v4.clone() {
                                    Adt3::C0(h2899, _, _, _, _, _, _, _) => {
                                        h2899
                                    }
                                };
                                nat_mul(a2900, a2901)?
                            };
                            let a2903 = v8;
                            nat_add(a2902, a2903)?
                        }, v4.clone())?, {
                            let a2910 = {
                                let a2908 = {
                                    let a2905 = match v7 {
                                        Adt14::C0(_, h2904, _) => {
                                            h2904
                                        }
                                    };
                                    let a2906 = 1u64;
                                    nat_sub(a2905, a2906)
                                };
                                let a2909 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2907, _, _, _) => {
                                        h2907
                                    }
                                };
                                nat_mul(a2908, a2909)?
                            };
                            let a2911 = 4u64;
                            nat_add(a2910, a2911)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2912, _, _, _) => {
                                h2912
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f125(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2913 = v1;
    let a2914 = 0u64;
    if if nat_eq(a2913, a2914) {
        true
    } else {
        let a2915 = v2;
        let a2916 = 0u64;
        nat_eq(a2915, a2916)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f126(v0: u64, v1: u64) -> bool {
    let a2917 = v0;
    let a2918 = 4294967295u64;
    if nat_eq(a2917, a2918) {
        true
    } else {
        let a2919 = v0;
        let a2920 = 2147483648u64;
        if nat_lt(a2919, a2920) {
            let a2921 = v0;
            let a2922 = v1;
            nat_lt(a2921, a2922)
        } else {
            false
        }
    }
}

pub fn f127(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt8> {
    let v5: u64 = {
        let a2929 = v3;
        let a2930 = {
            let a2926 = f106({
                let a2923 = v0;
                let a2924 = 2u64;
                let a2925 = 0u64;
                nat_quot(a2923, a2924, a2925)
            }, v1)?;
            let a2927 = v2;
            let a2928 = 0u64;
            nat_quot(a2926, a2927, a2928)
        };
        nat_add(a2929, a2930)?
    };
    let a2934 = {
        let a2931 = v0;
        let a2932 = 2u64;
        let a2933 = 0u64;
        nat_rem(a2931, a2932, a2933)
    };
    let a2935 = 0u64;
    if nat_eq(a2934, a2935) {
        Ok(Adt8::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2936 = v4;
        let a2937 = 0u64;
        if nat_eq(a2936, a2937) {
            Ok(Adt8::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt8::C0(Adt9::C1))
        }
    }
}

pub fn f128(v0: u64, v1: u64) -> R<u64> {
    let a2940 = 8u64;
    let a2941 = {
        let a2938 = f157(v0);
        let a2939 = v1;
        nat_mul(a2938, a2939)?
    };
    nat_add(a2940, a2941)
}

pub fn f129(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt8> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2942, _, _, _, _, _, _, _) => {
            h2942
        }
    };
    let v7: u64 = f157(v3);
    let v8: u64 = {
        let a2945 = 1u64;
        let a2946 = {
            let a2943 = v6;
            let a2944 = v7;
            nat_mul(a2943, a2944)?
        };
        nat_add(a2945, a2946)?
    };
    let a2947 = v0;
    let a2948 = 0u64;
    if nat_eq(a2947, a2948) {
        Ok(f158(0u64))
    } else {
        let a2949 = v0;
        let a2950 = v8;
        if nat_lt(a2949, a2950) {
            let v9: u64 = {
                let a2951 = v0;
                let a2952 = 1u64;
                nat_sub(a2951, a2952)
            };
            let v10: u64 = {
                let a2953 = v9;
                let a2954 = v7;
                let a2955 = 0u64;
                nat_rem(a2953, a2954, a2955)
            };
            let v11: u64 = f106({
                let a2961 = {
                    let a2956 = v1;
                    let a2957 = v6;
                    nat_mul(a2956, a2957)?
                };
                let a2962 = {
                    let a2958 = v9;
                    let a2959 = v7;
                    let a2960 = 0u64;
                    nat_quot(a2958, a2959, a2960)
                };
                nat_add(a2961, a2962)?
            }, v2)?;
            let a2963 = v3;
            let a2964 = 30u64;
            if nat_eq(a2963, a2964) {
                let a2965 = v10;
                let a2966 = 0u64;
                if nat_eq(a2965, a2966) {
                    Ok(Adt8::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2967 = v10;
                    let a2968 = 1u64;
                    if nat_eq(a2967, a2968) {
                        Ok(Adt8::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2969 = v10;
                        let a2970 = 2u64;
                        if nat_eq(a2969, a2970) {
                            Ok(Adt8::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt8::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2971 = v10;
                let a2972 = 0u64;
                if nat_eq(a2971, a2972) {
                    Ok(Adt8::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2973 = v10;
                    let a2974 = 1u64;
                    if nat_eq(a2973, a2974) {
                        Ok(Adt8::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt8::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2975 = v0;
                let a2976 = v8;
                nat_sub(a2975, a2976)
            };
            let a2977 = v12;
            let a2978 = 0u64;
            if nat_eq(a2977, a2978) {
                Ok(Adt8::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2979 = v12;
                let a2980 = 1u64;
                if nat_eq(a2979, a2980) {
                    Ok(Adt8::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2981 = v12;
                    let a2982 = 2u64;
                    if nat_eq(a2981, a2982) {
                        Ok(Adt8::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2983 = v12;
                        let a2984 = 3u64;
                        if nat_eq(a2983, a2984) {
                            Ok(Adt8::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2985 = v12;
                            let a2986 = 4u64;
                            if nat_eq(a2985, a2986) {
                                Ok(Adt8::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2987 = v12;
                                let a2988 = 5u64;
                                if nat_eq(a2987, a2988) {
                                    Ok(Adt8::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2989 = v12;
                                    let a2990 = 6u64;
                                    if nat_eq(a2989, a2990) {
                                        if f159(v5) {
                                            Ok(Adt8::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2991 = 3u64;
                                                let a2992 = v1;
                                                nat_add(a2991, a2992)?
                                            }))
                                        } else {
                                            Ok(Adt8::C0(Adt9::C1))
                                        }
                                    } else {
                                        Ok(Adt8::C0(Adt9::C1))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f130(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt8> {
    let v4: u64 = {
        let a2993 = v0;
        let a2994 = f80(v3);
        let a2995 = 0u64;
        nat_quot(a2993, a2994, a2995)
    };
    let v5: u64 = {
        let a3000 = 3u64;
        let a3001 = {
            let a2997 = v4;
            let a2998 = match v1.clone() {
                Adt3::C0(h2996, _, _, _, _, _, _, _) => {
                    h2996
                }
            };
            let a2999 = 0u64;
            nat_quot(a2997, a2998, a2999)
        };
        nat_add(a3000, a3001)?
    };
    let v6: u64 = f106(v4, v1)?;
    let v7: u64 = f106(v4, v2)?;
    let a3002 = v3;
    let a3003 = 30u64;
    if nat_eq(a3002, a3003) {
        let a3007 = {
            let a3004 = v0;
            let a3005 = 3u64;
            let a3006 = 0u64;
            nat_rem(a3004, a3005, a3006)
        };
        let a3008 = 0u64;
        if nat_eq(a3007, a3008) {
            Ok(Adt8::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3012 = {
                let a3009 = v0;
                let a3010 = 3u64;
                let a3011 = 0u64;
                nat_rem(a3009, a3010, a3011)
            };
            let a3013 = 1u64;
            if nat_eq(a3012, a3013) {
                Ok(Adt8::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt8::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt8::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f131(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3014 = v0;
        let a3015 = 8388608u64;
        let a3016 = 0u64;
        nat_quot(a3014, a3015, a3016)
    };
    let a3017 = v0;
    let a3018 = 2147483648u64;
    if if if nat_lt(a3017, a3018) {
        let a3019 = 127u64;
        let a3020 = v1;
        nat_le(a3019, a3020)
    } else {
        false
    } {
        let a3021 = v1;
        let a3022 = 190u64;
        nat_lt(a3021, a3022)
    } else {
        false
    } {
        let v2: u64 = {
            let a3026 = 8388608u64;
            let a3027 = {
                let a3023 = v0;
                let a3024 = 8388608u64;
                let a3025 = 0u64;
                nat_rem(a3023, a3024, a3025)
            };
            nat_add(a3026, a3027)?
        };
        let a3028 = v1;
        let a3029 = 150u64;
        if nat_lt(a3028, a3029) {
            let v3: u64 = f160({
                let a3030 = 150u64;
                let a3031 = v1;
                nat_sub(a3030, a3031)
            });
            let a3035 = {
                let a3032 = v2;
                let a3033 = v3;
                let a3034 = 0u64;
                nat_rem(a3032, a3033, a3034)
            };
            let a3036 = 0u64;
            if nat_eq(a3035, a3036) {
                let a3037 = v2;
                let a3038 = v3;
                let a3039 = 0u64;
                Ok(nat_quot(a3037, a3038, a3039))
            } else {
                Ok(0u64)
            }
        } else {
            let a3042 = f160({
                let a3040 = v1;
                let a3041 = 150u64;
                nat_sub(a3040, a3041)
            });
            let a3043 = v2;
            nat_mul(a3042, a3043)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64) -> bool {
    let a3044 = v0;
    let a3045 = 1u64;
    if nat_le(a3044, a3045) {
        true
    } else {
        let a3049 = {
            let a3046 = v1;
            let a3047 = v2;
            let a3048 = 1u64;
            nat_rem(a3046, a3047, a3048)
        };
        let a3050 = 0u64;
        nat_eq(a3049, a3050)
    }
}

pub fn f133(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3051 = v1;
    let a3052 = 1u64;
    if if if nat_le(a3051, a3052) {
        true
    } else {
        let a3053 = v0;
        let a3054 = v2;
        nat_le(a3053, a3054)
    } {
        let a3055 = v3;
        let a3056 = 1u64;
        if nat_le(a3055, a3056) {
            true
        } else {
            let a3063 = {
                let a3061 = v0;
                let a3062 = {
                    let a3059 = {
                        let a3057 = v1;
                        let a3058 = 1u64;
                        nat_sub(a3057, a3058)
                    };
                    let a3060 = v2;
                    nat_mul(a3059, a3060)?
                };
                nat_add(a3061, a3062)?
            };
            let a3064 = v4;
            nat_le(a3063, a3064)
        }
    } else {
        false
    } {
        let a3065 = v5;
        let a3066 = 1u64;
        if nat_le(a3065, a3066) {
            Ok(true)
        } else {
            let a3079 = {
                let a3077 = {
                    let a3071 = v0;
                    let a3072 = {
                        let a3069 = {
                            let a3067 = v1;
                            let a3068 = 1u64;
                            nat_sub(a3067, a3068)
                        };
                        let a3070 = v2;
                        nat_mul(a3069, a3070)?
                    };
                    nat_add(a3071, a3072)?
                };
                let a3078 = {
                    let a3075 = {
                        let a3073 = v3;
                        let a3074 = 1u64;
                        nat_sub(a3073, a3074)
                    };
                    let a3076 = v4;
                    nat_mul(a3075, a3076)?
                };
                nat_add(a3077, a3078)?
            };
            let a3080 = v6;
            Ok(nat_le(a3079, a3080))
        }
    } else {
        Ok(false)
    }
}

pub fn f134(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f161(v0, v1, v6, v7, v8) {
        let a3081 = v5;
        let a3082 = 26u64;
        nat_eq(a3081, a3082)
    } else {
        false
    } {
        let a3083 = v3;
        let a3084 = v7;
        nat_eq(a3083, a3084)
    } else {
        false
    } {
        let a3085 = v4;
        let a3086 = v8;
        nat_eq(a3085, a3086)
    } else {
        false
    } {
        let a3087 = v9;
        let a3088 = 1u64;
        nat_eq(a3087, a3088)
    } else {
        false
    } {
        let a3089 = v10;
        let a3090 = f162(v0);
        nat_eq(a3089, a3090)
    } else {
        false
    } {
        let a3091 = v11;
        let a3092 = v1;
        nat_eq(a3091, a3092)
    } else {
        false
    } {
        let a3093 = v12;
        let a3094 = v6;
        nat_eq(a3093, a3094)
    } else {
        false
    } {
        let a3095 = v13;
        let a3096 = v7;
        nat_eq(a3095, a3096)
    } else {
        false
    } {
        let a3097 = v14;
        let a3098 = v8;
        nat_eq(a3097, a3098)
    } else {
        false
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3099 = v0;
    let a3100 = 0u64;
    if if if nat_eq(a3099, a3100) {
        true
    } else {
        let a3101 = v1;
        let a3102 = 0u64;
        nat_eq(a3101, a3102)
    } {
        true
    } else {
        let a3103 = v2;
        let a3104 = 0u64;
        nat_eq(a3103, a3104)
    } {
        true
    } else {
        let a3105 = v3;
        let a3106 = 0u64;
        nat_eq(a3105, a3106)
    }
}

pub fn f136(v0: Adt3, v1: bool) -> R<u64> {
    f163(v0.clone(), {
        let a3108 = match v0.clone() {
            Adt3::C0(h3107, _, _, _, _, _, _, _) => {
                h3107
            }
        };
        let a3109 = 1u64;
        nat_sub(a3108, a3109)
    }, {
        let a3111 = match v0.clone() {
            Adt3::C0(_, h3110, _, _, _, _, _, _) => {
                h3110
            }
        };
        let a3112 = 1u64;
        nat_sub(a3111, a3112)
    }, {
        let a3114 = match v0.clone() {
            Adt3::C0(_, _, h3113, _, _, _, _, _) => {
                h3113
            }
        };
        let a3115 = 1u64;
        nat_sub(a3114, a3115)
    }, {
        let a3117 = match v0 {
            Adt3::C0(_, _, _, h3116, _, _, _, _) => {
                h3116
            }
        };
        let a3118 = 1u64;
        nat_sub(a3117, a3118)
    }, v1)
}

pub fn f137(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3119 = v0;
    let a3120 = 0u64;
    if nat_eq(a3119, a3120) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a3121 = v0;
            let a3122 = 1u64;
            nat_sub(a3121, a3122)
        };
        let v5: u64 = {
            let a3123 = v4;
            let a3124 = v1;
            let a3125 = 0u64;
            nat_quot(a3123, a3124, a3125)
        };
        let v6: u64 = {
            let a3133 = {
                let a3126 = v5;
                let a3127 = v2;
                nat_mul(a3126, a3127)?
            };
            let a3134 = {
                let a3131 = {
                    let a3128 = v4;
                    let a3129 = v1;
                    let a3130 = 0u64;
                    nat_rem(a3128, a3129, a3130)
                };
                let a3132 = v3;
                nat_mul(a3131, a3132)?
            };
            nat_add(a3133, a3134)?
        };
        let a3135 = v5;
        let a3136 = 0u64;
        if nat_eq(a3135, a3136) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a3145 = {
                    let a3139 = {
                        let a3137 = v5;
                        let a3138 = 1u64;
                        nat_sub(a3137, a3138)
                    };
                    let a3140 = v2;
                    nat_mul(a3139, a3140)?
                };
                let a3146 = {
                    let a3143 = {
                        let a3141 = v1;
                        let a3142 = 1u64;
                        nat_sub(a3141, a3142)
                    };
                    let a3144 = v3;
                    nat_mul(a3143, a3144)?
                };
                nat_add(a3145, a3146)?
            };
            let a3147 = v6;
            let a3148 = v7;
            if nat_lt(a3147, a3148) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f138(v0: Adt3, v1: u64) -> bool {
    let a3149 = v1;
    let a3150 = 0u64;
    if !nat_eq(a3149, a3150) {
        true
    } else {
        let a3152 = match v0 {
            Adt3::C0(_, _, _, _, h3151, _, _, _) => {
                h3151
            }
        };
        let a3153 = 4u64;
        nat_eq(a3152, a3153)
    }
}

pub fn f139(v0: u64, v1: bool) -> R<u64> {
    let a3156 = {
        let a3154 = v0;
        let a3155 = 2u64;
        nat_mul(a3154, a3155)?
    };
    let a3157 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a3156, a3157)
}

pub fn f140(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt9> {
    let m3158 = f164(v0, v1);
    match m3158 {
        None => {
            if v5 {
                let m3159 = f165(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m3159 {
                    Adt15::C0 => {
                        Ok(Adt9::C0)
                    }
                    Adt15::C1 => {
                        Ok(Adt9::C1)
                    }
                    Adt15::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt9::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt9::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a3160 = 4u64;
                let a3161 = v28;
                nat_le(a3160, a3161)
            };
            let a3162 = v2;
            let a3163 = 0u64;
            if if if if if nat_eq(a3162, a3163) {
                let a3164 = v4;
                let a3165 = 0u64;
                nat_eq(a3164, a3165)
            } else {
                false
            } {
                let a3166 = v6;
                let a3167 = v14;
                if nat_eq(a3166, a3167) {
                    let a3168 = v7;
                    let a3169 = v15;
                    nat_eq(a3168, a3169)
                } else {
                    false
                }
            } else {
                false
            } {
                let a3170 = v8;
                let a3171 = v16;
                if nat_eq(a3170, a3171) {
                    let a3172 = v9;
                    let a3173 = v17;
                    nat_eq(a3172, a3173)
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
                        let a3174 = v3;
                        let a3175 = 0u64;
                        nat_eq(a3174, a3175)
                    } else {
                        false
                    } {
                        f87(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt9::C2(v28, v29, v6, v7, v8, v9, {
                    let a3180 = {
                        let a3176 = v6;
                        let a3177 = v7;
                        nat_mul(a3176, a3177)?
                    };
                    let a3181 = {
                        let a3178 = v8;
                        let a3179 = v9;
                        nat_mul(a3178, a3179)?
                    };
                    nat_mul(a3180, a3181)?
                }))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f141(v0: u64) -> u64 {
    let m3182 = f86(v0);
    match m3182 {
        Adt13::C0 => {
            0u64
        }
        Adt13::C1(v1) => {
            let a3184 = match v1.clone() {
                Adt14::C0(_, h3183, _) => {
                    h3183
                }
            };
            let a3185 = 1u64;
            if if nat_eq(a3184, a3185) {
                let a3187 = match v1.clone() {
                    Adt14::C0(_, _, h3186) => {
                        h3186
                    }
                };
                let a3188 = 1u64;
                if if if nat_eq(a3187, a3188) {
                    true
                } else {
                    let a3190 = match v1.clone() {
                        Adt14::C0(_, _, h3189) => {
                            h3189
                        }
                    };
                    let a3191 = 2u64;
                    nat_eq(a3190, a3191)
                } {
                    true
                } else {
                    let a3193 = match v1.clone() {
                        Adt14::C0(_, _, h3192) => {
                            h3192
                        }
                    };
                    let a3194 = 4u64;
                    nat_eq(a3193, a3194)
                } {
                    true
                } else {
                    let a3196 = match v1.clone() {
                        Adt14::C0(_, _, h3195) => {
                            h3195
                        }
                    };
                    let a3197 = 8u64;
                    nat_eq(a3196, a3197)
                }
            } else {
                false
            } {
                match v1 {
                    Adt14::C0(_, _, h3198) => {
                        h3198
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f142(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3203 = {
        let a3201 = {
            let a3199 = v0;
            let a3200 = v1;
            nat_sub(a3199, a3200)
        };
        let a3202 = 1u64;
        nat_sub(a3201, a3202)
    };
    let a3204 = v2;
    nat_add(a3203, a3204)
}

pub fn f143(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f100({
        let a3205 = v0;
        let a3206 = v4;
        let a3207 = 0u64;
        nat_rem(a3205, a3206, a3207)
    }, {
        let a3208 = v1;
        let a3209 = v5;
        let a3210 = 0u64;
        nat_rem(a3208, a3209, a3210)
    }, {
        let a3211 = v2;
        let a3212 = v6;
        let a3213 = 0u64;
        nat_rem(a3211, a3212, a3213)
    }, {
        let a3214 = v3;
        let a3215 = v7;
        let a3216 = 0u64;
        nat_rem(a3214, a3215, a3216)
    }, v8, v9, v10, v11)
}

pub fn f144(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3217 = v1;
    let a3218 = 2147483648u64;
    if nat_lt(a3217, a3218) {
        let a3229 = {
            let a3227 = {
                let a3222 = {
                    let a3219 = v0;
                    let a3220 = v2;
                    let a3221 = 0u64;
                    nat_rem(a3219, a3220, a3221)
                };
                let a3223 = v2;
                nat_add(a3222, a3223)?
            };
            let a3228 = {
                let a3224 = v1;
                let a3225 = v2;
                let a3226 = 0u64;
                nat_rem(a3224, a3225, a3226)
            };
            nat_sub(a3227, a3228)
        };
        let a3230 = v2;
        let a3231 = 0u64;
        Ok(nat_rem(a3229, a3230, a3231))
    } else {
        let a3234 = {
            let a3232 = v0;
            let a3233 = f73(v1);
            nat_add(a3232, a3233)?
        };
        let a3235 = v2;
        let a3236 = 0u64;
        Ok(nat_rem(a3234, a3235, a3236))
    }
}

pub fn f145(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3239 = f72(v1);
    let a3240 = {
        let a3237 = v0;
        let a3238 = f73(v1);
        nat_add(a3237, a3238)?
    };
    if nat_le(a3239, a3240) {
        let a3241 = f146(v0, v1)?;
        let a3242 = v2;
        Ok(nat_lt(a3241, a3242))
    } else {
        Ok(false)
    }
}

pub fn f146(v0: u64, v1: u64) -> R<u64> {
    let a3245 = {
        let a3243 = v0;
        let a3244 = f73(v1);
        nat_add(a3243, a3244)?
    };
    let a3246 = f72(v1);
    Ok(nat_sub(a3245, a3246))
}

pub fn f147(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3247 = v0;
    let a3248 = v1;
    if nat_lt(a3247, a3248) {
        let a3249 = v1;
        let a3250 = v0;
        Ok(nat_sub(a3249, a3250))
    } else {
        let a3253 = v0;
        let a3254 = {
            let a3251 = v1;
            let a3252 = v2;
            nat_add(a3251, a3252)?
        };
        if nat_lt(a3253, a3254) {
            let a3255 = v0;
            let a3256 = v1;
            Ok(nat_sub(a3255, a3256))
        } else {
            let a3263 = {
                let a3259 = 2u64;
                let a3260 = {
                    let a3257 = v2;
                    let a3258 = 1u64;
                    nat_sub(a3257, a3258)
                };
                nat_mul(a3259, a3260)?
            };
            let a3264 = {
                let a3261 = v0;
                let a3262 = v1;
                nat_sub(a3261, a3262)
            };
            Ok(nat_sub(a3263, a3264))
        }
    }
}

pub fn f148(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3267 = {
        let a3265 = v2;
        let a3266 = f163(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a3265, a3266)?
    };
    let a3268 = f163(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3267, a3268))
}

pub fn f149(v0: u64, v1: u64, v2: u64) -> bool {
    let a3269 = v0;
    let a3270 = 0u64;
    if nat_eq(a3269, a3270) {
        let a3271 = v2;
        let a3272 = v1;
        nat_le(a3271, a3272)
    } else {
        let a3273 = v0;
        let a3274 = 1u64;
        if nat_eq(a3273, a3274) {
            let a3275 = v2;
            let a3276 = v1;
            nat_lt(a3275, a3276)
        } else {
            let a3277 = v0;
            let a3278 = 2u64;
            if nat_eq(a3277, a3278) {
                let a3279 = v1;
                let a3280 = v2;
                nat_le(a3279, a3280)
            } else {
                let a3281 = v0;
                let a3282 = 3u64;
                if nat_eq(a3281, a3282) {
                    let a3283 = v1;
                    let a3284 = v2;
                    nat_lt(a3283, a3284)
                } else {
                    let a3285 = v1;
                    let a3286 = v2;
                    nat_eq(a3285, a3286)
                }
            }
        }
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: u64) -> R<Adt8> {
    let a3287 = v0;
    let a3288 = 0u64;
    if nat_eq(a3287, a3288) {
        Ok(f158(0u64))
    } else {
        let a3291 = {
            let a3289 = v0;
            let a3290 = 1u64;
            nat_add(a3289, a3290)?
        };
        let a3292 = f98(v1.clone(), 13u64, v2)?;
        if nat_eq(a3291, a3292) {
            Ok(Adt8::C4({
                let a3293 = v2;
                let a3294 = 0u64;
                if nat_eq(a3293, a3294) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3295 = v0;
                let a3296 = 1u64;
                nat_sub(a3295, a3296)
            };
            if f138(v1.clone(), v2) {
                let v4: u64 = f139(match v1.clone() {
                    Adt3::C0(h3297, _, _, _, _, _, _, _) => {
                        h3297
                    }
                }, false)?;
                let v5: u64 = {
                    let a3298 = v3;
                    let a3299 = v4;
                    let a3300 = 0u64;
                    nat_quot(a3298, a3299, a3300)
                };
                let v6: u64 = {
                    let a3301 = v3;
                    let a3302 = v4;
                    let a3303 = 0u64;
                    nat_rem(a3301, a3302, a3303)
                };
                let a3306 = {
                    let a3304 = v6;
                    let a3305 = 1u64;
                    nat_add(a3304, a3305)?
                };
                let a3307 = v4;
                if nat_eq(a3306, a3307) {
                    Ok(Adt8::C4({
                        let a3308 = v2;
                        let a3309 = 0u64;
                        if nat_eq(a3308, a3309) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f151(v6, v5, v1, v2)
                }
            } else {
                f166({
                    let a3310 = v3;
                    let a3311 = 2u64;
                    let a3312 = 0u64;
                    nat_quot(a3310, a3311, a3312)
                }, {
                    let a3313 = v3;
                    let a3314 = 2u64;
                    let a3315 = 0u64;
                    nat_rem(a3313, a3314, a3315)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f151(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt8> {
    let a3316 = v0;
    let a3317 = 0u64;
    if nat_eq(a3316, a3317) {
        Ok(f158(1u64))
    } else {
        f166({
            let a3326 = {
                let a3319 = v1;
                let a3320 = match v2.clone() {
                    Adt3::C0(h3318, _, _, _, _, _, _, _) => {
                        h3318
                    }
                };
                nat_mul(a3319, a3320)?
            };
            let a3327 = {
                let a3323 = {
                    let a3321 = v0;
                    let a3322 = 1u64;
                    nat_sub(a3321, a3322)
                };
                let a3324 = 2u64;
                let a3325 = 0u64;
                nat_quot(a3323, a3324, a3325)
            };
            nat_add(a3326, a3327)?
        }, {
            let a3330 = {
                let a3328 = v0;
                let a3329 = 1u64;
                nat_sub(a3328, a3329)
            };
            let a3331 = 2u64;
            let a3332 = 0u64;
            nat_rem(a3330, a3331, a3332)
        }, v2, v3, 1u64)
    }
}

pub fn f152(v0: u64, v1: u64) -> R<u64> {
    let a3333 = v0;
    let a3334 = f75(v1)?;
    let a3335 = 0u64;
    Ok(nat_rem(a3333, a3334, a3335))
}

pub fn f153(v0: u64, v1: u64) -> R<u64> {
    let a3336 = 4u64;
    let a3337 = f167(v0, v1)?;
    nat_mul(a3336, a3337)
}

pub fn f154(v0: u64, v1: u64) -> R<u64> {
    let a3338 = 4u64;
    let a3339 = f168(v0, v1)?;
    nat_mul(a3338, a3339)
}

pub fn f155(v0: u64, v1: u64) -> R<bool> {
    let a3345 = {
        let a3342 = {
            let a3340 = f152(v0, v1)?;
            let a3341 = 2u64;
            nat_sub(a3340, a3341)
        };
        let a3343 = 2u64;
        let a3344 = 0u64;
        nat_rem(a3342, a3343, a3344)
    };
    let a3346 = 0u64;
    Ok(nat_eq(a3345, a3346))
}

pub fn f156(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3347 = f86(v0);
    match m3347 {
        Adt13::C0 => {
            Ok(None::<u64>)
        }
        Adt13::C1(v13) => {
            if if if if if f169(v0) {
                let a3348 = v1;
                let a3349 = v7;
                nat_lt(a3348, a3349)
            } else {
                false
            } {
                f123(v8, v2)
            } else {
                false
            } {
                let a3350 = v3;
                let a3351 = f170(v4, v5, v6)?;
                nat_lt(a3350, a3351)
            } else {
                false
            } {
                let a3356 = {
                    let a3353 = v7;
                    let a3354 = match v13.clone() {
                        Adt14::C0(_, h3352, _) => {
                            h3352
                        }
                    };
                    let a3355 = 0u64;
                    nat_rem(a3353, a3354, a3355)
                };
                let a3357 = 0u64;
                nat_eq(a3356, a3357)
            } else {
                false
            } {
                Ok(Some({
                    let a3386 = {
                        let a3376 = {
                            let a3366 = {
                                let a3362 = {
                                    let a3359 = v1;
                                    let a3360 = match v13 {
                                        Adt14::C0(_, h3358, _) => {
                                            h3358
                                        }
                                    };
                                    let a3361 = 0u64;
                                    nat_quot(a3359, a3360, a3361)
                                };
                                let a3363 = v9;
                                nat_mul(a3362, a3363)?
                            };
                            let a3367 = {
                                let a3364 = v2;
                                let a3365 = v10;
                                nat_mul(a3364, a3365)?
                            };
                            nat_add(a3366, a3367)?
                        };
                        let a3377 = {
                            let a3374 = {
                                let a3371 = {
                                    let a3368 = v3;
                                    let a3369 = v4;
                                    let a3370 = 0u64;
                                    nat_quot(a3368, a3369, a3370)
                                };
                                let a3372 = v5;
                                let a3373 = 0u64;
                                nat_rem(a3371, a3372, a3373)
                            };
                            let a3375 = v11;
                            nat_mul(a3374, a3375)?
                        };
                        nat_add(a3376, a3377)?
                    };
                    let a3387 = {
                        let a3384 = {
                            let a3381 = {
                                let a3378 = v3;
                                let a3379 = v4;
                                let a3380 = 0u64;
                                nat_quot(a3378, a3379, a3380)
                            };
                            let a3382 = v5;
                            let a3383 = 0u64;
                            nat_quot(a3381, a3382, a3383)
                        };
                        let a3385 = v12;
                        nat_mul(a3384, a3385)?
                    };
                    nat_add(a3386, a3387)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f157(v0: u64) -> u64 {
    let a3388 = v0;
    let a3389 = 30u64;
    if nat_eq(a3388, a3389) {
        4u64
    } else {
        3u64
    }
}

pub fn f158(v0: u64) -> Adt8 {
    Adt8::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f159(v0: u64) -> bool {
    let a3390 = 0u64;
    let a3391 = v0;
    if nat_lt(a3390, a3391) {
        let a3392 = v0;
        let a3393 = 2139095040u64;
        nat_le(a3392, a3393)
    } else {
        false
    }
}

pub fn f160(v0: u64) -> u64 {
    let a3394 = v0;
    let a3395 = 40u64;
    if nat_lt(a3394, a3395) {
        let a3396 = v0;
        let a3397 = 20u64;
        if nat_lt(a3396, a3397) {
            let a3398 = v0;
            let a3399 = 10u64;
            if nat_lt(a3398, a3399) {
                let a3400 = v0;
                let a3401 = 5u64;
                if nat_lt(a3400, a3401) {
                    let a3402 = v0;
                    let a3403 = 2u64;
                    if nat_lt(a3402, a3403) {
                        let a3404 = v0;
                        let a3405 = 1u64;
                        if nat_lt(a3404, a3405) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a3406 = v0;
                        let a3407 = 3u64;
                        if nat_lt(a3406, a3407) {
                            4u64
                        } else {
                            let a3408 = v0;
                            let a3409 = 4u64;
                            if nat_lt(a3408, a3409) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a3410 = v0;
                    let a3411 = 7u64;
                    if nat_lt(a3410, a3411) {
                        let a3412 = v0;
                        let a3413 = 6u64;
                        if nat_lt(a3412, a3413) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a3414 = v0;
                        let a3415 = 8u64;
                        if nat_lt(a3414, a3415) {
                            128u64
                        } else {
                            let a3416 = v0;
                            let a3417 = 9u64;
                            if nat_lt(a3416, a3417) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a3418 = v0;
                let a3419 = 15u64;
                if nat_lt(a3418, a3419) {
                    let a3420 = v0;
                    let a3421 = 12u64;
                    if nat_lt(a3420, a3421) {
                        let a3422 = v0;
                        let a3423 = 11u64;
                        if nat_lt(a3422, a3423) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a3424 = v0;
                        let a3425 = 13u64;
                        if nat_lt(a3424, a3425) {
                            4096u64
                        } else {
                            let a3426 = v0;
                            let a3427 = 14u64;
                            if nat_lt(a3426, a3427) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a3428 = v0;
                    let a3429 = 17u64;
                    if nat_lt(a3428, a3429) {
                        let a3430 = v0;
                        let a3431 = 16u64;
                        if nat_lt(a3430, a3431) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a3432 = v0;
                        let a3433 = 18u64;
                        if nat_lt(a3432, a3433) {
                            131072u64
                        } else {
                            let a3434 = v0;
                            let a3435 = 19u64;
                            if nat_lt(a3434, a3435) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a3436 = v0;
            let a3437 = 30u64;
            if nat_lt(a3436, a3437) {
                let a3438 = v0;
                let a3439 = 25u64;
                if nat_lt(a3438, a3439) {
                    let a3440 = v0;
                    let a3441 = 22u64;
                    if nat_lt(a3440, a3441) {
                        let a3442 = v0;
                        let a3443 = 21u64;
                        if nat_lt(a3442, a3443) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a3444 = v0;
                        let a3445 = 23u64;
                        if nat_lt(a3444, a3445) {
                            4194304u64
                        } else {
                            let a3446 = v0;
                            let a3447 = 24u64;
                            if nat_lt(a3446, a3447) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a3448 = v0;
                    let a3449 = 27u64;
                    if nat_lt(a3448, a3449) {
                        let a3450 = v0;
                        let a3451 = 26u64;
                        if nat_lt(a3450, a3451) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a3452 = v0;
                        let a3453 = 28u64;
                        if nat_lt(a3452, a3453) {
                            134217728u64
                        } else {
                            let a3454 = v0;
                            let a3455 = 29u64;
                            if nat_lt(a3454, a3455) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a3456 = v0;
                let a3457 = 35u64;
                if nat_lt(a3456, a3457) {
                    let a3458 = v0;
                    let a3459 = 32u64;
                    if nat_lt(a3458, a3459) {
                        let a3460 = v0;
                        let a3461 = 31u64;
                        if nat_lt(a3460, a3461) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3462 = v0;
                        let a3463 = 33u64;
                        if nat_lt(a3462, a3463) {
                            4294967296u64
                        } else {
                            let a3464 = v0;
                            let a3465 = 34u64;
                            if nat_lt(a3464, a3465) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3466 = v0;
                    let a3467 = 37u64;
                    if nat_lt(a3466, a3467) {
                        let a3468 = v0;
                        let a3469 = 36u64;
                        if nat_lt(a3468, a3469) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3470 = v0;
                        let a3471 = 38u64;
                        if nat_lt(a3470, a3471) {
                            137438953472u64
                        } else {
                            let a3472 = v0;
                            let a3473 = 39u64;
                            if nat_lt(a3472, a3473) {
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

pub fn f161(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f135(v1, v2, v3, v4) {
        f171(v0)
    } else {
        f169(v0)
    }
}

pub fn f162(v0: u64) -> u64 {
    let a3474 = v0;
    let a3475 = 26u64;
    if nat_eq(a3474, a3475) {
        26u64
    } else {
        0u64
    }
}

pub fn f163(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3496 = {
        let a3490 = {
            let a3484 = {
                let a3478 = v1;
                let a3479 = if v5 {
                    f73(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3476, _, _, _) => {
                            h3476
                        }
                    })
                } else {
                    f72(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3477, _, _, _) => {
                            h3477
                        }
                    })
                };
                nat_mul(a3478, a3479)?
            };
            let a3485 = {
                let a3482 = v2;
                let a3483 = if v5 {
                    f73(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3480, _, _) => {
                            h3480
                        }
                    })
                } else {
                    f72(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3481, _, _) => {
                            h3481
                        }
                    })
                };
                nat_mul(a3482, a3483)?
            };
            nat_add(a3484, a3485)?
        };
        let a3491 = {
            let a3488 = v3;
            let a3489 = if v5 {
                f73(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3486, _) => {
                        h3486
                    }
                })
            } else {
                f72(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3487, _) => {
                        h3487
                    }
                })
            };
            nat_mul(a3488, a3489)?
        };
        nat_add(a3490, a3491)?
    };
    let a3497 = {
        let a3494 = v4;
        let a3495 = if v5 {
            f73(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3492) => {
                    h3492
                }
            })
        } else {
            f72(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3493) => {
                    h3493
                }
            })
        };
        nat_mul(a3494, a3495)?
    };
    nat_add(a3496, a3497)
}

pub fn f164(v0: u64, v1: u64) -> Option<u64> {
    let a3498 = v0;
    let a3499 = 2u64;
    if nat_eq(a3498, a3499) {
        Some(0u64)
    } else {
        let a3500 = v0;
        let a3501 = 5u64;
        if nat_eq(a3500, a3501) {
            Some(1u64)
        } else {
            let a3502 = v0;
            let a3503 = 6u64;
            if nat_eq(a3502, a3503) {
                Some(2u64)
            } else {
                let a3504 = v0;
                let a3505 = 7u64;
                if nat_eq(a3504, a3505) {
                    Some(3u64)
                } else {
                    let a3506 = v0;
                    let a3507 = 9u64;
                    if nat_eq(a3506, a3507) {
                        Some(4u64)
                    } else {
                        let a3508 = v0;
                        let a3509 = 8u64;
                        if nat_eq(a3508, a3509) {
                            Some(5u64)
                        } else {
                            let a3510 = v0;
                            let a3511 = 89u64;
                            if nat_eq(a3510, a3511) {
                                let a3512 = v1;
                                let a3513 = 2u64;
                                if nat_eq(a3512, a3513) {
                                    Some(6u64)
                                } else {
                                    let a3514 = v1;
                                    let a3515 = 0u64;
                                    if nat_eq(a3514, a3515) {
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

pub fn f165(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt15> {
    let a3516 = v0;
    let a3517 = 30u64;
    if nat_eq(a3516, a3517) {
        let a3518 = v2;
        let a3519 = 0u64;
        if if if nat_eq(a3518, a3519) {
            true
        } else {
            let a3520 = v2;
            let a3521 = 1u64;
            nat_eq(a3520, a3521)
        } {
            let a3522 = v1;
            let a3523 = 0u64;
            if nat_eq(a3522, a3523) {
                true
            } else {
                f172(v1, v4)
            }
        } else {
            false
        } {
            f173(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt15::C0)
        }
    } else {
        if if if f174(v1) {
            f174(v2)
        } else {
            false
        } {
            let a3524 = v1;
            let a3525 = 0u64;
            if if nat_eq(a3524, a3525) {
                true
            } else {
                let a3526 = v2;
                let a3527 = 0u64;
                nat_eq(a3526, a3527)
            } {
                true
            } else {
                let a3528 = v1;
                let a3529 = v2;
                nat_eq(a3528, a3529)
            }
        } else {
            false
        } {
            f173(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt15::C0)
        }
    }
}

pub fn f166(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt8> {
    let a3530 = v1;
    let a3531 = 0u64;
    if nat_eq(a3530, a3531) {
        Ok(Adt8::C4({
            let a3532 = v3;
            let a3533 = 0u64;
            if nat_eq(a3532, a3533) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f106(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt8::C4({
            let a3534 = v3;
            let a3535 = 0u64;
            if nat_eq(a3534, a3535) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f167(v0: u64, v1: u64) -> R<u64> {
    let a3536 = v0;
    let a3537 = f75(v1)?;
    let a3538 = 0u64;
    Ok(nat_quot(a3536, a3537, a3538))
}

pub fn f168(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f167(v0, v1)?;
    let v3: u64 = {
        let a3541 = {
            let a3539 = f152(v0, v1)?;
            let a3540 = 2u64;
            nat_sub(a3539, a3540)
        };
        let a3542 = 2u64;
        let a3543 = 0u64;
        nat_quot(a3541, a3542, a3543)
    };
    let v4: u64 = {
        let a3544 = v2;
        let a3545 = v1;
        nat_mul(a3544, a3545)?
    };
    if f155(v0, v1)? {
        let a3546 = v4;
        let a3547 = v3;
        nat_add(a3546, a3547)
    } else {
        let a3559 = {
            let a3555 = v4;
            let a3556 = {
                let a3553 = {
                    let a3548 = v1;
                    let a3549 = 1u64;
                    nat_sub(a3548, a3549)
                };
                let a3554 = {
                    let a3550 = v2;
                    let a3551 = v1;
                    let a3552 = 0u64;
                    nat_rem(a3550, a3551, a3552)
                };
                nat_mul(a3553, a3554)?
            };
            nat_sub(a3555, a3556)
        };
        let a3560 = {
            let a3557 = v3;
            let a3558 = v1;
            nat_mul(a3557, a3558)?
        };
        nat_add(a3559, a3560)
    }
}

pub fn f169(v0: u64) -> bool {
    let a3561 = v0;
    let a3562 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3561, a3562) {
        true
    } else {
        let a3563 = v0;
        let a3564 = 1u64;
        nat_eq(a3563, a3564)
    } {
        true
    } else {
        let a3565 = v0;
        let a3566 = 2u64;
        nat_eq(a3565, a3566)
    } {
        true
    } else {
        let a3567 = v0;
        let a3568 = 3u64;
        nat_eq(a3567, a3568)
    } {
        true
    } else {
        let a3569 = v0;
        let a3570 = 6u64;
        nat_eq(a3569, a3570)
    } {
        true
    } else {
        let a3571 = v0;
        let a3572 = 7u64;
        nat_eq(a3571, a3572)
    } {
        true
    } else {
        let a3573 = v0;
        let a3574 = 8u64;
        nat_eq(a3573, a3574)
    } {
        true
    } else {
        let a3575 = v0;
        let a3576 = 10u64;
        nat_eq(a3575, a3576)
    } {
        true
    } else {
        let a3577 = v0;
        let a3578 = 11u64;
        nat_eq(a3577, a3578)
    } {
        true
    } else {
        let a3579 = v0;
        let a3580 = 12u64;
        nat_eq(a3579, a3580)
    } {
        true
    } else {
        let a3581 = v0;
        let a3582 = 13u64;
        nat_eq(a3581, a3582)
    } {
        true
    } else {
        let a3583 = v0;
        let a3584 = 14u64;
        nat_eq(a3583, a3584)
    } {
        true
    } else {
        let a3585 = v0;
        let a3586 = 16u64;
        nat_eq(a3585, a3586)
    } {
        true
    } else {
        let a3587 = v0;
        let a3588 = 17u64;
        nat_eq(a3587, a3588)
    } {
        true
    } else {
        let a3589 = v0;
        let a3590 = 18u64;
        nat_eq(a3589, a3590)
    } {
        true
    } else {
        let a3591 = v0;
        let a3592 = 19u64;
        nat_eq(a3591, a3592)
    } {
        true
    } else {
        let a3593 = v0;
        let a3594 = 20u64;
        nat_eq(a3593, a3594)
    } {
        true
    } else {
        let a3595 = v0;
        let a3596 = 21u64;
        nat_eq(a3595, a3596)
    } {
        true
    } else {
        let a3597 = v0;
        let a3598 = 22u64;
        nat_eq(a3597, a3598)
    } {
        true
    } else {
        let a3599 = v0;
        let a3600 = 23u64;
        nat_eq(a3599, a3600)
    } {
        true
    } else {
        let a3601 = v0;
        let a3602 = 26u64;
        nat_eq(a3601, a3602)
    } {
        true
    } else {
        let a3603 = v0;
        let a3604 = 29u64;
        nat_eq(a3603, a3604)
    } {
        true
    } else {
        let a3605 = v0;
        let a3606 = 30u64;
        nat_eq(a3605, a3606)
    } {
        true
    } else {
        let a3607 = v0;
        let a3608 = 34u64;
        nat_eq(a3607, a3608)
    } {
        true
    } else {
        let a3609 = v0;
        let a3610 = 35u64;
        nat_eq(a3609, a3610)
    } {
        true
    } else {
        let a3611 = v0;
        let a3612 = 39u64;
        nat_eq(a3611, a3612)
    } {
        true
    } else {
        let a3613 = v0;
        let a3614 = 40u64;
        nat_eq(a3613, a3614)
    } {
        true
    } else {
        let a3615 = v0;
        let a3616 = 41u64;
        nat_eq(a3615, a3616)
    } {
        true
    } else {
        let a3617 = v0;
        let a3618 = 42u64;
        nat_eq(a3617, a3618)
    } {
        true
    } else {
        let a3619 = v0;
        let a3620 = 43u64;
        nat_eq(a3619, a3620)
    }
}

pub fn f170(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3621 = v0;
    let a3622 = 0u64;
    if if if nat_eq(a3621, a3622) {
        true
    } else {
        let a3623 = v1;
        let a3624 = 0u64;
        nat_eq(a3623, a3624)
    } {
        true
    } else {
        let a3625 = v2;
        let a3626 = 0u64;
        nat_eq(a3625, a3626)
    } {
        Ok(0u64)
    } else {
        let a3629 = {
            let a3627 = v0;
            let a3628 = v1;
            nat_mul(a3627, a3628)?
        };
        let a3630 = v2;
        nat_mul(a3629, a3630)
    }
}

pub fn f171(v0: u64) -> bool {
    let m3631 = f86(v0);
    match m3631 {
        Adt13::C0 => {
            false
        }
        Adt13::C1(_) => {
            true
        }
    }
}

pub fn f172(v0: u64, v1: u64) -> bool {
    let a3632 = v0;
    let a3633 = 1u64;
    if nat_eq(a3632, a3633) {
        true
    } else {
        let v2: u64 = f175(v0);
        let a3634 = 0u64;
        let a3635 = v2;
        if nat_lt(a3634, a3635) {
            let a3639 = {
                let a3636 = v1;
                let a3637 = v2;
                let a3638 = 1u64;
                nat_rem(a3636, a3637, a3638)
            };
            let a3640 = 0u64;
            nat_eq(a3639, a3640)
        } else {
            false
        }
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt15> {
    let a3641 = v0;
    let a3642 = 28u64;
    if if if if nat_eq(a3641, a3642) {
        let a3643 = v1;
        let a3644 = 0u64;
        nat_eq(a3643, a3644)
    } else {
        false
    } {
        let a3645 = v2;
        let a3646 = 0u64;
        nat_eq(a3645, a3646)
    } else {
        false
    } {
        let a3647 = v3;
        let a3648 = 0u64;
        nat_eq(a3647, a3648)
    } else {
        false
    } {
        let a3649 = v4;
        let a3650 = v8;
        if if if if if if if if nat_eq(a3649, a3650) {
            let a3651 = v5;
            let a3652 = v12;
            nat_eq(a3651, a3652)
        } else {
            false
        } {
            let a3653 = v9;
            let a3654 = v13;
            nat_eq(a3653, a3654)
        } else {
            false
        } {
            let a3655 = v10;
            let a3656 = v14;
            nat_eq(a3655, a3656)
        } else {
            false
        } {
            let a3657 = v11;
            let a3658 = v15;
            nat_eq(a3657, a3658)
        } else {
            false
        } {
            let a3659 = 0u64;
            let a3660 = v6;
            nat_lt(a3659, a3660)
        } else {
            false
        } {
            let a3661 = 0u64;
            let a3662 = v7;
            nat_lt(a3661, a3662)
        } else {
            false
        } {
            let a3666 = {
                let a3663 = v10;
                let a3664 = v6;
                let a3665 = 1u64;
                nat_rem(a3663, a3664, a3665)
            };
            let a3667 = 0u64;
            if nat_eq(a3666, a3667) {
                let a3671 = {
                    let a3668 = v11;
                    let a3669 = v7;
                    let a3670 = 1u64;
                    nat_rem(a3668, a3669, a3670)
                };
                let a3672 = 0u64;
                nat_eq(a3671, a3672)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt15::C2(v5, v4, v9, v10, v11, {
                let a3673 = v10;
                let a3674 = v6;
                let a3675 = 0u64;
                nat_quot(a3673, a3674, a3675)
            }, {
                let a3676 = v11;
                let a3677 = v7;
                let a3678 = 0u64;
                nat_quot(a3676, a3677, a3678)
            }, {
                let a3679 = v5;
                let a3680 = v4;
                nat_mul(a3679, a3680)?
            }, {
                let a3681 = v4;
                let a3682 = v9;
                nat_mul(a3681, a3682)?
            }, {
                let a3683 = v5;
                let a3684 = v9;
                nat_mul(a3683, a3684)?
            }))
        } else {
            Ok(Adt15::C1)
        }
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f174(v0: u64) -> bool {
    let a3685 = v0;
    let a3686 = 0u64;
    if if nat_eq(a3685, a3686) {
        true
    } else {
        let a3687 = v0;
        let a3688 = 1u64;
        nat_eq(a3687, a3688)
    } {
        true
    } else {
        let a3689 = v0;
        let a3690 = 30u64;
        nat_eq(a3689, a3690)
    }
}

pub fn f175(v0: u64) -> u64 {
    if f169(v0) {
        let m3691 = f86(v0);
        match m3691 {
            Adt13::C0 => {
                0u64
            }
            Adt13::C1(v1) => {
                let a3693 = 1u64;
                let a3694 = match v1.clone() {
                    Adt14::C0(_, h3692, _) => {
                        h3692
                    }
                };
                if if nat_lt(a3693, a3694) {
                    true
                } else {
                    let a3695 = v0;
                    let a3696 = 43u64;
                    nat_eq(a3695, a3696)
                } {
                    match v1 {
                        Adt14::C0(_, h3697, _) => {
                            h3697
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
