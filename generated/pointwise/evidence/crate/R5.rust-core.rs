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
    C4(Adt8, u64, u64, u64),
    C5(u64),
    C6(u64, Adt3, Adt3, Adt4, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
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
    C28(Adt7),
    C29(Adt3),
    C30(Adt6, u64),
    C31(u64, u64),
    C32(u64),
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
    C0(Adt6, Adt6, bool, bool, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, Adt3, Adt3, Adt3, Adt6, Adt6, Adt6, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Adt10),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
    C6(u64, u64, u64, bool),
    C7(Adt11, Adt12, Adt12, Adt13, u64, u64),
    C8(Option<u64>),
    C9(Option<u64>),
    C10(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
    C5(u64),
    C6(Adt8),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64),
    C1(u64),
    C2(u64, u64, bool),
    C3(u64, u64, bool),
    C4(u64, u64, u64, u64),
    C5(u64),
    C6,
    C7(u64, u64, u64),
    C8(u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4,
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64),
    C1(u64),
    C2,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u64),
    C1(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt17 {
    C0,
    C1(Adt18),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt9> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6) => {
            let a2 = v1;
            let a3 = 64u64;
            if nat_eq(a2, a3) {
                Ok(Adt9::C0(f1(v2, v6)?))
            } else {
                let m4 = v3;
                match m4 {
                    None => {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                    Some(v7) => {
                        let a5 = v1;
                        let a6 = 48u64;
                        if if nat_eq(a5, a6) {
                            true
                        } else {
                            let a7 = v1;
                            let a8 = 56u64;
                            nat_eq(a7, a8)
                        } {
                            Ok(Adt9::C0(f2({
                                let a9 = v1;
                                let a10 = 56u64;
                                nat_eq(a9, a10)
                            }, v2, v7, v4, v6)?))
                        } else {
                            let a11 = v1;
                            let a12 = 53u64;
                            if if nat_eq(a11, a12) {
                                true
                            } else {
                                let a13 = v1;
                                let a14 = 54u64;
                                nat_eq(a13, a14)
                            } {
                                Ok(Adt9::C0(f3({
                                    let a15 = v1;
                                    let a16 = 54u64;
                                    nat_eq(a15, a16)
                                }, v2, v7, v4, v6)?))
                            } else {
                                let a17 = v1;
                                let a18 = 78u64;
                                if nat_eq(a17, a18) {
                                    Ok(Adt9::C0(f4(v2, v7, v4, v5, v6)?))
                                } else {
                                    let a19 = v1;
                                    let a20 = 23u64;
                                    if if if nat_eq(a19, a20) {
                                        true
                                    } else {
                                        let a21 = v1;
                                        let a22 = 26u64;
                                        nat_eq(a21, a22)
                                    } {
                                        true
                                    } else {
                                        let a23 = v1;
                                        let a24 = 25u64;
                                        nat_eq(a23, a24)
                                    } {
                                        Ok(Adt9::C0(f5(v1, v2, v7, v4, v6)?))
                                    } else {
                                        let a25 = v1;
                                        let a26 = 24u64;
                                        if nat_eq(a25, a26) {
                                            Ok(Adt9::C0(f6(v2, v7, v6)?))
                                        } else {
                                            let a27 = v1;
                                            let a28 = 27u64;
                                            if nat_eq(a27, a28) {
                                                Ok(Adt9::C0(f7(v2, v7, v6)?))
                                            } else {
                                                let a29 = v1;
                                                let a30 = 3u64;
                                                if nat_eq(a29, a30) {
                                                    Ok(Adt9::C0(f8(v7, v4, v5, v6)?))
                                                } else {
                                                    f9(v1, v2, v7, v4, v6)
                                                }
                                            }
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
            f12(v13, v14, v15, v16, v17)
        }
        Adt0::C2(v28) => {
            f14(v28)
        }
        Adt0::C3(v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41) => {
            Ok(f15(v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41))
        }
        Adt0::C4(v8, v9, v10, v11) => {
            let a32 = 0u64;
            let a33 = match v8.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h31, _, _, _, _, _, _, _) => {
                    h31
                }
            };
            if nat_lt(a32, a33) {
                f10(v8, v9, v10, v11)
            } else {
                f11(v8, v9, v10, v11)
            }
        }
        Adt0::C5(v12) => {
            Ok(Adt9::C9({
                let a34 = 3240099840u64;
                let a35 = v12;
                if if nat_le(a34, a35) {
                    let a36 = v12;
                    let a37 = 4286578688u64;
                    nat_le(a36, a37)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a38 = 1092616192u64;
                    let a39 = v12;
                    if if nat_le(a38, a39) {
                        let a40 = v12;
                        let a41 = 2139095040u64;
                        nat_le(a40, a41)
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
        Adt0::C6(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
            f13(v18, v19, v20, v21, v22, true, v23, v24, v25, v26, v27)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f16(v1.clone(), 1u64)?;
    let a43 = match v1.clone() {
        Adt2::C0(h42, _, _, _, _, _, _, _, _, _, _) => {
            h42
        }
    };
    let a44 = 0u64;
    if if if if if if if if nat_eq(a43, a44) {
        let a46 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h45, _, _, _, _, _) => {
                h45
            }
        };
        let a47 = 4u64;
        nat_eq(a46, a47)
    } else {
        false
    } {
        f17(match v0.clone() {
            Adt1::C0(h48, _, _, _, _, _, _, _, _, _, _, _) => {
                h48
            }
        })
    } else {
        false
    } {
        f17(match v0.clone() {
            Adt1::C0(_, h49, _, _, _, _, _, _, _, _, _, _) => {
                h49
            }
        })
    } else {
        false
    } {
        let a51 = 0u64;
        let a52 = match v0.clone() {
            Adt1::C0(_, _, h50, _, _, _, _, _, _, _, _, _) => {
                h50
            }
        };
        nat_lt(a51, a52)
    } else {
        false
    } {
        let a54 = match v0.clone() {
            Adt1::C0(_, _, h53, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        };
        let a55 = 2139095040u64;
        nat_lt(a54, a55)
    } else {
        false
    } {
        let a56 = v2;
        let a57 = 9223372036854775808u64;
        nat_lt(a56, a57)
    } else {
        false
    } {
        let a59 = f18(v2)?;
        let a60 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h58, _) => {
                h58
            }
        };
        nat_le(a59, a60)
    } else {
        false
    } {
        Ok(Adt10::C4(f19(v2)?, 4u64, f20(v1.clone(), 1u64), f20(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h61, _, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        }, match v0.clone() {
            Adt1::C0(_, h62, _, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        }, match v0 {
            Adt1::C0(_, _, h63, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m64 = v3;
    match m64 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f21(v0, v2.clone());
            let v7: Adt3 = f21(v0, v5.clone());
            let v8: Adt3 = f21(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h65, _, _, _, _, _, _, _) => {
                    h65
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h66, _, _, _, _, _, _) => {
                    h66
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h67, _, _, _, _, _) => {
                    h67
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h68, _, _, _, _) => {
                    h68
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h69, _, _, _, _, _, _, _) => {
                    h69
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h70, _, _, _, _, _, _) => {
                    h70
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h71, _, _, _, _, _) => {
                    h71
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _, _) => {
                    h72
                }
            };
            let v17: u64 = {
                let a77 = v13;
                let a78 = {
                    let a75 = {
                        let a73 = v9;
                        let a74 = 1u64;
                        nat_sub(a73, a74)
                    };
                    let a76 = v16;
                    nat_mul(a75, a76)?
                };
                nat_add(a77, a78)?
            };
            let v18: u64 = {
                let a83 = v14;
                let a84 = {
                    let a81 = {
                        let a79 = v10;
                        let a80 = 1u64;
                        nat_sub(a79, a80)
                    };
                    let a82 = v16;
                    nat_mul(a81, a82)?
                };
                nat_add(a83, a84)?
            };
            let v19: u64 = {
                let a87 = {
                    let a85 = v9;
                    let a86 = v10;
                    nat_mul(a85, a86)?
                };
                let a88 = v12;
                nat_mul(a87, a88)?
            };
            let v20: u64 = {
                let a91 = {
                    let a89 = v13;
                    let a90 = v14;
                    nat_mul(a89, a90)?
                };
                let a92 = v15;
                nat_mul(a91, a92)?
            };
            let v21: u64 = {
                let a93 = v19;
                let a94 = 32u64;
                if nat_lt(a93, a94) {
                    v19
                } else {
                    32u64
                }
            };
            let a96 = match v2.clone() {
                Adt2::C0(h95, _, _, _, _, _, _, _, _, _, _) => {
                    h95
                }
            };
            let a97 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a96, a97) {
                true
            } else {
                let a99 = match v2.clone() {
                    Adt2::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                        h98
                    }
                };
                let a100 = 1u64;
                nat_eq(a99, a100)
            } {
                let a102 = match v5.clone() {
                    Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                        h101
                    }
                };
                let a103 = 0u64;
                nat_eq(a102, a103)
            } else {
                false
            } {
                let a105 = match v4.clone() {
                    Adt2::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                };
                let a106 = 0u64;
                nat_eq(a105, a106)
            } else {
                false
            } {
                let a107 = v16;
                let a108 = 2147483648u64;
                nat_lt(a107, a108)
            } else {
                false
            } {
                let a109 = 0u64;
                let a110 = v9;
                nat_lt(a109, a110)
            } else {
                false
            } {
                let a111 = 0u64;
                let a112 = v10;
                nat_lt(a111, a112)
            } else {
                false
            } {
                let a113 = 0u64;
                let a114 = v11;
                nat_lt(a113, a114)
            } else {
                false
            } {
                let a115 = 0u64;
                let a116 = v12;
                nat_lt(a115, a116)
            } else {
                false
            } {
                let a117 = 0u64;
                let a118 = v13;
                nat_lt(a117, a118)
            } else {
                false
            } {
                let a119 = 0u64;
                let a120 = v14;
                nat_lt(a119, a120)
            } else {
                false
            } {
                let a121 = 0u64;
                let a122 = v15;
                nat_lt(a121, a122)
            } else {
                false
            } {
                let a123 = 0u64;
                let a124 = v16;
                nat_lt(a123, a124)
            } else {
                false
            } {
                let a126 = match v6.clone() {
                    Adt3::C0(_, _, _, h125, _, _, _, _) => {
                        h125
                    }
                };
                let a127 = v11;
                nat_eq(a126, a127)
            } else {
                false
            } {
                let a129 = match v8.clone() {
                    Adt3::C0(h128, _, _, _, _, _, _, _) => {
                        h128
                    }
                };
                let a130 = v17;
                nat_eq(a129, a130)
            } else {
                false
            } {
                let a132 = match v8.clone() {
                    Adt3::C0(_, h131, _, _, _, _, _, _) => {
                        h131
                    }
                };
                let a133 = v18;
                nat_eq(a132, a133)
            } else {
                false
            } {
                let a135 = match v8.clone() {
                    Adt3::C0(_, _, h134, _, _, _, _, _) => {
                        h134
                    }
                };
                let a136 = v15;
                nat_eq(a135, a136)
            } else {
                false
            } {
                let a138 = match v8.clone() {
                    Adt3::C0(_, _, _, h137, _, _, _, _) => {
                        h137
                    }
                };
                let a139 = v12;
                nat_eq(a138, a139)
            } else {
                false
            } {
                f22(v4.clone())?
            } else {
                false
            } {
                let a144 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
                        h140
                    }
                };
                let a145 = {
                    let a142 = match v2.clone() {
                        Adt2::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                            h141
                        }
                    };
                    let a143 = 1u64;
                    if nat_eq(a142, a143) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a144, a145)
            } else {
                false
            } {
                let a147 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h146, _, _, _, _, _) => {
                        h146
                    }
                };
                let a148 = 4u64;
                nat_eq(a147, a148)
            } else {
                false
            } {
                f23(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f23(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a153 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h149, _, _, _, _) => {
                            h149
                        }
                    };
                    let a154 = {
                        let a151 = v13;
                        let a152 = match v2 {
                            Adt2::C0(_, _, _, _, _, h150, _, _, _, _, _) => {
                                h150
                            }
                        };
                        nat_mul(a151, a152)?
                    };
                    nat_eq(a153, a154)
                } else {
                    let a156 = match v1.clone() {
                        Adt1::C0(_, h155, _, _, _, _, _, _, _, _, _, _) => {
                            h155
                        }
                    };
                    let a157 = 0u64;
                    if if if if if if nat_eq(a156, a157) {
                        let a159 = match v1 {
                            Adt1::C0(_, _, h158, _, _, _, _, _, _, _, _, _) => {
                                h158
                            }
                        };
                        let a160 = 1u64;
                        nat_eq(a159, a160)
                    } else {
                        false
                    } {
                        let a162 = match v2 {
                            Adt2::C0(_, _, _, _, h161, _, _, _, _, _, _) => {
                                h161
                            }
                        };
                        let a163 = 1u64;
                        nat_eq(a162, a163)
                    } else {
                        false
                    } {
                        let a165 = match v5.clone() {
                            Adt2::C0(_, _, _, h164, _, _, _, _, _, _, _) => {
                                h164
                            }
                        };
                        let a166 = 1u64;
                        nat_eq(a165, a166)
                    } else {
                        false
                    } {
                        let a168 = match v5 {
                            Adt2::C0(_, _, _, _, h167, _, _, _, _, _, _) => {
                                h167
                            }
                        };
                        let a169 = 1u64;
                        nat_eq(a168, a169)
                    } else {
                        false
                    } {
                        let a171 = match v4.clone() {
                            Adt2::C0(_, _, _, h170, _, _, _, _, _, _, _) => {
                                h170
                            }
                        };
                        let a172 = 1u64;
                        nat_eq(a171, a172)
                    } else {
                        false
                    } {
                        let a174 = match v4 {
                            Adt2::C0(_, _, _, _, h173, _, _, _, _, _, _) => {
                                h173
                            }
                        };
                        let a175 = 1u64;
                        nat_eq(a174, a175)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a176 = v11;
                    let a177 = v20;
                    nat_mul(a176, a177)?
                }, v21, {
                    let a178 = v21;
                    let a179 = v11;
                    nat_mul(a178, a179)?
                }, {
                    let a180 = v21;
                    let a181 = v20;
                    nat_mul(a180, a181)?
                }, {
                    let a186 = {
                        let a182 = v17;
                        let a183 = v18;
                        nat_mul(a182, a183)?
                    };
                    let a187 = {
                        let a184 = v15;
                        let a185 = v12;
                        nat_mul(a184, a185)?
                    };
                    nat_mul(a186, a187)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f3(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m188 = v3;
    match m188 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h189, _, _, _, _, _, _, _, _, _) => {
                    h189
                }
            }, match v1.clone() {
                Adt1::C0(h190, _, _, _, _, _, _, _, _, _, _, _) => {
                    h190
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h191, _, _, _, _, _, _, _, _) => {
                        h191
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h192, _, _, _, _, _, _, _, _, _) => {
                        h192
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h193, _, _, _, _, _) => {
                        h193
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h194, _, _, _, _, _, _, _) => {
                        h194
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h195, _, _, _, _, _, _, _, _) => {
                    h195
                }
            }, match v1.clone() {
                Adt1::C0(_, h196, _, _, _, _, _, _, _, _, _, _) => {
                    h196
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h197, _, _, _, _, _, _, _) => {
                        h197
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h198, _, _, _, _, _, _, _, _) => {
                        h198
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h199, _, _, _, _) => {
                        h199
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h200, _, _, _, _, _, _) => {
                        h200
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h201, _, _, _, _, _, _, _) => {
                        h201
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h202, _, _, _, _, _, _, _, _, _) => {
                        h202
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h203, _, _, _, _, _, _) => {
                        h203
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h204, _, _, _) => {
                        h204
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f24(v6.clone(), match v5.clone() {
                Adt2::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
                    h205
                }
            })?;
            let v10: u64 = f24(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h206, _, _, _, _, _, _, _, _) => {
                    h206
                }
            })?;
            let v11: u64 = if v0 {
                f24(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h207, _, _, _, _, _, _, _) => {
                        h207
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h208, _, _) => {
                        h208
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h209, _, _, _, _, _, _, _) => {
                        h209
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h210, _) => {
                        h210
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h211, _, _, _, _, _, _) => {
                        h211
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h212) => {
                        h212
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                        h213
                    }
                }
            };
            let v15: u64 = {
                let a218 = {
                    let a214 = v9;
                    let a215 = v10;
                    nat_mul(a214, a215)?
                };
                let a219 = {
                    let a216 = v11;
                    let a217 = v13;
                    nat_mul(a216, a217)?
                };
                nat_mul(a218, a219)?
            };
            let v16: u64 = {
                let a227 = {
                    let a222 = match v6.clone() {
                        Adt6::C0(h220, _, _, _) => {
                            h220
                        }
                    };
                    let a223 = match v7.clone() {
                        Adt6::C0(h221, _, _, _) => {
                            h221
                        }
                    };
                    nat_mul(a222, a223)?
                };
                let a228 = {
                    let a225 = match v8.clone() {
                        Adt6::C0(h224, _, _, _) => {
                            h224
                        }
                    };
                    let a226 = v12;
                    nat_mul(a225, a226)?
                };
                nat_mul(a227, a228)?
            };
            let v17: u64 = {
                let a229 = v15;
                let a230 = 32u64;
                if nat_lt(a229, a230) {
                    v15
                } else {
                    32u64
                }
            };
            let a232 = match v2.clone() {
                Adt2::C0(h231, _, _, _, _, _, _, _, _, _, _) => {
                    h231
                }
            };
            let a233 = 0u64;
            if if if if if if if if if if if nat_eq(a232, a233) {
                true
            } else {
                let a235 = match v2.clone() {
                    Adt2::C0(h234, _, _, _, _, _, _, _, _, _, _) => {
                        h234
                    }
                };
                let a236 = 1u64;
                nat_eq(a235, a236)
            } {
                let a238 = match v5.clone() {
                    Adt2::C0(h237, _, _, _, _, _, _, _, _, _, _) => {
                        h237
                    }
                };
                let a239 = 0u64;
                nat_eq(a238, a239)
            } else {
                false
            } {
                let a241 = match v4.clone() {
                    Adt2::C0(h240, _, _, _, _, _, _, _, _, _, _) => {
                        h240
                    }
                };
                let a242 = 0u64;
                nat_eq(a241, a242)
            } else {
                false
            } {
                let a244 = match v4.clone() {
                    Adt2::C0(_, h243, _, _, _, _, _, _, _, _, _) => {
                        h243
                    }
                };
                let a245 = v9;
                nat_eq(a244, a245)
            } else {
                false
            } {
                let a247 = match v4.clone() {
                    Adt2::C0(_, _, h246, _, _, _, _, _, _, _, _) => {
                        h246
                    }
                };
                let a248 = v10;
                nat_eq(a247, a248)
            } else {
                false
            } {
                if v0 {
                    let a249 = 0u64;
                    let a250 = v12;
                    if if if if if if if nat_lt(a249, a250) {
                        let a251 = v12;
                        let a252 = 2147483648u64;
                        nat_lt(a251, a252)
                    } else {
                        false
                    } {
                        let a253 = v14;
                        let a254 = 2147483648u64;
                        nat_lt(a253, a254)
                    } else {
                        false
                    } {
                        let a255 = v13;
                        let a256 = 2147483648u64;
                        nat_lt(a255, a256)
                    } else {
                        false
                    } {
                        let a260 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h257, _, _, _, _, _, _) => {
                                h257
                            }
                        };
                        let a261 = {
                            let a258 = v12;
                            let a259 = v14;
                            nat_mul(a258, a259)?
                        };
                        nat_eq(a260, a261)
                    } else {
                        false
                    } {
                        let a265 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h262, _, _, _, _, _, _) => {
                                h262
                            }
                        };
                        let a266 = {
                            let a263 = v12;
                            let a264 = v13;
                            nat_mul(a263, a264)?
                        };
                        nat_eq(a265, a266)
                    } else {
                        false
                    } {
                        let a268 = match v4.clone() {
                            Adt2::C0(_, _, _, h267, _, _, _, _, _, _, _) => {
                                h267
                            }
                        };
                        let a269 = v11;
                        nat_eq(a268, a269)
                    } else {
                        false
                    } {
                        let a273 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h270, _, _, _, _, _, _) => {
                                h270
                            }
                        };
                        let a274 = {
                            let a271 = v14;
                            let a272 = v13;
                            nat_mul(a271, a272)?
                        };
                        nat_eq(a273, a274)
                    } else {
                        false
                    }
                } else {
                    let a276 = match v2.clone() {
                        Adt2::C0(_, _, _, h275, _, _, _, _, _, _, _) => {
                            h275
                        }
                    };
                    let a277 = v12;
                    if if if nat_eq(a276, a277) {
                        let a279 = match v4.clone() {
                            Adt2::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                                h278
                            }
                        };
                        let a280 = v14;
                        nat_eq(a279, a280)
                    } else {
                        false
                    } {
                        let a282 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h281, _, _, _, _, _, _) => {
                                h281
                            }
                        };
                        let a283 = v13;
                        nat_eq(a282, a283)
                    } else {
                        false
                    } {
                        let a284 = 0u64;
                        let a285 = v16;
                        nat_lt(a284, a285)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a286 = 0u64;
                let a287 = v14;
                nat_lt(a286, a287)
            } else {
                false
            } {
                let a288 = 0u64;
                let a289 = v15;
                nat_lt(a288, a289)
            } else {
                false
            } {
                f23(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f23(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a290 = v16;
                    let a291 = v14;
                    nat_mul(a290, a291)?
                }, v17, {
                    let a292 = v17;
                    let a293 = v16;
                    nat_mul(a292, a293)?
                }, {
                    let a294 = v17;
                    let a295 = v14;
                    nat_mul(a294, a295)?
                }, 0u64, f20(v2, 1u64), f20(v5, 1u64), f20(v4, 1u64), v6, v7, v8, if v0 {
                    v12
                } else {
                    0u64
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m296 = v2;
    match m296 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m297 = v3;
            match m297 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a299 = match v1.clone() {
                        Adt2::C0(h298, _, _, _, _, _, _, _, _, _, _) => {
                            h298
                        }
                    };
                    let a300 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a299, a300) {
                        let a302 = match v5.clone() {
                            Adt2::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
                                h301
                            }
                        };
                        let a303 = 0u64;
                        nat_eq(a302, a303)
                    } else {
                        false
                    } {
                        let a305 = match v6.clone() {
                            Adt2::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
                                h304
                            }
                        };
                        let a306 = 0u64;
                        nat_eq(a305, a306)
                    } else {
                        false
                    } {
                        let a308 = match v4.clone() {
                            Adt2::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
                                h307
                            }
                        };
                        let a309 = 0u64;
                        nat_eq(a308, a309)
                    } else {
                        false
                    } {
                        f25(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f25(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f22(v1.clone())?
                    } else {
                        false
                    } {
                        f22(v5.clone())?
                    } else {
                        false
                    } {
                        f22(v6)?
                    } else {
                        false
                    } {
                        f22(v4.clone())?
                    } else {
                        false
                    } {
                        let a315 = match v1.clone() {
                            Adt2::C0(_, h310, _, _, _, _, _, _, _, _, _) => {
                                h310
                            }
                        };
                        let a316 = {
                            let a313 = match v5.clone() {
                                Adt2::C0(_, h311, _, _, _, _, _, _, _, _, _) => {
                                    h311
                                }
                            };
                            let a314 = match v5.clone() {
                                Adt2::C0(_, h312, _, _, _, _, _, _, _, _, _) => {
                                    h312
                                }
                            };
                            nat_mul(a313, a314)?
                        };
                        nat_eq(a315, a316)
                    } else {
                        false
                    } {
                        let a322 = match v1.clone() {
                            Adt2::C0(_, _, h317, _, _, _, _, _, _, _, _) => {
                                h317
                            }
                        };
                        let a323 = {
                            let a320 = match v5.clone() {
                                Adt2::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                                    h318
                                }
                            };
                            let a321 = match v5.clone() {
                                Adt2::C0(_, _, _, h319, _, _, _, _, _, _, _) => {
                                    h319
                                }
                            };
                            nat_mul(a320, a321)?
                        };
                        nat_eq(a322, a323)
                    } else {
                        false
                    } {
                        let a326 = match v1.clone() {
                            Adt2::C0(_, _, _, h324, _, _, _, _, _, _, _) => {
                                h324
                            }
                        };
                        let a327 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h325, _, _, _, _, _, _) => {
                                h325
                            }
                        };
                        nat_eq(a326, a327)
                    } else {
                        false
                    } {
                        let a329 = match v0.clone() {
                            Adt1::C0(h328, _, _, _, _, _, _, _, _, _, _, _) => {
                                h328
                            }
                        };
                        let a330 = 4294967296u64;
                        nat_lt(a329, a330)
                    } else {
                        false
                    } {
                        let a332 = match v0.clone() {
                            Adt1::C0(h331, _, _, _, _, _, _, _, _, _, _, _) => {
                                h331
                            }
                        };
                        let a333 = 0u64;
                        if nat_eq(a332, a333) {
                            if f23(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a336 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h334) => {
                                        h334
                                    }
                                };
                                let a337 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h335) => {
                                        h335
                                    }
                                };
                                nat_eq(a336, a337)
                            }
                        } else {
                            let a340 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h338) => {
                                    h338
                                }
                            };
                            let a341 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h339) => {
                                    h339
                                }
                            };
                            nat_eq(a340, a341)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a343 = match v4.clone() {
                                Adt2::C0(_, h342, _, _, _, _, _, _, _, _, _) => {
                                    h342
                                }
                            };
                            let a344 = 0u64;
                            if if if nat_eq(a343, a344) {
                                true
                            } else {
                                let a346 = match v4.clone() {
                                    Adt2::C0(_, _, h345, _, _, _, _, _, _, _, _) => {
                                        h345
                                    }
                                };
                                let a347 = 0u64;
                                nat_eq(a346, a347)
                            } {
                                true
                            } else {
                                let a349 = match v4.clone() {
                                    Adt2::C0(_, _, _, h348, _, _, _, _, _, _, _) => {
                                        h348
                                    }
                                };
                                let a350 = 0u64;
                                nat_eq(a349, a350)
                            } {
                                true
                            } else {
                                let a352 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h351, _, _, _, _, _, _) => {
                                        h351
                                    }
                                };
                                let a353 = 0u64;
                                nat_eq(a352, a353)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h354, _, _, _, _, _, _, _, _, _) => {
                                h354
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a356 = match v0 {
                                Adt1::C0(h355, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h355
                                }
                            };
                            let a357 = 0u64;
                            if nat_eq(a356, a357) {
                                f16(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f26(v8, match v5.clone() {
                                Adt2::C0(_, _, h358, _, _, _, _, _, _, _, _) => {
                                    h358
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h359, _, _, _, _, _, _, _) => {
                                    h359
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h360, _, _, _, _, _, _) => {
                                    h360
                                }
                            })?
                        };
                        Ok(Adt10::C4(f27(v9, v8, v10)?, 4u64, f20(v1, 1u64), f20(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f28(v2.clone());
    let v6: Adt14 = f28(v4.clone());
    let v8: Adt15 = {
        let m361 = v3;
        match m361 {
            None => {
                f29(v0, match v1.clone() {
                    Adt1::C0(h362, _, _, _, _, _, _, _, _, _, _, _) => {
                        h362
                    }
                }, match v1 {
                    Adt1::C0(_, h363, _, _, _, _, _, _, _, _, _, _) => {
                        h363
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f29(v0, match v1.clone() {
                    Adt1::C0(h364, _, _, _, _, _, _, _, _, _, _, _) => {
                        h364
                    }
                }, match v1 {
                    Adt1::C0(_, h365, _, _, _, _, _, _, _, _, _, _) => {
                        h365
                    }
                }, true, v5, f28(v7), v6)?
            }
        }
    };
    let m366 = v8;
    match m366 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f20(v2, 1u64), f20(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f6(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h367, _, _, _, _, _, _, _, _, _, _) => {
            h367
        }
    };
    let v4: u64 = f30(v3);
    let a369 = match v1.clone() {
        Adt2::C0(h368, _, _, _, _, _, _, _, _, _, _) => {
            h368
        }
    };
    let a370 = 0u64;
    if if if if if if if if nat_eq(a369, a370) {
        true
    } else {
        let a372 = match v1.clone() {
            Adt2::C0(h371, _, _, _, _, _, _, _, _, _, _) => {
                h371
            }
        };
        let a373 = 30u64;
        nat_eq(a372, a373)
    } {
        let a376 = match v1.clone() {
            Adt2::C0(h374, _, _, _, _, _, _, _, _, _, _) => {
                h374
            }
        };
        let a377 = match v2.clone() {
            Adt2::C0(h375, _, _, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        nat_eq(a376, a377)
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        f31(match v0.clone() {
            Adt1::C0(h378, _, _, _, _, _, _, _, _, _, _, _) => {
                h378
            }
        })
    } else {
        false
    } {
        f32(v1.clone(), v4)
    } else {
        false
    } {
        f32(v2.clone(), v4)
    } else {
        false
    } {
        if f23(v1.clone(), v2.clone())? {
            true
        } else {
            f33(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f34(match v1.clone() {
            Adt2::C0(_, h379, _, _, _, _, _, _, _, _, _) => {
                h379
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h380, _, _, _, _, _, _, _, _) => {
                h380
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h381, _, _, _, _, _, _, _) => {
                h381
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h382, _, _, _, _, _, _) => {
                h382
            }
        }) {
            0u64
        } else {
            let a388 = match v1.clone() {
                Adt2::C0(_, _, h383, _, _, _, _, _, _, _, _) => {
                    h383
                }
            };
            let a389 = {
                let a386 = match v1.clone() {
                    Adt2::C0(_, _, _, h384, _, _, _, _, _, _, _) => {
                        h384
                    }
                };
                let a387 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h385, _, _, _, _, _, _) => {
                        h385
                    }
                };
                nat_mul(a386, a387)?
            };
            nat_mul(a388, a389)?
        };
        let v6: u64 = {
            let a390 = v5;
            let a391 = 0u64;
            if if nat_eq(a390, a391) {
                true
            } else {
                f35(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a393 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h392, _) => {
                        h392
                    }
                };
                let a394 = v4;
                let a395 = 0u64;
                nat_quot(a393, a394, a395)
            }
        };
        Ok(Adt10::C4(f36(v3, match v1.clone() {
            Adt2::C0(_, h396, _, _, _, _, _, _, _, _, _) => {
                h396
            }
        }, v5, v6)?, v4, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h397, _, _, _, _, _, _, _, _, _, _, _) => {
                h397
            }
        }, v5, f37(match v1 {
            Adt2::C0(_, h398, _, _, _, _, _, _, _, _, _) => {
                h398
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a400 = match v1.clone() {
        Adt2::C0(h399, _, _, _, _, _, _, _, _, _, _) => {
            h399
        }
    };
    let a401 = 0u64;
    if if if if if nat_eq(a400, a401) {
        let a403 = match v2.clone() {
            Adt2::C0(h402, _, _, _, _, _, _, _, _, _, _) => {
                h402
            }
        };
        let a404 = 0u64;
        nat_eq(a403, a404)
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        f38(match v0.clone() {
            Adt1::C0(h405, _, _, _, _, _, _, _, _, _, _, _) => {
                h405
            }
        })
    } else {
        false
    } {
        if f23(v1.clone(), v2.clone())? {
            true
        } else {
            f33(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f20(v1, 1u64);
        let v4: u64 = {
            let a411 = match v3.clone() {
                Adt3::C0(_, h406, _, _, _, _, _, _) => {
                    h406
                }
            };
            let a412 = {
                let a409 = match v3.clone() {
                    Adt3::C0(_, _, h407, _, _, _, _, _) => {
                        h407
                    }
                };
                let a410 = match v3.clone() {
                    Adt3::C0(_, _, _, h408, _, _, _, _) => {
                        h408
                    }
                };
                nat_mul(a409, a410)?
            };
            nat_mul(a411, a412)?
        };
        Ok(Adt10::C4(f39(match v3.clone() {
            Adt3::C0(h413, _, _, _, _, _, _, _) => {
                h413
            }
        }, v4)?, 4u64, v3, f20(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h414, _, _, _, _, _, _, _, _, _, _, _) => {
                h414
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f8(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m415 = v1;
    match m415 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m416 = v2;
            match m416 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a418 = match v0.clone() {
                        Adt2::C0(h417, _, _, _, _, _, _, _, _, _, _) => {
                            h417
                        }
                    };
                    let a419 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a418, a419) {
                        let a421 = match v4.clone() {
                            Adt2::C0(h420, _, _, _, _, _, _, _, _, _, _) => {
                                h420
                            }
                        };
                        let a422 = 0u64;
                        nat_eq(a421, a422)
                    } else {
                        false
                    } {
                        let a424 = match v5.clone() {
                            Adt2::C0(h423, _, _, _, _, _, _, _, _, _, _) => {
                                h423
                            }
                        };
                        let a425 = 26u64;
                        nat_eq(a424, a425)
                    } else {
                        false
                    } {
                        let a427 = match v3.clone() {
                            Adt2::C0(h426, _, _, _, _, _, _, _, _, _, _) => {
                                h426
                            }
                        };
                        let a428 = 0u64;
                        nat_eq(a427, a428)
                    } else {
                        false
                    } {
                        f25(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a431 = match v0.clone() {
                            Adt2::C0(_, h429, _, _, _, _, _, _, _, _, _) => {
                                h429
                            }
                        };
                        let a432 = match v4.clone() {
                            Adt2::C0(_, h430, _, _, _, _, _, _, _, _, _) => {
                                h430
                            }
                        };
                        nat_eq(a431, a432)
                    } else {
                        false
                    } {
                        let a435 = match v0.clone() {
                            Adt2::C0(_, _, h433, _, _, _, _, _, _, _, _) => {
                                h433
                            }
                        };
                        let a436 = match v5.clone() {
                            Adt2::C0(_, h434, _, _, _, _, _, _, _, _, _) => {
                                h434
                            }
                        };
                        nat_eq(a435, a436)
                    } else {
                        false
                    } {
                        let a439 = match v0.clone() {
                            Adt2::C0(_, _, _, h437, _, _, _, _, _, _, _) => {
                                h437
                            }
                        };
                        let a440 = match v5.clone() {
                            Adt2::C0(_, _, h438, _, _, _, _, _, _, _, _) => {
                                h438
                            }
                        };
                        nat_eq(a439, a440)
                    } else {
                        false
                    } {
                        let a442 = 0u64;
                        let a443 = match v5.clone() {
                            Adt2::C0(_, _, _, h441, _, _, _, _, _, _, _) => {
                                h441
                            }
                        };
                        nat_lt(a442, a443)
                    } else {
                        false
                    } {
                        let a445 = 0u64;
                        let a446 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h444, _, _, _, _, _, _) => {
                                h444
                            }
                        };
                        nat_lt(a445, a446)
                    } else {
                        false
                    } {
                        let a448 = 4u64;
                        let a449 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h447, _, _, _, _, _) => {
                                h447
                            }
                        };
                        nat_le(a448, a449)
                    } else {
                        false
                    } {
                        let a451 = 4u64;
                        let a452 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h450, _, _, _, _, _) => {
                                h450
                            }
                        };
                        nat_le(a451, a452)
                    } else {
                        false
                    } {
                        let a454 = 4u64;
                        let a455 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h453, _, _, _, _, _) => {
                                h453
                            }
                        };
                        nat_le(a454, a455)
                    } else {
                        false
                    } {
                        let a457 = 4u64;
                        let a458 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h456, _, _, _, _, _) => {
                                h456
                            }
                        };
                        nat_le(a457, a458)
                    } else {
                        false
                    } {
                        f23(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f23(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f23(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f40(match v0.clone() {
                            Adt2::C0(_, h459, _, _, _, _, _, _, _, _, _) => {
                                h459
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h460, _, _, _, _, _, _, _, _) => {
                                h460
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h461, _, _, _, _, _, _, _) => {
                                h461
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h462, _, _, _, _, _, _) => {
                                h462
                            }
                        })?, 4u64, f20(v0, 1u64), f20(v3, 1u64), Adt4::C22(f20(v4, 1u64), f20(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h463, _, _, _, _, _, _, _, _, _, _, _) => {
            h463
        }
    };
    Ok(Adt9::C0({
        let a464 = v0;
        let a465 = 44u64;
        if nat_eq(a464, a465) {
            f41(v1, v2, v3, v4)?
        } else {
            let a466 = v0;
            let a467 = 52u64;
            if nat_eq(a466, a467) {
                f42(v1, v2, v4)?
            } else {
                let a468 = v0;
                let a469 = 73u64;
                if nat_eq(a468, a469) {
                    f43(v2, v3, v4)?
                } else {
                    let a470 = v0;
                    let a471 = 57u64;
                    if if nat_eq(a470, a471) {
                        true
                    } else {
                        let a472 = v0;
                        let a473 = 58u64;
                        nat_eq(a472, a473)
                    } {
                        let m474 = v3;
                        match m474 {
                            None => {
                                f44({
                                    let a475 = v0;
                                    let a476 = 58u64;
                                    nat_eq(a475, a476)
                                }, v1, v2, v4)?
                            }
                            Some(_) => {
                                Adt10::C1
                            }
                        }
                    } else {
                        let a477 = v0;
                        let a478 = 49u64;
                        if nat_eq(a477, a478) {
                            f45(v1, v2, v3, v4)?
                        } else {
                            let a479 = v0;
                            let a480 = 51u64;
                            if nat_eq(a479, a480) {
                                f46(v1, v2, v3, v4)?
                            } else {
                                let a481 = v0;
                                let a482 = 39u64;
                                if nat_eq(a481, a482) {
                                    f47(v2, v3, v4)?
                                } else {
                                    let a483 = v0;
                                    let a484 = 77u64;
                                    if nat_eq(a483, a484) {
                                        f48(v2, v4)?
                                    } else {
                                        let a485 = v0;
                                        let a486 = 40u64;
                                        if nat_eq(a485, a486) {
                                            f49(v2, v3, v4)?
                                        } else {
                                            if f50(v0) {
                                                let a487 = v0;
                                                let a488 = 33u64;
                                                if if if nat_eq(a487, a488) {
                                                    let a490 = match v4.clone() {
                                                        Adt2::C0(h489, _, _, _, _, _, _, _, _, _, _) => {
                                                            h489
                                                        }
                                                    };
                                                    let a491 = 0u64;
                                                    nat_eq(a490, a491)
                                                } else {
                                                    false
                                                } {
                                                    f51(match v2.clone() {
                                                        Adt2::C0(h492, _, _, _, _, _, _, _, _, _, _) => {
                                                            h492
                                                        }
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    f52(v2, v3, v4)?
                                                } else {
                                                    f53(v0, v2, {
                                                        let m493 = v3;
                                                        match m493 {
                                                            None => {
                                                                false
                                                            }
                                                            Some(_) => {
                                                                true
                                                            }
                                                        }
                                                    }, v4)?
                                                }
                                            } else {
                                                let a494 = v0;
                                                let a495 = 19u64;
                                                if nat_eq(a494, a495) {
                                                    f54(v2, v4)?
                                                } else {
                                                    let a496 = v0;
                                                    let a497 = 21u64;
                                                    if nat_eq(a496, a497) {
                                                        f55(v5, v2, v3, v4)?
                                                    } else {
                                                        let a498 = v0;
                                                        let a499 = 61u64;
                                                        if if if nat_eq(a498, a499) {
                                                            true
                                                        } else {
                                                            let a500 = v0;
                                                            let a501 = 62u64;
                                                            nat_eq(a500, a501)
                                                        } {
                                                            true
                                                        } else {
                                                            let a502 = v0;
                                                            let a503 = 63u64;
                                                            nat_eq(a502, a503)
                                                        } {
                                                            f56(v0, v1, v2, v4)?
                                                        } else {
                                                            let a504 = v0;
                                                            let a505 = 4u64;
                                                            if if nat_eq(a504, a505) {
                                                                true
                                                            } else {
                                                                let a506 = v0;
                                                                let a507 = 32u64;
                                                                nat_eq(a506, a507)
                                                            } {
                                                                f57(v0, v1, v2, v3, v4)?
                                                            } else {
                                                                let a508 = v0;
                                                                let a509 = 20u64;
                                                                if nat_eq(a508, a509) {
                                                                    f58(v2, v4)?
                                                                } else {
                                                                    let a510 = v0;
                                                                    let a511 = 42u64;
                                                                    if nat_eq(a510, a511) {
                                                                        f59(4u64, v2, v4)?
                                                                    } else {
                                                                        let a512 = v0;
                                                                        let a513 = 69u64;
                                                                        if nat_eq(a512, a513) {
                                                                            let a514 = v5;
                                                                            let a515 = 4u64;
                                                                            if nat_lt(a514, a515) {
                                                                                f59(v5, v2, v4)?
                                                                            } else {
                                                                                Adt10::C1
                                                                            }
                                                                        } else {
                                                                            let a516 = v0;
                                                                            let a517 = 70u64;
                                                                            if nat_eq(a516, a517) {
                                                                                f60(v5, v2, v4)?
                                                                            } else {
                                                                                let a518 = v0;
                                                                                let a519 = 75u64;
                                                                                if if nat_eq(a518, a519) {
                                                                                    true
                                                                                } else {
                                                                                    let a520 = v0;
                                                                                    let a521 = 76u64;
                                                                                    nat_eq(a520, a521)
                                                                                } {
                                                                                    f61({
                                                                                        let a522 = v0;
                                                                                        let a523 = 76u64;
                                                                                        nat_eq(a522, a523)
                                                                                    }, v1, v2, v4)?
                                                                                } else {
                                                                                    let a524 = v0;
                                                                                    let a525 = 31u64;
                                                                                    if if if nat_eq(a524, a525) {
                                                                                        true
                                                                                    } else {
                                                                                        let a526 = v0;
                                                                                        let a527 = 47u64;
                                                                                        nat_eq(a526, a527)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a528 = v0;
                                                                                        let a529 = 68u64;
                                                                                        nat_eq(a528, a529)
                                                                                    } {
                                                                                        f62(v0, v1, v2, v4)?
                                                                                    } else {
                                                                                        let a530 = v0;
                                                                                        let a531 = 15u64;
                                                                                        if nat_eq(a530, a531) {
                                                                                            f63(v2, v4)?
                                                                                        } else {
                                                                                            let a532 = v0;
                                                                                            let a533 = 17u64;
                                                                                            if nat_eq(a532, a533) {
                                                                                                f64(v2, v4)?
                                                                                            } else {
                                                                                                let a534 = v0;
                                                                                                let a535 = 18u64;
                                                                                                if nat_eq(a534, a535) {
                                                                                                    f65(v2, v3, v4)?
                                                                                                } else {
                                                                                                    let a536 = v0;
                                                                                                    let a537 = 13u64;
                                                                                                    if if if nat_eq(a536, a537) {
                                                                                                        true
                                                                                                    } else {
                                                                                                        let a538 = v0;
                                                                                                        let a539 = 14u64;
                                                                                                        nat_eq(a538, a539)
                                                                                                    } {
                                                                                                        true
                                                                                                    } else {
                                                                                                        let a540 = v0;
                                                                                                        let a541 = 16u64;
                                                                                                        nat_eq(a540, a541)
                                                                                                    } {
                                                                                                        f66(v0, v2, v4)?
                                                                                                    } else {
                                                                                                        let m542 = f67(v0, v5)?;
                                                                                                        match m542 {
                                                                                                            None => {
                                                                                                                f68(v0, v5, v2, v3, v4)?
                                                                                                            }
                                                                                                            Some(v8) => {
                                                                                                                f69(v8, v2, v4)?
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f10(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h543, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h543
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h544, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h544
        }
    };
    let v6: u64 = {
        let a545 = v3;
        let a546 = v5;
        let a547 = 0u64;
        nat_rem(a545, a546, a547)
    };
    let a548 = v1;
    let a549 = 0u64;
    if nat_eq(a548, a549) {
        let a551 = v3;
        let a552 = match v0.clone() {
            Adt8::C0(_, _, _, h550, _, _, _, _, _, _, _, _, _, _, _) => {
                h550
            }
        };
        if nat_lt(a551, a552) {
            Ok(Adt9::C8(Some(f70(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h553, _, _, _, _, _, _) => {
                    h553
                }
            }, {
                let a557 = v6;
                let a558 = {
                    let m556 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h555, _, _, _, _, _, _) => {
                            h555
                        }
                    };
                    match m556 {
                        Adt3::C0(h554, _, _, _, _, _, _, _) => {
                            h554
                        }
                    }
                };
                let a559 = 0u64;
                nat_rem(a557, a558, a559)
            }, {
                let a569 = {
                    let a563 = v6;
                    let a564 = {
                        let m562 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h561, _, _, _, _, _, _) => {
                                h561
                            }
                        };
                        match m562 {
                            Adt3::C0(h560, _, _, _, _, _, _, _) => {
                                h560
                            }
                        }
                    };
                    let a565 = 0u64;
                    nat_quot(a563, a564, a565)
                };
                let a570 = {
                    let m568 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h567, _, _, _, _, _, _) => {
                            h567
                        }
                    };
                    match m568 {
                        Adt3::C0(_, h566, _, _, _, _, _, _) => {
                            h566
                        }
                    }
                };
                let a571 = 0u64;
                nat_rem(a569, a570, a571)
            }, {
                let a581 = {
                    let a575 = v6;
                    let a576 = {
                        let m574 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h573, _, _, _, _, _, _) => {
                                h573
                            }
                        };
                        match m574 {
                            Adt3::C0(h572, _, _, _, _, _, _, _) => {
                                h572
                            }
                        }
                    };
                    let a577 = 0u64;
                    nat_quot(a575, a576, a577)
                };
                let a582 = {
                    let m580 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h579, _, _, _, _, _, _) => {
                            h579
                        }
                    };
                    match m580 {
                        Adt3::C0(_, h578, _, _, _, _, _, _) => {
                            h578
                        }
                    }
                };
                let a583 = 0u64;
                nat_quot(a581, a582, a583)
            }, {
                let a584 = v3;
                let a585 = v5;
                let a586 = 0u64;
                nat_quot(a584, a585, a586)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a592 = v2;
            let a593 = {
                let a589 = v3;
                let a590 = {
                    let a587 = v1;
                    let a588 = 1u64;
                    if nat_eq(a587, a588) {
                        v4
                    } else {
                        v5
                    }
                };
                let a591 = 0u64;
                nat_quot(a589, a590, a591)
            };
            nat_add(a592, a593)?
        };
        let a595 = v7;
        let a596 = match v0.clone() {
            Adt8::C0(h594, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h594
            }
        };
        if nat_lt(a595, a596) {
            let v8: u64 = {
                let a600 = v7;
                let a601 = {
                    let m599 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h598, _, _, _, _, _) => {
                            h598
                        }
                    };
                    match m599 {
                        Adt3::C0(h597, _, _, _, _, _, _, _) => {
                            h597
                        }
                    }
                };
                let a602 = 0u64;
                nat_rem(a600, a601, a602)
            };
            let v9: u64 = {
                let a606 = v7;
                let a607 = {
                    let m605 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h604, _, _, _, _, _) => {
                            h604
                        }
                    };
                    match m605 {
                        Adt3::C0(h603, _, _, _, _, _, _, _) => {
                            h603
                        }
                    }
                };
                let a608 = 0u64;
                nat_quot(a606, a607, a608)
            };
            let v10: u64 = {
                let a612 = v9;
                let a613 = {
                    let m611 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h610, _, _, _, _, _) => {
                            h610
                        }
                    };
                    match m611 {
                        Adt3::C0(_, h609, _, _, _, _, _, _) => {
                            h609
                        }
                    }
                };
                let a614 = 0u64;
                nat_rem(a612, a613, a614)
            };
            let v11: u64 = {
                let a618 = v9;
                let a619 = {
                    let m617 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h616, _, _, _, _, _) => {
                            h616
                        }
                    };
                    match m617 {
                        Adt3::C0(_, h615, _, _, _, _, _, _) => {
                            h615
                        }
                    }
                };
                let a620 = 0u64;
                nat_quot(a618, a619, a620)
            };
            let a621 = v1;
            let a622 = 1u64;
            if nat_eq(a621, a622) {
                Ok(Adt9::C8(Some(f70(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h623, _, _, _, _, _) => {
                        h623
                    }
                }, v8, v10, {
                    let a624 = v3;
                    let a625 = v4;
                    let a626 = 0u64;
                    nat_rem(a624, a625, a626)
                }, v11)?)))
            } else {
                let a627 = v1;
                let a628 = 2u64;
                if nat_eq(a627, a628) {
                    Ok(Adt9::C8(Some({
                        let a674 = f70(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h629, _, _, _, _) => {
                                h629
                            }
                        }, {
                            let a641 = {
                                let a633 = v8;
                                let a634 = {
                                    let m632 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h631, _, _, _) => {
                                            h631
                                        }
                                    };
                                    match m632 {
                                        Adt6::C0(_, h630, _, _) => {
                                            h630
                                        }
                                    }
                                };
                                nat_mul(a633, a634)?
                            };
                            let a642 = {
                                let a638 = v6;
                                let a639 = {
                                    let m637 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h636, _, _, _, _, _, _) => {
                                            h636
                                        }
                                    };
                                    match m637 {
                                        Adt3::C0(h635, _, _, _, _, _, _, _) => {
                                            h635
                                        }
                                    }
                                };
                                let a640 = 0u64;
                                nat_rem(a638, a639, a640)
                            };
                            nat_add(a641, a642)?
                        }, {
                            let a660 = {
                                let a646 = v10;
                                let a647 = {
                                    let m645 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h644, _, _, _) => {
                                            h644
                                        }
                                    };
                                    match m645 {
                                        Adt6::C0(_, h643, _, _) => {
                                            h643
                                        }
                                    }
                                };
                                nat_mul(a646, a647)?
                            };
                            let a661 = {
                                let a657 = {
                                    let a651 = v6;
                                    let a652 = {
                                        let m650 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h649, _, _, _, _, _, _) => {
                                                h649
                                            }
                                        };
                                        match m650 {
                                            Adt3::C0(h648, _, _, _, _, _, _, _) => {
                                                h648
                                            }
                                        }
                                    };
                                    let a653 = 0u64;
                                    nat_quot(a651, a652, a653)
                                };
                                let a658 = {
                                    let m656 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h655, _, _, _, _, _, _) => {
                                            h655
                                        }
                                    };
                                    match m656 {
                                        Adt3::C0(_, h654, _, _, _, _, _, _) => {
                                            h654
                                        }
                                    }
                                };
                                let a659 = 0u64;
                                nat_rem(a657, a658, a659)
                            };
                            nat_add(a660, a661)?
                        }, {
                            let a671 = {
                                let a665 = v6;
                                let a666 = {
                                    let m664 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h663, _, _, _, _, _, _) => {
                                            h663
                                        }
                                    };
                                    match m664 {
                                        Adt3::C0(h662, _, _, _, _, _, _, _) => {
                                            h662
                                        }
                                    }
                                };
                                let a667 = 0u64;
                                nat_quot(a665, a666, a667)
                            };
                            let a672 = {
                                let m670 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h669, _, _, _, _, _, _) => {
                                        h669
                                    }
                                };
                                match m670 {
                                    Adt3::C0(_, h668, _, _, _, _, _, _) => {
                                        h668
                                    }
                                }
                            };
                            let a673 = 0u64;
                            nat_quot(a671, a672, a673)
                        }, v11)?;
                        let a675 = 4u64;
                        let a676 = 0u64;
                        nat_quot(a674, a675, a676)
                    })))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f11(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a677 = v1;
    let a678 = 0u64;
    if nat_eq(a677, a678) {
        let a680 = v3;
        let a681 = match v0.clone() {
            Adt8::C0(_, _, _, h679, _, _, _, _, _, _, _, _, _, _, _) => {
                h679
            }
        };
        if nat_lt(a680, a681) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h682, _, _, _, _, _, _) => {
                    h682
                }
            };
            let v5: u64 = {
                let a684 = v3;
                let a685 = match v0.clone() {
                    Adt8::C0(_, _, h683, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h683
                    }
                };
                let a686 = 0u64;
                nat_quot(a684, a685, a686)
            };
            Ok(Adt9::C8(Some(f70(v4.clone(), f71(v5, v4.clone(), 0u64), f71(v5, v4.clone(), 1u64), f71(v5, v4, 2u64), f72(v0.clone(), v5, {
                let a688 = v3;
                let a689 = match v0 {
                    Adt8::C0(_, _, h687, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h687
                    }
                };
                let a690 = 0u64;
                nat_rem(a688, a689, a690)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h691, _, _, _, _) => {
                h691
            }
        };
        let v7: u64 = {
            let a692 = v1;
            let a693 = 1u64;
            if nat_eq(a692, a693) {
                match v0.clone() {
                    Adt8::C0(_, h694, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h694
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h695, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h695
                    }
                }
            }
        };
        let v8: u64 = {
            let a699 = v2;
            let a700 = {
                let a696 = v3;
                let a697 = v7;
                let a698 = 0u64;
                nat_quot(a696, a697, a698)
            };
            nat_add(a699, a700)?
        };
        let v9: u64 = {
            let a701 = v3;
            let a702 = v7;
            let a703 = 0u64;
            nat_rem(a701, a702, a703)
        };
        let v10: u64 = {
            let a705 = v8;
            let a706 = match v6.clone() {
                Adt3::C0(h704, _, _, _, _, _, _, _) => {
                    h704
                }
            };
            let a707 = 0u64;
            nat_rem(a705, a706, a707)
        };
        let v11: u64 = {
            let a709 = v8;
            let a710 = match v6.clone() {
                Adt3::C0(h708, _, _, _, _, _, _, _) => {
                    h708
                }
            };
            let a711 = 0u64;
            nat_quot(a709, a710, a711)
        };
        let v12: u64 = {
            let a713 = v11;
            let a714 = match v6.clone() {
                Adt3::C0(_, h712, _, _, _, _, _, _) => {
                    h712
                }
            };
            let a715 = 0u64;
            nat_rem(a713, a714, a715)
        };
        let v13: u64 = {
            let a717 = v11;
            let a718 = match v6.clone() {
                Adt3::C0(_, h716, _, _, _, _, _, _) => {
                    h716
                }
            };
            let a719 = 0u64;
            nat_quot(a717, a718, a719)
        };
        let a721 = v8;
        let a722 = match v0.clone() {
            Adt8::C0(h720, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h720
            }
        };
        if nat_lt(a721, a722) {
            let a723 = v1;
            let a724 = 1u64;
            if nat_eq(a723, a724) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h725, _, _, _, _, _, _) => {
                        h725
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h726, _, _, _, _, _) => {
                        h726
                    }
                };
                let m729 = f73(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h727, _, _, _) => {
                        h727
                    }
                }, match v15.clone() {
                    Adt3::C0(h728, _, _, _, _, _, _, _) => {
                        h728
                    }
                }, v10, f71(v9, v14.clone(), 0u64))?;
                match m729 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m732 = f73(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h730, _, _) => {
                                h730
                            }
                        }, match v15 {
                            Adt3::C0(_, h731, _, _, _, _, _, _) => {
                                h731
                            }
                        }, v12, f71(v9, v14, 1u64))?;
                        match m732 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f74(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a733 = v1;
                let a734 = 2u64;
                if nat_eq(a733, a734) {
                    Ok(Adt9::C8(Some({
                        let a736 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h735) => {
                                h735
                            }
                        };
                        let a737 = 0u64;
                        if nat_eq(a736, a737) {
                            f70(v6, v10, v12, v9, v13)?
                        } else {
                            f70(v6.clone(), v10, v12, {
                                let a739 = v13;
                                let a740 = match v6.clone() {
                                    Adt3::C0(_, _, h738, _, _, _, _, _) => {
                                        h738
                                    }
                                };
                                let a741 = 0u64;
                                nat_rem(a739, a740, a741)
                            }, {
                                let a749 = {
                                    let a747 = {
                                        let a743 = v13;
                                        let a744 = match v6 {
                                            Adt3::C0(_, _, h742, _, _, _, _, _) => {
                                                h742
                                            }
                                        };
                                        let a745 = 0u64;
                                        nat_quot(a743, a744, a745)
                                    };
                                    let a748 = match v0 {
                                        Adt8::C0(_, _, h746, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h746
                                        }
                                    };
                                    nat_mul(a747, a748)?
                                };
                                let a750 = v9;
                                nat_add(a749, a750)?
                            })?
                        }
                    })))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f12(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f13(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f13(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f75(v3.clone(), {
        let m751 = v3.clone();
        match m751 {
            Adt4::C0 => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f79(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C19(v52, v53) => {
                f80(v52, v0, v1, v2, v53, v4)?
            }
            Adt4::C20(v54, v55) => {
                f81(v54, v0, v1, v2, v55, v4)?
            }
            Adt4::C21 => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C22(v56, v57) => {
                f82(v0, v1, v2, v56, v57, v4)?
            }
            Adt4::C23(v71, v72, v73, v74) => {
                f89(v0, v74, v71, v72, v73, v4)?
            }
            Adt4::C24(v70) => {
                if v5 {
                    f87(Adt16::C0(v70), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f76(v0, v1, v2, v3)?
            }
            Adt4::C26(v66, v67, v68, v69) => {
                let a753 = v66;
                let a754 = 64u64;
                if if if nat_eq(a753, a754) {
                    true
                } else {
                    let a755 = v66;
                    let a756 = 65u64;
                    nat_eq(a755, a756)
                } {
                    true
                } else {
                    let a757 = v66;
                    let a758 = 66u64;
                    nat_eq(a757, a758)
                } {
                    if v5 {
                        f87(Adt16::C1(v66, v67, v68, v69), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a759 = v66;
                    let a760 = 0u64;
                    if if nat_eq(a759, a760) {
                        true
                    } else {
                        let a761 = v66;
                        let a762 = 30u64;
                        nat_eq(a761, a762)
                    } {
                        f88(v0, v1, v2, v66, v67, v68, v69, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v65) => {
                f86(v0, v1, v2, v65)?
            }
            Adt4::C28(v61) => {
                f83(v0, v1, v2, v61, v4)?
            }
            Adt4::C29(v62) => {
                f84(v0, v1, v2, v62)?
            }
            Adt4::C30(v63, _) => {
                f85(v0, v1, v2, v63)?
            }
            Adt4::C31(v33, v34) => {
                f77(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f78(v0, match v1 {
                    Adt3::C0(h752, _, _, _, _, _, _, _) => {
                        h752
                    }
                }, v35)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m763 = v0;
    match m763 {
        Adt4::C0 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C1 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C2(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C3(_, _, _, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C4(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C5(_, _, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C6(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C7(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C8(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C9(v18, v19) => {
            Ok(Adt9::C6(0u64, v18, v19, true))
        }
        Adt4::C10(_, v21, _) => {
            Ok(Adt9::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false))
        }
        Adt4::C11(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C12(_, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C13(_, _, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C14(_, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C15 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C16(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C17 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C18(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C21 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C22(_, _) => {
            Ok(Adt9::C6(7u64, 0u64, 0u64, false))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt9::C6(0u64, 0u64, 0u64, false))
        }
        Adt4::C24(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C25(v48, _, v50) => {
            Ok(Adt9::C6({
                let a768 = {
                    let a764 = 0u64;
                    let a765 = v48;
                    if nat_lt(a764, a765) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a769 = {
                    let a766 = 0u64;
                    let a767 = v50;
                    if nat_lt(a766, a767) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a768, a769)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v56, _, _, _) => {
            Ok(Adt9::C6({
                let a770 = v56;
                let a771 = 66u64;
                if nat_eq(a770, a771) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false))
        }
        Adt4::C27(_) => {
            Ok(Adt9::C6(2u64, 0u64, 0u64, false))
        }
        Adt4::C28(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C29(_) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C30(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C32(_) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
    }
}

pub fn f15(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a772 = v1;
    let a773 = 5u64;
    if if if if if nat_lt(a772, a773) {
        true
    } else {
        f90(v0, v1, v2, v3, v4)
    } {
        let a774 = v5;
        let a775 = 5u64;
        nat_lt(a774, a775)
    } else {
        false
    } {
        let a776 = v7;
        let a777 = 5u64;
        nat_lt(a776, a777)
    } else {
        false
    } {
        let a778 = v9;
        let a779 = 2u64;
        if nat_lt(a778, a779) {
            true
        } else {
            let a780 = v9;
            let a781 = 3u64;
            nat_eq(a780, a781)
        }
    } else {
        false
    } {
        Adt9::C7(f91(v0, v1, v2, v3, v4), f92(v5, v6), f92(v7, v8), f93({
            let a782 = v9;
            let a783 = 0u64;
            if nat_eq(a782, a783) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> R<u64> {
    let a795 = {
        let a789 = {
            let a785 = match v0.clone() {
                Adt2::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                    h784
                }
            };
            let a786 = v1;
            let a787 = 0u64;
            nat_quot(a785, a786, a787)
        };
        let a790 = match v0.clone() {
            Adt2::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                h788
            }
        };
        nat_mul(a789, a790)?
    };
    let a796 = {
        let a793 = match v0.clone() {
            Adt2::C0(_, _, _, h791, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a794 = match v0 {
            Adt2::C0(_, _, _, _, h792, _, _, _, _, _, _) => {
                h792
            }
        };
        nat_mul(a793, a794)?
    };
    nat_mul(a795, a796)
}

pub fn f17(v0: u64) -> bool {
    let a797 = v0;
    let a798 = 4294967296u64;
    if nat_lt(a797, a798) {
        let a802 = {
            let a799 = v0;
            let a800 = 2147483648u64;
            let a801 = 0u64;
            nat_rem(a799, a800, a801)
        };
        let a803 = 2139095040u64;
        nat_lt(a802, a803)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a804 = 4u64;
    let a805 = v0;
    nat_mul(a804, a805)
}

pub fn f19(v0: u64) -> R<u64> {
    let a806 = v0;
    let a807 = 0u64;
    if nat_eq(a806, a807) {
        Ok(0u64)
    } else {
        let a810 = 4u64;
        let a811 = {
            let a808 = 3u64;
            let a809 = v0;
            nat_mul(a808, a809)?
        };
        nat_add(a810, a811)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a813 = match v0.clone() {
            Adt2::C0(_, h812, _, _, _, _, _, _, _, _, _) => {
                h812
            }
        };
        let a814 = v1;
        let a815 = 0u64;
        nat_quot(a813, a814, a815)
    }, match v0.clone() {
        Adt2::C0(_, _, h816, _, _, _, _, _, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h817, _, _, _, _, _, _, _) => {
            h817
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h818, _, _, _, _, _, _) => {
            h818
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h819, _, _, _, _, _) => {
            h819
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h820, _, _, _, _) => {
            h820
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h821, _, _, _) => {
            h821
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h822, _, _) => {
            h822
        }
    })
}

pub fn f21(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f20(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h823, _, _, _, _, _, _, _, _, _) => {
                h823
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h824, _, _, _, _, _, _, _, _) => {
                h824
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h825, _, _, _, _, _, _, _) => {
                h825
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h826, _, _, _, _, _) => {
                h826
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h827, _, _, _, _) => {
                h827
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h828, _, _, _) => {
                h828
            }
        })
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a830 = match v0.clone() {
        Adt2::C0(_, h829, _, _, _, _, _, _, _, _, _) => {
            h829
        }
    };
    let a831 = 1u64;
    if if if if nat_eq(a830, a831) {
        true
    } else {
        let a833 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h832, _, _, _, _, _) => {
                h832
            }
        };
        let a834 = 4u64;
        nat_eq(a833, a834)
    } {
        let a836 = match v0.clone() {
            Adt2::C0(_, _, h835, _, _, _, _, _, _, _, _) => {
                h835
            }
        };
        let a837 = 1u64;
        if nat_eq(a836, a837) {
            true
        } else {
            let a842 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h838, _, _, _, _) => {
                    h838
                }
            };
            let a843 = {
                let a840 = match v0.clone() {
                    Adt2::C0(_, h839, _, _, _, _, _, _, _, _, _) => {
                        h839
                    }
                };
                let a841 = 4u64;
                nat_mul(a840, a841)?
            };
            nat_eq(a842, a843)
        }
    } else {
        false
    } {
        let a845 = match v0.clone() {
            Adt2::C0(_, _, _, h844, _, _, _, _, _, _, _) => {
                h844
            }
        };
        let a846 = 1u64;
        if nat_eq(a845, a846) {
            true
        } else {
            let a854 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h847, _, _, _) => {
                    h847
                }
            };
            let a855 = {
                let a852 = {
                    let a850 = match v0.clone() {
                        Adt2::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                            h848
                        }
                    };
                    let a851 = match v0.clone() {
                        Adt2::C0(_, _, h849, _, _, _, _, _, _, _, _) => {
                            h849
                        }
                    };
                    nat_mul(a850, a851)?
                };
                let a853 = 4u64;
                nat_mul(a852, a853)?
            };
            nat_eq(a854, a855)
        }
    } else {
        false
    } {
        let a857 = match v0.clone() {
            Adt2::C0(_, _, _, _, h856, _, _, _, _, _, _) => {
                h856
            }
        };
        let a858 = 1u64;
        if nat_eq(a857, a858) {
            Ok(true)
        } else {
            let a869 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h859, _, _) => {
                    h859
                }
            };
            let a870 = {
                let a867 = {
                    let a865 = {
                        let a862 = match v0.clone() {
                            Adt2::C0(_, h860, _, _, _, _, _, _, _, _, _) => {
                                h860
                            }
                        };
                        let a863 = match v0.clone() {
                            Adt2::C0(_, _, h861, _, _, _, _, _, _, _, _) => {
                                h861
                            }
                        };
                        nat_mul(a862, a863)?
                    };
                    let a866 = match v0 {
                        Adt2::C0(_, _, _, h864, _, _, _, _, _, _, _) => {
                            h864
                        }
                    };
                    nat_mul(a865, a866)?
                };
                let a868 = 4u64;
                nat_mul(a867, a868)?
            };
            Ok(nat_eq(a869, a870))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<bool> {
    let a872 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h871, _) => {
            h871
        }
    };
    let a873 = 0u64;
    if nat_eq(a872, a873) {
        Ok(true)
    } else {
        let a875 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h874, _) => {
                h874
            }
        };
        let a876 = 0u64;
        if nat_eq(a875, a876) {
            Ok(true)
        } else {
            let a878 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h877) => {
                    h877
                }
            };
            let a879 = 0u64;
            if nat_eq(a878, a879) {
                Ok(true)
            } else {
                let a881 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h880) => {
                        h880
                    }
                };
                let a882 = 0u64;
                if nat_eq(a881, a882) {
                    Ok(true)
                } else {
                    let a888 = {
                        let a885 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h883) => {
                                h883
                            }
                        };
                        let a886 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h884, _) => {
                                h884
                            }
                        };
                        nat_add(a885, a886)?
                    };
                    let a889 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h887) => {
                            h887
                        }
                    };
                    if nat_le(a888, a889) {
                        Ok(true)
                    } else {
                        let a895 = {
                            let a892 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h890) => {
                                    h890
                                }
                            };
                            let a893 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h891, _) => {
                                    h891
                                }
                            };
                            nat_add(a892, a893)?
                        };
                        let a896 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h894) => {
                                h894
                            }
                        };
                        Ok(nat_le(a895, a896))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt6, v1: u64) -> R<u64> {
    let a898 = match v0.clone() {
        Adt6::C0(h897, _, _, _) => {
            h897
        }
    };
    let a899 = 0u64;
    if nat_eq(a898, a899) {
        f94(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h900, _, _) => {
                h900
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h901, _) => {
                h901
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h902) => {
                h902
            }
        }), {
            let a904 = v1;
            let a905 = match v0 {
                Adt6::C0(_, _, _, h903) => {
                    h903
                }
            };
            nat_add(a904, a905)?
        })
    } else {
        f94(v0, v1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    let a908 = match v0.clone() {
        Adt2::C0(_, h906, _, _, _, _, _, _, _, _, _) => {
            h906
        }
    };
    let a909 = match v1.clone() {
        Adt2::C0(_, h907, _, _, _, _, _, _, _, _, _) => {
            h907
        }
    };
    if if if nat_eq(a908, a909) {
        let a912 = match v0.clone() {
            Adt2::C0(_, _, h910, _, _, _, _, _, _, _, _) => {
                h910
            }
        };
        let a913 = match v1.clone() {
            Adt2::C0(_, _, h911, _, _, _, _, _, _, _, _) => {
                h911
            }
        };
        nat_eq(a912, a913)
    } else {
        false
    } {
        let a916 = match v0.clone() {
            Adt2::C0(_, _, _, h914, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a917 = match v1.clone() {
            Adt2::C0(_, _, _, h915, _, _, _, _, _, _, _) => {
                h915
            }
        };
        nat_eq(a916, a917)
    } else {
        false
    } {
        let a920 = match v0 {
            Adt2::C0(_, _, _, _, h918, _, _, _, _, _, _) => {
                h918
            }
        };
        let a921 = match v1 {
            Adt2::C0(_, _, _, _, h919, _, _, _, _, _, _) => {
                h919
            }
        };
        nat_eq(a920, a921)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a926 = {
        let a922 = v0;
        let a923 = v1;
        nat_mul(a922, a923)?
    };
    let a927 = {
        let a924 = v2;
        let a925 = f95(v3);
        nat_mul(a924, a925)?
    };
    nat_mul(a926, a927)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a930 = v0;
    let a931 = {
        let a928 = f96(v1)?;
        let a929 = v2;
        nat_mul(a928, a929)?
    };
    nat_add(a930, a931)
}

pub fn f28(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h932, _, _, _, _, _, _, _, _, _, _) => {
            h932
        }
    }, match v0.clone() {
        Adt2::C0(_, h933, _, _, _, _, _, _, _, _, _) => {
            h933
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h934, _, _, _, _, _, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h935, _, _, _, _, _, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h936, _, _, _, _, _, _) => {
            h936
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h937, _, _, _, _, _) => {
            h937
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h938, _, _, _, _) => {
            h938
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h939, _, _, _) => {
            h939
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h940, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h941, _) => {
            h941
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h942) => {
            h942
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a943 = v0;
    let a944 = 23u64;
    if nat_eq(a943, a944) {
        f97(v1, v4, v6)
    } else {
        let a945 = v0;
        let a946 = 26u64;
        if nat_eq(a945, a946) {
            f98(v1, v2, v4, v6)
        } else {
            let a947 = v0;
            let a948 = 25u64;
            if nat_eq(a947, a948) {
                f99(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a949 = v0;
    let a950 = 30u64;
    if nat_eq(a949, a950) {
        2u64
    } else {
        4u64
    }
}

pub fn f31(v0: u64) -> bool {
    let a951 = v0;
    let a952 = 2139095040u64;
    if nat_le(a951, a952) {
        true
    } else {
        let a953 = v0;
        let a954 = 2147483648u64;
        nat_eq(a953, a954)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> bool {
    let a956 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h955, _, _, _, _, _) => {
            h955
        }
    };
    let a957 = v1;
    if if nat_eq(a956, a957) {
        let a962 = {
            let a959 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h958) => {
                    h958
                }
            };
            let a960 = v1;
            let a961 = 0u64;
            nat_rem(a959, a960, a961)
        };
        let a963 = 0u64;
        nat_eq(a962, a963)
    } else {
        false
    } {
        f100(match v0.clone() {
            Adt2::C0(_, _, h964, _, _, _, _, _, _, _, _) => {
                h964
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                h965
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h966, _, _, _, _, _, _) => {
                h966
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h967, _, _, _, _) => {
                h967
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h968, _, _, _) => {
                h968
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h969, _, _) => {
                h969
            }
        }, v1)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a972 = match v0.clone() {
            Adt2::C0(h970, _, _, _, _, _, _, _, _, _, _) => {
                h970
            }
        };
        let a973 = match v1.clone() {
            Adt2::C0(h971, _, _, _, _, _, _, _, _, _, _) => {
                h971
            }
        };
        nat_eq(a972, a973)
    } else {
        false
    } {
        let a976 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h974) => {
                h974
            }
        };
        let a977 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h975) => {
                h975
            }
        };
        nat_eq(a976, a977)
    } else {
        false
    } {
        let a980 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h978, _, _, _, _, _) => {
                h978
            }
        };
        let a981 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h979, _, _, _, _, _) => {
                h979
            }
        };
        nat_eq(a980, a981)
    } else {
        false
    } {
        let a984 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h982, _, _, _, _) => {
                h982
            }
        };
        let a985 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h983, _, _, _, _) => {
                h983
            }
        };
        nat_eq(a984, a985)
    } else {
        false
    } {
        let a988 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h986, _, _, _) => {
                h986
            }
        };
        let a989 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h987, _, _, _) => {
                h987
            }
        };
        nat_eq(a988, a989)
    } else {
        false
    } {
        let a992 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h990, _, _) => {
                h990
            }
        };
        let a993 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h991, _, _) => {
                h991
            }
        };
        nat_eq(a992, a993)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a994 = v0;
    let a995 = 0u64;
    if if if nat_eq(a994, a995) {
        true
    } else {
        let a996 = v1;
        let a997 = 0u64;
        nat_eq(a996, a997)
    } {
        true
    } else {
        let a998 = v2;
        let a999 = 0u64;
        nat_eq(a998, a999)
    } {
        true
    } else {
        let a1000 = v3;
        let a1001 = 0u64;
        nat_eq(a1000, a1001)
    }
}

pub fn f35(v0: Adt2, v1: u64) -> R<bool> {
    f101({
        let a1003 = match v0.clone() {
            Adt2::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1004 = v1;
        nat_mul(a1003, a1004)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1005, _, _, _, _, _, _, _, _) => {
            h1005
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1006, _, _, _, _, _, _, _) => {
            h1006
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1007, _, _, _, _, _, _) => {
            h1007
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1008, _, _, _, _) => {
            h1008
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1009, _, _, _) => {
            h1009
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1010, _, _) => {
            h1010
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1011 = v1;
    let a1012 = 0u64;
    if if nat_eq(a1011, a1012) {
        true
    } else {
        let a1013 = v2;
        let a1014 = 0u64;
        nat_eq(a1013, a1014)
    } {
        Ok(0u64)
    } else {
        let a1021 = {
            let a1015 = f102(v1, v2, v3)?;
            let a1016 = f103(v0, v1, v2)?;
            nat_add(a1015, a1016)?
        };
        let a1022 = {
            let a1019 = f104(v0);
            let a1020 = {
                let a1017 = v1;
                let a1018 = v2;
                nat_mul(a1017, a1018)?
            };
            nat_mul(a1019, a1020)?
        };
        nat_add(a1021, a1022)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1023 = v0;
    let a1024 = 0u64;
    if if nat_eq(a1023, a1024) {
        true
    } else {
        let a1025 = v1;
        let a1026 = 0u64;
        nat_eq(a1025, a1026)
    } {
        Ok(3u64)
    } else {
        let a1029 = {
            let a1027 = 3u64;
            let a1028 = v1;
            nat_add(a1027, a1028)?
        };
        let a1030 = v2;
        nat_add(a1029, a1030)
    }
}

pub fn f38(v0: u64) -> bool {
    let a1031 = v0;
    let a1032 = 2139095040u64;
    if nat_le(a1031, a1032) {
        true
    } else {
        let a1033 = v0;
        let a1034 = 2147483648u64;
        nat_eq(a1033, a1034)
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a1035 = v0;
    let a1036 = 0u64;
    if nat_eq(a1035, a1036) {
        Ok(0u64)
    } else {
        let a1037 = v1;
        let a1038 = f105(v0)?;
        nat_mul(a1037, a1038)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1039 = v0;
    let a1040 = 0u64;
    if if if if nat_eq(a1039, a1040) {
        true
    } else {
        let a1041 = v1;
        let a1042 = 0u64;
        nat_eq(a1041, a1042)
    } {
        true
    } else {
        let a1043 = v2;
        let a1044 = 0u64;
        nat_eq(a1043, a1044)
    } {
        true
    } else {
        let a1045 = v3;
        let a1046 = 0u64;
        nat_eq(a1045, a1046)
    } {
        Ok(0u64)
    } else {
        let a1059 = {
            let a1049 = 2u64;
            let a1050 = {
                let a1047 = v1;
                let a1048 = v2;
                nat_mul(a1047, a1048)?
            };
            nat_mul(a1049, a1050)?
        };
        let a1060 = {
            let a1057 = {
                let a1053 = {
                    let a1051 = v1;
                    let a1052 = v2;
                    nat_mul(a1051, a1052)?
                };
                let a1054 = v3;
                nat_mul(a1053, a1054)?
            };
            let a1058 = {
                let a1055 = 1u64;
                let a1056 = v0;
                nat_add(a1055, a1056)?
            };
            nat_mul(a1057, a1058)?
        };
        nat_add(a1059, a1060)
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1061 = v2;
    match m1061 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h1062, _, _, _, _, _, _, _, _, _) => {
                    h1062
                }
            };
            let v6: u64 = {
                let a1068 = match v1.clone() {
                    Adt2::C0(_, _, h1063, _, _, _, _, _, _, _, _) => {
                        h1063
                    }
                };
                let a1069 = {
                    let a1066 = match v1.clone() {
                        Adt2::C0(_, _, _, h1064, _, _, _, _, _, _, _) => {
                            h1064
                        }
                    };
                    let a1067 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1065, _, _, _, _, _, _) => {
                            h1065
                        }
                    };
                    nat_mul(a1066, a1067)?
                };
                nat_mul(a1068, a1069)?
            };
            if if if if if if if if if if f25(v1.clone(), v4.clone()) {
                f25(v1.clone(), v3.clone())
            } else {
                false
            } {
                f106(v1.clone(), v6)?
            } else {
                false
            } {
                f106(v4.clone(), v6)?
            } else {
                false
            } {
                f106(v3.clone(), v6)?
            } else {
                false
            } {
                if f23(v1.clone(), v3.clone())? {
                    true
                } else {
                    f33(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f23(v4.clone(), v3.clone())? {
                    true
                } else {
                    f33(v4, v3.clone())
                }
            } else {
                false
            } {
                let a1070 = v5;
                let a1071 = 2147483647u64;
                nat_le(a1070, a1071)
            } else {
                false
            } {
                let a1072 = v6;
                let a1073 = 2147483647u64;
                nat_le(a1072, a1073)
            } else {
                false
            } {
                let a1075 = match v0.clone() {
                    Adt1::C0(h1074, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1074
                    }
                };
                let a1076 = 4294967295u64;
                if nat_le(a1075, a1076) {
                    let a1078 = match v0.clone() {
                        Adt1::C0(_, h1077, _, _, _, _, _, _, _, _, _, _) => {
                            h1077
                        }
                    };
                    let a1079 = 4294967295u64;
                    nat_le(a1078, a1079)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a1080 = v5;
                    let a1081 = 0u64;
                    if nat_eq(a1080, a1081) {
                        0u64
                    } else {
                        let a1082 = v6;
                        let a1083 = f107(v5)?;
                        nat_mul(a1082, a1083)?
                    }
                }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h1084, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1084
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f42(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h1085, _, _, _, _, _, _, _, _, _, _, _) => {
            h1085
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h1086, _, _, _, _, _, _, _, _, _, _) => {
            h1086
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h1087, _, _, _, _, _, _, _, _, _) => {
            h1087
        }
    };
    let v6: u64 = {
        let a1089 = match v1.clone() {
            Adt2::C0(_, h1088, _, _, _, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1090 = v4;
        let a1091 = 0u64;
        nat_quot(a1089, a1090, a1091)
    };
    let v7: u64 = {
        let a1093 = match v1.clone() {
            Adt2::C0(h1092, _, _, _, _, _, _, _, _, _, _) => {
                h1092
            }
        };
        let a1094 = 0u64;
        if nat_eq(a1093, a1094) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f108(match v1.clone() {
        Adt2::C0(h1095, _, _, _, _, _, _, _, _, _, _) => {
            h1095
        }
    }) {
        let a1098 = match v1.clone() {
            Adt2::C0(h1096, _, _, _, _, _, _, _, _, _, _) => {
                h1096
            }
        };
        let a1099 = match v2.clone() {
            Adt2::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
                h1097
            }
        };
        nat_eq(a1098, a1099)
    } else {
        false
    } {
        let a1100 = 0u64;
        let a1101 = v3;
        nat_lt(a1100, a1101)
    } else {
        false
    } {
        let a1102 = v3;
        let a1103 = 2147483648u64;
        nat_lt(a1102, a1103)
    } else {
        false
    } {
        let a1104 = 0u64;
        let a1105 = v4;
        nat_lt(a1104, a1105)
    } else {
        false
    } {
        let a1106 = v4;
        let a1107 = 2147483648u64;
        nat_lt(a1106, a1107)
    } else {
        false
    } {
        let a1108 = v5;
        let a1109 = 2147483648u64;
        nat_lt(a1108, a1109)
    } else {
        false
    } {
        let a1110 = 0u64;
        let a1111 = v6;
        nat_lt(a1110, a1111)
    } else {
        false
    } {
        let a1113 = 0u64;
        let a1114 = match v2.clone() {
            Adt2::C0(_, h1112, _, _, _, _, _, _, _, _, _) => {
                h1112
            }
        };
        nat_lt(a1113, a1114)
    } else {
        false
    } {
        let a1118 = match v1.clone() {
            Adt2::C0(_, h1115, _, _, _, _, _, _, _, _, _) => {
                h1115
            }
        };
        let a1119 = {
            let a1116 = v6;
            let a1117 = v4;
            nat_mul(a1116, a1117)?
        };
        nat_eq(a1118, a1119)
    } else {
        false
    } {
        let a1121 = match v2.clone() {
            Adt2::C0(_, _, h1120, _, _, _, _, _, _, _, _) => {
                h1120
            }
        };
        let a1122 = v4;
        nat_eq(a1121, a1122)
    } else {
        false
    } {
        let a1135 = {
            let a1128 = {
                let a1126 = match v2.clone() {
                    Adt2::C0(_, h1123, _, _, _, _, _, _, _, _, _) => {
                        h1123
                    }
                };
                let a1127 = {
                    let a1124 = v5;
                    let a1125 = 2u64;
                    nat_mul(a1124, a1125)?
                };
                nat_add(a1126, a1127)?
            };
            let a1129 = v3;
            nat_add(a1128, a1129)?
        };
        let a1136 = {
            let a1133 = {
                let a1131 = match v1.clone() {
                    Adt2::C0(_, _, h1130, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                };
                let a1132 = v3;
                nat_mul(a1131, a1132)?
            };
            let a1134 = v6;
            nat_add(a1133, a1134)?
        };
        nat_eq(a1135, a1136)
    } else {
        false
    } {
        f109(v1.clone(), v7)?
    } else {
        false
    } {
        f109(v2.clone(), v7)?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a1137 = f16(v2.clone(), 1u64)?;
            let a1138 = f110(v8.clone())?;
            nat_mul(a1137, a1138)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h1139, _, _, _, _, _, _, _, _, _, _) => {
                h1139
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f43(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1140 = v1;
    match m1140 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a1142 = match v0.clone() {
                Adt2::C0(h1141, _, _, _, _, _, _, _, _, _, _) => {
                    h1141
                }
            };
            let a1143 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a1142, a1143) {
                let a1145 = match v3.clone() {
                    Adt2::C0(h1144, _, _, _, _, _, _, _, _, _, _) => {
                        h1144
                    }
                };
                let a1146 = 0u64;
                nat_eq(a1145, a1146)
            } else {
                false
            } {
                let a1148 = match v2.clone() {
                    Adt2::C0(h1147, _, _, _, _, _, _, _, _, _, _) => {
                        h1147
                    }
                };
                let a1149 = 0u64;
                nat_eq(a1148, a1149)
            } else {
                false
            } {
                let a1151 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h1150, _, _, _, _, _, _) => {
                        h1150
                    }
                };
                let a1152 = 1u64;
                nat_eq(a1151, a1152)
            } else {
                false
            } {
                let a1154 = match v3.clone() {
                    Adt2::C0(_, _, _, h1153, _, _, _, _, _, _, _) => {
                        h1153
                    }
                };
                let a1155 = 1u64;
                nat_eq(a1154, a1155)
            } else {
                false
            } {
                let a1157 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1156, _, _, _, _, _, _) => {
                        h1156
                    }
                };
                let a1158 = 1u64;
                nat_eq(a1157, a1158)
            } else {
                false
            } {
                let a1161 = match v0.clone() {
                    Adt2::C0(_, _, h1159, _, _, _, _, _, _, _, _) => {
                        h1159
                    }
                };
                let a1162 = match v3.clone() {
                    Adt2::C0(_, _, h1160, _, _, _, _, _, _, _, _) => {
                        h1160
                    }
                };
                nat_eq(a1161, a1162)
            } else {
                false
            } {
                let a1167 = match v3.clone() {
                    Adt2::C0(_, h1163, _, _, _, _, _, _, _, _, _) => {
                        h1163
                    }
                };
                let a1168 = {
                    let a1165 = match v0.clone() {
                        Adt2::C0(_, h1164, _, _, _, _, _, _, _, _, _) => {
                            h1164
                        }
                    };
                    let a1166 = 1u64;
                    nat_add(a1165, a1166)?
                };
                nat_le(a1167, a1168)
            } else {
                false
            } {
                let a1171 = match v2.clone() {
                    Adt2::C0(_, h1169, _, _, _, _, _, _, _, _, _) => {
                        h1169
                    }
                };
                let a1172 = match v0.clone() {
                    Adt2::C0(_, _, h1170, _, _, _, _, _, _, _, _) => {
                        h1170
                    }
                };
                nat_eq(a1171, a1172)
            } else {
                false
            } {
                let a1180 = match v2.clone() {
                    Adt2::C0(_, _, h1173, _, _, _, _, _, _, _, _) => {
                        h1173
                    }
                };
                let a1181 = {
                    let a1178 = {
                        let a1175 = match v0.clone() {
                            Adt2::C0(_, h1174, _, _, _, _, _, _, _, _, _) => {
                                h1174
                            }
                        };
                        let a1176 = 1u64;
                        nat_add(a1175, a1176)?
                    };
                    let a1179 = match v3.clone() {
                        Adt2::C0(_, h1177, _, _, _, _, _, _, _, _, _) => {
                            h1177
                        }
                    };
                    nat_sub(a1178, a1179)
                };
                nat_eq(a1180, a1181)
            } else {
                false
            } {
                let a1184 = match v2.clone() {
                    Adt2::C0(_, _, _, h1182, _, _, _, _, _, _, _) => {
                        h1182
                    }
                };
                let a1185 = match v0.clone() {
                    Adt2::C0(_, _, _, h1183, _, _, _, _, _, _, _) => {
                        h1183
                    }
                };
                nat_eq(a1184, a1185)
            } else {
                false
            } {
                let a1187 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h1186, _, _, _, _, _, _) => {
                        h1186
                    }
                };
                let a1188 = 1u64;
                nat_eq(a1187, a1188)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a1190 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h1189, _, _, _, _, _) => {
                        h1189
                    }
                };
                let a1191 = 4u64;
                nat_eq(a1190, a1191)
            } else {
                false
            } {
                let a1193 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h1192, _, _, _, _, _) => {
                        h1192
                    }
                };
                let a1194 = 4u64;
                nat_eq(a1193, a1194)
            } else {
                false
            } {
                let a1199 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1195, _, _, _, _) => {
                        h1195
                    }
                };
                let a1200 = {
                    let a1197 = match v0.clone() {
                        Adt2::C0(_, h1196, _, _, _, _, _, _, _, _, _) => {
                            h1196
                        }
                    };
                    let a1198 = 4u64;
                    nat_mul(a1197, a1198)?
                };
                nat_eq(a1199, a1200)
            } else {
                false
            } {
                let a1205 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1201, _, _, _, _) => {
                        h1201
                    }
                };
                let a1206 = {
                    let a1203 = match v3.clone() {
                        Adt2::C0(_, h1202, _, _, _, _, _, _, _, _, _) => {
                            h1202
                        }
                    };
                    let a1204 = 4u64;
                    nat_mul(a1203, a1204)?
                };
                nat_eq(a1205, a1206)
            } else {
                false
            } {
                f23(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f23(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a1208 = f16(v2.clone(), 1u64)?;
                    let a1209 = f111(match v3.clone() {
                        Adt2::C0(_, h1207, _, _, _, _, _, _, _, _, _) => {
                            h1207
                        }
                    })?;
                    nat_mul(a1208, a1209)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f44(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h1210, _, _, _, _, _, _, _, _, _, _) => {
            h1210
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h1211, _, _, _, _, _, _, _, _) => {
                h1211
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1212, _, _, _, _, _, _, _, _, _) => {
                h1212
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1213, _, _, _, _, _, _) => {
                h1213
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h1214, _, _, _, _, _, _, _, _) => {
                h1214
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h1215, _, _, _, _, _, _, _, _, _) => {
                h1215
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h1216, _, _, _, _, _, _, _) => {
                h1216
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1217, _, _, _, _, _) => {
                h1217
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a1219 = match v1.clone() {
        Adt1::C0(h1218, _, _, _, _, _, _, _, _, _, _, _) => {
            h1218
        }
    };
    let a1220 = 1u64;
    if if if if if if if if if if if nat_le(a1219, a1220) {
        let a1222 = match v2.clone() {
            Adt2::C0(h1221, _, _, _, _, _, _, _, _, _, _) => {
                h1221
            }
        };
        let a1223 = 1u64;
        nat_le(a1222, a1223)
    } else {
        false
    } {
        let a1225 = match v3.clone() {
            Adt2::C0(h1224, _, _, _, _, _, _, _, _, _, _) => {
                h1224
            }
        };
        let a1226 = 0u64;
        nat_eq(a1225, a1226)
    } else {
        false
    } {
        let a1231 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1227, _, _, _, _, _) => {
                h1227
            }
        };
        let a1232 = {
            let a1229 = match v2.clone() {
                Adt2::C0(h1228, _, _, _, _, _, _, _, _, _, _) => {
                    h1228
                }
            };
            let a1230 = 0u64;
            if nat_eq(a1229, a1230) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a1231, a1232)
    } else {
        false
    } {
        f22(v3.clone())?
    } else {
        false
    } {
        f23(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f112(v4.clone(), match v2.clone() {
            Adt2::C0(_, h1233, _, _, _, _, _, _, _, _, _) => {
                h1233
            }
        }, match v3.clone() {
            Adt2::C0(_, h1234, _, _, _, _, _, _, _, _, _) => {
                h1234
            }
        })?
    } else {
        false
    } {
        if v0 {
            f112(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1235, _, _, _, _, _, _, _, _) => {
                    h1235
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h1236, _, _, _, _, _, _, _, _) => {
                    h1236
                }
            })?
        } else {
            let a1239 = match v2.clone() {
                Adt2::C0(_, _, h1237, _, _, _, _, _, _, _, _) => {
                    h1237
                }
            };
            let a1240 = match v3.clone() {
                Adt2::C0(_, _, h1238, _, _, _, _, _, _, _, _) => {
                    h1238
                }
            };
            nat_eq(a1239, a1240)
        }
    } else {
        false
    } {
        let a1243 = match v2.clone() {
            Adt2::C0(_, _, _, h1241, _, _, _, _, _, _, _) => {
                h1241
            }
        };
        let a1244 = match v3.clone() {
            Adt2::C0(_, _, _, h1242, _, _, _, _, _, _, _) => {
                h1242
            }
        };
        nat_eq(a1243, a1244)
    } else {
        false
    } {
        let a1247 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1245, _, _, _, _, _, _) => {
                h1245
            }
        };
        let a1248 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1246, _, _, _, _, _, _) => {
                h1246
            }
        };
        nat_eq(a1247, a1248)
    } else {
        false
    } {
        let a1253 = {
            let a1251 = match v4.clone() {
                Adt6::C0(h1249, _, _, _) => {
                    h1249
                }
            };
            let a1252 = match v5.clone() {
                Adt6::C0(h1250, _, _, _) => {
                    h1250
                }
            };
            nat_mul(a1251, a1252)?
        };
        let a1254 = 2147483648u64;
        nat_lt(a1253, a1254)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a1256 = match v1 {
                Adt1::C0(h1255, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1255
                }
            };
            let a1257 = 0u64;
            nat_eq(a1256, a1257)
        }, match v2.clone() {
            Adt2::C0(h1258, _, _, _, _, _, _, _, _, _, _) => {
                h1258
            }
        });
        Ok(Adt10::C4({
            let a1259 = f16(v3.clone(), 1u64)?;
            let a1260 = f113(v6.clone())?;
            nat_mul(a1259, a1260)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f45(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1261 = v2;
    match m1261 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a1263 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1262, _, _, _, _, _) => {
                        h1262
                    }
                };
                let a1264 = 1u64;
                nat_eq(a1263, a1264)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1265, _, _, _, _, _, _, _, _, _) => {
                    h1265
                }
            }, match v0.clone() {
                Adt1::C0(h1266, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1266
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1267, _, _, _, _, _, _, _, _, _) => {
                    h1267
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1268, _, _, _, _, _, _, _) => {
                    h1268
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h1269, _, _, _, _, _, _, _, _) => {
                        h1269
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h1270, _, _, _, _, _, _, _, _, _, _) => {
                        h1270
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h1271, _, _, _, _, _, _, _, _) => {
                        h1271
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h1272, _, _, _, _, _, _) => {
                        h1272
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1273, _, _, _, _, _, _, _) => {
                        h1273
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h1274, _, _, _, _, _, _, _, _) => {
                        h1274
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h1275, _, _, _, _, _, _) => {
                        h1275
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1276, _, _, _, _, _, _, _) => {
                        h1276
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h1277, _, _, _, _, _, _, _, _) => {
                        h1277
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h1278, _, _, _, _, _, _, _, _, _, _) => {
                    h1278
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h1279, _, _, _, _, _, _, _, _, _, _) => {
                    h1279
                }
            };
            let v14: u64 = {
                let a1280 = v12;
                let a1281 = 0u64;
                if nat_eq(a1280, a1281) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a1283 = match v4.clone() {
                    Adt2::C0(_, h1282, _, _, _, _, _, _, _, _, _) => {
                        h1282
                    }
                };
                let a1284 = v14;
                nat_mul(a1283, a1284)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1285, _, _, _) => {
                        h1285
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1286, _, _, _, _) => {
                        h1286
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1287, _, _) => {
                        h1287
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1288, _, _, _) => {
                        h1288
                    }
                }
            };
            let a1290 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h1289, _, _, _, _, _) => {
                    h1289
                }
            };
            let a1291 = 1u64;
            if if if if if if if if if if nat_le(a1290, a1291) {
                let a1292 = v12;
                let a1293 = 0u64;
                if nat_eq(a1292, a1293) {
                    true
                } else {
                    let a1294 = v12;
                    let a1295 = 1u64;
                    if nat_eq(a1294, a1295) {
                        let a1296 = v13;
                        let a1297 = 1u64;
                        nat_eq(a1296, a1297)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a1298 = v13;
                let a1299 = 0u64;
                if nat_eq(a1298, a1299) {
                    true
                } else {
                    let a1300 = v13;
                    let a1301 = 1u64;
                    nat_eq(a1300, a1301)
                }
            } else {
                false
            } {
                let a1303 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1302, _, _, _, _, _) => {
                        h1302
                    }
                };
                let a1304 = v14;
                nat_eq(a1303, a1304)
            } else {
                false
            } {
                let a1312 = match v3.clone() {
                    Adt2::C0(_, h1305, _, _, _, _, _, _, _, _, _) => {
                        h1305
                    }
                };
                let a1313 = {
                    let a1310 = v9;
                    let a1311 = {
                        let a1308 = match v6.clone() {
                            Adt6::C0(h1306, _, _, _) => {
                                h1306
                            }
                        };
                        let a1309 = match v7.clone() {
                            Adt6::C0(h1307, _, _, _) => {
                                h1307
                            }
                        };
                        nat_mul(a1308, a1309)?
                    };
                    nat_mul(a1310, a1311)?
                };
                nat_eq(a1312, a1313)
            } else {
                false
            } {
                let a1316 = match v3.clone() {
                    Adt2::C0(_, _, h1314, _, _, _, _, _, _, _, _) => {
                        h1314
                    }
                };
                let a1317 = f94(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1315, _, _, _, _, _, _, _, _, _) => {
                        h1315
                    }
                })?;
                nat_eq(a1316, a1317)
            } else {
                false
            } {
                let a1319 = 0u64;
                let a1320 = match v3.clone() {
                    Adt2::C0(_, _, h1318, _, _, _, _, _, _, _, _) => {
                        h1318
                    }
                };
                nat_lt(a1319, a1320)
            } else {
                false
            } {
                if v5 {
                    let a1322 = match v1 {
                        Adt2::C0(_, _, _, h1321, _, _, _, _, _, _, _) => {
                            h1321
                        }
                    };
                    let a1323 = v9;
                    if if if nat_eq(a1322, a1323) {
                        let a1325 = match v3.clone() {
                            Adt2::C0(_, _, _, h1324, _, _, _, _, _, _, _) => {
                                h1324
                            }
                        };
                        let a1326 = f94(v7.clone(), v11)?;
                        nat_eq(a1325, a1326)
                    } else {
                        false
                    } {
                        let a1328 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1327, _, _, _, _, _, _) => {
                                h1327
                            }
                        };
                        let a1329 = v10;
                        nat_eq(a1328, a1329)
                    } else {
                        false
                    } {
                        let a1331 = 0u64;
                        let a1332 = match v3.clone() {
                            Adt2::C0(_, _, _, h1330, _, _, _, _, _, _, _) => {
                                h1330
                            }
                        };
                        nat_lt(a1331, a1332)
                    } else {
                        false
                    }
                } else {
                    let a1334 = match v1 {
                        Adt2::C0(_, _, h1333, _, _, _, _, _, _, _, _) => {
                            h1333
                        }
                    };
                    let a1335 = v9;
                    if if if nat_eq(a1334, a1335) {
                        let a1337 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1336, _, _, _, _, _, _) => {
                                h1336
                            }
                        };
                        let a1338 = 1u64;
                        nat_eq(a1337, a1338)
                    } else {
                        false
                    } {
                        let a1340 = match v3.clone() {
                            Adt2::C0(_, _, _, h1339, _, _, _, _, _, _, _) => {
                                h1339
                            }
                        };
                        let a1341 = v10;
                        nat_eq(a1340, a1341)
                    } else {
                        false
                    } {
                        let a1343 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1342, _, _, _, _, _, _) => {
                                h1342
                            }
                        };
                        let a1344 = 1u64;
                        nat_eq(a1343, a1344)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f34(match v4.clone() {
                    Adt2::C0(_, h1345, _, _, _, _, _, _, _, _, _) => {
                        h1345
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1365 = {
                        let a1362 = {
                            let a1360 = {
                                let a1354 = {
                                    let a1348 = {
                                        let a1346 = v10;
                                        let a1347 = 1u64;
                                        nat_sub(a1346, a1347)
                                    };
                                    let a1349 = v17;
                                    nat_mul(a1348, a1349)?
                                };
                                let a1355 = {
                                    let a1352 = {
                                        let a1350 = v9;
                                        let a1351 = 1u64;
                                        nat_sub(a1350, a1351)
                                    };
                                    let a1353 = v16;
                                    nat_mul(a1352, a1353)?
                                };
                                nat_add(a1354, a1355)?
                            };
                            let a1361 = {
                                let a1358 = {
                                    let a1356 = v11;
                                    let a1357 = 1u64;
                                    nat_sub(a1356, a1357)
                                };
                                let a1359 = v15;
                                nat_mul(a1358, a1359)?
                            };
                            nat_add(a1360, a1361)?
                        };
                        let a1363 = v15;
                        nat_add(a1362, a1363)?
                    };
                    let a1366 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1364, _) => {
                            h1364
                        }
                    };
                    nat_le(a1365, a1366)
                }
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1367 = v13;
                    let a1368 = 0u64;
                    if nat_eq(a1367, a1368) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1369, _, _, _, _, _, _, _, _, _) => {
                        h1369
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1370, _, _, _, _, _, _, _, _, _) => {
                            h1370
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1371, _, _, _, _, _, _, _, _) => {
                            h1371
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1372, _, _, _, _, _, _, _) => {
                            h1372
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1373, _, _, _, _, _) => {
                            h1373
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1374, _, _, _, _) => {
                            h1374
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1375, _, _, _) => {
                            h1375
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f46(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1376 = v2;
    match m1376 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1377, _, _, _, _, _, _, _, _, _) => {
                    h1377
                }
            }, match v0.clone() {
                Adt1::C0(h1378, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1378
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1379, _, _, _, _, _, _, _, _) => {
                    h1379
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1380, _, _, _, _, _) => {
                    h1380
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1381, _, _, _, _, _, _, _, _) => {
                    h1381
                }
            }, match v0.clone() {
                Adt1::C0(_, h1382, _, _, _, _, _, _, _, _, _, _) => {
                    h1382
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1383, _, _, _, _, _, _, _) => {
                    h1383
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1384, _, _, _, _) => {
                    h1384
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1385, _, _, _, _, _, _, _) => {
                    h1385
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1386, _, _, _, _, _, _, _, _, _) => {
                    h1386
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1387, _, _, _, _, _, _) => {
                    h1387
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1388, _, _, _) => {
                    h1388
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1389, _, _) => {
                    h1389
                }
            };
            let v9: u64 = f94(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1390, _, _, _, _, _, _, _) => {
                    h1390
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1391, _, _, _, _, _, _, _, _, _, _) => {
                    h1391
                }
            };
            let v11: u64 = {
                let a1399 = v8;
                let a1400 = {
                    let a1397 = match v5.clone() {
                        Adt6::C0(h1392, _, _, _) => {
                            h1392
                        }
                    };
                    let a1398 = {
                        let a1395 = match v6.clone() {
                            Adt6::C0(h1393, _, _, _) => {
                                h1393
                            }
                        };
                        let a1396 = match v7.clone() {
                            Adt6::C0(h1394, _, _, _) => {
                                h1394
                            }
                        };
                        nat_mul(a1395, a1396)?
                    };
                    nat_mul(a1397, a1398)?
                };
                nat_mul(a1399, a1400)?
            };
            let a1402 = match v4.clone() {
                Adt2::C0(h1401, _, _, _, _, _, _, _, _, _, _) => {
                    h1401
                }
            };
            let a1403 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1402, a1403) {
                let a1404 = v10;
                let a1405 = 0u64;
                if nat_eq(a1404, a1405) {
                    true
                } else {
                    let a1406 = v10;
                    let a1407 = 1u64;
                    nat_eq(a1406, a1407)
                }
            } else {
                false
            } {
                let a1408 = 0u64;
                let a1409 = v8;
                nat_lt(a1408, a1409)
            } else {
                false
            } {
                let a1410 = v8;
                let a1411 = 2147483648u64;
                nat_lt(a1410, a1411)
            } else {
                false
            } {
                let a1413 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1412, _, _, _, _, _) => {
                        h1412
                    }
                };
                let a1414 = 4u64;
                nat_eq(a1413, a1414)
            } else {
                false
            } {
                let a1415 = 0u64;
                let a1416 = v9;
                nat_lt(a1415, a1416)
            } else {
                false
            } {
                let a1418 = 0u64;
                let a1419 = match v3.clone() {
                    Adt2::C0(_, _, h1417, _, _, _, _, _, _, _, _) => {
                        h1417
                    }
                };
                nat_lt(a1418, a1419)
            } else {
                false
            } {
                let a1421 = 0u64;
                let a1422 = match v3.clone() {
                    Adt2::C0(_, _, _, h1420, _, _, _, _, _, _, _) => {
                        h1420
                    }
                };
                nat_lt(a1421, a1422)
            } else {
                false
            } {
                let a1424 = match v3.clone() {
                    Adt2::C0(_, h1423, _, _, _, _, _, _, _, _, _) => {
                        h1423
                    }
                };
                let a1425 = v11;
                nat_eq(a1424, a1425)
            } else {
                false
            } {
                let a1428 = match v3.clone() {
                    Adt2::C0(_, _, h1426, _, _, _, _, _, _, _, _) => {
                        h1426
                    }
                };
                let a1429 = f94(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1427, _, _, _, _, _, _, _, _, _) => {
                        h1427
                    }
                })?;
                nat_eq(a1428, a1429)
            } else {
                false
            } {
                let a1432 = match v3.clone() {
                    Adt2::C0(_, _, _, h1430, _, _, _, _, _, _, _) => {
                        h1430
                    }
                };
                let a1433 = f94(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1431, _, _, _, _, _, _, _, _) => {
                        h1431
                    }
                })?;
                nat_eq(a1432, a1433)
            } else {
                false
            } {
                let a1441 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1434, _, _, _, _, _, _) => {
                        h1434
                    }
                };
                let a1442 = {
                    let a1439 = v9;
                    let a1440 = {
                        let a1436 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1435, _, _, _, _, _, _) => {
                                h1435
                            }
                        };
                        let a1437 = v8;
                        let a1438 = 0u64;
                        nat_quot(a1436, a1437, a1438)
                    };
                    nat_mul(a1439, a1440)?
                };
                nat_eq(a1441, a1442)
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1443 = v10;
                    let a1444 = 0u64;
                    if nat_eq(a1443, a1444) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1445, _, _) => {
                        h1445
                    }
                }, {
                    let a1447 = v8;
                    let a1448 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1446, _, _) => {
                            h1446
                        }
                    };
                    nat_mul(a1447, a1448)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f47(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1449 = v1;
    match m1449 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f114(v0, v3, v2)
        }
    }
}

pub fn f48(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1451 = match v0.clone() {
        Adt2::C0(h1450, _, _, _, _, _, _, _, _, _, _) => {
            h1450
        }
    };
    let a1452 = 1u64;
    if if nat_eq(a1451, a1452) {
        true
    } else {
        let a1454 = match v0.clone() {
            Adt2::C0(h1453, _, _, _, _, _, _, _, _, _, _) => {
                h1453
            }
        };
        let a1455 = 30u64;
        nat_eq(a1454, a1455)
    } {
        let a1457 = match v1.clone() {
            Adt2::C0(h1456, _, _, _, _, _, _, _, _, _, _) => {
                h1456
            }
        };
        let a1458 = 1u64;
        if if if if if if if if if if if nat_eq(a1457, a1458) {
            let a1461 = match v0.clone() {
                Adt2::C0(_, h1459, _, _, _, _, _, _, _, _, _) => {
                    h1459
                }
            };
            let a1462 = match v1.clone() {
                Adt2::C0(_, h1460, _, _, _, _, _, _, _, _, _) => {
                    h1460
                }
            };
            nat_eq(a1461, a1462)
        } else {
            false
        } {
            let a1464 = 0u64;
            let a1465 = match v1.clone() {
                Adt2::C0(_, _, h1463, _, _, _, _, _, _, _, _) => {
                    h1463
                }
            };
            nat_lt(a1464, a1465)
        } else {
            false
        } {
            let a1468 = match v1.clone() {
                Adt2::C0(_, _, h1466, _, _, _, _, _, _, _, _) => {
                    h1466
                }
            };
            let a1469 = match v1.clone() {
                Adt2::C0(_, _, _, h1467, _, _, _, _, _, _, _) => {
                    h1467
                }
            };
            nat_eq(a1468, a1469)
        } else {
            false
        } {
            let a1471 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1470, _, _, _, _, _, _) => {
                    h1470
                }
            };
            let a1472 = 1u64;
            nat_eq(a1471, a1472)
        } else {
            false
        } {
            let a1480 = match v0.clone() {
                Adt2::C0(_, _, h1473, _, _, _, _, _, _, _, _) => {
                    h1473
                }
            };
            let a1481 = {
                let a1478 = {
                    let a1476 = match v1.clone() {
                        Adt2::C0(_, _, h1474, _, _, _, _, _, _, _, _) => {
                            h1474
                        }
                    };
                    let a1477 = match v1.clone() {
                        Adt2::C0(_, _, h1475, _, _, _, _, _, _, _, _) => {
                            h1475
                        }
                    };
                    nat_add(a1476, a1477)?
                };
                let a1479 = 1u64;
                nat_sub(a1478, a1479)
            };
            nat_eq(a1480, a1481)
        } else {
            false
        } {
            let a1483 = 0u64;
            let a1484 = match v0.clone() {
                Adt2::C0(_, _, _, h1482, _, _, _, _, _, _, _) => {
                    h1482
                }
            };
            nat_lt(a1483, a1484)
        } else {
            false
        } {
            let a1486 = 0u64;
            let a1487 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1485, _, _, _, _, _, _) => {
                    h1485
                }
            };
            nat_lt(a1486, a1487)
        } else {
            false
        } {
            let a1489 = 2u64;
            let a1490 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1488, _, _, _, _, _) => {
                    h1488
                }
            };
            nat_le(a1489, a1490)
        } else {
            false
        } {
            let a1492 = 2u64;
            let a1493 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1491, _, _, _, _, _) => {
                    h1491
                }
            };
            nat_le(a1492, a1493)
        } else {
            false
        } {
            f23(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f16(v1.clone(), 1u64)?, 2u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f49(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1494 = v1;
    match m1494 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f115(v0, v3, v2)
        }
    }
}

pub fn f50(v0: u64) -> bool {
    let a1495 = v0;
    let a1496 = 1u64;
    if if nat_eq(a1495, a1496) {
        true
    } else {
        let a1497 = v0;
        let a1498 = 33u64;
        nat_eq(a1497, a1498)
    } {
        true
    } else {
        let a1499 = v0;
        let a1500 = 34u64;
        nat_eq(a1499, a1500)
    }
}

pub fn f51(v0: u64) -> bool {
    if f116(v0) {
        !f117(v0)
    } else {
        false
    }
}

pub fn f52(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1501 = v1;
    match m1501 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1503 = f118(match v0.clone() {
                Adt2::C0(h1502, _, _, _, _, _, _, _, _, _, _) => {
                    h1502
                }
            });
            match m1503 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1504, _) => {
                            h1504
                        }
                    };
                    if if if if if if f33(v3, v2.clone()) {
                        f23(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1505 = f16(v0.clone(), 1u64)?;
                        let a1506 = f16(v2.clone(), 1u64)?;
                        nat_eq(a1505, a1506)
                    } else {
                        false
                    } {
                        let a1511 = {
                            let a1508 = match v0.clone() {
                                Adt2::C0(_, h1507, _, _, _, _, _, _, _, _, _) => {
                                    h1507
                                }
                            };
                            let a1509 = v5;
                            let a1510 = 1u64;
                            nat_rem(a1508, a1509, a1510)
                        };
                        let a1512 = 0u64;
                        if nat_eq(a1511, a1512) {
                            let a1515 = match v4 {
                                Adt18::C0(_, _, h1513) => {
                                    h1513
                                }
                            };
                            let a1516 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1514, _, _, _, _, _) => {
                                    h1514
                                }
                            };
                            nat_le(a1515, a1516)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1517 = v5;
                        let a1518 = 1u64;
                        if nat_eq(a1517, a1518) {
                            let a1520 = 4u64;
                            let a1521 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1519, _, _, _, _, _) => {
                                    h1519
                                }
                            };
                            nat_le(a1520, a1521)
                        } else {
                            let a1523 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1522, _, _, _, _, _) => {
                                    h1522
                                }
                            };
                            let a1524 = 4u64;
                            nat_eq(a1523, a1524)
                        }
                    } else {
                        false
                    } {
                        let a1529 = {
                            let a1526 = match v2.clone() {
                                Adt2::C0(_, h1525, _, _, _, _, _, _, _, _, _) => {
                                    h1525
                                }
                            };
                            let a1527 = v5;
                            let a1528 = 1u64;
                            nat_rem(a1526, a1527, a1528)
                        };
                        let a1530 = 0u64;
                        if nat_eq(a1529, a1530) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1531, _, _, _, _, _, _, _, _, _, _) => {
                                h1531
                            }
                        }, 0u64)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1534 = match v1.clone() {
        Adt2::C0(h1532, _, _, _, _, _, _, _, _, _, _) => {
            h1532
        }
    };
    let a1535 = match v3.clone() {
        Adt2::C0(h1533, _, _, _, _, _, _, _, _, _, _) => {
            h1533
        }
    };
    if nat_eq(a1534, a1535) {
        f119(v0, v1, v2, v3)
    } else {
        if if if if f117(match v1.clone() {
            Adt2::C0(h1536, _, _, _, _, _, _, _, _, _, _) => {
                h1536
            }
        }) {
            f117(match v3.clone() {
                Adt2::C0(h1537, _, _, _, _, _, _, _, _, _, _) => {
                    h1537
                }
            })
        } else {
            false
        } {
            let a1538 = v0;
            let a1539 = 33u64;
            if nat_eq(a1538, a1539) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1540 = f16(v1.clone(), 1u64)?;
            let a1541 = f16(v3.clone(), 1u64)?;
            nat_eq(a1540, a1541)
        } else {
            false
        } {
            let m1543 = f118(match v3.clone() {
                Adt2::C0(h1542, _, _, _, _, _, _, _, _, _, _) => {
                    h1542
                }
            });
            match m1543 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1544, _, _, _, _, _, _, _, _, _, _) => {
                            h1544
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1545, _, _, _, _, _, _, _, _, _, _) => {
                            h1545
                        }
                    };
                    let v7: bool = {
                        let a1546 = v5;
                        let a1547 = 43u64;
                        if nat_eq(a1546, a1547) {
                            true
                        } else {
                            let a1548 = v6;
                            let a1549 = 43u64;
                            nat_eq(a1548, a1549)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1550 = f16(v3.clone(), 1u64)?;
                        let a1551 = if v7 {
                            f120(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1550, a1551)?
                    }, match v4 {
                        Adt18::C0(_, _, h1552) => {
                            h1552
                        }
                    }, f20(v1, 1u64), f20(v3, 1u64), if v7 {
                        Adt4::C31(v5, v6)
                    } else {
                        Adt4::C11(v5, v6)
                    }))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    }
}

pub fn f54(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1554 = match v0.clone() {
        Adt2::C0(h1553, _, _, _, _, _, _, _, _, _, _) => {
            h1553
        }
    };
    let a1555 = 0u64;
    if if if if nat_eq(a1554, a1555) {
        true
    } else {
        let a1557 = match v0.clone() {
            Adt2::C0(h1556, _, _, _, _, _, _, _, _, _, _) => {
                h1556
            }
        };
        let a1558 = 1u64;
        nat_eq(a1557, a1558)
    } {
        true
    } else {
        let a1560 = match v0.clone() {
            Adt2::C0(h1559, _, _, _, _, _, _, _, _, _, _) => {
                h1559
            }
        };
        let a1561 = 25u64;
        if nat_eq(a1560, a1561) {
            true
        } else {
            let a1563 = match v0.clone() {
                Adt2::C0(h1562, _, _, _, _, _, _, _, _, _, _) => {
                    h1562
                }
            };
            let a1564 = 26u64;
            nat_eq(a1563, a1564)
        }
    } {
        true
    } else {
        let a1566 = match v0.clone() {
            Adt2::C0(h1565, _, _, _, _, _, _, _, _, _, _) => {
                h1565
            }
        };
        let a1567 = 30u64;
        nat_eq(a1566, a1567)
    } {
        let a1570 = match v0.clone() {
            Adt2::C0(h1568, _, _, _, _, _, _, _, _, _, _) => {
                h1568
            }
        };
        let a1571 = match v1.clone() {
            Adt2::C0(h1569, _, _, _, _, _, _, _, _, _, _) => {
                h1569
            }
        };
        if if nat_eq(a1570, a1571) {
            f121(match v1.clone() {
                Adt2::C0(_, h1572, _, _, _, _, _, _, _, _, _) => {
                    h1572
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1573, _, _, _, _, _, _, _, _) => {
                    h1573
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1574, _, _, _, _, _, _, _) => {
                    h1574
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1575, _, _, _, _, _, _) => {
                    h1575
                }
            }, match v0.clone() {
                Adt2::C0(_, h1576, _, _, _, _, _, _, _, _, _) => {
                    h1576
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1577, _, _, _, _, _, _, _, _) => {
                    h1577
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1578, _, _, _, _, _, _, _) => {
                    h1578
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1579, _, _, _, _, _, _) => {
                    h1579
                }
            })
        } else {
            false
        } {
            let m1581 = f118(match v0.clone() {
                Adt2::C0(h1580, _, _, _, _, _, _, _, _, _, _) => {
                    h1580
                }
            });
            match m1581 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1582) => {
                            h1582
                        }
                    }, f20(v0, 1u64), f20(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f55(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1583 = v2;
    match m1583 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1584 = v0;
            let a1585 = 4u64;
            if if if nat_lt(a1584, a1585) {
                let a1588 = match v1.clone() {
                    Adt2::C0(h1586, _, _, _, _, _, _, _, _, _, _) => {
                        h1586
                    }
                };
                let a1589 = match v4.clone() {
                    Adt2::C0(h1587, _, _, _, _, _, _, _, _, _, _) => {
                        h1587
                    }
                };
                if nat_eq(a1588, a1589) {
                    let a1592 = match v1.clone() {
                        Adt2::C0(h1590, _, _, _, _, _, _, _, _, _, _) => {
                            h1590
                        }
                    };
                    let a1593 = match v3.clone() {
                        Adt2::C0(h1591, _, _, _, _, _, _, _, _, _, _) => {
                            h1591
                        }
                    };
                    nat_eq(a1592, a1593)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f122(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1594, _, _, _, _, _, _, _, _, _) => {
                        h1594
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1595, _, _, _, _, _, _, _, _, _) => {
                        h1595
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1596, _, _, _, _, _, _, _, _, _) => {
                        h1596
                    }
                })? {
                    f122(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1597, _, _, _, _, _, _, _, _) => {
                            h1597
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1598, _, _, _, _, _, _, _, _) => {
                            h1598
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1599, _, _, _, _, _, _, _, _) => {
                            h1599
                        }
                    })?
                } else {
                    false
                } {
                    f122(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1600, _, _, _, _, _, _, _) => {
                            h1600
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1601, _, _, _, _, _, _, _) => {
                            h1601
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1602, _, _, _, _, _, _, _) => {
                            h1602
                        }
                    })?
                } else {
                    false
                } {
                    f122(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1603, _, _, _, _, _, _) => {
                            h1603
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1604, _, _, _, _, _, _) => {
                            h1604
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1605, _, _, _, _, _, _) => {
                            h1605
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1607 = f118(match v1.clone() {
                    Adt2::C0(h1606, _, _, _, _, _, _, _, _, _, _) => {
                        h1606
                    }
                });
                match m1607 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1608, _) => {
                                h1608
                            }
                        };
                        let a1613 = {
                            let a1610 = match v1.clone() {
                                Adt2::C0(_, h1609, _, _, _, _, _, _, _, _, _) => {
                                    h1609
                                }
                            };
                            let a1611 = v6;
                            let a1612 = 1u64;
                            nat_rem(a1610, a1611, a1612)
                        };
                        let a1614 = 0u64;
                        if if if nat_eq(a1613, a1614) {
                            let a1619 = {
                                let a1616 = match v4.clone() {
                                    Adt2::C0(_, h1615, _, _, _, _, _, _, _, _, _) => {
                                        h1615
                                    }
                                };
                                let a1617 = v6;
                                let a1618 = 1u64;
                                nat_rem(a1616, a1617, a1618)
                            };
                            let a1620 = 0u64;
                            nat_eq(a1619, a1620)
                        } else {
                            false
                        } {
                            let a1625 = {
                                let a1622 = match v3.clone() {
                                    Adt2::C0(_, h1621, _, _, _, _, _, _, _, _, _) => {
                                        h1621
                                    }
                                };
                                let a1623 = v6;
                                let a1624 = 1u64;
                                nat_rem(a1622, a1623, a1624)
                            };
                            let a1626 = 0u64;
                            nat_eq(a1625, a1626)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1627) => {
                                    h1627
                                }
                            }, f20(v1, v6), f20(v3, v6), Adt4::C2(v0, f20(v4, v6))))
                        } else {
                            Ok(Adt10::C1)
                        }
                    }
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f56(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1629 = match v2.clone() {
        Adt2::C0(h1628, _, _, _, _, _, _, _, _, _, _) => {
            h1628
        }
    };
    let a1630 = 0u64;
    if if nat_eq(a1629, a1630) {
        let a1632 = match v3.clone() {
            Adt2::C0(h1631, _, _, _, _, _, _, _, _, _, _) => {
                h1631
            }
        };
        let a1633 = 0u64;
        nat_eq(a1632, a1633)
    } else {
        false
    } {
        let a1634 = v0;
        let a1635 = 61u64;
        if nat_eq(a1634, a1635) {
            let v4: bool = {
                let a1637 = 0u64;
                let a1638 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1636, _, _, _) => {
                        h1636
                    }
                };
                nat_lt(a1637, a1638)
            };
            if if if if if if f123(match v2.clone() {
                Adt2::C0(_, h1639, _, _, _, _, _, _, _, _, _) => {
                    h1639
                }
            }, match v1.clone() {
                Adt1::C0(h1640, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1640
                }
            }, match v1.clone() {
                Adt1::C0(_, h1641, _, _, _, _, _, _, _, _, _, _) => {
                    h1641
                }
            }, match v3.clone() {
                Adt2::C0(_, h1642, _, _, _, _, _, _, _, _, _) => {
                    h1642
                }
            })? {
                f123(match v2.clone() {
                    Adt2::C0(_, _, h1643, _, _, _, _, _, _, _, _) => {
                        h1643
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1644, _, _, _, _, _, _, _, _, _) => {
                        h1644
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1645, _, _, _, _, _, _, _, _) => {
                        h1645
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1646, _, _, _, _, _, _, _, _) => {
                        h1646
                    }
                })?
            } else {
                false
            } {
                f123(match v2.clone() {
                    Adt2::C0(_, _, _, h1647, _, _, _, _, _, _, _) => {
                        h1647
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1648, _, _, _, _, _, _, _) => {
                        h1648
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1649, _, _, _, _, _, _) => {
                        h1649
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1650, _, _, _, _, _, _, _) => {
                        h1650
                    }
                })?
            } else {
                false
            } {
                f123(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1651, _, _, _, _, _, _) => {
                        h1651
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1652, _, _, _, _, _) => {
                        h1652
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1653, _, _, _, _) => {
                        h1653
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1654, _, _, _, _, _, _) => {
                        h1654
                    }
                })?
            } else {
                false
            } {
                let a1656 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1655, _, _, _) => {
                        h1655
                    }
                };
                let a1657 = 4294967295u64;
                nat_le(a1656, a1657)
            } else {
                false
            } {
                if v4 {
                    !f34(match v2.clone() {
                        Adt2::C0(_, h1658, _, _, _, _, _, _, _, _, _) => {
                            h1658
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1659, _, _, _, _, _, _, _, _) => {
                            h1659
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1660, _, _, _, _, _, _, _) => {
                            h1660
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1661, _, _, _, _, _, _) => {
                            h1661
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1662, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1662
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1663, _, _, _, _, _, _, _, _, _) => {
                        h1663
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1664, _, _, _, _, _, _, _) => {
                        h1664
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1665, _, _, _, _, _) => {
                        h1665
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1666 = v0;
            let a1667 = 62u64;
            if nat_eq(a1666, a1667) {
                let a1669 = match v1.clone() {
                    Adt1::C0(h1668, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1668
                    }
                };
                let a1670 = 2147483648u64;
                if if if if if nat_lt(a1669, a1670) {
                    let a1672 = match v1.clone() {
                        Adt1::C0(_, h1671, _, _, _, _, _, _, _, _, _, _) => {
                            h1671
                        }
                    };
                    let a1673 = 2147483648u64;
                    nat_lt(a1672, a1673)
                } else {
                    false
                } {
                    let a1676 = match v1.clone() {
                        Adt1::C0(h1674, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1674
                        }
                    };
                    let a1677 = match v2.clone() {
                        Adt2::C0(_, h1675, _, _, _, _, _, _, _, _, _) => {
                            h1675
                        }
                    };
                    nat_lt(a1676, a1677)
                } else {
                    false
                } {
                    let a1680 = match v1.clone() {
                        Adt1::C0(_, h1678, _, _, _, _, _, _, _, _, _, _) => {
                            h1678
                        }
                    };
                    let a1681 = match v2.clone() {
                        Adt2::C0(_, h1679, _, _, _, _, _, _, _, _, _) => {
                            h1679
                        }
                    };
                    nat_lt(a1680, a1681)
                } else {
                    false
                } {
                    let a1684 = match v2.clone() {
                        Adt2::C0(_, _, h1682, _, _, _, _, _, _, _, _) => {
                            h1682
                        }
                    };
                    let a1685 = match v3.clone() {
                        Adt2::C0(_, _, h1683, _, _, _, _, _, _, _, _) => {
                            h1683
                        }
                    };
                    if if if nat_eq(a1684, a1685) {
                        let a1688 = match v2.clone() {
                            Adt2::C0(_, _, _, h1686, _, _, _, _, _, _, _) => {
                                h1686
                            }
                        };
                        let a1689 = match v3.clone() {
                            Adt2::C0(_, _, _, h1687, _, _, _, _, _, _, _) => {
                                h1687
                            }
                        };
                        nat_eq(a1688, a1689)
                    } else {
                        false
                    } {
                        let a1692 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1690, _, _, _, _, _, _) => {
                                h1690
                            }
                        };
                        let a1693 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1691, _, _, _, _, _, _) => {
                                h1691
                            }
                        };
                        nat_eq(a1692, a1693)
                    } else {
                        false
                    } {
                        let a1702 = match v3.clone() {
                            Adt2::C0(_, h1694, _, _, _, _, _, _, _, _, _) => {
                                h1694
                            }
                        };
                        let a1703 = {
                            let a1700 = {
                                let a1697 = match v2.clone() {
                                    Adt2::C0(_, h1695, _, _, _, _, _, _, _, _, _) => {
                                        h1695
                                    }
                                };
                                let a1698 = match v1.clone() {
                                    Adt1::C0(h1696, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1696
                                    }
                                };
                                nat_add(a1697, a1698)?
                            };
                            let a1701 = match v1.clone() {
                                Adt1::C0(_, h1699, _, _, _, _, _, _, _, _, _, _) => {
                                    h1699
                                }
                            };
                            nat_add(a1700, a1701)?
                        };
                        nat_eq(a1702, a1703)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1704, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1704
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1705 = v0;
                let a1706 = 63u64;
                if nat_eq(a1705, a1706) {
                    if if if if if f124(match v2.clone() {
                        Adt2::C0(_, h1707, _, _, _, _, _, _, _, _, _) => {
                            h1707
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1708, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1708
                        }
                    })? {
                        f124(match v2.clone() {
                            Adt2::C0(_, _, h1709, _, _, _, _, _, _, _, _) => {
                                h1709
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1710, _, _, _, _, _, _, _, _, _, _) => {
                                h1710
                            }
                        })?
                    } else {
                        false
                    } {
                        f124(match v2.clone() {
                            Adt2::C0(_, _, _, h1711, _, _, _, _, _, _, _) => {
                                h1711
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1712, _, _, _, _, _, _, _, _, _) => {
                                h1712
                            }
                        })?
                    } else {
                        false
                    } {
                        f124(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1713, _, _, _, _, _, _) => {
                                h1713
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1714, _, _, _, _, _, _, _, _) => {
                                h1714
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1717 = match v2.clone() {
                            Adt2::C0(_, h1715, _, _, _, _, _, _, _, _, _) => {
                                h1715
                            }
                        };
                        let a1718 = match v3.clone() {
                            Adt2::C0(_, h1716, _, _, _, _, _, _, _, _, _) => {
                                h1716
                            }
                        };
                        if if if nat_eq(a1717, a1718) {
                            let a1721 = match v2.clone() {
                                Adt2::C0(_, _, h1719, _, _, _, _, _, _, _, _) => {
                                    h1719
                                }
                            };
                            let a1722 = match v3.clone() {
                                Adt2::C0(_, _, h1720, _, _, _, _, _, _, _, _) => {
                                    h1720
                                }
                            };
                            nat_eq(a1721, a1722)
                        } else {
                            false
                        } {
                            let a1725 = match v2.clone() {
                                Adt2::C0(_, _, _, h1723, _, _, _, _, _, _, _) => {
                                    h1723
                                }
                            };
                            let a1726 = match v3.clone() {
                                Adt2::C0(_, _, _, h1724, _, _, _, _, _, _, _) => {
                                    h1724
                                }
                            };
                            nat_eq(a1725, a1726)
                        } else {
                            false
                        } {
                            let a1729 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1727, _, _, _, _, _, _) => {
                                    h1727
                                }
                            };
                            let a1730 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1728, _, _, _, _, _, _) => {
                                    h1728
                                }
                            };
                            nat_eq(a1729, a1730)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1731, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1731
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1732, _, _, _, _, _, _, _, _, _, _) => {
                                h1732
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1733, _, _, _, _, _, _, _, _, _) => {
                                h1733
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1734, _, _, _, _, _, _, _, _) => {
                                h1734
                            }
                        })))
                    } else {
                        Ok(Adt10::C1)
                    }
                } else {
                    Ok(Adt10::C0)
                }
            }
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f57(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1735 = v3;
    match m1735 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1736 = v0;
                let a1737 = 4u64;
                nat_eq(a1736, a1737)
            };
            let v7: bool = {
                let a1739 = 0u64;
                let a1740 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1738, _, _, _, _, _, _, _) => {
                        h1738
                    }
                };
                nat_lt(a1739, a1740)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1741, _, _, _, _, _, _, _, _, _) => {
                    h1741
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1742, _, _, _, _, _, _, _, _) => {
                    h1742
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1743, _, _, _, _, _, _, _) => {
                    h1743
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1744, _, _, _, _, _, _) => {
                    h1744
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1745, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1745
                }
            }, match v1.clone() {
                Adt1::C0(_, h1746, _, _, _, _, _, _, _, _, _, _) => {
                    h1746
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1747, _, _, _, _, _, _, _, _, _) => {
                    h1747
                }
            });
            if if if if if if if if if if if if v6 {
                let a1749 = match v2.clone() {
                    Adt2::C0(h1748, _, _, _, _, _, _, _, _, _, _) => {
                        h1748
                    }
                };
                let a1750 = 0u64;
                if nat_eq(a1749, a1750) {
                    let a1752 = match v5.clone() {
                        Adt2::C0(h1751, _, _, _, _, _, _, _, _, _, _) => {
                            h1751
                        }
                    };
                    let a1753 = 0u64;
                    nat_eq(a1752, a1753)
                } else {
                    false
                }
            } else {
                let a1755 = match v2.clone() {
                    Adt2::C0(h1754, _, _, _, _, _, _, _, _, _, _) => {
                        h1754
                    }
                };
                let a1756 = 0u64;
                if if nat_eq(a1755, a1756) {
                    true
                } else {
                    let a1758 = match v2.clone() {
                        Adt2::C0(h1757, _, _, _, _, _, _, _, _, _, _) => {
                            h1757
                        }
                    };
                    let a1759 = 26u64;
                    nat_eq(a1758, a1759)
                } {
                    let a1761 = match v5.clone() {
                        Adt2::C0(h1760, _, _, _, _, _, _, _, _, _, _) => {
                            h1760
                        }
                    };
                    let a1762 = 0u64;
                    if nat_eq(a1761, a1762) {
                        true
                    } else {
                        let a1764 = match v5.clone() {
                            Adt2::C0(h1763, _, _, _, _, _, _, _, _, _, _) => {
                                h1763
                            }
                        };
                        let a1765 = 26u64;
                        nat_eq(a1764, a1765)
                    }
                } else {
                    false
                }
            } {
                let a1768 = match v2.clone() {
                    Adt2::C0(h1766, _, _, _, _, _, _, _, _, _, _) => {
                        h1766
                    }
                };
                let a1769 = match v4.clone() {
                    Adt2::C0(h1767, _, _, _, _, _, _, _, _, _, _) => {
                        h1767
                    }
                };
                nat_eq(a1768, a1769)
            } else {
                false
            } {
                f25(v2.clone(), v4.clone())
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                f22(v4.clone())?
            } else {
                false
            } {
                let a1771 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1770, _, _, _, _, _) => {
                        h1770
                    }
                };
                let a1772 = 4u64;
                nat_eq(a1771, a1772)
            } else {
                false
            } {
                let a1773 = v8;
                let a1774 = f16(v2.clone(), 1u64)?;
                nat_le(a1773, a1774)
            } else {
                false
            } {
                let a1776 = match v1.clone() {
                    Adt1::C0(_, _, _, h1775, _, _, _, _, _, _, _, _) => {
                        h1775
                    }
                };
                let a1777 = 1073741824u64;
                nat_lt(a1776, a1777)
            } else {
                false
            } {
                let a1779 = match v1.clone() {
                    Adt1::C0(h1778, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1778
                    }
                };
                let a1780 = 4294967295u64;
                if if if if nat_le(a1779, a1780) {
                    let a1782 = match v1.clone() {
                        Adt1::C0(_, h1781, _, _, _, _, _, _, _, _, _, _) => {
                            h1781
                        }
                    };
                    let a1783 = 4294967295u64;
                    nat_le(a1782, a1783)
                } else {
                    false
                } {
                    let a1785 = match v1.clone() {
                        Adt1::C0(_, _, h1784, _, _, _, _, _, _, _, _, _) => {
                            h1784
                        }
                    };
                    let a1786 = 4294967295u64;
                    nat_le(a1785, a1786)
                } else {
                    false
                } {
                    let a1788 = match v1.clone() {
                        Adt1::C0(_, _, _, h1787, _, _, _, _, _, _, _, _) => {
                            h1787
                        }
                    };
                    let a1789 = 4294967295u64;
                    nat_le(a1788, a1789)
                } else {
                    false
                } {
                    let a1791 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1790, _, _, _, _, _, _, _) => {
                            h1790
                        }
                    };
                    let a1792 = 4294967295u64;
                    nat_le(a1791, a1792)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1795 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1793) => {
                            h1793
                        }
                    };
                    let a1796 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1794) => {
                            h1794
                        }
                    };
                    nat_eq(a1795, a1796)
                } else {
                    true
                }
            } else {
                false
            } {
                f125(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1797, _, _, _, _, _, _, _, _) => {
                        h1797
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1798, _) => {
                        h1798
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f16(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a1799 = v10;
                    let a1800 = v8;
                    nat_add(a1799, a1800)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1801, _, _, _, _, _, _, _, _) => {
                        h1801
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f58(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1803 = match v0.clone() {
        Adt2::C0(h1802, _, _, _, _, _, _, _, _, _, _) => {
            h1802
        }
    };
    let a1804 = 0u64;
    if if if nat_eq(a1803, a1804) {
        let a1806 = match v1.clone() {
            Adt2::C0(h1805, _, _, _, _, _, _, _, _, _, _) => {
                h1805
            }
        };
        let a1807 = 0u64;
        nat_eq(a1806, a1807)
    } else {
        false
    } {
        f121(match v0.clone() {
            Adt2::C0(_, h1808, _, _, _, _, _, _, _, _, _) => {
                h1808
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1809, _, _, _, _, _, _, _, _) => {
                h1809
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1810, _, _, _, _, _, _, _) => {
                h1810
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1811, _, _, _, _, _, _) => {
                h1811
            }
        }, match v1.clone() {
            Adt2::C0(_, h1812, _, _, _, _, _, _, _, _, _) => {
                h1812
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1813, _, _, _, _, _, _, _, _) => {
                h1813
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1814, _, _, _, _, _, _, _) => {
                h1814
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1815, _, _, _, _, _, _) => {
                h1815
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1816 = v2;
            let a1817 = f16(v0.clone(), 1u64)?;
            nat_add(a1816, a1817)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f59(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1819 = match v1.clone() {
        Adt2::C0(h1818, _, _, _, _, _, _, _, _, _, _) => {
            h1818
        }
    };
    let a1820 = 0u64;
    if if if nat_eq(a1819, a1820) {
        true
    } else {
        let a1822 = match v1.clone() {
            Adt2::C0(h1821, _, _, _, _, _, _, _, _, _, _) => {
                h1821
            }
        };
        let a1823 = 1u64;
        nat_eq(a1822, a1823)
    } {
        true
    } else {
        let a1825 = match v1.clone() {
            Adt2::C0(h1824, _, _, _, _, _, _, _, _, _, _) => {
                h1824
            }
        };
        let a1826 = 30u64;
        nat_eq(a1825, a1826)
    } {
        let a1829 = match v1.clone() {
            Adt2::C0(h1827, _, _, _, _, _, _, _, _, _, _) => {
                h1827
            }
        };
        let a1830 = match v2.clone() {
            Adt2::C0(h1828, _, _, _, _, _, _, _, _, _, _) => {
                h1828
            }
        };
        if if if if if nat_eq(a1829, a1830) {
            let a1833 = match v1.clone() {
                Adt2::C0(_, h1831, _, _, _, _, _, _, _, _, _) => {
                    h1831
                }
            };
            let a1834 = match v2.clone() {
                Adt2::C0(_, h1832, _, _, _, _, _, _, _, _, _) => {
                    h1832
                }
            };
            nat_eq(a1833, a1834)
        } else {
            false
        } {
            let a1837 = match v1.clone() {
                Adt2::C0(_, _, _, h1835, _, _, _, _, _, _, _) => {
                    h1835
                }
            };
            let a1838 = match v2.clone() {
                Adt2::C0(_, _, _, h1836, _, _, _, _, _, _, _) => {
                    h1836
                }
            };
            nat_eq(a1837, a1838)
        } else {
            false
        } {
            let a1841 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1839, _, _, _, _, _, _) => {
                    h1839
                }
            };
            let a1842 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1840, _, _, _, _, _, _) => {
                    h1840
                }
            };
            nat_eq(a1841, a1842)
        } else {
            false
        } {
            let a1843 = v0;
            let a1844 = 4u64;
            if nat_eq(a1843, a1844) {
                let a1846 = match v1.clone() {
                    Adt2::C0(_, _, h1845, _, _, _, _, _, _, _, _) => {
                        h1845
                    }
                };
                let a1847 = 1u64;
                if nat_eq(a1846, a1847) {
                    let a1850 = match v1.clone() {
                        Adt2::C0(_, h1848, _, _, _, _, _, _, _, _, _) => {
                            h1848
                        }
                    };
                    let a1851 = match v2.clone() {
                        Adt2::C0(_, _, h1849, _, _, _, _, _, _, _, _) => {
                            h1849
                        }
                    };
                    nat_eq(a1850, a1851)
                } else {
                    false
                }
            } else {
                let a1852 = v0;
                let a1853 = 4u64;
                if if nat_lt(a1852, a1853) {
                    let a1856 = match v1.clone() {
                        Adt2::C0(_, h1854, _, _, _, _, _, _, _, _, _) => {
                            h1854
                        }
                    };
                    let a1857 = match v1.clone() {
                        Adt2::C0(_, _, h1855, _, _, _, _, _, _, _, _) => {
                            h1855
                        }
                    };
                    nat_eq(a1856, a1857)
                } else {
                    false
                } {
                    let a1860 = match v1.clone() {
                        Adt2::C0(_, _, h1858, _, _, _, _, _, _, _, _) => {
                            h1858
                        }
                    };
                    let a1861 = match v2.clone() {
                        Adt2::C0(_, _, h1859, _, _, _, _, _, _, _, _) => {
                            h1859
                        }
                    };
                    nat_eq(a1860, a1861)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1863 = f118(match v1.clone() {
                Adt2::C0(h1862, _, _, _, _, _, _, _, _, _, _) => {
                    h1862
                }
            });
            match m1863 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1864) => {
                            h1864
                        }
                    }, f20(v1, 1u64), f20(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f60(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f126(match v1.clone() {
        Adt2::C0(h1865, _, _, _, _, _, _, _, _, _, _) => {
            h1865
        }
    }, v0) {
        let a1868 = match v1.clone() {
            Adt2::C0(h1866, _, _, _, _, _, _, _, _, _, _) => {
                h1866
            }
        };
        let a1869 = match v2.clone() {
            Adt2::C0(h1867, _, _, _, _, _, _, _, _, _, _) => {
                h1867
            }
        };
        nat_eq(a1868, a1869)
    } else {
        false
    } {
        let a1872 = match v1.clone() {
            Adt2::C0(_, h1870, _, _, _, _, _, _, _, _, _) => {
                h1870
            }
        };
        let a1873 = match v2.clone() {
            Adt2::C0(_, h1871, _, _, _, _, _, _, _, _, _) => {
                h1871
            }
        };
        if if if nat_eq(a1872, a1873) {
            let a1876 = match v1.clone() {
                Adt2::C0(_, _, h1874, _, _, _, _, _, _, _, _) => {
                    h1874
                }
            };
            let a1877 = match v2.clone() {
                Adt2::C0(_, _, h1875, _, _, _, _, _, _, _, _) => {
                    h1875
                }
            };
            nat_eq(a1876, a1877)
        } else {
            false
        } {
            let a1880 = match v1.clone() {
                Adt2::C0(_, _, _, h1878, _, _, _, _, _, _, _) => {
                    h1878
                }
            };
            let a1881 = match v2.clone() {
                Adt2::C0(_, _, _, h1879, _, _, _, _, _, _, _) => {
                    h1879
                }
            };
            nat_eq(a1880, a1881)
        } else {
            false
        } {
            let a1884 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1882, _, _, _, _, _, _) => {
                    h1882
                }
            };
            let a1885 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1883, _, _, _, _, _, _) => {
                    h1883
                }
            };
            nat_eq(a1884, a1885)
        } else {
            false
        }
    } else {
        false
    } {
        let m1887 = f118(match v1.clone() {
            Adt2::C0(h1886, _, _, _, _, _, _, _, _, _, _) => {
                h1886
            }
        });
        match m1887 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1888) => {
                        h1888
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1889, _, _, _, _, _, _, _, _, _, _) => {
                        h1889
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f61(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1890, _, _, _, _, _, _, _, _, _, _, _) => {
                h1890
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1891, _, _, _, _, _, _, _, _, _) => {
                h1891
            }
        }
    };
    let a1892 = 0u64;
    let a1893 = v4;
    if if nat_lt(a1892, a1893) {
        let a1894 = v4;
        let a1895 = 2147483648u64;
        nat_lt(a1894, a1895)
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
        let v7: u64 = f127(match v5.clone() {
            Adt2::C0(_, _, h1896, _, _, _, _, _, _, _, _) => {
                h1896
            }
        }, v4)?;
        let v8: u64 = f127(match v5.clone() {
            Adt2::C0(_, _, _, h1897, _, _, _, _, _, _, _) => {
                h1897
            }
        }, v4)?;
        let a1899 = match v5.clone() {
            Adt2::C0(h1898, _, _, _, _, _, _, _, _, _, _) => {
                h1898
            }
        };
        let a1900 = 0u64;
        if if if if if nat_eq(a1899, a1900) {
            let a1902 = match v6.clone() {
                Adt2::C0(h1901, _, _, _, _, _, _, _, _, _, _) => {
                    h1901
                }
            };
            let a1903 = 0u64;
            nat_eq(a1902, a1903)
        } else {
            false
        } {
            let a1905 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1904, _, _, _, _, _, _) => {
                    h1904
                }
            };
            let a1906 = 1u64;
            nat_eq(a1905, a1906)
        } else {
            false
        } {
            let a1909 = match v5.clone() {
                Adt2::C0(_, h1907, _, _, _, _, _, _, _, _, _) => {
                    h1907
                }
            };
            let a1910 = match v6.clone() {
                Adt2::C0(_, h1908, _, _, _, _, _, _, _, _, _) => {
                    h1908
                }
            };
            nat_eq(a1909, a1910)
        } else {
            false
        } {
            if v0 {
                let a1912 = match v5.clone() {
                    Adt2::C0(_, _, h1911, _, _, _, _, _, _, _, _) => {
                        h1911
                    }
                };
                let a1913 = 2147483648u64;
                if if if nat_lt(a1912, a1913) {
                    let a1915 = match v5.clone() {
                        Adt2::C0(_, _, _, h1914, _, _, _, _, _, _, _) => {
                            h1914
                        }
                    };
                    let a1916 = 2147483648u64;
                    nat_lt(a1915, a1916)
                } else {
                    false
                } {
                    let a1919 = {
                        let a1917 = v7;
                        let a1918 = v8;
                        nat_mul(a1917, a1918)?
                    };
                    let a1920 = 2147483648u64;
                    nat_le(a1919, a1920)
                } else {
                    false
                } {
                    f128(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1922 = match v1.clone() {
                    Adt1::C0(h1921, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1921
                    }
                };
                let a1923 = v7;
                if if if if nat_eq(a1922, a1923) {
                    let a1925 = match v1 {
                        Adt1::C0(_, h1924, _, _, _, _, _, _, _, _, _, _) => {
                            h1924
                        }
                    };
                    let a1926 = v8;
                    nat_eq(a1925, a1926)
                } else {
                    false
                } {
                    f129(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1928 = match v6.clone() {
                        Adt2::C0(_, _, h1927, _, _, _, _, _, _, _, _) => {
                            h1927
                        }
                    };
                    let a1929 = v4;
                    if nat_eq(a1928, a1929) {
                        let a1931 = match v6.clone() {
                            Adt2::C0(_, _, _, h1930, _, _, _, _, _, _, _) => {
                                h1930
                            }
                        };
                        let a1932 = v4;
                        nat_eq(a1931, a1932)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1936 = match v6 {
                        Adt2::C0(_, _, _, _, h1933, _, _, _, _, _, _) => {
                            h1933
                        }
                    };
                    let a1937 = {
                        let a1934 = v7;
                        let a1935 = v8;
                        nat_mul(a1934, a1935)?
                    };
                    nat_eq(a1936, a1937)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f62(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1939 = match v2.clone() {
        Adt2::C0(h1938, _, _, _, _, _, _, _, _, _, _) => {
            h1938
        }
    };
    let a1940 = 0u64;
    if if if if if nat_eq(a1939, a1940) {
        true
    } else {
        let a1942 = match v2.clone() {
            Adt2::C0(h1941, _, _, _, _, _, _, _, _, _, _) => {
                h1941
            }
        };
        let a1943 = 1u64;
        nat_eq(a1942, a1943)
    } {
        true
    } else {
        let a1945 = match v2.clone() {
            Adt2::C0(h1944, _, _, _, _, _, _, _, _, _, _) => {
                h1944
            }
        };
        let a1946 = 30u64;
        nat_eq(a1945, a1946)
    } {
        let a1949 = match v2.clone() {
            Adt2::C0(h1947, _, _, _, _, _, _, _, _, _, _) => {
                h1947
            }
        };
        let a1950 = match v3.clone() {
            Adt2::C0(h1948, _, _, _, _, _, _, _, _, _, _) => {
                h1948
            }
        };
        nat_eq(a1949, a1950)
    } else {
        false
    } {
        if if f25(v2.clone(), v3.clone()) {
            let a1952 = match v1.clone() {
                Adt1::C0(h1951, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1951
                }
            };
            let a1953 = 4294967296u64;
            nat_lt(a1952, a1953)
        } else {
            false
        } {
            let a1955 = match v1.clone() {
                Adt1::C0(_, h1954, _, _, _, _, _, _, _, _, _, _) => {
                    h1954
                }
            };
            let a1956 = 4294967296u64;
            nat_lt(a1955, a1956)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a1958 = match v2.clone() {
                Adt2::C0(h1957, _, _, _, _, _, _, _, _, _, _) => {
                    h1957
                }
            };
            let a1959 = 0u64;
            if nat_eq(a1958, a1959) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a1960 = v0;
            let a1961 = 31u64;
            if nat_eq(a1960, a1961) {
                0u64
            } else {
                let a1962 = v0;
                let a1963 = 47u64;
                if nat_eq(a1962, a1963) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1964, _, _, _, _, _, _, _, _, _, _) => {
                h1964
            }
        }, match v1.clone() {
            Adt1::C0(h1965, _, _, _, _, _, _, _, _, _, _, _) => {
                h1965
            }
        }, match v1 {
            Adt1::C0(_, h1966, _, _, _, _, _, _, _, _, _, _) => {
                h1966
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f63(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1968 = match v0.clone() {
        Adt2::C0(h1967, _, _, _, _, _, _, _, _, _, _) => {
            h1967
        }
    };
    let a1969 = 0u64;
    if if if nat_eq(a1968, a1969) {
        let a1971 = match v1.clone() {
            Adt2::C0(h1970, _, _, _, _, _, _, _, _, _, _) => {
                h1970
            }
        };
        let a1972 = 0u64;
        nat_eq(a1971, a1972)
    } else {
        false
    } {
        f25(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v1.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f64(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1974 = match v0.clone() {
        Adt2::C0(h1973, _, _, _, _, _, _, _, _, _, _) => {
            h1973
        }
    };
    let a1975 = 0u64;
    if if nat_eq(a1974, a1975) {
        let a1977 = match v1.clone() {
            Adt2::C0(h1976, _, _, _, _, _, _, _, _, _, _) => {
                h1976
            }
        };
        let a1978 = 26u64;
        nat_eq(a1977, a1978)
    } else {
        false
    } {
        let a1980 = match v0.clone() {
            Adt2::C0(_, _, _, h1979, _, _, _, _, _, _, _) => {
                h1979
            }
        };
        let a1981 = 1u64;
        if if if if if if if nat_eq(a1980, a1981) {
            let a1983 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1982, _, _, _, _, _, _) => {
                    h1982
                }
            };
            let a1984 = 1u64;
            nat_eq(a1983, a1984)
        } else {
            false
        } {
            let a1986 = match v0.clone() {
                Adt2::C0(_, h1985, _, _, _, _, _, _, _, _, _) => {
                    h1985
                }
            };
            let a1987 = 2147483647u64;
            nat_le(a1986, a1987)
        } else {
            false
        } {
            let a1990 = match v1.clone() {
                Adt2::C0(_, h1988, _, _, _, _, _, _, _, _, _) => {
                    h1988
                }
            };
            let a1991 = match v0.clone() {
                Adt2::C0(_, _, h1989, _, _, _, _, _, _, _, _) => {
                    h1989
                }
            };
            nat_eq(a1990, a1991)
        } else {
            false
        } {
            let a1993 = match v1.clone() {
                Adt2::C0(_, _, h1992, _, _, _, _, _, _, _, _) => {
                    h1992
                }
            };
            let a1994 = 1u64;
            nat_eq(a1993, a1994)
        } else {
            false
        } {
            let a1996 = match v1.clone() {
                Adt2::C0(_, _, _, h1995, _, _, _, _, _, _, _) => {
                    h1995
                }
            };
            let a1997 = 1u64;
            nat_eq(a1996, a1997)
        } else {
            false
        } {
            let a1999 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1998, _, _, _, _, _, _) => {
                    h1998
                }
            };
            let a2000 = 1u64;
            nat_eq(a1999, a2000)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a2003 = match v0.clone() {
                    Adt2::C0(_, _, h2001, _, _, _, _, _, _, _, _) => {
                        h2001
                    }
                };
                let a2004 = f130(match v0.clone() {
                    Adt2::C0(_, h2002, _, _, _, _, _, _, _, _, _) => {
                        h2002
                    }
                })?;
                nat_mul(a2003, a2004)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f65(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2005 = v1;
    match m2005 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f131(f20(v0.clone(), 1u64))?;
            let a2007 = match v0.clone() {
                Adt2::C0(h2006, _, _, _, _, _, _, _, _, _, _) => {
                    h2006
                }
            };
            let a2008 = 26u64;
            if if if nat_eq(a2007, a2008) {
                let a2010 = match v3.clone() {
                    Adt2::C0(h2009, _, _, _, _, _, _, _, _, _, _) => {
                        h2009
                    }
                };
                let a2011 = 26u64;
                nat_eq(a2010, a2011)
            } else {
                false
            } {
                let a2013 = match v2.clone() {
                    Adt2::C0(h2012, _, _, _, _, _, _, _, _, _, _) => {
                        h2012
                    }
                };
                let a2014 = 27u64;
                nat_eq(a2013, a2014)
            } else {
                false
            } {
                if if if if if if f25(v0.clone(), v3.clone()) {
                    let a2016 = match v2.clone() {
                        Adt2::C0(_, h2015, _, _, _, _, _, _, _, _, _) => {
                            h2015
                        }
                    };
                    let a2017 = 1u64;
                    nat_eq(a2016, a2017)
                } else {
                    false
                } {
                    let a2019 = match v2.clone() {
                        Adt2::C0(_, _, h2018, _, _, _, _, _, _, _, _) => {
                            h2018
                        }
                    };
                    let a2020 = 1u64;
                    nat_eq(a2019, a2020)
                } else {
                    false
                } {
                    let a2022 = match v2.clone() {
                        Adt2::C0(_, _, _, h2021, _, _, _, _, _, _, _) => {
                            h2021
                        }
                    };
                    let a2023 = 1u64;
                    nat_eq(a2022, a2023)
                } else {
                    false
                } {
                    let a2025 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2024, _, _, _, _, _, _) => {
                            h2024
                        }
                    };
                    let a2026 = 1u64;
                    nat_eq(a2025, a2026)
                } else {
                    false
                } {
                    let a2027 = v4;
                    let a2028 = 9223372036854775808u64;
                    nat_lt(a2027, a2028)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2031 = 2u64;
                        let a2032 = {
                            let a2029 = v4;
                            let a2030 = 2u64;
                            nat_mul(a2029, a2030)?
                        };
                        nat_add(a2031, a2032)?
                    }, 8u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C18(v4, f20(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f66(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h2033, _, _, _, _, _, _, _, _, _, _) => {
            h2033
        }
    };
    let a2034 = v3;
    let a2035 = 0u64;
    if if nat_eq(a2034, a2035) {
        true
    } else {
        let a2036 = v0;
        let a2037 = 13u64;
        if nat_eq(a2036, a2037) {
            let a2038 = v3;
            let a2039 = 1u64;
            if nat_eq(a2038, a2039) {
                true
            } else {
                let a2040 = v3;
                let a2041 = 30u64;
                nat_eq(a2040, a2041)
            }
        } else {
            false
        }
    } {
        let a2043 = v3;
        let a2044 = match v2.clone() {
            Adt2::C0(h2042, _, _, _, _, _, _, _, _, _, _) => {
                h2042
            }
        };
        if if nat_eq(a2043, a2044) {
            let a2045 = v0;
            let a2046 = 13u64;
            if nat_eq(a2045, a2046) {
                let a2048 = match v2.clone() {
                    Adt2::C0(_, h2047, _, _, _, _, _, _, _, _, _) => {
                        h2047
                    }
                };
                let a2049 = 1u64;
                if if if nat_eq(a2048, a2049) {
                    let a2051 = match v2.clone() {
                        Adt2::C0(_, _, h2050, _, _, _, _, _, _, _, _) => {
                            h2050
                        }
                    };
                    let a2052 = 1u64;
                    nat_eq(a2051, a2052)
                } else {
                    false
                } {
                    let a2054 = match v2.clone() {
                        Adt2::C0(_, _, _, h2053, _, _, _, _, _, _, _) => {
                            h2053
                        }
                    };
                    let a2055 = 1u64;
                    nat_eq(a2054, a2055)
                } else {
                    false
                } {
                    let a2057 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2056, _, _, _, _, _, _) => {
                            h2056
                        }
                    };
                    let a2058 = 1u64;
                    nat_eq(a2057, a2058)
                } else {
                    false
                }
            } else {
                let a2060 = match v2.clone() {
                    Adt2::C0(_, h2059, _, _, _, _, _, _, _, _, _) => {
                        h2059
                    }
                };
                let a2061 = 1u64;
                if if if nat_eq(a2060, a2061) {
                    let a2064 = match v1.clone() {
                        Adt2::C0(_, _, h2062, _, _, _, _, _, _, _, _) => {
                            h2062
                        }
                    };
                    let a2065 = match v2.clone() {
                        Adt2::C0(_, _, h2063, _, _, _, _, _, _, _, _) => {
                            h2063
                        }
                    };
                    nat_eq(a2064, a2065)
                } else {
                    false
                } {
                    let a2068 = match v1.clone() {
                        Adt2::C0(_, _, _, h2066, _, _, _, _, _, _, _) => {
                            h2066
                        }
                    };
                    let a2069 = match v2.clone() {
                        Adt2::C0(_, _, _, h2067, _, _, _, _, _, _, _) => {
                            h2067
                        }
                    };
                    nat_eq(a2068, a2069)
                } else {
                    false
                } {
                    let a2072 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2070, _, _, _, _, _, _) => {
                            h2070
                        }
                    };
                    let a2073 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2071, _, _, _, _, _, _) => {
                            h2071
                        }
                    };
                    nat_eq(a2072, a2073)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f132(v4.clone(), v0, v3)?, {
                let a2074 = v3;
                let a2075 = 0u64;
                if nat_eq(a2074, a2075) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f20(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f67(v0: u64, v1: u64) -> R<Option<u64>> {
    let a2076 = v0;
    let a2077 = 89u64;
    if if nat_eq(a2076, a2077) {
        let a2078 = v1;
        let a2079 = 14u64;
        nat_eq(a2078, a2079)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a2080 = v0;
        let a2081 = 89u64;
        if if nat_eq(a2080, a2081) {
            let a2082 = v1;
            let a2083 = 15u64;
            nat_eq(a2082, a2083)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a2084 = v0;
            let a2085 = 89u64;
            if if nat_eq(a2084, a2085) {
                let a2086 = 8u64;
                let a2087 = v1;
                if nat_le(a2086, a2087) {
                    let a2088 = v1;
                    let a2089 = 9u64;
                    nat_le(a2088, a2089)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a2090 = v1;
                    let a2091 = 12u64;
                    nat_add(a2090, a2091)?
                }))
            } else {
                let a2092 = v0;
                let a2093 = 89u64;
                if if nat_eq(a2092, a2093) {
                    let a2094 = v1;
                    let a2095 = 4u64;
                    nat_eq(a2094, a2095)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a2096 = 10u64;
                    let a2097 = v0;
                    if if nat_le(a2096, a2097) {
                        let a2098 = v0;
                        let a2099 = 12u64;
                        nat_le(a2098, a2099)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a2100 = v0;
                            let a2101 = 3u64;
                            nat_add(a2100, a2101)?
                        }))
                    } else {
                        let a2102 = v0;
                        let a2103 = 89u64;
                        if if nat_eq(a2102, a2103) {
                            let a2104 = v1;
                            let a2105 = 7u64;
                            nat_eq(a2104, a2105)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a2106 = v0;
                            let a2107 = 89u64;
                            if if nat_eq(a2106, a2107) {
                                let a2108 = v1;
                                let a2109 = 10u64;
                                nat_eq(a2108, a2109)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a2110 = v0;
                                let a2111 = 89u64;
                                if if nat_eq(a2110, a2111) {
                                    let a2112 = v1;
                                    let a2113 = 13u64;
                                    nat_eq(a2112, a2113)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a2114 = v0;
                                    let a2115 = 8u64;
                                    if nat_eq(a2114, a2115) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a2116 = v0;
                                        let a2117 = 9u64;
                                        if nat_eq(a2116, a2117) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a2118 = v0;
                                            let a2119 = 89u64;
                                            if nat_eq(a2118, a2119) {
                                                let a2120 = v1;
                                                let a2121 = 0u64;
                                                if nat_eq(a2120, a2121) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a2122 = v1;
                                                    let a2123 = 1u64;
                                                    if nat_eq(a2122, a2123) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a2124 = v1;
                                                        let a2125 = 2u64;
                                                        if nat_eq(a2124, a2125) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a2126 = v1;
                                                            let a2127 = 3u64;
                                                            if nat_eq(a2126, a2127) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a2128 = v1;
                                                                let a2129 = 6u64;
                                                                if nat_eq(a2128, a2129) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a2130 = v1;
                                                                    let a2131 = 12u64;
                                                                    if nat_eq(a2130, a2131) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a2132 = v1;
                                                                        let a2133 = 11u64;
                                                                        if nat_eq(a2132, a2133) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a2134 = v1;
                                                                            let a2135 = 18u64;
                                                                            if nat_eq(a2134, a2135) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a2136 = v1;
                                                                                let a2137 = 19u64;
                                                                                if nat_eq(a2136, a2137) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a2138 = v1;
                                                                                    let a2139 = 20u64;
                                                                                    if nat_eq(a2138, a2139) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a2140 = v1;
                                                                                        let a2141 = 21u64;
                                                                                        if nat_eq(a2140, a2141) {
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
    }
}

pub fn f68(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2142 = f133(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m2142 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a2143 = v0;
            let a2144 = 2u64;
            if if if if nat_eq(a2143, a2144) {
                true
            } else {
                let a2145 = v0;
                let a2146 = 5u64;
                nat_eq(a2145, a2146)
            } {
                true
            } else {
                let a2147 = v0;
                let a2148 = 6u64;
                nat_eq(a2147, a2148)
            } {
                true
            } else {
                let a2149 = v0;
                let a2150 = 7u64;
                nat_eq(a2149, a2150)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2151 = v3;
                    match m2151 {
                        None => {
                            f20(v2, 1u64)
                        }
                        Some(v14) => {
                            f20(v14, 1u64)
                        }
                    }
                })))
            } else {
                Ok(Adt10::C5(67u64))
            }
        }
        Adt10::C3(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24) => {
            Ok(Adt10::C3(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24))
        }
        Adt10::C4(v25, v26, v27, v28, v29) => {
            Ok(Adt10::C4(v25, v26, v27, v28, v29))
        }
        Adt10::C5(v6) => {
            Ok(Adt10::C5(v6))
        }
        Adt10::C6(v5) => {
            Ok(Adt10::C6(v5))
        }
    }
}

pub fn f69(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a2153 = match v1.clone() {
        Adt2::C0(h2152, _, _, _, _, _, _, _, _, _, _) => {
            h2152
        }
    };
    let a2154 = 0u64;
    if if if if if nat_eq(a2153, a2154) {
        true
    } else {
        let a2156 = match v1.clone() {
            Adt2::C0(h2155, _, _, _, _, _, _, _, _, _, _) => {
                h2155
            }
        };
        let a2157 = 1u64;
        nat_eq(a2156, a2157)
    } {
        true
    } else {
        let a2159 = match v1.clone() {
            Adt2::C0(h2158, _, _, _, _, _, _, _, _, _, _) => {
                h2158
            }
        };
        let a2160 = 30u64;
        nat_eq(a2159, a2160)
    } {
        let a2163 = match v1.clone() {
            Adt2::C0(h2161, _, _, _, _, _, _, _, _, _, _) => {
                h2161
            }
        };
        let a2164 = match v2.clone() {
            Adt2::C0(h2162, _, _, _, _, _, _, _, _, _, _) => {
                h2162
            }
        };
        if nat_eq(a2163, a2164) {
            true
        } else {
            let a2166 = match v2.clone() {
                Adt2::C0(h2165, _, _, _, _, _, _, _, _, _, _) => {
                    h2165
                }
            };
            let a2167 = 0u64;
            nat_eq(a2166, a2167)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2169 = f16(v2.clone(), 1u64)?;
            let a2170 = f134(v0, match v2.clone() {
                Adt2::C0(h2168, _, _, _, _, _, _, _, _, _, _) => {
                    h2168
                }
            });
            nat_mul(a2169, a2170)?
        }, {
            let a2172 = match v2.clone() {
                Adt2::C0(h2171, _, _, _, _, _, _, _, _, _, _) => {
                    h2171
                }
            };
            let a2173 = 0u64;
            if nat_eq(a2172, a2173) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2174, _, _, _, _, _, _, _, _, _, _) => {
                h2174
            }
        }, match v2 {
            Adt2::C0(h2175, _, _, _, _, _, _, _, _, _, _) => {
                h2175
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f70(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f135(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2176, _, _, _) => {
            h2176
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2177, _, _) => {
            h2177
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2178, _) => {
            h2178
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2179) => {
            h2179
        }
    })
}

pub fn f71(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2181 = v0;
        let a2182 = match v1.clone() {
            Adt3::C0(h2180, _, _, _, _, _, _, _) => {
                h2180
            }
        };
        let a2183 = 0u64;
        nat_quot(a2181, a2182, a2183)
    };
    let v4: u64 = {
        let a2185 = v3;
        let a2186 = match v1.clone() {
            Adt3::C0(_, h2184, _, _, _, _, _, _) => {
                h2184
            }
        };
        let a2187 = 0u64;
        nat_quot(a2185, a2186, a2187)
    };
    let a2188 = v2;
    let a2189 = 0u64;
    if nat_eq(a2188, a2189) {
        let a2191 = v0;
        let a2192 = match v1 {
            Adt3::C0(h2190, _, _, _, _, _, _, _) => {
                h2190
            }
        };
        let a2193 = 0u64;
        nat_rem(a2191, a2192, a2193)
    } else {
        let a2194 = v2;
        let a2195 = 1u64;
        if nat_eq(a2194, a2195) {
            let a2197 = v3;
            let a2198 = match v1 {
                Adt3::C0(_, h2196, _, _, _, _, _, _) => {
                    h2196
                }
            };
            let a2199 = 0u64;
            nat_rem(a2197, a2198, a2199)
        } else {
            let a2200 = v2;
            let a2201 = 2u64;
            if nat_eq(a2200, a2201) {
                let a2203 = v4;
                let a2204 = match v1 {
                    Adt3::C0(_, _, h2202, _, _, _, _, _) => {
                        h2202
                    }
                };
                let a2205 = 0u64;
                nat_rem(a2203, a2204, a2205)
            } else {
                let a2207 = v4;
                let a2208 = match v1 {
                    Adt3::C0(_, _, h2206, _, _, _, _, _) => {
                        h2206
                    }
                };
                let a2209 = 0u64;
                nat_quot(a2207, a2208, a2209)
            }
        }
    }
}

pub fn f72(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2211 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2210) => {
            h2210
        }
    };
    let a2212 = 0u64;
    if nat_eq(a2211, a2212) {
        Ok(v2)
    } else {
        let a2217 = {
            let a2214 = v2;
            let a2215 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2213) => {
                    h2213
                }
            };
            nat_mul(a2214, a2215)?
        };
        let a2218 = f71(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2216, _, _, _, _, _, _) => {
                h2216
            }
        }, 3u64);
        nat_add(a2217, a2218)
    }
}

pub fn f73(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2228 = {
            let a2225 = {
                let a2220 = v2;
                let a2221 = match v0.clone() {
                    Adt6::C0(_, h2219, _, _) => {
                        h2219
                    }
                };
                nat_mul(a2220, a2221)?
            };
            let a2226 = {
                let a2223 = v3;
                let a2224 = match v0.clone() {
                    Adt6::C0(_, _, _, h2222) => {
                        h2222
                    }
                };
                nat_mul(a2223, a2224)?
            };
            nat_add(a2225, a2226)?
        };
        let a2229 = f136(match v0.clone() {
            Adt6::C0(_, _, h2227, _) => {
                h2227
            }
        });
        nat_add(a2228, a2229)?
    };
    let v5: u64 = f137(match v0 {
        Adt6::C0(_, _, h2230, _) => {
            h2230
        }
    });
    let a2231 = v5;
    let a2232 = v4;
    if if nat_le(a2231, a2232) {
        let a2235 = {
            let a2233 = v4;
            let a2234 = v5;
            nat_sub(a2233, a2234)
        };
        let a2236 = v1;
        nat_lt(a2235, a2236)
    } else {
        false
    } {
        Ok(Some({
            let a2237 = v4;
            let a2238 = v5;
            nat_sub(a2237, a2238)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f74(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2239, _, _, _, _, _) => {
            h2239
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2240, _, _, _, _, _, _) => {
            h2240
        }
    };
    let v7: u64 = f71(v1, v6.clone(), 2u64);
    let a2242 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2241) => {
            h2241
        }
    };
    let a2243 = 0u64;
    if nat_eq(a2242, a2243) {
        Ok(Adt9::C8(Some(f70(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2246 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2245, _, _, _, _) => {
                    h2245
                }
            };
            match m2246 {
                Adt3::C0(_, _, h2244, _, _, _, _, _) => {
                    h2244
                }
            }
        };
        let m2252 = f73(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2247, _) => {
                h2247
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2248, _, _, _, _, _) => {
                h2248
            }
        }, {
            let a2249 = v2;
            let a2250 = v8;
            let a2251 = 0u64;
            nat_rem(a2249, a2250, a2251)
        }, v7)?;
        match m2252 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2259 = {
                        let a2257 = {
                            let a2253 = v2;
                            let a2254 = v8;
                            let a2255 = 0u64;
                            nat_quot(a2253, a2254, a2255)
                        };
                        let a2258 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2256) => {
                                h2256
                            }
                        };
                        nat_mul(a2257, a2258)?
                    };
                    let a2260 = f71(v1, v6, 3u64);
                    nat_add(a2259, a2260)?
                };
                Ok(Adt9::C8(Some(f70(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f75(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2261 = v1;
    match m2261 {
        Adt9::C0(v10) => {
            Adt9::C0(v10)
        }
        Adt9::C1(v11, v12, v13) => {
            Adt9::C1(v11, v12, v13)
        }
        Adt9::C2(v14, v15, v16, v17, v18, v19) => {
            Adt9::C2(v14, v15, v16, v17, v18, v19)
        }
        Adt9::C3(v20, v21, v22, v23) => {
            f138(f139(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f140(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f141(v0.clone(), v24);
            f138(f142(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
        }
        Adt9::C5(v32, v33, v34, v35, v36, v37) => {
            Adt9::C5(v32, v33, v34, v35, v36, v37)
        }
        Adt9::C6(v38, v39, v40, v41) => {
            Adt9::C6(v38, v39, v40, v41)
        }
        Adt9::C7(v42, v43, v44, v45, v46, v47) => {
            Adt9::C7(v42, v43, v44, v45, v46, v47)
        }
        Adt9::C8(v2) => {
            Adt9::C8(v2)
        }
        Adt9::C9(v3) => {
            Adt9::C9(v3)
        }
        Adt9::C10(v4, v5, v6, v7, v8, v9) => {
            Adt9::C10(v4, v5, v6, v7, v8, v9)
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m2262 = v3;
    match m2262 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f143(v0, v1)?, f143(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f145(v0, v1, v2.clone())?, f143(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f146(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f147(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f148(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f147(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f149(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f150(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f151(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f143(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f152(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v31, v32) => {
            f153(v0, v1, v2, v31, v32)
        }
        Adt4::C12(v33, v34, v35) => {
            f154(v0, v1, v2, v33, v34, v35)
        }
        Adt4::C13(_, v37, _, _) => {
            Ok(Adt9::C3(v37, v37, f143(v0, v1)?, f143(v0, v2)?))
        }
        Adt4::C14(v40, v41, v42) => {
            let a2265 = v40;
            let a2266 = 23u64;
            if nat_eq(a2265, a2266) {
                f155(v0, v1, v2, v41, v42)
            } else {
                Ok(Adt9::C3(v41, v42, f143(v0, v1)?, f143(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f156(v0, v1, v2)
        }
        Adt4::C16(v43, v44) => {
            f157(v0, v1, v2, v43, v44)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v45, v46) => {
            f158(v0, v1, v46, v45)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f143(f144(v0, match v2.clone() {
                Adt3::C0(h2263, _, _, _, _, _, _, _) => {
                    h2263
                }
            }, match v2.clone() {
                Adt3::C0(_, h2264, _, _, _, _, _, _) => {
                    h2264
                }
            })?, v1)?, f143(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt9::C0(Adt10::C5(65u64)))
        }
        Adt4::C25(v51, v52, v53) => {
            f159(v0, v51, v52, v53)
        }
        Adt4::C26(v59, _, _, _) => {
            let a2267 = v59;
            let a2268 = 64u64;
            if if if nat_eq(a2267, a2268) {
                true
            } else {
                let a2269 = v59;
                let a2270 = 65u64;
                nat_eq(a2269, a2270)
            } {
                true
            } else {
                let a2271 = v59;
                let a2272 = 66u64;
                nat_eq(a2271, a2272)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v58) => {
            f86(v0, v1, v2, v58)
        }
        Adt4::C28(v54) => {
            f83(v0, v1, v2, v54, 0u64)
        }
        Adt4::C29(v55) => {
            f84(v0, v1, v2, v55)
        }
        Adt4::C30(v56, _) => {
            f85(v0, v1, v2, v56)
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C32(_) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f120(v3, v4);
    let v6: u64 = {
        let a2273 = v0;
        let a2274 = v5;
        let a2275 = 0u64;
        nat_quot(a2273, a2274, a2275)
    };
    let a2276 = v5;
    let a2277 = 1u64;
    if nat_eq(a2276, a2277) {
        Ok(f138(f160(v3, v4), v3, v4, 0u64, f143(v6, v1)?, 2u64, 0u64, 3u64, f143(v6, v2)?))
    } else {
        let a2281 = {
            let a2278 = v0;
            let a2279 = 2u64;
            let a2280 = 0u64;
            nat_rem(a2278, a2279, a2280)
        };
        let a2282 = 0u64;
        if nat_eq(a2281, a2282) {
            Ok(f138(f160(v3, 0u64), v3, 0u64, 0u64, f143(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f138(f160(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f143(v6, v2)?))
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f107(v1)?;
    let v4: u64 = {
        let a2283 = v0;
        let a2284 = v3;
        let a2285 = 0u64;
        nat_rem(a2283, a2284, a2285)
    };
    let v5: u64 = {
        let a2288 = {
            let a2286 = v1;
            let a2287 = 3u64;
            nat_mul(a2286, a2287)?
        };
        let a2289 = 1u64;
        nat_add(a2288, a2289)?
    };
    let a2290 = v4;
    let a2291 = 0u64;
    if nat_eq(a2290, a2291) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a2292 = v4;
        let a2293 = v5;
        if nat_eq(a2292, a2293) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a2298 = v4;
                let a2299 = {
                    let a2294 = v4;
                    let a2295 = v5;
                    if nat_lt(a2294, a2295) {
                        1u64
                    } else {
                        let a2296 = v5;
                        let a2297 = 1u64;
                        nat_add(a2296, a2297)?
                    }
                };
                nat_sub(a2298, a2299)
            };
            let v7: u64 = {
                let a2300 = v6;
                let a2301 = 3u64;
                let a2302 = 0u64;
                nat_rem(a2300, a2301, a2302)
            };
            let v8: u64 = {
                let a2313 = 4u64;
                let a2314 = {
                    let a2311 = {
                        let a2306 = {
                            let a2303 = v0;
                            let a2304 = v3;
                            let a2305 = 0u64;
                            nat_quot(a2303, a2304, a2305)
                        };
                        let a2307 = v1;
                        nat_mul(a2306, a2307)?
                    };
                    let a2312 = {
                        let a2308 = v6;
                        let a2309 = 3u64;
                        let a2310 = 0u64;
                        nat_quot(a2308, a2309, a2310)
                    };
                    nat_add(a2311, a2312)?
                };
                nat_mul(a2313, a2314)?
            };
            let a2315 = v4;
            let a2316 = v5;
            if nat_lt(a2315, a2316) {
                let a2317 = v7;
                let a2318 = 0u64;
                if nat_eq(a2317, a2318) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2319 = v7;
                    let a2320 = 1u64;
                    if nat_eq(a2319, a2320) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a2321 = v7;
                let a2322 = 0u64;
                if nat_eq(a2321, a2322) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2323 = v7;
                    let a2324 = 1u64;
                    if nat_eq(a2323, a2324) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f130(match v1.clone() {
        Adt3::C0(h2325, _, _, _, _, _, _, _) => {
            h2325
        }
    })?;
    let v5: u64 = {
        let a2326 = v0;
        let a2327 = v4;
        let a2328 = 0u64;
        nat_quot(a2326, a2327, a2328)
    };
    let v6: u64 = {
        let a2329 = v0;
        let a2330 = v4;
        let a2331 = 0u64;
        nat_rem(a2329, a2330, a2331)
    };
    let a2332 = v6;
    let a2333 = 0u64;
    if nat_eq(a2332, a2333) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2334 = v6;
        let a2335 = 1u64;
        if nat_eq(a2334, a2335) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2338 = {
                let a2336 = v6;
                let a2337 = 1u64;
                nat_add(a2336, a2337)?
            };
            let a2339 = v4;
            if nat_eq(a2338, a2339) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f143(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2340 = v6;
                    let a2341 = 2u64;
                    nat_sub(a2340, a2341)
                };
                let v8: u64 = {
                    let a2342 = v7;
                    let a2343 = 3u64;
                    let a2344 = 0u64;
                    nat_quot(a2342, a2343, a2344)
                };
                let v9: u64 = {
                    let a2345 = v7;
                    let a2346 = 3u64;
                    let a2347 = 0u64;
                    nat_rem(a2345, a2346, a2347)
                };
                let v10: u64 = f143({
                    let a2351 = {
                        let a2349 = v5;
                        let a2350 = match v1.clone() {
                            Adt3::C0(h2348, _, _, _, _, _, _, _) => {
                                h2348
                            }
                        };
                        nat_mul(a2349, a2350)?
                    };
                    let a2352 = v8;
                    nat_add(a2351, a2352)?
                }, v1)?;
                let a2353 = v9;
                let a2354 = 0u64;
                if nat_eq(a2353, a2354) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2355 = v9;
                    let a2356 = 1u64;
                    if nat_eq(a2355, a2356) {
                        let a2357 = v3;
                        let a2358 = 2u64;
                        if nat_eq(a2357, a2358) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2359 = v3;
                        let a2360 = 2u64;
                        if if nat_eq(a2359, a2360) {
                            true
                        } else {
                            let a2361 = v3;
                            let a2362 = 3u64;
                            nat_eq(a2361, a2362)
                        } {
                            Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(19u64, 2u64, v8, 2u64, 0u64, 1u64, 1u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2364 = 2u64;
        let a2365 = match v2.clone() {
            Adt3::C0(_, h2363, _, _, _, _, _, _) => {
                h2363
            }
        };
        nat_mul(a2364, a2365)?
    };
    let a2366 = v1;
    let a2367 = v6;
    if nat_lt(a2366, a2367) {
        let a2371 = {
            let a2368 = v1;
            let a2369 = 2u64;
            let a2370 = 0u64;
            nat_rem(a2368, a2369, a2370)
        };
        let a2372 = 0u64;
        if nat_eq(a2371, a2372) {
            Ok(Adt9::C4(19u64, 4u64, f143({
                let a2373 = v1;
                let a2374 = 2u64;
                let a2375 = 0u64;
                nat_quot(a2373, a2374, a2375)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2376 = v5;
            let a2377 = 2147483648u64;
            if if nat_lt(a2376, a2377) {
                let a2379 = v5;
                let a2380 = match v3 {
                    Adt3::C0(_, h2378, _, _, _, _, _, _) => {
                        h2378
                    }
                };
                nat_lt(a2379, a2380)
            } else {
                false
            } {
                Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v7: u64 = {
            let a2383 = match v3.clone() {
                Adt3::C0(h2381, _, _, _, _, _, _, _) => {
                    h2381
                }
            };
            let a2384 = match v3.clone() {
                Adt3::C0(_, h2382, _, _, _, _, _, _) => {
                    h2382
                }
            };
            nat_mul(a2383, a2384)?
        };
        let a2387 = {
            let a2385 = v1;
            let a2386 = v6;
            nat_sub(a2385, a2386)
        };
        let a2388 = v7;
        if nat_lt(a2387, a2388) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f143({
                let a2389 = v1;
                let a2390 = v6;
                nat_sub(a2389, a2390)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2393 = {
                    let a2391 = v1;
                    let a2392 = v6;
                    nat_sub(a2391, a2392)
                };
                let a2394 = v7;
                nat_sub(a2393, a2394)
            };
            let v9: u64 = {
                let a2396 = 1u64;
                let a2397 = f161(v0, match v2.clone() {
                    Adt3::C0(h2395, _, _, _, _, _, _, _) => {
                        h2395
                    }
                })?;
                nat_add(a2396, a2397)?
            };
            let v10: u64 = {
                let a2398 = v8;
                let a2399 = v9;
                let a2400 = 0u64;
                nat_quot(a2398, a2399, a2400)
            };
            let v11: u64 = {
                let a2401 = v8;
                let a2402 = v9;
                let a2403 = 0u64;
                nat_rem(a2401, a2402, a2403)
            };
            let a2404 = v11;
            let a2405 = 0u64;
            if nat_eq(a2404, a2405) {
                Ok(Adt9::C4(19u64, 4u64, f143(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f108(v0) {
                    let a2409 = {
                        let a2406 = v11;
                        let a2407 = 2u64;
                        let a2408 = 0u64;
                        nat_rem(a2406, a2407, a2408)
                    };
                    let a2410 = 1u64;
                    if nat_eq(a2409, a2410) {
                        Ok(Adt9::C4(6u64, 0u64, f143({
                            let a2419 = {
                                let a2412 = v10;
                                let a2413 = match v2.clone() {
                                    Adt3::C0(h2411, _, _, _, _, _, _, _) => {
                                        h2411
                                    }
                                };
                                nat_mul(a2412, a2413)?
                            };
                            let a2420 = {
                                let a2416 = {
                                    let a2414 = v11;
                                    let a2415 = 1u64;
                                    nat_sub(a2414, a2415)
                                };
                                let a2417 = 2u64;
                                let a2418 = 0u64;
                                nat_quot(a2416, a2417, a2418)
                            };
                            nat_add(a2419, a2420)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f143({
                            let a2429 = {
                                let a2422 = v5;
                                let a2423 = match v2.clone() {
                                    Adt3::C0(h2421, _, _, _, _, _, _, _) => {
                                        h2421
                                    }
                                };
                                nat_mul(a2422, a2423)?
                            };
                            let a2430 = {
                                let a2426 = {
                                    let a2424 = v11;
                                    let a2425 = 1u64;
                                    nat_sub(a2424, a2425)
                                };
                                let a2427 = 2u64;
                                let a2428 = 0u64;
                                nat_quot(a2426, a2427, a2428)
                            };
                            nat_add(a2429, a2430)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f143({
                            let a2439 = {
                                let a2432 = v5;
                                let a2433 = match v2 {
                                    Adt3::C0(h2431, _, _, _, _, _, _, _) => {
                                        h2431
                                    }
                                };
                                nat_mul(a2432, a2433)?
                            };
                            let a2440 = {
                                let a2436 = {
                                    let a2434 = v11;
                                    let a2435 = 1u64;
                                    nat_sub(a2434, a2435)
                                };
                                let a2437 = 2u64;
                                let a2438 = 0u64;
                                nat_quot(a2436, a2437, a2438)
                            };
                            nat_add(a2439, a2440)?
                        }, v3)?))
                    }
                } else {
                    f162(v0, v10, {
                        let a2441 = v11;
                        let a2442 = 1u64;
                        nat_sub(a2441, a2442)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f81(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2443 = v1;
    let a2444 = f163(v2.clone(), v4.clone())?;
    if nat_lt(a2443, a2444) {
        let a2447 = v1;
        let a2448 = {
            let a2445 = 2u64;
            let a2446 = f131(v4.clone())?;
            nat_mul(a2445, a2446)?
        };
        if nat_lt(a2447, a2448) {
            let a2452 = {
                let a2449 = v1;
                let a2450 = 2u64;
                let a2451 = 0u64;
                nat_rem(a2449, a2450, a2451)
            };
            let a2453 = 0u64;
            if nat_eq(a2452, a2453) {
                f164({
                    let a2454 = v1;
                    let a2455 = 2u64;
                    let a2456 = 0u64;
                    nat_quot(a2454, a2455, a2456)
                }, v4)
            } else {
                if f165(match v2 {
                    Adt3::C0(_, h2457, _, _, _, _, _, _) => {
                        h2457
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2460 = v1;
                let a2461 = {
                    let a2458 = 2u64;
                    let a2459 = f131(v4.clone())?;
                    nat_mul(a2458, a2459)?
                };
                nat_sub(a2460, a2461)
            };
            let v7: u64 = {
                let a2465 = v6;
                let a2466 = {
                    let a2463 = 1u64;
                    let a2464 = match v2.clone() {
                        Adt3::C0(h2462, _, _, _, _, _, _, _) => {
                            h2462
                        }
                    };
                    nat_add(a2463, a2464)?
                };
                let a2467 = 0u64;
                nat_quot(a2465, a2466, a2467)
            };
            let v8: u64 = {
                let a2471 = v6;
                let a2472 = {
                    let a2469 = 1u64;
                    let a2470 = match v2.clone() {
                        Adt3::C0(h2468, _, _, _, _, _, _, _) => {
                            h2468
                        }
                    };
                    nat_add(a2469, a2470)?
                };
                let a2473 = 0u64;
                nat_rem(a2471, a2472, a2473)
            };
            let a2474 = v8;
            let a2475 = 0u64;
            if nat_eq(a2474, a2475) {
                f164(v7, v4)
            } else {
                f166(v0, v7, {
                    let a2476 = v8;
                    let a2477 = 1u64;
                    nat_sub(a2476, a2477)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2480 = match v4.clone() {
            Adt3::C0(h2478, _, _, _, _, _, _, _) => {
                h2478
            }
        };
        let a2481 = match v4.clone() {
            Adt3::C0(_, h2479, _, _, _, _, _, _) => {
                h2479
            }
        };
        nat_mul(a2480, a2481)?
    };
    let v7: u64 = {
        let a2482 = 2u64;
        let a2483 = v6;
        nat_mul(a2482, a2483)?
    };
    let v8: u64 = f167(match v3.clone() {
        Adt3::C0(_, h2484, _, _, _, _, _, _) => {
            h2484
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2485, _, _, _, _, _) => {
            h2485
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2486, _, _, _, _) => {
            h2486
        }
    });
    let a2487 = v0;
    let a2488 = v7;
    if nat_lt(a2487, a2488) {
        let a2492 = {
            let a2489 = v0;
            let a2490 = 2u64;
            let a2491 = 0u64;
            nat_rem(a2489, a2490, a2491)
        };
        let a2493 = 0u64;
        if nat_eq(a2492, a2493) {
            Ok(Adt9::C4(19u64, 5u64, f143({
                let a2494 = v0;
                let a2495 = 2u64;
                let a2496 = 0u64;
                nat_quot(a2494, a2495, a2496)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f168(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2497 = v0;
            let a2498 = v7;
            nat_sub(a2497, a2498)
        };
        let v10: u64 = {
            let a2502 = v9;
            let a2503 = {
                let a2500 = 1u64;
                let a2501 = match v1.clone() {
                    Adt3::C0(h2499, _, _, _, _, _, _, _) => {
                        h2499
                    }
                };
                nat_add(a2500, a2501)?
            };
            let a2504 = 0u64;
            nat_quot(a2502, a2503, a2504)
        };
        let v11: u64 = {
            let a2508 = v9;
            let a2509 = {
                let a2506 = 1u64;
                let a2507 = match v1.clone() {
                    Adt3::C0(h2505, _, _, _, _, _, _, _) => {
                        h2505
                    }
                };
                nat_add(a2506, a2507)?
            };
            let a2510 = 0u64;
            nat_rem(a2508, a2509, a2510)
        };
        let a2511 = v11;
        let a2512 = 0u64;
        if nat_eq(a2511, a2512) {
            Ok(Adt9::C4(19u64, 5u64, f143({
                let a2513 = v10;
                let a2514 = v6;
                let a2515 = 0u64;
                nat_rem(a2513, a2514, a2515)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f168(v5, v8) {
                let v12: u64 = {
                    let a2521 = {
                        let a2517 = v10;
                        let a2518 = match v1.clone() {
                            Adt3::C0(h2516, _, _, _, _, _, _, _) => {
                                h2516
                            }
                        };
                        nat_mul(a2517, a2518)?
                    };
                    let a2522 = {
                        let a2519 = v11;
                        let a2520 = 1u64;
                        nat_sub(a2519, a2520)
                    };
                    nat_add(a2521, a2522)?
                };
                let a2523 = v5;
                let a2524 = 4294967295u64;
                if nat_eq(a2523, a2524) {
                    Ok(Adt9::C1(false, f143(v12, v1)?, f143(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f143(v12, v1)?, 4u64, f143({
                        let a2530 = {
                            let a2526 = v5;
                            let a2527 = match v3.clone() {
                                Adt3::C0(h2525, _, _, _, _, _, _, _) => {
                                    h2525
                                }
                            };
                            nat_mul(a2526, a2527)?
                        };
                        let a2531 = {
                            let a2528 = v11;
                            let a2529 = 1u64;
                            nat_sub(a2528, a2529)
                        };
                        nat_add(a2530, a2531)?
                    }, v3)?, 3u64, f143(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f113(v3.clone())?;
    let v6: u64 = {
        let a2532 = v0;
        let a2533 = v5;
        let a2534 = 0u64;
        nat_quot(a2532, a2533, a2534)
    };
    let v7: u64 = {
        let a2535 = v0;
        let a2536 = v5;
        let a2537 = 0u64;
        nat_rem(a2535, a2536, a2537)
    };
    let v8: u64 = f169(v3.clone())?;
    let a2538 = v7;
    let a2539 = 0u64;
    if nat_eq(a2538, a2539) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2540, _) => {
                h2540
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2544 = v5;
        let a2545 = {
            let a2542 = v7;
            let a2543 = if match v3.clone() {
                Adt7::C0(_, _, _, h2541, _) => {
                    h2541
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2542, a2543)?
        };
        if nat_le(a2544, a2545) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2546, _) => {
                    h2546
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f143(v6, v2)?))
            } else {
                let a2549 = {
                    let a2547 = v7;
                    let a2548 = 2u64;
                    nat_add(a2547, a2548)?
                };
                let a2550 = v5;
                if nat_eq(a2549, a2550) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2551, _, _) => {
                            h2551
                        }
                    } {
                        let a2558 = {
                            let m2554 = match v3.clone() {
                                Adt7::C0(h2553, _, _, _, _) => {
                                    h2553
                                }
                            };
                            match m2554 {
                                Adt6::C0(h2552, _, _, _) => {
                                    h2552
                                }
                            }
                        };
                        let a2559 = {
                            let m2557 = match v3 {
                                Adt7::C0(_, h2556, _, _, _) => {
                                    h2556
                                }
                            };
                            match m2557 {
                                Adt6::C0(h2555, _, _, _) => {
                                    h2555
                                }
                            }
                        };
                        nat_mul(a2558, a2559)?
                    } else {
                        f170(match v3 {
                            Adt7::C0(h2560, _, _, _, _) => {
                                h2560
                            }
                        }, match v1 {
                            Adt3::C0(h2561, _, _, _, _, _, _, _) => {
                                h2561
                            }
                        }, f71(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2562 = v9;
                        let a2563 = 0u64;
                        if nat_eq(a2562, a2563) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f143(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2566 = {
                    let a2564 = v7;
                    let a2565 = 1u64;
                    nat_sub(a2564, a2565)
                };
                let a2567 = v8;
                let a2568 = 0u64;
                nat_rem(a2566, a2567, a2568)
            };
            let m2574 = f171(v3.clone(), v1, v2, v6, {
                let a2571 = {
                    let a2569 = v7;
                    let a2570 = 1u64;
                    nat_sub(a2569, a2570)
                };
                let a2572 = v8;
                let a2573 = 0u64;
                nat_quot(a2571, a2572, a2573)
            })?;
            match m2574 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2576 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2575) => {
                            h2575
                        }
                    };
                    let a2577 = 1u64;
                    if if nat_eq(a2576, a2577) {
                        let a2578 = v10;
                        let a2579 = 0u64;
                        nat_eq(a2578, a2579)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2580, _) => {
                                h2580
                            }
                        } {
                            let a2583 = {
                                let a2581 = v10;
                                let a2582 = 1u64;
                                nat_add(a2581, a2582)?
                            };
                            let a2584 = v8;
                            if nat_eq(a2583, a2584) {
                                let a2585 = v4;
                                let a2586 = 2u64;
                                if nat_eq(a2585, a2586) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2588 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2587) => {
                                                h2587
                                            }
                                        };
                                        let a2589 = 1u64;
                                        if nat_eq(a2588, a2589) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2591 = match v3 {
                                            Adt7::C0(_, _, _, _, h2590) => {
                                                h2590
                                            }
                                        };
                                        let a2592 = 1u64;
                                        if nat_eq(a2591, a2592) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2594 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2593) => {
                                            h2593
                                        }
                                    };
                                    let a2595 = 1u64;
                                    if nat_eq(a2594, a2595) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2597 = match v3 {
                                        Adt7::C0(_, _, _, _, h2596) => {
                                            h2596
                                        }
                                    };
                                    let a2598 = 1u64;
                                    if nat_eq(a2597, a2598) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2600 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2599) => {
                                        h2599
                                    }
                                };
                                let a2601 = 1u64;
                                if nat_eq(a2600, a2601) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2603 = match v3 {
                                    Adt7::C0(_, _, _, _, h2602) => {
                                        h2602
                                    }
                                };
                                let a2604 = 1u64;
                                if nat_eq(a2603, a2604) {
                                    1u64
                                } else {
                                    v11
                                }
                            }, 1u64, 0u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f111(match v3.clone() {
        Adt3::C0(h2605, _, _, _, _, _, _, _) => {
            h2605
        }
    })?;
    let v5: u64 = {
        let a2606 = v0;
        let a2607 = v4;
        let a2608 = 0u64;
        nat_rem(a2606, a2607, a2608)
    };
    let v6: u64 = {
        let a2609 = v0;
        let a2610 = v4;
        let a2611 = 0u64;
        nat_quot(a2609, a2610, a2611)
    };
    let a2612 = v5;
    let a2613 = 0u64;
    if nat_eq(a2612, a2613) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2616 = {
            let a2614 = v5;
            let a2615 = 1u64;
            nat_add(a2614, a2615)?
        };
        let a2617 = v4;
        if nat_eq(a2616, a2617) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f143(v6, v2)?))
        } else {
            let a2621 = {
                let a2618 = v5;
                let a2619 = 2u64;
                let a2620 = 0u64;
                nat_rem(a2618, a2619, a2620)
            };
            let a2622 = 0u64;
            if nat_eq(a2621, a2622) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2625 = {
                        let a2623 = v5;
                        let a2624 = 1u64;
                        nat_sub(a2623, a2624)
                    };
                    let a2626 = 2u64;
                    let a2627 = 0u64;
                    nat_quot(a2625, a2626, a2627)
                };
                let v8: u64 = f71(v6, v2.clone(), 0u64);
                let v9: u64 = f71(v6, v2.clone(), 1u64);
                let v10: u64 = f71(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f70(v1, {
                    let a2628 = v9;
                    let a2629 = v7;
                    nat_add(a2628, a2629)?
                }, v8, v10, 0u64)?, 4u64, f70(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f110(v3.clone())?;
    let v5: u64 = {
        let a2630 = v0;
        let a2631 = v4;
        let a2632 = 0u64;
        nat_rem(a2630, a2631, a2632)
    };
    let v6: u64 = {
        let a2633 = v0;
        let a2634 = v4;
        let a2635 = 0u64;
        nat_quot(a2633, a2634, a2635)
    };
    let a2636 = v5;
    let a2637 = 0u64;
    if nat_eq(a2636, a2637) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2640 = {
            let a2638 = v5;
            let a2639 = 1u64;
            nat_add(a2638, a2639)?
        };
        let a2641 = v4;
        if nat_eq(a2640, a2641) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f143(v6, v2)?))
        } else {
            let a2645 = {
                let a2642 = v5;
                let a2643 = 2u64;
                let a2644 = 0u64;
                nat_rem(a2642, a2643, a2644)
            };
            let a2646 = 0u64;
            if nat_eq(a2645, a2646) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2648 = f71(v6, v2.clone(), 0u64);
                    let a2649 = match v3.clone() {
                        Adt6::C0(_, _, h2647, _) => {
                            h2647
                        }
                    };
                    nat_add(a2648, a2649)?
                };
                let v8: u64 = {
                    let a2657 = {
                        let a2654 = {
                            let a2651 = v7;
                            let a2652 = match v3.clone() {
                                Adt6::C0(_, h2650, _, _) => {
                                    h2650
                                }
                            };
                            nat_add(a2651, a2652)?
                        };
                        let a2655 = match v3.clone() {
                            Adt6::C0(h2653, _, _, _) => {
                                h2653
                            }
                        };
                        nat_sub(a2654, a2655)
                    };
                    let a2658 = match v3.clone() {
                        Adt6::C0(_, h2656, _, _) => {
                            h2656
                        }
                    };
                    let a2659 = 0u64;
                    nat_quot(a2657, a2658, a2659)
                };
                let v9: u64 = {
                    let a2665 = v8;
                    let a2666 = {
                        let a2662 = {
                            let a2660 = v5;
                            let a2661 = 1u64;
                            nat_sub(a2660, a2661)
                        };
                        let a2663 = 2u64;
                        let a2664 = 0u64;
                        nat_quot(a2662, a2663, a2664)
                    };
                    nat_add(a2665, a2666)?
                };
                let v10: u64 = {
                    let a2668 = v9;
                    let a2669 = match v3.clone() {
                        Adt6::C0(_, h2667, _, _) => {
                            h2667
                        }
                    };
                    nat_mul(a2668, a2669)?
                };
                let v11: u64 = {
                    let a2670 = v7;
                    let a2671 = v10;
                    nat_sub(a2670, a2671)
                };
                let a2673 = v9;
                let a2674 = match v1.clone() {
                    Adt3::C0(_, h2672, _, _, _, _, _, _) => {
                        h2672
                    }
                };
                if if if nat_lt(a2673, a2674) {
                    let a2675 = v10;
                    let a2676 = v7;
                    nat_le(a2675, a2676)
                } else {
                    false
                } {
                    let a2678 = v11;
                    let a2679 = match v3.clone() {
                        Adt6::C0(h2677, _, _, _) => {
                            h2677
                        }
                    };
                    nat_lt(a2678, a2679)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f70(v1, {
                        let a2683 = {
                            let a2681 = f71(v6, v2, 1u64);
                            let a2682 = match v3 {
                                Adt6::C0(h2680, _, _, _) => {
                                    h2680
                                }
                            };
                            nat_mul(a2681, a2682)?
                        };
                        let a2684 = v11;
                        nat_add(a2683, a2684)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f71(v0, v2.clone(), 0u64);
    let v5: u64 = f71(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2686 = v5;
        let a2687 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2685, _) => {
                h2685
            }
        };
        let a2688 = 0u64;
        nat_rem(a2686, a2687, a2688)
    };
    let v7: u64 = {
        let a2690 = v5;
        let a2691 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2689, _) => {
                h2689
            }
        };
        let a2692 = 0u64;
        nat_quot(a2690, a2691, a2692)
    };
    let v8: Option<u64> = f73(match v3.clone() {
        Adt5::C0(h2693, _, _, _, _, _, _, _) => {
            h2693
        }
    }, match v1.clone() {
        Adt3::C0(h2694, _, _, _, _, _, _, _) => {
            h2694
        }
    }, f71(v0, v2.clone(), 1u64), {
        let a2698 = v4;
        let a2699 = {
            let m2697 = match v3.clone() {
                Adt5::C0(h2696, _, _, _, _, _, _, _) => {
                    h2696
                }
            };
            match m2697 {
                Adt6::C0(h2695, _, _, _) => {
                    h2695
                }
            }
        };
        let a2700 = 0u64;
        nat_rem(a2698, a2699, a2700)
    })?;
    let v9: u64 = {
        let a2704 = v4;
        let a2705 = {
            let m2703 = match v3.clone() {
                Adt5::C0(h2702, _, _, _, _, _, _, _) => {
                    h2702
                }
            };
            match m2703 {
                Adt6::C0(h2701, _, _, _) => {
                    h2701
                }
            }
        };
        let a2706 = 0u64;
        nat_quot(a2704, a2705, a2706)
    };
    let v10: Option<u64> = f73(match v3.clone() {
        Adt5::C0(_, h2707, _, _, _, _, _, _) => {
            h2707
        }
    }, match v1.clone() {
        Adt3::C0(_, h2708, _, _, _, _, _, _) => {
            h2708
        }
    }, f71(v0, v2.clone(), 2u64), {
        let a2712 = v9;
        let a2713 = {
            let m2711 = match v3.clone() {
                Adt5::C0(_, h2710, _, _, _, _, _, _) => {
                    h2710
                }
            };
            match m2711 {
                Adt6::C0(h2709, _, _, _) => {
                    h2709
                }
            }
        };
        let a2714 = 0u64;
        nat_rem(a2712, a2713, a2714)
    })?;
    let v11: u64 = {
        let a2718 = v9;
        let a2719 = {
            let m2717 = match v3.clone() {
                Adt5::C0(_, h2716, _, _, _, _, _, _) => {
                    h2716
                }
            };
            match m2717 {
                Adt6::C0(h2715, _, _, _) => {
                    h2715
                }
            }
        };
        let a2720 = 0u64;
        nat_quot(a2718, a2719, a2720)
    };
    let v12: Option<u64> = f73(match v3.clone() {
        Adt5::C0(_, _, h2721, _, _, _, _, _) => {
            h2721
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2722, _, _, _, _, _) => {
            h2722
        }
    }, v6, {
        let a2726 = v11;
        let a2727 = {
            let m2725 = match v3.clone() {
                Adt5::C0(_, _, h2724, _, _, _, _, _) => {
                    h2724
                }
            };
            match m2725 {
                Adt6::C0(h2723, _, _, _) => {
                    h2723
                }
            }
        };
        let a2728 = 0u64;
        nat_rem(a2726, a2727, a2728)
    })?;
    let v13: u64 = {
        let a2732 = v11;
        let a2733 = {
            let m2731 = match v3.clone() {
                Adt5::C0(_, _, h2730, _, _, _, _, _) => {
                    h2730
                }
            };
            match m2731 {
                Adt6::C0(h2729, _, _, _) => {
                    h2729
                }
            }
        };
        let a2734 = 0u64;
        nat_quot(a2732, a2733, a2734)
    };
    let v17: Adt12 = {
        let m2735 = v8;
        match m2735 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2736 = v10;
                match m2736 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2737 = v12;
                        match m2737 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2746 = {
                                        let a2741 = f70(v1, v14, v15, v16, 0u64)?;
                                        let a2742 = {
                                            let a2739 = v7;
                                            let a2740 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2738, _, _, _) => {
                                                    h2738
                                                }
                                            };
                                            nat_mul(a2739, a2740)?
                                        };
                                        nat_add(a2741, a2742)?
                                    };
                                    let a2747 = {
                                        let a2744 = v13;
                                        let a2745 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2743, _, _, _, _) => {
                                                h2743
                                            }
                                        };
                                        nat_mul(a2744, a2745)?
                                    };
                                    nat_add(a2746, a2747)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2748, _, _) => {
            h2748
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2749) => {
            h2749
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f143(v0, v2)?), {
        let a2751 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2750, _, _) => {
                h2750
            }
        };
        let a2752 = 0u64;
        if nat_eq(a2751, a2752) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2754 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2753) => {
                h2753
            }
        };
        let a2755 = 0u64;
        if nat_eq(a2754, a2755) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f87(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2768 = f172(v0, v1, match v2.clone() {
        Adt3::C0(h2756, _, _, _, _, _, _, _) => {
            h2756
        }
    }, match v2.clone() {
        Adt3::C0(_, h2757, _, _, _, _, _, _) => {
            h2757
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2758, _, _, _, _, _) => {
            h2758
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2759, _, _, _, _) => {
            h2759
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2760, _, _, _) => {
            h2760
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2761, _, _) => {
            h2761
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2762, _) => {
            h2762
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2763) => {
            h2763
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2764, _, _, _) => {
            h2764
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2765, _, _) => {
            h2765
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2766, _) => {
            h2766
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2767) => {
            h2767
        }
    }, v4)?;
    match m2768 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f88(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2771 = v6;
        let a2772 = {
            let a2769 = 3u64;
            let a2770 = v5;
            nat_add(a2769, a2770)?
        };
        nat_sub(a2771, a2772)
    };
    let a2774 = v0;
    let a2775 = f36(v3, match v1.clone() {
        Adt3::C0(h2773, _, _, _, _, _, _, _) => {
            h2773
        }
    }, v5, v8)?;
    if nat_lt(a2774, a2775) {
        let v9: u64 = f102(match v1.clone() {
            Adt3::C0(h2776, _, _, _, _, _, _, _) => {
                h2776
            }
        }, v5, v8)?;
        let v10: u64 = f103(v3, match v1.clone() {
            Adt3::C0(h2777, _, _, _, _, _, _, _) => {
                h2777
            }
        }, v5)?;
        let a2778 = v0;
        let a2779 = v9;
        if nat_lt(a2778, a2779) {
            f173(v0, v2, f30(v3), {
                let a2780 = 3u64;
                let a2781 = v5;
                nat_add(a2780, a2781)?
            }, v7)
        } else {
            let a2784 = v0;
            let a2785 = {
                let a2782 = v9;
                let a2783 = v10;
                nat_add(a2782, a2783)?
            };
            if nat_lt(a2784, a2785) {
                let v11: u64 = {
                    let a2786 = v0;
                    let a2787 = v9;
                    nat_sub(a2786, a2787)
                };
                let v12: u64 = f174(v3, match v1.clone() {
                    Adt3::C0(h2788, _, _, _, _, _, _, _) => {
                        h2788
                    }
                })?;
                f175({
                    let a2789 = v11;
                    let a2790 = v12;
                    let a2791 = 0u64;
                    nat_rem(a2789, a2790, a2791)
                }, {
                    let a2792 = v11;
                    let a2793 = v12;
                    let a2794 = 0u64;
                    nat_quot(a2792, a2793, a2794)
                }, v1, v3, v4, v7)
            } else {
                f176({
                    let a2797 = v0;
                    let a2798 = {
                        let a2795 = v9;
                        let a2796 = v10;
                        nat_add(a2795, a2796)?
                    };
                    nat_sub(a2797, a2798)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2799 = v0;
    let a2800 = f19(v1)?;
    if nat_lt(a2799, a2800) {
        let a2801 = v0;
        let a2802 = 4u64;
        if nat_lt(a2801, a2802) {
            let a2803 = v0;
            let a2804 = 0u64;
            if nat_eq(a2803, a2804) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2805 = v0;
                let a2806 = 1u64;
                if nat_eq(a2805, a2806) {
                    let a2807 = v5;
                    let a2808 = 0u64;
                    if nat_eq(a2807, a2808) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2809 = v0;
                    let a2810 = 2u64;
                    if nat_eq(a2809, a2810) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2811 = v0;
            let a2812 = 4u64;
            if if nat_eq(a2811, a2812) {
                let a2813 = f177(v5)?;
                let a2814 = v1;
                !nat_eq(a2813, a2814)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2817 = {
                        let a2815 = v0;
                        let a2816 = 4u64;
                        nat_sub(a2815, a2816)
                    };
                    let a2818 = 3u64;
                    let a2819 = 0u64;
                    nat_quot(a2817, a2818, a2819)
                };
                let v7: u64 = {
                    let a2822 = {
                        let a2820 = v0;
                        let a2821 = 4u64;
                        nat_sub(a2820, a2821)
                    };
                    let a2823 = 3u64;
                    let a2824 = 0u64;
                    nat_rem(a2822, a2823, a2824)
                };
                let a2825 = v7;
                let a2826 = 0u64;
                if nat_eq(a2825, a2826) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2827 = v7;
                    let a2828 = 1u64;
                    if nat_eq(a2827, a2828) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f18(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f90(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2829 = v1;
        let a2830 = 5u64;
        nat_eq(a2829, a2830)
    } else {
        false
    } {
        let a2831 = v2;
        let a2832 = 16u64;
        nat_eq(a2831, a2832)
    } else {
        false
    } {
        let a2833 = v3;
        let a2834 = 0u64;
        nat_eq(a2833, a2834)
    } else {
        false
    } {
        let a2835 = v4;
        let a2836 = 0u64;
        nat_eq(a2835, a2836)
    } else {
        false
    }
}

pub fn f91(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2837 = v1;
    let a2838 = 0u64;
    if nat_eq(a2837, a2838) {
        if v0 {
            let a2839 = v3;
            let a2840 = 0u64;
            if if if nat_eq(a2839, a2840) {
                let a2841 = v4;
                let a2842 = 43u64;
                nat_eq(a2841, a2842)
            } else {
                false
            } {
                true
            } else {
                let a2843 = v3;
                let a2844 = 43u64;
                if nat_eq(a2843, a2844) {
                    let a2845 = v4;
                    let a2846 = 0u64;
                    nat_eq(a2845, a2846)
                } else {
                    false
                }
            } {
                f160(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2847 = v1;
        let a2848 = 1u64;
        if nat_eq(a2847, a2848) {
            Adt11::C1(v2)
        } else {
            let a2849 = v1;
            let a2850 = 2u64;
            if nat_eq(a2849, a2850) {
                Adt11::C5(v2)
            } else {
                let a2851 = v1;
                let a2852 = 3u64;
                if nat_eq(a2851, a2852) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2853 = v1;
                    let a2854 = 4u64;
                    if nat_eq(a2853, a2854) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f90(v0, v1, v2, v3, v4) {
                            Adt11::C7(v2, v3, v4)
                        } else {
                            Adt11::C6
                        }
                    }
                }
            }
        }
    }
}

pub fn f92(v0: u64, v1: u64) -> Adt12 {
    let a2855 = v0;
    let a2856 = 0u64;
    if nat_eq(a2855, a2856) {
        Adt12::C0(0u64, v1)
    } else {
        let a2857 = v0;
        let a2858 = 1u64;
        if nat_eq(a2857, a2858) {
            Adt12::C1(v1)
        } else {
            let a2859 = v0;
            let a2860 = 2u64;
            if nat_eq(a2859, a2860) {
                Adt12::C2(v1)
            } else {
                let a2861 = v0;
                let a2862 = 3u64;
                if nat_eq(a2861, a2862) {
                    Adt12::C3(v1)
                } else {
                    let a2863 = v0;
                    let a2864 = 13u64;
                    if nat_lt(a2863, a2864) {
                        Adt12::C0({
                            let a2865 = v0;
                            let a2866 = 3u64;
                            nat_sub(a2865, a2866)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f93(v0: u64, v1: u64) -> Adt13 {
    let a2867 = v0;
    let a2868 = 1u64;
    if nat_eq(a2867, a2868) {
        Adt13::C0(v1)
    } else {
        let a2869 = v0;
        let a2870 = 3u64;
        if if nat_eq(a2869, a2870) {
            true
        } else {
            let a2871 = v0;
            let a2872 = 6u64;
            nat_eq(a2871, a2872)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f94(v0: Adt6, v1: u64) -> R<u64> {
    let a2874 = 0u64;
    let a2875 = match v0.clone() {
        Adt6::C0(h2873, _, _, _) => {
            h2873
        }
    };
    if if if if if nat_lt(a2874, a2875) {
        let a2877 = 0u64;
        let a2878 = match v0.clone() {
            Adt6::C0(_, h2876, _, _) => {
                h2876
            }
        };
        nat_lt(a2877, a2878)
    } else {
        false
    } {
        let a2880 = 0u64;
        let a2881 = match v0.clone() {
            Adt6::C0(_, _, _, h2879) => {
                h2879
            }
        };
        nat_lt(a2880, a2881)
    } else {
        false
    } {
        let a2883 = match v0.clone() {
            Adt6::C0(_, h2882, _, _) => {
                h2882
            }
        };
        let a2884 = 2147483648u64;
        nat_lt(a2883, a2884)
    } else {
        false
    } {
        let a2886 = match v0.clone() {
            Adt6::C0(_, _, _, h2885) => {
                h2885
            }
        };
        let a2887 = 2147483648u64;
        nat_lt(a2886, a2887)
    } else {
        false
    } {
        let v2: u64 = {
            let a2891 = v1;
            let a2892 = {
                let a2889 = 2u64;
                let a2890 = f137(match v0.clone() {
                    Adt6::C0(_, _, h2888, _) => {
                        h2888
                    }
                });
                nat_mul(a2889, a2890)?
            };
            nat_add(a2891, a2892)?
        };
        let v3: u64 = {
            let a2904 = {
                let a2899 = 1u64;
                let a2900 = {
                    let a2897 = match v0.clone() {
                        Adt6::C0(_, _, _, h2893) => {
                            h2893
                        }
                    };
                    let a2898 = {
                        let a2895 = match v0.clone() {
                            Adt6::C0(h2894, _, _, _) => {
                                h2894
                            }
                        };
                        let a2896 = 1u64;
                        nat_sub(a2895, a2896)
                    };
                    nat_mul(a2897, a2898)?
                };
                nat_add(a2899, a2900)?
            };
            let a2905 = {
                let a2902 = 2u64;
                let a2903 = f136(match v0.clone() {
                    Adt6::C0(_, _, h2901, _) => {
                        h2901
                    }
                });
                nat_mul(a2902, a2903)?
            };
            nat_add(a2904, a2905)?
        };
        let a2906 = v3;
        let a2907 = v2;
        if nat_le(a2906, a2907) {
            let a2914 = 1u64;
            let a2915 = {
                let a2911 = {
                    let a2908 = v2;
                    let a2909 = v3;
                    nat_sub(a2908, a2909)
                };
                let a2912 = match v0 {
                    Adt6::C0(_, h2910, _, _) => {
                        h2910
                    }
                };
                let a2913 = 0u64;
                nat_quot(a2911, a2912, a2913)
            };
            nat_add(a2914, a2915)
        } else {
            let a2919 = {
                let a2916 = v3;
                let a2917 = v2;
                nat_sub(a2916, a2917)
            };
            let a2920 = match v0 {
                Adt6::C0(_, h2918, _, _) => {
                    h2918
                }
            };
            if nat_lt(a2919, a2920) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f95(v0: u64) -> u64 {
    let v1: u64 = {
        let a2921 = v0;
        let a2922 = 4294967296u64;
        let a2923 = 0u64;
        nat_rem(a2921, a2922, a2923)
    };
    let a2924 = v1;
    let a2925 = 2147483648u64;
    if nat_lt(a2924, a2925) {
        v1
    } else {
        0u64
    }
}

pub fn f96(v0: u64) -> R<u64> {
    let a2928 = 2u64;
    let a2929 = {
        let a2926 = 2u64;
        let a2927 = v0;
        nat_mul(a2926, a2927)?
    };
    nat_add(a2928, a2929)
}

pub fn f97(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2931 = match v1.clone() {
        Adt14::C0(h2930, _, _, _, _, _, _, _, _, _, _) => {
            h2930
        }
    };
    let a2932 = 0u64;
    if if if if nat_eq(a2931, a2932) {
        let a2934 = match v2.clone() {
            Adt14::C0(h2933, _, _, _, _, _, _, _, _, _, _) => {
                h2933
            }
        };
        let a2935 = 0u64;
        nat_eq(a2934, a2935)
    } else {
        false
    } {
        f178(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f179(match v1.clone() {
            Adt14::C0(_, h2936, _, _, _, _, _, _, _, _, _) => {
                h2936
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2937, _, _, _, _, _, _, _, _) => {
                h2937
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2938, _, _, _, _, _, _, _) => {
                h2938
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2939, _, _, _, _, _, _) => {
                h2939
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f180(v1.clone(), v2.clone())
            } else {
                false
            } {
                f181(v1.clone())
            } else {
                false
            } {
                f181(v2.clone())
            } else {
                false
            } {
                if f182(v1.clone(), v2.clone())? {
                    true
                } else {
                    f183(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f179(match v1.clone() {
            Adt14::C0(_, h2940, _, _, _, _, _, _, _, _, _) => {
                h2940
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2941, _, _, _, _, _, _, _, _) => {
                h2941
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2942, _, _, _, _, _, _, _) => {
                h2942
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2943, _, _, _, _, _, _) => {
                h2943
            }
        }) {
            0u64
        } else {
            let a2949 = match v1.clone() {
                Adt14::C0(_, _, h2944, _, _, _, _, _, _, _, _) => {
                    h2944
                }
            };
            let a2950 = {
                let a2947 = match v1.clone() {
                    Adt14::C0(_, _, _, h2945, _, _, _, _, _, _, _) => {
                        h2945
                    }
                };
                let a2948 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2946, _, _, _, _, _, _) => {
                        h2946
                    }
                };
                nat_mul(a2947, a2948)?
            };
            nat_mul(a2949, a2950)?
        };
        let v4: u64 = {
            let a2951 = v3;
            let a2952 = 0u64;
            if if nat_eq(a2951, a2952) {
                true
            } else {
                f184(v2.clone())?
            } {
                0u64
            } else {
                let a2954 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2953, _) => {
                        h2953
                    }
                };
                let a2955 = 4u64;
                let a2956 = 0u64;
                nat_quot(a2954, a2955, a2956)
            }
        };
        Ok(Adt15::C1(f185(f186(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2957, _, _, _, _, _) => {
                h2957
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2958, _, _, _, _, _) => {
                h2958
            }
        }), match v1 {
            Adt14::C0(_, h2959, _, _, _, _, _, _, _, _, _) => {
                h2959
            }
        }, v3, v4)?, 4u64, 64u64, v3, f187(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f98(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f188(v3.clone());
    let a2961 = match v2.clone() {
        Adt14::C0(h2960, _, _, _, _, _, _, _, _, _, _) => {
            h2960
        }
    };
    let a2962 = 0u64;
    if if if if nat_eq(a2961, a2962) {
        let a2964 = match v3.clone() {
            Adt14::C0(h2963, _, _, _, _, _, _, _, _, _, _) => {
                h2963
            }
        };
        let a2965 = 0u64;
        nat_eq(a2964, a2965)
    } else {
        false
    } {
        f178(v2.clone(), v3)
    } else {
        false
    } {
        if f179(match v2.clone() {
            Adt14::C0(_, h2966, _, _, _, _, _, _, _, _, _) => {
                h2966
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2967, _, _, _, _, _, _, _, _) => {
                h2967
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2968, _, _, _, _, _, _, _) => {
                h2968
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2969, _, _, _, _, _, _) => {
                h2969
            }
        }) {
            true
        } else {
            let a2971 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2970, _, _, _, _, _) => {
                    h2970
                }
            };
            let a2972 = 4u64;
            if if if if if nat_eq(a2971, a2972) {
                f189(match v2.clone() {
                    Adt14::C0(_, h2973, _, _, _, _, _, _, _, _, _) => {
                        h2973
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2974, _, _, _, _, _, _, _, _) => {
                        h2974
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2975, _, _, _, _, _, _, _) => {
                        h2975
                    }
                }, v0)?
            } else {
                false
            } {
                f181(v2.clone())
            } else {
                false
            } {
                f181(v4.clone())
            } else {
                false
            } {
                let a2977 = f190(v4.clone())?;
                let a2978 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2976, _) => {
                        h2976
                    }
                };
                nat_le(a2977, a2978)
            } else {
                false
            } {
                if f182(v2.clone(), v4.clone())? {
                    true
                } else {
                    f183(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f179(match v2.clone() {
            Adt14::C0(_, h2979, _, _, _, _, _, _, _, _, _) => {
                h2979
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2980, _, _, _, _, _, _, _, _) => {
                h2980
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2981, _, _, _, _, _, _, _) => {
                h2981
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2982, _, _, _, _, _, _) => {
                h2982
            }
        }) {
            true
        } else {
            f184(v4.clone())?
        } {
            0u64
        } else {
            let a2984 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2983, _) => {
                    h2983
                }
            };
            let a2985 = 4u64;
            let a2986 = 0u64;
            nat_quot(a2984, a2985, a2986)
        };
        Ok(Adt15::C1(f191(match v2.clone() {
            Adt14::C0(_, h2987, _, _, _, _, _, _, _, _, _) => {
                h2987
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2988, _, _, _, _, _, _, _, _) => {
                h2988
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2989, _, _, _, _, _, _, _) => {
                h2989
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2990, _, _, _, _, _, _) => {
                h2990
            }
        }, v0, v5)?, 4u64, 65u64, v0, f187(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f99(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f188(v4.clone());
    if if if if if if if v1 {
        let a2992 = match v2.clone() {
            Adt14::C0(h2991, _, _, _, _, _, _, _, _, _, _) => {
                h2991
            }
        };
        let a2993 = 0u64;
        nat_eq(a2992, a2993)
    } else {
        false
    } {
        let a2995 = match v3.clone() {
            Adt14::C0(h2994, _, _, _, _, _, _, _, _, _, _) => {
                h2994
            }
        };
        let a2996 = 0u64;
        nat_eq(a2995, a2996)
    } else {
        false
    } {
        let a2998 = match v4.clone() {
            Adt14::C0(h2997, _, _, _, _, _, _, _, _, _, _) => {
                h2997
            }
        };
        let a2999 = 0u64;
        nat_eq(a2998, a2999)
    } else {
        false
    } {
        f178(v2.clone(), v3.clone())
    } else {
        false
    } {
        f178(v2.clone(), v4)
    } else {
        false
    } {
        if f179(match v2.clone() {
            Adt14::C0(_, h3000, _, _, _, _, _, _, _, _, _) => {
                h3000
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3001, _, _, _, _, _, _, _, _) => {
                h3001
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3002, _, _, _, _, _, _, _) => {
                h3002
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3003, _, _, _, _, _, _) => {
                h3003
            }
        }) {
            true
        } else {
            let a3005 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3004, _, _, _, _, _) => {
                    h3004
                }
            };
            let a3006 = 4u64;
            if if if if if if if nat_eq(a3005, a3006) {
                let a3008 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3007, _, _, _, _, _) => {
                        h3007
                    }
                };
                let a3009 = 4u64;
                nat_eq(a3008, a3009)
            } else {
                false
            } {
                f181(v2.clone())
            } else {
                false
            } {
                f181(v3.clone())
            } else {
                false
            } {
                f181(v5.clone())
            } else {
                false
            } {
                let a3011 = f190(v5.clone())?;
                let a3012 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3010, _) => {
                        h3010
                    }
                };
                nat_le(a3011, a3012)
            } else {
                false
            } {
                if f182(v2.clone(), v5.clone())? {
                    true
                } else {
                    f183(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f182(v3.clone(), v5.clone())? {
                    true
                } else {
                    f183(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f179(match v2.clone() {
            Adt14::C0(_, h3013, _, _, _, _, _, _, _, _, _) => {
                h3013
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3014, _, _, _, _, _, _, _, _) => {
                h3014
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3015, _, _, _, _, _, _, _) => {
                h3015
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3016, _, _, _, _, _, _) => {
                h3016
            }
        }) {
            0u64
        } else {
            let a3022 = match v2.clone() {
                Adt14::C0(_, _, h3017, _, _, _, _, _, _, _, _) => {
                    h3017
                }
            };
            let a3023 = {
                let a3020 = match v2.clone() {
                    Adt14::C0(_, _, _, h3018, _, _, _, _, _, _, _) => {
                        h3018
                    }
                };
                let a3021 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3019, _, _, _, _, _, _) => {
                        h3019
                    }
                };
                nat_mul(a3020, a3021)?
            };
            nat_mul(a3022, a3023)?
        };
        let v7: u64 = {
            let a3024 = v6;
            let a3025 = 0u64;
            if if nat_eq(a3024, a3025) {
                true
            } else {
                f184(v5.clone())?
            } {
                0u64
            } else {
                let a3027 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3026, _) => {
                        h3026
                    }
                };
                let a3028 = 4u64;
                let a3029 = 0u64;
                nat_quot(a3027, a3028, a3029)
            }
        };
        Ok(Adt15::C1(f192(match v2 {
            Adt14::C0(_, h3030, _, _, _, _, _, _, _, _, _) => {
                h3030
            }
        }, v6, v7)?, 4u64, 66u64, v6, f187(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f193(v0, v3, v6) {
        f193(v1, v4, v6)
    } else {
        false
    } {
        f193(v2, v5, v6)
    } else {
        false
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f194(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f194(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f194(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f194(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f194(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f194(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f102(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3031 = v2;
    let a3032 = 0u64;
    if nat_eq(a3031, a3032) {
        Ok(0u64)
    } else {
        let a3035 = 2u64;
        let a3036 = {
            let a3033 = v0;
            let a3034 = v1;
            nat_mul(a3033, a3034)?
        };
        nat_mul(a3035, a3036)
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3037 = v2;
    let a3038 = f174(v0, v1)?;
    nat_mul(a3037, a3038)
}

pub fn f104(v0: u64) -> u64 {
    let a3039 = v0;
    let a3040 = 30u64;
    if nat_eq(a3039, a3040) {
        3u64
    } else {
        1u64
    }
}

pub fn f105(v0: u64) -> R<u64> {
    let a3043 = 5u64;
    let a3044 = {
        let a3041 = 4u64;
        let a3042 = v0;
        nat_mul(a3041, a3042)?
    };
    nat_add(a3043, a3044)
}

pub fn f106(v0: Adt2, v1: u64) -> R<bool> {
    let a3046 = match v0.clone() {
        Adt2::C0(h3045, _, _, _, _, _, _, _, _, _, _) => {
            h3045
        }
    };
    let a3047 = 0u64;
    if if nat_eq(a3046, a3047) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a3048 = v1;
        let a3049 = 1u64;
        if nat_le(a3048, a3049) {
            Ok(true)
        } else {
            let a3054 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3050, _, _, _, _) => {
                    h3050
                }
            };
            let a3055 = {
                let a3052 = 4u64;
                let a3053 = match v0 {
                    Adt2::C0(_, h3051, _, _, _, _, _, _, _, _, _) => {
                        h3051
                    }
                };
                nat_mul(a3052, a3053)?
            };
            Ok(nat_eq(a3054, a3055))
        }
    } else {
        Ok(false)
    }
}

pub fn f107(v0: u64) -> R<u64> {
    let a3058 = 2u64;
    let a3059 = {
        let a3056 = 6u64;
        let a3057 = v0;
        nat_mul(a3056, a3057)?
    };
    nat_add(a3058, a3059)
}

pub fn f108(v0: u64) -> bool {
    let a3060 = v0;
    let a3061 = 0u64;
    if if nat_eq(a3060, a3061) {
        true
    } else {
        let a3062 = v0;
        let a3063 = 1u64;
        nat_eq(a3062, a3063)
    } {
        true
    } else {
        let a3064 = v0;
        let a3065 = 30u64;
        nat_eq(a3064, a3065)
    }
}

pub fn f109(v0: Adt2, v1: u64) -> R<bool> {
    let a3067 = match v0.clone() {
        Adt2::C0(_, _, _, h3066, _, _, _, _, _, _, _) => {
            h3066
        }
    };
    let a3068 = 1u64;
    if if if nat_eq(a3067, a3068) {
        let a3070 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3069, _, _, _, _, _, _) => {
                h3069
            }
        };
        let a3071 = 1u64;
        nat_eq(a3070, a3071)
    } else {
        false
    } {
        let a3073 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h3072, _, _, _, _, _) => {
                h3072
            }
        };
        let a3074 = v1;
        nat_eq(a3073, a3074)
    } else {
        false
    } {
        let a3076 = match v0.clone() {
            Adt2::C0(_, _, h3075, _, _, _, _, _, _, _, _) => {
                h3075
            }
        };
        let a3077 = 1u64;
        if nat_eq(a3076, a3077) {
            Ok(true)
        } else {
            let a3082 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3078, _, _, _, _) => {
                    h3078
                }
            };
            let a3083 = {
                let a3080 = match v0 {
                    Adt2::C0(_, h3079, _, _, _, _, _, _, _, _, _) => {
                        h3079
                    }
                };
                let a3081 = v1;
                nat_mul(a3080, a3081)?
            };
            Ok(nat_eq(a3082, a3083))
        }
    } else {
        Ok(false)
    }
}

pub fn f110(v0: Adt6) -> R<u64> {
    let a3096 = 2u64;
    let a3097 = {
        let a3094 = 2u64;
        let a3095 = {
            let a3091 = {
                let a3088 = match v0.clone() {
                    Adt6::C0(h3084, _, _, _) => {
                        h3084
                    }
                };
                let a3089 = {
                    let a3086 = match v0.clone() {
                        Adt6::C0(_, h3085, _, _) => {
                            h3085
                        }
                    };
                    let a3087 = 1u64;
                    nat_sub(a3086, a3087)
                };
                nat_add(a3088, a3089)?
            };
            let a3092 = match v0 {
                Adt6::C0(_, h3090, _, _) => {
                    h3090
                }
            };
            let a3093 = 0u64;
            nat_quot(a3091, a3092, a3093)
        };
        nat_mul(a3094, a3095)?
    };
    nat_add(a3096, a3097)
}

pub fn f111(v0: u64) -> R<u64> {
    let a3100 = 2u64;
    let a3101 = {
        let a3098 = v0;
        let a3099 = 2u64;
        nat_mul(a3098, a3099)?
    };
    nat_add(a3100, a3101)
}

pub fn f112(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3105 = {
            let a3102 = v2;
            let a3103 = 1u64;
            nat_sub(a3102, a3103)
        };
        let a3106 = match v0.clone() {
            Adt6::C0(_, h3104, _, _) => {
                h3104
            }
        };
        nat_mul(a3105, a3106)?
    };
    let a3108 = 0u64;
    let a3109 = match v0.clone() {
        Adt6::C0(h3107, _, _, _) => {
            h3107
        }
    };
    if if if if if if if if nat_lt(a3108, a3109) {
        let a3111 = match v0.clone() {
            Adt6::C0(h3110, _, _, _) => {
                h3110
            }
        };
        let a3112 = 2147483648u64;
        nat_lt(a3111, a3112)
    } else {
        false
    } {
        let a3114 = 0u64;
        let a3115 = match v0.clone() {
            Adt6::C0(_, h3113, _, _) => {
                h3113
            }
        };
        nat_lt(a3114, a3115)
    } else {
        false
    } {
        let a3117 = match v0.clone() {
            Adt6::C0(_, h3116, _, _) => {
                h3116
            }
        };
        let a3118 = 2147483648u64;
        nat_lt(a3117, a3118)
    } else {
        false
    } {
        let a3119 = v1;
        let a3120 = 2147483648u64;
        nat_lt(a3119, a3120)
    } else {
        false
    } {
        let a3121 = 0u64;
        let a3122 = v2;
        nat_lt(a3121, a3122)
    } else {
        false
    } {
        let a3123 = v3;
        let a3124 = 2147483648u64;
        nat_lt(a3123, a3124)
    } else {
        false
    } {
        let a3136 = {
            let a3131 = {
                let a3128 = v3;
                let a3129 = {
                    let a3126 = match v0.clone() {
                        Adt6::C0(h3125, _, _, _) => {
                            h3125
                        }
                    };
                    let a3127 = 1u64;
                    nat_sub(a3126, a3127)
                };
                nat_add(a3128, a3129)?
            };
            let a3132 = f136(match v0.clone() {
                Adt6::C0(_, _, h3130, _) => {
                    h3130
                }
            });
            nat_add(a3131, a3132)?
        };
        let a3137 = {
            let a3134 = 2147483647u64;
            let a3135 = f137(match v0.clone() {
                Adt6::C0(_, _, h3133, _) => {
                    h3133
                }
            });
            nat_add(a3134, a3135)?
        };
        nat_le(a3136, a3137)
    } else {
        false
    } {
        let a3139 = match v0 {
            Adt6::C0(_, _, h3138, _) => {
                h3138
            }
        };
        let a3140 = 2147483648u64;
        Ok(!nat_eq(a3139, a3140))
    } else {
        Ok(false)
    }
}

pub fn f113(v0: Adt7) -> R<u64> {
    let a3152 = if match v0.clone() {
        Adt7::C0(_, _, _, h3141, _) => {
            h3141
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3153 = {
        let a3150 = {
            let a3148 = {
                let m3144 = match v0.clone() {
                    Adt7::C0(h3143, _, _, _, _) => {
                        h3143
                    }
                };
                match m3144 {
                    Adt6::C0(h3142, _, _, _) => {
                        h3142
                    }
                }
            };
            let a3149 = {
                let m3147 = match v0.clone() {
                    Adt7::C0(_, h3146, _, _, _) => {
                        h3146
                    }
                };
                match m3147 {
                    Adt6::C0(h3145, _, _, _) => {
                        h3145
                    }
                }
            };
            nat_mul(a3148, a3149)?
        };
        let a3151 = f169(v0)?;
        nat_mul(a3150, a3151)?
    };
    nat_add(a3152, a3153)
}

pub fn f114(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f195(match v0.clone() {
        Adt2::C0(h3154, _, _, _, _, _, _, _, _, _, _) => {
            h3154
        }
    }, match v0.clone() {
        Adt2::C0(_, h3155, _, _, _, _, _, _, _, _, _) => {
            h3155
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3156, _, _, _, _, _, _, _, _) => {
            h3156
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3157, _, _, _, _, _, _, _) => {
            h3157
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3158, _, _, _, _, _, _) => {
            h3158
        }
    }, match v1.clone() {
        Adt2::C0(h3159, _, _, _, _, _, _, _, _, _, _) => {
            h3159
        }
    }, match v1.clone() {
        Adt2::C0(_, h3160, _, _, _, _, _, _, _, _, _) => {
            h3160
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3161, _, _, _, _, _, _, _, _) => {
            h3161
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3162, _, _, _, _, _, _, _) => {
            h3162
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3163, _, _, _, _, _, _) => {
            h3163
        }
    }, match v2.clone() {
        Adt2::C0(h3164, _, _, _, _, _, _, _, _, _, _) => {
            h3164
        }
    }, match v2.clone() {
        Adt2::C0(_, h3165, _, _, _, _, _, _, _, _, _) => {
            h3165
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3166, _, _, _, _, _, _, _, _) => {
            h3166
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3167, _, _, _, _, _, _, _) => {
            h3167
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3168, _, _, _, _, _, _) => {
            h3168
        }
    }) {
        let m3170 = f118(match v0.clone() {
            Adt2::C0(h3169, _, _, _, _, _, _, _, _, _, _) => {
                h3169
            }
        });
        match m3170 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3176 = {
                    let a3173 = match v0.clone() {
                        Adt2::C0(_, h3171, _, _, _, _, _, _, _, _, _) => {
                            h3171
                        }
                    };
                    let a3174 = match v3.clone() {
                        Adt18::C0(_, h3172, _) => {
                            h3172
                        }
                    };
                    let a3175 = 0u64;
                    nat_rem(a3173, a3174, a3175)
                };
                let a3177 = 0u64;
                if if nat_eq(a3176, a3177) {
                    if f196(match v0.clone() {
                        Adt2::C0(_, h3178, _, _, _, _, _, _, _, _, _) => {
                            h3178
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3179, _, _, _, _, _, _, _, _, _) => {
                            h3179
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3180, _, _, _, _, _, _, _, _) => {
                            h3180
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3181, _, _, _, _, _, _, _) => {
                            h3181
                        }
                    }) {
                        true
                    } else {
                        let a3183 = 4u64;
                        let a3184 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3182, _, _, _, _, _) => {
                                h3182
                            }
                        };
                        if if nat_le(a3183, a3184) {
                            let a3186 = match v3.clone() {
                                Adt18::C0(_, h3185, _) => {
                                    h3185
                                }
                            };
                            let a3187 = 1u64;
                            if nat_eq(a3186, a3187) {
                                true
                            } else {
                                let a3190 = match v3.clone() {
                                    Adt18::C0(_, _, h3188) => {
                                        h3188
                                    }
                                };
                                let a3191 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3189, _, _, _, _, _) => {
                                        h3189
                                    }
                                };
                                nat_le(a3190, a3191)
                            }
                        } else {
                            false
                        } {
                            if f23(v0.clone(), v2.clone())? {
                                f23(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f20(v0.clone(), match v3 {
                        Adt18::C0(_, h3192, _) => {
                            h3192
                        }
                    });
                    Ok(Adt10::C4(if f196(match v0.clone() {
                        Adt2::C0(_, h3193, _, _, _, _, _, _, _, _, _) => {
                            h3193
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3194, _, _, _, _, _, _, _, _, _) => {
                            h3194
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3195, _, _, _, _, _, _, _, _) => {
                            h3195
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3196, _, _, _, _, _, _, _) => {
                            h3196
                        }
                    }) {
                        0u64
                    } else {
                        f163(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3197, _, _, _, _, _, _, _, _, _, _) => {
                            h3197
                        }
                    }, f20(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f115(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f116(match v0.clone() {
        Adt2::C0(h3198, _, _, _, _, _, _, _, _, _, _) => {
            h3198
        }
    }) {
        let a3204 = {
            let a3201 = match v0.clone() {
                Adt2::C0(_, h3199, _, _, _, _, _, _, _, _, _) => {
                    h3199
                }
            };
            let a3202 = f197(match v0.clone() {
                Adt2::C0(h3200, _, _, _, _, _, _, _, _, _, _) => {
                    h3200
                }
            });
            let a3203 = 1u64;
            nat_rem(a3201, a3202, a3203)
        };
        let a3205 = 0u64;
        nat_eq(a3204, a3205)
    } else {
        false
    } {
        let a3207 = match v1.clone() {
            Adt2::C0(h3206, _, _, _, _, _, _, _, _, _, _) => {
                h3206
            }
        };
        let a3208 = 26u64;
        nat_eq(a3207, a3208)
    } else {
        false
    } {
        let a3210 = match v2.clone() {
            Adt2::C0(h3209, _, _, _, _, _, _, _, _, _, _) => {
                h3209
            }
        };
        let a3211 = 0u64;
        nat_eq(a3210, a3211)
    } else {
        false
    } {
        let a3214 = match v0.clone() {
            Adt2::C0(_, h3212, _, _, _, _, _, _, _, _, _) => {
                h3212
            }
        };
        let a3215 = match v2.clone() {
            Adt2::C0(_, h3213, _, _, _, _, _, _, _, _, _) => {
                h3213
            }
        };
        nat_eq(a3214, a3215)
    } else {
        false
    } {
        let a3218 = match v1.clone() {
            Adt2::C0(_, h3216, _, _, _, _, _, _, _, _, _) => {
                h3216
            }
        };
        let a3219 = match v0.clone() {
            Adt2::C0(_, _, h3217, _, _, _, _, _, _, _, _) => {
                h3217
            }
        };
        nat_le(a3218, a3219)
    } else {
        false
    } {
        let a3221 = match v0.clone() {
            Adt2::C0(_, _, _, h3220, _, _, _, _, _, _, _) => {
                h3220
            }
        };
        let a3222 = 1u64;
        nat_eq(a3221, a3222)
    } else {
        false
    } {
        let a3224 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3223, _, _, _, _, _, _) => {
                h3223
            }
        };
        let a3225 = 1u64;
        nat_eq(a3224, a3225)
    } else {
        false
    } {
        let a3227 = match v2.clone() {
            Adt2::C0(_, _, _, h3226, _, _, _, _, _, _, _) => {
                h3226
            }
        };
        let a3228 = 1u64;
        nat_eq(a3227, a3228)
    } else {
        false
    } {
        let a3230 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3229, _, _, _, _, _, _) => {
                h3229
            }
        };
        let a3231 = 1u64;
        nat_eq(a3230, a3231)
    } else {
        false
    } {
        let a3233 = match v1.clone() {
            Adt2::C0(_, _, h3232, _, _, _, _, _, _, _, _) => {
                h3232
            }
        };
        let a3234 = 1u64;
        nat_eq(a3233, a3234)
    } else {
        false
    } {
        let a3236 = match v1.clone() {
            Adt2::C0(_, _, _, h3235, _, _, _, _, _, _, _) => {
                h3235
            }
        };
        let a3237 = 1u64;
        nat_eq(a3236, a3237)
    } else {
        false
    } {
        let a3239 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3238, _, _, _, _, _, _) => {
                h3238
            }
        };
        let a3240 = 1u64;
        nat_eq(a3239, a3240)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3241, _, _, _, _, _, _, _, _, _) => {
                h3241
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3242, _, _, _, _, _, _, _, _) => {
                h3242
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3243, _, _, _, _, _, _, _) => {
                h3243
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3244, _, _, _, _, _, _) => {
                h3244
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3246 = match v1.clone() {
                Adt2::C0(_, h3245, _, _, _, _, _, _, _, _, _) => {
                    h3245
                }
            };
            let a3247 = 0u64;
            if nat_eq(a3246, a3247) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3249 = match v0.clone() {
                    Adt2::C0(_, h3248, _, _, _, _, _, _, _, _, _) => {
                        h3248
                    }
                };
                let a3250 = 2147483647u64;
                if if if if if if nat_le(a3249, a3250) {
                    let a3252 = match v1.clone() {
                        Adt2::C0(_, h3251, _, _, _, _, _, _, _, _, _) => {
                            h3251
                        }
                    };
                    let a3253 = 2147483647u64;
                    nat_le(a3252, a3253)
                } else {
                    false
                } {
                    let a3255 = 4u64;
                    let a3256 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3254, _, _, _, _, _) => {
                            h3254
                        }
                    };
                    nat_le(a3255, a3256)
                } else {
                    false
                } {
                    let a3262 = {
                        let a3259 = match v2.clone() {
                            Adt2::C0(_, h3257, _, _, _, _, _, _, _, _, _) => {
                                h3257
                            }
                        };
                        let a3260 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3258, _, _, _, _, _) => {
                                h3258
                            }
                        };
                        nat_mul(a3259, a3260)?
                    };
                    let a3263 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3261, _, _, _, _) => {
                            h3261
                        }
                    };
                    nat_le(a3262, a3263)
                } else {
                    false
                } {
                    f23(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f23(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a3276 = {
                            let a3267 = {
                                let a3265 = 2u64;
                                let a3266 = match v1.clone() {
                                    Adt2::C0(_, h3264, _, _, _, _, _, _, _, _, _) => {
                                        h3264
                                    }
                                };
                                nat_mul(a3265, a3266)?
                            };
                            let a3268 = f16(v2.clone(), 1u64)?;
                            nat_add(a3267, a3268)?
                        };
                        let a3277 = {
                            let a3274 = match v1.clone() {
                                Adt2::C0(_, h3269, _, _, _, _, _, _, _, _, _) => {
                                    h3269
                                }
                            };
                            let a3275 = {
                                let a3272 = 1u64;
                                let a3273 = f161(match v0.clone() {
                                    Adt2::C0(h3270, _, _, _, _, _, _, _, _, _, _) => {
                                        h3270
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3271, _, _, _, _, _, _, _, _, _) => {
                                        h3271
                                    }
                                })?;
                                nat_add(a3272, a3273)?
                            };
                            nat_mul(a3274, a3275)?
                        };
                        nat_add(a3276, a3277)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3278, _, _, _, _, _, _, _, _, _) => {
                            h3278
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3279, _, _, _, _, _, _, _, _, _) => {
                            h3279
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3280, _, _, _, _, _) => {
                            h3280
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3281, _, _, _, _) => {
                            h3281
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3282, _, _, _) => {
                            h3282
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3283, _, _) => {
                            h3283
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3284, _, _, _, _, _, _, _, _, _, _) => {
                            h3284
                        }
                    }, f20(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f116(v0: u64) -> bool {
    let a3285 = v0;
    let a3286 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3285, a3286) {
        true
    } else {
        let a3287 = v0;
        let a3288 = 1u64;
        nat_eq(a3287, a3288)
    } {
        true
    } else {
        let a3289 = v0;
        let a3290 = 2u64;
        nat_eq(a3289, a3290)
    } {
        true
    } else {
        let a3291 = v0;
        let a3292 = 3u64;
        nat_eq(a3291, a3292)
    } {
        true
    } else {
        let a3293 = v0;
        let a3294 = 6u64;
        nat_eq(a3293, a3294)
    } {
        true
    } else {
        let a3295 = v0;
        let a3296 = 7u64;
        nat_eq(a3295, a3296)
    } {
        true
    } else {
        let a3297 = v0;
        let a3298 = 8u64;
        nat_eq(a3297, a3298)
    } {
        true
    } else {
        let a3299 = v0;
        let a3300 = 10u64;
        nat_eq(a3299, a3300)
    } {
        true
    } else {
        let a3301 = v0;
        let a3302 = 11u64;
        nat_eq(a3301, a3302)
    } {
        true
    } else {
        let a3303 = v0;
        let a3304 = 12u64;
        nat_eq(a3303, a3304)
    } {
        true
    } else {
        let a3305 = v0;
        let a3306 = 13u64;
        nat_eq(a3305, a3306)
    } {
        true
    } else {
        let a3307 = v0;
        let a3308 = 14u64;
        nat_eq(a3307, a3308)
    } {
        true
    } else {
        let a3309 = v0;
        let a3310 = 16u64;
        nat_eq(a3309, a3310)
    } {
        true
    } else {
        let a3311 = v0;
        let a3312 = 17u64;
        nat_eq(a3311, a3312)
    } {
        true
    } else {
        let a3313 = v0;
        let a3314 = 18u64;
        nat_eq(a3313, a3314)
    } {
        true
    } else {
        let a3315 = v0;
        let a3316 = 19u64;
        nat_eq(a3315, a3316)
    } {
        true
    } else {
        let a3317 = v0;
        let a3318 = 20u64;
        nat_eq(a3317, a3318)
    } {
        true
    } else {
        let a3319 = v0;
        let a3320 = 21u64;
        nat_eq(a3319, a3320)
    } {
        true
    } else {
        let a3321 = v0;
        let a3322 = 22u64;
        nat_eq(a3321, a3322)
    } {
        true
    } else {
        let a3323 = v0;
        let a3324 = 23u64;
        nat_eq(a3323, a3324)
    } {
        true
    } else {
        let a3325 = v0;
        let a3326 = 26u64;
        nat_eq(a3325, a3326)
    } {
        true
    } else {
        let a3327 = v0;
        let a3328 = 29u64;
        nat_eq(a3327, a3328)
    } {
        true
    } else {
        let a3329 = v0;
        let a3330 = 30u64;
        nat_eq(a3329, a3330)
    } {
        true
    } else {
        let a3331 = v0;
        let a3332 = 34u64;
        nat_eq(a3331, a3332)
    } {
        true
    } else {
        let a3333 = v0;
        let a3334 = 35u64;
        nat_eq(a3333, a3334)
    } {
        true
    } else {
        let a3335 = v0;
        let a3336 = 39u64;
        nat_eq(a3335, a3336)
    } {
        true
    } else {
        let a3337 = v0;
        let a3338 = 40u64;
        nat_eq(a3337, a3338)
    } {
        true
    } else {
        let a3339 = v0;
        let a3340 = 41u64;
        nat_eq(a3339, a3340)
    } {
        true
    } else {
        let a3341 = v0;
        let a3342 = 42u64;
        nat_eq(a3341, a3342)
    } {
        true
    } else {
        let a3343 = v0;
        let a3344 = 43u64;
        nat_eq(a3343, a3344)
    }
}

pub fn f117(v0: u64) -> bool {
    if f108(v0) {
        true
    } else {
        let a3345 = v0;
        let a3346 = 43u64;
        nat_eq(a3345, a3346)
    }
}

pub fn f118(v0: u64) -> Adt17 {
    let a3347 = v0;
    let a3348 = 0u64;
    if nat_eq(a3347, a3348) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3349 = v0;
        let a3350 = 1u64;
        if nat_eq(a3349, a3350) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3351 = v0;
            let a3352 = 2u64;
            if nat_eq(a3351, a3352) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3353 = v0;
                let a3354 = 3u64;
                if nat_eq(a3353, a3354) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3355 = v0;
                    let a3356 = 6u64;
                    if nat_eq(a3355, a3356) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3357 = v0;
                        let a3358 = 7u64;
                        if nat_eq(a3357, a3358) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3359 = v0;
                            let a3360 = 8u64;
                            if nat_eq(a3359, a3360) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3361 = v0;
                                let a3362 = 9u64;
                                if nat_eq(a3361, a3362) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3363 = v0;
                                    let a3364 = 10u64;
                                    if nat_eq(a3363, a3364) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3365 = v0;
                                        let a3366 = 11u64;
                                        if nat_eq(a3365, a3366) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3367 = v0;
                                            let a3368 = 12u64;
                                            if nat_eq(a3367, a3368) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3369 = v0;
                                                let a3370 = 13u64;
                                                if nat_eq(a3369, a3370) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3371 = v0;
                                                    let a3372 = 14u64;
                                                    if nat_eq(a3371, a3372) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3373 = v0;
                                                        let a3374 = 15u64;
                                                        if nat_eq(a3373, a3374) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3375 = v0;
                                                            let a3376 = 16u64;
                                                            if nat_eq(a3375, a3376) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3377 = v0;
                                                                let a3378 = 17u64;
                                                                if nat_eq(a3377, a3378) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3379 = v0;
                                                                    let a3380 = 18u64;
                                                                    if nat_eq(a3379, a3380) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3381 = v0;
                                                                        let a3382 = 19u64;
                                                                        if nat_eq(a3381, a3382) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3383 = v0;
                                                                            let a3384 = 20u64;
                                                                            if nat_eq(a3383, a3384) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3385 = v0;
                                                                                let a3386 = 21u64;
                                                                                if nat_eq(a3385, a3386) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3387 = v0;
                                                                                    let a3388 = 22u64;
                                                                                    if nat_eq(a3387, a3388) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3389 = v0;
                                                                                        let a3390 = 23u64;
                                                                                        if nat_eq(a3389, a3390) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3391 = v0;
                                                                                            let a3392 = 24u64;
                                                                                            if nat_eq(a3391, a3392) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3393 = v0;
                                                                                                let a3394 = 25u64;
                                                                                                if nat_eq(a3393, a3394) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3395 = v0;
                                                                                                    let a3396 = 26u64;
                                                                                                    if nat_eq(a3395, a3396) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3397 = v0;
                                                                                                        let a3398 = 27u64;
                                                                                                        if nat_eq(a3397, a3398) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3399 = v0;
                                                                                                            let a3400 = 28u64;
                                                                                                            if nat_eq(a3399, a3400) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3401 = v0;
                                                                                                                let a3402 = 29u64;
                                                                                                                if nat_eq(a3401, a3402) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3403 = v0;
                                                                                                                    let a3404 = 30u64;
                                                                                                                    if nat_eq(a3403, a3404) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3405 = v0;
                                                                                                                        let a3406 = 34u64;
                                                                                                                        if nat_eq(a3405, a3406) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3407 = v0;
                                                                                                                            let a3408 = 35u64;
                                                                                                                            if nat_eq(a3407, a3408) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3409 = v0;
                                                                                                                                let a3410 = 39u64;
                                                                                                                                if nat_eq(a3409, a3410) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3411 = v0;
                                                                                                                                    let a3412 = 40u64;
                                                                                                                                    if nat_eq(a3411, a3412) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3413 = v0;
                                                                                                                                        let a3414 = 41u64;
                                                                                                                                        if nat_eq(a3413, a3414) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3415 = v0;
                                                                                                                                            let a3416 = 42u64;
                                                                                                                                            if nat_eq(a3415, a3416) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3417 = v0;
                                                                                                                                                let a3418 = 43u64;
                                                                                                                                                if nat_eq(a3417, a3418) {
                                                                                                                                                    Adt17::C1(Adt18::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt17::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f119(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3421 = match v1.clone() {
        Adt2::C0(h3419, _, _, _, _, _, _, _, _, _, _) => {
            h3419
        }
    };
    let a3422 = match v3.clone() {
        Adt2::C0(h3420, _, _, _, _, _, _, _, _, _, _) => {
            h3420
        }
    };
    if if nat_eq(a3421, a3422) {
        let a3423 = v0;
        let a3424 = 33u64;
        if nat_eq(a3423, a3424) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3426 = f118(match v1.clone() {
            Adt2::C0(h3425, _, _, _, _, _, _, _, _, _, _) => {
                h3425
            }
        });
        match m3426 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3427, _) => {
                        h3427
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3432 = {
                    let a3429 = match v1.clone() {
                        Adt2::C0(_, h3428, _, _, _, _, _, _, _, _, _) => {
                            h3428
                        }
                    };
                    let a3430 = v5;
                    let a3431 = 1u64;
                    nat_rem(a3429, a3430, a3431)
                };
                let a3433 = 0u64;
                if if if nat_eq(a3432, a3433) {
                    let a3438 = {
                        let a3435 = match v3.clone() {
                            Adt2::C0(_, h3434, _, _, _, _, _, _, _, _, _) => {
                                h3434
                            }
                        };
                        let a3436 = v5;
                        let a3437 = 1u64;
                        nat_rem(a3435, a3436, a3437)
                    };
                    let a3439 = 0u64;
                    nat_eq(a3438, a3439)
                } else {
                    false
                } {
                    let a3440 = v6;
                    let a3441 = f16(v3.clone(), v5)?;
                    nat_eq(a3440, a3441)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3442) => {
                            h3442
                        }
                    }, f20(v1, v5), f20(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f120(v0: u64, v1: u64) -> u64 {
    let a3443 = v0;
    let a3444 = 0u64;
    if if nat_eq(a3443, a3444) {
        true
    } else {
        let a3445 = v1;
        let a3446 = 0u64;
        nat_eq(a3445, a3446)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f121(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f34(v4, v5, v6, v7) {
        f34(v0, v1, v2, v3)
    } else {
        let a3450 = {
            let a3447 = v0;
            let a3448 = v4;
            let a3449 = 1u64;
            nat_rem(a3447, a3448, a3449)
        };
        let a3451 = 0u64;
        if if if nat_eq(a3450, a3451) {
            let a3455 = {
                let a3452 = v1;
                let a3453 = v5;
                let a3454 = 1u64;
                nat_rem(a3452, a3453, a3454)
            };
            let a3456 = 0u64;
            nat_eq(a3455, a3456)
        } else {
            false
        } {
            let a3460 = {
                let a3457 = v2;
                let a3458 = v6;
                let a3459 = 1u64;
                nat_rem(a3457, a3458, a3459)
            };
            let a3461 = 0u64;
            nat_eq(a3460, a3461)
        } else {
            false
        } {
            let a3465 = {
                let a3462 = v3;
                let a3463 = v7;
                let a3464 = 1u64;
                nat_rem(a3462, a3463, a3464)
            };
            let a3466 = 0u64;
            nat_eq(a3465, a3466)
        } else {
            false
        }
    }
}

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3467 = v0;
    let a3468 = v1;
    if nat_eq(a3467, a3468) {
        let a3471 = v4;
        let a3472 = {
            let a3469 = v2;
            let a3470 = v3;
            nat_add(a3469, a3470)?
        };
        Ok(nat_eq(a3471, a3472))
    } else {
        let a3473 = v2;
        let a3474 = v3;
        if nat_eq(a3473, a3474) {
            let a3475 = v4;
            let a3476 = v2;
            Ok(nat_eq(a3475, a3476))
        } else {
            Ok(false)
        }
    }
}

pub fn f123(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3479 = {
            let a3477 = v0;
            let a3478 = f137(v1);
            nat_add(a3477, a3478)?
        };
        let a3480 = f137(v2);
        nat_add(a3479, a3480)?
    };
    let v5: u64 = {
        let a3481 = f136(v1);
        let a3482 = f136(v2);
        nat_add(a3481, a3482)?
    };
    let a3483 = v1;
    let a3484 = 4294967295u64;
    if if nat_le(a3483, a3484) {
        let a3485 = v2;
        let a3486 = 4294967295u64;
        nat_le(a3485, a3486)
    } else {
        false
    } {
        let a3487 = v5;
        let a3488 = v4;
        if nat_le(a3487, a3488) {
            let a3491 = v3;
            let a3492 = {
                let a3489 = v4;
                let a3490 = v5;
                nat_sub(a3489, a3490)
            };
            Ok(nat_eq(a3491, a3492))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f124(v0: u64, v1: u64) -> R<bool> {
    let a3493 = v1;
    let a3494 = 4294967295u64;
    if nat_le(a3493, a3494) {
        let a3497 = {
            let a3495 = f137(v1);
            let a3496 = f136(v1);
            nat_add(a3495, a3496)?
        };
        let a3498 = v0;
        Ok(nat_lt(a3497, a3498))
    } else {
        Ok(false)
    }
}

pub fn f125(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f198(v0.clone(), false)?;
    let v6: u64 = f198(v0, true)?;
    let a3499 = v6;
    let a3500 = v1;
    if if nat_le(a3499, a3500) {
        let a3507 = {
            let a3505 = {
                let a3501 = v1;
                let a3502 = v5;
                nat_add(a3501, a3502)?
            };
            let a3506 = {
                let a3503 = v3;
                let a3504 = 0u64;
                if nat_eq(a3503, a3504) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3505, a3506)?
        };
        let a3508 = v2;
        nat_le(a3507, a3508)
    } else {
        false
    } {
        if v4 {
            let a3513 = {
                let a3511 = {
                    let a3509 = v1;
                    let a3510 = v5;
                    nat_add(a3509, a3510)?
                };
                let a3512 = v6;
                nat_sub(a3511, a3512)
            };
            let a3514 = v2;
            Ok(nat_lt(a3513, a3514))
        } else {
            let a3519 = {
                let a3517 = {
                    let a3515 = v1;
                    let a3516 = v5;
                    nat_add(a3515, a3516)?
                };
                let a3518 = v6;
                nat_sub(a3517, a3518)
            };
            let a3520 = v2;
            Ok(nat_le(a3519, a3520))
        }
    } else {
        Ok(false)
    }
}

pub fn f126(v0: u64, v1: u64) -> bool {
    let a3521 = v0;
    let a3522 = 0u64;
    if if nat_eq(a3521, a3522) {
        true
    } else {
        let a3523 = v0;
        let a3524 = 1u64;
        nat_eq(a3523, a3524)
    } {
        let a3525 = v1;
        let a3526 = 4294967296u64;
        nat_lt(a3525, a3526)
    } else {
        false
    }
}

pub fn f127(v0: u64, v1: u64) -> R<u64> {
    let a3531 = {
        let a3529 = v0;
        let a3530 = {
            let a3527 = v1;
            let a3528 = 1u64;
            nat_sub(a3527, a3528)
        };
        nat_add(a3529, a3530)?
    };
    let a3532 = v1;
    let a3533 = 0u64;
    Ok(nat_quot(a3531, a3532, a3533))
}

pub fn f128(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h3534, _, _, _, _, _, _, _, _, _) => {
            h3534
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3535, _, _, _, _, _, _, _, _) => {
            h3535
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3536, _, _, _, _, _, _, _) => {
            h3536
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3537, _, _, _, _, _, _) => {
            h3537
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3540 = match v1.clone() {
                Adt2::C0(_, h3538, _, _, _, _, _, _, _, _, _) => {
                    h3538
                }
            };
            let a3541 = match v1.clone() {
                Adt2::C0(_, _, h3539, _, _, _, _, _, _, _, _) => {
                    h3539
                }
            };
            nat_mul(a3540, a3541)?
        };
        let v5: u64 = {
            let a3543 = v4;
            let a3544 = match v1.clone() {
                Adt2::C0(_, _, _, h3542, _, _, _, _, _, _, _) => {
                    h3542
                }
            };
            nat_mul(a3543, a3544)?
        };
        let a3557 = {
            let a3555 = {
                let a3546 = match v0.clone() {
                    Adt2::C0(_, h3545, _, _, _, _, _, _, _, _, _) => {
                        h3545
                    }
                };
                let a3547 = 1u64;
                nat_sub(a3546, a3547)
            };
            let a3556 = {
                let a3553 = f199(match v0.clone() {
                    Adt2::C0(_, _, h3548, _, _, _, _, _, _, _, _) => {
                        h3548
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3549, _, _, _, _, _, _, _, _, _) => {
                        h3549
                    }
                })?;
                let a3554 = f199(match v0 {
                    Adt2::C0(_, _, _, h3550, _, _, _, _, _, _, _) => {
                        h3550
                    }
                }, v2, {
                    let a3551 = v3;
                    let a3552 = v5;
                    nat_mul(a3551, a3552)?
                }, v4)?;
                nat_add(a3553, a3554)?
            };
            nat_add(a3555, a3556)?
        };
        let a3558 = f16(v1, 1u64)?;
        Ok(nat_lt(a3557, a3558))
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3559 = v0;
    let a3560 = 2147483648u64;
    if if if if nat_lt(a3559, a3560) {
        let a3561 = v1;
        let a3562 = 2147483648u64;
        nat_lt(a3561, a3562)
    } else {
        false
    } {
        let a3565 = {
            let a3563 = v0;
            let a3564 = v1;
            nat_mul(a3563, a3564)?
        };
        let a3566 = 2147483648u64;
        nat_lt(a3565, a3566)
    } else {
        false
    } {
        let a3571 = {
            let a3569 = {
                let a3567 = v0;
                let a3568 = 1u64;
                nat_sub(a3567, a3568)
            };
            let a3570 = v2;
            nat_mul(a3569, a3570)?
        };
        let a3572 = 2147483648u64;
        nat_lt(a3571, a3572)
    } else {
        false
    } {
        let a3577 = {
            let a3575 = {
                let a3573 = v1;
                let a3574 = 1u64;
                nat_sub(a3573, a3574)
            };
            let a3576 = v2;
            nat_mul(a3575, a3576)?
        };
        let a3578 = 2147483648u64;
        Ok(nat_lt(a3577, a3578))
    } else {
        Ok(false)
    }
}

pub fn f130(v0: u64) -> R<u64> {
    let a3581 = 3u64;
    let a3582 = {
        let a3579 = v0;
        let a3580 = 3u64;
        nat_mul(a3579, a3580)?
    };
    nat_add(a3581, a3582)
}

pub fn f131(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h3583, _, _, _, _, _, _, _) => {
            h3583
        }
    }, match v0.clone() {
        Adt3::C0(_, h3584, _, _, _, _, _, _) => {
            h3584
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3585, _, _, _, _, _) => {
            h3585
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3586, _, _, _, _) => {
            h3586
        }
    }) {
        Ok(0u64)
    } else {
        let a3595 = match v0.clone() {
            Adt3::C0(h3587, _, _, _, _, _, _, _) => {
                h3587
            }
        };
        let a3596 = {
            let a3593 = match v0.clone() {
                Adt3::C0(_, h3588, _, _, _, _, _, _) => {
                    h3588
                }
            };
            let a3594 = {
                let a3591 = match v0.clone() {
                    Adt3::C0(_, _, h3589, _, _, _, _, _) => {
                        h3589
                    }
                };
                let a3592 = match v0 {
                    Adt3::C0(_, _, _, h3590, _, _, _, _) => {
                        h3590
                    }
                };
                nat_mul(a3591, a3592)?
            };
            nat_mul(a3593, a3594)?
        };
        nat_mul(a3595, a3596)
    }
}

pub fn f132(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3597 = v1;
    let a3598 = 13u64;
    if if nat_eq(a3597, a3598) {
        f34(match v0.clone() {
            Adt3::C0(h3599, _, _, _, _, _, _, _) => {
                h3599
            }
        }, match v0.clone() {
            Adt3::C0(_, h3600, _, _, _, _, _, _) => {
                h3600
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3601, _, _, _, _, _) => {
                h3601
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3602, _, _, _, _) => {
                h3602
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3608 = match v0.clone() {
                Adt3::C0(_, h3603, _, _, _, _, _, _) => {
                    h3603
                }
            };
            let a3609 = {
                let a3606 = match v0.clone() {
                    Adt3::C0(_, _, h3604, _, _, _, _, _) => {
                        h3604
                    }
                };
                let a3607 = match v0.clone() {
                    Adt3::C0(_, _, _, h3605, _, _, _, _) => {
                        h3605
                    }
                };
                nat_mul(a3606, a3607)?
            };
            nat_mul(a3608, a3609)?
        };
        let a3610 = v1;
        let a3611 = 13u64;
        if nat_eq(a3610, a3611) {
            let a3620 = 2u64;
            let a3621 = if f200(v0.clone(), v2) {
                let a3613 = v3;
                let a3614 = f201(match v0 {
                    Adt3::C0(h3612, _, _, _, _, _, _, _) => {
                        h3612
                    }
                }, false)?;
                nat_mul(a3613, a3614)?
            } else {
                let a3618 = v3;
                let a3619 = {
                    let a3616 = match v0 {
                        Adt3::C0(h3615, _, _, _, _, _, _, _) => {
                            h3615
                        }
                    };
                    let a3617 = 2u64;
                    nat_mul(a3616, a3617)?
                };
                nat_mul(a3618, a3619)?
            };
            nat_add(a3620, a3621)
        } else {
            let a3625 = v3;
            let a3626 = f201(match v0 {
                Adt3::C0(h3622, _, _, _, _, _, _, _) => {
                    h3622
                }
            }, {
                let a3623 = v1;
                let a3624 = 16u64;
                nat_eq(a3623, a3624)
            })?;
            nat_mul(a3625, a3626)
        }
    }
}

pub fn f133(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3627 = v0;
    let a3628 = 30u64;
    if if nat_eq(a3627, a3628) {
        let a3631 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3629, _, _, _, _) => {
                h3629
            }
        };
        let a3632 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3630, _, _, _, _, _) => {
                h3630
            }
        };
        nat_lt(a3631, a3632)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3633 = v3;
        match m3633 {
            None => {
                f202(v0, v1, match v2.clone() {
                    Adt2::C0(h3634, _, _, _, _, _, _, _, _, _, _) => {
                        h3634
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3635, _, _, _, _, _, _, _, _, _, _) => {
                        h3635
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3636, _, _, _, _, _, _, _, _, _) => {
                        h3636
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3637, _, _, _, _, _, _, _, _) => {
                        h3637
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3638, _, _, _, _, _, _, _) => {
                        h3638
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3639, _, _, _, _, _, _) => {
                        h3639
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3640, _, _, _, _, _, _, _, _, _) => {
                        h3640
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3641, _, _, _, _, _, _, _, _) => {
                        h3641
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3642, _, _, _, _, _, _, _) => {
                        h3642
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3643, _, _, _, _, _, _) => {
                        h3643
                    }
                })
            }
            Some(v5) => {
                f202(v0, v1, match v2.clone() {
                    Adt2::C0(h3644, _, _, _, _, _, _, _, _, _, _) => {
                        h3644
                    }
                }, match v5.clone() {
                    Adt2::C0(h3645, _, _, _, _, _, _, _, _, _, _) => {
                        h3645
                    }
                }, match v4.clone() {
                    Adt2::C0(h3646, _, _, _, _, _, _, _, _, _, _) => {
                        h3646
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3647, _, _, _, _, _, _, _, _, _) => {
                        h3647
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3648, _, _, _, _, _, _, _, _) => {
                        h3648
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3649, _, _, _, _, _, _, _) => {
                        h3649
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3650, _, _, _, _, _, _) => {
                        h3650
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3651, _, _, _, _, _, _, _, _, _) => {
                        h3651
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3652, _, _, _, _, _, _, _, _) => {
                        h3652
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3653, _, _, _, _, _, _, _) => {
                        h3653
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3654, _, _, _, _, _, _) => {
                        h3654
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3655, _, _, _, _, _, _, _, _, _) => {
                        h3655
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3656, _, _, _, _, _, _, _, _) => {
                        h3656
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3657, _, _, _, _, _, _, _) => {
                        h3657
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3658, _, _, _, _, _, _) => {
                        h3658
                    }
                })
            }
        }
    }
}

pub fn f134(v0: u64, v1: u64) -> u64 {
    let a3659 = v0;
    let a3660 = 23u64;
    if nat_eq(a3659, a3660) {
        let a3661 = v1;
        let a3662 = 0u64;
        if nat_eq(a3661, a3662) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3675 = {
        let a3671 = {
            let a3667 = {
                let a3663 = v0;
                let a3664 = v4;
                nat_mul(a3663, a3664)?
            };
            let a3668 = {
                let a3665 = v1;
                let a3666 = v5;
                nat_mul(a3665, a3666)?
            };
            nat_add(a3667, a3668)?
        };
        let a3672 = {
            let a3669 = v2;
            let a3670 = v6;
            nat_mul(a3669, a3670)?
        };
        nat_add(a3671, a3672)?
    };
    let a3676 = {
        let a3673 = v3;
        let a3674 = v7;
        nat_mul(a3673, a3674)?
    };
    nat_add(a3675, a3676)
}

pub fn f136(v0: u64) -> u64 {
    let a3677 = v0;
    let a3678 = 2147483648u64;
    if nat_lt(a3677, a3678) {
        0u64
    } else {
        let a3679 = 4294967296u64;
        let a3680 = v0;
        nat_sub(a3679, a3680)
    }
}

pub fn f137(v0: u64) -> u64 {
    let a3681 = v0;
    let a3682 = 2147483648u64;
    if nat_lt(a3681, a3682) {
        v0
    } else {
        0u64
    }
}

pub fn f138(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3683 = v7;
        let a3684 = 6u64;
        if nat_eq(a3683, a3684) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f92(v3, v4), f92(v5, v6), f93(v7, v8), f203(v1), f203(v9))
}

pub fn f139(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3685 = v0;
    match m3685 {
        Adt4::C0 => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C1 => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C2(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C3(_, _, _, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C4(_) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C5(_, _, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C6(_, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C7(_) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C8(_) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C9(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C10(_, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C11(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C12(_, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C13(v33, v34, v35, v36) => {
            Adt11::C4(v33, v34, v35, v36)
        }
        Adt4::C14(v37, _, _) => {
            let a3686 = 13u64;
            let a3687 = v37;
            if nat_le(a3686, a3687) {
                Adt11::C7(v37, v1, v2)
            } else {
                Adt11::C3(v1, v2, true)
            }
        }
        Adt4::C15 => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C16(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C17 => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C18(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C19(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C20(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C21 => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C22(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C23(_, _, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C24(_) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C25(_, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C26(_, _, _, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C27(v57) => {
            Adt11::C2(match v57.clone() {
                Adt5::C0(_, _, _, _, _, h3688, _, _) => {
                    h3688
                }
            }, match v57 {
                Adt5::C0(_, _, _, _, _, _, _, h3689) => {
                    h3689
                }
            }, true)
        }
        Adt4::C28(_) => {
            Adt11::C6
        }
        Adt4::C29(_) => {
            Adt11::C6
        }
        Adt4::C30(_, _) => {
            Adt11::C6
        }
        Adt4::C31(_, _) => {
            Adt11::C2(v1, v2, true)
        }
        Adt4::C32(_) => {
            Adt11::C6
        }
    }
}

pub fn f140(v0: Adt4) -> u64 {
    let m3690 = v0;
    match m3690 {
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
        Adt4::C14(v35, _, _) => {
            v35
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
        Adt4::C28(_) => {
            0u64
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, _) => {
            0u64
        }
        Adt4::C31(_, _) => {
            0u64
        }
        Adt4::C32(_) => {
            0u64
        }
    }
}

pub fn f141(v0: Adt4, v1: u64) -> u64 {
    let m3691 = v0;
    match m3691 {
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
        Adt4::C16(_, v40) => {
            v40
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v43, _) => {
            let a3692 = v1;
            let a3693 = 6u64;
            if nat_eq(a3692, a3693) {
                v43
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
        Adt4::C26(v57, _, _, _) => {
            let a3702 = v1;
            let a3703 = 6u64;
            if if if nat_eq(a3702, a3703) {
                true
            } else {
                let a3704 = v1;
                let a3705 = 7u64;
                nat_eq(a3704, a3705)
            } {
                let a3706 = v57;
                let a3707 = 30u64;
                nat_eq(a3706, a3707)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v56) => {
            match v56 {
                Adt5::C0(_, _, _, _, _, _, _, h3701) => {
                    h3701
                }
            }
        }
        Adt4::C28(v52) => {
            let a3694 = v1;
            let a3695 = 6u64;
            if nat_eq(a3694, a3695) {
                match v52 {
                    Adt7::C0(_, _, _, _, h3696) => {
                        h3696
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v55) => {
            let a3697 = v1;
            let a3698 = 6u64;
            if if nat_eq(a3697, a3698) {
                true
            } else {
                let a3699 = v1;
                let a3700 = 7u64;
                nat_eq(a3699, a3700)
            } {
                v55
            } else {
                0u64
            }
        }
        Adt4::C31(_, _) => {
            0u64
        }
        Adt4::C32(_) => {
            0u64
        }
    }
}

pub fn f142(v0: Adt4, v1: u64) -> Adt11 {
    let a3708 = v1;
    let a3709 = 4u64;
    if nat_lt(a3708, a3709) {
        Adt11::C0(v1)
    } else {
        let a3710 = v1;
        let a3711 = 4u64;
        if nat_eq(a3710, a3711) {
            Adt11::C1(0u64)
        } else {
            let a3712 = v1;
            let a3713 = 5u64;
            if nat_eq(a3712, a3713) {
                Adt11::C1(3u64)
            } else {
                let a3714 = v1;
                let a3715 = 6u64;
                if nat_eq(a3714, a3715) {
                    Adt11::C2(f141(v0, v1), 0u64, false)
                } else {
                    let a3716 = v1;
                    let a3717 = 7u64;
                    if nat_eq(a3716, a3717) {
                        Adt11::C2(0u64, f141(v0, v1), false)
                    } else {
                        let a3718 = v1;
                        let a3719 = 16u64;
                        if nat_lt(a3718, a3719) {
                            Adt11::C1({
                                let a3720 = v1;
                                let a3721 = 8u64;
                                nat_sub(a3720, a3721)
                            })
                        } else {
                            let a3722 = v1;
                            let a3723 = 20u64;
                            if nat_lt(a3722, a3723) {
                                Adt11::C5({
                                    let a3724 = v1;
                                    let a3725 = 16u64;
                                    nat_sub(a3724, a3725)
                                })
                            } else {
                                let a3726 = v1;
                                let a3727 = 20u64;
                                if nat_eq(a3726, a3727) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3728 = v1;
                                    let a3729 = 21u64;
                                    if nat_eq(a3728, a3729) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3730 = v1;
                                        let a3731 = 22u64;
                                        if nat_eq(a3730, a3731) {
                                            Adt11::C0(5u64)
                                        } else {
                                            Adt11::C6
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f143(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3733 = v0;
        let a3734 = match v1.clone() {
            Adt3::C0(h3732, _, _, _, _, _, _, _) => {
                h3732
            }
        };
        let a3735 = 0u64;
        nat_quot(a3733, a3734, a3735)
    };
    let v3: u64 = {
        let a3737 = v2;
        let a3738 = match v1.clone() {
            Adt3::C0(_, h3736, _, _, _, _, _, _) => {
                h3736
            }
        };
        let a3739 = 0u64;
        nat_quot(a3737, a3738, a3739)
    };
    f135({
        let a3741 = v0;
        let a3742 = match v1.clone() {
            Adt3::C0(h3740, _, _, _, _, _, _, _) => {
                h3740
            }
        };
        let a3743 = 0u64;
        nat_rem(a3741, a3742, a3743)
    }, {
        let a3745 = v2;
        let a3746 = match v1.clone() {
            Adt3::C0(_, h3744, _, _, _, _, _, _) => {
                h3744
            }
        };
        let a3747 = 0u64;
        nat_rem(a3745, a3746, a3747)
    }, {
        let a3749 = v3;
        let a3750 = match v1.clone() {
            Adt3::C0(_, _, h3748, _, _, _, _, _) => {
                h3748
            }
        };
        let a3751 = 0u64;
        nat_rem(a3749, a3750, a3751)
    }, {
        let a3753 = v3;
        let a3754 = match v1.clone() {
            Adt3::C0(_, _, h3752, _, _, _, _, _) => {
                h3752
            }
        };
        let a3755 = 0u64;
        nat_quot(a3753, a3754, a3755)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3756, _, _, _) => {
            h3756
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3757, _, _) => {
            h3757
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3758, _) => {
            h3758
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3759) => {
            h3759
        }
    })
}

pub fn f144(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3777 = {
        let a3772 = f204(v2, {
            let a3763 = {
                let a3760 = v0;
                let a3761 = v1;
                let a3762 = 0u64;
                nat_quot(a3760, a3761, a3762)
            };
            let a3764 = v2;
            let a3765 = 0u64;
            nat_rem(a3763, a3764, a3765)
        }, {
            let a3769 = {
                let a3766 = v0;
                let a3767 = v1;
                let a3768 = 0u64;
                nat_quot(a3766, a3767, a3768)
            };
            let a3770 = v2;
            let a3771 = 0u64;
            nat_quot(a3769, a3770, a3771)
        })?;
        let a3773 = v1;
        nat_mul(a3772, a3773)?
    };
    let a3778 = {
        let a3774 = v0;
        let a3775 = v1;
        let a3776 = 0u64;
        nat_rem(a3774, a3775, a3776)
    };
    nat_add(a3777, a3778)
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3780 = v0;
        let a3781 = match v2.clone() {
            Adt3::C0(h3779, _, _, _, _, _, _, _) => {
                h3779
            }
        };
        let a3782 = 0u64;
        nat_quot(a3780, a3781, a3782)
    };
    let v4: u64 = {
        let a3784 = v3;
        let a3785 = match v2.clone() {
            Adt3::C0(_, h3783, _, _, _, _, _, _) => {
                h3783
            }
        };
        let a3786 = 0u64;
        nat_quot(a3784, a3785, a3786)
    };
    f205({
        let a3788 = v0;
        let a3789 = match v2.clone() {
            Adt3::C0(h3787, _, _, _, _, _, _, _) => {
                h3787
            }
        };
        let a3790 = 0u64;
        nat_rem(a3788, a3789, a3790)
    }, {
        let a3792 = v3;
        let a3793 = match v2.clone() {
            Adt3::C0(_, h3791, _, _, _, _, _, _) => {
                h3791
            }
        };
        let a3794 = 0u64;
        nat_rem(a3792, a3793, a3794)
    }, {
        let a3796 = v4;
        let a3797 = match v2.clone() {
            Adt3::C0(_, _, h3795, _, _, _, _, _) => {
                h3795
            }
        };
        let a3798 = 0u64;
        nat_rem(a3796, a3797, a3798)
    }, {
        let a3800 = v4;
        let a3801 = match v2 {
            Adt3::C0(_, _, h3799, _, _, _, _, _) => {
                h3799
            }
        };
        let a3802 = 0u64;
        nat_quot(a3800, a3801, a3802)
    }, match v1.clone() {
        Adt3::C0(h3803, _, _, _, _, _, _, _) => {
            h3803
        }
    }, match v1.clone() {
        Adt3::C0(_, h3804, _, _, _, _, _, _) => {
            h3804
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3805, _, _, _, _, _) => {
            h3805
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3806, _, _, _, _) => {
            h3806
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3807, _, _, _) => {
            h3807
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3808, _, _) => {
            h3808
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3809, _) => {
            h3809
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3810) => {
            h3810
        }
    })
}

pub fn f146(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3812 = v0;
        let a3813 = match v3.clone() {
            Adt3::C0(h3811, _, _, _, _, _, _, _) => {
                h3811
            }
        };
        let a3814 = 0u64;
        nat_quot(a3812, a3813, a3814)
    };
    let v6: u64 = {
        let a3816 = v5;
        let a3817 = match v3.clone() {
            Adt3::C0(_, h3815, _, _, _, _, _, _) => {
                h3815
            }
        };
        let a3818 = 0u64;
        nat_quot(a3816, a3817, a3818)
    };
    let v7: u64 = {
        let a3820 = v0;
        let a3821 = match v3.clone() {
            Adt3::C0(h3819, _, _, _, _, _, _, _) => {
                h3819
            }
        };
        let a3822 = 0u64;
        nat_rem(a3820, a3821, a3822)
    };
    let v8: u64 = {
        let a3824 = v5;
        let a3825 = match v3.clone() {
            Adt3::C0(_, h3823, _, _, _, _, _, _) => {
                h3823
            }
        };
        let a3826 = 0u64;
        nat_rem(a3824, a3825, a3826)
    };
    let v9: u64 = {
        let a3828 = v6;
        let a3829 = match v3.clone() {
            Adt3::C0(_, _, h3827, _, _, _, _, _) => {
                h3827
            }
        };
        let a3830 = 0u64;
        nat_rem(a3828, a3829, a3830)
    };
    let v10: u64 = {
        let a3832 = v6;
        let a3833 = match v3.clone() {
            Adt3::C0(_, _, h3831, _, _, _, _, _) => {
                h3831
            }
        };
        let a3834 = 0u64;
        nat_quot(a3832, a3833, a3834)
    };
    let v11: u64 = {
        let a3835 = v4;
        let a3836 = 0u64;
        if nat_eq(a3835, a3836) {
            match v1.clone() {
                Adt3::C0(h3837, _, _, _, _, _, _, _) => {
                    h3837
                }
            }
        } else {
            let a3838 = v4;
            let a3839 = 1u64;
            if nat_eq(a3838, a3839) {
                match v1.clone() {
                    Adt3::C0(_, h3840, _, _, _, _, _, _) => {
                        h3840
                    }
                }
            } else {
                let a3841 = v4;
                let a3842 = 2u64;
                if nat_eq(a3841, a3842) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3843, _, _, _, _, _) => {
                            h3843
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3844, _, _, _, _) => {
                            h3844
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3851 = v11;
        let a3852 = {
            let a3845 = v4;
            let a3846 = 0u64;
            if nat_eq(a3845, a3846) {
                v7
            } else {
                let a3847 = v4;
                let a3848 = 1u64;
                if nat_eq(a3847, a3848) {
                    v8
                } else {
                    let a3849 = v4;
                    let a3850 = 2u64;
                    if nat_eq(a3849, a3850) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3851, a3852)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f135(if if v12 {
        let a3853 = v4;
        let a3854 = 0u64;
        nat_eq(a3853, a3854)
    } else {
        false
    } {
        let a3855 = v7;
        let a3856 = v11;
        nat_sub(a3855, a3856)
    } else {
        v7
    }, if if v12 {
        let a3857 = v4;
        let a3858 = 1u64;
        nat_eq(a3857, a3858)
    } else {
        false
    } {
        let a3859 = v8;
        let a3860 = v11;
        nat_sub(a3859, a3860)
    } else {
        v8
    }, if if v12 {
        let a3861 = v4;
        let a3862 = 2u64;
        nat_eq(a3861, a3862)
    } else {
        false
    } {
        let a3863 = v9;
        let a3864 = v11;
        nat_sub(a3863, a3864)
    } else {
        v9
    }, if if v12 {
        let a3865 = v4;
        let a3866 = 3u64;
        nat_eq(a3865, a3866)
    } else {
        false
    } {
        let a3867 = v10;
        let a3868 = v11;
        nat_sub(a3867, a3868)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3869, _, _, _) => {
            h3869
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3870, _, _) => {
            h3870
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3871, _) => {
            h3871
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3872) => {
            h3872
        }
    })?, f143(v0, v3)?))
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f71(v0, v2.clone(), 0u64);
    let v9: u64 = f71(v0, v2.clone(), 1u64);
    let v10: u64 = f71(v0, v2.clone(), 2u64);
    let v11: u64 = f71(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f70(v1.clone(), f206(v8, v3, match v1.clone() {
            Adt3::C0(h3873, _, _, _, _, _, _, _) => {
                h3873
            }
        })?, f206(v9, v4, match v1.clone() {
            Adt3::C0(_, h3874, _, _, _, _, _, _) => {
                h3874
            }
        })?, f206(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3875, _, _, _, _, _) => {
                h3875
            }
        })?, f206(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3876, _, _, _, _) => {
                h3876
            }
        })?)?, f143(v0, v2)?))
    } else {
        if if if if f207(v8, v3, match v1.clone() {
            Adt3::C0(h3877, _, _, _, _, _, _, _) => {
                h3877
            }
        })? {
            f207(v9, v4, match v1.clone() {
                Adt3::C0(_, h3878, _, _, _, _, _, _) => {
                    h3878
                }
            })?
        } else {
            false
        } {
            f207(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3879, _, _, _, _, _) => {
                    h3879
                }
            })?
        } else {
            false
        } {
            f207(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3880, _, _, _, _) => {
                    h3880
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f70(v1, f208(v8, v3)?, f208(v9, v4)?, f208(v10, v5)?, f208(v11, v6)?)?, f143(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f143(v0, v2)?))
        }
    }
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f71(v0, v2.clone(), 0u64);
    let v5: u64 = f71(v0, v2.clone(), 1u64);
    let v6: u64 = f71(v0, v2.clone(), 2u64);
    let v7: u64 = f71(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f70(v1.clone(), f209(v4, v3, match v1 {
        Adt3::C0(h3881, _, _, _, _, _, _, _) => {
            h3881
        }
    })?, v5, v6, v7)?, f143(v0, v2)?))
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3882 = v0;
    let a3883 = v3;
    if nat_lt(a3882, a3883) {
        Ok(Adt9::C1(false, {
            let a3884 = v0;
            let a3885 = 4u64;
            nat_mul(a3884, a3885)?
        }, {
            let a3886 = v0;
            let a3887 = 4u64;
            nat_mul(a3886, a3887)?
        }))
    } else {
        let v6: u64 = {
            let a3888 = v0;
            let a3889 = v3;
            nat_sub(a3888, a3889)
        };
        let v7: u64 = f143(v6, v1)?;
        let v8: u64 = f210(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3890 = v0;
    let a3891 = v3;
    if nat_lt(a3890, a3891) {
        Ok(Adt9::C1(true, 0u64, f143(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3892 = v0;
            let a3893 = v3;
            nat_sub(a3892, a3893)
        };
        let v5: u64 = f145(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f143(v4, v1)?, 3u64, v5))
    }
}

pub fn f151(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3895 = v1;
        let a3896 = match v3.clone() {
            Adt3::C0(h3894, _, _, _, _, _, _, _) => {
                h3894
            }
        };
        let a3897 = 0u64;
        nat_rem(a3895, a3896, a3897)
    };
    let v5: u64 = {
        let a3903 = {
            let a3899 = v1;
            let a3900 = match v3.clone() {
                Adt3::C0(h3898, _, _, _, _, _, _, _) => {
                    h3898
                }
            };
            let a3901 = 0u64;
            nat_quot(a3899, a3900, a3901)
        };
        let a3904 = match v3.clone() {
            Adt3::C0(_, h3902, _, _, _, _, _, _) => {
                h3902
            }
        };
        let a3905 = 0u64;
        nat_rem(a3903, a3904, a3905)
    };
    let v6: bool = f211(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f145(v1, v2, v3.clone())?
    } else {
        0u64
    }, f143(v1, v3)?))
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3906 = v3;
        let a3907 = 8u64;
        nat_add(a3906, a3907)?
    }, 0u64, f143(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f145(v0, v5, v2.clone())?
    }, 3u64, f143(v0, v2)?))
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3908 = v4;
    let a3909 = 0u64;
    if if nat_eq(a3908, a3909) {
        f51(v3)
    } else {
        false
    } {
        let m3910 = f118(v3);
        match m3910 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h3911, _) => {
                        h3911
                    }
                };
                Ok(Adt9::C5(v3, f143(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h3912) => {
                        h3912
                    }
                }, f143({
                    let a3913 = v0;
                    let a3914 = v6;
                    nat_mul(a3913, a3914)?
                }, v2.clone())?, {
                    let a3920 = {
                        let a3918 = {
                            let a3915 = v6;
                            let a3916 = 1u64;
                            nat_sub(a3915, a3916)
                        };
                        let a3919 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3917, _, _, _) => {
                                h3917
                            }
                        };
                        nat_mul(a3918, a3919)?
                    };
                    let a3921 = 4u64;
                    nat_add(a3920, a3921)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3922, _, _, _) => {
                        h3922
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f143(v0, v1)?, f143(v0, v2)?))
    }
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3924 = v0;
        let a3925 = match v2.clone() {
            Adt3::C0(h3923, _, _, _, _, _, _, _) => {
                h3923
            }
        };
        let a3926 = 0u64;
        nat_rem(a3924, a3925, a3926)
    };
    let v7: u64 = {
        let a3928 = v0;
        let a3929 = match v2.clone() {
            Adt3::C0(h3927, _, _, _, _, _, _, _) => {
                h3927
            }
        };
        let a3930 = 0u64;
        nat_quot(a3928, a3929, a3930)
    };
    let v8: u64 = {
        let a3932 = v7;
        let a3933 = match v2.clone() {
            Adt3::C0(_, h3931, _, _, _, _, _, _) => {
                h3931
            }
        };
        let a3934 = 0u64;
        nat_rem(a3932, a3933, a3934)
    };
    let v9: u64 = {
        let a3936 = v7;
        let a3937 = match v2.clone() {
            Adt3::C0(_, h3935, _, _, _, _, _, _) => {
                h3935
            }
        };
        let a3938 = 0u64;
        nat_quot(a3936, a3937, a3938)
    };
    let v10: u64 = {
        let a3940 = v9;
        let a3941 = match v2.clone() {
            Adt3::C0(_, _, h3939, _, _, _, _, _) => {
                h3939
            }
        };
        let a3942 = 0u64;
        nat_rem(a3940, a3941, a3942)
    };
    let v11: u64 = {
        let a3944 = v9;
        let a3945 = match v2.clone() {
            Adt3::C0(_, _, h3943, _, _, _, _, _) => {
                h3943
            }
        };
        let a3946 = 0u64;
        nat_quot(a3944, a3945, a3946)
    };
    let v12: u64 = f143(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3955 = {
                let a3950 = {
                    let a3947 = v10;
                    let a3948 = v4;
                    let a3949 = 0u64;
                    nat_quot(a3947, a3948, a3949)
                };
                let a3951 = v5;
                nat_mul(a3950, a3951)?
            };
            let a3956 = {
                let a3952 = v8;
                let a3953 = v4;
                let a3954 = 0u64;
                nat_quot(a3952, a3953, a3954)
            };
            nat_add(a3955, a3956)?
        };
        let v14: u64 = {
            let a3976 = v6;
            let a3977 = {
                let a3974 = match v1.clone() {
                    Adt3::C0(h3957, _, _, _, _, _, _, _) => {
                        h3957
                    }
                };
                let a3975 = {
                    let a3972 = {
                        let a3958 = v8;
                        let a3959 = v4;
                        let a3960 = 0u64;
                        nat_rem(a3958, a3959, a3960)
                    };
                    let a3973 = {
                        let a3970 = match v1.clone() {
                            Adt3::C0(_, h3961, _, _, _, _, _, _) => {
                                h3961
                            }
                        };
                        let a3971 = {
                            let a3968 = {
                                let a3962 = v10;
                                let a3963 = v4;
                                let a3964 = 0u64;
                                nat_rem(a3962, a3963, a3964)
                            };
                            let a3969 = {
                                let a3966 = match v1.clone() {
                                    Adt3::C0(_, _, h3965, _, _, _, _, _) => {
                                        h3965
                                    }
                                };
                                let a3967 = v13;
                                nat_mul(a3966, a3967)?
                            };
                            nat_add(a3968, a3969)?
                        };
                        nat_mul(a3970, a3971)?
                    };
                    nat_add(a3972, a3973)?
                };
                nat_mul(a3974, a3975)?
            };
            nat_add(a3976, a3977)?
        };
        Ok(Adt9::C1(false, f143(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3983 = {
                let a3981 = {
                    let a3978 = v11;
                    let a3979 = v5;
                    let a3980 = 0u64;
                    nat_rem(a3978, a3979, a3980)
                };
                let a3982 = v4;
                nat_mul(a3981, a3982)?
            };
            let a3984 = v8;
            nat_add(a3983, a3984)?
        };
        let v16: u64 = {
            let a3990 = {
                let a3988 = {
                    let a3985 = v11;
                    let a3986 = v5;
                    let a3987 = 0u64;
                    nat_quot(a3985, a3986, a3987)
                };
                let a3989 = v4;
                nat_mul(a3988, a3989)?
            };
            let a3991 = v10;
            nat_add(a3990, a3991)?
        };
        let a3993 = v15;
        let a3994 = match v1.clone() {
            Adt3::C0(_, h3992, _, _, _, _, _, _) => {
                h3992
            }
        };
        if if nat_lt(a3993, a3994) {
            let a3996 = v16;
            let a3997 = match v1.clone() {
                Adt3::C0(_, _, h3995, _, _, _, _, _) => {
                    h3995
                }
            };
            nat_lt(a3996, a3997)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f143({
                let a4006 = v6;
                let a4007 = {
                    let a4004 = match v1.clone() {
                        Adt3::C0(h3998, _, _, _, _, _, _, _) => {
                            h3998
                        }
                    };
                    let a4005 = {
                        let a4002 = v15;
                        let a4003 = {
                            let a4000 = match v1.clone() {
                                Adt3::C0(_, h3999, _, _, _, _, _, _) => {
                                    h3999
                                }
                            };
                            let a4001 = v16;
                            nat_mul(a4000, a4001)?
                        };
                        nat_add(a4002, a4003)?
                    };
                    nat_mul(a4004, a4005)?
                };
                nat_add(a4006, a4007)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f134(23u64, v4);
    let v6: u64 = {
        let a4008 = v0;
        let a4009 = v5;
        let a4010 = 0u64;
        nat_rem(a4008, a4009, a4010)
    };
    let v7: u64 = {
        let a4011 = v0;
        let a4012 = v5;
        let a4013 = 0u64;
        nat_quot(a4011, a4012, a4013)
    };
    let v8: Adt13 = Adt13::C1(f143(v7, v2)?);
    let a4014 = v6;
    let a4015 = 0u64;
    if nat_eq(a4014, a4015) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f143(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f203(v3), 4u64))
    } else {
        let a4016 = v6;
        let a4017 = 1u64;
        if nat_eq(a4016, a4017) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4018 = v4;
                let a4019 = 0u64;
                if nat_eq(a4018, a4019) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f203(v4)))
        }
    }
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f143(v0, v2.clone())?;
    let v4: u64 = f143(v0, v1.clone())?;
    let a4024 = {
        let a4021 = v0;
        let a4022 = match v1 {
            Adt3::C0(h4020, _, _, _, _, _, _, _) => {
                h4020
            }
        };
        let a4023 = 0u64;
        nat_rem(a4021, a4022, a4023)
    };
    let a4025 = 0u64;
    if nat_eq(a4024, a4025) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f143({
            let a4026 = v0;
            let a4027 = 1u64;
            nat_sub(a4026, a4027)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4028 = v3;
    let a4029 = 13u64;
    if nat_eq(a4028, a4029) {
        f212(v0, v1, v4)
    } else {
        let v5: u64 = f201(match v1.clone() {
            Adt3::C0(h4030, _, _, _, _, _, _, _) => {
                h4030
            }
        }, {
            let a4031 = v3;
            let a4032 = 16u64;
            nat_eq(a4031, a4032)
        })?;
        let v6: u64 = {
            let a4033 = v0;
            let a4034 = v5;
            let a4035 = 0u64;
            nat_quot(a4033, a4034, a4035)
        };
        let v7: u64 = {
            let a4036 = v0;
            let a4037 = v5;
            let a4038 = 0u64;
            nat_rem(a4036, a4037, a4038)
        };
        let v8: u64 = {
            let a4042 = 1u64;
            let a4043 = {
                let a4040 = match v1.clone() {
                    Adt3::C0(h4039, _, _, _, _, _, _, _) => {
                        h4039
                    }
                };
                let a4041 = 2u64;
                nat_mul(a4040, a4041)?
            };
            nat_add(a4042, a4043)?
        };
        let a4044 = v7;
        let a4045 = v8;
        if nat_lt(a4044, a4045) {
            f213(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f143(v6, v2)?;
            let a4046 = v3;
            let a4047 = 14u64;
            if nat_eq(a4046, a4047) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4048 = v7;
                let a4049 = v8;
                if nat_eq(a4048, a4049) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4052 = v7;
                    let a4053 = {
                        let a4050 = v8;
                        let a4051 = 1u64;
                        nat_add(a4050, a4051)?
                    };
                    if nat_eq(a4052, a4053) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4054, _, _, _, _, _, _, _) => {
                                h4054
                            }
                        }, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(5u64, 1u64, 1u64, 1u64, 2u64, 3u64, v9))
                    }
                }
            }
        }
    }
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4055 = v0;
    let a4056 = 0u64;
    if nat_eq(a4055, a4056) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4063 = {
            let a4057 = v0;
            let a4058 = 1u64;
            nat_add(a4057, a4058)?
        };
        let a4064 = {
            let a4061 = 2u64;
            let a4062 = {
                let a4059 = v3;
                let a4060 = 2u64;
                nat_mul(a4059, a4060)?
            };
            nat_add(a4061, a4062)?
        };
        if nat_eq(a4063, a4064) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4065 = v0;
                let a4066 = 1u64;
                nat_sub(a4065, a4066)
            };
            let v5: u64 = {
                let a4067 = v4;
                let a4068 = 2u64;
                let a4069 = 0u64;
                nat_quot(a4067, a4068, a4069)
            };
            let a4073 = {
                let a4070 = v4;
                let a4071 = 2u64;
                let a4072 = 0u64;
                nat_rem(a4070, a4071, a4072)
            };
            let a4074 = 0u64;
            if nat_eq(a4073, a4074) {
                Ok(Adt9::C4(17u64, 0u64, f143(v5, v1)?, 4u64, f143(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f159(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4075 = v0;
    let a4076 = f27(v1, v2, v3)?;
    if nat_lt(a4075, a4076) {
        let a4077 = v0;
        let a4078 = v1;
        if nat_lt(a4077, a4078) {
            Ok(Adt9::C1(false, {
                let a4079 = 4u64;
                let a4080 = v0;
                nat_mul(a4079, a4080)?
            }, {
                let a4081 = 4u64;
                let a4082 = v0;
                nat_mul(a4081, a4082)?
            }))
        } else {
            let v4: u64 = {
                let a4083 = v0;
                let a4084 = v1;
                nat_sub(a4083, a4084)
            };
            let v5: u64 = f214(v4, v2)?;
            let a4085 = v5;
            let a4086 = 2u64;
            if nat_lt(a4085, a4086) {
                Ok(Adt9::C4(6u64, {
                    let a4087 = v5;
                    let a4088 = 0u64;
                    if nat_eq(a4087, a4088) {
                        4u64
                    } else {
                        5u64
                    }
                }, f215(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f216(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f217(v4, v2)? {
                    1u64
                } else {
                    0u64
                }, 3u64, v6))
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f160(v0: u64, v1: u64) -> Adt11 {
    let a4089 = v0;
    let a4090 = 43u64;
    if nat_eq(a4089, a4090) {
        Adt11::C8(3u64)
    } else {
        let a4091 = v1;
        let a4092 = 43u64;
        if nat_eq(a4091, a4092) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f161(v0: u64, v1: u64) -> R<u64> {
    if f108(v0) {
        let a4093 = 2u64;
        let a4094 = v1;
        nat_mul(a4093, a4094)
    } else {
        let a4095 = v1;
        let a4096 = f197(v0);
        let a4097 = 0u64;
        Ok(nat_quot(a4095, a4096, a4097))
    }
}

pub fn f162(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4098 = f118(v0);
    match m4098 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f135(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4099, _, _, _) => {
                    h4099
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4100, _, _) => {
                    h4100
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4101, _) => {
                    h4101
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4102) => {
                    h4102
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h4103) => {
                    h4103
                }
            }, f135({
                let a4105 = v2;
                let a4106 = match v6.clone() {
                    Adt18::C0(_, h4104, _) => {
                        h4104
                    }
                };
                nat_mul(a4105, a4106)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4107, _, _, _) => {
                    h4107
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4108, _, _) => {
                    h4108
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4109, _) => {
                    h4109
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4110) => {
                    h4110
                }
            })?, match v6 {
                Adt18::C0(_, h4111, _) => {
                    h4111
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4112, _, _, _) => {
                    h4112
                }
            }))
        }
    }
}

pub fn f163(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4120 = {
        let a4113 = 2u64;
        let a4114 = f131(v1.clone())?;
        nat_mul(a4113, a4114)?
    };
    let a4121 = {
        let a4118 = f131(v1)?;
        let a4119 = {
            let a4116 = 1u64;
            let a4117 = match v0 {
                Adt3::C0(h4115, _, _, _, _, _, _, _) => {
                    h4115
                }
            };
            nat_add(a4116, a4117)?
        };
        nat_mul(a4118, a4119)?
    };
    nat_add(a4120, a4121)
}

pub fn f164(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f143(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f165(v0: u64, v1: u64) -> bool {
    let a4122 = v1;
    let a4123 = 2147483647u64;
    if nat_le(a4122, a4123) {
        let a4124 = v1;
        let a4125 = v0;
        nat_lt(a4124, a4125)
    } else {
        false
    }
}

pub fn f166(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4126 = f118(v0);
    match m4126 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4128 = v2;
                let a4129 = match v7.clone() {
                    Adt18::C0(_, h4127, _) => {
                        h4127
                    }
                };
                nat_mul(a4128, a4129)?
            };
            let m4142 = f218(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4130, _, _, _, _, _, _, _) => {
                    h4130
                }
            }, match v5.clone() {
                Adt3::C0(_, h4131, _, _, _, _, _, _) => {
                    h4131
                }
            }, match v5 {
                Adt3::C0(_, _, h4132, _, _, _, _, _) => {
                    h4132
                }
            }, {
                let a4135 = match v3.clone() {
                    Adt3::C0(h4133, _, _, _, _, _, _, _) => {
                        h4133
                    }
                };
                let a4136 = match v7.clone() {
                    Adt18::C0(_, h4134, _) => {
                        h4134
                    }
                };
                nat_mul(a4135, a4136)?
            }, match v3.clone() {
                Adt3::C0(_, h4137, _, _, _, _, _, _) => {
                    h4137
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4138, _, _, _) => {
                    h4138
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4139, _, _) => {
                    h4139
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4140, _) => {
                    h4140
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4141) => {
                    h4141
                }
            })?;
            match m4142 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4143 = v0;
                    let a4144 = 0u64;
                    if if nat_eq(a4143, a4144) {
                        true
                    } else {
                        let a4145 = v0;
                        let a4146 = 26u64;
                        nat_eq(a4145, a4146)
                    } {
                        Ok(Adt9::C1(false, v9, f143({
                            let a4150 = {
                                let a4148 = v1;
                                let a4149 = match v4.clone() {
                                    Adt3::C0(h4147, _, _, _, _, _, _, _) => {
                                        h4147
                                    }
                                };
                                nat_mul(a4148, a4149)?
                            };
                            let a4151 = v8;
                            nat_add(a4150, a4151)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4152) => {
                                h4152
                            }
                        }, f143({
                            let a4156 = {
                                let a4154 = v1;
                                let a4155 = match v4.clone() {
                                    Adt3::C0(h4153, _, _, _, _, _, _, _) => {
                                        h4153
                                    }
                                };
                                nat_mul(a4154, a4155)?
                            };
                            let a4157 = v8;
                            nat_add(a4156, a4157)?
                        }, v4.clone())?, {
                            let a4164 = {
                                let a4162 = {
                                    let a4159 = match v7 {
                                        Adt18::C0(_, h4158, _) => {
                                            h4158
                                        }
                                    };
                                    let a4160 = 1u64;
                                    nat_sub(a4159, a4160)
                                };
                                let a4163 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4161, _, _, _) => {
                                        h4161
                                    }
                                };
                                nat_mul(a4162, a4163)?
                            };
                            let a4165 = 4u64;
                            nat_add(a4164, a4165)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4166, _, _, _) => {
                                h4166
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f167(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4167 = v1;
    let a4168 = 0u64;
    if if nat_eq(a4167, a4168) {
        true
    } else {
        let a4169 = v2;
        let a4170 = 0u64;
        nat_eq(a4169, a4170)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f168(v0: u64, v1: u64) -> bool {
    let a4171 = v0;
    let a4172 = 4294967295u64;
    if nat_eq(a4171, a4172) {
        true
    } else {
        let a4173 = v0;
        let a4174 = 2147483648u64;
        if nat_lt(a4173, a4174) {
            let a4175 = v0;
            let a4176 = v1;
            nat_lt(a4175, a4176)
        } else {
            false
        }
    }
}

pub fn f169(v0: Adt7) -> R<u64> {
    let a4181 = if match v0.clone() {
        Adt7::C0(_, _, _, h4177, _) => {
            h4177
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4182 = {
        let a4179 = match v0 {
            Adt7::C0(_, _, _, _, h4178) => {
                h4178
            }
        };
        let a4180 = 1u64;
        if nat_eq(a4179, a4180) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4181, a4182)
}

pub fn f170(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4187 = {
            let a4184 = v2;
            let a4185 = match v0.clone() {
                Adt6::C0(_, h4183, _, _) => {
                    h4183
                }
            };
            nat_mul(a4184, a4185)?
        };
        let a4188 = f136(match v0.clone() {
            Adt6::C0(_, _, h4186, _) => {
                h4186
            }
        });
        nat_add(a4187, a4188)?
    };
    let v4: u64 = f137(match v0.clone() {
        Adt6::C0(_, _, h4189, _) => {
            h4189
        }
    });
    let v5: u64 = {
        let a4193 = {
            let a4191 = v3;
            let a4192 = match v0 {
                Adt6::C0(h4190, _, _, _) => {
                    h4190
                }
            };
            nat_add(a4191, a4192)?
        };
        let a4194 = v4;
        nat_sub(a4193, a4194)
    };
    let a4199 = {
        let a4195 = v5;
        let a4196 = v1;
        if nat_lt(a4195, a4196) {
            v5
        } else {
            v1
        }
    };
    let a4200 = {
        let a4197 = v3;
        let a4198 = v4;
        nat_sub(a4197, a4198)
    };
    Ok(nat_sub(a4199, a4200))
}

pub fn f171(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4209 = f73(match v0.clone() {
        Adt7::C0(h4201, _, _, _, _) => {
            h4201
        }
    }, match v1.clone() {
        Adt3::C0(h4202, _, _, _, _, _, _, _) => {
            h4202
        }
    }, f71(v3, v2.clone(), 0u64), {
        let a4206 = v4;
        let a4207 = {
            let m4205 = match v0.clone() {
                Adt7::C0(h4204, _, _, _, _) => {
                    h4204
                }
            };
            match m4205 {
                Adt6::C0(h4203, _, _, _) => {
                    h4203
                }
            }
        };
        let a4208 = 0u64;
        nat_rem(a4206, a4207, a4208)
    })?;
    match m4209 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4218 = f73(match v0.clone() {
                Adt7::C0(_, h4210, _, _, _) => {
                    h4210
                }
            }, match v1.clone() {
                Adt3::C0(_, h4211, _, _, _, _, _, _) => {
                    h4211
                }
            }, f71(v3, v2.clone(), 1u64), {
                let a4215 = v4;
                let a4216 = {
                    let m4214 = match v0 {
                        Adt7::C0(h4213, _, _, _, _) => {
                            h4213
                        }
                    };
                    match m4214 {
                        Adt6::C0(h4212, _, _, _) => {
                            h4212
                        }
                    }
                };
                let a4217 = 0u64;
                nat_quot(a4215, a4216, a4217)
            })?;
            match m4218 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f70(v1, v5, v6, f71(v3, v2.clone(), 2u64), f71(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f172(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m4219 = v0;
    match m4219 {
        Adt16::C0(v19) => {
            f219(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4220 = v20;
            let a4221 = 66u64;
            if nat_eq(a4220, a4221) {
                f220(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f221(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f173(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4228 = v3;
        let a4229 = {
            let a4225 = f143({
                let a4222 = v0;
                let a4223 = 2u64;
                let a4224 = 0u64;
                nat_quot(a4222, a4223, a4224)
            }, v1)?;
            let a4226 = v2;
            let a4227 = 0u64;
            nat_quot(a4225, a4226, a4227)
        };
        nat_add(a4228, a4229)?
    };
    let a4233 = {
        let a4230 = v0;
        let a4231 = 2u64;
        let a4232 = 0u64;
        nat_rem(a4230, a4231, a4232)
    };
    let a4234 = 0u64;
    if nat_eq(a4233, a4234) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4235 = v4;
        let a4236 = 0u64;
        if nat_eq(a4235, a4236) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f174(v0: u64, v1: u64) -> R<u64> {
    let a4239 = 8u64;
    let a4240 = {
        let a4237 = f222(v0);
        let a4238 = v1;
        nat_mul(a4237, a4238)?
    };
    nat_add(a4239, a4240)
}

pub fn f175(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4241, _, _, _, _, _, _, _) => {
            h4241
        }
    };
    let v7: u64 = f222(v3);
    let v8: u64 = {
        let a4244 = 1u64;
        let a4245 = {
            let a4242 = v6;
            let a4243 = v7;
            nat_mul(a4242, a4243)?
        };
        nat_add(a4244, a4245)?
    };
    let a4246 = v0;
    let a4247 = 0u64;
    if nat_eq(a4246, a4247) {
        Ok(f223(0u64))
    } else {
        let a4248 = v0;
        let a4249 = v8;
        if nat_lt(a4248, a4249) {
            let v9: u64 = {
                let a4250 = v0;
                let a4251 = 1u64;
                nat_sub(a4250, a4251)
            };
            let v10: u64 = {
                let a4252 = v9;
                let a4253 = v7;
                let a4254 = 0u64;
                nat_rem(a4252, a4253, a4254)
            };
            let v11: u64 = f143({
                let a4260 = {
                    let a4255 = v1;
                    let a4256 = v6;
                    nat_mul(a4255, a4256)?
                };
                let a4261 = {
                    let a4257 = v9;
                    let a4258 = v7;
                    let a4259 = 0u64;
                    nat_quot(a4257, a4258, a4259)
                };
                nat_add(a4260, a4261)?
            }, v2)?;
            let a4262 = v3;
            let a4263 = 30u64;
            if nat_eq(a4262, a4263) {
                let a4264 = v10;
                let a4265 = 0u64;
                if nat_eq(a4264, a4265) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4266 = v10;
                    let a4267 = 1u64;
                    if nat_eq(a4266, a4267) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4268 = v10;
                        let a4269 = 2u64;
                        if nat_eq(a4268, a4269) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4270 = v10;
                let a4271 = 0u64;
                if nat_eq(a4270, a4271) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4272 = v10;
                    let a4273 = 1u64;
                    if nat_eq(a4272, a4273) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4274 = v0;
                let a4275 = v8;
                nat_sub(a4274, a4275)
            };
            let a4276 = v12;
            let a4277 = 0u64;
            if nat_eq(a4276, a4277) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4278 = v12;
                let a4279 = 1u64;
                if nat_eq(a4278, a4279) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4280 = v12;
                    let a4281 = 2u64;
                    if nat_eq(a4280, a4281) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4282 = v12;
                        let a4283 = 3u64;
                        if nat_eq(a4282, a4283) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4284 = v12;
                            let a4285 = 4u64;
                            if nat_eq(a4284, a4285) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4286 = v12;
                                let a4287 = 5u64;
                                if nat_eq(a4286, a4287) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4288 = v12;
                                    let a4289 = 6u64;
                                    if nat_eq(a4288, a4289) {
                                        if f224(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4290 = 3u64;
                                                let a4291 = v1;
                                                nat_add(a4290, a4291)?
                                            }))
                                        } else {
                                            Ok(Adt9::C0(Adt10::C1))
                                        }
                                    } else {
                                        Ok(Adt9::C0(Adt10::C1))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f176(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4292 = v0;
        let a4293 = f104(v3);
        let a4294 = 0u64;
        nat_quot(a4292, a4293, a4294)
    };
    let v5: u64 = {
        let a4299 = 3u64;
        let a4300 = {
            let a4296 = v4;
            let a4297 = match v1.clone() {
                Adt3::C0(h4295, _, _, _, _, _, _, _) => {
                    h4295
                }
            };
            let a4298 = 0u64;
            nat_quot(a4296, a4297, a4298)
        };
        nat_add(a4299, a4300)?
    };
    let v6: u64 = f143(v4, v1)?;
    let v7: u64 = f143(v4, v2)?;
    let a4301 = v3;
    let a4302 = 30u64;
    if nat_eq(a4301, a4302) {
        let a4306 = {
            let a4303 = v0;
            let a4304 = 3u64;
            let a4305 = 0u64;
            nat_rem(a4303, a4304, a4305)
        };
        let a4307 = 0u64;
        if nat_eq(a4306, a4307) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4311 = {
                let a4308 = v0;
                let a4309 = 3u64;
                let a4310 = 0u64;
                nat_rem(a4308, a4309, a4310)
            };
            let a4312 = 1u64;
            if nat_eq(a4311, a4312) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f177(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4313 = v0;
        let a4314 = 8388608u64;
        let a4315 = 0u64;
        nat_quot(a4313, a4314, a4315)
    };
    let a4316 = v0;
    let a4317 = 2147483648u64;
    if if if nat_lt(a4316, a4317) {
        let a4318 = 127u64;
        let a4319 = v1;
        nat_le(a4318, a4319)
    } else {
        false
    } {
        let a4320 = v1;
        let a4321 = 190u64;
        nat_lt(a4320, a4321)
    } else {
        false
    } {
        let v2: u64 = {
            let a4325 = 8388608u64;
            let a4326 = {
                let a4322 = v0;
                let a4323 = 8388608u64;
                let a4324 = 0u64;
                nat_rem(a4322, a4323, a4324)
            };
            nat_add(a4325, a4326)?
        };
        let a4327 = v1;
        let a4328 = 150u64;
        if nat_lt(a4327, a4328) {
            let v3: u64 = f225({
                let a4329 = 150u64;
                let a4330 = v1;
                nat_sub(a4329, a4330)
            });
            let a4334 = {
                let a4331 = v2;
                let a4332 = v3;
                let a4333 = 0u64;
                nat_rem(a4331, a4332, a4333)
            };
            let a4335 = 0u64;
            if nat_eq(a4334, a4335) {
                let a4336 = v2;
                let a4337 = v3;
                let a4338 = 0u64;
                Ok(nat_quot(a4336, a4337, a4338))
            } else {
                Ok(0u64)
            }
        } else {
            let a4341 = f225({
                let a4339 = v1;
                let a4340 = 150u64;
                nat_sub(a4339, a4340)
            });
            let a4342 = v2;
            nat_mul(a4341, a4342)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f178(v0: Adt14, v1: Adt14) -> bool {
    let a4345 = match v0.clone() {
        Adt14::C0(_, h4343, _, _, _, _, _, _, _, _, _) => {
            h4343
        }
    };
    let a4346 = match v1.clone() {
        Adt14::C0(_, h4344, _, _, _, _, _, _, _, _, _) => {
            h4344
        }
    };
    if if if nat_eq(a4345, a4346) {
        let a4349 = match v0.clone() {
            Adt14::C0(_, _, h4347, _, _, _, _, _, _, _, _) => {
                h4347
            }
        };
        let a4350 = match v1.clone() {
            Adt14::C0(_, _, h4348, _, _, _, _, _, _, _, _) => {
                h4348
            }
        };
        nat_eq(a4349, a4350)
    } else {
        false
    } {
        let a4353 = match v0.clone() {
            Adt14::C0(_, _, _, h4351, _, _, _, _, _, _, _) => {
                h4351
            }
        };
        let a4354 = match v1.clone() {
            Adt14::C0(_, _, _, h4352, _, _, _, _, _, _, _) => {
                h4352
            }
        };
        nat_eq(a4353, a4354)
    } else {
        false
    } {
        let a4357 = match v0 {
            Adt14::C0(_, _, _, _, h4355, _, _, _, _, _, _) => {
                h4355
            }
        };
        let a4358 = match v1 {
            Adt14::C0(_, _, _, _, h4356, _, _, _, _, _, _) => {
                h4356
            }
        };
        nat_eq(a4357, a4358)
    } else {
        false
    }
}

pub fn f179(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4359 = v0;
    let a4360 = 0u64;
    if if if nat_eq(a4359, a4360) {
        true
    } else {
        let a4361 = v1;
        let a4362 = 0u64;
        nat_eq(a4361, a4362)
    } {
        true
    } else {
        let a4363 = v2;
        let a4364 = 0u64;
        nat_eq(a4363, a4364)
    } {
        true
    } else {
        let a4365 = v3;
        let a4366 = 0u64;
        nat_eq(a4365, a4366)
    }
}

pub fn f180(v0: Adt14, v1: Adt14) -> bool {
    if f186(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4367, _, _, _, _, _) => {
            h4367
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4368, _, _, _, _, _) => {
            h4368
        }
    }) {
        let a4370 = match v0 {
            Adt14::C0(_, h4369, _, _, _, _, _, _, _, _, _) => {
                h4369
            }
        };
        let a4371 = 2147483647u64;
        nat_le(a4370, a4371)
    } else {
        true
    }
}

pub fn f181(v0: Adt14) -> bool {
    let a4376 = {
        let a4373 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4372) => {
                h4372
            }
        };
        let a4374 = 4u64;
        let a4375 = 0u64;
        nat_rem(a4373, a4374, a4375)
    };
    let a4377 = 0u64;
    if if nat_eq(a4376, a4377) {
        f226(match v0.clone() {
            Adt14::C0(_, h4378, _, _, _, _, _, _, _, _, _) => {
                h4378
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4379, _, _, _, _, _) => {
                h4379
            }
        })
    } else {
        false
    } {
        f100(match v0.clone() {
            Adt14::C0(_, _, h4380, _, _, _, _, _, _, _, _) => {
                h4380
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4381, _, _, _, _, _, _, _) => {
                h4381
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4382, _, _, _, _, _, _) => {
                h4382
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4383, _, _, _, _) => {
                h4383
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4384, _, _, _) => {
                h4384
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4385, _, _) => {
                h4385
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f182(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4387 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4386, _) => {
            h4386
        }
    };
    let a4388 = 0u64;
    if nat_eq(a4387, a4388) {
        Ok(true)
    } else {
        let a4390 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4389, _) => {
                h4389
            }
        };
        let a4391 = 0u64;
        if nat_eq(a4390, a4391) {
            Ok(true)
        } else {
            let a4393 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4392) => {
                    h4392
                }
            };
            let a4394 = 0u64;
            if nat_eq(a4393, a4394) {
                Ok(true)
            } else {
                let a4396 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4395) => {
                        h4395
                    }
                };
                let a4397 = 0u64;
                if nat_eq(a4396, a4397) {
                    Ok(true)
                } else {
                    let a4403 = {
                        let a4400 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4398) => {
                                h4398
                            }
                        };
                        let a4401 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4399, _) => {
                                h4399
                            }
                        };
                        nat_add(a4400, a4401)?
                    };
                    let a4404 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4402) => {
                            h4402
                        }
                    };
                    if nat_le(a4403, a4404) {
                        Ok(true)
                    } else {
                        let a4410 = {
                            let a4407 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4405) => {
                                    h4405
                                }
                            };
                            let a4408 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4406, _) => {
                                    h4406
                                }
                            };
                            nat_add(a4407, a4408)?
                        };
                        let a4411 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4409) => {
                                h4409
                            }
                        };
                        Ok(nat_le(a4410, a4411))
                    }
                }
            }
        }
    }
}

pub fn f183(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f178(v0.clone(), v1.clone()) {
        let a4414 = match v0.clone() {
            Adt14::C0(h4412, _, _, _, _, _, _, _, _, _, _) => {
                h4412
            }
        };
        let a4415 = match v1.clone() {
            Adt14::C0(h4413, _, _, _, _, _, _, _, _, _, _) => {
                h4413
            }
        };
        nat_eq(a4414, a4415)
    } else {
        false
    } {
        let a4418 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4416) => {
                h4416
            }
        };
        let a4419 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4417) => {
                h4417
            }
        };
        nat_eq(a4418, a4419)
    } else {
        false
    } {
        let a4422 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4420, _, _, _, _, _) => {
                h4420
            }
        };
        let a4423 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4421, _, _, _, _, _) => {
                h4421
            }
        };
        nat_eq(a4422, a4423)
    } else {
        false
    } {
        let a4426 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4424, _, _, _, _) => {
                h4424
            }
        };
        let a4427 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4425, _, _, _, _) => {
                h4425
            }
        };
        nat_eq(a4426, a4427)
    } else {
        false
    } {
        let a4430 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4428, _, _, _) => {
                h4428
            }
        };
        let a4431 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4429, _, _, _) => {
                h4429
            }
        };
        nat_eq(a4430, a4431)
    } else {
        false
    } {
        let a4434 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4432, _, _) => {
                h4432
            }
        };
        let a4435 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4433, _, _) => {
                h4433
            }
        };
        nat_eq(a4434, a4435)
    } else {
        false
    }
}

pub fn f184(v0: Adt14) -> R<bool> {
    if f227(match v0.clone() {
        Adt14::C0(_, h4436, _, _, _, _, _, _, _, _, _) => {
            h4436
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4437, _, _, _, _, _) => {
            h4437
        }
    }) {
        f101(f228(match v0.clone() {
            Adt14::C0(_, h4438, _, _, _, _, _, _, _, _, _) => {
                h4438
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4439, _, _, _, _, _) => {
                h4439
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4440, _, _, _, _, _, _, _, _) => {
                h4440
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4441, _, _, _, _, _, _, _) => {
                h4441
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4442, _, _, _, _, _, _) => {
                h4442
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4443, _, _, _, _) => {
                h4443
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4444, _, _, _) => {
                h4444
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4445, _, _) => {
                h4445
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f185(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4446 = v1;
    let a4447 = 0u64;
    if if nat_eq(a4446, a4447) {
        true
    } else {
        let a4448 = v2;
        let a4449 = 0u64;
        nat_eq(a4448, a4449)
    } {
        Ok(0u64)
    } else {
        let a4452 = f102(v1, v2, v3)?;
        let a4453 = {
            let a4450 = v2;
            let a4451 = f229(v0, v1)?;
            nat_mul(a4450, a4451)?
        };
        nat_add(a4452, a4453)
    }
}

pub fn f186(v0: u64, v1: u64) -> bool {
    let a4454 = v0;
    let a4455 = 4u64;
    if nat_eq(a4454, a4455) {
        let a4456 = v1;
        let a4457 = 4u64;
        nat_eq(a4456, a4457)
    } else {
        false
    }
}

pub fn f187(v0: u64) -> R<u64> {
    let a4458 = 6u64;
    let a4459 = v0;
    nat_add(a4458, a4459)
}

pub fn f188(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4460, _, _, _, _, _, _, _, _, _, _) => {
            h4460
        }
    }, match v0.clone() {
        Adt14::C0(_, h4461, _, _, _, _, _, _, _, _, _) => {
            h4461
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4462, _, _, _, _, _, _, _, _) => {
            h4462
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4463, _, _, _, _, _, _, _) => {
            h4463
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4464, _, _, _, _, _, _) => {
            h4464
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4465, _, _, _, _) => {
            h4465
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4466, _, _, _) => {
            h4466
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4467, _, _) => {
            h4467
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4468, _) => {
            h4468
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4469) => {
            h4469
        }
    })
}

pub fn f189(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4470 = v0;
    let a4471 = 2147483647u64;
    if if if nat_le(a4470, a4471) {
        let a4472 = 0u64;
        let a4473 = v3;
        if nat_lt(a4472, a4473) {
            let a4474 = v3;
            let a4475 = 2147483647u64;
            nat_le(a4474, a4475)
        } else {
            false
        }
    } else {
        false
    } {
        let a4480 = {
            let a4478 = {
                let a4476 = v2;
                let a4477 = v3;
                nat_add(a4476, a4477)?
            };
            let a4479 = 1u64;
            nat_sub(a4478, a4479)
        };
        let a4481 = 2147483647u64;
        nat_le(a4480, a4481)
    } else {
        false
    } {
        let a4486 = {
            let a4484 = {
                let a4482 = v0;
                let a4483 = v1;
                nat_mul(a4482, a4483)?
            };
            let a4485 = f230(v2, v3)?;
            nat_mul(a4484, a4485)?
        };
        let a4487 = 9223372036854775807u64;
        Ok(nat_le(a4486, a4487))
    } else {
        Ok(false)
    }
}

pub fn f190(v0: Adt14) -> R<u64> {
    let a4518 = {
        let a4510 = {
            let a4502 = {
                let a4494 = 4u64;
                let a4495 = {
                    let a4492 = {
                        let a4489 = match v0.clone() {
                            Adt14::C0(_, h4488, _, _, _, _, _, _, _, _, _) => {
                                h4488
                            }
                        };
                        let a4490 = 1u64;
                        nat_sub(a4489, a4490)
                    };
                    let a4493 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4491, _, _, _, _, _) => {
                            h4491
                        }
                    };
                    nat_mul(a4492, a4493)?
                };
                nat_add(a4494, a4495)?
            };
            let a4503 = {
                let a4500 = {
                    let a4497 = match v0.clone() {
                        Adt14::C0(_, _, h4496, _, _, _, _, _, _, _, _) => {
                            h4496
                        }
                    };
                    let a4498 = 1u64;
                    nat_sub(a4497, a4498)
                };
                let a4501 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4499, _, _, _, _) => {
                        h4499
                    }
                };
                nat_mul(a4500, a4501)?
            };
            nat_add(a4502, a4503)?
        };
        let a4511 = {
            let a4508 = {
                let a4505 = match v0.clone() {
                    Adt14::C0(_, _, _, h4504, _, _, _, _, _, _, _) => {
                        h4504
                    }
                };
                let a4506 = 1u64;
                nat_sub(a4505, a4506)
            };
            let a4509 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4507, _, _, _) => {
                    h4507
                }
            };
            nat_mul(a4508, a4509)?
        };
        nat_add(a4510, a4511)?
    };
    let a4519 = {
        let a4516 = {
            let a4513 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4512, _, _, _, _, _, _) => {
                    h4512
                }
            };
            let a4514 = 1u64;
            nat_sub(a4513, a4514)
        };
        let a4517 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4515, _, _) => {
                h4515
            }
        };
        nat_mul(a4516, a4517)?
    };
    nat_add(a4518, a4519)
}

pub fn f191(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4520 = v0;
    let a4521 = 0u64;
    if if if if nat_eq(a4520, a4521) {
        true
    } else {
        let a4522 = v1;
        let a4523 = 0u64;
        nat_eq(a4522, a4523)
    } {
        true
    } else {
        let a4524 = v2;
        let a4525 = 0u64;
        nat_eq(a4524, a4525)
    } {
        true
    } else {
        let a4526 = v3;
        let a4527 = 0u64;
        nat_eq(a4526, a4527)
    } {
        Ok(0u64)
    } else {
        let a4528 = f231(v0, v1, v2, v3, v5)?;
        let a4529 = f232(v0, v1, v2, v3, v4)?;
        nat_add(a4528, a4529)
    }
}

pub fn f192(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4530 = v0;
    let a4531 = 0u64;
    if if nat_eq(a4530, a4531) {
        true
    } else {
        let a4532 = v1;
        let a4533 = 0u64;
        nat_eq(a4532, a4533)
    } {
        Ok(0u64)
    } else {
        let a4536 = f102(v0, v1, v2)?;
        let a4537 = {
            let a4534 = v1;
            let a4535 = f233(v0)?;
            nat_mul(a4534, a4535)?
        };
        nat_add(a4536, a4537)
    }
}

pub fn f193(v0: u64, v1: u64, v2: u64) -> bool {
    let a4538 = v0;
    let a4539 = 1u64;
    if nat_le(a4538, a4539) {
        true
    } else {
        let a4543 = {
            let a4540 = v1;
            let a4541 = v2;
            let a4542 = 1u64;
            nat_rem(a4540, a4541, a4542)
        };
        let a4544 = 0u64;
        nat_eq(a4543, a4544)
    }
}

pub fn f194(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4545 = v1;
    let a4546 = 1u64;
    if if if nat_le(a4545, a4546) {
        true
    } else {
        let a4547 = v0;
        let a4548 = v2;
        nat_le(a4547, a4548)
    } {
        let a4549 = v3;
        let a4550 = 1u64;
        if nat_le(a4549, a4550) {
            true
        } else {
            let a4557 = {
                let a4555 = v0;
                let a4556 = {
                    let a4553 = {
                        let a4551 = v1;
                        let a4552 = 1u64;
                        nat_sub(a4551, a4552)
                    };
                    let a4554 = v2;
                    nat_mul(a4553, a4554)?
                };
                nat_add(a4555, a4556)?
            };
            let a4558 = v4;
            nat_le(a4557, a4558)
        }
    } else {
        false
    } {
        let a4559 = v5;
        let a4560 = 1u64;
        if nat_le(a4559, a4560) {
            Ok(true)
        } else {
            let a4573 = {
                let a4571 = {
                    let a4565 = v0;
                    let a4566 = {
                        let a4563 = {
                            let a4561 = v1;
                            let a4562 = 1u64;
                            nat_sub(a4561, a4562)
                        };
                        let a4564 = v2;
                        nat_mul(a4563, a4564)?
                    };
                    nat_add(a4565, a4566)?
                };
                let a4572 = {
                    let a4569 = {
                        let a4567 = v3;
                        let a4568 = 1u64;
                        nat_sub(a4567, a4568)
                    };
                    let a4570 = v4;
                    nat_mul(a4569, a4570)?
                };
                nat_add(a4571, a4572)?
            };
            let a4574 = v6;
            Ok(nat_le(a4573, a4574))
        }
    } else {
        Ok(false)
    }
}

pub fn f195(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f234(v0, v1, v6, v7, v8) {
        let a4575 = v5;
        let a4576 = 26u64;
        nat_eq(a4575, a4576)
    } else {
        false
    } {
        let a4577 = v3;
        let a4578 = v7;
        nat_eq(a4577, a4578)
    } else {
        false
    } {
        let a4579 = v4;
        let a4580 = v8;
        nat_eq(a4579, a4580)
    } else {
        false
    } {
        let a4581 = v9;
        let a4582 = 1u64;
        nat_eq(a4581, a4582)
    } else {
        false
    } {
        let a4583 = v10;
        let a4584 = f235(v0);
        nat_eq(a4583, a4584)
    } else {
        false
    } {
        let a4585 = v11;
        let a4586 = v1;
        nat_eq(a4585, a4586)
    } else {
        false
    } {
        let a4587 = v12;
        let a4588 = v6;
        nat_eq(a4587, a4588)
    } else {
        false
    } {
        let a4589 = v13;
        let a4590 = v7;
        nat_eq(a4589, a4590)
    } else {
        false
    } {
        let a4591 = v14;
        let a4592 = v8;
        nat_eq(a4591, a4592)
    } else {
        false
    }
}

pub fn f196(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4593 = v0;
    let a4594 = 0u64;
    if if if nat_eq(a4593, a4594) {
        true
    } else {
        let a4595 = v1;
        let a4596 = 0u64;
        nat_eq(a4595, a4596)
    } {
        true
    } else {
        let a4597 = v2;
        let a4598 = 0u64;
        nat_eq(a4597, a4598)
    } {
        true
    } else {
        let a4599 = v3;
        let a4600 = 0u64;
        nat_eq(a4599, a4600)
    }
}

pub fn f197(v0: u64) -> u64 {
    let m4601 = f118(v0);
    match m4601 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4602, _) => {
                    h4602
                }
            }
        }
    }
}

pub fn f198(v0: Adt3, v1: bool) -> R<u64> {
    f236(v0.clone(), {
        let a4604 = match v0.clone() {
            Adt3::C0(h4603, _, _, _, _, _, _, _) => {
                h4603
            }
        };
        let a4605 = 1u64;
        nat_sub(a4604, a4605)
    }, {
        let a4607 = match v0.clone() {
            Adt3::C0(_, h4606, _, _, _, _, _, _) => {
                h4606
            }
        };
        let a4608 = 1u64;
        nat_sub(a4607, a4608)
    }, {
        let a4610 = match v0.clone() {
            Adt3::C0(_, _, h4609, _, _, _, _, _) => {
                h4609
            }
        };
        let a4611 = 1u64;
        nat_sub(a4610, a4611)
    }, {
        let a4613 = match v0 {
            Adt3::C0(_, _, _, h4612, _, _, _, _) => {
                h4612
            }
        };
        let a4614 = 1u64;
        nat_sub(a4613, a4614)
    }, v1)
}

pub fn f199(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4615 = v0;
    let a4616 = 0u64;
    if nat_eq(a4615, a4616) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4617 = v0;
            let a4618 = 1u64;
            nat_sub(a4617, a4618)
        };
        let v5: u64 = {
            let a4619 = v4;
            let a4620 = v1;
            let a4621 = 0u64;
            nat_quot(a4619, a4620, a4621)
        };
        let v6: u64 = {
            let a4629 = {
                let a4622 = v5;
                let a4623 = v2;
                nat_mul(a4622, a4623)?
            };
            let a4630 = {
                let a4627 = {
                    let a4624 = v4;
                    let a4625 = v1;
                    let a4626 = 0u64;
                    nat_rem(a4624, a4625, a4626)
                };
                let a4628 = v3;
                nat_mul(a4627, a4628)?
            };
            nat_add(a4629, a4630)?
        };
        let a4631 = v5;
        let a4632 = 0u64;
        if nat_eq(a4631, a4632) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4641 = {
                    let a4635 = {
                        let a4633 = v5;
                        let a4634 = 1u64;
                        nat_sub(a4633, a4634)
                    };
                    let a4636 = v2;
                    nat_mul(a4635, a4636)?
                };
                let a4642 = {
                    let a4639 = {
                        let a4637 = v1;
                        let a4638 = 1u64;
                        nat_sub(a4637, a4638)
                    };
                    let a4640 = v3;
                    nat_mul(a4639, a4640)?
                };
                nat_add(a4641, a4642)?
            };
            let a4643 = v6;
            let a4644 = v7;
            if nat_lt(a4643, a4644) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f200(v0: Adt3, v1: u64) -> bool {
    let a4645 = v1;
    let a4646 = 0u64;
    if !nat_eq(a4645, a4646) {
        true
    } else {
        let a4648 = match v0 {
            Adt3::C0(_, _, _, _, h4647, _, _, _) => {
                h4647
            }
        };
        let a4649 = 4u64;
        nat_eq(a4648, a4649)
    }
}

pub fn f201(v0: u64, v1: bool) -> R<u64> {
    let a4652 = {
        let a4650 = v0;
        let a4651 = 2u64;
        nat_mul(a4650, a4651)?
    };
    let a4653 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4652, a4653)
}

pub fn f202(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4654 = f237(v0, v1);
    match m4654 {
        None => {
            if v5 {
                let m4655 = f238(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4655 {
                    Adt21::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt21::C1 => {
                        Ok(Adt10::C1)
                    }
                    Adt21::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt10::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt10::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a4656 = 4u64;
                let a4657 = v28;
                nat_le(a4656, a4657)
            };
            let a4658 = v2;
            let a4659 = 0u64;
            if if if if if nat_eq(a4658, a4659) {
                let a4660 = v4;
                let a4661 = 0u64;
                nat_eq(a4660, a4661)
            } else {
                false
            } {
                let a4662 = v6;
                let a4663 = v14;
                if nat_eq(a4662, a4663) {
                    let a4664 = v7;
                    let a4665 = v15;
                    nat_eq(a4664, a4665)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4666 = v8;
                let a4667 = v16;
                if nat_eq(a4666, a4667) {
                    let a4668 = v9;
                    let a4669 = v17;
                    nat_eq(a4668, a4669)
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
                        let a4670 = v3;
                        let a4671 = 0u64;
                        nat_eq(a4670, a4671)
                    } else {
                        false
                    } {
                        f121(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4676 = {
                        let a4672 = v6;
                        let a4673 = v7;
                        nat_mul(a4672, a4673)?
                    };
                    let a4677 = {
                        let a4674 = v8;
                        let a4675 = v9;
                        nat_mul(a4674, a4675)?
                    };
                    nat_mul(a4676, a4677)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f203(v0: u64) -> u64 {
    let m4678 = f118(v0);
    match m4678 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4680 = match v1.clone() {
                Adt18::C0(_, h4679, _) => {
                    h4679
                }
            };
            let a4681 = 1u64;
            if if nat_eq(a4680, a4681) {
                let a4683 = match v1.clone() {
                    Adt18::C0(_, _, h4682) => {
                        h4682
                    }
                };
                let a4684 = 1u64;
                if if if nat_eq(a4683, a4684) {
                    true
                } else {
                    let a4686 = match v1.clone() {
                        Adt18::C0(_, _, h4685) => {
                            h4685
                        }
                    };
                    let a4687 = 2u64;
                    nat_eq(a4686, a4687)
                } {
                    true
                } else {
                    let a4689 = match v1.clone() {
                        Adt18::C0(_, _, h4688) => {
                            h4688
                        }
                    };
                    let a4690 = 4u64;
                    nat_eq(a4689, a4690)
                } {
                    true
                } else {
                    let a4692 = match v1.clone() {
                        Adt18::C0(_, _, h4691) => {
                            h4691
                        }
                    };
                    let a4693 = 8u64;
                    nat_eq(a4692, a4693)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4694) => {
                        h4694
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f204(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4699 = {
        let a4697 = {
            let a4695 = v0;
            let a4696 = v1;
            nat_sub(a4695, a4696)
        };
        let a4698 = 1u64;
        nat_sub(a4697, a4698)
    };
    let a4700 = v2;
    nat_add(a4699, a4700)
}

pub fn f205(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f135({
        let a4701 = v0;
        let a4702 = v4;
        let a4703 = 0u64;
        nat_rem(a4701, a4702, a4703)
    }, {
        let a4704 = v1;
        let a4705 = v5;
        let a4706 = 0u64;
        nat_rem(a4704, a4705, a4706)
    }, {
        let a4707 = v2;
        let a4708 = v6;
        let a4709 = 0u64;
        nat_rem(a4707, a4708, a4709)
    }, {
        let a4710 = v3;
        let a4711 = v7;
        let a4712 = 0u64;
        nat_rem(a4710, a4711, a4712)
    }, v8, v9, v10, v11)
}

pub fn f206(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4713 = v1;
    let a4714 = 2147483648u64;
    if nat_lt(a4713, a4714) {
        let a4725 = {
            let a4723 = {
                let a4718 = {
                    let a4715 = v0;
                    let a4716 = v2;
                    let a4717 = 0u64;
                    nat_rem(a4715, a4716, a4717)
                };
                let a4719 = v2;
                nat_add(a4718, a4719)?
            };
            let a4724 = {
                let a4720 = v1;
                let a4721 = v2;
                let a4722 = 0u64;
                nat_rem(a4720, a4721, a4722)
            };
            nat_sub(a4723, a4724)
        };
        let a4726 = v2;
        let a4727 = 0u64;
        Ok(nat_rem(a4725, a4726, a4727))
    } else {
        let a4730 = {
            let a4728 = v0;
            let a4729 = f136(v1);
            nat_add(a4728, a4729)?
        };
        let a4731 = v2;
        let a4732 = 0u64;
        Ok(nat_rem(a4730, a4731, a4732))
    }
}

pub fn f207(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4735 = f137(v1);
    let a4736 = {
        let a4733 = v0;
        let a4734 = f136(v1);
        nat_add(a4733, a4734)?
    };
    if nat_le(a4735, a4736) {
        let a4737 = f208(v0, v1)?;
        let a4738 = v2;
        Ok(nat_lt(a4737, a4738))
    } else {
        Ok(false)
    }
}

pub fn f208(v0: u64, v1: u64) -> R<u64> {
    let a4741 = {
        let a4739 = v0;
        let a4740 = f136(v1);
        nat_add(a4739, a4740)?
    };
    let a4742 = f137(v1);
    Ok(nat_sub(a4741, a4742))
}

pub fn f209(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4743 = v0;
    let a4744 = v1;
    if nat_lt(a4743, a4744) {
        let a4745 = v1;
        let a4746 = v0;
        Ok(nat_sub(a4745, a4746))
    } else {
        let a4749 = v0;
        let a4750 = {
            let a4747 = v1;
            let a4748 = v2;
            nat_add(a4747, a4748)?
        };
        if nat_lt(a4749, a4750) {
            let a4751 = v0;
            let a4752 = v1;
            Ok(nat_sub(a4751, a4752))
        } else {
            let a4759 = {
                let a4755 = 2u64;
                let a4756 = {
                    let a4753 = v2;
                    let a4754 = 1u64;
                    nat_sub(a4753, a4754)
                };
                nat_mul(a4755, a4756)?
            };
            let a4760 = {
                let a4757 = v0;
                let a4758 = v1;
                nat_sub(a4757, a4758)
            };
            Ok(nat_sub(a4759, a4760))
        }
    }
}

pub fn f210(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4763 = {
        let a4761 = v2;
        let a4762 = f236(v1.clone(), f71(v0, v1.clone(), 0u64), f71(v0, v1.clone(), 1u64), f71(v0, v1.clone(), 2u64), f71(v0, v1.clone(), 3u64), false)?;
        nat_add(a4761, a4762)?
    };
    let a4764 = f236(v1.clone(), f71(v0, v1.clone(), 0u64), f71(v0, v1.clone(), 1u64), f71(v0, v1.clone(), 2u64), f71(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4763, a4764))
}

pub fn f211(v0: u64, v1: u64, v2: u64) -> bool {
    let a4765 = v0;
    let a4766 = 0u64;
    if nat_eq(a4765, a4766) {
        let a4767 = v2;
        let a4768 = v1;
        nat_le(a4767, a4768)
    } else {
        let a4769 = v0;
        let a4770 = 1u64;
        if nat_eq(a4769, a4770) {
            let a4771 = v2;
            let a4772 = v1;
            nat_lt(a4771, a4772)
        } else {
            let a4773 = v0;
            let a4774 = 2u64;
            if nat_eq(a4773, a4774) {
                let a4775 = v1;
                let a4776 = v2;
                nat_le(a4775, a4776)
            } else {
                let a4777 = v0;
                let a4778 = 3u64;
                if nat_eq(a4777, a4778) {
                    let a4779 = v1;
                    let a4780 = v2;
                    nat_lt(a4779, a4780)
                } else {
                    let a4781 = v1;
                    let a4782 = v2;
                    nat_eq(a4781, a4782)
                }
            }
        }
    }
}

pub fn f212(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4783 = v0;
    let a4784 = 0u64;
    if nat_eq(a4783, a4784) {
        Ok(f223(0u64))
    } else {
        let a4787 = {
            let a4785 = v0;
            let a4786 = 1u64;
            nat_add(a4785, a4786)?
        };
        let a4788 = f132(v1.clone(), 13u64, v2)?;
        if nat_eq(a4787, a4788) {
            Ok(Adt9::C4({
                let a4789 = v2;
                let a4790 = 0u64;
                if nat_eq(a4789, a4790) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4791 = v0;
                let a4792 = 1u64;
                nat_sub(a4791, a4792)
            };
            if f200(v1.clone(), v2) {
                let v4: u64 = f201(match v1.clone() {
                    Adt3::C0(h4793, _, _, _, _, _, _, _) => {
                        h4793
                    }
                }, false)?;
                let v5: u64 = {
                    let a4794 = v3;
                    let a4795 = v4;
                    let a4796 = 0u64;
                    nat_quot(a4794, a4795, a4796)
                };
                let v6: u64 = {
                    let a4797 = v3;
                    let a4798 = v4;
                    let a4799 = 0u64;
                    nat_rem(a4797, a4798, a4799)
                };
                let a4802 = {
                    let a4800 = v6;
                    let a4801 = 1u64;
                    nat_add(a4800, a4801)?
                };
                let a4803 = v4;
                if nat_eq(a4802, a4803) {
                    Ok(Adt9::C4({
                        let a4804 = v2;
                        let a4805 = 0u64;
                        if nat_eq(a4804, a4805) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f213(v6, v5, v1, v2)
                }
            } else {
                f239({
                    let a4806 = v3;
                    let a4807 = 2u64;
                    let a4808 = 0u64;
                    nat_quot(a4806, a4807, a4808)
                }, {
                    let a4809 = v3;
                    let a4810 = 2u64;
                    let a4811 = 0u64;
                    nat_rem(a4809, a4810, a4811)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f213(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4812 = v0;
    let a4813 = 0u64;
    if nat_eq(a4812, a4813) {
        Ok(f223(1u64))
    } else {
        f239({
            let a4822 = {
                let a4815 = v1;
                let a4816 = match v2.clone() {
                    Adt3::C0(h4814, _, _, _, _, _, _, _) => {
                        h4814
                    }
                };
                nat_mul(a4815, a4816)?
            };
            let a4823 = {
                let a4819 = {
                    let a4817 = v0;
                    let a4818 = 1u64;
                    nat_sub(a4817, a4818)
                };
                let a4820 = 2u64;
                let a4821 = 0u64;
                nat_quot(a4819, a4820, a4821)
            };
            nat_add(a4822, a4823)?
        }, {
            let a4826 = {
                let a4824 = v0;
                let a4825 = 1u64;
                nat_sub(a4824, a4825)
            };
            let a4827 = 2u64;
            let a4828 = 0u64;
            nat_rem(a4826, a4827, a4828)
        }, v2, v3, 1u64)
    }
}

pub fn f214(v0: u64, v1: u64) -> R<u64> {
    let a4829 = v0;
    let a4830 = f96(v1)?;
    let a4831 = 0u64;
    Ok(nat_rem(a4829, a4830, a4831))
}

pub fn f215(v0: u64, v1: u64) -> R<u64> {
    let a4832 = 4u64;
    let a4833 = f240(v0, v1)?;
    nat_mul(a4832, a4833)
}

pub fn f216(v0: u64, v1: u64) -> R<u64> {
    let a4834 = 4u64;
    let a4835 = f241(v0, v1)?;
    nat_mul(a4834, a4835)
}

pub fn f217(v0: u64, v1: u64) -> R<bool> {
    let a4841 = {
        let a4838 = {
            let a4836 = f214(v0, v1)?;
            let a4837 = 2u64;
            nat_sub(a4836, a4837)
        };
        let a4839 = 2u64;
        let a4840 = 0u64;
        nat_rem(a4838, a4839, a4840)
    };
    let a4842 = 0u64;
    Ok(nat_eq(a4841, a4842))
}

pub fn f218(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4843 = f118(v0);
    match m4843 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f116(v0) {
                let a4844 = v1;
                let a4845 = v7;
                nat_lt(a4844, a4845)
            } else {
                false
            } {
                f165(v8, v2)
            } else {
                false
            } {
                let a4846 = v3;
                let a4847 = f242(v4, v5, v6)?;
                nat_lt(a4846, a4847)
            } else {
                false
            } {
                let a4852 = {
                    let a4849 = v7;
                    let a4850 = match v13.clone() {
                        Adt18::C0(_, h4848, _) => {
                            h4848
                        }
                    };
                    let a4851 = 0u64;
                    nat_rem(a4849, a4850, a4851)
                };
                let a4853 = 0u64;
                nat_eq(a4852, a4853)
            } else {
                false
            } {
                Ok(Some({
                    let a4882 = {
                        let a4872 = {
                            let a4862 = {
                                let a4858 = {
                                    let a4855 = v1;
                                    let a4856 = match v13 {
                                        Adt18::C0(_, h4854, _) => {
                                            h4854
                                        }
                                    };
                                    let a4857 = 0u64;
                                    nat_quot(a4855, a4856, a4857)
                                };
                                let a4859 = v9;
                                nat_mul(a4858, a4859)?
                            };
                            let a4863 = {
                                let a4860 = v2;
                                let a4861 = v10;
                                nat_mul(a4860, a4861)?
                            };
                            nat_add(a4862, a4863)?
                        };
                        let a4873 = {
                            let a4870 = {
                                let a4867 = {
                                    let a4864 = v3;
                                    let a4865 = v4;
                                    let a4866 = 0u64;
                                    nat_quot(a4864, a4865, a4866)
                                };
                                let a4868 = v5;
                                let a4869 = 0u64;
                                nat_rem(a4867, a4868, a4869)
                            };
                            let a4871 = v11;
                            nat_mul(a4870, a4871)?
                        };
                        nat_add(a4872, a4873)?
                    };
                    let a4883 = {
                        let a4880 = {
                            let a4877 = {
                                let a4874 = v3;
                                let a4875 = v4;
                                let a4876 = 0u64;
                                nat_quot(a4874, a4875, a4876)
                            };
                            let a4878 = v5;
                            let a4879 = 0u64;
                            nat_quot(a4877, a4878, a4879)
                        };
                        let a4881 = v12;
                        nat_mul(a4880, a4881)?
                    };
                    nat_add(a4882, a4883)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f219(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f243(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f220(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f244(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f221(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4884 = v0;
    let a4885 = 64u64;
    if nat_eq(a4884, a4885) {
        f245(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4886 = v0;
        let a4887 = 65u64;
        if nat_eq(a4886, a4887) {
            f246(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f222(v0: u64) -> u64 {
    let a4888 = v0;
    let a4889 = 30u64;
    if nat_eq(a4888, a4889) {
        4u64
    } else {
        3u64
    }
}

pub fn f223(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f224(v0: u64) -> bool {
    let a4890 = 0u64;
    let a4891 = v0;
    if nat_lt(a4890, a4891) {
        let a4892 = v0;
        let a4893 = 2139095040u64;
        nat_le(a4892, a4893)
    } else {
        false
    }
}

pub fn f225(v0: u64) -> u64 {
    let a4894 = v0;
    let a4895 = 40u64;
    if nat_lt(a4894, a4895) {
        let a4896 = v0;
        let a4897 = 20u64;
        if nat_lt(a4896, a4897) {
            let a4898 = v0;
            let a4899 = 10u64;
            if nat_lt(a4898, a4899) {
                let a4900 = v0;
                let a4901 = 5u64;
                if nat_lt(a4900, a4901) {
                    let a4902 = v0;
                    let a4903 = 2u64;
                    if nat_lt(a4902, a4903) {
                        let a4904 = v0;
                        let a4905 = 1u64;
                        if nat_lt(a4904, a4905) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4906 = v0;
                        let a4907 = 3u64;
                        if nat_lt(a4906, a4907) {
                            4u64
                        } else {
                            let a4908 = v0;
                            let a4909 = 4u64;
                            if nat_lt(a4908, a4909) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4910 = v0;
                    let a4911 = 7u64;
                    if nat_lt(a4910, a4911) {
                        let a4912 = v0;
                        let a4913 = 6u64;
                        if nat_lt(a4912, a4913) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4914 = v0;
                        let a4915 = 8u64;
                        if nat_lt(a4914, a4915) {
                            128u64
                        } else {
                            let a4916 = v0;
                            let a4917 = 9u64;
                            if nat_lt(a4916, a4917) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4918 = v0;
                let a4919 = 15u64;
                if nat_lt(a4918, a4919) {
                    let a4920 = v0;
                    let a4921 = 12u64;
                    if nat_lt(a4920, a4921) {
                        let a4922 = v0;
                        let a4923 = 11u64;
                        if nat_lt(a4922, a4923) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4924 = v0;
                        let a4925 = 13u64;
                        if nat_lt(a4924, a4925) {
                            4096u64
                        } else {
                            let a4926 = v0;
                            let a4927 = 14u64;
                            if nat_lt(a4926, a4927) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4928 = v0;
                    let a4929 = 17u64;
                    if nat_lt(a4928, a4929) {
                        let a4930 = v0;
                        let a4931 = 16u64;
                        if nat_lt(a4930, a4931) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4932 = v0;
                        let a4933 = 18u64;
                        if nat_lt(a4932, a4933) {
                            131072u64
                        } else {
                            let a4934 = v0;
                            let a4935 = 19u64;
                            if nat_lt(a4934, a4935) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4936 = v0;
            let a4937 = 30u64;
            if nat_lt(a4936, a4937) {
                let a4938 = v0;
                let a4939 = 25u64;
                if nat_lt(a4938, a4939) {
                    let a4940 = v0;
                    let a4941 = 22u64;
                    if nat_lt(a4940, a4941) {
                        let a4942 = v0;
                        let a4943 = 21u64;
                        if nat_lt(a4942, a4943) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4944 = v0;
                        let a4945 = 23u64;
                        if nat_lt(a4944, a4945) {
                            4194304u64
                        } else {
                            let a4946 = v0;
                            let a4947 = 24u64;
                            if nat_lt(a4946, a4947) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4948 = v0;
                    let a4949 = 27u64;
                    if nat_lt(a4948, a4949) {
                        let a4950 = v0;
                        let a4951 = 26u64;
                        if nat_lt(a4950, a4951) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4952 = v0;
                        let a4953 = 28u64;
                        if nat_lt(a4952, a4953) {
                            134217728u64
                        } else {
                            let a4954 = v0;
                            let a4955 = 29u64;
                            if nat_lt(a4954, a4955) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4956 = v0;
                let a4957 = 35u64;
                if nat_lt(a4956, a4957) {
                    let a4958 = v0;
                    let a4959 = 32u64;
                    if nat_lt(a4958, a4959) {
                        let a4960 = v0;
                        let a4961 = 31u64;
                        if nat_lt(a4960, a4961) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4962 = v0;
                        let a4963 = 33u64;
                        if nat_lt(a4962, a4963) {
                            4294967296u64
                        } else {
                            let a4964 = v0;
                            let a4965 = 34u64;
                            if nat_lt(a4964, a4965) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4966 = v0;
                    let a4967 = 37u64;
                    if nat_lt(a4966, a4967) {
                        let a4968 = v0;
                        let a4969 = 36u64;
                        if nat_lt(a4968, a4969) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4970 = v0;
                        let a4971 = 38u64;
                        if nat_lt(a4970, a4971) {
                            137438953472u64
                        } else {
                            let a4972 = v0;
                            let a4973 = 39u64;
                            if nat_lt(a4972, a4973) {
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

pub fn f226(v0: u64, v1: u64) -> bool {
    let a4974 = v0;
    let a4975 = 1u64;
    if nat_le(a4974, a4975) {
        true
    } else {
        let a4979 = {
            let a4976 = v1;
            let a4977 = 4u64;
            let a4978 = 0u64;
            nat_rem(a4976, a4977, a4978)
        };
        let a4980 = 0u64;
        nat_eq(a4979, a4980)
    }
}

pub fn f227(v0: u64, v1: u64) -> bool {
    let a4981 = v0;
    let a4982 = 1u64;
    if nat_le(a4981, a4982) {
        true
    } else {
        let a4983 = 4u64;
        let a4984 = v1;
        nat_le(a4983, a4984)
    }
}

pub fn f228(v0: u64, v1: u64) -> R<u64> {
    let a4985 = v0;
    let a4986 = 0u64;
    if nat_eq(a4985, a4986) {
        Ok(0u64)
    } else {
        let a4991 = 4u64;
        let a4992 = {
            let a4989 = {
                let a4987 = v0;
                let a4988 = 1u64;
                nat_sub(a4987, a4988)
            };
            let a4990 = v1;
            nat_mul(a4989, a4990)?
        };
        nat_add(a4991, a4992)
    }
}

pub fn f229(v0: bool, v1: u64) -> R<u64> {
    let a4997 = {
        let a4995 = {
            let a4993 = f247(v0, v1)?;
            let a4994 = f248(v0, v1)?;
            nat_add(a4993, a4994)?
        };
        let a4996 = f249(v0);
        nat_add(a4995, a4996)?
    };
    let a4998 = v1;
    nat_add(a4997, a4998)
}

pub fn f230(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f250(v0, v1)?;
    let v3: u64 = {
        let a5003 = {
            let a5001 = {
                let a4999 = v1;
                let a5000 = 1u64;
                nat_sub(a4999, a5000)
            };
            let a5002 = v2;
            nat_mul(a5001, a5002)?
        };
        let a5004 = v0;
        nat_sub(a5003, a5004)
    };
    let a5005 = v2;
    let a5006 = v3;
    if nat_lt(a5005, a5006) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f231(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5007 = v4;
    let a5008 = 0u64;
    if nat_eq(a5007, a5008) {
        Ok(0u64)
    } else {
        let a5015 = 2u64;
        let a5016 = {
            let a5013 = {
                let a5009 = v0;
                let a5010 = v1;
                nat_mul(a5009, a5010)?
            };
            let a5014 = {
                let a5011 = v2;
                let a5012 = v3;
                nat_mul(a5011, a5012)?
            };
            nat_mul(a5013, a5014)?
        };
        nat_mul(a5015, a5016)
    }
}

pub fn f232(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5021 = v3;
    let a5022 = {
        let a5019 = {
            let a5017 = f251(v2, v4)?;
            let a5018 = f252(v0, v1, f250(v2, v4)?)?;
            nat_mul(a5017, a5018)?
        };
        let a5020 = f252(v0, v1, f253(v2, v4)?)?;
        nat_add(a5019, a5020)?
    };
    nat_mul(a5021, a5022)
}

pub fn f233(v0: u64) -> R<u64> {
    let a5025 = 13u64;
    let a5026 = {
        let a5023 = 9u64;
        let a5024 = v0;
        nat_mul(a5023, a5024)?
    };
    nat_add(a5025, a5026)
}

pub fn f234(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f196(v1, v2, v3, v4) {
        f254(v0)
    } else {
        f116(v0)
    }
}

pub fn f235(v0: u64) -> u64 {
    let a5027 = v0;
    let a5028 = 26u64;
    if nat_eq(a5027, a5028) {
        26u64
    } else {
        0u64
    }
}

pub fn f236(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a5049 = {
        let a5043 = {
            let a5037 = {
                let a5031 = v1;
                let a5032 = if v5 {
                    f136(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5029, _, _, _) => {
                            h5029
                        }
                    })
                } else {
                    f137(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5030, _, _, _) => {
                            h5030
                        }
                    })
                };
                nat_mul(a5031, a5032)?
            };
            let a5038 = {
                let a5035 = v2;
                let a5036 = if v5 {
                    f136(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5033, _, _) => {
                            h5033
                        }
                    })
                } else {
                    f137(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5034, _, _) => {
                            h5034
                        }
                    })
                };
                nat_mul(a5035, a5036)?
            };
            nat_add(a5037, a5038)?
        };
        let a5044 = {
            let a5041 = v3;
            let a5042 = if v5 {
                f136(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5039, _) => {
                        h5039
                    }
                })
            } else {
                f137(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5040, _) => {
                        h5040
                    }
                })
            };
            nat_mul(a5041, a5042)?
        };
        nat_add(a5043, a5044)?
    };
    let a5050 = {
        let a5047 = v4;
        let a5048 = if v5 {
            f136(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5045) => {
                    h5045
                }
            })
        } else {
            f137(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5046) => {
                    h5046
                }
            })
        };
        nat_mul(a5047, a5048)?
    };
    nat_add(a5049, a5050)
}

pub fn f237(v0: u64, v1: u64) -> Option<u64> {
    let a5051 = v0;
    let a5052 = 2u64;
    if nat_eq(a5051, a5052) {
        Some(0u64)
    } else {
        let a5053 = v0;
        let a5054 = 5u64;
        if nat_eq(a5053, a5054) {
            Some(1u64)
        } else {
            let a5055 = v0;
            let a5056 = 6u64;
            if nat_eq(a5055, a5056) {
                Some(2u64)
            } else {
                let a5057 = v0;
                let a5058 = 7u64;
                if nat_eq(a5057, a5058) {
                    Some(3u64)
                } else {
                    let a5059 = v0;
                    let a5060 = 9u64;
                    if nat_eq(a5059, a5060) {
                        Some(4u64)
                    } else {
                        let a5061 = v0;
                        let a5062 = 8u64;
                        if nat_eq(a5061, a5062) {
                            Some(5u64)
                        } else {
                            let a5063 = v0;
                            let a5064 = 89u64;
                            if nat_eq(a5063, a5064) {
                                let a5065 = v1;
                                let a5066 = 2u64;
                                if nat_eq(a5065, a5066) {
                                    Some(6u64)
                                } else {
                                    let a5067 = v1;
                                    let a5068 = 0u64;
                                    if nat_eq(a5067, a5068) {
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

pub fn f238(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5069 = v0;
    let a5070 = 30u64;
    if nat_eq(a5069, a5070) {
        let a5071 = v2;
        let a5072 = 0u64;
        if if if nat_eq(a5071, a5072) {
            true
        } else {
            let a5073 = v2;
            let a5074 = 1u64;
            nat_eq(a5073, a5074)
        } {
            let a5075 = v1;
            let a5076 = 0u64;
            if nat_eq(a5075, a5076) {
                true
            } else {
                f255(v1, v4)
            }
        } else {
            false
        } {
            f256(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f257(v1) {
            f257(v2)
        } else {
            false
        } {
            let a5077 = v1;
            let a5078 = 0u64;
            if if nat_eq(a5077, a5078) {
                true
            } else {
                let a5079 = v2;
                let a5080 = 0u64;
                nat_eq(a5079, a5080)
            } {
                true
            } else {
                let a5081 = v1;
                let a5082 = v2;
                nat_eq(a5081, a5082)
            }
        } else {
            false
        } {
            true
        } else {
            let a5083 = v1;
            let a5084 = 43u64;
            if nat_eq(a5083, a5084) {
                f257(v2)
            } else {
                false
            }
        } {
            f256(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f239(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5085 = v1;
    let a5086 = 0u64;
    if nat_eq(a5085, a5086) {
        Ok(Adt9::C4({
            let a5087 = v3;
            let a5088 = 0u64;
            if nat_eq(a5087, a5088) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f143(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5089 = v3;
            let a5090 = 0u64;
            if nat_eq(a5089, a5090) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f240(v0: u64, v1: u64) -> R<u64> {
    let a5091 = v0;
    let a5092 = f96(v1)?;
    let a5093 = 0u64;
    Ok(nat_quot(a5091, a5092, a5093))
}

pub fn f241(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f240(v0, v1)?;
    let v3: u64 = {
        let a5096 = {
            let a5094 = f214(v0, v1)?;
            let a5095 = 2u64;
            nat_sub(a5094, a5095)
        };
        let a5097 = 2u64;
        let a5098 = 0u64;
        nat_quot(a5096, a5097, a5098)
    };
    let v4: u64 = {
        let a5099 = v2;
        let a5100 = v1;
        nat_mul(a5099, a5100)?
    };
    if f217(v0, v1)? {
        let a5101 = v4;
        let a5102 = v3;
        nat_add(a5101, a5102)
    } else {
        let a5114 = {
            let a5110 = v4;
            let a5111 = {
                let a5108 = {
                    let a5103 = v1;
                    let a5104 = 1u64;
                    nat_sub(a5103, a5104)
                };
                let a5109 = {
                    let a5105 = v2;
                    let a5106 = v1;
                    let a5107 = 0u64;
                    nat_rem(a5105, a5106, a5107)
                };
                nat_mul(a5108, a5109)?
            };
            nat_sub(a5110, a5111)
        };
        let a5115 = {
            let a5112 = v3;
            let a5113 = v1;
            nat_mul(a5112, a5113)?
        };
        nat_add(a5114, a5115)
    }
}

pub fn f242(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5116 = v0;
    let a5117 = 0u64;
    if if if nat_eq(a5116, a5117) {
        true
    } else {
        let a5118 = v1;
        let a5119 = 0u64;
        nat_eq(a5118, a5119)
    } {
        true
    } else {
        let a5120 = v2;
        let a5121 = 0u64;
        nat_eq(a5120, a5121)
    } {
        Ok(0u64)
    } else {
        let a5124 = {
            let a5122 = v0;
            let a5123 = v1;
            nat_mul(a5122, a5123)?
        };
        let a5125 = v2;
        nat_mul(a5124, a5125)
    }
}

pub fn f243(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f105(match v1.clone() {
        Adt20::C0(h5126, _, _, _, _, _, _, _) => {
            h5126
        }
    })?;
    let v6: u64 = {
        let a5132 = match v1.clone() {
            Adt20::C0(_, h5127, _, _, _, _, _, _) => {
                h5127
            }
        };
        let a5133 = {
            let a5130 = match v1.clone() {
                Adt20::C0(_, _, h5128, _, _, _, _, _) => {
                    h5128
                }
            };
            let a5131 = match v1.clone() {
                Adt20::C0(_, _, _, h5129, _, _, _, _) => {
                    h5129
                }
            };
            nat_mul(a5130, a5131)?
        };
        nat_mul(a5132, a5133)?
    };
    let a5135 = v0;
    let a5136 = f39(match v1.clone() {
        Adt20::C0(h5134, _, _, _, _, _, _, _) => {
            h5134
        }
    }, v6)?;
    if nat_lt(a5135, a5136) {
        f258({
            let a5137 = v0;
            let a5138 = v5;
            let a5139 = 0u64;
            nat_rem(a5137, a5138, a5139)
        }, {
            let a5140 = v0;
            let a5141 = v5;
            let a5142 = 0u64;
            nat_quot(a5140, a5141, a5142)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f244(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5143 = v6;
        let a5144 = 6u64;
        nat_sub(a5143, a5144)
    };
    let v9: Adt20 = f259(v3);
    let a5146 = v0;
    let a5147 = f192(match v1.clone() {
        Adt20::C0(h5145, _, _, _, _, _, _, _) => {
            h5145
        }
    }, v5, v8)?;
    if nat_lt(a5146, a5147) {
        let v10: u64 = f102(match v1.clone() {
            Adt20::C0(h5148, _, _, _, _, _, _, _) => {
                h5148
            }
        }, v5, v8)?;
        let a5149 = v0;
        let a5150 = v10;
        if nat_lt(a5149, a5150) {
            f260(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5151 = v0;
                let a5152 = v10;
                nat_sub(a5151, a5152)
            };
            let v12: u64 = f233(match v1.clone() {
                Adt20::C0(h5153, _, _, _, _, _, _, _) => {
                    h5153
                }
            })?;
            f261({
                let a5154 = v11;
                let a5155 = v12;
                let a5156 = 0u64;
                nat_rem(a5154, a5155, a5156)
            }, {
                let a5157 = v11;
                let a5158 = v12;
                let a5159 = 0u64;
                nat_quot(a5157, a5158, a5159)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f245(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5160 = v5;
        let a5161 = 6u64;
        nat_sub(a5160, a5161)
    };
    let v8: bool = f186(match v1.clone() {
        Adt20::C0(_, _, _, _, h5162, _, _, _) => {
            h5162
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5163, _, _, _) => {
            h5163
        }
    });
    let a5165 = v0;
    let a5166 = f185(v8, match v1.clone() {
        Adt20::C0(h5164, _, _, _, _, _, _, _) => {
            h5164
        }
    }, v4, v7)?;
    if nat_lt(a5165, a5166) {
        let v9: u64 = f102(match v1.clone() {
            Adt20::C0(h5167, _, _, _, _, _, _, _) => {
                h5167
            }
        }, v4, v7)?;
        let a5168 = v0;
        let a5169 = v9;
        if nat_lt(a5168, a5169) {
            f260(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5170 = v0;
                let a5171 = v9;
                nat_sub(a5170, a5171)
            };
            let v11: u64 = f229(v8, match v1.clone() {
                Adt20::C0(h5172, _, _, _, _, _, _, _) => {
                    h5172
                }
            })?;
            f262({
                let a5173 = v10;
                let a5174 = v11;
                let a5175 = 0u64;
                nat_rem(a5173, a5174, a5175)
            }, {
                let a5176 = v10;
                let a5177 = v11;
                let a5178 = 0u64;
                nat_quot(a5176, a5177, a5178)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f246(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5179 = v5;
        let a5180 = 6u64;
        nat_sub(a5179, a5180)
    };
    let v8: Adt20 = f259(v2);
    let v9: u64 = f231(match v1.clone() {
        Adt20::C0(h5181, _, _, _, _, _, _, _) => {
            h5181
        }
    }, match v1.clone() {
        Adt20::C0(_, h5182, _, _, _, _, _, _) => {
            h5182
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5183, _, _, _, _, _) => {
            h5183
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5184, _, _, _, _) => {
            h5184
        }
    }, v7)?;
    let a5189 = v0;
    let a5190 = f191(match v1.clone() {
        Adt20::C0(h5185, _, _, _, _, _, _, _) => {
            h5185
        }
    }, match v1.clone() {
        Adt20::C0(_, h5186, _, _, _, _, _, _) => {
            h5186
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5187, _, _, _, _, _) => {
            h5187
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5188, _, _, _, _) => {
            h5188
        }
    }, v4, v7)?;
    if nat_lt(a5189, a5190) {
        let a5191 = v0;
        let a5192 = v9;
        if nat_lt(a5191, a5192) {
            f260(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f263({
                let a5193 = v0;
                let a5194 = v9;
                nat_sub(a5193, a5194)
            }, match v1.clone() {
                Adt20::C0(h5195, _, _, _, _, _, _, _) => {
                    h5195
                }
            }, match v1.clone() {
                Adt20::C0(_, h5196, _, _, _, _, _, _) => {
                    h5196
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5197, _, _, _, _, _) => {
                    h5197
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5198, _, _, _, _) => {
                    h5198
                }
            }, v4)?;
            f264(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f247(v0: bool, v1: u64) -> R<u64> {
    let a5201 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5202 = {
        let a5199 = f265(v0);
        let a5200 = v1;
        nat_mul(a5199, a5200)?
    };
    nat_add(a5201, a5202)
}

pub fn f248(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5210 = {
            let a5203 = 13u64;
            let a5204 = f266(v1);
            nat_mul(a5203, a5204)?
        };
        let a5211 = {
            let a5208 = 4u64;
            let a5209 = {
                let a5205 = v1;
                let a5206 = 4u64;
                let a5207 = 0u64;
                nat_rem(a5205, a5206, a5207)
            };
            nat_mul(a5208, a5209)?
        };
        nat_add(a5210, a5211)
    } else {
        let a5212 = 3u64;
        let a5213 = v1;
        nat_mul(a5212, a5213)
    }
}

pub fn f249(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f250(v0: u64, v1: u64) -> R<u64> {
    let a5218 = {
        let a5216 = {
            let a5214 = v0;
            let a5215 = v1;
            nat_add(a5214, a5215)?
        };
        let a5217 = 1u64;
        nat_sub(a5216, a5217)
    };
    let a5219 = v1;
    let a5220 = 0u64;
    Ok(nat_quot(a5218, a5219, a5220))
}

pub fn f251(v0: u64, v1: u64) -> R<u64> {
    let a5221 = v0;
    let a5222 = f250(v0, v1)?;
    let a5223 = 0u64;
    Ok(nat_quot(a5221, a5222, a5223))
}

pub fn f252(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5224 = v2;
    let a5225 = 0u64;
    if nat_eq(a5224, a5225) {
        Ok(0u64)
    } else {
        let a5230 = 11u64;
        let a5231 = {
            let a5228 = {
                let a5226 = v1;
                let a5227 = v2;
                nat_mul(a5226, a5227)?
            };
            let a5229 = f267(v0)?;
            nat_mul(a5228, a5229)?
        };
        nat_add(a5230, a5231)
    }
}

pub fn f253(v0: u64, v1: u64) -> R<u64> {
    let a5232 = v0;
    let a5233 = f250(v0, v1)?;
    let a5234 = 0u64;
    Ok(nat_rem(a5232, a5233, a5234))
}

pub fn f254(v0: u64) -> bool {
    let m5235 = f118(v0);
    match m5235 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f255(v0: u64, v1: u64) -> bool {
    let a5236 = v0;
    let a5237 = 1u64;
    if nat_eq(a5236, a5237) {
        true
    } else {
        let v2: u64 = f268(v0);
        let a5238 = 0u64;
        let a5239 = v2;
        if nat_lt(a5238, a5239) {
            let a5243 = {
                let a5240 = v1;
                let a5241 = v2;
                let a5242 = 1u64;
                nat_rem(a5240, a5241, a5242)
            };
            let a5244 = 0u64;
            nat_eq(a5243, a5244)
        } else {
            false
        }
    }
}

pub fn f256(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5245 = v0;
    let a5246 = 28u64;
    if if if if nat_eq(a5245, a5246) {
        let a5247 = v1;
        let a5248 = 0u64;
        nat_eq(a5247, a5248)
    } else {
        false
    } {
        let a5249 = v2;
        let a5250 = 0u64;
        nat_eq(a5249, a5250)
    } else {
        false
    } {
        let a5251 = v3;
        let a5252 = 0u64;
        nat_eq(a5251, a5252)
    } else {
        false
    } {
        let a5253 = v4;
        let a5254 = v8;
        if if if if if if if if nat_eq(a5253, a5254) {
            let a5255 = v5;
            let a5256 = v12;
            nat_eq(a5255, a5256)
        } else {
            false
        } {
            let a5257 = v9;
            let a5258 = v13;
            nat_eq(a5257, a5258)
        } else {
            false
        } {
            let a5259 = v10;
            let a5260 = v14;
            nat_eq(a5259, a5260)
        } else {
            false
        } {
            let a5261 = v11;
            let a5262 = v15;
            nat_eq(a5261, a5262)
        } else {
            false
        } {
            let a5263 = 0u64;
            let a5264 = v6;
            nat_lt(a5263, a5264)
        } else {
            false
        } {
            let a5265 = 0u64;
            let a5266 = v7;
            nat_lt(a5265, a5266)
        } else {
            false
        } {
            let a5270 = {
                let a5267 = v10;
                let a5268 = v6;
                let a5269 = 1u64;
                nat_rem(a5267, a5268, a5269)
            };
            let a5271 = 0u64;
            if nat_eq(a5270, a5271) {
                let a5275 = {
                    let a5272 = v11;
                    let a5273 = v7;
                    let a5274 = 1u64;
                    nat_rem(a5272, a5273, a5274)
                };
                let a5276 = 0u64;
                nat_eq(a5275, a5276)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5277 = v10;
                let a5278 = v6;
                let a5279 = 0u64;
                nat_quot(a5277, a5278, a5279)
            }, {
                let a5280 = v11;
                let a5281 = v7;
                let a5282 = 0u64;
                nat_quot(a5280, a5281, a5282)
            }, {
                let a5283 = v5;
                let a5284 = v4;
                nat_mul(a5283, a5284)?
            }, {
                let a5285 = v4;
                let a5286 = v9;
                nat_mul(a5285, a5286)?
            }, {
                let a5287 = v5;
                let a5288 = v9;
                nat_mul(a5287, a5288)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f257(v0: u64) -> bool {
    let a5289 = v0;
    let a5290 = 0u64;
    if if nat_eq(a5289, a5290) {
        true
    } else {
        let a5291 = v0;
        let a5292 = 1u64;
        nat_eq(a5291, a5292)
    } {
        true
    } else {
        let a5293 = v0;
        let a5294 = 30u64;
        nat_eq(a5293, a5294)
    }
}

pub fn f258(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5295, _, _, _, _, _, _, _) => {
            h5295
        }
    };
    let v7: u64 = {
        let a5298 = 1u64;
        let a5299 = {
            let a5296 = 3u64;
            let a5297 = v6;
            nat_mul(a5296, a5297)?
        };
        nat_add(a5298, a5299)?
    };
    let a5300 = v0;
    let a5301 = 0u64;
    if nat_eq(a5300, a5301) {
        Ok(f269(0u64))
    } else {
        let a5302 = v0;
        let a5303 = v7;
        if nat_lt(a5302, a5303) {
            let v8: u64 = {
                let a5304 = v0;
                let a5305 = 1u64;
                nat_sub(a5304, a5305)
            };
            let v9: u64 = {
                let a5311 = {
                    let a5306 = v1;
                    let a5307 = v6;
                    nat_mul(a5306, a5307)?
                };
                let a5312 = {
                    let a5308 = v8;
                    let a5309 = 3u64;
                    let a5310 = 0u64;
                    nat_quot(a5308, a5309, a5310)
                };
                nat_add(a5311, a5312)?
            };
            let v10: u64 = f270(v9, v2)?;
            let v11: u64 = {
                let a5313 = v8;
                let a5314 = 3u64;
                let a5315 = 0u64;
                nat_rem(a5313, a5314, a5315)
            };
            let a5316 = v11;
            let a5317 = 0u64;
            if nat_eq(a5316, a5317) {
                Ok(f271(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5318 = v11;
                let a5319 = 1u64;
                if nat_eq(a5318, a5319) {
                    Ok(f271(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f271(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5320 = v0;
            let a5321 = v7;
            if nat_eq(a5320, a5321) {
                Ok(f271(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5324 = v0;
                let a5325 = {
                    let a5322 = v7;
                    let a5323 = 1u64;
                    nat_add(a5322, a5323)?
                };
                if nat_eq(a5324, a5325) {
                    Ok(f271(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5328 = v0;
                    let a5329 = {
                        let a5326 = v7;
                        let a5327 = 2u64;
                        nat_add(a5326, a5327)?
                    };
                    if nat_eq(a5328, a5329) {
                        Ok(f271(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5332 = v0;
                        let a5333 = {
                            let a5330 = v7;
                            let a5331 = 3u64;
                            nat_add(a5330, a5331)?
                        };
                        if nat_eq(a5332, a5333) {
                            let a5334 = v5;
                            let a5335 = 1u64;
                            if if nat_eq(a5334, a5335) {
                                true
                            } else {
                                let a5336 = v5;
                                let a5337 = 2u64;
                                nat_eq(a5336, a5337)
                            } {
                                Ok(f271(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f271(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5344 = {
                                    let a5338 = v1;
                                    let a5339 = v6;
                                    nat_mul(a5338, a5339)?
                                };
                                let a5345 = {
                                    let a5342 = v0;
                                    let a5343 = {
                                        let a5340 = v7;
                                        let a5341 = 4u64;
                                        nat_add(a5340, a5341)?
                                    };
                                    nat_sub(a5342, a5343)
                                };
                                nat_add(a5344, a5345)?
                            };
                            Ok(f271(10u64, 0u64, f270(v12, v2)?, 1u64, 2u64, 3u64, f270(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f259(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5346, _, _, _, _, _, _, _) => {
            h5346
        }
    }, match v0.clone() {
        Adt20::C0(_, h5347, _, _, _, _, _, _) => {
            h5347
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5348, _, _, _, _, _) => {
            h5348
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5349, _, _, _, _) => {
            h5349
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5350, _, _) => {
            h5350
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5351, _) => {
            h5351
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5352) => {
            h5352
        }
    })
}

pub fn f260(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5359 = v3;
        let a5360 = {
            let a5356 = f270({
                let a5353 = v0;
                let a5354 = 2u64;
                let a5355 = 0u64;
                nat_quot(a5353, a5354, a5355)
            }, v1)?;
            let a5357 = v2;
            let a5358 = 0u64;
            nat_quot(a5356, a5357, a5358)
        };
        nat_add(a5359, a5360)?
    };
    let a5364 = {
        let a5361 = v0;
        let a5362 = 2u64;
        let a5363 = 0u64;
        nat_rem(a5361, a5362, a5363)
    };
    let a5365 = 0u64;
    if nat_eq(a5364, a5365) {
        Ok(f271(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5366 = v4;
        let a5367 = 0u64;
        if nat_eq(a5366, a5367) {
            Ok(f271(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f261(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f272(match v2.clone() {
        Adt20::C0(h5368, _, _, _, _, _, _, _) => {
            h5368
        }
    })?;
    let v7: u64 = f273(match v2.clone() {
        Adt20::C0(h5369, _, _, _, _, _, _, _) => {
            h5369
        }
    })?;
    let a5370 = v0;
    let a5371 = 2u64;
    if nat_lt(a5370, a5371) {
        Ok(f269(v0))
    } else {
        let a5372 = v0;
        let a5373 = v6;
        if nat_lt(a5372, a5373) {
            f274({
                let a5374 = v0;
                let a5375 = 2u64;
                nat_sub(a5374, a5375)
            }, v1, v2, v3)
        } else {
            let a5376 = v0;
            let a5377 = v7;
            if nat_lt(a5376, a5377) {
                Ok(f275({
                    let a5378 = v0;
                    let a5379 = v6;
                    nat_sub(a5378, a5379)
                }, match v2 {
                    Adt20::C0(h5380, _, _, _, _, _, _, _) => {
                        h5380
                    }
                }, v5))
            } else {
                let a5382 = v0;
                let a5383 = f233(match v2.clone() {
                    Adt20::C0(h5381, _, _, _, _, _, _, _) => {
                        h5381
                    }
                })?;
                if nat_lt(a5382, a5383) {
                    f276({
                        let a5384 = v0;
                        let a5385 = v7;
                        nat_sub(a5384, a5385)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f262(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f247(v5, match v2.clone() {
        Adt20::C0(h5386, _, _, _, _, _, _, _) => {
            h5386
        }
    })?;
    let v7: u64 = {
        let a5388 = v6;
        let a5389 = f248(v5, match v2.clone() {
            Adt20::C0(h5387, _, _, _, _, _, _, _) => {
                h5387
            }
        })?;
        nat_add(a5388, a5389)?
    };
    let v8: u64 = {
        let a5390 = v7;
        let a5391 = f249(v5);
        nat_add(a5390, a5391)?
    };
    let a5392 = v0;
    let a5393 = v6;
    if nat_lt(a5392, a5393) {
        f277(v0, v1, v2, v5)
    } else {
        let a5394 = v0;
        let a5395 = v7;
        if nat_lt(a5394, a5395) {
            f278({
                let a5396 = v0;
                let a5397 = v6;
                nat_sub(a5396, a5397)
            }, v1, v2, v3, v5)
        } else {
            let a5398 = v0;
            let a5399 = v8;
            if nat_lt(a5398, a5399) {
                Ok(f279({
                    let a5400 = v0;
                    let a5401 = v7;
                    nat_sub(a5400, a5401)
                }, match v2 {
                    Adt20::C0(h5402, _, _, _, _, _, _, _) => {
                        h5402
                    }
                }, v4, v5))
            } else {
                let a5406 = v0;
                let a5407 = {
                    let a5404 = v8;
                    let a5405 = match v2.clone() {
                        Adt20::C0(h5403, _, _, _, _, _, _, _) => {
                            h5403
                        }
                    };
                    nat_add(a5404, a5405)?
                };
                if nat_lt(a5406, a5407) {
                    let v9: u64 = f270({
                        let a5413 = {
                            let a5409 = v1;
                            let a5410 = match v2 {
                                Adt20::C0(h5408, _, _, _, _, _, _, _) => {
                                    h5408
                                }
                            };
                            nat_mul(a5409, a5410)?
                        };
                        let a5414 = {
                            let a5411 = v0;
                            let a5412 = v8;
                            nat_sub(a5411, a5412)
                        };
                        nat_add(a5413, a5414)?
                    }, v3)?;
                    Ok(f271(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f263(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f250(v3, v5)?;
    let v7: u64 = f252(v1, v2, v6)?;
    let v8: u64 = {
        let a5417 = {
            let a5415 = f251(v3, v5)?;
            let a5416 = v4;
            nat_mul(a5415, a5416)?
        };
        let a5418 = v7;
        nat_mul(a5417, a5418)?
    };
    let a5419 = v0;
    let a5420 = v8;
    if nat_lt(a5419, a5420) {
        let v9: u64 = {
            let a5421 = v0;
            let a5422 = v7;
            let a5423 = 0u64;
            nat_quot(a5421, a5422, a5423)
        };
        Ok(Adt22::C0({
            let a5427 = {
                let a5424 = v9;
                let a5425 = v4;
                let a5426 = 0u64;
                nat_quot(a5424, a5425, a5426)
            };
            let a5428 = v6;
            nat_mul(a5427, a5428)?
        }, v6, {
            let a5429 = v9;
            let a5430 = v4;
            let a5431 = 0u64;
            nat_rem(a5429, a5430, a5431)
        }, {
            let a5432 = v0;
            let a5433 = v7;
            let a5434 = 0u64;
            nat_rem(a5432, a5433, a5434)
        }))
    } else {
        let v10: u64 = {
            let a5435 = v0;
            let a5436 = v8;
            nat_sub(a5435, a5436)
        };
        let v11: u64 = f252(v1, v2, f253(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5437 = f251(v3, v5)?;
            let a5438 = v6;
            nat_mul(a5437, a5438)?
        }, f253(v3, v5)?, {
            let a5439 = v10;
            let a5440 = v11;
            let a5441 = 0u64;
            nat_quot(a5439, a5440, a5441)
        }, {
            let a5442 = v10;
            let a5443 = v11;
            let a5444 = 0u64;
            nat_rem(a5442, a5443, a5444)
        }))
    }
}

pub fn f264(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5445) => {
            h5445
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5446, _, _, _, _, _, _, _) => {
            h5446
        }
    };
    let v6: u64 = {
        let a5449 = match v0.clone() {
            Adt22::C0(_, h5447, _, _) => {
                h5447
            }
        };
        let a5450 = match v1.clone() {
            Adt20::C0(_, h5448, _, _, _, _, _, _) => {
                h5448
            }
        };
        nat_mul(a5449, a5450)?
    };
    let v7: u64 = {
        let a5451 = v6;
        let a5452 = v5;
        nat_mul(a5451, a5452)?
    };
    let v8: u64 = {
        let a5455 = {
            let a5453 = 2u64;
            let a5454 = v5;
            nat_mul(a5453, a5454)?
        };
        let a5456 = 2u64;
        nat_add(a5455, a5456)?
    };
    let v9: u64 = {
        let a5459 = {
            let a5457 = 4u64;
            let a5458 = v5;
            nat_mul(a5457, a5458)?
        };
        let a5460 = 2u64;
        nat_add(a5459, a5460)?
    };
    let v10: u64 = {
        let a5463 = 1u64;
        let a5464 = {
            let a5461 = v6;
            let a5462 = v8;
            nat_mul(a5461, a5462)?
        };
        nat_add(a5463, a5464)?
    };
    let v11: u64 = {
        let a5465 = v10;
        let a5466 = 4u64;
        nat_add(a5465, a5466)?
    };
    let v12: u64 = {
        let a5469 = v11;
        let a5470 = {
            let a5467 = v6;
            let a5468 = v9;
            nat_mul(a5467, a5468)?
        };
        nat_add(a5469, a5470)?
    };
    let v13: u64 = {
        let a5471 = v12;
        let a5472 = 6u64;
        nat_add(a5471, a5472)?
    };
    let a5473 = v4;
    let a5474 = 0u64;
    if nat_eq(a5473, a5474) {
        Ok(f269(0u64))
    } else {
        let a5475 = v4;
        let a5476 = v10;
        if nat_lt(a5475, a5476) {
            let v14: u64 = {
                let a5477 = v4;
                let a5478 = 1u64;
                nat_sub(a5477, a5478)
            };
            f280(false, {
                let a5479 = v14;
                let a5480 = v8;
                let a5481 = 0u64;
                nat_rem(a5479, a5480, a5481)
            }, {
                let a5482 = v14;
                let a5483 = v8;
                let a5484 = 0u64;
                nat_quot(a5482, a5483, a5484)
            }, v0, v1, v2)
        } else {
            let a5485 = v4;
            let a5486 = v11;
            if nat_lt(a5485, a5486) {
                Ok(f281({
                    let a5487 = v4;
                    let a5488 = v10;
                    nat_sub(a5487, a5488)
                }, v7))
            } else {
                let a5489 = v4;
                let a5490 = v12;
                if nat_lt(a5489, a5490) {
                    let v15: u64 = {
                        let a5491 = v4;
                        let a5492 = v11;
                        nat_sub(a5491, a5492)
                    };
                    f280(true, {
                        let a5493 = v15;
                        let a5494 = v9;
                        let a5495 = 0u64;
                        nat_rem(a5493, a5494, a5495)
                    }, {
                        let a5496 = v15;
                        let a5497 = v9;
                        let a5498 = 0u64;
                        nat_quot(a5496, a5497, a5498)
                    }, v0, v1, v2)
                } else {
                    let a5499 = v4;
                    let a5500 = v13;
                    if nat_lt(a5499, a5500) {
                        Ok(f282({
                            let a5501 = v4;
                            let a5502 = v12;
                            nat_sub(a5501, a5502)
                        }, v7, v3))
                    } else {
                        let a5505 = v4;
                        let a5506 = {
                            let a5503 = v13;
                            let a5504 = v7;
                            nat_add(a5503, a5504)?
                        };
                        if nat_lt(a5505, a5506) {
                            let v16: u64 = {
                                let a5507 = v4;
                                let a5508 = v13;
                                nat_sub(a5507, a5508)
                            };
                            let v17: u64 = {
                                let a5509 = v16;
                                let a5510 = v5;
                                let a5511 = 0u64;
                                nat_quot(a5509, a5510, a5511)
                            };
                            let v18: u64 = {
                                let a5517 = {
                                    let a5512 = f283(v17, v0, v1)?;
                                    let a5513 = v5;
                                    nat_mul(a5512, a5513)?
                                };
                                let a5518 = {
                                    let a5514 = v16;
                                    let a5515 = v5;
                                    let a5516 = 0u64;
                                    nat_rem(a5514, a5515, a5516)
                                };
                                nat_add(a5517, a5518)?
                            };
                            let v19: u64 = f270(v18, v2)?;
                            Ok(f271(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f265(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f266(v0: u64) -> u64 {
    let a5519 = v0;
    let a5520 = 4u64;
    let a5521 = 0u64;
    nat_quot(a5519, a5520, a5521)
}

pub fn f267(v0: u64) -> R<u64> {
    let a5524 = {
        let a5522 = 7u64;
        let a5523 = v0;
        nat_mul(a5522, a5523)?
    };
    let a5525 = 4u64;
    nat_add(a5524, a5525)
}

pub fn f268(v0: u64) -> u64 {
    if f116(v0) {
        let m5526 = f118(v0);
        match m5526 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5528 = 1u64;
                let a5529 = match v1.clone() {
                    Adt18::C0(_, h5527, _) => {
                        h5527
                    }
                };
                if if nat_lt(a5528, a5529) {
                    true
                } else {
                    let a5530 = v0;
                    let a5531 = 43u64;
                    nat_eq(a5530, a5531)
                } {
                    match v1 {
                        Adt18::C0(_, h5532, _) => {
                            h5532
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

pub fn f269(v0: u64) -> Adt19 {
    f271(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f270(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5534 = v0;
        let a5535 = match v1.clone() {
            Adt20::C0(h5533, _, _, _, _, _, _, _) => {
                h5533
            }
        };
        let a5536 = 0u64;
        nat_quot(a5534, a5535, a5536)
    };
    let v3: u64 = {
        let a5538 = v2;
        let a5539 = match v1.clone() {
            Adt20::C0(_, h5537, _, _, _, _, _, _) => {
                h5537
            }
        };
        let a5540 = 0u64;
        nat_quot(a5538, a5539, a5540)
    };
    f135({
        let a5542 = v0;
        let a5543 = match v1.clone() {
            Adt20::C0(h5541, _, _, _, _, _, _, _) => {
                h5541
            }
        };
        let a5544 = 0u64;
        nat_rem(a5542, a5543, a5544)
    }, {
        let a5546 = v2;
        let a5547 = match v1.clone() {
            Adt20::C0(_, h5545, _, _, _, _, _, _) => {
                h5545
            }
        };
        let a5548 = 0u64;
        nat_rem(a5546, a5547, a5548)
    }, {
        let a5550 = v3;
        let a5551 = match v1.clone() {
            Adt20::C0(_, _, h5549, _, _, _, _, _) => {
                h5549
            }
        };
        let a5552 = 0u64;
        nat_rem(a5550, a5551, a5552)
    }, {
        let a5554 = v3;
        let a5555 = match v1.clone() {
            Adt20::C0(_, _, h5553, _, _, _, _, _) => {
                h5553
            }
        };
        let a5556 = 0u64;
        nat_quot(a5554, a5555, a5556)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5557, _, _, _) => {
            h5557
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5558, _, _) => {
            h5558
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5559, _) => {
            h5559
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5560) => {
            h5560
        }
    })
}

pub fn f271(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5561 = v0;
    let a5562 = 4u64;
    if if if nat_le(a5561, a5562) {
        true
    } else {
        let a5563 = 9u64;
        let a5564 = v0;
        if nat_le(a5563, a5564) {
            let a5565 = v0;
            let a5566 = 12u64;
            nat_le(a5565, a5566)
        } else {
            false
        }
    } {
        true
    } else {
        let a5567 = v0;
        let a5568 = 16u64;
        if if nat_eq(a5567, a5568) {
            true
        } else {
            let a5569 = v0;
            let a5570 = 19u64;
            nat_eq(a5569, a5570)
        } {
            true
        } else {
            let a5571 = v0;
            let a5572 = 21u64;
            if nat_eq(a5571, a5572) {
                true
            } else {
                let a5573 = v0;
                let a5574 = 22u64;
                nat_eq(a5573, a5574)
            }
        }
    } {
        Adt19::C1({
            let a5575 = v0;
            let a5576 = 4u64;
            if if nat_lt(a5575, a5576) {
                true
            } else {
                let a5577 = 21u64;
                let a5578 = v0;
                nat_le(a5577, a5578)
            } {
                0u64
            } else {
                let a5579 = v0;
                let a5580 = 16u64;
                if if nat_eq(a5579, a5580) {
                    true
                } else {
                    let a5581 = v0;
                    let a5582 = 19u64;
                    nat_eq(a5581, a5582)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5583 = v0;
            let a5584 = 4u64;
            if nat_lt(a5583, a5584) {
                v0
            } else {
                let a5585 = v0;
                let a5586 = 4u64;
                if if nat_eq(a5585, a5586) {
                    true
                } else {
                    let a5587 = v0;
                    let a5588 = 16u64;
                    nat_eq(a5587, a5588)
                } {
                    0u64
                } else {
                    let a5589 = v0;
                    let a5590 = 19u64;
                    if nat_eq(a5589, a5590) {
                        3u64
                    } else {
                        let a5591 = 21u64;
                        let a5592 = v0;
                        if nat_le(a5591, a5592) {
                            let a5593 = v0;
                            let a5594 = 17u64;
                            nat_sub(a5593, a5594)
                        } else {
                            let a5595 = v0;
                            let a5596 = 8u64;
                            nat_sub(a5595, a5596)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f272(v0: u64) -> R<u64> {
    let a5599 = 2u64;
    let a5600 = {
        let a5597 = 6u64;
        let a5598 = v0;
        nat_mul(a5597, a5598)?
    };
    nat_add(a5599, a5600)
}

pub fn f273(v0: u64) -> R<u64> {
    let a5601 = f272(v0)?;
    let a5602 = 11u64;
    nat_add(a5601, a5602)
}

pub fn f274(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5603 = v0;
        let a5604 = 6u64;
        let a5605 = 0u64;
        nat_rem(a5603, a5604, a5605)
    };
    let v5: u64 = {
        let a5612 = {
            let a5607 = v1;
            let a5608 = match v2.clone() {
                Adt20::C0(h5606, _, _, _, _, _, _, _) => {
                    h5606
                }
            };
            nat_mul(a5607, a5608)?
        };
        let a5613 = {
            let a5609 = v0;
            let a5610 = 6u64;
            let a5611 = 0u64;
            nat_quot(a5609, a5610, a5611)
        };
        nat_add(a5612, a5613)?
    };
    let v6: u64 = f270(v5, v3)?;
    let v7: u64 = f270(v5, v2)?;
    let a5614 = v4;
    let a5615 = 0u64;
    if nat_eq(a5614, a5615) {
        Ok(f271(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5616 = v4;
        let a5617 = 1u64;
        if nat_eq(a5616, a5617) {
            Ok(f271(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5618 = v4;
            let a5619 = 2u64;
            if nat_eq(a5618, a5619) {
                Ok(f271(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5620 = v4;
                let a5621 = 3u64;
                if nat_eq(a5620, a5621) {
                    Ok(f271(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5622 = v4;
                    let a5623 = 4u64;
                    if nat_eq(a5622, a5623) {
                        Ok(f271(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f271(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f275(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5624 = v0;
    let a5625 = 0u64;
    if nat_eq(a5624, a5625) {
        f271(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5626 = v0;
        let a5627 = 1u64;
        if nat_eq(a5626, a5627) {
            f271(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5628 = v0;
            let a5629 = 2u64;
            if nat_eq(a5628, a5629) {
                f271(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5630 = v0;
                let a5631 = 3u64;
                if nat_eq(a5630, a5631) {
                    f271(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5632 = v0;
                    let a5633 = 4u64;
                    if nat_eq(a5632, a5633) {
                        f271(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5634 = v0;
                        let a5635 = 5u64;
                        if nat_eq(a5634, a5635) {
                            f271(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5636 = v0;
                            let a5637 = 6u64;
                            if nat_eq(a5636, a5637) {
                                f271(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5638 = v0;
                                let a5639 = 7u64;
                                if nat_eq(a5638, a5639) {
                                    f271(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5640 = v0;
                                    let a5641 = 8u64;
                                    if nat_eq(a5640, a5641) {
                                        f271(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5642 = v0;
                                        let a5643 = 9u64;
                                        if nat_eq(a5642, a5643) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5644 = v0;
                                            let a5645 = 10u64;
                                            if nat_eq(a5644, a5645) {
                                                f271(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
                                            } else {
                                                Adt19::C0
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f276(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5646 = v0;
        let a5647 = 3u64;
        let a5648 = 0u64;
        nat_rem(a5646, a5647, a5648)
    };
    let v6: u64 = {
        let a5655 = {
            let a5650 = v1;
            let a5651 = match v2.clone() {
                Adt20::C0(h5649, _, _, _, _, _, _, _) => {
                    h5649
                }
            };
            nat_mul(a5650, a5651)?
        };
        let a5656 = {
            let a5652 = v0;
            let a5653 = 3u64;
            let a5654 = 0u64;
            nat_quot(a5652, a5653, a5654)
        };
        nat_add(a5655, a5656)?
    };
    let v7: u64 = f270(v6, v3)?;
    let v8: u64 = f270(v6, v2)?;
    let v9: u64 = f270(v6, v4)?;
    let a5657 = v5;
    let a5658 = 0u64;
    if nat_eq(a5657, a5658) {
        Ok(f271(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5659 = v5;
        let a5660 = 1u64;
        if nat_eq(a5659, a5660) {
            Ok(f271(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f271(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f277(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5661, _, _, _, _, _, _, _) => {
            h5661
        }
    };
    let v5: u64 = {
        let a5664 = 1u64;
        let a5665 = {
            let a5662 = v4;
            let a5663 = f265(v3);
            nat_mul(a5662, a5663)?
        };
        nat_add(a5664, a5665)?
    };
    let a5666 = v0;
    let a5667 = 0u64;
    if nat_eq(a5666, a5667) {
        Ok(f269(0u64))
    } else {
        let a5668 = v0;
        let a5669 = v5;
        if nat_lt(a5668, a5669) {
            let v6: u64 = {
                let a5670 = v0;
                let a5671 = 1u64;
                nat_sub(a5670, a5671)
            };
            let v7: u64 = f270({
                let a5677 = {
                    let a5672 = v1;
                    let a5673 = v4;
                    nat_mul(a5672, a5673)?
                };
                let a5678 = {
                    let a5674 = v6;
                    let a5675 = f265(v3);
                    let a5676 = 0u64;
                    nat_quot(a5674, a5675, a5676)
                };
                nat_add(a5677, a5678)?
            }, v2)?;
            if v3 {
                let a5682 = {
                    let a5679 = v6;
                    let a5680 = 2u64;
                    let a5681 = 0u64;
                    nat_rem(a5679, a5680, a5681)
                };
                let a5683 = 0u64;
                if nat_eq(a5682, a5683) {
                    Ok(f271(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f271(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f271(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5684 = v0;
                let a5685 = v5;
                nat_sub(a5684, a5685)
            };
            let a5686 = v8;
            let a5687 = 0u64;
            if nat_eq(a5686, a5687) {
                Ok(f271(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5688 = v8;
                    let a5689 = 1u64;
                    nat_eq(a5688, a5689)
                } else {
                    false
                } {
                    Ok(f271(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5690 = v8;
                    let a5691 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5690, a5691) {
                        Ok(f271(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5692 = v8;
                        let a5693 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5692, a5693) {
                            Ok(f269(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f278(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5695 = 13u64;
        let a5696 = f266(match v2.clone() {
            Adt20::C0(h5694, _, _, _, _, _, _, _) => {
                h5694
            }
        });
        nat_mul(a5695, a5696)?
    } else {
        0u64
    };
    let a5697 = v0;
    let a5698 = v5;
    if nat_lt(a5697, a5698) {
        f284({
            let a5699 = v0;
            let a5700 = 13u64;
            let a5701 = 0u64;
            nat_rem(a5699, a5700, a5701)
        }, v1, {
            let a5702 = v0;
            let a5703 = 13u64;
            let a5704 = 0u64;
            nat_quot(a5702, a5703, a5704)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5705 = v0;
            let a5706 = v5;
            nat_sub(a5705, a5706)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f285({
            let a5707 = v6;
            let a5708 = v7;
            let a5709 = 0u64;
            nat_rem(a5707, a5708, a5709)
        }, {
            let a5719 = {
                let a5714 = {
                    let a5711 = v1;
                    let a5712 = match v2.clone() {
                        Adt20::C0(h5710, _, _, _, _, _, _, _) => {
                            h5710
                        }
                    };
                    nat_mul(a5711, a5712)?
                };
                let a5715 = if v4 {
                    f286(match v2.clone() {
                        Adt20::C0(h5713, _, _, _, _, _, _, _) => {
                            h5713
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5714, a5715)?
            };
            let a5720 = {
                let a5716 = v6;
                let a5717 = v7;
                let a5718 = 0u64;
                nat_quot(a5716, a5717, a5718)
            };
            nat_add(a5719, a5720)?
        }, v2, v3, v4)
    }
}

pub fn f279(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5721 = v0;
    let a5722 = 0u64;
    if nat_eq(a5721, a5722) {
        f271(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5723 = v0;
        let a5724 = 1u64;
        if nat_eq(a5723, a5724) {
            f271(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5725 = v0;
                let a5726 = 2u64;
                nat_eq(a5725, a5726)
            } else {
                false
            } {
                f271(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5727 = v0;
                let a5728 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5727, a5728) {
                    f271(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5729 = v0;
                    let a5730 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5729, a5730) {
                        f271(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5731 = v0;
                        let a5732 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5731, a5732) {
                            f271(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f280(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5736 = 1u64;
        let a5737 = {
            let a5734 = match v4.clone() {
                Adt20::C0(h5733, _, _, _, _, _, _, _) => {
                    h5733
                }
            };
            let a5735 = v6;
            nat_mul(a5734, a5735)?
        };
        nat_add(a5736, a5737)?
    };
    let a5738 = v1;
    let a5739 = 0u64;
    if nat_eq(a5738, a5739) {
        Ok(f269(1u64))
    } else {
        let a5740 = v1;
        let a5741 = v7;
        if nat_lt(a5740, a5741) {
            let v8: u64 = {
                let a5742 = v1;
                let a5743 = 1u64;
                nat_sub(a5742, a5743)
            };
            let v9: u64 = {
                let a5750 = {
                    let a5745 = f283(v2, v3, v4.clone())?;
                    let a5746 = match v4.clone() {
                        Adt20::C0(h5744, _, _, _, _, _, _, _) => {
                            h5744
                        }
                    };
                    nat_mul(a5745, a5746)?
                };
                let a5751 = {
                    let a5747 = v8;
                    let a5748 = v6;
                    let a5749 = 0u64;
                    nat_quot(a5747, a5748, a5749)
                };
                nat_add(a5750, a5751)?
            };
            let v10: u64 = f270(v9, v4)?;
            let v11: u64 = f270(v9, v5)?;
            let v12: u64 = {
                let a5752 = v8;
                let a5753 = v6;
                let a5754 = 0u64;
                nat_rem(a5752, a5753, a5754)
            };
            if v0 {
                let a5755 = v12;
                let a5756 = 0u64;
                if nat_eq(a5755, a5756) {
                    Ok(f271(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5757 = v12;
                    let a5758 = 1u64;
                    if nat_eq(a5757, a5758) {
                        Ok(f271(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5759 = v12;
                        let a5760 = 2u64;
                        if nat_eq(a5759, a5760) {
                            Ok(f271(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f271(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5761 = v12;
                let a5762 = 0u64;
                if nat_eq(a5761, a5762) {
                    Ok(f271(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f271(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5763 = v1;
            let a5764 = v7;
            if nat_eq(a5763, a5764) {
                Ok(f271(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f281(v0: u64, v1: u64) -> Adt19 {
    let a5765 = v0;
    let a5766 = 0u64;
    if nat_eq(a5765, a5766) {
        f271(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5767 = v0;
        let a5768 = 1u64;
        if nat_eq(a5767, a5768) {
            f271(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5769 = v0;
            let a5770 = 2u64;
            if nat_eq(a5769, a5770) {
                f271(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5771 = v0;
                let a5772 = 3u64;
                if nat_eq(a5771, a5772) {
                    f269(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f282(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5773 = v0;
    let a5774 = 0u64;
    if nat_eq(a5773, a5774) {
        f271(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5775 = v0;
        let a5776 = 1u64;
        if nat_eq(a5775, a5776) {
            f271(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5777 = v0;
            let a5778 = 2u64;
            if nat_eq(a5777, a5778) {
                f271(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5779 = v0;
                let a5780 = 3u64;
                if nat_eq(a5779, a5780) {
                    f271(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5781 = v0;
                    let a5782 = 4u64;
                    if nat_eq(a5781, a5782) {
                        f271(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5783 = v0;
                        let a5784 = 5u64;
                        if nat_eq(a5783, a5784) {
                            f271(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f283(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5795 = {
        let a5793 = {
            let a5790 = {
                let a5787 = match v1.clone() {
                    Adt22::C0(_, _, h5785, _) => {
                        h5785
                    }
                };
                let a5788 = match v2.clone() {
                    Adt20::C0(_, _, h5786, _, _, _, _, _) => {
                        h5786
                    }
                };
                nat_mul(a5787, a5788)?
            };
            let a5791 = match v1 {
                Adt22::C0(h5789, _, _, _) => {
                    h5789
                }
            };
            nat_add(a5790, a5791)?
        };
        let a5794 = match v2 {
            Adt20::C0(_, h5792, _, _, _, _, _, _) => {
                h5792
            }
        };
        nat_mul(a5793, a5794)?
    };
    let a5796 = v0;
    nat_add(a5795, a5796)
}

pub fn f284(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5804 = {
            let a5802 = {
                let a5798 = v1;
                let a5799 = match v3.clone() {
                    Adt20::C0(h5797, _, _, _, _, _, _, _) => {
                        h5797
                    }
                };
                nat_mul(a5798, a5799)?
            };
            let a5803 = {
                let a5800 = v2;
                let a5801 = 4u64;
                nat_mul(a5800, a5801)?
            };
            nat_add(a5802, a5803)?
        };
        let a5805 = f287(v0);
        nat_add(a5804, a5805)?
    };
    let v6: u64 = f270(v5, v3)?;
    let v7: u64 = f270(v5, v4)?;
    let a5806 = v0;
    let a5807 = 0u64;
    if if if if nat_eq(a5806, a5807) {
        true
    } else {
        let a5808 = v0;
        let a5809 = 2u64;
        nat_eq(a5808, a5809)
    } {
        true
    } else {
        let a5810 = v0;
        let a5811 = 5u64;
        nat_eq(a5810, a5811)
    } {
        true
    } else {
        let a5812 = v0;
        let a5813 = 7u64;
        nat_eq(a5812, a5813)
    } {
        Ok(f271(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5814 = v0;
        let a5815 = 1u64;
        if if if if nat_eq(a5814, a5815) {
            true
        } else {
            let a5816 = v0;
            let a5817 = 3u64;
            nat_eq(a5816, a5817)
        } {
            true
        } else {
            let a5818 = v0;
            let a5819 = 6u64;
            nat_eq(a5818, a5819)
        } {
            true
        } else {
            let a5820 = v0;
            let a5821 = 8u64;
            nat_eq(a5820, a5821)
        } {
            Ok(f271(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5822 = v0;
                let a5823 = 1u64;
                if nat_eq(a5822, a5823) {
                    4u64
                } else {
                    let a5824 = v0;
                    let a5825 = 6u64;
                    if nat_eq(a5824, a5825) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5826 = v0;
            let a5827 = 4u64;
            if nat_eq(a5826, a5827) {
                Ok(f271(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5828 = v0;
                let a5829 = 9u64;
                if nat_eq(a5828, a5829) {
                    Ok(f271(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5830 = v0;
                    let a5831 = 10u64;
                    if nat_eq(a5830, a5831) {
                        Ok(f271(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5832 = v0;
                        let a5833 = 11u64;
                        if nat_eq(a5832, a5833) {
                            Ok(f271(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5834 = v0;
                            let a5835 = 12u64;
                            if nat_eq(a5834, a5835) {
                                Ok(f271(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                            } else {
                                Ok(Adt19::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f285(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f270(v1, v2)?;
    let v6: u64 = f270(v1, v3)?;
    let a5836 = v0;
    let a5837 = 0u64;
    if nat_eq(a5836, a5837) {
        Ok(f271(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5838 = v0;
        let a5839 = 1u64;
        if nat_eq(a5838, a5839) {
            Ok(f271(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5840 = v0;
                let a5841 = 2u64;
                if nat_eq(a5840, a5841) {
                    Ok(f271(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5842 = v0;
                    let a5843 = 3u64;
                    if nat_eq(a5842, a5843) {
                        Ok(f271(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5844 = v0;
                let a5845 = 2u64;
                if nat_eq(a5844, a5845) {
                    Ok(f271(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f286(v0: u64) -> R<u64> {
    let a5846 = 4u64;
    let a5847 = f266(v0);
    nat_mul(a5846, a5847)
}

pub fn f287(v0: u64) -> u64 {
    let a5848 = v0;
    let a5849 = 5u64;
    if nat_lt(a5848, a5849) {
        let a5850 = v0;
        let a5851 = 2u64;
        if nat_lt(a5850, a5851) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5852 = v0;
        let a5853 = 7u64;
        if nat_lt(a5852, a5853) {
            2u64
        } else {
            3u64
        }
    }
}
