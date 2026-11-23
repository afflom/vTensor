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
        let a2052 = 15u64;
        nat_eq(a2051, a2052)
    } else {
        false
    } {
        Ok(Some(22u64))
    } else {
        let a2053 = v0;
        let a2054 = 89u64;
        if if nat_eq(a2053, a2054) {
            let a2055 = 8u64;
            let a2056 = v1;
            if nat_le(a2055, a2056) {
                let a2057 = v1;
                let a2058 = 9u64;
                nat_le(a2057, a2058)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Some({
                let a2059 = v1;
                let a2060 = 12u64;
                nat_add(a2059, a2060)?
            }))
        } else {
            let a2061 = v0;
            let a2062 = 89u64;
            if if nat_eq(a2061, a2062) {
                let a2063 = v1;
                let a2064 = 4u64;
                nat_eq(a2063, a2064)
            } else {
                false
            } {
                Ok(Some(19u64))
            } else {
                let a2065 = 10u64;
                let a2066 = v0;
                if if nat_le(a2065, a2066) {
                    let a2067 = v0;
                    let a2068 = 12u64;
                    nat_le(a2067, a2068)
                } else {
                    false
                } {
                    Ok(Some({
                        let a2069 = v0;
                        let a2070 = 3u64;
                        nat_add(a2069, a2070)?
                    }))
                } else {
                    let a2071 = v0;
                    let a2072 = 89u64;
                    if if nat_eq(a2071, a2072) {
                        let a2073 = v1;
                        let a2074 = 7u64;
                        nat_eq(a2073, a2074)
                    } else {
                        false
                    } {
                        Ok(Some(17u64))
                    } else {
                        let a2075 = v0;
                        let a2076 = 89u64;
                        if if nat_eq(a2075, a2076) {
                            let a2077 = v1;
                            let a2078 = 10u64;
                            nat_eq(a2077, a2078)
                        } else {
                            false
                        } {
                            Ok(Some(18u64))
                        } else {
                            let a2079 = v0;
                            let a2080 = 89u64;
                            if if nat_eq(a2079, a2080) {
                                let a2081 = v1;
                                let a2082 = 13u64;
                                nat_eq(a2081, a2082)
                            } else {
                                false
                            } {
                                Ok(Some(16u64))
                            } else {
                                let a2083 = v0;
                                let a2084 = 8u64;
                                if nat_eq(a2083, a2084) {
                                    Ok(Some(5u64))
                                } else {
                                    let a2085 = v0;
                                    let a2086 = 9u64;
                                    if nat_eq(a2085, a2086) {
                                        Ok(Some(6u64))
                                    } else {
                                        let a2087 = v0;
                                        let a2088 = 89u64;
                                        if nat_eq(a2087, a2088) {
                                            let a2089 = v1;
                                            let a2090 = 0u64;
                                            if nat_eq(a2089, a2090) {
                                                Ok(Some(0u64))
                                            } else {
                                                let a2091 = v1;
                                                let a2092 = 1u64;
                                                if nat_eq(a2091, a2092) {
                                                    Ok(Some(1u64))
                                                } else {
                                                    let a2093 = v1;
                                                    let a2094 = 2u64;
                                                    if nat_eq(a2093, a2094) {
                                                        Ok(Some(2u64))
                                                    } else {
                                                        let a2095 = v1;
                                                        let a2096 = 3u64;
                                                        if nat_eq(a2095, a2096) {
                                                            Ok(Some(3u64))
                                                        } else {
                                                            let a2097 = v1;
                                                            let a2098 = 6u64;
                                                            if nat_eq(a2097, a2098) {
                                                                Ok(Some(4u64))
                                                            } else {
                                                                let a2099 = v1;
                                                                let a2100 = 12u64;
                                                                if nat_eq(a2099, a2100) {
                                                                    Ok(Some(7u64))
                                                                } else {
                                                                    let a2101 = v1;
                                                                    let a2102 = 11u64;
                                                                    if nat_eq(a2101, a2102) {
                                                                        Ok(Some(8u64))
                                                                    } else {
                                                                        let a2103 = v1;
                                                                        let a2104 = 18u64;
                                                                        if nat_eq(a2103, a2104) {
                                                                            Ok(Some(9u64))
                                                                        } else {
                                                                            let a2105 = v1;
                                                                            let a2106 = 19u64;
                                                                            if nat_eq(a2105, a2106) {
                                                                                Ok(Some(10u64))
                                                                            } else {
                                                                                let a2107 = v1;
                                                                                let a2108 = 20u64;
                                                                                if nat_eq(a2107, a2108) {
                                                                                    Ok(Some(11u64))
                                                                                } else {
                                                                                    let a2109 = v1;
                                                                                    let a2110 = 21u64;
                                                                                    if nat_eq(a2109, a2110) {
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

pub fn f67(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2111 = f129(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m2111 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a2112 = v0;
            let a2113 = 2u64;
            if if if if nat_eq(a2112, a2113) {
                true
            } else {
                let a2114 = v0;
                let a2115 = 5u64;
                nat_eq(a2114, a2115)
            } {
                true
            } else {
                let a2116 = v0;
                let a2117 = 6u64;
                nat_eq(a2116, a2117)
            } {
                true
            } else {
                let a2118 = v0;
                let a2119 = 7u64;
                nat_eq(a2118, a2119)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2120 = v3;
                    match m2120 {
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
    let a2122 = match v1.clone() {
        Adt2::C0(h2121, _, _, _, _, _, _, _, _, _, _) => {
            h2121
        }
    };
    let a2123 = 0u64;
    if if if if if nat_eq(a2122, a2123) {
        true
    } else {
        let a2125 = match v1.clone() {
            Adt2::C0(h2124, _, _, _, _, _, _, _, _, _, _) => {
                h2124
            }
        };
        let a2126 = 1u64;
        nat_eq(a2125, a2126)
    } {
        true
    } else {
        let a2128 = match v1.clone() {
            Adt2::C0(h2127, _, _, _, _, _, _, _, _, _, _) => {
                h2127
            }
        };
        let a2129 = 30u64;
        nat_eq(a2128, a2129)
    } {
        let a2132 = match v1.clone() {
            Adt2::C0(h2130, _, _, _, _, _, _, _, _, _, _) => {
                h2130
            }
        };
        let a2133 = match v2.clone() {
            Adt2::C0(h2131, _, _, _, _, _, _, _, _, _, _) => {
                h2131
            }
        };
        if nat_eq(a2132, a2133) {
            true
        } else {
            let a2135 = match v2.clone() {
                Adt2::C0(h2134, _, _, _, _, _, _, _, _, _, _) => {
                    h2134
                }
            };
            let a2136 = 0u64;
            nat_eq(a2135, a2136)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v2.clone(), 1u64)?, {
            let a2138 = match v2.clone() {
                Adt2::C0(h2137, _, _, _, _, _, _, _, _, _, _) => {
                    h2137
                }
            };
            let a2139 = 0u64;
            if nat_eq(a2138, a2139) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2140, _, _, _, _, _, _, _, _, _, _) => {
                h2140
            }
        }, match v2 {
            Adt2::C0(h2141, _, _, _, _, _, _, _, _, _, _) => {
                h2141
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f69(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f130(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2142, _, _, _) => {
            h2142
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2143, _, _) => {
            h2143
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2144, _) => {
            h2144
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2145) => {
            h2145
        }
    })
}

pub fn f70(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2147 = v0;
        let a2148 = match v1.clone() {
            Adt3::C0(h2146, _, _, _, _, _, _, _) => {
                h2146
            }
        };
        let a2149 = 0u64;
        nat_quot(a2147, a2148, a2149)
    };
    let v4: u64 = {
        let a2151 = v3;
        let a2152 = match v1.clone() {
            Adt3::C0(_, h2150, _, _, _, _, _, _) => {
                h2150
            }
        };
        let a2153 = 0u64;
        nat_quot(a2151, a2152, a2153)
    };
    let a2154 = v2;
    let a2155 = 0u64;
    if nat_eq(a2154, a2155) {
        let a2157 = v0;
        let a2158 = match v1 {
            Adt3::C0(h2156, _, _, _, _, _, _, _) => {
                h2156
            }
        };
        let a2159 = 0u64;
        nat_rem(a2157, a2158, a2159)
    } else {
        let a2160 = v2;
        let a2161 = 1u64;
        if nat_eq(a2160, a2161) {
            let a2163 = v3;
            let a2164 = match v1 {
                Adt3::C0(_, h2162, _, _, _, _, _, _) => {
                    h2162
                }
            };
            let a2165 = 0u64;
            nat_rem(a2163, a2164, a2165)
        } else {
            let a2166 = v2;
            let a2167 = 2u64;
            if nat_eq(a2166, a2167) {
                let a2169 = v4;
                let a2170 = match v1 {
                    Adt3::C0(_, _, h2168, _, _, _, _, _) => {
                        h2168
                    }
                };
                let a2171 = 0u64;
                nat_rem(a2169, a2170, a2171)
            } else {
                let a2173 = v4;
                let a2174 = match v1 {
                    Adt3::C0(_, _, h2172, _, _, _, _, _) => {
                        h2172
                    }
                };
                let a2175 = 0u64;
                nat_quot(a2173, a2174, a2175)
            }
        }
    }
}

pub fn f71(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2177 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2176) => {
            h2176
        }
    };
    let a2178 = 0u64;
    if nat_eq(a2177, a2178) {
        Ok(v2)
    } else {
        let a2183 = {
            let a2180 = v2;
            let a2181 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2179) => {
                    h2179
                }
            };
            nat_mul(a2180, a2181)?
        };
        let a2184 = f70(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2182, _, _, _, _, _, _) => {
                h2182
            }
        }, 3u64);
        nat_add(a2183, a2184)
    }
}

pub fn f72(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2194 = {
            let a2191 = {
                let a2186 = v2;
                let a2187 = match v0.clone() {
                    Adt6::C0(_, h2185, _, _) => {
                        h2185
                    }
                };
                nat_mul(a2186, a2187)?
            };
            let a2192 = {
                let a2189 = v3;
                let a2190 = match v0.clone() {
                    Adt6::C0(_, _, _, h2188) => {
                        h2188
                    }
                };
                nat_mul(a2189, a2190)?
            };
            nat_add(a2191, a2192)?
        };
        let a2195 = f131(match v0.clone() {
            Adt6::C0(_, _, h2193, _) => {
                h2193
            }
        });
        nat_add(a2194, a2195)?
    };
    let v5: u64 = f132(match v0 {
        Adt6::C0(_, _, h2196, _) => {
            h2196
        }
    });
    let a2197 = v5;
    let a2198 = v4;
    if if nat_le(a2197, a2198) {
        let a2201 = {
            let a2199 = v4;
            let a2200 = v5;
            nat_sub(a2199, a2200)
        };
        let a2202 = v1;
        nat_lt(a2201, a2202)
    } else {
        false
    } {
        Ok(Some({
            let a2203 = v4;
            let a2204 = v5;
            nat_sub(a2203, a2204)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f73(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2205, _, _, _, _, _) => {
            h2205
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2206, _, _, _, _, _, _) => {
            h2206
        }
    };
    let v7: u64 = f70(v1, v6.clone(), 2u64);
    let a2208 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2207) => {
            h2207
        }
    };
    let a2209 = 0u64;
    if nat_eq(a2208, a2209) {
        Ok(Adt9::C8(Some(f69(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2212 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2211, _, _, _, _) => {
                    h2211
                }
            };
            match m2212 {
                Adt3::C0(_, _, h2210, _, _, _, _, _) => {
                    h2210
                }
            }
        };
        let m2218 = f72(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2213, _) => {
                h2213
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2214, _, _, _, _, _) => {
                h2214
            }
        }, {
            let a2215 = v2;
            let a2216 = v8;
            let a2217 = 0u64;
            nat_rem(a2215, a2216, a2217)
        }, v7)?;
        match m2218 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2225 = {
                        let a2223 = {
                            let a2219 = v2;
                            let a2220 = v8;
                            let a2221 = 0u64;
                            nat_quot(a2219, a2220, a2221)
                        };
                        let a2224 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2222) => {
                                h2222
                            }
                        };
                        nat_mul(a2223, a2224)?
                    };
                    let a2226 = f70(v1, v6, 3u64);
                    nat_add(a2225, a2226)?
                };
                Ok(Adt9::C8(Some(f69(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f74(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2227 = v1;
    match m2227 {
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
            f133(f134(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f135(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f136(v0.clone(), v24);
            f133(f137(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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
    let m2228 = v3;
    match m2228 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f138(v0, v1)?, f138(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f140(v0, v1, v2.clone())?, f138(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f141(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f142(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f143(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f142(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f144(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f145(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f146(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f138(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f147(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v30, v31) => {
            f148(v0, v1, v2, v30, v31)
        }
        Adt4::C12(v32, v33, v34) => {
            f149(v0, v1, v2, v32, v33, v34)
        }
        Adt4::C13(_, v36, _, _) => {
            Ok(Adt9::C3(v36, v36, f138(v0, v1)?, f138(v0, v2)?))
        }
        Adt4::C14(_, v40, v41) => {
            Ok(Adt9::C3(v40, v41, f138(v0, v1)?, f138(v0, v2)?))
        }
        Adt4::C15 => {
            f150(v0, v1, v2)
        }
        Adt4::C16(v42, v43) => {
            f151(v0, v1, v2, v42, v43)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v44, v45) => {
            f152(v0, v1, v45, v44)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f138(f139(v0, match v2.clone() {
                Adt3::C0(h2229, _, _, _, _, _, _, _) => {
                    h2229
                }
            }, match v2.clone() {
                Adt3::C0(_, h2230, _, _, _, _, _, _) => {
                    h2230
                }
            })?, v1)?, f138(v0, v2)?))
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
            f153(v0, v50, v51, v52)
        }
        Adt4::C26(v58, _, _, _) => {
            let a2231 = v58;
            let a2232 = 64u64;
            if if if nat_eq(a2231, a2232) {
                true
            } else {
                let a2233 = v58;
                let a2234 = 65u64;
                nat_eq(a2233, a2234)
            } {
                true
            } else {
                let a2235 = v58;
                let a2236 = 66u64;
                nat_eq(a2235, a2236)
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
        let a2237 = v0;
        let a2238 = v5;
        let a2239 = 0u64;
        nat_quot(a2237, a2238, a2239)
    };
    let a2240 = v5;
    let a2241 = 1u64;
    if nat_eq(a2240, a2241) {
        Ok(f133(f154(v3, v4), v3, v4, 0u64, f138(v6, v1)?, 2u64, 0u64, 3u64, f138(v6, v2)?))
    } else {
        let a2245 = {
            let a2242 = v0;
            let a2243 = 2u64;
            let a2244 = 0u64;
            nat_rem(a2242, a2243, a2244)
        };
        let a2246 = 0u64;
        if nat_eq(a2245, a2246) {
            Ok(f133(f154(v3, 0u64), v3, 0u64, 0u64, f138(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f133(f154(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f138(v6, v2)?))
        }
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f126(match v1.clone() {
        Adt3::C0(h2247, _, _, _, _, _, _, _) => {
            h2247
        }
    })?;
    let v5: u64 = {
        let a2248 = v0;
        let a2249 = v4;
        let a2250 = 0u64;
        nat_quot(a2248, a2249, a2250)
    };
    let v6: u64 = {
        let a2251 = v0;
        let a2252 = v4;
        let a2253 = 0u64;
        nat_rem(a2251, a2252, a2253)
    };
    let a2254 = v6;
    let a2255 = 0u64;
    if nat_eq(a2254, a2255) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2256 = v6;
        let a2257 = 1u64;
        if nat_eq(a2256, a2257) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2260 = {
                let a2258 = v6;
                let a2259 = 1u64;
                nat_add(a2258, a2259)?
            };
            let a2261 = v4;
            if nat_eq(a2260, a2261) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f138(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2262 = v6;
                    let a2263 = 2u64;
                    nat_sub(a2262, a2263)
                };
                let v8: u64 = {
                    let a2264 = v7;
                    let a2265 = 3u64;
                    let a2266 = 0u64;
                    nat_quot(a2264, a2265, a2266)
                };
                let v9: u64 = {
                    let a2267 = v7;
                    let a2268 = 3u64;
                    let a2269 = 0u64;
                    nat_rem(a2267, a2268, a2269)
                };
                let v10: u64 = f138({
                    let a2273 = {
                        let a2271 = v5;
                        let a2272 = match v1.clone() {
                            Adt3::C0(h2270, _, _, _, _, _, _, _) => {
                                h2270
                            }
                        };
                        nat_mul(a2271, a2272)?
                    };
                    let a2274 = v8;
                    nat_add(a2273, a2274)?
                }, v1)?;
                let a2275 = v9;
                let a2276 = 0u64;
                if nat_eq(a2275, a2276) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2277 = v9;
                    let a2278 = 1u64;
                    if nat_eq(a2277, a2278) {
                        let a2279 = v3;
                        let a2280 = 2u64;
                        if nat_eq(a2279, a2280) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2281 = v3;
                        let a2282 = 2u64;
                        if if nat_eq(a2281, a2282) {
                            true
                        } else {
                            let a2283 = v3;
                            let a2284 = 3u64;
                            nat_eq(a2283, a2284)
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
        let a2286 = 2u64;
        let a2287 = match v2.clone() {
            Adt3::C0(_, h2285, _, _, _, _, _, _) => {
                h2285
            }
        };
        nat_mul(a2286, a2287)?
    };
    let a2288 = v1;
    let a2289 = v6;
    if nat_lt(a2288, a2289) {
        let a2293 = {
            let a2290 = v1;
            let a2291 = 2u64;
            let a2292 = 0u64;
            nat_rem(a2290, a2291, a2292)
        };
        let a2294 = 0u64;
        if nat_eq(a2293, a2294) {
            Ok(Adt9::C4(19u64, 4u64, f138({
                let a2295 = v1;
                let a2296 = 2u64;
                let a2297 = 0u64;
                nat_quot(a2295, a2296, a2297)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2298 = v5;
            let a2299 = 2147483648u64;
            if if nat_lt(a2298, a2299) {
                let a2301 = v5;
                let a2302 = match v3 {
                    Adt3::C0(_, h2300, _, _, _, _, _, _) => {
                        h2300
                    }
                };
                nat_lt(a2301, a2302)
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
            let a2305 = match v3.clone() {
                Adt3::C0(h2303, _, _, _, _, _, _, _) => {
                    h2303
                }
            };
            let a2306 = match v3.clone() {
                Adt3::C0(_, h2304, _, _, _, _, _, _) => {
                    h2304
                }
            };
            nat_mul(a2305, a2306)?
        };
        let a2309 = {
            let a2307 = v1;
            let a2308 = v6;
            nat_sub(a2307, a2308)
        };
        let a2310 = v7;
        if nat_lt(a2309, a2310) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f138({
                let a2311 = v1;
                let a2312 = v6;
                nat_sub(a2311, a2312)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2315 = {
                    let a2313 = v1;
                    let a2314 = v6;
                    nat_sub(a2313, a2314)
                };
                let a2316 = v7;
                nat_sub(a2315, a2316)
            };
            let v9: u64 = {
                let a2318 = 1u64;
                let a2319 = f155(v0, match v2.clone() {
                    Adt3::C0(h2317, _, _, _, _, _, _, _) => {
                        h2317
                    }
                })?;
                nat_add(a2318, a2319)?
            };
            let v10: u64 = {
                let a2320 = v8;
                let a2321 = v9;
                let a2322 = 0u64;
                nat_quot(a2320, a2321, a2322)
            };
            let v11: u64 = {
                let a2323 = v8;
                let a2324 = v9;
                let a2325 = 0u64;
                nat_rem(a2323, a2324, a2325)
            };
            let a2326 = v11;
            let a2327 = 0u64;
            if nat_eq(a2326, a2327) {
                Ok(Adt9::C4(19u64, 4u64, f138(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f104(v0) {
                    let a2331 = {
                        let a2328 = v11;
                        let a2329 = 2u64;
                        let a2330 = 0u64;
                        nat_rem(a2328, a2329, a2330)
                    };
                    let a2332 = 1u64;
                    if nat_eq(a2331, a2332) {
                        Ok(Adt9::C4(6u64, 0u64, f138({
                            let a2341 = {
                                let a2334 = v10;
                                let a2335 = match v2.clone() {
                                    Adt3::C0(h2333, _, _, _, _, _, _, _) => {
                                        h2333
                                    }
                                };
                                nat_mul(a2334, a2335)?
                            };
                            let a2342 = {
                                let a2338 = {
                                    let a2336 = v11;
                                    let a2337 = 1u64;
                                    nat_sub(a2336, a2337)
                                };
                                let a2339 = 2u64;
                                let a2340 = 0u64;
                                nat_quot(a2338, a2339, a2340)
                            };
                            nat_add(a2341, a2342)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f138({
                            let a2351 = {
                                let a2344 = v5;
                                let a2345 = match v2.clone() {
                                    Adt3::C0(h2343, _, _, _, _, _, _, _) => {
                                        h2343
                                    }
                                };
                                nat_mul(a2344, a2345)?
                            };
                            let a2352 = {
                                let a2348 = {
                                    let a2346 = v11;
                                    let a2347 = 1u64;
                                    nat_sub(a2346, a2347)
                                };
                                let a2349 = 2u64;
                                let a2350 = 0u64;
                                nat_quot(a2348, a2349, a2350)
                            };
                            nat_add(a2351, a2352)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f138({
                            let a2361 = {
                                let a2354 = v5;
                                let a2355 = match v2 {
                                    Adt3::C0(h2353, _, _, _, _, _, _, _) => {
                                        h2353
                                    }
                                };
                                nat_mul(a2354, a2355)?
                            };
                            let a2362 = {
                                let a2358 = {
                                    let a2356 = v11;
                                    let a2357 = 1u64;
                                    nat_sub(a2356, a2357)
                                };
                                let a2359 = 2u64;
                                let a2360 = 0u64;
                                nat_quot(a2358, a2359, a2360)
                            };
                            nat_add(a2361, a2362)?
                        }, v3)?))
                    }
                } else {
                    f156(v0, v10, {
                        let a2363 = v11;
                        let a2364 = 1u64;
                        nat_sub(a2363, a2364)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f79(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2365 = v1;
    let a2366 = f157(v2.clone(), v4.clone())?;
    if nat_lt(a2365, a2366) {
        let a2369 = v1;
        let a2370 = {
            let a2367 = 2u64;
            let a2368 = f127(v4.clone())?;
            nat_mul(a2367, a2368)?
        };
        if nat_lt(a2369, a2370) {
            let a2374 = {
                let a2371 = v1;
                let a2372 = 2u64;
                let a2373 = 0u64;
                nat_rem(a2371, a2372, a2373)
            };
            let a2375 = 0u64;
            if nat_eq(a2374, a2375) {
                f158({
                    let a2376 = v1;
                    let a2377 = 2u64;
                    let a2378 = 0u64;
                    nat_quot(a2376, a2377, a2378)
                }, v4)
            } else {
                if f159(match v2 {
                    Adt3::C0(_, h2379, _, _, _, _, _, _) => {
                        h2379
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2382 = v1;
                let a2383 = {
                    let a2380 = 2u64;
                    let a2381 = f127(v4.clone())?;
                    nat_mul(a2380, a2381)?
                };
                nat_sub(a2382, a2383)
            };
            let v7: u64 = {
                let a2387 = v6;
                let a2388 = {
                    let a2385 = 1u64;
                    let a2386 = match v2.clone() {
                        Adt3::C0(h2384, _, _, _, _, _, _, _) => {
                            h2384
                        }
                    };
                    nat_add(a2385, a2386)?
                };
                let a2389 = 0u64;
                nat_quot(a2387, a2388, a2389)
            };
            let v8: u64 = {
                let a2393 = v6;
                let a2394 = {
                    let a2391 = 1u64;
                    let a2392 = match v2.clone() {
                        Adt3::C0(h2390, _, _, _, _, _, _, _) => {
                            h2390
                        }
                    };
                    nat_add(a2391, a2392)?
                };
                let a2395 = 0u64;
                nat_rem(a2393, a2394, a2395)
            };
            let a2396 = v8;
            let a2397 = 0u64;
            if nat_eq(a2396, a2397) {
                f158(v7, v4)
            } else {
                f160(v0, v7, {
                    let a2398 = v8;
                    let a2399 = 1u64;
                    nat_sub(a2398, a2399)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2402 = match v4.clone() {
            Adt3::C0(h2400, _, _, _, _, _, _, _) => {
                h2400
            }
        };
        let a2403 = match v4.clone() {
            Adt3::C0(_, h2401, _, _, _, _, _, _) => {
                h2401
            }
        };
        nat_mul(a2402, a2403)?
    };
    let v7: u64 = {
        let a2404 = 2u64;
        let a2405 = v6;
        nat_mul(a2404, a2405)?
    };
    let v8: u64 = f161(match v3.clone() {
        Adt3::C0(_, h2406, _, _, _, _, _, _) => {
            h2406
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2407, _, _, _, _, _) => {
            h2407
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2408, _, _, _, _) => {
            h2408
        }
    });
    let a2409 = v0;
    let a2410 = v7;
    if nat_lt(a2409, a2410) {
        let a2414 = {
            let a2411 = v0;
            let a2412 = 2u64;
            let a2413 = 0u64;
            nat_rem(a2411, a2412, a2413)
        };
        let a2415 = 0u64;
        if nat_eq(a2414, a2415) {
            Ok(Adt9::C4(19u64, 5u64, f138({
                let a2416 = v0;
                let a2417 = 2u64;
                let a2418 = 0u64;
                nat_quot(a2416, a2417, a2418)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f162(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2419 = v0;
            let a2420 = v7;
            nat_sub(a2419, a2420)
        };
        let v10: u64 = {
            let a2424 = v9;
            let a2425 = {
                let a2422 = 1u64;
                let a2423 = match v1.clone() {
                    Adt3::C0(h2421, _, _, _, _, _, _, _) => {
                        h2421
                    }
                };
                nat_add(a2422, a2423)?
            };
            let a2426 = 0u64;
            nat_quot(a2424, a2425, a2426)
        };
        let v11: u64 = {
            let a2430 = v9;
            let a2431 = {
                let a2428 = 1u64;
                let a2429 = match v1.clone() {
                    Adt3::C0(h2427, _, _, _, _, _, _, _) => {
                        h2427
                    }
                };
                nat_add(a2428, a2429)?
            };
            let a2432 = 0u64;
            nat_rem(a2430, a2431, a2432)
        };
        let a2433 = v11;
        let a2434 = 0u64;
        if nat_eq(a2433, a2434) {
            Ok(Adt9::C4(19u64, 5u64, f138({
                let a2435 = v10;
                let a2436 = v6;
                let a2437 = 0u64;
                nat_rem(a2435, a2436, a2437)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f162(v5, v8) {
                let v12: u64 = {
                    let a2443 = {
                        let a2439 = v10;
                        let a2440 = match v1.clone() {
                            Adt3::C0(h2438, _, _, _, _, _, _, _) => {
                                h2438
                            }
                        };
                        nat_mul(a2439, a2440)?
                    };
                    let a2444 = {
                        let a2441 = v11;
                        let a2442 = 1u64;
                        nat_sub(a2441, a2442)
                    };
                    nat_add(a2443, a2444)?
                };
                let a2445 = v5;
                let a2446 = 4294967295u64;
                if nat_eq(a2445, a2446) {
                    Ok(Adt9::C1(false, f138(v12, v1)?, f138(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f138(v12, v1)?, 4u64, f138({
                        let a2452 = {
                            let a2448 = v5;
                            let a2449 = match v3.clone() {
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
                    }, v3)?, 3u64, f138(v12, v2)?))
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
        let a2454 = v0;
        let a2455 = v5;
        let a2456 = 0u64;
        nat_quot(a2454, a2455, a2456)
    };
    let v7: u64 = {
        let a2457 = v0;
        let a2458 = v5;
        let a2459 = 0u64;
        nat_rem(a2457, a2458, a2459)
    };
    let v8: u64 = f163(v3.clone())?;
    let a2460 = v7;
    let a2461 = 0u64;
    if nat_eq(a2460, a2461) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2462, _) => {
                h2462
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2466 = v5;
        let a2467 = {
            let a2464 = v7;
            let a2465 = if match v3.clone() {
                Adt7::C0(_, _, _, h2463, _) => {
                    h2463
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2464, a2465)?
        };
        if nat_le(a2466, a2467) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2468, _) => {
                    h2468
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f138(v6, v2)?))
            } else {
                let a2471 = {
                    let a2469 = v7;
                    let a2470 = 2u64;
                    nat_add(a2469, a2470)?
                };
                let a2472 = v5;
                if nat_eq(a2471, a2472) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2473, _, _) => {
                            h2473
                        }
                    } {
                        let a2480 = {
                            let m2476 = match v3.clone() {
                                Adt7::C0(h2475, _, _, _, _) => {
                                    h2475
                                }
                            };
                            match m2476 {
                                Adt6::C0(h2474, _, _, _) => {
                                    h2474
                                }
                            }
                        };
                        let a2481 = {
                            let m2479 = match v3 {
                                Adt7::C0(_, h2478, _, _, _) => {
                                    h2478
                                }
                            };
                            match m2479 {
                                Adt6::C0(h2477, _, _, _) => {
                                    h2477
                                }
                            }
                        };
                        nat_mul(a2480, a2481)?
                    } else {
                        f164(match v3 {
                            Adt7::C0(h2482, _, _, _, _) => {
                                h2482
                            }
                        }, match v1 {
                            Adt3::C0(h2483, _, _, _, _, _, _, _) => {
                                h2483
                            }
                        }, f70(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2484 = v9;
                        let a2485 = 0u64;
                        if nat_eq(a2484, a2485) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f138(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2488 = {
                    let a2486 = v7;
                    let a2487 = 1u64;
                    nat_sub(a2486, a2487)
                };
                let a2489 = v8;
                let a2490 = 0u64;
                nat_rem(a2488, a2489, a2490)
            };
            let m2496 = f165(v3.clone(), v1, v2, v6, {
                let a2493 = {
                    let a2491 = v7;
                    let a2492 = 1u64;
                    nat_sub(a2491, a2492)
                };
                let a2494 = v8;
                let a2495 = 0u64;
                nat_quot(a2493, a2494, a2495)
            })?;
            match m2496 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2498 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2497) => {
                            h2497
                        }
                    };
                    let a2499 = 1u64;
                    if if nat_eq(a2498, a2499) {
                        let a2500 = v10;
                        let a2501 = 0u64;
                        nat_eq(a2500, a2501)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2502, _) => {
                                h2502
                            }
                        } {
                            let a2505 = {
                                let a2503 = v10;
                                let a2504 = 1u64;
                                nat_add(a2503, a2504)?
                            };
                            let a2506 = v8;
                            if nat_eq(a2505, a2506) {
                                let a2507 = v4;
                                let a2508 = 2u64;
                                if nat_eq(a2507, a2508) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2510 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2509) => {
                                                h2509
                                            }
                                        };
                                        let a2511 = 1u64;
                                        if nat_eq(a2510, a2511) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2513 = match v3 {
                                            Adt7::C0(_, _, _, _, h2512) => {
                                                h2512
                                            }
                                        };
                                        let a2514 = 1u64;
                                        if nat_eq(a2513, a2514) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2516 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2515) => {
                                            h2515
                                        }
                                    };
                                    let a2517 = 1u64;
                                    if nat_eq(a2516, a2517) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2519 = match v3 {
                                        Adt7::C0(_, _, _, _, h2518) => {
                                            h2518
                                        }
                                    };
                                    let a2520 = 1u64;
                                    if nat_eq(a2519, a2520) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2522 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2521) => {
                                        h2521
                                    }
                                };
                                let a2523 = 1u64;
                                if nat_eq(a2522, a2523) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2525 = match v3 {
                                    Adt7::C0(_, _, _, _, h2524) => {
                                        h2524
                                    }
                                };
                                let a2526 = 1u64;
                                if nat_eq(a2525, a2526) {
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
        Adt3::C0(h2527, _, _, _, _, _, _, _) => {
            h2527
        }
    })?;
    let v5: u64 = {
        let a2528 = v0;
        let a2529 = v4;
        let a2530 = 0u64;
        nat_rem(a2528, a2529, a2530)
    };
    let v6: u64 = {
        let a2531 = v0;
        let a2532 = v4;
        let a2533 = 0u64;
        nat_quot(a2531, a2532, a2533)
    };
    let a2534 = v5;
    let a2535 = 0u64;
    if nat_eq(a2534, a2535) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2538 = {
            let a2536 = v5;
            let a2537 = 1u64;
            nat_add(a2536, a2537)?
        };
        let a2539 = v4;
        if nat_eq(a2538, a2539) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f138(v6, v2)?))
        } else {
            let a2543 = {
                let a2540 = v5;
                let a2541 = 2u64;
                let a2542 = 0u64;
                nat_rem(a2540, a2541, a2542)
            };
            let a2544 = 0u64;
            if nat_eq(a2543, a2544) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2547 = {
                        let a2545 = v5;
                        let a2546 = 1u64;
                        nat_sub(a2545, a2546)
                    };
                    let a2548 = 2u64;
                    let a2549 = 0u64;
                    nat_quot(a2547, a2548, a2549)
                };
                let v8: u64 = f70(v6, v2.clone(), 0u64);
                let v9: u64 = f70(v6, v2.clone(), 1u64);
                let v10: u64 = f70(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f69(v1, {
                    let a2550 = v9;
                    let a2551 = v7;
                    nat_add(a2550, a2551)?
                }, v8, v10, 0u64)?, 4u64, f69(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f106(v3.clone())?;
    let v5: u64 = {
        let a2552 = v0;
        let a2553 = v4;
        let a2554 = 0u64;
        nat_rem(a2552, a2553, a2554)
    };
    let v6: u64 = {
        let a2555 = v0;
        let a2556 = v4;
        let a2557 = 0u64;
        nat_quot(a2555, a2556, a2557)
    };
    let a2558 = v5;
    let a2559 = 0u64;
    if nat_eq(a2558, a2559) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2562 = {
            let a2560 = v5;
            let a2561 = 1u64;
            nat_add(a2560, a2561)?
        };
        let a2563 = v4;
        if nat_eq(a2562, a2563) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f138(v6, v2)?))
        } else {
            let a2567 = {
                let a2564 = v5;
                let a2565 = 2u64;
                let a2566 = 0u64;
                nat_rem(a2564, a2565, a2566)
            };
            let a2568 = 0u64;
            if nat_eq(a2567, a2568) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2570 = f70(v6, v2.clone(), 0u64);
                    let a2571 = match v3.clone() {
                        Adt6::C0(_, _, h2569, _) => {
                            h2569
                        }
                    };
                    nat_add(a2570, a2571)?
                };
                let v8: u64 = {
                    let a2579 = {
                        let a2576 = {
                            let a2573 = v7;
                            let a2574 = match v3.clone() {
                                Adt6::C0(_, h2572, _, _) => {
                                    h2572
                                }
                            };
                            nat_add(a2573, a2574)?
                        };
                        let a2577 = match v3.clone() {
                            Adt6::C0(h2575, _, _, _) => {
                                h2575
                            }
                        };
                        nat_sub(a2576, a2577)
                    };
                    let a2580 = match v3.clone() {
                        Adt6::C0(_, h2578, _, _) => {
                            h2578
                        }
                    };
                    let a2581 = 0u64;
                    nat_quot(a2579, a2580, a2581)
                };
                let v9: u64 = {
                    let a2587 = v8;
                    let a2588 = {
                        let a2584 = {
                            let a2582 = v5;
                            let a2583 = 1u64;
                            nat_sub(a2582, a2583)
                        };
                        let a2585 = 2u64;
                        let a2586 = 0u64;
                        nat_quot(a2584, a2585, a2586)
                    };
                    nat_add(a2587, a2588)?
                };
                let v10: u64 = {
                    let a2590 = v9;
                    let a2591 = match v3.clone() {
                        Adt6::C0(_, h2589, _, _) => {
                            h2589
                        }
                    };
                    nat_mul(a2590, a2591)?
                };
                let v11: u64 = {
                    let a2592 = v7;
                    let a2593 = v10;
                    nat_sub(a2592, a2593)
                };
                let a2595 = v9;
                let a2596 = match v1.clone() {
                    Adt3::C0(_, h2594, _, _, _, _, _, _) => {
                        h2594
                    }
                };
                if if if nat_lt(a2595, a2596) {
                    let a2597 = v10;
                    let a2598 = v7;
                    nat_le(a2597, a2598)
                } else {
                    false
                } {
                    let a2600 = v11;
                    let a2601 = match v3.clone() {
                        Adt6::C0(h2599, _, _, _) => {
                            h2599
                        }
                    };
                    nat_lt(a2600, a2601)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f69(v1, {
                        let a2605 = {
                            let a2603 = f70(v6, v2, 1u64);
                            let a2604 = match v3 {
                                Adt6::C0(h2602, _, _, _) => {
                                    h2602
                                }
                            };
                            nat_mul(a2603, a2604)?
                        };
                        let a2606 = v11;
                        nat_add(a2605, a2606)?
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
        let a2608 = v5;
        let a2609 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2607, _) => {
                h2607
            }
        };
        let a2610 = 0u64;
        nat_rem(a2608, a2609, a2610)
    };
    let v7: u64 = {
        let a2612 = v5;
        let a2613 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2611, _) => {
                h2611
            }
        };
        let a2614 = 0u64;
        nat_quot(a2612, a2613, a2614)
    };
    let v8: Option<u64> = f72(match v3.clone() {
        Adt5::C0(h2615, _, _, _, _, _, _, _) => {
            h2615
        }
    }, match v1.clone() {
        Adt3::C0(h2616, _, _, _, _, _, _, _) => {
            h2616
        }
    }, f70(v0, v2.clone(), 1u64), {
        let a2620 = v4;
        let a2621 = {
            let m2619 = match v3.clone() {
                Adt5::C0(h2618, _, _, _, _, _, _, _) => {
                    h2618
                }
            };
            match m2619 {
                Adt6::C0(h2617, _, _, _) => {
                    h2617
                }
            }
        };
        let a2622 = 0u64;
        nat_rem(a2620, a2621, a2622)
    })?;
    let v9: u64 = {
        let a2626 = v4;
        let a2627 = {
            let m2625 = match v3.clone() {
                Adt5::C0(h2624, _, _, _, _, _, _, _) => {
                    h2624
                }
            };
            match m2625 {
                Adt6::C0(h2623, _, _, _) => {
                    h2623
                }
            }
        };
        let a2628 = 0u64;
        nat_quot(a2626, a2627, a2628)
    };
    let v10: Option<u64> = f72(match v3.clone() {
        Adt5::C0(_, h2629, _, _, _, _, _, _) => {
            h2629
        }
    }, match v1.clone() {
        Adt3::C0(_, h2630, _, _, _, _, _, _) => {
            h2630
        }
    }, f70(v0, v2.clone(), 2u64), {
        let a2634 = v9;
        let a2635 = {
            let m2633 = match v3.clone() {
                Adt5::C0(_, h2632, _, _, _, _, _, _) => {
                    h2632
                }
            };
            match m2633 {
                Adt6::C0(h2631, _, _, _) => {
                    h2631
                }
            }
        };
        let a2636 = 0u64;
        nat_rem(a2634, a2635, a2636)
    })?;
    let v11: u64 = {
        let a2640 = v9;
        let a2641 = {
            let m2639 = match v3.clone() {
                Adt5::C0(_, h2638, _, _, _, _, _, _) => {
                    h2638
                }
            };
            match m2639 {
                Adt6::C0(h2637, _, _, _) => {
                    h2637
                }
            }
        };
        let a2642 = 0u64;
        nat_quot(a2640, a2641, a2642)
    };
    let v12: Option<u64> = f72(match v3.clone() {
        Adt5::C0(_, _, h2643, _, _, _, _, _) => {
            h2643
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2644, _, _, _, _, _) => {
            h2644
        }
    }, v6, {
        let a2648 = v11;
        let a2649 = {
            let m2647 = match v3.clone() {
                Adt5::C0(_, _, h2646, _, _, _, _, _) => {
                    h2646
                }
            };
            match m2647 {
                Adt6::C0(h2645, _, _, _) => {
                    h2645
                }
            }
        };
        let a2650 = 0u64;
        nat_rem(a2648, a2649, a2650)
    })?;
    let v13: u64 = {
        let a2654 = v11;
        let a2655 = {
            let m2653 = match v3.clone() {
                Adt5::C0(_, _, h2652, _, _, _, _, _) => {
                    h2652
                }
            };
            match m2653 {
                Adt6::C0(h2651, _, _, _) => {
                    h2651
                }
            }
        };
        let a2656 = 0u64;
        nat_quot(a2654, a2655, a2656)
    };
    let v17: Adt12 = {
        let m2657 = v8;
        match m2657 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2658 = v10;
                match m2658 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2659 = v12;
                        match m2659 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2668 = {
                                        let a2663 = f69(v1, v14, v15, v16, 0u64)?;
                                        let a2664 = {
                                            let a2661 = v7;
                                            let a2662 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2660, _, _, _) => {
                                                    h2660
                                                }
                                            };
                                            nat_mul(a2661, a2662)?
                                        };
                                        nat_add(a2663, a2664)?
                                    };
                                    let a2669 = {
                                        let a2666 = v13;
                                        let a2667 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2665, _, _, _, _) => {
                                                h2665
                                            }
                                        };
                                        nat_mul(a2666, a2667)?
                                    };
                                    nat_add(a2668, a2669)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2670, _, _) => {
            h2670
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2671) => {
            h2671
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f138(v0, v2)?), {
        let a2673 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2672, _, _) => {
                h2672
            }
        };
        let a2674 = 0u64;
        if nat_eq(a2673, a2674) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2676 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2675) => {
                h2675
            }
        };
        let a2677 = 0u64;
        if nat_eq(a2676, a2677) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f85(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2690 = f166(v0, v1, match v2.clone() {
        Adt3::C0(h2678, _, _, _, _, _, _, _) => {
            h2678
        }
    }, match v2.clone() {
        Adt3::C0(_, h2679, _, _, _, _, _, _) => {
            h2679
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2680, _, _, _, _, _) => {
            h2680
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2681, _, _, _, _) => {
            h2681
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2682, _, _, _) => {
            h2682
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2683, _, _) => {
            h2683
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2684, _) => {
            h2684
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2685) => {
            h2685
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2686, _, _, _) => {
            h2686
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2687, _, _) => {
            h2687
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2688, _) => {
            h2688
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2689) => {
            h2689
        }
    }, v4)?;
    match m2690 {
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
        let a2693 = v6;
        let a2694 = {
            let a2691 = 3u64;
            let a2692 = v5;
            nat_add(a2691, a2692)?
        };
        nat_sub(a2693, a2694)
    };
    let a2696 = v0;
    let a2697 = f36(v3, match v1.clone() {
        Adt3::C0(h2695, _, _, _, _, _, _, _) => {
            h2695
        }
    }, v5, v8)?;
    if nat_lt(a2696, a2697) {
        let v9: u64 = f100(match v1.clone() {
            Adt3::C0(h2698, _, _, _, _, _, _, _) => {
                h2698
            }
        }, v5, v8)?;
        let v10: u64 = f101(v3, match v1.clone() {
            Adt3::C0(h2699, _, _, _, _, _, _, _) => {
                h2699
            }
        }, v5)?;
        let a2700 = v0;
        let a2701 = v9;
        if nat_lt(a2700, a2701) {
            f167(v0, v2, f30(v3), {
                let a2702 = 3u64;
                let a2703 = v5;
                nat_add(a2702, a2703)?
            }, v7)
        } else {
            let a2706 = v0;
            let a2707 = {
                let a2704 = v9;
                let a2705 = v10;
                nat_add(a2704, a2705)?
            };
            if nat_lt(a2706, a2707) {
                let v11: u64 = {
                    let a2708 = v0;
                    let a2709 = v9;
                    nat_sub(a2708, a2709)
                };
                let v12: u64 = f168(v3, match v1.clone() {
                    Adt3::C0(h2710, _, _, _, _, _, _, _) => {
                        h2710
                    }
                })?;
                f169({
                    let a2711 = v11;
                    let a2712 = v12;
                    let a2713 = 0u64;
                    nat_rem(a2711, a2712, a2713)
                }, {
                    let a2714 = v11;
                    let a2715 = v12;
                    let a2716 = 0u64;
                    nat_quot(a2714, a2715, a2716)
                }, v1, v3, v4, v7)
            } else {
                f170({
                    let a2719 = v0;
                    let a2720 = {
                        let a2717 = v9;
                        let a2718 = v10;
                        nat_add(a2717, a2718)?
                    };
                    nat_sub(a2719, a2720)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2721 = v0;
    let a2722 = f19(v1)?;
    if nat_lt(a2721, a2722) {
        let a2723 = v0;
        let a2724 = 4u64;
        if nat_lt(a2723, a2724) {
            let a2725 = v0;
            let a2726 = 0u64;
            if nat_eq(a2725, a2726) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2727 = v0;
                let a2728 = 1u64;
                if nat_eq(a2727, a2728) {
                    let a2729 = v5;
                    let a2730 = 0u64;
                    if nat_eq(a2729, a2730) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2731 = v0;
                    let a2732 = 2u64;
                    if nat_eq(a2731, a2732) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2733 = v0;
            let a2734 = 4u64;
            if if nat_eq(a2733, a2734) {
                let a2735 = f171(v5)?;
                let a2736 = v1;
                !nat_eq(a2735, a2736)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2739 = {
                        let a2737 = v0;
                        let a2738 = 4u64;
                        nat_sub(a2737, a2738)
                    };
                    let a2740 = 3u64;
                    let a2741 = 0u64;
                    nat_quot(a2739, a2740, a2741)
                };
                let v7: u64 = {
                    let a2744 = {
                        let a2742 = v0;
                        let a2743 = 4u64;
                        nat_sub(a2742, a2743)
                    };
                    let a2745 = 3u64;
                    let a2746 = 0u64;
                    nat_rem(a2744, a2745, a2746)
                };
                let a2747 = v7;
                let a2748 = 0u64;
                if nat_eq(a2747, a2748) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2749 = v7;
                    let a2750 = 1u64;
                    if nat_eq(a2749, a2750) {
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
        let a2751 = v1;
        let a2752 = 5u64;
        nat_eq(a2751, a2752)
    } else {
        false
    } {
        let a2753 = v2;
        let a2754 = 16u64;
        nat_eq(a2753, a2754)
    } else {
        false
    } {
        let a2755 = v3;
        let a2756 = 0u64;
        nat_eq(a2755, a2756)
    } else {
        false
    } {
        let a2757 = v4;
        let a2758 = 0u64;
        nat_eq(a2757, a2758)
    } else {
        false
    }
}

pub fn f89(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2759 = v1;
    let a2760 = 0u64;
    if nat_eq(a2759, a2760) {
        if v0 {
            let a2761 = v3;
            let a2762 = 0u64;
            if if nat_eq(a2761, a2762) {
                let a2763 = v4;
                let a2764 = 43u64;
                nat_eq(a2763, a2764)
            } else {
                false
            } {
                f154(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2765 = v1;
        let a2766 = 1u64;
        if nat_eq(a2765, a2766) {
            Adt11::C1(v2)
        } else {
            let a2767 = v1;
            let a2768 = 2u64;
            if nat_eq(a2767, a2768) {
                Adt11::C5(v2)
            } else {
                let a2769 = v1;
                let a2770 = 3u64;
                if nat_eq(a2769, a2770) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2771 = v1;
                    let a2772 = 4u64;
                    if nat_eq(a2771, a2772) {
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
    let a2773 = v0;
    let a2774 = 0u64;
    if nat_eq(a2773, a2774) {
        Adt12::C0(0u64, v1)
    } else {
        let a2775 = v0;
        let a2776 = 1u64;
        if nat_eq(a2775, a2776) {
            Adt12::C1(v1)
        } else {
            let a2777 = v0;
            let a2778 = 2u64;
            if nat_eq(a2777, a2778) {
                Adt12::C2(v1)
            } else {
                let a2779 = v0;
                let a2780 = 3u64;
                if nat_eq(a2779, a2780) {
                    Adt12::C3(v1)
                } else {
                    let a2781 = v0;
                    let a2782 = 13u64;
                    if nat_lt(a2781, a2782) {
                        Adt12::C0({
                            let a2783 = v0;
                            let a2784 = 3u64;
                            nat_sub(a2783, a2784)
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
    let a2785 = v0;
    let a2786 = 1u64;
    if nat_eq(a2785, a2786) {
        Adt13::C0(v1)
    } else {
        let a2787 = v0;
        let a2788 = 3u64;
        if if nat_eq(a2787, a2788) {
            true
        } else {
            let a2789 = v0;
            let a2790 = 6u64;
            nat_eq(a2789, a2790)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f92(v0: Adt6, v1: u64) -> R<u64> {
    let a2792 = 0u64;
    let a2793 = match v0.clone() {
        Adt6::C0(h2791, _, _, _) => {
            h2791
        }
    };
    if if if if if nat_lt(a2792, a2793) {
        let a2795 = 0u64;
        let a2796 = match v0.clone() {
            Adt6::C0(_, h2794, _, _) => {
                h2794
            }
        };
        nat_lt(a2795, a2796)
    } else {
        false
    } {
        let a2798 = 0u64;
        let a2799 = match v0.clone() {
            Adt6::C0(_, _, _, h2797) => {
                h2797
            }
        };
        nat_lt(a2798, a2799)
    } else {
        false
    } {
        let a2801 = match v0.clone() {
            Adt6::C0(_, h2800, _, _) => {
                h2800
            }
        };
        let a2802 = 2147483648u64;
        nat_lt(a2801, a2802)
    } else {
        false
    } {
        let a2804 = match v0.clone() {
            Adt6::C0(_, _, _, h2803) => {
                h2803
            }
        };
        let a2805 = 2147483648u64;
        nat_lt(a2804, a2805)
    } else {
        false
    } {
        let v2: u64 = {
            let a2809 = v1;
            let a2810 = {
                let a2807 = 2u64;
                let a2808 = f132(match v0.clone() {
                    Adt6::C0(_, _, h2806, _) => {
                        h2806
                    }
                });
                nat_mul(a2807, a2808)?
            };
            nat_add(a2809, a2810)?
        };
        let v3: u64 = {
            let a2822 = {
                let a2817 = 1u64;
                let a2818 = {
                    let a2815 = match v0.clone() {
                        Adt6::C0(_, _, _, h2811) => {
                            h2811
                        }
                    };
                    let a2816 = {
                        let a2813 = match v0.clone() {
                            Adt6::C0(h2812, _, _, _) => {
                                h2812
                            }
                        };
                        let a2814 = 1u64;
                        nat_sub(a2813, a2814)
                    };
                    nat_mul(a2815, a2816)?
                };
                nat_add(a2817, a2818)?
            };
            let a2823 = {
                let a2820 = 2u64;
                let a2821 = f131(match v0.clone() {
                    Adt6::C0(_, _, h2819, _) => {
                        h2819
                    }
                });
                nat_mul(a2820, a2821)?
            };
            nat_add(a2822, a2823)?
        };
        let a2824 = v3;
        let a2825 = v2;
        if nat_le(a2824, a2825) {
            let a2832 = 1u64;
            let a2833 = {
                let a2829 = {
                    let a2826 = v2;
                    let a2827 = v3;
                    nat_sub(a2826, a2827)
                };
                let a2830 = match v0 {
                    Adt6::C0(_, h2828, _, _) => {
                        h2828
                    }
                };
                let a2831 = 0u64;
                nat_quot(a2829, a2830, a2831)
            };
            nat_add(a2832, a2833)
        } else {
            let a2837 = {
                let a2834 = v3;
                let a2835 = v2;
                nat_sub(a2834, a2835)
            };
            let a2838 = match v0 {
                Adt6::C0(_, h2836, _, _) => {
                    h2836
                }
            };
            if nat_lt(a2837, a2838) {
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
        let a2839 = v0;
        let a2840 = 4294967296u64;
        let a2841 = 0u64;
        nat_rem(a2839, a2840, a2841)
    };
    let a2842 = v1;
    let a2843 = 2147483648u64;
    if nat_lt(a2842, a2843) {
        v1
    } else {
        0u64
    }
}

pub fn f94(v0: u64) -> R<u64> {
    let a2846 = 2u64;
    let a2847 = {
        let a2844 = 2u64;
        let a2845 = v0;
        nat_mul(a2844, a2845)?
    };
    nat_add(a2846, a2847)
}

pub fn f95(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2849 = match v1.clone() {
        Adt14::C0(h2848, _, _, _, _, _, _, _, _, _, _) => {
            h2848
        }
    };
    let a2850 = 0u64;
    if if if if nat_eq(a2849, a2850) {
        let a2852 = match v2.clone() {
            Adt14::C0(h2851, _, _, _, _, _, _, _, _, _, _) => {
                h2851
            }
        };
        let a2853 = 0u64;
        nat_eq(a2852, a2853)
    } else {
        false
    } {
        f172(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f173(match v1.clone() {
            Adt14::C0(_, h2854, _, _, _, _, _, _, _, _, _) => {
                h2854
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2855, _, _, _, _, _, _, _, _) => {
                h2855
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2856, _, _, _, _, _, _, _) => {
                h2856
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2857, _, _, _, _, _, _) => {
                h2857
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f174(v1.clone(), v2.clone())
            } else {
                false
            } {
                f175(v1.clone())
            } else {
                false
            } {
                f175(v2.clone())
            } else {
                false
            } {
                if f176(v1.clone(), v2.clone())? {
                    true
                } else {
                    f177(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f173(match v1.clone() {
            Adt14::C0(_, h2858, _, _, _, _, _, _, _, _, _) => {
                h2858
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2859, _, _, _, _, _, _, _, _) => {
                h2859
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2860, _, _, _, _, _, _, _) => {
                h2860
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2861, _, _, _, _, _, _) => {
                h2861
            }
        }) {
            0u64
        } else {
            let a2867 = match v1.clone() {
                Adt14::C0(_, _, h2862, _, _, _, _, _, _, _, _) => {
                    h2862
                }
            };
            let a2868 = {
                let a2865 = match v1.clone() {
                    Adt14::C0(_, _, _, h2863, _, _, _, _, _, _, _) => {
                        h2863
                    }
                };
                let a2866 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2864, _, _, _, _, _, _) => {
                        h2864
                    }
                };
                nat_mul(a2865, a2866)?
            };
            nat_mul(a2867, a2868)?
        };
        let v4: u64 = {
            let a2869 = v3;
            let a2870 = 0u64;
            if if nat_eq(a2869, a2870) {
                true
            } else {
                f178(v2.clone())?
            } {
                0u64
            } else {
                let a2872 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2871, _) => {
                        h2871
                    }
                };
                let a2873 = 4u64;
                let a2874 = 0u64;
                nat_quot(a2872, a2873, a2874)
            }
        };
        Ok(Adt15::C1(f179(f180(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2875, _, _, _, _, _) => {
                h2875
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2876, _, _, _, _, _) => {
                h2876
            }
        }), match v1 {
            Adt14::C0(_, h2877, _, _, _, _, _, _, _, _, _) => {
                h2877
            }
        }, v3, v4)?, 4u64, 64u64, v3, f181(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f96(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f182(v3.clone());
    let a2879 = match v2.clone() {
        Adt14::C0(h2878, _, _, _, _, _, _, _, _, _, _) => {
            h2878
        }
    };
    let a2880 = 0u64;
    if if if if nat_eq(a2879, a2880) {
        let a2882 = match v3.clone() {
            Adt14::C0(h2881, _, _, _, _, _, _, _, _, _, _) => {
                h2881
            }
        };
        let a2883 = 0u64;
        nat_eq(a2882, a2883)
    } else {
        false
    } {
        f172(v2.clone(), v3)
    } else {
        false
    } {
        if f173(match v2.clone() {
            Adt14::C0(_, h2884, _, _, _, _, _, _, _, _, _) => {
                h2884
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2885, _, _, _, _, _, _, _, _) => {
                h2885
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2886, _, _, _, _, _, _, _) => {
                h2886
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2887, _, _, _, _, _, _) => {
                h2887
            }
        }) {
            true
        } else {
            let a2889 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2888, _, _, _, _, _) => {
                    h2888
                }
            };
            let a2890 = 4u64;
            if if if if if nat_eq(a2889, a2890) {
                f183(match v2.clone() {
                    Adt14::C0(_, h2891, _, _, _, _, _, _, _, _, _) => {
                        h2891
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2892, _, _, _, _, _, _, _, _) => {
                        h2892
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2893, _, _, _, _, _, _, _) => {
                        h2893
                    }
                }, v0)?
            } else {
                false
            } {
                f175(v2.clone())
            } else {
                false
            } {
                f175(v4.clone())
            } else {
                false
            } {
                let a2895 = f184(v4.clone())?;
                let a2896 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2894, _) => {
                        h2894
                    }
                };
                nat_le(a2895, a2896)
            } else {
                false
            } {
                if f176(v2.clone(), v4.clone())? {
                    true
                } else {
                    f177(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f173(match v2.clone() {
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
            f178(v4.clone())?
        } {
            0u64
        } else {
            let a2902 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2901, _) => {
                    h2901
                }
            };
            let a2903 = 4u64;
            let a2904 = 0u64;
            nat_quot(a2902, a2903, a2904)
        };
        Ok(Adt15::C1(f185(match v2.clone() {
            Adt14::C0(_, h2905, _, _, _, _, _, _, _, _, _) => {
                h2905
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2906, _, _, _, _, _, _, _, _) => {
                h2906
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2907, _, _, _, _, _, _, _) => {
                h2907
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2908, _, _, _, _, _, _) => {
                h2908
            }
        }, v0, v5)?, 4u64, 65u64, v0, f181(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f97(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f182(v4.clone());
    if if if if if if if v1 {
        let a2910 = match v2.clone() {
            Adt14::C0(h2909, _, _, _, _, _, _, _, _, _, _) => {
                h2909
            }
        };
        let a2911 = 0u64;
        nat_eq(a2910, a2911)
    } else {
        false
    } {
        let a2913 = match v3.clone() {
            Adt14::C0(h2912, _, _, _, _, _, _, _, _, _, _) => {
                h2912
            }
        };
        let a2914 = 0u64;
        nat_eq(a2913, a2914)
    } else {
        false
    } {
        let a2916 = match v4.clone() {
            Adt14::C0(h2915, _, _, _, _, _, _, _, _, _, _) => {
                h2915
            }
        };
        let a2917 = 0u64;
        nat_eq(a2916, a2917)
    } else {
        false
    } {
        f172(v2.clone(), v3.clone())
    } else {
        false
    } {
        f172(v2.clone(), v4)
    } else {
        false
    } {
        if f173(match v2.clone() {
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
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2921, _, _, _, _, _, _) => {
                h2921
            }
        }) {
            true
        } else {
            let a2923 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2922, _, _, _, _, _) => {
                    h2922
                }
            };
            let a2924 = 4u64;
            if if if if if if if nat_eq(a2923, a2924) {
                let a2926 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2925, _, _, _, _, _) => {
                        h2925
                    }
                };
                let a2927 = 4u64;
                nat_eq(a2926, a2927)
            } else {
                false
            } {
                f175(v2.clone())
            } else {
                false
            } {
                f175(v3.clone())
            } else {
                false
            } {
                f175(v5.clone())
            } else {
                false
            } {
                let a2929 = f184(v5.clone())?;
                let a2930 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2928, _) => {
                        h2928
                    }
                };
                nat_le(a2929, a2930)
            } else {
                false
            } {
                if f176(v2.clone(), v5.clone())? {
                    true
                } else {
                    f177(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f176(v3.clone(), v5.clone())? {
                    true
                } else {
                    f177(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f173(match v2.clone() {
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
            0u64
        } else {
            let a2940 = match v2.clone() {
                Adt14::C0(_, _, h2935, _, _, _, _, _, _, _, _) => {
                    h2935
                }
            };
            let a2941 = {
                let a2938 = match v2.clone() {
                    Adt14::C0(_, _, _, h2936, _, _, _, _, _, _, _) => {
                        h2936
                    }
                };
                let a2939 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2937, _, _, _, _, _, _) => {
                        h2937
                    }
                };
                nat_mul(a2938, a2939)?
            };
            nat_mul(a2940, a2941)?
        };
        let v7: u64 = {
            let a2942 = v6;
            let a2943 = 0u64;
            if if nat_eq(a2942, a2943) {
                true
            } else {
                f178(v5.clone())?
            } {
                0u64
            } else {
                let a2945 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2944, _) => {
                        h2944
                    }
                };
                let a2946 = 4u64;
                let a2947 = 0u64;
                nat_quot(a2945, a2946, a2947)
            }
        };
        Ok(Adt15::C1(f186(match v2 {
            Adt14::C0(_, h2948, _, _, _, _, _, _, _, _, _) => {
                h2948
            }
        }, v6, v7)?, 4u64, 66u64, v6, f181(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f187(v0, v3, v6) {
        f187(v1, v4, v6)
    } else {
        false
    } {
        f187(v2, v5, v6)
    } else {
        false
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f188(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f188(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f188(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f188(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f188(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f188(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2949 = v2;
    let a2950 = 0u64;
    if nat_eq(a2949, a2950) {
        Ok(0u64)
    } else {
        let a2953 = 2u64;
        let a2954 = {
            let a2951 = v0;
            let a2952 = v1;
            nat_mul(a2951, a2952)?
        };
        nat_mul(a2953, a2954)
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2955 = v2;
    let a2956 = f168(v0, v1)?;
    nat_mul(a2955, a2956)
}

pub fn f102(v0: u64) -> u64 {
    let a2957 = v0;
    let a2958 = 30u64;
    if nat_eq(a2957, a2958) {
        3u64
    } else {
        1u64
    }
}

pub fn f103(v0: u64) -> R<u64> {
    let a2961 = 5u64;
    let a2962 = {
        let a2959 = 4u64;
        let a2960 = v0;
        nat_mul(a2959, a2960)?
    };
    nat_add(a2961, a2962)
}

pub fn f104(v0: u64) -> bool {
    let a2963 = v0;
    let a2964 = 0u64;
    if if nat_eq(a2963, a2964) {
        true
    } else {
        let a2965 = v0;
        let a2966 = 1u64;
        nat_eq(a2965, a2966)
    } {
        true
    } else {
        let a2967 = v0;
        let a2968 = 30u64;
        nat_eq(a2967, a2968)
    }
}

pub fn f105(v0: Adt2, v1: u64) -> R<bool> {
    let a2970 = match v0.clone() {
        Adt2::C0(_, _, _, h2969, _, _, _, _, _, _, _) => {
            h2969
        }
    };
    let a2971 = 1u64;
    if if if nat_eq(a2970, a2971) {
        let a2973 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2972, _, _, _, _, _, _) => {
                h2972
            }
        };
        let a2974 = 1u64;
        nat_eq(a2973, a2974)
    } else {
        false
    } {
        let a2976 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2975, _, _, _, _, _) => {
                h2975
            }
        };
        let a2977 = v1;
        nat_eq(a2976, a2977)
    } else {
        false
    } {
        let a2979 = match v0.clone() {
            Adt2::C0(_, _, h2978, _, _, _, _, _, _, _, _) => {
                h2978
            }
        };
        let a2980 = 1u64;
        if nat_eq(a2979, a2980) {
            Ok(true)
        } else {
            let a2985 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2981, _, _, _, _) => {
                    h2981
                }
            };
            let a2986 = {
                let a2983 = match v0 {
                    Adt2::C0(_, h2982, _, _, _, _, _, _, _, _, _) => {
                        h2982
                    }
                };
                let a2984 = v1;
                nat_mul(a2983, a2984)?
            };
            Ok(nat_eq(a2985, a2986))
        }
    } else {
        Ok(false)
    }
}

pub fn f106(v0: Adt6) -> R<u64> {
    let a2999 = 2u64;
    let a3000 = {
        let a2997 = 2u64;
        let a2998 = {
            let a2994 = {
                let a2991 = match v0.clone() {
                    Adt6::C0(h2987, _, _, _) => {
                        h2987
                    }
                };
                let a2992 = {
                    let a2989 = match v0.clone() {
                        Adt6::C0(_, h2988, _, _) => {
                            h2988
                        }
                    };
                    let a2990 = 1u64;
                    nat_sub(a2989, a2990)
                };
                nat_add(a2991, a2992)?
            };
            let a2995 = match v0 {
                Adt6::C0(_, h2993, _, _) => {
                    h2993
                }
            };
            let a2996 = 0u64;
            nat_quot(a2994, a2995, a2996)
        };
        nat_mul(a2997, a2998)?
    };
    nat_add(a2999, a3000)
}

pub fn f107(v0: u64) -> R<u64> {
    let a3003 = 2u64;
    let a3004 = {
        let a3001 = v0;
        let a3002 = 2u64;
        nat_mul(a3001, a3002)?
    };
    nat_add(a3003, a3004)
}

pub fn f108(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3008 = {
            let a3005 = v2;
            let a3006 = 1u64;
            nat_sub(a3005, a3006)
        };
        let a3009 = match v0.clone() {
            Adt6::C0(_, h3007, _, _) => {
                h3007
            }
        };
        nat_mul(a3008, a3009)?
    };
    let a3011 = 0u64;
    let a3012 = match v0.clone() {
        Adt6::C0(h3010, _, _, _) => {
            h3010
        }
    };
    if if if if if if if if nat_lt(a3011, a3012) {
        let a3014 = match v0.clone() {
            Adt6::C0(h3013, _, _, _) => {
                h3013
            }
        };
        let a3015 = 2147483648u64;
        nat_lt(a3014, a3015)
    } else {
        false
    } {
        let a3017 = 0u64;
        let a3018 = match v0.clone() {
            Adt6::C0(_, h3016, _, _) => {
                h3016
            }
        };
        nat_lt(a3017, a3018)
    } else {
        false
    } {
        let a3020 = match v0.clone() {
            Adt6::C0(_, h3019, _, _) => {
                h3019
            }
        };
        let a3021 = 2147483648u64;
        nat_lt(a3020, a3021)
    } else {
        false
    } {
        let a3022 = v1;
        let a3023 = 2147483648u64;
        nat_lt(a3022, a3023)
    } else {
        false
    } {
        let a3024 = 0u64;
        let a3025 = v2;
        nat_lt(a3024, a3025)
    } else {
        false
    } {
        let a3026 = v3;
        let a3027 = 2147483648u64;
        nat_lt(a3026, a3027)
    } else {
        false
    } {
        let a3039 = {
            let a3034 = {
                let a3031 = v3;
                let a3032 = {
                    let a3029 = match v0.clone() {
                        Adt6::C0(h3028, _, _, _) => {
                            h3028
                        }
                    };
                    let a3030 = 1u64;
                    nat_sub(a3029, a3030)
                };
                nat_add(a3031, a3032)?
            };
            let a3035 = f131(match v0.clone() {
                Adt6::C0(_, _, h3033, _) => {
                    h3033
                }
            });
            nat_add(a3034, a3035)?
        };
        let a3040 = {
            let a3037 = 2147483647u64;
            let a3038 = f132(match v0.clone() {
                Adt6::C0(_, _, h3036, _) => {
                    h3036
                }
            });
            nat_add(a3037, a3038)?
        };
        nat_le(a3039, a3040)
    } else {
        false
    } {
        let a3042 = match v0 {
            Adt6::C0(_, _, h3041, _) => {
                h3041
            }
        };
        let a3043 = 2147483648u64;
        Ok(!nat_eq(a3042, a3043))
    } else {
        Ok(false)
    }
}

pub fn f109(v0: Adt7) -> R<u64> {
    let a3055 = if match v0.clone() {
        Adt7::C0(_, _, _, h3044, _) => {
            h3044
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3056 = {
        let a3053 = {
            let a3051 = {
                let m3047 = match v0.clone() {
                    Adt7::C0(h3046, _, _, _, _) => {
                        h3046
                    }
                };
                match m3047 {
                    Adt6::C0(h3045, _, _, _) => {
                        h3045
                    }
                }
            };
            let a3052 = {
                let m3050 = match v0.clone() {
                    Adt7::C0(_, h3049, _, _, _) => {
                        h3049
                    }
                };
                match m3050 {
                    Adt6::C0(h3048, _, _, _) => {
                        h3048
                    }
                }
            };
            nat_mul(a3051, a3052)?
        };
        let a3054 = f163(v0)?;
        nat_mul(a3053, a3054)?
    };
    nat_add(a3055, a3056)
}

pub fn f110(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f189(match v0.clone() {
        Adt2::C0(h3057, _, _, _, _, _, _, _, _, _, _) => {
            h3057
        }
    }, match v0.clone() {
        Adt2::C0(_, h3058, _, _, _, _, _, _, _, _, _) => {
            h3058
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3059, _, _, _, _, _, _, _, _) => {
            h3059
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3060, _, _, _, _, _, _, _) => {
            h3060
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3061, _, _, _, _, _, _) => {
            h3061
        }
    }, match v1.clone() {
        Adt2::C0(h3062, _, _, _, _, _, _, _, _, _, _) => {
            h3062
        }
    }, match v1.clone() {
        Adt2::C0(_, h3063, _, _, _, _, _, _, _, _, _) => {
            h3063
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3064, _, _, _, _, _, _, _, _) => {
            h3064
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3065, _, _, _, _, _, _, _) => {
            h3065
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3066, _, _, _, _, _, _) => {
            h3066
        }
    }, match v2.clone() {
        Adt2::C0(h3067, _, _, _, _, _, _, _, _, _, _) => {
            h3067
        }
    }, match v2.clone() {
        Adt2::C0(_, h3068, _, _, _, _, _, _, _, _, _) => {
            h3068
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3069, _, _, _, _, _, _, _, _) => {
            h3069
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3070, _, _, _, _, _, _, _) => {
            h3070
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3071, _, _, _, _, _, _) => {
            h3071
        }
    }) {
        let m3073 = f114(match v0.clone() {
            Adt2::C0(h3072, _, _, _, _, _, _, _, _, _, _) => {
                h3072
            }
        });
        match m3073 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3079 = {
                    let a3076 = match v0.clone() {
                        Adt2::C0(_, h3074, _, _, _, _, _, _, _, _, _) => {
                            h3074
                        }
                    };
                    let a3077 = match v3.clone() {
                        Adt18::C0(_, h3075, _) => {
                            h3075
                        }
                    };
                    let a3078 = 0u64;
                    nat_rem(a3076, a3077, a3078)
                };
                let a3080 = 0u64;
                if if nat_eq(a3079, a3080) {
                    if f190(match v0.clone() {
                        Adt2::C0(_, h3081, _, _, _, _, _, _, _, _, _) => {
                            h3081
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3082, _, _, _, _, _, _, _, _, _) => {
                            h3082
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3083, _, _, _, _, _, _, _, _) => {
                            h3083
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3084, _, _, _, _, _, _, _) => {
                            h3084
                        }
                    }) {
                        true
                    } else {
                        let a3086 = 4u64;
                        let a3087 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3085, _, _, _, _, _) => {
                                h3085
                            }
                        };
                        if if nat_le(a3086, a3087) {
                            let a3089 = match v3.clone() {
                                Adt18::C0(_, h3088, _) => {
                                    h3088
                                }
                            };
                            let a3090 = 1u64;
                            if nat_eq(a3089, a3090) {
                                true
                            } else {
                                let a3093 = match v3.clone() {
                                    Adt18::C0(_, _, h3091) => {
                                        h3091
                                    }
                                };
                                let a3094 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3092, _, _, _, _, _) => {
                                        h3092
                                    }
                                };
                                nat_le(a3093, a3094)
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
                        Adt18::C0(_, h3095, _) => {
                            h3095
                        }
                    });
                    Ok(Adt10::C4(if f190(match v0.clone() {
                        Adt2::C0(_, h3096, _, _, _, _, _, _, _, _, _) => {
                            h3096
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3097, _, _, _, _, _, _, _, _, _) => {
                            h3097
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3098, _, _, _, _, _, _, _, _) => {
                            h3098
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3099, _, _, _, _, _, _, _) => {
                            h3099
                        }
                    }) {
                        0u64
                    } else {
                        f157(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3100, _, _, _, _, _, _, _, _, _, _) => {
                            h3100
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
        Adt2::C0(h3101, _, _, _, _, _, _, _, _, _, _) => {
            h3101
        }
    }) {
        let a3107 = {
            let a3104 = match v0.clone() {
                Adt2::C0(_, h3102, _, _, _, _, _, _, _, _, _) => {
                    h3102
                }
            };
            let a3105 = f191(match v0.clone() {
                Adt2::C0(h3103, _, _, _, _, _, _, _, _, _, _) => {
                    h3103
                }
            });
            let a3106 = 1u64;
            nat_rem(a3104, a3105, a3106)
        };
        let a3108 = 0u64;
        nat_eq(a3107, a3108)
    } else {
        false
    } {
        let a3110 = match v1.clone() {
            Adt2::C0(h3109, _, _, _, _, _, _, _, _, _, _) => {
                h3109
            }
        };
        let a3111 = 26u64;
        nat_eq(a3110, a3111)
    } else {
        false
    } {
        let a3113 = match v2.clone() {
            Adt2::C0(h3112, _, _, _, _, _, _, _, _, _, _) => {
                h3112
            }
        };
        let a3114 = 0u64;
        nat_eq(a3113, a3114)
    } else {
        false
    } {
        let a3117 = match v0.clone() {
            Adt2::C0(_, h3115, _, _, _, _, _, _, _, _, _) => {
                h3115
            }
        };
        let a3118 = match v2.clone() {
            Adt2::C0(_, h3116, _, _, _, _, _, _, _, _, _) => {
                h3116
            }
        };
        nat_eq(a3117, a3118)
    } else {
        false
    } {
        let a3121 = match v1.clone() {
            Adt2::C0(_, h3119, _, _, _, _, _, _, _, _, _) => {
                h3119
            }
        };
        let a3122 = match v0.clone() {
            Adt2::C0(_, _, h3120, _, _, _, _, _, _, _, _) => {
                h3120
            }
        };
        nat_le(a3121, a3122)
    } else {
        false
    } {
        let a3124 = match v0.clone() {
            Adt2::C0(_, _, _, h3123, _, _, _, _, _, _, _) => {
                h3123
            }
        };
        let a3125 = 1u64;
        nat_eq(a3124, a3125)
    } else {
        false
    } {
        let a3127 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3126, _, _, _, _, _, _) => {
                h3126
            }
        };
        let a3128 = 1u64;
        nat_eq(a3127, a3128)
    } else {
        false
    } {
        let a3130 = match v2.clone() {
            Adt2::C0(_, _, _, h3129, _, _, _, _, _, _, _) => {
                h3129
            }
        };
        let a3131 = 1u64;
        nat_eq(a3130, a3131)
    } else {
        false
    } {
        let a3133 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3132, _, _, _, _, _, _) => {
                h3132
            }
        };
        let a3134 = 1u64;
        nat_eq(a3133, a3134)
    } else {
        false
    } {
        let a3136 = match v1.clone() {
            Adt2::C0(_, _, h3135, _, _, _, _, _, _, _, _) => {
                h3135
            }
        };
        let a3137 = 1u64;
        nat_eq(a3136, a3137)
    } else {
        false
    } {
        let a3139 = match v1.clone() {
            Adt2::C0(_, _, _, h3138, _, _, _, _, _, _, _) => {
                h3138
            }
        };
        let a3140 = 1u64;
        nat_eq(a3139, a3140)
    } else {
        false
    } {
        let a3142 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3141, _, _, _, _, _, _) => {
                h3141
            }
        };
        let a3143 = 1u64;
        nat_eq(a3142, a3143)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3144, _, _, _, _, _, _, _, _, _) => {
                h3144
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3145, _, _, _, _, _, _, _, _) => {
                h3145
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3146, _, _, _, _, _, _, _) => {
                h3146
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3147, _, _, _, _, _, _) => {
                h3147
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3149 = match v1.clone() {
                Adt2::C0(_, h3148, _, _, _, _, _, _, _, _, _) => {
                    h3148
                }
            };
            let a3150 = 0u64;
            if nat_eq(a3149, a3150) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3152 = match v0.clone() {
                    Adt2::C0(_, h3151, _, _, _, _, _, _, _, _, _) => {
                        h3151
                    }
                };
                let a3153 = 2147483647u64;
                if if if if if if nat_le(a3152, a3153) {
                    let a3155 = match v1.clone() {
                        Adt2::C0(_, h3154, _, _, _, _, _, _, _, _, _) => {
                            h3154
                        }
                    };
                    let a3156 = 2147483647u64;
                    nat_le(a3155, a3156)
                } else {
                    false
                } {
                    let a3158 = 4u64;
                    let a3159 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3157, _, _, _, _, _) => {
                            h3157
                        }
                    };
                    nat_le(a3158, a3159)
                } else {
                    false
                } {
                    let a3165 = {
                        let a3162 = match v2.clone() {
                            Adt2::C0(_, h3160, _, _, _, _, _, _, _, _, _) => {
                                h3160
                            }
                        };
                        let a3163 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3161, _, _, _, _, _) => {
                                h3161
                            }
                        };
                        nat_mul(a3162, a3163)?
                    };
                    let a3166 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3164, _, _, _, _) => {
                            h3164
                        }
                    };
                    nat_le(a3165, a3166)
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
                        let a3179 = {
                            let a3170 = {
                                let a3168 = 2u64;
                                let a3169 = match v1.clone() {
                                    Adt2::C0(_, h3167, _, _, _, _, _, _, _, _, _) => {
                                        h3167
                                    }
                                };
                                nat_mul(a3168, a3169)?
                            };
                            let a3171 = f16(v2.clone(), 1u64)?;
                            nat_add(a3170, a3171)?
                        };
                        let a3180 = {
                            let a3177 = match v1.clone() {
                                Adt2::C0(_, h3172, _, _, _, _, _, _, _, _, _) => {
                                    h3172
                                }
                            };
                            let a3178 = {
                                let a3175 = 1u64;
                                let a3176 = f155(match v0.clone() {
                                    Adt2::C0(h3173, _, _, _, _, _, _, _, _, _, _) => {
                                        h3173
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3174, _, _, _, _, _, _, _, _, _) => {
                                        h3174
                                    }
                                })?;
                                nat_add(a3175, a3176)?
                            };
                            nat_mul(a3177, a3178)?
                        };
                        nat_add(a3179, a3180)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3181, _, _, _, _, _, _, _, _, _) => {
                            h3181
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3182, _, _, _, _, _, _, _, _, _) => {
                            h3182
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3183, _, _, _, _, _) => {
                            h3183
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3184, _, _, _, _) => {
                            h3184
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3185, _, _, _) => {
                            h3185
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3186, _, _) => {
                            h3186
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3187, _, _, _, _, _, _, _, _, _, _) => {
                            h3187
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
    let a3188 = v0;
    let a3189 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3188, a3189) {
        true
    } else {
        let a3190 = v0;
        let a3191 = 1u64;
        nat_eq(a3190, a3191)
    } {
        true
    } else {
        let a3192 = v0;
        let a3193 = 2u64;
        nat_eq(a3192, a3193)
    } {
        true
    } else {
        let a3194 = v0;
        let a3195 = 3u64;
        nat_eq(a3194, a3195)
    } {
        true
    } else {
        let a3196 = v0;
        let a3197 = 6u64;
        nat_eq(a3196, a3197)
    } {
        true
    } else {
        let a3198 = v0;
        let a3199 = 7u64;
        nat_eq(a3198, a3199)
    } {
        true
    } else {
        let a3200 = v0;
        let a3201 = 8u64;
        nat_eq(a3200, a3201)
    } {
        true
    } else {
        let a3202 = v0;
        let a3203 = 10u64;
        nat_eq(a3202, a3203)
    } {
        true
    } else {
        let a3204 = v0;
        let a3205 = 11u64;
        nat_eq(a3204, a3205)
    } {
        true
    } else {
        let a3206 = v0;
        let a3207 = 12u64;
        nat_eq(a3206, a3207)
    } {
        true
    } else {
        let a3208 = v0;
        let a3209 = 13u64;
        nat_eq(a3208, a3209)
    } {
        true
    } else {
        let a3210 = v0;
        let a3211 = 14u64;
        nat_eq(a3210, a3211)
    } {
        true
    } else {
        let a3212 = v0;
        let a3213 = 16u64;
        nat_eq(a3212, a3213)
    } {
        true
    } else {
        let a3214 = v0;
        let a3215 = 17u64;
        nat_eq(a3214, a3215)
    } {
        true
    } else {
        let a3216 = v0;
        let a3217 = 18u64;
        nat_eq(a3216, a3217)
    } {
        true
    } else {
        let a3218 = v0;
        let a3219 = 19u64;
        nat_eq(a3218, a3219)
    } {
        true
    } else {
        let a3220 = v0;
        let a3221 = 20u64;
        nat_eq(a3220, a3221)
    } {
        true
    } else {
        let a3222 = v0;
        let a3223 = 21u64;
        nat_eq(a3222, a3223)
    } {
        true
    } else {
        let a3224 = v0;
        let a3225 = 22u64;
        nat_eq(a3224, a3225)
    } {
        true
    } else {
        let a3226 = v0;
        let a3227 = 23u64;
        nat_eq(a3226, a3227)
    } {
        true
    } else {
        let a3228 = v0;
        let a3229 = 26u64;
        nat_eq(a3228, a3229)
    } {
        true
    } else {
        let a3230 = v0;
        let a3231 = 29u64;
        nat_eq(a3230, a3231)
    } {
        true
    } else {
        let a3232 = v0;
        let a3233 = 30u64;
        nat_eq(a3232, a3233)
    } {
        true
    } else {
        let a3234 = v0;
        let a3235 = 34u64;
        nat_eq(a3234, a3235)
    } {
        true
    } else {
        let a3236 = v0;
        let a3237 = 35u64;
        nat_eq(a3236, a3237)
    } {
        true
    } else {
        let a3238 = v0;
        let a3239 = 39u64;
        nat_eq(a3238, a3239)
    } {
        true
    } else {
        let a3240 = v0;
        let a3241 = 40u64;
        nat_eq(a3240, a3241)
    } {
        true
    } else {
        let a3242 = v0;
        let a3243 = 41u64;
        nat_eq(a3242, a3243)
    } {
        true
    } else {
        let a3244 = v0;
        let a3245 = 42u64;
        nat_eq(a3244, a3245)
    } {
        true
    } else {
        let a3246 = v0;
        let a3247 = 43u64;
        nat_eq(a3246, a3247)
    }
}

pub fn f113(v0: u64) -> bool {
    if f104(v0) {
        true
    } else {
        let a3248 = v0;
        let a3249 = 43u64;
        nat_eq(a3248, a3249)
    }
}

pub fn f114(v0: u64) -> Adt17 {
    let a3250 = v0;
    let a3251 = 0u64;
    if nat_eq(a3250, a3251) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3252 = v0;
        let a3253 = 1u64;
        if nat_eq(a3252, a3253) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3254 = v0;
            let a3255 = 2u64;
            if nat_eq(a3254, a3255) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3256 = v0;
                let a3257 = 3u64;
                if nat_eq(a3256, a3257) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3258 = v0;
                    let a3259 = 6u64;
                    if nat_eq(a3258, a3259) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3260 = v0;
                        let a3261 = 7u64;
                        if nat_eq(a3260, a3261) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3262 = v0;
                            let a3263 = 8u64;
                            if nat_eq(a3262, a3263) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3264 = v0;
                                let a3265 = 9u64;
                                if nat_eq(a3264, a3265) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3266 = v0;
                                    let a3267 = 10u64;
                                    if nat_eq(a3266, a3267) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3268 = v0;
                                        let a3269 = 11u64;
                                        if nat_eq(a3268, a3269) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3270 = v0;
                                            let a3271 = 12u64;
                                            if nat_eq(a3270, a3271) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3272 = v0;
                                                let a3273 = 13u64;
                                                if nat_eq(a3272, a3273) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3274 = v0;
                                                    let a3275 = 14u64;
                                                    if nat_eq(a3274, a3275) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3276 = v0;
                                                        let a3277 = 15u64;
                                                        if nat_eq(a3276, a3277) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3278 = v0;
                                                            let a3279 = 16u64;
                                                            if nat_eq(a3278, a3279) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3280 = v0;
                                                                let a3281 = 17u64;
                                                                if nat_eq(a3280, a3281) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3282 = v0;
                                                                    let a3283 = 18u64;
                                                                    if nat_eq(a3282, a3283) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3284 = v0;
                                                                        let a3285 = 19u64;
                                                                        if nat_eq(a3284, a3285) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3286 = v0;
                                                                            let a3287 = 20u64;
                                                                            if nat_eq(a3286, a3287) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3288 = v0;
                                                                                let a3289 = 21u64;
                                                                                if nat_eq(a3288, a3289) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3290 = v0;
                                                                                    let a3291 = 22u64;
                                                                                    if nat_eq(a3290, a3291) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3292 = v0;
                                                                                        let a3293 = 23u64;
                                                                                        if nat_eq(a3292, a3293) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3294 = v0;
                                                                                            let a3295 = 24u64;
                                                                                            if nat_eq(a3294, a3295) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3296 = v0;
                                                                                                let a3297 = 25u64;
                                                                                                if nat_eq(a3296, a3297) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3298 = v0;
                                                                                                    let a3299 = 26u64;
                                                                                                    if nat_eq(a3298, a3299) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3300 = v0;
                                                                                                        let a3301 = 27u64;
                                                                                                        if nat_eq(a3300, a3301) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3302 = v0;
                                                                                                            let a3303 = 28u64;
                                                                                                            if nat_eq(a3302, a3303) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3304 = v0;
                                                                                                                let a3305 = 29u64;
                                                                                                                if nat_eq(a3304, a3305) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3306 = v0;
                                                                                                                    let a3307 = 30u64;
                                                                                                                    if nat_eq(a3306, a3307) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3308 = v0;
                                                                                                                        let a3309 = 34u64;
                                                                                                                        if nat_eq(a3308, a3309) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3310 = v0;
                                                                                                                            let a3311 = 35u64;
                                                                                                                            if nat_eq(a3310, a3311) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3312 = v0;
                                                                                                                                let a3313 = 39u64;
                                                                                                                                if nat_eq(a3312, a3313) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3314 = v0;
                                                                                                                                    let a3315 = 40u64;
                                                                                                                                    if nat_eq(a3314, a3315) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3316 = v0;
                                                                                                                                        let a3317 = 41u64;
                                                                                                                                        if nat_eq(a3316, a3317) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3318 = v0;
                                                                                                                                            let a3319 = 42u64;
                                                                                                                                            if nat_eq(a3318, a3319) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3320 = v0;
                                                                                                                                                let a3321 = 43u64;
                                                                                                                                                if nat_eq(a3320, a3321) {
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
    let a3324 = match v1.clone() {
        Adt2::C0(h3322, _, _, _, _, _, _, _, _, _, _) => {
            h3322
        }
    };
    let a3325 = match v3.clone() {
        Adt2::C0(h3323, _, _, _, _, _, _, _, _, _, _) => {
            h3323
        }
    };
    if if nat_eq(a3324, a3325) {
        let a3326 = v0;
        let a3327 = 33u64;
        if nat_eq(a3326, a3327) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3329 = f114(match v1.clone() {
            Adt2::C0(h3328, _, _, _, _, _, _, _, _, _, _) => {
                h3328
            }
        });
        match m3329 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3330, _) => {
                        h3330
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3335 = {
                    let a3332 = match v1.clone() {
                        Adt2::C0(_, h3331, _, _, _, _, _, _, _, _, _) => {
                            h3331
                        }
                    };
                    let a3333 = v5;
                    let a3334 = 1u64;
                    nat_rem(a3332, a3333, a3334)
                };
                let a3336 = 0u64;
                if if if nat_eq(a3335, a3336) {
                    let a3341 = {
                        let a3338 = match v3.clone() {
                            Adt2::C0(_, h3337, _, _, _, _, _, _, _, _, _) => {
                                h3337
                            }
                        };
                        let a3339 = v5;
                        let a3340 = 1u64;
                        nat_rem(a3338, a3339, a3340)
                    };
                    let a3342 = 0u64;
                    nat_eq(a3341, a3342)
                } else {
                    false
                } {
                    let a3343 = v6;
                    let a3344 = f16(v3.clone(), v5)?;
                    nat_eq(a3343, a3344)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3345) => {
                            h3345
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
    let a3346 = v0;
    let a3347 = 0u64;
    if if nat_eq(a3346, a3347) {
        true
    } else {
        let a3348 = v1;
        let a3349 = 0u64;
        nat_eq(a3348, a3349)
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
        let a3353 = {
            let a3350 = v0;
            let a3351 = v4;
            let a3352 = 1u64;
            nat_rem(a3350, a3351, a3352)
        };
        let a3354 = 0u64;
        if if if nat_eq(a3353, a3354) {
            let a3358 = {
                let a3355 = v1;
                let a3356 = v5;
                let a3357 = 1u64;
                nat_rem(a3355, a3356, a3357)
            };
            let a3359 = 0u64;
            nat_eq(a3358, a3359)
        } else {
            false
        } {
            let a3363 = {
                let a3360 = v2;
                let a3361 = v6;
                let a3362 = 1u64;
                nat_rem(a3360, a3361, a3362)
            };
            let a3364 = 0u64;
            nat_eq(a3363, a3364)
        } else {
            false
        } {
            let a3368 = {
                let a3365 = v3;
                let a3366 = v7;
                let a3367 = 1u64;
                nat_rem(a3365, a3366, a3367)
            };
            let a3369 = 0u64;
            nat_eq(a3368, a3369)
        } else {
            false
        }
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3370 = v0;
    let a3371 = v1;
    if nat_eq(a3370, a3371) {
        let a3374 = v4;
        let a3375 = {
            let a3372 = v2;
            let a3373 = v3;
            nat_add(a3372, a3373)?
        };
        Ok(nat_eq(a3374, a3375))
    } else {
        let a3376 = v2;
        let a3377 = v3;
        if nat_eq(a3376, a3377) {
            let a3378 = v4;
            let a3379 = v2;
            Ok(nat_eq(a3378, a3379))
        } else {
            Ok(false)
        }
    }
}

pub fn f119(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3382 = {
            let a3380 = v0;
            let a3381 = f132(v1);
            nat_add(a3380, a3381)?
        };
        let a3383 = f132(v2);
        nat_add(a3382, a3383)?
    };
    let v5: u64 = {
        let a3384 = f131(v1);
        let a3385 = f131(v2);
        nat_add(a3384, a3385)?
    };
    let a3386 = v1;
    let a3387 = 4294967295u64;
    if if nat_le(a3386, a3387) {
        let a3388 = v2;
        let a3389 = 4294967295u64;
        nat_le(a3388, a3389)
    } else {
        false
    } {
        let a3390 = v5;
        let a3391 = v4;
        if nat_le(a3390, a3391) {
            let a3394 = v3;
            let a3395 = {
                let a3392 = v4;
                let a3393 = v5;
                nat_sub(a3392, a3393)
            };
            Ok(nat_eq(a3394, a3395))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f120(v0: u64, v1: u64) -> R<bool> {
    let a3396 = v1;
    let a3397 = 4294967295u64;
    if nat_le(a3396, a3397) {
        let a3400 = {
            let a3398 = f132(v1);
            let a3399 = f131(v1);
            nat_add(a3398, a3399)?
        };
        let a3401 = v0;
        Ok(nat_lt(a3400, a3401))
    } else {
        Ok(false)
    }
}

pub fn f121(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f192(v0.clone(), false)?;
    let v6: u64 = f192(v0, true)?;
    let a3402 = v6;
    let a3403 = v1;
    if if nat_le(a3402, a3403) {
        let a3410 = {
            let a3408 = {
                let a3404 = v1;
                let a3405 = v5;
                nat_add(a3404, a3405)?
            };
            let a3409 = {
                let a3406 = v3;
                let a3407 = 0u64;
                if nat_eq(a3406, a3407) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3408, a3409)?
        };
        let a3411 = v2;
        nat_le(a3410, a3411)
    } else {
        false
    } {
        if v4 {
            let a3416 = {
                let a3414 = {
                    let a3412 = v1;
                    let a3413 = v5;
                    nat_add(a3412, a3413)?
                };
                let a3415 = v6;
                nat_sub(a3414, a3415)
            };
            let a3417 = v2;
            Ok(nat_lt(a3416, a3417))
        } else {
            let a3422 = {
                let a3420 = {
                    let a3418 = v1;
                    let a3419 = v5;
                    nat_add(a3418, a3419)?
                };
                let a3421 = v6;
                nat_sub(a3420, a3421)
            };
            let a3423 = v2;
            Ok(nat_le(a3422, a3423))
        }
    } else {
        Ok(false)
    }
}

pub fn f122(v0: u64, v1: u64) -> bool {
    let a3424 = v0;
    let a3425 = 0u64;
    if if nat_eq(a3424, a3425) {
        true
    } else {
        let a3426 = v0;
        let a3427 = 1u64;
        nat_eq(a3426, a3427)
    } {
        let a3428 = v1;
        let a3429 = 4294967296u64;
        nat_lt(a3428, a3429)
    } else {
        false
    }
}

pub fn f123(v0: u64, v1: u64) -> R<u64> {
    let a3434 = {
        let a3432 = v0;
        let a3433 = {
            let a3430 = v1;
            let a3431 = 1u64;
            nat_sub(a3430, a3431)
        };
        nat_add(a3432, a3433)?
    };
    let a3435 = v1;
    let a3436 = 0u64;
    Ok(nat_quot(a3434, a3435, a3436))
}

pub fn f124(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h3437, _, _, _, _, _, _, _, _, _) => {
            h3437
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3438, _, _, _, _, _, _, _, _) => {
            h3438
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3439, _, _, _, _, _, _, _) => {
            h3439
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3440, _, _, _, _, _, _) => {
            h3440
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3443 = match v1.clone() {
                Adt2::C0(_, h3441, _, _, _, _, _, _, _, _, _) => {
                    h3441
                }
            };
            let a3444 = match v1.clone() {
                Adt2::C0(_, _, h3442, _, _, _, _, _, _, _, _) => {
                    h3442
                }
            };
            nat_mul(a3443, a3444)?
        };
        let v5: u64 = {
            let a3446 = v4;
            let a3447 = match v1.clone() {
                Adt2::C0(_, _, _, h3445, _, _, _, _, _, _, _) => {
                    h3445
                }
            };
            nat_mul(a3446, a3447)?
        };
        let a3460 = {
            let a3458 = {
                let a3449 = match v0.clone() {
                    Adt2::C0(_, h3448, _, _, _, _, _, _, _, _, _) => {
                        h3448
                    }
                };
                let a3450 = 1u64;
                nat_sub(a3449, a3450)
            };
            let a3459 = {
                let a3456 = f193(match v0.clone() {
                    Adt2::C0(_, _, h3451, _, _, _, _, _, _, _, _) => {
                        h3451
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3452, _, _, _, _, _, _, _, _, _) => {
                        h3452
                    }
                })?;
                let a3457 = f193(match v0 {
                    Adt2::C0(_, _, _, h3453, _, _, _, _, _, _, _) => {
                        h3453
                    }
                }, v2, {
                    let a3454 = v3;
                    let a3455 = v5;
                    nat_mul(a3454, a3455)?
                }, v4)?;
                nat_add(a3456, a3457)?
            };
            nat_add(a3458, a3459)?
        };
        let a3461 = f16(v1, 1u64)?;
        Ok(nat_lt(a3460, a3461))
    }
}

pub fn f125(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3462 = v0;
    let a3463 = 2147483648u64;
    if if if if nat_lt(a3462, a3463) {
        let a3464 = v1;
        let a3465 = 2147483648u64;
        nat_lt(a3464, a3465)
    } else {
        false
    } {
        let a3468 = {
            let a3466 = v0;
            let a3467 = v1;
            nat_mul(a3466, a3467)?
        };
        let a3469 = 2147483648u64;
        nat_lt(a3468, a3469)
    } else {
        false
    } {
        let a3474 = {
            let a3472 = {
                let a3470 = v0;
                let a3471 = 1u64;
                nat_sub(a3470, a3471)
            };
            let a3473 = v2;
            nat_mul(a3472, a3473)?
        };
        let a3475 = 2147483648u64;
        nat_lt(a3474, a3475)
    } else {
        false
    } {
        let a3480 = {
            let a3478 = {
                let a3476 = v1;
                let a3477 = 1u64;
                nat_sub(a3476, a3477)
            };
            let a3479 = v2;
            nat_mul(a3478, a3479)?
        };
        let a3481 = 2147483648u64;
        Ok(nat_lt(a3480, a3481))
    } else {
        Ok(false)
    }
}

pub fn f126(v0: u64) -> R<u64> {
    let a3484 = 3u64;
    let a3485 = {
        let a3482 = v0;
        let a3483 = 3u64;
        nat_mul(a3482, a3483)?
    };
    nat_add(a3484, a3485)
}

pub fn f127(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h3486, _, _, _, _, _, _, _) => {
            h3486
        }
    }, match v0.clone() {
        Adt3::C0(_, h3487, _, _, _, _, _, _) => {
            h3487
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3488, _, _, _, _, _) => {
            h3488
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3489, _, _, _, _) => {
            h3489
        }
    }) {
        Ok(0u64)
    } else {
        let a3498 = match v0.clone() {
            Adt3::C0(h3490, _, _, _, _, _, _, _) => {
                h3490
            }
        };
        let a3499 = {
            let a3496 = match v0.clone() {
                Adt3::C0(_, h3491, _, _, _, _, _, _) => {
                    h3491
                }
            };
            let a3497 = {
                let a3494 = match v0.clone() {
                    Adt3::C0(_, _, h3492, _, _, _, _, _) => {
                        h3492
                    }
                };
                let a3495 = match v0 {
                    Adt3::C0(_, _, _, h3493, _, _, _, _) => {
                        h3493
                    }
                };
                nat_mul(a3494, a3495)?
            };
            nat_mul(a3496, a3497)?
        };
        nat_mul(a3498, a3499)
    }
}

pub fn f128(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3500 = v1;
    let a3501 = 13u64;
    if if nat_eq(a3500, a3501) {
        f34(match v0.clone() {
            Adt3::C0(h3502, _, _, _, _, _, _, _) => {
                h3502
            }
        }, match v0.clone() {
            Adt3::C0(_, h3503, _, _, _, _, _, _) => {
                h3503
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3504, _, _, _, _, _) => {
                h3504
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3505, _, _, _, _) => {
                h3505
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3511 = match v0.clone() {
                Adt3::C0(_, h3506, _, _, _, _, _, _) => {
                    h3506
                }
            };
            let a3512 = {
                let a3509 = match v0.clone() {
                    Adt3::C0(_, _, h3507, _, _, _, _, _) => {
                        h3507
                    }
                };
                let a3510 = match v0.clone() {
                    Adt3::C0(_, _, _, h3508, _, _, _, _) => {
                        h3508
                    }
                };
                nat_mul(a3509, a3510)?
            };
            nat_mul(a3511, a3512)?
        };
        let a3513 = v1;
        let a3514 = 13u64;
        if nat_eq(a3513, a3514) {
            let a3523 = 2u64;
            let a3524 = if f194(v0.clone(), v2) {
                let a3516 = v3;
                let a3517 = f195(match v0 {
                    Adt3::C0(h3515, _, _, _, _, _, _, _) => {
                        h3515
                    }
                }, false)?;
                nat_mul(a3516, a3517)?
            } else {
                let a3521 = v3;
                let a3522 = {
                    let a3519 = match v0 {
                        Adt3::C0(h3518, _, _, _, _, _, _, _) => {
                            h3518
                        }
                    };
                    let a3520 = 2u64;
                    nat_mul(a3519, a3520)?
                };
                nat_mul(a3521, a3522)?
            };
            nat_add(a3523, a3524)
        } else {
            let a3528 = v3;
            let a3529 = f195(match v0 {
                Adt3::C0(h3525, _, _, _, _, _, _, _) => {
                    h3525
                }
            }, {
                let a3526 = v1;
                let a3527 = 16u64;
                nat_eq(a3526, a3527)
            })?;
            nat_mul(a3528, a3529)
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3530 = v0;
    let a3531 = 30u64;
    if if nat_eq(a3530, a3531) {
        let a3534 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3532, _, _, _, _) => {
                h3532
            }
        };
        let a3535 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3533, _, _, _, _, _) => {
                h3533
            }
        };
        nat_lt(a3534, a3535)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3536 = v3;
        match m3536 {
            None => {
                f196(v0, v1, match v2.clone() {
                    Adt2::C0(h3537, _, _, _, _, _, _, _, _, _, _) => {
                        h3537
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3538, _, _, _, _, _, _, _, _, _, _) => {
                        h3538
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3539, _, _, _, _, _, _, _, _, _) => {
                        h3539
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3540, _, _, _, _, _, _, _, _) => {
                        h3540
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3541, _, _, _, _, _, _, _) => {
                        h3541
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3542, _, _, _, _, _, _) => {
                        h3542
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3543, _, _, _, _, _, _, _, _, _) => {
                        h3543
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3544, _, _, _, _, _, _, _, _) => {
                        h3544
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3545, _, _, _, _, _, _, _) => {
                        h3545
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3546, _, _, _, _, _, _) => {
                        h3546
                    }
                })
            }
            Some(v5) => {
                f196(v0, v1, match v2.clone() {
                    Adt2::C0(h3547, _, _, _, _, _, _, _, _, _, _) => {
                        h3547
                    }
                }, match v5.clone() {
                    Adt2::C0(h3548, _, _, _, _, _, _, _, _, _, _) => {
                        h3548
                    }
                }, match v4.clone() {
                    Adt2::C0(h3549, _, _, _, _, _, _, _, _, _, _) => {
                        h3549
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3550, _, _, _, _, _, _, _, _, _) => {
                        h3550
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3551, _, _, _, _, _, _, _, _) => {
                        h3551
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3552, _, _, _, _, _, _, _) => {
                        h3552
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3553, _, _, _, _, _, _) => {
                        h3553
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3554, _, _, _, _, _, _, _, _, _) => {
                        h3554
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3555, _, _, _, _, _, _, _, _) => {
                        h3555
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3556, _, _, _, _, _, _, _) => {
                        h3556
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3557, _, _, _, _, _, _) => {
                        h3557
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3558, _, _, _, _, _, _, _, _, _) => {
                        h3558
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3559, _, _, _, _, _, _, _, _) => {
                        h3559
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3560, _, _, _, _, _, _, _) => {
                        h3560
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3561, _, _, _, _, _, _) => {
                        h3561
                    }
                })
            }
        }
    }
}

pub fn f130(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3574 = {
        let a3570 = {
            let a3566 = {
                let a3562 = v0;
                let a3563 = v4;
                nat_mul(a3562, a3563)?
            };
            let a3567 = {
                let a3564 = v1;
                let a3565 = v5;
                nat_mul(a3564, a3565)?
            };
            nat_add(a3566, a3567)?
        };
        let a3571 = {
            let a3568 = v2;
            let a3569 = v6;
            nat_mul(a3568, a3569)?
        };
        nat_add(a3570, a3571)?
    };
    let a3575 = {
        let a3572 = v3;
        let a3573 = v7;
        nat_mul(a3572, a3573)?
    };
    nat_add(a3574, a3575)
}

pub fn f131(v0: u64) -> u64 {
    let a3576 = v0;
    let a3577 = 2147483648u64;
    if nat_lt(a3576, a3577) {
        0u64
    } else {
        let a3578 = 4294967296u64;
        let a3579 = v0;
        nat_sub(a3578, a3579)
    }
}

pub fn f132(v0: u64) -> u64 {
    let a3580 = v0;
    let a3581 = 2147483648u64;
    if nat_lt(a3580, a3581) {
        v0
    } else {
        0u64
    }
}

pub fn f133(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3582 = v7;
        let a3583 = 6u64;
        if nat_eq(a3582, a3583) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f90(v3, v4), f90(v5, v6), f91(v7, v8), f197(v1), f197(v9))
}

pub fn f134(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3584 = v0;
    match m3584 {
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
            let a3585 = 13u64;
            let a3586 = v36;
            if nat_le(a3585, a3586) {
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
                Adt5::C0(_, _, _, _, _, h3587, _, _) => {
                    h3587
                }
            }, match v56 {
                Adt5::C0(_, _, _, _, _, _, _, h3588) => {
                    h3588
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

pub fn f135(v0: Adt4) -> u64 {
    let m3589 = v0;
    match m3589 {
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

pub fn f136(v0: Adt4, v1: u64) -> u64 {
    let m3590 = v0;
    match m3590 {
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
            let a3591 = v1;
            let a3592 = 6u64;
            if nat_eq(a3591, a3592) {
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
            let a3601 = v1;
            let a3602 = 6u64;
            if if if nat_eq(a3601, a3602) {
                true
            } else {
                let a3603 = v1;
                let a3604 = 7u64;
                nat_eq(a3603, a3604)
            } {
                let a3605 = v56;
                let a3606 = 30u64;
                nat_eq(a3605, a3606)
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
                Adt5::C0(_, _, _, _, _, _, _, h3600) => {
                    h3600
                }
            }
        }
        Adt4::C28(v51) => {
            let a3593 = v1;
            let a3594 = 6u64;
            if nat_eq(a3593, a3594) {
                match v51 {
                    Adt7::C0(_, _, _, _, h3595) => {
                        h3595
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
            let a3596 = v1;
            let a3597 = 6u64;
            if if nat_eq(a3596, a3597) {
                true
            } else {
                let a3598 = v1;
                let a3599 = 7u64;
                nat_eq(a3598, a3599)
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

pub fn f137(v0: Adt4, v1: u64) -> Adt11 {
    let a3607 = v1;
    let a3608 = 4u64;
    if nat_lt(a3607, a3608) {
        Adt11::C0(v1)
    } else {
        let a3609 = v1;
        let a3610 = 4u64;
        if nat_eq(a3609, a3610) {
            Adt11::C1(0u64)
        } else {
            let a3611 = v1;
            let a3612 = 5u64;
            if nat_eq(a3611, a3612) {
                Adt11::C1(3u64)
            } else {
                let a3613 = v1;
                let a3614 = 6u64;
                if nat_eq(a3613, a3614) {
                    Adt11::C2(f136(v0, v1), 0u64, false)
                } else {
                    let a3615 = v1;
                    let a3616 = 7u64;
                    if nat_eq(a3615, a3616) {
                        Adt11::C2(0u64, f136(v0, v1), false)
                    } else {
                        let a3617 = v1;
                        let a3618 = 16u64;
                        if nat_lt(a3617, a3618) {
                            Adt11::C1({
                                let a3619 = v1;
                                let a3620 = 8u64;
                                nat_sub(a3619, a3620)
                            })
                        } else {
                            let a3621 = v1;
                            let a3622 = 20u64;
                            if nat_lt(a3621, a3622) {
                                Adt11::C5({
                                    let a3623 = v1;
                                    let a3624 = 16u64;
                                    nat_sub(a3623, a3624)
                                })
                            } else {
                                let a3625 = v1;
                                let a3626 = 20u64;
                                if nat_eq(a3625, a3626) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3627 = v1;
                                    let a3628 = 21u64;
                                    if nat_eq(a3627, a3628) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3629 = v1;
                                        let a3630 = 22u64;
                                        if nat_eq(a3629, a3630) {
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

pub fn f138(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3632 = v0;
        let a3633 = match v1.clone() {
            Adt3::C0(h3631, _, _, _, _, _, _, _) => {
                h3631
            }
        };
        let a3634 = 0u64;
        nat_quot(a3632, a3633, a3634)
    };
    let v3: u64 = {
        let a3636 = v2;
        let a3637 = match v1.clone() {
            Adt3::C0(_, h3635, _, _, _, _, _, _) => {
                h3635
            }
        };
        let a3638 = 0u64;
        nat_quot(a3636, a3637, a3638)
    };
    f130({
        let a3640 = v0;
        let a3641 = match v1.clone() {
            Adt3::C0(h3639, _, _, _, _, _, _, _) => {
                h3639
            }
        };
        let a3642 = 0u64;
        nat_rem(a3640, a3641, a3642)
    }, {
        let a3644 = v2;
        let a3645 = match v1.clone() {
            Adt3::C0(_, h3643, _, _, _, _, _, _) => {
                h3643
            }
        };
        let a3646 = 0u64;
        nat_rem(a3644, a3645, a3646)
    }, {
        let a3648 = v3;
        let a3649 = match v1.clone() {
            Adt3::C0(_, _, h3647, _, _, _, _, _) => {
                h3647
            }
        };
        let a3650 = 0u64;
        nat_rem(a3648, a3649, a3650)
    }, {
        let a3652 = v3;
        let a3653 = match v1.clone() {
            Adt3::C0(_, _, h3651, _, _, _, _, _) => {
                h3651
            }
        };
        let a3654 = 0u64;
        nat_quot(a3652, a3653, a3654)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3655, _, _, _) => {
            h3655
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3656, _, _) => {
            h3656
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3657, _) => {
            h3657
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3658) => {
            h3658
        }
    })
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3676 = {
        let a3671 = f198(v2, {
            let a3662 = {
                let a3659 = v0;
                let a3660 = v1;
                let a3661 = 0u64;
                nat_quot(a3659, a3660, a3661)
            };
            let a3663 = v2;
            let a3664 = 0u64;
            nat_rem(a3662, a3663, a3664)
        }, {
            let a3668 = {
                let a3665 = v0;
                let a3666 = v1;
                let a3667 = 0u64;
                nat_quot(a3665, a3666, a3667)
            };
            let a3669 = v2;
            let a3670 = 0u64;
            nat_quot(a3668, a3669, a3670)
        })?;
        let a3672 = v1;
        nat_mul(a3671, a3672)?
    };
    let a3677 = {
        let a3673 = v0;
        let a3674 = v1;
        let a3675 = 0u64;
        nat_rem(a3673, a3674, a3675)
    };
    nat_add(a3676, a3677)
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3679 = v0;
        let a3680 = match v2.clone() {
            Adt3::C0(h3678, _, _, _, _, _, _, _) => {
                h3678
            }
        };
        let a3681 = 0u64;
        nat_quot(a3679, a3680, a3681)
    };
    let v4: u64 = {
        let a3683 = v3;
        let a3684 = match v2.clone() {
            Adt3::C0(_, h3682, _, _, _, _, _, _) => {
                h3682
            }
        };
        let a3685 = 0u64;
        nat_quot(a3683, a3684, a3685)
    };
    f199({
        let a3687 = v0;
        let a3688 = match v2.clone() {
            Adt3::C0(h3686, _, _, _, _, _, _, _) => {
                h3686
            }
        };
        let a3689 = 0u64;
        nat_rem(a3687, a3688, a3689)
    }, {
        let a3691 = v3;
        let a3692 = match v2.clone() {
            Adt3::C0(_, h3690, _, _, _, _, _, _) => {
                h3690
            }
        };
        let a3693 = 0u64;
        nat_rem(a3691, a3692, a3693)
    }, {
        let a3695 = v4;
        let a3696 = match v2.clone() {
            Adt3::C0(_, _, h3694, _, _, _, _, _) => {
                h3694
            }
        };
        let a3697 = 0u64;
        nat_rem(a3695, a3696, a3697)
    }, {
        let a3699 = v4;
        let a3700 = match v2 {
            Adt3::C0(_, _, h3698, _, _, _, _, _) => {
                h3698
            }
        };
        let a3701 = 0u64;
        nat_quot(a3699, a3700, a3701)
    }, match v1.clone() {
        Adt3::C0(h3702, _, _, _, _, _, _, _) => {
            h3702
        }
    }, match v1.clone() {
        Adt3::C0(_, h3703, _, _, _, _, _, _) => {
            h3703
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3704, _, _, _, _, _) => {
            h3704
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3705, _, _, _, _) => {
            h3705
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3706, _, _, _) => {
            h3706
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3707, _, _) => {
            h3707
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3708, _) => {
            h3708
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3709) => {
            h3709
        }
    })
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3711 = v0;
        let a3712 = match v3.clone() {
            Adt3::C0(h3710, _, _, _, _, _, _, _) => {
                h3710
            }
        };
        let a3713 = 0u64;
        nat_quot(a3711, a3712, a3713)
    };
    let v6: u64 = {
        let a3715 = v5;
        let a3716 = match v3.clone() {
            Adt3::C0(_, h3714, _, _, _, _, _, _) => {
                h3714
            }
        };
        let a3717 = 0u64;
        nat_quot(a3715, a3716, a3717)
    };
    let v7: u64 = {
        let a3719 = v0;
        let a3720 = match v3.clone() {
            Adt3::C0(h3718, _, _, _, _, _, _, _) => {
                h3718
            }
        };
        let a3721 = 0u64;
        nat_rem(a3719, a3720, a3721)
    };
    let v8: u64 = {
        let a3723 = v5;
        let a3724 = match v3.clone() {
            Adt3::C0(_, h3722, _, _, _, _, _, _) => {
                h3722
            }
        };
        let a3725 = 0u64;
        nat_rem(a3723, a3724, a3725)
    };
    let v9: u64 = {
        let a3727 = v6;
        let a3728 = match v3.clone() {
            Adt3::C0(_, _, h3726, _, _, _, _, _) => {
                h3726
            }
        };
        let a3729 = 0u64;
        nat_rem(a3727, a3728, a3729)
    };
    let v10: u64 = {
        let a3731 = v6;
        let a3732 = match v3.clone() {
            Adt3::C0(_, _, h3730, _, _, _, _, _) => {
                h3730
            }
        };
        let a3733 = 0u64;
        nat_quot(a3731, a3732, a3733)
    };
    let v11: u64 = {
        let a3734 = v4;
        let a3735 = 0u64;
        if nat_eq(a3734, a3735) {
            match v1.clone() {
                Adt3::C0(h3736, _, _, _, _, _, _, _) => {
                    h3736
                }
            }
        } else {
            let a3737 = v4;
            let a3738 = 1u64;
            if nat_eq(a3737, a3738) {
                match v1.clone() {
                    Adt3::C0(_, h3739, _, _, _, _, _, _) => {
                        h3739
                    }
                }
            } else {
                let a3740 = v4;
                let a3741 = 2u64;
                if nat_eq(a3740, a3741) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3742, _, _, _, _, _) => {
                            h3742
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3743, _, _, _, _) => {
                            h3743
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3750 = v11;
        let a3751 = {
            let a3744 = v4;
            let a3745 = 0u64;
            if nat_eq(a3744, a3745) {
                v7
            } else {
                let a3746 = v4;
                let a3747 = 1u64;
                if nat_eq(a3746, a3747) {
                    v8
                } else {
                    let a3748 = v4;
                    let a3749 = 2u64;
                    if nat_eq(a3748, a3749) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3750, a3751)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f130(if if v12 {
        let a3752 = v4;
        let a3753 = 0u64;
        nat_eq(a3752, a3753)
    } else {
        false
    } {
        let a3754 = v7;
        let a3755 = v11;
        nat_sub(a3754, a3755)
    } else {
        v7
    }, if if v12 {
        let a3756 = v4;
        let a3757 = 1u64;
        nat_eq(a3756, a3757)
    } else {
        false
    } {
        let a3758 = v8;
        let a3759 = v11;
        nat_sub(a3758, a3759)
    } else {
        v8
    }, if if v12 {
        let a3760 = v4;
        let a3761 = 2u64;
        nat_eq(a3760, a3761)
    } else {
        false
    } {
        let a3762 = v9;
        let a3763 = v11;
        nat_sub(a3762, a3763)
    } else {
        v9
    }, if if v12 {
        let a3764 = v4;
        let a3765 = 3u64;
        nat_eq(a3764, a3765)
    } else {
        false
    } {
        let a3766 = v10;
        let a3767 = v11;
        nat_sub(a3766, a3767)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3768, _, _, _) => {
            h3768
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3769, _, _) => {
            h3769
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3770, _) => {
            h3770
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3771) => {
            h3771
        }
    })?, f138(v0, v3)?))
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f70(v0, v2.clone(), 0u64);
    let v9: u64 = f70(v0, v2.clone(), 1u64);
    let v10: u64 = f70(v0, v2.clone(), 2u64);
    let v11: u64 = f70(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f69(v1.clone(), f200(v8, v3, match v1.clone() {
            Adt3::C0(h3772, _, _, _, _, _, _, _) => {
                h3772
            }
        })?, f200(v9, v4, match v1.clone() {
            Adt3::C0(_, h3773, _, _, _, _, _, _) => {
                h3773
            }
        })?, f200(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3774, _, _, _, _, _) => {
                h3774
            }
        })?, f200(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3775, _, _, _, _) => {
                h3775
            }
        })?)?, f138(v0, v2)?))
    } else {
        if if if if f201(v8, v3, match v1.clone() {
            Adt3::C0(h3776, _, _, _, _, _, _, _) => {
                h3776
            }
        })? {
            f201(v9, v4, match v1.clone() {
                Adt3::C0(_, h3777, _, _, _, _, _, _) => {
                    h3777
                }
            })?
        } else {
            false
        } {
            f201(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3778, _, _, _, _, _) => {
                    h3778
                }
            })?
        } else {
            false
        } {
            f201(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3779, _, _, _, _) => {
                    h3779
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f69(v1, f202(v8, v3)?, f202(v9, v4)?, f202(v10, v5)?, f202(v11, v6)?)?, f138(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f138(v0, v2)?))
        }
    }
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f70(v0, v2.clone(), 0u64);
    let v5: u64 = f70(v0, v2.clone(), 1u64);
    let v6: u64 = f70(v0, v2.clone(), 2u64);
    let v7: u64 = f70(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f69(v1.clone(), f203(v4, v3, match v1 {
        Adt3::C0(h3780, _, _, _, _, _, _, _) => {
            h3780
        }
    })?, v5, v6, v7)?, f138(v0, v2)?))
}

pub fn f144(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3781 = v0;
    let a3782 = v3;
    if nat_lt(a3781, a3782) {
        Ok(Adt9::C1(false, {
            let a3783 = v0;
            let a3784 = 4u64;
            nat_mul(a3783, a3784)?
        }, {
            let a3785 = v0;
            let a3786 = 4u64;
            nat_mul(a3785, a3786)?
        }))
    } else {
        let v6: u64 = {
            let a3787 = v0;
            let a3788 = v3;
            nat_sub(a3787, a3788)
        };
        let v7: u64 = f138(v6, v1)?;
        let v8: u64 = f204(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3789 = v0;
    let a3790 = v3;
    if nat_lt(a3789, a3790) {
        Ok(Adt9::C1(true, 0u64, f138(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3791 = v0;
            let a3792 = v3;
            nat_sub(a3791, a3792)
        };
        let v5: u64 = f140(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f138(v4, v1)?, 3u64, v5))
    }
}

pub fn f146(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3794 = v1;
        let a3795 = match v3.clone() {
            Adt3::C0(h3793, _, _, _, _, _, _, _) => {
                h3793
            }
        };
        let a3796 = 0u64;
        nat_rem(a3794, a3795, a3796)
    };
    let v5: u64 = {
        let a3802 = {
            let a3798 = v1;
            let a3799 = match v3.clone() {
                Adt3::C0(h3797, _, _, _, _, _, _, _) => {
                    h3797
                }
            };
            let a3800 = 0u64;
            nat_quot(a3798, a3799, a3800)
        };
        let a3803 = match v3.clone() {
            Adt3::C0(_, h3801, _, _, _, _, _, _) => {
                h3801
            }
        };
        let a3804 = 0u64;
        nat_rem(a3802, a3803, a3804)
    };
    let v6: bool = f205(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f140(v1, v2, v3.clone())?
    } else {
        0u64
    }, f138(v1, v3)?))
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3805 = v3;
        let a3806 = 8u64;
        nat_add(a3805, a3806)?
    }, 0u64, f138(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f140(v0, v5, v2.clone())?
    }, 3u64, f138(v0, v2)?))
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3807 = v4;
    let a3808 = 0u64;
    if if nat_eq(a3807, a3808) {
        f50(v3)
    } else {
        false
    } {
        let m3809 = f114(v3);
        match m3809 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h3810, _) => {
                        h3810
                    }
                };
                Ok(Adt9::C5(v3, f138(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h3811) => {
                        h3811
                    }
                }, f138({
                    let a3812 = v0;
                    let a3813 = v6;
                    nat_mul(a3812, a3813)?
                }, v2.clone())?, {
                    let a3819 = {
                        let a3817 = {
                            let a3814 = v6;
                            let a3815 = 1u64;
                            nat_sub(a3814, a3815)
                        };
                        let a3818 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3816, _, _, _) => {
                                h3816
                            }
                        };
                        nat_mul(a3817, a3818)?
                    };
                    let a3820 = 4u64;
                    nat_add(a3819, a3820)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3821, _, _, _) => {
                        h3821
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f138(v0, v1)?, f138(v0, v2)?))
    }
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3823 = v0;
        let a3824 = match v2.clone() {
            Adt3::C0(h3822, _, _, _, _, _, _, _) => {
                h3822
            }
        };
        let a3825 = 0u64;
        nat_rem(a3823, a3824, a3825)
    };
    let v7: u64 = {
        let a3827 = v0;
        let a3828 = match v2.clone() {
            Adt3::C0(h3826, _, _, _, _, _, _, _) => {
                h3826
            }
        };
        let a3829 = 0u64;
        nat_quot(a3827, a3828, a3829)
    };
    let v8: u64 = {
        let a3831 = v7;
        let a3832 = match v2.clone() {
            Adt3::C0(_, h3830, _, _, _, _, _, _) => {
                h3830
            }
        };
        let a3833 = 0u64;
        nat_rem(a3831, a3832, a3833)
    };
    let v9: u64 = {
        let a3835 = v7;
        let a3836 = match v2.clone() {
            Adt3::C0(_, h3834, _, _, _, _, _, _) => {
                h3834
            }
        };
        let a3837 = 0u64;
        nat_quot(a3835, a3836, a3837)
    };
    let v10: u64 = {
        let a3839 = v9;
        let a3840 = match v2.clone() {
            Adt3::C0(_, _, h3838, _, _, _, _, _) => {
                h3838
            }
        };
        let a3841 = 0u64;
        nat_rem(a3839, a3840, a3841)
    };
    let v11: u64 = {
        let a3843 = v9;
        let a3844 = match v2.clone() {
            Adt3::C0(_, _, h3842, _, _, _, _, _) => {
                h3842
            }
        };
        let a3845 = 0u64;
        nat_quot(a3843, a3844, a3845)
    };
    let v12: u64 = f138(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3854 = {
                let a3849 = {
                    let a3846 = v10;
                    let a3847 = v4;
                    let a3848 = 0u64;
                    nat_quot(a3846, a3847, a3848)
                };
                let a3850 = v5;
                nat_mul(a3849, a3850)?
            };
            let a3855 = {
                let a3851 = v8;
                let a3852 = v4;
                let a3853 = 0u64;
                nat_quot(a3851, a3852, a3853)
            };
            nat_add(a3854, a3855)?
        };
        let v14: u64 = {
            let a3875 = v6;
            let a3876 = {
                let a3873 = match v1.clone() {
                    Adt3::C0(h3856, _, _, _, _, _, _, _) => {
                        h3856
                    }
                };
                let a3874 = {
                    let a3871 = {
                        let a3857 = v8;
                        let a3858 = v4;
                        let a3859 = 0u64;
                        nat_rem(a3857, a3858, a3859)
                    };
                    let a3872 = {
                        let a3869 = match v1.clone() {
                            Adt3::C0(_, h3860, _, _, _, _, _, _) => {
                                h3860
                            }
                        };
                        let a3870 = {
                            let a3867 = {
                                let a3861 = v10;
                                let a3862 = v4;
                                let a3863 = 0u64;
                                nat_rem(a3861, a3862, a3863)
                            };
                            let a3868 = {
                                let a3865 = match v1.clone() {
                                    Adt3::C0(_, _, h3864, _, _, _, _, _) => {
                                        h3864
                                    }
                                };
                                let a3866 = v13;
                                nat_mul(a3865, a3866)?
                            };
                            nat_add(a3867, a3868)?
                        };
                        nat_mul(a3869, a3870)?
                    };
                    nat_add(a3871, a3872)?
                };
                nat_mul(a3873, a3874)?
            };
            nat_add(a3875, a3876)?
        };
        Ok(Adt9::C1(false, f138(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3882 = {
                let a3880 = {
                    let a3877 = v11;
                    let a3878 = v5;
                    let a3879 = 0u64;
                    nat_rem(a3877, a3878, a3879)
                };
                let a3881 = v4;
                nat_mul(a3880, a3881)?
            };
            let a3883 = v8;
            nat_add(a3882, a3883)?
        };
        let v16: u64 = {
            let a3889 = {
                let a3887 = {
                    let a3884 = v11;
                    let a3885 = v5;
                    let a3886 = 0u64;
                    nat_quot(a3884, a3885, a3886)
                };
                let a3888 = v4;
                nat_mul(a3887, a3888)?
            };
            let a3890 = v10;
            nat_add(a3889, a3890)?
        };
        let a3892 = v15;
        let a3893 = match v1.clone() {
            Adt3::C0(_, h3891, _, _, _, _, _, _) => {
                h3891
            }
        };
        if if nat_lt(a3892, a3893) {
            let a3895 = v16;
            let a3896 = match v1.clone() {
                Adt3::C0(_, _, h3894, _, _, _, _, _) => {
                    h3894
                }
            };
            nat_lt(a3895, a3896)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f138({
                let a3905 = v6;
                let a3906 = {
                    let a3903 = match v1.clone() {
                        Adt3::C0(h3897, _, _, _, _, _, _, _) => {
                            h3897
                        }
                    };
                    let a3904 = {
                        let a3901 = v15;
                        let a3902 = {
                            let a3899 = match v1.clone() {
                                Adt3::C0(_, h3898, _, _, _, _, _, _) => {
                                    h3898
                                }
                            };
                            let a3900 = v16;
                            nat_mul(a3899, a3900)?
                        };
                        nat_add(a3901, a3902)?
                    };
                    nat_mul(a3903, a3904)?
                };
                nat_add(a3905, a3906)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f138(v0, v2.clone())?;
    let v4: u64 = f138(v0, v1.clone())?;
    let a3911 = {
        let a3908 = v0;
        let a3909 = match v1 {
            Adt3::C0(h3907, _, _, _, _, _, _, _) => {
                h3907
            }
        };
        let a3910 = 0u64;
        nat_rem(a3908, a3909, a3910)
    };
    let a3912 = 0u64;
    if nat_eq(a3911, a3912) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f138({
            let a3913 = v0;
            let a3914 = 1u64;
            nat_sub(a3913, a3914)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3915 = v3;
    let a3916 = 13u64;
    if nat_eq(a3915, a3916) {
        f206(v0, v1, v4)
    } else {
        let v5: u64 = f195(match v1.clone() {
            Adt3::C0(h3917, _, _, _, _, _, _, _) => {
                h3917
            }
        }, {
            let a3918 = v3;
            let a3919 = 16u64;
            nat_eq(a3918, a3919)
        })?;
        let v6: u64 = {
            let a3920 = v0;
            let a3921 = v5;
            let a3922 = 0u64;
            nat_quot(a3920, a3921, a3922)
        };
        let v7: u64 = {
            let a3923 = v0;
            let a3924 = v5;
            let a3925 = 0u64;
            nat_rem(a3923, a3924, a3925)
        };
        let v8: u64 = {
            let a3929 = 1u64;
            let a3930 = {
                let a3927 = match v1.clone() {
                    Adt3::C0(h3926, _, _, _, _, _, _, _) => {
                        h3926
                    }
                };
                let a3928 = 2u64;
                nat_mul(a3927, a3928)?
            };
            nat_add(a3929, a3930)?
        };
        let a3931 = v7;
        let a3932 = v8;
        if nat_lt(a3931, a3932) {
            f207(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f138(v6, v2)?;
            let a3933 = v3;
            let a3934 = 14u64;
            if nat_eq(a3933, a3934) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3935 = v7;
                let a3936 = v8;
                if nat_eq(a3935, a3936) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3939 = v7;
                    let a3940 = {
                        let a3937 = v8;
                        let a3938 = 1u64;
                        nat_add(a3937, a3938)?
                    };
                    if nat_eq(a3939, a3940) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3941, _, _, _, _, _, _, _) => {
                                h3941
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

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3942 = v0;
    let a3943 = 0u64;
    if nat_eq(a3942, a3943) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3950 = {
            let a3944 = v0;
            let a3945 = 1u64;
            nat_add(a3944, a3945)?
        };
        let a3951 = {
            let a3948 = 2u64;
            let a3949 = {
                let a3946 = v3;
                let a3947 = 2u64;
                nat_mul(a3946, a3947)?
            };
            nat_add(a3948, a3949)?
        };
        if nat_eq(a3950, a3951) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3952 = v0;
                let a3953 = 1u64;
                nat_sub(a3952, a3953)
            };
            let v5: u64 = {
                let a3954 = v4;
                let a3955 = 2u64;
                let a3956 = 0u64;
                nat_quot(a3954, a3955, a3956)
            };
            let a3960 = {
                let a3957 = v4;
                let a3958 = 2u64;
                let a3959 = 0u64;
                nat_rem(a3957, a3958, a3959)
            };
            let a3961 = 0u64;
            if nat_eq(a3960, a3961) {
                Ok(Adt9::C4(17u64, 0u64, f138(v5, v1)?, 4u64, f138(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f153(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3962 = v0;
    let a3963 = f27(v1, v2, v3)?;
    if nat_lt(a3962, a3963) {
        let a3964 = v0;
        let a3965 = v1;
        if nat_lt(a3964, a3965) {
            Ok(Adt9::C1(false, {
                let a3966 = 4u64;
                let a3967 = v0;
                nat_mul(a3966, a3967)?
            }, {
                let a3968 = 4u64;
                let a3969 = v0;
                nat_mul(a3968, a3969)?
            }))
        } else {
            let v4: u64 = {
                let a3970 = v0;
                let a3971 = v1;
                nat_sub(a3970, a3971)
            };
            let v5: u64 = f208(v4, v2)?;
            let a3972 = v5;
            let a3973 = 2u64;
            if nat_lt(a3972, a3973) {
                Ok(Adt9::C4(6u64, {
                    let a3974 = v5;
                    let a3975 = 0u64;
                    if nat_eq(a3974, a3975) {
                        4u64
                    } else {
                        5u64
                    }
                }, f209(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f210(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f211(v4, v2)? {
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

pub fn f154(v0: u64, v1: u64) -> Adt11 {
    let a3976 = v0;
    let a3977 = 43u64;
    if nat_eq(a3976, a3977) {
        Adt11::C8(3u64)
    } else {
        let a3978 = v1;
        let a3979 = 43u64;
        if nat_eq(a3978, a3979) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f155(v0: u64, v1: u64) -> R<u64> {
    if f104(v0) {
        let a3980 = 2u64;
        let a3981 = v1;
        nat_mul(a3980, a3981)
    } else {
        let a3982 = v1;
        let a3983 = f191(v0);
        let a3984 = 0u64;
        Ok(nat_quot(a3982, a3983, a3984))
    }
}

pub fn f156(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m3985 = f114(v0);
    match m3985 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f130(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h3986, _, _, _) => {
                    h3986
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h3987, _, _) => {
                    h3987
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h3988, _) => {
                    h3988
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h3989) => {
                    h3989
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h3990) => {
                    h3990
                }
            }, f130({
                let a3992 = v2;
                let a3993 = match v6.clone() {
                    Adt18::C0(_, h3991, _) => {
                        h3991
                    }
                };
                nat_mul(a3992, a3993)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h3994, _, _, _) => {
                    h3994
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h3995, _, _) => {
                    h3995
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h3996, _) => {
                    h3996
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h3997) => {
                    h3997
                }
            })?, match v6 {
                Adt18::C0(_, h3998, _) => {
                    h3998
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h3999, _, _, _) => {
                    h3999
                }
            }))
        }
    }
}

pub fn f157(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4007 = {
        let a4000 = 2u64;
        let a4001 = f127(v1.clone())?;
        nat_mul(a4000, a4001)?
    };
    let a4008 = {
        let a4005 = f127(v1)?;
        let a4006 = {
            let a4003 = 1u64;
            let a4004 = match v0 {
                Adt3::C0(h4002, _, _, _, _, _, _, _) => {
                    h4002
                }
            };
            nat_add(a4003, a4004)?
        };
        nat_mul(a4005, a4006)?
    };
    nat_add(a4007, a4008)
}

pub fn f158(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f138(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f159(v0: u64, v1: u64) -> bool {
    let a4009 = v1;
    let a4010 = 2147483647u64;
    if nat_le(a4009, a4010) {
        let a4011 = v1;
        let a4012 = v0;
        nat_lt(a4011, a4012)
    } else {
        false
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4013 = f114(v0);
    match m4013 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4015 = v2;
                let a4016 = match v7.clone() {
                    Adt18::C0(_, h4014, _) => {
                        h4014
                    }
                };
                nat_mul(a4015, a4016)?
            };
            let m4029 = f212(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4017, _, _, _, _, _, _, _) => {
                    h4017
                }
            }, match v5.clone() {
                Adt3::C0(_, h4018, _, _, _, _, _, _) => {
                    h4018
                }
            }, match v5 {
                Adt3::C0(_, _, h4019, _, _, _, _, _) => {
                    h4019
                }
            }, {
                let a4022 = match v3.clone() {
                    Adt3::C0(h4020, _, _, _, _, _, _, _) => {
                        h4020
                    }
                };
                let a4023 = match v7.clone() {
                    Adt18::C0(_, h4021, _) => {
                        h4021
                    }
                };
                nat_mul(a4022, a4023)?
            }, match v3.clone() {
                Adt3::C0(_, h4024, _, _, _, _, _, _) => {
                    h4024
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4025, _, _, _) => {
                    h4025
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4026, _, _) => {
                    h4026
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4027, _) => {
                    h4027
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4028) => {
                    h4028
                }
            })?;
            match m4029 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4030 = v0;
                    let a4031 = 0u64;
                    if if nat_eq(a4030, a4031) {
                        true
                    } else {
                        let a4032 = v0;
                        let a4033 = 26u64;
                        nat_eq(a4032, a4033)
                    } {
                        Ok(Adt9::C1(false, v9, f138({
                            let a4037 = {
                                let a4035 = v1;
                                let a4036 = match v4.clone() {
                                    Adt3::C0(h4034, _, _, _, _, _, _, _) => {
                                        h4034
                                    }
                                };
                                nat_mul(a4035, a4036)?
                            };
                            let a4038 = v8;
                            nat_add(a4037, a4038)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4039) => {
                                h4039
                            }
                        }, f138({
                            let a4043 = {
                                let a4041 = v1;
                                let a4042 = match v4.clone() {
                                    Adt3::C0(h4040, _, _, _, _, _, _, _) => {
                                        h4040
                                    }
                                };
                                nat_mul(a4041, a4042)?
                            };
                            let a4044 = v8;
                            nat_add(a4043, a4044)?
                        }, v4.clone())?, {
                            let a4051 = {
                                let a4049 = {
                                    let a4046 = match v7 {
                                        Adt18::C0(_, h4045, _) => {
                                            h4045
                                        }
                                    };
                                    let a4047 = 1u64;
                                    nat_sub(a4046, a4047)
                                };
                                let a4050 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4048, _, _, _) => {
                                        h4048
                                    }
                                };
                                nat_mul(a4049, a4050)?
                            };
                            let a4052 = 4u64;
                            nat_add(a4051, a4052)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4053, _, _, _) => {
                                h4053
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f161(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4054 = v1;
    let a4055 = 0u64;
    if if nat_eq(a4054, a4055) {
        true
    } else {
        let a4056 = v2;
        let a4057 = 0u64;
        nat_eq(a4056, a4057)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f162(v0: u64, v1: u64) -> bool {
    let a4058 = v0;
    let a4059 = 4294967295u64;
    if nat_eq(a4058, a4059) {
        true
    } else {
        let a4060 = v0;
        let a4061 = 2147483648u64;
        if nat_lt(a4060, a4061) {
            let a4062 = v0;
            let a4063 = v1;
            nat_lt(a4062, a4063)
        } else {
            false
        }
    }
}

pub fn f163(v0: Adt7) -> R<u64> {
    let a4068 = if match v0.clone() {
        Adt7::C0(_, _, _, h4064, _) => {
            h4064
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4069 = {
        let a4066 = match v0 {
            Adt7::C0(_, _, _, _, h4065) => {
                h4065
            }
        };
        let a4067 = 1u64;
        if nat_eq(a4066, a4067) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4068, a4069)
}

pub fn f164(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4074 = {
            let a4071 = v2;
            let a4072 = match v0.clone() {
                Adt6::C0(_, h4070, _, _) => {
                    h4070
                }
            };
            nat_mul(a4071, a4072)?
        };
        let a4075 = f131(match v0.clone() {
            Adt6::C0(_, _, h4073, _) => {
                h4073
            }
        });
        nat_add(a4074, a4075)?
    };
    let v4: u64 = f132(match v0.clone() {
        Adt6::C0(_, _, h4076, _) => {
            h4076
        }
    });
    let v5: u64 = {
        let a4080 = {
            let a4078 = v3;
            let a4079 = match v0 {
                Adt6::C0(h4077, _, _, _) => {
                    h4077
                }
            };
            nat_add(a4078, a4079)?
        };
        let a4081 = v4;
        nat_sub(a4080, a4081)
    };
    let a4086 = {
        let a4082 = v5;
        let a4083 = v1;
        if nat_lt(a4082, a4083) {
            v5
        } else {
            v1
        }
    };
    let a4087 = {
        let a4084 = v3;
        let a4085 = v4;
        nat_sub(a4084, a4085)
    };
    Ok(nat_sub(a4086, a4087))
}

pub fn f165(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4096 = f72(match v0.clone() {
        Adt7::C0(h4088, _, _, _, _) => {
            h4088
        }
    }, match v1.clone() {
        Adt3::C0(h4089, _, _, _, _, _, _, _) => {
            h4089
        }
    }, f70(v3, v2.clone(), 0u64), {
        let a4093 = v4;
        let a4094 = {
            let m4092 = match v0.clone() {
                Adt7::C0(h4091, _, _, _, _) => {
                    h4091
                }
            };
            match m4092 {
                Adt6::C0(h4090, _, _, _) => {
                    h4090
                }
            }
        };
        let a4095 = 0u64;
        nat_rem(a4093, a4094, a4095)
    })?;
    match m4096 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4105 = f72(match v0.clone() {
                Adt7::C0(_, h4097, _, _, _) => {
                    h4097
                }
            }, match v1.clone() {
                Adt3::C0(_, h4098, _, _, _, _, _, _) => {
                    h4098
                }
            }, f70(v3, v2.clone(), 1u64), {
                let a4102 = v4;
                let a4103 = {
                    let m4101 = match v0 {
                        Adt7::C0(h4100, _, _, _, _) => {
                            h4100
                        }
                    };
                    match m4101 {
                        Adt6::C0(h4099, _, _, _) => {
                            h4099
                        }
                    }
                };
                let a4104 = 0u64;
                nat_quot(a4102, a4103, a4104)
            })?;
            match m4105 {
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

pub fn f166(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m4106 = v0;
    match m4106 {
        Adt16::C0(v19) => {
            f213(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4107 = v20;
            let a4108 = 66u64;
            if nat_eq(a4107, a4108) {
                f214(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f215(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f167(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4115 = v3;
        let a4116 = {
            let a4112 = f138({
                let a4109 = v0;
                let a4110 = 2u64;
                let a4111 = 0u64;
                nat_quot(a4109, a4110, a4111)
            }, v1)?;
            let a4113 = v2;
            let a4114 = 0u64;
            nat_quot(a4112, a4113, a4114)
        };
        nat_add(a4115, a4116)?
    };
    let a4120 = {
        let a4117 = v0;
        let a4118 = 2u64;
        let a4119 = 0u64;
        nat_rem(a4117, a4118, a4119)
    };
    let a4121 = 0u64;
    if nat_eq(a4120, a4121) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4122 = v4;
        let a4123 = 0u64;
        if nat_eq(a4122, a4123) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f168(v0: u64, v1: u64) -> R<u64> {
    let a4126 = 8u64;
    let a4127 = {
        let a4124 = f216(v0);
        let a4125 = v1;
        nat_mul(a4124, a4125)?
    };
    nat_add(a4126, a4127)
}

pub fn f169(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4128, _, _, _, _, _, _, _) => {
            h4128
        }
    };
    let v7: u64 = f216(v3);
    let v8: u64 = {
        let a4131 = 1u64;
        let a4132 = {
            let a4129 = v6;
            let a4130 = v7;
            nat_mul(a4129, a4130)?
        };
        nat_add(a4131, a4132)?
    };
    let a4133 = v0;
    let a4134 = 0u64;
    if nat_eq(a4133, a4134) {
        Ok(f217(0u64))
    } else {
        let a4135 = v0;
        let a4136 = v8;
        if nat_lt(a4135, a4136) {
            let v9: u64 = {
                let a4137 = v0;
                let a4138 = 1u64;
                nat_sub(a4137, a4138)
            };
            let v10: u64 = {
                let a4139 = v9;
                let a4140 = v7;
                let a4141 = 0u64;
                nat_rem(a4139, a4140, a4141)
            };
            let v11: u64 = f138({
                let a4147 = {
                    let a4142 = v1;
                    let a4143 = v6;
                    nat_mul(a4142, a4143)?
                };
                let a4148 = {
                    let a4144 = v9;
                    let a4145 = v7;
                    let a4146 = 0u64;
                    nat_quot(a4144, a4145, a4146)
                };
                nat_add(a4147, a4148)?
            }, v2)?;
            let a4149 = v3;
            let a4150 = 30u64;
            if nat_eq(a4149, a4150) {
                let a4151 = v10;
                let a4152 = 0u64;
                if nat_eq(a4151, a4152) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4153 = v10;
                    let a4154 = 1u64;
                    if nat_eq(a4153, a4154) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4155 = v10;
                        let a4156 = 2u64;
                        if nat_eq(a4155, a4156) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4157 = v10;
                let a4158 = 0u64;
                if nat_eq(a4157, a4158) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4159 = v10;
                    let a4160 = 1u64;
                    if nat_eq(a4159, a4160) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4161 = v0;
                let a4162 = v8;
                nat_sub(a4161, a4162)
            };
            let a4163 = v12;
            let a4164 = 0u64;
            if nat_eq(a4163, a4164) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4165 = v12;
                let a4166 = 1u64;
                if nat_eq(a4165, a4166) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4167 = v12;
                    let a4168 = 2u64;
                    if nat_eq(a4167, a4168) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4169 = v12;
                        let a4170 = 3u64;
                        if nat_eq(a4169, a4170) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4171 = v12;
                            let a4172 = 4u64;
                            if nat_eq(a4171, a4172) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4173 = v12;
                                let a4174 = 5u64;
                                if nat_eq(a4173, a4174) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4175 = v12;
                                    let a4176 = 6u64;
                                    if nat_eq(a4175, a4176) {
                                        if f218(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4177 = 3u64;
                                                let a4178 = v1;
                                                nat_add(a4177, a4178)?
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

pub fn f170(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4179 = v0;
        let a4180 = f102(v3);
        let a4181 = 0u64;
        nat_quot(a4179, a4180, a4181)
    };
    let v5: u64 = {
        let a4186 = 3u64;
        let a4187 = {
            let a4183 = v4;
            let a4184 = match v1.clone() {
                Adt3::C0(h4182, _, _, _, _, _, _, _) => {
                    h4182
                }
            };
            let a4185 = 0u64;
            nat_quot(a4183, a4184, a4185)
        };
        nat_add(a4186, a4187)?
    };
    let v6: u64 = f138(v4, v1)?;
    let v7: u64 = f138(v4, v2)?;
    let a4188 = v3;
    let a4189 = 30u64;
    if nat_eq(a4188, a4189) {
        let a4193 = {
            let a4190 = v0;
            let a4191 = 3u64;
            let a4192 = 0u64;
            nat_rem(a4190, a4191, a4192)
        };
        let a4194 = 0u64;
        if nat_eq(a4193, a4194) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4198 = {
                let a4195 = v0;
                let a4196 = 3u64;
                let a4197 = 0u64;
                nat_rem(a4195, a4196, a4197)
            };
            let a4199 = 1u64;
            if nat_eq(a4198, a4199) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f171(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4200 = v0;
        let a4201 = 8388608u64;
        let a4202 = 0u64;
        nat_quot(a4200, a4201, a4202)
    };
    let a4203 = v0;
    let a4204 = 2147483648u64;
    if if if nat_lt(a4203, a4204) {
        let a4205 = 127u64;
        let a4206 = v1;
        nat_le(a4205, a4206)
    } else {
        false
    } {
        let a4207 = v1;
        let a4208 = 190u64;
        nat_lt(a4207, a4208)
    } else {
        false
    } {
        let v2: u64 = {
            let a4212 = 8388608u64;
            let a4213 = {
                let a4209 = v0;
                let a4210 = 8388608u64;
                let a4211 = 0u64;
                nat_rem(a4209, a4210, a4211)
            };
            nat_add(a4212, a4213)?
        };
        let a4214 = v1;
        let a4215 = 150u64;
        if nat_lt(a4214, a4215) {
            let v3: u64 = f219({
                let a4216 = 150u64;
                let a4217 = v1;
                nat_sub(a4216, a4217)
            });
            let a4221 = {
                let a4218 = v2;
                let a4219 = v3;
                let a4220 = 0u64;
                nat_rem(a4218, a4219, a4220)
            };
            let a4222 = 0u64;
            if nat_eq(a4221, a4222) {
                let a4223 = v2;
                let a4224 = v3;
                let a4225 = 0u64;
                Ok(nat_quot(a4223, a4224, a4225))
            } else {
                Ok(0u64)
            }
        } else {
            let a4228 = f219({
                let a4226 = v1;
                let a4227 = 150u64;
                nat_sub(a4226, a4227)
            });
            let a4229 = v2;
            nat_mul(a4228, a4229)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f172(v0: Adt14, v1: Adt14) -> bool {
    let a4232 = match v0.clone() {
        Adt14::C0(_, h4230, _, _, _, _, _, _, _, _, _) => {
            h4230
        }
    };
    let a4233 = match v1.clone() {
        Adt14::C0(_, h4231, _, _, _, _, _, _, _, _, _) => {
            h4231
        }
    };
    if if if nat_eq(a4232, a4233) {
        let a4236 = match v0.clone() {
            Adt14::C0(_, _, h4234, _, _, _, _, _, _, _, _) => {
                h4234
            }
        };
        let a4237 = match v1.clone() {
            Adt14::C0(_, _, h4235, _, _, _, _, _, _, _, _) => {
                h4235
            }
        };
        nat_eq(a4236, a4237)
    } else {
        false
    } {
        let a4240 = match v0.clone() {
            Adt14::C0(_, _, _, h4238, _, _, _, _, _, _, _) => {
                h4238
            }
        };
        let a4241 = match v1.clone() {
            Adt14::C0(_, _, _, h4239, _, _, _, _, _, _, _) => {
                h4239
            }
        };
        nat_eq(a4240, a4241)
    } else {
        false
    } {
        let a4244 = match v0 {
            Adt14::C0(_, _, _, _, h4242, _, _, _, _, _, _) => {
                h4242
            }
        };
        let a4245 = match v1 {
            Adt14::C0(_, _, _, _, h4243, _, _, _, _, _, _) => {
                h4243
            }
        };
        nat_eq(a4244, a4245)
    } else {
        false
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4246 = v0;
    let a4247 = 0u64;
    if if if nat_eq(a4246, a4247) {
        true
    } else {
        let a4248 = v1;
        let a4249 = 0u64;
        nat_eq(a4248, a4249)
    } {
        true
    } else {
        let a4250 = v2;
        let a4251 = 0u64;
        nat_eq(a4250, a4251)
    } {
        true
    } else {
        let a4252 = v3;
        let a4253 = 0u64;
        nat_eq(a4252, a4253)
    }
}

pub fn f174(v0: Adt14, v1: Adt14) -> bool {
    if f180(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4254, _, _, _, _, _) => {
            h4254
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4255, _, _, _, _, _) => {
            h4255
        }
    }) {
        let a4257 = match v0 {
            Adt14::C0(_, h4256, _, _, _, _, _, _, _, _, _) => {
                h4256
            }
        };
        let a4258 = 2147483647u64;
        nat_le(a4257, a4258)
    } else {
        true
    }
}

pub fn f175(v0: Adt14) -> bool {
    let a4263 = {
        let a4260 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4259) => {
                h4259
            }
        };
        let a4261 = 4u64;
        let a4262 = 0u64;
        nat_rem(a4260, a4261, a4262)
    };
    let a4264 = 0u64;
    if if nat_eq(a4263, a4264) {
        f220(match v0.clone() {
            Adt14::C0(_, h4265, _, _, _, _, _, _, _, _, _) => {
                h4265
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4266, _, _, _, _, _) => {
                h4266
            }
        })
    } else {
        false
    } {
        f98(match v0.clone() {
            Adt14::C0(_, _, h4267, _, _, _, _, _, _, _, _) => {
                h4267
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4268, _, _, _, _, _, _, _) => {
                h4268
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4269, _, _, _, _, _, _) => {
                h4269
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4270, _, _, _, _) => {
                h4270
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4271, _, _, _) => {
                h4271
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4272, _, _) => {
                h4272
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f176(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4274 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4273, _) => {
            h4273
        }
    };
    let a4275 = 0u64;
    if nat_eq(a4274, a4275) {
        Ok(true)
    } else {
        let a4277 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4276, _) => {
                h4276
            }
        };
        let a4278 = 0u64;
        if nat_eq(a4277, a4278) {
            Ok(true)
        } else {
            let a4280 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4279) => {
                    h4279
                }
            };
            let a4281 = 0u64;
            if nat_eq(a4280, a4281) {
                Ok(true)
            } else {
                let a4283 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4282) => {
                        h4282
                    }
                };
                let a4284 = 0u64;
                if nat_eq(a4283, a4284) {
                    Ok(true)
                } else {
                    let a4290 = {
                        let a4287 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4285) => {
                                h4285
                            }
                        };
                        let a4288 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4286, _) => {
                                h4286
                            }
                        };
                        nat_add(a4287, a4288)?
                    };
                    let a4291 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4289) => {
                            h4289
                        }
                    };
                    if nat_le(a4290, a4291) {
                        Ok(true)
                    } else {
                        let a4297 = {
                            let a4294 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4292) => {
                                    h4292
                                }
                            };
                            let a4295 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4293, _) => {
                                    h4293
                                }
                            };
                            nat_add(a4294, a4295)?
                        };
                        let a4298 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4296) => {
                                h4296
                            }
                        };
                        Ok(nat_le(a4297, a4298))
                    }
                }
            }
        }
    }
}

pub fn f177(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f172(v0.clone(), v1.clone()) {
        let a4301 = match v0.clone() {
            Adt14::C0(h4299, _, _, _, _, _, _, _, _, _, _) => {
                h4299
            }
        };
        let a4302 = match v1.clone() {
            Adt14::C0(h4300, _, _, _, _, _, _, _, _, _, _) => {
                h4300
            }
        };
        nat_eq(a4301, a4302)
    } else {
        false
    } {
        let a4305 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4303) => {
                h4303
            }
        };
        let a4306 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4304) => {
                h4304
            }
        };
        nat_eq(a4305, a4306)
    } else {
        false
    } {
        let a4309 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4307, _, _, _, _, _) => {
                h4307
            }
        };
        let a4310 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4308, _, _, _, _, _) => {
                h4308
            }
        };
        nat_eq(a4309, a4310)
    } else {
        false
    } {
        let a4313 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4311, _, _, _, _) => {
                h4311
            }
        };
        let a4314 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4312, _, _, _, _) => {
                h4312
            }
        };
        nat_eq(a4313, a4314)
    } else {
        false
    } {
        let a4317 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4315, _, _, _) => {
                h4315
            }
        };
        let a4318 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4316, _, _, _) => {
                h4316
            }
        };
        nat_eq(a4317, a4318)
    } else {
        false
    } {
        let a4321 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4319, _, _) => {
                h4319
            }
        };
        let a4322 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4320, _, _) => {
                h4320
            }
        };
        nat_eq(a4321, a4322)
    } else {
        false
    }
}

pub fn f178(v0: Adt14) -> R<bool> {
    if f221(match v0.clone() {
        Adt14::C0(_, h4323, _, _, _, _, _, _, _, _, _) => {
            h4323
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4324, _, _, _, _, _) => {
            h4324
        }
    }) {
        f99(f222(match v0.clone() {
            Adt14::C0(_, h4325, _, _, _, _, _, _, _, _, _) => {
                h4325
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4326, _, _, _, _, _) => {
                h4326
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4327, _, _, _, _, _, _, _, _) => {
                h4327
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4328, _, _, _, _, _, _, _) => {
                h4328
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4329, _, _, _, _, _, _) => {
                h4329
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4330, _, _, _, _) => {
                h4330
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4331, _, _, _) => {
                h4331
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4332, _, _) => {
                h4332
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f179(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4333 = v1;
    let a4334 = 0u64;
    if if nat_eq(a4333, a4334) {
        true
    } else {
        let a4335 = v2;
        let a4336 = 0u64;
        nat_eq(a4335, a4336)
    } {
        Ok(0u64)
    } else {
        let a4339 = f100(v1, v2, v3)?;
        let a4340 = {
            let a4337 = v2;
            let a4338 = f223(v0, v1)?;
            nat_mul(a4337, a4338)?
        };
        nat_add(a4339, a4340)
    }
}

pub fn f180(v0: u64, v1: u64) -> bool {
    let a4341 = v0;
    let a4342 = 4u64;
    if nat_eq(a4341, a4342) {
        let a4343 = v1;
        let a4344 = 4u64;
        nat_eq(a4343, a4344)
    } else {
        false
    }
}

pub fn f181(v0: u64) -> R<u64> {
    let a4345 = 6u64;
    let a4346 = v0;
    nat_add(a4345, a4346)
}

pub fn f182(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4347, _, _, _, _, _, _, _, _, _, _) => {
            h4347
        }
    }, match v0.clone() {
        Adt14::C0(_, h4348, _, _, _, _, _, _, _, _, _) => {
            h4348
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4349, _, _, _, _, _, _, _, _) => {
            h4349
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4350, _, _, _, _, _, _, _) => {
            h4350
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4351, _, _, _, _, _, _) => {
            h4351
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4352, _, _, _, _) => {
            h4352
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4353, _, _, _) => {
            h4353
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4354, _, _) => {
            h4354
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4355, _) => {
            h4355
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4356) => {
            h4356
        }
    })
}

pub fn f183(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4357 = v0;
    let a4358 = 2147483647u64;
    if if if nat_le(a4357, a4358) {
        let a4359 = 0u64;
        let a4360 = v3;
        if nat_lt(a4359, a4360) {
            let a4361 = v3;
            let a4362 = 2147483647u64;
            nat_le(a4361, a4362)
        } else {
            false
        }
    } else {
        false
    } {
        let a4367 = {
            let a4365 = {
                let a4363 = v2;
                let a4364 = v3;
                nat_add(a4363, a4364)?
            };
            let a4366 = 1u64;
            nat_sub(a4365, a4366)
        };
        let a4368 = 2147483647u64;
        nat_le(a4367, a4368)
    } else {
        false
    } {
        let a4373 = {
            let a4371 = {
                let a4369 = v0;
                let a4370 = v1;
                nat_mul(a4369, a4370)?
            };
            let a4372 = f224(v2, v3)?;
            nat_mul(a4371, a4372)?
        };
        let a4374 = 9223372036854775807u64;
        Ok(nat_le(a4373, a4374))
    } else {
        Ok(false)
    }
}

pub fn f184(v0: Adt14) -> R<u64> {
    let a4405 = {
        let a4397 = {
            let a4389 = {
                let a4381 = 4u64;
                let a4382 = {
                    let a4379 = {
                        let a4376 = match v0.clone() {
                            Adt14::C0(_, h4375, _, _, _, _, _, _, _, _, _) => {
                                h4375
                            }
                        };
                        let a4377 = 1u64;
                        nat_sub(a4376, a4377)
                    };
                    let a4380 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4378, _, _, _, _, _) => {
                            h4378
                        }
                    };
                    nat_mul(a4379, a4380)?
                };
                nat_add(a4381, a4382)?
            };
            let a4390 = {
                let a4387 = {
                    let a4384 = match v0.clone() {
                        Adt14::C0(_, _, h4383, _, _, _, _, _, _, _, _) => {
                            h4383
                        }
                    };
                    let a4385 = 1u64;
                    nat_sub(a4384, a4385)
                };
                let a4388 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4386, _, _, _, _) => {
                        h4386
                    }
                };
                nat_mul(a4387, a4388)?
            };
            nat_add(a4389, a4390)?
        };
        let a4398 = {
            let a4395 = {
                let a4392 = match v0.clone() {
                    Adt14::C0(_, _, _, h4391, _, _, _, _, _, _, _) => {
                        h4391
                    }
                };
                let a4393 = 1u64;
                nat_sub(a4392, a4393)
            };
            let a4396 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4394, _, _, _) => {
                    h4394
                }
            };
            nat_mul(a4395, a4396)?
        };
        nat_add(a4397, a4398)?
    };
    let a4406 = {
        let a4403 = {
            let a4400 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4399, _, _, _, _, _, _) => {
                    h4399
                }
            };
            let a4401 = 1u64;
            nat_sub(a4400, a4401)
        };
        let a4404 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4402, _, _) => {
                h4402
            }
        };
        nat_mul(a4403, a4404)?
    };
    nat_add(a4405, a4406)
}

pub fn f185(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4407 = v0;
    let a4408 = 0u64;
    if if if if nat_eq(a4407, a4408) {
        true
    } else {
        let a4409 = v1;
        let a4410 = 0u64;
        nat_eq(a4409, a4410)
    } {
        true
    } else {
        let a4411 = v2;
        let a4412 = 0u64;
        nat_eq(a4411, a4412)
    } {
        true
    } else {
        let a4413 = v3;
        let a4414 = 0u64;
        nat_eq(a4413, a4414)
    } {
        Ok(0u64)
    } else {
        let a4415 = f225(v0, v1, v2, v3, v5)?;
        let a4416 = f226(v0, v1, v2, v3, v4)?;
        nat_add(a4415, a4416)
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4417 = v0;
    let a4418 = 0u64;
    if if nat_eq(a4417, a4418) {
        true
    } else {
        let a4419 = v1;
        let a4420 = 0u64;
        nat_eq(a4419, a4420)
    } {
        Ok(0u64)
    } else {
        let a4423 = f100(v0, v1, v2)?;
        let a4424 = {
            let a4421 = v1;
            let a4422 = f227(v0)?;
            nat_mul(a4421, a4422)?
        };
        nat_add(a4423, a4424)
    }
}

pub fn f187(v0: u64, v1: u64, v2: u64) -> bool {
    let a4425 = v0;
    let a4426 = 1u64;
    if nat_le(a4425, a4426) {
        true
    } else {
        let a4430 = {
            let a4427 = v1;
            let a4428 = v2;
            let a4429 = 1u64;
            nat_rem(a4427, a4428, a4429)
        };
        let a4431 = 0u64;
        nat_eq(a4430, a4431)
    }
}

pub fn f188(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4432 = v1;
    let a4433 = 1u64;
    if if if nat_le(a4432, a4433) {
        true
    } else {
        let a4434 = v0;
        let a4435 = v2;
        nat_le(a4434, a4435)
    } {
        let a4436 = v3;
        let a4437 = 1u64;
        if nat_le(a4436, a4437) {
            true
        } else {
            let a4444 = {
                let a4442 = v0;
                let a4443 = {
                    let a4440 = {
                        let a4438 = v1;
                        let a4439 = 1u64;
                        nat_sub(a4438, a4439)
                    };
                    let a4441 = v2;
                    nat_mul(a4440, a4441)?
                };
                nat_add(a4442, a4443)?
            };
            let a4445 = v4;
            nat_le(a4444, a4445)
        }
    } else {
        false
    } {
        let a4446 = v5;
        let a4447 = 1u64;
        if nat_le(a4446, a4447) {
            Ok(true)
        } else {
            let a4460 = {
                let a4458 = {
                    let a4452 = v0;
                    let a4453 = {
                        let a4450 = {
                            let a4448 = v1;
                            let a4449 = 1u64;
                            nat_sub(a4448, a4449)
                        };
                        let a4451 = v2;
                        nat_mul(a4450, a4451)?
                    };
                    nat_add(a4452, a4453)?
                };
                let a4459 = {
                    let a4456 = {
                        let a4454 = v3;
                        let a4455 = 1u64;
                        nat_sub(a4454, a4455)
                    };
                    let a4457 = v4;
                    nat_mul(a4456, a4457)?
                };
                nat_add(a4458, a4459)?
            };
            let a4461 = v6;
            Ok(nat_le(a4460, a4461))
        }
    } else {
        Ok(false)
    }
}

pub fn f189(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f228(v0, v1, v6, v7, v8) {
        let a4462 = v5;
        let a4463 = 26u64;
        nat_eq(a4462, a4463)
    } else {
        false
    } {
        let a4464 = v3;
        let a4465 = v7;
        nat_eq(a4464, a4465)
    } else {
        false
    } {
        let a4466 = v4;
        let a4467 = v8;
        nat_eq(a4466, a4467)
    } else {
        false
    } {
        let a4468 = v9;
        let a4469 = 1u64;
        nat_eq(a4468, a4469)
    } else {
        false
    } {
        let a4470 = v10;
        let a4471 = f229(v0);
        nat_eq(a4470, a4471)
    } else {
        false
    } {
        let a4472 = v11;
        let a4473 = v1;
        nat_eq(a4472, a4473)
    } else {
        false
    } {
        let a4474 = v12;
        let a4475 = v6;
        nat_eq(a4474, a4475)
    } else {
        false
    } {
        let a4476 = v13;
        let a4477 = v7;
        nat_eq(a4476, a4477)
    } else {
        false
    } {
        let a4478 = v14;
        let a4479 = v8;
        nat_eq(a4478, a4479)
    } else {
        false
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4480 = v0;
    let a4481 = 0u64;
    if if if nat_eq(a4480, a4481) {
        true
    } else {
        let a4482 = v1;
        let a4483 = 0u64;
        nat_eq(a4482, a4483)
    } {
        true
    } else {
        let a4484 = v2;
        let a4485 = 0u64;
        nat_eq(a4484, a4485)
    } {
        true
    } else {
        let a4486 = v3;
        let a4487 = 0u64;
        nat_eq(a4486, a4487)
    }
}

pub fn f191(v0: u64) -> u64 {
    let m4488 = f114(v0);
    match m4488 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4489, _) => {
                    h4489
                }
            }
        }
    }
}

pub fn f192(v0: Adt3, v1: bool) -> R<u64> {
    f230(v0.clone(), {
        let a4491 = match v0.clone() {
            Adt3::C0(h4490, _, _, _, _, _, _, _) => {
                h4490
            }
        };
        let a4492 = 1u64;
        nat_sub(a4491, a4492)
    }, {
        let a4494 = match v0.clone() {
            Adt3::C0(_, h4493, _, _, _, _, _, _) => {
                h4493
            }
        };
        let a4495 = 1u64;
        nat_sub(a4494, a4495)
    }, {
        let a4497 = match v0.clone() {
            Adt3::C0(_, _, h4496, _, _, _, _, _) => {
                h4496
            }
        };
        let a4498 = 1u64;
        nat_sub(a4497, a4498)
    }, {
        let a4500 = match v0 {
            Adt3::C0(_, _, _, h4499, _, _, _, _) => {
                h4499
            }
        };
        let a4501 = 1u64;
        nat_sub(a4500, a4501)
    }, v1)
}

pub fn f193(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4502 = v0;
    let a4503 = 0u64;
    if nat_eq(a4502, a4503) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4504 = v0;
            let a4505 = 1u64;
            nat_sub(a4504, a4505)
        };
        let v5: u64 = {
            let a4506 = v4;
            let a4507 = v1;
            let a4508 = 0u64;
            nat_quot(a4506, a4507, a4508)
        };
        let v6: u64 = {
            let a4516 = {
                let a4509 = v5;
                let a4510 = v2;
                nat_mul(a4509, a4510)?
            };
            let a4517 = {
                let a4514 = {
                    let a4511 = v4;
                    let a4512 = v1;
                    let a4513 = 0u64;
                    nat_rem(a4511, a4512, a4513)
                };
                let a4515 = v3;
                nat_mul(a4514, a4515)?
            };
            nat_add(a4516, a4517)?
        };
        let a4518 = v5;
        let a4519 = 0u64;
        if nat_eq(a4518, a4519) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4528 = {
                    let a4522 = {
                        let a4520 = v5;
                        let a4521 = 1u64;
                        nat_sub(a4520, a4521)
                    };
                    let a4523 = v2;
                    nat_mul(a4522, a4523)?
                };
                let a4529 = {
                    let a4526 = {
                        let a4524 = v1;
                        let a4525 = 1u64;
                        nat_sub(a4524, a4525)
                    };
                    let a4527 = v3;
                    nat_mul(a4526, a4527)?
                };
                nat_add(a4528, a4529)?
            };
            let a4530 = v6;
            let a4531 = v7;
            if nat_lt(a4530, a4531) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f194(v0: Adt3, v1: u64) -> bool {
    let a4532 = v1;
    let a4533 = 0u64;
    if !nat_eq(a4532, a4533) {
        true
    } else {
        let a4535 = match v0 {
            Adt3::C0(_, _, _, _, h4534, _, _, _) => {
                h4534
            }
        };
        let a4536 = 4u64;
        nat_eq(a4535, a4536)
    }
}

pub fn f195(v0: u64, v1: bool) -> R<u64> {
    let a4539 = {
        let a4537 = v0;
        let a4538 = 2u64;
        nat_mul(a4537, a4538)?
    };
    let a4540 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4539, a4540)
}

pub fn f196(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4541 = f231(v0, v1);
    match m4541 {
        None => {
            if v5 {
                let m4542 = f232(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4542 {
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
                let a4543 = 4u64;
                let a4544 = v28;
                nat_le(a4543, a4544)
            };
            let a4545 = v2;
            let a4546 = 0u64;
            if if if if if nat_eq(a4545, a4546) {
                let a4547 = v4;
                let a4548 = 0u64;
                nat_eq(a4547, a4548)
            } else {
                false
            } {
                let a4549 = v6;
                let a4550 = v14;
                if nat_eq(a4549, a4550) {
                    let a4551 = v7;
                    let a4552 = v15;
                    nat_eq(a4551, a4552)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4553 = v8;
                let a4554 = v16;
                if nat_eq(a4553, a4554) {
                    let a4555 = v9;
                    let a4556 = v17;
                    nat_eq(a4555, a4556)
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
                        let a4557 = v3;
                        let a4558 = 0u64;
                        nat_eq(a4557, a4558)
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
                    let a4563 = {
                        let a4559 = v6;
                        let a4560 = v7;
                        nat_mul(a4559, a4560)?
                    };
                    let a4564 = {
                        let a4561 = v8;
                        let a4562 = v9;
                        nat_mul(a4561, a4562)?
                    };
                    nat_mul(a4563, a4564)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f197(v0: u64) -> u64 {
    let m4565 = f114(v0);
    match m4565 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4567 = match v1.clone() {
                Adt18::C0(_, h4566, _) => {
                    h4566
                }
            };
            let a4568 = 1u64;
            if if nat_eq(a4567, a4568) {
                let a4570 = match v1.clone() {
                    Adt18::C0(_, _, h4569) => {
                        h4569
                    }
                };
                let a4571 = 1u64;
                if if if nat_eq(a4570, a4571) {
                    true
                } else {
                    let a4573 = match v1.clone() {
                        Adt18::C0(_, _, h4572) => {
                            h4572
                        }
                    };
                    let a4574 = 2u64;
                    nat_eq(a4573, a4574)
                } {
                    true
                } else {
                    let a4576 = match v1.clone() {
                        Adt18::C0(_, _, h4575) => {
                            h4575
                        }
                    };
                    let a4577 = 4u64;
                    nat_eq(a4576, a4577)
                } {
                    true
                } else {
                    let a4579 = match v1.clone() {
                        Adt18::C0(_, _, h4578) => {
                            h4578
                        }
                    };
                    let a4580 = 8u64;
                    nat_eq(a4579, a4580)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4581) => {
                        h4581
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f198(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4586 = {
        let a4584 = {
            let a4582 = v0;
            let a4583 = v1;
            nat_sub(a4582, a4583)
        };
        let a4585 = 1u64;
        nat_sub(a4584, a4585)
    };
    let a4587 = v2;
    nat_add(a4586, a4587)
}

pub fn f199(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f130({
        let a4588 = v0;
        let a4589 = v4;
        let a4590 = 0u64;
        nat_rem(a4588, a4589, a4590)
    }, {
        let a4591 = v1;
        let a4592 = v5;
        let a4593 = 0u64;
        nat_rem(a4591, a4592, a4593)
    }, {
        let a4594 = v2;
        let a4595 = v6;
        let a4596 = 0u64;
        nat_rem(a4594, a4595, a4596)
    }, {
        let a4597 = v3;
        let a4598 = v7;
        let a4599 = 0u64;
        nat_rem(a4597, a4598, a4599)
    }, v8, v9, v10, v11)
}

pub fn f200(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4600 = v1;
    let a4601 = 2147483648u64;
    if nat_lt(a4600, a4601) {
        let a4612 = {
            let a4610 = {
                let a4605 = {
                    let a4602 = v0;
                    let a4603 = v2;
                    let a4604 = 0u64;
                    nat_rem(a4602, a4603, a4604)
                };
                let a4606 = v2;
                nat_add(a4605, a4606)?
            };
            let a4611 = {
                let a4607 = v1;
                let a4608 = v2;
                let a4609 = 0u64;
                nat_rem(a4607, a4608, a4609)
            };
            nat_sub(a4610, a4611)
        };
        let a4613 = v2;
        let a4614 = 0u64;
        Ok(nat_rem(a4612, a4613, a4614))
    } else {
        let a4617 = {
            let a4615 = v0;
            let a4616 = f131(v1);
            nat_add(a4615, a4616)?
        };
        let a4618 = v2;
        let a4619 = 0u64;
        Ok(nat_rem(a4617, a4618, a4619))
    }
}

pub fn f201(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4622 = f132(v1);
    let a4623 = {
        let a4620 = v0;
        let a4621 = f131(v1);
        nat_add(a4620, a4621)?
    };
    if nat_le(a4622, a4623) {
        let a4624 = f202(v0, v1)?;
        let a4625 = v2;
        Ok(nat_lt(a4624, a4625))
    } else {
        Ok(false)
    }
}

pub fn f202(v0: u64, v1: u64) -> R<u64> {
    let a4628 = {
        let a4626 = v0;
        let a4627 = f131(v1);
        nat_add(a4626, a4627)?
    };
    let a4629 = f132(v1);
    Ok(nat_sub(a4628, a4629))
}

pub fn f203(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4630 = v0;
    let a4631 = v1;
    if nat_lt(a4630, a4631) {
        let a4632 = v1;
        let a4633 = v0;
        Ok(nat_sub(a4632, a4633))
    } else {
        let a4636 = v0;
        let a4637 = {
            let a4634 = v1;
            let a4635 = v2;
            nat_add(a4634, a4635)?
        };
        if nat_lt(a4636, a4637) {
            let a4638 = v0;
            let a4639 = v1;
            Ok(nat_sub(a4638, a4639))
        } else {
            let a4646 = {
                let a4642 = 2u64;
                let a4643 = {
                    let a4640 = v2;
                    let a4641 = 1u64;
                    nat_sub(a4640, a4641)
                };
                nat_mul(a4642, a4643)?
            };
            let a4647 = {
                let a4644 = v0;
                let a4645 = v1;
                nat_sub(a4644, a4645)
            };
            Ok(nat_sub(a4646, a4647))
        }
    }
}

pub fn f204(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4650 = {
        let a4648 = v2;
        let a4649 = f230(v1.clone(), f70(v0, v1.clone(), 0u64), f70(v0, v1.clone(), 1u64), f70(v0, v1.clone(), 2u64), f70(v0, v1.clone(), 3u64), false)?;
        nat_add(a4648, a4649)?
    };
    let a4651 = f230(v1.clone(), f70(v0, v1.clone(), 0u64), f70(v0, v1.clone(), 1u64), f70(v0, v1.clone(), 2u64), f70(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4650, a4651))
}

pub fn f205(v0: u64, v1: u64, v2: u64) -> bool {
    let a4652 = v0;
    let a4653 = 0u64;
    if nat_eq(a4652, a4653) {
        let a4654 = v2;
        let a4655 = v1;
        nat_le(a4654, a4655)
    } else {
        let a4656 = v0;
        let a4657 = 1u64;
        if nat_eq(a4656, a4657) {
            let a4658 = v2;
            let a4659 = v1;
            nat_lt(a4658, a4659)
        } else {
            let a4660 = v0;
            let a4661 = 2u64;
            if nat_eq(a4660, a4661) {
                let a4662 = v1;
                let a4663 = v2;
                nat_le(a4662, a4663)
            } else {
                let a4664 = v0;
                let a4665 = 3u64;
                if nat_eq(a4664, a4665) {
                    let a4666 = v1;
                    let a4667 = v2;
                    nat_lt(a4666, a4667)
                } else {
                    let a4668 = v1;
                    let a4669 = v2;
                    nat_eq(a4668, a4669)
                }
            }
        }
    }
}

pub fn f206(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4670 = v0;
    let a4671 = 0u64;
    if nat_eq(a4670, a4671) {
        Ok(f217(0u64))
    } else {
        let a4674 = {
            let a4672 = v0;
            let a4673 = 1u64;
            nat_add(a4672, a4673)?
        };
        let a4675 = f128(v1.clone(), 13u64, v2)?;
        if nat_eq(a4674, a4675) {
            Ok(Adt9::C4({
                let a4676 = v2;
                let a4677 = 0u64;
                if nat_eq(a4676, a4677) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4678 = v0;
                let a4679 = 1u64;
                nat_sub(a4678, a4679)
            };
            if f194(v1.clone(), v2) {
                let v4: u64 = f195(match v1.clone() {
                    Adt3::C0(h4680, _, _, _, _, _, _, _) => {
                        h4680
                    }
                }, false)?;
                let v5: u64 = {
                    let a4681 = v3;
                    let a4682 = v4;
                    let a4683 = 0u64;
                    nat_quot(a4681, a4682, a4683)
                };
                let v6: u64 = {
                    let a4684 = v3;
                    let a4685 = v4;
                    let a4686 = 0u64;
                    nat_rem(a4684, a4685, a4686)
                };
                let a4689 = {
                    let a4687 = v6;
                    let a4688 = 1u64;
                    nat_add(a4687, a4688)?
                };
                let a4690 = v4;
                if nat_eq(a4689, a4690) {
                    Ok(Adt9::C4({
                        let a4691 = v2;
                        let a4692 = 0u64;
                        if nat_eq(a4691, a4692) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f207(v6, v5, v1, v2)
                }
            } else {
                f233({
                    let a4693 = v3;
                    let a4694 = 2u64;
                    let a4695 = 0u64;
                    nat_quot(a4693, a4694, a4695)
                }, {
                    let a4696 = v3;
                    let a4697 = 2u64;
                    let a4698 = 0u64;
                    nat_rem(a4696, a4697, a4698)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f207(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4699 = v0;
    let a4700 = 0u64;
    if nat_eq(a4699, a4700) {
        Ok(f217(1u64))
    } else {
        f233({
            let a4709 = {
                let a4702 = v1;
                let a4703 = match v2.clone() {
                    Adt3::C0(h4701, _, _, _, _, _, _, _) => {
                        h4701
                    }
                };
                nat_mul(a4702, a4703)?
            };
            let a4710 = {
                let a4706 = {
                    let a4704 = v0;
                    let a4705 = 1u64;
                    nat_sub(a4704, a4705)
                };
                let a4707 = 2u64;
                let a4708 = 0u64;
                nat_quot(a4706, a4707, a4708)
            };
            nat_add(a4709, a4710)?
        }, {
            let a4713 = {
                let a4711 = v0;
                let a4712 = 1u64;
                nat_sub(a4711, a4712)
            };
            let a4714 = 2u64;
            let a4715 = 0u64;
            nat_rem(a4713, a4714, a4715)
        }, v2, v3, 1u64)
    }
}

pub fn f208(v0: u64, v1: u64) -> R<u64> {
    let a4716 = v0;
    let a4717 = f94(v1)?;
    let a4718 = 0u64;
    Ok(nat_rem(a4716, a4717, a4718))
}

pub fn f209(v0: u64, v1: u64) -> R<u64> {
    let a4719 = 4u64;
    let a4720 = f234(v0, v1)?;
    nat_mul(a4719, a4720)
}

pub fn f210(v0: u64, v1: u64) -> R<u64> {
    let a4721 = 4u64;
    let a4722 = f235(v0, v1)?;
    nat_mul(a4721, a4722)
}

pub fn f211(v0: u64, v1: u64) -> R<bool> {
    let a4728 = {
        let a4725 = {
            let a4723 = f208(v0, v1)?;
            let a4724 = 2u64;
            nat_sub(a4723, a4724)
        };
        let a4726 = 2u64;
        let a4727 = 0u64;
        nat_rem(a4725, a4726, a4727)
    };
    let a4729 = 0u64;
    Ok(nat_eq(a4728, a4729))
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4730 = f114(v0);
    match m4730 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f112(v0) {
                let a4731 = v1;
                let a4732 = v7;
                nat_lt(a4731, a4732)
            } else {
                false
            } {
                f159(v8, v2)
            } else {
                false
            } {
                let a4733 = v3;
                let a4734 = f236(v4, v5, v6)?;
                nat_lt(a4733, a4734)
            } else {
                false
            } {
                let a4739 = {
                    let a4736 = v7;
                    let a4737 = match v13.clone() {
                        Adt18::C0(_, h4735, _) => {
                            h4735
                        }
                    };
                    let a4738 = 0u64;
                    nat_rem(a4736, a4737, a4738)
                };
                let a4740 = 0u64;
                nat_eq(a4739, a4740)
            } else {
                false
            } {
                Ok(Some({
                    let a4769 = {
                        let a4759 = {
                            let a4749 = {
                                let a4745 = {
                                    let a4742 = v1;
                                    let a4743 = match v13 {
                                        Adt18::C0(_, h4741, _) => {
                                            h4741
                                        }
                                    };
                                    let a4744 = 0u64;
                                    nat_quot(a4742, a4743, a4744)
                                };
                                let a4746 = v9;
                                nat_mul(a4745, a4746)?
                            };
                            let a4750 = {
                                let a4747 = v2;
                                let a4748 = v10;
                                nat_mul(a4747, a4748)?
                            };
                            nat_add(a4749, a4750)?
                        };
                        let a4760 = {
                            let a4757 = {
                                let a4754 = {
                                    let a4751 = v3;
                                    let a4752 = v4;
                                    let a4753 = 0u64;
                                    nat_quot(a4751, a4752, a4753)
                                };
                                let a4755 = v5;
                                let a4756 = 0u64;
                                nat_rem(a4754, a4755, a4756)
                            };
                            let a4758 = v11;
                            nat_mul(a4757, a4758)?
                        };
                        nat_add(a4759, a4760)?
                    };
                    let a4770 = {
                        let a4767 = {
                            let a4764 = {
                                let a4761 = v3;
                                let a4762 = v4;
                                let a4763 = 0u64;
                                nat_quot(a4761, a4762, a4763)
                            };
                            let a4765 = v5;
                            let a4766 = 0u64;
                            nat_quot(a4764, a4765, a4766)
                        };
                        let a4768 = v12;
                        nat_mul(a4767, a4768)?
                    };
                    nat_add(a4769, a4770)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f213(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f237(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f214(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f238(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f215(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4771 = v0;
    let a4772 = 64u64;
    if nat_eq(a4771, a4772) {
        f239(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4773 = v0;
        let a4774 = 65u64;
        if nat_eq(a4773, a4774) {
            f240(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f216(v0: u64) -> u64 {
    let a4775 = v0;
    let a4776 = 30u64;
    if nat_eq(a4775, a4776) {
        4u64
    } else {
        3u64
    }
}

pub fn f217(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f218(v0: u64) -> bool {
    let a4777 = 0u64;
    let a4778 = v0;
    if nat_lt(a4777, a4778) {
        let a4779 = v0;
        let a4780 = 2139095040u64;
        nat_le(a4779, a4780)
    } else {
        false
    }
}

pub fn f219(v0: u64) -> u64 {
    let a4781 = v0;
    let a4782 = 40u64;
    if nat_lt(a4781, a4782) {
        let a4783 = v0;
        let a4784 = 20u64;
        if nat_lt(a4783, a4784) {
            let a4785 = v0;
            let a4786 = 10u64;
            if nat_lt(a4785, a4786) {
                let a4787 = v0;
                let a4788 = 5u64;
                if nat_lt(a4787, a4788) {
                    let a4789 = v0;
                    let a4790 = 2u64;
                    if nat_lt(a4789, a4790) {
                        let a4791 = v0;
                        let a4792 = 1u64;
                        if nat_lt(a4791, a4792) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4793 = v0;
                        let a4794 = 3u64;
                        if nat_lt(a4793, a4794) {
                            4u64
                        } else {
                            let a4795 = v0;
                            let a4796 = 4u64;
                            if nat_lt(a4795, a4796) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4797 = v0;
                    let a4798 = 7u64;
                    if nat_lt(a4797, a4798) {
                        let a4799 = v0;
                        let a4800 = 6u64;
                        if nat_lt(a4799, a4800) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4801 = v0;
                        let a4802 = 8u64;
                        if nat_lt(a4801, a4802) {
                            128u64
                        } else {
                            let a4803 = v0;
                            let a4804 = 9u64;
                            if nat_lt(a4803, a4804) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4805 = v0;
                let a4806 = 15u64;
                if nat_lt(a4805, a4806) {
                    let a4807 = v0;
                    let a4808 = 12u64;
                    if nat_lt(a4807, a4808) {
                        let a4809 = v0;
                        let a4810 = 11u64;
                        if nat_lt(a4809, a4810) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4811 = v0;
                        let a4812 = 13u64;
                        if nat_lt(a4811, a4812) {
                            4096u64
                        } else {
                            let a4813 = v0;
                            let a4814 = 14u64;
                            if nat_lt(a4813, a4814) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4815 = v0;
                    let a4816 = 17u64;
                    if nat_lt(a4815, a4816) {
                        let a4817 = v0;
                        let a4818 = 16u64;
                        if nat_lt(a4817, a4818) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4819 = v0;
                        let a4820 = 18u64;
                        if nat_lt(a4819, a4820) {
                            131072u64
                        } else {
                            let a4821 = v0;
                            let a4822 = 19u64;
                            if nat_lt(a4821, a4822) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4823 = v0;
            let a4824 = 30u64;
            if nat_lt(a4823, a4824) {
                let a4825 = v0;
                let a4826 = 25u64;
                if nat_lt(a4825, a4826) {
                    let a4827 = v0;
                    let a4828 = 22u64;
                    if nat_lt(a4827, a4828) {
                        let a4829 = v0;
                        let a4830 = 21u64;
                        if nat_lt(a4829, a4830) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4831 = v0;
                        let a4832 = 23u64;
                        if nat_lt(a4831, a4832) {
                            4194304u64
                        } else {
                            let a4833 = v0;
                            let a4834 = 24u64;
                            if nat_lt(a4833, a4834) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4835 = v0;
                    let a4836 = 27u64;
                    if nat_lt(a4835, a4836) {
                        let a4837 = v0;
                        let a4838 = 26u64;
                        if nat_lt(a4837, a4838) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4839 = v0;
                        let a4840 = 28u64;
                        if nat_lt(a4839, a4840) {
                            134217728u64
                        } else {
                            let a4841 = v0;
                            let a4842 = 29u64;
                            if nat_lt(a4841, a4842) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4843 = v0;
                let a4844 = 35u64;
                if nat_lt(a4843, a4844) {
                    let a4845 = v0;
                    let a4846 = 32u64;
                    if nat_lt(a4845, a4846) {
                        let a4847 = v0;
                        let a4848 = 31u64;
                        if nat_lt(a4847, a4848) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4849 = v0;
                        let a4850 = 33u64;
                        if nat_lt(a4849, a4850) {
                            4294967296u64
                        } else {
                            let a4851 = v0;
                            let a4852 = 34u64;
                            if nat_lt(a4851, a4852) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4853 = v0;
                    let a4854 = 37u64;
                    if nat_lt(a4853, a4854) {
                        let a4855 = v0;
                        let a4856 = 36u64;
                        if nat_lt(a4855, a4856) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4857 = v0;
                        let a4858 = 38u64;
                        if nat_lt(a4857, a4858) {
                            137438953472u64
                        } else {
                            let a4859 = v0;
                            let a4860 = 39u64;
                            if nat_lt(a4859, a4860) {
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

pub fn f220(v0: u64, v1: u64) -> bool {
    let a4861 = v0;
    let a4862 = 1u64;
    if nat_le(a4861, a4862) {
        true
    } else {
        let a4866 = {
            let a4863 = v1;
            let a4864 = 4u64;
            let a4865 = 0u64;
            nat_rem(a4863, a4864, a4865)
        };
        let a4867 = 0u64;
        nat_eq(a4866, a4867)
    }
}

pub fn f221(v0: u64, v1: u64) -> bool {
    let a4868 = v0;
    let a4869 = 1u64;
    if nat_le(a4868, a4869) {
        true
    } else {
        let a4870 = 4u64;
        let a4871 = v1;
        nat_le(a4870, a4871)
    }
}

pub fn f222(v0: u64, v1: u64) -> R<u64> {
    let a4872 = v0;
    let a4873 = 0u64;
    if nat_eq(a4872, a4873) {
        Ok(0u64)
    } else {
        let a4878 = 4u64;
        let a4879 = {
            let a4876 = {
                let a4874 = v0;
                let a4875 = 1u64;
                nat_sub(a4874, a4875)
            };
            let a4877 = v1;
            nat_mul(a4876, a4877)?
        };
        nat_add(a4878, a4879)
    }
}

pub fn f223(v0: bool, v1: u64) -> R<u64> {
    let a4884 = {
        let a4882 = {
            let a4880 = f241(v0, v1)?;
            let a4881 = f242(v0, v1)?;
            nat_add(a4880, a4881)?
        };
        let a4883 = f243(v0);
        nat_add(a4882, a4883)?
    };
    let a4885 = v1;
    nat_add(a4884, a4885)
}

pub fn f224(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f244(v0, v1)?;
    let v3: u64 = {
        let a4890 = {
            let a4888 = {
                let a4886 = v1;
                let a4887 = 1u64;
                nat_sub(a4886, a4887)
            };
            let a4889 = v2;
            nat_mul(a4888, a4889)?
        };
        let a4891 = v0;
        nat_sub(a4890, a4891)
    };
    let a4892 = v2;
    let a4893 = v3;
    if nat_lt(a4892, a4893) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f225(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4894 = v4;
    let a4895 = 0u64;
    if nat_eq(a4894, a4895) {
        Ok(0u64)
    } else {
        let a4902 = 2u64;
        let a4903 = {
            let a4900 = {
                let a4896 = v0;
                let a4897 = v1;
                nat_mul(a4896, a4897)?
            };
            let a4901 = {
                let a4898 = v2;
                let a4899 = v3;
                nat_mul(a4898, a4899)?
            };
            nat_mul(a4900, a4901)?
        };
        nat_mul(a4902, a4903)
    }
}

pub fn f226(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4908 = v3;
    let a4909 = {
        let a4906 = {
            let a4904 = f245(v2, v4)?;
            let a4905 = f246(v0, v1, f244(v2, v4)?)?;
            nat_mul(a4904, a4905)?
        };
        let a4907 = f246(v0, v1, f247(v2, v4)?)?;
        nat_add(a4906, a4907)?
    };
    nat_mul(a4908, a4909)
}

pub fn f227(v0: u64) -> R<u64> {
    let a4912 = 13u64;
    let a4913 = {
        let a4910 = 9u64;
        let a4911 = v0;
        nat_mul(a4910, a4911)?
    };
    nat_add(a4912, a4913)
}

pub fn f228(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f190(v1, v2, v3, v4) {
        f248(v0)
    } else {
        f112(v0)
    }
}

pub fn f229(v0: u64) -> u64 {
    let a4914 = v0;
    let a4915 = 26u64;
    if nat_eq(a4914, a4915) {
        26u64
    } else {
        0u64
    }
}

pub fn f230(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4936 = {
        let a4930 = {
            let a4924 = {
                let a4918 = v1;
                let a4919 = if v5 {
                    f131(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4916, _, _, _) => {
                            h4916
                        }
                    })
                } else {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4917, _, _, _) => {
                            h4917
                        }
                    })
                };
                nat_mul(a4918, a4919)?
            };
            let a4925 = {
                let a4922 = v2;
                let a4923 = if v5 {
                    f131(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4920, _, _) => {
                            h4920
                        }
                    })
                } else {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4921, _, _) => {
                            h4921
                        }
                    })
                };
                nat_mul(a4922, a4923)?
            };
            nat_add(a4924, a4925)?
        };
        let a4931 = {
            let a4928 = v3;
            let a4929 = if v5 {
                f131(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4926, _) => {
                        h4926
                    }
                })
            } else {
                f132(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4927, _) => {
                        h4927
                    }
                })
            };
            nat_mul(a4928, a4929)?
        };
        nat_add(a4930, a4931)?
    };
    let a4937 = {
        let a4934 = v4;
        let a4935 = if v5 {
            f131(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4932) => {
                    h4932
                }
            })
        } else {
            f132(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4933) => {
                    h4933
                }
            })
        };
        nat_mul(a4934, a4935)?
    };
    nat_add(a4936, a4937)
}

pub fn f231(v0: u64, v1: u64) -> Option<u64> {
    let a4938 = v0;
    let a4939 = 2u64;
    if nat_eq(a4938, a4939) {
        Some(0u64)
    } else {
        let a4940 = v0;
        let a4941 = 5u64;
        if nat_eq(a4940, a4941) {
            Some(1u64)
        } else {
            let a4942 = v0;
            let a4943 = 6u64;
            if nat_eq(a4942, a4943) {
                Some(2u64)
            } else {
                let a4944 = v0;
                let a4945 = 7u64;
                if nat_eq(a4944, a4945) {
                    Some(3u64)
                } else {
                    let a4946 = v0;
                    let a4947 = 9u64;
                    if nat_eq(a4946, a4947) {
                        Some(4u64)
                    } else {
                        let a4948 = v0;
                        let a4949 = 8u64;
                        if nat_eq(a4948, a4949) {
                            Some(5u64)
                        } else {
                            let a4950 = v0;
                            let a4951 = 89u64;
                            if nat_eq(a4950, a4951) {
                                let a4952 = v1;
                                let a4953 = 2u64;
                                if nat_eq(a4952, a4953) {
                                    Some(6u64)
                                } else {
                                    let a4954 = v1;
                                    let a4955 = 0u64;
                                    if nat_eq(a4954, a4955) {
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

pub fn f232(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4956 = v0;
    let a4957 = 30u64;
    if nat_eq(a4956, a4957) {
        let a4958 = v2;
        let a4959 = 0u64;
        if if if nat_eq(a4958, a4959) {
            true
        } else {
            let a4960 = v2;
            let a4961 = 1u64;
            nat_eq(a4960, a4961)
        } {
            let a4962 = v1;
            let a4963 = 0u64;
            if nat_eq(a4962, a4963) {
                true
            } else {
                f249(v1, v4)
            }
        } else {
            false
        } {
            f250(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f251(v1) {
            f251(v2)
        } else {
            false
        } {
            let a4964 = v1;
            let a4965 = 0u64;
            if if nat_eq(a4964, a4965) {
                true
            } else {
                let a4966 = v2;
                let a4967 = 0u64;
                nat_eq(a4966, a4967)
            } {
                true
            } else {
                let a4968 = v1;
                let a4969 = v2;
                nat_eq(a4968, a4969)
            }
        } else {
            false
        } {
            true
        } else {
            let a4970 = v1;
            let a4971 = 43u64;
            if nat_eq(a4970, a4971) {
                f251(v2)
            } else {
                false
            }
        } {
            f250(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f233(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4972 = v1;
    let a4973 = 0u64;
    if nat_eq(a4972, a4973) {
        Ok(Adt9::C4({
            let a4974 = v3;
            let a4975 = 0u64;
            if nat_eq(a4974, a4975) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f138(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a4976 = v3;
            let a4977 = 0u64;
            if nat_eq(a4976, a4977) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f234(v0: u64, v1: u64) -> R<u64> {
    let a4978 = v0;
    let a4979 = f94(v1)?;
    let a4980 = 0u64;
    Ok(nat_quot(a4978, a4979, a4980))
}

pub fn f235(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f234(v0, v1)?;
    let v3: u64 = {
        let a4983 = {
            let a4981 = f208(v0, v1)?;
            let a4982 = 2u64;
            nat_sub(a4981, a4982)
        };
        let a4984 = 2u64;
        let a4985 = 0u64;
        nat_quot(a4983, a4984, a4985)
    };
    let v4: u64 = {
        let a4986 = v2;
        let a4987 = v1;
        nat_mul(a4986, a4987)?
    };
    if f211(v0, v1)? {
        let a4988 = v4;
        let a4989 = v3;
        nat_add(a4988, a4989)
    } else {
        let a5001 = {
            let a4997 = v4;
            let a4998 = {
                let a4995 = {
                    let a4990 = v1;
                    let a4991 = 1u64;
                    nat_sub(a4990, a4991)
                };
                let a4996 = {
                    let a4992 = v2;
                    let a4993 = v1;
                    let a4994 = 0u64;
                    nat_rem(a4992, a4993, a4994)
                };
                nat_mul(a4995, a4996)?
            };
            nat_sub(a4997, a4998)
        };
        let a5002 = {
            let a4999 = v3;
            let a5000 = v1;
            nat_mul(a4999, a5000)?
        };
        nat_add(a5001, a5002)
    }
}

pub fn f236(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5003 = v0;
    let a5004 = 0u64;
    if if if nat_eq(a5003, a5004) {
        true
    } else {
        let a5005 = v1;
        let a5006 = 0u64;
        nat_eq(a5005, a5006)
    } {
        true
    } else {
        let a5007 = v2;
        let a5008 = 0u64;
        nat_eq(a5007, a5008)
    } {
        Ok(0u64)
    } else {
        let a5011 = {
            let a5009 = v0;
            let a5010 = v1;
            nat_mul(a5009, a5010)?
        };
        let a5012 = v2;
        nat_mul(a5011, a5012)
    }
}

pub fn f237(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f103(match v1.clone() {
        Adt20::C0(h5013, _, _, _, _, _, _, _) => {
            h5013
        }
    })?;
    let v6: u64 = {
        let a5019 = match v1.clone() {
            Adt20::C0(_, h5014, _, _, _, _, _, _) => {
                h5014
            }
        };
        let a5020 = {
            let a5017 = match v1.clone() {
                Adt20::C0(_, _, h5015, _, _, _, _, _) => {
                    h5015
                }
            };
            let a5018 = match v1.clone() {
                Adt20::C0(_, _, _, h5016, _, _, _, _) => {
                    h5016
                }
            };
            nat_mul(a5017, a5018)?
        };
        nat_mul(a5019, a5020)?
    };
    let a5022 = v0;
    let a5023 = f39(match v1.clone() {
        Adt20::C0(h5021, _, _, _, _, _, _, _) => {
            h5021
        }
    }, v6)?;
    if nat_lt(a5022, a5023) {
        f252({
            let a5024 = v0;
            let a5025 = v5;
            let a5026 = 0u64;
            nat_rem(a5024, a5025, a5026)
        }, {
            let a5027 = v0;
            let a5028 = v5;
            let a5029 = 0u64;
            nat_quot(a5027, a5028, a5029)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f238(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5030 = v6;
        let a5031 = 6u64;
        nat_sub(a5030, a5031)
    };
    let v9: Adt20 = f253(v3);
    let a5033 = v0;
    let a5034 = f186(match v1.clone() {
        Adt20::C0(h5032, _, _, _, _, _, _, _) => {
            h5032
        }
    }, v5, v8)?;
    if nat_lt(a5033, a5034) {
        let v10: u64 = f100(match v1.clone() {
            Adt20::C0(h5035, _, _, _, _, _, _, _) => {
                h5035
            }
        }, v5, v8)?;
        let a5036 = v0;
        let a5037 = v10;
        if nat_lt(a5036, a5037) {
            f254(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5038 = v0;
                let a5039 = v10;
                nat_sub(a5038, a5039)
            };
            let v12: u64 = f227(match v1.clone() {
                Adt20::C0(h5040, _, _, _, _, _, _, _) => {
                    h5040
                }
            })?;
            f255({
                let a5041 = v11;
                let a5042 = v12;
                let a5043 = 0u64;
                nat_rem(a5041, a5042, a5043)
            }, {
                let a5044 = v11;
                let a5045 = v12;
                let a5046 = 0u64;
                nat_quot(a5044, a5045, a5046)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f239(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5047 = v5;
        let a5048 = 6u64;
        nat_sub(a5047, a5048)
    };
    let v8: bool = f180(match v1.clone() {
        Adt20::C0(_, _, _, _, h5049, _, _, _) => {
            h5049
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5050, _, _, _) => {
            h5050
        }
    });
    let a5052 = v0;
    let a5053 = f179(v8, match v1.clone() {
        Adt20::C0(h5051, _, _, _, _, _, _, _) => {
            h5051
        }
    }, v4, v7)?;
    if nat_lt(a5052, a5053) {
        let v9: u64 = f100(match v1.clone() {
            Adt20::C0(h5054, _, _, _, _, _, _, _) => {
                h5054
            }
        }, v4, v7)?;
        let a5055 = v0;
        let a5056 = v9;
        if nat_lt(a5055, a5056) {
            f254(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5057 = v0;
                let a5058 = v9;
                nat_sub(a5057, a5058)
            };
            let v11: u64 = f223(v8, match v1.clone() {
                Adt20::C0(h5059, _, _, _, _, _, _, _) => {
                    h5059
                }
            })?;
            f256({
                let a5060 = v10;
                let a5061 = v11;
                let a5062 = 0u64;
                nat_rem(a5060, a5061, a5062)
            }, {
                let a5063 = v10;
                let a5064 = v11;
                let a5065 = 0u64;
                nat_quot(a5063, a5064, a5065)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f240(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5066 = v5;
        let a5067 = 6u64;
        nat_sub(a5066, a5067)
    };
    let v8: Adt20 = f253(v2);
    let v9: u64 = f225(match v1.clone() {
        Adt20::C0(h5068, _, _, _, _, _, _, _) => {
            h5068
        }
    }, match v1.clone() {
        Adt20::C0(_, h5069, _, _, _, _, _, _) => {
            h5069
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5070, _, _, _, _, _) => {
            h5070
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5071, _, _, _, _) => {
            h5071
        }
    }, v7)?;
    let a5076 = v0;
    let a5077 = f185(match v1.clone() {
        Adt20::C0(h5072, _, _, _, _, _, _, _) => {
            h5072
        }
    }, match v1.clone() {
        Adt20::C0(_, h5073, _, _, _, _, _, _) => {
            h5073
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5074, _, _, _, _, _) => {
            h5074
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5075, _, _, _, _) => {
            h5075
        }
    }, v4, v7)?;
    if nat_lt(a5076, a5077) {
        let a5078 = v0;
        let a5079 = v9;
        if nat_lt(a5078, a5079) {
            f254(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f257({
                let a5080 = v0;
                let a5081 = v9;
                nat_sub(a5080, a5081)
            }, match v1.clone() {
                Adt20::C0(h5082, _, _, _, _, _, _, _) => {
                    h5082
                }
            }, match v1.clone() {
                Adt20::C0(_, h5083, _, _, _, _, _, _) => {
                    h5083
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5084, _, _, _, _, _) => {
                    h5084
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5085, _, _, _, _) => {
                    h5085
                }
            }, v4)?;
            f258(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f241(v0: bool, v1: u64) -> R<u64> {
    let a5088 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5089 = {
        let a5086 = f259(v0);
        let a5087 = v1;
        nat_mul(a5086, a5087)?
    };
    nat_add(a5088, a5089)
}

pub fn f242(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5097 = {
            let a5090 = 13u64;
            let a5091 = f260(v1);
            nat_mul(a5090, a5091)?
        };
        let a5098 = {
            let a5095 = 4u64;
            let a5096 = {
                let a5092 = v1;
                let a5093 = 4u64;
                let a5094 = 0u64;
                nat_rem(a5092, a5093, a5094)
            };
            nat_mul(a5095, a5096)?
        };
        nat_add(a5097, a5098)
    } else {
        let a5099 = 3u64;
        let a5100 = v1;
        nat_mul(a5099, a5100)
    }
}

pub fn f243(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f244(v0: u64, v1: u64) -> R<u64> {
    let a5105 = {
        let a5103 = {
            let a5101 = v0;
            let a5102 = v1;
            nat_add(a5101, a5102)?
        };
        let a5104 = 1u64;
        nat_sub(a5103, a5104)
    };
    let a5106 = v1;
    let a5107 = 0u64;
    Ok(nat_quot(a5105, a5106, a5107))
}

pub fn f245(v0: u64, v1: u64) -> R<u64> {
    let a5108 = v0;
    let a5109 = f244(v0, v1)?;
    let a5110 = 0u64;
    Ok(nat_quot(a5108, a5109, a5110))
}

pub fn f246(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5111 = v2;
    let a5112 = 0u64;
    if nat_eq(a5111, a5112) {
        Ok(0u64)
    } else {
        let a5117 = 11u64;
        let a5118 = {
            let a5115 = {
                let a5113 = v1;
                let a5114 = v2;
                nat_mul(a5113, a5114)?
            };
            let a5116 = f261(v0)?;
            nat_mul(a5115, a5116)?
        };
        nat_add(a5117, a5118)
    }
}

pub fn f247(v0: u64, v1: u64) -> R<u64> {
    let a5119 = v0;
    let a5120 = f244(v0, v1)?;
    let a5121 = 0u64;
    Ok(nat_rem(a5119, a5120, a5121))
}

pub fn f248(v0: u64) -> bool {
    let m5122 = f114(v0);
    match m5122 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f249(v0: u64, v1: u64) -> bool {
    let a5123 = v0;
    let a5124 = 1u64;
    if nat_eq(a5123, a5124) {
        true
    } else {
        let v2: u64 = f262(v0);
        let a5125 = 0u64;
        let a5126 = v2;
        if nat_lt(a5125, a5126) {
            let a5130 = {
                let a5127 = v1;
                let a5128 = v2;
                let a5129 = 1u64;
                nat_rem(a5127, a5128, a5129)
            };
            let a5131 = 0u64;
            nat_eq(a5130, a5131)
        } else {
            false
        }
    }
}

pub fn f250(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5132 = v0;
    let a5133 = 28u64;
    if if if if nat_eq(a5132, a5133) {
        let a5134 = v1;
        let a5135 = 0u64;
        nat_eq(a5134, a5135)
    } else {
        false
    } {
        let a5136 = v2;
        let a5137 = 0u64;
        nat_eq(a5136, a5137)
    } else {
        false
    } {
        let a5138 = v3;
        let a5139 = 0u64;
        nat_eq(a5138, a5139)
    } else {
        false
    } {
        let a5140 = v4;
        let a5141 = v8;
        if if if if if if if if nat_eq(a5140, a5141) {
            let a5142 = v5;
            let a5143 = v12;
            nat_eq(a5142, a5143)
        } else {
            false
        } {
            let a5144 = v9;
            let a5145 = v13;
            nat_eq(a5144, a5145)
        } else {
            false
        } {
            let a5146 = v10;
            let a5147 = v14;
            nat_eq(a5146, a5147)
        } else {
            false
        } {
            let a5148 = v11;
            let a5149 = v15;
            nat_eq(a5148, a5149)
        } else {
            false
        } {
            let a5150 = 0u64;
            let a5151 = v6;
            nat_lt(a5150, a5151)
        } else {
            false
        } {
            let a5152 = 0u64;
            let a5153 = v7;
            nat_lt(a5152, a5153)
        } else {
            false
        } {
            let a5157 = {
                let a5154 = v10;
                let a5155 = v6;
                let a5156 = 1u64;
                nat_rem(a5154, a5155, a5156)
            };
            let a5158 = 0u64;
            if nat_eq(a5157, a5158) {
                let a5162 = {
                    let a5159 = v11;
                    let a5160 = v7;
                    let a5161 = 1u64;
                    nat_rem(a5159, a5160, a5161)
                };
                let a5163 = 0u64;
                nat_eq(a5162, a5163)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5164 = v10;
                let a5165 = v6;
                let a5166 = 0u64;
                nat_quot(a5164, a5165, a5166)
            }, {
                let a5167 = v11;
                let a5168 = v7;
                let a5169 = 0u64;
                nat_quot(a5167, a5168, a5169)
            }, {
                let a5170 = v5;
                let a5171 = v4;
                nat_mul(a5170, a5171)?
            }, {
                let a5172 = v4;
                let a5173 = v9;
                nat_mul(a5172, a5173)?
            }, {
                let a5174 = v5;
                let a5175 = v9;
                nat_mul(a5174, a5175)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f251(v0: u64) -> bool {
    let a5176 = v0;
    let a5177 = 0u64;
    if if nat_eq(a5176, a5177) {
        true
    } else {
        let a5178 = v0;
        let a5179 = 1u64;
        nat_eq(a5178, a5179)
    } {
        true
    } else {
        let a5180 = v0;
        let a5181 = 30u64;
        nat_eq(a5180, a5181)
    }
}

pub fn f252(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5182, _, _, _, _, _, _, _) => {
            h5182
        }
    };
    let v7: u64 = {
        let a5185 = 1u64;
        let a5186 = {
            let a5183 = 3u64;
            let a5184 = v6;
            nat_mul(a5183, a5184)?
        };
        nat_add(a5185, a5186)?
    };
    let a5187 = v0;
    let a5188 = 0u64;
    if nat_eq(a5187, a5188) {
        Ok(f263(0u64))
    } else {
        let a5189 = v0;
        let a5190 = v7;
        if nat_lt(a5189, a5190) {
            let v8: u64 = {
                let a5191 = v0;
                let a5192 = 1u64;
                nat_sub(a5191, a5192)
            };
            let v9: u64 = {
                let a5198 = {
                    let a5193 = v1;
                    let a5194 = v6;
                    nat_mul(a5193, a5194)?
                };
                let a5199 = {
                    let a5195 = v8;
                    let a5196 = 3u64;
                    let a5197 = 0u64;
                    nat_quot(a5195, a5196, a5197)
                };
                nat_add(a5198, a5199)?
            };
            let v10: u64 = f264(v9, v2)?;
            let v11: u64 = {
                let a5200 = v8;
                let a5201 = 3u64;
                let a5202 = 0u64;
                nat_rem(a5200, a5201, a5202)
            };
            let a5203 = v11;
            let a5204 = 0u64;
            if nat_eq(a5203, a5204) {
                Ok(f265(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5205 = v11;
                let a5206 = 1u64;
                if nat_eq(a5205, a5206) {
                    Ok(f265(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f265(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5207 = v0;
            let a5208 = v7;
            if nat_eq(a5207, a5208) {
                Ok(f265(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5211 = v0;
                let a5212 = {
                    let a5209 = v7;
                    let a5210 = 1u64;
                    nat_add(a5209, a5210)?
                };
                if nat_eq(a5211, a5212) {
                    Ok(f265(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5215 = v0;
                    let a5216 = {
                        let a5213 = v7;
                        let a5214 = 2u64;
                        nat_add(a5213, a5214)?
                    };
                    if nat_eq(a5215, a5216) {
                        Ok(f265(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5219 = v0;
                        let a5220 = {
                            let a5217 = v7;
                            let a5218 = 3u64;
                            nat_add(a5217, a5218)?
                        };
                        if nat_eq(a5219, a5220) {
                            let a5221 = v5;
                            let a5222 = 1u64;
                            if if nat_eq(a5221, a5222) {
                                true
                            } else {
                                let a5223 = v5;
                                let a5224 = 2u64;
                                nat_eq(a5223, a5224)
                            } {
                                Ok(f265(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f265(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5231 = {
                                    let a5225 = v1;
                                    let a5226 = v6;
                                    nat_mul(a5225, a5226)?
                                };
                                let a5232 = {
                                    let a5229 = v0;
                                    let a5230 = {
                                        let a5227 = v7;
                                        let a5228 = 4u64;
                                        nat_add(a5227, a5228)?
                                    };
                                    nat_sub(a5229, a5230)
                                };
                                nat_add(a5231, a5232)?
                            };
                            Ok(f265(10u64, 0u64, f264(v12, v2)?, 1u64, 2u64, 3u64, f264(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f253(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5233, _, _, _, _, _, _, _) => {
            h5233
        }
    }, match v0.clone() {
        Adt20::C0(_, h5234, _, _, _, _, _, _) => {
            h5234
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5235, _, _, _, _, _) => {
            h5235
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5236, _, _, _, _) => {
            h5236
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5237, _, _) => {
            h5237
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5238, _) => {
            h5238
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5239) => {
            h5239
        }
    })
}

pub fn f254(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5246 = v3;
        let a5247 = {
            let a5243 = f264({
                let a5240 = v0;
                let a5241 = 2u64;
                let a5242 = 0u64;
                nat_quot(a5240, a5241, a5242)
            }, v1)?;
            let a5244 = v2;
            let a5245 = 0u64;
            nat_quot(a5243, a5244, a5245)
        };
        nat_add(a5246, a5247)?
    };
    let a5251 = {
        let a5248 = v0;
        let a5249 = 2u64;
        let a5250 = 0u64;
        nat_rem(a5248, a5249, a5250)
    };
    let a5252 = 0u64;
    if nat_eq(a5251, a5252) {
        Ok(f265(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5253 = v4;
        let a5254 = 0u64;
        if nat_eq(a5253, a5254) {
            Ok(f265(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f255(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f266(match v2.clone() {
        Adt20::C0(h5255, _, _, _, _, _, _, _) => {
            h5255
        }
    })?;
    let v7: u64 = f267(match v2.clone() {
        Adt20::C0(h5256, _, _, _, _, _, _, _) => {
            h5256
        }
    })?;
    let a5257 = v0;
    let a5258 = 2u64;
    if nat_lt(a5257, a5258) {
        Ok(f263(v0))
    } else {
        let a5259 = v0;
        let a5260 = v6;
        if nat_lt(a5259, a5260) {
            f268({
                let a5261 = v0;
                let a5262 = 2u64;
                nat_sub(a5261, a5262)
            }, v1, v2, v3)
        } else {
            let a5263 = v0;
            let a5264 = v7;
            if nat_lt(a5263, a5264) {
                Ok(f269({
                    let a5265 = v0;
                    let a5266 = v6;
                    nat_sub(a5265, a5266)
                }, match v2 {
                    Adt20::C0(h5267, _, _, _, _, _, _, _) => {
                        h5267
                    }
                }, v5))
            } else {
                let a5269 = v0;
                let a5270 = f227(match v2.clone() {
                    Adt20::C0(h5268, _, _, _, _, _, _, _) => {
                        h5268
                    }
                })?;
                if nat_lt(a5269, a5270) {
                    f270({
                        let a5271 = v0;
                        let a5272 = v7;
                        nat_sub(a5271, a5272)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f256(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f241(v5, match v2.clone() {
        Adt20::C0(h5273, _, _, _, _, _, _, _) => {
            h5273
        }
    })?;
    let v7: u64 = {
        let a5275 = v6;
        let a5276 = f242(v5, match v2.clone() {
            Adt20::C0(h5274, _, _, _, _, _, _, _) => {
                h5274
            }
        })?;
        nat_add(a5275, a5276)?
    };
    let v8: u64 = {
        let a5277 = v7;
        let a5278 = f243(v5);
        nat_add(a5277, a5278)?
    };
    let a5279 = v0;
    let a5280 = v6;
    if nat_lt(a5279, a5280) {
        f271(v0, v1, v2, v5)
    } else {
        let a5281 = v0;
        let a5282 = v7;
        if nat_lt(a5281, a5282) {
            f272({
                let a5283 = v0;
                let a5284 = v6;
                nat_sub(a5283, a5284)
            }, v1, v2, v3, v5)
        } else {
            let a5285 = v0;
            let a5286 = v8;
            if nat_lt(a5285, a5286) {
                Ok(f273({
                    let a5287 = v0;
                    let a5288 = v7;
                    nat_sub(a5287, a5288)
                }, match v2 {
                    Adt20::C0(h5289, _, _, _, _, _, _, _) => {
                        h5289
                    }
                }, v4, v5))
            } else {
                let a5293 = v0;
                let a5294 = {
                    let a5291 = v8;
                    let a5292 = match v2.clone() {
                        Adt20::C0(h5290, _, _, _, _, _, _, _) => {
                            h5290
                        }
                    };
                    nat_add(a5291, a5292)?
                };
                if nat_lt(a5293, a5294) {
                    let v9: u64 = f264({
                        let a5300 = {
                            let a5296 = v1;
                            let a5297 = match v2 {
                                Adt20::C0(h5295, _, _, _, _, _, _, _) => {
                                    h5295
                                }
                            };
                            nat_mul(a5296, a5297)?
                        };
                        let a5301 = {
                            let a5298 = v0;
                            let a5299 = v8;
                            nat_sub(a5298, a5299)
                        };
                        nat_add(a5300, a5301)?
                    }, v3)?;
                    Ok(f265(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f257(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f244(v3, v5)?;
    let v7: u64 = f246(v1, v2, v6)?;
    let v8: u64 = {
        let a5304 = {
            let a5302 = f245(v3, v5)?;
            let a5303 = v4;
            nat_mul(a5302, a5303)?
        };
        let a5305 = v7;
        nat_mul(a5304, a5305)?
    };
    let a5306 = v0;
    let a5307 = v8;
    if nat_lt(a5306, a5307) {
        let v9: u64 = {
            let a5308 = v0;
            let a5309 = v7;
            let a5310 = 0u64;
            nat_quot(a5308, a5309, a5310)
        };
        Ok(Adt22::C0({
            let a5314 = {
                let a5311 = v9;
                let a5312 = v4;
                let a5313 = 0u64;
                nat_quot(a5311, a5312, a5313)
            };
            let a5315 = v6;
            nat_mul(a5314, a5315)?
        }, v6, {
            let a5316 = v9;
            let a5317 = v4;
            let a5318 = 0u64;
            nat_rem(a5316, a5317, a5318)
        }, {
            let a5319 = v0;
            let a5320 = v7;
            let a5321 = 0u64;
            nat_rem(a5319, a5320, a5321)
        }))
    } else {
        let v10: u64 = {
            let a5322 = v0;
            let a5323 = v8;
            nat_sub(a5322, a5323)
        };
        let v11: u64 = f246(v1, v2, f247(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5324 = f245(v3, v5)?;
            let a5325 = v6;
            nat_mul(a5324, a5325)?
        }, f247(v3, v5)?, {
            let a5326 = v10;
            let a5327 = v11;
            let a5328 = 0u64;
            nat_quot(a5326, a5327, a5328)
        }, {
            let a5329 = v10;
            let a5330 = v11;
            let a5331 = 0u64;
            nat_rem(a5329, a5330, a5331)
        }))
    }
}

pub fn f258(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5332) => {
            h5332
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5333, _, _, _, _, _, _, _) => {
            h5333
        }
    };
    let v6: u64 = {
        let a5336 = match v0.clone() {
            Adt22::C0(_, h5334, _, _) => {
                h5334
            }
        };
        let a5337 = match v1.clone() {
            Adt20::C0(_, h5335, _, _, _, _, _, _) => {
                h5335
            }
        };
        nat_mul(a5336, a5337)?
    };
    let v7: u64 = {
        let a5338 = v6;
        let a5339 = v5;
        nat_mul(a5338, a5339)?
    };
    let v8: u64 = {
        let a5342 = {
            let a5340 = 2u64;
            let a5341 = v5;
            nat_mul(a5340, a5341)?
        };
        let a5343 = 2u64;
        nat_add(a5342, a5343)?
    };
    let v9: u64 = {
        let a5346 = {
            let a5344 = 4u64;
            let a5345 = v5;
            nat_mul(a5344, a5345)?
        };
        let a5347 = 2u64;
        nat_add(a5346, a5347)?
    };
    let v10: u64 = {
        let a5350 = 1u64;
        let a5351 = {
            let a5348 = v6;
            let a5349 = v8;
            nat_mul(a5348, a5349)?
        };
        nat_add(a5350, a5351)?
    };
    let v11: u64 = {
        let a5352 = v10;
        let a5353 = 4u64;
        nat_add(a5352, a5353)?
    };
    let v12: u64 = {
        let a5356 = v11;
        let a5357 = {
            let a5354 = v6;
            let a5355 = v9;
            nat_mul(a5354, a5355)?
        };
        nat_add(a5356, a5357)?
    };
    let v13: u64 = {
        let a5358 = v12;
        let a5359 = 6u64;
        nat_add(a5358, a5359)?
    };
    let a5360 = v4;
    let a5361 = 0u64;
    if nat_eq(a5360, a5361) {
        Ok(f263(0u64))
    } else {
        let a5362 = v4;
        let a5363 = v10;
        if nat_lt(a5362, a5363) {
            let v14: u64 = {
                let a5364 = v4;
                let a5365 = 1u64;
                nat_sub(a5364, a5365)
            };
            f274(false, {
                let a5366 = v14;
                let a5367 = v8;
                let a5368 = 0u64;
                nat_rem(a5366, a5367, a5368)
            }, {
                let a5369 = v14;
                let a5370 = v8;
                let a5371 = 0u64;
                nat_quot(a5369, a5370, a5371)
            }, v0, v1, v2)
        } else {
            let a5372 = v4;
            let a5373 = v11;
            if nat_lt(a5372, a5373) {
                Ok(f275({
                    let a5374 = v4;
                    let a5375 = v10;
                    nat_sub(a5374, a5375)
                }, v7))
            } else {
                let a5376 = v4;
                let a5377 = v12;
                if nat_lt(a5376, a5377) {
                    let v15: u64 = {
                        let a5378 = v4;
                        let a5379 = v11;
                        nat_sub(a5378, a5379)
                    };
                    f274(true, {
                        let a5380 = v15;
                        let a5381 = v9;
                        let a5382 = 0u64;
                        nat_rem(a5380, a5381, a5382)
                    }, {
                        let a5383 = v15;
                        let a5384 = v9;
                        let a5385 = 0u64;
                        nat_quot(a5383, a5384, a5385)
                    }, v0, v1, v2)
                } else {
                    let a5386 = v4;
                    let a5387 = v13;
                    if nat_lt(a5386, a5387) {
                        Ok(f276({
                            let a5388 = v4;
                            let a5389 = v12;
                            nat_sub(a5388, a5389)
                        }, v7, v3))
                    } else {
                        let a5392 = v4;
                        let a5393 = {
                            let a5390 = v13;
                            let a5391 = v7;
                            nat_add(a5390, a5391)?
                        };
                        if nat_lt(a5392, a5393) {
                            let v16: u64 = {
                                let a5394 = v4;
                                let a5395 = v13;
                                nat_sub(a5394, a5395)
                            };
                            let v17: u64 = {
                                let a5396 = v16;
                                let a5397 = v5;
                                let a5398 = 0u64;
                                nat_quot(a5396, a5397, a5398)
                            };
                            let v18: u64 = {
                                let a5404 = {
                                    let a5399 = f277(v17, v0, v1)?;
                                    let a5400 = v5;
                                    nat_mul(a5399, a5400)?
                                };
                                let a5405 = {
                                    let a5401 = v16;
                                    let a5402 = v5;
                                    let a5403 = 0u64;
                                    nat_rem(a5401, a5402, a5403)
                                };
                                nat_add(a5404, a5405)?
                            };
                            let v19: u64 = f264(v18, v2)?;
                            Ok(f265(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f259(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f260(v0: u64) -> u64 {
    let a5406 = v0;
    let a5407 = 4u64;
    let a5408 = 0u64;
    nat_quot(a5406, a5407, a5408)
}

pub fn f261(v0: u64) -> R<u64> {
    let a5411 = {
        let a5409 = 7u64;
        let a5410 = v0;
        nat_mul(a5409, a5410)?
    };
    let a5412 = 4u64;
    nat_add(a5411, a5412)
}

pub fn f262(v0: u64) -> u64 {
    if f112(v0) {
        let m5413 = f114(v0);
        match m5413 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5415 = 1u64;
                let a5416 = match v1.clone() {
                    Adt18::C0(_, h5414, _) => {
                        h5414
                    }
                };
                if if nat_lt(a5415, a5416) {
                    true
                } else {
                    let a5417 = v0;
                    let a5418 = 43u64;
                    nat_eq(a5417, a5418)
                } {
                    match v1 {
                        Adt18::C0(_, h5419, _) => {
                            h5419
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

pub fn f263(v0: u64) -> Adt19 {
    f265(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f264(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5421 = v0;
        let a5422 = match v1.clone() {
            Adt20::C0(h5420, _, _, _, _, _, _, _) => {
                h5420
            }
        };
        let a5423 = 0u64;
        nat_quot(a5421, a5422, a5423)
    };
    let v3: u64 = {
        let a5425 = v2;
        let a5426 = match v1.clone() {
            Adt20::C0(_, h5424, _, _, _, _, _, _) => {
                h5424
            }
        };
        let a5427 = 0u64;
        nat_quot(a5425, a5426, a5427)
    };
    f130({
        let a5429 = v0;
        let a5430 = match v1.clone() {
            Adt20::C0(h5428, _, _, _, _, _, _, _) => {
                h5428
            }
        };
        let a5431 = 0u64;
        nat_rem(a5429, a5430, a5431)
    }, {
        let a5433 = v2;
        let a5434 = match v1.clone() {
            Adt20::C0(_, h5432, _, _, _, _, _, _) => {
                h5432
            }
        };
        let a5435 = 0u64;
        nat_rem(a5433, a5434, a5435)
    }, {
        let a5437 = v3;
        let a5438 = match v1.clone() {
            Adt20::C0(_, _, h5436, _, _, _, _, _) => {
                h5436
            }
        };
        let a5439 = 0u64;
        nat_rem(a5437, a5438, a5439)
    }, {
        let a5441 = v3;
        let a5442 = match v1.clone() {
            Adt20::C0(_, _, h5440, _, _, _, _, _) => {
                h5440
            }
        };
        let a5443 = 0u64;
        nat_quot(a5441, a5442, a5443)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5444, _, _, _) => {
            h5444
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5445, _, _) => {
            h5445
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5446, _) => {
            h5446
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5447) => {
            h5447
        }
    })
}

pub fn f265(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5448 = v0;
    let a5449 = 4u64;
    if if if nat_le(a5448, a5449) {
        true
    } else {
        let a5450 = 9u64;
        let a5451 = v0;
        if nat_le(a5450, a5451) {
            let a5452 = v0;
            let a5453 = 12u64;
            nat_le(a5452, a5453)
        } else {
            false
        }
    } {
        true
    } else {
        let a5454 = v0;
        let a5455 = 16u64;
        if if nat_eq(a5454, a5455) {
            true
        } else {
            let a5456 = v0;
            let a5457 = 19u64;
            nat_eq(a5456, a5457)
        } {
            true
        } else {
            let a5458 = v0;
            let a5459 = 21u64;
            if nat_eq(a5458, a5459) {
                true
            } else {
                let a5460 = v0;
                let a5461 = 22u64;
                nat_eq(a5460, a5461)
            }
        }
    } {
        Adt19::C1({
            let a5462 = v0;
            let a5463 = 4u64;
            if if nat_lt(a5462, a5463) {
                true
            } else {
                let a5464 = 21u64;
                let a5465 = v0;
                nat_le(a5464, a5465)
            } {
                0u64
            } else {
                let a5466 = v0;
                let a5467 = 16u64;
                if if nat_eq(a5466, a5467) {
                    true
                } else {
                    let a5468 = v0;
                    let a5469 = 19u64;
                    nat_eq(a5468, a5469)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5470 = v0;
            let a5471 = 4u64;
            if nat_lt(a5470, a5471) {
                v0
            } else {
                let a5472 = v0;
                let a5473 = 4u64;
                if if nat_eq(a5472, a5473) {
                    true
                } else {
                    let a5474 = v0;
                    let a5475 = 16u64;
                    nat_eq(a5474, a5475)
                } {
                    0u64
                } else {
                    let a5476 = v0;
                    let a5477 = 19u64;
                    if nat_eq(a5476, a5477) {
                        3u64
                    } else {
                        let a5478 = 21u64;
                        let a5479 = v0;
                        if nat_le(a5478, a5479) {
                            let a5480 = v0;
                            let a5481 = 17u64;
                            nat_sub(a5480, a5481)
                        } else {
                            let a5482 = v0;
                            let a5483 = 8u64;
                            nat_sub(a5482, a5483)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f266(v0: u64) -> R<u64> {
    let a5486 = 2u64;
    let a5487 = {
        let a5484 = 6u64;
        let a5485 = v0;
        nat_mul(a5484, a5485)?
    };
    nat_add(a5486, a5487)
}

pub fn f267(v0: u64) -> R<u64> {
    let a5488 = f266(v0)?;
    let a5489 = 11u64;
    nat_add(a5488, a5489)
}

pub fn f268(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5490 = v0;
        let a5491 = 6u64;
        let a5492 = 0u64;
        nat_rem(a5490, a5491, a5492)
    };
    let v5: u64 = {
        let a5499 = {
            let a5494 = v1;
            let a5495 = match v2.clone() {
                Adt20::C0(h5493, _, _, _, _, _, _, _) => {
                    h5493
                }
            };
            nat_mul(a5494, a5495)?
        };
        let a5500 = {
            let a5496 = v0;
            let a5497 = 6u64;
            let a5498 = 0u64;
            nat_quot(a5496, a5497, a5498)
        };
        nat_add(a5499, a5500)?
    };
    let v6: u64 = f264(v5, v3)?;
    let v7: u64 = f264(v5, v2)?;
    let a5501 = v4;
    let a5502 = 0u64;
    if nat_eq(a5501, a5502) {
        Ok(f265(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5503 = v4;
        let a5504 = 1u64;
        if nat_eq(a5503, a5504) {
            Ok(f265(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5505 = v4;
            let a5506 = 2u64;
            if nat_eq(a5505, a5506) {
                Ok(f265(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5507 = v4;
                let a5508 = 3u64;
                if nat_eq(a5507, a5508) {
                    Ok(f265(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5509 = v4;
                    let a5510 = 4u64;
                    if nat_eq(a5509, a5510) {
                        Ok(f265(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f265(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f269(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5511 = v0;
    let a5512 = 0u64;
    if nat_eq(a5511, a5512) {
        f265(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5513 = v0;
        let a5514 = 1u64;
        if nat_eq(a5513, a5514) {
            f265(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5515 = v0;
            let a5516 = 2u64;
            if nat_eq(a5515, a5516) {
                f265(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5517 = v0;
                let a5518 = 3u64;
                if nat_eq(a5517, a5518) {
                    f265(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5519 = v0;
                    let a5520 = 4u64;
                    if nat_eq(a5519, a5520) {
                        f265(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5521 = v0;
                        let a5522 = 5u64;
                        if nat_eq(a5521, a5522) {
                            f265(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5523 = v0;
                            let a5524 = 6u64;
                            if nat_eq(a5523, a5524) {
                                f265(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5525 = v0;
                                let a5526 = 7u64;
                                if nat_eq(a5525, a5526) {
                                    f265(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5527 = v0;
                                    let a5528 = 8u64;
                                    if nat_eq(a5527, a5528) {
                                        f265(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5529 = v0;
                                        let a5530 = 9u64;
                                        if nat_eq(a5529, a5530) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5531 = v0;
                                            let a5532 = 10u64;
                                            if nat_eq(a5531, a5532) {
                                                f265(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f270(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5533 = v0;
        let a5534 = 3u64;
        let a5535 = 0u64;
        nat_rem(a5533, a5534, a5535)
    };
    let v6: u64 = {
        let a5542 = {
            let a5537 = v1;
            let a5538 = match v2.clone() {
                Adt20::C0(h5536, _, _, _, _, _, _, _) => {
                    h5536
                }
            };
            nat_mul(a5537, a5538)?
        };
        let a5543 = {
            let a5539 = v0;
            let a5540 = 3u64;
            let a5541 = 0u64;
            nat_quot(a5539, a5540, a5541)
        };
        nat_add(a5542, a5543)?
    };
    let v7: u64 = f264(v6, v3)?;
    let v8: u64 = f264(v6, v2)?;
    let v9: u64 = f264(v6, v4)?;
    let a5544 = v5;
    let a5545 = 0u64;
    if nat_eq(a5544, a5545) {
        Ok(f265(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5546 = v5;
        let a5547 = 1u64;
        if nat_eq(a5546, a5547) {
            Ok(f265(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f265(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f271(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5548, _, _, _, _, _, _, _) => {
            h5548
        }
    };
    let v5: u64 = {
        let a5551 = 1u64;
        let a5552 = {
            let a5549 = v4;
            let a5550 = f259(v3);
            nat_mul(a5549, a5550)?
        };
        nat_add(a5551, a5552)?
    };
    let a5553 = v0;
    let a5554 = 0u64;
    if nat_eq(a5553, a5554) {
        Ok(f263(0u64))
    } else {
        let a5555 = v0;
        let a5556 = v5;
        if nat_lt(a5555, a5556) {
            let v6: u64 = {
                let a5557 = v0;
                let a5558 = 1u64;
                nat_sub(a5557, a5558)
            };
            let v7: u64 = f264({
                let a5564 = {
                    let a5559 = v1;
                    let a5560 = v4;
                    nat_mul(a5559, a5560)?
                };
                let a5565 = {
                    let a5561 = v6;
                    let a5562 = f259(v3);
                    let a5563 = 0u64;
                    nat_quot(a5561, a5562, a5563)
                };
                nat_add(a5564, a5565)?
            }, v2)?;
            if v3 {
                let a5569 = {
                    let a5566 = v6;
                    let a5567 = 2u64;
                    let a5568 = 0u64;
                    nat_rem(a5566, a5567, a5568)
                };
                let a5570 = 0u64;
                if nat_eq(a5569, a5570) {
                    Ok(f265(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f265(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f265(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5571 = v0;
                let a5572 = v5;
                nat_sub(a5571, a5572)
            };
            let a5573 = v8;
            let a5574 = 0u64;
            if nat_eq(a5573, a5574) {
                Ok(f265(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5575 = v8;
                    let a5576 = 1u64;
                    nat_eq(a5575, a5576)
                } else {
                    false
                } {
                    Ok(f265(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5577 = v8;
                    let a5578 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5577, a5578) {
                        Ok(f265(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5579 = v8;
                        let a5580 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5579, a5580) {
                            Ok(f263(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f272(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5582 = 13u64;
        let a5583 = f260(match v2.clone() {
            Adt20::C0(h5581, _, _, _, _, _, _, _) => {
                h5581
            }
        });
        nat_mul(a5582, a5583)?
    } else {
        0u64
    };
    let a5584 = v0;
    let a5585 = v5;
    if nat_lt(a5584, a5585) {
        f278({
            let a5586 = v0;
            let a5587 = 13u64;
            let a5588 = 0u64;
            nat_rem(a5586, a5587, a5588)
        }, v1, {
            let a5589 = v0;
            let a5590 = 13u64;
            let a5591 = 0u64;
            nat_quot(a5589, a5590, a5591)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5592 = v0;
            let a5593 = v5;
            nat_sub(a5592, a5593)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f279({
            let a5594 = v6;
            let a5595 = v7;
            let a5596 = 0u64;
            nat_rem(a5594, a5595, a5596)
        }, {
            let a5606 = {
                let a5601 = {
                    let a5598 = v1;
                    let a5599 = match v2.clone() {
                        Adt20::C0(h5597, _, _, _, _, _, _, _) => {
                            h5597
                        }
                    };
                    nat_mul(a5598, a5599)?
                };
                let a5602 = if v4 {
                    f280(match v2.clone() {
                        Adt20::C0(h5600, _, _, _, _, _, _, _) => {
                            h5600
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5601, a5602)?
            };
            let a5607 = {
                let a5603 = v6;
                let a5604 = v7;
                let a5605 = 0u64;
                nat_quot(a5603, a5604, a5605)
            };
            nat_add(a5606, a5607)?
        }, v2, v3, v4)
    }
}

pub fn f273(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5608 = v0;
    let a5609 = 0u64;
    if nat_eq(a5608, a5609) {
        f265(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5610 = v0;
        let a5611 = 1u64;
        if nat_eq(a5610, a5611) {
            f265(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5612 = v0;
                let a5613 = 2u64;
                nat_eq(a5612, a5613)
            } else {
                false
            } {
                f265(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5614 = v0;
                let a5615 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5614, a5615) {
                    f265(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5616 = v0;
                    let a5617 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5616, a5617) {
                        f265(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5618 = v0;
                        let a5619 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5618, a5619) {
                            f265(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f274(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5623 = 1u64;
        let a5624 = {
            let a5621 = match v4.clone() {
                Adt20::C0(h5620, _, _, _, _, _, _, _) => {
                    h5620
                }
            };
            let a5622 = v6;
            nat_mul(a5621, a5622)?
        };
        nat_add(a5623, a5624)?
    };
    let a5625 = v1;
    let a5626 = 0u64;
    if nat_eq(a5625, a5626) {
        Ok(f263(1u64))
    } else {
        let a5627 = v1;
        let a5628 = v7;
        if nat_lt(a5627, a5628) {
            let v8: u64 = {
                let a5629 = v1;
                let a5630 = 1u64;
                nat_sub(a5629, a5630)
            };
            let v9: u64 = {
                let a5637 = {
                    let a5632 = f277(v2, v3, v4.clone())?;
                    let a5633 = match v4.clone() {
                        Adt20::C0(h5631, _, _, _, _, _, _, _) => {
                            h5631
                        }
                    };
                    nat_mul(a5632, a5633)?
                };
                let a5638 = {
                    let a5634 = v8;
                    let a5635 = v6;
                    let a5636 = 0u64;
                    nat_quot(a5634, a5635, a5636)
                };
                nat_add(a5637, a5638)?
            };
            let v10: u64 = f264(v9, v4)?;
            let v11: u64 = f264(v9, v5)?;
            let v12: u64 = {
                let a5639 = v8;
                let a5640 = v6;
                let a5641 = 0u64;
                nat_rem(a5639, a5640, a5641)
            };
            if v0 {
                let a5642 = v12;
                let a5643 = 0u64;
                if nat_eq(a5642, a5643) {
                    Ok(f265(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5644 = v12;
                    let a5645 = 1u64;
                    if nat_eq(a5644, a5645) {
                        Ok(f265(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5646 = v12;
                        let a5647 = 2u64;
                        if nat_eq(a5646, a5647) {
                            Ok(f265(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f265(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5648 = v12;
                let a5649 = 0u64;
                if nat_eq(a5648, a5649) {
                    Ok(f265(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f265(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5650 = v1;
            let a5651 = v7;
            if nat_eq(a5650, a5651) {
                Ok(f265(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f275(v0: u64, v1: u64) -> Adt19 {
    let a5652 = v0;
    let a5653 = 0u64;
    if nat_eq(a5652, a5653) {
        f265(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5654 = v0;
        let a5655 = 1u64;
        if nat_eq(a5654, a5655) {
            f265(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5656 = v0;
            let a5657 = 2u64;
            if nat_eq(a5656, a5657) {
                f265(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5658 = v0;
                let a5659 = 3u64;
                if nat_eq(a5658, a5659) {
                    f263(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f276(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5660 = v0;
    let a5661 = 0u64;
    if nat_eq(a5660, a5661) {
        f265(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5662 = v0;
        let a5663 = 1u64;
        if nat_eq(a5662, a5663) {
            f265(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5664 = v0;
            let a5665 = 2u64;
            if nat_eq(a5664, a5665) {
                f265(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5666 = v0;
                let a5667 = 3u64;
                if nat_eq(a5666, a5667) {
                    f265(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5668 = v0;
                    let a5669 = 4u64;
                    if nat_eq(a5668, a5669) {
                        f265(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5670 = v0;
                        let a5671 = 5u64;
                        if nat_eq(a5670, a5671) {
                            f265(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f277(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5682 = {
        let a5680 = {
            let a5677 = {
                let a5674 = match v1.clone() {
                    Adt22::C0(_, _, h5672, _) => {
                        h5672
                    }
                };
                let a5675 = match v2.clone() {
                    Adt20::C0(_, _, h5673, _, _, _, _, _) => {
                        h5673
                    }
                };
                nat_mul(a5674, a5675)?
            };
            let a5678 = match v1 {
                Adt22::C0(h5676, _, _, _) => {
                    h5676
                }
            };
            nat_add(a5677, a5678)?
        };
        let a5681 = match v2 {
            Adt20::C0(_, h5679, _, _, _, _, _, _) => {
                h5679
            }
        };
        nat_mul(a5680, a5681)?
    };
    let a5683 = v0;
    nat_add(a5682, a5683)
}

pub fn f278(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5691 = {
            let a5689 = {
                let a5685 = v1;
                let a5686 = match v3.clone() {
                    Adt20::C0(h5684, _, _, _, _, _, _, _) => {
                        h5684
                    }
                };
                nat_mul(a5685, a5686)?
            };
            let a5690 = {
                let a5687 = v2;
                let a5688 = 4u64;
                nat_mul(a5687, a5688)?
            };
            nat_add(a5689, a5690)?
        };
        let a5692 = f281(v0);
        nat_add(a5691, a5692)?
    };
    let v6: u64 = f264(v5, v3)?;
    let v7: u64 = f264(v5, v4)?;
    let a5693 = v0;
    let a5694 = 0u64;
    if if if if nat_eq(a5693, a5694) {
        true
    } else {
        let a5695 = v0;
        let a5696 = 2u64;
        nat_eq(a5695, a5696)
    } {
        true
    } else {
        let a5697 = v0;
        let a5698 = 5u64;
        nat_eq(a5697, a5698)
    } {
        true
    } else {
        let a5699 = v0;
        let a5700 = 7u64;
        nat_eq(a5699, a5700)
    } {
        Ok(f265(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5701 = v0;
        let a5702 = 1u64;
        if if if if nat_eq(a5701, a5702) {
            true
        } else {
            let a5703 = v0;
            let a5704 = 3u64;
            nat_eq(a5703, a5704)
        } {
            true
        } else {
            let a5705 = v0;
            let a5706 = 6u64;
            nat_eq(a5705, a5706)
        } {
            true
        } else {
            let a5707 = v0;
            let a5708 = 8u64;
            nat_eq(a5707, a5708)
        } {
            Ok(f265(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5709 = v0;
                let a5710 = 1u64;
                if nat_eq(a5709, a5710) {
                    4u64
                } else {
                    let a5711 = v0;
                    let a5712 = 6u64;
                    if nat_eq(a5711, a5712) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5713 = v0;
            let a5714 = 4u64;
            if nat_eq(a5713, a5714) {
                Ok(f265(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5715 = v0;
                let a5716 = 9u64;
                if nat_eq(a5715, a5716) {
                    Ok(f265(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5717 = v0;
                    let a5718 = 10u64;
                    if nat_eq(a5717, a5718) {
                        Ok(f265(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5719 = v0;
                        let a5720 = 11u64;
                        if nat_eq(a5719, a5720) {
                            Ok(f265(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5721 = v0;
                            let a5722 = 12u64;
                            if nat_eq(a5721, a5722) {
                                Ok(f265(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f279(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f264(v1, v2)?;
    let v6: u64 = f264(v1, v3)?;
    let a5723 = v0;
    let a5724 = 0u64;
    if nat_eq(a5723, a5724) {
        Ok(f265(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5725 = v0;
        let a5726 = 1u64;
        if nat_eq(a5725, a5726) {
            Ok(f265(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5727 = v0;
                let a5728 = 2u64;
                if nat_eq(a5727, a5728) {
                    Ok(f265(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5729 = v0;
                    let a5730 = 3u64;
                    if nat_eq(a5729, a5730) {
                        Ok(f265(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5731 = v0;
                let a5732 = 2u64;
                if nat_eq(a5731, a5732) {
                    Ok(f265(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f280(v0: u64) -> R<u64> {
    let a5733 = 4u64;
    let a5734 = f260(v0);
    nat_mul(a5733, a5734)
}

pub fn f281(v0: u64) -> u64 {
    let a5735 = v0;
    let a5736 = 5u64;
    if nat_lt(a5735, a5736) {
        let a5737 = v0;
        let a5738 = 2u64;
        if nat_lt(a5737, a5738) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5739 = v0;
        let a5740 = 7u64;
        if nat_lt(a5739, a5740) {
            2u64
        } else {
            3u64
        }
    }
}
