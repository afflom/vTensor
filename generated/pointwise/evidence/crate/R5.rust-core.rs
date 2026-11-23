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
            if if if nat_eq(a2770, a2771) {
                let a2772 = v4;
                let a2773 = 43u64;
                nat_eq(a2772, a2773)
            } else {
                false
            } {
                true
            } else {
                let a2774 = v3;
                let a2775 = 43u64;
                if nat_eq(a2774, a2775) {
                    let a2776 = v4;
                    let a2777 = 0u64;
                    nat_eq(a2776, a2777)
                } else {
                    false
                }
            } {
                f156(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2778 = v1;
        let a2779 = 1u64;
        if nat_eq(a2778, a2779) {
            Adt11::C1(v2)
        } else {
            let a2780 = v1;
            let a2781 = 2u64;
            if nat_eq(a2780, a2781) {
                Adt11::C5(v2)
            } else {
                let a2782 = v1;
                let a2783 = 3u64;
                if nat_eq(a2782, a2783) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2784 = v1;
                    let a2785 = 4u64;
                    if nat_eq(a2784, a2785) {
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
    let a2786 = v0;
    let a2787 = 0u64;
    if nat_eq(a2786, a2787) {
        Adt12::C0(0u64, v1)
    } else {
        let a2788 = v0;
        let a2789 = 1u64;
        if nat_eq(a2788, a2789) {
            Adt12::C1(v1)
        } else {
            let a2790 = v0;
            let a2791 = 2u64;
            if nat_eq(a2790, a2791) {
                Adt12::C2(v1)
            } else {
                let a2792 = v0;
                let a2793 = 3u64;
                if nat_eq(a2792, a2793) {
                    Adt12::C3(v1)
                } else {
                    let a2794 = v0;
                    let a2795 = 13u64;
                    if nat_lt(a2794, a2795) {
                        Adt12::C0({
                            let a2796 = v0;
                            let a2797 = 3u64;
                            nat_sub(a2796, a2797)
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
    let a2798 = v0;
    let a2799 = 1u64;
    if nat_eq(a2798, a2799) {
        Adt13::C0(v1)
    } else {
        let a2800 = v0;
        let a2801 = 3u64;
        if if nat_eq(a2800, a2801) {
            true
        } else {
            let a2802 = v0;
            let a2803 = 6u64;
            nat_eq(a2802, a2803)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f92(v0: Adt6, v1: u64) -> R<u64> {
    let a2805 = 0u64;
    let a2806 = match v0.clone() {
        Adt6::C0(h2804, _, _, _) => {
            h2804
        }
    };
    if if if if if nat_lt(a2805, a2806) {
        let a2808 = 0u64;
        let a2809 = match v0.clone() {
            Adt6::C0(_, h2807, _, _) => {
                h2807
            }
        };
        nat_lt(a2808, a2809)
    } else {
        false
    } {
        let a2811 = 0u64;
        let a2812 = match v0.clone() {
            Adt6::C0(_, _, _, h2810) => {
                h2810
            }
        };
        nat_lt(a2811, a2812)
    } else {
        false
    } {
        let a2814 = match v0.clone() {
            Adt6::C0(_, h2813, _, _) => {
                h2813
            }
        };
        let a2815 = 2147483648u64;
        nat_lt(a2814, a2815)
    } else {
        false
    } {
        let a2817 = match v0.clone() {
            Adt6::C0(_, _, _, h2816) => {
                h2816
            }
        };
        let a2818 = 2147483648u64;
        nat_lt(a2817, a2818)
    } else {
        false
    } {
        let v2: u64 = {
            let a2822 = v1;
            let a2823 = {
                let a2820 = 2u64;
                let a2821 = f133(match v0.clone() {
                    Adt6::C0(_, _, h2819, _) => {
                        h2819
                    }
                });
                nat_mul(a2820, a2821)?
            };
            nat_add(a2822, a2823)?
        };
        let v3: u64 = {
            let a2835 = {
                let a2830 = 1u64;
                let a2831 = {
                    let a2828 = match v0.clone() {
                        Adt6::C0(_, _, _, h2824) => {
                            h2824
                        }
                    };
                    let a2829 = {
                        let a2826 = match v0.clone() {
                            Adt6::C0(h2825, _, _, _) => {
                                h2825
                            }
                        };
                        let a2827 = 1u64;
                        nat_sub(a2826, a2827)
                    };
                    nat_mul(a2828, a2829)?
                };
                nat_add(a2830, a2831)?
            };
            let a2836 = {
                let a2833 = 2u64;
                let a2834 = f132(match v0.clone() {
                    Adt6::C0(_, _, h2832, _) => {
                        h2832
                    }
                });
                nat_mul(a2833, a2834)?
            };
            nat_add(a2835, a2836)?
        };
        let a2837 = v3;
        let a2838 = v2;
        if nat_le(a2837, a2838) {
            let a2845 = 1u64;
            let a2846 = {
                let a2842 = {
                    let a2839 = v2;
                    let a2840 = v3;
                    nat_sub(a2839, a2840)
                };
                let a2843 = match v0 {
                    Adt6::C0(_, h2841, _, _) => {
                        h2841
                    }
                };
                let a2844 = 0u64;
                nat_quot(a2842, a2843, a2844)
            };
            nat_add(a2845, a2846)
        } else {
            let a2850 = {
                let a2847 = v3;
                let a2848 = v2;
                nat_sub(a2847, a2848)
            };
            let a2851 = match v0 {
                Adt6::C0(_, h2849, _, _) => {
                    h2849
                }
            };
            if nat_lt(a2850, a2851) {
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
        let a2852 = v0;
        let a2853 = 4294967296u64;
        let a2854 = 0u64;
        nat_rem(a2852, a2853, a2854)
    };
    let a2855 = v1;
    let a2856 = 2147483648u64;
    if nat_lt(a2855, a2856) {
        v1
    } else {
        0u64
    }
}

pub fn f94(v0: u64) -> R<u64> {
    let a2859 = 2u64;
    let a2860 = {
        let a2857 = 2u64;
        let a2858 = v0;
        nat_mul(a2857, a2858)?
    };
    nat_add(a2859, a2860)
}

pub fn f95(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2862 = match v1.clone() {
        Adt14::C0(h2861, _, _, _, _, _, _, _, _, _, _) => {
            h2861
        }
    };
    let a2863 = 0u64;
    if if if if nat_eq(a2862, a2863) {
        let a2865 = match v2.clone() {
            Adt14::C0(h2864, _, _, _, _, _, _, _, _, _, _) => {
                h2864
            }
        };
        let a2866 = 0u64;
        nat_eq(a2865, a2866)
    } else {
        false
    } {
        f174(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f175(match v1.clone() {
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
            Adt14::C0(_, h2871, _, _, _, _, _, _, _, _, _) => {
                h2871
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2872, _, _, _, _, _, _, _, _) => {
                h2872
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2873, _, _, _, _, _, _, _) => {
                h2873
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2874, _, _, _, _, _, _) => {
                h2874
            }
        }) {
            0u64
        } else {
            let a2880 = match v1.clone() {
                Adt14::C0(_, _, h2875, _, _, _, _, _, _, _, _) => {
                    h2875
                }
            };
            let a2881 = {
                let a2878 = match v1.clone() {
                    Adt14::C0(_, _, _, h2876, _, _, _, _, _, _, _) => {
                        h2876
                    }
                };
                let a2879 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2877, _, _, _, _, _, _) => {
                        h2877
                    }
                };
                nat_mul(a2878, a2879)?
            };
            nat_mul(a2880, a2881)?
        };
        let v4: u64 = {
            let a2882 = v3;
            let a2883 = 0u64;
            if if nat_eq(a2882, a2883) {
                true
            } else {
                f180(v2.clone())?
            } {
                0u64
            } else {
                let a2885 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2884, _) => {
                        h2884
                    }
                };
                let a2886 = 4u64;
                let a2887 = 0u64;
                nat_quot(a2885, a2886, a2887)
            }
        };
        Ok(Adt15::C1(f181(f182(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2888, _, _, _, _, _) => {
                h2888
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2889, _, _, _, _, _) => {
                h2889
            }
        }), match v1 {
            Adt14::C0(_, h2890, _, _, _, _, _, _, _, _, _) => {
                h2890
            }
        }, v3, v4)?, 4u64, 64u64, v3, f183(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f96(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f184(v3.clone());
    let a2892 = match v2.clone() {
        Adt14::C0(h2891, _, _, _, _, _, _, _, _, _, _) => {
            h2891
        }
    };
    let a2893 = 0u64;
    if if if if nat_eq(a2892, a2893) {
        let a2895 = match v3.clone() {
            Adt14::C0(h2894, _, _, _, _, _, _, _, _, _, _) => {
                h2894
            }
        };
        let a2896 = 0u64;
        nat_eq(a2895, a2896)
    } else {
        false
    } {
        f174(v2.clone(), v3)
    } else {
        false
    } {
        if f175(match v2.clone() {
            Adt14::C0(_, h2897, _, _, _, _, _, _, _, _, _) => {
                h2897
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2898, _, _, _, _, _, _, _, _) => {
                h2898
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2899, _, _, _, _, _, _, _) => {
                h2899
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2900, _, _, _, _, _, _) => {
                h2900
            }
        }) {
            true
        } else {
            let a2902 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2901, _, _, _, _, _) => {
                    h2901
                }
            };
            let a2903 = 4u64;
            if if if if if nat_eq(a2902, a2903) {
                f185(match v2.clone() {
                    Adt14::C0(_, h2904, _, _, _, _, _, _, _, _, _) => {
                        h2904
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2905, _, _, _, _, _, _, _, _) => {
                        h2905
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2906, _, _, _, _, _, _, _) => {
                        h2906
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
                let a2908 = f186(v4.clone())?;
                let a2909 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2907, _) => {
                        h2907
                    }
                };
                nat_le(a2908, a2909)
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
            Adt14::C0(_, h2910, _, _, _, _, _, _, _, _, _) => {
                h2910
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2911, _, _, _, _, _, _, _, _) => {
                h2911
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2912, _, _, _, _, _, _, _) => {
                h2912
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2913, _, _, _, _, _, _) => {
                h2913
            }
        }) {
            true
        } else {
            f180(v4.clone())?
        } {
            0u64
        } else {
            let a2915 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2914, _) => {
                    h2914
                }
            };
            let a2916 = 4u64;
            let a2917 = 0u64;
            nat_quot(a2915, a2916, a2917)
        };
        Ok(Adt15::C1(f187(match v2.clone() {
            Adt14::C0(_, h2918, _, _, _, _, _, _, _, _, _) => {
                h2918
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2919, _, _, _, _, _, _, _, _) => {
                h2919
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2920, _, _, _, _, _, _, _) => {
                h2920
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2921, _, _, _, _, _, _) => {
                h2921
            }
        }, v0, v5)?, 4u64, 65u64, v0, f183(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f97(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f184(v4.clone());
    if if if if if if if v1 {
        let a2923 = match v2.clone() {
            Adt14::C0(h2922, _, _, _, _, _, _, _, _, _, _) => {
                h2922
            }
        };
        let a2924 = 0u64;
        nat_eq(a2923, a2924)
    } else {
        false
    } {
        let a2926 = match v3.clone() {
            Adt14::C0(h2925, _, _, _, _, _, _, _, _, _, _) => {
                h2925
            }
        };
        let a2927 = 0u64;
        nat_eq(a2926, a2927)
    } else {
        false
    } {
        let a2929 = match v4.clone() {
            Adt14::C0(h2928, _, _, _, _, _, _, _, _, _, _) => {
                h2928
            }
        };
        let a2930 = 0u64;
        nat_eq(a2929, a2930)
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
            Adt14::C0(_, h2931, _, _, _, _, _, _, _, _, _) => {
                h2931
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2932, _, _, _, _, _, _, _, _) => {
                h2932
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2933, _, _, _, _, _, _, _) => {
                h2933
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2934, _, _, _, _, _, _) => {
                h2934
            }
        }) {
            true
        } else {
            let a2936 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2935, _, _, _, _, _) => {
                    h2935
                }
            };
            let a2937 = 4u64;
            if if if if if if if nat_eq(a2936, a2937) {
                let a2939 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2938, _, _, _, _, _) => {
                        h2938
                    }
                };
                let a2940 = 4u64;
                nat_eq(a2939, a2940)
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
                let a2942 = f186(v5.clone())?;
                let a2943 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2941, _) => {
                        h2941
                    }
                };
                nat_le(a2942, a2943)
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
            Adt14::C0(_, h2944, _, _, _, _, _, _, _, _, _) => {
                h2944
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2945, _, _, _, _, _, _, _, _) => {
                h2945
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2946, _, _, _, _, _, _, _) => {
                h2946
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2947, _, _, _, _, _, _) => {
                h2947
            }
        }) {
            0u64
        } else {
            let a2953 = match v2.clone() {
                Adt14::C0(_, _, h2948, _, _, _, _, _, _, _, _) => {
                    h2948
                }
            };
            let a2954 = {
                let a2951 = match v2.clone() {
                    Adt14::C0(_, _, _, h2949, _, _, _, _, _, _, _) => {
                        h2949
                    }
                };
                let a2952 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2950, _, _, _, _, _, _) => {
                        h2950
                    }
                };
                nat_mul(a2951, a2952)?
            };
            nat_mul(a2953, a2954)?
        };
        let v7: u64 = {
            let a2955 = v6;
            let a2956 = 0u64;
            if if nat_eq(a2955, a2956) {
                true
            } else {
                f180(v5.clone())?
            } {
                0u64
            } else {
                let a2958 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2957, _) => {
                        h2957
                    }
                };
                let a2959 = 4u64;
                let a2960 = 0u64;
                nat_quot(a2958, a2959, a2960)
            }
        };
        Ok(Adt15::C1(f188(match v2 {
            Adt14::C0(_, h2961, _, _, _, _, _, _, _, _, _) => {
                h2961
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
    let a2962 = v2;
    let a2963 = 0u64;
    if nat_eq(a2962, a2963) {
        Ok(0u64)
    } else {
        let a2966 = 2u64;
        let a2967 = {
            let a2964 = v0;
            let a2965 = v1;
            nat_mul(a2964, a2965)?
        };
        nat_mul(a2966, a2967)
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2968 = v2;
    let a2969 = f170(v0, v1)?;
    nat_mul(a2968, a2969)
}

pub fn f102(v0: u64) -> u64 {
    let a2970 = v0;
    let a2971 = 30u64;
    if nat_eq(a2970, a2971) {
        3u64
    } else {
        1u64
    }
}

pub fn f103(v0: u64) -> R<u64> {
    let a2974 = 5u64;
    let a2975 = {
        let a2972 = 4u64;
        let a2973 = v0;
        nat_mul(a2972, a2973)?
    };
    nat_add(a2974, a2975)
}

pub fn f104(v0: u64) -> bool {
    let a2976 = v0;
    let a2977 = 0u64;
    if if nat_eq(a2976, a2977) {
        true
    } else {
        let a2978 = v0;
        let a2979 = 1u64;
        nat_eq(a2978, a2979)
    } {
        true
    } else {
        let a2980 = v0;
        let a2981 = 30u64;
        nat_eq(a2980, a2981)
    }
}

pub fn f105(v0: Adt2, v1: u64) -> R<bool> {
    let a2983 = match v0.clone() {
        Adt2::C0(_, _, _, h2982, _, _, _, _, _, _, _) => {
            h2982
        }
    };
    let a2984 = 1u64;
    if if if nat_eq(a2983, a2984) {
        let a2986 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2985, _, _, _, _, _, _) => {
                h2985
            }
        };
        let a2987 = 1u64;
        nat_eq(a2986, a2987)
    } else {
        false
    } {
        let a2989 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2988, _, _, _, _, _) => {
                h2988
            }
        };
        let a2990 = v1;
        nat_eq(a2989, a2990)
    } else {
        false
    } {
        let a2992 = match v0.clone() {
            Adt2::C0(_, _, h2991, _, _, _, _, _, _, _, _) => {
                h2991
            }
        };
        let a2993 = 1u64;
        if nat_eq(a2992, a2993) {
            Ok(true)
        } else {
            let a2998 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2994, _, _, _, _) => {
                    h2994
                }
            };
            let a2999 = {
                let a2996 = match v0 {
                    Adt2::C0(_, h2995, _, _, _, _, _, _, _, _, _) => {
                        h2995
                    }
                };
                let a2997 = v1;
                nat_mul(a2996, a2997)?
            };
            Ok(nat_eq(a2998, a2999))
        }
    } else {
        Ok(false)
    }
}

pub fn f106(v0: Adt6) -> R<u64> {
    let a3012 = 2u64;
    let a3013 = {
        let a3010 = 2u64;
        let a3011 = {
            let a3007 = {
                let a3004 = match v0.clone() {
                    Adt6::C0(h3000, _, _, _) => {
                        h3000
                    }
                };
                let a3005 = {
                    let a3002 = match v0.clone() {
                        Adt6::C0(_, h3001, _, _) => {
                            h3001
                        }
                    };
                    let a3003 = 1u64;
                    nat_sub(a3002, a3003)
                };
                nat_add(a3004, a3005)?
            };
            let a3008 = match v0 {
                Adt6::C0(_, h3006, _, _) => {
                    h3006
                }
            };
            let a3009 = 0u64;
            nat_quot(a3007, a3008, a3009)
        };
        nat_mul(a3010, a3011)?
    };
    nat_add(a3012, a3013)
}

pub fn f107(v0: u64) -> R<u64> {
    let a3016 = 2u64;
    let a3017 = {
        let a3014 = v0;
        let a3015 = 2u64;
        nat_mul(a3014, a3015)?
    };
    nat_add(a3016, a3017)
}

pub fn f108(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3021 = {
            let a3018 = v2;
            let a3019 = 1u64;
            nat_sub(a3018, a3019)
        };
        let a3022 = match v0.clone() {
            Adt6::C0(_, h3020, _, _) => {
                h3020
            }
        };
        nat_mul(a3021, a3022)?
    };
    let a3024 = 0u64;
    let a3025 = match v0.clone() {
        Adt6::C0(h3023, _, _, _) => {
            h3023
        }
    };
    if if if if if if if if nat_lt(a3024, a3025) {
        let a3027 = match v0.clone() {
            Adt6::C0(h3026, _, _, _) => {
                h3026
            }
        };
        let a3028 = 2147483648u64;
        nat_lt(a3027, a3028)
    } else {
        false
    } {
        let a3030 = 0u64;
        let a3031 = match v0.clone() {
            Adt6::C0(_, h3029, _, _) => {
                h3029
            }
        };
        nat_lt(a3030, a3031)
    } else {
        false
    } {
        let a3033 = match v0.clone() {
            Adt6::C0(_, h3032, _, _) => {
                h3032
            }
        };
        let a3034 = 2147483648u64;
        nat_lt(a3033, a3034)
    } else {
        false
    } {
        let a3035 = v1;
        let a3036 = 2147483648u64;
        nat_lt(a3035, a3036)
    } else {
        false
    } {
        let a3037 = 0u64;
        let a3038 = v2;
        nat_lt(a3037, a3038)
    } else {
        false
    } {
        let a3039 = v3;
        let a3040 = 2147483648u64;
        nat_lt(a3039, a3040)
    } else {
        false
    } {
        let a3052 = {
            let a3047 = {
                let a3044 = v3;
                let a3045 = {
                    let a3042 = match v0.clone() {
                        Adt6::C0(h3041, _, _, _) => {
                            h3041
                        }
                    };
                    let a3043 = 1u64;
                    nat_sub(a3042, a3043)
                };
                nat_add(a3044, a3045)?
            };
            let a3048 = f132(match v0.clone() {
                Adt6::C0(_, _, h3046, _) => {
                    h3046
                }
            });
            nat_add(a3047, a3048)?
        };
        let a3053 = {
            let a3050 = 2147483647u64;
            let a3051 = f133(match v0.clone() {
                Adt6::C0(_, _, h3049, _) => {
                    h3049
                }
            });
            nat_add(a3050, a3051)?
        };
        nat_le(a3052, a3053)
    } else {
        false
    } {
        let a3055 = match v0 {
            Adt6::C0(_, _, h3054, _) => {
                h3054
            }
        };
        let a3056 = 2147483648u64;
        Ok(!nat_eq(a3055, a3056))
    } else {
        Ok(false)
    }
}

pub fn f109(v0: Adt7) -> R<u64> {
    let a3068 = if match v0.clone() {
        Adt7::C0(_, _, _, h3057, _) => {
            h3057
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3069 = {
        let a3066 = {
            let a3064 = {
                let m3060 = match v0.clone() {
                    Adt7::C0(h3059, _, _, _, _) => {
                        h3059
                    }
                };
                match m3060 {
                    Adt6::C0(h3058, _, _, _) => {
                        h3058
                    }
                }
            };
            let a3065 = {
                let m3063 = match v0.clone() {
                    Adt7::C0(_, h3062, _, _, _) => {
                        h3062
                    }
                };
                match m3063 {
                    Adt6::C0(h3061, _, _, _) => {
                        h3061
                    }
                }
            };
            nat_mul(a3064, a3065)?
        };
        let a3067 = f165(v0)?;
        nat_mul(a3066, a3067)?
    };
    nat_add(a3068, a3069)
}

pub fn f110(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f191(match v0.clone() {
        Adt2::C0(h3070, _, _, _, _, _, _, _, _, _, _) => {
            h3070
        }
    }, match v0.clone() {
        Adt2::C0(_, h3071, _, _, _, _, _, _, _, _, _) => {
            h3071
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3072, _, _, _, _, _, _, _, _) => {
            h3072
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3073, _, _, _, _, _, _, _) => {
            h3073
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3074, _, _, _, _, _, _) => {
            h3074
        }
    }, match v1.clone() {
        Adt2::C0(h3075, _, _, _, _, _, _, _, _, _, _) => {
            h3075
        }
    }, match v1.clone() {
        Adt2::C0(_, h3076, _, _, _, _, _, _, _, _, _) => {
            h3076
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3077, _, _, _, _, _, _, _, _) => {
            h3077
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3078, _, _, _, _, _, _, _) => {
            h3078
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3079, _, _, _, _, _, _) => {
            h3079
        }
    }, match v2.clone() {
        Adt2::C0(h3080, _, _, _, _, _, _, _, _, _, _) => {
            h3080
        }
    }, match v2.clone() {
        Adt2::C0(_, h3081, _, _, _, _, _, _, _, _, _) => {
            h3081
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3082, _, _, _, _, _, _, _, _) => {
            h3082
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3083, _, _, _, _, _, _, _) => {
            h3083
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3084, _, _, _, _, _, _) => {
            h3084
        }
    }) {
        let m3086 = f114(match v0.clone() {
            Adt2::C0(h3085, _, _, _, _, _, _, _, _, _, _) => {
                h3085
            }
        });
        match m3086 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3092 = {
                    let a3089 = match v0.clone() {
                        Adt2::C0(_, h3087, _, _, _, _, _, _, _, _, _) => {
                            h3087
                        }
                    };
                    let a3090 = match v3.clone() {
                        Adt18::C0(_, h3088, _) => {
                            h3088
                        }
                    };
                    let a3091 = 0u64;
                    nat_rem(a3089, a3090, a3091)
                };
                let a3093 = 0u64;
                if if nat_eq(a3092, a3093) {
                    if f192(match v0.clone() {
                        Adt2::C0(_, h3094, _, _, _, _, _, _, _, _, _) => {
                            h3094
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3095, _, _, _, _, _, _, _, _, _) => {
                            h3095
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3096, _, _, _, _, _, _, _, _) => {
                            h3096
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3097, _, _, _, _, _, _, _) => {
                            h3097
                        }
                    }) {
                        true
                    } else {
                        let a3099 = 4u64;
                        let a3100 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3098, _, _, _, _, _) => {
                                h3098
                            }
                        };
                        if if nat_le(a3099, a3100) {
                            let a3102 = match v3.clone() {
                                Adt18::C0(_, h3101, _) => {
                                    h3101
                                }
                            };
                            let a3103 = 1u64;
                            if nat_eq(a3102, a3103) {
                                true
                            } else {
                                let a3106 = match v3.clone() {
                                    Adt18::C0(_, _, h3104) => {
                                        h3104
                                    }
                                };
                                let a3107 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3105, _, _, _, _, _) => {
                                        h3105
                                    }
                                };
                                nat_le(a3106, a3107)
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
                        Adt18::C0(_, h3108, _) => {
                            h3108
                        }
                    });
                    Ok(Adt10::C4(if f192(match v0.clone() {
                        Adt2::C0(_, h3109, _, _, _, _, _, _, _, _, _) => {
                            h3109
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3110, _, _, _, _, _, _, _, _, _) => {
                            h3110
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3111, _, _, _, _, _, _, _, _) => {
                            h3111
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3112, _, _, _, _, _, _, _) => {
                            h3112
                        }
                    }) {
                        0u64
                    } else {
                        f159(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3113, _, _, _, _, _, _, _, _, _, _) => {
                            h3113
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
        Adt2::C0(h3114, _, _, _, _, _, _, _, _, _, _) => {
            h3114
        }
    }) {
        let a3120 = {
            let a3117 = match v0.clone() {
                Adt2::C0(_, h3115, _, _, _, _, _, _, _, _, _) => {
                    h3115
                }
            };
            let a3118 = f193(match v0.clone() {
                Adt2::C0(h3116, _, _, _, _, _, _, _, _, _, _) => {
                    h3116
                }
            });
            let a3119 = 1u64;
            nat_rem(a3117, a3118, a3119)
        };
        let a3121 = 0u64;
        nat_eq(a3120, a3121)
    } else {
        false
    } {
        let a3123 = match v1.clone() {
            Adt2::C0(h3122, _, _, _, _, _, _, _, _, _, _) => {
                h3122
            }
        };
        let a3124 = 26u64;
        nat_eq(a3123, a3124)
    } else {
        false
    } {
        let a3126 = match v2.clone() {
            Adt2::C0(h3125, _, _, _, _, _, _, _, _, _, _) => {
                h3125
            }
        };
        let a3127 = 0u64;
        nat_eq(a3126, a3127)
    } else {
        false
    } {
        let a3130 = match v0.clone() {
            Adt2::C0(_, h3128, _, _, _, _, _, _, _, _, _) => {
                h3128
            }
        };
        let a3131 = match v2.clone() {
            Adt2::C0(_, h3129, _, _, _, _, _, _, _, _, _) => {
                h3129
            }
        };
        nat_eq(a3130, a3131)
    } else {
        false
    } {
        let a3134 = match v1.clone() {
            Adt2::C0(_, h3132, _, _, _, _, _, _, _, _, _) => {
                h3132
            }
        };
        let a3135 = match v0.clone() {
            Adt2::C0(_, _, h3133, _, _, _, _, _, _, _, _) => {
                h3133
            }
        };
        nat_le(a3134, a3135)
    } else {
        false
    } {
        let a3137 = match v0.clone() {
            Adt2::C0(_, _, _, h3136, _, _, _, _, _, _, _) => {
                h3136
            }
        };
        let a3138 = 1u64;
        nat_eq(a3137, a3138)
    } else {
        false
    } {
        let a3140 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3139, _, _, _, _, _, _) => {
                h3139
            }
        };
        let a3141 = 1u64;
        nat_eq(a3140, a3141)
    } else {
        false
    } {
        let a3143 = match v2.clone() {
            Adt2::C0(_, _, _, h3142, _, _, _, _, _, _, _) => {
                h3142
            }
        };
        let a3144 = 1u64;
        nat_eq(a3143, a3144)
    } else {
        false
    } {
        let a3146 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3145, _, _, _, _, _, _) => {
                h3145
            }
        };
        let a3147 = 1u64;
        nat_eq(a3146, a3147)
    } else {
        false
    } {
        let a3149 = match v1.clone() {
            Adt2::C0(_, _, h3148, _, _, _, _, _, _, _, _) => {
                h3148
            }
        };
        let a3150 = 1u64;
        nat_eq(a3149, a3150)
    } else {
        false
    } {
        let a3152 = match v1.clone() {
            Adt2::C0(_, _, _, h3151, _, _, _, _, _, _, _) => {
                h3151
            }
        };
        let a3153 = 1u64;
        nat_eq(a3152, a3153)
    } else {
        false
    } {
        let a3155 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3154, _, _, _, _, _, _) => {
                h3154
            }
        };
        let a3156 = 1u64;
        nat_eq(a3155, a3156)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3157, _, _, _, _, _, _, _, _, _) => {
                h3157
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3158, _, _, _, _, _, _, _, _) => {
                h3158
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3159, _, _, _, _, _, _, _) => {
                h3159
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3160, _, _, _, _, _, _) => {
                h3160
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3162 = match v1.clone() {
                Adt2::C0(_, h3161, _, _, _, _, _, _, _, _, _) => {
                    h3161
                }
            };
            let a3163 = 0u64;
            if nat_eq(a3162, a3163) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3165 = match v0.clone() {
                    Adt2::C0(_, h3164, _, _, _, _, _, _, _, _, _) => {
                        h3164
                    }
                };
                let a3166 = 2147483647u64;
                if if if if if if nat_le(a3165, a3166) {
                    let a3168 = match v1.clone() {
                        Adt2::C0(_, h3167, _, _, _, _, _, _, _, _, _) => {
                            h3167
                        }
                    };
                    let a3169 = 2147483647u64;
                    nat_le(a3168, a3169)
                } else {
                    false
                } {
                    let a3171 = 4u64;
                    let a3172 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3170, _, _, _, _, _) => {
                            h3170
                        }
                    };
                    nat_le(a3171, a3172)
                } else {
                    false
                } {
                    let a3178 = {
                        let a3175 = match v2.clone() {
                            Adt2::C0(_, h3173, _, _, _, _, _, _, _, _, _) => {
                                h3173
                            }
                        };
                        let a3176 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3174, _, _, _, _, _) => {
                                h3174
                            }
                        };
                        nat_mul(a3175, a3176)?
                    };
                    let a3179 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3177, _, _, _, _) => {
                            h3177
                        }
                    };
                    nat_le(a3178, a3179)
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
                        let a3192 = {
                            let a3183 = {
                                let a3181 = 2u64;
                                let a3182 = match v1.clone() {
                                    Adt2::C0(_, h3180, _, _, _, _, _, _, _, _, _) => {
                                        h3180
                                    }
                                };
                                nat_mul(a3181, a3182)?
                            };
                            let a3184 = f16(v2.clone(), 1u64)?;
                            nat_add(a3183, a3184)?
                        };
                        let a3193 = {
                            let a3190 = match v1.clone() {
                                Adt2::C0(_, h3185, _, _, _, _, _, _, _, _, _) => {
                                    h3185
                                }
                            };
                            let a3191 = {
                                let a3188 = 1u64;
                                let a3189 = f157(match v0.clone() {
                                    Adt2::C0(h3186, _, _, _, _, _, _, _, _, _, _) => {
                                        h3186
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3187, _, _, _, _, _, _, _, _, _) => {
                                        h3187
                                    }
                                })?;
                                nat_add(a3188, a3189)?
                            };
                            nat_mul(a3190, a3191)?
                        };
                        nat_add(a3192, a3193)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3194, _, _, _, _, _, _, _, _, _) => {
                            h3194
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3195, _, _, _, _, _, _, _, _, _) => {
                            h3195
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3196, _, _, _, _, _) => {
                            h3196
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3197, _, _, _, _) => {
                            h3197
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3198, _, _, _) => {
                            h3198
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3199, _, _) => {
                            h3199
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3200, _, _, _, _, _, _, _, _, _, _) => {
                            h3200
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
    let a3201 = v0;
    let a3202 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3201, a3202) {
        true
    } else {
        let a3203 = v0;
        let a3204 = 1u64;
        nat_eq(a3203, a3204)
    } {
        true
    } else {
        let a3205 = v0;
        let a3206 = 2u64;
        nat_eq(a3205, a3206)
    } {
        true
    } else {
        let a3207 = v0;
        let a3208 = 3u64;
        nat_eq(a3207, a3208)
    } {
        true
    } else {
        let a3209 = v0;
        let a3210 = 6u64;
        nat_eq(a3209, a3210)
    } {
        true
    } else {
        let a3211 = v0;
        let a3212 = 7u64;
        nat_eq(a3211, a3212)
    } {
        true
    } else {
        let a3213 = v0;
        let a3214 = 8u64;
        nat_eq(a3213, a3214)
    } {
        true
    } else {
        let a3215 = v0;
        let a3216 = 10u64;
        nat_eq(a3215, a3216)
    } {
        true
    } else {
        let a3217 = v0;
        let a3218 = 11u64;
        nat_eq(a3217, a3218)
    } {
        true
    } else {
        let a3219 = v0;
        let a3220 = 12u64;
        nat_eq(a3219, a3220)
    } {
        true
    } else {
        let a3221 = v0;
        let a3222 = 13u64;
        nat_eq(a3221, a3222)
    } {
        true
    } else {
        let a3223 = v0;
        let a3224 = 14u64;
        nat_eq(a3223, a3224)
    } {
        true
    } else {
        let a3225 = v0;
        let a3226 = 16u64;
        nat_eq(a3225, a3226)
    } {
        true
    } else {
        let a3227 = v0;
        let a3228 = 17u64;
        nat_eq(a3227, a3228)
    } {
        true
    } else {
        let a3229 = v0;
        let a3230 = 18u64;
        nat_eq(a3229, a3230)
    } {
        true
    } else {
        let a3231 = v0;
        let a3232 = 19u64;
        nat_eq(a3231, a3232)
    } {
        true
    } else {
        let a3233 = v0;
        let a3234 = 20u64;
        nat_eq(a3233, a3234)
    } {
        true
    } else {
        let a3235 = v0;
        let a3236 = 21u64;
        nat_eq(a3235, a3236)
    } {
        true
    } else {
        let a3237 = v0;
        let a3238 = 22u64;
        nat_eq(a3237, a3238)
    } {
        true
    } else {
        let a3239 = v0;
        let a3240 = 23u64;
        nat_eq(a3239, a3240)
    } {
        true
    } else {
        let a3241 = v0;
        let a3242 = 26u64;
        nat_eq(a3241, a3242)
    } {
        true
    } else {
        let a3243 = v0;
        let a3244 = 29u64;
        nat_eq(a3243, a3244)
    } {
        true
    } else {
        let a3245 = v0;
        let a3246 = 30u64;
        nat_eq(a3245, a3246)
    } {
        true
    } else {
        let a3247 = v0;
        let a3248 = 34u64;
        nat_eq(a3247, a3248)
    } {
        true
    } else {
        let a3249 = v0;
        let a3250 = 35u64;
        nat_eq(a3249, a3250)
    } {
        true
    } else {
        let a3251 = v0;
        let a3252 = 39u64;
        nat_eq(a3251, a3252)
    } {
        true
    } else {
        let a3253 = v0;
        let a3254 = 40u64;
        nat_eq(a3253, a3254)
    } {
        true
    } else {
        let a3255 = v0;
        let a3256 = 41u64;
        nat_eq(a3255, a3256)
    } {
        true
    } else {
        let a3257 = v0;
        let a3258 = 42u64;
        nat_eq(a3257, a3258)
    } {
        true
    } else {
        let a3259 = v0;
        let a3260 = 43u64;
        nat_eq(a3259, a3260)
    }
}

