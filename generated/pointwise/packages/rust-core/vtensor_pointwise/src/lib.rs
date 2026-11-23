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
    C5(u64),
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
    C9(Option<u64>),
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
        Adt0::C1(v13, v14, v15, v16, v17) => {
            f9(v13, v14, v15, v16, v17)
        }
        Adt0::C2(v18) => {
            f10(v18)
        }
        Adt0::C3(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) => {
            Ok(f11(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31))
        }
        Adt0::C4(v8, v9, v10, v11) => {
            f8(v8, v9, v10, v11)
        }
        Adt0::C5(v12) => {
            Ok(Adt8::C9({
                let a21 = 3240099840u64;
                let a22 = v12;
                if if nat_le(a21, a22) {
                    let a23 = v12;
                    let a24 = 4286578688u64;
                    nat_le(a23, a24)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a25 = 1092616192u64;
                    let a26 = v12;
                    if if nat_le(a25, a26) {
                        let a27 = v12;
                        let a28 = 2139095040u64;
                        nat_le(a27, a28)
                    } else {
                        false
                    } {
                        Some(v12)
                    } else {
                        None::<u64>
                    }
                }
            }))
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt9> {
    let v2: u64 = f12(v1.clone(), 1u64)?;
    let a30 = match v1.clone() {
        Adt2::C0(h29, _, _, _, _, _, _, _, _, _, _) => {
            h29
        }
    };
    let a31 = 0u64;
    if if if if if if if if nat_eq(a30, a31) {
        let a33 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h32, _, _, _, _, _) => {
                h32
            }
        };
        let a34 = 4u64;
        nat_eq(a33, a34)
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(h35, _, _, _, _, _, _, _, _, _) => {
                h35
            }
        })
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(_, h36, _, _, _, _, _, _, _, _) => {
                h36
            }
        })
    } else {
        false
    } {
        let a38 = 0u64;
        let a39 = match v0.clone() {
            Adt1::C0(_, _, h37, _, _, _, _, _, _, _) => {
                h37
            }
        };
        nat_lt(a38, a39)
    } else {
        false
    } {
        let a41 = match v0.clone() {
            Adt1::C0(_, _, h40, _, _, _, _, _, _, _) => {
                h40
            }
        };
        let a42 = 2139095040u64;
        nat_lt(a41, a42)
    } else {
        false
    } {
        let a43 = v2;
        let a44 = 9223372036854775808u64;
        nat_lt(a43, a44)
    } else {
        false
    } {
        let a46 = f14(v2)?;
        let a47 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h45, _) => {
                h45
            }
        };
        nat_le(a46, a47)
    } else {
        false
    } {
        Ok(Adt9::C4(f15(v2)?, 4u64, f16(v1.clone(), 1u64), f16(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h48, _, _, _, _, _, _, _, _, _) => {
                h48
            }
        }, match v0.clone() {
            Adt1::C0(_, h49, _, _, _, _, _, _, _, _) => {
                h49
            }
        }, match v0 {
            Adt1::C0(_, _, h50, _, _, _, _, _, _, _) => {
                h50
            }
        }, v2)))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m51 = v2;
    match m51 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h52, _, _, _, _, _, _, _, _, _) => {
                    h52
                }
            }, match v0.clone() {
                Adt1::C0(h53, _, _, _, _, _, _, _, _, _) => {
                    h53
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h54, _, _, _, _, _, _, _) => {
                    h54
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h55, _, _, _, _, _) => {
                    h55
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h56, _, _, _, _, _, _, _, _) => {
                    h56
                }
            }, match v0.clone() {
                Adt1::C0(_, h57, _, _, _, _, _, _, _, _) => {
                    h57
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h58, _, _, _, _, _, _) => {
                    h58
                }
            }, match v0 {
                Adt1::C0(_, _, _, _, _, h59, _, _, _, _) => {
                    h59
                }
            });
            let v7: u64 = f17(v5.clone(), match v4.clone() {
                Adt2::C0(_, h60, _, _, _, _, _, _, _, _, _) => {
                    h60
                }
            })?;
            let v8: u64 = f17(v6.clone(), match v4.clone() {
                Adt2::C0(_, _, h61, _, _, _, _, _, _, _, _) => {
                    h61
                }
            })?;
            let v9: u64 = {
                let a65 = {
                    let a62 = v7;
                    let a63 = v8;
                    nat_mul(a62, a63)?
                };
                let a66 = match v4.clone() {
                    Adt2::C0(_, _, _, _, h64, _, _, _, _, _, _) => {
                        h64
                    }
                };
                nat_mul(a65, a66)?
            };
            let v10: u64 = match v1.clone() {
                Adt2::C0(_, _, _, _, h67, _, _, _, _, _, _) => {
                    h67
                }
            };
            let v11: u64 = {
                let a73 = {
                    let a70 = match v5.clone() {
                        Adt6::C0(h68, _, _, _) => {
                            h68
                        }
                    };
                    let a71 = match v6.clone() {
                        Adt6::C0(h69, _, _, _) => {
                            h69
                        }
                    };
                    nat_mul(a70, a71)?
                };
                let a74 = match v1.clone() {
                    Adt2::C0(_, _, _, h72, _, _, _, _, _, _, _) => {
                        h72
                    }
                };
                nat_mul(a73, a74)?
            };
            let v12: u64 = {
                let a75 = v9;
                let a76 = 32u64;
                if nat_lt(a75, a76) {
                    v9
                } else {
                    32u64
                }
            };
            let a78 = match v1.clone() {
                Adt2::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                    h77
                }
            };
            let a79 = 0u64;
            if if if if if if if if if if if if if if nat_eq(a78, a79) {
                true
            } else {
                let a81 = match v1.clone() {
                    Adt2::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                        h80
                    }
                };
                let a82 = 1u64;
                nat_eq(a81, a82)
            } {
                let a84 = match v4.clone() {
                    Adt2::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                        h83
                    }
                };
                let a85 = 0u64;
                nat_eq(a84, a85)
            } else {
                false
            } {
                let a87 = match v3.clone() {
                    Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                        h86
                    }
                };
                let a88 = 0u64;
                nat_eq(a87, a88)
            } else {
                false
            } {
                let a91 = match v1.clone() {
                    Adt2::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                        h89
                    }
                };
                let a92 = match v4.clone() {
                    Adt2::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                        h90
                    }
                };
                nat_eq(a91, a92)
            } else {
                false
            } {
                let a94 = match v3.clone() {
                    Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                        h93
                    }
                };
                let a95 = v7;
                nat_eq(a94, a95)
            } else {
                false
            } {
                let a97 = match v3.clone() {
                    Adt2::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                        h96
                    }
                };
                let a98 = v8;
                nat_eq(a97, a98)
            } else {
                false
            } {
                let a100 = match v3.clone() {
                    Adt2::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                        h99
                    }
                };
                let a101 = v10;
                nat_eq(a100, a101)
            } else {
                false
            } {
                let a104 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h102, _, _, _, _, _, _) => {
                        h102
                    }
                };
                let a105 = match v4.clone() {
                    Adt2::C0(_, _, _, _, h103, _, _, _, _, _, _) => {
                        h103
                    }
                };
                nat_eq(a104, a105)
            } else {
                false
            } {
                let a106 = 0u64;
                let a107 = v11;
                nat_lt(a106, a107)
            } else {
                false
            } {
                let a108 = 0u64;
                let a109 = v10;
                nat_lt(a108, a109)
            } else {
                false
            } {
                let a110 = 0u64;
                let a111 = v9;
                nat_lt(a110, a111)
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
                    let a112 = v11;
                    let a113 = v10;
                    nat_mul(a112, a113)?
                }, v12, {
                    let a114 = v12;
                    let a115 = v11;
                    nat_mul(a114, a115)?
                }, {
                    let a116 = v12;
                    let a117 = v10;
                    nat_mul(a116, a117)?
                }, match v1.clone() {
                    Adt2::C0(h118, _, _, _, _, _, _, _, _, _, _) => {
                        h118
                    }
                }, f16(v1, 1u64), f16(v4, 1u64), f16(v3, 1u64), v5, v6)))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let m119 = v2;
    match m119 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v5) => {
            let m120 = v3;
            match m120 {
                None => {
                    Ok(Adt9::C1)
                }
                Some(v6) => {
                    let a122 = match v1.clone() {
                        Adt2::C0(h121, _, _, _, _, _, _, _, _, _, _) => {
                            h121
                        }
                    };
                    let a123 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a122, a123) {
                        let a125 = match v5.clone() {
                            Adt2::C0(h124, _, _, _, _, _, _, _, _, _, _) => {
                                h124
                            }
                        };
                        let a126 = 0u64;
                        nat_eq(a125, a126)
                    } else {
                        false
                    } {
                        let a128 = match v6.clone() {
                            Adt2::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                                h127
                            }
                        };
                        let a129 = 0u64;
                        nat_eq(a128, a129)
                    } else {
                        false
                    } {
                        let a131 = match v4.clone() {
                            Adt2::C0(h130, _, _, _, _, _, _, _, _, _, _) => {
                                h130
                            }
                        };
                        let a132 = 0u64;
                        nat_eq(a131, a132)
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
                        let a138 = match v1.clone() {
                            Adt2::C0(_, h133, _, _, _, _, _, _, _, _, _) => {
                                h133
                            }
                        };
                        let a139 = {
                            let a136 = match v5.clone() {
                                Adt2::C0(_, h134, _, _, _, _, _, _, _, _, _) => {
                                    h134
                                }
                            };
                            let a137 = match v5.clone() {
                                Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                                    h135
                                }
                            };
                            nat_mul(a136, a137)?
                        };
                        nat_eq(a138, a139)
                    } else {
                        false
                    } {
                        let a145 = match v1.clone() {
                            Adt2::C0(_, _, h140, _, _, _, _, _, _, _, _) => {
                                h140
                            }
                        };
                        let a146 = {
                            let a143 = match v5.clone() {
                                Adt2::C0(_, _, h141, _, _, _, _, _, _, _, _) => {
                                    h141
                                }
                            };
                            let a144 = match v5.clone() {
                                Adt2::C0(_, _, _, h142, _, _, _, _, _, _, _) => {
                                    h142
                                }
                            };
                            nat_mul(a143, a144)?
                        };
                        nat_eq(a145, a146)
                    } else {
                        false
                    } {
                        let a149 = match v1.clone() {
                            Adt2::C0(_, _, _, h147, _, _, _, _, _, _, _) => {
                                h147
                            }
                        };
                        let a150 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                                h148
                            }
                        };
                        nat_eq(a149, a150)
                    } else {
                        false
                    } {
                        let a152 = match v0.clone() {
                            Adt1::C0(h151, _, _, _, _, _, _, _, _, _) => {
                                h151
                            }
                        };
                        let a153 = 4294967296u64;
                        nat_lt(a152, a153)
                    } else {
                        false
                    } {
                        let a155 = match v0.clone() {
                            Adt1::C0(h154, _, _, _, _, _, _, _, _, _) => {
                                h154
                            }
                        };
                        let a156 = 0u64;
                        if nat_eq(a155, a156) {
                            if f18(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a159 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h157) => {
                                        h157
                                    }
                                };
                                let a160 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h158) => {
                                        h158
                                    }
                                };
                                nat_eq(a159, a160)
                            }
                        } else {
                            let a163 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h161) => {
                                    h161
                                }
                            };
                            let a164 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h162) => {
                                    h162
                                }
                            };
                            nat_eq(a163, a164)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a166 = match v4.clone() {
                                Adt2::C0(_, h165, _, _, _, _, _, _, _, _, _) => {
                                    h165
                                }
                            };
                            let a167 = 0u64;
                            if if if nat_eq(a166, a167) {
                                true
                            } else {
                                let a169 = match v4.clone() {
                                    Adt2::C0(_, _, h168, _, _, _, _, _, _, _, _) => {
                                        h168
                                    }
                                };
                                let a170 = 0u64;
                                nat_eq(a169, a170)
                            } {
                                true
                            } else {
                                let a172 = match v4.clone() {
                                    Adt2::C0(_, _, _, h171, _, _, _, _, _, _, _) => {
                                        h171
                                    }
                                };
                                let a173 = 0u64;
                                nat_eq(a172, a173)
                            } {
                                true
                            } else {
                                let a175 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h174, _, _, _, _, _, _) => {
                                        h174
                                    }
                                };
                                let a176 = 0u64;
                                nat_eq(a175, a176)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h177, _, _, _, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a179 = match v0 {
                                Adt1::C0(h178, _, _, _, _, _, _, _, _, _) => {
                                    h178
                                }
                            };
                            let a180 = 0u64;
                            if nat_eq(a179, a180) {
                                f12(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f21(v8, match v5.clone() {
                                Adt2::C0(_, _, h181, _, _, _, _, _, _, _, _) => {
                                    h181
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h182, _, _, _, _, _, _, _) => {
                                    h182
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h183, _, _, _, _, _, _) => {
                                    h183
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
        Adt2::C0(h184, _, _, _, _, _, _, _, _, _, _) => {
            h184
        }
    };
    let v4: u64 = f23(v3);
    let a186 = match v1.clone() {
        Adt2::C0(h185, _, _, _, _, _, _, _, _, _, _) => {
            h185
        }
    };
    let a187 = 0u64;
    if if if if if if if if nat_eq(a186, a187) {
        true
    } else {
        let a189 = match v1.clone() {
            Adt2::C0(h188, _, _, _, _, _, _, _, _, _, _) => {
                h188
            }
        };
        let a190 = 30u64;
        nat_eq(a189, a190)
    } {
        let a193 = match v1.clone() {
            Adt2::C0(h191, _, _, _, _, _, _, _, _, _, _) => {
                h191
            }
        };
        let a194 = match v2.clone() {
            Adt2::C0(h192, _, _, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        nat_eq(a193, a194)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f24(match v0.clone() {
            Adt1::C0(h195, _, _, _, _, _, _, _, _, _) => {
                h195
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
            Adt2::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h197, _, _, _, _, _, _, _, _) => {
                h197
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h198, _, _, _, _, _, _, _) => {
                h198
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                h199
            }
        }) {
            0u64
        } else {
            let a205 = match v1.clone() {
                Adt2::C0(_, _, h200, _, _, _, _, _, _, _, _) => {
                    h200
                }
            };
            let a206 = {
                let a203 = match v1.clone() {
                    Adt2::C0(_, _, _, h201, _, _, _, _, _, _, _) => {
                        h201
                    }
                };
                let a204 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h202, _, _, _, _, _, _) => {
                        h202
                    }
                };
                nat_mul(a203, a204)?
            };
            nat_mul(a205, a206)?
        };
        let v6: u64 = {
            let a207 = v5;
            let a208 = 0u64;
            if if nat_eq(a207, a208) {
                true
            } else {
                f28(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a210 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h209, _) => {
                        h209
                    }
                };
                let a211 = v4;
                let a212 = 0u64;
                nat_quot(a210, a211, a212)
            }
        };
        Ok(Adt9::C4(f29(v3, match v1.clone() {
            Adt2::C0(_, h213, _, _, _, _, _, _, _, _, _) => {
                h213
            }
        }, v5, v6)?, v4, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h214, _, _, _, _, _, _, _, _, _) => {
                h214
            }
        }, v5, f30(match v1 {
            Adt2::C0(_, h215, _, _, _, _, _, _, _, _, _) => {
                h215
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt9> {
    let a217 = match v1.clone() {
        Adt2::C0(h216, _, _, _, _, _, _, _, _, _, _) => {
            h216
        }
    };
    let a218 = 0u64;
    if if if if if nat_eq(a217, a218) {
        let a220 = match v2.clone() {
            Adt2::C0(h219, _, _, _, _, _, _, _, _, _, _) => {
                h219
            }
        };
        let a221 = 0u64;
        nat_eq(a220, a221)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f31(match v0.clone() {
            Adt1::C0(h222, _, _, _, _, _, _, _, _, _) => {
                h222
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
            let a228 = match v3.clone() {
                Adt3::C0(_, h223, _, _, _, _, _, _) => {
                    h223
                }
            };
            let a229 = {
                let a226 = match v3.clone() {
                    Adt3::C0(_, _, h224, _, _, _, _, _) => {
                        h224
                    }
                };
                let a227 = match v3.clone() {
                    Adt3::C0(_, _, _, h225, _, _, _, _) => {
                        h225
                    }
                };
                nat_mul(a226, a227)?
            };
            nat_mul(a228, a229)?
        };
        Ok(Adt9::C4(f32(match v3.clone() {
            Adt3::C0(h230, _, _, _, _, _, _, _) => {
                h230
            }
        }, v4)?, 4u64, v3, f16(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h231, _, _, _, _, _, _, _, _, _) => {
                h231
            }
        })))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f6(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m232 = v1;
    match m232 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let m233 = v2;
            match m233 {
                None => {
                    Ok(Adt9::C1)
                }
                Some(v5) => {
                    let a235 = match v0.clone() {
                        Adt2::C0(h234, _, _, _, _, _, _, _, _, _, _) => {
                            h234
                        }
                    };
                    let a236 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a235, a236) {
                        let a238 = match v4.clone() {
                            Adt2::C0(h237, _, _, _, _, _, _, _, _, _, _) => {
                                h237
                            }
                        };
                        let a239 = 0u64;
                        nat_eq(a238, a239)
                    } else {
                        false
                    } {
                        let a241 = match v5.clone() {
                            Adt2::C0(h240, _, _, _, _, _, _, _, _, _, _) => {
                                h240
                            }
                        };
                        let a242 = 26u64;
                        nat_eq(a241, a242)
                    } else {
                        false
                    } {
                        let a244 = match v3.clone() {
                            Adt2::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                                h243
                            }
                        };
                        let a245 = 0u64;
                        nat_eq(a244, a245)
                    } else {
                        false
                    } {
                        f19(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a248 = match v0.clone() {
                            Adt2::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                                h246
                            }
                        };
                        let a249 = match v4.clone() {
                            Adt2::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                                h247
                            }
                        };
                        nat_eq(a248, a249)
                    } else {
                        false
                    } {
                        let a252 = match v0.clone() {
                            Adt2::C0(_, _, h250, _, _, _, _, _, _, _, _) => {
                                h250
                            }
                        };
                        let a253 = match v5.clone() {
                            Adt2::C0(_, h251, _, _, _, _, _, _, _, _, _) => {
                                h251
                            }
                        };
                        nat_eq(a252, a253)
                    } else {
                        false
                    } {
                        let a256 = match v0.clone() {
                            Adt2::C0(_, _, _, h254, _, _, _, _, _, _, _) => {
                                h254
                            }
                        };
                        let a257 = match v5.clone() {
                            Adt2::C0(_, _, h255, _, _, _, _, _, _, _, _) => {
                                h255
                            }
                        };
                        nat_eq(a256, a257)
                    } else {
                        false
                    } {
                        let a259 = 0u64;
                        let a260 = match v5.clone() {
                            Adt2::C0(_, _, _, h258, _, _, _, _, _, _, _) => {
                                h258
                            }
                        };
                        nat_lt(a259, a260)
                    } else {
                        false
                    } {
                        let a262 = 0u64;
                        let a263 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h261, _, _, _, _, _, _) => {
                                h261
                            }
                        };
                        nat_lt(a262, a263)
                    } else {
                        false
                    } {
                        let a265 = 4u64;
                        let a266 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h264, _, _, _, _, _) => {
                                h264
                            }
                        };
                        nat_le(a265, a266)
                    } else {
                        false
                    } {
                        let a268 = 4u64;
                        let a269 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h267, _, _, _, _, _) => {
                                h267
                            }
                        };
                        nat_le(a268, a269)
                    } else {
                        false
                    } {
                        let a271 = 4u64;
                        let a272 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h270, _, _, _, _, _) => {
                                h270
                            }
                        };
                        nat_le(a271, a272)
                    } else {
                        false
                    } {
                        let a274 = 4u64;
                        let a275 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h273, _, _, _, _, _) => {
                                h273
                            }
                        };
                        nat_le(a274, a275)
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
                            Adt2::C0(_, h276, _, _, _, _, _, _, _, _, _) => {
                                h276
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h277, _, _, _, _, _, _, _, _) => {
                                h277
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                                h278
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h279, _, _, _, _, _, _) => {
                                h279
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
        Adt1::C0(h280, _, _, _, _, _, _, _, _, _) => {
            h280
        }
    };
    Ok(Adt8::C0({
        let a281 = v0;
        let a282 = 49u64;
        if nat_eq(a281, a282) {
            f34(v1, v2, v3, v4)?
        } else {
            let a283 = v0;
            let a284 = 51u64;
            if nat_eq(a283, a284) {
                f35(v1, v2, v3, v4)?
            } else {
                let a285 = v0;
                let a286 = 39u64;
                if nat_eq(a285, a286) {
                    f36(v2, v3, v4)?
                } else {
                    let a287 = v0;
                    let a288 = 77u64;
                    if nat_eq(a287, a288) {
                        f37(v2, v4)?
                    } else {
                        let a289 = v0;
                        let a290 = 40u64;
                        if nat_eq(a289, a290) {
                            f38(v2, v3, v4)?
                        } else {
                            if f39(v0) {
                                f40(v0, v2, {
                                    let m291 = v3;
                                    match m291 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }, v4)?
                            } else {
                                let a292 = v0;
                                let a293 = 19u64;
                                if nat_eq(a292, a293) {
                                    f41(v2, v4)?
                                } else {
                                    let a294 = v0;
                                    let a295 = 21u64;
                                    if nat_eq(a294, a295) {
                                        f42(v5, v2, v3, v4)?
                                    } else {
                                        let a296 = v0;
                                        let a297 = 61u64;
                                        if if if nat_eq(a296, a297) {
                                            true
                                        } else {
                                            let a298 = v0;
                                            let a299 = 62u64;
                                            nat_eq(a298, a299)
                                        } {
                                            true
                                        } else {
                                            let a300 = v0;
                                            let a301 = 63u64;
                                            nat_eq(a300, a301)
                                        } {
                                            f43(v0, v1, v2, v4)?
                                        } else {
                                            let a302 = v0;
                                            let a303 = 4u64;
                                            if if nat_eq(a302, a303) {
                                                true
                                            } else {
                                                let a304 = v0;
                                                let a305 = 32u64;
                                                nat_eq(a304, a305)
                                            } {
                                                f44(v0, v1, v2, v3, v4)?
                                            } else {
                                                let a306 = v0;
                                                let a307 = 20u64;
                                                if nat_eq(a306, a307) {
                                                    f45(v2, v4)?
                                                } else {
                                                    let a308 = v0;
                                                    let a309 = 42u64;
                                                    if nat_eq(a308, a309) {
                                                        f46(4u64, v2, v4)?
                                                    } else {
                                                        let a310 = v0;
                                                        let a311 = 69u64;
                                                        if nat_eq(a310, a311) {
                                                            let a312 = v5;
                                                            let a313 = 4u64;
                                                            if nat_lt(a312, a313) {
                                                                f46(v5, v2, v4)?
                                                            } else {
                                                                Adt9::C1
                                                            }
                                                        } else {
                                                            let a314 = v0;
                                                            let a315 = 70u64;
                                                            if nat_eq(a314, a315) {
                                                                f47(v5, v2, v4)?
                                                            } else {
                                                                let a316 = v0;
                                                                let a317 = 75u64;
                                                                if if nat_eq(a316, a317) {
                                                                    true
                                                                } else {
                                                                    let a318 = v0;
                                                                    let a319 = 76u64;
                                                                    nat_eq(a318, a319)
                                                                } {
                                                                    f48({
                                                                        let a320 = v0;
                                                                        let a321 = 76u64;
                                                                        nat_eq(a320, a321)
                                                                    }, v1, v2, v4)?
                                                                } else {
                                                                    let a322 = v0;
                                                                    let a323 = 31u64;
                                                                    if if if nat_eq(a322, a323) {
                                                                        true
                                                                    } else {
                                                                        let a324 = v0;
                                                                        let a325 = 47u64;
                                                                        nat_eq(a324, a325)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a326 = v0;
                                                                        let a327 = 68u64;
                                                                        nat_eq(a326, a327)
                                                                    } {
                                                                        f49(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a328 = v0;
                                                                        let a329 = 15u64;
                                                                        if nat_eq(a328, a329) {
                                                                            f50(v2, v4)?
                                                                        } else {
                                                                            let a330 = v0;
                                                                            let a331 = 17u64;
                                                                            if nat_eq(a330, a331) {
                                                                                f51(v2, v4)?
                                                                            } else {
                                                                                let a332 = v0;
                                                                                let a333 = 18u64;
                                                                                if nat_eq(a332, a333) {
                                                                                    f52(v2, v3, v4)?
                                                                                } else {
                                                                                    let a334 = v0;
                                                                                    let a335 = 13u64;
                                                                                    if if if nat_eq(a334, a335) {
                                                                                        true
                                                                                    } else {
                                                                                        let a336 = v0;
                                                                                        let a337 = 14u64;
                                                                                        nat_eq(a336, a337)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a338 = v0;
                                                                                        let a339 = 16u64;
                                                                                        nat_eq(a338, a339)
                                                                                    } {
                                                                                        f53(v0, v2, v4)?
                                                                                    } else {
                                                                                        let m340 = f54(v0, v5)?;
                                                                                        match m340 {
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
    let a341 = v1;
    let a342 = 0u64;
    if nat_eq(a341, a342) {
        let a344 = v3;
        let a345 = match v0.clone() {
            Adt7::C0(_, _, _, h343, _, _, _, _, _, _, _, _, _) => {
                h343
            }
        };
        if nat_lt(a344, a345) {
            let v4: Adt3 = match v0.clone() {
                Adt7::C0(_, _, _, _, _, _, _, _, h346, _, _, _, _) => {
                    h346
                }
            };
            let v5: u64 = {
                let a348 = v3;
                let a349 = match v0.clone() {
                    Adt7::C0(_, _, h347, _, _, _, _, _, _, _, _, _, _) => {
                        h347
                    }
                };
                let a350 = 0u64;
                nat_quot(a348, a349, a350)
            };
            Ok(Adt8::C8(Some(f57(v4.clone(), f58(v5, v4.clone(), 0u64), f58(v5, v4.clone(), 1u64), f58(v5, v4, 2u64), {
                let a352 = v3;
                let a353 = match v0 {
                    Adt7::C0(_, _, h351, _, _, _, _, _, _, _, _, _, _) => {
                        h351
                    }
                };
                let a354 = 0u64;
                nat_rem(a352, a353, a354)
            })?)))
        } else {
            Ok(Adt8::C0(Adt9::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, _, _, h355, _, _) => {
                h355
            }
        };
        let v7: u64 = {
            let a356 = v1;
            let a357 = 1u64;
            if nat_eq(a356, a357) {
                match v0.clone() {
                    Adt7::C0(_, h358, _, _, _, _, _, _, _, _, _, _, _) => {
                        h358
                    }
                }
            } else {
                match v0.clone() {
                    Adt7::C0(_, _, h359, _, _, _, _, _, _, _, _, _, _) => {
                        h359
                    }
                }
            }
        };
        let v8: u64 = {
            let a363 = v2;
            let a364 = {
                let a360 = v3;
                let a361 = v7;
                let a362 = 0u64;
                nat_quot(a360, a361, a362)
            };
            nat_add(a363, a364)?
        };
        let v9: u64 = {
            let a365 = v3;
            let a366 = v7;
            let a367 = 0u64;
            nat_rem(a365, a366, a367)
        };
        let v10: u64 = {
            let a369 = v8;
            let a370 = match v6.clone() {
                Adt3::C0(h368, _, _, _, _, _, _, _) => {
                    h368
                }
            };
            let a371 = 0u64;
            nat_rem(a369, a370, a371)
        };
        let v11: u64 = {
            let a373 = v8;
            let a374 = match v6.clone() {
                Adt3::C0(h372, _, _, _, _, _, _, _) => {
                    h372
                }
            };
            let a375 = 0u64;
            nat_quot(a373, a374, a375)
        };
        let v12: u64 = {
            let a377 = v11;
            let a378 = match v6.clone() {
                Adt3::C0(_, h376, _, _, _, _, _, _) => {
                    h376
                }
            };
            let a379 = 0u64;
            nat_rem(a377, a378, a379)
        };
        let v13: u64 = {
            let a381 = v11;
            let a382 = match v6.clone() {
                Adt3::C0(_, h380, _, _, _, _, _, _) => {
                    h380
                }
            };
            let a383 = 0u64;
            nat_quot(a381, a382, a383)
        };
        let a385 = v8;
        let a386 = match v0.clone() {
            Adt7::C0(h384, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h384
            }
        };
        if nat_lt(a385, a386) {
            let a387 = v1;
            let a388 = 1u64;
            if nat_eq(a387, a388) {
                let v14: Adt3 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, h389, _, _, _, _) => {
                        h389
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, h390, _, _, _) => {
                        h390
                    }
                };
                let m393 = f59(match v0.clone() {
                    Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h391, _) => {
                        h391
                    }
                }, match v15.clone() {
                    Adt3::C0(h392, _, _, _, _, _, _, _) => {
                        h392
                    }
                }, v10, f58(v9, v14.clone(), 0u64))?;
                match m393 {
                    None => {
                        Ok(Adt8::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m396 = f59(match v0 {
                            Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h394) => {
                                h394
                            }
                        }, match v15.clone() {
                            Adt3::C0(_, h395, _, _, _, _, _, _) => {
                                h395
                            }
                        }, v12, f58(v9, v14.clone(), 1u64))?;
                        match m396 {
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
                let a397 = v1;
                let a398 = 2u64;
                if nat_eq(a397, a398) {
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
        let m399 = v3.clone();
        match m399 {
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
                let a400 = v53;
                let a401 = 64u64;
                if if if nat_eq(a400, a401) {
                    true
                } else {
                    let a402 = v53;
                    let a403 = 65u64;
                    nat_eq(a402, a403)
                } {
                    true
                } else {
                    let a404 = v53;
                    let a405 = 66u64;
                    nat_eq(a404, a405)
                } {
                    Adt8::C0(Adt9::C5(64u64))
                } else {
                    let a406 = v53;
                    let a407 = 0u64;
                    if if nat_eq(a406, a407) {
                        true
                    } else {
                        let a408 = v53;
                        let a409 = 30u64;
                        nat_eq(a408, a409)
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
    let m410 = v0;
    match m410 {
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
                let a415 = {
                    let a411 = 0u64;
                    let a412 = v45;
                    if nat_lt(a411, a412) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a416 = {
                    let a413 = 0u64;
                    let a414 = v47;
                    if nat_lt(a413, a414) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a415, a416)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v49, _, _, _) => {
            Ok(Adt8::C6({
                let a417 = v49;
                let a418 = 66u64;
                if nat_eq(a417, a418) {
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
    let a419 = v1;
    let a420 = 5u64;
    if if if if nat_lt(a419, a420) {
        let a421 = v5;
        let a422 = 5u64;
        nat_lt(a421, a422)
    } else {
        false
    } {
        let a423 = v7;
        let a424 = 5u64;
        nat_lt(a423, a424)
    } else {
        false
    } {
        let a425 = v9;
        let a426 = 2u64;
        if nat_lt(a425, a426) {
            true
        } else {
            let a427 = v9;
            let a428 = 3u64;
            nat_eq(a427, a428)
        }
    } else {
        false
    } {
        Adt8::C7(f69(v0, v1, v2, v3, v4), f70(v5, v6), f70(v7, v8), f71({
            let a429 = v9;
            let a430 = 0u64;
            if nat_eq(a429, a430) {
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
    let a442 = {
        let a436 = {
            let a432 = match v0.clone() {
                Adt2::C0(_, h431, _, _, _, _, _, _, _, _, _) => {
                    h431
                }
            };
            let a433 = v1;
            let a434 = 0u64;
            nat_quot(a432, a433, a434)
        };
        let a437 = match v0.clone() {
            Adt2::C0(_, _, h435, _, _, _, _, _, _, _, _) => {
                h435
            }
        };
        nat_mul(a436, a437)?
    };
    let a443 = {
        let a440 = match v0.clone() {
            Adt2::C0(_, _, _, h438, _, _, _, _, _, _, _) => {
                h438
            }
        };
        let a441 = match v0 {
            Adt2::C0(_, _, _, _, h439, _, _, _, _, _, _) => {
                h439
            }
        };
        nat_mul(a440, a441)?
    };
    nat_mul(a442, a443)
}

pub fn f13(v0: u64) -> bool {
    let a444 = v0;
    let a445 = 4294967296u64;
    if nat_lt(a444, a445) {
        let a449 = {
            let a446 = v0;
            let a447 = 2147483648u64;
            let a448 = 0u64;
            nat_rem(a446, a447, a448)
        };
        let a450 = 2139095040u64;
        nat_lt(a449, a450)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a451 = 4u64;
    let a452 = v0;
    nat_mul(a451, a452)
}

pub fn f15(v0: u64) -> R<u64> {
    let a453 = v0;
    let a454 = 0u64;
    if nat_eq(a453, a454) {
        Ok(0u64)
    } else {
        let a457 = 4u64;
        let a458 = {
            let a455 = 3u64;
            let a456 = v0;
            nat_mul(a455, a456)?
        };
        nat_add(a457, a458)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a460 = match v0.clone() {
            Adt2::C0(_, h459, _, _, _, _, _, _, _, _, _) => {
                h459
            }
        };
        let a461 = v1;
        let a462 = 0u64;
        nat_quot(a460, a461, a462)
    }, match v0.clone() {
        Adt2::C0(_, _, h463, _, _, _, _, _, _, _, _) => {
            h463
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h464, _, _, _, _, _, _, _) => {
            h464
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h465, _, _, _, _, _, _) => {
            h465
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h466, _, _, _, _, _) => {
            h466
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h467, _, _, _, _) => {
            h467
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h468, _, _, _) => {
            h468
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h469, _, _) => {
            h469
        }
    })
}

pub fn f17(v0: Adt6, v1: u64) -> R<u64> {
    let a471 = 0u64;
    let a472 = match v0.clone() {
        Adt6::C0(h470, _, _, _) => {
            h470
        }
    };
    if if if if if nat_lt(a471, a472) {
        let a474 = 0u64;
        let a475 = match v0.clone() {
            Adt6::C0(_, h473, _, _) => {
                h473
            }
        };
        nat_lt(a474, a475)
    } else {
        false
    } {
        let a477 = 0u64;
        let a478 = match v0.clone() {
            Adt6::C0(_, _, _, h476) => {
                h476
            }
        };
        nat_lt(a477, a478)
    } else {
        false
    } {
        let a480 = match v0.clone() {
            Adt6::C0(_, h479, _, _) => {
                h479
            }
        };
        let a481 = 2147483648u64;
        nat_lt(a480, a481)
    } else {
        false
    } {
        let a483 = match v0.clone() {
            Adt6::C0(_, _, _, h482) => {
                h482
            }
        };
        let a484 = 2147483648u64;
        nat_lt(a483, a484)
    } else {
        false
    } {
        let v2: u64 = {
            let a488 = v1;
            let a489 = {
                let a486 = 2u64;
                let a487 = f72(match v0.clone() {
                    Adt6::C0(_, _, h485, _) => {
                        h485
                    }
                });
                nat_mul(a486, a487)?
            };
            nat_add(a488, a489)?
        };
        let v3: u64 = {
            let a501 = {
                let a496 = 1u64;
                let a497 = {
                    let a494 = match v0.clone() {
                        Adt6::C0(_, _, _, h490) => {
                            h490
                        }
                    };
                    let a495 = {
                        let a492 = match v0.clone() {
                            Adt6::C0(h491, _, _, _) => {
                                h491
                            }
                        };
                        let a493 = 1u64;
                        nat_sub(a492, a493)
                    };
                    nat_mul(a494, a495)?
                };
                nat_add(a496, a497)?
            };
            let a502 = {
                let a499 = 2u64;
                let a500 = f73(match v0.clone() {
                    Adt6::C0(_, _, h498, _) => {
                        h498
                    }
                });
                nat_mul(a499, a500)?
            };
            nat_add(a501, a502)?
        };
        let a503 = v3;
        let a504 = v2;
        if nat_le(a503, a504) {
            let a511 = 1u64;
            let a512 = {
                let a508 = {
                    let a505 = v2;
                    let a506 = v3;
                    nat_sub(a505, a506)
                };
                let a509 = match v0 {
                    Adt6::C0(_, h507, _, _) => {
                        h507
                    }
                };
                let a510 = 0u64;
                nat_quot(a508, a509, a510)
            };
            nat_add(a511, a512)
        } else {
            let a516 = {
                let a513 = v3;
                let a514 = v2;
                nat_sub(a513, a514)
            };
            let a517 = match v0 {
                Adt6::C0(_, h515, _, _) => {
                    h515
                }
            };
            if nat_lt(a516, a517) {
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
    let a519 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h518, _) => {
            h518
        }
    };
    let a520 = 0u64;
    if nat_eq(a519, a520) {
        Ok(true)
    } else {
        let a522 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h521, _) => {
                h521
            }
        };
        let a523 = 0u64;
        if nat_eq(a522, a523) {
            Ok(true)
        } else {
            let a525 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h524) => {
                    h524
                }
            };
            let a526 = 0u64;
            if nat_eq(a525, a526) {
                Ok(true)
            } else {
                let a528 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h527) => {
                        h527
                    }
                };
                let a529 = 0u64;
                if nat_eq(a528, a529) {
                    Ok(true)
                } else {
                    let a535 = {
                        let a532 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h530) => {
                                h530
                            }
                        };
                        let a533 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h531, _) => {
                                h531
                            }
                        };
                        nat_add(a532, a533)?
                    };
                    let a536 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h534) => {
                            h534
                        }
                    };
                    if nat_le(a535, a536) {
                        Ok(true)
                    } else {
                        let a542 = {
                            let a539 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h537) => {
                                    h537
                                }
                            };
                            let a540 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h538, _) => {
                                    h538
                                }
                            };
                            nat_add(a539, a540)?
                        };
                        let a543 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h541) => {
                                h541
                            }
                        };
                        Ok(nat_le(a542, a543))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a546 = match v0.clone() {
        Adt2::C0(_, h544, _, _, _, _, _, _, _, _, _) => {
            h544
        }
    };
    let a547 = match v1.clone() {
        Adt2::C0(_, h545, _, _, _, _, _, _, _, _, _) => {
            h545
        }
    };
    if if if nat_eq(a546, a547) {
        let a550 = match v0.clone() {
            Adt2::C0(_, _, h548, _, _, _, _, _, _, _, _) => {
                h548
            }
        };
        let a551 = match v1.clone() {
            Adt2::C0(_, _, h549, _, _, _, _, _, _, _, _) => {
                h549
            }
        };
        nat_eq(a550, a551)
    } else {
        false
    } {
        let a554 = match v0.clone() {
            Adt2::C0(_, _, _, h552, _, _, _, _, _, _, _) => {
                h552
            }
        };
        let a555 = match v1.clone() {
            Adt2::C0(_, _, _, h553, _, _, _, _, _, _, _) => {
                h553
            }
        };
        nat_eq(a554, a555)
    } else {
        false
    } {
        let a558 = match v0 {
            Adt2::C0(_, _, _, _, h556, _, _, _, _, _, _) => {
                h556
            }
        };
        let a559 = match v1 {
            Adt2::C0(_, _, _, _, h557, _, _, _, _, _, _) => {
                h557
            }
        };
        nat_eq(a558, a559)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a561 = match v0.clone() {
        Adt2::C0(_, h560, _, _, _, _, _, _, _, _, _) => {
            h560
        }
    };
    let a562 = 1u64;
    if if if if nat_eq(a561, a562) {
        true
    } else {
        let a564 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h563, _, _, _, _, _) => {
                h563
            }
        };
        let a565 = 4u64;
        nat_eq(a564, a565)
    } {
        let a567 = match v0.clone() {
            Adt2::C0(_, _, h566, _, _, _, _, _, _, _, _) => {
                h566
            }
        };
        let a568 = 1u64;
        if nat_eq(a567, a568) {
            true
        } else {
            let a573 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h569, _, _, _, _) => {
                    h569
                }
            };
            let a574 = {
                let a571 = match v0.clone() {
                    Adt2::C0(_, h570, _, _, _, _, _, _, _, _, _) => {
                        h570
                    }
                };
                let a572 = 4u64;
                nat_mul(a571, a572)?
            };
            nat_eq(a573, a574)
        }
    } else {
        false
    } {
        let a576 = match v0.clone() {
            Adt2::C0(_, _, _, h575, _, _, _, _, _, _, _) => {
                h575
            }
        };
        let a577 = 1u64;
        if nat_eq(a576, a577) {
            true
        } else {
            let a585 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h578, _, _, _) => {
                    h578
                }
            };
            let a586 = {
                let a583 = {
                    let a581 = match v0.clone() {
                        Adt2::C0(_, h579, _, _, _, _, _, _, _, _, _) => {
                            h579
                        }
                    };
                    let a582 = match v0.clone() {
                        Adt2::C0(_, _, h580, _, _, _, _, _, _, _, _) => {
                            h580
                        }
                    };
                    nat_mul(a581, a582)?
                };
                let a584 = 4u64;
                nat_mul(a583, a584)?
            };
            nat_eq(a585, a586)
        }
    } else {
        false
    } {
        let a588 = match v0.clone() {
            Adt2::C0(_, _, _, _, h587, _, _, _, _, _, _) => {
                h587
            }
        };
        let a589 = 1u64;
        if nat_eq(a588, a589) {
            Ok(true)
        } else {
            let a600 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h590, _, _) => {
                    h590
                }
            };
            let a601 = {
                let a598 = {
                    let a596 = {
                        let a593 = match v0.clone() {
                            Adt2::C0(_, h591, _, _, _, _, _, _, _, _, _) => {
                                h591
                            }
                        };
                        let a594 = match v0.clone() {
                            Adt2::C0(_, _, h592, _, _, _, _, _, _, _, _) => {
                                h592
                            }
                        };
                        nat_mul(a593, a594)?
                    };
                    let a597 = match v0 {
                        Adt2::C0(_, _, _, h595, _, _, _, _, _, _, _) => {
                            h595
                        }
                    };
                    nat_mul(a596, a597)?
                };
                let a599 = 4u64;
                nat_mul(a598, a599)?
            };
            Ok(nat_eq(a600, a601))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a606 = {
        let a602 = v0;
        let a603 = v1;
        nat_mul(a602, a603)?
    };
    let a607 = {
        let a604 = v2;
        let a605 = f74(v3);
        nat_mul(a604, a605)?
    };
    nat_mul(a606, a607)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a610 = v0;
    let a611 = {
        let a608 = f75(v1)?;
        let a609 = v2;
        nat_mul(a608, a609)?
    };
    nat_add(a610, a611)
}

pub fn f23(v0: u64) -> u64 {
    let a612 = v0;
    let a613 = 30u64;
    if nat_eq(a612, a613) {
        2u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64) -> bool {
    let a614 = v0;
    let a615 = 2139095040u64;
    if nat_le(a614, a615) {
        true
    } else {
        let a616 = v0;
        let a617 = 2147483648u64;
        nat_eq(a616, a617)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> bool {
    let a619 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h618, _, _, _, _, _) => {
            h618
        }
    };
    let a620 = v1;
    if if nat_eq(a619, a620) {
        let a625 = {
            let a622 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h621) => {
                    h621
                }
            };
            let a623 = v1;
            let a624 = 0u64;
            nat_rem(a622, a623, a624)
        };
        let a626 = 0u64;
        nat_eq(a625, a626)
    } else {
        false
    } {
        f76(match v0.clone() {
            Adt2::C0(_, _, h627, _, _, _, _, _, _, _, _) => {
                h627
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h628, _, _, _, _, _, _, _) => {
                h628
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h629, _, _, _, _, _, _) => {
                h629
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h630, _, _, _, _) => {
                h630
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h631, _, _, _) => {
                h631
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h632, _, _) => {
                h632
            }
        }, v1)
    } else {
        false
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a635 = match v0.clone() {
            Adt2::C0(h633, _, _, _, _, _, _, _, _, _, _) => {
                h633
            }
        };
        let a636 = match v1.clone() {
            Adt2::C0(h634, _, _, _, _, _, _, _, _, _, _) => {
                h634
            }
        };
        nat_eq(a635, a636)
    } else {
        false
    } {
        let a639 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h637) => {
                h637
            }
        };
        let a640 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h638) => {
                h638
            }
        };
        nat_eq(a639, a640)
    } else {
        false
    } {
        let a643 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h641, _, _, _, _, _) => {
                h641
            }
        };
        let a644 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h642, _, _, _, _, _) => {
                h642
            }
        };
        nat_eq(a643, a644)
    } else {
        false
    } {
        let a647 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h645, _, _, _, _) => {
                h645
            }
        };
        let a648 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h646, _, _, _, _) => {
                h646
            }
        };
        nat_eq(a647, a648)
    } else {
        false
    } {
        let a651 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h649, _, _, _) => {
                h649
            }
        };
        let a652 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h650, _, _, _) => {
                h650
            }
        };
        nat_eq(a651, a652)
    } else {
        false
    } {
        let a655 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h653, _, _) => {
                h653
            }
        };
        let a656 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h654, _, _) => {
                h654
            }
        };
        nat_eq(a655, a656)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a657 = v0;
    let a658 = 0u64;
    if if if nat_eq(a657, a658) {
        true
    } else {
        let a659 = v1;
        let a660 = 0u64;
        nat_eq(a659, a660)
    } {
        true
    } else {
        let a661 = v2;
        let a662 = 0u64;
        nat_eq(a661, a662)
    } {
        true
    } else {
        let a663 = v3;
        let a664 = 0u64;
        nat_eq(a663, a664)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<bool> {
    f77({
        let a666 = match v0.clone() {
            Adt2::C0(_, h665, _, _, _, _, _, _, _, _, _) => {
                h665
            }
        };
        let a667 = v1;
        nat_mul(a666, a667)?
    }, match v0.clone() {
        Adt2::C0(_, _, h668, _, _, _, _, _, _, _, _) => {
            h668
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h669, _, _, _, _, _, _, _) => {
            h669
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h670, _, _, _, _, _, _) => {
            h670
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h671, _, _, _, _) => {
            h671
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h672, _, _, _) => {
            h672
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h673, _, _) => {
            h673
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a674 = v1;
    let a675 = 0u64;
    if if nat_eq(a674, a675) {
        true
    } else {
        let a676 = v2;
        let a677 = 0u64;
        nat_eq(a676, a677)
    } {
        Ok(0u64)
    } else {
        let a684 = {
            let a678 = f78(v1, v2, v3)?;
            let a679 = f79(v0, v1, v2)?;
            nat_add(a678, a679)?
        };
        let a685 = {
            let a682 = f80(v0);
            let a683 = {
                let a680 = v1;
                let a681 = v2;
                nat_mul(a680, a681)?
            };
            nat_mul(a682, a683)?
        };
        nat_add(a684, a685)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a686 = v0;
    let a687 = 0u64;
    if if nat_eq(a686, a687) {
        true
    } else {
        let a688 = v1;
        let a689 = 0u64;
        nat_eq(a688, a689)
    } {
        Ok(3u64)
    } else {
        let a692 = {
            let a690 = 3u64;
            let a691 = v1;
            nat_add(a690, a691)?
        };
        let a693 = v2;
        nat_add(a692, a693)
    }
}

pub fn f31(v0: u64) -> bool {
    let a694 = v0;
    let a695 = 2139095040u64;
    if nat_le(a694, a695) {
        true
    } else {
        let a696 = v0;
        let a697 = 2147483648u64;
        nat_eq(a696, a697)
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a698 = v0;
    let a699 = 0u64;
    if nat_eq(a698, a699) {
        Ok(0u64)
    } else {
        let a700 = v1;
        let a701 = f81(v0)?;
        nat_mul(a700, a701)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a702 = v0;
    let a703 = 0u64;
    if if if if nat_eq(a702, a703) {
        true
    } else {
        let a704 = v1;
        let a705 = 0u64;
        nat_eq(a704, a705)
    } {
        true
    } else {
        let a706 = v2;
        let a707 = 0u64;
        nat_eq(a706, a707)
    } {
        true
    } else {
        let a708 = v3;
        let a709 = 0u64;
        nat_eq(a708, a709)
    } {
        Ok(0u64)
    } else {
        let a722 = {
            let a712 = 2u64;
            let a713 = {
                let a710 = v1;
                let a711 = v2;
                nat_mul(a710, a711)?
            };
            nat_mul(a712, a713)?
        };
        let a723 = {
            let a720 = {
                let a716 = {
                    let a714 = v1;
                    let a715 = v2;
                    nat_mul(a714, a715)?
                };
                let a717 = v3;
                nat_mul(a716, a717)?
            };
            let a721 = {
                let a718 = 1u64;
                let a719 = v0;
                nat_add(a718, a719)?
            };
            nat_mul(a720, a721)?
        };
        nat_add(a722, a723)
    }
}

pub fn f34(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt9> {
    let m724 = v2;
    match m724 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a726 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h725, _, _, _) => {
                        h725
                    }
                };
                let a727 = 1u64;
                nat_eq(a726, a727)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h728, _, _, _, _, _, _, _, _, _) => {
                    h728
                }
            }, match v0.clone() {
                Adt1::C0(h729, _, _, _, _, _, _, _, _, _) => {
                    h729
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h730, _, _, _, _, _, _, _) => {
                    h730
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h731, _, _, _, _, _) => {
                    h731
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h732, _, _, _, _, _, _, _, _) => {
                        h732
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h733, _, _, _, _, _, _, _, _) => {
                        h733
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h734, _, _, _, _, _, _) => {
                        h734
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h735, _, _, _, _) => {
                        h735
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h736, _, _, _, _, _, _, _) => {
                        h736
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h737, _, _, _, _, _, _, _, _) => {
                        h737
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h738, _, _, _, _, _, _) => {
                        h738
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h739, _, _, _, _, _, _, _) => {
                        h739
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h740, _, _, _, _, _, _, _, _) => {
                        h740
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h741, _, _, _, _, _, _, _, _, _, _) => {
                    h741
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
                    h742
                }
            };
            let v14: u64 = {
                let a743 = v12;
                let a744 = 0u64;
                if nat_eq(a743, a744) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a746 = match v4.clone() {
                    Adt2::C0(_, h745, _, _, _, _, _, _, _, _, _) => {
                        h745
                    }
                };
                let a747 = v14;
                nat_mul(a746, a747)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h748, _, _, _) => {
                        h748
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h749, _, _, _, _) => {
                        h749
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h750, _, _) => {
                        h750
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h751, _, _, _) => {
                        h751
                    }
                }
            };
            let a753 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h752, _, _, _) => {
                    h752
                }
            };
            let a754 = 1u64;
            if if if if if if if if if if nat_le(a753, a754) {
                let a755 = v12;
                let a756 = 0u64;
                if nat_eq(a755, a756) {
                    true
                } else {
                    let a757 = v12;
                    let a758 = 1u64;
                    if nat_eq(a757, a758) {
                        let a759 = v13;
                        let a760 = 1u64;
                        nat_eq(a759, a760)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a761 = v13;
                let a762 = 0u64;
                if nat_eq(a761, a762) {
                    true
                } else {
                    let a763 = v13;
                    let a764 = 1u64;
                    nat_eq(a763, a764)
                }
            } else {
                false
            } {
                let a766 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h765, _, _, _, _, _) => {
                        h765
                    }
                };
                let a767 = v14;
                nat_eq(a766, a767)
            } else {
                false
            } {
                let a775 = match v3.clone() {
                    Adt2::C0(_, h768, _, _, _, _, _, _, _, _, _) => {
                        h768
                    }
                };
                let a776 = {
                    let a773 = v9;
                    let a774 = {
                        let a771 = match v6.clone() {
                            Adt6::C0(h769, _, _, _) => {
                                h769
                            }
                        };
                        let a772 = match v7.clone() {
                            Adt6::C0(h770, _, _, _) => {
                                h770
                            }
                        };
                        nat_mul(a771, a772)?
                    };
                    nat_mul(a773, a774)?
                };
                nat_eq(a775, a776)
            } else {
                false
            } {
                let a779 = match v3.clone() {
                    Adt2::C0(_, _, h777, _, _, _, _, _, _, _, _) => {
                        h777
                    }
                };
                let a780 = f17(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h778, _, _, _, _, _, _, _, _, _) => {
                        h778
                    }
                })?;
                nat_eq(a779, a780)
            } else {
                false
            } {
                let a782 = 0u64;
                let a783 = match v3.clone() {
                    Adt2::C0(_, _, h781, _, _, _, _, _, _, _, _) => {
                        h781
                    }
                };
                nat_lt(a782, a783)
            } else {
                false
            } {
                if v5 {
                    let a785 = match v1 {
                        Adt2::C0(_, _, _, h784, _, _, _, _, _, _, _) => {
                            h784
                        }
                    };
                    let a786 = v9;
                    if if if nat_eq(a785, a786) {
                        let a788 = match v3.clone() {
                            Adt2::C0(_, _, _, h787, _, _, _, _, _, _, _) => {
                                h787
                            }
                        };
                        let a789 = f17(v7.clone(), v11)?;
                        nat_eq(a788, a789)
                    } else {
                        false
                    } {
                        let a791 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h790, _, _, _, _, _, _) => {
                                h790
                            }
                        };
                        let a792 = v10;
                        nat_eq(a791, a792)
                    } else {
                        false
                    } {
                        let a794 = 0u64;
                        let a795 = match v3.clone() {
                            Adt2::C0(_, _, _, h793, _, _, _, _, _, _, _) => {
                                h793
                            }
                        };
                        nat_lt(a794, a795)
                    } else {
                        false
                    }
                } else {
                    let a797 = match v1 {
                        Adt2::C0(_, _, h796, _, _, _, _, _, _, _, _) => {
                            h796
                        }
                    };
                    let a798 = v9;
                    if if if nat_eq(a797, a798) {
                        let a800 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h799, _, _, _, _, _, _) => {
                                h799
                            }
                        };
                        let a801 = 1u64;
                        nat_eq(a800, a801)
                    } else {
                        false
                    } {
                        let a803 = match v3.clone() {
                            Adt2::C0(_, _, _, h802, _, _, _, _, _, _, _) => {
                                h802
                            }
                        };
                        let a804 = v10;
                        nat_eq(a803, a804)
                    } else {
                        false
                    } {
                        let a806 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h805, _, _, _, _, _, _) => {
                                h805
                            }
                        };
                        let a807 = 1u64;
                        nat_eq(a806, a807)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f27(match v4.clone() {
                    Adt2::C0(_, h808, _, _, _, _, _, _, _, _, _) => {
                        h808
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a828 = {
                        let a825 = {
                            let a823 = {
                                let a817 = {
                                    let a811 = {
                                        let a809 = v10;
                                        let a810 = 1u64;
                                        nat_sub(a809, a810)
                                    };
                                    let a812 = v17;
                                    nat_mul(a811, a812)?
                                };
                                let a818 = {
                                    let a815 = {
                                        let a813 = v9;
                                        let a814 = 1u64;
                                        nat_sub(a813, a814)
                                    };
                                    let a816 = v16;
                                    nat_mul(a815, a816)?
                                };
                                nat_add(a817, a818)?
                            };
                            let a824 = {
                                let a821 = {
                                    let a819 = v11;
                                    let a820 = 1u64;
                                    nat_sub(a819, a820)
                                };
                                let a822 = v15;
                                nat_mul(a821, a822)?
                            };
                            nat_add(a823, a824)?
                        };
                        let a826 = v15;
                        nat_add(a825, a826)?
                    };
                    let a829 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h827, _) => {
                            h827
                        }
                    };
                    nat_le(a828, a829)
                }
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt9::C4(f12(v3.clone(), 1u64)?, {
                    let a830 = v13;
                    let a831 = 0u64;
                    if nat_eq(a830, a831) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h832, _, _, _, _, _, _, _, _, _) => {
                        h832
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f16(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h833, _, _, _, _, _, _, _, _, _) => {
                            h833
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h834, _, _, _, _, _, _, _, _) => {
                            h834
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h835, _, _, _, _, _, _, _) => {
                            h835
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h836, _, _, _, _, _) => {
                            h836
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h837, _, _, _, _) => {
                            h837
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h838, _, _, _) => {
                            h838
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
    let m839 = v2;
    match m839 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h840, _, _, _, _, _, _, _, _, _) => {
                    h840
                }
            }, match v0.clone() {
                Adt1::C0(h841, _, _, _, _, _, _, _, _, _) => {
                    h841
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h842, _, _, _, _, _, _) => {
                    h842
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h843, _, _, _) => {
                    h843
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h844, _, _, _, _, _, _, _, _) => {
                    h844
                }
            }, match v0.clone() {
                Adt1::C0(_, h845, _, _, _, _, _, _, _, _) => {
                    h845
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h846, _, _, _, _, _) => {
                    h846
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h847, _, _) => {
                    h847
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                    h848
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h849, _, _, _, _, _, _, _) => {
                    h849
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h850, _, _, _, _) => {
                    h850
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h851, _) => {
                    h851
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h852) => {
                    h852
                }
            };
            let v9: u64 = f17(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h853, _, _, _, _, _, _, _) => {
                    h853
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h854, _, _, _, _, _, _, _, _, _, _) => {
                    h854
                }
            };
            let v11: u64 = {
                let a862 = v8;
                let a863 = {
                    let a860 = match v5.clone() {
                        Adt6::C0(h855, _, _, _) => {
                            h855
                        }
                    };
                    let a861 = {
                        let a858 = match v6.clone() {
                            Adt6::C0(h856, _, _, _) => {
                                h856
                            }
                        };
                        let a859 = match v7.clone() {
                            Adt6::C0(h857, _, _, _) => {
                                h857
                            }
                        };
                        nat_mul(a858, a859)?
                    };
                    nat_mul(a860, a861)?
                };
                nat_mul(a862, a863)?
            };
            let a865 = match v4.clone() {
                Adt2::C0(h864, _, _, _, _, _, _, _, _, _, _) => {
                    h864
                }
            };
            let a866 = 0u64;
            if if if if if if if if if if if if if nat_eq(a865, a866) {
                let a867 = v10;
                let a868 = 0u64;
                if nat_eq(a867, a868) {
                    true
                } else {
                    let a869 = v10;
                    let a870 = 1u64;
                    nat_eq(a869, a870)
                }
            } else {
                false
            } {
                let a871 = 0u64;
                let a872 = v8;
                nat_lt(a871, a872)
            } else {
                false
            } {
                let a873 = v8;
                let a874 = 2147483648u64;
                nat_lt(a873, a874)
            } else {
                false
            } {
                let a876 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h875, _, _, _, _, _) => {
                        h875
                    }
                };
                let a877 = 4u64;
                nat_eq(a876, a877)
            } else {
                false
            } {
                let a878 = 0u64;
                let a879 = v9;
                nat_lt(a878, a879)
            } else {
                false
            } {
                let a881 = 0u64;
                let a882 = match v3.clone() {
                    Adt2::C0(_, _, h880, _, _, _, _, _, _, _, _) => {
                        h880
                    }
                };
                nat_lt(a881, a882)
            } else {
                false
            } {
                let a884 = 0u64;
                let a885 = match v3.clone() {
                    Adt2::C0(_, _, _, h883, _, _, _, _, _, _, _) => {
                        h883
                    }
                };
                nat_lt(a884, a885)
            } else {
                false
            } {
                let a887 = match v3.clone() {
                    Adt2::C0(_, h886, _, _, _, _, _, _, _, _, _) => {
                        h886
                    }
                };
                let a888 = v11;
                nat_eq(a887, a888)
            } else {
                false
            } {
                let a891 = match v3.clone() {
                    Adt2::C0(_, _, h889, _, _, _, _, _, _, _, _) => {
                        h889
                    }
                };
                let a892 = f17(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h890, _, _, _, _, _, _, _, _, _) => {
                        h890
                    }
                })?;
                nat_eq(a891, a892)
            } else {
                false
            } {
                let a895 = match v3.clone() {
                    Adt2::C0(_, _, _, h893, _, _, _, _, _, _, _) => {
                        h893
                    }
                };
                let a896 = f17(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h894, _, _, _, _, _, _, _, _) => {
                        h894
                    }
                })?;
                nat_eq(a895, a896)
            } else {
                false
            } {
                let a904 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h897, _, _, _, _, _, _) => {
                        h897
                    }
                };
                let a905 = {
                    let a902 = v9;
                    let a903 = {
                        let a899 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h898, _, _, _, _, _, _) => {
                                h898
                            }
                        };
                        let a900 = v8;
                        let a901 = 0u64;
                        nat_quot(a899, a900, a901)
                    };
                    nat_mul(a902, a903)?
                };
                nat_eq(a904, a905)
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt9::C4(f12(v3.clone(), 1u64)?, {
                    let a906 = v10;
                    let a907 = 0u64;
                    if nat_eq(a906, a907) {
                        4u64
                    } else {
                        2u64
                    }
                }, f16(v4.clone(), 1u64), f16(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h908, _, _) => {
                        h908
                    }
                }, {
                    let a910 = v8;
                    let a911 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h909, _, _) => {
                            h909
                        }
                    };
                    nat_mul(a910, a911)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f36(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt9> {
    let m912 = v1;
    match m912 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v3) => {
            f82(v0, v3, v2)
        }
    }
}

