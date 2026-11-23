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
        let a465 = 52u64;
        if nat_eq(a464, a465) {
            f41(v1, v2, v4)?
        } else {
            let a466 = v0;
            let a467 = 73u64;
            if nat_eq(a466, a467) {
                f42(v2, v3, v4)?
            } else {
                let a468 = v0;
                let a469 = 57u64;
                if if nat_eq(a468, a469) {
                    true
                } else {
                    let a470 = v0;
                    let a471 = 58u64;
                    nat_eq(a470, a471)
                } {
                    let m472 = v3;
                    match m472 {
                        None => {
                            f43({
                                let a473 = v0;
                                let a474 = 58u64;
                                nat_eq(a473, a474)
                            }, v1, v2, v4)?
                        }
                        Some(_) => {
                            Adt10::C1
                        }
                    }
                } else {
                    let a475 = v0;
                    let a476 = 49u64;
                    if nat_eq(a475, a476) {
                        f44(v1, v2, v3, v4)?
                    } else {
                        let a477 = v0;
                        let a478 = 51u64;
                        if nat_eq(a477, a478) {
                            f45(v1, v2, v3, v4)?
                        } else {
                            let a479 = v0;
                            let a480 = 39u64;
                            if nat_eq(a479, a480) {
                                f46(v2, v3, v4)?
                            } else {
                                let a481 = v0;
                                let a482 = 77u64;
                                if nat_eq(a481, a482) {
                                    f47(v2, v4)?
                                } else {
                                    let a483 = v0;
                                    let a484 = 40u64;
                                    if nat_eq(a483, a484) {
                                        f48(v2, v3, v4)?
                                    } else {
                                        if f49(v0) {
                                            let a485 = v0;
                                            let a486 = 33u64;
                                            if if if nat_eq(a485, a486) {
                                                let a488 = match v4.clone() {
                                                    Adt2::C0(h487, _, _, _, _, _, _, _, _, _, _) => {
                                                        h487
                                                    }
                                                };
                                                let a489 = 0u64;
                                                nat_eq(a488, a489)
                                            } else {
                                                false
                                            } {
                                                f50(match v2.clone() {
                                                    Adt2::C0(h490, _, _, _, _, _, _, _, _, _, _) => {
                                                        h490
                                                    }
                                                })
                                            } else {
                                                false
                                            } {
                                                f51(v2, v3, v4)?
                                            } else {
                                                f52(v0, v2, {
                                                    let m491 = v3;
                                                    match m491 {
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
                                            let a492 = v0;
                                            let a493 = 19u64;
                                            if nat_eq(a492, a493) {
                                                f53(v2, v4)?
                                            } else {
                                                let a494 = v0;
                                                let a495 = 21u64;
                                                if nat_eq(a494, a495) {
                                                    f54(v5, v2, v3, v4)?
                                                } else {
                                                    let a496 = v0;
                                                    let a497 = 61u64;
                                                    if if if nat_eq(a496, a497) {
                                                        true
                                                    } else {
                                                        let a498 = v0;
                                                        let a499 = 62u64;
                                                        nat_eq(a498, a499)
                                                    } {
                                                        true
                                                    } else {
                                                        let a500 = v0;
                                                        let a501 = 63u64;
                                                        nat_eq(a500, a501)
                                                    } {
                                                        f55(v0, v1, v2, v4)?
                                                    } else {
                                                        let a502 = v0;
                                                        let a503 = 4u64;
                                                        if if nat_eq(a502, a503) {
                                                            true
                                                        } else {
                                                            let a504 = v0;
                                                            let a505 = 32u64;
                                                            nat_eq(a504, a505)
                                                        } {
                                                            f56(v0, v1, v2, v3, v4)?
                                                        } else {
                                                            let a506 = v0;
                                                            let a507 = 20u64;
                                                            if nat_eq(a506, a507) {
                                                                f57(v2, v4)?
                                                            } else {
                                                                let a508 = v0;
                                                                let a509 = 42u64;
                                                                if nat_eq(a508, a509) {
                                                                    f58(4u64, v2, v4)?
                                                                } else {
                                                                    let a510 = v0;
                                                                    let a511 = 69u64;
                                                                    if nat_eq(a510, a511) {
                                                                        let a512 = v5;
                                                                        let a513 = 4u64;
                                                                        if nat_lt(a512, a513) {
                                                                            f58(v5, v2, v4)?
                                                                        } else {
                                                                            Adt10::C1
                                                                        }
                                                                    } else {
                                                                        let a514 = v0;
                                                                        let a515 = 70u64;
                                                                        if nat_eq(a514, a515) {
                                                                            f59(v5, v2, v4)?
                                                                        } else {
                                                                            let a516 = v0;
                                                                            let a517 = 75u64;
                                                                            if if nat_eq(a516, a517) {
                                                                                true
                                                                            } else {
                                                                                let a518 = v0;
                                                                                let a519 = 76u64;
                                                                                nat_eq(a518, a519)
                                                                            } {
                                                                                f60({
                                                                                    let a520 = v0;
                                                                                    let a521 = 76u64;
                                                                                    nat_eq(a520, a521)
                                                                                }, v1, v2, v4)?
                                                                            } else {
                                                                                let a522 = v0;
                                                                                let a523 = 31u64;
                                                                                if if if nat_eq(a522, a523) {
                                                                                    true
                                                                                } else {
                                                                                    let a524 = v0;
                                                                                    let a525 = 47u64;
                                                                                    nat_eq(a524, a525)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a526 = v0;
                                                                                    let a527 = 68u64;
                                                                                    nat_eq(a526, a527)
                                                                                } {
                                                                                    f61(v0, v1, v2, v4)?
                                                                                } else {
                                                                                    let a528 = v0;
                                                                                    let a529 = 15u64;
                                                                                    if nat_eq(a528, a529) {
                                                                                        f62(v2, v4)?
                                                                                    } else {
                                                                                        let a530 = v0;
                                                                                        let a531 = 17u64;
                                                                                        if nat_eq(a530, a531) {
                                                                                            f63(v2, v4)?
                                                                                        } else {
                                                                                            let a532 = v0;
                                                                                            let a533 = 18u64;
                                                                                            if nat_eq(a532, a533) {
                                                                                                f64(v2, v3, v4)?
                                                                                            } else {
                                                                                                let a534 = v0;
                                                                                                let a535 = 13u64;
                                                                                                if if if nat_eq(a534, a535) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a536 = v0;
                                                                                                    let a537 = 14u64;
                                                                                                    nat_eq(a536, a537)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a538 = v0;
                                                                                                    let a539 = 16u64;
                                                                                                    nat_eq(a538, a539)
                                                                                                } {
                                                                                                    f65(v0, v2, v4)?
                                                                                                } else {
                                                                                                    let m540 = f66(v0, v5)?;
                                                                                                    match m540 {
                                                                                                        None => {
                                                                                                            f67(v0, v5, v2, v3, v4)?
                                                                                                        }
                                                                                                        Some(v8) => {
                                                                                                            f68(v8, v2, v4)?
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
        Adt8::C0(_, h541, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h541
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h542, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h542
        }
    };
    let v6: u64 = {
        let a543 = v3;
        let a544 = v5;
        let a545 = 0u64;
        nat_rem(a543, a544, a545)
    };
    let a546 = v1;
    let a547 = 0u64;
    if nat_eq(a546, a547) {
        let a549 = v3;
        let a550 = match v0.clone() {
            Adt8::C0(_, _, _, h548, _, _, _, _, _, _, _, _, _, _, _) => {
                h548
            }
        };
        if nat_lt(a549, a550) {
            Ok(Adt9::C8(Some(f69(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h551, _, _, _, _, _, _) => {
                    h551
                }
            }, {
                let a555 = v6;
                let a556 = {
                    let m554 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h553, _, _, _, _, _, _) => {
                            h553
                        }
                    };
                    match m554 {
                        Adt3::C0(h552, _, _, _, _, _, _, _) => {
                            h552
                        }
                    }
                };
                let a557 = 0u64;
                nat_rem(a555, a556, a557)
            }, {
                let a567 = {
                    let a561 = v6;
                    let a562 = {
                        let m560 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h559, _, _, _, _, _, _) => {
                                h559
                            }
                        };
                        match m560 {
                            Adt3::C0(h558, _, _, _, _, _, _, _) => {
                                h558
                            }
                        }
                    };
                    let a563 = 0u64;
                    nat_quot(a561, a562, a563)
                };
                let a568 = {
                    let m566 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h565, _, _, _, _, _, _) => {
                            h565
                        }
                    };
                    match m566 {
                        Adt3::C0(_, h564, _, _, _, _, _, _) => {
                            h564
                        }
                    }
                };
                let a569 = 0u64;
                nat_rem(a567, a568, a569)
            }, {
                let a579 = {
                    let a573 = v6;
                    let a574 = {
                        let m572 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h571, _, _, _, _, _, _) => {
                                h571
                            }
                        };
                        match m572 {
                            Adt3::C0(h570, _, _, _, _, _, _, _) => {
                                h570
                            }
                        }
                    };
                    let a575 = 0u64;
                    nat_quot(a573, a574, a575)
                };
                let a580 = {
                    let m578 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h577, _, _, _, _, _, _) => {
                            h577
                        }
                    };
                    match m578 {
                        Adt3::C0(_, h576, _, _, _, _, _, _) => {
                            h576
                        }
                    }
                };
                let a581 = 0u64;
                nat_quot(a579, a580, a581)
            }, {
                let a582 = v3;
                let a583 = v5;
                let a584 = 0u64;
                nat_quot(a582, a583, a584)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a590 = v2;
            let a591 = {
                let a587 = v3;
                let a588 = {
                    let a585 = v1;
                    let a586 = 1u64;
                    if nat_eq(a585, a586) {
                        v4
                    } else {
                        v5
                    }
                };
                let a589 = 0u64;
                nat_quot(a587, a588, a589)
            };
            nat_add(a590, a591)?
        };
        let a593 = v7;
        let a594 = match v0.clone() {
            Adt8::C0(h592, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h592
            }
        };
        if nat_lt(a593, a594) {
            let v8: u64 = {
                let a598 = v7;
                let a599 = {
                    let m597 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h596, _, _, _, _, _) => {
                            h596
                        }
                    };
                    match m597 {
                        Adt3::C0(h595, _, _, _, _, _, _, _) => {
                            h595
                        }
                    }
                };
                let a600 = 0u64;
                nat_rem(a598, a599, a600)
            };
            let v9: u64 = {
                let a604 = v7;
                let a605 = {
                    let m603 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h602, _, _, _, _, _) => {
                            h602
                        }
                    };
                    match m603 {
                        Adt3::C0(h601, _, _, _, _, _, _, _) => {
                            h601
                        }
                    }
                };
                let a606 = 0u64;
                nat_quot(a604, a605, a606)
            };
            let v10: u64 = {
                let a610 = v9;
                let a611 = {
                    let m609 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h608, _, _, _, _, _) => {
                            h608
                        }
                    };
                    match m609 {
                        Adt3::C0(_, h607, _, _, _, _, _, _) => {
                            h607
                        }
                    }
                };
                let a612 = 0u64;
                nat_rem(a610, a611, a612)
            };
            let v11: u64 = {
                let a616 = v9;
                let a617 = {
                    let m615 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h614, _, _, _, _, _) => {
                            h614
                        }
                    };
                    match m615 {
                        Adt3::C0(_, h613, _, _, _, _, _, _) => {
                            h613
                        }
                    }
                };
                let a618 = 0u64;
                nat_quot(a616, a617, a618)
            };
            let a619 = v1;
            let a620 = 1u64;
            if nat_eq(a619, a620) {
                Ok(Adt9::C8(Some(f69(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h621, _, _, _, _, _) => {
                        h621
                    }
                }, v8, v10, {
                    let a622 = v3;
                    let a623 = v4;
                    let a624 = 0u64;
                    nat_rem(a622, a623, a624)
                }, v11)?)))
            } else {
                let a625 = v1;
                let a626 = 2u64;
                if nat_eq(a625, a626) {
                    Ok(Adt9::C8(Some({
                        let a672 = f69(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h627, _, _, _, _) => {
                                h627
                            }
                        }, {
                            let a639 = {
                                let a631 = v8;
                                let a632 = {
                                    let m630 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h629, _, _, _) => {
                                            h629
                                        }
                                    };
                                    match m630 {
                                        Adt6::C0(_, h628, _, _) => {
                                            h628
                                        }
                                    }
                                };
                                nat_mul(a631, a632)?
                            };
                            let a640 = {
                                let a636 = v6;
                                let a637 = {
                                    let m635 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h634, _, _, _, _, _, _) => {
                                            h634
                                        }
                                    };
                                    match m635 {
                                        Adt3::C0(h633, _, _, _, _, _, _, _) => {
                                            h633
                                        }
                                    }
                                };
                                let a638 = 0u64;
                                nat_rem(a636, a637, a638)
                            };
                            nat_add(a639, a640)?
                        }, {
                            let a658 = {
                                let a644 = v10;
                                let a645 = {
                                    let m643 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h642, _, _, _) => {
                                            h642
                                        }
                                    };
                                    match m643 {
                                        Adt6::C0(_, h641, _, _) => {
                                            h641
                                        }
                                    }
                                };
                                nat_mul(a644, a645)?
                            };
                            let a659 = {
                                let a655 = {
                                    let a649 = v6;
                                    let a650 = {
                                        let m648 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h647, _, _, _, _, _, _) => {
                                                h647
                                            }
                                        };
                                        match m648 {
                                            Adt3::C0(h646, _, _, _, _, _, _, _) => {
                                                h646
                                            }
                                        }
                                    };
                                    let a651 = 0u64;
                                    nat_quot(a649, a650, a651)
                                };
                                let a656 = {
                                    let m654 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h653, _, _, _, _, _, _) => {
                                            h653
                                        }
                                    };
                                    match m654 {
                                        Adt3::C0(_, h652, _, _, _, _, _, _) => {
                                            h652
                                        }
                                    }
                                };
                                let a657 = 0u64;
                                nat_rem(a655, a656, a657)
                            };
                            nat_add(a658, a659)?
                        }, {
                            let a669 = {
                                let a663 = v6;
                                let a664 = {
                                    let m662 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h661, _, _, _, _, _, _) => {
                                            h661
                                        }
                                    };
                                    match m662 {
                                        Adt3::C0(h660, _, _, _, _, _, _, _) => {
                                            h660
                                        }
                                    }
                                };
                                let a665 = 0u64;
                                nat_quot(a663, a664, a665)
                            };
                            let a670 = {
                                let m668 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h667, _, _, _, _, _, _) => {
                                        h667
                                    }
                                };
                                match m668 {
                                    Adt3::C0(_, h666, _, _, _, _, _, _) => {
                                        h666
                                    }
                                }
                            };
                            let a671 = 0u64;
                            nat_quot(a669, a670, a671)
                        }, v11)?;
                        let a673 = 4u64;
                        let a674 = 0u64;
                        nat_quot(a672, a673, a674)
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
    let a675 = v1;
    let a676 = 0u64;
    if nat_eq(a675, a676) {
        let a678 = v3;
        let a679 = match v0.clone() {
            Adt8::C0(_, _, _, h677, _, _, _, _, _, _, _, _, _, _, _) => {
                h677
            }
        };
        if nat_lt(a678, a679) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h680, _, _, _, _, _, _) => {
                    h680
                }
            };
            let v5: u64 = {
                let a682 = v3;
                let a683 = match v0.clone() {
                    Adt8::C0(_, _, h681, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h681
                    }
                };
                let a684 = 0u64;
                nat_quot(a682, a683, a684)
            };
            Ok(Adt9::C8(Some(f69(v4.clone(), f70(v5, v4.clone(), 0u64), f70(v5, v4.clone(), 1u64), f70(v5, v4, 2u64), f71(v0.clone(), v5, {
                let a686 = v3;
                let a687 = match v0 {
                    Adt8::C0(_, _, h685, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h685
                    }
                };
                let a688 = 0u64;
                nat_rem(a686, a687, a688)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h689, _, _, _, _) => {
                h689
            }
        };
        let v7: u64 = {
            let a690 = v1;
            let a691 = 1u64;
            if nat_eq(a690, a691) {
                match v0.clone() {
                    Adt8::C0(_, h692, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h692
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h693, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h693
                    }
                }
            }
        };
        let v8: u64 = {
            let a697 = v2;
            let a698 = {
                let a694 = v3;
                let a695 = v7;
                let a696 = 0u64;
                nat_quot(a694, a695, a696)
            };
            nat_add(a697, a698)?
        };
        let v9: u64 = {
            let a699 = v3;
            let a700 = v7;
            let a701 = 0u64;
            nat_rem(a699, a700, a701)
        };
        let v10: u64 = {
            let a703 = v8;
            let a704 = match v6.clone() {
                Adt3::C0(h702, _, _, _, _, _, _, _) => {
                    h702
                }
            };
            let a705 = 0u64;
            nat_rem(a703, a704, a705)
        };
        let v11: u64 = {
            let a707 = v8;
            let a708 = match v6.clone() {
                Adt3::C0(h706, _, _, _, _, _, _, _) => {
                    h706
                }
            };
            let a709 = 0u64;
            nat_quot(a707, a708, a709)
        };
        let v12: u64 = {
            let a711 = v11;
            let a712 = match v6.clone() {
                Adt3::C0(_, h710, _, _, _, _, _, _) => {
                    h710
                }
            };
            let a713 = 0u64;
            nat_rem(a711, a712, a713)
        };
        let v13: u64 = {
            let a715 = v11;
            let a716 = match v6.clone() {
                Adt3::C0(_, h714, _, _, _, _, _, _) => {
                    h714
                }
            };
            let a717 = 0u64;
            nat_quot(a715, a716, a717)
        };
        let a719 = v8;
        let a720 = match v0.clone() {
            Adt8::C0(h718, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h718
            }
        };
        if nat_lt(a719, a720) {
            let a721 = v1;
            let a722 = 1u64;
            if nat_eq(a721, a722) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h723, _, _, _, _, _, _) => {
                        h723
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h724, _, _, _, _, _) => {
                        h724
                    }
                };
                let m727 = f72(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h725, _, _, _) => {
                        h725
                    }
                }, match v15.clone() {
                    Adt3::C0(h726, _, _, _, _, _, _, _) => {
                        h726
                    }
                }, v10, f70(v9, v14.clone(), 0u64))?;
                match m727 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m730 = f72(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h728, _, _) => {
                                h728
                            }
                        }, match v15 {
                            Adt3::C0(_, h729, _, _, _, _, _, _) => {
                                h729
                            }
                        }, v12, f70(v9, v14, 1u64))?;
                        match m730 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f73(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a731 = v1;
                let a732 = 2u64;
                if nat_eq(a731, a732) {
                    Ok(Adt9::C8(Some({
                        let a734 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h733) => {
                                h733
                            }
                        };
                        let a735 = 0u64;
                        if nat_eq(a734, a735) {
                            f69(v6, v10, v12, v9, v13)?
                        } else {
                            f69(v6.clone(), v10, v12, {
                                let a737 = v13;
                                let a738 = match v6.clone() {
                                    Adt3::C0(_, _, h736, _, _, _, _, _) => {
                                        h736
                                    }
                                };
                                let a739 = 0u64;
                                nat_rem(a737, a738, a739)
                            }, {
                                let a747 = {
                                    let a745 = {
                                        let a741 = v13;
                                        let a742 = match v6 {
                                            Adt3::C0(_, _, h740, _, _, _, _, _) => {
                                                h740
                                            }
                                        };
                                        let a743 = 0u64;
                                        nat_quot(a741, a742, a743)
                                    };
                                    let a746 = match v0 {
                                        Adt8::C0(_, _, h744, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h744
                                        }
                                    };
                                    nat_mul(a745, a746)?
                                };
                                let a748 = v9;
                                nat_add(a747, a748)?
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
    Ok(f74(v3.clone(), {
        let m749 = v3.clone();
        match m749 {
            Adt4::C0 => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f77(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C19(v51, v52) => {
                f78(v51, v0, v1, v2, v52, v4)?
            }
            Adt4::C20(v53, v54) => {
                f79(v53, v0, v1, v2, v54, v4)?
            }
            Adt4::C21 => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C22(v55, v56) => {
                f80(v0, v1, v2, v55, v56, v4)?
            }
            Adt4::C23(v70, v71, v72, v73) => {
                f87(v0, v73, v70, v71, v72, v4)?
            }
            Adt4::C24(v69) => {
                if v5 {
                    f85(Adt16::C0(v69), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f75(v0, v1, v2, v3)?
            }
            Adt4::C26(v65, v66, v67, v68) => {
                let a750 = v65;
                let a751 = 64u64;
                if if if nat_eq(a750, a751) {
                    true
                } else {
                    let a752 = v65;
                    let a753 = 65u64;
                    nat_eq(a752, a753)
                } {
                    true
                } else {
                    let a754 = v65;
                    let a755 = 66u64;
                    nat_eq(a754, a755)
                } {
                    if v5 {
                        f85(Adt16::C1(v65, v66, v67, v68), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a756 = v65;
                    let a757 = 0u64;
                    if if nat_eq(a756, a757) {
                        true
                    } else {
                        let a758 = v65;
                        let a759 = 30u64;
                        nat_eq(a758, a759)
                    } {
                        f86(v0, v1, v2, v65, v66, v67, v68, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v64) => {
                f84(v0, v1, v2, v64)?
            }
            Adt4::C28(v60) => {
                f81(v0, v1, v2, v60, v4)?
            }
            Adt4::C29(v61) => {
                f82(v0, v1, v2, v61)?
            }
            Adt4::C30(v62, _) => {
                f83(v0, v1, v2, v62)?
            }
            Adt4::C31(v33, v34) => {
                f76(v0, v1, v2, v33, v34)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m760 = v0;
    match m760 {
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
        Adt4::C25(v47, _, v49) => {
            Ok(Adt9::C6({
                let a765 = {
                    let a761 = 0u64;
                    let a762 = v47;
                    if nat_lt(a761, a762) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a766 = {
                    let a763 = 0u64;
                    let a764 = v49;
                    if nat_lt(a763, a764) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a765, a766)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v55, _, _, _) => {
            Ok(Adt9::C6({
                let a767 = v55;
                let a768 = 66u64;
                if nat_eq(a767, a768) {
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
    }
}

pub fn f15(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a769 = v1;
    let a770 = 5u64;
    if if if if if nat_lt(a769, a770) {
        true
    } else {
        f88(v0, v1, v2, v3, v4)
    } {
        let a771 = v5;
        let a772 = 5u64;
        nat_lt(a771, a772)
    } else {
        false
    } {
        let a773 = v7;
        let a774 = 5u64;
        nat_lt(a773, a774)
    } else {
        false
    } {
        let a775 = v9;
        let a776 = 2u64;
        if nat_lt(a775, a776) {
            true
        } else {
            let a777 = v9;
            let a778 = 3u64;
            nat_eq(a777, a778)
        }
    } else {
        false
    } {
        Adt9::C7(f89(v0, v1, v2, v3, v4), f90(v5, v6), f90(v7, v8), f91({
            let a779 = v9;
            let a780 = 0u64;
            if nat_eq(a779, a780) {
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
    let a792 = {
        let a786 = {
            let a782 = match v0.clone() {
                Adt2::C0(_, h781, _, _, _, _, _, _, _, _, _) => {
                    h781
                }
            };
            let a783 = v1;
            let a784 = 0u64;
            nat_quot(a782, a783, a784)
        };
        let a787 = match v0.clone() {
            Adt2::C0(_, _, h785, _, _, _, _, _, _, _, _) => {
                h785
            }
        };
        nat_mul(a786, a787)?
    };
    let a793 = {
        let a790 = match v0.clone() {
            Adt2::C0(_, _, _, h788, _, _, _, _, _, _, _) => {
                h788
            }
        };
        let a791 = match v0 {
            Adt2::C0(_, _, _, _, h789, _, _, _, _, _, _) => {
                h789
            }
        };
        nat_mul(a790, a791)?
    };
    nat_mul(a792, a793)
}

pub fn f17(v0: u64) -> bool {
    let a794 = v0;
    let a795 = 4294967296u64;
    if nat_lt(a794, a795) {
        let a799 = {
            let a796 = v0;
            let a797 = 2147483648u64;
            let a798 = 0u64;
            nat_rem(a796, a797, a798)
        };
        let a800 = 2139095040u64;
        nat_lt(a799, a800)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a801 = 4u64;
    let a802 = v0;
    nat_mul(a801, a802)
}

pub fn f19(v0: u64) -> R<u64> {
    let a803 = v0;
    let a804 = 0u64;
    if nat_eq(a803, a804) {
        Ok(0u64)
    } else {
        let a807 = 4u64;
        let a808 = {
            let a805 = 3u64;
            let a806 = v0;
            nat_mul(a805, a806)?
        };
        nat_add(a807, a808)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a810 = match v0.clone() {
            Adt2::C0(_, h809, _, _, _, _, _, _, _, _, _) => {
                h809
            }
        };
        let a811 = v1;
        let a812 = 0u64;
        nat_quot(a810, a811, a812)
    }, match v0.clone() {
        Adt2::C0(_, _, h813, _, _, _, _, _, _, _, _) => {
            h813
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h814, _, _, _, _, _, _, _) => {
            h814
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h815, _, _, _, _, _, _) => {
            h815
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h816, _, _, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h817, _, _, _, _) => {
            h817
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h818, _, _, _) => {
            h818
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h819, _, _) => {
            h819
        }
    })
}

pub fn f21(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f20(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h820, _, _, _, _, _, _, _, _, _) => {
                h820
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h821, _, _, _, _, _, _, _, _) => {
                h821
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h822, _, _, _, _, _, _, _) => {
                h822
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h823, _, _, _, _, _) => {
                h823
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h824, _, _, _, _) => {
                h824
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h825, _, _, _) => {
                h825
            }
        })
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a827 = match v0.clone() {
        Adt2::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
            h826
        }
    };
    let a828 = 1u64;
    if if if if nat_eq(a827, a828) {
        true
    } else {
        let a830 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h829, _, _, _, _, _) => {
                h829
            }
        };
        let a831 = 4u64;
        nat_eq(a830, a831)
    } {
        let a833 = match v0.clone() {
            Adt2::C0(_, _, h832, _, _, _, _, _, _, _, _) => {
                h832
            }
        };
        let a834 = 1u64;
        if nat_eq(a833, a834) {
            true
        } else {
            let a839 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h835, _, _, _, _) => {
                    h835
                }
            };
            let a840 = {
                let a837 = match v0.clone() {
                    Adt2::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                        h836
                    }
                };
                let a838 = 4u64;
                nat_mul(a837, a838)?
            };
            nat_eq(a839, a840)
        }
    } else {
        false
    } {
        let a842 = match v0.clone() {
            Adt2::C0(_, _, _, h841, _, _, _, _, _, _, _) => {
                h841
            }
        };
        let a843 = 1u64;
        if nat_eq(a842, a843) {
            true
        } else {
            let a851 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h844, _, _, _) => {
                    h844
                }
            };
            let a852 = {
                let a849 = {
                    let a847 = match v0.clone() {
                        Adt2::C0(_, h845, _, _, _, _, _, _, _, _, _) => {
                            h845
                        }
                    };
                    let a848 = match v0.clone() {
                        Adt2::C0(_, _, h846, _, _, _, _, _, _, _, _) => {
                            h846
                        }
                    };
                    nat_mul(a847, a848)?
                };
                let a850 = 4u64;
                nat_mul(a849, a850)?
            };
            nat_eq(a851, a852)
        }
    } else {
        false
    } {
        let a854 = match v0.clone() {
            Adt2::C0(_, _, _, _, h853, _, _, _, _, _, _) => {
                h853
            }
        };
        let a855 = 1u64;
        if nat_eq(a854, a855) {
            Ok(true)
        } else {
            let a866 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h856, _, _) => {
                    h856
                }
            };
            let a867 = {
                let a864 = {
                    let a862 = {
                        let a859 = match v0.clone() {
                            Adt2::C0(_, h857, _, _, _, _, _, _, _, _, _) => {
                                h857
                            }
                        };
                        let a860 = match v0.clone() {
                            Adt2::C0(_, _, h858, _, _, _, _, _, _, _, _) => {
                                h858
                            }
                        };
                        nat_mul(a859, a860)?
                    };
                    let a863 = match v0 {
                        Adt2::C0(_, _, _, h861, _, _, _, _, _, _, _) => {
                            h861
                        }
                    };
                    nat_mul(a862, a863)?
                };
                let a865 = 4u64;
                nat_mul(a864, a865)?
            };
            Ok(nat_eq(a866, a867))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<bool> {
    let a869 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h868, _) => {
            h868
        }
    };
    let a870 = 0u64;
    if nat_eq(a869, a870) {
        Ok(true)
    } else {
        let a872 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h871, _) => {
                h871
            }
        };
        let a873 = 0u64;
        if nat_eq(a872, a873) {
            Ok(true)
        } else {
            let a875 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h874) => {
                    h874
                }
            };
            let a876 = 0u64;
            if nat_eq(a875, a876) {
                Ok(true)
            } else {
                let a878 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h877) => {
                        h877
                    }
                };
                let a879 = 0u64;
                if nat_eq(a878, a879) {
                    Ok(true)
                } else {
                    let a885 = {
                        let a882 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h880) => {
                                h880
                            }
                        };
                        let a883 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h881, _) => {
                                h881
                            }
                        };
                        nat_add(a882, a883)?
                    };
                    let a886 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h884) => {
                            h884
                        }
                    };
                    if nat_le(a885, a886) {
                        Ok(true)
                    } else {
                        let a892 = {
                            let a889 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h887) => {
                                    h887
                                }
                            };
                            let a890 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h888, _) => {
                                    h888
                                }
                            };
                            nat_add(a889, a890)?
                        };
                        let a893 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h891) => {
                                h891
                            }
                        };
                        Ok(nat_le(a892, a893))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt6, v1: u64) -> R<u64> {
    let a895 = match v0.clone() {
        Adt6::C0(h894, _, _, _) => {
            h894
        }
    };
    let a896 = 0u64;
    if nat_eq(a895, a896) {
        f92(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h897, _, _) => {
                h897
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h898, _) => {
                h898
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h899) => {
                h899
            }
        }), {
            let a901 = v1;
            let a902 = match v0 {
                Adt6::C0(_, _, _, h900) => {
                    h900
                }
            };
            nat_add(a901, a902)?
        })
    } else {
        f92(v0, v1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    let a905 = match v0.clone() {
        Adt2::C0(_, h903, _, _, _, _, _, _, _, _, _) => {
            h903
        }
    };
    let a906 = match v1.clone() {
        Adt2::C0(_, h904, _, _, _, _, _, _, _, _, _) => {
            h904
        }
    };
    if if if nat_eq(a905, a906) {
        let a909 = match v0.clone() {
            Adt2::C0(_, _, h907, _, _, _, _, _, _, _, _) => {
                h907
            }
        };
        let a910 = match v1.clone() {
            Adt2::C0(_, _, h908, _, _, _, _, _, _, _, _) => {
                h908
            }
        };
        nat_eq(a909, a910)
    } else {
        false
    } {
        let a913 = match v0.clone() {
            Adt2::C0(_, _, _, h911, _, _, _, _, _, _, _) => {
                h911
            }
        };
        let a914 = match v1.clone() {
            Adt2::C0(_, _, _, h912, _, _, _, _, _, _, _) => {
                h912
            }
        };
        nat_eq(a913, a914)
    } else {
        false
    } {
        let a917 = match v0 {
            Adt2::C0(_, _, _, _, h915, _, _, _, _, _, _) => {
                h915
            }
        };
        let a918 = match v1 {
            Adt2::C0(_, _, _, _, h916, _, _, _, _, _, _) => {
                h916
            }
        };
        nat_eq(a917, a918)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a923 = {
        let a919 = v0;
        let a920 = v1;
        nat_mul(a919, a920)?
    };
    let a924 = {
        let a921 = v2;
        let a922 = f93(v3);
        nat_mul(a921, a922)?
    };
    nat_mul(a923, a924)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a927 = v0;
    let a928 = {
        let a925 = f94(v1)?;
        let a926 = v2;
        nat_mul(a925, a926)?
    };
    nat_add(a927, a928)
}

pub fn f28(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h929, _, _, _, _, _, _, _, _, _, _) => {
            h929
        }
    }, match v0.clone() {
        Adt2::C0(_, h930, _, _, _, _, _, _, _, _, _) => {
            h930
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h931, _, _, _, _, _, _, _, _) => {
            h931
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h932, _, _, _, _, _, _, _) => {
            h932
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h933, _, _, _, _, _, _) => {
            h933
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h934, _, _, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h935, _, _, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h936, _, _, _) => {
            h936
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h937, _, _) => {
            h937
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h938, _) => {
            h938
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h939) => {
            h939
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a940 = v0;
    let a941 = 23u64;
    if nat_eq(a940, a941) {
        f95(v1, v4, v6)
    } else {
        let a942 = v0;
        let a943 = 26u64;
        if nat_eq(a942, a943) {
            f96(v1, v2, v4, v6)
        } else {
            let a944 = v0;
            let a945 = 25u64;
            if nat_eq(a944, a945) {
                f97(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a946 = v0;
    let a947 = 30u64;
    if nat_eq(a946, a947) {
        2u64
    } else {
        4u64
    }
}

pub fn f31(v0: u64) -> bool {
    let a948 = v0;
    let a949 = 2139095040u64;
    if nat_le(a948, a949) {
        true
    } else {
        let a950 = v0;
        let a951 = 2147483648u64;
        nat_eq(a950, a951)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> bool {
    let a953 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h952, _, _, _, _, _) => {
            h952
        }
    };
    let a954 = v1;
    if if nat_eq(a953, a954) {
        let a959 = {
            let a956 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h955) => {
                    h955
                }
            };
            let a957 = v1;
            let a958 = 0u64;
            nat_rem(a956, a957, a958)
        };
        let a960 = 0u64;
        nat_eq(a959, a960)
    } else {
        false
    } {
        f98(match v0.clone() {
            Adt2::C0(_, _, h961, _, _, _, _, _, _, _, _) => {
                h961
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h962, _, _, _, _, _, _, _) => {
                h962
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h963, _, _, _, _, _, _) => {
                h963
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h964, _, _, _, _) => {
                h964
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h965, _, _, _) => {
                h965
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h966, _, _) => {
                h966
            }
        }, v1)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a969 = match v0.clone() {
            Adt2::C0(h967, _, _, _, _, _, _, _, _, _, _) => {
                h967
            }
        };
        let a970 = match v1.clone() {
            Adt2::C0(h968, _, _, _, _, _, _, _, _, _, _) => {
                h968
            }
        };
        nat_eq(a969, a970)
    } else {
        false
    } {
        let a973 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h971) => {
                h971
            }
        };
        let a974 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h972) => {
                h972
            }
        };
        nat_eq(a973, a974)
    } else {
        false
    } {
        let a977 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h975, _, _, _, _, _) => {
                h975
            }
        };
        let a978 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h976, _, _, _, _, _) => {
                h976
            }
        };
        nat_eq(a977, a978)
    } else {
        false
    } {
        let a981 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h979, _, _, _, _) => {
                h979
            }
        };
        let a982 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h980, _, _, _, _) => {
                h980
            }
        };
        nat_eq(a981, a982)
    } else {
        false
    } {
        let a985 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h983, _, _, _) => {
                h983
            }
        };
        let a986 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h984, _, _, _) => {
                h984
            }
        };
        nat_eq(a985, a986)
    } else {
        false
    } {
        let a989 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h987, _, _) => {
                h987
            }
        };
        let a990 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h988, _, _) => {
                h988
            }
        };
        nat_eq(a989, a990)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a991 = v0;
    let a992 = 0u64;
    if if if nat_eq(a991, a992) {
        true
    } else {
        let a993 = v1;
        let a994 = 0u64;
        nat_eq(a993, a994)
    } {
        true
    } else {
        let a995 = v2;
        let a996 = 0u64;
        nat_eq(a995, a996)
    } {
        true
    } else {
        let a997 = v3;
        let a998 = 0u64;
        nat_eq(a997, a998)
    }
}

pub fn f35(v0: Adt2, v1: u64) -> R<bool> {
    f99({
        let a1000 = match v0.clone() {
            Adt2::C0(_, h999, _, _, _, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1001 = v1;
        nat_mul(a1000, a1001)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1002, _, _, _, _, _, _, _, _) => {
            h1002
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1003, _, _, _, _, _, _, _) => {
            h1003
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1004, _, _, _, _, _, _) => {
            h1004
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1005, _, _, _, _) => {
            h1005
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1006, _, _, _) => {
            h1006
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1007, _, _) => {
            h1007
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1008 = v1;
    let a1009 = 0u64;
    if if nat_eq(a1008, a1009) {
        true
    } else {
        let a1010 = v2;
        let a1011 = 0u64;
        nat_eq(a1010, a1011)
    } {
        Ok(0u64)
    } else {
        let a1018 = {
            let a1012 = f100(v1, v2, v3)?;
            let a1013 = f101(v0, v1, v2)?;
            nat_add(a1012, a1013)?
        };
        let a1019 = {
            let a1016 = f102(v0);
            let a1017 = {
                let a1014 = v1;
                let a1015 = v2;
                nat_mul(a1014, a1015)?
            };
            nat_mul(a1016, a1017)?
        };
        nat_add(a1018, a1019)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1020 = v0;
    let a1021 = 0u64;
    if if nat_eq(a1020, a1021) {
        true
    } else {
        let a1022 = v1;
        let a1023 = 0u64;
        nat_eq(a1022, a1023)
    } {
        Ok(3u64)
    } else {
        let a1026 = {
            let a1024 = 3u64;
            let a1025 = v1;
            nat_add(a1024, a1025)?
        };
        let a1027 = v2;
        nat_add(a1026, a1027)
    }
}

pub fn f38(v0: u64) -> bool {
    let a1028 = v0;
    let a1029 = 2139095040u64;
    if nat_le(a1028, a1029) {
        true
    } else {
        let a1030 = v0;
        let a1031 = 2147483648u64;
        nat_eq(a1030, a1031)
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a1032 = v0;
    let a1033 = 0u64;
    if nat_eq(a1032, a1033) {
        Ok(0u64)
    } else {
        let a1034 = v1;
        let a1035 = f103(v0)?;
        nat_mul(a1034, a1035)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1036 = v0;
    let a1037 = 0u64;
    if if if if nat_eq(a1036, a1037) {
        true
    } else {
        let a1038 = v1;
        let a1039 = 0u64;
        nat_eq(a1038, a1039)
    } {
        true
    } else {
        let a1040 = v2;
        let a1041 = 0u64;
        nat_eq(a1040, a1041)
    } {
        true
    } else {
        let a1042 = v3;
        let a1043 = 0u64;
        nat_eq(a1042, a1043)
    } {
        Ok(0u64)
    } else {
        let a1056 = {
            let a1046 = 2u64;
            let a1047 = {
                let a1044 = v1;
                let a1045 = v2;
                nat_mul(a1044, a1045)?
            };
            nat_mul(a1046, a1047)?
        };
        let a1057 = {
            let a1054 = {
                let a1050 = {
                    let a1048 = v1;
                    let a1049 = v2;
                    nat_mul(a1048, a1049)?
                };
                let a1051 = v3;
                nat_mul(a1050, a1051)?
            };
            let a1055 = {
                let a1052 = 1u64;
                let a1053 = v0;
                nat_add(a1052, a1053)?
            };
            nat_mul(a1054, a1055)?
        };
        nat_add(a1056, a1057)
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h1058, _, _, _, _, _, _, _, _, _, _, _) => {
            h1058
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h1059, _, _, _, _, _, _, _, _, _, _) => {
            h1059
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h1060, _, _, _, _, _, _, _, _, _) => {
            h1060
        }
    };
    let v6: u64 = {
        let a1062 = match v1.clone() {
            Adt2::C0(_, h1061, _, _, _, _, _, _, _, _, _) => {
                h1061
            }
        };
        let a1063 = v4;
        let a1064 = 0u64;
        nat_quot(a1062, a1063, a1064)
    };
    let v7: u64 = {
        let a1066 = match v1.clone() {
            Adt2::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
                h1065
            }
        };
        let a1067 = 0u64;
        if nat_eq(a1066, a1067) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f104(match v1.clone() {
        Adt2::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    }) {
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
        nat_eq(a1071, a1072)
    } else {
        false
    } {
        let a1073 = 0u64;
        let a1074 = v3;
        nat_lt(a1073, a1074)
    } else {
        false
    } {
        let a1075 = v3;
        let a1076 = 2147483648u64;
        nat_lt(a1075, a1076)
    } else {
        false
    } {
        let a1077 = 0u64;
        let a1078 = v4;
        nat_lt(a1077, a1078)
    } else {
        false
    } {
        let a1079 = v4;
        let a1080 = 2147483648u64;
        nat_lt(a1079, a1080)
    } else {
        false
    } {
        let a1081 = v5;
        let a1082 = 2147483648u64;
        nat_lt(a1081, a1082)
    } else {
        false
    } {
        let a1083 = 0u64;
        let a1084 = v6;
        nat_lt(a1083, a1084)
    } else {
        false
    } {
        let a1086 = 0u64;
        let a1087 = match v2.clone() {
            Adt2::C0(_, h1085, _, _, _, _, _, _, _, _, _) => {
                h1085
            }
        };
        nat_lt(a1086, a1087)
    } else {
        false
    } {
        let a1091 = match v1.clone() {
            Adt2::C0(_, h1088, _, _, _, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1092 = {
            let a1089 = v6;
            let a1090 = v4;
            nat_mul(a1089, a1090)?
        };
        nat_eq(a1091, a1092)
    } else {
        false
    } {
        let a1094 = match v2.clone() {
            Adt2::C0(_, _, h1093, _, _, _, _, _, _, _, _) => {
                h1093
            }
        };
        let a1095 = v4;
        nat_eq(a1094, a1095)
    } else {
        false
    } {
        let a1108 = {
            let a1101 = {
                let a1099 = match v2.clone() {
                    Adt2::C0(_, h1096, _, _, _, _, _, _, _, _, _) => {
                        h1096
                    }
                };
                let a1100 = {
                    let a1097 = v5;
                    let a1098 = 2u64;
                    nat_mul(a1097, a1098)?
                };
                nat_add(a1099, a1100)?
            };
            let a1102 = v3;
            nat_add(a1101, a1102)?
        };
        let a1109 = {
            let a1106 = {
                let a1104 = match v1.clone() {
                    Adt2::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                        h1103
                    }
                };
                let a1105 = v3;
                nat_mul(a1104, a1105)?
            };
            let a1107 = v6;
            nat_add(a1106, a1107)?
        };
        nat_eq(a1108, a1109)
    } else {
        false
    } {
        f105(v1.clone(), v7)?
    } else {
        false
    } {
        f105(v2.clone(), v7)?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a1110 = f16(v2.clone(), 1u64)?;
            let a1111 = f106(v8.clone())?;
            nat_mul(a1110, a1111)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h1112, _, _, _, _, _, _, _, _, _, _) => {
                h1112
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f42(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1113 = v1;
    match m1113 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a1115 = match v0.clone() {
                Adt2::C0(h1114, _, _, _, _, _, _, _, _, _, _) => {
                    h1114
                }
            };
            let a1116 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a1115, a1116) {
                let a1118 = match v3.clone() {
                    Adt2::C0(h1117, _, _, _, _, _, _, _, _, _, _) => {
                        h1117
                    }
                };
                let a1119 = 0u64;
                nat_eq(a1118, a1119)
            } else {
                false
            } {
                let a1121 = match v2.clone() {
                    Adt2::C0(h1120, _, _, _, _, _, _, _, _, _, _) => {
                        h1120
                    }
                };
                let a1122 = 0u64;
                nat_eq(a1121, a1122)
            } else {
                false
            } {
                let a1124 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h1123, _, _, _, _, _, _) => {
                        h1123
                    }
                };
                let a1125 = 1u64;
                nat_eq(a1124, a1125)
            } else {
                false
            } {
                let a1127 = match v3.clone() {
                    Adt2::C0(_, _, _, h1126, _, _, _, _, _, _, _) => {
                        h1126
                    }
                };
                let a1128 = 1u64;
                nat_eq(a1127, a1128)
            } else {
                false
            } {
                let a1130 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1129, _, _, _, _, _, _) => {
                        h1129
                    }
                };
                let a1131 = 1u64;
                nat_eq(a1130, a1131)
            } else {
                false
            } {
                let a1134 = match v0.clone() {
                    Adt2::C0(_, _, h1132, _, _, _, _, _, _, _, _) => {
                        h1132
                    }
                };
                let a1135 = match v3.clone() {
                    Adt2::C0(_, _, h1133, _, _, _, _, _, _, _, _) => {
                        h1133
                    }
                };
                nat_eq(a1134, a1135)
            } else {
                false
            } {
                let a1140 = match v3.clone() {
                    Adt2::C0(_, h1136, _, _, _, _, _, _, _, _, _) => {
                        h1136
                    }
                };
                let a1141 = {
                    let a1138 = match v0.clone() {
                        Adt2::C0(_, h1137, _, _, _, _, _, _, _, _, _) => {
                            h1137
                        }
                    };
                    let a1139 = 1u64;
                    nat_add(a1138, a1139)?
                };
                nat_le(a1140, a1141)
            } else {
                false
            } {
                let a1144 = match v2.clone() {
                    Adt2::C0(_, h1142, _, _, _, _, _, _, _, _, _) => {
                        h1142
                    }
                };
                let a1145 = match v0.clone() {
                    Adt2::C0(_, _, h1143, _, _, _, _, _, _, _, _) => {
                        h1143
                    }
                };
                nat_eq(a1144, a1145)
            } else {
                false
            } {
                let a1153 = match v2.clone() {
                    Adt2::C0(_, _, h1146, _, _, _, _, _, _, _, _) => {
                        h1146
                    }
                };
                let a1154 = {
                    let a1151 = {
                        let a1148 = match v0.clone() {
                            Adt2::C0(_, h1147, _, _, _, _, _, _, _, _, _) => {
                                h1147
                            }
                        };
                        let a1149 = 1u64;
                        nat_add(a1148, a1149)?
                    };
                    let a1152 = match v3.clone() {
                        Adt2::C0(_, h1150, _, _, _, _, _, _, _, _, _) => {
                            h1150
                        }
                    };
                    nat_sub(a1151, a1152)
                };
                nat_eq(a1153, a1154)
            } else {
                false
            } {
                let a1157 = match v2.clone() {
                    Adt2::C0(_, _, _, h1155, _, _, _, _, _, _, _) => {
                        h1155
                    }
                };
                let a1158 = match v0.clone() {
                    Adt2::C0(_, _, _, h1156, _, _, _, _, _, _, _) => {
                        h1156
                    }
                };
                nat_eq(a1157, a1158)
            } else {
                false
            } {
                let a1160 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h1159, _, _, _, _, _, _) => {
                        h1159
                    }
                };
                let a1161 = 1u64;
                nat_eq(a1160, a1161)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a1163 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h1162, _, _, _, _, _) => {
                        h1162
                    }
                };
                let a1164 = 4u64;
                nat_eq(a1163, a1164)
            } else {
                false
            } {
                let a1166 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h1165, _, _, _, _, _) => {
                        h1165
                    }
                };
                let a1167 = 4u64;
                nat_eq(a1166, a1167)
            } else {
                false
            } {
                let a1172 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1168, _, _, _, _) => {
                        h1168
                    }
                };
                let a1173 = {
                    let a1170 = match v0.clone() {
                        Adt2::C0(_, h1169, _, _, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    let a1171 = 4u64;
                    nat_mul(a1170, a1171)?
                };
                nat_eq(a1172, a1173)
            } else {
                false
            } {
                let a1178 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1174, _, _, _, _) => {
                        h1174
                    }
                };
                let a1179 = {
                    let a1176 = match v3.clone() {
                        Adt2::C0(_, h1175, _, _, _, _, _, _, _, _, _) => {
                            h1175
                        }
                    };
                    let a1177 = 4u64;
                    nat_mul(a1176, a1177)?
                };
                nat_eq(a1178, a1179)
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
                    let a1181 = f16(v2.clone(), 1u64)?;
                    let a1182 = f107(match v3.clone() {
                        Adt2::C0(_, h1180, _, _, _, _, _, _, _, _, _) => {
                            h1180
                        }
                    })?;
                    nat_mul(a1181, a1182)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f43(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h1183, _, _, _, _, _, _, _, _, _, _) => {
            h1183
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h1184, _, _, _, _, _, _, _, _) => {
                h1184
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1185, _, _, _, _, _, _, _, _, _) => {
                h1185
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1186, _, _, _, _, _, _) => {
                h1186
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h1187, _, _, _, _, _, _, _, _) => {
                h1187
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h1188, _, _, _, _, _, _, _, _, _) => {
                h1188
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h1189, _, _, _, _, _, _, _) => {
                h1189
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1190, _, _, _, _, _) => {
                h1190
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a1192 = match v1.clone() {
        Adt1::C0(h1191, _, _, _, _, _, _, _, _, _, _, _) => {
            h1191
        }
    };
    let a1193 = 1u64;
    if if if if if if if if if if if nat_le(a1192, a1193) {
        let a1195 = match v2.clone() {
            Adt2::C0(h1194, _, _, _, _, _, _, _, _, _, _) => {
                h1194
            }
        };
        let a1196 = 1u64;
        nat_le(a1195, a1196)
    } else {
        false
    } {
        let a1198 = match v3.clone() {
            Adt2::C0(h1197, _, _, _, _, _, _, _, _, _, _) => {
                h1197
            }
        };
        let a1199 = 0u64;
        nat_eq(a1198, a1199)
    } else {
        false
    } {
        let a1204 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1200, _, _, _, _, _) => {
                h1200
            }
        };
        let a1205 = {
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
        };
        nat_eq(a1204, a1205)
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
        f108(v4.clone(), match v2.clone() {
            Adt2::C0(_, h1206, _, _, _, _, _, _, _, _, _) => {
                h1206
            }
        }, match v3.clone() {
            Adt2::C0(_, h1207, _, _, _, _, _, _, _, _, _) => {
                h1207
            }
        })?
    } else {
        false
    } {
        if v0 {
            f108(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1208, _, _, _, _, _, _, _, _) => {
                    h1208
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h1209, _, _, _, _, _, _, _, _) => {
                    h1209
                }
            })?
        } else {
            let a1212 = match v2.clone() {
                Adt2::C0(_, _, h1210, _, _, _, _, _, _, _, _) => {
                    h1210
                }
            };
            let a1213 = match v3.clone() {
                Adt2::C0(_, _, h1211, _, _, _, _, _, _, _, _) => {
                    h1211
                }
            };
            nat_eq(a1212, a1213)
        }
    } else {
        false
    } {
        let a1216 = match v2.clone() {
            Adt2::C0(_, _, _, h1214, _, _, _, _, _, _, _) => {
                h1214
            }
        };
        let a1217 = match v3.clone() {
            Adt2::C0(_, _, _, h1215, _, _, _, _, _, _, _) => {
                h1215
            }
        };
        nat_eq(a1216, a1217)
    } else {
        false
    } {
        let a1220 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1218, _, _, _, _, _, _) => {
                h1218
            }
        };
        let a1221 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1219, _, _, _, _, _, _) => {
                h1219
            }
        };
        nat_eq(a1220, a1221)
    } else {
        false
    } {
        let a1226 = {
            let a1224 = match v4.clone() {
                Adt6::C0(h1222, _, _, _) => {
                    h1222
                }
            };
            let a1225 = match v5.clone() {
                Adt6::C0(h1223, _, _, _) => {
                    h1223
                }
            };
            nat_mul(a1224, a1225)?
        };
        let a1227 = 2147483648u64;
        nat_lt(a1226, a1227)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a1229 = match v1 {
                Adt1::C0(h1228, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1228
                }
            };
            let a1230 = 0u64;
            nat_eq(a1229, a1230)
        }, match v2.clone() {
            Adt2::C0(h1231, _, _, _, _, _, _, _, _, _, _) => {
                h1231
            }
        });
        Ok(Adt10::C4({
            let a1232 = f16(v3.clone(), 1u64)?;
            let a1233 = f109(v6.clone())?;
            nat_mul(a1232, a1233)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f44(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1234 = v2;
    match m1234 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a1236 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1235, _, _, _, _, _) => {
                        h1235
                    }
                };
                let a1237 = 1u64;
                nat_eq(a1236, a1237)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1238, _, _, _, _, _, _, _, _, _) => {
                    h1238
                }
            }, match v0.clone() {
                Adt1::C0(h1239, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1239
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1240, _, _, _, _, _, _, _, _, _) => {
                    h1240
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1241, _, _, _, _, _, _, _) => {
                    h1241
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h1242, _, _, _, _, _, _, _, _) => {
                        h1242
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h1243, _, _, _, _, _, _, _, _, _, _) => {
                        h1243
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h1244, _, _, _, _, _, _, _, _) => {
                        h1244
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h1245, _, _, _, _, _, _) => {
                        h1245
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1246, _, _, _, _, _, _, _) => {
                        h1246
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h1247, _, _, _, _, _, _, _, _) => {
                        h1247
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h1248, _, _, _, _, _, _) => {
                        h1248
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1249, _, _, _, _, _, _, _) => {
                        h1249
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h1250, _, _, _, _, _, _, _, _) => {
                        h1250
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h1251, _, _, _, _, _, _, _, _, _, _) => {
                    h1251
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h1252, _, _, _, _, _, _, _, _, _, _) => {
                    h1252
                }
            };
            let v14: u64 = {
                let a1253 = v12;
                let a1254 = 0u64;
                if nat_eq(a1253, a1254) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a1256 = match v4.clone() {
                    Adt2::C0(_, h1255, _, _, _, _, _, _, _, _, _) => {
                        h1255
                    }
                };
                let a1257 = v14;
                nat_mul(a1256, a1257)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1258, _, _, _) => {
                        h1258
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1259, _, _, _, _) => {
                        h1259
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1260, _, _) => {
                        h1260
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1261, _, _, _) => {
                        h1261
                    }
                }
            };
            let a1263 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h1262, _, _, _, _, _) => {
                    h1262
                }
            };
            let a1264 = 1u64;
            if if if if if if if if if if nat_le(a1263, a1264) {
                let a1265 = v12;
                let a1266 = 0u64;
                if nat_eq(a1265, a1266) {
                    true
                } else {
                    let a1267 = v12;
                    let a1268 = 1u64;
                    if nat_eq(a1267, a1268) {
                        let a1269 = v13;
                        let a1270 = 1u64;
                        nat_eq(a1269, a1270)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a1271 = v13;
                let a1272 = 0u64;
                if nat_eq(a1271, a1272) {
                    true
                } else {
                    let a1273 = v13;
                    let a1274 = 1u64;
                    nat_eq(a1273, a1274)
                }
            } else {
                false
            } {
                let a1276 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1275, _, _, _, _, _) => {
                        h1275
                    }
                };
                let a1277 = v14;
                nat_eq(a1276, a1277)
            } else {
                false
            } {
                let a1285 = match v3.clone() {
                    Adt2::C0(_, h1278, _, _, _, _, _, _, _, _, _) => {
                        h1278
                    }
                };
                let a1286 = {
                    let a1283 = v9;
                    let a1284 = {
                        let a1281 = match v6.clone() {
                            Adt6::C0(h1279, _, _, _) => {
                                h1279
                            }
                        };
                        let a1282 = match v7.clone() {
                            Adt6::C0(h1280, _, _, _) => {
                                h1280
                            }
                        };
                        nat_mul(a1281, a1282)?
                    };
                    nat_mul(a1283, a1284)?
                };
                nat_eq(a1285, a1286)
            } else {
                false
            } {
                let a1289 = match v3.clone() {
                    Adt2::C0(_, _, h1287, _, _, _, _, _, _, _, _) => {
                        h1287
                    }
                };
                let a1290 = f92(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1288, _, _, _, _, _, _, _, _, _) => {
                        h1288
                    }
                })?;
                nat_eq(a1289, a1290)
            } else {
                false
            } {
                let a1292 = 0u64;
                let a1293 = match v3.clone() {
                    Adt2::C0(_, _, h1291, _, _, _, _, _, _, _, _) => {
                        h1291
                    }
                };
                nat_lt(a1292, a1293)
            } else {
                false
            } {
                if v5 {
                    let a1295 = match v1 {
                        Adt2::C0(_, _, _, h1294, _, _, _, _, _, _, _) => {
                            h1294
                        }
                    };
                    let a1296 = v9;
                    if if if nat_eq(a1295, a1296) {
                        let a1298 = match v3.clone() {
                            Adt2::C0(_, _, _, h1297, _, _, _, _, _, _, _) => {
                                h1297
                            }
                        };
                        let a1299 = f92(v7.clone(), v11)?;
                        nat_eq(a1298, a1299)
                    } else {
                        false
                    } {
                        let a1301 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1300, _, _, _, _, _, _) => {
                                h1300
                            }
                        };
                        let a1302 = v10;
                        nat_eq(a1301, a1302)
                    } else {
                        false
                    } {
                        let a1304 = 0u64;
                        let a1305 = match v3.clone() {
                            Adt2::C0(_, _, _, h1303, _, _, _, _, _, _, _) => {
                                h1303
                            }
                        };
                        nat_lt(a1304, a1305)
                    } else {
                        false
                    }
                } else {
                    let a1307 = match v1 {
                        Adt2::C0(_, _, h1306, _, _, _, _, _, _, _, _) => {
                            h1306
                        }
                    };
                    let a1308 = v9;
                    if if if nat_eq(a1307, a1308) {
                        let a1310 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1309, _, _, _, _, _, _) => {
                                h1309
                            }
                        };
                        let a1311 = 1u64;
                        nat_eq(a1310, a1311)
                    } else {
                        false
                    } {
                        let a1313 = match v3.clone() {
                            Adt2::C0(_, _, _, h1312, _, _, _, _, _, _, _) => {
                                h1312
                            }
                        };
                        let a1314 = v10;
                        nat_eq(a1313, a1314)
                    } else {
                        false
                    } {
                        let a1316 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1315, _, _, _, _, _, _) => {
                                h1315
                            }
                        };
                        let a1317 = 1u64;
                        nat_eq(a1316, a1317)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f34(match v4.clone() {
                    Adt2::C0(_, h1318, _, _, _, _, _, _, _, _, _) => {
                        h1318
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1338 = {
                        let a1335 = {
                            let a1333 = {
                                let a1327 = {
                                    let a1321 = {
                                        let a1319 = v10;
                                        let a1320 = 1u64;
                                        nat_sub(a1319, a1320)
                                    };
                                    let a1322 = v17;
                                    nat_mul(a1321, a1322)?
                                };
                                let a1328 = {
                                    let a1325 = {
                                        let a1323 = v9;
                                        let a1324 = 1u64;
                                        nat_sub(a1323, a1324)
                                    };
                                    let a1326 = v16;
                                    nat_mul(a1325, a1326)?
                                };
                                nat_add(a1327, a1328)?
                            };
                            let a1334 = {
                                let a1331 = {
                                    let a1329 = v11;
                                    let a1330 = 1u64;
                                    nat_sub(a1329, a1330)
                                };
                                let a1332 = v15;
                                nat_mul(a1331, a1332)?
                            };
                            nat_add(a1333, a1334)?
                        };
                        let a1336 = v15;
                        nat_add(a1335, a1336)?
                    };
                    let a1339 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1337, _) => {
                            h1337
                        }
                    };
                    nat_le(a1338, a1339)
                }
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1340 = v13;
                    let a1341 = 0u64;
                    if nat_eq(a1340, a1341) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1342, _, _, _, _, _, _, _, _, _) => {
                        h1342
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1343, _, _, _, _, _, _, _, _, _) => {
                            h1343
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1344, _, _, _, _, _, _, _, _) => {
                            h1344
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1345, _, _, _, _, _, _, _) => {
                            h1345
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1346, _, _, _, _, _) => {
                            h1346
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1347, _, _, _, _) => {
                            h1347
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1348, _, _, _) => {
                            h1348
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f45(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1349 = v2;
    match m1349 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1350, _, _, _, _, _, _, _, _, _) => {
                    h1350
                }
            }, match v0.clone() {
                Adt1::C0(h1351, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1351
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1352, _, _, _, _, _, _, _, _) => {
                    h1352
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1353, _, _, _, _, _) => {
                    h1353
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1354, _, _, _, _, _, _, _, _) => {
                    h1354
                }
            }, match v0.clone() {
                Adt1::C0(_, h1355, _, _, _, _, _, _, _, _, _, _) => {
                    h1355
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1356, _, _, _, _, _, _, _) => {
                    h1356
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1357, _, _, _, _) => {
                    h1357
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1358, _, _, _, _, _, _, _) => {
                    h1358
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1359, _, _, _, _, _, _, _, _, _) => {
                    h1359
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1360, _, _, _, _, _, _) => {
                    h1360
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1361, _, _, _) => {
                    h1361
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1362, _, _) => {
                    h1362
                }
            };
            let v9: u64 = f92(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1363, _, _, _, _, _, _, _) => {
                    h1363
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1364, _, _, _, _, _, _, _, _, _, _) => {
                    h1364
                }
            };
            let v11: u64 = {
                let a1372 = v8;
                let a1373 = {
                    let a1370 = match v5.clone() {
                        Adt6::C0(h1365, _, _, _) => {
                            h1365
                        }
                    };
                    let a1371 = {
                        let a1368 = match v6.clone() {
                            Adt6::C0(h1366, _, _, _) => {
                                h1366
                            }
                        };
                        let a1369 = match v7.clone() {
                            Adt6::C0(h1367, _, _, _) => {
                                h1367
                            }
                        };
                        nat_mul(a1368, a1369)?
                    };
                    nat_mul(a1370, a1371)?
                };
                nat_mul(a1372, a1373)?
            };
            let a1375 = match v4.clone() {
                Adt2::C0(h1374, _, _, _, _, _, _, _, _, _, _) => {
                    h1374
                }
            };
            let a1376 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1375, a1376) {
                let a1377 = v10;
                let a1378 = 0u64;
                if nat_eq(a1377, a1378) {
                    true
                } else {
                    let a1379 = v10;
                    let a1380 = 1u64;
                    nat_eq(a1379, a1380)
                }
            } else {
                false
            } {
                let a1381 = 0u64;
                let a1382 = v8;
                nat_lt(a1381, a1382)
            } else {
                false
            } {
                let a1383 = v8;
                let a1384 = 2147483648u64;
                nat_lt(a1383, a1384)
            } else {
                false
            } {
                let a1386 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1385, _, _, _, _, _) => {
                        h1385
                    }
                };
                let a1387 = 4u64;
                nat_eq(a1386, a1387)
            } else {
                false
            } {
                let a1388 = 0u64;
                let a1389 = v9;
                nat_lt(a1388, a1389)
            } else {
                false
            } {
                let a1391 = 0u64;
                let a1392 = match v3.clone() {
                    Adt2::C0(_, _, h1390, _, _, _, _, _, _, _, _) => {
                        h1390
                    }
                };
                nat_lt(a1391, a1392)
            } else {
                false
            } {
                let a1394 = 0u64;
                let a1395 = match v3.clone() {
                    Adt2::C0(_, _, _, h1393, _, _, _, _, _, _, _) => {
                        h1393
                    }
                };
                nat_lt(a1394, a1395)
            } else {
                false
            } {
                let a1397 = match v3.clone() {
                    Adt2::C0(_, h1396, _, _, _, _, _, _, _, _, _) => {
                        h1396
                    }
                };
                let a1398 = v11;
                nat_eq(a1397, a1398)
            } else {
                false
            } {
                let a1401 = match v3.clone() {
                    Adt2::C0(_, _, h1399, _, _, _, _, _, _, _, _) => {
                        h1399
                    }
                };
                let a1402 = f92(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1400, _, _, _, _, _, _, _, _, _) => {
                        h1400
                    }
                })?;
                nat_eq(a1401, a1402)
            } else {
                false
            } {
                let a1405 = match v3.clone() {
                    Adt2::C0(_, _, _, h1403, _, _, _, _, _, _, _) => {
                        h1403
                    }
                };
                let a1406 = f92(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1404, _, _, _, _, _, _, _, _) => {
                        h1404
                    }
                })?;
                nat_eq(a1405, a1406)
            } else {
                false
            } {
                let a1414 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1407, _, _, _, _, _, _) => {
                        h1407
                    }
                };
                let a1415 = {
                    let a1412 = v9;
                    let a1413 = {
                        let a1409 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1408, _, _, _, _, _, _) => {
                                h1408
                            }
                        };
                        let a1410 = v8;
                        let a1411 = 0u64;
                        nat_quot(a1409, a1410, a1411)
                    };
                    nat_mul(a1412, a1413)?
                };
                nat_eq(a1414, a1415)
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1416 = v10;
                    let a1417 = 0u64;
                    if nat_eq(a1416, a1417) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1418, _, _) => {
                        h1418
                    }
                }, {
                    let a1420 = v8;
                    let a1421 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1419, _, _) => {
                            h1419
                        }
                    };
                    nat_mul(a1420, a1421)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f46(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1422 = v1;
    match m1422 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f110(v0, v3, v2)
        }
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1424 = match v0.clone() {
        Adt2::C0(h1423, _, _, _, _, _, _, _, _, _, _) => {
            h1423
        }
    };
    let a1425 = 1u64;
    if if nat_eq(a1424, a1425) {
        true
    } else {
        let a1427 = match v0.clone() {
            Adt2::C0(h1426, _, _, _, _, _, _, _, _, _, _) => {
                h1426
            }
        };
        let a1428 = 30u64;
        nat_eq(a1427, a1428)
    } {
        let a1430 = match v1.clone() {
            Adt2::C0(h1429, _, _, _, _, _, _, _, _, _, _) => {
                h1429
            }
        };
        let a1431 = 1u64;
        if if if if if if if if if if if nat_eq(a1430, a1431) {
            let a1434 = match v0.clone() {
                Adt2::C0(_, h1432, _, _, _, _, _, _, _, _, _) => {
                    h1432
                }
            };
            let a1435 = match v1.clone() {
                Adt2::C0(_, h1433, _, _, _, _, _, _, _, _, _) => {
                    h1433
                }
            };
            nat_eq(a1434, a1435)
        } else {
            false
        } {
            let a1437 = 0u64;
            let a1438 = match v1.clone() {
                Adt2::C0(_, _, h1436, _, _, _, _, _, _, _, _) => {
                    h1436
                }
            };
            nat_lt(a1437, a1438)
        } else {
            false
        } {
            let a1441 = match v1.clone() {
                Adt2::C0(_, _, h1439, _, _, _, _, _, _, _, _) => {
                    h1439
                }
            };
            let a1442 = match v1.clone() {
                Adt2::C0(_, _, _, h1440, _, _, _, _, _, _, _) => {
                    h1440
                }
            };
            nat_eq(a1441, a1442)
        } else {
            false
        } {
            let a1444 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1443, _, _, _, _, _, _) => {
                    h1443
                }
            };
            let a1445 = 1u64;
            nat_eq(a1444, a1445)
        } else {
            false
        } {
            let a1453 = match v0.clone() {
                Adt2::C0(_, _, h1446, _, _, _, _, _, _, _, _) => {
                    h1446
                }
            };
            let a1454 = {
                let a1451 = {
                    let a1449 = match v1.clone() {
                        Adt2::C0(_, _, h1447, _, _, _, _, _, _, _, _) => {
                            h1447
                        }
                    };
                    let a1450 = match v1.clone() {
                        Adt2::C0(_, _, h1448, _, _, _, _, _, _, _, _) => {
                            h1448
                        }
                    };
                    nat_add(a1449, a1450)?
                };
                let a1452 = 1u64;
                nat_sub(a1451, a1452)
            };
            nat_eq(a1453, a1454)
        } else {
            false
        } {
            let a1456 = 0u64;
            let a1457 = match v0.clone() {
                Adt2::C0(_, _, _, h1455, _, _, _, _, _, _, _) => {
                    h1455
                }
            };
            nat_lt(a1456, a1457)
        } else {
            false
        } {
            let a1459 = 0u64;
            let a1460 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1458, _, _, _, _, _, _) => {
                    h1458
                }
            };
            nat_lt(a1459, a1460)
        } else {
            false
        } {
            let a1462 = 2u64;
            let a1463 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1461, _, _, _, _, _) => {
                    h1461
                }
            };
            nat_le(a1462, a1463)
        } else {
            false
        } {
            let a1465 = 2u64;
            let a1466 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1464, _, _, _, _, _) => {
                    h1464
                }
            };
            nat_le(a1465, a1466)
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

pub fn f48(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1467 = v1;
    match m1467 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f111(v0, v3, v2)
        }
    }
}

pub fn f49(v0: u64) -> bool {
    let a1468 = v0;
    let a1469 = 1u64;
    if if nat_eq(a1468, a1469) {
        true
    } else {
        let a1470 = v0;
        let a1471 = 33u64;
        nat_eq(a1470, a1471)
    } {
        true
    } else {
        let a1472 = v0;
        let a1473 = 34u64;
        nat_eq(a1472, a1473)
    }
}

pub fn f50(v0: u64) -> bool {
    if f112(v0) {
        !f113(v0)
    } else {
        false
    }
}

pub fn f51(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1474 = v1;
    match m1474 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1476 = f114(match v0.clone() {
                Adt2::C0(h1475, _, _, _, _, _, _, _, _, _, _) => {
                    h1475
                }
            });
            match m1476 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1477, _) => {
                            h1477
                        }
                    };
                    if if if if if if f33(v3, v2.clone()) {
                        f23(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1478 = f16(v0.clone(), 1u64)?;
                        let a1479 = f16(v2.clone(), 1u64)?;
                        nat_eq(a1478, a1479)
                    } else {
                        false
                    } {
                        let a1484 = {
                            let a1481 = match v0.clone() {
                                Adt2::C0(_, h1480, _, _, _, _, _, _, _, _, _) => {
                                    h1480
                                }
                            };
                            let a1482 = v5;
                            let a1483 = 1u64;
                            nat_rem(a1481, a1482, a1483)
                        };
                        let a1485 = 0u64;
                        if nat_eq(a1484, a1485) {
                            let a1488 = match v4 {
                                Adt18::C0(_, _, h1486) => {
                                    h1486
                                }
                            };
                            let a1489 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1487, _, _, _, _, _) => {
                                    h1487
                                }
                            };
                            nat_le(a1488, a1489)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1490 = v5;
                        let a1491 = 1u64;
                        if nat_eq(a1490, a1491) {
                            let a1493 = 4u64;
                            let a1494 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1492, _, _, _, _, _) => {
                                    h1492
                                }
                            };
                            nat_le(a1493, a1494)
                        } else {
                            let a1496 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1495, _, _, _, _, _) => {
                                    h1495
                                }
                            };
                            let a1497 = 4u64;
                            nat_eq(a1496, a1497)
                        }
                    } else {
                        false
                    } {
                        let a1502 = {
                            let a1499 = match v2.clone() {
                                Adt2::C0(_, h1498, _, _, _, _, _, _, _, _, _) => {
                                    h1498
                                }
                            };
                            let a1500 = v5;
                            let a1501 = 1u64;
                            nat_rem(a1499, a1500, a1501)
                        };
                        let a1503 = 0u64;
                        if nat_eq(a1502, a1503) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1504, _, _, _, _, _, _, _, _, _, _) => {
                                h1504
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

pub fn f52(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1507 = match v1.clone() {
        Adt2::C0(h1505, _, _, _, _, _, _, _, _, _, _) => {
            h1505
        }
    };
    let a1508 = match v3.clone() {
        Adt2::C0(h1506, _, _, _, _, _, _, _, _, _, _) => {
            h1506
        }
    };
    if nat_eq(a1507, a1508) {
        f115(v0, v1, v2, v3)
    } else {
        if if if if f113(match v1.clone() {
            Adt2::C0(h1509, _, _, _, _, _, _, _, _, _, _) => {
                h1509
            }
        }) {
            f113(match v3.clone() {
                Adt2::C0(h1510, _, _, _, _, _, _, _, _, _, _) => {
                    h1510
                }
            })
        } else {
            false
        } {
            let a1511 = v0;
            let a1512 = 33u64;
            if nat_eq(a1511, a1512) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1513 = f16(v1.clone(), 1u64)?;
            let a1514 = f16(v3.clone(), 1u64)?;
            nat_eq(a1513, a1514)
        } else {
            false
        } {
            let m1516 = f114(match v3.clone() {
                Adt2::C0(h1515, _, _, _, _, _, _, _, _, _, _) => {
                    h1515
                }
            });
            match m1516 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1517, _, _, _, _, _, _, _, _, _, _) => {
                            h1517
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1518, _, _, _, _, _, _, _, _, _, _) => {
                            h1518
                        }
                    };
                    let v7: bool = {
                        let a1519 = v5;
                        let a1520 = 43u64;
                        if nat_eq(a1519, a1520) {
                            true
                        } else {
                            let a1521 = v6;
                            let a1522 = 43u64;
                            nat_eq(a1521, a1522)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1523 = f16(v3.clone(), 1u64)?;
                        let a1524 = if v7 {
                            f116(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1523, a1524)?
                    }, match v4 {
                        Adt18::C0(_, _, h1525) => {
                            h1525
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

pub fn f53(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1527 = match v0.clone() {
        Adt2::C0(h1526, _, _, _, _, _, _, _, _, _, _) => {
            h1526
        }
    };
    let a1528 = 0u64;
    if if if if nat_eq(a1527, a1528) {
        true
    } else {
        let a1530 = match v0.clone() {
            Adt2::C0(h1529, _, _, _, _, _, _, _, _, _, _) => {
                h1529
            }
        };
        let a1531 = 1u64;
        nat_eq(a1530, a1531)
    } {
        true
    } else {
        let a1533 = match v0.clone() {
            Adt2::C0(h1532, _, _, _, _, _, _, _, _, _, _) => {
                h1532
            }
        };
        let a1534 = 25u64;
        if nat_eq(a1533, a1534) {
            true
        } else {
            let a1536 = match v0.clone() {
                Adt2::C0(h1535, _, _, _, _, _, _, _, _, _, _) => {
                    h1535
                }
            };
            let a1537 = 26u64;
            nat_eq(a1536, a1537)
        }
    } {
        true
    } else {
        let a1539 = match v0.clone() {
            Adt2::C0(h1538, _, _, _, _, _, _, _, _, _, _) => {
                h1538
            }
        };
        let a1540 = 30u64;
        nat_eq(a1539, a1540)
    } {
        let a1543 = match v0.clone() {
            Adt2::C0(h1541, _, _, _, _, _, _, _, _, _, _) => {
                h1541
            }
        };
        let a1544 = match v1.clone() {
            Adt2::C0(h1542, _, _, _, _, _, _, _, _, _, _) => {
                h1542
            }
        };
        if if nat_eq(a1543, a1544) {
            f117(match v1.clone() {
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
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1548, _, _, _, _, _, _) => {
                    h1548
                }
            }, match v0.clone() {
                Adt2::C0(_, h1549, _, _, _, _, _, _, _, _, _) => {
                    h1549
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1550, _, _, _, _, _, _, _, _) => {
                    h1550
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1551, _, _, _, _, _, _, _) => {
                    h1551
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1552, _, _, _, _, _, _) => {
                    h1552
                }
            })
        } else {
            false
        } {
            let m1554 = f114(match v0.clone() {
                Adt2::C0(h1553, _, _, _, _, _, _, _, _, _, _) => {
                    h1553
                }
            });
            match m1554 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1555) => {
                            h1555
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

pub fn f54(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1556 = v2;
    match m1556 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1557 = v0;
            let a1558 = 4u64;
            if if if nat_lt(a1557, a1558) {
                let a1561 = match v1.clone() {
                    Adt2::C0(h1559, _, _, _, _, _, _, _, _, _, _) => {
                        h1559
                    }
                };
                let a1562 = match v4.clone() {
                    Adt2::C0(h1560, _, _, _, _, _, _, _, _, _, _) => {
                        h1560
                    }
                };
                if nat_eq(a1561, a1562) {
                    let a1565 = match v1.clone() {
                        Adt2::C0(h1563, _, _, _, _, _, _, _, _, _, _) => {
                            h1563
                        }
                    };
                    let a1566 = match v3.clone() {
                        Adt2::C0(h1564, _, _, _, _, _, _, _, _, _, _) => {
                            h1564
                        }
                    };
                    nat_eq(a1565, a1566)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f118(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1567, _, _, _, _, _, _, _, _, _) => {
                        h1567
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1568, _, _, _, _, _, _, _, _, _) => {
                        h1568
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1569, _, _, _, _, _, _, _, _, _) => {
                        h1569
                    }
                })? {
                    f118(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1570, _, _, _, _, _, _, _, _) => {
                            h1570
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1571, _, _, _, _, _, _, _, _) => {
                            h1571
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1572, _, _, _, _, _, _, _, _) => {
                            h1572
                        }
                    })?
                } else {
                    false
                } {
                    f118(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1573, _, _, _, _, _, _, _) => {
                            h1573
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1574, _, _, _, _, _, _, _) => {
                            h1574
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1575, _, _, _, _, _, _, _) => {
                            h1575
                        }
                    })?
                } else {
                    false
                } {
                    f118(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1576, _, _, _, _, _, _) => {
                            h1576
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1577, _, _, _, _, _, _) => {
                            h1577
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1578, _, _, _, _, _, _) => {
                            h1578
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1580 = f114(match v1.clone() {
                    Adt2::C0(h1579, _, _, _, _, _, _, _, _, _, _) => {
                        h1579
                    }
                });
                match m1580 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1581, _) => {
                                h1581
                            }
                        };
                        let a1586 = {
                            let a1583 = match v1.clone() {
                                Adt2::C0(_, h1582, _, _, _, _, _, _, _, _, _) => {
                                    h1582
                                }
                            };
                            let a1584 = v6;
                            let a1585 = 1u64;
                            nat_rem(a1583, a1584, a1585)
                        };
                        let a1587 = 0u64;
                        if if if nat_eq(a1586, a1587) {
                            let a1592 = {
                                let a1589 = match v4.clone() {
                                    Adt2::C0(_, h1588, _, _, _, _, _, _, _, _, _) => {
                                        h1588
                                    }
                                };
                                let a1590 = v6;
                                let a1591 = 1u64;
                                nat_rem(a1589, a1590, a1591)
                            };
                            let a1593 = 0u64;
                            nat_eq(a1592, a1593)
                        } else {
                            false
                        } {
                            let a1598 = {
                                let a1595 = match v3.clone() {
                                    Adt2::C0(_, h1594, _, _, _, _, _, _, _, _, _) => {
                                        h1594
                                    }
                                };
                                let a1596 = v6;
                                let a1597 = 1u64;
                                nat_rem(a1595, a1596, a1597)
                            };
                            let a1599 = 0u64;
                            nat_eq(a1598, a1599)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1600) => {
                                    h1600
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

pub fn f55(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1602 = match v2.clone() {
        Adt2::C0(h1601, _, _, _, _, _, _, _, _, _, _) => {
            h1601
        }
    };
    let a1603 = 0u64;
    if if nat_eq(a1602, a1603) {
        let a1605 = match v3.clone() {
            Adt2::C0(h1604, _, _, _, _, _, _, _, _, _, _) => {
                h1604
            }
        };
        let a1606 = 0u64;
        nat_eq(a1605, a1606)
    } else {
        false
    } {
        let a1607 = v0;
        let a1608 = 61u64;
        if nat_eq(a1607, a1608) {
            let v4: bool = {
                let a1610 = 0u64;
                let a1611 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1609, _, _, _) => {
                        h1609
                    }
                };
                nat_lt(a1610, a1611)
            };
            if if if if if if f119(match v2.clone() {
                Adt2::C0(_, h1612, _, _, _, _, _, _, _, _, _) => {
                    h1612
                }
            }, match v1.clone() {
                Adt1::C0(h1613, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1613
                }
            }, match v1.clone() {
                Adt1::C0(_, h1614, _, _, _, _, _, _, _, _, _, _) => {
                    h1614
                }
            }, match v3.clone() {
                Adt2::C0(_, h1615, _, _, _, _, _, _, _, _, _) => {
                    h1615
                }
            })? {
                f119(match v2.clone() {
                    Adt2::C0(_, _, h1616, _, _, _, _, _, _, _, _) => {
                        h1616
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1617, _, _, _, _, _, _, _, _, _) => {
                        h1617
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1618, _, _, _, _, _, _, _, _) => {
                        h1618
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1619, _, _, _, _, _, _, _, _) => {
                        h1619
                    }
                })?
            } else {
                false
            } {
                f119(match v2.clone() {
                    Adt2::C0(_, _, _, h1620, _, _, _, _, _, _, _) => {
                        h1620
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1621, _, _, _, _, _, _, _) => {
                        h1621
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1622, _, _, _, _, _, _) => {
                        h1622
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1623, _, _, _, _, _, _, _) => {
                        h1623
                    }
                })?
            } else {
                false
            } {
                f119(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1624, _, _, _, _, _, _) => {
                        h1624
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1625, _, _, _, _, _) => {
                        h1625
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1626, _, _, _, _) => {
                        h1626
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1627, _, _, _, _, _, _) => {
                        h1627
                    }
                })?
            } else {
                false
            } {
                let a1629 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1628, _, _, _) => {
                        h1628
                    }
                };
                let a1630 = 4294967295u64;
                nat_le(a1629, a1630)
            } else {
                false
            } {
                if v4 {
                    !f34(match v2.clone() {
                        Adt2::C0(_, h1631, _, _, _, _, _, _, _, _, _) => {
                            h1631
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1632, _, _, _, _, _, _, _, _) => {
                            h1632
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1633, _, _, _, _, _, _, _) => {
                            h1633
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1634, _, _, _, _, _, _) => {
                            h1634
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1635, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1635
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1636, _, _, _, _, _, _, _, _, _) => {
                        h1636
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1637, _, _, _, _, _, _, _) => {
                        h1637
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1638, _, _, _, _, _) => {
                        h1638
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1639 = v0;
            let a1640 = 62u64;
            if nat_eq(a1639, a1640) {
                let a1642 = match v1.clone() {
                    Adt1::C0(h1641, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1641
                    }
                };
                let a1643 = 2147483648u64;
                if if if if if nat_lt(a1642, a1643) {
                    let a1645 = match v1.clone() {
                        Adt1::C0(_, h1644, _, _, _, _, _, _, _, _, _, _) => {
                            h1644
                        }
                    };
                    let a1646 = 2147483648u64;
                    nat_lt(a1645, a1646)
                } else {
                    false
                } {
                    let a1649 = match v1.clone() {
                        Adt1::C0(h1647, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1647
                        }
                    };
                    let a1650 = match v2.clone() {
                        Adt2::C0(_, h1648, _, _, _, _, _, _, _, _, _) => {
                            h1648
                        }
                    };
                    nat_lt(a1649, a1650)
                } else {
                    false
                } {
                    let a1653 = match v1.clone() {
                        Adt1::C0(_, h1651, _, _, _, _, _, _, _, _, _, _) => {
                            h1651
                        }
                    };
                    let a1654 = match v2.clone() {
                        Adt2::C0(_, h1652, _, _, _, _, _, _, _, _, _) => {
                            h1652
                        }
                    };
                    nat_lt(a1653, a1654)
                } else {
                    false
                } {
                    let a1657 = match v2.clone() {
                        Adt2::C0(_, _, h1655, _, _, _, _, _, _, _, _) => {
                            h1655
                        }
                    };
                    let a1658 = match v3.clone() {
                        Adt2::C0(_, _, h1656, _, _, _, _, _, _, _, _) => {
                            h1656
                        }
                    };
                    if if if nat_eq(a1657, a1658) {
                        let a1661 = match v2.clone() {
                            Adt2::C0(_, _, _, h1659, _, _, _, _, _, _, _) => {
                                h1659
                            }
                        };
                        let a1662 = match v3.clone() {
                            Adt2::C0(_, _, _, h1660, _, _, _, _, _, _, _) => {
                                h1660
                            }
                        };
                        nat_eq(a1661, a1662)
                    } else {
                        false
                    } {
                        let a1665 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1663, _, _, _, _, _, _) => {
                                h1663
                            }
                        };
                        let a1666 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1664, _, _, _, _, _, _) => {
                                h1664
                            }
                        };
                        nat_eq(a1665, a1666)
                    } else {
                        false
                    } {
                        let a1675 = match v3.clone() {
                            Adt2::C0(_, h1667, _, _, _, _, _, _, _, _, _) => {
                                h1667
                            }
                        };
                        let a1676 = {
                            let a1673 = {
                                let a1670 = match v2.clone() {
                                    Adt2::C0(_, h1668, _, _, _, _, _, _, _, _, _) => {
                                        h1668
                                    }
                                };
                                let a1671 = match v1.clone() {
                                    Adt1::C0(h1669, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1669
                                    }
                                };
                                nat_add(a1670, a1671)?
                            };
                            let a1674 = match v1.clone() {
                                Adt1::C0(_, h1672, _, _, _, _, _, _, _, _, _, _) => {
                                    h1672
                                }
                            };
                            nat_add(a1673, a1674)?
                        };
                        nat_eq(a1675, a1676)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1677, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1677
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1678 = v0;
                let a1679 = 63u64;
                if nat_eq(a1678, a1679) {
                    if if if if if f120(match v2.clone() {
                        Adt2::C0(_, h1680, _, _, _, _, _, _, _, _, _) => {
                            h1680
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1681, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1681
                        }
                    })? {
                        f120(match v2.clone() {
                            Adt2::C0(_, _, h1682, _, _, _, _, _, _, _, _) => {
                                h1682
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1683, _, _, _, _, _, _, _, _, _, _) => {
                                h1683
                            }
                        })?
                    } else {
                        false
                    } {
                        f120(match v2.clone() {
                            Adt2::C0(_, _, _, h1684, _, _, _, _, _, _, _) => {
                                h1684
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1685, _, _, _, _, _, _, _, _, _) => {
                                h1685
                            }
                        })?
                    } else {
                        false
                    } {
                        f120(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1686, _, _, _, _, _, _) => {
                                h1686
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1687, _, _, _, _, _, _, _, _) => {
                                h1687
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1690 = match v2.clone() {
                            Adt2::C0(_, h1688, _, _, _, _, _, _, _, _, _) => {
                                h1688
                            }
                        };
                        let a1691 = match v3.clone() {
                            Adt2::C0(_, h1689, _, _, _, _, _, _, _, _, _) => {
                                h1689
                            }
                        };
                        if if if nat_eq(a1690, a1691) {
                            let a1694 = match v2.clone() {
                                Adt2::C0(_, _, h1692, _, _, _, _, _, _, _, _) => {
                                    h1692
                                }
                            };
                            let a1695 = match v3.clone() {
                                Adt2::C0(_, _, h1693, _, _, _, _, _, _, _, _) => {
                                    h1693
                                }
                            };
                            nat_eq(a1694, a1695)
                        } else {
                            false
                        } {
                            let a1698 = match v2.clone() {
                                Adt2::C0(_, _, _, h1696, _, _, _, _, _, _, _) => {
                                    h1696
                                }
                            };
                            let a1699 = match v3.clone() {
                                Adt2::C0(_, _, _, h1697, _, _, _, _, _, _, _) => {
                                    h1697
                                }
                            };
                            nat_eq(a1698, a1699)
                        } else {
                            false
                        } {
                            let a1702 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1700, _, _, _, _, _, _) => {
                                    h1700
                                }
                            };
                            let a1703 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1701, _, _, _, _, _, _) => {
                                    h1701
                                }
                            };
                            nat_eq(a1702, a1703)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1704, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1704
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1705, _, _, _, _, _, _, _, _, _, _) => {
                                h1705
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1706, _, _, _, _, _, _, _, _, _) => {
                                h1706
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1707, _, _, _, _, _, _, _, _) => {
                                h1707
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

pub fn f56(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1708 = v3;
    match m1708 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1709 = v0;
                let a1710 = 4u64;
                nat_eq(a1709, a1710)
            };
            let v7: bool = {
                let a1712 = 0u64;
                let a1713 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1711, _, _, _, _, _, _, _) => {
                        h1711
                    }
                };
                nat_lt(a1712, a1713)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1714, _, _, _, _, _, _, _, _, _) => {
                    h1714
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1715, _, _, _, _, _, _, _, _) => {
                    h1715
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1716, _, _, _, _, _, _, _) => {
                    h1716
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1717, _, _, _, _, _, _) => {
                    h1717
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1718, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1718
                }
            }, match v1.clone() {
                Adt1::C0(_, h1719, _, _, _, _, _, _, _, _, _, _) => {
                    h1719
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1720, _, _, _, _, _, _, _, _, _) => {
                    h1720
                }
            });
            if if if if if if if if if if if if v6 {
                let a1722 = match v2.clone() {
                    Adt2::C0(h1721, _, _, _, _, _, _, _, _, _, _) => {
                        h1721
                    }
                };
                let a1723 = 0u64;
                if nat_eq(a1722, a1723) {
                    let a1725 = match v5.clone() {
                        Adt2::C0(h1724, _, _, _, _, _, _, _, _, _, _) => {
                            h1724
                        }
                    };
                    let a1726 = 0u64;
                    nat_eq(a1725, a1726)
                } else {
                    false
                }
            } else {
                let a1728 = match v2.clone() {
                    Adt2::C0(h1727, _, _, _, _, _, _, _, _, _, _) => {
                        h1727
                    }
                };
                let a1729 = 0u64;
                if if nat_eq(a1728, a1729) {
                    true
                } else {
                    let a1731 = match v2.clone() {
                        Adt2::C0(h1730, _, _, _, _, _, _, _, _, _, _) => {
                            h1730
                        }
                    };
                    let a1732 = 26u64;
                    nat_eq(a1731, a1732)
                } {
                    let a1734 = match v5.clone() {
                        Adt2::C0(h1733, _, _, _, _, _, _, _, _, _, _) => {
                            h1733
                        }
                    };
                    let a1735 = 0u64;
                    if nat_eq(a1734, a1735) {
                        true
                    } else {
                        let a1737 = match v5.clone() {
                            Adt2::C0(h1736, _, _, _, _, _, _, _, _, _, _) => {
                                h1736
                            }
                        };
                        let a1738 = 26u64;
                        nat_eq(a1737, a1738)
                    }
                } else {
                    false
                }
            } {
                let a1741 = match v2.clone() {
                    Adt2::C0(h1739, _, _, _, _, _, _, _, _, _, _) => {
                        h1739
                    }
                };
                let a1742 = match v4.clone() {
                    Adt2::C0(h1740, _, _, _, _, _, _, _, _, _, _) => {
                        h1740
                    }
                };
                nat_eq(a1741, a1742)
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
                let a1744 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1743, _, _, _, _, _) => {
                        h1743
                    }
                };
                let a1745 = 4u64;
                nat_eq(a1744, a1745)
            } else {
                false
            } {
                let a1746 = v8;
                let a1747 = f16(v2.clone(), 1u64)?;
                nat_le(a1746, a1747)
            } else {
                false
            } {
                let a1749 = match v1.clone() {
                    Adt1::C0(_, _, _, h1748, _, _, _, _, _, _, _, _) => {
                        h1748
                    }
                };
                let a1750 = 1073741824u64;
                nat_lt(a1749, a1750)
            } else {
                false
            } {
                let a1752 = match v1.clone() {
                    Adt1::C0(h1751, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1751
                    }
                };
                let a1753 = 4294967295u64;
                if if if if nat_le(a1752, a1753) {
                    let a1755 = match v1.clone() {
                        Adt1::C0(_, h1754, _, _, _, _, _, _, _, _, _, _) => {
                            h1754
                        }
                    };
                    let a1756 = 4294967295u64;
                    nat_le(a1755, a1756)
                } else {
                    false
                } {
                    let a1758 = match v1.clone() {
                        Adt1::C0(_, _, h1757, _, _, _, _, _, _, _, _, _) => {
                            h1757
                        }
                    };
                    let a1759 = 4294967295u64;
                    nat_le(a1758, a1759)
                } else {
                    false
                } {
                    let a1761 = match v1.clone() {
                        Adt1::C0(_, _, _, h1760, _, _, _, _, _, _, _, _) => {
                            h1760
                        }
                    };
                    let a1762 = 4294967295u64;
                    nat_le(a1761, a1762)
                } else {
                    false
                } {
                    let a1764 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1763, _, _, _, _, _, _, _) => {
                            h1763
                        }
                    };
                    let a1765 = 4294967295u64;
                    nat_le(a1764, a1765)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1768 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1766) => {
                            h1766
                        }
                    };
                    let a1769 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1767) => {
                            h1767
                        }
                    };
                    nat_eq(a1768, a1769)
                } else {
                    true
                }
            } else {
                false
            } {
                f121(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1770, _, _, _, _, _, _, _, _) => {
                        h1770
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1771, _) => {
                        h1771
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
                    let a1772 = v10;
                    let a1773 = v8;
                    nat_add(a1772, a1773)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1774, _, _, _, _, _, _, _, _) => {
                        h1774
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f57(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1776 = match v0.clone() {
        Adt2::C0(h1775, _, _, _, _, _, _, _, _, _, _) => {
            h1775
        }
    };
    let a1777 = 0u64;
    if if if nat_eq(a1776, a1777) {
        let a1779 = match v1.clone() {
            Adt2::C0(h1778, _, _, _, _, _, _, _, _, _, _) => {
                h1778
            }
        };
        let a1780 = 0u64;
        nat_eq(a1779, a1780)
    } else {
        false
    } {
        f117(match v0.clone() {
            Adt2::C0(_, h1781, _, _, _, _, _, _, _, _, _) => {
                h1781
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1782, _, _, _, _, _, _, _, _) => {
                h1782
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1783, _, _, _, _, _, _, _) => {
                h1783
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1784, _, _, _, _, _, _) => {
                h1784
            }
        }, match v1.clone() {
            Adt2::C0(_, h1785, _, _, _, _, _, _, _, _, _) => {
                h1785
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1786, _, _, _, _, _, _, _, _) => {
                h1786
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1787, _, _, _, _, _, _, _) => {
                h1787
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1788, _, _, _, _, _, _) => {
                h1788
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1789 = v2;
            let a1790 = f16(v0.clone(), 1u64)?;
            nat_add(a1789, a1790)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f58(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1792 = match v1.clone() {
        Adt2::C0(h1791, _, _, _, _, _, _, _, _, _, _) => {
            h1791
        }
    };
    let a1793 = 0u64;
    if if if nat_eq(a1792, a1793) {
        true
    } else {
        let a1795 = match v1.clone() {
            Adt2::C0(h1794, _, _, _, _, _, _, _, _, _, _) => {
                h1794
            }
        };
        let a1796 = 1u64;
        nat_eq(a1795, a1796)
    } {
        true
    } else {
        let a1798 = match v1.clone() {
            Adt2::C0(h1797, _, _, _, _, _, _, _, _, _, _) => {
                h1797
            }
        };
        let a1799 = 30u64;
        nat_eq(a1798, a1799)
    } {
        let a1802 = match v1.clone() {
            Adt2::C0(h1800, _, _, _, _, _, _, _, _, _, _) => {
                h1800
            }
        };
        let a1803 = match v2.clone() {
            Adt2::C0(h1801, _, _, _, _, _, _, _, _, _, _) => {
                h1801
            }
        };
        if if if if if nat_eq(a1802, a1803) {
            let a1806 = match v1.clone() {
                Adt2::C0(_, h1804, _, _, _, _, _, _, _, _, _) => {
                    h1804
                }
            };
            let a1807 = match v2.clone() {
                Adt2::C0(_, h1805, _, _, _, _, _, _, _, _, _) => {
                    h1805
                }
            };
            nat_eq(a1806, a1807)
        } else {
            false
        } {
            let a1810 = match v1.clone() {
                Adt2::C0(_, _, _, h1808, _, _, _, _, _, _, _) => {
                    h1808
                }
            };
            let a1811 = match v2.clone() {
                Adt2::C0(_, _, _, h1809, _, _, _, _, _, _, _) => {
                    h1809
                }
            };
            nat_eq(a1810, a1811)
        } else {
            false
        } {
            let a1814 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1812, _, _, _, _, _, _) => {
                    h1812
                }
            };
            let a1815 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1813, _, _, _, _, _, _) => {
                    h1813
                }
            };
            nat_eq(a1814, a1815)
        } else {
            false
        } {
            let a1816 = v0;
            let a1817 = 4u64;
            if nat_eq(a1816, a1817) {
                let a1819 = match v1.clone() {
                    Adt2::C0(_, _, h1818, _, _, _, _, _, _, _, _) => {
                        h1818
                    }
                };
                let a1820 = 1u64;
                if nat_eq(a1819, a1820) {
                    let a1823 = match v1.clone() {
                        Adt2::C0(_, h1821, _, _, _, _, _, _, _, _, _) => {
                            h1821
                        }
                    };
                    let a1824 = match v2.clone() {
                        Adt2::C0(_, _, h1822, _, _, _, _, _, _, _, _) => {
                            h1822
                        }
                    };
                    nat_eq(a1823, a1824)
                } else {
                    false
                }
            } else {
                let a1825 = v0;
                let a1826 = 4u64;
                if if nat_lt(a1825, a1826) {
                    let a1829 = match v1.clone() {
                        Adt2::C0(_, h1827, _, _, _, _, _, _, _, _, _) => {
                            h1827
                        }
                    };
                    let a1830 = match v1.clone() {
                        Adt2::C0(_, _, h1828, _, _, _, _, _, _, _, _) => {
                            h1828
                        }
                    };
                    nat_eq(a1829, a1830)
                } else {
                    false
                } {
                    let a1833 = match v1.clone() {
                        Adt2::C0(_, _, h1831, _, _, _, _, _, _, _, _) => {
                            h1831
                        }
                    };
                    let a1834 = match v2.clone() {
                        Adt2::C0(_, _, h1832, _, _, _, _, _, _, _, _) => {
                            h1832
                        }
                    };
                    nat_eq(a1833, a1834)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1836 = f114(match v1.clone() {
                Adt2::C0(h1835, _, _, _, _, _, _, _, _, _, _) => {
                    h1835
                }
            });
            match m1836 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1837) => {
                            h1837
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

pub fn f59(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f122(match v1.clone() {
        Adt2::C0(h1838, _, _, _, _, _, _, _, _, _, _) => {
            h1838
        }
    }, v0) {
        let a1841 = match v1.clone() {
            Adt2::C0(h1839, _, _, _, _, _, _, _, _, _, _) => {
                h1839
            }
        };
        let a1842 = match v2.clone() {
            Adt2::C0(h1840, _, _, _, _, _, _, _, _, _, _) => {
                h1840
            }
        };
        nat_eq(a1841, a1842)
    } else {
        false
    } {
        let a1845 = match v1.clone() {
            Adt2::C0(_, h1843, _, _, _, _, _, _, _, _, _) => {
                h1843
            }
        };
        let a1846 = match v2.clone() {
            Adt2::C0(_, h1844, _, _, _, _, _, _, _, _, _) => {
                h1844
            }
        };
        if if if nat_eq(a1845, a1846) {
            let a1849 = match v1.clone() {
                Adt2::C0(_, _, h1847, _, _, _, _, _, _, _, _) => {
                    h1847
                }
            };
            let a1850 = match v2.clone() {
                Adt2::C0(_, _, h1848, _, _, _, _, _, _, _, _) => {
                    h1848
                }
            };
            nat_eq(a1849, a1850)
        } else {
            false
        } {
            let a1853 = match v1.clone() {
                Adt2::C0(_, _, _, h1851, _, _, _, _, _, _, _) => {
                    h1851
                }
            };
            let a1854 = match v2.clone() {
                Adt2::C0(_, _, _, h1852, _, _, _, _, _, _, _) => {
                    h1852
                }
            };
            nat_eq(a1853, a1854)
        } else {
            false
        } {
            let a1857 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1855, _, _, _, _, _, _) => {
                    h1855
                }
            };
            let a1858 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1856, _, _, _, _, _, _) => {
                    h1856
                }
            };
            nat_eq(a1857, a1858)
        } else {
            false
        }
    } else {
        false
    } {
        let m1860 = f114(match v1.clone() {
            Adt2::C0(h1859, _, _, _, _, _, _, _, _, _, _) => {
                h1859
            }
        });
        match m1860 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1861) => {
                        h1861
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1862, _, _, _, _, _, _, _, _, _, _) => {
                        h1862
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f60(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1863, _, _, _, _, _, _, _, _, _, _, _) => {
                h1863
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1864, _, _, _, _, _, _, _, _, _) => {
                h1864
            }
        }
    };
    let a1865 = 0u64;
    let a1866 = v4;
    if if nat_lt(a1865, a1866) {
        let a1867 = v4;
        let a1868 = 2147483648u64;
        nat_lt(a1867, a1868)
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
        let v7: u64 = f123(match v5.clone() {
            Adt2::C0(_, _, h1869, _, _, _, _, _, _, _, _) => {
                h1869
            }
        }, v4)?;
        let v8: u64 = f123(match v5.clone() {
            Adt2::C0(_, _, _, h1870, _, _, _, _, _, _, _) => {
                h1870
            }
        }, v4)?;
        let a1872 = match v5.clone() {
            Adt2::C0(h1871, _, _, _, _, _, _, _, _, _, _) => {
                h1871
            }
        };
        let a1873 = 0u64;
        if if if if if nat_eq(a1872, a1873) {
            let a1875 = match v6.clone() {
                Adt2::C0(h1874, _, _, _, _, _, _, _, _, _, _) => {
                    h1874
                }
            };
            let a1876 = 0u64;
            nat_eq(a1875, a1876)
        } else {
            false
        } {
            let a1878 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1877, _, _, _, _, _, _) => {
                    h1877
                }
            };
            let a1879 = 1u64;
            nat_eq(a1878, a1879)
        } else {
            false
        } {
            let a1882 = match v5.clone() {
                Adt2::C0(_, h1880, _, _, _, _, _, _, _, _, _) => {
                    h1880
                }
            };
            let a1883 = match v6.clone() {
                Adt2::C0(_, h1881, _, _, _, _, _, _, _, _, _) => {
                    h1881
                }
            };
            nat_eq(a1882, a1883)
        } else {
            false
        } {
            if v0 {
                let a1885 = match v5.clone() {
                    Adt2::C0(_, _, h1884, _, _, _, _, _, _, _, _) => {
                        h1884
                    }
                };
                let a1886 = 2147483648u64;
                if if if nat_lt(a1885, a1886) {
                    let a1888 = match v5.clone() {
                        Adt2::C0(_, _, _, h1887, _, _, _, _, _, _, _) => {
                            h1887
                        }
                    };
                    let a1889 = 2147483648u64;
                    nat_lt(a1888, a1889)
                } else {
                    false
                } {
                    let a1892 = {
                        let a1890 = v7;
                        let a1891 = v8;
                        nat_mul(a1890, a1891)?
                    };
                    let a1893 = 2147483648u64;
                    nat_le(a1892, a1893)
                } else {
                    false
                } {
                    f124(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1895 = match v1.clone() {
                    Adt1::C0(h1894, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1894
                    }
                };
                let a1896 = v7;
                if if if if nat_eq(a1895, a1896) {
                    let a1898 = match v1 {
                        Adt1::C0(_, h1897, _, _, _, _, _, _, _, _, _, _) => {
                            h1897
                        }
                    };
                    let a1899 = v8;
                    nat_eq(a1898, a1899)
                } else {
                    false
                } {
                    f125(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1901 = match v6.clone() {
                        Adt2::C0(_, _, h1900, _, _, _, _, _, _, _, _) => {
                            h1900
                        }
                    };
                    let a1902 = v4;
                    if nat_eq(a1901, a1902) {
                        let a1904 = match v6.clone() {
                            Adt2::C0(_, _, _, h1903, _, _, _, _, _, _, _) => {
                                h1903
                            }
                        };
                        let a1905 = v4;
                        nat_eq(a1904, a1905)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1909 = match v6 {
                        Adt2::C0(_, _, _, _, h1906, _, _, _, _, _, _) => {
                            h1906
                        }
                    };
                    let a1910 = {
                        let a1907 = v7;
                        let a1908 = v8;
                        nat_mul(a1907, a1908)?
                    };
                    nat_eq(a1909, a1910)
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

pub fn f61(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1912 = match v2.clone() {
        Adt2::C0(h1911, _, _, _, _, _, _, _, _, _, _) => {
            h1911
        }
    };
    let a1913 = 0u64;
    if if if if if nat_eq(a1912, a1913) {
        true
    } else {
        let a1915 = match v2.clone() {
            Adt2::C0(h1914, _, _, _, _, _, _, _, _, _, _) => {
                h1914
            }
        };
        let a1916 = 1u64;
        nat_eq(a1915, a1916)
    } {
        true
    } else {
        let a1918 = match v2.clone() {
            Adt2::C0(h1917, _, _, _, _, _, _, _, _, _, _) => {
                h1917
            }
        };
        let a1919 = 30u64;
        nat_eq(a1918, a1919)
    } {
        let a1922 = match v2.clone() {
            Adt2::C0(h1920, _, _, _, _, _, _, _, _, _, _) => {
                h1920
            }
        };
        let a1923 = match v3.clone() {
            Adt2::C0(h1921, _, _, _, _, _, _, _, _, _, _) => {
                h1921
            }
        };
        nat_eq(a1922, a1923)
    } else {
        false
    } {
        if if f25(v2.clone(), v3.clone()) {
            let a1925 = match v1.clone() {
                Adt1::C0(h1924, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1924
                }
            };
            let a1926 = 4294967296u64;
            nat_lt(a1925, a1926)
        } else {
            false
        } {
            let a1928 = match v1.clone() {
                Adt1::C0(_, h1927, _, _, _, _, _, _, _, _, _, _) => {
                    h1927
                }
            };
            let a1929 = 4294967296u64;
            nat_lt(a1928, a1929)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a1931 = match v2.clone() {
                Adt2::C0(h1930, _, _, _, _, _, _, _, _, _, _) => {
                    h1930
                }
            };
            let a1932 = 0u64;
            if nat_eq(a1931, a1932) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a1933 = v0;
            let a1934 = 31u64;
            if nat_eq(a1933, a1934) {
                0u64
            } else {
                let a1935 = v0;
                let a1936 = 47u64;
                if nat_eq(a1935, a1936) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1937, _, _, _, _, _, _, _, _, _, _) => {
                h1937
            }
        }, match v1.clone() {
            Adt1::C0(h1938, _, _, _, _, _, _, _, _, _, _, _) => {
                h1938
            }
        }, match v1 {
            Adt1::C0(_, h1939, _, _, _, _, _, _, _, _, _, _) => {
                h1939
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f62(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1941 = match v0.clone() {
        Adt2::C0(h1940, _, _, _, _, _, _, _, _, _, _) => {
            h1940
        }
    };
    let a1942 = 0u64;
    if if if nat_eq(a1941, a1942) {
        let a1944 = match v1.clone() {
            Adt2::C0(h1943, _, _, _, _, _, _, _, _, _, _) => {
                h1943
            }
        };
        let a1945 = 0u64;
        nat_eq(a1944, a1945)
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

pub fn f63(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1947 = match v0.clone() {
        Adt2::C0(h1946, _, _, _, _, _, _, _, _, _, _) => {
            h1946
        }
    };
    let a1948 = 0u64;
    if if nat_eq(a1947, a1948) {
        let a1950 = match v1.clone() {
            Adt2::C0(h1949, _, _, _, _, _, _, _, _, _, _) => {
                h1949
            }
        };
        let a1951 = 26u64;
        nat_eq(a1950, a1951)
    } else {
        false
    } {
        let a1953 = match v0.clone() {
            Adt2::C0(_, _, _, h1952, _, _, _, _, _, _, _) => {
                h1952
            }
        };
        let a1954 = 1u64;
        if if if if if if if nat_eq(a1953, a1954) {
            let a1956 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1955, _, _, _, _, _, _) => {
                    h1955
                }
            };
            let a1957 = 1u64;
            nat_eq(a1956, a1957)
        } else {
            false
        } {
            let a1959 = match v0.clone() {
                Adt2::C0(_, h1958, _, _, _, _, _, _, _, _, _) => {
                    h1958
                }
            };
            let a1960 = 2147483647u64;
            nat_le(a1959, a1960)
        } else {
            false
        } {
            let a1963 = match v1.clone() {
                Adt2::C0(_, h1961, _, _, _, _, _, _, _, _, _) => {
                    h1961
                }
            };
            let a1964 = match v0.clone() {
                Adt2::C0(_, _, h1962, _, _, _, _, _, _, _, _) => {
                    h1962
                }
            };
            nat_eq(a1963, a1964)
        } else {
            false
        } {
            let a1966 = match v1.clone() {
                Adt2::C0(_, _, h1965, _, _, _, _, _, _, _, _) => {
                    h1965
                }
            };
            let a1967 = 1u64;
            nat_eq(a1966, a1967)
        } else {
            false
        } {
            let a1969 = match v1.clone() {
                Adt2::C0(_, _, _, h1968, _, _, _, _, _, _, _) => {
                    h1968
                }
            };
            let a1970 = 1u64;
            nat_eq(a1969, a1970)
        } else {
            false
        } {
            let a1972 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1971, _, _, _, _, _, _) => {
                    h1971
                }
            };
            let a1973 = 1u64;
            nat_eq(a1972, a1973)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1976 = match v0.clone() {
                    Adt2::C0(_, _, h1974, _, _, _, _, _, _, _, _) => {
                        h1974
                    }
                };
                let a1977 = f126(match v0.clone() {
                    Adt2::C0(_, h1975, _, _, _, _, _, _, _, _, _) => {
                        h1975
                    }
                })?;
                nat_mul(a1976, a1977)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f64(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1978 = v1;
    match m1978 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f127(f20(v0.clone(), 1u64))?;
            let a1980 = match v0.clone() {
                Adt2::C0(h1979, _, _, _, _, _, _, _, _, _, _) => {
                    h1979
                }
            };
            let a1981 = 26u64;
            if if if nat_eq(a1980, a1981) {
                let a1983 = match v3.clone() {
                    Adt2::C0(h1982, _, _, _, _, _, _, _, _, _, _) => {
                        h1982
                    }
                };
                let a1984 = 26u64;
                nat_eq(a1983, a1984)
            } else {
                false
            } {
                let a1986 = match v2.clone() {
                    Adt2::C0(h1985, _, _, _, _, _, _, _, _, _, _) => {
                        h1985
                    }
                };
                let a1987 = 27u64;
                nat_eq(a1986, a1987)
            } else {
                false
            } {
                if if if if if if f25(v0.clone(), v3.clone()) {
                    let a1989 = match v2.clone() {
                        Adt2::C0(_, h1988, _, _, _, _, _, _, _, _, _) => {
                            h1988
                        }
                    };
                    let a1990 = 1u64;
                    nat_eq(a1989, a1990)
                } else {
                    false
                } {
                    let a1992 = match v2.clone() {
                        Adt2::C0(_, _, h1991, _, _, _, _, _, _, _, _) => {
                            h1991
                        }
                    };
                    let a1993 = 1u64;
                    nat_eq(a1992, a1993)
                } else {
                    false
                } {
                    let a1995 = match v2.clone() {
                        Adt2::C0(_, _, _, h1994, _, _, _, _, _, _, _) => {
                            h1994
                        }
                    };
                    let a1996 = 1u64;
                    nat_eq(a1995, a1996)
                } else {
                    false
                } {
                    let a1998 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1997, _, _, _, _, _, _) => {
                            h1997
                        }
                    };
                    let a1999 = 1u64;
                    nat_eq(a1998, a1999)
                } else {
                    false
                } {
                    let a2000 = v4;
                    let a2001 = 9223372036854775808u64;
                    nat_lt(a2000, a2001)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2004 = 2u64;
                        let a2005 = {
                            let a2002 = v4;
                            let a2003 = 2u64;
                            nat_mul(a2002, a2003)?
                        };
                        nat_add(a2004, a2005)?
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

pub fn f65(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h2006, _, _, _, _, _, _, _, _, _, _) => {
            h2006
        }
    };
    let a2007 = v3;
    let a2008 = 0u64;
    if if nat_eq(a2007, a2008) {
        true
    } else {
        let a2009 = v0;
        let a2010 = 13u64;
        if nat_eq(a2009, a2010) {
            let a2011 = v3;
            let a2012 = 1u64;
            if nat_eq(a2011, a2012) {
                true
            } else {
                let a2013 = v3;
                let a2014 = 30u64;
                nat_eq(a2013, a2014)
            }
        } else {
            false
        }
    } {
        let a2016 = v3;
        let a2017 = match v2.clone() {
            Adt2::C0(h2015, _, _, _, _, _, _, _, _, _, _) => {
                h2015
            }
        };
        if if nat_eq(a2016, a2017) {
            let a2018 = v0;
            let a2019 = 13u64;
            if nat_eq(a2018, a2019) {
                let a2021 = match v2.clone() {
                    Adt2::C0(_, h2020, _, _, _, _, _, _, _, _, _) => {
                        h2020
                    }
                };
                let a2022 = 1u64;
                if if if nat_eq(a2021, a2022) {
                    let a2024 = match v2.clone() {
                        Adt2::C0(_, _, h2023, _, _, _, _, _, _, _, _) => {
                            h2023
                        }
                    };
                    let a2025 = 1u64;
                    nat_eq(a2024, a2025)
                } else {
                    false
                } {
                    let a2027 = match v2.clone() {
                        Adt2::C0(_, _, _, h2026, _, _, _, _, _, _, _) => {
                            h2026
                        }
                    };
                    let a2028 = 1u64;
                    nat_eq(a2027, a2028)
                } else {
                    false
                } {
                    let a2030 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2029, _, _, _, _, _, _) => {
                            h2029
                        }
                    };
                    let a2031 = 1u64;
                    nat_eq(a2030, a2031)
                } else {
                    false
                }
            } else {
                let a2033 = match v2.clone() {
                    Adt2::C0(_, h2032, _, _, _, _, _, _, _, _, _) => {
                        h2032
                    }
                };
                let a2034 = 1u64;
                if if if nat_eq(a2033, a2034) {
                    let a2037 = match v1.clone() {
                        Adt2::C0(_, _, h2035, _, _, _, _, _, _, _, _) => {
                            h2035
                        }
                    };
                    let a2038 = match v2.clone() {
                        Adt2::C0(_, _, h2036, _, _, _, _, _, _, _, _) => {
                            h2036
                        }
                    };
                    nat_eq(a2037, a2038)
                } else {
                    false
                } {
                    let a2041 = match v1.clone() {
                        Adt2::C0(_, _, _, h2039, _, _, _, _, _, _, _) => {
                            h2039
                        }
                    };
                    let a2042 = match v2.clone() {
                        Adt2::C0(_, _, _, h2040, _, _, _, _, _, _, _) => {
                            h2040
                        }
                    };
                    nat_eq(a2041, a2042)
                } else {
                    false
                } {
                    let a2045 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2043, _, _, _, _, _, _) => {
                            h2043
                        }
                    };
                    let a2046 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2044, _, _, _, _, _, _) => {
                            h2044
                        }
                    };
                    nat_eq(a2045, a2046)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f128(v4.clone(), v0, v3)?, {
                let a2047 = v3;
                let a2048 = 0u64;
                if nat_eq(a2047, a2048) {
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

pub fn f66(v0: u64, v1: u64) -> R<Option<u64>> {
    let a2049 = v0;
    let a2050 = 89u64;
    if if nat_eq(a2049, a2050) {
        let a2051 = v1;
        let a2052 = 14u64;
        nat_eq(a2051, a2052)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a2053 = v0;
        let a2054 = 89u64;
        if if nat_eq(a2053, a2054) {
            let a2055 = v1;
            let a2056 = 15u64;
            nat_eq(a2055, a2056)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a2057 = v0;
            let a2058 = 89u64;
            if if nat_eq(a2057, a2058) {
                let a2059 = 8u64;
                let a2060 = v1;
                if nat_le(a2059, a2060) {
                    let a2061 = v1;
                    let a2062 = 9u64;
                    nat_le(a2061, a2062)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a2063 = v1;
                    let a2064 = 12u64;
                    nat_add(a2063, a2064)?
                }))
            } else {
                let a2065 = v0;
                let a2066 = 89u64;
                if if nat_eq(a2065, a2066) {
                    let a2067 = v1;
                    let a2068 = 4u64;
                    nat_eq(a2067, a2068)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a2069 = 10u64;
                    let a2070 = v0;
                    if if nat_le(a2069, a2070) {
                        let a2071 = v0;
                        let a2072 = 12u64;
                        nat_le(a2071, a2072)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a2073 = v0;
                            let a2074 = 3u64;
                            nat_add(a2073, a2074)?
                        }))
                    } else {
                        let a2075 = v0;
                        let a2076 = 89u64;
                        if if nat_eq(a2075, a2076) {
                            let a2077 = v1;
                            let a2078 = 7u64;
                            nat_eq(a2077, a2078)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a2079 = v0;
                            let a2080 = 89u64;
                            if if nat_eq(a2079, a2080) {
                                let a2081 = v1;
                                let a2082 = 10u64;
                                nat_eq(a2081, a2082)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a2083 = v0;
                                let a2084 = 89u64;
                                if if nat_eq(a2083, a2084) {
                                    let a2085 = v1;
                                    let a2086 = 13u64;
                                    nat_eq(a2085, a2086)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a2087 = v0;
                                    let a2088 = 8u64;
                                    if nat_eq(a2087, a2088) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a2089 = v0;
                                        let a2090 = 9u64;
                                        if nat_eq(a2089, a2090) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a2091 = v0;
                                            let a2092 = 89u64;
                                            if nat_eq(a2091, a2092) {
                                                let a2093 = v1;
                                                let a2094 = 0u64;
                                                if nat_eq(a2093, a2094) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a2095 = v1;
                                                    let a2096 = 1u64;
                                                    if nat_eq(a2095, a2096) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a2097 = v1;
                                                        let a2098 = 2u64;
                                                        if nat_eq(a2097, a2098) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a2099 = v1;
                                                            let a2100 = 3u64;
                                                            if nat_eq(a2099, a2100) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a2101 = v1;
                                                                let a2102 = 6u64;
                                                                if nat_eq(a2101, a2102) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a2103 = v1;
                                                                    let a2104 = 12u64;
                                                                    if nat_eq(a2103, a2104) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a2105 = v1;
                                                                        let a2106 = 11u64;
                                                                        if nat_eq(a2105, a2106) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a2107 = v1;
                                                                            let a2108 = 18u64;
                                                                            if nat_eq(a2107, a2108) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a2109 = v1;
                                                                                let a2110 = 19u64;
                                                                                if nat_eq(a2109, a2110) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a2111 = v1;
                                                                                    let a2112 = 20u64;
                                                                                    if nat_eq(a2111, a2112) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a2113 = v1;
                                                                                        let a2114 = 21u64;
                                                                                        if nat_eq(a2113, a2114) {
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

pub fn f67(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2115 = f129(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m2115 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a2116 = v0;
            let a2117 = 2u64;
            if if if if nat_eq(a2116, a2117) {
                true
            } else {
                let a2118 = v0;
                let a2119 = 5u64;
                nat_eq(a2118, a2119)
            } {
                true
            } else {
                let a2120 = v0;
                let a2121 = 6u64;
                nat_eq(a2120, a2121)
            } {
                true
            } else {
                let a2122 = v0;
                let a2123 = 7u64;
                nat_eq(a2122, a2123)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2124 = v3;
                    match m2124 {
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

pub fn f68(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a2126 = match v1.clone() {
        Adt2::C0(h2125, _, _, _, _, _, _, _, _, _, _) => {
            h2125
        }
    };
    let a2127 = 0u64;
    if if if if if nat_eq(a2126, a2127) {
        true
    } else {
        let a2129 = match v1.clone() {
            Adt2::C0(h2128, _, _, _, _, _, _, _, _, _, _) => {
                h2128
            }
        };
        let a2130 = 1u64;
        nat_eq(a2129, a2130)
    } {
        true
    } else {
        let a2132 = match v1.clone() {
            Adt2::C0(h2131, _, _, _, _, _, _, _, _, _, _) => {
                h2131
            }
        };
        let a2133 = 30u64;
        nat_eq(a2132, a2133)
    } {
        let a2136 = match v1.clone() {
            Adt2::C0(h2134, _, _, _, _, _, _, _, _, _, _) => {
                h2134
            }
        };
        let a2137 = match v2.clone() {
            Adt2::C0(h2135, _, _, _, _, _, _, _, _, _, _) => {
                h2135
            }
        };
        if nat_eq(a2136, a2137) {
            true
        } else {
            let a2139 = match v2.clone() {
                Adt2::C0(h2138, _, _, _, _, _, _, _, _, _, _) => {
                    h2138
                }
            };
            let a2140 = 0u64;
            nat_eq(a2139, a2140)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2142 = f16(v2.clone(), 1u64)?;
            let a2143 = f130(v0, match v2.clone() {
                Adt2::C0(h2141, _, _, _, _, _, _, _, _, _, _) => {
                    h2141
                }
            });
            nat_mul(a2142, a2143)?
        }, {
            let a2145 = match v2.clone() {
                Adt2::C0(h2144, _, _, _, _, _, _, _, _, _, _) => {
                    h2144
                }
            };
            let a2146 = 0u64;
            if nat_eq(a2145, a2146) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2147, _, _, _, _, _, _, _, _, _, _) => {
                h2147
            }
        }, match v2 {
            Adt2::C0(h2148, _, _, _, _, _, _, _, _, _, _) => {
                h2148
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f69(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f131(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2149, _, _, _) => {
            h2149
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2150, _, _) => {
            h2150
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2151, _) => {
            h2151
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2152) => {
            h2152
        }
    })
}

pub fn f70(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2154 = v0;
        let a2155 = match v1.clone() {
            Adt3::C0(h2153, _, _, _, _, _, _, _) => {
                h2153
            }
        };
        let a2156 = 0u64;
        nat_quot(a2154, a2155, a2156)
    };
    let v4: u64 = {
        let a2158 = v3;
        let a2159 = match v1.clone() {
            Adt3::C0(_, h2157, _, _, _, _, _, _) => {
                h2157
            }
        };
        let a2160 = 0u64;
        nat_quot(a2158, a2159, a2160)
    };
    let a2161 = v2;
    let a2162 = 0u64;
    if nat_eq(a2161, a2162) {
        let a2164 = v0;
        let a2165 = match v1 {
            Adt3::C0(h2163, _, _, _, _, _, _, _) => {
                h2163
            }
        };
        let a2166 = 0u64;
        nat_rem(a2164, a2165, a2166)
    } else {
        let a2167 = v2;
        let a2168 = 1u64;
        if nat_eq(a2167, a2168) {
            let a2170 = v3;
            let a2171 = match v1 {
                Adt3::C0(_, h2169, _, _, _, _, _, _) => {
                    h2169
                }
            };
            let a2172 = 0u64;
            nat_rem(a2170, a2171, a2172)
        } else {
            let a2173 = v2;
            let a2174 = 2u64;
            if nat_eq(a2173, a2174) {
                let a2176 = v4;
                let a2177 = match v1 {
                    Adt3::C0(_, _, h2175, _, _, _, _, _) => {
                        h2175
                    }
                };
                let a2178 = 0u64;
                nat_rem(a2176, a2177, a2178)
            } else {
                let a2180 = v4;
                let a2181 = match v1 {
                    Adt3::C0(_, _, h2179, _, _, _, _, _) => {
                        h2179
                    }
                };
                let a2182 = 0u64;
                nat_quot(a2180, a2181, a2182)
            }
        }
    }
}

pub fn f71(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2184 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2183) => {
            h2183
        }
    };
    let a2185 = 0u64;
    if nat_eq(a2184, a2185) {
        Ok(v2)
    } else {
        let a2190 = {
            let a2187 = v2;
            let a2188 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2186) => {
                    h2186
                }
            };
            nat_mul(a2187, a2188)?
        };
        let a2191 = f70(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2189, _, _, _, _, _, _) => {
                h2189
            }
        }, 3u64);
        nat_add(a2190, a2191)
    }
}

pub fn f72(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2201 = {
            let a2198 = {
                let a2193 = v2;
                let a2194 = match v0.clone() {
                    Adt6::C0(_, h2192, _, _) => {
                        h2192
                    }
                };
                nat_mul(a2193, a2194)?
            };
            let a2199 = {
                let a2196 = v3;
                let a2197 = match v0.clone() {
                    Adt6::C0(_, _, _, h2195) => {
                        h2195
                    }
                };
                nat_mul(a2196, a2197)?
            };
            nat_add(a2198, a2199)?
        };
        let a2202 = f132(match v0.clone() {
            Adt6::C0(_, _, h2200, _) => {
                h2200
            }
        });
        nat_add(a2201, a2202)?
    };
    let v5: u64 = f133(match v0 {
        Adt6::C0(_, _, h2203, _) => {
            h2203
        }
    });
    let a2204 = v5;
    let a2205 = v4;
    if if nat_le(a2204, a2205) {
        let a2208 = {
            let a2206 = v4;
            let a2207 = v5;
            nat_sub(a2206, a2207)
        };
        let a2209 = v1;
        nat_lt(a2208, a2209)
    } else {
        false
    } {
        Ok(Some({
            let a2210 = v4;
            let a2211 = v5;
            nat_sub(a2210, a2211)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f73(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2212, _, _, _, _, _) => {
            h2212
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2213, _, _, _, _, _, _) => {
            h2213
        }
    };
    let v7: u64 = f70(v1, v6.clone(), 2u64);
    let a2215 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2214) => {
            h2214
        }
    };
    let a2216 = 0u64;
    if nat_eq(a2215, a2216) {
        Ok(Adt9::C8(Some(f69(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2219 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2218, _, _, _, _) => {
                    h2218
                }
            };
            match m2219 {
                Adt3::C0(_, _, h2217, _, _, _, _, _) => {
                    h2217
                }
            }
        };
        let m2225 = f72(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2220, _) => {
                h2220
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2221, _, _, _, _, _) => {
                h2221
            }
        }, {
            let a2222 = v2;
            let a2223 = v8;
            let a2224 = 0u64;
            nat_rem(a2222, a2223, a2224)
        }, v7)?;
        match m2225 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2232 = {
                        let a2230 = {
                            let a2226 = v2;
                            let a2227 = v8;
                            let a2228 = 0u64;
                            nat_quot(a2226, a2227, a2228)
                        };
                        let a2231 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2229) => {
                                h2229
                            }
                        };
                        nat_mul(a2230, a2231)?
                    };
                    let a2233 = f70(v1, v6, 3u64);
                    nat_add(a2232, a2233)?
                };
                Ok(Adt9::C8(Some(f69(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f74(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2234 = v1;
    match m2234 {
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
            f134(f135(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f136(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f137(v0.clone(), v24);
            f134(f138(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m2235 = v3;
    match m2235 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f139(v0, v1)?, f139(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f141(v0, v1, v2.clone())?, f139(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f142(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f143(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f144(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f143(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f145(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f146(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f147(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f139(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f148(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v30, v31) => {
            f149(v0, v1, v2, v30, v31)
        }
        Adt4::C12(v32, v33, v34) => {
            f150(v0, v1, v2, v32, v33, v34)
        }
        Adt4::C13(_, v36, _, _) => {
            Ok(Adt9::C3(v36, v36, f139(v0, v1)?, f139(v0, v2)?))
        }
        Adt4::C14(v39, v40, v41) => {
            let a2238 = v39;
            let a2239 = 23u64;
            if nat_eq(a2238, a2239) {
                f151(v0, v1, v2, v40, v41)
            } else {
                Ok(Adt9::C3(v40, v41, f139(v0, v1)?, f139(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f152(v0, v1, v2)
        }
        Adt4::C16(v42, v43) => {
            f153(v0, v1, v2, v42, v43)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v44, v45) => {
            f154(v0, v1, v45, v44)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f139(f140(v0, match v2.clone() {
                Adt3::C0(h2236, _, _, _, _, _, _, _) => {
                    h2236
                }
            }, match v2.clone() {
                Adt3::C0(_, h2237, _, _, _, _, _, _) => {
                    h2237
                }
            })?, v1)?, f139(v0, v2)?))
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
        Adt4::C25(v50, v51, v52) => {
            f155(v0, v50, v51, v52)
        }
        Adt4::C26(v58, _, _, _) => {
            let a2240 = v58;
            let a2241 = 64u64;
            if if if nat_eq(a2240, a2241) {
                true
            } else {
                let a2242 = v58;
                let a2243 = 65u64;
                nat_eq(a2242, a2243)
            } {
                true
            } else {
                let a2244 = v58;
                let a2245 = 66u64;
                nat_eq(a2244, a2245)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v57) => {
            f84(v0, v1, v2, v57)
        }
        Adt4::C28(v53) => {
            f81(v0, v1, v2, v53, 0u64)
        }
        Adt4::C29(v54) => {
            f82(v0, v1, v2, v54)
        }
        Adt4::C30(v55, _) => {
            f83(v0, v1, v2, v55)
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f116(v3, v4);
    let v6: u64 = {
        let a2246 = v0;
        let a2247 = v5;
        let a2248 = 0u64;
        nat_quot(a2246, a2247, a2248)
    };
    let a2249 = v5;
    let a2250 = 1u64;
    if nat_eq(a2249, a2250) {
        Ok(f134(f156(v3, v4), v3, v4, 0u64, f139(v6, v1)?, 2u64, 0u64, 3u64, f139(v6, v2)?))
    } else {
        let a2254 = {
            let a2251 = v0;
            let a2252 = 2u64;
            let a2253 = 0u64;
            nat_rem(a2251, a2252, a2253)
        };
        let a2255 = 0u64;
        if nat_eq(a2254, a2255) {
            Ok(f134(f156(v3, 0u64), v3, 0u64, 0u64, f139(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f134(f156(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f139(v6, v2)?))
        }
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f126(match v1.clone() {
        Adt3::C0(h2256, _, _, _, _, _, _, _) => {
            h2256
        }
    })?;
    let v5: u64 = {
        let a2257 = v0;
        let a2258 = v4;
        let a2259 = 0u64;
        nat_quot(a2257, a2258, a2259)
    };
    let v6: u64 = {
        let a2260 = v0;
        let a2261 = v4;
        let a2262 = 0u64;
        nat_rem(a2260, a2261, a2262)
    };
    let a2263 = v6;
    let a2264 = 0u64;
    if nat_eq(a2263, a2264) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2265 = v6;
        let a2266 = 1u64;
        if nat_eq(a2265, a2266) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2269 = {
                let a2267 = v6;
                let a2268 = 1u64;
                nat_add(a2267, a2268)?
            };
            let a2270 = v4;
            if nat_eq(a2269, a2270) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f139(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2271 = v6;
                    let a2272 = 2u64;
                    nat_sub(a2271, a2272)
                };
                let v8: u64 = {
                    let a2273 = v7;
                    let a2274 = 3u64;
                    let a2275 = 0u64;
                    nat_quot(a2273, a2274, a2275)
                };
                let v9: u64 = {
                    let a2276 = v7;
                    let a2277 = 3u64;
                    let a2278 = 0u64;
                    nat_rem(a2276, a2277, a2278)
                };
                let v10: u64 = f139({
                    let a2282 = {
                        let a2280 = v5;
                        let a2281 = match v1.clone() {
                            Adt3::C0(h2279, _, _, _, _, _, _, _) => {
                                h2279
                            }
                        };
                        nat_mul(a2280, a2281)?
                    };
                    let a2283 = v8;
                    nat_add(a2282, a2283)?
                }, v1)?;
                let a2284 = v9;
                let a2285 = 0u64;
                if nat_eq(a2284, a2285) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2286 = v9;
                    let a2287 = 1u64;
                    if nat_eq(a2286, a2287) {
                        let a2288 = v3;
                        let a2289 = 2u64;
                        if nat_eq(a2288, a2289) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2290 = v3;
                        let a2291 = 2u64;
                        if if nat_eq(a2290, a2291) {
                            true
                        } else {
                            let a2292 = v3;
                            let a2293 = 3u64;
                            nat_eq(a2292, a2293)
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

pub fn f78(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2295 = 2u64;
        let a2296 = match v2.clone() {
            Adt3::C0(_, h2294, _, _, _, _, _, _) => {
                h2294
            }
        };
        nat_mul(a2295, a2296)?
    };
    let a2297 = v1;
    let a2298 = v6;
    if nat_lt(a2297, a2298) {
        let a2302 = {
            let a2299 = v1;
            let a2300 = 2u64;
            let a2301 = 0u64;
            nat_rem(a2299, a2300, a2301)
        };
        let a2303 = 0u64;
        if nat_eq(a2302, a2303) {
            Ok(Adt9::C4(19u64, 4u64, f139({
                let a2304 = v1;
                let a2305 = 2u64;
                let a2306 = 0u64;
                nat_quot(a2304, a2305, a2306)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2307 = v5;
            let a2308 = 2147483648u64;
            if if nat_lt(a2307, a2308) {
                let a2310 = v5;
                let a2311 = match v3 {
                    Adt3::C0(_, h2309, _, _, _, _, _, _) => {
                        h2309
                    }
                };
                nat_lt(a2310, a2311)
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
            let a2314 = match v3.clone() {
                Adt3::C0(h2312, _, _, _, _, _, _, _) => {
                    h2312
                }
            };
            let a2315 = match v3.clone() {
                Adt3::C0(_, h2313, _, _, _, _, _, _) => {
                    h2313
                }
            };
            nat_mul(a2314, a2315)?
        };
        let a2318 = {
            let a2316 = v1;
            let a2317 = v6;
            nat_sub(a2316, a2317)
        };
        let a2319 = v7;
        if nat_lt(a2318, a2319) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f139({
                let a2320 = v1;
                let a2321 = v6;
                nat_sub(a2320, a2321)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2324 = {
                    let a2322 = v1;
                    let a2323 = v6;
                    nat_sub(a2322, a2323)
                };
                let a2325 = v7;
                nat_sub(a2324, a2325)
            };
            let v9: u64 = {
                let a2327 = 1u64;
                let a2328 = f157(v0, match v2.clone() {
                    Adt3::C0(h2326, _, _, _, _, _, _, _) => {
                        h2326
                    }
                })?;
                nat_add(a2327, a2328)?
            };
            let v10: u64 = {
                let a2329 = v8;
                let a2330 = v9;
                let a2331 = 0u64;
                nat_quot(a2329, a2330, a2331)
            };
            let v11: u64 = {
                let a2332 = v8;
                let a2333 = v9;
                let a2334 = 0u64;
                nat_rem(a2332, a2333, a2334)
            };
            let a2335 = v11;
            let a2336 = 0u64;
            if nat_eq(a2335, a2336) {
                Ok(Adt9::C4(19u64, 4u64, f139(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f104(v0) {
                    let a2340 = {
                        let a2337 = v11;
                        let a2338 = 2u64;
                        let a2339 = 0u64;
                        nat_rem(a2337, a2338, a2339)
                    };
                    let a2341 = 1u64;
                    if nat_eq(a2340, a2341) {
                        Ok(Adt9::C4(6u64, 0u64, f139({
                            let a2350 = {
                                let a2343 = v10;
                                let a2344 = match v2.clone() {
                                    Adt3::C0(h2342, _, _, _, _, _, _, _) => {
                                        h2342
                                    }
                                };
                                nat_mul(a2343, a2344)?
                            };
                            let a2351 = {
                                let a2347 = {
                                    let a2345 = v11;
                                    let a2346 = 1u64;
                                    nat_sub(a2345, a2346)
                                };
                                let a2348 = 2u64;
                                let a2349 = 0u64;
                                nat_quot(a2347, a2348, a2349)
                            };
                            nat_add(a2350, a2351)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f139({
                            let a2360 = {
                                let a2353 = v5;
                                let a2354 = match v2.clone() {
                                    Adt3::C0(h2352, _, _, _, _, _, _, _) => {
                                        h2352
                                    }
                                };
                                nat_mul(a2353, a2354)?
                            };
                            let a2361 = {
                                let a2357 = {
                                    let a2355 = v11;
                                    let a2356 = 1u64;
                                    nat_sub(a2355, a2356)
                                };
                                let a2358 = 2u64;
                                let a2359 = 0u64;
                                nat_quot(a2357, a2358, a2359)
                            };
                            nat_add(a2360, a2361)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f139({
                            let a2370 = {
                                let a2363 = v5;
                                let a2364 = match v2 {
                                    Adt3::C0(h2362, _, _, _, _, _, _, _) => {
                                        h2362
                                    }
                                };
                                nat_mul(a2363, a2364)?
                            };
                            let a2371 = {
                                let a2367 = {
                                    let a2365 = v11;
                                    let a2366 = 1u64;
                                    nat_sub(a2365, a2366)
                                };
                                let a2368 = 2u64;
                                let a2369 = 0u64;
                                nat_quot(a2367, a2368, a2369)
                            };
                            nat_add(a2370, a2371)?
                        }, v3)?))
                    }
                } else {
                    f158(v0, v10, {
                        let a2372 = v11;
                        let a2373 = 1u64;
                        nat_sub(a2372, a2373)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f79(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2374 = v1;
    let a2375 = f159(v2.clone(), v4.clone())?;
    if nat_lt(a2374, a2375) {
        let a2378 = v1;
        let a2379 = {
            let a2376 = 2u64;
            let a2377 = f127(v4.clone())?;
            nat_mul(a2376, a2377)?
        };
        if nat_lt(a2378, a2379) {
            let a2383 = {
                let a2380 = v1;
                let a2381 = 2u64;
                let a2382 = 0u64;
                nat_rem(a2380, a2381, a2382)
            };
            let a2384 = 0u64;
            if nat_eq(a2383, a2384) {
                f160({
                    let a2385 = v1;
                    let a2386 = 2u64;
                    let a2387 = 0u64;
                    nat_quot(a2385, a2386, a2387)
                }, v4)
            } else {
                if f161(match v2 {
                    Adt3::C0(_, h2388, _, _, _, _, _, _) => {
                        h2388
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2391 = v1;
                let a2392 = {
                    let a2389 = 2u64;
                    let a2390 = f127(v4.clone())?;
                    nat_mul(a2389, a2390)?
                };
                nat_sub(a2391, a2392)
            };
            let v7: u64 = {
                let a2396 = v6;
                let a2397 = {
                    let a2394 = 1u64;
                    let a2395 = match v2.clone() {
                        Adt3::C0(h2393, _, _, _, _, _, _, _) => {
                            h2393
                        }
                    };
                    nat_add(a2394, a2395)?
                };
                let a2398 = 0u64;
                nat_quot(a2396, a2397, a2398)
            };
            let v8: u64 = {
                let a2402 = v6;
                let a2403 = {
                    let a2400 = 1u64;
                    let a2401 = match v2.clone() {
                        Adt3::C0(h2399, _, _, _, _, _, _, _) => {
                            h2399
                        }
                    };
                    nat_add(a2400, a2401)?
                };
                let a2404 = 0u64;
                nat_rem(a2402, a2403, a2404)
            };
            let a2405 = v8;
            let a2406 = 0u64;
            if nat_eq(a2405, a2406) {
                f160(v7, v4)
            } else {
                f162(v0, v7, {
                    let a2407 = v8;
                    let a2408 = 1u64;
                    nat_sub(a2407, a2408)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2411 = match v4.clone() {
            Adt3::C0(h2409, _, _, _, _, _, _, _) => {
                h2409
            }
        };
        let a2412 = match v4.clone() {
            Adt3::C0(_, h2410, _, _, _, _, _, _) => {
                h2410
            }
        };
        nat_mul(a2411, a2412)?
    };
    let v7: u64 = {
        let a2413 = 2u64;
        let a2414 = v6;
        nat_mul(a2413, a2414)?
    };
    let v8: u64 = f163(match v3.clone() {
        Adt3::C0(_, h2415, _, _, _, _, _, _) => {
            h2415
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2416, _, _, _, _, _) => {
            h2416
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2417, _, _, _, _) => {
            h2417
        }
    });
    let a2418 = v0;
    let a2419 = v7;
    if nat_lt(a2418, a2419) {
        let a2423 = {
            let a2420 = v0;
            let a2421 = 2u64;
            let a2422 = 0u64;
            nat_rem(a2420, a2421, a2422)
        };
        let a2424 = 0u64;
        if nat_eq(a2423, a2424) {
            Ok(Adt9::C4(19u64, 5u64, f139({
                let a2425 = v0;
                let a2426 = 2u64;
                let a2427 = 0u64;
                nat_quot(a2425, a2426, a2427)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f164(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2428 = v0;
            let a2429 = v7;
            nat_sub(a2428, a2429)
        };
        let v10: u64 = {
            let a2433 = v9;
            let a2434 = {
                let a2431 = 1u64;
                let a2432 = match v1.clone() {
                    Adt3::C0(h2430, _, _, _, _, _, _, _) => {
                        h2430
                    }
                };
                nat_add(a2431, a2432)?
            };
            let a2435 = 0u64;
            nat_quot(a2433, a2434, a2435)
        };
        let v11: u64 = {
            let a2439 = v9;
            let a2440 = {
                let a2437 = 1u64;
                let a2438 = match v1.clone() {
                    Adt3::C0(h2436, _, _, _, _, _, _, _) => {
                        h2436
                    }
                };
                nat_add(a2437, a2438)?
            };
            let a2441 = 0u64;
            nat_rem(a2439, a2440, a2441)
        };
        let a2442 = v11;
        let a2443 = 0u64;
        if nat_eq(a2442, a2443) {
            Ok(Adt9::C4(19u64, 5u64, f139({
                let a2444 = v10;
                let a2445 = v6;
                let a2446 = 0u64;
                nat_rem(a2444, a2445, a2446)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f164(v5, v8) {
                let v12: u64 = {
                    let a2452 = {
                        let a2448 = v10;
                        let a2449 = match v1.clone() {
                            Adt3::C0(h2447, _, _, _, _, _, _, _) => {
                                h2447
                            }
                        };
                        nat_mul(a2448, a2449)?
                    };
                    let a2453 = {
                        let a2450 = v11;
                        let a2451 = 1u64;
                        nat_sub(a2450, a2451)
                    };
                    nat_add(a2452, a2453)?
                };
                let a2454 = v5;
                let a2455 = 4294967295u64;
                if nat_eq(a2454, a2455) {
                    Ok(Adt9::C1(false, f139(v12, v1)?, f139(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f139(v12, v1)?, 4u64, f139({
                        let a2461 = {
                            let a2457 = v5;
                            let a2458 = match v3.clone() {
                                Adt3::C0(h2456, _, _, _, _, _, _, _) => {
                                    h2456
                                }
                            };
                            nat_mul(a2457, a2458)?
                        };
                        let a2462 = {
                            let a2459 = v11;
                            let a2460 = 1u64;
                            nat_sub(a2459, a2460)
                        };
                        nat_add(a2461, a2462)?
                    }, v3)?, 3u64, f139(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f109(v3.clone())?;
    let v6: u64 = {
        let a2463 = v0;
        let a2464 = v5;
        let a2465 = 0u64;
        nat_quot(a2463, a2464, a2465)
    };
    let v7: u64 = {
        let a2466 = v0;
        let a2467 = v5;
        let a2468 = 0u64;
        nat_rem(a2466, a2467, a2468)
    };
    let v8: u64 = f165(v3.clone())?;
    let a2469 = v7;
    let a2470 = 0u64;
    if nat_eq(a2469, a2470) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2471, _) => {
                h2471
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2475 = v5;
        let a2476 = {
            let a2473 = v7;
            let a2474 = if match v3.clone() {
                Adt7::C0(_, _, _, h2472, _) => {
                    h2472
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2473, a2474)?
        };
        if nat_le(a2475, a2476) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2477, _) => {
                    h2477
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f139(v6, v2)?))
            } else {
                let a2480 = {
                    let a2478 = v7;
                    let a2479 = 2u64;
                    nat_add(a2478, a2479)?
                };
                let a2481 = v5;
                if nat_eq(a2480, a2481) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2482, _, _) => {
                            h2482
                        }
                    } {
                        let a2489 = {
                            let m2485 = match v3.clone() {
                                Adt7::C0(h2484, _, _, _, _) => {
                                    h2484
                                }
                            };
                            match m2485 {
                                Adt6::C0(h2483, _, _, _) => {
                                    h2483
                                }
                            }
                        };
                        let a2490 = {
                            let m2488 = match v3 {
                                Adt7::C0(_, h2487, _, _, _) => {
                                    h2487
                                }
                            };
                            match m2488 {
                                Adt6::C0(h2486, _, _, _) => {
                                    h2486
                                }
                            }
                        };
                        nat_mul(a2489, a2490)?
                    } else {
                        f166(match v3 {
                            Adt7::C0(h2491, _, _, _, _) => {
                                h2491
                            }
                        }, match v1 {
                            Adt3::C0(h2492, _, _, _, _, _, _, _) => {
                                h2492
                            }
                        }, f70(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2493 = v9;
                        let a2494 = 0u64;
                        if nat_eq(a2493, a2494) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f139(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2497 = {
                    let a2495 = v7;
                    let a2496 = 1u64;
                    nat_sub(a2495, a2496)
                };
                let a2498 = v8;
                let a2499 = 0u64;
                nat_rem(a2497, a2498, a2499)
            };
            let m2505 = f167(v3.clone(), v1, v2, v6, {
                let a2502 = {
                    let a2500 = v7;
                    let a2501 = 1u64;
                    nat_sub(a2500, a2501)
                };
                let a2503 = v8;
                let a2504 = 0u64;
                nat_quot(a2502, a2503, a2504)
            })?;
            match m2505 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2507 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2506) => {
                            h2506
                        }
                    };
                    let a2508 = 1u64;
                    if if nat_eq(a2507, a2508) {
                        let a2509 = v10;
                        let a2510 = 0u64;
                        nat_eq(a2509, a2510)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2511, _) => {
                                h2511
                            }
                        } {
                            let a2514 = {
                                let a2512 = v10;
                                let a2513 = 1u64;
                                nat_add(a2512, a2513)?
                            };
                            let a2515 = v8;
                            if nat_eq(a2514, a2515) {
                                let a2516 = v4;
                                let a2517 = 2u64;
                                if nat_eq(a2516, a2517) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2519 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2518) => {
                                                h2518
                                            }
                                        };
                                        let a2520 = 1u64;
                                        if nat_eq(a2519, a2520) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2522 = match v3 {
                                            Adt7::C0(_, _, _, _, h2521) => {
                                                h2521
                                            }
                                        };
                                        let a2523 = 1u64;
                                        if nat_eq(a2522, a2523) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2525 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2524) => {
                                            h2524
                                        }
                                    };
                                    let a2526 = 1u64;
                                    if nat_eq(a2525, a2526) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2528 = match v3 {
                                        Adt7::C0(_, _, _, _, h2527) => {
                                            h2527
                                        }
                                    };
                                    let a2529 = 1u64;
                                    if nat_eq(a2528, a2529) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2531 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2530) => {
                                        h2530
                                    }
                                };
                                let a2532 = 1u64;
                                if nat_eq(a2531, a2532) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2534 = match v3 {
                                    Adt7::C0(_, _, _, _, h2533) => {
                                        h2533
                                    }
                                };
                                let a2535 = 1u64;
                                if nat_eq(a2534, a2535) {
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

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f107(match v3.clone() {
        Adt3::C0(h2536, _, _, _, _, _, _, _) => {
            h2536
        }
    })?;
    let v5: u64 = {
        let a2537 = v0;
        let a2538 = v4;
        let a2539 = 0u64;
        nat_rem(a2537, a2538, a2539)
    };
    let v6: u64 = {
        let a2540 = v0;
        let a2541 = v4;
        let a2542 = 0u64;
        nat_quot(a2540, a2541, a2542)
    };
    let a2543 = v5;
    let a2544 = 0u64;
    if nat_eq(a2543, a2544) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2547 = {
            let a2545 = v5;
            let a2546 = 1u64;
            nat_add(a2545, a2546)?
        };
        let a2548 = v4;
        if nat_eq(a2547, a2548) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f139(v6, v2)?))
        } else {
            let a2552 = {
                let a2549 = v5;
                let a2550 = 2u64;
                let a2551 = 0u64;
                nat_rem(a2549, a2550, a2551)
            };
            let a2553 = 0u64;
            if nat_eq(a2552, a2553) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2556 = {
                        let a2554 = v5;
                        let a2555 = 1u64;
                        nat_sub(a2554, a2555)
                    };
                    let a2557 = 2u64;
                    let a2558 = 0u64;
                    nat_quot(a2556, a2557, a2558)
                };
                let v8: u64 = f70(v6, v2.clone(), 0u64);
                let v9: u64 = f70(v6, v2.clone(), 1u64);
                let v10: u64 = f70(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f69(v1, {
                    let a2559 = v9;
                    let a2560 = v7;
                    nat_add(a2559, a2560)?
                }, v8, v10, 0u64)?, 4u64, f69(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f106(v3.clone())?;
    let v5: u64 = {
        let a2561 = v0;
        let a2562 = v4;
        let a2563 = 0u64;
        nat_rem(a2561, a2562, a2563)
    };
    let v6: u64 = {
        let a2564 = v0;
        let a2565 = v4;
        let a2566 = 0u64;
        nat_quot(a2564, a2565, a2566)
    };
    let a2567 = v5;
    let a2568 = 0u64;
    if nat_eq(a2567, a2568) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2571 = {
            let a2569 = v5;
            let a2570 = 1u64;
            nat_add(a2569, a2570)?
        };
        let a2572 = v4;
        if nat_eq(a2571, a2572) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f139(v6, v2)?))
        } else {
            let a2576 = {
                let a2573 = v5;
                let a2574 = 2u64;
                let a2575 = 0u64;
                nat_rem(a2573, a2574, a2575)
            };
            let a2577 = 0u64;
            if nat_eq(a2576, a2577) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2579 = f70(v6, v2.clone(), 0u64);
                    let a2580 = match v3.clone() {
                        Adt6::C0(_, _, h2578, _) => {
                            h2578
                        }
                    };
                    nat_add(a2579, a2580)?
                };
                let v8: u64 = {
                    let a2588 = {
                        let a2585 = {
                            let a2582 = v7;
                            let a2583 = match v3.clone() {
                                Adt6::C0(_, h2581, _, _) => {
                                    h2581
                                }
                            };
                            nat_add(a2582, a2583)?
                        };
                        let a2586 = match v3.clone() {
                            Adt6::C0(h2584, _, _, _) => {
                                h2584
                            }
                        };
                        nat_sub(a2585, a2586)
                    };
                    let a2589 = match v3.clone() {
                        Adt6::C0(_, h2587, _, _) => {
                            h2587
                        }
                    };
                    let a2590 = 0u64;
                    nat_quot(a2588, a2589, a2590)
                };
                let v9: u64 = {
                    let a2596 = v8;
                    let a2597 = {
                        let a2593 = {
                            let a2591 = v5;
                            let a2592 = 1u64;
                            nat_sub(a2591, a2592)
                        };
                        let a2594 = 2u64;
                        let a2595 = 0u64;
                        nat_quot(a2593, a2594, a2595)
                    };
                    nat_add(a2596, a2597)?
                };
                let v10: u64 = {
                    let a2599 = v9;
                    let a2600 = match v3.clone() {
                        Adt6::C0(_, h2598, _, _) => {
                            h2598
                        }
                    };
                    nat_mul(a2599, a2600)?
                };
                let v11: u64 = {
                    let a2601 = v7;
                    let a2602 = v10;
                    nat_sub(a2601, a2602)
                };
                let a2604 = v9;
                let a2605 = match v1.clone() {
                    Adt3::C0(_, h2603, _, _, _, _, _, _) => {
                        h2603
                    }
                };
                if if if nat_lt(a2604, a2605) {
                    let a2606 = v10;
                    let a2607 = v7;
                    nat_le(a2606, a2607)
                } else {
                    false
                } {
                    let a2609 = v11;
                    let a2610 = match v3.clone() {
                        Adt6::C0(h2608, _, _, _) => {
                            h2608
                        }
                    };
                    nat_lt(a2609, a2610)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f69(v1, {
                        let a2614 = {
                            let a2612 = f70(v6, v2, 1u64);
                            let a2613 = match v3 {
                                Adt6::C0(h2611, _, _, _) => {
                                    h2611
                                }
                            };
                            nat_mul(a2612, a2613)?
                        };
                        let a2615 = v11;
                        nat_add(a2614, a2615)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f70(v0, v2.clone(), 0u64);
    let v5: u64 = f70(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2617 = v5;
        let a2618 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2616, _) => {
                h2616
            }
        };
        let a2619 = 0u64;
        nat_rem(a2617, a2618, a2619)
    };
    let v7: u64 = {
        let a2621 = v5;
        let a2622 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2620, _) => {
                h2620
            }
        };
        let a2623 = 0u64;
        nat_quot(a2621, a2622, a2623)
    };
    let v8: Option<u64> = f72(match v3.clone() {
        Adt5::C0(h2624, _, _, _, _, _, _, _) => {
            h2624
        }
    }, match v1.clone() {
        Adt3::C0(h2625, _, _, _, _, _, _, _) => {
            h2625
        }
    }, f70(v0, v2.clone(), 1u64), {
        let a2629 = v4;
        let a2630 = {
            let m2628 = match v3.clone() {
                Adt5::C0(h2627, _, _, _, _, _, _, _) => {
                    h2627
                }
            };
            match m2628 {
                Adt6::C0(h2626, _, _, _) => {
                    h2626
                }
            }
        };
        let a2631 = 0u64;
        nat_rem(a2629, a2630, a2631)
    })?;
    let v9: u64 = {
        let a2635 = v4;
        let a2636 = {
            let m2634 = match v3.clone() {
                Adt5::C0(h2633, _, _, _, _, _, _, _) => {
                    h2633
                }
            };
            match m2634 {
                Adt6::C0(h2632, _, _, _) => {
                    h2632
                }
            }
        };
        let a2637 = 0u64;
        nat_quot(a2635, a2636, a2637)
    };
    let v10: Option<u64> = f72(match v3.clone() {
        Adt5::C0(_, h2638, _, _, _, _, _, _) => {
            h2638
        }
    }, match v1.clone() {
        Adt3::C0(_, h2639, _, _, _, _, _, _) => {
            h2639
        }
    }, f70(v0, v2.clone(), 2u64), {
        let a2643 = v9;
        let a2644 = {
            let m2642 = match v3.clone() {
                Adt5::C0(_, h2641, _, _, _, _, _, _) => {
                    h2641
                }
            };
            match m2642 {
                Adt6::C0(h2640, _, _, _) => {
                    h2640
                }
            }
        };
        let a2645 = 0u64;
        nat_rem(a2643, a2644, a2645)
    })?;
    let v11: u64 = {
        let a2649 = v9;
        let a2650 = {
            let m2648 = match v3.clone() {
                Adt5::C0(_, h2647, _, _, _, _, _, _) => {
                    h2647
                }
            };
            match m2648 {
                Adt6::C0(h2646, _, _, _) => {
                    h2646
                }
            }
        };
        let a2651 = 0u64;
        nat_quot(a2649, a2650, a2651)
    };
    let v12: Option<u64> = f72(match v3.clone() {
        Adt5::C0(_, _, h2652, _, _, _, _, _) => {
            h2652
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2653, _, _, _, _, _) => {
            h2653
        }
    }, v6, {
        let a2657 = v11;
        let a2658 = {
            let m2656 = match v3.clone() {
                Adt5::C0(_, _, h2655, _, _, _, _, _) => {
                    h2655
                }
            };
            match m2656 {
                Adt6::C0(h2654, _, _, _) => {
                    h2654
                }
            }
        };
        let a2659 = 0u64;
        nat_rem(a2657, a2658, a2659)
    })?;
    let v13: u64 = {
        let a2663 = v11;
        let a2664 = {
            let m2662 = match v3.clone() {
                Adt5::C0(_, _, h2661, _, _, _, _, _) => {
                    h2661
                }
            };
            match m2662 {
                Adt6::C0(h2660, _, _, _) => {
                    h2660
                }
            }
        };
        let a2665 = 0u64;
        nat_quot(a2663, a2664, a2665)
    };
    let v17: Adt12 = {
        let m2666 = v8;
        match m2666 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2667 = v10;
                match m2667 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2668 = v12;
                        match m2668 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2677 = {
                                        let a2672 = f69(v1, v14, v15, v16, 0u64)?;
                                        let a2673 = {
                                            let a2670 = v7;
                                            let a2671 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2669, _, _, _) => {
                                                    h2669
                                                }
                                            };
                                            nat_mul(a2670, a2671)?
                                        };
                                        nat_add(a2672, a2673)?
                                    };
                                    let a2678 = {
                                        let a2675 = v13;
                                        let a2676 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2674, _, _, _, _) => {
                                                h2674
                                            }
                                        };
                                        nat_mul(a2675, a2676)?
                                    };
                                    nat_add(a2677, a2678)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2679, _, _) => {
            h2679
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2680) => {
            h2680
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f139(v0, v2)?), {
        let a2682 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2681, _, _) => {
                h2681
            }
        };
        let a2683 = 0u64;
        if nat_eq(a2682, a2683) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2685 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2684) => {
                h2684
            }
        };
        let a2686 = 0u64;
        if nat_eq(a2685, a2686) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f85(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2699 = f168(v0, v1, match v2.clone() {
        Adt3::C0(h2687, _, _, _, _, _, _, _) => {
            h2687
        }
    }, match v2.clone() {
        Adt3::C0(_, h2688, _, _, _, _, _, _) => {
            h2688
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2689, _, _, _, _, _) => {
            h2689
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2690, _, _, _, _) => {
            h2690
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2691, _, _, _) => {
            h2691
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2692, _, _) => {
            h2692
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2693, _) => {
            h2693
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2694) => {
            h2694
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2695, _, _, _) => {
            h2695
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2696, _, _) => {
            h2696
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2697, _) => {
            h2697
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2698) => {
            h2698
        }
    }, v4)?;
    match m2699 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2702 = v6;
        let a2703 = {
            let a2700 = 3u64;
            let a2701 = v5;
            nat_add(a2700, a2701)?
        };
        nat_sub(a2702, a2703)
    };
    let a2705 = v0;
    let a2706 = f36(v3, match v1.clone() {
        Adt3::C0(h2704, _, _, _, _, _, _, _) => {
            h2704
        }
    }, v5, v8)?;
    if nat_lt(a2705, a2706) {
        let v9: u64 = f100(match v1.clone() {
            Adt3::C0(h2707, _, _, _, _, _, _, _) => {
                h2707
            }
        }, v5, v8)?;
        let v10: u64 = f101(v3, match v1.clone() {
            Adt3::C0(h2708, _, _, _, _, _, _, _) => {
                h2708
            }
        }, v5)?;
        let a2709 = v0;
        let a2710 = v9;
        if nat_lt(a2709, a2710) {
            f169(v0, v2, f30(v3), {
                let a2711 = 3u64;
                let a2712 = v5;
                nat_add(a2711, a2712)?
            }, v7)
        } else {
            let a2715 = v0;
            let a2716 = {
                let a2713 = v9;
                let a2714 = v10;
                nat_add(a2713, a2714)?
            };
            if nat_lt(a2715, a2716) {
                let v11: u64 = {
                    let a2717 = v0;
                    let a2718 = v9;
                    nat_sub(a2717, a2718)
                };
                let v12: u64 = f170(v3, match v1.clone() {
                    Adt3::C0(h2719, _, _, _, _, _, _, _) => {
                        h2719
                    }
                })?;
                f171({
                    let a2720 = v11;
                    let a2721 = v12;
                    let a2722 = 0u64;
                    nat_rem(a2720, a2721, a2722)
                }, {
                    let a2723 = v11;
                    let a2724 = v12;
                    let a2725 = 0u64;
                    nat_quot(a2723, a2724, a2725)
                }, v1, v3, v4, v7)
            } else {
                f172({
                    let a2728 = v0;
                    let a2729 = {
                        let a2726 = v9;
                        let a2727 = v10;
                        nat_add(a2726, a2727)?
                    };
                    nat_sub(a2728, a2729)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2730 = v0;
    let a2731 = f19(v1)?;
    if nat_lt(a2730, a2731) {
        let a2732 = v0;
        let a2733 = 4u64;
        if nat_lt(a2732, a2733) {
            let a2734 = v0;
            let a2735 = 0u64;
            if nat_eq(a2734, a2735) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2736 = v0;
                let a2737 = 1u64;
                if nat_eq(a2736, a2737) {
                    let a2738 = v5;
                    let a2739 = 0u64;
                    if nat_eq(a2738, a2739) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2740 = v0;
                    let a2741 = 2u64;
                    if nat_eq(a2740, a2741) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2742 = v0;
            let a2743 = 4u64;
            if if nat_eq(a2742, a2743) {
                let a2744 = f173(v5)?;
                let a2745 = v1;
                !nat_eq(a2744, a2745)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2748 = {
                        let a2746 = v0;
                        let a2747 = 4u64;
                        nat_sub(a2746, a2747)
                    };
                    let a2749 = 3u64;
                    let a2750 = 0u64;
                    nat_quot(a2748, a2749, a2750)
                };
                let v7: u64 = {
                    let a2753 = {
                        let a2751 = v0;
                        let a2752 = 4u64;
                        nat_sub(a2751, a2752)
                    };
                    let a2754 = 3u64;
                    let a2755 = 0u64;
                    nat_rem(a2753, a2754, a2755)
                };
                let a2756 = v7;
                let a2757 = 0u64;
                if nat_eq(a2756, a2757) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2758 = v7;
                    let a2759 = 1u64;
                    if nat_eq(a2758, a2759) {
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

pub fn f88(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2760 = v1;
        let a2761 = 5u64;
        nat_eq(a2760, a2761)
    } else {
        false
    } {
        let a2762 = v2;
        let a2763 = 16u64;
        nat_eq(a2762, a2763)
    } else {
        false
    } {
        let a2764 = v3;
        let a2765 = 0u64;
        nat_eq(a2764, a2765)
    } else {
        false
    } {
        let a2766 = v4;
        let a2767 = 0u64;
        nat_eq(a2766, a2767)
    } else {
        false
    }
}

pub fn f89(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2768 = v1;
    let a2769 = 0u64;
    if nat_eq(a2768, a2769) {
        if v0 {
            let a2770 = v3;
            let a2771 = 0u64;
            if if nat_eq(a2770, a2771) {
                let a2772 = v4;
                let a2773 = 43u64;
                nat_eq(a2772, a2773)
            } else {
                false
            } {
                f156(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2774 = v1;
        let a2775 = 1u64;
        if nat_eq(a2774, a2775) {
            Adt11::C1(v2)
        } else {
            let a2776 = v1;
            let a2777 = 2u64;
            if nat_eq(a2776, a2777) {
                Adt11::C5(v2)
            } else {
                let a2778 = v1;
                let a2779 = 3u64;
                if nat_eq(a2778, a2779) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2780 = v1;
                    let a2781 = 4u64;
                    if nat_eq(a2780, a2781) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f88(v0, v1, v2, v3, v4) {
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

pub fn f90(v0: u64, v1: u64) -> Adt12 {
    let a2782 = v0;
    let a2783 = 0u64;
    if nat_eq(a2782, a2783) {
        Adt12::C0(0u64, v1)
    } else {
        let a2784 = v0;
        let a2785 = 1u64;
        if nat_eq(a2784, a2785) {
            Adt12::C1(v1)
        } else {
            let a2786 = v0;
            let a2787 = 2u64;
            if nat_eq(a2786, a2787) {
                Adt12::C2(v1)
            } else {
                let a2788 = v0;
                let a2789 = 3u64;
                if nat_eq(a2788, a2789) {
                    Adt12::C3(v1)
                } else {
                    let a2790 = v0;
                    let a2791 = 13u64;
                    if nat_lt(a2790, a2791) {
                        Adt12::C0({
                            let a2792 = v0;
                            let a2793 = 3u64;
                            nat_sub(a2792, a2793)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f91(v0: u64, v1: u64) -> Adt13 {
    let a2794 = v0;
    let a2795 = 1u64;
    if nat_eq(a2794, a2795) {
        Adt13::C0(v1)
    } else {
        let a2796 = v0;
        let a2797 = 3u64;
        if if nat_eq(a2796, a2797) {
            true
        } else {
            let a2798 = v0;
            let a2799 = 6u64;
            nat_eq(a2798, a2799)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f92(v0: Adt6, v1: u64) -> R<u64> {
    let a2801 = 0u64;
    let a2802 = match v0.clone() {
        Adt6::C0(h2800, _, _, _) => {
            h2800
        }
    };
    if if if if if nat_lt(a2801, a2802) {
        let a2804 = 0u64;
        let a2805 = match v0.clone() {
            Adt6::C0(_, h2803, _, _) => {
                h2803
            }
        };
        nat_lt(a2804, a2805)
    } else {
        false
    } {
        let a2807 = 0u64;
        let a2808 = match v0.clone() {
            Adt6::C0(_, _, _, h2806) => {
                h2806
            }
        };
        nat_lt(a2807, a2808)
    } else {
        false
    } {
        let a2810 = match v0.clone() {
            Adt6::C0(_, h2809, _, _) => {
                h2809
            }
        };
        let a2811 = 2147483648u64;
        nat_lt(a2810, a2811)
    } else {
        false
    } {
        let a2813 = match v0.clone() {
            Adt6::C0(_, _, _, h2812) => {
                h2812
            }
        };
        let a2814 = 2147483648u64;
        nat_lt(a2813, a2814)
    } else {
        false
    } {
        let v2: u64 = {
            let a2818 = v1;
            let a2819 = {
                let a2816 = 2u64;
                let a2817 = f133(match v0.clone() {
                    Adt6::C0(_, _, h2815, _) => {
                        h2815
                    }
                });
                nat_mul(a2816, a2817)?
            };
            nat_add(a2818, a2819)?
        };
        let v3: u64 = {
            let a2831 = {
                let a2826 = 1u64;
                let a2827 = {
                    let a2824 = match v0.clone() {
                        Adt6::C0(_, _, _, h2820) => {
                            h2820
                        }
                    };
                    let a2825 = {
                        let a2822 = match v0.clone() {
                            Adt6::C0(h2821, _, _, _) => {
                                h2821
                            }
                        };
                        let a2823 = 1u64;
                        nat_sub(a2822, a2823)
                    };
                    nat_mul(a2824, a2825)?
                };
                nat_add(a2826, a2827)?
            };
            let a2832 = {
                let a2829 = 2u64;
                let a2830 = f132(match v0.clone() {
                    Adt6::C0(_, _, h2828, _) => {
                        h2828
                    }
                });
                nat_mul(a2829, a2830)?
            };
            nat_add(a2831, a2832)?
        };
        let a2833 = v3;
        let a2834 = v2;
        if nat_le(a2833, a2834) {
            let a2841 = 1u64;
            let a2842 = {
                let a2838 = {
                    let a2835 = v2;
                    let a2836 = v3;
                    nat_sub(a2835, a2836)
                };
                let a2839 = match v0 {
                    Adt6::C0(_, h2837, _, _) => {
                        h2837
                    }
                };
                let a2840 = 0u64;
                nat_quot(a2838, a2839, a2840)
            };
            nat_add(a2841, a2842)
        } else {
            let a2846 = {
                let a2843 = v3;
                let a2844 = v2;
                nat_sub(a2843, a2844)
            };
            let a2847 = match v0 {
                Adt6::C0(_, h2845, _, _) => {
                    h2845
                }
            };
            if nat_lt(a2846, a2847) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f93(v0: u64) -> u64 {
    let v1: u64 = {
        let a2848 = v0;
        let a2849 = 4294967296u64;
        let a2850 = 0u64;
        nat_rem(a2848, a2849, a2850)
    };
    let a2851 = v1;
    let a2852 = 2147483648u64;
    if nat_lt(a2851, a2852) {
        v1
    } else {
        0u64
    }
}

pub fn f94(v0: u64) -> R<u64> {
    let a2855 = 2u64;
    let a2856 = {
        let a2853 = 2u64;
        let a2854 = v0;
        nat_mul(a2853, a2854)?
    };
    nat_add(a2855, a2856)
}

pub fn f95(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2858 = match v1.clone() {
        Adt14::C0(h2857, _, _, _, _, _, _, _, _, _, _) => {
            h2857
        }
    };
    let a2859 = 0u64;
    if if if if nat_eq(a2858, a2859) {
        let a2861 = match v2.clone() {
            Adt14::C0(h2860, _, _, _, _, _, _, _, _, _, _) => {
                h2860
            }
        };
        let a2862 = 0u64;
        nat_eq(a2861, a2862)
    } else {
        false
    } {
        f174(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f175(match v1.clone() {
            Adt14::C0(_, h2863, _, _, _, _, _, _, _, _, _) => {
                h2863
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2864, _, _, _, _, _, _, _, _) => {
                h2864
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2865, _, _, _, _, _, _, _) => {
                h2865
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2866, _, _, _, _, _, _) => {
                h2866
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f176(v1.clone(), v2.clone())
            } else {
                false
            } {
                f177(v1.clone())
            } else {
                false
            } {
                f177(v2.clone())
            } else {
                false
            } {
                if f178(v1.clone(), v2.clone())? {
                    true
                } else {
                    f179(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f175(match v1.clone() {
            Adt14::C0(_, h2867, _, _, _, _, _, _, _, _, _) => {
                h2867
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2868, _, _, _, _, _, _, _, _) => {
                h2868
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2869, _, _, _, _, _, _, _) => {
                h2869
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2870, _, _, _, _, _, _) => {
                h2870
            }
        }) {
            0u64
        } else {
            let a2876 = match v1.clone() {
                Adt14::C0(_, _, h2871, _, _, _, _, _, _, _, _) => {
                    h2871
                }
            };
            let a2877 = {
                let a2874 = match v1.clone() {
                    Adt14::C0(_, _, _, h2872, _, _, _, _, _, _, _) => {
                        h2872
                    }
                };
                let a2875 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2873, _, _, _, _, _, _) => {
                        h2873
                    }
                };
                nat_mul(a2874, a2875)?
            };
            nat_mul(a2876, a2877)?
        };
        let v4: u64 = {
            let a2878 = v3;
            let a2879 = 0u64;
            if if nat_eq(a2878, a2879) {
                true
            } else {
                f180(v2.clone())?
            } {
                0u64
            } else {
                let a2881 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2880, _) => {
                        h2880
                    }
                };
                let a2882 = 4u64;
                let a2883 = 0u64;
                nat_quot(a2881, a2882, a2883)
            }
        };
        Ok(Adt15::C1(f181(f182(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2884, _, _, _, _, _) => {
                h2884
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2885, _, _, _, _, _) => {
                h2885
            }
        }), match v1 {
            Adt14::C0(_, h2886, _, _, _, _, _, _, _, _, _) => {
                h2886
            }
        }, v3, v4)?, 4u64, 64u64, v3, f183(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f96(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f184(v3.clone());
    let a2888 = match v2.clone() {
        Adt14::C0(h2887, _, _, _, _, _, _, _, _, _, _) => {
            h2887
        }
    };
    let a2889 = 0u64;
    if if if if nat_eq(a2888, a2889) {
        let a2891 = match v3.clone() {
            Adt14::C0(h2890, _, _, _, _, _, _, _, _, _, _) => {
                h2890
            }
        };
        let a2892 = 0u64;
        nat_eq(a2891, a2892)
    } else {
        false
    } {
        f174(v2.clone(), v3)
    } else {
        false
    } {
        if f175(match v2.clone() {
            Adt14::C0(_, h2893, _, _, _, _, _, _, _, _, _) => {
                h2893
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2894, _, _, _, _, _, _, _, _) => {
                h2894
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2895, _, _, _, _, _, _, _) => {
                h2895
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2896, _, _, _, _, _, _) => {
                h2896
            }
        }) {
            true
        } else {
            let a2898 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2897, _, _, _, _, _) => {
                    h2897
                }
            };
            let a2899 = 4u64;
            if if if if if nat_eq(a2898, a2899) {
                f185(match v2.clone() {
                    Adt14::C0(_, h2900, _, _, _, _, _, _, _, _, _) => {
                        h2900
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2901, _, _, _, _, _, _, _, _) => {
                        h2901
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2902, _, _, _, _, _, _, _) => {
                        h2902
                    }
                }, v0)?
            } else {
                false
            } {
                f177(v2.clone())
            } else {
                false
            } {
                f177(v4.clone())
            } else {
                false
            } {
                let a2904 = f186(v4.clone())?;
                let a2905 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2903, _) => {
                        h2903
                    }
                };
                nat_le(a2904, a2905)
            } else {
                false
            } {
                if f178(v2.clone(), v4.clone())? {
                    true
                } else {
                    f179(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f175(match v2.clone() {
            Adt14::C0(_, h2906, _, _, _, _, _, _, _, _, _) => {
                h2906
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2907, _, _, _, _, _, _, _, _) => {
                h2907
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2908, _, _, _, _, _, _, _) => {
                h2908
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2909, _, _, _, _, _, _) => {
                h2909
            }
        }) {
            true
        } else {
            f180(v4.clone())?
        } {
            0u64
        } else {
            let a2911 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2910, _) => {
                    h2910
                }
            };
            let a2912 = 4u64;
            let a2913 = 0u64;
            nat_quot(a2911, a2912, a2913)
        };
        Ok(Adt15::C1(f187(match v2.clone() {
            Adt14::C0(_, h2914, _, _, _, _, _, _, _, _, _) => {
                h2914
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2915, _, _, _, _, _, _, _, _) => {
                h2915
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2916, _, _, _, _, _, _, _) => {
                h2916
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2917, _, _, _, _, _, _) => {
                h2917
            }
        }, v0, v5)?, 4u64, 65u64, v0, f183(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f97(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f184(v4.clone());
    if if if if if if if v1 {
        let a2919 = match v2.clone() {
            Adt14::C0(h2918, _, _, _, _, _, _, _, _, _, _) => {
                h2918
            }
        };
        let a2920 = 0u64;
        nat_eq(a2919, a2920)
    } else {
        false
    } {
        let a2922 = match v3.clone() {
            Adt14::C0(h2921, _, _, _, _, _, _, _, _, _, _) => {
                h2921
            }
        };
        let a2923 = 0u64;
        nat_eq(a2922, a2923)
    } else {
        false
    } {
        let a2925 = match v4.clone() {
            Adt14::C0(h2924, _, _, _, _, _, _, _, _, _, _) => {
                h2924
            }
        };
        let a2926 = 0u64;
        nat_eq(a2925, a2926)
    } else {
        false
    } {
        f174(v2.clone(), v3.clone())
    } else {
        false
    } {
        f174(v2.clone(), v4)
    } else {
        false
    } {
        if f175(match v2.clone() {
            Adt14::C0(_, h2927, _, _, _, _, _, _, _, _, _) => {
                h2927
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2928, _, _, _, _, _, _, _, _) => {
                h2928
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2929, _, _, _, _, _, _, _) => {
                h2929
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2930, _, _, _, _, _, _) => {
                h2930
            }
        }) {
            true
        } else {
            let a2932 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2931, _, _, _, _, _) => {
                    h2931
                }
            };
            let a2933 = 4u64;
            if if if if if if if nat_eq(a2932, a2933) {
                let a2935 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2934, _, _, _, _, _) => {
                        h2934
                    }
                };
                let a2936 = 4u64;
                nat_eq(a2935, a2936)
            } else {
                false
            } {
                f177(v2.clone())
            } else {
                false
            } {
                f177(v3.clone())
            } else {
                false
            } {
                f177(v5.clone())
            } else {
                false
            } {
                let a2938 = f186(v5.clone())?;
                let a2939 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2937, _) => {
                        h2937
                    }
                };
                nat_le(a2938, a2939)
            } else {
                false
            } {
                if f178(v2.clone(), v5.clone())? {
                    true
                } else {
                    f179(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f178(v3.clone(), v5.clone())? {
                    true
                } else {
                    f179(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f175(match v2.clone() {
            Adt14::C0(_, h2940, _, _, _, _, _, _, _, _, _) => {
                h2940
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2941, _, _, _, _, _, _, _, _) => {
                h2941
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2942, _, _, _, _, _, _, _) => {
                h2942
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2943, _, _, _, _, _, _) => {
                h2943
            }
        }) {
            0u64
        } else {
            let a2949 = match v2.clone() {
                Adt14::C0(_, _, h2944, _, _, _, _, _, _, _, _) => {
                    h2944
                }
            };
            let a2950 = {
                let a2947 = match v2.clone() {
                    Adt14::C0(_, _, _, h2945, _, _, _, _, _, _, _) => {
                        h2945
                    }
                };
                let a2948 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2946, _, _, _, _, _, _) => {
                        h2946
                    }
                };
                nat_mul(a2947, a2948)?
            };
            nat_mul(a2949, a2950)?
        };
        let v7: u64 = {
            let a2951 = v6;
            let a2952 = 0u64;
            if if nat_eq(a2951, a2952) {
                true
            } else {
                f180(v5.clone())?
            } {
                0u64
            } else {
                let a2954 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2953, _) => {
                        h2953
                    }
                };
                let a2955 = 4u64;
                let a2956 = 0u64;
                nat_quot(a2954, a2955, a2956)
            }
        };
        Ok(Adt15::C1(f188(match v2 {
            Adt14::C0(_, h2957, _, _, _, _, _, _, _, _, _) => {
                h2957
            }
        }, v6, v7)?, 4u64, 66u64, v6, f183(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f189(v0, v3, v6) {
        f189(v1, v4, v6)
    } else {
        false
    } {
        f189(v2, v5, v6)
    } else {
        false
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f190(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f190(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f190(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f190(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f190(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f190(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2958 = v2;
    let a2959 = 0u64;
    if nat_eq(a2958, a2959) {
        Ok(0u64)
    } else {
        let a2962 = 2u64;
        let a2963 = {
            let a2960 = v0;
            let a2961 = v1;
            nat_mul(a2960, a2961)?
        };
        nat_mul(a2962, a2963)
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2964 = v2;
    let a2965 = f170(v0, v1)?;
    nat_mul(a2964, a2965)
}

pub fn f102(v0: u64) -> u64 {
    let a2966 = v0;
    let a2967 = 30u64;
    if nat_eq(a2966, a2967) {
        3u64
    } else {
        1u64
    }
}

pub fn f103(v0: u64) -> R<u64> {
    let a2970 = 5u64;
    let a2971 = {
        let a2968 = 4u64;
        let a2969 = v0;
        nat_mul(a2968, a2969)?
    };
    nat_add(a2970, a2971)
}

pub fn f104(v0: u64) -> bool {
    let a2972 = v0;
    let a2973 = 0u64;
    if if nat_eq(a2972, a2973) {
        true
    } else {
        let a2974 = v0;
        let a2975 = 1u64;
        nat_eq(a2974, a2975)
    } {
        true
    } else {
        let a2976 = v0;
        let a2977 = 30u64;
        nat_eq(a2976, a2977)
    }
}

pub fn f105(v0: Adt2, v1: u64) -> R<bool> {
    let a2979 = match v0.clone() {
        Adt2::C0(_, _, _, h2978, _, _, _, _, _, _, _) => {
            h2978
        }
    };
    let a2980 = 1u64;
    if if if nat_eq(a2979, a2980) {
        let a2982 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2981, _, _, _, _, _, _) => {
                h2981
            }
        };
        let a2983 = 1u64;
        nat_eq(a2982, a2983)
    } else {
        false
    } {
        let a2985 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2984, _, _, _, _, _) => {
                h2984
            }
        };
        let a2986 = v1;
        nat_eq(a2985, a2986)
    } else {
        false
    } {
        let a2988 = match v0.clone() {
            Adt2::C0(_, _, h2987, _, _, _, _, _, _, _, _) => {
                h2987
            }
        };
        let a2989 = 1u64;
        if nat_eq(a2988, a2989) {
            Ok(true)
        } else {
            let a2994 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2990, _, _, _, _) => {
                    h2990
                }
            };
            let a2995 = {
                let a2992 = match v0 {
                    Adt2::C0(_, h2991, _, _, _, _, _, _, _, _, _) => {
                        h2991
                    }
                };
                let a2993 = v1;
                nat_mul(a2992, a2993)?
            };
            Ok(nat_eq(a2994, a2995))
        }
    } else {
        Ok(false)
    }
}

pub fn f106(v0: Adt6) -> R<u64> {
    let a3008 = 2u64;
    let a3009 = {
        let a3006 = 2u64;
        let a3007 = {
            let a3003 = {
                let a3000 = match v0.clone() {
                    Adt6::C0(h2996, _, _, _) => {
                        h2996
                    }
                };
                let a3001 = {
                    let a2998 = match v0.clone() {
                        Adt6::C0(_, h2997, _, _) => {
                            h2997
                        }
                    };
                    let a2999 = 1u64;
                    nat_sub(a2998, a2999)
                };
                nat_add(a3000, a3001)?
            };
            let a3004 = match v0 {
                Adt6::C0(_, h3002, _, _) => {
                    h3002
                }
            };
            let a3005 = 0u64;
            nat_quot(a3003, a3004, a3005)
        };
        nat_mul(a3006, a3007)?
    };
    nat_add(a3008, a3009)
}

pub fn f107(v0: u64) -> R<u64> {
    let a3012 = 2u64;
    let a3013 = {
        let a3010 = v0;
        let a3011 = 2u64;
        nat_mul(a3010, a3011)?
    };
    nat_add(a3012, a3013)
}

pub fn f108(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3017 = {
            let a3014 = v2;
            let a3015 = 1u64;
            nat_sub(a3014, a3015)
        };
        let a3018 = match v0.clone() {
            Adt6::C0(_, h3016, _, _) => {
                h3016
            }
        };
        nat_mul(a3017, a3018)?
    };
    let a3020 = 0u64;
    let a3021 = match v0.clone() {
        Adt6::C0(h3019, _, _, _) => {
            h3019
        }
    };
    if if if if if if if if nat_lt(a3020, a3021) {
        let a3023 = match v0.clone() {
            Adt6::C0(h3022, _, _, _) => {
                h3022
            }
        };
        let a3024 = 2147483648u64;
        nat_lt(a3023, a3024)
    } else {
        false
    } {
        let a3026 = 0u64;
        let a3027 = match v0.clone() {
            Adt6::C0(_, h3025, _, _) => {
                h3025
            }
        };
        nat_lt(a3026, a3027)
    } else {
        false
    } {
        let a3029 = match v0.clone() {
            Adt6::C0(_, h3028, _, _) => {
                h3028
            }
        };
        let a3030 = 2147483648u64;
        nat_lt(a3029, a3030)
    } else {
        false
    } {
        let a3031 = v1;
        let a3032 = 2147483648u64;
        nat_lt(a3031, a3032)
    } else {
        false
    } {
        let a3033 = 0u64;
        let a3034 = v2;
        nat_lt(a3033, a3034)
    } else {
        false
    } {
        let a3035 = v3;
        let a3036 = 2147483648u64;
        nat_lt(a3035, a3036)
    } else {
        false
    } {
        let a3048 = {
            let a3043 = {
                let a3040 = v3;
                let a3041 = {
                    let a3038 = match v0.clone() {
                        Adt6::C0(h3037, _, _, _) => {
                            h3037
                        }
                    };
                    let a3039 = 1u64;
                    nat_sub(a3038, a3039)
                };
                nat_add(a3040, a3041)?
            };
            let a3044 = f132(match v0.clone() {
                Adt6::C0(_, _, h3042, _) => {
                    h3042
                }
            });
            nat_add(a3043, a3044)?
        };
        let a3049 = {
            let a3046 = 2147483647u64;
            let a3047 = f133(match v0.clone() {
                Adt6::C0(_, _, h3045, _) => {
                    h3045
                }
            });
            nat_add(a3046, a3047)?
        };
        nat_le(a3048, a3049)
    } else {
        false
    } {
        let a3051 = match v0 {
            Adt6::C0(_, _, h3050, _) => {
                h3050
            }
        };
        let a3052 = 2147483648u64;
        Ok(!nat_eq(a3051, a3052))
    } else {
        Ok(false)
    }
}

pub fn f109(v0: Adt7) -> R<u64> {
    let a3064 = if match v0.clone() {
        Adt7::C0(_, _, _, h3053, _) => {
            h3053
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3065 = {
        let a3062 = {
            let a3060 = {
                let m3056 = match v0.clone() {
                    Adt7::C0(h3055, _, _, _, _) => {
                        h3055
                    }
                };
                match m3056 {
                    Adt6::C0(h3054, _, _, _) => {
                        h3054
                    }
                }
            };
            let a3061 = {
                let m3059 = match v0.clone() {
                    Adt7::C0(_, h3058, _, _, _) => {
                        h3058
                    }
                };
                match m3059 {
                    Adt6::C0(h3057, _, _, _) => {
                        h3057
                    }
                }
            };
            nat_mul(a3060, a3061)?
        };
        let a3063 = f165(v0)?;
        nat_mul(a3062, a3063)?
    };
    nat_add(a3064, a3065)
}

pub fn f110(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f191(match v0.clone() {
        Adt2::C0(h3066, _, _, _, _, _, _, _, _, _, _) => {
            h3066
        }
    }, match v0.clone() {
        Adt2::C0(_, h3067, _, _, _, _, _, _, _, _, _) => {
            h3067
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3068, _, _, _, _, _, _, _, _) => {
            h3068
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3069, _, _, _, _, _, _, _) => {
            h3069
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3070, _, _, _, _, _, _) => {
            h3070
        }
    }, match v1.clone() {
        Adt2::C0(h3071, _, _, _, _, _, _, _, _, _, _) => {
            h3071
        }
    }, match v1.clone() {
        Adt2::C0(_, h3072, _, _, _, _, _, _, _, _, _) => {
            h3072
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3073, _, _, _, _, _, _, _, _) => {
            h3073
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3074, _, _, _, _, _, _, _) => {
            h3074
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3075, _, _, _, _, _, _) => {
            h3075
        }
    }, match v2.clone() {
        Adt2::C0(h3076, _, _, _, _, _, _, _, _, _, _) => {
            h3076
        }
    }, match v2.clone() {
        Adt2::C0(_, h3077, _, _, _, _, _, _, _, _, _) => {
            h3077
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3078, _, _, _, _, _, _, _, _) => {
            h3078
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3079, _, _, _, _, _, _, _) => {
            h3079
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3080, _, _, _, _, _, _) => {
            h3080
        }
    }) {
        let m3082 = f114(match v0.clone() {
            Adt2::C0(h3081, _, _, _, _, _, _, _, _, _, _) => {
                h3081
            }
        });
        match m3082 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3088 = {
                    let a3085 = match v0.clone() {
                        Adt2::C0(_, h3083, _, _, _, _, _, _, _, _, _) => {
                            h3083
                        }
                    };
                    let a3086 = match v3.clone() {
                        Adt18::C0(_, h3084, _) => {
                            h3084
                        }
                    };
                    let a3087 = 0u64;
                    nat_rem(a3085, a3086, a3087)
                };
                let a3089 = 0u64;
                if if nat_eq(a3088, a3089) {
                    if f192(match v0.clone() {
                        Adt2::C0(_, h3090, _, _, _, _, _, _, _, _, _) => {
                            h3090
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3091, _, _, _, _, _, _, _, _, _) => {
                            h3091
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3092, _, _, _, _, _, _, _, _) => {
                            h3092
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3093, _, _, _, _, _, _, _) => {
                            h3093
                        }
                    }) {
                        true
                    } else {
                        let a3095 = 4u64;
                        let a3096 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3094, _, _, _, _, _) => {
                                h3094
                            }
                        };
                        if if nat_le(a3095, a3096) {
                            let a3098 = match v3.clone() {
                                Adt18::C0(_, h3097, _) => {
                                    h3097
                                }
                            };
                            let a3099 = 1u64;
                            if nat_eq(a3098, a3099) {
                                true
                            } else {
                                let a3102 = match v3.clone() {
                                    Adt18::C0(_, _, h3100) => {
                                        h3100
                                    }
                                };
                                let a3103 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3101, _, _, _, _, _) => {
                                        h3101
                                    }
                                };
                                nat_le(a3102, a3103)
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
                        Adt18::C0(_, h3104, _) => {
                            h3104
                        }
                    });
                    Ok(Adt10::C4(if f192(match v0.clone() {
                        Adt2::C0(_, h3105, _, _, _, _, _, _, _, _, _) => {
                            h3105
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3106, _, _, _, _, _, _, _, _, _) => {
                            h3106
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3107, _, _, _, _, _, _, _, _) => {
                            h3107
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3108, _, _, _, _, _, _, _) => {
                            h3108
                        }
                    }) {
                        0u64
                    } else {
                        f159(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3109, _, _, _, _, _, _, _, _, _, _) => {
                            h3109
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

pub fn f111(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f112(match v0.clone() {
        Adt2::C0(h3110, _, _, _, _, _, _, _, _, _, _) => {
            h3110
        }
    }) {
        let a3116 = {
            let a3113 = match v0.clone() {
                Adt2::C0(_, h3111, _, _, _, _, _, _, _, _, _) => {
                    h3111
                }
            };
            let a3114 = f193(match v0.clone() {
                Adt2::C0(h3112, _, _, _, _, _, _, _, _, _, _) => {
                    h3112
                }
            });
            let a3115 = 1u64;
            nat_rem(a3113, a3114, a3115)
        };
        let a3117 = 0u64;
        nat_eq(a3116, a3117)
    } else {
        false
    } {
        let a3119 = match v1.clone() {
            Adt2::C0(h3118, _, _, _, _, _, _, _, _, _, _) => {
                h3118
            }
        };
        let a3120 = 26u64;
        nat_eq(a3119, a3120)
    } else {
        false
    } {
        let a3122 = match v2.clone() {
            Adt2::C0(h3121, _, _, _, _, _, _, _, _, _, _) => {
                h3121
            }
        };
        let a3123 = 0u64;
        nat_eq(a3122, a3123)
    } else {
        false
    } {
        let a3126 = match v0.clone() {
            Adt2::C0(_, h3124, _, _, _, _, _, _, _, _, _) => {
                h3124
            }
        };
        let a3127 = match v2.clone() {
            Adt2::C0(_, h3125, _, _, _, _, _, _, _, _, _) => {
                h3125
            }
        };
        nat_eq(a3126, a3127)
    } else {
        false
    } {
        let a3130 = match v1.clone() {
            Adt2::C0(_, h3128, _, _, _, _, _, _, _, _, _) => {
                h3128
            }
        };
        let a3131 = match v0.clone() {
            Adt2::C0(_, _, h3129, _, _, _, _, _, _, _, _) => {
                h3129
            }
        };
        nat_le(a3130, a3131)
    } else {
        false
    } {
        let a3133 = match v0.clone() {
            Adt2::C0(_, _, _, h3132, _, _, _, _, _, _, _) => {
                h3132
            }
        };
        let a3134 = 1u64;
        nat_eq(a3133, a3134)
    } else {
        false
    } {
        let a3136 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3135, _, _, _, _, _, _) => {
                h3135
            }
        };
        let a3137 = 1u64;
        nat_eq(a3136, a3137)
    } else {
        false
    } {
        let a3139 = match v2.clone() {
            Adt2::C0(_, _, _, h3138, _, _, _, _, _, _, _) => {
                h3138
            }
        };
        let a3140 = 1u64;
        nat_eq(a3139, a3140)
    } else {
        false
    } {
        let a3142 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3141, _, _, _, _, _, _) => {
                h3141
            }
        };
        let a3143 = 1u64;
        nat_eq(a3142, a3143)
    } else {
        false
    } {
        let a3145 = match v1.clone() {
            Adt2::C0(_, _, h3144, _, _, _, _, _, _, _, _) => {
                h3144
            }
        };
        let a3146 = 1u64;
        nat_eq(a3145, a3146)
    } else {
        false
    } {
        let a3148 = match v1.clone() {
            Adt2::C0(_, _, _, h3147, _, _, _, _, _, _, _) => {
                h3147
            }
        };
        let a3149 = 1u64;
        nat_eq(a3148, a3149)
    } else {
        false
    } {
        let a3151 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3150, _, _, _, _, _, _) => {
                h3150
            }
        };
        let a3152 = 1u64;
        nat_eq(a3151, a3152)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3153, _, _, _, _, _, _, _, _, _) => {
                h3153
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3154, _, _, _, _, _, _, _, _) => {
                h3154
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3155, _, _, _, _, _, _, _) => {
                h3155
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3156, _, _, _, _, _, _) => {
                h3156
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3158 = match v1.clone() {
                Adt2::C0(_, h3157, _, _, _, _, _, _, _, _, _) => {
                    h3157
                }
            };
            let a3159 = 0u64;
            if nat_eq(a3158, a3159) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3161 = match v0.clone() {
                    Adt2::C0(_, h3160, _, _, _, _, _, _, _, _, _) => {
                        h3160
                    }
                };
                let a3162 = 2147483647u64;
                if if if if if if nat_le(a3161, a3162) {
                    let a3164 = match v1.clone() {
                        Adt2::C0(_, h3163, _, _, _, _, _, _, _, _, _) => {
                            h3163
                        }
                    };
                    let a3165 = 2147483647u64;
                    nat_le(a3164, a3165)
                } else {
                    false
                } {
                    let a3167 = 4u64;
                    let a3168 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3166, _, _, _, _, _) => {
                            h3166
                        }
                    };
                    nat_le(a3167, a3168)
                } else {
                    false
                } {
                    let a3174 = {
                        let a3171 = match v2.clone() {
                            Adt2::C0(_, h3169, _, _, _, _, _, _, _, _, _) => {
                                h3169
                            }
                        };
                        let a3172 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3170, _, _, _, _, _) => {
                                h3170
                            }
                        };
                        nat_mul(a3171, a3172)?
                    };
                    let a3175 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3173, _, _, _, _) => {
                            h3173
                        }
                    };
                    nat_le(a3174, a3175)
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
                        let a3188 = {
                            let a3179 = {
                                let a3177 = 2u64;
                                let a3178 = match v1.clone() {
                                    Adt2::C0(_, h3176, _, _, _, _, _, _, _, _, _) => {
                                        h3176
                                    }
                                };
                                nat_mul(a3177, a3178)?
                            };
                            let a3180 = f16(v2.clone(), 1u64)?;
                            nat_add(a3179, a3180)?
                        };
                        let a3189 = {
                            let a3186 = match v1.clone() {
                                Adt2::C0(_, h3181, _, _, _, _, _, _, _, _, _) => {
                                    h3181
                                }
                            };
                            let a3187 = {
                                let a3184 = 1u64;
                                let a3185 = f157(match v0.clone() {
                                    Adt2::C0(h3182, _, _, _, _, _, _, _, _, _, _) => {
                                        h3182
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3183, _, _, _, _, _, _, _, _, _) => {
                                        h3183
                                    }
                                })?;
                                nat_add(a3184, a3185)?
                            };
                            nat_mul(a3186, a3187)?
                        };
                        nat_add(a3188, a3189)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3190, _, _, _, _, _, _, _, _, _) => {
                            h3190
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3191, _, _, _, _, _, _, _, _, _) => {
                            h3191
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3192, _, _, _, _, _) => {
                            h3192
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3193, _, _, _, _) => {
                            h3193
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3194, _, _, _) => {
                            h3194
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3195, _, _) => {
                            h3195
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3196, _, _, _, _, _, _, _, _, _, _) => {
                            h3196
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

pub fn f112(v0: u64) -> bool {
    let a3197 = v0;
    let a3198 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3197, a3198) {
        true
    } else {
        let a3199 = v0;
        let a3200 = 1u64;
        nat_eq(a3199, a3200)
    } {
        true
    } else {
        let a3201 = v0;
        let a3202 = 2u64;
        nat_eq(a3201, a3202)
    } {
        true
    } else {
        let a3203 = v0;
        let a3204 = 3u64;
        nat_eq(a3203, a3204)
    } {
        true
    } else {
        let a3205 = v0;
        let a3206 = 6u64;
        nat_eq(a3205, a3206)
    } {
        true
    } else {
        let a3207 = v0;
        let a3208 = 7u64;
        nat_eq(a3207, a3208)
    } {
        true
    } else {
        let a3209 = v0;
        let a3210 = 8u64;
        nat_eq(a3209, a3210)
    } {
        true
    } else {
        let a3211 = v0;
        let a3212 = 10u64;
        nat_eq(a3211, a3212)
    } {
        true
    } else {
        let a3213 = v0;
        let a3214 = 11u64;
        nat_eq(a3213, a3214)
    } {
        true
    } else {
        let a3215 = v0;
        let a3216 = 12u64;
        nat_eq(a3215, a3216)
    } {
        true
    } else {
        let a3217 = v0;
        let a3218 = 13u64;
        nat_eq(a3217, a3218)
    } {
        true
    } else {
        let a3219 = v0;
        let a3220 = 14u64;
        nat_eq(a3219, a3220)
    } {
        true
    } else {
        let a3221 = v0;
        let a3222 = 16u64;
        nat_eq(a3221, a3222)
    } {
        true
    } else {
        let a3223 = v0;
        let a3224 = 17u64;
        nat_eq(a3223, a3224)
    } {
        true
    } else {
        let a3225 = v0;
        let a3226 = 18u64;
        nat_eq(a3225, a3226)
    } {
        true
    } else {
        let a3227 = v0;
        let a3228 = 19u64;
        nat_eq(a3227, a3228)
    } {
        true
    } else {
        let a3229 = v0;
        let a3230 = 20u64;
        nat_eq(a3229, a3230)
    } {
        true
    } else {
        let a3231 = v0;
        let a3232 = 21u64;
        nat_eq(a3231, a3232)
    } {
        true
    } else {
        let a3233 = v0;
        let a3234 = 22u64;
        nat_eq(a3233, a3234)
    } {
        true
    } else {
        let a3235 = v0;
        let a3236 = 23u64;
        nat_eq(a3235, a3236)
    } {
        true
    } else {
        let a3237 = v0;
        let a3238 = 26u64;
        nat_eq(a3237, a3238)
    } {
        true
    } else {
        let a3239 = v0;
        let a3240 = 29u64;
        nat_eq(a3239, a3240)
    } {
        true
    } else {
        let a3241 = v0;
        let a3242 = 30u64;
        nat_eq(a3241, a3242)
    } {
        true
    } else {
        let a3243 = v0;
        let a3244 = 34u64;
        nat_eq(a3243, a3244)
    } {
        true
    } else {
        let a3245 = v0;
        let a3246 = 35u64;
        nat_eq(a3245, a3246)
    } {
        true
    } else {
        let a3247 = v0;
        let a3248 = 39u64;
        nat_eq(a3247, a3248)
    } {
        true
    } else {
        let a3249 = v0;
        let a3250 = 40u64;
        nat_eq(a3249, a3250)
    } {
        true
    } else {
        let a3251 = v0;
        let a3252 = 41u64;
        nat_eq(a3251, a3252)
    } {
        true
    } else {
        let a3253 = v0;
        let a3254 = 42u64;
        nat_eq(a3253, a3254)
    } {
        true
    } else {
        let a3255 = v0;
        let a3256 = 43u64;
        nat_eq(a3255, a3256)
    }
}

pub fn f113(v0: u64) -> bool {
    if f104(v0) {
        true
    } else {
        let a3257 = v0;
        let a3258 = 43u64;
        nat_eq(a3257, a3258)
    }
}

pub fn f114(v0: u64) -> Adt17 {
    let a3259 = v0;
    let a3260 = 0u64;
    if nat_eq(a3259, a3260) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3261 = v0;
        let a3262 = 1u64;
        if nat_eq(a3261, a3262) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3263 = v0;
            let a3264 = 2u64;
            if nat_eq(a3263, a3264) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3265 = v0;
                let a3266 = 3u64;
                if nat_eq(a3265, a3266) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3267 = v0;
                    let a3268 = 6u64;
                    if nat_eq(a3267, a3268) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3269 = v0;
                        let a3270 = 7u64;
                        if nat_eq(a3269, a3270) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3271 = v0;
                            let a3272 = 8u64;
                            if nat_eq(a3271, a3272) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3273 = v0;
                                let a3274 = 9u64;
                                if nat_eq(a3273, a3274) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3275 = v0;
                                    let a3276 = 10u64;
                                    if nat_eq(a3275, a3276) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3277 = v0;
                                        let a3278 = 11u64;
                                        if nat_eq(a3277, a3278) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3279 = v0;
                                            let a3280 = 12u64;
                                            if nat_eq(a3279, a3280) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3281 = v0;
                                                let a3282 = 13u64;
                                                if nat_eq(a3281, a3282) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3283 = v0;
                                                    let a3284 = 14u64;
                                                    if nat_eq(a3283, a3284) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3285 = v0;
                                                        let a3286 = 15u64;
                                                        if nat_eq(a3285, a3286) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3287 = v0;
                                                            let a3288 = 16u64;
                                                            if nat_eq(a3287, a3288) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3289 = v0;
                                                                let a3290 = 17u64;
                                                                if nat_eq(a3289, a3290) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3291 = v0;
                                                                    let a3292 = 18u64;
                                                                    if nat_eq(a3291, a3292) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3293 = v0;
                                                                        let a3294 = 19u64;
                                                                        if nat_eq(a3293, a3294) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3295 = v0;
                                                                            let a3296 = 20u64;
                                                                            if nat_eq(a3295, a3296) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3297 = v0;
                                                                                let a3298 = 21u64;
                                                                                if nat_eq(a3297, a3298) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3299 = v0;
                                                                                    let a3300 = 22u64;
                                                                                    if nat_eq(a3299, a3300) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3301 = v0;
                                                                                        let a3302 = 23u64;
                                                                                        if nat_eq(a3301, a3302) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3303 = v0;
                                                                                            let a3304 = 24u64;
                                                                                            if nat_eq(a3303, a3304) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3305 = v0;
                                                                                                let a3306 = 25u64;
                                                                                                if nat_eq(a3305, a3306) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3307 = v0;
                                                                                                    let a3308 = 26u64;
                                                                                                    if nat_eq(a3307, a3308) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3309 = v0;
                                                                                                        let a3310 = 27u64;
                                                                                                        if nat_eq(a3309, a3310) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3311 = v0;
                                                                                                            let a3312 = 28u64;
                                                                                                            if nat_eq(a3311, a3312) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3313 = v0;
                                                                                                                let a3314 = 29u64;
                                                                                                                if nat_eq(a3313, a3314) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3315 = v0;
                                                                                                                    let a3316 = 30u64;
                                                                                                                    if nat_eq(a3315, a3316) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3317 = v0;
                                                                                                                        let a3318 = 34u64;
                                                                                                                        if nat_eq(a3317, a3318) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3319 = v0;
                                                                                                                            let a3320 = 35u64;
                                                                                                                            if nat_eq(a3319, a3320) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3321 = v0;
                                                                                                                                let a3322 = 39u64;
                                                                                                                                if nat_eq(a3321, a3322) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3323 = v0;
                                                                                                                                    let a3324 = 40u64;
                                                                                                                                    if nat_eq(a3323, a3324) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3325 = v0;
                                                                                                                                        let a3326 = 41u64;
                                                                                                                                        if nat_eq(a3325, a3326) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3327 = v0;
                                                                                                                                            let a3328 = 42u64;
                                                                                                                                            if nat_eq(a3327, a3328) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3329 = v0;
                                                                                                                                                let a3330 = 43u64;
                                                                                                                                                if nat_eq(a3329, a3330) {
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

pub fn f115(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3333 = match v1.clone() {
        Adt2::C0(h3331, _, _, _, _, _, _, _, _, _, _) => {
            h3331
        }
    };
    let a3334 = match v3.clone() {
        Adt2::C0(h3332, _, _, _, _, _, _, _, _, _, _) => {
            h3332
        }
    };
    if if nat_eq(a3333, a3334) {
        let a3335 = v0;
        let a3336 = 33u64;
        if nat_eq(a3335, a3336) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3338 = f114(match v1.clone() {
            Adt2::C0(h3337, _, _, _, _, _, _, _, _, _, _) => {
                h3337
            }
        });
        match m3338 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3339, _) => {
                        h3339
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3344 = {
                    let a3341 = match v1.clone() {
                        Adt2::C0(_, h3340, _, _, _, _, _, _, _, _, _) => {
                            h3340
                        }
                    };
                    let a3342 = v5;
                    let a3343 = 1u64;
                    nat_rem(a3341, a3342, a3343)
                };
                let a3345 = 0u64;
                if if if nat_eq(a3344, a3345) {
                    let a3350 = {
                        let a3347 = match v3.clone() {
                            Adt2::C0(_, h3346, _, _, _, _, _, _, _, _, _) => {
                                h3346
                            }
                        };
                        let a3348 = v5;
                        let a3349 = 1u64;
                        nat_rem(a3347, a3348, a3349)
                    };
                    let a3351 = 0u64;
                    nat_eq(a3350, a3351)
                } else {
                    false
                } {
                    let a3352 = v6;
                    let a3353 = f16(v3.clone(), v5)?;
                    nat_eq(a3352, a3353)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3354) => {
                            h3354
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

pub fn f116(v0: u64, v1: u64) -> u64 {
    let a3355 = v0;
    let a3356 = 0u64;
    if if nat_eq(a3355, a3356) {
        true
    } else {
        let a3357 = v1;
        let a3358 = 0u64;
        nat_eq(a3357, a3358)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f117(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f34(v4, v5, v6, v7) {
        f34(v0, v1, v2, v3)
    } else {
        let a3362 = {
            let a3359 = v0;
            let a3360 = v4;
            let a3361 = 1u64;
            nat_rem(a3359, a3360, a3361)
        };
        let a3363 = 0u64;
        if if if nat_eq(a3362, a3363) {
            let a3367 = {
                let a3364 = v1;
                let a3365 = v5;
                let a3366 = 1u64;
                nat_rem(a3364, a3365, a3366)
            };
            let a3368 = 0u64;
            nat_eq(a3367, a3368)
        } else {
            false
        } {
            let a3372 = {
                let a3369 = v2;
                let a3370 = v6;
                let a3371 = 1u64;
                nat_rem(a3369, a3370, a3371)
            };
            let a3373 = 0u64;
            nat_eq(a3372, a3373)
        } else {
            false
        } {
            let a3377 = {
                let a3374 = v3;
                let a3375 = v7;
                let a3376 = 1u64;
                nat_rem(a3374, a3375, a3376)
            };
            let a3378 = 0u64;
            nat_eq(a3377, a3378)
        } else {
            false
        }
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3379 = v0;
    let a3380 = v1;
    if nat_eq(a3379, a3380) {
        let a3383 = v4;
        let a3384 = {
            let a3381 = v2;
            let a3382 = v3;
            nat_add(a3381, a3382)?
        };
        Ok(nat_eq(a3383, a3384))
    } else {
        let a3385 = v2;
        let a3386 = v3;
        if nat_eq(a3385, a3386) {
            let a3387 = v4;
            let a3388 = v2;
            Ok(nat_eq(a3387, a3388))
        } else {
            Ok(false)
        }
    }
}

pub fn f119(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3391 = {
            let a3389 = v0;
            let a3390 = f133(v1);
            nat_add(a3389, a3390)?
        };
        let a3392 = f133(v2);
        nat_add(a3391, a3392)?
    };
    let v5: u64 = {
        let a3393 = f132(v1);
        let a3394 = f132(v2);
        nat_add(a3393, a3394)?
    };
    let a3395 = v1;
    let a3396 = 4294967295u64;
    if if nat_le(a3395, a3396) {
        let a3397 = v2;
        let a3398 = 4294967295u64;
        nat_le(a3397, a3398)
    } else {
        false
    } {
        let a3399 = v5;
        let a3400 = v4;
        if nat_le(a3399, a3400) {
            let a3403 = v3;
            let a3404 = {
                let a3401 = v4;
                let a3402 = v5;
                nat_sub(a3401, a3402)
            };
            Ok(nat_eq(a3403, a3404))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f120(v0: u64, v1: u64) -> R<bool> {
    let a3405 = v1;
    let a3406 = 4294967295u64;
    if nat_le(a3405, a3406) {
        let a3409 = {
            let a3407 = f133(v1);
            let a3408 = f132(v1);
            nat_add(a3407, a3408)?
        };
        let a3410 = v0;
        Ok(nat_lt(a3409, a3410))
    } else {
        Ok(false)
    }
}

pub fn f121(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f194(v0.clone(), false)?;
    let v6: u64 = f194(v0, true)?;
    let a3411 = v6;
    let a3412 = v1;
    if if nat_le(a3411, a3412) {
        let a3419 = {
            let a3417 = {
                let a3413 = v1;
                let a3414 = v5;
                nat_add(a3413, a3414)?
            };
            let a3418 = {
                let a3415 = v3;
                let a3416 = 0u64;
                if nat_eq(a3415, a3416) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3417, a3418)?
        };
        let a3420 = v2;
        nat_le(a3419, a3420)
    } else {
        false
    } {
        if v4 {
            let a3425 = {
                let a3423 = {
                    let a3421 = v1;
                    let a3422 = v5;
                    nat_add(a3421, a3422)?
                };
                let a3424 = v6;
                nat_sub(a3423, a3424)
            };
            let a3426 = v2;
            Ok(nat_lt(a3425, a3426))
        } else {
            let a3431 = {
                let a3429 = {
                    let a3427 = v1;
                    let a3428 = v5;
                    nat_add(a3427, a3428)?
                };
                let a3430 = v6;
                nat_sub(a3429, a3430)
            };
            let a3432 = v2;
            Ok(nat_le(a3431, a3432))
        }
    } else {
        Ok(false)
    }
}

pub fn f122(v0: u64, v1: u64) -> bool {
    let a3433 = v0;
    let a3434 = 0u64;
    if if nat_eq(a3433, a3434) {
        true
    } else {
        let a3435 = v0;
        let a3436 = 1u64;
        nat_eq(a3435, a3436)
    } {
        let a3437 = v1;
        let a3438 = 4294967296u64;
        nat_lt(a3437, a3438)
    } else {
        false
    }
}

pub fn f123(v0: u64, v1: u64) -> R<u64> {
    let a3443 = {
        let a3441 = v0;
        let a3442 = {
            let a3439 = v1;
            let a3440 = 1u64;
            nat_sub(a3439, a3440)
        };
        nat_add(a3441, a3442)?
    };
    let a3444 = v1;
    let a3445 = 0u64;
    Ok(nat_quot(a3443, a3444, a3445))
}

pub fn f124(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h3446, _, _, _, _, _, _, _, _, _) => {
            h3446
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3447, _, _, _, _, _, _, _, _) => {
            h3447
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3448, _, _, _, _, _, _, _) => {
            h3448
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3449, _, _, _, _, _, _) => {
            h3449
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3452 = match v1.clone() {
                Adt2::C0(_, h3450, _, _, _, _, _, _, _, _, _) => {
                    h3450
                }
            };
            let a3453 = match v1.clone() {
                Adt2::C0(_, _, h3451, _, _, _, _, _, _, _, _) => {
                    h3451
                }
            };
            nat_mul(a3452, a3453)?
        };
        let v5: u64 = {
            let a3455 = v4;
            let a3456 = match v1.clone() {
                Adt2::C0(_, _, _, h3454, _, _, _, _, _, _, _) => {
                    h3454
                }
            };
            nat_mul(a3455, a3456)?
        };
        let a3469 = {
            let a3467 = {
                let a3458 = match v0.clone() {
                    Adt2::C0(_, h3457, _, _, _, _, _, _, _, _, _) => {
                        h3457
                    }
                };
                let a3459 = 1u64;
                nat_sub(a3458, a3459)
            };
            let a3468 = {
                let a3465 = f195(match v0.clone() {
                    Adt2::C0(_, _, h3460, _, _, _, _, _, _, _, _) => {
                        h3460
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3461, _, _, _, _, _, _, _, _, _) => {
                        h3461
                    }
                })?;
                let a3466 = f195(match v0 {
                    Adt2::C0(_, _, _, h3462, _, _, _, _, _, _, _) => {
                        h3462
                    }
                }, v2, {
                    let a3463 = v3;
                    let a3464 = v5;
                    nat_mul(a3463, a3464)?
                }, v4)?;
                nat_add(a3465, a3466)?
            };
            nat_add(a3467, a3468)?
        };
        let a3470 = f16(v1, 1u64)?;
        Ok(nat_lt(a3469, a3470))
    }
}

pub fn f125(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3471 = v0;
    let a3472 = 2147483648u64;
    if if if if nat_lt(a3471, a3472) {
        let a3473 = v1;
        let a3474 = 2147483648u64;
        nat_lt(a3473, a3474)
    } else {
        false
    } {
        let a3477 = {
            let a3475 = v0;
            let a3476 = v1;
            nat_mul(a3475, a3476)?
        };
        let a3478 = 2147483648u64;
        nat_lt(a3477, a3478)
    } else {
        false
    } {
        let a3483 = {
            let a3481 = {
                let a3479 = v0;
                let a3480 = 1u64;
                nat_sub(a3479, a3480)
            };
            let a3482 = v2;
            nat_mul(a3481, a3482)?
        };
        let a3484 = 2147483648u64;
        nat_lt(a3483, a3484)
    } else {
        false
    } {
        let a3489 = {
            let a3487 = {
                let a3485 = v1;
                let a3486 = 1u64;
                nat_sub(a3485, a3486)
            };
            let a3488 = v2;
            nat_mul(a3487, a3488)?
        };
        let a3490 = 2147483648u64;
        Ok(nat_lt(a3489, a3490))
    } else {
        Ok(false)
    }
}

pub fn f126(v0: u64) -> R<u64> {
    let a3493 = 3u64;
    let a3494 = {
        let a3491 = v0;
        let a3492 = 3u64;
        nat_mul(a3491, a3492)?
    };
    nat_add(a3493, a3494)
}

pub fn f127(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h3495, _, _, _, _, _, _, _) => {
            h3495
        }
    }, match v0.clone() {
        Adt3::C0(_, h3496, _, _, _, _, _, _) => {
            h3496
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3497, _, _, _, _, _) => {
            h3497
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3498, _, _, _, _) => {
            h3498
        }
    }) {
        Ok(0u64)
    } else {
        let a3507 = match v0.clone() {
            Adt3::C0(h3499, _, _, _, _, _, _, _) => {
                h3499
            }
        };
        let a3508 = {
            let a3505 = match v0.clone() {
                Adt3::C0(_, h3500, _, _, _, _, _, _) => {
                    h3500
                }
            };
            let a3506 = {
                let a3503 = match v0.clone() {
                    Adt3::C0(_, _, h3501, _, _, _, _, _) => {
                        h3501
                    }
                };
                let a3504 = match v0 {
                    Adt3::C0(_, _, _, h3502, _, _, _, _) => {
                        h3502
                    }
                };
                nat_mul(a3503, a3504)?
            };
            nat_mul(a3505, a3506)?
        };
        nat_mul(a3507, a3508)
    }
}

pub fn f128(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3509 = v1;
    let a3510 = 13u64;
    if if nat_eq(a3509, a3510) {
        f34(match v0.clone() {
            Adt3::C0(h3511, _, _, _, _, _, _, _) => {
                h3511
            }
        }, match v0.clone() {
            Adt3::C0(_, h3512, _, _, _, _, _, _) => {
                h3512
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3513, _, _, _, _, _) => {
                h3513
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3514, _, _, _, _) => {
                h3514
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3520 = match v0.clone() {
                Adt3::C0(_, h3515, _, _, _, _, _, _) => {
                    h3515
                }
            };
            let a3521 = {
                let a3518 = match v0.clone() {
                    Adt3::C0(_, _, h3516, _, _, _, _, _) => {
                        h3516
                    }
                };
                let a3519 = match v0.clone() {
                    Adt3::C0(_, _, _, h3517, _, _, _, _) => {
                        h3517
                    }
                };
                nat_mul(a3518, a3519)?
            };
            nat_mul(a3520, a3521)?
        };
        let a3522 = v1;
        let a3523 = 13u64;
        if nat_eq(a3522, a3523) {
            let a3532 = 2u64;
            let a3533 = if f196(v0.clone(), v2) {
                let a3525 = v3;
                let a3526 = f197(match v0 {
                    Adt3::C0(h3524, _, _, _, _, _, _, _) => {
                        h3524
                    }
                }, false)?;
                nat_mul(a3525, a3526)?
            } else {
                let a3530 = v3;
                let a3531 = {
                    let a3528 = match v0 {
                        Adt3::C0(h3527, _, _, _, _, _, _, _) => {
                            h3527
                        }
                    };
                    let a3529 = 2u64;
                    nat_mul(a3528, a3529)?
                };
                nat_mul(a3530, a3531)?
            };
            nat_add(a3532, a3533)
        } else {
            let a3537 = v3;
            let a3538 = f197(match v0 {
                Adt3::C0(h3534, _, _, _, _, _, _, _) => {
                    h3534
                }
            }, {
                let a3535 = v1;
                let a3536 = 16u64;
                nat_eq(a3535, a3536)
            })?;
            nat_mul(a3537, a3538)
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3539 = v0;
    let a3540 = 30u64;
    if if nat_eq(a3539, a3540) {
        let a3543 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3541, _, _, _, _) => {
                h3541
            }
        };
        let a3544 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3542, _, _, _, _, _) => {
                h3542
            }
        };
        nat_lt(a3543, a3544)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3545 = v3;
        match m3545 {
            None => {
                f198(v0, v1, match v2.clone() {
                    Adt2::C0(h3546, _, _, _, _, _, _, _, _, _, _) => {
                        h3546
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3547, _, _, _, _, _, _, _, _, _, _) => {
                        h3547
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3548, _, _, _, _, _, _, _, _, _) => {
                        h3548
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3549, _, _, _, _, _, _, _, _) => {
                        h3549
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3550, _, _, _, _, _, _, _) => {
                        h3550
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3551, _, _, _, _, _, _) => {
                        h3551
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3552, _, _, _, _, _, _, _, _, _) => {
                        h3552
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3553, _, _, _, _, _, _, _, _) => {
                        h3553
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3554, _, _, _, _, _, _, _) => {
                        h3554
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3555, _, _, _, _, _, _) => {
                        h3555
                    }
                })
            }
            Some(v5) => {
                f198(v0, v1, match v2.clone() {
                    Adt2::C0(h3556, _, _, _, _, _, _, _, _, _, _) => {
                        h3556
                    }
                }, match v5.clone() {
                    Adt2::C0(h3557, _, _, _, _, _, _, _, _, _, _) => {
                        h3557
                    }
                }, match v4.clone() {
                    Adt2::C0(h3558, _, _, _, _, _, _, _, _, _, _) => {
                        h3558
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3559, _, _, _, _, _, _, _, _, _) => {
                        h3559
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3560, _, _, _, _, _, _, _, _) => {
                        h3560
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3561, _, _, _, _, _, _, _) => {
                        h3561
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3562, _, _, _, _, _, _) => {
                        h3562
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3563, _, _, _, _, _, _, _, _, _) => {
                        h3563
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3564, _, _, _, _, _, _, _, _) => {
                        h3564
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3565, _, _, _, _, _, _, _) => {
                        h3565
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3566, _, _, _, _, _, _) => {
                        h3566
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3567, _, _, _, _, _, _, _, _, _) => {
                        h3567
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3568, _, _, _, _, _, _, _, _) => {
                        h3568
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3569, _, _, _, _, _, _, _) => {
                        h3569
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3570, _, _, _, _, _, _) => {
                        h3570
                    }
                })
            }
        }
    }
}

pub fn f130(v0: u64, v1: u64) -> u64 {
    let a3571 = v0;
    let a3572 = 23u64;
    if nat_eq(a3571, a3572) {
        let a3573 = v1;
        let a3574 = 0u64;
        if nat_eq(a3573, a3574) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3587 = {
        let a3583 = {
            let a3579 = {
                let a3575 = v0;
                let a3576 = v4;
                nat_mul(a3575, a3576)?
            };
            let a3580 = {
                let a3577 = v1;
                let a3578 = v5;
                nat_mul(a3577, a3578)?
            };
            nat_add(a3579, a3580)?
        };
        let a3584 = {
            let a3581 = v2;
            let a3582 = v6;
            nat_mul(a3581, a3582)?
        };
        nat_add(a3583, a3584)?
    };
    let a3588 = {
        let a3585 = v3;
        let a3586 = v7;
        nat_mul(a3585, a3586)?
    };
    nat_add(a3587, a3588)
}

pub fn f132(v0: u64) -> u64 {
    let a3589 = v0;
    let a3590 = 2147483648u64;
    if nat_lt(a3589, a3590) {
        0u64
    } else {
        let a3591 = 4294967296u64;
        let a3592 = v0;
        nat_sub(a3591, a3592)
    }
}

pub fn f133(v0: u64) -> u64 {
    let a3593 = v0;
    let a3594 = 2147483648u64;
    if nat_lt(a3593, a3594) {
        v0
    } else {
        0u64
    }
}

pub fn f134(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3595 = v7;
        let a3596 = 6u64;
        if nat_eq(a3595, a3596) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f90(v3, v4), f90(v5, v6), f91(v7, v8), f199(v1), f199(v9))
}

pub fn f135(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3597 = v0;
    match m3597 {
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
        Adt4::C13(v32, v33, v34, v35) => {
            Adt11::C4(v32, v33, v34, v35)
        }
        Adt4::C14(v36, _, _) => {
            let a3598 = 13u64;
            let a3599 = v36;
            if nat_le(a3598, a3599) {
                Adt11::C7(v36, v1, v2)
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
        Adt4::C27(v56) => {
            Adt11::C2(match v56.clone() {
                Adt5::C0(_, _, _, _, _, h3600, _, _) => {
                    h3600
                }
            }, match v56 {
                Adt5::C0(_, _, _, _, _, _, _, h3601) => {
                    h3601
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
    }
}

pub fn f136(v0: Adt4) -> u64 {
    let m3602 = v0;
    match m3602 {
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
        Adt4::C14(v34, _, _) => {
            v34
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
    }
}

pub fn f137(v0: Adt4, v1: u64) -> u64 {
    let m3603 = v0;
    match m3603 {
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
        Adt4::C16(_, v39) => {
            v39
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v42, _) => {
            let a3604 = v1;
            let a3605 = 6u64;
            if nat_eq(a3604, a3605) {
                v42
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
        Adt4::C26(v56, _, _, _) => {
            let a3614 = v1;
            let a3615 = 6u64;
            if if if nat_eq(a3614, a3615) {
                true
            } else {
                let a3616 = v1;
                let a3617 = 7u64;
                nat_eq(a3616, a3617)
            } {
                let a3618 = v56;
                let a3619 = 30u64;
                nat_eq(a3618, a3619)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v55) => {
            match v55 {
                Adt5::C0(_, _, _, _, _, _, _, h3613) => {
                    h3613
                }
            }
        }
        Adt4::C28(v51) => {
            let a3606 = v1;
            let a3607 = 6u64;
            if nat_eq(a3606, a3607) {
                match v51 {
                    Adt7::C0(_, _, _, _, h3608) => {
                        h3608
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v54) => {
            let a3609 = v1;
            let a3610 = 6u64;
            if if nat_eq(a3609, a3610) {
                true
            } else {
                let a3611 = v1;
                let a3612 = 7u64;
                nat_eq(a3611, a3612)
            } {
                v54
            } else {
                0u64
            }
        }
        Adt4::C31(_, _) => {
            0u64
        }
    }
}

pub fn f138(v0: Adt4, v1: u64) -> Adt11 {
    let a3620 = v1;
    let a3621 = 4u64;
    if nat_lt(a3620, a3621) {
        Adt11::C0(v1)
    } else {
        let a3622 = v1;
        let a3623 = 4u64;
        if nat_eq(a3622, a3623) {
            Adt11::C1(0u64)
        } else {
            let a3624 = v1;
            let a3625 = 5u64;
            if nat_eq(a3624, a3625) {
                Adt11::C1(3u64)
            } else {
                let a3626 = v1;
                let a3627 = 6u64;
                if nat_eq(a3626, a3627) {
                    Adt11::C2(f137(v0, v1), 0u64, false)
                } else {
                    let a3628 = v1;
                    let a3629 = 7u64;
                    if nat_eq(a3628, a3629) {
                        Adt11::C2(0u64, f137(v0, v1), false)
                    } else {
                        let a3630 = v1;
                        let a3631 = 16u64;
                        if nat_lt(a3630, a3631) {
                            Adt11::C1({
                                let a3632 = v1;
                                let a3633 = 8u64;
                                nat_sub(a3632, a3633)
                            })
                        } else {
                            let a3634 = v1;
                            let a3635 = 20u64;
                            if nat_lt(a3634, a3635) {
                                Adt11::C5({
                                    let a3636 = v1;
                                    let a3637 = 16u64;
                                    nat_sub(a3636, a3637)
                                })
                            } else {
                                let a3638 = v1;
                                let a3639 = 20u64;
                                if nat_eq(a3638, a3639) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3640 = v1;
                                    let a3641 = 21u64;
                                    if nat_eq(a3640, a3641) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3642 = v1;
                                        let a3643 = 22u64;
                                        if nat_eq(a3642, a3643) {
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

pub fn f139(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3645 = v0;
        let a3646 = match v1.clone() {
            Adt3::C0(h3644, _, _, _, _, _, _, _) => {
                h3644
            }
        };
        let a3647 = 0u64;
        nat_quot(a3645, a3646, a3647)
    };
    let v3: u64 = {
        let a3649 = v2;
        let a3650 = match v1.clone() {
            Adt3::C0(_, h3648, _, _, _, _, _, _) => {
                h3648
            }
        };
        let a3651 = 0u64;
        nat_quot(a3649, a3650, a3651)
    };
    f131({
        let a3653 = v0;
        let a3654 = match v1.clone() {
            Adt3::C0(h3652, _, _, _, _, _, _, _) => {
                h3652
            }
        };
        let a3655 = 0u64;
        nat_rem(a3653, a3654, a3655)
    }, {
        let a3657 = v2;
        let a3658 = match v1.clone() {
            Adt3::C0(_, h3656, _, _, _, _, _, _) => {
                h3656
            }
        };
        let a3659 = 0u64;
        nat_rem(a3657, a3658, a3659)
    }, {
        let a3661 = v3;
        let a3662 = match v1.clone() {
            Adt3::C0(_, _, h3660, _, _, _, _, _) => {
                h3660
            }
        };
        let a3663 = 0u64;
        nat_rem(a3661, a3662, a3663)
    }, {
        let a3665 = v3;
        let a3666 = match v1.clone() {
            Adt3::C0(_, _, h3664, _, _, _, _, _) => {
                h3664
            }
        };
        let a3667 = 0u64;
        nat_quot(a3665, a3666, a3667)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3668, _, _, _) => {
            h3668
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3669, _, _) => {
            h3669
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3670, _) => {
            h3670
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3671) => {
            h3671
        }
    })
}

pub fn f140(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3689 = {
        let a3684 = f200(v2, {
            let a3675 = {
                let a3672 = v0;
                let a3673 = v1;
                let a3674 = 0u64;
                nat_quot(a3672, a3673, a3674)
            };
            let a3676 = v2;
            let a3677 = 0u64;
            nat_rem(a3675, a3676, a3677)
        }, {
            let a3681 = {
                let a3678 = v0;
                let a3679 = v1;
                let a3680 = 0u64;
                nat_quot(a3678, a3679, a3680)
            };
            let a3682 = v2;
            let a3683 = 0u64;
            nat_quot(a3681, a3682, a3683)
        })?;
        let a3685 = v1;
        nat_mul(a3684, a3685)?
    };
    let a3690 = {
        let a3686 = v0;
        let a3687 = v1;
        let a3688 = 0u64;
        nat_rem(a3686, a3687, a3688)
    };
    nat_add(a3689, a3690)
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3692 = v0;
        let a3693 = match v2.clone() {
            Adt3::C0(h3691, _, _, _, _, _, _, _) => {
                h3691
            }
        };
        let a3694 = 0u64;
        nat_quot(a3692, a3693, a3694)
    };
    let v4: u64 = {
        let a3696 = v3;
        let a3697 = match v2.clone() {
            Adt3::C0(_, h3695, _, _, _, _, _, _) => {
                h3695
            }
        };
        let a3698 = 0u64;
        nat_quot(a3696, a3697, a3698)
    };
    f201({
        let a3700 = v0;
        let a3701 = match v2.clone() {
            Adt3::C0(h3699, _, _, _, _, _, _, _) => {
                h3699
            }
        };
        let a3702 = 0u64;
        nat_rem(a3700, a3701, a3702)
    }, {
        let a3704 = v3;
        let a3705 = match v2.clone() {
            Adt3::C0(_, h3703, _, _, _, _, _, _) => {
                h3703
            }
        };
        let a3706 = 0u64;
        nat_rem(a3704, a3705, a3706)
    }, {
        let a3708 = v4;
        let a3709 = match v2.clone() {
            Adt3::C0(_, _, h3707, _, _, _, _, _) => {
                h3707
            }
        };
        let a3710 = 0u64;
        nat_rem(a3708, a3709, a3710)
    }, {
        let a3712 = v4;
        let a3713 = match v2 {
            Adt3::C0(_, _, h3711, _, _, _, _, _) => {
                h3711
            }
        };
        let a3714 = 0u64;
        nat_quot(a3712, a3713, a3714)
    }, match v1.clone() {
        Adt3::C0(h3715, _, _, _, _, _, _, _) => {
            h3715
        }
    }, match v1.clone() {
        Adt3::C0(_, h3716, _, _, _, _, _, _) => {
            h3716
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3717, _, _, _, _, _) => {
            h3717
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3718, _, _, _, _) => {
            h3718
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3719, _, _, _) => {
            h3719
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3720, _, _) => {
            h3720
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3721, _) => {
            h3721
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3722) => {
            h3722
        }
    })
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3724 = v0;
        let a3725 = match v3.clone() {
            Adt3::C0(h3723, _, _, _, _, _, _, _) => {
                h3723
            }
        };
        let a3726 = 0u64;
        nat_quot(a3724, a3725, a3726)
    };
    let v6: u64 = {
        let a3728 = v5;
        let a3729 = match v3.clone() {
            Adt3::C0(_, h3727, _, _, _, _, _, _) => {
                h3727
            }
        };
        let a3730 = 0u64;
        nat_quot(a3728, a3729, a3730)
    };
    let v7: u64 = {
        let a3732 = v0;
        let a3733 = match v3.clone() {
            Adt3::C0(h3731, _, _, _, _, _, _, _) => {
                h3731
            }
        };
        let a3734 = 0u64;
        nat_rem(a3732, a3733, a3734)
    };
    let v8: u64 = {
        let a3736 = v5;
        let a3737 = match v3.clone() {
            Adt3::C0(_, h3735, _, _, _, _, _, _) => {
                h3735
            }
        };
        let a3738 = 0u64;
        nat_rem(a3736, a3737, a3738)
    };
    let v9: u64 = {
        let a3740 = v6;
        let a3741 = match v3.clone() {
            Adt3::C0(_, _, h3739, _, _, _, _, _) => {
                h3739
            }
        };
        let a3742 = 0u64;
        nat_rem(a3740, a3741, a3742)
    };
    let v10: u64 = {
        let a3744 = v6;
        let a3745 = match v3.clone() {
            Adt3::C0(_, _, h3743, _, _, _, _, _) => {
                h3743
            }
        };
        let a3746 = 0u64;
        nat_quot(a3744, a3745, a3746)
    };
    let v11: u64 = {
        let a3747 = v4;
        let a3748 = 0u64;
        if nat_eq(a3747, a3748) {
            match v1.clone() {
                Adt3::C0(h3749, _, _, _, _, _, _, _) => {
                    h3749
                }
            }
        } else {
            let a3750 = v4;
            let a3751 = 1u64;
            if nat_eq(a3750, a3751) {
                match v1.clone() {
                    Adt3::C0(_, h3752, _, _, _, _, _, _) => {
                        h3752
                    }
                }
            } else {
                let a3753 = v4;
                let a3754 = 2u64;
                if nat_eq(a3753, a3754) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3755, _, _, _, _, _) => {
                            h3755
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3756, _, _, _, _) => {
                            h3756
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3763 = v11;
        let a3764 = {
            let a3757 = v4;
            let a3758 = 0u64;
            if nat_eq(a3757, a3758) {
                v7
            } else {
                let a3759 = v4;
                let a3760 = 1u64;
                if nat_eq(a3759, a3760) {
                    v8
                } else {
                    let a3761 = v4;
                    let a3762 = 2u64;
                    if nat_eq(a3761, a3762) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3763, a3764)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f131(if if v12 {
        let a3765 = v4;
        let a3766 = 0u64;
        nat_eq(a3765, a3766)
    } else {
        false
    } {
        let a3767 = v7;
        let a3768 = v11;
        nat_sub(a3767, a3768)
    } else {
        v7
    }, if if v12 {
        let a3769 = v4;
        let a3770 = 1u64;
        nat_eq(a3769, a3770)
    } else {
        false
    } {
        let a3771 = v8;
        let a3772 = v11;
        nat_sub(a3771, a3772)
    } else {
        v8
    }, if if v12 {
        let a3773 = v4;
        let a3774 = 2u64;
        nat_eq(a3773, a3774)
    } else {
        false
    } {
        let a3775 = v9;
        let a3776 = v11;
        nat_sub(a3775, a3776)
    } else {
        v9
    }, if if v12 {
        let a3777 = v4;
        let a3778 = 3u64;
        nat_eq(a3777, a3778)
    } else {
        false
    } {
        let a3779 = v10;
        let a3780 = v11;
        nat_sub(a3779, a3780)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3781, _, _, _) => {
            h3781
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3782, _, _) => {
            h3782
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3783, _) => {
            h3783
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3784) => {
            h3784
        }
    })?, f139(v0, v3)?))
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f70(v0, v2.clone(), 0u64);
    let v9: u64 = f70(v0, v2.clone(), 1u64);
    let v10: u64 = f70(v0, v2.clone(), 2u64);
    let v11: u64 = f70(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f69(v1.clone(), f202(v8, v3, match v1.clone() {
            Adt3::C0(h3785, _, _, _, _, _, _, _) => {
                h3785
            }
        })?, f202(v9, v4, match v1.clone() {
            Adt3::C0(_, h3786, _, _, _, _, _, _) => {
                h3786
            }
        })?, f202(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3787, _, _, _, _, _) => {
                h3787
            }
        })?, f202(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3788, _, _, _, _) => {
                h3788
            }
        })?)?, f139(v0, v2)?))
    } else {
        if if if if f203(v8, v3, match v1.clone() {
            Adt3::C0(h3789, _, _, _, _, _, _, _) => {
                h3789
            }
        })? {
            f203(v9, v4, match v1.clone() {
                Adt3::C0(_, h3790, _, _, _, _, _, _) => {
                    h3790
                }
            })?
        } else {
            false
        } {
            f203(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3791, _, _, _, _, _) => {
                    h3791
                }
            })?
        } else {
            false
        } {
            f203(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3792, _, _, _, _) => {
                    h3792
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f69(v1, f204(v8, v3)?, f204(v9, v4)?, f204(v10, v5)?, f204(v11, v6)?)?, f139(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f139(v0, v2)?))
        }
    }
}

pub fn f144(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f70(v0, v2.clone(), 0u64);
    let v5: u64 = f70(v0, v2.clone(), 1u64);
    let v6: u64 = f70(v0, v2.clone(), 2u64);
    let v7: u64 = f70(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f69(v1.clone(), f205(v4, v3, match v1 {
        Adt3::C0(h3793, _, _, _, _, _, _, _) => {
            h3793
        }
    })?, v5, v6, v7)?, f139(v0, v2)?))
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3794 = v0;
    let a3795 = v3;
    if nat_lt(a3794, a3795) {
        Ok(Adt9::C1(false, {
            let a3796 = v0;
            let a3797 = 4u64;
            nat_mul(a3796, a3797)?
        }, {
            let a3798 = v0;
            let a3799 = 4u64;
            nat_mul(a3798, a3799)?
        }))
    } else {
        let v6: u64 = {
            let a3800 = v0;
            let a3801 = v3;
            nat_sub(a3800, a3801)
        };
        let v7: u64 = f139(v6, v1)?;
        let v8: u64 = f206(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f146(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3802 = v0;
    let a3803 = v3;
    if nat_lt(a3802, a3803) {
        Ok(Adt9::C1(true, 0u64, f139(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3804 = v0;
            let a3805 = v3;
            nat_sub(a3804, a3805)
        };
        let v5: u64 = f141(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f139(v4, v1)?, 3u64, v5))
    }
}

pub fn f147(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3807 = v1;
        let a3808 = match v3.clone() {
            Adt3::C0(h3806, _, _, _, _, _, _, _) => {
                h3806
            }
        };
        let a3809 = 0u64;
        nat_rem(a3807, a3808, a3809)
    };
    let v5: u64 = {
        let a3815 = {
            let a3811 = v1;
            let a3812 = match v3.clone() {
                Adt3::C0(h3810, _, _, _, _, _, _, _) => {
                    h3810
                }
            };
            let a3813 = 0u64;
            nat_quot(a3811, a3812, a3813)
        };
        let a3816 = match v3.clone() {
            Adt3::C0(_, h3814, _, _, _, _, _, _) => {
                h3814
            }
        };
        let a3817 = 0u64;
        nat_rem(a3815, a3816, a3817)
    };
    let v6: bool = f207(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f141(v1, v2, v3.clone())?
    } else {
        0u64
    }, f139(v1, v3)?))
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3818 = v3;
        let a3819 = 8u64;
        nat_add(a3818, a3819)?
    }, 0u64, f139(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f141(v0, v5, v2.clone())?
    }, 3u64, f139(v0, v2)?))
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3820 = v4;
    let a3821 = 0u64;
    if if nat_eq(a3820, a3821) {
        f50(v3)
    } else {
        false
    } {
        let m3822 = f114(v3);
        match m3822 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h3823, _) => {
                        h3823
                    }
                };
                Ok(Adt9::C5(v3, f139(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h3824) => {
                        h3824
                    }
                }, f139({
                    let a3825 = v0;
                    let a3826 = v6;
                    nat_mul(a3825, a3826)?
                }, v2.clone())?, {
                    let a3832 = {
                        let a3830 = {
                            let a3827 = v6;
                            let a3828 = 1u64;
                            nat_sub(a3827, a3828)
                        };
                        let a3831 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3829, _, _, _) => {
                                h3829
                            }
                        };
                        nat_mul(a3830, a3831)?
                    };
                    let a3833 = 4u64;
                    nat_add(a3832, a3833)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3834, _, _, _) => {
                        h3834
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f139(v0, v1)?, f139(v0, v2)?))
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3836 = v0;
        let a3837 = match v2.clone() {
            Adt3::C0(h3835, _, _, _, _, _, _, _) => {
                h3835
            }
        };
        let a3838 = 0u64;
        nat_rem(a3836, a3837, a3838)
    };
    let v7: u64 = {
        let a3840 = v0;
        let a3841 = match v2.clone() {
            Adt3::C0(h3839, _, _, _, _, _, _, _) => {
                h3839
            }
        };
        let a3842 = 0u64;
        nat_quot(a3840, a3841, a3842)
    };
    let v8: u64 = {
        let a3844 = v7;
        let a3845 = match v2.clone() {
            Adt3::C0(_, h3843, _, _, _, _, _, _) => {
                h3843
            }
        };
        let a3846 = 0u64;
        nat_rem(a3844, a3845, a3846)
    };
    let v9: u64 = {
        let a3848 = v7;
        let a3849 = match v2.clone() {
            Adt3::C0(_, h3847, _, _, _, _, _, _) => {
                h3847
            }
        };
        let a3850 = 0u64;
        nat_quot(a3848, a3849, a3850)
    };
    let v10: u64 = {
        let a3852 = v9;
        let a3853 = match v2.clone() {
            Adt3::C0(_, _, h3851, _, _, _, _, _) => {
                h3851
            }
        };
        let a3854 = 0u64;
        nat_rem(a3852, a3853, a3854)
    };
    let v11: u64 = {
        let a3856 = v9;
        let a3857 = match v2.clone() {
            Adt3::C0(_, _, h3855, _, _, _, _, _) => {
                h3855
            }
        };
        let a3858 = 0u64;
        nat_quot(a3856, a3857, a3858)
    };
    let v12: u64 = f139(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3867 = {
                let a3862 = {
                    let a3859 = v10;
                    let a3860 = v4;
                    let a3861 = 0u64;
                    nat_quot(a3859, a3860, a3861)
                };
                let a3863 = v5;
                nat_mul(a3862, a3863)?
            };
            let a3868 = {
                let a3864 = v8;
                let a3865 = v4;
                let a3866 = 0u64;
                nat_quot(a3864, a3865, a3866)
            };
            nat_add(a3867, a3868)?
        };
        let v14: u64 = {
            let a3888 = v6;
            let a3889 = {
                let a3886 = match v1.clone() {
                    Adt3::C0(h3869, _, _, _, _, _, _, _) => {
                        h3869
                    }
                };
                let a3887 = {
                    let a3884 = {
                        let a3870 = v8;
                        let a3871 = v4;
                        let a3872 = 0u64;
                        nat_rem(a3870, a3871, a3872)
                    };
                    let a3885 = {
                        let a3882 = match v1.clone() {
                            Adt3::C0(_, h3873, _, _, _, _, _, _) => {
                                h3873
                            }
                        };
                        let a3883 = {
                            let a3880 = {
                                let a3874 = v10;
                                let a3875 = v4;
                                let a3876 = 0u64;
                                nat_rem(a3874, a3875, a3876)
                            };
                            let a3881 = {
                                let a3878 = match v1.clone() {
                                    Adt3::C0(_, _, h3877, _, _, _, _, _) => {
                                        h3877
                                    }
                                };
                                let a3879 = v13;
                                nat_mul(a3878, a3879)?
                            };
                            nat_add(a3880, a3881)?
                        };
                        nat_mul(a3882, a3883)?
                    };
                    nat_add(a3884, a3885)?
                };
                nat_mul(a3886, a3887)?
            };
            nat_add(a3888, a3889)?
        };
        Ok(Adt9::C1(false, f139(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3895 = {
                let a3893 = {
                    let a3890 = v11;
                    let a3891 = v5;
                    let a3892 = 0u64;
                    nat_rem(a3890, a3891, a3892)
                };
                let a3894 = v4;
                nat_mul(a3893, a3894)?
            };
            let a3896 = v8;
            nat_add(a3895, a3896)?
        };
        let v16: u64 = {
            let a3902 = {
                let a3900 = {
                    let a3897 = v11;
                    let a3898 = v5;
                    let a3899 = 0u64;
                    nat_quot(a3897, a3898, a3899)
                };
                let a3901 = v4;
                nat_mul(a3900, a3901)?
            };
            let a3903 = v10;
            nat_add(a3902, a3903)?
        };
        let a3905 = v15;
        let a3906 = match v1.clone() {
            Adt3::C0(_, h3904, _, _, _, _, _, _) => {
                h3904
            }
        };
        if if nat_lt(a3905, a3906) {
            let a3908 = v16;
            let a3909 = match v1.clone() {
                Adt3::C0(_, _, h3907, _, _, _, _, _) => {
                    h3907
                }
            };
            nat_lt(a3908, a3909)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f139({
                let a3918 = v6;
                let a3919 = {
                    let a3916 = match v1.clone() {
                        Adt3::C0(h3910, _, _, _, _, _, _, _) => {
                            h3910
                        }
                    };
                    let a3917 = {
                        let a3914 = v15;
                        let a3915 = {
                            let a3912 = match v1.clone() {
                                Adt3::C0(_, h3911, _, _, _, _, _, _) => {
                                    h3911
                                }
                            };
                            let a3913 = v16;
                            nat_mul(a3912, a3913)?
                        };
                        nat_add(a3914, a3915)?
                    };
                    nat_mul(a3916, a3917)?
                };
                nat_add(a3918, a3919)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f130(23u64, v4);
    let v6: u64 = {
        let a3920 = v0;
        let a3921 = v5;
        let a3922 = 0u64;
        nat_rem(a3920, a3921, a3922)
    };
    let v7: u64 = {
        let a3923 = v0;
        let a3924 = v5;
        let a3925 = 0u64;
        nat_quot(a3923, a3924, a3925)
    };
    let v8: Adt13 = Adt13::C1(f139(v7, v2)?);
    let a3926 = v6;
    let a3927 = 0u64;
    if nat_eq(a3926, a3927) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f139(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f199(v3), 4u64))
    } else {
        let a3928 = v6;
        let a3929 = 1u64;
        if nat_eq(a3928, a3929) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a3930 = v4;
                let a3931 = 0u64;
                if nat_eq(a3930, a3931) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f199(v4)))
        }
    }
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f139(v0, v2.clone())?;
    let v4: u64 = f139(v0, v1.clone())?;
    let a3936 = {
        let a3933 = v0;
        let a3934 = match v1 {
            Adt3::C0(h3932, _, _, _, _, _, _, _) => {
                h3932
            }
        };
        let a3935 = 0u64;
        nat_rem(a3933, a3934, a3935)
    };
    let a3937 = 0u64;
    if nat_eq(a3936, a3937) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f139({
            let a3938 = v0;
            let a3939 = 1u64;
            nat_sub(a3938, a3939)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3940 = v3;
    let a3941 = 13u64;
    if nat_eq(a3940, a3941) {
        f208(v0, v1, v4)
    } else {
        let v5: u64 = f197(match v1.clone() {
            Adt3::C0(h3942, _, _, _, _, _, _, _) => {
                h3942
            }
        }, {
            let a3943 = v3;
            let a3944 = 16u64;
            nat_eq(a3943, a3944)
        })?;
        let v6: u64 = {
            let a3945 = v0;
            let a3946 = v5;
            let a3947 = 0u64;
            nat_quot(a3945, a3946, a3947)
        };
        let v7: u64 = {
            let a3948 = v0;
            let a3949 = v5;
            let a3950 = 0u64;
            nat_rem(a3948, a3949, a3950)
        };
        let v8: u64 = {
            let a3954 = 1u64;
            let a3955 = {
                let a3952 = match v1.clone() {
                    Adt3::C0(h3951, _, _, _, _, _, _, _) => {
                        h3951
                    }
                };
                let a3953 = 2u64;
                nat_mul(a3952, a3953)?
            };
            nat_add(a3954, a3955)?
        };
        let a3956 = v7;
        let a3957 = v8;
        if nat_lt(a3956, a3957) {
            f209(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f139(v6, v2)?;
            let a3958 = v3;
            let a3959 = 14u64;
            if nat_eq(a3958, a3959) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3960 = v7;
                let a3961 = v8;
                if nat_eq(a3960, a3961) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3964 = v7;
                    let a3965 = {
                        let a3962 = v8;
                        let a3963 = 1u64;
                        nat_add(a3962, a3963)?
                    };
                    if nat_eq(a3964, a3965) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3966, _, _, _, _, _, _, _) => {
                                h3966
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

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3967 = v0;
    let a3968 = 0u64;
    if nat_eq(a3967, a3968) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3975 = {
            let a3969 = v0;
            let a3970 = 1u64;
            nat_add(a3969, a3970)?
        };
        let a3976 = {
            let a3973 = 2u64;
            let a3974 = {
                let a3971 = v3;
                let a3972 = 2u64;
                nat_mul(a3971, a3972)?
            };
            nat_add(a3973, a3974)?
        };
        if nat_eq(a3975, a3976) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3977 = v0;
                let a3978 = 1u64;
                nat_sub(a3977, a3978)
            };
            let v5: u64 = {
                let a3979 = v4;
                let a3980 = 2u64;
                let a3981 = 0u64;
                nat_quot(a3979, a3980, a3981)
            };
            let a3985 = {
                let a3982 = v4;
                let a3983 = 2u64;
                let a3984 = 0u64;
                nat_rem(a3982, a3983, a3984)
            };
            let a3986 = 0u64;
            if nat_eq(a3985, a3986) {
                Ok(Adt9::C4(17u64, 0u64, f139(v5, v1)?, 4u64, f139(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f155(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3987 = v0;
    let a3988 = f27(v1, v2, v3)?;
    if nat_lt(a3987, a3988) {
        let a3989 = v0;
        let a3990 = v1;
        if nat_lt(a3989, a3990) {
            Ok(Adt9::C1(false, {
                let a3991 = 4u64;
                let a3992 = v0;
                nat_mul(a3991, a3992)?
            }, {
                let a3993 = 4u64;
                let a3994 = v0;
                nat_mul(a3993, a3994)?
            }))
        } else {
            let v4: u64 = {
                let a3995 = v0;
                let a3996 = v1;
                nat_sub(a3995, a3996)
            };
            let v5: u64 = f210(v4, v2)?;
            let a3997 = v5;
            let a3998 = 2u64;
            if nat_lt(a3997, a3998) {
                Ok(Adt9::C4(6u64, {
                    let a3999 = v5;
                    let a4000 = 0u64;
                    if nat_eq(a3999, a4000) {
                        4u64
                    } else {
                        5u64
                    }
                }, f211(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f212(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f213(v4, v2)? {
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

pub fn f156(v0: u64, v1: u64) -> Adt11 {
    let a4001 = v0;
    let a4002 = 43u64;
    if nat_eq(a4001, a4002) {
        Adt11::C8(3u64)
    } else {
        let a4003 = v1;
        let a4004 = 43u64;
        if nat_eq(a4003, a4004) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    if f104(v0) {
        let a4005 = 2u64;
        let a4006 = v1;
        nat_mul(a4005, a4006)
    } else {
        let a4007 = v1;
        let a4008 = f193(v0);
        let a4009 = 0u64;
        Ok(nat_quot(a4007, a4008, a4009))
    }
}

pub fn f158(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4010 = f114(v0);
    match m4010 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f131(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4011, _, _, _) => {
                    h4011
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4012, _, _) => {
                    h4012
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4013, _) => {
                    h4013
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4014) => {
                    h4014
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h4015) => {
                    h4015
                }
            }, f131({
                let a4017 = v2;
                let a4018 = match v6.clone() {
                    Adt18::C0(_, h4016, _) => {
                        h4016
                    }
                };
                nat_mul(a4017, a4018)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4019, _, _, _) => {
                    h4019
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4020, _, _) => {
                    h4020
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4021, _) => {
                    h4021
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4022) => {
                    h4022
                }
            })?, match v6 {
                Adt18::C0(_, h4023, _) => {
                    h4023
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4024, _, _, _) => {
                    h4024
                }
            }))
        }
    }
}

pub fn f159(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4032 = {
        let a4025 = 2u64;
        let a4026 = f127(v1.clone())?;
        nat_mul(a4025, a4026)?
    };
    let a4033 = {
        let a4030 = f127(v1)?;
        let a4031 = {
            let a4028 = 1u64;
            let a4029 = match v0 {
                Adt3::C0(h4027, _, _, _, _, _, _, _) => {
                    h4027
                }
            };
            nat_add(a4028, a4029)?
        };
        nat_mul(a4030, a4031)?
    };
    nat_add(a4032, a4033)
}

pub fn f160(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f139(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f161(v0: u64, v1: u64) -> bool {
    let a4034 = v1;
    let a4035 = 2147483647u64;
    if nat_le(a4034, a4035) {
        let a4036 = v1;
        let a4037 = v0;
        nat_lt(a4036, a4037)
    } else {
        false
    }
}

pub fn f162(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4038 = f114(v0);
    match m4038 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4040 = v2;
                let a4041 = match v7.clone() {
                    Adt18::C0(_, h4039, _) => {
                        h4039
                    }
                };
                nat_mul(a4040, a4041)?
            };
            let m4054 = f214(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4042, _, _, _, _, _, _, _) => {
                    h4042
                }
            }, match v5.clone() {
                Adt3::C0(_, h4043, _, _, _, _, _, _) => {
                    h4043
                }
            }, match v5 {
                Adt3::C0(_, _, h4044, _, _, _, _, _) => {
                    h4044
                }
            }, {
                let a4047 = match v3.clone() {
                    Adt3::C0(h4045, _, _, _, _, _, _, _) => {
                        h4045
                    }
                };
                let a4048 = match v7.clone() {
                    Adt18::C0(_, h4046, _) => {
                        h4046
                    }
                };
                nat_mul(a4047, a4048)?
            }, match v3.clone() {
                Adt3::C0(_, h4049, _, _, _, _, _, _) => {
                    h4049
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4050, _, _, _) => {
                    h4050
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4051, _, _) => {
                    h4051
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4052, _) => {
                    h4052
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4053) => {
                    h4053
                }
            })?;
            match m4054 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4055 = v0;
                    let a4056 = 0u64;
                    if if nat_eq(a4055, a4056) {
                        true
                    } else {
                        let a4057 = v0;
                        let a4058 = 26u64;
                        nat_eq(a4057, a4058)
                    } {
                        Ok(Adt9::C1(false, v9, f139({
                            let a4062 = {
                                let a4060 = v1;
                                let a4061 = match v4.clone() {
                                    Adt3::C0(h4059, _, _, _, _, _, _, _) => {
                                        h4059
                                    }
                                };
                                nat_mul(a4060, a4061)?
                            };
                            let a4063 = v8;
                            nat_add(a4062, a4063)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4064) => {
                                h4064
                            }
                        }, f139({
                            let a4068 = {
                                let a4066 = v1;
                                let a4067 = match v4.clone() {
                                    Adt3::C0(h4065, _, _, _, _, _, _, _) => {
                                        h4065
                                    }
                                };
                                nat_mul(a4066, a4067)?
                            };
                            let a4069 = v8;
                            nat_add(a4068, a4069)?
                        }, v4.clone())?, {
                            let a4076 = {
                                let a4074 = {
                                    let a4071 = match v7 {
                                        Adt18::C0(_, h4070, _) => {
                                            h4070
                                        }
                                    };
                                    let a4072 = 1u64;
                                    nat_sub(a4071, a4072)
                                };
                                let a4075 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4073, _, _, _) => {
                                        h4073
                                    }
                                };
                                nat_mul(a4074, a4075)?
                            };
                            let a4077 = 4u64;
                            nat_add(a4076, a4077)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4078, _, _, _) => {
                                h4078
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f163(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4079 = v1;
    let a4080 = 0u64;
    if if nat_eq(a4079, a4080) {
        true
    } else {
        let a4081 = v2;
        let a4082 = 0u64;
        nat_eq(a4081, a4082)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f164(v0: u64, v1: u64) -> bool {
    let a4083 = v0;
    let a4084 = 4294967295u64;
    if nat_eq(a4083, a4084) {
        true
    } else {
        let a4085 = v0;
        let a4086 = 2147483648u64;
        if nat_lt(a4085, a4086) {
            let a4087 = v0;
            let a4088 = v1;
            nat_lt(a4087, a4088)
        } else {
            false
        }
    }
}

pub fn f165(v0: Adt7) -> R<u64> {
    let a4093 = if match v0.clone() {
        Adt7::C0(_, _, _, h4089, _) => {
            h4089
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4094 = {
        let a4091 = match v0 {
            Adt7::C0(_, _, _, _, h4090) => {
                h4090
            }
        };
        let a4092 = 1u64;
        if nat_eq(a4091, a4092) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4093, a4094)
}

pub fn f166(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4099 = {
            let a4096 = v2;
            let a4097 = match v0.clone() {
                Adt6::C0(_, h4095, _, _) => {
                    h4095
                }
            };
            nat_mul(a4096, a4097)?
        };
        let a4100 = f132(match v0.clone() {
            Adt6::C0(_, _, h4098, _) => {
                h4098
            }
        });
        nat_add(a4099, a4100)?
    };
    let v4: u64 = f133(match v0.clone() {
        Adt6::C0(_, _, h4101, _) => {
            h4101
        }
    });
    let v5: u64 = {
        let a4105 = {
            let a4103 = v3;
            let a4104 = match v0 {
                Adt6::C0(h4102, _, _, _) => {
                    h4102
                }
            };
            nat_add(a4103, a4104)?
        };
        let a4106 = v4;
        nat_sub(a4105, a4106)
    };
    let a4111 = {
        let a4107 = v5;
        let a4108 = v1;
        if nat_lt(a4107, a4108) {
            v5
        } else {
            v1
        }
    };
    let a4112 = {
        let a4109 = v3;
        let a4110 = v4;
        nat_sub(a4109, a4110)
    };
    Ok(nat_sub(a4111, a4112))
}

pub fn f167(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4121 = f72(match v0.clone() {
        Adt7::C0(h4113, _, _, _, _) => {
            h4113
        }
    }, match v1.clone() {
        Adt3::C0(h4114, _, _, _, _, _, _, _) => {
            h4114
        }
    }, f70(v3, v2.clone(), 0u64), {
        let a4118 = v4;
        let a4119 = {
            let m4117 = match v0.clone() {
                Adt7::C0(h4116, _, _, _, _) => {
                    h4116
                }
            };
            match m4117 {
                Adt6::C0(h4115, _, _, _) => {
                    h4115
                }
            }
        };
        let a4120 = 0u64;
        nat_rem(a4118, a4119, a4120)
    })?;
    match m4121 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4130 = f72(match v0.clone() {
                Adt7::C0(_, h4122, _, _, _) => {
                    h4122
                }
            }, match v1.clone() {
                Adt3::C0(_, h4123, _, _, _, _, _, _) => {
                    h4123
                }
            }, f70(v3, v2.clone(), 1u64), {
                let a4127 = v4;
                let a4128 = {
                    let m4126 = match v0 {
                        Adt7::C0(h4125, _, _, _, _) => {
                            h4125
                        }
                    };
                    match m4126 {
                        Adt6::C0(h4124, _, _, _) => {
                            h4124
                        }
                    }
                };
                let a4129 = 0u64;
                nat_quot(a4127, a4128, a4129)
            })?;
            match m4130 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f69(v1, v5, v6, f70(v3, v2.clone(), 2u64), f70(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f168(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m4131 = v0;
    match m4131 {
        Adt16::C0(v19) => {
            f215(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4132 = v20;
            let a4133 = 66u64;
            if nat_eq(a4132, a4133) {
                f216(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f217(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f169(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4140 = v3;
        let a4141 = {
            let a4137 = f139({
                let a4134 = v0;
                let a4135 = 2u64;
                let a4136 = 0u64;
                nat_quot(a4134, a4135, a4136)
            }, v1)?;
            let a4138 = v2;
            let a4139 = 0u64;
            nat_quot(a4137, a4138, a4139)
        };
        nat_add(a4140, a4141)?
    };
    let a4145 = {
        let a4142 = v0;
        let a4143 = 2u64;
        let a4144 = 0u64;
        nat_rem(a4142, a4143, a4144)
    };
    let a4146 = 0u64;
    if nat_eq(a4145, a4146) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4147 = v4;
        let a4148 = 0u64;
        if nat_eq(a4147, a4148) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f170(v0: u64, v1: u64) -> R<u64> {
    let a4151 = 8u64;
    let a4152 = {
        let a4149 = f218(v0);
        let a4150 = v1;
        nat_mul(a4149, a4150)?
    };
    nat_add(a4151, a4152)
}

pub fn f171(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4153, _, _, _, _, _, _, _) => {
            h4153
        }
    };
    let v7: u64 = f218(v3);
    let v8: u64 = {
        let a4156 = 1u64;
        let a4157 = {
            let a4154 = v6;
            let a4155 = v7;
            nat_mul(a4154, a4155)?
        };
        nat_add(a4156, a4157)?
    };
    let a4158 = v0;
    let a4159 = 0u64;
    if nat_eq(a4158, a4159) {
        Ok(f219(0u64))
    } else {
        let a4160 = v0;
        let a4161 = v8;
        if nat_lt(a4160, a4161) {
            let v9: u64 = {
                let a4162 = v0;
                let a4163 = 1u64;
                nat_sub(a4162, a4163)
            };
            let v10: u64 = {
                let a4164 = v9;
                let a4165 = v7;
                let a4166 = 0u64;
                nat_rem(a4164, a4165, a4166)
            };
            let v11: u64 = f139({
                let a4172 = {
                    let a4167 = v1;
                    let a4168 = v6;
                    nat_mul(a4167, a4168)?
                };
                let a4173 = {
                    let a4169 = v9;
                    let a4170 = v7;
                    let a4171 = 0u64;
                    nat_quot(a4169, a4170, a4171)
                };
                nat_add(a4172, a4173)?
            }, v2)?;
            let a4174 = v3;
            let a4175 = 30u64;
            if nat_eq(a4174, a4175) {
                let a4176 = v10;
                let a4177 = 0u64;
                if nat_eq(a4176, a4177) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4178 = v10;
                    let a4179 = 1u64;
                    if nat_eq(a4178, a4179) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4180 = v10;
                        let a4181 = 2u64;
                        if nat_eq(a4180, a4181) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4182 = v10;
                let a4183 = 0u64;
                if nat_eq(a4182, a4183) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4184 = v10;
                    let a4185 = 1u64;
                    if nat_eq(a4184, a4185) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4186 = v0;
                let a4187 = v8;
                nat_sub(a4186, a4187)
            };
            let a4188 = v12;
            let a4189 = 0u64;
            if nat_eq(a4188, a4189) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4190 = v12;
                let a4191 = 1u64;
                if nat_eq(a4190, a4191) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4192 = v12;
                    let a4193 = 2u64;
                    if nat_eq(a4192, a4193) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4194 = v12;
                        let a4195 = 3u64;
                        if nat_eq(a4194, a4195) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4196 = v12;
                            let a4197 = 4u64;
                            if nat_eq(a4196, a4197) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4198 = v12;
                                let a4199 = 5u64;
                                if nat_eq(a4198, a4199) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4200 = v12;
                                    let a4201 = 6u64;
                                    if nat_eq(a4200, a4201) {
                                        if f220(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4202 = 3u64;
                                                let a4203 = v1;
                                                nat_add(a4202, a4203)?
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

pub fn f172(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4204 = v0;
        let a4205 = f102(v3);
        let a4206 = 0u64;
        nat_quot(a4204, a4205, a4206)
    };
    let v5: u64 = {
        let a4211 = 3u64;
        let a4212 = {
            let a4208 = v4;
            let a4209 = match v1.clone() {
                Adt3::C0(h4207, _, _, _, _, _, _, _) => {
                    h4207
                }
            };
            let a4210 = 0u64;
            nat_quot(a4208, a4209, a4210)
        };
        nat_add(a4211, a4212)?
    };
    let v6: u64 = f139(v4, v1)?;
    let v7: u64 = f139(v4, v2)?;
    let a4213 = v3;
    let a4214 = 30u64;
    if nat_eq(a4213, a4214) {
        let a4218 = {
            let a4215 = v0;
            let a4216 = 3u64;
            let a4217 = 0u64;
            nat_rem(a4215, a4216, a4217)
        };
        let a4219 = 0u64;
        if nat_eq(a4218, a4219) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4223 = {
                let a4220 = v0;
                let a4221 = 3u64;
                let a4222 = 0u64;
                nat_rem(a4220, a4221, a4222)
            };
            let a4224 = 1u64;
            if nat_eq(a4223, a4224) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f173(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4225 = v0;
        let a4226 = 8388608u64;
        let a4227 = 0u64;
        nat_quot(a4225, a4226, a4227)
    };
    let a4228 = v0;
    let a4229 = 2147483648u64;
    if if if nat_lt(a4228, a4229) {
        let a4230 = 127u64;
        let a4231 = v1;
        nat_le(a4230, a4231)
    } else {
        false
    } {
        let a4232 = v1;
        let a4233 = 190u64;
        nat_lt(a4232, a4233)
    } else {
        false
    } {
        let v2: u64 = {
            let a4237 = 8388608u64;
            let a4238 = {
                let a4234 = v0;
                let a4235 = 8388608u64;
                let a4236 = 0u64;
                nat_rem(a4234, a4235, a4236)
            };
            nat_add(a4237, a4238)?
        };
        let a4239 = v1;
        let a4240 = 150u64;
        if nat_lt(a4239, a4240) {
            let v3: u64 = f221({
                let a4241 = 150u64;
                let a4242 = v1;
                nat_sub(a4241, a4242)
            });
            let a4246 = {
                let a4243 = v2;
                let a4244 = v3;
                let a4245 = 0u64;
                nat_rem(a4243, a4244, a4245)
            };
            let a4247 = 0u64;
            if nat_eq(a4246, a4247) {
                let a4248 = v2;
                let a4249 = v3;
                let a4250 = 0u64;
                Ok(nat_quot(a4248, a4249, a4250))
            } else {
                Ok(0u64)
            }
        } else {
            let a4253 = f221({
                let a4251 = v1;
                let a4252 = 150u64;
                nat_sub(a4251, a4252)
            });
            let a4254 = v2;
            nat_mul(a4253, a4254)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f174(v0: Adt14, v1: Adt14) -> bool {
    let a4257 = match v0.clone() {
        Adt14::C0(_, h4255, _, _, _, _, _, _, _, _, _) => {
            h4255
        }
    };
    let a4258 = match v1.clone() {
        Adt14::C0(_, h4256, _, _, _, _, _, _, _, _, _) => {
            h4256
        }
    };
    if if if nat_eq(a4257, a4258) {
        let a4261 = match v0.clone() {
            Adt14::C0(_, _, h4259, _, _, _, _, _, _, _, _) => {
                h4259
            }
        };
        let a4262 = match v1.clone() {
            Adt14::C0(_, _, h4260, _, _, _, _, _, _, _, _) => {
                h4260
            }
        };
        nat_eq(a4261, a4262)
    } else {
        false
    } {
        let a4265 = match v0.clone() {
            Adt14::C0(_, _, _, h4263, _, _, _, _, _, _, _) => {
                h4263
            }
        };
        let a4266 = match v1.clone() {
            Adt14::C0(_, _, _, h4264, _, _, _, _, _, _, _) => {
                h4264
            }
        };
        nat_eq(a4265, a4266)
    } else {
        false
    } {
        let a4269 = match v0 {
            Adt14::C0(_, _, _, _, h4267, _, _, _, _, _, _) => {
                h4267
            }
        };
        let a4270 = match v1 {
            Adt14::C0(_, _, _, _, h4268, _, _, _, _, _, _) => {
                h4268
            }
        };
        nat_eq(a4269, a4270)
    } else {
        false
    }
}

pub fn f175(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4271 = v0;
    let a4272 = 0u64;
    if if if nat_eq(a4271, a4272) {
        true
    } else {
        let a4273 = v1;
        let a4274 = 0u64;
        nat_eq(a4273, a4274)
    } {
        true
    } else {
        let a4275 = v2;
        let a4276 = 0u64;
        nat_eq(a4275, a4276)
    } {
        true
    } else {
        let a4277 = v3;
        let a4278 = 0u64;
        nat_eq(a4277, a4278)
    }
}

pub fn f176(v0: Adt14, v1: Adt14) -> bool {
    if f182(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4279, _, _, _, _, _) => {
            h4279
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4280, _, _, _, _, _) => {
            h4280
        }
    }) {
        let a4282 = match v0 {
            Adt14::C0(_, h4281, _, _, _, _, _, _, _, _, _) => {
                h4281
            }
        };
        let a4283 = 2147483647u64;
        nat_le(a4282, a4283)
    } else {
        true
    }
}

pub fn f177(v0: Adt14) -> bool {
    let a4288 = {
        let a4285 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4284) => {
                h4284
            }
        };
        let a4286 = 4u64;
        let a4287 = 0u64;
        nat_rem(a4285, a4286, a4287)
    };
    let a4289 = 0u64;
    if if nat_eq(a4288, a4289) {
        f222(match v0.clone() {
            Adt14::C0(_, h4290, _, _, _, _, _, _, _, _, _) => {
                h4290
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4291, _, _, _, _, _) => {
                h4291
            }
        })
    } else {
        false
    } {
        f98(match v0.clone() {
            Adt14::C0(_, _, h4292, _, _, _, _, _, _, _, _) => {
                h4292
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4293, _, _, _, _, _, _, _) => {
                h4293
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4294, _, _, _, _, _, _) => {
                h4294
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4295, _, _, _, _) => {
                h4295
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4296, _, _, _) => {
                h4296
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4297, _, _) => {
                h4297
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f178(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4299 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4298, _) => {
            h4298
        }
    };
    let a4300 = 0u64;
    if nat_eq(a4299, a4300) {
        Ok(true)
    } else {
        let a4302 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4301, _) => {
                h4301
            }
        };
        let a4303 = 0u64;
        if nat_eq(a4302, a4303) {
            Ok(true)
        } else {
            let a4305 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4304) => {
                    h4304
                }
            };
            let a4306 = 0u64;
            if nat_eq(a4305, a4306) {
                Ok(true)
            } else {
                let a4308 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4307) => {
                        h4307
                    }
                };
                let a4309 = 0u64;
                if nat_eq(a4308, a4309) {
                    Ok(true)
                } else {
                    let a4315 = {
                        let a4312 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4310) => {
                                h4310
                            }
                        };
                        let a4313 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4311, _) => {
                                h4311
                            }
                        };
                        nat_add(a4312, a4313)?
                    };
                    let a4316 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4314) => {
                            h4314
                        }
                    };
                    if nat_le(a4315, a4316) {
                        Ok(true)
                    } else {
                        let a4322 = {
                            let a4319 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4317) => {
                                    h4317
                                }
                            };
                            let a4320 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4318, _) => {
                                    h4318
                                }
                            };
                            nat_add(a4319, a4320)?
                        };
                        let a4323 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4321) => {
                                h4321
                            }
                        };
                        Ok(nat_le(a4322, a4323))
                    }
                }
            }
        }
    }
}

pub fn f179(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f174(v0.clone(), v1.clone()) {
        let a4326 = match v0.clone() {
            Adt14::C0(h4324, _, _, _, _, _, _, _, _, _, _) => {
                h4324
            }
        };
        let a4327 = match v1.clone() {
            Adt14::C0(h4325, _, _, _, _, _, _, _, _, _, _) => {
                h4325
            }
        };
        nat_eq(a4326, a4327)
    } else {
        false
    } {
        let a4330 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4328) => {
                h4328
            }
        };
        let a4331 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4329) => {
                h4329
            }
        };
        nat_eq(a4330, a4331)
    } else {
        false
    } {
        let a4334 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4332, _, _, _, _, _) => {
                h4332
            }
        };
        let a4335 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4333, _, _, _, _, _) => {
                h4333
            }
        };
        nat_eq(a4334, a4335)
    } else {
        false
    } {
        let a4338 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4336, _, _, _, _) => {
                h4336
            }
        };
        let a4339 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4337, _, _, _, _) => {
                h4337
            }
        };
        nat_eq(a4338, a4339)
    } else {
        false
    } {
        let a4342 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4340, _, _, _) => {
                h4340
            }
        };
        let a4343 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4341, _, _, _) => {
                h4341
            }
        };
        nat_eq(a4342, a4343)
    } else {
        false
    } {
        let a4346 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4344, _, _) => {
                h4344
            }
        };
        let a4347 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4345, _, _) => {
                h4345
            }
        };
        nat_eq(a4346, a4347)
    } else {
        false
    }
}

pub fn f180(v0: Adt14) -> R<bool> {
    if f223(match v0.clone() {
        Adt14::C0(_, h4348, _, _, _, _, _, _, _, _, _) => {
            h4348
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4349, _, _, _, _, _) => {
            h4349
        }
    }) {
        f99(f224(match v0.clone() {
            Adt14::C0(_, h4350, _, _, _, _, _, _, _, _, _) => {
                h4350
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4351, _, _, _, _, _) => {
                h4351
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4352, _, _, _, _, _, _, _, _) => {
                h4352
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4353, _, _, _, _, _, _, _) => {
                h4353
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4354, _, _, _, _, _, _) => {
                h4354
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4355, _, _, _, _) => {
                h4355
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4356, _, _, _) => {
                h4356
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4357, _, _) => {
                h4357
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f181(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4358 = v1;
    let a4359 = 0u64;
    if if nat_eq(a4358, a4359) {
        true
    } else {
        let a4360 = v2;
        let a4361 = 0u64;
        nat_eq(a4360, a4361)
    } {
        Ok(0u64)
    } else {
        let a4364 = f100(v1, v2, v3)?;
        let a4365 = {
            let a4362 = v2;
            let a4363 = f225(v0, v1)?;
            nat_mul(a4362, a4363)?
        };
        nat_add(a4364, a4365)
    }
}

pub fn f182(v0: u64, v1: u64) -> bool {
    let a4366 = v0;
    let a4367 = 4u64;
    if nat_eq(a4366, a4367) {
        let a4368 = v1;
        let a4369 = 4u64;
        nat_eq(a4368, a4369)
    } else {
        false
    }
}

pub fn f183(v0: u64) -> R<u64> {
    let a4370 = 6u64;
    let a4371 = v0;
    nat_add(a4370, a4371)
}

pub fn f184(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4372, _, _, _, _, _, _, _, _, _, _) => {
            h4372
        }
    }, match v0.clone() {
        Adt14::C0(_, h4373, _, _, _, _, _, _, _, _, _) => {
            h4373
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4374, _, _, _, _, _, _, _, _) => {
            h4374
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4375, _, _, _, _, _, _, _) => {
            h4375
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4376, _, _, _, _, _, _) => {
            h4376
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4377, _, _, _, _) => {
            h4377
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4378, _, _, _) => {
            h4378
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4379, _, _) => {
            h4379
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4380, _) => {
            h4380
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4381) => {
            h4381
        }
    })
}

pub fn f185(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4382 = v0;
    let a4383 = 2147483647u64;
    if if if nat_le(a4382, a4383) {
        let a4384 = 0u64;
        let a4385 = v3;
        if nat_lt(a4384, a4385) {
            let a4386 = v3;
            let a4387 = 2147483647u64;
            nat_le(a4386, a4387)
        } else {
            false
        }
    } else {
        false
    } {
        let a4392 = {
            let a4390 = {
                let a4388 = v2;
                let a4389 = v3;
                nat_add(a4388, a4389)?
            };
            let a4391 = 1u64;
            nat_sub(a4390, a4391)
        };
        let a4393 = 2147483647u64;
        nat_le(a4392, a4393)
    } else {
        false
    } {
        let a4398 = {
            let a4396 = {
                let a4394 = v0;
                let a4395 = v1;
                nat_mul(a4394, a4395)?
            };
            let a4397 = f226(v2, v3)?;
            nat_mul(a4396, a4397)?
        };
        let a4399 = 9223372036854775807u64;
        Ok(nat_le(a4398, a4399))
    } else {
        Ok(false)
    }
}

pub fn f186(v0: Adt14) -> R<u64> {
    let a4430 = {
        let a4422 = {
            let a4414 = {
                let a4406 = 4u64;
                let a4407 = {
                    let a4404 = {
                        let a4401 = match v0.clone() {
                            Adt14::C0(_, h4400, _, _, _, _, _, _, _, _, _) => {
                                h4400
                            }
                        };
                        let a4402 = 1u64;
                        nat_sub(a4401, a4402)
                    };
                    let a4405 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4403, _, _, _, _, _) => {
                            h4403
                        }
                    };
                    nat_mul(a4404, a4405)?
                };
                nat_add(a4406, a4407)?
            };
            let a4415 = {
                let a4412 = {
                    let a4409 = match v0.clone() {
                        Adt14::C0(_, _, h4408, _, _, _, _, _, _, _, _) => {
                            h4408
                        }
                    };
                    let a4410 = 1u64;
                    nat_sub(a4409, a4410)
                };
                let a4413 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4411, _, _, _, _) => {
                        h4411
                    }
                };
                nat_mul(a4412, a4413)?
            };
            nat_add(a4414, a4415)?
        };
        let a4423 = {
            let a4420 = {
                let a4417 = match v0.clone() {
                    Adt14::C0(_, _, _, h4416, _, _, _, _, _, _, _) => {
                        h4416
                    }
                };
                let a4418 = 1u64;
                nat_sub(a4417, a4418)
            };
            let a4421 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4419, _, _, _) => {
                    h4419
                }
            };
            nat_mul(a4420, a4421)?
        };
        nat_add(a4422, a4423)?
    };
    let a4431 = {
        let a4428 = {
            let a4425 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4424, _, _, _, _, _, _) => {
                    h4424
                }
            };
            let a4426 = 1u64;
            nat_sub(a4425, a4426)
        };
        let a4429 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4427, _, _) => {
                h4427
            }
        };
        nat_mul(a4428, a4429)?
    };
    nat_add(a4430, a4431)
}

pub fn f187(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4432 = v0;
    let a4433 = 0u64;
    if if if if nat_eq(a4432, a4433) {
        true
    } else {
        let a4434 = v1;
        let a4435 = 0u64;
        nat_eq(a4434, a4435)
    } {
        true
    } else {
        let a4436 = v2;
        let a4437 = 0u64;
        nat_eq(a4436, a4437)
    } {
        true
    } else {
        let a4438 = v3;
        let a4439 = 0u64;
        nat_eq(a4438, a4439)
    } {
        Ok(0u64)
    } else {
        let a4440 = f227(v0, v1, v2, v3, v5)?;
        let a4441 = f228(v0, v1, v2, v3, v4)?;
        nat_add(a4440, a4441)
    }
}

pub fn f188(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4442 = v0;
    let a4443 = 0u64;
    if if nat_eq(a4442, a4443) {
        true
    } else {
        let a4444 = v1;
        let a4445 = 0u64;
        nat_eq(a4444, a4445)
    } {
        Ok(0u64)
    } else {
        let a4448 = f100(v0, v1, v2)?;
        let a4449 = {
            let a4446 = v1;
            let a4447 = f229(v0)?;
            nat_mul(a4446, a4447)?
        };
        nat_add(a4448, a4449)
    }
}

pub fn f189(v0: u64, v1: u64, v2: u64) -> bool {
    let a4450 = v0;
    let a4451 = 1u64;
    if nat_le(a4450, a4451) {
        true
    } else {
        let a4455 = {
            let a4452 = v1;
            let a4453 = v2;
            let a4454 = 1u64;
            nat_rem(a4452, a4453, a4454)
        };
        let a4456 = 0u64;
        nat_eq(a4455, a4456)
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4457 = v1;
    let a4458 = 1u64;
    if if if nat_le(a4457, a4458) {
        true
    } else {
        let a4459 = v0;
        let a4460 = v2;
        nat_le(a4459, a4460)
    } {
        let a4461 = v3;
        let a4462 = 1u64;
        if nat_le(a4461, a4462) {
            true
        } else {
            let a4469 = {
                let a4467 = v0;
                let a4468 = {
                    let a4465 = {
                        let a4463 = v1;
                        let a4464 = 1u64;
                        nat_sub(a4463, a4464)
                    };
                    let a4466 = v2;
                    nat_mul(a4465, a4466)?
                };
                nat_add(a4467, a4468)?
            };
            let a4470 = v4;
            nat_le(a4469, a4470)
        }
    } else {
        false
    } {
        let a4471 = v5;
        let a4472 = 1u64;
        if nat_le(a4471, a4472) {
            Ok(true)
        } else {
            let a4485 = {
                let a4483 = {
                    let a4477 = v0;
                    let a4478 = {
                        let a4475 = {
                            let a4473 = v1;
                            let a4474 = 1u64;
                            nat_sub(a4473, a4474)
                        };
                        let a4476 = v2;
                        nat_mul(a4475, a4476)?
                    };
                    nat_add(a4477, a4478)?
                };
                let a4484 = {
                    let a4481 = {
                        let a4479 = v3;
                        let a4480 = 1u64;
                        nat_sub(a4479, a4480)
                    };
                    let a4482 = v4;
                    nat_mul(a4481, a4482)?
                };
                nat_add(a4483, a4484)?
            };
            let a4486 = v6;
            Ok(nat_le(a4485, a4486))
        }
    } else {
        Ok(false)
    }
}

pub fn f191(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f230(v0, v1, v6, v7, v8) {
        let a4487 = v5;
        let a4488 = 26u64;
        nat_eq(a4487, a4488)
    } else {
        false
    } {
        let a4489 = v3;
        let a4490 = v7;
        nat_eq(a4489, a4490)
    } else {
        false
    } {
        let a4491 = v4;
        let a4492 = v8;
        nat_eq(a4491, a4492)
    } else {
        false
    } {
        let a4493 = v9;
        let a4494 = 1u64;
        nat_eq(a4493, a4494)
    } else {
        false
    } {
        let a4495 = v10;
        let a4496 = f231(v0);
        nat_eq(a4495, a4496)
    } else {
        false
    } {
        let a4497 = v11;
        let a4498 = v1;
        nat_eq(a4497, a4498)
    } else {
        false
    } {
        let a4499 = v12;
        let a4500 = v6;
        nat_eq(a4499, a4500)
    } else {
        false
    } {
        let a4501 = v13;
        let a4502 = v7;
        nat_eq(a4501, a4502)
    } else {
        false
    } {
        let a4503 = v14;
        let a4504 = v8;
        nat_eq(a4503, a4504)
    } else {
        false
    }
}

pub fn f192(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4505 = v0;
    let a4506 = 0u64;
    if if if nat_eq(a4505, a4506) {
        true
    } else {
        let a4507 = v1;
        let a4508 = 0u64;
        nat_eq(a4507, a4508)
    } {
        true
    } else {
        let a4509 = v2;
        let a4510 = 0u64;
        nat_eq(a4509, a4510)
    } {
        true
    } else {
        let a4511 = v3;
        let a4512 = 0u64;
        nat_eq(a4511, a4512)
    }
}

pub fn f193(v0: u64) -> u64 {
    let m4513 = f114(v0);
    match m4513 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4514, _) => {
                    h4514
                }
            }
        }
    }
}

pub fn f194(v0: Adt3, v1: bool) -> R<u64> {
    f232(v0.clone(), {
        let a4516 = match v0.clone() {
            Adt3::C0(h4515, _, _, _, _, _, _, _) => {
                h4515
            }
        };
        let a4517 = 1u64;
        nat_sub(a4516, a4517)
    }, {
        let a4519 = match v0.clone() {
            Adt3::C0(_, h4518, _, _, _, _, _, _) => {
                h4518
            }
        };
        let a4520 = 1u64;
        nat_sub(a4519, a4520)
    }, {
        let a4522 = match v0.clone() {
            Adt3::C0(_, _, h4521, _, _, _, _, _) => {
                h4521
            }
        };
        let a4523 = 1u64;
        nat_sub(a4522, a4523)
    }, {
        let a4525 = match v0 {
            Adt3::C0(_, _, _, h4524, _, _, _, _) => {
                h4524
            }
        };
        let a4526 = 1u64;
        nat_sub(a4525, a4526)
    }, v1)
}

pub fn f195(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4527 = v0;
    let a4528 = 0u64;
    if nat_eq(a4527, a4528) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4529 = v0;
            let a4530 = 1u64;
            nat_sub(a4529, a4530)
        };
        let v5: u64 = {
            let a4531 = v4;
            let a4532 = v1;
            let a4533 = 0u64;
            nat_quot(a4531, a4532, a4533)
        };
        let v6: u64 = {
            let a4541 = {
                let a4534 = v5;
                let a4535 = v2;
                nat_mul(a4534, a4535)?
            };
            let a4542 = {
                let a4539 = {
                    let a4536 = v4;
                    let a4537 = v1;
                    let a4538 = 0u64;
                    nat_rem(a4536, a4537, a4538)
                };
                let a4540 = v3;
                nat_mul(a4539, a4540)?
            };
            nat_add(a4541, a4542)?
        };
        let a4543 = v5;
        let a4544 = 0u64;
        if nat_eq(a4543, a4544) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4553 = {
                    let a4547 = {
                        let a4545 = v5;
                        let a4546 = 1u64;
                        nat_sub(a4545, a4546)
                    };
                    let a4548 = v2;
                    nat_mul(a4547, a4548)?
                };
                let a4554 = {
                    let a4551 = {
                        let a4549 = v1;
                        let a4550 = 1u64;
                        nat_sub(a4549, a4550)
                    };
                    let a4552 = v3;
                    nat_mul(a4551, a4552)?
                };
                nat_add(a4553, a4554)?
            };
            let a4555 = v6;
            let a4556 = v7;
            if nat_lt(a4555, a4556) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f196(v0: Adt3, v1: u64) -> bool {
    let a4557 = v1;
    let a4558 = 0u64;
    if !nat_eq(a4557, a4558) {
        true
    } else {
        let a4560 = match v0 {
            Adt3::C0(_, _, _, _, h4559, _, _, _) => {
                h4559
            }
        };
        let a4561 = 4u64;
        nat_eq(a4560, a4561)
    }
}

pub fn f197(v0: u64, v1: bool) -> R<u64> {
    let a4564 = {
        let a4562 = v0;
        let a4563 = 2u64;
        nat_mul(a4562, a4563)?
    };
    let a4565 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4564, a4565)
}

pub fn f198(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4566 = f233(v0, v1);
    match m4566 {
        None => {
            if v5 {
                let m4567 = f234(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4567 {
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
                let a4568 = 4u64;
                let a4569 = v28;
                nat_le(a4568, a4569)
            };
            let a4570 = v2;
            let a4571 = 0u64;
            if if if if if nat_eq(a4570, a4571) {
                let a4572 = v4;
                let a4573 = 0u64;
                nat_eq(a4572, a4573)
            } else {
                false
            } {
                let a4574 = v6;
                let a4575 = v14;
                if nat_eq(a4574, a4575) {
                    let a4576 = v7;
                    let a4577 = v15;
                    nat_eq(a4576, a4577)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4578 = v8;
                let a4579 = v16;
                if nat_eq(a4578, a4579) {
                    let a4580 = v9;
                    let a4581 = v17;
                    nat_eq(a4580, a4581)
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
                        let a4582 = v3;
                        let a4583 = 0u64;
                        nat_eq(a4582, a4583)
                    } else {
                        false
                    } {
                        f117(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4588 = {
                        let a4584 = v6;
                        let a4585 = v7;
                        nat_mul(a4584, a4585)?
                    };
                    let a4589 = {
                        let a4586 = v8;
                        let a4587 = v9;
                        nat_mul(a4586, a4587)?
                    };
                    nat_mul(a4588, a4589)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f199(v0: u64) -> u64 {
    let m4590 = f114(v0);
    match m4590 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4592 = match v1.clone() {
                Adt18::C0(_, h4591, _) => {
                    h4591
                }
            };
            let a4593 = 1u64;
            if if nat_eq(a4592, a4593) {
                let a4595 = match v1.clone() {
                    Adt18::C0(_, _, h4594) => {
                        h4594
                    }
                };
                let a4596 = 1u64;
                if if if nat_eq(a4595, a4596) {
                    true
                } else {
                    let a4598 = match v1.clone() {
                        Adt18::C0(_, _, h4597) => {
                            h4597
                        }
                    };
                    let a4599 = 2u64;
                    nat_eq(a4598, a4599)
                } {
                    true
                } else {
                    let a4601 = match v1.clone() {
                        Adt18::C0(_, _, h4600) => {
                            h4600
                        }
                    };
                    let a4602 = 4u64;
                    nat_eq(a4601, a4602)
                } {
                    true
                } else {
                    let a4604 = match v1.clone() {
                        Adt18::C0(_, _, h4603) => {
                            h4603
                        }
                    };
                    let a4605 = 8u64;
                    nat_eq(a4604, a4605)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4606) => {
                        h4606
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f200(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4611 = {
        let a4609 = {
            let a4607 = v0;
            let a4608 = v1;
            nat_sub(a4607, a4608)
        };
        let a4610 = 1u64;
        nat_sub(a4609, a4610)
    };
    let a4612 = v2;
    nat_add(a4611, a4612)
}

pub fn f201(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f131({
        let a4613 = v0;
        let a4614 = v4;
        let a4615 = 0u64;
        nat_rem(a4613, a4614, a4615)
    }, {
        let a4616 = v1;
        let a4617 = v5;
        let a4618 = 0u64;
        nat_rem(a4616, a4617, a4618)
    }, {
        let a4619 = v2;
        let a4620 = v6;
        let a4621 = 0u64;
        nat_rem(a4619, a4620, a4621)
    }, {
        let a4622 = v3;
        let a4623 = v7;
        let a4624 = 0u64;
        nat_rem(a4622, a4623, a4624)
    }, v8, v9, v10, v11)
}

pub fn f202(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4625 = v1;
    let a4626 = 2147483648u64;
    if nat_lt(a4625, a4626) {
        let a4637 = {
            let a4635 = {
                let a4630 = {
                    let a4627 = v0;
                    let a4628 = v2;
                    let a4629 = 0u64;
                    nat_rem(a4627, a4628, a4629)
                };
                let a4631 = v2;
                nat_add(a4630, a4631)?
            };
            let a4636 = {
                let a4632 = v1;
                let a4633 = v2;
                let a4634 = 0u64;
                nat_rem(a4632, a4633, a4634)
            };
            nat_sub(a4635, a4636)
        };
        let a4638 = v2;
        let a4639 = 0u64;
        Ok(nat_rem(a4637, a4638, a4639))
    } else {
        let a4642 = {
            let a4640 = v0;
            let a4641 = f132(v1);
            nat_add(a4640, a4641)?
        };
        let a4643 = v2;
        let a4644 = 0u64;
        Ok(nat_rem(a4642, a4643, a4644))
    }
}

pub fn f203(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4647 = f133(v1);
    let a4648 = {
        let a4645 = v0;
        let a4646 = f132(v1);
        nat_add(a4645, a4646)?
    };
    if nat_le(a4647, a4648) {
        let a4649 = f204(v0, v1)?;
        let a4650 = v2;
        Ok(nat_lt(a4649, a4650))
    } else {
        Ok(false)
    }
}

pub fn f204(v0: u64, v1: u64) -> R<u64> {
    let a4653 = {
        let a4651 = v0;
        let a4652 = f132(v1);
        nat_add(a4651, a4652)?
    };
    let a4654 = f133(v1);
    Ok(nat_sub(a4653, a4654))
}

pub fn f205(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4655 = v0;
    let a4656 = v1;
    if nat_lt(a4655, a4656) {
        let a4657 = v1;
        let a4658 = v0;
        Ok(nat_sub(a4657, a4658))
    } else {
        let a4661 = v0;
        let a4662 = {
            let a4659 = v1;
            let a4660 = v2;
            nat_add(a4659, a4660)?
        };
        if nat_lt(a4661, a4662) {
            let a4663 = v0;
            let a4664 = v1;
            Ok(nat_sub(a4663, a4664))
        } else {
            let a4671 = {
                let a4667 = 2u64;
                let a4668 = {
                    let a4665 = v2;
                    let a4666 = 1u64;
                    nat_sub(a4665, a4666)
                };
                nat_mul(a4667, a4668)?
            };
            let a4672 = {
                let a4669 = v0;
                let a4670 = v1;
                nat_sub(a4669, a4670)
            };
            Ok(nat_sub(a4671, a4672))
        }
    }
}

pub fn f206(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4675 = {
        let a4673 = v2;
        let a4674 = f232(v1.clone(), f70(v0, v1.clone(), 0u64), f70(v0, v1.clone(), 1u64), f70(v0, v1.clone(), 2u64), f70(v0, v1.clone(), 3u64), false)?;
        nat_add(a4673, a4674)?
    };
    let a4676 = f232(v1.clone(), f70(v0, v1.clone(), 0u64), f70(v0, v1.clone(), 1u64), f70(v0, v1.clone(), 2u64), f70(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4675, a4676))
}

pub fn f207(v0: u64, v1: u64, v2: u64) -> bool {
    let a4677 = v0;
    let a4678 = 0u64;
    if nat_eq(a4677, a4678) {
        let a4679 = v2;
        let a4680 = v1;
        nat_le(a4679, a4680)
    } else {
        let a4681 = v0;
        let a4682 = 1u64;
        if nat_eq(a4681, a4682) {
            let a4683 = v2;
            let a4684 = v1;
            nat_lt(a4683, a4684)
        } else {
            let a4685 = v0;
            let a4686 = 2u64;
            if nat_eq(a4685, a4686) {
                let a4687 = v1;
                let a4688 = v2;
                nat_le(a4687, a4688)
            } else {
                let a4689 = v0;
                let a4690 = 3u64;
                if nat_eq(a4689, a4690) {
                    let a4691 = v1;
                    let a4692 = v2;
                    nat_lt(a4691, a4692)
                } else {
                    let a4693 = v1;
                    let a4694 = v2;
                    nat_eq(a4693, a4694)
                }
            }
        }
    }
}

pub fn f208(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4695 = v0;
    let a4696 = 0u64;
    if nat_eq(a4695, a4696) {
        Ok(f219(0u64))
    } else {
        let a4699 = {
            let a4697 = v0;
            let a4698 = 1u64;
            nat_add(a4697, a4698)?
        };
        let a4700 = f128(v1.clone(), 13u64, v2)?;
        if nat_eq(a4699, a4700) {
            Ok(Adt9::C4({
                let a4701 = v2;
                let a4702 = 0u64;
                if nat_eq(a4701, a4702) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4703 = v0;
                let a4704 = 1u64;
                nat_sub(a4703, a4704)
            };
            if f196(v1.clone(), v2) {
                let v4: u64 = f197(match v1.clone() {
                    Adt3::C0(h4705, _, _, _, _, _, _, _) => {
                        h4705
                    }
                }, false)?;
                let v5: u64 = {
                    let a4706 = v3;
                    let a4707 = v4;
                    let a4708 = 0u64;
                    nat_quot(a4706, a4707, a4708)
                };
                let v6: u64 = {
                    let a4709 = v3;
                    let a4710 = v4;
                    let a4711 = 0u64;
                    nat_rem(a4709, a4710, a4711)
                };
                let a4714 = {
                    let a4712 = v6;
                    let a4713 = 1u64;
                    nat_add(a4712, a4713)?
                };
                let a4715 = v4;
                if nat_eq(a4714, a4715) {
                    Ok(Adt9::C4({
                        let a4716 = v2;
                        let a4717 = 0u64;
                        if nat_eq(a4716, a4717) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f209(v6, v5, v1, v2)
                }
            } else {
                f235({
                    let a4718 = v3;
                    let a4719 = 2u64;
                    let a4720 = 0u64;
                    nat_quot(a4718, a4719, a4720)
                }, {
                    let a4721 = v3;
                    let a4722 = 2u64;
                    let a4723 = 0u64;
                    nat_rem(a4721, a4722, a4723)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f209(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4724 = v0;
    let a4725 = 0u64;
    if nat_eq(a4724, a4725) {
        Ok(f219(1u64))
    } else {
        f235({
            let a4734 = {
                let a4727 = v1;
                let a4728 = match v2.clone() {
                    Adt3::C0(h4726, _, _, _, _, _, _, _) => {
                        h4726
                    }
                };
                nat_mul(a4727, a4728)?
            };
            let a4735 = {
                let a4731 = {
                    let a4729 = v0;
                    let a4730 = 1u64;
                    nat_sub(a4729, a4730)
                };
                let a4732 = 2u64;
                let a4733 = 0u64;
                nat_quot(a4731, a4732, a4733)
            };
            nat_add(a4734, a4735)?
        }, {
            let a4738 = {
                let a4736 = v0;
                let a4737 = 1u64;
                nat_sub(a4736, a4737)
            };
            let a4739 = 2u64;
            let a4740 = 0u64;
            nat_rem(a4738, a4739, a4740)
        }, v2, v3, 1u64)
    }
}

pub fn f210(v0: u64, v1: u64) -> R<u64> {
    let a4741 = v0;
    let a4742 = f94(v1)?;
    let a4743 = 0u64;
    Ok(nat_rem(a4741, a4742, a4743))
}

pub fn f211(v0: u64, v1: u64) -> R<u64> {
    let a4744 = 4u64;
    let a4745 = f236(v0, v1)?;
    nat_mul(a4744, a4745)
}

pub fn f212(v0: u64, v1: u64) -> R<u64> {
    let a4746 = 4u64;
    let a4747 = f237(v0, v1)?;
    nat_mul(a4746, a4747)
}

pub fn f213(v0: u64, v1: u64) -> R<bool> {
    let a4753 = {
        let a4750 = {
            let a4748 = f210(v0, v1)?;
            let a4749 = 2u64;
            nat_sub(a4748, a4749)
        };
        let a4751 = 2u64;
        let a4752 = 0u64;
        nat_rem(a4750, a4751, a4752)
    };
    let a4754 = 0u64;
    Ok(nat_eq(a4753, a4754))
}

pub fn f214(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4755 = f114(v0);
    match m4755 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f112(v0) {
                let a4756 = v1;
                let a4757 = v7;
                nat_lt(a4756, a4757)
            } else {
                false
            } {
                f161(v8, v2)
            } else {
                false
            } {
                let a4758 = v3;
                let a4759 = f238(v4, v5, v6)?;
                nat_lt(a4758, a4759)
            } else {
                false
            } {
                let a4764 = {
                    let a4761 = v7;
                    let a4762 = match v13.clone() {
                        Adt18::C0(_, h4760, _) => {
                            h4760
                        }
                    };
                    let a4763 = 0u64;
                    nat_rem(a4761, a4762, a4763)
                };
                let a4765 = 0u64;
                nat_eq(a4764, a4765)
            } else {
                false
            } {
                Ok(Some({
                    let a4794 = {
                        let a4784 = {
                            let a4774 = {
                                let a4770 = {
                                    let a4767 = v1;
                                    let a4768 = match v13 {
                                        Adt18::C0(_, h4766, _) => {
                                            h4766
                                        }
                                    };
                                    let a4769 = 0u64;
                                    nat_quot(a4767, a4768, a4769)
                                };
                                let a4771 = v9;
                                nat_mul(a4770, a4771)?
                            };
                            let a4775 = {
                                let a4772 = v2;
                                let a4773 = v10;
                                nat_mul(a4772, a4773)?
                            };
                            nat_add(a4774, a4775)?
                        };
                        let a4785 = {
                            let a4782 = {
                                let a4779 = {
                                    let a4776 = v3;
                                    let a4777 = v4;
                                    let a4778 = 0u64;
                                    nat_quot(a4776, a4777, a4778)
                                };
                                let a4780 = v5;
                                let a4781 = 0u64;
                                nat_rem(a4779, a4780, a4781)
                            };
                            let a4783 = v11;
                            nat_mul(a4782, a4783)?
                        };
                        nat_add(a4784, a4785)?
                    };
                    let a4795 = {
                        let a4792 = {
                            let a4789 = {
                                let a4786 = v3;
                                let a4787 = v4;
                                let a4788 = 0u64;
                                nat_quot(a4786, a4787, a4788)
                            };
                            let a4790 = v5;
                            let a4791 = 0u64;
                            nat_quot(a4789, a4790, a4791)
                        };
                        let a4793 = v12;
                        nat_mul(a4792, a4793)?
                    };
                    nat_add(a4794, a4795)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f215(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f239(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f216(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f240(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f217(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4796 = v0;
    let a4797 = 64u64;
    if nat_eq(a4796, a4797) {
        f241(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4798 = v0;
        let a4799 = 65u64;
        if nat_eq(a4798, a4799) {
            f242(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f218(v0: u64) -> u64 {
    let a4800 = v0;
    let a4801 = 30u64;
    if nat_eq(a4800, a4801) {
        4u64
    } else {
        3u64
    }
}

pub fn f219(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f220(v0: u64) -> bool {
    let a4802 = 0u64;
    let a4803 = v0;
    if nat_lt(a4802, a4803) {
        let a4804 = v0;
        let a4805 = 2139095040u64;
        nat_le(a4804, a4805)
    } else {
        false
    }
}

pub fn f221(v0: u64) -> u64 {
    let a4806 = v0;
    let a4807 = 40u64;
    if nat_lt(a4806, a4807) {
        let a4808 = v0;
        let a4809 = 20u64;
        if nat_lt(a4808, a4809) {
            let a4810 = v0;
            let a4811 = 10u64;
            if nat_lt(a4810, a4811) {
                let a4812 = v0;
                let a4813 = 5u64;
                if nat_lt(a4812, a4813) {
                    let a4814 = v0;
                    let a4815 = 2u64;
                    if nat_lt(a4814, a4815) {
                        let a4816 = v0;
                        let a4817 = 1u64;
                        if nat_lt(a4816, a4817) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4818 = v0;
                        let a4819 = 3u64;
                        if nat_lt(a4818, a4819) {
                            4u64
                        } else {
                            let a4820 = v0;
                            let a4821 = 4u64;
                            if nat_lt(a4820, a4821) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4822 = v0;
                    let a4823 = 7u64;
                    if nat_lt(a4822, a4823) {
                        let a4824 = v0;
                        let a4825 = 6u64;
                        if nat_lt(a4824, a4825) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4826 = v0;
                        let a4827 = 8u64;
                        if nat_lt(a4826, a4827) {
                            128u64
                        } else {
                            let a4828 = v0;
                            let a4829 = 9u64;
                            if nat_lt(a4828, a4829) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4830 = v0;
                let a4831 = 15u64;
                if nat_lt(a4830, a4831) {
                    let a4832 = v0;
                    let a4833 = 12u64;
                    if nat_lt(a4832, a4833) {
                        let a4834 = v0;
                        let a4835 = 11u64;
                        if nat_lt(a4834, a4835) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4836 = v0;
                        let a4837 = 13u64;
                        if nat_lt(a4836, a4837) {
                            4096u64
                        } else {
                            let a4838 = v0;
                            let a4839 = 14u64;
                            if nat_lt(a4838, a4839) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4840 = v0;
                    let a4841 = 17u64;
                    if nat_lt(a4840, a4841) {
                        let a4842 = v0;
                        let a4843 = 16u64;
                        if nat_lt(a4842, a4843) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4844 = v0;
                        let a4845 = 18u64;
                        if nat_lt(a4844, a4845) {
                            131072u64
                        } else {
                            let a4846 = v0;
                            let a4847 = 19u64;
                            if nat_lt(a4846, a4847) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4848 = v0;
            let a4849 = 30u64;
            if nat_lt(a4848, a4849) {
                let a4850 = v0;
                let a4851 = 25u64;
                if nat_lt(a4850, a4851) {
                    let a4852 = v0;
                    let a4853 = 22u64;
                    if nat_lt(a4852, a4853) {
                        let a4854 = v0;
                        let a4855 = 21u64;
                        if nat_lt(a4854, a4855) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4856 = v0;
                        let a4857 = 23u64;
                        if nat_lt(a4856, a4857) {
                            4194304u64
                        } else {
                            let a4858 = v0;
                            let a4859 = 24u64;
                            if nat_lt(a4858, a4859) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4860 = v0;
                    let a4861 = 27u64;
                    if nat_lt(a4860, a4861) {
                        let a4862 = v0;
                        let a4863 = 26u64;
                        if nat_lt(a4862, a4863) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4864 = v0;
                        let a4865 = 28u64;
                        if nat_lt(a4864, a4865) {
                            134217728u64
                        } else {
                            let a4866 = v0;
                            let a4867 = 29u64;
                            if nat_lt(a4866, a4867) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4868 = v0;
                let a4869 = 35u64;
                if nat_lt(a4868, a4869) {
                    let a4870 = v0;
                    let a4871 = 32u64;
                    if nat_lt(a4870, a4871) {
                        let a4872 = v0;
                        let a4873 = 31u64;
                        if nat_lt(a4872, a4873) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4874 = v0;
                        let a4875 = 33u64;
                        if nat_lt(a4874, a4875) {
                            4294967296u64
                        } else {
                            let a4876 = v0;
                            let a4877 = 34u64;
                            if nat_lt(a4876, a4877) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4878 = v0;
                    let a4879 = 37u64;
                    if nat_lt(a4878, a4879) {
                        let a4880 = v0;
                        let a4881 = 36u64;
                        if nat_lt(a4880, a4881) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4882 = v0;
                        let a4883 = 38u64;
                        if nat_lt(a4882, a4883) {
                            137438953472u64
                        } else {
                            let a4884 = v0;
                            let a4885 = 39u64;
                            if nat_lt(a4884, a4885) {
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

pub fn f222(v0: u64, v1: u64) -> bool {
    let a4886 = v0;
    let a4887 = 1u64;
    if nat_le(a4886, a4887) {
        true
    } else {
        let a4891 = {
            let a4888 = v1;
            let a4889 = 4u64;
            let a4890 = 0u64;
            nat_rem(a4888, a4889, a4890)
        };
        let a4892 = 0u64;
        nat_eq(a4891, a4892)
    }
}

pub fn f223(v0: u64, v1: u64) -> bool {
    let a4893 = v0;
    let a4894 = 1u64;
    if nat_le(a4893, a4894) {
        true
    } else {
        let a4895 = 4u64;
        let a4896 = v1;
        nat_le(a4895, a4896)
    }
}

pub fn f224(v0: u64, v1: u64) -> R<u64> {
    let a4897 = v0;
    let a4898 = 0u64;
    if nat_eq(a4897, a4898) {
        Ok(0u64)
    } else {
        let a4903 = 4u64;
        let a4904 = {
            let a4901 = {
                let a4899 = v0;
                let a4900 = 1u64;
                nat_sub(a4899, a4900)
            };
            let a4902 = v1;
            nat_mul(a4901, a4902)?
        };
        nat_add(a4903, a4904)
    }
}

pub fn f225(v0: bool, v1: u64) -> R<u64> {
    let a4909 = {
        let a4907 = {
            let a4905 = f243(v0, v1)?;
            let a4906 = f244(v0, v1)?;
            nat_add(a4905, a4906)?
        };
        let a4908 = f245(v0);
        nat_add(a4907, a4908)?
    };
    let a4910 = v1;
    nat_add(a4909, a4910)
}

pub fn f226(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f246(v0, v1)?;
    let v3: u64 = {
        let a4915 = {
            let a4913 = {
                let a4911 = v1;
                let a4912 = 1u64;
                nat_sub(a4911, a4912)
            };
            let a4914 = v2;
            nat_mul(a4913, a4914)?
        };
        let a4916 = v0;
        nat_sub(a4915, a4916)
    };
    let a4917 = v2;
    let a4918 = v3;
    if nat_lt(a4917, a4918) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f227(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4919 = v4;
    let a4920 = 0u64;
    if nat_eq(a4919, a4920) {
        Ok(0u64)
    } else {
        let a4927 = 2u64;
        let a4928 = {
            let a4925 = {
                let a4921 = v0;
                let a4922 = v1;
                nat_mul(a4921, a4922)?
            };
            let a4926 = {
                let a4923 = v2;
                let a4924 = v3;
                nat_mul(a4923, a4924)?
            };
            nat_mul(a4925, a4926)?
        };
        nat_mul(a4927, a4928)
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4933 = v3;
    let a4934 = {
        let a4931 = {
            let a4929 = f247(v2, v4)?;
            let a4930 = f248(v0, v1, f246(v2, v4)?)?;
            nat_mul(a4929, a4930)?
        };
        let a4932 = f248(v0, v1, f249(v2, v4)?)?;
        nat_add(a4931, a4932)?
    };
    nat_mul(a4933, a4934)
}

pub fn f229(v0: u64) -> R<u64> {
    let a4937 = 13u64;
    let a4938 = {
        let a4935 = 9u64;
        let a4936 = v0;
        nat_mul(a4935, a4936)?
    };
    nat_add(a4937, a4938)
}

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f192(v1, v2, v3, v4) {
        f250(v0)
    } else {
        f112(v0)
    }
}

pub fn f231(v0: u64) -> u64 {
    let a4939 = v0;
    let a4940 = 26u64;
    if nat_eq(a4939, a4940) {
        26u64
    } else {
        0u64
    }
}

pub fn f232(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4961 = {
        let a4955 = {
            let a4949 = {
                let a4943 = v1;
                let a4944 = if v5 {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4941, _, _, _) => {
                            h4941
                        }
                    })
                } else {
                    f133(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4942, _, _, _) => {
                            h4942
                        }
                    })
                };
                nat_mul(a4943, a4944)?
            };
            let a4950 = {
                let a4947 = v2;
                let a4948 = if v5 {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4945, _, _) => {
                            h4945
                        }
                    })
                } else {
                    f133(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4946, _, _) => {
                            h4946
                        }
                    })
                };
                nat_mul(a4947, a4948)?
            };
            nat_add(a4949, a4950)?
        };
        let a4956 = {
            let a4953 = v3;
            let a4954 = if v5 {
                f132(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4951, _) => {
                        h4951
                    }
                })
            } else {
                f133(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4952, _) => {
                        h4952
                    }
                })
            };
            nat_mul(a4953, a4954)?
        };
        nat_add(a4955, a4956)?
    };
    let a4962 = {
        let a4959 = v4;
        let a4960 = if v5 {
            f132(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4957) => {
                    h4957
                }
            })
        } else {
            f133(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4958) => {
                    h4958
                }
            })
        };
        nat_mul(a4959, a4960)?
    };
    nat_add(a4961, a4962)
}

pub fn f233(v0: u64, v1: u64) -> Option<u64> {
    let a4963 = v0;
    let a4964 = 2u64;
    if nat_eq(a4963, a4964) {
        Some(0u64)
    } else {
        let a4965 = v0;
        let a4966 = 5u64;
        if nat_eq(a4965, a4966) {
            Some(1u64)
        } else {
            let a4967 = v0;
            let a4968 = 6u64;
            if nat_eq(a4967, a4968) {
                Some(2u64)
            } else {
                let a4969 = v0;
                let a4970 = 7u64;
                if nat_eq(a4969, a4970) {
                    Some(3u64)
                } else {
                    let a4971 = v0;
                    let a4972 = 9u64;
                    if nat_eq(a4971, a4972) {
                        Some(4u64)
                    } else {
                        let a4973 = v0;
                        let a4974 = 8u64;
                        if nat_eq(a4973, a4974) {
                            Some(5u64)
                        } else {
                            let a4975 = v0;
                            let a4976 = 89u64;
                            if nat_eq(a4975, a4976) {
                                let a4977 = v1;
                                let a4978 = 2u64;
                                if nat_eq(a4977, a4978) {
                                    Some(6u64)
                                } else {
                                    let a4979 = v1;
                                    let a4980 = 0u64;
                                    if nat_eq(a4979, a4980) {
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

pub fn f234(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4981 = v0;
    let a4982 = 30u64;
    if nat_eq(a4981, a4982) {
        let a4983 = v2;
        let a4984 = 0u64;
        if if if nat_eq(a4983, a4984) {
            true
        } else {
            let a4985 = v2;
            let a4986 = 1u64;
            nat_eq(a4985, a4986)
        } {
            let a4987 = v1;
            let a4988 = 0u64;
            if nat_eq(a4987, a4988) {
                true
            } else {
                f251(v1, v4)
            }
        } else {
            false
        } {
            f252(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f253(v1) {
            f253(v2)
        } else {
            false
        } {
            let a4989 = v1;
            let a4990 = 0u64;
            if if nat_eq(a4989, a4990) {
                true
            } else {
                let a4991 = v2;
                let a4992 = 0u64;
                nat_eq(a4991, a4992)
            } {
                true
            } else {
                let a4993 = v1;
                let a4994 = v2;
                nat_eq(a4993, a4994)
            }
        } else {
            false
        } {
            true
        } else {
            let a4995 = v1;
            let a4996 = 43u64;
            if nat_eq(a4995, a4996) {
                f253(v2)
            } else {
                false
            }
        } {
            f252(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f235(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4997 = v1;
    let a4998 = 0u64;
    if nat_eq(a4997, a4998) {
        Ok(Adt9::C4({
            let a4999 = v3;
            let a5000 = 0u64;
            if nat_eq(a4999, a5000) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f139(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5001 = v3;
            let a5002 = 0u64;
            if nat_eq(a5001, a5002) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f236(v0: u64, v1: u64) -> R<u64> {
    let a5003 = v0;
    let a5004 = f94(v1)?;
    let a5005 = 0u64;
    Ok(nat_quot(a5003, a5004, a5005))
}

pub fn f237(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f236(v0, v1)?;
    let v3: u64 = {
        let a5008 = {
            let a5006 = f210(v0, v1)?;
            let a5007 = 2u64;
            nat_sub(a5006, a5007)
        };
        let a5009 = 2u64;
        let a5010 = 0u64;
        nat_quot(a5008, a5009, a5010)
    };
    let v4: u64 = {
        let a5011 = v2;
        let a5012 = v1;
        nat_mul(a5011, a5012)?
    };
    if f213(v0, v1)? {
        let a5013 = v4;
        let a5014 = v3;
        nat_add(a5013, a5014)
    } else {
        let a5026 = {
            let a5022 = v4;
            let a5023 = {
                let a5020 = {
                    let a5015 = v1;
                    let a5016 = 1u64;
                    nat_sub(a5015, a5016)
                };
                let a5021 = {
                    let a5017 = v2;
                    let a5018 = v1;
                    let a5019 = 0u64;
                    nat_rem(a5017, a5018, a5019)
                };
                nat_mul(a5020, a5021)?
            };
            nat_sub(a5022, a5023)
        };
        let a5027 = {
            let a5024 = v3;
            let a5025 = v1;
            nat_mul(a5024, a5025)?
        };
        nat_add(a5026, a5027)
    }
}

pub fn f238(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5028 = v0;
    let a5029 = 0u64;
    if if if nat_eq(a5028, a5029) {
        true
    } else {
        let a5030 = v1;
        let a5031 = 0u64;
        nat_eq(a5030, a5031)
    } {
        true
    } else {
        let a5032 = v2;
        let a5033 = 0u64;
        nat_eq(a5032, a5033)
    } {
        Ok(0u64)
    } else {
        let a5036 = {
            let a5034 = v0;
            let a5035 = v1;
            nat_mul(a5034, a5035)?
        };
        let a5037 = v2;
        nat_mul(a5036, a5037)
    }
}

pub fn f239(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f103(match v1.clone() {
        Adt20::C0(h5038, _, _, _, _, _, _, _) => {
            h5038
        }
    })?;
    let v6: u64 = {
        let a5044 = match v1.clone() {
            Adt20::C0(_, h5039, _, _, _, _, _, _) => {
                h5039
            }
        };
        let a5045 = {
            let a5042 = match v1.clone() {
                Adt20::C0(_, _, h5040, _, _, _, _, _) => {
                    h5040
                }
            };
            let a5043 = match v1.clone() {
                Adt20::C0(_, _, _, h5041, _, _, _, _) => {
                    h5041
                }
            };
            nat_mul(a5042, a5043)?
        };
        nat_mul(a5044, a5045)?
    };
    let a5047 = v0;
    let a5048 = f39(match v1.clone() {
        Adt20::C0(h5046, _, _, _, _, _, _, _) => {
            h5046
        }
    }, v6)?;
    if nat_lt(a5047, a5048) {
        f254({
            let a5049 = v0;
            let a5050 = v5;
            let a5051 = 0u64;
            nat_rem(a5049, a5050, a5051)
        }, {
            let a5052 = v0;
            let a5053 = v5;
            let a5054 = 0u64;
            nat_quot(a5052, a5053, a5054)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f240(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5055 = v6;
        let a5056 = 6u64;
        nat_sub(a5055, a5056)
    };
    let v9: Adt20 = f255(v3);
    let a5058 = v0;
    let a5059 = f188(match v1.clone() {
        Adt20::C0(h5057, _, _, _, _, _, _, _) => {
            h5057
        }
    }, v5, v8)?;
    if nat_lt(a5058, a5059) {
        let v10: u64 = f100(match v1.clone() {
            Adt20::C0(h5060, _, _, _, _, _, _, _) => {
                h5060
            }
        }, v5, v8)?;
        let a5061 = v0;
        let a5062 = v10;
        if nat_lt(a5061, a5062) {
            f256(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5063 = v0;
                let a5064 = v10;
                nat_sub(a5063, a5064)
            };
            let v12: u64 = f229(match v1.clone() {
                Adt20::C0(h5065, _, _, _, _, _, _, _) => {
                    h5065
                }
            })?;
            f257({
                let a5066 = v11;
                let a5067 = v12;
                let a5068 = 0u64;
                nat_rem(a5066, a5067, a5068)
            }, {
                let a5069 = v11;
                let a5070 = v12;
                let a5071 = 0u64;
                nat_quot(a5069, a5070, a5071)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f241(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5072 = v5;
        let a5073 = 6u64;
        nat_sub(a5072, a5073)
    };
    let v8: bool = f182(match v1.clone() {
        Adt20::C0(_, _, _, _, h5074, _, _, _) => {
            h5074
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5075, _, _, _) => {
            h5075
        }
    });
    let a5077 = v0;
    let a5078 = f181(v8, match v1.clone() {
        Adt20::C0(h5076, _, _, _, _, _, _, _) => {
            h5076
        }
    }, v4, v7)?;
    if nat_lt(a5077, a5078) {
        let v9: u64 = f100(match v1.clone() {
            Adt20::C0(h5079, _, _, _, _, _, _, _) => {
                h5079
            }
        }, v4, v7)?;
        let a5080 = v0;
        let a5081 = v9;
        if nat_lt(a5080, a5081) {
            f256(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5082 = v0;
                let a5083 = v9;
                nat_sub(a5082, a5083)
            };
            let v11: u64 = f225(v8, match v1.clone() {
                Adt20::C0(h5084, _, _, _, _, _, _, _) => {
                    h5084
                }
            })?;
            f258({
                let a5085 = v10;
                let a5086 = v11;
                let a5087 = 0u64;
                nat_rem(a5085, a5086, a5087)
            }, {
                let a5088 = v10;
                let a5089 = v11;
                let a5090 = 0u64;
                nat_quot(a5088, a5089, a5090)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f242(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5091 = v5;
        let a5092 = 6u64;
        nat_sub(a5091, a5092)
    };
    let v8: Adt20 = f255(v2);
    let v9: u64 = f227(match v1.clone() {
        Adt20::C0(h5093, _, _, _, _, _, _, _) => {
            h5093
        }
    }, match v1.clone() {
        Adt20::C0(_, h5094, _, _, _, _, _, _) => {
            h5094
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5095, _, _, _, _, _) => {
            h5095
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5096, _, _, _, _) => {
            h5096
        }
    }, v7)?;
    let a5101 = v0;
    let a5102 = f187(match v1.clone() {
        Adt20::C0(h5097, _, _, _, _, _, _, _) => {
            h5097
        }
    }, match v1.clone() {
        Adt20::C0(_, h5098, _, _, _, _, _, _) => {
            h5098
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5099, _, _, _, _, _) => {
            h5099
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5100, _, _, _, _) => {
            h5100
        }
    }, v4, v7)?;
    if nat_lt(a5101, a5102) {
        let a5103 = v0;
        let a5104 = v9;
        if nat_lt(a5103, a5104) {
            f256(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f259({
                let a5105 = v0;
                let a5106 = v9;
                nat_sub(a5105, a5106)
            }, match v1.clone() {
                Adt20::C0(h5107, _, _, _, _, _, _, _) => {
                    h5107
                }
            }, match v1.clone() {
                Adt20::C0(_, h5108, _, _, _, _, _, _) => {
                    h5108
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5109, _, _, _, _, _) => {
                    h5109
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5110, _, _, _, _) => {
                    h5110
                }
            }, v4)?;
            f260(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f243(v0: bool, v1: u64) -> R<u64> {
    let a5113 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5114 = {
        let a5111 = f261(v0);
        let a5112 = v1;
        nat_mul(a5111, a5112)?
    };
    nat_add(a5113, a5114)
}

pub fn f244(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5122 = {
            let a5115 = 13u64;
            let a5116 = f262(v1);
            nat_mul(a5115, a5116)?
        };
        let a5123 = {
            let a5120 = 4u64;
            let a5121 = {
                let a5117 = v1;
                let a5118 = 4u64;
                let a5119 = 0u64;
                nat_rem(a5117, a5118, a5119)
            };
            nat_mul(a5120, a5121)?
        };
        nat_add(a5122, a5123)
    } else {
        let a5124 = 3u64;
        let a5125 = v1;
        nat_mul(a5124, a5125)
    }
}

pub fn f245(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f246(v0: u64, v1: u64) -> R<u64> {
    let a5130 = {
        let a5128 = {
            let a5126 = v0;
            let a5127 = v1;
            nat_add(a5126, a5127)?
        };
        let a5129 = 1u64;
        nat_sub(a5128, a5129)
    };
    let a5131 = v1;
    let a5132 = 0u64;
    Ok(nat_quot(a5130, a5131, a5132))
}

pub fn f247(v0: u64, v1: u64) -> R<u64> {
    let a5133 = v0;
    let a5134 = f246(v0, v1)?;
    let a5135 = 0u64;
    Ok(nat_quot(a5133, a5134, a5135))
}

pub fn f248(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5136 = v2;
    let a5137 = 0u64;
    if nat_eq(a5136, a5137) {
        Ok(0u64)
    } else {
        let a5142 = 11u64;
        let a5143 = {
            let a5140 = {
                let a5138 = v1;
                let a5139 = v2;
                nat_mul(a5138, a5139)?
            };
            let a5141 = f263(v0)?;
            nat_mul(a5140, a5141)?
        };
        nat_add(a5142, a5143)
    }
}

pub fn f249(v0: u64, v1: u64) -> R<u64> {
    let a5144 = v0;
    let a5145 = f246(v0, v1)?;
    let a5146 = 0u64;
    Ok(nat_rem(a5144, a5145, a5146))
}

pub fn f250(v0: u64) -> bool {
    let m5147 = f114(v0);
    match m5147 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f251(v0: u64, v1: u64) -> bool {
    let a5148 = v0;
    let a5149 = 1u64;
    if nat_eq(a5148, a5149) {
        true
    } else {
        let v2: u64 = f264(v0);
        let a5150 = 0u64;
        let a5151 = v2;
        if nat_lt(a5150, a5151) {
            let a5155 = {
                let a5152 = v1;
                let a5153 = v2;
                let a5154 = 1u64;
                nat_rem(a5152, a5153, a5154)
            };
            let a5156 = 0u64;
            nat_eq(a5155, a5156)
        } else {
            false
        }
    }
}

pub fn f252(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5157 = v0;
    let a5158 = 28u64;
    if if if if nat_eq(a5157, a5158) {
        let a5159 = v1;
        let a5160 = 0u64;
        nat_eq(a5159, a5160)
    } else {
        false
    } {
        let a5161 = v2;
        let a5162 = 0u64;
        nat_eq(a5161, a5162)
    } else {
        false
    } {
        let a5163 = v3;
        let a5164 = 0u64;
        nat_eq(a5163, a5164)
    } else {
        false
    } {
        let a5165 = v4;
        let a5166 = v8;
        if if if if if if if if nat_eq(a5165, a5166) {
            let a5167 = v5;
            let a5168 = v12;
            nat_eq(a5167, a5168)
        } else {
            false
        } {
            let a5169 = v9;
            let a5170 = v13;
            nat_eq(a5169, a5170)
        } else {
            false
        } {
            let a5171 = v10;
            let a5172 = v14;
            nat_eq(a5171, a5172)
        } else {
            false
        } {
            let a5173 = v11;
            let a5174 = v15;
            nat_eq(a5173, a5174)
        } else {
            false
        } {
            let a5175 = 0u64;
            let a5176 = v6;
            nat_lt(a5175, a5176)
        } else {
            false
        } {
            let a5177 = 0u64;
            let a5178 = v7;
            nat_lt(a5177, a5178)
        } else {
            false
        } {
            let a5182 = {
                let a5179 = v10;
                let a5180 = v6;
                let a5181 = 1u64;
                nat_rem(a5179, a5180, a5181)
            };
            let a5183 = 0u64;
            if nat_eq(a5182, a5183) {
                let a5187 = {
                    let a5184 = v11;
                    let a5185 = v7;
                    let a5186 = 1u64;
                    nat_rem(a5184, a5185, a5186)
                };
                let a5188 = 0u64;
                nat_eq(a5187, a5188)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5189 = v10;
                let a5190 = v6;
                let a5191 = 0u64;
                nat_quot(a5189, a5190, a5191)
            }, {
                let a5192 = v11;
                let a5193 = v7;
                let a5194 = 0u64;
                nat_quot(a5192, a5193, a5194)
            }, {
                let a5195 = v5;
                let a5196 = v4;
                nat_mul(a5195, a5196)?
            }, {
                let a5197 = v4;
                let a5198 = v9;
                nat_mul(a5197, a5198)?
            }, {
                let a5199 = v5;
                let a5200 = v9;
                nat_mul(a5199, a5200)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f253(v0: u64) -> bool {
    let a5201 = v0;
    let a5202 = 0u64;
    if if nat_eq(a5201, a5202) {
        true
    } else {
        let a5203 = v0;
        let a5204 = 1u64;
        nat_eq(a5203, a5204)
    } {
        true
    } else {
        let a5205 = v0;
        let a5206 = 30u64;
        nat_eq(a5205, a5206)
    }
}

pub fn f254(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5207, _, _, _, _, _, _, _) => {
            h5207
        }
    };
    let v7: u64 = {
        let a5210 = 1u64;
        let a5211 = {
            let a5208 = 3u64;
            let a5209 = v6;
            nat_mul(a5208, a5209)?
        };
        nat_add(a5210, a5211)?
    };
    let a5212 = v0;
    let a5213 = 0u64;
    if nat_eq(a5212, a5213) {
        Ok(f265(0u64))
    } else {
        let a5214 = v0;
        let a5215 = v7;
        if nat_lt(a5214, a5215) {
            let v8: u64 = {
                let a5216 = v0;
                let a5217 = 1u64;
                nat_sub(a5216, a5217)
            };
            let v9: u64 = {
                let a5223 = {
                    let a5218 = v1;
                    let a5219 = v6;
                    nat_mul(a5218, a5219)?
                };
                let a5224 = {
                    let a5220 = v8;
                    let a5221 = 3u64;
                    let a5222 = 0u64;
                    nat_quot(a5220, a5221, a5222)
                };
                nat_add(a5223, a5224)?
            };
            let v10: u64 = f266(v9, v2)?;
            let v11: u64 = {
                let a5225 = v8;
                let a5226 = 3u64;
                let a5227 = 0u64;
                nat_rem(a5225, a5226, a5227)
            };
            let a5228 = v11;
            let a5229 = 0u64;
            if nat_eq(a5228, a5229) {
                Ok(f267(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5230 = v11;
                let a5231 = 1u64;
                if nat_eq(a5230, a5231) {
                    Ok(f267(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5232 = v0;
            let a5233 = v7;
            if nat_eq(a5232, a5233) {
                Ok(f267(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5236 = v0;
                let a5237 = {
                    let a5234 = v7;
                    let a5235 = 1u64;
                    nat_add(a5234, a5235)?
                };
                if nat_eq(a5236, a5237) {
                    Ok(f267(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5240 = v0;
                    let a5241 = {
                        let a5238 = v7;
                        let a5239 = 2u64;
                        nat_add(a5238, a5239)?
                    };
                    if nat_eq(a5240, a5241) {
                        Ok(f267(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5244 = v0;
                        let a5245 = {
                            let a5242 = v7;
                            let a5243 = 3u64;
                            nat_add(a5242, a5243)?
                        };
                        if nat_eq(a5244, a5245) {
                            let a5246 = v5;
                            let a5247 = 1u64;
                            if if nat_eq(a5246, a5247) {
                                true
                            } else {
                                let a5248 = v5;
                                let a5249 = 2u64;
                                nat_eq(a5248, a5249)
                            } {
                                Ok(f267(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f267(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5256 = {
                                    let a5250 = v1;
                                    let a5251 = v6;
                                    nat_mul(a5250, a5251)?
                                };
                                let a5257 = {
                                    let a5254 = v0;
                                    let a5255 = {
                                        let a5252 = v7;
                                        let a5253 = 4u64;
                                        nat_add(a5252, a5253)?
                                    };
                                    nat_sub(a5254, a5255)
                                };
                                nat_add(a5256, a5257)?
                            };
                            Ok(f267(10u64, 0u64, f266(v12, v2)?, 1u64, 2u64, 3u64, f266(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f255(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5258, _, _, _, _, _, _, _) => {
            h5258
        }
    }, match v0.clone() {
        Adt20::C0(_, h5259, _, _, _, _, _, _) => {
            h5259
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5260, _, _, _, _, _) => {
            h5260
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5261, _, _, _, _) => {
            h5261
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5262, _, _) => {
            h5262
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5263, _) => {
            h5263
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5264) => {
            h5264
        }
    })
}

pub fn f256(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5271 = v3;
        let a5272 = {
            let a5268 = f266({
                let a5265 = v0;
                let a5266 = 2u64;
                let a5267 = 0u64;
                nat_quot(a5265, a5266, a5267)
            }, v1)?;
            let a5269 = v2;
            let a5270 = 0u64;
            nat_quot(a5268, a5269, a5270)
        };
        nat_add(a5271, a5272)?
    };
    let a5276 = {
        let a5273 = v0;
        let a5274 = 2u64;
        let a5275 = 0u64;
        nat_rem(a5273, a5274, a5275)
    };
    let a5277 = 0u64;
    if nat_eq(a5276, a5277) {
        Ok(f267(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5278 = v4;
        let a5279 = 0u64;
        if nat_eq(a5278, a5279) {
            Ok(f267(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f257(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f268(match v2.clone() {
        Adt20::C0(h5280, _, _, _, _, _, _, _) => {
            h5280
        }
    })?;
    let v7: u64 = f269(match v2.clone() {
        Adt20::C0(h5281, _, _, _, _, _, _, _) => {
            h5281
        }
    })?;
    let a5282 = v0;
    let a5283 = 2u64;
    if nat_lt(a5282, a5283) {
        Ok(f265(v0))
    } else {
        let a5284 = v0;
        let a5285 = v6;
        if nat_lt(a5284, a5285) {
            f270({
                let a5286 = v0;
                let a5287 = 2u64;
                nat_sub(a5286, a5287)
            }, v1, v2, v3)
        } else {
            let a5288 = v0;
            let a5289 = v7;
            if nat_lt(a5288, a5289) {
                Ok(f271({
                    let a5290 = v0;
                    let a5291 = v6;
                    nat_sub(a5290, a5291)
                }, match v2 {
                    Adt20::C0(h5292, _, _, _, _, _, _, _) => {
                        h5292
                    }
                }, v5))
            } else {
                let a5294 = v0;
                let a5295 = f229(match v2.clone() {
                    Adt20::C0(h5293, _, _, _, _, _, _, _) => {
                        h5293
                    }
                })?;
                if nat_lt(a5294, a5295) {
                    f272({
                        let a5296 = v0;
                        let a5297 = v7;
                        nat_sub(a5296, a5297)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f258(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f243(v5, match v2.clone() {
        Adt20::C0(h5298, _, _, _, _, _, _, _) => {
            h5298
        }
    })?;
    let v7: u64 = {
        let a5300 = v6;
        let a5301 = f244(v5, match v2.clone() {
            Adt20::C0(h5299, _, _, _, _, _, _, _) => {
                h5299
            }
        })?;
        nat_add(a5300, a5301)?
    };
    let v8: u64 = {
        let a5302 = v7;
        let a5303 = f245(v5);
        nat_add(a5302, a5303)?
    };
    let a5304 = v0;
    let a5305 = v6;
    if nat_lt(a5304, a5305) {
        f273(v0, v1, v2, v5)
    } else {
        let a5306 = v0;
        let a5307 = v7;
        if nat_lt(a5306, a5307) {
            f274({
                let a5308 = v0;
                let a5309 = v6;
                nat_sub(a5308, a5309)
            }, v1, v2, v3, v5)
        } else {
            let a5310 = v0;
            let a5311 = v8;
            if nat_lt(a5310, a5311) {
                Ok(f275({
                    let a5312 = v0;
                    let a5313 = v7;
                    nat_sub(a5312, a5313)
                }, match v2 {
                    Adt20::C0(h5314, _, _, _, _, _, _, _) => {
                        h5314
                    }
                }, v4, v5))
            } else {
                let a5318 = v0;
                let a5319 = {
                    let a5316 = v8;
                    let a5317 = match v2.clone() {
                        Adt20::C0(h5315, _, _, _, _, _, _, _) => {
                            h5315
                        }
                    };
                    nat_add(a5316, a5317)?
                };
                if nat_lt(a5318, a5319) {
                    let v9: u64 = f266({
                        let a5325 = {
                            let a5321 = v1;
                            let a5322 = match v2 {
                                Adt20::C0(h5320, _, _, _, _, _, _, _) => {
                                    h5320
                                }
                            };
                            nat_mul(a5321, a5322)?
                        };
                        let a5326 = {
                            let a5323 = v0;
                            let a5324 = v8;
                            nat_sub(a5323, a5324)
                        };
                        nat_add(a5325, a5326)?
                    }, v3)?;
                    Ok(f267(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f259(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f246(v3, v5)?;
    let v7: u64 = f248(v1, v2, v6)?;
    let v8: u64 = {
        let a5329 = {
            let a5327 = f247(v3, v5)?;
            let a5328 = v4;
            nat_mul(a5327, a5328)?
        };
        let a5330 = v7;
        nat_mul(a5329, a5330)?
    };
    let a5331 = v0;
    let a5332 = v8;
    if nat_lt(a5331, a5332) {
        let v9: u64 = {
            let a5333 = v0;
            let a5334 = v7;
            let a5335 = 0u64;
            nat_quot(a5333, a5334, a5335)
        };
        Ok(Adt22::C0({
            let a5339 = {
                let a5336 = v9;
                let a5337 = v4;
                let a5338 = 0u64;
                nat_quot(a5336, a5337, a5338)
            };
            let a5340 = v6;
            nat_mul(a5339, a5340)?
        }, v6, {
            let a5341 = v9;
            let a5342 = v4;
            let a5343 = 0u64;
            nat_rem(a5341, a5342, a5343)
        }, {
            let a5344 = v0;
            let a5345 = v7;
            let a5346 = 0u64;
            nat_rem(a5344, a5345, a5346)
        }))
    } else {
        let v10: u64 = {
            let a5347 = v0;
            let a5348 = v8;
            nat_sub(a5347, a5348)
        };
        let v11: u64 = f248(v1, v2, f249(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5349 = f247(v3, v5)?;
            let a5350 = v6;
            nat_mul(a5349, a5350)?
        }, f249(v3, v5)?, {
            let a5351 = v10;
            let a5352 = v11;
            let a5353 = 0u64;
            nat_quot(a5351, a5352, a5353)
        }, {
            let a5354 = v10;
            let a5355 = v11;
            let a5356 = 0u64;
            nat_rem(a5354, a5355, a5356)
        }))
    }
}

pub fn f260(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5357) => {
            h5357
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5358, _, _, _, _, _, _, _) => {
            h5358
        }
    };
    let v6: u64 = {
        let a5361 = match v0.clone() {
            Adt22::C0(_, h5359, _, _) => {
                h5359
            }
        };
        let a5362 = match v1.clone() {
            Adt20::C0(_, h5360, _, _, _, _, _, _) => {
                h5360
            }
        };
        nat_mul(a5361, a5362)?
    };
    let v7: u64 = {
        let a5363 = v6;
        let a5364 = v5;
        nat_mul(a5363, a5364)?
    };
    let v8: u64 = {
        let a5367 = {
            let a5365 = 2u64;
            let a5366 = v5;
            nat_mul(a5365, a5366)?
        };
        let a5368 = 2u64;
        nat_add(a5367, a5368)?
    };
    let v9: u64 = {
        let a5371 = {
            let a5369 = 4u64;
            let a5370 = v5;
            nat_mul(a5369, a5370)?
        };
        let a5372 = 2u64;
        nat_add(a5371, a5372)?
    };
    let v10: u64 = {
        let a5375 = 1u64;
        let a5376 = {
            let a5373 = v6;
            let a5374 = v8;
            nat_mul(a5373, a5374)?
        };
        nat_add(a5375, a5376)?
    };
    let v11: u64 = {
        let a5377 = v10;
        let a5378 = 4u64;
        nat_add(a5377, a5378)?
    };
    let v12: u64 = {
        let a5381 = v11;
        let a5382 = {
            let a5379 = v6;
            let a5380 = v9;
            nat_mul(a5379, a5380)?
        };
        nat_add(a5381, a5382)?
    };
    let v13: u64 = {
        let a5383 = v12;
        let a5384 = 6u64;
        nat_add(a5383, a5384)?
    };
    let a5385 = v4;
    let a5386 = 0u64;
    if nat_eq(a5385, a5386) {
        Ok(f265(0u64))
    } else {
        let a5387 = v4;
        let a5388 = v10;
        if nat_lt(a5387, a5388) {
            let v14: u64 = {
                let a5389 = v4;
                let a5390 = 1u64;
                nat_sub(a5389, a5390)
            };
            f276(false, {
                let a5391 = v14;
                let a5392 = v8;
                let a5393 = 0u64;
                nat_rem(a5391, a5392, a5393)
            }, {
                let a5394 = v14;
                let a5395 = v8;
                let a5396 = 0u64;
                nat_quot(a5394, a5395, a5396)
            }, v0, v1, v2)
        } else {
            let a5397 = v4;
            let a5398 = v11;
            if nat_lt(a5397, a5398) {
                Ok(f277({
                    let a5399 = v4;
                    let a5400 = v10;
                    nat_sub(a5399, a5400)
                }, v7))
            } else {
                let a5401 = v4;
                let a5402 = v12;
                if nat_lt(a5401, a5402) {
                    let v15: u64 = {
                        let a5403 = v4;
                        let a5404 = v11;
                        nat_sub(a5403, a5404)
                    };
                    f276(true, {
                        let a5405 = v15;
                        let a5406 = v9;
                        let a5407 = 0u64;
                        nat_rem(a5405, a5406, a5407)
                    }, {
                        let a5408 = v15;
                        let a5409 = v9;
                        let a5410 = 0u64;
                        nat_quot(a5408, a5409, a5410)
                    }, v0, v1, v2)
                } else {
                    let a5411 = v4;
                    let a5412 = v13;
                    if nat_lt(a5411, a5412) {
                        Ok(f278({
                            let a5413 = v4;
                            let a5414 = v12;
                            nat_sub(a5413, a5414)
                        }, v7, v3))
                    } else {
                        let a5417 = v4;
                        let a5418 = {
                            let a5415 = v13;
                            let a5416 = v7;
                            nat_add(a5415, a5416)?
                        };
                        if nat_lt(a5417, a5418) {
                            let v16: u64 = {
                                let a5419 = v4;
                                let a5420 = v13;
                                nat_sub(a5419, a5420)
                            };
                            let v17: u64 = {
                                let a5421 = v16;
                                let a5422 = v5;
                                let a5423 = 0u64;
                                nat_quot(a5421, a5422, a5423)
                            };
                            let v18: u64 = {
                                let a5429 = {
                                    let a5424 = f279(v17, v0, v1)?;
                                    let a5425 = v5;
                                    nat_mul(a5424, a5425)?
                                };
                                let a5430 = {
                                    let a5426 = v16;
                                    let a5427 = v5;
                                    let a5428 = 0u64;
                                    nat_rem(a5426, a5427, a5428)
                                };
                                nat_add(a5429, a5430)?
                            };
                            let v19: u64 = f266(v18, v2)?;
                            Ok(f267(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f261(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f262(v0: u64) -> u64 {
    let a5431 = v0;
    let a5432 = 4u64;
    let a5433 = 0u64;
    nat_quot(a5431, a5432, a5433)
}

pub fn f263(v0: u64) -> R<u64> {
    let a5436 = {
        let a5434 = 7u64;
        let a5435 = v0;
        nat_mul(a5434, a5435)?
    };
    let a5437 = 4u64;
    nat_add(a5436, a5437)
}

pub fn f264(v0: u64) -> u64 {
    if f112(v0) {
        let m5438 = f114(v0);
        match m5438 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5440 = 1u64;
                let a5441 = match v1.clone() {
                    Adt18::C0(_, h5439, _) => {
                        h5439
                    }
                };
                if if nat_lt(a5440, a5441) {
                    true
                } else {
                    let a5442 = v0;
                    let a5443 = 43u64;
                    nat_eq(a5442, a5443)
                } {
                    match v1 {
                        Adt18::C0(_, h5444, _) => {
                            h5444
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

pub fn f265(v0: u64) -> Adt19 {
    f267(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f266(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5446 = v0;
        let a5447 = match v1.clone() {
            Adt20::C0(h5445, _, _, _, _, _, _, _) => {
                h5445
            }
        };
        let a5448 = 0u64;
        nat_quot(a5446, a5447, a5448)
    };
    let v3: u64 = {
        let a5450 = v2;
        let a5451 = match v1.clone() {
            Adt20::C0(_, h5449, _, _, _, _, _, _) => {
                h5449
            }
        };
        let a5452 = 0u64;
        nat_quot(a5450, a5451, a5452)
    };
    f131({
        let a5454 = v0;
        let a5455 = match v1.clone() {
            Adt20::C0(h5453, _, _, _, _, _, _, _) => {
                h5453
            }
        };
        let a5456 = 0u64;
        nat_rem(a5454, a5455, a5456)
    }, {
        let a5458 = v2;
        let a5459 = match v1.clone() {
            Adt20::C0(_, h5457, _, _, _, _, _, _) => {
                h5457
            }
        };
        let a5460 = 0u64;
        nat_rem(a5458, a5459, a5460)
    }, {
        let a5462 = v3;
        let a5463 = match v1.clone() {
            Adt20::C0(_, _, h5461, _, _, _, _, _) => {
                h5461
            }
        };
        let a5464 = 0u64;
        nat_rem(a5462, a5463, a5464)
    }, {
        let a5466 = v3;
        let a5467 = match v1.clone() {
            Adt20::C0(_, _, h5465, _, _, _, _, _) => {
                h5465
            }
        };
        let a5468 = 0u64;
        nat_quot(a5466, a5467, a5468)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5469, _, _, _) => {
            h5469
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5470, _, _) => {
            h5470
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5471, _) => {
            h5471
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5472) => {
            h5472
        }
    })
}

pub fn f267(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5473 = v0;
    let a5474 = 4u64;
    if if if nat_le(a5473, a5474) {
        true
    } else {
        let a5475 = 9u64;
        let a5476 = v0;
        if nat_le(a5475, a5476) {
            let a5477 = v0;
            let a5478 = 12u64;
            nat_le(a5477, a5478)
        } else {
            false
        }
    } {
        true
    } else {
        let a5479 = v0;
        let a5480 = 16u64;
        if if nat_eq(a5479, a5480) {
            true
        } else {
            let a5481 = v0;
            let a5482 = 19u64;
            nat_eq(a5481, a5482)
        } {
            true
        } else {
            let a5483 = v0;
            let a5484 = 21u64;
            if nat_eq(a5483, a5484) {
                true
            } else {
                let a5485 = v0;
                let a5486 = 22u64;
                nat_eq(a5485, a5486)
            }
        }
    } {
        Adt19::C1({
            let a5487 = v0;
            let a5488 = 4u64;
            if if nat_lt(a5487, a5488) {
                true
            } else {
                let a5489 = 21u64;
                let a5490 = v0;
                nat_le(a5489, a5490)
            } {
                0u64
            } else {
                let a5491 = v0;
                let a5492 = 16u64;
                if if nat_eq(a5491, a5492) {
                    true
                } else {
                    let a5493 = v0;
                    let a5494 = 19u64;
                    nat_eq(a5493, a5494)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5495 = v0;
            let a5496 = 4u64;
            if nat_lt(a5495, a5496) {
                v0
            } else {
                let a5497 = v0;
                let a5498 = 4u64;
                if if nat_eq(a5497, a5498) {
                    true
                } else {
                    let a5499 = v0;
                    let a5500 = 16u64;
                    nat_eq(a5499, a5500)
                } {
                    0u64
                } else {
                    let a5501 = v0;
                    let a5502 = 19u64;
                    if nat_eq(a5501, a5502) {
                        3u64
                    } else {
                        let a5503 = 21u64;
                        let a5504 = v0;
                        if nat_le(a5503, a5504) {
                            let a5505 = v0;
                            let a5506 = 17u64;
                            nat_sub(a5505, a5506)
                        } else {
                            let a5507 = v0;
                            let a5508 = 8u64;
                            nat_sub(a5507, a5508)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f268(v0: u64) -> R<u64> {
    let a5511 = 2u64;
    let a5512 = {
        let a5509 = 6u64;
        let a5510 = v0;
        nat_mul(a5509, a5510)?
    };
    nat_add(a5511, a5512)
}

pub fn f269(v0: u64) -> R<u64> {
    let a5513 = f268(v0)?;
    let a5514 = 11u64;
    nat_add(a5513, a5514)
}

pub fn f270(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5515 = v0;
        let a5516 = 6u64;
        let a5517 = 0u64;
        nat_rem(a5515, a5516, a5517)
    };
    let v5: u64 = {
        let a5524 = {
            let a5519 = v1;
            let a5520 = match v2.clone() {
                Adt20::C0(h5518, _, _, _, _, _, _, _) => {
                    h5518
                }
            };
            nat_mul(a5519, a5520)?
        };
        let a5525 = {
            let a5521 = v0;
            let a5522 = 6u64;
            let a5523 = 0u64;
            nat_quot(a5521, a5522, a5523)
        };
        nat_add(a5524, a5525)?
    };
    let v6: u64 = f266(v5, v3)?;
    let v7: u64 = f266(v5, v2)?;
    let a5526 = v4;
    let a5527 = 0u64;
    if nat_eq(a5526, a5527) {
        Ok(f267(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5528 = v4;
        let a5529 = 1u64;
        if nat_eq(a5528, a5529) {
            Ok(f267(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5530 = v4;
            let a5531 = 2u64;
            if nat_eq(a5530, a5531) {
                Ok(f267(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5532 = v4;
                let a5533 = 3u64;
                if nat_eq(a5532, a5533) {
                    Ok(f267(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5534 = v4;
                    let a5535 = 4u64;
                    if nat_eq(a5534, a5535) {
                        Ok(f267(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f267(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f271(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5536 = v0;
    let a5537 = 0u64;
    if nat_eq(a5536, a5537) {
        f267(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5538 = v0;
        let a5539 = 1u64;
        if nat_eq(a5538, a5539) {
            f267(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5540 = v0;
            let a5541 = 2u64;
            if nat_eq(a5540, a5541) {
                f267(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5542 = v0;
                let a5543 = 3u64;
                if nat_eq(a5542, a5543) {
                    f267(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5544 = v0;
                    let a5545 = 4u64;
                    if nat_eq(a5544, a5545) {
                        f267(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5546 = v0;
                        let a5547 = 5u64;
                        if nat_eq(a5546, a5547) {
                            f267(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5548 = v0;
                            let a5549 = 6u64;
                            if nat_eq(a5548, a5549) {
                                f267(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5550 = v0;
                                let a5551 = 7u64;
                                if nat_eq(a5550, a5551) {
                                    f267(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5552 = v0;
                                    let a5553 = 8u64;
                                    if nat_eq(a5552, a5553) {
                                        f267(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5554 = v0;
                                        let a5555 = 9u64;
                                        if nat_eq(a5554, a5555) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5556 = v0;
                                            let a5557 = 10u64;
                                            if nat_eq(a5556, a5557) {
                                                f267(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f272(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5558 = v0;
        let a5559 = 3u64;
        let a5560 = 0u64;
        nat_rem(a5558, a5559, a5560)
    };
    let v6: u64 = {
        let a5567 = {
            let a5562 = v1;
            let a5563 = match v2.clone() {
                Adt20::C0(h5561, _, _, _, _, _, _, _) => {
                    h5561
                }
            };
            nat_mul(a5562, a5563)?
        };
        let a5568 = {
            let a5564 = v0;
            let a5565 = 3u64;
            let a5566 = 0u64;
            nat_quot(a5564, a5565, a5566)
        };
        nat_add(a5567, a5568)?
    };
    let v7: u64 = f266(v6, v3)?;
    let v8: u64 = f266(v6, v2)?;
    let v9: u64 = f266(v6, v4)?;
    let a5569 = v5;
    let a5570 = 0u64;
    if nat_eq(a5569, a5570) {
        Ok(f267(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5571 = v5;
        let a5572 = 1u64;
        if nat_eq(a5571, a5572) {
            Ok(f267(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f267(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f273(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5573, _, _, _, _, _, _, _) => {
            h5573
        }
    };
    let v5: u64 = {
        let a5576 = 1u64;
        let a5577 = {
            let a5574 = v4;
            let a5575 = f261(v3);
            nat_mul(a5574, a5575)?
        };
        nat_add(a5576, a5577)?
    };
    let a5578 = v0;
    let a5579 = 0u64;
    if nat_eq(a5578, a5579) {
        Ok(f265(0u64))
    } else {
        let a5580 = v0;
        let a5581 = v5;
        if nat_lt(a5580, a5581) {
            let v6: u64 = {
                let a5582 = v0;
                let a5583 = 1u64;
                nat_sub(a5582, a5583)
            };
            let v7: u64 = f266({
                let a5589 = {
                    let a5584 = v1;
                    let a5585 = v4;
                    nat_mul(a5584, a5585)?
                };
                let a5590 = {
                    let a5586 = v6;
                    let a5587 = f261(v3);
                    let a5588 = 0u64;
                    nat_quot(a5586, a5587, a5588)
                };
                nat_add(a5589, a5590)?
            }, v2)?;
            if v3 {
                let a5594 = {
                    let a5591 = v6;
                    let a5592 = 2u64;
                    let a5593 = 0u64;
                    nat_rem(a5591, a5592, a5593)
                };
                let a5595 = 0u64;
                if nat_eq(a5594, a5595) {
                    Ok(f267(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f267(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5596 = v0;
                let a5597 = v5;
                nat_sub(a5596, a5597)
            };
            let a5598 = v8;
            let a5599 = 0u64;
            if nat_eq(a5598, a5599) {
                Ok(f267(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5600 = v8;
                    let a5601 = 1u64;
                    nat_eq(a5600, a5601)
                } else {
                    false
                } {
                    Ok(f267(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5602 = v8;
                    let a5603 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5602, a5603) {
                        Ok(f267(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5604 = v8;
                        let a5605 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5604, a5605) {
                            Ok(f265(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f274(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5607 = 13u64;
        let a5608 = f262(match v2.clone() {
            Adt20::C0(h5606, _, _, _, _, _, _, _) => {
                h5606
            }
        });
        nat_mul(a5607, a5608)?
    } else {
        0u64
    };
    let a5609 = v0;
    let a5610 = v5;
    if nat_lt(a5609, a5610) {
        f280({
            let a5611 = v0;
            let a5612 = 13u64;
            let a5613 = 0u64;
            nat_rem(a5611, a5612, a5613)
        }, v1, {
            let a5614 = v0;
            let a5615 = 13u64;
            let a5616 = 0u64;
            nat_quot(a5614, a5615, a5616)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5617 = v0;
            let a5618 = v5;
            nat_sub(a5617, a5618)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f281({
            let a5619 = v6;
            let a5620 = v7;
            let a5621 = 0u64;
            nat_rem(a5619, a5620, a5621)
        }, {
            let a5631 = {
                let a5626 = {
                    let a5623 = v1;
                    let a5624 = match v2.clone() {
                        Adt20::C0(h5622, _, _, _, _, _, _, _) => {
                            h5622
                        }
                    };
                    nat_mul(a5623, a5624)?
                };
                let a5627 = if v4 {
                    f282(match v2.clone() {
                        Adt20::C0(h5625, _, _, _, _, _, _, _) => {
                            h5625
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5626, a5627)?
            };
            let a5632 = {
                let a5628 = v6;
                let a5629 = v7;
                let a5630 = 0u64;
                nat_quot(a5628, a5629, a5630)
            };
            nat_add(a5631, a5632)?
        }, v2, v3, v4)
    }
}

pub fn f275(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5633 = v0;
    let a5634 = 0u64;
    if nat_eq(a5633, a5634) {
        f267(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5635 = v0;
        let a5636 = 1u64;
        if nat_eq(a5635, a5636) {
            f267(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5637 = v0;
                let a5638 = 2u64;
                nat_eq(a5637, a5638)
            } else {
                false
            } {
                f267(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5639 = v0;
                let a5640 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5639, a5640) {
                    f267(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5641 = v0;
                    let a5642 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5641, a5642) {
                        f267(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5643 = v0;
                        let a5644 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5643, a5644) {
                            f267(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f276(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5648 = 1u64;
        let a5649 = {
            let a5646 = match v4.clone() {
                Adt20::C0(h5645, _, _, _, _, _, _, _) => {
                    h5645
                }
            };
            let a5647 = v6;
            nat_mul(a5646, a5647)?
        };
        nat_add(a5648, a5649)?
    };
    let a5650 = v1;
    let a5651 = 0u64;
    if nat_eq(a5650, a5651) {
        Ok(f265(1u64))
    } else {
        let a5652 = v1;
        let a5653 = v7;
        if nat_lt(a5652, a5653) {
            let v8: u64 = {
                let a5654 = v1;
                let a5655 = 1u64;
                nat_sub(a5654, a5655)
            };
            let v9: u64 = {
                let a5662 = {
                    let a5657 = f279(v2, v3, v4.clone())?;
                    let a5658 = match v4.clone() {
                        Adt20::C0(h5656, _, _, _, _, _, _, _) => {
                            h5656
                        }
                    };
                    nat_mul(a5657, a5658)?
                };
                let a5663 = {
                    let a5659 = v8;
                    let a5660 = v6;
                    let a5661 = 0u64;
                    nat_quot(a5659, a5660, a5661)
                };
                nat_add(a5662, a5663)?
            };
            let v10: u64 = f266(v9, v4)?;
            let v11: u64 = f266(v9, v5)?;
            let v12: u64 = {
                let a5664 = v8;
                let a5665 = v6;
                let a5666 = 0u64;
                nat_rem(a5664, a5665, a5666)
            };
            if v0 {
                let a5667 = v12;
                let a5668 = 0u64;
                if nat_eq(a5667, a5668) {
                    Ok(f267(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5669 = v12;
                    let a5670 = 1u64;
                    if nat_eq(a5669, a5670) {
                        Ok(f267(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5671 = v12;
                        let a5672 = 2u64;
                        if nat_eq(a5671, a5672) {
                            Ok(f267(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f267(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5673 = v12;
                let a5674 = 0u64;
                if nat_eq(a5673, a5674) {
                    Ok(f267(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f267(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5675 = v1;
            let a5676 = v7;
            if nat_eq(a5675, a5676) {
                Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f277(v0: u64, v1: u64) -> Adt19 {
    let a5677 = v0;
    let a5678 = 0u64;
    if nat_eq(a5677, a5678) {
        f267(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5679 = v0;
        let a5680 = 1u64;
        if nat_eq(a5679, a5680) {
            f267(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5681 = v0;
            let a5682 = 2u64;
            if nat_eq(a5681, a5682) {
                f267(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5683 = v0;
                let a5684 = 3u64;
                if nat_eq(a5683, a5684) {
                    f265(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f278(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5685 = v0;
    let a5686 = 0u64;
    if nat_eq(a5685, a5686) {
        f267(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5687 = v0;
        let a5688 = 1u64;
        if nat_eq(a5687, a5688) {
            f267(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5689 = v0;
            let a5690 = 2u64;
            if nat_eq(a5689, a5690) {
                f267(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5691 = v0;
                let a5692 = 3u64;
                if nat_eq(a5691, a5692) {
                    f267(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5693 = v0;
                    let a5694 = 4u64;
                    if nat_eq(a5693, a5694) {
                        f267(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5695 = v0;
                        let a5696 = 5u64;
                        if nat_eq(a5695, a5696) {
                            f267(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f279(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5707 = {
        let a5705 = {
            let a5702 = {
                let a5699 = match v1.clone() {
                    Adt22::C0(_, _, h5697, _) => {
                        h5697
                    }
                };
                let a5700 = match v2.clone() {
                    Adt20::C0(_, _, h5698, _, _, _, _, _) => {
                        h5698
                    }
                };
                nat_mul(a5699, a5700)?
            };
            let a5703 = match v1 {
                Adt22::C0(h5701, _, _, _) => {
                    h5701
                }
            };
            nat_add(a5702, a5703)?
        };
        let a5706 = match v2 {
            Adt20::C0(_, h5704, _, _, _, _, _, _) => {
                h5704
            }
        };
        nat_mul(a5705, a5706)?
    };
    let a5708 = v0;
    nat_add(a5707, a5708)
}

pub fn f280(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5716 = {
            let a5714 = {
                let a5710 = v1;
                let a5711 = match v3.clone() {
                    Adt20::C0(h5709, _, _, _, _, _, _, _) => {
                        h5709
                    }
                };
                nat_mul(a5710, a5711)?
            };
            let a5715 = {
                let a5712 = v2;
                let a5713 = 4u64;
                nat_mul(a5712, a5713)?
            };
            nat_add(a5714, a5715)?
        };
        let a5717 = f283(v0);
        nat_add(a5716, a5717)?
    };
    let v6: u64 = f266(v5, v3)?;
    let v7: u64 = f266(v5, v4)?;
    let a5718 = v0;
    let a5719 = 0u64;
    if if if if nat_eq(a5718, a5719) {
        true
    } else {
        let a5720 = v0;
        let a5721 = 2u64;
        nat_eq(a5720, a5721)
    } {
        true
    } else {
        let a5722 = v0;
        let a5723 = 5u64;
        nat_eq(a5722, a5723)
    } {
        true
    } else {
        let a5724 = v0;
        let a5725 = 7u64;
        nat_eq(a5724, a5725)
    } {
        Ok(f267(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5726 = v0;
        let a5727 = 1u64;
        if if if if nat_eq(a5726, a5727) {
            true
        } else {
            let a5728 = v0;
            let a5729 = 3u64;
            nat_eq(a5728, a5729)
        } {
            true
        } else {
            let a5730 = v0;
            let a5731 = 6u64;
            nat_eq(a5730, a5731)
        } {
            true
        } else {
            let a5732 = v0;
            let a5733 = 8u64;
            nat_eq(a5732, a5733)
        } {
            Ok(f267(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5734 = v0;
                let a5735 = 1u64;
                if nat_eq(a5734, a5735) {
                    4u64
                } else {
                    let a5736 = v0;
                    let a5737 = 6u64;
                    if nat_eq(a5736, a5737) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5738 = v0;
            let a5739 = 4u64;
            if nat_eq(a5738, a5739) {
                Ok(f267(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5740 = v0;
                let a5741 = 9u64;
                if nat_eq(a5740, a5741) {
                    Ok(f267(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5742 = v0;
                    let a5743 = 10u64;
                    if nat_eq(a5742, a5743) {
                        Ok(f267(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5744 = v0;
                        let a5745 = 11u64;
                        if nat_eq(a5744, a5745) {
                            Ok(f267(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5746 = v0;
                            let a5747 = 12u64;
                            if nat_eq(a5746, a5747) {
                                Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f281(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f266(v1, v2)?;
    let v6: u64 = f266(v1, v3)?;
    let a5748 = v0;
    let a5749 = 0u64;
    if nat_eq(a5748, a5749) {
        Ok(f267(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5750 = v0;
        let a5751 = 1u64;
        if nat_eq(a5750, a5751) {
            Ok(f267(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5752 = v0;
                let a5753 = 2u64;
                if nat_eq(a5752, a5753) {
                    Ok(f267(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5754 = v0;
                    let a5755 = 3u64;
                    if nat_eq(a5754, a5755) {
                        Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5756 = v0;
                let a5757 = 2u64;
                if nat_eq(a5756, a5757) {
                    Ok(f267(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f282(v0: u64) -> R<u64> {
    let a5758 = 4u64;
    let a5759 = f262(v0);
    nat_mul(a5758, a5759)
}

pub fn f283(v0: u64) -> u64 {
    let a5760 = v0;
    let a5761 = 5u64;
    if nat_lt(a5760, a5761) {
        let a5762 = v0;
        let a5763 = 2u64;
        if nat_lt(a5762, a5763) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5764 = v0;
        let a5765 = 7u64;
        if nat_lt(a5764, a5765) {
            2u64
        } else {
            3u64
        }
    }
}