pub fn f113(v0: u64) -> bool {
    if f104(v0) {
        true
    } else {
        let a3261 = v0;
        let a3262 = 43u64;
        nat_eq(a3261, a3262)
    }
}

pub fn f114(v0: u64) -> Adt17 {
    let a3263 = v0;
    let a3264 = 0u64;
    if nat_eq(a3263, a3264) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3265 = v0;
        let a3266 = 1u64;
        if nat_eq(a3265, a3266) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3267 = v0;
            let a3268 = 2u64;
            if nat_eq(a3267, a3268) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3269 = v0;
                let a3270 = 3u64;
                if nat_eq(a3269, a3270) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3271 = v0;
                    let a3272 = 6u64;
                    if nat_eq(a3271, a3272) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3273 = v0;
                        let a3274 = 7u64;
                        if nat_eq(a3273, a3274) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3275 = v0;
                            let a3276 = 8u64;
                            if nat_eq(a3275, a3276) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3277 = v0;
                                let a3278 = 9u64;
                                if nat_eq(a3277, a3278) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3279 = v0;
                                    let a3280 = 10u64;
                                    if nat_eq(a3279, a3280) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3281 = v0;
                                        let a3282 = 11u64;
                                        if nat_eq(a3281, a3282) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3283 = v0;
                                            let a3284 = 12u64;
                                            if nat_eq(a3283, a3284) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3285 = v0;
                                                let a3286 = 13u64;
                                                if nat_eq(a3285, a3286) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3287 = v0;
                                                    let a3288 = 14u64;
                                                    if nat_eq(a3287, a3288) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3289 = v0;
                                                        let a3290 = 15u64;
                                                        if nat_eq(a3289, a3290) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3291 = v0;
                                                            let a3292 = 16u64;
                                                            if nat_eq(a3291, a3292) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3293 = v0;
                                                                let a3294 = 17u64;
                                                                if nat_eq(a3293, a3294) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3295 = v0;
                                                                    let a3296 = 18u64;
                                                                    if nat_eq(a3295, a3296) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3297 = v0;
                                                                        let a3298 = 19u64;
                                                                        if nat_eq(a3297, a3298) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3299 = v0;
                                                                            let a3300 = 20u64;
                                                                            if nat_eq(a3299, a3300) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3301 = v0;
                                                                                let a3302 = 21u64;
                                                                                if nat_eq(a3301, a3302) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3303 = v0;
                                                                                    let a3304 = 22u64;
                                                                                    if nat_eq(a3303, a3304) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3305 = v0;
                                                                                        let a3306 = 23u64;
                                                                                        if nat_eq(a3305, a3306) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3307 = v0;
                                                                                            let a3308 = 24u64;
                                                                                            if nat_eq(a3307, a3308) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3309 = v0;
                                                                                                let a3310 = 25u64;
                                                                                                if nat_eq(a3309, a3310) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3311 = v0;
                                                                                                    let a3312 = 26u64;
                                                                                                    if nat_eq(a3311, a3312) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3313 = v0;
                                                                                                        let a3314 = 27u64;
                                                                                                        if nat_eq(a3313, a3314) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3315 = v0;
                                                                                                            let a3316 = 28u64;
                                                                                                            if nat_eq(a3315, a3316) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3317 = v0;
                                                                                                                let a3318 = 29u64;
                                                                                                                if nat_eq(a3317, a3318) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3319 = v0;
                                                                                                                    let a3320 = 30u64;
                                                                                                                    if nat_eq(a3319, a3320) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3321 = v0;
                                                                                                                        let a3322 = 34u64;
                                                                                                                        if nat_eq(a3321, a3322) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3323 = v0;
                                                                                                                            let a3324 = 35u64;
                                                                                                                            if nat_eq(a3323, a3324) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3325 = v0;
                                                                                                                                let a3326 = 39u64;
                                                                                                                                if nat_eq(a3325, a3326) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3327 = v0;
                                                                                                                                    let a3328 = 40u64;
                                                                                                                                    if nat_eq(a3327, a3328) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3329 = v0;
                                                                                                                                        let a3330 = 41u64;
                                                                                                                                        if nat_eq(a3329, a3330) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3331 = v0;
                                                                                                                                            let a3332 = 42u64;
                                                                                                                                            if nat_eq(a3331, a3332) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3333 = v0;
                                                                                                                                                let a3334 = 43u64;
                                                                                                                                                if nat_eq(a3333, a3334) {
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
    let a3337 = match v1.clone() {
        Adt2::C0(h3335, _, _, _, _, _, _, _, _, _, _) => {
            h3335
        }
    };
    let a3338 = match v3.clone() {
        Adt2::C0(h3336, _, _, _, _, _, _, _, _, _, _) => {
            h3336
        }
    };
    if if nat_eq(a3337, a3338) {
        let a3339 = v0;
        let a3340 = 33u64;
        if nat_eq(a3339, a3340) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3342 = f114(match v1.clone() {
            Adt2::C0(h3341, _, _, _, _, _, _, _, _, _, _) => {
                h3341
            }
        });
        match m3342 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3343, _) => {
                        h3343
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3348 = {
                    let a3345 = match v1.clone() {
                        Adt2::C0(_, h3344, _, _, _, _, _, _, _, _, _) => {
                            h3344
                        }
                    };
                    let a3346 = v5;
                    let a3347 = 1u64;
                    nat_rem(a3345, a3346, a3347)
                };
                let a3349 = 0u64;
                if if if nat_eq(a3348, a3349) {
                    let a3354 = {
                        let a3351 = match v3.clone() {
                            Adt2::C0(_, h3350, _, _, _, _, _, _, _, _, _) => {
                                h3350
                            }
                        };
                        let a3352 = v5;
                        let a3353 = 1u64;
                        nat_rem(a3351, a3352, a3353)
                    };
                    let a3355 = 0u64;
                    nat_eq(a3354, a3355)
                } else {
                    false
                } {
                    let a3356 = v6;
                    let a3357 = f16(v3.clone(), v5)?;
                    nat_eq(a3356, a3357)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3358) => {
                            h3358
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
    let a3359 = v0;
    let a3360 = 0u64;
    if if nat_eq(a3359, a3360) {
        true
    } else {
        let a3361 = v1;
        let a3362 = 0u64;
        nat_eq(a3361, a3362)
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
        let a3366 = {
            let a3363 = v0;
            let a3364 = v4;
            let a3365 = 1u64;
            nat_rem(a3363, a3364, a3365)
        };
        let a3367 = 0u64;
        if if if nat_eq(a3366, a3367) {
            let a3371 = {
                let a3368 = v1;
                let a3369 = v5;
                let a3370 = 1u64;
                nat_rem(a3368, a3369, a3370)
            };
            let a3372 = 0u64;
            nat_eq(a3371, a3372)
        } else {
            false
        } {
            let a3376 = {
                let a3373 = v2;
                let a3374 = v6;
                let a3375 = 1u64;
                nat_rem(a3373, a3374, a3375)
            };
            let a3377 = 0u64;
            nat_eq(a3376, a3377)
        } else {
            false
        } {
            let a3381 = {
                let a3378 = v3;
                let a3379 = v7;
                let a3380 = 1u64;
                nat_rem(a3378, a3379, a3380)
            };
            let a3382 = 0u64;
            nat_eq(a3381, a3382)
        } else {
            false
        }
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3383 = v0;
    let a3384 = v1;
    if nat_eq(a3383, a3384) {
        let a3387 = v4;
        let a3388 = {
            let a3385 = v2;
            let a3386 = v3;
            nat_add(a3385, a3386)?
        };
        Ok(nat_eq(a3387, a3388))
    } else {
        let a3389 = v2;
        let a3390 = v3;
        if nat_eq(a3389, a3390) {
            let a3391 = v4;
            let a3392 = v2;
            Ok(nat_eq(a3391, a3392))
        } else {
            Ok(false)
        }
    }
}

pub fn f119(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3395 = {
            let a3393 = v0;
            let a3394 = f133(v1);
            nat_add(a3393, a3394)?
        };
        let a3396 = f133(v2);
        nat_add(a3395, a3396)?
    };
    let v5: u64 = {
        let a3397 = f132(v1);
        let a3398 = f132(v2);
        nat_add(a3397, a3398)?
    };
    let a3399 = v1;
    let a3400 = 4294967295u64;
    if if nat_le(a3399, a3400) {
        let a3401 = v2;
        let a3402 = 4294967295u64;
        nat_le(a3401, a3402)
    } else {
        false
    } {
        let a3403 = v5;
        let a3404 = v4;
        if nat_le(a3403, a3404) {
            let a3407 = v3;
            let a3408 = {
                let a3405 = v4;
                let a3406 = v5;
                nat_sub(a3405, a3406)
            };
            Ok(nat_eq(a3407, a3408))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f120(v0: u64, v1: u64) -> R<bool> {
    let a3409 = v1;
    let a3410 = 4294967295u64;
    if nat_le(a3409, a3410) {
        let a3413 = {
            let a3411 = f133(v1);
            let a3412 = f132(v1);
            nat_add(a3411, a3412)?
        };
        let a3414 = v0;
        Ok(nat_lt(a3413, a3414))
    } else {
        Ok(false)
    }
}

pub fn f121(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f194(v0.clone(), false)?;
    let v6: u64 = f194(v0, true)?;
    let a3415 = v6;
    let a3416 = v1;
    if if nat_le(a3415, a3416) {
        let a3423 = {
            let a3421 = {
                let a3417 = v1;
                let a3418 = v5;
                nat_add(a3417, a3418)?
            };
            let a3422 = {
                let a3419 = v3;
                let a3420 = 0u64;
                if nat_eq(a3419, a3420) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3421, a3422)?
        };
        let a3424 = v2;
        nat_le(a3423, a3424)
    } else {
        false
    } {
        if v4 {
            let a3429 = {
                let a3427 = {
                    let a3425 = v1;
                    let a3426 = v5;
                    nat_add(a3425, a3426)?
                };
                let a3428 = v6;
                nat_sub(a3427, a3428)
            };
            let a3430 = v2;
            Ok(nat_lt(a3429, a3430))
        } else {
            let a3435 = {
                let a3433 = {
                    let a3431 = v1;
                    let a3432 = v5;
                    nat_add(a3431, a3432)?
                };
                let a3434 = v6;
                nat_sub(a3433, a3434)
            };
            let a3436 = v2;
            Ok(nat_le(a3435, a3436))
        }
    } else {
        Ok(false)
    }
}

pub fn f122(v0: u64, v1: u64) -> bool {
    let a3437 = v0;
    let a3438 = 0u64;
    if if nat_eq(a3437, a3438) {
        true
    } else {
        let a3439 = v0;
        let a3440 = 1u64;
        nat_eq(a3439, a3440)
    } {
        let a3441 = v1;
        let a3442 = 4294967296u64;
        nat_lt(a3441, a3442)
    } else {
        false
    }
}

pub fn f123(v0: u64, v1: u64) -> R<u64> {
    let a3447 = {
        let a3445 = v0;
        let a3446 = {
            let a3443 = v1;
            let a3444 = 1u64;
            nat_sub(a3443, a3444)
        };
        nat_add(a3445, a3446)?
    };
    let a3448 = v1;
    let a3449 = 0u64;
    Ok(nat_quot(a3447, a3448, a3449))
}

pub fn f124(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h3450, _, _, _, _, _, _, _, _, _) => {
            h3450
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3451, _, _, _, _, _, _, _, _) => {
            h3451
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3452, _, _, _, _, _, _, _) => {
            h3452
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3453, _, _, _, _, _, _) => {
            h3453
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3456 = match v1.clone() {
                Adt2::C0(_, h3454, _, _, _, _, _, _, _, _, _) => {
                    h3454
                }
            };
            let a3457 = match v1.clone() {
                Adt2::C0(_, _, h3455, _, _, _, _, _, _, _, _) => {
                    h3455
                }
            };
            nat_mul(a3456, a3457)?
        };
        let v5: u64 = {
            let a3459 = v4;
            let a3460 = match v1.clone() {
                Adt2::C0(_, _, _, h3458, _, _, _, _, _, _, _) => {
                    h3458
                }
            };
            nat_mul(a3459, a3460)?
        };
        let a3473 = {
            let a3471 = {
                let a3462 = match v0.clone() {
                    Adt2::C0(_, h3461, _, _, _, _, _, _, _, _, _) => {
                        h3461
                    }
                };
                let a3463 = 1u64;
                nat_sub(a3462, a3463)
            };
            let a3472 = {
                let a3469 = f195(match v0.clone() {
                    Adt2::C0(_, _, h3464, _, _, _, _, _, _, _, _) => {
                        h3464
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3465, _, _, _, _, _, _, _, _, _) => {
                        h3465
                    }
                })?;
                let a3470 = f195(match v0 {
                    Adt2::C0(_, _, _, h3466, _, _, _, _, _, _, _) => {
                        h3466
                    }
                }, v2, {
                    let a3467 = v3;
                    let a3468 = v5;
                    nat_mul(a3467, a3468)?
                }, v4)?;
                nat_add(a3469, a3470)?
            };
            nat_add(a3471, a3472)?
        };
        let a3474 = f16(v1, 1u64)?;
        Ok(nat_lt(a3473, a3474))
    }
}

pub fn f125(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3475 = v0;
    let a3476 = 2147483648u64;
    if if if if nat_lt(a3475, a3476) {
        let a3477 = v1;
        let a3478 = 2147483648u64;
        nat_lt(a3477, a3478)
    } else {
        false
    } {
        let a3481 = {
            let a3479 = v0;
            let a3480 = v1;
            nat_mul(a3479, a3480)?
        };
        let a3482 = 2147483648u64;
        nat_lt(a3481, a3482)
    } else {
        false
    } {
        let a3487 = {
            let a3485 = {
                let a3483 = v0;
                let a3484 = 1u64;
                nat_sub(a3483, a3484)
            };
            let a3486 = v2;
            nat_mul(a3485, a3486)?
        };
        let a3488 = 2147483648u64;
        nat_lt(a3487, a3488)
    } else {
        false
    } {
        let a3493 = {
            let a3491 = {
                let a3489 = v1;
                let a3490 = 1u64;
                nat_sub(a3489, a3490)
            };
            let a3492 = v2;
            nat_mul(a3491, a3492)?
        };
        let a3494 = 2147483648u64;
        Ok(nat_lt(a3493, a3494))
    } else {
        Ok(false)
    }
}

pub fn f126(v0: u64) -> R<u64> {
    let a3497 = 3u64;
    let a3498 = {
        let a3495 = v0;
        let a3496 = 3u64;
        nat_mul(a3495, a3496)?
    };
    nat_add(a3497, a3498)
}

pub fn f127(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h3499, _, _, _, _, _, _, _) => {
            h3499
        }
    }, match v0.clone() {
        Adt3::C0(_, h3500, _, _, _, _, _, _) => {
            h3500
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3501, _, _, _, _, _) => {
            h3501
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3502, _, _, _, _) => {
            h3502
        }
    }) {
        Ok(0u64)
    } else {
        let a3511 = match v0.clone() {
            Adt3::C0(h3503, _, _, _, _, _, _, _) => {
                h3503
            }
        };
        let a3512 = {
            let a3509 = match v0.clone() {
                Adt3::C0(_, h3504, _, _, _, _, _, _) => {
                    h3504
                }
            };
            let a3510 = {
                let a3507 = match v0.clone() {
                    Adt3::C0(_, _, h3505, _, _, _, _, _) => {
                        h3505
                    }
                };
                let a3508 = match v0 {
                    Adt3::C0(_, _, _, h3506, _, _, _, _) => {
                        h3506
                    }
                };
                nat_mul(a3507, a3508)?
            };
            nat_mul(a3509, a3510)?
        };
        nat_mul(a3511, a3512)
    }
}

pub fn f128(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3513 = v1;
    let a3514 = 13u64;
    if if nat_eq(a3513, a3514) {
        f34(match v0.clone() {
            Adt3::C0(h3515, _, _, _, _, _, _, _) => {
                h3515
            }
        }, match v0.clone() {
            Adt3::C0(_, h3516, _, _, _, _, _, _) => {
                h3516
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3517, _, _, _, _, _) => {
                h3517
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3518, _, _, _, _) => {
                h3518
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3524 = match v0.clone() {
                Adt3::C0(_, h3519, _, _, _, _, _, _) => {
                    h3519
                }
            };
            let a3525 = {
                let a3522 = match v0.clone() {
                    Adt3::C0(_, _, h3520, _, _, _, _, _) => {
                        h3520
                    }
                };
                let a3523 = match v0.clone() {
                    Adt3::C0(_, _, _, h3521, _, _, _, _) => {
                        h3521
                    }
                };
                nat_mul(a3522, a3523)?
            };
            nat_mul(a3524, a3525)?
        };
        let a3526 = v1;
        let a3527 = 13u64;
        if nat_eq(a3526, a3527) {
            let a3536 = 2u64;
            let a3537 = if f196(v0.clone(), v2) {
                let a3529 = v3;
                let a3530 = f197(match v0 {
                    Adt3::C0(h3528, _, _, _, _, _, _, _) => {
                        h3528
                    }
                }, false)?;
                nat_mul(a3529, a3530)?
            } else {
                let a3534 = v3;
                let a3535 = {
                    let a3532 = match v0 {
                        Adt3::C0(h3531, _, _, _, _, _, _, _) => {
                            h3531
                        }
                    };
                    let a3533 = 2u64;
                    nat_mul(a3532, a3533)?
                };
                nat_mul(a3534, a3535)?
            };
            nat_add(a3536, a3537)
        } else {
            let a3541 = v3;
            let a3542 = f197(match v0 {
                Adt3::C0(h3538, _, _, _, _, _, _, _) => {
                    h3538
                }
            }, {
                let a3539 = v1;
                let a3540 = 16u64;
                nat_eq(a3539, a3540)
            })?;
            nat_mul(a3541, a3542)
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3543 = v0;
    let a3544 = 30u64;
    if if nat_eq(a3543, a3544) {
        let a3547 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3545, _, _, _, _) => {
                h3545
            }
        };
        let a3548 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3546, _, _, _, _, _) => {
                h3546
            }
        };
        nat_lt(a3547, a3548)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3549 = v3;
        match m3549 {
            None => {
                f198(v0, v1, match v2.clone() {
                    Adt2::C0(h3550, _, _, _, _, _, _, _, _, _, _) => {
                        h3550
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3551, _, _, _, _, _, _, _, _, _, _) => {
                        h3551
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3552, _, _, _, _, _, _, _, _, _) => {
                        h3552
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3553, _, _, _, _, _, _, _, _) => {
                        h3553
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3554, _, _, _, _, _, _, _) => {
                        h3554
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3555, _, _, _, _, _, _) => {
                        h3555
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3556, _, _, _, _, _, _, _, _, _) => {
                        h3556
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3557, _, _, _, _, _, _, _, _) => {
                        h3557
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3558, _, _, _, _, _, _, _) => {
                        h3558
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3559, _, _, _, _, _, _) => {
                        h3559
                    }
                })
            }
            Some(v5) => {
                f198(v0, v1, match v2.clone() {
                    Adt2::C0(h3560, _, _, _, _, _, _, _, _, _, _) => {
                        h3560
                    }
                }, match v5.clone() {
                    Adt2::C0(h3561, _, _, _, _, _, _, _, _, _, _) => {
                        h3561
                    }
                }, match v4.clone() {
                    Adt2::C0(h3562, _, _, _, _, _, _, _, _, _, _) => {
                        h3562
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3563, _, _, _, _, _, _, _, _, _) => {
                        h3563
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3564, _, _, _, _, _, _, _, _) => {
                        h3564
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3565, _, _, _, _, _, _, _) => {
                        h3565
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3566, _, _, _, _, _, _) => {
                        h3566
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3567, _, _, _, _, _, _, _, _, _) => {
                        h3567
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3568, _, _, _, _, _, _, _, _) => {
                        h3568
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3569, _, _, _, _, _, _, _) => {
                        h3569
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3570, _, _, _, _, _, _) => {
                        h3570
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3571, _, _, _, _, _, _, _, _, _) => {
                        h3571
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3572, _, _, _, _, _, _, _, _) => {
                        h3572
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3573, _, _, _, _, _, _, _) => {
                        h3573
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3574, _, _, _, _, _, _) => {
                        h3574
                    }
                })
            }
        }
    }
}

pub fn f130(v0: u64, v1: u64) -> u64 {
    let a3575 = v0;
    let a3576 = 23u64;
    if nat_eq(a3575, a3576) {
        let a3577 = v1;
        let a3578 = 0u64;
        if nat_eq(a3577, a3578) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3591 = {
        let a3587 = {
            let a3583 = {
                let a3579 = v0;
                let a3580 = v4;
                nat_mul(a3579, a3580)?
            };
            let a3584 = {
                let a3581 = v1;
                let a3582 = v5;
                nat_mul(a3581, a3582)?
            };
            nat_add(a3583, a3584)?
        };
        let a3588 = {
            let a3585 = v2;
            let a3586 = v6;
            nat_mul(a3585, a3586)?
        };
        nat_add(a3587, a3588)?
    };
    let a3592 = {
        let a3589 = v3;
        let a3590 = v7;
        nat_mul(a3589, a3590)?
    };
    nat_add(a3591, a3592)
}

pub fn f132(v0: u64) -> u64 {
    let a3593 = v0;
    let a3594 = 2147483648u64;
    if nat_lt(a3593, a3594) {
        0u64
    } else {
        let a3595 = 4294967296u64;
        let a3596 = v0;
        nat_sub(a3595, a3596)
    }
}

pub fn f133(v0: u64) -> u64 {
    let a3597 = v0;
    let a3598 = 2147483648u64;
    if nat_lt(a3597, a3598) {
        v0
    } else {
        0u64
    }
}

pub fn f134(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3599 = v7;
        let a3600 = 6u64;
        if nat_eq(a3599, a3600) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f90(v3, v4), f90(v5, v6), f91(v7, v8), f199(v1), f199(v9))
}

pub fn f135(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3601 = v0;
    match m3601 {
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
            let a3602 = 13u64;
            let a3603 = v36;
            if nat_le(a3602, a3603) {
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
                Adt5::C0(_, _, _, _, _, h3604, _, _) => {
                    h3604
                }
            }, match v56 {
                Adt5::C0(_, _, _, _, _, _, _, h3605) => {
                    h3605
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
    let m3606 = v0;
    match m3606 {
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
    let m3607 = v0;
    match m3607 {
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
            let a3608 = v1;
            let a3609 = 6u64;
            if nat_eq(a3608, a3609) {
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
            let a3618 = v1;
            let a3619 = 6u64;
            if if if nat_eq(a3618, a3619) {
                true
            } else {
                let a3620 = v1;
                let a3621 = 7u64;
                nat_eq(a3620, a3621)
            } {
                let a3622 = v56;
                let a3623 = 30u64;
                nat_eq(a3622, a3623)
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
                Adt5::C0(_, _, _, _, _, _, _, h3617) => {
                    h3617
                }
            }
        }
        Adt4::C28(v51) => {
            let a3610 = v1;
            let a3611 = 6u64;
            if nat_eq(a3610, a3611) {
                match v51 {
                    Adt7::C0(_, _, _, _, h3612) => {
                        h3612
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
            let a3613 = v1;
            let a3614 = 6u64;
            if if nat_eq(a3613, a3614) {
                true
            } else {
                let a3615 = v1;
                let a3616 = 7u64;
                nat_eq(a3615, a3616)
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
    let a3624 = v1;
    let a3625 = 4u64;
    if nat_lt(a3624, a3625) {
        Adt11::C0(v1)
    } else {
        let a3626 = v1;
        let a3627 = 4u64;
        if nat_eq(a3626, a3627) {
            Adt11::C1(0u64)
        } else {
            let a3628 = v1;
            let a3629 = 5u64;
            if nat_eq(a3628, a3629) {
                Adt11::C1(3u64)
            } else {
                let a3630 = v1;
                let a3631 = 6u64;
                if nat_eq(a3630, a3631) {
                    Adt11::C2(f137(v0, v1), 0u64, false)
                } else {
                    let a3632 = v1;
                    let a3633 = 7u64;
                    if nat_eq(a3632, a3633) {
                        Adt11::C2(0u64, f137(v0, v1), false)
                    } else {
                        let a3634 = v1;
                        let a3635 = 16u64;
                        if nat_lt(a3634, a3635) {
                            Adt11::C1({
                                let a3636 = v1;
                                let a3637 = 8u64;
                                nat_sub(a3636, a3637)
                            })
                        } else {
                            let a3638 = v1;
                            let a3639 = 20u64;
                            if nat_lt(a3638, a3639) {
                                Adt11::C5({
                                    let a3640 = v1;
                                    let a3641 = 16u64;
                                    nat_sub(a3640, a3641)
                                })
                            } else {
                                let a3642 = v1;
                                let a3643 = 20u64;
                                if nat_eq(a3642, a3643) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3644 = v1;
                                    let a3645 = 21u64;
                                    if nat_eq(a3644, a3645) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3646 = v1;
                                        let a3647 = 22u64;
                                        if nat_eq(a3646, a3647) {
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
        let a3649 = v0;
        let a3650 = match v1.clone() {
            Adt3::C0(h3648, _, _, _, _, _, _, _) => {
                h3648
            }
        };
        let a3651 = 0u64;
        nat_quot(a3649, a3650, a3651)
    };
    let v3: u64 = {
        let a3653 = v2;
        let a3654 = match v1.clone() {
            Adt3::C0(_, h3652, _, _, _, _, _, _) => {
                h3652
            }
        };
        let a3655 = 0u64;
        nat_quot(a3653, a3654, a3655)
    };
    f131({
        let a3657 = v0;
        let a3658 = match v1.clone() {
            Adt3::C0(h3656, _, _, _, _, _, _, _) => {
                h3656
            }
        };
        let a3659 = 0u64;
        nat_rem(a3657, a3658, a3659)
    }, {
        let a3661 = v2;
        let a3662 = match v1.clone() {
            Adt3::C0(_, h3660, _, _, _, _, _, _) => {
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
        nat_rem(a3665, a3666, a3667)
    }, {
        let a3669 = v3;
        let a3670 = match v1.clone() {
            Adt3::C0(_, _, h3668, _, _, _, _, _) => {
                h3668
            }
        };
        let a3671 = 0u64;
        nat_quot(a3669, a3670, a3671)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3672, _, _, _) => {
            h3672
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3673, _, _) => {
            h3673
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3674, _) => {
            h3674
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3675) => {
            h3675
        }
    })
}

pub fn f140(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3693 = {
        let a3688 = f200(v2, {
            let a3679 = {
                let a3676 = v0;
                let a3677 = v1;
                let a3678 = 0u64;
                nat_quot(a3676, a3677, a3678)
            };
            let a3680 = v2;
            let a3681 = 0u64;
            nat_rem(a3679, a3680, a3681)
        }, {
            let a3685 = {
                let a3682 = v0;
                let a3683 = v1;
                let a3684 = 0u64;
                nat_quot(a3682, a3683, a3684)
            };
            let a3686 = v2;
            let a3687 = 0u64;
            nat_quot(a3685, a3686, a3687)
        })?;
        let a3689 = v1;
        nat_mul(a3688, a3689)?
    };
    let a3694 = {
        let a3690 = v0;
        let a3691 = v1;
        let a3692 = 0u64;
        nat_rem(a3690, a3691, a3692)
    };
    nat_add(a3693, a3694)
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3696 = v0;
        let a3697 = match v2.clone() {
            Adt3::C0(h3695, _, _, _, _, _, _, _) => {
                h3695
            }
        };
        let a3698 = 0u64;
        nat_quot(a3696, a3697, a3698)
    };
    let v4: u64 = {
        let a3700 = v3;
        let a3701 = match v2.clone() {
            Adt3::C0(_, h3699, _, _, _, _, _, _) => {
                h3699
            }
        };
        let a3702 = 0u64;
        nat_quot(a3700, a3701, a3702)
    };
    f201({
        let a3704 = v0;
        let a3705 = match v2.clone() {
            Adt3::C0(h3703, _, _, _, _, _, _, _) => {
                h3703
            }
        };
        let a3706 = 0u64;
        nat_rem(a3704, a3705, a3706)
    }, {
        let a3708 = v3;
        let a3709 = match v2.clone() {
            Adt3::C0(_, h3707, _, _, _, _, _, _) => {
                h3707
            }
        };
        let a3710 = 0u64;
        nat_rem(a3708, a3709, a3710)
    }, {
        let a3712 = v4;
        let a3713 = match v2.clone() {
            Adt3::C0(_, _, h3711, _, _, _, _, _) => {
                h3711
            }
        };
        let a3714 = 0u64;
        nat_rem(a3712, a3713, a3714)
    }, {
        let a3716 = v4;
        let a3717 = match v2 {
            Adt3::C0(_, _, h3715, _, _, _, _, _) => {
                h3715
            }
        };
        let a3718 = 0u64;
        nat_quot(a3716, a3717, a3718)
    }, match v1.clone() {
        Adt3::C0(h3719, _, _, _, _, _, _, _) => {
            h3719
        }
    }, match v1.clone() {
        Adt3::C0(_, h3720, _, _, _, _, _, _) => {
            h3720
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3721, _, _, _, _, _) => {
            h3721
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3722, _, _, _, _) => {
            h3722
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3723, _, _, _) => {
            h3723
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3724, _, _) => {
            h3724
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3725, _) => {
            h3725
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3726) => {
            h3726
        }
    })
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3728 = v0;
        let a3729 = match v3.clone() {
            Adt3::C0(h3727, _, _, _, _, _, _, _) => {
                h3727
            }
        };
        let a3730 = 0u64;
        nat_quot(a3728, a3729, a3730)
    };
    let v6: u64 = {
        let a3732 = v5;
        let a3733 = match v3.clone() {
            Adt3::C0(_, h3731, _, _, _, _, _, _) => {
                h3731
            }
        };
        let a3734 = 0u64;
        nat_quot(a3732, a3733, a3734)
    };
    let v7: u64 = {
        let a3736 = v0;
        let a3737 = match v3.clone() {
            Adt3::C0(h3735, _, _, _, _, _, _, _) => {
                h3735
            }
        };
        let a3738 = 0u64;
        nat_rem(a3736, a3737, a3738)
    };
    let v8: u64 = {
        let a3740 = v5;
        let a3741 = match v3.clone() {
            Adt3::C0(_, h3739, _, _, _, _, _, _) => {
                h3739
            }
        };
        let a3742 = 0u64;
        nat_rem(a3740, a3741, a3742)
    };
    let v9: u64 = {
        let a3744 = v6;
        let a3745 = match v3.clone() {
            Adt3::C0(_, _, h3743, _, _, _, _, _) => {
                h3743
            }
        };
        let a3746 = 0u64;
        nat_rem(a3744, a3745, a3746)
    };
    let v10: u64 = {
        let a3748 = v6;
        let a3749 = match v3.clone() {
            Adt3::C0(_, _, h3747, _, _, _, _, _) => {
                h3747
            }
        };
        let a3750 = 0u64;
        nat_quot(a3748, a3749, a3750)
    };
    let v11: u64 = {
        let a3751 = v4;
        let a3752 = 0u64;
        if nat_eq(a3751, a3752) {
            match v1.clone() {
                Adt3::C0(h3753, _, _, _, _, _, _, _) => {
                    h3753
                }
            }
        } else {
            let a3754 = v4;
            let a3755 = 1u64;
            if nat_eq(a3754, a3755) {
                match v1.clone() {
                    Adt3::C0(_, h3756, _, _, _, _, _, _) => {
                        h3756
                    }
                }
            } else {
                let a3757 = v4;
                let a3758 = 2u64;
                if nat_eq(a3757, a3758) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3759, _, _, _, _, _) => {
                            h3759
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3760, _, _, _, _) => {
                            h3760
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3767 = v11;
        let a3768 = {
            let a3761 = v4;
            let a3762 = 0u64;
            if nat_eq(a3761, a3762) {
                v7
            } else {
                let a3763 = v4;
                let a3764 = 1u64;
                if nat_eq(a3763, a3764) {
                    v8
                } else {
                    let a3765 = v4;
                    let a3766 = 2u64;
                    if nat_eq(a3765, a3766) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3767, a3768)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f131(if if v12 {
        let a3769 = v4;
        let a3770 = 0u64;
        nat_eq(a3769, a3770)
    } else {
        false
    } {
        let a3771 = v7;
        let a3772 = v11;
        nat_sub(a3771, a3772)
    } else {
        v7
    }, if if v12 {
        let a3773 = v4;
        let a3774 = 1u64;
        nat_eq(a3773, a3774)
    } else {
        false
    } {
        let a3775 = v8;
        let a3776 = v11;
        nat_sub(a3775, a3776)
    } else {
        v8
    }, if if v12 {
        let a3777 = v4;
        let a3778 = 2u64;
        nat_eq(a3777, a3778)
    } else {
        false
    } {
        let a3779 = v9;
        let a3780 = v11;
        nat_sub(a3779, a3780)
    } else {
        v9
    }, if if v12 {
        let a3781 = v4;
        let a3782 = 3u64;
        nat_eq(a3781, a3782)
    } else {
        false
    } {
        let a3783 = v10;
        let a3784 = v11;
        nat_sub(a3783, a3784)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3785, _, _, _) => {
            h3785
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3786, _, _) => {
            h3786
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3787, _) => {
            h3787
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3788) => {
            h3788
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
            Adt3::C0(h3789, _, _, _, _, _, _, _) => {
                h3789
            }
        })?, f202(v9, v4, match v1.clone() {
            Adt3::C0(_, h3790, _, _, _, _, _, _) => {
                h3790
            }
        })?, f202(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3791, _, _, _, _, _) => {
                h3791
            }
        })?, f202(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3792, _, _, _, _) => {
                h3792
            }
        })?)?, f139(v0, v2)?))
    } else {
        if if if if f203(v8, v3, match v1.clone() {
            Adt3::C0(h3793, _, _, _, _, _, _, _) => {
                h3793
            }
        })? {
            f203(v9, v4, match v1.clone() {
                Adt3::C0(_, h3794, _, _, _, _, _, _) => {
                    h3794
                }
            })?
        } else {
            false
        } {
            f203(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3795, _, _, _, _, _) => {
                    h3795
                }
            })?
        } else {
            false
        } {
            f203(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3796, _, _, _, _) => {
                    h3796
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
        Adt3::C0(h3797, _, _, _, _, _, _, _) => {
            h3797
        }
    })?, v5, v6, v7)?, f139(v0, v2)?))
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3798 = v0;
    let a3799 = v3;
    if nat_lt(a3798, a3799) {
        Ok(Adt9::C1(false, {
            let a3800 = v0;
            let a3801 = 4u64;
            nat_mul(a3800, a3801)?
        }, {
            let a3802 = v0;
            let a3803 = 4u64;
            nat_mul(a3802, a3803)?
        }))
    } else {
        let v6: u64 = {
            let a3804 = v0;
            let a3805 = v3;
            nat_sub(a3804, a3805)
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
    let a3806 = v0;
    let a3807 = v3;
    if nat_lt(a3806, a3807) {
        Ok(Adt9::C1(true, 0u64, f139(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3808 = v0;
            let a3809 = v3;
            nat_sub(a3808, a3809)
        };
        let v5: u64 = f141(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f139(v4, v1)?, 3u64, v5))
    }
}

pub fn f147(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3811 = v1;
        let a3812 = match v3.clone() {
            Adt3::C0(h3810, _, _, _, _, _, _, _) => {
                h3810
            }
        };
        let a3813 = 0u64;
        nat_rem(a3811, a3812, a3813)
    };
    let v5: u64 = {
        let a3819 = {
            let a3815 = v1;
            let a3816 = match v3.clone() {
                Adt3::C0(h3814, _, _, _, _, _, _, _) => {
                    h3814
                }
            };
            let a3817 = 0u64;
            nat_quot(a3815, a3816, a3817)
        };
        let a3820 = match v3.clone() {
            Adt3::C0(_, h3818, _, _, _, _, _, _) => {
                h3818
            }
        };
        let a3821 = 0u64;
        nat_rem(a3819, a3820, a3821)
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
        let a3822 = v3;
        let a3823 = 8u64;
        nat_add(a3822, a3823)?
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
    let a3824 = v4;
    let a3825 = 0u64;
    if if nat_eq(a3824, a3825) {
        f50(v3)
    } else {
        false
    } {
        let m3826 = f114(v3);
        match m3826 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h3827, _) => {
                        h3827
                    }
                };
                Ok(Adt9::C5(v3, f139(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h3828) => {
                        h3828
                    }
                }, f139({
                    let a3829 = v0;
                    let a3830 = v6;
                    nat_mul(a3829, a3830)?
                }, v2.clone())?, {
                    let a3836 = {
                        let a3834 = {
                            let a3831 = v6;
                            let a3832 = 1u64;
                            nat_sub(a3831, a3832)
                        };
                        let a3835 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3833, _, _, _) => {
                                h3833
                            }
                        };
                        nat_mul(a3834, a3835)?
                    };
                    let a3837 = 4u64;
                    nat_add(a3836, a3837)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3838, _, _, _) => {
                        h3838
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
        let a3840 = v0;
        let a3841 = match v2.clone() {
            Adt3::C0(h3839, _, _, _, _, _, _, _) => {
                h3839
            }
        };
        let a3842 = 0u64;
        nat_rem(a3840, a3841, a3842)
    };
    let v7: u64 = {
        let a3844 = v0;
        let a3845 = match v2.clone() {
            Adt3::C0(h3843, _, _, _, _, _, _, _) => {
                h3843
            }
        };
        let a3846 = 0u64;
        nat_quot(a3844, a3845, a3846)
    };
    let v8: u64 = {
        let a3848 = v7;
        let a3849 = match v2.clone() {
            Adt3::C0(_, h3847, _, _, _, _, _, _) => {
                h3847
            }
        };
        let a3850 = 0u64;
        nat_rem(a3848, a3849, a3850)
    };
    let v9: u64 = {
        let a3852 = v7;
        let a3853 = match v2.clone() {
            Adt3::C0(_, h3851, _, _, _, _, _, _) => {
                h3851
            }
        };
        let a3854 = 0u64;
        nat_quot(a3852, a3853, a3854)
    };
    let v10: u64 = {
        let a3856 = v9;
        let a3857 = match v2.clone() {
            Adt3::C0(_, _, h3855, _, _, _, _, _) => {
                h3855
            }
        };
        let a3858 = 0u64;
        nat_rem(a3856, a3857, a3858)
    };
    let v11: u64 = {
        let a3860 = v9;
        let a3861 = match v2.clone() {
            Adt3::C0(_, _, h3859, _, _, _, _, _) => {
                h3859
            }
        };
        let a3862 = 0u64;
        nat_quot(a3860, a3861, a3862)
    };
    let v12: u64 = f139(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3871 = {
                let a3866 = {
                    let a3863 = v10;
                    let a3864 = v4;
                    let a3865 = 0u64;
                    nat_quot(a3863, a3864, a3865)
                };
                let a3867 = v5;
                nat_mul(a3866, a3867)?
            };
            let a3872 = {
                let a3868 = v8;
                let a3869 = v4;
                let a3870 = 0u64;
                nat_quot(a3868, a3869, a3870)
            };
            nat_add(a3871, a3872)?
        };
        let v14: u64 = {
            let a3892 = v6;
            let a3893 = {
                let a3890 = match v1.clone() {
                    Adt3::C0(h3873, _, _, _, _, _, _, _) => {
                        h3873
                    }
                };
                let a3891 = {
                    let a3888 = {
                        let a3874 = v8;
                        let a3875 = v4;
                        let a3876 = 0u64;
                        nat_rem(a3874, a3875, a3876)
                    };
                    let a3889 = {
                        let a3886 = match v1.clone() {
                            Adt3::C0(_, h3877, _, _, _, _, _, _) => {
                                h3877
                            }
                        };
                        let a3887 = {
                            let a3884 = {
                                let a3878 = v10;
                                let a3879 = v4;
                                let a3880 = 0u64;
                                nat_rem(a3878, a3879, a3880)
                            };
                            let a3885 = {
                                let a3882 = match v1.clone() {
                                    Adt3::C0(_, _, h3881, _, _, _, _, _) => {
                                        h3881
                                    }
                                };
                                let a3883 = v13;
                                nat_mul(a3882, a3883)?
                            };
                            nat_add(a3884, a3885)?
                        };
                        nat_mul(a3886, a3887)?
                    };
                    nat_add(a3888, a3889)?
                };
                nat_mul(a3890, a3891)?
            };
            nat_add(a3892, a3893)?
        };
        Ok(Adt9::C1(false, f139(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3899 = {
                let a3897 = {
                    let a3894 = v11;
                    let a3895 = v5;
                    let a3896 = 0u64;
                    nat_rem(a3894, a3895, a3896)
                };
                let a3898 = v4;
                nat_mul(a3897, a3898)?
            };
            let a3900 = v8;
            nat_add(a3899, a3900)?
        };
        let v16: u64 = {
            let a3906 = {
                let a3904 = {
                    let a3901 = v11;
                    let a3902 = v5;
                    let a3903 = 0u64;
                    nat_quot(a3901, a3902, a3903)
                };
                let a3905 = v4;
                nat_mul(a3904, a3905)?
            };
            let a3907 = v10;
            nat_add(a3906, a3907)?
        };
        let a3909 = v15;
        let a3910 = match v1.clone() {
            Adt3::C0(_, h3908, _, _, _, _, _, _) => {
                h3908
            }
        };
        if if nat_lt(a3909, a3910) {
            let a3912 = v16;
            let a3913 = match v1.clone() {
                Adt3::C0(_, _, h3911, _, _, _, _, _) => {
                    h3911
                }
            };
            nat_lt(a3912, a3913)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f139({
                let a3922 = v6;
                let a3923 = {
                    let a3920 = match v1.clone() {
                        Adt3::C0(h3914, _, _, _, _, _, _, _) => {
                            h3914
                        }
                    };
                    let a3921 = {
                        let a3918 = v15;
                        let a3919 = {
                            let a3916 = match v1.clone() {
                                Adt3::C0(_, h3915, _, _, _, _, _, _) => {
                                    h3915
                                }
                            };
                            let a3917 = v16;
                            nat_mul(a3916, a3917)?
                        };
                        nat_add(a3918, a3919)?
                    };
                    nat_mul(a3920, a3921)?
                };
                nat_add(a3922, a3923)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f130(23u64, v4);
    let v6: u64 = {
        let a3924 = v0;
        let a3925 = v5;
        let a3926 = 0u64;
        nat_rem(a3924, a3925, a3926)
    };
    let v7: u64 = {
        let a3927 = v0;
        let a3928 = v5;
        let a3929 = 0u64;
        nat_quot(a3927, a3928, a3929)
    };
    let v8: Adt13 = Adt13::C1(f139(v7, v2)?);
    let a3930 = v6;
    let a3931 = 0u64;
    if nat_eq(a3930, a3931) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f139(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f199(v3), 4u64))
    } else {
        let a3932 = v6;
        let a3933 = 1u64;
        if nat_eq(a3932, a3933) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a3934 = v4;
                let a3935 = 0u64;
                if nat_eq(a3934, a3935) {
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
    let a3940 = {
        let a3937 = v0;
        let a3938 = match v1 {
            Adt3::C0(h3936, _, _, _, _, _, _, _) => {
                h3936
            }
        };
        let a3939 = 0u64;
        nat_rem(a3937, a3938, a3939)
    };
    let a3941 = 0u64;
    if nat_eq(a3940, a3941) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f139({
            let a3942 = v0;
            let a3943 = 1u64;
            nat_sub(a3942, a3943)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3944 = v3;
    let a3945 = 13u64;
    if nat_eq(a3944, a3945) {
        f208(v0, v1, v4)
    } else {
        let v5: u64 = f197(match v1.clone() {
            Adt3::C0(h3946, _, _, _, _, _, _, _) => {
                h3946
            }
        }, {
            let a3947 = v3;
            let a3948 = 16u64;
            nat_eq(a3947, a3948)
        })?;
        let v6: u64 = {
            let a3949 = v0;
            let a3950 = v5;
            let a3951 = 0u64;
            nat_quot(a3949, a3950, a3951)
        };
        let v7: u64 = {
            let a3952 = v0;
            let a3953 = v5;
            let a3954 = 0u64;
            nat_rem(a3952, a3953, a3954)
        };
        let v8: u64 = {
            let a3958 = 1u64;
            let a3959 = {
                let a3956 = match v1.clone() {
                    Adt3::C0(h3955, _, _, _, _, _, _, _) => {
                        h3955
                    }
                };
                let a3957 = 2u64;
                nat_mul(a3956, a3957)?
            };
            nat_add(a3958, a3959)?
        };
        let a3960 = v7;
        let a3961 = v8;
        if nat_lt(a3960, a3961) {
            f209(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f139(v6, v2)?;
            let a3962 = v3;
            let a3963 = 14u64;
            if nat_eq(a3962, a3963) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3964 = v7;
                let a3965 = v8;
                if nat_eq(a3964, a3965) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3968 = v7;
                    let a3969 = {
                        let a3966 = v8;
                        let a3967 = 1u64;
                        nat_add(a3966, a3967)?
                    };
                    if nat_eq(a3968, a3969) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3970, _, _, _, _, _, _, _) => {
                                h3970
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
    let a3971 = v0;
    let a3972 = 0u64;
    if nat_eq(a3971, a3972) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3979 = {
            let a3973 = v0;
            let a3974 = 1u64;
            nat_add(a3973, a3974)?
        };
        let a3980 = {
            let a3977 = 2u64;
            let a3978 = {
                let a3975 = v3;
                let a3976 = 2u64;
                nat_mul(a3975, a3976)?
            };
            nat_add(a3977, a3978)?
        };
        if nat_eq(a3979, a3980) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3981 = v0;
                let a3982 = 1u64;
                nat_sub(a3981, a3982)
            };
            let v5: u64 = {
                let a3983 = v4;
                let a3984 = 2u64;
                let a3985 = 0u64;
                nat_quot(a3983, a3984, a3985)
            };
            let a3989 = {
                let a3986 = v4;
                let a3987 = 2u64;
                let a3988 = 0u64;
                nat_rem(a3986, a3987, a3988)
            };
            let a3990 = 0u64;
            if nat_eq(a3989, a3990) {
                Ok(Adt9::C4(17u64, 0u64, f139(v5, v1)?, 4u64, f139(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f155(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3991 = v0;
    let a3992 = f27(v1, v2, v3)?;
    if nat_lt(a3991, a3992) {
        let a3993 = v0;
        let a3994 = v1;
        if nat_lt(a3993, a3994) {
            Ok(Adt9::C1(false, {
                let a3995 = 4u64;
                let a3996 = v0;
                nat_mul(a3995, a3996)?
            }, {
                let a3997 = 4u64;
                let a3998 = v0;
                nat_mul(a3997, a3998)?
            }))
        } else {
            let v4: u64 = {
                let a3999 = v0;
                let a4000 = v1;
                nat_sub(a3999, a4000)
            };
            let v5: u64 = f210(v4, v2)?;
            let a4001 = v5;
            let a4002 = 2u64;
            if nat_lt(a4001, a4002) {
                Ok(Adt9::C4(6u64, {
                    let a4003 = v5;
                    let a4004 = 0u64;
                    if nat_eq(a4003, a4004) {
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
    let a4005 = v0;
    let a4006 = 43u64;
    if nat_eq(a4005, a4006) {
        Adt11::C8(3u64)
    } else {
        let a4007 = v1;
        let a4008 = 43u64;
        if nat_eq(a4007, a4008) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    if f104(v0) {
        let a4009 = 2u64;
        let a4010 = v1;
        nat_mul(a4009, a4010)
    } else {
        let a4011 = v1;
        let a4012 = f193(v0);
        let a4013 = 0u64;
        Ok(nat_quot(a4011, a4012, a4013))
    }
}

pub fn f158(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4014 = f114(v0);
    match m4014 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f131(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4015, _, _, _) => {
                    h4015
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4016, _, _) => {
                    h4016
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4017, _) => {
                    h4017
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4018) => {
                    h4018
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h4019) => {
                    h4019
                }
            }, f131({
                let a4021 = v2;
                let a4022 = match v6.clone() {
                    Adt18::C0(_, h4020, _) => {
                        h4020
                    }
                };
                nat_mul(a4021, a4022)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4023, _, _, _) => {
                    h4023
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4024, _, _) => {
                    h4024
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4025, _) => {
                    h4025
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4026) => {
                    h4026
                }
            })?, match v6 {
                Adt18::C0(_, h4027, _) => {
                    h4027
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4028, _, _, _) => {
                    h4028
                }
            }))
        }
    }
}

pub fn f159(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4036 = {
        let a4029 = 2u64;
        let a4030 = f127(v1.clone())?;
        nat_mul(a4029, a4030)?
    };
    let a4037 = {
        let a4034 = f127(v1)?;
        let a4035 = {
            let a4032 = 1u64;
            let a4033 = match v0 {
                Adt3::C0(h4031, _, _, _, _, _, _, _) => {
                    h4031
                }
            };
            nat_add(a4032, a4033)?
        };
        nat_mul(a4034, a4035)?
    };
    nat_add(a4036, a4037)
}

pub fn f160(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f139(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f161(v0: u64, v1: u64) -> bool {
    let a4038 = v1;
    let a4039 = 2147483647u64;
    if nat_le(a4038, a4039) {
        let a4040 = v1;
        let a4041 = v0;
        nat_lt(a4040, a4041)
    } else {
        false
    }
}

pub fn f162(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4042 = f114(v0);
    match m4042 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4044 = v2;
                let a4045 = match v7.clone() {
                    Adt18::C0(_, h4043, _) => {
                        h4043
                    }
                };
                nat_mul(a4044, a4045)?
            };
            let m4058 = f214(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4046, _, _, _, _, _, _, _) => {
                    h4046
                }
            }, match v5.clone() {
                Adt3::C0(_, h4047, _, _, _, _, _, _) => {
                    h4047
                }
            }, match v5 {
                Adt3::C0(_, _, h4048, _, _, _, _, _) => {
                    h4048
                }
            }, {
                let a4051 = match v3.clone() {
                    Adt3::C0(h4049, _, _, _, _, _, _, _) => {
                        h4049
                    }
                };
                let a4052 = match v7.clone() {
                    Adt18::C0(_, h4050, _) => {
                        h4050
                    }
                };
                nat_mul(a4051, a4052)?
            }, match v3.clone() {
                Adt3::C0(_, h4053, _, _, _, _, _, _) => {
                    h4053
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4054, _, _, _) => {
                    h4054
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4055, _, _) => {
                    h4055
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4056, _) => {
                    h4056
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4057) => {
                    h4057
                }
            })?;
            match m4058 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4059 = v0;
                    let a4060 = 0u64;
                    if if nat_eq(a4059, a4060) {
                        true
                    } else {
                        let a4061 = v0;
                        let a4062 = 26u64;
                        nat_eq(a4061, a4062)
                    } {
                        Ok(Adt9::C1(false, v9, f139({
                            let a4066 = {
                                let a4064 = v1;
                                let a4065 = match v4.clone() {
                                    Adt3::C0(h4063, _, _, _, _, _, _, _) => {
                                        h4063
                                    }
                                };
                                nat_mul(a4064, a4065)?
                            };
                            let a4067 = v8;
                            nat_add(a4066, a4067)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4068) => {
                                h4068
                            }
                        }, f139({
                            let a4072 = {
                                let a4070 = v1;
                                let a4071 = match v4.clone() {
                                    Adt3::C0(h4069, _, _, _, _, _, _, _) => {
                                        h4069
                                    }
                                };
                                nat_mul(a4070, a4071)?
                            };
                            let a4073 = v8;
                            nat_add(a4072, a4073)?
                        }, v4.clone())?, {
                            let a4080 = {
                                let a4078 = {
                                    let a4075 = match v7 {
                                        Adt18::C0(_, h4074, _) => {
                                            h4074
                                        }
                                    };
                                    let a4076 = 1u64;
                                    nat_sub(a4075, a4076)
                                };
                                let a4079 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4077, _, _, _) => {
                                        h4077
                                    }
                                };
                                nat_mul(a4078, a4079)?
                            };
                            let a4081 = 4u64;
                            nat_add(a4080, a4081)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4082, _, _, _) => {
                                h4082
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f163(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4083 = v1;
    let a4084 = 0u64;
    if if nat_eq(a4083, a4084) {
        true
    } else {
        let a4085 = v2;
        let a4086 = 0u64;
        nat_eq(a4085, a4086)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f164(v0: u64, v1: u64) -> bool {
    let a4087 = v0;
    let a4088 = 4294967295u64;
    if nat_eq(a4087, a4088) {
        true
    } else {
        let a4089 = v0;
        let a4090 = 2147483648u64;
        if nat_lt(a4089, a4090) {
            let a4091 = v0;
            let a4092 = v1;
            nat_lt(a4091, a4092)
        } else {
            false
        }
    }
}

pub fn f165(v0: Adt7) -> R<u64> {
    let a4097 = if match v0.clone() {
        Adt7::C0(_, _, _, h4093, _) => {
            h4093
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4098 = {
        let a4095 = match v0 {
            Adt7::C0(_, _, _, _, h4094) => {
                h4094
            }
        };
        let a4096 = 1u64;
        if nat_eq(a4095, a4096) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4097, a4098)
}

pub fn f166(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4103 = {
            let a4100 = v2;
            let a4101 = match v0.clone() {
                Adt6::C0(_, h4099, _, _) => {
                    h4099
                }
            };
            nat_mul(a4100, a4101)?
        };
        let a4104 = f132(match v0.clone() {
            Adt6::C0(_, _, h4102, _) => {
                h4102
            }
        });
        nat_add(a4103, a4104)?
    };
    let v4: u64 = f133(match v0.clone() {
        Adt6::C0(_, _, h4105, _) => {
            h4105
        }
    });
    let v5: u64 = {
        let a4109 = {
            let a4107 = v3;
            let a4108 = match v0 {
                Adt6::C0(h4106, _, _, _) => {
                    h4106
                }
            };
            nat_add(a4107, a4108)?
        };
        let a4110 = v4;
        nat_sub(a4109, a4110)
    };
    let a4115 = {
        let a4111 = v5;
        let a4112 = v1;
        if nat_lt(a4111, a4112) {
            v5
        } else {
            v1
        }
    };
    let a4116 = {
        let a4113 = v3;
        let a4114 = v4;
        nat_sub(a4113, a4114)
    };
    Ok(nat_sub(a4115, a4116))
}

pub fn f167(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4125 = f72(match v0.clone() {
        Adt7::C0(h4117, _, _, _, _) => {
            h4117
        }
    }, match v1.clone() {
        Adt3::C0(h4118, _, _, _, _, _, _, _) => {
            h4118
        }
    }, f70(v3, v2.clone(), 0u64), {
        let a4122 = v4;
        let a4123 = {
            let m4121 = match v0.clone() {
                Adt7::C0(h4120, _, _, _, _) => {
                    h4120
                }
            };
            match m4121 {
                Adt6::C0(h4119, _, _, _) => {
                    h4119
                }
            }
        };
        let a4124 = 0u64;
        nat_rem(a4122, a4123, a4124)
    })?;
    match m4125 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4134 = f72(match v0.clone() {
                Adt7::C0(_, h4126, _, _, _) => {
                    h4126
                }
            }, match v1.clone() {
                Adt3::C0(_, h4127, _, _, _, _, _, _) => {
                    h4127
                }
            }, f70(v3, v2.clone(), 1u64), {
                let a4131 = v4;
                let a4132 = {
                    let m4130 = match v0 {
                        Adt7::C0(h4129, _, _, _, _) => {
                            h4129
                        }
                    };
                    match m4130 {
                        Adt6::C0(h4128, _, _, _) => {
                            h4128
                        }
                    }
                };
                let a4133 = 0u64;
                nat_quot(a4131, a4132, a4133)
            })?;
            match m4134 {
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
    let m4135 = v0;
    match m4135 {
        Adt16::C0(v19) => {
            f215(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4136 = v20;
            let a4137 = 66u64;
            if nat_eq(a4136, a4137) {
                f216(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f217(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f169(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4144 = v3;
        let a4145 = {
            let a4141 = f139({
                let a4138 = v0;
                let a4139 = 2u64;
                let a4140 = 0u64;
                nat_quot(a4138, a4139, a4140)
            }, v1)?;
            let a4142 = v2;
            let a4143 = 0u64;
            nat_quot(a4141, a4142, a4143)
        };
        nat_add(a4144, a4145)?
    };
    let a4149 = {
        let a4146 = v0;
        let a4147 = 2u64;
        let a4148 = 0u64;
        nat_rem(a4146, a4147, a4148)
    };
    let a4150 = 0u64;
    if nat_eq(a4149, a4150) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4151 = v4;
        let a4152 = 0u64;
        if nat_eq(a4151, a4152) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f170(v0: u64, v1: u64) -> R<u64> {
    let a4155 = 8u64;
    let a4156 = {
        let a4153 = f218(v0);
        let a4154 = v1;
        nat_mul(a4153, a4154)?
    };
    nat_add(a4155, a4156)
}

pub fn f171(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4157, _, _, _, _, _, _, _) => {
            h4157
        }
    };
    let v7: u64 = f218(v3);
    let v8: u64 = {
        let a4160 = 1u64;
        let a4161 = {
            let a4158 = v6;
            let a4159 = v7;
            nat_mul(a4158, a4159)?
        };
        nat_add(a4160, a4161)?
    };
    let a4162 = v0;
    let a4163 = 0u64;
    if nat_eq(a4162, a4163) {
        Ok(f219(0u64))
    } else {
        let a4164 = v0;
        let a4165 = v8;
        if nat_lt(a4164, a4165) {
            let v9: u64 = {
                let a4166 = v0;
                let a4167 = 1u64;
                nat_sub(a4166, a4167)
            };
            let v10: u64 = {
                let a4168 = v9;
                let a4169 = v7;
                let a4170 = 0u64;
                nat_rem(a4168, a4169, a4170)
            };
            let v11: u64 = f139({
                let a4176 = {
                    let a4171 = v1;
                    let a4172 = v6;
                    nat_mul(a4171, a4172)?
                };
                let a4177 = {
                    let a4173 = v9;
                    let a4174 = v7;
                    let a4175 = 0u64;
                    nat_quot(a4173, a4174, a4175)
                };
                nat_add(a4176, a4177)?
            }, v2)?;
            let a4178 = v3;
            let a4179 = 30u64;
            if nat_eq(a4178, a4179) {
                let a4180 = v10;
                let a4181 = 0u64;
                if nat_eq(a4180, a4181) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4182 = v10;
                    let a4183 = 1u64;
                    if nat_eq(a4182, a4183) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4184 = v10;
                        let a4185 = 2u64;
                        if nat_eq(a4184, a4185) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4186 = v10;
                let a4187 = 0u64;
                if nat_eq(a4186, a4187) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4188 = v10;
                    let a4189 = 1u64;
                    if nat_eq(a4188, a4189) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4190 = v0;
                let a4191 = v8;
                nat_sub(a4190, a4191)
            };
            let a4192 = v12;
            let a4193 = 0u64;
            if nat_eq(a4192, a4193) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4194 = v12;
                let a4195 = 1u64;
                if nat_eq(a4194, a4195) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4196 = v12;
                    let a4197 = 2u64;
                    if nat_eq(a4196, a4197) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4198 = v12;
                        let a4199 = 3u64;
                        if nat_eq(a4198, a4199) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4200 = v12;
                            let a4201 = 4u64;
                            if nat_eq(a4200, a4201) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4202 = v12;
                                let a4203 = 5u64;
                                if nat_eq(a4202, a4203) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4204 = v12;
                                    let a4205 = 6u64;
                                    if nat_eq(a4204, a4205) {
                                        if f220(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4206 = 3u64;
                                                let a4207 = v1;
                                                nat_add(a4206, a4207)?
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
        let a4208 = v0;
        let a4209 = f102(v3);
        let a4210 = 0u64;
        nat_quot(a4208, a4209, a4210)
    };
    let v5: u64 = {
        let a4215 = 3u64;
        let a4216 = {
            let a4212 = v4;
            let a4213 = match v1.clone() {
                Adt3::C0(h4211, _, _, _, _, _, _, _) => {
                    h4211
                }
            };
            let a4214 = 0u64;
            nat_quot(a4212, a4213, a4214)
        };
        nat_add(a4215, a4216)?
    };
    let v6: u64 = f139(v4, v1)?;
    let v7: u64 = f139(v4, v2)?;
    let a4217 = v3;
    let a4218 = 30u64;
    if nat_eq(a4217, a4218) {
        let a4222 = {
            let a4219 = v0;
            let a4220 = 3u64;
            let a4221 = 0u64;
            nat_rem(a4219, a4220, a4221)
        };
        let a4223 = 0u64;
        if nat_eq(a4222, a4223) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4227 = {
                let a4224 = v0;
                let a4225 = 3u64;
                let a4226 = 0u64;
                nat_rem(a4224, a4225, a4226)
            };
            let a4228 = 1u64;
            if nat_eq(a4227, a4228) {
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
        let a4229 = v0;
        let a4230 = 8388608u64;
        let a4231 = 0u64;
        nat_quot(a4229, a4230, a4231)
    };
    let a4232 = v0;
    let a4233 = 2147483648u64;
    if if if nat_lt(a4232, a4233) {
        let a4234 = 127u64;
        let a4235 = v1;
        nat_le(a4234, a4235)
    } else {
        false
    } {
        let a4236 = v1;
        let a4237 = 190u64;
        nat_lt(a4236, a4237)
    } else {
        false
    } {
        let v2: u64 = {
            let a4241 = 8388608u64;
            let a4242 = {
                let a4238 = v0;
                let a4239 = 8388608u64;
                let a4240 = 0u64;
                nat_rem(a4238, a4239, a4240)
            };
            nat_add(a4241, a4242)?
        };
        let a4243 = v1;
        let a4244 = 150u64;
        if nat_lt(a4243, a4244) {
            let v3: u64 = f221({
                let a4245 = 150u64;
                let a4246 = v1;
                nat_sub(a4245, a4246)
            });
            let a4250 = {
                let a4247 = v2;
                let a4248 = v3;
                let a4249 = 0u64;
                nat_rem(a4247, a4248, a4249)
            };
            let a4251 = 0u64;
            if nat_eq(a4250, a4251) {
                let a4252 = v2;
                let a4253 = v3;
                let a4254 = 0u64;
                Ok(nat_quot(a4252, a4253, a4254))
            } else {
                Ok(0u64)
            }
        } else {
            let a4257 = f221({
                let a4255 = v1;
                let a4256 = 150u64;
                nat_sub(a4255, a4256)
            });
            let a4258 = v2;
            nat_mul(a4257, a4258)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f174(v0: Adt14, v1: Adt14) -> bool {
    let a4261 = match v0.clone() {
        Adt14::C0(_, h4259, _, _, _, _, _, _, _, _, _) => {
            h4259
        }
    };
    let a4262 = match v1.clone() {
        Adt14::C0(_, h4260, _, _, _, _, _, _, _, _, _) => {
            h4260
        }
    };
    if if if nat_eq(a4261, a4262) {
        let a4265 = match v0.clone() {
            Adt14::C0(_, _, h4263, _, _, _, _, _, _, _, _) => {
                h4263
            }
        };
        let a4266 = match v1.clone() {
            Adt14::C0(_, _, h4264, _, _, _, _, _, _, _, _) => {
                h4264
            }
        };
        nat_eq(a4265, a4266)
    } else {
        false
    } {
        let a4269 = match v0.clone() {
            Adt14::C0(_, _, _, h4267, _, _, _, _, _, _, _) => {
                h4267
            }
        };
        let a4270 = match v1.clone() {
            Adt14::C0(_, _, _, h4268, _, _, _, _, _, _, _) => {
                h4268
            }
        };
        nat_eq(a4269, a4270)
    } else {
        false
    } {
        let a4273 = match v0 {
            Adt14::C0(_, _, _, _, h4271, _, _, _, _, _, _) => {
                h4271
            }
        };
        let a4274 = match v1 {
            Adt14::C0(_, _, _, _, h4272, _, _, _, _, _, _) => {
                h4272
            }
        };
        nat_eq(a4273, a4274)
    } else {
        false
    }
}

pub fn f175(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4275 = v0;
    let a4276 = 0u64;
    if if if nat_eq(a4275, a4276) {
        true
    } else {
        let a4277 = v1;
        let a4278 = 0u64;
        nat_eq(a4277, a4278)
    } {
        true
    } else {
        let a4279 = v2;
        let a4280 = 0u64;
        nat_eq(a4279, a4280)
    } {
        true
    } else {
        let a4281 = v3;
        let a4282 = 0u64;
        nat_eq(a4281, a4282)
    }
}

pub fn f176(v0: Adt14, v1: Adt14) -> bool {
    if f182(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4283, _, _, _, _, _) => {
            h4283
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4284, _, _, _, _, _) => {
            h4284
        }
    }) {
        let a4286 = match v0 {
            Adt14::C0(_, h4285, _, _, _, _, _, _, _, _, _) => {
                h4285
            }
        };
        let a4287 = 2147483647u64;
        nat_le(a4286, a4287)
    } else {
        true
    }
}

pub fn f177(v0: Adt14) -> bool {
    let a4292 = {
        let a4289 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4288) => {
                h4288
            }
        };
        let a4290 = 4u64;
        let a4291 = 0u64;
        nat_rem(a4289, a4290, a4291)
    };
    let a4293 = 0u64;
    if if nat_eq(a4292, a4293) {
        f222(match v0.clone() {
            Adt14::C0(_, h4294, _, _, _, _, _, _, _, _, _) => {
                h4294
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4295, _, _, _, _, _) => {
                h4295
            }
        })
    } else {
        false
    } {
        f98(match v0.clone() {
            Adt14::C0(_, _, h4296, _, _, _, _, _, _, _, _) => {
                h4296
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4297, _, _, _, _, _, _, _) => {
                h4297
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4298, _, _, _, _, _, _) => {
                h4298
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4299, _, _, _, _) => {
                h4299
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4300, _, _, _) => {
                h4300
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4301, _, _) => {
                h4301
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f178(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4303 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4302, _) => {
            h4302
        }
    };
    let a4304 = 0u64;
    if nat_eq(a4303, a4304) {
        Ok(true)
    } else {
        let a4306 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4305, _) => {
                h4305
            }
        };
        let a4307 = 0u64;
        if nat_eq(a4306, a4307) {
            Ok(true)
        } else {
            let a4309 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4308) => {
                    h4308
                }
            };
            let a4310 = 0u64;
            if nat_eq(a4309, a4310) {
                Ok(true)
            } else {
                let a4312 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4311) => {
                        h4311
                    }
                };
                let a4313 = 0u64;
                if nat_eq(a4312, a4313) {
                    Ok(true)
                } else {
                    let a4319 = {
                        let a4316 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4314) => {
                                h4314
                            }
                        };
                        let a4317 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4315, _) => {
                                h4315
                            }
                        };
                        nat_add(a4316, a4317)?
                    };
                    let a4320 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4318) => {
                            h4318
                        }
                    };
                    if nat_le(a4319, a4320) {
                        Ok(true)
                    } else {
                        let a4326 = {
                            let a4323 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4321) => {
                                    h4321
                                }
                            };
                            let a4324 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4322, _) => {
                                    h4322
                                }
                            };
                            nat_add(a4323, a4324)?
                        };
                        let a4327 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4325) => {
                                h4325
                            }
                        };
                        Ok(nat_le(a4326, a4327))
                    }
                }
            }
        }
    }
}

pub fn f179(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f174(v0.clone(), v1.clone()) {
        let a4330 = match v0.clone() {
            Adt14::C0(h4328, _, _, _, _, _, _, _, _, _, _) => {
                h4328
            }
        };
        let a4331 = match v1.clone() {
            Adt14::C0(h4329, _, _, _, _, _, _, _, _, _, _) => {
                h4329
            }
        };
        nat_eq(a4330, a4331)
    } else {
        false
    } {
        let a4334 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4332) => {
                h4332
            }
        };
        let a4335 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4333) => {
                h4333
            }
        };
        nat_eq(a4334, a4335)
    } else {
        false
    } {
        let a4338 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4336, _, _, _, _, _) => {
                h4336
            }
        };
        let a4339 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4337, _, _, _, _, _) => {
                h4337
            }
        };
        nat_eq(a4338, a4339)
    } else {
        false
    } {
        let a4342 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4340, _, _, _, _) => {
                h4340
            }
        };
        let a4343 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4341, _, _, _, _) => {
                h4341
            }
        };
        nat_eq(a4342, a4343)
    } else {
        false
    } {
        let a4346 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4344, _, _, _) => {
                h4344
            }
        };
        let a4347 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4345, _, _, _) => {
                h4345
            }
        };
        nat_eq(a4346, a4347)
    } else {
        false
    } {
        let a4350 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4348, _, _) => {
                h4348
            }
        };
        let a4351 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4349, _, _) => {
                h4349
            }
        };
        nat_eq(a4350, a4351)
    } else {
        false
    }
}

pub fn f180(v0: Adt14) -> R<bool> {
    if f223(match v0.clone() {
        Adt14::C0(_, h4352, _, _, _, _, _, _, _, _, _) => {
            h4352
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4353, _, _, _, _, _) => {
            h4353
        }
    }) {
        f99(f224(match v0.clone() {
            Adt14::C0(_, h4354, _, _, _, _, _, _, _, _, _) => {
                h4354
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4355, _, _, _, _, _) => {
                h4355
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4356, _, _, _, _, _, _, _, _) => {
                h4356
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4357, _, _, _, _, _, _, _) => {
                h4357
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4358, _, _, _, _, _, _) => {
                h4358
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4359, _, _, _, _) => {
                h4359
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4360, _, _, _) => {
                h4360
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4361, _, _) => {
                h4361
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f181(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4362 = v1;
    let a4363 = 0u64;
    if if nat_eq(a4362, a4363) {
        true
    } else {
        let a4364 = v2;
        let a4365 = 0u64;
        nat_eq(a4364, a4365)
    } {
        Ok(0u64)
    } else {
        let a4368 = f100(v1, v2, v3)?;
        let a4369 = {
            let a4366 = v2;
            let a4367 = f225(v0, v1)?;
            nat_mul(a4366, a4367)?
        };
        nat_add(a4368, a4369)
    }
}

pub fn f182(v0: u64, v1: u64) -> bool {
    let a4370 = v0;
    let a4371 = 4u64;
    if nat_eq(a4370, a4371) {
        let a4372 = v1;
        let a4373 = 4u64;
        nat_eq(a4372, a4373)
    } else {
        false
    }
}

pub fn f183(v0: u64) -> R<u64> {
    let a4374 = 6u64;
    let a4375 = v0;
    nat_add(a4374, a4375)
}

pub fn f184(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4376, _, _, _, _, _, _, _, _, _, _) => {
            h4376
        }
    }, match v0.clone() {
        Adt14::C0(_, h4377, _, _, _, _, _, _, _, _, _) => {
            h4377
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4378, _, _, _, _, _, _, _, _) => {
            h4378
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4379, _, _, _, _, _, _, _) => {
            h4379
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4380, _, _, _, _, _, _) => {
            h4380
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4381, _, _, _, _) => {
            h4381
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4382, _, _, _) => {
            h4382
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4383, _, _) => {
            h4383
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4384, _) => {
            h4384
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4385) => {
            h4385
        }
    })
}

pub fn f185(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4386 = v0;
    let a4387 = 2147483647u64;
    if if if nat_le(a4386, a4387) {
        let a4388 = 0u64;
        let a4389 = v3;
        if nat_lt(a4388, a4389) {
            let a4390 = v3;
            let a4391 = 2147483647u64;
            nat_le(a4390, a4391)
        } else {
            false
        }
    } else {
        false
    } {
        let a4396 = {
            let a4394 = {
                let a4392 = v2;
                let a4393 = v3;
                nat_add(a4392, a4393)?
            };
            let a4395 = 1u64;
            nat_sub(a4394, a4395)
        };
        let a4397 = 2147483647u64;
        nat_le(a4396, a4397)
    } else {
        false
    } {
        let a4402 = {
            let a4400 = {
                let a4398 = v0;
                let a4399 = v1;
                nat_mul(a4398, a4399)?
            };
            let a4401 = f226(v2, v3)?;
            nat_mul(a4400, a4401)?
        };
        let a4403 = 9223372036854775807u64;
        Ok(nat_le(a4402, a4403))
    } else {
        Ok(false)
    }
}

pub fn f186(v0: Adt14) -> R<u64> {
    let a4434 = {
        let a4426 = {
            let a4418 = {
                let a4410 = 4u64;
                let a4411 = {
                    let a4408 = {
                        let a4405 = match v0.clone() {
                            Adt14::C0(_, h4404, _, _, _, _, _, _, _, _, _) => {
                                h4404
                            }
                        };
                        let a4406 = 1u64;
                        nat_sub(a4405, a4406)
                    };
                    let a4409 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4407, _, _, _, _, _) => {
                            h4407
                        }
                    };
                    nat_mul(a4408, a4409)?
                };
                nat_add(a4410, a4411)?
            };
            let a4419 = {
                let a4416 = {
                    let a4413 = match v0.clone() {
                        Adt14::C0(_, _, h4412, _, _, _, _, _, _, _, _) => {
                            h4412
                        }
                    };
                    let a4414 = 1u64;
                    nat_sub(a4413, a4414)
                };
                let a4417 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4415, _, _, _, _) => {
                        h4415
                    }
                };
                nat_mul(a4416, a4417)?
            };
            nat_add(a4418, a4419)?
        };
        let a4427 = {
            let a4424 = {
                let a4421 = match v0.clone() {
                    Adt14::C0(_, _, _, h4420, _, _, _, _, _, _, _) => {
                        h4420
                    }
                };
                let a4422 = 1u64;
                nat_sub(a4421, a4422)
            };
            let a4425 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4423, _, _, _) => {
                    h4423
                }
            };
            nat_mul(a4424, a4425)?
        };
        nat_add(a4426, a4427)?
    };
    let a4435 = {
        let a4432 = {
            let a4429 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4428, _, _, _, _, _, _) => {
                    h4428
                }
            };
            let a4430 = 1u64;
            nat_sub(a4429, a4430)
        };
        let a4433 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4431, _, _) => {
                h4431
            }
        };
        nat_mul(a4432, a4433)?
    };
    nat_add(a4434, a4435)
}