pub fn f37(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a914 = match v0.clone() {
        Adt2::C0(h913, _, _, _, _, _, _, _, _, _, _) => {
            h913
        }
    };
    let a915 = 1u64;
    if if nat_eq(a914, a915) {
        true
    } else {
        let a917 = match v0.clone() {
            Adt2::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a918 = 30u64;
        nat_eq(a917, a918)
    } {
        let a920 = match v1.clone() {
            Adt2::C0(h919, _, _, _, _, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = 1u64;
        if if if if if if if if if if if nat_eq(a920, a921) {
            let a924 = match v0.clone() {
                Adt2::C0(_, h922, _, _, _, _, _, _, _, _, _) => {
                    h922
                }
            };
            let a925 = match v1.clone() {
                Adt2::C0(_, h923, _, _, _, _, _, _, _, _, _) => {
                    h923
                }
            };
            nat_eq(a924, a925)
        } else {
            false
        } {
            let a927 = 0u64;
            let a928 = match v1.clone() {
                Adt2::C0(_, _, h926, _, _, _, _, _, _, _, _) => {
                    h926
                }
            };
            nat_lt(a927, a928)
        } else {
            false
        } {
            let a931 = match v1.clone() {
                Adt2::C0(_, _, h929, _, _, _, _, _, _, _, _) => {
                    h929
                }
            };
            let a932 = match v1.clone() {
                Adt2::C0(_, _, _, h930, _, _, _, _, _, _, _) => {
                    h930
                }
            };
            nat_eq(a931, a932)
        } else {
            false
        } {
            let a934 = match v1.clone() {
                Adt2::C0(_, _, _, _, h933, _, _, _, _, _, _) => {
                    h933
                }
            };
            let a935 = 1u64;
            nat_eq(a934, a935)
        } else {
            false
        } {
            let a943 = match v0.clone() {
                Adt2::C0(_, _, h936, _, _, _, _, _, _, _, _) => {
                    h936
                }
            };
            let a944 = {
                let a941 = {
                    let a939 = match v1.clone() {
                        Adt2::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                            h937
                        }
                    };
                    let a940 = match v1.clone() {
                        Adt2::C0(_, _, h938, _, _, _, _, _, _, _, _) => {
                            h938
                        }
                    };
                    nat_add(a939, a940)?
                };
                let a942 = 1u64;
                nat_sub(a941, a942)
            };
            nat_eq(a943, a944)
        } else {
            false
        } {
            let a946 = 0u64;
            let a947 = match v0.clone() {
                Adt2::C0(_, _, _, h945, _, _, _, _, _, _, _) => {
                    h945
                }
            };
            nat_lt(a946, a947)
        } else {
            false
        } {
            let a949 = 0u64;
            let a950 = match v0.clone() {
                Adt2::C0(_, _, _, _, h948, _, _, _, _, _, _) => {
                    h948
                }
            };
            nat_lt(a949, a950)
        } else {
            false
        } {
            let a952 = 2u64;
            let a953 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h951, _, _, _, _, _) => {
                    h951
                }
            };
            nat_le(a952, a953)
        } else {
            false
        } {
            let a955 = 2u64;
            let a956 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h954, _, _, _, _, _) => {
                    h954
                }
            };
            nat_le(a955, a956)
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
    let m957 = v1;
    match m957 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v3) => {
            f83(v0, v3, v2)
        }
    }
}

pub fn f39(v0: u64) -> bool {
    let a958 = v0;
    let a959 = 1u64;
    if if nat_eq(a958, a959) {
        true
    } else {
        let a960 = v0;
        let a961 = 33u64;
        nat_eq(a960, a961)
    } {
        true
    } else {
        let a962 = v0;
        let a963 = 34u64;
        nat_eq(a962, a963)
    }
}