pub fn f187(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4436 = v0;
    let a4437 = 0u64;
    if if if if nat_eq(a4436, a4437) {
        true
    } else {
        let a4438 = v1;
        let a4439 = 0u64;
        nat_eq(a4438, a4439)
    } {
        true
    } else {
        let a4440 = v2;
        let a4441 = 0u64;
        nat_eq(a4440, a4441)
    } {
        true
    } else {
        let a4442 = v3;
        let a4443 = 0u64;
        nat_eq(a4442, a4443)
    } {
        Ok(0u64)
    } else {
        let a4444 = f227(v0, v1, v2, v3, v5)?;
        let a4445 = f228(v0, v1, v2, v3, v4)?;
        nat_add(a4444, a4445)
    }
}

pub fn f188(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4446 = v0;
    let a4447 = 0u64;
    if if nat_eq(a4446, a4447) {
        true
    } else {
        let a4448 = v1;
        let a4449 = 0u64;
        nat_eq(a4448, a4449)
    } {
        Ok(0u64)
    } else {
        let a4452 = f100(v0, v1, v2)?;
        let a4453 = {
            let a4450 = v1;
            let a4451 = f229(v0)?;
            nat_mul(a4450, a4451)?
        };
        nat_add(a4452, a4453)
    }
}

pub fn f189(v0: u64, v1: u64, v2: u64) -> bool {
    let a4454 = v0;
    let a4455 = 1u64;
    if nat_le(a4454, a4455) {
        true
    } else {
        let a4459 = {
            let a4456 = v1;
            let a4457 = v2;
            let a4458 = 1u64;
            nat_rem(a4456, a4457, a4458)
        };
        let a4460 = 0u64;
        nat_eq(a4459, a4460)
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4461 = v1;
    let a4462 = 1u64;
    if if if nat_le(a4461, a4462) {
        true
    } else {
        let a4463 = v0;
        let a4464 = v2;
        nat_le(a4463, a4464)
    } {
        let a4465 = v3;
        let a4466 = 1u64;
        if nat_le(a4465, a4466) {
            true
        } else {
            let a4473 = {
                let a4471 = v0;
                let a4472 = {
                    let a4469 = {
                        let a4467 = v1;
                        let a4468 = 1u64;
                        nat_sub(a4467, a4468)
                    };
                    let a4470 = v2;
                    nat_mul(a4469, a4470)?
                };
                nat_add(a4471, a4472)?
            };
            let a4474 = v4;
            nat_le(a4473, a4474)
        }
    } else {
        false
    } {
        let a4475 = v5;
        let a4476 = 1u64;
        if nat_le(a4475, a4476) {
            Ok(true)
        } else {
            let a4489 = {
                let a4487 = {
                    let a4481 = v0;
                    let a4482 = {
                        let a4479 = {
                            let a4477 = v1;
                            let a4478 = 1u64;
                            nat_sub(a4477, a4478)
                        };
                        let a4480 = v2;
                        nat_mul(a4479, a4480)?
                    };
                    nat_add(a4481, a4482)?
                };
                let a4488 = {
                    let a4485 = {
                        let a4483 = v3;
                        let a4484 = 1u64;
                        nat_sub(a4483, a4484)
                    };
                    let a4486 = v4;
                    nat_mul(a4485, a4486)?
                };
                nat_add(a4487, a4488)?
            };
            let a4490 = v6;
            Ok(nat_le(a4489, a4490))
        }
    } else {
        Ok(false)
    }
}

pub fn f191(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f230(v0, v1, v6, v7, v8) {
        let a4491 = v5;
        let a4492 = 26u64;
        nat_eq(a4491, a4492)
    } else {
        false
    } {
        let a4493 = v3;
        let a4494 = v7;
        nat_eq(a4493, a4494)
    } else {
        false
    } {
        let a4495 = v4;
        let a4496 = v8;
        nat_eq(a4495, a4496)
    } else {
        false
    } {
        let a4497 = v9;
        let a4498 = 1u64;
        nat_eq(a4497, a4498)
    } else {
        false
    } {
        let a4499 = v10;
        let a4500 = f231(v0);
        nat_eq(a4499, a4500)
    } else {
        false
    } {
        let a4501 = v11;
        let a4502 = v1;
        nat_eq(a4501, a4502)
    } else {
        false
    } {
        let a4503 = v12;
        let a4504 = v6;
        nat_eq(a4503, a4504)
    } else {
        false
    } {
        let a4505 = v13;
        let a4506 = v7;
        nat_eq(a4505, a4506)
    } else {
        false
    } {
        let a4507 = v14;
        let a4508 = v8;
        nat_eq(a4507, a4508)
    } else {
        false
    }
}

pub fn f192(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4509 = v0;
    let a4510 = 0u64;
    if if if nat_eq(a4509, a4510) {
        true
    } else {
        let a4511 = v1;
        let a4512 = 0u64;
        nat_eq(a4511, a4512)
    } {
        true
    } else {
        let a4513 = v2;
        let a4514 = 0u64;
        nat_eq(a4513, a4514)
    } {
        true
    } else {
        let a4515 = v3;
        let a4516 = 0u64;
        nat_eq(a4515, a4516)
    }
}

pub fn f193(v0: u64) -> u64 {
    let m4517 = f114(v0);
    match m4517 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4518, _) => {
                    h4518
                }
            }
        }
    }
}

pub fn f194(v0: Adt3, v1: bool) -> R<u64> {
    f232(v0.clone(), {
        let a4520 = match v0.clone() {
            Adt3::C0(h4519, _, _, _, _, _, _, _) => {
                h4519
            }
        };
        let a4521 = 1u64;
        nat_sub(a4520, a4521)
    }, {
        let a4523 = match v0.clone() {
            Adt3::C0(_, h4522, _, _, _, _, _, _) => {
                h4522
            }
        };
        let a4524 = 1u64;
        nat_sub(a4523, a4524)
    }, {
        let a4526 = match v0.clone() {
            Adt3::C0(_, _, h4525, _, _, _, _, _) => {
                h4525
            }
        };
        let a4527 = 1u64;
        nat_sub(a4526, a4527)
    }, {
        let a4529 = match v0 {
            Adt3::C0(_, _, _, h4528, _, _, _, _) => {
                h4528
            }
        };
        let a4530 = 1u64;
        nat_sub(a4529, a4530)
    }, v1)
}

pub fn f195(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4531 = v0;
    let a4532 = 0u64;
    if nat_eq(a4531, a4532) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4533 = v0;
            let a4534 = 1u64;
            nat_sub(a4533, a4534)
        };
        let v5: u64 = {
            let a4535 = v4;
            let a4536 = v1;
            let a4537 = 0u64;
            nat_quot(a4535, a4536, a4537)
        };
        let v6: u64 = {
            let a4545 = {
                let a4538 = v5;
                let a4539 = v2;
                nat_mul(a4538, a4539)?
            };
            let a4546 = {
                let a4543 = {
                    let a4540 = v4;
                    let a4541 = v1;
                    let a4542 = 0u64;
                    nat_rem(a4540, a4541, a4542)
                };
                let a4544 = v3;
                nat_mul(a4543, a4544)?
            };
            nat_add(a4545, a4546)?
        };
        let a4547 = v5;
        let a4548 = 0u64;
        if nat_eq(a4547, a4548) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4557 = {
                    let a4551 = {
                        let a4549 = v5;
                        let a4550 = 1u64;
                        nat_sub(a4549, a4550)
                    };
                    let a4552 = v2;
                    nat_mul(a4551, a4552)?
                };
                let a4558 = {
                    let a4555 = {
                        let a4553 = v1;
                        let a4554 = 1u64;
                        nat_sub(a4553, a4554)
                    };
                    let a4556 = v3;
                    nat_mul(a4555, a4556)?
                };
                nat_add(a4557, a4558)?
            };
            let a4559 = v6;
            let a4560 = v7;
            if nat_lt(a4559, a4560) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f196(v0: Adt3, v1: u64) -> bool {
    let a4561 = v1;
    let a4562 = 0u64;
    if !nat_eq(a4561, a4562) {
        true
    } else {
        let a4564 = match v0 {
            Adt3::C0(_, _, _, _, h4563, _, _, _) => {
                h4563
            }
        };
        let a4565 = 4u64;
        nat_eq(a4564, a4565)
    }
}