pub fn f40(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt9> {
    let a966 = match v1.clone() {
        Adt2::C0(h964, _, _, _, _, _, _, _, _, _, _) => {
            h964
        }
    };
    let a967 = match v3.clone() {
        Adt2::C0(h965, _, _, _, _, _, _, _, _, _, _) => {
            h965
        }
    };
    if nat_eq(a966, a967) {
        f84(v0, v1, v2, v3)
    } else {
        if if if if f85(match v1.clone() {
            Adt2::C0(h968, _, _, _, _, _, _, _, _, _, _) => {
                h968
            }
        }) {
            f85(match v3.clone() {
                Adt2::C0(h969, _, _, _, _, _, _, _, _, _, _) => {
                    h969
                }
            })
        } else {
            false
        } {
            let a970 = v0;
            let a971 = 33u64;
            if nat_eq(a970, a971) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a972 = f12(v1.clone(), 1u64)?;
            let a973 = f12(v3.clone(), 1u64)?;
            nat_eq(a972, a973)
        } else {
            false
        } {
            let m975 = f86(match v3.clone() {
                Adt2::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
                    h974
                }
            });
            match m975 {
                Adt13::C0 => {
                    Ok(Adt9::C0)
                }
                Adt13::C1(v4) => {
                    Ok(Adt9::C4(f12(v3.clone(), 1u64)?, match v4 {
                        Adt14::C0(_, _, h976) => {
                            h976
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h977, _, _, _, _, _, _, _, _, _, _) => {
                            h977
                        }
                    }, match v3 {
                        Adt2::C0(h978, _, _, _, _, _, _, _, _, _, _) => {
                            h978
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
    let a980 = match v0.clone() {
        Adt2::C0(h979, _, _, _, _, _, _, _, _, _, _) => {
            h979
        }
    };
    let a981 = 0u64;
    if if if if nat_eq(a980, a981) {
        true
    } else {
        let a983 = match v0.clone() {
            Adt2::C0(h982, _, _, _, _, _, _, _, _, _, _) => {
                h982
            }
        };
        let a984 = 1u64;
        nat_eq(a983, a984)
    } {
        true
    } else {
        let a986 = match v0.clone() {
            Adt2::C0(h985, _, _, _, _, _, _, _, _, _, _) => {
                h985
            }
        };
        let a987 = 25u64;
        if nat_eq(a986, a987) {
            true
        } else {
            let a989 = match v0.clone() {
                Adt2::C0(h988, _, _, _, _, _, _, _, _, _, _) => {
                    h988
                }
            };
            let a990 = 26u64;
            nat_eq(a989, a990)
        }
    } {
        true
    } else {
        let a992 = match v0.clone() {
            Adt2::C0(h991, _, _, _, _, _, _, _, _, _, _) => {
                h991
            }
        };
        let a993 = 30u64;
        nat_eq(a992, a993)
    } {
        let a996 = match v0.clone() {
            Adt2::C0(h994, _, _, _, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a997 = match v1.clone() {
            Adt2::C0(h995, _, _, _, _, _, _, _, _, _, _) => {
                h995
            }
        };
        if if nat_eq(a996, a997) {
            f87(match v1.clone() {
                Adt2::C0(_, h998, _, _, _, _, _, _, _, _, _) => {
                    h998
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h999, _, _, _, _, _, _, _, _) => {
                    h999
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1000, _, _, _, _, _, _, _) => {
                    h1000
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1001, _, _, _, _, _, _) => {
                    h1001
                }
            }, match v0.clone() {
                Adt2::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
                    h1002
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1003, _, _, _, _, _, _, _, _) => {
                    h1003
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1004, _, _, _, _, _, _, _) => {
                    h1004
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1005, _, _, _, _, _, _) => {
                    h1005
                }
            })
        } else {
            false
        } {
            let m1007 = f86(match v0.clone() {
                Adt2::C0(h1006, _, _, _, _, _, _, _, _, _, _) => {
                    h1006
                }
            });
            match m1007 {
                Adt13::C0 => {
                    Ok(Adt9::C0)
                }
                Adt13::C1(v2) => {
                    Ok(Adt9::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt14::C0(_, _, h1008) => {
                            h1008
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
    let m1009 = v2;
    match m1009 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v4) => {
            let a1010 = v0;
            let a1011 = 4u64;
            if if if nat_lt(a1010, a1011) {
                let a1014 = match v1.clone() {
                    Adt2::C0(h1012, _, _, _, _, _, _, _, _, _, _) => {
                        h1012
                    }
                };
                let a1015 = match v4.clone() {
                    Adt2::C0(h1013, _, _, _, _, _, _, _, _, _, _) => {
                        h1013
                    }
                };
                if nat_eq(a1014, a1015) {
                    let a1018 = match v1.clone() {
                        Adt2::C0(h1016, _, _, _, _, _, _, _, _, _, _) => {
                            h1016
                        }
                    };
                    let a1019 = match v3.clone() {
                        Adt2::C0(h1017, _, _, _, _, _, _, _, _, _, _) => {
                            h1017
                        }
                    };
                    nat_eq(a1018, a1019)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f88(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1020, _, _, _, _, _, _, _, _, _) => {
                        h1020
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1021, _, _, _, _, _, _, _, _, _) => {
                        h1021
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1022, _, _, _, _, _, _, _, _, _) => {
                        h1022
                    }
                })? {
                    f88(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1023, _, _, _, _, _, _, _, _) => {
                            h1023
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1024, _, _, _, _, _, _, _, _) => {
                            h1024
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1025, _, _, _, _, _, _, _, _) => {
                            h1025
                        }
                    })?
                } else {
                    false
                } {
                    f88(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1026, _, _, _, _, _, _, _) => {
                            h1026
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1027, _, _, _, _, _, _, _) => {
                            h1027
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1028, _, _, _, _, _, _, _) => {
                            h1028
                        }
                    })?
                } else {
                    false
                } {
                    f88(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1029, _, _, _, _, _, _) => {
                            h1029
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1030, _, _, _, _, _, _) => {
                            h1030
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1031, _, _, _, _, _, _) => {
                            h1031
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1033 = f86(match v1.clone() {
                    Adt2::C0(h1032, _, _, _, _, _, _, _, _, _, _) => {
                        h1032
                    }
                });
                match m1033 {
                    Adt13::C0 => {
                        Ok(Adt9::C0)
                    }
                    Adt13::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt14::C0(_, h1034, _) => {
                                h1034
                            }
                        };
                        let a1039 = {
                            let a1036 = match v1.clone() {
                                Adt2::C0(_, h1035, _, _, _, _, _, _, _, _, _) => {
                                    h1035
                                }
                            };
                            let a1037 = v6;
                            let a1038 = 1u64;
                            nat_rem(a1036, a1037, a1038)
                        };
                        let a1040 = 0u64;
                        if if if nat_eq(a1039, a1040) {
                            let a1045 = {
                                let a1042 = match v4.clone() {
                                    Adt2::C0(_, h1041, _, _, _, _, _, _, _, _, _) => {
                                        h1041
                                    }
                                };
                                let a1043 = v6;
                                let a1044 = 1u64;
                                nat_rem(a1042, a1043, a1044)
                            };
                            let a1046 = 0u64;
                            nat_eq(a1045, a1046)
                        } else {
                            false
                        } {
                            let a1051 = {
                                let a1048 = match v3.clone() {
                                    Adt2::C0(_, h1047, _, _, _, _, _, _, _, _, _) => {
                                        h1047
                                    }
                                };
                                let a1049 = v6;
                                let a1050 = 1u64;
                                nat_rem(a1048, a1049, a1050)
                            };
                            let a1052 = 0u64;
                            nat_eq(a1051, a1052)
                        } else {
                            false
                        } {
                            Ok(Adt9::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt14::C0(_, _, h1053) => {
                                    h1053
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
    let a1055 = match v2.clone() {
        Adt2::C0(h1054, _, _, _, _, _, _, _, _, _, _) => {
            h1054
        }
    };
    let a1056 = 0u64;
    if if nat_eq(a1055, a1056) {
        let a1058 = match v3.clone() {
            Adt2::C0(h1057, _, _, _, _, _, _, _, _, _, _) => {
                h1057
            }
        };
        let a1059 = 0u64;
        nat_eq(a1058, a1059)
    } else {
        false
    } {
        let a1060 = v0;
        let a1061 = 61u64;
        if nat_eq(a1060, a1061) {
            let v4: bool = {
                let a1063 = 0u64;
                let a1064 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1062, _) => {
                        h1062
                    }
                };
                nat_lt(a1063, a1064)
            };
            if if if if if if f89(match v2.clone() {
                Adt2::C0(_, h1065, _, _, _, _, _, _, _, _, _) => {
                    h1065
                }
            }, match v1.clone() {
                Adt1::C0(h1066, _, _, _, _, _, _, _, _, _) => {
                    h1066
                }
            }, match v1.clone() {
                Adt1::C0(_, h1067, _, _, _, _, _, _, _, _) => {
                    h1067
                }
            }, match v3.clone() {
                Adt2::C0(_, h1068, _, _, _, _, _, _, _, _, _) => {
                    h1068
                }
            })? {
                f89(match v2.clone() {
                    Adt2::C0(_, _, h1069, _, _, _, _, _, _, _, _) => {
                        h1069
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1070, _, _, _, _, _, _, _) => {
                        h1070
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1071, _, _, _, _, _, _) => {
                        h1071
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1072, _, _, _, _, _, _, _, _) => {
                        h1072
                    }
                })?
            } else {
                false
            } {
                f89(match v2.clone() {
                    Adt2::C0(_, _, _, h1073, _, _, _, _, _, _, _) => {
                        h1073
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1074, _, _, _, _, _) => {
                        h1074
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1075, _, _, _, _) => {
                        h1075
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1076, _, _, _, _, _, _, _) => {
                        h1076
                    }
                })?
            } else {
                false
            } {
                f89(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1077, _, _, _, _, _, _) => {
                        h1077
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1078, _, _, _) => {
                        h1078
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1079, _, _) => {
                        h1079
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1080, _, _, _, _, _, _) => {
                        h1080
                    }
                })?
            } else {
                false
            } {
                let a1082 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1081, _) => {
                        h1081
                    }
                };
                let a1083 = 4294967295u64;
                nat_le(a1082, a1083)
            } else {
                false
            } {
                if v4 {
                    !f27(match v2.clone() {
                        Adt2::C0(_, h1084, _, _, _, _, _, _, _, _, _) => {
                            h1084
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1085, _, _, _, _, _, _, _, _) => {
                            h1085
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1086, _, _, _, _, _, _, _) => {
                            h1086
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1087, _, _, _, _, _, _) => {
                            h1087
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt9::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1088, _, _, _, _, _, _, _, _, _) => {
                        h1088
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1089, _, _, _, _, _, _, _) => {
                        h1089
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1090, _, _, _, _, _) => {
                        h1090
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1091, _, _, _) => {
                        h1091
                    }
                }, v4)))
            } else {
                Ok(Adt9::C1)
            }
        } else {
            let a1092 = v0;
            let a1093 = 62u64;
            if nat_eq(a1092, a1093) {
                let a1095 = match v1.clone() {
                    Adt1::C0(h1094, _, _, _, _, _, _, _, _, _) => {
                        h1094
                    }
                };
                let a1096 = 2147483648u64;
                if if if if if nat_lt(a1095, a1096) {
                    let a1098 = match v1.clone() {
                        Adt1::C0(_, h1097, _, _, _, _, _, _, _, _) => {
                            h1097
                        }
                    };
                    let a1099 = 2147483648u64;
                    nat_lt(a1098, a1099)
                } else {
                    false
                } {
                    let a1102 = match v1.clone() {
                        Adt1::C0(h1100, _, _, _, _, _, _, _, _, _) => {
                            h1100
                        }
                    };
                    let a1103 = match v2.clone() {
                        Adt2::C0(_, h1101, _, _, _, _, _, _, _, _, _) => {
                            h1101
                        }
                    };
                    nat_lt(a1102, a1103)
                } else {
                    false
                } {
                    let a1106 = match v1.clone() {
                        Adt1::C0(_, h1104, _, _, _, _, _, _, _, _) => {
                            h1104
                        }
                    };
                    let a1107 = match v2.clone() {
                        Adt2::C0(_, h1105, _, _, _, _, _, _, _, _, _) => {
                            h1105
                        }
                    };
                    nat_lt(a1106, a1107)
                } else {
                    false
                } {
                    let a1110 = match v2.clone() {
                        Adt2::C0(_, _, h1108, _, _, _, _, _, _, _, _) => {
                            h1108
                        }
                    };
                    let a1111 = match v3.clone() {
                        Adt2::C0(_, _, h1109, _, _, _, _, _, _, _, _) => {
                            h1109
                        }
                    };
                    if if if nat_eq(a1110, a1111) {
                        let a1114 = match v2.clone() {
                            Adt2::C0(_, _, _, h1112, _, _, _, _, _, _, _) => {
                                h1112
                            }
                        };
                        let a1115 = match v3.clone() {
                            Adt2::C0(_, _, _, h1113, _, _, _, _, _, _, _) => {
                                h1113
                            }
                        };
                        nat_eq(a1114, a1115)
                    } else {
                        false
                    } {
                        let a1118 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1116, _, _, _, _, _, _) => {
                                h1116
                            }
                        };
                        let a1119 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1117, _, _, _, _, _, _) => {
                                h1117
                            }
                        };
                        nat_eq(a1118, a1119)
                    } else {
                        false
                    } {
                        let a1128 = match v3.clone() {
                            Adt2::C0(_, h1120, _, _, _, _, _, _, _, _, _) => {
                                h1120
                            }
                        };
                        let a1129 = {
                            let a1126 = {
                                let a1123 = match v2.clone() {
                                    Adt2::C0(_, h1121, _, _, _, _, _, _, _, _, _) => {
                                        h1121
                                    }
                                };
                                let a1124 = match v1.clone() {
                                    Adt1::C0(h1122, _, _, _, _, _, _, _, _, _) => {
                                        h1122
                                    }
                                };
                                nat_add(a1123, a1124)?
                            };
                            let a1127 = match v1.clone() {
                                Adt1::C0(_, h1125, _, _, _, _, _, _, _, _) => {
                                    h1125
                                }
                            };
                            nat_add(a1126, a1127)?
                        };
                        nat_eq(a1128, a1129)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt9::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1130, _, _, _, _, _, _, _, _, _) => {
                            h1130
                        }
                    })))
                } else {
                    Ok(Adt9::C1)
                }
            } else {
                let a1131 = v0;
                let a1132 = 63u64;
                if nat_eq(a1131, a1132) {
                    if if if if if f90(match v2.clone() {
                        Adt2::C0(_, h1133, _, _, _, _, _, _, _, _, _) => {
                            h1133
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1134, _, _, _, _, _, _, _, _, _) => {
                            h1134
                        }
                    })? {
                        f90(match v2.clone() {
                            Adt2::C0(_, _, h1135, _, _, _, _, _, _, _, _) => {
                                h1135
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1136, _, _, _, _, _, _, _, _) => {
                                h1136
                            }
                        })?
                    } else {
                        false
                    } {
                        f90(match v2.clone() {
                            Adt2::C0(_, _, _, h1137, _, _, _, _, _, _, _) => {
                                h1137
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1138, _, _, _, _, _, _, _) => {
                                h1138
                            }
                        })?
                    } else {
                        false
                    } {
                        f90(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1139, _, _, _, _, _, _) => {
                                h1139
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1140, _, _, _, _, _, _) => {
                                h1140
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1143 = match v2.clone() {
                            Adt2::C0(_, h1141, _, _, _, _, _, _, _, _, _) => {
                                h1141
                            }
                        };
                        let a1144 = match v3.clone() {
                            Adt2::C0(_, h1142, _, _, _, _, _, _, _, _, _) => {
                                h1142
                            }
                        };
                        if if if nat_eq(a1143, a1144) {
                            let a1147 = match v2.clone() {
                                Adt2::C0(_, _, h1145, _, _, _, _, _, _, _, _) => {
                                    h1145
                                }
                            };
                            let a1148 = match v3.clone() {
                                Adt2::C0(_, _, h1146, _, _, _, _, _, _, _, _) => {
                                    h1146
                                }
                            };
                            nat_eq(a1147, a1148)
                        } else {
                            false
                        } {
                            let a1151 = match v2.clone() {
                                Adt2::C0(_, _, _, h1149, _, _, _, _, _, _, _) => {
                                    h1149
                                }
                            };
                            let a1152 = match v3.clone() {
                                Adt2::C0(_, _, _, h1150, _, _, _, _, _, _, _) => {
                                    h1150
                                }
                            };
                            nat_eq(a1151, a1152)
                        } else {
                            false
                        } {
                            let a1155 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1153, _, _, _, _, _, _) => {
                                    h1153
                                }
                            };
                            let a1156 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1154, _, _, _, _, _, _) => {
                                    h1154
                                }
                            };
                            nat_eq(a1155, a1156)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1157, _, _, _, _, _, _, _, _, _) => {
                                h1157
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1158, _, _, _, _, _, _, _, _) => {
                                h1158
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1159, _, _, _, _, _, _, _) => {
                                h1159
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1160, _, _, _, _, _, _) => {
                                h1160
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
    let m1161 = v3;
    match m1161 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1162 = v0;
                let a1163 = 4u64;
                nat_eq(a1162, a1163)
            };
            let v7: bool = {
                let a1165 = 0u64;
                let a1166 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1164, _, _, _, _, _) => {
                        h1164
                    }
                };
                nat_lt(a1165, a1166)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1167, _, _, _, _, _, _, _, _, _) => {
                    h1167
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1168, _, _, _, _, _, _, _, _) => {
                    h1168
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1169, _, _, _, _, _, _, _) => {
                    h1169
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1170, _, _, _, _, _, _) => {
                    h1170
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1171, _, _, _, _, _, _, _, _, _) => {
                    h1171
                }
            }, match v1.clone() {
                Adt1::C0(_, h1172, _, _, _, _, _, _, _, _) => {
                    h1172
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1173, _, _, _, _, _, _, _) => {
                    h1173
                }
            });
            if if if if if if if if if if if if v6 {
                let a1175 = match v2.clone() {
                    Adt2::C0(h1174, _, _, _, _, _, _, _, _, _, _) => {
                        h1174
                    }
                };
                let a1176 = 0u64;
                if nat_eq(a1175, a1176) {
                    let a1178 = match v5.clone() {
                        Adt2::C0(h1177, _, _, _, _, _, _, _, _, _, _) => {
                            h1177
                        }
                    };
                    let a1179 = 0u64;
                    nat_eq(a1178, a1179)
                } else {
                    false
                }
            } else {
                let a1181 = match v2.clone() {
                    Adt2::C0(h1180, _, _, _, _, _, _, _, _, _, _) => {
                        h1180
                    }
                };
                let a1182 = 0u64;
                if if nat_eq(a1181, a1182) {
                    true
                } else {
                    let a1184 = match v2.clone() {
                        Adt2::C0(h1183, _, _, _, _, _, _, _, _, _, _) => {
                            h1183
                        }
                    };
                    let a1185 = 26u64;
                    nat_eq(a1184, a1185)
                } {
                    let a1187 = match v5.clone() {
                        Adt2::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
                            h1186
                        }
                    };
                    let a1188 = 0u64;
                    if nat_eq(a1187, a1188) {
                        true
                    } else {
                        let a1190 = match v5.clone() {
                            Adt2::C0(h1189, _, _, _, _, _, _, _, _, _, _) => {
                                h1189
                            }
                        };
                        let a1191 = 26u64;
                        nat_eq(a1190, a1191)
                    }
                } else {
                    false
                }
            } {
                let a1194 = match v2.clone() {
                    Adt2::C0(h1192, _, _, _, _, _, _, _, _, _, _) => {
                        h1192
                    }
                };
                let a1195 = match v4.clone() {
                    Adt2::C0(h1193, _, _, _, _, _, _, _, _, _, _) => {
                        h1193
                    }
                };
                nat_eq(a1194, a1195)
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
                let a1197 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1196, _, _, _, _, _) => {
                        h1196
                    }
                };
                let a1198 = 4u64;
                nat_eq(a1197, a1198)
            } else {
                false
            } {
                let a1199 = v8;
                let a1200 = f12(v2.clone(), 1u64)?;
                nat_le(a1199, a1200)
            } else {
                false
            } {
                let a1202 = match v1.clone() {
                    Adt1::C0(_, _, _, h1201, _, _, _, _, _, _) => {
                        h1201
                    }
                };
                let a1203 = 1073741824u64;
                nat_lt(a1202, a1203)
            } else {
                false
            } {
                let a1205 = match v1.clone() {
                    Adt1::C0(h1204, _, _, _, _, _, _, _, _, _) => {
                        h1204
                    }
                };
                let a1206 = 4294967295u64;
                if if if if nat_le(a1205, a1206) {
                    let a1208 = match v1.clone() {
                        Adt1::C0(_, h1207, _, _, _, _, _, _, _, _) => {
                            h1207
                        }
                    };
                    let a1209 = 4294967295u64;
                    nat_le(a1208, a1209)
                } else {
                    false
                } {
                    let a1211 = match v1.clone() {
                        Adt1::C0(_, _, h1210, _, _, _, _, _, _, _) => {
                            h1210
                        }
                    };
                    let a1212 = 4294967295u64;
                    nat_le(a1211, a1212)
                } else {
                    false
                } {
                    let a1214 = match v1.clone() {
                        Adt1::C0(_, _, _, h1213, _, _, _, _, _, _) => {
                            h1213
                        }
                    };
                    let a1215 = 4294967295u64;
                    nat_le(a1214, a1215)
                } else {
                    false
                } {
                    let a1217 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1216, _, _, _, _, _) => {
                            h1216
                        }
                    };
                    let a1218 = 4294967295u64;
                    nat_le(a1217, a1218)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1221 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1219) => {
                            h1219
                        }
                    };
                    let a1222 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1220) => {
                            h1220
                        }
                    };
                    nat_eq(a1221, a1222)
                } else {
                    true
                }
            } else {
                false
            } {
                f91(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1223, _, _, _, _, _, _) => {
                        h1223
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1224, _) => {
                        h1224
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
                    let a1225 = v10;
                    let a1226 = v8;
                    nat_add(a1225, a1226)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1227, _, _, _, _, _, _) => {
                        h1227
                    }
                }, v6)))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f45(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a1229 = match v0.clone() {
        Adt2::C0(h1228, _, _, _, _, _, _, _, _, _, _) => {
            h1228
        }
    };
    let a1230 = 0u64;
    if if if nat_eq(a1229, a1230) {
        let a1232 = match v1.clone() {
            Adt2::C0(h1231, _, _, _, _, _, _, _, _, _, _) => {
                h1231
            }
        };
        let a1233 = 0u64;
        nat_eq(a1232, a1233)
    } else {
        false
    } {
        f87(match v0.clone() {
            Adt2::C0(_, h1234, _, _, _, _, _, _, _, _, _) => {
                h1234
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1235, _, _, _, _, _, _, _, _) => {
                h1235
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1236, _, _, _, _, _, _, _) => {
                h1236
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1237, _, _, _, _, _, _) => {
                h1237
            }
        }, match v1.clone() {
            Adt2::C0(_, h1238, _, _, _, _, _, _, _, _, _) => {
                h1238
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1239, _, _, _, _, _, _, _, _) => {
                h1239
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1240, _, _, _, _, _, _, _) => {
                h1240
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1241, _, _, _, _, _, _) => {
                h1241
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt9::C4({
            let a1242 = v2;
            let a1243 = f12(v0.clone(), 1u64)?;
            nat_add(a1242, a1243)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f46(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt9> {
    let a1245 = match v1.clone() {
        Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
            h1244
        }
    };
    let a1246 = 0u64;
    if if if nat_eq(a1245, a1246) {
        true
    } else {
        let a1248 = match v1.clone() {
            Adt2::C0(h1247, _, _, _, _, _, _, _, _, _, _) => {
                h1247
            }
        };
        let a1249 = 1u64;
        nat_eq(a1248, a1249)
    } {
        true
    } else {
        let a1251 = match v1.clone() {
            Adt2::C0(h1250, _, _, _, _, _, _, _, _, _, _) => {
                h1250
            }
        };
        let a1252 = 30u64;
        nat_eq(a1251, a1252)
    } {
        let a1255 = match v1.clone() {
            Adt2::C0(h1253, _, _, _, _, _, _, _, _, _, _) => {
                h1253
            }
        };
        let a1256 = match v2.clone() {
            Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                h1254
            }
        };
        if if if if if nat_eq(a1255, a1256) {
            let a1259 = match v1.clone() {
                Adt2::C0(_, h1257, _, _, _, _, _, _, _, _, _) => {
                    h1257
                }
            };
            let a1260 = match v2.clone() {
                Adt2::C0(_, h1258, _, _, _, _, _, _, _, _, _) => {
                    h1258
                }
            };
            nat_eq(a1259, a1260)
        } else {
            false
        } {
            let a1263 = match v1.clone() {
                Adt2::C0(_, _, _, h1261, _, _, _, _, _, _, _) => {
                    h1261
                }
            };
            let a1264 = match v2.clone() {
                Adt2::C0(_, _, _, h1262, _, _, _, _, _, _, _) => {
                    h1262
                }
            };
            nat_eq(a1263, a1264)
        } else {
            false
        } {
            let a1267 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1265, _, _, _, _, _, _) => {
                    h1265
                }
            };
            let a1268 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1266, _, _, _, _, _, _) => {
                    h1266
                }
            };
            nat_eq(a1267, a1268)
        } else {
            false
        } {
            let a1269 = v0;
            let a1270 = 4u64;
            if nat_eq(a1269, a1270) {
                let a1272 = match v1.clone() {
                    Adt2::C0(_, _, h1271, _, _, _, _, _, _, _, _) => {
                        h1271
                    }
                };
                let a1273 = 1u64;
                if nat_eq(a1272, a1273) {
                    let a1276 = match v1.clone() {
                        Adt2::C0(_, h1274, _, _, _, _, _, _, _, _, _) => {
                            h1274
                        }
                    };
                    let a1277 = match v2.clone() {
                        Adt2::C0(_, _, h1275, _, _, _, _, _, _, _, _) => {
                            h1275
                        }
                    };
                    nat_eq(a1276, a1277)
                } else {
                    false
                }
            } else {
                let a1278 = v0;
                let a1279 = 4u64;
                if if nat_lt(a1278, a1279) {
                    let a1282 = match v1.clone() {
                        Adt2::C0(_, h1280, _, _, _, _, _, _, _, _, _) => {
                            h1280
                        }
                    };
                    let a1283 = match v1.clone() {
                        Adt2::C0(_, _, h1281, _, _, _, _, _, _, _, _) => {
                            h1281
                        }
                    };
                    nat_eq(a1282, a1283)
                } else {
                    false
                } {
                    let a1286 = match v1.clone() {
                        Adt2::C0(_, _, h1284, _, _, _, _, _, _, _, _) => {
                            h1284
                        }
                    };
                    let a1287 = match v2.clone() {
                        Adt2::C0(_, _, h1285, _, _, _, _, _, _, _, _) => {
                            h1285
                        }
                    };
                    nat_eq(a1286, a1287)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1289 = f86(match v1.clone() {
                Adt2::C0(h1288, _, _, _, _, _, _, _, _, _, _) => {
                    h1288
                }
            });
            match m1289 {
                Adt13::C0 => {
                    Ok(Adt9::C0)
                }
                Adt13::C1(v3) => {
                    Ok(Adt9::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt14::C0(_, _, h1290) => {
                            h1290
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
        Adt2::C0(h1291, _, _, _, _, _, _, _, _, _, _) => {
            h1291
        }
    }, v0) {
        let a1294 = match v1.clone() {
            Adt2::C0(h1292, _, _, _, _, _, _, _, _, _, _) => {
                h1292
            }
        };
        let a1295 = match v2.clone() {
            Adt2::C0(h1293, _, _, _, _, _, _, _, _, _, _) => {
                h1293
            }
        };
        nat_eq(a1294, a1295)
    } else {
        false
    } {
        let a1298 = match v1.clone() {
            Adt2::C0(_, h1296, _, _, _, _, _, _, _, _, _) => {
                h1296
            }
        };
        let a1299 = match v2.clone() {
            Adt2::C0(_, h1297, _, _, _, _, _, _, _, _, _) => {
                h1297
            }
        };
        if if if nat_eq(a1298, a1299) {
            let a1302 = match v1.clone() {
                Adt2::C0(_, _, h1300, _, _, _, _, _, _, _, _) => {
                    h1300
                }
            };
            let a1303 = match v2.clone() {
                Adt2::C0(_, _, h1301, _, _, _, _, _, _, _, _) => {
                    h1301
                }
            };
            nat_eq(a1302, a1303)
        } else {
            false
        } {
            let a1306 = match v1.clone() {
                Adt2::C0(_, _, _, h1304, _, _, _, _, _, _, _) => {
                    h1304
                }
            };
            let a1307 = match v2.clone() {
                Adt2::C0(_, _, _, h1305, _, _, _, _, _, _, _) => {
                    h1305
                }
            };
            nat_eq(a1306, a1307)
        } else {
            false
        } {
            let a1310 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1308, _, _, _, _, _, _) => {
                    h1308
                }
            };
            let a1311 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1309, _, _, _, _, _, _) => {
                    h1309
                }
            };
            nat_eq(a1310, a1311)
        } else {
            false
        }
    } else {
        false
    } {
        let m1313 = f86(match v1.clone() {
            Adt2::C0(h1312, _, _, _, _, _, _, _, _, _, _) => {
                h1312
            }
        });
        match m1313 {
            Adt13::C0 => {
                Ok(Adt9::C0)
            }
            Adt13::C1(v3) => {
                Ok(Adt9::C4(f12(v2.clone(), 1u64)?, match v3 {
                    Adt14::C0(_, _, h1314) => {
                        h1314
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1315, _, _, _, _, _, _, _, _, _, _) => {
                        h1315
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
            Adt1::C0(h1316, _, _, _, _, _, _, _, _, _) => {
                h1316
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1317, _, _, _, _, _, _, _) => {
                h1317
            }
        }
    };
    let a1318 = 0u64;
    let a1319 = v4;
    if if nat_lt(a1318, a1319) {
        let a1320 = v4;
        let a1321 = 2147483648u64;
        nat_lt(a1320, a1321)
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
            Adt2::C0(_, _, h1322, _, _, _, _, _, _, _, _) => {
                h1322
            }
        }, v4)?;
        let v8: u64 = f93(match v5.clone() {
            Adt2::C0(_, _, _, h1323, _, _, _, _, _, _, _) => {
                h1323
            }
        }, v4)?;
        let a1325 = match v5.clone() {
            Adt2::C0(h1324, _, _, _, _, _, _, _, _, _, _) => {
                h1324
            }
        };
        let a1326 = 0u64;
        if if if if if nat_eq(a1325, a1326) {
            let a1328 = match v6.clone() {
                Adt2::C0(h1327, _, _, _, _, _, _, _, _, _, _) => {
                    h1327
                }
            };
            let a1329 = 0u64;
            nat_eq(a1328, a1329)
        } else {
            false
        } {
            let a1331 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1330, _, _, _, _, _, _) => {
                    h1330
                }
            };
            let a1332 = 1u64;
            nat_eq(a1331, a1332)
        } else {
            false
        } {
            let a1335 = match v5.clone() {
                Adt2::C0(_, h1333, _, _, _, _, _, _, _, _, _) => {
                    h1333
                }
            };
            let a1336 = match v6.clone() {
                Adt2::C0(_, h1334, _, _, _, _, _, _, _, _, _) => {
                    h1334
                }
            };
            nat_eq(a1335, a1336)
        } else {
            false
        } {
            if v0 {
                let a1338 = match v5.clone() {
                    Adt2::C0(_, _, h1337, _, _, _, _, _, _, _, _) => {
                        h1337
                    }
                };
                let a1339 = 2147483648u64;
                if if if nat_lt(a1338, a1339) {
                    let a1341 = match v5.clone() {
                        Adt2::C0(_, _, _, h1340, _, _, _, _, _, _, _) => {
                            h1340
                        }
                    };
                    let a1342 = 2147483648u64;
                    nat_lt(a1341, a1342)
                } else {
                    false
                } {
                    let a1345 = {
                        let a1343 = v7;
                        let a1344 = v8;
                        nat_mul(a1343, a1344)?
                    };
                    let a1346 = 2147483648u64;
                    nat_le(a1345, a1346)
                } else {
                    false
                } {
                    f94(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1348 = match v1.clone() {
                    Adt1::C0(h1347, _, _, _, _, _, _, _, _, _) => {
                        h1347
                    }
                };
                let a1349 = v7;
                if if if if nat_eq(a1348, a1349) {
                    let a1351 = match v1 {
                        Adt1::C0(_, h1350, _, _, _, _, _, _, _, _) => {
                            h1350
                        }
                    };
                    let a1352 = v8;
                    nat_eq(a1351, a1352)
                } else {
                    false
                } {
                    f95(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1354 = match v6.clone() {
                        Adt2::C0(_, _, h1353, _, _, _, _, _, _, _, _) => {
                            h1353
                        }
                    };
                    let a1355 = v4;
                    if nat_eq(a1354, a1355) {
                        let a1357 = match v6.clone() {
                            Adt2::C0(_, _, _, h1356, _, _, _, _, _, _, _) => {
                                h1356
                            }
                        };
                        let a1358 = v4;
                        nat_eq(a1357, a1358)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1362 = match v6 {
                        Adt2::C0(_, _, _, _, h1359, _, _, _, _, _, _) => {
                            h1359
                        }
                    };
                    let a1363 = {
                        let a1360 = v7;
                        let a1361 = v8;
                        nat_mul(a1360, a1361)?
                    };
                    nat_eq(a1362, a1363)
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
    let a1365 = match v2.clone() {
        Adt2::C0(h1364, _, _, _, _, _, _, _, _, _, _) => {
            h1364
        }
    };
    let a1366 = 0u64;
    if if if if if nat_eq(a1365, a1366) {
        true
    } else {
        let a1368 = match v2.clone() {
            Adt2::C0(h1367, _, _, _, _, _, _, _, _, _, _) => {
                h1367
            }
        };
        let a1369 = 1u64;
        nat_eq(a1368, a1369)
    } {
        true
    } else {
        let a1371 = match v2.clone() {
            Adt2::C0(h1370, _, _, _, _, _, _, _, _, _, _) => {
                h1370
            }
        };
        let a1372 = 30u64;
        nat_eq(a1371, a1372)
    } {
        let a1375 = match v2.clone() {
            Adt2::C0(h1373, _, _, _, _, _, _, _, _, _, _) => {
                h1373
            }
        };
        let a1376 = match v3.clone() {
            Adt2::C0(h1374, _, _, _, _, _, _, _, _, _, _) => {
                h1374
            }
        };
        nat_eq(a1375, a1376)
    } else {
        false
    } {
        if if f19(v2.clone(), v3.clone()) {
            let a1378 = match v1.clone() {
                Adt1::C0(h1377, _, _, _, _, _, _, _, _, _) => {
                    h1377
                }
            };
            let a1379 = 4294967296u64;
            nat_lt(a1378, a1379)
        } else {
            false
        } {
            let a1381 = match v1.clone() {
                Adt1::C0(_, h1380, _, _, _, _, _, _, _, _) => {
                    h1380
                }
            };
            let a1382 = 4294967296u64;
            nat_lt(a1381, a1382)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt9::C4(f12(v3.clone(), 1u64)?, {
            let a1384 = match v2.clone() {
                Adt2::C0(h1383, _, _, _, _, _, _, _, _, _, _) => {
                    h1383
                }
            };
            let a1385 = 0u64;
            if nat_eq(a1384, a1385) {
                4u64
            } else {
                2u64
            }
        }, f16(v2.clone(), 1u64), f16(v3, 1u64), Adt4::C13({
            let a1386 = v0;
            let a1387 = 31u64;
            if nat_eq(a1386, a1387) {
                0u64
            } else {
                let a1388 = v0;
                let a1389 = 47u64;
                if nat_eq(a1388, a1389) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1390, _, _, _, _, _, _, _, _, _, _) => {
                h1390
            }
        }, match v1.clone() {
            Adt1::C0(h1391, _, _, _, _, _, _, _, _, _) => {
                h1391
            }
        }, match v1 {
            Adt1::C0(_, h1392, _, _, _, _, _, _, _, _) => {
                h1392
            }
        })))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f50(v0: Adt2, v1: Adt2) -> R<Adt9> {
    let a1394 = match v0.clone() {
        Adt2::C0(h1393, _, _, _, _, _, _, _, _, _, _) => {
            h1393
        }
    };
    let a1395 = 0u64;
    if if if nat_eq(a1394, a1395) {
        let a1397 = match v1.clone() {
            Adt2::C0(h1396, _, _, _, _, _, _, _, _, _, _) => {
                h1396
            }
        };
        let a1398 = 0u64;
        nat_eq(a1397, a1398)
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
    let a1400 = match v0.clone() {
        Adt2::C0(h1399, _, _, _, _, _, _, _, _, _, _) => {
            h1399
        }
    };
    let a1401 = 0u64;
    if if nat_eq(a1400, a1401) {
        let a1403 = match v1.clone() {
            Adt2::C0(h1402, _, _, _, _, _, _, _, _, _, _) => {
                h1402
            }
        };
        let a1404 = 26u64;
        nat_eq(a1403, a1404)
    } else {
        false
    } {
        let a1406 = match v0.clone() {
            Adt2::C0(_, _, _, h1405, _, _, _, _, _, _, _) => {
                h1405
            }
        };
        let a1407 = 1u64;
        if if if if if if if nat_eq(a1406, a1407) {
            let a1409 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1408, _, _, _, _, _, _) => {
                    h1408
                }
            };
            let a1410 = 1u64;
            nat_eq(a1409, a1410)
        } else {
            false
        } {
            let a1412 = match v0.clone() {
                Adt2::C0(_, h1411, _, _, _, _, _, _, _, _, _) => {
                    h1411
                }
            };
            let a1413 = 2147483647u64;
            nat_le(a1412, a1413)
        } else {
            false
        } {
            let a1416 = match v1.clone() {
                Adt2::C0(_, h1414, _, _, _, _, _, _, _, _, _) => {
                    h1414
                }
            };
            let a1417 = match v0.clone() {
                Adt2::C0(_, _, h1415, _, _, _, _, _, _, _, _) => {
                    h1415
                }
            };
            nat_eq(a1416, a1417)
        } else {
            false
        } {
            let a1419 = match v1.clone() {
                Adt2::C0(_, _, h1418, _, _, _, _, _, _, _, _) => {
                    h1418
                }
            };
            let a1420 = 1u64;
            nat_eq(a1419, a1420)
        } else {
            false
        } {
            let a1422 = match v1.clone() {
                Adt2::C0(_, _, _, h1421, _, _, _, _, _, _, _) => {
                    h1421
                }
            };
            let a1423 = 1u64;
            nat_eq(a1422, a1423)
        } else {
            false
        } {
            let a1425 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1424, _, _, _, _, _, _) => {
                    h1424
                }
            };
            let a1426 = 1u64;
            nat_eq(a1425, a1426)
        } else {
            false
        } {
            Ok(Adt9::C4({
                let a1429 = match v0.clone() {
                    Adt2::C0(_, _, h1427, _, _, _, _, _, _, _, _) => {
                        h1427
                    }
                };
                let a1430 = f96(match v0.clone() {
                    Adt2::C0(_, h1428, _, _, _, _, _, _, _, _, _) => {
                        h1428
                    }
                })?;
                nat_mul(a1429, a1430)?
            }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f52(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt9> {
    let m1431 = v1;
    match m1431 {
        None => {
            Ok(Adt9::C1)
        }
        Some(v3) => {
            let v4: u64 = f97(f16(v0.clone(), 1u64))?;
            let a1433 = match v0.clone() {
                Adt2::C0(h1432, _, _, _, _, _, _, _, _, _, _) => {
                    h1432
                }
            };
            let a1434 = 26u64;
            if if if nat_eq(a1433, a1434) {
                let a1436 = match v3.clone() {
                    Adt2::C0(h1435, _, _, _, _, _, _, _, _, _, _) => {
                        h1435
                    }
                };
                let a1437 = 26u64;
                nat_eq(a1436, a1437)
            } else {
                false
            } {
                let a1439 = match v2.clone() {
                    Adt2::C0(h1438, _, _, _, _, _, _, _, _, _, _) => {
                        h1438
                    }
                };
                let a1440 = 27u64;
                nat_eq(a1439, a1440)
            } else {
                false
            } {
                if if if if if if f19(v0.clone(), v3.clone()) {
                    let a1442 = match v2.clone() {
                        Adt2::C0(_, h1441, _, _, _, _, _, _, _, _, _) => {
                            h1441
                        }
                    };
                    let a1443 = 1u64;
                    nat_eq(a1442, a1443)
                } else {
                    false
                } {
                    let a1445 = match v2.clone() {
                        Adt2::C0(_, _, h1444, _, _, _, _, _, _, _, _) => {
                            h1444
                        }
                    };
                    let a1446 = 1u64;
                    nat_eq(a1445, a1446)
                } else {
                    false
                } {
                    let a1448 = match v2.clone() {
                        Adt2::C0(_, _, _, h1447, _, _, _, _, _, _, _) => {
                            h1447
                        }
                    };
                    let a1449 = 1u64;
                    nat_eq(a1448, a1449)
                } else {
                    false
                } {
                    let a1451 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1450, _, _, _, _, _, _) => {
                            h1450
                        }
                    };
                    let a1452 = 1u64;
                    nat_eq(a1451, a1452)
                } else {
                    false
                } {
                    let a1453 = v4;
                    let a1454 = 9223372036854775808u64;
                    nat_lt(a1453, a1454)
                } else {
                    false
                } {
                    Ok(Adt9::C4({
                        let a1457 = 2u64;
                        let a1458 = {
                            let a1455 = v4;
                            let a1456 = 2u64;
                            nat_mul(a1455, a1456)?
                        };
                        nat_add(a1457, a1458)?
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
        Adt2::C0(h1459, _, _, _, _, _, _, _, _, _, _) => {
            h1459
        }
    };
    let a1460 = v3;
    let a1461 = 0u64;
    if if nat_eq(a1460, a1461) {
        true
    } else {
        let a1462 = v0;
        let a1463 = 13u64;
        if nat_eq(a1462, a1463) {
            let a1464 = v3;
            let a1465 = 1u64;
            if nat_eq(a1464, a1465) {
                true
            } else {
                let a1466 = v3;
                let a1467 = 30u64;
                nat_eq(a1466, a1467)
            }
        } else {
            false
        }
    } {
        let a1469 = v3;
        let a1470 = match v2.clone() {
            Adt2::C0(h1468, _, _, _, _, _, _, _, _, _, _) => {
                h1468
            }
        };
        if if nat_eq(a1469, a1470) {
            let a1471 = v0;
            let a1472 = 13u64;
            if nat_eq(a1471, a1472) {
                let a1474 = match v2.clone() {
                    Adt2::C0(_, h1473, _, _, _, _, _, _, _, _, _) => {
                        h1473
                    }
                };
                let a1475 = 1u64;
                if if if nat_eq(a1474, a1475) {
                    let a1477 = match v2.clone() {
                        Adt2::C0(_, _, h1476, _, _, _, _, _, _, _, _) => {
                            h1476
                        }
                    };
                    let a1478 = 1u64;
                    nat_eq(a1477, a1478)
                } else {
                    false
                } {
                    let a1480 = match v2.clone() {
                        Adt2::C0(_, _, _, h1479, _, _, _, _, _, _, _) => {
                            h1479
                        }
                    };
                    let a1481 = 1u64;
                    nat_eq(a1480, a1481)
                } else {
                    false
                } {
                    let a1483 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1482, _, _, _, _, _, _) => {
                            h1482
                        }
                    };
                    let a1484 = 1u64;
                    nat_eq(a1483, a1484)
                } else {
                    false
                }
            } else {
                let a1486 = match v2.clone() {
                    Adt2::C0(_, h1485, _, _, _, _, _, _, _, _, _) => {
                        h1485
                    }
                };
                let a1487 = 1u64;
                if if if nat_eq(a1486, a1487) {
                    let a1490 = match v1.clone() {
                        Adt2::C0(_, _, h1488, _, _, _, _, _, _, _, _) => {
                            h1488
                        }
                    };
                    let a1491 = match v2.clone() {
                        Adt2::C0(_, _, h1489, _, _, _, _, _, _, _, _) => {
                            h1489
                        }
                    };
                    nat_eq(a1490, a1491)
                } else {
                    false
                } {
                    let a1494 = match v1.clone() {
                        Adt2::C0(_, _, _, h1492, _, _, _, _, _, _, _) => {
                            h1492
                        }
                    };
                    let a1495 = match v2.clone() {
                        Adt2::C0(_, _, _, h1493, _, _, _, _, _, _, _) => {
                            h1493
                        }
                    };
                    nat_eq(a1494, a1495)
                } else {
                    false
                } {
                    let a1498 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1496, _, _, _, _, _, _) => {
                            h1496
                        }
                    };
                    let a1499 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1497, _, _, _, _, _, _) => {
                            h1497
                        }
                    };
                    nat_eq(a1498, a1499)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f16(v1, 1u64);
            Ok(Adt9::C4(f98(v4.clone(), v0, v3)?, {
                let a1500 = v3;
                let a1501 = 0u64;
                if nat_eq(a1500, a1501) {
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
    let a1502 = v0;
    let a1503 = 89u64;
    if if nat_eq(a1502, a1503) {
        let a1504 = 8u64;
        let a1505 = v1;
        if nat_le(a1504, a1505) {
            let a1506 = v1;
            let a1507 = 9u64;
            nat_le(a1506, a1507)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a1508 = v1;
            let a1509 = 12u64;
            nat_add(a1508, a1509)?
        }))
    } else {
        let a1510 = v0;
        let a1511 = 89u64;
        if if nat_eq(a1510, a1511) {
            let a1512 = v1;
            let a1513 = 4u64;
            nat_eq(a1512, a1513)
        } else {
            false
        } {
            Ok(Some(19u64))
        } else {
            let a1514 = 10u64;
            let a1515 = v0;
            if if nat_le(a1514, a1515) {
                let a1516 = v0;
                let a1517 = 12u64;
                nat_le(a1516, a1517)
            } else {
                false
            } {
                Ok(Some({
                    let a1518 = v0;
                    let a1519 = 3u64;
                    nat_add(a1518, a1519)?
                }))
            } else {
                let a1520 = v0;
                let a1521 = 89u64;
                if if nat_eq(a1520, a1521) {
                    let a1522 = v1;
                    let a1523 = 7u64;
                    nat_eq(a1522, a1523)
                } else {
                    false
                } {
                    Ok(Some(17u64))
                } else {
                    let a1524 = v0;
                    let a1525 = 89u64;
                    if if nat_eq(a1524, a1525) {
                        let a1526 = v1;
                        let a1527 = 10u64;
                        nat_eq(a1526, a1527)
                    } else {
                        false
                    } {
                        Ok(Some(18u64))
                    } else {
                        let a1528 = v0;
                        let a1529 = 89u64;
                        if if nat_eq(a1528, a1529) {
                            let a1530 = v1;
                            let a1531 = 13u64;
                            nat_eq(a1530, a1531)
                        } else {
                            false
                        } {
                            Ok(Some(16u64))
                        } else {
                            let a1532 = v0;
                            let a1533 = 8u64;
                            if nat_eq(a1532, a1533) {
                                Ok(Some(5u64))
                            } else {
                                let a1534 = v0;
                                let a1535 = 9u64;
                                if nat_eq(a1534, a1535) {
                                    Ok(Some(6u64))
                                } else {
                                    let a1536 = v0;
                                    let a1537 = 89u64;
                                    if nat_eq(a1536, a1537) {
                                        let a1538 = v1;
                                        let a1539 = 0u64;
                                        if nat_eq(a1538, a1539) {
                                            Ok(Some(0u64))
                                        } else {
                                            let a1540 = v1;
                                            let a1541 = 1u64;
                                            if nat_eq(a1540, a1541) {
                                                Ok(Some(1u64))
                                            } else {
                                                let a1542 = v1;
                                                let a1543 = 2u64;
                                                if nat_eq(a1542, a1543) {
                                                    Ok(Some(2u64))
                                                } else {
                                                    let a1544 = v1;
                                                    let a1545 = 3u64;
                                                    if nat_eq(a1544, a1545) {
                                                        Ok(Some(3u64))
                                                    } else {
                                                        let a1546 = v1;
                                                        let a1547 = 6u64;
                                                        if nat_eq(a1546, a1547) {
                                                            Ok(Some(4u64))
                                                        } else {
                                                            let a1548 = v1;
                                                            let a1549 = 12u64;
                                                            if nat_eq(a1548, a1549) {
                                                                Ok(Some(7u64))
                                                            } else {
                                                                let a1550 = v1;
                                                                let a1551 = 11u64;
                                                                if nat_eq(a1550, a1551) {
                                                                    Ok(Some(8u64))
                                                                } else {
                                                                    let a1552 = v1;
                                                                    let a1553 = 18u64;
                                                                    if nat_eq(a1552, a1553) {
                                                                        Ok(Some(9u64))
                                                                    } else {
                                                                        let a1554 = v1;
                                                                        let a1555 = 19u64;
                                                                        if nat_eq(a1554, a1555) {
                                                                            Ok(Some(10u64))
                                                                        } else {
                                                                            let a1556 = v1;
                                                                            let a1557 = 20u64;
                                                                            if nat_eq(a1556, a1557) {
                                                                                Ok(Some(11u64))
                                                                            } else {
                                                                                let a1558 = v1;
                                                                                let a1559 = 21u64;
                                                                                if nat_eq(a1558, a1559) {
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
}

pub fn f55(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let m1560 = f99(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1560 {
        Adt9::C0 => {
            Ok(Adt9::C0)
        }
        Adt9::C1 => {
            let a1561 = v0;
            let a1562 = 2u64;
            if if if if nat_eq(a1561, a1562) {
                true
            } else {
                let a1563 = v0;
                let a1564 = 5u64;
                nat_eq(a1563, a1564)
            } {
                true
            } else {
                let a1565 = v0;
                let a1566 = 6u64;
                nat_eq(a1565, a1566)
            } {
                true
            } else {
                let a1567 = v0;
                let a1568 = 7u64;
                nat_eq(a1567, a1568)
            } {
                Ok(Adt9::C5(67u64))
            } else {
                Ok(Adt9::C1)
            }
        }
        Adt9::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt9::C4(v13, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1569 = v3;
                    match m1569 {
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
    let a1571 = match v1.clone() {
        Adt2::C0(h1570, _, _, _, _, _, _, _, _, _, _) => {
            h1570
        }
    };
    let a1572 = 0u64;
    if if if if if nat_eq(a1571, a1572) {
        true
    } else {
        let a1574 = match v1.clone() {
            Adt2::C0(h1573, _, _, _, _, _, _, _, _, _, _) => {
                h1573
            }
        };
        let a1575 = 1u64;
        nat_eq(a1574, a1575)
    } {
        true
    } else {
        let a1577 = match v1.clone() {
            Adt2::C0(h1576, _, _, _, _, _, _, _, _, _, _) => {
                h1576
            }
        };
        let a1578 = 30u64;
        nat_eq(a1577, a1578)
    } {
        let a1581 = match v1.clone() {
            Adt2::C0(h1579, _, _, _, _, _, _, _, _, _, _) => {
                h1579
            }
        };
        let a1582 = match v2.clone() {
            Adt2::C0(h1580, _, _, _, _, _, _, _, _, _, _) => {
                h1580
            }
        };
        if nat_eq(a1581, a1582) {
            true
        } else {
            let a1584 = match v2.clone() {
                Adt2::C0(h1583, _, _, _, _, _, _, _, _, _, _) => {
                    h1583
                }
            };
            let a1585 = 0u64;
            nat_eq(a1584, a1585)
        }
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt9::C4(f12(v2.clone(), 1u64)?, {
            let a1587 = match v2.clone() {
                Adt2::C0(h1586, _, _, _, _, _, _, _, _, _, _) => {
                    h1586
                }
            };
            let a1588 = 0u64;
            if nat_eq(a1587, a1588) {
                4u64
            } else {
                2u64
            }
        }, f16(v1.clone(), 1u64), f16(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1589, _, _, _, _, _, _, _, _, _, _) => {
                h1589
            }
        }, match v2 {
            Adt2::C0(h1590, _, _, _, _, _, _, _, _, _, _) => {
                h1590
            }
        })))
    } else {
        Ok(Adt9::C1)
    }
}

pub fn f57(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f100(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1591, _, _, _) => {
            h1591
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1592, _, _) => {
            h1592
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1593, _) => {
            h1593
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1594) => {
            h1594
        }
    })
}

pub fn f58(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1596 = v0;
        let a1597 = match v1.clone() {
            Adt3::C0(h1595, _, _, _, _, _, _, _) => {
                h1595
            }
        };
        let a1598 = 0u64;
        nat_quot(a1596, a1597, a1598)
    };
    let v4: u64 = {
        let a1600 = v3;
        let a1601 = match v1.clone() {
            Adt3::C0(_, h1599, _, _, _, _, _, _) => {
                h1599
            }
        };
        let a1602 = 0u64;
        nat_quot(a1600, a1601, a1602)
    };
    let a1603 = v2;
    let a1604 = 0u64;
    if nat_eq(a1603, a1604) {
        let a1606 = v0;
        let a1607 = match v1 {
            Adt3::C0(h1605, _, _, _, _, _, _, _) => {
                h1605
            }
        };
        let a1608 = 0u64;
        nat_rem(a1606, a1607, a1608)
    } else {
        let a1609 = v2;
        let a1610 = 1u64;
        if nat_eq(a1609, a1610) {
            let a1612 = v3;
            let a1613 = match v1 {
                Adt3::C0(_, h1611, _, _, _, _, _, _) => {
                    h1611
                }
            };
            let a1614 = 0u64;
            nat_rem(a1612, a1613, a1614)
        } else {
            let a1615 = v2;
            let a1616 = 2u64;
            if nat_eq(a1615, a1616) {
                let a1618 = v4;
                let a1619 = match v1 {
                    Adt3::C0(_, _, h1617, _, _, _, _, _) => {
                        h1617
                    }
                };
                let a1620 = 0u64;
                nat_rem(a1618, a1619, a1620)
            } else {
                let a1622 = v4;
                let a1623 = match v1 {
                    Adt3::C0(_, _, h1621, _, _, _, _, _) => {
                        h1621
                    }
                };
                let a1624 = 0u64;
                nat_quot(a1622, a1623, a1624)
            }
        }
    }
}

pub fn f59(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1634 = {
            let a1631 = {
                let a1626 = v2;
                let a1627 = match v0.clone() {
                    Adt6::C0(_, h1625, _, _) => {
                        h1625
                    }
                };
                nat_mul(a1626, a1627)?
            };
            let a1632 = {
                let a1629 = v3;
                let a1630 = match v0.clone() {
                    Adt6::C0(_, _, _, h1628) => {
                        h1628
                    }
                };
                nat_mul(a1629, a1630)?
            };
            nat_add(a1631, a1632)?
        };
        let a1635 = f73(match v0.clone() {
            Adt6::C0(_, _, h1633, _) => {
                h1633
            }
        });
        nat_add(a1634, a1635)?
    };
    let v5: u64 = f72(match v0 {
        Adt6::C0(_, _, h1636, _) => {
            h1636
        }
    });
    let a1637 = v5;
    let a1638 = v4;
    if if nat_le(a1637, a1638) {
        let a1641 = {
            let a1639 = v4;
            let a1640 = v5;
            nat_sub(a1639, a1640)
        };
        let a1642 = v1;
        nat_lt(a1641, a1642)
    } else {
        false
    } {
        Ok(Some({
            let a1643 = v4;
            let a1644 = v5;
            nat_sub(a1643, a1644)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f60(v0: Adt4, v1: Adt8) -> Adt8 {
    let m1645 = v1;
    match m1645 {
        Adt8::C0(v4) => {
            Adt8::C0(v4)
        }
        Adt8::C1(v5, v6, v7) => {
            Adt8::C1(v5, v6, v7)
        }
        Adt8::C2(v8, v9, v10, v11, v12, v13) => {
            Adt8::C2(v8, v9, v10, v11, v12, v13)
        }
        Adt8::C3(v14, v15, v16, v17) => {
            f101(f102(v0.clone(), v14, v15), v14, v15, 0u64, v16, 2u64, f103(v0), 3u64, v17)
        }
        Adt8::C4(v18, v19, v20, v21, v22, v23, v24) => {
            let v25: u64 = f104(v0.clone(), v18);
            f101(f105(v0, v18), v25, v25, v19, v20, v21, v22, v23, v24)
        }
        Adt8::C5(v26, v27, v28, v29, v30, v31) => {
            Adt8::C5(v26, v27, v28, v29, v30, v31)
        }
        Adt8::C6(v32, v33, v34, v35) => {
            Adt8::C6(v32, v33, v34, v35)
        }
        Adt8::C7(v36, v37, v38, v39, v40, v41) => {
            Adt8::C7(v36, v37, v38, v39, v40, v41)
        }
        Adt8::C8(v2) => {
            Adt8::C8(v2)
        }
        Adt8::C9(v3) => {
            Adt8::C9(v3)
        }
    }
}

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt8> {
    let m1646 = v3;
    match m1646 {
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
                Adt3::C0(h1647, _, _, _, _, _, _, _) => {
                    h1647
                }
            }, match v2.clone() {
                Adt3::C0(_, h1648, _, _, _, _, _, _) => {
                    h1648
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
            let a1649 = v52;
            let a1650 = 64u64;
            if if if nat_eq(a1649, a1650) {
                true
            } else {
                let a1651 = v52;
                let a1652 = 65u64;
                nat_eq(a1651, a1652)
            } {
                true
            } else {
                let a1653 = v52;
                let a1654 = 66u64;
                nat_eq(a1653, a1654)
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
        Adt3::C0(h1655, _, _, _, _, _, _, _) => {
            h1655
        }
    })?;
    let v5: u64 = {
        let a1656 = v0;
        let a1657 = v4;
        let a1658 = 0u64;
        nat_quot(a1656, a1657, a1658)
    };
    let v6: u64 = {
        let a1659 = v0;
        let a1660 = v4;
        let a1661 = 0u64;
        nat_rem(a1659, a1660, a1661)
    };
    let a1662 = v6;
    let a1663 = 0u64;
    if nat_eq(a1662, a1663) {
        Ok(Adt8::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1664 = v6;
        let a1665 = 1u64;
        if nat_eq(a1664, a1665) {
            Ok(Adt8::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1668 = {
                let a1666 = v6;
                let a1667 = 1u64;
                nat_add(a1666, a1667)?
            };
            let a1669 = v4;
            if nat_eq(a1668, a1669) {
                Ok(Adt8::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f106(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1670 = v6;
                    let a1671 = 2u64;
                    nat_sub(a1670, a1671)
                };
                let v8: u64 = {
                    let a1672 = v7;
                    let a1673 = 3u64;
                    let a1674 = 0u64;
                    nat_quot(a1672, a1673, a1674)
                };
                let v9: u64 = {
                    let a1675 = v7;
                    let a1676 = 3u64;
                    let a1677 = 0u64;
                    nat_rem(a1675, a1676, a1677)
                };
                let v10: u64 = f106({
                    let a1681 = {
                        let a1679 = v5;
                        let a1680 = match v1.clone() {
                            Adt3::C0(h1678, _, _, _, _, _, _, _) => {
                                h1678
                            }
                        };
                        nat_mul(a1679, a1680)?
                    };
                    let a1682 = v8;
                    nat_add(a1681, a1682)?
                }, v1)?;
                let a1683 = v9;
                let a1684 = 0u64;
                if nat_eq(a1683, a1684) {
                    Ok(Adt8::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1685 = v9;
                    let a1686 = 1u64;
                    if nat_eq(a1685, a1686) {
                        let a1687 = v3;
                        let a1688 = 2u64;
                        if nat_eq(a1687, a1688) {
                            Ok(Adt8::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt8::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1689 = v3;
                        let a1690 = 2u64;
                        if if nat_eq(a1689, a1690) {
                            true
                        } else {
                            let a1691 = v3;
                            let a1692 = 3u64;
                            nat_eq(a1691, a1692)
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
        let a1694 = 2u64;
        let a1695 = match v1.clone() {
            Adt3::C0(_, h1693, _, _, _, _, _, _) => {
                h1693
            }
        };
        nat_mul(a1694, a1695)?
    };
    let a1696 = v0;
    let a1697 = v5;
    if nat_lt(a1696, a1697) {
        let a1701 = {
            let a1698 = v0;
            let a1699 = 2u64;
            let a1700 = 0u64;
            nat_rem(a1698, a1699, a1700)
        };
        let a1702 = 0u64;
        if nat_eq(a1701, a1702) {
            Ok(Adt8::C4(19u64, 4u64, f106({
                let a1703 = v0;
                let a1704 = 2u64;
                let a1705 = 0u64;
                nat_quot(a1703, a1704, a1705)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1706 = v4;
            let a1707 = 2147483648u64;
            if if nat_lt(a1706, a1707) {
                let a1709 = v4;
                let a1710 = match v2 {
                    Adt3::C0(_, h1708, _, _, _, _, _, _) => {
                        h1708
                    }
                };
                nat_lt(a1709, a1710)
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
            let a1713 = match v2.clone() {
                Adt3::C0(h1711, _, _, _, _, _, _, _) => {
                    h1711
                }
            };
            let a1714 = match v2.clone() {
                Adt3::C0(_, h1712, _, _, _, _, _, _) => {
                    h1712
                }
            };
            nat_mul(a1713, a1714)?
        };
        let a1717 = {
            let a1715 = v0;
            let a1716 = v5;
            nat_sub(a1715, a1716)
        };
        let a1718 = v6;
        if nat_lt(a1717, a1718) {
            Ok(Adt8::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f106({
                let a1719 = v0;
                let a1720 = v5;
                nat_sub(a1719, a1720)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1723 = {
                    let a1721 = v0;
                    let a1722 = v5;
                    nat_sub(a1721, a1722)
                };
                let a1724 = v6;
                nat_sub(a1723, a1724)
            };
            let v8: u64 = {
                let a1728 = 1u64;
                let a1729 = {
                    let a1726 = 2u64;
                    let a1727 = match v1.clone() {
                        Adt3::C0(h1725, _, _, _, _, _, _, _) => {
                            h1725
                        }
                    };
                    nat_mul(a1726, a1727)?
                };
                nat_add(a1728, a1729)?
            };
            let v9: u64 = {
                let a1730 = v7;
                let a1731 = v8;
                let a1732 = 0u64;
                nat_quot(a1730, a1731, a1732)
            };
            let v10: u64 = {
                let a1733 = v7;
                let a1734 = v8;
                let a1735 = 0u64;
                nat_rem(a1733, a1734, a1735)
            };
            let a1736 = v10;
            let a1737 = 0u64;
            if nat_eq(a1736, a1737) {
                Ok(Adt8::C4(19u64, 4u64, f106(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1741 = {
                    let a1738 = v10;
                    let a1739 = 2u64;
                    let a1740 = 0u64;
                    nat_rem(a1738, a1739, a1740)
                };
                let a1742 = 1u64;
                if nat_eq(a1741, a1742) {
                    Ok(Adt8::C4(6u64, 0u64, f106({
                        let a1751 = {
                            let a1744 = v9;
                            let a1745 = match v1.clone() {
                                Adt3::C0(h1743, _, _, _, _, _, _, _) => {
                                    h1743
                                }
                            };
                            nat_mul(a1744, a1745)?
                        };
                        let a1752 = {
                            let a1748 = {
                                let a1746 = v10;
                                let a1747 = 1u64;
                                nat_sub(a1746, a1747)
                            };
                            let a1749 = 2u64;
                            let a1750 = 0u64;
                            nat_quot(a1748, a1749, a1750)
                        };
                        nat_add(a1751, a1752)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt8::C4(4u64, 3u64, f106({
                        let a1761 = {
                            let a1754 = v4;
                            let a1755 = match v1.clone() {
                                Adt3::C0(h1753, _, _, _, _, _, _, _) => {
                                    h1753
                                }
                            };
                            nat_mul(a1754, a1755)?
                        };
                        let a1762 = {
                            let a1758 = {
                                let a1756 = v10;
                                let a1757 = 1u64;
                                nat_sub(a1756, a1757)
                            };
                            let a1759 = 2u64;
                            let a1760 = 0u64;
                            nat_quot(a1758, a1759, a1760)
                        };
                        nat_add(a1761, a1762)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f106({
                        let a1771 = {
                            let a1764 = v4;
                            let a1765 = match v1 {
                                Adt3::C0(h1763, _, _, _, _, _, _, _) => {
                                    h1763
                                }
                            };
                            nat_mul(a1764, a1765)?
                        };
                        let a1772 = {
                            let a1768 = {
                                let a1766 = v10;
                                let a1767 = 1u64;
                                nat_sub(a1766, a1767)
                            };
                            let a1769 = 2u64;
                            let a1770 = 0u64;
                            nat_quot(a1768, a1769, a1770)
                        };
                        nat_add(a1771, a1772)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f64(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt8> {
    let a1773 = v1;
    let a1774 = f121(v2.clone(), v4.clone())?;
    if nat_lt(a1773, a1774) {
        let a1777 = v1;
        let a1778 = {
            let a1775 = 2u64;
            let a1776 = f97(v4.clone())?;
            nat_mul(a1775, a1776)?
        };
        if nat_lt(a1777, a1778) {
            let a1782 = {
                let a1779 = v1;
                let a1780 = 2u64;
                let a1781 = 0u64;
                nat_rem(a1779, a1780, a1781)
            };
            let a1783 = 0u64;
            if nat_eq(a1782, a1783) {
                f122({
                    let a1784 = v1;
                    let a1785 = 2u64;
                    let a1786 = 0u64;
                    nat_quot(a1784, a1785, a1786)
                }, v4)
            } else {
                if f123(match v2 {
                    Adt3::C0(_, h1787, _, _, _, _, _, _) => {
                        h1787
                    }
                }, v5) {
                    Ok(Adt8::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt8::C0(Adt9::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1790 = v1;
                let a1791 = {
                    let a1788 = 2u64;
                    let a1789 = f97(v4.clone())?;
                    nat_mul(a1788, a1789)?
                };
                nat_sub(a1790, a1791)
            };
            let v7: u64 = {
                let a1795 = v6;
                let a1796 = {
                    let a1793 = 1u64;
                    let a1794 = match v2.clone() {
                        Adt3::C0(h1792, _, _, _, _, _, _, _) => {
                            h1792
                        }
                    };
                    nat_add(a1793, a1794)?
                };
                let a1797 = 0u64;
                nat_quot(a1795, a1796, a1797)
            };
            let v8: u64 = {
                let a1801 = v6;
                let a1802 = {
                    let a1799 = 1u64;
                    let a1800 = match v2.clone() {
                        Adt3::C0(h1798, _, _, _, _, _, _, _) => {
                            h1798
                        }
                    };
                    nat_add(a1799, a1800)?
                };
                let a1803 = 0u64;
                nat_rem(a1801, a1802, a1803)
            };
            let a1804 = v8;
            let a1805 = 0u64;
            if nat_eq(a1804, a1805) {
                f122(v7, v4)
            } else {
                f124(v0, v7, {
                    let a1806 = v8;
                    let a1807 = 1u64;
                    nat_sub(a1806, a1807)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt8::C0(Adt9::C1))
    }
}

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt8> {
    let v6: u64 = {
        let a1810 = match v4.clone() {
            Adt3::C0(h1808, _, _, _, _, _, _, _) => {
                h1808
            }
        };
        let a1811 = match v4.clone() {
            Adt3::C0(_, h1809, _, _, _, _, _, _) => {
                h1809
            }
        };
        nat_mul(a1810, a1811)?
    };
    let v7: u64 = {
        let a1812 = 2u64;
        let a1813 = v6;
        nat_mul(a1812, a1813)?
    };
    let v8: u64 = f125(match v3.clone() {
        Adt3::C0(_, h1814, _, _, _, _, _, _) => {
            h1814
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1815, _, _, _, _, _) => {
            h1815
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1816, _, _, _, _) => {
            h1816
        }
    });
    let a1817 = v0;
    let a1818 = v7;
    if nat_lt(a1817, a1818) {
        let a1822 = {
            let a1819 = v0;
            let a1820 = 2u64;
            let a1821 = 0u64;
            nat_rem(a1819, a1820, a1821)
        };
        let a1823 = 0u64;
        if nat_eq(a1822, a1823) {
            Ok(Adt8::C4(19u64, 5u64, f106({
                let a1824 = v0;
                let a1825 = 2u64;
                let a1826 = 0u64;
                nat_quot(a1824, a1825, a1826)
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
            let a1827 = v0;
            let a1828 = v7;
            nat_sub(a1827, a1828)
        };
        let v10: u64 = {
            let a1832 = v9;
            let a1833 = {
                let a1830 = 1u64;
                let a1831 = match v1.clone() {
                    Adt3::C0(h1829, _, _, _, _, _, _, _) => {
                        h1829
                    }
                };
                nat_add(a1830, a1831)?
            };
            let a1834 = 0u64;
            nat_quot(a1832, a1833, a1834)
        };
        let v11: u64 = {
            let a1838 = v9;
            let a1839 = {
                let a1836 = 1u64;
                let a1837 = match v1.clone() {
                    Adt3::C0(h1835, _, _, _, _, _, _, _) => {
                        h1835
                    }
                };
                nat_add(a1836, a1837)?
            };
            let a1840 = 0u64;
            nat_rem(a1838, a1839, a1840)
        };
        let a1841 = v11;
        let a1842 = 0u64;
        if nat_eq(a1841, a1842) {
            Ok(Adt8::C4(19u64, 5u64, f106({
                let a1843 = v10;
                let a1844 = v6;
                let a1845 = 0u64;
                nat_rem(a1843, a1844, a1845)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f126(v5, v8) {
                let v12: u64 = {
                    let a1851 = {
                        let a1847 = v10;
                        let a1848 = match v1.clone() {
                            Adt3::C0(h1846, _, _, _, _, _, _, _) => {
                                h1846
                            }
                        };
                        nat_mul(a1847, a1848)?
                    };
                    let a1852 = {
                        let a1849 = v11;
                        let a1850 = 1u64;
                        nat_sub(a1849, a1850)
                    };
                    nat_add(a1851, a1852)?
                };
                let a1853 = v5;
                let a1854 = 4294967295u64;
                if nat_eq(a1853, a1854) {
                    Ok(Adt8::C1(false, f106(v12, v1)?, f106(v12, v2)?))
                } else {
                    Ok(Adt8::C4(8u64, 0u64, f106(v12, v1)?, 4u64, f106({
                        let a1860 = {
                            let a1856 = v5;
                            let a1857 = match v3.clone() {
                                Adt3::C0(h1855, _, _, _, _, _, _, _) => {
                                    h1855
                                }
                            };
                            nat_mul(a1856, a1857)?
                        };
                        let a1861 = {
                            let a1858 = v11;
                            let a1859 = 1u64;
                            nat_sub(a1858, a1859)
                        };
                        nat_add(a1860, a1861)?
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
        let a1863 = v5;
        let a1864 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1862, _) => {
                h1862
            }
        };
        let a1865 = 0u64;
        nat_rem(a1863, a1864, a1865)
    };
    let v7: u64 = {
        let a1867 = v5;
        let a1868 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1866, _) => {
                h1866
            }
        };
        let a1869 = 0u64;
        nat_quot(a1867, a1868, a1869)
    };
    let v8: Option<u64> = f59(match v3.clone() {
        Adt5::C0(h1870, _, _, _, _, _, _, _) => {
            h1870
        }
    }, match v1.clone() {
        Adt3::C0(h1871, _, _, _, _, _, _, _) => {
            h1871
        }
    }, f58(v0, v2.clone(), 1u64), {
        let a1875 = v4;
        let a1876 = {
            let m1874 = match v3.clone() {
                Adt5::C0(h1873, _, _, _, _, _, _, _) => {
                    h1873
                }
            };
            match m1874 {
                Adt6::C0(h1872, _, _, _) => {
                    h1872
                }
            }
        };
        let a1877 = 0u64;
        nat_rem(a1875, a1876, a1877)
    })?;
    let v9: u64 = {
        let a1881 = v4;
        let a1882 = {
            let m1880 = match v3.clone() {
                Adt5::C0(h1879, _, _, _, _, _, _, _) => {
                    h1879
                }
            };
            match m1880 {
                Adt6::C0(h1878, _, _, _) => {
                    h1878
                }
            }
        };
        let a1883 = 0u64;
        nat_quot(a1881, a1882, a1883)
    };
    let v10: Option<u64> = f59(match v3.clone() {
        Adt5::C0(_, h1884, _, _, _, _, _, _) => {
            h1884
        }
    }, match v1.clone() {
        Adt3::C0(_, h1885, _, _, _, _, _, _) => {
            h1885
        }
    }, f58(v0, v2.clone(), 2u64), {
        let a1889 = v9;
        let a1890 = {
            let m1888 = match v3.clone() {
                Adt5::C0(_, h1887, _, _, _, _, _, _) => {
                    h1887
                }
            };
            match m1888 {
                Adt6::C0(h1886, _, _, _) => {
                    h1886
                }
            }
        };
        let a1891 = 0u64;
        nat_rem(a1889, a1890, a1891)
    })?;
    let v11: u64 = {
        let a1895 = v9;
        let a1896 = {
            let m1894 = match v3.clone() {
                Adt5::C0(_, h1893, _, _, _, _, _, _) => {
                    h1893
                }
            };
            match m1894 {
                Adt6::C0(h1892, _, _, _) => {
                    h1892
                }
            }
        };
        let a1897 = 0u64;
        nat_quot(a1895, a1896, a1897)
    };
    let v12: Option<u64> = f59(match v3.clone() {
        Adt5::C0(_, _, h1898, _, _, _, _, _) => {
            h1898
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1899, _, _, _, _, _) => {
            h1899
        }
    }, v6, {
        let a1903 = v11;
        let a1904 = {
            let m1902 = match v3.clone() {
                Adt5::C0(_, _, h1901, _, _, _, _, _) => {
                    h1901
                }
            };
            match m1902 {
                Adt6::C0(h1900, _, _, _) => {
                    h1900
                }
            }
        };
        let a1905 = 0u64;
        nat_rem(a1903, a1904, a1905)
    })?;
    let v13: u64 = {
        let a1909 = v11;
        let a1910 = {
            let m1908 = match v3.clone() {
                Adt5::C0(_, _, h1907, _, _, _, _, _) => {
                    h1907
                }
            };
            match m1908 {
                Adt6::C0(h1906, _, _, _) => {
                    h1906
                }
            }
        };
        let a1911 = 0u64;
        nat_quot(a1909, a1910, a1911)
    };
    let v17: Adt11 = {
        let m1912 = v8;
        match m1912 {
            None => {
                Adt11::C2(0u64)
            }
            Some(v14) => {
                let m1913 = v10;
                match m1913 {
                    None => {
                        Adt11::C2(0u64)
                    }
                    Some(v15) => {
                        let m1914 = v12;
                        match m1914 {
                            None => {
                                Adt11::C2(0u64)
                            }
                            Some(v16) => {
                                Adt11::C0(1u64, {
                                    let a1923 = {
                                        let a1918 = f57(v1, v14, v15, v16, 0u64)?;
                                        let a1919 = {
                                            let a1916 = v7;
                                            let a1917 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h1915, _, _, _) => {
                                                    h1915
                                                }
                                            };
                                            nat_mul(a1916, a1917)?
                                        };
                                        nat_add(a1918, a1919)?
                                    };
                                    let a1924 = {
                                        let a1921 = v13;
                                        let a1922 = match v3.clone() {
                                            Adt5::C0(_, _, _, h1920, _, _, _, _) => {
                                                h1920
                                            }
                                        };
                                        nat_mul(a1921, a1922)?
                                    };
                                    nat_add(a1923, a1924)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt8::C7(Adt10::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h1925, _, _) => {
            h1925
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h1926) => {
            h1926
        }
    }, true), v17, Adt11::C2(0u64), Adt12::C1(f106(v0, v2)?), {
        let a1928 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h1927, _, _) => {
                h1927
            }
        };
        let a1929 = 0u64;
        if nat_eq(a1928, a1929) {
            4u64
        } else {
            2u64
        }
    }, {
        let a1931 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h1930) => {
                h1930
            }
        };
        let a1932 = 0u64;
        if nat_eq(a1931, a1932) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt8> {
    let v8: u64 = {
        let a1935 = v6;
        let a1936 = {
            let a1933 = 3u64;
            let a1934 = v5;
            nat_add(a1933, a1934)?
        };
        nat_sub(a1935, a1936)
    };
    let a1938 = v0;
    let a1939 = f29(v3, match v1.clone() {
        Adt3::C0(h1937, _, _, _, _, _, _, _) => {
            h1937
        }
    }, v5, v8)?;
    if nat_lt(a1938, a1939) {
        let v9: u64 = f78(match v1.clone() {
            Adt3::C0(h1940, _, _, _, _, _, _, _) => {
                h1940
            }
        }, v5, v8)?;
        let v10: u64 = f79(v3, match v1.clone() {
            Adt3::C0(h1941, _, _, _, _, _, _, _) => {
                h1941
            }
        }, v5)?;
        let a1942 = v0;
        let a1943 = v9;
        if nat_lt(a1942, a1943) {
            f127(v0, v2, f23(v3), {
                let a1944 = 3u64;
                let a1945 = v5;
                nat_add(a1944, a1945)?
            }, v7)
        } else {
            let a1948 = v0;
            let a1949 = {
                let a1946 = v9;
                let a1947 = v10;
                nat_add(a1946, a1947)?
            };
            if nat_lt(a1948, a1949) {
                let v11: u64 = {
                    let a1950 = v0;
                    let a1951 = v9;
                    nat_sub(a1950, a1951)
                };
                let v12: u64 = f128(v3, match v1.clone() {
                    Adt3::C0(h1952, _, _, _, _, _, _, _) => {
                        h1952
                    }
                })?;
                f129({
                    let a1953 = v11;
                    let a1954 = v12;
                    let a1955 = 0u64;
                    nat_rem(a1953, a1954, a1955)
                }, {
                    let a1956 = v11;
                    let a1957 = v12;
                    let a1958 = 0u64;
                    nat_quot(a1956, a1957, a1958)
                }, v1, v3, v4, v7)
            } else {
                f130({
                    let a1961 = v0;
                    let a1962 = {
                        let a1959 = v9;
                        let a1960 = v10;
                        nat_add(a1959, a1960)?
                    };
                    nat_sub(a1961, a1962)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt8::C0(Adt9::C1))
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt8> {
    let a1963 = v0;
    let a1964 = f15(v1)?;
    if nat_lt(a1963, a1964) {
        let a1965 = v0;
        let a1966 = 4u64;
        if nat_lt(a1965, a1966) {
            let a1967 = v0;
            let a1968 = 0u64;
            if nat_eq(a1967, a1968) {
                Ok(Adt8::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1969 = v0;
                let a1970 = 1u64;
                if nat_eq(a1969, a1970) {
                    let a1971 = v5;
                    let a1972 = 0u64;
                    if nat_eq(a1971, a1972) {
                        Ok(Adt8::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt8::C0(Adt9::C1))
                    }
                } else {
                    let a1973 = v0;
                    let a1974 = 2u64;
                    if nat_eq(a1973, a1974) {
                        Ok(Adt8::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt8::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1975 = v0;
            let a1976 = 4u64;
            if if nat_eq(a1975, a1976) {
                let a1977 = f131(v5)?;
                let a1978 = v1;
                !nat_eq(a1977, a1978)
            } else {
                false
            } {
                Ok(Adt8::C0(Adt9::C1))
            } else {
                let v6: u64 = {
                    let a1981 = {
                        let a1979 = v0;
                        let a1980 = 4u64;
                        nat_sub(a1979, a1980)
                    };
                    let a1982 = 3u64;
                    let a1983 = 0u64;
                    nat_quot(a1981, a1982, a1983)
                };
                let v7: u64 = {
                    let a1986 = {
                        let a1984 = v0;
                        let a1985 = 4u64;
                        nat_sub(a1984, a1985)
                    };
                    let a1987 = 3u64;
                    let a1988 = 0u64;
                    nat_rem(a1986, a1987, a1988)
                };
                let a1989 = v7;
                let a1990 = 0u64;
                if nat_eq(a1989, a1990) {
                    Ok(Adt8::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1991 = v7;
                    let a1992 = 1u64;
                    if nat_eq(a1991, a1992) {
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
    let a1993 = v1;
    let a1994 = 0u64;
    if nat_eq(a1993, a1994) {
        if v0 {
            Adt10::C2(v3, v4, false)
        } else {
            Adt10::C0(v2)
        }
    } else {
        let a1995 = v1;
        let a1996 = 1u64;
        if nat_eq(a1995, a1996) {
            Adt10::C1(v2)
        } else {
            let a1997 = v1;
            let a1998 = 2u64;
            if nat_eq(a1997, a1998) {
                Adt10::C5(v2)
            } else {
                let a1999 = v1;
                let a2000 = 3u64;
                if nat_eq(a1999, a2000) {
                    Adt10::C3(v2, v2, false)
                } else {
                    let a2001 = v1;
                    let a2002 = 4u64;
                    if nat_eq(a2001, a2002) {
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
    let a2003 = v0;
    let a2004 = 0u64;
    if nat_eq(a2003, a2004) {
        Adt11::C0(0u64, v1)
    } else {
        let a2005 = v0;
        let a2006 = 1u64;
        if nat_eq(a2005, a2006) {
            Adt11::C1(v1)
        } else {
            let a2007 = v0;
            let a2008 = 2u64;
            if nat_eq(a2007, a2008) {
                Adt11::C2(v1)
            } else {
                let a2009 = v0;
                let a2010 = 3u64;
                if nat_eq(a2009, a2010) {
                    Adt11::C3(v1)
                } else {
                    let a2011 = v0;
                    let a2012 = 13u64;
                    if nat_lt(a2011, a2012) {
                        Adt11::C0({
                            let a2013 = v0;
                            let a2014 = 3u64;
                            nat_sub(a2013, a2014)
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
    let a2015 = v0;
    let a2016 = 1u64;
    if nat_eq(a2015, a2016) {
        Adt12::C0(v1)
    } else {
        let a2017 = v0;
        let a2018 = 3u64;
        if if nat_eq(a2017, a2018) {
            true
        } else {
            let a2019 = v0;
            let a2020 = 6u64;
            nat_eq(a2019, a2020)
        } {
            Adt12::C1(v1)
        } else {
            Adt12::C2
        }
    }
}

pub fn f72(v0: u64) -> u64 {
    let a2021 = v0;
    let a2022 = 2147483648u64;
    if nat_lt(a2021, a2022) {
        v0
    } else {
        0u64
    }
}

pub fn f73(v0: u64) -> u64 {
    let a2023 = v0;
    let a2024 = 2147483648u64;
    if nat_lt(a2023, a2024) {
        0u64
    } else {
        let a2025 = 4294967296u64;
        let a2026 = v0;
        nat_sub(a2025, a2026)
    }
}

pub fn f74(v0: u64) -> u64 {
    let v1: u64 = {
        let a2027 = v0;
        let a2028 = 4294967296u64;
        let a2029 = 0u64;
        nat_rem(a2027, a2028, a2029)
    };
    let a2030 = v1;
    let a2031 = 2147483648u64;
    if nat_lt(a2030, a2031) {
        v1
    } else {
        0u64
    }
}

pub fn f75(v0: u64) -> R<u64> {
    let a2034 = 2u64;
    let a2035 = {
        let a2032 = 2u64;
        let a2033 = v0;
        nat_mul(a2032, a2033)?
    };
    nat_add(a2034, a2035)
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
    let a2036 = v2;
    let a2037 = 0u64;
    if nat_eq(a2036, a2037) {
        Ok(0u64)
    } else {
        let a2040 = 2u64;
        let a2041 = {
            let a2038 = v0;
            let a2039 = v1;
            nat_mul(a2038, a2039)?
        };
        nat_mul(a2040, a2041)
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2042 = v2;
    let a2043 = f128(v0, v1)?;
    nat_mul(a2042, a2043)
}

pub fn f80(v0: u64) -> u64 {
    let a2044 = v0;
    let a2045 = 30u64;
    if nat_eq(a2044, a2045) {
        3u64
    } else {
        1u64
    }
}

pub fn f81(v0: u64) -> R<u64> {
    let a2048 = 5u64;
    let a2049 = {
        let a2046 = 4u64;
        let a2047 = v0;
        nat_mul(a2046, a2047)?
    };
    nat_add(a2048, a2049)
}

pub fn f82(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt9> {
    if f134(match v0.clone() {
        Adt2::C0(h2050, _, _, _, _, _, _, _, _, _, _) => {
            h2050
        }
    }, match v0.clone() {
        Adt2::C0(_, h2051, _, _, _, _, _, _, _, _, _) => {
            h2051
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2052, _, _, _, _, _, _, _, _) => {
            h2052
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2053, _, _, _, _, _, _, _) => {
            h2053
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2054, _, _, _, _, _, _) => {
            h2054
        }
    }, match v1.clone() {
        Adt2::C0(h2055, _, _, _, _, _, _, _, _, _, _) => {
            h2055
        }
    }, match v1.clone() {
        Adt2::C0(_, h2056, _, _, _, _, _, _, _, _, _) => {
            h2056
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2057, _, _, _, _, _, _, _, _) => {
            h2057
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2058, _, _, _, _, _, _, _) => {
            h2058
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2059, _, _, _, _, _, _) => {
            h2059
        }
    }, match v2.clone() {
        Adt2::C0(h2060, _, _, _, _, _, _, _, _, _, _) => {
            h2060
        }
    }, match v2.clone() {
        Adt2::C0(_, h2061, _, _, _, _, _, _, _, _, _) => {
            h2061
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2062, _, _, _, _, _, _, _, _) => {
            h2062
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2063, _, _, _, _, _, _, _) => {
            h2063
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2064, _, _, _, _, _, _) => {
            h2064
        }
    }) {
        let m2066 = f86(match v0.clone() {
            Adt2::C0(h2065, _, _, _, _, _, _, _, _, _, _) => {
                h2065
            }
        });
        match m2066 {
            Adt13::C0 => {
                Ok(Adt9::C0)
            }
            Adt13::C1(v3) => {
                let a2072 = {
                    let a2069 = match v0.clone() {
                        Adt2::C0(_, h2067, _, _, _, _, _, _, _, _, _) => {
                            h2067
                        }
                    };
                    let a2070 = match v3.clone() {
                        Adt14::C0(_, h2068, _) => {
                            h2068
                        }
                    };
                    let a2071 = 0u64;
                    nat_rem(a2069, a2070, a2071)
                };
                let a2073 = 0u64;
                if if nat_eq(a2072, a2073) {
                    if f135(match v0.clone() {
                        Adt2::C0(_, h2074, _, _, _, _, _, _, _, _, _) => {
                            h2074
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2075, _, _, _, _, _, _, _, _, _) => {
                            h2075
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2076, _, _, _, _, _, _, _, _) => {
                            h2076
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2077, _, _, _, _, _, _, _) => {
                            h2077
                        }
                    }) {
                        true
                    } else {
                        let a2079 = 4u64;
                        let a2080 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2078, _, _, _, _, _) => {
                                h2078
                            }
                        };
                        if if nat_le(a2079, a2080) {
                            let a2082 = match v3.clone() {
                                Adt14::C0(_, h2081, _) => {
                                    h2081
                                }
                            };
                            let a2083 = 1u64;
                            if nat_eq(a2082, a2083) {
                                true
                            } else {
                                let a2086 = match v3.clone() {
                                    Adt14::C0(_, _, h2084) => {
                                        h2084
                                    }
                                };
                                let a2087 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2085, _, _, _, _, _) => {
                                        h2085
                                    }
                                };
                                nat_le(a2086, a2087)
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
                        Adt14::C0(_, h2088, _) => {
                            h2088
                        }
                    });
                    Ok(Adt9::C4(if f135(match v0.clone() {
                        Adt2::C0(_, h2089, _, _, _, _, _, _, _, _, _) => {
                            h2089
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2090, _, _, _, _, _, _, _, _, _) => {
                            h2090
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2091, _, _, _, _, _, _, _, _) => {
                            h2091
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2092, _, _, _, _, _, _, _) => {
                            h2092
                        }
                    }) {
                        0u64
                    } else {
                        f121(v4.clone(), f16(v1.clone(), 1u64))?
                    }, 4u64, v4, f16(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2093, _, _, _, _, _, _, _, _, _, _) => {
                            h2093
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
        Adt2::C0(h2094, _, _, _, _, _, _, _, _, _, _) => {
            h2094
        }
    }) {
        let a2096 = match v1.clone() {
            Adt2::C0(h2095, _, _, _, _, _, _, _, _, _, _) => {
                h2095
            }
        };
        let a2097 = 26u64;
        nat_eq(a2096, a2097)
    } else {
        false
    } {
        let a2099 = match v2.clone() {
            Adt2::C0(h2098, _, _, _, _, _, _, _, _, _, _) => {
                h2098
            }
        };
        let a2100 = 0u64;
        nat_eq(a2099, a2100)
    } else {
        false
    } {
        let a2103 = match v0.clone() {
            Adt2::C0(_, h2101, _, _, _, _, _, _, _, _, _) => {
                h2101
            }
        };
        let a2104 = match v2.clone() {
            Adt2::C0(_, h2102, _, _, _, _, _, _, _, _, _) => {
                h2102
            }
        };
        nat_eq(a2103, a2104)
    } else {
        false
    } {
        let a2107 = match v1.clone() {
            Adt2::C0(_, h2105, _, _, _, _, _, _, _, _, _) => {
                h2105
            }
        };
        let a2108 = match v0.clone() {
            Adt2::C0(_, _, h2106, _, _, _, _, _, _, _, _) => {
                h2106
            }
        };
        nat_le(a2107, a2108)
    } else {
        false
    } {
        let a2110 = match v0.clone() {
            Adt2::C0(_, _, _, h2109, _, _, _, _, _, _, _) => {
                h2109
            }
        };
        let a2111 = 1u64;
        nat_eq(a2110, a2111)
    } else {
        false
    } {
        let a2113 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2112, _, _, _, _, _, _) => {
                h2112
            }
        };
        let a2114 = 1u64;
        nat_eq(a2113, a2114)
    } else {
        false
    } {
        let a2116 = match v2.clone() {
            Adt2::C0(_, _, _, h2115, _, _, _, _, _, _, _) => {
                h2115
            }
        };
        let a2117 = 1u64;
        nat_eq(a2116, a2117)
    } else {
        false
    } {
        let a2119 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2118, _, _, _, _, _, _) => {
                h2118
            }
        };
        let a2120 = 1u64;
        nat_eq(a2119, a2120)
    } else {
        false
    } {
        let a2122 = match v1.clone() {
            Adt2::C0(_, _, h2121, _, _, _, _, _, _, _, _) => {
                h2121
            }
        };
        let a2123 = 1u64;
        nat_eq(a2122, a2123)
    } else {
        false
    } {
        let a2125 = match v1.clone() {
            Adt2::C0(_, _, _, h2124, _, _, _, _, _, _, _) => {
                h2124
            }
        };
        let a2126 = 1u64;
        nat_eq(a2125, a2126)
    } else {
        false
    } {
        let a2128 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2127, _, _, _, _, _, _) => {
                h2127
            }
        };
        let a2129 = 1u64;
        nat_eq(a2128, a2129)
    } else {
        false
    } {
        if f27(match v2.clone() {
            Adt2::C0(_, h2130, _, _, _, _, _, _, _, _, _) => {
                h2130
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2131, _, _, _, _, _, _, _, _) => {
                h2131
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2132, _, _, _, _, _, _, _) => {
                h2132
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2133, _, _, _, _, _, _) => {
                h2133
            }
        }) {
            Ok(Adt9::C4(0u64, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2135 = match v1.clone() {
                Adt2::C0(_, h2134, _, _, _, _, _, _, _, _, _) => {
                    h2134
                }
            };
            let a2136 = 0u64;
            if nat_eq(a2135, a2136) {
                Ok(Adt9::C4(f12(v2.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2138 = match v0.clone() {
                    Adt2::C0(_, h2137, _, _, _, _, _, _, _, _, _) => {
                        h2137
                    }
                };
                let a2139 = 2147483647u64;
                if if if if if if nat_le(a2138, a2139) {
                    let a2141 = match v1.clone() {
                        Adt2::C0(_, h2140, _, _, _, _, _, _, _, _, _) => {
                            h2140
                        }
                    };
                    let a2142 = 2147483647u64;
                    nat_le(a2141, a2142)
                } else {
                    false
                } {
                    let a2144 = 4u64;
                    let a2145 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2143, _, _, _, _, _) => {
                            h2143
                        }
                    };
                    nat_le(a2144, a2145)
                } else {
                    false
                } {
                    let a2151 = {
                        let a2148 = match v2.clone() {
                            Adt2::C0(_, h2146, _, _, _, _, _, _, _, _, _) => {
                                h2146
                            }
                        };
                        let a2149 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2147, _, _, _, _, _) => {
                                h2147
                            }
                        };
                        nat_mul(a2148, a2149)?
                    };
                    let a2152 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2150, _, _, _, _) => {
                            h2150
                        }
                    };
                    nat_le(a2151, a2152)
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
                        let a2166 = {
                            let a2156 = {
                                let a2154 = 2u64;
                                let a2155 = match v1.clone() {
                                    Adt2::C0(_, h2153, _, _, _, _, _, _, _, _, _) => {
                                        h2153
                                    }
                                };
                                nat_mul(a2154, a2155)?
                            };
                            let a2157 = f12(v2.clone(), 1u64)?;
                            nat_add(a2156, a2157)?
                        };
                        let a2167 = {
                            let a2164 = match v1.clone() {
                                Adt2::C0(_, h2158, _, _, _, _, _, _, _, _, _) => {
                                    h2158
                                }
                            };
                            let a2165 = {
                                let a2162 = 1u64;
                                let a2163 = {
                                    let a2160 = 2u64;
                                    let a2161 = match v0.clone() {
                                        Adt2::C0(_, h2159, _, _, _, _, _, _, _, _, _) => {
                                            h2159
                                        }
                                    };
                                    nat_mul(a2160, a2161)?
                                };
                                nat_add(a2162, a2163)?
                            };
                            nat_mul(a2164, a2165)?
                        };
                        nat_add(a2166, a2167)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2168, _, _, _, _, _, _, _, _, _) => {
                            h2168
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2169, _, _, _, _, _, _, _, _, _) => {
                            h2169
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2170, _, _, _, _, _) => {
                            h2170
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2171, _, _, _, _) => {
                            h2171
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2172, _, _, _) => {
                            h2172
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2173, _, _) => {
                            h2173
                        }
                    }), f16(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2174, _, _, _, _, _, _, _, _, _, _) => {
                            h2174
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
    let a2177 = match v1.clone() {
        Adt2::C0(h2175, _, _, _, _, _, _, _, _, _, _) => {
            h2175
        }
    };
    let a2178 = match v3.clone() {
        Adt2::C0(h2176, _, _, _, _, _, _, _, _, _, _) => {
            h2176
        }
    };
    if if nat_eq(a2177, a2178) {
        let a2179 = v0;
        let a2180 = 33u64;
        if nat_eq(a2179, a2180) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2182 = f86(match v1.clone() {
            Adt2::C0(h2181, _, _, _, _, _, _, _, _, _, _) => {
                h2181
            }
        });
        match m2182 {
            Adt13::C0 => {
                Ok(Adt9::C0)
            }
            Adt13::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt14::C0(_, h2183, _) => {
                        h2183
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a2188 = {
                    let a2185 = match v1.clone() {
                        Adt2::C0(_, h2184, _, _, _, _, _, _, _, _, _) => {
                            h2184
                        }
                    };
                    let a2186 = v5;
                    let a2187 = 1u64;
                    nat_rem(a2185, a2186, a2187)
                };
                let a2189 = 0u64;
                if if if nat_eq(a2188, a2189) {
                    let a2194 = {
                        let a2191 = match v3.clone() {
                            Adt2::C0(_, h2190, _, _, _, _, _, _, _, _, _) => {
                                h2190
                            }
                        };
                        let a2192 = v5;
                        let a2193 = 1u64;
                        nat_rem(a2191, a2192, a2193)
                    };
                    let a2195 = 0u64;
                    nat_eq(a2194, a2195)
                } else {
                    false
                } {
                    let a2196 = v6;
                    let a2197 = f12(v3.clone(), v5)?;
                    nat_eq(a2196, a2197)
                } else {
                    false
                } {
                    Ok(Adt9::C4(v6, match v4 {
                        Adt14::C0(_, _, h2198) => {
                            h2198
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
    let a2199 = v0;
    let a2200 = 0u64;
    if if nat_eq(a2199, a2200) {
        true
    } else {
        let a2201 = v0;
        let a2202 = 1u64;
        nat_eq(a2201, a2202)
    } {
        true
    } else {
        let a2203 = v0;
        let a2204 = 30u64;
        nat_eq(a2203, a2204)
    }
}

pub fn f86(v0: u64) -> Adt13 {
    let a2205 = v0;
    let a2206 = 0u64;
    if nat_eq(a2205, a2206) {
        Adt13::C1(Adt14::C0(0u64, 1u64, 4u64))
    } else {
        let a2207 = v0;
        let a2208 = 1u64;
        if nat_eq(a2207, a2208) {
            Adt13::C1(Adt14::C0(1u64, 1u64, 2u64))
        } else {
            let a2209 = v0;
            let a2210 = 2u64;
            if nat_eq(a2209, a2210) {
                Adt13::C1(Adt14::C0(2u64, 32u64, 18u64))
            } else {
                let a2211 = v0;
                let a2212 = 3u64;
                if nat_eq(a2211, a2212) {
                    Adt13::C1(Adt14::C0(3u64, 32u64, 20u64))
                } else {
                    let a2213 = v0;
                    let a2214 = 6u64;
                    if nat_eq(a2213, a2214) {
                        Adt13::C1(Adt14::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2215 = v0;
                        let a2216 = 7u64;
                        if nat_eq(a2215, a2216) {
                            Adt13::C1(Adt14::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2217 = v0;
                            let a2218 = 8u64;
                            if nat_eq(a2217, a2218) {
                                Adt13::C1(Adt14::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2219 = v0;
                                let a2220 = 9u64;
                                if nat_eq(a2219, a2220) {
                                    Adt13::C1(Adt14::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2221 = v0;
                                    let a2222 = 10u64;
                                    if nat_eq(a2221, a2222) {
                                        Adt13::C1(Adt14::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2223 = v0;
                                        let a2224 = 11u64;
                                        if nat_eq(a2223, a2224) {
                                            Adt13::C1(Adt14::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2225 = v0;
                                            let a2226 = 12u64;
                                            if nat_eq(a2225, a2226) {
                                                Adt13::C1(Adt14::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2227 = v0;
                                                let a2228 = 13u64;
                                                if nat_eq(a2227, a2228) {
                                                    Adt13::C1(Adt14::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2229 = v0;
                                                    let a2230 = 14u64;
                                                    if nat_eq(a2229, a2230) {
                                                        Adt13::C1(Adt14::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2231 = v0;
                                                        let a2232 = 15u64;
                                                        if nat_eq(a2231, a2232) {
                                                            Adt13::C1(Adt14::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2233 = v0;
                                                            let a2234 = 16u64;
                                                            if nat_eq(a2233, a2234) {
                                                                Adt13::C1(Adt14::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2235 = v0;
                                                                let a2236 = 17u64;
                                                                if nat_eq(a2235, a2236) {
                                                                    Adt13::C1(Adt14::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2237 = v0;
                                                                    let a2238 = 18u64;
                                                                    if nat_eq(a2237, a2238) {
                                                                        Adt13::C1(Adt14::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2239 = v0;
                                                                        let a2240 = 19u64;
                                                                        if nat_eq(a2239, a2240) {
                                                                            Adt13::C1(Adt14::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2241 = v0;
                                                                            let a2242 = 20u64;
                                                                            if nat_eq(a2241, a2242) {
                                                                                Adt13::C1(Adt14::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2243 = v0;
                                                                                let a2244 = 21u64;
                                                                                if nat_eq(a2243, a2244) {
                                                                                    Adt13::C1(Adt14::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2245 = v0;
                                                                                    let a2246 = 22u64;
                                                                                    if nat_eq(a2245, a2246) {
                                                                                        Adt13::C1(Adt14::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2247 = v0;
                                                                                        let a2248 = 23u64;
                                                                                        if nat_eq(a2247, a2248) {
                                                                                            Adt13::C1(Adt14::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2249 = v0;
                                                                                            let a2250 = 24u64;
                                                                                            if nat_eq(a2249, a2250) {
                                                                                                Adt13::C1(Adt14::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2251 = v0;
                                                                                                let a2252 = 25u64;
                                                                                                if nat_eq(a2251, a2252) {
                                                                                                    Adt13::C1(Adt14::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2253 = v0;
                                                                                                    let a2254 = 26u64;
                                                                                                    if nat_eq(a2253, a2254) {
                                                                                                        Adt13::C1(Adt14::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2255 = v0;
                                                                                                        let a2256 = 27u64;
                                                                                                        if nat_eq(a2255, a2256) {
                                                                                                            Adt13::C1(Adt14::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2257 = v0;
                                                                                                            let a2258 = 28u64;
                                                                                                            if nat_eq(a2257, a2258) {
                                                                                                                Adt13::C1(Adt14::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2259 = v0;
                                                                                                                let a2260 = 29u64;
                                                                                                                if nat_eq(a2259, a2260) {
                                                                                                                    Adt13::C1(Adt14::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2261 = v0;
                                                                                                                    let a2262 = 30u64;
                                                                                                                    if nat_eq(a2261, a2262) {
                                                                                                                        Adt13::C1(Adt14::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2263 = v0;
                                                                                                                        let a2264 = 34u64;
                                                                                                                        if nat_eq(a2263, a2264) {
                                                                                                                            Adt13::C1(Adt14::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2265 = v0;
                                                                                                                            let a2266 = 35u64;
                                                                                                                            if nat_eq(a2265, a2266) {
                                                                                                                                Adt13::C1(Adt14::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2267 = v0;
                                                                                                                                let a2268 = 39u64;
                                                                                                                                if nat_eq(a2267, a2268) {
                                                                                                                                    Adt13::C1(Adt14::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2269 = v0;
                                                                                                                                    let a2270 = 40u64;
                                                                                                                                    if nat_eq(a2269, a2270) {
                                                                                                                                        Adt13::C1(Adt14::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2271 = v0;
                                                                                                                                        let a2272 = 41u64;
                                                                                                                                        if nat_eq(a2271, a2272) {
                                                                                                                                            Adt13::C1(Adt14::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2273 = v0;
                                                                                                                                            let a2274 = 42u64;
                                                                                                                                            if nat_eq(a2273, a2274) {
                                                                                                                                                Adt13::C1(Adt14::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2275 = v0;
                                                                                                                                                let a2276 = 43u64;
                                                                                                                                                if nat_eq(a2275, a2276) {
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
        let a2280 = {
            let a2277 = v0;
            let a2278 = v4;
            let a2279 = 1u64;
            nat_rem(a2277, a2278, a2279)
        };
        let a2281 = 0u64;
        if if if nat_eq(a2280, a2281) {
            let a2285 = {
                let a2282 = v1;
                let a2283 = v5;
                let a2284 = 1u64;
                nat_rem(a2282, a2283, a2284)
            };
            let a2286 = 0u64;
            nat_eq(a2285, a2286)
        } else {
            false
        } {
            let a2290 = {
                let a2287 = v2;
                let a2288 = v6;
                let a2289 = 1u64;
                nat_rem(a2287, a2288, a2289)
            };
            let a2291 = 0u64;
            nat_eq(a2290, a2291)
        } else {
            false
        } {
            let a2295 = {
                let a2292 = v3;
                let a2293 = v7;
                let a2294 = 1u64;
                nat_rem(a2292, a2293, a2294)
            };
            let a2296 = 0u64;
            nat_eq(a2295, a2296)
        } else {
            false
        }
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2297 = v0;
    let a2298 = v1;
    if nat_eq(a2297, a2298) {
        let a2301 = v4;
        let a2302 = {
            let a2299 = v2;
            let a2300 = v3;
            nat_add(a2299, a2300)?
        };
        Ok(nat_eq(a2301, a2302))
    } else {
        let a2303 = v2;
        let a2304 = v3;
        if nat_eq(a2303, a2304) {
            let a2305 = v4;
            let a2306 = v2;
            Ok(nat_eq(a2305, a2306))
        } else {
            Ok(false)
        }
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2309 = {
            let a2307 = v0;
            let a2308 = f72(v1);
            nat_add(a2307, a2308)?
        };
        let a2310 = f72(v2);
        nat_add(a2309, a2310)?
    };
    let v5: u64 = {
        let a2311 = f73(v1);
        let a2312 = f73(v2);
        nat_add(a2311, a2312)?
    };
    let a2313 = v1;
    let a2314 = 4294967295u64;
    if if nat_le(a2313, a2314) {
        let a2315 = v2;
        let a2316 = 4294967295u64;
        nat_le(a2315, a2316)
    } else {
        false
    } {
        let a2317 = v5;
        let a2318 = v4;
        if nat_le(a2317, a2318) {
            let a2321 = v3;
            let a2322 = {
                let a2319 = v4;
                let a2320 = v5;
                nat_sub(a2319, a2320)
            };
            Ok(nat_eq(a2321, a2322))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f90(v0: u64, v1: u64) -> R<bool> {
    let a2323 = v1;
    let a2324 = 4294967295u64;
    if nat_le(a2323, a2324) {
        let a2327 = {
            let a2325 = f72(v1);
            let a2326 = f73(v1);
            nat_add(a2325, a2326)?
        };
        let a2328 = v0;
        Ok(nat_lt(a2327, a2328))
    } else {
        Ok(false)
    }
}

pub fn f91(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f136(v0.clone(), false)?;
    let v6: u64 = f136(v0, true)?;
    let a2329 = v6;
    let a2330 = v1;
    if if nat_le(a2329, a2330) {
        let a2337 = {
            let a2335 = {
                let a2331 = v1;
                let a2332 = v5;
                nat_add(a2331, a2332)?
            };
            let a2336 = {
                let a2333 = v3;
                let a2334 = 0u64;
                if nat_eq(a2333, a2334) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2335, a2336)?
        };
        let a2338 = v2;
        nat_le(a2337, a2338)
    } else {
        false
    } {
        if v4 {
            let a2343 = {
                let a2341 = {
                    let a2339 = v1;
                    let a2340 = v5;
                    nat_add(a2339, a2340)?
                };
                let a2342 = v6;
                nat_sub(a2341, a2342)
            };
            let a2344 = v2;
            Ok(nat_lt(a2343, a2344))
        } else {
            let a2349 = {
                let a2347 = {
                    let a2345 = v1;
                    let a2346 = v5;
                    nat_add(a2345, a2346)?
                };
                let a2348 = v6;
                nat_sub(a2347, a2348)
            };
            let a2350 = v2;
            Ok(nat_le(a2349, a2350))
        }
    } else {
        Ok(false)
    }
}

pub fn f92(v0: u64, v1: u64) -> bool {
    let a2351 = v0;
    let a2352 = 0u64;
    if if nat_eq(a2351, a2352) {
        true
    } else {
        let a2353 = v0;
        let a2354 = 1u64;
        nat_eq(a2353, a2354)
    } {
        let a2355 = v1;
        let a2356 = 4294967296u64;
        nat_lt(a2355, a2356)
    } else {
        false
    }
}

pub fn f93(v0: u64, v1: u64) -> R<u64> {
    let a2361 = {
        let a2359 = v0;
        let a2360 = {
            let a2357 = v1;
            let a2358 = 1u64;
            nat_sub(a2357, a2358)
        };
        nat_add(a2359, a2360)?
    };
    let a2362 = v1;
    let a2363 = 0u64;
    Ok(nat_quot(a2361, a2362, a2363))
}

pub fn f94(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f27(match v0.clone() {
        Adt2::C0(_, h2364, _, _, _, _, _, _, _, _, _) => {
            h2364
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2365, _, _, _, _, _, _, _, _) => {
            h2365
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2366, _, _, _, _, _, _, _) => {
            h2366
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2367, _, _, _, _, _, _) => {
            h2367
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2370 = match v1.clone() {
                Adt2::C0(_, h2368, _, _, _, _, _, _, _, _, _) => {
                    h2368
                }
            };
            let a2371 = match v1.clone() {
                Adt2::C0(_, _, h2369, _, _, _, _, _, _, _, _) => {
                    h2369
                }
            };
            nat_mul(a2370, a2371)?
        };
        let v5: u64 = {
            let a2373 = v4;
            let a2374 = match v1.clone() {
                Adt2::C0(_, _, _, h2372, _, _, _, _, _, _, _) => {
                    h2372
                }
            };
            nat_mul(a2373, a2374)?
        };
        let a2387 = {
            let a2385 = {
                let a2376 = match v0.clone() {
                    Adt2::C0(_, h2375, _, _, _, _, _, _, _, _, _) => {
                        h2375
                    }
                };
                let a2377 = 1u64;
                nat_sub(a2376, a2377)
            };
            let a2386 = {
                let a2383 = f137(match v0.clone() {
                    Adt2::C0(_, _, h2378, _, _, _, _, _, _, _, _) => {
                        h2378
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2379, _, _, _, _, _, _, _, _, _) => {
                        h2379
                    }
                })?;
                let a2384 = f137(match v0 {
                    Adt2::C0(_, _, _, h2380, _, _, _, _, _, _, _) => {
                        h2380
                    }
                }, v2, {
                    let a2381 = v3;
                    let a2382 = v5;
                    nat_mul(a2381, a2382)?
                }, v4)?;
                nat_add(a2383, a2384)?
            };
            nat_add(a2385, a2386)?
        };
        let a2388 = f12(v1, 1u64)?;
        Ok(nat_lt(a2387, a2388))
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2389 = v0;
    let a2390 = 2147483648u64;
    if if if if nat_lt(a2389, a2390) {
        let a2391 = v1;
        let a2392 = 2147483648u64;
        nat_lt(a2391, a2392)
    } else {
        false
    } {
        let a2395 = {
            let a2393 = v0;
            let a2394 = v1;
            nat_mul(a2393, a2394)?
        };
        let a2396 = 2147483648u64;
        nat_lt(a2395, a2396)
    } else {
        false
    } {
        let a2401 = {
            let a2399 = {
                let a2397 = v0;
                let a2398 = 1u64;
                nat_sub(a2397, a2398)
            };
            let a2400 = v2;
            nat_mul(a2399, a2400)?
        };
        let a2402 = 2147483648u64;
        nat_lt(a2401, a2402)
    } else {
        false
    } {
        let a2407 = {
            let a2405 = {
                let a2403 = v1;
                let a2404 = 1u64;
                nat_sub(a2403, a2404)
            };
            let a2406 = v2;
            nat_mul(a2405, a2406)?
        };
        let a2408 = 2147483648u64;
        Ok(nat_lt(a2407, a2408))
    } else {
        Ok(false)
    }
}

pub fn f96(v0: u64) -> R<u64> {
    let a2411 = 3u64;
    let a2412 = {
        let a2409 = v0;
        let a2410 = 3u64;
        nat_mul(a2409, a2410)?
    };
    nat_add(a2411, a2412)
}

pub fn f97(v0: Adt3) -> R<u64> {
    if f27(match v0.clone() {
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
    }) {
        Ok(0u64)
    } else {
        let a2425 = match v0.clone() {
            Adt3::C0(h2417, _, _, _, _, _, _, _) => {
                h2417
            }
        };
        let a2426 = {
            let a2423 = match v0.clone() {
                Adt3::C0(_, h2418, _, _, _, _, _, _) => {
                    h2418
                }
            };
            let a2424 = {
                let a2421 = match v0.clone() {
                    Adt3::C0(_, _, h2419, _, _, _, _, _) => {
                        h2419
                    }
                };
                let a2422 = match v0 {
                    Adt3::C0(_, _, _, h2420, _, _, _, _) => {
                        h2420
                    }
                };
                nat_mul(a2421, a2422)?
            };
            nat_mul(a2423, a2424)?
        };
        nat_mul(a2425, a2426)
    }
}

pub fn f98(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2427 = v1;
    let a2428 = 13u64;
    if if nat_eq(a2427, a2428) {
        f27(match v0.clone() {
            Adt3::C0(h2429, _, _, _, _, _, _, _) => {
                h2429
            }
        }, match v0.clone() {
            Adt3::C0(_, h2430, _, _, _, _, _, _) => {
                h2430
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2431, _, _, _, _, _) => {
                h2431
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2432, _, _, _, _) => {
                h2432
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2438 = match v0.clone() {
                Adt3::C0(_, h2433, _, _, _, _, _, _) => {
                    h2433
                }
            };
            let a2439 = {
                let a2436 = match v0.clone() {
                    Adt3::C0(_, _, h2434, _, _, _, _, _) => {
                        h2434
                    }
                };
                let a2437 = match v0.clone() {
                    Adt3::C0(_, _, _, h2435, _, _, _, _) => {
                        h2435
                    }
                };
                nat_mul(a2436, a2437)?
            };
            nat_mul(a2438, a2439)?
        };
        let a2440 = v1;
        let a2441 = 13u64;
        if nat_eq(a2440, a2441) {
            let a2450 = 2u64;
            let a2451 = if f138(v0.clone(), v2) {
                let a2443 = v3;
                let a2444 = f139(match v0 {
                    Adt3::C0(h2442, _, _, _, _, _, _, _) => {
                        h2442
                    }
                }, false)?;
                nat_mul(a2443, a2444)?
            } else {
                let a2448 = v3;
                let a2449 = {
                    let a2446 = match v0 {
                        Adt3::C0(h2445, _, _, _, _, _, _, _) => {
                            h2445
                        }
                    };
                    let a2447 = 2u64;
                    nat_mul(a2446, a2447)?
                };
                nat_mul(a2448, a2449)?
            };
            nat_add(a2450, a2451)
        } else {
            let a2455 = v3;
            let a2456 = f139(match v0 {
                Adt3::C0(h2452, _, _, _, _, _, _, _) => {
                    h2452
                }
            }, {
                let a2453 = v1;
                let a2454 = 16u64;
                nat_eq(a2453, a2454)
            })?;
            nat_mul(a2455, a2456)
        }
    }
}

pub fn f99(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a2457 = v0;
    let a2458 = 30u64;
    if if nat_eq(a2457, a2458) {
        let a2461 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2459, _, _, _, _) => {
                h2459
            }
        };
        let a2462 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2460, _, _, _, _, _) => {
                h2460
            }
        };
        nat_lt(a2461, a2462)
    } else {
        false
    } {
        Ok(Adt9::C1)
    } else {
        let m2463 = v3;
        match m2463 {
            None => {
                f140(v0, v1, match v2.clone() {
                    Adt2::C0(h2464, _, _, _, _, _, _, _, _, _, _) => {
                        h2464
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2465, _, _, _, _, _, _, _, _, _, _) => {
                        h2465
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2466, _, _, _, _, _, _, _, _, _) => {
                        h2466
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2467, _, _, _, _, _, _, _, _) => {
                        h2467
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2468, _, _, _, _, _, _, _) => {
                        h2468
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2469, _, _, _, _, _, _) => {
                        h2469
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2470, _, _, _, _, _, _, _, _, _) => {
                        h2470
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2471, _, _, _, _, _, _, _, _) => {
                        h2471
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2472, _, _, _, _, _, _, _) => {
                        h2472
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2473, _, _, _, _, _, _) => {
                        h2473
                    }
                })
            }
            Some(v5) => {
                f140(v0, v1, match v2.clone() {
                    Adt2::C0(h2474, _, _, _, _, _, _, _, _, _, _) => {
                        h2474
                    }
                }, match v5.clone() {
                    Adt2::C0(h2475, _, _, _, _, _, _, _, _, _, _) => {
                        h2475
                    }
                }, match v4.clone() {
                    Adt2::C0(h2476, _, _, _, _, _, _, _, _, _, _) => {
                        h2476
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h2477, _, _, _, _, _, _, _, _, _) => {
                        h2477
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2478, _, _, _, _, _, _, _, _) => {
                        h2478
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2479, _, _, _, _, _, _, _) => {
                        h2479
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2480, _, _, _, _, _, _) => {
                        h2480
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h2481, _, _, _, _, _, _, _, _, _) => {
                        h2481
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h2482, _, _, _, _, _, _, _, _) => {
                        h2482
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h2483, _, _, _, _, _, _, _) => {
                        h2483
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h2484, _, _, _, _, _, _) => {
                        h2484
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2485, _, _, _, _, _, _, _, _, _) => {
                        h2485
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2486, _, _, _, _, _, _, _, _) => {
                        h2486
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2487, _, _, _, _, _, _, _) => {
                        h2487
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2488, _, _, _, _, _, _) => {
                        h2488
                    }
                })
            }
        }
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2501 = {
        let a2497 = {
            let a2493 = {
                let a2489 = v0;
                let a2490 = v4;
                nat_mul(a2489, a2490)?
            };
            let a2494 = {
                let a2491 = v1;
                let a2492 = v5;
                nat_mul(a2491, a2492)?
            };
            nat_add(a2493, a2494)?
        };
        let a2498 = {
            let a2495 = v2;
            let a2496 = v6;
            nat_mul(a2495, a2496)?
        };
        nat_add(a2497, a2498)?
    };
    let a2502 = {
        let a2499 = v3;
        let a2500 = v7;
        nat_mul(a2499, a2500)?
    };
    nat_add(a2501, a2502)
}

pub fn f101(v0: Adt10, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt8 {
    let v9: u64 = {
        let a2503 = v7;
        let a2504 = 6u64;
        if nat_eq(a2503, a2504) {
            27u64
        } else {
            v2
        }
    };
    Adt8::C7(v0, f70(v3, v4), f70(v5, v6), f71(v7, v8), f141(v1), f141(v9))
}

pub fn f102(v0: Adt4, v1: u64, v2: u64) -> Adt10 {
    let m2505 = v0;
    match m2505 {
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
            let a2506 = 13u64;
            let a2507 = v34;
            if nat_le(a2506, a2507) {
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
                Adt5::C0(_, _, _, _, _, h2508, _, _) => {
                    h2508
                }
            }, match v50 {
                Adt5::C0(_, _, _, _, _, _, _, h2509) => {
                    h2509
                }
            }, true)
        }
    }
}

pub fn f103(v0: Adt4) -> u64 {
    let m2510 = v0;
    match m2510 {
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
    let m2511 = v0;
    match m2511 {
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
            let a2512 = v1;
            let a2513 = 6u64;
            if nat_eq(a2512, a2513) {
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
            let a2515 = v1;
            let a2516 = 6u64;
            if if if nat_eq(a2515, a2516) {
                true
            } else {
                let a2517 = v1;
                let a2518 = 7u64;
                nat_eq(a2517, a2518)
            } {
                let a2519 = v50;
                let a2520 = 30u64;
                nat_eq(a2519, a2520)
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
                Adt5::C0(_, _, _, _, _, _, _, h2514) => {
                    h2514
                }
            }
        }
    }
}

pub fn f105(v0: Adt4, v1: u64) -> Adt10 {
    let a2521 = v1;
    let a2522 = 4u64;
    if nat_lt(a2521, a2522) {
        Adt10::C0(v1)
    } else {
        let a2523 = v1;
        let a2524 = 4u64;
        if nat_eq(a2523, a2524) {
            Adt10::C1(0u64)
        } else {
            let a2525 = v1;
            let a2526 = 5u64;
            if nat_eq(a2525, a2526) {
                Adt10::C1(3u64)
            } else {
                let a2527 = v1;
                let a2528 = 6u64;
                if nat_eq(a2527, a2528) {
                    Adt10::C2(f104(v0, v1), 0u64, false)
                } else {
                    let a2529 = v1;
                    let a2530 = 7u64;
                    if nat_eq(a2529, a2530) {
                        Adt10::C2(0u64, f104(v0, v1), false)
                    } else {
                        let a2531 = v1;
                        let a2532 = 16u64;
                        if nat_lt(a2531, a2532) {
                            Adt10::C1({
                                let a2533 = v1;
                                let a2534 = 8u64;
                                nat_sub(a2533, a2534)
                            })
                        } else {
                            let a2535 = v1;
                            let a2536 = 20u64;
                            if nat_lt(a2535, a2536) {
                                Adt10::C5({
                                    let a2537 = v1;
                                    let a2538 = 16u64;
                                    nat_sub(a2537, a2538)
                                })
                            } else {
                                let a2539 = v1;
                                let a2540 = 20u64;
                                if nat_eq(a2539, a2540) {
                                    Adt10::C3(0u64, 0u64, false)
                                } else {
                                    let a2541 = v1;
                                    let a2542 = 21u64;
                                    if nat_eq(a2541, a2542) {
                                        Adt10::C0(4u64)
                                    } else {
                                        let a2543 = v1;
                                        let a2544 = 22u64;
                                        if nat_eq(a2543, a2544) {
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
        let a2546 = v0;
        let a2547 = match v1.clone() {
            Adt3::C0(h2545, _, _, _, _, _, _, _) => {
                h2545
            }
        };
        let a2548 = 0u64;
        nat_quot(a2546, a2547, a2548)
    };
    let v3: u64 = {
        let a2550 = v2;
        let a2551 = match v1.clone() {
            Adt3::C0(_, h2549, _, _, _, _, _, _) => {
                h2549
            }
        };
        let a2552 = 0u64;
        nat_quot(a2550, a2551, a2552)
    };
    f100({
        let a2554 = v0;
        let a2555 = match v1.clone() {
            Adt3::C0(h2553, _, _, _, _, _, _, _) => {
                h2553
            }
        };
        let a2556 = 0u64;
        nat_rem(a2554, a2555, a2556)
    }, {
        let a2558 = v2;
        let a2559 = match v1.clone() {
            Adt3::C0(_, h2557, _, _, _, _, _, _) => {
                h2557
            }
        };
        let a2560 = 0u64;
        nat_rem(a2558, a2559, a2560)
    }, {
        let a2562 = v3;
        let a2563 = match v1.clone() {
            Adt3::C0(_, _, h2561, _, _, _, _, _) => {
                h2561
            }
        };
        let a2564 = 0u64;
        nat_rem(a2562, a2563, a2564)
    }, {
        let a2566 = v3;
        let a2567 = match v1.clone() {
            Adt3::C0(_, _, h2565, _, _, _, _, _) => {
                h2565
            }
        };
        let a2568 = 0u64;
        nat_quot(a2566, a2567, a2568)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2569, _, _, _) => {
            h2569
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2570, _, _) => {
            h2570
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2571, _) => {
            h2571
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2572) => {
            h2572
        }
    })
}

pub fn f107(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2590 = {
        let a2585 = f142(v2, {
            let a2576 = {
                let a2573 = v0;
                let a2574 = v1;
                let a2575 = 0u64;
                nat_quot(a2573, a2574, a2575)
            };
            let a2577 = v2;
            let a2578 = 0u64;
            nat_rem(a2576, a2577, a2578)
        }, {
            let a2582 = {
                let a2579 = v0;
                let a2580 = v1;
                let a2581 = 0u64;
                nat_quot(a2579, a2580, a2581)
            };
            let a2583 = v2;
            let a2584 = 0u64;
            nat_quot(a2582, a2583, a2584)
        })?;
        let a2586 = v1;
        nat_mul(a2585, a2586)?
    };
    let a2591 = {
        let a2587 = v0;
        let a2588 = v1;
        let a2589 = 0u64;
        nat_rem(a2587, a2588, a2589)
    };
    nat_add(a2590, a2591)
}

pub fn f108(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2593 = v0;
        let a2594 = match v2.clone() {
            Adt3::C0(h2592, _, _, _, _, _, _, _) => {
                h2592
            }
        };
        let a2595 = 0u64;
        nat_quot(a2593, a2594, a2595)
    };
    let v4: u64 = {
        let a2597 = v3;
        let a2598 = match v2.clone() {
            Adt3::C0(_, h2596, _, _, _, _, _, _) => {
                h2596
            }
        };
        let a2599 = 0u64;
        nat_quot(a2597, a2598, a2599)
    };
    f143({
        let a2601 = v0;
        let a2602 = match v2.clone() {
            Adt3::C0(h2600, _, _, _, _, _, _, _) => {
                h2600
            }
        };
        let a2603 = 0u64;
        nat_rem(a2601, a2602, a2603)
    }, {
        let a2605 = v3;
        let a2606 = match v2.clone() {
            Adt3::C0(_, h2604, _, _, _, _, _, _) => {
                h2604
            }
        };
        let a2607 = 0u64;
        nat_rem(a2605, a2606, a2607)
    }, {
        let a2609 = v4;
        let a2610 = match v2.clone() {
            Adt3::C0(_, _, h2608, _, _, _, _, _) => {
                h2608
            }
        };
        let a2611 = 0u64;
        nat_rem(a2609, a2610, a2611)
    }, {
        let a2613 = v4;
        let a2614 = match v2 {
            Adt3::C0(_, _, h2612, _, _, _, _, _) => {
                h2612
            }
        };
        let a2615 = 0u64;
        nat_quot(a2613, a2614, a2615)
    }, match v1.clone() {
        Adt3::C0(h2616, _, _, _, _, _, _, _) => {
            h2616
        }
    }, match v1.clone() {
        Adt3::C0(_, h2617, _, _, _, _, _, _) => {
            h2617
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2618, _, _, _, _, _) => {
            h2618
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2619, _, _, _, _) => {
            h2619
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2620, _, _, _) => {
            h2620
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2621, _, _) => {
            h2621
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2622, _) => {
            h2622
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2623) => {
            h2623
        }
    })
}

pub fn f109(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt8> {
    let v5: u64 = {
        let a2625 = v0;
        let a2626 = match v3.clone() {
            Adt3::C0(h2624, _, _, _, _, _, _, _) => {
                h2624
            }
        };
        let a2627 = 0u64;
        nat_quot(a2625, a2626, a2627)
    };
    let v6: u64 = {
        let a2629 = v5;
        let a2630 = match v3.clone() {
            Adt3::C0(_, h2628, _, _, _, _, _, _) => {
                h2628
            }
        };
        let a2631 = 0u64;
        nat_quot(a2629, a2630, a2631)
    };
    let v7: u64 = {
        let a2633 = v0;
        let a2634 = match v3.clone() {
            Adt3::C0(h2632, _, _, _, _, _, _, _) => {
                h2632
            }
        };
        let a2635 = 0u64;
        nat_rem(a2633, a2634, a2635)
    };
    let v8: u64 = {
        let a2637 = v5;
        let a2638 = match v3.clone() {
            Adt3::C0(_, h2636, _, _, _, _, _, _) => {
                h2636
            }
        };
        let a2639 = 0u64;
        nat_rem(a2637, a2638, a2639)
    };
    let v9: u64 = {
        let a2641 = v6;
        let a2642 = match v3.clone() {
            Adt3::C0(_, _, h2640, _, _, _, _, _) => {
                h2640
            }
        };
        let a2643 = 0u64;
        nat_rem(a2641, a2642, a2643)
    };
    let v10: u64 = {
        let a2645 = v6;
        let a2646 = match v3.clone() {
            Adt3::C0(_, _, h2644, _, _, _, _, _) => {
                h2644
            }
        };
        let a2647 = 0u64;
        nat_quot(a2645, a2646, a2647)
    };
    let v11: u64 = {
        let a2648 = v4;
        let a2649 = 0u64;
        if nat_eq(a2648, a2649) {
            match v1.clone() {
                Adt3::C0(h2650, _, _, _, _, _, _, _) => {
                    h2650
                }
            }
        } else {
            let a2651 = v4;
            let a2652 = 1u64;
            if nat_eq(a2651, a2652) {
                match v1.clone() {
                    Adt3::C0(_, h2653, _, _, _, _, _, _) => {
                        h2653
                    }
                }
            } else {
                let a2654 = v4;
                let a2655 = 2u64;
                if nat_eq(a2654, a2655) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2656, _, _, _, _, _) => {
                            h2656
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2657, _, _, _, _) => {
                            h2657
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2664 = v11;
        let a2665 = {
            let a2658 = v4;
            let a2659 = 0u64;
            if nat_eq(a2658, a2659) {
                v7
            } else {
                let a2660 = v4;
                let a2661 = 1u64;
                if nat_eq(a2660, a2661) {
                    v8
                } else {
                    let a2662 = v4;
                    let a2663 = 2u64;
                    if nat_eq(a2662, a2663) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2664, a2665)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt8::C1(v12, f100(if if v12 {
        let a2666 = v4;
        let a2667 = 0u64;
        nat_eq(a2666, a2667)
    } else {
        false
    } {
        let a2668 = v7;
        let a2669 = v11;
        nat_sub(a2668, a2669)
    } else {
        v7
    }, if if v12 {
        let a2670 = v4;
        let a2671 = 1u64;
        nat_eq(a2670, a2671)
    } else {
        false
    } {
        let a2672 = v8;
        let a2673 = v11;
        nat_sub(a2672, a2673)
    } else {
        v8
    }, if if v12 {
        let a2674 = v4;
        let a2675 = 2u64;
        nat_eq(a2674, a2675)
    } else {
        false
    } {
        let a2676 = v9;
        let a2677 = v11;
        nat_sub(a2676, a2677)
    } else {
        v9
    }, if if v12 {
        let a2678 = v4;
        let a2679 = 3u64;
        nat_eq(a2678, a2679)
    } else {
        false
    } {
        let a2680 = v10;
        let a2681 = v11;
        nat_sub(a2680, a2681)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2682, _, _, _) => {
            h2682
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2683, _, _) => {
            h2683
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2684, _) => {
            h2684
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2685) => {
            h2685
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
            Adt3::C0(h2686, _, _, _, _, _, _, _) => {
                h2686
            }
        })?, f144(v9, v4, match v1.clone() {
            Adt3::C0(_, h2687, _, _, _, _, _, _) => {
                h2687
            }
        })?, f144(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2688, _, _, _, _, _) => {
                h2688
            }
        })?, f144(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2689, _, _, _, _) => {
                h2689
            }
        })?)?, f106(v0, v2)?))
    } else {
        if if if if f145(v8, v3, match v1.clone() {
            Adt3::C0(h2690, _, _, _, _, _, _, _) => {
                h2690
            }
        })? {
            f145(v9, v4, match v1.clone() {
                Adt3::C0(_, h2691, _, _, _, _, _, _) => {
                    h2691
                }
            })?
        } else {
            false
        } {
            f145(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2692, _, _, _, _, _) => {
                    h2692
                }
            })?
        } else {
            false
        } {
            f145(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2693, _, _, _, _) => {
                    h2693
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
        Adt3::C0(h2694, _, _, _, _, _, _, _) => {
            h2694
        }
    })?, v5, v6, v7)?, f106(v0, v2)?))
}

pub fn f112(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt8> {
    let a2695 = v0;
    let a2696 = v3;
    if nat_lt(a2695, a2696) {
        Ok(Adt8::C1(false, {
            let a2697 = v0;
            let a2698 = 4u64;
            nat_mul(a2697, a2698)?
        }, {
            let a2699 = v0;
            let a2700 = 4u64;
            nat_mul(a2699, a2700)?
        }))
    } else {
        let v6: u64 = {
            let a2701 = v0;
            let a2702 = v3;
            nat_sub(a2701, a2702)
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
    let a2703 = v0;
    let a2704 = v3;
    if nat_lt(a2703, a2704) {
        Ok(Adt8::C1(true, 0u64, f106(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2705 = v0;
            let a2706 = v3;
            nat_sub(a2705, a2706)
        };
        let v5: u64 = f108(v4, v2, v1.clone())?;
        Ok(Adt8::C4(8u64, 3u64, v5, 0u64, f106(v4, v1)?, 3u64, v5))
    }
}

pub fn f114(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt8> {
    let v4: u64 = {
        let a2708 = v1;
        let a2709 = match v3.clone() {
            Adt3::C0(h2707, _, _, _, _, _, _, _) => {
                h2707
            }
        };
        let a2710 = 0u64;
        nat_rem(a2708, a2709, a2710)
    };
    let v5: u64 = {
        let a2716 = {
            let a2712 = v1;
            let a2713 = match v3.clone() {
                Adt3::C0(h2711, _, _, _, _, _, _, _) => {
                    h2711
                }
            };
            let a2714 = 0u64;
            nat_quot(a2712, a2713, a2714)
        };
        let a2717 = match v3.clone() {
            Adt3::C0(_, h2715, _, _, _, _, _, _) => {
                h2715
            }
        };
        let a2718 = 0u64;
        nat_rem(a2716, a2717, a2718)
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
        let a2719 = v3;
        let a2720 = 8u64;
        nat_add(a2719, a2720)?
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
        let a2722 = v0;
        let a2723 = match v2.clone() {
            Adt3::C0(h2721, _, _, _, _, _, _, _) => {
                h2721
            }
        };
        let a2724 = 0u64;
        nat_rem(a2722, a2723, a2724)
    };
    let v7: u64 = {
        let a2726 = v0;
        let a2727 = match v2.clone() {
            Adt3::C0(h2725, _, _, _, _, _, _, _) => {
                h2725
            }
        };
        let a2728 = 0u64;
        nat_quot(a2726, a2727, a2728)
    };
    let v8: u64 = {
        let a2730 = v7;
        let a2731 = match v2.clone() {
            Adt3::C0(_, h2729, _, _, _, _, _, _) => {
                h2729
            }
        };
        let a2732 = 0u64;
        nat_rem(a2730, a2731, a2732)
    };
    let v9: u64 = {
        let a2734 = v7;
        let a2735 = match v2.clone() {
            Adt3::C0(_, h2733, _, _, _, _, _, _) => {
                h2733
            }
        };
        let a2736 = 0u64;
        nat_quot(a2734, a2735, a2736)
    };
    let v10: u64 = {
        let a2738 = v9;
        let a2739 = match v2.clone() {
            Adt3::C0(_, _, h2737, _, _, _, _, _) => {
                h2737
            }
        };
        let a2740 = 0u64;
        nat_rem(a2738, a2739, a2740)
    };
    let v11: u64 = {
        let a2742 = v9;
        let a2743 = match v2.clone() {
            Adt3::C0(_, _, h2741, _, _, _, _, _) => {
                h2741
            }
        };
        let a2744 = 0u64;
        nat_quot(a2742, a2743, a2744)
    };
    let v12: u64 = f106(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2753 = {
                let a2748 = {
                    let a2745 = v10;
                    let a2746 = v4;
                    let a2747 = 0u64;
                    nat_quot(a2745, a2746, a2747)
                };
                let a2749 = v5;
                nat_mul(a2748, a2749)?
            };
            let a2754 = {
                let a2750 = v8;
                let a2751 = v4;
                let a2752 = 0u64;
                nat_quot(a2750, a2751, a2752)
            };
            nat_add(a2753, a2754)?
        };
        let v14: u64 = {
            let a2774 = v6;
            let a2775 = {
                let a2772 = match v1.clone() {
                    Adt3::C0(h2755, _, _, _, _, _, _, _) => {
                        h2755
                    }
                };
                let a2773 = {
                    let a2770 = {
                        let a2756 = v8;
                        let a2757 = v4;
                        let a2758 = 0u64;
                        nat_rem(a2756, a2757, a2758)
                    };
                    let a2771 = {
                        let a2768 = match v1.clone() {
                            Adt3::C0(_, h2759, _, _, _, _, _, _) => {
                                h2759
                            }
                        };
                        let a2769 = {
                            let a2766 = {
                                let a2760 = v10;
                                let a2761 = v4;
                                let a2762 = 0u64;
                                nat_rem(a2760, a2761, a2762)
                            };
                            let a2767 = {
                                let a2764 = match v1.clone() {
                                    Adt3::C0(_, _, h2763, _, _, _, _, _) => {
                                        h2763
                                    }
                                };
                                let a2765 = v13;
                                nat_mul(a2764, a2765)?
                            };
                            nat_add(a2766, a2767)?
                        };
                        nat_mul(a2768, a2769)?
                    };
                    nat_add(a2770, a2771)?
                };
                nat_mul(a2772, a2773)?
            };
            nat_add(a2774, a2775)?
        };
        Ok(Adt8::C1(false, f106(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2781 = {
                let a2779 = {
                    let a2776 = v11;
                    let a2777 = v5;
                    let a2778 = 0u64;
                    nat_rem(a2776, a2777, a2778)
                };
                let a2780 = v4;
                nat_mul(a2779, a2780)?
            };
            let a2782 = v8;
            nat_add(a2781, a2782)?
        };
        let v16: u64 = {
            let a2788 = {
                let a2786 = {
                    let a2783 = v11;
                    let a2784 = v5;
                    let a2785 = 0u64;
                    nat_quot(a2783, a2784, a2785)
                };
                let a2787 = v4;
                nat_mul(a2786, a2787)?
            };
            let a2789 = v10;
            nat_add(a2788, a2789)?
        };
        let a2791 = v15;
        let a2792 = match v1.clone() {
            Adt3::C0(_, h2790, _, _, _, _, _, _) => {
                h2790
            }
        };
        if if nat_lt(a2791, a2792) {
            let a2794 = v16;
            let a2795 = match v1.clone() {
                Adt3::C0(_, _, h2793, _, _, _, _, _) => {
                    h2793
                }
            };
            nat_lt(a2794, a2795)
        } else {
            false
        } {
            Ok(Adt8::C1(false, f106({
                let a2804 = v6;
                let a2805 = {
                    let a2802 = match v1.clone() {
                        Adt3::C0(h2796, _, _, _, _, _, _, _) => {
                            h2796
                        }
                    };
                    let a2803 = {
                        let a2800 = v15;
                        let a2801 = {
                            let a2798 = match v1.clone() {
                                Adt3::C0(_, h2797, _, _, _, _, _, _) => {
                                    h2797
                                }
                            };
                            let a2799 = v16;
                            nat_mul(a2798, a2799)?
                        };
                        nat_add(a2800, a2801)?
                    };
                    nat_mul(a2802, a2803)?
                };
                nat_add(a2804, a2805)?
            }, v1)?, v12))
        } else {
            Ok(Adt8::C1(true, 0u64, v12))
        }
    }
}

pub fn f117(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt8> {
    let v3: u64 = f106(v0, v2.clone())?;
    let v4: u64 = f106(v0, v1.clone())?;
    let a2810 = {
        let a2807 = v0;
        let a2808 = match v1 {
            Adt3::C0(h2806, _, _, _, _, _, _, _) => {
                h2806
            }
        };
        let a2809 = 0u64;
        nat_rem(a2807, a2808, a2809)
    };
    let a2811 = 0u64;
    if nat_eq(a2810, a2811) {
        Ok(Adt8::C1(false, v4, v3))
    } else {
        Ok(Adt8::C4(8u64, 3u64, f106({
            let a2812 = v0;
            let a2813 = 1u64;
            nat_sub(a2812, a2813)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f118(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt8> {
    let a2814 = v3;
    let a2815 = 13u64;
    if nat_eq(a2814, a2815) {
        f150(v0, v1, v4)
    } else {
        let v5: u64 = f139(match v1.clone() {
            Adt3::C0(h2816, _, _, _, _, _, _, _) => {
                h2816
            }
        }, {
            let a2817 = v3;
            let a2818 = 16u64;
            nat_eq(a2817, a2818)
        })?;
        let v6: u64 = {
            let a2819 = v0;
            let a2820 = v5;
            let a2821 = 0u64;
            nat_quot(a2819, a2820, a2821)
        };
        let v7: u64 = {
            let a2822 = v0;
            let a2823 = v5;
            let a2824 = 0u64;
            nat_rem(a2822, a2823, a2824)
        };
        let v8: u64 = {
            let a2828 = 1u64;
            let a2829 = {
                let a2826 = match v1.clone() {
                    Adt3::C0(h2825, _, _, _, _, _, _, _) => {
                        h2825
                    }
                };
                let a2827 = 2u64;
                nat_mul(a2826, a2827)?
            };
            nat_add(a2828, a2829)?
        };
        let a2830 = v7;
        let a2831 = v8;
        if nat_lt(a2830, a2831) {
            f151(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f106(v6, v2)?;
            let a2832 = v3;
            let a2833 = 14u64;
            if nat_eq(a2832, a2833) {
                Ok(Adt8::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2834 = v7;
                let a2835 = v8;
                if nat_eq(a2834, a2835) {
                    Ok(Adt8::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2838 = v7;
                    let a2839 = {
                        let a2836 = v8;
                        let a2837 = 1u64;
                        nat_add(a2836, a2837)?
                    };
                    if nat_eq(a2838, a2839) {
                        Ok(Adt8::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2840, _, _, _, _, _, _, _) => {
                                h2840
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
    let a2841 = v0;
    let a2842 = 0u64;
    if nat_eq(a2841, a2842) {
        Ok(Adt8::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2849 = {
            let a2843 = v0;
            let a2844 = 1u64;
            nat_add(a2843, a2844)?
        };
        let a2850 = {
            let a2847 = 2u64;
            let a2848 = {
                let a2845 = v3;
                let a2846 = 2u64;
                nat_mul(a2845, a2846)?
            };
            nat_add(a2847, a2848)?
        };
        if nat_eq(a2849, a2850) {
            Ok(Adt8::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2851 = v0;
                let a2852 = 1u64;
                nat_sub(a2851, a2852)
            };
            let v5: u64 = {
                let a2853 = v4;
                let a2854 = 2u64;
                let a2855 = 0u64;
                nat_quot(a2853, a2854, a2855)
            };
            let a2859 = {
                let a2856 = v4;
                let a2857 = 2u64;
                let a2858 = 0u64;
                nat_rem(a2856, a2857, a2858)
            };
            let a2860 = 0u64;
            if nat_eq(a2859, a2860) {
                Ok(Adt8::C4(17u64, 0u64, f106(v5, v1)?, 4u64, f106(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt8::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f120(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt8> {
    let a2861 = v0;
    let a2862 = f22(v1, v2, v3)?;
    if nat_lt(a2861, a2862) {
        let a2863 = v0;
        let a2864 = v1;
        if nat_lt(a2863, a2864) {
            Ok(Adt8::C1(false, {
                let a2865 = 4u64;
                let a2866 = v0;
                nat_mul(a2865, a2866)?
            }, {
                let a2867 = 4u64;
                let a2868 = v0;
                nat_mul(a2867, a2868)?
            }))
        } else {
            let v4: u64 = {
                let a2869 = v0;
                let a2870 = v1;
                nat_sub(a2869, a2870)
            };
            let v5: u64 = f152(v4, v2)?;
            let a2871 = v5;
            let a2872 = 2u64;
            if nat_lt(a2871, a2872) {
                Ok(Adt8::C4(6u64, {
                    let a2873 = v5;
                    let a2874 = 0u64;
                    if nat_eq(a2873, a2874) {
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
    let a2882 = {
        let a2875 = 2u64;
        let a2876 = f97(v1.clone())?;
        nat_mul(a2875, a2876)?
    };
    let a2883 = {
        let a2880 = f97(v1)?;
        let a2881 = {
            let a2878 = 1u64;
            let a2879 = match v0 {
                Adt3::C0(h2877, _, _, _, _, _, _, _) => {
                    h2877
                }
            };
            nat_add(a2878, a2879)?
        };
        nat_mul(a2880, a2881)?
    };
    nat_add(a2882, a2883)
}

pub fn f122(v0: u64, v1: Adt3) -> R<Adt8> {
    Ok(Adt8::C4(19u64, 4u64, f106(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f123(v0: u64, v1: u64) -> bool {
    let a2884 = v1;
    let a2885 = 2147483647u64;
    if nat_le(a2884, a2885) {
        let a2886 = v1;
        let a2887 = v0;
        nat_lt(a2886, a2887)
    } else {
        false
    }
}

pub fn f124(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt8> {
    let m2888 = f86(v0);
    match m2888 {
        Adt13::C0 => {
            Ok(Adt8::C0(Adt9::C1))
        }
        Adt13::C1(v7) => {
            let v8: u64 = {
                let a2890 = v2;
                let a2891 = match v7.clone() {
                    Adt14::C0(_, h2889, _) => {
                        h2889
                    }
                };
                nat_mul(a2890, a2891)?
            };
            let m2904 = f156(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2892, _, _, _, _, _, _, _) => {
                    h2892
                }
            }, match v5.clone() {
                Adt3::C0(_, h2893, _, _, _, _, _, _) => {
                    h2893
                }
            }, match v5 {
                Adt3::C0(_, _, h2894, _, _, _, _, _) => {
                    h2894
                }
            }, {
                let a2897 = match v3.clone() {
                    Adt3::C0(h2895, _, _, _, _, _, _, _) => {
                        h2895
                    }
                };
                let a2898 = match v7.clone() {
                    Adt14::C0(_, h2896, _) => {
                        h2896
                    }
                };
                nat_mul(a2897, a2898)?
            }, match v3.clone() {
                Adt3::C0(_, h2899, _, _, _, _, _, _) => {
                    h2899
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2900, _, _, _) => {
                    h2900
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2901, _, _) => {
                    h2901
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2902, _) => {
                    h2902
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2903) => {
                    h2903
                }
            })?;
            match m2904 {
                None => {
                    Ok(Adt8::C0(Adt9::C1))
                }
                Some(v9) => {
                    let a2905 = v0;
                    let a2906 = 0u64;
                    if if nat_eq(a2905, a2906) {
                        true
                    } else {
                        let a2907 = v0;
                        let a2908 = 26u64;
                        nat_eq(a2907, a2908)
                    } {
                        Ok(Adt8::C1(false, v9, f106({
                            let a2912 = {
                                let a2910 = v1;
                                let a2911 = match v4.clone() {
                                    Adt3::C0(h2909, _, _, _, _, _, _, _) => {
                                        h2909
                                    }
                                };
                                nat_mul(a2910, a2911)?
                            };
                            let a2913 = v8;
                            nat_add(a2912, a2913)?
                        }, v4)?))
                    } else {
                        Ok(Adt8::C5(v0, v9, match v7.clone() {
                            Adt14::C0(_, _, h2914) => {
                                h2914
                            }
                        }, f106({
                            let a2918 = {
                                let a2916 = v1;
                                let a2917 = match v4.clone() {
                                    Adt3::C0(h2915, _, _, _, _, _, _, _) => {
                                        h2915
                                    }
                                };
                                nat_mul(a2916, a2917)?
                            };
                            let a2919 = v8;
                            nat_add(a2918, a2919)?
                        }, v4.clone())?, {
                            let a2926 = {
                                let a2924 = {
                                    let a2921 = match v7 {
                                        Adt14::C0(_, h2920, _) => {
                                            h2920
                                        }
                                    };
                                    let a2922 = 1u64;
                                    nat_sub(a2921, a2922)
                                };
                                let a2925 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2923, _, _, _) => {
                                        h2923
                                    }
                                };
                                nat_mul(a2924, a2925)?
                            };
                            let a2927 = 4u64;
                            nat_add(a2926, a2927)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2928, _, _, _) => {
                                h2928
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f125(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2929 = v1;
    let a2930 = 0u64;
    if if nat_eq(a2929, a2930) {
        true
    } else {
        let a2931 = v2;
        let a2932 = 0u64;
        nat_eq(a2931, a2932)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f126(v0: u64, v1: u64) -> bool {
    let a2933 = v0;
    let a2934 = 4294967295u64;
    if nat_eq(a2933, a2934) {
        true
    } else {
        let a2935 = v0;
        let a2936 = 2147483648u64;
        if nat_lt(a2935, a2936) {
            let a2937 = v0;
            let a2938 = v1;
            nat_lt(a2937, a2938)
        } else {
            false
        }
    }
}

pub fn f127(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt8> {
    let v5: u64 = {
        let a2945 = v3;
        let a2946 = {
            let a2942 = f106({
                let a2939 = v0;
                let a2940 = 2u64;
                let a2941 = 0u64;
                nat_quot(a2939, a2940, a2941)
            }, v1)?;
            let a2943 = v2;
            let a2944 = 0u64;
            nat_quot(a2942, a2943, a2944)
        };
        nat_add(a2945, a2946)?
    };
    let a2950 = {
        let a2947 = v0;
        let a2948 = 2u64;
        let a2949 = 0u64;
        nat_rem(a2947, a2948, a2949)
    };
    let a2951 = 0u64;
    if nat_eq(a2950, a2951) {
        Ok(Adt8::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2952 = v4;
        let a2953 = 0u64;
        if nat_eq(a2952, a2953) {
            Ok(Adt8::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt8::C0(Adt9::C1))
        }
    }
}

pub fn f128(v0: u64, v1: u64) -> R<u64> {
    let a2956 = 8u64;
    let a2957 = {
        let a2954 = f157(v0);
        let a2955 = v1;
        nat_mul(a2954, a2955)?
    };
    nat_add(a2956, a2957)
}

pub fn f129(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt8> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2958, _, _, _, _, _, _, _) => {
            h2958
        }
    };
    let v7: u64 = f157(v3);
    let v8: u64 = {
        let a2961 = 1u64;
        let a2962 = {
            let a2959 = v6;
            let a2960 = v7;
            nat_mul(a2959, a2960)?
        };
        nat_add(a2961, a2962)?
    };
    let a2963 = v0;
    let a2964 = 0u64;
    if nat_eq(a2963, a2964) {
        Ok(f158(0u64))
    } else {
        let a2965 = v0;
        let a2966 = v8;
        if nat_lt(a2965, a2966) {
            let v9: u64 = {
                let a2967 = v0;
                let a2968 = 1u64;
                nat_sub(a2967, a2968)
            };
            let v10: u64 = {
                let a2969 = v9;
                let a2970 = v7;
                let a2971 = 0u64;
                nat_rem(a2969, a2970, a2971)
            };
            let v11: u64 = f106({
                let a2977 = {
                    let a2972 = v1;
                    let a2973 = v6;
                    nat_mul(a2972, a2973)?
                };
                let a2978 = {
                    let a2974 = v9;
                    let a2975 = v7;
                    let a2976 = 0u64;
                    nat_quot(a2974, a2975, a2976)
                };
                nat_add(a2977, a2978)?
            }, v2)?;
            let a2979 = v3;
            let a2980 = 30u64;
            if nat_eq(a2979, a2980) {
                let a2981 = v10;
                let a2982 = 0u64;
                if nat_eq(a2981, a2982) {
                    Ok(Adt8::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2983 = v10;
                    let a2984 = 1u64;
                    if nat_eq(a2983, a2984) {
                        Ok(Adt8::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2985 = v10;
                        let a2986 = 2u64;
                        if nat_eq(a2985, a2986) {
                            Ok(Adt8::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt8::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2987 = v10;
                let a2988 = 0u64;
                if nat_eq(a2987, a2988) {
                    Ok(Adt8::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2989 = v10;
                    let a2990 = 1u64;
                    if nat_eq(a2989, a2990) {
                        Ok(Adt8::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt8::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2991 = v0;
                let a2992 = v8;
                nat_sub(a2991, a2992)
            };
            let a2993 = v12;
            let a2994 = 0u64;
            if nat_eq(a2993, a2994) {
                Ok(Adt8::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2995 = v12;
                let a2996 = 1u64;
                if nat_eq(a2995, a2996) {
                    Ok(Adt8::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2997 = v12;
                    let a2998 = 2u64;
                    if nat_eq(a2997, a2998) {
                        Ok(Adt8::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2999 = v12;
                        let a3000 = 3u64;
                        if nat_eq(a2999, a3000) {
                            Ok(Adt8::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3001 = v12;
                            let a3002 = 4u64;
                            if nat_eq(a3001, a3002) {
                                Ok(Adt8::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3003 = v12;
                                let a3004 = 5u64;
                                if nat_eq(a3003, a3004) {
                                    Ok(Adt8::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3005 = v12;
                                    let a3006 = 6u64;
                                    if nat_eq(a3005, a3006) {
                                        if f159(v5) {
                                            Ok(Adt8::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3007 = 3u64;
                                                let a3008 = v1;
                                                nat_add(a3007, a3008)?
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
        let a3009 = v0;
        let a3010 = f80(v3);
        let a3011 = 0u64;
        nat_quot(a3009, a3010, a3011)
    };
    let v5: u64 = {
        let a3016 = 3u64;
        let a3017 = {
            let a3013 = v4;
            let a3014 = match v1.clone() {
                Adt3::C0(h3012, _, _, _, _, _, _, _) => {
                    h3012
                }
            };
            let a3015 = 0u64;
            nat_quot(a3013, a3014, a3015)
        };
        nat_add(a3016, a3017)?
    };
    let v6: u64 = f106(v4, v1)?;
    let v7: u64 = f106(v4, v2)?;
    let a3018 = v3;
    let a3019 = 30u64;
    if nat_eq(a3018, a3019) {
        let a3023 = {
            let a3020 = v0;
            let a3021 = 3u64;
            let a3022 = 0u64;
            nat_rem(a3020, a3021, a3022)
        };
        let a3024 = 0u64;
        if nat_eq(a3023, a3024) {
            Ok(Adt8::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3028 = {
                let a3025 = v0;
                let a3026 = 3u64;
                let a3027 = 0u64;
                nat_rem(a3025, a3026, a3027)
            };
            let a3029 = 1u64;
            if nat_eq(a3028, a3029) {
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
        let a3030 = v0;
        let a3031 = 8388608u64;
        let a3032 = 0u64;
        nat_quot(a3030, a3031, a3032)
    };
    let a3033 = v0;
    let a3034 = 2147483648u64;
    if if if nat_lt(a3033, a3034) {
        let a3035 = 127u64;
        let a3036 = v1;
        nat_le(a3035, a3036)
    } else {
        false
    } {
        let a3037 = v1;
        let a3038 = 190u64;
        nat_lt(a3037, a3038)
    } else {
        false
    } {
        let v2: u64 = {
            let a3042 = 8388608u64;
            let a3043 = {
                let a3039 = v0;
                let a3040 = 8388608u64;
                let a3041 = 0u64;
                nat_rem(a3039, a3040, a3041)
            };
            nat_add(a3042, a3043)?
        };
        let a3044 = v1;
        let a3045 = 150u64;
        if nat_lt(a3044, a3045) {
            let v3: u64 = f160({
                let a3046 = 150u64;
                let a3047 = v1;
                nat_sub(a3046, a3047)
            });
            let a3051 = {
                let a3048 = v2;
                let a3049 = v3;
                let a3050 = 0u64;
                nat_rem(a3048, a3049, a3050)
            };
            let a3052 = 0u64;
            if nat_eq(a3051, a3052) {
                let a3053 = v2;
                let a3054 = v3;
                let a3055 = 0u64;
                Ok(nat_quot(a3053, a3054, a3055))
            } else {
                Ok(0u64)
            }
        } else {
            let a3058 = f160({
                let a3056 = v1;
                let a3057 = 150u64;
                nat_sub(a3056, a3057)
            });
            let a3059 = v2;
            nat_mul(a3058, a3059)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64) -> bool {
    let a3060 = v0;
    let a3061 = 1u64;
    if nat_le(a3060, a3061) {
        true
    } else {
        let a3065 = {
            let a3062 = v1;
            let a3063 = v2;
            let a3064 = 1u64;
            nat_rem(a3062, a3063, a3064)
        };
        let a3066 = 0u64;
        nat_eq(a3065, a3066)
    }
}

pub fn f133(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3067 = v1;
    let a3068 = 1u64;
    if if if nat_le(a3067, a3068) {
        true
    } else {
        let a3069 = v0;
        let a3070 = v2;
        nat_le(a3069, a3070)
    } {
        let a3071 = v3;
        let a3072 = 1u64;
        if nat_le(a3071, a3072) {
            true
        } else {
            let a3079 = {
                let a3077 = v0;
                let a3078 = {
                    let a3075 = {
                        let a3073 = v1;
                        let a3074 = 1u64;
                        nat_sub(a3073, a3074)
                    };
                    let a3076 = v2;
                    nat_mul(a3075, a3076)?
                };
                nat_add(a3077, a3078)?
            };
            let a3080 = v4;
            nat_le(a3079, a3080)
        }
    } else {
        false
    } {
        let a3081 = v5;
        let a3082 = 1u64;
        if nat_le(a3081, a3082) {
            Ok(true)
        } else {
            let a3095 = {
                let a3093 = {
                    let a3087 = v0;
                    let a3088 = {
                        let a3085 = {
                            let a3083 = v1;
                            let a3084 = 1u64;
                            nat_sub(a3083, a3084)
                        };
                        let a3086 = v2;
                        nat_mul(a3085, a3086)?
                    };
                    nat_add(a3087, a3088)?
                };
                let a3094 = {
                    let a3091 = {
                        let a3089 = v3;
                        let a3090 = 1u64;
                        nat_sub(a3089, a3090)
                    };
                    let a3092 = v4;
                    nat_mul(a3091, a3092)?
                };
                nat_add(a3093, a3094)?
            };
            let a3096 = v6;
            Ok(nat_le(a3095, a3096))
        }
    } else {
        Ok(false)
    }
}

pub fn f134(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f161(v0, v1, v6, v7, v8) {
        let a3097 = v5;
        let a3098 = 26u64;
        nat_eq(a3097, a3098)
    } else {
        false
    } {
        let a3099 = v3;
        let a3100 = v7;
        nat_eq(a3099, a3100)
    } else {
        false
    } {
        let a3101 = v4;
        let a3102 = v8;
        nat_eq(a3101, a3102)
    } else {
        false
    } {
        let a3103 = v9;
        let a3104 = 1u64;
        nat_eq(a3103, a3104)
    } else {
        false
    } {
        let a3105 = v10;
        let a3106 = f162(v0);
        nat_eq(a3105, a3106)
    } else {
        false
    } {
        let a3107 = v11;
        let a3108 = v1;
        nat_eq(a3107, a3108)
    } else {
        false
    } {
        let a3109 = v12;
        let a3110 = v6;
        nat_eq(a3109, a3110)
    } else {
        false
    } {
        let a3111 = v13;
        let a3112 = v7;
        nat_eq(a3111, a3112)
    } else {
        false
    } {
        let a3113 = v14;
        let a3114 = v8;
        nat_eq(a3113, a3114)
    } else {
        false
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3115 = v0;
    let a3116 = 0u64;
    if if if nat_eq(a3115, a3116) {
        true
    } else {
        let a3117 = v1;
        let a3118 = 0u64;
        nat_eq(a3117, a3118)
    } {
        true
    } else {
        let a3119 = v2;
        let a3120 = 0u64;
        nat_eq(a3119, a3120)
    } {
        true
    } else {
        let a3121 = v3;
        let a3122 = 0u64;
        nat_eq(a3121, a3122)
    }
}

pub fn f136(v0: Adt3, v1: bool) -> R<u64> {
    f163(v0.clone(), {
        let a3124 = match v0.clone() {
            Adt3::C0(h3123, _, _, _, _, _, _, _) => {
                h3123
            }
        };
        let a3125 = 1u64;
        nat_sub(a3124, a3125)
    }, {
        let a3127 = match v0.clone() {
            Adt3::C0(_, h3126, _, _, _, _, _, _) => {
                h3126
            }
        };
        let a3128 = 1u64;
        nat_sub(a3127, a3128)
    }, {
        let a3130 = match v0.clone() {
            Adt3::C0(_, _, h3129, _, _, _, _, _) => {
                h3129
            }
        };
        let a3131 = 1u64;
        nat_sub(a3130, a3131)
    }, {
        let a3133 = match v0 {
            Adt3::C0(_, _, _, h3132, _, _, _, _) => {
                h3132
            }
        };
        let a3134 = 1u64;
        nat_sub(a3133, a3134)
    }, v1)
}

pub fn f137(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3135 = v0;
    let a3136 = 0u64;
    if nat_eq(a3135, a3136) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a3137 = v0;
            let a3138 = 1u64;
            nat_sub(a3137, a3138)
        };
        let v5: u64 = {
            let a3139 = v4;
            let a3140 = v1;
            let a3141 = 0u64;
            nat_quot(a3139, a3140, a3141)
        };
        let v6: u64 = {
            let a3149 = {
                let a3142 = v5;
                let a3143 = v2;
                nat_mul(a3142, a3143)?
            };
            let a3150 = {
                let a3147 = {
                    let a3144 = v4;
                    let a3145 = v1;
                    let a3146 = 0u64;
                    nat_rem(a3144, a3145, a3146)
                };
                let a3148 = v3;
                nat_mul(a3147, a3148)?
            };
            nat_add(a3149, a3150)?
        };
        let a3151 = v5;
        let a3152 = 0u64;
        if nat_eq(a3151, a3152) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a3161 = {
                    let a3155 = {
                        let a3153 = v5;
                        let a3154 = 1u64;
                        nat_sub(a3153, a3154)
                    };
                    let a3156 = v2;
                    nat_mul(a3155, a3156)?
                };
                let a3162 = {
                    let a3159 = {
                        let a3157 = v1;
                        let a3158 = 1u64;
                        nat_sub(a3157, a3158)
                    };
                    let a3160 = v3;
                    nat_mul(a3159, a3160)?
                };
                nat_add(a3161, a3162)?
            };
            let a3163 = v6;
            let a3164 = v7;
            if nat_lt(a3163, a3164) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f138(v0: Adt3, v1: u64) -> bool {
    let a3165 = v1;
    let a3166 = 0u64;
    if !nat_eq(a3165, a3166) {
        true
    } else {
        let a3168 = match v0 {
            Adt3::C0(_, _, _, _, h3167, _, _, _) => {
                h3167
            }
        };
        let a3169 = 4u64;
        nat_eq(a3168, a3169)
    }
}

pub fn f139(v0: u64, v1: bool) -> R<u64> {
    let a3172 = {
        let a3170 = v0;
        let a3171 = 2u64;
        nat_mul(a3170, a3171)?
    };
    let a3173 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a3172, a3173)
}

pub fn f140(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt9> {
    let m3174 = f164(v0, v1);
    match m3174 {
        None => {
            if v5 {
                let m3175 = f165(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m3175 {
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
                let a3176 = 4u64;
                let a3177 = v28;
                nat_le(a3176, a3177)
            };
            let a3178 = v2;
            let a3179 = 0u64;
            if if if if if nat_eq(a3178, a3179) {
                let a3180 = v4;
                let a3181 = 0u64;
                nat_eq(a3180, a3181)
            } else {
                false
            } {
                let a3182 = v6;
                let a3183 = v14;
                if nat_eq(a3182, a3183) {
                    let a3184 = v7;
                    let a3185 = v15;
                    nat_eq(a3184, a3185)
                } else {
                    false
                }
            } else {
                false
            } {
                let a3186 = v8;
                let a3187 = v16;
                if nat_eq(a3186, a3187) {
                    let a3188 = v9;
                    let a3189 = v17;
                    nat_eq(a3188, a3189)
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
                        let a3190 = v3;
                        let a3191 = 0u64;
                        nat_eq(a3190, a3191)
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
                    let a3196 = {
                        let a3192 = v6;
                        let a3193 = v7;
                        nat_mul(a3192, a3193)?
                    };
                    let a3197 = {
                        let a3194 = v8;
                        let a3195 = v9;
                        nat_mul(a3194, a3195)?
                    };
                    nat_mul(a3196, a3197)?
                }))
            } else {
                Ok(Adt9::C1)
            }
        }
    }
}

pub fn f141(v0: u64) -> u64 {
    let m3198 = f86(v0);
    match m3198 {
        Adt13::C0 => {
            0u64
        }
        Adt13::C1(v1) => {
            let a3200 = match v1.clone() {
                Adt14::C0(_, h3199, _) => {
                    h3199
                }
            };
            let a3201 = 1u64;
            if if nat_eq(a3200, a3201) {
                let a3203 = match v1.clone() {
                    Adt14::C0(_, _, h3202) => {
                        h3202
                    }
                };
                let a3204 = 1u64;
                if if if nat_eq(a3203, a3204) {
                    true
                } else {
                    let a3206 = match v1.clone() {
                        Adt14::C0(_, _, h3205) => {
                            h3205
                        }
                    };
                    let a3207 = 2u64;
                    nat_eq(a3206, a3207)
                } {
                    true
                } else {
                    let a3209 = match v1.clone() {
                        Adt14::C0(_, _, h3208) => {
                            h3208
                        }
                    };
                    let a3210 = 4u64;
                    nat_eq(a3209, a3210)
                } {
                    true
                } else {
                    let a3212 = match v1.clone() {
                        Adt14::C0(_, _, h3211) => {
                            h3211
                        }
                    };
                    let a3213 = 8u64;
                    nat_eq(a3212, a3213)
                }
            } else {
                false
            } {
                match v1 {
                    Adt14::C0(_, _, h3214) => {
                        h3214
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f142(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3219 = {
        let a3217 = {
            let a3215 = v0;
            let a3216 = v1;
            nat_sub(a3215, a3216)
        };
        let a3218 = 1u64;
        nat_sub(a3217, a3218)
    };
    let a3220 = v2;
    nat_add(a3219, a3220)
}

pub fn f143(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f100({
        let a3221 = v0;
        let a3222 = v4;
        let a3223 = 0u64;
        nat_rem(a3221, a3222, a3223)
    }, {
        let a3224 = v1;
        let a3225 = v5;
        let a3226 = 0u64;
        nat_rem(a3224, a3225, a3226)
    }, {
        let a3227 = v2;
        let a3228 = v6;
        let a3229 = 0u64;
        nat_rem(a3227, a3228, a3229)
    }, {
        let a3230 = v3;
        let a3231 = v7;
        let a3232 = 0u64;
        nat_rem(a3230, a3231, a3232)
    }, v8, v9, v10, v11)
}

pub fn f144(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3233 = v1;
    let a3234 = 2147483648u64;
    if nat_lt(a3233, a3234) {
        let a3245 = {
            let a3243 = {
                let a3238 = {
                    let a3235 = v0;
                    let a3236 = v2;
                    let a3237 = 0u64;
                    nat_rem(a3235, a3236, a3237)
                };
                let a3239 = v2;
                nat_add(a3238, a3239)?
            };
            let a3244 = {
                let a3240 = v1;
                let a3241 = v2;
                let a3242 = 0u64;
                nat_rem(a3240, a3241, a3242)
            };
            nat_sub(a3243, a3244)
        };
        let a3246 = v2;
        let a3247 = 0u64;
        Ok(nat_rem(a3245, a3246, a3247))
    } else {
        let a3250 = {
            let a3248 = v0;
            let a3249 = f73(v1);
            nat_add(a3248, a3249)?
        };
        let a3251 = v2;
        let a3252 = 0u64;
        Ok(nat_rem(a3250, a3251, a3252))
    }
}

pub fn f145(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3255 = f72(v1);
    let a3256 = {
        let a3253 = v0;
        let a3254 = f73(v1);
        nat_add(a3253, a3254)?
    };
    if nat_le(a3255, a3256) {
        let a3257 = f146(v0, v1)?;
        let a3258 = v2;
        Ok(nat_lt(a3257, a3258))
    } else {
        Ok(false)
    }
}

pub fn f146(v0: u64, v1: u64) -> R<u64> {
    let a3261 = {
        let a3259 = v0;
        let a3260 = f73(v1);
        nat_add(a3259, a3260)?
    };
    let a3262 = f72(v1);
    Ok(nat_sub(a3261, a3262))
}

pub fn f147(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3263 = v0;
    let a3264 = v1;
    if nat_lt(a3263, a3264) {
        let a3265 = v1;
        let a3266 = v0;
        Ok(nat_sub(a3265, a3266))
    } else {
        let a3269 = v0;
        let a3270 = {
            let a3267 = v1;
            let a3268 = v2;
            nat_add(a3267, a3268)?
        };
        if nat_lt(a3269, a3270) {
            let a3271 = v0;
            let a3272 = v1;
            Ok(nat_sub(a3271, a3272))
        } else {
            let a3279 = {
                let a3275 = 2u64;
                let a3276 = {
                    let a3273 = v2;
                    let a3274 = 1u64;
                    nat_sub(a3273, a3274)
                };
                nat_mul(a3275, a3276)?
            };
            let a3280 = {
                let a3277 = v0;
                let a3278 = v1;
                nat_sub(a3277, a3278)
            };
            Ok(nat_sub(a3279, a3280))
        }
    }
}

pub fn f148(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3283 = {
        let a3281 = v2;
        let a3282 = f163(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a3281, a3282)?
    };
    let a3284 = f163(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3283, a3284))
}

pub fn f149(v0: u64, v1: u64, v2: u64) -> bool {
    let a3285 = v0;
    let a3286 = 0u64;
    if nat_eq(a3285, a3286) {
        let a3287 = v2;
        let a3288 = v1;
        nat_le(a3287, a3288)
    } else {
        let a3289 = v0;
        let a3290 = 1u64;
        if nat_eq(a3289, a3290) {
            let a3291 = v2;
            let a3292 = v1;
            nat_lt(a3291, a3292)
        } else {
            let a3293 = v0;
            let a3294 = 2u64;
            if nat_eq(a3293, a3294) {
                let a3295 = v1;
                let a3296 = v2;
                nat_le(a3295, a3296)
            } else {
                let a3297 = v0;
                let a3298 = 3u64;
                if nat_eq(a3297, a3298) {
                    let a3299 = v1;
                    let a3300 = v2;
                    nat_lt(a3299, a3300)
                } else {
                    let a3301 = v1;
                    let a3302 = v2;
                    nat_eq(a3301, a3302)
                }
            }
        }
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: u64) -> R<Adt8> {
    let a3303 = v0;
    let a3304 = 0u64;
    if nat_eq(a3303, a3304) {
        Ok(f158(0u64))
    } else {
        let a3307 = {
            let a3305 = v0;
            let a3306 = 1u64;
            nat_add(a3305, a3306)?
        };
        let a3308 = f98(v1.clone(), 13u64, v2)?;
        if nat_eq(a3307, a3308) {
            Ok(Adt8::C4({
                let a3309 = v2;
                let a3310 = 0u64;
                if nat_eq(a3309, a3310) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3311 = v0;
                let a3312 = 1u64;
                nat_sub(a3311, a3312)
            };
            if f138(v1.clone(), v2) {
                let v4: u64 = f139(match v1.clone() {
                    Adt3::C0(h3313, _, _, _, _, _, _, _) => {
                        h3313
                    }
                }, false)?;
                let v5: u64 = {
                    let a3314 = v3;
                    let a3315 = v4;
                    let a3316 = 0u64;
                    nat_quot(a3314, a3315, a3316)
                };
                let v6: u64 = {
                    let a3317 = v3;
                    let a3318 = v4;
                    let a3319 = 0u64;
                    nat_rem(a3317, a3318, a3319)
                };
                let a3322 = {
                    let a3320 = v6;
                    let a3321 = 1u64;
                    nat_add(a3320, a3321)?
                };
                let a3323 = v4;
                if nat_eq(a3322, a3323) {
                    Ok(Adt8::C4({
                        let a3324 = v2;
                        let a3325 = 0u64;
                        if nat_eq(a3324, a3325) {
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
                    let a3326 = v3;
                    let a3327 = 2u64;
                    let a3328 = 0u64;
                    nat_quot(a3326, a3327, a3328)
                }, {
                    let a3329 = v3;
                    let a3330 = 2u64;
                    let a3331 = 0u64;
                    nat_rem(a3329, a3330, a3331)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f151(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt8> {
    let a3332 = v0;
    let a3333 = 0u64;
    if nat_eq(a3332, a3333) {
        Ok(f158(1u64))
    } else {
        f166({
            let a3342 = {
                let a3335 = v1;
                let a3336 = match v2.clone() {
                    Adt3::C0(h3334, _, _, _, _, _, _, _) => {
                        h3334
                    }
                };
                nat_mul(a3335, a3336)?
            };
            let a3343 = {
                let a3339 = {
                    let a3337 = v0;
                    let a3338 = 1u64;
                    nat_sub(a3337, a3338)
                };
                let a3340 = 2u64;
                let a3341 = 0u64;
                nat_quot(a3339, a3340, a3341)
            };
            nat_add(a3342, a3343)?
        }, {
            let a3346 = {
                let a3344 = v0;
                let a3345 = 1u64;
                nat_sub(a3344, a3345)
            };
            let a3347 = 2u64;
            let a3348 = 0u64;
            nat_rem(a3346, a3347, a3348)
        }, v2, v3, 1u64)
    }
}

pub fn f152(v0: u64, v1: u64) -> R<u64> {
    let a3349 = v0;
    let a3350 = f75(v1)?;
    let a3351 = 0u64;
    Ok(nat_rem(a3349, a3350, a3351))
}

pub fn f153(v0: u64, v1: u64) -> R<u64> {
    let a3352 = 4u64;
    let a3353 = f167(v0, v1)?;
    nat_mul(a3352, a3353)
}

pub fn f154(v0: u64, v1: u64) -> R<u64> {
    let a3354 = 4u64;
    let a3355 = f168(v0, v1)?;
    nat_mul(a3354, a3355)
}

pub fn f155(v0: u64, v1: u64) -> R<bool> {
    let a3361 = {
        let a3358 = {
            let a3356 = f152(v0, v1)?;
            let a3357 = 2u64;
            nat_sub(a3356, a3357)
        };
        let a3359 = 2u64;
        let a3360 = 0u64;
        nat_rem(a3358, a3359, a3360)
    };
    let a3362 = 0u64;
    Ok(nat_eq(a3361, a3362))
}

pub fn f156(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3363 = f86(v0);
    match m3363 {
        Adt13::C0 => {
            Ok(None::<u64>)
        }
        Adt13::C1(v13) => {
            if if if if if f169(v0) {
                let a3364 = v1;
                let a3365 = v7;
                nat_lt(a3364, a3365)
            } else {
                false
            } {
                f123(v8, v2)
            } else {
                false
            } {
                let a3366 = v3;
                let a3367 = f170(v4, v5, v6)?;
                nat_lt(a3366, a3367)
            } else {
                false
            } {
                let a3372 = {
                    let a3369 = v7;
                    let a3370 = match v13.clone() {
                        Adt14::C0(_, h3368, _) => {
                            h3368
                        }
                    };
                    let a3371 = 0u64;
                    nat_rem(a3369, a3370, a3371)
                };
                let a3373 = 0u64;
                nat_eq(a3372, a3373)
            } else {
                false
            } {
                Ok(Some({
                    let a3402 = {
                        let a3392 = {
                            let a3382 = {
                                let a3378 = {
                                    let a3375 = v1;
                                    let a3376 = match v13 {
                                        Adt14::C0(_, h3374, _) => {
                                            h3374
                                        }
                                    };
                                    let a3377 = 0u64;
                                    nat_quot(a3375, a3376, a3377)
                                };
                                let a3379 = v9;
                                nat_mul(a3378, a3379)?
                            };
                            let a3383 = {
                                let a3380 = v2;
                                let a3381 = v10;
                                nat_mul(a3380, a3381)?
                            };
                            nat_add(a3382, a3383)?
                        };
                        let a3393 = {
                            let a3390 = {
                                let a3387 = {
                                    let a3384 = v3;
                                    let a3385 = v4;
                                    let a3386 = 0u64;
                                    nat_quot(a3384, a3385, a3386)
                                };
                                let a3388 = v5;
                                let a3389 = 0u64;
                                nat_rem(a3387, a3388, a3389)
                            };
                            let a3391 = v11;
                            nat_mul(a3390, a3391)?
                        };
                        nat_add(a3392, a3393)?
                    };
                    let a3403 = {
                        let a3400 = {
                            let a3397 = {
                                let a3394 = v3;
                                let a3395 = v4;
                                let a3396 = 0u64;
                                nat_quot(a3394, a3395, a3396)
                            };
                            let a3398 = v5;
                            let a3399 = 0u64;
                            nat_quot(a3397, a3398, a3399)
                        };
                        let a3401 = v12;
                        nat_mul(a3400, a3401)?
                    };
                    nat_add(a3402, a3403)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f157(v0: u64) -> u64 {
    let a3404 = v0;
    let a3405 = 30u64;
    if nat_eq(a3404, a3405) {
        4u64
    } else {
        3u64
    }
}

pub fn f158(v0: u64) -> Adt8 {
    Adt8::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f159(v0: u64) -> bool {
    let a3406 = 0u64;
    let a3407 = v0;
    if nat_lt(a3406, a3407) {
        let a3408 = v0;
        let a3409 = 2139095040u64;
        nat_le(a3408, a3409)
    } else {
        false
    }
}

pub fn f160(v0: u64) -> u64 {
    let a3410 = v0;
    let a3411 = 40u64;
    if nat_lt(a3410, a3411) {
        let a3412 = v0;
        let a3413 = 20u64;
        if nat_lt(a3412, a3413) {
            let a3414 = v0;
            let a3415 = 10u64;
            if nat_lt(a3414, a3415) {
                let a3416 = v0;
                let a3417 = 5u64;
                if nat_lt(a3416, a3417) {
                    let a3418 = v0;
                    let a3419 = 2u64;
                    if nat_lt(a3418, a3419) {
                        let a3420 = v0;
                        let a3421 = 1u64;
                        if nat_lt(a3420, a3421) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a3422 = v0;
                        let a3423 = 3u64;
                        if nat_lt(a3422, a3423) {
                            4u64
                        } else {
                            let a3424 = v0;
                            let a3425 = 4u64;
                            if nat_lt(a3424, a3425) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a3426 = v0;
                    let a3427 = 7u64;
                    if nat_lt(a3426, a3427) {
                        let a3428 = v0;
                        let a3429 = 6u64;
                        if nat_lt(a3428, a3429) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a3430 = v0;
                        let a3431 = 8u64;
                        if nat_lt(a3430, a3431) {
                            128u64
                        } else {
                            let a3432 = v0;
                            let a3433 = 9u64;
                            if nat_lt(a3432, a3433) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a3434 = v0;
                let a3435 = 15u64;
                if nat_lt(a3434, a3435) {
                    let a3436 = v0;
                    let a3437 = 12u64;
                    if nat_lt(a3436, a3437) {
                        let a3438 = v0;
                        let a3439 = 11u64;
                        if nat_lt(a3438, a3439) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a3440 = v0;
                        let a3441 = 13u64;
                        if nat_lt(a3440, a3441) {
                            4096u64
                        } else {
                            let a3442 = v0;
                            let a3443 = 14u64;
                            if nat_lt(a3442, a3443) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a3444 = v0;
                    let a3445 = 17u64;
                    if nat_lt(a3444, a3445) {
                        let a3446 = v0;
                        let a3447 = 16u64;
                        if nat_lt(a3446, a3447) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a3448 = v0;
                        let a3449 = 18u64;
                        if nat_lt(a3448, a3449) {
                            131072u64
                        } else {
                            let a3450 = v0;
                            let a3451 = 19u64;
                            if nat_lt(a3450, a3451) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a3452 = v0;
            let a3453 = 30u64;
            if nat_lt(a3452, a3453) {
                let a3454 = v0;
                let a3455 = 25u64;
                if nat_lt(a3454, a3455) {
                    let a3456 = v0;
                    let a3457 = 22u64;
                    if nat_lt(a3456, a3457) {
                        let a3458 = v0;
                        let a3459 = 21u64;
                        if nat_lt(a3458, a3459) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a3460 = v0;
                        let a3461 = 23u64;
                        if nat_lt(a3460, a3461) {
                            4194304u64
                        } else {
                            let a3462 = v0;
                            let a3463 = 24u64;
                            if nat_lt(a3462, a3463) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a3464 = v0;
                    let a3465 = 27u64;
                    if nat_lt(a3464, a3465) {
                        let a3466 = v0;
                        let a3467 = 26u64;
                        if nat_lt(a3466, a3467) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a3468 = v0;
                        let a3469 = 28u64;
                        if nat_lt(a3468, a3469) {
                            134217728u64
                        } else {
                            let a3470 = v0;
                            let a3471 = 29u64;
                            if nat_lt(a3470, a3471) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a3472 = v0;
                let a3473 = 35u64;
                if nat_lt(a3472, a3473) {
                    let a3474 = v0;
                    let a3475 = 32u64;
                    if nat_lt(a3474, a3475) {
                        let a3476 = v0;
                        let a3477 = 31u64;
                        if nat_lt(a3476, a3477) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3478 = v0;
                        let a3479 = 33u64;
                        if nat_lt(a3478, a3479) {
                            4294967296u64
                        } else {
                            let a3480 = v0;
                            let a3481 = 34u64;
                            if nat_lt(a3480, a3481) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3482 = v0;
                    let a3483 = 37u64;
                    if nat_lt(a3482, a3483) {
                        let a3484 = v0;
                        let a3485 = 36u64;
                        if nat_lt(a3484, a3485) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3486 = v0;
                        let a3487 = 38u64;
                        if nat_lt(a3486, a3487) {
                            137438953472u64
                        } else {
                            let a3488 = v0;
                            let a3489 = 39u64;
                            if nat_lt(a3488, a3489) {
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
    let a3490 = v0;
    let a3491 = 26u64;
    if nat_eq(a3490, a3491) {
        26u64
    } else {
        0u64
    }
}

pub fn f163(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3512 = {
        let a3506 = {
            let a3500 = {
                let a3494 = v1;
                let a3495 = if v5 {
                    f73(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3492, _, _, _) => {
                            h3492
                        }
                    })
                } else {
                    f72(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3493, _, _, _) => {
                            h3493
                        }
                    })
                };
                nat_mul(a3494, a3495)?
            };
            let a3501 = {
                let a3498 = v2;
                let a3499 = if v5 {
                    f73(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3496, _, _) => {
                            h3496
                        }
                    })
                } else {
                    f72(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3497, _, _) => {
                            h3497
                        }
                    })
                };
                nat_mul(a3498, a3499)?
            };
            nat_add(a3500, a3501)?
        };
        let a3507 = {
            let a3504 = v3;
            let a3505 = if v5 {
                f73(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3502, _) => {
                        h3502
                    }
                })
            } else {
                f72(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3503, _) => {
                        h3503
                    }
                })
            };
            nat_mul(a3504, a3505)?
        };
        nat_add(a3506, a3507)?
    };
    let a3513 = {
        let a3510 = v4;
        let a3511 = if v5 {
            f73(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3508) => {
                    h3508
                }
            })
        } else {
            f72(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3509) => {
                    h3509
                }
            })
        };
        nat_mul(a3510, a3511)?
    };
    nat_add(a3512, a3513)
}

pub fn f164(v0: u64, v1: u64) -> Option<u64> {
    let a3514 = v0;
    let a3515 = 2u64;
    if nat_eq(a3514, a3515) {
        Some(0u64)
    } else {
        let a3516 = v0;
        let a3517 = 5u64;
        if nat_eq(a3516, a3517) {
            Some(1u64)
        } else {
            let a3518 = v0;
            let a3519 = 6u64;
            if nat_eq(a3518, a3519) {
                Some(2u64)
            } else {
                let a3520 = v0;
                let a3521 = 7u64;
                if nat_eq(a3520, a3521) {
                    Some(3u64)
                } else {
                    let a3522 = v0;
                    let a3523 = 9u64;
                    if nat_eq(a3522, a3523) {
                        Some(4u64)
                    } else {
                        let a3524 = v0;
                        let a3525 = 8u64;
                        if nat_eq(a3524, a3525) {
                            Some(5u64)
                        } else {
                            let a3526 = v0;
                            let a3527 = 89u64;
                            if nat_eq(a3526, a3527) {
                                let a3528 = v1;
                                let a3529 = 2u64;
                                if nat_eq(a3528, a3529) {
                                    Some(6u64)
                                } else {
                                    let a3530 = v1;
                                    let a3531 = 0u64;
                                    if nat_eq(a3530, a3531) {
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
    let a3532 = v0;
    let a3533 = 30u64;
    if nat_eq(a3532, a3533) {
        let a3534 = v2;
        let a3535 = 0u64;
        if if if nat_eq(a3534, a3535) {
            true
        } else {
            let a3536 = v2;
            let a3537 = 1u64;
            nat_eq(a3536, a3537)
        } {
            let a3538 = v1;
            let a3539 = 0u64;
            if nat_eq(a3538, a3539) {
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
            let a3540 = v1;
            let a3541 = 0u64;
            if if nat_eq(a3540, a3541) {
                true
            } else {
                let a3542 = v2;
                let a3543 = 0u64;
                nat_eq(a3542, a3543)
            } {
                true
            } else {
                let a3544 = v1;
                let a3545 = v2;
                nat_eq(a3544, a3545)
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
    let a3546 = v1;
    let a3547 = 0u64;
    if nat_eq(a3546, a3547) {
        Ok(Adt8::C4({
            let a3548 = v3;
            let a3549 = 0u64;
            if nat_eq(a3548, a3549) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f106(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt8::C4({
            let a3550 = v3;
            let a3551 = 0u64;
            if nat_eq(a3550, a3551) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f167(v0: u64, v1: u64) -> R<u64> {
    let a3552 = v0;
    let a3553 = f75(v1)?;
    let a3554 = 0u64;
    Ok(nat_quot(a3552, a3553, a3554))
}

pub fn f168(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f167(v0, v1)?;
    let v3: u64 = {
        let a3557 = {
            let a3555 = f152(v0, v1)?;
            let a3556 = 2u64;
            nat_sub(a3555, a3556)
        };
        let a3558 = 2u64;
        let a3559 = 0u64;
        nat_quot(a3557, a3558, a3559)
    };
    let v4: u64 = {
        let a3560 = v2;
        let a3561 = v1;
        nat_mul(a3560, a3561)?
    };
    if f155(v0, v1)? {
        let a3562 = v4;
        let a3563 = v3;
        nat_add(a3562, a3563)
    } else {
        let a3575 = {
            let a3571 = v4;
            let a3572 = {
                let a3569 = {
                    let a3564 = v1;
                    let a3565 = 1u64;
                    nat_sub(a3564, a3565)
                };
                let a3570 = {
                    let a3566 = v2;
                    let a3567 = v1;
                    let a3568 = 0u64;
                    nat_rem(a3566, a3567, a3568)
                };
                nat_mul(a3569, a3570)?
            };
            nat_sub(a3571, a3572)
        };
        let a3576 = {
            let a3573 = v3;
            let a3574 = v1;
            nat_mul(a3573, a3574)?
        };
        nat_add(a3575, a3576)
    }
}

pub fn f169(v0: u64) -> bool {
    let a3577 = v0;
    let a3578 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3577, a3578) {
        true
    } else {
        let a3579 = v0;
        let a3580 = 1u64;
        nat_eq(a3579, a3580)
    } {
        true
    } else {
        let a3581 = v0;
        let a3582 = 2u64;
        nat_eq(a3581, a3582)
    } {
        true
    } else {
        let a3583 = v0;
        let a3584 = 3u64;
        nat_eq(a3583, a3584)
    } {
        true
    } else {
        let a3585 = v0;
        let a3586 = 6u64;
        nat_eq(a3585, a3586)
    } {
        true
    } else {
        let a3587 = v0;
        let a3588 = 7u64;
        nat_eq(a3587, a3588)
    } {
        true
    } else {
        let a3589 = v0;
        let a3590 = 8u64;
        nat_eq(a3589, a3590)
    } {
        true
    } else {
        let a3591 = v0;
        let a3592 = 10u64;
        nat_eq(a3591, a3592)
    } {
        true
    } else {
        let a3593 = v0;
        let a3594 = 11u64;
        nat_eq(a3593, a3594)
    } {
        true
    } else {
        let a3595 = v0;
        let a3596 = 12u64;
        nat_eq(a3595, a3596)
    } {
        true
    } else {
        let a3597 = v0;
        let a3598 = 13u64;
        nat_eq(a3597, a3598)
    } {
        true
    } else {
        let a3599 = v0;
        let a3600 = 14u64;
        nat_eq(a3599, a3600)
    } {
        true
    } else {
        let a3601 = v0;
        let a3602 = 16u64;
        nat_eq(a3601, a3602)
    } {
        true
    } else {
        let a3603 = v0;
        let a3604 = 17u64;
        nat_eq(a3603, a3604)
    } {
        true
    } else {
        let a3605 = v0;
        let a3606 = 18u64;
        nat_eq(a3605, a3606)
    } {
        true
    } else {
        let a3607 = v0;
        let a3608 = 19u64;
        nat_eq(a3607, a3608)
    } {
        true
    } else {
        let a3609 = v0;
        let a3610 = 20u64;
        nat_eq(a3609, a3610)
    } {
        true
    } else {
        let a3611 = v0;
        let a3612 = 21u64;
        nat_eq(a3611, a3612)
    } {
        true
    } else {
        let a3613 = v0;
        let a3614 = 22u64;
        nat_eq(a3613, a3614)
    } {
        true
    } else {
        let a3615 = v0;
        let a3616 = 23u64;
        nat_eq(a3615, a3616)
    } {
        true
    } else {
        let a3617 = v0;
        let a3618 = 26u64;
        nat_eq(a3617, a3618)
    } {
        true
    } else {
        let a3619 = v0;
        let a3620 = 29u64;
        nat_eq(a3619, a3620)
    } {
        true
    } else {
        let a3621 = v0;
        let a3622 = 30u64;
        nat_eq(a3621, a3622)
    } {
        true
    } else {
        let a3623 = v0;
        let a3624 = 34u64;
        nat_eq(a3623, a3624)
    } {
        true
    } else {
        let a3625 = v0;
        let a3626 = 35u64;
        nat_eq(a3625, a3626)
    } {
        true
    } else {
        let a3627 = v0;
        let a3628 = 39u64;
        nat_eq(a3627, a3628)
    } {
        true
    } else {
        let a3629 = v0;
        let a3630 = 40u64;
        nat_eq(a3629, a3630)
    } {
        true
    } else {
        let a3631 = v0;
        let a3632 = 41u64;
        nat_eq(a3631, a3632)
    } {
        true
    } else {
        let a3633 = v0;
        let a3634 = 42u64;
        nat_eq(a3633, a3634)
    } {
        true
    } else {
        let a3635 = v0;
        let a3636 = 43u64;
        nat_eq(a3635, a3636)
    }
}

pub fn f170(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3637 = v0;
    let a3638 = 0u64;
    if if if nat_eq(a3637, a3638) {
        true
    } else {
        let a3639 = v1;
        let a3640 = 0u64;
        nat_eq(a3639, a3640)
    } {
        true
    } else {
        let a3641 = v2;
        let a3642 = 0u64;
        nat_eq(a3641, a3642)
    } {
        Ok(0u64)
    } else {
        let a3645 = {
            let a3643 = v0;
            let a3644 = v1;
            nat_mul(a3643, a3644)?
        };
        let a3646 = v2;
        nat_mul(a3645, a3646)
    }
}

pub fn f171(v0: u64) -> bool {
    let m3647 = f86(v0);
    match m3647 {
        Adt13::C0 => {
            false
        }
        Adt13::C1(_) => {
            true
        }
    }
}

pub fn f172(v0: u64, v1: u64) -> bool {
    let a3648 = v0;
    let a3649 = 1u64;
    if nat_eq(a3648, a3649) {
        true
    } else {
        let v2: u64 = f175(v0);
        let a3650 = 0u64;
        let a3651 = v2;
        if nat_lt(a3650, a3651) {
            let a3655 = {
                let a3652 = v1;
                let a3653 = v2;
                let a3654 = 1u64;
                nat_rem(a3652, a3653, a3654)
            };
            let a3656 = 0u64;
            nat_eq(a3655, a3656)
        } else {
            false
        }
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt15> {
    let a3657 = v0;
    let a3658 = 28u64;
    if if if if nat_eq(a3657, a3658) {
        let a3659 = v1;
        let a3660 = 0u64;
        nat_eq(a3659, a3660)
    } else {
        false
    } {
        let a3661 = v2;
        let a3662 = 0u64;
        nat_eq(a3661, a3662)
    } else {
        false
    } {
        let a3663 = v3;
        let a3664 = 0u64;
        nat_eq(a3663, a3664)
    } else {
        false
    } {
        let a3665 = v4;
        let a3666 = v8;
        if if if if if if if if nat_eq(a3665, a3666) {
            let a3667 = v5;
            let a3668 = v12;
            nat_eq(a3667, a3668)
        } else {
            false
        } {
            let a3669 = v9;
            let a3670 = v13;
            nat_eq(a3669, a3670)
        } else {
            false
        } {
            let a3671 = v10;
            let a3672 = v14;
            nat_eq(a3671, a3672)
        } else {
            false
        } {
            let a3673 = v11;
            let a3674 = v15;
            nat_eq(a3673, a3674)
        } else {
            false
        } {
            let a3675 = 0u64;
            let a3676 = v6;
            nat_lt(a3675, a3676)
        } else {
            false
        } {
            let a3677 = 0u64;
            let a3678 = v7;
            nat_lt(a3677, a3678)
        } else {
            false
        } {
            let a3682 = {
                let a3679 = v10;
                let a3680 = v6;
                let a3681 = 1u64;
                nat_rem(a3679, a3680, a3681)
            };
            let a3683 = 0u64;
            if nat_eq(a3682, a3683) {
                let a3687 = {
                    let a3684 = v11;
                    let a3685 = v7;
                    let a3686 = 1u64;
                    nat_rem(a3684, a3685, a3686)
                };
                let a3688 = 0u64;
                nat_eq(a3687, a3688)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt15::C2(v5, v4, v9, v10, v11, {
                let a3689 = v10;
                let a3690 = v6;
                let a3691 = 0u64;
                nat_quot(a3689, a3690, a3691)
            }, {
                let a3692 = v11;
                let a3693 = v7;
                let a3694 = 0u64;
                nat_quot(a3692, a3693, a3694)
            }, {
                let a3695 = v5;
                let a3696 = v4;
                nat_mul(a3695, a3696)?
            }, {
                let a3697 = v4;
                let a3698 = v9;
                nat_mul(a3697, a3698)?
            }, {
                let a3699 = v5;
                let a3700 = v9;
                nat_mul(a3699, a3700)?
            }))
        } else {
            Ok(Adt15::C1)
        }
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f174(v0: u64) -> bool {
    let a3701 = v0;
    let a3702 = 0u64;
    if if nat_eq(a3701, a3702) {
        true
    } else {
        let a3703 = v0;
        let a3704 = 1u64;
        nat_eq(a3703, a3704)
    } {
        true
    } else {
        let a3705 = v0;
        let a3706 = 30u64;
        nat_eq(a3705, a3706)
    }
}

pub fn f175(v0: u64) -> u64 {
    if f169(v0) {
        let m3707 = f86(v0);
        match m3707 {
            Adt13::C0 => {
                0u64
            }
            Adt13::C1(v1) => {
                let a3709 = 1u64;
                let a3710 = match v1.clone() {
                    Adt14::C0(_, h3708, _) => {
                        h3708
                    }
                };
                if if nat_lt(a3709, a3710) {
                    true
                } else {
                    let a3711 = v0;
                    let a3712 = 43u64;
                    nat_eq(a3711, a3712)
                } {
                    match v1 {
                        Adt14::C0(_, h3713, _) => {
                            h3713
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

pub fn run(p0: Adt0) -> R<Adt8> {
    f0(p0)
}