pub fn f197(v0: u64, v1: bool) -> R<u64> {
    let a4568 = {
        let a4566 = v0;
        let a4567 = 2u64;
        nat_mul(a4566, a4567)?
    };
    let a4569 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4568, a4569)
}

pub fn f198(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4570 = f233(v0, v1);
    match m4570 {
        None => {
            if v5 {
                let m4571 = f234(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4571 {
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
                let a4572 = 4u64;
                let a4573 = v28;
                nat_le(a4572, a4573)
            };
            let a4574 = v2;
            let a4575 = 0u64;
            if if if if if nat_eq(a4574, a4575) {
                let a4576 = v4;
                let a4577 = 0u64;
                nat_eq(a4576, a4577)
            } else {
                false
            } {
                let a4578 = v6;
                let a4579 = v14;
                if nat_eq(a4578, a4579) {
                    let a4580 = v7;
                    let a4581 = v15;
                    nat_eq(a4580, a4581)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4582 = v8;
                let a4583 = v16;
                if nat_eq(a4582, a4583) {
                    let a4584 = v9;
                    let a4585 = v17;
                    nat_eq(a4584, a4585)
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
                        let a4586 = v3;
                        let a4587 = 0u64;
                        nat_eq(a4586, a4587)
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
                    let a4592 = {
                        let a4588 = v6;
                        let a4589 = v7;
                        nat_mul(a4588, a4589)?
                    };
                    let a4593 = {
                        let a4590 = v8;
                        let a4591 = v9;
                        nat_mul(a4590, a4591)?
                    };
                    nat_mul(a4592, a4593)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f199(v0: u64) -> u64 {
    let m4594 = f114(v0);
    match m4594 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4596 = match v1.clone() {
                Adt18::C0(_, h4595, _) => {
                    h4595
                }
            };
            let a4597 = 1u64;
            if if nat_eq(a4596, a4597) {
                let a4599 = match v1.clone() {
                    Adt18::C0(_, _, h4598) => {
                        h4598
                    }
                };
                let a4600 = 1u64;
                if if if nat_eq(a4599, a4600) {
                    true
                } else {
                    let a4602 = match v1.clone() {
                        Adt18::C0(_, _, h4601) => {
                            h4601
                        }
                    };
                    let a4603 = 2u64;
                    nat_eq(a4602, a4603)
                } {
                    true
                } else {
                    let a4605 = match v1.clone() {
                        Adt18::C0(_, _, h4604) => {
                            h4604
                        }
                    };
                    let a4606 = 4u64;
                    nat_eq(a4605, a4606)
                } {
                    true
                } else {
                    let a4608 = match v1.clone() {
                        Adt18::C0(_, _, h4607) => {
                            h4607
                        }
                    };
                    let a4609 = 8u64;
                    nat_eq(a4608, a4609)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4610) => {
                        h4610
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f200(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4615 = {
        let a4613 = {
            let a4611 = v0;
            let a4612 = v1;
            nat_sub(a4611, a4612)
        };
        let a4614 = 1u64;
        nat_sub(a4613, a4614)
    };
    let a4616 = v2;
    nat_add(a4615, a4616)
}

pub fn f201(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f131({
        let a4617 = v0;
        let a4618 = v4;
        let a4619 = 0u64;
        nat_rem(a4617, a4618, a4619)
    }, {
        let a4620 = v1;
        let a4621 = v5;
        let a4622 = 0u64;
        nat_rem(a4620, a4621, a4622)
    }, {
        let a4623 = v2;
        let a4624 = v6;
        let a4625 = 0u64;
        nat_rem(a4623, a4624, a4625)
    }, {
        let a4626 = v3;
        let a4627 = v7;
        let a4628 = 0u64;
        nat_rem(a4626, a4627, a4628)
    }, v8, v9, v10, v11)
}

pub fn f202(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4629 = v1;
    let a4630 = 2147483648u64;
    if nat_lt(a4629, a4630) {
        let a4641 = {
            let a4639 = {
                let a4634 = {
                    let a4631 = v0;
                    let a4632 = v2;
                    let a4633 = 0u64;
                    nat_rem(a4631, a4632, a4633)
                };
                let a4635 = v2;
                nat_add(a4634, a4635)?
            };
            let a4640 = {
                let a4636 = v1;
                let a4637 = v2;
                let a4638 = 0u64;
                nat_rem(a4636, a4637, a4638)
            };
            nat_sub(a4639, a4640)
        };
        let a4642 = v2;
        let a4643 = 0u64;
        Ok(nat_rem(a4641, a4642, a4643))
    } else {
        let a4646 = {
            let a4644 = v0;
            let a4645 = f132(v1);
            nat_add(a4644, a4645)?
        };
        let a4647 = v2;
        let a4648 = 0u64;
        Ok(nat_rem(a4646, a4647, a4648))
    }
}

pub fn f203(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4651 = f133(v1);
    let a4652 = {
        let a4649 = v0;
        let a4650 = f132(v1);
        nat_add(a4649, a4650)?
    };
    if nat_le(a4651, a4652) {
        let a4653 = f204(v0, v1)?;
        let a4654 = v2;
        Ok(nat_lt(a4653, a4654))
    } else {
        Ok(false)
    }
}

pub fn f204(v0: u64, v1: u64) -> R<u64> {
    let a4657 = {
        let a4655 = v0;
        let a4656 = f132(v1);
        nat_add(a4655, a4656)?
    };
    let a4658 = f133(v1);
    Ok(nat_sub(a4657, a4658))
}

pub fn f205(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4659 = v0;
    let a4660 = v1;
    if nat_lt(a4659, a4660) {
        let a4661 = v1;
        let a4662 = v0;
        Ok(nat_sub(a4661, a4662))
    } else {
        let a4665 = v0;
        let a4666 = {
            let a4663 = v1;
            let a4664 = v2;
            nat_add(a4663, a4664)?
        };
        if nat_lt(a4665, a4666) {
            let a4667 = v0;
            let a4668 = v1;
            Ok(nat_sub(a4667, a4668))
        } else {
            let a4675 = {
                let a4671 = 2u64;
                let a4672 = {
                    let a4669 = v2;
                    let a4670 = 1u64;
                    nat_sub(a4669, a4670)
                };
                nat_mul(a4671, a4672)?
            };
            let a4676 = {
                let a4673 = v0;
                let a4674 = v1;
                nat_sub(a4673, a4674)
            };
            Ok(nat_sub(a4675, a4676))
        }
    }
}

pub fn f206(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4679 = {
        let a4677 = v2;
        let a4678 = f232(v1.clone(), f70(v0, v1.clone(), 0u64), f70(v0, v1.clone(), 1u64), f70(v0, v1.clone(), 2u64), f70(v0, v1.clone(), 3u64), false)?;
        nat_add(a4677, a4678)?
    };
    let a4680 = f232(v1.clone(), f70(v0, v1.clone(), 0u64), f70(v0, v1.clone(), 1u64), f70(v0, v1.clone(), 2u64), f70(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4679, a4680))
}

pub fn f207(v0: u64, v1: u64, v2: u64) -> bool {
    let a4681 = v0;
    let a4682 = 0u64;
    if nat_eq(a4681, a4682) {
        let a4683 = v2;
        let a4684 = v1;
        nat_le(a4683, a4684)
    } else {
        let a4685 = v0;
        let a4686 = 1u64;
        if nat_eq(a4685, a4686) {
            let a4687 = v2;
            let a4688 = v1;
            nat_lt(a4687, a4688)
        } else {
            let a4689 = v0;
            let a4690 = 2u64;
            if nat_eq(a4689, a4690) {
                let a4691 = v1;
                let a4692 = v2;
                nat_le(a4691, a4692)
            } else {
                let a4693 = v0;
                let a4694 = 3u64;
                if nat_eq(a4693, a4694) {
                    let a4695 = v1;
                    let a4696 = v2;
                    nat_lt(a4695, a4696)
                } else {
                    let a4697 = v1;
                    let a4698 = v2;
                    nat_eq(a4697, a4698)
                }
            }
        }
    }
}

pub fn f208(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4699 = v0;
    let a4700 = 0u64;
    if nat_eq(a4699, a4700) {
        Ok(f219(0u64))
    } else {
        let a4703 = {
            let a4701 = v0;
            let a4702 = 1u64;
            nat_add(a4701, a4702)?
        };
        let a4704 = f128(v1.clone(), 13u64, v2)?;
        if nat_eq(a4703, a4704) {
            Ok(Adt9::C4({
                let a4705 = v2;
                let a4706 = 0u64;
                if nat_eq(a4705, a4706) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4707 = v0;
                let a4708 = 1u64;
                nat_sub(a4707, a4708)
            };
            if f196(v1.clone(), v2) {
                let v4: u64 = f197(match v1.clone() {
                    Adt3::C0(h4709, _, _, _, _, _, _, _) => {
                        h4709
                    }
                }, false)?;
                let v5: u64 = {
                    let a4710 = v3;
                    let a4711 = v4;
                    let a4712 = 0u64;
                    nat_quot(a4710, a4711, a4712)
                };
                let v6: u64 = {
                    let a4713 = v3;
                    let a4714 = v4;
                    let a4715 = 0u64;
                    nat_rem(a4713, a4714, a4715)
                };
                let a4718 = {
                    let a4716 = v6;
                    let a4717 = 1u64;
                    nat_add(a4716, a4717)?
                };
                let a4719 = v4;
                if nat_eq(a4718, a4719) {
                    Ok(Adt9::C4({
                        let a4720 = v2;
                        let a4721 = 0u64;
                        if nat_eq(a4720, a4721) {
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
                    let a4722 = v3;
                    let a4723 = 2u64;
                    let a4724 = 0u64;
                    nat_quot(a4722, a4723, a4724)
                }, {
                    let a4725 = v3;
                    let a4726 = 2u64;
                    let a4727 = 0u64;
                    nat_rem(a4725, a4726, a4727)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f209(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4728 = v0;
    let a4729 = 0u64;
    if nat_eq(a4728, a4729) {
        Ok(f219(1u64))
    } else {
        f235({
            let a4738 = {
                let a4731 = v1;
                let a4732 = match v2.clone() {
                    Adt3::C0(h4730, _, _, _, _, _, _, _) => {
                        h4730
                    }
                };
                nat_mul(a4731, a4732)?
            };
            let a4739 = {
                let a4735 = {
                    let a4733 = v0;
                    let a4734 = 1u64;
                    nat_sub(a4733, a4734)
                };
                let a4736 = 2u64;
                let a4737 = 0u64;
                nat_quot(a4735, a4736, a4737)
            };
            nat_add(a4738, a4739)?
        }, {
            let a4742 = {
                let a4740 = v0;
                let a4741 = 1u64;
                nat_sub(a4740, a4741)
            };
            let a4743 = 2u64;
            let a4744 = 0u64;
            nat_rem(a4742, a4743, a4744)
        }, v2, v3, 1u64)
    }
}

pub fn f210(v0: u64, v1: u64) -> R<u64> {
    let a4745 = v0;
    let a4746 = f94(v1)?;
    let a4747 = 0u64;
    Ok(nat_rem(a4745, a4746, a4747))
}

pub fn f211(v0: u64, v1: u64) -> R<u64> {
    let a4748 = 4u64;
    let a4749 = f236(v0, v1)?;
    nat_mul(a4748, a4749)
}

pub fn f212(v0: u64, v1: u64) -> R<u64> {
    let a4750 = 4u64;
    let a4751 = f237(v0, v1)?;
    nat_mul(a4750, a4751)
}

pub fn f213(v0: u64, v1: u64) -> R<bool> {
    let a4757 = {
        let a4754 = {
            let a4752 = f210(v0, v1)?;
            let a4753 = 2u64;
            nat_sub(a4752, a4753)
        };
        let a4755 = 2u64;
        let a4756 = 0u64;
        nat_rem(a4754, a4755, a4756)
    };
    let a4758 = 0u64;
    Ok(nat_eq(a4757, a4758))
}

pub fn f214(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4759 = f114(v0);
    match m4759 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f112(v0) {
                let a4760 = v1;
                let a4761 = v7;
                nat_lt(a4760, a4761)
            } else {
                false
            } {
                f161(v8, v2)
            } else {
                false
            } {
                let a4762 = v3;
                let a4763 = f238(v4, v5, v6)?;
                nat_lt(a4762, a4763)
            } else {
                false
            } {
                let a4768 = {
                    let a4765 = v7;
                    let a4766 = match v13.clone() {
                        Adt18::C0(_, h4764, _) => {
                            h4764
                        }
                    };
                    let a4767 = 0u64;
                    nat_rem(a4765, a4766, a4767)
                };
                let a4769 = 0u64;
                nat_eq(a4768, a4769)
            } else {
                false
            } {
                Ok(Some({
                    let a4798 = {
                        let a4788 = {
                            let a4778 = {
                                let a4774 = {
                                    let a4771 = v1;
                                    let a4772 = match v13 {
                                        Adt18::C0(_, h4770, _) => {
                                            h4770
                                        }
                                    };
                                    let a4773 = 0u64;
                                    nat_quot(a4771, a4772, a4773)
                                };
                                let a4775 = v9;
                                nat_mul(a4774, a4775)?
                            };
                            let a4779 = {
                                let a4776 = v2;
                                let a4777 = v10;
                                nat_mul(a4776, a4777)?
                            };
                            nat_add(a4778, a4779)?
                        };
                        let a4789 = {
                            let a4786 = {
                                let a4783 = {
                                    let a4780 = v3;
                                    let a4781 = v4;
                                    let a4782 = 0u64;
                                    nat_quot(a4780, a4781, a4782)
                                };
                                let a4784 = v5;
                                let a4785 = 0u64;
                                nat_rem(a4783, a4784, a4785)
                            };
                            let a4787 = v11;
                            nat_mul(a4786, a4787)?
                        };
                        nat_add(a4788, a4789)?
                    };
                    let a4799 = {
                        let a4796 = {
                            let a4793 = {
                                let a4790 = v3;
                                let a4791 = v4;
                                let a4792 = 0u64;
                                nat_quot(a4790, a4791, a4792)
                            };
                            let a4794 = v5;
                            let a4795 = 0u64;
                            nat_quot(a4793, a4794, a4795)
                        };
                        let a4797 = v12;
                        nat_mul(a4796, a4797)?
                    };
                    nat_add(a4798, a4799)?
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
    let a4800 = v0;
    let a4801 = 64u64;
    if nat_eq(a4800, a4801) {
        f241(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4802 = v0;
        let a4803 = 65u64;
        if nat_eq(a4802, a4803) {
            f242(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f218(v0: u64) -> u64 {
    let a4804 = v0;
    let a4805 = 30u64;
    if nat_eq(a4804, a4805) {
        4u64
    } else {
        3u64
    }
}

pub fn f219(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f220(v0: u64) -> bool {
    let a4806 = 0u64;
    let a4807 = v0;
    if nat_lt(a4806, a4807) {
        let a4808 = v0;
        let a4809 = 2139095040u64;
        nat_le(a4808, a4809)
    } else {
        false
    }
}

pub fn f221(v0: u64) -> u64 {
    let a4810 = v0;
    let a4811 = 40u64;
    if nat_lt(a4810, a4811) {
        let a4812 = v0;
        let a4813 = 20u64;
        if nat_lt(a4812, a4813) {
            let a4814 = v0;
            let a4815 = 10u64;
            if nat_lt(a4814, a4815) {
                let a4816 = v0;
                let a4817 = 5u64;
                if nat_lt(a4816, a4817) {
                    let a4818 = v0;
                    let a4819 = 2u64;
                    if nat_lt(a4818, a4819) {
                        let a4820 = v0;
                        let a4821 = 1u64;
                        if nat_lt(a4820, a4821) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4822 = v0;
                        let a4823 = 3u64;
                        if nat_lt(a4822, a4823) {
                            4u64
                        } else {
                            let a4824 = v0;
                            let a4825 = 4u64;
                            if nat_lt(a4824, a4825) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4826 = v0;
                    let a4827 = 7u64;
                    if nat_lt(a4826, a4827) {
                        let a4828 = v0;
                        let a4829 = 6u64;
                        if nat_lt(a4828, a4829) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4830 = v0;
                        let a4831 = 8u64;
                        if nat_lt(a4830, a4831) {
                            128u64
                        } else {
                            let a4832 = v0;
                            let a4833 = 9u64;
                            if nat_lt(a4832, a4833) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4834 = v0;
                let a4835 = 15u64;
                if nat_lt(a4834, a4835) {
                    let a4836 = v0;
                    let a4837 = 12u64;
                    if nat_lt(a4836, a4837) {
                        let a4838 = v0;
                        let a4839 = 11u64;
                        if nat_lt(a4838, a4839) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4840 = v0;
                        let a4841 = 13u64;
                        if nat_lt(a4840, a4841) {
                            4096u64
                        } else {
                            let a4842 = v0;
                            let a4843 = 14u64;
                            if nat_lt(a4842, a4843) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4844 = v0;
                    let a4845 = 17u64;
                    if nat_lt(a4844, a4845) {
                        let a4846 = v0;
                        let a4847 = 16u64;
                        if nat_lt(a4846, a4847) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4848 = v0;
                        let a4849 = 18u64;
                        if nat_lt(a4848, a4849) {
                            131072u64
                        } else {
                            let a4850 = v0;
                            let a4851 = 19u64;
                            if nat_lt(a4850, a4851) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4852 = v0;
            let a4853 = 30u64;
            if nat_lt(a4852, a4853) {
                let a4854 = v0;
                let a4855 = 25u64;
                if nat_lt(a4854, a4855) {
                    let a4856 = v0;
                    let a4857 = 22u64;
                    if nat_lt(a4856, a4857) {
                        let a4858 = v0;
                        let a4859 = 21u64;
                        if nat_lt(a4858, a4859) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4860 = v0;
                        let a4861 = 23u64;
                        if nat_lt(a4860, a4861) {
                            4194304u64
                        } else {
                            let a4862 = v0;
                            let a4863 = 24u64;
                            if nat_lt(a4862, a4863) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4864 = v0;
                    let a4865 = 27u64;
                    if nat_lt(a4864, a4865) {
                        let a4866 = v0;
                        let a4867 = 26u64;
                        if nat_lt(a4866, a4867) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4868 = v0;
                        let a4869 = 28u64;
                        if nat_lt(a4868, a4869) {
                            134217728u64
                        } else {
                            let a4870 = v0;
                            let a4871 = 29u64;
                            if nat_lt(a4870, a4871) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4872 = v0;
                let a4873 = 35u64;
                if nat_lt(a4872, a4873) {
                    let a4874 = v0;
                    let a4875 = 32u64;
                    if nat_lt(a4874, a4875) {
                        let a4876 = v0;
                        let a4877 = 31u64;
                        if nat_lt(a4876, a4877) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4878 = v0;
                        let a4879 = 33u64;
                        if nat_lt(a4878, a4879) {
                            4294967296u64
                        } else {
                            let a4880 = v0;
                            let a4881 = 34u64;
                            if nat_lt(a4880, a4881) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4882 = v0;
                    let a4883 = 37u64;
                    if nat_lt(a4882, a4883) {
                        let a4884 = v0;
                        let a4885 = 36u64;
                        if nat_lt(a4884, a4885) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4886 = v0;
                        let a4887 = 38u64;
                        if nat_lt(a4886, a4887) {
                            137438953472u64
                        } else {
                            let a4888 = v0;
                            let a4889 = 39u64;
                            if nat_lt(a4888, a4889) {
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
    let a4890 = v0;
    let a4891 = 1u64;
    if nat_le(a4890, a4891) {
        true
    } else {
        let a4895 = {
            let a4892 = v1;
            let a4893 = 4u64;
            let a4894 = 0u64;
            nat_rem(a4892, a4893, a4894)
        };
        let a4896 = 0u64;
        nat_eq(a4895, a4896)
    }
}

pub fn f223(v0: u64, v1: u64) -> bool {
    let a4897 = v0;
    let a4898 = 1u64;
    if nat_le(a4897, a4898) {
        true
    } else {
        let a4899 = 4u64;
        let a4900 = v1;
        nat_le(a4899, a4900)
    }
}

pub fn f224(v0: u64, v1: u64) -> R<u64> {
    let a4901 = v0;
    let a4902 = 0u64;
    if nat_eq(a4901, a4902) {
        Ok(0u64)
    } else {
        let a4907 = 4u64;
        let a4908 = {
            let a4905 = {
                let a4903 = v0;
                let a4904 = 1u64;
                nat_sub(a4903, a4904)
            };
            let a4906 = v1;
            nat_mul(a4905, a4906)?
        };
        nat_add(a4907, a4908)
    }
}

pub fn f225(v0: bool, v1: u64) -> R<u64> {
    let a4913 = {
        let a4911 = {
            let a4909 = f243(v0, v1)?;
            let a4910 = f244(v0, v1)?;
            nat_add(a4909, a4910)?
        };
        let a4912 = f245(v0);
        nat_add(a4911, a4912)?
    };
    let a4914 = v1;
    nat_add(a4913, a4914)
}

pub fn f226(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f246(v0, v1)?;
    let v3: u64 = {
        let a4919 = {
            let a4917 = {
                let a4915 = v1;
                let a4916 = 1u64;
                nat_sub(a4915, a4916)
            };
            let a4918 = v2;
            nat_mul(a4917, a4918)?
        };
        let a4920 = v0;
        nat_sub(a4919, a4920)
    };
    let a4921 = v2;
    let a4922 = v3;
    if nat_lt(a4921, a4922) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f227(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4923 = v4;
    let a4924 = 0u64;
    if nat_eq(a4923, a4924) {
        Ok(0u64)
    } else {
        let a4931 = 2u64;
        let a4932 = {
            let a4929 = {
                let a4925 = v0;
                let a4926 = v1;
                nat_mul(a4925, a4926)?
            };
            let a4930 = {
                let a4927 = v2;
                let a4928 = v3;
                nat_mul(a4927, a4928)?
            };
            nat_mul(a4929, a4930)?
        };
        nat_mul(a4931, a4932)
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4937 = v3;
    let a4938 = {
        let a4935 = {
            let a4933 = f247(v2, v4)?;
            let a4934 = f248(v0, v1, f246(v2, v4)?)?;
            nat_mul(a4933, a4934)?
        };
        let a4936 = f248(v0, v1, f249(v2, v4)?)?;
        nat_add(a4935, a4936)?
    };
    nat_mul(a4937, a4938)
}

pub fn f229(v0: u64) -> R<u64> {
    let a4941 = 13u64;
    let a4942 = {
        let a4939 = 9u64;
        let a4940 = v0;
        nat_mul(a4939, a4940)?
    };
    nat_add(a4941, a4942)
}

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f192(v1, v2, v3, v4) {
        f250(v0)
    } else {
        f112(v0)
    }
}

pub fn f231(v0: u64) -> u64 {
    let a4943 = v0;
    let a4944 = 26u64;
    if nat_eq(a4943, a4944) {
        26u64
    } else {
        0u64
    }
}

pub fn f232(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4965 = {
        let a4959 = {
            let a4953 = {
                let a4947 = v1;
                let a4948 = if v5 {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4945, _, _, _) => {
                            h4945
                        }
                    })
                } else {
                    f133(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4946, _, _, _) => {
                            h4946
                        }
                    })
                };
                nat_mul(a4947, a4948)?
            };
            let a4954 = {
                let a4951 = v2;
                let a4952 = if v5 {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4949, _, _) => {
                            h4949
                        }
                    })
                } else {
                    f133(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4950, _, _) => {
                            h4950
                        }
                    })
                };
                nat_mul(a4951, a4952)?
            };
            nat_add(a4953, a4954)?
        };
        let a4960 = {
            let a4957 = v3;
            let a4958 = if v5 {
                f132(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4955, _) => {
                        h4955
                    }
                })
            } else {
                f133(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4956, _) => {
                        h4956
                    }
                })
            };
            nat_mul(a4957, a4958)?
        };
        nat_add(a4959, a4960)?
    };
    let a4966 = {
        let a4963 = v4;
        let a4964 = if v5 {
            f132(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4961) => {
                    h4961
                }
            })
        } else {
            f133(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4962) => {
                    h4962
                }
            })
        };
        nat_mul(a4963, a4964)?
    };
    nat_add(a4965, a4966)
}

pub fn f233(v0: u64, v1: u64) -> Option<u64> {
    let a4967 = v0;
    let a4968 = 2u64;
    if nat_eq(a4967, a4968) {
        Some(0u64)
    } else {
        let a4969 = v0;
        let a4970 = 5u64;
        if nat_eq(a4969, a4970) {
            Some(1u64)
        } else {
            let a4971 = v0;
            let a4972 = 6u64;
            if nat_eq(a4971, a4972) {
                Some(2u64)
            } else {
                let a4973 = v0;
                let a4974 = 7u64;
                if nat_eq(a4973, a4974) {
                    Some(3u64)
                } else {
                    let a4975 = v0;
                    let a4976 = 9u64;
                    if nat_eq(a4975, a4976) {
                        Some(4u64)
                    } else {
                        let a4977 = v0;
                        let a4978 = 8u64;
                        if nat_eq(a4977, a4978) {
                            Some(5u64)
                        } else {
                            let a4979 = v0;
                            let a4980 = 89u64;
                            if nat_eq(a4979, a4980) {
                                let a4981 = v1;
                                let a4982 = 2u64;
                                if nat_eq(a4981, a4982) {
                                    Some(6u64)
                                } else {
                                    let a4983 = v1;
                                    let a4984 = 0u64;
                                    if nat_eq(a4983, a4984) {
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
    let a4985 = v0;
    let a4986 = 30u64;
    if nat_eq(a4985, a4986) {
        let a4987 = v2;
        let a4988 = 0u64;
        if if if nat_eq(a4987, a4988) {
            true
        } else {
            let a4989 = v2;
            let a4990 = 1u64;
            nat_eq(a4989, a4990)
        } {
            let a4991 = v1;
            let a4992 = 0u64;
            if nat_eq(a4991, a4992) {
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
            let a4993 = v1;
            let a4994 = 0u64;
            if if nat_eq(a4993, a4994) {
                true
            } else {
                let a4995 = v2;
                let a4996 = 0u64;
                nat_eq(a4995, a4996)
            } {
                true
            } else {
                let a4997 = v1;
                let a4998 = v2;
                nat_eq(a4997, a4998)
            }
        } else {
            false
        } {
            true
        } else {
            let a4999 = v1;
            let a5000 = 43u64;
            if nat_eq(a4999, a5000) {
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
    let a5001 = v1;
    let a5002 = 0u64;
    if nat_eq(a5001, a5002) {
        Ok(Adt9::C4({
            let a5003 = v3;
            let a5004 = 0u64;
            if nat_eq(a5003, a5004) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f139(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5005 = v3;
            let a5006 = 0u64;
            if nat_eq(a5005, a5006) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f236(v0: u64, v1: u64) -> R<u64> {
    let a5007 = v0;
    let a5008 = f94(v1)?;
    let a5009 = 0u64;
    Ok(nat_quot(a5007, a5008, a5009))
}

pub fn f237(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f236(v0, v1)?;
    let v3: u64 = {
        let a5012 = {
            let a5010 = f210(v0, v1)?;
            let a5011 = 2u64;
            nat_sub(a5010, a5011)
        };
        let a5013 = 2u64;
        let a5014 = 0u64;
        nat_quot(a5012, a5013, a5014)
    };
    let v4: u64 = {
        let a5015 = v2;
        let a5016 = v1;
        nat_mul(a5015, a5016)?
    };
    if f213(v0, v1)? {
        let a5017 = v4;
        let a5018 = v3;
        nat_add(a5017, a5018)
    } else {
        let a5030 = {
            let a5026 = v4;
            let a5027 = {
                let a5024 = {
                    let a5019 = v1;
                    let a5020 = 1u64;
                    nat_sub(a5019, a5020)
                };
                let a5025 = {
                    let a5021 = v2;
                    let a5022 = v1;
                    let a5023 = 0u64;
                    nat_rem(a5021, a5022, a5023)
                };
                nat_mul(a5024, a5025)?
            };
            nat_sub(a5026, a5027)
        };
        let a5031 = {
            let a5028 = v3;
            let a5029 = v1;
            nat_mul(a5028, a5029)?
        };
        nat_add(a5030, a5031)
    }
}

pub fn f238(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5032 = v0;
    let a5033 = 0u64;
    if if if nat_eq(a5032, a5033) {
        true
    } else {
        let a5034 = v1;
        let a5035 = 0u64;
        nat_eq(a5034, a5035)
    } {
        true
    } else {
        let a5036 = v2;
        let a5037 = 0u64;
        nat_eq(a5036, a5037)
    } {
        Ok(0u64)
    } else {
        let a5040 = {
            let a5038 = v0;
            let a5039 = v1;
            nat_mul(a5038, a5039)?
        };
        let a5041 = v2;
        nat_mul(a5040, a5041)
    }
}

pub fn f239(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f103(match v1.clone() {
        Adt20::C0(h5042, _, _, _, _, _, _, _) => {
            h5042
        }
    })?;
    let v6: u64 = {
        let a5048 = match v1.clone() {
            Adt20::C0(_, h5043, _, _, _, _, _, _) => {
                h5043
            }
        };
        let a5049 = {
            let a5046 = match v1.clone() {
                Adt20::C0(_, _, h5044, _, _, _, _, _) => {
                    h5044
                }
            };
            let a5047 = match v1.clone() {
                Adt20::C0(_, _, _, h5045, _, _, _, _) => {
                    h5045
                }
            };
            nat_mul(a5046, a5047)?
        };
        nat_mul(a5048, a5049)?
    };
    let a5051 = v0;
    let a5052 = f39(match v1.clone() {
        Adt20::C0(h5050, _, _, _, _, _, _, _) => {
            h5050
        }
    }, v6)?;
    if nat_lt(a5051, a5052) {
        f254({
            let a5053 = v0;
            let a5054 = v5;
            let a5055 = 0u64;
            nat_rem(a5053, a5054, a5055)
        }, {
            let a5056 = v0;
            let a5057 = v5;
            let a5058 = 0u64;
            nat_quot(a5056, a5057, a5058)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f240(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5059 = v6;
        let a5060 = 6u64;
        nat_sub(a5059, a5060)
    };
    let v9: Adt20 = f255(v3);
    let a5062 = v0;
    let a5063 = f188(match v1.clone() {
        Adt20::C0(h5061, _, _, _, _, _, _, _) => {
            h5061
        }
    }, v5, v8)?;
    if nat_lt(a5062, a5063) {
        let v10: u64 = f100(match v1.clone() {
            Adt20::C0(h5064, _, _, _, _, _, _, _) => {
                h5064
            }
        }, v5, v8)?;
        let a5065 = v0;
        let a5066 = v10;
        if nat_lt(a5065, a5066) {
            f256(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5067 = v0;
                let a5068 = v10;
                nat_sub(a5067, a5068)
            };
            let v12: u64 = f229(match v1.clone() {
                Adt20::C0(h5069, _, _, _, _, _, _, _) => {
                    h5069
                }
            })?;
            f257({
                let a5070 = v11;
                let a5071 = v12;
                let a5072 = 0u64;
                nat_rem(a5070, a5071, a5072)
            }, {
                let a5073 = v11;
                let a5074 = v12;
                let a5075 = 0u64;
                nat_quot(a5073, a5074, a5075)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f241(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5076 = v5;
        let a5077 = 6u64;
        nat_sub(a5076, a5077)
    };
    let v8: bool = f182(match v1.clone() {
        Adt20::C0(_, _, _, _, h5078, _, _, _) => {
            h5078
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5079, _, _, _) => {
            h5079
        }
    });
    let a5081 = v0;
    let a5082 = f181(v8, match v1.clone() {
        Adt20::C0(h5080, _, _, _, _, _, _, _) => {
            h5080
        }
    }, v4, v7)?;
    if nat_lt(a5081, a5082) {
        let v9: u64 = f100(match v1.clone() {
            Adt20::C0(h5083, _, _, _, _, _, _, _) => {
                h5083
            }
        }, v4, v7)?;
        let a5084 = v0;
        let a5085 = v9;
        if nat_lt(a5084, a5085) {
            f256(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5086 = v0;
                let a5087 = v9;
                nat_sub(a5086, a5087)
            };
            let v11: u64 = f225(v8, match v1.clone() {
                Adt20::C0(h5088, _, _, _, _, _, _, _) => {
                    h5088
                }
            })?;
            f258({
                let a5089 = v10;
                let a5090 = v11;
                let a5091 = 0u64;
                nat_rem(a5089, a5090, a5091)
            }, {
                let a5092 = v10;
                let a5093 = v11;
                let a5094 = 0u64;
                nat_quot(a5092, a5093, a5094)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f242(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5095 = v5;
        let a5096 = 6u64;
        nat_sub(a5095, a5096)
    };
    let v8: Adt20 = f255(v2);
    let v9: u64 = f227(match v1.clone() {
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
    }, v7)?;
    let a5105 = v0;
    let a5106 = f187(match v1.clone() {
        Adt20::C0(h5101, _, _, _, _, _, _, _) => {
            h5101
        }
    }, match v1.clone() {
        Adt20::C0(_, h5102, _, _, _, _, _, _) => {
            h5102
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5103, _, _, _, _, _) => {
            h5103
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5104, _, _, _, _) => {
            h5104
        }
    }, v4, v7)?;
    if nat_lt(a5105, a5106) {
        let a5107 = v0;
        let a5108 = v9;
        if nat_lt(a5107, a5108) {
            f256(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f259({
                let a5109 = v0;
                let a5110 = v9;
                nat_sub(a5109, a5110)
            }, match v1.clone() {
                Adt20::C0(h5111, _, _, _, _, _, _, _) => {
                    h5111
                }
            }, match v1.clone() {
                Adt20::C0(_, h5112, _, _, _, _, _, _) => {
                    h5112
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5113, _, _, _, _, _) => {
                    h5113
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5114, _, _, _, _) => {
                    h5114
                }
            }, v4)?;
            f260(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f243(v0: bool, v1: u64) -> R<u64> {
    let a5117 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5118 = {
        let a5115 = f261(v0);
        let a5116 = v1;
        nat_mul(a5115, a5116)?
    };
    nat_add(a5117, a5118)
}

pub fn f244(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5126 = {
            let a5119 = 13u64;
            let a5120 = f262(v1);
            nat_mul(a5119, a5120)?
        };
        let a5127 = {
            let a5124 = 4u64;
            let a5125 = {
                let a5121 = v1;
                let a5122 = 4u64;
                let a5123 = 0u64;
                nat_rem(a5121, a5122, a5123)
            };
            nat_mul(a5124, a5125)?
        };
        nat_add(a5126, a5127)
    } else {
        let a5128 = 3u64;
        let a5129 = v1;
        nat_mul(a5128, a5129)
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
    let a5134 = {
        let a5132 = {
            let a5130 = v0;
            let a5131 = v1;
            nat_add(a5130, a5131)?
        };
        let a5133 = 1u64;
        nat_sub(a5132, a5133)
    };
    let a5135 = v1;
    let a5136 = 0u64;
    Ok(nat_quot(a5134, a5135, a5136))
}

pub fn f247(v0: u64, v1: u64) -> R<u64> {
    let a5137 = v0;
    let a5138 = f246(v0, v1)?;
    let a5139 = 0u64;
    Ok(nat_quot(a5137, a5138, a5139))
}

pub fn f248(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5140 = v2;
    let a5141 = 0u64;
    if nat_eq(a5140, a5141) {
        Ok(0u64)
    } else {
        let a5146 = 11u64;
        let a5147 = {
            let a5144 = {
                let a5142 = v1;
                let a5143 = v2;
                nat_mul(a5142, a5143)?
            };
            let a5145 = f263(v0)?;
            nat_mul(a5144, a5145)?
        };
        nat_add(a5146, a5147)
    }
}

pub fn f249(v0: u64, v1: u64) -> R<u64> {
    let a5148 = v0;
    let a5149 = f246(v0, v1)?;
    let a5150 = 0u64;
    Ok(nat_rem(a5148, a5149, a5150))
}

pub fn f250(v0: u64) -> bool {
    let m5151 = f114(v0);
    match m5151 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f251(v0: u64, v1: u64) -> bool {
    let a5152 = v0;
    let a5153 = 1u64;
    if nat_eq(a5152, a5153) {
        true
    } else {
        let v2: u64 = f264(v0);
        let a5154 = 0u64;
        let a5155 = v2;
        if nat_lt(a5154, a5155) {
            let a5159 = {
                let a5156 = v1;
                let a5157 = v2;
                let a5158 = 1u64;
                nat_rem(a5156, a5157, a5158)
            };
            let a5160 = 0u64;
            nat_eq(a5159, a5160)
        } else {
            false
        }
    }
}

pub fn f252(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5161 = v0;
    let a5162 = 28u64;
    if if if if nat_eq(a5161, a5162) {
        let a5163 = v1;
        let a5164 = 0u64;
        nat_eq(a5163, a5164)
    } else {
        false
    } {
        let a5165 = v2;
        let a5166 = 0u64;
        nat_eq(a5165, a5166)
    } else {
        false
    } {
        let a5167 = v3;
        let a5168 = 0u64;
        nat_eq(a5167, a5168)
    } else {
        false
    } {
        let a5169 = v4;
        let a5170 = v8;
        if if if if if if if if nat_eq(a5169, a5170) {
            let a5171 = v5;
            let a5172 = v12;
            nat_eq(a5171, a5172)
        } else {
            false
        } {
            let a5173 = v9;
            let a5174 = v13;
            nat_eq(a5173, a5174)
        } else {
            false
        } {
            let a5175 = v10;
            let a5176 = v14;
            nat_eq(a5175, a5176)
        } else {
            false
        } {
            let a5177 = v11;
            let a5178 = v15;
            nat_eq(a5177, a5178)
        } else {
            false
        } {
            let a5179 = 0u64;
            let a5180 = v6;
            nat_lt(a5179, a5180)
        } else {
            false
        } {
            let a5181 = 0u64;
            let a5182 = v7;
            nat_lt(a5181, a5182)
        } else {
            false
        } {
            let a5186 = {
                let a5183 = v10;
                let a5184 = v6;
                let a5185 = 1u64;
                nat_rem(a5183, a5184, a5185)
            };
            let a5187 = 0u64;
            if nat_eq(a5186, a5187) {
                let a5191 = {
                    let a5188 = v11;
                    let a5189 = v7;
                    let a5190 = 1u64;
                    nat_rem(a5188, a5189, a5190)
                };
                let a5192 = 0u64;
                nat_eq(a5191, a5192)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5193 = v10;
                let a5194 = v6;
                let a5195 = 0u64;
                nat_quot(a5193, a5194, a5195)
            }, {
                let a5196 = v11;
                let a5197 = v7;
                let a5198 = 0u64;
                nat_quot(a5196, a5197, a5198)
            }, {
                let a5199 = v5;
                let a5200 = v4;
                nat_mul(a5199, a5200)?
            }, {
                let a5201 = v4;
                let a5202 = v9;
                nat_mul(a5201, a5202)?
            }, {
                let a5203 = v5;
                let a5204 = v9;
                nat_mul(a5203, a5204)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f253(v0: u64) -> bool {
    let a5205 = v0;
    let a5206 = 0u64;
    if if nat_eq(a5205, a5206) {
        true
    } else {
        let a5207 = v0;
        let a5208 = 1u64;
        nat_eq(a5207, a5208)
    } {
        true
    } else {
        let a5209 = v0;
        let a5210 = 30u64;
        nat_eq(a5209, a5210)
    }
}

pub fn f254(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5211, _, _, _, _, _, _, _) => {
            h5211
        }
    };
    let v7: u64 = {
        let a5214 = 1u64;
        let a5215 = {
            let a5212 = 3u64;
            let a5213 = v6;
            nat_mul(a5212, a5213)?
        };
        nat_add(a5214, a5215)?
    };
    let a5216 = v0;
    let a5217 = 0u64;
    if nat_eq(a5216, a5217) {
        Ok(f265(0u64))
    } else {
        let a5218 = v0;
        let a5219 = v7;
        if nat_lt(a5218, a5219) {
            let v8: u64 = {
                let a5220 = v0;
                let a5221 = 1u64;
                nat_sub(a5220, a5221)
            };
            let v9: u64 = {
                let a5227 = {
                    let a5222 = v1;
                    let a5223 = v6;
                    nat_mul(a5222, a5223)?
                };
                let a5228 = {
                    let a5224 = v8;
                    let a5225 = 3u64;
                    let a5226 = 0u64;
                    nat_quot(a5224, a5225, a5226)
                };
                nat_add(a5227, a5228)?
            };
            let v10: u64 = f266(v9, v2)?;
            let v11: u64 = {
                let a5229 = v8;
                let a5230 = 3u64;
                let a5231 = 0u64;
                nat_rem(a5229, a5230, a5231)
            };
            let a5232 = v11;
            let a5233 = 0u64;
            if nat_eq(a5232, a5233) {
                Ok(f267(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5234 = v11;
                let a5235 = 1u64;
                if nat_eq(a5234, a5235) {
                    Ok(f267(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5236 = v0;
            let a5237 = v7;
            if nat_eq(a5236, a5237) {
                Ok(f267(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5240 = v0;
                let a5241 = {
                    let a5238 = v7;
                    let a5239 = 1u64;
                    nat_add(a5238, a5239)?
                };
                if nat_eq(a5240, a5241) {
                    Ok(f267(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5244 = v0;
                    let a5245 = {
                        let a5242 = v7;
                        let a5243 = 2u64;
                        nat_add(a5242, a5243)?
                    };
                    if nat_eq(a5244, a5245) {
                        Ok(f267(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5248 = v0;
                        let a5249 = {
                            let a5246 = v7;
                            let a5247 = 3u64;
                            nat_add(a5246, a5247)?
                        };
                        if nat_eq(a5248, a5249) {
                            let a5250 = v5;
                            let a5251 = 1u64;
                            if if nat_eq(a5250, a5251) {
                                true
                            } else {
                                let a5252 = v5;
                                let a5253 = 2u64;
                                nat_eq(a5252, a5253)
                            } {
                                Ok(f267(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f267(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5260 = {
                                    let a5254 = v1;
                                    let a5255 = v6;
                                    nat_mul(a5254, a5255)?
                                };
                                let a5261 = {
                                    let a5258 = v0;
                                    let a5259 = {
                                        let a5256 = v7;
                                        let a5257 = 4u64;
                                        nat_add(a5256, a5257)?
                                    };
                                    nat_sub(a5258, a5259)
                                };
                                nat_add(a5260, a5261)?
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
        Adt20::C0(h5262, _, _, _, _, _, _, _) => {
            h5262
        }
    }, match v0.clone() {
        Adt20::C0(_, h5263, _, _, _, _, _, _) => {
            h5263
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5264, _, _, _, _, _) => {
            h5264
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5265, _, _, _, _) => {
            h5265
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5266, _, _) => {
            h5266
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5267, _) => {
            h5267
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5268) => {
            h5268
        }
    })
}

pub fn f256(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5275 = v3;
        let a5276 = {
            let a5272 = f266({
                let a5269 = v0;
                let a5270 = 2u64;
                let a5271 = 0u64;
                nat_quot(a5269, a5270, a5271)
            }, v1)?;
            let a5273 = v2;
            let a5274 = 0u64;
            nat_quot(a5272, a5273, a5274)
        };
        nat_add(a5275, a5276)?
    };
    let a5280 = {
        let a5277 = v0;
        let a5278 = 2u64;
        let a5279 = 0u64;
        nat_rem(a5277, a5278, a5279)
    };
    let a5281 = 0u64;
    if nat_eq(a5280, a5281) {
        Ok(f267(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5282 = v4;
        let a5283 = 0u64;
        if nat_eq(a5282, a5283) {
            Ok(f267(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f257(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f268(match v2.clone() {
        Adt20::C0(h5284, _, _, _, _, _, _, _) => {
            h5284
        }
    })?;
    let v7: u64 = f269(match v2.clone() {
        Adt20::C0(h5285, _, _, _, _, _, _, _) => {
            h5285
        }
    })?;
    let a5286 = v0;
    let a5287 = 2u64;
    if nat_lt(a5286, a5287) {
        Ok(f265(v0))
    } else {
        let a5288 = v0;
        let a5289 = v6;
        if nat_lt(a5288, a5289) {
            f270({
                let a5290 = v0;
                let a5291 = 2u64;
                nat_sub(a5290, a5291)
            }, v1, v2, v3)
        } else {
            let a5292 = v0;
            let a5293 = v7;
            if nat_lt(a5292, a5293) {
                Ok(f271({
                    let a5294 = v0;
                    let a5295 = v6;
                    nat_sub(a5294, a5295)
                }, match v2 {
                    Adt20::C0(h5296, _, _, _, _, _, _, _) => {
                        h5296
                    }
                }, v5))
            } else {
                let a5298 = v0;
                let a5299 = f229(match v2.clone() {
                    Adt20::C0(h5297, _, _, _, _, _, _, _) => {
                        h5297
                    }
                })?;
                if nat_lt(a5298, a5299) {
                    f272({
                        let a5300 = v0;
                        let a5301 = v7;
                        nat_sub(a5300, a5301)
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
        Adt20::C0(h5302, _, _, _, _, _, _, _) => {
            h5302
        }
    })?;
    let v7: u64 = {
        let a5304 = v6;
        let a5305 = f244(v5, match v2.clone() {
            Adt20::C0(h5303, _, _, _, _, _, _, _) => {
                h5303
            }
        })?;
        nat_add(a5304, a5305)?
    };
    let v8: u64 = {
        let a5306 = v7;
        let a5307 = f245(v5);
        nat_add(a5306, a5307)?
    };
    let a5308 = v0;
    let a5309 = v6;
    if nat_lt(a5308, a5309) {
        f273(v0, v1, v2, v5)
    } else {
        let a5310 = v0;
        let a5311 = v7;
        if nat_lt(a5310, a5311) {
            f274({
                let a5312 = v0;
                let a5313 = v6;
                nat_sub(a5312, a5313)
            }, v1, v2, v3, v5)
        } else {
            let a5314 = v0;
            let a5315 = v8;
            if nat_lt(a5314, a5315) {
                Ok(f275({
                    let a5316 = v0;
                    let a5317 = v7;
                    nat_sub(a5316, a5317)
                }, match v2 {
                    Adt20::C0(h5318, _, _, _, _, _, _, _) => {
                        h5318
                    }
                }, v4, v5))
            } else {
                let a5322 = v0;
                let a5323 = {
                    let a5320 = v8;
                    let a5321 = match v2.clone() {
                        Adt20::C0(h5319, _, _, _, _, _, _, _) => {
                            h5319
                        }
                    };
                    nat_add(a5320, a5321)?
                };
                if nat_lt(a5322, a5323) {
                    let v9: u64 = f266({
                        let a5329 = {
                            let a5325 = v1;
                            let a5326 = match v2 {
                                Adt20::C0(h5324, _, _, _, _, _, _, _) => {
                                    h5324
                                }
                            };
                            nat_mul(a5325, a5326)?
                        };
                        let a5330 = {
                            let a5327 = v0;
                            let a5328 = v8;
                            nat_sub(a5327, a5328)
                        };
                        nat_add(a5329, a5330)?
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
        let a5333 = {
            let a5331 = f247(v3, v5)?;
            let a5332 = v4;
            nat_mul(a5331, a5332)?
        };
        let a5334 = v7;
        nat_mul(a5333, a5334)?
    };
    let a5335 = v0;
    let a5336 = v8;
    if nat_lt(a5335, a5336) {
        let v9: u64 = {
            let a5337 = v0;
            let a5338 = v7;
            let a5339 = 0u64;
            nat_quot(a5337, a5338, a5339)
        };
        Ok(Adt22::C0({
            let a5343 = {
                let a5340 = v9;
                let a5341 = v4;
                let a5342 = 0u64;
                nat_quot(a5340, a5341, a5342)
            };
            let a5344 = v6;
            nat_mul(a5343, a5344)?
        }, v6, {
            let a5345 = v9;
            let a5346 = v4;
            let a5347 = 0u64;
            nat_rem(a5345, a5346, a5347)
        }, {
            let a5348 = v0;
            let a5349 = v7;
            let a5350 = 0u64;
            nat_rem(a5348, a5349, a5350)
        }))
    } else {
        let v10: u64 = {
            let a5351 = v0;
            let a5352 = v8;
            nat_sub(a5351, a5352)
        };
        let v11: u64 = f248(v1, v2, f249(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5353 = f247(v3, v5)?;
            let a5354 = v6;
            nat_mul(a5353, a5354)?
        }, f249(v3, v5)?, {
            let a5355 = v10;
            let a5356 = v11;
            let a5357 = 0u64;
            nat_quot(a5355, a5356, a5357)
        }, {
            let a5358 = v10;
            let a5359 = v11;
            let a5360 = 0u64;
            nat_rem(a5358, a5359, a5360)
        }))
    }
}

pub fn f260(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5361) => {
            h5361
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5362, _, _, _, _, _, _, _) => {
            h5362
        }
    };
    let v6: u64 = {
        let a5365 = match v0.clone() {
            Adt22::C0(_, h5363, _, _) => {
                h5363
            }
        };
        let a5366 = match v1.clone() {
            Adt20::C0(_, h5364, _, _, _, _, _, _) => {
                h5364
            }
        };
        nat_mul(a5365, a5366)?
    };
    let v7: u64 = {
        let a5367 = v6;
        let a5368 = v5;
        nat_mul(a5367, a5368)?
    };
    let v8: u64 = {
        let a5371 = {
            let a5369 = 2u64;
            let a5370 = v5;
            nat_mul(a5369, a5370)?
        };
        let a5372 = 2u64;
        nat_add(a5371, a5372)?
    };
    let v9: u64 = {
        let a5375 = {
            let a5373 = 4u64;
            let a5374 = v5;
            nat_mul(a5373, a5374)?
        };
        let a5376 = 2u64;
        nat_add(a5375, a5376)?
    };
    let v10: u64 = {
        let a5379 = 1u64;
        let a5380 = {
            let a5377 = v6;
            let a5378 = v8;
            nat_mul(a5377, a5378)?
        };
        nat_add(a5379, a5380)?
    };
    let v11: u64 = {
        let a5381 = v10;
        let a5382 = 4u64;
        nat_add(a5381, a5382)?
    };
    let v12: u64 = {
        let a5385 = v11;
        let a5386 = {
            let a5383 = v6;
            let a5384 = v9;
            nat_mul(a5383, a5384)?
        };
        nat_add(a5385, a5386)?
    };
    let v13: u64 = {
        let a5387 = v12;
        let a5388 = 6u64;
        nat_add(a5387, a5388)?
    };
    let a5389 = v4;
    let a5390 = 0u64;
    if nat_eq(a5389, a5390) {
        Ok(f265(0u64))
    } else {
        let a5391 = v4;
        let a5392 = v10;
        if nat_lt(a5391, a5392) {
            let v14: u64 = {
                let a5393 = v4;
                let a5394 = 1u64;
                nat_sub(a5393, a5394)
            };
            f276(false, {
                let a5395 = v14;
                let a5396 = v8;
                let a5397 = 0u64;
                nat_rem(a5395, a5396, a5397)
            }, {
                let a5398 = v14;
                let a5399 = v8;
                let a5400 = 0u64;
                nat_quot(a5398, a5399, a5400)
            }, v0, v1, v2)
        } else {
            let a5401 = v4;
            let a5402 = v11;
            if nat_lt(a5401, a5402) {
                Ok(f277({
                    let a5403 = v4;
                    let a5404 = v10;
                    nat_sub(a5403, a5404)
                }, v7))
            } else {
                let a5405 = v4;
                let a5406 = v12;
                if nat_lt(a5405, a5406) {
                    let v15: u64 = {
                        let a5407 = v4;
                        let a5408 = v11;
                        nat_sub(a5407, a5408)
                    };
                    f276(true, {
                        let a5409 = v15;
                        let a5410 = v9;
                        let a5411 = 0u64;
                        nat_rem(a5409, a5410, a5411)
                    }, {
                        let a5412 = v15;
                        let a5413 = v9;
                        let a5414 = 0u64;
                        nat_quot(a5412, a5413, a5414)
                    }, v0, v1, v2)
                } else {
                    let a5415 = v4;
                    let a5416 = v13;
                    if nat_lt(a5415, a5416) {
                        Ok(f278({
                            let a5417 = v4;
                            let a5418 = v12;
                            nat_sub(a5417, a5418)
                        }, v7, v3))
                    } else {
                        let a5421 = v4;
                        let a5422 = {
                            let a5419 = v13;
                            let a5420 = v7;
                            nat_add(a5419, a5420)?
                        };
                        if nat_lt(a5421, a5422) {
                            let v16: u64 = {
                                let a5423 = v4;
                                let a5424 = v13;
                                nat_sub(a5423, a5424)
                            };
                            let v17: u64 = {
                                let a5425 = v16;
                                let a5426 = v5;
                                let a5427 = 0u64;
                                nat_quot(a5425, a5426, a5427)
                            };
                            let v18: u64 = {
                                let a5433 = {
                                    let a5428 = f279(v17, v0, v1)?;
                                    let a5429 = v5;
                                    nat_mul(a5428, a5429)?
                                };
                                let a5434 = {
                                    let a5430 = v16;
                                    let a5431 = v5;
                                    let a5432 = 0u64;
                                    nat_rem(a5430, a5431, a5432)
                                };
                                nat_add(a5433, a5434)?
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
    let a5435 = v0;
    let a5436 = 4u64;
    let a5437 = 0u64;
    nat_quot(a5435, a5436, a5437)
}

pub fn f263(v0: u64) -> R<u64> {
    let a5440 = {
        let a5438 = 7u64;
        let a5439 = v0;
        nat_mul(a5438, a5439)?
    };
    let a5441 = 4u64;
    nat_add(a5440, a5441)
}

pub fn f264(v0: u64) -> u64 {
    if f112(v0) {
        let m5442 = f114(v0);
        match m5442 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5444 = 1u64;
                let a5445 = match v1.clone() {
                    Adt18::C0(_, h5443, _) => {
                        h5443
                    }
                };
                if if nat_lt(a5444, a5445) {
                    true
                } else {
                    let a5446 = v0;
                    let a5447 = 43u64;
                    nat_eq(a5446, a5447)
                } {
                    match v1 {
                        Adt18::C0(_, h5448, _) => {
                            h5448
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
        let a5450 = v0;
        let a5451 = match v1.clone() {
            Adt20::C0(h5449, _, _, _, _, _, _, _) => {
                h5449
            }
        };
        let a5452 = 0u64;
        nat_quot(a5450, a5451, a5452)
    };
    let v3: u64 = {
        let a5454 = v2;
        let a5455 = match v1.clone() {
            Adt20::C0(_, h5453, _, _, _, _, _, _) => {
                h5453
            }
        };
        let a5456 = 0u64;
        nat_quot(a5454, a5455, a5456)
    };
    f131({
        let a5458 = v0;
        let a5459 = match v1.clone() {
            Adt20::C0(h5457, _, _, _, _, _, _, _) => {
                h5457
            }
        };
        let a5460 = 0u64;
        nat_rem(a5458, a5459, a5460)
    }, {
        let a5462 = v2;
        let a5463 = match v1.clone() {
            Adt20::C0(_, h5461, _, _, _, _, _, _) => {
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
        nat_rem(a5466, a5467, a5468)
    }, {
        let a5470 = v3;
        let a5471 = match v1.clone() {
            Adt20::C0(_, _, h5469, _, _, _, _, _) => {
                h5469
            }
        };
        let a5472 = 0u64;
        nat_quot(a5470, a5471, a5472)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5473, _, _, _) => {
            h5473
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5474, _, _) => {
            h5474
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5475, _) => {
            h5475
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5476) => {
            h5476
        }
    })
}

pub fn f267(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5477 = v0;
    let a5478 = 4u64;
    if if if nat_le(a5477, a5478) {
        true
    } else {
        let a5479 = 9u64;
        let a5480 = v0;
        if nat_le(a5479, a5480) {
            let a5481 = v0;
            let a5482 = 12u64;
            nat_le(a5481, a5482)
        } else {
            false
        }
    } {
        true
    } else {
        let a5483 = v0;
        let a5484 = 16u64;
        if if nat_eq(a5483, a5484) {
            true
        } else {
            let a5485 = v0;
            let a5486 = 19u64;
            nat_eq(a5485, a5486)
        } {
            true
        } else {
            let a5487 = v0;
            let a5488 = 21u64;
            if nat_eq(a5487, a5488) {
                true
            } else {
                let a5489 = v0;
                let a5490 = 22u64;
                nat_eq(a5489, a5490)
            }
        }
    } {
        Adt19::C1({
            let a5491 = v0;
            let a5492 = 4u64;
            if if nat_lt(a5491, a5492) {
                true
            } else {
                let a5493 = 21u64;
                let a5494 = v0;
                nat_le(a5493, a5494)
            } {
                0u64
            } else {
                let a5495 = v0;
                let a5496 = 16u64;
                if if nat_eq(a5495, a5496) {
                    true
                } else {
                    let a5497 = v0;
                    let a5498 = 19u64;
                    nat_eq(a5497, a5498)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5499 = v0;
            let a5500 = 4u64;
            if nat_lt(a5499, a5500) {
                v0
            } else {
                let a5501 = v0;
                let a5502 = 4u64;
                if if nat_eq(a5501, a5502) {
                    true
                } else {
                    let a5503 = v0;
                    let a5504 = 16u64;
                    nat_eq(a5503, a5504)
                } {
                    0u64
                } else {
                    let a5505 = v0;
                    let a5506 = 19u64;
                    if nat_eq(a5505, a5506) {
                        3u64
                    } else {
                        let a5507 = 21u64;
                        let a5508 = v0;
                        if nat_le(a5507, a5508) {
                            let a5509 = v0;
                            let a5510 = 17u64;
                            nat_sub(a5509, a5510)
                        } else {
                            let a5511 = v0;
                            let a5512 = 8u64;
                            nat_sub(a5511, a5512)
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
    let a5515 = 2u64;
    let a5516 = {
        let a5513 = 6u64;
        let a5514 = v0;
        nat_mul(a5513, a5514)?
    };
    nat_add(a5515, a5516)
}

pub fn f269(v0: u64) -> R<u64> {
    let a5517 = f268(v0)?;
    let a5518 = 11u64;
    nat_add(a5517, a5518)
}

pub fn f270(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5519 = v0;
        let a5520 = 6u64;
        let a5521 = 0u64;
        nat_rem(a5519, a5520, a5521)
    };
    let v5: u64 = {
        let a5528 = {
            let a5523 = v1;
            let a5524 = match v2.clone() {
                Adt20::C0(h5522, _, _, _, _, _, _, _) => {
                    h5522
                }
            };
            nat_mul(a5523, a5524)?
        };
        let a5529 = {
            let a5525 = v0;
            let a5526 = 6u64;
            let a5527 = 0u64;
            nat_quot(a5525, a5526, a5527)
        };
        nat_add(a5528, a5529)?
    };
    let v6: u64 = f266(v5, v3)?;
    let v7: u64 = f266(v5, v2)?;
    let a5530 = v4;
    let a5531 = 0u64;
    if nat_eq(a5530, a5531) {
        Ok(f267(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5532 = v4;
        let a5533 = 1u64;
        if nat_eq(a5532, a5533) {
            Ok(f267(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5534 = v4;
            let a5535 = 2u64;
            if nat_eq(a5534, a5535) {
                Ok(f267(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5536 = v4;
                let a5537 = 3u64;
                if nat_eq(a5536, a5537) {
                    Ok(f267(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5538 = v4;
                    let a5539 = 4u64;
                    if nat_eq(a5538, a5539) {
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
    let a5540 = v0;
    let a5541 = 0u64;
    if nat_eq(a5540, a5541) {
        f267(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5542 = v0;
        let a5543 = 1u64;
        if nat_eq(a5542, a5543) {
            f267(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5544 = v0;
            let a5545 = 2u64;
            if nat_eq(a5544, a5545) {
                f267(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5546 = v0;
                let a5547 = 3u64;
                if nat_eq(a5546, a5547) {
                    f267(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5548 = v0;
                    let a5549 = 4u64;
                    if nat_eq(a5548, a5549) {
                        f267(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5550 = v0;
                        let a5551 = 5u64;
                        if nat_eq(a5550, a5551) {
                            f267(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5552 = v0;
                            let a5553 = 6u64;
                            if nat_eq(a5552, a5553) {
                                f267(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5554 = v0;
                                let a5555 = 7u64;
                                if nat_eq(a5554, a5555) {
                                    f267(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5556 = v0;
                                    let a5557 = 8u64;
                                    if nat_eq(a5556, a5557) {
                                        f267(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5558 = v0;
                                        let a5559 = 9u64;
                                        if nat_eq(a5558, a5559) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5560 = v0;
                                            let a5561 = 10u64;
                                            if nat_eq(a5560, a5561) {
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
        let a5562 = v0;
        let a5563 = 3u64;
        let a5564 = 0u64;
        nat_rem(a5562, a5563, a5564)
    };
    let v6: u64 = {
        let a5571 = {
            let a5566 = v1;
            let a5567 = match v2.clone() {
                Adt20::C0(h5565, _, _, _, _, _, _, _) => {
                    h5565
                }
            };
            nat_mul(a5566, a5567)?
        };
        let a5572 = {
            let a5568 = v0;
            let a5569 = 3u64;
            let a5570 = 0u64;
            nat_quot(a5568, a5569, a5570)
        };
        nat_add(a5571, a5572)?
    };
    let v7: u64 = f266(v6, v3)?;
    let v8: u64 = f266(v6, v2)?;
    let v9: u64 = f266(v6, v4)?;
    let a5573 = v5;
    let a5574 = 0u64;
    if nat_eq(a5573, a5574) {
        Ok(f267(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5575 = v5;
        let a5576 = 1u64;
        if nat_eq(a5575, a5576) {
            Ok(f267(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f267(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f273(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5577, _, _, _, _, _, _, _) => {
            h5577
        }
    };
    let v5: u64 = {
        let a5580 = 1u64;
        let a5581 = {
            let a5578 = v4;
            let a5579 = f261(v3);
            nat_mul(a5578, a5579)?
        };
        nat_add(a5580, a5581)?
    };
    let a5582 = v0;
    let a5583 = 0u64;
    if nat_eq(a5582, a5583) {
        Ok(f265(0u64))
    } else {
        let a5584 = v0;
        let a5585 = v5;
        if nat_lt(a5584, a5585) {
            let v6: u64 = {
                let a5586 = v0;
                let a5587 = 1u64;
                nat_sub(a5586, a5587)
            };
            let v7: u64 = f266({
                let a5593 = {
                    let a5588 = v1;
                    let a5589 = v4;
                    nat_mul(a5588, a5589)?
                };
                let a5594 = {
                    let a5590 = v6;
                    let a5591 = f261(v3);
                    let a5592 = 0u64;
                    nat_quot(a5590, a5591, a5592)
                };
                nat_add(a5593, a5594)?
            }, v2)?;
            if v3 {
                let a5598 = {
                    let a5595 = v6;
                    let a5596 = 2u64;
                    let a5597 = 0u64;
                    nat_rem(a5595, a5596, a5597)
                };
                let a5599 = 0u64;
                if nat_eq(a5598, a5599) {
                    Ok(f267(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f267(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5600 = v0;
                let a5601 = v5;
                nat_sub(a5600, a5601)
            };
            let a5602 = v8;
            let a5603 = 0u64;
            if nat_eq(a5602, a5603) {
                Ok(f267(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5604 = v8;
                    let a5605 = 1u64;
                    nat_eq(a5604, a5605)
                } else {
                    false
                } {
                    Ok(f267(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5606 = v8;
                    let a5607 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5606, a5607) {
                        Ok(f267(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5608 = v8;
                        let a5609 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5608, a5609) {
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
        let a5611 = 13u64;
        let a5612 = f262(match v2.clone() {
            Adt20::C0(h5610, _, _, _, _, _, _, _) => {
                h5610
            }
        });
        nat_mul(a5611, a5612)?
    } else {
        0u64
    };
    let a5613 = v0;
    let a5614 = v5;
    if nat_lt(a5613, a5614) {
        f280({
            let a5615 = v0;
            let a5616 = 13u64;
            let a5617 = 0u64;
            nat_rem(a5615, a5616, a5617)
        }, v1, {
            let a5618 = v0;
            let a5619 = 13u64;
            let a5620 = 0u64;
            nat_quot(a5618, a5619, a5620)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5621 = v0;
            let a5622 = v5;
            nat_sub(a5621, a5622)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f281({
            let a5623 = v6;
            let a5624 = v7;
            let a5625 = 0u64;
            nat_rem(a5623, a5624, a5625)
        }, {
            let a5635 = {
                let a5630 = {
                    let a5627 = v1;
                    let a5628 = match v2.clone() {
                        Adt20::C0(h5626, _, _, _, _, _, _, _) => {
                            h5626
                        }
                    };
                    nat_mul(a5627, a5628)?
                };
                let a5631 = if v4 {
                    f282(match v2.clone() {
                        Adt20::C0(h5629, _, _, _, _, _, _, _) => {
                            h5629
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5630, a5631)?
            };
            let a5636 = {
                let a5632 = v6;
                let a5633 = v7;
                let a5634 = 0u64;
                nat_quot(a5632, a5633, a5634)
            };
            nat_add(a5635, a5636)?
        }, v2, v3, v4)
    }
}

pub fn f275(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5637 = v0;
    let a5638 = 0u64;
    if nat_eq(a5637, a5638) {
        f267(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5639 = v0;
        let a5640 = 1u64;
        if nat_eq(a5639, a5640) {
            f267(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5641 = v0;
                let a5642 = 2u64;
                nat_eq(a5641, a5642)
            } else {
                false
            } {
                f267(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5643 = v0;
                let a5644 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5643, a5644) {
                    f267(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5645 = v0;
                    let a5646 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5645, a5646) {
                        f267(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5647 = v0;
                        let a5648 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5647, a5648) {
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
        let a5652 = 1u64;
        let a5653 = {
            let a5650 = match v4.clone() {
                Adt20::C0(h5649, _, _, _, _, _, _, _) => {
                    h5649
                }
            };
            let a5651 = v6;
            nat_mul(a5650, a5651)?
        };
        nat_add(a5652, a5653)?
    };
    let a5654 = v1;
    let a5655 = 0u64;
    if nat_eq(a5654, a5655) {
        Ok(f265(1u64))
    } else {
        let a5656 = v1;
        let a5657 = v7;
        if nat_lt(a5656, a5657) {
            let v8: u64 = {
                let a5658 = v1;
                let a5659 = 1u64;
                nat_sub(a5658, a5659)
            };
            let v9: u64 = {
                let a5666 = {
                    let a5661 = f279(v2, v3, v4.clone())?;
                    let a5662 = match v4.clone() {
                        Adt20::C0(h5660, _, _, _, _, _, _, _) => {
                            h5660
                        }
                    };
                    nat_mul(a5661, a5662)?
                };
                let a5667 = {
                    let a5663 = v8;
                    let a5664 = v6;
                    let a5665 = 0u64;
                    nat_quot(a5663, a5664, a5665)
                };
                nat_add(a5666, a5667)?
            };
            let v10: u64 = f266(v9, v4)?;
            let v11: u64 = f266(v9, v5)?;
            let v12: u64 = {
                let a5668 = v8;
                let a5669 = v6;
                let a5670 = 0u64;
                nat_rem(a5668, a5669, a5670)
            };
            if v0 {
                let a5671 = v12;
                let a5672 = 0u64;
                if nat_eq(a5671, a5672) {
                    Ok(f267(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5673 = v12;
                    let a5674 = 1u64;
                    if nat_eq(a5673, a5674) {
                        Ok(f267(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5675 = v12;
                        let a5676 = 2u64;
                        if nat_eq(a5675, a5676) {
                            Ok(f267(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f267(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5677 = v12;
                let a5678 = 0u64;
                if nat_eq(a5677, a5678) {
                    Ok(f267(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f267(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5679 = v1;
            let a5680 = v7;
            if nat_eq(a5679, a5680) {
                Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f277(v0: u64, v1: u64) -> Adt19 {
    let a5681 = v0;
    let a5682 = 0u64;
    if nat_eq(a5681, a5682) {
        f267(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5683 = v0;
        let a5684 = 1u64;
        if nat_eq(a5683, a5684) {
            f267(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5685 = v0;
            let a5686 = 2u64;
            if nat_eq(a5685, a5686) {
                f267(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5687 = v0;
                let a5688 = 3u64;
                if nat_eq(a5687, a5688) {
                    f265(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f278(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5689 = v0;
    let a5690 = 0u64;
    if nat_eq(a5689, a5690) {
        f267(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5691 = v0;
        let a5692 = 1u64;
        if nat_eq(a5691, a5692) {
            f267(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5693 = v0;
            let a5694 = 2u64;
            if nat_eq(a5693, a5694) {
                f267(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5695 = v0;
                let a5696 = 3u64;
                if nat_eq(a5695, a5696) {
                    f267(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5697 = v0;
                    let a5698 = 4u64;
                    if nat_eq(a5697, a5698) {
                        f267(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5699 = v0;
                        let a5700 = 5u64;
                        if nat_eq(a5699, a5700) {
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
    let a5711 = {
        let a5709 = {
            let a5706 = {
                let a5703 = match v1.clone() {
                    Adt22::C0(_, _, h5701, _) => {
                        h5701
                    }
                };
                let a5704 = match v2.clone() {
                    Adt20::C0(_, _, h5702, _, _, _, _, _) => {
                        h5702
                    }
                };
                nat_mul(a5703, a5704)?
            };
            let a5707 = match v1 {
                Adt22::C0(h5705, _, _, _) => {
                    h5705
                }
            };
            nat_add(a5706, a5707)?
        };
        let a5710 = match v2 {
            Adt20::C0(_, h5708, _, _, _, _, _, _) => {
                h5708
            }
        };
        nat_mul(a5709, a5710)?
    };
    let a5712 = v0;
    nat_add(a5711, a5712)
}

pub fn f280(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5720 = {
            let a5718 = {
                let a5714 = v1;
                let a5715 = match v3.clone() {
                    Adt20::C0(h5713, _, _, _, _, _, _, _) => {
                        h5713
                    }
                };
                nat_mul(a5714, a5715)?
            };
            let a5719 = {
                let a5716 = v2;
                let a5717 = 4u64;
                nat_mul(a5716, a5717)?
            };
            nat_add(a5718, a5719)?
        };
        let a5721 = f283(v0);
        nat_add(a5720, a5721)?
    };
    let v6: u64 = f266(v5, v3)?;
    let v7: u64 = f266(v5, v4)?;
    let a5722 = v0;
    let a5723 = 0u64;
    if if if if nat_eq(a5722, a5723) {
        true
    } else {
        let a5724 = v0;
        let a5725 = 2u64;
        nat_eq(a5724, a5725)
    } {
        true
    } else {
        let a5726 = v0;
        let a5727 = 5u64;
        nat_eq(a5726, a5727)
    } {
        true
    } else {
        let a5728 = v0;
        let a5729 = 7u64;
        nat_eq(a5728, a5729)
    } {
        Ok(f267(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5730 = v0;
        let a5731 = 1u64;
        if if if if nat_eq(a5730, a5731) {
            true
        } else {
            let a5732 = v0;
            let a5733 = 3u64;
            nat_eq(a5732, a5733)
        } {
            true
        } else {
            let a5734 = v0;
            let a5735 = 6u64;
            nat_eq(a5734, a5735)
        } {
            true
        } else {
            let a5736 = v0;
            let a5737 = 8u64;
            nat_eq(a5736, a5737)
        } {
            Ok(f267(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5738 = v0;
                let a5739 = 1u64;
                if nat_eq(a5738, a5739) {
                    4u64
                } else {
                    let a5740 = v0;
                    let a5741 = 6u64;
                    if nat_eq(a5740, a5741) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5742 = v0;
            let a5743 = 4u64;
            if nat_eq(a5742, a5743) {
                Ok(f267(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5744 = v0;
                let a5745 = 9u64;
                if nat_eq(a5744, a5745) {
                    Ok(f267(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5746 = v0;
                    let a5747 = 10u64;
                    if nat_eq(a5746, a5747) {
                        Ok(f267(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5748 = v0;
                        let a5749 = 11u64;
                        if nat_eq(a5748, a5749) {
                            Ok(f267(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5750 = v0;
                            let a5751 = 12u64;
                            if nat_eq(a5750, a5751) {
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
    let a5752 = v0;
    let a5753 = 0u64;
    if nat_eq(a5752, a5753) {
        Ok(f267(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5754 = v0;
        let a5755 = 1u64;
        if nat_eq(a5754, a5755) {
            Ok(f267(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5756 = v0;
                let a5757 = 2u64;
                if nat_eq(a5756, a5757) {
                    Ok(f267(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5758 = v0;
                    let a5759 = 3u64;
                    if nat_eq(a5758, a5759) {
                        Ok(f267(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5760 = v0;
                let a5761 = 2u64;
                if nat_eq(a5760, a5761) {
                    Ok(f267(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f282(v0: u64) -> R<u64> {
    let a5762 = 4u64;
    let a5763 = f262(v0);
    nat_mul(a5762, a5763)
}

pub fn f283(v0: u64) -> u64 {
    let a5764 = v0;
    let a5765 = 5u64;
    if nat_lt(a5764, a5765) {
        let a5766 = v0;
        let a5767 = 2u64;
        if nat_lt(a5766, a5767) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5768 = v0;
        let a5769 = 7u64;
        if nat_lt(a5768, a5769) {
            2u64
        } else {
            3u64
        }
    }
}
